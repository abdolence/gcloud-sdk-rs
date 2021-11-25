/// Google service account
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GoogleServiceAccount {
    /// Email address of the service account.
    #[prost(string, tag = "1")]
    pub account_email: ::prost::alloc::string::String,
    /// Unique identifier for the service account.
    #[prost(string, tag = "2")]
    pub subject_id: ::prost::alloc::string::String,
}
/// AWS access key (see
/// [AWS Security
/// Credentials](<https://docs.aws.amazon.com/general/latest/gr/aws-security-credentials.html>)).
///
/// For information on our data retention policy for user credentials, see
/// [User credentials](/storage-transfer/docs/data-retention#user-credentials).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AwsAccessKey {
    /// Required. AWS access key ID.
    #[prost(string, tag = "1")]
    pub access_key_id: ::prost::alloc::string::String,
    /// Required. AWS secret access key. This field is not returned in RPC
    /// responses.
    #[prost(string, tag = "2")]
    pub secret_access_key: ::prost::alloc::string::String,
}
/// Azure credentials
///
/// For information on our data retention policy for user credentials, see
/// [User credentials](/storage-transfer/docs/data-retention#user-credentials).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AzureCredentials {
    /// Required. Azure shared access signature (SAS).
    ///
    /// <aside class="note">
    /// <strong>Note:</strong>Copying data from Azure Data Lake
    /// Storage (ADLS) Gen 2 is in \[Preview\](/products/#product-launch-stages).
    /// During Preview, if you are copying data from ADLS Gen 2, you must use an
    /// account SAS.
    /// </aside>
    ///
    /// For more information about SAS, see
    /// [Grant limited access to Azure Storage resources using shared access
    /// signatures
    /// (SAS)](<https://docs.microsoft.com/en-us/azure/storage/common/storage-sas-overview>).
    #[prost(string, tag = "2")]
    pub sas_token: ::prost::alloc::string::String,
}
/// Conditions that determine which objects will be transferred. Applies only
/// to Cloud Data Sources such as S3, Azure, and Cloud Storage.
///
/// The "last modification time" refers to the time of the
/// last change to the object's content or metadata — specifically, this is
/// the `updated` property of Cloud Storage objects, the `LastModified` field
/// of S3 objects, and the `Last-Modified` header of Azure blobs.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ObjectConditions {
    /// If specified, only objects with a "last modification time" before
    /// `NOW` - `min_time_elapsed_since_last_modification` and objects that don't
    ///  have a "last modification time" are transferred.
    ///
    /// For each \[TransferOperation][google.storagetransfer.v1.TransferOperation\]
    /// started by this \[TransferJob][google.storagetransfer.v1.TransferJob\], `NOW`
    /// refers to the \[start_time\]
    /// \[google.storagetransfer.v1.TransferOperation.start_time\] of the
    /// `TransferOperation`.
    #[prost(message, optional, tag = "1")]
    pub min_time_elapsed_since_last_modification: ::core::option::Option<::prost_types::Duration>,
    /// If specified, only objects with a "last modification time" on or after
    /// `NOW` - `max_time_elapsed_since_last_modification` and objects that don't
    /// have a "last modification time" are transferred.
    ///
    /// For each \[TransferOperation][google.storagetransfer.v1.TransferOperation\]
    /// started by this \[TransferJob][google.storagetransfer.v1.TransferJob\],
    /// `NOW` refers to the \[start_time\]
    /// \[google.storagetransfer.v1.TransferOperation.start_time\] of the
    /// `TransferOperation`.
    #[prost(message, optional, tag = "2")]
    pub max_time_elapsed_since_last_modification: ::core::option::Option<::prost_types::Duration>,
    /// If you specify `include_prefixes`, Storage Transfer Service uses the items
    /// in the `include_prefixes` array to determine which objects to include in a
    /// transfer. Objects must start with one of the matching `include_prefixes`
    /// for inclusion in the transfer. If \[exclude_prefixes\]
    /// \[google.storagetransfer.v1.ObjectConditions.exclude_prefixes\] is specified,
    /// objects must not start with any of the `exclude_prefixes` specified for
    /// inclusion in the transfer.
    ///
    /// The following are requirements of `include_prefixes`:
    ///
    ///   * Each include-prefix can contain any sequence of Unicode characters, to
    ///     a max length of 1024 bytes when UTF8-encoded, and must not contain
    ///     Carriage Return or Line Feed characters.  Wildcard matching and regular
    ///     expression matching are not supported.
    ///
    ///   * Each include-prefix must omit the leading slash. For example, to
    ///     include the object `s3://my-aws-bucket/logs/y=2015/requests.gz`,
    ///     specify the include-prefix as `logs/y=2015/requests.gz`.
    ///
    ///   * None of the include-prefix values can be empty, if specified.
    ///
    ///   * Each include-prefix must include a distinct portion of the object
    ///     namespace. No include-prefix may be a prefix of another
    ///     include-prefix.
    ///
    /// The max size of `include_prefixes` is 1000.
    ///
    /// For more information, see [Filtering objects from
    /// transfers](/storage-transfer/docs/filtering-objects-from-transfers).
    #[prost(string, repeated, tag = "3")]
    pub include_prefixes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// If you specify `exclude_prefixes`, Storage Transfer Service uses the items
    /// in the `exclude_prefixes` array to determine which objects to exclude from
    /// a transfer. Objects must not start with one of the matching
    /// `exclude_prefixes` for inclusion in a transfer.
    ///
    /// The following are requirements of `exclude_prefixes`:
    ///
    ///   * Each exclude-prefix can contain any sequence of Unicode characters, to
    ///     a max length of 1024 bytes when UTF8-encoded, and must not contain
    ///     Carriage Return or Line Feed characters.  Wildcard matching and regular
    ///     expression matching are not supported.
    ///
    ///   * Each exclude-prefix must omit the leading slash. For example, to
    ///     exclude the object `s3://my-aws-bucket/logs/y=2015/requests.gz`,
    ///     specify the exclude-prefix as `logs/y=2015/requests.gz`.
    ///
    ///   * None of the exclude-prefix values can be empty, if specified.
    ///
    ///   * Each exclude-prefix must exclude a distinct portion of the object
    ///     namespace. No exclude-prefix may be a prefix of another
    ///     exclude-prefix.
    ///
    ///   * If \[include_prefixes\]
    ///     \[google.storagetransfer.v1.ObjectConditions.include_prefixes\] is
    ///     specified, then each exclude-prefix must start with the value of a path
    ///     explicitly included by `include_prefixes`.
    ///
    /// The max size of `exclude_prefixes` is 1000.
    ///
    /// For more information, see [Filtering objects from
    /// transfers](/storage-transfer/docs/filtering-objects-from-transfers).
    #[prost(string, repeated, tag = "4")]
    pub exclude_prefixes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// If specified, only objects with a "last modification time" on or after
    /// this timestamp and objects that don't have a "last modification time" are
    /// transferred.
    ///
    /// The `last_modified_since` and `last_modified_before` fields can be used
    /// together for chunked data processing. For example, consider a script that
    /// processes each day's worth of data at a time. For that you'd set each
    /// of the fields as follows:
    ///
    /// *  `last_modified_since` to the start of the day
    ///
    /// *  `last_modified_before` to the end of the day
    #[prost(message, optional, tag = "5")]
    pub last_modified_since: ::core::option::Option<::prost_types::Timestamp>,
    /// If specified, only objects with a "last modification time" before this
    /// timestamp and objects that don't have a "last modification time" will be
    /// transferred.
    #[prost(message, optional, tag = "6")]
    pub last_modified_before: ::core::option::Option<::prost_types::Timestamp>,
}
/// In a GcsData resource, an object's name is the Cloud Storage object's
/// name and its "last modification time" refers to the object's `updated`
/// property of Cloud Storage objects, which changes when the content or the
/// metadata of the object is updated.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GcsData {
    /// Required. Cloud Storage bucket name. Must meet
    /// [Bucket Name Requirements](/storage/docs/naming#requirements).
    #[prost(string, tag = "1")]
    pub bucket_name: ::prost::alloc::string::String,
    /// Root path to transfer objects.
    ///
    /// Must be an empty string or full path name that ends with a '/'. This field
    /// is treated as an object prefix. As such, it should generally not begin with
    /// a '/'.
    ///
    /// The root path value must meet
    /// [Object Name Requirements](/storage/docs/naming#objectnames).
    #[prost(string, tag = "3")]
    pub path: ::prost::alloc::string::String,
}
/// An AwsS3Data resource can be a data source, but not a data sink.
/// In an AwsS3Data resource, an object's name is the S3 object's key name.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AwsS3Data {
    /// Required. S3 Bucket name (see
    /// [Creating a
    /// bucket](<https://docs.aws.amazon.com/AmazonS3/latest/dev/create-bucket-get-location-example.html>)).
    #[prost(string, tag = "1")]
    pub bucket_name: ::prost::alloc::string::String,
    /// Input only. AWS access key used to sign the API requests to the AWS S3
    /// bucket. Permissions on the bucket must be granted to the access ID of the
    /// AWS access key. This field is required.
    ///
    /// For information on our data retention policy for user credentials, see
    /// [User credentials](/storage-transfer/docs/data-retention#user-credentials).
    #[prost(message, optional, tag = "2")]
    pub aws_access_key: ::core::option::Option<AwsAccessKey>,
    /// Root path to transfer objects.
    ///
    /// Must be an empty string or full path name that ends with a '/'. This field
    /// is treated as an object prefix. As such, it should generally not begin with
    /// a '/'.
    #[prost(string, tag = "3")]
    pub path: ::prost::alloc::string::String,
    /// Input only. Role arn to support temporary credentials via
    /// AssumeRoleWithWebIdentity.
    ///
    /// When role arn is provided, transfer service will fetch temporary
    /// credentials for the session using AssumeRoleWithWebIdentity call for the
    /// provided role using the \[GoogleServiceAccount\] for this project.
    #[prost(string, tag = "4")]
    pub role_arn: ::prost::alloc::string::String,
}
/// An AzureBlobStorageData resource can be a data source, but not a data sink.
/// An AzureBlobStorageData resource represents one Azure container. The storage
/// account determines the [Azure
/// endpoint](<https://docs.microsoft.com/en-us/azure/storage/common/storage-create-storage-account#storage-account-endpoints>).
/// In an AzureBlobStorageData resource, a blobs's name is the [Azure Blob
/// Storage blob's key
/// name](<https://docs.microsoft.com/en-us/rest/api/storageservices/naming-and-referencing-containers--blobs--and-metadata#blob-names>).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AzureBlobStorageData {
    /// Required. The name of the Azure Storage account.
    #[prost(string, tag = "1")]
    pub storage_account: ::prost::alloc::string::String,
    /// Required. Input only. Credentials used to authenticate API requests to
    /// Azure.
    ///
    /// For information on our data retention policy for user credentials, see
    /// [User credentials](/storage-transfer/docs/data-retention#user-credentials).
    #[prost(message, optional, tag = "2")]
    pub azure_credentials: ::core::option::Option<AzureCredentials>,
    /// Required. The container to transfer from the Azure Storage account.
    #[prost(string, tag = "4")]
    pub container: ::prost::alloc::string::String,
    /// Root path to transfer objects.
    ///
    /// Must be an empty string or full path name that ends with a '/'. This field
    /// is treated as an object prefix. As such, it should generally not begin with
    /// a '/'.
    #[prost(string, tag = "5")]
    pub path: ::prost::alloc::string::String,
}
/// An HttpData resource specifies a list of objects on the web to be transferred
/// over HTTP.  The information of the objects to be transferred is contained in
/// a file referenced by a URL. The first line in the file must be
/// `"TsvHttpData-1.0"`, which specifies the format of the file.  Subsequent
/// lines specify the information of the list of objects, one object per list
/// entry. Each entry has the following tab-delimited fields:
///
/// * **HTTP URL** — The location of the object.
///
/// * **Length** — The size of the object in bytes.
///
/// * **MD5** — The base64-encoded MD5 hash of the object.
///
/// For an example of a valid TSV file, see
/// [Transferring data from
/// URLs](<https://cloud.google.com/storage-transfer/docs/create-url-list>).
///
/// When transferring data based on a URL list, keep the following in mind:
///
/// * When an object located at `http(s)://hostname:port/<URL-path>` is
/// transferred to a data sink, the name of the object at the data sink is
/// `<hostname>/<URL-path>`.
///
/// * If the specified size of an object does not match the actual size of the
/// object fetched, the object will not be transferred.
///
/// * If the specified MD5 does not match the MD5 computed from the transferred
/// bytes, the object transfer will fail.
///
/// * Ensure that each URL you specify is publicly accessible. For
/// example, in Cloud Storage you can
/// [share an object publicly]
/// (/storage/docs/cloud-console#_sharingdata) and get a link to it.
///
/// * Storage Transfer Service obeys `robots.txt` rules and requires the source
/// HTTP server to support `Range` requests and to return a `Content-Length`
/// header in each response.
///
/// * \[ObjectConditions][google.storagetransfer.v1.ObjectConditions\] have no
/// effect when filtering objects to transfer.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HttpData {
    /// Required. The URL that points to the file that stores the object list
    /// entries. This file must allow public access.  Currently, only URLs with
    /// HTTP and HTTPS schemes are supported.
    #[prost(string, tag = "1")]
    pub list_url: ::prost::alloc::string::String,
}
/// TransferOptions define the actions to be performed on objects in a transfer.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransferOptions {
    /// When to overwrite objects that already exist in the sink. The default is
    /// that only objects that are different from the source are ovewritten. If
    /// true, all objects in the sink whose name matches an object in the source
    /// will be overwritten with the source object.
    #[prost(bool, tag = "1")]
    pub overwrite_objects_already_existing_in_sink: bool,
    /// Whether objects that exist only in the sink should be deleted.
    ///
    /// **Note:** This option and \[delete_objects_from_source_after_transfer\]
    /// \[google.storagetransfer.v1.TransferOptions.delete_objects_from_source_after_transfer\]
    /// are mutually exclusive.
    #[prost(bool, tag = "2")]
    pub delete_objects_unique_in_sink: bool,
    /// Whether objects should be deleted from the source after they are
    /// transferred to the sink.
    ///
    /// **Note:** This option and \[delete_objects_unique_in_sink\]
    /// \[google.storagetransfer.v1.TransferOptions.delete_objects_unique_in_sink\]
    /// are mutually exclusive.
    #[prost(bool, tag = "3")]
    pub delete_objects_from_source_after_transfer: bool,
}
/// Configuration for running a transfer.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransferSpec {
    /// Only objects that satisfy these object conditions are included in the set
    /// of data source and data sink objects.  Object conditions based on
    /// objects' "last modification time" do not exclude objects in a data sink.
    #[prost(message, optional, tag = "5")]
    pub object_conditions: ::core::option::Option<ObjectConditions>,
    /// If the option
    /// \[delete_objects_unique_in_sink][google.storagetransfer.v1.TransferOptions.delete_objects_unique_in_sink\]
    /// is `true` and time-based object conditions such as 'last modification time'
    /// are specified, the request fails with an
    /// \[INVALID_ARGUMENT][google.rpc.Code.INVALID_ARGUMENT\] error.
    #[prost(message, optional, tag = "6")]
    pub transfer_options: ::core::option::Option<TransferOptions>,
    /// The write sink for the data.
    #[prost(oneof = "transfer_spec::DataSink", tags = "4")]
    pub data_sink: ::core::option::Option<transfer_spec::DataSink>,
    /// The read source of the data.
    #[prost(oneof = "transfer_spec::DataSource", tags = "1, 2, 3, 8")]
    pub data_source: ::core::option::Option<transfer_spec::DataSource>,
}
/// Nested message and enum types in `TransferSpec`.
pub mod transfer_spec {
    /// The write sink for the data.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum DataSink {
        /// A Cloud Storage data sink.
        #[prost(message, tag = "4")]
        GcsDataSink(super::GcsData),
    }
    /// The read source of the data.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum DataSource {
        /// A Cloud Storage data source.
        #[prost(message, tag = "1")]
        GcsDataSource(super::GcsData),
        /// An AWS S3 data source.
        #[prost(message, tag = "2")]
        AwsS3DataSource(super::AwsS3Data),
        /// An HTTP URL data source.
        #[prost(message, tag = "3")]
        HttpDataSource(super::HttpData),
        /// An Azure Blob Storage data source.
        #[prost(message, tag = "8")]
        AzureBlobStorageDataSource(super::AzureBlobStorageData),
    }
}
/// Transfers can be scheduled to recur or to run just once.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Schedule {
    /// Required. The start date of a transfer. Date boundaries are determined
    /// relative to UTC time. If `schedule_start_date` and
    /// \[start_time_of_day][google.storagetransfer.v1.Schedule.start_time_of_day\]
    /// are in the past relative to the job's creation time, the transfer starts
    /// the day after you schedule the transfer request.
    ///
    /// **Note:** When starting jobs at or near midnight UTC it is possible that
    /// a job will start later than expected. For example, if you send an outbound
    /// request on June 1 one millisecond prior to midnight UTC and the Storage
    /// Transfer Service server receives the request on June 2, then it will create
    /// a TransferJob with `schedule_start_date` set to June 2 and a
    /// `start_time_of_day` set to midnight UTC. The first scheduled
    /// \[TransferOperation][google.storagetransfer.v1.TransferOperation\] will take
    /// place on June 3 at midnight UTC.
    #[prost(message, optional, tag = "1")]
    pub schedule_start_date: ::core::option::Option<super::super::r#type::Date>,
    /// The last day a transfer runs. Date boundaries are determined relative to
    /// UTC time. A job will run once per 24 hours within the following guidelines:
    ///
    /// *   If `schedule_end_date` and
    /// \[schedule_start_date][google.storagetransfer.v1.Schedule.schedule_start_date\]
    /// are the same and in
    ///     the future relative to UTC, the transfer is executed only one time.
    /// *   If `schedule_end_date` is later than `schedule_start_date`  and
    ///     `schedule_end_date` is in the future relative to UTC, the job will
    ///     run each day at
    ///     \[start_time_of_day][google.storagetransfer.v1.Schedule.start_time_of_day\]
    ///     through `schedule_end_date`.
    #[prost(message, optional, tag = "2")]
    pub schedule_end_date: ::core::option::Option<super::super::r#type::Date>,
    /// The time in UTC that a transfer job is scheduled to run. Transfers may
    /// start later than this time.
    ///
    /// If `start_time_of_day` is not specified:
    ///
    /// *   One-time transfers run immediately.
    /// *   Recurring transfers run immediately, and each day at midnight UTC,
    ///     through
    ///     \[schedule_end_date][google.storagetransfer.v1.Schedule.schedule_end_date\].
    ///
    /// If `start_time_of_day` is specified:
    ///
    /// *   One-time transfers run at the specified time.
    /// *   Recurring transfers run at the specified time each day, through
    ///     `schedule_end_date`.
    #[prost(message, optional, tag = "3")]
    pub start_time_of_day: ::core::option::Option<super::super::r#type::TimeOfDay>,
    /// The time in UTC that no further transfer operations are scheduled. Combined
    /// with
    /// \[schedule_end_date][google.storagetransfer.v1.Schedule.schedule_end_date\],
    /// `end_time_of_day` specifies the end date and time for starting new transfer
    /// operations. This field must be greater than or equal to the timestamp
    /// corresponding to the combintation of
    /// \[schedule_start_date][google.storagetransfer.v1.Schedule.schedule_start_date\]
    /// and
    /// \[start_time_of_day][google.storagetransfer.v1.Schedule.start_time_of_day\],
    /// and is subject to the following:
    ///
    /// *   If `end_time_of_day` is not set and `schedule_end_date` is set, then
    ///     a default value of `23:59:59` is used for `end_time_of_day`.
    ///
    /// *   If `end_time_of_day` is set and `schedule_end_date` is not set, then
    ///     \[INVALID_ARGUMENT][google.rpc.Code.INVALID_ARGUMENT\] is returned.
    #[prost(message, optional, tag = "4")]
    pub end_time_of_day: ::core::option::Option<super::super::r#type::TimeOfDay>,
    /// Interval between the start of each scheduled TransferOperation. If
    /// unspecified, the default value is 24 hours. This value may not be less than
    /// 1 hour.
    #[prost(message, optional, tag = "5")]
    pub repeat_interval: ::core::option::Option<::prost_types::Duration>,
}
/// This resource represents the configuration of a transfer job that runs
/// periodically.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransferJob {
    /// A unique name (within the transfer project) assigned when the job is
    /// created.  If this field is empty in a CreateTransferJobRequest, Storage
    /// Transfer Service will assign a unique name. Otherwise, the specified name
    /// is used as the unique name for this job.
    ///
    /// If the specified name is in use by a job, the creation request fails with
    /// an \[ALREADY_EXISTS][google.rpc.Code.ALREADY_EXISTS\] error.
    ///
    /// This name must start with `"transferJobs/"` prefix and end with a letter or
    /// a number, and should be no more than 128 characters. This name must not
    /// start with 'transferJobs/OPI'. 'transferJobs/OPI' is a reserved prefix.
    /// Example:
    /// `"transferJobs/^(?!OPI)\[A-Za-z0-9-._~]*[A-Za-z0-9\]$"`
    ///
    /// Invalid job names will fail with an
    /// \[INVALID_ARGUMENT][google.rpc.Code.INVALID_ARGUMENT\] error.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// A description provided by the user for the job. Its max length is 1024
    /// bytes when Unicode-encoded.
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// The ID of the Google Cloud Platform Project that owns the job.
    #[prost(string, tag = "3")]
    pub project_id: ::prost::alloc::string::String,
    /// Transfer specification.
    #[prost(message, optional, tag = "4")]
    pub transfer_spec: ::core::option::Option<TransferSpec>,
    /// Notification configuration.
    #[prost(message, optional, tag = "11")]
    pub notification_config: ::core::option::Option<NotificationConfig>,
    /// Specifies schedule for the transfer job.
    /// This is an optional field. When the field is not set, the job will never
    /// execute a transfer, unless you invoke RunTransferJob or update the job to
    /// have a non-empty schedule.
    #[prost(message, optional, tag = "5")]
    pub schedule: ::core::option::Option<Schedule>,
    /// Status of the job. This value MUST be specified for
    /// `CreateTransferJobRequests`.
    ///
    /// **Note:** The effect of the new job status takes place during a subsequent
    /// job run. For example, if you change the job status from
    /// \[ENABLED][google.storagetransfer.v1.TransferJob.Status.ENABLED\] to
    /// \[DISABLED][google.storagetransfer.v1.TransferJob.Status.DISABLED\], and an
    /// operation spawned by the transfer is running, the status change would not
    /// affect the current operation.
    #[prost(enumeration = "transfer_job::Status", tag = "6")]
    pub status: i32,
    /// Output only. The time that the transfer job was created.
    #[prost(message, optional, tag = "7")]
    pub creation_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The time that the transfer job was last modified.
    #[prost(message, optional, tag = "8")]
    pub last_modification_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The time that the transfer job was deleted.
    #[prost(message, optional, tag = "9")]
    pub deletion_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The name of the most recently started TransferOperation of this JobConfig.
    /// Present if a TransferOperation has been created for this JobConfig.
    #[prost(string, tag = "12")]
    pub latest_operation_name: ::prost::alloc::string::String,
}
/// Nested message and enum types in `TransferJob`.
pub mod transfer_job {
    /// The status of the transfer job.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Status {
        /// Zero is an illegal value.
        Unspecified = 0,
        /// New transfers will be performed based on the schedule.
        Enabled = 1,
        /// New transfers will not be scheduled.
        Disabled = 2,
        /// This is a soft delete state. After a transfer job is set to this
        /// state, the job and all the transfer executions are subject to
        /// garbage collection. Transfer jobs become eligible for garbage collection
        /// 30 days after their status is set to `DELETED`.
        Deleted = 3,
    }
}
/// An entry describing an error that has occurred.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ErrorLogEntry {
    /// Required. A URL that refers to the target (a data source, a data sink,
    /// or an object) with which the error is associated.
    #[prost(string, tag = "1")]
    pub url: ::prost::alloc::string::String,
    /// A list of messages that carry the error details.
    #[prost(string, repeated, tag = "3")]
    pub error_details: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// A summary of errors by error code, plus a count and sample error log
/// entries.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ErrorSummary {
    /// Required.
    #[prost(enumeration = "super::super::rpc::Code", tag = "1")]
    pub error_code: i32,
    /// Required. Count of this type of error.
    #[prost(int64, tag = "2")]
    pub error_count: i64,
    /// Error samples.
    ///
    /// At most 5 error log entries will be recorded for a given
    /// error code for a single transfer operation.
    #[prost(message, repeated, tag = "3")]
    pub error_log_entries: ::prost::alloc::vec::Vec<ErrorLogEntry>,
}
/// A collection of counters that report the progress of a transfer operation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransferCounters {
    /// Objects found in the data source that are scheduled to be transferred,
    /// excluding any that are filtered based on object conditions or skipped due
    /// to sync.
    #[prost(int64, tag = "1")]
    pub objects_found_from_source: i64,
    /// Bytes found in the data source that are scheduled to be transferred,
    /// excluding any that are filtered based on object conditions or skipped due
    /// to sync.
    #[prost(int64, tag = "2")]
    pub bytes_found_from_source: i64,
    /// Objects found only in the data sink that are scheduled to be deleted.
    #[prost(int64, tag = "3")]
    pub objects_found_only_from_sink: i64,
    /// Bytes found only in the data sink that are scheduled to be deleted.
    #[prost(int64, tag = "4")]
    pub bytes_found_only_from_sink: i64,
    /// Objects in the data source that are not transferred because they already
    /// exist in the data sink.
    #[prost(int64, tag = "5")]
    pub objects_from_source_skipped_by_sync: i64,
    /// Bytes in the data source that are not transferred because they already
    /// exist in the data sink.
    #[prost(int64, tag = "6")]
    pub bytes_from_source_skipped_by_sync: i64,
    /// Objects that are copied to the data sink.
    #[prost(int64, tag = "7")]
    pub objects_copied_to_sink: i64,
    /// Bytes that are copied to the data sink.
    #[prost(int64, tag = "8")]
    pub bytes_copied_to_sink: i64,
    /// Objects that are deleted from the data source.
    #[prost(int64, tag = "9")]
    pub objects_deleted_from_source: i64,
    /// Bytes that are deleted from the data source.
    #[prost(int64, tag = "10")]
    pub bytes_deleted_from_source: i64,
    /// Objects that are deleted from the data sink.
    #[prost(int64, tag = "11")]
    pub objects_deleted_from_sink: i64,
    /// Bytes that are deleted from the data sink.
    #[prost(int64, tag = "12")]
    pub bytes_deleted_from_sink: i64,
    /// Objects in the data source that failed to be transferred or that failed
    /// to be deleted after being transferred.
    #[prost(int64, tag = "13")]
    pub objects_from_source_failed: i64,
    /// Bytes in the data source that failed to be transferred or that failed to
    /// be deleted after being transferred.
    #[prost(int64, tag = "14")]
    pub bytes_from_source_failed: i64,
    /// Objects that failed to be deleted from the data sink.
    #[prost(int64, tag = "15")]
    pub objects_failed_to_delete_from_sink: i64,
    /// Bytes that failed to be deleted from the data sink.
    #[prost(int64, tag = "16")]
    pub bytes_failed_to_delete_from_sink: i64,
}
/// Specification to configure notifications published to Cloud Pub/Sub.
/// Notifications will be published to the customer-provided topic using the
/// following `PubsubMessage.attributes`:
///
/// * `"eventType"`: one of the
/// \[EventType][google.storagetransfer.v1.NotificationConfig.EventType\] values
/// * `"payloadFormat"`: one of the
/// \[PayloadFormat][google.storagetransfer.v1.NotificationConfig.PayloadFormat\]
/// values
/// * `"projectId"`: the
/// \[project_id][google.storagetransfer.v1.TransferOperation.project_id\] of the
/// `TransferOperation`
/// * `"transferJobName"`: the
/// \[transfer_job_name][google.storagetransfer.v1.TransferOperation.transfer_job_name\]
/// of the `TransferOperation`
/// * `"transferOperationName"`: the
/// \[name][google.storagetransfer.v1.TransferOperation.name\] of the
/// `TransferOperation`
///
/// The `PubsubMessage.data` will contain a
/// \[TransferOperation][google.storagetransfer.v1.TransferOperation\] resource
/// formatted according to the specified `PayloadFormat`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NotificationConfig {
    /// Required. The `Topic.name` of the Cloud Pub/Sub topic to which to publish
    /// notifications. Must be of the format: `projects/{project}/topics/{topic}`.
    /// Not matching this format will result in an
    /// \[INVALID_ARGUMENT][google.rpc.Code.INVALID_ARGUMENT\] error.
    #[prost(string, tag = "1")]
    pub pubsub_topic: ::prost::alloc::string::String,
    /// Event types for which a notification is desired. If empty, send
    /// notifications for all event types.
    #[prost(enumeration = "notification_config::EventType", repeated, tag = "2")]
    pub event_types: ::prost::alloc::vec::Vec<i32>,
    /// Required. The desired format of the notification message payloads.
    #[prost(enumeration = "notification_config::PayloadFormat", tag = "3")]
    pub payload_format: i32,
}
/// Nested message and enum types in `NotificationConfig`.
pub mod notification_config {
    /// Enum for specifying event types for which notifications are to be
    /// published.
    ///
    /// Additional event types may be added in the future. Clients should either
    /// safely ignore unrecognized event types or explicitly specify which event
    /// types they are prepared to accept.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum EventType {
        /// Illegal value, to avoid allowing a default.
        Unspecified = 0,
        /// `TransferOperation` completed with status
        /// \[SUCCESS][google.storagetransfer.v1.TransferOperation.Status.SUCCESS\].
        TransferOperationSuccess = 1,
        /// `TransferOperation` completed with status
        /// \[FAILED][google.storagetransfer.v1.TransferOperation.Status.FAILED\].
        TransferOperationFailed = 2,
        /// `TransferOperation` completed with status
        /// \[ABORTED][google.storagetransfer.v1.TransferOperation.Status.ABORTED\].
        TransferOperationAborted = 3,
    }
    /// Enum for specifying the format of a notification message's payload.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum PayloadFormat {
        /// Illegal value, to avoid allowing a default.
        Unspecified = 0,
        /// No payload is included with the notification.
        None = 1,
        /// `TransferOperation` is [formatted as a JSON
        /// response](<https://developers.google.com/protocol-buffers/docs/proto3#json>),
        /// in application/json.
        Json = 2,
    }
}
/// A description of the execution of a transfer.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransferOperation {
    /// A globally unique ID assigned by the system.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The ID of the Google Cloud Platform Project that owns the operation.
    #[prost(string, tag = "2")]
    pub project_id: ::prost::alloc::string::String,
    /// Transfer specification.
    #[prost(message, optional, tag = "3")]
    pub transfer_spec: ::core::option::Option<TransferSpec>,
    /// Notification configuration.
    #[prost(message, optional, tag = "10")]
    pub notification_config: ::core::option::Option<NotificationConfig>,
    /// Start time of this transfer execution.
    #[prost(message, optional, tag = "4")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// End time of this transfer execution.
    #[prost(message, optional, tag = "5")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Status of the transfer operation.
    #[prost(enumeration = "transfer_operation::Status", tag = "6")]
    pub status: i32,
    /// Information about the progress of the transfer operation.
    #[prost(message, optional, tag = "7")]
    pub counters: ::core::option::Option<TransferCounters>,
    /// Summarizes errors encountered with sample error log entries.
    #[prost(message, repeated, tag = "8")]
    pub error_breakdowns: ::prost::alloc::vec::Vec<ErrorSummary>,
    /// The name of the transfer job that triggers this transfer operation.
    #[prost(string, tag = "9")]
    pub transfer_job_name: ::prost::alloc::string::String,
}
/// Nested message and enum types in `TransferOperation`.
pub mod transfer_operation {
    /// The status of a TransferOperation.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Status {
        /// Zero is an illegal value.
        Unspecified = 0,
        /// In progress.
        InProgress = 1,
        /// Paused.
        Paused = 2,
        /// Completed successfully.
        Success = 3,
        /// Terminated due to an unrecoverable failure.
        Failed = 4,
        /// Aborted by the user.
        Aborted = 5,
        /// Temporarily delayed by the system. No user action is required.
        Queued = 6,
    }
}
/// Request passed to GetGoogleServiceAccount.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetGoogleServiceAccountRequest {
    /// Required. The ID of the Google Cloud Platform Console project that the
    /// Google service account is associated with.
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
}
/// Request passed to CreateTransferJob.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateTransferJobRequest {
    /// Required. The job to create.
    #[prost(message, optional, tag = "1")]
    pub transfer_job: ::core::option::Option<TransferJob>,
}
/// Request passed to UpdateTransferJob.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateTransferJobRequest {
    /// Required. The name of job to update.
    #[prost(string, tag = "1")]
    pub job_name: ::prost::alloc::string::String,
    /// Required. The ID of the Google Cloud Platform Console project that owns the
    /// job.
    #[prost(string, tag = "2")]
    pub project_id: ::prost::alloc::string::String,
    /// Required. The job to update. `transferJob` is expected to specify only
    /// four fields:
    /// \[description][google.storagetransfer.v1.TransferJob.description\],
    /// \[transfer_spec][google.storagetransfer.v1.TransferJob.transfer_spec\],
    /// \[notification_config][google.storagetransfer.v1.TransferJob.notification_config\],
    /// and \[status][google.storagetransfer.v1.TransferJob.status\].  An
    /// `UpdateTransferJobRequest` that specifies other fields are rejected with
    /// the error \[INVALID_ARGUMENT][google.rpc.Code.INVALID_ARGUMENT\]. Updating a
    /// job status to
    /// \[DELETED][google.storagetransfer.v1.TransferJob.Status.DELETED\] requires
    /// `storagetransfer.jobs.delete` permissions.
    #[prost(message, optional, tag = "3")]
    pub transfer_job: ::core::option::Option<TransferJob>,
    /// The field mask of the fields in `transferJob` that are to be updated in
    /// this request.  Fields in `transferJob` that can be updated are:
    /// \[description][google.storagetransfer.v1.TransferJob.description\],
    /// \[transfer_spec][google.storagetransfer.v1.TransferJob.transfer_spec\],
    /// \[notification_config][google.storagetransfer.v1.TransferJob.notification_config\],
    /// and \[status][google.storagetransfer.v1.TransferJob.status\].  To update the
    /// `transfer_spec` of the job, a complete transfer specification must be
    /// provided. An incomplete specification missing any required fields is
    /// rejected with the error
    /// \[INVALID_ARGUMENT][google.rpc.Code.INVALID_ARGUMENT\].
    #[prost(message, optional, tag = "4")]
    pub update_transfer_job_field_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request passed to GetTransferJob.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTransferJobRequest {
    /// Required.
    /// The job to get.
    #[prost(string, tag = "1")]
    pub job_name: ::prost::alloc::string::String,
    /// Required. The ID of the Google Cloud Platform Console project that owns the
    /// job.
    #[prost(string, tag = "2")]
    pub project_id: ::prost::alloc::string::String,
}
/// `projectId`, `jobNames`, and `jobStatuses` are query parameters that can
/// be specified when listing transfer jobs.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTransferJobsRequest {
    /// Required. A list of query parameters specified as JSON text in the form of:
    /// `{"projectId":"my_project_id",
    ///  "jobNames":\["jobid1","jobid2",...\],
    ///  "jobStatuses":\["status1","status2",...\]}`
    ///
    /// Since `jobNames` and `jobStatuses` support multiple values, their values
    /// must be specified with array notation. `projectId` is required.
    /// `jobNames` and `jobStatuses` are optional.  The valid values for
    /// `jobStatuses` are case-insensitive:
    /// \[ENABLED][google.storagetransfer.v1.TransferJob.Status.ENABLED\],
    /// \[DISABLED][google.storagetransfer.v1.TransferJob.Status.DISABLED\], and
    /// \[DELETED][google.storagetransfer.v1.TransferJob.Status.DELETED\].
    #[prost(string, tag = "1")]
    pub filter: ::prost::alloc::string::String,
    /// The list page size. The max allowed value is 256.
    #[prost(int32, tag = "4")]
    pub page_size: i32,
    /// The list page token.
    #[prost(string, tag = "5")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response from ListTransferJobs.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTransferJobsResponse {
    /// A list of transfer jobs.
    #[prost(message, repeated, tag = "1")]
    pub transfer_jobs: ::prost::alloc::vec::Vec<TransferJob>,
    /// The list next page token.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request passed to PauseTransferOperation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PauseTransferOperationRequest {
    /// Required. The name of the transfer operation.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request passed to ResumeTransferOperation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResumeTransferOperationRequest {
    /// Required. The name of the transfer operation.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request passed to RunTransferJob.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RunTransferJobRequest {
    /// Required. The name of the transfer job.
    #[prost(string, tag = "1")]
    pub job_name: ::prost::alloc::string::String,
    /// Required. The ID of the Google Cloud Platform Console project that owns the
    /// transfer job.
    #[prost(string, tag = "2")]
    pub project_id: ::prost::alloc::string::String,
}
#[doc = r" Generated client implementations."]
pub mod storage_transfer_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Storage Transfer Service and its protos."]
    #[doc = " Transfers data between between Google Cloud Storage buckets or from a data"]
    #[doc = " source external to Google to a Cloud Storage bucket."]
    #[derive(Debug, Clone)]
    pub struct StorageTransferServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> StorageTransferServiceClient<T>
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
        ) -> StorageTransferServiceClient<InterceptedService<T, F>>
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
            StorageTransferServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Returns the Google service account that is used by Storage Transfer"]
        #[doc = " Service to access buckets in the project where transfers"]
        #[doc = " run or in other projects. Each Google service account is associated"]
        #[doc = " with one Google Cloud Platform Console project. Users"]
        #[doc = " should add this service account to the Google Cloud Storage bucket"]
        #[doc = " ACLs to grant access to Storage Transfer Service. This service"]
        #[doc = " account is created and owned by Storage Transfer Service and can"]
        #[doc = " only be used by Storage Transfer Service."]
        pub async fn get_google_service_account(
            &mut self,
            request: impl tonic::IntoRequest<super::GetGoogleServiceAccountRequest>,
        ) -> Result<tonic::Response<super::GoogleServiceAccount>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.storagetransfer.v1.StorageTransferService/GetGoogleServiceAccount",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a transfer job that runs periodically."]
        pub async fn create_transfer_job(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateTransferJobRequest>,
        ) -> Result<tonic::Response<super::TransferJob>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.storagetransfer.v1.StorageTransferService/CreateTransferJob",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates a transfer job. Updating a job's transfer spec does not affect"]
        #[doc = " transfer operations that are running already."]
        #[doc = ""]
        #[doc = " **Note:** The job's [status][google.storagetransfer.v1.TransferJob.status]"]
        #[doc = " field can be modified using this RPC (for example, to set a job's status to"]
        #[doc = " [DELETED][google.storagetransfer.v1.TransferJob.Status.DELETED],"]
        #[doc = " [DISABLED][google.storagetransfer.v1.TransferJob.Status.DISABLED], or"]
        #[doc = " [ENABLED][google.storagetransfer.v1.TransferJob.Status.ENABLED])."]
        pub async fn update_transfer_job(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateTransferJobRequest>,
        ) -> Result<tonic::Response<super::TransferJob>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.storagetransfer.v1.StorageTransferService/UpdateTransferJob",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets a transfer job."]
        pub async fn get_transfer_job(
            &mut self,
            request: impl tonic::IntoRequest<super::GetTransferJobRequest>,
        ) -> Result<tonic::Response<super::TransferJob>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.storagetransfer.v1.StorageTransferService/GetTransferJob",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists transfer jobs."]
        pub async fn list_transfer_jobs(
            &mut self,
            request: impl tonic::IntoRequest<super::ListTransferJobsRequest>,
        ) -> Result<tonic::Response<super::ListTransferJobsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.storagetransfer.v1.StorageTransferService/ListTransferJobs",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Pauses a transfer operation."]
        pub async fn pause_transfer_operation(
            &mut self,
            request: impl tonic::IntoRequest<super::PauseTransferOperationRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.storagetransfer.v1.StorageTransferService/PauseTransferOperation",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Resumes a transfer operation that is paused."]
        pub async fn resume_transfer_operation(
            &mut self,
            request: impl tonic::IntoRequest<super::ResumeTransferOperationRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.storagetransfer.v1.StorageTransferService/ResumeTransferOperation",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Attempts to start a new TransferOperation for the current TransferJob. A"]
        #[doc = " TransferJob has a maximum of one active TransferOperation. If this method"]
        #[doc = " is called while a TransferOperation is active, an error wil be returned."]
        pub async fn run_transfer_job(
            &mut self,
            request: impl tonic::IntoRequest<super::RunTransferJobRequest>,
        ) -> Result<tonic::Response<super::super::super::longrunning::Operation>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.storagetransfer.v1.StorageTransferService/RunTransferJob",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
