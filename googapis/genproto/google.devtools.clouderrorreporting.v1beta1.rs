/// Description of a group of similar error events.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ErrorGroup {
    /// The group resource name.
    /// Example: <code>projects/my-project-123/groups/CNSgkpnppqKCUw</code>
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Group IDs are unique for a given project. If the same kind of error
    /// occurs in different service contexts, it will receive the same group ID.
    #[prost(string, tag = "2")]
    pub group_id: ::prost::alloc::string::String,
    /// Associated tracking issues.
    #[prost(message, repeated, tag = "3")]
    pub tracking_issues: ::prost::alloc::vec::Vec<TrackingIssue>,
    /// Error group's resolution status.
    /// An unspecified resolution status will be interpreted as OPEN
    #[prost(enumeration = "ResolutionStatus", tag = "5")]
    pub resolution_status: i32,
}
/// Information related to tracking the progress on resolving the error.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TrackingIssue {
    /// A URL pointing to a related entry in an issue tracking system.
    /// Example: `<https://github.com/user/project/issues/4`>
    #[prost(string, tag = "1")]
    pub url: ::prost::alloc::string::String,
}
/// An error event which is returned by the Error Reporting system.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ErrorEvent {
    /// Time when the event occurred as provided in the error report.
    /// If the report did not contain a timestamp, the time the error was received
    /// by the Error Reporting system is used.
    #[prost(message, optional, tag = "1")]
    pub event_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The `ServiceContext` for which this error was reported.
    #[prost(message, optional, tag = "2")]
    pub service_context: ::core::option::Option<ServiceContext>,
    /// The stack trace that was reported or logged by the service.
    #[prost(string, tag = "3")]
    pub message: ::prost::alloc::string::String,
    /// Data about the context in which the error occurred.
    #[prost(message, optional, tag = "5")]
    pub context: ::core::option::Option<ErrorContext>,
}
/// Describes a running service that sends errors.
/// Its version changes over time and multiple versions can run in parallel.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServiceContext {
    /// An identifier of the service, such as the name of the
    /// executable, job, or Google App Engine service name. This field is expected
    /// to have a low number of values that are relatively stable over time, as
    /// opposed to `version`, which can be changed whenever new code is deployed.
    ///
    /// Contains the service name for error reports extracted from Google
    /// App Engine logs or `default` if the App Engine default service is used.
    #[prost(string, tag = "2")]
    pub service: ::prost::alloc::string::String,
    /// Represents the source code version that the developer provided,
    /// which could represent a version label or a Git SHA-1 hash, for example.
    /// For App Engine standard environment, the version is set to the version of
    /// the app.
    #[prost(string, tag = "3")]
    pub version: ::prost::alloc::string::String,
    /// Type of the MonitoredResource. List of possible values:
    /// <https://cloud.google.com/monitoring/api/resources>
    ///
    /// Value is set automatically for incoming errors and must not be set when
    /// reporting errors.
    #[prost(string, tag = "4")]
    pub resource_type: ::prost::alloc::string::String,
}
/// A description of the context in which an error occurred.
/// This data should be provided by the application when reporting an error,
/// unless the
/// error report has been generated automatically from Google App Engine logs.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ErrorContext {
    /// The HTTP request which was processed when the error was
    /// triggered.
    #[prost(message, optional, tag = "1")]
    pub http_request: ::core::option::Option<HttpRequestContext>,
    /// The user who caused or was affected by the crash.
    /// This can be a user ID, an email address, or an arbitrary token that
    /// uniquely identifies the user.
    /// When sending an error report, leave this field empty if the user was not
    /// logged in. In this case the
    /// Error Reporting system will use other data, such as remote IP address, to
    /// distinguish affected users. See `affected_users_count` in
    /// `ErrorGroupStats`.
    #[prost(string, tag = "2")]
    pub user: ::prost::alloc::string::String,
    /// The location in the source code where the decision was made to
    /// report the error, usually the place where it was logged.
    /// For a logged exception this would be the source line where the
    /// exception is logged, usually close to the place where it was
    /// caught.
    #[prost(message, optional, tag = "3")]
    pub report_location: ::core::option::Option<SourceLocation>,
}
/// HTTP request data that is related to a reported error.
/// This data should be provided by the application when reporting an error,
/// unless the
/// error report has been generated automatically from Google App Engine logs.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HttpRequestContext {
    /// The type of HTTP request, such as `GET`, `POST`, etc.
    #[prost(string, tag = "1")]
    pub method: ::prost::alloc::string::String,
    /// The URL of the request.
    #[prost(string, tag = "2")]
    pub url: ::prost::alloc::string::String,
    /// The user agent information that is provided with the request.
    #[prost(string, tag = "3")]
    pub user_agent: ::prost::alloc::string::String,
    /// The referrer information that is provided with the request.
    #[prost(string, tag = "4")]
    pub referrer: ::prost::alloc::string::String,
    /// The HTTP response status code for the request.
    #[prost(int32, tag = "5")]
    pub response_status_code: i32,
    /// The IP address from which the request originated.
    /// This can be IPv4, IPv6, or a token which is derived from the
    /// IP address, depending on the data that has been provided
    /// in the error report.
    #[prost(string, tag = "6")]
    pub remote_ip: ::prost::alloc::string::String,
}
/// Indicates a location in the source code of the service for which errors are
/// reported. `functionName` must be provided by the application when reporting
/// an error, unless the error report contains a `message` with a supported
/// exception stack trace. All fields are optional for the later case.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SourceLocation {
    /// The source code filename, which can include a truncated relative
    /// path, or a full path from a production machine.
    #[prost(string, tag = "1")]
    pub file_path: ::prost::alloc::string::String,
    /// 1-based. 0 indicates that the line number is unknown.
    #[prost(int32, tag = "2")]
    pub line_number: i32,
    /// Human-readable name of a function or method.
    /// The value can include optional context like the class or package name.
    /// For example, `my.package.MyClass.method` in case of Java.
    #[prost(string, tag = "4")]
    pub function_name: ::prost::alloc::string::String,
}
/// Resolution status of an error group.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ResolutionStatus {
    /// Status is unknown. When left unspecified in requests, it is treated like
    /// OPEN.
    Unspecified = 0,
    /// The error group is not being addressed. This is the default for
    /// new groups. It is also used for errors re-occurring after marked RESOLVED.
    Open = 1,
    /// Error Group manually acknowledged, it can have an issue link attached.
    Acknowledged = 2,
    /// Error Group manually resolved, more events for this group are not expected
    /// to occur.
    Resolved = 3,
    /// The error group is muted and excluded by default on group stats requests.
    Muted = 4,
}
/// A request to return an individual group.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetGroupRequest {
    /// Required. The group resource name. Written as
    /// `projects/{projectID}/groups/{group_name}`. Call
    /// \[`groupStats.list`\](<https://cloud.google.com/error-reporting/reference/rest/v1beta1/projects.groupStats/list>)
    /// to return a list of groups belonging to this project.
    ///
    /// Example: `projects/my-project-123/groups/my-group`
    #[prost(string, tag = "1")]
    pub group_name: ::prost::alloc::string::String,
}
/// A request to replace the existing data for the given group.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateGroupRequest {
    /// Required. The group which replaces the resource on the server.
    #[prost(message, optional, tag = "1")]
    pub group: ::core::option::Option<ErrorGroup>,
}
#[doc = r" Generated client implementations."]
pub mod error_group_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Service for retrieving and updating individual error groups."]
    #[derive(Debug, Clone)]
    pub struct ErrorGroupServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> ErrorGroupServiceClient<T>
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
        ) -> ErrorGroupServiceClient<InterceptedService<T, F>>
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
            ErrorGroupServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Get the specified group."]
        pub async fn get_group(
            &mut self,
            request: impl tonic::IntoRequest<super::GetGroupRequest>,
        ) -> Result<tonic::Response<super::ErrorGroup>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.devtools.clouderrorreporting.v1beta1.ErrorGroupService/GetGroup",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Replace the data for the specified group."]
        #[doc = " Fails if the group does not exist."]
        pub async fn update_group(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateGroupRequest>,
        ) -> Result<tonic::Response<super::ErrorGroup>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.devtools.clouderrorreporting.v1beta1.ErrorGroupService/UpdateGroup",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Specifies a set of `ErrorGroupStats` to return.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListGroupStatsRequest {
    /// Required. The resource name of the Google Cloud Platform project. Written
    /// as `projects/{projectID}` or `projects/{projectNumber}`, where `{projectID}`
    /// and `{projectNumber}` can be found in the
    /// [Google Cloud Console](<https://support.google.com/cloud/answer/6158840>).
    ///
    /// Examples: `projects/my-project-123`, `projects/5551234`.
    #[prost(string, tag = "1")]
    pub project_name: ::prost::alloc::string::String,
    /// Optional. List all <code>ErrorGroupStats</code> with these IDs.
    #[prost(string, repeated, tag = "2")]
    pub group_id: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Optional. List only <code>ErrorGroupStats</code> which belong to a service
    /// context that matches the filter.
    /// Data for all service contexts is returned if this field is not specified.
    #[prost(message, optional, tag = "3")]
    pub service_filter: ::core::option::Option<ServiceContextFilter>,
    /// Optional. List data for the given time range.
    /// If not set, a default time range is used. The field
    /// <code>time_range_begin</code> in the response will specify the beginning
    /// of this time range.
    /// Only <code>ErrorGroupStats</code> with a non-zero count in the given time
    /// range are returned, unless the request contains an explicit
    /// <code>group_id</code> list. If a <code>group_id</code> list is given, also
    /// <code>ErrorGroupStats</code> with zero occurrences are returned.
    #[prost(message, optional, tag = "5")]
    pub time_range: ::core::option::Option<QueryTimeRange>,
    /// Optional. The preferred duration for a single returned `TimedCount`.
    /// If not set, no timed counts are returned.
    #[prost(message, optional, tag = "6")]
    pub timed_count_duration: ::core::option::Option<::prost_types::Duration>,
    /// Optional. The alignment of the timed counts to be returned.
    /// Default is `ALIGNMENT_EQUAL_AT_END`.
    #[prost(enumeration = "TimedCountAlignment", tag = "7")]
    pub alignment: i32,
    /// Optional. Time where the timed counts shall be aligned if rounded
    /// alignment is chosen. Default is 00:00 UTC.
    #[prost(message, optional, tag = "8")]
    pub alignment_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Optional. The sort order in which the results are returned.
    /// Default is `COUNT_DESC`.
    #[prost(enumeration = "ErrorGroupOrder", tag = "9")]
    pub order: i32,
    /// Optional. The maximum number of results to return per response.
    /// Default is 20.
    #[prost(int32, tag = "11")]
    pub page_size: i32,
    /// Optional. A `next_page_token` provided by a previous response. To view
    /// additional results, pass this token along with the identical query
    /// parameters as the first request.
    #[prost(string, tag = "12")]
    pub page_token: ::prost::alloc::string::String,
}
/// Contains a set of requested error group stats.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListGroupStatsResponse {
    /// The error group stats which match the given request.
    #[prost(message, repeated, tag = "1")]
    pub error_group_stats: ::prost::alloc::vec::Vec<ErrorGroupStats>,
    /// If non-empty, more results are available.
    /// Pass this token, along with the same query parameters as the first
    /// request, to view the next page of results.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// The timestamp specifies the start time to which the request was restricted.
    /// The start time is set based on the requested time range. It may be adjusted
    /// to a later time if a project has exceeded the storage quota and older data
    /// has been deleted.
    #[prost(message, optional, tag = "4")]
    pub time_range_begin: ::core::option::Option<::prost_types::Timestamp>,
}
/// Data extracted for a specific group based on certain filter criteria,
/// such as a given time period and/or service filter.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ErrorGroupStats {
    /// Group data that is independent of the filter criteria.
    #[prost(message, optional, tag = "1")]
    pub group: ::core::option::Option<ErrorGroup>,
    /// Approximate total number of events in the given group that match
    /// the filter criteria.
    #[prost(int64, tag = "2")]
    pub count: i64,
    /// Approximate number of affected users in the given group that
    /// match the filter criteria.
    /// Users are distinguished by data in the `ErrorContext` of the
    /// individual error events, such as their login name or their remote
    /// IP address in case of HTTP requests.
    /// The number of affected users can be zero even if the number of
    /// errors is non-zero if no data was provided from which the
    /// affected user could be deduced.
    /// Users are counted based on data in the request
    /// context that was provided in the error report. If more users are
    /// implicitly affected, such as due to a crash of the whole service,
    /// this is not reflected here.
    #[prost(int64, tag = "3")]
    pub affected_users_count: i64,
    /// Approximate number of occurrences over time.
    /// Timed counts returned by ListGroups are guaranteed to be:
    ///
    /// - Inside the requested time interval
    /// - Non-overlapping, and
    /// - Ordered by ascending time.
    #[prost(message, repeated, tag = "4")]
    pub timed_counts: ::prost::alloc::vec::Vec<TimedCount>,
    /// Approximate first occurrence that was ever seen for this group
    /// and which matches the given filter criteria, ignoring the
    /// time_range that was specified in the request.
    #[prost(message, optional, tag = "5")]
    pub first_seen_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Approximate last occurrence that was ever seen for this group and
    /// which matches the given filter criteria, ignoring the time_range
    /// that was specified in the request.
    #[prost(message, optional, tag = "6")]
    pub last_seen_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Service contexts with a non-zero error count for the given filter
    /// criteria. This list can be truncated if multiple services are affected.
    /// Refer to `num_affected_services` for the total count.
    #[prost(message, repeated, tag = "7")]
    pub affected_services: ::prost::alloc::vec::Vec<ServiceContext>,
    /// The total number of services with a non-zero error count for the given
    /// filter criteria.
    #[prost(int32, tag = "8")]
    pub num_affected_services: i32,
    /// An arbitrary event that is chosen as representative for the whole group.
    /// The representative event is intended to be used as a quick preview for
    /// the whole group. Events in the group are usually sufficiently similar
    /// to each other such that showing an arbitrary representative provides
    /// insight into the characteristics of the group as a whole.
    #[prost(message, optional, tag = "9")]
    pub representative: ::core::option::Option<ErrorEvent>,
}
/// The number of errors in a given time period.
/// All numbers are approximate since the error events are sampled
/// before counting them.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TimedCount {
    /// Approximate number of occurrences in the given time period.
    #[prost(int64, tag = "1")]
    pub count: i64,
    /// Start of the time period to which `count` refers (included).
    #[prost(message, optional, tag = "2")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// End of the time period to which `count` refers (excluded).
    #[prost(message, optional, tag = "3")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Specifies a set of error events to return.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListEventsRequest {
    /// Required. The resource name of the Google Cloud Platform project. Written
    /// as `projects/{projectID}`, where `{projectID}` is the
    /// [Google Cloud Platform project
    /// ID](<https://support.google.com/cloud/answer/6158840>).
    ///
    /// Example: `projects/my-project-123`.
    #[prost(string, tag = "1")]
    pub project_name: ::prost::alloc::string::String,
    /// Required. The group for which events shall be returned.
    #[prost(string, tag = "2")]
    pub group_id: ::prost::alloc::string::String,
    /// Optional. List only ErrorGroups which belong to a service context that
    /// matches the filter.
    /// Data for all service contexts is returned if this field is not specified.
    #[prost(message, optional, tag = "3")]
    pub service_filter: ::core::option::Option<ServiceContextFilter>,
    /// Optional. List only data for the given time range.
    /// If not set a default time range is used. The field time_range_begin
    /// in the response will specify the beginning of this time range.
    #[prost(message, optional, tag = "4")]
    pub time_range: ::core::option::Option<QueryTimeRange>,
    /// Optional. The maximum number of results to return per response.
    #[prost(int32, tag = "6")]
    pub page_size: i32,
    /// Optional. A `next_page_token` provided by a previous response.
    #[prost(string, tag = "7")]
    pub page_token: ::prost::alloc::string::String,
}
/// Contains a set of requested error events.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListEventsResponse {
    /// The error events which match the given request.
    #[prost(message, repeated, tag = "1")]
    pub error_events: ::prost::alloc::vec::Vec<ErrorEvent>,
    /// If non-empty, more results are available.
    /// Pass this token, along with the same query parameters as the first
    /// request, to view the next page of results.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// The timestamp specifies the start time to which the request was restricted.
    #[prost(message, optional, tag = "4")]
    pub time_range_begin: ::core::option::Option<::prost_types::Timestamp>,
}
/// Requests might be rejected or the resulting timed count durations might be
/// adjusted for lower durations.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTimeRange {
    /// Restricts the query to the specified time range.
    #[prost(enumeration = "query_time_range::Period", tag = "1")]
    pub period: i32,
}
/// Nested message and enum types in `QueryTimeRange`.
pub mod query_time_range {
    /// The supported time ranges.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Period {
        /// Do not use.
        Unspecified = 0,
        /// Retrieve data for the last hour.
        /// Recommended minimum timed count duration: 1 min.
        Period1Hour = 1,
        /// Retrieve data for the last 6 hours.
        /// Recommended minimum timed count duration: 10 min.
        Period6Hours = 2,
        /// Retrieve data for the last day.
        /// Recommended minimum timed count duration: 1 hour.
        Period1Day = 3,
        /// Retrieve data for the last week.
        /// Recommended minimum timed count duration: 6 hours.
        Period1Week = 4,
        /// Retrieve data for the last 30 days.
        /// Recommended minimum timed count duration: 1 day.
        Period30Days = 5,
    }
}
/// Specifies criteria for filtering a subset of service contexts.
/// The fields in the filter correspond to the fields in `ServiceContext`.
/// Only exact, case-sensitive matches are supported.
/// If a field is unset or empty, it matches arbitrary values.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServiceContextFilter {
    /// Optional. The exact value to match against
    /// \[`ServiceContext.service`\](/error-reporting/reference/rest/v1beta1/ServiceContext#FIELDS.service).
    #[prost(string, tag = "2")]
    pub service: ::prost::alloc::string::String,
    /// Optional. The exact value to match against
    /// \[`ServiceContext.version`\](/error-reporting/reference/rest/v1beta1/ServiceContext#FIELDS.version).
    #[prost(string, tag = "3")]
    pub version: ::prost::alloc::string::String,
    /// Optional. The exact value to match against
    /// \[`ServiceContext.resource_type`\](/error-reporting/reference/rest/v1beta1/ServiceContext#FIELDS.resource_type).
    #[prost(string, tag = "4")]
    pub resource_type: ::prost::alloc::string::String,
}
/// Deletes all events in the project.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteEventsRequest {
    /// Required. The resource name of the Google Cloud Platform project. Written
    /// as `projects/{projectID}`, where `{projectID}` is the
    /// [Google Cloud Platform project
    /// ID](<https://support.google.com/cloud/answer/6158840>).
    ///
    /// Example: `projects/my-project-123`.
    #[prost(string, tag = "1")]
    pub project_name: ::prost::alloc::string::String,
}
/// Response message for deleting error events.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteEventsResponse {}
/// Specifies how the time periods of error group counts are aligned.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TimedCountAlignment {
    /// No alignment specified.
    ErrorCountAlignmentUnspecified = 0,
    /// The time periods shall be consecutive, have width equal to the
    /// requested duration, and be aligned at the `alignment_time` provided in
    /// the request.
    /// The `alignment_time` does not have to be inside the query period but
    /// even if it is outside, only time periods are returned which overlap
    /// with the query period.
    /// A rounded alignment will typically result in a
    /// different size of the first or the last time period.
    AlignmentEqualRounded = 1,
    /// The time periods shall be consecutive, have width equal to the
    /// requested duration, and be aligned at the end of the requested time
    /// period. This can result in a different size of the
    /// first time period.
    AlignmentEqualAtEnd = 2,
}
/// A sorting order of error groups.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ErrorGroupOrder {
    /// No group order specified.
    GroupOrderUnspecified = 0,
    /// Total count of errors in the given time window in descending order.
    CountDesc = 1,
    /// Timestamp when the group was last seen in the given time window
    /// in descending order.
    LastSeenDesc = 2,
    /// Timestamp when the group was created in descending order.
    CreatedDesc = 3,
    /// Number of affected users in the given time window in descending order.
    AffectedUsersDesc = 4,
}
#[doc = r" Generated client implementations."]
pub mod error_stats_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " An API for retrieving and managing error statistics as well as data for"]
    #[doc = " individual events."]
    #[derive(Debug, Clone)]
    pub struct ErrorStatsServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> ErrorStatsServiceClient<T>
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
        ) -> ErrorStatsServiceClient<InterceptedService<T, F>>
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
            ErrorStatsServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Lists the specified groups."]
        pub async fn list_group_stats(
            &mut self,
            request: impl tonic::IntoRequest<super::ListGroupStatsRequest>,
        ) -> Result<tonic::Response<super::ListGroupStatsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.devtools.clouderrorreporting.v1beta1.ErrorStatsService/ListGroupStats",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists the specified events."]
        pub async fn list_events(
            &mut self,
            request: impl tonic::IntoRequest<super::ListEventsRequest>,
        ) -> Result<tonic::Response<super::ListEventsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.devtools.clouderrorreporting.v1beta1.ErrorStatsService/ListEvents",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes all error events of a given project."]
        pub async fn delete_events(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteEventsRequest>,
        ) -> Result<tonic::Response<super::DeleteEventsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.devtools.clouderrorreporting.v1beta1.ErrorStatsService/DeleteEvents",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// A request for reporting an individual error event.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReportErrorEventRequest {
    /// Required. The resource name of the Google Cloud Platform project. Written
    /// as `projects/{projectId}`, where `{projectId}` is the
    /// [Google Cloud Platform project
    /// ID](<https://support.google.com/cloud/answer/6158840>).
    ///
    /// Example: // `projects/my-project-123`.
    #[prost(string, tag = "1")]
    pub project_name: ::prost::alloc::string::String,
    /// Required. The error event to be reported.
    #[prost(message, optional, tag = "2")]
    pub event: ::core::option::Option<ReportedErrorEvent>,
}
/// Response for reporting an individual error event.
/// Data may be added to this message in the future.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReportErrorEventResponse {}
/// An error event which is reported to the Error Reporting system.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReportedErrorEvent {
    /// Optional. Time when the event occurred.
    /// If not provided, the time when the event was received by the
    /// Error Reporting system will be used.
    #[prost(message, optional, tag = "1")]
    pub event_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Required. The service context in which this error has occurred.
    #[prost(message, optional, tag = "2")]
    pub service_context: ::core::option::Option<ServiceContext>,
    /// Required. The error message.
    /// If no `context.reportLocation` is provided, the message must contain a
    /// header (typically consisting of the exception type name and an error
    /// message) and an exception stack trace in one of the supported programming
    /// languages and formats.
    /// Supported languages are Java, Python, JavaScript, Ruby, C#, PHP, and Go.
    /// Supported stack trace formats are:
    ///
    /// * **Java**: Must be the return value of
    /// \[`Throwable.printStackTrace()`\](<https://docs.oracle.com/javase/7/docs/api/java/lang/Throwable.html#printStackTrace%28%29>).
    /// * **Python**: Must be the return value of
    /// \[`traceback.format_exc()`\](<https://docs.python.org/2/library/traceback.html#traceback.format_exc>).
    /// * **JavaScript**: Must be the value of
    /// \[`error.stack`\](<https://github.com/v8/v8/wiki/Stack-Trace-API>) as returned
    /// by V8.
    /// * **Ruby**: Must contain frames returned by
    /// \[`Exception.backtrace`\](<https://ruby-doc.org/core-2.2.0/Exception.html#method-i-backtrace>).
    /// * **C#**: Must be the return value of
    /// \[`Exception.ToString()`\](<https://msdn.microsoft.com/en-us/library/system.exception.tostring.aspx>).
    /// * **PHP**: Must start with `PHP (Notice|Parse error|Fatal error|Warning)`
    /// and contain the result of
    /// \[`(string)$exception`\](<http://php.net/manual/en/exception.tostring.php>).
    /// * **Go**: Must be the return value of
    /// \[`runtime.Stack()`\](<https://golang.org/pkg/runtime/debug/#Stack>).
    #[prost(string, tag = "3")]
    pub message: ::prost::alloc::string::String,
    /// Optional. A description of the context in which the error occurred.
    #[prost(message, optional, tag = "4")]
    pub context: ::core::option::Option<ErrorContext>,
}
#[doc = r" Generated client implementations."]
pub mod report_errors_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " An API for reporting error events."]
    #[derive(Debug, Clone)]
    pub struct ReportErrorsServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> ReportErrorsServiceClient<T>
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
        ) -> ReportErrorsServiceClient<InterceptedService<T, F>>
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
            ReportErrorsServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Report an individual error event and record the event to a log."]
        #[doc = ""]
        #[doc = " This endpoint accepts **either** an OAuth token,"]
        #[doc = " **or** an [API key](https://support.google.com/cloud/answer/6158862)"]
        #[doc = " for authentication. To use an API key, append it to the URL as the value of"]
        #[doc = " a `key` parameter. For example:"]
        #[doc = ""]
        #[doc = " `POST"]
        #[doc = " https://clouderrorreporting.googleapis.com/v1beta1/{projectName}/events:report?key=123ABC456`"]
        #[doc = ""]
        #[doc = " **Note:** [Error Reporting](/error-reporting) is a global service built"]
        #[doc = " on Cloud Logging and doesn't analyze logs stored"]
        #[doc = " in regional log buckets or logs routed to other Google Cloud projects."]
        #[doc = ""]
        #[doc = " For more information, see"]
        #[doc = " [Using Error Reporting with regionalized"]
        #[doc = " logs](/error-reporting/docs/regionalization)."]
        pub async fn report_error_event(
            &mut self,
            request: impl tonic::IntoRequest<super::ReportErrorEventRequest>,
        ) -> Result<tonic::Response<super::ReportErrorEventResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.devtools.clouderrorreporting.v1beta1.ReportErrorsService/ReportErrorEvent",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
