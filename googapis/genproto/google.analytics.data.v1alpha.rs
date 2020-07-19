/// A contiguous set of days: startDate, startDate + 1, ..., endDate. Requests
/// are allowed up to 4 date ranges, and the union of the ranges can cover up to
/// 1 year.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DateRange {
    /// The inclusive start date for the query in the format `YYYY-MM-DD`. Cannot
    /// be after `end_date`. The format `NdaysAgo`, `yesterday`, or `today` is also
    /// accepted, and in that case, the date is inferred based on the property's
    /// reporting time zone.
    #[prost(string, tag = "1")]
    pub start_date: std::string::String,
    /// The inclusive end date for the query in the format `YYYY-MM-DD`. Cannot
    /// be before `start_date`. The format `NdaysAgo`, `yesterday`, or `today` is
    /// also accepted, and in that case, the date is inferred based on the
    /// property's reporting time zone.
    #[prost(string, tag = "2")]
    pub end_date: std::string::String,
    /// Assigns a name to this date range. The dimension `dateRange` is valued to
    /// this name in a report response. If set, cannot begin with `date_range_` or
    /// `RESERVED_`. If not set, date ranges are named by their zero based index in
    /// the request: `date_range_0`, `date_range_1`, etc.
    #[prost(string, tag = "3")]
    pub name: std::string::String,
}
/// The unique identifier of the property whose events are tracked.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Entity {
    /// A Google Analytics App + Web property id.
    #[prost(string, tag = "1")]
    pub property_id: std::string::String,
}
/// Dimensions are attributes of your data. For example, the dimension City
/// indicates the city, for example, "Paris" or "New York", from which an event
/// originates. Requests are allowed up to 8 dimensions.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Dimension {
    /// The name of the dimension.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// One dimension can be the result of an expression of multiple dimensions.
    /// For example, dimension "country, city": concatenate(country, ", ", city).
    #[prost(message, optional, tag = "2")]
    pub dimension_expression: ::std::option::Option<DimensionExpression>,
}
/// Used to express a dimension which is the result of a formula of multiple
/// dimensions. Example usages:
/// 1) lower_case(dimension)
/// 2) concatenate(dimension1, symbol, dimension2).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DimensionExpression {
    /// Specify one type of dimension expression for `DimensionExpression`.
    #[prost(oneof = "dimension_expression::OneExpression", tags = "4, 5, 6")]
    pub one_expression: ::std::option::Option<dimension_expression::OneExpression>,
}
pub mod dimension_expression {
    /// Used to convert a dimension value to a single case.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CaseExpression {
        /// Name of a dimension. The name must refer back to a name in dimensions
        /// field of the request.
        #[prost(string, tag = "1")]
        pub dimension_name: std::string::String,
    }
    /// Used to combine dimension values to a single dimension.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ConcatenateExpression {
        /// Names of dimensions. The names must refer back to names in the dimensions
        /// field of the request.
        #[prost(string, repeated, tag = "1")]
        pub dimension_names: ::std::vec::Vec<std::string::String>,
        /// The delimiter placed between dimension names.
        ///
        /// Delimiters are often single characters such as "|" or "," but can be
        /// longer strings. If a dimension value contains the delimiter, both will be
        /// present in response with no distinction. For example if dimension 1 value
        /// = "US,FR", dimension 2 value = "JP", and delimiter = ",", then the
        /// response will contain "US,FR,JP".
        #[prost(string, tag = "2")]
        pub delimiter: std::string::String,
    }
    /// Specify one type of dimension expression for `DimensionExpression`.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum OneExpression {
        /// Used to convert a dimension value to lower case.
        #[prost(message, tag = "4")]
        LowerCase(CaseExpression),
        /// Used to convert a dimension value to upper case.
        #[prost(message, tag = "5")]
        UpperCase(CaseExpression),
        /// Used to combine dimension values to a single dimension.
        /// For example, dimension "country, city": concatenate(country, ", ", city).
        #[prost(message, tag = "6")]
        Concatenate(ConcatenateExpression),
    }
}
/// The quantitative measurements of a report. For example, the metric eventCount
/// is the total number of events. Requests are allowed up to 10 metrics.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Metric {
    /// The name of the metric.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// A mathematical expression for derived metrics. For example, the metric
    /// Event count per user is eventCount/totalUsers.
    #[prost(string, tag = "2")]
    pub expression: std::string::String,
    /// Indicates if a metric is invisible.
    /// If a metric is invisible, the metric is not in the response, but can be
    /// used in filters, order_bys or being referred to in a metric expression.
    #[prost(bool, tag = "3")]
    pub invisible: bool,
}
/// To express dimension or metric filters.
/// The fields in the same FilterExpression need to be either all dimensions or
/// all metrics.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FilterExpression {
    /// Specify one type of filter expression for `FilterExpression`.
    #[prost(oneof = "filter_expression::Expr", tags = "1, 2, 3, 4")]
    pub expr: ::std::option::Option<filter_expression::Expr>,
}
pub mod filter_expression {
    /// Specify one type of filter expression for `FilterExpression`.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Expr {
        /// The FilterExpressions in and_group have an AND relationship.
        #[prost(message, tag = "1")]
        AndGroup(super::FilterExpressionList),
        /// The FilterExpressions in or_group have an OR relationship.
        #[prost(message, tag = "2")]
        OrGroup(super::FilterExpressionList),
        /// The FilterExpression is NOT of not_expression.
        #[prost(message, tag = "3")]
        NotExpression(Box<super::FilterExpression>),
        /// A primitive filter.
        /// All fields in filter in same FilterExpression needs to be either all
        /// dimensions or metrics.
        #[prost(message, tag = "4")]
        Filter(super::Filter),
    }
}
/// A list of filter expressions.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FilterExpressionList {
    /// A list of filter expressions.
    #[prost(message, repeated, tag = "1")]
    pub expressions: ::std::vec::Vec<FilterExpression>,
}
/// An expression to filter dimension or metric values.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Filter {
    /// The dimension name or metric name. Must be a name defined in dimensions
    /// or metrics.
    #[prost(string, tag = "1")]
    pub field_name: std::string::String,
    /// Specify one type of filter for `Filter`.
    #[prost(oneof = "filter::OneFilter", tags = "2, 3, 4, 5, 6")]
    pub one_filter: ::std::option::Option<filter::OneFilter>,
}
pub mod filter {
    /// The filter for string
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct StringFilter {
        /// The match type for this filter.
        #[prost(enumeration = "string_filter::MatchType", tag = "1")]
        pub match_type: i32,
        /// The string value used for the matching.
        #[prost(string, tag = "2")]
        pub value: std::string::String,
        /// If true, the string value is case sensitive.
        #[prost(bool, tag = "3")]
        pub case_sensitive: bool,
    }
    pub mod string_filter {
        /// The match type of a string filter
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
        #[repr(i32)]
        pub enum MatchType {
            /// Unspecified
            Unspecified = 0,
            /// Exact match of the string value.
            Exact = 1,
            /// Begins with the string value.
            BeginsWith = 2,
            /// Ends with the string value.
            EndsWith = 3,
            /// Contains the string value.
            Contains = 4,
            /// Full regular expression match with the string value.
            FullRegexp = 5,
            /// Partial regular expression match with the string value.
            PartialRegexp = 6,
        }
    }
    /// The result needs to be in a list of string values.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct InListFilter {
        /// The list of string values.
        /// Must be non-empty.
        #[prost(string, repeated, tag = "1")]
        pub values: ::std::vec::Vec<std::string::String>,
        /// If true, the string value is case sensitive.
        #[prost(bool, tag = "2")]
        pub case_sensitive: bool,
    }
    /// Filters for numeric or date values.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct NumericFilter {
        /// The operation type for this filter.
        #[prost(enumeration = "numeric_filter::Operation", tag = "1")]
        pub operation: i32,
        /// A numeric value or a date value.
        #[prost(message, optional, tag = "2")]
        pub value: ::std::option::Option<super::NumericValue>,
    }
    pub mod numeric_filter {
        /// The operation applied to a numeric filter
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
        #[repr(i32)]
        pub enum Operation {
            /// Unspecified.
            Unspecified = 0,
            /// Equal
            Equal = 1,
            /// Less than
            LessThan = 2,
            /// Less than or equal
            LessThanOrEqual = 3,
            /// Greater than
            GreaterThan = 4,
            /// Greater than or equal
            GreaterThanOrEqual = 5,
        }
    }
    /// To express that the result needs to be between two numbers (inclusive).
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct BetweenFilter {
        /// Begins with this number.
        #[prost(message, optional, tag = "1")]
        pub from_value: ::std::option::Option<super::NumericValue>,
        /// Ends with this number.
        #[prost(message, optional, tag = "2")]
        pub to_value: ::std::option::Option<super::NumericValue>,
    }
    /// Specify one type of filter for `Filter`.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum OneFilter {
        /// A filter for null values.
        #[prost(bool, tag = "2")]
        NullFilter(bool),
        /// Strings related filter.
        #[prost(message, tag = "3")]
        StringFilter(StringFilter),
        /// A filter for in list values.
        #[prost(message, tag = "4")]
        InListFilter(InListFilter),
        /// A filter for numeric or date values.
        #[prost(message, tag = "5")]
        NumericFilter(NumericFilter),
        /// A filter for two values.
        #[prost(message, tag = "6")]
        BetweenFilter(BetweenFilter),
    }
}
/// The sort options.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrderBy {
    /// If true, sorts by descending order.
    #[prost(bool, tag = "4")]
    pub desc: bool,
    /// Specify one type of order by for `OrderBy`.
    #[prost(oneof = "order_by::OneOrderBy", tags = "1, 2, 3")]
    pub one_order_by: ::std::option::Option<order_by::OneOrderBy>,
}
pub mod order_by {
    /// Sorts by metric values.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct MetricOrderBy {
        /// A metric name in the request to order by.
        #[prost(string, tag = "1")]
        pub metric_name: std::string::String,
    }
    /// Sorts by dimension values.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DimensionOrderBy {
        /// A dimension name in the request to order by.
        #[prost(string, tag = "1")]
        pub dimension_name: std::string::String,
        /// Controls the rule for dimension value ordering.
        #[prost(enumeration = "dimension_order_by::OrderType", tag = "2")]
        pub order_type: i32,
    }
    pub mod dimension_order_by {
        /// Rule to order the string dimension values by.
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
        #[repr(i32)]
        pub enum OrderType {
            /// Unspecified.
            Unspecified = 0,
            /// Alphanumeric sort by Unicode code point. For example, "2" < "A" < "X" <
            /// "b" < "z".
            Alphanumeric = 1,
            /// Case insensitive alphanumeric sort by lower case Unicode code point.
            /// For example, "2" < "A" < "b" < "X" < "z".
            CaseInsensitiveAlphanumeric = 2,
            /// Dimension values are converted to numbers before sorting. For example
            /// in NUMERIC sort, "25" < "100", and in `ALPHANUMERIC` sort, "100" <
            /// "25". Non-numeric dimension values all have equal ordering value below
            /// all numeric values.
            Numeric = 3,
        }
    }
    /// Sorts by a pivot column group.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PivotOrderBy {
        /// In the response to order by, order rows by this column. Must be a metric
        /// name from the request.
        #[prost(string, tag = "1")]
        pub metric_name: std::string::String,
        /// Used to select a dimension name and value pivot. If multiple pivot
        /// selections are given, the sort occurs on rows where all pivot selection
        /// dimension name and value pairs match the row's dimension name and value
        /// pair.
        #[prost(message, repeated, tag = "2")]
        pub pivot_selections: ::std::vec::Vec<pivot_order_by::PivotSelection>,
    }
    pub mod pivot_order_by {
        /// A pair of dimension names and values. Rows with this dimension pivot pair
        /// are ordered by the metric's value.
        ///
        /// For example if pivots = {{"browser", "Chrome"}} and
        /// metric_name = "Sessions",
        /// then the rows will be sorted based on Sessions in Chrome.
        ///
        ///     ---------|----------|----------------|----------|----------------
        ///              |  Chrome  |    Chrome      |  Safari  |     Safari
        ///     ---------|----------|----------------|----------|----------------
        ///      Country | Sessions | Pages/Sessions | Sessions | Pages/Sessions
        ///     ---------|----------|----------------|----------|----------------
        ///         US   |    2     |       2        |     3    |        1
        ///     ---------|----------|----------------|----------|----------------
        ///       Canada |    3     |       1        |     4    |        1
        ///     ---------|----------|----------------|----------|----------------
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct PivotSelection {
            /// Must be a dimension name from the request.
            #[prost(string, tag = "1")]
            pub dimension_name: std::string::String,
            /// Order by only when the named dimension is this value.
            #[prost(string, tag = "2")]
            pub dimension_value: std::string::String,
        }
    }
    /// Specify one type of order by for `OrderBy`.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum OneOrderBy {
        /// Sorts results by a metric's values.
        #[prost(message, tag = "1")]
        Metric(MetricOrderBy),
        /// Sorts results by a dimension's values.
        #[prost(message, tag = "2")]
        Dimension(DimensionOrderBy),
        /// Sorts results by a metric's values within a pivot column group.
        #[prost(message, tag = "3")]
        Pivot(PivotOrderBy),
    }
}
/// Describes the visible dimension columns and rows in the report response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Pivot {
    /// Dimension names for visible columns in the report response. Including
    /// "dateRange" produces a date range column; for each row in the response,
    /// dimension values in the date range column will indicate the corresponding
    /// date range from the request.
    #[prost(string, repeated, tag = "1")]
    pub field_names: ::std::vec::Vec<std::string::String>,
    /// Specifies how dimensions are ordered in the pivot. In the first Pivot, the
    /// OrderBys determine Row and DimensionHeader ordering; in subsequent Pivots,
    /// the OrderBys determine only DimensionHeader ordering. Dimensions specified
    /// in these OrderBys must be a subset of Pivot.field_names.
    #[prost(message, repeated, tag = "2")]
    pub order_bys: ::std::vec::Vec<OrderBy>,
    /// The row count of the start row. The first row is counted as row 0.
    #[prost(int64, tag = "3")]
    pub offset: i64,
    /// The number of rows to return in this pivot.
    /// If zero or unspecified, all rows are returned.
    #[prost(int64, tag = "4")]
    pub limit: i64,
    /// Aggregate the metrics by dimensions in this pivot using the specified
    /// metric_aggregations.
    #[prost(enumeration = "MetricAggregation", repeated, tag = "5")]
    pub metric_aggregations: ::std::vec::Vec<i32>,
}
/// Specification for a cohort report.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CohortSpec {
    /// The definition for the cohorts.
    #[prost(message, repeated, tag = "1")]
    pub cohorts: ::std::vec::Vec<Cohort>,
    /// The data ranges of cohorts.
    #[prost(message, optional, tag = "2")]
    pub cohorts_range: ::std::option::Option<CohortsRange>,
    /// Settings of a cohort report.
    #[prost(message, optional, tag = "3")]
    pub cohort_report_settings: ::std::option::Option<CohortReportSettings>,
}
/// Defines a cohort. A cohort is a group of users who share a common
/// characteristic. For example, all users with the same acquisition date
/// belong to the same cohort.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Cohort {
    /// Assigns a name to this cohort. The dimension `cohort` is valued to this
    /// name in a report response. If not set, a cohort is named the empty string.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// The dimension used by cohort. Only supports `firstTouchDate` for retention
    /// report.
    #[prost(string, tag = "2")]
    pub dimension: std::string::String,
    /// The cohort selects users whose first visit date is between start date
    /// and end date defined in the date_range. The date range should be aligned
    /// with the cohort's granularity.
    /// If CohortsRange uses daily granularity, the date range can be aligned to
    /// any day.
    /// If CohortsRange uses weekly granularity, the date range should be aligned
    /// to the week boundary, starting at Sunday and ending Saturday. If
    /// CohortsRange uses monthly granularity, the date range should be aligned to
    /// the month, starting at the first and ending on the last day of the month.
    #[prost(message, optional, tag = "3")]
    pub date_range: ::std::option::Option<DateRange>,
}
/// Settings of a cohort report.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CohortReportSettings {
    /// If true, accumulates the result from first visit day to the end day.
    #[prost(bool, tag = "1")]
    pub accumulate: bool,
    /// If true, the report is for lifetime value report and should pivot on user
    /// event.
    #[prost(bool, tag = "2")]
    pub pivot_on_user_event: bool,
    /// If some values are missing while computing ratios, we want to compute the
    /// ratios only based on non-missing values.
    /// This field should be set to true only for a totals request.
    #[prost(bool, tag = "4")]
    pub missing_value_as_zero: bool,
}
/// Describes date range for a cohort report.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CohortsRange {
    /// Reporting date range for each cohort is calculated based on these three
    /// fields.
    #[prost(enumeration = "cohorts_range::Granularity", tag = "1")]
    pub granularity: i32,
    /// For daily cohorts, this will be the start day offset.
    /// For weekly cohorts, this will be the week offset.
    #[prost(int32, tag = "2")]
    pub start_offset: i32,
    /// For daily cohorts, this will be the end day offset.
    /// For weekly cohorts, this will be the week offset.
    #[prost(int32, tag = "3")]
    pub end_offset: i32,
}
pub mod cohorts_range {
    /// Reporting granularity for the cohorts.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Granularity {
        /// Unspecified.
        Unspecified = 0,
        /// Daily
        Daily = 1,
        /// Weekly
        Weekly = 2,
        /// Monthly
        Monthly = 3,
    }
}
/// Response's metadata carrying additional information about the report content.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseMetaData {
    /// If true, indicates some buckets of dimension combinations are rolled into
    /// "(other)" row. This can happen for high cardinality reports.
    #[prost(bool, tag = "3")]
    pub data_loss_from_other_row: bool,
}
/// Describes the metric column in the report.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MetricHeader {
    /// Metric name.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Metric data type.
    #[prost(enumeration = "MetricType", tag = "2")]
    pub r#type: i32,
}
/// Dimensions' values in a pivot.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PivotHeader {
    /// The size is the same as the cardinality of the corresponding dimension
    /// combinations.
    #[prost(message, repeated, tag = "1")]
    pub dimension_headers: ::std::vec::Vec<DimensionHeader>,
    /// The cardinality of the pivot as if offset = 0 and limit = -1.
    #[prost(int32, tag = "2")]
    pub row_count: i32,
}
/// The header for the dimensions.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DimensionHeader {
    /// Values of multiple dimensions in a pivot.
    #[prost(message, repeated, tag = "1")]
    pub dimension_values: ::std::vec::Vec<DimensionValue>,
}
/// Report data for each row.
/// For example if RunReportRequest contains:
///
/// ```none
/// dimensions {
///   name: "eventName"
/// }
/// dimensions {
///   name: "countryId"
/// }
/// metrics {
///   name: "eventCount"
/// }
/// ```
///
/// One row with 'in_app_purchase' as the eventName, 'us' as the countryId, and
/// 15 as the eventCount, would be:
///
/// ```none
/// dimension_values {
///   name: 'in_app_purchase'
///   name: 'us'
/// }
/// metric_values {
///   int64_value: 15
/// }
/// ```
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Row {
    /// List of requested dimension values. In a PivotReport, dimension_values
    /// are only listed for dimensions included in a pivot.
    #[prost(message, repeated, tag = "1")]
    pub dimension_values: ::std::vec::Vec<DimensionValue>,
    /// List of requested visible metric values.
    #[prost(message, repeated, tag = "2")]
    pub metric_values: ::std::vec::Vec<MetricValue>,
}
/// The value of a dimension.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DimensionValue {
    /// One kind of dimension value
    #[prost(oneof = "dimension_value::OneValue", tags = "1")]
    pub one_value: ::std::option::Option<dimension_value::OneValue>,
}
pub mod dimension_value {
    /// One kind of dimension value
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum OneValue {
        /// Value as a string if the dimension type is a string.
        #[prost(string, tag = "1")]
        Value(std::string::String),
    }
}
/// The value of a metric.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MetricValue {
    /// One of metric value
    #[prost(oneof = "metric_value::OneValue", tags = "4")]
    pub one_value: ::std::option::Option<metric_value::OneValue>,
}
pub mod metric_value {
    /// One of metric value
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum OneValue {
        /// Measurement value. See MetricHeader for type.
        #[prost(string, tag = "4")]
        Value(std::string::String),
    }
}
/// To represent a number.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NumericValue {
    /// One of a numeric value
    #[prost(oneof = "numeric_value::OneValue", tags = "1, 2")]
    pub one_value: ::std::option::Option<numeric_value::OneValue>,
}
pub mod numeric_value {
    /// One of a numeric value
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum OneValue {
        /// Integer value
        #[prost(int64, tag = "1")]
        Int64Value(i64),
        /// Double value
        #[prost(double, tag = "2")]
        DoubleValue(f64),
    }
}
/// Current state of all quotas for this Analytics Property. If any quota for a
/// property is exhausted, all requests to that property will return Resource
/// Exhausted errors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PropertyQuota {
    /// Analytics Properties can use up to 25,000 tokens per day. Most requests
    /// consume fewer than 10 tokens.
    #[prost(message, optional, tag = "1")]
    pub tokens_per_day: ::std::option::Option<QuotaStatus>,
    /// Analytics Properties can use up to 5,000 tokens per day. An API request
    /// consumes a single number of tokens, and that number is deducted from both
    /// the hourly and daily quotas.
    #[prost(message, optional, tag = "2")]
    pub tokens_per_hour: ::std::option::Option<QuotaStatus>,
    /// Analytics Properties can send up to 10 concurrent requests.
    #[prost(message, optional, tag = "3")]
    pub concurrent_requests: ::std::option::Option<QuotaStatus>,
    /// Analytics Properties and cloud project pairs can have up to 10
    /// server errors per hour.
    #[prost(message, optional, tag = "4")]
    pub server_errors_per_project_per_hour: ::std::option::Option<QuotaStatus>,
}
/// Current state for a particular quota group.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuotaStatus {
    /// Quota consumed by this request.
    #[prost(int32, tag = "1")]
    pub consumed: i32,
    /// Quota remaining after this request.
    #[prost(int32, tag = "2")]
    pub remaining: i32,
}
/// Represents aggregation of metrics.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum MetricAggregation {
    /// Unspecified operator.
    Unspecified = 0,
    /// SUM operator.
    Total = 1,
    /// Minimum operator.
    Minimum = 5,
    /// Maximum operator.
    Maximum = 6,
    /// Count operator.
    Count = 4,
}
/// Type of a metric value.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum MetricType {
    /// Unspecified type.
    Unspecified = 0,
    /// Integer type.
    TypeInteger = 1,
    /// Floating point type.
    TypeFloat = 2,
}
/// The request to generate a report.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RunReportRequest {
    /// A property whose events are tracked. Within a batch request, this entity
    /// should either be unspecified or consistent with the batch-level entity.
    #[prost(message, optional, tag = "1")]
    pub entity: ::std::option::Option<Entity>,
    /// The dimensions requested and displayed.
    #[prost(message, repeated, tag = "2")]
    pub dimensions: ::std::vec::Vec<Dimension>,
    /// The metrics requested and displayed.
    #[prost(message, repeated, tag = "3")]
    pub metrics: ::std::vec::Vec<Metric>,
    /// Date ranges of data to read. If multiple date ranges are requested, each
    /// response row will contain a zero based date range index. If two date
    /// ranges overlap, the event data for the overlapping days is included in the
    /// response rows for both date ranges.
    #[prost(message, repeated, tag = "4")]
    pub date_ranges: ::std::vec::Vec<DateRange>,
    /// The row count of the start row. The first row is counted as row 0.
    #[prost(int64, tag = "5")]
    pub offset: i64,
    /// The number of rows to return.
    /// If zero or unspecified, all rows are returned.
    #[prost(int64, tag = "6")]
    pub limit: i64,
    /// Aggregation of metrics. Aggregated metric values will be shown in rows
    /// where the dimension_values are set to "RESERVED_(MetricAggregation)".
    #[prost(enumeration = "MetricAggregation", repeated, tag = "7")]
    pub metric_aggregations: ::std::vec::Vec<i32>,
    /// The filter clause of dimensions.
    #[prost(message, optional, tag = "8")]
    pub dimension_filter: ::std::option::Option<FilterExpression>,
    /// The filter clause of metrics. Applied at post aggregation phase, similar to
    /// SQL having-clause.
    #[prost(message, optional, tag = "9")]
    pub metric_filter: ::std::option::Option<FilterExpression>,
    /// Specifies how rows are ordered in the response.
    #[prost(message, repeated, tag = "10")]
    pub order_bys: ::std::vec::Vec<OrderBy>,
    /// A currency code in ISO4217 format, such as "AED", "USD", "JPY".
    /// If the field is empty, the report uses the entity's default currency.
    #[prost(string, tag = "11")]
    pub currency_code: std::string::String,
    /// Cohort group associated with this request. If there is a cohort group
    /// in the request the 'cohort' dimension must be present.
    #[prost(message, optional, tag = "12")]
    pub cohort_spec: ::std::option::Option<CohortSpec>,
    /// If false, rows with metrics being 0 will not be returned.
    #[prost(bool, tag = "13")]
    pub keep_empty_rows: bool,
    /// Toggles whether to return the current state of this Analytics Property's
    /// quota. Quota is returned in [PropertyQuota](#PropertyQuota).
    #[prost(bool, tag = "14")]
    pub return_property_quota: bool,
}
/// The response report table corresponding to a request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RunReportResponse {
    /// Describes metric columns.
    #[prost(message, repeated, tag = "1")]
    pub metric_headers: ::std::vec::Vec<MetricHeader>,
    /// Rows of dimension value combinations and metric values in the report.
    #[prost(message, repeated, tag = "2")]
    pub rows: ::std::vec::Vec<Row>,
    /// If requested, the totaled values of metrics.
    #[prost(message, repeated, tag = "8")]
    pub totals: ::std::vec::Vec<Row>,
    /// If requested, the maximum values of metrics.
    #[prost(message, repeated, tag = "9")]
    pub maximums: ::std::vec::Vec<Row>,
    /// If requested, the minimum values of metrics.
    #[prost(message, repeated, tag = "10")]
    pub minimums: ::std::vec::Vec<Row>,
    /// Metadata for the report.
    #[prost(message, optional, tag = "6")]
    pub metadata: ::std::option::Option<ResponseMetaData>,
    /// This Analytics Property's quota state including this request.
    #[prost(message, optional, tag = "7")]
    pub property_quota: ::std::option::Option<PropertyQuota>,
}
/// The request to generate a pivot report.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RunPivotReportRequest {
    /// A property whose events are tracked. Within a batch request, this entity
    /// should either be unspecified or consistent with the batch-level entity.
    #[prost(message, optional, tag = "1")]
    pub entity: ::std::option::Option<Entity>,
    /// The dimensions requested. All defined dimensions must be used by one of the
    /// following: dimension_expression, dimension_filter, pivots, order_bys.
    #[prost(message, repeated, tag = "2")]
    pub dimensions: ::std::vec::Vec<Dimension>,
    /// The metrics requested, at least one metric needs to be specified. All
    /// defined metrics must be used by one of the following: metric_expression,
    /// metric_filter, order_bys.
    #[prost(message, repeated, tag = "3")]
    pub metrics: ::std::vec::Vec<Metric>,
    /// The filter clause of dimensions. Requests are validated that all
    /// field_names in the dimension_filter are dimensions and are defined in
    /// Dimensions.
    #[prost(message, optional, tag = "4")]
    pub dimension_filter: ::std::option::Option<FilterExpression>,
    /// The filter clause of metrics. Applied at post aggregation phase, similar to
    /// SQL having-clause. Requests are validated that all field_names in the
    /// metric_filter are metrics and are defined in Metrics.
    #[prost(message, optional, tag = "5")]
    pub metric_filter: ::std::option::Option<FilterExpression>,
    /// Describes the visual format of the report's dimensions in columns or rows.
    /// The union of the fieldNames (dimension names) in all pivots must be a
    /// subset of dimension names defined in Dimensions. No two pivots can share a
    /// dimension. A dimension is only visible if it appears in a pivot.
    #[prost(message, repeated, tag = "6")]
    pub pivots: ::std::vec::Vec<Pivot>,
    /// The date range to retrieve event data for the report. If multiple date
    /// ranges are specified, event data from each date range is used in the
    /// report. A special dimension with field name "dateRange" can be included in
    /// a Pivot's field names; if included, the report compares between date
    /// ranges. This dateRanges field is not used in cohorts reports.
    #[prost(message, repeated, tag = "7")]
    pub date_ranges: ::std::vec::Vec<DateRange>,
    /// A currency code in ISO4217 format, such as "AED", "USD", "JPY".
    /// If the field is empty, the report uses the entity's default currency.
    #[prost(string, tag = "8")]
    pub currency_code: std::string::String,
    /// Cohort group associated with this request. If there is a cohort group
    /// in the request the 'cohort' dimension must be present.
    #[prost(message, optional, tag = "9")]
    pub cohort_spec: ::std::option::Option<CohortSpec>,
    /// If false, rows with metrics being 0 will not be returned.
    #[prost(bool, tag = "10")]
    pub keep_empty_rows: bool,
    /// Toggles whether to return the current state of this Analytics Property's
    /// quota. Quota is returned in [PropertyQuota](#PropertyQuota).
    #[prost(bool, tag = "11")]
    pub return_property_quota: bool,
}
/// The response pivot report table corresponding to a pivot request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RunPivotReportResponse {
    /// Summarizes the columns and rows created by a pivot. Each pivot in the
    /// request produces one header in the response. If we have a request like
    /// this:
    ///
    ///     "pivots": [{
    ///       "fieldNames": ["country",
    ///         "city"]
    ///     },
    ///     {
    ///       "fieldNames": "eventName"
    ///     }]
    ///
    /// We will have the following `pivotHeaders` in the response:
    ///
    ///     "pivotHeaders" : [{
    ///       "dimensionHeaders": [{
    ///         "dimensionValues": [
    ///            { "value": "United Kingdom" },
    ///            { "value": "London" }
    ///          ]
    ///       },
    ///       {
    ///         "dimensionValues": [
    ///         { "value": "Japan" },
    ///         { "value": "Osaka" }
    ///         ]
    ///       }]
    ///     },
    ///     {
    ///       "dimensionHeaders": [{
    ///         "dimensionValues": [{ "value": "session_start" }]
    ///       },
    ///       {
    ///         "dimensionValues": [{ "value": "scroll" }]
    ///       }]
    ///     }]
    #[prost(message, repeated, tag = "1")]
    pub pivot_headers: ::std::vec::Vec<PivotHeader>,
    /// Describes metric columns.
    #[prost(message, repeated, tag = "2")]
    pub metric_headers: ::std::vec::Vec<MetricHeader>,
    /// Rows of dimension value combinations and metric values in the report.
    #[prost(message, repeated, tag = "3")]
    pub rows: ::std::vec::Vec<Row>,
    /// Aggregation of metric values. Can be totals, minimums, or maximums. The
    /// returned aggregations are controlled by the metric_aggregations in the
    /// pivot. The type of aggregation returned in each row is shown by the
    /// dimension_values which are set to "RESERVED_<MetricAggregation>".
    #[prost(message, repeated, tag = "4")]
    pub aggregates: ::std::vec::Vec<Row>,
    /// Metadata for the report.
    #[prost(message, optional, tag = "5")]
    pub metadata: ::std::option::Option<ResponseMetaData>,
    /// This Analytics Property's quota state including this request.
    #[prost(message, optional, tag = "6")]
    pub property_quota: ::std::option::Option<PropertyQuota>,
}
/// The batch request containing multiple report requests.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchRunReportsRequest {
    /// A property whose events are tracked. This entity must be specified for the
    /// batch. The entity within RunReportRequest may either be unspecified or
    /// consistent with this entity.
    #[prost(message, optional, tag = "1")]
    pub entity: ::std::option::Option<Entity>,
    /// Individual requests. Each request has a separate report response. Each
    /// batch request is allowed up to 5 requests.
    #[prost(message, repeated, tag = "2")]
    pub requests: ::std::vec::Vec<RunReportRequest>,
}
/// The batch response containing multiple reports.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchRunReportsResponse {
    /// Individual responses. Each response has a separate report request.
    #[prost(message, repeated, tag = "1")]
    pub reports: ::std::vec::Vec<RunReportResponse>,
}
/// The batch request containing multiple pivot report requests.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchRunPivotReportsRequest {
    /// A property whose events are tracked. This entity must be specified for the
    /// batch. The entity within RunPivotReportRequest may either be unspecified or
    /// consistent with this entity.
    #[prost(message, optional, tag = "1")]
    pub entity: ::std::option::Option<Entity>,
    /// Individual requests. Each request has a separate pivot report response.
    /// Each batch request is allowed up to 5 requests.
    #[prost(message, repeated, tag = "2")]
    pub requests: ::std::vec::Vec<RunPivotReportRequest>,
}
/// The batch response containing multiple pivot reports.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchRunPivotReportsResponse {
    /// Individual responses. Each response has a separate pivot report request.
    #[prost(message, repeated, tag = "1")]
    pub pivot_reports: ::std::vec::Vec<RunPivotReportResponse>,
}
#[doc = r" Generated client implementations."]
pub mod alpha_analytics_data_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Google Analytics reporting data service."]
    pub struct AlphaAnalyticsDataClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> AlphaAnalyticsDataClient<T>
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
        #[doc = " Returns a customized report of your Google Analytics event data. Reports"]
        #[doc = " contain statistics derived from data collected by the Google Analytics"]
        #[doc = " tracking code. The data returned from the API is as a table with columns"]
        #[doc = " for the requested dimensions and metrics. Metrics are individual"]
        #[doc = " measurements of user activity on your property, such as active users or"]
        #[doc = " event count. Dimensions break down metrics across some common criteria,"]
        #[doc = " such as country or event name."]
        pub async fn run_report(
            &mut self,
            request: impl tonic::IntoRequest<super::RunReportRequest>,
        ) -> Result<tonic::Response<super::RunReportResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.data.v1alpha.AlphaAnalyticsData/RunReport",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns a customized pivot report of your Google Analytics event data."]
        #[doc = " Pivot reports are more advanced and expressive formats than regular"]
        #[doc = " reports. In a pivot report, dimensions are only visible if they are"]
        #[doc = " included in a pivot. Multiple pivots can be specified to further dissect"]
        #[doc = " your data."]
        pub async fn run_pivot_report(
            &mut self,
            request: impl tonic::IntoRequest<super::RunPivotReportRequest>,
        ) -> Result<tonic::Response<super::RunPivotReportResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.data.v1alpha.AlphaAnalyticsData/RunPivotReport",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns multiple reports in a batch. All reports must be for the same"]
        #[doc = " Entity."]
        pub async fn batch_run_reports(
            &mut self,
            request: impl tonic::IntoRequest<super::BatchRunReportsRequest>,
        ) -> Result<tonic::Response<super::BatchRunReportsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.data.v1alpha.AlphaAnalyticsData/BatchRunReports",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns multiple pivot reports in a batch. All reports must be for the same"]
        #[doc = " Entity."]
        pub async fn batch_run_pivot_reports(
            &mut self,
            request: impl tonic::IntoRequest<super::BatchRunPivotReportsRequest>,
        ) -> Result<tonic::Response<super::BatchRunPivotReportsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.data.v1alpha.AlphaAnalyticsData/BatchRunPivotReports",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for AlphaAnalyticsDataClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for AlphaAnalyticsDataClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "AlphaAnalyticsDataClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod alpha_analytics_data_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with AlphaAnalyticsDataServer."]
    #[async_trait]
    pub trait AlphaAnalyticsData: Send + Sync + 'static {
        #[doc = " Returns a customized report of your Google Analytics event data. Reports"]
        #[doc = " contain statistics derived from data collected by the Google Analytics"]
        #[doc = " tracking code. The data returned from the API is as a table with columns"]
        #[doc = " for the requested dimensions and metrics. Metrics are individual"]
        #[doc = " measurements of user activity on your property, such as active users or"]
        #[doc = " event count. Dimensions break down metrics across some common criteria,"]
        #[doc = " such as country or event name."]
        async fn run_report(
            &self,
            request: tonic::Request<super::RunReportRequest>,
        ) -> Result<tonic::Response<super::RunReportResponse>, tonic::Status>;
        #[doc = " Returns a customized pivot report of your Google Analytics event data."]
        #[doc = " Pivot reports are more advanced and expressive formats than regular"]
        #[doc = " reports. In a pivot report, dimensions are only visible if they are"]
        #[doc = " included in a pivot. Multiple pivots can be specified to further dissect"]
        #[doc = " your data."]
        async fn run_pivot_report(
            &self,
            request: tonic::Request<super::RunPivotReportRequest>,
        ) -> Result<tonic::Response<super::RunPivotReportResponse>, tonic::Status>;
        #[doc = " Returns multiple reports in a batch. All reports must be for the same"]
        #[doc = " Entity."]
        async fn batch_run_reports(
            &self,
            request: tonic::Request<super::BatchRunReportsRequest>,
        ) -> Result<tonic::Response<super::BatchRunReportsResponse>, tonic::Status>;
        #[doc = " Returns multiple pivot reports in a batch. All reports must be for the same"]
        #[doc = " Entity."]
        async fn batch_run_pivot_reports(
            &self,
            request: tonic::Request<super::BatchRunPivotReportsRequest>,
        ) -> Result<tonic::Response<super::BatchRunPivotReportsResponse>, tonic::Status>;
    }
    #[doc = " Google Analytics reporting data service."]
    #[derive(Debug)]
    pub struct AlphaAnalyticsDataServer<T: AlphaAnalyticsData> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: AlphaAnalyticsData> AlphaAnalyticsDataServer<T> {
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
    impl<T, B> Service<http::Request<B>> for AlphaAnalyticsDataServer<T>
    where
        T: AlphaAnalyticsData,
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
                "/google.analytics.data.v1alpha.AlphaAnalyticsData/RunReport" => {
                    #[allow(non_camel_case_types)]
                    struct RunReportSvc<T: AlphaAnalyticsData>(pub Arc<T>);
                    impl<T: AlphaAnalyticsData> tonic::server::UnaryService<super::RunReportRequest>
                        for RunReportSvc<T>
                    {
                        type Response = super::RunReportResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RunReportRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).run_report(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = RunReportSvc(inner);
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
                "/google.analytics.data.v1alpha.AlphaAnalyticsData/RunPivotReport" => {
                    #[allow(non_camel_case_types)]
                    struct RunPivotReportSvc<T: AlphaAnalyticsData>(pub Arc<T>);
                    impl<T: AlphaAnalyticsData>
                        tonic::server::UnaryService<super::RunPivotReportRequest>
                        for RunPivotReportSvc<T>
                    {
                        type Response = super::RunPivotReportResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RunPivotReportRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).run_pivot_report(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = RunPivotReportSvc(inner);
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
                "/google.analytics.data.v1alpha.AlphaAnalyticsData/BatchRunReports" => {
                    #[allow(non_camel_case_types)]
                    struct BatchRunReportsSvc<T: AlphaAnalyticsData>(pub Arc<T>);
                    impl<T: AlphaAnalyticsData>
                        tonic::server::UnaryService<super::BatchRunReportsRequest>
                        for BatchRunReportsSvc<T>
                    {
                        type Response = super::BatchRunReportsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BatchRunReportsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).batch_run_reports(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = BatchRunReportsSvc(inner);
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
                "/google.analytics.data.v1alpha.AlphaAnalyticsData/BatchRunPivotReports" => {
                    #[allow(non_camel_case_types)]
                    struct BatchRunPivotReportsSvc<T: AlphaAnalyticsData>(pub Arc<T>);
                    impl<T: AlphaAnalyticsData>
                        tonic::server::UnaryService<super::BatchRunPivotReportsRequest>
                        for BatchRunPivotReportsSvc<T>
                    {
                        type Response = super::BatchRunPivotReportsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BatchRunPivotReportsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).batch_run_pivot_reports(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = BatchRunPivotReportsSvc(inner);
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
    impl<T: AlphaAnalyticsData> Clone for AlphaAnalyticsDataServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: AlphaAnalyticsData> Clone for _Inner<T> {
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
