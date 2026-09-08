use hyper::header::HeaderValue;
use jiff::{SignedDuration, Timestamp};
use tokio::sync::RwLock;

use crate::token_source::*;
use tracing::*;

/// A token together with its pre-validated `authorization` header value, so that
/// serving a cache hit never re-parses or re-allocates the header.
struct CachedToken {
    token: Token,
    authorization: HeaderValue,
}

impl CachedToken {
    fn from_token(token: Token) -> crate::error::Result<Self> {
        let mut authorization = HeaderValue::from_str(&token.header_value())?;
        authorization.set_sensitive(true);
        Ok(Self {
            token,
            authorization,
        })
    }
}

pub struct GoogleAuthTokenGenerator {
    token_source: BoxSource,
    cached_token: RwLock<Option<CachedToken>>,
}

impl GoogleAuthTokenGenerator {
    pub async fn new(
        token_source_type: TokenSourceType,
        token_scopes: Vec<String>,
    ) -> crate::error::Result<GoogleAuthTokenGenerator> {
        let token_source: BoxSource = create_source(token_source_type, token_scopes).await?;

        Ok(GoogleAuthTokenGenerator {
            token_source,
            cached_token: RwLock::new(None),
        })
    }

    pub async fn clear_cache(&self) {
        let mut write_state = self.cached_token.write().await;
        *write_state = None;
    }

    pub async fn create_token(&self) -> crate::error::Result<Token> {
        self.with_cached(|cached| cached.token.clone()).await
    }

    /// The `authorization` header value for the current token, already validated
    /// and marked sensitive; cloning it is a `Bytes` refcount bump, not an allocation.
    pub async fn authorization_header(&self) -> crate::error::Result<HeaderValue> {
        self.with_cached(|cached| cached.authorization.clone())
            .await
    }

    /// Runs the double-checked refresh and applies `pick` to the resulting cached
    /// token by reference, so a cache hit clones only what the caller asks for.
    async fn with_cached<R>(
        &self,
        pick: impl FnOnce(&CachedToken) -> R,
    ) -> crate::error::Result<R> {
        let now = Timestamp::now();

        {
            let read_state = self.cached_token.read().await;
            // Give a bit more time for network call
            if let Some(cached) = read_state.as_ref() {
                if cached
                    .token
                    .expiry
                    .gt(&now.add(SignedDuration::from_secs(15)))
                {
                    return Ok(pick(cached));
                }
            }
        }

        let mut write_token = self.cached_token.write().await;
        match write_token.as_ref() {
            Some(updated_cached) if updated_cached.token.expiry.gt(&now) => {
                Ok(pick(updated_cached))
            }
            _ => {
                let new_token = self.token_source.token().await?;
                debug!(
                    "Created a new Google OAuth token. Type: {}. Expiring: {}.",
                    new_token.token_type, new_token.expiry,
                );
                let new_cached = CachedToken::from_token(new_token)?;
                let result = pick(&new_cached);
                *write_token = Some(new_cached);
                Ok(result)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token_source::Source;
    use async_trait::async_trait;
    use secret_vault_value::SecretValue;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;

    struct CountingSource {
        calls: Arc<AtomicUsize>,
    }

    #[async_trait]
    impl Source for CountingSource {
        async fn token(&self) -> crate::error::Result<Token> {
            self.calls.fetch_add(1, Ordering::SeqCst);
            Ok(Token {
                token_type: "Bearer".to_string(),
                token: SecretValue::from("cached-token"),
                expiry: Timestamp::now() + SignedDuration::from_hours(1),
            })
        }
    }

    #[tokio::test]
    async fn authorization_header_reuses_cached_token() {
        let calls = Arc::new(AtomicUsize::new(0));
        let source = CountingSource {
            calls: calls.clone(),
        };
        let generator = GoogleAuthTokenGenerator::new(
            TokenSourceType::ExternalSource(Box::new(source)),
            vec![],
        )
        .await
        .unwrap();

        let first = generator.authorization_header().await.unwrap();
        let second = generator.authorization_header().await.unwrap();

        assert_eq!(first, second);
        assert!(first.is_sensitive());
        assert_eq!(calls.load(Ordering::SeqCst), 1);
    }
}
