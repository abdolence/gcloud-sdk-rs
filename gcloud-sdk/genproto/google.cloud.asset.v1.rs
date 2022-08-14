///  An asset in Google Cloud and its temporal metadata, including the time window
///  when it was observed and its status during that window.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TemporalAsset {
    ///  The time window when the asset data and state was observed.
    #[prost(message, optional, tag="1")]
    pub window: ::core::option::Option<TimeWindow>,
    ///  Whether the asset has been deleted or not.
    #[prost(bool, tag="2")]
    pub deleted: bool,
    ///  An asset in Google Cloud.
    #[prost(message, optional, tag="3")]
    pub asset: ::core::option::Option<Asset>,
    ///  State of prior_asset.
    #[prost(enumeration="temporal_asset::PriorAssetState", tag="4")]
    pub prior_asset_state: i32,
    ///  Prior copy of the asset. Populated if prior_asset_state is PRESENT.
    ///  Currently this is only set for responses in Real-Time Feed.
    #[prost(message, optional, tag="5")]
    pub prior_asset: ::core::option::Option<Asset>,
}
/// Nested message and enum types in `TemporalAsset`.
pub mod temporal_asset {
    ///  State of prior asset.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum PriorAssetState {
        ///  prior_asset is not applicable for the current asset.
        Unspecified = 0,
        ///  prior_asset is populated correctly.
        Present = 1,
        ///  Failed to set prior_asset.
        Invalid = 2,
        ///  Current asset is the first known state.
        DoesNotExist = 3,
        ///  prior_asset is a deletion.
        Deleted = 4,
    }
    impl PriorAssetState {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                PriorAssetState::Unspecified => "PRIOR_ASSET_STATE_UNSPECIFIED",
                PriorAssetState::Present => "PRESENT",
                PriorAssetState::Invalid => "INVALID",
                PriorAssetState::DoesNotExist => "DOES_NOT_EXIST",
                PriorAssetState::Deleted => "DELETED",
            }
        }
    }
}
///  A time window specified by its `start_time` and `end_time`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TimeWindow {
    ///  Start time of the time window (exclusive).
    #[prost(message, optional, tag="1")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    ///  End time of the time window (inclusive). If not specified, the current
    ///  timestamp is used instead.
    #[prost(message, optional, tag="2")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
}
///  An asset in Google Cloud. An asset can be any resource in the Google Cloud
///  [resource
///  hierarchy](<https://cloud.google.com/resource-manager/docs/cloud-platform-resource-hierarchy>),
///  a resource outside the Google Cloud resource hierarchy (such as Google
///  Kubernetes Engine clusters and objects), or a policy (e.g. Cloud IAM policy),
///  or a relationship (e.g. an INSTANCE_TO_INSTANCEGROUP relationship).
///  See [Supported asset
///  types](<https://cloud.google.com/asset-inventory/docs/supported-asset-types>)
///  for more information.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Asset {
    ///  The last update timestamp of an asset. update_time is updated when
    ///  create/update/delete operation is performed.
    #[prost(message, optional, tag="11")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    ///  The full name of the asset. Example:
    ///  `//compute.googleapis.com/projects/my_project_123/zones/zone1/instances/instance1`
    ///
    ///  See [Resource
    ///  names](<https://cloud.google.com/apis/design/resource_names#full_resource_name>)
    ///  for more information.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    ///  The type of the asset. Example: `compute.googleapis.com/Disk`
    ///
    ///  See [Supported asset
    ///  types](<https://cloud.google.com/asset-inventory/docs/supported-asset-types>)
    ///  for more information.
    #[prost(string, tag="2")]
    pub asset_type: ::prost::alloc::string::String,
    ///  A representation of the resource.
    #[prost(message, optional, tag="3")]
    pub resource: ::core::option::Option<Resource>,
    ///  A representation of the Cloud IAM policy set on a Google Cloud resource.
    ///  There can be a maximum of one Cloud IAM policy set on any given resource.
    ///  In addition, Cloud IAM policies inherit their granted access scope from any
    ///  policies set on parent resources in the resource hierarchy. Therefore, the
    ///  effectively policy is the union of both the policy set on this resource
    ///  and each policy set on all of the resource's ancestry resource levels in
    ///  the hierarchy. See
    ///  [this topic](<https://cloud.google.com/iam/help/allow-policies/inheritance>)
    ///  for more information.
    #[prost(message, optional, tag="4")]
    pub iam_policy: ::core::option::Option<super::super::super::iam::v1::Policy>,
    ///  A representation of an [organization
    ///  policy](<https://cloud.google.com/resource-manager/docs/organization-policy/overview#organization_policy>).
    ///  There can be more than one organization policy with different constraints
    ///  set on a given resource.
    #[prost(message, repeated, tag="6")]
    pub org_policy: ::prost::alloc::vec::Vec<super::super::orgpolicy::v1::Policy>,
    ///  A representation of runtime OS Inventory information. See [this
    ///  topic](<https://cloud.google.com/compute/docs/instances/os-inventory-management>)
    ///  for more information.
    #[prost(message, optional, tag="12")]
    pub os_inventory: ::core::option::Option<super::super::osconfig::v1::Inventory>,
    ///  DEPRECATED. This field only presents for the purpose of
    ///  backward-compatibility. The server will never generate responses with this
    ///  field.
    ///  The related assets of the asset of one relationship type. One asset
    ///  only represents one type of relationship.
    #[deprecated]
    #[prost(message, optional, tag="13")]
    pub related_assets: ::core::option::Option<RelatedAssets>,
    ///  One related asset of the current asset.
    #[prost(message, optional, tag="15")]
    pub related_asset: ::core::option::Option<RelatedAsset>,
    ///  The ancestry path of an asset in Google Cloud [resource
    ///  hierarchy](<https://cloud.google.com/resource-manager/docs/cloud-platform-resource-hierarchy>),
    ///  represented as a list of relative resource names. An ancestry path starts
    ///  with the closest ancestor in the hierarchy and ends at root. If the asset
    ///  is a project, folder, or organization, the ancestry path starts from the
    ///  asset itself.
    ///
    ///  Example: `["projects/123456789", "folders/5432", "organizations/1234"]`
    #[prost(string, repeated, tag="10")]
    pub ancestors: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    ///  A representation of an [access
    ///  policy](<https://cloud.google.com/access-context-manager/docs/overview#access-policies>).
    #[prost(oneof="asset::AccessContextPolicy", tags="7, 8, 9")]
    pub access_context_policy: ::core::option::Option<asset::AccessContextPolicy>,
}
/// Nested message and enum types in `Asset`.
pub mod asset {
    ///  A representation of an [access
    ///  policy](<https://cloud.google.com/access-context-manager/docs/overview#access-policies>).
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum AccessContextPolicy {
        ///  Please also refer to the [access policy user
        ///  guide](<https://cloud.google.com/access-context-manager/docs/overview#access-policies>).
        #[prost(message, tag="7")]
        AccessPolicy(super::super::super::super::identity::accesscontextmanager::v1::AccessPolicy),
        ///  Please also refer to the [access level user
        ///  guide](<https://cloud.google.com/access-context-manager/docs/overview#access-levels>).
        #[prost(message, tag="8")]
        AccessLevel(super::super::super::super::identity::accesscontextmanager::v1::AccessLevel),
        ///  Please also refer to the [service perimeter user
        ///  guide](<https://cloud.google.com/vpc-service-controls/docs/overview>).
        #[prost(message, tag="9")]
        ServicePerimeter(super::super::super::super::identity::accesscontextmanager::v1::ServicePerimeter),
    }
}
///  A representation of a Google Cloud resource.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Resource {
    ///  The API version. Example: `v1`
    #[prost(string, tag="1")]
    pub version: ::prost::alloc::string::String,
    ///  The URL of the discovery document containing the resource's JSON schema.
    ///  Example:
    ///  `<https://www.googleapis.com/discovery/v1/apis/compute/v1/rest`>
    ///
    ///  This value is unspecified for resources that do not have an API based on a
    ///  discovery document, such as Cloud Bigtable.
    #[prost(string, tag="2")]
    pub discovery_document_uri: ::prost::alloc::string::String,
    ///  The JSON schema name listed in the discovery document. Example:
    ///  `Project`
    ///
    ///  This value is unspecified for resources that do not have an API based on a
    ///  discovery document, such as Cloud Bigtable.
    #[prost(string, tag="3")]
    pub discovery_name: ::prost::alloc::string::String,
    ///  The REST URL for accessing the resource. An HTTP `GET` request using this
    ///  URL returns the resource itself. Example:
    ///  `<https://cloudresourcemanager.googleapis.com/v1/projects/my-project-123`>
    ///
    ///  This value is unspecified for resources without a REST API.
    #[prost(string, tag="4")]
    pub resource_url: ::prost::alloc::string::String,
    ///  The full name of the immediate parent of this resource. See
    ///  [Resource
    ///  Names](<https://cloud.google.com/apis/design/resource_names#full_resource_name>)
    ///  for more information.
    ///
    ///  For Google Cloud assets, this value is the parent resource defined in the
    ///  [Cloud IAM policy
    ///  hierarchy](<https://cloud.google.com/iam/docs/overview#policy_hierarchy>).
    ///  Example:
    ///  `//cloudresourcemanager.googleapis.com/projects/my_project_123`
    ///
    ///  For third-party assets, this field may be set differently.
    #[prost(string, tag="5")]
    pub parent: ::prost::alloc::string::String,
    ///  The content of the resource, in which some sensitive fields are removed
    ///  and may not be present.
    #[prost(message, optional, tag="6")]
    pub data: ::core::option::Option<::prost_types::Struct>,
    ///  The location of the resource in Google Cloud, such as its zone and region.
    ///  For more information, see <https://cloud.google.com/about/locations/.>
    #[prost(string, tag="8")]
    pub location: ::prost::alloc::string::String,
}
///  DEPRECATED. This message only presents for the purpose of
///  backward-compatibility. The server will never populate this message in
///  responses.
///  The detailed related assets with the `relationship_type`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RelatedAssets {
    ///  The detailed relationship attributes.
    #[prost(message, optional, tag="1")]
    pub relationship_attributes: ::core::option::Option<RelationshipAttributes>,
    ///  The peer resources of the relationship.
    #[prost(message, repeated, tag="2")]
    pub assets: ::prost::alloc::vec::Vec<RelatedAsset>,
}
///  DEPRECATED. This message only presents for the purpose of
///  backward-compatibility. The server will never populate this message in
///  responses.
///  The relationship attributes which include  `type`, `source_resource_type`,
///  `target_resource_type` and `action`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RelationshipAttributes {
    ///  The unique identifier of the relationship type. Example:
    ///  `INSTANCE_TO_INSTANCEGROUP`
    #[prost(string, tag="4")]
    pub r#type: ::prost::alloc::string::String,
    ///  The source asset type. Example: `compute.googleapis.com/Instance`
    #[prost(string, tag="1")]
    pub source_resource_type: ::prost::alloc::string::String,
    ///  The target asset type. Example: `compute.googleapis.com/Disk`
    #[prost(string, tag="2")]
    pub target_resource_type: ::prost::alloc::string::String,
    ///  The detail of the relationship, e.g. `contains`, `attaches`
    #[prost(string, tag="3")]
    pub action: ::prost::alloc::string::String,
}
///  An asset identifier in Google Cloud which contains its name, type and
///  ancestors. An asset can be any resource in the Google Cloud [resource
///  hierarchy](<https://cloud.google.com/resource-manager/docs/cloud-platform-resource-hierarchy>),
///  a resource outside the Google Cloud resource hierarchy (such as Google
///  Kubernetes Engine clusters and objects), or a policy (e.g. Cloud IAM policy).
///  See [Supported asset
///  types](<https://cloud.google.com/asset-inventory/docs/supported-asset-types>)
///  for more information.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RelatedAsset {
    ///  The full name of the asset. Example:
    ///  `//compute.googleapis.com/projects/my_project_123/zones/zone1/instances/instance1`
    ///
    ///  See [Resource
    ///  names](<https://cloud.google.com/apis/design/resource_names#full_resource_name>)
    ///  for more information.
    #[prost(string, tag="1")]
    pub asset: ::prost::alloc::string::String,
    ///  The type of the asset. Example: `compute.googleapis.com/Disk`
    ///
    ///  See [Supported asset
    ///  types](<https://cloud.google.com/asset-inventory/docs/supported-asset-types>)
    ///  for more information.
    #[prost(string, tag="2")]
    pub asset_type: ::prost::alloc::string::String,
    ///  The ancestors of an asset in Google Cloud [resource
    ///  hierarchy](<https://cloud.google.com/resource-manager/docs/cloud-platform-resource-hierarchy>),
    ///  represented as a list of relative resource names. An ancestry path starts
    ///  with the closest ancestor in the hierarchy and ends at root.
    ///
    ///  Example: `["projects/123456789", "folders/5432", "organizations/1234"]`
    #[prost(string, repeated, tag="3")]
    pub ancestors: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    ///  The unique identifier of the relationship type. Example:
    ///  `INSTANCE_TO_INSTANCEGROUP`
    #[prost(string, tag="4")]
    pub relationship_type: ::prost::alloc::string::String,
}
///  A result of Resource Search, containing information of a cloud resource.
///  Next ID: 29
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResourceSearchResult {
    ///  The full resource name of this resource. Example:
    ///  `//compute.googleapis.com/projects/my_project_123/zones/zone1/instances/instance1`.
    ///  See [Cloud Asset Inventory Resource Name
    ///  Format](<https://cloud.google.com/asset-inventory/docs/resource-name-format>)
    ///  for more information.
    ///
    ///  To search against the `name`:
    ///
    ///  * use a field query. Example: `name:instance1`
    ///  * use a free text query. Example: `instance1`
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    ///  The type of this resource. Example: `compute.googleapis.com/Disk`.
    ///
    ///  To search against the `asset_type`:
    ///
    ///  * specify the `asset_type` field in your search request.
    #[prost(string, tag="2")]
    pub asset_type: ::prost::alloc::string::String,
    ///  The project that this resource belongs to, in the form of
    ///  projects/{PROJECT_NUMBER}. This field is available when the resource
    ///  belongs to a project.
    ///
    ///  To search against `project`:
    ///
    ///  * use a field query. Example: `project:12345`
    ///  * use a free text query. Example: `12345`
    ///  * specify the `scope` field as this project in your search request.
    #[prost(string, tag="3")]
    pub project: ::prost::alloc::string::String,
    ///  The folder(s) that this resource belongs to, in the form of
    ///  folders/{FOLDER_NUMBER}. This field is available when the resource
    ///  belongs to one or more folders.
    ///
    ///  To search against `folders`:
    ///
    ///  * use a field query. Example: `folders:(123 OR 456)`
    ///  * use a free text query. Example: `123`
    ///  * specify the `scope` field as this folder in your search request.
    #[prost(string, repeated, tag="17")]
    pub folders: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    ///  The organization that this resource belongs to, in the form of
    ///  organizations/{ORGANIZATION_NUMBER}. This field is available when the
    ///  resource belongs to an organization.
    ///
    ///  To search against `organization`:
    ///
    ///  * use a field query. Example: `organization:123`
    ///  * use a free text query. Example: `123`
    ///  * specify the `scope` field as this organization in your search request.
    #[prost(string, tag="18")]
    pub organization: ::prost::alloc::string::String,
    ///  The display name of this resource. This field is available only when the
    ///  resource's Protobuf contains it.
    ///
    ///  To search against the `display_name`:
    ///
    ///  * use a field query. Example: `displayName:"My Instance"`
    ///  * use a free text query. Example: `"My Instance"`
    #[prost(string, tag="4")]
    pub display_name: ::prost::alloc::string::String,
    ///  One or more paragraphs of text description of this resource. Maximum length
    ///  could be up to 1M bytes. This field is available only when the resource's
    ///  Protobuf contains it.
    ///
    ///  To search against the `description`:
    ///
    ///  * use a field query. Example: `description:"important instance"`
    ///  * use a free text query. Example: `"important instance"`
    #[prost(string, tag="5")]
    pub description: ::prost::alloc::string::String,
    ///  Location can be `global`, regional like `us-east1`, or zonal like
    ///  `us-west1-b`. This field is available only when the resource's Protobuf
    ///  contains it.
    ///
    ///  To search against the `location`:
    ///
    ///  * use a field query. Example: `location:us-west*`
    ///  * use a free text query. Example: `us-west*`
    #[prost(string, tag="6")]
    pub location: ::prost::alloc::string::String,
    ///  Labels associated with this resource. See [Labelling and grouping GCP
    ///  resources](<https://cloud.google.com/blog/products/gcp/labelling-and-grouping-your-google-cloud-platform-resources>)
    ///  for more information. This field is available only when the resource's
    ///  Protobuf contains it.
    ///
    ///  To search against the `labels`:
    ///
    ///  * use a field query:
    ///      - query on any label's key or value. Example: `labels:prod`
    ///      - query by a given label. Example: `labels.env:prod`
    ///      - query by a given label's existence. Example: `labels.env:*`
    ///  * use a free text query. Example: `prod`
    #[prost(map="string, string", tag="7")]
    pub labels: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    ///  Network tags associated with this resource. Like labels, network tags are a
    ///  type of annotations used to group GCP resources. See [Labelling GCP
    ///  resources](<https://cloud.google.com/blog/products/gcp/labelling-and-grouping-your-google-cloud-platform-resources>)
    ///  for more information. This field is available only when the resource's
    ///  Protobuf contains it.
    ///
    ///  To search against the `network_tags`:
    ///
    ///  * use a field query. Example: `networkTags:internal`
    ///  * use a free text query. Example: `internal`
    #[prost(string, repeated, tag="8")]
    pub network_tags: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    ///  The Cloud KMS
    ///  \[CryptoKey\](<https://cloud.google.com/kms/docs/reference/rest/v1/projects.locations.keyRings.cryptoKeys>)
    ///  name or
    ///  \[CryptoKeyVersion\](<https://cloud.google.com/kms/docs/reference/rest/v1/projects.locations.keyRings.cryptoKeys.cryptoKeyVersions>)
    ///  name. This field is available only when the resource's Protobuf contains
    ///  it.
    ///
    ///  To search against the `kms_key`:
    ///
    ///  * use a field query. Example: `kmsKey:key`
    ///  * use a free text query. Example: `key`
    #[prost(string, tag="10")]
    pub kms_key: ::prost::alloc::string::String,
    ///  The create timestamp of this resource, at which the resource was created.
    ///  The granularity is in seconds. Timestamp.nanos will always be 0. This field
    ///  is available only when the resource's Protobuf contains it.
    ///
    ///  To search against `create_time`:
    ///
    ///  * use a field query.
    ///      - value in seconds since unix epoch. Example: `createTime > 1609459200`
    ///      - value in date string. Example: `createTime > 2021-01-01`
    ///      - value in date-time string (must be quoted). Example: `createTime >
    ///      "2021-01-01T00:00:00"`
    #[prost(message, optional, tag="11")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    ///  The last update timestamp of this resource, at which the resource was last
    ///  modified or deleted. The granularity is in seconds. Timestamp.nanos will
    ///  always be 0. This field is available only when the resource's Protobuf
    ///  contains it.
    ///
    ///  To search against `update_time`:
    ///
    ///  * use a field query.
    ///      - value in seconds since unix epoch. Example: `updateTime < 1609459200`
    ///      - value in date string. Example: `updateTime < 2021-01-01`
    ///      - value in date-time string (must be quoted). Example: `updateTime <
    ///      "2021-01-01T00:00:00"`
    #[prost(message, optional, tag="12")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    ///  The state of this resource. Different resources types have different state
    ///  definitions that are mapped from various fields of different resource
    ///  types. This field is available only when the resource's Protobuf contains
    ///  it.
    ///
    ///  Example:
    ///  If the resource is an instance provided by Compute Engine,
    ///  its state will include PROVISIONING, STAGING, RUNNING, STOPPING,
    ///  SUSPENDING, SUSPENDED, REPAIRING, and TERMINATED. See `status` definition
    ///  in [API
    ///  Reference](<https://cloud.google.com/compute/docs/reference/rest/v1/instances>).
    ///  If the resource is a project provided by Cloud Resource Manager, its state
    ///  will include LIFECYCLE_STATE_UNSPECIFIED, ACTIVE, DELETE_REQUESTED and
    ///  DELETE_IN_PROGRESS. See `lifecycleState` definition in [API
    ///  Reference](<https://cloud.google.com/resource-manager/reference/rest/v1/projects>).
    ///
    ///  To search against the `state`:
    ///
    ///  * use a field query. Example: `state:RUNNING`
    ///  * use a free text query. Example: `RUNNING`
    #[prost(string, tag="13")]
    pub state: ::prost::alloc::string::String,
    ///  The additional searchable attributes of this resource. The attributes may
    ///  vary from one resource type to another. Examples: `projectId` for Project,
    ///  `dnsName` for DNS ManagedZone. This field contains a subset of the resource
    ///  metadata fields that are returned by the List or Get APIs provided by the
    ///  corresponding GCP service (e.g., Compute Engine). see [API references and
    ///  supported searchable
    ///  attributes](<https://cloud.google.com/asset-inventory/docs/supported-asset-types#searchable_asset_types>)
    ///  to see which fields are included.
    ///
    ///  You can search values of these fields through free text search. However,
    ///  you should not consume the field programically as the field names and
    ///  values may change as the GCP service updates to a new incompatible API
    ///  version.
    ///
    ///  To search against the `additional_attributes`:
    ///
    ///  * use a free text query to match the attributes values. Example: to search
    ///    `additional_attributes = { dnsName: "foobar" }`, you can issue a query
    ///    `foobar`.
    #[prost(message, optional, tag="9")]
    pub additional_attributes: ::core::option::Option<::prost_types::Struct>,
    ///  The full resource name of this resource's parent, if it has one.
    ///  To search against the `parent_full_resource_name`:
    ///
    ///  * use a field query. Example:
    ///  `parentFullResourceName:"project-name"`
    ///  * use a free text query. Example:
    ///  `project-name`
    #[prost(string, tag="19")]
    pub parent_full_resource_name: ::prost::alloc::string::String,
    ///  Versioned resource representations of this resource. This is repeated
    ///  because there could be multiple versions of resource representations during
    ///  version migration.
    ///
    ///  This `versioned_resources` field is not searchable. Some attributes of the
    ///  resource representations are exposed in `additional_attributes` field, so
    ///  as to allow users to search on them.
    #[prost(message, repeated, tag="16")]
    pub versioned_resources: ::prost::alloc::vec::Vec<VersionedResource>,
    ///  Attached resources of this resource. For example, an OSConfig
    ///  Inventory is an attached resource of a Compute Instance. This field is
    ///  repeated because a resource could have multiple attached resources.
    ///
    ///  This `attached_resources` field is not searchable. Some attributes
    ///  of the attached resources are exposed in `additional_attributes` field, so
    ///  as to allow users to search on them.
    #[prost(message, repeated, tag="20")]
    pub attached_resources: ::prost::alloc::vec::Vec<AttachedResource>,
    ///  A map of related resources of this resource, keyed by the
    ///  relationship type. A relationship type is in the format of
    ///  {SourceType}_{ACTION}_{DestType}. Example: `DISK_TO_INSTANCE`,
    ///  `DISK_TO_NETWORK`, `INSTANCE_TO_INSTANCEGROUP`.
    ///  See [supported relationship
    ///  types](<https://cloud.google.com/asset-inventory/docs/supported-asset-types#supported_relationship_types>).
    #[prost(map="string, message", tag="21")]
    pub relationships: ::std::collections::HashMap<::prost::alloc::string::String, RelatedResources>,
    ///  TagKey namespaced names, in the format of {ORG_ID}/{TAG_KEY_SHORT_NAME}.
    ///  To search against the `tagKeys`:
    ///
    ///  * use a field query. Example:
    ///      - `tagKeys:"123456789/env*"`
    ///      - `tagKeys="123456789/env"`
    ///      - `tagKeys:"env"`
    ///
    ///  * use a free text query. Example:
    ///      - `env`
    #[prost(string, repeated, tag="23")]
    pub tag_keys: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    ///  TagValue namespaced names, in the format of
    ///  {ORG_ID}/{TAG_KEY_SHORT_NAME}/{TAG_VALUE_SHORT_NAME}.
    ///  To search against the `tagValues`:
    ///
    ///  * use a field query. Example:
    ///      - `tagValues:"env"`
    ///      - `tagValues:"env/prod"`
    ///      - `tagValues:"123456789/env/prod*"`
    ///      - `tagValues="123456789/env/prod"`
    ///
    ///  * use a free text query. Example:
    ///      - `prod`
    #[prost(string, repeated, tag="25")]
    pub tag_values: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    ///  TagValue IDs, in the format of tagValues/{TAG_VALUE_ID}.
    ///  To search against the `tagValueIds`:
    ///
    ///  * use a field query. Example:
    ///      - `tagValueIds:"456"`
    ///      - `tagValueIds="tagValues/456"`
    ///
    ///  * use a free text query. Example:
    ///      - `456`
    #[prost(string, repeated, tag="26")]
    pub tag_value_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    ///  The type of this resource's immediate parent, if there is one.
    ///
    ///  To search against the `parent_asset_type`:
    ///
    ///  * use a field query. Example:
    ///  `parentAssetType:"cloudresourcemanager.googleapis.com/Project"`
    ///  * use a free text query. Example:
    ///  `cloudresourcemanager.googleapis.com/Project`
    #[prost(string, tag="103")]
    pub parent_asset_type: ::prost::alloc::string::String,
}
///  Resource representation as defined by the corresponding service providing the
///  resource for a given API version.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VersionedResource {
    ///  API version of the resource.
    ///
    ///  Example:
    ///  If the resource is an instance provided by Compute Engine v1 API as defined
    ///  in `<https://cloud.google.com/compute/docs/reference/rest/v1/instances`,>
    ///  version will be "v1".
    #[prost(string, tag="1")]
    pub version: ::prost::alloc::string::String,
    ///  JSON representation of the resource as defined by the corresponding
    ///  service providing this resource.
    ///
    ///  Example:
    ///  If the resource is an instance provided by Compute Engine, this field will
    ///  contain the JSON representation of the instance as defined by Compute
    ///  Engine:
    ///  `<https://cloud.google.com/compute/docs/reference/rest/v1/instances`.>
    ///
    ///  You can find the resource definition for each supported resource type in
    ///  this table:
    ///  `<https://cloud.google.com/asset-inventory/docs/supported-asset-types#searchable_asset_types`>
    #[prost(message, optional, tag="2")]
    pub resource: ::core::option::Option<::prost_types::Struct>,
}
///  Attached resource representation, which is defined by the corresponding
///  service provider. It represents an attached resource's payload.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AttachedResource {
    ///  The type of this attached resource.
    ///
    ///  Example: `osconfig.googleapis.com/Inventory`
    ///
    ///  You can find the supported attached asset types of each resource in this
    ///  table:
    ///  `<https://cloud.google.com/asset-inventory/docs/supported-asset-types#searchable_asset_types`>
    #[prost(string, tag="1")]
    pub asset_type: ::prost::alloc::string::String,
    ///  Versioned resource representations of this attached resource. This is
    ///  repeated because there could be multiple versions of the attached resource
    ///  representations during version migration.
    #[prost(message, repeated, tag="3")]
    pub versioned_resources: ::prost::alloc::vec::Vec<VersionedResource>,
}
///  The related resources of the primary resource.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RelatedResources {
    ///  The detailed related resources of the primary resource.
    #[prost(message, repeated, tag="1")]
    pub related_resources: ::prost::alloc::vec::Vec<RelatedResource>,
}
///  The detailed related resource.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RelatedResource {
    ///  The type of the asset. Example: `compute.googleapis.com/Instance`
    #[prost(string, tag="1")]
    pub asset_type: ::prost::alloc::string::String,
    ///  The full resource name of the related resource. Example:
    ///  `//compute.googleapis.com/projects/my_proj_123/zones/instance/instance123`
    #[prost(string, tag="2")]
    pub full_resource_name: ::prost::alloc::string::String,
}
///  A result of IAM Policy search, containing information of an IAM policy.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IamPolicySearchResult {
    ///  The full resource name of the resource associated with this IAM policy.
    ///  Example:
    ///  `//compute.googleapis.com/projects/my_project_123/zones/zone1/instances/instance1`.
    ///  See [Cloud Asset Inventory Resource Name
    ///  Format](<https://cloud.google.com/asset-inventory/docs/resource-name-format>)
    ///  for more information.
    ///
    ///  To search against the `resource`:
    ///
    ///  * use a field query. Example: `resource:organizations/123`
    #[prost(string, tag="1")]
    pub resource: ::prost::alloc::string::String,
    ///  The type of the resource associated with this IAM policy. Example:
    ///  `compute.googleapis.com/Disk`.
    ///
    ///  To search against the `asset_type`:
    ///
    ///  * specify the `asset_types` field in your search request.
    #[prost(string, tag="5")]
    pub asset_type: ::prost::alloc::string::String,
    ///  The project that the associated GCP resource belongs to, in the form of
    ///  projects/{PROJECT_NUMBER}. If an IAM policy is set on a resource (like VM
    ///  instance, Cloud Storage bucket), the project field will indicate the
    ///  project that contains the resource. If an IAM policy is set on a folder or
    ///  orgnization, this field will be empty.
    ///
    ///  To search against the `project`:
    ///
    ///  * specify the `scope` field as this project in your search request.
    #[prost(string, tag="2")]
    pub project: ::prost::alloc::string::String,
    ///  The folder(s) that the IAM policy belongs to, in the form of
    ///  folders/{FOLDER_NUMBER}. This field is available when the IAM policy
    ///  belongs to one or more folders.
    ///
    ///  To search against `folders`:
    ///
    ///  * use a field query. Example: `folders:(123 OR 456)`
    ///  * use a free text query. Example: `123`
    ///  * specify the `scope` field as this folder in your search request.
    #[prost(string, repeated, tag="6")]
    pub folders: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    ///  The organization that the IAM policy belongs to, in the form
    ///  of organizations/{ORGANIZATION_NUMBER}. This field is available when the
    ///  IAM policy belongs to an organization.
    ///
    ///  To search against `organization`:
    ///
    ///  * use a field query. Example: `organization:123`
    ///  * use a free text query. Example: `123`
    ///  * specify the `scope` field as this organization in your search request.
    #[prost(string, tag="7")]
    pub organization: ::prost::alloc::string::String,
    ///  The IAM policy directly set on the given resource. Note that the original
    ///  IAM policy can contain multiple bindings. This only contains the bindings
    ///  that match the given query. For queries that don't contain a constrain on
    ///  policies (e.g., an empty query), this contains all the bindings.
    ///
    ///  To search against the `policy` bindings:
    ///
    ///  * use a field query:
    ///      - query by the policy contained members. Example:
    ///        `policy:amy@gmail.com`
    ///      - query by the policy contained roles. Example:
    ///        `policy:roles/compute.admin`
    ///      - query by the policy contained roles' included permissions. Example:
    ///        `policy.role.permissions:compute.instances.create`
    #[prost(message, optional, tag="3")]
    pub policy: ::core::option::Option<super::super::super::iam::v1::Policy>,
    ///  Explanation about the IAM policy search result. It contains additional
    ///  information to explain why the search result matches the query.
    #[prost(message, optional, tag="4")]
    pub explanation: ::core::option::Option<iam_policy_search_result::Explanation>,
}
/// Nested message and enum types in `IamPolicySearchResult`.
pub mod iam_policy_search_result {
    ///  Explanation about the IAM policy search result.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Explanation {
        ///  The map from roles to their included permissions that match the
        ///  permission query (i.e., a query containing `policy.role.permissions:`).
        ///  Example: if query `policy.role.permissions:compute.disk.get`
        ///  matches a policy binding that contains owner role, the
        ///  matched_permissions will be `{"roles/owner": \["compute.disk.get"\]}`. The
        ///  roles can also be found in the returned `policy` bindings. Note that the
        ///  map is populated only for requests with permission queries.
        #[prost(map="string, message", tag="1")]
        pub matched_permissions: ::std::collections::HashMap<::prost::alloc::string::String, explanation::Permissions>,
    }
    /// Nested message and enum types in `Explanation`.
    pub mod explanation {
        ///  IAM permissions
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Permissions {
            ///  A list of permissions. A sample permission string: `compute.disk.get`.
            #[prost(string, repeated, tag="1")]
            pub permissions: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        }
    }
}
///  Represents the detailed state of an entity under analysis, such as a
///  resource, an identity or an access.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IamPolicyAnalysisState {
    ///  The Google standard error code that best describes the state.
    ///  For example:
    ///  - OK means the analysis on this entity has been successfully finished;
    ///  - PERMISSION_DENIED means an access denied error is encountered;
    ///  - DEADLINE_EXCEEDED means the analysis on this entity hasn't been started
    ///  in time;
    #[prost(enumeration="super::super::super::rpc::Code", tag="1")]
    pub code: i32,
    ///  The human-readable description of the cause of failure.
    #[prost(string, tag="2")]
    pub cause: ::prost::alloc::string::String,
}
///  The Condition evaluation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConditionEvaluation {
    ///  The evaluation result.
    #[prost(enumeration="condition_evaluation::EvaluationValue", tag="1")]
    pub evaluation_value: i32,
}
/// Nested message and enum types in `ConditionEvaluation`.
pub mod condition_evaluation {
    ///  Value of this expression.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum EvaluationValue {
        ///  Reserved for future use.
        Unspecified = 0,
        ///  The evaluation result is `true`.
        True = 1,
        ///  The evaluation result is `false`.
        False = 2,
        ///  The evaluation result is `conditional` when the condition expression
        ///  contains variables that are either missing input values or have not been
        ///  supported by Analyzer yet.
        Conditional = 3,
    }
    impl EvaluationValue {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                EvaluationValue::Unspecified => "EVALUATION_VALUE_UNSPECIFIED",
                EvaluationValue::True => "TRUE",
                EvaluationValue::False => "FALSE",
                EvaluationValue::Conditional => "CONDITIONAL",
            }
        }
    }
}
///  IAM Policy analysis result, consisting of one IAM policy binding and derived
///  access control lists.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IamPolicyAnalysisResult {
    ///  The [full resource
    ///  name](<https://cloud.google.com/asset-inventory/docs/resource-name-format>)
    ///  of the resource to which the
    ///  \[iam_binding][google.cloud.asset.v1.IamPolicyAnalysisResult.iam_binding\]
    ///  policy attaches.
    #[prost(string, tag="1")]
    pub attached_resource_full_name: ::prost::alloc::string::String,
    ///  The Cloud IAM policy binding under analysis.
    #[prost(message, optional, tag="2")]
    pub iam_binding: ::core::option::Option<super::super::super::iam::v1::Binding>,
    ///  The access control lists derived from the
    ///  \[iam_binding][google.cloud.asset.v1.IamPolicyAnalysisResult.iam_binding\]
    ///  that match or potentially match resource and access selectors specified in
    ///  the request.
    #[prost(message, repeated, tag="3")]
    pub access_control_lists: ::prost::alloc::vec::Vec<iam_policy_analysis_result::AccessControlList>,
    ///  The identity list derived from members of the
    ///  \[iam_binding][google.cloud.asset.v1.IamPolicyAnalysisResult.iam_binding\]
    ///  that match or potentially match identity selector specified in the request.
    #[prost(message, optional, tag="4")]
    pub identity_list: ::core::option::Option<iam_policy_analysis_result::IdentityList>,
    ///  Represents whether all analyses on the
    ///  \[iam_binding][google.cloud.asset.v1.IamPolicyAnalysisResult.iam_binding\]
    ///  have successfully finished.
    #[prost(bool, tag="5")]
    pub fully_explored: bool,
}
/// Nested message and enum types in `IamPolicyAnalysisResult`.
pub mod iam_policy_analysis_result {
    ///  A Google Cloud resource under analysis.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Resource {
        ///  The [full resource
        ///  name](<https://cloud.google.com/asset-inventory/docs/resource-name-format>)
        #[prost(string, tag="1")]
        pub full_resource_name: ::prost::alloc::string::String,
        ///  The analysis state of this resource.
        #[prost(message, optional, tag="2")]
        pub analysis_state: ::core::option::Option<super::IamPolicyAnalysisState>,
    }
    ///  An IAM role or permission under analysis.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Access {
        ///  The analysis state of this access.
        #[prost(message, optional, tag="3")]
        pub analysis_state: ::core::option::Option<super::IamPolicyAnalysisState>,
        #[prost(oneof="access::OneofAccess", tags="1, 2")]
        pub oneof_access: ::core::option::Option<access::OneofAccess>,
    }
    /// Nested message and enum types in `Access`.
    pub mod access {
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum OneofAccess {
            ///  The role.
            #[prost(string, tag="1")]
            Role(::prost::alloc::string::String),
            ///  The permission.
            #[prost(string, tag="2")]
            Permission(::prost::alloc::string::String),
        }
    }
    ///  An identity under analysis.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Identity {
        ///  The identity name in any form of members appear in
        ///  [IAM policy
        ///  binding](<https://cloud.google.com/iam/reference/rest/v1/Binding>), such
        ///  as:
        ///  - user:foo@google.com
        ///  - group:group1@google.com
        ///  - serviceAccount:s1@prj1.iam.gserviceaccount.com
        ///  - projectOwner:some_project_id
        ///  - domain:google.com
        ///  - allUsers
        ///  - etc.
        #[prost(string, tag="1")]
        pub name: ::prost::alloc::string::String,
        ///  The analysis state of this identity.
        #[prost(message, optional, tag="2")]
        pub analysis_state: ::core::option::Option<super::IamPolicyAnalysisState>,
    }
    ///  A directional edge.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Edge {
        ///  The source node of the edge. For example, it could be a full resource
        ///  name for a resource node or an email of an identity.
        #[prost(string, tag="1")]
        pub source_node: ::prost::alloc::string::String,
        ///  The target node of the edge. For example, it could be a full resource
        ///  name for a resource node or an email of an identity.
        #[prost(string, tag="2")]
        pub target_node: ::prost::alloc::string::String,
    }
    ///  An access control list, derived from the above IAM policy binding, which
    ///  contains a set of resources and accesses. May include one
    ///  item from each set to compose an access control entry.
    ///
    ///  NOTICE that there could be multiple access control lists for one IAM policy
    ///  binding. The access control lists are created based on resource and access
    ///  combinations.
    ///
    ///  For example, assume we have the following cases in one IAM policy binding:
    ///  - Permission P1 and P2 apply to resource R1 and R2;
    ///  - Permission P3 applies to resource R2 and R3;
    ///
    ///  This will result in the following access control lists:
    ///  - AccessControlList 1: [R1, R2], [P1, P2]
    ///  - AccessControlList 2: [R2, R3], \[P3\]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct AccessControlList {
        ///  The resources that match one of the following conditions:
        ///  - The resource_selector, if it is specified in request;
        ///  - Otherwise, resources reachable from the policy attached resource.
        #[prost(message, repeated, tag="1")]
        pub resources: ::prost::alloc::vec::Vec<Resource>,
        ///  The accesses that match one of the following conditions:
        ///  - The access_selector, if it is specified in request;
        ///  - Otherwise, access specifiers reachable from the policy binding's role.
        #[prost(message, repeated, tag="2")]
        pub accesses: ::prost::alloc::vec::Vec<Access>,
        ///  Resource edges of the graph starting from the policy attached
        ///  resource to any descendant resources. The
        ///  \[Edge.source_node][google.cloud.asset.v1.IamPolicyAnalysisResult.Edge.source_node\]
        ///  contains the full resource name of a parent resource and
        ///  \[Edge.target_node][google.cloud.asset.v1.IamPolicyAnalysisResult.Edge.target_node\]
        ///  contains the full resource name of a child resource. This field is
        ///  present only if the output_resource_edges option is enabled in request.
        #[prost(message, repeated, tag="3")]
        pub resource_edges: ::prost::alloc::vec::Vec<Edge>,
        ///  Condition evaluation for this AccessControlList, if there is a condition
        ///  defined in the above IAM policy binding.
        #[prost(message, optional, tag="4")]
        pub condition_evaluation: ::core::option::Option<super::ConditionEvaluation>,
    }
    ///  The identities and group edges.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct IdentityList {
        ///  Only the identities that match one of the following conditions will be
        ///  presented:
        ///  - The identity_selector, if it is specified in request;
        ///  - Otherwise, identities reachable from the policy binding's members.
        #[prost(message, repeated, tag="1")]
        pub identities: ::prost::alloc::vec::Vec<Identity>,
        ///  Group identity edges of the graph starting from the binding's
        ///  group members to any node of the
        ///  \[identities][google.cloud.asset.v1.IamPolicyAnalysisResult.IdentityList.identities\].
        ///  The
        ///  \[Edge.source_node][google.cloud.asset.v1.IamPolicyAnalysisResult.Edge.source_node\]
        ///  contains a group, such as `group:parent@google.com`. The
        ///  \[Edge.target_node][google.cloud.asset.v1.IamPolicyAnalysisResult.Edge.target_node\]
        ///  contains a member of the group, such as `group:child@google.com` or
        ///  `user:foo@google.com`. This field is present only if the
        ///  output_group_edges option is enabled in request.
        #[prost(message, repeated, tag="2")]
        pub group_edges: ::prost::alloc::vec::Vec<Edge>,
    }
}
///  Represents the metadata of the longrunning operation for the
///  AnalyzeIamPolicyLongrunning rpc.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnalyzeIamPolicyLongrunningMetadata {
    ///  Output only. The time the operation was created.
    #[prost(message, optional, tag="1")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
}
///  Export asset request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportAssetsRequest {
    ///  Required. The relative name of the root asset. This can only be an
    ///  organization number (such as "organizations/123"), a project ID (such as
    ///  "projects/my-project-id"), or a project number (such as "projects/12345"),
    ///  or a folder number (such as "folders/123").
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    ///  Timestamp to take an asset snapshot. This can only be set to a timestamp
    ///  between the current time and the current time minus 35 days (inclusive).
    ///  If not specified, the current time will be used. Due to delays in resource
    ///  data collection and indexing, there is a volatile window during which
    ///  running the same query may get different results.
    #[prost(message, optional, tag="2")]
    pub read_time: ::core::option::Option<::prost_types::Timestamp>,
    ///  A list of asset types to take a snapshot for. For example:
    ///  "compute.googleapis.com/Disk".
    ///
    ///  Regular expressions are also supported. For example:
    ///
    ///  * "compute.googleapis.com.*" snapshots resources whose asset type starts
    ///  with "compute.googleapis.com".
    ///  * ".*Instance" snapshots resources whose asset type ends with "Instance".
    ///  * ".*Instance.*" snapshots resources whose asset type contains "Instance".
    ///
    ///  See \[RE2\](<https://github.com/google/re2/wiki/Syntax>) for all supported
    ///  regular expression syntax. If the regular expression does not match any
    ///  supported asset type, an INVALID_ARGUMENT error will be returned.
    ///
    ///  If specified, only matching assets will be returned, otherwise, it will
    ///  snapshot all asset types. See [Introduction to Cloud Asset
    ///  Inventory](<https://cloud.google.com/asset-inventory/docs/overview>)
    ///  for all supported asset types.
    #[prost(string, repeated, tag="3")]
    pub asset_types: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    ///  Asset content type. If not specified, no content but the asset name will be
    ///  returned.
    #[prost(enumeration="ContentType", tag="4")]
    pub content_type: i32,
    ///  Required. Output configuration indicating where the results will be output
    ///  to.
    #[prost(message, optional, tag="5")]
    pub output_config: ::core::option::Option<OutputConfig>,
    ///  A list of relationship types to export, for example:
    ///  `INSTANCE_TO_INSTANCEGROUP`. This field should only be specified if
    ///  content_type=RELATIONSHIP.
    ///  * If specified:
    ///  it snapshots specified relationships. It returns an error if
    ///  any of the \[relationship_types\] doesn't belong to the supported
    ///  relationship types of the \[asset_types\] or if any of the \[asset_types\]
    ///  doesn't belong to the source types of the \[relationship_types\].
    ///  * Otherwise:
    ///  it snapshots the supported relationships for all \[asset_types\] or returns
    ///  an error if any of the \[asset_types\] has no relationship support.
    ///  An unspecified asset types field means all supported asset_types.
    ///  See [Introduction to Cloud Asset
    ///  Inventory](<https://cloud.google.com/asset-inventory/docs/overview>) for all
    ///  supported asset types and relationship types.
    #[prost(string, repeated, tag="6")]
    pub relationship_types: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
///  The export asset response. This message is returned by the
///  \[google.longrunning.Operations.GetOperation][google.longrunning.Operations.GetOperation\]
///  method in the returned
///  \[google.longrunning.Operation.response][google.longrunning.Operation.response\]
///  field.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportAssetsResponse {
    ///  Time the snapshot was taken.
    #[prost(message, optional, tag="1")]
    pub read_time: ::core::option::Option<::prost_types::Timestamp>,
    ///  Output configuration indicating where the results were output to.
    #[prost(message, optional, tag="2")]
    pub output_config: ::core::option::Option<OutputConfig>,
    ///  Output result indicating where the assets were exported to. For example, a
    ///  set of actual Google Cloud Storage object uris where the assets are
    ///  exported to. The uris can be different from what \[output_config\] has
    ///  specified, as the service will split the output object into multiple ones
    ///  once it exceeds a single Google Cloud Storage object limit.
    #[prost(message, optional, tag="3")]
    pub output_result: ::core::option::Option<OutputResult>,
}
///  ListAssets request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAssetsRequest {
    ///  Required. Name of the organization, folder, or project the assets belong
    ///  to. Format: "organizations/\[organization-number\]" (such as
    ///  "organizations/123"), "projects/\[project-id\]" (such as
    ///  "projects/my-project-id"), "projects/\[project-number\]" (such as
    ///  "projects/12345"), or "folders/\[folder-number\]" (such as "folders/12345").
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    ///  Timestamp to take an asset snapshot. This can only be set to a timestamp
    ///  between the current time and the current time minus 35 days (inclusive).
    ///  If not specified, the current time will be used. Due to delays in resource
    ///  data collection and indexing, there is a volatile window during which
    ///  running the same query may get different results.
    #[prost(message, optional, tag="2")]
    pub read_time: ::core::option::Option<::prost_types::Timestamp>,
    ///  A list of asset types to take a snapshot for. For example:
    ///  "compute.googleapis.com/Disk".
    ///
    ///  Regular expression is also supported. For example:
    ///
    ///  * "compute.googleapis.com.*" snapshots resources whose asset type starts
    ///  with "compute.googleapis.com".
    ///  * ".*Instance" snapshots resources whose asset type ends with "Instance".
    ///  * ".*Instance.*" snapshots resources whose asset type contains "Instance".
    ///
    ///  See \[RE2\](<https://github.com/google/re2/wiki/Syntax>) for all supported
    ///  regular expression syntax. If the regular expression does not match any
    ///  supported asset type, an INVALID_ARGUMENT error will be returned.
    ///
    ///  If specified, only matching assets will be returned, otherwise, it will
    ///  snapshot all asset types. See [Introduction to Cloud Asset
    ///  Inventory](<https://cloud.google.com/asset-inventory/docs/overview>)
    ///  for all supported asset types.
    #[prost(string, repeated, tag="3")]
    pub asset_types: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    ///  Asset content type. If not specified, no content but the asset name will
    ///  be returned.
    #[prost(enumeration="ContentType", tag="4")]
    pub content_type: i32,
    ///  The maximum number of assets to be returned in a single response. Default
    ///  is 100, minimum is 1, and maximum is 1000.
    #[prost(int32, tag="5")]
    pub page_size: i32,
    ///  The `next_page_token` returned from the previous `ListAssetsResponse`, or
    ///  unspecified for the first `ListAssetsRequest`. It is a continuation of a
    ///  prior `ListAssets` call, and the API should return the next page of assets.
    #[prost(string, tag="6")]
    pub page_token: ::prost::alloc::string::String,
    ///  A list of relationship types to output, for example:
    ///  `INSTANCE_TO_INSTANCEGROUP`. This field should only be specified if
    ///  content_type=RELATIONSHIP.
    ///  * If specified:
    ///  it snapshots specified relationships. It returns an error if
    ///  any of the \[relationship_types\] doesn't belong to the supported
    ///  relationship types of the \[asset_types\] or if any of the \[asset_types\]
    ///  doesn't belong to the source types of the \[relationship_types\].
    ///  * Otherwise:
    ///  it snapshots the supported relationships for all \[asset_types\] or returns
    ///  an error if any of the \[asset_types\] has no relationship support.
    ///  An unspecified asset types field means all supported asset_types.
    ///  See [Introduction to Cloud Asset
    ///  Inventory](<https://cloud.google.com/asset-inventory/docs/overview>)
    ///  for all supported asset types and relationship types.
    #[prost(string, repeated, tag="7")]
    pub relationship_types: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
///  ListAssets response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAssetsResponse {
    ///  Time the snapshot was taken.
    #[prost(message, optional, tag="1")]
    pub read_time: ::core::option::Option<::prost_types::Timestamp>,
    ///  Assets.
    #[prost(message, repeated, tag="2")]
    pub assets: ::prost::alloc::vec::Vec<Asset>,
    ///  Token to retrieve the next page of results. It expires 72 hours after the
    ///  page token for the first page is generated. Set to empty if there are no
    ///  remaining results.
    #[prost(string, tag="3")]
    pub next_page_token: ::prost::alloc::string::String,
}
///  Batch get assets history request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchGetAssetsHistoryRequest {
    ///  Required. The relative name of the root asset. It can only be an
    ///  organization number (such as "organizations/123"), a project ID (such as
    ///  "projects/my-project-id")", or a project number (such as "projects/12345").
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    ///  A list of the full names of the assets.
    ///  See: <https://cloud.google.com/asset-inventory/docs/resource-name-format>
    ///  Example:
    ///
    ///  `//compute.googleapis.com/projects/my_project_123/zones/zone1/instances/instance1`.
    ///
    ///  The request becomes a no-op if the asset name list is empty, and the max
    ///  size of the asset name list is 100 in one request.
    #[prost(string, repeated, tag="2")]
    pub asset_names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    ///  Optional. The content type.
    #[prost(enumeration="ContentType", tag="3")]
    pub content_type: i32,
    ///  Optional. The time window for the asset history. Both start_time and
    ///  end_time are optional and if set, it must be after the current time minus
    ///  35 days. If end_time is not set, it is default to current timestamp.
    ///  If start_time is not set, the snapshot of the assets at end_time will be
    ///  returned. The returned results contain all temporal assets whose time
    ///  window overlap with read_time_window.
    #[prost(message, optional, tag="4")]
    pub read_time_window: ::core::option::Option<TimeWindow>,
    ///  Optional. A list of relationship types to output, for example:
    ///  `INSTANCE_TO_INSTANCEGROUP`. This field should only be specified if
    ///  content_type=RELATIONSHIP.
    ///  * If specified:
    ///  it outputs specified relationships' history on the \[asset_names\]. It
    ///  returns an error if any of the \[relationship_types\] doesn't belong to the
    ///  supported relationship types of the \[asset_names\] or if any of the
    ///  \[asset_names\]'s types doesn't belong to the source types of the
    ///  \[relationship_types\].
    ///  * Otherwise:
    ///  it outputs the supported relationships' history on the \[asset_names\] or
    ///  returns an error if any of the \[asset_names\]'s types has no relationship
    ///  support.
    ///  See [Introduction to Cloud Asset
    ///  Inventory](<https://cloud.google.com/asset-inventory/docs/overview>) for all
    ///  supported asset types and relationship types.
    #[prost(string, repeated, tag="5")]
    pub relationship_types: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
///  Batch get assets history response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchGetAssetsHistoryResponse {
    ///  A list of assets with valid time windows.
    #[prost(message, repeated, tag="1")]
    pub assets: ::prost::alloc::vec::Vec<TemporalAsset>,
}
///  Create asset feed request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateFeedRequest {
    ///  Required. The name of the project/folder/organization where this feed
    ///  should be created in. It can only be an organization number (such as
    ///  "organizations/123"), a folder number (such as "folders/123"), a project ID
    ///  (such as "projects/my-project-id")", or a project number (such as
    ///  "projects/12345").
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    ///  Required. This is the client-assigned asset feed identifier and it needs to
    ///  be unique under a specific parent project/folder/organization.
    #[prost(string, tag="2")]
    pub feed_id: ::prost::alloc::string::String,
    ///  Required. The feed details. The field `name` must be empty and it will be
    ///  generated in the format of: projects/project_number/feeds/feed_id
    ///  folders/folder_number/feeds/feed_id
    ///  organizations/organization_number/feeds/feed_id
    #[prost(message, optional, tag="3")]
    pub feed: ::core::option::Option<Feed>,
}
///  Get asset feed request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetFeedRequest {
    ///  Required. The name of the Feed and it must be in the format of:
    ///  projects/project_number/feeds/feed_id
    ///  folders/folder_number/feeds/feed_id
    ///  organizations/organization_number/feeds/feed_id
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
///  List asset feeds request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListFeedsRequest {
    ///  Required. The parent project/folder/organization whose feeds are to be
    ///  listed. It can only be using project/folder/organization number (such as
    ///  "folders/12345")", or a project ID (such as "projects/my-project-id").
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListFeedsResponse {
    ///  A list of feeds.
    #[prost(message, repeated, tag="1")]
    pub feeds: ::prost::alloc::vec::Vec<Feed>,
}
///  Update asset feed request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateFeedRequest {
    ///  Required. The new values of feed details. It must match an existing feed
    ///  and the field `name` must be in the format of:
    ///  projects/project_number/feeds/feed_id or
    ///  folders/folder_number/feeds/feed_id or
    ///  organizations/organization_number/feeds/feed_id.
    #[prost(message, optional, tag="1")]
    pub feed: ::core::option::Option<Feed>,
    ///  Required. Only updates the `feed` fields indicated by this mask.
    ///  The field mask must not be empty, and it must not contain fields that
    ///  are immutable or only set by the server.
    #[prost(message, optional, tag="2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteFeedRequest {
    ///  Required. The name of the feed and it must be in the format of:
    ///  projects/project_number/feeds/feed_id
    ///  folders/folder_number/feeds/feed_id
    ///  organizations/organization_number/feeds/feed_id
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
///  Output configuration for export assets destination.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OutputConfig {
    ///  Asset export destination.
    #[prost(oneof="output_config::Destination", tags="1, 2")]
    pub destination: ::core::option::Option<output_config::Destination>,
}
/// Nested message and enum types in `OutputConfig`.
pub mod output_config {
    ///  Asset export destination.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Destination {
        ///  Destination on Cloud Storage.
        #[prost(message, tag="1")]
        GcsDestination(super::GcsDestination),
        ///  Destination on BigQuery. The output table stores the fields in asset
        ///  Protobuf as columns in BigQuery.
        #[prost(message, tag="2")]
        BigqueryDestination(super::BigQueryDestination),
    }
}
///  Output result of export assets.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OutputResult {
    ///  Asset export result.
    #[prost(oneof="output_result::Result", tags="1")]
    pub result: ::core::option::Option<output_result::Result>,
}
/// Nested message and enum types in `OutputResult`.
pub mod output_result {
    ///  Asset export result.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Result {
        ///  Export result on Cloud Storage.
        #[prost(message, tag="1")]
        GcsResult(super::GcsOutputResult),
    }
}
///  A Cloud Storage output result.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GcsOutputResult {
    ///  List of uris of the Cloud Storage objects. Example:
    ///  "gs://bucket_name/object_name".
    #[prost(string, repeated, tag="1")]
    pub uris: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
///  A Cloud Storage location.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GcsDestination {
    ///  Required.
    #[prost(oneof="gcs_destination::ObjectUri", tags="1, 2")]
    pub object_uri: ::core::option::Option<gcs_destination::ObjectUri>,
}
/// Nested message and enum types in `GcsDestination`.
pub mod gcs_destination {
    ///  Required.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ObjectUri {
        ///  The uri of the Cloud Storage object. It's the same uri that is used by
        ///  gsutil. Example: "gs://bucket_name/object_name". See [Viewing and
        ///  Editing Object
        ///  Metadata](<https://cloud.google.com/storage/docs/viewing-editing-metadata>)
        ///  for more information.
        ///
        ///  If the specified Cloud Storage object already exists and there is no
        ///  \[hold\](<https://cloud.google.com/storage/docs/object-holds>), it will be
        ///  overwritten with the exported result.
        #[prost(string, tag="1")]
        Uri(::prost::alloc::string::String),
        ///  The uri prefix of all generated Cloud Storage objects. Example:
        ///  "gs://bucket_name/object_name_prefix". Each object uri is in format:
        ///  "gs://bucket_name/object_name_prefix/<asset type>/<shard number> and only
        ///  contains assets for that type. <shard number> starts from 0. Example:
        ///  "gs://bucket_name/object_name_prefix/compute.googleapis.com/Disk/0" is
        ///  the first shard of output objects containing all
        ///  compute.googleapis.com/Disk assets. An INVALID_ARGUMENT error will be
        ///  returned if file with the same name "gs://bucket_name/object_name_prefix"
        ///  already exists.
        #[prost(string, tag="2")]
        UriPrefix(::prost::alloc::string::String),
    }
}
///  A BigQuery destination for exporting assets to.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BigQueryDestination {
    ///  Required. The BigQuery dataset in format
    ///  "projects/projectId/datasets/datasetId", to which the snapshot result
    ///  should be exported. If this dataset does not exist, the export call returns
    ///  an INVALID_ARGUMENT error. Setting the `contentType` for `exportAssets`
    ///  determines the
    ///  \[schema\](/asset-inventory/docs/exporting-to-bigquery#bigquery-schema)
    ///  of the BigQuery table. Setting `separateTablesPerAssetType` to `TRUE` also
    ///  influences the schema.
    #[prost(string, tag="1")]
    pub dataset: ::prost::alloc::string::String,
    ///  Required. The BigQuery table to which the snapshot result should be
    ///  written. If this table does not exist, a new table with the given name
    ///  will be created.
    #[prost(string, tag="2")]
    pub table: ::prost::alloc::string::String,
    ///  If the destination table already exists and this flag is `TRUE`, the
    ///  table will be overwritten by the contents of assets snapshot. If the flag
    ///  is `FALSE` or unset and the destination table already exists, the export
    ///  call returns an INVALID_ARGUMEMT error.
    #[prost(bool, tag="3")]
    pub force: bool,
    ///  \[partition_spec\] determines whether to export to partitioned table(s) and
    ///  how to partition the data.
    ///
    ///  If \[partition_spec\] is unset or \[partition_spec.partition_key\] is unset or
    ///  `PARTITION_KEY_UNSPECIFIED`, the snapshot results will be exported to
    ///  non-partitioned table(s). \[force\] will decide whether to overwrite existing
    ///  table(s).
    ///
    ///  If \[partition_spec\] is specified. First, the snapshot results will be
    ///  written to partitioned table(s) with two additional timestamp columns,
    ///  readTime and requestTime, one of which will be the partition key. Secondly,
    ///  in the case when any destination table already exists, it will first try to
    ///  update existing table's schema as necessary by appending additional
    ///  columns. Then, if \[force\] is `TRUE`, the corresponding partition will be
    ///  overwritten by the snapshot results (data in different partitions will
    ///  remain intact); if \[force\] is unset or `FALSE`, it will append the data. An
    ///  error will be returned if the schema update or data appension fails.
    #[prost(message, optional, tag="4")]
    pub partition_spec: ::core::option::Option<PartitionSpec>,
    ///  If this flag is `TRUE`, the snapshot results will be written to one or
    ///  multiple tables, each of which contains results of one asset type. The
    ///  \[force\] and \[partition_spec\] fields will apply to each of them.
    ///
    ///  Field \[table\] will be concatenated with "_" and the asset type names (see
    ///  <https://cloud.google.com/asset-inventory/docs/supported-asset-types> for
    ///  supported asset types) to construct per-asset-type table names, in which
    ///  all non-alphanumeric characters like "." and "/" will be substituted by
    ///  "_". Example: if field \[table\] is "mytable" and snapshot results
    ///  contain "storage.googleapis.com/Bucket" assets, the corresponding table
    ///  name will be "mytable_storage_googleapis_com_Bucket". If any of these
    ///  tables does not exist, a new table with the concatenated name will be
    ///  created.
    ///
    ///  When \[content_type\] in the ExportAssetsRequest is `RESOURCE`, the schema of
    ///  each table will include RECORD-type columns mapped to the nested fields in
    ///  the Asset.resource.data field of that asset type (up to the 15 nested level
    ///  BigQuery supports
    ///  (<https://cloud.google.com/bigquery/docs/nested-repeated#limitations>)). The
    ///  fields in >15 nested levels will be stored in JSON format string as a child
    ///  column of its parent RECORD column.
    ///
    ///  If error occurs when exporting to any table, the whole export call will
    ///  return an error but the export results that already succeed will persist.
    ///  Example: if exporting to table_type_A succeeds when exporting to
    ///  table_type_B fails during one export call, the results in table_type_A will
    ///  persist and there will not be partial results persisting in a table.
    #[prost(bool, tag="5")]
    pub separate_tables_per_asset_type: bool,
}
///  Specifications of BigQuery partitioned table as export destination.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PartitionSpec {
    ///  The partition key for BigQuery partitioned table.
    #[prost(enumeration="partition_spec::PartitionKey", tag="1")]
    pub partition_key: i32,
}
/// Nested message and enum types in `PartitionSpec`.
pub mod partition_spec {
    ///  This enum is used to determine the partition key column when exporting
    ///  assets to BigQuery partitioned table(s). Note that, if the partition key is
    ///  a timestamp column, the actual partition is based on its date value
    ///  (expressed in UTC. see details in
    ///  <https://cloud.google.com/bigquery/docs/partitioned-tables#date_timestamp_partitioned_tables>).
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum PartitionKey {
        ///  Unspecified partition key. If used, it means using non-partitioned table.
        Unspecified = 0,
        ///  The time when the snapshot is taken. If specified as partition key, the
        ///  result table(s) is partitoned by the additional timestamp column,
        ///  readTime. If \[read_time\] in ExportAssetsRequest is specified, the
        ///  readTime column's value will be the same as it. Otherwise, its value will
        ///  be the current time that is used to take the snapshot.
        ReadTime = 1,
        ///  The time when the request is received and started to be processed. If
        ///  specified as partition key, the result table(s) is partitoned by the
        ///  requestTime column, an additional timestamp column representing when the
        ///  request was received.
        RequestTime = 2,
    }
    impl PartitionKey {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                PartitionKey::Unspecified => "PARTITION_KEY_UNSPECIFIED",
                PartitionKey::ReadTime => "READ_TIME",
                PartitionKey::RequestTime => "REQUEST_TIME",
            }
        }
    }
}
///  A Pub/Sub destination.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PubsubDestination {
    ///  The name of the Pub/Sub topic to publish to.
    ///  Example: `projects/PROJECT_ID/topics/TOPIC_ID`.
    #[prost(string, tag="1")]
    pub topic: ::prost::alloc::string::String,
}
///  Output configuration for asset feed destination.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeedOutputConfig {
    ///  Asset feed destination.
    #[prost(oneof="feed_output_config::Destination", tags="1")]
    pub destination: ::core::option::Option<feed_output_config::Destination>,
}
/// Nested message and enum types in `FeedOutputConfig`.
pub mod feed_output_config {
    ///  Asset feed destination.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Destination {
        ///  Destination on Pub/Sub.
        #[prost(message, tag="1")]
        PubsubDestination(super::PubsubDestination),
    }
}
///  An asset feed used to export asset updates to a destinations.
///  An asset feed filter controls what updates are exported.
///  The asset feed must be created within a project, organization, or
///  folder. Supported destinations are:
///  Pub/Sub topics.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Feed {
    ///  Required. The format will be
    ///  projects/{project_number}/feeds/{client-assigned_feed_identifier} or
    ///  folders/{folder_number}/feeds/{client-assigned_feed_identifier} or
    ///  organizations/{organization_number}/feeds/{client-assigned_feed_identifier}
    ///
    ///  The client-assigned feed identifier must be unique within the parent
    ///  project/folder/organization.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    ///  A list of the full names of the assets to receive updates. You must specify
    ///  either or both of asset_names and asset_types. Only asset updates matching
    ///  specified asset_names or asset_types are exported to the feed.
    ///  Example:
    ///  `//compute.googleapis.com/projects/my_project_123/zones/zone1/instances/instance1`.
    ///  For a list of the full names for supported asset types, see [Resource
    ///  name format](/asset-inventory/docs/resource-name-format).
    #[prost(string, repeated, tag="2")]
    pub asset_names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    ///  A list of types of the assets to receive updates. You must specify either
    ///  or both of asset_names and asset_types. Only asset updates matching
    ///  specified asset_names or asset_types are exported to the feed.
    ///  Example: `"compute.googleapis.com/Disk"`
    ///
    ///  For a list of all supported asset types, see
    ///  [Supported asset types](/asset-inventory/docs/supported-asset-types).
    #[prost(string, repeated, tag="3")]
    pub asset_types: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    ///  Asset content type. If not specified, no content but the asset name and
    ///  type will be returned.
    #[prost(enumeration="ContentType", tag="4")]
    pub content_type: i32,
    ///  Required. Feed output configuration defining where the asset updates are
    ///  published to.
    #[prost(message, optional, tag="5")]
    pub feed_output_config: ::core::option::Option<FeedOutputConfig>,
    ///  A condition which determines whether an asset update should be published.
    ///  If specified, an asset will be returned only when the expression evaluates
    ///  to true.
    ///  When set, `expression` field in the `Expr` must be a valid [CEL expression]
    ///  (<https://github.com/google/cel-spec>) on a TemporalAsset with name
    ///  `temporal_asset`. Example: a Feed with expression ("temporal_asset.deleted
    ///  == true") will only publish Asset deletions. Other fields of `Expr` are
    ///  optional.
    ///
    ///  See our [user
    ///  guide](<https://cloud.google.com/asset-inventory/docs/monitoring-asset-changes-with-condition>)
    ///  for detailed instructions.
    #[prost(message, optional, tag="6")]
    pub condition: ::core::option::Option<super::super::super::r#type::Expr>,
    ///  A list of relationship types to output, for example:
    ///  `INSTANCE_TO_INSTANCEGROUP`. This field should only be specified if
    ///  content_type=RELATIONSHIP.
    ///  * If specified:
    ///  it outputs specified relationship updates on the \[asset_names\] or the
    ///  \[asset_types\]. It returns an error if any of the \[relationship_types\]
    ///  doesn't belong to the supported relationship types of the \[asset_names\] or
    ///  \[asset_types\], or any of the \[asset_names\] or the \[asset_types\] doesn't
    ///  belong to the source types of the \[relationship_types\].
    ///  * Otherwise:
    ///  it outputs the supported relationships of the types of \[asset_names\] and
    ///  \[asset_types\] or returns an error if any of the \[asset_names\] or the
    ///  \[asset_types\] has no replationship support.
    ///  See [Introduction to Cloud Asset
    ///  Inventory](<https://cloud.google.com/asset-inventory/docs/overview>)
    ///  for all supported asset types and relationship types.
    #[prost(string, repeated, tag="7")]
    pub relationship_types: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
///  Search all resources request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchAllResourcesRequest {
    ///  Required. A scope can be a project, a folder, or an organization. The
    ///  search is limited to the resources within the `scope`. The caller must be
    ///  granted the
    ///  \[`cloudasset.assets.searchAllResources`\](<https://cloud.google.com/asset-inventory/docs/access-control#required_permissions>)
    ///  permission on the desired scope.
    ///
    ///  The allowed values are:
    ///
    ///  * projects/{PROJECT_ID} (e.g., "projects/foo-bar")
    ///  * projects/{PROJECT_NUMBER} (e.g., "projects/12345678")
    ///  * folders/{FOLDER_NUMBER} (e.g., "folders/1234567")
    ///  * organizations/{ORGANIZATION_NUMBER} (e.g., "organizations/123456")
    #[prost(string, tag="1")]
    pub scope: ::prost::alloc::string::String,
    ///  Optional. The query statement. See [how to construct a
    ///  query](<https://cloud.google.com/asset-inventory/docs/searching-resources#how_to_construct_a_query>)
    ///  for more information. If not specified or empty, it will search all the
    ///  resources within the specified `scope`.
    ///
    ///  Examples:
    ///
    ///  * `name:Important` to find Cloud resources whose name contains
    ///    "Important" as a word.
    ///  * `name=Important` to find the Cloud resource whose name is exactly
    ///    "Important".
    ///  * `displayName:Impor*` to find Cloud resources whose display name
    ///    contains "Impor" as a prefix of any word in the field.
    ///  * `location:us-west*` to find Cloud resources whose location contains both
    ///    "us" and "west" as prefixes.
    ///  * `labels:prod` to find Cloud resources whose labels contain "prod" as
    ///    a key or value.
    ///  * `labels.env:prod` to find Cloud resources that have a label "env"
    ///    and its value is "prod".
    ///  * `labels.env:*` to find Cloud resources that have a label "env".
    ///  * `kmsKey:key` to find Cloud resources encrypted with a customer-managed
    ///    encryption key whose name contains the word "key".
    ///  * `relationships:instance-group-1` to find Cloud resources that have
    ///    relationships with "instance-group-1" in the related resource name.
    ///  * `relationships:INSTANCE_TO_INSTANCEGROUP` to find compute instances that
    ///    have relationships of type "INSTANCE_TO_INSTANCEGROUP".
    ///  * `relationships.INSTANCE_TO_INSTANCEGROUP:instance-group-1` to find
    ///    compute instances that have relationships with "instance-group-1" in the
    ///    compute instance group resource name, for relationship type
    ///    "INSTANCE_TO_INSTANCEGROUP".
    ///  * `state:ACTIVE` to find Cloud resources whose state contains "ACTIVE" as a
    ///    word.
    ///  * `NOT state:ACTIVE` to find Cloud resources whose state doesn't contain
    ///    "ACTIVE" as a word.
    ///  * `createTime<1609459200` to find Cloud resources that were created before
    ///    "2021-01-01 00:00:00 UTC". 1609459200 is the epoch timestamp of
    ///    "2021-01-01 00:00:00 UTC" in seconds.
    ///  * `updateTime>1609459200` to find Cloud resources that were updated after
    ///    "2021-01-01 00:00:00 UTC". 1609459200 is the epoch timestamp of
    ///    "2021-01-01 00:00:00 UTC" in seconds.
    ///  * `Important` to find Cloud resources that contain "Important" as a word
    ///    in any of the searchable fields.
    ///  * `Impor*` to find Cloud resources that contain "Impor" as a prefix of any
    ///    word in any of the searchable fields.
    ///  * `Important location:(us-west1 OR global)` to find Cloud
    ///    resources that contain "Important" as a word in any of the searchable
    ///    fields and are also located in the "us-west1" region or the "global"
    ///    location.
    #[prost(string, tag="2")]
    pub query: ::prost::alloc::string::String,
    ///  Optional. A list of asset types that this request searches for. If empty,
    ///  it will search all the [searchable asset
    ///  types](<https://cloud.google.com/asset-inventory/docs/supported-asset-types#searchable_asset_types>).
    ///
    ///  Regular expressions are also supported. For example:
    ///
    ///  * "compute.googleapis.com.*" snapshots resources whose asset type starts
    ///  with "compute.googleapis.com".
    ///  * ".*Instance" snapshots resources whose asset type ends with "Instance".
    ///  * ".*Instance.*" snapshots resources whose asset type contains "Instance".
    ///
    ///  See \[RE2\](<https://github.com/google/re2/wiki/Syntax>) for all supported
    ///  regular expression syntax. If the regular expression does not match any
    ///  supported asset type, an INVALID_ARGUMENT error will be returned.
    #[prost(string, repeated, tag="3")]
    pub asset_types: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    ///  Optional. The page size for search result pagination. Page size is capped
    ///  at 500 even if a larger value is given. If set to zero, server will pick an
    ///  appropriate default. Returned results may be fewer than requested. When
    ///  this happens, there could be more results as long as `next_page_token` is
    ///  returned.
    #[prost(int32, tag="4")]
    pub page_size: i32,
    ///  Optional. If present, then retrieve the next batch of results from the
    ///  preceding call to this method. `page_token` must be the value of
    ///  `next_page_token` from the previous response. The values of all other
    ///  method parameters, must be identical to those in the previous call.
    #[prost(string, tag="5")]
    pub page_token: ::prost::alloc::string::String,
    ///  Optional. A comma-separated list of fields specifying the sorting order of
    ///  the results. The default order is ascending. Add " DESC" after the field
    ///  name to indicate descending order. Redundant space characters are ignored.
    ///  Example: "location DESC, name".
    ///  Only singular primitive fields in the response are sortable:
    ///
    ///    * name
    ///    * assetType
    ///    * project
    ///    * displayName
    ///    * description
    ///    * location
    ///    * kmsKey
    ///    * createTime
    ///    * updateTime
    ///    * state
    ///    * parentFullResourceName
    ///    * parentAssetType
    ///
    ///  All the other fields such as repeated fields (e.g., `networkTags`), map
    ///  fields (e.g., `labels`) and struct fields (e.g., `additionalAttributes`)
    ///  are not supported.
    #[prost(string, tag="6")]
    pub order_by: ::prost::alloc::string::String,
    ///  Optional. A comma-separated list of fields specifying which fields to be
    ///  returned in ResourceSearchResult. Only '*' or combination of top level
    ///  fields can be specified. Field names of both snake_case and camelCase are
    ///  supported. Examples: `"*"`, `"name,location"`, `"name,versionedResources"`.
    ///
    ///  The read_mask paths must be valid field paths listed but not limited to
    ///  (both snake_case and camelCase are supported):
    ///
    ///    * name
    ///    * assetType
    ///    * project
    ///    * displayName
    ///    * description
    ///    * location
    ///    * tagKeys
    ///    * tagValues
    ///    * tagValueIds
    ///    * labels
    ///    * networkTags
    ///    * kmsKey
    ///    * createTime
    ///    * updateTime
    ///    * state
    ///    * additionalAttributes
    ///    * versionedResources
    ///
    ///  If read_mask is not specified, all fields except versionedResources will
    ///  be returned.
    ///  If only '*' is specified, all fields including versionedResources will be
    ///  returned.
    ///  Any invalid field path will trigger INVALID_ARGUMENT error.
    #[prost(message, optional, tag="8")]
    pub read_mask: ::core::option::Option<::prost_types::FieldMask>,
}
///  Search all resources response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchAllResourcesResponse {
    ///  A list of Resources that match the search query. It contains the resource
    ///  standard metadata information.
    #[prost(message, repeated, tag="1")]
    pub results: ::prost::alloc::vec::Vec<ResourceSearchResult>,
    ///  If there are more results than those appearing in this response, then
    ///  `next_page_token` is included. To get the next set of results, call this
    ///  method again using the value of `next_page_token` as `page_token`.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
///  Search all IAM policies request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchAllIamPoliciesRequest {
    ///  Required. A scope can be a project, a folder, or an organization. The
    ///  search is limited to the IAM policies within the `scope`. The caller must
    ///  be granted the
    ///  \[`cloudasset.assets.searchAllIamPolicies`\](<https://cloud.google.com/asset-inventory/docs/access-control#required_permissions>)
    ///  permission on the desired scope.
    ///
    ///  The allowed values are:
    ///
    ///  * projects/{PROJECT_ID} (e.g., "projects/foo-bar")
    ///  * projects/{PROJECT_NUMBER} (e.g., "projects/12345678")
    ///  * folders/{FOLDER_NUMBER} (e.g., "folders/1234567")
    ///  * organizations/{ORGANIZATION_NUMBER} (e.g., "organizations/123456")
    #[prost(string, tag="1")]
    pub scope: ::prost::alloc::string::String,
    ///  Optional. The query statement. See [how to construct a
    ///  query](<https://cloud.google.com/asset-inventory/docs/searching-iam-policies#how_to_construct_a_query>)
    ///  for more information. If not specified or empty, it will search all the
    ///  IAM policies within the specified `scope`. Note that the query string is
    ///  compared against each Cloud IAM policy binding, including its principals,
    ///  roles, and Cloud IAM conditions. The returned Cloud IAM policies will only
    ///  contain the bindings that match your query. To learn more about the IAM
    ///  policy structure, see the [IAM policy
    ///  documentation](<https://cloud.google.com/iam/help/allow-policies/structure>).
    ///
    ///  Examples:
    ///
    ///  * `policy:amy@gmail.com` to find IAM policy bindings that specify user
    ///    "amy@gmail.com".
    ///  * `policy:roles/compute.admin` to find IAM policy bindings that specify
    ///    the Compute Admin role.
    ///  * `policy:comp*` to find IAM policy bindings that contain "comp" as a
    ///    prefix of any word in the binding.
    ///  * `policy.role.permissions:storage.buckets.update` to find IAM policy
    ///    bindings that specify a role containing "storage.buckets.update"
    ///    permission. Note that if callers don't have `iam.roles.get` access to a
    ///    role's included permissions, policy bindings that specify this role will
    ///    be dropped from the search results.
    ///  * `policy.role.permissions:upd*` to find IAM policy bindings that specify a
    ///    role containing "upd" as a prefix of any word in the role permission.
    ///    Note that if callers don't have `iam.roles.get` access to a role's
    ///    included permissions, policy bindings that specify this role will be
    ///    dropped from the search results.
    ///  * `resource:organizations/123456` to find IAM policy bindings
    ///    that are set on "organizations/123456".
    ///  * `resource=//cloudresourcemanager.googleapis.com/projects/myproject` to
    ///    find IAM policy bindings that are set on the project named "myproject".
    ///  * `Important` to find IAM policy bindings that contain "Important" as a
    ///    word in any of the searchable fields (except for the included
    ///    permissions).
    ///  * `resource:(instance1 OR instance2) policy:amy` to find
    ///    IAM policy bindings that are set on resources "instance1" or
    ///    "instance2" and also specify user "amy".
    ///  * `roles:roles/compute.admin` to find IAM policy bindings that specify the
    ///    Compute Admin role.
    ///  * `memberTypes:user` to find IAM policy bindings that contain the
    ///    principal type "user".
    #[prost(string, tag="2")]
    pub query: ::prost::alloc::string::String,
    ///  Optional. The page size for search result pagination. Page size is capped
    ///  at 500 even if a larger value is given. If set to zero, server will pick an
    ///  appropriate default. Returned results may be fewer than requested. When
    ///  this happens, there could be more results as long as `next_page_token` is
    ///  returned.
    #[prost(int32, tag="3")]
    pub page_size: i32,
    ///  Optional. If present, retrieve the next batch of results from the preceding
    ///  call to this method. `page_token` must be the value of `next_page_token`
    ///  from the previous response. The values of all other method parameters must
    ///  be identical to those in the previous call.
    #[prost(string, tag="4")]
    pub page_token: ::prost::alloc::string::String,
    ///  Optional. A list of asset types that the IAM policies are attached to. If
    ///  empty, it will search the IAM policies that are attached to all the
    ///  [searchable asset
    ///  types](<https://cloud.google.com/asset-inventory/docs/supported-asset-types#searchable_asset_types>).
    ///
    ///  Regular expressions are also supported. For example:
    ///
    ///  * "compute.googleapis.com.*" snapshots IAM policies attached to asset type
    ///  starts with "compute.googleapis.com".
    ///  * ".*Instance" snapshots IAM policies attached to asset type ends with
    ///  "Instance".
    ///  * ".*Instance.*" snapshots IAM policies attached to asset type contains
    ///  "Instance".
    ///
    ///  See \[RE2\](<https://github.com/google/re2/wiki/Syntax>) for all supported
    ///  regular expression syntax. If the regular expression does not match any
    ///  supported asset type, an INVALID_ARGUMENT error will be returned.
    #[prost(string, repeated, tag="5")]
    pub asset_types: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    ///  Optional. A comma-separated list of fields specifying the sorting order of
    ///  the results. The default order is ascending. Add " DESC" after the field
    ///  name to indicate descending order. Redundant space characters are ignored.
    ///  Example: "assetType DESC, resource".
    ///  Only singular primitive fields in the response are sortable:
    ///    * resource
    ///    * assetType
    ///    * project
    ///  All the other fields such as repeated fields (e.g., `folders`) and
    ///  non-primitive fields (e.g., `policy`) are not supported.
    #[prost(string, tag="7")]
    pub order_by: ::prost::alloc::string::String,
}
///  Search all IAM policies response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchAllIamPoliciesResponse {
    ///  A list of IamPolicy that match the search query. Related information such
    ///  as the associated resource is returned along with the policy.
    #[prost(message, repeated, tag="1")]
    pub results: ::prost::alloc::vec::Vec<IamPolicySearchResult>,
    ///  Set if there are more results than those appearing in this response; to get
    ///  the next set of results, call this method again, using this value as the
    ///  `page_token`.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
///  IAM policy analysis query message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IamPolicyAnalysisQuery {
    ///  Required. The relative name of the root asset. Only resources and IAM
    ///  policies within the scope will be analyzed.
    ///
    ///  This can only be an organization number (such as "organizations/123"), a
    ///  folder number (such as "folders/123"), a project ID (such as
    ///  "projects/my-project-id"), or a project number (such as "projects/12345").
    ///
    ///  To know how to get organization id, visit [here
    ///  ](<https://cloud.google.com/resource-manager/docs/creating-managing-organization#retrieving_your_organization_id>).
    ///
    ///  To know how to get folder or project id, visit [here
    ///  ](<https://cloud.google.com/resource-manager/docs/creating-managing-folders#viewing_or_listing_folders_and_projects>).
    #[prost(string, tag="1")]
    pub scope: ::prost::alloc::string::String,
    ///  Optional. Specifies a resource for analysis.
    #[prost(message, optional, tag="2")]
    pub resource_selector: ::core::option::Option<iam_policy_analysis_query::ResourceSelector>,
    ///  Optional. Specifies an identity for analysis.
    #[prost(message, optional, tag="3")]
    pub identity_selector: ::core::option::Option<iam_policy_analysis_query::IdentitySelector>,
    ///  Optional. Specifies roles or permissions for analysis. This is optional.
    #[prost(message, optional, tag="4")]
    pub access_selector: ::core::option::Option<iam_policy_analysis_query::AccessSelector>,
    ///  Optional. The query options.
    #[prost(message, optional, tag="5")]
    pub options: ::core::option::Option<iam_policy_analysis_query::Options>,
    ///  Optional. The hypothetical context for IAM conditions evaluation.
    #[prost(message, optional, tag="6")]
    pub condition_context: ::core::option::Option<iam_policy_analysis_query::ConditionContext>,
}
/// Nested message and enum types in `IamPolicyAnalysisQuery`.
pub mod iam_policy_analysis_query {
    ///  Specifies the resource to analyze for access policies, which may be set
    ///  directly on the resource, or on ancestors such as organizations, folders or
    ///  projects.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ResourceSelector {
        ///  Required. The [full resource name]
        ///  (<https://cloud.google.com/asset-inventory/docs/resource-name-format>)
        ///  of a resource of [supported resource
        ///  types](<https://cloud.google.com/asset-inventory/docs/supported-asset-types#analyzable_asset_types>).
        #[prost(string, tag="1")]
        pub full_resource_name: ::prost::alloc::string::String,
    }
    ///  Specifies an identity for which to determine resource access, based on
    ///  roles assigned either directly to them or to the groups they belong to,
    ///  directly or indirectly.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct IdentitySelector {
        ///  Required. The identity appear in the form of principals in
        ///  [IAM policy
        ///  binding](<https://cloud.google.com/iam/reference/rest/v1/Binding>).
        ///
        ///  The examples of supported forms are:
        ///  "user:mike@example.com",
        ///  "group:admins@example.com",
        ///  "domain:google.com",
        ///  "serviceAccount:my-project-id@appspot.gserviceaccount.com".
        ///
        ///  Notice that wildcard characters (such as * and ?) are not supported.
        ///  You must give a specific identity.
        #[prost(string, tag="1")]
        pub identity: ::prost::alloc::string::String,
    }
    ///  Specifies roles and/or permissions to analyze, to determine both the
    ///  identities possessing them and the resources they control. If multiple
    ///  values are specified, results will include roles or permissions matching
    ///  any of them. The total number of roles and permissions should be equal or
    ///  less than 10.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct AccessSelector {
        ///  Optional. The roles to appear in result.
        #[prost(string, repeated, tag="1")]
        pub roles: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        ///  Optional. The permissions to appear in result.
        #[prost(string, repeated, tag="2")]
        pub permissions: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
    ///  Contains query options.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Options {
        ///  Optional. If true, the identities section of the result will expand any
        ///  Google groups appearing in an IAM policy binding.
        ///
        ///  If
        ///  \[IamPolicyAnalysisQuery.identity_selector][google.cloud.asset.v1.IamPolicyAnalysisQuery.identity_selector\]
        ///  is specified, the identity in the result will be determined by the
        ///  selector, and this flag is not allowed to set.
        ///
        ///  If true, the default max expansion per group is 1000 for
        ///  AssetService.AnalyzeIamPolicy][].
        ///
        ///  Default is false.
        #[prost(bool, tag="1")]
        pub expand_groups: bool,
        ///  Optional. If true, the access section of result will expand any roles
        ///  appearing in IAM policy bindings to include their permissions.
        ///
        ///  If
        ///  \[IamPolicyAnalysisQuery.access_selector][google.cloud.asset.v1.IamPolicyAnalysisQuery.access_selector\]
        ///  is specified, the access section of the result will be determined by the
        ///  selector, and this flag is not allowed to set.
        ///
        ///  Default is false.
        #[prost(bool, tag="2")]
        pub expand_roles: bool,
        ///  Optional. If true and
        ///  \[IamPolicyAnalysisQuery.resource_selector][google.cloud.asset.v1.IamPolicyAnalysisQuery.resource_selector\]
        ///  is not specified, the resource section of the result will expand any
        ///  resource attached to an IAM policy to include resources lower in the
        ///  resource hierarchy.
        ///
        ///  For example, if the request analyzes for which resources user A has
        ///  permission P, and the results include an IAM policy with P on a GCP
        ///  folder, the results will also include resources in that folder with
        ///  permission P.
        ///
        ///  If true and
        ///  \[IamPolicyAnalysisQuery.resource_selector][google.cloud.asset.v1.IamPolicyAnalysisQuery.resource_selector\]
        ///  is specified, the resource section of the result will expand the
        ///  specified resource to include resources lower in the resource hierarchy.
        ///  Only project or lower resources are supported. Folder and organization
        ///  resource cannot be used together with this option.
        ///
        ///  For example, if the request analyzes for which users have permission P on
        ///  a GCP project with this option enabled, the results will include all
        ///  users who have permission P on that project or any lower resource.
        ///
        ///  If true, the default max expansion per resource is 1000 for
        ///  AssetService.AnalyzeIamPolicy][] and 100000 for
        ///  AssetService.AnalyzeIamPolicyLongrunning][].
        ///
        ///  Default is false.
        #[prost(bool, tag="3")]
        pub expand_resources: bool,
        ///  Optional. If true, the result will output the relevant parent/child
        ///  relationships between resources. Default is false.
        #[prost(bool, tag="4")]
        pub output_resource_edges: bool,
        ///  Optional. If true, the result will output the relevant membership
        ///  relationships between groups and other groups, and between groups and
        ///  principals. Default is false.
        #[prost(bool, tag="5")]
        pub output_group_edges: bool,
        ///  Optional. If true, the response will include access analysis from
        ///  identities to resources via service account impersonation. This is a very
        ///  expensive operation, because many derived queries will be executed. We
        ///  highly recommend you use
        ///  \[AssetService.AnalyzeIamPolicyLongrunning][google.cloud.asset.v1.AssetService.AnalyzeIamPolicyLongrunning\]
        ///  rpc instead.
        ///
        ///  For example, if the request analyzes for which resources user A has
        ///  permission P, and there's an IAM policy states user A has
        ///  iam.serviceAccounts.getAccessToken permission to a service account SA,
        ///  and there's another IAM policy states service account SA has permission P
        ///  to a GCP folder F, then user A potentially has access to the GCP folder
        ///  F. And those advanced analysis results will be included in
        ///  \[AnalyzeIamPolicyResponse.service_account_impersonation_analysis][google.cloud.asset.v1.AnalyzeIamPolicyResponse.service_account_impersonation_analysis\].
        ///
        ///  Another example, if the request analyzes for who has
        ///  permission P to a GCP folder F, and there's an IAM policy states user A
        ///  has iam.serviceAccounts.actAs permission to a service account SA, and
        ///  there's another IAM policy states service account SA has permission P to
        ///  the GCP folder F, then user A potentially has access to the GCP folder
        ///  F. And those advanced analysis results will be included in
        ///  \[AnalyzeIamPolicyResponse.service_account_impersonation_analysis][google.cloud.asset.v1.AnalyzeIamPolicyResponse.service_account_impersonation_analysis\].
        ///
        ///  Only the following permissions are considered in this analysis:
        ///
        ///  * `iam.serviceAccounts.actAs`
        ///  * `iam.serviceAccounts.signBlob`
        ///  * `iam.serviceAccounts.signJwt`
        ///  * `iam.serviceAccounts.getAccessToken`
        ///  * `iam.serviceAccounts.getOpenIdToken`
        ///  * `iam.serviceAccounts.implicitDelegation`
        ///
        ///  Default is false.
        #[prost(bool, tag="6")]
        pub analyze_service_account_impersonation: bool,
    }
    ///  The IAM conditions context.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ConditionContext {
        ///  The IAM conditions time context.
        #[prost(oneof="condition_context::TimeContext", tags="1")]
        pub time_context: ::core::option::Option<condition_context::TimeContext>,
    }
    /// Nested message and enum types in `ConditionContext`.
    pub mod condition_context {
        ///  The IAM conditions time context.
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum TimeContext {
            ///  The hypothetical access timestamp to evaluate IAM conditions. Note that
            ///  this value must not be earlier than the current time; otherwise, an
            ///  INVALID_ARGUMENT error will be returned.
            #[prost(message, tag="1")]
            AccessTime(::prost_types::Timestamp),
        }
    }
}
///  A request message for
///  \[AssetService.AnalyzeIamPolicy][google.cloud.asset.v1.AssetService.AnalyzeIamPolicy\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnalyzeIamPolicyRequest {
    ///  Required. The request query.
    #[prost(message, optional, tag="1")]
    pub analysis_query: ::core::option::Option<IamPolicyAnalysisQuery>,
    ///  Optional. The name of a saved query, which must be in the format of:
    ///
    ///  * projects/project_number/savedQueries/saved_query_id
    ///  * folders/folder_number/savedQueries/saved_query_id
    ///  * organizations/organization_number/savedQueries/saved_query_id
    ///
    ///  If both `analysis_query` and `saved_analysis_query` are provided, they
    ///  will be merged together with the `saved_analysis_query` as base and
    ///  the `analysis_query` as overrides. For more details of the merge behavior,
    ///  please refer to the
    ///  \[MergeFrom\](<https://developers.google.com/protocol-buffers/docs/reference/cpp/google.protobuf.message#Message.MergeFrom.details>)
    ///  page.
    ///
    ///  Note that you cannot override primitive fields with default value, such as
    ///  0 or empty string, etc., because we use proto3, which doesn't support field
    ///  presence yet.
    #[prost(string, tag="3")]
    pub saved_analysis_query: ::prost::alloc::string::String,
    ///  Optional. Amount of time executable has to complete.  See JSON
    ///  representation of
    ///  \[Duration\](<https://developers.google.com/protocol-buffers/docs/proto3#json>).
    ///
    ///  If this field is set with a value less than the RPC deadline, and the
    ///  execution of your query hasn't finished in the specified
    ///  execution timeout,  you will get a response with partial result.
    ///  Otherwise, your query's execution will continue until the RPC deadline.
    ///  If it's not finished until then, you will get a  DEADLINE_EXCEEDED error.
    ///
    ///  Default is empty.
    #[prost(message, optional, tag="2")]
    pub execution_timeout: ::core::option::Option<::prost_types::Duration>,
}
///  A response message for
///  \[AssetService.AnalyzeIamPolicy][google.cloud.asset.v1.AssetService.AnalyzeIamPolicy\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnalyzeIamPolicyResponse {
    ///  The main analysis that matches the original request.
    #[prost(message, optional, tag="1")]
    pub main_analysis: ::core::option::Option<analyze_iam_policy_response::IamPolicyAnalysis>,
    ///  The service account impersonation analysis if
    ///  \[AnalyzeIamPolicyRequest.analyze_service_account_impersonation][\] is
    ///  enabled.
    #[prost(message, repeated, tag="2")]
    pub service_account_impersonation_analysis: ::prost::alloc::vec::Vec<analyze_iam_policy_response::IamPolicyAnalysis>,
    ///  Represents whether all entries in the
    ///  \[main_analysis][google.cloud.asset.v1.AnalyzeIamPolicyResponse.main_analysis\]
    ///  and
    ///  \[service_account_impersonation_analysis][google.cloud.asset.v1.AnalyzeIamPolicyResponse.service_account_impersonation_analysis\]
    ///  have been fully explored to answer the query in the request.
    #[prost(bool, tag="3")]
    pub fully_explored: bool,
}
/// Nested message and enum types in `AnalyzeIamPolicyResponse`.
pub mod analyze_iam_policy_response {
    ///  An analysis message to group the query and results.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct IamPolicyAnalysis {
        ///  The analysis query.
        #[prost(message, optional, tag="1")]
        pub analysis_query: ::core::option::Option<super::IamPolicyAnalysisQuery>,
        ///  A list of
        ///  \[IamPolicyAnalysisResult][google.cloud.asset.v1.IamPolicyAnalysisResult\]
        ///  that matches the analysis query, or empty if no result is found.
        #[prost(message, repeated, tag="2")]
        pub analysis_results: ::prost::alloc::vec::Vec<super::IamPolicyAnalysisResult>,
        ///  Represents whether all entries in the
        ///  \[analysis_results][google.cloud.asset.v1.AnalyzeIamPolicyResponse.IamPolicyAnalysis.analysis_results\]
        ///  have been fully explored to answer the query.
        #[prost(bool, tag="3")]
        pub fully_explored: bool,
        ///  A list of non-critical errors happened during the query handling.
        #[prost(message, repeated, tag="5")]
        pub non_critical_errors: ::prost::alloc::vec::Vec<super::IamPolicyAnalysisState>,
    }
}
///  Output configuration for export IAM policy analysis destination.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IamPolicyAnalysisOutputConfig {
    ///  IAM policy analysis export destination.
    #[prost(oneof="iam_policy_analysis_output_config::Destination", tags="1, 2")]
    pub destination: ::core::option::Option<iam_policy_analysis_output_config::Destination>,
}
/// Nested message and enum types in `IamPolicyAnalysisOutputConfig`.
pub mod iam_policy_analysis_output_config {
    ///  A Cloud Storage location.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GcsDestination {
        ///  Required. The uri of the Cloud Storage object. It's the same uri that is
        ///  used by gsutil. Example: "gs://bucket_name/object_name". See [Viewing and
        ///  Editing Object
        ///  Metadata](<https://cloud.google.com/storage/docs/viewing-editing-metadata>)
        ///  for more information.
        ///
        ///  If the specified Cloud Storage object already exists and there is no
        ///  \[hold\](<https://cloud.google.com/storage/docs/object-holds>), it will be
        ///  overwritten with the analysis result.
        #[prost(string, tag="1")]
        pub uri: ::prost::alloc::string::String,
    }
    ///  A BigQuery destination.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct BigQueryDestination {
        ///  Required. The BigQuery dataset in format
        ///  "projects/projectId/datasets/datasetId", to which the analysis results
        ///  should be exported. If this dataset does not exist, the export call will
        ///  return an INVALID_ARGUMENT error.
        #[prost(string, tag="1")]
        pub dataset: ::prost::alloc::string::String,
        ///  Required. The prefix of the BigQuery tables to which the analysis results
        ///  will be written. Tables will be created based on this table_prefix if not
        ///  exist:
        ///  * <table_prefix>_analysis table will contain export operation's metadata.
        ///  * <table_prefix>_analysis_result will contain all the
        ///    \[IamPolicyAnalysisResult][google.cloud.asset.v1.IamPolicyAnalysisResult\].
        ///  When \[partition_key\] is specified, both tables will be partitioned based
        ///  on the \[partition_key\].
        #[prost(string, tag="2")]
        pub table_prefix: ::prost::alloc::string::String,
        ///  The partition key for BigQuery partitioned table.
        #[prost(enumeration="big_query_destination::PartitionKey", tag="3")]
        pub partition_key: i32,
        ///  Optional. Specifies the action that occurs if the destination table or
        ///  partition already exists. The following values are supported:
        ///
        ///  * WRITE_TRUNCATE: If the table or partition already exists, BigQuery
        ///  overwrites the entire table or all the partitions data.
        ///  * WRITE_APPEND: If the table or partition already exists, BigQuery
        ///  appends the data to the table or the latest partition.
        ///  * WRITE_EMPTY: If the table already exists and contains data, an error is
        ///  returned.
        ///
        ///  The default value is WRITE_APPEND. Each action is atomic and only occurs
        ///  if BigQuery is able to complete the job successfully. Details are at
        ///  <https://cloud.google.com/bigquery/docs/loading-data-local#appending_to_or_overwriting_a_table_using_a_local_file.>
        #[prost(string, tag="4")]
        pub write_disposition: ::prost::alloc::string::String,
    }
    /// Nested message and enum types in `BigQueryDestination`.
    pub mod big_query_destination {
        ///  This enum determines the partition key column for the bigquery tables.
        ///  Partitioning can improve query performance and reduce query cost by
        ///  filtering partitions. Refer to
        ///  <https://cloud.google.com/bigquery/docs/partitioned-tables> for details.
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
        #[repr(i32)]
        pub enum PartitionKey {
            ///  Unspecified partition key. Tables won't be partitioned using this
            ///  option.
            Unspecified = 0,
            ///  The time when the request is received. If specified as partition key,
            ///  the result table(s) is partitoned by the RequestTime column, an
            ///  additional timestamp column representing when the request was received.
            RequestTime = 1,
        }
        impl PartitionKey {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    PartitionKey::Unspecified => "PARTITION_KEY_UNSPECIFIED",
                    PartitionKey::RequestTime => "REQUEST_TIME",
                }
            }
        }
    }
    ///  IAM policy analysis export destination.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Destination {
        ///  Destination on Cloud Storage.
        #[prost(message, tag="1")]
        GcsDestination(GcsDestination),
        ///  Destination on BigQuery.
        #[prost(message, tag="2")]
        BigqueryDestination(BigQueryDestination),
    }
}
///  A request message for
///  \[AssetService.AnalyzeIamPolicyLongrunning][google.cloud.asset.v1.AssetService.AnalyzeIamPolicyLongrunning\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnalyzeIamPolicyLongrunningRequest {
    ///  Required. The request query.
    #[prost(message, optional, tag="1")]
    pub analysis_query: ::core::option::Option<IamPolicyAnalysisQuery>,
    ///  Optional. The name of a saved query, which must be in the format of:
    ///
    ///  * projects/project_number/savedQueries/saved_query_id
    ///  * folders/folder_number/savedQueries/saved_query_id
    ///  * organizations/organization_number/savedQueries/saved_query_id
    ///
    ///  If both `analysis_query` and `saved_analysis_query` are provided, they
    ///  will be merged together with the `saved_analysis_query` as base and
    ///  the `analysis_query` as overrides. For more details of the merge behavior,
    ///  please refer to the
    ///  \[MergeFrom\](<https://developers.google.com/protocol-buffers/docs/reference/cpp/google.protobuf.message#Message.MergeFrom.details>)
    ///  doc.
    ///
    ///  Note that you cannot override primitive fields with default value, such as
    ///  0 or empty string, etc., because we use proto3, which doesn't support field
    ///  presence yet.
    #[prost(string, tag="3")]
    pub saved_analysis_query: ::prost::alloc::string::String,
    ///  Required. Output configuration indicating where the results will be output
    ///  to.
    #[prost(message, optional, tag="2")]
    pub output_config: ::core::option::Option<IamPolicyAnalysisOutputConfig>,
}
///  A response message for
///  \[AssetService.AnalyzeIamPolicyLongrunning][google.cloud.asset.v1.AssetService.AnalyzeIamPolicyLongrunning\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnalyzeIamPolicyLongrunningResponse {
}
///  A saved query which can be shared with others or used later.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SavedQuery {
    ///  The resource name of the saved query. The format must be:
    ///
    ///  * projects/project_number/savedQueries/saved_query_id
    ///  * folders/folder_number/savedQueries/saved_query_id
    ///  * organizations/organization_number/savedQueries/saved_query_id
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    ///  The description of this saved query. This value should be fewer than 255
    ///  characters.
    #[prost(string, tag="2")]
    pub description: ::prost::alloc::string::String,
    ///  Output only. The create time of this saved query.
    #[prost(message, optional, tag="3")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    ///  Output only. The account's email address who has created this saved query.
    #[prost(string, tag="4")]
    pub creator: ::prost::alloc::string::String,
    ///  Output only. The last update time of this saved query.
    #[prost(message, optional, tag="5")]
    pub last_update_time: ::core::option::Option<::prost_types::Timestamp>,
    ///  Output only. The account's email address who has updated this saved query
    ///  most recently.
    #[prost(string, tag="6")]
    pub last_updater: ::prost::alloc::string::String,
    ///  Labels applied on the resource.
    ///  This value should not contain more than 10 entries. The key and value of
    ///  each entry must be non-empty and fewer than 64 characters.
    #[prost(map="string, string", tag="7")]
    pub labels: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    ///  The query content.
    #[prost(message, optional, tag="8")]
    pub content: ::core::option::Option<saved_query::QueryContent>,
}
/// Nested message and enum types in `SavedQuery`.
pub mod saved_query {
    ///  The query content.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct QueryContent {
        #[prost(oneof="query_content::QueryContent", tags="1")]
        pub query_content: ::core::option::Option<query_content::QueryContent>,
    }
    /// Nested message and enum types in `QueryContent`.
    pub mod query_content {
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum QueryContent {
            ///  An IAM Policy Analysis query, which could be used in
            ///  the
            ///  \[AssetService.AnalyzeIamPolicy][google.cloud.asset.v1.AssetService.AnalyzeIamPolicy\]
            ///  rpc or the
            ///  \[AssetService.AnalyzeIamPolicyLongrunning][google.cloud.asset.v1.AssetService.AnalyzeIamPolicyLongrunning\]
            ///  rpc.
            #[prost(message, tag="1")]
            IamPolicyAnalysisQuery(super::super::IamPolicyAnalysisQuery),
        }
    }
}
///  Request to create a saved query.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateSavedQueryRequest {
    ///  Required. The name of the project/folder/organization where this
    ///  saved_query should be created in. It can only be an organization number
    ///  (such as "organizations/123"), a folder number (such as "folders/123"), a
    ///  project ID (such as "projects/my-project-id")", or a project number (such
    ///  as "projects/12345").
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    ///  Required. The saved_query details. The `name` field must be empty as it
    ///  will be generated based on the parent and saved_query_id.
    #[prost(message, optional, tag="2")]
    pub saved_query: ::core::option::Option<SavedQuery>,
    ///  Required. The ID to use for the saved query, which must be unique in the
    ///  specified parent. It will become the final component of the saved query's
    ///  resource name.
    ///
    ///  This value should be 4-63 characters, and valid characters
    ///  are /\[a-z][0-9\]-/.
    ///
    ///  Notice that this field is required in the saved query creation, and the
    ///  `name` field of the `saved_query` will be ignored.
    #[prost(string, tag="3")]
    pub saved_query_id: ::prost::alloc::string::String,
}
///  Request to get a saved query.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSavedQueryRequest {
    ///  Required. The name of the saved query and it must be in the format of:
    ///
    ///  * projects/project_number/savedQueries/saved_query_id
    ///  * folders/folder_number/savedQueries/saved_query_id
    ///  * organizations/organization_number/savedQueries/saved_query_id
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
///  Request to list saved queries.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSavedQueriesRequest {
    ///  Required. The parent project/folder/organization whose savedQueries are to
    ///  be listed. It can only be using project/folder/organization number (such as
    ///  "folders/12345")", or a project ID (such as "projects/my-project-id").
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    ///  Optional. The expression to filter resources.
    ///  The expression is a list of zero or more restrictions combined via logical
    ///  operators `AND` and `OR`. When `AND` and `OR` are both used in the
    ///  expression, parentheses must be appropriately used to group the
    ///  combinations. The expression may also contain regular expressions.
    ///
    ///  See <https://google.aip.dev/160> for more information on the grammar.
    #[prost(string, tag="4")]
    pub filter: ::prost::alloc::string::String,
    ///  Optional. The maximum number of saved queries to return per page. The
    ///  service may return fewer than this value. If unspecified, at most 50 will
    ///  be returned.
    ///   The maximum value is 1000; values above 1000 will be coerced to 1000.
    #[prost(int32, tag="2")]
    pub page_size: i32,
    ///  Optional. A page token, received from a previous `ListSavedQueries` call.
    ///  Provide this to retrieve the subsequent page.
    ///
    ///  When paginating, all other parameters provided to `ListSavedQueries` must
    ///  match the call that provided the page token.
    #[prost(string, tag="3")]
    pub page_token: ::prost::alloc::string::String,
}
///  Response of listing saved queries.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSavedQueriesResponse {
    ///  A list of savedQueries.
    #[prost(message, repeated, tag="1")]
    pub saved_queries: ::prost::alloc::vec::Vec<SavedQuery>,
    ///  A token, which can be sent as `page_token` to retrieve the next page.
    ///  If this field is omitted, there are no subsequent pages.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
///  Request to update a saved query.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateSavedQueryRequest {
    ///  Required. The saved query to update.
    ///
    ///  The saved query's `name` field is used to identify the one to update,
    ///  which has format as below:
    ///
    ///  * projects/project_number/savedQueries/saved_query_id
    ///  * folders/folder_number/savedQueries/saved_query_id
    ///  * organizations/organization_number/savedQueries/saved_query_id
    #[prost(message, optional, tag="1")]
    pub saved_query: ::core::option::Option<SavedQuery>,
    ///  Required. The list of fields to update.
    #[prost(message, optional, tag="2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
///  Request to delete a saved query.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteSavedQueryRequest {
    ///  Required. The name of the saved query to delete. It must be in the format
    ///  of:
    ///
    ///  * projects/project_number/savedQueries/saved_query_id
    ///  * folders/folder_number/savedQueries/saved_query_id
    ///  * organizations/organization_number/savedQueries/saved_query_id
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
///  The request message for performing resource move analysis.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnalyzeMoveRequest {
    ///  Required. Name of the resource to perform the analysis against.
    ///  Only GCP Project are supported as of today. Hence, this can only be Project
    ///  ID (such as "projects/my-project-id") or a Project Number (such as
    ///  "projects/12345").
    #[prost(string, tag="1")]
    pub resource: ::prost::alloc::string::String,
    ///  Required. Name of the GCP Folder or Organization to reparent the target
    ///  resource. The analysis will be performed against hypothetically moving the
    ///  resource to this specified desitination parent. This can only be a Folder
    ///  number (such as "folders/123") or an Organization number (such as
    ///  "organizations/123").
    #[prost(string, tag="2")]
    pub destination_parent: ::prost::alloc::string::String,
    ///  Analysis view indicating what information should be included in the
    ///  analysis response. If unspecified, the default view is FULL.
    #[prost(enumeration="analyze_move_request::AnalysisView", tag="3")]
    pub view: i32,
}
/// Nested message and enum types in `AnalyzeMoveRequest`.
pub mod analyze_move_request {
    ///  View enum for supporting partial analysis responses.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum AnalysisView {
        ///  The default/unset value.
        ///  The API will default to the FULL view.
        Unspecified = 0,
        ///  Full analysis including all level of impacts of the specified resource
        ///  move.
        Full = 1,
        ///  Basic analysis only including blockers which will prevent the specified
        ///  resource move at runtime.
        Basic = 2,
    }
    impl AnalysisView {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                AnalysisView::Unspecified => "ANALYSIS_VIEW_UNSPECIFIED",
                AnalysisView::Full => "FULL",
                AnalysisView::Basic => "BASIC",
            }
        }
    }
}
///  The response message for resource move analysis.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnalyzeMoveResponse {
    ///  The list of analyses returned from performing the intended resource move
    ///  analysis. The analysis is grouped by different Cloud services.
    #[prost(message, repeated, tag="1")]
    pub move_analysis: ::prost::alloc::vec::Vec<MoveAnalysis>,
}
///  A message to group the analysis information.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MoveAnalysis {
    ///  The user friendly display name of the analysis. E.g. IAM, Organization
    ///  Policy etc.
    #[prost(string, tag="1")]
    pub display_name: ::prost::alloc::string::String,
    #[prost(oneof="move_analysis::Result", tags="2, 3")]
    pub result: ::core::option::Option<move_analysis::Result>,
}
/// Nested message and enum types in `MoveAnalysis`.
pub mod move_analysis {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Result {
        ///  Analysis result of moving the target resource.
        #[prost(message, tag="2")]
        Analysis(super::MoveAnalysisResult),
        ///  Description of error encountered when performing the analysis.
        #[prost(message, tag="3")]
        Error(super::super::super::super::rpc::Status),
    }
}
///  An analysis result including blockers and warnings.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MoveAnalysisResult {
    ///  Blocking information that would prevent the target resource from moving
    ///  to the specified destination at runtime.
    #[prost(message, repeated, tag="1")]
    pub blockers: ::prost::alloc::vec::Vec<MoveImpact>,
    ///  Warning information indicating that moving the target resource to the
    ///  specified destination might be unsafe. This can include important policy
    ///  information and configuration changes, but will not block moves at runtime.
    #[prost(message, repeated, tag="2")]
    pub warnings: ::prost::alloc::vec::Vec<MoveImpact>,
}
///  A message to group impacts of moving the target resource.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MoveImpact {
    ///  User friendly impact detail in a free form message.
    #[prost(string, tag="1")]
    pub detail: ::prost::alloc::string::String,
}
///  Output configuration query assets.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAssetsOutputConfig {
    ///  BigQuery destination where the query results will be saved.
    #[prost(message, optional, tag="1")]
    pub bigquery_destination: ::core::option::Option<query_assets_output_config::BigQueryDestination>,
}
/// Nested message and enum types in `QueryAssetsOutputConfig`.
pub mod query_assets_output_config {
    ///  BigQuery destination.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct BigQueryDestination {
        ///  Required. The BigQuery dataset where the query results will be saved. It
        ///  has the format of "projects/{projectId}/datasets/{datasetId}".
        #[prost(string, tag="1")]
        pub dataset: ::prost::alloc::string::String,
        ///  Required. The BigQuery table where the query results will be saved. If
        ///  this table does not exist, a new table with the given name will be
        ///  created.
        #[prost(string, tag="2")]
        pub table: ::prost::alloc::string::String,
        ///  Specifies the action that occurs if the destination table or partition
        ///  already exists. The following values are supported:
        ///
        ///  * WRITE_TRUNCATE: If the table or partition already exists, BigQuery
        ///  overwrites the entire table or all the partitions data.
        ///  * WRITE_APPEND: If the table or partition already exists, BigQuery
        ///  appends the data to the table or the latest partition.
        ///  * WRITE_EMPTY: If the table already exists and contains data, an error is
        ///  returned.
        #[prost(string, tag="3")]
        pub write_disposition: ::prost::alloc::string::String,
    }
}
///  QueryAssets request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAssetsRequest {
    ///  Required. The relative name of the root asset. This can only be an
    ///  organization number (such as "organizations/123"), a project ID (such as
    ///  "projects/my-project-id"), or a project number (such as "projects/12345"),
    ///  or a folder number (such as "folders/123").
    ///
    ///  Only assets belonging to the `parent` will be returned.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    ///  Optional. The maximum number of rows to return in the results. Responses
    ///  are limited to 10 MB and 1000 rows.
    ///
    ///  By default, the maximum row count is 1000. When the byte or row count limit
    ///  is reached, the rest of the query results will be paginated.
    ///
    ///  The field will be ignored when \[output_config\] is specified.
    #[prost(int32, tag="4")]
    pub page_size: i32,
    ///  Optional. A page token received from previous `QueryAssets`.
    ///
    ///  The field will be ignored when \[output_config\] is specified.
    #[prost(string, tag="5")]
    pub page_token: ::prost::alloc::string::String,
    ///  Optional. Specifies the maximum amount of time that the client is willing
    ///  to wait for the query to complete. By default, this limit is 5 min for the
    ///  first query, and 1 minute for the following queries. If the query is
    ///  complete, the `done` field in the `QueryAssetsResponse` is true, otherwise
    ///  false.
    ///
    ///  Like BigQuery [jobs.query
    ///  API](<https://cloud.google.com/bigquery/docs/reference/rest/v2/jobs/query#queryrequest>)
    ///  The call is not guaranteed to wait for the specified timeout; it typically
    ///  returns after around 200 seconds (200,000 milliseconds), even if the query
    ///  is not complete.
    ///
    ///  The field will be ignored when \[output_config\] is specified.
    #[prost(message, optional, tag="6")]
    pub timeout: ::core::option::Option<::prost_types::Duration>,
    ///  Optional. Destination where the query results will be saved.
    ///
    ///  When this field is specified, the query results won't be saved in the
    ///  \[QueryAssetsResponse.query_result\]. Instead
    ///  \[QueryAssetsResponse.output_config\] will be set.
    ///
    ///  Meanwhile, \[QueryAssetsResponse.job_reference\] will be set and can be used
    ///  to check the status of the query job when passed to a following
    ///  \[QueryAssets\] API call.
    #[prost(message, optional, tag="9")]
    pub output_config: ::core::option::Option<QueryAssetsOutputConfig>,
    #[prost(oneof="query_assets_request::Query", tags="2, 3")]
    pub query: ::core::option::Option<query_assets_request::Query>,
    ///  Specifies what time period or point in time to query asset metadata at.
    ///  * unset - query asset metadata as it is right now
    ///  * \[read_time_window\] - query asset metadata as it was at any point in time
    ///  between \[start_time\] and \[end_time\].
    ///  * \[read_time\] - query asset metadata as it was at that point in time.
    ///  If data for the timestamp/date range selected does not exist,
    ///  it will simply return a valid response with no rows.
    #[prost(oneof="query_assets_request::Time", tags="7, 8")]
    pub time: ::core::option::Option<query_assets_request::Time>,
}
/// Nested message and enum types in `QueryAssetsRequest`.
pub mod query_assets_request {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Query {
        ///  Optional. A SQL statement that's compatible with [BigQuery Standard
        ///  SQL](<http://cloud/bigquery/docs/reference/standard-sql/enabling-standard-sql>).
        #[prost(string, tag="2")]
        Statement(::prost::alloc::string::String),
        ///  Optional. Reference to the query job, which is from the
        ///  `QueryAssetsResponse` of previous `QueryAssets` call.
        #[prost(string, tag="3")]
        JobReference(::prost::alloc::string::String),
    }
    ///  Specifies what time period or point in time to query asset metadata at.
    ///  * unset - query asset metadata as it is right now
    ///  * \[read_time_window\] - query asset metadata as it was at any point in time
    ///  between \[start_time\] and \[end_time\].
    ///  * \[read_time\] - query asset metadata as it was at that point in time.
    ///  If data for the timestamp/date range selected does not exist,
    ///  it will simply return a valid response with no rows.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Time {
        ///  Optional. \[start_time\] is required. \[start_time\] must be less than
        ///  \[end_time\] Defaults \[end_time\] to now if \[start_time\] is set and
        ///  \[end_time\] isn't. Maximum permitted time range is 7 days.
        #[prost(message, tag="7")]
        ReadTimeWindow(super::TimeWindow),
        ///  Optional. Queries cloud assets as they appeared at the specified point in
        ///  time.
        #[prost(message, tag="8")]
        ReadTime(::prost_types::Timestamp),
    }
}
///  QueryAssets response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAssetsResponse {
    ///  Reference to a query job.
    #[prost(string, tag="1")]
    pub job_reference: ::prost::alloc::string::String,
    ///  The query response, which can be either an `error` or a valid `response`.
    ///
    ///  If `done` == `false` and the query result is being saved in a output, the
    ///  output_config field will be set.
    ///  If `done` == `true`, exactly one of
    ///  `error`, `query_result` or `output_config` will be set.
    #[prost(bool, tag="2")]
    pub done: bool,
    #[prost(oneof="query_assets_response::Response", tags="3, 4, 5")]
    pub response: ::core::option::Option<query_assets_response::Response>,
}
/// Nested message and enum types in `QueryAssetsResponse`.
pub mod query_assets_response {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Response {
        ///  Error status.
        #[prost(message, tag="3")]
        Error(super::super::super::super::rpc::Status),
        ///  Result of the query.
        #[prost(message, tag="4")]
        QueryResult(super::QueryResult),
        ///  Output configuration which indicates instead of being returned in API
        ///  response on the fly, the query result will be saved in a specific output.
        #[prost(message, tag="5")]
        OutputConfig(super::QueryAssetsOutputConfig),
    }
}
///  Execution results of the query.
///
///  The result is formatted as rows represented by BigQuery compatible \[schema\].
///  When pagination is necessary, it will contains the page token to retrieve
///  the results of following pages.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryResult {
    ///  Each row hold a query result in the format of `Struct`.
    #[prost(message, repeated, tag="1")]
    pub rows: ::prost::alloc::vec::Vec<::prost_types::Struct>,
    ///  Describes the format of the \[rows\].
    #[prost(message, optional, tag="2")]
    pub schema: ::core::option::Option<TableSchema>,
    ///  Token to retrieve the next page of the results.
    #[prost(string, tag="3")]
    pub next_page_token: ::prost::alloc::string::String,
    ///  Total rows of the whole query results.
    #[prost(int64, tag="4")]
    pub total_rows: i64,
}
///  BigQuery Compatible table schema.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TableSchema {
    ///  Describes the fields in a table.
    #[prost(message, repeated, tag="1")]
    pub fields: ::prost::alloc::vec::Vec<TableFieldSchema>,
}
///  A field in TableSchema.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TableFieldSchema {
    ///  The field name. The name must contain only letters (a-z, A-Z),
    ///  numbers (0-9), or underscores (_), and must start with a letter or
    ///  underscore. The maximum length is 128 characters.
    #[prost(string, tag="1")]
    pub field: ::prost::alloc::string::String,
    ///  The field data type. Possible values include
    ///  * STRING
    ///  * BYTES
    ///  * INTEGER
    ///  * FLOAT
    ///  * BOOLEAN
    ///  * TIMESTAMP
    ///  * DATE
    ///  * TIME
    ///  * DATETIME
    ///  * GEOGRAPHY,
    ///  * NUMERIC,
    ///  * BIGNUMERIC,
    ///  * RECORD
    ///  (where RECORD indicates that the field contains a nested schema).
    #[prost(string, tag="2")]
    pub r#type: ::prost::alloc::string::String,
    ///  The field mode. Possible values include NULLABLE, REQUIRED and
    ///  REPEATED. The default value is NULLABLE.
    #[prost(string, tag="3")]
    pub mode: ::prost::alloc::string::String,
    ///  Describes the nested schema fields if the type property is set
    ///  to RECORD.
    #[prost(message, repeated, tag="4")]
    pub fields: ::prost::alloc::vec::Vec<TableFieldSchema>,
}
///  A request message for
///  \[AssetService.BatchGetEffectiveIamPolicies][google.cloud.asset.v1.AssetService.BatchGetEffectiveIamPolicies\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchGetEffectiveIamPoliciesRequest {
    ///  Required. Only IAM policies on or below the scope will be returned.
    ///
    ///  This can only be an organization number (such as "organizations/123"), a
    ///  folder number (such as "folders/123"), a project ID (such as
    ///  "projects/my-project-id"), or a project number (such as "projects/12345").
    ///
    ///  To know how to get organization id, visit [here
    ///  ](<https://cloud.google.com/resource-manager/docs/creating-managing-organization#retrieving_your_organization_id>).
    ///
    ///  To know how to get folder or project id, visit [here
    ///  ](<https://cloud.google.com/resource-manager/docs/creating-managing-folders#viewing_or_listing_folders_and_projects>).
    #[prost(string, tag="1")]
    pub scope: ::prost::alloc::string::String,
    ///  Required. The names refer to the \[full_resource_names\]
    ///  (<https://cloud.google.com/asset-inventory/docs/resource-name-format>)
    ///  of [searchable asset
    ///  types](<https://cloud.google.com/asset-inventory/docs/supported-asset-types#searchable_asset_types>).
    ///  A maximum of 20 resources' effective policies can be retrieved in a batch.
    #[prost(string, repeated, tag="3")]
    pub names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
///  A response message for
///  \[AssetService.BatchGetEffectiveIamPolicies][google.cloud.asset.v1.AssetService.BatchGetEffectiveIamPolicies\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchGetEffectiveIamPoliciesResponse {
    ///  The effective policies for a batch of resources. Note that the results
    ///  order is the same as the order of
    ///  \[BatchGetEffectiveIamPoliciesRequest.names][google.cloud.asset.v1.BatchGetEffectiveIamPoliciesRequest.names\].
    ///  When a resource does not have any effective IAM policies, its corresponding
    ///  policy_result will contain empty
    ///  \[EffectiveIamPolicy.policies][google.cloud.asset.v1.BatchGetEffectiveIamPoliciesResponse.EffectiveIamPolicy.policies\].
    #[prost(message, repeated, tag="2")]
    pub policy_results: ::prost::alloc::vec::Vec<batch_get_effective_iam_policies_response::EffectiveIamPolicy>,
}
/// Nested message and enum types in `BatchGetEffectiveIamPoliciesResponse`.
pub mod batch_get_effective_iam_policies_response {
    ///  The effective IAM policies on one resource.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct EffectiveIamPolicy {
        ///  The \[full_resource_name\]
        ///  (<https://cloud.google.com/asset-inventory/docs/resource-name-format>)
        ///  for which the
        ///  \[policies][google.cloud.asset.v1.BatchGetEffectiveIamPoliciesResponse.EffectiveIamPolicy.policies\]
        ///  are computed. This is one of the
        ///  \[BatchGetEffectiveIamPoliciesRequest.names][google.cloud.asset.v1.BatchGetEffectiveIamPoliciesRequest.names\]
        ///  the caller provides in the request.
        #[prost(string, tag="1")]
        pub full_resource_name: ::prost::alloc::string::String,
        ///  The effective policies for the
        ///  \[full_resource_name][google.cloud.asset.v1.BatchGetEffectiveIamPoliciesResponse.EffectiveIamPolicy.full_resource_name\].
        ///
        ///  These policies include the policy set on the
        ///  \[full_resource_name][google.cloud.asset.v1.BatchGetEffectiveIamPoliciesResponse.EffectiveIamPolicy.full_resource_name\]
        ///  and those set on its parents and ancestors up to the
        ///  \[BatchGetEffectiveIamPoliciesRequest.scope][google.cloud.asset.v1.BatchGetEffectiveIamPoliciesRequest.scope\].
        ///  Note that these policies are not filtered according to the resource type
        ///  of the
        ///  \[full_resource_name][google.cloud.asset.v1.BatchGetEffectiveIamPoliciesResponse.EffectiveIamPolicy.full_resource_name\].
        ///
        ///  These policies are hierarchically ordered by
        ///  \[PolicyInfo.attached_resource][google.cloud.asset.v1.BatchGetEffectiveIamPoliciesResponse.EffectiveIamPolicy.PolicyInfo.attached_resource\]
        ///  starting from
        ///  \[full_resource_name][google.cloud.asset.v1.BatchGetEffectiveIamPoliciesResponse.EffectiveIamPolicy.full_resource_name\]
        ///  itself to its parents and ancestors, such that policies\[i\]'s
        ///  \[PolicyInfo.attached_resource][google.cloud.asset.v1.BatchGetEffectiveIamPoliciesResponse.EffectiveIamPolicy.PolicyInfo.attached_resource\]
        ///  is the child of policies\[i+1\]'s
        ///  \[PolicyInfo.attached_resource][google.cloud.asset.v1.BatchGetEffectiveIamPoliciesResponse.EffectiveIamPolicy.PolicyInfo.attached_resource\],
        ///  if policies\[i+1\] exists.
        #[prost(message, repeated, tag="2")]
        pub policies: ::prost::alloc::vec::Vec<effective_iam_policy::PolicyInfo>,
    }
    /// Nested message and enum types in `EffectiveIamPolicy`.
    pub mod effective_iam_policy {
        ///  The IAM policy and its attached resource.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct PolicyInfo {
            ///  The full resource name the
            ///  \[policy][google.cloud.asset.v1.BatchGetEffectiveIamPoliciesResponse.EffectiveIamPolicy.PolicyInfo.policy\]
            ///  is directly attached to.
            #[prost(string, tag="1")]
            pub attached_resource: ::prost::alloc::string::String,
            ///  The IAM policy that's directly attached to the
            ///  \[attached_resource][google.cloud.asset.v1.BatchGetEffectiveIamPoliciesResponse.EffectiveIamPolicy.PolicyInfo.attached_resource\].
            #[prost(message, optional, tag="2")]
            pub policy: ::core::option::Option<super::super::super::super::super::iam::v1::Policy>,
        }
    }
}
///  Asset content type.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ContentType {
    ///  Unspecified content type.
    Unspecified = 0,
    ///  Resource metadata.
    Resource = 1,
    ///  The actual IAM policy set on a resource.
    IamPolicy = 2,
    ///  The Cloud Organization Policy set on an asset.
    OrgPolicy = 4,
    ///  The Cloud Access context manager Policy set on an asset.
    AccessPolicy = 5,
    ///  The runtime OS Inventory information.
    OsInventory = 6,
    ///  The related resources.
    Relationship = 7,
}
impl ContentType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ContentType::Unspecified => "CONTENT_TYPE_UNSPECIFIED",
            ContentType::Resource => "RESOURCE",
            ContentType::IamPolicy => "IAM_POLICY",
            ContentType::OrgPolicy => "ORG_POLICY",
            ContentType::AccessPolicy => "ACCESS_POLICY",
            ContentType::OsInventory => "OS_INVENTORY",
            ContentType::Relationship => "RELATIONSHIP",
        }
    }
}
/// Generated client implementations.
pub mod asset_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Asset service definition.
    #[derive(Debug, Clone)]
    pub struct AssetServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl AssetServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> AssetServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> AssetServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            AssetServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Exports assets with time and resource types to a given Cloud Storage
        /// location/BigQuery table. For Cloud Storage location destinations, the
        /// output format is newline-delimited JSON. Each line represents a
        /// [google.cloud.asset.v1.Asset][google.cloud.asset.v1.Asset] in the JSON
        /// format; for BigQuery table destinations, the output table stores the fields
        /// in asset Protobuf as columns. This API implements the
        /// [google.longrunning.Operation][google.longrunning.Operation] API, which
        /// allows you to keep track of the export. We recommend intervals of at least
        /// 2 seconds with exponential retry to poll the export operation result. For
        /// regular-size resource parent, the export operation usually finishes within
        /// 5 minutes.
        pub async fn export_assets(
            &mut self,
            request: impl tonic::IntoRequest<super::ExportAssetsRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
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
        /// Lists assets with time and resource types and returns paged results in
        /// response.
        pub async fn list_assets(
            &mut self,
            request: impl tonic::IntoRequest<super::ListAssetsRequest>,
        ) -> Result<tonic::Response<super::ListAssetsResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.asset.v1.AssetService/ListAssets",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Batch gets the update history of assets that overlap a time window.
        /// For IAM_POLICY content, this API outputs history when the asset and its
        /// attached IAM POLICY both exist. This can create gaps in the output history.
        /// Otherwise, this API outputs history with asset in both non-delete or
        /// deleted status.
        /// If a specified asset does not exist, this API returns an INVALID_ARGUMENT
        /// error.
        pub async fn batch_get_assets_history(
            &mut self,
            request: impl tonic::IntoRequest<super::BatchGetAssetsHistoryRequest>,
        ) -> Result<
            tonic::Response<super::BatchGetAssetsHistoryResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
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
        /// Creates a feed in a parent project/folder/organization to listen to its
        /// asset updates.
        pub async fn create_feed(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateFeedRequest>,
        ) -> Result<tonic::Response<super::Feed>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
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
        /// Gets details about an asset feed.
        pub async fn get_feed(
            &mut self,
            request: impl tonic::IntoRequest<super::GetFeedRequest>,
        ) -> Result<tonic::Response<super::Feed>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.asset.v1.AssetService/GetFeed",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists all asset feeds in a parent project/folder/organization.
        pub async fn list_feeds(
            &mut self,
            request: impl tonic::IntoRequest<super::ListFeedsRequest>,
        ) -> Result<tonic::Response<super::ListFeedsResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
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
        /// Updates an asset feed configuration.
        pub async fn update_feed(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateFeedRequest>,
        ) -> Result<tonic::Response<super::Feed>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
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
        /// Deletes an asset feed.
        pub async fn delete_feed(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteFeedRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
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
        /// Searches all Cloud resources within the specified scope, such as a project,
        /// folder, or organization. The caller must be granted the
        /// `cloudasset.assets.searchAllResources` permission on the desired scope,
        /// otherwise the request will be rejected.
        pub async fn search_all_resources(
            &mut self,
            request: impl tonic::IntoRequest<super::SearchAllResourcesRequest>,
        ) -> Result<tonic::Response<super::SearchAllResourcesResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
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
        /// Searches all IAM policies within the specified scope, such as a project,
        /// folder, or organization. The caller must be granted the
        /// `cloudasset.assets.searchAllIamPolicies` permission on the desired scope,
        /// otherwise the request will be rejected.
        pub async fn search_all_iam_policies(
            &mut self,
            request: impl tonic::IntoRequest<super::SearchAllIamPoliciesRequest>,
        ) -> Result<
            tonic::Response<super::SearchAllIamPoliciesResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
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
        /// Analyzes IAM policies to answer which identities have what accesses on
        /// which resources.
        pub async fn analyze_iam_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::AnalyzeIamPolicyRequest>,
        ) -> Result<tonic::Response<super::AnalyzeIamPolicyResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
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
        /// Analyzes IAM policies asynchronously to answer which identities have what
        /// accesses on which resources, and writes the analysis results to a Google
        /// Cloud Storage or a BigQuery destination. For Cloud Storage destination, the
        /// output format is the JSON format that represents a
        /// [AnalyzeIamPolicyResponse][google.cloud.asset.v1.AnalyzeIamPolicyResponse].
        /// This method implements the
        /// [google.longrunning.Operation][google.longrunning.Operation], which allows
        /// you to track the operation status. We recommend intervals of at least 2
        /// seconds with exponential backoff retry to poll the operation result. The
        /// metadata contains the metadata for the long-running operation.
        pub async fn analyze_iam_policy_longrunning(
            &mut self,
            request: impl tonic::IntoRequest<super::AnalyzeIamPolicyLongrunningRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
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
        /// Analyze moving a resource to a specified destination without kicking off
        /// the actual move. The analysis is best effort depending on the user's
        /// permissions of viewing different hierarchical policies and configurations.
        /// The policies and configuration are subject to change before the actual
        /// resource migration takes place.
        pub async fn analyze_move(
            &mut self,
            request: impl tonic::IntoRequest<super::AnalyzeMoveRequest>,
        ) -> Result<tonic::Response<super::AnalyzeMoveResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.asset.v1.AssetService/AnalyzeMove",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Issue a job that queries assets using a SQL statement compatible with
        /// [BigQuery Standard
        /// SQL](http://cloud/bigquery/docs/reference/standard-sql/enabling-standard-sql).
        ///
        /// If the query execution finishes within timeout and there's no pagination,
        /// the full query results will be returned in the `QueryAssetsResponse`.
        ///
        /// Otherwise, full query results can be obtained by issuing extra requests
        /// with the `job_reference` from the a previous `QueryAssets` call.
        ///
        /// Note, the query result has approximately 10 GB limitation enforced by
        /// BigQuery
        /// https://cloud.google.com/bigquery/docs/best-practices-performance-output,
        /// queries return larger results will result in errors.
        pub async fn query_assets(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryAssetsRequest>,
        ) -> Result<tonic::Response<super::QueryAssetsResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.asset.v1.AssetService/QueryAssets",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates a saved query in a parent project/folder/organization.
        pub async fn create_saved_query(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateSavedQueryRequest>,
        ) -> Result<tonic::Response<super::SavedQuery>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.asset.v1.AssetService/CreateSavedQuery",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets details about a saved query.
        pub async fn get_saved_query(
            &mut self,
            request: impl tonic::IntoRequest<super::GetSavedQueryRequest>,
        ) -> Result<tonic::Response<super::SavedQuery>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.asset.v1.AssetService/GetSavedQuery",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists all saved queries in a parent project/folder/organization.
        pub async fn list_saved_queries(
            &mut self,
            request: impl tonic::IntoRequest<super::ListSavedQueriesRequest>,
        ) -> Result<tonic::Response<super::ListSavedQueriesResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.asset.v1.AssetService/ListSavedQueries",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates a saved query.
        pub async fn update_saved_query(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateSavedQueryRequest>,
        ) -> Result<tonic::Response<super::SavedQuery>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.asset.v1.AssetService/UpdateSavedQuery",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes a saved query.
        pub async fn delete_saved_query(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteSavedQueryRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.asset.v1.AssetService/DeleteSavedQuery",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets effective IAM policies for a batch of resources.
        pub async fn batch_get_effective_iam_policies(
            &mut self,
            request: impl tonic::IntoRequest<super::BatchGetEffectiveIamPoliciesRequest>,
        ) -> Result<
            tonic::Response<super::BatchGetEffectiveIamPoliciesResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.asset.v1.AssetService/BatchGetEffectiveIamPolicies",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
