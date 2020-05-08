/// A recommendation along with a suggested action. E.g., a rightsizing
/// recommendation for an underutilized VM, IAM role recommendations, etc
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Recommendation {
    /// Name of recommendation.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Free-form human readable summary in English. The maximum length is 500
    /// characters.
    #[prost(string, tag = "2")]
    pub description: std::string::String,
    /// Contains an identifier for a subtype of recommendations produced for the
    /// same recommender. Subtype is a function of content and impact, meaning a
    /// new subtype might be added when significant changes to `content` or
    /// `primary_impact.category` are introduced. See the Recommenders section
    /// to see a list of subtypes for a given Recommender.
    ///
    /// Examples:
    ///   For recommender = "google.iam.policy.Recommender",
    ///   recommender_subtype can be one of "REMOVE_ROLE"/"REPLACE_ROLE"
    #[prost(string, tag = "12")]
    pub recommender_subtype: std::string::String,
    /// Last time this recommendation was refreshed by the system that created it
    /// in the first place.
    #[prost(message, optional, tag = "4")]
    pub last_refresh_time: ::std::option::Option<::prost_types::Timestamp>,
    /// The primary impact that this recommendation can have while trying to
    /// optimize for one category.
    #[prost(message, optional, tag = "5")]
    pub primary_impact: ::std::option::Option<Impact>,
    /// Optional set of additional impact that this recommendation may have when
    /// trying to optimize for the primary category. These may be positive
    /// or negative.
    #[prost(message, repeated, tag = "6")]
    pub additional_impact: ::std::vec::Vec<Impact>,
    /// Content of the recommendation describing recommended changes to resources.
    #[prost(message, optional, tag = "7")]
    pub content: ::std::option::Option<RecommendationContent>,
    /// Information for state. Contains state and metadata.
    #[prost(message, optional, tag = "10")]
    pub state_info: ::std::option::Option<RecommendationStateInfo>,
    /// Fingerprint of the Recommendation. Provides optimistic locking when
    /// updating states.
    #[prost(string, tag = "11")]
    pub etag: std::string::String,
}
/// Contains what resources are changing and how they are changing.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RecommendationContent {
    /// Operations to one or more Google Cloud resources grouped in such a way
    /// that, all operations within one group are expected to be performed
    /// atomically and in an order.
    #[prost(message, repeated, tag = "2")]
    pub operation_groups: ::std::vec::Vec<OperationGroup>,
}
/// Group of operations that need to be performed atomically.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationGroup {
    /// List of operations across one or more resources that belong to this group.
    /// Loosely based on RFC6902 and should be performed in the order they appear.
    #[prost(message, repeated, tag = "1")]
    pub operations: ::std::vec::Vec<Operation>,
}
/// Contains an operation for a resource loosely based on the JSON-PATCH format
/// with support for:
///
/// * Custom filters for describing partial array patch.
/// * Extended path values for describing nested arrays.
/// * Custom fields for describing the resource for which the operation is being
///   described.
/// * Allows extension to custom operations not natively supported by RFC6902.
/// See https://tools.ietf.org/html/rfc6902 for details on the original RFC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Operation {
    /// Type of this operation. Contains one of 'and', 'remove', 'replace', 'move',
    /// 'copy', 'test' and custom operations. This field is case-insensitive and
    /// always populated.
    #[prost(string, tag = "1")]
    pub action: std::string::String,
    /// Type of GCP resource being modified/tested. This field is always populated.
    /// Example: cloudresourcemanager.googleapis.com/Project,
    /// compute.googleapis.com/Instance
    #[prost(string, tag = "2")]
    pub resource_type: std::string::String,
    /// Contains the fully qualified resource name. This field is always populated.
    /// ex: //cloudresourcemanager.googleapis.com/projects/foo.
    #[prost(string, tag = "3")]
    pub resource: std::string::String,
    /// Path to the target field being operated on. If the operation is at the
    /// resource level, then path should be "/". This field is always populated.
    #[prost(string, tag = "4")]
    pub path: std::string::String,
    /// Can be set with action 'copy' to copy resource configuration across
    /// different resources of the same type. Example: A resource clone can be
    /// done via action = 'copy', path = "/", from = "/",
    /// source_resource = <source> and resource_name = <target>.
    /// This field is empty for all other values of `action`.
    #[prost(string, tag = "5")]
    pub source_resource: std::string::String,
    /// Can be set with action 'copy' or 'move' to indicate the source field within
    /// resource or source_resource, ignored if provided for other operation types.
    #[prost(string, tag = "6")]
    pub source_path: std::string::String,
    /// Set of filters to apply if `path` refers to array elements or nested array
    /// elements in order to narrow down to a single unique element that is being
    /// tested/modified.
    /// This is intended to be an exact match per filter. To perform advanced
    /// matching, use path_value_matchers.
    ///
    /// * Example: {
    ///   "/versions/*/name" : "it-123"
    ///   "/versions/*/targetSize/percent": 20
    ///  }
    /// * Example: {
    ///   "/bindings/*/role": "roles/admin"
    ///   "/bindings/*/condition" : null
    ///  }
    /// * Example: {
    ///   "/bindings/*/role": "roles/admin"
    ///   "/bindings/*/members/*" : ["x@google.com", "y@google.com"]
    ///  }
    /// When both path_filters and path_value_matchers are set, an implicit AND
    /// must be performed.
    #[prost(map = "string, message", tag = "8")]
    pub path_filters: ::std::collections::HashMap<std::string::String, ::prost_types::Value>,
    /// Similar to path_filters, this contains set of filters to apply if `path`
    /// field referes to array elements. This is meant to support value matching
    /// beyond exact match. To perform exact match, use path_filters.
    /// When both path_filters and path_value_matchers are set, an implicit AND
    /// must be performed.
    #[prost(map = "string, message", tag = "11")]
    pub path_value_matchers: ::std::collections::HashMap<std::string::String, ValueMatcher>,
    /// One of the fields in the following block will be set and intend to
    /// describe a value for 'path' field.
    #[prost(oneof = "operation::PathValue", tags = "7, 10")]
    pub path_value: ::std::option::Option<operation::PathValue>,
}
pub mod operation {
    /// One of the fields in the following block will be set and intend to
    /// describe a value for 'path' field.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum PathValue {
        /// Value for the `path` field. Will be set for actions:'add'/'replace'.
        /// Maybe set for action: 'test'. Either this or `value_matcher` will be set
        /// for 'test' operation. An exact match must be performed.
        #[prost(message, tag = "7")]
        Value(::prost_types::Value),
        /// Can be set for action 'test' for advanced matching for the value of
        /// 'path' field. Either this or `value` will be set for 'test' operation.
        #[prost(message, tag = "10")]
        ValueMatcher(super::ValueMatcher),
    }
}
/// Contains various matching options for values for a GCP resource field.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValueMatcher {
    #[prost(oneof = "value_matcher::MatchVariant", tags = "1")]
    pub match_variant: ::std::option::Option<value_matcher::MatchVariant>,
}
pub mod value_matcher {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum MatchVariant {
        /// To be used for full regex matching. The regular expression is using the
        /// Google RE2 syntax (https://github.com/google/re2/wiki/Syntax), so to be
        /// used with RE2::FullMatch
        #[prost(string, tag = "1")]
        MatchesPattern(std::string::String),
    }
}
/// Contains metadata about how much money a recommendation can save or incur.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CostProjection {
    /// An approximate projection on amount saved or amount incurred. Negative cost
    /// units indicate cost savings and positive cost units indicate increase.
    /// See google.type.Money documentation for positive/negative units.
    #[prost(message, optional, tag = "1")]
    pub cost: ::std::option::Option<super::super::super::r#type::Money>,
    /// Duration for which this cost applies.
    #[prost(message, optional, tag = "2")]
    pub duration: ::std::option::Option<::prost_types::Duration>,
}
/// Contains the impact a recommendation can have for a given category.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Impact {
    /// Category that is being targeted.
    #[prost(enumeration = "impact::Category", tag = "1")]
    pub category: i32,
    /// Contains projections (if any) for this category.
    #[prost(oneof = "impact::Projection", tags = "100")]
    pub projection: ::std::option::Option<impact::Projection>,
}
pub mod impact {
    /// The category of the impact.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Category {
        /// Default unspecified category. Don't use directly.
        Unspecified = 0,
        /// Indicates a potential increase or decrease in cost.
        Cost = 1,
        /// Indicates a potential increase or decrease in security.
        Security = 2,
        /// Indicates a potential increase or decrease in performance.
        Performance = 3,
        /// Indicates a potential increase or decrease in manageability.
        Manageability = 4,
    }
    /// Contains projections (if any) for this category.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Projection {
        /// Use with CategoryType.COST
        #[prost(message, tag = "100")]
        CostProjection(super::CostProjection),
    }
}
/// Information for state. Contains state and metadata.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RecommendationStateInfo {
    /// The state of the recommendation, Eg ACTIVE, SUCCEEDED, FAILED.
    #[prost(enumeration = "recommendation_state_info::State", tag = "1")]
    pub state: i32,
    /// A map of metadata for the state, provided by user or automations systems.
    #[prost(map = "string, string", tag = "2")]
    pub state_metadata: ::std::collections::HashMap<std::string::String, std::string::String>,
}
pub mod recommendation_state_info {
    /// Represents Recommendation State
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// Default state. Don't use directly.
        Unspecified = 0,
        /// Recommendation is active and can be applied. Recommendations content can
        /// be updated by Google.
        ///
        /// ACTIVE recommendations can be marked as CLAIMED, SUCCEEDED, or FAILED.
        Active = 1,
        /// Recommendation is in claimed state. Recommendations content is
        /// immutable and cannot be updated by Google.
        ///
        /// CLAIMED recommendations can be marked as CLAIMED, SUCCEEDED, or FAILED.
        Claimed = 6,
        /// Recommendation is in succeeded state. Recommendations content is
        /// immutable and cannot be updated by Google.
        ///
        /// SUCCEEDED recommendations can be marked as SUCCEEDED, or FAILED.
        Succeeded = 3,
        /// Recommendation is in failed state. Recommendations content is immutable
        /// and cannot be updated by Google.
        ///
        /// FAILED recommendations can be marked as SUCCEEDED, or FAILED.
        Failed = 4,
        /// Recommendation is in dismissed state. Recommendation content can be
        /// updated by Google.
        ///
        /// DISMISSED recommendations can be marked as ACTIVE.
        Dismissed = 5,
    }
}
/// Request for the `ListRecommendations` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListRecommendationsRequest {
    /// Required. The container resource on which to execute the request.
    /// Acceptable formats:
    ///
    /// 1.
    /// "projects/[PROJECT_NUMBER]/locations/[LOCATION]/recommenders/[RECOMMENDER_ID]",
    ///
    /// LOCATION here refers to GCP Locations:
    /// https://cloud.google.com/about/locations/
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Optional. The maximum number of results to return from this request.  Non-positive
    /// values are ignored. If not specified, the server will determine the number
    /// of results to return.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. If present, retrieves the next batch of results from the preceding call to
    /// this method. `page_token` must be the value of `next_page_token` from the
    /// previous response. The values of other method parameters must be identical
    /// to those in the previous call.
    #[prost(string, tag = "3")]
    pub page_token: std::string::String,
    /// Filter expression to restrict the recommendations returned. Supported
    /// filter fields: state_info.state
    /// Eg: `state_info.state:"DISMISSED" or state_info.state:"FAILED"
    #[prost(string, tag = "5")]
    pub filter: std::string::String,
}
/// Response to the `ListRecommendations` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListRecommendationsResponse {
    /// The set of recommendations for the `parent` resource.
    #[prost(message, repeated, tag = "1")]
    pub recommendations: ::std::vec::Vec<Recommendation>,
    /// A token that can be used to request the next page of results. This field is
    /// empty if there are no additional results.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
}
/// Request to the `GetRecommendation` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRecommendationRequest {
    /// Required. Name of the recommendation.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Request for the `MarkRecommendationClaimed` Method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarkRecommendationClaimedRequest {
    /// Required. Name of the recommendation.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// State properties to include with this state. Overwrites any existing
    /// `state_metadata`.
    /// Keys must match the regex /^[a-z0-9][a-z0-9_.-]{0,62}$/.
    /// Values must match the regex /^[a-zA-Z0-9_./-]{0,255}$/.
    #[prost(map = "string, string", tag = "2")]
    pub state_metadata: ::std::collections::HashMap<std::string::String, std::string::String>,
    /// Required. Fingerprint of the Recommendation. Provides optimistic locking.
    #[prost(string, tag = "3")]
    pub etag: std::string::String,
}
/// Request for the `MarkRecommendationSucceeded` Method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarkRecommendationSucceededRequest {
    /// Required. Name of the recommendation.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// State properties to include with this state. Overwrites any existing
    /// `state_metadata`.
    /// Keys must match the regex /^[a-z0-9][a-z0-9_.-]{0,62}$/.
    /// Values must match the regex /^[a-zA-Z0-9_./-]{0,255}$/.
    #[prost(map = "string, string", tag = "2")]
    pub state_metadata: ::std::collections::HashMap<std::string::String, std::string::String>,
    /// Required. Fingerprint of the Recommendation. Provides optimistic locking.
    #[prost(string, tag = "3")]
    pub etag: std::string::String,
}
/// Request for the `MarkRecommendationFailed` Method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarkRecommendationFailedRequest {
    /// Required. Name of the recommendation.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// State properties to include with this state. Overwrites any existing
    /// `state_metadata`.
    /// Keys must match the regex /^[a-z0-9][a-z0-9_.-]{0,62}$/.
    /// Values must match the regex /^[a-zA-Z0-9_./-]{0,255}$/.
    #[prost(map = "string, string", tag = "2")]
    pub state_metadata: ::std::collections::HashMap<std::string::String, std::string::String>,
    /// Required. Fingerprint of the Recommendation. Provides optimistic locking.
    #[prost(string, tag = "3")]
    pub etag: std::string::String,
}
#[doc = r" Generated client implementations."]
pub mod recommender_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Provides recommendations for cloud customers for various categories like"]
    #[doc = " performance optimization, cost savings, reliability, feature discovery, etc."]
    #[doc = " These recommendations are generated automatically based on analysis of user"]
    #[doc = " resources, configuration and monitoring metrics."]
    pub struct RecommenderClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl RecommenderClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> RecommenderClient<T>
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
        #[doc = " Lists recommendations for a Cloud project. Requires the recommender.*.list"]
        #[doc = " IAM permission for the specified recommender."]
        pub async fn list_recommendations(
            &mut self,
            request: impl tonic::IntoRequest<super::ListRecommendationsRequest>,
        ) -> Result<tonic::Response<super::ListRecommendationsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.recommender.v1.Recommender/ListRecommendations",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets the requested recommendation. Requires the recommender.*.get"]
        #[doc = " IAM permission for the specified recommender."]
        pub async fn get_recommendation(
            &mut self,
            request: impl tonic::IntoRequest<super::GetRecommendationRequest>,
        ) -> Result<tonic::Response<super::Recommendation>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.recommender.v1.Recommender/GetRecommendation",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Mark the Recommendation State as Claimed. Users can use this method to"]
        #[doc = " indicate to the Recommender API that they are starting to apply the"]
        #[doc = " recommendation themselves. This stops the recommendation content from being"]
        #[doc = " updated."]
        #[doc = ""]
        #[doc = " MarkRecommendationClaimed can be applied to recommendations in CLAIMED,"]
        #[doc = " SUCCEEDED, FAILED, or ACTIVE state."]
        #[doc = ""]
        #[doc = " Requires the recommender.*.update IAM permission for the specified"]
        #[doc = " recommender."]
        pub async fn mark_recommendation_claimed(
            &mut self,
            request: impl tonic::IntoRequest<super::MarkRecommendationClaimedRequest>,
        ) -> Result<tonic::Response<super::Recommendation>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.recommender.v1.Recommender/MarkRecommendationClaimed",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Mark the Recommendation State as Succeeded. Users can use this method to"]
        #[doc = " indicate to the Recommender API that they have applied the recommendation"]
        #[doc = " themselves, and the operation was successful. This stops the recommendation"]
        #[doc = " content from being updated."]
        #[doc = ""]
        #[doc = " MarkRecommendationSucceeded can be applied to recommendations in ACTIVE,"]
        #[doc = " CLAIMED, SUCCEEDED, or FAILED state."]
        #[doc = ""]
        #[doc = " Requires the recommender.*.update IAM permission for the specified"]
        #[doc = " recommender."]
        pub async fn mark_recommendation_succeeded(
            &mut self,
            request: impl tonic::IntoRequest<super::MarkRecommendationSucceededRequest>,
        ) -> Result<tonic::Response<super::Recommendation>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.recommender.v1.Recommender/MarkRecommendationSucceeded",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Mark the Recommendation State as Failed. Users can use this method to"]
        #[doc = " indicate to the Recommender API that they have applied the recommendation"]
        #[doc = " themselves, and the operation failed. This stops the recommendation content"]
        #[doc = " from being updated."]
        #[doc = ""]
        #[doc = " MarkRecommendationFailed can be applied to recommendations in ACTIVE,"]
        #[doc = " CLAIMED, SUCCEEDED, or FAILED state."]
        #[doc = ""]
        #[doc = " Requires the recommender.*.update IAM permission for the specified"]
        #[doc = " recommender."]
        pub async fn mark_recommendation_failed(
            &mut self,
            request: impl tonic::IntoRequest<super::MarkRecommendationFailedRequest>,
        ) -> Result<tonic::Response<super::Recommendation>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.recommender.v1.Recommender/MarkRecommendationFailed",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for RecommenderClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for RecommenderClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "RecommenderClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod recommender_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with RecommenderServer."]
    #[async_trait]
    pub trait Recommender: Send + Sync + 'static {
        #[doc = " Lists recommendations for a Cloud project. Requires the recommender.*.list"]
        #[doc = " IAM permission for the specified recommender."]
        async fn list_recommendations(
            &self,
            request: tonic::Request<super::ListRecommendationsRequest>,
        ) -> Result<tonic::Response<super::ListRecommendationsResponse>, tonic::Status>;
        #[doc = " Gets the requested recommendation. Requires the recommender.*.get"]
        #[doc = " IAM permission for the specified recommender."]
        async fn get_recommendation(
            &self,
            request: tonic::Request<super::GetRecommendationRequest>,
        ) -> Result<tonic::Response<super::Recommendation>, tonic::Status>;
        #[doc = " Mark the Recommendation State as Claimed. Users can use this method to"]
        #[doc = " indicate to the Recommender API that they are starting to apply the"]
        #[doc = " recommendation themselves. This stops the recommendation content from being"]
        #[doc = " updated."]
        #[doc = ""]
        #[doc = " MarkRecommendationClaimed can be applied to recommendations in CLAIMED,"]
        #[doc = " SUCCEEDED, FAILED, or ACTIVE state."]
        #[doc = ""]
        #[doc = " Requires the recommender.*.update IAM permission for the specified"]
        #[doc = " recommender."]
        async fn mark_recommendation_claimed(
            &self,
            request: tonic::Request<super::MarkRecommendationClaimedRequest>,
        ) -> Result<tonic::Response<super::Recommendation>, tonic::Status>;
        #[doc = " Mark the Recommendation State as Succeeded. Users can use this method to"]
        #[doc = " indicate to the Recommender API that they have applied the recommendation"]
        #[doc = " themselves, and the operation was successful. This stops the recommendation"]
        #[doc = " content from being updated."]
        #[doc = ""]
        #[doc = " MarkRecommendationSucceeded can be applied to recommendations in ACTIVE,"]
        #[doc = " CLAIMED, SUCCEEDED, or FAILED state."]
        #[doc = ""]
        #[doc = " Requires the recommender.*.update IAM permission for the specified"]
        #[doc = " recommender."]
        async fn mark_recommendation_succeeded(
            &self,
            request: tonic::Request<super::MarkRecommendationSucceededRequest>,
        ) -> Result<tonic::Response<super::Recommendation>, tonic::Status>;
        #[doc = " Mark the Recommendation State as Failed. Users can use this method to"]
        #[doc = " indicate to the Recommender API that they have applied the recommendation"]
        #[doc = " themselves, and the operation failed. This stops the recommendation content"]
        #[doc = " from being updated."]
        #[doc = ""]
        #[doc = " MarkRecommendationFailed can be applied to recommendations in ACTIVE,"]
        #[doc = " CLAIMED, SUCCEEDED, or FAILED state."]
        #[doc = ""]
        #[doc = " Requires the recommender.*.update IAM permission for the specified"]
        #[doc = " recommender."]
        async fn mark_recommendation_failed(
            &self,
            request: tonic::Request<super::MarkRecommendationFailedRequest>,
        ) -> Result<tonic::Response<super::Recommendation>, tonic::Status>;
    }
    #[doc = " Provides recommendations for cloud customers for various categories like"]
    #[doc = " performance optimization, cost savings, reliability, feature discovery, etc."]
    #[doc = " These recommendations are generated automatically based on analysis of user"]
    #[doc = " resources, configuration and monitoring metrics."]
    #[derive(Debug)]
    #[doc(hidden)]
    pub struct RecommenderServer<T: Recommender> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: Recommender> RecommenderServer<T> {
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
    impl<T, B> Service<http::Request<B>> for RecommenderServer<T>
    where
        T: Recommender,
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
                "/google.cloud.recommender.v1.Recommender/ListRecommendations" => {
                    #[allow(non_camel_case_types)]
                    struct ListRecommendationsSvc<T: Recommender>(pub Arc<T>);
                    impl<T: Recommender>
                        tonic::server::UnaryService<super::ListRecommendationsRequest>
                        for ListRecommendationsSvc<T>
                    {
                        type Response = super::ListRecommendationsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListRecommendationsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.list_recommendations(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ListRecommendationsSvc(inner);
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
                "/google.cloud.recommender.v1.Recommender/GetRecommendation" => {
                    #[allow(non_camel_case_types)]
                    struct GetRecommendationSvc<T: Recommender>(pub Arc<T>);
                    impl<T: Recommender>
                        tonic::server::UnaryService<super::GetRecommendationRequest>
                        for GetRecommendationSvc<T>
                    {
                        type Response = super::Recommendation;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetRecommendationRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.get_recommendation(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetRecommendationSvc(inner);
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
                "/google.cloud.recommender.v1.Recommender/MarkRecommendationClaimed" => {
                    #[allow(non_camel_case_types)]
                    struct MarkRecommendationClaimedSvc<T: Recommender>(pub Arc<T>);
                    impl<T: Recommender>
                        tonic::server::UnaryService<super::MarkRecommendationClaimedRequest>
                        for MarkRecommendationClaimedSvc<T>
                    {
                        type Response = super::Recommendation;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MarkRecommendationClaimedRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { inner.mark_recommendation_claimed(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = MarkRecommendationClaimedSvc(inner);
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
                "/google.cloud.recommender.v1.Recommender/MarkRecommendationSucceeded" => {
                    #[allow(non_camel_case_types)]
                    struct MarkRecommendationSucceededSvc<T: Recommender>(pub Arc<T>);
                    impl<T: Recommender>
                        tonic::server::UnaryService<super::MarkRecommendationSucceededRequest>
                        for MarkRecommendationSucceededSvc<T>
                    {
                        type Response = super::Recommendation;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MarkRecommendationSucceededRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { inner.mark_recommendation_succeeded(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = MarkRecommendationSucceededSvc(inner);
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
                "/google.cloud.recommender.v1.Recommender/MarkRecommendationFailed" => {
                    #[allow(non_camel_case_types)]
                    struct MarkRecommendationFailedSvc<T: Recommender>(pub Arc<T>);
                    impl<T: Recommender>
                        tonic::server::UnaryService<super::MarkRecommendationFailedRequest>
                        for MarkRecommendationFailedSvc<T>
                    {
                        type Response = super::Recommendation;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MarkRecommendationFailedRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { inner.mark_recommendation_failed(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = MarkRecommendationFailedSvc(inner);
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
    impl<T: Recommender> Clone for RecommenderServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: Recommender> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Recommender> tonic::transport::NamedService for RecommenderServer<T> {
        const NAME: &'static str = "google.cloud.recommender.v1.Recommender";
    }
}
