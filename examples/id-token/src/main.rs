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

    let creds = GoogleEnvironment::find_default_creds(&gcloud_sdk::GCP_DEFAULT_SCOPES).await?;
    if let Some(creds) = creds {
        println!("Using credentials from environment variable: GOOGLE_APPLICATION_CREDENTIALS");
    } else {
        println!("No credentials found. Please set the GOOGLE_APPLICATION_CREDENTIALS environment variable.");
    }

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
    let mut metadata_client = GceMetadataClient::new(GCP_DEFAULT_SCOPES.clone());
    if metadata_client.init().await {
        let resp2 = metadata_client.id_token("test-aud").await?;
        println!("Response: {:?}", resp2.as_sensitive_str());
    }

    Ok(())
}
