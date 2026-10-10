//! Service-to-service authentication with Google ID tokens, checked end to end against
//! Google: mints an ID token of a service account, verifies it with Google's keys, and
//! calls a local axum service protected by the verifying layer.
//!
//! ```sh
//! ID_TOKEN_SERVICE_ACCOUNT=invoker@my-project.iam.gserviceaccount.com \
//! ID_TOKEN_AUDIENCE=https://orders-abc123-ew.a.run.app \
//! cargo run -p id-token-example
//! ```
//!
//! Credentials that hold a service account (a key file, the metadata server, an
//! impersonated or federated account) mint as that account, which must be
//! `ID_TOKEN_SERVICE_ACCOUNT`. User credentials from `gcloud auth application-default
//! login` impersonate `ID_TOKEN_SERVICE_ACCOUNT` instead, which needs
//! `roles/iam.serviceAccountOpenIdTokenCreator` on it.

use std::sync::Arc;

use axum::routing::get;
use axum::Router;
use gcloud_sdk::error::ErrorKind;
use gcloud_sdk::id_token_verify::{
    IdTokenVerifier, IdTokenVerifyError, InvalidIdToken, PrincipalEmail, VerifiedIdToken,
};
use gcloud_sdk::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let subscriber = tracing_subscriber::fmt()
        .with_env_filter("gcloud_sdk=info")
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    let (Ok(service_account), Ok(audience)) = (
        std::env::var("ID_TOKEN_SERVICE_ACCOUNT"),
        std::env::var("ID_TOKEN_AUDIENCE"),
    ) else {
        println!("Skipped: set ID_TOKEN_SERVICE_ACCOUNT and ID_TOKEN_AUDIENCE to run this check.");
        return Ok(());
    };
    let expected_email = PrincipalEmail::new(service_account.clone());
    let service_account = ServiceAccountEmail::new(service_account);
    let audience = IdTokenAudience::new(audience);

    // Calling side: ID tokens for the audience, cached and refreshed before they expire.
    let source = match IdTokenSource::new(audience.clone(), TokenSourceType::Default).await {
        Err(err) if matches!(err.kind(), ErrorKind::IdTokenNeedsImpersonation(_)) => {
            println!(
                "Default credentials hold no service account, impersonating {service_account}"
            );
            IdTokenSource::impersonating(
                audience.clone(),
                service_account.clone(),
                TokenSourceType::Default,
            )
            .await?
        }
        other => other?,
    };
    let id_tokens = Arc::new(GoogleAuthTokenGenerator::from_source(source));
    let id_token = id_tokens.create_token().await?;

    // Receiving side: Google's keys, the audience and the token's lifetime are checked.
    let verifier = Arc::new(IdTokenVerifier::new(audience.clone())?);
    let verified = verifier.verify(id_token.token.as_sensitive_str()).await?;
    check(
        "token verifies for its audience, with the service account as the principal",
        verified.verified_email() == Some(&expected_email),
    )?;
    let other_audience = IdTokenVerifier::new(IdTokenAudience::new("https://other.invalid"))?;
    check(
        "token is refused for another audience",
        matches!(
            other_audience
                .verify(id_token.token.as_sensitive_str())
                .await,
            Err(IdTokenVerifyError::InvalidToken(
                InvalidIdToken::WrongAudience
            ))
        ),
    )?;

    // An axum service that accepts only verified tokens, on loopback.
    let app = Router::new()
        .route("/whoami", get(whoami))
        .layer(VerifyIdTokenLayer::new(verifier));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await?;
    let url = format!("http://{}/whoami", listener.local_addr()?);
    tokio::spawn(async move { axum::serve(listener, app).await });

    let anonymous = reqwest::Client::new().get(&url).send().await?;
    check(
        "call without a token is refused with 401",
        anonymous.status() == reqwest::StatusCode::UNAUTHORIZED,
    )?;

    let client = reqwest_middleware::ClientBuilder::new(reqwest::Client::new())
        .with(GoogleAuthReqwestMiddleware::new(id_tokens))
        .build();
    let response = client.get(&url).send().await?;
    check(
        "call with a token is answered with 200",
        response.status() == reqwest::StatusCode::OK,
    )?;
    check(
        "handler sees the service account email",
        response.text().await? == expected_email.to_string(),
    )?;

    Ok(())
}

async fn whoami(caller: VerifiedIdToken) -> String {
    caller
        .verified_email()
        .map(ToString::to_string)
        .unwrap_or_default()
}

fn check(name: &str, passed: bool) -> Result<(), Box<dyn std::error::Error>> {
    if passed {
        println!("PASS {name}");
        Ok(())
    } else {
        println!("FAIL {name}");
        Err(format!("check failed: {name}").into())
    }
}
