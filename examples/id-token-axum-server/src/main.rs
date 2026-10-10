//! An axum service that accepts only requests carrying a Google-signed ID token for its
//! own audience, from an allowlist of callers.
//!
//! ```sh
//! ID_TOKEN_AUDIENCE=https://orders-abc123-ew.a.run.app \
//! ALLOWED_CALLERS=invoker@my-project.iam.gserviceaccount.com \
//! cargo run -p id-token-axum-server-example
//! ```
//!
//! `ID_TOKEN_AUDIENCE` is the audience callers mint their tokens for: on Cloud Run, the
//! service URL. `ALLOWED_CALLERS` is a comma-separated list of service account emails.
//! `PORT` defaults to 8080, the port Cloud Run sets.

use std::sync::Arc;

use axum::routing::get;
use axum::Router;
use gcloud_sdk::id_token_verify::{
    IdTokenVerifier, PrincipalEmail, VerifiedIdToken, VerifyIdTokenLayer,
};
use gcloud_sdk::IdTokenAudience;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let audience = IdTokenAudience::new(std::env::var("ID_TOKEN_AUDIENCE")?);
    let allowed_callers: Vec<PrincipalEmail> = std::env::var("ALLOWED_CALLERS")?
        .split(',')
        .map(|email| PrincipalEmail::new(email.trim()))
        .collect();
    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());

    let verifier = Arc::new(IdTokenVerifier::new(audience)?);
    let app =
        Router::new()
            .route("/", get(whoami))
            .layer(VerifyIdTokenLayer::new(verifier).authorize(move |token| {
                token
                    .verified_email()
                    .is_some_and(|email| allowed_callers.contains(email))
            }));

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}")).await?;
    println!("Listening on {}", listener.local_addr()?);
    axum::serve(listener, app).await?;
    Ok(())
}

async fn whoami(caller: VerifiedIdToken) -> String {
    let email = caller
        .verified_email()
        .map(ToString::to_string)
        .unwrap_or_default();
    println!(
        "Request from {email}, token expires at {}",
        caller.expires_at()
    );
    email
}
