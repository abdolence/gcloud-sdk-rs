/// Describes how to combine multiple time series to provide different views of
/// the data.  Aggregation consists of an alignment step on individual time
/// series (`alignment_period` and `per_series_aligner`) followed by an optional
/// reduction step of the data across the aligned time series
/// (`cross_series_reducer` and `group_by_fields`).  For more details, see
/// [Aggregation](/monitoring/api/learn_more#aggregation).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Aggregation {
    /// The alignment period for per-[time series][TimeSeries]
    /// alignment. If present, `alignmentPeriod` must be at least 60
    /// seconds.  After per-time series alignment, each time series will
    /// contain data points only on the period boundaries. If
    /// `perSeriesAligner` is not specified or equals `ALIGN_NONE`, then
    /// this field is ignored. If `perSeriesAligner` is specified and
    /// does not equal `ALIGN_NONE`, then this field must be defined;
    /// otherwise an error is returned.
    #[prost(message, optional, tag = "1")]
    pub alignment_period: ::std::option::Option<::prost_types::Duration>,
    /// The approach to be used to align individual time series. Not all
    /// alignment functions may be applied to all time series, depending
    /// on the metric type and value type of the original time
    /// series. Alignment may change the metric type or the value type of
    /// the time series.
    ///
    /// Time series data must be aligned in order to perform cross-time
    /// series reduction. If `crossSeriesReducer` is specified, then
    /// `perSeriesAligner` must be specified and not equal `ALIGN_NONE`
    /// and `alignmentPeriod` must be specified; otherwise, an error is
    /// returned.
    #[prost(enumeration = "aggregation::Aligner", tag = "2")]
    pub per_series_aligner: i32,
    /// The approach to be used to combine time series. Not all reducer
    /// functions may be applied to all time series, depending on the
    /// metric type and the value type of the original time
    /// series. Reduction may change the metric type of value type of the
    /// time series.
    ///
    /// Time series data must be aligned in order to perform cross-time
    /// series reduction. If `crossSeriesReducer` is specified, then
    /// `perSeriesAligner` must be specified and not equal `ALIGN_NONE`
    /// and `alignmentPeriod` must be specified; otherwise, an error is
    /// returned.
    #[prost(enumeration = "aggregation::Reducer", tag = "4")]
    pub cross_series_reducer: i32,
    /// The set of fields to preserve when `crossSeriesReducer` is
    /// specified. The `groupByFields` determine how the time series are
    /// partitioned into subsets prior to applying the aggregation
    /// function. Each subset contains time series that have the same
    /// value for each of the grouping fields. Each individual time
    /// series is a member of exactly one subset. The
    /// `crossSeriesReducer` is applied to each subset of time series.
    /// It is not possible to reduce across different resource types, so
    /// this field implicitly contains `resource.type`.  Fields not
    /// specified in `groupByFields` are aggregated away.  If
    /// `groupByFields` is not specified and all the time series have
    /// the same resource type, then the time series are aggregated into
    /// a single output time series. If `crossSeriesReducer` is not
    /// defined, this field is ignored.
    #[prost(string, repeated, tag = "5")]
    pub group_by_fields: ::std::vec::Vec<std::string::String>,
}
pub mod aggregation {
    /// The Aligner describes how to bring the data points in a single
    /// time series into temporal alignment.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Aligner {
        /// No alignment. Raw data is returned. Not valid if cross-time
        /// series reduction is requested. The value type of the result is
        /// the same as the value type of the input.
        AlignNone = 0,
        /// Align and convert to delta metric type. This alignment is valid
        /// for cumulative metrics and delta metrics. Aligning an existing
        /// delta metric to a delta metric requires that the alignment
        /// period be increased. The value type of the result is the same
        /// as the value type of the input.
        ///
        /// One can think of this aligner as a rate but without time units; that
        /// is, the output is conceptually (second_point - first_point).
        AlignDelta = 1,
        /// Align and convert to a rate. This alignment is valid for
        /// cumulative metrics and delta metrics with numeric values. The output is a
        /// gauge metric with value type
        /// [DOUBLE][google.api.MetricDescriptor.ValueType.DOUBLE].
        ///
        /// One can think of this aligner as conceptually providing the slope of
        /// the line that passes through the value at the start and end of the
        /// window. In other words, this is conceptually ((y1 - y0)/(t1 - t0)),
        /// and the output unit is one that has a "/time" dimension.
        ///
        /// If, by rate, you are looking for percentage change, see the
        /// `ALIGN_PERCENT_CHANGE` aligner option.
        AlignRate = 2,
        /// Align by interpolating between adjacent points around the
        /// period boundary. This alignment is valid for gauge
        /// metrics with numeric values. The value type of the result is the same
        /// as the value type of the input.
        AlignInterpolate = 3,
        /// Align by shifting the oldest data point before the period
        /// boundary to the boundary. This alignment is valid for gauge
        /// metrics. The value type of the result is the same as the
        /// value type of the input.
        AlignNextOlder = 4,
        /// Align time series via aggregation. The resulting data point in
        /// the alignment period is the minimum of all data points in the
        /// period. This alignment is valid for gauge and delta metrics with numeric
        /// values. The value type of the result is the same as the value
        /// type of the input.
        AlignMin = 10,
        /// Align time series via aggregation. The resulting data point in
        /// the alignment period is the maximum of all data points in the
        /// period. This alignment is valid for gauge and delta metrics with numeric
        /// values. The value type of the result is the same as the value
        /// type of the input.
        AlignMax = 11,
        /// Align time series via aggregation. The resulting data point in
        /// the alignment period is the average or arithmetic mean of all
        /// data points in the period. This alignment is valid for gauge and delta
        /// metrics with numeric values. The value type of the output is
        /// [DOUBLE][google.api.MetricDescriptor.ValueType.DOUBLE].
        AlignMean = 12,
        /// Align time series via aggregation. The resulting data point in
        /// the alignment period is the count of all data points in the
        /// period. This alignment is valid for gauge and delta metrics with numeric
        /// or Boolean values. The value type of the output is
        /// [INT64][google.api.MetricDescriptor.ValueType.INT64].
        AlignCount = 13,
        /// Align time series via aggregation. The resulting data point in
        /// the alignment period is the sum of all data points in the
        /// period. This alignment is valid for gauge and delta metrics with numeric
        /// and distribution values. The value type of the output is the
        /// same as the value type of the input.
        AlignSum = 14,
        /// Align time series via aggregation. The resulting data point in
        /// the alignment period is the standard deviation of all data
        /// points in the period. This alignment is valid for gauge and delta metrics
        /// with numeric values. The value type of the output is
        /// [DOUBLE][google.api.MetricDescriptor.ValueType.DOUBLE].
        AlignStddev = 15,
        /// Align time series via aggregation. The resulting data point in
        /// the alignment period is the count of True-valued data points in the
        /// period. This alignment is valid for gauge metrics with
        /// Boolean values. The value type of the output is
        /// [INT64][google.api.MetricDescriptor.ValueType.INT64].
        AlignCountTrue = 16,
        /// Align time series via aggregation. The resulting data point in
        /// the alignment period is the count of False-valued data points in the
        /// period. This alignment is valid for gauge metrics with
        /// Boolean values. The value type of the output is
        /// [INT64][google.api.MetricDescriptor.ValueType.INT64].
        AlignCountFalse = 24,
        /// Align time series via aggregation. The resulting data point in
        /// the alignment period is the fraction of True-valued data points in the
        /// period. This alignment is valid for gauge metrics with Boolean values.
        /// The output value is in the range [0, 1] and has value type
        /// [DOUBLE][google.api.MetricDescriptor.ValueType.DOUBLE].
        AlignFractionTrue = 17,
        /// Align time series via aggregation. The resulting data point in
        /// the alignment period is the 99th percentile of all data
        /// points in the period. This alignment is valid for gauge and delta metrics
        /// with distribution values. The output is a gauge metric with value type
        /// [DOUBLE][google.api.MetricDescriptor.ValueType.DOUBLE].
        AlignPercentile99 = 18,
        /// Align time series via aggregation. The resulting data point in
        /// the alignment period is the 95th percentile of all data
        /// points in the period. This alignment is valid for gauge and delta metrics
        /// with distribution values. The output is a gauge metric with value type
        /// [DOUBLE][google.api.MetricDescriptor.ValueType.DOUBLE].
        AlignPercentile95 = 19,
        /// Align time series via aggregation. The resulting data point in
        /// the alignment period is the 50th percentile of all data
        /// points in the period. This alignment is valid for gauge and delta metrics
        /// with distribution values. The output is a gauge metric with value type
        /// [DOUBLE][google.api.MetricDescriptor.ValueType.DOUBLE].
        AlignPercentile50 = 20,
        /// Align time series via aggregation. The resulting data point in
        /// the alignment period is the 5th percentile of all data
        /// points in the period. This alignment is valid for gauge and delta metrics
        /// with distribution values. The output is a gauge metric with value type
        /// [DOUBLE][google.api.MetricDescriptor.ValueType.DOUBLE].
        AlignPercentile05 = 21,
        /// Align and convert to a percentage change. This alignment is valid for
        /// gauge and delta metrics with numeric values. This alignment conceptually
        /// computes the equivalent of "((current - previous)/previous)*100"
        /// where previous value is determined based on the alignmentPeriod.
        /// In the event that previous is 0 the calculated value is infinity with the
        /// exception that if both (current - previous) and previous are 0 the
        /// calculated value is 0.
        /// A 10 minute moving mean is computed at each point of the time window
        /// prior to the above calculation to smooth the metric and prevent false
        /// positives from very short lived spikes.
        /// Only applicable for data that is >= 0. Any values < 0 are treated as
        /// no data. While delta metrics are accepted by this alignment special care
        /// should be taken that the values for the metric will always be positive.
        /// The output is a gauge metric with value type
        /// [DOUBLE][google.api.MetricDescriptor.ValueType.DOUBLE].
        AlignPercentChange = 23,
    }
    /// A Reducer describes how to aggregate data points from multiple
    /// time series into a single time series.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Reducer {
        /// No cross-time series reduction. The output of the aligner is
        /// returned.
        ReduceNone = 0,
        /// Reduce by computing the mean across time series for each
        /// alignment period. This reducer is valid for delta and
        /// gauge metrics with numeric or distribution values. The value type of the
        /// output is [DOUBLE][google.api.MetricDescriptor.ValueType.DOUBLE].
        ReduceMean = 1,
        /// Reduce by computing the minimum across time series for each
        /// alignment period. This reducer is valid for delta and
        /// gauge metrics with numeric values. The value type of the output
        /// is the same as the value type of the input.
        ReduceMin = 2,
        /// Reduce by computing the maximum across time series for each
        /// alignment period. This reducer is valid for delta and
        /// gauge metrics with numeric values. The value type of the output
        /// is the same as the value type of the input.
        ReduceMax = 3,
        /// Reduce by computing the sum across time series for each
        /// alignment period. This reducer is valid for delta and
        /// gauge metrics with numeric and distribution values. The value type of
        /// the output is the same as the value type of the input.
        ReduceSum = 4,
        /// Reduce by computing the standard deviation across time series
        /// for each alignment period. This reducer is valid for delta
        /// and gauge metrics with numeric or distribution values. The value type of
        /// the output is [DOUBLE][google.api.MetricDescriptor.ValueType.DOUBLE].
        ReduceStddev = 5,
        /// Reduce by computing the count of data points across time series
        /// for each alignment period. This reducer is valid for delta
        /// and gauge metrics of numeric, Boolean, distribution, and string value
        /// type. The value type of the output is
        /// [INT64][google.api.MetricDescriptor.ValueType.INT64].
        ReduceCount = 6,
        /// Reduce by computing the count of True-valued data points across time
        /// series for each alignment period. This reducer is valid for delta
        /// and gauge metrics of Boolean value type. The value type of
        /// the output is [INT64][google.api.MetricDescriptor.ValueType.INT64].
        ReduceCountTrue = 7,
        /// Reduce by computing the count of False-valued data points across time
        /// series for each alignment period. This reducer is valid for delta
        /// and gauge metrics of Boolean value type. The value type of
        /// the output is [INT64][google.api.MetricDescriptor.ValueType.INT64].
        ReduceCountFalse = 15,
        /// Reduce by computing the fraction of True-valued data points across time
        /// series for each alignment period. This reducer is valid for delta
        /// and gauge metrics of Boolean value type. The output value is in the
        /// range [0, 1] and has value type
        /// [DOUBLE][google.api.MetricDescriptor.ValueType.DOUBLE].
        ReduceFractionTrue = 8,
        /// Reduce by computing 99th percentile of data points across time series
        /// for each alignment period. This reducer is valid for gauge and delta
        /// metrics of numeric and distribution type. The value of the output is
        /// [DOUBLE][google.api.MetricDescriptor.ValueType.DOUBLE]
        ReducePercentile99 = 9,
        /// Reduce by computing 95th percentile of data points across time series
        /// for each alignment period. This reducer is valid for gauge and delta
        /// metrics of numeric and distribution type. The value of the output is
        /// [DOUBLE][google.api.MetricDescriptor.ValueType.DOUBLE]
        ReducePercentile95 = 10,
        /// Reduce by computing 50th percentile of data points across time series
        /// for each alignment period. This reducer is valid for gauge and delta
        /// metrics of numeric and distribution type. The value of the output is
        /// [DOUBLE][google.api.MetricDescriptor.ValueType.DOUBLE]
        ReducePercentile50 = 11,
        /// Reduce by computing 5th percentile of data points across time series
        /// for each alignment period. This reducer is valid for gauge and delta
        /// metrics of numeric and distribution type. The value of the output is
        /// [DOUBLE][google.api.MetricDescriptor.ValueType.DOUBLE]
        ReducePercentile05 = 12,
    }
}
/// Describes a ranking-based time series filter. Each input time series is
/// ranked with an aligner. The filter lets through up to `num_time_series` time
/// series, selecting them based on the relative ranking.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PickTimeSeriesFilter {
    /// `rankingMethod` is applied to each time series independently to produce the
    /// value which will be used to compare the time series to other time series.
    #[prost(enumeration = "pick_time_series_filter::Method", tag = "1")]
    pub ranking_method: i32,
    /// How many time series to return.
    #[prost(int32, tag = "2")]
    pub num_time_series: i32,
    /// How to use the ranking to select time series that pass through the filter.
    #[prost(enumeration = "pick_time_series_filter::Direction", tag = "3")]
    pub direction: i32,
}
pub mod pick_time_series_filter {
    /// The value reducers that can be applied to a PickTimeSeriesFilter.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Method {
        /// Not allowed in well-formed requests.
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
        /// Not allowed in well-formed requests.
        Unspecified = 0,
        /// Pass the highest ranking inputs.
        Top = 1,
        /// Pass the lowest ranking inputs.
        Bottom = 2,
    }
}
/// A filter that ranks streams based on their statistical relation to other
/// streams in a request.
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
    /// [`unit`](/monitoring/api/ref_v3/rest/v3/projects.metricDescriptors)
    /// field in `MetricDescriptor`.
    #[prost(string, tag = "5")]
    pub unit_override: std::string::String,
    /// Parameters needed to obtain data for the chart.
    #[prost(oneof = "time_series_query::Source", tags = "1, 2")]
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
    }
}
/// A filter that defines a subset of time series data that is displayed in a
/// widget. Time series data is fetched using the
/// [`ListTimeSeries`](/monitoring/api/ref_v3/rest/v3/projects.timeSeries/list)
/// method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TimeSeriesFilter {
    /// Required. The [monitoring filter](/monitoring/api/v3/filters) that identifies the
    /// metric types, resources, and projects to query.
    #[prost(string, tag = "1")]
    pub filter: std::string::String,
    /// By default, the raw time series data is returned.
    /// Use this field to combine multiple time series for different views of the
    /// data.
    #[prost(message, optional, tag = "2")]
    pub aggregation: ::std::option::Option<Aggregation>,
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
        /// Required. The [monitoring filter](/monitoring/api/v3/filters) that identifies the
        /// metric types, resources, and projects to query.
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
    /// Fields for querying time series data from the
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
        /// The type of sparkchart to show in this chartView.
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
    /// The data displayed in this chart.
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
        /// Fields for querying time series data from the
        /// Stackdriver metrics API.
        #[prost(message, optional, tag = "1")]
        pub time_series_query: ::std::option::Option<super::TimeSeriesQuery>,
        /// How this data should be plotted on the chart.
        #[prost(enumeration = "data_set::PlotType", tag = "2")]
        pub plot_type: i32,
        /// A template string for naming `TimeSeries` in the resulting data set.
        /// This should be a string with interpolations of the form ${label_name},
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
    /// The resource name of the dashboard.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// The mutable, human-readable name.
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
    /// Required. The project on which to execute the request. The format is
    /// `"projects/{project_id_or_number}"`. The {project_id_or_number} must match
    /// the dashboard resource name.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Required. The initial dashboard specification.
    #[prost(message, optional, tag = "2")]
    pub dashboard: ::std::option::Option<Dashboard>,
}
/// The `ListDashboards` request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDashboardsRequest {
    /// Required. The scope of the dashboards to list. A project scope must be
    /// specified in the form of `"projects/{project_id_or_number}"`.
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
    /// use that value as `pageToken` in the next call to this method.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
}
/// The `GetDashboard` request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDashboardRequest {
    /// Required. The resource name of the Dashboard. The format is one of
    /// `"dashboards/{dashboard_id}"` (for system dashboards) or
    /// `"projects/{project_id_or_number}/dashboards/{dashboard_id}"`
    /// (for custom dashboards).
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// The `DeleteDashboard` request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteDashboardRequest {
    /// Required. The resource name of the Dashboard. The format is
    /// `"projects/{project_id_or_number}/dashboards/{dashboard_id}"`.
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
    impl DashboardsServiceClient<tonic::transport::Channel> {
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
    impl<T: DashboardsService> tonic::transport::NamedService for DashboardsServiceServer<T> {
        const NAME: &'static str = "google.monitoring.dashboard.v1.DashboardsService";
    }
}
