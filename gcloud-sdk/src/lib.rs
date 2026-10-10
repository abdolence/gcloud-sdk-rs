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
mod adc;
mod auth_token_generator;
pub mod error;
mod id_token;
#[cfg(feature = "id-token-verify")]
pub mod id_token_verify;
#[cfg(feature = "id-token-verify")]
mod jwt_crypto;
mod metadata;
pub use auth_token_generator::GoogleAuthTokenGenerator;
pub use id_token::{IdTokenAudience, ServiceAccountEmail};
pub use middleware::GoogleAuthMiddlewareLayer;

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
/// The crate that mints every token, at the version this crate is built against: build
/// [`Credentials`](google_cloud_auth::credentials::Credentials) with it for
/// [`GoogleApi::from_function_with_credentials`] and [`GoogleAuthTokenGenerator`].
pub use google_cloud_auth;
pub use hyper::HeaderMap;
pub use prost;
pub use prost_types;
pub use secret_vault_value::SecretValue;
pub use tonic;
