use gcloud_sdk::google::spanner::admin::database::v1::{
    database_admin_client::DatabaseAdminClient, ListDatabasesRequest,
};
use gcloud_sdk::google_cached_client::{CachedGoogleApiClient, CachedGoogleApiClientBuilder};
use gcloud_sdk::google_tonic_connector::GoogleConnectorInterceptor;
use tonic::{transport::Channel, Request};

pub struct GoogleSpannerClientBuilder;

impl GoogleSpannerClientBuilder {
    pub fn new() -> Self {
        Self {}
    }
}

pub type GoogleDatabaseAdminClient = DatabaseAdminClient<
    tonic::service::interceptor::InterceptedService<Channel, GoogleConnectorInterceptor>,
>;

impl CachedGoogleApiClientBuilder<GoogleDatabaseAdminClient> for GoogleSpannerClientBuilder {
    fn create_client(
        &self,
        channel: Channel,
        interceptor: GoogleConnectorInterceptor,
    ) -> GoogleDatabaseAdminClient {
        DatabaseAdminClient::with_interceptor(channel, interceptor)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let project = std::env::var("PROJECT")?;
    let instance = std::env::var("INSTANCE")?;

    let spanner_client_factory =
        CachedGoogleApiClient::new(GoogleSpannerClientBuilder::new(), "spanner.googleapis.com", None);

    let mut spanner_client = spanner_client_factory.get().await?;

    let response = spanner_client
        .list_databases(Request::new(ListDatabasesRequest {
            parent: format!("projects/{}/instances/{}", project, instance),
            page_size: 100,
            ..Default::default()
        }))
        .await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}
