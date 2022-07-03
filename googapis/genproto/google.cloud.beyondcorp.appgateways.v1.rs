/// Request message for BeyondCorp.ListAppGateways.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAppGatewaysRequest {
    /// Required. The resource name of the AppGateway location using the form:
    /// `projects/{project_id}/locations/{location_id}`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. The maximum number of items to return.
    /// If not specified, a default value of 50 will be used by the service.
    /// Regardless of the page_size value, the response may include a partial list
    /// and a caller should only rely on response's
    /// \[next_page_token][BeyondCorp.ListAppGatewaysResponse.next_page_token\] to
    /// determine if there are more instances left to be queried.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. The next_page_token value returned from a previous
    /// ListAppGatewaysRequest, if any.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. A filter specifying constraints of a list operation.
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Optional. Specifies the ordering of results. See
    /// [Sorting
    /// order](<https://cloud.google.com/apis/design/design_patterns#sorting_order>)
    /// for more information.
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
/// Response message for BeyondCorp.ListAppGateways.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAppGatewaysResponse {
    /// A list of BeyondCorp AppGateways in the project.
    #[prost(message, repeated, tag = "1")]
    pub app_gateways: ::prost::alloc::vec::Vec<AppGateway>,
    /// A token to retrieve the next page of results, or empty if there are no more
    /// results in the list.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// A list of locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Request message for BeyondCorp.GetAppGateway.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAppGatewayRequest {
    /// Required. BeyondCorp AppGateway name using the form:
    /// `projects/{project_id}/locations/{location_id}/appGateways/{app_gateway_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for BeyondCorp.CreateAppGateway.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateAppGatewayRequest {
    /// Required. The resource project name of the AppGateway location using the
    /// form: `projects/{project_id}/locations/{location_id}`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. User-settable AppGateway resource ID.
    ///  * Must start with a letter.
    ///  * Must contain between 4-63 characters from `/\[a-z][0-9\]-/`.
    ///  * Must end with a number or a letter.
    #[prost(string, tag = "2")]
    pub app_gateway_id: ::prost::alloc::string::String,
    /// Required. A BeyondCorp AppGateway resource.
    #[prost(message, optional, tag = "3")]
    pub app_gateway: ::core::option::Option<AppGateway>,
    /// Optional. An optional request ID to identify requests. Specify a unique
    /// request ID so that if you must retry your request, the server will know to
    /// ignore the request if it has already been completed. The server will
    /// guarantee that for at least 60 minutes since the first request.
    ///
    /// For example, consider a situation where you make an initial request and t
    /// he request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "4")]
    pub request_id: ::prost::alloc::string::String,
    /// Optional. If set, validates request by executing a dry-run which would not
    /// alter the resource in any way.
    #[prost(bool, tag = "5")]
    pub validate_only: bool,
}
/// Request message for BeyondCorp.DeleteAppGateway.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteAppGatewayRequest {
    /// Required. BeyondCorp AppGateway name using the form:
    /// `projects/{project_id}/locations/{location_id}/appGateways/{app_gateway_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. An optional request ID to identify requests. Specify a unique
    /// request ID so that if you must retry your request, the server will know to
    /// ignore the request if it has already been completed. The server will
    /// guarantee that for at least 60 minutes after the first request.
    ///
    /// For example, consider a situation where you make an initial request and t
    /// he request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "2")]
    pub request_id: ::prost::alloc::string::String,
    /// Optional. If set, validates request by executing a dry-run which would not
    /// alter the resource in any way.
    #[prost(bool, tag = "3")]
    pub validate_only: bool,
}
/// A BeyondCorp AppGateway resource represents a BeyondCorp protected AppGateway
/// to a remote application. It creates all the necessary GCP components needed
/// for creating a BeyondCorp protected AppGateway. Multiple connectors can be
/// authorised for a single AppGateway.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AppGateway {
    /// Required. Unique resource name of the AppGateway.
    /// The name is ignored when creating an AppGateway.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. Timestamp when the resource was created.
    #[prost(message, optional, tag = "2")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Timestamp when the resource was last modified.
    #[prost(message, optional, tag = "3")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Optional. Resource labels to represent user provided metadata.
    #[prost(map = "string, string", tag = "4")]
    pub labels:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Optional. An arbitrary user-provided name for the AppGateway. Cannot exceed
    /// 64 characters.
    #[prost(string, tag = "5")]
    pub display_name: ::prost::alloc::string::String,
    /// Output only. A unique identifier for the instance generated by the
    /// system.
    #[prost(string, tag = "6")]
    pub uid: ::prost::alloc::string::String,
    /// Required. The type of network connectivity used by the AppGateway.
    #[prost(enumeration = "app_gateway::Type", tag = "7")]
    pub r#type: i32,
    /// Output only. The current state of the AppGateway.
    #[prost(enumeration = "app_gateway::State", tag = "8")]
    pub state: i32,
    /// Output only. Server-defined URI for this resource.
    #[prost(string, tag = "9")]
    pub uri: ::prost::alloc::string::String,
    /// Output only. A list of connections allocated for the Gateway
    #[prost(message, repeated, tag = "10")]
    pub allocated_connections: ::prost::alloc::vec::Vec<app_gateway::AllocatedConnection>,
    /// Required. The type of hosting used by the AppGateway.
    #[prost(enumeration = "app_gateway::HostType", tag = "11")]
    pub host_type: i32,
}
/// Nested message and enum types in `AppGateway`.
pub mod app_gateway {
    /// Allocated connection of the AppGateway.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct AllocatedConnection {
        /// Required. The PSC uri of an allocated connection
        #[prost(string, tag = "1")]
        pub psc_uri: ::prost::alloc::string::String,
        /// Required. The ingress port of an allocated connection
        #[prost(int32, tag = "2")]
        pub ingress_port: i32,
    }
    /// Enum containing list of all possible network connectivity options
    /// supported by BeyondCorp AppGateway.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Type {
        /// Default value. This value is unused.
        Unspecified = 0,
        /// TCP Proxy based BeyondCorp Connection. API will default to this if unset.
        TcpProxy = 1,
    }
    /// Represents the different states of an AppGateway.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// Default value. This value is unused.
        Unspecified = 0,
        /// AppGateway is being created.
        Creating = 1,
        /// AppGateway has been created.
        Created = 2,
        /// AppGateway's configuration is being updated.
        Updating = 3,
        /// AppGateway is being deleted.
        Deleting = 4,
        /// AppGateway is down and may be restored in the future.
        /// This happens when CCFE sends ProjectState = OFF.
        Down = 5,
    }
    /// Enum containing list of all possible host types supported by BeyondCorp
    /// Connection.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum HostType {
        /// Default value. This value is unused.
        Unspecified = 0,
        /// AppGateway hosted in a GCP regional managed instance group.
        GcpRegionalMig = 1,
    }
}
/// Represents the metadata of the long-running operation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AppGatewayOperationMetadata {
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
    pub status_message: ::prost::alloc::string::String,
    /// Output only. Identifies whether the user has requested cancellation
    /// of the operation. Operations that have successfully been cancelled
    /// have \[Operation.error][\] value with a
    /// \[google.rpc.Status.code][google.rpc.Status.code\] of 1, corresponding to
    /// `Code.CANCELLED`.
    #[prost(bool, tag = "6")]
    pub requested_cancellation: bool,
    /// Output only. API version used to start the operation.
    #[prost(string, tag = "7")]
    pub api_version: ::prost::alloc::string::String,
}
#[doc = r" Generated client implementations."]
pub mod app_gateways_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " ## API Overview"]
    #[doc = ""]
    #[doc = " The `beyondcorp.googleapis.com` service implements the Google Cloud"]
    #[doc = " BeyondCorp API."]
    #[doc = ""]
    #[doc = " ## Data Model"]
    #[doc = ""]
    #[doc = " The AppGatewaysService exposes the following resources:"]
    #[doc = ""]
    #[doc = " * AppGateways, named as follows:"]
    #[doc = "   `projects/{project_id}/locations/{location_id}/appGateways/{app_gateway_id}`."]
    #[doc = ""]
    #[doc = " The AppGatewaysService service provides methods to manage"]
    #[doc = " (create/read/update/delete) BeyondCorp AppGateways."]
    #[derive(Debug, Clone)]
    pub struct AppGatewaysServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> AppGatewaysServiceClient<T>
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
        ) -> AppGatewaysServiceClient<InterceptedService<T, F>>
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
            AppGatewaysServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Lists AppGateways in a given project and location."]
        pub async fn list_app_gateways(
            &mut self,
            request: impl tonic::IntoRequest<super::ListAppGatewaysRequest>,
        ) -> Result<tonic::Response<super::ListAppGatewaysResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.beyondcorp.appgateways.v1.AppGatewaysService/ListAppGateways",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets details of a single AppGateway."]
        pub async fn get_app_gateway(
            &mut self,
            request: impl tonic::IntoRequest<super::GetAppGatewayRequest>,
        ) -> Result<tonic::Response<super::AppGateway>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.beyondcorp.appgateways.v1.AppGatewaysService/GetAppGateway",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a new AppGateway in a given project and location."]
        pub async fn create_app_gateway(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateAppGatewayRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.beyondcorp.appgateways.v1.AppGatewaysService/CreateAppGateway",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes a single AppGateway."]
        pub async fn delete_app_gateway(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteAppGatewayRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.beyondcorp.appgateways.v1.AppGatewaysService/DeleteAppGateway",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
