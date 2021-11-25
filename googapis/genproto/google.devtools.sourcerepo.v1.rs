/// A repository (or repo) is a Git repository storing versioned source content.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Repo {
    /// Resource name of the repository, of the form
    /// `projects/<project>/repos/<repo>`.  The repo name may contain slashes.
    /// eg, `projects/myproject/repos/name/with/slash`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The disk usage of the repo, in bytes. Read-only field. Size is only
    /// returned by GetRepo.
    #[prost(int64, tag = "2")]
    pub size: i64,
    /// URL to clone the repository from Google Cloud Source Repositories.
    /// Read-only field.
    #[prost(string, tag = "3")]
    pub url: ::prost::alloc::string::String,
    /// How this repository mirrors a repository managed by another service.
    /// Read-only field.
    #[prost(message, optional, tag = "4")]
    pub mirror_config: ::core::option::Option<MirrorConfig>,
}
/// Configuration to automatically mirror a repository from another
/// hosting service, for example GitHub or BitBucket.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MirrorConfig {
    /// URL of the main repository at the other hosting service.
    #[prost(string, tag = "1")]
    pub url: ::prost::alloc::string::String,
    /// ID of the webhook listening to updates to trigger mirroring.
    /// Removing this webhook from the other hosting service will stop
    /// Google Cloud Source Repositories from receiving notifications,
    /// and thereby disabling mirroring.
    #[prost(string, tag = "2")]
    pub webhook_id: ::prost::alloc::string::String,
    /// ID of the SSH deploy key at the other hosting service.
    /// Removing this key from the other service would deauthorize
    /// Google Cloud Source Repositories from mirroring.
    #[prost(string, tag = "3")]
    pub deploy_key_id: ::prost::alloc::string::String,
}
/// Request for GetRepo.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRepoRequest {
    /// The name of the requested repository. Values are of the form
    /// `projects/<project>/repos/<repo>`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request for ListRepos.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListReposRequest {
    /// The project ID whose repos should be listed. Values are of the form
    /// `projects/<project>`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Maximum number of repositories to return; between 1 and 500.
    /// If not set or zero, defaults to 100 at the server.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Resume listing repositories where a prior ListReposResponse
    /// left off. This is an opaque token that must be obtained from
    /// a recent, prior ListReposResponse's next_page_token field.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response for ListRepos.  The size is not set in the returned repositories.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListReposResponse {
    /// The listed repos.
    #[prost(message, repeated, tag = "1")]
    pub repos: ::prost::alloc::vec::Vec<Repo>,
    /// If non-empty, additional repositories exist within the project. These
    /// can be retrieved by including this value in the next ListReposRequest's
    /// page_token field.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request for CreateRepo
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateRepoRequest {
    /// The project in which to create the repo. Values are of the form
    /// `projects/<project>`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The repo to create.  Only name should be set; setting other fields
    /// is an error.  The project in the name should match the parent field.
    #[prost(message, optional, tag = "2")]
    pub repo: ::core::option::Option<Repo>,
}
/// Request for DeleteRepo.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteRepoRequest {
    /// The name of the repo to delete. Values are of the form
    /// `projects/<project>/repos/<repo>`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
#[doc = r" Generated client implementations."]
pub mod source_repo_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " The Source Repo API service."]
    #[derive(Debug, Clone)]
    pub struct SourceRepoClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> SourceRepoClient<T>
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
        ) -> SourceRepoClient<InterceptedService<T, F>>
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
            SourceRepoClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Returns all repos belonging to a project. The sizes of the repos are"]
        #[doc = " not set by ListRepos.  To get the size of a repo, use GetRepo."]
        pub async fn list_repos(
            &mut self,
            request: impl tonic::IntoRequest<super::ListReposRequest>,
        ) -> Result<tonic::Response<super::ListReposResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.devtools.sourcerepo.v1.SourceRepo/ListRepos",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns information about a repo."]
        pub async fn get_repo(
            &mut self,
            request: impl tonic::IntoRequest<super::GetRepoRequest>,
        ) -> Result<tonic::Response<super::Repo>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.devtools.sourcerepo.v1.SourceRepo/GetRepo",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a repo in the given project with the given name."]
        #[doc = ""]
        #[doc = " If the named repository already exists, `CreateRepo` returns"]
        #[doc = " `ALREADY_EXISTS`."]
        pub async fn create_repo(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateRepoRequest>,
        ) -> Result<tonic::Response<super::Repo>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.devtools.sourcerepo.v1.SourceRepo/CreateRepo",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes a repo."]
        pub async fn delete_repo(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteRepoRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.devtools.sourcerepo.v1.SourceRepo/DeleteRepo",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Sets the access control policy on the specified resource. Replaces any"]
        #[doc = " existing policy."]
        pub async fn set_iam_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::super::super::super::iam::v1::SetIamPolicyRequest>,
        ) -> Result<tonic::Response<super::super::super::super::iam::v1::Policy>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.devtools.sourcerepo.v1.SourceRepo/SetIamPolicy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets the access control policy for a resource."]
        #[doc = " Returns an empty policy if the resource exists and does not have a policy"]
        #[doc = " set."]
        pub async fn get_iam_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::super::super::super::iam::v1::GetIamPolicyRequest>,
        ) -> Result<tonic::Response<super::super::super::super::iam::v1::Policy>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.devtools.sourcerepo.v1.SourceRepo/GetIamPolicy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns permissions that a caller has on the specified resource."]
        #[doc = " If the resource does not exist, this will return an empty set of"]
        #[doc = " permissions, not a NOT_FOUND error."]
        pub async fn test_iam_permissions(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::iam::v1::TestIamPermissionsRequest,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::iam::v1::TestIamPermissionsResponse>,
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
                "/google.devtools.sourcerepo.v1.SourceRepo/TestIamPermissions",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
