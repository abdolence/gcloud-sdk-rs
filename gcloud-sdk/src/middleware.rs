use crate::token_source::auth_token_generator::GoogleAuthTokenGenerator;
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
/// or sets it outright if absent, matching the historical "amend" behaviour.
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

/// Merges headers whose name survived the http crate's multi-value iteration
/// (i.e. `insert`, last value per name wins), matching historical behaviour.
fn merge_headers(target: &mut HeaderMap, additional_headers: HeaderMap) {
    for (maybe_name, value) in additional_headers {
        if let Some(name) = maybe_name {
            target.insert(name, value);
        }
    }
}

#[derive(Clone)]
pub struct GoogleAuthMiddlewareService<T> {
    inner: T,
    token_generator: Arc<GoogleAuthTokenGenerator>,
    /// Every header added to each request except `authorization`, already validated.
    headers: Arc<HeaderMap>,
}

impl<T> GoogleAuthMiddlewareService<T> {
    pub fn new(
        service: T,
        token_generator: Arc<GoogleAuthTokenGenerator>,
        cloud_resource_prefix: Option<String>,
    ) -> crate::error::Result<GoogleAuthMiddlewareService<T>> {
        Ok(GoogleAuthMiddlewareService {
            inner: service,
            token_generator,
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

    pub fn set_additional_headers(
        &mut self,
        additional_headers: HeaderMap,
    ) -> crate::error::Result<()> {
        merge_headers(Arc::make_mut(&mut self.headers), additional_headers);
        Ok(())
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
        let generator = Arc::clone(&self.token_generator);
        let headers = Arc::clone(&self.headers);

        // tower's documented idiom for a `Clone` inner service: the instance we already
        // polled ready goes into the future, and the service keeps a fresh clone.
        let clone = self.inner.clone();
        let mut inner = std::mem::replace(&mut self.inner, clone);

        Box::pin(async move {
            let begin_time = Timestamp::now();
            let authorization = generator.authorization_header().await.map_err(Box::new)?;
            let token_generated_time = Timestamp::now();

            let req_headers = req.headers_mut();
            req_headers.insert(hyper::header::AUTHORIZATION, authorization);
            for (name, value) in headers.iter() {
                req_headers.insert(name.clone(), value.clone());
            }

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
    token_generator: Arc<GoogleAuthTokenGenerator>,
    headers: Arc<HeaderMap>,
}

impl GoogleAuthMiddlewareLayer {
    pub fn new(
        token_generator: GoogleAuthTokenGenerator,
        cloud_resource_prefix: Option<String>,
    ) -> crate::error::Result<Self> {
        Ok(GoogleAuthMiddlewareLayer {
            token_generator: Arc::new(token_generator),
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

    pub fn set_additional_headers(
        &mut self,
        additional_headers: HeaderMap,
    ) -> crate::error::Result<()> {
        merge_headers(Arc::make_mut(&mut self.headers), additional_headers);
        Ok(())
    }
}

impl<S> Layer<S> for GoogleAuthMiddlewareLayer {
    type Service = GoogleAuthMiddlewareService<S>;

    fn layer(&self, service: S) -> GoogleAuthMiddlewareService<S> {
        GoogleAuthMiddlewareService {
            inner: service,
            token_generator: Arc::clone(&self.token_generator),
            headers: Arc::clone(&self.headers),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token_source::{Source, Token, TokenSourceType};
    use async_trait::async_trait;
    use hyper::{Request, Response};
    use jiff::{SignedDuration, Timestamp};
    use secret_vault_value::SecretValue;
    use std::convert::Infallible;

    struct DummySource;

    #[async_trait]
    impl Source for DummySource {
        async fn token(&self) -> crate::error::Result<Token> {
            Ok(Token {
                token_type: "Bearer".to_string(),
                token: SecretValue::from("dummy-token"),
                expiry: Timestamp::now() + SignedDuration::from_hours(1),
            })
        }
    }

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

    #[tokio::test]
    async fn test_headers_presence() {
        let token_generator = GoogleAuthTokenGenerator::new(
            TokenSourceType::ExternalSource(Box::new(DummySource)),
            vec![],
        )
        .await
        .unwrap();

        let (tx, mut rx) = tokio::sync::mpsc::channel(1);
        let dummy_service = DummyService { tx: Arc::new(tx) };
        let mut service =
            GoogleAuthMiddlewareService::new(dummy_service, Arc::new(token_generator), None)
                .unwrap();

        let req = Request::builder()
            .uri("http://example.com")
            .body("".to_string())
            .unwrap();

        tower::Service::call(&mut service, req).await.unwrap();

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
    async fn authorization_header_is_marked_sensitive() {
        let token_generator = GoogleAuthTokenGenerator::new(
            TokenSourceType::ExternalSource(Box::new(DummySource)),
            vec![],
        )
        .await
        .unwrap();

        let (tx, mut rx) = tokio::sync::mpsc::channel(1);
        let dummy_service = DummyService { tx: Arc::new(tx) };
        let mut service =
            GoogleAuthMiddlewareService::new(dummy_service, Arc::new(token_generator), None)
                .unwrap();

        let req = Request::builder()
            .uri("http://example.com")
            .body("".to_string())
            .unwrap();

        tower::Service::call(&mut service, req).await.unwrap();

        let captured_req = rx.recv().await.unwrap();
        assert!(captured_req
            .headers()
            .get("authorization")
            .unwrap()
            .is_sensitive());
    }

    #[tokio::test]
    async fn test_headers_amend() {
        let token_generator = GoogleAuthTokenGenerator::new(
            TokenSourceType::ExternalSource(Box::new(DummySource)),
            vec![],
        )
        .await
        .unwrap();

        let (tx, mut rx) = tokio::sync::mpsc::channel(1);
        let dummy_service = DummyService { tx: Arc::new(tx) };

        let layer = GoogleAuthMiddlewareLayer::new(token_generator, None)
            .unwrap()
            .amend_user_agent("extra-ua".to_string())
            .unwrap()
            .amend_x_goog_api_client("extra-client".to_string())
            .unwrap();

        let mut service = layer.layer(dummy_service);

        let req = Request::builder()
            .uri("http://example.com")
            .body("".to_string())
            .unwrap();

        tower::Service::call(&mut service, req).await.unwrap();

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
        let token_generator = GoogleAuthTokenGenerator::new(
            TokenSourceType::ExternalSource(Box::new(DummySource)),
            vec![],
        )
        .await
        .unwrap();

        let layer_result = GoogleAuthMiddlewareLayer::new(token_generator, None)
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
        let token_generator = GoogleAuthTokenGenerator::new(
            TokenSourceType::ExternalSource(Box::new(DummySource)),
            vec![],
        )
        .await
        .unwrap();

        let (tx, mut rx) = tokio::sync::mpsc::channel(1);
        let dummy_service = DummyService { tx: Arc::new(tx) };
        let base_service =
            GoogleAuthMiddlewareService::new(dummy_service, Arc::new(token_generator), None)
                .unwrap();

        let mut amended = base_service.clone();
        amended.append_user_agent("extra".to_string()).unwrap();

        let mut sibling = base_service.clone();

        let req = Request::builder()
            .uri("http://example.com")
            .body("".to_string())
            .unwrap();

        tower::Service::call(&mut sibling, req).await.unwrap();

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
        let token_generator = GoogleAuthTokenGenerator::new(
            TokenSourceType::ExternalSource(Box::new(DummySource)),
            vec![],
        )
        .await
        .unwrap();

        let (tx, mut rx) = tokio::sync::mpsc::channel(1);
        let dummy_service = DummyService { tx: Arc::new(tx) };
        let mut service =
            GoogleAuthMiddlewareService::new(dummy_service, Arc::new(token_generator), None)
                .unwrap();
        let mut test_headers = hyper::HeaderMap::new();
        test_headers.insert("x-test-header", "test-value".parse().unwrap());
        service.set_additional_headers(test_headers).unwrap();

        let req = Request::builder()
            .uri("http://example.com")
            .body("".to_string())
            .unwrap();

        tower::Service::call(&mut service, req).await.unwrap();

        let captured_req = rx.recv().await.unwrap();
        assert_eq!(
            captured_req.headers().get("x-test-header").unwrap(),
            "test-value"
        );
    }
}
