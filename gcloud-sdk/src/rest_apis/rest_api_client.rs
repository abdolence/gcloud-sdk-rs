use crate::{GoogleAuthTokenGenerator, TokenSourceType, GCP_DEFAULT_SCOPES};
use async_trait::async_trait;
use hyper::Uri;
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

pub fn create_hyper_uri_with_params<'p, PT, TS>(url_str: &str, params: &'p PT) -> Uri
where
    PT: std::iter::IntoIterator<Item = (&'p str, Option<&'p TS>)> + Clone,
    TS: std::string::ToString + 'p,
{
    let url_query_params: Vec<(String, String)> = params
        .clone()
        .into_iter()
        .map(|(k, vo)| vo.map(|v| (k.to_string(), v.to_string())))
        .flatten()
        .collect();

    let url: url::Url = url::Url::parse_with_params(url_str, url_query_params)
        .unwrap()
        .as_str()
        .parse()
        .unwrap();

    url.as_str().parse().unwrap()
}
