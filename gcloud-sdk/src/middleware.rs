use crate::GoogleAuthHeaders;
use futures::{Future, TryFutureExt};
use hyper::header::{HeaderMap, HeaderName, HeaderValue, USER_AGENT};
use jiff::Timestamp;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};
use tonic::client::GrpcService;
use tower::Service;
use tower_layer::Layer;
use tracing::*;

const X_GOOG_API_CLIENT: HeaderName = HeaderName::from_static("x-goog-api-client");
const GOOGLE_CLOUD_RESOURCE_PREFIX: HeaderName =
    HeaderName::from_static("google-cloud-resource-prefix");

fn default_headers(cloud_resource_prefix: Option<String>) -> crate::error::Result<HeaderMap> {
    let mut headers = HeaderMap::new();
    let default_agent =
        HeaderValue::from_static(concat!("gcloud-sdk-rs/", env!("CARGO_PKG_VERSION")));
    headers.insert(USER_AGENT, default_agent.clone());
    headers.insert(X_GOOG_API_CLIENT, default_agent);
    if let Some(prefix) = cloud_resource_prefix {
        headers.insert(
            GOOGLE_CLOUD_RESOURCE_PREFIX,
            HeaderValue::from_str(&prefix)?,
        );
    }
    Ok(headers)
}

/// Appends `extra` to whatever `name` currently holds in `headers` (space separated),
/// or sets it outright if absent.
fn append_header_value(
    headers: &HeaderMap,
    name: &HeaderName,
    extra: &str,
) -> crate::error::Result<HeaderValue> {
    let combined = match headers.get(name).and_then(|v| v.to_str().ok()) {
        Some(current) => format!("{current} {extra}"),
        None => extra.to_string(),
    };
    Ok(HeaderValue::from_str(&combined)?)
}

/// Sets every header of `source` on `target`, replacing all the values `target` holds
/// under each of its names.
///
/// `iter()` yields one pair per value, so the names are cleared first and the pairs then
/// appended: `insert`ing them one by one would keep only the last value of a repeated
/// name.
pub(crate) fn replace_headers(target: &mut HeaderMap, source: &HeaderMap) {
    for name in source.keys() {
        target.remove(name);
    }
    target.extend(
        source
            .iter()
            .map(|(name, value)| (name.clone(), value.clone())),
    );
}

#[derive(Clone)]
pub struct GoogleAuthMiddlewareService<T> {
    inner: T,
    auth_headers: Arc<GoogleAuthHeaders>,
    /// Every header added to each request except `authorization`, already validated.
    headers: Arc<HeaderMap>,
}

impl<T> GoogleAuthMiddlewareService<T> {
    pub fn new(
        service: T,
        auth_headers: impl Into<Arc<GoogleAuthHeaders>>,
        cloud_resource_prefix: Option<String>,
    ) -> crate::error::Result<GoogleAuthMiddlewareService<T>> {
        Ok(GoogleAuthMiddlewareService {
            inner: service,
            auth_headers: auth_headers.into(),
            headers: Arc::new(default_headers(cloud_resource_prefix)?),
        })
    }

    pub fn set_user_agent(&mut self, user_agent: String) -> crate::error::Result<()> {
        let value = HeaderValue::from_str(&user_agent)?;
        Arc::make_mut(&mut self.headers).insert(USER_AGENT, value);
        Ok(())
    }

    pub fn set_x_goog_api_client(&mut self, x_goog_api_client: String) -> crate::error::Result<()> {
        let value = HeaderValue::from_str(&x_goog_api_client)?;
        Arc::make_mut(&mut self.headers).insert(X_GOOG_API_CLIENT, value);
        Ok(())
    }

    pub fn set_cloud_resource_prefix(
        &mut self,
        cloud_resource_prefix: String,
    ) -> crate::error::Result<()> {
        let value = HeaderValue::from_str(&cloud_resource_prefix)?;
        Arc::make_mut(&mut self.headers).insert(GOOGLE_CLOUD_RESOURCE_PREFIX, value);
        Ok(())
    }

    pub fn append_user_agent(&mut self, user_agent: String) -> crate::error::Result<()> {
        let value = append_header_value(&self.headers, &USER_AGENT, &user_agent)?;
        Arc::make_mut(&mut self.headers).insert(USER_AGENT, value);
        Ok(())
    }

    pub fn append_x_goog_api_client(
        &mut self,
        x_goog_api_client: String,
    ) -> crate::error::Result<()> {
        let value = append_header_value(&self.headers, &X_GOOG_API_CLIENT, &x_goog_api_client)?;
        Arc::make_mut(&mut self.headers).insert(X_GOOG_API_CLIENT, value);
        Ok(())
    }

    pub fn set_additional_headers(&mut self, additional_headers: HeaderMap) {
        Arc::make_mut(&mut self.headers).extend(additional_headers);
    }

    /// Wraps `inner` in a middleware service that shares this one's [`GoogleAuthHeaders`](crate::GoogleAuthHeaders), so
    /// both fetch and refresh one token, and carries the same headers except
    /// `google-cloud-resource-prefix`, which is set to `cloud_resource_prefix` or left
    /// out when it is `None`.
    pub(crate) fn with_inner<U>(
        &self,
        inner: U,
        cloud_resource_prefix: Option<String>,
    ) -> crate::error::Result<GoogleAuthMiddlewareService<U>> {
        let mut headers = HeaderMap::clone(&self.headers);
        headers.remove(GOOGLE_CLOUD_RESOURCE_PREFIX);
        if let Some(prefix) = cloud_resource_prefix {
            headers.insert(
                GOOGLE_CLOUD_RESOURCE_PREFIX,
                HeaderValue::from_str(&prefix)?,
            );
        }
        Ok(GoogleAuthMiddlewareService {
            inner,
            auth_headers: Arc::clone(&self.auth_headers),
            headers: Arc::new(headers),
        })
    }
}

impl<T, RequestBody> Service<hyper::Request<RequestBody>> for GoogleAuthMiddlewareService<T>
where
    T: GrpcService<RequestBody> + Send + Clone + 'static,
    T::Future: 'static + Send,
    RequestBody: 'static + Send,
    T::ResponseBody: 'static + Send,
    T::Error: 'static + Send,
{
    type Response = hyper::Response<T::ResponseBody>;
    type Error = Box<dyn std::error::Error + Send + Sync + 'static>;
    type Future =
        Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + 'static>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx).map_err(Into::into)
    }

    fn call(&mut self, mut req: hyper::Request<RequestBody>) -> Self::Future {
        let auth_headers = Arc::clone(&self.auth_headers);
        let headers = Arc::clone(&self.headers);

        // tower's documented idiom for a `Clone` inner service: the instance we already
        // polled ready goes into the future, and the service keeps a fresh clone.
        let clone = self.inner.clone();
        let mut inner = std::mem::replace(&mut self.inner, clone);

        Box::pin(async move {
            let begin_time = Timestamp::now();
            let request_auth_headers = auth_headers.headers().await.map_err(Box::new)?;
            let token_generated_time = Timestamp::now();

            let req_headers = req.headers_mut();
            replace_headers(req_headers, &request_auth_headers);
            replace_headers(req_headers, &headers);

            let req_uri = req.uri().clone();
            inner
                .call(req)
                .map_ok(|x| {
                    let finished_time = Timestamp::now();
                    debug!(
                        %req_uri,
                        "OK: took {}ms (incl. token gen: {}ms)",
                        finished_time.duration_since(begin_time).as_millis(),
                        token_generated_time.duration_since(begin_time).as_millis()
                    );
                    x
                })
                .await
                .map_err(|e| {
                    let finished_time = Timestamp::now();
                    error!(
                        %req_uri,
                        "Err: took {}ms (incl. token gen: {}ms)",
                        finished_time.duration_since(begin_time).as_millis(),
                        token_generated_time.duration_since(begin_time).as_millis()
                    );
                    e.into()
                })
        })
    }
}

pub struct GoogleAuthMiddlewareLayer {
    auth_headers: Arc<GoogleAuthHeaders>,
    headers: Arc<HeaderMap>,
}

impl GoogleAuthMiddlewareLayer {
    pub fn new(
        auth_headers: impl Into<Arc<GoogleAuthHeaders>>,
        cloud_resource_prefix: Option<String>,
    ) -> crate::error::Result<Self> {
        Ok(GoogleAuthMiddlewareLayer {
            auth_headers: auth_headers.into(),
            headers: Arc::new(default_headers(cloud_resource_prefix)?),
        })
    }

    pub fn amend_user_agent(mut self, user_agent: String) -> crate::error::Result<Self> {
        let value = append_header_value(&self.headers, &USER_AGENT, &user_agent)?;
        Arc::make_mut(&mut self.headers).insert(USER_AGENT, value);
        Ok(self)
    }

    pub fn amend_x_goog_api_client(
        mut self,
        x_goog_api_client: String,
    ) -> crate::error::Result<Self> {
        let value = append_header_value(&self.headers, &X_GOOG_API_CLIENT, &x_goog_api_client)?;
        Arc::make_mut(&mut self.headers).insert(X_GOOG_API_CLIENT, value);
        Ok(self)
    }

    pub fn set_additional_headers(&mut self, additional_headers: HeaderMap) {
        Arc::make_mut(&mut self.headers).extend(additional_headers);
    }
}

impl<S> Layer<S> for GoogleAuthMiddlewareLayer {
    type Service = GoogleAuthMiddlewareService<S>;

    fn layer(&self, service: S) -> GoogleAuthMiddlewareService<S> {
        GoogleAuthMiddlewareService {
            inner: service,
            auth_headers: Arc::clone(&self.auth_headers),
            headers: Arc::clone(&self.headers),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::StubCredentials;
    use google_cloud_auth::credentials::Credentials;
    use hyper::{Request, Response};
    use std::convert::Infallible;

    #[derive(Clone)]
    struct DummyService {
        tx: Arc<tokio::sync::mpsc::Sender<Request<String>>>,
    }

    impl Service<Request<String>> for DummyService {
        type Response = Response<String>;
        type Error = Infallible;
        type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }

        fn call(&mut self, req: Request<String>) -> Self::Future {
            let tx = self.tx.clone();
            Box::pin(async move {
                tx.send(req).await.unwrap();
                Ok(Response::builder()
                    .status(200)
                    .body("".to_string())
                    .unwrap())
            })
        }
    }

    fn request() -> Request<String> {
        Request::builder()
            .uri("http://example.com")
            .body("".to_string())
            .unwrap()
    }

    #[tokio::test]
    async fn test_headers_presence() {
        let (tx, mut rx) = tokio::sync::mpsc::channel(1);
        let dummy_service = DummyService { tx: Arc::new(tx) };
        let mut service = GoogleAuthMiddlewareService::new(
            dummy_service,
            Arc::new(GoogleAuthHeaders::from(Credentials::from(
                StubCredentials::bearer("dummy-token"),
            ))),
            None,
        )
        .unwrap();

        tower::Service::call(&mut service, request()).await.unwrap();

        let captured_req = rx.recv().await.unwrap();
        let expected_default = format!("gcloud-sdk-rs/{}", env!("CARGO_PKG_VERSION"));
        assert_eq!(
            captured_req
                .headers()
                .get(hyper::header::USER_AGENT)
                .unwrap(),
            expected_default.as_str()
        );
        assert_eq!(
            captured_req.headers().get("x-goog-api-client").unwrap(),
            expected_default.as_str()
        );
        assert_eq!(
            captured_req.headers().get("authorization").unwrap(),
            "Bearer dummy-token"
        );
    }

    #[tokio::test]
    async fn every_credentials_header_is_forwarded() {
        let mut credentials_headers = HeaderMap::new();
        credentials_headers.insert(
            hyper::header::AUTHORIZATION,
            HeaderValue::from_static("Bearer billed-token"),
        );
        credentials_headers.insert(
            "x-goog-user-project",
            HeaderValue::from_static("billing-project"),
        );
        let credentials = StubCredentials::new(credentials_headers);

        let (tx, mut rx) = tokio::sync::mpsc::channel(1);
        let mut service = GoogleAuthMiddlewareService::new(
            DummyService { tx: Arc::new(tx) },
            Arc::new(GoogleAuthHeaders::from(Credentials::from(
                credentials.clone(),
            ))),
            None,
        )
        .unwrap();

        tower::Service::call(&mut service, request()).await.unwrap();

        let captured_req = rx.recv().await.unwrap();
        let authorization = captured_req.headers().get("authorization").unwrap();
        assert_eq!(authorization, "Bearer billed-token");
        assert!(authorization.is_sensitive());
        assert_eq!(
            captured_req.headers().get("x-goog-user-project"),
            Some(&HeaderValue::from_static("billing-project"))
        );
    }

    #[tokio::test]
    async fn headers_are_refreshed_when_the_entity_tag_changes() {
        let credentials = StubCredentials::bearer("first-token");
        let (tx, mut rx) = tokio::sync::mpsc::channel(3);
        let mut service = GoogleAuthMiddlewareService::new(
            DummyService { tx: Arc::new(tx) },
            Arc::new(GoogleAuthHeaders::from(Credentials::from(
                credentials.clone(),
            ))),
            None,
        )
        .unwrap();

        tower::Service::call(&mut service, request()).await.unwrap();
        tower::Service::call(&mut service, request()).await.unwrap();
        credentials.rotate_bearer("second-token");
        tower::Service::call(&mut service, request()).await.unwrap();

        let authorizations: Vec<String> = [
            rx.recv().await.unwrap(),
            rx.recv().await.unwrap(),
            rx.recv().await.unwrap(),
        ]
        .iter()
        .map(|req| req.headers()["authorization"].to_str().unwrap().to_string())
        .collect();
        assert_eq!(
            authorizations,
            [
                "Bearer first-token",
                "Bearer first-token",
                "Bearer second-token"
            ]
        );
        // The second request presented the first headers' tag and was served the cached
        // headers.
        assert_eq!(credentials.not_modified(), 1);
        assert_eq!(credentials.served(), 2);
    }

    #[tokio::test]
    async fn test_headers_amend() {
        let (tx, mut rx) = tokio::sync::mpsc::channel(1);
        let dummy_service = DummyService { tx: Arc::new(tx) };

        let layer = GoogleAuthMiddlewareLayer::new(
            GoogleAuthHeaders::from(Credentials::from(StubCredentials::bearer("dummy-token"))),
            None,
        )
        .unwrap()
        .amend_user_agent("extra-ua".to_string())
        .unwrap()
        .amend_x_goog_api_client("extra-client".to_string())
        .unwrap();

        let mut service = layer.layer(dummy_service);

        tower::Service::call(&mut service, request()).await.unwrap();

        let captured_req = rx.recv().await.unwrap();
        let expected_ua = format!("gcloud-sdk-rs/{} extra-ua", env!("CARGO_PKG_VERSION"));
        let expected_client = format!("gcloud-sdk-rs/{} extra-client", env!("CARGO_PKG_VERSION"));

        assert_eq!(
            captured_req
                .headers()
                .get(hyper::header::USER_AGENT)
                .unwrap(),
            expected_ua.as_str()
        );
        assert_eq!(
            captured_req.headers().get("x-goog-api-client").unwrap(),
            expected_client.as_str()
        );
    }

    #[tokio::test]
    async fn invalid_user_agent_is_rejected_at_setter() {
        let layer_result = GoogleAuthMiddlewareLayer::new(
            GoogleAuthHeaders::from(Credentials::from(StubCredentials::bearer("dummy-token"))),
            None,
        )
        .unwrap()
        .amend_user_agent("bad\nvalue".to_string());

        match layer_result {
            Err(e) => assert!(matches!(
                e.into_kind(),
                crate::error::ErrorKind::HeaderValue(_)
            )),
            Ok(_) => panic!("expected an invalid header value to be rejected"),
        }
    }

    #[tokio::test]
    async fn amended_clone_does_not_change_sibling() {
        let (tx, mut rx) = tokio::sync::mpsc::channel(1);
        let dummy_service = DummyService { tx: Arc::new(tx) };
        let base_service = GoogleAuthMiddlewareService::new(
            dummy_service,
            Arc::new(GoogleAuthHeaders::from(Credentials::from(
                StubCredentials::bearer("dummy-token"),
            ))),
            None,
        )
        .unwrap();

        let mut amended = base_service.clone();
        amended.append_user_agent("extra".to_string()).unwrap();

        let mut sibling = base_service.clone();

        tower::Service::call(&mut sibling, request()).await.unwrap();

        let captured_req = rx.recv().await.unwrap();
        let expected_default = format!("gcloud-sdk-rs/{}", env!("CARGO_PKG_VERSION"));
        assert_eq!(
            captured_req
                .headers()
                .get(hyper::header::USER_AGENT)
                .unwrap(),
            expected_default.as_str()
        );
    }

    #[tokio::test]
    async fn test_additional_headers() {
        let (tx, mut rx) = tokio::sync::mpsc::channel(1);
        let dummy_service = DummyService { tx: Arc::new(tx) };
        let mut service = GoogleAuthMiddlewareService::new(
            dummy_service,
            Arc::new(GoogleAuthHeaders::from(Credentials::from(
                StubCredentials::bearer("dummy-token"),
            ))),
            None,
        )
        .unwrap();
        let mut test_headers = hyper::HeaderMap::new();
        test_headers.insert("x-test-header", "test-value".parse().unwrap());
        service.set_additional_headers(test_headers);

        tower::Service::call(&mut service, request()).await.unwrap();

        let captured_req = rx.recv().await.unwrap();
        assert_eq!(
            captured_req.headers().get("x-test-header").unwrap(),
            "test-value"
        );
    }

    #[tokio::test]
    async fn additional_headers_keep_every_value_of_a_repeated_name() {
        let (tx, mut rx) = tokio::sync::mpsc::channel(1);
        let dummy_service = DummyService { tx: Arc::new(tx) };
        let mut service = GoogleAuthMiddlewareService::new(
            dummy_service,
            Arc::new(GoogleAuthHeaders::from(Credentials::from(
                StubCredentials::bearer("dummy-token"),
            ))),
            None,
        )
        .unwrap();

        let mut test_headers = hyper::HeaderMap::new();
        test_headers.append("x-multi", "first".parse().unwrap());
        test_headers.append("x-multi", "second".parse().unwrap());
        service.set_additional_headers(test_headers);

        tower::Service::call(&mut service, request()).await.unwrap();

        let captured_req = rx.recv().await.unwrap();
        let values: Vec<&str> = captured_req
            .headers()
            .get_all("x-multi")
            .iter()
            .map(|v| v.to_str().unwrap())
            .collect();
        assert_eq!(values, vec!["first", "second"]);
    }

    #[tokio::test]
    async fn with_inner_shares_the_token_and_replaces_the_resource_prefix() {
        let credentials = StubCredentials::bearer("counted-token");

        let (tx, mut rx) = tokio::sync::mpsc::channel(2);
        let mut extra = hyper::HeaderMap::new();
        extra.insert("x-test-header", "test-value".parse().unwrap());
        let mut layer = GoogleAuthMiddlewareLayer::new(
            GoogleAuthHeaders::from(Credentials::from(credentials.clone())),
            Some("projects/first".to_string()),
        )
        .unwrap()
        .amend_user_agent("extra-ua".to_string())
        .unwrap();
        layer.set_additional_headers(extra);
        let mut first = layer.layer(DummyService {
            tx: Arc::new(tx.clone()),
        });

        let (other_tx, mut other_rx) = tokio::sync::mpsc::channel(2);
        let other = DummyService {
            tx: Arc::new(other_tx),
        };
        let mut with_prefix = first
            .with_inner(other.clone(), Some("projects/second".to_string()))
            .unwrap();
        let mut without_prefix = first.with_inner(other, None).unwrap();

        for service in [&mut first, &mut with_prefix, &mut without_prefix] {
            tower::Service::call(service, request()).await.unwrap();
        }

        let first_req = rx.recv().await.unwrap();
        let with_prefix_req = other_rx.recv().await.unwrap();
        let without_prefix_req = other_rx.recv().await.unwrap();

        let expected_ua = format!("gcloud-sdk-rs/{} extra-ua", env!("CARGO_PKG_VERSION"));
        for req in [&first_req, &with_prefix_req, &without_prefix_req] {
            assert_eq!(
                req.headers().get(hyper::header::USER_AGENT).unwrap(),
                expected_ua.as_str()
            );
            assert_eq!(req.headers().get("x-test-header").unwrap(), "test-value");
            assert_eq!(
                req.headers().get("authorization").unwrap(),
                "Bearer counted-token"
            );
        }
        assert_eq!(
            first_req
                .headers()
                .get(GOOGLE_CLOUD_RESOURCE_PREFIX)
                .unwrap(),
            "projects/first"
        );
        assert_eq!(
            with_prefix_req
                .headers()
                .get(GOOGLE_CLOUD_RESOURCE_PREFIX)
                .unwrap(),
            "projects/second"
        );
        assert!(without_prefix_req
            .headers()
            .get(GOOGLE_CLOUD_RESOURCE_PREFIX)
            .is_none());
        // One `GoogleAuthHeaders` behind all three services: the headers were derived once and
        // then served from its cache.
        assert_eq!(credentials.served(), 1);
    }
}
