# googapis

[![ci](https://github.com/mechiru/googapis/workflows/ci/badge.svg)](https://github.com/mechiru/googapis/actions?query=workflow:ci)
[![Rust Documentation](https://docs.rs/googapis/badge.svg)](https://docs.rs/googapis)
[![Latest Version](https://img.shields.io/crates/v/googapis.svg)](https://crates.io/crates/googapis)

This library generated from [Google API](https://github.com/googleapis/googleapis) using [tonic-build](https://github.com/hyperium/tonic/tree/master/tonic-build).

## Overview
This library contains all the code generated from the Google API.

When using each product API, you must explicitly include it in your build using a feature flag.
For example, if you want to use [Cloud Pub/Sub](https://cloud.google.com/pubsub), write `features = ["google-pubsub-v1"]` to Cargo.toml.

The feature name is the period of the package name of each proto file, replaced by a hyphen.
If you specify a package, it will automatically load the dependent packages and include them in the build.
It means that `features = ["google-spanner-admin-database-v1"]` is the same as the code below:
```rust
pub mod google {
    pub mod api {
        tonic::include_proto!("google.api");
    }
    pub mod iam {
        pub mod v1 {
            tonic::include_proto!("google.iam.v1");
        }
    }
    pub mod longrunning {
        tonic::include_proto!("google.longrunning");
    }
    pub mod r#type {
        tonic::include_proto!("google.r#type");
    }
    pub mod rpc {
        tonic::include_proto!("google.rpc");
    }
    pub mod spanner {
        pub mod admin {
            pub mod database {
                pub mod v1 {
                    tonic::include_proto!("google.spanner.admin.database.v1");
                }
            }
        }
    }
}
```

In addition, multiple features can be specified.

The list of available features can be found [here](./googapis/Cargo.toml#L22-L240).

## Version matrices
| googapis | tonic | tonic-build |
|----------|-------|-------------|
| 0.1.x    | 0.2.x | 0.2.x       |
| 0.2.x    | 0.2.x | 0.2.x       |
| 0.3.x    | 0.3.x | 0.3.x       |

## Example
The complete code can be found [here](./examples/spanner-admin).

Cargo.toml:
```toml
[dependencies]
googapis = { version = "0.1", features = ["google-spanner-admin-database-v1"] }
gouth = { version = "0.1" }
tonic = { version = "0.2", features = ["tls"] }
prost = { version = "0.6" }
prost-types = { version = "0.6" }
tokio = { version = "0.2", features = ["rt-threaded", "time", "stream", "fs", "macros", "uds"] }
```

main.rs:
```rust
use googapis::{
    google::spanner::admin::database::v1::{
        database_admin_client::DatabaseAdminClient, ListDatabasesRequest,
    },
    CERTIFICATES,
};
use gouth::Token;
use tonic::{
    metadata::MetadataValue,
    transport::{Certificate, Channel, ClientTlsConfig},
    Request,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let project = std::env::var("PROJECT")?;
    let instance = std::env::var("INSTANCE")?;
    let token = Token::new()?;

    let tls_config = ClientTlsConfig::new()
        .ca_certificate(Certificate::from_pem(CERTIFICATES))
        .domain_name("spanner.googleapis.com");

    let channel = Channel::from_static("https://spanner.googleapis.com")
        .tls_config(tls_config)?
        .connect()
        .await?;

    let mut service = DatabaseAdminClient::with_interceptor(channel, move |mut req: Request<()>| {
        let token = &*token.header_value().unwrap();
        let meta = MetadataValue::from_str(token).unwrap();
        req.metadata_mut().insert("authorization", meta);
        Ok(req)
    });

    let response = service
        .list_databases(Request::new(ListDatabasesRequest {
            parent: format!("projects/{}/instances/{}", project, instance),
            page_size: 100,
            ..Default::default()
        }))
        .await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}
```

## License
Licensed under either of [Apache License, Version 2.0](./LICENSE-APACHE) or [MIT license](./LICENSE-MIT) at your option.
