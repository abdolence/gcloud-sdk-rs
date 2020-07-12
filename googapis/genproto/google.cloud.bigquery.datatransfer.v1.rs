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
    pub start_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Defines time to stop scheduling transfer runs. A transfer run cannot be
    /// scheduled at or after the end time. The end time can be changed at any
    /// moment. The time when a data transfer can be trigerred manually is not
    /// limited by this option.
    #[prost(message, optional, tag = "2")]
    pub end_time: ::std::option::Option<::prost_types::Timestamp>,
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
    /// Transfer config names have the form of
    /// `projects/{project_id}/locations/{region}/transferConfigs/{config_id}`.
    /// The name is automatically generated based on the config_id specified in
    /// CreateTransferConfigRequest along with project_id and region. If config_id
    /// is not provided, usually a uuid, even though it is not guaranteed or
    /// required, will be generated for config_id.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// User specified display name for the data transfer.
    #[prost(string, tag = "3")]
    pub display_name: std::string::String,
    /// Data source id. Cannot be changed once data transfer is created.
    #[prost(string, tag = "5")]
    pub data_source_id: std::string::String,
    /// Data transfer specific parameters.
    #[prost(message, optional, tag = "9")]
    pub params: ::std::option::Option<::prost_types::Struct>,
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
    /// https://cloud.google.com/appengine/docs/flexible/python/scheduling-jobs-with-cron-yaml#the_schedule_format
    /// NOTE: the granularity should be at least 8 hours, or less frequent.
    #[prost(string, tag = "7")]
    pub schedule: std::string::String,
    /// Options customizing the data transfer schedule.
    #[prost(message, optional, tag = "24")]
    pub schedule_options: ::std::option::Option<ScheduleOptions>,
    /// The number of days to look back to automatically refresh the data.
    /// For example, if `data_refresh_window_days = 10`, then every day
    /// BigQuery reingests data for [today-10, today-1], rather than ingesting data
    /// for just [today-1].
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
    pub update_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Output only. Next time when data transfer will run.
    #[prost(message, optional, tag = "8")]
    pub next_run_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Output only. State of the most recently updated transfer run.
    #[prost(enumeration = "TransferState", tag = "10")]
    pub state: i32,
    /// Deprecated. Unique ID of the user on whose behalf transfer is done.
    #[prost(int64, tag = "11")]
    pub user_id: i64,
    /// Output only. Region in which BigQuery dataset is located.
    #[prost(string, tag = "14")]
    pub dataset_region: std::string::String,
    /// Pub/Sub topic where notifications will be sent after transfer runs
    /// associated with this transfer config finish.
    #[prost(string, tag = "15")]
    pub notification_pubsub_topic: std::string::String,
    /// Email notifications will be sent according to these preferences
    /// to the email address of the user who owns this transfer config.
    #[prost(message, optional, tag = "18")]
    pub email_preferences: ::std::option::Option<EmailPreferences>,
    /// The desination of the transfer config.
    #[prost(oneof = "transfer_config::Destination", tags = "2")]
    pub destination: ::std::option::Option<transfer_config::Destination>,
}
pub mod transfer_config {
    /// The desination of the transfer config.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Destination {
        /// The BigQuery target dataset id.
        #[prost(string, tag = "2")]
        DestinationDatasetId(std::string::String),
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
    pub name: std::string::String,
    /// Minimum time after which a transfer run can be started.
    #[prost(message, optional, tag = "3")]
    pub schedule_time: ::std::option::Option<::prost_types::Timestamp>,
    /// For batch transfer runs, specifies the date and time of the data should be
    /// ingested.
    #[prost(message, optional, tag = "10")]
    pub run_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Status of the transfer run.
    #[prost(message, optional, tag = "21")]
    pub error_status: ::std::option::Option<super::super::super::super::rpc::Status>,
    /// Output only. Time when transfer run was started.
    /// Parameter ignored by server for input requests.
    #[prost(message, optional, tag = "4")]
    pub start_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Output only. Time when transfer run ended.
    /// Parameter ignored by server for input requests.
    #[prost(message, optional, tag = "5")]
    pub end_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Output only. Last time the data transfer run state was updated.
    #[prost(message, optional, tag = "6")]
    pub update_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Output only. Data transfer specific parameters.
    #[prost(message, optional, tag = "9")]
    pub params: ::std::option::Option<::prost_types::Struct>,
    /// Output only. Data source id.
    #[prost(string, tag = "7")]
    pub data_source_id: std::string::String,
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
    pub schedule: std::string::String,
    /// Output only. Pub/Sub topic where a notification will be sent after this
    /// transfer run finishes
    #[prost(string, tag = "23")]
    pub notification_pubsub_topic: std::string::String,
    /// Output only. Email notifications will be sent according to these
    /// preferences to the email address of the user who owns the transfer config
    /// this run was derived from.
    #[prost(message, optional, tag = "25")]
    pub email_preferences: ::std::option::Option<EmailPreferences>,
    /// Data transfer destination.
    #[prost(oneof = "transfer_run::Destination", tags = "2")]
    pub destination: ::std::option::Option<transfer_run::Destination>,
}
pub mod transfer_run {
    /// Data transfer destination.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Destination {
        /// Output only. The BigQuery target dataset id.
        #[prost(string, tag = "2")]
        DestinationDatasetId(std::string::String),
    }
}
/// Represents a user facing message for a particular data transfer run.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransferMessage {
    /// Time when message was logged.
    #[prost(message, optional, tag = "1")]
    pub message_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Message severity.
    #[prost(enumeration = "transfer_message::MessageSeverity", tag = "2")]
    pub severity: i32,
    /// Message text.
    #[prost(string, tag = "3")]
    pub message_text: std::string::String,
}
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
    /// State placeholder.
    Unspecified = 0,
    /// Data transfer is scheduled and is waiting to be picked up by
    /// data transfer backend.
    Pending = 2,
    /// Data transfer is in progress.
    Running = 3,
    /// Data transfer completed successfully.
    Succeeded = 4,
    /// Data transfer failed.
    Failed = 5,
    /// Data transfer is cancelled.
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
    pub param_id: std::string::String,
    /// Parameter display name in the user interface.
    #[prost(string, tag = "2")]
    pub display_name: std::string::String,
    /// Parameter description.
    #[prost(string, tag = "3")]
    pub description: std::string::String,
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
    pub validation_regex: std::string::String,
    /// All possible values for the parameter.
    #[prost(string, repeated, tag = "8")]
    pub allowed_values: ::std::vec::Vec<std::string::String>,
    /// For integer and double values specifies minimum allowed value.
    #[prost(message, optional, tag = "9")]
    pub min_value: ::std::option::Option<f64>,
    /// For integer and double values specifies maxminum allowed value.
    #[prost(message, optional, tag = "10")]
    pub max_value: ::std::option::Option<f64>,
    /// Deprecated. This field has no effect.
    #[prost(message, repeated, tag = "11")]
    pub fields: ::std::vec::Vec<DataSourceParameter>,
    /// Description of the requirements for this field, in case the user input does
    /// not fulfill the regex pattern or min/max values.
    #[prost(string, tag = "12")]
    pub validation_description: std::string::String,
    /// URL to a help document to further explain the naming requirements.
    #[prost(string, tag = "13")]
    pub validation_help_url: std::string::String,
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
    pub name: std::string::String,
    /// Data source id.
    #[prost(string, tag = "2")]
    pub data_source_id: std::string::String,
    /// User friendly data source name.
    #[prost(string, tag = "3")]
    pub display_name: std::string::String,
    /// User friendly data source description string.
    #[prost(string, tag = "4")]
    pub description: std::string::String,
    /// Data source client id which should be used to receive refresh token.
    #[prost(string, tag = "5")]
    pub client_id: std::string::String,
    /// Api auth scopes for which refresh token needs to be obtained. These are
    /// scopes needed by a data source to prepare data and ingest them into
    /// BigQuery, e.g., https://www.googleapis.com/auth/bigquery
    #[prost(string, repeated, tag = "6")]
    pub scopes: ::std::vec::Vec<std::string::String>,
    /// Deprecated. This field has no effect.
    #[prost(enumeration = "TransferType", tag = "7")]
    pub transfer_type: i32,
    /// Deprecated. This field has no effect.
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
    pub default_schedule: std::string::String,
    /// Specifies whether the data source supports a user defined schedule, or
    /// operates on the default schedule.
    /// When set to `true`, user can override default schedule.
    #[prost(bool, tag = "11")]
    pub supports_custom_schedule: bool,
    /// Data source parameters.
    #[prost(message, repeated, tag = "12")]
    pub parameters: ::std::vec::Vec<DataSourceParameter>,
    /// Url for the help document for this data source.
    #[prost(string, tag = "13")]
    pub help_url: std::string::String,
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
    pub minimum_schedule_interval: ::std::option::Option<::prost_types::Duration>,
}
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
        /// Use First Party Client OAuth. First Party Client OAuth doesn't require a
        /// refresh token to get an offline access token. Instead, it uses a
        /// client-signed JWT assertion to retrieve an access token.
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
    pub name: std::string::String,
}
/// Request to list supported data sources and their data transfer settings.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDataSourcesRequest {
    /// Required. The BigQuery project id for which data sources should be returned.
    /// Must be in the form: `projects/{project_id}` or
    /// `projects/{project_id}/locations/{location_id}
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Pagination token, which can be used to request a specific page
    /// of `ListDataSourcesRequest` list results. For multiple-page
    /// results, `ListDataSourcesResponse` outputs
    /// a `next_page` token, which can be used as the
    /// `page_token` value to request the next page of list results.
    #[prost(string, tag = "3")]
    pub page_token: std::string::String,
    /// Page size. The default page size is the maximum value of 1000 results.
    #[prost(int32, tag = "4")]
    pub page_size: i32,
}
/// Returns list of supported data sources and their metadata.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDataSourcesResponse {
    /// List of supported data sources and their transfer settings.
    #[prost(message, repeated, tag = "1")]
    pub data_sources: ::std::vec::Vec<DataSource>,
    /// Output only. The next-pagination token. For multiple-page list results,
    /// this token can be used as the
    /// `ListDataSourcesRequest.page_token`
    /// to request the next page of list results.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
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
    pub parent: std::string::String,
    /// Required. Data transfer configuration to create.
    #[prost(message, optional, tag = "2")]
    pub transfer_config: ::std::option::Option<TransferConfig>,
    /// Optional OAuth2 authorization code to use with this transfer configuration.
    /// This is required if new credentials are needed, as indicated by
    /// `CheckValidCreds`.
    /// In order to obtain authorization_code, please make a
    /// request to
    /// https://www.gstatic.com/bigquerydatatransfer/oauthz/auth?client_id=<datatransferapiclientid>&scope=<data_source_scopes>&redirect_uri=<redirect_uri>
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
    pub authorization_code: std::string::String,
    /// Optional version info. If users want to find a very recent access token,
    /// that is, immediately after approving access, users have to set the
    /// version_info claim in the token request. To obtain the version_info, users
    /// must use the "none+gsession" response type. which be return a
    /// version_info back in the authorization response which be be put in a JWT
    /// claim in the token request.
    #[prost(string, tag = "5")]
    pub version_info: std::string::String,
    /// Optional service account name. If this field is set, transfer config will
    /// be created with this service account credentials. It requires that
    /// requesting user calling this API has permissions to act as this service
    /// account.
    #[prost(string, tag = "6")]
    pub service_account_name: std::string::String,
}
/// A request to update a transfer configuration. To update the user id of the
/// transfer configuration, an authorization code needs to be provided.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateTransferConfigRequest {
    /// Required. Data transfer configuration to create.
    #[prost(message, optional, tag = "1")]
    pub transfer_config: ::std::option::Option<TransferConfig>,
    /// Optional OAuth2 authorization code to use with this transfer configuration.
    /// If it is provided, the transfer configuration will be associated with the
    /// authorizing user.
    /// In order to obtain authorization_code, please make a
    /// request to
    /// https://www.gstatic.com/bigquerydatatransfer/oauthz/auth?client_id=<datatransferapiclientid>&scope=<data_source_scopes>&redirect_uri=<redirect_uri>
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
    pub authorization_code: std::string::String,
    /// Required. Required list of fields to be updated in this request.
    #[prost(message, optional, tag = "4")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
    /// Optional version info. If users want to find a very recent access token,
    /// that is, immediately after approving access, users have to set the
    /// version_info claim in the token request. To obtain the version_info, users
    /// must use the "none+gsession" response type. which be return a
    /// version_info back in the authorization response which be be put in a JWT
    /// claim in the token request.
    #[prost(string, tag = "5")]
    pub version_info: std::string::String,
    /// Optional service account name. If this field is set and
    /// "service_account_name" is set in update_mask, transfer config will be
    /// updated to use this service account credentials. It requires that
    /// requesting user calling this API has permissions to act as this service
    /// account.
    #[prost(string, tag = "6")]
    pub service_account_name: std::string::String,
}
/// A request to get data transfer information.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTransferConfigRequest {
    /// Required. The field will contain name of the resource requested, for example:
    /// `projects/{project_id}/transferConfigs/{config_id}` or
    /// `projects/{project_id}/locations/{location_id}/transferConfigs/{config_id}`
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// A request to delete data transfer information. All associated transfer runs
/// and log messages will be deleted as well.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteTransferConfigRequest {
    /// Required. The field will contain name of the resource requested, for example:
    /// `projects/{project_id}/transferConfigs/{config_id}` or
    /// `projects/{project_id}/locations/{location_id}/transferConfigs/{config_id}`
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// A request to get data transfer run information.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTransferRunRequest {
    /// Required. The field will contain name of the resource requested, for example:
    /// `projects/{project_id}/transferConfigs/{config_id}/runs/{run_id}` or
    /// `projects/{project_id}/locations/{location_id}/transferConfigs/{config_id}/runs/{run_id}`
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// A request to delete data transfer run information.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteTransferRunRequest {
    /// Required. The field will contain name of the resource requested, for example:
    /// `projects/{project_id}/transferConfigs/{config_id}/runs/{run_id}` or
    /// `projects/{project_id}/locations/{location_id}/transferConfigs/{config_id}/runs/{run_id}`
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// A request to list data transfers configured for a BigQuery project.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTransferConfigsRequest {
    /// Required. The BigQuery project id for which data sources
    /// should be returned: `projects/{project_id}` or
    /// `projects/{project_id}/locations/{location_id}`
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// When specified, only configurations of requested data sources are returned.
    #[prost(string, repeated, tag = "2")]
    pub data_source_ids: ::std::vec::Vec<std::string::String>,
    /// Pagination token, which can be used to request a specific page
    /// of `ListTransfersRequest` list results. For multiple-page
    /// results, `ListTransfersResponse` outputs
    /// a `next_page` token, which can be used as the
    /// `page_token` value to request the next page of list results.
    #[prost(string, tag = "3")]
    pub page_token: std::string::String,
    /// Page size. The default page size is the maximum value of 1000 results.
    #[prost(int32, tag = "4")]
    pub page_size: i32,
}
/// The returned list of pipelines in the project.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTransferConfigsResponse {
    /// Output only. The stored pipeline transfer configurations.
    #[prost(message, repeated, tag = "1")]
    pub transfer_configs: ::std::vec::Vec<TransferConfig>,
    /// Output only. The next-pagination token. For multiple-page list results,
    /// this token can be used as the
    /// `ListTransferConfigsRequest.page_token`
    /// to request the next page of list results.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
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
    pub parent: std::string::String,
    /// When specified, only transfer runs with requested states are returned.
    #[prost(enumeration = "TransferState", repeated, tag = "2")]
    pub states: ::std::vec::Vec<i32>,
    /// Pagination token, which can be used to request a specific page
    /// of `ListTransferRunsRequest` list results. For multiple-page
    /// results, `ListTransferRunsResponse` outputs
    /// a `next_page` token, which can be used as the
    /// `page_token` value to request the next page of list results.
    #[prost(string, tag = "3")]
    pub page_token: std::string::String,
    /// Page size. The default page size is the maximum value of 1000 results.
    #[prost(int32, tag = "4")]
    pub page_size: i32,
    /// Indicates how run attempts are to be pulled.
    #[prost(enumeration = "list_transfer_runs_request::RunAttempt", tag = "5")]
    pub run_attempt: i32,
}
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
    pub transfer_runs: ::std::vec::Vec<TransferRun>,
    /// Output only. The next-pagination token. For multiple-page list results,
    /// this token can be used as the
    /// `ListTransferRunsRequest.page_token`
    /// to request the next page of list results.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
}
/// A request to get user facing log messages associated with data transfer run.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTransferLogsRequest {
    /// Required. Transfer run name in the form:
    /// `projects/{project_id}/transferConfigs/{config_id}/runs/{run_id}` or
    /// `projects/{project_id}/locations/{location_id}/transferConfigs/{config_id}/runs/{run_id}`
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Pagination token, which can be used to request a specific page
    /// of `ListTransferLogsRequest` list results. For multiple-page
    /// results, `ListTransferLogsResponse` outputs
    /// a `next_page` token, which can be used as the
    /// `page_token` value to request the next page of list results.
    #[prost(string, tag = "4")]
    pub page_token: std::string::String,
    /// Page size. The default page size is the maximum value of 1000 results.
    #[prost(int32, tag = "5")]
    pub page_size: i32,
    /// Message types to return. If not populated - INFO, WARNING and ERROR
    /// messages are returned.
    #[prost(enumeration = "transfer_message::MessageSeverity", repeated, tag = "6")]
    pub message_types: ::std::vec::Vec<i32>,
}
/// The returned list transfer run messages.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTransferLogsResponse {
    /// Output only. The stored pipeline transfer messages.
    #[prost(message, repeated, tag = "1")]
    pub transfer_messages: ::std::vec::Vec<TransferMessage>,
    /// Output only. The next-pagination token. For multiple-page list results,
    /// this token can be used as the
    /// `GetTransferRunLogRequest.page_token`
    /// to request the next page of list results.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
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
    pub name: std::string::String,
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
    pub parent: std::string::String,
    /// Required. Start time of the range of transfer runs. For example,
    /// `"2017-05-25T00:00:00+00:00"`.
    #[prost(message, optional, tag = "2")]
    pub start_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Required. End time of the range of transfer runs. For example,
    /// `"2017-05-30T00:00:00+00:00"`.
    #[prost(message, optional, tag = "3")]
    pub end_time: ::std::option::Option<::prost_types::Timestamp>,
}
/// A response to schedule transfer runs for a time range.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScheduleTransferRunsResponse {
    /// The transfer runs that were scheduled.
    #[prost(message, repeated, tag = "1")]
    pub runs: ::std::vec::Vec<TransferRun>,
}
/// A request to start manual transfer runs.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StartManualTransferRunsRequest {
    /// Transfer configuration name in the form:
    /// `projects/{project_id}/transferConfigs/{config_id}` or
    /// `projects/{project_id}/locations/{location_id}/transferConfigs/{config_id}`.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// The requested time specification - this can be a time range or a specific
    /// run_time.
    #[prost(oneof = "start_manual_transfer_runs_request::Time", tags = "3, 4")]
    pub time: ::std::option::Option<start_manual_transfer_runs_request::Time>,
}
pub mod start_manual_transfer_runs_request {
    /// A specification for a time range, this will request transfer runs with
    /// run_time between start_time (inclusive) and end_time (exclusive).
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TimeRange {
        /// Start time of the range of transfer runs. For example,
        /// `"2017-05-25T00:00:00+00:00"`. The start_time must be strictly less than
        /// the end_time. Creates transfer runs where run_time is in the range betwen
        /// start_time (inclusive) and end_time (exlusive).
        #[prost(message, optional, tag = "1")]
        pub start_time: ::std::option::Option<::prost_types::Timestamp>,
        /// End time of the range of transfer runs. For example,
        /// `"2017-05-30T00:00:00+00:00"`. The end_time must not be in the future.
        /// Creates transfer runs where run_time is in the range betwen start_time
        /// (inclusive) and end_time (exlusive).
        #[prost(message, optional, tag = "2")]
        pub end_time: ::std::option::Option<::prost_types::Timestamp>,
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
    pub runs: ::std::vec::Vec<TransferRun>,
}
#[doc = r" Generated client implementations."]
pub mod data_transfer_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " The Google BigQuery Data Transfer Service API enables BigQuery users to"]
    #[doc = " configure the transfer of their data from other Google Products into"]
    #[doc = " BigQuery. This service contains methods that are end user exposed. It backs"]
    #[doc = " up the frontend."]
    pub struct DataTransferServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> DataTransferServiceClient<T>
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
        #[doc = " Returns information about all data transfers in the project."]
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
            let path = http :: uri :: PathAndQuery :: from_static ( "/google.cloud.bigquery.datatransfer.v1.DataTransferService/StartManualTransferRuns" ) ;
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
    impl<T: Clone> Clone for DataTransferServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for DataTransferServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "DataTransferServiceClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod data_transfer_service_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with DataTransferServiceServer."]
    #[async_trait]
    pub trait DataTransferService: Send + Sync + 'static {
        #[doc = " Retrieves a supported data source and returns its settings,"]
        #[doc = " which can be used for UI rendering."]
        async fn get_data_source(
            &self,
            request: tonic::Request<super::GetDataSourceRequest>,
        ) -> Result<tonic::Response<super::DataSource>, tonic::Status>;
        #[doc = " Lists supported data sources and returns their settings,"]
        #[doc = " which can be used for UI rendering."]
        async fn list_data_sources(
            &self,
            request: tonic::Request<super::ListDataSourcesRequest>,
        ) -> Result<tonic::Response<super::ListDataSourcesResponse>, tonic::Status>;
        #[doc = " Creates a new data transfer configuration."]
        async fn create_transfer_config(
            &self,
            request: tonic::Request<super::CreateTransferConfigRequest>,
        ) -> Result<tonic::Response<super::TransferConfig>, tonic::Status>;
        #[doc = " Updates a data transfer configuration."]
        #[doc = " All fields must be set, even if they are not updated."]
        async fn update_transfer_config(
            &self,
            request: tonic::Request<super::UpdateTransferConfigRequest>,
        ) -> Result<tonic::Response<super::TransferConfig>, tonic::Status>;
        #[doc = " Deletes a data transfer configuration,"]
        #[doc = " including any associated transfer runs and logs."]
        async fn delete_transfer_config(
            &self,
            request: tonic::Request<super::DeleteTransferConfigRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status>;
        #[doc = " Returns information about a data transfer config."]
        async fn get_transfer_config(
            &self,
            request: tonic::Request<super::GetTransferConfigRequest>,
        ) -> Result<tonic::Response<super::TransferConfig>, tonic::Status>;
        #[doc = " Returns information about all data transfers in the project."]
        async fn list_transfer_configs(
            &self,
            request: tonic::Request<super::ListTransferConfigsRequest>,
        ) -> Result<tonic::Response<super::ListTransferConfigsResponse>, tonic::Status>;
        #[doc = " Creates transfer runs for a time range [start_time, end_time]."]
        #[doc = " For each date - or whatever granularity the data source supports - in the"]
        #[doc = " range, one transfer run is created."]
        #[doc = " Note that runs are created per UTC time in the time range."]
        #[doc = " DEPRECATED: use StartManualTransferRuns instead."]
        async fn schedule_transfer_runs(
            &self,
            request: tonic::Request<super::ScheduleTransferRunsRequest>,
        ) -> Result<tonic::Response<super::ScheduleTransferRunsResponse>, tonic::Status>;
        #[doc = " Start manual transfer runs to be executed now with schedule_time equal to"]
        #[doc = " current time. The transfer runs can be created for a time range where the"]
        #[doc = " run_time is between start_time (inclusive) and end_time (exclusive), or for"]
        #[doc = " a specific run_time."]
        async fn start_manual_transfer_runs(
            &self,
            request: tonic::Request<super::StartManualTransferRunsRequest>,
        ) -> Result<tonic::Response<super::StartManualTransferRunsResponse>, tonic::Status>;
        #[doc = " Returns information about the particular transfer run."]
        async fn get_transfer_run(
            &self,
            request: tonic::Request<super::GetTransferRunRequest>,
        ) -> Result<tonic::Response<super::TransferRun>, tonic::Status>;
        #[doc = " Deletes the specified transfer run."]
        async fn delete_transfer_run(
            &self,
            request: tonic::Request<super::DeleteTransferRunRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status>;
        #[doc = " Returns information about running and completed jobs."]
        async fn list_transfer_runs(
            &self,
            request: tonic::Request<super::ListTransferRunsRequest>,
        ) -> Result<tonic::Response<super::ListTransferRunsResponse>, tonic::Status>;
        #[doc = " Returns user facing log messages for the data transfer run."]
        async fn list_transfer_logs(
            &self,
            request: tonic::Request<super::ListTransferLogsRequest>,
        ) -> Result<tonic::Response<super::ListTransferLogsResponse>, tonic::Status>;
        #[doc = " Returns true if valid credentials exist for the given data source and"]
        #[doc = " requesting user."]
        #[doc = " Some data sources doesn't support service account, so we need to talk to"]
        #[doc = " them on behalf of the end user. This API just checks whether we have OAuth"]
        #[doc = " token for the particular user, which is a pre-requisite before user can"]
        #[doc = " create a transfer config."]
        async fn check_valid_creds(
            &self,
            request: tonic::Request<super::CheckValidCredsRequest>,
        ) -> Result<tonic::Response<super::CheckValidCredsResponse>, tonic::Status>;
    }
    #[doc = " The Google BigQuery Data Transfer Service API enables BigQuery users to"]
    #[doc = " configure the transfer of their data from other Google Products into"]
    #[doc = " BigQuery. This service contains methods that are end user exposed. It backs"]
    #[doc = " up the frontend."]
    #[derive(Debug)]
    #[doc(hidden)]
    pub struct DataTransferServiceServer<T: DataTransferService> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: DataTransferService> DataTransferServiceServer<T> {
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
    impl<T, B> Service<http::Request<B>> for DataTransferServiceServer<T>
    where
        T: DataTransferService,
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
            match req . uri ( ) . path ( ) { "/google.cloud.bigquery.datatransfer.v1.DataTransferService/GetDataSource" => { # [ allow ( non_camel_case_types ) ] struct GetDataSourceSvc < T : DataTransferService > ( pub Arc < T > ) ; impl < T : DataTransferService > tonic :: server :: UnaryService < super :: GetDataSourceRequest > for GetDataSourceSvc < T > { type Response = super :: DataSource ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: GetDataSourceRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { inner . get_data_source ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = GetDataSourceSvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } "/google.cloud.bigquery.datatransfer.v1.DataTransferService/ListDataSources" => { # [ allow ( non_camel_case_types ) ] struct ListDataSourcesSvc < T : DataTransferService > ( pub Arc < T > ) ; impl < T : DataTransferService > tonic :: server :: UnaryService < super :: ListDataSourcesRequest > for ListDataSourcesSvc < T > { type Response = super :: ListDataSourcesResponse ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: ListDataSourcesRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { inner . list_data_sources ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = ListDataSourcesSvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } "/google.cloud.bigquery.datatransfer.v1.DataTransferService/CreateTransferConfig" => { # [ allow ( non_camel_case_types ) ] struct CreateTransferConfigSvc < T : DataTransferService > ( pub Arc < T > ) ; impl < T : DataTransferService > tonic :: server :: UnaryService < super :: CreateTransferConfigRequest > for CreateTransferConfigSvc < T > { type Response = super :: TransferConfig ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: CreateTransferConfigRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { inner . create_transfer_config ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = CreateTransferConfigSvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } "/google.cloud.bigquery.datatransfer.v1.DataTransferService/UpdateTransferConfig" => { # [ allow ( non_camel_case_types ) ] struct UpdateTransferConfigSvc < T : DataTransferService > ( pub Arc < T > ) ; impl < T : DataTransferService > tonic :: server :: UnaryService < super :: UpdateTransferConfigRequest > for UpdateTransferConfigSvc < T > { type Response = super :: TransferConfig ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: UpdateTransferConfigRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { inner . update_transfer_config ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = UpdateTransferConfigSvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } "/google.cloud.bigquery.datatransfer.v1.DataTransferService/DeleteTransferConfig" => { # [ allow ( non_camel_case_types ) ] struct DeleteTransferConfigSvc < T : DataTransferService > ( pub Arc < T > ) ; impl < T : DataTransferService > tonic :: server :: UnaryService < super :: DeleteTransferConfigRequest > for DeleteTransferConfigSvc < T > { type Response = ( ) ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: DeleteTransferConfigRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { inner . delete_transfer_config ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = DeleteTransferConfigSvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } "/google.cloud.bigquery.datatransfer.v1.DataTransferService/GetTransferConfig" => { # [ allow ( non_camel_case_types ) ] struct GetTransferConfigSvc < T : DataTransferService > ( pub Arc < T > ) ; impl < T : DataTransferService > tonic :: server :: UnaryService < super :: GetTransferConfigRequest > for GetTransferConfigSvc < T > { type Response = super :: TransferConfig ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: GetTransferConfigRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { inner . get_transfer_config ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = GetTransferConfigSvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } "/google.cloud.bigquery.datatransfer.v1.DataTransferService/ListTransferConfigs" => { # [ allow ( non_camel_case_types ) ] struct ListTransferConfigsSvc < T : DataTransferService > ( pub Arc < T > ) ; impl < T : DataTransferService > tonic :: server :: UnaryService < super :: ListTransferConfigsRequest > for ListTransferConfigsSvc < T > { type Response = super :: ListTransferConfigsResponse ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: ListTransferConfigsRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { inner . list_transfer_configs ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = ListTransferConfigsSvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } "/google.cloud.bigquery.datatransfer.v1.DataTransferService/ScheduleTransferRuns" => { # [ allow ( non_camel_case_types ) ] struct ScheduleTransferRunsSvc < T : DataTransferService > ( pub Arc < T > ) ; impl < T : DataTransferService > tonic :: server :: UnaryService < super :: ScheduleTransferRunsRequest > for ScheduleTransferRunsSvc < T > { type Response = super :: ScheduleTransferRunsResponse ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: ScheduleTransferRunsRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { inner . schedule_transfer_runs ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = ScheduleTransferRunsSvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } "/google.cloud.bigquery.datatransfer.v1.DataTransferService/StartManualTransferRuns" => { # [ allow ( non_camel_case_types ) ] struct StartManualTransferRunsSvc < T : DataTransferService > ( pub Arc < T > ) ; impl < T : DataTransferService > tonic :: server :: UnaryService < super :: StartManualTransferRunsRequest > for StartManualTransferRunsSvc < T > { type Response = super :: StartManualTransferRunsResponse ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: StartManualTransferRunsRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { inner . start_manual_transfer_runs ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = StartManualTransferRunsSvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } "/google.cloud.bigquery.datatransfer.v1.DataTransferService/GetTransferRun" => { # [ allow ( non_camel_case_types ) ] struct GetTransferRunSvc < T : DataTransferService > ( pub Arc < T > ) ; impl < T : DataTransferService > tonic :: server :: UnaryService < super :: GetTransferRunRequest > for GetTransferRunSvc < T > { type Response = super :: TransferRun ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: GetTransferRunRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { inner . get_transfer_run ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = GetTransferRunSvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } "/google.cloud.bigquery.datatransfer.v1.DataTransferService/DeleteTransferRun" => { # [ allow ( non_camel_case_types ) ] struct DeleteTransferRunSvc < T : DataTransferService > ( pub Arc < T > ) ; impl < T : DataTransferService > tonic :: server :: UnaryService < super :: DeleteTransferRunRequest > for DeleteTransferRunSvc < T > { type Response = ( ) ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: DeleteTransferRunRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { inner . delete_transfer_run ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = DeleteTransferRunSvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } "/google.cloud.bigquery.datatransfer.v1.DataTransferService/ListTransferRuns" => { # [ allow ( non_camel_case_types ) ] struct ListTransferRunsSvc < T : DataTransferService > ( pub Arc < T > ) ; impl < T : DataTransferService > tonic :: server :: UnaryService < super :: ListTransferRunsRequest > for ListTransferRunsSvc < T > { type Response = super :: ListTransferRunsResponse ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: ListTransferRunsRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { inner . list_transfer_runs ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = ListTransferRunsSvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } "/google.cloud.bigquery.datatransfer.v1.DataTransferService/ListTransferLogs" => { # [ allow ( non_camel_case_types ) ] struct ListTransferLogsSvc < T : DataTransferService > ( pub Arc < T > ) ; impl < T : DataTransferService > tonic :: server :: UnaryService < super :: ListTransferLogsRequest > for ListTransferLogsSvc < T > { type Response = super :: ListTransferLogsResponse ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: ListTransferLogsRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { inner . list_transfer_logs ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = ListTransferLogsSvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } "/google.cloud.bigquery.datatransfer.v1.DataTransferService/CheckValidCreds" => { # [ allow ( non_camel_case_types ) ] struct CheckValidCredsSvc < T : DataTransferService > ( pub Arc < T > ) ; impl < T : DataTransferService > tonic :: server :: UnaryService < super :: CheckValidCredsRequest > for CheckValidCredsSvc < T > { type Response = super :: CheckValidCredsResponse ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: CheckValidCredsRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { inner . check_valid_creds ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = CheckValidCredsSvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } _ => Box :: pin ( async move { Ok ( http :: Response :: builder ( ) . status ( 200 ) . header ( "grpc-status" , "12" ) . body ( tonic :: body :: BoxBody :: empty ( ) ) . unwrap ( ) ) } ) , }
        }
    }
    impl<T: DataTransferService> Clone for DataTransferServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: DataTransferService> Clone for _Inner<T> {
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
