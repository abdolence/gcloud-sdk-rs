/// A representation of the trigger resource.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Trigger {
    /// Required. The resource name of the trigger. Must be unique within the
    /// location on the project and must be in
    /// `projects/{project}/locations/{location}/triggers/{trigger}` format.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. Server assigned unique identifier for the trigger. The value
    /// is a UUID4 string and guaranteed to remain unchanged until the resource is
    /// deleted.
    #[prost(string, tag = "2")]
    pub uid: ::prost::alloc::string::String,
    /// Output only. The creation time.
    #[prost(message, optional, tag = "5")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The last-modified time.
    #[prost(message, optional, tag = "6")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Required. null The list of filters that applies to event attributes. Only
    /// events that match all the provided filters will be sent to the destination.
    #[prost(message, repeated, tag = "8")]
    pub event_filters: ::prost::alloc::vec::Vec<EventFilter>,
    /// Optional. The IAM service account email associated with the trigger. The
    /// service account represents the identity of the trigger.
    ///
    /// The principal who calls this API must have `iam.serviceAccounts.actAs`
    /// permission in the service account. See
    /// <https://cloud.google.com/iam/docs/understanding-service-accounts?hl=en#sa_common>
    /// for more information.
    ///
    /// For Cloud Run destinations, this service account is used to generate
    /// identity tokens when invoking the service. See
    /// <https://cloud.google.com/run/docs/triggering/pubsub-push#create-service-account>
    /// for information on how to invoke authenticated Cloud Run services.
    /// In order to create Audit Log triggers, the service account should also
    /// have `roles/eventarc.eventReceiver` IAM role.
    #[prost(string, tag = "9")]
    pub service_account: ::prost::alloc::string::String,
    /// Required. Destination specifies where the events should be sent to.
    #[prost(message, optional, tag = "10")]
    pub destination: ::core::option::Option<Destination>,
    /// Optional. In order to deliver messages, Eventarc may use other GCP
    /// products as transport intermediary. This field contains a reference to that
    /// transport intermediary. This information can be used for debugging
    /// purposes.
    #[prost(message, optional, tag = "11")]
    pub transport: ::core::option::Option<Transport>,
    /// Optional. User labels attached to the triggers that can be used to group
    /// resources.
    #[prost(map = "string, string", tag = "12")]
    pub labels:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Output only. This checksum is computed by the server based on the value of
    /// other fields, and may be sent only on create requests to ensure the client
    /// has an up-to-date value before proceeding.
    #[prost(string, tag = "99")]
    pub etag: ::prost::alloc::string::String,
}
/// Filters events based on exact matches on the CloudEvents attributes.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventFilter {
    /// Required. The name of a CloudEvents attribute. Currently, only a subset of
    /// attributes are supported for filtering.
    ///
    /// All triggers MUST provide a filter for the 'type' attribute.
    #[prost(string, tag = "1")]
    pub attribute: ::prost::alloc::string::String,
    /// Required. The value for the attribute.
    #[prost(string, tag = "2")]
    pub value: ::prost::alloc::string::String,
}
/// Represents a target of an invocation over HTTP.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Destination {
    #[prost(oneof = "destination::Descriptor", tags = "1")]
    pub descriptor: ::core::option::Option<destination::Descriptor>,
}
/// Nested message and enum types in `Destination`.
pub mod destination {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Descriptor {
        /// Cloud Run fully-managed service that receives the events. The service
        /// should be running in the same project of the trigger.
        #[prost(message, tag = "1")]
        CloudRun(super::CloudRun),
    }
}
/// Represents the transport intermediaries created for the trigger in order to
/// deliver events.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Transport {
    #[prost(oneof = "transport::Intermediary", tags = "1")]
    pub intermediary: ::core::option::Option<transport::Intermediary>,
}
/// Nested message and enum types in `Transport`.
pub mod transport {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Intermediary {
        /// The Pub/Sub topic and subscription used by Eventarc as delivery
        /// intermediary.
        #[prost(message, tag = "1")]
        Pubsub(super::Pubsub),
    }
}
/// Represents a Cloud Run destination.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CloudRun {
    /// Required. The name of the Cloud Run service being addressed. See
    /// <https://cloud.google.com/run/docs/reference/rest/v1/namespaces.services.>
    ///
    /// Only services located in the same project of the trigger object
    /// can be addressed.
    #[prost(string, tag = "1")]
    pub service: ::prost::alloc::string::String,
    /// Optional. The relative path on the Cloud Run service the events should be
    /// sent to.
    ///
    /// The value must conform to the definition of URI path segment (section 3.3
    /// of RFC2396). Examples: "/route", "route", "route/subroute".
    #[prost(string, tag = "2")]
    pub path: ::prost::alloc::string::String,
    /// Required. The region the Cloud Run service is deployed in.
    #[prost(string, tag = "3")]
    pub region: ::prost::alloc::string::String,
}
/// Represents a Pub/Sub transport.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Pubsub {
    /// Optional. The name of the Pub/Sub topic created and managed by Eventarc
    /// system as a transport for the event delivery. Format:
    /// `projects/{PROJECT_ID}/topics/{TOPIC_NAME}`.
    ///
    /// You may set an existing topic for triggers of the type
    /// `google.cloud.pubsub.topic.v1.messagePublished` only. The topic you provide
    /// here will not be deleted by Eventarc at trigger deletion.
    #[prost(string, tag = "1")]
    pub topic: ::prost::alloc::string::String,
    /// Output only. The name of the Pub/Sub subscription created and managed by
    /// Eventarc system as a transport for the event delivery. Format:
    /// `projects/{PROJECT_ID}/subscriptions/{SUBSCRIPTION_NAME}`.
    #[prost(string, tag = "2")]
    pub subscription: ::prost::alloc::string::String,
}
/// The request message for the GetTrigger method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTriggerRequest {
    /// Required. The name of the trigger to get.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// The request message for the ListTriggers method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTriggersRequest {
    /// Required. The parent collection to list triggers on.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of triggers to return on each page.
    /// Note: The service may send fewer.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The page token; provide the value from the `next_page_token` field in a
    /// previous `ListTriggers` call to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to `ListTriggers` must match
    /// the call that provided the page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// The sorting order of the resources returned. Value should be a comma
    /// separated list of fields. The default sorting oder is ascending. To specify
    /// descending order for a field, append a ` desc` suffix; for example:
    /// `name desc, trigger_id`.
    #[prost(string, tag = "4")]
    pub order_by: ::prost::alloc::string::String,
}
/// The response message for the ListTriggers method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTriggersResponse {
    /// The requested triggers, up to the number specified in `page_size`.
    #[prost(message, repeated, tag = "1")]
    pub triggers: ::prost::alloc::vec::Vec<Trigger>,
    /// A page token that can be sent to ListTriggers to request the next page.
    /// If this is empty, then there are no more pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Unreachable resources, if any.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// The request message for the CreateTrigger method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateTriggerRequest {
    /// Required. The parent collection in which to add this trigger.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The trigger to create.
    #[prost(message, optional, tag = "2")]
    pub trigger: ::core::option::Option<Trigger>,
    /// Required. The user-provided ID to be assigned to the trigger.
    #[prost(string, tag = "3")]
    pub trigger_id: ::prost::alloc::string::String,
    /// Required. If set, validate the request and preview the review, but do not actually
    /// post it.
    #[prost(bool, tag = "4")]
    pub validate_only: bool,
}
/// The request message for the UpdateTrigger method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateTriggerRequest {
    /// The trigger to be updated.
    #[prost(message, optional, tag = "1")]
    pub trigger: ::core::option::Option<Trigger>,
    /// The fields to be updated; only fields explicitly provided will be updated.
    /// If no field mask is provided, all provided fields in the request will be
    /// updated. To update all fields, provide a field mask of "*".
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// If set to true, and the trigger is not found, a new trigger will be
    /// created. In this situation, `update_mask` is ignored.
    #[prost(bool, tag = "3")]
    pub allow_missing: bool,
    /// Required. If set, validate the request and preview the review, but do not actually
    /// post it.
    #[prost(bool, tag = "4")]
    pub validate_only: bool,
}
/// The request message for the DeleteTrigger method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteTriggerRequest {
    /// Required. The name of the trigger to be deleted.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// If provided, the trigger will only be deleted if the etag matches the
    /// current etag on the resource.
    #[prost(string, tag = "2")]
    pub etag: ::prost::alloc::string::String,
    /// If set to true, and the trigger is not found, the request will succeed
    /// but no action will be taken on the server.
    #[prost(bool, tag = "3")]
    pub allow_missing: bool,
    /// Required. If set, validate the request and preview the review, but do not actually
    /// post it.
    #[prost(bool, tag = "4")]
    pub validate_only: bool,
}
/// Represents the metadata of the long-running operation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationMetadata {
    /// Output only. The time the operation was created.
    #[prost(message, optional, tag = "1")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The time the operation finished running.
    #[prost(message, optional, tag = "2")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Server-defined resource path for the target of the operation.
    #[prost(string, tag = "3")]
    pub target: ::prost::alloc::string::String,
    /// Output only. Name of the verb executed by the operation.
    #[prost(string, tag = "4")]
    pub verb: ::prost::alloc::string::String,
    /// Output only. Human-readable status of the operation, if any.
    #[prost(string, tag = "5")]
    pub status_message: ::prost::alloc::string::String,
    /// Output only. Identifies whether the user has requested cancellation
    /// of the operation. Operations that have successfully been cancelled
    /// have \[Operation.error][\] value with a \[google.rpc.Status.code][google.rpc.Status.code\] of 1,
    /// corresponding to `Code.CANCELLED`.
    #[prost(bool, tag = "6")]
    pub requested_cancellation: bool,
    /// Output only. API version used to start the operation.
    #[prost(string, tag = "7")]
    pub api_version: ::prost::alloc::string::String,
}
#[doc = r" Generated client implementations."]
pub mod eventarc_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Eventarc allows users to subscribe to various events that are provided by"]
    #[doc = " Google Cloud services and forward them to supported destinations."]
    #[derive(Debug, Clone)]
    pub struct EventarcClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> EventarcClient<T>
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
        ) -> EventarcClient<InterceptedService<T, F>>
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
            EventarcClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Get a single trigger."]
        pub async fn get_trigger(
            &mut self,
            request: impl tonic::IntoRequest<super::GetTriggerRequest>,
        ) -> Result<tonic::Response<super::Trigger>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.eventarc.v1.Eventarc/GetTrigger",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " List triggers."]
        pub async fn list_triggers(
            &mut self,
            request: impl tonic::IntoRequest<super::ListTriggersRequest>,
        ) -> Result<tonic::Response<super::ListTriggersResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.eventarc.v1.Eventarc/ListTriggers",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Create a new trigger in a particular project and location."]
        pub async fn create_trigger(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateTriggerRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.eventarc.v1.Eventarc/CreateTrigger",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Update a single trigger."]
        pub async fn update_trigger(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateTriggerRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.eventarc.v1.Eventarc/UpdateTrigger",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Delete a single trigger."]
        pub async fn delete_trigger(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteTriggerRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.eventarc.v1.Eventarc/DeleteTrigger",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
