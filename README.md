# Google Cloud SDK for Rust

[![Latest Version](https://img.shields.io/crates/v/gcloud-sdk.svg)](https://crates.io/crates/gcloud-sdk)
![tests and formatting](https://github.com/abdolence/gcloud-sdk-rs/workflows/tests%20&amp;%20formatting/badge.svg)
![security audit](https://github.com/abdolence/gcloud-sdk-rs/workflows/security%20audit/badge.svg)
![unsafe](https://img.shields.io/badge/unsafe-forbidden-success.svg)

Async gRPC clients for every Google Cloud API, generated from [googleapis](https://github.com/googleapis/googleapis) with Tonic,
and the glue to authenticate them and your own services.

The library provides:
- tonic clients and prost types for every Google API, each behind its own feature;
- a tower middleware that attaches Google credentials to every request, with authentication from the official
  [google-cloud-auth](https://crates.io/crates/google-cloud-auth) crate;
- [service-to-service authentication](#service-to-service-authentication) with Google ID tokens: minting them for Cloud Run and IAP,
  a reqwest middleware, and an axum/tower layer that verifies them;
- `tls-roots` (native-tls) and `tls-webpki-roots` (rustls) options for the connections.

This is not an official Google project.

## gcloud-sdk and the official Google Cloud Rust SDK
Google now publishes an official [Google Cloud Rust SDK](https://github.com/googleapis/google-cloud-rust):
high-level clients for most Google Cloud services and `google-cloud-auth` for credentials.
Since 0.33 gcloud-sdk builds on it.

### Authentication comes from Google
Every access token and ID token is minted by `google-cloud-auth`.
Credential formats, new credential sources and security fixes come from Google, and gcloud-sdk no longer maintains a second copy of them.
It also covers cases the old token sources of gcloud-sdk did not:
- workload identity federation with executable sources, workforce pools, and AWS without the AWS SDK;
- impersonation with scopes, delegates and token lifetime;
- quota projects (`x-goog-user-project`, `GOOGLE_CLOUD_QUOTA_PROJECT`) and universe domains;
- token refresh before expiry, with retries.

The REST clients are removed for the same reason: the official crates cover them, see [REST APIs moved to the official SDK](#rest-apis-moved-to-the-official-sdk).

### The gaps gcloud-sdk fills
The official SDK still leaves gaps, and closes them slowly. As of October 2026:
- **Raw gRPC access.** The official crates expose no tonic clients or prost types and take no channel or tower layers of your own.
  gcloud-sdk generates a tonic client for every API in googleapis, so any method of any API is available as soon as it appears in the protos.
- **Data-plane APIs.** Bigtable, Firestore (including `Listen`) and Datastore are not published as official crates, Spanner is pre-1.0,
  and BigQuery Storage Read sits behind an unstable cfg.
- **Streaming.** Bidirectional and server streaming calls work as tonic streams, for example Firestore `Listen`, Pub/Sub `StreamingPull`
  and BigQuery Storage Write.
- **Your own tower stack.** The clients are plain tonic clients over a tower service, so you can add your own layers, timeouts and channels.
- **native-tls.** The official SDK supports rustls only and declined native-tls in [googleapis/google-cloud-rust#4316](https://github.com/googleapis/google-cloud-rust/issues/4316).
  With `tls-roots`, gcloud-sdk uses native-tls for the gRPC channels and for the token requests of `google-cloud-auth`.

### Opinionated integrations
gcloud-sdk makes choices the official SDK leaves to you:
- `VerifyIdTokenLayer` verifies Google ID tokens in axum and tonic servers, with typed claims and an `authorize` check;
- `GoogleAuthMiddlewareLayer` and `GoogleAuthReqwestMiddleware` attach credentials to tonic and reqwest calls;
- the higher-level crates listed in [High-level APIs](#high-level-apis) build on these clients: Firestore, BigQuery, Secret Manager, KMS, Cloud Trace.

### Which one to use
- The official crates, when a high-level client exists for your service and you need nothing from the list above.
- gcloud-sdk, for raw gRPC access, the data-plane APIs above, streaming, your own tower layers, native-tls,
  or service-to-service authentication.

The two combine in one application. The authentication of gcloud-sdk is `google-cloud-auth`, re-exported as `gcloud_sdk::google_cloud_auth`,
so one `Credentials` serves the official clients and the gcloud-sdk clients.

## Migrating from 0.32
0.33 is a breaking release. The token sources of gcloud-sdk are replaced by `google-cloud-auth`, and the REST clients are removed.

### API mapping

| 0.32 | 0.33 |
|---|---|
| `Source`, `BoxSource`, `ExternalJwtFunctionSource`, `TokenSourceType::ExternalSource` | implement `google_cloud_auth::credentials::CredentialsProvider`, then `Credentials::from(..)` |
| `TokenSourceType::Default` with scopes | `credentials::Builder::default().with_scopes(..).build()?` |
| `TokenSourceType::Json`, `TokenSourceType::File` | `credentials::{service_account, user_account, impersonated, external_account}::Builder::new(json)` |
| `TokenSourceType::MetadataServer` | `credentials::mds::Builder::default()` |
| `GceMetadataClient` | none |
| `Token`, `Token::generate_for_scopes` | `credentials::Builder::default().build_access_token_credentials()?.access_token()` |
| `GoogleAuthTokenGenerator::new(..)`, `GoogleAuthTokenGenerator::from_source(..)` | `GoogleAuthTokenGenerator::from(..)` with `Credentials` or `IDTokenCredentials` |
| `authorization_header()`, `create_token()` | `headers()`: every authentication header, `authorization` included |
| `clear_cache()` | none, `google-cloud-auth` refreshes the tokens |
| `IdTokenSource::new(audience, TokenSourceType::Default)` | `GoogleAuthTokenGenerator::id_token(&audience)` |
| `IdTokenSource::impersonating(audience, service_account, TokenSourceType::Default)` | `GoogleAuthTokenGenerator::id_token_impersonating(&audience, &service_account, credentials::Builder::default().build()?)` |
| `IdTokenSource::new(audience, Json / File / MetadataServer)` | `GoogleAuthTokenGenerator::from(idtoken::{service_account, mds}::Builder::new(..).build()?)` |
| `from_function_with_token_source`, `from_function_with_token_source_and_headers` | `from_function_with_credentials`, `from_function_with_credentials_and_headers` |
| `with_token_source`, `with_token_source_and_headers` | `with_credentials`, `with_credentials_and_headers` |
| `with_token_source_and_middleware` | `with_middleware` |
| `GoogleEnvironment::find_default_creds` | none |
| `ErrorKind::Auth(AuthErrorDetails)` | `ErrorKind::Credentials(CredentialsError)`, and `ErrorKind::CredentialsBuild` when credentials cannot be built |
| `rest` feature, `GoogleRestApi`, `google_rest_apis` | removed, see [REST APIs moved to the official SDK](#rest-apis-moved-to-the-official-sdk) |

The `credentials` and `idtoken` paths above are modules of `gcloud_sdk::google_cloud_auth::credentials`.
`from_function` and `from_function_with_scopes` keep their signatures.

### Removed features
- `rest` and every `google-rest-*` feature;
- `external-account-aws`: AWS workload identity federation is built into `google-cloud-auth`;
- about 85 pre-release API features (`v1alpha`, `v1beta1`, etc.) superseded by a GA version of the same API,
  to keep the crate under the crates.io size limit. The Gemini (`google-ai-generativelanguage-v1beta`), Vertex AI (`google-cloud-aiplatform-v1beta1`)
  and text-to-speech (`google-cloud-texttospeech-v1beta1`) betas are kept.

New features:
- `auth-default-crypto` (default): the aws-lc-rs rustls provider and ID token backend of `google-cloud-auth`.

### Behaviour changes
- The middleware sends every header the credentials produce, not only `authorization`, so the quota project reaches the server as `x-goog-user-project`.
- Tokens are refreshed before they expire, and failed token requests are retried.
- Service account keys sign a JWT that is used as the access token directly, without the exchange at `token_uri`.
- The project ID lookup on the metadata server uses `GCE_METADATA_HOST`, else `metadata.google.internal`, the same host `google-cloud-auth` uses.
  It no longer probes `169.254.169.254`.
- Workload identity federation without service account impersonation fails `GoogleAuthTokenGenerator::id_token` with
  `ErrorKind::IdTokenNeedsImpersonation`, the same way as user credentials. With impersonation it mints as the impersonated service account.

### Known limits
- **MSRV is 1.91**, the MSRV of `google-cloud-auth`.
- **Credentials must be built inside a Tokio runtime.** Building them spawns their refresh task.
- **aws-lc-rs comes in by default** through `auth-default-crypto`, and it is built from C sources.
  With `default-features = false`, install a rustls `CryptoProvider` before building credentials, see [crypto providers](#crypto-providers).
- **AWS workload identity federation reads only the environment keys and EC2 IMDS.** ECS task roles, `~/.aws` profiles and SSO no longer work.
- **Service account keys must be PKCS#8**, the format Google issues them in.

## REST APIs moved to the official SDK
0.33 removes the REST clients generated from OpenAPI specs. Each one has an official crate, except FCM and Identity Toolkit v3.
Each official crate was checked with a read-only call against a live project, authenticated through `google-cloud-auth` 1.17.0.

| Removed module | Official crate |
|---|---|
| `storage_v1` | [google-cloud-storage](https://crates.io/crates/google-cloud-storage) |
| `cloudresourcemanager_v3` | [google-cloud-resourcemanager-v3](https://crates.io/crates/google-cloud-resourcemanager-v3) |
| `bigquery_v2` | [google-cloud-bigquery-v2](https://crates.io/crates/google-cloud-bigquery-v2) |
| `compute_v1` | [google-cloud-compute-v1](https://crates.io/crates/google-cloud-compute-v1), with a feature for each resource |
| `dns_v1` | [google-cloud-dns-v1](https://crates.io/crates/google-cloud-dns-v1) |
| `sqladmin_v1` | [google-cloud-sql-v1](https://crates.io/crates/google-cloud-sql-v1) |
| `lustre_v1` | [google-cloud-lustre-v1](https://crates.io/crates/google-cloud-lustre-v1) |
| `servicecontrol_v1` | [google-cloud-api-servicecontrol-v1](https://crates.io/crates/google-cloud-api-servicecontrol-v1) |
| `servicecontrol_v2` | [google-cloud-api-servicecontrol-v2](https://crates.io/crates/google-cloud-api-servicecontrol-v2) |
| `fcm_v1` | none; community crates: [google-fcm1](https://crates.io/crates/google-fcm1), [fcm-service](https://crates.io/crates/fcm-service) |
| `identitytoolkit_v3` | none; community crate: [google-identitytoolkit3](https://crates.io/crates/google-identitytoolkit3). For the Identity Platform admin API, the gRPC `google-cloud-identitytoolkit-v2` feature of gcloud-sdk |

## Getting started

### Features
Each Google API is behind its own feature, and you must enable the ones you use.
For example, for [Cloud Pub/Sub](https://cloud.google.com/pubsub) write `features = ["google-pubsub-v1"]` in Cargo.toml.

The feature name is the proto package name with periods replaced by hyphens: `google.pubsub.v1` is `google-pubsub-v1`.

The full list is on [docs.rs](https://docs.rs/crate/gcloud-sdk/latest/features), and in the `# Google API features` section of [gcloud-sdk/Cargo.toml](./gcloud-sdk/Cargo.toml).

The library features:
- `tls-roots`: default, native-tls with the system roots;
- `tls-webpki-roots`: rustls with the webpki roots;
- `auth-default-crypto`: default, the aws-lc-rs crypto of `google-cloud-auth`;
- `jwt-aws-lc-rs` (default) and `jwt-rust-crypto`: the crypto of the ID token verifier, see [JWT crypto provider](#jwt-crypto-provider);
- `id-token-verify`, `axum`, `reqwest-middleware`: [service-to-service authentication](#service-to-service-authentication).

### Example for gRPC

```toml
[dependencies]
gcloud-sdk = { version = "0.33", features = ["google-firestore-v1"] }
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

```rust
use gcloud_sdk::google::firestore::v1::firestore_client::FirestoreClient;
use gcloud_sdk::google::firestore::v1::ListDocumentsRequest;
use gcloud_sdk::*;

let google_project_id = GoogleEnvironment::detect_google_project_id()
    .await
    .expect("No Google Project ID detected. Please specify it explicitly using env variable: PROJECT_ID");
let cloud_resource_prefix = format!("projects/{}/databases/(default)", google_project_id);

// The Application Default Credentials, for the cloud-platform scope
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

Full example available [here](examples/firestore-client). More examples are located [here](examples).

### Crypto providers
`google-cloud-auth` signs service account JWTs with a rustls `CryptoProvider`.
With the default features it uses aws-lc-rs.

With `default-features = false`, enable a TLS feature and install a provider yourself before building credentials,
with the same `rustls` version as `google-cloud-auth` (0.23):

```toml
[dependencies]
gcloud-sdk = { version = "0.33", default-features = false, features = ["tls-roots", "google-firestore-v1"] }
rustls = { version = "0.23", features = ["ring"] }
```

```rust
rustls::crypto::ring::default_provider()
    .install_default()
    .expect("Failed to install rustls crypto provider");
```

The same call fixes this error, which appears when more than one rustls provider is compiled into your application:

```
no process-level CryptoProvider available -- call CryptoProvider::install_default() before this point
```

## Google authentication
The library authenticates with `google-cloud-auth`. `GoogleApi::from_function` uses the Application Default Credentials (ADC)
for the `https://www.googleapis.com/auth/cloud-platform` scope. ADC looks for credentials in the following places, preferring the first location found:
- a JSON file whose path is in the `GOOGLE_APPLICATION_CREDENTIALS` environment variable;
- the file `gcloud auth application-default login` writes;
- the metadata server on Compute Engine, GKE, Cloud Run, etc.

The JSON file can be a service account key, user credentials, impersonated service account credentials or a workload identity federation config.

Credentials must be built inside a Tokio runtime.

### Custom scopes
`from_function_with_scopes()` takes the scopes instead of `from_function()`:

```rust
let client: GoogleApi<FirestoreClient<GoogleAuthMiddleware>> =
    GoogleApi::from_function_with_scopes(
        FirestoreClient::new,
        "https://firestore.googleapis.com",
        Some(cloud_resource_prefix),
        vec!["https://www.googleapis.com/auth/datastore".to_string()],
    )
    .await?;
```

### Custom credentials
`from_function_with_credentials()` takes any `Credentials` built with `google-cloud-auth`, for example a service account key that is not the ADC:

```rust
use gcloud_sdk::google_cloud_auth::credentials::service_account;

let credentials = service_account::Builder::new(serde_json::from_str(&service_account_key)?).build()?;

let client: GoogleApi<FirestoreClient<GoogleAuthMiddleware>> =
    GoogleApi::from_function_with_credentials(
        FirestoreClient::new,
        "https://firestore.googleapis.com",
        Some(cloud_resource_prefix),
        credentials,
    )
    .await?;
```

For tokens from your own source, implement `google_cloud_auth::credentials::CredentialsProvider` and wrap it with `Credentials::from(..)`.
The same `Credentials` work with the official Google Cloud Rust clients.

### Workload identity federation
Workload identity federation gives "keyless" access from other clouds and CI systems. Point `GOOGLE_APPLICATION_CREDENTIALS`
at the `external_account` config from `gcloud iam workload-identity-pools create-cred-config`. The supported sources:
- file: an OIDC or SAML token in a text or JSON file;
- URL: an OIDC or SAML token from a URL, for example in GitHub Actions;
- AWS: the `AWS_ACCESS_KEY_ID` environment keys, else EC2 IMDS (IMDSv2 when the config has a session token URL). ECS task roles, `~/.aws` profiles and SSO are not supported;
- executable: a command that prints the token. It runs only with `GOOGLE_EXTERNAL_ACCOUNT_ALLOW_EXECUTABLES=1`.

Workforce identity pools and service account impersonation in the config work as well.

### Local development
Don't confuse `gcloud auth login` with `gcloud auth application-default login` for local development,
since the first authorizes only the `gcloud` tool to access the Cloud Platform.

The latter obtains user access credentials via a web flow and puts them in the well-known location for ADC.
So to work for local development you need to use `gcloud auth application-default login`.
`gcloud auth application-default login --impersonate-service-account=...` runs the code as a service account instead.

### Routing headers
Some of the APIs (notably KMS, Artifact Registry and others) require additional headers such as `x-goog-request-params`.
You can find an example how to handle it [here](https://github.com/abdolence/kms-aead-rs/blob/b8bb496800625a660be0c9896366d7407b8aa714/src/providers/gcp_kms_encryption.rs#L114)

## Service-to-service authentication
The library supports Google-signed ID tokens on both sides of a call between services:
- minting them to call Cloud Run, Cloud Run functions and services behind Identity-Aware Proxy (IAP);
- attaching them to gRPC and HTTP requests;
- verifying them in the service that receives the call (`id-token-verify` feature);
- protecting an axum service (`axum` feature) and calling it with reqwest (`reqwest-middleware` feature).

Full examples: [axum service](examples/id-token-axum-server), [reqwest client](examples/id-token-reqwest-client).
[examples/id-token](examples/id-token) mints and verifies a token with the core API only,
and is the end-to-end check of the keyless CI workflow.

### Calling a Cloud Run or IAP service
`GoogleAuthTokenGenerator::id_token` mints ID tokens for one audience:
- the URL of a Cloud Run service, or a custom audience configured for it;
- the OAuth client ID of a resource behind IAP.

Google ID tokens are valid for one hour. `google-cloud-auth` caches the token and refreshes it before it expires.

gRPC (for example a tonic service on Cloud Run), with the default features:

```rust
use gcloud_sdk::{
    GoogleApi, GoogleAuthMiddleware, GoogleAuthMiddlewareLayer, GoogleAuthTokenGenerator,
    IdTokenAudience,
};

let audience = IdTokenAudience::new("https://orders-abc123-ew.a.run.app");
let id_tokens = GoogleAuthTokenGenerator::id_token(&audience).await?;

let orders_client: GoogleApi<OrdersClient<GoogleAuthMiddleware>> =
    GoogleApi::from_function_with_middleware(
        OrdersClient::new,
        "https://orders-abc123-ew.a.run.app",
        GoogleAuthMiddlewareLayer::new(id_tokens, None)?,
    )
    .await?;
```

HTTP with `reqwest-middleware` feature:

```toml
gcloud-sdk = { version = "0.33", features = ["reqwest-middleware"] }
reqwest = "0.13"
reqwest-middleware = "0.5"
```

```rust
use gcloud_sdk::GoogleAuthReqwestMiddleware;

let client = reqwest_middleware::ClientBuilder::new(reqwest::Client::new())
    .with(GoogleAuthReqwestMiddleware::new(id_tokens))
    .build();

let response = client
    .get("https://orders-abc123-ew.a.run.app/orders")
    .send()
    .await?;
```

Full example available [here](examples/id-token-reqwest-client).

Without the `reqwest-middleware` feature, `id_tokens.headers().await?` gives the headers, `authorization` among them, for any HTTP client.

### Which credentials work
`GoogleAuthTokenGenerator::id_token` mints with the identity of the ADC:
- service account key file: as that service account;
- metadata server (Cloud Run, GKE, Compute Engine, etc.): as the service account attached to the workload;
- `gcloud auth application-default login --impersonate-service-account=...` and workload identity
  federation with service account impersonation: as the impersonated service account.

User credentials from `gcloud auth application-default login` and workload identity federation without
service account impersonation cannot mint an ID token for an audience,
so `id_token` returns `ErrorKind::IdTokenNeedsImpersonation` for them.
Use `GoogleAuthTokenGenerator::id_token_impersonating` with any credentials to mint ID tokens of a service account through
IAM Credentials API:

```rust
use gcloud_sdk::google_cloud_auth::credentials::Builder as CredentialsBuilder;
use gcloud_sdk::{GoogleAuthTokenGenerator, IdTokenAudience, ServiceAccountEmail};

let id_tokens = GoogleAuthTokenGenerator::id_token_impersonating(
    &IdTokenAudience::new("https://orders-abc123-ew.a.run.app"),
    &ServiceAccountEmail::new("invoker@my-project.iam.gserviceaccount.com"),
    CredentialsBuilder::default().build()?,
)
.await?;
```

The caller needs `roles/iam.serviceAccountOpenIdTokenCreator` on that service account.
This is also the way for local development, using your own `gcloud auth application-default login` credentials.

For a key file or a metadata server that is not the ADC, build the ID token credentials with `google_cloud_auth::credentials::idtoken`
and pass them to `GoogleAuthTokenGenerator::from(..)`.

### Receiving and verifying tokens
`IdTokenVerifier` from `id-token-verify` feature checks:
- RS256 signature with Google keys from `https://www.googleapis.com/oauth2/v3/certs`;
- `iss` is Google, and `aud` is the audience of your service;
- `exp`, `nbf` and `iat`, with 30 seconds of leeway.

The keys are cached for the `max-age` of the response. A token signed with an unknown key ID refetches them,
at most once every 30 seconds. When a fetch fails, the cached keys stay in use until their `max-age` passes.

```toml
gcloud-sdk = { version = "0.33", features = ["id-token-verify"] }
```

```rust
use gcloud_sdk::id_token_verify::{IdTokenVerifier, IdTokenVerifyError};
use gcloud_sdk::IdTokenAudience;

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

```toml
gcloud-sdk = { version = "0.33", features = ["id-token-verify", "axum"] }
axum = "0.8"
```

```rust
use std::sync::Arc;

use axum::routing::get;
use axum::Router;
use gcloud_sdk::id_token_verify::{
    IdTokenVerifier, PrincipalEmail, VerifiedIdToken, VerifyIdTokenLayer,
};
use gcloud_sdk::IdTokenAudience;

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

Full example available [here](examples/id-token-axum-server).

### JWT crypto provider
`IdTokenVerifier` verifies JWTs with `jsonwebtoken`, which needs a crypto provider. The library has a feature for each:
- `jwt-aws-lc-rs`: default feature, uses aws-lc-rs, which is built from C sources and needs a C compiler;
- `jwt-rust-crypto`: the pure Rust alternative, uses the crates of RustCrypto.

With `default-features = false` enable one of them, or install a `jsonwebtoken` crypto provider in your application yourself.
With both enabled, aws-lc-rs is used.
The library installs the provider of the enabled feature as the process default of `jsonwebtoken`, unless your application installed one before.

These features choose the crypto of the verifier only. `auth-default-crypto` and `tls-webpki-roots` build aws-lc-rs for `google-cloud-auth` and rustls anyway.

## High-level APIs
Sometimes using proto generated APIs are tedious and cumbersome, so you may need to introduce facade APIs on top of them:
* [firestore](https://github.com/abdolence/firestore-rs) - to work with Firestore;
* [bigquery](https://github.com/abdolence/bigquery-rs) - to work with BigQuery;
* [secret-vault](https://github.com/abdolence/secret-vault-rs) - to read secrets from Google Secret Manager;
* [kms-aead](https://github.com/abdolence/kms-aead-rs) - envelope encryption using Google KMS and Ring AEAD;
* [opentelemetry-gcloud-trace](https://github.com/abdolence/opentelemetry-gcloud-trace-rs) - Google Cloud Trace support for OpenTelemetry project.

## License
Licensed under either of [Apache License, Version 2.0](./LICENSE-APACHE)
or [MIT license](./LICENSE-MIT) at your option.

## Authors
- Abdulla Abdurakhmanov

The library was started as a fork of [mechiru/googapis](https://github.com/mechiru/googapis) and [mechiru/gouth](https://github.com/mechiru/gouth) libraries, but now includes much more:

- Tower-based middleware layer that hides the complexity of working with credentials and TLS behind async clients;
- gRPC clients for every Google API, with a feature for each;
- Improved observability with tracing and measuring execution time of endpoints;
- Uses synchronisation primitives (such as Mutex) from tokio everywhere and has direct dependencies to tokio runtime;
- Security-related protocol extensions for Google Secret Manager and KMS;
- Service-to-service authentication with Google ID tokens: minting them, verifying them, a reqwest middleware and an axum layer.
