/// Common audit log format for Google Cloud Platform API operations.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuditLog {
    /// The name of the API service performing the operation. For example,
    /// `"datastore.googleapis.com"`.
    #[prost(string, tag = "7")]
    pub service_name: ::prost::alloc::string::String,
    /// The name of the service method or operation.
    /// For API calls, this should be the name of the API method.
    /// For example,
    ///
    ///     "google.datastore.v1.Datastore.RunQuery"
    ///     "google.logging.v1.LoggingService.DeleteLog"
    #[prost(string, tag = "8")]
    pub method_name: ::prost::alloc::string::String,
    /// The resource or collection that is the target of the operation.
    /// The name is a scheme-less URI, not including the API service name.
    /// For example:
    ///
    ///     "shelves/SHELF_ID/books"
    ///     "shelves/SHELF_ID/books/BOOK_ID"
    #[prost(string, tag = "11")]
    pub resource_name: ::prost::alloc::string::String,
    /// The resource location information.
    #[prost(message, optional, tag = "20")]
    pub resource_location: ::core::option::Option<ResourceLocation>,
    /// The resource's original state before mutation. Present only for
    /// operations which have successfully modified the targeted resource(s).
    /// In general, this field should contain all changed fields, except those
    /// that are already been included in `request`, `response`, `metadata` or
    /// `service_data` fields.
    /// When the JSON object represented here has a proto equivalent,
    /// the proto name will be indicated in the `@type` property.
    #[prost(message, optional, tag = "19")]
    pub resource_original_state: ::core::option::Option<::prost_types::Struct>,
    /// The number of items returned from a List or Query API method,
    /// if applicable.
    #[prost(int64, tag = "12")]
    pub num_response_items: i64,
    /// The status of the overall operation.
    #[prost(message, optional, tag = "2")]
    pub status: ::core::option::Option<super::super::rpc::Status>,
    /// Authentication information.
    #[prost(message, optional, tag = "3")]
    pub authentication_info: ::core::option::Option<AuthenticationInfo>,
    /// Authorization information. If there are multiple
    /// resources or permissions involved, then there is
    /// one AuthorizationInfo element for each {resource, permission} tuple.
    #[prost(message, repeated, tag = "9")]
    pub authorization_info: ::prost::alloc::vec::Vec<AuthorizationInfo>,
    /// Metadata about the operation.
    #[prost(message, optional, tag = "4")]
    pub request_metadata: ::core::option::Option<RequestMetadata>,
    /// The operation request. This may not include all request parameters,
    /// such as those that are too large, privacy-sensitive, or duplicated
    /// elsewhere in the log record.
    /// It should never include user-generated data, such as file contents.
    /// When the JSON object represented here has a proto equivalent, the proto
    /// name will be indicated in the `@type` property.
    #[prost(message, optional, tag = "16")]
    pub request: ::core::option::Option<::prost_types::Struct>,
    /// The operation response. This may not include all response elements,
    /// such as those that are too large, privacy-sensitive, or duplicated
    /// elsewhere in the log record.
    /// It should never include user-generated data, such as file contents.
    /// When the JSON object represented here has a proto equivalent, the proto
    /// name will be indicated in the `@type` property.
    #[prost(message, optional, tag = "17")]
    pub response: ::core::option::Option<::prost_types::Struct>,
    /// Other service-specific data about the request, response, and other
    /// information associated with the current audited event.
    #[prost(message, optional, tag = "18")]
    pub metadata: ::core::option::Option<::prost_types::Struct>,
    /// Deprecated, use `metadata` field instead.
    /// Other service-specific data about the request, response, and other
    /// activities.
    #[prost(message, optional, tag = "15")]
    pub service_data: ::core::option::Option<::prost_types::Any>,
}
/// Authentication information for the operation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthenticationInfo {
    /// The email address of the authenticated user (or service account on behalf
    /// of third party principal) making the request. For privacy reasons, the
    /// principal email address is redacted for all read-only operations that fail
    /// with a "permission denied" error.
    #[prost(string, tag = "1")]
    pub principal_email: ::prost::alloc::string::String,
    /// The authority selector specified by the requestor, if any.
    /// It is not guaranteed that the principal was allowed to use this authority.
    #[prost(string, tag = "2")]
    pub authority_selector: ::prost::alloc::string::String,
    /// The third party identification (if any) of the authenticated user making
    /// the request.
    /// When the JSON object represented here has a proto equivalent, the proto
    /// name will be indicated in the `@type` property.
    #[prost(message, optional, tag = "4")]
    pub third_party_principal: ::core::option::Option<::prost_types::Struct>,
    /// The name of the service account key used to create or exchange
    /// credentials for authenticating the service account making the request.
    /// This is a scheme-less URI full resource name. For example:
    ///
    /// "//iam.googleapis.com/projects/{PROJECT_ID}/serviceAccounts/{ACCOUNT}/keys/{key}"
    #[prost(string, tag = "5")]
    pub service_account_key_name: ::prost::alloc::string::String,
    /// Identity delegation history of an authenticated service account that makes
    /// the request. It contains information on the real authorities that try to
    /// access GCP resources by delegating on a service account. When multiple
    /// authorities present, they are guaranteed to be sorted based on the original
    /// ordering of the identity delegation events.
    #[prost(message, repeated, tag = "6")]
    pub service_account_delegation_info: ::prost::alloc::vec::Vec<ServiceAccountDelegationInfo>,
    /// String representation of identity of requesting party.
    /// Populated for both first and third party identities.
    #[prost(string, tag = "8")]
    pub principal_subject: ::prost::alloc::string::String,
}
/// Authorization information for the operation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthorizationInfo {
    /// The resource being accessed, as a REST-style string. For example:
    ///
    ///     bigquery.googleapis.com/projects/PROJECTID/datasets/DATASETID
    #[prost(string, tag = "1")]
    pub resource: ::prost::alloc::string::String,
    /// The required IAM permission.
    #[prost(string, tag = "2")]
    pub permission: ::prost::alloc::string::String,
    /// Whether or not authorization for `resource` and `permission`
    /// was granted.
    #[prost(bool, tag = "3")]
    pub granted: bool,
    /// Resource attributes used in IAM condition evaluation. This field contains
    /// resource attributes like resource type and resource name.
    ///
    /// To get the whole view of the attributes used in IAM
    /// condition evaluation, the user must also look into
    /// `AuditLog.request_metadata.request_attributes`.
    #[prost(message, optional, tag = "5")]
    pub resource_attributes:
        ::core::option::Option<super::super::rpc::context::attribute_context::Resource>,
}
/// Metadata about the request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestMetadata {
    /// The IP address of the caller.
    /// For caller from internet, this will be public IPv4 or IPv6 address.
    /// For caller from a Compute Engine VM with external IP address, this
    /// will be the VM's external IP address. For caller from a Compute
    /// Engine VM without external IP address, if the VM is in the same
    /// organization (or project) as the accessed resource, `caller_ip` will
    /// be the VM's internal IPv4 address, otherwise the `caller_ip` will be
    /// redacted to "gce-internal-ip".
    /// See https://cloud.google.com/compute/docs/vpc/ for more information.
    #[prost(string, tag = "1")]
    pub caller_ip: ::prost::alloc::string::String,
    /// The user agent of the caller.
    /// This information is not authenticated and should be treated accordingly.
    /// For example:
    ///
    /// +   `google-api-python-client/1.4.0`:
    ///     The request was made by the Google API client for Python.
    /// +   `Cloud SDK Command Line Tool apitools-client/1.0 gcloud/0.9.62`:
    ///     The request was made by the Google Cloud SDK CLI (gcloud).
    /// +   `AppEngine-Google; (+http://code.google.com/appengine; appid:
    /// s~my-project`:
    ///     The request was made from the `my-project` App Engine app.
    #[prost(string, tag = "2")]
    pub caller_supplied_user_agent: ::prost::alloc::string::String,
    /// The network of the caller.
    /// Set only if the network host project is part of the same GCP organization
    /// (or project) as the accessed resource.
    /// See https://cloud.google.com/compute/docs/vpc/ for more information.
    /// This is a scheme-less URI full resource name. For example:
    ///
    ///     "//compute.googleapis.com/projects/PROJECT_ID/global/networks/NETWORK_ID"
    #[prost(string, tag = "3")]
    pub caller_network: ::prost::alloc::string::String,
    /// Request attributes used in IAM condition evaluation. This field contains
    /// request attributes like request time and access levels associated with
    /// the request.
    ///
    ///
    /// To get the whole view of the attributes used in IAM
    /// condition evaluation, the user must also look into
    /// `AuditLog.authentication_info.resource_attributes`.
    #[prost(message, optional, tag = "7")]
    pub request_attributes:
        ::core::option::Option<super::super::rpc::context::attribute_context::Request>,
    /// The destination of a network activity, such as accepting a TCP connection.
    /// In a multi hop network activity, the destination represents the receiver of
    /// the last hop. Only two fields are used in this message, Peer.port and
    /// Peer.ip. These fields are optionally populated by those services utilizing
    /// the IAM condition feature.
    #[prost(message, optional, tag = "8")]
    pub destination_attributes:
        ::core::option::Option<super::super::rpc::context::attribute_context::Peer>,
}
/// Location information about a resource.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResourceLocation {
    /// The locations of a resource after the execution of the operation.
    /// Requests to create or delete a location based resource must populate
    /// the 'current_locations' field and not the 'original_locations' field.
    /// For example:
    ///
    ///     "europe-west1-a"
    ///     "us-east1"
    ///     "nam3"
    #[prost(string, repeated, tag = "1")]
    pub current_locations: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The locations of a resource prior to the execution of the operation.
    /// Requests that mutate the resource's location must populate both the
    /// 'original_locations' as well as the 'current_locations' fields.
    /// For example:
    ///
    ///     "europe-west1-a"
    ///     "us-east1"
    ///     "nam3"
    #[prost(string, repeated, tag = "2")]
    pub original_locations: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Identity delegation history of an authenticated service account.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServiceAccountDelegationInfo {
    /// Entity that creates credentials for service account and assumes its
    /// identity for authentication.
    #[prost(oneof = "service_account_delegation_info::Authority", tags = "1, 2")]
    pub authority: ::core::option::Option<service_account_delegation_info::Authority>,
}
/// Nested message and enum types in `ServiceAccountDelegationInfo`.
pub mod service_account_delegation_info {
    /// First party identity principal.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct FirstPartyPrincipal {
        /// The email address of a Google account.
        #[prost(string, tag = "1")]
        pub principal_email: ::prost::alloc::string::String,
        /// Metadata about the service that uses the service account.
        #[prost(message, optional, tag = "2")]
        pub service_metadata: ::core::option::Option<::prost_types::Struct>,
    }
    /// Third party identity principal.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ThirdPartyPrincipal {
        /// Metadata about third party identity.
        #[prost(message, optional, tag = "1")]
        pub third_party_claims: ::core::option::Option<::prost_types::Struct>,
    }
    /// Entity that creates credentials for service account and assumes its
    /// identity for authentication.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Authority {
        /// First party (Google) identity as the real authority.
        #[prost(message, tag = "1")]
        FirstPartyPrincipal(FirstPartyPrincipal),
        /// Third party identity as the real authority.
        #[prost(message, tag = "2")]
        ThirdPartyPrincipal(ThirdPartyPrincipal),
    }
}
