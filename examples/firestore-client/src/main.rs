use gcloud_sdk::google::firestore::v1::firestore_client::FirestoreClient;
use gcloud_sdk::*;
use gcloud_sdk::google::firestore::v1::ListDocumentsRequest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let google_project_id = std::env::var("PROJECT_ID")?;

    let cloud_resource_prefix = format!("projects/{}/databases/(default)", google_project_id);

    let firestore_client: GoogleApiClientFn<FirestoreClient<GoogleConnectorInterceptedService>> =
        GoogleApiClient::from_function(
            FirestoreClient::with_interceptor,
            "https://firestore.googleapis.com",
            chrono::Duration::minutes(15), // max caching client duration
            // cloud resource prefix: used only for some of the APIs (such as Firestore)
            Some(cloud_resource_prefix.clone()),
        )
        .await?;

    let response =
        firestore_client
            .get()
            .await?
            .list_documents(tonic::Request::new(ListDocumentsRequest {
                parent: format!(
                    "{}/documents",
                    cloud_resource_prefix
                ),
            ..Default::default()
        }))
        .await?;

    println!("Response from firestore: {:?}", response);

    Ok(())
}
