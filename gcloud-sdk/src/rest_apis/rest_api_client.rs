use crate::{GoogleAuthTokenGenerator, TokenSourceType, GCP_DEFAULT_SCOPES};
use async_trait::async_trait;
use std::sync::Arc;

#[derive(Clone)]
pub struct GoogleRestApi {
    pub client: reqwest::Client,
    pub token_generator: Arc<GoogleAuthTokenGenerator>,
}

impl GoogleRestApi {
    pub async fn new() -> crate::error::Result<Self> {
        Self::with_token_source(TokenSourceType::Default, GCP_DEFAULT_SCOPES.clone()).await
    }

    pub async fn with_token_source(
        token_source_type: TokenSourceType,
        token_scopes: Vec<String>,
    ) -> crate::error::Result<Self> {
        let token_generator =
            GoogleAuthTokenGenerator::new(token_source_type, token_scopes).await?;

        Ok(Self {
            client: reqwest::Client::new(),
            token_generator: Arc::new(token_generator),
        })
    }

    pub async fn with_google_token<'a>(
        &self,
        request: reqwest::RequestBuilder,
    ) -> crate::error::Result<reqwest::RequestBuilder> {
        let token = self.token_generator.create_token().await?;
        Ok(request.header(reqwest::header::AUTHORIZATION, token.header_value()))
    }
}
