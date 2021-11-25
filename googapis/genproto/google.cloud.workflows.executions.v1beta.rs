/// A running instance of a \[Workflow][google.cloud.workflows.v1beta.Workflow\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Execution {
    /// Output only. The resource name of the execution.
    /// Format:
    /// projects/{project}/locations/{location}/workflows/{workflow}/executions/{execution}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. Marks the beginning of execution.
    #[prost(message, optional, tag = "2")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Marks the end of execution, successful or not.
    #[prost(message, optional, tag = "3")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Current state of the execution.
    #[prost(enumeration = "execution::State", tag = "4")]
    pub state: i32,
    /// Input parameters of the execution represented as a JSON string.
    /// The size limit is 32KB.
    #[prost(string, tag = "5")]
    pub argument: ::prost::alloc::string::String,
    /// Output only. Output of the execution represented as a JSON string. The
    /// value can only be present if the execution's state is `SUCCEEDED`.
    #[prost(string, tag = "6")]
    pub result: ::prost::alloc::string::String,
    /// Output only. The error which caused the execution to finish prematurely.
    /// The value is only present if the execution's state is `FAILED`
    /// or `CANCELLED`.
    #[prost(message, optional, tag = "7")]
    pub error: ::core::option::Option<execution::Error>,
    /// Output only. Revision of the workflow this execution is using.
    #[prost(string, tag = "8")]
    pub workflow_revision_id: ::prost::alloc::string::String,
}
/// Nested message and enum types in `Execution`.
pub mod execution {
    /// Error describes why the execution was abnormally terminated.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Error {
        /// Error payload returned by the execution, represented as a JSON string.
        #[prost(string, tag = "1")]
        pub payload: ::prost::alloc::string::String,
        /// Human readable error context, helpful for debugging purposes.
        #[prost(string, tag = "2")]
        pub context: ::prost::alloc::string::String,
    }
    /// Describes the current state of the execution. More states may be added
    /// in the future.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// Invalid state.
        Unspecified = 0,
        /// The execution is in progress.
        Active = 1,
        /// The execution finished successfully.
        Succeeded = 2,
        /// The execution failed with an error.
        Failed = 3,
        /// The execution was stopped intentionally.
        Cancelled = 4,
    }
}
/// Request for the
/// \[ListExecutions][google.cloud.workflows.executions.v1beta.Executions.ListExecutions\]
/// method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListExecutionsRequest {
    /// Required. Name of the workflow for which the executions should be listed.
    /// Format: projects/{project}/locations/{location}/workflows/{workflow}
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Maximum number of executions to return per call.
    /// Max supported value depends on the selected Execution view: it's 10000 for
    /// BASIC and 100 for FULL. The default value used if the field is not
    /// specified is 100, regardless of the selected view. Values greater than
    /// the max value will be coerced down to it.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A page token, received from a previous `ListExecutions` call.
    /// Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to `ListExecutions` must
    /// match the call that provided the page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. A view defining which fields should be filled in the returned executions.
    /// The API will default to the BASIC view.
    #[prost(enumeration = "ExecutionView", tag = "4")]
    pub view: i32,
}
/// Response for the
/// \[ListExecutions][google.cloud.workflows.executions.v1beta.Executions.ListExecutions\]
/// method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListExecutionsResponse {
    /// The executions which match the request.
    #[prost(message, repeated, tag = "1")]
    pub executions: ::prost::alloc::vec::Vec<Execution>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request for the
/// \[CreateExecution][google.cloud.workflows.executions.v1beta.Executions.CreateExecution\]
/// method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateExecutionRequest {
    /// Required. Name of the workflow for which an execution should be created.
    /// Format: projects/{project}/locations/{location}/workflows/{workflow}
    /// The latest revision of the workflow will be used.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. Execution to be created.
    #[prost(message, optional, tag = "2")]
    pub execution: ::core::option::Option<Execution>,
}
/// Request for the
/// \[GetExecution][google.cloud.workflows.executions.v1beta.Executions.GetExecution\]
/// method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetExecutionRequest {
    /// Required. Name of the execution to be retrieved.
    /// Format:
    /// projects/{project}/locations/{location}/workflows/{workflow}/executions/{execution}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. A view defining which fields should be filled in the returned execution.
    /// The API will default to the FULL view.
    #[prost(enumeration = "ExecutionView", tag = "2")]
    pub view: i32,
}
/// Request for the
/// \[CancelExecution][google.cloud.workflows.executions.v1beta.Executions.CancelExecution\]
/// method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CancelExecutionRequest {
    /// Required. Name of the execution to be cancelled.
    /// Format:
    /// projects/{project}/locations/{location}/workflows/{workflow}/executions/{execution}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Defines possible views for execution resource.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ExecutionView {
    /// The default / unset value.
    Unspecified = 0,
    /// Includes only basic metadata about the execution.
    /// Following fields are returned: name, start_time, end_time, state
    /// and workflow_revision_id.
    Basic = 1,
    /// Includes all data.
    Full = 2,
}
#[doc = r" Generated client implementations."]
pub mod executions_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Executions is used to start and manage running instances of"]
    #[doc = " [Workflows][google.cloud.workflows.v1beta.Workflow] called executions."]
    #[derive(Debug, Clone)]
    pub struct ExecutionsClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> ExecutionsClient<T>
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
        ) -> ExecutionsClient<InterceptedService<T, F>>
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
            ExecutionsClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Returns a list of executions which belong to the workflow with"]
        #[doc = " the given name. The method returns executions of all workflow"]
        #[doc = " revisions. Returned executions are ordered by their start time (newest"]
        #[doc = " first)."]
        pub async fn list_executions(
            &mut self,
            request: impl tonic::IntoRequest<super::ListExecutionsRequest>,
        ) -> Result<tonic::Response<super::ListExecutionsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.workflows.executions.v1beta.Executions/ListExecutions",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a new execution using the latest revision of the given workflow."]
        pub async fn create_execution(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateExecutionRequest>,
        ) -> Result<tonic::Response<super::Execution>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.workflows.executions.v1beta.Executions/CreateExecution",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns an execution of the given name."]
        pub async fn get_execution(
            &mut self,
            request: impl tonic::IntoRequest<super::GetExecutionRequest>,
        ) -> Result<tonic::Response<super::Execution>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.workflows.executions.v1beta.Executions/GetExecution",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Cancels an execution of the given name."]
        pub async fn cancel_execution(
            &mut self,
            request: impl tonic::IntoRequest<super::CancelExecutionRequest>,
        ) -> Result<tonic::Response<super::Execution>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.workflows.executions.v1beta.Executions/CancelExecution",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
