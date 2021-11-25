/// Represents preferences for sending email notifications for transfer run
/// events.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EmailPreferences {
    /// If true, email notifications will be sent on transfer run failures.
    #[prost(bool, tag = "1")]
    pub enable_failure_email: bool,
}
/// Options customizing the data transfer schedule.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScheduleOptions {
    /// If true, automatic scheduling of data transfer runs for this configuration
    /// will be disabled. The runs can be started on ad-hoc basis using
    /// StartManualTransferRuns API. When automatic scheduling is disabled, the
    /// TransferConfig.schedule field will be ignored.
    #[prost(bool, tag = "3")]
    pub disable_auto_scheduling: bool,
    /// Specifies time to start scheduling transfer runs. The first run will be
    /// scheduled at or after the start time according to a recurrence pattern
    /// defined in the schedule string. The start time can be changed at any
    /// moment. The time when a data transfer can be trigerred manually is not
    /// limited by this option.
    #[prost(message, optional, tag = "1")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Defines time to stop scheduling transfer runs. A transfer run cannot be
    /// scheduled at or after the end time. The end time can be changed at any
    /// moment. The time when a data transfer can be trigerred manually is not
    /// limited by this option.
    #[prost(message, optional, tag = "2")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Represents a data transfer configuration. A transfer configuration
/// contains all metadata needed to perform a data transfer. For example,
/// `destination_dataset_id` specifies where data should be stored.
/// When a new transfer configuration is created, the specified
/// `destination_dataset_id` is created when needed and shared with the
/// appropriate data source service account.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransferConfig {
    /// The resource name of the transfer config.
    /// Transfer config names have the form
    /// `projects/{project_id}/locations/{region}/transferConfigs/{config_id}`.
    /// Where `config_id` is usually a uuid, even though it is not
    /// guaranteed or required. The name is ignored when creating a transfer
    /// config.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// User specified display name for the data transfer.
    #[prost(string, tag = "3")]
    pub display_name: ::prost::alloc::string::String,
    /// Data source id. Cannot be changed once data transfer is created.
    #[prost(string, tag = "5")]
    pub data_source_id: ::prost::alloc::string::String,
    /// Parameters specific to each data source. For more information see the
    /// bq tab in the 'Setting up a data transfer' section for each data source.
    /// For example the parameters for Cloud Storage transfers are listed here:
    /// <https://cloud.google.com/bigquery-transfer/docs/cloud-storage-transfer#bq>
    #[prost(message, optional, tag = "9")]
    pub params: ::core::option::Option<::prost_types::Struct>,
    /// Data transfer schedule.
    /// If the data source does not support a custom schedule, this should be
    /// empty. If it is empty, the default value for the data source will be
    /// used.
    /// The specified times are in UTC.
    /// Examples of valid format:
    /// `1st,3rd monday of month 15:30`,
    /// `every wed,fri of jan,jun 13:15`, and
    /// `first sunday of quarter 00:00`.
    /// See more explanation about the format here:
    /// <https://cloud.google.com/appengine/docs/flexible/python/scheduling-jobs-with-cron-yaml#the_schedule_format>
    /// NOTE: the granularity should be at least 8 hours, or less frequent.
    #[prost(string, tag = "7")]
    pub schedule: ::prost::alloc::string::String,
    /// Options customizing the data transfer schedule.
    #[prost(message, optional, tag = "24")]
    pub schedule_options: ::core::option::Option<ScheduleOptions>,
    /// The number of days to look back to automatically refresh the data.
    /// For example, if `data_refresh_window_days = 10`, then every day
    /// BigQuery reingests data for [today-10, today-1], rather than ingesting data
    /// for just \[today-1\].
    /// Only valid if the data source supports the feature. Set the value to  0
    /// to use the default value.
    #[prost(int32, tag = "12")]
    pub data_refresh_window_days: i32,
    /// Is this config disabled. When set to true, no runs are scheduled
    /// for a given transfer.
    #[prost(bool, tag = "13")]
    pub disabled: bool,
    /// Output only. Data transfer modification time. Ignored by server on input.
    #[prost(message, optional, tag = "4")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Next time when data transfer will run.
    #[prost(message, optional, tag = "8")]
    pub next_run_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. State of the most recently updated transfer run.
    #[prost(enumeration = "TransferState", tag = "10")]
    pub state: i32,
    /// Deprecated. Unique ID of the user on whose behalf transfer is done.
    #[prost(int64, tag = "11")]
    pub user_id: i64,
    /// Output only. Region in which BigQuery dataset is located.
    #[prost(string, tag = "14")]
    pub dataset_region: ::prost::alloc::string::String,
    /// Pub/Sub topic where notifications will be sent after transfer runs
    /// associated with this transfer config finish.
    ///
    /// The format for specifying a pubsub topic is:
    /// `projects/{project}/topics/{topic}`
    #[prost(string, tag = "15")]
    pub notification_pubsub_topic: ::prost::alloc::string::String,
    /// Email notifications will be sent according to these preferences
    /// to the email address of the user who owns this transfer config.
    #[prost(message, optional, tag = "18")]
    pub email_preferences: ::core::option::Option<EmailPreferences>,
    /// The desination of the transfer config.
    #[prost(oneof = "transfer_config::Destination", tags = "2")]
    pub destination: ::core::option::Option<transfer_config::Destination>,
}
/// Nested message and enum types in `TransferConfig`.
pub mod transfer_config {
    /// The desination of the transfer config.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Destination {
        /// The BigQuery target dataset id.
        #[prost(string, tag = "2")]
        DestinationDatasetId(::prost::alloc::string::String),
    }
}
/// Represents a data transfer run.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransferRun {
    /// The resource name of the transfer run.
    /// Transfer run names have the form
    /// `projects/{project_id}/locations/{location}/transferConfigs/{config_id}/runs/{run_id}`.
    /// The name is ignored when creating a transfer run.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Minimum time after which a transfer run can be started.
    #[prost(message, optional, tag = "3")]
    pub schedule_time: ::core::option::Option<::prost_types::Timestamp>,
    /// For batch transfer runs, specifies the date and time of the data should be
    /// ingested.
    #[prost(message, optional, tag = "10")]
    pub run_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Status of the transfer run.
    #[prost(message, optional, tag = "21")]
    pub error_status: ::core::option::Option<super::super::super::super::rpc::Status>,
    /// Output only. Time when transfer run was started.
    /// Parameter ignored by server for input requests.
    #[prost(message, optional, tag = "4")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Time when transfer run ended.
    /// Parameter ignored by server for input requests.
    #[prost(message, optional, tag = "5")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Last time the data transfer run state was updated.
    #[prost(message, optional, tag = "6")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Parameters specific to each data source. For more information see the
    /// bq tab in the 'Setting up a data transfer' section for each data source.
    /// For example the parameters for Cloud Storage transfers are listed here:
    /// <https://cloud.google.com/bigquery-transfer/docs/cloud-storage-transfer#bq>
    #[prost(message, optional, tag = "9")]
    pub params: ::core::option::Option<::prost_types::Struct>,
    /// Output only. Data source id.
    #[prost(string, tag = "7")]
    pub data_source_id: ::prost::alloc::string::String,
    /// Data transfer run state. Ignored for input requests.
    #[prost(enumeration = "TransferState", tag = "8")]
    pub state: i32,
    /// Deprecated. Unique ID of the user on whose behalf transfer is done.
    #[prost(int64, tag = "11")]
    pub user_id: i64,
    /// Output only. Describes the schedule of this transfer run if it was
    /// created as part of a regular schedule. For batch transfer runs that are
    /// scheduled manually, this is empty.
    /// NOTE: the system might choose to delay the schedule depending on the
    /// current load, so `schedule_time` doesn't always match this.
    #[prost(string, tag = "12")]
    pub schedule: ::prost::alloc::string::String,
    /// Output only. Pub/Sub topic where a notification will be sent after this
    /// transfer run finishes.
    ///
    /// The format for specifying a pubsub topic is:
    /// `projects/{project}/topics/{topic}`
    #[prost(string, tag = "23")]
    pub notification_pubsub_topic: ::prost::alloc::string::String,
    /// Output only. Email notifications will be sent according to these
    /// preferences to the email address of the user who owns the transfer config
    /// this run was derived from.
    #[prost(message, optional, tag = "25")]
    pub email_preferences: ::core::option::Option<EmailPreferences>,
    /// Data transfer destination.
    #[prost(oneof = "transfer_run::Destination", tags = "2")]
    pub destination: ::core::option::Option<transfer_run::Destination>,
}
/// Nested message and enum types in `TransferRun`.
pub mod transfer_run {
    /// Data transfer destination.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Destination {
        /// Output only. The BigQuery target dataset id.
        #[prost(string, tag = "2")]
        DestinationDatasetId(::prost::alloc::string::String),
    }
}
/// Represents a user facing message for a particular data transfer run.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransferMessage {
    /// Time when message was logged.
    #[prost(message, optional, tag = "1")]
    pub message_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Message severity.
    #[prost(enumeration = "transfer_message::MessageSeverity", tag = "2")]
    pub severity: i32,
    /// Message text.
    #[prost(string, tag = "3")]
    pub message_text: ::prost::alloc::string::String,
}
/// Nested message and enum types in `TransferMessage`.
pub mod transfer_message {
    /// Represents data transfer user facing message severity.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum MessageSeverity {
        /// No severity specified.
        Unspecified = 0,
        /// Informational message.
        Info = 1,
        /// Warning message.
        Warning = 2,
        /// Error message.
        Error = 3,
    }
}
/// DEPRECATED. Represents data transfer type.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TransferType {
    /// Invalid or Unknown transfer type placeholder.
    Unspecified = 0,
    /// Batch data transfer.
    Batch = 1,
    /// Streaming data transfer. Streaming data source currently doesn't
    /// support multiple transfer configs per project.
    Streaming = 2,
}
/// Represents data transfer run state.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TransferState {
    /// State placeholder (0).
    Unspecified = 0,
    /// Data transfer is scheduled and is waiting to be picked up by
    /// data transfer backend (2).
    Pending = 2,
    /// Data transfer is in progress (3).
    Running = 3,
    /// Data transfer completed successfully (4).
    Succeeded = 4,
    /// Data transfer failed (5).
    Failed = 5,
    /// Data transfer is cancelled (6).
    Cancelled = 6,
}
/// Represents a data source parameter with validation rules, so that
/// parameters can be rendered in the UI. These parameters are given to us by
/// supported data sources, and include all needed information for rendering
/// and validation.
/// Thus, whoever uses this api can decide to generate either generic ui,
/// or custom data source specific forms.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DataSourceParameter {
    /// Parameter identifier.
    #[prost(string, tag = "1")]
    pub param_id: ::prost::alloc::string::String,
    /// Parameter display name in the user interface.
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    /// Parameter description.
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    /// Parameter type.
    #[prost(enumeration = "data_source_parameter::Type", tag = "4")]
    pub r#type: i32,
    /// Is parameter required.
    #[prost(bool, tag = "5")]
    pub required: bool,
    /// Deprecated. This field has no effect.
    #[prost(bool, tag = "6")]
    pub repeated: bool,
    /// Regular expression which can be used for parameter validation.
    #[prost(string, tag = "7")]
    pub validation_regex: ::prost::alloc::string::String,
    /// All possible values for the parameter.
    #[prost(string, repeated, tag = "8")]
    pub allowed_values: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// For integer and double values specifies minimum allowed value.
    #[prost(message, optional, tag = "9")]
    pub min_value: ::core::option::Option<f64>,
    /// For integer and double values specifies maxminum allowed value.
    #[prost(message, optional, tag = "10")]
    pub max_value: ::core::option::Option<f64>,
    /// Deprecated. This field has no effect.
    #[prost(message, repeated, tag = "11")]
    pub fields: ::prost::alloc::vec::Vec<DataSourceParameter>,
    /// Description of the requirements for this field, in case the user input does
    /// not fulfill the regex pattern or min/max values.
    #[prost(string, tag = "12")]
    pub validation_description: ::prost::alloc::string::String,
    /// URL to a help document to further explain the naming requirements.
    #[prost(string, tag = "13")]
    pub validation_help_url: ::prost::alloc::string::String,
    /// Cannot be changed after initial creation.
    #[prost(bool, tag = "14")]
    pub immutable: bool,
    /// Deprecated. This field has no effect.
    #[prost(bool, tag = "15")]
    pub recurse: bool,
    /// If true, it should not be used in new transfers, and it should not be
    /// visible to users.
    #[prost(bool, tag = "20")]
    pub deprecated: bool,
}
/// Nested message and enum types in `DataSourceParameter`.
pub mod data_source_parameter {
    /// Parameter type.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Type {
        /// Type unspecified.
        Unspecified = 0,
        /// String parameter.
        String = 1,
        /// Integer parameter (64-bits).
        /// Will be serialized to json as string.
        Integer = 2,
        /// Double precision floating point parameter.
        Double = 3,
        /// Boolean parameter.
        Boolean = 4,
        /// Deprecated. This field has no effect.
        Record = 5,
        /// Page ID for a Google+ Page.
        PlusPage = 6,
    }
}
/// Represents data source metadata. Metadata is sufficient to
/// render UI and request proper OAuth tokens.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DataSource {
    /// Output only. Data source resource name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Data source id.
    #[prost(string, tag = "2")]
    pub data_source_id: ::prost::alloc::string::String,
    /// User friendly data source name.
    #[prost(string, tag = "3")]
    pub display_name: ::prost::alloc::string::String,
    /// User friendly data source description string.
    #[prost(string, tag = "4")]
    pub description: ::prost::alloc::string::String,
    /// Data source client id which should be used to receive refresh token.
    #[prost(string, tag = "5")]
    pub client_id: ::prost::alloc::string::String,
    /// Api auth scopes for which refresh token needs to be obtained. These are
    /// scopes needed by a data source to prepare data and ingest them into
    /// BigQuery, e.g., <https://www.googleapis.com/auth/bigquery>
    #[prost(string, repeated, tag = "6")]
    pub scopes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Deprecated. This field has no effect.
    #[deprecated]
    #[prost(enumeration = "TransferType", tag = "7")]
    pub transfer_type: i32,
    /// Deprecated. This field has no effect.
    #[deprecated]
    #[prost(bool, tag = "8")]
    pub supports_multiple_transfers: bool,
    /// The number of seconds to wait for an update from the data source
    /// before the Data Transfer Service marks the transfer as FAILED.
    #[prost(int32, tag = "9")]
    pub update_deadline_seconds: i32,
    /// Default data transfer schedule.
    /// Examples of valid schedules include:
    /// `1st,3rd monday of month 15:30`,
    /// `every wed,fri of jan,jun 13:15`, and
    /// `first sunday of quarter 00:00`.
    #[prost(string, tag = "10")]
    pub default_schedule: ::prost::alloc::string::String,
    /// Specifies whether the data source supports a user defined schedule, or
    /// operates on the default schedule.
    /// When set to `true`, user can override default schedule.
    #[prost(bool, tag = "11")]
    pub supports_custom_schedule: bool,
    /// Data source parameters.
    #[prost(message, repeated, tag = "12")]
    pub parameters: ::prost::alloc::vec::Vec<DataSourceParameter>,
    /// Url for the help document for this data source.
    #[prost(string, tag = "13")]
    pub help_url: ::prost::alloc::string::String,
    /// Indicates the type of authorization.
    #[prost(enumeration = "data_source::AuthorizationType", tag = "14")]
    pub authorization_type: i32,
    /// Specifies whether the data source supports automatic data refresh for the
    /// past few days, and how it's supported.
    /// For some data sources, data might not be complete until a few days later,
    /// so it's useful to refresh data automatically.
    #[prost(enumeration = "data_source::DataRefreshType", tag = "15")]
    pub data_refresh_type: i32,
    /// Default data refresh window on days.
    /// Only meaningful when `data_refresh_type` = `SLIDING_WINDOW`.
    #[prost(int32, tag = "16")]
    pub default_data_refresh_window_days: i32,
    /// Disables backfilling and manual run scheduling
    /// for the data source.
    #[prost(bool, tag = "17")]
    pub manual_runs_disabled: bool,
    /// The minimum interval for scheduler to schedule runs.
    #[prost(message, optional, tag = "18")]
    pub minimum_schedule_interval: ::core::option::Option<::prost_types::Duration>,
}
/// Nested message and enum types in `DataSource`.
pub mod data_source {
    /// The type of authorization needed for this data source.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum AuthorizationType {
        /// Type unspecified.
        Unspecified = 0,
        /// Use OAuth 2 authorization codes that can be exchanged
        /// for a refresh token on the backend.
        AuthorizationCode = 1,
        /// Return an authorization code for a given Google+ page that can then be
        /// exchanged for a refresh token on the backend.
        GooglePlusAuthorizationCode = 2,
        /// Use First Party OAuth.
        FirstPartyOauth = 3,
    }
    /// Represents how the data source supports data auto refresh.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum DataRefreshType {
        /// The data source won't support data auto refresh, which is default value.
        Unspecified = 0,
        /// The data source supports data auto refresh, and runs will be scheduled
        /// for the past few days. Does not allow custom values to be set for each
        /// transfer config.
        SlidingWindow = 1,
        /// The data source supports data auto refresh, and runs will be scheduled
        /// for the past few days. Allows custom values to be set for each transfer
        /// config.
        CustomSlidingWindow = 2,
    }
}
/// A request to get data source info.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDataSourceRequest {
    /// Required. The field will contain name of the resource requested, for example:
    /// `projects/{project_id}/dataSources/{data_source_id}` or
    /// `projects/{project_id}/locations/{location_id}/dataSources/{data_source_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request to list supported data sources and their data transfer settings.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDataSourcesRequest {
    /// Required. The BigQuery project id for which data sources should be returned.
    /// Must be in the form: `projects/{project_id}` or
    /// `projects/{project_id}/locations/{location_id}
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Pagination token, which can be used to request a specific page
    /// of `ListDataSourcesRequest` list results. For multiple-page
    /// results, `ListDataSourcesResponse` outputs
    /// a `next_page` token, which can be used as the
    /// `page_token` value to request the next page of list results.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Page size. The default page size is the maximum value of 1000 results.
    #[prost(int32, tag = "4")]
    pub page_size: i32,
}
/// Returns list of supported data sources and their metadata.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDataSourcesResponse {
    /// List of supported data sources and their transfer settings.
    #[prost(message, repeated, tag = "1")]
    pub data_sources: ::prost::alloc::vec::Vec<DataSource>,
    /// Output only. The next-pagination token. For multiple-page list results,
    /// this token can be used as the
    /// `ListDataSourcesRequest.page_token`
    /// to request the next page of list results.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// A request to create a data transfer configuration. If new credentials are
/// needed for this transfer configuration, an authorization code must be
/// provided. If an authorization code is provided, the transfer configuration
/// will be associated with the user id corresponding to the
/// authorization code. Otherwise, the transfer configuration will be associated
/// with the calling user.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateTransferConfigRequest {
    /// Required. The BigQuery project id where the transfer configuration should be created.
    /// Must be in the format projects/{project_id}/locations/{location_id} or
    /// projects/{project_id}. If specified location and location of the
    /// destination bigquery dataset do not match - the request will fail.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. Data transfer configuration to create.
    #[prost(message, optional, tag = "2")]
    pub transfer_config: ::core::option::Option<TransferConfig>,
    /// Optional OAuth2 authorization code to use with this transfer configuration.
    /// This is required if new credentials are needed, as indicated by
    /// `CheckValidCreds`.
    /// In order to obtain authorization_code, please make a
    /// request to
    /// <https://www.gstatic.com/bigquerydatatransfer/oauthz/auth?client_id=<datatransferapiclientid>&scope=<data_source_scopes>&redirect_uri=<redirect_uri>>
    ///
    /// * client_id should be OAuth client_id of BigQuery DTS API for the given
    ///   data source returned by ListDataSources method.
    /// * data_source_scopes are the scopes returned by ListDataSources method.
    /// * redirect_uri is an optional parameter. If not specified, then
    ///   authorization code is posted to the opener of authorization flow window.
    ///   Otherwise it will be sent to the redirect uri. A special value of
    ///   urn:ietf:wg:oauth:2.0:oob means that authorization code should be
    ///   returned in the title bar of the browser, with the page text prompting
    ///   the user to copy the code and paste it in the application.
    #[prost(string, tag = "3")]
    pub authorization_code: ::prost::alloc::string::String,
    /// Optional version info. If users want to find a very recent access token,
    /// that is, immediately after approving access, users have to set the
    /// version_info claim in the token request. To obtain the version_info, users
    /// must use the "none+gsession" response type. which be return a
    /// version_info back in the authorization response which be be put in a JWT
    /// claim in the token request.
    #[prost(string, tag = "5")]
    pub version_info: ::prost::alloc::string::String,
    /// Optional service account name. If this field is set, transfer config will
    /// be created with this service account credentials. It requires that
    /// requesting user calling this API has permissions to act as this service
    /// account.
    #[prost(string, tag = "6")]
    pub service_account_name: ::prost::alloc::string::String,
}
/// A request to update a transfer configuration. To update the user id of the
/// transfer configuration, an authorization code needs to be provided.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateTransferConfigRequest {
    /// Required. Data transfer configuration to create.
    #[prost(message, optional, tag = "1")]
    pub transfer_config: ::core::option::Option<TransferConfig>,
    /// Optional OAuth2 authorization code to use with this transfer configuration.
    /// If it is provided, the transfer configuration will be associated with the
    /// authorizing user.
    /// In order to obtain authorization_code, please make a
    /// request to
    /// <https://www.gstatic.com/bigquerydatatransfer/oauthz/auth?client_id=<datatransferapiclientid>&scope=<data_source_scopes>&redirect_uri=<redirect_uri>>
    ///
    /// * client_id should be OAuth client_id of BigQuery DTS API for the given
    ///   data source returned by ListDataSources method.
    /// * data_source_scopes are the scopes returned by ListDataSources method.
    /// * redirect_uri is an optional parameter. If not specified, then
    ///   authorization code is posted to the opener of authorization flow window.
    ///   Otherwise it will be sent to the redirect uri. A special value of
    ///   urn:ietf:wg:oauth:2.0:oob means that authorization code should be
    ///   returned in the title bar of the browser, with the page text prompting
    ///   the user to copy the code and paste it in the application.
    #[prost(string, tag = "3")]
    pub authorization_code: ::prost::alloc::string::String,
    /// Required. Required list of fields to be updated in this request.
    #[prost(message, optional, tag = "4")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Optional version info. If users want to find a very recent access token,
    /// that is, immediately after approving access, users have to set the
    /// version_info claim in the token request. To obtain the version_info, users
    /// must use the "none+gsession" response type. which be return a
    /// version_info back in the authorization response which be be put in a JWT
    /// claim in the token request.
    #[prost(string, tag = "5")]
    pub version_info: ::prost::alloc::string::String,
    /// Optional service account name. If this field is set and
    /// "service_account_name" is set in update_mask, transfer config will be
    /// updated to use this service account credentials. It requires that
    /// requesting user calling this API has permissions to act as this service
    /// account.
    #[prost(string, tag = "6")]
    pub service_account_name: ::prost::alloc::string::String,
}
/// A request to get data transfer information.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTransferConfigRequest {
    /// Required. The field will contain name of the resource requested, for example:
    /// `projects/{project_id}/transferConfigs/{config_id}` or
    /// `projects/{project_id}/locations/{location_id}/transferConfigs/{config_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// A request to delete data transfer information. All associated transfer runs
/// and log messages will be deleted as well.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteTransferConfigRequest {
    /// Required. The field will contain name of the resource requested, for example:
    /// `projects/{project_id}/transferConfigs/{config_id}` or
    /// `projects/{project_id}/locations/{location_id}/transferConfigs/{config_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// A request to get data transfer run information.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTransferRunRequest {
    /// Required. The field will contain name of the resource requested, for example:
    /// `projects/{project_id}/transferConfigs/{config_id}/runs/{run_id}` or
    /// `projects/{project_id}/locations/{location_id}/transferConfigs/{config_id}/runs/{run_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// A request to delete data transfer run information.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteTransferRunRequest {
    /// Required. The field will contain name of the resource requested, for example:
    /// `projects/{project_id}/transferConfigs/{config_id}/runs/{run_id}` or
    /// `projects/{project_id}/locations/{location_id}/transferConfigs/{config_id}/runs/{run_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// A request to list data transfers configured for a BigQuery project.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTransferConfigsRequest {
    /// Required. The BigQuery project id for which data sources
    /// should be returned: `projects/{project_id}` or
    /// `projects/{project_id}/locations/{location_id}`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// When specified, only configurations of requested data sources are returned.
    #[prost(string, repeated, tag = "2")]
    pub data_source_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Pagination token, which can be used to request a specific page
    /// of `ListTransfersRequest` list results. For multiple-page
    /// results, `ListTransfersResponse` outputs
    /// a `next_page` token, which can be used as the
    /// `page_token` value to request the next page of list results.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Page size. The default page size is the maximum value of 1000 results.
    #[prost(int32, tag = "4")]
    pub page_size: i32,
}
/// The returned list of pipelines in the project.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTransferConfigsResponse {
    /// Output only. The stored pipeline transfer configurations.
    #[prost(message, repeated, tag = "1")]
    pub transfer_configs: ::prost::alloc::vec::Vec<TransferConfig>,
    /// Output only. The next-pagination token. For multiple-page list results,
    /// this token can be used as the
    /// `ListTransferConfigsRequest.page_token`
    /// to request the next page of list results.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// A request to list data transfer runs. UI can use this method to show/filter
/// specific data transfer runs. The data source can use this method to request
/// all scheduled transfer runs.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTransferRunsRequest {
    /// Required. Name of transfer configuration for which transfer runs should be retrieved.
    /// Format of transfer configuration resource name is:
    /// `projects/{project_id}/transferConfigs/{config_id}` or
    /// `projects/{project_id}/locations/{location_id}/transferConfigs/{config_id}`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// When specified, only transfer runs with requested states are returned.
    #[prost(enumeration = "TransferState", repeated, tag = "2")]
    pub states: ::prost::alloc::vec::Vec<i32>,
    /// Pagination token, which can be used to request a specific page
    /// of `ListTransferRunsRequest` list results. For multiple-page
    /// results, `ListTransferRunsResponse` outputs
    /// a `next_page` token, which can be used as the
    /// `page_token` value to request the next page of list results.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Page size. The default page size is the maximum value of 1000 results.
    #[prost(int32, tag = "4")]
    pub page_size: i32,
    /// Indicates how run attempts are to be pulled.
    #[prost(enumeration = "list_transfer_runs_request::RunAttempt", tag = "5")]
    pub run_attempt: i32,
}
/// Nested message and enum types in `ListTransferRunsRequest`.
pub mod list_transfer_runs_request {
    /// Represents which runs should be pulled.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum RunAttempt {
        /// All runs should be returned.
        Unspecified = 0,
        /// Only latest run per day should be returned.
        Latest = 1,
    }
}
/// The returned list of pipelines in the project.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTransferRunsResponse {
    /// Output only. The stored pipeline transfer runs.
    #[prost(message, repeated, tag = "1")]
    pub transfer_runs: ::prost::alloc::vec::Vec<TransferRun>,
    /// Output only. The next-pagination token. For multiple-page list results,
    /// this token can be used as the
    /// `ListTransferRunsRequest.page_token`
    /// to request the next page of list results.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// A request to get user facing log messages associated with data transfer run.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTransferLogsRequest {
    /// Required. Transfer run name in the form:
    /// `projects/{project_id}/transferConfigs/{config_id}/runs/{run_id}` or
    /// `projects/{project_id}/locations/{location_id}/transferConfigs/{config_id}/runs/{run_id}`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Pagination token, which can be used to request a specific page
    /// of `ListTransferLogsRequest` list results. For multiple-page
    /// results, `ListTransferLogsResponse` outputs
    /// a `next_page` token, which can be used as the
    /// `page_token` value to request the next page of list results.
    #[prost(string, tag = "4")]
    pub page_token: ::prost::alloc::string::String,
    /// Page size. The default page size is the maximum value of 1000 results.
    #[prost(int32, tag = "5")]
    pub page_size: i32,
    /// Message types to return. If not populated - INFO, WARNING and ERROR
    /// messages are returned.
    #[prost(enumeration = "transfer_message::MessageSeverity", repeated, tag = "6")]
    pub message_types: ::prost::alloc::vec::Vec<i32>,
}
/// The returned list transfer run messages.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTransferLogsResponse {
    /// Output only. The stored pipeline transfer messages.
    #[prost(message, repeated, tag = "1")]
    pub transfer_messages: ::prost::alloc::vec::Vec<TransferMessage>,
    /// Output only. The next-pagination token. For multiple-page list results,
    /// this token can be used as the
    /// `GetTransferRunLogRequest.page_token`
    /// to request the next page of list results.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// A request to determine whether the user has valid credentials. This method
/// is used to limit the number of OAuth popups in the user interface. The
/// user id is inferred from the API call context.
/// If the data source has the Google+ authorization type, this method
/// returns false, as it cannot be determined whether the credentials are
/// already valid merely based on the user id.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckValidCredsRequest {
    /// Required. The data source in the form:
    /// `projects/{project_id}/dataSources/{data_source_id}` or
    /// `projects/{project_id}/locations/{location_id}/dataSources/{data_source_id}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// A response indicating whether the credentials exist and are valid.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckValidCredsResponse {
    /// If set to `true`, the credentials exist and are valid.
    #[prost(bool, tag = "1")]
    pub has_valid_creds: bool,
}
/// A request to schedule transfer runs for a time range.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScheduleTransferRunsRequest {
    /// Required. Transfer configuration name in the form:
    /// `projects/{project_id}/transferConfigs/{config_id}` or
    /// `projects/{project_id}/locations/{location_id}/transferConfigs/{config_id}`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. Start time of the range of transfer runs. For example,
    /// `"2017-05-25T00:00:00+00:00"`.
    #[prost(message, optional, tag = "2")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Required. End time of the range of transfer runs. For example,
    /// `"2017-05-30T00:00:00+00:00"`.
    #[prost(message, optional, tag = "3")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// A response to schedule transfer runs for a time range.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScheduleTransferRunsResponse {
    /// The transfer runs that were scheduled.
    #[prost(message, repeated, tag = "1")]
    pub runs: ::prost::alloc::vec::Vec<TransferRun>,
}
/// A request to start manual transfer runs.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StartManualTransferRunsRequest {
    /// Transfer configuration name in the form:
    /// `projects/{project_id}/transferConfigs/{config_id}` or
    /// `projects/{project_id}/locations/{location_id}/transferConfigs/{config_id}`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The requested time specification - this can be a time range or a specific
    /// run_time.
    #[prost(oneof = "start_manual_transfer_runs_request::Time", tags = "3, 4")]
    pub time: ::core::option::Option<start_manual_transfer_runs_request::Time>,
}
/// Nested message and enum types in `StartManualTransferRunsRequest`.
pub mod start_manual_transfer_runs_request {
    /// A specification for a time range, this will request transfer runs with
    /// run_time between start_time (inclusive) and end_time (exclusive).
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TimeRange {
        /// Start time of the range of transfer runs. For example,
        /// `"2017-05-25T00:00:00+00:00"`. The start_time must be strictly less than
        /// the end_time. Creates transfer runs where run_time is in the range
        /// between start_time (inclusive) and end_time (exclusive).
        #[prost(message, optional, tag = "1")]
        pub start_time: ::core::option::Option<::prost_types::Timestamp>,
        /// End time of the range of transfer runs. For example,
        /// `"2017-05-30T00:00:00+00:00"`. The end_time must not be in the future.
        /// Creates transfer runs where run_time is in the range between start_time
        /// (inclusive) and end_time (exclusive).
        #[prost(message, optional, tag = "2")]
        pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    }
    /// The requested time specification - this can be a time range or a specific
    /// run_time.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Time {
        /// Time range for the transfer runs that should be started.
        #[prost(message, tag = "3")]
        RequestedTimeRange(TimeRange),
        /// Specific run_time for a transfer run to be started. The
        /// requested_run_time must not be in the future.
        #[prost(message, tag = "4")]
        RequestedRunTime(::prost_types::Timestamp),
    }
}
/// A response to start manual transfer runs.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StartManualTransferRunsResponse {
    /// The transfer runs that were created.
    #[prost(message, repeated, tag = "1")]
    pub runs: ::prost::alloc::vec::Vec<TransferRun>,
}
#[doc = r" Generated client implementations."]
pub mod data_transfer_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " The Google BigQuery Data Transfer Service API enables BigQuery users to"]
    #[doc = " configure the transfer of their data from other Google Products into"]
    #[doc = " BigQuery. This service contains methods that are end user exposed. It backs"]
    #[doc = " up the frontend."]
    #[derive(Debug, Clone)]
    pub struct DataTransferServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> DataTransferServiceClient<T>
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
        ) -> DataTransferServiceClient<InterceptedService<T, F>>
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
            DataTransferServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Retrieves a supported data source and returns its settings,"]
        #[doc = " which can be used for UI rendering."]
        pub async fn get_data_source(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDataSourceRequest>,
        ) -> Result<tonic::Response<super::DataSource>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.bigquery.datatransfer.v1.DataTransferService/GetDataSource",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists supported data sources and returns their settings,"]
        #[doc = " which can be used for UI rendering."]
        pub async fn list_data_sources(
            &mut self,
            request: impl tonic::IntoRequest<super::ListDataSourcesRequest>,
        ) -> Result<tonic::Response<super::ListDataSourcesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.bigquery.datatransfer.v1.DataTransferService/ListDataSources",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a new data transfer configuration."]
        pub async fn create_transfer_config(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateTransferConfigRequest>,
        ) -> Result<tonic::Response<super::TransferConfig>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.bigquery.datatransfer.v1.DataTransferService/CreateTransferConfig",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates a data transfer configuration."]
        #[doc = " All fields must be set, even if they are not updated."]
        pub async fn update_transfer_config(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateTransferConfigRequest>,
        ) -> Result<tonic::Response<super::TransferConfig>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.bigquery.datatransfer.v1.DataTransferService/UpdateTransferConfig",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes a data transfer configuration,"]
        #[doc = " including any associated transfer runs and logs."]
        pub async fn delete_transfer_config(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteTransferConfigRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.bigquery.datatransfer.v1.DataTransferService/DeleteTransferConfig",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns information about a data transfer config."]
        pub async fn get_transfer_config(
            &mut self,
            request: impl tonic::IntoRequest<super::GetTransferConfigRequest>,
        ) -> Result<tonic::Response<super::TransferConfig>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.bigquery.datatransfer.v1.DataTransferService/GetTransferConfig",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns information about all transfer configs owned by a project in the"]
        #[doc = " specified location."]
        pub async fn list_transfer_configs(
            &mut self,
            request: impl tonic::IntoRequest<super::ListTransferConfigsRequest>,
        ) -> Result<tonic::Response<super::ListTransferConfigsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.bigquery.datatransfer.v1.DataTransferService/ListTransferConfigs",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates transfer runs for a time range [start_time, end_time]."]
        #[doc = " For each date - or whatever granularity the data source supports - in the"]
        #[doc = " range, one transfer run is created."]
        #[doc = " Note that runs are created per UTC time in the time range."]
        #[doc = " DEPRECATED: use StartManualTransferRuns instead."]
        pub async fn schedule_transfer_runs(
            &mut self,
            request: impl tonic::IntoRequest<super::ScheduleTransferRunsRequest>,
        ) -> Result<tonic::Response<super::ScheduleTransferRunsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.bigquery.datatransfer.v1.DataTransferService/ScheduleTransferRuns",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Start manual transfer runs to be executed now with schedule_time equal to"]
        #[doc = " current time. The transfer runs can be created for a time range where the"]
        #[doc = " run_time is between start_time (inclusive) and end_time (exclusive), or for"]
        #[doc = " a specific run_time."]
        pub async fn start_manual_transfer_runs(
            &mut self,
            request: impl tonic::IntoRequest<super::StartManualTransferRunsRequest>,
        ) -> Result<tonic::Response<super::StartManualTransferRunsResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ("/google.cloud.bigquery.datatransfer.v1.DataTransferService/StartManualTransferRuns") ;
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns information about the particular transfer run."]
        pub async fn get_transfer_run(
            &mut self,
            request: impl tonic::IntoRequest<super::GetTransferRunRequest>,
        ) -> Result<tonic::Response<super::TransferRun>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.bigquery.datatransfer.v1.DataTransferService/GetTransferRun",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes the specified transfer run."]
        pub async fn delete_transfer_run(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteTransferRunRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.bigquery.datatransfer.v1.DataTransferService/DeleteTransferRun",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns information about running and completed jobs."]
        pub async fn list_transfer_runs(
            &mut self,
            request: impl tonic::IntoRequest<super::ListTransferRunsRequest>,
        ) -> Result<tonic::Response<super::ListTransferRunsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.bigquery.datatransfer.v1.DataTransferService/ListTransferRuns",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns user facing log messages for the data transfer run."]
        pub async fn list_transfer_logs(
            &mut self,
            request: impl tonic::IntoRequest<super::ListTransferLogsRequest>,
        ) -> Result<tonic::Response<super::ListTransferLogsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.bigquery.datatransfer.v1.DataTransferService/ListTransferLogs",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns true if valid credentials exist for the given data source and"]
        #[doc = " requesting user."]
        #[doc = " Some data sources doesn't support service account, so we need to talk to"]
        #[doc = " them on behalf of the end user. This API just checks whether we have OAuth"]
        #[doc = " token for the particular user, which is a pre-requisite before user can"]
        #[doc = " create a transfer config."]
        pub async fn check_valid_creds(
            &mut self,
            request: impl tonic::IntoRequest<super::CheckValidCredsRequest>,
        ) -> Result<tonic::Response<super::CheckValidCredsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.bigquery.datatransfer.v1.DataTransferService/CheckValidCreds",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
