use arrow::ipc::reader::StreamReader;
use arrow::record_batch::RecordBatch;
use gcloud_sdk::google::cloud::bigquery::storage::v1::big_query_read_client::BigQueryReadClient;
use gcloud_sdk::google::cloud::bigquery::storage::v1::{
    read_rows_response, read_session, CreateReadSessionRequest, DataFormat, ReadRowsRequest,
    ReadRowsResponse, ReadSession,
};
use std::io::Cursor;
use std::sync::Arc;
use std::time::Instant;

use gcloud_sdk::*;

fn read_rows_response_to_record_batch(response: ReadRowsResponse, schema: &Vec<u8>) -> RecordBatch {
    let mut buffer = Vec::new();
    // TODO: We're not quite zero-copy with this approach. Maybe using the
    // StreamReader once for the whole thing would be better?
    buffer.append(&mut schema.clone());
    // TODO: Bubble up if we unexpectedly get a record batch with no rows.
    // TODO: This might not actually be unexpected? What happens when there's a
    // super selective row filter?
    let mut serialized_record_batch = match response.rows.unwrap() {
        read_rows_response::Rows::ArrowRecordBatch(value) => value.serialized_record_batch,
        _ => panic!("unexpectedly got some format other than arrow bytes"),
    };
    buffer.append(&mut serialized_record_batch);

    let cursor = Cursor::new(buffer);

    // TODO: Bubble up if we unexpectedly get a bad record batch.
    let mut reader = StreamReader::try_new(cursor, None).unwrap();

    // TODO: maybe double-check that there are no recordbatches after this?
    // There should only be one if the API returned the expected results.
    reader.next().unwrap().expect("missing recordbatch")
}

async fn read_stream(
    read_client: Arc<GoogleApi<BigQueryReadClient<GoogleAuthMiddleware>>>,
    schema: Arc<Vec<u8>>,
    stream_name: String,
    tx: Arc<tokio::sync::mpsc::Sender<RecordBatch>>,
) {
    let read_rows_request = ReadRowsRequest {
        read_stream: stream_name.clone(),
        offset: 0,
    };

    let messages = read_client
        .get()
        .read_rows(read_rows_request)
        .await
        .unwrap();
    let mut messages = messages.into_inner();

    'messages: loop {
        // TODO: if there's an error, call read_rows with the most recent
        // offset to resume.
        let message = messages.message().await.unwrap();
        match message {
            Some(value) => {
                // If there's an error here, that means the receiver dropped, so
                // we should just exit rather than try to restart the stream at
                // offset.
                tx.send(read_rows_response_to_record_batch(value, &schema))
                    .await
                    .unwrap();
            }
            None => {
                break 'messages;
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Debug logging
    let subscriber = tracing_subscriber::fmt()
        .with_env_filter("gcloud_sdk=debug")
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    let start = Instant::now();

    // Detect Google project ID using environment variables PROJECT_ID/GCP_PROJECT_ID
    // or GKE metadata server when the app runs inside GKE
    let google_project_id = GoogleEnvironment::detect_google_project_id().await
        .expect("No Google Project ID detected. Please specify it explicitly using env variable: PROJECT_ID");

    let read_client: GoogleApi<BigQueryReadClient<GoogleAuthMiddleware>> =
        GoogleApi::from_function(
            BigQueryReadClient::new,
            "https://bigquerystorage.googleapis.com",
            None,
        )
        .await?;

    let read_session = ReadSession {
        data_format: DataFormat::Arrow as i32,
        table: "projects/bigquery-public-data/datasets/usa_names/tables/usa_1910_2013".to_string(),
        ..Default::default()
    };

    let request = CreateReadSessionRequest {
        parent: format!("projects/{google_project_id}"),
        // If you are reading from a query results table where order matters,
        // limit this to a single stream.
        max_stream_count: match std::thread::available_parallelism() {
            Ok(value) => value.get() as i32,
            Err(_) => 1,
        },
        read_session: Some(read_session),
        ..Default::default()
    };

    let read_session = read_client
        .get()
        .create_read_session(request)
        .await?
        .into_inner();
    let schema = match read_session.schema.unwrap() {
        read_session::Schema::ArrowSchema(value) => value.serialized_schema,
        _ => panic!("unexpectedly got schema type other than arrow"),
    };

    let (tx, mut rx) = tokio::sync::mpsc::channel(1024); // Create an MPSC channel
    let shared_tx = Arc::new(tx);
    let shared_client = Arc::new(read_client);
    let shared_schema = Arc::new(schema);
    let mut handles = Vec::new();

    for stream in read_session.streams {
        let stream_name = stream.name;
        let handle = tokio::task::spawn(read_stream(
            shared_client.clone(),
            shared_schema.clone(),
            stream_name,
            shared_tx.clone(),
        ));
        handles.push(handle);
    }

    // Don't need the sender here anymore. Drop it so that the receiver can know
    // when all the other uses of sender have finished.
    std::mem::drop(shared_tx);

    let mut batches = Vec::new();

    loop {
        match rx.recv().await {
            Some(value) => batches.push(value),
            None => break,
        }
    }

    let duration = start.elapsed();

    let mut messages_received = 0;
    let mut rows_received = 0;
    for batch in batches {
        messages_received += 1;
        rows_received += batch.num_rows();
    }

    println!(
        "Time to download and deserialized all messages: {:?}",
        duration
    );
    println!("Messages received: {:?}", messages_received);
    println!("Rows received: {:?}", rows_received);

    // Wait for all spawned threads to complete
    for handle in handles {
        handle.await?;
    }

    Ok(())
}
