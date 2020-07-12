/// A single strongly-typed value.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TypedValue {
    /// The typed value field.
    #[prost(oneof = "typed_value::Value", tags = "1, 2, 3, 4, 5")]
    pub value: ::std::option::Option<typed_value::Value>,
}
pub mod typed_value {
    /// The typed value field.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Value {
        /// A Boolean value: `true` or `false`.
        #[prost(bool, tag = "1")]
        BoolValue(bool),
        /// A 64-bit integer. Its range is approximately &plusmn;9.2x10<sup>18</sup>.
        #[prost(int64, tag = "2")]
        Int64Value(i64),
        /// A 64-bit double-precision floating-point number. Its magnitude
        /// is approximately &plusmn;10<sup>&plusmn;300</sup> and it has 16
        /// significant digits of precision.
        #[prost(double, tag = "3")]
        DoubleValue(f64),
        /// A variable-length string value.
        #[prost(string, tag = "4")]
        StringValue(std::string::String),
        /// A distribution value.
        #[prost(message, tag = "5")]
        DistributionValue(super::super::super::api::Distribution),
    }
}
/// A closed time interval. It extends from the start time to the end time, and
/// includes both: `[startTime, endTime]`. Valid time intervals depend on the
/// [`MetricKind`](https://cloud.google.com/monitoring/api/ref_v3/rest/v3/projects.metricDescriptors#MetricKind)
/// of the metric value. In no case can the end time be earlier than the start
/// time.
///
/// * For a `GAUGE` metric, the `startTime` value is technically optional; if
///   no value is specified, the start time defaults to the value of the
///   end time, and the interval represents a single point in time. If both
///   start and end times are specified, they must be identical. Such an
///   interval is valid only for `GAUGE` metrics, which are point-in-time
///   measurements.
///
/// * For `DELTA` and `CUMULATIVE` metrics, the start time must be earlier
///   than the end time.
///
/// * In all cases, the start time of the next interval must be
///   at least a millisecond after the end time of the previous interval.
///   Because the interval is closed, if the start time of a new interval
///   is the same as the end time of the previous interval, data written
///   at the new start time could overwrite data written at the previous
///   end time.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TimeInterval {
    /// Required. The end of the time interval.
    #[prost(message, optional, tag = "2")]
    pub end_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Optional. The beginning of the time interval.  The default value
    /// for the start time is the end time. The start time must not be
    /// later than the end time.
    #[prost(message, optional, tag = "1")]
    pub start_time: ::std::option::Option<::prost_types::Timestamp>,
}
/// Describes how to combine multiple time series to provide a different view of
/// the data.  Aggregation of time series is done in two steps. First, each time
/// series in the set is _aligned_ to the same time interval boundaries, then the
/// set of time series is optionally _reduced_ in number.
///
/// Alignment consists of applying the `per_series_aligner` operation
/// to each time series after its data has been divided into regular
/// `alignment_period` time intervals. This process takes _all_ of the data
/// points in an alignment period, applies a mathematical transformation such as
/// averaging, minimum, maximum, delta, etc., and converts them into a single
/// data point per period.
///
/// Reduction is when the aligned and transformed time series can optionally be
/// combined, reducing the number of time series through similar mathematical
/// transformations. Reduction involves applying a `cross_series_reducer` to
/// all the time series, optionally sorting the time series into subsets with
/// `group_by_fields`, and applying the reducer to each subset.
///
/// The raw time series data can contain a huge amount of information from
/// multiple sources. Alignment and reduction transforms this mass of data into
/// a more manageable and representative collection of data, for example "the
/// 95% latency across the average of all tasks in a cluster". This
/// representative data can be more easily graphed and comprehended, and the
/// individual time series data is still available for later drilldown. For more
/// details, see [Filtering and
/// aggregation](https://cloud.google.com/monitoring/api/v3/aggregation).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Aggregation {
    /// The `alignment_period` specifies a time interval, in seconds, that is used
    /// to divide the data in all the
    /// [time series][google.monitoring.v3.TimeSeries] into consistent blocks of
    /// time. This will be done before the per-series aligner can be applied to
    /// the data.
    ///
    /// The value must be at least 60 seconds. If a per-series aligner other than
    /// `ALIGN_NONE` is specified, this field is required or an error is returned.
    /// If no per-series aligner is specified, or the aligner `ALIGN_NONE` is
    /// specified, then this field is ignored.
    #[prost(message, optional, tag = "1")]
    pub alignment_period: ::std::option::Option<::prost_types::Duration>,
    /// An `Aligner` describes how to bring the data points in a single
    /// time series into temporal alignment. Except for `ALIGN_NONE`, all
    /// alignments cause all the data points in an `alignment_period` to be
    /// mathematically grouped together, resulting in a single data point for
    /// each `alignment_period` with end timestamp at the end of the period.
    ///
    /// Not all alignment operations may be applied to all time series. The valid
    /// choices depend on the `metric_kind` and `value_type` of the original time
    /// series. Alignment can change the `metric_kind` or the `value_type` of
    /// the time series.
    ///
    /// Time series data must be aligned in order to perform cross-time
    /// series reduction. If `cross_series_reducer` is specified, then
    /// `per_series_aligner` must be specified and not equal to `ALIGN_NONE`
    /// and `alignment_period` must be specified; otherwise, an error is
    /// returned.
    #[prost(enumeration = "aggregation::Aligner", tag = "2")]
    pub per_series_aligner: i32,
    /// The reduction operation to be used to combine time series into a single
    /// time series, where the value of each data point in the resulting series is
    /// a function of all the already aligned values in the input time series.
    ///
    /// Not all reducer operations can be applied to all time series. The valid
    /// choices depend on the `metric_kind` and the `value_type` of the original
    /// time series. Reduction can yield a time series with a different
    /// `metric_kind` or `value_type` than the input time series.
    ///
    /// Time series data must first be aligned (see `per_series_aligner`) in order
    /// to perform cross-time series reduction. If `cross_series_reducer` is
    /// specified, then `per_series_aligner` must be specified, and must not be
    /// `ALIGN_NONE`. An `alignment_period` must also be specified; otherwise, an
    /// error is returned.
    #[prost(enumeration = "aggregation::Reducer", tag = "4")]
    pub cross_series_reducer: i32,
    /// The set of fields to preserve when `cross_series_reducer` is
    /// specified. The `group_by_fields` determine how the time series are
    /// partitioned into subsets prior to applying the aggregation
    /// operation. Each subset contains time series that have the same
    /// value for each of the grouping fields. Each individual time
    /// series is a member of exactly one subset. The
    /// `cross_series_reducer` is applied to each subset of time series.
    /// It is not possible to reduce across different resource types, so
    /// this field implicitly contains `resource.type`.  Fields not
    /// specified in `group_by_fields` are aggregated away.  If
    /// `group_by_fields` is not specified and all the time series have
    /// the same resource type, then the time series are aggregated into
    /// a single output time series. If `cross_series_reducer` is not
    /// defined, this field is ignored.
    #[prost(string, repeated, tag = "5")]
    pub group_by_fields: ::std::vec::Vec<std::string::String>,
}
pub mod aggregation {
    /// The `Aligner` specifies the operation that will be applied to the data
    /// points in each alignment period in a time series. Except for
    /// `ALIGN_NONE`, which specifies that no operation be applied, each alignment
    /// operation replaces the set of data values in each alignment period with
    /// a single value: the result of applying the operation to the data values.
    /// An aligned time series has a single data value at the end of each
    /// `alignment_period`.
    ///
    /// An alignment operation can change the data type of the values, too. For
    /// example, if you apply a counting operation to boolean values, the data
    /// `value_type` in the original time series is `BOOLEAN`, but the `value_type`
    /// in the aligned result is `INT64`.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Aligner {
        /// No alignment. Raw data is returned. Not valid if cross-series reduction
        /// is requested. The `value_type` of the result is the same as the
        /// `value_type` of the input.
        AlignNone = 0,
        /// Align and convert to
        /// [DELTA][google.api.MetricDescriptor.MetricKind.DELTA].
        /// The output is `delta = y1 - y0`.
        ///
        /// This alignment is valid for
        /// [CUMULATIVE][google.api.MetricDescriptor.MetricKind.CUMULATIVE] and
        /// `DELTA` metrics. If the selected alignment period results in periods
        /// with no data, then the aligned value for such a period is created by
        /// interpolation. The `value_type`  of the aligned result is the same as
        /// the `value_type` of the input.
        AlignDelta = 1,
        /// Align and convert to a rate. The result is computed as
        /// `rate = (y1 - y0)/(t1 - t0)`, or "delta over time".
        /// Think of this aligner as providing the slope of the line that passes
        /// through the value at the start and at the end of the `alignment_period`.
        ///
        /// This aligner is valid for `CUMULATIVE`
        /// and `DELTA` metrics with numeric values. If the selected alignment
        /// period results in periods with no data, then the aligned value for
        /// such a period is created by interpolation. The output is a `GAUGE`
        /// metric with `value_type` `DOUBLE`.
        ///
        /// If, by "rate", you mean "percentage change", see the
        /// `ALIGN_PERCENT_CHANGE` aligner instead.
        AlignRate = 2,
        /// Align by interpolating between adjacent points around the alignment
        /// period boundary. This aligner is valid for `GAUGE` metrics with
        /// numeric values. The `value_type` of the aligned result is the same as the
        /// `value_type` of the input.
        AlignInterpolate = 3,
        /// Align by moving the most recent data point before the end of the
        /// alignment period to the boundary at the end of the alignment
        /// period. This aligner is valid for `GAUGE` metrics. The `value_type` of
        /// the aligned result is the same as the `value_type` of the input.
        AlignNextOlder = 4,
        /// Align the time series by returning the minimum value in each alignment
        /// period. This aligner is valid for `GAUGE` and `DELTA` metrics with
        /// numeric values. The `value_type` of the aligned result is the same as
        /// the `value_type` of the input.
        AlignMin = 10,
        /// Align the time series by returning the maximum value in each alignment
        /// period. This aligner is valid for `GAUGE` and `DELTA` metrics with
        /// numeric values. The `value_type` of the aligned result is the same as
        /// the `value_type` of the input.
        AlignMax = 11,
        /// Align the time series by returning the mean value in each alignment
        /// period. This aligner is valid for `GAUGE` and `DELTA` metrics with
        /// numeric values. The `value_type` of the aligned result is `DOUBLE`.
        AlignMean = 12,
        /// Align the time series by returning the number of values in each alignment
        /// period. This aligner is valid for `GAUGE` and `DELTA` metrics with
        /// numeric or Boolean values. The `value_type` of the aligned result is
        /// `INT64`.
        AlignCount = 13,
        /// Align the time series by returning the sum of the values in each
        /// alignment period. This aligner is valid for `GAUGE` and `DELTA`
        /// metrics with numeric and distribution values. The `value_type` of the
        /// aligned result is the same as the `value_type` of the input.
        AlignSum = 14,
        /// Align the time series by returning the standard deviation of the values
        /// in each alignment period. This aligner is valid for `GAUGE` and
        /// `DELTA` metrics with numeric values. The `value_type` of the output is
        /// `DOUBLE`.
        AlignStddev = 15,
        /// Align the time series by returning the number of `True` values in
        /// each alignment period. This aligner is valid for `GAUGE` metrics with
        /// Boolean values. The `value_type` of the output is `INT64`.
        AlignCountTrue = 16,
        /// Align the time series by returning the number of `False` values in
        /// each alignment period. This aligner is valid for `GAUGE` metrics with
        /// Boolean values. The `value_type` of the output is `INT64`.
        AlignCountFalse = 24,
        /// Align the time series by returning the ratio of the number of `True`
        /// values to the total number of values in each alignment period. This
        /// aligner is valid for `GAUGE` metrics with Boolean values. The output
        /// value is in the range [0.0, 1.0] and has `value_type` `DOUBLE`.
        AlignFractionTrue = 17,
        /// Align the time series by using [percentile
        /// aggregation](https://en.wikipedia.org/wiki/Percentile). The resulting
        /// data point in each alignment period is the 99th percentile of all data
        /// points in the period. This aligner is valid for `GAUGE` and `DELTA`
        /// metrics with distribution values. The output is a `GAUGE` metric with
        /// `value_type` `DOUBLE`.
        AlignPercentile99 = 18,
        /// Align the time series by using [percentile
        /// aggregation](https://en.wikipedia.org/wiki/Percentile). The resulting
        /// data point in each alignment period is the 95th percentile of all data
        /// points in the period. This aligner is valid for `GAUGE` and `DELTA`
        /// metrics with distribution values. The output is a `GAUGE` metric with
        /// `value_type` `DOUBLE`.
        AlignPercentile95 = 19,
        /// Align the time series by using [percentile
        /// aggregation](https://en.wikipedia.org/wiki/Percentile). The resulting
        /// data point in each alignment period is the 50th percentile of all data
        /// points in the period. This aligner is valid for `GAUGE` and `DELTA`
        /// metrics with distribution values. The output is a `GAUGE` metric with
        /// `value_type` `DOUBLE`.
        AlignPercentile50 = 20,
        /// Align the time series by using [percentile
        /// aggregation](https://en.wikipedia.org/wiki/Percentile). The resulting
        /// data point in each alignment period is the 5th percentile of all data
        /// points in the period. This aligner is valid for `GAUGE` and `DELTA`
        /// metrics with distribution values. The output is a `GAUGE` metric with
        /// `value_type` `DOUBLE`.
        AlignPercentile05 = 21,
        /// Align and convert to a percentage change. This aligner is valid for
        /// `GAUGE` and `DELTA` metrics with numeric values. This alignment returns
        /// `((current - previous)/previous) * 100`, where the value of `previous` is
        /// determined based on the `alignment_period`.
        ///
        /// If the values of `current` and `previous` are both 0, then the returned
        /// value is 0. If only `previous` is 0, the returned value is infinity.
        ///
        /// A 10-minute moving mean is computed at each point of the alignment period
        /// prior to the above calculation to smooth the metric and prevent false
        /// positives from very short-lived spikes. The moving mean is only
        /// applicable for data whose values are `>= 0`. Any values `< 0` are
        /// treated as a missing datapoint, and are ignored. While `DELTA`
        /// metrics are accepted by this alignment, special care should be taken that
        /// the values for the metric will always be positive. The output is a
        /// `GAUGE` metric with `value_type` `DOUBLE`.
        AlignPercentChange = 23,
    }
    /// A Reducer operation describes how to aggregate data points from multiple
    /// time series into a single time series, where the value of each data point
    /// in the resulting series is a function of all the already aligned values in
    /// the input time series.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Reducer {
        /// No cross-time series reduction. The output of the `Aligner` is
        /// returned.
        ReduceNone = 0,
        /// Reduce by computing the mean value across time series for each
        /// alignment period. This reducer is valid for
        /// [DELTA][google.api.MetricDescriptor.MetricKind.DELTA] and
        /// [GAUGE][google.api.MetricDescriptor.MetricKind.GAUGE] metrics with
        /// numeric or distribution values. The `value_type` of the output is
        /// [DOUBLE][google.api.MetricDescriptor.ValueType.DOUBLE].
        ReduceMean = 1,
        /// Reduce by computing the minimum value across time series for each
        /// alignment period. This reducer is valid for `DELTA` and `GAUGE` metrics
        /// with numeric values. The `value_type` of the output is the same as the
        /// `value_type` of the input.
        ReduceMin = 2,
        /// Reduce by computing the maximum value across time series for each
        /// alignment period. This reducer is valid for `DELTA` and `GAUGE` metrics
        /// with numeric values. The `value_type` of the output is the same as the
        /// `value_type` of the input.
        ReduceMax = 3,
        /// Reduce by computing the sum across time series for each
        /// alignment period. This reducer is valid for `DELTA` and `GAUGE` metrics
        /// with numeric and distribution values. The `value_type` of the output is
        /// the same as the `value_type` of the input.
        ReduceSum = 4,
        /// Reduce by computing the standard deviation across time series
        /// for each alignment period. This reducer is valid for `DELTA` and
        /// `GAUGE` metrics with numeric or distribution values. The `value_type`
        /// of the output is `DOUBLE`.
        ReduceStddev = 5,
        /// Reduce by computing the number of data points across time series
        /// for each alignment period. This reducer is valid for `DELTA` and
        /// `GAUGE` metrics of numeric, Boolean, distribution, and string
        /// `value_type`. The `value_type` of the output is `INT64`.
        ReduceCount = 6,
        /// Reduce by computing the number of `True`-valued data points across time
        /// series for each alignment period. This reducer is valid for `DELTA` and
        /// `GAUGE` metrics of Boolean `value_type`. The `value_type` of the output
        /// is `INT64`.
        ReduceCountTrue = 7,
        /// Reduce by computing the number of `False`-valued data points across time
        /// series for each alignment period. This reducer is valid for `DELTA` and
        /// `GAUGE` metrics of Boolean `value_type`. The `value_type` of the output
        /// is `INT64`.
        ReduceCountFalse = 15,
        /// Reduce by computing the ratio of the number of `True`-valued data points
        /// to the total number of data points for each alignment period. This
        /// reducer is valid for `DELTA` and `GAUGE` metrics of Boolean `value_type`.
        /// The output value is in the range [0.0, 1.0] and has `value_type`
        /// `DOUBLE`.
        ReduceFractionTrue = 8,
        /// Reduce by computing the [99th
        /// percentile](https://en.wikipedia.org/wiki/Percentile) of data points
        /// across time series for each alignment period. This reducer is valid for
        /// `GAUGE` and `DELTA` metrics of numeric and distribution type. The value
        /// of the output is `DOUBLE`.
        ReducePercentile99 = 9,
        /// Reduce by computing the [95th
        /// percentile](https://en.wikipedia.org/wiki/Percentile) of data points
        /// across time series for each alignment period. This reducer is valid for
        /// `GAUGE` and `DELTA` metrics of numeric and distribution type. The value
        /// of the output is `DOUBLE`.
        ReducePercentile95 = 10,
        /// Reduce by computing the [50th
        /// percentile](https://en.wikipedia.org/wiki/Percentile) of data points
        /// across time series for each alignment period. This reducer is valid for
        /// `GAUGE` and `DELTA` metrics of numeric and distribution type. The value
        /// of the output is `DOUBLE`.
        ReducePercentile50 = 11,
        /// Reduce by computing the [5th
        /// percentile](https://en.wikipedia.org/wiki/Percentile) of data points
        /// across time series for each alignment period. This reducer is valid for
        /// `GAUGE` and `DELTA` metrics of numeric and distribution type. The value
        /// of the output is `DOUBLE`.
        ReducePercentile05 = 12,
    }
}
/// Specifies an ordering relationship on two arguments, called `left` and
/// `right`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ComparisonType {
    /// No ordering relationship is specified.
    ComparisonUnspecified = 0,
    /// True if the left argument is greater than the right argument.
    ComparisonGt = 1,
    /// True if the left argument is greater than or equal to the right argument.
    ComparisonGe = 2,
    /// True if the left argument is less than the right argument.
    ComparisonLt = 3,
    /// True if the left argument is less than or equal to the right argument.
    ComparisonLe = 4,
    /// True if the left argument is equal to the right argument.
    ComparisonEq = 5,
    /// True if the left argument is not equal to the right argument.
    ComparisonNe = 6,
}
/// The tier of service for a Workspace. Please see the
/// [service tiers
/// documentation](https://cloud.google.com/monitoring/workspaces/tiers) for more
/// details.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ServiceTier {
    /// An invalid sentinel value, used to indicate that a tier has not
    /// been provided explicitly.
    Unspecified = 0,
    /// The Stackdriver Basic tier, a free tier of service that provides basic
    /// features, a moderate allotment of logs, and access to built-in metrics.
    /// A number of features are not available in this tier. For more details,
    /// see [the service tiers
    /// documentation](https://cloud.google.com/monitoring/workspaces/tiers).
    Basic = 1,
    /// The Stackdriver Premium tier, a higher, more expensive tier of service
    /// that provides access to all Stackdriver features, lets you use Stackdriver
    /// with AWS accounts, and has a larger allotments for logs and metrics. For
    /// more details, see [the service tiers
    /// documentation](https://cloud.google.com/monitoring/workspaces/tiers).
    Premium = 2,
}
/// Describes a change made to a configuration.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutationRecord {
    /// When the change occurred.
    #[prost(message, optional, tag = "1")]
    pub mutate_time: ::std::option::Option<::prost_types::Timestamp>,
    /// The email address of the user making the change.
    #[prost(string, tag = "2")]
    pub mutated_by: std::string::String,
}
/// A description of the conditions under which some aspect of your system is
/// considered to be "unhealthy" and the ways to notify people or services about
/// this state. For an overview of alert policies, see
/// [Introduction to Alerting](https://cloud.google.com/monitoring/alerts/).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AlertPolicy {
    /// Required if the policy exists. The resource name for this policy. The
    /// format is:
    ///
    ///     projects/[PROJECT_ID_OR_NUMBER]/alertPolicies/[ALERT_POLICY_ID]
    ///
    /// `[ALERT_POLICY_ID]` is assigned by Stackdriver Monitoring when the policy
    /// is created.  When calling the
    /// [alertPolicies.create][google.monitoring.v3.AlertPolicyService.CreateAlertPolicy]
    /// method, do not include the `name` field in the alerting policy passed as
    /// part of the request.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// A short name or phrase used to identify the policy in dashboards,
    /// notifications, and incidents. To avoid confusion, don't use the same
    /// display name for multiple policies in the same project. The name is
    /// limited to 512 Unicode characters.
    #[prost(string, tag = "2")]
    pub display_name: std::string::String,
    /// Documentation that is included with notifications and incidents related to
    /// this policy. Best practice is for the documentation to include information
    /// to help responders understand, mitigate, escalate, and correct the
    /// underlying problems detected by the alerting policy. Notification channels
    /// that have limited capacity might not show this documentation.
    #[prost(message, optional, tag = "13")]
    pub documentation: ::std::option::Option<alert_policy::Documentation>,
    /// User-supplied key/value data to be used for organizing and
    /// identifying the `AlertPolicy` objects.
    ///
    /// The field can contain up to 64 entries. Each key and value is limited to
    /// 63 Unicode characters or 128 bytes, whichever is smaller. Labels and
    /// values can contain only lowercase letters, numerals, underscores, and
    /// dashes. Keys must begin with a letter.
    #[prost(map = "string, string", tag = "16")]
    pub user_labels: ::std::collections::HashMap<std::string::String, std::string::String>,
    /// A list of conditions for the policy. The conditions are combined by AND or
    /// OR according to the `combiner` field. If the combined conditions evaluate
    /// to true, then an incident is created. A policy can have from one to six
    /// conditions.
    /// If `condition_time_series_query_language` is present, it must be the only
    /// `condition`.
    #[prost(message, repeated, tag = "12")]
    pub conditions: ::std::vec::Vec<alert_policy::Condition>,
    /// How to combine the results of multiple conditions to determine if an
    /// incident should be opened.
    /// If `condition_time_series_query_language` is present, this must be
    /// `COMBINE_UNSPECIFIED`.
    #[prost(enumeration = "alert_policy::ConditionCombinerType", tag = "6")]
    pub combiner: i32,
    /// Whether or not the policy is enabled. On write, the default interpretation
    /// if unset is that the policy is enabled. On read, clients should not make
    /// any assumption about the state if it has not been populated. The
    /// field should always be populated on List and Get operations, unless
    /// a field projection has been specified that strips it out.
    #[prost(message, optional, tag = "17")]
    pub enabled: ::std::option::Option<bool>,
    /// Read-only description of how the alert policy is invalid. OK if the alert
    /// policy is valid. If not OK, the alert policy will not generate incidents.
    #[prost(message, optional, tag = "18")]
    pub validity: ::std::option::Option<super::super::rpc::Status>,
    /// Identifies the notification channels to which notifications should be sent
    /// when incidents are opened or closed or when new violations occur on
    /// an already opened incident. Each element of this array corresponds to
    /// the `name` field in each of the
    /// [`NotificationChannel`][google.monitoring.v3.NotificationChannel]
    /// objects that are returned from the [`ListNotificationChannels`]
    /// [google.monitoring.v3.NotificationChannelService.ListNotificationChannels]
    /// method. The format of the entries in this field is:
    ///
    ///     projects/[PROJECT_ID_OR_NUMBER]/notificationChannels/[CHANNEL_ID]
    #[prost(string, repeated, tag = "14")]
    pub notification_channels: ::std::vec::Vec<std::string::String>,
    /// A read-only record of the creation of the alerting policy. If provided
    /// in a call to create or update, this field will be ignored.
    #[prost(message, optional, tag = "10")]
    pub creation_record: ::std::option::Option<MutationRecord>,
    /// A read-only record of the most recent change to the alerting policy. If
    /// provided in a call to create or update, this field will be ignored.
    #[prost(message, optional, tag = "11")]
    pub mutation_record: ::std::option::Option<MutationRecord>,
}
pub mod alert_policy {
    /// A content string and a MIME type that describes the content string's
    /// format.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Documentation {
        /// The text of the documentation, interpreted according to `mime_type`.
        /// The content may not exceed 8,192 Unicode characters and may not exceed
        /// more than 10,240 bytes when encoded in UTF-8 format, whichever is
        /// smaller.
        #[prost(string, tag = "1")]
        pub content: std::string::String,
        /// The format of the `content` field. Presently, only the value
        /// `"text/markdown"` is supported. See
        /// [Markdown](https://en.wikipedia.org/wiki/Markdown) for more information.
        #[prost(string, tag = "2")]
        pub mime_type: std::string::String,
    }
    /// A condition is a true/false test that determines when an alerting policy
    /// should open an incident. If a condition evaluates to true, it signifies
    /// that something is wrong.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Condition {
        /// Required if the condition exists. The unique resource name for this
        /// condition. Its format is:
        ///
        ///     projects/[PROJECT_ID_OR_NUMBER]/alertPolicies/[POLICY_ID]/conditions/[CONDITION_ID]
        ///
        /// `[CONDITION_ID]` is assigned by Stackdriver Monitoring when the
        /// condition is created as part of a new or updated alerting policy.
        ///
        /// When calling the
        /// [alertPolicies.create][google.monitoring.v3.AlertPolicyService.CreateAlertPolicy]
        /// method, do not include the `name` field in the conditions of the
        /// requested alerting policy. Stackdriver Monitoring creates the
        /// condition identifiers and includes them in the new policy.
        ///
        /// When calling the
        /// [alertPolicies.update][google.monitoring.v3.AlertPolicyService.UpdateAlertPolicy]
        /// method to update a policy, including a condition `name` causes the
        /// existing condition to be updated. Conditions without names are added to
        /// the updated policy. Existing conditions are deleted if they are not
        /// updated.
        ///
        /// Best practice is to preserve `[CONDITION_ID]` if you make only small
        /// changes, such as those to condition thresholds, durations, or trigger
        /// values.  Otherwise, treat the change as a new condition and let the
        /// existing condition be deleted.
        #[prost(string, tag = "12")]
        pub name: std::string::String,
        /// A short name or phrase used to identify the condition in dashboards,
        /// notifications, and incidents. To avoid confusion, don't use the same
        /// display name for multiple conditions in the same policy.
        #[prost(string, tag = "6")]
        pub display_name: std::string::String,
        /// Only one of the following condition types will be specified.
        #[prost(oneof = "condition::Condition", tags = "1, 2")]
        pub condition: ::std::option::Option<condition::Condition>,
    }
    pub mod condition {
        /// Specifies how many time series must fail a predicate to trigger a
        /// condition. If not specified, then a `{count: 1}` trigger is used.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Trigger {
            /// A type of trigger.
            #[prost(oneof = "trigger::Type", tags = "1, 2")]
            pub r#type: ::std::option::Option<trigger::Type>,
        }
        pub mod trigger {
            /// A type of trigger.
            #[derive(Clone, PartialEq, ::prost::Oneof)]
            pub enum Type {
                /// The absolute number of time series that must fail
                /// the predicate for the condition to be triggered.
                #[prost(int32, tag = "1")]
                Count(i32),
                /// The percentage of time series that must fail the
                /// predicate for the condition to be triggered.
                #[prost(double, tag = "2")]
                Percent(f64),
            }
        }
        /// A condition type that compares a collection of time series
        /// against a threshold.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct MetricThreshold {
            /// A [filter](https://cloud.google.com/monitoring/api/v3/filters) that
            /// identifies which time series should be compared with the threshold.
            ///
            /// The filter is similar to the one that is specified in the
            /// [`ListTimeSeries`
            /// request](https://cloud.google.com/monitoring/api/ref_v3/rest/v3/projects.timeSeries/list)
            /// (that call is useful to verify the time series that will be retrieved /
            /// processed) and must specify the metric type and optionally may contain
            /// restrictions on resource type, resource labels, and metric labels.
            /// This field may not exceed 2048 Unicode characters in length.
            #[prost(string, tag = "2")]
            pub filter: std::string::String,
            /// Specifies the alignment of data points in individual time series as
            /// well as how to combine the retrieved time series together (such as
            /// when aggregating multiple streams on each resource to a single
            /// stream for each resource or when aggregating streams across all
            /// members of a group of resrouces). Multiple aggregations
            /// are applied in the order specified.
            ///
            /// This field is similar to the one in the [`ListTimeSeries`
            /// request](https://cloud.google.com/monitoring/api/ref_v3/rest/v3/projects.timeSeries/list).
            /// It is advisable to use the `ListTimeSeries` method when debugging this
            /// field.
            #[prost(message, repeated, tag = "8")]
            pub aggregations: ::std::vec::Vec<super::super::Aggregation>,
            /// A [filter](https://cloud.google.com/monitoring/api/v3/filters) that
            /// identifies a time series that should be used as the denominator of a
            /// ratio that will be compared with the threshold. If a
            /// `denominator_filter` is specified, the time series specified by the
            /// `filter` field will be used as the numerator.
            ///
            /// The filter must specify the metric type and optionally may contain
            /// restrictions on resource type, resource labels, and metric labels.
            /// This field may not exceed 2048 Unicode characters in length.
            #[prost(string, tag = "9")]
            pub denominator_filter: std::string::String,
            /// Specifies the alignment of data points in individual time series
            /// selected by `denominatorFilter` as
            /// well as how to combine the retrieved time series together (such as
            /// when aggregating multiple streams on each resource to a single
            /// stream for each resource or when aggregating streams across all
            /// members of a group of resources).
            ///
            /// When computing ratios, the `aggregations` and
            /// `denominator_aggregations` fields must use the same alignment period
            /// and produce time series that have the same periodicity and labels.
            #[prost(message, repeated, tag = "10")]
            pub denominator_aggregations: ::std::vec::Vec<super::super::Aggregation>,
            /// The comparison to apply between the time series (indicated by `filter`
            /// and `aggregation`) and the threshold (indicated by `threshold_value`).
            /// The comparison is applied on each time series, with the time series
            /// on the left-hand side and the threshold on the right-hand side.
            ///
            /// Only `COMPARISON_LT` and `COMPARISON_GT` are supported currently.
            #[prost(enumeration = "super::super::ComparisonType", tag = "4")]
            pub comparison: i32,
            /// A value against which to compare the time series.
            #[prost(double, tag = "5")]
            pub threshold_value: f64,
            /// The amount of time that a time series must violate the
            /// threshold to be considered failing. Currently, only values
            /// that are a multiple of a minute--e.g., 0, 60, 120, or 300
            /// seconds--are supported. If an invalid value is given, an
            /// error will be returned. When choosing a duration, it is useful to
            /// keep in mind the frequency of the underlying time series data
            /// (which may also be affected by any alignments specified in the
            /// `aggregations` field); a good duration is long enough so that a single
            /// outlier does not generate spurious alerts, but short enough that
            /// unhealthy states are detected and alerted on quickly.
            #[prost(message, optional, tag = "6")]
            pub duration: ::std::option::Option<::prost_types::Duration>,
            /// The number/percent of time series for which the comparison must hold
            /// in order for the condition to trigger. If unspecified, then the
            /// condition will trigger if the comparison is true for any of the
            /// time series that have been identified by `filter` and `aggregations`,
            /// or by the ratio, if `denominator_filter` and `denominator_aggregations`
            /// are specified.
            #[prost(message, optional, tag = "7")]
            pub trigger: ::std::option::Option<Trigger>,
        }
        /// A condition type that checks that monitored resources
        /// are reporting data. The configuration defines a metric and
        /// a set of monitored resources. The predicate is considered in violation
        /// when a time series for the specified metric of a monitored
        /// resource does not include any data in the specified `duration`.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct MetricAbsence {
            /// A [filter](https://cloud.google.com/monitoring/api/v3/filters) that
            /// identifies which time series should be compared with the threshold.
            ///
            /// The filter is similar to the one that is specified in the
            /// [`ListTimeSeries`
            /// request](https://cloud.google.com/monitoring/api/ref_v3/rest/v3/projects.timeSeries/list)
            /// (that call is useful to verify the time series that will be retrieved /
            /// processed) and must specify the metric type and optionally may contain
            /// restrictions on resource type, resource labels, and metric labels.
            /// This field may not exceed 2048 Unicode characters in length.
            #[prost(string, tag = "1")]
            pub filter: std::string::String,
            /// Specifies the alignment of data points in individual time series as
            /// well as how to combine the retrieved time series together (such as
            /// when aggregating multiple streams on each resource to a single
            /// stream for each resource or when aggregating streams across all
            /// members of a group of resrouces). Multiple aggregations
            /// are applied in the order specified.
            ///
            /// This field is similar to the one in the [`ListTimeSeries`
            /// request](https://cloud.google.com/monitoring/api/ref_v3/rest/v3/projects.timeSeries/list).
            /// It is advisable to use the `ListTimeSeries` method when debugging this
            /// field.
            #[prost(message, repeated, tag = "5")]
            pub aggregations: ::std::vec::Vec<super::super::Aggregation>,
            /// The amount of time that a time series must fail to report new
            /// data to be considered failing. Currently, only values that
            /// are a multiple of a minute--e.g.  60, 120, or 300
            /// seconds--are supported. If an invalid value is given, an
            /// error will be returned. The `Duration.nanos` field is
            /// ignored.
            #[prost(message, optional, tag = "2")]
            pub duration: ::std::option::Option<::prost_types::Duration>,
            /// The number/percent of time series for which the comparison must hold
            /// in order for the condition to trigger. If unspecified, then the
            /// condition will trigger if the comparison is true for any of the
            /// time series that have been identified by `filter` and `aggregations`.
            #[prost(message, optional, tag = "3")]
            pub trigger: ::std::option::Option<Trigger>,
        }
        /// Only one of the following condition types will be specified.
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Condition {
            /// A condition that compares a time series against a threshold.
            #[prost(message, tag = "1")]
            ConditionThreshold(MetricThreshold),
            /// A condition that checks that a time series continues to
            /// receive new data points.
            #[prost(message, tag = "2")]
            ConditionAbsent(MetricAbsence),
        }
    }
    /// Operators for combining conditions.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ConditionCombinerType {
        /// An unspecified combiner.
        CombineUnspecified = 0,
        /// Combine conditions using the logical `AND` operator. An
        /// incident is created only if all the conditions are met
        /// simultaneously. This combiner is satisfied if all conditions are
        /// met, even if they are met on completely different resources.
        And = 1,
        /// Combine conditions using the logical `OR` operator. An incident
        /// is created if any of the listed conditions is met.
        Or = 2,
        /// Combine conditions using logical `AND` operator, but unlike the regular
        /// `AND` option, an incident is created only if all conditions are met
        /// simultaneously on at least one resource.
        AndWithMatchingResource = 3,
    }
}
/// The protocol for the `CreateAlertPolicy` request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateAlertPolicyRequest {
    /// Required. The project in which to create the alerting policy. The format is:
    ///
    ///     projects/[PROJECT_ID_OR_NUMBER]
    ///
    /// Note that this field names the parent container in which the alerting
    /// policy will be written, not the name of the created policy. The alerting
    /// policy that is returned will have a name that contains a normalized
    /// representation of this name as a prefix but adds a suffix of the form
    /// `/alertPolicies/[ALERT_POLICY_ID]`, identifying the policy in the
    /// container.
    #[prost(string, tag = "3")]
    pub name: std::string::String,
    /// Required. The requested alerting policy. You should omit the `name` field in this
    /// policy. The name will be returned in the new policy, including
    /// a new `[ALERT_POLICY_ID]` value.
    #[prost(message, optional, tag = "2")]
    pub alert_policy: ::std::option::Option<AlertPolicy>,
}
/// The protocol for the `GetAlertPolicy` request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAlertPolicyRequest {
    /// Required. The alerting policy to retrieve. The format is:
    ///
    ///     projects/[PROJECT_ID_OR_NUMBER]/alertPolicies/[ALERT_POLICY_ID]
    #[prost(string, tag = "3")]
    pub name: std::string::String,
}
/// The protocol for the `ListAlertPolicies` request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAlertPoliciesRequest {
    /// Required. The project whose alert policies are to be listed. The format is:
    ///
    ///     projects/[PROJECT_ID_OR_NUMBER]
    ///
    /// Note that this field names the parent container in which the alerting
    /// policies to be listed are stored. To retrieve a single alerting policy
    /// by name, use the
    /// [GetAlertPolicy][google.monitoring.v3.AlertPolicyService.GetAlertPolicy]
    /// operation, instead.
    #[prost(string, tag = "4")]
    pub name: std::string::String,
    /// If provided, this field specifies the criteria that must be met by
    /// alert policies to be included in the response.
    ///
    /// For more details, see [sorting and
    /// filtering](https://cloud.google.com/monitoring/api/v3/sorting-and-filtering).
    #[prost(string, tag = "5")]
    pub filter: std::string::String,
    /// A comma-separated list of fields by which to sort the result. Supports
    /// the same set of field references as the `filter` field. Entries can be
    /// prefixed with a minus sign to sort by the field in descending order.
    ///
    /// For more details, see [sorting and
    /// filtering](https://cloud.google.com/monitoring/api/v3/sorting-and-filtering).
    #[prost(string, tag = "6")]
    pub order_by: std::string::String,
    /// The maximum number of results to return in a single response.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// If this field is not empty then it must contain the `nextPageToken` value
    /// returned by a previous call to this method.  Using this field causes the
    /// method to return more results from the previous method call.
    #[prost(string, tag = "3")]
    pub page_token: std::string::String,
}
/// The protocol for the `ListAlertPolicies` response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAlertPoliciesResponse {
    /// The returned alert policies.
    #[prost(message, repeated, tag = "3")]
    pub alert_policies: ::std::vec::Vec<AlertPolicy>,
    /// If there might be more results than were returned, then this field is set
    /// to a non-empty value. To see the additional results,
    /// use that value as `page_token` in the next call to this method.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
}
/// The protocol for the `UpdateAlertPolicy` request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateAlertPolicyRequest {
    /// Optional. A list of alerting policy field names. If this field is not
    /// empty, each listed field in the existing alerting policy is set to the
    /// value of the corresponding field in the supplied policy (`alert_policy`),
    /// or to the field's default value if the field is not in the supplied
    /// alerting policy.  Fields not listed retain their previous value.
    ///
    /// Examples of valid field masks include `display_name`, `documentation`,
    /// `documentation.content`, `documentation.mime_type`, `user_labels`,
    /// `user_label.nameofkey`, `enabled`, `conditions`, `combiner`, etc.
    ///
    /// If this field is empty, then the supplied alerting policy replaces the
    /// existing policy. It is the same as deleting the existing policy and
    /// adding the supplied policy, except for the following:
    ///
    /// +   The new policy will have the same `[ALERT_POLICY_ID]` as the former
    ///     policy. This gives you continuity with the former policy in your
    ///     notifications and incidents.
    /// +   Conditions in the new policy will keep their former `[CONDITION_ID]` if
    ///     the supplied condition includes the `name` field with that
    ///     `[CONDITION_ID]`. If the supplied condition omits the `name` field,
    ///     then a new `[CONDITION_ID]` is created.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
    /// Required. The updated alerting policy or the updated values for the
    /// fields listed in `update_mask`.
    /// If `update_mask` is not empty, any fields in this policy that are
    /// not in `update_mask` are ignored.
    #[prost(message, optional, tag = "3")]
    pub alert_policy: ::std::option::Option<AlertPolicy>,
}
/// The protocol for the `DeleteAlertPolicy` request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteAlertPolicyRequest {
    /// Required. The alerting policy to delete. The format is:
    ///
    ///     projects/[PROJECT_ID_OR_NUMBER]/alertPolicies/[ALERT_POLICY_ID]
    ///
    /// For more information, see [AlertPolicy][google.monitoring.v3.AlertPolicy].
    #[prost(string, tag = "3")]
    pub name: std::string::String,
}
#[doc = r" Generated client implementations."]
pub mod alert_policy_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " The AlertPolicyService API is used to manage (list, create, delete,"]
    #[doc = " edit) alert policies in Stackdriver Monitoring. An alerting policy is"]
    #[doc = " a description of the conditions under which some aspect of your"]
    #[doc = " system is considered to be \"unhealthy\" and the ways to notify"]
    #[doc = " people or services about this state. In addition to using this API, alert"]
    #[doc = " policies can also be managed through"]
    #[doc = " [Stackdriver Monitoring](https://cloud.google.com/monitoring/docs/),"]
    #[doc = " which can be reached by clicking the \"Monitoring\" tab in"]
    #[doc = " [Cloud Console](https://console.cloud.google.com/)."]
    pub struct AlertPolicyServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> AlertPolicyServiceClient<T>
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
        #[doc = " Lists the existing alerting policies for the project."]
        pub async fn list_alert_policies(
            &mut self,
            request: impl tonic::IntoRequest<super::ListAlertPoliciesRequest>,
        ) -> Result<tonic::Response<super::ListAlertPoliciesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.monitoring.v3.AlertPolicyService/ListAlertPolicies",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets a single alerting policy."]
        pub async fn get_alert_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::GetAlertPolicyRequest>,
        ) -> Result<tonic::Response<super::AlertPolicy>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.monitoring.v3.AlertPolicyService/GetAlertPolicy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a new alerting policy."]
        pub async fn create_alert_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateAlertPolicyRequest>,
        ) -> Result<tonic::Response<super::AlertPolicy>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.monitoring.v3.AlertPolicyService/CreateAlertPolicy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes an alerting policy."]
        pub async fn delete_alert_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteAlertPolicyRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.monitoring.v3.AlertPolicyService/DeleteAlertPolicy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates an alerting policy. You can either replace the entire policy with"]
        #[doc = " a new one or replace only certain fields in the current alerting policy by"]
        #[doc = " specifying the fields to be updated via `updateMask`. Returns the"]
        #[doc = " updated alerting policy."]
        pub async fn update_alert_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateAlertPolicyRequest>,
        ) -> Result<tonic::Response<super::AlertPolicy>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.monitoring.v3.AlertPolicyService/UpdateAlertPolicy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for AlertPolicyServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for AlertPolicyServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "AlertPolicyServiceClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod alert_policy_service_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with AlertPolicyServiceServer."]
    #[async_trait]
    pub trait AlertPolicyService: Send + Sync + 'static {
        #[doc = " Lists the existing alerting policies for the project."]
        async fn list_alert_policies(
            &self,
            request: tonic::Request<super::ListAlertPoliciesRequest>,
        ) -> Result<tonic::Response<super::ListAlertPoliciesResponse>, tonic::Status>;
        #[doc = " Gets a single alerting policy."]
        async fn get_alert_policy(
            &self,
            request: tonic::Request<super::GetAlertPolicyRequest>,
        ) -> Result<tonic::Response<super::AlertPolicy>, tonic::Status>;
        #[doc = " Creates a new alerting policy."]
        async fn create_alert_policy(
            &self,
            request: tonic::Request<super::CreateAlertPolicyRequest>,
        ) -> Result<tonic::Response<super::AlertPolicy>, tonic::Status>;
        #[doc = " Deletes an alerting policy."]
        async fn delete_alert_policy(
            &self,
            request: tonic::Request<super::DeleteAlertPolicyRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status>;
        #[doc = " Updates an alerting policy. You can either replace the entire policy with"]
        #[doc = " a new one or replace only certain fields in the current alerting policy by"]
        #[doc = " specifying the fields to be updated via `updateMask`. Returns the"]
        #[doc = " updated alerting policy."]
        async fn update_alert_policy(
            &self,
            request: tonic::Request<super::UpdateAlertPolicyRequest>,
        ) -> Result<tonic::Response<super::AlertPolicy>, tonic::Status>;
    }
    #[doc = " The AlertPolicyService API is used to manage (list, create, delete,"]
    #[doc = " edit) alert policies in Stackdriver Monitoring. An alerting policy is"]
    #[doc = " a description of the conditions under which some aspect of your"]
    #[doc = " system is considered to be \"unhealthy\" and the ways to notify"]
    #[doc = " people or services about this state. In addition to using this API, alert"]
    #[doc = " policies can also be managed through"]
    #[doc = " [Stackdriver Monitoring](https://cloud.google.com/monitoring/docs/),"]
    #[doc = " which can be reached by clicking the \"Monitoring\" tab in"]
    #[doc = " [Cloud Console](https://console.cloud.google.com/)."]
    #[derive(Debug)]
    #[doc(hidden)]
    pub struct AlertPolicyServiceServer<T: AlertPolicyService> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: AlertPolicyService> AlertPolicyServiceServer<T> {
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
    impl<T, B> Service<http::Request<B>> for AlertPolicyServiceServer<T>
    where
        T: AlertPolicyService,
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
                "/google.monitoring.v3.AlertPolicyService/ListAlertPolicies" => {
                    #[allow(non_camel_case_types)]
                    struct ListAlertPoliciesSvc<T: AlertPolicyService>(pub Arc<T>);
                    impl<T: AlertPolicyService>
                        tonic::server::UnaryService<super::ListAlertPoliciesRequest>
                        for ListAlertPoliciesSvc<T>
                    {
                        type Response = super::ListAlertPoliciesResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListAlertPoliciesRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.list_alert_policies(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ListAlertPoliciesSvc(inner);
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
                "/google.monitoring.v3.AlertPolicyService/GetAlertPolicy" => {
                    #[allow(non_camel_case_types)]
                    struct GetAlertPolicySvc<T: AlertPolicyService>(pub Arc<T>);
                    impl<T: AlertPolicyService>
                        tonic::server::UnaryService<super::GetAlertPolicyRequest>
                        for GetAlertPolicySvc<T>
                    {
                        type Response = super::AlertPolicy;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetAlertPolicyRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.get_alert_policy(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetAlertPolicySvc(inner);
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
                "/google.monitoring.v3.AlertPolicyService/CreateAlertPolicy" => {
                    #[allow(non_camel_case_types)]
                    struct CreateAlertPolicySvc<T: AlertPolicyService>(pub Arc<T>);
                    impl<T: AlertPolicyService>
                        tonic::server::UnaryService<super::CreateAlertPolicyRequest>
                        for CreateAlertPolicySvc<T>
                    {
                        type Response = super::AlertPolicy;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateAlertPolicyRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.create_alert_policy(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = CreateAlertPolicySvc(inner);
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
                "/google.monitoring.v3.AlertPolicyService/DeleteAlertPolicy" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteAlertPolicySvc<T: AlertPolicyService>(pub Arc<T>);
                    impl<T: AlertPolicyService>
                        tonic::server::UnaryService<super::DeleteAlertPolicyRequest>
                        for DeleteAlertPolicySvc<T>
                    {
                        type Response = ();
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteAlertPolicyRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.delete_alert_policy(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = DeleteAlertPolicySvc(inner);
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
                "/google.monitoring.v3.AlertPolicyService/UpdateAlertPolicy" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateAlertPolicySvc<T: AlertPolicyService>(pub Arc<T>);
                    impl<T: AlertPolicyService>
                        tonic::server::UnaryService<super::UpdateAlertPolicyRequest>
                        for UpdateAlertPolicySvc<T>
                    {
                        type Response = super::AlertPolicy;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateAlertPolicyRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.update_alert_policy(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = UpdateAlertPolicySvc(inner);
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
    impl<T: AlertPolicyService> Clone for AlertPolicyServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: AlertPolicyService> Clone for _Inner<T> {
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
/// A set of (label, value) pairs which were dropped during aggregation, attached
/// to google.api.Distribution.Exemplars in google.api.Distribution values during
/// aggregation.
///
/// These values are used in combination with the label values that remain on the
/// aggregated Distribution timeseries to construct the full label set for the
/// exemplar values.  The resulting full label set may be used to identify the
/// specific task/job/instance (for example) which may be contributing to a
/// long-tail, while allowing the storage savings of only storing aggregated
/// distribution values for a large group.
///
/// Note that there are no guarantees on ordering of the labels from
/// exemplar-to-exemplar and from distribution-to-distribution in the same
/// stream, and there may be duplicates.  It is up to clients to resolve any
/// ambiguities.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DroppedLabels {
    /// Map from label to its value, for all labels dropped in any aggregation.
    #[prost(map = "string, string", tag = "1")]
    pub label: ::std::collections::HashMap<std::string::String, std::string::String>,
}
/// The description of a dynamic collection of monitored resources. Each group
/// has a filter that is matched against monitored resources and their associated
/// metadata. If a group's filter matches an available monitored resource, then
/// that resource is a member of that group.  Groups can contain any number of
/// monitored resources, and each monitored resource can be a member of any
/// number of groups.
///
/// Groups can be nested in parent-child hierarchies. The `parentName` field
/// identifies an optional parent for each group.  If a group has a parent, then
/// the only monitored resources available to be matched by the group's filter
/// are the resources contained in the parent group.  In other words, a group
/// contains the monitored resources that match its filter and the filters of all
/// the group's ancestors.  A group without a parent can contain any monitored
/// resource.
///
/// For example, consider an infrastructure running a set of instances with two
/// user-defined tags: `"environment"` and `"role"`. A parent group has a filter,
/// `environment="production"`.  A child of that parent group has a filter,
/// `role="transcoder"`.  The parent group contains all instances in the
/// production environment, regardless of their roles.  The child group contains
/// instances that have the transcoder role *and* are in the production
/// environment.
///
/// The monitored resources contained in a group can change at any moment,
/// depending on what resources exist and what filters are associated with the
/// group and its ancestors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Group {
    /// Output only. The name of this group. The format is:
    ///
    ///     projects/[PROJECT_ID_OR_NUMBER]/groups/[GROUP_ID]
    ///
    /// When creating a group, this field is ignored and a new name is created
    /// consisting of the project specified in the call to `CreateGroup`
    /// and a unique `[GROUP_ID]` that is generated automatically.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// A user-assigned name for this group, used only for display purposes.
    #[prost(string, tag = "2")]
    pub display_name: std::string::String,
    /// The name of the group's parent, if it has one. The format is:
    ///
    ///     projects/[PROJECT_ID_OR_NUMBER]/groups/[GROUP_ID]
    ///
    /// For groups with no parent, `parent_name` is the empty string, `""`.
    #[prost(string, tag = "3")]
    pub parent_name: std::string::String,
    /// The filter used to determine which monitored resources belong to this
    /// group.
    #[prost(string, tag = "5")]
    pub filter: std::string::String,
    /// If true, the members of this group are considered to be a cluster.
    /// The system can perform additional analysis on groups that are clusters.
    #[prost(bool, tag = "6")]
    pub is_cluster: bool,
}
/// The `ListGroup` request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListGroupsRequest {
    /// Required. The project whose groups are to be listed. The format is:
    ///
    ///     projects/[PROJECT_ID_OR_NUMBER]
    #[prost(string, tag = "7")]
    pub name: std::string::String,
    /// A positive number that is the maximum number of results to return.
    #[prost(int32, tag = "5")]
    pub page_size: i32,
    /// If this field is not empty then it must contain the `next_page_token` value
    /// returned by a previous call to this method.  Using this field causes the
    /// method to return additional results from the previous method call.
    #[prost(string, tag = "6")]
    pub page_token: std::string::String,
    /// An optional filter consisting of a single group name.  The filters limit
    /// the groups returned based on their parent-child relationship with the
    /// specified group. If no filter is specified, all groups are returned.
    #[prost(oneof = "list_groups_request::Filter", tags = "2, 3, 4")]
    pub filter: ::std::option::Option<list_groups_request::Filter>,
}
pub mod list_groups_request {
    /// An optional filter consisting of a single group name.  The filters limit
    /// the groups returned based on their parent-child relationship with the
    /// specified group. If no filter is specified, all groups are returned.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Filter {
        /// A group name. The format is:
        ///
        ///     projects/[PROJECT_ID_OR_NUMBER]/groups/[GROUP_ID]
        ///
        /// Returns groups whose `parent_name` field contains the group
        /// name.  If no groups have this parent, the results are empty.
        #[prost(string, tag = "2")]
        ChildrenOfGroup(std::string::String),
        /// A group name. The format is:
        ///
        ///     projects/[PROJECT_ID_OR_NUMBER]/groups/[GROUP_ID]
        ///
        /// Returns groups that are ancestors of the specified group.
        /// The groups are returned in order, starting with the immediate parent and
        /// ending with the most distant ancestor.  If the specified group has no
        /// immediate parent, the results are empty.
        #[prost(string, tag = "3")]
        AncestorsOfGroup(std::string::String),
        /// A group name. The format is:
        ///
        ///     projects/[PROJECT_ID_OR_NUMBER]/groups/[GROUP_ID]
        ///
        /// Returns the descendants of the specified group.  This is a superset of
        /// the results returned by the `children_of_group` filter, and includes
        /// children-of-children, and so forth.
        #[prost(string, tag = "4")]
        DescendantsOfGroup(std::string::String),
    }
}
/// The `ListGroups` response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListGroupsResponse {
    /// The groups that match the specified filters.
    #[prost(message, repeated, tag = "1")]
    pub group: ::std::vec::Vec<Group>,
    /// If there are more results than have been returned, then this field is set
    /// to a non-empty value.  To see the additional results,
    /// use that value as `page_token` in the next call to this method.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
}
/// The `GetGroup` request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetGroupRequest {
    /// Required. The group to retrieve. The format is:
    ///
    ///     projects/[PROJECT_ID_OR_NUMBER]/groups/[GROUP_ID]
    #[prost(string, tag = "3")]
    pub name: std::string::String,
}
/// The `CreateGroup` request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateGroupRequest {
    /// Required. The project in which to create the group. The format is:
    ///
    ///     projects/[PROJECT_ID_OR_NUMBER]
    #[prost(string, tag = "4")]
    pub name: std::string::String,
    /// Required. A group definition. It is an error to define the `name` field because
    /// the system assigns the name.
    #[prost(message, optional, tag = "2")]
    pub group: ::std::option::Option<Group>,
    /// If true, validate this request but do not create the group.
    #[prost(bool, tag = "3")]
    pub validate_only: bool,
}
/// The `UpdateGroup` request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateGroupRequest {
    /// Required. The new definition of the group.  All fields of the existing group,
    /// excepting `name`, are replaced with the corresponding fields of this group.
    #[prost(message, optional, tag = "2")]
    pub group: ::std::option::Option<Group>,
    /// If true, validate this request but do not update the existing group.
    #[prost(bool, tag = "3")]
    pub validate_only: bool,
}
/// The `DeleteGroup` request. The default behavior is to be able to delete a
/// single group without any descendants.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteGroupRequest {
    /// Required. The group to delete. The format is:
    ///
    ///     projects/[PROJECT_ID_OR_NUMBER]/groups/[GROUP_ID]
    #[prost(string, tag = "3")]
    pub name: std::string::String,
    /// If this field is true, then the request means to delete a group with all
    /// its descendants. Otherwise, the request means to delete a group only when
    /// it has no descendants. The default value is false.
    #[prost(bool, tag = "4")]
    pub recursive: bool,
}
/// The `ListGroupMembers` request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListGroupMembersRequest {
    /// Required. The group whose members are listed. The format is:
    ///
    ///     projects/[PROJECT_ID_OR_NUMBER]/groups/[GROUP_ID]
    #[prost(string, tag = "7")]
    pub name: std::string::String,
    /// A positive number that is the maximum number of results to return.
    #[prost(int32, tag = "3")]
    pub page_size: i32,
    /// If this field is not empty then it must contain the `next_page_token` value
    /// returned by a previous call to this method.  Using this field causes the
    /// method to return additional results from the previous method call.
    #[prost(string, tag = "4")]
    pub page_token: std::string::String,
    /// An optional [list
    /// filter](https://cloud.google.com/monitoring/api/learn_more#filtering)
    /// describing the members to be returned.  The filter may reference the type,
    /// labels, and metadata of monitored resources that comprise the group. For
    /// example, to return only resources representing Compute Engine VM instances,
    /// use this filter:
    ///
    ///     `resource.type = "gce_instance"`
    #[prost(string, tag = "5")]
    pub filter: std::string::String,
    /// An optional time interval for which results should be returned. Only
    /// members that were part of the group during the specified interval are
    /// included in the response.  If no interval is provided then the group
    /// membership over the last minute is returned.
    #[prost(message, optional, tag = "6")]
    pub interval: ::std::option::Option<TimeInterval>,
}
/// The `ListGroupMembers` response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListGroupMembersResponse {
    /// A set of monitored resources in the group.
    #[prost(message, repeated, tag = "1")]
    pub members: ::std::vec::Vec<super::super::api::MonitoredResource>,
    /// If there are more results than have been returned, then this field is
    /// set to a non-empty value.  To see the additional results, use that value as
    /// `page_token` in the next call to this method.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
    /// The total number of elements matching this request.
    #[prost(int32, tag = "3")]
    pub total_size: i32,
}
#[doc = r" Generated client implementations."]
pub mod group_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " The Group API lets you inspect and manage your"]
    #[doc = " [groups](#google.monitoring.v3.Group)."]
    #[doc = ""]
    #[doc = " A group is a named filter that is used to identify"]
    #[doc = " a collection of monitored resources. Groups are typically used to"]
    #[doc = " mirror the physical and/or logical topology of the environment."]
    #[doc = " Because group membership is computed dynamically, monitored"]
    #[doc = " resources that are started in the future are automatically placed"]
    #[doc = " in matching groups. By using a group to name monitored resources in,"]
    #[doc = " for example, an alert policy, the target of that alert policy is"]
    #[doc = " updated automatically as monitored resources are added and removed"]
    #[doc = " from the infrastructure."]
    pub struct GroupServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> GroupServiceClient<T>
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
        #[doc = " Lists the existing groups."]
        pub async fn list_groups(
            &mut self,
            request: impl tonic::IntoRequest<super::ListGroupsRequest>,
        ) -> Result<tonic::Response<super::ListGroupsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.monitoring.v3.GroupService/ListGroups",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets a single group."]
        pub async fn get_group(
            &mut self,
            request: impl tonic::IntoRequest<super::GetGroupRequest>,
        ) -> Result<tonic::Response<super::Group>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/google.monitoring.v3.GroupService/GetGroup");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a new group."]
        pub async fn create_group(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateGroupRequest>,
        ) -> Result<tonic::Response<super::Group>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.monitoring.v3.GroupService/CreateGroup",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates an existing group."]
        #[doc = " You can change any group attributes except `name`."]
        pub async fn update_group(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateGroupRequest>,
        ) -> Result<tonic::Response<super::Group>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.monitoring.v3.GroupService/UpdateGroup",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes an existing group."]
        pub async fn delete_group(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteGroupRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.monitoring.v3.GroupService/DeleteGroup",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists the monitored resources that are members of a group."]
        pub async fn list_group_members(
            &mut self,
            request: impl tonic::IntoRequest<super::ListGroupMembersRequest>,
        ) -> Result<tonic::Response<super::ListGroupMembersResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.monitoring.v3.GroupService/ListGroupMembers",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for GroupServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for GroupServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "GroupServiceClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod group_service_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with GroupServiceServer."]
    #[async_trait]
    pub trait GroupService: Send + Sync + 'static {
        #[doc = " Lists the existing groups."]
        async fn list_groups(
            &self,
            request: tonic::Request<super::ListGroupsRequest>,
        ) -> Result<tonic::Response<super::ListGroupsResponse>, tonic::Status>;
        #[doc = " Gets a single group."]
        async fn get_group(
            &self,
            request: tonic::Request<super::GetGroupRequest>,
        ) -> Result<tonic::Response<super::Group>, tonic::Status>;
        #[doc = " Creates a new group."]
        async fn create_group(
            &self,
            request: tonic::Request<super::CreateGroupRequest>,
        ) -> Result<tonic::Response<super::Group>, tonic::Status>;
        #[doc = " Updates an existing group."]
        #[doc = " You can change any group attributes except `name`."]
        async fn update_group(
            &self,
            request: tonic::Request<super::UpdateGroupRequest>,
        ) -> Result<tonic::Response<super::Group>, tonic::Status>;
        #[doc = " Deletes an existing group."]
        async fn delete_group(
            &self,
            request: tonic::Request<super::DeleteGroupRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status>;
        #[doc = " Lists the monitored resources that are members of a group."]
        async fn list_group_members(
            &self,
            request: tonic::Request<super::ListGroupMembersRequest>,
        ) -> Result<tonic::Response<super::ListGroupMembersResponse>, tonic::Status>;
    }
    #[doc = " The Group API lets you inspect and manage your"]
    #[doc = " [groups](#google.monitoring.v3.Group)."]
    #[doc = ""]
    #[doc = " A group is a named filter that is used to identify"]
    #[doc = " a collection of monitored resources. Groups are typically used to"]
    #[doc = " mirror the physical and/or logical topology of the environment."]
    #[doc = " Because group membership is computed dynamically, monitored"]
    #[doc = " resources that are started in the future are automatically placed"]
    #[doc = " in matching groups. By using a group to name monitored resources in,"]
    #[doc = " for example, an alert policy, the target of that alert policy is"]
    #[doc = " updated automatically as monitored resources are added and removed"]
    #[doc = " from the infrastructure."]
    #[derive(Debug)]
    #[doc(hidden)]
    pub struct GroupServiceServer<T: GroupService> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: GroupService> GroupServiceServer<T> {
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
    impl<T, B> Service<http::Request<B>> for GroupServiceServer<T>
    where
        T: GroupService,
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
                "/google.monitoring.v3.GroupService/ListGroups" => {
                    #[allow(non_camel_case_types)]
                    struct ListGroupsSvc<T: GroupService>(pub Arc<T>);
                    impl<T: GroupService> tonic::server::UnaryService<super::ListGroupsRequest> for ListGroupsSvc<T> {
                        type Response = super::ListGroupsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListGroupsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.list_groups(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ListGroupsSvc(inner);
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
                "/google.monitoring.v3.GroupService/GetGroup" => {
                    #[allow(non_camel_case_types)]
                    struct GetGroupSvc<T: GroupService>(pub Arc<T>);
                    impl<T: GroupService> tonic::server::UnaryService<super::GetGroupRequest> for GetGroupSvc<T> {
                        type Response = super::Group;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetGroupRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.get_group(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetGroupSvc(inner);
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
                "/google.monitoring.v3.GroupService/CreateGroup" => {
                    #[allow(non_camel_case_types)]
                    struct CreateGroupSvc<T: GroupService>(pub Arc<T>);
                    impl<T: GroupService> tonic::server::UnaryService<super::CreateGroupRequest> for CreateGroupSvc<T> {
                        type Response = super::Group;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateGroupRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.create_group(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = CreateGroupSvc(inner);
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
                "/google.monitoring.v3.GroupService/UpdateGroup" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateGroupSvc<T: GroupService>(pub Arc<T>);
                    impl<T: GroupService> tonic::server::UnaryService<super::UpdateGroupRequest> for UpdateGroupSvc<T> {
                        type Response = super::Group;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateGroupRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.update_group(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = UpdateGroupSvc(inner);
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
                "/google.monitoring.v3.GroupService/DeleteGroup" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteGroupSvc<T: GroupService>(pub Arc<T>);
                    impl<T: GroupService> tonic::server::UnaryService<super::DeleteGroupRequest> for DeleteGroupSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteGroupRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.delete_group(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = DeleteGroupSvc(inner);
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
                "/google.monitoring.v3.GroupService/ListGroupMembers" => {
                    #[allow(non_camel_case_types)]
                    struct ListGroupMembersSvc<T: GroupService>(pub Arc<T>);
                    impl<T: GroupService>
                        tonic::server::UnaryService<super::ListGroupMembersRequest>
                        for ListGroupMembersSvc<T>
                    {
                        type Response = super::ListGroupMembersResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListGroupMembersRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.list_group_members(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ListGroupMembersSvc(inner);
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
    impl<T: GroupService> Clone for GroupServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: GroupService> Clone for _Inner<T> {
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
/// A single data point in a time series.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Point {
    /// The time interval to which the data point applies.  For `GAUGE` metrics,
    /// the start time is optional, but if it is supplied, it must equal the
    /// end time.  For `DELTA` metrics, the start
    /// and end time should specify a non-zero interval, with subsequent points
    /// specifying contiguous and non-overlapping intervals.  For `CUMULATIVE`
    /// metrics, the start and end time should specify a non-zero interval, with
    /// subsequent points specifying the same start time and increasing end times,
    /// until an event resets the cumulative value to zero and sets a new start
    /// time for the following points.
    #[prost(message, optional, tag = "1")]
    pub interval: ::std::option::Option<TimeInterval>,
    /// The value of the data point.
    #[prost(message, optional, tag = "2")]
    pub value: ::std::option::Option<TypedValue>,
}
/// A collection of data points that describes the time-varying values
/// of a metric. A time series is identified by a combination of a
/// fully-specified monitored resource and a fully-specified metric.
/// This type is used for both listing and creating time series.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TimeSeries {
    /// The associated metric. A fully-specified metric used to identify the time
    /// series.
    #[prost(message, optional, tag = "1")]
    pub metric: ::std::option::Option<super::super::api::Metric>,
    /// The associated monitored resource.  Custom metrics can use only certain
    /// monitored resource types in their time series data.
    #[prost(message, optional, tag = "2")]
    pub resource: ::std::option::Option<super::super::api::MonitoredResource>,
    /// Output only. The associated monitored resource metadata. When reading a
    /// a timeseries, this field will include metadata labels that are explicitly
    /// named in the reduction. When creating a timeseries, this field is ignored.
    #[prost(message, optional, tag = "7")]
    pub metadata: ::std::option::Option<super::super::api::MonitoredResourceMetadata>,
    /// The metric kind of the time series. When listing time series, this metric
    /// kind might be different from the metric kind of the associated metric if
    /// this time series is an alignment or reduction of other time series.
    ///
    /// When creating a time series, this field is optional. If present, it must be
    /// the same as the metric kind of the associated metric. If the associated
    /// metric's descriptor must be auto-created, then this field specifies the
    /// metric kind of the new descriptor and must be either `GAUGE` (the default)
    /// or `CUMULATIVE`.
    #[prost(
        enumeration = "super::super::api::metric_descriptor::MetricKind",
        tag = "3"
    )]
    pub metric_kind: i32,
    /// The value type of the time series. When listing time series, this value
    /// type might be different from the value type of the associated metric if
    /// this time series is an alignment or reduction of other time series.
    ///
    /// When creating a time series, this field is optional. If present, it must be
    /// the same as the type of the data in the `points` field.
    #[prost(
        enumeration = "super::super::api::metric_descriptor::ValueType",
        tag = "4"
    )]
    pub value_type: i32,
    /// The data points of this time series. When listing time series, points are
    /// returned in reverse time order.
    ///
    /// When creating a time series, this field must contain exactly one point and
    /// the point's type must be the same as the value type of the associated
    /// metric. If the associated metric's descriptor must be auto-created, then
    /// the value type of the descriptor is determined by the point's type, which
    /// must be `BOOL`, `INT64`, `DOUBLE`, or `DISTRIBUTION`.
    #[prost(message, repeated, tag = "5")]
    pub points: ::std::vec::Vec<Point>,
}
/// A descriptor for the labels and points in a timeseries.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TimeSeriesDescriptor {
    /// Descriptors for the labels.
    #[prost(message, repeated, tag = "1")]
    pub label_descriptors: ::std::vec::Vec<super::super::api::LabelDescriptor>,
    /// Descriptors for the point data value columns.
    #[prost(message, repeated, tag = "5")]
    pub point_descriptors: ::std::vec::Vec<time_series_descriptor::ValueDescriptor>,
}
pub mod time_series_descriptor {
    /// A descriptor for the value columns in a data point.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ValueDescriptor {
        /// The value key.
        #[prost(string, tag = "1")]
        pub key: std::string::String,
        /// The value type.
        #[prost(
            enumeration = "super::super::super::api::metric_descriptor::ValueType",
            tag = "2"
        )]
        pub value_type: i32,
        /// The value stream kind.
        #[prost(
            enumeration = "super::super::super::api::metric_descriptor::MetricKind",
            tag = "3"
        )]
        pub metric_kind: i32,
    }
}
/// Represents the values of a time series associated with a
/// TimeSeriesDescriptor.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TimeSeriesData {
    /// The values of the labels in the time series identifier, given in the same
    /// order as the `label_descriptors` field of the TimeSeriesDescriptor
    /// associated with this object. Each value must have a value of the type
    /// given in the corresponding entry of `label_descriptors`.
    #[prost(message, repeated, tag = "1")]
    pub label_values: ::std::vec::Vec<LabelValue>,
    /// The points in the time series.
    #[prost(message, repeated, tag = "2")]
    pub point_data: ::std::vec::Vec<time_series_data::PointData>,
}
pub mod time_series_data {
    /// A point's value columns and time interval. Each point has one or more
    /// point values corresponding to the entries in `point_descriptors` field in
    /// the TimeSeriesDescriptor associated with this object.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PointData {
        /// The values that make up the point.
        #[prost(message, repeated, tag = "1")]
        pub values: ::std::vec::Vec<super::TypedValue>,
        /// The time interval associated with the point.
        #[prost(message, optional, tag = "2")]
        pub time_interval: ::std::option::Option<super::TimeInterval>,
    }
}
/// A label value.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LabelValue {
    /// The label value can be a bool, int64, or string.
    #[prost(oneof = "label_value::Value", tags = "1, 2, 3")]
    pub value: ::std::option::Option<label_value::Value>,
}
pub mod label_value {
    /// The label value can be a bool, int64, or string.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Value {
        /// A bool label value.
        #[prost(bool, tag = "1")]
        BoolValue(bool),
        /// An int64 label value.
        #[prost(int64, tag = "2")]
        Int64Value(i64),
        /// A string label value.
        #[prost(string, tag = "3")]
        StringValue(std::string::String),
    }
}
/// An error associated with a query in the time series query language format.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryError {
    /// The location of the time series query language text that this error applies
    /// to.
    #[prost(message, optional, tag = "1")]
    pub locator: ::std::option::Option<TextLocator>,
    /// The error message.
    #[prost(string, tag = "2")]
    pub message: std::string::String,
}
/// A locator for text. Indicates a particular part of the text of a request or
/// of an object referenced in the request.
///
/// For example, suppose the request field `text` contains:
///
///   text: "The quick brown fox jumps over the lazy dog."
///
/// Then the locator:
///
///   source: "text"
///   start_position {
///     line: 1
///     column: 17
///   }
///   end_position {
///     line: 1
///     column: 19
///   }
///
/// refers to the part of the text: "fox".
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TextLocator {
    /// The source of the text. The source may be a field in the request, in which
    /// case its format is the format of the
    /// google.rpc.BadRequest.FieldViolation.field field in
    /// https://cloud.google.com/apis/design/errors#error_details. It may also be
    /// be a source other than the request field (e.g. a macro definition
    /// referenced in the text of the query), in which case this is the name of
    /// the source (e.g. the macro name).
    #[prost(string, tag = "1")]
    pub source: std::string::String,
    /// The position of the first byte within the text.
    #[prost(message, optional, tag = "2")]
    pub start_position: ::std::option::Option<text_locator::Position>,
    /// The position of the last byte within the text.
    #[prost(message, optional, tag = "3")]
    pub end_position: ::std::option::Option<text_locator::Position>,
    /// If `source`, `start_position`, and `end_position` describe a call on
    /// some object (e.g. a macro in the time series query language text) and a
    /// location is to be designated in that object's text, `nested_locator`
    /// identifies the location within that object.
    #[prost(message, optional, boxed, tag = "4")]
    pub nested_locator: ::std::option::Option<::std::boxed::Box<TextLocator>>,
    /// When `nested_locator` is set, this field gives the reason for the nesting.
    /// Usually, the reason is a macro invocation. In that case, the macro name
    /// (including the leading '@') signals the location of the macro call
    /// in the text and a macro argument name (including the leading '$') signals
    /// the location of the macro argument inside the macro body that got
    /// substituted away.
    #[prost(string, tag = "5")]
    pub nesting_reason: std::string::String,
}
pub mod text_locator {
    /// The position of a byte within the text.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Position {
        /// The line, starting with 1, where the byte is positioned.
        #[prost(int32, tag = "1")]
        pub line: i32,
        /// The column within the line, starting with 1, where the byte is
        /// positioned. This is a byte index even though the text is UTF-8.
        #[prost(int32, tag = "2")]
        pub column: i32,
    }
}
/// The `ListMonitoredResourceDescriptors` request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListMonitoredResourceDescriptorsRequest {
    /// Required. The project on which to execute the request. The format is:
    ///
    ///     projects/[PROJECT_ID_OR_NUMBER]
    #[prost(string, tag = "5")]
    pub name: std::string::String,
    /// An optional [filter](https://cloud.google.com/monitoring/api/v3/filters)
    /// describing the descriptors to be returned.  The filter can reference the
    /// descriptor's type and labels. For example, the following filter returns
    /// only Google Compute Engine descriptors that have an `id` label:
    ///
    ///     resource.type = starts_with("gce_") AND resource.label:id
    #[prost(string, tag = "2")]
    pub filter: std::string::String,
    /// A positive number that is the maximum number of results to return.
    #[prost(int32, tag = "3")]
    pub page_size: i32,
    /// If this field is not empty then it must contain the `nextPageToken` value
    /// returned by a previous call to this method.  Using this field causes the
    /// method to return additional results from the previous method call.
    #[prost(string, tag = "4")]
    pub page_token: std::string::String,
}
/// The `ListMonitoredResourceDescriptors` response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListMonitoredResourceDescriptorsResponse {
    /// The monitored resource descriptors that are available to this project
    /// and that match `filter`, if present.
    #[prost(message, repeated, tag = "1")]
    pub resource_descriptors: ::std::vec::Vec<super::super::api::MonitoredResourceDescriptor>,
    /// If there are more results than have been returned, then this field is set
    /// to a non-empty value.  To see the additional results,
    /// use that value as `page_token` in the next call to this method.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
}
/// The `GetMonitoredResourceDescriptor` request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMonitoredResourceDescriptorRequest {
    /// Required. The monitored resource descriptor to get.  The format is:
    ///
    ///     projects/[PROJECT_ID_OR_NUMBER]/monitoredResourceDescriptors/[RESOURCE_TYPE]
    ///
    /// The `[RESOURCE_TYPE]` is a predefined type, such as
    /// `cloudsql_database`.
    #[prost(string, tag = "3")]
    pub name: std::string::String,
}
/// The `ListMetricDescriptors` request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListMetricDescriptorsRequest {
    /// Required. The project on which to execute the request. The format is:
    ///
    ///     projects/[PROJECT_ID_OR_NUMBER]
    #[prost(string, tag = "5")]
    pub name: std::string::String,
    /// If this field is empty, all custom and
    /// system-defined metric descriptors are returned.
    /// Otherwise, the [filter](https://cloud.google.com/monitoring/api/v3/filters)
    /// specifies which metric descriptors are to be
    /// returned. For example, the following filter matches all
    /// [custom metrics](https://cloud.google.com/monitoring/custom-metrics):
    ///
    ///     metric.type = starts_with("custom.googleapis.com/")
    #[prost(string, tag = "2")]
    pub filter: std::string::String,
    /// A positive number that is the maximum number of results to return.
    #[prost(int32, tag = "3")]
    pub page_size: i32,
    /// If this field is not empty then it must contain the `nextPageToken` value
    /// returned by a previous call to this method.  Using this field causes the
    /// method to return additional results from the previous method call.
    #[prost(string, tag = "4")]
    pub page_token: std::string::String,
}
/// The `ListMetricDescriptors` response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListMetricDescriptorsResponse {
    /// The metric descriptors that are available to the project
    /// and that match the value of `filter`, if present.
    #[prost(message, repeated, tag = "1")]
    pub metric_descriptors: ::std::vec::Vec<super::super::api::MetricDescriptor>,
    /// If there are more results than have been returned, then this field is set
    /// to a non-empty value.  To see the additional results,
    /// use that value as `page_token` in the next call to this method.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
}
/// The `GetMetricDescriptor` request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMetricDescriptorRequest {
    /// Required. The metric descriptor on which to execute the request. The format is:
    ///
    ///     projects/[PROJECT_ID_OR_NUMBER]/metricDescriptors/[METRIC_ID]
    ///
    /// An example value of `[METRIC_ID]` is
    /// `"compute.googleapis.com/instance/disk/read_bytes_count"`.
    #[prost(string, tag = "3")]
    pub name: std::string::String,
}
/// The `CreateMetricDescriptor` request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateMetricDescriptorRequest {
    /// Required. The project on which to execute the request. The format is:
    ///
    ///     projects/[PROJECT_ID_OR_NUMBER]
    #[prost(string, tag = "3")]
    pub name: std::string::String,
    /// Required. The new [custom metric](https://cloud.google.com/monitoring/custom-metrics)
    /// descriptor.
    #[prost(message, optional, tag = "2")]
    pub metric_descriptor: ::std::option::Option<super::super::api::MetricDescriptor>,
}
/// The `DeleteMetricDescriptor` request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteMetricDescriptorRequest {
    /// Required. The metric descriptor on which to execute the request. The format is:
    ///
    ///     projects/[PROJECT_ID_OR_NUMBER]/metricDescriptors/[METRIC_ID]
    ///
    /// An example of `[METRIC_ID]` is:
    /// `"custom.googleapis.com/my_test_metric"`.
    #[prost(string, tag = "3")]
    pub name: std::string::String,
}
/// The `ListTimeSeries` request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTimeSeriesRequest {
    /// Required. The project on which to execute the request. The format is:
    ///
    ///     projects/[PROJECT_ID_OR_NUMBER]
    #[prost(string, tag = "10")]
    pub name: std::string::String,
    /// Required. A [monitoring filter](https://cloud.google.com/monitoring/api/v3/filters)
    /// that specifies which time series should be returned.  The filter must
    /// specify a single metric type, and can additionally specify metric labels
    /// and other information. For example:
    ///
    ///     metric.type = "compute.googleapis.com/instance/cpu/usage_time" AND
    ///         metric.labels.instance_name = "my-instance-name"
    #[prost(string, tag = "2")]
    pub filter: std::string::String,
    /// Required. The time interval for which results should be returned. Only time series
    /// that contain data points in the specified interval are included
    /// in the response.
    #[prost(message, optional, tag = "4")]
    pub interval: ::std::option::Option<TimeInterval>,
    /// Specifies the alignment of data points in individual time series as
    /// well as how to combine the retrieved time series across specified labels.
    ///
    /// By default (if no `aggregation` is explicitly specified), the raw time
    /// series data is returned.
    #[prost(message, optional, tag = "5")]
    pub aggregation: ::std::option::Option<Aggregation>,
    /// Unsupported: must be left blank. The points in each time series are
    /// currently returned in reverse time order (most recent to oldest).
    #[prost(string, tag = "6")]
    pub order_by: std::string::String,
    /// Required. Specifies which information is returned about the time series.
    #[prost(enumeration = "list_time_series_request::TimeSeriesView", tag = "7")]
    pub view: i32,
    /// A positive number that is the maximum number of results to return. If
    /// `page_size` is empty or more than 100,000 results, the effective
    /// `page_size` is 100,000 results. If `view` is set to `FULL`, this is the
    /// maximum number of `Points` returned. If `view` is set to `HEADERS`, this is
    /// the maximum number of `TimeSeries` returned.
    #[prost(int32, tag = "8")]
    pub page_size: i32,
    /// If this field is not empty then it must contain the `nextPageToken` value
    /// returned by a previous call to this method.  Using this field causes the
    /// method to return additional results from the previous method call.
    #[prost(string, tag = "9")]
    pub page_token: std::string::String,
}
pub mod list_time_series_request {
    /// Controls which fields are returned by `ListTimeSeries`.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum TimeSeriesView {
        /// Returns the identity of the metric(s), the time series,
        /// and the time series data.
        Full = 0,
        /// Returns the identity of the metric and the time series resource,
        /// but not the time series data.
        Headers = 1,
    }
}
/// The `ListTimeSeries` response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTimeSeriesResponse {
    /// One or more time series that match the filter included in the request.
    #[prost(message, repeated, tag = "1")]
    pub time_series: ::std::vec::Vec<TimeSeries>,
    /// If there are more results than have been returned, then this field is set
    /// to a non-empty value.  To see the additional results,
    /// use that value as `page_token` in the next call to this method.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
    /// Query execution errors that may have caused the time series data returned
    /// to be incomplete.
    #[prost(message, repeated, tag = "3")]
    pub execution_errors: ::std::vec::Vec<super::super::rpc::Status>,
}
/// The `CreateTimeSeries` request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateTimeSeriesRequest {
    /// Required. The project on which to execute the request. The format is:
    ///
    ///     projects/[PROJECT_ID_OR_NUMBER]
    #[prost(string, tag = "3")]
    pub name: std::string::String,
    /// Required. The new data to be added to a list of time series.
    /// Adds at most one data point to each of several time series.  The new data
    /// point must be more recent than any other point in its time series.  Each
    /// `TimeSeries` value must fully specify a unique time series by supplying
    /// all label values for the metric and the monitored resource.
    ///
    /// The maximum number of `TimeSeries` objects per `Create` request is 200.
    #[prost(message, repeated, tag = "2")]
    pub time_series: ::std::vec::Vec<TimeSeries>,
}
/// DEPRECATED. Used to hold per-time-series error status.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateTimeSeriesError {
    /// DEPRECATED. Time series ID that resulted in the `status` error.
    #[prost(message, optional, tag = "1")]
    pub time_series: ::std::option::Option<TimeSeries>,
    /// DEPRECATED. The status of the requested write operation for `time_series`.
    #[prost(message, optional, tag = "2")]
    pub status: ::std::option::Option<super::super::rpc::Status>,
}
/// Summary of the result of a failed request to write data to a time series.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateTimeSeriesSummary {
    /// The number of points in the request.
    #[prost(int32, tag = "1")]
    pub total_point_count: i32,
    /// The number of points that were successfully written.
    #[prost(int32, tag = "2")]
    pub success_point_count: i32,
    /// The number of points that failed to be written. Order is not guaranteed.
    #[prost(message, repeated, tag = "3")]
    pub errors: ::std::vec::Vec<create_time_series_summary::Error>,
}
pub mod create_time_series_summary {
    /// Detailed information about an error category.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Error {
        /// The status of the requested write operation.
        #[prost(message, optional, tag = "1")]
        pub status: ::std::option::Option<super::super::super::rpc::Status>,
        /// The number of points that couldn't be written because of `status`.
        #[prost(int32, tag = "2")]
        pub point_count: i32,
    }
}
/// The `QueryTimeSeries` request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTimeSeriesRequest {
    /// Required. The project on which to execute the request. The format is:
    ///
    ///     projects/[PROJECT_ID_OR_NUMBER]
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Required. The query in the monitoring query language format. The default
    /// time zone is in UTC.
    #[prost(string, tag = "7")]
    pub query: std::string::String,
    /// A positive number that is the maximum number of time_series_data to return.
    #[prost(int32, tag = "9")]
    pub page_size: i32,
    /// If this field is not empty then it must contain the `nextPageToken` value
    /// returned by a previous call to this method.  Using this field causes the
    /// method to return additional results from the previous method call.
    #[prost(string, tag = "10")]
    pub page_token: std::string::String,
}
/// The `QueryTimeSeries` response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTimeSeriesResponse {
    /// The descriptor for the time series data.
    #[prost(message, optional, tag = "8")]
    pub time_series_descriptor: ::std::option::Option<TimeSeriesDescriptor>,
    /// The time series data.
    #[prost(message, repeated, tag = "9")]
    pub time_series_data: ::std::vec::Vec<TimeSeriesData>,
    /// If there are more results than have been returned, then this field is set
    /// to a non-empty value.  To see the additional results, use that value as
    /// `page_token` in the next call to this method.
    #[prost(string, tag = "10")]
    pub next_page_token: std::string::String,
    /// Query execution errors that may have caused the time series data returned
    /// to be incomplete. The available data will be available in the
    /// response.
    #[prost(message, repeated, tag = "11")]
    pub partial_errors: ::std::vec::Vec<super::super::rpc::Status>,
}
/// This is an error detail intended to be used with INVALID_ARGUMENT errors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryErrorList {
    /// Errors in parsing the time series query language text. The number of errors
    /// in the response may be limited.
    #[prost(message, repeated, tag = "1")]
    pub errors: ::std::vec::Vec<QueryError>,
    /// A summary of all the errors.
    #[prost(string, tag = "2")]
    pub error_summary: std::string::String,
}
#[doc = r" Generated client implementations."]
pub mod metric_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Manages metric descriptors, monitored resource descriptors, and"]
    #[doc = " time series data."]
    pub struct MetricServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> MetricServiceClient<T>
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
        #[doc = " Lists monitored resource descriptors that match a filter. This method does not require a Workspace."]
        pub async fn list_monitored_resource_descriptors(
            &mut self,
            request: impl tonic::IntoRequest<super::ListMonitoredResourceDescriptorsRequest>,
        ) -> Result<tonic::Response<super::ListMonitoredResourceDescriptorsResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.monitoring.v3.MetricService/ListMonitoredResourceDescriptors",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets a single monitored resource descriptor. This method does not require a Workspace."]
        pub async fn get_monitored_resource_descriptor(
            &mut self,
            request: impl tonic::IntoRequest<super::GetMonitoredResourceDescriptorRequest>,
        ) -> Result<
            tonic::Response<super::super::super::api::MonitoredResourceDescriptor>,
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
                "/google.monitoring.v3.MetricService/GetMonitoredResourceDescriptor",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists metric descriptors that match a filter. This method does not require a Workspace."]
        pub async fn list_metric_descriptors(
            &mut self,
            request: impl tonic::IntoRequest<super::ListMetricDescriptorsRequest>,
        ) -> Result<tonic::Response<super::ListMetricDescriptorsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.monitoring.v3.MetricService/ListMetricDescriptors",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets a single metric descriptor. This method does not require a Workspace."]
        pub async fn get_metric_descriptor(
            &mut self,
            request: impl tonic::IntoRequest<super::GetMetricDescriptorRequest>,
        ) -> Result<tonic::Response<super::super::super::api::MetricDescriptor>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.monitoring.v3.MetricService/GetMetricDescriptor",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a new metric descriptor."]
        #[doc = " User-created metric descriptors define"]
        #[doc = " [custom metrics](https://cloud.google.com/monitoring/custom-metrics)."]
        pub async fn create_metric_descriptor(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateMetricDescriptorRequest>,
        ) -> Result<tonic::Response<super::super::super::api::MetricDescriptor>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.monitoring.v3.MetricService/CreateMetricDescriptor",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes a metric descriptor. Only user-created"]
        #[doc = " [custom metrics](https://cloud.google.com/monitoring/custom-metrics) can be"]
        #[doc = " deleted."]
        pub async fn delete_metric_descriptor(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteMetricDescriptorRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.monitoring.v3.MetricService/DeleteMetricDescriptor",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists time series that match a filter. This method does not require a Workspace."]
        pub async fn list_time_series(
            &mut self,
            request: impl tonic::IntoRequest<super::ListTimeSeriesRequest>,
        ) -> Result<tonic::Response<super::ListTimeSeriesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.monitoring.v3.MetricService/ListTimeSeries",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates or adds data to one or more time series."]
        #[doc = " The response is empty if all time series in the request were written."]
        #[doc = " If any time series could not be written, a corresponding failure message is"]
        #[doc = " included in the error response."]
        pub async fn create_time_series(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateTimeSeriesRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.monitoring.v3.MetricService/CreateTimeSeries",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for MetricServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for MetricServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "MetricServiceClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod metric_service_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with MetricServiceServer."]
    #[async_trait]
    pub trait MetricService: Send + Sync + 'static {
        #[doc = " Lists monitored resource descriptors that match a filter. This method does not require a Workspace."]
        async fn list_monitored_resource_descriptors(
            &self,
            request: tonic::Request<super::ListMonitoredResourceDescriptorsRequest>,
        ) -> Result<tonic::Response<super::ListMonitoredResourceDescriptorsResponse>, tonic::Status>;
        #[doc = " Gets a single monitored resource descriptor. This method does not require a Workspace."]
        async fn get_monitored_resource_descriptor(
            &self,
            request: tonic::Request<super::GetMonitoredResourceDescriptorRequest>,
        ) -> Result<
            tonic::Response<super::super::super::api::MonitoredResourceDescriptor>,
            tonic::Status,
        >;
        #[doc = " Lists metric descriptors that match a filter. This method does not require a Workspace."]
        async fn list_metric_descriptors(
            &self,
            request: tonic::Request<super::ListMetricDescriptorsRequest>,
        ) -> Result<tonic::Response<super::ListMetricDescriptorsResponse>, tonic::Status>;
        #[doc = " Gets a single metric descriptor. This method does not require a Workspace."]
        async fn get_metric_descriptor(
            &self,
            request: tonic::Request<super::GetMetricDescriptorRequest>,
        ) -> Result<tonic::Response<super::super::super::api::MetricDescriptor>, tonic::Status>;
        #[doc = " Creates a new metric descriptor."]
        #[doc = " User-created metric descriptors define"]
        #[doc = " [custom metrics](https://cloud.google.com/monitoring/custom-metrics)."]
        async fn create_metric_descriptor(
            &self,
            request: tonic::Request<super::CreateMetricDescriptorRequest>,
        ) -> Result<tonic::Response<super::super::super::api::MetricDescriptor>, tonic::Status>;
        #[doc = " Deletes a metric descriptor. Only user-created"]
        #[doc = " [custom metrics](https://cloud.google.com/monitoring/custom-metrics) can be"]
        #[doc = " deleted."]
        async fn delete_metric_descriptor(
            &self,
            request: tonic::Request<super::DeleteMetricDescriptorRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status>;
        #[doc = " Lists time series that match a filter. This method does not require a Workspace."]
        async fn list_time_series(
            &self,
            request: tonic::Request<super::ListTimeSeriesRequest>,
        ) -> Result<tonic::Response<super::ListTimeSeriesResponse>, tonic::Status>;
        #[doc = " Creates or adds data to one or more time series."]
        #[doc = " The response is empty if all time series in the request were written."]
        #[doc = " If any time series could not be written, a corresponding failure message is"]
        #[doc = " included in the error response."]
        async fn create_time_series(
            &self,
            request: tonic::Request<super::CreateTimeSeriesRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status>;
    }
    #[doc = " Manages metric descriptors, monitored resource descriptors, and"]
    #[doc = " time series data."]
    #[derive(Debug)]
    #[doc(hidden)]
    pub struct MetricServiceServer<T: MetricService> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: MetricService> MetricServiceServer<T> {
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
    impl<T, B> Service<http::Request<B>> for MetricServiceServer<T>
    where
        T: MetricService,
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
                "/google.monitoring.v3.MetricService/ListMonitoredResourceDescriptors" => {
                    #[allow(non_camel_case_types)]
                    struct ListMonitoredResourceDescriptorsSvc<T: MetricService>(pub Arc<T>);
                    impl<T: MetricService>
                        tonic::server::UnaryService<super::ListMonitoredResourceDescriptorsRequest>
                        for ListMonitoredResourceDescriptorsSvc<T>
                    {
                        type Response = super::ListMonitoredResourceDescriptorsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListMonitoredResourceDescriptorsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                inner.list_monitored_resource_descriptors(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ListMonitoredResourceDescriptorsSvc(inner);
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
                "/google.monitoring.v3.MetricService/GetMonitoredResourceDescriptor" => {
                    #[allow(non_camel_case_types)]
                    struct GetMonitoredResourceDescriptorSvc<T: MetricService>(pub Arc<T>);
                    impl<T: MetricService>
                        tonic::server::UnaryService<super::GetMonitoredResourceDescriptorRequest>
                        for GetMonitoredResourceDescriptorSvc<T>
                    {
                        type Response = super::super::super::api::MonitoredResourceDescriptor;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetMonitoredResourceDescriptorRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                inner.get_monitored_resource_descriptor(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetMonitoredResourceDescriptorSvc(inner);
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
                "/google.monitoring.v3.MetricService/ListMetricDescriptors" => {
                    #[allow(non_camel_case_types)]
                    struct ListMetricDescriptorsSvc<T: MetricService>(pub Arc<T>);
                    impl<T: MetricService>
                        tonic::server::UnaryService<super::ListMetricDescriptorsRequest>
                        for ListMetricDescriptorsSvc<T>
                    {
                        type Response = super::ListMetricDescriptorsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListMetricDescriptorsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.list_metric_descriptors(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ListMetricDescriptorsSvc(inner);
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
                "/google.monitoring.v3.MetricService/GetMetricDescriptor" => {
                    #[allow(non_camel_case_types)]
                    struct GetMetricDescriptorSvc<T: MetricService>(pub Arc<T>);
                    impl<T: MetricService>
                        tonic::server::UnaryService<super::GetMetricDescriptorRequest>
                        for GetMetricDescriptorSvc<T>
                    {
                        type Response = super::super::super::api::MetricDescriptor;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetMetricDescriptorRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.get_metric_descriptor(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetMetricDescriptorSvc(inner);
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
                "/google.monitoring.v3.MetricService/CreateMetricDescriptor" => {
                    #[allow(non_camel_case_types)]
                    struct CreateMetricDescriptorSvc<T: MetricService>(pub Arc<T>);
                    impl<T: MetricService>
                        tonic::server::UnaryService<super::CreateMetricDescriptorRequest>
                        for CreateMetricDescriptorSvc<T>
                    {
                        type Response = super::super::super::api::MetricDescriptor;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateMetricDescriptorRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.create_metric_descriptor(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = CreateMetricDescriptorSvc(inner);
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
                "/google.monitoring.v3.MetricService/DeleteMetricDescriptor" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteMetricDescriptorSvc<T: MetricService>(pub Arc<T>);
                    impl<T: MetricService>
                        tonic::server::UnaryService<super::DeleteMetricDescriptorRequest>
                        for DeleteMetricDescriptorSvc<T>
                    {
                        type Response = ();
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteMetricDescriptorRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.delete_metric_descriptor(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = DeleteMetricDescriptorSvc(inner);
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
                "/google.monitoring.v3.MetricService/ListTimeSeries" => {
                    #[allow(non_camel_case_types)]
                    struct ListTimeSeriesSvc<T: MetricService>(pub Arc<T>);
                    impl<T: MetricService> tonic::server::UnaryService<super::ListTimeSeriesRequest>
                        for ListTimeSeriesSvc<T>
                    {
                        type Response = super::ListTimeSeriesResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListTimeSeriesRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.list_time_series(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ListTimeSeriesSvc(inner);
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
                "/google.monitoring.v3.MetricService/CreateTimeSeries" => {
                    #[allow(non_camel_case_types)]
                    struct CreateTimeSeriesSvc<T: MetricService>(pub Arc<T>);
                    impl<T: MetricService>
                        tonic::server::UnaryService<super::CreateTimeSeriesRequest>
                        for CreateTimeSeriesSvc<T>
                    {
                        type Response = ();
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateTimeSeriesRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.create_time_series(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = CreateTimeSeriesSvc(inner);
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
    impl<T: MetricService> Clone for MetricServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: MetricService> Clone for _Inner<T> {
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
/// A description of a notification channel. The descriptor includes
/// the properties of the channel and the set of labels or fields that
/// must be specified to configure channels of a given type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NotificationChannelDescriptor {
    /// The full REST resource name for this descriptor. The format is:
    ///
    ///     projects/[PROJECT_ID_OR_NUMBER]/notificationChannelDescriptors/[TYPE]
    ///
    /// In the above, `[TYPE]` is the value of the `type` field.
    #[prost(string, tag = "6")]
    pub name: std::string::String,
    /// The type of notification channel, such as "email", "sms", etc.
    /// Notification channel types are globally unique.
    #[prost(string, tag = "1")]
    pub r#type: std::string::String,
    /// A human-readable name for the notification channel type.  This
    /// form of the name is suitable for a user interface.
    #[prost(string, tag = "2")]
    pub display_name: std::string::String,
    /// A human-readable description of the notification channel
    /// type. The description may include a description of the properties
    /// of the channel and pointers to external documentation.
    #[prost(string, tag = "3")]
    pub description: std::string::String,
    /// The set of labels that must be defined to identify a particular
    /// channel of the corresponding type. Each label includes a
    /// description for how that field should be populated.
    #[prost(message, repeated, tag = "4")]
    pub labels: ::std::vec::Vec<super::super::api::LabelDescriptor>,
    /// The tiers that support this notification channel; the project service tier
    /// must be one of the supported_tiers.
    #[prost(enumeration = "ServiceTier", repeated, packed = "false", tag = "5")]
    pub supported_tiers: ::std::vec::Vec<i32>,
    /// The product launch stage for channels of this type.
    #[prost(enumeration = "super::super::api::LaunchStage", tag = "7")]
    pub launch_stage: i32,
}
/// A `NotificationChannel` is a medium through which an alert is
/// delivered when a policy violation is detected. Examples of channels
/// include email, SMS, and third-party messaging applications. Fields
/// containing sensitive information like authentication tokens or
/// contact info are only partially populated on retrieval.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NotificationChannel {
    /// The type of the notification channel. This field matches the
    /// value of the [NotificationChannelDescriptor.type][google.monitoring.v3.NotificationChannelDescriptor.type] field.
    #[prost(string, tag = "1")]
    pub r#type: std::string::String,
    /// The full REST resource name for this channel. The format is:
    ///
    ///     projects/[PROJECT_ID_OR_NUMBER]/notificationChannels/[CHANNEL_ID]
    ///
    /// The `[CHANNEL_ID]` is automatically assigned by the server on creation.
    #[prost(string, tag = "6")]
    pub name: std::string::String,
    /// An optional human-readable name for this notification channel. It is
    /// recommended that you specify a non-empty and unique name in order to
    /// make it easier to identify the channels in your project, though this is
    /// not enforced. The display name is limited to 512 Unicode characters.
    #[prost(string, tag = "3")]
    pub display_name: std::string::String,
    /// An optional human-readable description of this notification channel. This
    /// description may provide additional details, beyond the display
    /// name, for the channel. This may not exceed 1024 Unicode characters.
    #[prost(string, tag = "4")]
    pub description: std::string::String,
    /// Configuration fields that define the channel and its behavior. The
    /// permissible and required labels are specified in the
    /// [NotificationChannelDescriptor.labels][google.monitoring.v3.NotificationChannelDescriptor.labels] of the
    /// `NotificationChannelDescriptor` corresponding to the `type` field.
    #[prost(map = "string, string", tag = "5")]
    pub labels: ::std::collections::HashMap<std::string::String, std::string::String>,
    /// User-supplied key/value data that does not need to conform to
    /// the corresponding `NotificationChannelDescriptor`'s schema, unlike
    /// the `labels` field. This field is intended to be used for organizing
    /// and identifying the `NotificationChannel` objects.
    ///
    /// The field can contain up to 64 entries. Each key and value is limited to
    /// 63 Unicode characters or 128 bytes, whichever is smaller. Labels and
    /// values can contain only lowercase letters, numerals, underscores, and
    /// dashes. Keys must begin with a letter.
    #[prost(map = "string, string", tag = "8")]
    pub user_labels: ::std::collections::HashMap<std::string::String, std::string::String>,
    /// Indicates whether this channel has been verified or not. On a
    /// [`ListNotificationChannels`][google.monitoring.v3.NotificationChannelService.ListNotificationChannels]
    /// or
    /// [`GetNotificationChannel`][google.monitoring.v3.NotificationChannelService.GetNotificationChannel]
    /// operation, this field is expected to be populated.
    ///
    /// If the value is `UNVERIFIED`, then it indicates that the channel is
    /// non-functioning (it both requires verification and lacks verification);
    /// otherwise, it is assumed that the channel works.
    ///
    /// If the channel is neither `VERIFIED` nor `UNVERIFIED`, it implies that
    /// the channel is of a type that does not require verification or that
    /// this specific channel has been exempted from verification because it was
    /// created prior to verification being required for channels of this type.
    ///
    /// This field cannot be modified using a standard
    /// [`UpdateNotificationChannel`][google.monitoring.v3.NotificationChannelService.UpdateNotificationChannel]
    /// operation. To change the value of this field, you must call
    /// [`VerifyNotificationChannel`][google.monitoring.v3.NotificationChannelService.VerifyNotificationChannel].
    #[prost(enumeration = "notification_channel::VerificationStatus", tag = "9")]
    pub verification_status: i32,
    /// Whether notifications are forwarded to the described channel. This makes
    /// it possible to disable delivery of notifications to a particular channel
    /// without removing the channel from all alerting policies that reference
    /// the channel. This is a more convenient approach when the change is
    /// temporary and you want to receive notifications from the same set
    /// of alerting policies on the channel at some point in the future.
    #[prost(message, optional, tag = "11")]
    pub enabled: ::std::option::Option<bool>,
}
pub mod notification_channel {
    /// Indicates whether the channel has been verified or not. It is illegal
    /// to specify this field in a
    /// [`CreateNotificationChannel`][google.monitoring.v3.NotificationChannelService.CreateNotificationChannel]
    /// or an
    /// [`UpdateNotificationChannel`][google.monitoring.v3.NotificationChannelService.UpdateNotificationChannel]
    /// operation.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum VerificationStatus {
        /// Sentinel value used to indicate that the state is unknown, omitted, or
        /// is not applicable (as in the case of channels that neither support
        /// nor require verification in order to function).
        Unspecified = 0,
        /// The channel has yet to be verified and requires verification to function.
        /// Note that this state also applies to the case where the verification
        /// process has been initiated by sending a verification code but where
        /// the verification code has not been submitted to complete the process.
        Unverified = 1,
        /// It has been proven that notifications can be received on this
        /// notification channel and that someone on the project has access
        /// to messages that are delivered to that channel.
        Verified = 2,
    }
}
/// The `ListNotificationChannelDescriptors` request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListNotificationChannelDescriptorsRequest {
    /// Required. The REST resource name of the parent from which to retrieve
    /// the notification channel descriptors. The expected syntax is:
    ///
    ///     projects/[PROJECT_ID_OR_NUMBER]
    ///
    /// Note that this names the parent container in which to look for the
    /// descriptors; to retrieve a single descriptor by name, use the
    /// [GetNotificationChannelDescriptor][google.monitoring.v3.NotificationChannelService.GetNotificationChannelDescriptor]
    /// operation, instead.
    #[prost(string, tag = "4")]
    pub name: std::string::String,
    /// The maximum number of results to return in a single response. If
    /// not set to a positive number, a reasonable value will be chosen by the
    /// service.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// If non-empty, `page_token` must contain a value returned as the
    /// `next_page_token` in a previous response to request the next set
    /// of results.
    #[prost(string, tag = "3")]
    pub page_token: std::string::String,
}
/// The `ListNotificationChannelDescriptors` response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListNotificationChannelDescriptorsResponse {
    /// The monitored resource descriptors supported for the specified
    /// project, optionally filtered.
    #[prost(message, repeated, tag = "1")]
    pub channel_descriptors: ::std::vec::Vec<NotificationChannelDescriptor>,
    /// If not empty, indicates that there may be more results that match
    /// the request. Use the value in the `page_token` field in a
    /// subsequent request to fetch the next set of results. If empty,
    /// all results have been returned.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
}
/// The `GetNotificationChannelDescriptor` response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetNotificationChannelDescriptorRequest {
    /// Required. The channel type for which to execute the request. The format is:
    ///
    ///     projects/[PROJECT_ID_OR_NUMBER]/notificationChannelDescriptors/[CHANNEL_TYPE]
    #[prost(string, tag = "3")]
    pub name: std::string::String,
}
/// The `CreateNotificationChannel` request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateNotificationChannelRequest {
    /// Required. The project on which to execute the request. The format is:
    ///
    ///     projects/[PROJECT_ID_OR_NUMBER]
    ///
    /// This names the container into which the channel will be
    /// written, this does not name the newly created channel. The resulting
    /// channel's name will have a normalized version of this field as a prefix,
    /// but will add `/notificationChannels/[CHANNEL_ID]` to identify the channel.
    #[prost(string, tag = "3")]
    pub name: std::string::String,
    /// Required. The definition of the `NotificationChannel` to create.
    #[prost(message, optional, tag = "2")]
    pub notification_channel: ::std::option::Option<NotificationChannel>,
}
/// The `ListNotificationChannels` request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListNotificationChannelsRequest {
    /// Required. The project on which to execute the request. The format is:
    ///
    ///     projects/[PROJECT_ID_OR_NUMBER]
    ///
    /// This names the container
    /// in which to look for the notification channels; it does not name a
    /// specific channel. To query a specific channel by REST resource name, use
    /// the
    /// [`GetNotificationChannel`][google.monitoring.v3.NotificationChannelService.GetNotificationChannel]
    /// operation.
    #[prost(string, tag = "5")]
    pub name: std::string::String,
    /// If provided, this field specifies the criteria that must be met by
    /// notification channels to be included in the response.
    ///
    /// For more details, see [sorting and
    /// filtering](https://cloud.google.com/monitoring/api/v3/sorting-and-filtering).
    #[prost(string, tag = "6")]
    pub filter: std::string::String,
    /// A comma-separated list of fields by which to sort the result. Supports
    /// the same set of fields as in `filter`. Entries can be prefixed with
    /// a minus sign to sort in descending rather than ascending order.
    ///
    /// For more details, see [sorting and
    /// filtering](https://cloud.google.com/monitoring/api/v3/sorting-and-filtering).
    #[prost(string, tag = "7")]
    pub order_by: std::string::String,
    /// The maximum number of results to return in a single response. If
    /// not set to a positive number, a reasonable value will be chosen by the
    /// service.
    #[prost(int32, tag = "3")]
    pub page_size: i32,
    /// If non-empty, `page_token` must contain a value returned as the
    /// `next_page_token` in a previous response to request the next set
    /// of results.
    #[prost(string, tag = "4")]
    pub page_token: std::string::String,
}
/// The `ListNotificationChannels` response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListNotificationChannelsResponse {
    /// The notification channels defined for the specified project.
    #[prost(message, repeated, tag = "3")]
    pub notification_channels: ::std::vec::Vec<NotificationChannel>,
    /// If not empty, indicates that there may be more results that match
    /// the request. Use the value in the `page_token` field in a
    /// subsequent request to fetch the next set of results. If empty,
    /// all results have been returned.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
}
/// The `GetNotificationChannel` request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetNotificationChannelRequest {
    /// Required. The channel for which to execute the request. The format is:
    ///
    ///     projects/[PROJECT_ID_OR_NUMBER]/notificationChannels/[CHANNEL_ID]
    #[prost(string, tag = "3")]
    pub name: std::string::String,
}
/// The `UpdateNotificationChannel` request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateNotificationChannelRequest {
    /// The fields to update.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
    /// Required. A description of the changes to be applied to the specified
    /// notification channel. The description must provide a definition for
    /// fields to be updated; the names of these fields should also be
    /// included in the `update_mask`.
    #[prost(message, optional, tag = "3")]
    pub notification_channel: ::std::option::Option<NotificationChannel>,
}
/// The `DeleteNotificationChannel` request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteNotificationChannelRequest {
    /// Required. The channel for which to execute the request. The format is:
    ///
    ///     projects/[PROJECT_ID_OR_NUMBER]/notificationChannels/[CHANNEL_ID]
    #[prost(string, tag = "3")]
    pub name: std::string::String,
    /// If true, the notification channel will be deleted regardless of its
    /// use in alert policies (the policies will be updated to remove the
    /// channel). If false, channels that are still referenced by an existing
    /// alerting policy will fail to be deleted in a delete operation.
    #[prost(bool, tag = "5")]
    pub force: bool,
}
/// The `SendNotificationChannelVerificationCode` request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendNotificationChannelVerificationCodeRequest {
    /// Required. The notification channel to which to send a verification code.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// The `GetNotificationChannelVerificationCode` request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetNotificationChannelVerificationCodeRequest {
    /// Required. The notification channel for which a verification code is to be generated
    /// and retrieved. This must name a channel that is already verified; if
    /// the specified channel is not verified, the request will fail.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// The desired expiration time. If specified, the API will guarantee that
    /// the returned code will not be valid after the specified timestamp;
    /// however, the API cannot guarantee that the returned code will be
    /// valid for at least as long as the requested time (the API puts an upper
    /// bound on the amount of time for which a code may be valid). If omitted,
    /// a default expiration will be used, which may be less than the max
    /// permissible expiration (so specifying an expiration may extend the
    /// code's lifetime over omitting an expiration, even though the API does
    /// impose an upper limit on the maximum expiration that is permitted).
    #[prost(message, optional, tag = "2")]
    pub expire_time: ::std::option::Option<::prost_types::Timestamp>,
}
/// The `GetNotificationChannelVerificationCode` request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetNotificationChannelVerificationCodeResponse {
    /// The verification code, which may be used to verify other channels
    /// that have an equivalent identity (i.e. other channels of the same
    /// type with the same fingerprint such as other email channels with
    /// the same email address or other sms channels with the same number).
    #[prost(string, tag = "1")]
    pub code: std::string::String,
    /// The expiration time associated with the code that was returned. If
    /// an expiration was provided in the request, this is the minimum of the
    /// requested expiration in the request and the max permitted expiration.
    #[prost(message, optional, tag = "2")]
    pub expire_time: ::std::option::Option<::prost_types::Timestamp>,
}
/// The `VerifyNotificationChannel` request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VerifyNotificationChannelRequest {
    /// Required. The notification channel to verify.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Required. The verification code that was delivered to the channel as
    /// a result of invoking the `SendNotificationChannelVerificationCode` API
    /// method or that was retrieved from a verified channel via
    /// `GetNotificationChannelVerificationCode`. For example, one might have
    /// "G-123456" or "TKNZGhhd2EyN3I1MnRnMjRv" (in general, one is only
    /// guaranteed that the code is valid UTF-8; one should not
    /// make any assumptions regarding the structure or format of the code).
    #[prost(string, tag = "2")]
    pub code: std::string::String,
}
#[doc = r" Generated client implementations."]
pub mod notification_channel_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " The Notification Channel API provides access to configuration that"]
    #[doc = " controls how messages related to incidents are sent."]
    pub struct NotificationChannelServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> NotificationChannelServiceClient<T>
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
        #[doc = " Lists the descriptors for supported channel types. The use of descriptors"]
        #[doc = " makes it possible for new channel types to be dynamically added."]
        pub async fn list_notification_channel_descriptors(
            &mut self,
            request: impl tonic::IntoRequest<super::ListNotificationChannelDescriptorsRequest>,
        ) -> Result<tonic::Response<super::ListNotificationChannelDescriptorsResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ( "/google.monitoring.v3.NotificationChannelService/ListNotificationChannelDescriptors" ) ;
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets a single channel descriptor. The descriptor indicates which fields"]
        #[doc = " are expected / permitted for a notification channel of the given type."]
        pub async fn get_notification_channel_descriptor(
            &mut self,
            request: impl tonic::IntoRequest<super::GetNotificationChannelDescriptorRequest>,
        ) -> Result<tonic::Response<super::NotificationChannelDescriptor>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.monitoring.v3.NotificationChannelService/GetNotificationChannelDescriptor",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists the notification channels that have been created for the project."]
        pub async fn list_notification_channels(
            &mut self,
            request: impl tonic::IntoRequest<super::ListNotificationChannelsRequest>,
        ) -> Result<tonic::Response<super::ListNotificationChannelsResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.monitoring.v3.NotificationChannelService/ListNotificationChannels",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets a single notification channel. The channel includes the relevant"]
        #[doc = " configuration details with which the channel was created. However, the"]
        #[doc = " response may truncate or omit passwords, API keys, or other private key"]
        #[doc = " matter and thus the response may not be 100% identical to the information"]
        #[doc = " that was supplied in the call to the create method."]
        pub async fn get_notification_channel(
            &mut self,
            request: impl tonic::IntoRequest<super::GetNotificationChannelRequest>,
        ) -> Result<tonic::Response<super::NotificationChannel>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.monitoring.v3.NotificationChannelService/GetNotificationChannel",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a new notification channel, representing a single notification"]
        #[doc = " endpoint such as an email address, SMS number, or PagerDuty service."]
        pub async fn create_notification_channel(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateNotificationChannelRequest>,
        ) -> Result<tonic::Response<super::NotificationChannel>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.monitoring.v3.NotificationChannelService/CreateNotificationChannel",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates a notification channel. Fields not specified in the field mask"]
        #[doc = " remain unchanged."]
        pub async fn update_notification_channel(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateNotificationChannelRequest>,
        ) -> Result<tonic::Response<super::NotificationChannel>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.monitoring.v3.NotificationChannelService/UpdateNotificationChannel",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes a notification channel."]
        pub async fn delete_notification_channel(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteNotificationChannelRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.monitoring.v3.NotificationChannelService/DeleteNotificationChannel",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Causes a verification code to be delivered to the channel. The code"]
        #[doc = " can then be supplied in `VerifyNotificationChannel` to verify the channel."]
        pub async fn send_notification_channel_verification_code(
            &mut self,
            request: impl tonic::IntoRequest<super::SendNotificationChannelVerificationCodeRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ( "/google.monitoring.v3.NotificationChannelService/SendNotificationChannelVerificationCode" ) ;
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Requests a verification code for an already verified channel that can then"]
        #[doc = " be used in a call to VerifyNotificationChannel() on a different channel"]
        #[doc = " with an equivalent identity in the same or in a different project. This"]
        #[doc = " makes it possible to copy a channel between projects without requiring"]
        #[doc = " manual reverification of the channel. If the channel is not in the"]
        #[doc = " verified state, this method will fail (in other words, this may only be"]
        #[doc = " used if the SendNotificationChannelVerificationCode and"]
        #[doc = " VerifyNotificationChannel paths have already been used to put the given"]
        #[doc = " channel into the verified state)."]
        #[doc = ""]
        #[doc = " There is no guarantee that the verification codes returned by this method"]
        #[doc = " will be of a similar structure or form as the ones that are delivered"]
        #[doc = " to the channel via SendNotificationChannelVerificationCode; while"]
        #[doc = " VerifyNotificationChannel() will recognize both the codes delivered via"]
        #[doc = " SendNotificationChannelVerificationCode() and returned from"]
        #[doc = " GetNotificationChannelVerificationCode(), it is typically the case that"]
        #[doc = " the verification codes delivered via"]
        #[doc = " SendNotificationChannelVerificationCode() will be shorter and also"]
        #[doc = " have a shorter expiration (e.g. codes such as \"G-123456\") whereas"]
        #[doc = " GetVerificationCode() will typically return a much longer, websafe base"]
        #[doc = " 64 encoded string that has a longer expiration time."]
        pub async fn get_notification_channel_verification_code(
            &mut self,
            request: impl tonic::IntoRequest<super::GetNotificationChannelVerificationCodeRequest>,
        ) -> Result<
            tonic::Response<super::GetNotificationChannelVerificationCodeResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ( "/google.monitoring.v3.NotificationChannelService/GetNotificationChannelVerificationCode" ) ;
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Verifies a `NotificationChannel` by proving receipt of the code"]
        #[doc = " delivered to the channel as a result of calling"]
        #[doc = " `SendNotificationChannelVerificationCode`."]
        pub async fn verify_notification_channel(
            &mut self,
            request: impl tonic::IntoRequest<super::VerifyNotificationChannelRequest>,
        ) -> Result<tonic::Response<super::NotificationChannel>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.monitoring.v3.NotificationChannelService/VerifyNotificationChannel",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for NotificationChannelServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for NotificationChannelServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "NotificationChannelServiceClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod notification_channel_service_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with NotificationChannelServiceServer."]
    #[async_trait]
    pub trait NotificationChannelService: Send + Sync + 'static {
        #[doc = " Lists the descriptors for supported channel types. The use of descriptors"]
        #[doc = " makes it possible for new channel types to be dynamically added."]
        async fn list_notification_channel_descriptors(
            &self,
            request: tonic::Request<super::ListNotificationChannelDescriptorsRequest>,
        ) -> Result<tonic::Response<super::ListNotificationChannelDescriptorsResponse>, tonic::Status>;
        #[doc = " Gets a single channel descriptor. The descriptor indicates which fields"]
        #[doc = " are expected / permitted for a notification channel of the given type."]
        async fn get_notification_channel_descriptor(
            &self,
            request: tonic::Request<super::GetNotificationChannelDescriptorRequest>,
        ) -> Result<tonic::Response<super::NotificationChannelDescriptor>, tonic::Status>;
        #[doc = " Lists the notification channels that have been created for the project."]
        async fn list_notification_channels(
            &self,
            request: tonic::Request<super::ListNotificationChannelsRequest>,
        ) -> Result<tonic::Response<super::ListNotificationChannelsResponse>, tonic::Status>;
        #[doc = " Gets a single notification channel. The channel includes the relevant"]
        #[doc = " configuration details with which the channel was created. However, the"]
        #[doc = " response may truncate or omit passwords, API keys, or other private key"]
        #[doc = " matter and thus the response may not be 100% identical to the information"]
        #[doc = " that was supplied in the call to the create method."]
        async fn get_notification_channel(
            &self,
            request: tonic::Request<super::GetNotificationChannelRequest>,
        ) -> Result<tonic::Response<super::NotificationChannel>, tonic::Status>;
        #[doc = " Creates a new notification channel, representing a single notification"]
        #[doc = " endpoint such as an email address, SMS number, or PagerDuty service."]
        async fn create_notification_channel(
            &self,
            request: tonic::Request<super::CreateNotificationChannelRequest>,
        ) -> Result<tonic::Response<super::NotificationChannel>, tonic::Status>;
        #[doc = " Updates a notification channel. Fields not specified in the field mask"]
        #[doc = " remain unchanged."]
        async fn update_notification_channel(
            &self,
            request: tonic::Request<super::UpdateNotificationChannelRequest>,
        ) -> Result<tonic::Response<super::NotificationChannel>, tonic::Status>;
        #[doc = " Deletes a notification channel."]
        async fn delete_notification_channel(
            &self,
            request: tonic::Request<super::DeleteNotificationChannelRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status>;
        #[doc = " Causes a verification code to be delivered to the channel. The code"]
        #[doc = " can then be supplied in `VerifyNotificationChannel` to verify the channel."]
        async fn send_notification_channel_verification_code(
            &self,
            request: tonic::Request<super::SendNotificationChannelVerificationCodeRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status>;
        #[doc = " Requests a verification code for an already verified channel that can then"]
        #[doc = " be used in a call to VerifyNotificationChannel() on a different channel"]
        #[doc = " with an equivalent identity in the same or in a different project. This"]
        #[doc = " makes it possible to copy a channel between projects without requiring"]
        #[doc = " manual reverification of the channel. If the channel is not in the"]
        #[doc = " verified state, this method will fail (in other words, this may only be"]
        #[doc = " used if the SendNotificationChannelVerificationCode and"]
        #[doc = " VerifyNotificationChannel paths have already been used to put the given"]
        #[doc = " channel into the verified state)."]
        #[doc = ""]
        #[doc = " There is no guarantee that the verification codes returned by this method"]
        #[doc = " will be of a similar structure or form as the ones that are delivered"]
        #[doc = " to the channel via SendNotificationChannelVerificationCode; while"]
        #[doc = " VerifyNotificationChannel() will recognize both the codes delivered via"]
        #[doc = " SendNotificationChannelVerificationCode() and returned from"]
        #[doc = " GetNotificationChannelVerificationCode(), it is typically the case that"]
        #[doc = " the verification codes delivered via"]
        #[doc = " SendNotificationChannelVerificationCode() will be shorter and also"]
        #[doc = " have a shorter expiration (e.g. codes such as \"G-123456\") whereas"]
        #[doc = " GetVerificationCode() will typically return a much longer, websafe base"]
        #[doc = " 64 encoded string that has a longer expiration time."]
        async fn get_notification_channel_verification_code(
            &self,
            request: tonic::Request<super::GetNotificationChannelVerificationCodeRequest>,
        ) -> Result<
            tonic::Response<super::GetNotificationChannelVerificationCodeResponse>,
            tonic::Status,
        >;
        #[doc = " Verifies a `NotificationChannel` by proving receipt of the code"]
        #[doc = " delivered to the channel as a result of calling"]
        #[doc = " `SendNotificationChannelVerificationCode`."]
        async fn verify_notification_channel(
            &self,
            request: tonic::Request<super::VerifyNotificationChannelRequest>,
        ) -> Result<tonic::Response<super::NotificationChannel>, tonic::Status>;
    }
    #[doc = " The Notification Channel API provides access to configuration that"]
    #[doc = " controls how messages related to incidents are sent."]
    #[derive(Debug)]
    #[doc(hidden)]
    pub struct NotificationChannelServiceServer<T: NotificationChannelService> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: NotificationChannelService> NotificationChannelServiceServer<T> {
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
    impl<T, B> Service<http::Request<B>> for NotificationChannelServiceServer<T>
    where
        T: NotificationChannelService,
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
            match req . uri ( ) . path ( ) { "/google.monitoring.v3.NotificationChannelService/ListNotificationChannelDescriptors" => { # [ allow ( non_camel_case_types ) ] struct ListNotificationChannelDescriptorsSvc < T : NotificationChannelService > ( pub Arc < T > ) ; impl < T : NotificationChannelService > tonic :: server :: UnaryService < super :: ListNotificationChannelDescriptorsRequest > for ListNotificationChannelDescriptorsSvc < T > { type Response = super :: ListNotificationChannelDescriptorsResponse ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: ListNotificationChannelDescriptorsRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { inner . list_notification_channel_descriptors ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = ListNotificationChannelDescriptorsSvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } "/google.monitoring.v3.NotificationChannelService/GetNotificationChannelDescriptor" => { # [ allow ( non_camel_case_types ) ] struct GetNotificationChannelDescriptorSvc < T : NotificationChannelService > ( pub Arc < T > ) ; impl < T : NotificationChannelService > tonic :: server :: UnaryService < super :: GetNotificationChannelDescriptorRequest > for GetNotificationChannelDescriptorSvc < T > { type Response = super :: NotificationChannelDescriptor ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: GetNotificationChannelDescriptorRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { inner . get_notification_channel_descriptor ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = GetNotificationChannelDescriptorSvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } "/google.monitoring.v3.NotificationChannelService/ListNotificationChannels" => { # [ allow ( non_camel_case_types ) ] struct ListNotificationChannelsSvc < T : NotificationChannelService > ( pub Arc < T > ) ; impl < T : NotificationChannelService > tonic :: server :: UnaryService < super :: ListNotificationChannelsRequest > for ListNotificationChannelsSvc < T > { type Response = super :: ListNotificationChannelsResponse ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: ListNotificationChannelsRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { inner . list_notification_channels ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = ListNotificationChannelsSvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } "/google.monitoring.v3.NotificationChannelService/GetNotificationChannel" => { # [ allow ( non_camel_case_types ) ] struct GetNotificationChannelSvc < T : NotificationChannelService > ( pub Arc < T > ) ; impl < T : NotificationChannelService > tonic :: server :: UnaryService < super :: GetNotificationChannelRequest > for GetNotificationChannelSvc < T > { type Response = super :: NotificationChannel ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: GetNotificationChannelRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { inner . get_notification_channel ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = GetNotificationChannelSvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } "/google.monitoring.v3.NotificationChannelService/CreateNotificationChannel" => { # [ allow ( non_camel_case_types ) ] struct CreateNotificationChannelSvc < T : NotificationChannelService > ( pub Arc < T > ) ; impl < T : NotificationChannelService > tonic :: server :: UnaryService < super :: CreateNotificationChannelRequest > for CreateNotificationChannelSvc < T > { type Response = super :: NotificationChannel ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: CreateNotificationChannelRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { inner . create_notification_channel ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = CreateNotificationChannelSvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } "/google.monitoring.v3.NotificationChannelService/UpdateNotificationChannel" => { # [ allow ( non_camel_case_types ) ] struct UpdateNotificationChannelSvc < T : NotificationChannelService > ( pub Arc < T > ) ; impl < T : NotificationChannelService > tonic :: server :: UnaryService < super :: UpdateNotificationChannelRequest > for UpdateNotificationChannelSvc < T > { type Response = super :: NotificationChannel ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: UpdateNotificationChannelRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { inner . update_notification_channel ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = UpdateNotificationChannelSvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } "/google.monitoring.v3.NotificationChannelService/DeleteNotificationChannel" => { # [ allow ( non_camel_case_types ) ] struct DeleteNotificationChannelSvc < T : NotificationChannelService > ( pub Arc < T > ) ; impl < T : NotificationChannelService > tonic :: server :: UnaryService < super :: DeleteNotificationChannelRequest > for DeleteNotificationChannelSvc < T > { type Response = ( ) ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: DeleteNotificationChannelRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { inner . delete_notification_channel ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = DeleteNotificationChannelSvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } "/google.monitoring.v3.NotificationChannelService/SendNotificationChannelVerificationCode" => { # [ allow ( non_camel_case_types ) ] struct SendNotificationChannelVerificationCodeSvc < T : NotificationChannelService > ( pub Arc < T > ) ; impl < T : NotificationChannelService > tonic :: server :: UnaryService < super :: SendNotificationChannelVerificationCodeRequest > for SendNotificationChannelVerificationCodeSvc < T > { type Response = ( ) ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: SendNotificationChannelVerificationCodeRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { inner . send_notification_channel_verification_code ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = SendNotificationChannelVerificationCodeSvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } "/google.monitoring.v3.NotificationChannelService/GetNotificationChannelVerificationCode" => { # [ allow ( non_camel_case_types ) ] struct GetNotificationChannelVerificationCodeSvc < T : NotificationChannelService > ( pub Arc < T > ) ; impl < T : NotificationChannelService > tonic :: server :: UnaryService < super :: GetNotificationChannelVerificationCodeRequest > for GetNotificationChannelVerificationCodeSvc < T > { type Response = super :: GetNotificationChannelVerificationCodeResponse ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: GetNotificationChannelVerificationCodeRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { inner . get_notification_channel_verification_code ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = GetNotificationChannelVerificationCodeSvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } "/google.monitoring.v3.NotificationChannelService/VerifyNotificationChannel" => { # [ allow ( non_camel_case_types ) ] struct VerifyNotificationChannelSvc < T : NotificationChannelService > ( pub Arc < T > ) ; impl < T : NotificationChannelService > tonic :: server :: UnaryService < super :: VerifyNotificationChannelRequest > for VerifyNotificationChannelSvc < T > { type Response = super :: NotificationChannel ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: VerifyNotificationChannelRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { inner . verify_notification_channel ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = VerifyNotificationChannelSvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } _ => Box :: pin ( async move { Ok ( http :: Response :: builder ( ) . status ( 200 ) . header ( "grpc-status" , "12" ) . body ( tonic :: body :: BoxBody :: empty ( ) ) . unwrap ( ) ) } ) , }
        }
    }
    impl<T: NotificationChannelService> Clone for NotificationChannelServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: NotificationChannelService> Clone for _Inner<T> {
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
/// A `Service` is a discrete, autonomous, and network-accessible unit, designed
/// to solve an individual concern
/// ([Wikipedia](https://en.wikipedia.org/wiki/Service-orientation)). In
/// Cloud Monitoring, a `Service` acts as the root resource under which
/// operational aspects of the service are accessible.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Service {
    /// Resource name for this Service. The format is:
    ///
    ///     projects/[PROJECT_ID_OR_NUMBER]/services/[SERVICE_ID]
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Name used for UI elements listing this Service.
    #[prost(string, tag = "2")]
    pub display_name: std::string::String,
    /// Configuration for how to query telemetry on a Service.
    #[prost(message, optional, tag = "13")]
    pub telemetry: ::std::option::Option<service::Telemetry>,
    /// REQUIRED. Service-identifying atoms specifying the underlying service.
    #[prost(oneof = "service::Identifier", tags = "6, 7, 8, 9, 10")]
    pub identifier: ::std::option::Option<service::Identifier>,
}
pub mod service {
    /// Custom view of service telemetry. Currently a place-holder pending final
    /// design.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Custom {}
    /// App Engine service. Learn more at https://cloud.google.com/appengine.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct AppEngine {
        /// The ID of the App Engine module underlying this service. Corresponds to
        /// the `module_id` resource label in the `gae_app` monitored resource:
        /// https://cloud.google.com/monitoring/api/resources#tag_gae_app
        #[prost(string, tag = "1")]
        pub module_id: std::string::String,
    }
    /// Cloud Endpoints service. Learn more at https://cloud.google.com/endpoints.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CloudEndpoints {
        /// The name of the Cloud Endpoints service underlying this service.
        /// Corresponds to the `service` resource label in the `api` monitored
        /// resource: https://cloud.google.com/monitoring/api/resources#tag_api
        #[prost(string, tag = "1")]
        pub service: std::string::String,
    }
    /// Istio service scoped to a single Kubernetes cluster. Learn more at
    /// http://istio.io.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ClusterIstio {
        /// The location of the Kubernetes cluster in which this Istio service is
        /// defined. Corresponds to the `location` resource label in `k8s_cluster`
        /// resources.
        #[prost(string, tag = "1")]
        pub location: std::string::String,
        /// The name of the Kubernetes cluster in which this Istio service is
        /// defined. Corresponds to the `cluster_name` resource label in
        /// `k8s_cluster` resources.
        #[prost(string, tag = "2")]
        pub cluster_name: std::string::String,
        /// The namespace of the Istio service underlying this service. Corresponds
        /// to the `destination_service_namespace` metric label in Istio metrics.
        #[prost(string, tag = "3")]
        pub service_namespace: std::string::String,
        /// The name of the Istio service underlying this service. Corresponds to the
        /// `destination_service_name` metric label in Istio metrics.
        #[prost(string, tag = "4")]
        pub service_name: std::string::String,
    }
    /// Istio service scoped to an Istio mesh
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct MeshIstio {
        /// Identifier for the mesh in which this Istio service is defined.
        /// Corresponds to the `mesh_uid` metric label in Istio metrics.
        #[prost(string, tag = "1")]
        pub mesh_uid: std::string::String,
        /// The namespace of the Istio service underlying this service. Corresponds
        /// to the `destination_service_namespace` metric label in Istio metrics.
        #[prost(string, tag = "3")]
        pub service_namespace: std::string::String,
        /// The name of the Istio service underlying this service. Corresponds to the
        /// `destination_service_name` metric label in Istio metrics.
        #[prost(string, tag = "4")]
        pub service_name: std::string::String,
    }
    /// Configuration for how to query telemetry on a Service.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Telemetry {
        /// The full name of the resource that defines this service. Formatted as
        /// described in https://cloud.google.com/apis/design/resource_names.
        #[prost(string, tag = "1")]
        pub resource_name: std::string::String,
    }
    /// REQUIRED. Service-identifying atoms specifying the underlying service.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Identifier {
        /// Custom service type.
        #[prost(message, tag = "6")]
        Custom(Custom),
        /// Type used for App Engine services.
        #[prost(message, tag = "7")]
        AppEngine(AppEngine),
        /// Type used for Cloud Endpoints services.
        #[prost(message, tag = "8")]
        CloudEndpoints(CloudEndpoints),
        /// Type used for Istio services that live in a Kubernetes cluster.
        #[prost(message, tag = "9")]
        ClusterIstio(ClusterIstio),
        /// Type used for Istio services scoped to an Istio mesh.
        #[prost(message, tag = "10")]
        MeshIstio(MeshIstio),
    }
}
/// A Service-Level Objective (SLO) describes a level of desired good service. It
/// consists of a service-level indicator (SLI), a performance goal, and a period
/// over which the objective is to be evaluated against that goal. The SLO can
/// use SLIs defined in a number of different manners. Typical SLOs might include
/// "99% of requests in each rolling week have latency below 200 milliseconds" or
/// "99.5% of requests in each calendar month return successfully."
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServiceLevelObjective {
    /// Resource name for this `ServiceLevelObjective`. The format is:
    ///
    ///     projects/[PROJECT_ID_OR_NUMBER]/services/[SERVICE_ID]/serviceLevelObjectives/[SLO_NAME]
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Name used for UI elements listing this SLO.
    #[prost(string, tag = "11")]
    pub display_name: std::string::String,
    /// The definition of good service, used to measure and calculate the quality
    /// of the `Service`'s performance with respect to a single aspect of service
    /// quality.
    #[prost(message, optional, tag = "3")]
    pub service_level_indicator: ::std::option::Option<ServiceLevelIndicator>,
    /// The fraction of service that must be good in order for this objective to be
    /// met. `0 < goal <= 0.999`.
    #[prost(double, tag = "4")]
    pub goal: f64,
    /// The time period over which the objective will be evaluated.
    #[prost(oneof = "service_level_objective::Period", tags = "5, 6")]
    pub period: ::std::option::Option<service_level_objective::Period>,
}
pub mod service_level_objective {
    /// `ServiceLevelObjective.View` determines what form of
    /// `ServiceLevelObjective` is returned from `GetServiceLevelObjective`,
    /// `ListServiceLevelObjectives`, and `ListServiceLevelObjectiveVersions` RPCs.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum View {
        /// Same as FULL.
        Unspecified = 0,
        /// Return the embedded `ServiceLevelIndicator` in the form in which it was
        /// defined. If it was defined using a `BasicSli`, return that `BasicSli`.
        Full = 2,
        /// For `ServiceLevelIndicator`s using `BasicSli` articulation, instead
        /// return the `ServiceLevelIndicator` with its mode of computation fully
        /// spelled out as a `RequestBasedSli`. For `ServiceLevelIndicator`s using
        /// `RequestBasedSli` or `WindowsBasedSli`, return the
        /// `ServiceLevelIndicator` as it was provided.
        Explicit = 1,
    }
    /// The time period over which the objective will be evaluated.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Period {
        /// A rolling time period, semantically "in the past `<rolling_period>`".
        /// Must be an integer multiple of 1 day no larger than 30 days.
        #[prost(message, tag = "5")]
        RollingPeriod(::prost_types::Duration),
        /// A calendar period, semantically "since the start of the current
        /// `<calendar_period>`". At this time, only `DAY`, `WEEK`, `FORTNIGHT`, and
        /// `MONTH` are supported.
        #[prost(enumeration = "super::super::super::r#type::CalendarPeriod", tag = "6")]
        CalendarPeriod(i32),
    }
}
/// A Service-Level Indicator (SLI) describes the "performance" of a service. For
/// some services, the SLI is well-defined. In such cases, the SLI can be
/// described easily by referencing the well-known SLI and providing the needed
/// parameters. Alternatively, a "custom" SLI can be defined with a query to the
/// underlying metric store. An SLI is defined to be `good_service /
/// total_service` over any queried time interval. The value of performance
/// always falls into the range `0 <= performance <= 1`. A custom SLI describes
/// how to compute this ratio, whether this is by dividing values from a pair of
/// time series, cutting a `Distribution` into good and bad counts, or counting
/// time windows in which the service complies with a criterion. For separation
/// of concerns, a single Service-Level Indicator measures performance for only
/// one aspect of service quality, such as fraction of successful queries or
/// fast-enough queries.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServiceLevelIndicator {
    /// Service level indicators can be grouped by whether the "unit" of service
    /// being measured is based on counts of good requests or on counts of good
    /// time windows
    #[prost(oneof = "service_level_indicator::Type", tags = "4, 1, 2")]
    pub r#type: ::std::option::Option<service_level_indicator::Type>,
}
pub mod service_level_indicator {
    /// Service level indicators can be grouped by whether the "unit" of service
    /// being measured is based on counts of good requests or on counts of good
    /// time windows
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Type {
        /// Basic SLI on a well-known service type.
        #[prost(message, tag = "4")]
        BasicSli(super::BasicSli),
        /// Request-based SLIs
        #[prost(message, tag = "1")]
        RequestBased(super::RequestBasedSli),
        /// Windows-based SLIs
        #[prost(message, tag = "2")]
        WindowsBased(super::WindowsBasedSli),
    }
}
/// An SLI measuring performance on a well-known service type. Performance will
/// be computed on the basis of pre-defined metrics. The type of the
/// `service_resource` determines the metrics to use and the
/// `service_resource.labels` and `metric_labels` are used to construct a
/// monitoring filter to filter that metric down to just the data relevant to
/// this service.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BasicSli {
    /// OPTIONAL: The set of RPCs to which this SLI is relevant. Telemetry from
    /// other methods will not be used to calculate performance for this SLI. If
    /// omitted, this SLI applies to all the Service's methods. For service types
    /// that don't support breaking down by method, setting this field will result
    /// in an error.
    #[prost(string, repeated, tag = "7")]
    pub method: ::std::vec::Vec<std::string::String>,
    /// OPTIONAL: The set of locations to which this SLI is relevant. Telemetry
    /// from other locations will not be used to calculate performance for this
    /// SLI. If omitted, this SLI applies to all locations in which the Service has
    /// activity. For service types that don't support breaking down by location,
    /// setting this field will result in an error.
    #[prost(string, repeated, tag = "8")]
    pub location: ::std::vec::Vec<std::string::String>,
    /// OPTIONAL: The set of API versions to which this SLI is relevant. Telemetry
    /// from other API versions will not be used to calculate performance for this
    /// SLI. If omitted, this SLI applies to all API versions. For service types
    /// that don't support breaking down by version, setting this field will result
    /// in an error.
    #[prost(string, repeated, tag = "9")]
    pub version: ::std::vec::Vec<std::string::String>,
    /// This SLI can be evaluated on the basis of availability or latency.
    #[prost(oneof = "basic_sli::SliCriteria", tags = "2, 3")]
    pub sli_criteria: ::std::option::Option<basic_sli::SliCriteria>,
}
pub mod basic_sli {
    /// Future parameters for the availability SLI.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct AvailabilityCriteria {}
    /// Parameters for a latency threshold SLI.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct LatencyCriteria {
        /// Good service is defined to be the count of requests made to this service
        /// that return in no more than `threshold`.
        #[prost(message, optional, tag = "3")]
        pub threshold: ::std::option::Option<::prost_types::Duration>,
    }
    /// This SLI can be evaluated on the basis of availability or latency.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum SliCriteria {
        /// Good service is defined to be the count of requests made to this service
        /// that return successfully.
        #[prost(message, tag = "2")]
        Availability(AvailabilityCriteria),
        /// Good service is defined to be the count of requests made to this service
        /// that are fast enough with respect to `latency.threshold`.
        #[prost(message, tag = "3")]
        Latency(LatencyCriteria),
    }
}
/// Range of numerical values, inclusive of `min` and exclusive of `max`. If the
/// open range "< range.max" is desired, set `range.min = -infinity`. If the open
/// range ">= range.min" is desired, set `range.max = infinity`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Range {
    /// Range minimum.
    #[prost(double, tag = "1")]
    pub min: f64,
    /// Range maximum.
    #[prost(double, tag = "2")]
    pub max: f64,
}
/// Service Level Indicators for which atomic units of service are counted
/// directly.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestBasedSli {
    /// The means to compute a ratio of `good_service` to `total_service`.
    #[prost(oneof = "request_based_sli::Method", tags = "1, 3")]
    pub method: ::std::option::Option<request_based_sli::Method>,
}
pub mod request_based_sli {
    /// The means to compute a ratio of `good_service` to `total_service`.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Method {
        /// `good_total_ratio` is used when the ratio of `good_service` to
        /// `total_service` is computed from two `TimeSeries`.
        #[prost(message, tag = "1")]
        GoodTotalRatio(super::TimeSeriesRatio),
        /// `distribution_cut` is used when `good_service` is a count of values
        /// aggregated in a `Distribution` that fall into a good range. The
        /// `total_service` is the total count of all values aggregated in the
        /// `Distribution`.
        #[prost(message, tag = "3")]
        DistributionCut(super::DistributionCut),
    }
}
/// A `TimeSeriesRatio` specifies two `TimeSeries` to use for computing the
/// `good_service / total_service` ratio. The specified `TimeSeries` must have
/// `ValueType = DOUBLE` or `ValueType = INT64` and must have `MetricKind =
/// DELTA` or `MetricKind = CUMULATIVE`. The `TimeSeriesRatio` must specify
/// exactly two of good, bad, and total, and the relationship `good_service +
/// bad_service = total_service` will be assumed.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TimeSeriesRatio {
    /// A [monitoring filter](https://cloud.google.com/monitoring/api/v3/filters)
    /// specifying a `TimeSeries` quantifying good service provided. Must have
    /// `ValueType = DOUBLE` or `ValueType = INT64` and must have `MetricKind =
    /// DELTA` or `MetricKind = CUMULATIVE`.
    #[prost(string, tag = "4")]
    pub good_service_filter: std::string::String,
    /// A [monitoring filter](https://cloud.google.com/monitoring/api/v3/filters)
    /// specifying a `TimeSeries` quantifying bad service, either demanded service
    /// that was not provided or demanded service that was of inadequate quality.
    /// Must have `ValueType = DOUBLE` or `ValueType = INT64` and must have
    /// `MetricKind = DELTA` or `MetricKind = CUMULATIVE`.
    #[prost(string, tag = "5")]
    pub bad_service_filter: std::string::String,
    /// A [monitoring filter](https://cloud.google.com/monitoring/api/v3/filters)
    /// specifying a `TimeSeries` quantifying total demanded service. Must have
    /// `ValueType = DOUBLE` or `ValueType = INT64` and must have `MetricKind =
    /// DELTA` or `MetricKind = CUMULATIVE`.
    #[prost(string, tag = "6")]
    pub total_service_filter: std::string::String,
}
/// A `DistributionCut` defines a `TimeSeries` and thresholds used for measuring
/// good service and total service. The `TimeSeries` must have `ValueType =
/// DISTRIBUTION` and `MetricKind = DELTA` or `MetricKind = CUMULATIVE`. The
/// computed `good_service` will be the count of values x in the `Distribution`
/// such that `range.min <= x < range.max`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DistributionCut {
    /// A [monitoring filter](https://cloud.google.com/monitoring/api/v3/filters)
    /// specifying a `TimeSeries` aggregating values. Must have `ValueType =
    /// DISTRIBUTION` and `MetricKind = DELTA` or `MetricKind = CUMULATIVE`.
    #[prost(string, tag = "4")]
    pub distribution_filter: std::string::String,
    /// Range of values considered "good." For a one-sided range, set one bound to
    /// an infinite value.
    #[prost(message, optional, tag = "5")]
    pub range: ::std::option::Option<Range>,
}
/// A `WindowsBasedSli` defines `good_service` as the count of time windows for
/// which the provided service was of good quality. Criteria for determining
/// if service was good are embedded in the `window_criterion`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WindowsBasedSli {
    /// Duration over which window quality is evaluated. Must be an integer
    /// fraction of a day and at least `60s`.
    #[prost(message, optional, tag = "4")]
    pub window_period: ::std::option::Option<::prost_types::Duration>,
    /// The criterion to use for evaluating window goodness.
    #[prost(oneof = "windows_based_sli::WindowCriterion", tags = "5, 2, 6, 7")]
    pub window_criterion: ::std::option::Option<windows_based_sli::WindowCriterion>,
}
pub mod windows_based_sli {
    /// A `PerformanceThreshold` is used when each window is good when that window
    /// has a sufficiently high `performance`.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PerformanceThreshold {
        /// If window `performance >= threshold`, the window is counted as good.
        #[prost(double, tag = "2")]
        pub threshold: f64,
        /// The means, either a request-based SLI or a basic SLI, by which to compute
        /// performance over a window.
        #[prost(oneof = "performance_threshold::Type", tags = "1, 3")]
        pub r#type: ::std::option::Option<performance_threshold::Type>,
    }
    pub mod performance_threshold {
        /// The means, either a request-based SLI or a basic SLI, by which to compute
        /// performance over a window.
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Type {
            /// `RequestBasedSli` to evaluate to judge window quality.
            #[prost(message, tag = "1")]
            Performance(super::super::RequestBasedSli),
            /// `BasicSli` to evaluate to judge window quality.
            #[prost(message, tag = "3")]
            BasicSliPerformance(super::super::BasicSli),
        }
    }
    /// A `MetricRange` is used when each window is good when the value x of a
    /// single `TimeSeries` satisfies `range.min <= x < range.max`. The provided
    /// `TimeSeries` must have `ValueType = INT64` or `ValueType = DOUBLE` and
    /// `MetricKind = GAUGE`.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct MetricRange {
        /// A [monitoring filter](https://cloud.google.com/monitoring/api/v3/filters)
        /// specifying the `TimeSeries` to use for evaluating window quality.
        #[prost(string, tag = "1")]
        pub time_series: std::string::String,
        /// Range of values considered "good." For a one-sided range, set one bound
        /// to an infinite value.
        #[prost(message, optional, tag = "4")]
        pub range: ::std::option::Option<super::Range>,
    }
    /// The criterion to use for evaluating window goodness.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum WindowCriterion {
        /// A [monitoring filter](https://cloud.google.com/monitoring/api/v3/filters)
        /// specifying a `TimeSeries` with `ValueType = BOOL`. The window is good if
        /// any `true` values appear in the window.
        #[prost(string, tag = "5")]
        GoodBadMetricFilter(std::string::String),
        /// A window is good if its `performance` is high enough.
        #[prost(message, tag = "2")]
        GoodTotalRatioThreshold(PerformanceThreshold),
        /// A window is good if the metric's value is in a good range, averaged
        /// across returned streams.
        #[prost(message, tag = "6")]
        MetricMeanInRange(MetricRange),
        /// A window is good if the metric's value is in a good range, summed across
        /// returned streams.
        #[prost(message, tag = "7")]
        MetricSumInRange(MetricRange),
    }
}
/// The `CreateService` request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateServiceRequest {
    /// Required. Resource name of the parent workspace. The format is:
    ///
    ///     projects/[PROJECT_ID_OR_NUMBER]
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Optional. The Service id to use for this Service. If omitted, an id will be
    /// generated instead. Must match the pattern `[a-z0-9\-]+`
    #[prost(string, tag = "3")]
    pub service_id: std::string::String,
    /// Required. The `Service` to create.
    #[prost(message, optional, tag = "2")]
    pub service: ::std::option::Option<Service>,
}
/// The `GetService` request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetServiceRequest {
    /// Required. Resource name of the `Service`. The format is:
    ///
    ///     projects/[PROJECT_ID_OR_NUMBER]/services/[SERVICE_ID]
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// The `ListServices` request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListServicesRequest {
    /// Required. Resource name of the parent containing the listed services, either a
    /// project or a Monitoring Workspace. The formats are:
    ///
    ///     projects/[PROJECT_ID_OR_NUMBER]
    ///     workspaces/[HOST_PROJECT_ID_OR_NUMBER]
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// A filter specifying what `Service`s to return. The filter currently
    /// supports the following fields:
    ///
    ///     - `identifier_case`
    ///     - `app_engine.module_id`
    ///     - `cloud_endpoints.service`
    ///     - `cluster_istio.location`
    ///     - `cluster_istio.cluster_name`
    ///     - `cluster_istio.service_namespace`
    ///     - `cluster_istio.service_name`
    ///
    /// `identifier_case` refers to which option in the identifier oneof is
    /// populated. For example, the filter `identifier_case = "CUSTOM"` would match
    /// all services with a value for the `custom` field. Valid options are
    /// "CUSTOM", "APP_ENGINE", "CLOUD_ENDPOINTS", and "CLUSTER_ISTIO".
    #[prost(string, tag = "2")]
    pub filter: std::string::String,
    /// A non-negative number that is the maximum number of results to return.
    /// When 0, use default page size.
    #[prost(int32, tag = "3")]
    pub page_size: i32,
    /// If this field is not empty then it must contain the `nextPageToken` value
    /// returned by a previous call to this method.  Using this field causes the
    /// method to return additional results from the previous method call.
    #[prost(string, tag = "4")]
    pub page_token: std::string::String,
}
/// The `ListServices` response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListServicesResponse {
    /// The `Service`s matching the specified filter.
    #[prost(message, repeated, tag = "1")]
    pub services: ::std::vec::Vec<Service>,
    /// If there are more results than have been returned, then this field is set
    /// to a non-empty value.  To see the additional results,
    /// use that value as `page_token` in the next call to this method.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
}
/// The `UpdateService` request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateServiceRequest {
    /// Required. The `Service` to draw updates from.
    /// The given `name` specifies the resource to update.
    #[prost(message, optional, tag = "1")]
    pub service: ::std::option::Option<Service>,
    /// A set of field paths defining which fields to use for the update.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
}
/// The `DeleteService` request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteServiceRequest {
    /// Required. Resource name of the `Service` to delete. The format is:
    ///
    ///     projects/[PROJECT_ID_OR_NUMBER]/services/[SERVICE_ID]
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// The `CreateServiceLevelObjective` request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateServiceLevelObjectiveRequest {
    /// Required. Resource name of the parent `Service`. The format is:
    ///
    ///     projects/[PROJECT_ID_OR_NUMBER]/services/[SERVICE_ID]
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Optional. The ServiceLevelObjective id to use for this
    /// ServiceLevelObjective. If omitted, an id will be generated instead. Must
    /// match the pattern `[a-z0-9\-]+`
    #[prost(string, tag = "3")]
    pub service_level_objective_id: std::string::String,
    /// Required. The `ServiceLevelObjective` to create.
    /// The provided `name` will be respected if no `ServiceLevelObjective` exists
    /// with this name.
    #[prost(message, optional, tag = "2")]
    pub service_level_objective: ::std::option::Option<ServiceLevelObjective>,
}
/// The `GetServiceLevelObjective` request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetServiceLevelObjectiveRequest {
    /// Required. Resource name of the `ServiceLevelObjective` to get. The format is:
    ///
    ///     projects/[PROJECT_ID_OR_NUMBER]/services/[SERVICE_ID]/serviceLevelObjectives/[SLO_NAME]
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// View of the `ServiceLevelObjective` to return. If `DEFAULT`, return the
    /// `ServiceLevelObjective` as originally defined. If `EXPLICIT` and the
    /// `ServiceLevelObjective` is defined in terms of a `BasicSli`, replace the
    /// `BasicSli` with a `RequestBasedSli` spelling out how the SLI is computed.
    #[prost(enumeration = "service_level_objective::View", tag = "2")]
    pub view: i32,
}
/// The `ListServiceLevelObjectives` request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListServiceLevelObjectivesRequest {
    /// Required. Resource name of the parent containing the listed SLOs, either a
    /// project or a Monitoring Workspace. The formats are:
    ///
    ///     projects/[PROJECT_ID_OR_NUMBER]/services/[SERVICE_ID]
    ///     workspaces/[HOST_PROJECT_ID_OR_NUMBER]/services/-
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// A filter specifying what `ServiceLevelObjective`s to return.
    #[prost(string, tag = "2")]
    pub filter: std::string::String,
    /// A non-negative number that is the maximum number of results to return.
    /// When 0, use default page size.
    #[prost(int32, tag = "3")]
    pub page_size: i32,
    /// If this field is not empty then it must contain the `nextPageToken` value
    /// returned by a previous call to this method.  Using this field causes the
    /// method to return additional results from the previous method call.
    #[prost(string, tag = "4")]
    pub page_token: std::string::String,
    /// View of the `ServiceLevelObjective`s to return. If `DEFAULT`, return each
    /// `ServiceLevelObjective` as originally defined. If `EXPLICIT` and the
    /// `ServiceLevelObjective` is defined in terms of a `BasicSli`, replace the
    /// `BasicSli` with a `RequestBasedSli` spelling out how the SLI is computed.
    #[prost(enumeration = "service_level_objective::View", tag = "5")]
    pub view: i32,
}
/// The `ListServiceLevelObjectives` response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListServiceLevelObjectivesResponse {
    /// The `ServiceLevelObjective`s matching the specified filter.
    #[prost(message, repeated, tag = "1")]
    pub service_level_objectives: ::std::vec::Vec<ServiceLevelObjective>,
    /// If there are more results than have been returned, then this field is set
    /// to a non-empty value.  To see the additional results,
    /// use that value as `page_token` in the next call to this method.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
}
/// The `UpdateServiceLevelObjective` request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateServiceLevelObjectiveRequest {
    /// Required. The `ServiceLevelObjective` to draw updates from.
    /// The given `name` specifies the resource to update.
    #[prost(message, optional, tag = "1")]
    pub service_level_objective: ::std::option::Option<ServiceLevelObjective>,
    /// A set of field paths defining which fields to use for the update.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
}
/// The `DeleteServiceLevelObjective` request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteServiceLevelObjectiveRequest {
    /// Required. Resource name of the `ServiceLevelObjective` to delete. The format is:
    ///
    ///     projects/[PROJECT_ID_OR_NUMBER]/services/[SERVICE_ID]/serviceLevelObjectives/[SLO_NAME]
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
#[doc = r" Generated client implementations."]
pub mod service_monitoring_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " The Cloud Monitoring Service-Oriented Monitoring API has endpoints for"]
    #[doc = " managing and querying aspects of a workspace's services. These include the"]
    #[doc = " `Service`'s monitored resources, its Service-Level Objectives, and a taxonomy"]
    #[doc = " of categorized Health Metrics."]
    pub struct ServiceMonitoringServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> ServiceMonitoringServiceClient<T>
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
        #[doc = " Create a `Service`."]
        pub async fn create_service(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateServiceRequest>,
        ) -> Result<tonic::Response<super::Service>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.monitoring.v3.ServiceMonitoringService/CreateService",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Get the named `Service`."]
        pub async fn get_service(
            &mut self,
            request: impl tonic::IntoRequest<super::GetServiceRequest>,
        ) -> Result<tonic::Response<super::Service>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.monitoring.v3.ServiceMonitoringService/GetService",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " List `Service`s for this workspace."]
        pub async fn list_services(
            &mut self,
            request: impl tonic::IntoRequest<super::ListServicesRequest>,
        ) -> Result<tonic::Response<super::ListServicesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.monitoring.v3.ServiceMonitoringService/ListServices",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Update this `Service`."]
        pub async fn update_service(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateServiceRequest>,
        ) -> Result<tonic::Response<super::Service>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.monitoring.v3.ServiceMonitoringService/UpdateService",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Soft delete this `Service`."]
        pub async fn delete_service(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteServiceRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.monitoring.v3.ServiceMonitoringService/DeleteService",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Create a `ServiceLevelObjective` for the given `Service`."]
        pub async fn create_service_level_objective(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateServiceLevelObjectiveRequest>,
        ) -> Result<tonic::Response<super::ServiceLevelObjective>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.monitoring.v3.ServiceMonitoringService/CreateServiceLevelObjective",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Get a `ServiceLevelObjective` by name."]
        pub async fn get_service_level_objective(
            &mut self,
            request: impl tonic::IntoRequest<super::GetServiceLevelObjectiveRequest>,
        ) -> Result<tonic::Response<super::ServiceLevelObjective>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.monitoring.v3.ServiceMonitoringService/GetServiceLevelObjective",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " List the `ServiceLevelObjective`s for the given `Service`."]
        pub async fn list_service_level_objectives(
            &mut self,
            request: impl tonic::IntoRequest<super::ListServiceLevelObjectivesRequest>,
        ) -> Result<tonic::Response<super::ListServiceLevelObjectivesResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.monitoring.v3.ServiceMonitoringService/ListServiceLevelObjectives",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Update the given `ServiceLevelObjective`."]
        pub async fn update_service_level_objective(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateServiceLevelObjectiveRequest>,
        ) -> Result<tonic::Response<super::ServiceLevelObjective>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.monitoring.v3.ServiceMonitoringService/UpdateServiceLevelObjective",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Delete the given `ServiceLevelObjective`."]
        pub async fn delete_service_level_objective(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteServiceLevelObjectiveRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.monitoring.v3.ServiceMonitoringService/DeleteServiceLevelObjective",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for ServiceMonitoringServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for ServiceMonitoringServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "ServiceMonitoringServiceClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod service_monitoring_service_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with ServiceMonitoringServiceServer."]
    #[async_trait]
    pub trait ServiceMonitoringService: Send + Sync + 'static {
        #[doc = " Create a `Service`."]
        async fn create_service(
            &self,
            request: tonic::Request<super::CreateServiceRequest>,
        ) -> Result<tonic::Response<super::Service>, tonic::Status>;
        #[doc = " Get the named `Service`."]
        async fn get_service(
            &self,
            request: tonic::Request<super::GetServiceRequest>,
        ) -> Result<tonic::Response<super::Service>, tonic::Status>;
        #[doc = " List `Service`s for this workspace."]
        async fn list_services(
            &self,
            request: tonic::Request<super::ListServicesRequest>,
        ) -> Result<tonic::Response<super::ListServicesResponse>, tonic::Status>;
        #[doc = " Update this `Service`."]
        async fn update_service(
            &self,
            request: tonic::Request<super::UpdateServiceRequest>,
        ) -> Result<tonic::Response<super::Service>, tonic::Status>;
        #[doc = " Soft delete this `Service`."]
        async fn delete_service(
            &self,
            request: tonic::Request<super::DeleteServiceRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status>;
        #[doc = " Create a `ServiceLevelObjective` for the given `Service`."]
        async fn create_service_level_objective(
            &self,
            request: tonic::Request<super::CreateServiceLevelObjectiveRequest>,
        ) -> Result<tonic::Response<super::ServiceLevelObjective>, tonic::Status>;
        #[doc = " Get a `ServiceLevelObjective` by name."]
        async fn get_service_level_objective(
            &self,
            request: tonic::Request<super::GetServiceLevelObjectiveRequest>,
        ) -> Result<tonic::Response<super::ServiceLevelObjective>, tonic::Status>;
        #[doc = " List the `ServiceLevelObjective`s for the given `Service`."]
        async fn list_service_level_objectives(
            &self,
            request: tonic::Request<super::ListServiceLevelObjectivesRequest>,
        ) -> Result<tonic::Response<super::ListServiceLevelObjectivesResponse>, tonic::Status>;
        #[doc = " Update the given `ServiceLevelObjective`."]
        async fn update_service_level_objective(
            &self,
            request: tonic::Request<super::UpdateServiceLevelObjectiveRequest>,
        ) -> Result<tonic::Response<super::ServiceLevelObjective>, tonic::Status>;
        #[doc = " Delete the given `ServiceLevelObjective`."]
        async fn delete_service_level_objective(
            &self,
            request: tonic::Request<super::DeleteServiceLevelObjectiveRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status>;
    }
    #[doc = " The Cloud Monitoring Service-Oriented Monitoring API has endpoints for"]
    #[doc = " managing and querying aspects of a workspace's services. These include the"]
    #[doc = " `Service`'s monitored resources, its Service-Level Objectives, and a taxonomy"]
    #[doc = " of categorized Health Metrics."]
    #[derive(Debug)]
    #[doc(hidden)]
    pub struct ServiceMonitoringServiceServer<T: ServiceMonitoringService> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: ServiceMonitoringService> ServiceMonitoringServiceServer<T> {
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
    impl<T, B> Service<http::Request<B>> for ServiceMonitoringServiceServer<T>
    where
        T: ServiceMonitoringService,
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
                "/google.monitoring.v3.ServiceMonitoringService/CreateService" => {
                    #[allow(non_camel_case_types)]
                    struct CreateServiceSvc<T: ServiceMonitoringService>(pub Arc<T>);
                    impl<T: ServiceMonitoringService>
                        tonic::server::UnaryService<super::CreateServiceRequest>
                        for CreateServiceSvc<T>
                    {
                        type Response = super::Service;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateServiceRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.create_service(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = CreateServiceSvc(inner);
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
                "/google.monitoring.v3.ServiceMonitoringService/GetService" => {
                    #[allow(non_camel_case_types)]
                    struct GetServiceSvc<T: ServiceMonitoringService>(pub Arc<T>);
                    impl<T: ServiceMonitoringService>
                        tonic::server::UnaryService<super::GetServiceRequest> for GetServiceSvc<T>
                    {
                        type Response = super::Service;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetServiceRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.get_service(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetServiceSvc(inner);
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
                "/google.monitoring.v3.ServiceMonitoringService/ListServices" => {
                    #[allow(non_camel_case_types)]
                    struct ListServicesSvc<T: ServiceMonitoringService>(pub Arc<T>);
                    impl<T: ServiceMonitoringService>
                        tonic::server::UnaryService<super::ListServicesRequest>
                        for ListServicesSvc<T>
                    {
                        type Response = super::ListServicesResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListServicesRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.list_services(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ListServicesSvc(inner);
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
                "/google.monitoring.v3.ServiceMonitoringService/UpdateService" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateServiceSvc<T: ServiceMonitoringService>(pub Arc<T>);
                    impl<T: ServiceMonitoringService>
                        tonic::server::UnaryService<super::UpdateServiceRequest>
                        for UpdateServiceSvc<T>
                    {
                        type Response = super::Service;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateServiceRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.update_service(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = UpdateServiceSvc(inner);
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
                "/google.monitoring.v3.ServiceMonitoringService/DeleteService" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteServiceSvc<T: ServiceMonitoringService>(pub Arc<T>);
                    impl<T: ServiceMonitoringService>
                        tonic::server::UnaryService<super::DeleteServiceRequest>
                        for DeleteServiceSvc<T>
                    {
                        type Response = ();
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteServiceRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.delete_service(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = DeleteServiceSvc(inner);
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
                "/google.monitoring.v3.ServiceMonitoringService/CreateServiceLevelObjective" => {
                    #[allow(non_camel_case_types)]
                    struct CreateServiceLevelObjectiveSvc<T: ServiceMonitoringService>(pub Arc<T>);
                    impl<T: ServiceMonitoringService>
                        tonic::server::UnaryService<super::CreateServiceLevelObjectiveRequest>
                        for CreateServiceLevelObjectiveSvc<T>
                    {
                        type Response = super::ServiceLevelObjective;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateServiceLevelObjectiveRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { inner.create_service_level_objective(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = CreateServiceLevelObjectiveSvc(inner);
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
                "/google.monitoring.v3.ServiceMonitoringService/GetServiceLevelObjective" => {
                    #[allow(non_camel_case_types)]
                    struct GetServiceLevelObjectiveSvc<T: ServiceMonitoringService>(pub Arc<T>);
                    impl<T: ServiceMonitoringService>
                        tonic::server::UnaryService<super::GetServiceLevelObjectiveRequest>
                        for GetServiceLevelObjectiveSvc<T>
                    {
                        type Response = super::ServiceLevelObjective;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetServiceLevelObjectiveRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { inner.get_service_level_objective(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetServiceLevelObjectiveSvc(inner);
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
                "/google.monitoring.v3.ServiceMonitoringService/ListServiceLevelObjectives" => {
                    #[allow(non_camel_case_types)]
                    struct ListServiceLevelObjectivesSvc<T: ServiceMonitoringService>(pub Arc<T>);
                    impl<T: ServiceMonitoringService>
                        tonic::server::UnaryService<super::ListServiceLevelObjectivesRequest>
                        for ListServiceLevelObjectivesSvc<T>
                    {
                        type Response = super::ListServiceLevelObjectivesResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListServiceLevelObjectivesRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { inner.list_service_level_objectives(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ListServiceLevelObjectivesSvc(inner);
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
                "/google.monitoring.v3.ServiceMonitoringService/UpdateServiceLevelObjective" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateServiceLevelObjectiveSvc<T: ServiceMonitoringService>(pub Arc<T>);
                    impl<T: ServiceMonitoringService>
                        tonic::server::UnaryService<super::UpdateServiceLevelObjectiveRequest>
                        for UpdateServiceLevelObjectiveSvc<T>
                    {
                        type Response = super::ServiceLevelObjective;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateServiceLevelObjectiveRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { inner.update_service_level_objective(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = UpdateServiceLevelObjectiveSvc(inner);
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
                "/google.monitoring.v3.ServiceMonitoringService/DeleteServiceLevelObjective" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteServiceLevelObjectiveSvc<T: ServiceMonitoringService>(pub Arc<T>);
                    impl<T: ServiceMonitoringService>
                        tonic::server::UnaryService<super::DeleteServiceLevelObjectiveRequest>
                        for DeleteServiceLevelObjectiveSvc<T>
                    {
                        type Response = ();
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteServiceLevelObjectiveRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { inner.delete_service_level_objective(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = DeleteServiceLevelObjectiveSvc(inner);
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
    impl<T: ServiceMonitoringService> Clone for ServiceMonitoringServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: ServiceMonitoringService> Clone for _Inner<T> {
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
/// The context of a span, attached to
/// [Exemplars][google.api.Distribution.Exemplars]
/// in [Distribution][google.api.Distribution] values during aggregation.
///
/// It contains the name of a span with format:
///
///     projects/[PROJECT_ID_OR_NUMBER]/traces/[TRACE_ID]/spans/[SPAN_ID]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpanContext {
    /// The resource name of the span. The format is:
    ///
    ///     projects/[PROJECT_ID_OR_NUMBER]/traces/[TRACE_ID]/spans/[SPAN_ID]
    ///
    /// `[TRACE_ID]` is a unique identifier for a trace within a project;
    /// it is a 32-character hexadecimal encoding of a 16-byte array.
    ///
    /// `[SPAN_ID]` is a unique identifier for a span within a trace; it
    /// is a 16-character hexadecimal encoding of an 8-byte array.
    #[prost(string, tag = "1")]
    pub span_name: std::string::String,
}
/// An internal checker allows Uptime checks to run on private/internal GCP
/// resources.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InternalChecker {
    /// A unique resource name for this InternalChecker. The format is:
    ///
    ///     projects/[PROJECT_ID_OR_NUMBER]/internalCheckers/[INTERNAL_CHECKER_ID]
    ///
    /// `[PROJECT_ID_OR_NUMBER]` is the Stackdriver Workspace project for the
    /// Uptime check config associated with the internal checker.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// The checker's human-readable name. The display name
    /// should be unique within a Stackdriver Workspace in order to make it easier
    /// to identify; however, uniqueness is not enforced.
    #[prost(string, tag = "2")]
    pub display_name: std::string::String,
    /// The [GCP VPC network](https://cloud.google.com/vpc/docs/vpc) where the
    /// internal resource lives (ex: "default").
    #[prost(string, tag = "3")]
    pub network: std::string::String,
    /// The GCP zone the Uptime check should egress from. Only respected for
    /// internal Uptime checks, where internal_network is specified.
    #[prost(string, tag = "4")]
    pub gcp_zone: std::string::String,
    /// The GCP project ID where the internal checker lives. Not necessary
    /// the same as the Workspace project.
    #[prost(string, tag = "6")]
    pub peer_project_id: std::string::String,
    /// The current operational state of the internal checker.
    #[prost(enumeration = "internal_checker::State", tag = "7")]
    pub state: i32,
}
pub mod internal_checker {
    /// Operational states for an internal checker.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// An internal checker should never be in the unspecified state.
        Unspecified = 0,
        /// The checker is being created, provisioned, and configured. A checker in
        /// this state can be returned by `ListInternalCheckers` or
        /// `GetInternalChecker`, as well as by examining the [long running
        /// Operation](https://cloud.google.com/apis/design/design_patterns#long_running_operations)
        /// that created it.
        Creating = 1,
        /// The checker is running and available for use. A checker in this state
        /// can be returned by `ListInternalCheckers` or `GetInternalChecker` as
        /// well as by examining the [long running
        /// Operation](https://cloud.google.com/apis/design/design_patterns#long_running_operations)
        /// that created it.
        /// If a checker is being torn down, it is neither visible nor usable, so
        /// there is no "deleting" or "down" state.
        Running = 2,
    }
}
/// This message configures which resources and services to monitor for
/// availability.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UptimeCheckConfig {
    /// A unique resource name for this Uptime check configuration. The format is:
    ///
    ///      projects/[PROJECT_ID_OR_NUMBER]/uptimeCheckConfigs/[UPTIME_CHECK_ID]
    ///
    /// This field should be omitted when creating the Uptime check configuration;
    /// on create, the resource name is assigned by the server and included in the
    /// response.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// A human-friendly name for the Uptime check configuration. The display name
    /// should be unique within a Stackdriver Workspace in order to make it easier
    /// to identify; however, uniqueness is not enforced. Required.
    #[prost(string, tag = "2")]
    pub display_name: std::string::String,
    /// How often, in seconds, the Uptime check is performed.
    /// Currently, the only supported values are `60s` (1 minute), `300s`
    /// (5 minutes), `600s` (10 minutes), and `900s` (15 minutes). Optional,
    /// defaults to `60s`.
    #[prost(message, optional, tag = "7")]
    pub period: ::std::option::Option<::prost_types::Duration>,
    /// The maximum amount of time to wait for the request to complete (must be
    /// between 1 and 60 seconds). Required.
    #[prost(message, optional, tag = "8")]
    pub timeout: ::std::option::Option<::prost_types::Duration>,
    /// The content that is expected to appear in the data returned by the target
    /// server against which the check is run.  Currently, only the first entry
    /// in the `content_matchers` list is supported, and additional entries will
    /// be ignored. This field is optional and should only be specified if a
    /// content match is required as part of the/ Uptime check.
    #[prost(message, repeated, tag = "9")]
    pub content_matchers: ::std::vec::Vec<uptime_check_config::ContentMatcher>,
    /// The list of regions from which the check will be run.
    /// Some regions contain one location, and others contain more than one.
    /// If this field is specified, enough regions must be provided to include a
    /// minimum of 3 locations.  Not specifying this field will result in Uptime
    /// checks running from all available regions.
    #[prost(enumeration = "UptimeCheckRegion", repeated, tag = "10")]
    pub selected_regions: ::std::vec::Vec<i32>,
    /// If this is `true`, then checks are made only from the 'internal_checkers'.
    /// If it is `false`, then checks are made only from the 'selected_regions'.
    /// It is an error to provide 'selected_regions' when is_internal is `true`,
    /// or to provide 'internal_checkers' when is_internal is `false`.
    #[prost(bool, tag = "15")]
    pub is_internal: bool,
    /// The internal checkers that this check will egress from. If `is_internal` is
    /// `true` and this list is empty, the check will egress from all the
    /// InternalCheckers configured for the project that owns this
    /// `UptimeCheckConfig`.
    #[prost(message, repeated, tag = "14")]
    pub internal_checkers: ::std::vec::Vec<InternalChecker>,
    /// The resource the check is checking. Required.
    #[prost(oneof = "uptime_check_config::Resource", tags = "3, 4")]
    pub resource: ::std::option::Option<uptime_check_config::Resource>,
    /// The type of Uptime check request.
    #[prost(oneof = "uptime_check_config::CheckRequestType", tags = "5, 6")]
    pub check_request_type: ::std::option::Option<uptime_check_config::CheckRequestType>,
}
pub mod uptime_check_config {
    /// The resource submessage for group checks. It can be used instead of a
    /// monitored resource, when multiple resources are being monitored.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ResourceGroup {
        /// The group of resources being monitored. Should be only the `[GROUP_ID]`,
        /// and not the full-path
        /// `projects/[PROJECT_ID_OR_NUMBER]/groups/[GROUP_ID]`.
        #[prost(string, tag = "1")]
        pub group_id: std::string::String,
        /// The resource type of the group members.
        #[prost(enumeration = "super::GroupResourceType", tag = "2")]
        pub resource_type: i32,
    }
    /// Information involved in an HTTP/HTTPS Uptime check request.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct HttpCheck {
        /// The HTTP request method to use for the check.
        #[prost(enumeration = "http_check::RequestMethod", tag = "8")]
        pub request_method: i32,
        /// If `true`, use HTTPS instead of HTTP to run the check.
        #[prost(bool, tag = "1")]
        pub use_ssl: bool,
        /// Optional (defaults to "/"). The path to the page against which to run
        /// the check. Will be combined with the `host` (specified within the
        /// `monitored_resource`) and `port` to construct the full URL. If the
        /// provided path does not begin with "/", a "/" will be prepended
        /// automatically.
        #[prost(string, tag = "2")]
        pub path: std::string::String,
        /// Optional (defaults to 80 when `use_ssl` is `false`, and 443 when
        /// `use_ssl` is `true`). The TCP port on the HTTP server against which to
        /// run the check. Will be combined with host (specified within the
        /// `monitored_resource`) and `path` to construct the full URL.
        #[prost(int32, tag = "3")]
        pub port: i32,
        /// The authentication information. Optional when creating an HTTP check;
        /// defaults to empty.
        #[prost(message, optional, tag = "4")]
        pub auth_info: ::std::option::Option<http_check::BasicAuthentication>,
        /// Boolean specifiying whether to encrypt the header information.
        /// Encryption should be specified for any headers related to authentication
        /// that you do not wish to be seen when retrieving the configuration. The
        /// server will be responsible for encrypting the headers.
        /// On Get/List calls, if `mask_headers` is set to `true` then the headers
        /// will be obscured with `******.`
        #[prost(bool, tag = "5")]
        pub mask_headers: bool,
        /// The list of headers to send as part of the Uptime check request.
        /// If two headers have the same key and different values, they should
        /// be entered as a single header, with the value being a comma-separated
        /// list of all the desired values as described at
        /// https://www.w3.org/Protocols/rfc2616/rfc2616.txt (page 31).
        /// Entering two separate headers with the same key in a Create call will
        /// cause the first to be overwritten by the second.
        /// The maximum number of headers allowed is 100.
        #[prost(map = "string, string", tag = "6")]
        pub headers: ::std::collections::HashMap<std::string::String, std::string::String>,
        /// The content type to use for the check.
        #[prost(enumeration = "http_check::ContentType", tag = "9")]
        pub content_type: i32,
        /// Boolean specifying whether to include SSL certificate validation as a
        /// part of the Uptime check. Only applies to checks where
        /// `monitored_resource` is set to `uptime_url`. If `use_ssl` is `false`,
        /// setting `validate_ssl` to `true` has no effect.
        #[prost(bool, tag = "7")]
        pub validate_ssl: bool,
        /// The request body associated with the HTTP request. If `content_type` is
        /// `URL_ENCODED`, the body passed in must be URL-encoded. Users can provide
        /// a `Content-Length` header via the `headers` field or the API will do
        /// so. The maximum byte size is 1 megabyte. Note: As with all `bytes` fields
        /// JSON representations are base64 encoded.
        #[prost(bytes, tag = "10")]
        pub body: std::vec::Vec<u8>,
    }
    pub mod http_check {
        /// The authentication parameters to provide to the specified resource or
        /// URL that requires a username and password. Currently, only
        /// [Basic HTTP authentication](https://tools.ietf.org/html/rfc7617) is
        /// supported in Uptime checks.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct BasicAuthentication {
            /// The username to use when authenticating with the HTTP server.
            #[prost(string, tag = "1")]
            pub username: std::string::String,
            /// The password to use when authenticating with the HTTP server.
            #[prost(string, tag = "2")]
            pub password: std::string::String,
        }
        /// The HTTP request method options.
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
        #[repr(i32)]
        pub enum RequestMethod {
            /// No request method specified.
            MethodUnspecified = 0,
            /// GET request.
            Get = 1,
            /// POST request.
            Post = 2,
        }
        /// Header options corresponding to the Content-Type of the body in HTTP
        /// requests. Note that a `Content-Type` header cannot be present in the
        /// `headers` field if this field is specified.
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
        #[repr(i32)]
        pub enum ContentType {
            /// No content type specified. If the request method is POST, an
            /// unspecified content type results in a check creation rejection.
            TypeUnspecified = 0,
            /// `body` is in URL-encoded form. Equivalent to setting the `Content-Type`
            /// to `application/x-www-form-urlencoded` in the HTTP request.
            UrlEncoded = 1,
        }
    }
    /// Information required for a TCP Uptime check request.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TcpCheck {
        /// The TCP port on the server against which to run the check. Will be
        /// combined with host (specified within the `monitored_resource`) to
        /// construct the full URL. Required.
        #[prost(int32, tag = "1")]
        pub port: i32,
    }
    /// Optional. Used to perform content matching. This allows matching based on
    /// substrings and regular expressions, together with their negations. Only the
    /// first 4&nbsp;MB of an HTTP or HTTPS check's response (and the first
    /// 1&nbsp;MB of a TCP check's response) are examined for purposes of content
    /// matching.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ContentMatcher {
        /// String or regex content to match. Maximum 1024 bytes. An empty `content`
        /// string indicates no content matching is to be performed.
        #[prost(string, tag = "1")]
        pub content: std::string::String,
        /// The type of content matcher that will be applied to the server output,
        /// compared to the `content` string when the check is run.
        #[prost(enumeration = "content_matcher::ContentMatcherOption", tag = "2")]
        pub matcher: i32,
    }
    pub mod content_matcher {
        /// Options to perform content matching.
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
        #[repr(i32)]
        pub enum ContentMatcherOption {
            /// No content matcher type specified (maintained for backward
            /// compatibility, but deprecated for future use).
            /// Treated as `CONTAINS_STRING`.
            Unspecified = 0,
            /// Selects substring matching. The match succeeds if the output contains
            /// the `content` string.  This is the default value for checks without
            /// a `matcher` option, or where the value of `matcher` is
            /// `CONTENT_MATCHER_OPTION_UNSPECIFIED`.
            ContainsString = 1,
            /// Selects negation of substring matching. The match succeeds if the
            /// output does _NOT_ contain the `content` string.
            NotContainsString = 2,
            /// Selects regular-expression matching. The match succeeds of the output
            /// matches the regular expression specified in the `content` string.
            MatchesRegex = 3,
            /// Selects negation of regular-expression matching. The match succeeds if
            /// the output does _NOT_ match the regular expression specified in the
            /// `content` string.
            NotMatchesRegex = 4,
        }
    }
    /// The resource the check is checking. Required.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Resource {
        /// The [monitored
        /// resource](https://cloud.google.com/monitoring/api/resources) associated
        /// with the configuration.
        /// The following monitored resource types are supported for Uptime checks:
        ///   `uptime_url`,
        ///   `gce_instance`,
        ///   `gae_app`,
        ///   `aws_ec2_instance`,
        ///   `aws_elb_load_balancer`
        #[prost(message, tag = "3")]
        MonitoredResource(super::super::super::api::MonitoredResource),
        /// The group resource associated with the configuration.
        #[prost(message, tag = "4")]
        ResourceGroup(ResourceGroup),
    }
    /// The type of Uptime check request.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum CheckRequestType {
        /// Contains information needed to make an HTTP or HTTPS check.
        #[prost(message, tag = "5")]
        HttpCheck(HttpCheck),
        /// Contains information needed to make a TCP check.
        #[prost(message, tag = "6")]
        TcpCheck(TcpCheck),
    }
}
/// Contains the region, location, and list of IP
/// addresses where checkers in the location run from.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UptimeCheckIp {
    /// A broad region category in which the IP address is located.
    #[prost(enumeration = "UptimeCheckRegion", tag = "1")]
    pub region: i32,
    /// A more specific location within the region that typically encodes
    /// a particular city/town/metro (and its containing state/province or country)
    /// within the broader umbrella region category.
    #[prost(string, tag = "2")]
    pub location: std::string::String,
    /// The IP address from which the Uptime check originates. This is a fully
    /// specified IP address (not an IP address range). Most IP addresses, as of
    /// this publication, are in IPv4 format; however, one should not rely on the
    /// IP addresses being in IPv4 format indefinitely, and should support
    /// interpreting this field in either IPv4 or IPv6 format.
    #[prost(string, tag = "3")]
    pub ip_address: std::string::String,
}
/// The regions from which an Uptime check can be run.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum UptimeCheckRegion {
    /// Default value if no region is specified. Will result in Uptime checks
    /// running from all regions.
    RegionUnspecified = 0,
    /// Allows checks to run from locations within the United States of America.
    Usa = 1,
    /// Allows checks to run from locations within the continent of Europe.
    Europe = 2,
    /// Allows checks to run from locations within the continent of South
    /// America.
    SouthAmerica = 3,
    /// Allows checks to run from locations within the Asia Pacific area (ex:
    /// Singapore).
    AsiaPacific = 4,
}
/// The supported resource types that can be used as values of
/// `group_resource.resource_type`.
/// `INSTANCE` includes `gce_instance` and `aws_ec2_instance` resource types.
/// The resource types `gae_app` and `uptime_url` are not valid here because
/// group checks on App Engine modules and URLs are not allowed.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum GroupResourceType {
    /// Default value (not valid).
    ResourceTypeUnspecified = 0,
    /// A group of instances from Google Cloud Platform (GCP) or
    /// Amazon Web Services (AWS).
    Instance = 1,
    /// A group of Amazon ELB load balancers.
    AwsElbLoadBalancer = 2,
}
/// The protocol for the `ListUptimeCheckConfigs` request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListUptimeCheckConfigsRequest {
    /// Required. The project whose Uptime check configurations are listed. The format is:
    ///
    ///     projects/[PROJECT_ID_OR_NUMBER]
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// The maximum number of results to return in a single response. The server
    /// may further constrain the maximum number of results returned in a single
    /// page. If the page_size is <=0, the server will decide the number of results
    /// to be returned.
    #[prost(int32, tag = "3")]
    pub page_size: i32,
    /// If this field is not empty then it must contain the `nextPageToken` value
    /// returned by a previous call to this method.  Using this field causes the
    /// method to return more results from the previous method call.
    #[prost(string, tag = "4")]
    pub page_token: std::string::String,
}
/// The protocol for the `ListUptimeCheckConfigs` response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListUptimeCheckConfigsResponse {
    /// The returned Uptime check configurations.
    #[prost(message, repeated, tag = "1")]
    pub uptime_check_configs: ::std::vec::Vec<UptimeCheckConfig>,
    /// This field represents the pagination token to retrieve the next page of
    /// results. If the value is empty, it means no further results for the
    /// request. To retrieve the next page of results, the value of the
    /// next_page_token is passed to the subsequent List method call (in the
    /// request message's page_token field).
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
    /// The total number of Uptime check configurations for the project,
    /// irrespective of any pagination.
    #[prost(int32, tag = "3")]
    pub total_size: i32,
}
/// The protocol for the `GetUptimeCheckConfig` request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetUptimeCheckConfigRequest {
    /// Required. The Uptime check configuration to retrieve. The format is:
    ///
    ///     projects/[PROJECT_ID_OR_NUMBER]/uptimeCheckConfigs/[UPTIME_CHECK_ID]
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// The protocol for the `CreateUptimeCheckConfig` request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateUptimeCheckConfigRequest {
    /// Required. The project in which to create the Uptime check. The format is:
    ///
    ///     projects/[PROJECT_ID_OR_NUMBER]
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Required. The new Uptime check configuration.
    #[prost(message, optional, tag = "2")]
    pub uptime_check_config: ::std::option::Option<UptimeCheckConfig>,
}
/// The protocol for the `UpdateUptimeCheckConfig` request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateUptimeCheckConfigRequest {
    /// Optional. If present, only the listed fields in the current Uptime check
    /// configuration are updated with values from the new configuration. If this
    /// field is empty, then the current configuration is completely replaced with
    /// the new configuration.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
    /// Required. If an `updateMask` has been specified, this field gives
    /// the values for the set of fields mentioned in the `updateMask`. If an
    /// `updateMask` has not been given, this Uptime check configuration replaces
    /// the current configuration. If a field is mentioned in `updateMask` but
    /// the corresonding field is omitted in this partial Uptime check
    /// configuration, it has the effect of deleting/clearing the field from the
    /// configuration on the server.
    ///
    /// The following fields can be updated: `display_name`,
    /// `http_check`, `tcp_check`, `timeout`, `content_matchers`, and
    /// `selected_regions`.
    #[prost(message, optional, tag = "3")]
    pub uptime_check_config: ::std::option::Option<UptimeCheckConfig>,
}
/// The protocol for the `DeleteUptimeCheckConfig` request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteUptimeCheckConfigRequest {
    /// Required. The Uptime check configuration to delete. The format is:
    ///
    ///     projects/[PROJECT_ID_OR_NUMBER]/uptimeCheckConfigs/[UPTIME_CHECK_ID]
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// The protocol for the `ListUptimeCheckIps` request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListUptimeCheckIpsRequest {
    /// The maximum number of results to return in a single response. The server
    /// may further constrain the maximum number of results returned in a single
    /// page. If the page_size is <=0, the server will decide the number of results
    /// to be returned.
    /// NOTE: this field is not yet implemented
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// If this field is not empty then it must contain the `nextPageToken` value
    /// returned by a previous call to this method.  Using this field causes the
    /// method to return more results from the previous method call.
    /// NOTE: this field is not yet implemented
    #[prost(string, tag = "3")]
    pub page_token: std::string::String,
}
/// The protocol for the `ListUptimeCheckIps` response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListUptimeCheckIpsResponse {
    /// The returned list of IP addresses (including region and location) that the
    /// checkers run from.
    #[prost(message, repeated, tag = "1")]
    pub uptime_check_ips: ::std::vec::Vec<UptimeCheckIp>,
    /// This field represents the pagination token to retrieve the next page of
    /// results. If the value is empty, it means no further results for the
    /// request. To retrieve the next page of results, the value of the
    /// next_page_token is passed to the subsequent List method call (in the
    /// request message's page_token field).
    /// NOTE: this field is not yet implemented
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
}
#[doc = r" Generated client implementations."]
pub mod uptime_check_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " The UptimeCheckService API is used to manage (list, create, delete, edit)"]
    #[doc = " Uptime check configurations in the Stackdriver Monitoring product. An Uptime"]
    #[doc = " check is a piece of configuration that determines which resources and"]
    #[doc = " services to monitor for availability. These configurations can also be"]
    #[doc = " configured interactively by navigating to the [Cloud Console]"]
    #[doc = " (http://console.cloud.google.com), selecting the appropriate project,"]
    #[doc = " clicking on \"Monitoring\" on the left-hand side to navigate to Stackdriver,"]
    #[doc = " and then clicking on \"Uptime\"."]
    pub struct UptimeCheckServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> UptimeCheckServiceClient<T>
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
        #[doc = " Lists the existing valid Uptime check configurations for the project"]
        #[doc = " (leaving out any invalid configurations)."]
        pub async fn list_uptime_check_configs(
            &mut self,
            request: impl tonic::IntoRequest<super::ListUptimeCheckConfigsRequest>,
        ) -> Result<tonic::Response<super::ListUptimeCheckConfigsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.monitoring.v3.UptimeCheckService/ListUptimeCheckConfigs",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets a single Uptime check configuration."]
        pub async fn get_uptime_check_config(
            &mut self,
            request: impl tonic::IntoRequest<super::GetUptimeCheckConfigRequest>,
        ) -> Result<tonic::Response<super::UptimeCheckConfig>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.monitoring.v3.UptimeCheckService/GetUptimeCheckConfig",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a new Uptime check configuration."]
        pub async fn create_uptime_check_config(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateUptimeCheckConfigRequest>,
        ) -> Result<tonic::Response<super::UptimeCheckConfig>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.monitoring.v3.UptimeCheckService/CreateUptimeCheckConfig",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates an Uptime check configuration. You can either replace the entire"]
        #[doc = " configuration with a new one or replace only certain fields in the current"]
        #[doc = " configuration by specifying the fields to be updated via `updateMask`."]
        #[doc = " Returns the updated configuration."]
        pub async fn update_uptime_check_config(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateUptimeCheckConfigRequest>,
        ) -> Result<tonic::Response<super::UptimeCheckConfig>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.monitoring.v3.UptimeCheckService/UpdateUptimeCheckConfig",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes an Uptime check configuration. Note that this method will fail"]
        #[doc = " if the Uptime check configuration is referenced by an alert policy or"]
        #[doc = " other dependent configs that would be rendered invalid by the deletion."]
        pub async fn delete_uptime_check_config(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteUptimeCheckConfigRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.monitoring.v3.UptimeCheckService/DeleteUptimeCheckConfig",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns the list of IP addresses that checkers run from"]
        pub async fn list_uptime_check_ips(
            &mut self,
            request: impl tonic::IntoRequest<super::ListUptimeCheckIpsRequest>,
        ) -> Result<tonic::Response<super::ListUptimeCheckIpsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.monitoring.v3.UptimeCheckService/ListUptimeCheckIps",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for UptimeCheckServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for UptimeCheckServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "UptimeCheckServiceClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod uptime_check_service_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with UptimeCheckServiceServer."]
    #[async_trait]
    pub trait UptimeCheckService: Send + Sync + 'static {
        #[doc = " Lists the existing valid Uptime check configurations for the project"]
        #[doc = " (leaving out any invalid configurations)."]
        async fn list_uptime_check_configs(
            &self,
            request: tonic::Request<super::ListUptimeCheckConfigsRequest>,
        ) -> Result<tonic::Response<super::ListUptimeCheckConfigsResponse>, tonic::Status>;
        #[doc = " Gets a single Uptime check configuration."]
        async fn get_uptime_check_config(
            &self,
            request: tonic::Request<super::GetUptimeCheckConfigRequest>,
        ) -> Result<tonic::Response<super::UptimeCheckConfig>, tonic::Status>;
        #[doc = " Creates a new Uptime check configuration."]
        async fn create_uptime_check_config(
            &self,
            request: tonic::Request<super::CreateUptimeCheckConfigRequest>,
        ) -> Result<tonic::Response<super::UptimeCheckConfig>, tonic::Status>;
        #[doc = " Updates an Uptime check configuration. You can either replace the entire"]
        #[doc = " configuration with a new one or replace only certain fields in the current"]
        #[doc = " configuration by specifying the fields to be updated via `updateMask`."]
        #[doc = " Returns the updated configuration."]
        async fn update_uptime_check_config(
            &self,
            request: tonic::Request<super::UpdateUptimeCheckConfigRequest>,
        ) -> Result<tonic::Response<super::UptimeCheckConfig>, tonic::Status>;
        #[doc = " Deletes an Uptime check configuration. Note that this method will fail"]
        #[doc = " if the Uptime check configuration is referenced by an alert policy or"]
        #[doc = " other dependent configs that would be rendered invalid by the deletion."]
        async fn delete_uptime_check_config(
            &self,
            request: tonic::Request<super::DeleteUptimeCheckConfigRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status>;
        #[doc = " Returns the list of IP addresses that checkers run from"]
        async fn list_uptime_check_ips(
            &self,
            request: tonic::Request<super::ListUptimeCheckIpsRequest>,
        ) -> Result<tonic::Response<super::ListUptimeCheckIpsResponse>, tonic::Status>;
    }
    #[doc = " The UptimeCheckService API is used to manage (list, create, delete, edit)"]
    #[doc = " Uptime check configurations in the Stackdriver Monitoring product. An Uptime"]
    #[doc = " check is a piece of configuration that determines which resources and"]
    #[doc = " services to monitor for availability. These configurations can also be"]
    #[doc = " configured interactively by navigating to the [Cloud Console]"]
    #[doc = " (http://console.cloud.google.com), selecting the appropriate project,"]
    #[doc = " clicking on \"Monitoring\" on the left-hand side to navigate to Stackdriver,"]
    #[doc = " and then clicking on \"Uptime\"."]
    #[derive(Debug)]
    #[doc(hidden)]
    pub struct UptimeCheckServiceServer<T: UptimeCheckService> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: UptimeCheckService> UptimeCheckServiceServer<T> {
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
    impl<T, B> Service<http::Request<B>> for UptimeCheckServiceServer<T>
    where
        T: UptimeCheckService,
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
                "/google.monitoring.v3.UptimeCheckService/ListUptimeCheckConfigs" => {
                    #[allow(non_camel_case_types)]
                    struct ListUptimeCheckConfigsSvc<T: UptimeCheckService>(pub Arc<T>);
                    impl<T: UptimeCheckService>
                        tonic::server::UnaryService<super::ListUptimeCheckConfigsRequest>
                        for ListUptimeCheckConfigsSvc<T>
                    {
                        type Response = super::ListUptimeCheckConfigsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListUptimeCheckConfigsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.list_uptime_check_configs(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ListUptimeCheckConfigsSvc(inner);
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
                "/google.monitoring.v3.UptimeCheckService/GetUptimeCheckConfig" => {
                    #[allow(non_camel_case_types)]
                    struct GetUptimeCheckConfigSvc<T: UptimeCheckService>(pub Arc<T>);
                    impl<T: UptimeCheckService>
                        tonic::server::UnaryService<super::GetUptimeCheckConfigRequest>
                        for GetUptimeCheckConfigSvc<T>
                    {
                        type Response = super::UptimeCheckConfig;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetUptimeCheckConfigRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.get_uptime_check_config(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetUptimeCheckConfigSvc(inner);
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
                "/google.monitoring.v3.UptimeCheckService/CreateUptimeCheckConfig" => {
                    #[allow(non_camel_case_types)]
                    struct CreateUptimeCheckConfigSvc<T: UptimeCheckService>(pub Arc<T>);
                    impl<T: UptimeCheckService>
                        tonic::server::UnaryService<super::CreateUptimeCheckConfigRequest>
                        for CreateUptimeCheckConfigSvc<T>
                    {
                        type Response = super::UptimeCheckConfig;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateUptimeCheckConfigRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { inner.create_uptime_check_config(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = CreateUptimeCheckConfigSvc(inner);
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
                "/google.monitoring.v3.UptimeCheckService/UpdateUptimeCheckConfig" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateUptimeCheckConfigSvc<T: UptimeCheckService>(pub Arc<T>);
                    impl<T: UptimeCheckService>
                        tonic::server::UnaryService<super::UpdateUptimeCheckConfigRequest>
                        for UpdateUptimeCheckConfigSvc<T>
                    {
                        type Response = super::UptimeCheckConfig;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateUptimeCheckConfigRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { inner.update_uptime_check_config(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = UpdateUptimeCheckConfigSvc(inner);
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
                "/google.monitoring.v3.UptimeCheckService/DeleteUptimeCheckConfig" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteUptimeCheckConfigSvc<T: UptimeCheckService>(pub Arc<T>);
                    impl<T: UptimeCheckService>
                        tonic::server::UnaryService<super::DeleteUptimeCheckConfigRequest>
                        for DeleteUptimeCheckConfigSvc<T>
                    {
                        type Response = ();
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteUptimeCheckConfigRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { inner.delete_uptime_check_config(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = DeleteUptimeCheckConfigSvc(inner);
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
                "/google.monitoring.v3.UptimeCheckService/ListUptimeCheckIps" => {
                    #[allow(non_camel_case_types)]
                    struct ListUptimeCheckIpsSvc<T: UptimeCheckService>(pub Arc<T>);
                    impl<T: UptimeCheckService>
                        tonic::server::UnaryService<super::ListUptimeCheckIpsRequest>
                        for ListUptimeCheckIpsSvc<T>
                    {
                        type Response = super::ListUptimeCheckIpsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListUptimeCheckIpsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.list_uptime_check_ips(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ListUptimeCheckIpsSvc(inner);
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
    impl<T: UptimeCheckService> Clone for UptimeCheckServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: UptimeCheckService> Clone for _Inner<T> {
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
