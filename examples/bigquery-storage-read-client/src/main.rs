use arrow::ipc::reader::StreamReader;
use arrow::record_batch::RecordBatch;
use gcloud_sdk::google::cloud::bigquery::storage::v1::big_query_read_client::BigQueryReadClient;
use gcloud_sdk::google::cloud::bigquery::storage::v1::{
    read_rows_response, read_session, CreateReadSessionRequest, DataFormat, ReadRowsRequest,
    ReadRowsResponse, ReadSession,
};
use std::io::Cursor;

use gcloud_sdk::*;

fn read_rows_response_to_record_batch(response: ReadRowsResponse, schema: &Vec<u8>) -> RecordBatch {
    let mut buffer = Vec::new();
    // TODO: bubble up the error if we unexpectedly get a record batch with no
    // schema or not an ArrowSchema.
    // TODO: why is the schema empty?
    // let mut schema = match response.schema.unwrap() {
    //     read_rows_response::Schema::ArrowSchema(value) => value.serialized_schema,
    //     _ => panic!("unexpectedly got some format other than arrow bytes"),
    // };
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Debug logging
    let subscriber = tracing_subscriber::fmt()
        .with_env_filter("gcloud_sdk=debug")
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

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
        max_stream_count: 1,
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

    let mut messages_received = 0;
    let mut rows_received = 0;
    let mut batches = Vec::new();

    // TODO: request more than one stream and do this in parallel (with some
    // thread safety added).
    for stream in read_session.streams {
        let stream_name = stream.name;
        let read_rows_request = ReadRowsRequest {
            read_stream: stream_name,
            offset: 0,
        };
        let messages = read_client.get().read_rows(read_rows_request).await?;
        let mut messages = messages.into_inner();

        'messages: loop {
            // TODO: if there's an error, call read_rows with the most recent
            // offset to resume.
            let message = messages.message().await?;
            match message {
                Some(value) => {
                    messages_received += 1;
                    rows_received += value.row_count;
                    batches.push(read_rows_response_to_record_batch(value, &schema));
                }
                None => {
                    break 'messages;
                }
            }
        }
    }

    let mut rows_deserialized = 0;
    for batch in batches {
        rows_deserialized += batch.num_rows();
    }

    println!("Messages received: {:?}", messages_received);
    println!("Rows received: {:?}", rows_received);
    println!("Rows deserialized: {:?}", rows_deserialized);

    Ok(())
}
