# gcloud-sdk for Rust

[![ci](https://github.com/latestbit/gcloud-sdk/workflows/ci/badge.svg)](https://github.com/mechiru/googapis/actions?query=workflow:ci)
[![Rust Documentation](https://docs.rs/gcloud-sdk/badge.svg)](https://mechiru.github.io/googapis/googapis/index.html)
[![Latest Version](https://img.shields.io/crates/v/gcloud-sdk.svg)](https://crates.io/crates/googapis)

This library generated from [Google API](https://github.com/googleapis/googleapis) using [tonic-build](https://github.com/hyperium/tonic/tree/master/tonic-build).

This is not offical gcloud sdk from Google (and it doesn't exist at the moment of this page was updated).

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

The list of available features can be found [here](./googapis/Cargo.toml#L22-L315).

## Example
The complete code can be found [here](./examples/spanner-admin).

Cargo.toml:
```toml
[dependencies]
gcloud-sdk = { version = "0.7", features = ["google-spanner-admin-database-v1"] }
tonic = { version = "0.7", features = ["tls"] }
prost = "0.10"
prost-types = "0.10"
tokio = { version = "1.14", features = ["rt-multi-thread", "time", "fs", "macros"] }
```

## Fork
The library is a fork of [mechiru/googapis](https://github.com/mechiru/googapis) and [mechiru/gouth](https://github.com/mechiru/gouth) to keep up with
the updates and API proto descriptions from Google more frequently and simplify dependencies management.

### Why not to contribute back?
- Different goals from googapis.
    * This fork focuses on simplicity and provided authentication capabilities natively.
    * Has direct dependency to tokio runtime and uses synchronisation primitives (such as Mutex) from tokio.
    * Provides Facade API and caching client implementation
- Different development cycles - the original development was updated less frequently than it was needed for me.

I'd be glad to contribute all of the changes back if author sees the same goals now.

## License
Licensed under either of [Apache License, Version 2.0](./LICENSE-APACHE)
or [MIT license](./LICENSE-MIT) at your option.

## Authors
- [mechiru](https://github.com/mechiru) - the original project
- Abdulla Abdurakhmanov - updated for recent deps, introduced caching client implementation, merged with gouth.
