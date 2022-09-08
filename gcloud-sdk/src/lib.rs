//! # Google Cloud SDK for Rust
//!
//! Library provides all available APIs generated based on proto interfaces and
//! the middleware based on Tower to provide transparent and easy-to-use Google Authentication
//!
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
//! ```
//!
//! Complete example available at: https://github.com/abdolence/gcloud-sdk-rs/tree/master/src/examples
//!

mod apis;
pub use apis::*;

pub mod error;
mod token_source;
pub use token_source::auth_token_generator::GoogleAuthTokenGenerator;
pub use token_source::{Token, TokenSourceType};

mod api_client;
pub use api_client::*;

mod middleware;

pub mod proto_ext;
