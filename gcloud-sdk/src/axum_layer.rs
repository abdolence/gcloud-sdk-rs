//! Protects an axum service with Google ID tokens: [`VerifyIdTokenLayer`] verifies the
//! bearer token of each request with an [`IdTokenVerifier`], and handlers read the
//! verified claims with the [`VerifiedIdToken`] extractor.
//!
//! The layer is a plain tower layer, so it also wraps a tonic server.
//!
//! ```ignore
//! let verifier = Arc::new(IdTokenVerifier::new(IdTokenAudience::new(
//!     "https://orders-abc123-ew.a.run.app",
//! ))?);
//! let app = Router::new()
//!     .route("/orders", post(create_order))
//!     .layer(VerifyIdTokenLayer::new(verifier).authorize(|token| {
//!         token.verified_email() == Some(&PrincipalEmail::new("caller@my-project.iam.gserviceaccount.com"))
//!     }));
//!
//! async fn create_order(token: VerifiedIdToken) -> String {
//!     format!("hello, {}", token.subject())
//! }
//! ```

use std::convert::Infallible;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};

use axum_core::extract::FromRequestParts;
use futures::future::BoxFuture;
use futures::{Future, FutureExt};
use hyper::header::{HeaderMap, HeaderValue, AUTHORIZATION, WWW_AUTHENTICATE};
use hyper::http::request::Parts;
use hyper::{Request, Response, StatusCode};
use tower::{BoxError, Service};
use tower_layer::Layer;
use tracing::*;

use crate::id_token_verify::{IdTokenVerifier, IdTokenVerifyError, VerifiedIdToken};

type Authorize =
    dyn Fn(&VerifiedIdToken) -> BoxFuture<'static, Result<bool, BoxError>> + Send + Sync;

/// A tower layer that lets a request through only with a valid Google ID token in its
/// `authorization: Bearer` header, and puts the token's claims into the request's
/// extensions for the [`VerifiedIdToken`] extractor.
///
/// Requests it refuses never reach the inner service. They get an empty response:
/// - 401 with `www-authenticate: Bearer` when there is no bearer token;
/// - 401 with `www-authenticate: Bearer error="invalid_token"` when the token does not
///   verify;
/// - 503 when the signing keys cannot be fetched, or when
///   [`authorize_async`](Self::authorize_async) fails;
/// - 403 when the token verifies but [`authorize`](Self::authorize) or
///   [`authorize_async`](Self::authorize_async) refuses it.
#[derive(Clone)]
pub struct VerifyIdTokenLayer {
    verifier: Arc<IdTokenVerifier>,
    authorize: Option<Arc<Authorize>>,
}

impl VerifyIdTokenLayer {
    /// A layer that admits every token `verifier` accepts.
    pub fn new(verifier: Arc<IdTokenVerifier>) -> Self {
        Self {
            verifier,
            authorize: None,
        }
    }

    /// Admits only the verified tokens for which `authorize` returns true, such as those
    /// of an allowlist of callers; the others get 403. Replaces any earlier `authorize` or
    /// [`authorize_async`](Self::authorize_async).
    pub fn authorize(
        self,
        authorize: impl Fn(&VerifiedIdToken) -> bool + Send + Sync + 'static,
    ) -> Self {
        self.authorize_async(move |token| std::future::ready(Ok::<_, Infallible>(authorize(token))))
    }

    /// Admits only the verified tokens for which the future returned by `check`
    /// resolves to `Ok(true)`, for checks that need async calls, such as an allowlist kept
    /// in a database. `Ok(false)` gets 403. `Err` gets 503: the error is logged with
    /// `warn!` and never goes into the response. Replaces any earlier
    /// [`authorize`](Self::authorize) or `authorize_async`.
    ///
    /// The future is `'static`, so it cannot borrow the token: copy what the check needs,
    /// such as the email, before the `async move` block.
    pub fn authorize_async<F, Fut, E>(mut self, check: F) -> Self
    where
        F: Fn(&VerifiedIdToken) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<bool, E>> + Send + 'static,
        E: Into<BoxError>,
    {
        self.authorize = Some(Arc::new(move |token: &VerifiedIdToken| {
            let verdict = check(token);
            async move { verdict.await.map_err(Into::into) }.boxed()
        }));
        self
    }

    async fn admit(&self, headers: &HeaderMap) -> Result<VerifiedIdToken, Refusal> {
        let token = bearer_token(headers).ok_or(Refusal::MissingToken)?;
        let verified = self.verifier.verify(token).await.map_err(|err| match err {
            IdTokenVerifyError::InvalidToken(reason) => {
                debug!(%reason, "Refusing a request with an invalid ID token");
                Refusal::InvalidToken
            }
            IdTokenVerifyError::KeysUnavailable(error) => {
                warn!(%error, "Refusing a request: ID token signing keys are unavailable");
                Refusal::KeysUnavailable
            }
        })?;
        let Some(authorize) = &self.authorize else {
            return Ok(verified);
        };
        match authorize(&verified).await {
            Ok(true) => Ok(verified),
            Ok(false) => Err(Refusal::Forbidden),
            Err(error) => {
                warn!(%error, "Refusing a request: the authorize_async check failed");
                Err(Refusal::AuthorizationUnavailable)
            }
        }
    }
}

impl<S> Layer<S> for VerifyIdTokenLayer {
    type Service = VerifyIdToken<S>;

    fn layer(&self, inner: S) -> Self::Service {
        VerifyIdToken {
            inner,
            layer: self.clone(),
        }
    }
}

/// The service made by [`VerifyIdTokenLayer`].
#[derive(Clone)]
pub struct VerifyIdToken<S> {
    inner: S,
    layer: VerifyIdTokenLayer,
}

impl<S, ReqBody, ResBody> Service<Request<ReqBody>> for VerifyIdToken<S>
where
    S: Service<Request<ReqBody>, Response = Response<ResBody>> + Clone + Send + 'static,
    S::Future: Send + 'static,
    ReqBody: Send + 'static,
    ResBody: Default + Send + 'static,
{
    type Response = Response<ResBody>;
    type Error = S::Error;
    type Future =
        Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + 'static>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut req: Request<ReqBody>) -> Self::Future {
        let layer = self.layer.clone();
        // tower's idiom for a `Clone` inner service: the instance polled ready goes into
        // the future, and the service keeps a fresh clone.
        let clone = self.inner.clone();
        let mut inner = std::mem::replace(&mut self.inner, clone);

        Box::pin(async move {
            match layer.admit(req.headers()).await {
                Ok(verified) => {
                    req.extensions_mut().insert(verified);
                    inner.call(req).await
                }
                Err(refusal) => Ok(refusal.into()),
            }
        })
    }
}

/// Why a request was refused.
enum Refusal {
    MissingToken,
    InvalidToken,
    KeysUnavailable,
    AuthorizationUnavailable,
    Forbidden,
}

impl<B: Default> From<Refusal> for Response<B> {
    fn from(refusal: Refusal) -> Self {
        let (status, challenge) = match refusal {
            Refusal::MissingToken => (StatusCode::UNAUTHORIZED, Some("Bearer")),
            Refusal::InvalidToken => (
                StatusCode::UNAUTHORIZED,
                Some(r#"Bearer error="invalid_token""#),
            ),
            Refusal::KeysUnavailable | Refusal::AuthorizationUnavailable => {
                (StatusCode::SERVICE_UNAVAILABLE, None)
            }
            Refusal::Forbidden => (StatusCode::FORBIDDEN, None),
        };
        let mut response = Response::new(B::default());
        *response.status_mut() = status;
        if let Some(challenge) = challenge {
            response
                .headers_mut()
                .insert(WWW_AUTHENTICATE, HeaderValue::from_static(challenge));
        }
        response
    }
}

/// The token of an `authorization: Bearer <token>` header, with the scheme matched
/// case-insensitively.
fn bearer_token(headers: &HeaderMap) -> Option<&str> {
    let (scheme, token) = headers.get(AUTHORIZATION)?.to_str().ok()?.split_once(' ')?;
    let token = token.trim();
    (scheme.eq_ignore_ascii_case("bearer") && !token.is_empty()).then_some(token)
}

/// The claims of the token [`VerifyIdTokenLayer`] verified for this request.
///
/// Rejects with 500 when the route is not behind a [`VerifyIdTokenLayer`], since that
/// is a mistake in the server, not in the request.
impl<S: Send + Sync> FromRequestParts<S> for VerifiedIdToken {
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        parts
            .extensions
            .get::<VerifiedIdToken>()
            .cloned()
            .ok_or(StatusCode::INTERNAL_SERVER_ERROR)
    }
}

// The tests sign tokens, which needs a crypto provider.
#[cfg(all(test, any(feature = "jwt-aws-lc-rs", feature = "jwt-rust-crypto")))]
mod tests {
    use super::*;
    use crate::error::ErrorKind;
    use crate::id_token_verify::{IdTokenKeys, IdTokenKeysSource, PrincipalEmail};
    use crate::test_support::{signed_jwt, test_jwk_set};
    use crate::IdTokenAudience;
    use async_trait::async_trait;
    use jiff::{SignedDuration, Timestamp};
    use std::time::Duration;

    const AUDIENCE: &str = "https://orders-abc123-ew.a.run.app";
    const CALLER: &str = "caller@my-project.iam.gserviceaccount.com";
    const KEY_ID: &str = "current-key";

    enum TestKeys {
        Published,
        Unavailable,
    }

    #[async_trait]
    impl IdTokenKeysSource for TestKeys {
        async fn fetch_keys(&self) -> crate::error::Result<IdTokenKeys> {
            match self {
                Self::Published => Ok(IdTokenKeys::new(
                    test_jwk_set(&[KEY_ID]),
                    Duration::from_secs(3600),
                )),
                Self::Unavailable => {
                    Err(ErrorKind::HttpStatus(reqwest::StatusCode::SERVICE_UNAVAILABLE).into())
                }
            }
        }
    }

    fn verifier(keys: TestKeys) -> Arc<IdTokenVerifier> {
        Arc::new(IdTokenVerifier::with_keys_source(
            IdTokenAudience::new(AUDIENCE),
            keys,
        ))
    }

    fn token_for(audience: &str) -> String {
        let now = Timestamp::now();
        let claims = serde_json::json!({
            "iss": "https://accounts.google.com",
            "aud": audience,
            "sub": "112233445566778899",
            "email": CALLER,
            "email_verified": true,
            "iat": now.as_second(),
            "exp": (now + SignedDuration::from_hours(1)).as_second(),
        });
        signed_jwt(KEY_ID, &claims)
    }

    fn request(authorization: Option<String>) -> Request<String> {
        let mut builder = Request::builder().uri("http://orders.internal/orders");
        if let Some(authorization) = authorization {
            builder = builder.header(AUTHORIZATION, authorization);
        }
        builder.body(String::new()).unwrap()
    }

    /// Answers 200 and copies the claims the layer passed on into the response's
    /// extensions, so that tests see what the inner service saw.
    #[derive(Clone)]
    struct EchoClaims;

    impl Service<Request<String>> for EchoClaims {
        type Response = Response<String>;
        type Error = Infallible;
        type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }

        fn call(&mut self, req: Request<String>) -> Self::Future {
            let claims = req.extensions().get::<VerifiedIdToken>().cloned();
            Box::pin(async move {
                let mut response = Response::new(String::new());
                if let Some(claims) = claims {
                    response.extensions_mut().insert(claims);
                }
                Ok(response)
            })
        }
    }

    async fn send(layer: VerifyIdTokenLayer, req: Request<String>) -> Response<String> {
        layer.layer(EchoClaims).call(req).await.unwrap()
    }

    #[tokio::test]
    async fn missing_token_is_unauthorized() {
        let response = send(
            VerifyIdTokenLayer::new(verifier(TestKeys::Published)),
            request(None),
        )
        .await;

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
        assert_eq!(
            response.headers().get(WWW_AUTHENTICATE),
            Some(&HeaderValue::from_static("Bearer"))
        );
        assert!(response.extensions().get::<VerifiedIdToken>().is_none());
    }

    #[tokio::test]
    async fn token_for_another_audience_is_unauthorized() {
        let token = token_for("https://billing-abc123-ew.a.run.app");

        let response = send(
            VerifyIdTokenLayer::new(verifier(TestKeys::Published)),
            request(Some(format!("Bearer {token}"))),
        )
        .await;

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
        assert_eq!(
            response.headers().get(WWW_AUTHENTICATE),
            Some(&HeaderValue::from_static(r#"Bearer error="invalid_token""#))
        );
        assert!(response.extensions().get::<VerifiedIdToken>().is_none());
    }

    #[tokio::test]
    async fn unavailable_keys_are_service_unavailable() {
        let response = send(
            VerifyIdTokenLayer::new(verifier(TestKeys::Unavailable)),
            request(Some(format!("Bearer {}", token_for(AUDIENCE)))),
        )
        .await;

        assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);
        assert!(response.extensions().get::<VerifiedIdToken>().is_none());
    }

    #[tokio::test]
    async fn refused_principal_is_forbidden() {
        let layer = VerifyIdTokenLayer::new(verifier(TestKeys::Published)).authorize(|token| {
            token.verified_email()
                == Some(&PrincipalEmail::new(
                    "billing@my-project.iam.gserviceaccount.com",
                ))
        });

        let response = send(
            layer,
            request(Some(format!("Bearer {}", token_for(AUDIENCE)))),
        )
        .await;

        assert_eq!(response.status(), StatusCode::FORBIDDEN);
        assert!(response.extensions().get::<VerifiedIdToken>().is_none());
    }

    #[tokio::test]
    async fn verified_claims_reach_the_inner_service() {
        let layer = VerifyIdTokenLayer::new(verifier(TestKeys::Published))
            .authorize(|token| token.verified_email() == Some(&PrincipalEmail::new(CALLER)));

        let response = send(
            layer,
            request(Some(format!("bearer {}", token_for(AUDIENCE)))),
        )
        .await;

        assert_eq!(response.status(), StatusCode::OK);
        let claims = response.extensions().get::<VerifiedIdToken>().unwrap();
        assert_eq!(claims.verified_email(), Some(&PrincipalEmail::new(CALLER)));
    }

    #[tokio::test]
    async fn principal_admitted_by_async_check_reaches_the_inner_service() {
        let layer =
            VerifyIdTokenLayer::new(verifier(TestKeys::Published)).authorize_async(|token| {
                let email = token.verified_email().cloned();
                async move {
                    tokio::task::yield_now().await;
                    Ok::<_, Infallible>(email == Some(PrincipalEmail::new(CALLER)))
                }
            });

        let response = send(
            layer,
            request(Some(format!("Bearer {}", token_for(AUDIENCE)))),
        )
        .await;

        assert_eq!(response.status(), StatusCode::OK);
        let claims = response.extensions().get::<VerifiedIdToken>().unwrap();
        assert_eq!(claims.verified_email(), Some(&PrincipalEmail::new(CALLER)));
    }

    #[tokio::test]
    async fn failed_async_check_is_service_unavailable() {
        let layer = VerifyIdTokenLayer::new(verifier(TestKeys::Published))
            .authorize_async(|_| async { Err::<bool, _>("the allowlist is unavailable") });

        let response = send(
            layer,
            request(Some(format!("Bearer {}", token_for(AUDIENCE)))),
        )
        .await;

        assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);
        assert!(response.headers().get(WWW_AUTHENTICATE).is_none());
        assert!(response.body().is_empty());
        assert!(response.extensions().get::<VerifiedIdToken>().is_none());
    }

    #[tokio::test]
    async fn extractor_reads_the_verified_claims() {
        let claims = verifier(TestKeys::Published)
            .verify(&token_for(AUDIENCE))
            .await
            .unwrap();
        let (mut parts, _) = request(None).into_parts();
        parts.extensions.insert(claims);

        let extracted = VerifiedIdToken::from_request_parts(&mut parts, &())
            .await
            .unwrap();

        assert_eq!(
            extracted.verified_email(),
            Some(&PrincipalEmail::new(CALLER))
        );
    }

    #[tokio::test]
    async fn extractor_outside_the_layer_is_an_internal_error() {
        let (mut parts, _) = request(None).into_parts();

        let rejection = VerifiedIdToken::from_request_parts(&mut parts, &())
            .await
            .unwrap_err();

        assert_eq!(rejection, StatusCode::INTERNAL_SERVER_ERROR);
    }
}
