//! This library generated from [Google API][`googleapis`] using [tonic-build][`tonic-build`].
//!
//! ## Example
//! The complete code can be found [here][`spanner-admin-example`].
//!
//! Cargo.toml:
//! ```toml
//! [dependencies]
//! gcloud-sdk = { version = "0.7", features = ["google-spanner-admin-database-v1"] }
//! tonic = { version = "0.7", features = ["tls"] }
//! prost = "0.10"
//! prost-types = "0.10"
//! tokio = { version = "1.14", features = ["rt-multi-thread", "time", "fs", "macros"] }
//! ```
//!
//! main.rs:
//! ```ignore
//! use gcloud_sdk::{
//!     google::spanner::admin::database::v1::{
//!         database_admin_client::DatabaseAdminClient, ListDatabasesRequest,
//!     },
//!     CERTIFICATES,
//! };
//! use gcloud_sdk::Token;
//! use tonic::{
//!     metadata::MetadataValue,
//!     transport::{Certificate, Channel, ClientTlsConfig},
//!     Request,
//! };
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let project = std::env::var("PROJECT")?;
//!     let instance = std::env::var("INSTANCE")?;
//!     let token = Token::new()?;
//!
//!     let tls_config = ClientTlsConfig::new()
//!         .ca_certificate(Certificate::from_pem(CERTIFICATES))
//!         .domain_name("spanner.googleapis.com");
//!
//!     let channel = Channel::from_static("https://spanner.googleapis.com")
//!         .tls_config(tls_config)?
//!         .connect()
//!         .await?;
//!
//!     let mut service = DatabaseAdminClient::with_interceptor(channel, move |mut req: Request<()>| {
//!         let token = &*token.header_value().unwrap();
//!         let meta = MetadataValue::from_str(token).unwrap();
//!         req.metadata_mut().insert("authorization", meta);
//!         Ok(req)
//!     });
//!
//!     let response = service
//!         .list_databases(Request::new(ListDatabasesRequest {
//!             parent: format!("projects/{}/instances/{}", project, instance),
//!             page_size: 100,
//!             ..Default::default()
//!         }))
//!         .await?;
//!
//!     println!("RESPONSE={:?}", response);
//!
//!     Ok(())
//! }
//! ```
//!
//! [`googleapis`]: https://github.com/googleapis/googleapis
//! [`tonic-build`]: https://github.com/hyperium/tonic/tree/master/tonic-build
//! [`spanner-admin-example`]: https://github.com/mechiru/googapis/tree/master/examples/spanner-admin

/// The minimal google root set downloaded from https://pki.goog/roots.pem.
///
/// # Example
/// ```no_run
/// # use tonic::transport::{Certificate, ClientTlsConfig};
/// # use gcloud_sdk::CERTIFICATES;
/// let tls_config = ClientTlsConfig::new()
///     .ca_certificate(Certificate::from_pem(CERTIFICATES))
///     .domain_name("spanner.googleapis.com");
/// ````
pub const CERTIFICATES: &[u8] = include_bytes!("../data/roots.pem");

#[allow(unused_macros)]
macro_rules! include_proto {
    ($package: tt) => {
        include!(concat!("../genproto", concat!("/", $package, ".rs")));
    };
}

include!("googapis.rs");
