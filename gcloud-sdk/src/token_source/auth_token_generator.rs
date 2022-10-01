use std::sync::Arc;

use chrono::prelude::*;
use tokio::sync::RwLock;

use crate::token_source::*;
use tracing::*;

pub struct GoogleAuthTokenGenerator {
    token_source_type: TokenSourceType,
    token_source: BoxSource,
    cached_token: Arc<RwLock<Option<Token>>>,
}

impl GoogleAuthTokenGenerator {
    pub async fn new(
        token_source_type: TokenSourceType,
        token_scopes: Vec<String>,
    ) -> crate::error::Result<GoogleAuthTokenGenerator> {
        let token_source: BoxSource =
            create_source(token_source_type.clone(), token_scopes).await?;

        Ok(GoogleAuthTokenGenerator {
            token_source_type,
            token_source,
            cached_token: Arc::new(RwLock::new(None)),
        })
    }

    pub async fn clear_cache(&self) {
        let mut write_state = self.cached_token.write().await;
        *write_state = None;
    }

    pub async fn create_token(&self) -> crate::error::Result<Token> {
        let existing_token: Option<Token> = {
            let read_state = self.cached_token.read().await;
            read_state.clone()
        };

        let now = Utc::now();

        match existing_token {
            Some(token) if token.expiry.gt(&now) => Ok(token),
            _ => {
                let new_token = {
                    let mut write_token = self.cached_token.write().await;

                    match write_token.as_ref() {
                        Some(updated_token) if updated_token.expiry.gt(&now) => {
                            updated_token.clone()
                        }
                        _ => {
                            let new_token = self.token_source.token().await?;
                            *write_token = Some(new_token.clone());
                            debug!(
                                "Created a new Google OAuth token. Type: {}. Expiring: {}. Source: {:?}",
                                new_token.type_,
                                new_token.expiry, self.token_source_type
                            );
                            new_token
                        }
                    }
                };
                Ok(new_token)
            }
        }
    }
}
