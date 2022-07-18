use std::marker::PhantomData;
use std::time::Duration;

use crate::token_source::auth_token_generator::GoogleAuthTokenGenerator;
use async_trait::async_trait;
use tonic::transport::Channel;
use tower::ServiceBuilder;
use tracing::*;

use crate::middleware::{GoogleAuthMiddlewareLayer, GoogleAuthMiddlewareService};
use crate::token_source::*;

#[async_trait]
pub trait GoogleApiClientBuilder<C> {
    fn create_client(&self, channel: GoogleAuthMiddlewareService<Channel>) -> C;
}

pub struct GoogleApiClient<B, C>
where
    B: GoogleApiClientBuilder<C>,
    C: Clone,
{
    builder: B,
    service: GoogleAuthMiddlewareService<Channel>,
    _ph: PhantomData<C>,
}

impl<B, C> GoogleApiClient<B, C>
where
    B: GoogleApiClientBuilder<C>,
    C: Clone,
{
    pub async fn with_token_source(
        builder: B,
        google_api_url: &'static str,
        cloud_resource_prefix: Option<String>,
        token_source_type: TokenSourceType,
        token_scopes: Vec<String>,
    ) -> crate::error::Result<Self> {
        debug!(
            "Creating a new Google API client for {}. Scopes: {:?}",
            google_api_url, token_scopes
        );

        let channel = init_google_services_channel(google_api_url).await?;

        let token_generator =
            GoogleAuthTokenGenerator::new(token_source_type, token_scopes).await?;

        let middleware = GoogleAuthMiddlewareLayer::new(token_generator, cloud_resource_prefix);

        let service: GoogleAuthMiddlewareService<Channel> =
            ServiceBuilder::new().layer(middleware).service(channel);

        Ok(Self {
            builder,
            service,
            _ph: PhantomData::default(),
        })
    }

    pub fn get(&self) -> C {
        self.builder.create_client(self.service.clone())
    }

    pub async fn clear_cache(&self) {
        todo!()
    }
}

pub struct GoogleApiClientBuilderFunction<C>
where
    C: Clone,
{
    f: fn(GoogleAuthMiddlewareService<Channel>) -> C,
}

impl<C> GoogleApiClientBuilder<C> for GoogleApiClientBuilderFunction<C>
where
    C: Clone,
{
    fn create_client(&self, channel: GoogleAuthMiddlewareService<Channel>) -> C {
        (self.f)(channel)
    }
}

impl<C> GoogleApiClient<GoogleApiClientBuilderFunction<C>, C>
where
    C: Clone,
{
    pub async fn from_function(
        builder_fn: fn(GoogleAuthMiddlewareService<Channel>) -> C,
        google_api_url: &'static str,
        cloud_resource_prefix_meta: Option<String>,
    ) -> crate::error::Result<Self> {
        Self::from_function_with_scopes(
            builder_fn,
            google_api_url,
            cloud_resource_prefix_meta,
            vec!["https://www.googleapis.com/auth/cloud-platform".into()],
        )
        .await
    }

    pub async fn from_function_with_scopes(
        builder_fn: fn(GoogleAuthMiddlewareService<Channel>) -> C,
        google_api_url: &'static str,
        cloud_resource_prefix_meta: Option<String>,
        token_scopes: Vec<String>,
    ) -> crate::error::Result<Self> {
        let builder: GoogleApiClientBuilderFunction<C> =
            GoogleApiClientBuilderFunction { f: builder_fn };

        Self::with_token_source(
            builder,
            google_api_url,
            cloud_resource_prefix_meta,
            TokenSourceType::Default,
            token_scopes,
        )
        .await
    }
}

pub type GoogleAuthMiddleware = GoogleAuthMiddlewareService<Channel>;
pub type GoogleApi<C> = GoogleApiClient<GoogleApiClientBuilderFunction<C>, C>;

async fn init_google_services_channel(
    api_url: &'static str,
) -> Result<Channel, crate::error::Error> {
    let domain_name = api_url.to_string().replace("https://", "");
    let tls_config = init_google_services_channel_tls_config(domain_name);

    Ok(Channel::from_static(api_url)
        .tls_config(tls_config)?
        .connect_timeout(Duration::from_secs(30))
        .tcp_keepalive(Some(Duration::from_secs(5)))
        .keep_alive_timeout(Duration::from_secs(60))
        .http2_keep_alive_interval(Duration::from_secs(10))
        .keep_alive_while_idle(true)
        .connect()
        .await?)
}

fn init_google_services_channel_tls_config(
    domain_name: String,
) -> tonic::transport::ClientTlsConfig {
    tonic::transport::ClientTlsConfig::new()
        .ca_certificate(tonic::transport::Certificate::from_pem(
            crate::apis::CERTIFICATES,
        ))
        .domain_name(domain_name)
}
