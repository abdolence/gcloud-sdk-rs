use std::ops::Add;
use std::sync::Arc;

use async_trait::async_trait;
use chrono::prelude::*;
use chrono::Duration;
use tokio::sync::RwLock;
use tonic::codegen::InterceptedService;
use tonic::transport::Channel;

use crate::google_connector_interceptor::GoogleConnectorInterceptor;
use crate::source::TokenSourceType;
use crate::source::*;

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
    pub async fn from_builder(
        builder: B,
        google_api_url: &'static str,
        max_duration: Duration,
        cloud_resource_prefix_meta: Option<String>,
    ) -> crate::error::Result<Self> {
        Self::with_token_source(
            TokenSourceType::Default,
            vec!["https://www.googleapis.com/auth/cloud-platform".into()],
            builder,
            google_api_url,
            max_duration,
            cloud_resource_prefix_meta,
        )
            .await
    }

    pub async fn with_token_source(
        token_source_type: TokenSourceType,
        token_scopes: Vec<String>,
        builder: B,
        google_api_url: &'static str,
        max_duration: Duration,
        cloud_resource_prefix_meta: Option<String>,
    ) -> crate::error::Result<Self> {
        debug!("Creating a new Google API client for {}. Max duration: {}. Scopes: {:?}", google_api_url, max_duration, token_scopes);

        let source: BoxSource = create_source(token_source_type, token_scopes).await?;

        Ok(Self {
            state: Arc::new(RwLock::new(None)),
            builder: builder,
            google_api_url: google_api_url,
            cloud_resource_prefix_meta: cloud_resource_prefix_meta,
            max_duration: max_duration,
            token_source: source,
        })
    }

    pub async fn get(&self) -> crate::error::Result<C> {
        let existing_state: Option<CachedGoogleApiClientState<C>> = {
            let read_state = self.state.read().await;
            read_state.clone()
        };

        let now = Utc::now();

        match existing_state {
            Some(state) if state.expired_at.lt(&now) => {
                Ok(state.client)
            }
            _ => {
                let domain_name = self.google_api_url.to_string().replace("https://", "");

                let tls_config =
                    GoogleConnectorInterceptor::init_google_services_channel_tls_config(
                        domain_name,
                    );

                let channel = GoogleConnectorInterceptor::init_google_services_channel(
                    self.google_api_url,
                    &tls_config,
                )
                    .await?;

                let interceptor = GoogleConnectorInterceptor::new(
                    &self.token_source,
                    self.cloud_resource_prefix_meta.clone(),
                )
                    .await?;

                let token_expiry_date = interceptor.expiry_date().min(now.add(self.max_duration));

                debug!("Creating a new instance of Google API client for {}. Expiry date: {}", self.google_api_url, token_expiry_date);

                let new_client = self.builder.create_client(channel, interceptor);
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
}

pub struct GoogleApiClientBuilderFunction<C> where C: Clone {
    f: fn(tonic::transport::Channel, GoogleConnectorInterceptor) -> C
}

impl<C> GoogleApiClientBuilder<C> for GoogleApiClientBuilderFunction<C> where C: Clone {
    fn create_client(&self, channel: Channel, interceptor: GoogleConnectorInterceptor) -> C {
        (self.f)(channel, interceptor)
    }
}

impl<C> GoogleApiClient<GoogleApiClientBuilderFunction<C>, C>
    where C: Clone,
{
    pub async fn from_function(builder_fn: fn(tonic::transport::Channel, GoogleConnectorInterceptor) -> C, google_api_url: &'static str,
                               max_duration: Duration,
                               cloud_resource_prefix_meta: Option<String>) -> crate::error::Result<Self> {
        let builder: GoogleApiClientBuilderFunction<C> = GoogleApiClientBuilderFunction {
            f: builder_fn
        };

        Self::from_builder(builder, google_api_url, max_duration, cloud_resource_prefix_meta).await
    }
}

pub type GoogleConnectorInterceptedService = InterceptedService<Channel, GoogleConnectorInterceptor>;

pub type GoogleApiClientFn<C> = GoogleApiClient<
    GoogleApiClientBuilderFunction<C>,
    C
>;
