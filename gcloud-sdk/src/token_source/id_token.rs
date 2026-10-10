use std::fmt;

use async_trait::async_trait;
use jiff::Timestamp;
use once_cell::sync::Lazy;
use secret_vault_value::SecretValue;
use tracing::*;
use url::Url;

use crate::error::{ErrorKind, IdTokenUnsupportedCredentials};
use crate::token_source::credentials::{
    auth_error_from_response, from_file, from_json, httpc_post, Credentials,
    ServiceAccountImpersonationSourceCredentials,
};
use crate::token_source::metadata::Metadata;
use crate::token_source::{
    create_source, find_default_credentials, from_metadata, BoxSource, DefaultCredentials, Source,
    Token, TokenSourceType,
};
use crate::GCP_DEFAULT_SCOPES;

/// The audience an ID token is minted for and checked against: the URL of a Cloud Run
/// service or a Cloud Run function, a custom audience configured on the service, or the
/// OAuth client ID of a resource behind Identity-Aware Proxy.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IdTokenAudience(String);

impl IdTokenAudience {
    pub fn new(audience: impl Into<String>) -> Self {
        Self(audience.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for IdTokenAudience {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// The email of a service account, such as
/// `invoker@my-project.iam.gserviceaccount.com`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ServiceAccountEmail(String);

impl ServiceAccountEmail {
    pub fn new(email: impl Into<String>) -> Self {
        Self(email.into())
    }
}

impl fmt::Display for ServiceAccountEmail {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// Mints Google-signed ID tokens for one audience, for calling services that check them,
/// such as Cloud Run, Cloud Run functions and Identity-Aware Proxy.
///
/// Each call to [`Source::token`] mints a new token, so wrap the source in a
/// [`GoogleAuthTokenGenerator`](crate::GoogleAuthTokenGenerator) with
/// [`from_source`](crate::GoogleAuthTokenGenerator::from_source) to cache it until shortly
/// before its `exp`. The token is a [`SecretValue`] and is never logged.
#[derive(Debug)]
pub struct IdTokenSource {
    audience: IdTokenAudience,
    flow: IdTokenFlow,
}

impl IdTokenSource {
    /// A source that mints ID tokens with the identity of the credentials
    /// `token_source_type` finds:
    /// - a service account key file signs an assertion and exchanges it at the key's
    ///   token endpoint;
    /// - the metadata server mints them for its service account;
    /// - an `impersonated_service_account` credentials file, such as
    ///   `gcloud auth application-default login --impersonate-service-account` writes,
    ///   and `external_account` credentials with a `service_account_impersonation_url`
    ///   impersonate the service account that URL names.
    ///
    /// Fails with [`ErrorKind::IdTokenNeedsImpersonation`] for credentials that hold no
    /// service account: `authorized_user` credentials, `external_account` credentials
    /// without impersonation, and [`TokenSourceType::ExternalSource`]. Use
    /// [`impersonating`](Self::impersonating) for those.
    pub async fn new(
        audience: IdTokenAudience,
        token_source_type: TokenSourceType,
    ) -> crate::error::Result<Self> {
        let scopes = GCP_DEFAULT_SCOPES.as_slice();
        let flow = match token_source_type {
            TokenSourceType::Default => match find_default_credentials(scopes).await? {
                DefaultCredentials::File(credentials) => IdTokenFlow::try_from(credentials)?,
                DefaultCredentials::MetadataServer(metadata) => {
                    IdTokenFlow::MetadataServer(metadata)
                }
            },
            TokenSourceType::Json(json) => {
                IdTokenFlow::try_from(from_json(json.as_bytes(), scopes)?)?
            }
            TokenSourceType::File(path) => IdTokenFlow::try_from(from_file(path, scopes)?)?,
            TokenSourceType::MetadataServer => {
                IdTokenFlow::from_metadata_server("default".to_string()).await?
            }
            TokenSourceType::MetadataServerWithAccount(account) => {
                IdTokenFlow::from_metadata_server(account).await?
            }
            TokenSourceType::ExternalSource(_) => {
                return Err(ErrorKind::IdTokenNeedsImpersonation(
                    IdTokenUnsupportedCredentials::ExternalSource,
                )
                .into())
            }
        };
        debug!(%audience, ?flow, "Created an ID token source");
        Ok(Self { audience, flow })
    }

    /// A source that mints ID tokens with the identity of `service_account`, through the
    /// IAM Credentials `generateIdToken` method. The access token that
    /// `token_source_type` yields authorizes the call, so its principal needs the
    /// `iam.serviceAccounts.getOpenIdToken` permission on `service_account`, which the
    /// Service Account OpenID Connect Identity Token Creator role grants.
    pub async fn impersonating(
        audience: IdTokenAudience,
        service_account: ServiceAccountEmail,
        token_source_type: TokenSourceType,
    ) -> crate::error::Result<Self> {
        let source = create_source(token_source_type, GCP_DEFAULT_SCOPES.clone()).await?;
        let flow = IdTokenFlow::Impersonation(Impersonation::new(
            source,
            service_account,
            &IAM_CREDENTIALS_URL,
        ));
        debug!(%audience, ?flow, "Created an ID token source");
        Ok(Self { audience, flow })
    }
}

#[async_trait]
impl Source for IdTokenSource {
    async fn token(&self) -> crate::error::Result<Token> {
        let id_token = match &self.flow {
            IdTokenFlow::ServiceAccountKey(service_account) => {
                service_account.id_token(&self.audience).await?
            }
            IdTokenFlow::MetadataServer(metadata) => {
                metadata.id_token_with_email(&self.audience).await?
            }
            IdTokenFlow::Impersonation(impersonation) => {
                impersonation.id_token(&self.audience).await?
            }
        };
        let expiry = id_token_expiry(&id_token)?;
        Ok(Token::new("Bearer".to_string(), id_token, expiry))
    }
}

#[derive(serde::Deserialize)]
struct IdTokenExpiry {
    #[serde(with = "jiff::fmt::serde::timestamp::second::required")]
    exp: Timestamp,
}

/// The `exp` claim of an ID token, read without checking its signature: the token
/// comes straight from Google over TLS, and only its holder's refresh depends on it.
fn id_token_expiry(id_token: &SecretValue) -> crate::error::Result<Timestamp> {
    let claims: IdTokenExpiry =
        jsonwebtoken::dangerous::insecure_decode_claims(id_token.as_sensitive_bytes())?;
    Ok(claims.exp)
}

impl From<IdTokenSource> for BoxSource {
    fn from(v: IdTokenSource) -> Self {
        Box::new(v)
    }
}

/// How an [`IdTokenSource`] obtains its tokens.
enum IdTokenFlow {
    ServiceAccountKey(crate::token_source::credentials::ServiceAccount),
    MetadataServer(Metadata),
    Impersonation(Impersonation),
}

impl fmt::Debug for IdTokenFlow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ServiceAccountKey(_) => f.write_str("ServiceAccountKey"),
            Self::MetadataServer(_) => f.write_str("MetadataServer"),
            Self::Impersonation(impersonation) => f
                .debug_tuple("Impersonation")
                .field(&impersonation.service_account)
                .finish(),
        }
    }
}

impl IdTokenFlow {
    async fn from_metadata_server(account: String) -> crate::error::Result<Self> {
        match from_metadata(GCP_DEFAULT_SCOPES.as_slice(), account).await? {
            Some(metadata) => Ok(Self::MetadataServer(metadata)),
            None => Err(ErrorKind::TokenSource.into()),
        }
    }
}

impl TryFrom<Credentials> for IdTokenFlow {
    type Error = crate::error::Error;

    fn try_from(credentials: Credentials) -> Result<Self, Self::Error> {
        match credentials {
            Credentials::ServiceAccount(service_account) => {
                Ok(Self::ServiceAccountKey(service_account))
            }
            Credentials::User(_) => Err(ErrorKind::IdTokenNeedsImpersonation(
                IdTokenUnsupportedCredentials::AuthorizedUser,
            )
            .into()),
            // The source credentials call generateIdToken directly, as they would call
            // generateAccessToken, rather than through an access token of the
            // impersonated account, which would need permission on itself.
            Credentials::ServiceAccountImpersonation(impersonated) => {
                let service_account = ServiceAccountEmail::from_impersonation_url(
                    &impersonated.service_account_impersonation_url,
                )?;
                let source: BoxSource = match impersonated.source_credentials {
                    ServiceAccountImpersonationSourceCredentials::ServiceAccount(
                        mut source_account,
                    ) => {
                        source_account.scopes = GCP_DEFAULT_SCOPES.clone();
                        Credentials::ServiceAccount(source_account).into()
                    }
                    ServiceAccountImpersonationSourceCredentials::User(user) => {
                        Credentials::User(user).into()
                    }
                };
                Ok(Self::Impersonation(Impersonation::new(
                    source,
                    service_account,
                    &IAM_CREDENTIALS_URL,
                )))
            }
            // The federated token from the security token service calls generateIdToken,
            // the way it calls generateAccessToken for access tokens.
            Credentials::ExternalAccount(mut external_account) => {
                let Some(url) = external_account.service_account_impersonation_url.take() else {
                    return Err(ErrorKind::IdTokenNeedsImpersonation(
                        IdTokenUnsupportedCredentials::ExternalAccount,
                    )
                    .into());
                };
                let service_account = ServiceAccountEmail::from_impersonation_url(&url)?;
                external_account.scopes = GCP_DEFAULT_SCOPES.clone();
                Ok(Self::Impersonation(Impersonation::new(
                    Credentials::ExternalAccount(external_account).into(),
                    service_account,
                    &IAM_CREDENTIALS_URL,
                )))
            }
        }
    }
}

static IAM_CREDENTIALS_URL: Lazy<Url> = Lazy::new(|| {
    Url::parse("https://iamcredentials.googleapis.com").expect("the IAM Credentials URL is valid")
});

/// IAM Credentials `generateIdToken` calls for one service account.
struct Impersonation {
    /// Yields the access token that authorizes each call.
    source: BoxSource,
    service_account: ServiceAccountEmail,
    /// The service account's `generateIdToken` endpoint.
    url: Url,
}

// https://cloud.google.com/iam/docs/reference/credentials/rest/v1/projects.serviceAccounts/generateIdToken
#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct GenerateIdTokenRequest<'a> {
    audience: &'a str,
    include_email: bool,
}

#[derive(serde::Deserialize)]
struct GenerateIdTokenResponse {
    token: SecretValue,
}

impl Impersonation {
    fn new(source: BoxSource, service_account: ServiceAccountEmail, iam_credentials: &Url) -> Self {
        let mut url = iam_credentials.clone();
        // Pushing the email as one path segment percent-encodes any `/`, `?` or `#` in it,
        // so the email cannot change which endpoint is called.
        url.path_segments_mut()
            .expect("an http(s) URL has path segments")
            .pop_if_empty()
            .extend([
                "v1",
                "projects",
                "-",
                "serviceAccounts",
                &format!("{}:generateIdToken", service_account.0),
            ]);
        Self {
            source,
            service_account,
            url,
        }
    }

    async fn id_token(&self, audience: &IdTokenAudience) -> crate::error::Result<SecretValue> {
        let access_token = self.source.token().await?;
        let response = httpc_post(self.url.as_str())
            .header(
                reqwest::header::AUTHORIZATION,
                access_token.authorization()?,
            )
            .json(&GenerateIdTokenRequest {
                audience: audience.as_str(),
                include_email: true,
            })
            .send()
            .await?;
        if response.status().is_success() {
            Ok(response.json::<GenerateIdTokenResponse>().await?.token)
        } else {
            Err(auth_error_from_response(response).await)
        }
    }
}

impl ServiceAccountEmail {
    /// The service account a `service_account_impersonation_url` names: the last path
    /// segment, up to the `:generateAccessToken` method.
    fn from_impersonation_url(url: &str) -> crate::error::Result<Self> {
        let resource = url.rsplit('/').next().unwrap_or_default();
        let email = resource.split(':').next().unwrap_or_default();
        if email.is_empty() {
            Err(ErrorKind::InvalidImpersonationUrl(url.to_string()).into())
        } else {
            Ok(Self::new(email))
        }
    }
}

// The tests mint JWTs, which needs a crypto provider.
#[cfg(all(test, any(feature = "jwt-aws-lc-rs", feature = "jwt-rust-crypto")))]
mod tests {
    use super::*;
    use crate::test_support::{signed_jwt, StubResponse, StubServer, TEST_RSA_PRIVATE_KEY};
    use jiff::SignedDuration;

    const AUDIENCE: &str = "https://orders-abc123-ew.a.run.app";
    const KEY_ID: &str = "test-key-id";
    const CLIENT_EMAIL: &str = "caller@my-project.iam.gserviceaccount.com";
    const TARGET_SERVICE_ACCOUNT: &str = "invoker@my-project.iam.gserviceaccount.com";

    fn id_token_expiring_at(expiry: Timestamp) -> String {
        signed_jwt(
            "google-key",
            &serde_json::json!({
                "iss": "https://accounts.google.com",
                "aud": AUDIENCE,
                "sub": "1234567890",
                "iat": expiry.as_second() - 3600,
                "exp": expiry.as_second(),
            }),
        )
    }

    fn service_account_key_json(token_uri: &str) -> String {
        serde_json::json!({
            "type": "service_account",
            "client_email": CLIENT_EMAIL,
            "private_key_id": KEY_ID,
            "private_key": TEST_RSA_PRIVATE_KEY,
            "token_uri": token_uri,
        })
        .to_string()
    }

    fn whole_second(timestamp: Timestamp) -> Timestamp {
        Timestamp::from_second(timestamp.as_second()).unwrap()
    }

    #[tokio::test]
    async fn service_account_key_exchanges_a_target_audience_assertion() {
        let expiry = whole_second(Timestamp::now() + SignedDuration::from_hours(1));
        let id_token = id_token_expiring_at(expiry);
        let token_endpoint = StubServer::start(vec![StubResponse::json(
            "200 OK",
            serde_json::json!({ "id_token": id_token }).to_string(),
        )])
        .await;
        let token_uri = format!("{}/token", token_endpoint.url);

        let source = IdTokenSource::new(
            IdTokenAudience::new(AUDIENCE),
            TokenSourceType::Json(service_account_key_json(&token_uri)),
        )
        .await
        .unwrap();
        let token = source.token().await.unwrap();

        assert_eq!(token.token_type, "Bearer");
        assert_eq!(token.token.as_sensitive_str(), id_token);
        assert_eq!(token.expiry, expiry);

        let received = token_endpoint.received();
        assert_eq!(received.len(), 1);
        assert_eq!(received[0].request_line, "POST /token HTTP/1.1");
        let form: std::collections::HashMap<String, String> =
            url::form_urlencoded::parse(received[0].body.as_bytes())
                .into_owned()
                .collect();
        assert_eq!(
            form["grant_type"],
            "urn:ietf:params:oauth:grant-type:jwt-bearer"
        );
        let assertion = jsonwebtoken::dangerous::insecure_decode::<serde_json::Value>(
            form["assertion"].as_bytes(),
        )
        .unwrap();
        assert_eq!(assertion.header.kid.as_deref(), Some(KEY_ID));
        assert_eq!(assertion.claims["iss"], CLIENT_EMAIL);
        assert_eq!(assertion.claims["aud"], token_uri.as_str());
        assert_eq!(assertion.claims["target_audience"], AUDIENCE);
    }

    struct FixedAccessToken;

    #[async_trait]
    impl Source for FixedAccessToken {
        async fn token(&self) -> crate::error::Result<Token> {
            Ok(Token::new(
                "Bearer".to_string(),
                SecretValue::from("caller-access-token"),
                Timestamp::now() + SignedDuration::from_hours(1),
            ))
        }
    }

    #[tokio::test]
    async fn impersonation_calls_generate_id_token_with_the_access_token() {
        let expiry = whole_second(Timestamp::now() + SignedDuration::from_hours(1));
        let id_token = id_token_expiring_at(expiry);
        let iam = StubServer::start(vec![StubResponse::json(
            "200 OK",
            serde_json::json!({ "token": id_token }).to_string(),
        )])
        .await;

        let source = IdTokenSource {
            audience: IdTokenAudience::new(AUDIENCE),
            flow: IdTokenFlow::Impersonation(Impersonation::new(
                Box::new(FixedAccessToken),
                ServiceAccountEmail::new(TARGET_SERVICE_ACCOUNT),
                &Url::parse(&iam.url).unwrap(),
            )),
        };
        let token = source.token().await.unwrap();

        assert_eq!(token.token.as_sensitive_str(), id_token);
        assert_eq!(token.expiry, expiry);
        let received = iam.received();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].request_line,
            format!(
                "POST /v1/projects/-/serviceAccounts/{TARGET_SERVICE_ACCOUNT}:generateIdToken HTTP/1.1"
            )
        );
        assert_eq!(
            received[0].header("authorization"),
            Some("Bearer caller-access-token")
        );
        let body: serde_json::Value = serde_json::from_str(&received[0].body).unwrap();
        assert_eq!(
            body,
            serde_json::json!({ "audience": AUDIENCE, "includeEmail": true })
        );
    }

    #[tokio::test]
    async fn impersonated_credentials_file_impersonates_the_service_account_it_names() {
        let credentials = serde_json::json!({
            "type": "impersonated_service_account",
            "service_account_impersonation_url": format!(
                "https://iamcredentials.googleapis.com/v1/projects/-/serviceAccounts/{TARGET_SERVICE_ACCOUNT}:generateAccessToken"
            ),
            "source_credentials": {
                "type": "authorized_user",
                "client_id": "client-id",
                "client_secret": "client-secret",
                "refresh_token": "refresh-token",
            },
        });

        let source = IdTokenSource::new(
            IdTokenAudience::new(AUDIENCE),
            TokenSourceType::Json(credentials.to_string()),
        )
        .await
        .unwrap();

        match source.flow {
            IdTokenFlow::Impersonation(impersonation) => {
                assert_eq!(
                    impersonation.service_account,
                    ServiceAccountEmail::new(TARGET_SERVICE_ACCOUNT)
                );
                assert_eq!(
                    impersonation.url.as_str(),
                    format!("https://iamcredentials.googleapis.com/v1/projects/-/serviceAccounts/{TARGET_SERVICE_ACCOUNT}:generateIdToken")
                );
            }
            other => panic!("expected impersonation, got {other:?}"),
        }
    }

    #[tokio::test]
    async fn authorized_user_credentials_need_impersonation() {
        let credentials = serde_json::json!({
            "type": "authorized_user",
            "client_id": "client-id",
            "client_secret": "client-secret",
            "refresh_token": "refresh-token",
        });

        let err = IdTokenSource::new(
            IdTokenAudience::new(AUDIENCE),
            TokenSourceType::Json(credentials.to_string()),
        )
        .await
        .unwrap_err();

        assert!(matches!(
            err.kind(),
            ErrorKind::IdTokenNeedsImpersonation(IdTokenUnsupportedCredentials::AuthorizedUser)
        ));
    }

    #[tokio::test]
    async fn generator_refreshes_an_id_token_inside_the_refresh_margin() {
        let soon = whole_second(Timestamp::now() + SignedDuration::from_secs(10));
        let later = whole_second(Timestamp::now() + SignedDuration::from_hours(1));
        let token_endpoint = StubServer::start(vec![
            StubResponse::json(
                "200 OK",
                serde_json::json!({ "id_token": id_token_expiring_at(soon) }).to_string(),
            ),
            StubResponse::json(
                "200 OK",
                serde_json::json!({ "id_token": id_token_expiring_at(later) }).to_string(),
            ),
        ])
        .await;
        let source = IdTokenSource::new(
            IdTokenAudience::new(AUDIENCE),
            TokenSourceType::Json(service_account_key_json(&format!(
                "{}/token",
                token_endpoint.url
            ))),
        )
        .await
        .unwrap();
        let generator = crate::GoogleAuthTokenGenerator::from_source(source);

        let first = generator.create_token().await.unwrap();
        let second = generator.create_token().await.unwrap();
        let third = generator.create_token().await.unwrap();

        assert_eq!(first.expiry, soon);
        assert_eq!(second.expiry, later);
        assert_eq!(third.expiry, later);
        assert_eq!(token_endpoint.received().len(), 2);
    }
}
