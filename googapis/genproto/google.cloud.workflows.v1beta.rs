/// Workflow program to be executed by Workflows.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Workflow {
    /// The resource name of the workflow.
    /// Format: projects/{project}/locations/{location}/workflows/{workflow}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Description of the workflow provided by the user.
    /// Must be at most 1000 unicode characters long.
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// Output only. State of the workflow deployment.
    #[prost(enumeration = "workflow::State", tag = "3")]
    pub state: i32,
    /// Output only. The revision of the workflow.
    /// A new revision of a workflow is created as a result of updating the
    /// following fields of a workflow:
    /// - `source_code`
    /// - `service_account`
    /// The format is "000001-a4d", where the first 6 characters define
    /// the zero-padded revision ordinal number. They are followed by a hyphen and
    /// 3 hexadecimal random characters.
    #[prost(string, tag = "4")]
    pub revision_id: ::prost::alloc::string::String,
    /// Output only. The timestamp of when the workflow was created.
    #[prost(message, optional, tag = "5")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The last update timestamp of the workflow.
    #[prost(message, optional, tag = "6")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The timestamp that the latest revision of the workflow
    /// was created.
    #[prost(message, optional, tag = "7")]
    pub revision_create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Labels associated with this workflow.
    /// Labels can contain at most 64 entries. Keys and values can be no longer
    /// than 63 characters and can only contain lowercase letters, numeric
    /// characters, underscores and dashes. Label keys must start with a letter.
    /// International characters are allowed.
    #[prost(map = "string, string", tag = "8")]
    pub labels:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Name of the service account associated with the latest workflow version.
    /// This service account represents the identity of the workflow and determines
    /// what permissions the workflow has.
    /// Format: projects/{project}/serviceAccounts/{account}
    ///
    /// Using `-` as a wildcard for the `{project}` will infer the project from
    /// the account. The `{account}` value can be the `email` address or the
    /// `unique_id` of the service account.
    ///
    /// If not provided, workflow will use the project's default service account.
    /// Modifying this field for an existing workflow results in a new workflow
    /// revision.
    #[prost(string, tag = "9")]
    pub service_account: ::prost::alloc::string::String,
    /// Required. Location of the workflow source code.
    /// Modifying this field for an existing workflow results in a new workflow
    /// revision.
    #[prost(oneof = "workflow::SourceCode", tags = "10")]
    pub source_code: ::core::option::Option<workflow::SourceCode>,
}
/// Nested message and enum types in `Workflow`.
pub mod workflow {
    /// Describes the current state of workflow deployment. More states may be
    /// added in the future.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// Invalid state.
        Unspecified = 0,
        /// The workflow has been deployed successfully and is serving.
        Active = 1,
    }
    /// Required. Location of the workflow source code.
    /// Modifying this field for an existing workflow results in a new workflow
    /// revision.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum SourceCode {
        /// Workflow code to be executed. The size limit is 32KB.
        #[prost(string, tag = "10")]
        SourceContents(::prost::alloc::string::String),
    }
}
/// Request for the
/// \[ListWorkflows][google.cloud.workflows.v1beta.Workflows.ListWorkflows\]
/// method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListWorkflowsRequest {
    /// Required. Project and location from which the workflows should be listed.
    /// Format: projects/{project}/locations/{location}
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Maximum number of workflows to return per call. The service may return
    /// fewer than this value. If the value is not specified, a default value of
    /// 500 will be used. The maximum permitted value is 1000 and values greater
    /// than 1000 will be coerced down to 1000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A page token, received from a previous `ListWorkflows` call.
    /// Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to `ListWorkflows` must
    /// match the call that provided the page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Filter to restrict results to specific workflows.
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Comma-separated list of fields that that specify the order of the results.
    /// Default sorting order for a field is ascending. To specify descending order
    /// for a field, append a " desc" suffix.
    /// If not specified, the results will be returned in an unspecified order.
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
/// Response for the
/// \[ListWorkflows][google.cloud.workflows.v1beta.Workflows.ListWorkflows\]
/// method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListWorkflowsResponse {
    /// The workflows which match the request.
    #[prost(message, repeated, tag = "1")]
    pub workflows: ::prost::alloc::vec::Vec<Workflow>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Unreachable resources.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Request for the
/// \[GetWorkflow][google.cloud.workflows.v1beta.Workflows.GetWorkflow\] method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetWorkflowRequest {
    /// Required. Name of the workflow which information should be retrieved.
    /// Format: projects/{project}/locations/{location}/workflows/{workflow}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request for the
/// \[CreateWorkflow][google.cloud.workflows.v1beta.Workflows.CreateWorkflow\]
/// method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateWorkflowRequest {
    /// Required. Project and location in which the workflow should be created.
    /// Format:  projects/{project}/locations/{location}
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. Workflow to be created.
    #[prost(message, optional, tag = "2")]
    pub workflow: ::core::option::Option<Workflow>,
    /// Required. The ID of the workflow to be created. It has to fulfill the
    /// following requirements:
    ///
    /// * Must contain only letters, numbers, underscores and hyphens.
    /// * Must start with a letter.
    /// * Must be between 1-64 characters.
    /// * Must end with a number or a letter.
    /// * Must be unique within the customer project and location.
    #[prost(string, tag = "3")]
    pub workflow_id: ::prost::alloc::string::String,
}
/// Request for the
/// \[DeleteWorkflow][google.cloud.workflows.v1beta.Workflows.DeleteWorkflow\]
/// method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteWorkflowRequest {
    /// Required. Name of the workflow to be deleted.
    /// Format: projects/{project}/locations/{location}/workflows/{workflow}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request for the
/// \[UpdateWorkflow][google.cloud.workflows.v1beta.Workflows.UpdateWorkflow\]
/// method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateWorkflowRequest {
    /// Required. Workflow to be updated.
    #[prost(message, optional, tag = "1")]
    pub workflow: ::core::option::Option<Workflow>,
    /// List of fields to be updated. If not present, the entire workflow
    /// will be updated.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Represents the metadata of the long-running operation.
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
    /// API version used to start the operation.
    #[prost(string, tag = "5")]
    pub api_version: ::prost::alloc::string::String,
}
#[doc = r" Generated client implementations."]
pub mod workflows_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Workflows is used to deploy and execute workflow programs."]
    #[doc = " Workflows makes sure the program executes reliably, despite hardware and"]
    #[doc = " networking interruptions."]
    #[derive(Debug, Clone)]
    pub struct WorkflowsClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> WorkflowsClient<T>
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
        ) -> WorkflowsClient<InterceptedService<T, F>>
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
            WorkflowsClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Lists Workflows in a given project and location."]
        #[doc = " The default order is not specified."]
        pub async fn list_workflows(
            &mut self,
            request: impl tonic::IntoRequest<super::ListWorkflowsRequest>,
        ) -> Result<tonic::Response<super::ListWorkflowsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.workflows.v1beta.Workflows/ListWorkflows",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets details of a single Workflow."]
        pub async fn get_workflow(
            &mut self,
            request: impl tonic::IntoRequest<super::GetWorkflowRequest>,
        ) -> Result<tonic::Response<super::Workflow>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.workflows.v1beta.Workflows/GetWorkflow",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a new workflow. If a workflow with the specified name already"]
        #[doc = " exists in the specified project and location, the long running operation"]
        #[doc = " will return [ALREADY_EXISTS][google.rpc.Code.ALREADY_EXISTS] error."]
        pub async fn create_workflow(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateWorkflowRequest>,
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
                "/google.cloud.workflows.v1beta.Workflows/CreateWorkflow",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes a workflow with the specified name."]
        #[doc = " This method also cancels and deletes all running executions of the"]
        #[doc = " workflow."]
        pub async fn delete_workflow(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteWorkflowRequest>,
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
                "/google.cloud.workflows.v1beta.Workflows/DeleteWorkflow",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates an existing workflow."]
        #[doc = " Running this method has no impact on already running executions of the"]
        #[doc = " workflow. A new revision of the workflow may be created as a result of a"]
        #[doc = " successful update operation. In that case, such revision will be used"]
        #[doc = " in new workflow executions."]
        pub async fn update_workflow(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateWorkflowRequest>,
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
                "/google.cloud.workflows.v1beta.Workflows/UpdateWorkflow",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
