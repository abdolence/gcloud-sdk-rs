/// The values associated with a key of an attribute.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AttributeValues {
    /// The list of values associated with a key.
    #[prost(bytes, repeated, tag = "1")]
    pub values: ::std::vec::Vec<std::vec::Vec<u8>>,
}
/// A message that is published by publishers and delivered to subscribers.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PubSubMessage {
    /// The key used for routing messages to partitions or for compaction (e.g.,
    /// keep the last N messages per key). If the key is empty, the message is
    /// routed to an arbitrary partition.
    #[prost(bytes, tag = "1")]
    pub key: std::vec::Vec<u8>,
    /// The payload of the message.
    #[prost(bytes, tag = "2")]
    pub data: std::vec::Vec<u8>,
    /// Optional attributes that can be used for message metadata/headers.
    #[prost(map = "string, message", tag = "3")]
    pub attributes: ::std::collections::HashMap<std::string::String, AttributeValues>,
    /// An optional, user-specified event time.
    #[prost(message, optional, tag = "4")]
    pub event_time: ::std::option::Option<::prost_types::Timestamp>,
}
/// A cursor that describes the position of a message within a topic partition.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Cursor {
    /// The offset of a message within a topic partition. Must be greater than or
    /// equal 0.
    #[prost(int64, tag = "1")]
    pub offset: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SequencedMessage {
    /// The position of a message within the partition where it is stored.
    #[prost(message, optional, tag = "1")]
    pub cursor: ::std::option::Option<Cursor>,
    /// The time when the message was received by the server when it was first
    /// published.
    #[prost(message, optional, tag = "2")]
    pub publish_time: ::std::option::Option<::prost_types::Timestamp>,
    /// The user message.
    #[prost(message, optional, tag = "3")]
    pub message: ::std::option::Option<PubSubMessage>,
    /// The size in bytes of this message for flow control and quota purposes.
    #[prost(int64, tag = "4")]
    pub size_bytes: i64,
}
/// Metadata about a topic resource.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Topic {
    /// The name of the topic.
    /// Structured like:
    /// projects/{project_number}/locations/{location}/topics/{topic_id}
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// The settings for this topic's partitions.
    #[prost(message, optional, tag = "2")]
    pub partition_config: ::std::option::Option<topic::PartitionConfig>,
    /// The settings for this topic's message retention.
    #[prost(message, optional, tag = "3")]
    pub retention_config: ::std::option::Option<topic::RetentionConfig>,
}
pub mod topic {
    /// The settings for a topic's partitions.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PartitionConfig {
        /// The number of partitions in the topic. Must be at least 1.
        #[prost(int64, tag = "1")]
        pub count: i64,
        /// Every partition in the topic is allocated throughput equivalent to
        /// `scale` times the standard partition throughput (4 MiB/s). This is also
        /// reflected in the cost of this topic; a topic with `scale` of 2 and count
        /// of 10 is charged for 20 partitions. This value must be in the range
        /// [1,4].
        #[prost(int32, tag = "2")]
        pub scale: i32,
    }
    /// The settings for a topic's message retention.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct RetentionConfig {
        /// The provisioned storage, in bytes, per partition. If the number of bytes
        /// stored in any of the topic's partitions grows beyond this value, older
        /// messages will be dropped to make room for newer ones, regardless of the
        /// value of `period`.
        #[prost(int64, tag = "1")]
        pub per_partition_bytes: i64,
        /// How long a published message is retained. If unset, messages will be
        /// retained as long as the bytes retained for each partition is below
        /// `per_partition_bytes`.
        #[prost(message, optional, tag = "2")]
        pub period: ::std::option::Option<::prost_types::Duration>,
    }
}
/// Metadata about a subscription resource.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Subscription {
    /// The name of the subscription.
    /// Structured like:
    /// projects/{project_number}/locations/{location}/subscriptions/{subscription_id}
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// The name of the topic this subscription is attached to.
    /// Structured like:
    /// projects/{project_number}/locations/{location}/topics/{topic_id}
    #[prost(string, tag = "2")]
    pub topic: std::string::String,
    /// The settings for this subscription's message delivery.
    #[prost(message, optional, tag = "3")]
    pub delivery_config: ::std::option::Option<subscription::DeliveryConfig>,
}
pub mod subscription {
    /// The settings for a subscription's message delivery.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DeliveryConfig {
        /// The DeliveryRequirement for this subscription.
        #[prost(enumeration = "delivery_config::DeliveryRequirement", tag = "3")]
        pub delivery_requirement: i32,
    }
    pub mod delivery_config {
        /// When this subscription should send messages to subscribers relative to
        /// messages persistence in storage.
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
        #[repr(i32)]
        pub enum DeliveryRequirement {
            /// Default value. This value is unused.
            Unspecified = 0,
            /// The server does not wait for a published message to be successfully
            /// written to storage before delivering it to subscribers. As such, a
            /// subscriber may receive a message for which the write to storage failed.
            /// If the subscriber re-reads the offset of that message later on (e.g.,
            /// after a `Seek` operation), there may be a gap at that offset. Even if
            /// not re-reading messages, the delivery of messages for which the write
            /// to storage fails may be inconsistent across subscriptions, with some
            /// receiving the message (e.g., those connected at the time the message is
            /// published) and others not receiving it (e.g., those disconnected at
            /// publish time). Note that offsets are never reused, so even if
            /// DELIVER_IMMEDIATELY is used, subscribers will not receive different
            /// messages when re-reading, they will just see gaps. EXAMPLE:
            ///   (0) Topic 'topic1' is created with a single partition.
            ///   (1) Two subscriptions 'sub1' and 'sub2' are created on topic1. sub1
            ///       has 'DELIVER_IMMEDIATELY', sub2 has 'DELIVER_AFTER_STORED'.
            ///   (2) A stream is opened for sub1 but not sub2.
            ///   (3) A stream is opened for a publisher client using pub1.
            ///   (4) pub1 successfully publishes m0 at offset 0 and m0 is delivered to
            ///       sub1.
            ///   (5) pub1 publishes m1 at offset 1 and m1 is delivered to sub1 but the
            ///       write to storage fails (their stream then breaks).
            ///   (6) A stream is reopened for pub1.
            ///   (6) pub1 successfully publishes m2 at offset 2 and m2 is delivered to
            ///       sub1.
            ///   (some time elapses...)
            ///   (7) A stream is opened for sub2 and it receives m0 and m2 but not m1.
            ///   (8) sub1 seeks to offset 1 but only receives m2 and not m1.
            DeliverImmediately = 1,
            /// The server will not deliver a published message to subscribers until
            /// the message has been successfully written to storage. This will result
            /// in higher end-to-end latency, but consistent delivery.
            DeliverAfterStored = 2,
        }
    }
}
/// Request for CreateTopic.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateTopicRequest {
    /// Required. The parent location in which to create the topic.
    /// Structured like `projects/{project_number}/locations/{location}`.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Required. Configuration of the topic to create. Its `name` field is ignored.
    #[prost(message, optional, tag = "2")]
    pub topic: ::std::option::Option<Topic>,
    /// Required. The ID to use for the topic, which will become the final component of
    /// the topic's name.
    ///
    /// This value is structured like: `my-topic-name`.
    #[prost(string, tag = "3")]
    pub topic_id: std::string::String,
}
/// Request for GetTopic.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTopicRequest {
    /// Required. The name of the topic whose configuration to return.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Request for GetTopicPartitions.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTopicPartitionsRequest {
    /// Required. The topic whose partition information to return.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Response for GetTopicPartitions.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TopicPartitions {
    /// The number of partitions in the topic.
    #[prost(int64, tag = "1")]
    pub partition_count: i64,
}
/// Request for ListTopics.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTopicsRequest {
    /// Required. The parent whose topics are to be listed.
    /// Structured like `projects/{project_number}/locations/{location}`.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// The maximum number of topics to return. The service may return fewer than
    /// this value.
    /// If unset or zero, all topics for the parent will be returned.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A page token, received from a previous `ListTopics` call.
    /// Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to `ListTopics` must match
    /// the call that provided the page token.
    #[prost(string, tag = "3")]
    pub page_token: std::string::String,
}
/// Response for ListTopics.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTopicsResponse {
    /// The list of topic in the requested parent. The order of the topics is
    /// unspecified.
    #[prost(message, repeated, tag = "1")]
    pub topics: ::std::vec::Vec<Topic>,
    /// A token that can be sent as `page_token` to retrieve the next page of
    /// results. If this field is omitted, there are no more results.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
}
/// Request for UpdateTopic.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateTopicRequest {
    /// Required. The topic to update. Its `name` field must be populated.
    #[prost(message, optional, tag = "1")]
    pub topic: ::std::option::Option<Topic>,
    /// Required. A mask specifying the topic fields to change.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
}
/// Request for DeleteTopic.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteTopicRequest {
    /// Required. The name of the topic to delete.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Request for ListTopicSubscriptions.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTopicSubscriptionsRequest {
    /// Required. The name of the topic whose subscriptions to list.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// The maximum number of subscriptions to return. The service may return fewer
    /// than this value.
    /// If unset or zero, all subscriptions for the given topic will be returned.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A page token, received from a previous `ListTopicSubscriptions` call.
    /// Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to `ListTopicSubscriptions`
    /// must match the call that provided the page token.
    #[prost(string, tag = "3")]
    pub page_token: std::string::String,
}
/// Response for ListTopicSubscriptions.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTopicSubscriptionsResponse {
    /// The names of subscriptions attached to the topic. The order of the
    /// subscriptions is unspecified.
    #[prost(string, repeated, tag = "1")]
    pub subscriptions: ::std::vec::Vec<std::string::String>,
    /// A token that can be sent as `page_token` to retrieve the next page of
    /// results. If this field is omitted, there are no more results.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
}
/// Request for CreateSubscription.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateSubscriptionRequest {
    /// Required. The parent location in which to create the subscription.
    /// Structured like `projects/{project_number}/locations/{location}`.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Required. Configuration of the subscription to create. Its `name` field is ignored.
    #[prost(message, optional, tag = "2")]
    pub subscription: ::std::option::Option<Subscription>,
    /// Required. The ID to use for the subscription, which will become the final component
    /// of the subscription's name.
    ///
    /// This value is structured like: `my-sub-name`.
    #[prost(string, tag = "3")]
    pub subscription_id: std::string::String,
}
/// Request for GetSubscription.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSubscriptionRequest {
    /// Required. The name of the subscription whose configuration to return.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Request for ListSubscriptions.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSubscriptionsRequest {
    /// Required. The parent whose subscriptions are to be listed.
    /// Structured like `projects/{project_number}/locations/{location}`.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// The maximum number of subscriptions to return. The service may return fewer
    /// than this value.
    /// If unset or zero, all subscriptions for the parent will be returned.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A page token, received from a previous `ListSubscriptions` call.
    /// Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to `ListSubscriptions` must
    /// match the call that provided the page token.
    #[prost(string, tag = "3")]
    pub page_token: std::string::String,
}
/// Response for ListSubscriptions.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSubscriptionsResponse {
    /// The list of subscriptions in the requested parent. The order of the
    /// subscriptions is unspecified.
    #[prost(message, repeated, tag = "1")]
    pub subscriptions: ::std::vec::Vec<Subscription>,
    /// A token that can be sent as `page_token` to retrieve the next page of
    /// results. If this field is omitted, there are no more results.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
}
/// Request for UpdateSubscription.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateSubscriptionRequest {
    /// Required. The subscription to update. Its `name` field must be populated.
    /// Topic field must not be populated.
    #[prost(message, optional, tag = "1")]
    pub subscription: ::std::option::Option<Subscription>,
    /// Required. A mask specifying the subscription fields to change.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
}
/// Request for DeleteSubscription.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteSubscriptionRequest {
    /// Required. The name of the subscription to delete.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
#[doc = r" Generated client implementations."]
pub mod admin_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " The service that a client application uses to manage topics and"]
    #[doc = " subscriptions, such creating, listing, and deleting topics and subscriptions."]
    pub struct AdminServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> AdminServiceClient<T>
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
        #[doc = " Creates a new topic."]
        pub async fn create_topic(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateTopicRequest>,
        ) -> Result<tonic::Response<super::Topic>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.pubsublite.v1.AdminService/CreateTopic",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns the topic configuration."]
        pub async fn get_topic(
            &mut self,
            request: impl tonic::IntoRequest<super::GetTopicRequest>,
        ) -> Result<tonic::Response<super::Topic>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.pubsublite.v1.AdminService/GetTopic",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns the partition information for the requested topic."]
        pub async fn get_topic_partitions(
            &mut self,
            request: impl tonic::IntoRequest<super::GetTopicPartitionsRequest>,
        ) -> Result<tonic::Response<super::TopicPartitions>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.pubsublite.v1.AdminService/GetTopicPartitions",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns the list of topics for the given project."]
        pub async fn list_topics(
            &mut self,
            request: impl tonic::IntoRequest<super::ListTopicsRequest>,
        ) -> Result<tonic::Response<super::ListTopicsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.pubsublite.v1.AdminService/ListTopics",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates properties of the specified topic."]
        pub async fn update_topic(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateTopicRequest>,
        ) -> Result<tonic::Response<super::Topic>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.pubsublite.v1.AdminService/UpdateTopic",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes the specified topic."]
        pub async fn delete_topic(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteTopicRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.pubsublite.v1.AdminService/DeleteTopic",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists the subscriptions attached to the specified topic."]
        pub async fn list_topic_subscriptions(
            &mut self,
            request: impl tonic::IntoRequest<super::ListTopicSubscriptionsRequest>,
        ) -> Result<tonic::Response<super::ListTopicSubscriptionsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.pubsublite.v1.AdminService/ListTopicSubscriptions",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a new subscription."]
        pub async fn create_subscription(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateSubscriptionRequest>,
        ) -> Result<tonic::Response<super::Subscription>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.pubsublite.v1.AdminService/CreateSubscription",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns the subscription configuration."]
        pub async fn get_subscription(
            &mut self,
            request: impl tonic::IntoRequest<super::GetSubscriptionRequest>,
        ) -> Result<tonic::Response<super::Subscription>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.pubsublite.v1.AdminService/GetSubscription",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns the list of subscriptions for the given project."]
        pub async fn list_subscriptions(
            &mut self,
            request: impl tonic::IntoRequest<super::ListSubscriptionsRequest>,
        ) -> Result<tonic::Response<super::ListSubscriptionsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.pubsublite.v1.AdminService/ListSubscriptions",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates properties of the specified subscription."]
        pub async fn update_subscription(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateSubscriptionRequest>,
        ) -> Result<tonic::Response<super::Subscription>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.pubsublite.v1.AdminService/UpdateSubscription",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes the specified subscription."]
        pub async fn delete_subscription(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteSubscriptionRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.pubsublite.v1.AdminService/DeleteSubscription",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for AdminServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for AdminServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "AdminServiceClient {{ ... }}")
        }
    }
}
/// The first streaming request that must be sent on a newly-opened stream. The
/// client must wait for the response before sending subsequent requests on the
/// stream.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InitialCommitCursorRequest {
    /// The subscription for which to manage committed cursors.
    #[prost(string, tag = "1")]
    pub subscription: std::string::String,
    /// The partition for which to manage committed cursors. Partitions are zero
    /// indexed, so `partition` must be in the range [0, topic.num_partitions).
    #[prost(int64, tag = "2")]
    pub partition: i64,
}
/// Response to an InitialCommitCursorRequest.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InitialCommitCursorResponse {}
/// Streaming request to update the committed cursor. Subsequent
/// SequencedCommitCursorRequests override outstanding ones.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SequencedCommitCursorRequest {
    /// The new value for the committed cursor.
    #[prost(message, optional, tag = "1")]
    pub cursor: ::std::option::Option<Cursor>,
}
/// Response to a SequencedCommitCursorRequest.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SequencedCommitCursorResponse {
    /// The number of outstanding SequencedCommitCursorRequests acknowledged by
    /// this response. Note that SequencedCommitCursorRequests are acknowledged in
    /// the order that they are received.
    #[prost(int64, tag = "1")]
    pub acknowledged_commits: i64,
}
/// A request sent from the client to the server on a stream.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamingCommitCursorRequest {
    #[prost(oneof = "streaming_commit_cursor_request::Request", tags = "1, 2")]
    pub request: ::std::option::Option<streaming_commit_cursor_request::Request>,
}
pub mod streaming_commit_cursor_request {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Request {
        /// Initial request on the stream.
        #[prost(message, tag = "1")]
        Initial(super::InitialCommitCursorRequest),
        /// Request to commit a new cursor value.
        #[prost(message, tag = "2")]
        Commit(super::SequencedCommitCursorRequest),
    }
}
/// Response to a StreamingCommitCursorRequest.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamingCommitCursorResponse {
    #[prost(oneof = "streaming_commit_cursor_response::Request", tags = "1, 2")]
    pub request: ::std::option::Option<streaming_commit_cursor_response::Request>,
}
pub mod streaming_commit_cursor_response {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Request {
        /// Initial response on the stream.
        #[prost(message, tag = "1")]
        Initial(super::InitialCommitCursorResponse),
        /// Response to committing a new cursor value.
        #[prost(message, tag = "2")]
        Commit(super::SequencedCommitCursorResponse),
    }
}
/// Request for CommitCursor.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommitCursorRequest {
    /// The subscription for which to update the cursor.
    #[prost(string, tag = "1")]
    pub subscription: std::string::String,
    /// The partition for which to update the cursor. Partitions are zero indexed,
    /// so `partition` must be in the range [0, topic.num_partitions).
    #[prost(int64, tag = "2")]
    pub partition: i64,
    /// The new value for the committed cursor.
    #[prost(message, optional, tag = "3")]
    pub cursor: ::std::option::Option<Cursor>,
}
/// Response for CommitCursor.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommitCursorResponse {}
/// Request for ListPartitionCursors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPartitionCursorsRequest {
    /// Required. The subscription for which to retrieve cursors.
    /// Structured like
    /// `projects/{project_number}/locations/{location}/subscriptions/{subscription_id}`.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// The maximum number of cursors to return. The service may return fewer than
    /// this value.
    /// If unset or zero, all cursors for the parent will be returned.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A page token, received from a previous `ListPartitionCursors` call.
    /// Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to `ListPartitionCursors`
    /// must match the call that provided the page token.
    #[prost(string, tag = "3")]
    pub page_token: std::string::String,
}
/// A pair of a Cursor and the partition it is for.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PartitionCursor {
    /// The partition this is for.
    #[prost(int64, tag = "1")]
    pub partition: i64,
    /// The value of the cursor.
    #[prost(message, optional, tag = "2")]
    pub cursor: ::std::option::Option<Cursor>,
}
/// Response for ListPartitionCursors
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPartitionCursorsResponse {
    /// The partition cursors from this request.
    #[prost(message, repeated, tag = "1")]
    pub partition_cursors: ::std::vec::Vec<PartitionCursor>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
}
#[doc = r" Generated client implementations."]
pub mod cursor_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " The service that a subscriber client application uses to manage committed"]
    #[doc = " cursors while receiving messsages. A cursor represents a subscriber's"]
    #[doc = " progress within a topic partition for a given subscription."]
    pub struct CursorServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> CursorServiceClient<T>
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
        #[doc = " Establishes a stream with the server for managing committed cursors."]
        pub async fn streaming_commit_cursor(
            &mut self,
            request: impl tonic::IntoStreamingRequest<Message = super::StreamingCommitCursorRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::StreamingCommitCursorResponse>>,
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
                "/google.cloud.pubsublite.v1.CursorService/StreamingCommitCursor",
            );
            self.inner
                .streaming(request.into_streaming_request(), path, codec)
                .await
        }
        #[doc = " Updates the committed cursor."]
        pub async fn commit_cursor(
            &mut self,
            request: impl tonic::IntoRequest<super::CommitCursorRequest>,
        ) -> Result<tonic::Response<super::CommitCursorResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.pubsublite.v1.CursorService/CommitCursor",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns all committed cursor information for a subscription."]
        pub async fn list_partition_cursors(
            &mut self,
            request: impl tonic::IntoRequest<super::ListPartitionCursorsRequest>,
        ) -> Result<tonic::Response<super::ListPartitionCursorsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.pubsublite.v1.CursorService/ListPartitionCursors",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for CursorServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for CursorServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "CursorServiceClient {{ ... }}")
        }
    }
}
/// The first request that must be sent on a newly-opened stream.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InitialPublishRequest {
    /// The topic to which messages will be written.
    #[prost(string, tag = "1")]
    pub topic: std::string::String,
    /// The partition within the topic to which messages will be written.
    /// Partitions are zero indexed, so `partition` must be in the range [0,
    /// topic.num_partitions).
    #[prost(int64, tag = "2")]
    pub partition: i64,
}
/// Response to an InitialPublishRequest.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InitialPublishResponse {}
/// Request to publish messages to the topic.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MessagePublishRequest {
    #[prost(message, repeated, tag = "1")]
    pub messages: ::std::vec::Vec<PubSubMessage>,
}
/// Response to a MessagePublishRequest.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MessagePublishResponse {
    /// The cursor of the first published message in the batch. The cursors for any
    /// remaining messages in the batch are guaranteed to be sequential.
    #[prost(message, optional, tag = "1")]
    pub start_cursor: ::std::option::Option<Cursor>,
}
/// Request sent from the client to the server on a stream.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PublishRequest {
    #[prost(oneof = "publish_request::RequestType", tags = "1, 2")]
    pub request_type: ::std::option::Option<publish_request::RequestType>,
}
pub mod publish_request {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum RequestType {
        /// Initial request on the stream.
        #[prost(message, tag = "1")]
        InitialRequest(super::InitialPublishRequest),
        /// Request to publish messages.
        #[prost(message, tag = "2")]
        MessagePublishRequest(super::MessagePublishRequest),
    }
}
/// Response to a PublishRequest.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PublishResponse {
    #[prost(oneof = "publish_response::ResponseType", tags = "1, 2")]
    pub response_type: ::std::option::Option<publish_response::ResponseType>,
}
pub mod publish_response {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ResponseType {
        /// Initial response on the stream.
        #[prost(message, tag = "1")]
        InitialResponse(super::InitialPublishResponse),
        /// Response to publishing messages.
        #[prost(message, tag = "2")]
        MessageResponse(super::MessagePublishResponse),
    }
}
#[doc = r" Generated client implementations."]
pub mod publisher_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " The service that a publisher client application uses to publish messages to"]
    #[doc = " topics. Published messages are retained by the service for the duration of"]
    #[doc = " the retention period configured for the respective topic, and are delivered"]
    #[doc = " to subscriber clients upon request (via the `SubscriberService`)."]
    pub struct PublisherServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> PublisherServiceClient<T>
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
        #[doc = " Establishes a stream with the server for publishing messages. Once the"]
        #[doc = " stream is initialized, the client publishes messages by sending publish"]
        #[doc = " requests on the stream. The server responds with a PublishResponse for each"]
        #[doc = " PublishRequest sent by the client, in the same order that the requests"]
        #[doc = " were sent. Note that multiple PublishRequests can be in flight"]
        #[doc = " simultaneously, but they will be processed by the server in the order that"]
        #[doc = " they are sent by the client on a given stream."]
        pub async fn publish(
            &mut self,
            request: impl tonic::IntoStreamingRequest<Message = super::PublishRequest>,
        ) -> Result<tonic::Response<tonic::codec::Streaming<super::PublishResponse>>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.pubsublite.v1.PublisherService/Publish",
            );
            self.inner
                .streaming(request.into_streaming_request(), path, codec)
                .await
        }
    }
    impl<T: Clone> Clone for PublisherServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for PublisherServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "PublisherServiceClient {{ ... }}")
        }
    }
}
/// The first request that must be sent on a newly-opened stream. The client must
/// wait for the response before sending subsequent requests on the stream.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InitialSubscribeRequest {
    /// The subscription from which to receive messages.
    #[prost(string, tag = "1")]
    pub subscription: std::string::String,
    /// The partition from which to receive messages. Partitions are zero indexed,
    /// so `partition` must be in the range [0, topic.num_partitions).
    #[prost(int64, tag = "2")]
    pub partition: i64,
}
/// Response to an InitialSubscribeRequest.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InitialSubscribeResponse {
    /// The cursor from which the subscriber will start receiving messages once
    /// flow control tokens become available.
    #[prost(message, optional, tag = "1")]
    pub cursor: ::std::option::Option<Cursor>,
}
/// Request to update the stream's delivery cursor based on the given target.
/// Resets the server available tokens to 0. SeekRequests may not be sent while
/// another SeekRequest is outstanding (i.e., has not received a SeekResponse) on
/// the same stream. SeekRequests past head result in stream breakage.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SeekRequest {
    /// The target to seek to. Must be set.
    #[prost(oneof = "seek_request::Target", tags = "1, 2")]
    pub target: ::std::option::Option<seek_request::Target>,
}
pub mod seek_request {
    /// A special target in the partition that takes no other parameters.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum NamedTarget {
        /// Default value. This value is unused.
        Unspecified = 0,
        /// A target corresponding to the most recently published message in the
        /// partition.
        Head = 1,
        /// A target corresponding to the committed cursor for the given subscription
        /// and topic partition.
        CommittedCursor = 2,
    }
    /// The target to seek to. Must be set.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Target {
        /// A named target.
        #[prost(enumeration = "NamedTarget", tag = "1")]
        NamedTarget(i32),
        /// A target corresponding to the cursor, pointing to anywhere in the
        /// topic partition.
        #[prost(message, tag = "2")]
        Cursor(super::Cursor),
    }
}
/// Response to a SeekRequest.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SeekResponse {
    /// The new delivery cursor for the current stream.
    #[prost(message, optional, tag = "1")]
    pub cursor: ::std::option::Option<Cursor>,
}
/// Request to grant tokens to the server, requesting delivery of messages when
/// they become available.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FlowControlRequest {
    /// The number of message tokens to grant. Must be greater than or equal to 0.
    #[prost(int64, tag = "1")]
    pub allowed_messages: i64,
    /// The number of byte tokens to grant. Must be greater than or equal to 0.
    #[prost(int64, tag = "2")]
    pub allowed_bytes: i64,
}
/// A request sent from the client to the server on a stream.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeRequest {
    #[prost(oneof = "subscribe_request::Request", tags = "1, 2, 3")]
    pub request: ::std::option::Option<subscribe_request::Request>,
}
pub mod subscribe_request {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Request {
        /// Initial request on the stream.
        #[prost(message, tag = "1")]
        Initial(super::InitialSubscribeRequest),
        /// Request to update the stream's delivery cursor.
        #[prost(message, tag = "2")]
        Seek(super::SeekRequest),
        /// Request to grant tokens to the server,
        #[prost(message, tag = "3")]
        FlowControl(super::FlowControlRequest),
    }
}
/// Response containing a list of messages. Upon delivering a MessageResponse to
/// the client, the server:
/// *  Updates the stream's delivery cursor to one greater than the cursor of the
///    last message in the list.
/// *  Subtracts the total number of bytes and messages from the tokens available
///    to the server.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MessageResponse {
    /// Messages from the topic partition.
    #[prost(message, repeated, tag = "1")]
    pub messages: ::std::vec::Vec<SequencedMessage>,
}
/// Response to SubscribeRequest.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeResponse {
    #[prost(oneof = "subscribe_response::Response", tags = "1, 2, 3")]
    pub response: ::std::option::Option<subscribe_response::Response>,
}
pub mod subscribe_response {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Response {
        /// Initial response on the stream.
        #[prost(message, tag = "1")]
        Initial(super::InitialSubscribeResponse),
        /// Response to a Seek operation.
        #[prost(message, tag = "2")]
        Seek(super::SeekResponse),
        /// Response containing messages from the topic partition.
        #[prost(message, tag = "3")]
        Messages(super::MessageResponse),
    }
}
#[doc = r" Generated client implementations."]
pub mod subscriber_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " The service that a subscriber client application uses to receive messages"]
    #[doc = " from subscriptions."]
    pub struct SubscriberServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> SubscriberServiceClient<T>
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
        #[doc = " Establishes a stream with the server for receiving messages."]
        pub async fn subscribe(
            &mut self,
            request: impl tonic::IntoStreamingRequest<Message = super::SubscribeRequest>,
        ) -> Result<tonic::Response<tonic::codec::Streaming<super::SubscribeResponse>>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.pubsublite.v1.SubscriberService/Subscribe",
            );
            self.inner
                .streaming(request.into_streaming_request(), path, codec)
                .await
        }
    }
    impl<T: Clone> Clone for SubscriberServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for SubscriberServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "SubscriberServiceClient {{ ... }}")
        }
    }
}
