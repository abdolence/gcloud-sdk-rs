use crate::{GoogleAuthTokenGenerator, TokenSourceType, GCP_DEFAULT_SCOPES};
use async_trait::async_trait;
use hyper::Uri;
use reqwest::{IntoUrl, Method, Request, RequestBuilder};
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
        let client = reqwest::Client::new();
        Self::with_client_token_source(client, token_source_type, token_scopes).await
    }

    pub async fn with_client_token_source(
        client: reqwest::Client,
        token_source_type: TokenSourceType,
        token_scopes: Vec<String>,
    ) -> crate::error::Result<Self> {
        let token_generator =
            GoogleAuthTokenGenerator::new(token_source_type, token_scopes).await?;

        Ok(Self {
            client,
            token_generator: Arc::new(token_generator),
        })
    }

    pub async fn with_google_token<'a>(
        &self,
        request: RequestBuilder,
    ) -> crate::error::Result<RequestBuilder> {
        let authorization = self.token_generator.authorization_header().await?;
        Ok(request.header(reqwest::header::AUTHORIZATION, authorization))
    }

    /// A [`GoogleAuthReqwestMiddleware`](crate::GoogleAuthReqwestMiddleware) over this
    /// client's token generator, so requests sent through a `reqwest-middleware` client
    /// reuse the tokens this client has already minted.
    #[cfg(feature = "reqwest-middleware")]
    pub fn middleware(&self) -> crate::GoogleAuthReqwestMiddleware {
        crate::GoogleAuthReqwestMiddleware::new(self.token_generator.clone())
    }

    pub async fn get<U: IntoUrl>(&self, url: U) -> crate::error::Result<RequestBuilder> {
        self.with_google_token(self.client.request(Method::GET, url))
            .await
    }
    pub async fn post<U: IntoUrl>(&self, url: U) -> crate::error::Result<RequestBuilder> {
        self.with_google_token(self.client.request(Method::POST, url))
            .await
    }
    pub async fn put<U: IntoUrl>(&self, url: U) -> crate::error::Result<RequestBuilder> {
        self.with_google_token(self.client.request(Method::PUT, url))
            .await
    }
    pub async fn patch<U: IntoUrl>(&self, url: U) -> crate::error::Result<RequestBuilder> {
        self.with_google_token(self.client.request(Method::PATCH, url))
            .await
    }
    pub async fn delete<U: IntoUrl>(&self, url: U) -> crate::error::Result<RequestBuilder> {
        self.with_google_token(self.client.request(Method::DELETE, url))
            .await
    }
    pub async fn head<U: IntoUrl>(&self, url: U) -> crate::error::Result<RequestBuilder> {
        self.with_google_token(self.client.request(Method::HEAD, url))
            .await
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
        .filter_map(|(k, vo)| vo.map(|v| (k.to_string(), v.to_string())))
        .collect();

    let url: url::Url = url::Url::parse_with_params(url_str, url_query_params)
        .unwrap()
        .as_str()
        .parse()
        .unwrap();

    url.as_str().parse().unwrap()
}

#[cfg(all(test, feature = "reqwest-middleware"))]
mod tests {
    use super::*;
    use crate::test_support::{StubResponse, StubServer};
    use crate::token_source::{Source, Token};
    use jiff::{SignedDuration, Timestamp};
    use secret_vault_value::SecretValue;
    use std::sync::atomic::{AtomicUsize, Ordering};

    struct CountingSource {
        mints: Arc<AtomicUsize>,
    }

    #[async_trait]
    impl Source for CountingSource {
        async fn token(&self) -> crate::error::Result<Token> {
            self.mints.fetch_add(1, Ordering::SeqCst);
            Ok(Token::new(
                "Bearer".to_string(),
                SecretValue::from("shared-token"),
                Timestamp::now() + SignedDuration::from_hours(1),
            ))
        }
    }

    #[tokio::test]
    async fn middleware_shares_the_token_cache() {
        let mints = Arc::new(AtomicUsize::new(0));
        let api = GoogleRestApi {
            client: reqwest::Client::new(),
            token_generator: Arc::new(GoogleAuthTokenGenerator::from_source(Box::new(
                CountingSource {
                    mints: mints.clone(),
                },
            )
                as crate::BoxSource)),
        };
        let service = StubServer::start(vec![
            StubResponse::json("200 OK", "{}"),
            StubResponse::json("200 OK", "{}"),
        ])
        .await;
        let middleware_client = reqwest_middleware::ClientBuilder::new(reqwest::Client::new())
            .with(api.middleware())
            .build();

        api.get(format!("{}/rest", service.url))
            .await
            .unwrap()
            .send()
            .await
            .unwrap();
        middleware_client
            .get(format!("{}/middleware", service.url))
            .send()
            .await
            .unwrap();

        let received = service.received();
        assert_eq!(
            received[0].header("authorization"),
            Some("Bearer shared-token")
        );
        assert_eq!(
            received[1].header("authorization"),
            Some("Bearer shared-token")
        );
        assert_eq!(mints.load(Ordering::SeqCst), 1);
    }
}
