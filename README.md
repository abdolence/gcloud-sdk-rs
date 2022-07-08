# gcloud-sdk for Rust

[![ci](https://github.com/abdolence/gcloud-sdk-rs/workflows/ci/badge.svg)](https://github.com/abdolence/gcloud-sdk-rs/actions?query=workflow:ci)
[![Rust Documentation](https://docs.rs/gcloud-sdk/badge.svg)](https://docs.rs/gcloud-sdk)
[![Latest Version](https://img.shields.io/crates/v/gcloud-sdk.svg)](https://crates.io/crates/gcloud-sdk)

This library generated from [Google API](https://github.com/googleapis/googleapis) using [tonic-build](https://github.com/hyperium/tonic/tree/master/tonic-build).

This is not official gcloud sdk from Google (and it doesn't exist for Rust at the moment of this page was updated).

## Overview
This library contains all the code generated from the Google API.

When using each product API, you must explicitly include it in your build using a feature flag.
For example, if you want to use [Cloud Pub/Sub](https://cloud.google.com/pubsub), write `features = ["google-pubsub-v1"]` to Cargo.toml.

The feature name is the period of the package name of each proto file, replaced by a hyphen.
If you specify a package, it will automatically load the dependent packages and include them in the build.
It means that `features = ["google-spanner-admin-database-v1"]`.

In addition, multiple features can be specified.

The list of available features can be found [here](./gcloud-sdk/Cargo.toml#L22-L390).

## Example

```rust
use gcloud_sdk::*;

use gcloud_sdk::google::spanner::admin::database::v1::{
    database_admin_client::DatabaseAdminClient, ListDatabasesRequest,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let project = std::env::var("PROJECT")?;
    let instance = std::env::var("INSTANCE")?;

    // The library handles getting token from environment automatically
    let spanner_client: GoogleApiClientFn<DatabaseAdminClient<GoogleConnectorInterceptedService>> = 
        GoogleApiClient::from_function(
            DatabaseAdminClient::with_interceptor,
            "https://spanner.googleapis.com",
            chrono::Duration::minutes(15), // max caching client duration
            None, // cloud resource prefix: used only for some of the APIs (such as Firestore) 
        )
            .await?;

    let response = spanner_client
        .get()
        .await? // The library caches clients and tokens automatically
        .list_databases(tonic::Request::new(ListDatabasesRequest {
            parent: format!("projects/{}/instances/{}", project, instance),
            page_size: 100,
            ..Default::default()
        }))
        .await?;

    Ok(())
}

```

Cargo.toml:
```toml
[dependencies]
gcloud-sdk = { version = "0.10", features = ["google-spanner-admin-database-v1"] }
tonic = { version = "0.7", features = ["tls"] }
prost = "0.10"
prost-types = "0.10"
tokio = { version = "1.14", features = ["rt-multi-thread", "time", "fs", "macros"] }
chrono = "0.4"
```

## Google authentication

Default Scope is `https://www.googleapis.com/auth/cloud-platform`.

To specify custom scopes there is `from_function_with_scopes()` function
instead of `from_function()`;

Looks for credentials in the following places, preferring the first location found:
- A JSON file whose path is specified by the GOOGLE_APPLICATION_CREDENTIALS environment variable.
- A JSON file in a location known to the gcloud command-line tool.
- On Google Compute Engine, it fetches credentials from the metadata server.

## Fork
The library is a fork of [mechiru/googapis](https://github.com/mechiru/googapis) and [mechiru/gouth](https://github.com/mechiru/gouth) to keep up with
the updates and API proto descriptions from Google more frequently and simplify dependencies management.

### Why not to contribute back?
- Different goals from googapis.
    * This fork focuses on simplicity and provided authentication capabilities natively.
    * Has direct dependencies to tokio runtime and chrono.
      Uses synchronisation primitives (such as Mutex) from tokio everywhere.
    * Provides facade API for the caching async client implementation
      that hides complexity working with tokens and TLS.
- Different development cycles - the original development was updated less frequently than it was needed for me.

I'd be glad to contribute all of the changes back if author sees the same goals.

## License
Licensed under either of [Apache License, Version 2.0](./LICENSE-APACHE)
or [MIT license](./LICENSE-MIT) at your option.

## Authors
- [mechiru](https://github.com/mechiru) - the original project
- Abdulla Abdurakhmanov - updated for recent deps, introduced caching client implementation, merged/refactorings with gouth.
