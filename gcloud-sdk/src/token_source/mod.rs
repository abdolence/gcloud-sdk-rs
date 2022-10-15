use std::convert::TryFrom;
use std::fmt::Debug;
use std::ops::Add;
use std::path::PathBuf;

use async_trait::async_trait;
use chrono::prelude::*;
use secret_vault_value::SecretValue;

pub mod auth_token_generator;
pub mod credentials;
pub mod metadata;

pub use credentials::{from_env_var, from_well_known_file};
use metadata::from_metadata;

pub use credentials::{from_file, from_json};
use tracing::*;

mod ext_creds_source;

pub type BoxSource = Box<dyn Source + Send + Sync + 'static>;

#[async_trait]
pub trait Source {
    async fn token(&self) -> crate::error::Result<Token>;
}

pub async fn create_source(
    token_source_type: TokenSourceType,
    token_scopes: Vec<String>,
) -> crate::error::Result<BoxSource> {
    match token_source_type {
        TokenSourceType::Default => Ok(find_default(&token_scopes).await?),
        TokenSourceType::Json(json) => Ok(from_json(json.as_bytes(), &token_scopes)?.into()),
        TokenSourceType::File(path) => Ok(from_file(path, &token_scopes)?.into()),
        TokenSourceType::MetadataServer => {
            if let Some(src) = from_metadata(&token_scopes, "default".to_string()).await? {
                Ok(src.into())
            } else {
                Err(crate::error::ErrorKind::TokenSource.into())
            }
        }
        TokenSourceType::MetadataServerWithAccount(account) => {
            if let Some(src) = from_metadata(&token_scopes, account).await? {
                Ok(src.into())
            } else {
                Err(crate::error::ErrorKind::TokenSource.into())
            }
        }
    }
}

// Looks for credentials in the following places, preferring the first location found:
// - A JSON file whose path is specified by the `GOOGLE_APPLICATION_CREDENTIALS` environment variable.
// - A JSON file in a location known to the gcloud command-line tool.
// - On Google Compute Engine, it fetches credentials from the metadata server.
pub async fn find_default(token_scopes: &[String]) -> crate::error::Result<BoxSource> {
    debug!("Finding default token for scopes: {:?}", token_scopes);

    if let Some(src) = from_env_var(token_scopes)? {
        debug!("Creating token based on environment variable: GOOGLE_APPLICATION_CREDENTIALS");
        return Ok(src.into());
    }
    if let Some(src) = from_well_known_file(token_scopes)? {
        debug!("Creating token based on standard config files such as application_default_credentials.json");
        return Ok(src.into());
    }
    if let Some(src) = from_metadata(token_scopes, "default".to_string()).await? {
        debug!("Creating token based on metadata server");
        return Ok(src.into());
    }
    warn!("None of the possible sources detected for Google OAuth token");
    Err(crate::error::ErrorKind::TokenSource.into())
}

#[derive(Debug, Clone)]
pub struct Token {
    pub type_: String,
    pub token: SecretValue,
    pub expiry: DateTime<Utc>,
}

impl Token {
    pub async fn generate() -> crate::error::Result<Token> {
        let token_source: BoxSource =
            create_source(TokenSourceType::Default, crate::GCP_DEFAULT_SCOPES.clone()).await?;
        token_source.token().await
    }

    pub async fn generate_for_scopes(
        token_source_type: TokenSourceType,
        token_scopes: Vec<String>,
    ) -> crate::error::Result<Token> {
        let token_source: BoxSource = create_source(token_source_type, token_scopes).await?;
        token_source.token().await
    }

    pub fn header_value(&self) -> String {
        format!("{} {}", self.type_, self.token.as_sensitive_str())
    }
}

impl TryFrom<TokenResponse> for Token {
    type Error = crate::error::Error;

    fn try_from(v: TokenResponse) -> Result<Self, Self::Error> {
        if v.token_type.is_empty()
            || v.access_token.as_sensitive_bytes().is_empty()
            || v.expires_in == 0
        {
            Err(crate::error::ErrorKind::TokenData.into())
        } else {
            Ok(Token {
                type_: v.token_type,
                token: v.access_token,
                expiry: Utc::now().add(chrono::Duration::seconds(v.expires_in.try_into().unwrap())),
            })
        }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct TokenResponse {
    token_type: String,
    access_token: SecretValue,
    expires_in: u64,
}

impl TryFrom<&str> for TokenResponse {
    type Error = crate::error::Error;

    fn try_from(v: &str) -> Result<Self, Self::Error> {
        let resp = serde_json::from_str(v).map_err(crate::error::ErrorKind::TokenJson)?;
        Ok(resp)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! test_token_try_from {
        () => {};
        ($name:ident, $in:expr, $ok:expr; $($tt:tt)*) => {
            #[test]
            fn $name() {
                assert_eq!(Token::try_from($in).is_ok(), $ok)
            }
            test_token_try_from!($($tt)*);
        };
    }

    test_token_try_from!(
        test_token_try_from_token_type,
        TokenResponse {
            token_type: String::new(),
            access_token: TokenValue("secret".into()),
            expires_in: 1,
        },
        false;

        test_token_try_from_access_token,
        TokenResponse {
            token_type: "type".into(),
            access_token: TokenValue(String::new()),
            expires_in: 1,
        },
        false;

        test_token_try_from_expires_in,
        TokenResponse {
            token_type: "type".into(),
            access_token: TokenValue("secret".into()),
            expires_in: 0,
        },
        false;

        test_token_try_from_ok,
        TokenResponse {
            token_type: "type".into(),
            access_token: TokenValue("secret".into()),
            expires_in: 1,
        },
        true;
    );
}

#[derive(Clone, Debug)]
pub enum TokenSourceType {
    Default,
    Json(String),
    File(PathBuf),
    MetadataServer,
    MetadataServerWithAccount(String),
}
