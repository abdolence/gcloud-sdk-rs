use std::marker::PhantomData;
use std::time::Duration;

use async_trait::async_trait;
use google_cloud_auth::credentials::Credentials;
use once_cell::sync::Lazy;
use tonic::transport::Channel;
use tower::ServiceBuilder;
use tracing::*;

use crate::adc::AdcFile;
use crate::metadata::MetadataServer;
use crate::middleware::{GoogleAuthMiddlewareLayer, GoogleAuthMiddlewareService};
use crate::GoogleAuthHeaders;

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
    pub async fn with_credentials<S: AsRef<str>>(
        builder: B,
        google_api_url: S,
        cloud_resource_prefix: Option<String>,
        credentials: Credentials,
    ) -> crate::error::Result<Self> {
        Self::with_credentials_and_headers(
            builder,
            google_api_url,
            cloud_resource_prefix,
            credentials,
            hyper::HeaderMap::new(),
        )
        .await
    }

    pub async fn with_credentials_and_headers<S: AsRef<str>>(
        builder: B,
        google_api_url: S,
        cloud_resource_prefix: Option<String>,
        credentials: Credentials,
        additional_headers: hyper::HeaderMap,
    ) -> crate::error::Result<Self> {
        debug!(
            "Creating a new Google API client for {}",
            google_api_url.as_ref()
        );

        let mut middleware = GoogleAuthMiddlewareLayer::new(
            GoogleAuthHeaders::from(credentials),
            cloud_resource_prefix,
        )?;
        middleware.set_additional_headers(additional_headers);

        Self::with_middleware(builder, google_api_url, middleware).await
    }

    pub async fn with_middleware<S: AsRef<str>>(
        builder: B,
        google_api_url: S,
        middleware: GoogleAuthMiddlewareLayer,
    ) -> crate::error::Result<Self> {
        let channel = GoogleEnvironment::init_google_services_channel(google_api_url).await?;
        Ok(Self::on_channel(builder, channel, middleware))
    }

    fn on_channel(builder: B, channel: Channel, middleware: GoogleAuthMiddlewareLayer) -> Self {
        Self {
            builder,
            service: ServiceBuilder::new().layer(middleware).service(channel),
            _ph: PhantomData,
        }
    }

    pub fn get(&self) -> C {
        self.builder.create_client(self.service.clone())
    }

    /// Builds a client of another API on this client's authenticated channel: same
    /// connection, authentication and headers, for an API served from the same endpoint
    /// (Firestore's admin and long-running operations services, for example).
    pub fn get_with<C2>(&self, f: impl FnOnce(GoogleAuthMiddlewareService<Channel>) -> C2) -> C2 {
        f(self.service.clone())
    }

    /// Builds a client of an API served from another endpoint that reuses this client's
    /// authentication: one [`GoogleAuthHeaders`], so the token is fetched and refreshed once for
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
    /// A client that authenticates with the Application Default Credentials, for the
    /// `cloud-platform` scope.
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

    /// A client that authenticates with the Application Default Credentials, for
    /// `token_scopes`.
    ///
    /// Fails with [`ErrorKind::CryptoProviderMissing`](crate::error::ErrorKind::CryptoProviderMissing)
    /// for a service account key, used directly or as the source of an impersonation,
    /// when the `auth-default-crypto` feature is off and no rustls crypto provider is
    /// installed.
    pub async fn from_function_with_scopes<S: AsRef<str>>(
        builder_fn: fn(GoogleAuthMiddlewareService<Channel>) -> C,
        google_api_url: S,
        cloud_resource_prefix_meta: Option<String>,
        token_scopes: Vec<String>,
    ) -> crate::error::Result<Self> {
        Self::from_function_with_scopes_and_headers(
            builder_fn,
            google_api_url,
            cloud_resource_prefix_meta,
            token_scopes,
            hyper::HeaderMap::new(),
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
        // The channel comes first: with a single rustls provider compiled in, its TLS
        // setup installs that provider, which the credentials may need to sign with.
        let channel = GoogleEnvironment::init_google_services_channel(google_api_url).await?;
        let mut middleware = GoogleAuthMiddlewareLayer::new(
            GoogleAuthHeaders::access_tokens_from_adc(token_scopes)?,
            cloud_resource_prefix_meta,
        )?;
        middleware.set_additional_headers(headers);
        Ok(Self::on_channel(
            GoogleApiClientBuilderFunction { f: builder_fn },
            channel,
            middleware,
        ))
    }

    /// A client that authenticates with `credentials`, built with
    /// [`google_cloud_auth`](crate::google_cloud_auth): a service account key, impersonation,
    /// workload identity federation, or a
    /// [`CredentialsProvider`](google_cloud_auth::credentials::CredentialsProvider) of
    /// your own.
    ///
    /// `credentials` built from a service account key need a rustls crypto provider when
    /// the `auth-default-crypto` feature is off. They are opaque here, so this
    /// constructor cannot check for one.
    pub async fn from_function_with_credentials<S: AsRef<str>>(
        builder_fn: fn(GoogleAuthMiddlewareService<Channel>) -> C,
        google_api_url: S,
        cloud_resource_prefix_meta: Option<String>,
        credentials: Credentials,
    ) -> crate::error::Result<Self> {
        Self::from_function_with_credentials_and_headers(
            builder_fn,
            google_api_url,
            cloud_resource_prefix_meta,
            credentials,
            hyper::HeaderMap::new(),
        )
        .await
    }

    pub async fn from_function_with_credentials_and_headers<S: AsRef<str>>(
        builder_fn: fn(GoogleAuthMiddlewareService<Channel>) -> C,
        google_api_url: S,
        cloud_resource_prefix_meta: Option<String>,
        credentials: Credentials,
        headers: hyper::HeaderMap,
    ) -> crate::error::Result<Self> {
        Self::with_credentials_and_headers(
            GoogleApiClientBuilderFunction { f: builder_fn },
            google_api_url,
            cloud_resource_prefix_meta,
            credentials,
            headers,
        )
        .await
    }

    pub async fn from_function_with_middleware<S: AsRef<str>>(
        builder_fn: fn(GoogleAuthMiddlewareService<Channel>) -> C,
        google_api_url: S,
        middleware: GoogleAuthMiddlewareLayer,
    ) -> crate::error::Result<Self> {
        Self::with_middleware(
            GoogleApiClientBuilderFunction { f: builder_fn },
            google_api_url,
            middleware,
        )
        .await
    }
}

pub type GoogleAuthMiddleware = GoogleAuthMiddlewareService<Channel>;
pub type GoogleApi<C> = GoogleApiClient<GoogleApiClientBuilderFunction<C>, C>;

pub struct GoogleEnvironment;

impl GoogleEnvironment {
    /// The Google Cloud project of this environment: the first of the `GCP_PROJECT`,
    /// `PROJECT_ID`, `GCP_PROJECT_ID` and `GOOGLE_CLOUD_PROJECT` environment variables
    /// that is set, else the project the Application Default Credentials file names, else
    /// the project of the metadata server.
    pub async fn detect_google_project_id() -> Option<String> {
        Self::detect_project_id_in(
            |name| std::env::var(name).ok(),
            AdcFile::load(),
            MetadataServer::from_env(),
        )
        .await
    }

    async fn detect_project_id_in(
        env: impl Fn(&str) -> Option<String>,
        adc: Option<AdcFile>,
        metadata: MetadataServer,
    ) -> Option<String> {
        if let Some(project_id) = PROJECT_ID_ENV_VARS.into_iter().find_map(env) {
            debug!("Detected GCP Project ID using environment variables");
            return Some(project_id);
        }
        if let Some(project_id) = adc.and_then(|adc| adc.project_id().map(ToString::to_string)) {
            debug!("Detected GCP Project ID in the Application Default Credentials file");
            return Some(project_id);
        }
        let project_id = metadata.project_id().await;
        if project_id.is_some() {
            debug!("Detected GCP Project ID using the metadata server");
        } else {
            debug!(
                env = ?PROJECT_ID_ENV_VARS,
                "No GCP Project ID detected in this environment; set one of the environment variables"
            );
        }
        project_id
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

/// The environment variables that name the project, in the order they are read.
const PROJECT_ID_ENV_VARS: [&str; 4] = [
    "GCP_PROJECT",
    "PROJECT_ID",
    "GCP_PROJECT_ID",
    "GOOGLE_CLOUD_PROJECT",
];

pub static GCP_DEFAULT_SCOPES: Lazy<Vec<String>> =
    Lazy::new(|| vec!["https://www.googleapis.com/auth/cloud-platform".into()]);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::{StubCredentials, StubResponse, StubServer};
    use serde_json::json;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;

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
        let credentials = StubCredentials::bearer("counted-token");
        let middleware = GoogleAuthMiddlewareLayer::new(
            GoogleAuthHeaders::from(Credentials::from(credentials.clone())),
            None,
        )
        .unwrap();
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

        // A client built with `get_with` reuses the same `Arc<GoogleAuthHeaders>`
        // as one built with `get`, so its header cache is shared: the headers are served
        // in full once, not once per client.
        assert_eq!(credentials.served(), 1);
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
        let credentials = StubCredentials::bearer("counted-token");
        let (first_url, first_connections) = silent_endpoint().await;
        let (second_url, second_connections) = silent_endpoint().await;

        let first: GoogleApi<GoogleAuthMiddlewareService<Channel>> =
            GoogleApi::from_function_with_credentials(
                |svc| svc,
                first_url,
                None,
                Credentials::from(credentials.clone()),
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
        assert_eq!(credentials.served(), 1);
    }

    async fn detected_project_id(
        env: &[(&str, &str)],
        adc: Option<serde_json::Value>,
        metadata: &StubServer,
    ) -> Option<String> {
        GoogleEnvironment::detect_project_id_in(
            |name| {
                env.iter()
                    .find(|(set, _)| *set == name)
                    .map(|(_, value)| value.to_string())
            },
            adc.map(AdcFile::from),
            MetadataServer::at(metadata.url.clone()),
        )
        .await
    }

    #[tokio::test]
    async fn project_id_sources_are_read_in_order() {
        let metadata = StubServer::start(vec![StubResponse::text("200 OK", "metadata-project")
            .with_header("Metadata-Flavor", "Google")])
        .await;
        let mut env = vec![
            ("GCP_PROJECT", "gcp-project"),
            ("PROJECT_ID", "project-id"),
            ("GCP_PROJECT_ID", "gcp-project-id"),
            ("GOOGLE_CLOUD_PROJECT", "google-cloud-project"),
        ];
        let mut adc = json!({
            "type": "impersonated_service_account",
            "project_id": "adc-project",
            "quota_project_id": "adc-quota-project",
            "source_credentials": {
                "type": "service_account",
                "project_id": "source-project",
                "quota_project_id": "source-quota-project",
            },
        });

        let mut detected = Vec::new();
        while !env.is_empty() {
            detected.push(detected_project_id(&env, Some(adc.clone()), &metadata).await);
            env.remove(0);
        }
        for field in [
            "/project_id",
            "/quota_project_id",
            "/source_credentials/project_id",
        ] {
            detected.push(detected_project_id(&env, Some(adc.clone()), &metadata).await);
            let (parent, name) = field.rsplit_once('/').unwrap();
            adc.pointer_mut(parent)
                .and_then(serde_json::Value::as_object_mut)
                .unwrap()
                .remove(name);
        }
        detected.push(detected_project_id(&env, Some(adc), &metadata).await);
        detected.push(detected_project_id(&env, None, &metadata).await);

        assert_eq!(
            detected,
            [
                "gcp-project",
                "project-id",
                "gcp-project-id",
                "google-cloud-project",
                "adc-project",
                "adc-quota-project",
                "source-project",
                "source-quota-project",
                "metadata-project",
            ]
            .map(|project| Some(project.to_string()))
        );
    }
}
