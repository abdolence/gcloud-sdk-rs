use gcloud_sdk::google::logging::v2::logging_service_v2_client::LoggingServiceV2Client;
use gcloud_sdk::google::logging::v2::{log_entry, LogEntry, WriteLogEntriesRequest};
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

    let logging_client: GoogleApi<LoggingServiceV2Client<GoogleAuthMiddleware>> =
        GoogleApi::from_function(
            LoggingServiceV2Client::new,
            "https://logging.googleapis.com",
            // cloud resource prefix: used only for some of the APIs (such as Firestore)
            None,
        )
        .await?;

    let log_name = format!("projects/{}/logs/testlog", google_project_id);

    let response = logging_client
        .get()
        .write_log_entries(tonic::Request::new(WriteLogEntriesRequest {
            log_name: log_name.clone(),
            entries: vec![LogEntry {
                log_name: log_name.clone(),
                payload: Some(log_entry::Payload::TextPayload(
                    "Test log message".to_string(),
                )),
                ..Default::default()
            }],
            ..Default::default()
        }))
        .await?;

    println!("Response: {:?}", response);

    Ok(())
}
