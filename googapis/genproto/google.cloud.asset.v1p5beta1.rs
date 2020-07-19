/// Cloud asset. This includes all Google Cloud Platform resources,
/// Cloud IAM policies, and other non-GCP assets.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Asset {
    /// The full name of the asset. For example:
    /// `//compute.googleapis.com/projects/my_project_123/zones/zone1/instances/instance1`.
    /// See [Resource
    /// Names](https://cloud.google.com/apis/design/resource_names#full_resource_name)
    /// for more information.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Type of the asset. Example: "compute.googleapis.com/Disk".
    #[prost(string, tag = "2")]
    pub asset_type: std::string::String,
    /// Representation of the resource.
    #[prost(message, optional, tag = "3")]
    pub resource: ::std::option::Option<Resource>,
    /// Representation of the actual Cloud IAM policy set on a cloud resource. For
    /// each resource, there must be at most one Cloud IAM policy set on it.
    #[prost(message, optional, tag = "4")]
    pub iam_policy: ::std::option::Option<super::super::super::iam::v1::Policy>,
    /// Representation of the Cloud Organization Policy set on an asset. For each
    /// asset, there could be multiple Organization policies with different
    /// constraints.
    #[prost(message, repeated, tag = "6")]
    pub org_policy: ::std::vec::Vec<super::super::orgpolicy::v1::Policy>,
    /// Asset's ancestry path in Cloud Resource Manager (CRM) hierarchy,
    /// represented as a list of relative resource names. Ancestry path starts with
    /// the closest CRM ancestor and ends at root. If the asset is a CRM
    /// project/folder/organization, this starts from the asset itself.
    ///
    /// Example: ["projects/123456789", "folders/5432", "organizations/1234"]
    #[prost(string, repeated, tag = "10")]
    pub ancestors: ::std::vec::Vec<std::string::String>,
    /// Representation of the Cloud Organization access policy.
    #[prost(oneof = "asset::AccessContextPolicy", tags = "7, 8, 9")]
    pub access_context_policy: ::std::option::Option<asset::AccessContextPolicy>,
}
pub mod asset {
    /// Representation of the Cloud Organization access policy.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum AccessContextPolicy {
        #[prost(message, tag = "7")]
        AccessPolicy(super::super::super::super::identity::accesscontextmanager::v1::AccessPolicy),
        #[prost(message, tag = "8")]
        AccessLevel(super::super::super::super::identity::accesscontextmanager::v1::AccessLevel),
        #[prost(message, tag = "9")]
        ServicePerimeter(
            super::super::super::super::identity::accesscontextmanager::v1::ServicePerimeter,
        ),
    }
}
/// Representation of a cloud resource.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Resource {
    /// The API version. Example: "v1".
    #[prost(string, tag = "1")]
    pub version: std::string::String,
    /// The URL of the discovery document containing the resource's JSON schema.
    /// For example:
    /// `"https://www.googleapis.com/discovery/v1/apis/compute/v1/rest"`.
    /// It will be left unspecified for resources without a discovery-based API,
    /// such as Cloud Bigtable.
    #[prost(string, tag = "2")]
    pub discovery_document_uri: std::string::String,
    /// The JSON schema name listed in the discovery document.
    /// Example: "Project". It will be left unspecified for resources (such as
    /// Cloud Bigtable) without a discovery-based API.
    #[prost(string, tag = "3")]
    pub discovery_name: std::string::String,
    /// The REST URL for accessing the resource. An HTTP GET operation using this
    /// URL returns the resource itself.
    /// Example:
    /// `https://cloudresourcemanager.googleapis.com/v1/projects/my-project-123`.
    /// It will be left unspecified for resources without a REST API.
    #[prost(string, tag = "4")]
    pub resource_url: std::string::String,
    /// The full name of the immediate parent of this resource. See
    /// [Resource
    /// Names](https://cloud.google.com/apis/design/resource_names#full_resource_name)
    /// for more information.
    ///
    /// For GCP assets, it is the parent resource defined in the [Cloud IAM policy
    /// hierarchy](https://cloud.google.com/iam/docs/overview#policy_hierarchy).
    /// For example:
    /// `"//cloudresourcemanager.googleapis.com/projects/my_project_123"`.
    ///
    /// For third-party assets, it is up to the users to define.
    #[prost(string, tag = "5")]
    pub parent: std::string::String,
    /// The content of the resource, in which some sensitive fields are scrubbed
    /// away and may not be present.
    #[prost(message, optional, tag = "6")]
    pub data: ::std::option::Option<::prost_types::Struct>,
}
/// ListAssets request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAssetsRequest {
    /// Required. Name of the organization or project the assets belong to. Format:
    /// "organizations/[organization-number]" (such as "organizations/123"),
    /// "projects/[project-number]" (such as "projects/my-project-id"), or
    /// "projects/[project-id]" (such as "projects/12345").
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Timestamp to take an asset snapshot. This can only be set to a timestamp
    /// between 2018-10-02 UTC (inclusive) and the current time. If not specified,
    /// the current time will be used. Due to delays in resource data collection
    /// and indexing, there is a volatile window during which running the same
    /// query may get different results.
    #[prost(message, optional, tag = "2")]
    pub read_time: ::std::option::Option<::prost_types::Timestamp>,
    /// A list of asset types of which to take a snapshot for. For  example:
    /// "compute.googleapis.com/Disk". If specified, only matching assets will be
    /// returned. See [Introduction to Cloud Asset
    /// Inventory](https://cloud.google.com/resource-manager/docs/cloud-asset-inventory/overview)
    /// for all supported asset types.
    #[prost(string, repeated, tag = "3")]
    pub asset_types: ::std::vec::Vec<std::string::String>,
    /// Asset content type. If not specified, no content but the asset name will
    /// be returned.
    #[prost(enumeration = "ContentType", tag = "4")]
    pub content_type: i32,
    /// The maximum number of assets to be returned in a single response. Default
    /// is 100, minimum is 1, and maximum is 1000.
    #[prost(int32, tag = "5")]
    pub page_size: i32,
    /// The `next_page_token` returned from the previous `ListAssetsResponse`, or
    /// unspecified for the first `ListAssetsRequest`. It is a continuation of a
    /// prior `ListAssets` call, and the API should return the next page of assets.
    #[prost(string, tag = "6")]
    pub page_token: std::string::String,
}
/// ListAssets response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAssetsResponse {
    /// Time the snapshot was taken.
    #[prost(message, optional, tag = "1")]
    pub read_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Assets.
    #[prost(message, repeated, tag = "2")]
    pub assets: ::std::vec::Vec<Asset>,
    /// Token to retrieve the next page of results. Set to empty if there are no
    /// remaining results.
    #[prost(string, tag = "3")]
    pub next_page_token: std::string::String,
}
/// Asset content type.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ContentType {
    /// Unspecified content type.
    Unspecified = 0,
    /// Resource metadata.
    Resource = 1,
    /// The actual IAM policy set on a resource.
    IamPolicy = 2,
    /// The Cloud Organization Policy set on an asset.
    OrgPolicy = 4,
    /// The Cloud Access context mananger Policy set on an asset.
    AccessPolicy = 5,
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
        #[doc = " Lists assets with time and resource types and returns paged results in"]
        #[doc = " response."]
        pub async fn list_assets(
            &mut self,
            request: impl tonic::IntoRequest<super::ListAssetsRequest>,
        ) -> Result<tonic::Response<super::ListAssetsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.asset.v1p5beta1.AssetService/ListAssets",
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
        #[doc = " Lists assets with time and resource types and returns paged results in"]
        #[doc = " response."]
        async fn list_assets(
            &self,
            request: tonic::Request<super::ListAssetsRequest>,
        ) -> Result<tonic::Response<super::ListAssetsResponse>, tonic::Status>;
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
                "/google.cloud.asset.v1p5beta1.AssetService/ListAssets" => {
                    #[allow(non_camel_case_types)]
                    struct ListAssetsSvc<T: AssetService>(pub Arc<T>);
                    impl<T: AssetService> tonic::server::UnaryService<super::ListAssetsRequest> for ListAssetsSvc<T> {
                        type Response = super::ListAssetsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListAssetsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).list_assets(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ListAssetsSvc(inner);
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
