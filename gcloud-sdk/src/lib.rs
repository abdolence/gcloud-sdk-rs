//! # Google Cloud SDK for Rust
//!
//! Library provides all available Google gRPC APIs generated from their proto interfaces.
//!
//! The library also provides an easy-to-use client API for gRPC
//! that supports Google Authentication natively.
//!
//! ## gRPC example
//! ```ignore
//!
//!     let firestore_client: GoogleApi<FirestoreClient<GoogleAuthMiddleware>> =
//!        GoogleApi::from_function(
//!            FirestoreClient::new,
//!            "https://firestore.googleapis.com",
//!            // cloud resource prefix: used only for some of the APIs (such as Firestore)
//!            Some(cloud_resource_prefix.clone()),
//!        )
//!        .await?;
//!
//!     let response = firestore_client
//!         .get()
//!         .list_documents(tonic::Request::new(ListDocumentsRequest {
//!             parent: format!("{}/documents", cloud_resource_prefix),
//!             ..Default::default()
//!         }))
//!         .await?;
//!
//! ```
//!
//! Complete examples available on [github](https://github.com/abdolence/gcloud-sdk-rs/tree/master/src/examples).
//!

#![allow(unexpected_cfgs)]
mod apis;
pub use apis::*;

#[cfg(feature = "axum")]
mod axum_layer;
#[cfg(feature = "axum")]
pub use axum_layer::{VerifyIdToken, VerifyIdTokenLayer};
pub mod error;
#[cfg(feature = "id-token-verify")]
pub mod id_token_verify;
mod jwt_crypto;
mod token_source;
pub use middleware::GoogleAuthMiddlewareLayer;
pub use token_source::auth_token_generator::GoogleAuthTokenGenerator;
pub use token_source::id_token::{IdTokenAudience, IdTokenSource, ServiceAccountEmail};
pub use token_source::metadata::Metadata as GceMetadataClient;
pub use token_source::{BoxSource, ExternalJwtFunctionSource, Source, Token, TokenSourceType};

mod api_client;
pub use api_client::*;

mod middleware;

#[cfg(feature = "reqwest-middleware")]
mod reqwest_auth_middleware;
#[cfg(feature = "reqwest-middleware")]
pub use reqwest_auth_middleware::GoogleAuthReqwestMiddleware;

#[cfg(test)]
mod test_support;

pub mod proto_ext;

pub const GCLOUD_SDK_USER_AGENT: &str = concat!("gcloud-sdk-rs/v", env!("CARGO_PKG_VERSION"));

// Re-exports
pub use hyper::HeaderMap;
pub use prost;
pub use prost_types;
pub use secret_vault_value::SecretValue;
pub use tonic;
