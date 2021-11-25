/// Definition of a Serverless VPC Access connector.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Connector {
    /// The resource name in the format `projects/*/locations/*/connectors/*`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Name of a VPC network.
    #[prost(string, tag = "2")]
    pub network: ::prost::alloc::string::String,
    /// The range of internal addresses that follows RFC 4632 notation.
    /// Example: `10.132.0.0/28`.
    #[prost(string, tag = "3")]
    pub ip_cidr_range: ::prost::alloc::string::String,
    /// Output only. State of the VPC access connector.
    #[prost(enumeration = "connector::State", tag = "4")]
    pub state: i32,
    /// Minimum throughput of the connector in Mbps. Default and min is 200.
    #[prost(int32, tag = "5")]
    pub min_throughput: i32,
    /// Maximum throughput of the connector in Mbps. Default is 200, max is 1000.
    #[prost(int32, tag = "6")]
    pub max_throughput: i32,
    /// Output only. List of projects using the connector.
    #[prost(string, repeated, tag = "7")]
    pub connected_projects: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The subnet in which to house the VPC Access Connector.
    #[prost(message, optional, tag = "8")]
    pub subnet: ::core::option::Option<connector::Subnet>,
}
/// Nested message and enum types in `Connector`.
pub mod connector {
    /// The subnet in which to house the connector
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Subnet {
        /// Subnet name (relative, not fully qualified).
        /// E.g. if the full subnet selfLink is
        /// <https://compute.googleapis.com/compute/v1/projects/{project}/regions/{region}/subnetworks/{subnetName}>
        /// the correct input for this field would be {subnetName}
        #[prost(string, tag = "1")]
        pub name: ::prost::alloc::string::String,
        /// Project in which the subnet exists.
        /// If not set, this project is assumed to be the project for which
        /// the connector create request was issued.
        #[prost(string, tag = "2")]
        pub project_id: ::prost::alloc::string::String,
    }
    /// State of a connector.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// Invalid state.
        Unspecified = 0,
        /// Connector is deployed and ready to receive traffic.
        Ready = 1,
        /// An Insert operation is in progress. Transient condition.
        Creating = 2,
        /// A Delete operation is in progress. Transient condition.
        Deleting = 3,
        /// Connector is in a bad state, manual deletion recommended.
        Error = 4,
        /// The connector is being updated.
        Updating = 5,
    }
}
/// Request for creating a Serverless VPC Access connector.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateConnectorRequest {
    /// Required. The project and location in which the configuration should be created,
    /// specified in the format `projects/*/locations/*`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The ID to use for this connector.
    #[prost(string, tag = "2")]
    pub connector_id: ::prost::alloc::string::String,
    /// Required. Resource to create.
    #[prost(message, optional, tag = "3")]
    pub connector: ::core::option::Option<Connector>,
}
/// Request for getting a Serverless VPC Access connector.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetConnectorRequest {
    /// Required. Name of a Serverless VPC Access connector to get.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request for listing Serverless VPC Access connectors in a location.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListConnectorsRequest {
    /// Required. The project and location from which the routes should be listed.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Maximum number of functions to return per call.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Continuation token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response for listing Serverless VPC Access connectors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListConnectorsResponse {
    /// List of Serverless VPC Access connectors.
    #[prost(message, repeated, tag = "1")]
    pub connectors: ::prost::alloc::vec::Vec<Connector>,
    /// Continuation token.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request for deleting a Serverless VPC Access connector.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteConnectorRequest {
    /// Required. Name of a Serverless VPC Access connector to delete.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Metadata for google.longrunning.Operation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationMetadata {
    /// Output only. Method that initiated the operation e.g.
    /// google.cloud.vpcaccess.v1.Connectors.CreateConnector.
    #[prost(string, tag = "1")]
    pub method: ::prost::alloc::string::String,
    /// Output only. Time when the operation was created.
    #[prost(message, optional, tag = "2")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Time when the operation completed.
    #[prost(message, optional, tag = "3")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Name of the resource that this operation is acting on e.g.
    /// projects/my-project/locations/us-central1/connectors/v1.
    #[prost(string, tag = "5")]
    pub target: ::prost::alloc::string::String,
}
#[doc = r" Generated client implementations."]
pub mod vpc_access_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Serverless VPC Access API allows users to create and manage connectors for"]
    #[doc = " App Engine, Cloud Functions and Cloud Run to have internal connections to"]
    #[doc = " Virtual Private Cloud networks."]
    #[derive(Debug, Clone)]
    pub struct VpcAccessServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> VpcAccessServiceClient<T>
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
        ) -> VpcAccessServiceClient<InterceptedService<T, F>>
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
            VpcAccessServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Creates a Serverless VPC Access connector, returns an operation."]
        pub async fn create_connector(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateConnectorRequest>,
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
                "/google.cloud.vpcaccess.v1.VpcAccessService/CreateConnector",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets a Serverless VPC Access connector. Returns NOT_FOUND if the resource"]
        #[doc = " does not exist."]
        pub async fn get_connector(
            &mut self,
            request: impl tonic::IntoRequest<super::GetConnectorRequest>,
        ) -> Result<tonic::Response<super::Connector>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.vpcaccess.v1.VpcAccessService/GetConnector",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists Serverless VPC Access connectors."]
        pub async fn list_connectors(
            &mut self,
            request: impl tonic::IntoRequest<super::ListConnectorsRequest>,
        ) -> Result<tonic::Response<super::ListConnectorsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.vpcaccess.v1.VpcAccessService/ListConnectors",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes a Serverless VPC Access connector. Returns NOT_FOUND if the"]
        #[doc = " resource does not exist."]
        pub async fn delete_connector(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteConnectorRequest>,
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
                "/google.cloud.vpcaccess.v1.VpcAccessService/DeleteConnector",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
