use gcloud_sdk::google::iam::credentials::v1::iam_credentials_client::IamCredentialsClient;
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

    let service_account = std::env::var("TEST_SERVICE_ACCOUNT")?;

    let resp = client
        .get()
        .sign_blob(tonic::Request::new(
            gcloud_sdk::google::iam::credentials::v1::SignBlobRequest {
                name: format!("projects/-/serviceAccounts/{}", service_account),
                delegates: vec![],
                payload: "https://storage.googleapis.com/example-bucket/cat.jpeg"
                    .as_bytes()
                    .to_vec(),
            },
        ))
        .await?;

    println!("Response: {:x?}", resp.into_inner().signed_blob);

    Ok(())
}
