# Google Cloud SDK for Rust

[![Latest Version](https://img.shields.io/crates/v/gcloud-sdk.svg)](https://crates.io/crates/gcloud-sdk)
![tests and formatting](https://github.com/abdolence/gcloud-sdk-rs/workflows/tests%20&amp;%20formatting/badge.svg)
![security audit](https://github.com/abdolence/gcloud-sdk-rs/workflows/security%20audit/badge.svg)
![unsafe](https://img.shields.io/badge/unsafe-forbidden-success.svg)

Async Google Cloud Platform (GCP) gRPC/REST APIs client implementation based on Tonic middleware and Reqwest.

## Disclaimer
This is NOT OFFICIAL Google Cloud SDK (there is early versions with limited functionality of the official Google Cloud SDK now [here](https://github.com/googleapis/google-cloud-rust)).

# Overview
This library contains all the code generated from the Google API for gRPC and REST APIs.

## How API/models are generated:
- gRPC APIs: generated from [Google API](https://github.com/googleapis/googleapis) using [tonic-build](https://github.com/hyperium/tonic/tree/master/tonic-build).
- REST APIs (only for the APIs not available to use through gRPC): generated from [Google OpenAPI spec](https://github.com/APIs-guru/openapi-directory/tree/main/APIs/googleapis.com) using [OpenAPI generator]( https://openapi-generator.tech).

## Features
When using each product API, you must explicitly include it in your build using a feature flag.
For example, if you want to use [Cloud Pub/Sub](https://cloud.google.com/pubsub), write `features = ["google-pubsub-v1"]` to Cargo.toml.

The feature name is the period of the package name of each proto file, replaced by a hyphen.

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
gcloud-sdk = { version = "0.28", features = ["google-firestore-v1"] }
```

### Crypto provider error

Depends on your other dependencies you may see the error like:

```
no process-level CryptoProvider available -- call CryptoProvider::install_default() before this point 
```

This is because the TLS providers are not installed by default and you can choose different.
The easiest way to fix is just to include one of the provider, for example:

```toml
[dependencies]
rustls = "0.23"
```

If you have multiple you may need to call `CryptoProvider::install_default()` before using the client.

```rust
rustls::crypto::ring::default_provider().install_default().expect("Failed to install rustls crypto provider");
```

## Example for REST API

```rust
let google_rest_client = gcloud_sdk::GoogleRestApi::new().await?;

let response = gcloud_sdk::google_rest_apis::storage_v1::buckets_api::storage_buckets_list(
    &google_rest_client.create_google_storage_v1_config().await?,
    gcloud_sdk::google_rest_apis::storage_v1::buckets_api::StoragePeriodBucketsPeriodListParams {
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

### Workload Identity Federation
The library provides the support for workload identity federation support to use "keyless" integrations with different providers:
- URL based OIDC/SAML (for example GitHub actions) with text/json file formats;
- File based OIDC/SAML  with text/json file formats;
- AWS external account: authentication from AWS computing instances(e.g. EC2, lambda, ECS, etc.) is now supported as "external-account-aws" feature in https://github.com/abdolence/gcloud-sdk-rs/pull/172.
However, it is not intensively tested yet, so please report issues if there's a problem.

### Local development
Don't confuse `gcloud auth login` with `gcloud auth application-default login` for local development,
since the first authorize only `gcloud` tool to access the Cloud Platform.

The latter obtains user access credentials via a web flow and puts them in the well-known location for Application Default Credentials (ADC).
This command is useful when you are developing code that would normally use a service account but need to run the code in a local development environment where it's easier to provide user credentials.
So to work for local development you need to use `gcloud auth application-default login`.

### Routing headers
Some of the APIs (notable KMS, Artifact Registry and others) require additionally specify headers such as `x-goog-request-params`.
You can find an example how to handle it [here](https://github.com/abdolence/kms-aead-rs/blob/b8bb496800625a660be0c9896366d7407b8aa714/src/providers/gcp_kms_encryption.rs#L114)

## Service-to-service authentication
The library supports Google-signed ID tokens on both sides of a call between services:
- minting them to call Cloud Run, Cloud Run functions and services behind Identity-Aware Proxy (IAP);
- attaching them to gRPC and HTTP requests;
- verifying them in the service that receives the call (`id-token-verify` feature);
- protecting an axum service (`axum` feature) and calling it with reqwest (`reqwest-middleware` feature).

### Calling a Cloud Run or IAP service
`IdTokenSource` mints ID tokens for one audience:
- the URL of a Cloud Run service, or a custom audience configured for it;
- the OAuth client ID of a resource behind IAP.

Google ID tokens are valid for one hour. `GoogleAuthTokenGenerator::from_source` caches the token
and refreshes it before `exp`, the same way as access tokens.

gRPC (for example a tonic service on Cloud Run):

```rust
let audience = IdTokenAudience::new("https://orders-abc123-ew.a.run.app");
let id_tokens = GoogleAuthTokenGenerator::from_source(
    IdTokenSource::new(audience, TokenSourceType::Default).await?,
);

let orders_client: GoogleApi<OrdersClient<GoogleAuthMiddleware>> =
    GoogleApi::from_function_with_middleware(
        OrdersClient::new,
        "https://orders-abc123-ew.a.run.app",
        GoogleAuthMiddlewareLayer::new(id_tokens, None)?,
    )
    .await?;
```

HTTP with `reqwest-middleware` feature:

```rust
let client = reqwest_middleware::ClientBuilder::new(reqwest::Client::new())
    .with(GoogleAuthReqwestMiddleware::new(id_tokens))
    .build();

let response = client
    .get("https://orders-abc123-ew.a.run.app/orders")
    .send()
    .await?;
```

Without the feature, `id_tokens.authorization_header().await?` gives the `authorization` header value
for any HTTP client.

### Which credentials work
`IdTokenSource::new` takes the identity of the credentials it finds:
- service account key file: signs a JWT with the key and exchanges it for an ID token;
- metadata server (Cloud Run, GKE, Compute Engine, etc.): the service account attached to the workload;
- `gcloud auth application-default login --impersonate-service-account=...` and workload identity
  federation with service account impersonation: the impersonated service account.

User credentials from `gcloud auth application-default login` and workload identity federation without
service account impersonation cannot mint an ID token for an audience,
so `IdTokenSource::new` returns `ErrorKind::IdTokenNeedsImpersonation` for them.
Use `IdTokenSource::impersonating` with any credentials to mint ID tokens of a service account through
IAM Credentials API:

```rust
let source = IdTokenSource::impersonating(
    IdTokenAudience::new("https://orders-abc123-ew.a.run.app"),
    ServiceAccountEmail::new("invoker@my-project.iam.gserviceaccount.com"),
    TokenSourceType::Default,
)
.await?;
```

The caller needs `roles/iam.serviceAccountOpenIdTokenCreator` on that service account.
This is also the way for local development, using your own `gcloud auth application-default login` credentials.

### Receiving and verifying tokens
`IdTokenVerifier` from `id-token-verify` feature checks:
- RS256 signature with Google keys from `https://www.googleapis.com/oauth2/v3/certs`;
- `iss` is Google, and `aud` is the audience of your service;
- `exp`, `nbf` and `iat`, with 30 seconds of leeway.

The keys are cached for the `max-age` of the response. A token signed with an unknown key ID refetches them,
at most once every 30 seconds. When a fetch fails, the cached keys stay in use until their `max-age` passes.

```rust
let verifier = IdTokenVerifier::new(IdTokenAudience::new("https://orders-abc123-ew.a.run.app"))?;

match verifier.verify(bearer_token).await {
    Ok(verified) => {
        // Your own allowlist of callers
        let caller = verified.verified_email();
    }
    // Answer 401
    Err(IdTokenVerifyError::InvalidToken(reason)) => {}
    // Answer 503: the token may be valid, but Google keys are not available
    Err(IdTokenVerifyError::KeysUnavailable(error)) => {}
}
```

`IdTokenVerifier::with_keys_source` takes the keys from your own `IdTokenKeysSource` instead,
for example a locally generated RSA key in tests.

The verifier checks signatures with the [JWT crypto provider](#jwt-crypto-provider) of the library features.

### Protecting an axum service
`VerifyIdTokenLayer` from `axum` feature verifies the bearer token of every request:
- 401 for a missing or invalid token;
- 403 when the token is valid, but your `authorize` check refuses the caller;
- 503 when Google keys are not available.

Handlers receive the verified claims as `VerifiedIdToken`:

```rust
let verifier = Arc::new(IdTokenVerifier::new(IdTokenAudience::new(
    "https://orders-abc123-ew.a.run.app",
))?);
let billing = PrincipalEmail::new("billing@my-project.iam.gserviceaccount.com");

let app = Router::new()
    .route("/orders", get(list_orders))
    .layer(
        VerifyIdTokenLayer::new(verifier)
            .authorize(move |token| token.verified_email() == Some(&billing)),
    );

async fn list_orders(caller: VerifiedIdToken) -> String {
    format!("Orders for {:?}", caller.verified_email())
}
```

The layer is a tower layer, so it works the same way for tonic servers.

Full example available [here](examples/id-token).

### JWT crypto provider
Service account keys sign JWTs with `jsonwebtoken`, which needs a crypto provider. The library has a feature for each:
- `jwt-aws-lc-rs`: default feature, uses aws-lc-rs, which is built from C sources and needs a C compiler;
- `jwt-rust-crypto`: uses the pure Rust crates of RustCrypto.

With `default-features = false` enable one of them, otherwise service account keys fail with `ErrorKind::JwtCryptoProviderMissing`.
With both enabled, aws-lc-rs is used.
The library installs the provider as the process default of `jsonwebtoken`, unless your application installed one before.

`tls-webpki-roots` already builds aws-lc-rs for rustls, so `jwt-aws-lc-rs` adds nothing new with it.
`jwt-rust-crypto` avoids the C build only together with `tls-roots`.

## High-level APIs
Sometimes using proto generated APIs are tedious and cumbersome, so you may need to introduce facade APIs on top of them:
* [firestore](https://github.com/abdolence/firestore-rs) - to work with Firestore;
* [bigquery](https://github.com/abdolence/bigquery-rs) - to work with BigQuery;
* [secret-vault](https://github.com/abdolence/secret-vault-rs) - to read secrets from Google Secret Manager;
* [kms-aead](https://github.com/abdolence/kms-aead-rs) - envelope encryption using Google KMS and Ring AEAD.
* [opentelemetry-gcloud-trace](https://github.com/abdolence/opentelemetry-gcloud-trace-rs) - Google Cloud Trace support for OpenTelemetry project.

## License
Licensed under either of [Apache License, Version 2.0](./LICENSE-APACHE)
or [MIT license](./LICENSE-MIT) at your option.

## Authors
- Abdulla Abdurakhmanov
- [mechiru](https://github.com/mechiru) - the original project (gRPC proto generator and token sources implementation)

The library was started as a fork of [mechiru/googapis](https://github.com/mechiru/googapis) and [mechiru/gouth](https://github.com/mechiru/gouth) libraries, but now includes much more:

- Google API client/tokens management and Tower-based middleware layer to simplify development to provide an async client implementation that hides complexity working with tokens and TLS.
- Google REST APIs support additionally to gRPC.
- Workload Identity Federation support.
- Improved observability with tracing and measuring execution time of endpoints.
- Uses synchronisation primitives (such as Mutex) from tokio everywhere and has direct dependencies to tokio runtime.
- Security-related protocol extensions for Google Secret Manager and KMS.
