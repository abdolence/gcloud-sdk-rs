use gcloud_sdk::google::devtools::artifactregistry::v1::{
    artifact_registry_client::ArtifactRegistryClient, ListRepositoriesRequest,
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

    let artifactregistry_client: GoogleApi<ArtifactRegistryClient<GoogleAuthMiddleware>> =
        GoogleApi::from_function(
            ArtifactRegistryClient::new,
            "https://artifactregistry.googleapis.com",
            // cloud resource prefix: used only for some of the APIs (such as Firestore)
            None,
        )
        .await?;

    let response = artifactregistry_client
        .get()
        .list_repositories(tonic::Request::new(ListRepositoriesRequest {
            parent: format!("projects/{google_project_id}/locations/us"),
            ..Default::default()
        }))
        .await?;

    println!("Response from artifactregistry: {response:?}");

    Ok(())
}
