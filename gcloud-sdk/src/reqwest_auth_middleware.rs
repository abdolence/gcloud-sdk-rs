use std::sync::Arc;

use async_trait::async_trait;
use reqwest_middleware::{Middleware, Next};

use crate::GoogleAuthTokenGenerator;

/// A `reqwest-middleware` middleware that sets the `authorization` header of every
/// request to the current token of a [`GoogleAuthTokenGenerator`]: an ID token for a
/// generator over an [`IdTokenSource`](crate::IdTokenSource), or an access token for one
/// over Google credentials.
#[derive(Clone)]
pub struct GoogleAuthReqwestMiddleware {
    token_generator: Arc<GoogleAuthTokenGenerator>,
}

impl GoogleAuthReqwestMiddleware {
    pub fn new(token_generator: impl Into<Arc<GoogleAuthTokenGenerator>>) -> Self {
        Self {
            token_generator: token_generator.into(),
        }
    }
}

#[async_trait]
impl Middleware for GoogleAuthReqwestMiddleware {
    async fn handle(
        &self,
        mut req: reqwest::Request,
        extensions: &mut hyper::http::Extensions,
        next: Next<'_>,
    ) -> reqwest_middleware::Result<reqwest::Response> {
        let authorization = self
            .token_generator
            .authorization_header()
            .await
            .map_err(reqwest_middleware::Error::middleware)?;
        req.headers_mut()
            .insert(reqwest::header::AUTHORIZATION, authorization);
        next.run(req, extensions).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::{StubResponse, StubServer};
    use crate::token_source::{Source, Token};
    use jiff::{SignedDuration, Timestamp};
    use secret_vault_value::SecretValue;

    struct FixedToken;

    #[async_trait]
    impl Source for FixedToken {
        async fn token(&self) -> crate::error::Result<Token> {
            Ok(Token::new(
                "Bearer".to_string(),
                SecretValue::from("minted-id-token"),
                Timestamp::now() + SignedDuration::from_hours(1),
            ))
        }
    }

    #[tokio::test]
    async fn requests_carry_the_generator_token() {
        let service = StubServer::start(vec![StubResponse::json("200 OK", "{}")]).await;
        let client = reqwest_middleware::ClientBuilder::new(reqwest::Client::new())
            .with(GoogleAuthReqwestMiddleware::new(
                GoogleAuthTokenGenerator::from_source(Box::new(FixedToken) as crate::BoxSource),
            ))
            .build();

        let response = client
            .get(format!("{}/orders", service.url))
            .send()
            .await
            .unwrap();

        assert_eq!(response.status(), reqwest::StatusCode::OK);
        assert_eq!(
            service.received()[0].header("authorization"),
            Some("Bearer minted-id-token")
        );
    }
}
