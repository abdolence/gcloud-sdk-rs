use gcloud_sdk::google::firestore::v1::firestore_client::FirestoreClient;
use gcloud_sdk::google::firestore::v1::ListDocumentsRequest;
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

    let cloud_resource_prefix = format!("projects/{}/databases/(default)", google_project_id);

    let firestore_client: GoogleApi<FirestoreClient<GoogleAuthMiddleware>> =
        GoogleApi::from_function(
            FirestoreClient::new,
            "https://firestore.googleapis.com",
            // cloud resource prefix: used only for some of the APIs (such as Firestore)
            Some(cloud_resource_prefix.clone()),
        )
        .await?;

    let response = firestore_client
        .get()
        .list_documents(tonic::Request::new(ListDocumentsRequest {
            parent: format!("{}/documents", cloud_resource_prefix),
            ..Default::default()
        }))
        .await?;
    println!("Response from firestore: {:?}", response);

    Ok(())
}
