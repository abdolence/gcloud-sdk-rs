/// Feature represents the settings and status of any Hub Feature.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Feature {
    /// Output only. The full, unique name of this Feature resource in the format
    /// `projects/*/locations/*/features/*`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// GCP labels for this Feature.
    #[prost(map = "string, string", tag = "2")]
    pub labels:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Output only. State of the Feature resource itself.
    #[prost(message, optional, tag = "3")]
    pub resource_state: ::core::option::Option<FeatureResourceState>,
    /// Optional. Hub-wide Feature configuration. If this Feature does not support any
    /// Hub-wide configuration, this field may be unused.
    #[prost(message, optional, tag = "4")]
    pub spec: ::core::option::Option<CommonFeatureSpec>,
    /// Optional. Membership-specific configuration for this Feature. If this Feature does
    /// not support any per-Membership configuration, this field may be unused.
    ///
    /// The keys indicate which Membership the configuration is for, in the form:
    ///
    ///     projects/{p}/locations/{l}/memberships/{m}
    ///
    /// Where {p} is the project, {l} is a valid location and {m} is a valid
    /// Membership in this project at that location. {p} WILL match the Feature's
    /// project.
    ///
    /// {p} will always be returned as the project number, but the project ID is
    /// also accepted during input. If the same Membership is specified in the map
    /// twice (using the project ID form, and the project number form), exactly
    /// ONE of the entries will be saved, with no guarantees as to which. For this
    /// reason, it is recommended the same format be used for all entries when
    /// mutating a Feature.
    #[prost(map = "string, message", tag = "5")]
    pub membership_specs:
        ::std::collections::HashMap<::prost::alloc::string::String, MembershipFeatureSpec>,
    /// Output only. The Hub-wide Feature state.
    #[prost(message, optional, tag = "6")]
    pub state: ::core::option::Option<CommonFeatureState>,
    /// Output only. Membership-specific Feature status. If this Feature does
    /// report any per-Membership status, this field may be unused.
    ///
    /// The keys indicate which Membership the state is for, in the form:
    ///
    ///     projects/{p}/locations/{l}/memberships/{m}
    ///
    /// Where {p} is the project number, {l} is a valid location and {m} is a valid
    /// Membership in this project at that location. {p} MUST match the Feature's
    /// project number.
    #[prost(map = "string, message", tag = "7")]
    pub membership_states:
        ::std::collections::HashMap<::prost::alloc::string::String, MembershipFeatureState>,
    /// Output only. When the Feature resource was created.
    #[prost(message, optional, tag = "8")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. When the Feature resource was last updated.
    #[prost(message, optional, tag = "9")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. When the Feature resource was deleted.
    #[prost(message, optional, tag = "10")]
    pub delete_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// FeatureResourceState describes the state of a Feature *resource* in the
/// GkeHub API. See `FeatureState` for the "running state" of the Feature in the
/// Hub and across Memberships.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeatureResourceState {
    /// The current state of the Feature resource in the Hub API.
    #[prost(enumeration = "feature_resource_state::State", tag = "1")]
    pub state: i32,
}
/// Nested message and enum types in `FeatureResourceState`.
pub mod feature_resource_state {
    /// State describes the lifecycle status of a Feature.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// State is unknown or not set.
        Unspecified = 0,
        /// The Feature is being enabled, and the Feature resource is being created.
        /// Once complete, the corresponding Feature will be enabled in this Hub.
        Enabling = 1,
        /// The Feature is enabled in this Hub, and the Feature resource is fully
        /// available.
        Active = 2,
        /// The Feature is being disabled in this Hub, and the Feature resource
        /// is being deleted.
        Disabling = 3,
        /// The Feature resource is being updated.
        Updating = 4,
        /// The Feature resource is being updated by the Hub Service.
        ServiceUpdating = 5,
    }
}
/// FeatureState describes the high-level state of a Feature. It may be used to
/// describe a Feature's state at the environ-level, or per-membershop, depending
/// on the context.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeatureState {
    /// The high-level, machine-readable status of this Feature.
    #[prost(enumeration = "feature_state::Code", tag = "1")]
    pub code: i32,
    /// A human-readable description of the current status.
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// The time this status and any related Feature-specific details were updated.
    #[prost(message, optional, tag = "3")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Nested message and enum types in `FeatureState`.
pub mod feature_state {
    /// Code represents a machine-readable, high-level status of the Feature.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Code {
        /// Unknown or not set.
        Unspecified = 0,
        /// The Feature is operating normally.
        Ok = 1,
        /// The Feature has encountered an issue, and is operating in a degraded
        /// state. The Feature may need intervention to return to normal operation.
        /// See the description and any associated Feature-specific details for more
        /// information.
        Warning = 2,
        /// The Feature is not operating or is in a severely degraded state.
        /// The Feature may need intervention to return to normal operation.
        /// See the description and any associated Feature-specific details for more
        /// information.
        Error = 3,
    }
}
/// CommonFeatureSpec contains Hub-wide configuration information
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommonFeatureSpec {
    #[prost(oneof = "common_feature_spec::FeatureSpec", tags = "102")]
    pub feature_spec: ::core::option::Option<common_feature_spec::FeatureSpec>,
}
/// Nested message and enum types in `CommonFeatureSpec`.
pub mod common_feature_spec {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum FeatureSpec {
        /// Multicluster Ingress-specific spec.
        #[prost(message, tag = "102")]
        Multiclusteringress(super::super::multiclusteringress::v1::FeatureSpec),
    }
}
/// CommonFeatureState contains Hub-wide Feature status information.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommonFeatureState {
    /// Output only. The "running state" of the Feature in this Hub.
    #[prost(message, optional, tag = "1")]
    pub state: ::core::option::Option<FeatureState>,
}
/// MembershipFeatureSpec contains configuration information for a single
/// Membership.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MembershipFeatureSpec {
    #[prost(oneof = "membership_feature_spec::FeatureSpec", tags = "106")]
    pub feature_spec: ::core::option::Option<membership_feature_spec::FeatureSpec>,
}
/// Nested message and enum types in `MembershipFeatureSpec`.
pub mod membership_feature_spec {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum FeatureSpec {
        /// Config Management-specific spec.
        #[prost(message, tag = "106")]
        Configmanagement(super::super::configmanagement::v1::MembershipSpec),
    }
}
/// MembershipFeatureState contains Feature status information for a single
/// Membership.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MembershipFeatureState {
    /// The high-level state of this Feature for a single membership.
    #[prost(message, optional, tag = "1")]
    pub state: ::core::option::Option<FeatureState>,
    #[prost(oneof = "membership_feature_state::FeatureState", tags = "106")]
    pub feature_state: ::core::option::Option<membership_feature_state::FeatureState>,
}
/// Nested message and enum types in `MembershipFeatureState`.
pub mod membership_feature_state {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum FeatureState {
        /// Config Management-specific state.
        #[prost(message, tag = "106")]
        Configmanagement(super::super::configmanagement::v1::MembershipState),
    }
}
/// Membership contains information about a member cluster.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Membership {
    /// Output only. The full, unique name of this Membership resource in the format
    /// `projects/*/locations/*/memberships/{membership_id}`, set during creation.
    ///
    /// `membership_id` must be a valid RFC 1123 compliant DNS label:
    ///
    ///   1. At most 63 characters in length
    ///   2. It must consist of lower case alphanumeric characters or `-`
    ///   3. It must start and end with an alphanumeric character
    ///
    /// Which can be expressed as the regex: `\[a-z0-9]([-a-z0-9]*[a-z0-9\])?`,
    /// with a maximum length of 63 characters.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. GCP labels for this membership.
    #[prost(map = "string, string", tag = "2")]
    pub labels:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Output only. Description of this membership, limited to 63 characters.
    /// Must match the regex: `\[a-zA-Z0-9\][a-zA-Z0-9_\-\.\ ]*`
    ///
    /// This field is present for legacy purposes.
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    /// Output only. State of the Membership resource.
    #[prost(message, optional, tag = "5")]
    pub state: ::core::option::Option<MembershipState>,
    /// Output only. When the Membership was created.
    #[prost(message, optional, tag = "6")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. When the Membership was last updated.
    #[prost(message, optional, tag = "7")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. When the Membership was deleted.
    #[prost(message, optional, tag = "8")]
    pub delete_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Optional. An externally-generated and managed ID for this Membership. This ID may
    /// be modified after creation, but this is not recommended.
    ///
    /// The ID must match the regex: `\[a-zA-Z0-9][a-zA-Z0-9_\-\.\]*`
    ///
    /// If this Membership represents a Kubernetes cluster, this value should be
    /// set to the UID of the `kube-system` namespace object.
    #[prost(string, tag = "9")]
    pub external_id: ::prost::alloc::string::String,
    /// Output only. For clusters using Connect, the timestamp of the most recent connection
    /// established with Google Cloud. This time is updated every several minutes,
    /// not continuously. For clusters that do not use GKE Connect, or that have
    /// never connected successfully, this field will be unset.
    #[prost(message, optional, tag = "10")]
    pub last_connection_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Google-generated UUID for this resource. This is unique across all
    /// Membership resources. If a Membership resource is deleted and another
    /// resource with the same name is created, it gets a different unique_id.
    #[prost(string, tag = "11")]
    pub unique_id: ::prost::alloc::string::String,
    /// Optional. How to identify workloads from this Membership.
    /// See the documentation on Workload Identity for more details:
    /// <https://cloud.google.com/kubernetes-engine/docs/how-to/workload-identity>
    #[prost(message, optional, tag = "12")]
    pub authority: ::core::option::Option<Authority>,
    /// Type of resource represented by this Membership
    #[prost(oneof = "membership::Type", tags = "4")]
    pub r#type: ::core::option::Option<membership::Type>,
}
/// Nested message and enum types in `Membership`.
pub mod membership {
    /// Type of resource represented by this Membership
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Type {
        /// Optional. Endpoint information to reach this member.
        #[prost(message, tag = "4")]
        Endpoint(super::MembershipEndpoint),
    }
}
/// MembershipEndpoint contains information needed to contact a Kubernetes API,
/// endpoint and any additional Kubernetes metadata.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MembershipEndpoint {
    /// Optional. GKE-specific information. Only present if this Membership is a GKE cluster.
    #[prost(message, optional, tag = "1")]
    pub gke_cluster: ::core::option::Option<GkeCluster>,
    /// Output only. Useful Kubernetes-specific metadata.
    #[prost(message, optional, tag = "2")]
    pub kubernetes_metadata: ::core::option::Option<KubernetesMetadata>,
}
/// GkeCluster contains information specific to GKE clusters.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GkeCluster {
    /// Immutable. Self-link of the GCP resource for the GKE cluster. For example:
    ///
    ///     //container.googleapis.com/projects/my-project/locations/us-west1-a/clusters/my-cluster
    ///
    /// Zonal clusters are also supported.
    #[prost(string, tag = "1")]
    pub resource_link: ::prost::alloc::string::String,
}
/// KubernetesMetadata provides informational metadata for Memberships
/// representing Kubernetes clusters.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KubernetesMetadata {
    /// Output only. Kubernetes API server version string as reported by `/version`.
    #[prost(string, tag = "1")]
    pub kubernetes_api_server_version: ::prost::alloc::string::String,
    /// Output only. Node providerID as reported by the first node in the list of nodes on
    /// the Kubernetes endpoint. On Kubernetes platforms that support zero-node
    /// clusters (like GKE-on-GCP), the node_count will be zero and the
    /// node_provider_id will be empty.
    #[prost(string, tag = "2")]
    pub node_provider_id: ::prost::alloc::string::String,
    /// Output only. Node count as reported by Kubernetes nodes resources.
    #[prost(int32, tag = "3")]
    pub node_count: i32,
    /// Output only. vCPU count as reported by Kubernetes nodes resources.
    #[prost(int32, tag = "4")]
    pub vcpu_count: i32,
    /// Output only. The total memory capacity as reported by the sum of all Kubernetes nodes
    /// resources, defined in MB.
    #[prost(int32, tag = "5")]
    pub memory_mb: i32,
    /// Output only. The time at which these details were last updated. This update_time is
    /// different from the Membership-level update_time since EndpointDetails are
    /// updated internally for API consumers.
    #[prost(message, optional, tag = "100")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// MembershipState describes the state of a Membership resource.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MembershipState {
    /// Output only. The current state of the Membership resource.
    #[prost(enumeration = "membership_state::Code", tag = "1")]
    pub code: i32,
}
/// Nested message and enum types in `MembershipState`.
pub mod membership_state {
    /// Code describes the state of a Membership resource.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Code {
        /// The code is not set.
        Unspecified = 0,
        /// The cluster is being registered.
        Creating = 1,
        /// The cluster is registered.
        Ready = 2,
        /// The cluster is being unregistered.
        Deleting = 3,
        /// The Membership is being updated.
        Updating = 4,
        /// The Membership is being updated by the Hub Service.
        ServiceUpdating = 5,
    }
}
/// Authority encodes how Google will recognize identities from this Membership.
/// See the workload identity documentation for more details:
/// <https://cloud.google.com/kubernetes-engine/docs/how-to/workload-identity>
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Authority {
    /// Optional. A JSON Web Token (JWT) issuer URI. `issuer` must start with `<https://`> and
    /// be a valid URL with length <2000 characters.
    ///
    /// If set, then Google will allow valid OIDC tokens from this issuer to
    /// authenticate within the workload_identity_pool. OIDC discovery will be
    /// performed on this URI to validate tokens from the issuer.
    ///
    /// Clearing `issuer` disables Workload Identity. `issuer` cannot be directly
    /// modified; it must be cleared (and Workload Identity disabled) before using
    /// a new issuer (and re-enabling Workload Identity).
    #[prost(string, tag = "1")]
    pub issuer: ::prost::alloc::string::String,
    /// Output only. The name of the workload identity pool in which `issuer` will be
    /// recognized.
    ///
    /// There is a single Workload Identity Pool per Hub that is shared
    /// between all Memberships that belong to that Hub. For a Hub hosted in
    /// {PROJECT_ID}, the workload pool format is `{PROJECT_ID}.hub.id.goog`,
    /// although this is subject to change in newer versions of this API.
    #[prost(string, tag = "2")]
    pub workload_identity_pool: ::prost::alloc::string::String,
    /// Output only. An identity provider that reflects the `issuer` in the workload identity
    /// pool.
    #[prost(string, tag = "3")]
    pub identity_provider: ::prost::alloc::string::String,
    /// Optional. OIDC verification keys for this Membership in JWKS format (RFC 7517).
    ///
    /// When this field is set, OIDC discovery will NOT be performed on `issuer`,
    /// and instead OIDC tokens will be validated using this field.
    #[prost(bytes = "vec", tag = "4")]
    pub oidc_jwks: ::prost::alloc::vec::Vec<u8>,
}
/// Request message for `GkeHub.ListMemberships` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListMembershipsRequest {
    /// Required. The parent (project and location) where the Memberships will be listed.
    /// Specified in the format `projects/*/locations/*`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. When requesting a 'page' of resources, `page_size` specifies number of
    /// resources to return. If unspecified or set to 0, all resources will
    /// be returned.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. Token returned by previous call to `ListMemberships` which
    /// specifies the position in the list from where to continue listing the
    /// resources.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. Lists Memberships that match the filter expression, following the syntax
    /// outlined in <https://google.aip.dev/160.>
    ///
    /// Examples:
    ///
    ///   - Name is `bar` in project `foo-proj` and location `global`:
    ///
    ///       name = "projects/foo-proj/locations/global/membership/bar"
    ///
    ///   - Memberships that have a label called `foo`:
    ///
    ///       labels.foo:*
    ///
    ///   - Memberships that have a label called `foo` whose value is `bar`:
    ///
    ///       labels.foo = bar
    ///
    ///   - Memberships in the CREATING state:
    ///
    ///       state = CREATING
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Optional. One or more fields to compare and use to sort the output.
    /// See <https://google.aip.dev/132#ordering.>
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
/// Response message for the `GkeHub.ListMemberships` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListMembershipsResponse {
    /// The list of matching Memberships.
    #[prost(message, repeated, tag = "1")]
    pub resources: ::prost::alloc::vec::Vec<Membership>,
    /// A token to request the next page of resources from the
    /// `ListMemberships` method. The value of an empty string means that
    /// there are no more resources to return.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// List of locations that could not be reached while fetching this list.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Request message for `GkeHub.GetMembership` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMembershipRequest {
    /// Required. The Membership resource name in the format
    /// `projects/*/locations/*/memberships/*`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for the `GkeHub.CreateMembership` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateMembershipRequest {
    /// Required. The parent (project and location) where the Memberships will be created.
    /// Specified in the format `projects/*/locations/*`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. Client chosen ID for the membership. `membership_id` must be a valid RFC
    /// 1123 compliant DNS label:
    ///
    ///   1. At most 63 characters in length
    ///   2. It must consist of lower case alphanumeric characters or `-`
    ///   3. It must start and end with an alphanumeric character
    ///
    /// Which can be expressed as the regex: `\[a-z0-9]([-a-z0-9]*[a-z0-9\])?`,
    /// with a maximum length of 63 characters.
    #[prost(string, tag = "2")]
    pub membership_id: ::prost::alloc::string::String,
    /// Required. The membership to create.
    #[prost(message, optional, tag = "3")]
    pub resource: ::core::option::Option<Membership>,
    /// Optional. A request ID to identify requests. Specify a unique request ID
    /// so that if you must retry your request, the server will know to ignore
    /// the request if it has already been completed. The server will guarantee
    /// that for at least 60 minutes after the first request.
    ///
    /// For example, consider a situation where you make an initial request and
    /// the request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "4")]
    pub request_id: ::prost::alloc::string::String,
}
/// Request message for `GkeHub.DeleteMembership` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteMembershipRequest {
    /// Required. The Membership resource name in the format
    /// `projects/*/locations/*/memberships/*`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. A request ID to identify requests. Specify a unique request ID
    /// so that if you must retry your request, the server will know to ignore
    /// the request if it has already been completed. The server will guarantee
    /// that for at least 60 minutes after the first request.
    ///
    /// For example, consider a situation where you make an initial request and
    /// the request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "2")]
    pub request_id: ::prost::alloc::string::String,
}
/// Request message for `GkeHub.UpdateMembership` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateMembershipRequest {
    /// Required. The Membership resource name in the format
    /// `projects/*/locations/*/memberships/*`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. Mask of fields to update.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Required. Only fields specified in update_mask are updated.
    /// If you specify a field in the update_mask but don't specify its value here
    /// that field will be deleted.
    /// If you are updating a map field, set the value of a key to null or empty
    /// string to delete the key from the map. It's not possible to update a key's
    /// value to the empty string.
    /// If you specify the update_mask to be a special path "*", fully replaces all
    /// user-modifiable fields to match `resource`.
    #[prost(message, optional, tag = "3")]
    pub resource: ::core::option::Option<Membership>,
    /// Optional. A request ID to identify requests. Specify a unique request ID
    /// so that if you must retry your request, the server will know to ignore
    /// the request if it has already been completed. The server will guarantee
    /// that for at least 60 minutes after the first request.
    ///
    /// For example, consider a situation where you make an initial request and
    /// the request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "4")]
    pub request_id: ::prost::alloc::string::String,
}
/// Request message for `GkeHub.GenerateConnectManifest`
/// method.
/// .
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateConnectManifestRequest {
    /// Required. The Membership resource name the Agent will associate with, in the format
    /// `projects/*/locations/*/memberships/*`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. Namespace for GKE Connect agent resources. Defaults to `gke-connect`.
    ///
    /// The Connect Agent is authorized automatically when run in the default
    /// namespace. Otherwise, explicit authorization must be granted with an
    /// additional IAM binding.
    #[prost(string, tag = "2")]
    pub namespace: ::prost::alloc::string::String,
    /// Optional. URI of a proxy if connectivity from the agent to gkeconnect.googleapis.com
    /// requires the use of a proxy. Format must be in the form
    /// `http(s)://{proxy_address}`, depending on the HTTP/HTTPS protocol
    /// supported by the proxy. This will direct the connect agent's outbound
    /// traffic through a HTTP(S) proxy.
    #[prost(bytes = "vec", tag = "3")]
    pub proxy: ::prost::alloc::vec::Vec<u8>,
    /// Optional. The Connect agent version to use. Defaults to the most current version.
    #[prost(string, tag = "4")]
    pub version: ::prost::alloc::string::String,
    /// Optional. If true, generate the resources for upgrade only. Some resources
    /// generated only for installation (e.g. secrets) will be excluded.
    #[prost(bool, tag = "5")]
    pub is_upgrade: bool,
    /// Optional. The registry to fetch the connect agent image from. Defaults to
    /// gcr.io/gkeconnect.
    #[prost(string, tag = "6")]
    pub registry: ::prost::alloc::string::String,
    /// Optional. The image pull secret content for the registry, if not public.
    #[prost(bytes = "vec", tag = "7")]
    pub image_pull_secret_content: ::prost::alloc::vec::Vec<u8>,
}
/// GenerateConnectManifestResponse contains manifest information for
/// installing/upgrading a Connect agent.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateConnectManifestResponse {
    /// The ordered list of Kubernetes resources that need to be applied to the
    /// cluster for GKE Connect agent installation/upgrade.
    #[prost(message, repeated, tag = "1")]
    pub manifest: ::prost::alloc::vec::Vec<ConnectAgentResource>,
}
/// ConnectAgentResource represents a Kubernetes resource manifest for Connect
/// Agent deployment.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConnectAgentResource {
    /// Kubernetes type of the resource.
    #[prost(message, optional, tag = "1")]
    pub r#type: ::core::option::Option<TypeMeta>,
    /// YAML manifest of the resource.
    #[prost(string, tag = "2")]
    pub manifest: ::prost::alloc::string::String,
}
/// TypeMeta is the type information needed for content unmarshalling of
/// Kubernetes resources in the manifest.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TypeMeta {
    /// Kind of the resource (e.g. Deployment).
    #[prost(string, tag = "1")]
    pub kind: ::prost::alloc::string::String,
    /// APIVersion of the resource (e.g. v1).
    #[prost(string, tag = "2")]
    pub api_version: ::prost::alloc::string::String,
}
/// Request message for `GkeHub.ListFeatures` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListFeaturesRequest {
    /// The parent (project and location) where the Features will be listed.
    /// Specified in the format `projects/*/locations/*`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// When requesting a 'page' of resources, `page_size` specifies number of
    /// resources to return. If unspecified or set to 0, all resources will
    /// be returned.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Token returned by previous call to `ListFeatures` which
    /// specifies the position in the list from where to continue listing the
    /// resources.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Lists Features that match the filter expression, following the syntax
    /// outlined in <https://google.aip.dev/160.>
    ///
    /// Examples:
    ///
    ///   - Feature with the name "servicemesh" in project "foo-proj":
    ///
    ///       name = "projects/foo-proj/locations/global/features/servicemesh"
    ///
    ///   - Features that have a label called `foo`:
    ///
    ///       labels.foo:*
    ///
    ///   - Features that have a label called `foo` whose value is `bar`:
    ///
    ///       labels.foo = bar
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// One or more fields to compare and use to sort the output.
    /// See <https://google.aip.dev/132#ordering.>
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
/// Response message for the `GkeHub.ListFeatures` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListFeaturesResponse {
    /// The list of matching Features
    #[prost(message, repeated, tag = "1")]
    pub resources: ::prost::alloc::vec::Vec<Feature>,
    /// A token to request the next page of resources from the
    /// `ListFeatures` method. The value of an empty string means
    /// that there are no more resources to return.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for `GkeHub.GetFeature` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetFeatureRequest {
    /// The Feature resource name in the format
    /// `projects/*/locations/*/features/*`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for the `GkeHub.CreateFeature` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateFeatureRequest {
    /// The parent (project and location) where the Feature will be created.
    /// Specified in the format `projects/*/locations/*`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The ID of the feature to create.
    #[prost(string, tag = "2")]
    pub feature_id: ::prost::alloc::string::String,
    /// The Feature resource to create.
    #[prost(message, optional, tag = "3")]
    pub resource: ::core::option::Option<Feature>,
    /// Optional. A request ID to identify requests. Specify a unique request ID
    /// so that if you must retry your request, the server will know to ignore
    /// the request if it has already been completed. The server will guarantee
    /// that for at least 60 minutes after the first request.
    ///
    /// For example, consider a situation where you make an initial request and
    /// the request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "4")]
    pub request_id: ::prost::alloc::string::String,
}
/// Request message for `GkeHub.DeleteFeature` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteFeatureRequest {
    /// The Feature resource name in the format
    /// `projects/*/locations/*/features/*`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// If set to true, the delete will ignore any outstanding resources for
    /// this Feature (that is, `FeatureState.has_resources` is set to true). These
    /// resources will NOT be cleaned up or modified in any way.
    #[prost(bool, tag = "2")]
    pub force: bool,
    /// Optional. A request ID to identify requests. Specify a unique request ID
    /// so that if you must retry your request, the server will know to ignore
    /// the request if it has already been completed. The server will guarantee
    /// that for at least 60 minutes after the first request.
    ///
    /// For example, consider a situation where you make an initial request and
    /// the request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "3")]
    pub request_id: ::prost::alloc::string::String,
}
/// Request message for `GkeHub.UpdateFeature` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateFeatureRequest {
    /// The Feature resource name in the format
    /// `projects/*/locations/*/features/*`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Mask of fields to update.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Only fields specified in update_mask are updated.
    /// If you specify a field in the update_mask but don't specify its value here
    /// that field will be deleted.
    /// If you are updating a map field, set the value of a key to null or empty
    /// string to delete the key from the map. It's not possible to update a key's
    /// value to the empty string.
    /// If you specify the update_mask to be a special path "*", fully replaces all
    /// user-modifiable fields to match `resource`.
    #[prost(message, optional, tag = "3")]
    pub resource: ::core::option::Option<Feature>,
    /// Optional. A request ID to identify requests. Specify a unique request ID
    /// so that if you must retry your request, the server will know to ignore
    /// the request if it has already been completed. The server will guarantee
    /// that for at least 60 minutes after the first request.
    ///
    /// For example, consider a situation where you make an initial request and
    /// the request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "4")]
    pub request_id: ::prost::alloc::string::String,
}
/// Represents the metadata of the long-running operation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationMetadata {
    /// Output only. The time the operation was created.
    #[prost(message, optional, tag = "1")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The time the operation finished running.
    #[prost(message, optional, tag = "2")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Server-defined resource path for the target of the operation.
    #[prost(string, tag = "3")]
    pub target: ::prost::alloc::string::String,
    /// Output only. Name of the verb executed by the operation.
    #[prost(string, tag = "4")]
    pub verb: ::prost::alloc::string::String,
    /// Output only. Human-readable status of the operation, if any.
    #[prost(string, tag = "5")]
    pub status_detail: ::prost::alloc::string::String,
    /// Output only. Identifies whether the user has requested cancellation
    /// of the operation. Operations that have successfully been cancelled
    /// have \[Operation.error][\] value with a \[google.rpc.Status.code][google.rpc.Status.code\] of 1,
    /// corresponding to `Code.CANCELLED`.
    #[prost(bool, tag = "6")]
    pub cancel_requested: bool,
    /// Output only. API version used to start the operation.
    #[prost(string, tag = "7")]
    pub api_version: ::prost::alloc::string::String,
}
#[doc = r" Generated client implementations."]
pub mod gke_hub_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " The GKE Hub service handles the registration of many Kubernetes clusters to"]
    #[doc = " Google Cloud, and the management of multi-cluster features over those"]
    #[doc = " clusters."]
    #[doc = ""]
    #[doc = " The GKE Hub service operates on the following resources:"]
    #[doc = ""]
    #[doc = " * [Membership][google.cloud.gkehub.v1.Membership]"]
    #[doc = " * [Feature][google.cloud.gkehub.v1.Feature]"]
    #[doc = ""]
    #[doc = " GKE Hub is currently only available in the global region."]
    #[doc = ""]
    #[doc = " **Membership management may be non-trivial:** it is recommended to use one"]
    #[doc = " of the Google-provided client libraries or tools where possible when working"]
    #[doc = " with Membership resources."]
    #[derive(Debug, Clone)]
    pub struct GkeHubClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> GkeHubClient<T>
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
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> GkeHubClient<InterceptedService<T, F>>
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
            GkeHubClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Lists Memberships in a given project and location."]
        pub async fn list_memberships(
            &mut self,
            request: impl tonic::IntoRequest<super::ListMembershipsRequest>,
        ) -> Result<tonic::Response<super::ListMembershipsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.gkehub.v1.GkeHub/ListMemberships",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists Features in a given project and location."]
        pub async fn list_features(
            &mut self,
            request: impl tonic::IntoRequest<super::ListFeaturesRequest>,
        ) -> Result<tonic::Response<super::ListFeaturesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/google.cloud.gkehub.v1.GkeHub/ListFeatures");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets the details of a Membership."]
        pub async fn get_membership(
            &mut self,
            request: impl tonic::IntoRequest<super::GetMembershipRequest>,
        ) -> Result<tonic::Response<super::Membership>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.gkehub.v1.GkeHub/GetMembership",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets details of a single Feature."]
        pub async fn get_feature(
            &mut self,
            request: impl tonic::IntoRequest<super::GetFeatureRequest>,
        ) -> Result<tonic::Response<super::Feature>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/google.cloud.gkehub.v1.GkeHub/GetFeature");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a new Membership."]
        #[doc = ""]
        #[doc = " **This is currently only supported for GKE clusters on Google Cloud**."]
        #[doc = " To register other clusters, follow the instructions at"]
        #[doc = " https://cloud.google.com/anthos/multicluster-management/connect/registering-a-cluster."]
        pub async fn create_membership(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateMembershipRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.gkehub.v1.GkeHub/CreateMembership",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Adds a new Feature."]
        pub async fn create_feature(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateFeatureRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.gkehub.v1.GkeHub/CreateFeature",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Removes a Membership."]
        #[doc = ""]
        #[doc = " **This is currently only supported for GKE clusters on Google Cloud**."]
        #[doc = " To unregister other clusters, follow the instructions at"]
        #[doc = " https://cloud.google.com/anthos/multicluster-management/connect/unregistering-a-cluster."]
        pub async fn delete_membership(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteMembershipRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.gkehub.v1.GkeHub/DeleteMembership",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Removes a Feature."]
        pub async fn delete_feature(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteFeatureRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.gkehub.v1.GkeHub/DeleteFeature",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates an existing Membership."]
        pub async fn update_membership(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateMembershipRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.gkehub.v1.GkeHub/UpdateMembership",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates an existing Feature."]
        pub async fn update_feature(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateFeatureRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.gkehub.v1.GkeHub/UpdateFeature",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Generates the manifest for deployment of the GKE connect agent."]
        #[doc = ""]
        #[doc = " **This method is used internally by Google-provided libraries.**"]
        #[doc = " Most clients should not need to call this method directly."]
        pub async fn generate_connect_manifest(
            &mut self,
            request: impl tonic::IntoRequest<super::GenerateConnectManifestRequest>,
        ) -> Result<tonic::Response<super::GenerateConnectManifestResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.gkehub.v1.GkeHub/GenerateConnectManifest",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
