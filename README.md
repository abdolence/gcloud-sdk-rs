# gcloud-sdk for Rust

[![Latest Version](https://img.shields.io/crates/v/gcloud-sdk.svg)](https://crates.io/crates/gcloud-sdk)
![tests and formatting](https://github.com/abdolence/gcloud-sdk-rs/workflows/tests%20&amp;%20formatting/badge.svg)
![security audit](https://github.com/abdolence/gcloud-sdk-rs/workflows/security%20audit/badge.svg)
![unsafe](https://img.shields.io/badge/unsafe-forbidden-success.svg)

## Disclaimer
This is NOT OFFICIAL Google Cloud SDK (and it doesn't exist for Rust at the time this page updated).

# Overview
This library contains all the code generated from the Google API for gRPC and REST APIs.

## How models are generated:
- gRPC: generated from [Google API](https://github.com/googleapis/googleapis) using [tonic-build](https://github.com/hyperium/tonic/tree/master/tonic-build).
- REST: generate from [Google OpenAPI spec](https://github.com/APIs-guru/openapi-directory/tree/main/APIs/googleapis.com) using [OpenAPI generator]( https://openapi-generator.tech).

## Features
When using each product API, you must explicitly include it in your build using a feature flag.
For example, if you want to use [Cloud Pub/Sub](https://cloud.google.com/pubsub), write `features = ["google-pubsub-v1"]` to Cargo.toml.

The feature name is the period of the package name of each proto file, replaced by a hyphen.
If you specify a package, it will automatically load the dependent packages and include them in the build.
It means that `features = ["google-firestore-v1"]`.

In addition, multiple features can be specified.

The list of available features can be found [here](./gcloud-sdk/Cargo.toml#L22-L390).

## Example for gRPC

```rust
    // The library handles getting token from environment automatically
    let firestore_client: GoogleApi<FirestoreClient<GoogleAuthMiddleware>> =
        GoogleApi::from_function(
            FirestoreClient::new,
            "https://firestore.googleapis.com",
            // cloud resource prefix: used only for some of the APIs (such as Firestore)
            Some(cloud_resource_prefix.clone()),
        )
            .await?;

    let response = firestore_client
        .get()
        .list_documents(tonic::Request::new(ListDocumentsRequest {
            parent: format!("{}/documents", cloud_resource_prefix),
            ..Default::default()
        }))
        .await?;
```
More complete examples are located [here](examples).

Cargo.toml:
```toml
[dependencies]
gcloud-sdk = { version = "0.18", features = ["google-firestore-v1"] }
tonic = { version = "0.8", features = ["tls"] }
prost = "0.11"
prost-types = "0.11"
```

## Example for REST API

```rust
let google_rest_client = gcloud_sdk::GoogleRestApi::new().await?;

let response = gcloud_sdk::google_rest_apis::storage::buckets_api::storage_buckets_list(
    &google_rest_client.create_google_storage_config().await?,
    gcloud_sdk::google_rest_apis::storage::buckets_api::StoragePeriodBucketsPeriodListParams {
        project: google_project_id,
        ..Default::default()
    }
).await?;
```

## Google authentication

Default Scope is `https://www.googleapis.com/auth/cloud-platform`.

To specify custom scopes there is `from_function_with_scopes()` function
instead of `from_function()`;

Looks for credentials in the following places, preferring the first location found:
- A JSON file whose path is specified by the GOOGLE_APPLICATION_CREDENTIALS environment variable.
- A JSON file in a location known to the gcloud command-line tool using `gcloud auth application-default login`.
- On Google Compute Engine, it fetches credentials from the metadata server.

### Local development
Don't confuse `gcloud auth login` with `gcloud auth application-default login` for local development,
since the first authorize only `gcloud` tool to access the Cloud Platform.

The latter obtains user access credentials via a web flow and puts them in the well-known location for Application Default Credentials (ADC).
This command is useful when you are developing code that would normally use a service account but need to run the code in a local development environment where it's easier to provide user credentials.
So to work for local development you need to use `gcloud auth application-default login`.

## Fork-based
The library based on a fork of [mechiru/googapis](https://github.com/mechiru/googapis) and [mechiru/gouth](https://github.com/mechiru/gouth) libraries and also adds additional functionality not available originally:

- Google API client management and Tower-based middleware layer to simplify development to provide an async client implementation that hides complexity working with tokens and TLS.
- Google REST APIs support additionally to gRPC.
- Improved observability with tracing and measuring execution time of endpoints. 
- Uses synchronisation primitives (such as Mutex) from tokio everywhere and has direct dependencies to tokio runtime.
- Security-related protocol extensions for Google Secret Manager and KMS.

### Why not to contribute back?
- Different goals from googapis.
    * This fork focuses on simplicity and provided authentication capabilities natively.
    * Provides a high level facade API for Google API client.
- Different development cycles - the original development was updated less frequently than it was needed.

## High-level APIs
Sometimes using proto generated APIs are tedious and cumbersome, so you may need to introduce facade APIs on top of them:
* [firestore-rs](https://github.com/abdolence/firestore-rs) - to work with Firestore;
* [secret-vault](https://github.com/abdolence/secret-vault-rs) - to read secrets from Google Secret Manager;
* [kms-aead](https://github.com/abdolence/kms-aead-rs) - envelope encryption using Google KMS and Ring AEAD.
* [opentelemetry-gcloud-trace](https://github.com/abdolence/opentelemetry-gcloud-trace-rs) - Google Cloud Trace support for OpenTelemetry project.

## License
Licensed under either of [Apache License, Version 2.0](./LICENSE-APACHE)
or [MIT license](./LICENSE-MIT) at your option.

## Authors
- [mechiru](https://github.com/mechiru) - the original project
- Abdulla Abdurakhmanov - updated for recent deps, the transparent client implementation, security extensions for Google KMS and Secret Manager API.
