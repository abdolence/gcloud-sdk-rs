/// An alert affecting a customer.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Alert {
    /// Output only. The unique identifier of the Google account of the customer.
    #[prost(string, tag = "1")]
    pub customer_id: ::prost::alloc::string::String,
    /// Output only. The unique identifier for the alert.
    #[prost(string, tag = "2")]
    pub alert_id: ::prost::alloc::string::String,
    /// Output only. The time this alert was created.
    #[prost(message, optional, tag = "3")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Required. The time the event that caused this alert was started or
    /// detected.
    #[prost(message, optional, tag = "4")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Optional. The time the event that caused this alert ceased being active.
    /// If provided, the end time must not be earlier than the start time.
    /// If not provided, it indicates an ongoing alert.
    #[prost(message, optional, tag = "5")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Required. The type of the alert.
    /// This is output only after alert is created.
    /// For a list of available alert types see
    /// [Google Workspace Alert
    /// types](<https://developers.google.com/admin-sdk/alertcenter/reference/alert-types>).
    #[prost(string, tag = "6")]
    pub r#type: ::prost::alloc::string::String,
    /// Required. A unique identifier for the system that reported the alert.
    /// This is output only after alert is created.
    ///
    /// Supported sources are any of the following:
    ///
    /// * Google Operations
    /// * Mobile device management
    /// * Gmail phishing
    /// * Domain wide takeout
    /// * State sponsored attack
    /// * Google identity
    #[prost(string, tag = "7")]
    pub source: ::prost::alloc::string::String,
    /// Optional. The data associated with this alert, for example
    /// \[google.apps.alertcenter.type.DeviceCompromised\] \[google.apps.alertcenter.type.DeviceCompromised\].
    #[prost(message, optional, tag = "8")]
    pub data: ::core::option::Option<::prost_types::Any>,
    /// Output only. An optional
    /// [Security Investigation Tool](<https://support.google.com/a/answer/7575955>)
    /// query for this alert.
    #[prost(string, tag = "9")]
    pub security_investigation_tool_link: ::prost::alloc::string::String,
    /// Output only. `True` if this alert is marked for deletion.
    #[prost(bool, tag = "11")]
    pub deleted: bool,
    /// Output only. The metadata associated with this alert.
    #[prost(message, optional, tag = "12")]
    pub metadata: ::core::option::Option<AlertMetadata>,
    /// Output only. The time this alert was last updated.
    #[prost(message, optional, tag = "13")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Optional. `etag` is used for optimistic concurrency control as a way to help
    /// prevent simultaneous updates of an alert from overwriting each other.
    /// It is strongly suggested that systems make use of the `etag` in the
    /// read-modify-write cycle to perform alert updates in order to avoid race
    /// conditions: An `etag` is returned in the response which contains alerts,
    /// and systems are expected to put that etag in the request to update alert to
    /// ensure that their change will be applied to the same version of the alert.
    ///
    /// If no `etag` is provided in the call to update alert, then the existing
    /// alert is overwritten blindly.
    #[prost(string, tag = "14")]
    pub etag: ::prost::alloc::string::String,
}
/// A customer feedback about an alert.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AlertFeedback {
    /// Output only. The unique identifier of the Google account of the customer.
    #[prost(string, tag = "1")]
    pub customer_id: ::prost::alloc::string::String,
    /// Output only. The alert identifier.
    #[prost(string, tag = "2")]
    pub alert_id: ::prost::alloc::string::String,
    /// Output only. The unique identifier for the feedback.
    #[prost(string, tag = "3")]
    pub feedback_id: ::prost::alloc::string::String,
    /// Output only. The time this feedback was created.
    #[prost(message, optional, tag = "4")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Required. The type of the feedback.
    #[prost(enumeration = "AlertFeedbackType", tag = "5")]
    pub r#type: i32,
    /// Output only. The email of the user that provided the feedback.
    #[prost(string, tag = "6")]
    pub email: ::prost::alloc::string::String,
}
/// An alert metadata.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AlertMetadata {
    /// Output only. The unique identifier of the Google account of the customer.
    #[prost(string, tag = "1")]
    pub customer_id: ::prost::alloc::string::String,
    /// Output only. The alert identifier.
    #[prost(string, tag = "2")]
    pub alert_id: ::prost::alloc::string::String,
    /// The current status of the alert.
    /// The supported values are the following:
    ///
    /// * NOT_STARTED
    /// * IN_PROGRESS
    /// * CLOSED
    #[prost(string, tag = "4")]
    pub status: ::prost::alloc::string::String,
    /// The email address of the user assigned to the alert.
    #[prost(string, tag = "5")]
    pub assignee: ::prost::alloc::string::String,
    /// Output only. The time this metadata was last updated.
    #[prost(message, optional, tag = "6")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The severity value of the alert. Alert Center will set this field at alert
    /// creation time, default's to an empty string when it could not be
    /// determined.
    /// The supported values for update actions on this field are the following:
    ///
    /// * HIGH
    /// * MEDIUM
    /// * LOW
    #[prost(string, tag = "7")]
    pub severity: ::prost::alloc::string::String,
    /// Optional. `etag` is used for optimistic concurrency control as a way to
    /// help prevent simultaneous updates of an alert metadata from overwriting
    /// each other. It is strongly suggested that systems make use of the `etag` in
    /// the read-modify-write cycle to perform metatdata updates in order to avoid
    /// race conditions: An `etag` is returned in the response which contains alert
    /// metadata, and systems are expected to put that etag in the request to
    /// update alert metadata to ensure that their change will be applied to the
    /// same version of the alert metadata.
    ///
    /// If no `etag` is provided in the call to update alert metadata, then the
    /// existing alert metadata is overwritten blindly.
    #[prost(string, tag = "8")]
    pub etag: ::prost::alloc::string::String,
}
/// Customer-level settings.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Settings {
    /// The list of notifications.
    #[prost(message, repeated, tag = "1")]
    pub notifications: ::prost::alloc::vec::Vec<settings::Notification>,
}
/// Nested message and enum types in `Settings`.
pub mod settings {
    /// Settings for callback notifications.
    /// For more details see [Google Workspace Alert
    /// Notification](<https://developers.google.com/admin-sdk/alertcenter/guides/notifications>).
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Notification {
        /// Exactly one destination to be specified.
        #[prost(oneof = "notification::Destination", tags = "1")]
        pub destination: ::core::option::Option<notification::Destination>,
    }
    /// Nested message and enum types in `Notification`.
    pub mod notification {
        /// A reference to a Cloud Pubsub topic.
        ///
        /// To register for notifications, the owner of the topic must grant
        /// `alerts-api-push-notifications@system.gserviceaccount.com` the
        ///  `projects.topics.publish` permission.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct CloudPubsubTopic {
            /// The `name` field of a Cloud Pubsub \[Topic\]
            /// (<https://cloud.google.com/pubsub/docs/reference/rest/v1/projects.topics#Topic>).
            #[prost(string, tag = "1")]
            pub topic_name: ::prost::alloc::string::String,
            /// Optional. The format of the payload that would be sent.
            /// If not specified the format will be JSON.
            #[prost(enumeration = "PayloadFormat", tag = "2")]
            pub payload_format: i32,
        }
        /// The format of the payload.
        #[derive(
            Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration,
        )]
        #[repr(i32)]
        pub enum PayloadFormat {
            /// Payload format is not specified (will use JSON as default).
            Unspecified = 0,
            /// Use JSON.
            Json = 1,
        }
        /// Exactly one destination to be specified.
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Destination {
            /// A Google Cloud Pub/sub topic destination.
            #[prost(message, tag = "1")]
            CloudPubsubTopic(CloudPubsubTopic),
        }
    }
}
/// A request to perform batch delete on alerts.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchDeleteAlertsRequest {
    /// Optional. The unique identifier of the Google Workspace organization
    /// account of the customer the alerts are associated with.
    #[prost(string, tag = "1")]
    pub customer_id: ::prost::alloc::string::String,
    /// Required. list of alert IDs.
    #[prost(string, repeated, tag = "2")]
    pub alert_id: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Response to batch delete operation on alerts.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchDeleteAlertsResponse {
    /// The successful list of alert IDs.
    #[prost(string, repeated, tag = "1")]
    pub success_alert_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The status details for each failed alert_id.
    #[prost(map = "string, message", tag = "2")]
    pub failed_alert_status: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        super::super::super::rpc::Status,
    >,
}
/// A request to perform batch undelete on alerts.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchUndeleteAlertsRequest {
    /// Optional. The unique identifier of the Google Workspace organization
    /// account of the customer the alerts are associated with.
    #[prost(string, tag = "1")]
    pub customer_id: ::prost::alloc::string::String,
    /// Required. list of alert IDs.
    #[prost(string, repeated, tag = "2")]
    pub alert_id: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Response to batch undelete operation on alerts.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchUndeleteAlertsResponse {
    /// The successful list of alert IDs.
    #[prost(string, repeated, tag = "1")]
    pub success_alert_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The status details for each failed alert_id.
    #[prost(map = "string, message", tag = "2")]
    pub failed_alert_status: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        super::super::super::rpc::Status,
    >,
}
/// An alert listing request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAlertsRequest {
    /// Optional. The unique identifier of the Google Workspace organization
    /// account of the customer the alerts are associated with.
    /// Inferred from the caller identity if not provided.
    #[prost(string, tag = "1")]
    pub customer_id: ::prost::alloc::string::String,
    /// Optional. The requested page size. Server may return fewer items than
    /// requested. If unspecified, server picks an appropriate default.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. A token identifying a page of results the server should return.
    /// If empty, a new iteration is started. To continue an iteration, pass in
    /// the value from the previous ListAlertsResponse's
    /// \[next_page_token][google.apps.alertcenter.v1beta1.ListAlertsResponse.next_page_token\] field.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. A query string for filtering alert results.
    /// For more details, see [Query
    /// filters](<https://developers.google.com/admin-sdk/alertcenter/guides/query-filters>) and [Supported
    /// query filter
    /// fields](<https://developers.google.com/admin-sdk/alertcenter/reference/filter-fields#alerts.list>).
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Optional. The sort order of the list results.
    /// If not specified results may be returned in arbitrary order.
    /// You can sort the results in descending order based on the creation
    /// timestamp using `order_by="create_time desc"`.
    /// Currently, supported sorting are `create_time asc`, `create_time desc`,
    /// `update_time desc`
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
/// Response message for an alert listing request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAlertsResponse {
    /// The list of alerts.
    #[prost(message, repeated, tag = "1")]
    pub alerts: ::prost::alloc::vec::Vec<Alert>,
    /// The token for the next page. If not empty, indicates that there may be more
    /// alerts that match the listing request; this value can be used in a
    /// subsequent \[ListAlertsRequest][google.apps.alertcenter.v1beta1.ListAlertsRequest\] to get alerts continuing from last result
    /// of the current list call.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request for a specific alert.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAlertRequest {
    /// Optional. The unique identifier of the Google Workspace organization
    /// account of the customer the alert is associated with.
    /// Inferred from the caller identity if not provided.
    #[prost(string, tag = "1")]
    pub customer_id: ::prost::alloc::string::String,
    /// Required. The identifier of the alert to retrieve.
    #[prost(string, tag = "2")]
    pub alert_id: ::prost::alloc::string::String,
}
/// A request to mark a specific alert for deletion.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteAlertRequest {
    /// Optional. The unique identifier of the Google Workspace organization
    /// account of the customer the alert is associated with.
    /// Inferred from the caller identity if not provided.
    #[prost(string, tag = "1")]
    pub customer_id: ::prost::alloc::string::String,
    /// Required. The identifier of the alert to delete.
    #[prost(string, tag = "2")]
    pub alert_id: ::prost::alloc::string::String,
}
/// A request to undelete a specific alert that was marked for deletion.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UndeleteAlertRequest {
    /// Optional. The unique identifier of the Google Workspace organization
    /// account of the customer the alert is associated with.
    /// Inferred from the caller identity if not provided.
    #[prost(string, tag = "1")]
    pub customer_id: ::prost::alloc::string::String,
    /// Required. The identifier of the alert to undelete.
    #[prost(string, tag = "2")]
    pub alert_id: ::prost::alloc::string::String,
}
/// A request to create a new alert feedback.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateAlertFeedbackRequest {
    /// Optional. The unique identifier of the Google Workspace organization
    /// account of the customer the alert is associated with.
    /// Inferred from the caller identity if not provided.
    #[prost(string, tag = "1")]
    pub customer_id: ::prost::alloc::string::String,
    /// Required. The identifier of the alert this feedback belongs to.
    #[prost(string, tag = "2")]
    pub alert_id: ::prost::alloc::string::String,
    /// Required. The new alert feedback to create.
    #[prost(message, optional, tag = "3")]
    pub feedback: ::core::option::Option<AlertFeedback>,
}
/// An alert feedback listing request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAlertFeedbackRequest {
    /// Optional. The unique identifier of the Google Workspace organization
    /// account of the customer the alert feedback are associated with.
    /// Inferred from the caller identity if not provided.
    #[prost(string, tag = "1")]
    pub customer_id: ::prost::alloc::string::String,
    /// Required. The alert identifier.
    /// The "-" wildcard could be used to represent all alerts.
    #[prost(string, tag = "2")]
    pub alert_id: ::prost::alloc::string::String,
    /// Optional. A query string for filtering alert feedback results.
    /// For more details, see [Query
    /// filters](<https://developers.google.com/admin-sdk/alertcenter/guides/query-filters>) and [Supported
    /// query filter
    /// fields](<https://developers.google.com/admin-sdk/alertcenter/reference/filter-fields#alerts.feedback.list>).
    #[prost(string, tag = "3")]
    pub filter: ::prost::alloc::string::String,
}
/// Response message for an alert feedback listing request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAlertFeedbackResponse {
    /// The list of alert feedback.
    /// Feedback entries for each alert are ordered by creation time descending.
    #[prost(message, repeated, tag = "1")]
    pub feedback: ::prost::alloc::vec::Vec<AlertFeedback>,
}
/// Get the alert metadata.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAlertMetadataRequest {
    /// Optional. The unique identifier of the Google Workspace organization
    /// account of the customer the alert metadata is associated with.
    /// Inferred from the caller identity if not provided.
    #[prost(string, tag = "1")]
    pub customer_id: ::prost::alloc::string::String,
    /// Required. The identifier of the alert this metadata belongs to.
    #[prost(string, tag = "2")]
    pub alert_id: ::prost::alloc::string::String,
}
/// Get the customer level settings.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSettingsRequest {
    /// Optional. The unique identifier of the Google Workspace organization
    /// account of the customer the alert settings are associated with.
    /// Inferred from the caller identity if not provided.
    #[prost(string, tag = "1")]
    pub customer_id: ::prost::alloc::string::String,
}
/// Update the customer level settings.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateSettingsRequest {
    /// Optional. The unique identifier of the Google Workspace organization
    /// account of the customer the alert settings are associated with.
    /// Inferred from the caller identity if not provided.
    #[prost(string, tag = "1")]
    pub customer_id: ::prost::alloc::string::String,
    /// The customer settings to update.
    #[prost(message, optional, tag = "2")]
    pub settings: ::core::option::Option<Settings>,
}
/// The type of alert feedback.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AlertFeedbackType {
    /// The feedback type is not specified.
    Unspecified = 0,
    /// The alert report is not useful.
    NotUseful = 1,
    /// The alert report is somewhat useful.
    SomewhatUseful = 2,
    /// The alert report is very useful.
    VeryUseful = 3,
}
#[doc = r" Generated client implementations."]
pub mod alert_center_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Google Workspace Alert Center API (beta)."]
    #[derive(Debug, Clone)]
    pub struct AlertCenterServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> AlertCenterServiceClient<T>
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
        ) -> AlertCenterServiceClient<InterceptedService<T, F>>
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
            AlertCenterServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Lists the alerts."]
        pub async fn list_alerts(
            &mut self,
            request: impl tonic::IntoRequest<super::ListAlertsRequest>,
        ) -> Result<tonic::Response<super::ListAlertsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.apps.alertcenter.v1beta1.AlertCenterService/ListAlerts",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets the specified alert. Attempting to get a nonexistent alert returns"]
        #[doc = " `NOT_FOUND` error."]
        pub async fn get_alert(
            &mut self,
            request: impl tonic::IntoRequest<super::GetAlertRequest>,
        ) -> Result<tonic::Response<super::Alert>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.apps.alertcenter.v1beta1.AlertCenterService/GetAlert",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Marks the specified alert for deletion. An alert that has been marked for"]
        #[doc = " deletion is removed from Alert Center after 30 days."]
        #[doc = " Marking an alert for deletion has no effect on an alert which has"]
        #[doc = " already been marked for deletion. Attempting to mark a nonexistent alert"]
        #[doc = " for deletion results in a `NOT_FOUND` error."]
        pub async fn delete_alert(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteAlertRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.apps.alertcenter.v1beta1.AlertCenterService/DeleteAlert",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Restores, or \"undeletes\", an alert that was marked for deletion within the"]
        #[doc = " past 30 days. Attempting to undelete an alert which was marked for deletion"]
        #[doc = " over 30 days ago (which has been removed from the Alert Center database) or"]
        #[doc = " a nonexistent alert returns a `NOT_FOUND` error. Attempting to"]
        #[doc = " undelete an alert which has not been marked for deletion has no effect."]
        pub async fn undelete_alert(
            &mut self,
            request: impl tonic::IntoRequest<super::UndeleteAlertRequest>,
        ) -> Result<tonic::Response<super::Alert>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.apps.alertcenter.v1beta1.AlertCenterService/UndeleteAlert",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates new feedback for an alert. Attempting to create a feedback for"]
        #[doc = " a non-existent alert returns `NOT_FOUND` error. Attempting to create a"]
        #[doc = " feedback for an alert that is marked for deletion returns"]
        #[doc = " `FAILED_PRECONDITION' error."]
        pub async fn create_alert_feedback(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateAlertFeedbackRequest>,
        ) -> Result<tonic::Response<super::AlertFeedback>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.apps.alertcenter.v1beta1.AlertCenterService/CreateAlertFeedback",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists all the feedback for an alert. Attempting to list feedbacks for"]
        #[doc = " a non-existent alert returns `NOT_FOUND` error."]
        pub async fn list_alert_feedback(
            &mut self,
            request: impl tonic::IntoRequest<super::ListAlertFeedbackRequest>,
        ) -> Result<tonic::Response<super::ListAlertFeedbackResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.apps.alertcenter.v1beta1.AlertCenterService/ListAlertFeedback",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns the metadata of an alert. Attempting to get metadata for"]
        #[doc = " a non-existent alert returns `NOT_FOUND` error."]
        pub async fn get_alert_metadata(
            &mut self,
            request: impl tonic::IntoRequest<super::GetAlertMetadataRequest>,
        ) -> Result<tonic::Response<super::AlertMetadata>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.apps.alertcenter.v1beta1.AlertCenterService/GetAlertMetadata",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns customer-level settings."]
        pub async fn get_settings(
            &mut self,
            request: impl tonic::IntoRequest<super::GetSettingsRequest>,
        ) -> Result<tonic::Response<super::Settings>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.apps.alertcenter.v1beta1.AlertCenterService/GetSettings",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates the customer-level settings."]
        pub async fn update_settings(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateSettingsRequest>,
        ) -> Result<tonic::Response<super::Settings>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.apps.alertcenter.v1beta1.AlertCenterService/UpdateSettings",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Performs batch delete operation on alerts."]
        pub async fn batch_delete_alerts(
            &mut self,
            request: impl tonic::IntoRequest<super::BatchDeleteAlertsRequest>,
        ) -> Result<tonic::Response<super::BatchDeleteAlertsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.apps.alertcenter.v1beta1.AlertCenterService/BatchDeleteAlerts",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Performs batch undelete operation on alerts."]
        pub async fn batch_undelete_alerts(
            &mut self,
            request: impl tonic::IntoRequest<super::BatchUndeleteAlertsRequest>,
        ) -> Result<tonic::Response<super::BatchUndeleteAlertsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.apps.alertcenter.v1beta1.AlertCenterService/BatchUndeleteAlerts",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
