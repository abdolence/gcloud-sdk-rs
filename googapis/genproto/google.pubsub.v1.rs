/// A policy constraining the storage of messages published to the topic.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MessageStoragePolicy {
    /// A list of IDs of GCP regions where messages that are published to the topic
    /// may be persisted in storage. Messages published by publishers running in
    /// non-allowed GCP regions (or running outside of GCP altogether) will be
    /// routed for storage in one of the allowed regions. An empty list means that
    /// no regions are allowed, and is not a valid configuration.
    #[prost(string, repeated, tag = "1")]
    pub allowed_persistence_regions: ::std::vec::Vec<std::string::String>,
}
/// A topic resource.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Topic {
    /// Required. The name of the topic. It must have the format
    /// `"projects/{project}/topics/{topic}"`. `{topic}` must start with a letter,
    /// and contain only letters (`[A-Za-z]`), numbers (`[0-9]`), dashes (`-`),
    /// underscores (`_`), periods (`.`), tildes (`~`), plus (`+`) or percent
    /// signs (`%`). It must be between 3 and 255 characters in length, and it
    /// must not start with `"goog"`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// See <a href="https://cloud.google.com/pubsub/docs/labels"> Creating and
    /// managing labels</a>.
    #[prost(map = "string, string", tag = "2")]
    pub labels: ::std::collections::HashMap<std::string::String, std::string::String>,
    /// Policy constraining the set of Google Cloud Platform regions where messages
    /// published to the topic may be stored. If not present, then no constraints
    /// are in effect.
    #[prost(message, optional, tag = "3")]
    pub message_storage_policy: ::std::option::Option<MessageStoragePolicy>,
    /// The resource name of the Cloud KMS CryptoKey to be used to protect access
    /// to messages published on this topic.
    ///
    /// The expected format is `projects/*/locations/*/keyRings/*/cryptoKeys/*`.
    #[prost(string, tag = "5")]
    pub kms_key_name: std::string::String,
}
/// A message that is published by publishers and consumed by subscribers. The
/// message must contain either a non-empty data field or at least one attribute.
/// Note that client libraries represent this object differently
/// depending on the language. See the corresponding
/// <a href="https://cloud.google.com/pubsub/docs/reference/libraries">client
/// library documentation</a> for more information. See
/// <a href="https://cloud.google.com/pubsub/quotas">Quotas and limits</a>
/// for more information about message limits.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PubsubMessage {
    /// The message data field. If this field is empty, the message must contain
    /// at least one attribute.
    #[prost(bytes, tag = "1")]
    pub data: std::vec::Vec<u8>,
    /// Attributes for this message. If this field is empty, the message must
    /// contain non-empty data. This can be used to filter messages on the
    /// subscription.
    #[prost(map = "string, string", tag = "2")]
    pub attributes: ::std::collections::HashMap<std::string::String, std::string::String>,
    /// ID of this message, assigned by the server when the message is published.
    /// Guaranteed to be unique within the topic. This value may be read by a
    /// subscriber that receives a `PubsubMessage` via a `Pull` call or a push
    /// delivery. It must not be populated by the publisher in a `Publish` call.
    #[prost(string, tag = "3")]
    pub message_id: std::string::String,
    /// The time at which the message was published, populated by the server when
    /// it receives the `Publish` call. It must not be populated by the
    /// publisher in a `Publish` call.
    #[prost(message, optional, tag = "4")]
    pub publish_time: ::std::option::Option<::prost_types::Timestamp>,
    /// If non-empty, identifies related messages for which publish order should be
    /// respected. If a `Subscription` has `enable_message_ordering` set to `true`,
    /// messages published with the same non-empty `ordering_key` value will be
    /// delivered to subscribers in the order in which they are received by the
    /// Pub/Sub system. All `PubsubMessage`s published in a given `PublishRequest`
    /// must specify the same `ordering_key` value.
    /// <b>EXPERIMENTAL:</b> This feature is part of a closed alpha release. This
    /// API might be changed in backward-incompatible ways and is not recommended
    /// for production use. It is not subject to any SLA or deprecation policy.
    #[prost(string, tag = "5")]
    pub ordering_key: std::string::String,
}
/// Request for the GetTopic method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTopicRequest {
    /// Required. The name of the topic to get.
    /// Format is `projects/{project}/topics/{topic}`.
    #[prost(string, tag = "1")]
    pub topic: std::string::String,
}
/// Request for the UpdateTopic method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateTopicRequest {
    /// Required. The updated topic object.
    #[prost(message, optional, tag = "1")]
    pub topic: ::std::option::Option<Topic>,
    /// Required. Indicates which fields in the provided topic to update. Must be
    /// specified and non-empty. Note that if `update_mask` contains
    /// "message_storage_policy" but the `message_storage_policy` is not set in
    /// the `topic` provided above, then the updated value is determined by the
    /// policy configured at the project or organization level.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
}
/// Request for the Publish method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PublishRequest {
    /// Required. The messages in the request will be published on this topic.
    /// Format is `projects/{project}/topics/{topic}`.
    #[prost(string, tag = "1")]
    pub topic: std::string::String,
    /// Required. The messages to publish.
    #[prost(message, repeated, tag = "2")]
    pub messages: ::std::vec::Vec<PubsubMessage>,
}
/// Response for the `Publish` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PublishResponse {
    /// The server-assigned ID of each published message, in the same order as
    /// the messages in the request. IDs are guaranteed to be unique within
    /// the topic.
    #[prost(string, repeated, tag = "1")]
    pub message_ids: ::std::vec::Vec<std::string::String>,
}
/// Request for the `ListTopics` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTopicsRequest {
    /// Required. The name of the project in which to list topics.
    /// Format is `projects/{project-id}`.
    #[prost(string, tag = "1")]
    pub project: std::string::String,
    /// Maximum number of topics to return.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The value returned by the last `ListTopicsResponse`; indicates that this is
    /// a continuation of a prior `ListTopics` call, and that the system should
    /// return the next page of data.
    #[prost(string, tag = "3")]
    pub page_token: std::string::String,
}
/// Response for the `ListTopics` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTopicsResponse {
    /// The resulting topics.
    #[prost(message, repeated, tag = "1")]
    pub topics: ::std::vec::Vec<Topic>,
    /// If not empty, indicates that there may be more topics that match the
    /// request; this value should be passed in a new `ListTopicsRequest`.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
}
/// Request for the `ListTopicSubscriptions` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTopicSubscriptionsRequest {
    /// Required. The name of the topic that subscriptions are attached to.
    /// Format is `projects/{project}/topics/{topic}`.
    #[prost(string, tag = "1")]
    pub topic: std::string::String,
    /// Maximum number of subscription names to return.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The value returned by the last `ListTopicSubscriptionsResponse`; indicates
    /// that this is a continuation of a prior `ListTopicSubscriptions` call, and
    /// that the system should return the next page of data.
    #[prost(string, tag = "3")]
    pub page_token: std::string::String,
}
/// Response for the `ListTopicSubscriptions` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTopicSubscriptionsResponse {
    /// The names of subscriptions attached to the topic specified in the request.
    #[prost(string, repeated, tag = "1")]
    pub subscriptions: ::std::vec::Vec<std::string::String>,
    /// If not empty, indicates that there may be more subscriptions that match
    /// the request; this value should be passed in a new
    /// `ListTopicSubscriptionsRequest` to get more subscriptions.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
}
/// Request for the `ListTopicSnapshots` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTopicSnapshotsRequest {
    /// Required. The name of the topic that snapshots are attached to.
    /// Format is `projects/{project}/topics/{topic}`.
    #[prost(string, tag = "1")]
    pub topic: std::string::String,
    /// Maximum number of snapshot names to return.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The value returned by the last `ListTopicSnapshotsResponse`; indicates
    /// that this is a continuation of a prior `ListTopicSnapshots` call, and
    /// that the system should return the next page of data.
    #[prost(string, tag = "3")]
    pub page_token: std::string::String,
}
/// Response for the `ListTopicSnapshots` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTopicSnapshotsResponse {
    /// The names of the snapshots that match the request.
    #[prost(string, repeated, tag = "1")]
    pub snapshots: ::std::vec::Vec<std::string::String>,
    /// If not empty, indicates that there may be more snapshots that match
    /// the request; this value should be passed in a new
    /// `ListTopicSnapshotsRequest` to get more snapshots.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
}
/// Request for the `DeleteTopic` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteTopicRequest {
    /// Required. Name of the topic to delete.
    /// Format is `projects/{project}/topics/{topic}`.
    #[prost(string, tag = "1")]
    pub topic: std::string::String,
}
/// Request for the DetachSubscription method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DetachSubscriptionRequest {
    /// Required. The subscription to detach.
    /// Format is `projects/{project}/subscriptions/{subscription}`.
    #[prost(string, tag = "1")]
    pub subscription: std::string::String,
}
/// Response for the DetachSubscription method.
/// Reserved for future use.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DetachSubscriptionResponse {}
/// A subscription resource.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Subscription {
    /// Required. The name of the subscription. It must have the format
    /// `"projects/{project}/subscriptions/{subscription}"`. `{subscription}` must
    /// start with a letter, and contain only letters (`[A-Za-z]`), numbers
    /// (`[0-9]`), dashes (`-`), underscores (`_`), periods (`.`), tildes (`~`),
    /// plus (`+`) or percent signs (`%`). It must be between 3 and 255 characters
    /// in length, and it must not start with `"goog"`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Required. The name of the topic from which this subscription is receiving
    /// messages. Format is `projects/{project}/topics/{topic}`. The value of this
    /// field will be `_deleted-topic_` if the topic has been deleted.
    #[prost(string, tag = "2")]
    pub topic: std::string::String,
    /// If push delivery is used with this subscription, this field is
    /// used to configure it. An empty `pushConfig` signifies that the subscriber
    /// will pull and ack messages using API methods.
    #[prost(message, optional, tag = "4")]
    pub push_config: ::std::option::Option<PushConfig>,
    /// The approximate amount of time (on a best-effort basis) Pub/Sub waits for
    /// the subscriber to acknowledge receipt before resending the message. In the
    /// interval after the message is delivered and before it is acknowledged, it
    /// is considered to be <i>outstanding</i>. During that time period, the
    /// message will not be redelivered (on a best-effort basis).
    ///
    /// For pull subscriptions, this value is used as the initial value for the ack
    /// deadline. To override this value for a given message, call
    /// `ModifyAckDeadline` with the corresponding `ack_id` if using
    /// non-streaming pull or send the `ack_id` in a
    /// `StreamingModifyAckDeadlineRequest` if using streaming pull.
    /// The minimum custom deadline you can specify is 10 seconds.
    /// The maximum custom deadline you can specify is 600 seconds (10 minutes).
    /// If this parameter is 0, a default value of 10 seconds is used.
    ///
    /// For push delivery, this value is also used to set the request timeout for
    /// the call to the push endpoint.
    ///
    /// If the subscriber never acknowledges the message, the Pub/Sub
    /// system will eventually redeliver the message.
    #[prost(int32, tag = "5")]
    pub ack_deadline_seconds: i32,
    /// Indicates whether to retain acknowledged messages. If true, then
    /// messages are not expunged from the subscription's backlog, even if they are
    /// acknowledged, until they fall out of the `message_retention_duration`
    /// window. This must be true if you would like to
    /// <a
    /// href="https://cloud.google.com/pubsub/docs/replay-overview#seek_to_a_time">
    /// Seek to a timestamp</a>.
    #[prost(bool, tag = "7")]
    pub retain_acked_messages: bool,
    /// How long to retain unacknowledged messages in the subscription's backlog,
    /// from the moment a message is published.
    /// If `retain_acked_messages` is true, then this also configures the retention
    /// of acknowledged messages, and thus configures how far back in time a `Seek`
    /// can be done. Defaults to 7 days. Cannot be more than 7 days or less than 10
    /// minutes.
    #[prost(message, optional, tag = "8")]
    pub message_retention_duration: ::std::option::Option<::prost_types::Duration>,
    /// See <a href="https://cloud.google.com/pubsub/docs/labels"> Creating and
    /// managing labels</a>.
    #[prost(map = "string, string", tag = "9")]
    pub labels: ::std::collections::HashMap<std::string::String, std::string::String>,
    /// If true, messages published with the same `ordering_key` in `PubsubMessage`
    /// will be delivered to the subscribers in the order in which they
    /// are received by the Pub/Sub system. Otherwise, they may be delivered in
    /// any order.
    /// <b>EXPERIMENTAL:</b> This feature is part of a closed alpha release. This
    /// API might be changed in backward-incompatible ways and is not recommended
    /// for production use. It is not subject to any SLA or deprecation policy.
    #[prost(bool, tag = "10")]
    pub enable_message_ordering: bool,
    /// A policy that specifies the conditions for this subscription's expiration.
    /// A subscription is considered active as long as any connected subscriber is
    /// successfully consuming messages from the subscription or is issuing
    /// operations on the subscription. If `expiration_policy` is not set, a
    /// *default policy* with `ttl` of 31 days will be used. The minimum allowed
    /// value for `expiration_policy.ttl` is 1 day.
    #[prost(message, optional, tag = "11")]
    pub expiration_policy: ::std::option::Option<ExpirationPolicy>,
    /// An expression written in the Pub/Sub [filter
    /// language](https://cloud.google.com/pubsub/docs/filtering). If non-empty,
    /// then only `PubsubMessage`s whose `attributes` field matches the filter are
    /// delivered on this subscription. If empty, then no messages are filtered
    /// out.
    #[prost(string, tag = "12")]
    pub filter: std::string::String,
    /// A policy that specifies the conditions for dead lettering messages in
    /// this subscription. If dead_letter_policy is not set, dead lettering
    /// is disabled.
    ///
    /// The Cloud Pub/Sub service account associated with this subscriptions's
    /// parent project (i.e.,
    /// service-{project_number}@gcp-sa-pubsub.iam.gserviceaccount.com) must have
    /// permission to Acknowledge() messages on this subscription.
    #[prost(message, optional, tag = "13")]
    pub dead_letter_policy: ::std::option::Option<DeadLetterPolicy>,
    /// A policy that specifies how Pub/Sub retries message delivery for this
    /// subscription.
    ///
    /// If not set, the default retry policy is applied. This generally implies
    /// that messages will be retried as soon as possible for healthy subscribers.
    /// RetryPolicy will be triggered on NACKs or acknowledgement deadline
    /// exceeded events for a given message.
    #[prost(message, optional, tag = "14")]
    pub retry_policy: ::std::option::Option<RetryPolicy>,
    /// Indicates whether the subscription is detached from its topic. Detached
    /// subscriptions don't receive messages from their topic and don't retain any
    /// backlog. `Pull` and `StreamingPull` requests will return
    /// FAILED_PRECONDITION. If the subscription is a push subscription, pushes to
    /// the endpoint will not be made.
    #[prost(bool, tag = "15")]
    pub detached: bool,
}
/// A policy that specifies how Cloud Pub/Sub retries message delivery.
///
/// Retry delay will be exponential based on provided minimum and maximum
/// backoffs. https://en.wikipedia.org/wiki/Exponential_backoff.
///
/// RetryPolicy will be triggered on NACKs or acknowledgement deadline exceeded
/// events for a given message.
///
/// Retry Policy is implemented on a best effort basis. At times, the delay
/// between consecutive deliveries may not match the configuration. That is,
/// delay can be more or less than configured backoff.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RetryPolicy {
    /// The minimum delay between consecutive deliveries of a given message.
    /// Value should be between 0 and 600 seconds. Defaults to 10 seconds.
    #[prost(message, optional, tag = "1")]
    pub minimum_backoff: ::std::option::Option<::prost_types::Duration>,
    /// The maximum delay between consecutive deliveries of a given message.
    /// Value should be between 0 and 600 seconds. Defaults to 600 seconds.
    #[prost(message, optional, tag = "2")]
    pub maximum_backoff: ::std::option::Option<::prost_types::Duration>,
}
/// Dead lettering is done on a best effort basis. The same message might be
/// dead lettered multiple times.
///
/// If validation on any of the fields fails at subscription creation/updation,
/// the create/update subscription request will fail.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeadLetterPolicy {
    /// The name of the topic to which dead letter messages should be published.
    /// Format is `projects/{project}/topics/{topic}`.The Cloud Pub/Sub service
    /// account associated with the enclosing subscription's parent project (i.e.,
    /// service-{project_number}@gcp-sa-pubsub.iam.gserviceaccount.com) must have
    /// permission to Publish() to this topic.
    ///
    /// The operation will fail if the topic does not exist.
    /// Users should ensure that there is a subscription attached to this topic
    /// since messages published to a topic with no subscriptions are lost.
    #[prost(string, tag = "1")]
    pub dead_letter_topic: std::string::String,
    /// The maximum number of delivery attempts for any message. The value must be
    /// between 5 and 100.
    ///
    /// The number of delivery attempts is defined as 1 + (the sum of number of
    /// NACKs and number of times the acknowledgement deadline has been exceeded
    /// for the message).
    ///
    /// A NACK is any call to ModifyAckDeadline with a 0 deadline. Note that
    /// client libraries may automatically extend ack_deadlines.
    ///
    /// This field will be honored on a best effort basis.
    ///
    /// If this parameter is 0, a default value of 5 is used.
    #[prost(int32, tag = "2")]
    pub max_delivery_attempts: i32,
}
/// A policy that specifies the conditions for resource expiration (i.e.,
/// automatic resource deletion).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExpirationPolicy {
    /// Specifies the "time-to-live" duration for an associated resource. The
    /// resource expires if it is not active for a period of `ttl`. The definition
    /// of "activity" depends on the type of the associated resource. The minimum
    /// and maximum allowed values for `ttl` depend on the type of the associated
    /// resource, as well. If `ttl` is not set, the associated resource never
    /// expires.
    #[prost(message, optional, tag = "1")]
    pub ttl: ::std::option::Option<::prost_types::Duration>,
}
/// Configuration for a push delivery endpoint.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PushConfig {
    /// A URL locating the endpoint to which messages should be pushed.
    /// For example, a Webhook endpoint might use `https://example.com/push`.
    #[prost(string, tag = "1")]
    pub push_endpoint: std::string::String,
    /// Endpoint configuration attributes that can be used to control different
    /// aspects of the message delivery.
    ///
    /// The only currently supported attribute is `x-goog-version`, which you can
    /// use to change the format of the pushed message. This attribute
    /// indicates the version of the data expected by the endpoint. This
    /// controls the shape of the pushed message (i.e., its fields and metadata).
    ///
    /// If not present during the `CreateSubscription` call, it will default to
    /// the version of the Pub/Sub API used to make such call. If not present in a
    /// `ModifyPushConfig` call, its value will not be changed. `GetSubscription`
    /// calls will always return a valid version, even if the subscription was
    /// created without this attribute.
    ///
    /// The only supported values for the `x-goog-version` attribute are:
    ///
    /// * `v1beta1`: uses the push format defined in the v1beta1 Pub/Sub API.
    /// * `v1` or `v1beta2`: uses the push format defined in the v1 Pub/Sub API.
    ///
    /// For example:
    /// <pre><code>attributes { "x-goog-version": "v1" } </code></pre>
    #[prost(map = "string, string", tag = "2")]
    pub attributes: ::std::collections::HashMap<std::string::String, std::string::String>,
    /// An authentication method used by push endpoints to verify the source of
    /// push requests. This can be used with push endpoints that are private by
    /// default to allow requests only from the Cloud Pub/Sub system, for example.
    /// This field is optional and should be set only by users interested in
    /// authenticated push.
    #[prost(oneof = "push_config::AuthenticationMethod", tags = "3")]
    pub authentication_method: ::std::option::Option<push_config::AuthenticationMethod>,
}
pub mod push_config {
    /// Contains information needed for generating an
    /// [OpenID Connect
    /// token](https://developers.google.com/identity/protocols/OpenIDConnect).
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct OidcToken {
        /// [Service account
        /// email](https://cloud.google.com/iam/docs/service-accounts)
        /// to be used for generating the OIDC token. The caller (for
        /// CreateSubscription, UpdateSubscription, and ModifyPushConfig RPCs) must
        /// have the iam.serviceAccounts.actAs permission for the service account.
        #[prost(string, tag = "1")]
        pub service_account_email: std::string::String,
        /// Audience to be used when generating OIDC token. The audience claim
        /// identifies the recipients that the JWT is intended for. The audience
        /// value is a single case-sensitive string. Having multiple values (array)
        /// for the audience field is not supported. More info about the OIDC JWT
        /// token audience here: https://tools.ietf.org/html/rfc7519#section-4.1.3
        /// Note: if not specified, the Push endpoint URL will be used.
        #[prost(string, tag = "2")]
        pub audience: std::string::String,
    }
    /// An authentication method used by push endpoints to verify the source of
    /// push requests. This can be used with push endpoints that are private by
    /// default to allow requests only from the Cloud Pub/Sub system, for example.
    /// This field is optional and should be set only by users interested in
    /// authenticated push.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum AuthenticationMethod {
        /// If specified, Pub/Sub will generate and attach an OIDC JWT token as an
        /// `Authorization` header in the HTTP request for every pushed message.
        #[prost(message, tag = "3")]
        OidcToken(OidcToken),
    }
}
/// A message and its corresponding acknowledgment ID.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReceivedMessage {
    /// This ID can be used to acknowledge the received message.
    #[prost(string, tag = "1")]
    pub ack_id: std::string::String,
    /// The message.
    #[prost(message, optional, tag = "2")]
    pub message: ::std::option::Option<PubsubMessage>,
    /// The approximate number of times that Cloud Pub/Sub has attempted to deliver
    /// the associated message to a subscriber.
    ///
    /// More precisely, this is 1 + (number of NACKs) +
    /// (number of ack_deadline exceeds) for this message.
    ///
    /// A NACK is any call to ModifyAckDeadline with a 0 deadline. An ack_deadline
    /// exceeds event is whenever a message is not acknowledged within
    /// ack_deadline. Note that ack_deadline is initially
    /// Subscription.ackDeadlineSeconds, but may get extended automatically by
    /// the client library.
    ///
    /// Upon the first delivery of a given message, `delivery_attempt` will have a
    /// value of 1. The value is calculated at best effort and is approximate.
    ///
    /// If a DeadLetterPolicy is not set on the subscription, this will be 0.
    #[prost(int32, tag = "3")]
    pub delivery_attempt: i32,
}
/// Request for the GetSubscription method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSubscriptionRequest {
    /// Required. The name of the subscription to get.
    /// Format is `projects/{project}/subscriptions/{sub}`.
    #[prost(string, tag = "1")]
    pub subscription: std::string::String,
}
/// Request for the UpdateSubscription method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateSubscriptionRequest {
    /// Required. The updated subscription object.
    #[prost(message, optional, tag = "1")]
    pub subscription: ::std::option::Option<Subscription>,
    /// Required. Indicates which fields in the provided subscription to update.
    /// Must be specified and non-empty.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
}
/// Request for the `ListSubscriptions` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSubscriptionsRequest {
    /// Required. The name of the project in which to list subscriptions.
    /// Format is `projects/{project-id}`.
    #[prost(string, tag = "1")]
    pub project: std::string::String,
    /// Maximum number of subscriptions to return.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The value returned by the last `ListSubscriptionsResponse`; indicates that
    /// this is a continuation of a prior `ListSubscriptions` call, and that the
    /// system should return the next page of data.
    #[prost(string, tag = "3")]
    pub page_token: std::string::String,
}
/// Response for the `ListSubscriptions` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSubscriptionsResponse {
    /// The subscriptions that match the request.
    #[prost(message, repeated, tag = "1")]
    pub subscriptions: ::std::vec::Vec<Subscription>,
    /// If not empty, indicates that there may be more subscriptions that match
    /// the request; this value should be passed in a new
    /// `ListSubscriptionsRequest` to get more subscriptions.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
}
/// Request for the DeleteSubscription method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteSubscriptionRequest {
    /// Required. The subscription to delete.
    /// Format is `projects/{project}/subscriptions/{sub}`.
    #[prost(string, tag = "1")]
    pub subscription: std::string::String,
}
/// Request for the ModifyPushConfig method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModifyPushConfigRequest {
    /// Required. The name of the subscription.
    /// Format is `projects/{project}/subscriptions/{sub}`.
    #[prost(string, tag = "1")]
    pub subscription: std::string::String,
    /// Required. The push configuration for future deliveries.
    ///
    /// An empty `pushConfig` indicates that the Pub/Sub system should
    /// stop pushing messages from the given subscription and allow
    /// messages to be pulled and acknowledged - effectively pausing
    /// the subscription if `Pull` or `StreamingPull` is not called.
    #[prost(message, optional, tag = "2")]
    pub push_config: ::std::option::Option<PushConfig>,
}
/// Request for the `Pull` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PullRequest {
    /// Required. The subscription from which messages should be pulled.
    /// Format is `projects/{project}/subscriptions/{sub}`.
    #[prost(string, tag = "1")]
    pub subscription: std::string::String,
    /// Optional. If this field set to true, the system will respond immediately
    /// even if it there are no messages available to return in the `Pull`
    /// response. Otherwise, the system may wait (for a bounded amount of time)
    /// until at least one message is available, rather than returning no messages.
    /// Warning: setting this field to `true` is discouraged because it adversely
    /// impacts the performance of `Pull` operations. We recommend that users do
    /// not set this field.
    #[prost(bool, tag = "2")]
    pub return_immediately: bool,
    /// Required. The maximum number of messages to return for this request. Must
    /// be a positive integer. The Pub/Sub system may return fewer than the number
    /// specified.
    #[prost(int32, tag = "3")]
    pub max_messages: i32,
}
/// Response for the `Pull` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PullResponse {
    /// Received Pub/Sub messages. The list will be empty if there are no more
    /// messages available in the backlog. For JSON, the response can be entirely
    /// empty. The Pub/Sub system may return fewer than the `maxMessages` requested
    /// even if there are more messages available in the backlog.
    #[prost(message, repeated, tag = "1")]
    pub received_messages: ::std::vec::Vec<ReceivedMessage>,
}
/// Request for the ModifyAckDeadline method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModifyAckDeadlineRequest {
    /// Required. The name of the subscription.
    /// Format is `projects/{project}/subscriptions/{sub}`.
    #[prost(string, tag = "1")]
    pub subscription: std::string::String,
    /// Required. List of acknowledgment IDs.
    #[prost(string, repeated, tag = "4")]
    pub ack_ids: ::std::vec::Vec<std::string::String>,
    /// Required. The new ack deadline with respect to the time this request was
    /// sent to the Pub/Sub system. For example, if the value is 10, the new ack
    /// deadline will expire 10 seconds after the `ModifyAckDeadline` call was
    /// made. Specifying zero might immediately make the message available for
    /// delivery to another subscriber client. This typically results in an
    /// increase in the rate of message redeliveries (that is, duplicates).
    /// The minimum deadline you can specify is 0 seconds.
    /// The maximum deadline you can specify is 600 seconds (10 minutes).
    #[prost(int32, tag = "3")]
    pub ack_deadline_seconds: i32,
}
/// Request for the Acknowledge method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AcknowledgeRequest {
    /// Required. The subscription whose message is being acknowledged.
    /// Format is `projects/{project}/subscriptions/{sub}`.
    #[prost(string, tag = "1")]
    pub subscription: std::string::String,
    /// Required. The acknowledgment ID for the messages being acknowledged that
    /// was returned by the Pub/Sub system in the `Pull` response. Must not be
    /// empty.
    #[prost(string, repeated, tag = "2")]
    pub ack_ids: ::std::vec::Vec<std::string::String>,
}
/// Request for the `StreamingPull` streaming RPC method. This request is used to
/// establish the initial stream as well as to stream acknowledgements and ack
/// deadline modifications from the client to the server.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamingPullRequest {
    /// Required. The subscription for which to initialize the new stream. This
    /// must be provided in the first request on the stream, and must not be set in
    /// subsequent requests from client to server.
    /// Format is `projects/{project}/subscriptions/{sub}`.
    #[prost(string, tag = "1")]
    pub subscription: std::string::String,
    /// List of acknowledgement IDs for acknowledging previously received messages
    /// (received on this stream or a different stream). If an ack ID has expired,
    /// the corresponding message may be redelivered later. Acknowledging a message
    /// more than once will not result in an error. If the acknowledgement ID is
    /// malformed, the stream will be aborted with status `INVALID_ARGUMENT`.
    #[prost(string, repeated, tag = "2")]
    pub ack_ids: ::std::vec::Vec<std::string::String>,
    /// The list of new ack deadlines for the IDs listed in
    /// `modify_deadline_ack_ids`. The size of this list must be the same as the
    /// size of `modify_deadline_ack_ids`. If it differs the stream will be aborted
    /// with `INVALID_ARGUMENT`. Each element in this list is applied to the
    /// element in the same position in `modify_deadline_ack_ids`. The new ack
    /// deadline is with respect to the time this request was sent to the Pub/Sub
    /// system. Must be >= 0. For example, if the value is 10, the new ack deadline
    /// will expire 10 seconds after this request is received. If the value is 0,
    /// the message is immediately made available for another streaming or
    /// non-streaming pull request. If the value is < 0 (an error), the stream will
    /// be aborted with status `INVALID_ARGUMENT`.
    #[prost(int32, repeated, tag = "3")]
    pub modify_deadline_seconds: ::std::vec::Vec<i32>,
    /// List of acknowledgement IDs whose deadline will be modified based on the
    /// corresponding element in `modify_deadline_seconds`. This field can be used
    /// to indicate that more time is needed to process a message by the
    /// subscriber, or to make the message available for redelivery if the
    /// processing was interrupted.
    #[prost(string, repeated, tag = "4")]
    pub modify_deadline_ack_ids: ::std::vec::Vec<std::string::String>,
    /// Required. The ack deadline to use for the stream. This must be provided in
    /// the first request on the stream, but it can also be updated on subsequent
    /// requests from client to server. The minimum deadline you can specify is 10
    /// seconds. The maximum deadline you can specify is 600 seconds (10 minutes).
    #[prost(int32, tag = "5")]
    pub stream_ack_deadline_seconds: i32,
    /// A unique identifier that is used to distinguish client instances from each
    /// other. Only needs to be provided on the initial request. When a stream
    /// disconnects and reconnects for the same stream, the client_id should be set
    /// to the same value so that state associated with the old stream can be
    /// transferred to the new stream. The same client_id should not be used for
    /// different client instances.
    #[prost(string, tag = "6")]
    pub client_id: std::string::String,
    /// Flow control settings for the maximum number of outstanding messages. When
    /// there are `max_outstanding_messages` or more currently sent to the
    /// streaming pull client that have not yet been acked or nacked, the server
    /// stops sending more messages. The sending of messages resumes once the
    /// number of outstanding messages is less than this value. If the value is
    /// <= 0, there is no limit to the number of outstanding messages. This
    /// property can only be set on the initial StreamingPullRequest. If it is set
    /// on a subsequent request, the stream will be aborted with status
    /// `INVALID_ARGUMENT`.
    #[prost(int64, tag = "7")]
    pub max_outstanding_messages: i64,
    /// Flow control settings for the maximum number of outstanding bytes. When
    /// there are `max_outstanding_bytes` or more worth of messages currently sent
    /// to the streaming pull client that have not yet been acked or nacked, the
    /// server will stop sending more messages. The sending of messages resumes
    /// once the number of outstanding bytes is less than this value. If the value
    /// is <= 0, there is no limit to the number of outstanding bytes. This
    /// property can only be set on the initial StreamingPullRequest. If it is set
    /// on a subsequent request, the stream will be aborted with status
    /// `INVALID_ARGUMENT`.
    #[prost(int64, tag = "8")]
    pub max_outstanding_bytes: i64,
}
/// Response for the `StreamingPull` method. This response is used to stream
/// messages from the server to the client.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamingPullResponse {
    /// Received Pub/Sub messages. This will not be empty.
    #[prost(message, repeated, tag = "1")]
    pub received_messages: ::std::vec::Vec<ReceivedMessage>,
}
/// Request for the `CreateSnapshot` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateSnapshotRequest {
    /// Required. User-provided name for this snapshot. If the name is not provided
    /// in the request, the server will assign a random name for this snapshot on
    /// the same project as the subscription. Note that for REST API requests, you
    /// must specify a name.  See the <a
    /// href="https://cloud.google.com/pubsub/docs/admin#resource_names"> resource
    /// name rules</a>. Format is `projects/{project}/snapshots/{snap}`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Required. The subscription whose backlog the snapshot retains.
    /// Specifically, the created snapshot is guaranteed to retain:
    ///  (a) The existing backlog on the subscription. More precisely, this is
    ///      defined as the messages in the subscription's backlog that are
    ///      unacknowledged upon the successful completion of the
    ///      `CreateSnapshot` request; as well as:
    ///  (b) Any messages published to the subscription's topic following the
    ///      successful completion of the CreateSnapshot request.
    /// Format is `projects/{project}/subscriptions/{sub}`.
    #[prost(string, tag = "2")]
    pub subscription: std::string::String,
    /// See <a href="https://cloud.google.com/pubsub/docs/labels"> Creating and
    /// managing labels</a>.
    #[prost(map = "string, string", tag = "3")]
    pub labels: ::std::collections::HashMap<std::string::String, std::string::String>,
}
/// Request for the UpdateSnapshot method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateSnapshotRequest {
    /// Required. The updated snapshot object.
    #[prost(message, optional, tag = "1")]
    pub snapshot: ::std::option::Option<Snapshot>,
    /// Required. Indicates which fields in the provided snapshot to update.
    /// Must be specified and non-empty.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
}
/// A snapshot resource. Snapshots are used in
/// <a href="https://cloud.google.com/pubsub/docs/replay-overview">Seek</a>
/// operations, which allow
/// you to manage message acknowledgments in bulk. That is, you can set the
/// acknowledgment state of messages in an existing subscription to the state
/// captured by a snapshot.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Snapshot {
    /// The name of the snapshot.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// The name of the topic from which this snapshot is retaining messages.
    #[prost(string, tag = "2")]
    pub topic: std::string::String,
    /// The snapshot is guaranteed to exist up until this time.
    /// A newly-created snapshot expires no later than 7 days from the time of its
    /// creation. Its exact lifetime is determined at creation by the existing
    /// backlog in the source subscription. Specifically, the lifetime of the
    /// snapshot is `7 days - (age of oldest unacked message in the subscription)`.
    /// For example, consider a subscription whose oldest unacked message is 3 days
    /// old. If a snapshot is created from this subscription, the snapshot -- which
    /// will always capture this 3-day-old backlog as long as the snapshot
    /// exists -- will expire in 4 days. The service will refuse to create a
    /// snapshot that would expire in less than 1 hour after creation.
    #[prost(message, optional, tag = "3")]
    pub expire_time: ::std::option::Option<::prost_types::Timestamp>,
    /// See <a href="https://cloud.google.com/pubsub/docs/labels"> Creating and
    /// managing labels</a>.
    #[prost(map = "string, string", tag = "4")]
    pub labels: ::std::collections::HashMap<std::string::String, std::string::String>,
}
/// Request for the GetSnapshot method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSnapshotRequest {
    /// Required. The name of the snapshot to get.
    /// Format is `projects/{project}/snapshots/{snap}`.
    #[prost(string, tag = "1")]
    pub snapshot: std::string::String,
}
/// Request for the `ListSnapshots` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSnapshotsRequest {
    /// Required. The name of the project in which to list snapshots.
    /// Format is `projects/{project-id}`.
    #[prost(string, tag = "1")]
    pub project: std::string::String,
    /// Maximum number of snapshots to return.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The value returned by the last `ListSnapshotsResponse`; indicates that this
    /// is a continuation of a prior `ListSnapshots` call, and that the system
    /// should return the next page of data.
    #[prost(string, tag = "3")]
    pub page_token: std::string::String,
}
/// Response for the `ListSnapshots` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSnapshotsResponse {
    /// The resulting snapshots.
    #[prost(message, repeated, tag = "1")]
    pub snapshots: ::std::vec::Vec<Snapshot>,
    /// If not empty, indicates that there may be more snapshot that match the
    /// request; this value should be passed in a new `ListSnapshotsRequest`.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
}
/// Request for the `DeleteSnapshot` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteSnapshotRequest {
    /// Required. The name of the snapshot to delete.
    /// Format is `projects/{project}/snapshots/{snap}`.
    #[prost(string, tag = "1")]
    pub snapshot: std::string::String,
}
/// Request for the `Seek` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SeekRequest {
    /// Required. The subscription to affect.
    #[prost(string, tag = "1")]
    pub subscription: std::string::String,
    #[prost(oneof = "seek_request::Target", tags = "2, 3")]
    pub target: ::std::option::Option<seek_request::Target>,
}
pub mod seek_request {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Target {
        /// The time to seek to.
        /// Messages retained in the subscription that were published before this
        /// time are marked as acknowledged, and messages retained in the
        /// subscription that were published after this time are marked as
        /// unacknowledged. Note that this operation affects only those messages
        /// retained in the subscription (configured by the combination of
        /// `message_retention_duration` and `retain_acked_messages`). For example,
        /// if `time` corresponds to a point before the message retention
        /// window (or to a point before the system's notion of the subscription
        /// creation time), only retained messages will be marked as unacknowledged,
        /// and already-expunged messages will not be restored.
        #[prost(message, tag = "2")]
        Time(::prost_types::Timestamp),
        /// The snapshot to seek to. The snapshot's topic must be the same as that of
        /// the provided subscription.
        /// Format is `projects/{project}/snapshots/{snap}`.
        #[prost(string, tag = "3")]
        Snapshot(std::string::String),
    }
}
/// Response for the `Seek` method (this response is empty).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SeekResponse {}
#[doc = r" Generated client implementations."]
pub mod publisher_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " The service that an application uses to manipulate topics, and to send"]
    #[doc = " messages to a topic."]
    pub struct PublisherClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> PublisherClient<T>
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
        #[doc = " Creates the given topic with the given name. See the"]
        #[doc = " <a href=\"https://cloud.google.com/pubsub/docs/admin#resource_names\">"]
        #[doc = " resource name rules</a>."]
        pub async fn create_topic(
            &mut self,
            request: impl tonic::IntoRequest<super::Topic>,
        ) -> Result<tonic::Response<super::Topic>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/google.pubsub.v1.Publisher/CreateTopic");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates an existing topic. Note that certain properties of a"]
        #[doc = " topic are not modifiable."]
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
            let path =
                http::uri::PathAndQuery::from_static("/google.pubsub.v1.Publisher/UpdateTopic");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Adds one or more messages to the topic. Returns `NOT_FOUND` if the topic"]
        #[doc = " does not exist."]
        pub async fn publish(
            &mut self,
            request: impl tonic::IntoRequest<super::PublishRequest>,
        ) -> Result<tonic::Response<super::PublishResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/google.pubsub.v1.Publisher/Publish");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets the configuration of a topic."]
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
            let path = http::uri::PathAndQuery::from_static("/google.pubsub.v1.Publisher/GetTopic");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists matching topics."]
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
            let path =
                http::uri::PathAndQuery::from_static("/google.pubsub.v1.Publisher/ListTopics");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists the names of the attached subscriptions on this topic."]
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
                "/google.pubsub.v1.Publisher/ListTopicSubscriptions",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists the names of the snapshots on this topic. Snapshots are used in"]
        #[doc = " <a href=\"https://cloud.google.com/pubsub/docs/replay-overview\">Seek</a>"]
        #[doc = " operations, which allow"]
        #[doc = " you to manage message acknowledgments in bulk. That is, you can set the"]
        #[doc = " acknowledgment state of messages in an existing subscription to the state"]
        #[doc = " captured by a snapshot."]
        pub async fn list_topic_snapshots(
            &mut self,
            request: impl tonic::IntoRequest<super::ListTopicSnapshotsRequest>,
        ) -> Result<tonic::Response<super::ListTopicSnapshotsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.pubsub.v1.Publisher/ListTopicSnapshots",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes the topic with the given name. Returns `NOT_FOUND` if the topic"]
        #[doc = " does not exist. After a topic is deleted, a new topic may be created with"]
        #[doc = " the same name; this is an entirely new topic with none of the old"]
        #[doc = " configuration or subscriptions. Existing subscriptions to this topic are"]
        #[doc = " not deleted, but their `topic` field is set to `_deleted-topic_`."]
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
            let path =
                http::uri::PathAndQuery::from_static("/google.pubsub.v1.Publisher/DeleteTopic");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Detaches a subscription from this topic. All messages retained in the"]
        #[doc = " subscription are dropped. Subsequent `Pull` and `StreamingPull` requests"]
        #[doc = " will return FAILED_PRECONDITION. If the subscription is a push"]
        #[doc = " subscription, pushes to the endpoint will stop."]
        pub async fn detach_subscription(
            &mut self,
            request: impl tonic::IntoRequest<super::DetachSubscriptionRequest>,
        ) -> Result<tonic::Response<super::DetachSubscriptionResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.pubsub.v1.Publisher/DetachSubscription",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for PublisherClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for PublisherClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "PublisherClient {{ ... }}")
        }
    }
}
#[doc = r" Generated client implementations."]
pub mod subscriber_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " The service that an application uses to manipulate subscriptions and to"]
    #[doc = " consume messages from a subscription via the `Pull` method or by"]
    #[doc = " establishing a bi-directional stream using the `StreamingPull` method."]
    pub struct SubscriberClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> SubscriberClient<T>
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
        #[doc = " Creates a subscription to a given topic. See the"]
        #[doc = " <a href=\"https://cloud.google.com/pubsub/docs/admin#resource_names\">"]
        #[doc = " resource name rules</a>."]
        #[doc = " If the subscription already exists, returns `ALREADY_EXISTS`."]
        #[doc = " If the corresponding topic doesn't exist, returns `NOT_FOUND`."]
        #[doc = ""]
        #[doc = " If the name is not provided in the request, the server will assign a random"]
        #[doc = " name for this subscription on the same project as the topic, conforming"]
        #[doc = " to the"]
        #[doc = " [resource name"]
        #[doc = " format](https://cloud.google.com/pubsub/docs/admin#resource_names). The"]
        #[doc = " generated name is populated in the returned Subscription object. Note that"]
        #[doc = " for REST API requests, you must specify a name in the request."]
        pub async fn create_subscription(
            &mut self,
            request: impl tonic::IntoRequest<super::Subscription>,
        ) -> Result<tonic::Response<super::Subscription>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.pubsub.v1.Subscriber/CreateSubscription",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets the configuration details of a subscription."]
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
                "/google.pubsub.v1.Subscriber/GetSubscription",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates an existing subscription. Note that certain properties of a"]
        #[doc = " subscription, such as its topic, are not modifiable."]
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
                "/google.pubsub.v1.Subscriber/UpdateSubscription",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists matching subscriptions."]
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
                "/google.pubsub.v1.Subscriber/ListSubscriptions",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes an existing subscription. All messages retained in the subscription"]
        #[doc = " are immediately dropped. Calls to `Pull` after deletion will return"]
        #[doc = " `NOT_FOUND`. After a subscription is deleted, a new one may be created with"]
        #[doc = " the same name, but the new one has no association with the old"]
        #[doc = " subscription or its topic unless the same topic is specified."]
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
                "/google.pubsub.v1.Subscriber/DeleteSubscription",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Modifies the ack deadline for a specific message. This method is useful"]
        #[doc = " to indicate that more time is needed to process a message by the"]
        #[doc = " subscriber, or to make the message available for redelivery if the"]
        #[doc = " processing was interrupted. Note that this does not modify the"]
        #[doc = " subscription-level `ackDeadlineSeconds` used for subsequent messages."]
        pub async fn modify_ack_deadline(
            &mut self,
            request: impl tonic::IntoRequest<super::ModifyAckDeadlineRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.pubsub.v1.Subscriber/ModifyAckDeadline",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Acknowledges the messages associated with the `ack_ids` in the"]
        #[doc = " `AcknowledgeRequest`. The Pub/Sub system can remove the relevant messages"]
        #[doc = " from the subscription."]
        #[doc = ""]
        #[doc = " Acknowledging a message whose ack deadline has expired may succeed,"]
        #[doc = " but such a message may be redelivered later. Acknowledging a message more"]
        #[doc = " than once will not result in an error."]
        pub async fn acknowledge(
            &mut self,
            request: impl tonic::IntoRequest<super::AcknowledgeRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/google.pubsub.v1.Subscriber/Acknowledge");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Pulls messages from the server. The server may return `UNAVAILABLE` if"]
        #[doc = " there are too many concurrent pull requests pending for the given"]
        #[doc = " subscription."]
        pub async fn pull(
            &mut self,
            request: impl tonic::IntoRequest<super::PullRequest>,
        ) -> Result<tonic::Response<super::PullResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/google.pubsub.v1.Subscriber/Pull");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Establishes a stream with the server, which sends messages down to the"]
        #[doc = " client. The client streams acknowledgements and ack deadline modifications"]
        #[doc = " back to the server. The server will close the stream and return the status"]
        #[doc = " on any error. The server may close the stream with status `UNAVAILABLE` to"]
        #[doc = " reassign server-side resources, in which case, the client should"]
        #[doc = " re-establish the stream. Flow control can be achieved by configuring the"]
        #[doc = " underlying RPC channel."]
        pub async fn streaming_pull(
            &mut self,
            request: impl tonic::IntoStreamingRequest<Message = super::StreamingPullRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::StreamingPullResponse>>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/google.pubsub.v1.Subscriber/StreamingPull");
            self.inner
                .streaming(request.into_streaming_request(), path, codec)
                .await
        }
        #[doc = " Modifies the `PushConfig` for a specified subscription."]
        #[doc = ""]
        #[doc = " This may be used to change a push subscription to a pull one (signified by"]
        #[doc = " an empty `PushConfig`) or vice versa, or change the endpoint URL and other"]
        #[doc = " attributes of a push subscription. Messages will accumulate for delivery"]
        #[doc = " continuously through the call regardless of changes to the `PushConfig`."]
        pub async fn modify_push_config(
            &mut self,
            request: impl tonic::IntoRequest<super::ModifyPushConfigRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.pubsub.v1.Subscriber/ModifyPushConfig",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets the configuration details of a snapshot. Snapshots are used in"]
        #[doc = " <a href=\"https://cloud.google.com/pubsub/docs/replay-overview\">Seek</a>"]
        #[doc = " operations, which allow you to manage message acknowledgments in bulk. That"]
        #[doc = " is, you can set the acknowledgment state of messages in an existing"]
        #[doc = " subscription to the state captured by a snapshot."]
        pub async fn get_snapshot(
            &mut self,
            request: impl tonic::IntoRequest<super::GetSnapshotRequest>,
        ) -> Result<tonic::Response<super::Snapshot>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/google.pubsub.v1.Subscriber/GetSnapshot");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists the existing snapshots. Snapshots are used in"]
        #[doc = " <a href=\"https://cloud.google.com/pubsub/docs/replay-overview\">Seek</a>"]
        #[doc = " operations, which allow"]
        #[doc = " you to manage message acknowledgments in bulk. That is, you can set the"]
        #[doc = " acknowledgment state of messages in an existing subscription to the state"]
        #[doc = " captured by a snapshot."]
        pub async fn list_snapshots(
            &mut self,
            request: impl tonic::IntoRequest<super::ListSnapshotsRequest>,
        ) -> Result<tonic::Response<super::ListSnapshotsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/google.pubsub.v1.Subscriber/ListSnapshots");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a snapshot from the requested subscription. Snapshots are used in"]
        #[doc = " <a href=\"https://cloud.google.com/pubsub/docs/replay-overview\">Seek</a>"]
        #[doc = " operations, which allow"]
        #[doc = " you to manage message acknowledgments in bulk. That is, you can set the"]
        #[doc = " acknowledgment state of messages in an existing subscription to the state"]
        #[doc = " captured by a snapshot."]
        #[doc = " <br><br>If the snapshot already exists, returns `ALREADY_EXISTS`."]
        #[doc = " If the requested subscription doesn't exist, returns `NOT_FOUND`."]
        #[doc = " If the backlog in the subscription is too old -- and the resulting snapshot"]
        #[doc = " would expire in less than 1 hour -- then `FAILED_PRECONDITION` is returned."]
        #[doc = " See also the `Snapshot.expire_time` field. If the name is not provided in"]
        #[doc = " the request, the server will assign a random"]
        #[doc = " name for this snapshot on the same project as the subscription, conforming"]
        #[doc = " to the"]
        #[doc = " [resource name"]
        #[doc = " format](https://cloud.google.com/pubsub/docs/admin#resource_names). The"]
        #[doc = " generated name is populated in the returned Snapshot object. Note that for"]
        #[doc = " REST API requests, you must specify a name in the request."]
        pub async fn create_snapshot(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateSnapshotRequest>,
        ) -> Result<tonic::Response<super::Snapshot>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/google.pubsub.v1.Subscriber/CreateSnapshot");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates an existing snapshot. Snapshots are used in"]
        #[doc = " <a href=\"https://cloud.google.com/pubsub/docs/replay-overview\">Seek</a>"]
        #[doc = " operations, which allow"]
        #[doc = " you to manage message acknowledgments in bulk. That is, you can set the"]
        #[doc = " acknowledgment state of messages in an existing subscription to the state"]
        #[doc = " captured by a snapshot."]
        pub async fn update_snapshot(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateSnapshotRequest>,
        ) -> Result<tonic::Response<super::Snapshot>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/google.pubsub.v1.Subscriber/UpdateSnapshot");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Removes an existing snapshot. Snapshots are used in"]
        #[doc = " <a href=\"https://cloud.google.com/pubsub/docs/replay-overview\">Seek</a>"]
        #[doc = " operations, which allow"]
        #[doc = " you to manage message acknowledgments in bulk. That is, you can set the"]
        #[doc = " acknowledgment state of messages in an existing subscription to the state"]
        #[doc = " captured by a snapshot.<br><br>"]
        #[doc = " When the snapshot is deleted, all messages retained in the snapshot"]
        #[doc = " are immediately dropped. After a snapshot is deleted, a new one may be"]
        #[doc = " created with the same name, but the new one has no association with the old"]
        #[doc = " snapshot or its subscription, unless the same subscription is specified."]
        pub async fn delete_snapshot(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteSnapshotRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/google.pubsub.v1.Subscriber/DeleteSnapshot");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Seeks an existing subscription to a point in time or to a given snapshot,"]
        #[doc = " whichever is provided in the request. Snapshots are used in"]
        #[doc = " <a href=\"https://cloud.google.com/pubsub/docs/replay-overview\">Seek</a>"]
        #[doc = " operations, which allow"]
        #[doc = " you to manage message acknowledgments in bulk. That is, you can set the"]
        #[doc = " acknowledgment state of messages in an existing subscription to the state"]
        #[doc = " captured by a snapshot. Note that both the subscription and the snapshot"]
        #[doc = " must be on the same topic."]
        pub async fn seek(
            &mut self,
            request: impl tonic::IntoRequest<super::SeekRequest>,
        ) -> Result<tonic::Response<super::SeekResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/google.pubsub.v1.Subscriber/Seek");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for SubscriberClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for SubscriberClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "SubscriberClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod publisher_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with PublisherServer."]
    #[async_trait]
    pub trait Publisher: Send + Sync + 'static {
        #[doc = " Creates the given topic with the given name. See the"]
        #[doc = " <a href=\"https://cloud.google.com/pubsub/docs/admin#resource_names\">"]
        #[doc = " resource name rules</a>."]
        async fn create_topic(
            &self,
            request: tonic::Request<super::Topic>,
        ) -> Result<tonic::Response<super::Topic>, tonic::Status>;
        #[doc = " Updates an existing topic. Note that certain properties of a"]
        #[doc = " topic are not modifiable."]
        async fn update_topic(
            &self,
            request: tonic::Request<super::UpdateTopicRequest>,
        ) -> Result<tonic::Response<super::Topic>, tonic::Status>;
        #[doc = " Adds one or more messages to the topic. Returns `NOT_FOUND` if the topic"]
        #[doc = " does not exist."]
        async fn publish(
            &self,
            request: tonic::Request<super::PublishRequest>,
        ) -> Result<tonic::Response<super::PublishResponse>, tonic::Status>;
        #[doc = " Gets the configuration of a topic."]
        async fn get_topic(
            &self,
            request: tonic::Request<super::GetTopicRequest>,
        ) -> Result<tonic::Response<super::Topic>, tonic::Status>;
        #[doc = " Lists matching topics."]
        async fn list_topics(
            &self,
            request: tonic::Request<super::ListTopicsRequest>,
        ) -> Result<tonic::Response<super::ListTopicsResponse>, tonic::Status>;
        #[doc = " Lists the names of the attached subscriptions on this topic."]
        async fn list_topic_subscriptions(
            &self,
            request: tonic::Request<super::ListTopicSubscriptionsRequest>,
        ) -> Result<tonic::Response<super::ListTopicSubscriptionsResponse>, tonic::Status>;
        #[doc = " Lists the names of the snapshots on this topic. Snapshots are used in"]
        #[doc = " <a href=\"https://cloud.google.com/pubsub/docs/replay-overview\">Seek</a>"]
        #[doc = " operations, which allow"]
        #[doc = " you to manage message acknowledgments in bulk. That is, you can set the"]
        #[doc = " acknowledgment state of messages in an existing subscription to the state"]
        #[doc = " captured by a snapshot."]
        async fn list_topic_snapshots(
            &self,
            request: tonic::Request<super::ListTopicSnapshotsRequest>,
        ) -> Result<tonic::Response<super::ListTopicSnapshotsResponse>, tonic::Status>;
        #[doc = " Deletes the topic with the given name. Returns `NOT_FOUND` if the topic"]
        #[doc = " does not exist. After a topic is deleted, a new topic may be created with"]
        #[doc = " the same name; this is an entirely new topic with none of the old"]
        #[doc = " configuration or subscriptions. Existing subscriptions to this topic are"]
        #[doc = " not deleted, but their `topic` field is set to `_deleted-topic_`."]
        async fn delete_topic(
            &self,
            request: tonic::Request<super::DeleteTopicRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status>;
        #[doc = " Detaches a subscription from this topic. All messages retained in the"]
        #[doc = " subscription are dropped. Subsequent `Pull` and `StreamingPull` requests"]
        #[doc = " will return FAILED_PRECONDITION. If the subscription is a push"]
        #[doc = " subscription, pushes to the endpoint will stop."]
        async fn detach_subscription(
            &self,
            request: tonic::Request<super::DetachSubscriptionRequest>,
        ) -> Result<tonic::Response<super::DetachSubscriptionResponse>, tonic::Status>;
    }
    #[doc = " The service that an application uses to manipulate topics, and to send"]
    #[doc = " messages to a topic."]
    #[derive(Debug)]
    pub struct PublisherServer<T: Publisher> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: Publisher> PublisherServer<T> {
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
    impl<T, B> Service<http::Request<B>> for PublisherServer<T>
    where
        T: Publisher,
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
                "/google.pubsub.v1.Publisher/CreateTopic" => {
                    #[allow(non_camel_case_types)]
                    struct CreateTopicSvc<T: Publisher>(pub Arc<T>);
                    impl<T: Publisher> tonic::server::UnaryService<super::Topic> for CreateTopicSvc<T> {
                        type Response = super::Topic;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(&mut self, request: tonic::Request<super::Topic>) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).create_topic(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = CreateTopicSvc(inner);
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
                "/google.pubsub.v1.Publisher/UpdateTopic" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateTopicSvc<T: Publisher>(pub Arc<T>);
                    impl<T: Publisher> tonic::server::UnaryService<super::UpdateTopicRequest> for UpdateTopicSvc<T> {
                        type Response = super::Topic;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateTopicRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).update_topic(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = UpdateTopicSvc(inner);
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
                "/google.pubsub.v1.Publisher/Publish" => {
                    #[allow(non_camel_case_types)]
                    struct PublishSvc<T: Publisher>(pub Arc<T>);
                    impl<T: Publisher> tonic::server::UnaryService<super::PublishRequest> for PublishSvc<T> {
                        type Response = super::PublishResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PublishRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).publish(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = PublishSvc(inner);
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
                "/google.pubsub.v1.Publisher/GetTopic" => {
                    #[allow(non_camel_case_types)]
                    struct GetTopicSvc<T: Publisher>(pub Arc<T>);
                    impl<T: Publisher> tonic::server::UnaryService<super::GetTopicRequest> for GetTopicSvc<T> {
                        type Response = super::Topic;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetTopicRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_topic(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetTopicSvc(inner);
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
                "/google.pubsub.v1.Publisher/ListTopics" => {
                    #[allow(non_camel_case_types)]
                    struct ListTopicsSvc<T: Publisher>(pub Arc<T>);
                    impl<T: Publisher> tonic::server::UnaryService<super::ListTopicsRequest> for ListTopicsSvc<T> {
                        type Response = super::ListTopicsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListTopicsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).list_topics(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ListTopicsSvc(inner);
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
                "/google.pubsub.v1.Publisher/ListTopicSubscriptions" => {
                    #[allow(non_camel_case_types)]
                    struct ListTopicSubscriptionsSvc<T: Publisher>(pub Arc<T>);
                    impl<T: Publisher>
                        tonic::server::UnaryService<super::ListTopicSubscriptionsRequest>
                        for ListTopicSubscriptionsSvc<T>
                    {
                        type Response = super::ListTopicSubscriptionsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListTopicSubscriptionsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).list_topic_subscriptions(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ListTopicSubscriptionsSvc(inner);
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
                "/google.pubsub.v1.Publisher/ListTopicSnapshots" => {
                    #[allow(non_camel_case_types)]
                    struct ListTopicSnapshotsSvc<T: Publisher>(pub Arc<T>);
                    impl<T: Publisher> tonic::server::UnaryService<super::ListTopicSnapshotsRequest>
                        for ListTopicSnapshotsSvc<T>
                    {
                        type Response = super::ListTopicSnapshotsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListTopicSnapshotsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).list_topic_snapshots(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ListTopicSnapshotsSvc(inner);
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
                "/google.pubsub.v1.Publisher/DeleteTopic" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteTopicSvc<T: Publisher>(pub Arc<T>);
                    impl<T: Publisher> tonic::server::UnaryService<super::DeleteTopicRequest> for DeleteTopicSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteTopicRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).delete_topic(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = DeleteTopicSvc(inner);
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
                "/google.pubsub.v1.Publisher/DetachSubscription" => {
                    #[allow(non_camel_case_types)]
                    struct DetachSubscriptionSvc<T: Publisher>(pub Arc<T>);
                    impl<T: Publisher> tonic::server::UnaryService<super::DetachSubscriptionRequest>
                        for DetachSubscriptionSvc<T>
                    {
                        type Response = super::DetachSubscriptionResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DetachSubscriptionRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).detach_subscription(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = DetachSubscriptionSvc(inner);
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
    impl<T: Publisher> Clone for PublisherServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: Publisher> Clone for _Inner<T> {
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
#[doc = r" Generated server implementations."]
pub mod subscriber_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with SubscriberServer."]
    #[async_trait]
    pub trait Subscriber: Send + Sync + 'static {
        #[doc = " Creates a subscription to a given topic. See the"]
        #[doc = " <a href=\"https://cloud.google.com/pubsub/docs/admin#resource_names\">"]
        #[doc = " resource name rules</a>."]
        #[doc = " If the subscription already exists, returns `ALREADY_EXISTS`."]
        #[doc = " If the corresponding topic doesn't exist, returns `NOT_FOUND`."]
        #[doc = ""]
        #[doc = " If the name is not provided in the request, the server will assign a random"]
        #[doc = " name for this subscription on the same project as the topic, conforming"]
        #[doc = " to the"]
        #[doc = " [resource name"]
        #[doc = " format](https://cloud.google.com/pubsub/docs/admin#resource_names). The"]
        #[doc = " generated name is populated in the returned Subscription object. Note that"]
        #[doc = " for REST API requests, you must specify a name in the request."]
        async fn create_subscription(
            &self,
            request: tonic::Request<super::Subscription>,
        ) -> Result<tonic::Response<super::Subscription>, tonic::Status>;
        #[doc = " Gets the configuration details of a subscription."]
        async fn get_subscription(
            &self,
            request: tonic::Request<super::GetSubscriptionRequest>,
        ) -> Result<tonic::Response<super::Subscription>, tonic::Status>;
        #[doc = " Updates an existing subscription. Note that certain properties of a"]
        #[doc = " subscription, such as its topic, are not modifiable."]
        async fn update_subscription(
            &self,
            request: tonic::Request<super::UpdateSubscriptionRequest>,
        ) -> Result<tonic::Response<super::Subscription>, tonic::Status>;
        #[doc = " Lists matching subscriptions."]
        async fn list_subscriptions(
            &self,
            request: tonic::Request<super::ListSubscriptionsRequest>,
        ) -> Result<tonic::Response<super::ListSubscriptionsResponse>, tonic::Status>;
        #[doc = " Deletes an existing subscription. All messages retained in the subscription"]
        #[doc = " are immediately dropped. Calls to `Pull` after deletion will return"]
        #[doc = " `NOT_FOUND`. After a subscription is deleted, a new one may be created with"]
        #[doc = " the same name, but the new one has no association with the old"]
        #[doc = " subscription or its topic unless the same topic is specified."]
        async fn delete_subscription(
            &self,
            request: tonic::Request<super::DeleteSubscriptionRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status>;
        #[doc = " Modifies the ack deadline for a specific message. This method is useful"]
        #[doc = " to indicate that more time is needed to process a message by the"]
        #[doc = " subscriber, or to make the message available for redelivery if the"]
        #[doc = " processing was interrupted. Note that this does not modify the"]
        #[doc = " subscription-level `ackDeadlineSeconds` used for subsequent messages."]
        async fn modify_ack_deadline(
            &self,
            request: tonic::Request<super::ModifyAckDeadlineRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status>;
        #[doc = " Acknowledges the messages associated with the `ack_ids` in the"]
        #[doc = " `AcknowledgeRequest`. The Pub/Sub system can remove the relevant messages"]
        #[doc = " from the subscription."]
        #[doc = ""]
        #[doc = " Acknowledging a message whose ack deadline has expired may succeed,"]
        #[doc = " but such a message may be redelivered later. Acknowledging a message more"]
        #[doc = " than once will not result in an error."]
        async fn acknowledge(
            &self,
            request: tonic::Request<super::AcknowledgeRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status>;
        #[doc = " Pulls messages from the server. The server may return `UNAVAILABLE` if"]
        #[doc = " there are too many concurrent pull requests pending for the given"]
        #[doc = " subscription."]
        async fn pull(
            &self,
            request: tonic::Request<super::PullRequest>,
        ) -> Result<tonic::Response<super::PullResponse>, tonic::Status>;
        #[doc = "Server streaming response type for the StreamingPull method."]
        type StreamingPullStream: Stream<Item = Result<super::StreamingPullResponse, tonic::Status>>
            + Send
            + Sync
            + 'static;
        #[doc = " Establishes a stream with the server, which sends messages down to the"]
        #[doc = " client. The client streams acknowledgements and ack deadline modifications"]
        #[doc = " back to the server. The server will close the stream and return the status"]
        #[doc = " on any error. The server may close the stream with status `UNAVAILABLE` to"]
        #[doc = " reassign server-side resources, in which case, the client should"]
        #[doc = " re-establish the stream. Flow control can be achieved by configuring the"]
        #[doc = " underlying RPC channel."]
        async fn streaming_pull(
            &self,
            request: tonic::Request<tonic::Streaming<super::StreamingPullRequest>>,
        ) -> Result<tonic::Response<Self::StreamingPullStream>, tonic::Status>;
        #[doc = " Modifies the `PushConfig` for a specified subscription."]
        #[doc = ""]
        #[doc = " This may be used to change a push subscription to a pull one (signified by"]
        #[doc = " an empty `PushConfig`) or vice versa, or change the endpoint URL and other"]
        #[doc = " attributes of a push subscription. Messages will accumulate for delivery"]
        #[doc = " continuously through the call regardless of changes to the `PushConfig`."]
        async fn modify_push_config(
            &self,
            request: tonic::Request<super::ModifyPushConfigRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status>;
        #[doc = " Gets the configuration details of a snapshot. Snapshots are used in"]
        #[doc = " <a href=\"https://cloud.google.com/pubsub/docs/replay-overview\">Seek</a>"]
        #[doc = " operations, which allow you to manage message acknowledgments in bulk. That"]
        #[doc = " is, you can set the acknowledgment state of messages in an existing"]
        #[doc = " subscription to the state captured by a snapshot."]
        async fn get_snapshot(
            &self,
            request: tonic::Request<super::GetSnapshotRequest>,
        ) -> Result<tonic::Response<super::Snapshot>, tonic::Status>;
        #[doc = " Lists the existing snapshots. Snapshots are used in"]
        #[doc = " <a href=\"https://cloud.google.com/pubsub/docs/replay-overview\">Seek</a>"]
        #[doc = " operations, which allow"]
        #[doc = " you to manage message acknowledgments in bulk. That is, you can set the"]
        #[doc = " acknowledgment state of messages in an existing subscription to the state"]
        #[doc = " captured by a snapshot."]
        async fn list_snapshots(
            &self,
            request: tonic::Request<super::ListSnapshotsRequest>,
        ) -> Result<tonic::Response<super::ListSnapshotsResponse>, tonic::Status>;
        #[doc = " Creates a snapshot from the requested subscription. Snapshots are used in"]
        #[doc = " <a href=\"https://cloud.google.com/pubsub/docs/replay-overview\">Seek</a>"]
        #[doc = " operations, which allow"]
        #[doc = " you to manage message acknowledgments in bulk. That is, you can set the"]
        #[doc = " acknowledgment state of messages in an existing subscription to the state"]
        #[doc = " captured by a snapshot."]
        #[doc = " <br><br>If the snapshot already exists, returns `ALREADY_EXISTS`."]
        #[doc = " If the requested subscription doesn't exist, returns `NOT_FOUND`."]
        #[doc = " If the backlog in the subscription is too old -- and the resulting snapshot"]
        #[doc = " would expire in less than 1 hour -- then `FAILED_PRECONDITION` is returned."]
        #[doc = " See also the `Snapshot.expire_time` field. If the name is not provided in"]
        #[doc = " the request, the server will assign a random"]
        #[doc = " name for this snapshot on the same project as the subscription, conforming"]
        #[doc = " to the"]
        #[doc = " [resource name"]
        #[doc = " format](https://cloud.google.com/pubsub/docs/admin#resource_names). The"]
        #[doc = " generated name is populated in the returned Snapshot object. Note that for"]
        #[doc = " REST API requests, you must specify a name in the request."]
        async fn create_snapshot(
            &self,
            request: tonic::Request<super::CreateSnapshotRequest>,
        ) -> Result<tonic::Response<super::Snapshot>, tonic::Status>;
        #[doc = " Updates an existing snapshot. Snapshots are used in"]
        #[doc = " <a href=\"https://cloud.google.com/pubsub/docs/replay-overview\">Seek</a>"]
        #[doc = " operations, which allow"]
        #[doc = " you to manage message acknowledgments in bulk. That is, you can set the"]
        #[doc = " acknowledgment state of messages in an existing subscription to the state"]
        #[doc = " captured by a snapshot."]
        async fn update_snapshot(
            &self,
            request: tonic::Request<super::UpdateSnapshotRequest>,
        ) -> Result<tonic::Response<super::Snapshot>, tonic::Status>;
        #[doc = " Removes an existing snapshot. Snapshots are used in"]
        #[doc = " <a href=\"https://cloud.google.com/pubsub/docs/replay-overview\">Seek</a>"]
        #[doc = " operations, which allow"]
        #[doc = " you to manage message acknowledgments in bulk. That is, you can set the"]
        #[doc = " acknowledgment state of messages in an existing subscription to the state"]
        #[doc = " captured by a snapshot.<br><br>"]
        #[doc = " When the snapshot is deleted, all messages retained in the snapshot"]
        #[doc = " are immediately dropped. After a snapshot is deleted, a new one may be"]
        #[doc = " created with the same name, but the new one has no association with the old"]
        #[doc = " snapshot or its subscription, unless the same subscription is specified."]
        async fn delete_snapshot(
            &self,
            request: tonic::Request<super::DeleteSnapshotRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status>;
        #[doc = " Seeks an existing subscription to a point in time or to a given snapshot,"]
        #[doc = " whichever is provided in the request. Snapshots are used in"]
        #[doc = " <a href=\"https://cloud.google.com/pubsub/docs/replay-overview\">Seek</a>"]
        #[doc = " operations, which allow"]
        #[doc = " you to manage message acknowledgments in bulk. That is, you can set the"]
        #[doc = " acknowledgment state of messages in an existing subscription to the state"]
        #[doc = " captured by a snapshot. Note that both the subscription and the snapshot"]
        #[doc = " must be on the same topic."]
        async fn seek(
            &self,
            request: tonic::Request<super::SeekRequest>,
        ) -> Result<tonic::Response<super::SeekResponse>, tonic::Status>;
    }
    #[doc = " The service that an application uses to manipulate subscriptions and to"]
    #[doc = " consume messages from a subscription via the `Pull` method or by"]
    #[doc = " establishing a bi-directional stream using the `StreamingPull` method."]
    #[derive(Debug)]
    pub struct SubscriberServer<T: Subscriber> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: Subscriber> SubscriberServer<T> {
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
    impl<T, B> Service<http::Request<B>> for SubscriberServer<T>
    where
        T: Subscriber,
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
                "/google.pubsub.v1.Subscriber/CreateSubscription" => {
                    #[allow(non_camel_case_types)]
                    struct CreateSubscriptionSvc<T: Subscriber>(pub Arc<T>);
                    impl<T: Subscriber> tonic::server::UnaryService<super::Subscription> for CreateSubscriptionSvc<T> {
                        type Response = super::Subscription;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::Subscription>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).create_subscription(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = CreateSubscriptionSvc(inner);
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
                "/google.pubsub.v1.Subscriber/GetSubscription" => {
                    #[allow(non_camel_case_types)]
                    struct GetSubscriptionSvc<T: Subscriber>(pub Arc<T>);
                    impl<T: Subscriber> tonic::server::UnaryService<super::GetSubscriptionRequest>
                        for GetSubscriptionSvc<T>
                    {
                        type Response = super::Subscription;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetSubscriptionRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_subscription(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetSubscriptionSvc(inner);
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
                "/google.pubsub.v1.Subscriber/UpdateSubscription" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateSubscriptionSvc<T: Subscriber>(pub Arc<T>);
                    impl<T: Subscriber>
                        tonic::server::UnaryService<super::UpdateSubscriptionRequest>
                        for UpdateSubscriptionSvc<T>
                    {
                        type Response = super::Subscription;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateSubscriptionRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).update_subscription(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = UpdateSubscriptionSvc(inner);
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
                "/google.pubsub.v1.Subscriber/ListSubscriptions" => {
                    #[allow(non_camel_case_types)]
                    struct ListSubscriptionsSvc<T: Subscriber>(pub Arc<T>);
                    impl<T: Subscriber> tonic::server::UnaryService<super::ListSubscriptionsRequest>
                        for ListSubscriptionsSvc<T>
                    {
                        type Response = super::ListSubscriptionsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListSubscriptionsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).list_subscriptions(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ListSubscriptionsSvc(inner);
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
                "/google.pubsub.v1.Subscriber/DeleteSubscription" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteSubscriptionSvc<T: Subscriber>(pub Arc<T>);
                    impl<T: Subscriber>
                        tonic::server::UnaryService<super::DeleteSubscriptionRequest>
                        for DeleteSubscriptionSvc<T>
                    {
                        type Response = ();
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteSubscriptionRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).delete_subscription(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = DeleteSubscriptionSvc(inner);
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
                "/google.pubsub.v1.Subscriber/ModifyAckDeadline" => {
                    #[allow(non_camel_case_types)]
                    struct ModifyAckDeadlineSvc<T: Subscriber>(pub Arc<T>);
                    impl<T: Subscriber> tonic::server::UnaryService<super::ModifyAckDeadlineRequest>
                        for ModifyAckDeadlineSvc<T>
                    {
                        type Response = ();
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ModifyAckDeadlineRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).modify_ack_deadline(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ModifyAckDeadlineSvc(inner);
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
                "/google.pubsub.v1.Subscriber/Acknowledge" => {
                    #[allow(non_camel_case_types)]
                    struct AcknowledgeSvc<T: Subscriber>(pub Arc<T>);
                    impl<T: Subscriber> tonic::server::UnaryService<super::AcknowledgeRequest> for AcknowledgeSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AcknowledgeRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).acknowledge(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = AcknowledgeSvc(inner);
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
                "/google.pubsub.v1.Subscriber/Pull" => {
                    #[allow(non_camel_case_types)]
                    struct PullSvc<T: Subscriber>(pub Arc<T>);
                    impl<T: Subscriber> tonic::server::UnaryService<super::PullRequest> for PullSvc<T> {
                        type Response = super::PullResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PullRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).pull(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = PullSvc(inner);
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
                "/google.pubsub.v1.Subscriber/StreamingPull" => {
                    #[allow(non_camel_case_types)]
                    struct StreamingPullSvc<T: Subscriber>(pub Arc<T>);
                    impl<T: Subscriber> tonic::server::StreamingService<super::StreamingPullRequest>
                        for StreamingPullSvc<T>
                    {
                        type Response = super::StreamingPullResponse;
                        type ResponseStream = T::StreamingPullStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<tonic::Streaming<super::StreamingPullRequest>>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).streaming_pull(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1;
                        let inner = inner.0;
                        let method = StreamingPullSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/google.pubsub.v1.Subscriber/ModifyPushConfig" => {
                    #[allow(non_camel_case_types)]
                    struct ModifyPushConfigSvc<T: Subscriber>(pub Arc<T>);
                    impl<T: Subscriber> tonic::server::UnaryService<super::ModifyPushConfigRequest>
                        for ModifyPushConfigSvc<T>
                    {
                        type Response = ();
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ModifyPushConfigRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).modify_push_config(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ModifyPushConfigSvc(inner);
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
                "/google.pubsub.v1.Subscriber/GetSnapshot" => {
                    #[allow(non_camel_case_types)]
                    struct GetSnapshotSvc<T: Subscriber>(pub Arc<T>);
                    impl<T: Subscriber> tonic::server::UnaryService<super::GetSnapshotRequest> for GetSnapshotSvc<T> {
                        type Response = super::Snapshot;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetSnapshotRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_snapshot(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetSnapshotSvc(inner);
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
                "/google.pubsub.v1.Subscriber/ListSnapshots" => {
                    #[allow(non_camel_case_types)]
                    struct ListSnapshotsSvc<T: Subscriber>(pub Arc<T>);
                    impl<T: Subscriber> tonic::server::UnaryService<super::ListSnapshotsRequest>
                        for ListSnapshotsSvc<T>
                    {
                        type Response = super::ListSnapshotsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListSnapshotsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).list_snapshots(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ListSnapshotsSvc(inner);
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
                "/google.pubsub.v1.Subscriber/CreateSnapshot" => {
                    #[allow(non_camel_case_types)]
                    struct CreateSnapshotSvc<T: Subscriber>(pub Arc<T>);
                    impl<T: Subscriber> tonic::server::UnaryService<super::CreateSnapshotRequest>
                        for CreateSnapshotSvc<T>
                    {
                        type Response = super::Snapshot;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateSnapshotRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).create_snapshot(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = CreateSnapshotSvc(inner);
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
                "/google.pubsub.v1.Subscriber/UpdateSnapshot" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateSnapshotSvc<T: Subscriber>(pub Arc<T>);
                    impl<T: Subscriber> tonic::server::UnaryService<super::UpdateSnapshotRequest>
                        for UpdateSnapshotSvc<T>
                    {
                        type Response = super::Snapshot;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateSnapshotRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).update_snapshot(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = UpdateSnapshotSvc(inner);
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
                "/google.pubsub.v1.Subscriber/DeleteSnapshot" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteSnapshotSvc<T: Subscriber>(pub Arc<T>);
                    impl<T: Subscriber> tonic::server::UnaryService<super::DeleteSnapshotRequest>
                        for DeleteSnapshotSvc<T>
                    {
                        type Response = ();
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteSnapshotRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).delete_snapshot(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = DeleteSnapshotSvc(inner);
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
                "/google.pubsub.v1.Subscriber/Seek" => {
                    #[allow(non_camel_case_types)]
                    struct SeekSvc<T: Subscriber>(pub Arc<T>);
                    impl<T: Subscriber> tonic::server::UnaryService<super::SeekRequest> for SeekSvc<T> {
                        type Response = super::SeekResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SeekRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).seek(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = SeekSvc(inner);
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
    impl<T: Subscriber> Clone for SubscriberServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: Subscriber> Clone for _Inner<T> {
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
