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
/// Describes a ranking-based time series filter. Each input time series is
/// ranked with an aligner. The filter will allow up to `num_time_series` time
/// series to pass through it, selecting them based on the relative ranking.
///
/// For example, if `ranking_method` is `METHOD_MEAN`,`direction` is `BOTTOM`,
/// and `num_time_series` is 3, then the 3 times series with the lowest mean
/// values will pass through the filter.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PickTimeSeriesFilter {
    /// `ranking_method` is applied to each time series independently to produce
    /// the value which will be used to compare the time series to other time
    /// series.
    #[prost(enumeration = "pick_time_series_filter::Method", tag = "1")]
    pub ranking_method: i32,
    /// How many time series to allow to pass through the filter.
    #[prost(int32, tag = "2")]
    pub num_time_series: i32,
    /// How to use the ranking to select time series that pass through the filter.
    #[prost(enumeration = "pick_time_series_filter::Direction", tag = "3")]
    pub direction: i32,
}
pub mod pick_time_series_filter {
    /// The value reducers that can be applied to a `PickTimeSeriesFilter`.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Method {
        /// Not allowed. You must specify a different `Method` if you specify a
        /// `PickTimeSeriesFilter`.
        Unspecified = 0,
        /// Select the mean of all values.
        Mean = 1,
        /// Select the maximum value.
        Max = 2,
        /// Select the minimum value.
        Min = 3,
        /// Compute the sum of all values.
        Sum = 4,
        /// Select the most recent value.
        Latest = 5,
    }
    /// Describes the ranking directions.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Direction {
        /// Not allowed. You must specify a different `Direction` if you specify a
        /// `PickTimeSeriesFilter`.
        Unspecified = 0,
        /// Pass the highest `num_time_series` ranking inputs.
        Top = 1,
        /// Pass the lowest `num_time_series` ranking inputs.
        Bottom = 2,
    }
}
/// A filter that ranks streams based on their statistical relation to other
/// streams in a request.
/// Note: This field is deprecated and completely ignored by the API.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StatisticalTimeSeriesFilter {
    /// `rankingMethod` is applied to a set of time series, and then the produced
    /// value for each individual time series is used to compare a given time
    /// series to others.
    /// These are methods that cannot be applied stream-by-stream, but rather
    /// require the full context of a request to evaluate time series.
    #[prost(enumeration = "statistical_time_series_filter::Method", tag = "1")]
    pub ranking_method: i32,
    /// How many time series to output.
    #[prost(int32, tag = "2")]
    pub num_time_series: i32,
}
pub mod statistical_time_series_filter {
    /// The filter methods that can be applied to a stream.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Method {
        /// Not allowed in well-formed requests.
        Unspecified = 0,
        /// Compute the outlier score of each stream.
        ClusterOutlier = 1,
    }
}
/// TimeSeriesQuery collects the set of supported methods for querying time
/// series data from the Stackdriver metrics API.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TimeSeriesQuery {
    /// The unit of data contained in fetched time series. If non-empty, this
    /// unit will override any unit that accompanies fetched data. The format is
    /// the same as the
    /// [`unit`](https://cloud.google.com/monitoring/api/ref_v3/rest/v3/projects.metricDescriptors)
    /// field in `MetricDescriptor`.
    #[prost(string, tag = "5")]
    pub unit_override: std::string::String,
    /// Parameters needed to obtain data for the chart.
    #[prost(oneof = "time_series_query::Source", tags = "1, 2, 3")]
    pub source: ::std::option::Option<time_series_query::Source>,
}
pub mod time_series_query {
    /// Parameters needed to obtain data for the chart.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Source {
        /// Filter parameters to fetch time series.
        #[prost(message, tag = "1")]
        TimeSeriesFilter(super::TimeSeriesFilter),
        /// Parameters to fetch a ratio between two time series filters.
        #[prost(message, tag = "2")]
        TimeSeriesFilterRatio(super::TimeSeriesFilterRatio),
        /// A query used to fetch time series.
        #[prost(string, tag = "3")]
        TimeSeriesQueryLanguage(std::string::String),
    }
}
/// A filter that defines a subset of time series data that is displayed in a
/// widget. Time series data is fetched using the
/// [`ListTimeSeries`](https://cloud.google.com/monitoring/api/ref_v3/rest/v3/projects.timeSeries/list)
/// method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TimeSeriesFilter {
    /// Required. The [monitoring filter](https://cloud.google.com/monitoring/api/v3/filters)
    /// that identifies the metric types, resources, and projects to query.
    #[prost(string, tag = "1")]
    pub filter: std::string::String,
    /// By default, the raw time series data is returned.
    /// Use this field to combine multiple time series for different views of the
    /// data.
    #[prost(message, optional, tag = "2")]
    pub aggregation: ::std::option::Option<Aggregation>,
    /// Apply a second aggregation after `aggregation` is applied.
    #[prost(message, optional, tag = "3")]
    pub secondary_aggregation: ::std::option::Option<Aggregation>,
    /// Selects an optional time series filter.
    #[prost(oneof = "time_series_filter::OutputFilter", tags = "4, 5")]
    pub output_filter: ::std::option::Option<time_series_filter::OutputFilter>,
}
pub mod time_series_filter {
    /// Selects an optional time series filter.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum OutputFilter {
        /// Ranking based time series filter.
        #[prost(message, tag = "4")]
        PickTimeSeriesFilter(super::PickTimeSeriesFilter),
        /// Statistics based time series filter.
        /// Note: This field is deprecated and completely ignored by the API.
        #[prost(message, tag = "5")]
        StatisticalTimeSeriesFilter(super::StatisticalTimeSeriesFilter),
    }
}
/// A pair of time series filters that define a ratio computation. The output
/// time series is the pair-wise division of each aligned element from the
/// numerator and denominator time series.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TimeSeriesFilterRatio {
    /// The numerator of the ratio.
    #[prost(message, optional, tag = "1")]
    pub numerator: ::std::option::Option<time_series_filter_ratio::RatioPart>,
    /// The denominator of the ratio.
    #[prost(message, optional, tag = "2")]
    pub denominator: ::std::option::Option<time_series_filter_ratio::RatioPart>,
    /// Apply a second aggregation after the ratio is computed.
    #[prost(message, optional, tag = "3")]
    pub secondary_aggregation: ::std::option::Option<Aggregation>,
    /// Selects an optional filter that is applied to the time series after
    /// computing the ratio.
    #[prost(oneof = "time_series_filter_ratio::OutputFilter", tags = "4, 5")]
    pub output_filter: ::std::option::Option<time_series_filter_ratio::OutputFilter>,
}
pub mod time_series_filter_ratio {
    /// Describes a query to build the numerator or denominator of a
    /// TimeSeriesFilterRatio.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct RatioPart {
        /// Required. The [monitoring
        /// filter](https://cloud.google.com/monitoring/api/v3/filters) that
        /// identifies the metric types, resources, and projects to query.
        #[prost(string, tag = "1")]
        pub filter: std::string::String,
        /// By default, the raw time series data is returned.
        /// Use this field to combine multiple time series for different views of the
        /// data.
        #[prost(message, optional, tag = "2")]
        pub aggregation: ::std::option::Option<super::Aggregation>,
    }
    /// Selects an optional filter that is applied to the time series after
    /// computing the ratio.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum OutputFilter {
        /// Ranking based time series filter.
        #[prost(message, tag = "4")]
        PickTimeSeriesFilter(super::PickTimeSeriesFilter),
        /// Statistics based time series filter.
        /// Note: This field is deprecated and completely ignored by the API.
        #[prost(message, tag = "5")]
        StatisticalTimeSeriesFilter(super::StatisticalTimeSeriesFilter),
    }
}
/// Defines a threshold for categorizing time series values.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Threshold {
    /// A label for the threshold.
    #[prost(string, tag = "1")]
    pub label: std::string::String,
    /// The value of the threshold. The value should be defined in the native scale
    /// of the metric.
    #[prost(double, tag = "2")]
    pub value: f64,
    /// The state color for this threshold. Color is not allowed in a XyChart.
    #[prost(enumeration = "threshold::Color", tag = "3")]
    pub color: i32,
    /// The direction for the current threshold. Direction is not allowed in a
    /// XyChart.
    #[prost(enumeration = "threshold::Direction", tag = "4")]
    pub direction: i32,
}
pub mod threshold {
    /// The color suggests an interpretation to the viewer when actual values cross
    /// the threshold. Comments on each color provide UX guidance on how users can
    /// be expected to interpret a given state color.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Color {
        /// Color is unspecified. Not allowed in well-formed requests.
        Unspecified = 0,
        /// Crossing the threshold is "concerning" behavior.
        Yellow = 4,
        /// Crossing the threshold is "emergency" behavior.
        Red = 6,
    }
    /// Whether the threshold is considered crossed by an actual value above or
    /// below its threshold value.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Direction {
        /// Not allowed in well-formed requests.
        Unspecified = 0,
        /// The threshold will be considered crossed if the actual value is above
        /// the threshold value.
        Above = 1,
        /// The threshold will be considered crossed if the actual value is below
        /// the threshold value.
        Below = 2,
    }
}
/// Defines the possible types of spark chart supported by the `Scorecard`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SparkChartType {
    /// Not allowed in well-formed requests.
    Unspecified = 0,
    /// The sparkline will be rendered as a small line chart.
    SparkLine = 1,
    /// The sparkbar will be rendered as a small bar chart.
    SparkBar = 2,
}
/// A widget showing the latest value of a metric, and how this value relates to
/// one or more thresholds.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Scorecard {
    /// Required. Fields for querying time series data from the
    /// Stackdriver metrics API.
    #[prost(message, optional, tag = "1")]
    pub time_series_query: ::std::option::Option<TimeSeriesQuery>,
    /// The thresholds used to determine the state of the scorecard given the
    /// time series' current value. For an actual value x, the scorecard is in a
    /// danger state if x is less than or equal to a danger threshold that triggers
    /// below, or greater than or equal to a danger threshold that triggers above.
    /// Similarly, if x is above/below a warning threshold that triggers
    /// above/below, then the scorecard is in a warning state - unless x also puts
    /// it in a danger state. (Danger trumps warning.)
    ///
    /// As an example, consider a scorecard with the following four thresholds:
    /// {
    ///   value: 90,
    ///   category: 'DANGER',
    ///   trigger: 'ABOVE',
    /// },
    /// {
    ///   value: 70,
    ///   category: 'WARNING',
    ///   trigger: 'ABOVE',
    /// },
    /// {
    ///   value: 10,
    ///   category: 'DANGER',
    ///   trigger: 'BELOW',
    /// },
    /// {
    ///   value: 20,
    ///   category: 'WARNING',
    ///   trigger: 'BELOW',
    /// }
    ///
    /// Then: values less than or equal to 10 would put the scorecard in a DANGER
    /// state, values greater than 10 but less than or equal to 20 a WARNING state,
    /// values strictly between 20 and 70 an OK state, values greater than or equal
    /// to 70 but less than 90 a WARNING state, and values greater than or equal to
    /// 90 a DANGER state.
    #[prost(message, repeated, tag = "6")]
    pub thresholds: ::std::vec::Vec<Threshold>,
    /// Defines the optional additional chart shown on the scorecard. If
    /// neither is included - then a default scorecard is shown.
    #[prost(oneof = "scorecard::DataView", tags = "4, 5")]
    pub data_view: ::std::option::Option<scorecard::DataView>,
}
pub mod scorecard {
    /// A gauge chart shows where the current value sits within a pre-defined
    /// range. The upper and lower bounds should define the possible range of
    /// values for the scorecard's query (inclusive).
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GaugeView {
        /// The lower bound for this gauge chart. The value of the chart should
        /// always be greater than or equal to this.
        #[prost(double, tag = "1")]
        pub lower_bound: f64,
        /// The upper bound for this gauge chart. The value of the chart should
        /// always be less than or equal to this.
        #[prost(double, tag = "2")]
        pub upper_bound: f64,
    }
    /// A sparkChart is a small chart suitable for inclusion in a table-cell or
    /// inline in text. This message contains the configuration for a sparkChart
    /// to show up on a Scorecard, showing recent trends of the scorecard's
    /// timeseries.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SparkChartView {
        /// Required. The type of sparkchart to show in this chartView.
        #[prost(enumeration = "super::SparkChartType", tag = "1")]
        pub spark_chart_type: i32,
        /// The lower bound on data point frequency in the chart implemented by
        /// specifying the minimum alignment period to use in a time series query.
        /// For example, if the data is published once every 10 minutes it would not
        /// make sense to fetch and align data at one minute intervals. This field is
        /// optional and exists only as a hint.
        #[prost(message, optional, tag = "2")]
        pub min_alignment_period: ::std::option::Option<::prost_types::Duration>,
    }
    /// Defines the optional additional chart shown on the scorecard. If
    /// neither is included - then a default scorecard is shown.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum DataView {
        /// Will cause the scorecard to show a gauge chart.
        #[prost(message, tag = "4")]
        GaugeView(GaugeView),
        /// Will cause the scorecard to show a spark chart.
        #[prost(message, tag = "5")]
        SparkChartView(SparkChartView),
    }
}
/// A widget that displays textual content.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Text {
    /// The text content to be displayed.
    #[prost(string, tag = "1")]
    pub content: std::string::String,
    /// How the text content is formatted.
    #[prost(enumeration = "text::Format", tag = "2")]
    pub format: i32,
}
pub mod text {
    /// The format type of the text content.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Format {
        /// Format is unspecified. Defaults to MARKDOWN.
        Unspecified = 0,
        /// The text contains Markdown formatting.
        Markdown = 1,
        /// The text contains no special formatting.
        Raw = 2,
    }
}
/// A chart that displays data on a 2D (X and Y axes) plane.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct XyChart {
    /// Required. The data displayed in this chart.
    #[prost(message, repeated, tag = "1")]
    pub data_sets: ::std::vec::Vec<xy_chart::DataSet>,
    /// The duration used to display a comparison chart. A comparison chart
    /// simultaneously shows values from two similar-length time periods
    /// (e.g., week-over-week metrics).
    /// The duration must be positive, and it can only be applied to charts with
    /// data sets of LINE plot type.
    #[prost(message, optional, tag = "4")]
    pub timeshift_duration: ::std::option::Option<::prost_types::Duration>,
    /// Threshold lines drawn horizontally across the chart.
    #[prost(message, repeated, tag = "5")]
    pub thresholds: ::std::vec::Vec<Threshold>,
    /// The properties applied to the X axis.
    #[prost(message, optional, tag = "6")]
    pub x_axis: ::std::option::Option<xy_chart::Axis>,
    /// The properties applied to the Y axis.
    #[prost(message, optional, tag = "7")]
    pub y_axis: ::std::option::Option<xy_chart::Axis>,
    /// Display options for the chart.
    #[prost(message, optional, tag = "8")]
    pub chart_options: ::std::option::Option<ChartOptions>,
}
pub mod xy_chart {
    /// Groups a time series query definition with charting options.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DataSet {
        /// Required. Fields for querying time series data from the
        /// Stackdriver metrics API.
        #[prost(message, optional, tag = "1")]
        pub time_series_query: ::std::option::Option<super::TimeSeriesQuery>,
        /// How this data should be plotted on the chart.
        #[prost(enumeration = "data_set::PlotType", tag = "2")]
        pub plot_type: i32,
        /// A template string for naming `TimeSeries` in the resulting data set.
        /// This should be a string with interpolations of the form `${label_name}`,
        /// which will resolve to the label's value.
        #[prost(string, tag = "3")]
        pub legend_template: std::string::String,
        /// Optional. The lower bound on data point frequency for this data set, implemented by
        /// specifying the minimum alignment period to use in a time series query
        /// For example, if the data is published once every 10 minutes, the
        /// `min_alignment_period` should be at least 10 minutes. It would not
        /// make sense to fetch and align data at one minute intervals.
        #[prost(message, optional, tag = "4")]
        pub min_alignment_period: ::std::option::Option<::prost_types::Duration>,
    }
    pub mod data_set {
        /// The types of plotting strategies for data sets.
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
        #[repr(i32)]
        pub enum PlotType {
            /// Plot type is unspecified. The view will default to `LINE`.
            Unspecified = 0,
            /// The data is plotted as a set of lines (one line per series).
            Line = 1,
            /// The data is plotted as a set of filled areas (one area per series),
            /// with the areas stacked vertically (the base of each area is the top of
            /// its predecessor, and the base of the first area is the X axis). Since
            /// the areas do not overlap, each is filled with a different opaque color.
            StackedArea = 2,
            /// The data is plotted as a set of rectangular boxes (one box per series),
            /// with the boxes stacked vertically (the base of each box is the top of
            /// its predecessor, and the base of the first box is the X axis). Since
            /// the boxes do not overlap, each is filled with a different opaque color.
            StackedBar = 3,
            /// The data is plotted as a heatmap. The series being plotted must have a
            /// `DISTRIBUTION` value type. The value of each bucket in the distribution
            /// is displayed as a color. This type is not currently available in the
            /// Stackdriver Monitoring application.
            Heatmap = 4,
        }
    }
    /// A chart axis.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Axis {
        /// The label of the axis.
        #[prost(string, tag = "1")]
        pub label: std::string::String,
        /// The axis scale. By default, a linear scale is used.
        #[prost(enumeration = "axis::Scale", tag = "2")]
        pub scale: i32,
    }
    pub mod axis {
        /// Types of scales used in axes.
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
        #[repr(i32)]
        pub enum Scale {
            /// Scale is unspecified. The view will default to `LINEAR`.
            Unspecified = 0,
            /// Linear scale.
            Linear = 1,
            /// Logarithmic scale (base 10).
            Log10 = 2,
        }
    }
}
/// Options to control visual rendering of a chart.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChartOptions {
    /// The chart mode.
    #[prost(enumeration = "chart_options::Mode", tag = "1")]
    pub mode: i32,
}
pub mod chart_options {
    /// Chart mode options.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Mode {
        /// Mode is unspecified. The view will default to `COLOR`.
        Unspecified = 0,
        /// The chart distinguishes data series using different color. Line
        /// colors may get reused when there are many lines in the chart.
        Color = 1,
        /// The chart uses the Stackdriver x-ray mode, in which each
        /// data set is plotted using the same semi-transparent color.
        XRay = 2,
        /// The chart displays statistics such as average, median, 95th percentile,
        /// and more.
        Stats = 3,
    }
}
/// Widget contains a single dashboard component and configuration of how to
/// present the component in the dashboard.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Widget {
    /// Optional. The title of the widget.
    #[prost(string, tag = "1")]
    pub title: std::string::String,
    /// Content defines the component used to populate the widget.
    #[prost(oneof = "widget::Content", tags = "2, 3, 4, 5")]
    pub content: ::std::option::Option<widget::Content>,
}
pub mod widget {
    /// Content defines the component used to populate the widget.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Content {
        /// A chart of time series data.
        #[prost(message, tag = "2")]
        XyChart(super::XyChart),
        /// A scorecard summarizing time series data.
        #[prost(message, tag = "3")]
        Scorecard(super::Scorecard),
        /// A raw string or markdown displaying textual content.
        #[prost(message, tag = "4")]
        Text(super::Text),
        /// A blank space.
        #[prost(message, tag = "5")]
        Blank(()),
    }
}
/// A basic layout divides the available space into vertical columns of equal
/// width and arranges a list of widgets using a row-first strategy.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GridLayout {
    /// The number of columns into which the view's width is divided. If omitted
    /// or set to zero, a system default will be used while rendering.
    #[prost(int64, tag = "1")]
    pub columns: i64,
    /// The informational elements that are arranged into the columns row-first.
    #[prost(message, repeated, tag = "2")]
    pub widgets: ::std::vec::Vec<Widget>,
}
/// A simplified layout that divides the available space into rows
/// and arranges a set of widgets horizontally in each row.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RowLayout {
    /// The rows of content to display.
    #[prost(message, repeated, tag = "1")]
    pub rows: ::std::vec::Vec<row_layout::Row>,
}
pub mod row_layout {
    /// Defines the layout properties and content for a row.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Row {
        /// The relative weight of this row. The row weight is used to adjust the
        /// height of rows on the screen (relative to peers). Greater the weight,
        /// greater the height of the row on the screen. If omitted, a value
        /// of 1 is used while rendering.
        #[prost(int64, tag = "1")]
        pub weight: i64,
        /// The display widgets arranged horizontally in this row.
        #[prost(message, repeated, tag = "2")]
        pub widgets: ::std::vec::Vec<super::Widget>,
    }
}
/// A simplified layout that divides the available space into vertical columns
/// and arranges a set of widgets vertically in each column.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ColumnLayout {
    /// The columns of content to display.
    #[prost(message, repeated, tag = "1")]
    pub columns: ::std::vec::Vec<column_layout::Column>,
}
pub mod column_layout {
    /// Defines the layout properties and content for a column.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Column {
        /// The relative weight of this column. The column weight is used to adjust
        /// the width of columns on the screen (relative to peers).
        /// Greater the weight, greater the width of the column on the screen.
        /// If omitted, a value of 1 is used while rendering.
        #[prost(int64, tag = "1")]
        pub weight: i64,
        /// The display widgets arranged vertically in this column.
        #[prost(message, repeated, tag = "2")]
        pub widgets: ::std::vec::Vec<super::Widget>,
    }
}
/// A Google Stackdriver dashboard. Dashboards define the content and layout
/// of pages in the Stackdriver web application.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Dashboard {
    /// Immutable. The resource name of the dashboard.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Required. The mutable, human-readable name.
    #[prost(string, tag = "2")]
    pub display_name: std::string::String,
    /// `etag` is used for optimistic concurrency control as a way to help
    /// prevent simultaneous updates of a policy from overwriting each other.
    /// An `etag` is returned in the response to `GetDashboard`, and
    /// users are expected to put that etag in the request to `UpdateDashboard` to
    /// ensure that their change will be applied to the same version of the
    /// Dashboard configuration. The field should not be passed during
    /// dashboard creation.
    #[prost(string, tag = "4")]
    pub etag: std::string::String,
    /// A dashboard's root container element that defines the layout style.
    #[prost(oneof = "dashboard::Layout", tags = "5, 8, 9")]
    pub layout: ::std::option::Option<dashboard::Layout>,
}
pub mod dashboard {
    /// A dashboard's root container element that defines the layout style.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Layout {
        /// Content is arranged with a basic layout that re-flows a simple list of
        /// informational elements like widgets or tiles.
        #[prost(message, tag = "5")]
        GridLayout(super::GridLayout),
        /// The content is divided into equally spaced rows and the widgets are
        /// arranged horizontally.
        #[prost(message, tag = "8")]
        RowLayout(super::RowLayout),
        /// The content is divided into equally spaced columns and the widgets are
        /// arranged vertically.
        #[prost(message, tag = "9")]
        ColumnLayout(super::ColumnLayout),
    }
}
/// The `CreateDashboard` request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateDashboardRequest {
    /// Required. The project on which to execute the request. The format is:
    ///
    ///     projects/[PROJECT_ID_OR_NUMBER]
    ///
    /// The `[PROJECT_ID_OR_NUMBER]` must match the dashboard resource name.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Required. The initial dashboard specification.
    #[prost(message, optional, tag = "2")]
    pub dashboard: ::std::option::Option<Dashboard>,
}
/// The `ListDashboards` request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDashboardsRequest {
    /// Required. The scope of the dashboards to list. The format is:
    ///
    ///     projects/[PROJECT_ID_OR_NUMBER]
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// A positive number that is the maximum number of results to return.
    /// If unspecified, a default of 1000 is used.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// If this field is not empty then it must contain the `nextPageToken` value
    /// returned by a previous call to this method.  Using this field causes the
    /// method to return additional results from the previous method call.
    #[prost(string, tag = "3")]
    pub page_token: std::string::String,
}
/// The `ListDashboards` request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDashboardsResponse {
    /// The list of requested dashboards.
    #[prost(message, repeated, tag = "1")]
    pub dashboards: ::std::vec::Vec<Dashboard>,
    /// If there are more results than have been returned, then this field is set
    /// to a non-empty value.  To see the additional results,
    /// use that value as `page_token` in the next call to this method.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
}
/// The `GetDashboard` request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDashboardRequest {
    /// Required. The resource name of the Dashboard. The format is one of:
    ///
    ///  -  `dashboards/[DASHBOARD_ID]` (for system dashboards)
    ///  -  `projects/[PROJECT_ID_OR_NUMBER]/dashboards/[DASHBOARD_ID]`
    ///       (for custom dashboards).
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// The `DeleteDashboard` request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteDashboardRequest {
    /// Required. The resource name of the Dashboard. The format is:
    ///
    ///     projects/[PROJECT_ID_OR_NUMBER]/dashboards/[DASHBOARD_ID]
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// The `UpdateDashboard` request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateDashboardRequest {
    /// Required. The dashboard that will replace the existing dashboard.
    #[prost(message, optional, tag = "1")]
    pub dashboard: ::std::option::Option<Dashboard>,
}
#[doc = r" Generated client implementations."]
pub mod dashboards_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Manages Stackdriver dashboards. A dashboard is an arrangement of data display"]
    #[doc = " widgets in a specific layout."]
    pub struct DashboardsServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> DashboardsServiceClient<T>
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
        #[doc = " Creates a new custom dashboard."]
        #[doc = ""]
        #[doc = " This method requires the `monitoring.dashboards.create` permission"]
        #[doc = " on the specified project. For more information, see"]
        #[doc = " [Google Cloud IAM](https://cloud.google.com/iam)."]
        pub async fn create_dashboard(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateDashboardRequest>,
        ) -> Result<tonic::Response<super::Dashboard>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.monitoring.dashboard.v1.DashboardsService/CreateDashboard",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists the existing dashboards."]
        #[doc = ""]
        #[doc = " This method requires the `monitoring.dashboards.list` permission"]
        #[doc = " on the specified project. For more information, see"]
        #[doc = " [Google Cloud IAM](https://cloud.google.com/iam)."]
        pub async fn list_dashboards(
            &mut self,
            request: impl tonic::IntoRequest<super::ListDashboardsRequest>,
        ) -> Result<tonic::Response<super::ListDashboardsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.monitoring.dashboard.v1.DashboardsService/ListDashboards",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Fetches a specific dashboard."]
        #[doc = ""]
        #[doc = " This method requires the `monitoring.dashboards.get` permission"]
        #[doc = " on the specified dashboard. For more information, see"]
        #[doc = " [Google Cloud IAM](https://cloud.google.com/iam)."]
        pub async fn get_dashboard(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDashboardRequest>,
        ) -> Result<tonic::Response<super::Dashboard>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.monitoring.dashboard.v1.DashboardsService/GetDashboard",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes an existing custom dashboard."]
        #[doc = ""]
        #[doc = " This method requires the `monitoring.dashboards.delete` permission"]
        #[doc = " on the specified dashboard. For more information, see"]
        #[doc = " [Google Cloud IAM](https://cloud.google.com/iam)."]
        pub async fn delete_dashboard(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteDashboardRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.monitoring.dashboard.v1.DashboardsService/DeleteDashboard",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Replaces an existing custom dashboard with a new definition."]
        #[doc = ""]
        #[doc = " This method requires the `monitoring.dashboards.update` permission"]
        #[doc = " on the specified dashboard. For more information, see"]
        #[doc = " [Google Cloud IAM](https://cloud.google.com/iam)."]
        pub async fn update_dashboard(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateDashboardRequest>,
        ) -> Result<tonic::Response<super::Dashboard>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.monitoring.dashboard.v1.DashboardsService/UpdateDashboard",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for DashboardsServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for DashboardsServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "DashboardsServiceClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod dashboards_service_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with DashboardsServiceServer."]
    #[async_trait]
    pub trait DashboardsService: Send + Sync + 'static {
        #[doc = " Creates a new custom dashboard."]
        #[doc = ""]
        #[doc = " This method requires the `monitoring.dashboards.create` permission"]
        #[doc = " on the specified project. For more information, see"]
        #[doc = " [Google Cloud IAM](https://cloud.google.com/iam)."]
        async fn create_dashboard(
            &self,
            request: tonic::Request<super::CreateDashboardRequest>,
        ) -> Result<tonic::Response<super::Dashboard>, tonic::Status>;
        #[doc = " Lists the existing dashboards."]
        #[doc = ""]
        #[doc = " This method requires the `monitoring.dashboards.list` permission"]
        #[doc = " on the specified project. For more information, see"]
        #[doc = " [Google Cloud IAM](https://cloud.google.com/iam)."]
        async fn list_dashboards(
            &self,
            request: tonic::Request<super::ListDashboardsRequest>,
        ) -> Result<tonic::Response<super::ListDashboardsResponse>, tonic::Status>;
        #[doc = " Fetches a specific dashboard."]
        #[doc = ""]
        #[doc = " This method requires the `monitoring.dashboards.get` permission"]
        #[doc = " on the specified dashboard. For more information, see"]
        #[doc = " [Google Cloud IAM](https://cloud.google.com/iam)."]
        async fn get_dashboard(
            &self,
            request: tonic::Request<super::GetDashboardRequest>,
        ) -> Result<tonic::Response<super::Dashboard>, tonic::Status>;
        #[doc = " Deletes an existing custom dashboard."]
        #[doc = ""]
        #[doc = " This method requires the `monitoring.dashboards.delete` permission"]
        #[doc = " on the specified dashboard. For more information, see"]
        #[doc = " [Google Cloud IAM](https://cloud.google.com/iam)."]
        async fn delete_dashboard(
            &self,
            request: tonic::Request<super::DeleteDashboardRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status>;
        #[doc = " Replaces an existing custom dashboard with a new definition."]
        #[doc = ""]
        #[doc = " This method requires the `monitoring.dashboards.update` permission"]
        #[doc = " on the specified dashboard. For more information, see"]
        #[doc = " [Google Cloud IAM](https://cloud.google.com/iam)."]
        async fn update_dashboard(
            &self,
            request: tonic::Request<super::UpdateDashboardRequest>,
        ) -> Result<tonic::Response<super::Dashboard>, tonic::Status>;
    }
    #[doc = " Manages Stackdriver dashboards. A dashboard is an arrangement of data display"]
    #[doc = " widgets in a specific layout."]
    #[derive(Debug)]
    #[doc(hidden)]
    pub struct DashboardsServiceServer<T: DashboardsService> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: DashboardsService> DashboardsServiceServer<T> {
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
    impl<T, B> Service<http::Request<B>> for DashboardsServiceServer<T>
    where
        T: DashboardsService,
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
                "/google.monitoring.dashboard.v1.DashboardsService/CreateDashboard" => {
                    #[allow(non_camel_case_types)]
                    struct CreateDashboardSvc<T: DashboardsService>(pub Arc<T>);
                    impl<T: DashboardsService>
                        tonic::server::UnaryService<super::CreateDashboardRequest>
                        for CreateDashboardSvc<T>
                    {
                        type Response = super::Dashboard;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateDashboardRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.create_dashboard(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = CreateDashboardSvc(inner);
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
                "/google.monitoring.dashboard.v1.DashboardsService/ListDashboards" => {
                    #[allow(non_camel_case_types)]
                    struct ListDashboardsSvc<T: DashboardsService>(pub Arc<T>);
                    impl<T: DashboardsService>
                        tonic::server::UnaryService<super::ListDashboardsRequest>
                        for ListDashboardsSvc<T>
                    {
                        type Response = super::ListDashboardsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListDashboardsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.list_dashboards(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ListDashboardsSvc(inner);
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
                "/google.monitoring.dashboard.v1.DashboardsService/GetDashboard" => {
                    #[allow(non_camel_case_types)]
                    struct GetDashboardSvc<T: DashboardsService>(pub Arc<T>);
                    impl<T: DashboardsService>
                        tonic::server::UnaryService<super::GetDashboardRequest>
                        for GetDashboardSvc<T>
                    {
                        type Response = super::Dashboard;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetDashboardRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.get_dashboard(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetDashboardSvc(inner);
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
                "/google.monitoring.dashboard.v1.DashboardsService/DeleteDashboard" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteDashboardSvc<T: DashboardsService>(pub Arc<T>);
                    impl<T: DashboardsService>
                        tonic::server::UnaryService<super::DeleteDashboardRequest>
                        for DeleteDashboardSvc<T>
                    {
                        type Response = ();
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteDashboardRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.delete_dashboard(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = DeleteDashboardSvc(inner);
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
                "/google.monitoring.dashboard.v1.DashboardsService/UpdateDashboard" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateDashboardSvc<T: DashboardsService>(pub Arc<T>);
                    impl<T: DashboardsService>
                        tonic::server::UnaryService<super::UpdateDashboardRequest>
                        for UpdateDashboardSvc<T>
                    {
                        type Response = super::Dashboard;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateDashboardRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.update_dashboard(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = UpdateDashboardSvc(inner);
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
    impl<T: DashboardsService> Clone for DashboardsServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: DashboardsService> Clone for _Inner<T> {
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
