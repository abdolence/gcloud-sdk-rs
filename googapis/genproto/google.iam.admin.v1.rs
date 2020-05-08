/// A service account in the Identity and Access Management API.
///
/// To create a service account, specify the `project_id` and the `account_id`
/// for the account.  The `account_id` is unique within the project, and is used
/// to generate the service account email address and a stable
/// `unique_id`.
///
/// If the account already exists, the account's resource name is returned
/// in the format of projects/{PROJECT_ID}/serviceAccounts/{ACCOUNT}. The caller
/// can use the name in other methods to access the account.
///
/// All other methods can identify the service account using the format
/// `projects/{PROJECT_ID}/serviceAccounts/{ACCOUNT}`.
/// Using `-` as a wildcard for the `PROJECT_ID` will infer the project from
/// the account. The `ACCOUNT` value can be the `email` address or the
/// `unique_id` of the service account.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServiceAccount {
    /// The resource name of the service account in the following format:
    /// `projects/{PROJECT_ID}/serviceAccounts/{ACCOUNT}`.
    ///
    /// Requests using `-` as a wildcard for the `PROJECT_ID` will infer the
    /// project from the `account` and the `ACCOUNT` value can be the `email`
    /// address or the `unique_id` of the service account.
    ///
    /// In responses the resource name will always be in the format
    /// `projects/{PROJECT_ID}/serviceAccounts/{ACCOUNT}`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// @OutputOnly The id of the project that owns the service account.
    #[prost(string, tag = "2")]
    pub project_id: std::string::String,
    /// @OutputOnly The unique and stable id of the service account.
    #[prost(string, tag = "4")]
    pub unique_id: std::string::String,
    /// @OutputOnly The email address of the service account.
    #[prost(string, tag = "5")]
    pub email: std::string::String,
    /// Optional. A user-specified name for the service account.
    /// Must be less than or equal to 100 UTF-8 bytes.
    #[prost(string, tag = "6")]
    pub display_name: std::string::String,
    /// Optional. Note: `etag` is an inoperable legacy field that is only returned
    /// for backwards compatibility.
    #[prost(bytes, tag = "7")]
    pub etag: std::vec::Vec<u8>,
    /// @OutputOnly. The OAuth2 client id for the service account.
    /// This is used in conjunction with the OAuth2 clientconfig API to make
    /// three legged OAuth2 (3LO) flows to access the data of Google users.
    #[prost(string, tag = "9")]
    pub oauth2_client_id: std::string::String,
}
/// The service account create request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateServiceAccountRequest {
    /// Required. The resource name of the project associated with the service
    /// accounts, such as `projects/my-project-123`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Required. The account id that is used to generate the service account
    /// email address and a stable unique id. It is unique within a project,
    /// must be 6-30 characters long, and match the regular expression
    /// `[a-z]([-a-z0-9]*[a-z0-9])` to comply with RFC1035.
    #[prost(string, tag = "2")]
    pub account_id: std::string::String,
    /// The [ServiceAccount][google.iam.admin.v1.ServiceAccount] resource to
    /// create. Currently, only the following values are user assignable:
    /// `display_name` and `description`.
    #[prost(message, optional, tag = "3")]
    pub service_account: ::std::option::Option<ServiceAccount>,
}
/// The service account list request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListServiceAccountsRequest {
    /// Required. The resource name of the project associated with the service
    /// accounts, such as `projects/my-project-123`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Optional limit on the number of service accounts to include in the
    /// response. Further accounts can subsequently be obtained by including the
    /// [ListServiceAccountsResponse.next_page_token][google.iam.admin.v1.ListServiceAccountsResponse.next_page_token]
    /// in a subsequent request.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional pagination token returned in an earlier
    /// [ListServiceAccountsResponse.next_page_token][google.iam.admin.v1.ListServiceAccountsResponse.next_page_token].
    #[prost(string, tag = "3")]
    pub page_token: std::string::String,
}
/// The service account list response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListServiceAccountsResponse {
    /// The list of matching service accounts.
    #[prost(message, repeated, tag = "1")]
    pub accounts: ::std::vec::Vec<ServiceAccount>,
    /// To retrieve the next page of results, set
    /// [ListServiceAccountsRequest.page_token][google.iam.admin.v1.ListServiceAccountsRequest.page_token]
    /// to this value.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
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
    pub name: std::string::String,
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
    pub name: std::string::String,
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
    pub name: std::string::String,
    /// Filters the types of keys the user wants to include in the list
    /// response. Duplicate key types are not allowed. If no key type
    /// is provided, all keys are returned.
    #[prost(
        enumeration = "list_service_account_keys_request::KeyType",
        repeated,
        tag = "2"
    )]
    pub key_types: ::std::vec::Vec<i32>,
}
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
    pub keys: ::std::vec::Vec<ServiceAccountKey>,
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
    pub name: std::string::String,
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
/// lifetime. We recommend caching the public key set for a service account for
/// no more than 24 hours to ensure you have access to the latest keys.
///
/// Public keys for all service accounts are also published at the OAuth2
/// Service Account API.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServiceAccountKey {
    /// The resource name of the service account key in the following format
    /// `projects/{PROJECT_ID}/serviceAccounts/{ACCOUNT}/keys/{key}`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
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
    #[prost(bytes, tag = "3")]
    pub private_key_data: std::vec::Vec<u8>,
    /// The public key data. Only provided in `GetServiceAccountKey` responses.
    #[prost(bytes, tag = "7")]
    pub public_key_data: std::vec::Vec<u8>,
    /// The key can be used after this timestamp.
    #[prost(message, optional, tag = "4")]
    pub valid_after_time: ::std::option::Option<::prost_types::Timestamp>,
    /// The key can be used before this timestamp.
    /// For system-managed key pairs, this timestamp is the end time for the
    /// private key signing operation. The public key could still be used
    /// for verification for a few hours after this time.
    #[prost(message, optional, tag = "5")]
    pub valid_before_time: ::std::option::Option<::prost_types::Timestamp>,
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
    pub name: std::string::String,
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
/// The service account key delete request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteServiceAccountKeyRequest {
    /// Required. The resource name of the service account key in the following format:
    /// `projects/{PROJECT_ID}/serviceAccounts/{ACCOUNT}/keys/{key}`.
    /// Using `-` as a wildcard for the `PROJECT_ID` will infer the project from
    /// the account. The `ACCOUNT` value can be the `email` address or the
    /// `unique_id` of the service account.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// The service account sign blob request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignBlobRequest {
    /// Required. The resource name of the service account in the following format:
    /// `projects/{PROJECT_ID}/serviceAccounts/{ACCOUNT}`.
    /// Using `-` as a wildcard for the `PROJECT_ID` will infer the project from
    /// the account. The `ACCOUNT` value can be the `email` address or the
    /// `unique_id` of the service account.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Required. The bytes to sign.
    #[prost(bytes, tag = "2")]
    pub bytes_to_sign: std::vec::Vec<u8>,
}
/// The service account sign blob response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignBlobResponse {
    /// The id of the key used to sign the blob.
    #[prost(string, tag = "1")]
    pub key_id: std::string::String,
    /// The signed blob.
    #[prost(bytes, tag = "2")]
    pub signature: std::vec::Vec<u8>,
}
/// The service account sign JWT request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignJwtRequest {
    /// Required. The resource name of the service account in the following format:
    /// `projects/{PROJECT_ID}/serviceAccounts/{ACCOUNT}`.
    /// Using `-` as a wildcard for the `PROJECT_ID` will infer the project from
    /// the account. The `ACCOUNT` value can be the `email` address or the
    /// `unique_id` of the service account.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Required. The JWT payload to sign, a JSON JWT Claim set.
    #[prost(string, tag = "2")]
    pub payload: std::string::String,
}
/// The service account sign JWT response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignJwtResponse {
    /// The id of the key used to sign the JWT.
    #[prost(string, tag = "1")]
    pub key_id: std::string::String,
    /// The signed JWT.
    #[prost(string, tag = "2")]
    pub signed_jwt: std::string::String,
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
    pub name: std::string::String,
    /// Optional. A human-readable title for the role.  Typically this
    /// is limited to 100 UTF-8 bytes.
    #[prost(string, tag = "2")]
    pub title: std::string::String,
    /// Optional. A human-readable description for the role.
    #[prost(string, tag = "3")]
    pub description: std::string::String,
    /// The names of the permissions this role grants when bound in an IAM policy.
    #[prost(string, repeated, tag = "7")]
    pub included_permissions: ::std::vec::Vec<std::string::String>,
    /// The current launch stage of the role. If the `ALPHA` launch stage has been
    /// selected for a role, the `stage` field will not be included in the
    /// returned definition for the role.
    #[prost(enumeration = "role::RoleLaunchStage", tag = "8")]
    pub stage: i32,
    /// Used to perform a consistent read-modify-write.
    #[prost(bytes, tag = "9")]
    pub etag: std::vec::Vec<u8>,
    /// The current deleted state of the role. This field is read only.
    /// It will be ignored in calls to CreateRole and UpdateRole.
    #[prost(bool, tag = "11")]
    pub deleted: bool,
}
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
    pub full_resource_name: std::string::String,
    #[prost(enumeration = "RoleView", tag = "2")]
    pub view: i32,
    /// Optional limit on the number of roles to include in the response.
    #[prost(int32, tag = "3")]
    pub page_size: i32,
    /// Optional pagination token returned in an earlier
    /// QueryGrantableRolesResponse.
    #[prost(string, tag = "4")]
    pub page_token: std::string::String,
}
/// The grantable role query response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGrantableRolesResponse {
    /// The list of matching roles.
    #[prost(message, repeated, tag = "1")]
    pub roles: ::std::vec::Vec<Role>,
    /// To retrieve the next page of results, set
    /// `QueryGrantableRolesRequest.page_token` to this value.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
}
/// The request to get all roles defined under a resource.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListRolesRequest {
    /// The `parent` parameter's value depends on the target resource for the
    /// request, namely
    /// [`roles`](/iam/reference/rest/v1/roles),
    /// [`projects`](/iam/reference/rest/v1/projects.roles), or
    /// [`organizations`](/iam/reference/rest/v1/organizations.roles). Each
    /// resource type's `parent` value format is described below:
    ///
    /// * [`roles.list()`](/iam/reference/rest/v1/roles/list): An empty string.
    ///   This method doesn't require a resource; it simply returns all
    ///   [predefined roles](/iam/docs/understanding-roles#predefined_roles) in
    ///   Cloud IAM. Example request URL:
    ///   `https://iam.googleapis.com/v1/roles`
    ///
    /// * [`projects.roles.list()`](/iam/reference/rest/v1/projects.roles/list):
    ///   `projects/{PROJECT_ID}`. This method lists all project-level
    ///   [custom roles](/iam/docs/understanding-custom-roles).
    ///   Example request URL:
    ///   `https://iam.googleapis.com/v1/projects/{PROJECT_ID}/roles`
    ///
    /// * [`organizations.roles.list()`](/iam/reference/rest/v1/organizations.roles/list):
    ///   `organizations/{ORGANIZATION_ID}`. This method lists all
    ///   organization-level [custom roles](/iam/docs/understanding-custom-roles).
    ///   Example request URL:
    ///   `https://iam.googleapis.com/v1/organizations/{ORGANIZATION_ID}/roles`
    ///
    /// Note: Wildcard (*) values are invalid; you must specify a complete project
    /// ID or organization ID.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Optional limit on the number of roles to include in the response.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional pagination token returned in an earlier ListRolesResponse.
    #[prost(string, tag = "3")]
    pub page_token: std::string::String,
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
    pub roles: ::std::vec::Vec<Role>,
    /// To retrieve the next page of results, set
    /// `ListRolesRequest.page_token` to this value.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
}
/// The request to get the definition of an existing role.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRoleRequest {
    /// The `name` parameter's value depends on the target resource for the
    /// request, namely
    /// [`roles`](/iam/reference/rest/v1/roles),
    /// [`projects`](/iam/reference/rest/v1/projects.roles), or
    /// [`organizations`](/iam/reference/rest/v1/organizations.roles). Each
    /// resource type's `name` value format is described below:
    ///
    /// * [`roles.get()`](/iam/reference/rest/v1/roles/get): `roles/{ROLE_NAME}`.
    ///   This method returns results from all
    ///   [predefined roles](/iam/docs/understanding-roles#predefined_roles) in
    ///   Cloud IAM. Example request URL:
    ///   `https://iam.googleapis.com/v1/roles/{ROLE_NAME}`
    ///
    /// * [`projects.roles.get()`](/iam/reference/rest/v1/projects.roles/get):
    ///   `projects/{PROJECT_ID}/roles/{CUSTOM_ROLE_ID}`. This method returns only
    ///   [custom roles](/iam/docs/understanding-custom-roles) that have been
    ///   created at the project level. Example request URL:
    ///   `https://iam.googleapis.com/v1/projects/{PROJECT_ID}/roles/{CUSTOM_ROLE_ID}`
    ///
    /// * [`organizations.roles.get()`](/iam/reference/rest/v1/organizations.roles/get):
    ///   `organizations/{ORGANIZATION_ID}/roles/{CUSTOM_ROLE_ID}`. This method
    ///   returns only [custom roles](/iam/docs/understanding-custom-roles) that
    ///   have been created at the organization level. Example request URL:
    ///   `https://iam.googleapis.com/v1/organizations/{ORGANIZATION_ID}/roles/{CUSTOM_ROLE_ID}`
    ///
    /// Note: Wildcard (*) values are invalid; you must specify a complete project
    /// ID or organization ID.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// The request to create a new role.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateRoleRequest {
    /// The `parent` parameter's value depends on the target resource for the
    /// request, namely
    /// [`projects`](/iam/reference/rest/v1/projects.roles) or
    /// [`organizations`](/iam/reference/rest/v1/organizations.roles). Each
    /// resource type's `parent` value format is described below:
    ///
    /// * [`projects.roles.create()`](/iam/reference/rest/v1/projects.roles/create):
    ///   `projects/{PROJECT_ID}`. This method creates project-level
    ///   [custom roles](/iam/docs/understanding-custom-roles).
    ///   Example request URL:
    ///   `https://iam.googleapis.com/v1/projects/{PROJECT_ID}/roles`
    ///
    /// * [`organizations.roles.create()`](/iam/reference/rest/v1/organizations.roles/create):
    ///   `organizations/{ORGANIZATION_ID}`. This method creates organization-level
    ///   [custom roles](/iam/docs/understanding-custom-roles). Example request
    ///   URL:
    ///   `https://iam.googleapis.com/v1/organizations/{ORGANIZATION_ID}/roles`
    ///
    /// Note: Wildcard (*) values are invalid; you must specify a complete project
    /// ID or organization ID.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// The role ID to use for this role.
    #[prost(string, tag = "2")]
    pub role_id: std::string::String,
    /// The Role resource to create.
    #[prost(message, optional, tag = "3")]
    pub role: ::std::option::Option<Role>,
}
/// The request to update a role.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateRoleRequest {
    /// The `name` parameter's value depends on the target resource for the
    /// request, namely
    /// [`projects`](/iam/reference/rest/v1/projects.roles) or
    /// [`organizations`](/iam/reference/rest/v1/organizations.roles). Each
    /// resource type's `name` value format is described below:
    ///
    /// * [`projects.roles.patch()`](/iam/reference/rest/v1/projects.roles/patch):
    ///   `projects/{PROJECT_ID}/roles/{CUSTOM_ROLE_ID}`. This method updates only
    ///   [custom roles](/iam/docs/understanding-custom-roles) that have been
    ///   created at the project level. Example request URL:
    ///   `https://iam.googleapis.com/v1/projects/{PROJECT_ID}/roles/{CUSTOM_ROLE_ID}`
    ///
    /// * [`organizations.roles.patch()`](/iam/reference/rest/v1/organizations.roles/patch):
    ///   `organizations/{ORGANIZATION_ID}/roles/{CUSTOM_ROLE_ID}`. This method
    ///   updates only [custom roles](/iam/docs/understanding-custom-roles) that
    ///   have been created at the organization level. Example request URL:
    ///   `https://iam.googleapis.com/v1/organizations/{ORGANIZATION_ID}/roles/{CUSTOM_ROLE_ID}`
    ///
    /// Note: Wildcard (*) values are invalid; you must specify a complete project
    /// ID or organization ID.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// The updated role.
    #[prost(message, optional, tag = "2")]
    pub role: ::std::option::Option<Role>,
    /// A mask describing which fields in the Role have changed.
    #[prost(message, optional, tag = "3")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
}
/// The request to delete an existing role.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteRoleRequest {
    /// The `name` parameter's value depends on the target resource for the
    /// request, namely
    /// [`projects`](/iam/reference/rest/v1/projects.roles) or
    /// [`organizations`](/iam/reference/rest/v1/organizations.roles). Each
    /// resource type's `name` value format is described below:
    ///
    /// * [`projects.roles.delete()`](/iam/reference/rest/v1/projects.roles/delete):
    ///   `projects/{PROJECT_ID}/roles/{CUSTOM_ROLE_ID}`. This method deletes only
    ///   [custom roles](/iam/docs/understanding-custom-roles) that have been
    ///   created at the project level. Example request URL:
    ///   `https://iam.googleapis.com/v1/projects/{PROJECT_ID}/roles/{CUSTOM_ROLE_ID}`
    ///
    /// * [`organizations.roles.delete()`](/iam/reference/rest/v1/organizations.roles/delete):
    ///   `organizations/{ORGANIZATION_ID}/roles/{CUSTOM_ROLE_ID}`. This method
    ///   deletes only [custom roles](/iam/docs/understanding-custom-roles) that
    ///   have been created at the organization level. Example request URL:
    ///   `https://iam.googleapis.com/v1/organizations/{ORGANIZATION_ID}/roles/{CUSTOM_ROLE_ID}`
    ///
    /// Note: Wildcard (*) values are invalid; you must specify a complete project
    /// ID or organization ID.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Used to perform a consistent read-modify-write.
    #[prost(bytes, tag = "2")]
    pub etag: std::vec::Vec<u8>,
}
/// The request to undelete an existing role.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UndeleteRoleRequest {
    /// The `name` parameter's value depends on the target resource for the
    /// request, namely
    /// [`projects`](/iam/reference/rest/v1/projects.roles) or
    /// [`organizations`](/iam/reference/rest/v1/organizations.roles). Each
    /// resource type's `name` value format is described below:
    ///
    /// * [`projects.roles.undelete()`](/iam/reference/rest/v1/projects.roles/undelete):
    ///   `projects/{PROJECT_ID}/roles/{CUSTOM_ROLE_ID}`. This method undeletes
    ///   only [custom roles](/iam/docs/understanding-custom-roles) that have been
    ///   created at the project level. Example request URL:
    ///   `https://iam.googleapis.com/v1/projects/{PROJECT_ID}/roles/{CUSTOM_ROLE_ID}`
    ///
    /// * [`organizations.roles.undelete()`](/iam/reference/rest/v1/organizations.roles/undelete):
    ///   `organizations/{ORGANIZATION_ID}/roles/{CUSTOM_ROLE_ID}`. This method
    ///   undeletes only [custom roles](/iam/docs/understanding-custom-roles) that
    ///   have been created at the organization level. Example request URL:
    ///   `https://iam.googleapis.com/v1/organizations/{ORGANIZATION_ID}/roles/{CUSTOM_ROLE_ID}`
    ///
    /// Note: Wildcard (*) values are invalid; you must specify a complete project
    /// ID or organization ID.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Used to perform a consistent read-modify-write.
    #[prost(bytes, tag = "2")]
    pub etag: std::vec::Vec<u8>,
}
/// A permission which can be included by a role.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Permission {
    /// The name of this Permission.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// The title of this Permission.
    #[prost(string, tag = "2")]
    pub title: std::string::String,
    /// A brief description of what this Permission is used for.
    /// This permission can ONLY be used in predefined roles.
    #[prost(string, tag = "3")]
    pub description: std::string::String,
    /// This permission can ONLY be used in predefined roles.
    #[prost(bool, tag = "4")]
    pub only_in_predefined_roles: bool,
    /// The current launch stage of the permission.
    #[prost(enumeration = "permission::PermissionLaunchStage", tag = "5")]
    pub stage: i32,
    /// The current custom role support level.
    #[prost(enumeration = "permission::CustomRolesSupportLevel", tag = "6")]
    pub custom_roles_support_level: i32,
}
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
    pub full_resource_name: std::string::String,
    /// Optional limit on the number of permissions to include in the response.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional pagination token returned in an earlier
    /// QueryTestablePermissionsRequest.
    #[prost(string, tag = "3")]
    pub page_token: std::string::String,
}
/// The response containing permissions which can be tested on a resource.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTestablePermissionsResponse {
    /// The Permissions testable on the requested resource.
    #[prost(message, repeated, tag = "1")]
    pub permissions: ::std::vec::Vec<Permission>,
    /// To retrieve the next page of results, set
    /// `QueryTestableRolesRequest.page_token` to this value.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
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
    /// For more information, see https://tools.ietf.org/html/rfc7292.
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
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Creates and manages service account objects."]
    #[doc = ""]
    #[doc = " Service account is an account that belongs to your project instead"]
    #[doc = " of to an individual end user. It is used to authenticate calls"]
    #[doc = " to a Google API."]
    #[doc = ""]
    #[doc = " To create a service account, specify the `project_id` and `account_id`"]
    #[doc = " for the account.  The `account_id` is unique within the project, and used"]
    #[doc = " to generate the service account email address and a stable"]
    #[doc = " `unique_id`."]
    #[doc = ""]
    #[doc = " All other methods can identify accounts using the format"]
    #[doc = " `projects/{PROJECT_ID}/serviceAccounts/{ACCOUNT}`."]
    #[doc = " Using `-` as a wildcard for the `PROJECT_ID` will infer the project from"]
    #[doc = " the account. The `ACCOUNT` value can be the `email` address or the"]
    #[doc = " `unique_id` of the service account."]
    pub struct IamClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl IamClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> IamClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + HttpBody + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as HttpBody>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = tonic::client::Grpc::with_interceptor(inner, interceptor);
            Self { inner }
        }
        #[doc = " Lists [ServiceAccounts][google.iam.admin.v1.ServiceAccount] for a project."]
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
        #[doc = " Creates a [ServiceAccount][google.iam.admin.v1.ServiceAccount]"]
        #[doc = " and returns it."]
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
        #[doc = " Updates a [ServiceAccount][google.iam.admin.v1.ServiceAccount]."]
        #[doc = ""]
        #[doc = " Currently, only the following fields are updatable:"]
        #[doc = " `display_name` and `description`."]
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
        #[doc = " Deletes a [ServiceAccount][google.iam.admin.v1.ServiceAccount]."]
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
        #[doc = " Lists [ServiceAccountKeys][google.iam.admin.v1.ServiceAccountKey]."]
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
        #[doc = " Gets the [ServiceAccountKey][google.iam.admin.v1.ServiceAccountKey]"]
        #[doc = " by key id."]
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
        #[doc = " Creates a [ServiceAccountKey][google.iam.admin.v1.ServiceAccountKey]"]
        #[doc = " and returns it."]
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
        #[doc = " Deletes a [ServiceAccountKey][google.iam.admin.v1.ServiceAccountKey]."]
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
        #[doc = " Signs a blob using a service account's system-managed private key."]
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
        #[doc = " Signs a JWT using a service account's system-managed private key."]
        #[doc = ""]
        #[doc = " If no expiry time (`exp`) is provided in the `SignJwtRequest`, IAM sets an"]
        #[doc = " an expiry time of one hour by default. If you request an expiry time of"]
        #[doc = " more than one hour, the request will fail."]
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
        #[doc = " Returns the Cloud IAM access control policy for a"]
        #[doc = " [ServiceAccount][google.iam.admin.v1.ServiceAccount]."]
        #[doc = ""]
        #[doc = " Note: Service accounts are both"]
        #[doc = " [resources and"]
        #[doc = " identities](/iam/docs/service-accounts#service_account_permissions). This"]
        #[doc = " method treats the service account as a resource. It returns the Cloud IAM"]
        #[doc = " policy that reflects what members have access to the service account."]
        #[doc = ""]
        #[doc = " This method does not return what resources the service account has access"]
        #[doc = " to. To see if a service account has access to a resource, call the"]
        #[doc = " `getIamPolicy` method on the target resource. For example, to view grants"]
        #[doc = " for a project, call the"]
        #[doc = " [projects.getIamPolicy](/resource-manager/reference/rest/v1/projects/getIamPolicy)"]
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
        #[doc = " Sets the Cloud IAM access control policy for a"]
        #[doc = " [ServiceAccount][google.iam.admin.v1.ServiceAccount]."]
        #[doc = ""]
        #[doc = " Note: Service accounts are both"]
        #[doc = " [resources and"]
        #[doc = " identities](/iam/docs/service-accounts#service_account_permissions). This"]
        #[doc = " method treats the service account as a resource. Use it to grant members"]
        #[doc = " access to the service account, such as when they need to impersonate it."]
        #[doc = ""]
        #[doc = " This method does not grant the service account access to other resources,"]
        #[doc = " such as projects. To grant a service account access to resources, include"]
        #[doc = " the service account in the Cloud IAM policy for the desired resource, then"]
        #[doc = " call the appropriate `setIamPolicy` method on the target resource. For"]
        #[doc = " example, to grant a service account access to a project, call the"]
        #[doc = " [projects.setIamPolicy](/resource-manager/reference/rest/v1/projects/setIamPolicy)"]
        #[doc = " method."]
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
        #[doc = " Tests the specified permissions against the IAM access control policy"]
        #[doc = " for a [ServiceAccount][google.iam.admin.v1.ServiceAccount]."]
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
        #[doc = " Queries roles that can be granted on a particular resource."]
        #[doc = " A role is grantable if it can be used as the role in a binding for a policy"]
        #[doc = " for that resource."]
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
        #[doc = " Lists the Roles defined on a resource."]
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
        #[doc = " Gets a Role definition."]
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
        #[doc = " Creates a new Role."]
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
        #[doc = " Updates a Role definition."]
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
        #[doc = " Soft deletes a role. The role is suspended and cannot be used to create new"]
        #[doc = " IAM Policy Bindings."]
        #[doc = " The Role will not be included in `ListRoles()` unless `show_deleted` is set"]
        #[doc = " in the `ListRolesRequest`. The Role contains the deleted boolean set."]
        #[doc = " Existing Bindings remains, but are inactive. The Role can be undeleted"]
        #[doc = " within 7 days. After 7 days the Role is deleted and all Bindings associated"]
        #[doc = " with the role are removed."]
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
        #[doc = " Undelete a Role, bringing it back in its previous state."]
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
        #[doc = " Lists the permissions testable on a resource."]
        #[doc = " A permission is testable if it can be tested for an identity on a resource."]
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
    }
    impl<T: Clone> Clone for IamClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for IamClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "IamClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod iam_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with IamServer."]
    #[async_trait]
    pub trait Iam: Send + Sync + 'static {
        #[doc = " Lists [ServiceAccounts][google.iam.admin.v1.ServiceAccount] for a project."]
        async fn list_service_accounts(
            &self,
            request: tonic::Request<super::ListServiceAccountsRequest>,
        ) -> Result<tonic::Response<super::ListServiceAccountsResponse>, tonic::Status>;
        #[doc = " Gets a [ServiceAccount][google.iam.admin.v1.ServiceAccount]."]
        async fn get_service_account(
            &self,
            request: tonic::Request<super::GetServiceAccountRequest>,
        ) -> Result<tonic::Response<super::ServiceAccount>, tonic::Status>;
        #[doc = " Creates a [ServiceAccount][google.iam.admin.v1.ServiceAccount]"]
        #[doc = " and returns it."]
        async fn create_service_account(
            &self,
            request: tonic::Request<super::CreateServiceAccountRequest>,
        ) -> Result<tonic::Response<super::ServiceAccount>, tonic::Status>;
        #[doc = " Updates a [ServiceAccount][google.iam.admin.v1.ServiceAccount]."]
        #[doc = ""]
        #[doc = " Currently, only the following fields are updatable:"]
        #[doc = " `display_name` and `description`."]
        async fn update_service_account(
            &self,
            request: tonic::Request<super::ServiceAccount>,
        ) -> Result<tonic::Response<super::ServiceAccount>, tonic::Status>;
        #[doc = " Deletes a [ServiceAccount][google.iam.admin.v1.ServiceAccount]."]
        async fn delete_service_account(
            &self,
            request: tonic::Request<super::DeleteServiceAccountRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status>;
        #[doc = " Lists [ServiceAccountKeys][google.iam.admin.v1.ServiceAccountKey]."]
        async fn list_service_account_keys(
            &self,
            request: tonic::Request<super::ListServiceAccountKeysRequest>,
        ) -> Result<tonic::Response<super::ListServiceAccountKeysResponse>, tonic::Status>;
        #[doc = " Gets the [ServiceAccountKey][google.iam.admin.v1.ServiceAccountKey]"]
        #[doc = " by key id."]
        async fn get_service_account_key(
            &self,
            request: tonic::Request<super::GetServiceAccountKeyRequest>,
        ) -> Result<tonic::Response<super::ServiceAccountKey>, tonic::Status>;
        #[doc = " Creates a [ServiceAccountKey][google.iam.admin.v1.ServiceAccountKey]"]
        #[doc = " and returns it."]
        async fn create_service_account_key(
            &self,
            request: tonic::Request<super::CreateServiceAccountKeyRequest>,
        ) -> Result<tonic::Response<super::ServiceAccountKey>, tonic::Status>;
        #[doc = " Deletes a [ServiceAccountKey][google.iam.admin.v1.ServiceAccountKey]."]
        async fn delete_service_account_key(
            &self,
            request: tonic::Request<super::DeleteServiceAccountKeyRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status>;
        #[doc = " Signs a blob using a service account's system-managed private key."]
        async fn sign_blob(
            &self,
            request: tonic::Request<super::SignBlobRequest>,
        ) -> Result<tonic::Response<super::SignBlobResponse>, tonic::Status>;
        #[doc = " Signs a JWT using a service account's system-managed private key."]
        #[doc = ""]
        #[doc = " If no expiry time (`exp`) is provided in the `SignJwtRequest`, IAM sets an"]
        #[doc = " an expiry time of one hour by default. If you request an expiry time of"]
        #[doc = " more than one hour, the request will fail."]
        async fn sign_jwt(
            &self,
            request: tonic::Request<super::SignJwtRequest>,
        ) -> Result<tonic::Response<super::SignJwtResponse>, tonic::Status>;
        #[doc = " Returns the Cloud IAM access control policy for a"]
        #[doc = " [ServiceAccount][google.iam.admin.v1.ServiceAccount]."]
        #[doc = ""]
        #[doc = " Note: Service accounts are both"]
        #[doc = " [resources and"]
        #[doc = " identities](/iam/docs/service-accounts#service_account_permissions). This"]
        #[doc = " method treats the service account as a resource. It returns the Cloud IAM"]
        #[doc = " policy that reflects what members have access to the service account."]
        #[doc = ""]
        #[doc = " This method does not return what resources the service account has access"]
        #[doc = " to. To see if a service account has access to a resource, call the"]
        #[doc = " `getIamPolicy` method on the target resource. For example, to view grants"]
        #[doc = " for a project, call the"]
        #[doc = " [projects.getIamPolicy](/resource-manager/reference/rest/v1/projects/getIamPolicy)"]
        #[doc = " method."]
        async fn get_iam_policy(
            &self,
            request: tonic::Request<super::super::super::v1::GetIamPolicyRequest>,
        ) -> Result<tonic::Response<super::super::super::v1::Policy>, tonic::Status>;
        #[doc = " Sets the Cloud IAM access control policy for a"]
        #[doc = " [ServiceAccount][google.iam.admin.v1.ServiceAccount]."]
        #[doc = ""]
        #[doc = " Note: Service accounts are both"]
        #[doc = " [resources and"]
        #[doc = " identities](/iam/docs/service-accounts#service_account_permissions). This"]
        #[doc = " method treats the service account as a resource. Use it to grant members"]
        #[doc = " access to the service account, such as when they need to impersonate it."]
        #[doc = ""]
        #[doc = " This method does not grant the service account access to other resources,"]
        #[doc = " such as projects. To grant a service account access to resources, include"]
        #[doc = " the service account in the Cloud IAM policy for the desired resource, then"]
        #[doc = " call the appropriate `setIamPolicy` method on the target resource. For"]
        #[doc = " example, to grant a service account access to a project, call the"]
        #[doc = " [projects.setIamPolicy](/resource-manager/reference/rest/v1/projects/setIamPolicy)"]
        #[doc = " method."]
        async fn set_iam_policy(
            &self,
            request: tonic::Request<super::super::super::v1::SetIamPolicyRequest>,
        ) -> Result<tonic::Response<super::super::super::v1::Policy>, tonic::Status>;
        #[doc = " Tests the specified permissions against the IAM access control policy"]
        #[doc = " for a [ServiceAccount][google.iam.admin.v1.ServiceAccount]."]
        async fn test_iam_permissions(
            &self,
            request: tonic::Request<super::super::super::v1::TestIamPermissionsRequest>,
        ) -> Result<
            tonic::Response<super::super::super::v1::TestIamPermissionsResponse>,
            tonic::Status,
        >;
        #[doc = " Queries roles that can be granted on a particular resource."]
        #[doc = " A role is grantable if it can be used as the role in a binding for a policy"]
        #[doc = " for that resource."]
        async fn query_grantable_roles(
            &self,
            request: tonic::Request<super::QueryGrantableRolesRequest>,
        ) -> Result<tonic::Response<super::QueryGrantableRolesResponse>, tonic::Status>;
        #[doc = " Lists the Roles defined on a resource."]
        async fn list_roles(
            &self,
            request: tonic::Request<super::ListRolesRequest>,
        ) -> Result<tonic::Response<super::ListRolesResponse>, tonic::Status>;
        #[doc = " Gets a Role definition."]
        async fn get_role(
            &self,
            request: tonic::Request<super::GetRoleRequest>,
        ) -> Result<tonic::Response<super::Role>, tonic::Status>;
        #[doc = " Creates a new Role."]
        async fn create_role(
            &self,
            request: tonic::Request<super::CreateRoleRequest>,
        ) -> Result<tonic::Response<super::Role>, tonic::Status>;
        #[doc = " Updates a Role definition."]
        async fn update_role(
            &self,
            request: tonic::Request<super::UpdateRoleRequest>,
        ) -> Result<tonic::Response<super::Role>, tonic::Status>;
        #[doc = " Soft deletes a role. The role is suspended and cannot be used to create new"]
        #[doc = " IAM Policy Bindings."]
        #[doc = " The Role will not be included in `ListRoles()` unless `show_deleted` is set"]
        #[doc = " in the `ListRolesRequest`. The Role contains the deleted boolean set."]
        #[doc = " Existing Bindings remains, but are inactive. The Role can be undeleted"]
        #[doc = " within 7 days. After 7 days the Role is deleted and all Bindings associated"]
        #[doc = " with the role are removed."]
        async fn delete_role(
            &self,
            request: tonic::Request<super::DeleteRoleRequest>,
        ) -> Result<tonic::Response<super::Role>, tonic::Status>;
        #[doc = " Undelete a Role, bringing it back in its previous state."]
        async fn undelete_role(
            &self,
            request: tonic::Request<super::UndeleteRoleRequest>,
        ) -> Result<tonic::Response<super::Role>, tonic::Status>;
        #[doc = " Lists the permissions testable on a resource."]
        #[doc = " A permission is testable if it can be tested for an identity on a resource."]
        async fn query_testable_permissions(
            &self,
            request: tonic::Request<super::QueryTestablePermissionsRequest>,
        ) -> Result<tonic::Response<super::QueryTestablePermissionsResponse>, tonic::Status>;
    }
    #[doc = " Creates and manages service account objects."]
    #[doc = ""]
    #[doc = " Service account is an account that belongs to your project instead"]
    #[doc = " of to an individual end user. It is used to authenticate calls"]
    #[doc = " to a Google API."]
    #[doc = ""]
    #[doc = " To create a service account, specify the `project_id` and `account_id`"]
    #[doc = " for the account.  The `account_id` is unique within the project, and used"]
    #[doc = " to generate the service account email address and a stable"]
    #[doc = " `unique_id`."]
    #[doc = ""]
    #[doc = " All other methods can identify accounts using the format"]
    #[doc = " `projects/{PROJECT_ID}/serviceAccounts/{ACCOUNT}`."]
    #[doc = " Using `-` as a wildcard for the `PROJECT_ID` will infer the project from"]
    #[doc = " the account. The `ACCOUNT` value can be the `email` address or the"]
    #[doc = " `unique_id` of the service account."]
    #[derive(Debug)]
    #[doc(hidden)]
    pub struct IamServer<T: Iam> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: Iam> IamServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, None);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, Some(interceptor.into()));
            Self { inner }
        }
    }
    impl<T, B> Service<http::Request<B>> for IamServer<T>
    where
        T: Iam,
        B: HttpBody + Send + Sync + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/google.iam.admin.v1.IAM/ListServiceAccounts" => {
                    #[allow(non_camel_case_types)]
                    struct ListServiceAccountsSvc<T: Iam>(pub Arc<T>);
                    impl<T: Iam> tonic::server::UnaryService<super::ListServiceAccountsRequest>
                        for ListServiceAccountsSvc<T>
                    {
                        type Response = super::ListServiceAccountsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListServiceAccountsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.list_service_accounts(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ListServiceAccountsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/google.iam.admin.v1.IAM/GetServiceAccount" => {
                    #[allow(non_camel_case_types)]
                    struct GetServiceAccountSvc<T: Iam>(pub Arc<T>);
                    impl<T: Iam> tonic::server::UnaryService<super::GetServiceAccountRequest>
                        for GetServiceAccountSvc<T>
                    {
                        type Response = super::ServiceAccount;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetServiceAccountRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.get_service_account(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetServiceAccountSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/google.iam.admin.v1.IAM/CreateServiceAccount" => {
                    #[allow(non_camel_case_types)]
                    struct CreateServiceAccountSvc<T: Iam>(pub Arc<T>);
                    impl<T: Iam> tonic::server::UnaryService<super::CreateServiceAccountRequest>
                        for CreateServiceAccountSvc<T>
                    {
                        type Response = super::ServiceAccount;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateServiceAccountRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.create_service_account(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = CreateServiceAccountSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/google.iam.admin.v1.IAM/UpdateServiceAccount" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateServiceAccountSvc<T: Iam>(pub Arc<T>);
                    impl<T: Iam> tonic::server::UnaryService<super::ServiceAccount> for UpdateServiceAccountSvc<T> {
                        type Response = super::ServiceAccount;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ServiceAccount>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.update_service_account(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = UpdateServiceAccountSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/google.iam.admin.v1.IAM/DeleteServiceAccount" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteServiceAccountSvc<T: Iam>(pub Arc<T>);
                    impl<T: Iam> tonic::server::UnaryService<super::DeleteServiceAccountRequest>
                        for DeleteServiceAccountSvc<T>
                    {
                        type Response = ();
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteServiceAccountRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.delete_service_account(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = DeleteServiceAccountSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/google.iam.admin.v1.IAM/ListServiceAccountKeys" => {
                    #[allow(non_camel_case_types)]
                    struct ListServiceAccountKeysSvc<T: Iam>(pub Arc<T>);
                    impl<T: Iam> tonic::server::UnaryService<super::ListServiceAccountKeysRequest>
                        for ListServiceAccountKeysSvc<T>
                    {
                        type Response = super::ListServiceAccountKeysResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListServiceAccountKeysRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.list_service_account_keys(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ListServiceAccountKeysSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/google.iam.admin.v1.IAM/GetServiceAccountKey" => {
                    #[allow(non_camel_case_types)]
                    struct GetServiceAccountKeySvc<T: Iam>(pub Arc<T>);
                    impl<T: Iam> tonic::server::UnaryService<super::GetServiceAccountKeyRequest>
                        for GetServiceAccountKeySvc<T>
                    {
                        type Response = super::ServiceAccountKey;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetServiceAccountKeyRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.get_service_account_key(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetServiceAccountKeySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/google.iam.admin.v1.IAM/CreateServiceAccountKey" => {
                    #[allow(non_camel_case_types)]
                    struct CreateServiceAccountKeySvc<T: Iam>(pub Arc<T>);
                    impl<T: Iam> tonic::server::UnaryService<super::CreateServiceAccountKeyRequest>
                        for CreateServiceAccountKeySvc<T>
                    {
                        type Response = super::ServiceAccountKey;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateServiceAccountKeyRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { inner.create_service_account_key(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = CreateServiceAccountKeySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/google.iam.admin.v1.IAM/DeleteServiceAccountKey" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteServiceAccountKeySvc<T: Iam>(pub Arc<T>);
                    impl<T: Iam> tonic::server::UnaryService<super::DeleteServiceAccountKeyRequest>
                        for DeleteServiceAccountKeySvc<T>
                    {
                        type Response = ();
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteServiceAccountKeyRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { inner.delete_service_account_key(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = DeleteServiceAccountKeySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/google.iam.admin.v1.IAM/SignBlob" => {
                    #[allow(non_camel_case_types)]
                    struct SignBlobSvc<T: Iam>(pub Arc<T>);
                    impl<T: Iam> tonic::server::UnaryService<super::SignBlobRequest> for SignBlobSvc<T> {
                        type Response = super::SignBlobResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SignBlobRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.sign_blob(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = SignBlobSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/google.iam.admin.v1.IAM/SignJwt" => {
                    #[allow(non_camel_case_types)]
                    struct SignJwtSvc<T: Iam>(pub Arc<T>);
                    impl<T: Iam> tonic::server::UnaryService<super::SignJwtRequest> for SignJwtSvc<T> {
                        type Response = super::SignJwtResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SignJwtRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.sign_jwt(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = SignJwtSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/google.iam.admin.v1.IAM/GetIamPolicy" => {
                    #[allow(non_camel_case_types)]
                    struct GetIamPolicySvc<T: Iam>(pub Arc<T>);
                    impl<T: Iam>
                        tonic::server::UnaryService<super::super::super::v1::GetIamPolicyRequest>
                        for GetIamPolicySvc<T>
                    {
                        type Response = super::super::super::v1::Policy;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::super::v1::GetIamPolicyRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.get_iam_policy(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetIamPolicySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/google.iam.admin.v1.IAM/SetIamPolicy" => {
                    #[allow(non_camel_case_types)]
                    struct SetIamPolicySvc<T: Iam>(pub Arc<T>);
                    impl<T: Iam>
                        tonic::server::UnaryService<super::super::super::v1::SetIamPolicyRequest>
                        for SetIamPolicySvc<T>
                    {
                        type Response = super::super::super::v1::Policy;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::super::v1::SetIamPolicyRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.set_iam_policy(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = SetIamPolicySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/google.iam.admin.v1.IAM/TestIamPermissions" => {
                    #[allow(non_camel_case_types)]
                    struct TestIamPermissionsSvc<T: Iam>(pub Arc<T>);
                    impl<T: Iam>
                        tonic::server::UnaryService<
                            super::super::super::v1::TestIamPermissionsRequest,
                        > for TestIamPermissionsSvc<T>
                    {
                        type Response = super::super::super::v1::TestIamPermissionsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::super::v1::TestIamPermissionsRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.test_iam_permissions(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = TestIamPermissionsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/google.iam.admin.v1.IAM/QueryGrantableRoles" => {
                    #[allow(non_camel_case_types)]
                    struct QueryGrantableRolesSvc<T: Iam>(pub Arc<T>);
                    impl<T: Iam> tonic::server::UnaryService<super::QueryGrantableRolesRequest>
                        for QueryGrantableRolesSvc<T>
                    {
                        type Response = super::QueryGrantableRolesResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryGrantableRolesRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.query_grantable_roles(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = QueryGrantableRolesSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/google.iam.admin.v1.IAM/ListRoles" => {
                    #[allow(non_camel_case_types)]
                    struct ListRolesSvc<T: Iam>(pub Arc<T>);
                    impl<T: Iam> tonic::server::UnaryService<super::ListRolesRequest> for ListRolesSvc<T> {
                        type Response = super::ListRolesResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListRolesRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.list_roles(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ListRolesSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/google.iam.admin.v1.IAM/GetRole" => {
                    #[allow(non_camel_case_types)]
                    struct GetRoleSvc<T: Iam>(pub Arc<T>);
                    impl<T: Iam> tonic::server::UnaryService<super::GetRoleRequest> for GetRoleSvc<T> {
                        type Response = super::Role;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetRoleRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.get_role(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetRoleSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/google.iam.admin.v1.IAM/CreateRole" => {
                    #[allow(non_camel_case_types)]
                    struct CreateRoleSvc<T: Iam>(pub Arc<T>);
                    impl<T: Iam> tonic::server::UnaryService<super::CreateRoleRequest> for CreateRoleSvc<T> {
                        type Response = super::Role;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateRoleRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.create_role(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = CreateRoleSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/google.iam.admin.v1.IAM/UpdateRole" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateRoleSvc<T: Iam>(pub Arc<T>);
                    impl<T: Iam> tonic::server::UnaryService<super::UpdateRoleRequest> for UpdateRoleSvc<T> {
                        type Response = super::Role;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateRoleRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.update_role(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = UpdateRoleSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/google.iam.admin.v1.IAM/DeleteRole" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteRoleSvc<T: Iam>(pub Arc<T>);
                    impl<T: Iam> tonic::server::UnaryService<super::DeleteRoleRequest> for DeleteRoleSvc<T> {
                        type Response = super::Role;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteRoleRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.delete_role(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = DeleteRoleSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/google.iam.admin.v1.IAM/UndeleteRole" => {
                    #[allow(non_camel_case_types)]
                    struct UndeleteRoleSvc<T: Iam>(pub Arc<T>);
                    impl<T: Iam> tonic::server::UnaryService<super::UndeleteRoleRequest> for UndeleteRoleSvc<T> {
                        type Response = super::Role;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UndeleteRoleRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.undelete_role(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = UndeleteRoleSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/google.iam.admin.v1.IAM/QueryTestablePermissions" => {
                    #[allow(non_camel_case_types)]
                    struct QueryTestablePermissionsSvc<T: Iam>(pub Arc<T>);
                    impl<T: Iam> tonic::server::UnaryService<super::QueryTestablePermissionsRequest>
                        for QueryTestablePermissionsSvc<T>
                    {
                        type Response = super::QueryTestablePermissionsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryTestablePermissionsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { inner.query_testable_permissions(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = QueryTestablePermissionsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .body(tonic::body::BoxBody::empty())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: Iam> Clone for IamServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: Iam> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Iam> tonic::transport::NamedService for IamServer<T> {
        const NAME: &'static str = "google.iam.admin.v1.IAM";
    }
}
