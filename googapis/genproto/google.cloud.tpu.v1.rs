/// Sets the scheduling options for this node.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SchedulingConfig {
    /// Defines whether the node is preemptible.
    #[prost(bool, tag = "1")]
    pub preemptible: bool,
    /// Whether the node is created under a reservation.
    #[prost(bool, tag = "2")]
    pub reserved: bool,
}
/// A network endpoint over which a TPU worker can be reached.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NetworkEndpoint {
    /// The IP address of this network endpoint.
    #[prost(string, tag = "1")]
    pub ip_address: ::prost::alloc::string::String,
    /// The port of this network endpoint.
    #[prost(int32, tag = "2")]
    pub port: i32,
}
/// A TPU instance.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Node {
    /// Output only. Immutable. The name of the TPU
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The user-supplied description of the TPU. Maximum of 512 characters.
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    /// Required. The type of hardware accelerators associated with this node.
    #[prost(string, tag = "5")]
    pub accelerator_type: ::prost::alloc::string::String,
    /// Output only. DEPRECATED! Use network_endpoints instead.
    /// The network address for the TPU Node as visible to Compute Engine
    /// instances.
    #[deprecated]
    #[prost(string, tag = "8")]
    pub ip_address: ::prost::alloc::string::String,
    /// Output only. DEPRECATED! Use network_endpoints instead.
    /// The network port for the TPU Node as visible to Compute Engine instances.
    #[deprecated]
    #[prost(string, tag = "14")]
    pub port: ::prost::alloc::string::String,
    /// Output only. The current state for the TPU Node.
    #[prost(enumeration = "node::State", tag = "9")]
    pub state: i32,
    /// Output only. If this field is populated, it contains a description of why the TPU Node
    /// is unhealthy.
    #[prost(string, tag = "10")]
    pub health_description: ::prost::alloc::string::String,
    /// Required. The version of Tensorflow running in the Node.
    #[prost(string, tag = "11")]
    pub tensorflow_version: ::prost::alloc::string::String,
    /// The name of a network they wish to peer the TPU node to. It must be a
    /// preexisting Compute Engine network inside of the project on which this API
    /// has been activated. If none is provided, "default" will be used.
    #[prost(string, tag = "12")]
    pub network: ::prost::alloc::string::String,
    /// The CIDR block that the TPU node will use when selecting an IP address.
    /// This CIDR block must be a /29 block; the Compute Engine networks API
    /// forbids a smaller block, and using a larger block would be wasteful (a
    /// node can only consume one IP address). Errors will occur if the CIDR block
    /// has already been used for a currently existing TPU node, the CIDR block
    /// conflicts with any subnetworks in the user's provided network, or the
    /// provided network is peered with another network that is using that CIDR
    /// block.
    #[prost(string, tag = "13")]
    pub cidr_block: ::prost::alloc::string::String,
    /// Output only. The service account used to run the tensor flow services within the node.
    /// To share resources, including Google Cloud Storage data, with the
    /// Tensorflow job running in the Node, this account must have permissions to
    /// that data.
    #[prost(string, tag = "15")]
    pub service_account: ::prost::alloc::string::String,
    /// Output only. The time when the node was created.
    #[prost(message, optional, tag = "16")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The scheduling options for this node.
    #[prost(message, optional, tag = "17")]
    pub scheduling_config: ::core::option::Option<SchedulingConfig>,
    /// Output only. The network endpoints where TPU workers can be accessed and
    /// sent work. It is recommended that Tensorflow clients of the node reach out
    /// to the 0th entry in this map first.
    #[prost(message, repeated, tag = "21")]
    pub network_endpoints: ::prost::alloc::vec::Vec<NetworkEndpoint>,
    /// The health status of the TPU node.
    #[prost(enumeration = "node::Health", tag = "22")]
    pub health: i32,
    /// Resource labels to represent user-provided metadata.
    #[prost(map = "string, string", tag = "24")]
    pub labels:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Whether the VPC peering for the node is set up through Service Networking
    /// API. The VPC Peering should be set up before provisioning the node.
    /// If this field is set, cidr_block field should not be specified. If the
    /// network, that you want to peer the TPU Node to, is Shared VPC networks,
    /// the node must be created with this this field enabled.
    #[prost(bool, tag = "27")]
    pub use_service_networking: bool,
    /// Output only. The API version that created this Node.
    #[prost(enumeration = "node::ApiVersion", tag = "38")]
    pub api_version: i32,
    /// Output only. The Symptoms that have occurred to the TPU Node.
    #[prost(message, repeated, tag = "39")]
    pub symptoms: ::prost::alloc::vec::Vec<Symptom>,
}
/// Nested message and enum types in `Node`.
pub mod node {
    /// Represents the different states of a TPU node during its lifecycle.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// TPU node state is not known/set.
        Unspecified = 0,
        /// TPU node is being created.
        Creating = 1,
        /// TPU node has been created and is fully usable.
        Ready = 2,
        /// TPU node is restarting.
        Restarting = 3,
        /// TPU node is undergoing reimaging.
        Reimaging = 4,
        /// TPU node is being deleted.
        Deleting = 5,
        /// TPU node is being repaired and may be unusable. Details can be
        /// found in the `help_description` field.
        Repairing = 6,
        /// TPU node is stopped.
        Stopped = 8,
        /// TPU node is currently stopping.
        Stopping = 9,
        /// TPU node is currently starting.
        Starting = 10,
        /// TPU node has been preempted. Only applies to Preemptible TPU Nodes.
        Preempted = 11,
        /// TPU node has been terminated due to maintenance or has reached the end of
        /// its life cycle (for preemptible nodes).
        Terminated = 12,
        /// TPU node is currently hiding.
        Hiding = 13,
        /// TPU node has been hidden.
        Hidden = 14,
        /// TPU node is currently unhiding.
        Unhiding = 15,
    }
    /// Health defines the status of a TPU node as reported by
    /// Health Monitor.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Health {
        /// Health status is unknown: not initialized or failed to retrieve.
        Unspecified = 0,
        /// The resource is healthy.
        Healthy = 1,
        /// The resource is unhealthy.
        DeprecatedUnhealthy = 2,
        /// The resource is unresponsive.
        Timeout = 3,
        /// The in-guest ML stack is unhealthy.
        UnhealthyTensorflow = 4,
        /// The node is under maintenance/priority boost caused rescheduling and
        /// will resume running once rescheduled.
        UnhealthyMaintenance = 5,
    }
    /// TPU API Version.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ApiVersion {
        /// API version is unknown.
        Unspecified = 0,
        /// TPU API V1Alpha1 version.
        V1Alpha1 = 1,
        /// TPU API V1 version.
        V1 = 2,
        /// TPU API V2Alpha1 version.
        V2Alpha1 = 3,
    }
}
/// Request for \[ListNodes][google.cloud.tpu.v1.Tpu.ListNodes\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListNodesRequest {
    /// Required. The parent resource name.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of items to return.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The next_page_token value returned from a previous List request, if any.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response for \[ListNodes][google.cloud.tpu.v1.Tpu.ListNodes\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListNodesResponse {
    /// The listed nodes.
    #[prost(message, repeated, tag = "1")]
    pub nodes: ::prost::alloc::vec::Vec<Node>,
    /// The next page token or empty if none.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Request for \[GetNode][google.cloud.tpu.v1.Tpu.GetNode\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetNodeRequest {
    /// Required. The resource name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request for \[CreateNode][google.cloud.tpu.v1.Tpu.CreateNode\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateNodeRequest {
    /// Required. The parent resource name.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The unqualified resource name.
    #[prost(string, tag = "2")]
    pub node_id: ::prost::alloc::string::String,
    /// Required. The node.
    #[prost(message, optional, tag = "3")]
    pub node: ::core::option::Option<Node>,
}
/// Request for \[DeleteNode][google.cloud.tpu.v1.Tpu.DeleteNode\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteNodeRequest {
    /// Required. The resource name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request for \[ReimageNode][google.cloud.tpu.v1.Tpu.ReimageNode\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReimageNodeRequest {
    /// The resource name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The version for reimage to create.
    #[prost(string, tag = "2")]
    pub tensorflow_version: ::prost::alloc::string::String,
}
/// Request for \[StopNode][google.cloud.tpu.v1.Tpu.StopNode\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StopNodeRequest {
    /// The resource name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request for \[StartNode][google.cloud.tpu.v1.Tpu.StartNode\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StartNodeRequest {
    /// The resource name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// A tensorflow version that a Node can be configured with.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TensorFlowVersion {
    /// The resource name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// the tensorflow version.
    #[prost(string, tag = "2")]
    pub version: ::prost::alloc::string::String,
}
/// Request for \[GetTensorFlowVersion][google.cloud.tpu.v1.Tpu.GetTensorFlowVersion\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTensorFlowVersionRequest {
    /// Required. The resource name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request for \[ListTensorFlowVersions][google.cloud.tpu.v1.Tpu.ListTensorFlowVersions\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTensorFlowVersionsRequest {
    /// Required. The parent resource name.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of items to return.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The next_page_token value returned from a previous List request, if any.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// List filter.
    #[prost(string, tag = "5")]
    pub filter: ::prost::alloc::string::String,
    /// Sort results.
    #[prost(string, tag = "6")]
    pub order_by: ::prost::alloc::string::String,
}
/// Response for \[ListTensorFlowVersions][google.cloud.tpu.v1.Tpu.ListTensorFlowVersions\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTensorFlowVersionsResponse {
    /// The listed nodes.
    #[prost(message, repeated, tag = "1")]
    pub tensorflow_versions: ::prost::alloc::vec::Vec<TensorFlowVersion>,
    /// The next page token or empty if none.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// A accelerator type that a Node can be configured with.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AcceleratorType {
    /// The resource name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// the accelerator type.
    #[prost(string, tag = "2")]
    pub r#type: ::prost::alloc::string::String,
}
/// Request for \[GetAcceleratorType][google.cloud.tpu.v1.Tpu.GetAcceleratorType\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAcceleratorTypeRequest {
    /// Required. The resource name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request for \[ListAcceleratorTypes][google.cloud.tpu.v1.Tpu.ListAcceleratorTypes\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAcceleratorTypesRequest {
    /// Required. The parent resource name.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of items to return.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The next_page_token value returned from a previous List request, if any.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// List filter.
    #[prost(string, tag = "5")]
    pub filter: ::prost::alloc::string::String,
    /// Sort results.
    #[prost(string, tag = "6")]
    pub order_by: ::prost::alloc::string::String,
}
/// Response for \[ListAcceleratorTypes][google.cloud.tpu.v1.Tpu.ListAcceleratorTypes\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAcceleratorTypesResponse {
    /// The listed nodes.
    #[prost(message, repeated, tag = "1")]
    pub accelerator_types: ::prost::alloc::vec::Vec<AcceleratorType>,
    /// The next page token or empty if none.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
// Note: the following OperationMetadata message was added manually.
// This is caused by a conflict with some other message and will
// be resolved separately. Please make sure to add this message back
// if it's removed during public proto regeneration.

/// Metadata describing an \[Operation][google.longrunning.Operation\]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationMetadata {
    /// The time the operation was created.
    #[prost(message, optional, tag = "1")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The time the operation finished running.
    #[prost(message, optional, tag = "2")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Target of the operation - for example
    /// projects/project-1/connectivityTests/test-1
    #[prost(string, tag = "3")]
    pub target: ::prost::alloc::string::String,
    /// Name of the verb executed by the operation.
    #[prost(string, tag = "4")]
    pub verb: ::prost::alloc::string::String,
    /// Human-readable status of the operation, if any.
    #[prost(string, tag = "5")]
    pub status_detail: ::prost::alloc::string::String,
    /// Specifies if cancellation was requested for the operation.
    #[prost(bool, tag = "6")]
    pub cancel_requested: bool,
    /// API version.
    #[prost(string, tag = "7")]
    pub api_version: ::prost::alloc::string::String,
}
/// A Symptom instance.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Symptom {
    /// Timestamp when the Symptom is created.
    #[prost(message, optional, tag = "1")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Type of the Symptom.
    #[prost(enumeration = "symptom::SymptomType", tag = "2")]
    pub symptom_type: i32,
    /// Detailed information of the current Symptom.
    #[prost(string, tag = "3")]
    pub details: ::prost::alloc::string::String,
    /// A string used to uniquely distinguish a worker within a TPU node.
    #[prost(string, tag = "4")]
    pub worker_id: ::prost::alloc::string::String,
}
/// Nested message and enum types in `Symptom`.
pub mod symptom {
    /// SymptomType represents the different types of Symptoms that a TPU can be
    /// at.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum SymptomType {
        /// Unspecified symptom.
        Unspecified = 0,
        /// TPU VM memory is low.
        LowMemory = 1,
        /// TPU runtime is out of memory.
        OutOfMemory = 2,
        /// TPU runtime execution has timed out.
        ExecuteTimedOut = 3,
        /// TPU runtime fails to construct a mesh that recognizes each TPU device's
        /// neighbors.
        MeshBuildFail = 4,
        /// TPU HBM is out of memory.
        HbmOutOfMemory = 5,
        /// Abusive behaviors have been identified on the current project.
        ProjectAbuse = 6,
    }
}
#[doc = r" Generated client implementations."]
pub mod tpu_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Manages TPU nodes and other resources"]
    #[doc = ""]
    #[doc = " TPU API v1"]
    #[derive(Debug, Clone)]
    pub struct TpuClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> TpuClient<T>
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
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> TpuClient<InterceptedService<T, F>>
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
            TpuClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Lists nodes."]
        pub async fn list_nodes(
            &mut self,
            request: impl tonic::IntoRequest<super::ListNodesRequest>,
        ) -> Result<tonic::Response<super::ListNodesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/google.cloud.tpu.v1.Tpu/ListNodes");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets the details of a node."]
        pub async fn get_node(
            &mut self,
            request: impl tonic::IntoRequest<super::GetNodeRequest>,
        ) -> Result<tonic::Response<super::Node>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/google.cloud.tpu.v1.Tpu/GetNode");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a node."]
        pub async fn create_node(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateNodeRequest>,
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
            let path = http::uri::PathAndQuery::from_static("/google.cloud.tpu.v1.Tpu/CreateNode");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes a node."]
        pub async fn delete_node(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteNodeRequest>,
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
            let path = http::uri::PathAndQuery::from_static("/google.cloud.tpu.v1.Tpu/DeleteNode");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Reimages a node's OS."]
        pub async fn reimage_node(
            &mut self,
            request: impl tonic::IntoRequest<super::ReimageNodeRequest>,
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
            let path = http::uri::PathAndQuery::from_static("/google.cloud.tpu.v1.Tpu/ReimageNode");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Stops a node."]
        pub async fn stop_node(
            &mut self,
            request: impl tonic::IntoRequest<super::StopNodeRequest>,
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
            let path = http::uri::PathAndQuery::from_static("/google.cloud.tpu.v1.Tpu/StopNode");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Starts a node."]
        pub async fn start_node(
            &mut self,
            request: impl tonic::IntoRequest<super::StartNodeRequest>,
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
            let path = http::uri::PathAndQuery::from_static("/google.cloud.tpu.v1.Tpu/StartNode");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " List TensorFlow versions supported by this API."]
        pub async fn list_tensor_flow_versions(
            &mut self,
            request: impl tonic::IntoRequest<super::ListTensorFlowVersionsRequest>,
        ) -> Result<tonic::Response<super::ListTensorFlowVersionsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.tpu.v1.Tpu/ListTensorFlowVersions",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets TensorFlow Version."]
        pub async fn get_tensor_flow_version(
            &mut self,
            request: impl tonic::IntoRequest<super::GetTensorFlowVersionRequest>,
        ) -> Result<tonic::Response<super::TensorFlowVersion>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.tpu.v1.Tpu/GetTensorFlowVersion",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists accelerator types supported by this API."]
        pub async fn list_accelerator_types(
            &mut self,
            request: impl tonic::IntoRequest<super::ListAcceleratorTypesRequest>,
        ) -> Result<tonic::Response<super::ListAcceleratorTypesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.tpu.v1.Tpu/ListAcceleratorTypes",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets AcceleratorType."]
        pub async fn get_accelerator_type(
            &mut self,
            request: impl tonic::IntoRequest<super::GetAcceleratorTypeRequest>,
        ) -> Result<tonic::Response<super::AcceleratorType>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/google.cloud.tpu.v1.Tpu/GetAcceleratorType");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
