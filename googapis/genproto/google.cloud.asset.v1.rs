/// An asset in Google Cloud and its temporal metadata, including the time window
/// when it was observed and its status during that window.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TemporalAsset {
    /// The time window when the asset data and state was observed.
    #[prost(message, optional, tag = "1")]
    pub window: ::core::option::Option<TimeWindow>,
    /// Whether the asset has been deleted or not.
    #[prost(bool, tag = "2")]
    pub deleted: bool,
    /// An asset in Google Cloud.
    #[prost(message, optional, tag = "3")]
    pub asset: ::core::option::Option<Asset>,
    /// State of prior_asset.
    #[prost(enumeration = "temporal_asset::PriorAssetState", tag = "4")]
    pub prior_asset_state: i32,
    /// Prior copy of the asset. Populated if prior_asset_state is PRESENT.
    /// Currently this is only set for responses in Real-Time Feed.
    #[prost(message, optional, tag = "5")]
    pub prior_asset: ::core::option::Option<Asset>,
}
/// Nested message and enum types in `TemporalAsset`.
pub mod temporal_asset {
    /// State of prior asset.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum PriorAssetState {
        /// prior_asset is not applicable for the current asset.
        Unspecified = 0,
        /// prior_asset is populated correctly.
        Present = 1,
        /// Failed to set prior_asset.
        Invalid = 2,
        /// Current asset is the first known state.
        DoesNotExist = 3,
        /// prior_asset is a deletion.
        Deleted = 4,
    }
}
/// A time window specified by its `start_time` and `end_time`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TimeWindow {
    /// Start time of the time window (exclusive).
    #[prost(message, optional, tag = "1")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// End time of the time window (inclusive). If not specified, the current
    /// timestamp is used instead.
    #[prost(message, optional, tag = "2")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// An asset in Google Cloud. An asset can be any resource in the Google Cloud
/// [resource
/// hierarchy](https://cloud.google.com/resource-manager/docs/cloud-platform-resource-hierarchy),
/// a resource outside the Google Cloud resource hierarchy (such as Google
/// Kubernetes Engine clusters and objects), or a policy (e.g. Cloud IAM policy).
/// See [Supported asset
/// types](https://cloud.google.com/asset-inventory/docs/supported-asset-types)
/// for more information.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Asset {
    /// The last update timestamp of an asset. update_time is updated when
    /// create/update/delete operation is performed.
    #[prost(message, optional, tag = "11")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The full name of the asset. Example:
    /// `//compute.googleapis.com/projects/my_project_123/zones/zone1/instances/instance1`
    ///
    /// See [Resource
    /// names](https://cloud.google.com/apis/design/resource_names#full_resource_name)
    /// for more information.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The type of the asset. Example: `compute.googleapis.com/Disk`
    ///
    /// See [Supported asset
    /// types](https://cloud.google.com/asset-inventory/docs/supported-asset-types)
    /// for more information.
    #[prost(string, tag = "2")]
    pub asset_type: ::prost::alloc::string::String,
    /// A representation of the resource.
    #[prost(message, optional, tag = "3")]
    pub resource: ::core::option::Option<Resource>,
    /// A representation of the Cloud IAM policy set on a Google Cloud resource.
    /// There can be a maximum of one Cloud IAM policy set on any given resource.
    /// In addition, Cloud IAM policies inherit their granted access scope from any
    /// policies set on parent resources in the resource hierarchy. Therefore, the
    /// effectively policy is the union of both the policy set on this resource
    /// and each policy set on all of the resource's ancestry resource levels in
    /// the hierarchy. See
    /// [this topic](https://cloud.google.com/iam/docs/policies#inheritance) for
    /// more information.
    #[prost(message, optional, tag = "4")]
    pub iam_policy: ::core::option::Option<super::super::super::iam::v1::Policy>,
    /// A representation of an [organization
    /// policy](https://cloud.google.com/resource-manager/docs/organization-policy/overview#organization_policy).
    /// There can be more than one organization policy with different constraints
    /// set on a given resource.
    #[prost(message, repeated, tag = "6")]
    pub org_policy: ::prost::alloc::vec::Vec<super::super::orgpolicy::v1::Policy>,
    /// A representation of runtime OS Inventory information. See [this
    /// topic](https://cloud.google.com/compute/docs/instances/os-inventory-management)
    /// for more information.
    #[prost(message, optional, tag = "12")]
    pub os_inventory: ::core::option::Option<super::super::osconfig::v1::Inventory>,
    /// The ancestry path of an asset in Google Cloud [resource
    /// hierarchy](https://cloud.google.com/resource-manager/docs/cloud-platform-resource-hierarchy),
    /// represented as a list of relative resource names. An ancestry path starts
    /// with the closest ancestor in the hierarchy and ends at root. If the asset
    /// is a project, folder, or organization, the ancestry path starts from the
    /// asset itself.
    ///
    /// Example: `["projects/123456789", "folders/5432", "organizations/1234"]`
    #[prost(string, repeated, tag = "10")]
    pub ancestors: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// A representation of an [access
    /// policy](https://cloud.google.com/access-context-manager/docs/overview#access-policies).
    #[prost(oneof = "asset::AccessContextPolicy", tags = "7, 8, 9")]
    pub access_context_policy: ::core::option::Option<asset::AccessContextPolicy>,
}
/// Nested message and enum types in `Asset`.
pub mod asset {
    /// A representation of an [access
    /// policy](https://cloud.google.com/access-context-manager/docs/overview#access-policies).
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum AccessContextPolicy {
        /// Please also refer to the [access policy user
        /// guide](https://cloud.google.com/access-context-manager/docs/overview#access-policies).
        #[prost(message, tag = "7")]
        AccessPolicy(super::super::super::super::identity::accesscontextmanager::v1::AccessPolicy),
        /// Please also refer to the [access level user
        /// guide](https://cloud.google.com/access-context-manager/docs/overview#access-levels).
        #[prost(message, tag = "8")]
        AccessLevel(super::super::super::super::identity::accesscontextmanager::v1::AccessLevel),
        /// Please also refer to the [service perimeter user
        /// guide](https://cloud.google.com/vpc-service-controls/docs/overview).
        #[prost(message, tag = "9")]
        ServicePerimeter(
            super::super::super::super::identity::accesscontextmanager::v1::ServicePerimeter,
        ),
    }
}
/// A representation of a Google Cloud resource.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Resource {
    /// The API version. Example: `v1`
    #[prost(string, tag = "1")]
    pub version: ::prost::alloc::string::String,
    /// The URL of the discovery document containing the resource's JSON schema.
    /// Example:
    /// `https://www.googleapis.com/discovery/v1/apis/compute/v1/rest`
    ///
    /// This value is unspecified for resources that do not have an API based on a
    /// discovery document, such as Cloud Bigtable.
    #[prost(string, tag = "2")]
    pub discovery_document_uri: ::prost::alloc::string::String,
    /// The JSON schema name listed in the discovery document. Example:
    /// `Project`
    ///
    /// This value is unspecified for resources that do not have an API based on a
    /// discovery document, such as Cloud Bigtable.
    #[prost(string, tag = "3")]
    pub discovery_name: ::prost::alloc::string::String,
    /// The REST URL for accessing the resource. An HTTP `GET` request using this
    /// URL returns the resource itself. Example:
    /// `https://cloudresourcemanager.googleapis.com/v1/projects/my-project-123`
    ///
    /// This value is unspecified for resources without a REST API.
    #[prost(string, tag = "4")]
    pub resource_url: ::prost::alloc::string::String,
    /// The full name of the immediate parent of this resource. See
    /// [Resource
    /// Names](https://cloud.google.com/apis/design/resource_names#full_resource_name)
    /// for more information.
    ///
    /// For Google Cloud assets, this value is the parent resource defined in the
    /// [Cloud IAM policy
    /// hierarchy](https://cloud.google.com/iam/docs/overview#policy_hierarchy).
    /// Example:
    /// `//cloudresourcemanager.googleapis.com/projects/my_project_123`
    ///
    /// For third-party assets, this field may be set differently.
    #[prost(string, tag = "5")]
    pub parent: ::prost::alloc::string::String,
    /// The content of the resource, in which some sensitive fields are removed
    /// and may not be present.
    #[prost(message, optional, tag = "6")]
    pub data: ::core::option::Option<::prost_types::Struct>,
    /// The location of the resource in Google Cloud, such as its zone and region.
    /// For more information, see https://cloud.google.com/about/locations/.
    #[prost(string, tag = "8")]
    pub location: ::prost::alloc::string::String,
}
/// A result of Resource Search, containing information of a cloud resource.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResourceSearchResult {
    /// The full resource name of this resource. Example:
    /// `//compute.googleapis.com/projects/my_project_123/zones/zone1/instances/instance1`.
    /// See [Cloud Asset Inventory Resource Name
    /// Format](https://cloud.google.com/asset-inventory/docs/resource-name-format)
    /// for more information.
    ///
    /// To search against the `name`:
    ///
    /// * use a field query. Example: `name:instance1`
    /// * use a free text query. Example: `instance1`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The type of this resource. Example: `compute.googleapis.com/Disk`.
    ///
    /// To search against the `asset_type`:
    ///
    /// * specify the `asset_type` field in your search request.
    #[prost(string, tag = "2")]
    pub asset_type: ::prost::alloc::string::String,
    /// The project that this resource belongs to, in the form of
    /// projects/{PROJECT_NUMBER}.
    ///
    /// To search against the `project`:
    ///
    /// * specify the `scope` field as this project in your search request.
    #[prost(string, tag = "3")]
    pub project: ::prost::alloc::string::String,
    /// The display name of this resource.
    ///
    /// To search against the `display_name`:
    ///
    /// * use a field query. Example: `displayName:"My Instance"`
    /// * use a free text query. Example: `"My Instance"`
    #[prost(string, tag = "4")]
    pub display_name: ::prost::alloc::string::String,
    /// One or more paragraphs of text description of this resource. Maximum length
    /// could be up to 1M bytes.
    ///
    /// To search against the `description`:
    ///
    /// * use a field query. Example: `description:"*important instance*"`
    /// * use a free text query. Example: `"*important instance*"`
    #[prost(string, tag = "5")]
    pub description: ::prost::alloc::string::String,
    /// Location can be `global`, regional like `us-east1`, or zonal like
    /// `us-west1-b`.
    ///
    /// To search against the `location`:
    ///
    /// * use a field query. Example: `location:us-west*`
    /// * use a free text query. Example: `us-west*`
    #[prost(string, tag = "6")]
    pub location: ::prost::alloc::string::String,
    /// Labels associated with this resource. See [Labelling and grouping GCP
    /// resources](https://cloud.google.com/blog/products/gcp/labelling-and-grouping-your-google-cloud-platform-resources)
    /// for more information.
    ///
    /// To search against the `labels`:
    ///
    /// * use a field query:
    ///     - query on any label's key or value. Example: `labels:prod`
    ///     - query by a given label. Example: `labels.env:prod`
    ///     - query by a given label's existence. Example: `labels.env:*`
    /// * use a free text query. Example: `prod`
    #[prost(map = "string, string", tag = "7")]
    pub labels:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Network tags associated with this resource. Like labels, network tags are a
    /// type of annotations used to group GCP resources. See [Labelling GCP
    /// resources](https://cloud.google.com/blog/products/gcp/labelling-and-grouping-your-google-cloud-platform-resources)
    /// for more information.
    ///
    /// To search against the `network_tags`:
    ///
    /// * use a field query. Example: `networkTags:internal`
    /// * use a free text query. Example: `internal`
    #[prost(string, repeated, tag = "8")]
    pub network_tags: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The additional searchable attributes of this resource. The attributes may
    /// vary from one resource type to another. Examples: `projectId` for Project,
    /// `dnsName` for DNS ManagedZone. This field contains a subset of the resource
    /// metadata fields that are returned by the List or Get APIs provided by the
    /// corresponding GCP service (e.g., Compute Engine). see [API references and
    /// supported searchable
    /// attributes](https://cloud.google.com/asset-inventory/docs/supported-asset-types#searchable_asset_types)
    /// for more information.
    ///
    /// You can search values of these fields through free text search. However,
    /// you should not consume the field programically as the field names and
    /// values may change as the GCP service updates to a new incompatible API
    /// version.
    ///
    /// To search against the `additional_attributes`:
    ///
    /// * use a free text query to match the attributes values. Example: to search
    ///   `additional_attributes = { dnsName: "foobar" }`, you can issue a query
    ///   `foobar`.
    #[prost(message, optional, tag = "9")]
    pub additional_attributes: ::core::option::Option<::prost_types::Struct>,
}
/// A result of IAM Policy search, containing information of an IAM policy.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IamPolicySearchResult {
    /// The full resource name of the resource associated with this IAM policy.
    /// Example:
    /// `//compute.googleapis.com/projects/my_project_123/zones/zone1/instances/instance1`.
    /// See [Cloud Asset Inventory Resource Name
    /// Format](https://cloud.google.com/asset-inventory/docs/resource-name-format)
    /// for more information.
    ///
    /// To search against the `resource`:
    ///
    /// * use a field query. Example: `resource:organizations/123`
    #[prost(string, tag = "1")]
    pub resource: ::prost::alloc::string::String,
    /// The project that the associated GCP resource belongs to, in the form of
    /// projects/{PROJECT_NUMBER}. If an IAM policy is set on a resource (like VM
    /// instance, Cloud Storage bucket), the project field will indicate the
    /// project that contains the resource. If an IAM policy is set on a folder or
    /// orgnization, the project field will be empty.
    ///
    /// To search against the `project`:
    ///
    /// * specify the `scope` field as this project in your search request.
    #[prost(string, tag = "2")]
    pub project: ::prost::alloc::string::String,
    /// The IAM policy directly set on the given resource. Note that the original
    /// IAM policy can contain multiple bindings. This only contains the bindings
    /// that match the given query. For queries that don't contain a constrain on
    /// policies (e.g., an empty query), this contains all the bindings.
    ///
    /// To search against the `policy` bindings:
    ///
    /// * use a field query:
    ///     - query by the policy contained members. Example:
    ///       `policy:amy@gmail.com`
    ///     - query by the policy contained roles. Example:
    ///       `policy:roles/compute.admin`
    ///     - query by the policy contained roles' included permissions. Example:
    ///       `policy.role.permissions:compute.instances.create`
    #[prost(message, optional, tag = "3")]
    pub policy: ::core::option::Option<super::super::super::iam::v1::Policy>,
    /// Explanation about the IAM policy search result. It contains additional
    /// information to explain why the search result matches the query.
    #[prost(message, optional, tag = "4")]
    pub explanation: ::core::option::Option<iam_policy_search_result::Explanation>,
}
/// Nested message and enum types in `IamPolicySearchResult`.
pub mod iam_policy_search_result {
    /// Explanation about the IAM policy search result.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Explanation {
        /// The map from roles to their included permissions that match the
        /// permission query (i.e., a query containing `policy.role.permissions:`).
        /// Example: if query `policy.role.permissions:compute.disk.get`
        /// matches a policy binding that contains owner role, the
        /// matched_permissions will be `{"roles/owner": ["compute.disk.get"]}`. The
        /// roles can also be found in the returned `policy` bindings. Note that the
        /// map is populated only for requests with permission queries.
        #[prost(map = "string, message", tag = "1")]
        pub matched_permissions:
            ::std::collections::HashMap<::prost::alloc::string::String, explanation::Permissions>,
    }
    /// Nested message and enum types in `Explanation`.
    pub mod explanation {
        /// IAM permissions
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Permissions {
            /// A list of permissions. A sample permission string: `compute.disk.get`.
            #[prost(string, repeated, tag = "1")]
            pub permissions: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        }
    }
}
/// Represents the detailed state of an entity under analysis, such as a
/// resource, an identity or an access.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IamPolicyAnalysisState {
    /// The Google standard error code that best describes the state.
    /// For example:
    /// - OK means the analysis on this entity has been successfully finished;
    /// - PERMISSION_DENIED means an access denied error is encountered;
    /// - DEADLINE_EXCEEDED means the analysis on this entity hasn't been started
    /// in time;
    #[prost(enumeration = "super::super::super::rpc::Code", tag = "1")]
    pub code: i32,
    /// The human-readable description of the cause of failure.
    #[prost(string, tag = "2")]
    pub cause: ::prost::alloc::string::String,
}
/// IAM Policy analysis result, consisting of one IAM policy binding and derived
/// access control lists.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IamPolicyAnalysisResult {
    /// The [full resource
    /// name](https://cloud.google.com/asset-inventory/docs/resource-name-format)
    /// of the resource to which the [iam_binding][google.cloud.asset.v1.IamPolicyAnalysisResult.iam_binding] policy attaches.
    #[prost(string, tag = "1")]
    pub attached_resource_full_name: ::prost::alloc::string::String,
    /// The Cloud IAM policy binding under analysis.
    #[prost(message, optional, tag = "2")]
    pub iam_binding: ::core::option::Option<super::super::super::iam::v1::Binding>,
    /// The access control lists derived from the [iam_binding][google.cloud.asset.v1.IamPolicyAnalysisResult.iam_binding] that match or
    /// potentially match resource and access selectors specified in the request.
    #[prost(message, repeated, tag = "3")]
    pub access_control_lists:
        ::prost::alloc::vec::Vec<iam_policy_analysis_result::AccessControlList>,
    /// The identity list derived from members of the [iam_binding][google.cloud.asset.v1.IamPolicyAnalysisResult.iam_binding] that match or
    /// potentially match identity selector specified in the request.
    #[prost(message, optional, tag = "4")]
    pub identity_list: ::core::option::Option<iam_policy_analysis_result::IdentityList>,
    /// Represents whether all analyses on the [iam_binding][google.cloud.asset.v1.IamPolicyAnalysisResult.iam_binding] have successfully
    /// finished.
    #[prost(bool, tag = "5")]
    pub fully_explored: bool,
}
/// Nested message and enum types in `IamPolicyAnalysisResult`.
pub mod iam_policy_analysis_result {
    /// A Google Cloud resource under analysis.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Resource {
        /// The [full resource
        /// name](https://cloud.google.com/asset-inventory/docs/resource-name-format)
        #[prost(string, tag = "1")]
        pub full_resource_name: ::prost::alloc::string::String,
        /// The analysis state of this resource.
        #[prost(message, optional, tag = "2")]
        pub analysis_state: ::core::option::Option<super::IamPolicyAnalysisState>,
    }
    /// An IAM role or permission under analysis.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Access {
        /// The analysis state of this access.
        #[prost(message, optional, tag = "3")]
        pub analysis_state: ::core::option::Option<super::IamPolicyAnalysisState>,
        #[prost(oneof = "access::OneofAccess", tags = "1, 2")]
        pub oneof_access: ::core::option::Option<access::OneofAccess>,
    }
    /// Nested message and enum types in `Access`.
    pub mod access {
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum OneofAccess {
            /// The role.
            #[prost(string, tag = "1")]
            Role(::prost::alloc::string::String),
            /// The permission.
            #[prost(string, tag = "2")]
            Permission(::prost::alloc::string::String),
        }
    }
    /// An identity under analysis.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Identity {
        /// The identity name in any form of members appear in
        /// [IAM policy
        /// binding](https://cloud.google.com/iam/reference/rest/v1/Binding), such
        /// as:
        /// - user:foo@google.com
        /// - group:group1@google.com
        /// - serviceAccount:s1@prj1.iam.gserviceaccount.com
        /// - projectOwner:some_project_id
        /// - domain:google.com
        /// - allUsers
        /// - etc.
        #[prost(string, tag = "1")]
        pub name: ::prost::alloc::string::String,
        /// The analysis state of this identity.
        #[prost(message, optional, tag = "2")]
        pub analysis_state: ::core::option::Option<super::IamPolicyAnalysisState>,
    }
    /// A directional edge.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Edge {
        /// The source node of the edge. For example, it could be a full resource
        /// name for a resource node or an email of an identity.
        #[prost(string, tag = "1")]
        pub source_node: ::prost::alloc::string::String,
        /// The target node of the edge. For example, it could be a full resource
        /// name for a resource node or an email of an identity.
        #[prost(string, tag = "2")]
        pub target_node: ::prost::alloc::string::String,
    }
    /// An access control list, derived from the above IAM policy binding, which
    /// contains a set of resources and accesses. May include one
    /// item from each set to compose an access control entry.
    ///
    /// NOTICE that there could be multiple access control lists for one IAM policy
    /// binding. The access control lists are created based on resource and access
    /// combinations.
    ///
    /// For example, assume we have the following cases in one IAM policy binding:
    /// - Permission P1 and P2 apply to resource R1 and R2;
    /// - Permission P3 applies to resource R2 and R3;
    ///
    /// This will result in the following access control lists:
    /// - AccessControlList 1: [R1, R2], [P1, P2]
    /// - AccessControlList 2: [R2, R3], [P3]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct AccessControlList {
        /// The resources that match one of the following conditions:
        /// - The resource_selector, if it is specified in request;
        /// - Otherwise, resources reachable from the policy attached resource.
        #[prost(message, repeated, tag = "1")]
        pub resources: ::prost::alloc::vec::Vec<Resource>,
        /// The accesses that match one of the following conditions:
        /// - The access_selector, if it is specified in request;
        /// - Otherwise, access specifiers reachable from the policy binding's role.
        #[prost(message, repeated, tag = "2")]
        pub accesses: ::prost::alloc::vec::Vec<Access>,
        /// Resource edges of the graph starting from the policy attached
        /// resource to any descendant resources. The [Edge.source_node][google.cloud.asset.v1.IamPolicyAnalysisResult.Edge.source_node] contains
        /// the full resource name of a parent resource and [Edge.target_node][google.cloud.asset.v1.IamPolicyAnalysisResult.Edge.target_node]
        /// contains the full resource name of a child resource. This field is
        /// present only if the output_resource_edges option is enabled in request.
        #[prost(message, repeated, tag = "3")]
        pub resource_edges: ::prost::alloc::vec::Vec<Edge>,
    }
    /// The identities and group edges.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct IdentityList {
        /// Only the identities that match one of the following conditions will be
        /// presented:
        /// - The identity_selector, if it is specified in request;
        /// - Otherwise, identities reachable from the policy binding's members.
        #[prost(message, repeated, tag = "1")]
        pub identities: ::prost::alloc::vec::Vec<Identity>,
        /// Group identity edges of the graph starting from the binding's
        /// group members to any node of the [identities][google.cloud.asset.v1.IamPolicyAnalysisResult.IdentityList.identities]. The [Edge.source_node][google.cloud.asset.v1.IamPolicyAnalysisResult.Edge.source_node]
        /// contains a group, such as `group:parent@google.com`. The
        /// [Edge.target_node][google.cloud.asset.v1.IamPolicyAnalysisResult.Edge.target_node] contains a member of the group,
        /// such as `group:child@google.com` or `user:foo@google.com`.
        /// This field is present only if the output_group_edges option is enabled in
        /// request.
        #[prost(message, repeated, tag = "2")]
        pub group_edges: ::prost::alloc::vec::Vec<Edge>,
    }
}
/// Export asset request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportAssetsRequest {
    /// Required. The relative name of the root asset. This can only be an
    /// organization number (such as "organizations/123"), a project ID (such as
    /// "projects/my-project-id"), or a project number (such as "projects/12345"),
    /// or a folder number (such as "folders/123").
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Timestamp to take an asset snapshot. This can only be set to a timestamp
    /// between the current time and the current time minus 35 days (inclusive).
    /// If not specified, the current time will be used. Due to delays in resource
    /// data collection and indexing, there is a volatile window during which
    /// running the same query may get different results.
    #[prost(message, optional, tag = "2")]
    pub read_time: ::core::option::Option<::prost_types::Timestamp>,
    /// A list of asset types to take a snapshot for. For example:
    /// "compute.googleapis.com/Disk".
    ///
    /// Regular expressions are also supported. For example:
    ///
    /// * "compute.googleapis.com.*" snapshots resources whose asset type starts
    /// with "compute.googleapis.com".
    /// * ".*Instance" snapshots resources whose asset type ends with "Instance".
    /// * ".*Instance.*" snapshots resources whose asset type contains "Instance".
    ///
    /// See [RE2](https://github.com/google/re2/wiki/Syntax) for all supported
    /// regular expression syntax. If the regular expression does not match any
    /// supported asset type, an INVALID_ARGUMENT error will be returned.
    ///
    /// If specified, only matching assets will be returned, otherwise, it will
    /// snapshot all asset types. See [Introduction to Cloud Asset
    /// Inventory](https://cloud.google.com/asset-inventory/docs/overview)
    /// for all supported asset types.
    #[prost(string, repeated, tag = "3")]
    pub asset_types: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Asset content type. If not specified, no content but the asset name will be
    /// returned.
    #[prost(enumeration = "ContentType", tag = "4")]
    pub content_type: i32,
    /// Required. Output configuration indicating where the results will be output to.
    #[prost(message, optional, tag = "5")]
    pub output_config: ::core::option::Option<OutputConfig>,
}
/// The export asset response. This message is returned by the
/// [google.longrunning.Operations.GetOperation][google.longrunning.Operations.GetOperation] method in the returned
/// [google.longrunning.Operation.response][google.longrunning.Operation.response] field.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportAssetsResponse {
    /// Time the snapshot was taken.
    #[prost(message, optional, tag = "1")]
    pub read_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output configuration indicating where the results were output to.
    #[prost(message, optional, tag = "2")]
    pub output_config: ::core::option::Option<OutputConfig>,
    /// Output result indicating where the assets were exported to. For example, a
    /// set of actual Google Cloud Storage object uris where the assets are
    /// exported to. The uris can be different from what [output_config] has
    /// specified, as the service will split the output object into multiple ones
    /// once it exceeds a single Google Cloud Storage object limit.
    #[prost(message, optional, tag = "3")]
    pub output_result: ::core::option::Option<OutputResult>,
}
/// Batch get assets history request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchGetAssetsHistoryRequest {
    /// Required. The relative name of the root asset. It can only be an
    /// organization number (such as "organizations/123"), a project ID (such as
    /// "projects/my-project-id")", or a project number (such as "projects/12345").
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// A list of the full names of the assets.
    /// See: https://cloud.google.com/asset-inventory/docs/resource-name-format
    /// Example:
    ///
    /// `//compute.googleapis.com/projects/my_project_123/zones/zone1/instances/instance1`.
    ///
    /// The request becomes a no-op if the asset name list is empty, and the max
    /// size of the asset name list is 100 in one request.
    #[prost(string, repeated, tag = "2")]
    pub asset_names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Optional. The content type.
    #[prost(enumeration = "ContentType", tag = "3")]
    pub content_type: i32,
    /// Optional. The time window for the asset history. Both start_time and
    /// end_time are optional and if set, it must be after the current time minus
    /// 35 days. If end_time is not set, it is default to current timestamp.
    /// If start_time is not set, the snapshot of the assets at end_time will be
    /// returned. The returned results contain all temporal assets whose time
    /// window overlap with read_time_window.
    #[prost(message, optional, tag = "4")]
    pub read_time_window: ::core::option::Option<TimeWindow>,
}
/// Batch get assets history response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchGetAssetsHistoryResponse {
    /// A list of assets with valid time windows.
    #[prost(message, repeated, tag = "1")]
    pub assets: ::prost::alloc::vec::Vec<TemporalAsset>,
}
/// Create asset feed request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateFeedRequest {
    /// Required. The name of the project/folder/organization where this feed
    /// should be created in. It can only be an organization number (such as
    /// "organizations/123"), a folder number (such as "folders/123"), a project ID
    /// (such as "projects/my-project-id")", or a project number (such as
    /// "projects/12345").
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. This is the client-assigned asset feed identifier and it needs to
    /// be unique under a specific parent project/folder/organization.
    #[prost(string, tag = "2")]
    pub feed_id: ::prost::alloc::string::String,
    /// Required. The feed details. The field `name` must be empty and it will be generated
    /// in the format of:
    /// projects/project_number/feeds/feed_id
    /// folders/folder_number/feeds/feed_id
    /// organizations/organization_number/feeds/feed_id
    #[prost(message, optional, tag = "3")]
    pub feed: ::core::option::Option<Feed>,
}
/// Get asset feed request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetFeedRequest {
    /// Required. The name of the Feed and it must be in the format of:
    /// projects/project_number/feeds/feed_id
    /// folders/folder_number/feeds/feed_id
    /// organizations/organization_number/feeds/feed_id
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// List asset feeds request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListFeedsRequest {
    /// Required. The parent project/folder/organization whose feeds are to be
    /// listed. It can only be using project/folder/organization number (such as
    /// "folders/12345")", or a project ID (such as "projects/my-project-id").
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListFeedsResponse {
    /// A list of feeds.
    #[prost(message, repeated, tag = "1")]
    pub feeds: ::prost::alloc::vec::Vec<Feed>,
}
/// Update asset feed request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateFeedRequest {
    /// Required. The new values of feed details. It must match an existing feed and the
    /// field `name` must be in the format of:
    /// projects/project_number/feeds/feed_id or
    /// folders/folder_number/feeds/feed_id or
    /// organizations/organization_number/feeds/feed_id.
    #[prost(message, optional, tag = "1")]
    pub feed: ::core::option::Option<Feed>,
    /// Required. Only updates the `feed` fields indicated by this mask.
    /// The field mask must not be empty, and it must not contain fields that
    /// are immutable or only set by the server.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteFeedRequest {
    /// Required. The name of the feed and it must be in the format of:
    /// projects/project_number/feeds/feed_id
    /// folders/folder_number/feeds/feed_id
    /// organizations/organization_number/feeds/feed_id
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Output configuration for export assets destination.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OutputConfig {
    /// Asset export destination.
    #[prost(oneof = "output_config::Destination", tags = "1, 2")]
    pub destination: ::core::option::Option<output_config::Destination>,
}
/// Nested message and enum types in `OutputConfig`.
pub mod output_config {
    /// Asset export destination.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Destination {
        /// Destination on Cloud Storage.
        #[prost(message, tag = "1")]
        GcsDestination(super::GcsDestination),
        /// Destination on BigQuery. The output table stores the fields in asset
        /// proto as columns in BigQuery.
        #[prost(message, tag = "2")]
        BigqueryDestination(super::BigQueryDestination),
    }
}
/// Output result of export assets.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OutputResult {
    /// Asset export result.
    #[prost(oneof = "output_result::Result", tags = "1")]
    pub result: ::core::option::Option<output_result::Result>,
}
/// Nested message and enum types in `OutputResult`.
pub mod output_result {
    /// Asset export result.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Result {
        /// Export result on Cloud Storage.
        #[prost(message, tag = "1")]
        GcsResult(super::GcsOutputResult),
    }
}
/// A Cloud Storage output result.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GcsOutputResult {
    /// List of uris of the Cloud Storage objects. Example:
    /// "gs://bucket_name/object_name".
    #[prost(string, repeated, tag = "1")]
    pub uris: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// A Cloud Storage location.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GcsDestination {
    /// Required.
    #[prost(oneof = "gcs_destination::ObjectUri", tags = "1, 2")]
    pub object_uri: ::core::option::Option<gcs_destination::ObjectUri>,
}
/// Nested message and enum types in `GcsDestination`.
pub mod gcs_destination {
    /// Required.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ObjectUri {
        /// The uri of the Cloud Storage object. It's the same uri that is used by
        /// gsutil. Example: "gs://bucket_name/object_name". See [Viewing and
        /// Editing Object
        /// Metadata](https://cloud.google.com/storage/docs/viewing-editing-metadata)
        /// for more information.
        #[prost(string, tag = "1")]
        Uri(::prost::alloc::string::String),
        /// The uri prefix of all generated Cloud Storage objects. Example:
        /// "gs://bucket_name/object_name_prefix". Each object uri is in format:
        /// "gs://bucket_name/object_name_prefix/<asset type>/<shard number> and only
        /// contains assets for that type. <shard number> starts from 0. Example:
        /// "gs://bucket_name/object_name_prefix/compute.googleapis.com/Disk/0" is
        /// the first shard of output objects containing all
        /// compute.googleapis.com/Disk assets. An INVALID_ARGUMENT error will be
        /// returned if file with the same name "gs://bucket_name/object_name_prefix"
        /// already exists.
        #[prost(string, tag = "2")]
        UriPrefix(::prost::alloc::string::String),
    }
}
/// A BigQuery destination for exporting assets to.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BigQueryDestination {
    /// Required. The BigQuery dataset in format
    /// "projects/projectId/datasets/datasetId", to which the snapshot result
    /// should be exported. If this dataset does not exist, the export call returns
    /// an INVALID_ARGUMENT error.
    #[prost(string, tag = "1")]
    pub dataset: ::prost::alloc::string::String,
    /// Required. The BigQuery table to which the snapshot result should be
    /// written. If this table does not exist, a new table with the given name
    /// will be created.
    #[prost(string, tag = "2")]
    pub table: ::prost::alloc::string::String,
    /// If the destination table already exists and this flag is `TRUE`, the
    /// table will be overwritten by the contents of assets snapshot. If the flag
    /// is `FALSE` or unset and the destination table already exists, the export
    /// call returns an INVALID_ARGUMEMT error.
    #[prost(bool, tag = "3")]
    pub force: bool,
    /// [partition_spec] determines whether to export to partitioned table(s) and
    /// how to partition the data.
    ///
    /// If [partition_spec] is unset or [partition_spec.partition_key] is unset or
    /// `PARTITION_KEY_UNSPECIFIED`, the snapshot results will be exported to
    /// non-partitioned table(s). [force] will decide whether to overwrite existing
    /// table(s).
    ///
    /// If [partition_spec] is specified. First, the snapshot results will be
    /// written to partitioned table(s) with two additional timestamp columns,
    /// readTime and requestTime, one of which will be the partition key. Secondly,
    /// in the case when any destination table already exists, it will first try to
    /// update existing table's schema as necessary by appending additional
    /// columns. Then, if [force] is `TRUE`, the corresponding partition will be
    /// overwritten by the snapshot results (data in different partitions will
    /// remain intact); if [force] is unset or `FALSE`, it will append the data. An
    /// error will be returned if the schema update or data appension fails.
    #[prost(message, optional, tag = "4")]
    pub partition_spec: ::core::option::Option<PartitionSpec>,
    /// If this flag is `TRUE`, the snapshot results will be written to one or
    /// multiple tables, each of which contains results of one asset type. The
    /// [force] and [partition_spec] fields will apply to each of them.
    ///
    /// Field [table] will be concatenated with "_" and the asset type names (see
    /// https://cloud.google.com/asset-inventory/docs/supported-asset-types for
    /// supported asset types) to construct per-asset-type table names, in which
    /// all non-alphanumeric characters like "." and "/" will be substituted by
    /// "_". Example: if field [table] is "mytable" and snapshot results
    /// contain "storage.googleapis.com/Bucket" assets, the corresponding table
    /// name will be "mytable_storage_googleapis_com_Bucket". If any of these
    /// tables does not exist, a new table with the concatenated name will be
    /// created.
    ///
    /// When [content_type] in the ExportAssetsRequest is `RESOURCE`, the schema of
    /// each table will include RECORD-type columns mapped to the nested fields in
    /// the Asset.resource.data field of that asset type (up to the 15 nested level
    /// BigQuery supports
    /// (https://cloud.google.com/bigquery/docs/nested-repeated#limitations)). The
    /// fields in >15 nested levels will be stored in JSON format string as a child
    /// column of its parent RECORD column.
    ///
    /// If error occurs when exporting to any table, the whole export call will
    /// return an error but the export results that already succeed will persist.
    /// Example: if exporting to table_type_A succeeds when exporting to
    /// table_type_B fails during one export call, the results in table_type_A will
    /// persist and there will not be partial results persisting in a table.
    #[prost(bool, tag = "5")]
    pub separate_tables_per_asset_type: bool,
}
/// Specifications of BigQuery partitioned table as export destination.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PartitionSpec {
    /// The partition key for BigQuery partitioned table.
    #[prost(enumeration = "partition_spec::PartitionKey", tag = "1")]
    pub partition_key: i32,
}
/// Nested message and enum types in `PartitionSpec`.
pub mod partition_spec {
    /// This enum is used to determine the partition key column when exporting
    /// assets to BigQuery partitioned table(s). Note that, if the partition key is
    /// a timestamp column, the actual partition is based on its date value
    /// (expressed in UTC. see details in
    /// https://cloud.google.com/bigquery/docs/partitioned-tables#date_timestamp_partitioned_tables).
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum PartitionKey {
        /// Unspecified partition key. If used, it means using non-partitioned table.
        Unspecified = 0,
        /// The time when the snapshot is taken. If specified as partition key, the
        /// result table(s) is partitoned by the additional timestamp column,
        /// readTime. If [read_time] in ExportAssetsRequest is specified, the
        /// readTime column's value will be the same as it. Otherwise, its value will
        /// be the current time that is used to take the snapshot.
        ReadTime = 1,
        /// The time when the request is received and started to be processed. If
        /// specified as partition key, the result table(s) is partitoned by the
        /// requestTime column, an additional timestamp column representing when the
        /// request was received.
        RequestTime = 2,
    }
}
/// A Pub/Sub destination.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PubsubDestination {
    /// The name of the Pub/Sub topic to publish to.
    /// Example: `projects/PROJECT_ID/topics/TOPIC_ID`.
    #[prost(string, tag = "1")]
    pub topic: ::prost::alloc::string::String,
}
/// Output configuration for asset feed destination.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeedOutputConfig {
    /// Asset feed destination.
    #[prost(oneof = "feed_output_config::Destination", tags = "1")]
    pub destination: ::core::option::Option<feed_output_config::Destination>,
}
/// Nested message and enum types in `FeedOutputConfig`.
pub mod feed_output_config {
    /// Asset feed destination.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Destination {
        /// Destination on Pub/Sub.
        #[prost(message, tag = "1")]
        PubsubDestination(super::PubsubDestination),
    }
}
/// An asset feed used to export asset updates to a destinations.
/// An asset feed filter controls what updates are exported.
/// The asset feed must be created within a project, organization, or
/// folder. Supported destinations are:
/// Pub/Sub topics.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Feed {
    /// Required. The format will be
    /// projects/{project_number}/feeds/{client-assigned_feed_identifier} or
    /// folders/{folder_number}/feeds/{client-assigned_feed_identifier} or
    /// organizations/{organization_number}/feeds/{client-assigned_feed_identifier}
    ///
    /// The client-assigned feed identifier must be unique within the parent
    /// project/folder/organization.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// A list of the full names of the assets to receive updates. You must specify
    /// either or both of asset_names and asset_types. Only asset updates matching
    /// specified asset_names or asset_types are exported to the feed.
    /// Example:
    /// `//compute.googleapis.com/projects/my_project_123/zones/zone1/instances/instance1`.
    /// See [Resource
    /// Names](https://cloud.google.com/apis/design/resource_names#full_resource_name)
    /// for more info.
    #[prost(string, repeated, tag = "2")]
    pub asset_names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// A list of types of the assets to receive updates. You must specify either
    /// or both of asset_names and asset_types. Only asset updates matching
    /// specified asset_names or asset_types are exported to the feed.
    /// Example: `"compute.googleapis.com/Disk"`
    ///
    /// See [this
    /// topic](https://cloud.google.com/asset-inventory/docs/supported-asset-types)
    /// for a list of all supported asset types.
    #[prost(string, repeated, tag = "3")]
    pub asset_types: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Asset content type. If not specified, no content but the asset name and
    /// type will be returned.
    #[prost(enumeration = "ContentType", tag = "4")]
    pub content_type: i32,
    /// Required. Feed output configuration defining where the asset updates are
    /// published to.
    #[prost(message, optional, tag = "5")]
    pub feed_output_config: ::core::option::Option<FeedOutputConfig>,
    /// A condition which determines whether an asset update should be published.
    /// If specified, an asset will be returned only when the expression evaluates
    /// to true.
    /// When set, `expression` field in the `Expr` must be a valid [CEL expression]
    /// (https://github.com/google/cel-spec) on a TemporalAsset with name
    /// `temporal_asset`. Example: a Feed with expression ("temporal_asset.deleted
    /// == true") will only publish Asset deletions. Other fields of `Expr` are
    /// optional.
    ///
    /// See our [user
    /// guide](https://cloud.google.com/asset-inventory/docs/monitoring-asset-changes#feed_with_condition)
    /// for detailed instructions.
    #[prost(message, optional, tag = "6")]
    pub condition: ::core::option::Option<super::super::super::r#type::Expr>,
}
/// Search all resources request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchAllResourcesRequest {
    /// Required. A scope can be a project, a folder, or an organization. The search is
    /// limited to the resources within the `scope`. The caller must be granted the
    /// [`cloudasset.assets.searchAllResources`](http://cloud.google.com/asset-inventory/docs/access-control#required_permissions)
    /// permission on the desired scope.
    ///
    /// The allowed values are:
    ///
    /// * projects/{PROJECT_ID} (e.g., "projects/foo-bar")
    /// * projects/{PROJECT_NUMBER} (e.g., "projects/12345678")
    /// * folders/{FOLDER_NUMBER} (e.g., "folders/1234567")
    /// * organizations/{ORGANIZATION_NUMBER} (e.g., "organizations/123456")
    #[prost(string, tag = "1")]
    pub scope: ::prost::alloc::string::String,
    /// Optional. The query statement. See [how to construct a
    /// query](http://cloud.google.com/asset-inventory/docs/searching-resources#how_to_construct_a_query)
    /// for more information. If not specified or empty, it will search all the
    /// resources within the specified `scope`. Note that the query string is
    /// compared against each Cloud IAM policy binding, including its members,
    /// roles, and Cloud IAM conditions. The returned Cloud IAM policies will only
    /// contain the bindings that match your query. To learn more about the IAM
    /// policy structure, see [IAM policy
    /// doc](https://cloud.google.com/iam/docs/policies#structure).
    ///
    /// Examples:
    ///
    /// * `name:Important` to find Cloud resources whose name contains
    ///   "Important" as a word.
    /// * `displayName:Impor*` to find Cloud resources whose display name
    ///   contains "Impor" as a prefix.
    /// * `description:*por*` to find Cloud resources whose description
    ///   contains "por" as a substring.
    /// * `location:us-west*` to find Cloud resources whose location is
    ///   prefixed with "us-west".
    /// * `labels:prod` to find Cloud resources whose labels contain "prod" as
    ///   a key or value.
    /// * `labels.env:prod` to find Cloud resources that have a label "env"
    ///   and its value is "prod".
    /// * `labels.env:*` to find Cloud resources that have a label "env".
    /// * `Important` to find Cloud resources that contain "Important" as a word
    ///   in any of the searchable fields.
    /// * `Impor*` to find Cloud resources that contain "Impor" as a prefix
    ///   in any of the searchable fields.
    /// * `*por*` to find Cloud resources that contain "por" as a substring in
    ///   any of the searchable fields.
    /// * `Important location:(us-west1 OR global)` to find Cloud
    ///   resources that contain "Important" as a word in any of the searchable
    ///   fields and are also located in the "us-west1" region or the "global"
    ///   location.
    #[prost(string, tag = "2")]
    pub query: ::prost::alloc::string::String,
    /// Optional. A list of asset types that this request searches for. If empty, it will
    /// search all the [searchable asset
    /// types](https://cloud.google.com/asset-inventory/docs/supported-asset-types#searchable_asset_types).
    #[prost(string, repeated, tag = "3")]
    pub asset_types: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Optional. The page size for search result pagination. Page size is capped at 500 even
    /// if a larger value is given. If set to zero, server will pick an appropriate
    /// default. Returned results may be fewer than requested. When this happens,
    /// there could be more results as long as `next_page_token` is returned.
    #[prost(int32, tag = "4")]
    pub page_size: i32,
    /// Optional. If present, then retrieve the next batch of results from the preceding call
    /// to this method. `page_token` must be the value of `next_page_token` from
    /// the previous response. The values of all other method parameters, must be
    /// identical to those in the previous call.
    #[prost(string, tag = "5")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. A comma separated list of fields specifying the sorting order of the
    /// results. The default order is ascending. Add " DESC" after the field name
    /// to indicate descending order. Redundant space characters are ignored.
    /// Example: "location DESC, name". Only string fields in the response are
    /// sortable, including `name`, `displayName`, `description`, `location`. All
    /// the other fields such as repeated fields (e.g., `networkTags`), map
    /// fields (e.g., `labels`) and struct fields (e.g., `additionalAttributes`)
    /// are not supported.
    #[prost(string, tag = "6")]
    pub order_by: ::prost::alloc::string::String,
}
/// Search all resources response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchAllResourcesResponse {
    /// A list of Resources that match the search query. It contains the resource
    /// standard metadata information.
    #[prost(message, repeated, tag = "1")]
    pub results: ::prost::alloc::vec::Vec<ResourceSearchResult>,
    /// If there are more results than those appearing in this response, then
    /// `next_page_token` is included. To get the next set of results, call this
    /// method again using the value of `next_page_token` as `page_token`.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Search all IAM policies request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchAllIamPoliciesRequest {
    /// Required. A scope can be a project, a folder, or an organization. The search is
    /// limited to the IAM policies within the `scope`. The caller must be granted
    /// the
    /// [`cloudasset.assets.searchAllIamPolicies`](http://cloud.google.com/asset-inventory/docs/access-control#required_permissions)
    /// permission on the desired scope.
    ///
    /// The allowed values are:
    ///
    /// * projects/{PROJECT_ID} (e.g., "projects/foo-bar")
    /// * projects/{PROJECT_NUMBER} (e.g., "projects/12345678")
    /// * folders/{FOLDER_NUMBER} (e.g., "folders/1234567")
    /// * organizations/{ORGANIZATION_NUMBER} (e.g., "organizations/123456")
    #[prost(string, tag = "1")]
    pub scope: ::prost::alloc::string::String,
    /// Optional. The query statement. See [how to construct a
    /// query](https://cloud.google.com/asset-inventory/docs/searching-iam-policies#how_to_construct_a_query)
    /// for more information. If not specified or empty, it will search all the
    /// IAM policies within the specified `scope`.
    ///
    /// Examples:
    ///
    /// * `policy:amy@gmail.com` to find IAM policy bindings that specify user
    ///   "amy@gmail.com".
    /// * `policy:roles/compute.admin` to find IAM policy bindings that specify
    ///   the Compute Admin role.
    /// * `policy.role.permissions:storage.buckets.update` to find IAM policy
    ///   bindings that specify a role containing "storage.buckets.update"
    ///   permission. Note that if callers don't have `iam.roles.get` access to a
    ///   role's included permissions, policy bindings that specify this role will
    ///   be dropped from the search results.
    /// * `resource:organizations/123456` to find IAM policy bindings
    ///   that are set on "organizations/123456".
    /// * `Important` to find IAM policy bindings that contain "Important" as a
    ///   word in any of the searchable fields (except for the included
    ///   permissions).
    /// * `*por*` to find IAM policy bindings that contain "por" as a substring
    ///   in any of the searchable fields (except for the included permissions).
    /// * `resource:(instance1 OR instance2) policy:amy` to find
    ///   IAM policy bindings that are set on resources "instance1" or
    ///   "instance2" and also specify user "amy".
    #[prost(string, tag = "2")]
    pub query: ::prost::alloc::string::String,
    /// Optional. The page size for search result pagination. Page size is capped at 500 even
    /// if a larger value is given. If set to zero, server will pick an appropriate
    /// default. Returned results may be fewer than requested. When this happens,
    /// there could be more results as long as `next_page_token` is returned.
    #[prost(int32, tag = "3")]
    pub page_size: i32,
    /// Optional. If present, retrieve the next batch of results from the preceding call to
    /// this method. `page_token` must be the value of `next_page_token` from the
    /// previous response. The values of all other method parameters must be
    /// identical to those in the previous call.
    #[prost(string, tag = "4")]
    pub page_token: ::prost::alloc::string::String,
}
/// Search all IAM policies response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchAllIamPoliciesResponse {
    /// A list of IamPolicy that match the search query. Related information such
    /// as the associated resource is returned along with the policy.
    #[prost(message, repeated, tag = "1")]
    pub results: ::prost::alloc::vec::Vec<IamPolicySearchResult>,
    /// Set if there are more results than those appearing in this response; to get
    /// the next set of results, call this method again, using this value as the
    /// `page_token`.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// IAM policy analysis query message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IamPolicyAnalysisQuery {
    /// Required. The relative name of the root asset. Only resources and IAM policies within
    /// the scope will be analyzed.
    ///
    /// This can only be an organization number (such as "organizations/123"), a
    /// folder number (such as "folders/123"), a project ID (such as
    /// "projects/my-project-id"), or a project number (such as "projects/12345").
    ///
    /// To know how to get organization id, visit [here
    /// ](https://cloud.google.com/resource-manager/docs/creating-managing-organization#retrieving_your_organization_id).
    ///
    /// To know how to get folder or project id, visit [here
    /// ](https://cloud.google.com/resource-manager/docs/creating-managing-folders#viewing_or_listing_folders_and_projects).
    #[prost(string, tag = "1")]
    pub scope: ::prost::alloc::string::String,
    /// Optional. Specifies a resource for analysis.
    #[prost(message, optional, tag = "2")]
    pub resource_selector: ::core::option::Option<iam_policy_analysis_query::ResourceSelector>,
    /// Optional. Specifies an identity for analysis.
    #[prost(message, optional, tag = "3")]
    pub identity_selector: ::core::option::Option<iam_policy_analysis_query::IdentitySelector>,
    /// Optional. Specifies roles or permissions for analysis. This is optional.
    #[prost(message, optional, tag = "4")]
    pub access_selector: ::core::option::Option<iam_policy_analysis_query::AccessSelector>,
    /// Optional. The query options.
    #[prost(message, optional, tag = "5")]
    pub options: ::core::option::Option<iam_policy_analysis_query::Options>,
}
/// Nested message and enum types in `IamPolicyAnalysisQuery`.
pub mod iam_policy_analysis_query {
    /// Specifies the resource to analyze for access policies, which may be set
    /// directly on the resource, or on ancestors such as organizations, folders or
    /// projects.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ResourceSelector {
        /// Required. The [full resource name]
        /// (https://cloud.google.com/asset-inventory/docs/resource-name-format)
        /// of a resource of [supported resource
        /// types](https://cloud.google.com/asset-inventory/docs/supported-asset-types#analyzable_asset_types).
        #[prost(string, tag = "1")]
        pub full_resource_name: ::prost::alloc::string::String,
    }
    /// Specifies an identity for which to determine resource access, based on
    /// roles assigned either directly to them or to the groups they belong to,
    /// directly or indirectly.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct IdentitySelector {
        /// Required. The identity appear in the form of members in
        /// [IAM policy
        /// binding](https://cloud.google.com/iam/reference/rest/v1/Binding).
        ///
        /// The examples of supported forms are:
        /// "user:mike@example.com",
        /// "group:admins@example.com",
        /// "domain:google.com",
        /// "serviceAccount:my-project-id@appspot.gserviceaccount.com".
        ///
        /// Notice that wildcard characters (such as * and ?) are not supported.
        /// You must give a specific identity.
        #[prost(string, tag = "1")]
        pub identity: ::prost::alloc::string::String,
    }
    /// Specifies roles and/or permissions to analyze, to determine both the
    /// identities possessing them and the resources they control. If multiple
    /// values are specified, results will include roles or permissions matching
    /// any of them. The total number of roles and permissions should be equal or
    /// less than 10.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct AccessSelector {
        /// Optional. The roles to appear in result.
        #[prost(string, repeated, tag = "1")]
        pub roles: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// Optional. The permissions to appear in result.
        #[prost(string, repeated, tag = "2")]
        pub permissions: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
    /// Contains query options.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Options {
        /// Optional. If true, the identities section of the result will expand any
        /// Google groups appearing in an IAM policy binding.
        ///
        /// If [IamPolicyAnalysisQuery.identity_selector][google.cloud.asset.v1.IamPolicyAnalysisQuery.identity_selector] is specified, the
        /// identity in the result will be determined by the selector, and this flag
        /// is not allowed to set.
        ///
        /// Default is false.
        #[prost(bool, tag = "1")]
        pub expand_groups: bool,
        /// Optional. If true, the access section of result will expand any roles
        /// appearing in IAM policy bindings to include their permissions.
        ///
        /// If [IamPolicyAnalysisQuery.access_selector][google.cloud.asset.v1.IamPolicyAnalysisQuery.access_selector] is specified, the access
        /// section of the result will be determined by the selector, and this flag
        /// is not allowed to set.
        ///
        /// Default is false.
        #[prost(bool, tag = "2")]
        pub expand_roles: bool,
        /// Optional. If true and [IamPolicyAnalysisQuery.resource_selector][google.cloud.asset.v1.IamPolicyAnalysisQuery.resource_selector] is not
        /// specified, the resource section of the result will expand any resource
        /// attached to an IAM policy to include resources lower in the resource
        /// hierarchy.
        ///
        /// For example, if the request analyzes for which resources user A has
        /// permission P, and the results include an IAM policy with P on a GCP
        /// folder, the results will also include resources in that folder with
        /// permission P.
        ///
        /// If true and [IamPolicyAnalysisQuery.resource_selector][google.cloud.asset.v1.IamPolicyAnalysisQuery.resource_selector] is specified,
        /// the resource section of the result will expand the specified resource to
        /// include resources lower in the resource hierarchy. Only project or
        /// lower resources are supported. Folder and organization resource cannot be
        /// used together with this option.
        ///
        /// For example, if the request analyzes for which users have permission P on
        /// a GCP project with this option enabled, the results will include all
        /// users who have permission P on that project or any lower resource.
        ///
        /// Default is false.
        #[prost(bool, tag = "3")]
        pub expand_resources: bool,
        /// Optional. If true, the result will output resource edges, starting
        /// from the policy attached resource, to any expanded resources.
        /// Default is false.
        #[prost(bool, tag = "4")]
        pub output_resource_edges: bool,
        /// Optional. If true, the result will output group identity edges, starting
        /// from the binding's group members, to any expanded identities.
        /// Default is false.
        #[prost(bool, tag = "5")]
        pub output_group_edges: bool,
        /// Optional. If true, the response will include access analysis from identities to
        /// resources via service account impersonation. This is a very expensive
        /// operation, because many derived queries will be executed. We highly
        /// recommend you use [AssetService.AnalyzeIamPolicyLongrunning][google.cloud.asset.v1.AssetService.AnalyzeIamPolicyLongrunning] rpc
        /// instead.
        ///
        /// For example, if the request analyzes for which resources user A has
        /// permission P, and there's an IAM policy states user A has
        /// iam.serviceAccounts.getAccessToken permission to a service account SA,
        /// and there's another IAM policy states service account SA has permission P
        /// to a GCP folder F, then user A potentially has access to the GCP folder
        /// F. And those advanced analysis results will be included in
        /// [AnalyzeIamPolicyResponse.service_account_impersonation_analysis][google.cloud.asset.v1.AnalyzeIamPolicyResponse.service_account_impersonation_analysis].
        ///
        /// Another example, if the request analyzes for who has
        /// permission P to a GCP folder F, and there's an IAM policy states user A
        /// has iam.serviceAccounts.actAs permission to a service account SA, and
        /// there's another IAM policy states service account SA has permission P to
        /// the GCP folder F, then user A potentially has access to the GCP folder
        /// F. And those advanced analysis results will be included in
        /// [AnalyzeIamPolicyResponse.service_account_impersonation_analysis][google.cloud.asset.v1.AnalyzeIamPolicyResponse.service_account_impersonation_analysis].
        ///
        /// Default is false.
        #[prost(bool, tag = "6")]
        pub analyze_service_account_impersonation: bool,
    }
}
/// A request message for [AssetService.AnalyzeIamPolicy][google.cloud.asset.v1.AssetService.AnalyzeIamPolicy].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnalyzeIamPolicyRequest {
    /// Required. The request query.
    #[prost(message, optional, tag = "1")]
    pub analysis_query: ::core::option::Option<IamPolicyAnalysisQuery>,
    /// Optional. Amount of time executable has to complete.  See JSON representation of
    /// [Duration](https://developers.google.com/protocol-buffers/docs/proto3#json).
    ///
    /// If this field is set with a value less than the RPC deadline, and the
    /// execution of your query hasn't finished in the specified
    /// execution timeout,  you will get a response with partial result.
    /// Otherwise, your query's execution will continue until the RPC deadline.
    /// If it's not finished until then, you will get a  DEADLINE_EXCEEDED error.
    ///
    /// Default is empty.
    #[prost(message, optional, tag = "2")]
    pub execution_timeout: ::core::option::Option<::prost_types::Duration>,
}
/// A response message for [AssetService.AnalyzeIamPolicy][google.cloud.asset.v1.AssetService.AnalyzeIamPolicy].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnalyzeIamPolicyResponse {
    /// The main analysis that matches the original request.
    #[prost(message, optional, tag = "1")]
    pub main_analysis: ::core::option::Option<analyze_iam_policy_response::IamPolicyAnalysis>,
    /// The service account impersonation analysis if
    /// [AnalyzeIamPolicyRequest.analyze_service_account_impersonation][] is
    /// enabled.
    #[prost(message, repeated, tag = "2")]
    pub service_account_impersonation_analysis:
        ::prost::alloc::vec::Vec<analyze_iam_policy_response::IamPolicyAnalysis>,
    /// Represents whether all entries in the [main_analysis][google.cloud.asset.v1.AnalyzeIamPolicyResponse.main_analysis] and
    /// [service_account_impersonation_analysis][google.cloud.asset.v1.AnalyzeIamPolicyResponse.service_account_impersonation_analysis] have been fully explored to
    /// answer the query in the request.
    #[prost(bool, tag = "3")]
    pub fully_explored: bool,
}
/// Nested message and enum types in `AnalyzeIamPolicyResponse`.
pub mod analyze_iam_policy_response {
    /// An analysis message to group the query and results.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct IamPolicyAnalysis {
        /// The analysis query.
        #[prost(message, optional, tag = "1")]
        pub analysis_query: ::core::option::Option<super::IamPolicyAnalysisQuery>,
        /// A list of [IamPolicyAnalysisResult][google.cloud.asset.v1.IamPolicyAnalysisResult] that matches the analysis query, or
        /// empty if no result is found.
        #[prost(message, repeated, tag = "2")]
        pub analysis_results: ::prost::alloc::vec::Vec<super::IamPolicyAnalysisResult>,
        /// Represents whether all entries in the [analysis_results][google.cloud.asset.v1.AnalyzeIamPolicyResponse.IamPolicyAnalysis.analysis_results] have been
        /// fully explored to answer the query.
        #[prost(bool, tag = "3")]
        pub fully_explored: bool,
        /// A list of non-critical errors happened during the query handling.
        #[prost(message, repeated, tag = "5")]
        pub non_critical_errors: ::prost::alloc::vec::Vec<super::IamPolicyAnalysisState>,
    }
}
/// Output configuration for export IAM policy analysis destination.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IamPolicyAnalysisOutputConfig {
    /// IAM policy analysis export destination.
    #[prost(
        oneof = "iam_policy_analysis_output_config::Destination",
        tags = "1, 2"
    )]
    pub destination: ::core::option::Option<iam_policy_analysis_output_config::Destination>,
}
/// Nested message and enum types in `IamPolicyAnalysisOutputConfig`.
pub mod iam_policy_analysis_output_config {
    /// A Cloud Storage location.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GcsDestination {
        /// Required. The uri of the Cloud Storage object. It's the same uri that is used by
        /// gsutil. For example: "gs://bucket_name/object_name". See
        /// [Quickstart: Using the gsutil tool]
        /// (https://cloud.google.com/storage/docs/quickstart-gsutil) for examples.
        #[prost(string, tag = "1")]
        pub uri: ::prost::alloc::string::String,
    }
    /// A BigQuery destination.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct BigQueryDestination {
        /// Required. The BigQuery dataset in format "projects/projectId/datasets/datasetId",
        /// to which the analysis results should be exported. If this dataset does
        /// not exist, the export call will return an INVALID_ARGUMENT error.
        #[prost(string, tag = "1")]
        pub dataset: ::prost::alloc::string::String,
        /// Required. The prefix of the BigQuery tables to which the analysis results will be
        /// written. Tables will be created based on this table_prefix if not exist:
        /// * <table_prefix>_analysis table will contain export operation's metadata.
        /// * <table_prefix>_analysis_result will contain all the
        ///   [IamPolicyAnalysisResult][google.cloud.asset.v1.IamPolicyAnalysisResult].
        /// When [partition_key] is specified, both tables will be partitioned based
        /// on the [partition_key].
        #[prost(string, tag = "2")]
        pub table_prefix: ::prost::alloc::string::String,
        /// The partition key for BigQuery partitioned table.
        #[prost(enumeration = "big_query_destination::PartitionKey", tag = "3")]
        pub partition_key: i32,
        /// Optional. Specifies the action that occurs if the destination table or partition
        /// already exists. The following values are supported:
        ///
        /// * WRITE_TRUNCATE: If the table or partition already exists, BigQuery
        /// overwrites the entire table or all the partitions data.
        /// * WRITE_APPEND: If the table or partition already exists, BigQuery
        /// appends the data to the table or the latest partition.
        /// * WRITE_EMPTY: If the table already exists and contains data, an error is
        /// returned.
        ///
        /// The default value is WRITE_APPEND. Each action is atomic and only occurs
        /// if BigQuery is able to complete the job successfully. Details are at
        /// https://cloud.google.com/bigquery/docs/loading-data-local#appending_to_or_overwriting_a_table_using_a_local_file.
        #[prost(string, tag = "4")]
        pub write_disposition: ::prost::alloc::string::String,
    }
    /// Nested message and enum types in `BigQueryDestination`.
    pub mod big_query_destination {
        /// This enum determines the partition key column for the bigquery tables.
        /// Partitioning can improve query performance and reduce query cost by
        /// filtering partitions. Refer to
        /// https://cloud.google.com/bigquery/docs/partitioned-tables for details.
        #[derive(
            Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration,
        )]
        #[repr(i32)]
        pub enum PartitionKey {
            /// Unspecified partition key. Tables won't be partitioned using this
            /// option.
            Unspecified = 0,
            /// The time when the request is received. If specified as partition key,
            /// the result table(s) is partitoned by the RequestTime column, an
            /// additional timestamp column representing when the request was received.
            RequestTime = 1,
        }
    }
    /// IAM policy analysis export destination.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Destination {
        /// Destination on Cloud Storage.
        #[prost(message, tag = "1")]
        GcsDestination(GcsDestination),
        /// Destination on BigQuery.
        #[prost(message, tag = "2")]
        BigqueryDestination(BigQueryDestination),
    }
}
/// A request message for [AssetService.AnalyzeIamPolicyLongrunning][google.cloud.asset.v1.AssetService.AnalyzeIamPolicyLongrunning].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnalyzeIamPolicyLongrunningRequest {
    /// Required. The request query.
    #[prost(message, optional, tag = "1")]
    pub analysis_query: ::core::option::Option<IamPolicyAnalysisQuery>,
    /// Required. Output configuration indicating where the results will be output to.
    #[prost(message, optional, tag = "2")]
    pub output_config: ::core::option::Option<IamPolicyAnalysisOutputConfig>,
}
/// A response message for [AssetService.AnalyzeIamPolicyLongrunning][google.cloud.asset.v1.AssetService.AnalyzeIamPolicyLongrunning].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnalyzeIamPolicyLongrunningResponse {}
/// Asset content type.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ContentType {
    /// Unspecified content type.
    Unspecified = 0,
    /// Resource metadata.
    Resource = 1,
    /// The actual IAM policy set on a resource.
    IamPolicy = 2,
    /// The Cloud Organization Policy set on an asset.
    OrgPolicy = 4,
    /// The Cloud Access context manager Policy set on an asset.
    AccessPolicy = 5,
    /// The runtime OS Inventory information.
    OsInventory = 6,
}
#[doc = r" Generated client implementations."]
pub mod asset_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Asset service definition."]
    pub struct AssetServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> AssetServiceClient<T>
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
        #[doc = " Exports assets with time and resource types to a given Cloud Storage"]
        #[doc = " location/BigQuery table. For Cloud Storage location destinations, the"]
        #[doc = " output format is newline-delimited JSON. Each line represents a"]
        #[doc = " [google.cloud.asset.v1.Asset][google.cloud.asset.v1.Asset] in the JSON format; for BigQuery table"]
        #[doc = " destinations, the output table stores the fields in asset proto as columns."]
        #[doc = " This API implements the [google.longrunning.Operation][google.longrunning.Operation] API"]
        #[doc = " , which allows you to keep track of the export. We recommend intervals of"]
        #[doc = " at least 2 seconds with exponential retry to poll the export operation"]
        #[doc = " result. For regular-size resource parent, the export operation usually"]
        #[doc = " finishes within 5 minutes."]
        pub async fn export_assets(
            &mut self,
            request: impl tonic::IntoRequest<super::ExportAssetsRequest>,
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
                "/google.cloud.asset.v1.AssetService/ExportAssets",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Batch gets the update history of assets that overlap a time window."]
        #[doc = " For IAM_POLICY content, this API outputs history when the asset and its"]
        #[doc = " attached IAM POLICY both exist. This can create gaps in the output history."]
        #[doc = " Otherwise, this API outputs history with asset in both non-delete or"]
        #[doc = " deleted status."]
        #[doc = " If a specified asset does not exist, this API returns an INVALID_ARGUMENT"]
        #[doc = " error."]
        pub async fn batch_get_assets_history(
            &mut self,
            request: impl tonic::IntoRequest<super::BatchGetAssetsHistoryRequest>,
        ) -> Result<tonic::Response<super::BatchGetAssetsHistoryResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.asset.v1.AssetService/BatchGetAssetsHistory",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a feed in a parent project/folder/organization to listen to its"]
        #[doc = " asset updates."]
        pub async fn create_feed(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateFeedRequest>,
        ) -> Result<tonic::Response<super::Feed>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.asset.v1.AssetService/CreateFeed",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets details about an asset feed."]
        pub async fn get_feed(
            &mut self,
            request: impl tonic::IntoRequest<super::GetFeedRequest>,
        ) -> Result<tonic::Response<super::Feed>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/google.cloud.asset.v1.AssetService/GetFeed");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists all asset feeds in a parent project/folder/organization."]
        pub async fn list_feeds(
            &mut self,
            request: impl tonic::IntoRequest<super::ListFeedsRequest>,
        ) -> Result<tonic::Response<super::ListFeedsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.asset.v1.AssetService/ListFeeds",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates an asset feed configuration."]
        pub async fn update_feed(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateFeedRequest>,
        ) -> Result<tonic::Response<super::Feed>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.asset.v1.AssetService/UpdateFeed",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes an asset feed."]
        pub async fn delete_feed(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteFeedRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.asset.v1.AssetService/DeleteFeed",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Searches all Cloud resources within the specified scope, such as a project,"]
        #[doc = " folder, or organization. The caller must be granted the"]
        #[doc = " `cloudasset.assets.searchAllResources` permission on the desired scope,"]
        #[doc = " otherwise the request will be rejected."]
        pub async fn search_all_resources(
            &mut self,
            request: impl tonic::IntoRequest<super::SearchAllResourcesRequest>,
        ) -> Result<tonic::Response<super::SearchAllResourcesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.asset.v1.AssetService/SearchAllResources",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Searches all IAM policies within the specified scope, such as a project,"]
        #[doc = " folder, or organization. The caller must be granted the"]
        #[doc = " `cloudasset.assets.searchAllIamPolicies` permission on the desired scope,"]
        #[doc = " otherwise the request will be rejected."]
        pub async fn search_all_iam_policies(
            &mut self,
            request: impl tonic::IntoRequest<super::SearchAllIamPoliciesRequest>,
        ) -> Result<tonic::Response<super::SearchAllIamPoliciesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.asset.v1.AssetService/SearchAllIamPolicies",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Analyzes IAM policies to answer which identities have what accesses on"]
        #[doc = " which resources."]
        pub async fn analyze_iam_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::AnalyzeIamPolicyRequest>,
        ) -> Result<tonic::Response<super::AnalyzeIamPolicyResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.asset.v1.AssetService/AnalyzeIamPolicy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Analyzes IAM policies asynchronously to answer which identities have what"]
        #[doc = " accesses on which resources, and writes the analysis results to a Google"]
        #[doc = " Cloud Storage or a BigQuery destination. For Cloud Storage destination, the"]
        #[doc = " output format is the JSON format that represents a"]
        #[doc = " [AnalyzeIamPolicyResponse][google.cloud.asset.v1.AnalyzeIamPolicyResponse]. This method implements the"]
        #[doc = " [google.longrunning.Operation][google.longrunning.Operation], which allows you to track the operation"]
        #[doc = " status. We recommend intervals of at least 2 seconds with exponential"]
        #[doc = " backoff retry to poll the operation result. The metadata contains the"]
        #[doc = " request to help callers to map responses to requests."]
        pub async fn analyze_iam_policy_longrunning(
            &mut self,
            request: impl tonic::IntoRequest<super::AnalyzeIamPolicyLongrunningRequest>,
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
                "/google.cloud.asset.v1.AssetService/AnalyzeIamPolicyLongrunning",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for AssetServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for AssetServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "AssetServiceClient {{ ... }}")
        }
    }
}
