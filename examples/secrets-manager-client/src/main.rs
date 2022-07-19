use gcloud_sdk::google::cloud::secretmanager::v1::secret_manager_service_client::SecretManagerServiceClient;
use gcloud_sdk::google::cloud::secretmanager::v1::{
    AccessSecretVersionRequest, GetSecretVersionRequest, ListSecretsRequest,
};
use gcloud_sdk::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let google_project_id = std::env::var("PROJECT_ID")?;

    // Debug logging
    let subscriber = tracing_subscriber::fmt()
        .with_env_filter("gcloud_sdk=debug")
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    let secrets_client: GoogleApi<SecretManagerServiceClient<GoogleAuthMiddleware>> =
        GoogleApi::from_function(
            SecretManagerServiceClient::new,
            "https://secretmanager.googleapis.com",
            // cloud resource prefix: used only for some of the APIs (such as Firestore)
            None,
        )
        .await?;

    let response = secrets_client
        .get()
        .access_secret_version(tonic::Request::new(AccessSecretVersionRequest {
            name: format!(
                "projects/{}/secrets/tm-http-sessions-secret/versions/latest",
                google_project_id,
            ),
            ..Default::default()
        }))
        .await?;
    println!("Response: {:#?}", response);

    Ok(())
}
