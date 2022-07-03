//! This library provides auto-renewed tokens for GCP service authentication.
//!
//! # Example
//! ```no_run
//! use gcloud_sdk::Token;
//!
//! let token = Token::new().unwrap();
//! println!("authorization: {}", token.header_value().unwrap());
//! ```

use std::{path::PathBuf, sync::Arc, time::Instant};
use tokio::sync::Mutex;

use crate::source::{self, find_default, from_file, from_json, BoxSource};

/// The access token is acquired at the required timing.
/// It will check the expiration date at the time of request
/// and update it if it has expired.
pub struct Token {
    inner: Mutex<Inner>,
}

impl Token {
    /// Create a token using the default settings.
    pub async fn new() -> crate::Result<Self> {
        Builder::new().build().await
    }

    /// Get the value of the authorization header.
    /// If it has expired, it will be updated automatically.
    pub async fn header_value(&self) -> crate::Result<Arc<String>> {
        let mut inner = self.inner.lock().await;
        let v = match inner.curr {
            Some(ref c) if !c.token.expired(Instant::now()) => c.header.clone(),
            _ => {
                let token = inner.source.token().await?;
                let cache = Cache::from(token);
                let ret = cache.header.clone();
                inner.curr = Some(cache);
                ret
            }
        };
        Ok(v)
    }
}

impl From<Inner> for Token {
    fn from(v: Inner) -> Self {
        Self {
            inner: Mutex::new(v),
        }
    }
}

struct Inner {
    source: BoxSource,
    curr: Option<Cache>,
}

impl From<BoxSource> for Inner {
    fn from(v: BoxSource) -> Self {
        Self {
            source: v,
            curr: None,
        }
    }
}

struct Cache {
    token: source::Token,
    header: Arc<String>,
}

impl From<source::Token> for Cache {
    fn from(v: source::Token) -> Self {
        let header = format!("{} {}", v.type_, v.token);
        Self {
            token: v,
            header: Arc::new(header),
        }
    }
}

enum SourceType {
    Default,
    Json(String),
    File(PathBuf),
}

impl Default for SourceType {
    fn default() -> Self {
        Self::Default
    }
}

/// Builder configures the credentials source or scopes of OAuth 2.0.
pub struct Builder {
    type_: SourceType,
    scopes: Vec<String>,
}

impl Default for Builder {
    fn default() -> Self {
        Self {
            type_: Default::default(),
            scopes: vec!["https://www.googleapis.com/auth/cloud-platform".into()],
        }
    }
}

impl Builder {
    /// Create a builder with the default settings.
    pub fn new() -> Self {
        Self::default()
    }

    /// Configure the credential json data.
    pub fn json(mut self, json: impl AsRef<str>) -> Self {
        self.type_ = SourceType::Json(json.as_ref().to_owned());
        self
    }

    /// Configure the credential file path.
    pub fn file(mut self, path: impl Into<PathBuf>) -> Self {
        self.type_ = SourceType::File(path.into());
        self
    }

    /// Configure the scope of OAuth 2.0.
    pub fn scopes<T: AsRef<str>>(mut self, scopes: &[T]) -> Self {
        self.scopes = scopes
            .iter()
            .map(AsRef::as_ref)
            .map(ToOwned::to_owned)
            .collect();
        self
    }

    /// Create a token using the builder settings.
    /// This library has not yet made API access for authentication.
    pub async fn build(self) -> crate::Result<Token> {
        use SourceType::*;
        let source: BoxSource = match self.type_ {
            Default => find_default(&self.scopes).await?,
            Json(json) => from_json(json.as_bytes(), &self.scopes)?.into(),
            File(path) => from_file(path, &self.scopes)?.into(),
        };
        Ok(Token::from(Inner::from(source)))
    }
}
