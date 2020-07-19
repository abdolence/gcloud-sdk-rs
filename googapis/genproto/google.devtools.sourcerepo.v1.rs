/// A repository (or repo) is a Git repository storing versioned source content.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Repo {
    /// Resource name of the repository, of the form
    /// `projects/<project>/repos/<repo>`.  The repo name may contain slashes.
    /// eg, `projects/myproject/repos/name/with/slash`
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// The disk usage of the repo, in bytes. Read-only field. Size is only
    /// returned by GetRepo.
    #[prost(int64, tag = "2")]
    pub size: i64,
    /// URL to clone the repository from Google Cloud Source Repositories.
    /// Read-only field.
    #[prost(string, tag = "3")]
    pub url: std::string::String,
    /// How this repository mirrors a repository managed by another service.
    /// Read-only field.
    #[prost(message, optional, tag = "4")]
    pub mirror_config: ::std::option::Option<MirrorConfig>,
}
/// Configuration to automatically mirror a repository from another
/// hosting service, for example GitHub or BitBucket.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MirrorConfig {
    /// URL of the main repository at the other hosting service.
    #[prost(string, tag = "1")]
    pub url: std::string::String,
    /// ID of the webhook listening to updates to trigger mirroring.
    /// Removing this webhook from the other hosting service will stop
    /// Google Cloud Source Repositories from receiving notifications,
    /// and thereby disabling mirroring.
    #[prost(string, tag = "2")]
    pub webhook_id: std::string::String,
    /// ID of the SSH deploy key at the other hosting service.
    /// Removing this key from the other service would deauthorize
    /// Google Cloud Source Repositories from mirroring.
    #[prost(string, tag = "3")]
    pub deploy_key_id: std::string::String,
}
/// Request for GetRepo.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRepoRequest {
    /// The name of the requested repository. Values are of the form
    /// `projects/<project>/repos/<repo>`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Request for ListRepos.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListReposRequest {
    /// The project ID whose repos should be listed. Values are of the form
    /// `projects/<project>`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Maximum number of repositories to return; between 1 and 500.
    /// If not set or zero, defaults to 100 at the server.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Resume listing repositories where a prior ListReposResponse
    /// left off. This is an opaque token that must be obtained from
    /// a recent, prior ListReposResponse's next_page_token field.
    #[prost(string, tag = "3")]
    pub page_token: std::string::String,
}
/// Response for ListRepos.  The size is not set in the returned repositories.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListReposResponse {
    /// The listed repos.
    #[prost(message, repeated, tag = "1")]
    pub repos: ::std::vec::Vec<Repo>,
    /// If non-empty, additional repositories exist within the project. These
    /// can be retrieved by including this value in the next ListReposRequest's
    /// page_token field.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
}
/// Request for CreateRepo
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateRepoRequest {
    /// The project in which to create the repo. Values are of the form
    /// `projects/<project>`.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// The repo to create.  Only name should be set; setting other fields
    /// is an error.  The project in the name should match the parent field.
    #[prost(message, optional, tag = "2")]
    pub repo: ::std::option::Option<Repo>,
}
/// Request for DeleteRepo.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteRepoRequest {
    /// The name of the repo to delete. Values are of the form
    /// `projects/<project>/repos/<repo>`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
#[doc = r" Generated client implementations."]
pub mod source_repo_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " The Source Repo API service."]
    pub struct SourceRepoClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> SourceRepoClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + HttpBody + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as HttpBody>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = tonic::client::Grpc::with_interceptor(inner, interceptor);
            Self { inner }
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
    impl<T: Clone> Clone for SourceRepoClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for SourceRepoClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "SourceRepoClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod source_repo_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with SourceRepoServer."]
    #[async_trait]
    pub trait SourceRepo: Send + Sync + 'static {
        #[doc = " Returns all repos belonging to a project. The sizes of the repos are"]
        #[doc = " not set by ListRepos.  To get the size of a repo, use GetRepo."]
        async fn list_repos(
            &self,
            request: tonic::Request<super::ListReposRequest>,
        ) -> Result<tonic::Response<super::ListReposResponse>, tonic::Status>;
        #[doc = " Returns information about a repo."]
        async fn get_repo(
            &self,
            request: tonic::Request<super::GetRepoRequest>,
        ) -> Result<tonic::Response<super::Repo>, tonic::Status>;
        #[doc = " Creates a repo in the given project with the given name."]
        #[doc = ""]
        #[doc = " If the named repository already exists, `CreateRepo` returns"]
        #[doc = " `ALREADY_EXISTS`."]
        async fn create_repo(
            &self,
            request: tonic::Request<super::CreateRepoRequest>,
        ) -> Result<tonic::Response<super::Repo>, tonic::Status>;
        #[doc = " Deletes a repo."]
        async fn delete_repo(
            &self,
            request: tonic::Request<super::DeleteRepoRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status>;
        #[doc = " Sets the access control policy on the specified resource. Replaces any"]
        #[doc = " existing policy."]
        async fn set_iam_policy(
            &self,
            request: tonic::Request<super::super::super::super::iam::v1::SetIamPolicyRequest>,
        ) -> Result<tonic::Response<super::super::super::super::iam::v1::Policy>, tonic::Status>;
        #[doc = " Gets the access control policy for a resource."]
        #[doc = " Returns an empty policy if the resource exists and does not have a policy"]
        #[doc = " set."]
        async fn get_iam_policy(
            &self,
            request: tonic::Request<super::super::super::super::iam::v1::GetIamPolicyRequest>,
        ) -> Result<tonic::Response<super::super::super::super::iam::v1::Policy>, tonic::Status>;
        #[doc = " Returns permissions that a caller has on the specified resource."]
        #[doc = " If the resource does not exist, this will return an empty set of"]
        #[doc = " permissions, not a NOT_FOUND error."]
        async fn test_iam_permissions(
            &self,
            request: tonic::Request<super::super::super::super::iam::v1::TestIamPermissionsRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::iam::v1::TestIamPermissionsResponse>,
            tonic::Status,
        >;
    }
    #[doc = " The Source Repo API service."]
    #[derive(Debug)]
    pub struct SourceRepoServer<T: SourceRepo> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: SourceRepo> SourceRepoServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, None);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, Some(interceptor.into()));
            Self { inner }
        }
    }
    impl<T, B> Service<http::Request<B>> for SourceRepoServer<T>
    where
        T: SourceRepo,
        B: HttpBody + Send + Sync + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/google.devtools.sourcerepo.v1.SourceRepo/ListRepos" => {
                    #[allow(non_camel_case_types)]
                    struct ListReposSvc<T: SourceRepo>(pub Arc<T>);
                    impl<T: SourceRepo> tonic::server::UnaryService<super::ListReposRequest> for ListReposSvc<T> {
                        type Response = super::ListReposResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListReposRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).list_repos(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ListReposSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/google.devtools.sourcerepo.v1.SourceRepo/GetRepo" => {
                    #[allow(non_camel_case_types)]
                    struct GetRepoSvc<T: SourceRepo>(pub Arc<T>);
                    impl<T: SourceRepo> tonic::server::UnaryService<super::GetRepoRequest> for GetRepoSvc<T> {
                        type Response = super::Repo;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetRepoRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_repo(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetRepoSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/google.devtools.sourcerepo.v1.SourceRepo/CreateRepo" => {
                    #[allow(non_camel_case_types)]
                    struct CreateRepoSvc<T: SourceRepo>(pub Arc<T>);
                    impl<T: SourceRepo> tonic::server::UnaryService<super::CreateRepoRequest> for CreateRepoSvc<T> {
                        type Response = super::Repo;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateRepoRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).create_repo(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = CreateRepoSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/google.devtools.sourcerepo.v1.SourceRepo/DeleteRepo" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteRepoSvc<T: SourceRepo>(pub Arc<T>);
                    impl<T: SourceRepo> tonic::server::UnaryService<super::DeleteRepoRequest> for DeleteRepoSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteRepoRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).delete_repo(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = DeleteRepoSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/google.devtools.sourcerepo.v1.SourceRepo/SetIamPolicy" => {
                    #[allow(non_camel_case_types)]
                    struct SetIamPolicySvc<T: SourceRepo>(pub Arc<T>);
                    impl<T: SourceRepo>
                        tonic::server::UnaryService<
                            super::super::super::super::iam::v1::SetIamPolicyRequest,
                        > for SetIamPolicySvc<T>
                    {
                        type Response = super::super::super::super::iam::v1::Policy;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::super::super::iam::v1::SetIamPolicyRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).set_iam_policy(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = SetIamPolicySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/google.devtools.sourcerepo.v1.SourceRepo/GetIamPolicy" => {
                    #[allow(non_camel_case_types)]
                    struct GetIamPolicySvc<T: SourceRepo>(pub Arc<T>);
                    impl<T: SourceRepo>
                        tonic::server::UnaryService<
                            super::super::super::super::iam::v1::GetIamPolicyRequest,
                        > for GetIamPolicySvc<T>
                    {
                        type Response = super::super::super::super::iam::v1::Policy;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::super::super::iam::v1::GetIamPolicyRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_iam_policy(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetIamPolicySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/google.devtools.sourcerepo.v1.SourceRepo/TestIamPermissions" => {
                    #[allow(non_camel_case_types)]
                    struct TestIamPermissionsSvc<T: SourceRepo>(pub Arc<T>);
                    impl<T: SourceRepo>
                        tonic::server::UnaryService<
                            super::super::super::super::iam::v1::TestIamPermissionsRequest,
                        > for TestIamPermissionsSvc<T>
                    {
                        type Response =
                            super::super::super::super::iam::v1::TestIamPermissionsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::super::super::iam::v1::TestIamPermissionsRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).test_iam_permissions(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = TestIamPermissionsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .body(tonic::body::BoxBody::empty())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: SourceRepo> Clone for SourceRepoServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: SourceRepo> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
}
