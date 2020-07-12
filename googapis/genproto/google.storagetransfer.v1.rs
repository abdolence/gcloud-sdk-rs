/// Google service account
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GoogleServiceAccount {
    /// Required.
    #[prost(string, tag = "1")]
    pub account_email: std::string::String,
}
/// AWS access key (see
/// [AWS Security
/// Credentials](http://docs.aws.amazon.com/general/latest/gr/aws-security-credentials.html)).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AwsAccessKey {
    /// AWS access key ID.
    /// Required.
    #[prost(string, tag = "1")]
    pub access_key_id: std::string::String,
    /// AWS secret access key. This field is not returned in RPC responses.
    /// Required.
    #[prost(string, tag = "2")]
    pub secret_access_key: std::string::String,
}
/// Conditions that determine which objects will be transferred.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ObjectConditions {
    /// If unspecified, `minTimeElapsedSinceLastModification` takes a zero value
    /// and `maxTimeElapsedSinceLastModification` takes the maximum possible
    /// value of Duration. Objects that satisfy the object conditions
    /// must either have a `lastModificationTime` greater or equal to
    /// `NOW` - `maxTimeElapsedSinceLastModification` and less than
    /// `NOW` - `minTimeElapsedSinceLastModification`, or not have a
    /// `lastModificationTime`.
    #[prost(message, optional, tag = "1")]
    pub min_time_elapsed_since_last_modification: ::std::option::Option<::prost_types::Duration>,
    /// `maxTimeElapsedSinceLastModification` is the complement to
    /// `minTimeElapsedSinceLastModification`.
    #[prost(message, optional, tag = "2")]
    pub max_time_elapsed_since_last_modification: ::std::option::Option<::prost_types::Duration>,
    /// If `includePrefixes` is specified, objects that satisfy the object
    /// conditions must have names that start with one of the `includePrefixes`
    /// and that do not start with any of the `excludePrefixes`. If
    /// `includePrefixes` is not specified, all objects except those that have
    /// names starting with one of the `excludePrefixes` must satisfy the object
    /// conditions.
    ///
    /// Requirements:
    ///
    ///   * Each include-prefix and exclude-prefix can contain any sequence of
    ///     Unicode characters, of max length 1024 bytes when UTF8-encoded, and
    ///     must not contain Carriage Return or Line Feed characters.  Wildcard
    ///     matching and regular expression matching are not supported.
    ///
    ///   * Each include-prefix and exclude-prefix must omit the leading slash.
    ///     For example, to include the `requests.gz` object in a transfer from
    ///     `s3://my-aws-bucket/logs/y=2015/requests.gz`, specify the include
    ///     prefix as `logs/y=2015/requests.gz`.
    ///
    ///   * None of the include-prefix or the exclude-prefix values can be empty,
    ///     if specified.
    ///
    ///   * Each include-prefix must include a distinct portion of the object
    ///     namespace, i.e., no include-prefix may be a prefix of another
    ///     include-prefix.
    ///
    ///   * Each exclude-prefix must exclude a distinct portion of the object
    ///     namespace, i.e., no exclude-prefix may be a prefix of another
    ///     exclude-prefix.
    ///
    ///   * If `includePrefixes` is specified, then each exclude-prefix must start
    ///     with the value of a path explicitly included by `includePrefixes`.
    ///
    /// The max size of `includePrefixes` is 1000.
    #[prost(string, repeated, tag = "3")]
    pub include_prefixes: ::std::vec::Vec<std::string::String>,
    /// `excludePrefixes` must follow the requirements described for
    /// `includePrefixes`.
    ///
    /// The max size of `excludePrefixes` is 1000.
    #[prost(string, repeated, tag = "4")]
    pub exclude_prefixes: ::std::vec::Vec<std::string::String>,
}
/// In a GcsData, an object's name is the Google Cloud Storage object's name and
/// its `lastModificationTime` refers to the object's updated time, which changes
/// when the content or the metadata of the object is updated.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GcsData {
    /// Google Cloud Storage bucket name (see
    /// [Bucket Name
    /// Requirements](https://cloud.google.com/storage/docs/bucket-naming#requirements)).
    /// Required.
    #[prost(string, tag = "1")]
    pub bucket_name: std::string::String,
}
/// An AwsS3Data can be a data source, but not a data sink.
/// In an AwsS3Data, an object's name is the S3 object's key name.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AwsS3Data {
    /// S3 Bucket name (see
    /// [Creating a
    /// bucket](http://docs.aws.amazon.com/AmazonS3/latest/dev/create-bucket-get-location-example.html)).
    /// Required.
    #[prost(string, tag = "1")]
    pub bucket_name: std::string::String,
    /// AWS access key used to sign the API requests to the AWS S3 bucket.
    /// Permissions on the bucket must be granted to the access ID of the
    /// AWS access key.
    /// Required.
    #[prost(message, optional, tag = "2")]
    pub aws_access_key: ::std::option::Option<AwsAccessKey>,
}
/// An HttpData specifies a list of objects on the web to be transferred over
/// HTTP.  The information of the objects to be transferred is contained in a
/// file referenced by a URL. The first line in the file must be
/// "TsvHttpData-1.0", which specifies the format of the file.  Subsequent lines
/// specify the information of the list of objects, one object per list entry.
/// Each entry has the following tab-delimited fields:
///
/// * HTTP URL - The location of the object.
///
/// * Length - The size of the object in bytes.
///
/// * MD5 - The base64-encoded MD5 hash of the object.
///
/// For an example of a valid TSV file, see
/// [Transferring data from
/// URLs](https://cloud.google.com/storage/transfer/create-url-list).
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
/// bytes, the object transfer will fail. For more information, see
/// [Generating MD5 hashes](https://cloud.google.com/storage/transfer/#md5)
///
/// * Ensure that each URL you specify is publicly accessible. For
/// example, in Google Cloud Storage you can
/// [share an object publicly]
/// (https://cloud.google.com/storage/docs/cloud-console#_sharingdata) and get
/// a link to it.
///
/// * Storage Transfer Service obeys `robots.txt` rules and requires the source
/// HTTP server to support `Range` requests and to return a `Content-Length`
/// header in each response.
///
/// * [ObjectConditions](#ObjectConditions) have no effect when filtering objects
/// to transfer.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HttpData {
    /// The URL that points to the file that stores the object list entries.
    /// This file must allow public access.  Currently, only URLs with HTTP and
    /// HTTPS schemes are supported.
    /// Required.
    #[prost(string, tag = "1")]
    pub list_url: std::string::String,
}
/// TransferOptions uses three boolean parameters to define the actions
/// to be performed on objects in a transfer.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransferOptions {
    /// Whether overwriting objects that already exist in the sink is allowed.
    #[prost(bool, tag = "1")]
    pub overwrite_objects_already_existing_in_sink: bool,
    /// Whether objects that exist only in the sink should be deleted.  Note that
    /// this option and `deleteObjectsFromSourceAfterTransfer` are mutually
    /// exclusive.
    #[prost(bool, tag = "2")]
    pub delete_objects_unique_in_sink: bool,
    /// Whether objects should be deleted from the source after they are
    /// transferred to the sink.  Note that this option and
    /// `deleteObjectsUniqueInSink` are mutually exclusive.
    #[prost(bool, tag = "3")]
    pub delete_objects_from_source_after_transfer: bool,
}
/// Configuration for running a transfer.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransferSpec {
    /// Only objects that satisfy these object conditions are included in the set
    /// of data source and data sink objects.  Object conditions based on
    /// objects' `lastModificationTime` do not exclude objects in a data sink.
    #[prost(message, optional, tag = "5")]
    pub object_conditions: ::std::option::Option<ObjectConditions>,
    /// If the option `deleteObjectsUniqueInSink` is `true`, object conditions
    /// based on objects' `lastModificationTime` are ignored and do not exclude
    /// objects in a data source or a data sink.
    #[prost(message, optional, tag = "6")]
    pub transfer_options: ::std::option::Option<TransferOptions>,
    /// The read source of the data.
    #[prost(oneof = "transfer_spec::DataSource", tags = "1, 2, 3")]
    pub data_source: ::std::option::Option<transfer_spec::DataSource>,
    /// The write sink for the data.
    #[prost(oneof = "transfer_spec::DataSink", tags = "4")]
    pub data_sink: ::std::option::Option<transfer_spec::DataSink>,
}
pub mod transfer_spec {
    /// The read source of the data.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum DataSource {
        /// A Google Cloud Storage data source.
        #[prost(message, tag = "1")]
        GcsDataSource(super::GcsData),
        /// An AWS S3 data source.
        #[prost(message, tag = "2")]
        AwsS3DataSource(super::AwsS3Data),
        /// An HTTP URL data source.
        #[prost(message, tag = "3")]
        HttpDataSource(super::HttpData),
    }
    /// The write sink for the data.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum DataSink {
        /// A Google Cloud Storage data sink.
        #[prost(message, tag = "4")]
        GcsDataSink(super::GcsData),
    }
}
/// Transfers can be scheduled to recur or to run just once.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Schedule {
    /// The first day the recurring transfer is scheduled to run. If
    /// `scheduleStartDate` is in the past, the transfer will run for the first
    /// time on the following day.
    /// Required.
    #[prost(message, optional, tag = "1")]
    pub schedule_start_date: ::std::option::Option<super::super::r#type::Date>,
    /// The last day the recurring transfer will be run. If `scheduleEndDate`
    /// is the same as `scheduleStartDate`, the transfer will be executed only
    /// once.
    #[prost(message, optional, tag = "2")]
    pub schedule_end_date: ::std::option::Option<super::super::r#type::Date>,
    /// The time in UTC at which the transfer will be scheduled to start in a day.
    /// Transfers may start later than this time. If not specified, recurring and
    /// one-time transfers that are scheduled to run today will run immediately;
    /// recurring transfers that are scheduled to run on a future date will start
    /// at approximately midnight UTC on that date. Note that when configuring a
    /// transfer with the Cloud Platform Console, the transfer's start time in a
    /// day is specified in your local timezone.
    #[prost(message, optional, tag = "3")]
    pub start_time_of_day: ::std::option::Option<super::super::r#type::TimeOfDay>,
}
/// This resource represents the configuration of a transfer job that runs
/// periodically.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransferJob {
    /// A globally unique name assigned by Storage Transfer Service when the
    /// job is created. This field should be left empty in requests to create a new
    /// transfer job; otherwise, the requests result in an `INVALID_ARGUMENT`
    /// error.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// A description provided by the user for the job. Its max length is 1024
    /// bytes when Unicode-encoded.
    #[prost(string, tag = "2")]
    pub description: std::string::String,
    /// The ID of the Google Cloud Platform Console project that owns the job.
    #[prost(string, tag = "3")]
    pub project_id: std::string::String,
    /// Transfer specification.
    #[prost(message, optional, tag = "4")]
    pub transfer_spec: ::std::option::Option<TransferSpec>,
    /// Schedule specification.
    #[prost(message, optional, tag = "5")]
    pub schedule: ::std::option::Option<Schedule>,
    /// Status of the job. This value MUST be specified for
    /// `CreateTransferJobRequests`.
    ///
    /// NOTE: The effect of the new job status takes place during a subsequent job
    /// run. For example, if you change the job status from `ENABLED` to
    /// `DISABLED`, and an operation spawned by the transfer is running, the status
    /// change would not affect the current operation.
    #[prost(enumeration = "transfer_job::Status", tag = "6")]
    pub status: i32,
    /// This field cannot be changed by user requests.
    #[prost(message, optional, tag = "7")]
    pub creation_time: ::std::option::Option<::prost_types::Timestamp>,
    /// This field cannot be changed by user requests.
    #[prost(message, optional, tag = "8")]
    pub last_modification_time: ::std::option::Option<::prost_types::Timestamp>,
    /// This field cannot be changed by user requests.
    #[prost(message, optional, tag = "9")]
    pub deletion_time: ::std::option::Option<::prost_types::Timestamp>,
}
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
        /// garbage collection.
        Deleted = 3,
    }
}
/// An entry describing an error that has occurred.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ErrorLogEntry {
    /// A URL that refers to the target (a data source, a data sink,
    /// or an object) with which the error is associated.
    /// Required.
    #[prost(string, tag = "1")]
    pub url: std::string::String,
    /// A list of messages that carry the error details.
    #[prost(string, repeated, tag = "3")]
    pub error_details: ::std::vec::Vec<std::string::String>,
}
/// A summary of errors by error code, plus a count and sample error log
/// entries.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ErrorSummary {
    /// Required.
    #[prost(enumeration = "super::super::rpc::Code", tag = "1")]
    pub error_code: i32,
    /// Count of this type of error.
    /// Required.
    #[prost(int64, tag = "2")]
    pub error_count: i64,
    /// Error samples.
    #[prost(message, repeated, tag = "3")]
    pub error_log_entries: ::std::vec::Vec<ErrorLogEntry>,
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
    /// Objects in the data source that failed during the transfer.
    #[prost(int64, tag = "13")]
    pub objects_from_source_failed: i64,
    /// Bytes in the data source that failed during the transfer.
    #[prost(int64, tag = "14")]
    pub bytes_from_source_failed: i64,
    /// Objects that failed to be deleted from the data sink.
    #[prost(int64, tag = "15")]
    pub objects_failed_to_delete_from_sink: i64,
    /// Bytes that failed to be deleted from the data sink.
    #[prost(int64, tag = "16")]
    pub bytes_failed_to_delete_from_sink: i64,
}
/// A description of the execution of a transfer.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransferOperation {
    /// A globally unique ID assigned by the system.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// The ID of the Google Cloud Platform Console project that owns the
    /// operation. Required.
    #[prost(string, tag = "2")]
    pub project_id: std::string::String,
    /// Transfer specification.
    /// Required.
    #[prost(message, optional, tag = "3")]
    pub transfer_spec: ::std::option::Option<TransferSpec>,
    /// Start time of this transfer execution.
    #[prost(message, optional, tag = "4")]
    pub start_time: ::std::option::Option<::prost_types::Timestamp>,
    /// End time of this transfer execution.
    #[prost(message, optional, tag = "5")]
    pub end_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Status of the transfer operation.
    #[prost(enumeration = "transfer_operation::Status", tag = "6")]
    pub status: i32,
    /// Information about the progress of the transfer operation.
    #[prost(message, optional, tag = "7")]
    pub counters: ::std::option::Option<TransferCounters>,
    /// Summarizes errors encountered with sample error log entries.
    #[prost(message, repeated, tag = "8")]
    pub error_breakdowns: ::std::vec::Vec<ErrorSummary>,
    /// The name of the transfer job that triggers this transfer operation.
    #[prost(string, tag = "9")]
    pub transfer_job_name: std::string::String,
}
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
    }
}
/// Request passed to GetGoogleServiceAccount.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetGoogleServiceAccountRequest {
    /// The ID of the Google Cloud Platform Console project that the Google service
    /// account is associated with.
    /// Required.
    #[prost(string, tag = "1")]
    pub project_id: std::string::String,
}
/// Request passed to CreateTransferJob.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateTransferJobRequest {
    /// The job to create.
    /// Required.
    #[prost(message, optional, tag = "1")]
    pub transfer_job: ::std::option::Option<TransferJob>,
}
/// Request passed to UpdateTransferJob.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateTransferJobRequest {
    /// The name of job to update.
    /// Required.
    #[prost(string, tag = "1")]
    pub job_name: std::string::String,
    /// The ID of the Google Cloud Platform Console project that owns the job.
    /// Required.
    #[prost(string, tag = "2")]
    pub project_id: std::string::String,
    /// The job to update. `transferJob` is expected to specify only three fields:
    /// `description`, `transferSpec`, and `status`.  An UpdateTransferJobRequest
    /// that specifies other fields will be rejected with an error
    /// `INVALID_ARGUMENT`.
    /// Required.
    #[prost(message, optional, tag = "3")]
    pub transfer_job: ::std::option::Option<TransferJob>,
    /// The field mask of the fields in `transferJob` that are to be updated in
    /// this request.  Fields in `transferJob` that can be updated are:
    /// `description`, `transferSpec`, and `status`.  To update the `transferSpec`
    /// of the job, a complete transfer specification has to be provided. An
    /// incomplete specification which misses any required fields will be rejected
    /// with the error `INVALID_ARGUMENT`.
    #[prost(message, optional, tag = "4")]
    pub update_transfer_job_field_mask: ::std::option::Option<::prost_types::FieldMask>,
}
/// Request passed to GetTransferJob.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTransferJobRequest {
    /// The job to get.
    /// Required.
    #[prost(string, tag = "1")]
    pub job_name: std::string::String,
    /// The ID of the Google Cloud Platform Console project that owns the job.
    /// Required.
    #[prost(string, tag = "2")]
    pub project_id: std::string::String,
}
/// `project_id`, `job_names`, and `job_statuses` are query parameters that can
/// be specified when listing transfer jobs.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTransferJobsRequest {
    /// A list of query parameters specified as JSON text in the form of
    /// {"project_id":"my_project_id",
    /// "job_names":["jobid1","jobid2",...],
    /// "job_statuses":["status1","status2",...]}.
    /// Since `job_names` and `job_statuses` support multiple values, their values
    /// must be specified with array notation. `project_id` is required.
    /// `job_names` and `job_statuses` are optional.  The valid values for
    /// `job_statuses` are case-insensitive: `ENABLED`, `DISABLED`, and `DELETED`.
    #[prost(string, tag = "1")]
    pub filter: std::string::String,
    /// The list page size. The max allowed value is 256.
    #[prost(int32, tag = "4")]
    pub page_size: i32,
    /// The list page token.
    #[prost(string, tag = "5")]
    pub page_token: std::string::String,
}
/// Response from ListTransferJobs.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTransferJobsResponse {
    /// A list of transfer jobs.
    #[prost(message, repeated, tag = "1")]
    pub transfer_jobs: ::std::vec::Vec<TransferJob>,
    /// The list next page token.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
}
/// Request passed to PauseTransferOperation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PauseTransferOperationRequest {
    /// The name of the transfer operation.
    /// Required.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Request passed to ResumeTransferOperation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResumeTransferOperationRequest {
    /// The name of the transfer operation.
    /// Required.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
#[doc = r" Generated client implementations."]
pub mod storage_transfer_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Transfers data between between Google Cloud Storage buckets or from a data"]
    #[doc = " source external to Google to a Cloud Storage bucket."]
    pub struct StorageTransferServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> StorageTransferServiceClient<T>
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
        #[doc = " transfer operations that are running already. Updating the scheduling"]
        #[doc = " of a job is not allowed."]
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
    }
    impl<T: Clone> Clone for StorageTransferServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for StorageTransferServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "StorageTransferServiceClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod storage_transfer_service_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with StorageTransferServiceServer."]
    #[async_trait]
    pub trait StorageTransferService: Send + Sync + 'static {
        #[doc = " Returns the Google service account that is used by Storage Transfer"]
        #[doc = " Service to access buckets in the project where transfers"]
        #[doc = " run or in other projects. Each Google service account is associated"]
        #[doc = " with one Google Cloud Platform Console project. Users"]
        #[doc = " should add this service account to the Google Cloud Storage bucket"]
        #[doc = " ACLs to grant access to Storage Transfer Service. This service"]
        #[doc = " account is created and owned by Storage Transfer Service and can"]
        #[doc = " only be used by Storage Transfer Service."]
        async fn get_google_service_account(
            &self,
            request: tonic::Request<super::GetGoogleServiceAccountRequest>,
        ) -> Result<tonic::Response<super::GoogleServiceAccount>, tonic::Status>;
        #[doc = " Creates a transfer job that runs periodically."]
        async fn create_transfer_job(
            &self,
            request: tonic::Request<super::CreateTransferJobRequest>,
        ) -> Result<tonic::Response<super::TransferJob>, tonic::Status>;
        #[doc = " Updates a transfer job. Updating a job's transfer spec does not affect"]
        #[doc = " transfer operations that are running already. Updating the scheduling"]
        #[doc = " of a job is not allowed."]
        async fn update_transfer_job(
            &self,
            request: tonic::Request<super::UpdateTransferJobRequest>,
        ) -> Result<tonic::Response<super::TransferJob>, tonic::Status>;
        #[doc = " Gets a transfer job."]
        async fn get_transfer_job(
            &self,
            request: tonic::Request<super::GetTransferJobRequest>,
        ) -> Result<tonic::Response<super::TransferJob>, tonic::Status>;
        #[doc = " Lists transfer jobs."]
        async fn list_transfer_jobs(
            &self,
            request: tonic::Request<super::ListTransferJobsRequest>,
        ) -> Result<tonic::Response<super::ListTransferJobsResponse>, tonic::Status>;
        #[doc = " Pauses a transfer operation."]
        async fn pause_transfer_operation(
            &self,
            request: tonic::Request<super::PauseTransferOperationRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status>;
        #[doc = " Resumes a transfer operation that is paused."]
        async fn resume_transfer_operation(
            &self,
            request: tonic::Request<super::ResumeTransferOperationRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status>;
    }
    #[doc = " Transfers data between between Google Cloud Storage buckets or from a data"]
    #[doc = " source external to Google to a Cloud Storage bucket."]
    #[derive(Debug)]
    #[doc(hidden)]
    pub struct StorageTransferServiceServer<T: StorageTransferService> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: StorageTransferService> StorageTransferServiceServer<T> {
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
    impl<T, B> Service<http::Request<B>> for StorageTransferServiceServer<T>
    where
        T: StorageTransferService,
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
                "/google.storagetransfer.v1.StorageTransferService/GetGoogleServiceAccount" => {
                    #[allow(non_camel_case_types)]
                    struct GetGoogleServiceAccountSvc<T: StorageTransferService>(pub Arc<T>);
                    impl<T: StorageTransferService>
                        tonic::server::UnaryService<super::GetGoogleServiceAccountRequest>
                        for GetGoogleServiceAccountSvc<T>
                    {
                        type Response = super::GoogleServiceAccount;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetGoogleServiceAccountRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { inner.get_google_service_account(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetGoogleServiceAccountSvc(inner);
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
                "/google.storagetransfer.v1.StorageTransferService/CreateTransferJob" => {
                    #[allow(non_camel_case_types)]
                    struct CreateTransferJobSvc<T: StorageTransferService>(pub Arc<T>);
                    impl<T: StorageTransferService>
                        tonic::server::UnaryService<super::CreateTransferJobRequest>
                        for CreateTransferJobSvc<T>
                    {
                        type Response = super::TransferJob;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateTransferJobRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.create_transfer_job(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = CreateTransferJobSvc(inner);
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
                "/google.storagetransfer.v1.StorageTransferService/UpdateTransferJob" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateTransferJobSvc<T: StorageTransferService>(pub Arc<T>);
                    impl<T: StorageTransferService>
                        tonic::server::UnaryService<super::UpdateTransferJobRequest>
                        for UpdateTransferJobSvc<T>
                    {
                        type Response = super::TransferJob;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateTransferJobRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.update_transfer_job(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = UpdateTransferJobSvc(inner);
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
                "/google.storagetransfer.v1.StorageTransferService/GetTransferJob" => {
                    #[allow(non_camel_case_types)]
                    struct GetTransferJobSvc<T: StorageTransferService>(pub Arc<T>);
                    impl<T: StorageTransferService>
                        tonic::server::UnaryService<super::GetTransferJobRequest>
                        for GetTransferJobSvc<T>
                    {
                        type Response = super::TransferJob;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetTransferJobRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.get_transfer_job(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetTransferJobSvc(inner);
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
                "/google.storagetransfer.v1.StorageTransferService/ListTransferJobs" => {
                    #[allow(non_camel_case_types)]
                    struct ListTransferJobsSvc<T: StorageTransferService>(pub Arc<T>);
                    impl<T: StorageTransferService>
                        tonic::server::UnaryService<super::ListTransferJobsRequest>
                        for ListTransferJobsSvc<T>
                    {
                        type Response = super::ListTransferJobsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListTransferJobsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.list_transfer_jobs(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ListTransferJobsSvc(inner);
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
                "/google.storagetransfer.v1.StorageTransferService/PauseTransferOperation" => {
                    #[allow(non_camel_case_types)]
                    struct PauseTransferOperationSvc<T: StorageTransferService>(pub Arc<T>);
                    impl<T: StorageTransferService>
                        tonic::server::UnaryService<super::PauseTransferOperationRequest>
                        for PauseTransferOperationSvc<T>
                    {
                        type Response = ();
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PauseTransferOperationRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.pause_transfer_operation(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = PauseTransferOperationSvc(inner);
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
                "/google.storagetransfer.v1.StorageTransferService/ResumeTransferOperation" => {
                    #[allow(non_camel_case_types)]
                    struct ResumeTransferOperationSvc<T: StorageTransferService>(pub Arc<T>);
                    impl<T: StorageTransferService>
                        tonic::server::UnaryService<super::ResumeTransferOperationRequest>
                        for ResumeTransferOperationSvc<T>
                    {
                        type Response = ();
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ResumeTransferOperationRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.resume_transfer_operation(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ResumeTransferOperationSvc(inner);
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
    impl<T: StorageTransferService> Clone for StorageTransferServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: StorageTransferService> Clone for _Inner<T> {
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
