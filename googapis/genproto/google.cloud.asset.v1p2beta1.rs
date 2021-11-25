/// Temporal asset. In addition to the asset, the temporal asset includes the
/// status of the asset and valid from and to time of it.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TemporalAsset {
    /// The time window when the asset data and state was observed.
    #[prost(message, optional, tag = "1")]
    pub window: ::core::option::Option<TimeWindow>,
    /// If the asset is deleted or not.
    #[prost(bool, tag = "2")]
    pub deleted: bool,
    /// Asset.
    #[prost(message, optional, tag = "3")]
    pub asset: ::core::option::Option<Asset>,
}
/// A time window of (start_time, end_time].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TimeWindow {
    /// Start time of the time window (exclusive).
    #[prost(message, optional, tag = "1")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// End time of the time window (inclusive).
    /// Current timestamp if not specified.
    #[prost(message, optional, tag = "2")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Cloud asset. This includes all Google Cloud Platform resources,
/// Cloud IAM policies, and other non-GCP assets.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Asset {
    /// The full name of the asset. For example:
    /// `//compute.googleapis.com/projects/my_project_123/zones/zone1/instances/instance1`.
    /// See [Resource
    /// Names](<https://cloud.google.com/apis/design/resource_names#full_resource_name>)
    /// for more information.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Type of the asset. Example: "compute.googleapis.com/Disk".
    #[prost(string, tag = "2")]
    pub asset_type: ::prost::alloc::string::String,
    /// Representation of the resource.
    #[prost(message, optional, tag = "3")]
    pub resource: ::core::option::Option<Resource>,
    /// Representation of the actual Cloud IAM policy set on a cloud resource. For
    /// each resource, there must be at most one Cloud IAM policy set on it.
    #[prost(message, optional, tag = "4")]
    pub iam_policy: ::core::option::Option<super::super::super::iam::v1::Policy>,
    /// Asset's ancestry path in Cloud Resource Manager (CRM) hierarchy,
    /// represented as a list of relative resource names. Ancestry path starts with
    /// the closest CRM ancestor and ends at root. If the asset is a CRM
    /// project/folder/organization, this starts from the asset itself.
    ///
    /// Example: ["projects/123456789", "folders/5432", "organizations/1234"]
    #[prost(string, repeated, tag = "6")]
    pub ancestors: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Representation of a cloud resource.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Resource {
    /// The API version. Example: "v1".
    #[prost(string, tag = "1")]
    pub version: ::prost::alloc::string::String,
    /// The URL of the discovery document containing the resource's JSON schema.
    /// For example:
    /// `"<https://www.googleapis.com/discovery/v1/apis/compute/v1/rest"`.>
    /// It will be left unspecified for resources without a discovery-based API,
    /// such as Cloud Bigtable.
    #[prost(string, tag = "2")]
    pub discovery_document_uri: ::prost::alloc::string::String,
    /// The JSON schema name listed in the discovery document.
    /// Example: "Project". It will be left unspecified for resources (such as
    /// Cloud Bigtable) without a discovery-based API.
    #[prost(string, tag = "3")]
    pub discovery_name: ::prost::alloc::string::String,
    /// The REST URL for accessing the resource. An HTTP GET operation using this
    /// URL returns the resource itself.
    /// Example:
    /// `<https://cloudresourcemanager.googleapis.com/v1/projects/my-project-123`.>
    /// It will be left unspecified for resources without a REST API.
    #[prost(string, tag = "4")]
    pub resource_url: ::prost::alloc::string::String,
    /// The full name of the immediate parent of this resource. See
    /// [Resource
    /// Names](<https://cloud.google.com/apis/design/resource_names#full_resource_name>)
    /// for more information.
    ///
    /// For GCP assets, it is the parent resource defined in the [Cloud IAM policy
    /// hierarchy](<https://cloud.google.com/iam/docs/overview#policy_hierarchy>).
    /// For example:
    /// `"//cloudresourcemanager.googleapis.com/projects/my_project_123"`.
    ///
    /// For third-party assets, it is up to the users to define.
    #[prost(string, tag = "5")]
    pub parent: ::prost::alloc::string::String,
    /// The content of the resource, in which some sensitive fields are scrubbed
    /// away and may not be present.
    #[prost(message, optional, tag = "6")]
    pub data: ::core::option::Option<::prost_types::Struct>,
}
/// Create asset feed request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateFeedRequest {
    /// Required. The name of the project/folder/organization where this feed
    /// should be created in. It can only be an organization number (such as
    /// "organizations/123"), a folder number (such as "folders/123"), a project ID
    /// (such as "projects/my-project-id")", or a project number (such as
    /// "projects/12345").
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. This is the client-assigned asset feed identifier and it needs to
    /// be unique under a specific parent project/folder/organization.
    #[prost(string, tag = "2")]
    pub feed_id: ::prost::alloc::string::String,
    /// Required. The feed details. The field `name` must be empty and it will be generated
    /// in the format of:
    /// projects/project_number/feeds/feed_id
    /// folders/folder_number/feeds/feed_id
    /// organizations/organization_number/feeds/feed_id
    #[prost(message, optional, tag = "3")]
    pub feed: ::core::option::Option<Feed>,
}
/// Get asset feed request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetFeedRequest {
    /// Required. The name of the Feed and it must be in the format of:
    /// projects/project_number/feeds/feed_id
    /// folders/folder_number/feeds/feed_id
    /// organizations/organization_number/feeds/feed_id
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// List asset feeds request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListFeedsRequest {
    /// Required. The parent project/folder/organization whose feeds are to be
    /// listed. It can only be using project/folder/organization number (such as
    /// "folders/12345")", or a project ID (such as "projects/my-project-id").
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListFeedsResponse {
    /// A list of feeds.
    #[prost(message, repeated, tag = "1")]
    pub feeds: ::prost::alloc::vec::Vec<Feed>,
}
/// Update asset feed request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateFeedRequest {
    /// Required. The new values of feed details. It must match an existing feed and the
    /// field `name` must be in the format of:
    /// projects/project_number/feeds/feed_id or
    /// folders/folder_number/feeds/feed_id or
    /// organizations/organization_number/feeds/feed_id.
    #[prost(message, optional, tag = "1")]
    pub feed: ::core::option::Option<Feed>,
    /// Required. Only updates the `feed` fields indicated by this mask.
    /// The field mask must not be empty, and it must not contain fields that
    /// are immutable or only set by the server.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteFeedRequest {
    /// Required. The name of the feed and it must be in the format of:
    /// projects/project_number/feeds/feed_id
    /// folders/folder_number/feeds/feed_id
    /// organizations/organization_number/feeds/feed_id
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Output configuration for export assets destination.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OutputConfig {
    /// Asset export destination.
    #[prost(oneof = "output_config::Destination", tags = "1")]
    pub destination: ::core::option::Option<output_config::Destination>,
}
/// Nested message and enum types in `OutputConfig`.
pub mod output_config {
    /// Asset export destination.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Destination {
        /// Destination on Cloud Storage.
        #[prost(message, tag = "1")]
        GcsDestination(super::GcsDestination),
    }
}
/// A Cloud Storage location.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GcsDestination {
    /// Required.
    #[prost(oneof = "gcs_destination::ObjectUri", tags = "1")]
    pub object_uri: ::core::option::Option<gcs_destination::ObjectUri>,
}
/// Nested message and enum types in `GcsDestination`.
pub mod gcs_destination {
    /// Required.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ObjectUri {
        /// The uri of the Cloud Storage object. It's the same uri that is used by
        /// gsutil. For example: "gs://bucket_name/object_name". See [Viewing and
        /// Editing Object
        /// Metadata](<https://cloud.google.com/storage/docs/viewing-editing-metadata>)
        /// for more information.
        #[prost(string, tag = "1")]
        Uri(::prost::alloc::string::String),
    }
}
/// A Cloud Pubsub destination.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PubsubDestination {
    /// The name of the Cloud Pub/Sub topic to publish to.
    /// For example: `projects/PROJECT_ID/topics/TOPIC_ID`.
    #[prost(string, tag = "1")]
    pub topic: ::prost::alloc::string::String,
}
/// Output configuration for asset feed destination.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeedOutputConfig {
    /// Asset feed destination.
    #[prost(oneof = "feed_output_config::Destination", tags = "1")]
    pub destination: ::core::option::Option<feed_output_config::Destination>,
}
/// Nested message and enum types in `FeedOutputConfig`.
pub mod feed_output_config {
    /// Asset feed destination.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Destination {
        /// Destination on Cloud Pubsub.
        #[prost(message, tag = "1")]
        PubsubDestination(super::PubsubDestination),
    }
}
/// An asset feed used to export asset updates to a destinations.
/// An asset feed filter controls what updates are exported.
/// The asset feed must be created within a project, organization, or
/// folder. Supported destinations are:
/// Cloud Pub/Sub topics.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Feed {
    /// Required. The format will be
    /// projects/{project_number}/feeds/{client-assigned_feed_identifier} or
    /// folders/{folder_number}/feeds/{client-assigned_feed_identifier} or
    /// organizations/{organization_number}/feeds/{client-assigned_feed_identifier}
    ///
    /// The client-assigned feed identifier must be unique within the parent
    /// project/folder/organization.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// A list of the full names of the assets to receive updates. You must specify
    /// either or both of asset_names and asset_types. Only asset updates matching
    /// specified asset_names and asset_types are exported to the feed. For
    /// example:
    /// `//compute.googleapis.com/projects/my_project_123/zones/zone1/instances/instance1`.
    /// See [Resource
    /// Names](<https://cloud.google.com/apis/design/resource_names#full_resource_name>)
    /// for more info.
    #[prost(string, repeated, tag = "2")]
    pub asset_names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// A list of types of the assets to receive updates. You must specify either
    /// or both of asset_names and asset_types. Only asset updates matching
    /// specified asset_names and asset_types are exported to the feed.
    /// For example:
    /// "compute.googleapis.com/Disk" See [Introduction to Cloud Asset
    /// Inventory](<https://cloud.google.com/resource-manager/docs/cloud-asset-inventory/overview>)
    /// for all supported asset types.
    #[prost(string, repeated, tag = "3")]
    pub asset_types: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Asset content type. If not specified, no content but the asset name and
    /// type will be returned.
    #[prost(enumeration = "ContentType", tag = "4")]
    pub content_type: i32,
    /// Required. Feed output configuration defining where the asset updates are
    /// published to.
    #[prost(message, optional, tag = "5")]
    pub feed_output_config: ::core::option::Option<FeedOutputConfig>,
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
}
#[doc = r" Generated client implementations."]
pub mod asset_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Asset service definition."]
    #[derive(Debug, Clone)]
    pub struct AssetServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> AssetServiceClient<T>
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
        ) -> AssetServiceClient<InterceptedService<T, F>>
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
            AssetServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Creates a feed in a parent project/folder/organization to listen to its"]
        #[doc = " asset updates."]
        pub async fn create_feed(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateFeedRequest>,
        ) -> Result<tonic::Response<super::Feed>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.asset.v1p2beta1.AssetService/CreateFeed",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets details about an asset feed."]
        pub async fn get_feed(
            &mut self,
            request: impl tonic::IntoRequest<super::GetFeedRequest>,
        ) -> Result<tonic::Response<super::Feed>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.asset.v1p2beta1.AssetService/GetFeed",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists all asset feeds in a parent project/folder/organization."]
        pub async fn list_feeds(
            &mut self,
            request: impl tonic::IntoRequest<super::ListFeedsRequest>,
        ) -> Result<tonic::Response<super::ListFeedsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.asset.v1p2beta1.AssetService/ListFeeds",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates an asset feed configuration."]
        pub async fn update_feed(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateFeedRequest>,
        ) -> Result<tonic::Response<super::Feed>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.asset.v1p2beta1.AssetService/UpdateFeed",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes an asset feed."]
        pub async fn delete_feed(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteFeedRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.asset.v1p2beta1.AssetService/DeleteFeed",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
