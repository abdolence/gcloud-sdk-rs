use std::sync::{Arc, Mutex, PoisonError, RwLock};

use google_cloud_auth::credentials::idtoken::{self, IDTokenCredentials};
use google_cloud_auth::credentials::{
    external_account, CacheableResource, Credentials, CredentialsProvider, EntityTag,
};
use google_cloud_auth::errors::CredentialsError;
use hyper::header::{Entry, HeaderMap, HeaderValue, AUTHORIZATION};
use hyper::http::Extensions;
use serde_json::Value;

use crate::adc::{AdcFile, AdcKind};
use crate::error::{ErrorKind, IdTokenUnsupportedCredentials};
use crate::{IdTokenAudience, ServiceAccountEmail, GCP_DEFAULT_SCOPES};

/// The authentication headers of google-cloud-auth credentials, for every request the
/// tonic and reqwest middleware send: `authorization` with an access token or an ID token,
/// and whatever else the credentials add, such as `x-goog-user-project` for a quota
/// project.
///
/// The credentials mint, cache and refresh the tokens. The generator keeps the headers of
/// the current token, with `authorization` marked sensitive so that it stays out of logs,
/// and takes new ones only when the credentials report a new token.
///
/// [`id_token`](Self::id_token) and [`id_token_impersonating`](Self::id_token_impersonating)
/// build credentials, which spawns their refresh task on the current Tokio runtime.
#[derive(Debug)]
pub struct GoogleAuthTokenGenerator {
    credentials: Credentials,
    /// Replaced whole under the lock, so a poisoned lock still holds consistent headers.
    cached: RwLock<Option<CachedHeaders>>,
}

#[derive(Debug, Clone)]
struct CachedHeaders {
    entity_tag: EntityTag,
    headers: Arc<HeaderMap>,
}

impl GoogleAuthTokenGenerator {
    /// ID tokens for `audience`, minted with the identity of the Application Default
    /// Credentials:
    /// - a service account key file, as that service account;
    /// - the metadata server, as the service account of the instance;
    /// - `impersonated_service_account` credentials, such as
    ///   `gcloud auth application-default login --impersonate-service-account` writes, as
    ///   the service account they impersonate;
    /// - workload identity federation with a `service_account_impersonation_url`, as the
    ///   service account that URL names, with the federated token calling
    ///   `generateIdToken` the way it calls `generateAccessToken` for access tokens.
    ///
    /// Fails with [`ErrorKind::IdTokenNeedsImpersonation`] for credentials that hold no
    /// service account: `authorized_user` credentials, and workload identity federation
    /// without impersonation. Use [`id_token_impersonating`](Self::id_token_impersonating)
    /// for those.
    pub async fn id_token(audience: &IdTokenAudience) -> crate::error::Result<Self> {
        Self::id_token_from_adc(AdcFile::load(), audience)
    }

    fn id_token_from_adc(
        adc: Option<AdcFile>,
        audience: &IdTokenAudience,
    ) -> crate::error::Result<Self> {
        let credentials = match adc.as_ref().map(AdcFile::kind) {
            Some(AdcKind::AuthorizedUser) => {
                return Err(ErrorKind::IdTokenNeedsImpersonation(
                    IdTokenUnsupportedCredentials::AuthorizedUser,
                )
                .into())
            }
            Some(AdcKind::ExternalAccount(config)) => {
                federated_id_token_credentials(config, audience)?
            }
            Some(AdcKind::Other) | None => idtoken::Builder::new(audience.as_str())
                .with_include_email()
                .build()?,
        };
        Ok(Self::from(credentials))
    }

    /// ID tokens for `audience`, minted with the identity of `service_account` through the
    /// IAM Credentials `generateIdToken` method, called with the access tokens of
    /// `source_credentials`. Their principal needs the `iam.serviceAccounts.getOpenIdToken`
    /// permission on `service_account`, which the Service Account OpenID Connect Identity
    /// Token Creator role grants.
    pub async fn id_token_impersonating(
        audience: &IdTokenAudience,
        service_account: &ServiceAccountEmail,
        source_credentials: Credentials,
    ) -> crate::error::Result<Self> {
        let credentials = idtoken::impersonated::Builder::from_source_credentials(
            audience.as_str(),
            service_account.to_string(),
            source_credentials,
        )
        .with_include_email()
        .build()?;
        Ok(Self::from(credentials))
    }

    /// The headers to set on a request, replacing any it carries under the same names.
    pub async fn headers(&self) -> crate::error::Result<Arc<HeaderMap>> {
        let cached = self
            .cached
            .read()
            .unwrap_or_else(PoisonError::into_inner)
            .clone();
        let mut extensions = Extensions::new();
        if let Some(cached) = &cached {
            extensions.insert(cached.entity_tag.clone());
        }
        match self.credentials.headers(extensions).await? {
            CacheableResource::NotModified => {
                cached.map(|cached| cached.headers).ok_or_else(|| {
                    CredentialsError::from_msg(
                        false,
                        "the credentials reported no change to headers they never served",
                    )
                    .into()
                })
            }
            CacheableResource::New {
                entity_tag,
                mut data,
            } => {
                if let Entry::Occupied(mut authorization) = data.entry(AUTHORIZATION) {
                    authorization
                        .iter_mut()
                        .for_each(|value| value.set_sensitive(true));
                }
                let headers = Arc::new(data);
                *self.cached.write().unwrap_or_else(PoisonError::into_inner) =
                    Some(CachedHeaders {
                        entity_tag,
                        headers: Arc::clone(&headers),
                    });
                Ok(headers)
            }
        }
    }
}

/// Access tokens, or whatever headers `credentials` serve.
impl From<Credentials> for GoogleAuthTokenGenerator {
    fn from(credentials: Credentials) -> Self {
        Self {
            credentials,
            cached: RwLock::new(None),
        }
    }
}

/// ID tokens from any google-cloud-auth ID token credentials, such as
/// `idtoken::service_account::Builder` builds from a key that is not the Application
/// Default Credentials.
impl From<IDTokenCredentials> for GoogleAuthTokenGenerator {
    fn from(credentials: IDTokenCredentials) -> Self {
        Self::from(Credentials::from(IdTokenHeaders::from(credentials)))
    }
}

/// ID tokens for the service account a workload identity federation `config` impersonates.
///
/// The ID tokens are minted with the federated token itself, the source of the
/// impersonation: an ID token minted with the impersonated account's own access token
/// would need that account to hold `getOpenIdToken` on itself.
fn federated_id_token_credentials(
    config: &Value,
    audience: &IdTokenAudience,
) -> crate::error::Result<IDTokenCredentials> {
    let Some(impersonation_url) = config
        .get("service_account_impersonation_url")
        .and_then(Value::as_str)
    else {
        return Err(ErrorKind::IdTokenNeedsImpersonation(
            IdTokenUnsupportedCredentials::ExternalAccount,
        )
        .into());
    };
    let service_account = ServiceAccountEmail::from_impersonation_url(impersonation_url)?;
    let mut federation = config.clone();
    if let Some(fields) = federation.as_object_mut() {
        fields.remove("service_account_impersonation_url");
    }
    let federated = external_account::Builder::new(federation)
        .with_scopes(GCP_DEFAULT_SCOPES.iter())
        .build()?;
    Ok(idtoken::impersonated::Builder::from_source_credentials(
        audience.as_str(),
        service_account.to_string(),
        federated,
    )
    .with_include_email()
    .build()?)
}

/// Serves the ID tokens of `IDTokenCredentials` as an `authorization` header, under an
/// entity tag that changes only when the token does, so that a [`GoogleAuthTokenGenerator`]
/// caches them like access token headers.
#[derive(Debug)]
struct IdTokenHeaders {
    credentials: IDTokenCredentials,
    /// Replaced whole under the lock, so a poisoned lock still holds a consistent header.
    current: Mutex<Option<TaggedAuthorization>>,
}

#[derive(Debug, Clone)]
struct TaggedAuthorization {
    entity_tag: EntityTag,
    authorization: HeaderValue,
}

impl From<IDTokenCredentials> for IdTokenHeaders {
    fn from(credentials: IDTokenCredentials) -> Self {
        Self {
            credentials,
            current: Mutex::new(None),
        }
    }
}

impl IdTokenHeaders {
    /// The tagged header of `id_token`: the current one when the token is unchanged,
    /// else a new one under a new tag.
    fn tagged(&self, id_token: &str) -> Result<TaggedAuthorization, CredentialsError> {
        let mut current = self.current.lock().unwrap_or_else(PoisonError::into_inner);
        if let Some(tagged) = current.as_ref().filter(|tagged| tagged.is_for(id_token)) {
            return Ok(tagged.clone());
        }
        let mut authorization = HeaderValue::from_str(&format!("Bearer {id_token}"))
            .map_err(|error| CredentialsError::from_source(false, error))?;
        authorization.set_sensitive(true);
        let tagged = TaggedAuthorization {
            entity_tag: EntityTag::new(),
            authorization,
        };
        *current = Some(tagged.clone());
        Ok(tagged)
    }
}

impl TaggedAuthorization {
    fn is_for(&self, id_token: &str) -> bool {
        self.authorization.as_bytes().strip_prefix(b"Bearer ") == Some(id_token.as_bytes())
    }
}

impl CredentialsProvider for IdTokenHeaders {
    async fn headers(
        &self,
        extensions: Extensions,
    ) -> Result<CacheableResource<HeaderMap>, CredentialsError> {
        let id_token = self.credentials.id_token().await?;
        let tagged = self.tagged(&id_token)?;
        if extensions.get::<EntityTag>() == Some(&tagged.entity_tag) {
            return Ok(CacheableResource::NotModified);
        }
        let mut headers = HeaderMap::new();
        headers.insert(AUTHORIZATION, tagged.authorization);
        Ok(CacheableResource::New {
            entity_tag: tagged.entity_tag,
            data: headers,
        })
    }

    async fn universe_domain(&self) -> Option<String> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::{ErrorKind, IdTokenUnsupportedCredentials};
    use google_cloud_auth::credentials::idtoken::IDTokenCredentialsProvider;
    use serde_json::json;
    use std::sync::Mutex;

    const AUDIENCE: &str = "https://orders-abc123-ew.a.run.app";
    const INVOKER: &str = "invoker@my-project.iam.gserviceaccount.com";

    /// ID token credentials that serve the token last scripted. Clones share the script.
    #[derive(Debug, Clone)]
    struct ScriptedIdToken {
        token: Arc<Mutex<String>>,
    }

    impl ScriptedIdToken {
        fn new(token: &str) -> Self {
            Self {
                token: Arc::new(Mutex::new(token.to_string())),
            }
        }

        fn rotate(&self, token: &str) {
            *self.token.lock().unwrap() = token.to_string();
        }
    }

    impl IDTokenCredentialsProvider for ScriptedIdToken {
        async fn id_token(&self) -> Result<String, CredentialsError> {
            Ok(self.token.lock().unwrap().clone())
        }
    }

    #[tokio::test]
    async fn id_token_is_served_as_a_sensitive_bearer_header() {
        let generator = GoogleAuthTokenGenerator::from(IDTokenCredentials::from(
            ScriptedIdToken::new("minted-id-token"),
        ));

        let headers = generator.headers().await.unwrap();

        assert_eq!(headers[AUTHORIZATION], "Bearer minted-id-token");
        assert!(headers[AUTHORIZATION].is_sensitive());
    }

    #[tokio::test]
    async fn id_token_headers_keep_their_entity_tag_until_the_token_changes() {
        let id_token = ScriptedIdToken::new("first-id-token");
        let credentials = Credentials::from(IdTokenHeaders::from(IDTokenCredentials::from(
            id_token.clone(),
        )));

        let CacheableResource::New { entity_tag, .. } =
            credentials.headers(Extensions::new()).await.unwrap()
        else {
            panic!("expected the first headers in full");
        };
        let mut holding_first = Extensions::new();
        holding_first.insert(entity_tag.clone());
        assert_eq!(
            credentials.headers(holding_first.clone()).await.unwrap(),
            CacheableResource::NotModified
        );

        id_token.rotate("second-id-token");
        match credentials.headers(holding_first).await.unwrap() {
            CacheableResource::New {
                entity_tag: rotated,
                data,
            } => {
                assert_ne!(rotated, entity_tag);
                assert_eq!(data[AUTHORIZATION], "Bearer second-id-token");
            }
            CacheableResource::NotModified => panic!("expected the rotated token"),
        }
    }

    fn federation_config() -> serde_json::Value {
        json!({
            "type": "external_account",
            "audience": "//iam.googleapis.com/projects/123/locations/global/workloadIdentityPools/github/providers/github",
            "subject_token_type": "urn:ietf:params:oauth:token-type:jwt",
            "token_url": "https://sts.googleapis.com/v1/token",
            "credential_source": { "file": "/nonexistent/oidc-token" },
        })
    }

    fn id_token_error(adc: serde_json::Value) -> ErrorKind {
        GoogleAuthTokenGenerator::id_token_from_adc(
            Some(AdcFile::from(adc)),
            &IdTokenAudience::new(AUDIENCE),
        )
        .unwrap_err()
        .into_kind()
    }

    #[tokio::test]
    async fn authorized_user_credentials_need_impersonation() {
        let user = json!({
            "type": "authorized_user",
            "client_id": "client-id",
            "client_secret": "client-secret",
            "refresh_token": "refresh-token",
        });

        assert!(matches!(
            id_token_error(user),
            ErrorKind::IdTokenNeedsImpersonation(IdTokenUnsupportedCredentials::AuthorizedUser)
        ));
    }

    #[tokio::test]
    async fn external_account_credentials_without_impersonation_need_impersonation() {
        assert!(matches!(
            id_token_error(federation_config()),
            ErrorKind::IdTokenNeedsImpersonation(IdTokenUnsupportedCredentials::ExternalAccount)
        ));
    }

    #[tokio::test]
    async fn external_account_credentials_with_impersonation_mint_id_tokens() {
        let mut config = federation_config();
        config["service_account_impersonation_url"] = json!(format!(
            "https://iamcredentials.googleapis.com/v1/projects/-/serviceAccounts/{INVOKER}:generateAccessToken"
        ));

        GoogleAuthTokenGenerator::id_token_from_adc(
            Some(AdcFile::from(config)),
            &IdTokenAudience::new(AUDIENCE),
        )
        .unwrap();
    }
}
