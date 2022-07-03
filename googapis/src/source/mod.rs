use std::{
    convert::TryFrom,
    time::{Duration, Instant},
};

use async_trait::async_trait;

mod credentials;
mod metadata;

use credentials::{from_env_var, from_well_known_file};
use metadata::from_metadata;

pub use credentials::{from_file, from_json};

pub type BoxSource = Box<dyn Source + Send + Sync + 'static>;

#[async_trait]
pub trait Source {
    async fn token(&self) -> crate::Result<Token>;
}

// Looks for credentials in the following places, preferring the first location found:
// - A JSON file whose path is specified by the `GOOGLE_APPLICATION_CREDENTIALS` environment variable.
// - A JSON file in a location known to the gcloud command-line tool.
// - On Google Compute Engine, it fetches credentials from the metadata server.
pub async fn find_default(scopes: &[String]) -> crate::Result<BoxSource> {
    if let Some(src) = from_env_var(scopes)? {
        return Ok(src.into());
    }
    if let Some(src) = from_well_known_file(scopes)? {
        return Ok(src.into());
    }
    if let Some(src) = from_metadata(scopes).await? {
        return Ok(src.into());
    }
    Err(crate::ErrorKind::TokenSource.into())
}

#[derive(Debug)]
pub struct Token {
    pub type_: String,
    pub token: String,
    pub expiry: std::time::Instant,
}

impl Token {
    const EXPIRY_DELTA: Duration = Duration::from_secs(10);

    pub fn expired(&self, now: Instant) -> bool {
        self.expiry
            .checked_duration_since(now)
            .map(|d| d < Self::EXPIRY_DELTA)
            .unwrap_or(true)
    }
}

impl TryFrom<TokenResponse> for Token {
    type Error = crate::Error;

    fn try_from(v: TokenResponse) -> Result<Self, Self::Error> {
        if v.token_type.is_empty() || v.access_token.is_empty() || v.expires_in == 0 {
            Err(crate::ErrorKind::TokenData.into())
        } else {
            Ok(Token {
                type_: v.token_type,
                token: v.access_token,
                expiry: Instant::now() + Duration::from_secs(v.expires_in),
            })
        }
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct TokenResponse {
    token_type: String,
    access_token: String,
    expires_in: u64,
}

impl TryFrom<&str> for TokenResponse {
    type Error = crate::Error;

    fn try_from(v: &str) -> Result<Self, Self::Error> {
        let resp = serde_json::from_str(v).map_err(crate::ErrorKind::TokenJson)?;
        Ok(resp)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_token_expired() {
        let t = Token {
            type_: String::new(),
            token: String::new(),
            expiry: Instant::now(),
        };
        assert!(t.expired(Instant::now()));
        assert!(t.expired(Instant::now() - Duration::from_secs(5)));
        assert!(!t.expired(Instant::now() - Duration::from_secs(30)));
    }

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
            access_token: "secret".into(),
            expires_in: 1,
        },
        false;

        test_token_try_from_access_token,
        TokenResponse {
            token_type: "type".into(),
            access_token: String::new(),
            expires_in: 1,
        },
        false;

        test_token_try_from_expires_in,
        TokenResponse {
            token_type: "type".into(),
            access_token: "secret".into(),
            expires_in: 0,
        },
        false;

        test_token_try_from_ok,
        TokenResponse {
            token_type: "type".into(),
            access_token: "secret".into(),
            expires_in: 1,
        },
        true;
    );
}
