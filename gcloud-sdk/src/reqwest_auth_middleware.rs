use std::sync::Arc;

use async_trait::async_trait;
use reqwest_middleware::{Middleware, Next};

use crate::middleware::replace_headers;
use crate::GoogleAuthHeaders;

/// A `reqwest-middleware` middleware that sets the authentication headers of a
/// [`GoogleAuthHeaders`] on every request: `authorization` with an access token or
/// an ID token, and any other header the credentials add, such as `x-goog-user-project`.
#[derive(Clone)]
pub struct GoogleAuthReqwestMiddleware {
    auth_headers: Arc<GoogleAuthHeaders>,
}

impl GoogleAuthReqwestMiddleware {
    pub fn new(auth_headers: impl Into<Arc<GoogleAuthHeaders>>) -> Self {
        Self {
            auth_headers: auth_headers.into(),
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
        let auth_headers = self
            .auth_headers
            .headers()
            .await
            .map_err(reqwest_middleware::Error::middleware)?;
        replace_headers(req.headers_mut(), &auth_headers);
        next.run(req, extensions).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::StubCredentials;
    use google_cloud_auth::credentials::Credentials;
    use reqwest::header::{HeaderMap, HeaderValue};
    use std::sync::Mutex;

    /// Ends the chain with an empty `200 OK` instead of sending the request, keeping the
    /// headers it received.
    #[derive(Clone, Default)]
    struct CaptureHeaders {
        received: Arc<Mutex<Vec<HeaderMap>>>,
    }

    #[async_trait]
    impl Middleware for CaptureHeaders {
        async fn handle(
            &self,
            req: reqwest::Request,
            _extensions: &mut hyper::http::Extensions,
            _next: Next<'_>,
        ) -> reqwest_middleware::Result<reqwest::Response> {
            self.received.lock().unwrap().push(req.headers().clone());
            Ok(reqwest::Response::from(hyper::Response::new(
                Vec::<u8>::new(),
            )))
        }
    }

    #[tokio::test]
    async fn every_credentials_header_is_forwarded_and_authorization_is_sensitive() {
        let mut credentials_headers = HeaderMap::new();
        credentials_headers.insert(
            reqwest::header::AUTHORIZATION,
            HeaderValue::from_static("Bearer billed-token"),
        );
        credentials_headers.insert(
            "x-goog-user-project",
            HeaderValue::from_static("billing-project"),
        );
        let auth_headers =
            GoogleAuthHeaders::from(Credentials::from(StubCredentials::new(credentials_headers)));
        let capture = CaptureHeaders::default();
        let client = reqwest_middleware::ClientBuilder::new(reqwest::Client::new())
            .with(GoogleAuthReqwestMiddleware::new(auth_headers))
            .with(capture.clone())
            .build();

        client.get("http://orders.invalid/").send().await.unwrap();

        let received = capture.received.lock().unwrap();
        let authorization = &received[0][reqwest::header::AUTHORIZATION];
        assert_eq!(authorization, "Bearer billed-token");
        assert!(authorization.is_sensitive());
        assert_eq!(received[0]["x-goog-user-project"], "billing-project");
    }
}
