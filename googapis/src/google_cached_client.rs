use crate::google_tonic_connector::GoogleConnectorInterceptor;
use async_trait::async_trait;
use chrono::prelude::*;
use std::sync::{Arc, RwLock};
use tonic::transport::Channel;

#[async_trait]
pub trait CachedGoogleApiClientBuilder<C> {
    fn create_client(&self, channel: Channel, interceptor: GoogleConnectorInterceptor) -> C;
}

pub struct CachedGoogleApiClient<B, C>
where
    B: CachedGoogleApiClientBuilder<C>,
    C: Clone,
{
    builder: B,
    google_api_url: &'static str,
    state: Arc<RwLock<Option<CachedGoogleApiClientState<C>>>>,
}

#[derive(Clone)]
struct CachedGoogleApiClientState<C>
where
    C: Clone,
{
    cached_at: DateTime<Utc>,
    client: C,
}

impl<B, C> CachedGoogleApiClient<B, C>
where
    B: CachedGoogleApiClientBuilder<C>,
    C: Clone,
{
    pub fn new(builder: B, google_api_url: &'static str) -> Self {
        Self {
            state: Arc::new(RwLock::new(None)),
            builder: builder,
            google_api_url: google_api_url,
        }
    }

    pub async fn get(&self) -> crate::Result<C> {
        let existing_state: Option<CachedGoogleApiClientState<C>> = {
            let read_state = self.state.read().unwrap();
            read_state.clone()
        };

        let now = Utc::now();

        match existing_state {
            Some(state) if now.signed_duration_since(state.cached_at).num_minutes() < 15 => {
                Ok(state.client)
            }
            _ => {
                let tls_config =
                    GoogleConnectorInterceptor::init_google_services_channel_tls_config(
                        self.google_api_url.into(),
                    );

                let channel = GoogleConnectorInterceptor::init_google_services_channel(
                    self.google_api_url.as_ref(),
                    &tls_config,
                )
                .await?;

                let interceptor = GoogleConnectorInterceptor::new().await?;

                let new_client = self.builder.create_client(channel, interceptor);
                {
                    let mut write_state = self.state.write().unwrap();
                    *write_state = Some(CachedGoogleApiClientState {
                        cached_at: now,
                        client: new_client.clone(),
                    })
                }
                Ok(new_client)
            }
        }
    }
}
