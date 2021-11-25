/// Network configuration for a Data Fusion instance. These configurations
/// are used for peering with the customer network. Configurations are optional
/// when a public Data Fusion instance is to be created. However, providing
/// these configurations allows several benefits, such as reduced network latency
/// while accessing the customer resources from managed Data Fusion instance
/// nodes, as well as access to the customer on-prem resources.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NetworkConfig {
    /// Name of the network in the customer project with which the Tenant Project
    /// will be peered for executing pipelines. In case of shared VPC where the
    /// network resides in another host project the network should specified in
    /// the form of projects/{host-project-id}/global/networks/{network}
    #[prost(string, tag = "1")]
    pub network: ::prost::alloc::string::String,
    /// The IP range in CIDR notation to use for the managed Data Fusion instance
    /// nodes. This range must not overlap with any other ranges used in the Data
    /// Fusion instance network.
    #[prost(string, tag = "2")]
    pub ip_allocation: ::prost::alloc::string::String,
}
/// The Data Fusion version.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Version {
    /// The version number of the Data Fusion instance, such as '6.0.1.0'.
    #[prost(string, tag = "1")]
    pub version_number: ::prost::alloc::string::String,
    /// Whether this is currently the default version for Cloud Data Fusion
    #[prost(bool, tag = "2")]
    pub default_version: bool,
    /// Represents a list of available feature names for a given version.
    #[prost(string, repeated, tag = "3")]
    pub available_features: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Identifies Data Fusion accelerators for an instance.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Accelerator {
    /// The type of an accelator for a CDF instance.
    #[prost(enumeration = "accelerator::AcceleratorType", tag = "1")]
    pub accelerator_type: i32,
}
/// Nested message and enum types in `Accelerator`.
pub mod accelerator {
    /// Each type represents an Accelerator (Add-On) supported by Cloud Data Fusion
    /// service.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum AcceleratorType {
        /// Default value, if unspecified.
        Unspecified = 0,
        /// Change Data Capture accelerator for CDF.
        Cdc = 1,
        /// Cloud Healthcare accelerator for CDF. This accelerator is to enable Cloud
        /// Healthcare specific CDF plugins developed by Healthcare team.
        Healthcare = 2,
    }
}
/// The crypto key configuration. This field is used by the Customer-managed
/// encryption keys (CMEK) feature.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CryptoKeyConfig {
    /// The name of the key which is used to encrypt/decrypt customer data. For key
    /// in Cloud KMS, the key should be in the format of
    /// `projects/*/locations/*/keyRings/*/cryptoKeys/*`.
    #[prost(string, tag = "1")]
    pub key_reference: ::prost::alloc::string::String,
}
/// Represents a Data Fusion instance.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Instance {
    /// Output only. The name of this instance is in the form of
    /// projects/{project}/locations/{location}/instances/{instance}.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// A description of this instance.
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// Required. Instance type.
    #[prost(enumeration = "instance::Type", tag = "3")]
    pub r#type: i32,
    /// Option to enable Stackdriver Logging.
    #[prost(bool, tag = "4")]
    pub enable_stackdriver_logging: bool,
    /// Option to enable Stackdriver Monitoring.
    #[prost(bool, tag = "5")]
    pub enable_stackdriver_monitoring: bool,
    /// Specifies whether the Data Fusion instance should be private. If set to
    /// true, all Data Fusion nodes will have private IP addresses and will not be
    /// able to access the public internet.
    #[prost(bool, tag = "6")]
    pub private_instance: bool,
    /// Network configuration options. These are required when a private Data
    /// Fusion instance is to be created.
    #[prost(message, optional, tag = "7")]
    pub network_config: ::core::option::Option<NetworkConfig>,
    /// The resource labels for instance to use to annotate any related underlying
    /// resources such as Compute Engine VMs. The character '=' is not allowed to
    /// be used within the labels.
    #[prost(map = "string, string", tag = "8")]
    pub labels:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Map of additional options used to configure the behavior of
    /// Data Fusion instance.
    #[prost(map = "string, string", tag = "9")]
    pub options:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Output only. The time the instance was created.
    #[prost(message, optional, tag = "10")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The time the instance was last updated.
    #[prost(message, optional, tag = "11")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The current state of this Data Fusion instance.
    #[prost(enumeration = "instance::State", tag = "12")]
    pub state: i32,
    /// Output only. Additional information about the current state of this Data
    /// Fusion instance if available.
    #[prost(string, tag = "13")]
    pub state_message: ::prost::alloc::string::String,
    /// Output only. Endpoint on which the Data Fusion UI is accessible.
    #[prost(string, tag = "14")]
    pub service_endpoint: ::prost::alloc::string::String,
    /// Name of the zone in which the Data Fusion instance will be created. Only
    /// DEVELOPER instances use this field.
    #[prost(string, tag = "15")]
    pub zone: ::prost::alloc::string::String,
    /// Current version of Data Fusion.
    #[prost(string, tag = "16")]
    pub version: ::prost::alloc::string::String,
    /// Output only. Deprecated. Use tenant_project_id instead to extract the
    /// tenant project ID.
    #[deprecated]
    #[prost(string, tag = "17")]
    pub service_account: ::prost::alloc::string::String,
    /// Display name for an instance.
    #[prost(string, tag = "18")]
    pub display_name: ::prost::alloc::string::String,
    /// Available versions that the instance can be upgraded to using
    /// UpdateInstanceRequest.
    #[prost(message, repeated, tag = "19")]
    pub available_version: ::prost::alloc::vec::Vec<Version>,
    /// Output only. Endpoint on which the REST APIs is accessible.
    #[prost(string, tag = "20")]
    pub api_endpoint: ::prost::alloc::string::String,
    /// Output only. Cloud Storage bucket generated by Data Fusion in the customer
    /// project.
    #[prost(string, tag = "21")]
    pub gcs_bucket: ::prost::alloc::string::String,
    /// List of accelerators enabled for this CDF instance.
    #[prost(message, repeated, tag = "22")]
    pub accelerators: ::prost::alloc::vec::Vec<Accelerator>,
    /// Output only. P4 service account for the customer project.
    #[prost(string, tag = "23")]
    pub p4_service_account: ::prost::alloc::string::String,
    /// Output only. The name of the tenant project.
    #[prost(string, tag = "24")]
    pub tenant_project_id: ::prost::alloc::string::String,
    /// User-managed service account to set on Dataproc when Cloud Data Fusion
    /// creates Dataproc to run data processing pipelines.
    ///
    /// This allows users to have fine-grained access control on Dataproc's
    /// accesses to cloud resources.
    #[prost(string, tag = "25")]
    pub dataproc_service_account: ::prost::alloc::string::String,
    /// Option to enable granular role-based access control.
    #[prost(bool, tag = "26")]
    pub enable_rbac: bool,
    /// The crypto key configuration. This field is used by the Customer-Managed
    /// Encryption Keys (CMEK) feature.
    #[prost(message, optional, tag = "27")]
    pub crypto_key_config: ::core::option::Option<CryptoKeyConfig>,
}
/// Nested message and enum types in `Instance`.
pub mod instance {
    /// Represents the type of Data Fusion instance. Each type is configured with
    /// the default settings for processing and memory.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Type {
        /// No type specified. The instance creation will fail.
        Unspecified = 0,
        /// Basic Data Fusion instance. In Basic type, the user will be able to
        /// create data pipelines using point and click UI. However, there are
        /// certain limitations, such as fewer number of concurrent pipelines, no
        /// support for streaming pipelines, etc.
        Basic = 1,
        /// Enterprise Data Fusion instance. In Enterprise type, the user will have
        /// all features available, such as support for streaming pipelines,
        /// unlimited number of concurrent pipelines, etc.
        Enterprise = 2,
        /// Developer Data Fusion instance. In Developer type, the user will have all
        /// features available but with restrictive capabilities. This is to help
        /// enterprises design and develop their data ingestion and integration
        /// pipelines at low cost.
        Developer = 3,
    }
    /// Represents the state of a Data Fusion instance
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// Instance does not have a state yet
        Unspecified = 0,
        /// Instance is being created
        Creating = 1,
        /// Instance is running and ready for requests
        Running = 2,
        /// Instance creation failed
        Failed = 3,
        /// Instance is being deleted
        Deleting = 4,
        /// Instance is being upgraded
        Upgrading = 5,
        /// Instance is being restarted
        Restarting = 6,
        /// Instance is being updated on customer request
        Updating = 7,
        /// Instance is being auto-updated
        AutoUpdating = 8,
        /// Instance is being auto-upgraded
        AutoUpgrading = 9,
    }
}
/// Request message for listing Data Fusion instances.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListInstancesRequest {
    /// The project and location for which to retrieve instance information
    /// in the format projects/{project}/locations/{location}. If the location is
    /// specified as '-' (wildcard), then all regions available to the project
    /// are queried, and the results are aggregated.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of items to return.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The next_page_token value to use if there are additional
    /// results to retrieve for this list request.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// List filter.
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Sort results. Supported values are "name", "name desc",  or "" (unsorted).
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
/// Response message for the list instance request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListInstancesResponse {
    /// Represents a list of Data Fusion instances.
    #[prost(message, repeated, tag = "1")]
    pub instances: ::prost::alloc::vec::Vec<Instance>,
    /// Token to retrieve the next page of results or empty if there are no more
    /// results in the list.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Request message for the list available versions request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAvailableVersionsRequest {
    /// Required. The project and location for which to retrieve instance
    /// information in the format projects/{project}/locations/{location}.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of items to return.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The next_page_token value to use if there are additional
    /// results to retrieve for this list request.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Whether or not to return the latest patch of every available minor version.
    /// If true, only the latest patch will be returned. Ex. if allowed versions is
    /// [6.1.1, 6.1.2, 6.2.0] then response will be [6.1.2, 6.2.0]
    #[prost(bool, tag = "4")]
    pub latest_patch_only: bool,
}
/// Response message for the list available versions request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAvailableVersionsResponse {
    /// Represents a list of versions that are supported.
    #[prost(message, repeated, tag = "1")]
    pub available_versions: ::prost::alloc::vec::Vec<Version>,
    /// Token to retrieve the next page of results or empty if there are no more
    /// results in the list.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for getting details about a Data Fusion instance.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetInstanceRequest {
    /// The instance resource name in the format
    /// projects/{project}/locations/{location}/instances/{instance}.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for creating a Data Fusion instance.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateInstanceRequest {
    /// The instance's project and location in the format
    /// projects/{project}/locations/{location}.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The name of the instance to create.
    #[prost(string, tag = "2")]
    pub instance_id: ::prost::alloc::string::String,
    /// An instance resource.
    #[prost(message, optional, tag = "3")]
    pub instance: ::core::option::Option<Instance>,
}
/// Request message for deleting a Data Fusion instance.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteInstanceRequest {
    /// The instance resource name in the format
    /// projects/{project}/locations/{location}/instances/{instance}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for updating a Data Fusion instance.
/// Data Fusion only allows updating the labels, options, and stack driver
/// settings.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateInstanceRequest {
    /// The instance resource that replaces the resource on the server. Currently,
    /// Data Fusion only allows replacing labels, options, and stack driver
    /// settings. All other fields will be ignored.
    #[prost(message, optional, tag = "1")]
    pub instance: ::core::option::Option<Instance>,
    /// Field mask is used to specify the fields that the update will overwrite
    /// in an instance resource. The fields specified in the update_mask are
    /// relative to the resource, not the full request.
    /// A field will be overwritten if it is in the mask.
    /// If the user does not provide a mask, all the supported fields (labels and
    /// options currently) will be overwritten.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for restarting a Data Fusion instance.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RestartInstanceRequest {
    /// Name of the Data Fusion instance which need to be restarted in the form of
    /// projects/{project}/locations/{location}/instances/{instance}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for upgrading a Data Fusion instance.
/// To change the instance properties, instance update should be used.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpgradeInstanceRequest {
    /// Name of the Data Fusion instance which need to be upgraded in the form of
    /// projects/{project}/locations/{location}/instances/{instance}
    /// Instance will be upgraded with the latest stable version of the Data
    /// Fusion.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Represents the metadata of a long-running operation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationMetadata {
    /// The time the operation was created.
    #[prost(message, optional, tag = "1")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The time the operation finished running.
    #[prost(message, optional, tag = "2")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Server-defined resource path for the target of the operation.
    #[prost(string, tag = "3")]
    pub target: ::prost::alloc::string::String,
    /// Name of the verb executed by the operation.
    #[prost(string, tag = "4")]
    pub verb: ::prost::alloc::string::String,
    /// Human-readable status of the operation if any.
    #[prost(string, tag = "5")]
    pub status_detail: ::prost::alloc::string::String,
    /// Identifies whether the user has requested cancellation
    /// of the operation. Operations that have successfully been cancelled
    /// have \[Operation.error][\] value with a
    /// \[google.rpc.Status.code][google.rpc.Status.code\] of 1, corresponding to
    /// `Code.CANCELLED`.
    #[prost(bool, tag = "6")]
    pub requested_cancellation: bool,
    /// API version used to start the operation.
    #[prost(string, tag = "7")]
    pub api_version: ::prost::alloc::string::String,
}
/// Request message for RemoveIamPolicy method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveIamPolicyRequest {
    /// The resource on which IAM policy to be removed is attached to.
    #[prost(string, tag = "1")]
    pub resource: ::prost::alloc::string::String,
}
/// Response message for RemoveIamPolicy method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveIamPolicyResponse {}
/// List namespaces request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListNamespacesRequest {
    /// Required. The instance to list its namespaces.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of items to return.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The next_page_token value to use if there are additional
    /// results to retrieve for this list request.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// By default, only basic information about a namespace is returned
    /// (e.g. name). When `NAMESPACE_VIEW_FULL` is specified, additional
    /// information associated with a namespace gets returned
    /// (e.g. IAM policy set on the namespace)
    #[prost(enumeration = "NamespaceView", tag = "4")]
    pub view: i32,
}
/// IAMPolicy encapsulates the IAM policy name, definition and status of
/// policy fetching.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IamPolicy {
    /// Policy definition if IAM policy fetching is successful,
    /// otherwise empty.
    #[prost(message, optional, tag = "1")]
    pub policy: ::core::option::Option<super::super::super::iam::v1::Policy>,
    /// Status of iam policy fetching.
    #[prost(message, optional, tag = "2")]
    pub status: ::core::option::Option<super::super::super::rpc::Status>,
}
/// Represents the information of a namespace
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Namespace {
    /// Name of the given namespace.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// IAM policy associated with this namespace.
    #[prost(message, optional, tag = "2")]
    pub iam_policy: ::core::option::Option<IamPolicy>,
}
/// List namespaces response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListNamespacesResponse {
    /// List of namespaces
    #[prost(message, repeated, tag = "1")]
    pub namespaces: ::prost::alloc::vec::Vec<Namespace>,
    /// Token to retrieve the next page of results or empty if there are no more
    /// results in the list.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// DNS peering configuration. These configurations are used to create
/// DNS peering with the customer Cloud DNS.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DnsPeering {
    /// Required. Name of the zone.
    #[prost(string, tag = "1")]
    pub zone: ::prost::alloc::string::String,
    /// Required. Name of the dns.
    #[prost(string, tag = "2")]
    pub domain: ::prost::alloc::string::String,
    /// Optional. Optional description of the dns zone.
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    /// Optional. Optional target project to which dns peering should happen.
    #[prost(string, tag = "4")]
    pub target_project: ::prost::alloc::string::String,
    /// Optional. Optional target network to which dns peering should happen.
    #[prost(string, tag = "5")]
    pub target_network: ::prost::alloc::string::String,
}
/// Request message to create dns peering.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddDnsPeeringRequest {
    /// The resource on which DNS peering will be created.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Dns peering config.
    #[prost(message, optional, tag = "2")]
    pub dns_peering: ::core::option::Option<DnsPeering>,
}
/// Response message for set dns peering method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddDnsPeeringResponse {}
/// Request message to remove dns peering.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveDnsPeeringRequest {
    /// The resource on which DNS peering will be removed.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The zone to be removed.
    #[prost(string, tag = "2")]
    pub zone: ::prost::alloc::string::String,
}
/// Response message for set dns peering method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveDnsPeeringResponse {}
/// List dns peering request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDnsPeeringsRequest {
    /// Required. The resource on which dns peering will be listed.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of items to return.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The next_page_token value to use if there are additional
    /// results to retrieve for this list request.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// List dns peering response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDnsPeeringsResponse {
    /// List of dns peering configs.
    #[prost(message, repeated, tag = "1")]
    pub dns_peerings: ::prost::alloc::vec::Vec<DnsPeering>,
    /// Token to retrieve the next page of results or empty if there are no more
    /// results in the list.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// A view for Namespace
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum NamespaceView {
    /// Default/unset value, which will use BASIC view.
    Unspecified = 0,
    /// Show the most basic metadata of a namespace
    Basic = 1,
    /// Returns all metadata of a namespace
    Full = 2,
}
#[doc = r" Generated client implementations."]
pub mod data_fusion_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Service for creating and managing Data Fusion instances."]
    #[doc = " Data Fusion enables ETL developers to build code-free, data integration"]
    #[doc = " pipelines via a point-and-click UI."]
    #[derive(Debug, Clone)]
    pub struct DataFusionClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> DataFusionClient<T>
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
        ) -> DataFusionClient<InterceptedService<T, F>>
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
            DataFusionClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Lists possible versions for Data Fusion instances in the specified project"]
        #[doc = " and location."]
        pub async fn list_available_versions(
            &mut self,
            request: impl tonic::IntoRequest<super::ListAvailableVersionsRequest>,
        ) -> Result<tonic::Response<super::ListAvailableVersionsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.datafusion.v1beta1.DataFusion/ListAvailableVersions",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists Data Fusion instances in the specified project and location."]
        pub async fn list_instances(
            &mut self,
            request: impl tonic::IntoRequest<super::ListInstancesRequest>,
        ) -> Result<tonic::Response<super::ListInstancesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.datafusion.v1beta1.DataFusion/ListInstances",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets details of a single Data Fusion instance."]
        pub async fn get_instance(
            &mut self,
            request: impl tonic::IntoRequest<super::GetInstanceRequest>,
        ) -> Result<tonic::Response<super::Instance>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.datafusion.v1beta1.DataFusion/GetInstance",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a new Data Fusion instance in the specified project and location."]
        pub async fn create_instance(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateInstanceRequest>,
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
                "/google.cloud.datafusion.v1beta1.DataFusion/CreateInstance",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes a single Data Fusion instance."]
        pub async fn delete_instance(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteInstanceRequest>,
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
                "/google.cloud.datafusion.v1beta1.DataFusion/DeleteInstance",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates a single Data Fusion instance."]
        pub async fn update_instance(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateInstanceRequest>,
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
                "/google.cloud.datafusion.v1beta1.DataFusion/UpdateInstance",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Restart a single Data Fusion instance."]
        #[doc = " At the end of an operation instance is fully restarted."]
        pub async fn restart_instance(
            &mut self,
            request: impl tonic::IntoRequest<super::RestartInstanceRequest>,
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
                "/google.cloud.datafusion.v1beta1.DataFusion/RestartInstance",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Upgrade a single Data Fusion instance."]
        #[doc = " At the end of an operation instance is fully upgraded."]
        pub async fn upgrade_instance(
            &mut self,
            request: impl tonic::IntoRequest<super::UpgradeInstanceRequest>,
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
                "/google.cloud.datafusion.v1beta1.DataFusion/UpgradeInstance",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Remove IAM policy that is currently set on the given resource."]
        pub async fn remove_iam_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::RemoveIamPolicyRequest>,
        ) -> Result<tonic::Response<super::RemoveIamPolicyResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.datafusion.v1beta1.DataFusion/RemoveIamPolicy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " List namespaces in a given instance"]
        pub async fn list_namespaces(
            &mut self,
            request: impl tonic::IntoRequest<super::ListNamespacesRequest>,
        ) -> Result<tonic::Response<super::ListNamespacesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.datafusion.v1beta1.DataFusion/ListNamespaces",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Add DNS peering on the given resource."]
        pub async fn add_dns_peering(
            &mut self,
            request: impl tonic::IntoRequest<super::AddDnsPeeringRequest>,
        ) -> Result<tonic::Response<super::AddDnsPeeringResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.datafusion.v1beta1.DataFusion/AddDnsPeering",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Remove DNS peering on the given resource."]
        pub async fn remove_dns_peering(
            &mut self,
            request: impl tonic::IntoRequest<super::RemoveDnsPeeringRequest>,
        ) -> Result<tonic::Response<super::RemoveDnsPeeringResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.datafusion.v1beta1.DataFusion/RemoveDnsPeering",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " List DNS peering for a given resource."]
        pub async fn list_dns_peerings(
            &mut self,
            request: impl tonic::IntoRequest<super::ListDnsPeeringsRequest>,
        ) -> Result<tonic::Response<super::ListDnsPeeringsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.datafusion.v1beta1.DataFusion/ListDnsPeerings",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
