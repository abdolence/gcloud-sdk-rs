# Migrating gcloud-sdk

Breaking changes between minor versions, newest first.

## From 0.32 to 0.33
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
| `GoogleAuthTokenGenerator` | `GoogleAuthHeaders` |
| `GoogleAuthTokenGenerator::new(..)` for the ADC | `GoogleAuthHeaders::from_adc()`, `GoogleAuthHeaders::from_adc_with_scopes(..)` |
| `GoogleAuthTokenGenerator::new(..)`, `GoogleAuthTokenGenerator::from_source(..)` for other credentials | `GoogleAuthHeaders::from(..)` with `Credentials` or `IDTokenCredentials` |
| `authorization_header()`, `create_token()` | `headers()`: every authentication header, `authorization` included |
| `clear_cache()` | none, `google-cloud-auth` refreshes the tokens |
| `IdTokenSource::new(audience, TokenSourceType::Default)` | `GoogleAuthHeaders::id_token_from_adc(&audience)` |
| `IdTokenSource::impersonating(audience, service_account, TokenSourceType::Default)` | `GoogleAuthHeaders::id_token_impersonating(&audience, &service_account, credentials::Builder::default().build()?)` |
| `IdTokenSource::new(audience, Json / File / MetadataServer)` | `GoogleAuthHeaders::from(idtoken::{service_account, mds}::Builder::new(..).build()?)` |
| `from_function_with_token_source`, `from_function_with_token_source_and_headers` | `from_function_with_credentials`, `from_function_with_credentials_and_headers` |
| `with_token_source`, `with_token_source_and_headers` | `with_credentials`, `with_credentials_and_headers` |
| `with_token_source_and_middleware` | `with_middleware` |
| `GoogleEnvironment::find_default_creds` | none |
| `ErrorKind::Auth(AuthErrorDetails)` | `ErrorKind::Credentials(CredentialsError)`, and `ErrorKind::CredentialsBuild` when credentials cannot be built |
| `ErrorKind::Metadata` | none, it came from `GceMetadataClient` |
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
- `auth-default-crypto` (default): the aws-lc-rs rustls provider and ID token backend of `google-cloud-auth`;
- `jwt-custom-provider`: for applications that install their own `jsonwebtoken` crypto provider, see [JWT crypto provider](README.md#jwt-crypto-provider).

`id-token-verify` now needs one of `jwt-aws-lc-rs`, `jwt-rust-crypto` or `jwt-custom-provider`, otherwise the build fails.

### Behaviour changes
- The middleware sends every header the credentials produce, not only `authorization`, so the quota project reaches the server as `x-goog-user-project`.
- Tokens are refreshed before they expire, and failed token requests are retried.
- Service account keys sign a JWT that is used as the access token directly, without the exchange at `token_uri`.
- `GoogleEnvironment::detect_google_project_id` takes the first project ID it finds:
  1. the `GCP_PROJECT`, `PROJECT_ID`, `GCP_PROJECT_ID` and `GOOGLE_CLOUD_PROJECT` environment variables;
  2. the `project_id` of the ADC file, then its `quota_project_id`;
  3. the `project_id` of the source credentials in the ADC file, then their `quota_project_id`;
  4. the metadata server at `GCE_METADATA_HOST`, else `metadata.google.internal`, the same host `google-cloud-auth` uses.
     Only an answer with `Metadata-Flavor: Google` counts, and `169.254.169.254` is no longer probed.
- Workload identity federation without service account impersonation fails `GoogleAuthHeaders::id_token_from_adc` with
  `ErrorKind::IdTokenNeedsImpersonation`, the same way as user credentials. With impersonation it mints as the impersonated service account.
- With `default-features = false` and no rustls `CryptoProvider` installed, the ADC constructors return `ErrorKind::CryptoProviderMissing`
  instead of a panic when the ADC is a service account key, see [crypto providers](README.md#crypto-providers).

### Known limits
- **MSRV is 1.91**, the MSRV of `google-cloud-auth`.
- **Credentials must be built inside a Tokio runtime.** Building them spawns their refresh task.
- **aws-lc-rs comes in by default** through `auth-default-crypto`, and it is built from C sources.
  With `default-features = false`, install a rustls `CryptoProvider` before building credentials, see [crypto providers](README.md#crypto-providers).
- **AWS workload identity federation reads only the environment keys and EC2 IMDS.** ECS task roles, `~/.aws` profiles and SSO no longer work.
- **Service account keys must be PKCS#8**, the format Google issues them in.

### REST APIs moved to the official SDK
0.33 removes the REST clients generated from OpenAPI specs. Each one has an official crate, except FCM and Identity Toolkit v3.
Each official crate was called against a live project, authenticated through `google-cloud-auth` 1.17.0:
- `storage_v1`, `cloudresourcemanager_v3`, `bigquery_v2`, `compute_v1`, `dns_v1` and `sqladmin_v1`: a read-only call returned data;
- `lustre_v1`: the API is disabled on the test project, and the call returned `SERVICE_DISABLED`;
- `servicecontrol_v1` and `servicecontrol_v2`: a `check` on a non-existent service returned `PERMISSION_DENIED`.

For the last three, the calls show only that the request reached Google with valid authentication.

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
