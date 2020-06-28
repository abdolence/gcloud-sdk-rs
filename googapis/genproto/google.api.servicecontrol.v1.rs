/// Defines the errors to be returned in
/// [google.api.servicecontrol.v1.CheckResponse.check_errors][google.api.servicecontrol.v1.CheckResponse.check_errors].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckError {
    /// The error code.
    #[prost(enumeration = "check_error::Code", tag = "1")]
    pub code: i32,
    /// Subject to whom this error applies. See the specific code enum for more
    /// details on this field. For example:
    ///
    /// - "project:<project-id or project-number>"
    /// - "folder:<folder-id>"
    /// - "organization:<organization-id>"
    #[prost(string, tag = "4")]
    pub subject: std::string::String,
    /// Free-form text providing details on the error cause of the error.
    #[prost(string, tag = "2")]
    pub detail: std::string::String,
    /// Contains public information about the check error. If available,
    /// `status.code` will be non zero and client can propagate it out as public
    /// error.
    #[prost(message, optional, tag = "3")]
    pub status: ::std::option::Option<super::super::super::rpc::Status>,
}
pub mod check_error {
    /// Error codes for Check responses.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Code {
        /// This is never used in `CheckResponse`.
        ErrorCodeUnspecified = 0,
        /// The consumer's project id, network container, or resource container was
        /// not found. Same as
        /// [google.rpc.Code.NOT_FOUND][google.rpc.Code.NOT_FOUND].
        NotFound = 5,
        /// The consumer doesn't have access to the specified resource.
        /// Same as
        /// [google.rpc.Code.PERMISSION_DENIED][google.rpc.Code.PERMISSION_DENIED].
        PermissionDenied = 7,
        /// Quota check failed. Same as
        /// [google.rpc.Code.RESOURCE_EXHAUSTED][google.rpc.Code.RESOURCE_EXHAUSTED].
        ResourceExhausted = 8,
        /// The consumer hasn't activated the service.
        ServiceNotActivated = 104,
        /// The consumer cannot access the service because billing is disabled.
        BillingDisabled = 107,
        /// The consumer's project has been marked as deleted (soft deletion).
        ProjectDeleted = 108,
        /// The consumer's project number or id does not represent a valid project.
        ProjectInvalid = 114,
        /// The input consumer info does not represent a valid consumer folder or
        /// organization.
        ConsumerInvalid = 125,
        /// The IP address of the consumer is invalid for the specific consumer
        /// project.
        IpAddressBlocked = 109,
        /// The referer address of the consumer request is invalid for the specific
        /// consumer project.
        RefererBlocked = 110,
        /// The client application of the consumer request is invalid for the
        /// specific consumer project.
        ClientAppBlocked = 111,
        /// The API targeted by this request is invalid for the specified consumer
        /// project.
        ApiTargetBlocked = 122,
        /// The consumer's API key is invalid.
        ApiKeyInvalid = 105,
        /// The consumer's API Key has expired.
        ApiKeyExpired = 112,
        /// The consumer's API Key was not found in config record.
        ApiKeyNotFound = 113,
        /// The credential in the request can not be verified.
        InvalidCredential = 123,
        /// The backend server for looking up project id/number is unavailable.
        NamespaceLookupUnavailable = 300,
        /// The backend server for checking service status is unavailable.
        ServiceStatusUnavailable = 301,
        /// The backend server for checking billing status is unavailable.
        BillingStatusUnavailable = 302,
        /// Cloud Resource Manager backend server is unavailable.
        CloudResourceManagerBackendUnavailable = 305,
    }
}
/// Distribution represents a frequency distribution of double-valued sample
/// points. It contains the size of the population of sample points plus
/// additional optional information:
///
///   - the arithmetic mean of the samples
///   - the minimum and maximum of the samples
///   - the sum-squared-deviation of the samples, used to compute variance
///   - a histogram of the values of the sample points
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Distribution {
    /// The total number of samples in the distribution. Must be >= 0.
    #[prost(int64, tag = "1")]
    pub count: i64,
    /// The arithmetic mean of the samples in the distribution. If `count` is
    /// zero then this field must be zero.
    #[prost(double, tag = "2")]
    pub mean: f64,
    /// The minimum of the population of values. Ignored if `count` is zero.
    #[prost(double, tag = "3")]
    pub minimum: f64,
    /// The maximum of the population of values. Ignored if `count` is zero.
    #[prost(double, tag = "4")]
    pub maximum: f64,
    /// The sum of squared deviations from the mean:
    ///   Sum[i=1..count]((x_i - mean)^2)
    /// where each x_i is a sample values. If `count` is zero then this field
    /// must be zero, otherwise validation of the request fails.
    #[prost(double, tag = "5")]
    pub sum_of_squared_deviation: f64,
    /// The number of samples in each histogram bucket. `bucket_counts` are
    /// optional. If present, they must sum to the `count` value.
    ///
    /// The buckets are defined below in `bucket_option`. There are N buckets.
    /// `bucket_counts[0]` is the number of samples in the underflow bucket.
    /// `bucket_counts[1]` to `bucket_counts[N-1]` are the numbers of samples
    /// in each of the finite buckets. And `bucket_counts[N] is the number
    /// of samples in the overflow bucket. See the comments of `bucket_option`
    /// below for more details.
    ///
    /// Any suffix of trailing zeros may be omitted.
    #[prost(int64, repeated, tag = "6")]
    pub bucket_counts: ::std::vec::Vec<i64>,
    /// Defines the buckets in the histogram. `bucket_option` and `bucket_counts`
    /// must be both set, or both unset.
    ///
    /// Buckets are numbered in the range of [0, N], with a total of N+1 buckets.
    /// There must be at least two buckets (a single-bucket histogram gives
    /// no information that isn't already provided by `count`).
    ///
    /// The first bucket is the underflow bucket which has a lower bound
    /// of -inf. The last bucket is the overflow bucket which has an
    /// upper bound of +inf. All other buckets (if any) are called "finite"
    /// buckets because they have finite lower and upper bounds. As described
    /// below, there are three ways to define the finite buckets.
    ///
    ///   (1) Buckets with constant width.
    ///   (2) Buckets with exponentially growing widths.
    ///   (3) Buckets with arbitrary user-provided widths.
    ///
    /// In all cases, the buckets cover the entire real number line (-inf,
    /// +inf). Bucket upper bounds are exclusive and lower bounds are
    /// inclusive. The upper bound of the underflow bucket is equal to the
    /// lower bound of the smallest finite bucket; the lower bound of the
    /// overflow bucket is equal to the upper bound of the largest finite
    /// bucket.
    #[prost(oneof = "distribution::BucketOption", tags = "7, 8, 9")]
    pub bucket_option: ::std::option::Option<distribution::BucketOption>,
}
pub mod distribution {
    /// Describing buckets with constant width.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct LinearBuckets {
        /// The number of finite buckets. With the underflow and overflow buckets,
        /// the total number of buckets is `num_finite_buckets` + 2.
        /// See comments on `bucket_options` for details.
        #[prost(int32, tag = "1")]
        pub num_finite_buckets: i32,
        /// The i'th linear bucket covers the interval
        ///   [offset + (i-1) * width, offset + i * width)
        /// where i ranges from 1 to num_finite_buckets, inclusive.
        /// Must be strictly positive.
        #[prost(double, tag = "2")]
        pub width: f64,
        /// The i'th linear bucket covers the interval
        ///   [offset + (i-1) * width, offset + i * width)
        /// where i ranges from 1 to num_finite_buckets, inclusive.
        #[prost(double, tag = "3")]
        pub offset: f64,
    }
    /// Describing buckets with exponentially growing width.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ExponentialBuckets {
        /// The number of finite buckets. With the underflow and overflow buckets,
        /// the total number of buckets is `num_finite_buckets` + 2.
        /// See comments on `bucket_options` for details.
        #[prost(int32, tag = "1")]
        pub num_finite_buckets: i32,
        /// The i'th exponential bucket covers the interval
        ///   [scale * growth_factor^(i-1), scale * growth_factor^i)
        /// where i ranges from 1 to num_finite_buckets inclusive.
        /// Must be larger than 1.0.
        #[prost(double, tag = "2")]
        pub growth_factor: f64,
        /// The i'th exponential bucket covers the interval
        ///   [scale * growth_factor^(i-1), scale * growth_factor^i)
        /// where i ranges from 1 to num_finite_buckets inclusive.
        /// Must be > 0.
        #[prost(double, tag = "3")]
        pub scale: f64,
    }
    /// Describing buckets with arbitrary user-provided width.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ExplicitBuckets {
        /// 'bound' is a list of strictly increasing boundaries between
        /// buckets. Note that a list of length N-1 defines N buckets because
        /// of fenceposting. See comments on `bucket_options` for details.
        ///
        /// The i'th finite bucket covers the interval
        ///   [bound[i-1], bound[i])
        /// where i ranges from 1 to bound_size() - 1. Note that there are no
        /// finite buckets at all if 'bound' only contains a single element; in
        /// that special case the single bound defines the boundary between the
        /// underflow and overflow buckets.
        ///
        /// bucket number                   lower bound    upper bound
        ///  i == 0 (underflow)              -inf           bound[i]
        ///  0 < i < bound_size()            bound[i-1]     bound[i]
        ///  i == bound_size() (overflow)    bound[i-1]     +inf
        #[prost(double, repeated, tag = "1")]
        pub bounds: ::std::vec::Vec<f64>,
    }
    /// Defines the buckets in the histogram. `bucket_option` and `bucket_counts`
    /// must be both set, or both unset.
    ///
    /// Buckets are numbered in the range of [0, N], with a total of N+1 buckets.
    /// There must be at least two buckets (a single-bucket histogram gives
    /// no information that isn't already provided by `count`).
    ///
    /// The first bucket is the underflow bucket which has a lower bound
    /// of -inf. The last bucket is the overflow bucket which has an
    /// upper bound of +inf. All other buckets (if any) are called "finite"
    /// buckets because they have finite lower and upper bounds. As described
    /// below, there are three ways to define the finite buckets.
    ///
    ///   (1) Buckets with constant width.
    ///   (2) Buckets with exponentially growing widths.
    ///   (3) Buckets with arbitrary user-provided widths.
    ///
    /// In all cases, the buckets cover the entire real number line (-inf,
    /// +inf). Bucket upper bounds are exclusive and lower bounds are
    /// inclusive. The upper bound of the underflow bucket is equal to the
    /// lower bound of the smallest finite bucket; the lower bound of the
    /// overflow bucket is equal to the upper bound of the largest finite
    /// bucket.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum BucketOption {
        /// Buckets with constant width.
        #[prost(message, tag = "7")]
        LinearBuckets(LinearBuckets),
        /// Buckets with exponentially growing width.
        #[prost(message, tag = "8")]
        ExponentialBuckets(ExponentialBuckets),
        /// Buckets with arbitrary user-provided width.
        #[prost(message, tag = "9")]
        ExplicitBuckets(ExplicitBuckets),
    }
}
/// An individual log entry.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LogEntry {
    /// Required. The log to which this log entry belongs. Examples: `"syslog"`,
    /// `"book_log"`.
    #[prost(string, tag = "10")]
    pub name: std::string::String,
    /// The time the event described by the log entry occurred. If
    /// omitted, defaults to operation start time.
    #[prost(message, optional, tag = "11")]
    pub timestamp: ::std::option::Option<::prost_types::Timestamp>,
    /// The severity of the log entry. The default value is
    /// `LogSeverity.DEFAULT`.
    #[prost(
        enumeration = "super::super::super::logging::r#type::LogSeverity",
        tag = "12"
    )]
    pub severity: i32,
    /// A unique ID for the log entry used for deduplication. If omitted,
    /// the implementation will generate one based on operation_id.
    #[prost(string, tag = "4")]
    pub insert_id: std::string::String,
    /// A set of user-defined (key, value) data that provides additional
    /// information about the log entry.
    #[prost(map = "string, string", tag = "13")]
    pub labels: ::std::collections::HashMap<std::string::String, std::string::String>,
    /// The log entry payload, which can be one of multiple types.
    #[prost(oneof = "log_entry::Payload", tags = "2, 3, 6")]
    pub payload: ::std::option::Option<log_entry::Payload>,
}
pub mod log_entry {
    /// The log entry payload, which can be one of multiple types.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Payload {
        /// The log entry payload, represented as a protocol buffer that is
        /// expressed as a JSON object. The only accepted type currently is
        /// [AuditLog][google.cloud.audit.AuditLog].
        #[prost(message, tag = "2")]
        ProtoPayload(::prost_types::Any),
        /// The log entry payload, represented as a Unicode string (UTF-8).
        #[prost(string, tag = "3")]
        TextPayload(std::string::String),
        /// The log entry payload, represented as a structure that
        /// is expressed as a JSON object.
        #[prost(message, tag = "6")]
        StructPayload(::prost_types::Struct),
    }
}
/// Represents a single metric value.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MetricValue {
    /// The labels describing the metric value.
    /// See comments on
    /// [google.api.servicecontrol.v1.Operation.labels][google.api.servicecontrol.v1.Operation.labels]
    /// for the overriding relationship. Note that this map must not contain
    /// monitored resource labels.
    #[prost(map = "string, string", tag = "1")]
    pub labels: ::std::collections::HashMap<std::string::String, std::string::String>,
    /// The start of the time period over which this metric value's measurement
    /// applies. The time period has different semantics for different metric
    /// types (cumulative, delta, and gauge). See the metric definition
    /// documentation in the service configuration for details.
    #[prost(message, optional, tag = "2")]
    pub start_time: ::std::option::Option<::prost_types::Timestamp>,
    /// The end of the time period over which this metric value's measurement
    /// applies.
    #[prost(message, optional, tag = "3")]
    pub end_time: ::std::option::Option<::prost_types::Timestamp>,
    /// The value. The type of value used in the request must
    /// agree with the metric definition in the service configuration, otherwise
    /// the MetricValue is rejected.
    #[prost(oneof = "metric_value::Value", tags = "4, 5, 6, 7, 8")]
    pub value: ::std::option::Option<metric_value::Value>,
}
pub mod metric_value {
    /// The value. The type of value used in the request must
    /// agree with the metric definition in the service configuration, otherwise
    /// the MetricValue is rejected.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Value {
        /// A boolean value.
        #[prost(bool, tag = "4")]
        BoolValue(bool),
        /// A signed 64-bit integer value.
        #[prost(int64, tag = "5")]
        Int64Value(i64),
        /// A double precision floating point value.
        #[prost(double, tag = "6")]
        DoubleValue(f64),
        /// A text string value.
        #[prost(string, tag = "7")]
        StringValue(std::string::String),
        /// A distribution value.
        #[prost(message, tag = "8")]
        DistributionValue(super::Distribution),
    }
}
/// Represents a set of metric values in the same metric.
/// Each metric value in the set should have a unique combination of start time,
/// end time, and label values.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MetricValueSet {
    /// The metric name defined in the service configuration.
    #[prost(string, tag = "1")]
    pub metric_name: std::string::String,
    /// The values in this metric.
    #[prost(message, repeated, tag = "2")]
    pub metric_values: ::std::vec::Vec<MetricValue>,
}
/// Represents information regarding an operation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Operation {
    /// Identity of the operation. This must be unique within the scope of the
    /// service that generated the operation. If the service calls
    /// Check() and Report() on the same operation, the two calls should carry
    /// the same id.
    ///
    /// UUID version 4 is recommended, though not required.
    /// In scenarios where an operation is computed from existing information
    /// and an idempotent id is desirable for deduplication purpose, UUID version 5
    /// is recommended. See RFC 4122 for details.
    #[prost(string, tag = "1")]
    pub operation_id: std::string::String,
    /// Fully qualified name of the operation. Reserved for future use.
    #[prost(string, tag = "2")]
    pub operation_name: std::string::String,
    /// Identity of the consumer who is using the service.
    /// This field should be filled in for the operations initiated by a
    /// consumer, but not for service-initiated operations that are
    /// not related to a specific consumer.
    ///
    /// - This can be in one of the following formats:
    ///     - project:PROJECT_ID,
    ///     - project`_`number:PROJECT_NUMBER,
    ///     - projects/PROJECT_ID or PROJECT_NUMBER,
    ///     - folders/FOLDER_NUMBER,
    ///     - organizations/ORGANIZATION_NUMBER,
    ///     - api`_`key:API_KEY.
    #[prost(string, tag = "3")]
    pub consumer_id: std::string::String,
    /// Required. Start time of the operation.
    #[prost(message, optional, tag = "4")]
    pub start_time: ::std::option::Option<::prost_types::Timestamp>,
    /// End time of the operation.
    /// Required when the operation is used in
    /// [ServiceController.Report][google.api.servicecontrol.v1.ServiceController.Report],
    /// but optional when the operation is used in
    /// [ServiceController.Check][google.api.servicecontrol.v1.ServiceController.Check].
    #[prost(message, optional, tag = "5")]
    pub end_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Labels describing the operation. Only the following labels are allowed:
    ///
    /// - Labels describing monitored resources as defined in
    ///   the service configuration.
    /// - Default labels of metric values. When specified, labels defined in the
    ///   metric value override these default.
    /// - The following labels defined by Google Cloud Platform:
    ///     - `cloud.googleapis.com/location` describing the location where the
    ///        operation happened,
    ///     - `servicecontrol.googleapis.com/user_agent` describing the user agent
    ///        of the API request,
    ///     - `servicecontrol.googleapis.com/service_agent` describing the service
    ///        used to handle the API request (e.g. ESP),
    ///     - `servicecontrol.googleapis.com/platform` describing the platform
    ///        where the API is served, such as App Engine, Compute Engine, or
    ///        Kubernetes Engine.
    #[prost(map = "string, string", tag = "6")]
    pub labels: ::std::collections::HashMap<std::string::String, std::string::String>,
    /// Represents information about this operation. Each MetricValueSet
    /// corresponds to a metric defined in the service configuration.
    /// The data type used in the MetricValueSet must agree with
    /// the data type specified in the metric definition.
    ///
    /// Within a single operation, it is not allowed to have more than one
    /// MetricValue instances that have the same metric names and identical
    /// label value combinations. If a request has such duplicated MetricValue
    /// instances, the entire request is rejected with
    /// an invalid argument error.
    #[prost(message, repeated, tag = "7")]
    pub metric_value_sets: ::std::vec::Vec<MetricValueSet>,
    /// Represents information to be logged.
    #[prost(message, repeated, tag = "8")]
    pub log_entries: ::std::vec::Vec<LogEntry>,
    /// DO NOT USE. This is an experimental field.
    #[prost(enumeration = "operation::Importance", tag = "11")]
    pub importance: i32,
}
pub mod operation {
    /// Defines the importance of the data contained in the operation.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Importance {
        /// The API implementation may cache and aggregate the data.
        /// The data may be lost when rare and unexpected system failures occur.
        Low = 0,
        /// The API implementation doesn't cache and aggregate the data.
        /// If the method returns successfully, it's guaranteed that the data has
        /// been persisted in durable storage.
        High = 1,
    }
}
/// Request message for the AllocateQuota method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AllocateQuotaRequest {
    /// Name of the service as specified in the service configuration. For example,
    /// `"pubsub.googleapis.com"`.
    ///
    /// See [google.api.Service][google.api.Service] for the definition of a
    /// service name.
    #[prost(string, tag = "1")]
    pub service_name: std::string::String,
    /// Operation that describes the quota allocation.
    #[prost(message, optional, tag = "2")]
    pub allocate_operation: ::std::option::Option<QuotaOperation>,
    /// Specifies which version of service configuration should be used to process
    /// the request. If unspecified or no matching version can be found, the latest
    /// one will be used.
    #[prost(string, tag = "4")]
    pub service_config_id: std::string::String,
}
/// Represents information regarding a quota operation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuotaOperation {
    /// Identity of the operation. This is expected to be unique within the scope
    /// of the service that generated the operation, and guarantees idempotency in
    /// case of retries.
    ///
    /// In order to ensure best performance and latency in the Quota backends,
    /// operation_ids are optimally associated with time, so that related
    /// operations can be accessed fast in storage. For this reason, the
    /// recommended token for services that intend to operate at a high QPS is
    /// Unix time in nanos + UUID
    #[prost(string, tag = "1")]
    pub operation_id: std::string::String,
    /// Fully qualified name of the API method for which this quota operation is
    /// requested. This name is used for matching quota rules or metric rules and
    /// billing status rules defined in service configuration.
    ///
    /// This field should not be set if any of the following is true:
    /// (1) the quota operation is performed on non-API resources.
    /// (2) quota_metrics is set because the caller is doing quota override.
    ///
    /// Example of an RPC method name:
    ///     google.example.library.v1.LibraryService.CreateShelf
    #[prost(string, tag = "2")]
    pub method_name: std::string::String,
    /// Identity of the consumer for whom this quota operation is being performed.
    ///
    /// This can be in one of the following formats:
    ///   project:<project_id>,
    ///   project_number:<project_number>,
    ///   api_key:<api_key>.
    #[prost(string, tag = "3")]
    pub consumer_id: std::string::String,
    /// Labels describing the operation.
    #[prost(map = "string, string", tag = "4")]
    pub labels: ::std::collections::HashMap<std::string::String, std::string::String>,
    /// Represents information about this operation. Each MetricValueSet
    /// corresponds to a metric defined in the service configuration.
    /// The data type used in the MetricValueSet must agree with
    /// the data type specified in the metric definition.
    ///
    /// Within a single operation, it is not allowed to have more than one
    /// MetricValue instances that have the same metric names and identical
    /// label value combinations. If a request has such duplicated MetricValue
    /// instances, the entire request is rejected with
    /// an invalid argument error.
    ///
    /// This field is mutually exclusive with method_name.
    #[prost(message, repeated, tag = "5")]
    pub quota_metrics: ::std::vec::Vec<MetricValueSet>,
    /// Quota mode for this operation.
    #[prost(enumeration = "quota_operation::QuotaMode", tag = "6")]
    pub quota_mode: i32,
}
pub mod quota_operation {
    /// Supported quota modes.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum QuotaMode {
        /// Guard against implicit default. Must not be used.
        Unspecified = 0,
        /// For AllocateQuota request, allocates quota for the amount specified in
        /// the service configuration or specified using the quota metrics. If the
        /// amount is higher than the available quota, allocation error will be
        /// returned and no quota will be allocated.
        /// If multiple quotas are part of the request, and one fails, none of the
        /// quotas are allocated or released.
        Normal = 1,
        /// The operation allocates quota for the amount specified in the service
        /// configuration or specified using the quota metrics. If the amount is
        /// higher than the available quota, request does not fail but all available
        /// quota will be allocated.
        /// For rate quota, BEST_EFFORT will continue to deduct from other groups
        /// even if one does not have enough quota. For allocation, it will find the
        /// minimum available amount across all groups and deduct that amount from
        /// all the affected groups.
        BestEffort = 2,
        /// For AllocateQuota request, only checks if there is enough quota
        /// available and does not change the available quota. No lock is placed on
        /// the available quota either.
        CheckOnly = 3,
    }
}
/// Response message for the AllocateQuota method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AllocateQuotaResponse {
    /// The same operation_id value used in the AllocateQuotaRequest. Used for
    /// logging and diagnostics purposes.
    #[prost(string, tag = "1")]
    pub operation_id: std::string::String,
    /// Indicates the decision of the allocate.
    #[prost(message, repeated, tag = "2")]
    pub allocate_errors: ::std::vec::Vec<QuotaError>,
    /// Quota metrics to indicate the result of allocation. Depending on the
    /// request, one or more of the following metrics will be included:
    ///
    /// 1. Per quota group or per quota metric incremental usage will be specified
    /// using the following delta metric :
    ///   "serviceruntime.googleapis.com/api/consumer/quota_used_count"
    ///
    /// 2. The quota limit reached condition will be specified using the following
    /// boolean metric :
    ///   "serviceruntime.googleapis.com/quota/exceeded"
    #[prost(message, repeated, tag = "3")]
    pub quota_metrics: ::std::vec::Vec<MetricValueSet>,
    /// ID of the actual config used to process the request.
    #[prost(string, tag = "4")]
    pub service_config_id: std::string::String,
}
/// Represents error information for
/// [QuotaOperation][google.api.servicecontrol.v1.QuotaOperation].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuotaError {
    /// Error code.
    #[prost(enumeration = "quota_error::Code", tag = "1")]
    pub code: i32,
    /// Subject to whom this error applies. See the specific enum for more details
    /// on this field. For example, "clientip:<ip address of client>" or
    /// "project:<Google developer project id>".
    #[prost(string, tag = "2")]
    pub subject: std::string::String,
    /// Free-form text that provides details on the cause of the error.
    #[prost(string, tag = "3")]
    pub description: std::string::String,
}
pub mod quota_error {
    /// Error codes related to project config validations are deprecated since the
    /// quota controller methods do not perform these validations. Instead services
    /// have to call the Check method, without quota_properties field, to perform
    /// these validations before calling the quota controller methods. These
    /// methods check only for project deletion to be wipe out compliant.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Code {
        /// This is never used.
        Unspecified = 0,
        /// Quota allocation failed.
        /// Same as
        /// [google.rpc.Code.RESOURCE_EXHAUSTED][google.rpc.Code.RESOURCE_EXHAUSTED].
        ResourceExhausted = 8,
        /// Consumer cannot access the service because the service requires active
        /// billing.
        BillingNotActive = 107,
        /// Consumer's project has been marked as deleted (soft deletion).
        ProjectDeleted = 108,
        /// Specified API key is invalid.
        ApiKeyInvalid = 105,
        /// Specified API Key has expired.
        ApiKeyExpired = 112,
    }
}
#[doc = r" Generated client implementations."]
pub mod quota_controller_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " [Google Quota Control API](/service-control/overview)"]
    #[doc = ""]
    #[doc = " Allows clients to allocate and release quota against a [managed"]
    #[doc = " service](https://cloud.google.com/service-management/reference/rpc/google.api/servicemanagement.v1#google.api.servicemanagement.v1.ManagedService)."]
    pub struct QuotaControllerClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl QuotaControllerClient<tonic::transport::Channel> {
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
    impl<T> QuotaControllerClient<T>
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
        #[doc = " Attempts to allocate quota for the specified consumer. It should be called"]
        #[doc = " before the operation is executed."]
        #[doc = ""]
        #[doc = " This method requires the `servicemanagement.services.quota`"]
        #[doc = " permission on the specified service. For more information, see"]
        #[doc = " [Cloud IAM](https://cloud.google.com/iam)."]
        #[doc = ""]
        #[doc = " **NOTE:** The client **must** fail-open on server errors `INTERNAL`,"]
        #[doc = " `UNKNOWN`, `DEADLINE_EXCEEDED`, and `UNAVAILABLE`. To ensure system"]
        #[doc = " reliability, the server may inject these errors to prohibit any hard"]
        #[doc = " dependency on the quota functionality."]
        pub async fn allocate_quota(
            &mut self,
            request: impl tonic::IntoRequest<super::AllocateQuotaRequest>,
        ) -> Result<tonic::Response<super::AllocateQuotaResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.api.servicecontrol.v1.QuotaController/AllocateQuota",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for QuotaControllerClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for QuotaControllerClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "QuotaControllerClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod quota_controller_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with QuotaControllerServer."]
    #[async_trait]
    pub trait QuotaController: Send + Sync + 'static {
        #[doc = " Attempts to allocate quota for the specified consumer. It should be called"]
        #[doc = " before the operation is executed."]
        #[doc = ""]
        #[doc = " This method requires the `servicemanagement.services.quota`"]
        #[doc = " permission on the specified service. For more information, see"]
        #[doc = " [Cloud IAM](https://cloud.google.com/iam)."]
        #[doc = ""]
        #[doc = " **NOTE:** The client **must** fail-open on server errors `INTERNAL`,"]
        #[doc = " `UNKNOWN`, `DEADLINE_EXCEEDED`, and `UNAVAILABLE`. To ensure system"]
        #[doc = " reliability, the server may inject these errors to prohibit any hard"]
        #[doc = " dependency on the quota functionality."]
        async fn allocate_quota(
            &self,
            request: tonic::Request<super::AllocateQuotaRequest>,
        ) -> Result<tonic::Response<super::AllocateQuotaResponse>, tonic::Status>;
    }
    #[doc = " [Google Quota Control API](/service-control/overview)"]
    #[doc = ""]
    #[doc = " Allows clients to allocate and release quota against a [managed"]
    #[doc = " service](https://cloud.google.com/service-management/reference/rpc/google.api/servicemanagement.v1#google.api.servicemanagement.v1.ManagedService)."]
    #[derive(Debug)]
    #[doc(hidden)]
    pub struct QuotaControllerServer<T: QuotaController> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: QuotaController> QuotaControllerServer<T> {
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
    impl<T, B> Service<http::Request<B>> for QuotaControllerServer<T>
    where
        T: QuotaController,
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
                "/google.api.servicecontrol.v1.QuotaController/AllocateQuota" => {
                    #[allow(non_camel_case_types)]
                    struct AllocateQuotaSvc<T: QuotaController>(pub Arc<T>);
                    impl<T: QuotaController>
                        tonic::server::UnaryService<super::AllocateQuotaRequest>
                        for AllocateQuotaSvc<T>
                    {
                        type Response = super::AllocateQuotaResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AllocateQuotaRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.allocate_quota(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = AllocateQuotaSvc(inner);
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
    impl<T: QuotaController> Clone for QuotaControllerServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: QuotaController> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: QuotaController> tonic::transport::NamedService for QuotaControllerServer<T> {
        const NAME: &'static str = "google.api.servicecontrol.v1.QuotaController";
    }
}
/// Request message for the Check method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckRequest {
    /// The service name as specified in its service configuration. For example,
    /// `"pubsub.googleapis.com"`.
    ///
    /// See
    /// [google.api.Service](https://cloud.google.com/service-management/reference/rpc/google.api#google.api.Service)
    /// for the definition of a service name.
    #[prost(string, tag = "1")]
    pub service_name: std::string::String,
    /// The operation to be checked.
    #[prost(message, optional, tag = "2")]
    pub operation: ::std::option::Option<Operation>,
    /// Specifies which version of service configuration should be used to process
    /// the request.
    ///
    /// If unspecified or no matching version can be found, the
    /// latest one will be used.
    #[prost(string, tag = "4")]
    pub service_config_id: std::string::String,
}
/// Response message for the Check method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckResponse {
    /// The same operation_id value used in the
    /// [CheckRequest][google.api.servicecontrol.v1.CheckRequest]. Used for logging
    /// and diagnostics purposes.
    #[prost(string, tag = "1")]
    pub operation_id: std::string::String,
    /// Indicate the decision of the check.
    ///
    /// If no check errors are present, the service should process the operation.
    /// Otherwise the service should use the list of errors to determine the
    /// appropriate action.
    #[prost(message, repeated, tag = "2")]
    pub check_errors: ::std::vec::Vec<CheckError>,
    /// The actual config id used to process the request.
    #[prost(string, tag = "5")]
    pub service_config_id: std::string::String,
    /// The current service rollout id used to process the request.
    #[prost(string, tag = "11")]
    pub service_rollout_id: std::string::String,
    /// Feedback data returned from the server during processing a Check request.
    #[prost(message, optional, tag = "6")]
    pub check_info: ::std::option::Option<check_response::CheckInfo>,
}
pub mod check_response {
    /// Contains additional information about the check operation.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CheckInfo {
        /// Consumer info of this check.
        #[prost(message, optional, tag = "2")]
        pub consumer_info: ::std::option::Option<ConsumerInfo>,
    }
    /// `ConsumerInfo` provides information about the consumer.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ConsumerInfo {
        /// The Google cloud project number, e.g. 1234567890. A value of 0 indicates
        /// no project number is found.
        ///
        /// NOTE: This field is deprecated after Service Control support flexible
        /// consumer id. New code should not depend on this field anymore.
        #[prost(int64, tag = "1")]
        pub project_number: i64,
        /// The type of the consumer which should have been defined in
        /// [Google Resource Manager](https://cloud.google.com/resource-manager/).
        #[prost(enumeration = "consumer_info::ConsumerType", tag = "2")]
        pub r#type: i32,
        /// The consumer identity number, can be Google cloud project number, folder
        /// number or organization number e.g. 1234567890. A value of 0 indicates no
        /// consumer number is found.
        #[prost(int64, tag = "3")]
        pub consumer_number: i64,
    }
    pub mod consumer_info {
        /// The type of the consumer as defined in
        /// [Google Resource Manager](https://cloud.google.com/resource-manager/).
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
        #[repr(i32)]
        pub enum ConsumerType {
            /// This is never used.
            Unspecified = 0,
            /// The consumer is a Google Cloud Project.
            Project = 1,
            /// The consumer is a Google Cloud Folder.
            Folder = 2,
            /// The consumer is a Google Cloud Organization.
            Organization = 3,
            /// Service-specific resource container which is defined by the service
            /// producer to offer their users the ability to manage service control
            /// functionalities at a finer level of granularity than the PROJECT.
            ServiceSpecific = 4,
        }
    }
}
/// Request message for the Report method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReportRequest {
    /// The service name as specified in its service configuration. For example,
    /// `"pubsub.googleapis.com"`.
    ///
    /// See
    /// [google.api.Service](https://cloud.google.com/service-management/reference/rpc/google.api#google.api.Service)
    /// for the definition of a service name.
    #[prost(string, tag = "1")]
    pub service_name: std::string::String,
    /// Operations to be reported.
    ///
    /// Typically the service should report one operation per request.
    /// Putting multiple operations into a single request is allowed, but should
    /// be used only when multiple operations are natually available at the time
    /// of the report.
    ///
    /// There is no limit on the number of operations in the same ReportRequest,
    /// however the ReportRequest size should be no larger than 1MB. See
    /// [ReportResponse.report_errors][google.api.servicecontrol.v1.ReportResponse.report_errors]
    /// for partial failure behavior.
    #[prost(message, repeated, tag = "2")]
    pub operations: ::std::vec::Vec<Operation>,
    /// Specifies which version of service config should be used to process the
    /// request.
    ///
    /// If unspecified or no matching version can be found, the
    /// latest one will be used.
    #[prost(string, tag = "3")]
    pub service_config_id: std::string::String,
}
/// Response message for the Report method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReportResponse {
    /// Partial failures, one for each `Operation` in the request that failed
    /// processing. There are three possible combinations of the RPC status:
    ///
    /// 1. The combination of a successful RPC status and an empty `report_errors`
    ///    list indicates a complete success where all `Operations` in the
    ///    request are processed successfully.
    /// 2. The combination of a successful RPC status and a non-empty
    ///    `report_errors` list indicates a partial success where some
    ///    `Operations` in the request succeeded. Each
    ///    `Operation` that failed processing has a corresponding item
    ///    in this list.
    /// 3. A failed RPC status indicates a general non-deterministic failure.
    ///    When this happens, it's impossible to know which of the
    ///    'Operations' in the request succeeded or failed.
    #[prost(message, repeated, tag = "1")]
    pub report_errors: ::std::vec::Vec<report_response::ReportError>,
    /// The actual config id used to process the request.
    #[prost(string, tag = "2")]
    pub service_config_id: std::string::String,
    /// The current service rollout id used to process the request.
    #[prost(string, tag = "4")]
    pub service_rollout_id: std::string::String,
}
pub mod report_response {
    /// Represents the processing error of one
    /// [Operation][google.api.servicecontrol.v1.Operation] in the request.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ReportError {
        /// The
        /// [Operation.operation_id][google.api.servicecontrol.v1.Operation.operation_id]
        /// value from the request.
        #[prost(string, tag = "1")]
        pub operation_id: std::string::String,
        /// Details of the error when processing the
        /// [Operation][google.api.servicecontrol.v1.Operation].
        #[prost(message, optional, tag = "2")]
        pub status: ::std::option::Option<super::super::super::super::rpc::Status>,
    }
}
#[doc = r" Generated client implementations."]
pub mod service_controller_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " [Google Service Control API](/service-control/overview)"]
    #[doc = ""]
    #[doc = " Lets clients check and report operations against a [managed"]
    #[doc = " service](https://cloud.google.com/service-management/reference/rpc/google.api/servicemanagement.v1#google.api.servicemanagement.v1.ManagedService)."]
    pub struct ServiceControllerClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ServiceControllerClient<tonic::transport::Channel> {
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
    impl<T> ServiceControllerClient<T>
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
        #[doc = " Checks whether an operation on a service should be allowed to proceed"]
        #[doc = " based on the configuration of the service and related policies. It must be"]
        #[doc = " called before the operation is executed."]
        #[doc = ""]
        #[doc = " If feasible, the client should cache the check results and reuse them for"]
        #[doc = " 60 seconds. In case of any server errors, the client should rely on the"]
        #[doc = " cached results for much longer time to avoid outage."]
        #[doc = " WARNING: There is general 60s delay for the configuration and policy"]
        #[doc = " propagation, therefore callers MUST NOT depend on the `Check` method having"]
        #[doc = " the latest policy information."]
        #[doc = ""]
        #[doc = " NOTE: the [CheckRequest][google.api.servicecontrol.v1.CheckRequest] has the"]
        #[doc = " size limit of 64KB."]
        #[doc = ""]
        #[doc = " This method requires the `servicemanagement.services.check` permission"]
        #[doc = " on the specified service. For more information, see"]
        #[doc = " [Cloud IAM](https://cloud.google.com/iam)."]
        pub async fn check(
            &mut self,
            request: impl tonic::IntoRequest<super::CheckRequest>,
        ) -> Result<tonic::Response<super::CheckResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.api.servicecontrol.v1.ServiceController/Check",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Reports operation results to Google Service Control, such as logs and"]
        #[doc = " metrics. It should be called after an operation is completed."]
        #[doc = ""]
        #[doc = " If feasible, the client should aggregate reporting data for up to 5"]
        #[doc = " seconds to reduce API traffic. Limiting aggregation to 5 seconds is to"]
        #[doc = " reduce data loss during client crashes. Clients should carefully choose"]
        #[doc = " the aggregation time window to avoid data loss risk more than 0.01%"]
        #[doc = " for business and compliance reasons."]
        #[doc = ""]
        #[doc = " NOTE: the [ReportRequest][google.api.servicecontrol.v1.ReportRequest] has"]
        #[doc = " the size limit (wire-format byte size) of 1MB."]
        #[doc = ""]
        #[doc = " This method requires the `servicemanagement.services.report` permission"]
        #[doc = " on the specified service. For more information, see"]
        #[doc = " [Google Cloud IAM](https://cloud.google.com/iam)."]
        pub async fn report(
            &mut self,
            request: impl tonic::IntoRequest<super::ReportRequest>,
        ) -> Result<tonic::Response<super::ReportResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.api.servicecontrol.v1.ServiceController/Report",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for ServiceControllerClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for ServiceControllerClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "ServiceControllerClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod service_controller_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with ServiceControllerServer."]
    #[async_trait]
    pub trait ServiceController: Send + Sync + 'static {
        #[doc = " Checks whether an operation on a service should be allowed to proceed"]
        #[doc = " based on the configuration of the service and related policies. It must be"]
        #[doc = " called before the operation is executed."]
        #[doc = ""]
        #[doc = " If feasible, the client should cache the check results and reuse them for"]
        #[doc = " 60 seconds. In case of any server errors, the client should rely on the"]
        #[doc = " cached results for much longer time to avoid outage."]
        #[doc = " WARNING: There is general 60s delay for the configuration and policy"]
        #[doc = " propagation, therefore callers MUST NOT depend on the `Check` method having"]
        #[doc = " the latest policy information."]
        #[doc = ""]
        #[doc = " NOTE: the [CheckRequest][google.api.servicecontrol.v1.CheckRequest] has the"]
        #[doc = " size limit of 64KB."]
        #[doc = ""]
        #[doc = " This method requires the `servicemanagement.services.check` permission"]
        #[doc = " on the specified service. For more information, see"]
        #[doc = " [Cloud IAM](https://cloud.google.com/iam)."]
        async fn check(
            &self,
            request: tonic::Request<super::CheckRequest>,
        ) -> Result<tonic::Response<super::CheckResponse>, tonic::Status>;
        #[doc = " Reports operation results to Google Service Control, such as logs and"]
        #[doc = " metrics. It should be called after an operation is completed."]
        #[doc = ""]
        #[doc = " If feasible, the client should aggregate reporting data for up to 5"]
        #[doc = " seconds to reduce API traffic. Limiting aggregation to 5 seconds is to"]
        #[doc = " reduce data loss during client crashes. Clients should carefully choose"]
        #[doc = " the aggregation time window to avoid data loss risk more than 0.01%"]
        #[doc = " for business and compliance reasons."]
        #[doc = ""]
        #[doc = " NOTE: the [ReportRequest][google.api.servicecontrol.v1.ReportRequest] has"]
        #[doc = " the size limit (wire-format byte size) of 1MB."]
        #[doc = ""]
        #[doc = " This method requires the `servicemanagement.services.report` permission"]
        #[doc = " on the specified service. For more information, see"]
        #[doc = " [Google Cloud IAM](https://cloud.google.com/iam)."]
        async fn report(
            &self,
            request: tonic::Request<super::ReportRequest>,
        ) -> Result<tonic::Response<super::ReportResponse>, tonic::Status>;
    }
    #[doc = " [Google Service Control API](/service-control/overview)"]
    #[doc = ""]
    #[doc = " Lets clients check and report operations against a [managed"]
    #[doc = " service](https://cloud.google.com/service-management/reference/rpc/google.api/servicemanagement.v1#google.api.servicemanagement.v1.ManagedService)."]
    #[derive(Debug)]
    #[doc(hidden)]
    pub struct ServiceControllerServer<T: ServiceController> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: ServiceController> ServiceControllerServer<T> {
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
    impl<T, B> Service<http::Request<B>> for ServiceControllerServer<T>
    where
        T: ServiceController,
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
                "/google.api.servicecontrol.v1.ServiceController/Check" => {
                    #[allow(non_camel_case_types)]
                    struct CheckSvc<T: ServiceController>(pub Arc<T>);
                    impl<T: ServiceController> tonic::server::UnaryService<super::CheckRequest> for CheckSvc<T> {
                        type Response = super::CheckResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CheckRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.check(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = CheckSvc(inner);
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
                "/google.api.servicecontrol.v1.ServiceController/Report" => {
                    #[allow(non_camel_case_types)]
                    struct ReportSvc<T: ServiceController>(pub Arc<T>);
                    impl<T: ServiceController> tonic::server::UnaryService<super::ReportRequest> for ReportSvc<T> {
                        type Response = super::ReportResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ReportRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.report(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ReportSvc(inner);
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
    impl<T: ServiceController> Clone for ServiceControllerServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: ServiceController> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: ServiceController> tonic::transport::NamedService for ServiceControllerServer<T> {
        const NAME: &'static str = "google.api.servicecontrol.v1.ServiceController";
    }
}
