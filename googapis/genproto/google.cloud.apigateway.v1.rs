/// An API that can be served by one or more Gateways.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Api {
    /// Output only. Resource name of the API.
    /// Format: projects/{project}/locations/global/apis/{api}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. Created time.
    #[prost(message, optional, tag = "2")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Updated time.
    #[prost(message, optional, tag = "3")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Optional. Resource labels to represent user-provided metadata.
    /// Refer to cloud documentation on labels for more details.
    /// <https://cloud.google.com/compute/docs/labeling-resources>
    #[prost(map = "string, string", tag = "4")]
    pub labels:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Optional. Display name.
    #[prost(string, tag = "5")]
    pub display_name: ::prost::alloc::string::String,
    /// Optional. Immutable. The name of a Google Managed Service (
    /// <https://cloud.google.com/service-infrastructure/docs/glossary#managed>). If
    /// not specified, a new Service will automatically be created in the same
    /// project as this API.
    #[prost(string, tag = "7")]
    pub managed_service: ::prost::alloc::string::String,
    /// Output only. State of the API.
    #[prost(enumeration = "api::State", tag = "12")]
    pub state: i32,
}
/// Nested message and enum types in `Api`.
pub mod api {
    /// All the possible API states.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// API does not have a state yet.
        Unspecified = 0,
        /// API is being created.
        Creating = 1,
        /// API is active.
        Active = 2,
        /// API creation failed.
        Failed = 3,
        /// API is being deleted.
        Deleting = 4,
        /// API is being updated.
        Updating = 5,
    }
}
/// An API Configuration is a combination of settings for both the Managed
/// Service and Gateways serving this API Config.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApiConfig {
    /// Output only. Resource name of the API Config.
    /// Format: projects/{project}/locations/global/apis/{api}/configs/{api_config}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. Created time.
    #[prost(message, optional, tag = "2")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Updated time.
    #[prost(message, optional, tag = "3")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Optional. Resource labels to represent user-provided metadata.
    /// Refer to cloud documentation on labels for more details.
    /// <https://cloud.google.com/compute/docs/labeling-resources>
    #[prost(map = "string, string", tag = "4")]
    pub labels:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Optional. Display name.
    #[prost(string, tag = "5")]
    pub display_name: ::prost::alloc::string::String,
    /// Immutable. The Google Cloud IAM Service Account that Gateways serving this config
    /// should use to authenticate to other services. This may either be the
    /// Service Account's email
    /// (`{ACCOUNT_ID}@{PROJECT}.iam.gserviceaccount.com`) or its full resource
    /// name (`projects/{PROJECT}/accounts/{UNIQUE_ID}`). This is most often used
    /// when the service is a GCP resource such as a Cloud Run Service or an
    /// IAP-secured service.
    #[prost(string, tag = "14")]
    pub gateway_service_account: ::prost::alloc::string::String,
    /// Output only. The ID of the associated Service Config (
    /// <https://cloud.google.com/service-infrastructure/docs/glossary#config>).
    #[prost(string, tag = "12")]
    pub service_config_id: ::prost::alloc::string::String,
    /// Output only. State of the API Config.
    #[prost(enumeration = "api_config::State", tag = "8")]
    pub state: i32,
    /// Optional. OpenAPI specification documents. If specified, grpc_services and
    /// managed_service_configs must not be included.
    #[prost(message, repeated, tag = "9")]
    pub openapi_documents: ::prost::alloc::vec::Vec<api_config::OpenApiDocument>,
    /// Optional. gRPC service definition files. If specified, openapi_documents must
    /// not be included.
    #[prost(message, repeated, tag = "10")]
    pub grpc_services: ::prost::alloc::vec::Vec<api_config::GrpcServiceDefinition>,
    /// Optional. Service Configuration files. At least one must be included when using gRPC
    /// service definitions. See
    /// <https://cloud.google.com/endpoints/docs/grpc/grpc-service-config#service_configuration_overview>
    /// for the expected file contents.
    ///
    /// If multiple files are specified, the files are merged with the following
    /// rules:
    /// * All singular scalar fields are merged using "last one wins" semantics in
    /// the order of the files uploaded.
    /// * Repeated fields are concatenated.
    /// * Singular embedded messages are merged using these rules for nested
    /// fields.
    #[prost(message, repeated, tag = "11")]
    pub managed_service_configs: ::prost::alloc::vec::Vec<api_config::File>,
}
/// Nested message and enum types in `ApiConfig`.
pub mod api_config {
    /// A lightweight description of a file.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct File {
        /// The file path (full or relative path). This is typically the path of the
        /// file when it is uploaded.
        #[prost(string, tag = "1")]
        pub path: ::prost::alloc::string::String,
        /// The bytes that constitute the file.
        #[prost(bytes = "vec", tag = "2")]
        pub contents: ::prost::alloc::vec::Vec<u8>,
    }
    /// An OpenAPI Specification Document describing an API.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct OpenApiDocument {
        /// The OpenAPI Specification document file.
        #[prost(message, optional, tag = "1")]
        pub document: ::core::option::Option<File>,
    }
    /// A gRPC service definition.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GrpcServiceDefinition {
        /// Input only. File descriptor set, generated by protoc.
        ///
        /// To generate, use protoc with imports and source info included.
        /// For an example test.proto file, the following command would put the value
        /// in a new file named out.pb.
        ///
        /// $ protoc --include_imports --include_source_info test.proto -o out.pb
        #[prost(message, optional, tag = "1")]
        pub file_descriptor_set: ::core::option::Option<File>,
        /// Optional. Uncompiled proto files associated with the descriptor set, used for
        /// display purposes (server-side compilation is not supported). These
        /// should match the inputs to 'protoc' command used to generate
        /// file_descriptor_set.
        #[prost(message, repeated, tag = "2")]
        pub source: ::prost::alloc::vec::Vec<File>,
    }
    /// All the possible API Config states.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// API Config does not have a state yet.
        Unspecified = 0,
        /// API Config is being created and deployed to the API Controller.
        Creating = 1,
        /// API Config is ready for use by Gateways.
        Active = 2,
        /// API Config creation failed.
        Failed = 3,
        /// API Config is being deleted.
        Deleting = 4,
        /// API Config is being updated.
        Updating = 5,
        /// API Config settings are being activated in downstream systems.
        /// API Configs in this state cannot be used by Gateways.
        Activating = 6,
    }
}
/// A Gateway is an API-aware HTTP proxy. It performs API-Method and/or
/// API-Consumer specific actions based on an API Config such as authentication,
/// policy enforcement, and backend selection.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Gateway {
    /// Output only. Resource name of the Gateway.
    /// Format: projects/{project}/locations/{location}/gateways/{gateway}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. Created time.
    #[prost(message, optional, tag = "2")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Updated time.
    #[prost(message, optional, tag = "3")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Optional. Resource labels to represent user-provided metadata.
    /// Refer to cloud documentation on labels for more details.
    /// <https://cloud.google.com/compute/docs/labeling-resources>
    #[prost(map = "string, string", tag = "4")]
    pub labels:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Optional. Display name.
    #[prost(string, tag = "5")]
    pub display_name: ::prost::alloc::string::String,
    /// Required. Resource name of the API Config for this Gateway.
    /// Format: projects/{project}/locations/global/apis/{api}/configs/{apiConfig}
    #[prost(string, tag = "6")]
    pub api_config: ::prost::alloc::string::String,
    /// Output only. The current state of the Gateway.
    #[prost(enumeration = "gateway::State", tag = "7")]
    pub state: i32,
    /// Output only. The default API Gateway host name of the form
    /// `{gateway_id}-{hash}.{region_code}.gateway.dev`.
    #[prost(string, tag = "9")]
    pub default_hostname: ::prost::alloc::string::String,
}
/// Nested message and enum types in `Gateway`.
pub mod gateway {
    /// All the possible Gateway states.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// Gateway does not have a state yet.
        Unspecified = 0,
        /// Gateway is being created.
        Creating = 1,
        /// Gateway is running and ready for requests.
        Active = 2,
        /// Gateway creation failed.
        Failed = 3,
        /// Gateway is being deleted.
        Deleting = 4,
        /// Gateway is being updated.
        Updating = 5,
    }
}
/// Request message for ApiGatewayService.ListGateways
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListGatewaysRequest {
    /// Required. Parent resource of the Gateway, of the form:
    /// `projects/*/locations/*`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Page size.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Filter.
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Order by parameters.
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
/// Response message for ApiGatewayService.ListGateways
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListGatewaysResponse {
    /// Gateways.
    #[prost(message, repeated, tag = "1")]
    pub gateways: ::prost::alloc::vec::Vec<Gateway>,
    /// Next page token.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable_locations: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Request message for ApiGatewayService.GetGateway
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetGatewayRequest {
    /// Required. Resource name of the form:
    /// `projects/*/locations/*/gateways/*`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for ApiGatewayService.CreateGateway
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateGatewayRequest {
    /// Required. Parent resource of the Gateway, of the form:
    /// `projects/*/locations/*`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. Identifier to assign to the Gateway. Must be unique within scope of
    /// the parent resource.
    #[prost(string, tag = "2")]
    pub gateway_id: ::prost::alloc::string::String,
    /// Required. Gateway resource.
    #[prost(message, optional, tag = "3")]
    pub gateway: ::core::option::Option<Gateway>,
}
/// Request message for ApiGatewayService.UpdateGateway
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateGatewayRequest {
    /// Field mask is used to specify the fields to be overwritten in the
    /// Gateway resource by the update.
    /// The fields specified in the update_mask are relative to the resource, not
    /// the full request. A field will be overwritten if it is in the mask. If the
    /// user does not provide a mask then all fields will be overwritten.
    #[prost(message, optional, tag = "1")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Required. Gateway resource.
    #[prost(message, optional, tag = "2")]
    pub gateway: ::core::option::Option<Gateway>,
}
/// Request message for ApiGatewayService.DeleteGateway
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteGatewayRequest {
    /// Required. Resource name of the form:
    /// `projects/*/locations/*/gateways/*`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for ApiGatewayService.ListApis
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListApisRequest {
    /// Required. Parent resource of the API, of the form:
    /// `projects/*/locations/global`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Page size.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Filter.
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Order by parameters.
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
/// Response message for ApiGatewayService.ListApis
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListApisResponse {
    /// APIs.
    #[prost(message, repeated, tag = "1")]
    pub apis: ::prost::alloc::vec::Vec<Api>,
    /// Next page token.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable_locations: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Request message for ApiGatewayService.GetApi
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetApiRequest {
    /// Required. Resource name of the form:
    /// `projects/*/locations/global/apis/*`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for ApiGatewayService.CreateApi
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateApiRequest {
    /// Required. Parent resource of the API, of the form:
    /// `projects/*/locations/global`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. Identifier to assign to the API. Must be unique within scope of
    /// the parent resource.
    #[prost(string, tag = "2")]
    pub api_id: ::prost::alloc::string::String,
    /// Required. API resource.
    #[prost(message, optional, tag = "3")]
    pub api: ::core::option::Option<Api>,
}
/// Request message for ApiGatewayService.UpdateApi
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateApiRequest {
    /// Field mask is used to specify the fields to be overwritten in the
    /// Api resource by the update.
    /// The fields specified in the update_mask are relative to the resource, not
    /// the full request. A field will be overwritten if it is in the mask. If the
    /// user does not provide a mask then all fields will be overwritten.
    #[prost(message, optional, tag = "1")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Required. API resource.
    #[prost(message, optional, tag = "2")]
    pub api: ::core::option::Option<Api>,
}
/// Request message for ApiGatewayService.DeleteApi
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteApiRequest {
    /// Required. Resource name of the form:
    /// `projects/*/locations/global/apis/*`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for ApiGatewayService.ListApiConfigs
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListApiConfigsRequest {
    /// Required. Parent resource of the API Config, of the form:
    /// `projects/*/locations/global/apis/*`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Page size.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Filter.
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Order by parameters.
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
/// Response message for ApiGatewayService.ListApiConfigs
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListApiConfigsResponse {
    /// API Configs.
    #[prost(message, repeated, tag = "1")]
    pub api_configs: ::prost::alloc::vec::Vec<ApiConfig>,
    /// Next page token.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable_locations: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Request message for ApiGatewayService.GetApiConfig
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetApiConfigRequest {
    /// Required. Resource name of the form:
    /// `projects/*/locations/global/apis/*/configs/*`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Specifies which fields of the API Config are returned in the response.
    /// Defaults to `BASIC` view.
    #[prost(enumeration = "get_api_config_request::ConfigView", tag = "3")]
    pub view: i32,
}
/// Nested message and enum types in `GetApiConfigRequest`.
pub mod get_api_config_request {
    /// Enum to control which fields should be included in the response.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ConfigView {
        Unspecified = 0,
        /// Do not include configuration source files.
        Basic = 1,
        /// Include configuration source files.
        Full = 2,
    }
}
/// Request message for ApiGatewayService.CreateApiConfig
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateApiConfigRequest {
    /// Required. Parent resource of the API Config, of the form:
    /// `projects/*/locations/global/apis/*`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. Identifier to assign to the API Config. Must be unique within scope of
    /// the parent resource.
    #[prost(string, tag = "2")]
    pub api_config_id: ::prost::alloc::string::String,
    /// Required. API resource.
    #[prost(message, optional, tag = "3")]
    pub api_config: ::core::option::Option<ApiConfig>,
}
/// Request message for ApiGatewayService.UpdateApiConfig
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateApiConfigRequest {
    /// Field mask is used to specify the fields to be overwritten in the
    /// ApiConfig resource by the update.
    /// The fields specified in the update_mask are relative to the resource, not
    /// the full request. A field will be overwritten if it is in the mask. If the
    /// user does not provide a mask then all fields will be overwritten.
    #[prost(message, optional, tag = "1")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Required. API Config resource.
    #[prost(message, optional, tag = "2")]
    pub api_config: ::core::option::Option<ApiConfig>,
}
/// Request message for ApiGatewayService.DeleteApiConfig
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteApiConfigRequest {
    /// Required. Resource name of the form:
    /// `projects/*/locations/global/apis/*/configs/*`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
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
    pub status_message: ::prost::alloc::string::String,
    /// Output only. Identifies whether the user has requested cancellation
    /// of the operation. Operations that have successfully been cancelled
    /// have \[Operation.error][\] value with a \[google.rpc.Status.code][google.rpc.Status.code\] of 1,
    /// corresponding to `Code.CANCELLED`.
    #[prost(bool, tag = "6")]
    pub requested_cancellation: bool,
    /// Output only. API version used to start the operation.
    #[prost(string, tag = "7")]
    pub api_version: ::prost::alloc::string::String,
    /// Output only. Diagnostics generated during processing of configuration source files.
    #[prost(message, repeated, tag = "8")]
    pub diagnostics: ::prost::alloc::vec::Vec<operation_metadata::Diagnostic>,
}
/// Nested message and enum types in `OperationMetadata`.
pub mod operation_metadata {
    /// Diagnostic information from configuration processing.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Diagnostic {
        /// Location of the diagnostic.
        #[prost(string, tag = "1")]
        pub location: ::prost::alloc::string::String,
        /// The diagnostic message.
        #[prost(string, tag = "2")]
        pub message: ::prost::alloc::string::String,
    }
}
#[doc = r" Generated client implementations."]
pub mod api_gateway_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " The API Gateway Service is the interface for managing API Gateways."]
    #[derive(Debug, Clone)]
    pub struct ApiGatewayServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> ApiGatewayServiceClient<T>
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
        ) -> ApiGatewayServiceClient<InterceptedService<T, F>>
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
            ApiGatewayServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Lists Gateways in a given project and location."]
        pub async fn list_gateways(
            &mut self,
            request: impl tonic::IntoRequest<super::ListGatewaysRequest>,
        ) -> Result<tonic::Response<super::ListGatewaysResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.apigateway.v1.ApiGatewayService/ListGateways",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets details of a single Gateway."]
        pub async fn get_gateway(
            &mut self,
            request: impl tonic::IntoRequest<super::GetGatewayRequest>,
        ) -> Result<tonic::Response<super::Gateway>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.apigateway.v1.ApiGatewayService/GetGateway",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a new Gateway in a given project and location."]
        pub async fn create_gateway(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateGatewayRequest>,
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
                "/google.cloud.apigateway.v1.ApiGatewayService/CreateGateway",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates the parameters of a single Gateway."]
        pub async fn update_gateway(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateGatewayRequest>,
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
                "/google.cloud.apigateway.v1.ApiGatewayService/UpdateGateway",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes a single Gateway."]
        pub async fn delete_gateway(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteGatewayRequest>,
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
                "/google.cloud.apigateway.v1.ApiGatewayService/DeleteGateway",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists Apis in a given project and location."]
        pub async fn list_apis(
            &mut self,
            request: impl tonic::IntoRequest<super::ListApisRequest>,
        ) -> Result<tonic::Response<super::ListApisResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.apigateway.v1.ApiGatewayService/ListApis",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets details of a single Api."]
        pub async fn get_api(
            &mut self,
            request: impl tonic::IntoRequest<super::GetApiRequest>,
        ) -> Result<tonic::Response<super::Api>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.apigateway.v1.ApiGatewayService/GetApi",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a new Api in a given project and location."]
        pub async fn create_api(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateApiRequest>,
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
                "/google.cloud.apigateway.v1.ApiGatewayService/CreateApi",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates the parameters of a single Api."]
        pub async fn update_api(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateApiRequest>,
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
                "/google.cloud.apigateway.v1.ApiGatewayService/UpdateApi",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes a single Api."]
        pub async fn delete_api(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteApiRequest>,
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
                "/google.cloud.apigateway.v1.ApiGatewayService/DeleteApi",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists ApiConfigs in a given project and location."]
        pub async fn list_api_configs(
            &mut self,
            request: impl tonic::IntoRequest<super::ListApiConfigsRequest>,
        ) -> Result<tonic::Response<super::ListApiConfigsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.apigateway.v1.ApiGatewayService/ListApiConfigs",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets details of a single ApiConfig."]
        pub async fn get_api_config(
            &mut self,
            request: impl tonic::IntoRequest<super::GetApiConfigRequest>,
        ) -> Result<tonic::Response<super::ApiConfig>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.apigateway.v1.ApiGatewayService/GetApiConfig",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a new ApiConfig in a given project and location."]
        pub async fn create_api_config(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateApiConfigRequest>,
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
                "/google.cloud.apigateway.v1.ApiGatewayService/CreateApiConfig",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates the parameters of a single ApiConfig."]
        pub async fn update_api_config(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateApiConfigRequest>,
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
                "/google.cloud.apigateway.v1.ApiGatewayService/UpdateApiConfig",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes a single ApiConfig."]
        pub async fn delete_api_config(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteApiConfigRequest>,
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
                "/google.cloud.apigateway.v1.ApiGatewayService/DeleteApiConfig",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
