use gcloud_sdk::google::firestore::v1::firestore_client::FirestoreClient;
use gcloud_sdk::google::firestore::v1::ListDocumentsRequest;
use gcloud_sdk::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let google_project_id = std::env::var("PROJECT_ID")?;

    // Debug logging
    let subscriber = tracing_subscriber::fmt()
        .with_env_filter("gcloud_sdk=debug")
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

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
