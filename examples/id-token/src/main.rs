//! Mints a Google ID token for an audience and verifies it with Google's keys, using the
//! core API only.
//!
//! ```sh
//! ID_TOKEN_AUDIENCE=https://orders-abc123-ew.a.run.app \
//! ID_TOKEN_SERVICE_ACCOUNT=invoker@my-project.iam.gserviceaccount.com \
//! cargo run -p id-token-example
//! ```
//!
//! Credentials that hold a service account (a key file, the metadata server, an
//! impersonated or federated account) mint as that account. User credentials from
//! `gcloud auth application-default login` cannot, and impersonate
//! `ID_TOKEN_SERVICE_ACCOUNT` instead, which needs
//! `roles/iam.serviceAccountOpenIdTokenCreator` on it. When `ID_TOKEN_SERVICE_ACCOUNT`
//! is set, the verified email must be that account.

use gcloud_sdk::error::ErrorKind;
use gcloud_sdk::id_token_verify::{
    IdTokenVerifier, IdTokenVerifyError, InvalidIdToken, PrincipalEmail,
};
use gcloud_sdk::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let subscriber = tracing_subscriber::fmt()
        .with_env_filter("gcloud_sdk=info")
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    let Ok(audience) = std::env::var("ID_TOKEN_AUDIENCE") else {
        println!("Skipped: set ID_TOKEN_AUDIENCE to run this check.");
        return Ok(());
    };
    let audience = IdTokenAudience::new(audience);
    let service_account = std::env::var("ID_TOKEN_SERVICE_ACCOUNT").ok();

    let source = match IdTokenSource::new(audience.clone(), TokenSourceType::Default).await {
        Err(err) if matches!(err.kind(), ErrorKind::IdTokenNeedsImpersonation(_)) => {
            let Some(service_account) = &service_account else {
                return Err(err.into());
            };
            println!(
                "Default credentials hold no service account, impersonating {service_account}"
            );
            IdTokenSource::impersonating(
                audience.clone(),
                ServiceAccountEmail::new(service_account.clone()),
                TokenSourceType::Default,
            )
            .await?
        }
        other => other?,
    };
    let id_token = GoogleAuthTokenGenerator::from_source(source)
        .create_token()
        .await?;

    let verified = IdTokenVerifier::new(audience)?
        .verify(id_token.token.as_sensitive_str())
        .await?;
    let email = verified
        .verified_email()
        .map(ToString::to_string)
        .unwrap_or_default();
    println!("Verified email: {email}");
    println!("Verified audience: {}", verified.audience());
    println!("Expires at: {}", verified.expires_at());
    if let Some(service_account) = service_account {
        check(
            "the principal is the service account",
            verified.verified_email() == Some(&PrincipalEmail::new(service_account)),
        )?;
    }

    let other_audience = IdTokenVerifier::new(IdTokenAudience::new("https://other.invalid"))?;
    check(
        "the token is refused for another audience",
        matches!(
            other_audience
                .verify(id_token.token.as_sensitive_str())
                .await,
            Err(IdTokenVerifyError::InvalidToken(
                InvalidIdToken::WrongAudience
            ))
        ),
    )?;

    Ok(())
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
