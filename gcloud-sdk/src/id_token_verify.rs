//! Verification of Google-signed ID tokens, for the receiving side of a
//! service-to-service call.
//!
//! Signatures are checked with the crypto provider of the `jwt-aws-lc-rs` or
//! `jwt-rust-crypto` feature. Without either, every token is refused with
//! [`IdTokenVerifyError::KeysUnavailable`] carrying
//! [`ErrorKind::JwtCryptoProviderMissing`].

use std::collections::HashMap;
use std::fmt;
use std::sync::Arc;
use std::time::{Duration, Instant};

use async_trait::async_trait;
use jiff::{SignedDuration, Timestamp};
use jsonwebtoken::jwk::JwkSet;
use jsonwebtoken::{Algorithm, DecodingKey, Validation};
use tokio::sync::{Mutex, RwLock};
use tracing::*;

use crate::error::ErrorKind;
use crate::IdTokenAudience;

/// Google's ID token signing keys, the `jwks_uri` of
/// <https://accounts.google.com/.well-known/openid-configuration>.
const GOOGLE_KEYS_URL: &str = "https://www.googleapis.com/oauth2/v3/certs";

/// <https://developers.google.com/identity/openid-connect/openid-connect#validatinganidtoken>
const GOOGLE_ISSUERS: [&str; 2] = ["https://accounts.google.com", "accounts.google.com"];

/// Clock skew allowed on `exp`, `nbf` and `iat`.
const LEEWAY: SignedDuration = SignedDuration::from_secs(30);

/// How long keys are used when their response carries no `Cache-Control: max-age`.
const KEYS_DEFAULT_MAX_AGE: Duration = Duration::from_secs(5 * 60);

/// The longest keys are used without a fetch, whatever their `max-age`.
const KEYS_MAX_AGE_LIMIT: Duration = Duration::from_secs(24 * 60 * 60);

/// The shortest time between two fetches caused by unknown key IDs or by failures, so
/// that tokens with made-up key IDs cannot make the verifier hammer the keys endpoint.
const REFETCH_INTERVAL: Duration = Duration::from_secs(30);

const KEYS_FETCH_TIMEOUT: Duration = Duration::from_secs(10);

/// The email of the principal an ID token was minted for: a service account, or a user
/// for tokens minted with user credentials.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PrincipalEmail(String);

impl PrincipalEmail {
    pub fn new(email: impl Into<String>) -> Self {
        Self(email.into())
    }
}

impl fmt::Display for PrincipalEmail {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// The claims of an ID token whose signature, issuer, audience and lifetime
/// [`IdTokenVerifier::verify`] checked. Only the verifier creates it.
#[derive(Debug, Clone)]
pub struct VerifiedIdToken {
    audience: IdTokenAudience,
    subject: String,
    email: Option<PrincipalEmail>,
    email_verified: Option<bool>,
    hosted_domain: Option<String>,
    expires_at: Timestamp,
}

impl VerifiedIdToken {
    /// The `aud` claim, which is the verifier's audience.
    pub fn audience(&self) -> &IdTokenAudience {
        &self.audience
    }

    /// The `sub` claim: the principal's unique, stable ID.
    pub fn subject(&self) -> &str {
        &self.subject
    }

    /// The `email` claim, present when the token was minted with the principal's email,
    /// whether or not Google verified it.
    pub fn email(&self) -> Option<&PrincipalEmail> {
        self.email.as_ref()
    }

    /// The `email_verified` claim.
    pub fn email_verified(&self) -> Option<bool> {
        self.email_verified
    }

    /// The email when the token says Google verified it: the principal to authorize.
    pub fn verified_email(&self) -> Option<&PrincipalEmail> {
        match self.email_verified {
            Some(true) => self.email.as_ref(),
            Some(false) | None => None,
        }
    }

    /// The `hd` claim: the Google Workspace or Cloud Identity domain of a user.
    pub fn hosted_domain(&self) -> Option<&str> {
        self.hosted_domain.as_deref()
    }

    /// The `exp` claim.
    pub fn expires_at(&self) -> Timestamp {
        self.expires_at
    }
}

/// Why [`IdTokenVerifier::verify`] refused a token.
#[derive(Debug)]
pub enum IdTokenVerifyError {
    /// The token is not a valid Google ID token for the audience; the caller is not
    /// authenticated.
    InvalidToken(InvalidIdToken),
    /// The signing keys could not be fetched, or no crypto provider is enabled to use
    /// them, so the token could not be checked; it may be valid. A failed fetch is
    /// reported to every verification for a short interval before the next fetch, hence
    /// the shared error.
    KeysUnavailable(Arc<crate::error::Error>),
}

impl fmt::Display for IdTokenVerifyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidToken(reason) => write!(f, "invalid ID token: {reason}"),
            Self::KeysUnavailable(error) => {
                write!(f, "ID token signing keys are unavailable: {error}")
            }
        }
    }
}

impl std::error::Error for IdTokenVerifyError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::InvalidToken(reason) => Some(reason),
            Self::KeysUnavailable(error) => Some(error.as_ref()),
        }
    }
}

impl From<InvalidIdToken> for IdTokenVerifyError {
    fn from(reason: InvalidIdToken) -> Self {
        Self::InvalidToken(reason)
    }
}

/// What is wrong with an invalid ID token.
#[derive(Debug)]
#[non_exhaustive]
pub enum InvalidIdToken {
    /// Not an RS256 JWT, or its claims are missing or of the wrong type.
    Malformed(jsonwebtoken::errors::Error),
    /// The key ID in the header, if any, is not among Google's signing keys.
    UnknownKey(Option<String>),
    /// The signature does not match the key.
    BadSignature,
    /// `exp` has passed.
    Expired,
    /// `nbf` or `iat` is in the future.
    NotYetValid,
    /// `aud` is not the verifier's audience.
    WrongAudience,
    /// `iss` is not Google.
    WrongIssuer,
}

impl fmt::Display for InvalidIdToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Malformed(error) => write!(f, "malformed token: {error}"),
            Self::UnknownKey(Some(kid)) => write!(f, "unknown signing key {kid}"),
            Self::UnknownKey(None) => write!(f, "no signing key ID"),
            Self::BadSignature => write!(f, "bad signature"),
            Self::Expired => write!(f, "expired"),
            Self::NotYetValid => write!(f, "not yet valid"),
            Self::WrongAudience => write!(f, "wrong audience"),
            Self::WrongIssuer => write!(f, "wrong issuer"),
        }
    }
}

impl std::error::Error for InvalidIdToken {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Malformed(error) => Some(error),
            _ => None,
        }
    }
}

impl From<jsonwebtoken::errors::Error> for InvalidIdToken {
    fn from(error: jsonwebtoken::errors::Error) -> Self {
        use jsonwebtoken::errors::ErrorKind as JwtErrorKind;
        match error.kind() {
            JwtErrorKind::InvalidSignature => Self::BadSignature,
            JwtErrorKind::ExpiredSignature => Self::Expired,
            JwtErrorKind::ImmatureSignature => Self::NotYetValid,
            JwtErrorKind::InvalidAudience => Self::WrongAudience,
            JwtErrorKind::InvalidIssuer => Self::WrongIssuer,
            _ => Self::Malformed(error),
        }
    }
}

/// Signing keys, and how long they may be used before they are fetched again.
pub struct IdTokenKeys {
    keys: JwkSet,
    max_age: Duration,
}

impl IdTokenKeys {
    /// `keys` may be used for `max_age` after they are fetched, up to a day. Only keys
    /// with a key ID (`kid`) are used.
    pub fn new(keys: JwkSet, max_age: Duration) -> Self {
        Self { keys, max_age }
    }
}

/// Where an [`IdTokenVerifier`] gets its signing keys: Google's published keys by
/// default, or a local key in tests.
#[async_trait]
pub trait IdTokenKeysSource: Send + Sync {
    /// The signing keys as of now. An error means the keys are unavailable.
    async fn fetch_keys(&self) -> crate::error::Result<IdTokenKeys>;
}

/// Google's published signing keys, fetched over HTTPS.
struct GoogleKeys {
    client: reqwest::Client,
    url: String,
}

#[async_trait]
impl IdTokenKeysSource for GoogleKeys {
    async fn fetch_keys(&self) -> crate::error::Result<IdTokenKeys> {
        let response = self.client.get(&self.url).send().await?;
        let status = response.status();
        if !status.is_success() {
            return Err(ErrorKind::HttpStatus(status).into());
        }
        let max_age = cache_max_age(response.headers()).unwrap_or(KEYS_DEFAULT_MAX_AGE);
        let keys = response.json::<JwkSet>().await?;
        Ok(IdTokenKeys::new(keys, max_age))
    }
}

/// The `max-age` directive of a response's `Cache-Control` header.
fn cache_max_age(headers: &reqwest::header::HeaderMap) -> Option<Duration> {
    let cache_control = headers.get(reqwest::header::CACHE_CONTROL)?.to_str().ok()?;
    cache_control
        .split(',')
        .find_map(|directive| directive.trim().strip_prefix("max-age="))
        .and_then(|seconds| seconds.trim().parse().ok())
        .map(Duration::from_secs)
}

/// Verifies Google-signed ID tokens for one audience: the RS256 signature against
/// Google's signing keys, `iss`, `aud`, and `exp`, `nbf` and `iat` with 30 seconds of
/// leeway.
///
/// Keys are fetched on first use and cached for the `max-age` of their response. A
/// token signed with a key that is not cached causes a refetch, at most once every 30
/// seconds. When a fetch fails, cached keys stay in use until their `max-age` passes.
/// Share one verifier, behind an `Arc`, across requests.
pub struct IdTokenVerifier {
    validation: Validation,
    keys_source: Box<dyn IdTokenKeysSource>,
    cache: RwLock<KeyCache>,
    /// Held while fetching, so that concurrent verifications fetch once.
    fetching: Mutex<()>,
    refetch_interval: Duration,
}

impl IdTokenVerifier {
    /// A verifier that fetches Google's signing keys from
    /// <https://www.googleapis.com/oauth2/v3/certs>.
    pub fn new(audience: IdTokenAudience) -> crate::error::Result<Self> {
        let client = reqwest::Client::builder()
            .user_agent(crate::GCLOUD_SDK_USER_AGENT)
            .timeout(KEYS_FETCH_TIMEOUT)
            .build()?;
        Ok(Self::with_keys_source(
            audience,
            GoogleKeys {
                client,
                url: GOOGLE_KEYS_URL.to_string(),
            },
        ))
    }

    /// A verifier that gets its signing keys from `keys_source`, such as a fixed local
    /// key in tests. The other checks are the same as [`new`](Self::new)'s.
    pub fn with_keys_source(
        audience: IdTokenAudience,
        keys_source: impl IdTokenKeysSource + 'static,
    ) -> Self {
        let mut validation = Validation::new(Algorithm::RS256);
        validation.set_audience(&[audience.as_str()]);
        validation.set_issuer(&GOOGLE_ISSUERS);
        validation.set_required_spec_claims(&["exp", "iss", "aud", "sub"]);
        validation.leeway = LEEWAY.as_secs().unsigned_abs();
        validation.validate_nbf = true;
        Self {
            validation,
            keys_source: Box::new(keys_source),
            cache: RwLock::new(KeyCache::default()),
            fetching: Mutex::new(()),
            refetch_interval: REFETCH_INTERVAL,
        }
    }

    /// Verifies `token`, the bearer token of an `authorization` header, and returns its
    /// claims.
    pub async fn verify(&self, token: &str) -> Result<VerifiedIdToken, IdTokenVerifyError> {
        crate::jwt_crypto::ensure_provider()
            .map_err(|error| IdTokenVerifyError::KeysUnavailable(Arc::new(error)))?;
        let header = jsonwebtoken::decode_header(token).map_err(InvalidIdToken::Malformed)?;
        if header.alg != Algorithm::RS256 {
            return Err(InvalidIdToken::Malformed(
                jsonwebtoken::errors::ErrorKind::InvalidAlgorithm.into(),
            )
            .into());
        }
        let kid = header.kid.ok_or(InvalidIdToken::UnknownKey(None))?;
        let key = self.key(kid).await?;
        let claims = jsonwebtoken::decode::<GoogleIdTokenClaims>(token, &key, &self.validation)
            .map_err(InvalidIdToken::from)?
            .claims;
        // jsonwebtoken checks `exp` and `nbf` but not `iat`.
        if claims.iat > Timestamp::now() + LEEWAY {
            return Err(InvalidIdToken::NotYetValid.into());
        }
        Ok(claims.into())
    }

    async fn key(&self, kid: String) -> Result<Arc<DecodingKey>, IdTokenVerifyError> {
        if let Some(found) = self.cache.read().await.lookup(&kid, self.refetch_interval) {
            return found;
        }
        let _fetching = self.fetching.lock().await;
        // Another verification may have fetched while this one waited for the lock.
        if let Some(found) = self.cache.read().await.lookup(&kid, self.refetch_interval) {
            return found;
        }
        self.cache.write().await.last_fetch = Some(Instant::now());
        let fetched = self.keys_source.fetch_keys().await;
        let mut cache = self.cache.write().await;
        match fetched {
            Ok(keys) => {
                cache.store(keys);
                match cache.keys.get(&kid) {
                    Some(key) => Ok(Arc::clone(key)),
                    None => Err(InvalidIdToken::UnknownKey(Some(kid)).into()),
                }
            }
            Err(error) => {
                warn!(%error, "Fetching ID token signing keys failed");
                let error = Arc::new(error);
                cache.last_failure = Some(Arc::clone(&error));
                Err(IdTokenVerifyError::KeysUnavailable(error))
            }
        }
    }
}

#[derive(serde::Deserialize)]
struct GoogleIdTokenClaims {
    aud: String,
    sub: String,
    #[serde(with = "jiff::fmt::serde::timestamp::second::required")]
    exp: Timestamp,
    #[serde(with = "jiff::fmt::serde::timestamp::second::required")]
    iat: Timestamp,
    email: Option<String>,
    email_verified: Option<bool>,
    hd: Option<String>,
}

impl From<GoogleIdTokenClaims> for VerifiedIdToken {
    fn from(claims: GoogleIdTokenClaims) -> Self {
        Self {
            audience: IdTokenAudience::new(claims.aud),
            subject: claims.sub,
            email: claims.email.map(PrincipalEmail::new),
            email_verified: claims.email_verified,
            hosted_domain: claims.hd,
            expires_at: claims.exp,
        }
    }
}

/// The signing keys by key ID, and what the verifier knows about fetching them.
#[derive(Default)]
struct KeyCache {
    keys: HashMap<String, Arc<DecodingKey>>,
    /// When the keys stop being fresh; `None` until a fetch succeeds.
    fresh_until: Option<Instant>,
    /// When the last fetch started, whether it succeeded or not.
    last_fetch: Option<Instant>,
    /// The error of the last fetch, when it failed.
    last_failure: Option<Arc<crate::error::Error>>,
}

impl KeyCache {
    /// The outcome for `kid` without fetching, or `None` when the keys must be fetched.
    fn lookup(
        &self,
        kid: &str,
        refetch_interval: Duration,
    ) -> Option<Result<Arc<DecodingKey>, IdTokenVerifyError>> {
        let now = Instant::now();
        let fresh = self.fresh_until.is_some_and(|until| now < until);
        if fresh {
            if let Some(key) = self.keys.get(kid) {
                return Some(Ok(Arc::clone(key)));
            }
        }
        let fetched_recently = self
            .last_fetch
            .is_some_and(|at| now.duration_since(at) < refetch_interval);
        if !fetched_recently {
            return None;
        }
        match &self.last_failure {
            Some(error) => Some(Err(IdTokenVerifyError::KeysUnavailable(Arc::clone(error)))),
            None if fresh => Some(Err(InvalidIdToken::UnknownKey(Some(kid.to_string())).into())),
            // Keys whose max-age is shorter than the refetch interval.
            None => None,
        }
    }

    fn store(&mut self, fetched: IdTokenKeys) {
        self.keys = fetched
            .keys
            .keys
            .iter()
            .filter_map(|jwk| {
                let kid = jwk.common.key_id.clone()?;
                match DecodingKey::from_jwk(jwk) {
                    Ok(key) => Some((kid, Arc::new(key))),
                    Err(error) => {
                        warn!(%kid, %error, "Skipping an unusable ID token signing key");
                        None
                    }
                }
            })
            .collect();
        self.fresh_until = Some(Instant::now() + fetched.max_age.min(KEYS_MAX_AGE_LIMIT));
        self.last_failure = None;
    }
}

// The tests sign tokens, which needs a crypto provider.
#[cfg(all(test, any(feature = "jwt-aws-lc-rs", feature = "jwt-rust-crypto")))]
mod tests {
    use super::*;
    use crate::test_support::{signed_jwt, test_jwk_set, StubResponse, StubServer};
    use std::collections::VecDeque;
    use std::sync::atomic::{AtomicUsize, Ordering};

    const AUDIENCE: &str = "https://orders-abc123-ew.a.run.app";
    const CALLER: &str = "caller@my-project.iam.gserviceaccount.com";
    const CURRENT_KEY: &str = "current-key";
    const ROTATED_KEY: &str = "rotated-key";

    fn claims() -> serde_json::Value {
        let now = Timestamp::now();
        serde_json::json!({
            "iss": "https://accounts.google.com",
            "aud": AUDIENCE,
            "sub": "112233445566778899",
            "email": CALLER,
            "email_verified": true,
            "iat": now.as_second(),
            "exp": (now + SignedDuration::from_hours(1)).as_second(),
        })
    }

    /// Answers each fetch with the next scripted result and counts the fetches.
    struct ScriptedKeys {
        results: std::sync::Mutex<VecDeque<crate::error::Result<IdTokenKeys>>>,
        fetches: Arc<AtomicUsize>,
    }

    impl ScriptedKeys {
        fn new(results: Vec<crate::error::Result<IdTokenKeys>>) -> (Self, Arc<AtomicUsize>) {
            let fetches = Arc::new(AtomicUsize::new(0));
            let source = Self {
                results: std::sync::Mutex::new(results.into()),
                fetches: Arc::clone(&fetches),
            };
            (source, fetches)
        }
    }

    #[async_trait]
    impl IdTokenKeysSource for ScriptedKeys {
        async fn fetch_keys(&self) -> crate::error::Result<IdTokenKeys> {
            self.fetches.fetch_add(1, Ordering::SeqCst);
            self.results
                .lock()
                .unwrap()
                .pop_front()
                .expect("a scripted result for every fetch")
        }
    }

    fn keys(key_ids: &[&str]) -> crate::error::Result<IdTokenKeys> {
        Ok(IdTokenKeys::new(
            test_jwk_set(key_ids),
            Duration::from_secs(3600),
        ))
    }

    fn unavailable() -> crate::error::Result<IdTokenKeys> {
        Err(ErrorKind::HttpStatus(reqwest::StatusCode::SERVICE_UNAVAILABLE).into())
    }

    fn verifier(
        results: Vec<crate::error::Result<IdTokenKeys>>,
    ) -> (IdTokenVerifier, Arc<AtomicUsize>) {
        let (source, fetches) = ScriptedKeys::new(results);
        (
            IdTokenVerifier::with_keys_source(IdTokenAudience::new(AUDIENCE), source),
            fetches,
        )
    }

    #[tokio::test]
    async fn valid_token_yields_its_claims() {
        let (verifier, _) = verifier(vec![keys(&[CURRENT_KEY])]);
        let claims = claims();

        let verified = verifier
            .verify(&signed_jwt(CURRENT_KEY, &claims))
            .await
            .unwrap();

        assert_eq!(verified.audience(), &IdTokenAudience::new(AUDIENCE));
        assert_eq!(verified.subject(), "112233445566778899");
        assert_eq!(
            verified.verified_email(),
            Some(&PrincipalEmail::new(CALLER))
        );
        assert_eq!(verified.email_verified(), Some(true));
        assert_eq!(verified.hosted_domain(), None);
        assert_eq!(
            verified.expires_at().as_second(),
            claims["exp"].as_i64().unwrap()
        );
    }

    #[tokio::test]
    async fn expired_token_is_invalid() {
        let (verifier, _) = verifier(vec![keys(&[CURRENT_KEY])]);
        let mut expired = claims();
        expired["exp"] = (Timestamp::now() - SignedDuration::from_mins(5))
            .as_second()
            .into();

        let err = verifier
            .verify(&signed_jwt(CURRENT_KEY, &expired))
            .await
            .unwrap_err();

        assert!(matches!(
            err,
            IdTokenVerifyError::InvalidToken(InvalidIdToken::Expired)
        ));
    }

    #[tokio::test]
    async fn token_for_another_audience_is_invalid() {
        let (verifier, _) = verifier(vec![keys(&[CURRENT_KEY])]);
        let mut other_audience = claims();
        other_audience["aud"] = "https://billing-abc123-ew.a.run.app".into();

        let err = verifier
            .verify(&signed_jwt(CURRENT_KEY, &other_audience))
            .await
            .unwrap_err();

        assert!(matches!(
            err,
            IdTokenVerifyError::InvalidToken(InvalidIdToken::WrongAudience)
        ));
    }

    #[tokio::test]
    async fn token_from_another_issuer_is_invalid() {
        let (verifier, _) = verifier(vec![keys(&[CURRENT_KEY])]);
        let mut other_issuer = claims();
        other_issuer["iss"] = "https://issuer.example.com".into();

        let err = verifier
            .verify(&signed_jwt(CURRENT_KEY, &other_issuer))
            .await
            .unwrap_err();

        assert!(matches!(
            err,
            IdTokenVerifyError::InvalidToken(InvalidIdToken::WrongIssuer)
        ));
    }

    #[tokio::test]
    async fn unknown_key_refetches_the_keys() {
        let (mut verifier, fetches) = verifier(vec![
            keys(&[CURRENT_KEY]),
            keys(&[CURRENT_KEY, ROTATED_KEY]),
        ]);
        verifier.refetch_interval = Duration::ZERO;

        verifier
            .verify(&signed_jwt(CURRENT_KEY, &claims()))
            .await
            .unwrap();
        verifier
            .verify(&signed_jwt(ROTATED_KEY, &claims()))
            .await
            .unwrap();

        assert_eq!(fetches.load(Ordering::SeqCst), 2);
    }

    #[tokio::test]
    async fn unknown_key_inside_the_refetch_interval_is_invalid() {
        let (verifier, fetches) = verifier(vec![keys(&[CURRENT_KEY])]);

        verifier
            .verify(&signed_jwt(CURRENT_KEY, &claims()))
            .await
            .unwrap();
        let err = verifier
            .verify(&signed_jwt(ROTATED_KEY, &claims()))
            .await
            .unwrap_err();

        assert!(matches!(
            err,
            IdTokenVerifyError::InvalidToken(InvalidIdToken::UnknownKey(Some(ref kid))) if kid == ROTATED_KEY
        ));
        assert_eq!(fetches.load(Ordering::SeqCst), 1);
    }

    #[tokio::test]
    async fn failed_fetch_makes_keys_unavailable() {
        let (verifier, _) = verifier(vec![unavailable()]);

        let err = verifier
            .verify(&signed_jwt(CURRENT_KEY, &claims()))
            .await
            .unwrap_err();

        assert!(matches!(err, IdTokenVerifyError::KeysUnavailable(_)));
    }

    #[tokio::test]
    async fn cached_keys_stay_in_use_after_a_failed_refetch() {
        let (mut verifier, fetches) = verifier(vec![keys(&[CURRENT_KEY]), unavailable()]);
        verifier.refetch_interval = Duration::ZERO;

        verifier
            .verify(&signed_jwt(CURRENT_KEY, &claims()))
            .await
            .unwrap();
        let err = verifier
            .verify(&signed_jwt(ROTATED_KEY, &claims()))
            .await
            .unwrap_err();
        verifier
            .verify(&signed_jwt(CURRENT_KEY, &claims()))
            .await
            .unwrap();

        assert!(matches!(err, IdTokenVerifyError::KeysUnavailable(_)));
        assert_eq!(fetches.load(Ordering::SeqCst), 2);
    }

    #[tokio::test]
    async fn keys_are_fetched_again_after_their_max_age() {
        let (verifier, fetches) = verifier(vec![
            Ok(IdTokenKeys::new(
                test_jwk_set(&[CURRENT_KEY]),
                Duration::ZERO,
            )),
            keys(&[CURRENT_KEY]),
        ]);

        verifier
            .verify(&signed_jwt(CURRENT_KEY, &claims()))
            .await
            .unwrap();
        verifier
            .verify(&signed_jwt(CURRENT_KEY, &claims()))
            .await
            .unwrap();

        assert_eq!(fetches.load(Ordering::SeqCst), 2);
    }

    #[tokio::test]
    async fn google_keys_last_for_the_cache_max_age() {
        let keys_endpoint = StubServer::start(vec![StubResponse::json(
            "200 OK",
            serde_json::to_string(&test_jwk_set(&[CURRENT_KEY])).unwrap(),
        )
        .with_header(
            "cache-control",
            "public, max-age=24948, must-revalidate, no-transform",
        )])
        .await;
        let google_keys = GoogleKeys {
            client: reqwest::Client::new(),
            url: format!("{}/oauth2/v3/certs", keys_endpoint.url),
        };

        let fetched = google_keys.fetch_keys().await.unwrap();

        assert_eq!(fetched.max_age, Duration::from_secs(24948));
        assert!(fetched.keys.find(CURRENT_KEY).is_some());
        assert_eq!(
            keys_endpoint.received()[0].request_line,
            "GET /oauth2/v3/certs HTTP/1.1"
        );
    }
}

#[cfg(all(test, not(any(feature = "jwt-aws-lc-rs", feature = "jwt-rust-crypto"))))]
mod missing_provider_tests {
    use super::*;

    struct NoKeys;

    #[async_trait]
    impl IdTokenKeysSource for NoKeys {
        async fn fetch_keys(&self) -> crate::error::Result<IdTokenKeys> {
            Ok(IdTokenKeys::new(
                JwkSet { keys: vec![] },
                Duration::from_secs(3600),
            ))
        }
    }

    #[tokio::test]
    async fn verifying_without_a_crypto_provider_makes_keys_unavailable() {
        let verifier = IdTokenVerifier::with_keys_source(
            IdTokenAudience::new("https://orders-abc123-ew.a.run.app"),
            NoKeys,
        );

        let err = verifier
            .verify("header.claims.signature")
            .await
            .unwrap_err();

        assert!(matches!(
            err,
            IdTokenVerifyError::KeysUnavailable(ref error)
                if matches!(error.kind(), ErrorKind::JwtCryptoProviderMissing)
        ));
    }
}
