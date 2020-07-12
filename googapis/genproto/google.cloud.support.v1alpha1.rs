/// The request message for `GetSupportAccount`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSupportAccountRequest {
    /// The resource name of the support accounts. For example:
    /// `supportAccounts/accountA`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// The request message for `ListSupportAccount`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSupportAccountsRequest {
    /// The filter applied to search results. It only supports filtering a support
    /// account list by a cloud_resource. For example, to filter results by support
    /// accounts associated with an Organization, its value should be:
    /// "cloud_resource:organizations/<organization_id>"
    #[prost(string, tag = "1")]
    pub filter: std::string::String,
    /// Maximum number of accounts fetched with each request.
    #[prost(int64, tag = "2")]
    pub page_size: i64,
    /// A token identifying the page of results to return. If unspecified, the
    /// first page is retrieved.
    #[prost(string, tag = "3")]
    pub page_token: std::string::String,
}
/// The response message for `ListSupportAccount`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSupportAccountsResponse {
    /// A list of support accounts.
    #[prost(message, repeated, tag = "1")]
    pub accounts: ::std::vec::Vec<super::common::SupportAccount>,
    /// A token to retrieve the next page of results. This should be passed on in
    /// `page_token` field of `ListSupportAccountRequest` for next request. If
    /// unspecified, there are no more results to retrieve.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
}
/// The request message for `GetCase` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCaseRequest {
    /// Name of case resource requested.
    /// For example: "supportAccounts/accountA/cases/123"
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// The request message for `ListCase` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCasesRequest {
    /// Name of the account resource for which cases are requested. For example:
    /// "supportAccounts/accountA"
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// The filter applied to the search results. Currently it only accepts "OPEN"
    /// or "CLOSED" strings, filtering out cases that are open or resolved.
    #[prost(string, tag = "2")]
    pub filter: std::string::String,
    /// Maximum number of cases fetched with each request.
    #[prost(int64, tag = "3")]
    pub page_size: i64,
    /// A token identifying the page of results to return. If unspecified, the
    /// first page is retrieved.
    #[prost(string, tag = "4")]
    pub page_token: std::string::String,
}
/// The response message for `ListCase` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCasesResponse {
    /// A list of cases.
    #[prost(message, repeated, tag = "1")]
    pub cases: ::std::vec::Vec<super::common::Case>,
    /// A token to retrieve the next page of results. This should be passed on in
    /// `page_token` field of `ListCaseRequest` for next request. If unspecified,
    /// there are no more results to retrieve.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
}
/// The request message for `ListComments` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCommentsRequest {
    /// The resource name of case for which comments should be listed.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// The response message for `ListComments` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCommentsResponse {
    /// A list of comments.
    #[prost(message, repeated, tag = "1")]
    pub comments: ::std::vec::Vec<super::common::Comment>,
}
/// The request message for `CreateCase` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateCaseRequest {
    /// The resource name for `SupportAccount` under which this case is created.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// The case resource to create.
    #[prost(message, optional, tag = "2")]
    pub case: ::std::option::Option<super::common::Case>,
}
/// The request message for `UpdateCase` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateCaseRequest {
    /// The case resource to update.
    #[prost(message, optional, tag = "1")]
    pub case: ::std::option::Option<super::common::Case>,
    /// A field that represents attributes of a Case object that should be updated
    /// as part of this request.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
}
/// The request message for `CreateComment` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateCommentRequest {
    /// The resource name of case to which this comment should be added.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// The `Comment` to be added to this case.
    #[prost(message, optional, tag = "2")]
    pub comment: ::std::option::Option<super::common::Comment>,
}
/// The request message for `GetIssueTaxonomy` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetIssueTaxonomyRequest {}
#[doc = r" Generated client implementations."]
pub mod cloud_support_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Retrieves the list of Google Cloud Platform Support accounts and manages"]
    #[doc = " support cases associated with them."]
    pub struct CloudSupportClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> CloudSupportClient<T>
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
        #[doc = " Retrieves the support account details given an account identifier."]
        #[doc = " The authenticated user calling this method must be the account owner."]
        pub async fn get_support_account(
            &mut self,
            request: impl tonic::IntoRequest<super::GetSupportAccountRequest>,
        ) -> Result<tonic::Response<super::super::common::SupportAccount>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.support.v1alpha1.CloudSupport/GetSupportAccount",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Retrieves the list of accounts the current authenticated user has access"]
        #[doc = " to."]
        pub async fn list_support_accounts(
            &mut self,
            request: impl tonic::IntoRequest<super::ListSupportAccountsRequest>,
        ) -> Result<tonic::Response<super::ListSupportAccountsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.support.v1alpha1.CloudSupport/ListSupportAccounts",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Retrieves the details for a support case. The current authenticated user"]
        #[doc = " calling this method must have permissions to view this case."]
        pub async fn get_case(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCaseRequest>,
        ) -> Result<tonic::Response<super::super::common::Case>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.support.v1alpha1.CloudSupport/GetCase",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Retrieves the list of support cases associated with an account. The current"]
        #[doc = " authenticated user must have the permission to list and view these cases."]
        pub async fn list_cases(
            &mut self,
            request: impl tonic::IntoRequest<super::ListCasesRequest>,
        ) -> Result<tonic::Response<super::ListCasesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.support.v1alpha1.CloudSupport/ListCases",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists all comments from a case."]
        pub async fn list_comments(
            &mut self,
            request: impl tonic::IntoRequest<super::ListCommentsRequest>,
        ) -> Result<tonic::Response<super::ListCommentsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.support.v1alpha1.CloudSupport/ListComments",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a case and associates it with a"]
        #[doc = " [SupportAccount][google.cloud.support.v1alpha2.SupportAcccount]. The"]
        #[doc = " authenticated user attempting this action must have permissions to create a"]
        #[doc = " `Case` under that [SupportAccount]."]
        pub async fn create_case(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateCaseRequest>,
        ) -> Result<tonic::Response<super::super::common::Case>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.support.v1alpha1.CloudSupport/CreateCase",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates a support case. Only a small set of details (priority, subject and"]
        #[doc = " cc_address) can be update after a case is created."]
        pub async fn update_case(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateCaseRequest>,
        ) -> Result<tonic::Response<super::super::common::Case>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.support.v1alpha1.CloudSupport/UpdateCase",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Adds a new comment to a case."]
        pub async fn create_comment(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateCommentRequest>,
        ) -> Result<tonic::Response<super::super::common::Comment>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.support.v1alpha1.CloudSupport/CreateComment",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Retrieves the taxonomy of product categories and components to be used"]
        #[doc = " while creating a support case."]
        pub async fn get_issue_taxonomy(
            &mut self,
            request: impl tonic::IntoRequest<super::GetIssueTaxonomyRequest>,
        ) -> Result<tonic::Response<super::super::common::IssueTaxonomy>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.support.v1alpha1.CloudSupport/GetIssueTaxonomy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for CloudSupportClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for CloudSupportClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "CloudSupportClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod cloud_support_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with CloudSupportServer."]
    #[async_trait]
    pub trait CloudSupport: Send + Sync + 'static {
        #[doc = " Retrieves the support account details given an account identifier."]
        #[doc = " The authenticated user calling this method must be the account owner."]
        async fn get_support_account(
            &self,
            request: tonic::Request<super::GetSupportAccountRequest>,
        ) -> Result<tonic::Response<super::super::common::SupportAccount>, tonic::Status>;
        #[doc = " Retrieves the list of accounts the current authenticated user has access"]
        #[doc = " to."]
        async fn list_support_accounts(
            &self,
            request: tonic::Request<super::ListSupportAccountsRequest>,
        ) -> Result<tonic::Response<super::ListSupportAccountsResponse>, tonic::Status>;
        #[doc = " Retrieves the details for a support case. The current authenticated user"]
        #[doc = " calling this method must have permissions to view this case."]
        async fn get_case(
            &self,
            request: tonic::Request<super::GetCaseRequest>,
        ) -> Result<tonic::Response<super::super::common::Case>, tonic::Status>;
        #[doc = " Retrieves the list of support cases associated with an account. The current"]
        #[doc = " authenticated user must have the permission to list and view these cases."]
        async fn list_cases(
            &self,
            request: tonic::Request<super::ListCasesRequest>,
        ) -> Result<tonic::Response<super::ListCasesResponse>, tonic::Status>;
        #[doc = " Lists all comments from a case."]
        async fn list_comments(
            &self,
            request: tonic::Request<super::ListCommentsRequest>,
        ) -> Result<tonic::Response<super::ListCommentsResponse>, tonic::Status>;
        #[doc = " Creates a case and associates it with a"]
        #[doc = " [SupportAccount][google.cloud.support.v1alpha2.SupportAcccount]. The"]
        #[doc = " authenticated user attempting this action must have permissions to create a"]
        #[doc = " `Case` under that [SupportAccount]."]
        async fn create_case(
            &self,
            request: tonic::Request<super::CreateCaseRequest>,
        ) -> Result<tonic::Response<super::super::common::Case>, tonic::Status>;
        #[doc = " Updates a support case. Only a small set of details (priority, subject and"]
        #[doc = " cc_address) can be update after a case is created."]
        async fn update_case(
            &self,
            request: tonic::Request<super::UpdateCaseRequest>,
        ) -> Result<tonic::Response<super::super::common::Case>, tonic::Status>;
        #[doc = " Adds a new comment to a case."]
        async fn create_comment(
            &self,
            request: tonic::Request<super::CreateCommentRequest>,
        ) -> Result<tonic::Response<super::super::common::Comment>, tonic::Status>;
        #[doc = " Retrieves the taxonomy of product categories and components to be used"]
        #[doc = " while creating a support case."]
        async fn get_issue_taxonomy(
            &self,
            request: tonic::Request<super::GetIssueTaxonomyRequest>,
        ) -> Result<tonic::Response<super::super::common::IssueTaxonomy>, tonic::Status>;
    }
    #[doc = " Retrieves the list of Google Cloud Platform Support accounts and manages"]
    #[doc = " support cases associated with them."]
    #[derive(Debug)]
    #[doc(hidden)]
    pub struct CloudSupportServer<T: CloudSupport> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: CloudSupport> CloudSupportServer<T> {
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
    impl<T, B> Service<http::Request<B>> for CloudSupportServer<T>
    where
        T: CloudSupport,
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
                "/google.cloud.support.v1alpha1.CloudSupport/GetSupportAccount" => {
                    #[allow(non_camel_case_types)]
                    struct GetSupportAccountSvc<T: CloudSupport>(pub Arc<T>);
                    impl<T: CloudSupport>
                        tonic::server::UnaryService<super::GetSupportAccountRequest>
                        for GetSupportAccountSvc<T>
                    {
                        type Response = super::super::common::SupportAccount;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetSupportAccountRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.get_support_account(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetSupportAccountSvc(inner);
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
                "/google.cloud.support.v1alpha1.CloudSupport/ListSupportAccounts" => {
                    #[allow(non_camel_case_types)]
                    struct ListSupportAccountsSvc<T: CloudSupport>(pub Arc<T>);
                    impl<T: CloudSupport>
                        tonic::server::UnaryService<super::ListSupportAccountsRequest>
                        for ListSupportAccountsSvc<T>
                    {
                        type Response = super::ListSupportAccountsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListSupportAccountsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.list_support_accounts(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ListSupportAccountsSvc(inner);
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
                "/google.cloud.support.v1alpha1.CloudSupport/GetCase" => {
                    #[allow(non_camel_case_types)]
                    struct GetCaseSvc<T: CloudSupport>(pub Arc<T>);
                    impl<T: CloudSupport> tonic::server::UnaryService<super::GetCaseRequest> for GetCaseSvc<T> {
                        type Response = super::super::common::Case;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetCaseRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.get_case(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetCaseSvc(inner);
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
                "/google.cloud.support.v1alpha1.CloudSupport/ListCases" => {
                    #[allow(non_camel_case_types)]
                    struct ListCasesSvc<T: CloudSupport>(pub Arc<T>);
                    impl<T: CloudSupport> tonic::server::UnaryService<super::ListCasesRequest> for ListCasesSvc<T> {
                        type Response = super::ListCasesResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListCasesRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.list_cases(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ListCasesSvc(inner);
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
                "/google.cloud.support.v1alpha1.CloudSupport/ListComments" => {
                    #[allow(non_camel_case_types)]
                    struct ListCommentsSvc<T: CloudSupport>(pub Arc<T>);
                    impl<T: CloudSupport> tonic::server::UnaryService<super::ListCommentsRequest>
                        for ListCommentsSvc<T>
                    {
                        type Response = super::ListCommentsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListCommentsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.list_comments(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ListCommentsSvc(inner);
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
                "/google.cloud.support.v1alpha1.CloudSupport/CreateCase" => {
                    #[allow(non_camel_case_types)]
                    struct CreateCaseSvc<T: CloudSupport>(pub Arc<T>);
                    impl<T: CloudSupport> tonic::server::UnaryService<super::CreateCaseRequest> for CreateCaseSvc<T> {
                        type Response = super::super::common::Case;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateCaseRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.create_case(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = CreateCaseSvc(inner);
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
                "/google.cloud.support.v1alpha1.CloudSupport/UpdateCase" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateCaseSvc<T: CloudSupport>(pub Arc<T>);
                    impl<T: CloudSupport> tonic::server::UnaryService<super::UpdateCaseRequest> for UpdateCaseSvc<T> {
                        type Response = super::super::common::Case;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateCaseRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.update_case(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = UpdateCaseSvc(inner);
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
                "/google.cloud.support.v1alpha1.CloudSupport/CreateComment" => {
                    #[allow(non_camel_case_types)]
                    struct CreateCommentSvc<T: CloudSupport>(pub Arc<T>);
                    impl<T: CloudSupport> tonic::server::UnaryService<super::CreateCommentRequest>
                        for CreateCommentSvc<T>
                    {
                        type Response = super::super::common::Comment;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateCommentRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.create_comment(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = CreateCommentSvc(inner);
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
                "/google.cloud.support.v1alpha1.CloudSupport/GetIssueTaxonomy" => {
                    #[allow(non_camel_case_types)]
                    struct GetIssueTaxonomySvc<T: CloudSupport>(pub Arc<T>);
                    impl<T: CloudSupport>
                        tonic::server::UnaryService<super::GetIssueTaxonomyRequest>
                        for GetIssueTaxonomySvc<T>
                    {
                        type Response = super::super::common::IssueTaxonomy;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetIssueTaxonomyRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.get_issue_taxonomy(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetIssueTaxonomySvc(inner);
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
    impl<T: CloudSupport> Clone for CloudSupportServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: CloudSupport> Clone for _Inner<T> {
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
