/// The standard metadata of a cloud resource.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StandardResourceMetadata {
    /// The full resource name. For example:
    /// `//compute.googleapis.com/projects/my_project_123/zones/zone1/instances/instance1`.
    /// See [Resource
    /// Names](https://cloud.google.com/apis/design/resource_names#full_resource_name)
    /// for more information.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// The type of this resource.
    /// For example: "compute.googleapis.com/Disk".
    #[prost(string, tag = "2")]
    pub asset_type: std::string::String,
    /// The project that this resource belongs to, in the form of
    /// `projects/{project_number}`.
    #[prost(string, tag = "3")]
    pub project: std::string::String,
    /// The display name of this resource.
    #[prost(string, tag = "4")]
    pub display_name: std::string::String,
    /// One or more paragraphs of text description of this resource. Maximum length
    /// could be up to 1M bytes.
    #[prost(string, tag = "5")]
    pub description: std::string::String,
    /// Additional searchable attributes of this resource.
    /// Informational only. The exact set of attributes is subject to change.
    /// For example: project id, DNS name etc.
    #[prost(string, repeated, tag = "10")]
    pub additional_attributes: ::std::vec::Vec<std::string::String>,
    /// Location can be "global", regional like "us-east1", or zonal like
    /// "us-west1-b".
    #[prost(string, tag = "11")]
    pub location: std::string::String,
    /// Labels associated with this resource. See [Labelling and grouping GCP
    /// resources](https://cloud.google.com/blog/products/gcp/labelling-and-grouping-your-google-cloud-platform-resources)
    /// for more information.
    #[prost(map = "string, string", tag = "12")]
    pub labels: ::std::collections::HashMap<std::string::String, std::string::String>,
    /// Network tags associated with this resource. Like labels, network tags are a
    /// type of annotations used to group GCP resources. See [Labelling GCP
    /// resources](lhttps://cloud.google.com/blog/products/gcp/labelling-and-grouping-your-google-cloud-platform-resources)
    /// for more information.
    #[prost(string, repeated, tag = "13")]
    pub network_tags: ::std::vec::Vec<std::string::String>,
}
/// The result for a IAM Policy search.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IamPolicySearchResult {
    /// The [full resource
    /// name](https://cloud.google.com/apis/design/resource_names#full_resource_name)
    /// of the resource associated with this IAM policy.
    #[prost(string, tag = "1")]
    pub resource: std::string::String,
    /// The project that the associated GCP resource belongs to, in the form of
    /// `projects/{project_number}`. If an IAM policy is set on a resource (like VM
    /// instance, Cloud Storage bucket), the project field will indicate the
    /// project that contains the resource. If an IAM policy is set on a folder or
    /// orgnization, the project field will be empty.
    #[prost(string, tag = "3")]
    pub project: std::string::String,
    /// The IAM policy directly set on the given resource. Note that the original
    /// IAM policy can contain multiple bindings. This only contains the bindings
    /// that match the given query. For queries that don't contain a constrain on
    /// policies (e.g. an empty query), this contains all the bindings.
    #[prost(message, optional, tag = "4")]
    pub policy: ::std::option::Option<super::super::super::iam::v1::Policy>,
    /// Explanation about the IAM policy search result. It contains additional
    /// information to explain why the search result matches the query.
    #[prost(message, optional, tag = "5")]
    pub explanation: ::std::option::Option<iam_policy_search_result::Explanation>,
}
pub mod iam_policy_search_result {
    /// Explanation about the IAM policy search result.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Explanation {
        /// The map from roles to their included permission matching the permission
        /// query (e.g. containing `policy.role.permissions:`). A sample role string:
        /// "roles/compute.instanceAdmin". The roles can also be found in the
        /// returned `policy` bindings. Note that the map is populated only if
        /// requesting with a permission query.
        #[prost(map = "string, message", tag = "1")]
        pub matched_permissions:
            ::std::collections::HashMap<std::string::String, super::Permissions>,
    }
}
/// IAM permissions
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Permissions {
    /// A list of permissions. A sample permission string: "compute.disk.get".
    #[prost(string, repeated, tag = "1")]
    pub permissions: ::std::vec::Vec<std::string::String>,
}
/// Search all resources request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchAllResourcesRequest {
    /// Required. The relative name of an asset. The search is limited to the resources
    /// within the `scope`. The allowed value must be:
    /// * Organization number (such as "organizations/123")
    /// * Folder number(such as "folders/1234")
    /// * Project number (such as "projects/12345")
    /// * Project id (such as "projects/abc")
    #[prost(string, tag = "1")]
    pub scope: std::string::String,
    /// Optional. The query statement.
    #[prost(string, tag = "2")]
    pub query: std::string::String,
    /// Optional. A list of asset types that this request searches for. If empty, it will
    /// search all the supported asset types.
    #[prost(string, repeated, tag = "3")]
    pub asset_types: ::std::vec::Vec<std::string::String>,
    /// Optional. The page size for search result pagination. Page size is capped at 500 even
    /// if a larger value is given. If set to zero, server will pick an appropriate
    /// default. Returned results may be fewer than requested. When this happens,
    /// there could be more results as long as `next_page_token` is returned.
    #[prost(int32, tag = "4")]
    pub page_size: i32,
    /// Optional. If present, then retrieve the next batch of results from the preceding call
    /// to this method.  `page_token` must be the value of `next_page_token` from
    /// the previous response. The values of all other method parameters, must be
    /// identical to those in the previous call.
    #[prost(string, tag = "5")]
    pub page_token: std::string::String,
    /// Optional. A comma separated list of fields specifying the sorting order of the
    /// results. The default order is ascending. Add " desc" after the field name
    /// to indicate descending order. Redundant space characters are ignored. For
    /// example, "  foo ,  bar  desc  ".
    #[prost(string, tag = "10")]
    pub order_by: std::string::String,
}
/// Search all resources response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchAllResourcesResponse {
    /// A list of resource that match the search query.
    #[prost(message, repeated, tag = "1")]
    pub results: ::std::vec::Vec<StandardResourceMetadata>,
    /// If there are more results than those appearing in this response, then
    /// `next_page_token` is included.  To get the next set of results, call this
    /// method again using the value of `next_page_token` as `page_token`.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
}
/// Search all IAM policies request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchAllIamPoliciesRequest {
    /// Required. The relative name of an asset. The search is limited to the resources
    /// within the `scope`. The allowed value must be:
    /// * Organization number (such as "organizations/123")
    /// * Folder number(such as "folders/1234")
    /// * Project number (such as "projects/12345")
    /// * Project id (such as "projects/abc")
    #[prost(string, tag = "1")]
    pub scope: std::string::String,
    /// Optional. The query statement.
    /// Examples:
    /// * "policy:myuser@mydomain.com"
    /// * "policy:(myuser@mydomain.com viewer)"
    #[prost(string, tag = "2")]
    pub query: std::string::String,
    /// Optional. The page size for search result pagination. Page size is capped at 500 even
    /// if a larger value is given. If set to zero, server will pick an appropriate
    /// default. Returned results may be fewer than requested. When this happens,
    /// there could be more results as long as `next_page_token` is returned.
    #[prost(int32, tag = "3")]
    pub page_size: i32,
    /// Optional. If present, retrieve the next batch of results from the preceding call to
    /// this method. `page_token` must be the value of `next_page_token` from the
    /// previous response. The values of all other method parameters must be
    /// identical to those in the previous call.
    #[prost(string, tag = "4")]
    pub page_token: std::string::String,
}
/// Search all IAM policies response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchAllIamPoliciesResponse {
    /// A list of IamPolicy that match the search query. Related information such
    /// as the associated resource is returned along with the policy.
    #[prost(message, repeated, tag = "1")]
    pub results: ::std::vec::Vec<IamPolicySearchResult>,
    /// Set if there are more results than those appearing in this response; to get
    /// the next set of results, call this method again, using this value as the
    /// `page_token`.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
}
#[doc = r" Generated client implementations."]
pub mod asset_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Asset service definition."]
    pub struct AssetServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> AssetServiceClient<T>
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
        #[doc = " Searches all the resources under a given accessible CRM scope"]
        #[doc = " (project/folder/organization). This RPC gives callers"]
        #[doc = " especially admins the ability to search all the resources under a scope,"]
        #[doc = " even if they don't have .get permission of all the resources. Callers"]
        #[doc = " should have cloud.assets.SearchAllResources permission on the requested"]
        #[doc = " scope, otherwise it will be rejected."]
        pub async fn search_all_resources(
            &mut self,
            request: impl tonic::IntoRequest<super::SearchAllResourcesRequest>,
        ) -> Result<tonic::Response<super::SearchAllResourcesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.asset.v1p1beta1.AssetService/SearchAllResources",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Searches all the IAM policies under a given accessible CRM scope"]
        #[doc = " (project/folder/organization). This RPC gives callers"]
        #[doc = " especially admins the ability to search all the IAM policies under a scope,"]
        #[doc = " even if they don't have .getIamPolicy permission of all the IAM policies."]
        #[doc = " Callers should have cloud.assets.SearchAllIamPolicies permission on the"]
        #[doc = " requested scope, otherwise it will be rejected."]
        pub async fn search_all_iam_policies(
            &mut self,
            request: impl tonic::IntoRequest<super::SearchAllIamPoliciesRequest>,
        ) -> Result<tonic::Response<super::SearchAllIamPoliciesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.asset.v1p1beta1.AssetService/SearchAllIamPolicies",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for AssetServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for AssetServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "AssetServiceClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod asset_service_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with AssetServiceServer."]
    #[async_trait]
    pub trait AssetService: Send + Sync + 'static {
        #[doc = " Searches all the resources under a given accessible CRM scope"]
        #[doc = " (project/folder/organization). This RPC gives callers"]
        #[doc = " especially admins the ability to search all the resources under a scope,"]
        #[doc = " even if they don't have .get permission of all the resources. Callers"]
        #[doc = " should have cloud.assets.SearchAllResources permission on the requested"]
        #[doc = " scope, otherwise it will be rejected."]
        async fn search_all_resources(
            &self,
            request: tonic::Request<super::SearchAllResourcesRequest>,
        ) -> Result<tonic::Response<super::SearchAllResourcesResponse>, tonic::Status>;
        #[doc = " Searches all the IAM policies under a given accessible CRM scope"]
        #[doc = " (project/folder/organization). This RPC gives callers"]
        #[doc = " especially admins the ability to search all the IAM policies under a scope,"]
        #[doc = " even if they don't have .getIamPolicy permission of all the IAM policies."]
        #[doc = " Callers should have cloud.assets.SearchAllIamPolicies permission on the"]
        #[doc = " requested scope, otherwise it will be rejected."]
        async fn search_all_iam_policies(
            &self,
            request: tonic::Request<super::SearchAllIamPoliciesRequest>,
        ) -> Result<tonic::Response<super::SearchAllIamPoliciesResponse>, tonic::Status>;
    }
    #[doc = " Asset service definition."]
    #[derive(Debug)]
    pub struct AssetServiceServer<T: AssetService> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: AssetService> AssetServiceServer<T> {
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
    impl<T, B> Service<http::Request<B>> for AssetServiceServer<T>
    where
        T: AssetService,
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
                "/google.cloud.asset.v1p1beta1.AssetService/SearchAllResources" => {
                    #[allow(non_camel_case_types)]
                    struct SearchAllResourcesSvc<T: AssetService>(pub Arc<T>);
                    impl<T: AssetService>
                        tonic::server::UnaryService<super::SearchAllResourcesRequest>
                        for SearchAllResourcesSvc<T>
                    {
                        type Response = super::SearchAllResourcesResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SearchAllResourcesRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).search_all_resources(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = SearchAllResourcesSvc(inner);
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
                "/google.cloud.asset.v1p1beta1.AssetService/SearchAllIamPolicies" => {
                    #[allow(non_camel_case_types)]
                    struct SearchAllIamPoliciesSvc<T: AssetService>(pub Arc<T>);
                    impl<T: AssetService>
                        tonic::server::UnaryService<super::SearchAllIamPoliciesRequest>
                        for SearchAllIamPoliciesSvc<T>
                    {
                        type Response = super::SearchAllIamPoliciesResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SearchAllIamPoliciesRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).search_all_iam_policies(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = SearchAllIamPoliciesSvc(inner);
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
    impl<T: AssetService> Clone for AssetServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: AssetService> Clone for _Inner<T> {
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
