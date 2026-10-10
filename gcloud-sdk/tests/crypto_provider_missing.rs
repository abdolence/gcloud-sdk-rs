//! Service account key credentials without a rustls crypto provider.
//!
//! The provider is process-global, so this file is its own test binary, and nothing in
//! it installs one.
#![cfg(not(feature = "auth-default-crypto"))]

use gcloud_sdk::error::ErrorKind;
use gcloud_sdk::{GoogleApi, GoogleAuthHeaders, GoogleAuthMiddleware, IdTokenAudience};

/// A service account key. The check fails before the key is parsed, so the key itself
/// does not need to be valid.
fn service_account_key() -> serde_json::Value {
    serde_json::json!({
        "type": "service_account",
        "project_id": "orders",
        "private_key_id": "orders-key",
        "private_key": "-----BEGIN PRIVATE KEY-----\nunused\n-----END PRIVATE KEY-----\n",
        "client_email": "invoker@orders.iam.gserviceaccount.com",
    })
}

/// A service account key file, as the Application Default Credentials.
fn key_file_as_adc() -> std::path::PathBuf {
    let path = std::env::temp_dir().join(format!(
        "gcloud-sdk-crypto-provider-missing-{}.json",
        std::process::id()
    ));
    std::fs::write(&path, service_account_key().to_string()).unwrap();
    std::env::set_var("GOOGLE_APPLICATION_CREDENTIALS", &path);
    path
}

#[tokio::test]
async fn service_account_key_without_a_crypto_provider_is_a_typed_error() {
    assert!(rustls::crypto::CryptoProvider::get_default().is_none());
    let key_file = key_file_as_adc();
    // A listener the channel's eager connect succeeds against; no request reaches it.
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let api_url = format!("http://{}", listener.local_addr().unwrap());

    let id_tokens =
        GoogleAuthHeaders::id_token_from_adc(&IdTokenAudience::new("https://orders.example")).await;
    let access_tokens = GoogleAuthHeaders::from_adc().await;
    let client: gcloud_sdk::error::Result<GoogleApi<GoogleAuthMiddleware>> =
        GoogleApi::from_function(|service| service, api_url, None).await;
    let key = GoogleAuthHeaders::from_service_account_key(
        service_account_key(),
        gcloud_sdk::GCP_DEFAULT_SCOPES.clone(),
    )
    .await;

    std::fs::remove_file(key_file).unwrap();
    assert!(matches!(
        id_tokens.unwrap_err().kind(),
        ErrorKind::CryptoProviderMissing
    ));
    assert!(matches!(
        access_tokens.unwrap_err().kind(),
        ErrorKind::CryptoProviderMissing
    ));
    assert!(matches!(
        client.err().unwrap().kind(),
        ErrorKind::CryptoProviderMissing
    ));
    assert!(matches!(
        key.err().unwrap().kind(),
        ErrorKind::CryptoProviderMissing
    ));
}
