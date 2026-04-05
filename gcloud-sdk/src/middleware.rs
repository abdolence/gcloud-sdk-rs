use crate::token_source::auth_token_generator::GoogleAuthTokenGenerator;
use chrono::Utc;
use futures::{Future, TryFutureExt};
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};
use tonic::client::GrpcService;
use tower::Service;
use tower_layer::Layer;
use tracing::*;

#[derive(Clone)]
pub struct GoogleAuthMiddlewareService<T>
where
    T: Clone,
{
    google_service: Option<T>,
    token_generator: Arc<GoogleAuthTokenGenerator>,
    cloud_resource_prefix: Option<String>,
    user_agent: String,
    x_goog_api_client: String,
}

impl<T> GoogleAuthMiddlewareService<T>
where
    T: Clone,
{
    pub fn new(
        service: T,
        token_generator: Arc<GoogleAuthTokenGenerator>,
        cloud_resource_prefix: Option<String>,
    ) -> GoogleAuthMiddlewareService<T> {
        GoogleAuthMiddlewareService {
            google_service: Some(service),
            token_generator,
            cloud_resource_prefix,
            user_agent: format!("gcloud-sdk-rs/{}", env!("CARGO_PKG_VERSION")),
            x_goog_api_client: format!("gcloud-sdk-rs/{}", env!("CARGO_PKG_VERSION")),
        }
    }

    pub fn set_user_agent(&mut self, user_agent: String) {
        self.user_agent = user_agent;
    }

    pub fn set_x_goog_api_client(&mut self, x_goog_api_client: String) {
        self.x_goog_api_client = x_goog_api_client;
    }

    pub fn append_user_agent(&mut self, user_agent: String) {
        self.user_agent = format!("{} {}", self.user_agent, user_agent);
    }

    pub fn append_x_goog_api_client(&mut self, x_goog_api_client: String) {
        self.x_goog_api_client = format!("{} {}", self.x_goog_api_client, x_goog_api_client);
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
        if let Some(ref mut google_service) = self.google_service.as_mut() {
            google_service.poll_ready(cx).map_err(|e| e.into())
        } else {
            Poll::Pending
        }
    }

    fn call(&mut self, mut req: hyper::Request<RequestBody>) -> Self::Future {
        let generator = self.token_generator.clone();
        let cloud_resource_prefix = self.cloud_resource_prefix.clone();
        let user_agent = self.user_agent.clone();
        let x_goog_api_client = self.x_goog_api_client.clone();

        if let Some(mut google_service) = self.google_service.take() {
            self.google_service = Some(google_service.clone());
            Box::pin(async move {
                let begin_time = Utc::now();
                let token = generator.create_token().await.map_err(Box::new)?;
                let token_generated_time = Utc::now();
                let headers = req.headers_mut();
                headers.insert("authorization", token.header_value().parse()?);
                if let Some(cloud_resource_prefix_value) = cloud_resource_prefix {
                    headers.insert(
                        "google-cloud-resource-prefix",
                        cloud_resource_prefix_value.parse()?,
                    );
                }
                headers.insert(hyper::header::USER_AGENT, user_agent.parse()?);
                headers.insert("x-goog-api-client", x_goog_api_client.parse()?);

                let req_uri_str = req.uri().to_string();
                google_service
                    .call(req)
                    .map_ok(|x| {
                        let finished_time = Utc::now();
                        debug!(
                            "OK: {} took {}ms (incl. token gen: {}ms)",
                            req_uri_str,
                            finished_time
                                .signed_duration_since(begin_time)
                                .num_milliseconds(),
                            token_generated_time
                                .signed_duration_since(begin_time)
                                .num_milliseconds()
                        );
                        x
                    })
                    .await
                    .map_err(|e| {
                        let finished_time = Utc::now();
                        error!(
                            "Err: {} took {}ms (incl. token gen: {}ms)",
                            req_uri_str,
                            finished_time
                                .signed_duration_since(begin_time)
                                .num_milliseconds(),
                            token_generated_time
                                .signed_duration_since(begin_time)
                                .num_milliseconds()
                        );
                        e.into()
                    })
            })
        } else {
            panic!("Should never happen, system error");
        }
    }
}

pub struct GoogleAuthMiddlewareLayer {
    token_generator: Arc<GoogleAuthTokenGenerator>,
    cloud_resource_prefix: Option<String>,
    user_agent: String,
    x_goog_api_client: String,
}

impl GoogleAuthMiddlewareLayer {
    pub fn new(
        token_generator: GoogleAuthTokenGenerator,
        cloud_resource_prefix: Option<String>,
    ) -> Self {
        GoogleAuthMiddlewareLayer {
            token_generator: Arc::new(token_generator),
            cloud_resource_prefix,
            user_agent: format!("gcloud-sdk-rs/{}", env!("CARGO_PKG_VERSION")),
            x_goog_api_client: format!("gcloud-sdk-rs/{}", env!("CARGO_PKG_VERSION")),
        }
    }

    pub fn amend_user_agent(mut self, user_agent: String) -> Self {
        self.user_agent = format!("{} {}", self.user_agent, user_agent);
        self
    }

    pub fn amend_x_goog_api_client(mut self, x_goog_api_client: String) -> Self {
        self.x_goog_api_client = format!("{} {}", self.x_goog_api_client, x_goog_api_client);
        self
    }
}

impl<S> Layer<S> for GoogleAuthMiddlewareLayer
where
    S: Clone,
{
    type Service = GoogleAuthMiddlewareService<S>;

    fn layer(&self, service: S) -> GoogleAuthMiddlewareService<S> {
        let mut middleware_service = GoogleAuthMiddlewareService::new(
            service,
            self.token_generator.clone(),
            self.cloud_resource_prefix.clone(),
        );
        middleware_service.set_user_agent(self.user_agent.clone());
        middleware_service.set_x_goog_api_client(self.x_goog_api_client.clone());
        middleware_service
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token_source::{Source, Token, TokenSourceType};
    use async_trait::async_trait;
    use hyper::{Request, Response};
    use secret_vault_value::SecretValue;
    use std::convert::Infallible;

    struct DummySource;

    #[async_trait]
    impl Source for DummySource {
        async fn token(&self) -> crate::error::Result<Token> {
            Ok(Token {
                token_type: "Bearer".to_string(),
                token: SecretValue::from("dummy-token"),
                expiry: Utc::now() + chrono::Duration::hours(1),
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
        let mut service = GoogleAuthMiddlewareService::new(dummy_service, Arc::new(token_generator), None);

        let req = Request::builder()
            .uri("http://example.com")
            .body("".to_string())
            .unwrap();

        tower::Service::call(&mut service, req).await.unwrap();

        let captured_req = rx.recv().await.unwrap();
        let expected_default = format!("gcloud-sdk-rs/{}", env!("CARGO_PKG_VERSION"));
        assert_eq!(
            captured_req.headers().get(hyper::header::USER_AGENT).unwrap(),
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
            .amend_user_agent("extra-ua".to_string())
            .amend_x_goog_api_client("extra-client".to_string());

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
            captured_req.headers().get(hyper::header::USER_AGENT).unwrap(),
            expected_ua.as_str()
        );
        assert_eq!(
            captured_req.headers().get("x-goog-api-client").unwrap(),
            expected_client.as_str()
        );
    }
}
