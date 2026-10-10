//! Calls a service protected by Google ID tokens, such as a Cloud Run service, with a
//! `reqwest-middleware` client that attaches an ID token for the service URL.
//!
//! ```sh
//! SERVICE_URL=https://orders-abc123-ew.a.run.app \
//! cargo run -p id-token-reqwest-client-example
//! ```
//!
//! User credentials from `gcloud auth application-default login` cannot mint ID tokens
//! themselves: set `IMPERSONATE_SERVICE_ACCOUNT` to a service account on which they have
//! `roles/iam.serviceAccountOpenIdTokenCreator`.

use gcloud_sdk::{
    GoogleAuthReqwestMiddleware, GoogleAuthTokenGenerator, IdTokenAudience, IdTokenSource,
    ServiceAccountEmail, TokenSourceType,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let service_url = std::env::var("SERVICE_URL")?;
    let audience = IdTokenAudience::new(service_url.clone());

    let source = match std::env::var("IMPERSONATE_SERVICE_ACCOUNT") {
        Ok(service_account) => {
            IdTokenSource::impersonating(
                audience,
                ServiceAccountEmail::new(service_account),
                TokenSourceType::Default,
            )
            .await?
        }
        Err(_) => IdTokenSource::new(audience, TokenSourceType::Default).await?,
    };

    let client = reqwest_middleware::ClientBuilder::new(reqwest::Client::new())
        .with(GoogleAuthReqwestMiddleware::new(
            GoogleAuthTokenGenerator::from_source(source),
        ))
        .build();

    let response = client.get(&service_url).send().await?;
    println!("{}: {}", response.status(), response.text().await?);
    Ok(())
}
