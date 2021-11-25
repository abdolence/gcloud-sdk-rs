/// Presents a workspace
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Workspace {
    /// The Workspace name in the format of "projects/*/locations/*/workspaces/*".
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for listing Workspaces.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListWorkspacesRequest {
    /// The parent used for listing. It should have the format of
    /// `projects/{number}/locations/{location}`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The page size for list pagination.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The page token for list pagination.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// A list of workspaces.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListWorkspacesResponse {
    /// The list of workspaces.
    #[prost(message, repeated, tag = "1")]
    pub items: ::prost::alloc::vec::Vec<Workspace>,
    /// The next page token for list pagination.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for retrieving a Workspace.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetWorkspaceRequest {
    /// The name of the Workspace to retrieve.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for creating a Workspace.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateWorkspaceRequest {
    /// The namespace in which the Workspace should be created.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The Workspace instance to create.
    #[prost(message, optional, tag = "2")]
    pub workspace: ::core::option::Option<Workspace>,
}
/// Request message for replacing a Workspace.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateWorkspaceRequest {
    /// The name of the Workspace being replaced.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The Workspace object being replaced.
    #[prost(message, optional, tag = "2")]
    pub workspace: ::core::option::Option<Workspace>,
}
/// Request message for deleting a Workspace.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteWorkspaceRequest {
    /// The name of the Workspace to delete.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
#[doc = r" Generated client implementations."]
pub mod workspaces_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Manages workspaces."]
    #[derive(Debug, Clone)]
    pub struct WorkspacesClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> WorkspacesClient<T>
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
        ) -> WorkspacesClient<InterceptedService<T, F>>
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
            WorkspacesClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " List workspaces."]
        pub async fn list_workspaces(
            &mut self,
            request: impl tonic::IntoRequest<super::ListWorkspacesRequest>,
        ) -> Result<tonic::Response<super::ListWorkspacesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.example.endpointsapis.v1.Workspaces/ListWorkspaces",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Get information about a Workspace."]
        pub async fn get_workspace(
            &mut self,
            request: impl tonic::IntoRequest<super::GetWorkspaceRequest>,
        ) -> Result<tonic::Response<super::Workspace>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.example.endpointsapis.v1.Workspaces/GetWorkspace",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Create a Workspace."]
        pub async fn create_workspace(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateWorkspaceRequest>,
        ) -> Result<tonic::Response<super::Workspace>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.example.endpointsapis.v1.Workspaces/CreateWorkspace",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates a Workspace."]
        pub async fn update_workspace(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateWorkspaceRequest>,
        ) -> Result<tonic::Response<super::Workspace>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.example.endpointsapis.v1.Workspaces/UpdateWorkspace",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes a Workspace."]
        pub async fn delete_workspace(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteWorkspaceRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.example.endpointsapis.v1.Workspaces/DeleteWorkspace",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
