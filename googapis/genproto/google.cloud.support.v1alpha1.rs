/// The request message for `GetSupportAccount`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSupportAccountRequest {
    /// The resource name of the support accounts. For example:
    /// `supportAccounts/accountA`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// The request message for `ListSupportAccount`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSupportAccountsRequest {
    /// The filter applied to search results. It only supports filtering a support
    /// account list by a cloud_resource. For example, to filter results by support
    /// accounts associated with an Organization, its value should be:
    /// "cloud_resource:organizations/<organization_id>"
    #[prost(string, tag = "1")]
    pub filter: ::prost::alloc::string::String,
    /// Maximum number of accounts fetched with each request.
    #[prost(int64, tag = "2")]
    pub page_size: i64,
    /// A token identifying the page of results to return. If unspecified, the
    /// first page is retrieved.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// The response message for `ListSupportAccount`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSupportAccountsResponse {
    /// A list of support accounts.
    #[prost(message, repeated, tag = "1")]
    pub accounts: ::prost::alloc::vec::Vec<super::common::SupportAccount>,
    /// A token to retrieve the next page of results. This should be passed on in
    /// `page_token` field of `ListSupportAccountRequest` for next request. If
    /// unspecified, there are no more results to retrieve.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// The request message for `GetCase` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCaseRequest {
    /// Name of case resource requested.
    /// For example: "supportAccounts/accountA/cases/123"
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// The request message for `ListCase` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCasesRequest {
    /// Name of the account resource for which cases are requested. For example:
    /// "supportAccounts/accountA"
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The filter applied to the search results. Currently it only accepts "OPEN"
    /// or "CLOSED" strings, filtering out cases that are open or resolved.
    #[prost(string, tag = "2")]
    pub filter: ::prost::alloc::string::String,
    /// Maximum number of cases fetched with each request.
    #[prost(int64, tag = "3")]
    pub page_size: i64,
    /// A token identifying the page of results to return. If unspecified, the
    /// first page is retrieved.
    #[prost(string, tag = "4")]
    pub page_token: ::prost::alloc::string::String,
}
/// The response message for `ListCase` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCasesResponse {
    /// A list of cases.
    #[prost(message, repeated, tag = "1")]
    pub cases: ::prost::alloc::vec::Vec<super::common::Case>,
    /// A token to retrieve the next page of results. This should be passed on in
    /// `page_token` field of `ListCaseRequest` for next request. If unspecified,
    /// there are no more results to retrieve.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// The request message for `ListComments` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCommentsRequest {
    /// The resource name of case for which comments should be listed.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// The response message for `ListComments` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCommentsResponse {
    /// A list of comments.
    #[prost(message, repeated, tag = "1")]
    pub comments: ::prost::alloc::vec::Vec<super::common::Comment>,
}
/// The request message for `CreateCase` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateCaseRequest {
    /// The resource name for `SupportAccount` under which this case is created.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The case resource to create.
    #[prost(message, optional, tag = "2")]
    pub case: ::core::option::Option<super::common::Case>,
}
/// The request message for `UpdateCase` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateCaseRequest {
    /// The case resource to update.
    #[prost(message, optional, tag = "1")]
    pub case: ::core::option::Option<super::common::Case>,
    /// A field that represents attributes of a Case object that should be updated
    /// as part of this request.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// The request message for `CreateComment` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateCommentRequest {
    /// The resource name of case to which this comment should be added.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The `Comment` to be added to this case.
    #[prost(message, optional, tag = "2")]
    pub comment: ::core::option::Option<super::common::Comment>,
}
/// The request message for `GetIssueTaxonomy` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetIssueTaxonomyRequest {}
#[doc = r" Generated client implementations."]
pub mod cloud_support_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Retrieves the list of Google Cloud Platform Support accounts and manages"]
    #[doc = " support cases associated with them."]
    #[derive(Debug, Clone)]
    pub struct CloudSupportClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> CloudSupportClient<T>
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
        ) -> CloudSupportClient<InterceptedService<T, F>>
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
            CloudSupportClient::new(InterceptedService::new(inner, interceptor))
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
}
