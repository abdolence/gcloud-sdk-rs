//! This library generated from [Google API][`googleapis`] using [tonic-build][`tonic-build`].
//!
//! ## Example
//! The complete code can be found [here][`spanner-admin-example`].
//!
//! Cargo.toml:
//! ```toml
//! [dependencies]
//! googapis = { version = "0.1", features = ["google-spanner-admin-database-v1"] }
//! gouth = { version = "0.1" }
//! tonic = { version = "0.2", features = ["tls"] }
//! prost = { version = "0.6" }
//! prost-types = { version = "0.6" }
//! tokio = { version = "0.2", features = ["rt-threaded", "time", "stream", "fs", "macros", "uds"] }
//! ```
//!
//! main.rs:
//! ```ignore
//! use googapis::{
//!     google::spanner::admin::database::v1::{
//!         database_admin_client::DatabaseAdminClient, ListDatabasesRequest,
//!     },
//!     CERTIFICATES,
//! };
//! use gouth::Token;
//! use tonic::{
//!     metadata::MetadataValue,
//!     transport::{Certificate, Channel, ClientTlsConfig},
//!     Request,
//! };
//!
//! // PROJECT=your-project-id DATABASE=your-spanner-instance cargo run
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let project = std::env::var("PROJECT")?;
//!     let database = std::env::var("DATABASE")?;
//!     let token = Token::new()?;
//!
//!     let tls_config = ClientTlsConfig::new()
//!         .ca_certificate(Certificate::from_pem(CERTIFICATES))
//!         .domain_name("spanner.googleapis.com");
//!
//!     let channel = Channel::from_static("https://spanner.googleapis.com")
//!         .tls_config(tls_config)
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
//!             parent: format!("projects/{}/instances/{}", project, database),
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
/// use tonic::transport::{Certificate, ClientTlsConfig};
/// use googapis::CERTIFICATES;
///
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
