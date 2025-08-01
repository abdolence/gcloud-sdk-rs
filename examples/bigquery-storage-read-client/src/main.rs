use futures::TryStreamExt;
use gcloud_sdk::google::cloud::bigquery::storage::v1::append_rows_request::ProtoData;
use gcloud_sdk::google::cloud::bigquery::storage::v1::big_query_read_client::BigQueryReadClient;
use gcloud_sdk::google::cloud::bigquery::storage::v1::{
    DataFormat, ProtoRows, ProtoSchema, CreateReadSessionRequest, ReadSession, ReadRowsRequest
};

use gcloud_sdk::*;


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

    let read_session = ReadSession{
        data_format: DataFormat::Arrow as i32,
        table: "projects/bigquery-public-data/datasets/usa_names/tables/usa_1910_2013".to_string(),
        ..Default::default()
    };

    let request = CreateReadSessionRequest{
        parent: format!(
            "projects/{google_project_id}"
        ),
        max_stream_count: 1,
        read_session: Some(read_session),
        ..Default::default()
    };

    let read_session = read_client.get().create_read_session(request).await?;

    let mut batches = Vec::new();

    // TODO: request more than one stream and do this in parallel (with some
    // thread safety added).
    for stream in read_session.into_inner().streams {
        let stream_name = stream.name;
        let read_rows_request = ReadRowsRequest{
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
                    batches.push(value);
                },
                None => {break 'messages;}
            }
        }
    }

    // let response = write_client.get().append_rows(rows_stream).await?;

    // let response_stream = response.into_inner();

    // let collected: Vec<AppendRowsResponse> = response_stream.try_collect().await?;

    println!("Response: {:?}", batches.len());

    Ok(())
}
