use std::ops::Add;
use std::sync::Arc;

use async_trait::async_trait;
use chrono::prelude::*;
use chrono::Duration;
use tokio::sync::RwLock;
use tonic::codegen::InterceptedService;
use tonic::transport::Channel;

use crate::google_connector_interceptor::GoogleConnectorInterceptor;
use crate::token_source::TokenSourceType;
use crate::token_source::*;

use tracing::*;

#[async_trait]
pub trait GoogleApiClientBuilder<C> {
    fn create_client(&self, channel: Channel, interceptor: GoogleConnectorInterceptor) -> C;
}

pub struct GoogleApiClient<B, C>
where
    B: GoogleApiClientBuilder<C>,
    C: Clone,
{
    builder: B,
    google_api_url: &'static str,
    cloud_resource_prefix_meta: Option<String>,
    max_duration: chrono::Duration,
    state: Arc<RwLock<Option<CachedGoogleApiClientState<C>>>>,
    token_source: BoxSource,
    channel: Channel,
}

#[derive(Clone)]
struct CachedGoogleApiClientState<C>
where
    C: Clone,
{
    expired_at: DateTime<Utc>,
    client: C,
}

impl<B, C> GoogleApiClient<B, C>
where
    B: GoogleApiClientBuilder<C>,
    C: Clone,
{
    pub async fn with_token_source(
        builder: B,
        google_api_url: &'static str,
        max_duration: Duration,
        cloud_resource_prefix_meta: Option<String>,
        token_source_type: TokenSourceType,
        token_scopes: Vec<String>,
    ) -> crate::error::Result<Self> {
        debug!(
            "Creating a new Google API client for {}. Max duration: {}. Scopes: {:?}",
            google_api_url, max_duration, token_scopes
        );

        let source: BoxSource = create_source(token_source_type, token_scopes).await?;
        let channel = Self::init_google_services_channel(google_api_url).await?;

        Ok(Self {
            state: Arc::new(RwLock::new(None)),
            builder: builder,
            google_api_url: google_api_url,
            cloud_resource_prefix_meta: cloud_resource_prefix_meta,
            max_duration: max_duration,
            token_source: source,
            channel: channel,
        })
    }

    pub async fn get(&self) -> crate::error::Result<C> {
        let existing_state: Option<CachedGoogleApiClientState<C>> = {
            let read_state = self.state.read().await;
            read_state.clone()
        };

        let now = Utc::now();

        match existing_state {
            Some(state) if state.expired_at.gt(&now) => Ok(state.client),
            _ => {
                let interceptor = GoogleConnectorInterceptor::new(
                    &self.token_source,
                    self.cloud_resource_prefix_meta.clone(),
                )
                .await?;

                let token_expiry_date = interceptor.expiry_date().min(now.add(self.max_duration));

                debug!(
                    "Creating a new instance of Google API client for {}. Expiry date: {}",
                    self.google_api_url, token_expiry_date
                );

                let new_client = self
                    .builder
                    .create_client(self.channel.clone(), interceptor);
                {
                    let mut write_state = self.state.write().await;
                    *write_state = Some(CachedGoogleApiClientState {
                        client: new_client.clone(),
                        expired_at: token_expiry_date,
                    })
                }
                Ok(new_client)
            }
        }
    }

    pub async fn clear_cache(&self) {
        let mut write_state = self.state.write().await;
        *write_state = None;
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

    async fn init_google_services_channel(
        api_url: &'static str
    ) -> Result<Channel, crate::error::Error> {
        let domain_name = api_url.to_string().replace("https://", "");
        let tls_config = Self::init_google_services_channel_tls_config(domain_name);

        Ok(Channel::from_static(api_url)
            .tls_config(tls_config)?
            .connect()
            .await?)
    }
}

pub struct GoogleApiClientBuilderFunction<C>
where
    C: Clone,
{
    f: fn(tonic::transport::Channel, GoogleConnectorInterceptor) -> C,
}

impl<C> GoogleApiClientBuilder<C> for GoogleApiClientBuilderFunction<C>
where
    C: Clone,
{
    fn create_client(&self, channel: Channel, interceptor: GoogleConnectorInterceptor) -> C {
        (self.f)(channel, interceptor)
    }
}

impl<C> GoogleApiClient<GoogleApiClientBuilderFunction<C>, C>
where
    C: Clone,
{
    pub async fn from_function(
        builder_fn: fn(tonic::transport::Channel, GoogleConnectorInterceptor) -> C,
        google_api_url: &'static str,
        max_duration: Duration,
        cloud_resource_prefix_meta: Option<String>,
    ) -> crate::error::Result<Self> {
        Self::from_function_with_scopes(
            builder_fn,
            google_api_url,
            max_duration,
            cloud_resource_prefix_meta,
            vec!["https://www.googleapis.com/auth/cloud-platform".into()],
        )
        .await
    }

    pub async fn from_function_with_scopes(
        builder_fn: fn(tonic::transport::Channel, GoogleConnectorInterceptor) -> C,
        google_api_url: &'static str,
        max_duration: Duration,
        cloud_resource_prefix_meta: Option<String>,
        token_scopes: Vec<String>,
    ) -> crate::error::Result<Self> {
        let builder: GoogleApiClientBuilderFunction<C> =
            GoogleApiClientBuilderFunction { f: builder_fn };

        Self::with_token_source(
            builder,
            google_api_url,
            max_duration,
            cloud_resource_prefix_meta,
            TokenSourceType::Default,
            token_scopes,
        )
        .await
    }
}

pub type GoogleConnectorInterceptedService =
    InterceptedService<Channel, GoogleConnectorInterceptor>;

pub type GoogleApiClientFn<C> = GoogleApiClient<GoogleApiClientBuilderFunction<C>, C>;
