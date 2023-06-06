use gcloud_sdk::google::iam::credentials::v1::iam_credentials_client::IamCredentialsClient;
use gcloud_sdk::google::iam::credentials::v1::GenerateIdTokenRequest;
use gcloud_sdk::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Debug logging
    let subscriber = tracing_subscriber::fmt()
        .with_env_filter("gcloud_sdk=debug")
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    let client: GoogleApi<IamCredentialsClient<GoogleAuthMiddleware>> = GoogleApi::from_function(
        IamCredentialsClient::new,
        "https://iamcredentials.googleapis.com",
        // cloud resource prefix: used only for some of the APIs (such as Firestore)
        None,
    )
    .await?;

    // Option 1: using service account impersonation
    let resp = client
        .get()
        .generate_id_token(tonic::Request::new(GenerateIdTokenRequest {
            name: format!(
                "projects/-/serviceAccounts/{}",
                std::env::var("SERVICE_ACCOUNT")?
            ),
            audience: "test-aud".to_string(),
            ..Default::default()
        }))
        .await?;

    println!("Response: {:?}", resp);

    // Option 2: using Metadata server (will work only on GCP)
    let resp2 = GceMetadataClient::new(GCP_DEFAULT_SCOPES.clone())
        .id_token("test-aud")
        .await?;
    println!("Response: {:?}", resp2.as_sensitive_str());

    Ok(())
}
