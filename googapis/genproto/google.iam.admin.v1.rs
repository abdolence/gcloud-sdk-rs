/// Audit log information specific to Cloud IAM admin APIs. This message is
/// serialized as an `Any` type in the `ServiceData` message of an
/// `AuditLog` message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuditData {
    /// The permission_delta when when creating or updating a Role.
    #[prost(message, optional, tag = "1")]
    pub permission_delta: ::core::option::Option<audit_data::PermissionDelta>,
}
/// Nested message and enum types in `AuditData`.
pub mod audit_data {
    /// A PermissionDelta message to record the added_permissions and
    /// removed_permissions inside a role.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PermissionDelta {
        /// Added permissions.
        #[prost(string, repeated, tag = "1")]
        pub added_permissions: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// Removed permissions.
        #[prost(string, repeated, tag = "2")]
        pub removed_permissions: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
}
/// An IAM service account.
///
/// A service account is an account for an application or a virtual machine (VM)
/// instance, not a person. You can use a service account to call Google APIs. To
/// learn more, read the [overview of service
/// accounts](<https://cloud.google.com/iam/help/service-accounts/overview>).
///
/// When you create a service account, you specify the project ID that owns the
/// service account, as well as a name that must be unique within the project.
/// IAM uses these values to create an email address that identifies the service
/// account.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServiceAccount {
    /// The resource name of the service account.
    ///
    /// Use one of the following formats:
    ///
    /// * `projects/{PROJECT_ID}/serviceAccounts/{EMAIL_ADDRESS}`
    /// * `projects/{PROJECT_ID}/serviceAccounts/{UNIQUE_ID}`
    ///
    /// As an alternative, you can use the `-` wildcard character instead of the
    /// project ID:
    ///
    /// * `projects/-/serviceAccounts/{EMAIL_ADDRESS}`
    /// * `projects/-/serviceAccounts/{UNIQUE_ID}`
    ///
    /// When possible, avoid using the `-` wildcard character, because it can cause
    /// response messages to contain misleading error codes. For example, if you
    /// try to get the service account
    /// `projects/-/serviceAccounts/fake@example.com`, which does not exist, the
    /// response contains an HTTP `403 Forbidden` error instead of a `404 Not
    /// Found` error.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. The ID of the project that owns the service account.
    #[prost(string, tag = "2")]
    pub project_id: ::prost::alloc::string::String,
    /// Output only. The unique, stable numeric ID for the service account.
    ///
    /// Each service account retains its unique ID even if you delete the service
    /// account. For example, if you delete a service account, then create a new
    /// service account with the same name, the new service account has a different
    /// unique ID than the deleted service account.
    #[prost(string, tag = "4")]
    pub unique_id: ::prost::alloc::string::String,
    /// Output only. The email address of the service account.
    #[prost(string, tag = "5")]
    pub email: ::prost::alloc::string::String,
    /// Optional. A user-specified, human-readable name for the service account. The maximum
    /// length is 100 UTF-8 bytes.
    #[prost(string, tag = "6")]
    pub display_name: ::prost::alloc::string::String,
    /// Deprecated. Do not use.
    #[deprecated]
    #[prost(bytes = "vec", tag = "7")]
    pub etag: ::prost::alloc::vec::Vec<u8>,
    /// Optional. A user-specified, human-readable description of the service account. The
    /// maximum length is 256 UTF-8 bytes.
    #[prost(string, tag = "8")]
    pub description: ::prost::alloc::string::String,
    /// Output only. The OAuth 2.0 client ID for the service account.
    #[prost(string, tag = "9")]
    pub oauth2_client_id: ::prost::alloc::string::String,
    /// Output only. Whether the service account is disabled.
    #[prost(bool, tag = "11")]
    pub disabled: bool,
}
/// The service account create request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateServiceAccountRequest {
    /// Required. The resource name of the project associated with the service
    /// accounts, such as `projects/my-project-123`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The account id that is used to generate the service account
    /// email address and a stable unique id. It is unique within a project,
    /// must be 6-30 characters long, and match the regular expression
    /// `\[a-z]([-a-z0-9]*[a-z0-9\])` to comply with RFC1035.
    #[prost(string, tag = "2")]
    pub account_id: ::prost::alloc::string::String,
    /// The \[ServiceAccount][google.iam.admin.v1.ServiceAccount\] resource to
    /// create. Currently, only the following values are user assignable:
    /// `display_name` and `description`.
    #[prost(message, optional, tag = "3")]
    pub service_account: ::core::option::Option<ServiceAccount>,
}
/// The service account list request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListServiceAccountsRequest {
    /// Required. The resource name of the project associated with the service
    /// accounts, such as `projects/my-project-123`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional limit on the number of service accounts to include in the
    /// response. Further accounts can subsequently be obtained by including the
    /// \[ListServiceAccountsResponse.next_page_token][google.iam.admin.v1.ListServiceAccountsResponse.next_page_token\]
    /// in a subsequent request.
    ///
    /// The default is 20, and the maximum is 100.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional pagination token returned in an earlier
    /// \[ListServiceAccountsResponse.next_page_token][google.iam.admin.v1.ListServiceAccountsResponse.next_page_token\].
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// The service account list response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListServiceAccountsResponse {
    /// The list of matching service accounts.
    #[prost(message, repeated, tag = "1")]
    pub accounts: ::prost::alloc::vec::Vec<ServiceAccount>,
    /// To retrieve the next page of results, set
    /// \[ListServiceAccountsRequest.page_token][google.iam.admin.v1.ListServiceAccountsRequest.page_token\]
    /// to this value.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// The service account get request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetServiceAccountRequest {
    /// Required. The resource name of the service account in the following format:
    /// `projects/{PROJECT_ID}/serviceAccounts/{ACCOUNT}`.
    /// Using `-` as a wildcard for the `PROJECT_ID` will infer the project from
    /// the account. The `ACCOUNT` value can be the `email` address or the
    /// `unique_id` of the service account.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// The service account delete request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteServiceAccountRequest {
    /// Required. The resource name of the service account in the following format:
    /// `projects/{PROJECT_ID}/serviceAccounts/{ACCOUNT}`.
    /// Using `-` as a wildcard for the `PROJECT_ID` will infer the project from
    /// the account. The `ACCOUNT` value can be the `email` address or the
    /// `unique_id` of the service account.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// The request for
/// \[PatchServiceAccount][google.iam.admin.v1.PatchServiceAccount\].
///
/// You can patch only the `display_name` and `description` fields. You must use
/// the `update_mask` field to specify which of these fields you want to patch.
///
/// Only the fields specified in the request are guaranteed to be returned in
/// the response. Other fields may be empty in the response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PatchServiceAccountRequest {
    #[prost(message, optional, tag = "1")]
    pub service_account: ::core::option::Option<ServiceAccount>,
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// The service account undelete request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UndeleteServiceAccountRequest {
    /// The resource name of the service account in the following format:
    /// `projects/{PROJECT_ID}/serviceAccounts/{ACCOUNT_UNIQUE_ID}`.
    /// Using `-` as a wildcard for the `PROJECT_ID` will infer the project from
    /// the account.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UndeleteServiceAccountResponse {
    /// Metadata for the restored service account.
    #[prost(message, optional, tag = "1")]
    pub restored_account: ::core::option::Option<ServiceAccount>,
}
/// The service account enable request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EnableServiceAccountRequest {
    /// The resource name of the service account in the following format:
    /// `projects/{PROJECT_ID}/serviceAccounts/{ACCOUNT}`.
    /// Using `-` as a wildcard for the `PROJECT_ID` will infer the project from
    /// the account. The `ACCOUNT` value can be the `email` address or the
    /// `unique_id` of the service account.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// The service account disable request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DisableServiceAccountRequest {
    /// The resource name of the service account in the following format:
    /// `projects/{PROJECT_ID}/serviceAccounts/{ACCOUNT}`.
    /// Using `-` as a wildcard for the `PROJECT_ID` will infer the project from
    /// the account. The `ACCOUNT` value can be the `email` address or the
    /// `unique_id` of the service account.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// The service account keys list request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListServiceAccountKeysRequest {
    /// Required. The resource name of the service account in the following format:
    /// `projects/{PROJECT_ID}/serviceAccounts/{ACCOUNT}`.
    ///
    /// Using `-` as a wildcard for the `PROJECT_ID`, will infer the project from
    /// the account. The `ACCOUNT` value can be the `email` address or the
    /// `unique_id` of the service account.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Filters the types of keys the user wants to include in the list
    /// response. Duplicate key types are not allowed. If no key type
    /// is provided, all keys are returned.
    #[prost(
        enumeration = "list_service_account_keys_request::KeyType",
        repeated,
        tag = "2"
    )]
    pub key_types: ::prost::alloc::vec::Vec<i32>,
}
/// Nested message and enum types in `ListServiceAccountKeysRequest`.
pub mod list_service_account_keys_request {
    /// `KeyType` filters to selectively retrieve certain varieties
    /// of keys.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum KeyType {
        /// Unspecified key type. The presence of this in the
        /// message will immediately result in an error.
        Unspecified = 0,
        /// User-managed keys (managed and rotated by the user).
        UserManaged = 1,
        /// System-managed keys (managed and rotated by Google).
        SystemManaged = 2,
    }
}
/// The service account keys list response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListServiceAccountKeysResponse {
    /// The public keys for the service account.
    #[prost(message, repeated, tag = "1")]
    pub keys: ::prost::alloc::vec::Vec<ServiceAccountKey>,
}
/// The service account key get by id request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetServiceAccountKeyRequest {
    /// Required. The resource name of the service account key in the following format:
    /// `projects/{PROJECT_ID}/serviceAccounts/{ACCOUNT}/keys/{key}`.
    ///
    /// Using `-` as a wildcard for the `PROJECT_ID` will infer the project from
    /// the account. The `ACCOUNT` value can be the `email` address or the
    /// `unique_id` of the service account.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The output format of the public key requested.
    /// X509_PEM is the default output format.
    #[prost(enumeration = "ServiceAccountPublicKeyType", tag = "2")]
    pub public_key_type: i32,
}
/// Represents a service account key.
///
/// A service account has two sets of key-pairs: user-managed, and
/// system-managed.
///
/// User-managed key-pairs can be created and deleted by users.  Users are
/// responsible for rotating these keys periodically to ensure security of
/// their service accounts.  Users retain the private key of these key-pairs,
/// and Google retains ONLY the public key.
///
/// System-managed keys are automatically rotated by Google, and are used for
/// signing for a maximum of two weeks. The rotation process is probabilistic,
/// and usage of the new key will gradually ramp up and down over the key's
/// lifetime.
///
/// If you cache the public key set for a service account, we recommend that you
/// update the cache every 15 minutes. User-managed keys can be added and removed
/// at any time, so it is important to update the cache frequently. For
/// Google-managed keys, Google will publish a key at least 6 hours before it is
/// first used for signing and will keep publishing it for at least 6 hours after
/// it was last used for signing.
///
/// Public keys for all service accounts are also published at the OAuth2
/// Service Account API.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServiceAccountKey {
    /// The resource name of the service account key in the following format
    /// `projects/{PROJECT_ID}/serviceAccounts/{ACCOUNT}/keys/{key}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The output format for the private key.
    /// Only provided in `CreateServiceAccountKey` responses, not
    /// in `GetServiceAccountKey` or `ListServiceAccountKey` responses.
    ///
    /// Google never exposes system-managed private keys, and never retains
    /// user-managed private keys.
    #[prost(enumeration = "ServiceAccountPrivateKeyType", tag = "2")]
    pub private_key_type: i32,
    /// Specifies the algorithm (and possibly key size) for the key.
    #[prost(enumeration = "ServiceAccountKeyAlgorithm", tag = "8")]
    pub key_algorithm: i32,
    /// The private key data. Only provided in `CreateServiceAccountKey`
    /// responses. Make sure to keep the private key data secure because it
    /// allows for the assertion of the service account identity.
    /// When base64 decoded, the private key data can be used to authenticate with
    /// Google API client libraries and with
    /// <a href="/sdk/gcloud/reference/auth/activate-service-account">gcloud
    /// auth activate-service-account</a>.
    #[prost(bytes = "vec", tag = "3")]
    pub private_key_data: ::prost::alloc::vec::Vec<u8>,
    /// The public key data. Only provided in `GetServiceAccountKey` responses.
    #[prost(bytes = "vec", tag = "7")]
    pub public_key_data: ::prost::alloc::vec::Vec<u8>,
    /// The key can be used after this timestamp.
    #[prost(message, optional, tag = "4")]
    pub valid_after_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The key can be used before this timestamp.
    /// For system-managed key pairs, this timestamp is the end time for the
    /// private key signing operation. The public key could still be used
    /// for verification for a few hours after this time.
    #[prost(message, optional, tag = "5")]
    pub valid_before_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The key origin.
    #[prost(enumeration = "ServiceAccountKeyOrigin", tag = "9")]
    pub key_origin: i32,
    /// The key type.
    #[prost(enumeration = "list_service_account_keys_request::KeyType", tag = "10")]
    pub key_type: i32,
}
/// The service account key create request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateServiceAccountKeyRequest {
    /// Required. The resource name of the service account in the following format:
    /// `projects/{PROJECT_ID}/serviceAccounts/{ACCOUNT}`.
    /// Using `-` as a wildcard for the `PROJECT_ID` will infer the project from
    /// the account. The `ACCOUNT` value can be the `email` address or the
    /// `unique_id` of the service account.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The output format of the private key. The default value is
    /// `TYPE_GOOGLE_CREDENTIALS_FILE`, which is the Google Credentials File
    /// format.
    #[prost(enumeration = "ServiceAccountPrivateKeyType", tag = "2")]
    pub private_key_type: i32,
    /// Which type of key and algorithm to use for the key.
    /// The default is currently a 2K RSA key.  However this may change in the
    /// future.
    #[prost(enumeration = "ServiceAccountKeyAlgorithm", tag = "3")]
    pub key_algorithm: i32,
}
/// The service account key upload request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UploadServiceAccountKeyRequest {
    /// The resource name of the service account in the following format:
    /// `projects/{PROJECT_ID}/serviceAccounts/{ACCOUNT}`.
    /// Using `-` as a wildcard for the `PROJECT_ID` will infer the project from
    /// the account. The `ACCOUNT` value can be the `email` address or the
    /// `unique_id` of the service account.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// A field that allows clients to upload their own public key. If set,
    /// use this public key data to create a service account key for given
    /// service account.
    /// Please note, the expected format for this field is X509_PEM.
    #[prost(bytes = "vec", tag = "2")]
    pub public_key_data: ::prost::alloc::vec::Vec<u8>,
}
/// The service account key delete request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteServiceAccountKeyRequest {
    /// Required. The resource name of the service account key in the following format:
    /// `projects/{PROJECT_ID}/serviceAccounts/{ACCOUNT}/keys/{key}`.
    /// Using `-` as a wildcard for the `PROJECT_ID` will infer the project from
    /// the account. The `ACCOUNT` value can be the `email` address or the
    /// `unique_id` of the service account.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Deprecated. [Migrate to Service Account Credentials
/// API](<https://cloud.google.com/iam/help/credentials/migrate-api>).
///
/// The service account sign blob request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignBlobRequest {
    /// Required. Deprecated. [Migrate to Service Account Credentials
    /// API](<https://cloud.google.com/iam/help/credentials/migrate-api>).
    ///
    /// The resource name of the service account in the following format:
    /// `projects/{PROJECT_ID}/serviceAccounts/{ACCOUNT}`.
    /// Using `-` as a wildcard for the `PROJECT_ID` will infer the project from
    /// the account. The `ACCOUNT` value can be the `email` address or the
    /// `unique_id` of the service account.
    #[deprecated]
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. Deprecated. [Migrate to Service Account Credentials
    /// API](<https://cloud.google.com/iam/help/credentials/migrate-api>).
    ///
    /// The bytes to sign.
    #[deprecated]
    #[prost(bytes = "vec", tag = "2")]
    pub bytes_to_sign: ::prost::alloc::vec::Vec<u8>,
}
/// Deprecated. [Migrate to Service Account Credentials
/// API](<https://cloud.google.com/iam/help/credentials/migrate-api>).
///
/// The service account sign blob response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignBlobResponse {
    /// Deprecated. [Migrate to Service Account Credentials
    /// API](<https://cloud.google.com/iam/help/credentials/migrate-api>).
    ///
    /// The id of the key used to sign the blob.
    #[deprecated]
    #[prost(string, tag = "1")]
    pub key_id: ::prost::alloc::string::String,
    /// Deprecated. [Migrate to Service Account Credentials
    /// API](<https://cloud.google.com/iam/help/credentials/migrate-api>).
    ///
    /// The signed blob.
    #[deprecated]
    #[prost(bytes = "vec", tag = "2")]
    pub signature: ::prost::alloc::vec::Vec<u8>,
}
/// Deprecated. [Migrate to Service Account Credentials
/// API](<https://cloud.google.com/iam/help/credentials/migrate-api>).
///
/// The service account sign JWT request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignJwtRequest {
    /// Required. Deprecated. [Migrate to Service Account Credentials
    /// API](<https://cloud.google.com/iam/help/credentials/migrate-api>).
    ///
    /// The resource name of the service account in the following format:
    /// `projects/{PROJECT_ID}/serviceAccounts/{ACCOUNT}`.
    /// Using `-` as a wildcard for the `PROJECT_ID` will infer the project from
    /// the account. The `ACCOUNT` value can be the `email` address or the
    /// `unique_id` of the service account.
    #[deprecated]
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. Deprecated. [Migrate to Service Account Credentials
    /// API](<https://cloud.google.com/iam/help/credentials/migrate-api>).
    ///
    /// The JWT payload to sign. Must be a serialized JSON object that contains a
    /// JWT Claims Set. For example: `{"sub": "user@example.com", "iat": 313435}`
    ///
    /// If the JWT Claims Set contains an expiration time (`exp`) claim, it must be
    /// an integer timestamp that is not in the past and no more than 1 hour in the
    /// future.
    ///
    /// If the JWT Claims Set does not contain an expiration time (`exp`) claim,
    /// this claim is added automatically, with a timestamp that is 1 hour in the
    /// future.
    #[deprecated]
    #[prost(string, tag = "2")]
    pub payload: ::prost::alloc::string::String,
}
/// Deprecated. [Migrate to Service Account Credentials
/// API](<https://cloud.google.com/iam/help/credentials/migrate-api>).
///
/// The service account sign JWT response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignJwtResponse {
    /// Deprecated. [Migrate to Service Account Credentials
    /// API](<https://cloud.google.com/iam/help/credentials/migrate-api>).
    ///
    /// The id of the key used to sign the JWT.
    #[deprecated]
    #[prost(string, tag = "1")]
    pub key_id: ::prost::alloc::string::String,
    /// Deprecated. [Migrate to Service Account Credentials
    /// API](<https://cloud.google.com/iam/help/credentials/migrate-api>).
    ///
    /// The signed JWT.
    #[deprecated]
    #[prost(string, tag = "2")]
    pub signed_jwt: ::prost::alloc::string::String,
}
/// A role in the Identity and Access Management API.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Role {
    /// The name of the role.
    ///
    /// When Role is used in CreateRole, the role name must not be set.
    ///
    /// When Role is used in output and other input such as UpdateRole, the role
    /// name is the complete path, e.g., roles/logging.viewer for predefined roles
    /// and organizations/{ORGANIZATION_ID}/roles/logging.viewer for custom roles.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. A human-readable title for the role.  Typically this
    /// is limited to 100 UTF-8 bytes.
    #[prost(string, tag = "2")]
    pub title: ::prost::alloc::string::String,
    /// Optional. A human-readable description for the role.
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    /// The names of the permissions this role grants when bound in an IAM policy.
    #[prost(string, repeated, tag = "7")]
    pub included_permissions: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The current launch stage of the role. If the `ALPHA` launch stage has been
    /// selected for a role, the `stage` field will not be included in the
    /// returned definition for the role.
    #[prost(enumeration = "role::RoleLaunchStage", tag = "8")]
    pub stage: i32,
    /// Used to perform a consistent read-modify-write.
    #[prost(bytes = "vec", tag = "9")]
    pub etag: ::prost::alloc::vec::Vec<u8>,
    /// The current deleted state of the role. This field is read only.
    /// It will be ignored in calls to CreateRole and UpdateRole.
    #[prost(bool, tag = "11")]
    pub deleted: bool,
}
/// Nested message and enum types in `Role`.
pub mod role {
    /// A stage representing a role's lifecycle phase.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum RoleLaunchStage {
        /// The user has indicated this role is currently in an Alpha phase. If this
        /// launch stage is selected, the `stage` field will not be included when
        /// requesting the definition for a given role.
        Alpha = 0,
        /// The user has indicated this role is currently in a Beta phase.
        Beta = 1,
        /// The user has indicated this role is generally available.
        Ga = 2,
        /// The user has indicated this role is being deprecated.
        Deprecated = 4,
        /// This role is disabled and will not contribute permissions to any members
        /// it is granted to in policies.
        Disabled = 5,
        /// The user has indicated this role is currently in an EAP phase.
        Eap = 6,
    }
}
/// The grantable role query request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGrantableRolesRequest {
    /// Required. The full resource name to query from the list of grantable roles.
    ///
    /// The name follows the Google Cloud Platform resource format.
    /// For example, a Cloud Platform project with id `my-project` will be named
    /// `//cloudresourcemanager.googleapis.com/projects/my-project`.
    #[prost(string, tag = "1")]
    pub full_resource_name: ::prost::alloc::string::String,
    #[prost(enumeration = "RoleView", tag = "2")]
    pub view: i32,
    /// Optional limit on the number of roles to include in the response.
    ///
    /// The default is 300, and the maximum is 1,000.
    #[prost(int32, tag = "3")]
    pub page_size: i32,
    /// Optional pagination token returned in an earlier
    /// QueryGrantableRolesResponse.
    #[prost(string, tag = "4")]
    pub page_token: ::prost::alloc::string::String,
}
/// The grantable role query response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGrantableRolesResponse {
    /// The list of matching roles.
    #[prost(message, repeated, tag = "1")]
    pub roles: ::prost::alloc::vec::Vec<Role>,
    /// To retrieve the next page of results, set
    /// `QueryGrantableRolesRequest.page_token` to this value.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// The request to get all roles defined under a resource.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListRolesRequest {
    /// The `parent` parameter's value depends on the target resource for the
    /// request, namely
    /// \[`roles`\](/iam/reference/rest/v1/roles),
    /// \[`projects`\](/iam/reference/rest/v1/projects.roles), or
    /// \[`organizations`\](/iam/reference/rest/v1/organizations.roles). Each
    /// resource type's `parent` value format is described below:
    ///
    /// * \[`roles.list()`\](/iam/reference/rest/v1/roles/list): An empty string.
    ///   This method doesn't require a resource; it simply returns all
    ///   [predefined roles](/iam/docs/understanding-roles#predefined_roles) in
    ///   Cloud IAM. Example request URL:
    ///   `<https://iam.googleapis.com/v1/roles`>
    ///
    /// * \[`projects.roles.list()`\](/iam/reference/rest/v1/projects.roles/list):
    ///   `projects/{PROJECT_ID}`. This method lists all project-level
    ///   [custom roles](/iam/docs/understanding-custom-roles).
    ///   Example request URL:
    ///   `<https://iam.googleapis.com/v1/projects/{PROJECT_ID}/roles`>
    ///
    /// * \[`organizations.roles.list()`\](/iam/reference/rest/v1/organizations.roles/list):
    ///   `organizations/{ORGANIZATION_ID}`. This method lists all
    ///   organization-level [custom roles](/iam/docs/understanding-custom-roles).
    ///   Example request URL:
    ///   `<https://iam.googleapis.com/v1/organizations/{ORGANIZATION_ID}/roles`>
    ///
    /// Note: Wildcard (*) values are invalid; you must specify a complete project
    /// ID or organization ID.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional limit on the number of roles to include in the response.
    ///
    /// The default is 300, and the maximum is 1,000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional pagination token returned in an earlier ListRolesResponse.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional view for the returned Role objects. When `FULL` is specified,
    /// the `includedPermissions` field is returned, which includes a list of all
    /// permissions in the role. The default value is `BASIC`, which does not
    /// return the `includedPermissions` field.
    #[prost(enumeration = "RoleView", tag = "4")]
    pub view: i32,
    /// Include Roles that have been deleted.
    #[prost(bool, tag = "6")]
    pub show_deleted: bool,
}
/// The response containing the roles defined under a resource.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListRolesResponse {
    /// The Roles defined on this resource.
    #[prost(message, repeated, tag = "1")]
    pub roles: ::prost::alloc::vec::Vec<Role>,
    /// To retrieve the next page of results, set
    /// `ListRolesRequest.page_token` to this value.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// The request to get the definition of an existing role.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRoleRequest {
    /// The `name` parameter's value depends on the target resource for the
    /// request, namely
    /// \[`roles`\](/iam/reference/rest/v1/roles),
    /// \[`projects`\](/iam/reference/rest/v1/projects.roles), or
    /// \[`organizations`\](/iam/reference/rest/v1/organizations.roles). Each
    /// resource type's `name` value format is described below:
    ///
    /// * \[`roles.get()`\](/iam/reference/rest/v1/roles/get): `roles/{ROLE_NAME}`.
    ///   This method returns results from all
    ///   [predefined roles](/iam/docs/understanding-roles#predefined_roles) in
    ///   Cloud IAM. Example request URL:
    ///   `<https://iam.googleapis.com/v1/roles/{ROLE_NAME}`>
    ///
    /// * \[`projects.roles.get()`\](/iam/reference/rest/v1/projects.roles/get):
    ///   `projects/{PROJECT_ID}/roles/{CUSTOM_ROLE_ID}`. This method returns only
    ///   [custom roles](/iam/docs/understanding-custom-roles) that have been
    ///   created at the project level. Example request URL:
    ///   `<https://iam.googleapis.com/v1/projects/{PROJECT_ID}/roles/{CUSTOM_ROLE_ID}`>
    ///
    /// * \[`organizations.roles.get()`\](/iam/reference/rest/v1/organizations.roles/get):
    ///   `organizations/{ORGANIZATION_ID}/roles/{CUSTOM_ROLE_ID}`. This method
    ///   returns only [custom roles](/iam/docs/understanding-custom-roles) that
    ///   have been created at the organization level. Example request URL:
    ///   `<https://iam.googleapis.com/v1/organizations/{ORGANIZATION_ID}/roles/{CUSTOM_ROLE_ID}`>
    ///
    /// Note: Wildcard (*) values are invalid; you must specify a complete project
    /// ID or organization ID.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// The request to create a new role.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateRoleRequest {
    /// The `parent` parameter's value depends on the target resource for the
    /// request, namely
    /// \[`projects`\](/iam/reference/rest/v1/projects.roles) or
    /// \[`organizations`\](/iam/reference/rest/v1/organizations.roles). Each
    /// resource type's `parent` value format is described below:
    ///
    /// * \[`projects.roles.create()`\](/iam/reference/rest/v1/projects.roles/create):
    ///   `projects/{PROJECT_ID}`. This method creates project-level
    ///   [custom roles](/iam/docs/understanding-custom-roles).
    ///   Example request URL:
    ///   `<https://iam.googleapis.com/v1/projects/{PROJECT_ID}/roles`>
    ///
    /// * \[`organizations.roles.create()`\](/iam/reference/rest/v1/organizations.roles/create):
    ///   `organizations/{ORGANIZATION_ID}`. This method creates organization-level
    ///   [custom roles](/iam/docs/understanding-custom-roles). Example request
    ///   URL:
    ///   `<https://iam.googleapis.com/v1/organizations/{ORGANIZATION_ID}/roles`>
    ///
    /// Note: Wildcard (*) values are invalid; you must specify a complete project
    /// ID or organization ID.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The role ID to use for this role.
    ///
    /// A role ID may contain alphanumeric characters, underscores (`_`), and
    /// periods (`.`). It must contain a minimum of 3 characters and a maximum of
    /// 64 characters.
    #[prost(string, tag = "2")]
    pub role_id: ::prost::alloc::string::String,
    /// The Role resource to create.
    #[prost(message, optional, tag = "3")]
    pub role: ::core::option::Option<Role>,
}
/// The request to update a role.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateRoleRequest {
    /// The `name` parameter's value depends on the target resource for the
    /// request, namely
    /// \[`projects`\](/iam/reference/rest/v1/projects.roles) or
    /// \[`organizations`\](/iam/reference/rest/v1/organizations.roles). Each
    /// resource type's `name` value format is described below:
    ///
    /// * \[`projects.roles.patch()`\](/iam/reference/rest/v1/projects.roles/patch):
    ///   `projects/{PROJECT_ID}/roles/{CUSTOM_ROLE_ID}`. This method updates only
    ///   [custom roles](/iam/docs/understanding-custom-roles) that have been
    ///   created at the project level. Example request URL:
    ///   `<https://iam.googleapis.com/v1/projects/{PROJECT_ID}/roles/{CUSTOM_ROLE_ID}`>
    ///
    /// * \[`organizations.roles.patch()`\](/iam/reference/rest/v1/organizations.roles/patch):
    ///   `organizations/{ORGANIZATION_ID}/roles/{CUSTOM_ROLE_ID}`. This method
    ///   updates only [custom roles](/iam/docs/understanding-custom-roles) that
    ///   have been created at the organization level. Example request URL:
    ///   `<https://iam.googleapis.com/v1/organizations/{ORGANIZATION_ID}/roles/{CUSTOM_ROLE_ID}`>
    ///
    /// Note: Wildcard (*) values are invalid; you must specify a complete project
    /// ID or organization ID.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The updated role.
    #[prost(message, optional, tag = "2")]
    pub role: ::core::option::Option<Role>,
    /// A mask describing which fields in the Role have changed.
    #[prost(message, optional, tag = "3")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// The request to delete an existing role.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteRoleRequest {
    /// The `name` parameter's value depends on the target resource for the
    /// request, namely
    /// \[`projects`\](/iam/reference/rest/v1/projects.roles) or
    /// \[`organizations`\](/iam/reference/rest/v1/organizations.roles). Each
    /// resource type's `name` value format is described below:
    ///
    /// * \[`projects.roles.delete()`\](/iam/reference/rest/v1/projects.roles/delete):
    ///   `projects/{PROJECT_ID}/roles/{CUSTOM_ROLE_ID}`. This method deletes only
    ///   [custom roles](/iam/docs/understanding-custom-roles) that have been
    ///   created at the project level. Example request URL:
    ///   `<https://iam.googleapis.com/v1/projects/{PROJECT_ID}/roles/{CUSTOM_ROLE_ID}`>
    ///
    /// * \[`organizations.roles.delete()`\](/iam/reference/rest/v1/organizations.roles/delete):
    ///   `organizations/{ORGANIZATION_ID}/roles/{CUSTOM_ROLE_ID}`. This method
    ///   deletes only [custom roles](/iam/docs/understanding-custom-roles) that
    ///   have been created at the organization level. Example request URL:
    ///   `<https://iam.googleapis.com/v1/organizations/{ORGANIZATION_ID}/roles/{CUSTOM_ROLE_ID}`>
    ///
    /// Note: Wildcard (*) values are invalid; you must specify a complete project
    /// ID or organization ID.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Used to perform a consistent read-modify-write.
    #[prost(bytes = "vec", tag = "2")]
    pub etag: ::prost::alloc::vec::Vec<u8>,
}
/// The request to undelete an existing role.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UndeleteRoleRequest {
    /// The `name` parameter's value depends on the target resource for the
    /// request, namely
    /// \[`projects`\](/iam/reference/rest/v1/projects.roles) or
    /// \[`organizations`\](/iam/reference/rest/v1/organizations.roles). Each
    /// resource type's `name` value format is described below:
    ///
    /// * \[`projects.roles.undelete()`\](/iam/reference/rest/v1/projects.roles/undelete):
    ///   `projects/{PROJECT_ID}/roles/{CUSTOM_ROLE_ID}`. This method undeletes
    ///   only [custom roles](/iam/docs/understanding-custom-roles) that have been
    ///   created at the project level. Example request URL:
    ///   `<https://iam.googleapis.com/v1/projects/{PROJECT_ID}/roles/{CUSTOM_ROLE_ID}`>
    ///
    /// * \[`organizations.roles.undelete()`\](/iam/reference/rest/v1/organizations.roles/undelete):
    ///   `organizations/{ORGANIZATION_ID}/roles/{CUSTOM_ROLE_ID}`. This method
    ///   undeletes only [custom roles](/iam/docs/understanding-custom-roles) that
    ///   have been created at the organization level. Example request URL:
    ///   `<https://iam.googleapis.com/v1/organizations/{ORGANIZATION_ID}/roles/{CUSTOM_ROLE_ID}`>
    ///
    /// Note: Wildcard (*) values are invalid; you must specify a complete project
    /// ID or organization ID.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Used to perform a consistent read-modify-write.
    #[prost(bytes = "vec", tag = "2")]
    pub etag: ::prost::alloc::vec::Vec<u8>,
}
/// A permission which can be included by a role.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Permission {
    /// The name of this Permission.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The title of this Permission.
    #[prost(string, tag = "2")]
    pub title: ::prost::alloc::string::String,
    /// A brief description of what this Permission is used for.
    /// This permission can ONLY be used in predefined roles.
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    #[deprecated]
    #[prost(bool, tag = "4")]
    pub only_in_predefined_roles: bool,
    /// The current launch stage of the permission.
    #[prost(enumeration = "permission::PermissionLaunchStage", tag = "5")]
    pub stage: i32,
    /// The current custom role support level.
    #[prost(enumeration = "permission::CustomRolesSupportLevel", tag = "6")]
    pub custom_roles_support_level: i32,
    /// The service API associated with the permission is not enabled.
    #[prost(bool, tag = "7")]
    pub api_disabled: bool,
    /// The preferred name for this permission. If present, then this permission is
    /// an alias of, and equivalent to, the listed primary_permission.
    #[prost(string, tag = "8")]
    pub primary_permission: ::prost::alloc::string::String,
}
/// Nested message and enum types in `Permission`.
pub mod permission {
    /// A stage representing a permission's lifecycle phase.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum PermissionLaunchStage {
        /// The permission is currently in an alpha phase.
        Alpha = 0,
        /// The permission is currently in a beta phase.
        Beta = 1,
        /// The permission is generally available.
        Ga = 2,
        /// The permission is being deprecated.
        Deprecated = 3,
    }
    /// The state of the permission with regards to custom roles.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum CustomRolesSupportLevel {
        /// Permission is fully supported for custom role use.
        Supported = 0,
        /// Permission is being tested to check custom role compatibility.
        Testing = 1,
        /// Permission is not supported for custom role use.
        NotSupported = 2,
    }
}
/// A request to get permissions which can be tested on a resource.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTestablePermissionsRequest {
    /// Required. The full resource name to query from the list of testable
    /// permissions.
    ///
    /// The name follows the Google Cloud Platform resource format.
    /// For example, a Cloud Platform project with id `my-project` will be named
    /// `//cloudresourcemanager.googleapis.com/projects/my-project`.
    #[prost(string, tag = "1")]
    pub full_resource_name: ::prost::alloc::string::String,
    /// Optional limit on the number of permissions to include in the response.
    ///
    /// The default is 100, and the maximum is 1,000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional pagination token returned in an earlier
    /// QueryTestablePermissionsRequest.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// The response containing permissions which can be tested on a resource.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTestablePermissionsResponse {
    /// The Permissions testable on the requested resource.
    #[prost(message, repeated, tag = "1")]
    pub permissions: ::prost::alloc::vec::Vec<Permission>,
    /// To retrieve the next page of results, set
    /// `QueryTestableRolesRequest.page_token` to this value.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// A request to get the list of auditable services for a resource.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAuditableServicesRequest {
    /// Required. The full resource name to query from the list of auditable
    /// services.
    ///
    /// The name follows the Google Cloud Platform resource format.
    /// For example, a Cloud Platform project with id `my-project` will be named
    /// `//cloudresourcemanager.googleapis.com/projects/my-project`.
    #[prost(string, tag = "1")]
    pub full_resource_name: ::prost::alloc::string::String,
}
/// A response containing a list of auditable services for a resource.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAuditableServicesResponse {
    /// The auditable services for a resource.
    #[prost(message, repeated, tag = "1")]
    pub services: ::prost::alloc::vec::Vec<query_auditable_services_response::AuditableService>,
}
/// Nested message and enum types in `QueryAuditableServicesResponse`.
pub mod query_auditable_services_response {
    /// Contains information about an auditable service.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct AuditableService {
        /// Public name of the service.
        /// For example, the service name for Cloud IAM is 'iam.googleapis.com'.
        #[prost(string, tag = "1")]
        pub name: ::prost::alloc::string::String,
    }
}
/// The request to lint a Cloud IAM policy object.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LintPolicyRequest {
    /// The full resource name of the policy this lint request is about.
    ///
    /// The name follows the Google Cloud Platform (GCP) resource format.
    /// For example, a GCP project with ID `my-project` will be named
    /// `//cloudresourcemanager.googleapis.com/projects/my-project`.
    ///
    /// The resource name is not used to read the policy instance from the Cloud
    /// IAM database. The candidate policy for lint has to be provided in the same
    /// request object.
    #[prost(string, tag = "1")]
    pub full_resource_name: ::prost::alloc::string::String,
    /// Required. The Cloud IAM object to be linted.
    #[prost(oneof = "lint_policy_request::LintObject", tags = "5")]
    pub lint_object: ::core::option::Option<lint_policy_request::LintObject>,
}
/// Nested message and enum types in `LintPolicyRequest`.
pub mod lint_policy_request {
    /// Required. The Cloud IAM object to be linted.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum LintObject {
        /// \[google.iam.v1.Binding.condition\] \[google.iam.v1.Binding.condition\] object to be linted.
        #[prost(message, tag = "5")]
        Condition(super::super::super::super::r#type::Expr),
    }
}
/// Structured response of a single validation unit.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LintResult {
    /// The validation unit level.
    #[prost(enumeration = "lint_result::Level", tag = "1")]
    pub level: i32,
    /// The validation unit name, for instance
    /// "lintValidationUnits/ConditionComplexityCheck".
    #[prost(string, tag = "2")]
    pub validation_unit_name: ::prost::alloc::string::String,
    /// The validation unit severity.
    #[prost(enumeration = "lint_result::Severity", tag = "3")]
    pub severity: i32,
    /// The name of the field for which this lint result is about.
    ///
    /// For nested messages `field_name` consists of names of the embedded fields
    /// separated by period character. The top-level qualifier is the input object
    /// to lint in the request. For example, the `field_name` value
    /// `condition.expression` identifies a lint result for the `expression` field
    /// of the provided condition.
    #[prost(string, tag = "5")]
    pub field_name: ::prost::alloc::string::String,
    /// 0-based character position of problematic construct within the object
    /// identified by `field_name`. Currently, this is populated only for condition
    /// expression.
    #[prost(int32, tag = "6")]
    pub location_offset: i32,
    /// Human readable debug message associated with the issue.
    #[prost(string, tag = "7")]
    pub debug_message: ::prost::alloc::string::String,
}
/// Nested message and enum types in `LintResult`.
pub mod lint_result {
    /// Possible Level values of a validation unit corresponding to its domain
    /// of discourse.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Level {
        /// Level is unspecified.
        Unspecified = 0,
        /// A validation unit which operates on an individual condition within a
        /// binding.
        Condition = 3,
    }
    /// Possible Severity values of an issued result.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Severity {
        /// Severity is unspecified.
        Unspecified = 0,
        /// A validation unit returns an error only for critical issues. If an
        /// attempt is made to set the problematic policy without rectifying the
        /// critical issue, it causes the `setPolicy` operation to fail.
        Error = 1,
        /// Any issue which is severe enough but does not cause an error.
        /// For example, suspicious constructs in the input object will not
        /// necessarily fail `setPolicy`, but there is a high likelihood that they
        /// won't behave as expected during policy evaluation in `checkPolicy`.
        /// This includes the following common scenarios:
        ///
        /// - Unsatisfiable condition: Expired timestamp in date/time condition.
        /// - Ineffective condition: Condition on a <member, role> pair which is
        ///   granted unconditionally in another binding of the same policy.
        Warning = 2,
        /// Reserved for the issues that are not severe as `ERROR`/`WARNING`, but
        /// need special handling. For instance, messages about skipped validation
        /// units are issued as `NOTICE`.
        Notice = 3,
        /// Any informative statement which is not severe enough to raise
        /// `ERROR`/`WARNING`/`NOTICE`, like auto-correction recommendations on the
        /// input content. Note that current version of the linter does not utilize
        /// `INFO`.
        Info = 4,
        /// Deprecated severity level.
        Deprecated = 5,
    }
}
/// The response of a lint operation. An empty response indicates
/// the operation was able to fully execute and no lint issue was found.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LintPolicyResponse {
    /// List of lint results sorted by `severity` in descending order.
    #[prost(message, repeated, tag = "1")]
    pub lint_results: ::prost::alloc::vec::Vec<LintResult>,
}
/// Supported key algorithms.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ServiceAccountKeyAlgorithm {
    /// An unspecified key algorithm.
    KeyAlgUnspecified = 0,
    /// 1k RSA Key.
    KeyAlgRsa1024 = 1,
    /// 2k RSA Key.
    KeyAlgRsa2048 = 2,
}
/// Supported private key output formats.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ServiceAccountPrivateKeyType {
    /// Unspecified. Equivalent to `TYPE_GOOGLE_CREDENTIALS_FILE`.
    TypeUnspecified = 0,
    /// PKCS12 format.
    /// The password for the PKCS12 file is `notasecret`.
    /// For more information, see <https://tools.ietf.org/html/rfc7292.>
    TypePkcs12File = 1,
    /// Google Credentials File format.
    TypeGoogleCredentialsFile = 2,
}
/// Supported public key output formats.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ServiceAccountPublicKeyType {
    /// Unspecified. Returns nothing here.
    TypeNone = 0,
    /// X509 PEM format.
    TypeX509PemFile = 1,
    /// Raw public key.
    TypeRawPublicKey = 2,
}
/// Service Account Key Origin.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ServiceAccountKeyOrigin {
    /// Unspecified key origin.
    OriginUnspecified = 0,
    /// Key is provided by user.
    UserProvided = 1,
    /// Key is provided by Google.
    GoogleProvided = 2,
}
/// A view for Role objects.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum RoleView {
    /// Omits the `included_permissions` field.
    /// This is the default value.
    Basic = 0,
    /// Returns all fields.
    Full = 1,
}
#[doc = r" Generated client implementations."]
pub mod iam_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Creates and manages Identity and Access Management (IAM) resources."]
    #[doc = ""]
    #[doc = " You can use this service to work with all of the following resources:"]
    #[doc = ""]
    #[doc = " * **Service accounts**, which identify an application or a virtual machine"]
    #[doc = "   (VM) instance rather than a person"]
    #[doc = " * **Service account keys**, which service accounts use to authenticate with"]
    #[doc = "   Google APIs"]
    #[doc = " * **IAM policies for service accounts**, which specify the roles that a"]
    #[doc = "   member has for the service account"]
    #[doc = " * **IAM custom roles**, which help you limit the number of permissions that"]
    #[doc = "   you grant to members"]
    #[doc = ""]
    #[doc = " In addition, you can use this service to complete the following tasks, among"]
    #[doc = " others:"]
    #[doc = ""]
    #[doc = " * Test whether a service account can use specific permissions"]
    #[doc = " * Check which roles you can grant for a specific resource"]
    #[doc = " * Lint, or validate, condition expressions in an IAM policy"]
    #[derive(Debug, Clone)]
    pub struct IamClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> IamClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> IamClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            IamClient::new(InterceptedService::new(inner, interceptor))
        }
        #[doc = r" Compress requests with `gzip`."]
        #[doc = r""]
        #[doc = r" This requires the server to support it otherwise it might respond with an"]
        #[doc = r" error."]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        #[doc = r" Enable decompressing responses with `gzip`."]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        #[doc = " Lists every [ServiceAccount][google.iam.admin.v1.ServiceAccount] that belongs to a specific project."]
        pub async fn list_service_accounts(
            &mut self,
            request: impl tonic::IntoRequest<super::ListServiceAccountsRequest>,
        ) -> Result<tonic::Response<super::ListServiceAccountsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.iam.admin.v1.IAM/ListServiceAccounts",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets a [ServiceAccount][google.iam.admin.v1.ServiceAccount]."]
        pub async fn get_service_account(
            &mut self,
            request: impl tonic::IntoRequest<super::GetServiceAccountRequest>,
        ) -> Result<tonic::Response<super::ServiceAccount>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/google.iam.admin.v1.IAM/GetServiceAccount");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a [ServiceAccount][google.iam.admin.v1.ServiceAccount]."]
        pub async fn create_service_account(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateServiceAccountRequest>,
        ) -> Result<tonic::Response<super::ServiceAccount>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.iam.admin.v1.IAM/CreateServiceAccount",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " **Note:** We are in the process of deprecating this method. Use"]
        #[doc = " [PatchServiceAccount][google.iam.admin.v1.IAM.PatchServiceAccount] instead."]
        #[doc = ""]
        #[doc = " Updates a [ServiceAccount][google.iam.admin.v1.ServiceAccount]."]
        #[doc = ""]
        #[doc = " You can update only the `display_name` and `description` fields."]
        pub async fn update_service_account(
            &mut self,
            request: impl tonic::IntoRequest<super::ServiceAccount>,
        ) -> Result<tonic::Response<super::ServiceAccount>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.iam.admin.v1.IAM/UpdateServiceAccount",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Patches a [ServiceAccount][google.iam.admin.v1.ServiceAccount]."]
        pub async fn patch_service_account(
            &mut self,
            request: impl tonic::IntoRequest<super::PatchServiceAccountRequest>,
        ) -> Result<tonic::Response<super::ServiceAccount>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.iam.admin.v1.IAM/PatchServiceAccount",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes a [ServiceAccount][google.iam.admin.v1.ServiceAccount]."]
        #[doc = ""]
        #[doc = " **Warning:** After you delete a service account, you might not be able to"]
        #[doc = " undelete it. If you know that you need to re-enable the service account in"]
        #[doc = " the future, use [DisableServiceAccount][google.iam.admin.v1.IAM.DisableServiceAccount] instead."]
        #[doc = ""]
        #[doc = " If you delete a service account, IAM permanently removes the service"]
        #[doc = " account 30 days later. Google Cloud cannot recover the service account"]
        #[doc = " after it is permanently removed, even if you file a support request."]
        #[doc = ""]
        #[doc = " To help avoid unplanned outages, we recommend that you disable the service"]
        #[doc = " account before you delete it. Use [DisableServiceAccount][google.iam.admin.v1.IAM.DisableServiceAccount] to disable the"]
        #[doc = " service account, then wait at least 24 hours and watch for unintended"]
        #[doc = " consequences. If there are no unintended consequences, you can delete the"]
        #[doc = " service account."]
        pub async fn delete_service_account(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteServiceAccountRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.iam.admin.v1.IAM/DeleteServiceAccount",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Restores a deleted [ServiceAccount][google.iam.admin.v1.ServiceAccount]."]
        #[doc = ""]
        #[doc = " **Important:** It is not always possible to restore a deleted service"]
        #[doc = " account. Use this method only as a last resort."]
        #[doc = ""]
        #[doc = " After you delete a service account, IAM permanently removes the service"]
        #[doc = " account 30 days later. There is no way to restore a deleted service account"]
        #[doc = " that has been permanently removed."]
        pub async fn undelete_service_account(
            &mut self,
            request: impl tonic::IntoRequest<super::UndeleteServiceAccountRequest>,
        ) -> Result<tonic::Response<super::UndeleteServiceAccountResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.iam.admin.v1.IAM/UndeleteServiceAccount",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Enables a [ServiceAccount][google.iam.admin.v1.ServiceAccount] that was disabled by"]
        #[doc = " [DisableServiceAccount][google.iam.admin.v1.IAM.DisableServiceAccount]."]
        #[doc = ""]
        #[doc = " If the service account is already enabled, then this method has no effect."]
        #[doc = ""]
        #[doc = " If the service account was disabled by other means—for example, if Google"]
        #[doc = " disabled the service account because it was compromised—you cannot use this"]
        #[doc = " method to enable the service account."]
        pub async fn enable_service_account(
            &mut self,
            request: impl tonic::IntoRequest<super::EnableServiceAccountRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.iam.admin.v1.IAM/EnableServiceAccount",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Disables a [ServiceAccount][google.iam.admin.v1.ServiceAccount] immediately."]
        #[doc = ""]
        #[doc = " If an application uses the service account to authenticate, that"]
        #[doc = " application can no longer call Google APIs or access Google Cloud"]
        #[doc = " resources. Existing access tokens for the service account are rejected, and"]
        #[doc = " requests for new access tokens will fail."]
        #[doc = ""]
        #[doc = " To re-enable the service account, use [EnableServiceAccount][google.iam.admin.v1.IAM.EnableServiceAccount]. After you"]
        #[doc = " re-enable the service account, its existing access tokens will be accepted,"]
        #[doc = " and you can request new access tokens."]
        #[doc = ""]
        #[doc = " To help avoid unplanned outages, we recommend that you disable the service"]
        #[doc = " account before you delete it. Use this method to disable the service"]
        #[doc = " account, then wait at least 24 hours and watch for unintended consequences."]
        #[doc = " If there are no unintended consequences, you can delete the service account"]
        #[doc = " with [DeleteServiceAccount][google.iam.admin.v1.IAM.DeleteServiceAccount]."]
        pub async fn disable_service_account(
            &mut self,
            request: impl tonic::IntoRequest<super::DisableServiceAccountRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.iam.admin.v1.IAM/DisableServiceAccount",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists every [ServiceAccountKey][google.iam.admin.v1.ServiceAccountKey] for a service account."]
        pub async fn list_service_account_keys(
            &mut self,
            request: impl tonic::IntoRequest<super::ListServiceAccountKeysRequest>,
        ) -> Result<tonic::Response<super::ListServiceAccountKeysResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.iam.admin.v1.IAM/ListServiceAccountKeys",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets a [ServiceAccountKey][google.iam.admin.v1.ServiceAccountKey]."]
        pub async fn get_service_account_key(
            &mut self,
            request: impl tonic::IntoRequest<super::GetServiceAccountKeyRequest>,
        ) -> Result<tonic::Response<super::ServiceAccountKey>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.iam.admin.v1.IAM/GetServiceAccountKey",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a [ServiceAccountKey][google.iam.admin.v1.ServiceAccountKey]."]
        pub async fn create_service_account_key(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateServiceAccountKeyRequest>,
        ) -> Result<tonic::Response<super::ServiceAccountKey>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.iam.admin.v1.IAM/CreateServiceAccountKey",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a [ServiceAccountKey][google.iam.admin.v1.ServiceAccountKey], using a public key that you provide."]
        pub async fn upload_service_account_key(
            &mut self,
            request: impl tonic::IntoRequest<super::UploadServiceAccountKeyRequest>,
        ) -> Result<tonic::Response<super::ServiceAccountKey>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.iam.admin.v1.IAM/UploadServiceAccountKey",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes a [ServiceAccountKey][google.iam.admin.v1.ServiceAccountKey]. Deleting a service account key does not"]
        #[doc = " revoke short-lived credentials that have been issued based on the service"]
        #[doc = " account key."]
        pub async fn delete_service_account_key(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteServiceAccountKeyRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.iam.admin.v1.IAM/DeleteServiceAccountKey",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " **Note:** This method is deprecated. Use the"]
        #[doc = " [`signBlob`](https://cloud.google.com/iam/help/rest-credentials/v1/projects.serviceAccounts/signBlob)"]
        #[doc = " method in the IAM Service Account Credentials API instead. If you currently"]
        #[doc = " use this method, see the [migration"]
        #[doc = " guide](https://cloud.google.com/iam/help/credentials/migrate-api) for"]
        #[doc = " instructions."]
        #[doc = ""]
        #[doc = " Signs a blob using the system-managed private key for a [ServiceAccount][google.iam.admin.v1.ServiceAccount]."]
        pub async fn sign_blob(
            &mut self,
            request: impl tonic::IntoRequest<super::SignBlobRequest>,
        ) -> Result<tonic::Response<super::SignBlobResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/google.iam.admin.v1.IAM/SignBlob");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " **Note:** This method is deprecated. Use the"]
        #[doc = " [`signJwt`](https://cloud.google.com/iam/help/rest-credentials/v1/projects.serviceAccounts/signJwt)"]
        #[doc = " method in the IAM Service Account Credentials API instead. If you currently"]
        #[doc = " use this method, see the [migration"]
        #[doc = " guide](https://cloud.google.com/iam/help/credentials/migrate-api) for"]
        #[doc = " instructions."]
        #[doc = ""]
        #[doc = " Signs a JSON Web Token (JWT) using the system-managed private key for a"]
        #[doc = " [ServiceAccount][google.iam.admin.v1.ServiceAccount]."]
        pub async fn sign_jwt(
            &mut self,
            request: impl tonic::IntoRequest<super::SignJwtRequest>,
        ) -> Result<tonic::Response<super::SignJwtResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/google.iam.admin.v1.IAM/SignJwt");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets the IAM policy that is attached to a [ServiceAccount][google.iam.admin.v1.ServiceAccount]. This IAM"]
        #[doc = " policy specifies which members have access to the service account."]
        #[doc = ""]
        #[doc = " This method does not tell you whether the service account has been granted"]
        #[doc = " any roles on other resources. To check whether a service account has role"]
        #[doc = " grants on a resource, use the `getIamPolicy` method for that resource. For"]
        #[doc = " example, to view the role grants for a project, call the Resource Manager"]
        #[doc = " API's"]
        #[doc = " [`projects.getIamPolicy`](https://cloud.google.com/resource-manager/reference/rest/v1/projects/getIamPolicy)"]
        #[doc = " method."]
        pub async fn get_iam_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::super::super::v1::GetIamPolicyRequest>,
        ) -> Result<tonic::Response<super::super::super::v1::Policy>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/google.iam.admin.v1.IAM/GetIamPolicy");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Sets the IAM policy that is attached to a [ServiceAccount][google.iam.admin.v1.ServiceAccount]."]
        #[doc = ""]
        #[doc = " Use this method to grant or revoke access to the service account. For"]
        #[doc = " example, you could grant a member the ability to impersonate the service"]
        #[doc = " account."]
        #[doc = ""]
        #[doc = " This method does not enable the service account to access other resources."]
        #[doc = " To grant roles to a service account on a resource, follow these steps:"]
        #[doc = ""]
        #[doc = " 1. Call the resource's `getIamPolicy` method to get its current IAM policy."]
        #[doc = " 2. Edit the policy so that it binds the service account to an IAM role for"]
        #[doc = " the resource."]
        #[doc = " 3. Call the resource's `setIamPolicy` method to update its IAM policy."]
        #[doc = ""]
        #[doc = " For detailed instructions, see"]
        #[doc = " [Granting roles to a service account for specific"]
        #[doc = " resources](https://cloud.google.com/iam/help/service-accounts/granting-access-to-service-accounts)."]
        pub async fn set_iam_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::super::super::v1::SetIamPolicyRequest>,
        ) -> Result<tonic::Response<super::super::super::v1::Policy>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/google.iam.admin.v1.IAM/SetIamPolicy");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Tests whether the caller has the specified permissions on a"]
        #[doc = " [ServiceAccount][google.iam.admin.v1.ServiceAccount]."]
        pub async fn test_iam_permissions(
            &mut self,
            request: impl tonic::IntoRequest<super::super::super::v1::TestIamPermissionsRequest>,
        ) -> Result<
            tonic::Response<super::super::super::v1::TestIamPermissionsResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/google.iam.admin.v1.IAM/TestIamPermissions");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists roles that can be granted on a Google Cloud resource. A role is"]
        #[doc = " grantable if the IAM policy for the resource can contain bindings to the"]
        #[doc = " role."]
        pub async fn query_grantable_roles(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryGrantableRolesRequest>,
        ) -> Result<tonic::Response<super::QueryGrantableRolesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.iam.admin.v1.IAM/QueryGrantableRoles",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists every predefined [Role][google.iam.admin.v1.Role] that IAM supports, or every custom role"]
        #[doc = " that is defined for an organization or project."]
        pub async fn list_roles(
            &mut self,
            request: impl tonic::IntoRequest<super::ListRolesRequest>,
        ) -> Result<tonic::Response<super::ListRolesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/google.iam.admin.v1.IAM/ListRoles");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets the definition of a [Role][google.iam.admin.v1.Role]."]
        pub async fn get_role(
            &mut self,
            request: impl tonic::IntoRequest<super::GetRoleRequest>,
        ) -> Result<tonic::Response<super::Role>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/google.iam.admin.v1.IAM/GetRole");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a new custom [Role][google.iam.admin.v1.Role]."]
        pub async fn create_role(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateRoleRequest>,
        ) -> Result<tonic::Response<super::Role>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/google.iam.admin.v1.IAM/CreateRole");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates the definition of a custom [Role][google.iam.admin.v1.Role]."]
        pub async fn update_role(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateRoleRequest>,
        ) -> Result<tonic::Response<super::Role>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/google.iam.admin.v1.IAM/UpdateRole");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes a custom [Role][google.iam.admin.v1.Role]."]
        #[doc = ""]
        #[doc = " When you delete a custom role, the following changes occur immediately:"]
        #[doc = ""]
        #[doc = " * You cannot bind a member to the custom role in an IAM"]
        #[doc = " [Policy][google.iam.v1.Policy]."]
        #[doc = " * Existing bindings to the custom role are not changed, but they have no"]
        #[doc = " effect."]
        #[doc = " * By default, the response from [ListRoles][google.iam.admin.v1.IAM.ListRoles] does not include the custom"]
        #[doc = " role."]
        #[doc = ""]
        #[doc = " You have 7 days to undelete the custom role. After 7 days, the following"]
        #[doc = " changes occur:"]
        #[doc = ""]
        #[doc = " * The custom role is permanently deleted and cannot be recovered."]
        #[doc = " * If an IAM policy contains a binding to the custom role, the binding is"]
        #[doc = " permanently removed."]
        pub async fn delete_role(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteRoleRequest>,
        ) -> Result<tonic::Response<super::Role>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/google.iam.admin.v1.IAM/DeleteRole");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Undeletes a custom [Role][google.iam.admin.v1.Role]."]
        pub async fn undelete_role(
            &mut self,
            request: impl tonic::IntoRequest<super::UndeleteRoleRequest>,
        ) -> Result<tonic::Response<super::Role>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/google.iam.admin.v1.IAM/UndeleteRole");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists every permission that you can test on a resource. A permission is"]
        #[doc = " testable if you can check whether a member has that permission on the"]
        #[doc = " resource."]
        pub async fn query_testable_permissions(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryTestablePermissionsRequest>,
        ) -> Result<tonic::Response<super::QueryTestablePermissionsResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.iam.admin.v1.IAM/QueryTestablePermissions",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns a list of services that allow you to opt into audit logs that are"]
        #[doc = " not generated by default."]
        #[doc = ""]
        #[doc = " To learn more about audit logs, see the [Logging"]
        #[doc = " documentation](https://cloud.google.com/logging/docs/audit)."]
        pub async fn query_auditable_services(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryAuditableServicesRequest>,
        ) -> Result<tonic::Response<super::QueryAuditableServicesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.iam.admin.v1.IAM/QueryAuditableServices",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lints, or validates, an IAM policy. Currently checks the"]
        #[doc = " [google.iam.v1.Binding.condition][google.iam.v1.Binding.condition] field, which contains a condition"]
        #[doc = " expression for a role binding."]
        #[doc = ""]
        #[doc = " Successful calls to this method always return an HTTP `200 OK` status code,"]
        #[doc = " even if the linter detects an issue in the IAM policy."]
        pub async fn lint_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::LintPolicyRequest>,
        ) -> Result<tonic::Response<super::LintPolicyResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/google.iam.admin.v1.IAM/LintPolicy");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
