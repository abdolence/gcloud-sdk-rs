use std::marker::PhantomData;
use std::time::Duration;

use crate::token_source::auth_token_generator::GoogleAuthTokenGenerator;
use async_trait::async_trait;
use once_cell::sync::Lazy;
use tonic::transport::Channel;
use tower::ServiceBuilder;
use tracing::*;

use crate::middleware::{GoogleAuthMiddlewareLayer, GoogleAuthMiddlewareService};
use crate::token_source::credentials::CredentialsInfo;
use crate::token_source::*;

#[async_trait]
pub trait GoogleApiClientBuilder<C>
where
    C: Clone + Send,
{
    fn create_client(&self, channel: GoogleAuthMiddlewareService<Channel>) -> C;
}

#[derive(Clone)]
pub struct GoogleApiClient<B, C>
where
    B: GoogleApiClientBuilder<C>,
    C: Clone + Send,
{
    builder: B,
    service: GoogleAuthMiddlewareService<Channel>,
    _ph: PhantomData<C>,
}

impl<B, C> GoogleApiClient<B, C>
where
    B: GoogleApiClientBuilder<C>,
    C: Clone + Send,
{
    pub async fn with_token_source<S: AsRef<str>>(
        builder: B,
        google_api_url: S,
        cloud_resource_prefix: Option<String>,
        token_source_type: TokenSourceType,
        token_scopes: Vec<String>,
    ) -> crate::error::Result<Self> {
        Self::with_token_source_and_headers(
            builder,
            google_api_url,
            cloud_resource_prefix,
            token_source_type,
            token_scopes,
            hyper::HeaderMap::new(),
        )
        .await
    }

    pub async fn with_token_source_and_headers<S: AsRef<str>>(
        builder: B,
        google_api_url: S,
        cloud_resource_prefix: Option<String>,
        token_source_type: TokenSourceType,
        token_scopes: Vec<String>,
        additional_headers: hyper::HeaderMap,
    ) -> crate::error::Result<Self> {
        debug!(
            "Creating a new Google API client for {}. Scopes: {:?}",
            google_api_url.as_ref(),
            token_scopes
        );

        let token_generator =
            GoogleAuthTokenGenerator::new(token_source_type, token_scopes).await?;

        let mut middleware =
            GoogleAuthMiddlewareLayer::new(token_generator, cloud_resource_prefix)?;
        middleware.set_additional_headers(additional_headers);

        Self::with_token_source_and_middleware(builder, google_api_url, middleware).await
    }

    pub async fn with_token_source_and_middleware<S: AsRef<str>>(
        builder: B,
        google_api_url: S,
        middleware: GoogleAuthMiddlewareLayer,
    ) -> crate::error::Result<Self> {
        let channel = GoogleEnvironment::init_google_services_channel(google_api_url).await?;

        let service: GoogleAuthMiddlewareService<Channel> =
            ServiceBuilder::new().layer(middleware).service(channel);

        Ok(Self {
            builder,
            service,
            _ph: PhantomData,
        })
    }

    pub fn get(&self) -> C {
        self.builder.create_client(self.service.clone())
    }

    /// Builds a client of another API on this client's authenticated channel: same
    /// connection, token source and headers, for an API served from the same endpoint
    /// (Firestore's admin and long-running operations services, for example).
    pub fn get_with<C2>(&self, f: impl FnOnce(GoogleAuthMiddlewareService<Channel>) -> C2) -> C2 {
        f(self.service.clone())
    }

    /// Builds a client of an API served from another endpoint that reuses this client's
    /// authentication: one token generator, so the token is fetched and refreshed once for
    /// both clients (BigQuery and Cloud Storage, for example).
    ///
    /// Unlike [`get_with`](Self::get_with), which shares the channel and so reaches only
    /// the same host, this opens a new channel to `google_api_url`. The user agent,
    /// `x-goog-api-client` and additional headers are carried over;
    /// `google-cloud-resource-prefix` is not, since another API usually needs another
    /// prefix or none, and is set from `cloud_resource_prefix` instead.
    pub async fn connect_with_endpoint<C2, S: AsRef<str>>(
        &self,
        builder_fn: fn(GoogleAuthMiddlewareService<Channel>) -> C2,
        google_api_url: S,
        cloud_resource_prefix: Option<String>,
    ) -> crate::error::Result<GoogleApi<C2>>
    where
        C2: Clone + Send,
    {
        let channel = GoogleEnvironment::init_google_services_channel(google_api_url).await?;
        Ok(GoogleApiClient {
            builder: GoogleApiClientBuilderFunction { f: builder_fn },
            service: self.service.with_inner(channel, cloud_resource_prefix)?,
            _ph: PhantomData,
        })
    }

    pub fn amend_user_agent(mut self, user_agent: String) -> crate::error::Result<Self> {
        self.service.append_user_agent(user_agent)?;
        Ok(self)
    }

    pub fn amend_x_goog_api_client(
        mut self,
        x_goog_api_client: String,
    ) -> crate::error::Result<Self> {
        self.service.append_x_goog_api_client(x_goog_api_client)?;
        Ok(self)
    }
}

#[derive(Clone)]
pub struct GoogleApiClientBuilderFunction<C>
where
    C: Clone + Send,
{
    f: fn(GoogleAuthMiddlewareService<Channel>) -> C,
}

impl<C> GoogleApiClientBuilder<C> for GoogleApiClientBuilderFunction<C>
where
    C: Clone + Send,
{
    fn create_client(&self, channel: GoogleAuthMiddlewareService<Channel>) -> C {
        (self.f)(channel)
    }
}

impl<C> GoogleApiClient<GoogleApiClientBuilderFunction<C>, C>
where
    C: Clone + Send,
{
    pub async fn from_function<S: AsRef<str>>(
        builder_fn: fn(GoogleAuthMiddlewareService<Channel>) -> C,
        google_api_url: S,
        cloud_resource_prefix_meta: Option<String>,
    ) -> crate::error::Result<Self> {
        Self::from_function_with_scopes(
            builder_fn,
            google_api_url,
            cloud_resource_prefix_meta,
            GCP_DEFAULT_SCOPES.clone(),
        )
        .await
    }

    pub async fn from_function_with_headers<S: AsRef<str>>(
        builder_fn: fn(GoogleAuthMiddlewareService<Channel>) -> C,
        google_api_url: S,
        cloud_resource_prefix_meta: Option<String>,
        headers: hyper::HeaderMap,
    ) -> crate::error::Result<Self> {
        Self::from_function_with_scopes_and_headers(
            builder_fn,
            google_api_url,
            cloud_resource_prefix_meta,
            GCP_DEFAULT_SCOPES.clone(),
            headers,
        )
        .await
    }

    pub async fn from_function_with_scopes<S: AsRef<str>>(
        builder_fn: fn(GoogleAuthMiddlewareService<Channel>) -> C,
        google_api_url: S,
        cloud_resource_prefix_meta: Option<String>,
        token_scopes: Vec<String>,
    ) -> crate::error::Result<Self> {
        Self::from_function_with_token_source(
            builder_fn,
            google_api_url,
            cloud_resource_prefix_meta,
            token_scopes,
            TokenSourceType::Default,
        )
        .await
    }

    pub async fn from_function_with_scopes_and_headers<S: AsRef<str>>(
        builder_fn: fn(GoogleAuthMiddlewareService<Channel>) -> C,
        google_api_url: S,
        cloud_resource_prefix_meta: Option<String>,
        token_scopes: Vec<String>,
        headers: hyper::HeaderMap,
    ) -> crate::error::Result<Self> {
        Self::from_function_with_token_source_and_headers(
            builder_fn,
            google_api_url,
            cloud_resource_prefix_meta,
            token_scopes,
            TokenSourceType::Default,
            headers,
        )
        .await
    }

    pub async fn from_function_with_token_source<S: AsRef<str>>(
        builder_fn: fn(GoogleAuthMiddlewareService<Channel>) -> C,
        google_api_url: S,
        cloud_resource_prefix_meta: Option<String>,
        token_scopes: Vec<String>,
        token_source_type: TokenSourceType,
    ) -> crate::error::Result<Self> {
        let builder: GoogleApiClientBuilderFunction<C> =
            GoogleApiClientBuilderFunction { f: builder_fn };

        Self::with_token_source(
            builder,
            google_api_url,
            cloud_resource_prefix_meta,
            token_source_type,
            token_scopes,
        )
        .await
    }

    pub async fn from_function_with_token_source_and_headers<S: AsRef<str>>(
        builder_fn: fn(GoogleAuthMiddlewareService<Channel>) -> C,
        google_api_url: S,
        cloud_resource_prefix_meta: Option<String>,
        token_scopes: Vec<String>,
        token_source_type: TokenSourceType,
        headers: hyper::HeaderMap,
    ) -> crate::error::Result<Self> {
        let builder: GoogleApiClientBuilderFunction<C> =
            GoogleApiClientBuilderFunction { f: builder_fn };

        Self::with_token_source_and_headers(
            builder,
            google_api_url,
            cloud_resource_prefix_meta,
            token_source_type,
            token_scopes,
            headers,
        )
        .await
    }

    pub async fn from_function_with_middleware<S: AsRef<str>>(
        builder_fn: fn(GoogleAuthMiddlewareService<Channel>) -> C,
        google_api_url: S,
        middleware: GoogleAuthMiddlewareLayer,
    ) -> crate::error::Result<Self> {
        let builder: GoogleApiClientBuilderFunction<C> =
            GoogleApiClientBuilderFunction { f: builder_fn };

        Self::with_token_source_and_middleware(builder, google_api_url, middleware).await
    }
}

pub type GoogleAuthMiddleware = GoogleAuthMiddlewareService<Channel>;
pub type GoogleApi<C> = GoogleApiClient<GoogleApiClientBuilderFunction<C>, C>;

pub struct GoogleEnvironment;

impl GoogleEnvironment {
    pub async fn detect_google_project_id() -> Option<String> {
        let for_env = std::env::var("GCP_PROJECT")
            .ok()
            .or_else(|| std::env::var("PROJECT_ID").ok())
            .or_else(|| std::env::var("GCP_PROJECT_ID").ok());
        if for_env.is_some() {
            debug!("Detected GCP Project ID using environment variables");
            for_env
        } else {
            let local_creds = match crate::token_source::from_env_var(&GCP_DEFAULT_SCOPES) {
                Ok(Some(creds)) => Some(creds),
                Ok(None) | Err(_) => crate::token_source::from_well_known_file(&GCP_DEFAULT_SCOPES)
                    .ok()
                    .flatten(),
            };

            let local_quota_project_id =
                local_creds.and_then(|creds| creds.quota_project_id().map(ToString::to_string));

            if local_quota_project_id.is_some() {
                debug!("Detected default project id from local defined in quota_project_id for the service account file.");
                local_quota_project_id
            } else {
                let mut metadata_server =
                    crate::token_source::metadata::Metadata::new(GCP_DEFAULT_SCOPES.clone());
                if metadata_server.init().await {
                    let metadata_result = metadata_server.detect_google_project_id().await;
                    if metadata_result.is_some() {
                        debug!("Detected GCP Project ID using GKE metadata server");
                        metadata_result
                    } else {
                        debug!("No GCP Project ID detected in this environment. Please specify it explicitly using environment variables: `PROJECT_ID`,`GCP_PROJECT_ID`, or `GCP_PROJECT`");
                        metadata_result
                    }
                } else {
                    debug!("No GCP Project ID detected in this environment. Please specify it explicitly using environment variables: `PROJECT_ID`,`GCP_PROJECT_ID`, or `GCP_PROJECT`");
                    None
                }
            }
        }
    }

    pub async fn find_default_creds(
        token_scopes: &[String],
    ) -> crate::error::Result<Option<CredentialsInfo>> {
        debug!("Finding default credentials for scopes: {:?}", token_scopes);

        if let Some(src) = from_env_var(token_scopes)? {
            debug!("Creating credentials based on environment variable: GOOGLE_APPLICATION_CREDENTIALS");
            return Ok(src.to_credentials_info());
        }
        if let Some(src) = from_well_known_file(token_scopes)? {
            debug!("Creating credentials based on standard config files such as application_default_credentials.json");
            return Ok(src.to_credentials_info());
        }
        let mut metadata_server = crate::token_source::metadata::Metadata::new(token_scopes);
        if metadata_server.init().await {
            let metadata_result_email = metadata_server.email().await;
            if let Some(email) = metadata_result_email {
                debug!("Detected SA email using GKE metadata server");
                return Ok(Some(CredentialsInfo {
                    client_email: email,
                    project_id: metadata_server.detect_google_project_id().await,
                }));
            }
        }
        Ok(None)
    }

    pub async fn init_google_services_channel<S: AsRef<str>>(
        api_url: S,
    ) -> Result<Channel, crate::error::Error> {
        let api_url_string = api_url.as_ref().to_string();
        let base_config = Channel::from_shared(api_url_string.clone())?
            .connect_timeout(Duration::from_secs(30))
            .tcp_keepalive(Some(Duration::from_secs(60)))
            .keep_alive_timeout(Duration::from_secs(60))
            .http2_keep_alive_interval(Duration::from_secs(60))
            .keep_alive_while_idle(true);

        let config = if let Some(domain_name) = tls_server_name(&api_url_string)? {
            let tls_config = Self::init_tls_config(domain_name);
            base_config.tls_config(tls_config)?
        } else {
            base_config
        };

        Ok(config.connect().await?)
    }

    #[cfg(not(any(feature = "tls-roots", feature = "tls-webpki-roots")))]
    fn init_tls_config(domain_name: String) -> tonic::transport::ClientTlsConfig {
        tonic::transport::ClientTlsConfig::new()
            .ca_certificate(tonic::transport::Certificate::from_pem(
                crate::apis::CERTIFICATES,
            ))
            .domain_name(domain_name)
    }

    #[cfg(feature = "tls-roots")]
    fn init_tls_config(domain_name: String) -> tonic::transport::ClientTlsConfig {
        tonic::transport::ClientTlsConfig::new()
            .with_native_roots()
            .domain_name(domain_name)
    }

    #[cfg(all(feature = "tls-webpki-roots", not(feature = "tls-roots")))]
    fn init_tls_config(domain_name: String) -> tonic::transport::ClientTlsConfig {
        tonic::transport::ClientTlsConfig::new()
            .with_webpki_roots()
            .domain_name(domain_name)
    }
}

/// The TLS server name to verify for a gRPC API URL.
///
/// Returns the URL's host, without port or path, for an `https` URL, and `None`
/// for a plain `http` URL, which connects without TLS. Any other scheme, a URL
/// without a scheme or host, or an unparseable URL is an error.
fn tls_server_name(api_url: &str) -> Result<Option<String>, crate::error::Error> {
    let uri: hyper::http::Uri = api_url.parse()?;
    let invalid = |reason: &str| {
        crate::error::Error::from(crate::error::ErrorKind::InvalidApiUrl(format!(
            "{reason}: {api_url}"
        )))
    };
    match uri.scheme_str() {
        Some("https") => {
            let host = uri
                .host()
                .filter(|host| !host.is_empty())
                .ok_or_else(|| invalid("missing host"))?;
            let host = host
                .strip_prefix('[')
                .and_then(|h| h.strip_suffix(']'))
                .unwrap_or(host);
            Ok(Some(host.to_string()))
        }
        Some("http") => Ok(None),
        Some(_) => Err(invalid("unsupported scheme")),
        None => Err(invalid("missing scheme")),
    }
}

pub static GCP_DEFAULT_SCOPES: Lazy<Vec<String>> =
    Lazy::new(|| vec!["https://www.googleapis.com/auth/cloud-platform".into()]);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tls_server_name_is_the_host_of_a_plain_https_url() {
        assert_eq!(
            tls_server_name("https://bigquery.googleapis.com").unwrap(),
            Some("bigquery.googleapis.com".to_string())
        );
    }

    #[test]
    fn tls_server_name_ignores_a_trailing_slash() {
        assert_eq!(
            tls_server_name("https://bigquery.googleapis.com/").unwrap(),
            Some("bigquery.googleapis.com".to_string())
        );
    }

    #[test]
    fn tls_server_name_ignores_the_path() {
        assert_eq!(
            tls_server_name("https://bigquery.googleapis.com/bigquery/v2").unwrap(),
            Some("bigquery.googleapis.com".to_string())
        );
    }

    #[test]
    fn tls_server_name_ignores_an_explicit_port() {
        assert_eq!(
            tls_server_name("https://bigquery.googleapis.com:443").unwrap(),
            Some("bigquery.googleapis.com".to_string())
        );
    }

    #[test]
    fn plain_http_url_uses_no_tls() {
        assert_eq!(tls_server_name("http://localhost:8080").unwrap(), None);
        assert_eq!(tls_server_name("http://localhost:8080/").unwrap(), None);
    }

    #[test]
    fn url_without_scheme_or_host_is_an_error() {
        assert!(tls_server_name("bigquery.googleapis.com").is_err());
        assert!(tls_server_name("/bigquery/v2").is_err());
        assert!(tls_server_name("https://").is_err());
        assert!(tls_server_name("not a url").is_err());
        assert!(tls_server_name("").is_err());
    }
    use crate::token_source::{Source, Token, TokenSourceType};
    use secret_vault_value::SecretValue;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;

    struct CountingSource {
        calls: Arc<AtomicUsize>,
    }

    #[async_trait]
    impl Source for CountingSource {
        async fn token(&self) -> crate::error::Result<Token> {
            self.calls.fetch_add(1, Ordering::SeqCst);
            Ok(Token {
                token_type: "Bearer".to_string(),
                token: SecretValue::from("counted-token"),
                expiry: jiff::Timestamp::now() + jiff::SignedDuration::from_hours(1),
            })
        }
    }

    // Neither address is dialed: the middleware fetches the token before handing the
    // request to the channel, and a closed loopback port fails the connection quickly
    // without ever reaching the network.
    async fn probe(service: &mut GoogleAuthMiddlewareService<Channel>) {
        let req = hyper::Request::builder()
            .uri("http://127.0.0.1:1/")
            .body(tonic::body::Body::empty())
            .unwrap();
        let ready = tower::ServiceExt::ready(service).await.unwrap();
        let _ = tower::Service::call(ready, req).await;
    }

    #[tokio::test]
    async fn get_with_shares_the_authenticated_channel() {
        let calls = Arc::new(AtomicUsize::new(0));
        let token_generator = GoogleAuthTokenGenerator::new(
            TokenSourceType::ExternalSource(Box::new(CountingSource {
                calls: calls.clone(),
            })),
            vec![],
        )
        .await
        .unwrap();
        let middleware = GoogleAuthMiddlewareLayer::new(token_generator, None).unwrap();
        let channel = Channel::from_static("http://127.0.0.1:1").connect_lazy();
        let service: GoogleAuthMiddlewareService<Channel> =
            ServiceBuilder::new().layer(middleware).service(channel);

        let client: GoogleApiClient<
            GoogleApiClientBuilderFunction<GoogleAuthMiddlewareService<Channel>>,
            GoogleAuthMiddlewareService<Channel>,
        > = GoogleApiClient {
            builder: GoogleApiClientBuilderFunction { f: |svc| svc },
            service,
            _ph: PhantomData,
        };

        let mut from_get = client.get();
        let mut from_get_with = client.get_with(|svc| svc);

        probe(&mut from_get).await;
        probe(&mut from_get_with).await;

        // A client built with `get_with` reuses the same `Arc<GoogleAuthTokenGenerator>`
        // as one built with `get`, so its token cache is shared: the source is asked for
        // a token once, not once per client.
        assert_eq!(calls.load(Ordering::SeqCst), 1);
    }

    // A gRPC endpoint that accepts connections and never answers, counting the
    // connections it received: the channel's eager connect succeeds against it, and a
    // request sent to it reaches the middleware's token fetch before it stalls.
    async fn silent_endpoint() -> (String, Arc<AtomicUsize>) {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let url = format!("http://{}", listener.local_addr().unwrap());
        let connections = Arc::new(AtomicUsize::new(0));
        let counter = connections.clone();
        tokio::spawn(async move {
            let mut held = Vec::new();
            while let Ok((socket, _)) = listener.accept().await {
                counter.fetch_add(1, Ordering::SeqCst);
                held.push(socket);
            }
        });
        (url, connections)
    }

    async fn send_stalled(mut service: GoogleAuthMiddlewareService<Channel>) {
        let _ = tokio::time::timeout(Duration::from_millis(200), probe(&mut service)).await;
    }

    #[tokio::test]
    async fn connect_with_endpoint_shares_auth_with_a_client_on_another_host() {
        let calls = Arc::new(AtomicUsize::new(0));
        let (first_url, first_connections) = silent_endpoint().await;
        let (second_url, second_connections) = silent_endpoint().await;

        let first: GoogleApi<GoogleAuthMiddlewareService<Channel>> =
            GoogleApi::from_function_with_token_source(
                |svc| svc,
                first_url,
                None,
                vec![],
                TokenSourceType::ExternalSource(Box::new(CountingSource {
                    calls: calls.clone(),
                })),
            )
            .await
            .unwrap();
        let second = first
            .connect_with_endpoint(|svc| svc, second_url, None)
            .await
            .unwrap();

        send_stalled(first.get()).await;
        send_stalled(second.get()).await;

        assert_eq!(first_connections.load(Ordering::SeqCst), 1);
        assert_eq!(second_connections.load(Ordering::SeqCst), 1);
        assert_eq!(calls.load(Ordering::SeqCst), 1);
    }
}
