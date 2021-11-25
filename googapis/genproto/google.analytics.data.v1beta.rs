/// A contiguous set of days: startDate, startDate + 1, ..., endDate. Requests
/// are allowed up to 4 date ranges.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DateRange {
    /// The inclusive start date for the query in the format `YYYY-MM-DD`. Cannot
    /// be after `end_date`. The format `NdaysAgo`, `yesterday`, or `today` is also
    /// accepted, and in that case, the date is inferred based on the property's
    /// reporting time zone.
    #[prost(string, tag = "1")]
    pub start_date: ::prost::alloc::string::String,
    /// The inclusive end date for the query in the format `YYYY-MM-DD`. Cannot
    /// be before `start_date`. The format `NdaysAgo`, `yesterday`, or `today` is
    /// also accepted, and in that case, the date is inferred based on the
    /// property's reporting time zone.
    #[prost(string, tag = "2")]
    pub end_date: ::prost::alloc::string::String,
    /// Assigns a name to this date range. The dimension `dateRange` is valued to
    /// this name in a report response. If set, cannot begin with `date_range_` or
    /// `RESERVED_`. If not set, date ranges are named by their zero based index in
    /// the request: `date_range_0`, `date_range_1`, etc.
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
}
/// A contiguous set of minutes: startMinutesAgo, startMinutesAgo + 1, ...,
/// endMinutesAgo. Requests are allowed up to 2 minute ranges.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MinuteRange {
    /// The inclusive start minute for the query as a number of minutes before now.
    /// For example, `"startMinutesAgo": 29` specifies the report should include
    /// event data from 29 minutes ago and after. Cannot be after `endMinutesAgo`.
    ///
    /// If unspecified, `startMinutesAgo` is defaulted to 29. Standard Analytics
    /// properties can request up to the last 30 minutes of event data
    /// (`startMinutesAgo <= 29`), and 360 Analytics properties can request up to
    /// the last 60 minutes of event data (`startMinutesAgo <= 59`).
    #[prost(int32, optional, tag = "1")]
    pub start_minutes_ago: ::core::option::Option<i32>,
    /// The inclusive end minute for the query as a number of minutes before now.
    /// Cannot be before `startMinutesAgo`. For example, `"endMinutesAgo": 15`
    /// specifies the report should include event data from prior to 15 minutes
    /// ago.
    ///
    /// If unspecified, `endMinutesAgo` is defaulted to 0. Standard Analytics
    /// properties can request any minute in the last 30 minutes of event data
    /// (`endMinutesAgo <= 29`), and 360 Analytics properties can request any
    /// minute in the last 60 minutes of event data (`endMinutesAgo <= 59`).
    #[prost(int32, optional, tag = "2")]
    pub end_minutes_ago: ::core::option::Option<i32>,
    /// Assigns a name to this minute range. The dimension `dateRange` is valued to
    /// this name in a report response. If set, cannot begin with `date_range_` or
    /// `RESERVED_`. If not set, minute ranges are named by their zero based index
    /// in the request: `date_range_0`, `date_range_1`, etc.
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
}
/// Dimensions are attributes of your data. For example, the dimension city
/// indicates the city from which an event originates. Dimension values in report
/// responses are strings; for example, city could be "Paris" or "New York".
/// Requests are allowed up to 9 dimensions.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Dimension {
    /// The name of the dimension. See the [API
    /// Dimensions](<https://developers.google.com/analytics/devguides/reporting/data/v1/api-schema#dimensions>)
    /// for the list of dimension names.
    ///
    /// If `dimensionExpression` is specified, `name` can be any string that you
    /// would like within the allowed character set. For example if a
    /// `dimensionExpression` concatenates `country` and `city`, you could call
    /// that dimension `countryAndCity`. Dimension names that you choose must match
    /// the regular expression `^\[a-zA-Z0-9_\]$`.
    ///
    /// Dimensions are referenced by `name` in `dimensionFilter`, `orderBys`,
    /// `dimensionExpression`, and `pivots`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// One dimension can be the result of an expression of multiple dimensions.
    /// For example, dimension "country, city": concatenate(country, ", ", city).
    #[prost(message, optional, tag = "2")]
    pub dimension_expression: ::core::option::Option<DimensionExpression>,
}
/// Used to express a dimension which is the result of a formula of multiple
/// dimensions. Example usages:
/// 1) lower_case(dimension)
/// 2) concatenate(dimension1, symbol, dimension2).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DimensionExpression {
    /// Specify one type of dimension expression for `DimensionExpression`.
    #[prost(oneof = "dimension_expression::OneExpression", tags = "4, 5, 6")]
    pub one_expression: ::core::option::Option<dimension_expression::OneExpression>,
}
/// Nested message and enum types in `DimensionExpression`.
pub mod dimension_expression {
    /// Used to convert a dimension value to a single case.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CaseExpression {
        /// Name of a dimension. The name must refer back to a name in dimensions
        /// field of the request.
        #[prost(string, tag = "1")]
        pub dimension_name: ::prost::alloc::string::String,
    }
    /// Used to combine dimension values to a single dimension.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ConcatenateExpression {
        /// Names of dimensions. The names must refer back to names in the dimensions
        /// field of the request.
        #[prost(string, repeated, tag = "1")]
        pub dimension_names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// The delimiter placed between dimension names.
        ///
        /// Delimiters are often single characters such as "|" or "," but can be
        /// longer strings. If a dimension value contains the delimiter, both will be
        /// present in response with no distinction. For example if dimension 1 value
        /// = "US,FR", dimension 2 value = "JP", and delimiter = ",", then the
        /// response will contain "US,FR,JP".
        #[prost(string, tag = "2")]
        pub delimiter: ::prost::alloc::string::String,
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
/// The quantitative measurements of a report. For example, the metric
/// `eventCount` is the total number of events. Requests are allowed up to 10
/// metrics.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Metric {
    /// The name of the metric. See the [API
    /// Metrics](<https://developers.google.com/analytics/devguides/reporting/data/v1/api-schema#metrics>)
    /// for the list of metric names.
    ///
    /// If `expression` is specified, `name` can be any string that you would like
    /// within the allowed character set. For example if `expression` is
    /// `screenPageViews/sessions`, you could call that metric's name =
    /// `viewsPerSession`. Metric names that you choose must match the regular
    /// expression `^\[a-zA-Z0-9_\]$`.
    ///
    /// Metrics are referenced by `name` in `metricFilter`, `orderBys`, and metric
    /// `expression`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// A mathematical expression for derived metrics. For example, the metric
    /// Event count per user is `eventCount/totalUsers`.
    #[prost(string, tag = "2")]
    pub expression: ::prost::alloc::string::String,
    /// Indicates if a metric is invisible in the report response. If a metric is
    /// invisible, the metric will not produce a column in the response, but can be
    /// used in `metricFilter`, `orderBys`, or a metric `expression`.
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
    pub expr: ::core::option::Option<filter_expression::Expr>,
}
/// Nested message and enum types in `FilterExpression`.
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
        NotExpression(::prost::alloc::boxed::Box<super::FilterExpression>),
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
    pub expressions: ::prost::alloc::vec::Vec<FilterExpression>,
}
/// An expression to filter dimension or metric values.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Filter {
    /// The dimension name or metric name. Must be a name defined in dimensions
    /// or metrics.
    #[prost(string, tag = "1")]
    pub field_name: ::prost::alloc::string::String,
    /// Specify one type of filter for `Filter`.
    #[prost(oneof = "filter::OneFilter", tags = "3, 4, 5, 6")]
    pub one_filter: ::core::option::Option<filter::OneFilter>,
}
/// Nested message and enum types in `Filter`.
pub mod filter {
    /// The filter for string
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct StringFilter {
        /// The match type for this filter.
        #[prost(enumeration = "string_filter::MatchType", tag = "1")]
        pub match_type: i32,
        /// The string value used for the matching.
        #[prost(string, tag = "2")]
        pub value: ::prost::alloc::string::String,
        /// If true, the string value is case sensitive.
        #[prost(bool, tag = "3")]
        pub case_sensitive: bool,
    }
    /// Nested message and enum types in `StringFilter`.
    pub mod string_filter {
        /// The match type of a string filter
        #[derive(
            Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration,
        )]
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
        pub values: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
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
        pub value: ::core::option::Option<super::NumericValue>,
    }
    /// Nested message and enum types in `NumericFilter`.
    pub mod numeric_filter {
        /// The operation applied to a numeric filter
        #[derive(
            Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration,
        )]
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
        pub from_value: ::core::option::Option<super::NumericValue>,
        /// Ends with this number.
        #[prost(message, optional, tag = "2")]
        pub to_value: ::core::option::Option<super::NumericValue>,
    }
    /// Specify one type of filter for `Filter`.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum OneFilter {
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
    pub one_order_by: ::core::option::Option<order_by::OneOrderBy>,
}
/// Nested message and enum types in `OrderBy`.
pub mod order_by {
    /// Sorts by metric values.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct MetricOrderBy {
        /// A metric name in the request to order by.
        #[prost(string, tag = "1")]
        pub metric_name: ::prost::alloc::string::String,
    }
    /// Sorts by dimension values.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DimensionOrderBy {
        /// A dimension name in the request to order by.
        #[prost(string, tag = "1")]
        pub dimension_name: ::prost::alloc::string::String,
        /// Controls the rule for dimension value ordering.
        #[prost(enumeration = "dimension_order_by::OrderType", tag = "2")]
        pub order_type: i32,
    }
    /// Nested message and enum types in `DimensionOrderBy`.
    pub mod dimension_order_by {
        /// Rule to order the string dimension values by.
        #[derive(
            Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration,
        )]
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
        pub metric_name: ::prost::alloc::string::String,
        /// Used to select a dimension name and value pivot. If multiple pivot
        /// selections are given, the sort occurs on rows where all pivot selection
        /// dimension name and value pairs match the row's dimension name and value
        /// pair.
        #[prost(message, repeated, tag = "2")]
        pub pivot_selections: ::prost::alloc::vec::Vec<pivot_order_by::PivotSelection>,
    }
    /// Nested message and enum types in `PivotOrderBy`.
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
            pub dimension_name: ::prost::alloc::string::String,
            /// Order by only when the named dimension is this value.
            #[prost(string, tag = "2")]
            pub dimension_value: ::prost::alloc::string::String,
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
    pub field_names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Specifies how dimensions are ordered in the pivot. In the first Pivot, the
    /// OrderBys determine Row and PivotDimensionHeader ordering; in subsequent
    /// Pivots, the OrderBys determine only PivotDimensionHeader ordering.
    /// Dimensions specified in these OrderBys must be a subset of
    /// Pivot.field_names.
    #[prost(message, repeated, tag = "2")]
    pub order_bys: ::prost::alloc::vec::Vec<OrderBy>,
    /// The row count of the start row. The first row is counted as row 0.
    #[prost(int64, tag = "3")]
    pub offset: i64,
    /// The number of unique combinations of dimension values to return in this
    /// pivot. The `limit` parameter is required. A `limit` of 10,000 is common for
    /// single pivot requests.
    ///
    /// The product of the `limit` for each `pivot` in a `RunPivotReportRequest`
    /// must not exceed 100,000. For example, a two pivot request with `limit:
    /// 1000` in each pivot will fail because the product is `1,000,000`.
    #[prost(int64, tag = "4")]
    pub limit: i64,
    /// Aggregate the metrics by dimensions in this pivot using the specified
    /// metric_aggregations.
    #[prost(enumeration = "MetricAggregation", repeated, tag = "5")]
    pub metric_aggregations: ::prost::alloc::vec::Vec<i32>,
}
/// The specification of cohorts for a cohort report.
///
/// Cohort reports create a time series of user retention for the cohort. For
/// example, you could select the cohort of users that were acquired in the first
/// week of September and follow that cohort for the next six weeks. Selecting
/// the users acquired in the first week of September cohort is specified in the
/// `cohort` object. Following that cohort for the next six weeks is specified in
/// the `cohortsRange` object.
///
/// For examples, see [Cohort Report
/// Examples](<https://developers.google.com/analytics/devguides/reporting/data/v1/advanced#cohort_report_examples>).
///
/// The report response could show a weekly time series where say your app has
/// retained 60% of this cohort after three weeks and 25% of this cohort after
/// six weeks. These two percentages can be calculated by the metric
/// `cohortActiveUsers/cohortTotalUsers` and will be separate rows in the report.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CohortSpec {
    /// Defines the selection criteria to group users into cohorts.
    ///
    /// Most cohort reports define only a single cohort. If multiple cohorts are
    /// specified, each cohort can be recognized in the report by their name.
    #[prost(message, repeated, tag = "1")]
    pub cohorts: ::prost::alloc::vec::Vec<Cohort>,
    /// Cohort reports follow cohorts over an extended reporting date range. This
    /// range specifies an offset duration to follow the cohorts over.
    #[prost(message, optional, tag = "2")]
    pub cohorts_range: ::core::option::Option<CohortsRange>,
    /// Optional settings for a cohort report.
    #[prost(message, optional, tag = "3")]
    pub cohort_report_settings: ::core::option::Option<CohortReportSettings>,
}
/// Defines a cohort selection criteria. A cohort is a group of users who share
/// a common characteristic. For example, users with the same `firstSessionDate`
/// belong to the same cohort.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Cohort {
    /// Assigns a name to this cohort. The dimension `cohort` is valued to this
    /// name in a report response. If set, cannot begin with `cohort_` or
    /// `RESERVED_`. If not set, cohorts are named by their zero based index
    /// `cohort_0`, `cohort_1`, etc.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Dimension used by the cohort. Required and only supports
    /// `firstSessionDate`.
    #[prost(string, tag = "2")]
    pub dimension: ::prost::alloc::string::String,
    /// The cohort selects users whose first touch date is between start date and
    /// end date defined in the `dateRange`. This `dateRange` does not specify the
    /// full date range of event data that is present in a cohort report. In a
    /// cohort report, this `dateRange` is extended by the granularity and offset
    /// present in the `cohortsRange`; event data for the extended reporting date
    /// range is present in a cohort report.
    ///
    /// In a cohort request, this `dateRange` is required and the `dateRanges` in
    /// the `RunReportRequest` or `RunPivotReportRequest` must be unspecified.
    ///
    /// This `dateRange` should generally be aligned with the cohort's granularity.
    /// If `CohortsRange` uses daily granularity, this `dateRange` can be a single
    /// day. If `CohortsRange` uses weekly granularity, this `dateRange` can be
    /// aligned to a week boundary, starting at Sunday and ending Saturday. If
    /// `CohortsRange` uses monthly granularity, this `dateRange` can be aligned to
    /// a month, starting at the first and ending on the last day of the month.
    #[prost(message, optional, tag = "3")]
    pub date_range: ::core::option::Option<DateRange>,
}
/// Configures the extended reporting date range for a cohort report. Specifies
/// an offset duration to follow the cohorts over.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CohortsRange {
    /// Required. The granularity used to interpret the `startOffset` and
    /// `endOffset` for the extended reporting date range for a cohort report.
    #[prost(enumeration = "cohorts_range::Granularity", tag = "1")]
    pub granularity: i32,
    /// `startOffset` specifies the start date of the extended reporting date range
    /// for a cohort report. `startOffset` is commonly set to 0 so that reports
    /// contain data from the acquisition of the cohort forward.
    ///
    /// If `granularity` is `DAILY`, the `startDate` of the extended reporting date
    /// range is `startDate` of the cohort plus `startOffset` days.
    ///
    /// If `granularity` is `WEEKLY`, the `startDate` of the extended reporting
    /// date range is `startDate` of the cohort plus `startOffset * 7` days.
    ///
    /// If `granularity` is `MONTHLY`, the `startDate` of the extended reporting
    /// date range is `startDate` of the cohort plus `startOffset * 30` days.
    #[prost(int32, tag = "2")]
    pub start_offset: i32,
    /// Required. `endOffset` specifies the end date of the extended reporting date
    /// range for a cohort report. `endOffset` can be any positive integer but is
    /// commonly set to 5 to 10 so that reports contain data on the cohort for the
    /// next several granularity time periods.
    ///
    /// If `granularity` is `DAILY`, the `endDate` of the extended reporting date
    /// range is `endDate` of the cohort plus `endOffset` days.
    ///
    /// If `granularity` is `WEEKLY`, the `endDate` of the extended reporting date
    /// range is `endDate` of the cohort plus `endOffset * 7` days.
    ///
    /// If `granularity` is `MONTHLY`, the `endDate` of the extended reporting date
    /// range is `endDate` of the cohort plus `endOffset * 30` days.
    #[prost(int32, tag = "3")]
    pub end_offset: i32,
}
/// Nested message and enum types in `CohortsRange`.
pub mod cohorts_range {
    /// The granularity used to interpret the `startOffset` and `endOffset` for the
    /// extended reporting date range for a cohort report.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Granularity {
        /// Should never be specified.
        Unspecified = 0,
        /// Daily granularity. Commonly used if the cohort's `dateRange` is a single
        /// day and the request contains `cohortNthDay`.
        Daily = 1,
        /// Weekly granularity. Commonly used if the cohort's `dateRange` is a week
        /// in duration (starting on Sunday and ending on Saturday) and the request
        /// contains `cohortNthWeek`.
        Weekly = 2,
        /// Monthly granularity. Commonly used if the cohort's `dateRange` is a month
        /// in duration and the request contains `cohortNthMonth`.
        Monthly = 3,
    }
}
/// Optional settings of a cohort report.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CohortReportSettings {
    /// If true, accumulates the result from first touch day to the end day. Not
    /// supported in `RunReportRequest`.
    #[prost(bool, tag = "1")]
    pub accumulate: bool,
}
/// Response's metadata carrying additional information about the report content.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseMetaData {
    /// If true, indicates some buckets of dimension combinations are rolled into
    /// "(other)" row. This can happen for high cardinality reports.
    #[prost(bool, tag = "3")]
    pub data_loss_from_other_row: bool,
    /// Describes the schema restrictions actively enforced in creating this
    /// report. To learn more, see [Access and data-restriction
    /// management](<https://support.google.com/analytics/answer/10851388>).
    #[prost(message, optional, tag = "4")]
    pub schema_restriction_response:
        ::core::option::Option<response_meta_data::SchemaRestrictionResponse>,
    /// The currency code used in this report. Intended to be used in formatting
    /// currency metrics like `purchaseRevenue` for visualization. If currency_code
    /// was specified in the request, this response parameter will echo the request
    /// parameter; otherwise, this response parameter is the property's current
    /// currency_code.
    ///
    /// Currency codes are string encodings of currency types from the ISO 4217
    /// standard (<https://en.wikipedia.org/wiki/ISO_4217>); for example "USD",
    /// "EUR", "JPY". To learn more, see
    /// <https://support.google.com/analytics/answer/9796179.>
    #[prost(string, optional, tag = "5")]
    pub currency_code: ::core::option::Option<::prost::alloc::string::String>,
    /// The property's current timezone. Intended to be used to interpret
    /// time-based dimensions like `hour` and `minute`. Formatted as strings from
    /// the IANA Time Zone database (<https://www.iana.org/time-zones>); for example
    /// "America/New_York" or "Asia/Tokyo".
    #[prost(string, optional, tag = "6")]
    pub time_zone: ::core::option::Option<::prost::alloc::string::String>,
    /// If empty reason is specified, the report is empty for this reason.
    #[prost(string, optional, tag = "7")]
    pub empty_reason: ::core::option::Option<::prost::alloc::string::String>,
}
/// Nested message and enum types in `ResponseMetaData`.
pub mod response_meta_data {
    /// The schema restrictions actively enforced in creating this report. To learn
    /// more, see [Access and data-restriction
    /// management](<https://support.google.com/analytics/answer/10851388>).
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SchemaRestrictionResponse {
        /// All restrictions actively enforced in creating the report. For example,
        /// `purchaseRevenue` always has the restriction type `REVENUE_DATA`.
        /// However, this active response restriction is only populated if the user's
        /// custom role disallows access to `REVENUE_DATA`.
        #[prost(message, repeated, tag = "1")]
        pub active_metric_restrictions:
            ::prost::alloc::vec::Vec<schema_restriction_response::ActiveMetricRestriction>,
    }
    /// Nested message and enum types in `SchemaRestrictionResponse`.
    pub mod schema_restriction_response {
        /// A metric actively restricted in creating the report.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ActiveMetricRestriction {
            /// The name of the restricted metric.
            #[prost(string, optional, tag = "1")]
            pub metric_name: ::core::option::Option<::prost::alloc::string::String>,
            /// The reason for this metric's restriction.
            #[prost(enumeration = "super::super::RestrictedMetricType", repeated, tag = "2")]
            pub restricted_metric_types: ::prost::alloc::vec::Vec<i32>,
        }
    }
}
/// Describes a dimension column in the report. Dimensions requested in a report
/// produce column entries within rows and DimensionHeaders. However, dimensions
/// used exclusively within filters or expressions do not produce columns in a
/// report; correspondingly, those dimensions do not produce headers.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DimensionHeader {
    /// The dimension's name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Describes a metric column in the report. Visible metrics requested in a
/// report produce column entries within rows and MetricHeaders. However,
/// metrics used exclusively within filters or expressions do not produce columns
/// in a report; correspondingly, those metrics do not produce headers.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MetricHeader {
    /// The metric's name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The metric's data type.
    #[prost(enumeration = "MetricType", tag = "2")]
    pub r#type: i32,
}
/// Dimensions' values in a single pivot.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PivotHeader {
    /// The size is the same as the cardinality of the corresponding dimension
    /// combinations.
    #[prost(message, repeated, tag = "1")]
    pub pivot_dimension_headers: ::prost::alloc::vec::Vec<PivotDimensionHeader>,
    /// The cardinality of the pivot. The total number of rows for this pivot's
    /// fields regardless of how the parameters `offset` and `limit` are specified
    /// in the request.
    #[prost(int32, tag = "2")]
    pub row_count: i32,
}
/// Summarizes dimension values from a row for this pivot.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PivotDimensionHeader {
    /// Values of multiple dimensions in a pivot.
    #[prost(message, repeated, tag = "1")]
    pub dimension_values: ::prost::alloc::vec::Vec<DimensionValue>,
}
/// Report data for each row.
/// For example if RunReportRequest contains:
///
/// ```none
/// "dimensions": [
///   {
///     "name": "eventName"
///   },
///   {
///     "name": "countryId"
///   }
/// ],
/// "metrics": [
///   {
///     "name": "eventCount"
///   }
/// ]
/// ```
///
/// One row with 'in_app_purchase' as the eventName, 'JP' as the countryId, and
/// 15 as the eventCount, would be:
///
/// ```none
/// "dimensionValues": [
///   {
///     "value": "in_app_purchase"
///   },
///   {
///     "value": "JP"
///   }
/// ],
/// "metricValues": [
///   {
///     "value": "15"
///   }
/// ]
/// ```
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Row {
    /// List of requested dimension values. In a PivotReport, dimension_values
    /// are only listed for dimensions included in a pivot.
    #[prost(message, repeated, tag = "1")]
    pub dimension_values: ::prost::alloc::vec::Vec<DimensionValue>,
    /// List of requested visible metric values.
    #[prost(message, repeated, tag = "2")]
    pub metric_values: ::prost::alloc::vec::Vec<MetricValue>,
}
/// The value of a dimension.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DimensionValue {
    /// One kind of dimension value
    #[prost(oneof = "dimension_value::OneValue", tags = "1")]
    pub one_value: ::core::option::Option<dimension_value::OneValue>,
}
/// Nested message and enum types in `DimensionValue`.
pub mod dimension_value {
    /// One kind of dimension value
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum OneValue {
        /// Value as a string if the dimension type is a string.
        #[prost(string, tag = "1")]
        Value(::prost::alloc::string::String),
    }
}
/// The value of a metric.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MetricValue {
    /// One of metric value
    #[prost(oneof = "metric_value::OneValue", tags = "4")]
    pub one_value: ::core::option::Option<metric_value::OneValue>,
}
/// Nested message and enum types in `MetricValue`.
pub mod metric_value {
    /// One of metric value
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum OneValue {
        /// Measurement value. See MetricHeader for type.
        #[prost(string, tag = "4")]
        Value(::prost::alloc::string::String),
    }
}
/// To represent a number.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NumericValue {
    /// One of a numeric value
    #[prost(oneof = "numeric_value::OneValue", tags = "1, 2")]
    pub one_value: ::core::option::Option<numeric_value::OneValue>,
}
/// Nested message and enum types in `NumericValue`.
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
    /// Standard Analytics Properties can use up to 25,000 tokens per day;
    /// Analytics 360 Properties can use 250,000 tokens per day. Most requests
    /// consume fewer than 10 tokens.
    #[prost(message, optional, tag = "1")]
    pub tokens_per_day: ::core::option::Option<QuotaStatus>,
    /// Standard Analytics Properties can use up to 5,000 tokens per hour;
    /// Analytics 360 Properties can use 50,000 tokens per hour. An API request
    /// consumes a single number of tokens, and that number is deducted from both
    /// the hourly and daily quotas.
    #[prost(message, optional, tag = "2")]
    pub tokens_per_hour: ::core::option::Option<QuotaStatus>,
    /// Standard Analytics Properties can send up to 10 concurrent requests;
    /// Analytics 360 Properties can use up to 50 concurrent requests.
    #[prost(message, optional, tag = "3")]
    pub concurrent_requests: ::core::option::Option<QuotaStatus>,
    /// Standard Analytics Properties and cloud project pairs can have up to 10
    /// server errors per hour; Analytics 360 Properties and cloud project pairs
    /// can have up to 50 server errors per hour.
    #[prost(message, optional, tag = "4")]
    pub server_errors_per_project_per_hour: ::core::option::Option<QuotaStatus>,
    /// Analytics Properties can send up to 120 requests with potentially
    /// thresholded dimensions per hour. In a batch request, each report request
    /// is individually counted for this quota if the request contains potentially
    /// thresholded dimensions.
    #[prost(message, optional, tag = "5")]
    pub potentially_thresholded_requests_per_hour: ::core::option::Option<QuotaStatus>,
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
/// Explains a dimension.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DimensionMetadata {
    /// This dimension's name. Useable in \[Dimension\](#Dimension)'s `name`. For
    /// example, `eventName`.
    #[prost(string, tag = "1")]
    pub api_name: ::prost::alloc::string::String,
    /// This dimension's name within the Google Analytics user interface. For
    /// example, `Event name`.
    #[prost(string, tag = "2")]
    pub ui_name: ::prost::alloc::string::String,
    /// Description of how this dimension is used and calculated.
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    /// Still usable but deprecated names for this dimension. If populated, this
    /// dimension is available by either `apiName` or one of `deprecatedApiNames`
    /// for a period of time. After the deprecation period, the dimension will be
    /// available only by `apiName`.
    #[prost(string, repeated, tag = "4")]
    pub deprecated_api_names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// True if the dimension is a custom dimension for this property.
    #[prost(bool, tag = "5")]
    pub custom_definition: bool,
    /// The display name of the category that this dimension belongs to. Similar
    /// dimensions and metrics are categorized together.
    #[prost(string, tag = "7")]
    pub category: ::prost::alloc::string::String,
}
/// Explains a metric.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MetricMetadata {
    /// A metric name. Useable in \[Metric\](#Metric)'s `name`. For example,
    /// `eventCount`.
    #[prost(string, tag = "1")]
    pub api_name: ::prost::alloc::string::String,
    /// This metric's name within the Google Analytics user interface. For example,
    /// `Event count`.
    #[prost(string, tag = "2")]
    pub ui_name: ::prost::alloc::string::String,
    /// Description of how this metric is used and calculated.
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    /// Still usable but deprecated names for this metric. If populated, this
    /// metric is available by either `apiName` or one of `deprecatedApiNames`
    /// for a period of time. After the deprecation period, the metric will be
    /// available only by `apiName`.
    #[prost(string, repeated, tag = "4")]
    pub deprecated_api_names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The type of this metric.
    #[prost(enumeration = "MetricType", tag = "5")]
    pub r#type: i32,
    /// The mathematical expression for this derived metric. Can be used in
    /// \[Metric\](#Metric)'s `expression` field for equivalent reports. Most metrics
    /// are not expressions, and for non-expressions, this field is empty.
    #[prost(string, tag = "6")]
    pub expression: ::prost::alloc::string::String,
    /// True if the metric is a custom metric for this property.
    #[prost(bool, tag = "7")]
    pub custom_definition: bool,
    /// If reasons are specified, your access is blocked to this metric for this
    /// property. API requests from you to this property for this metric will
    /// succeed; however, the report will contain only zeros for this metric. API
    /// requests with metric filters on blocked metrics will fail. If reasons are
    /// empty, you have access to this metric.
    ///
    /// To learn more, see [Access and data-restriction
    /// management](<https://support.google.com/analytics/answer/10851388>).
    #[prost(enumeration = "metric_metadata::BlockedReason", repeated, tag = "8")]
    pub blocked_reasons: ::prost::alloc::vec::Vec<i32>,
    /// The display name of the category that this metrics belongs to. Similar
    /// dimensions and metrics are categorized together.
    #[prost(string, tag = "10")]
    pub category: ::prost::alloc::string::String,
}
/// Nested message and enum types in `MetricMetadata`.
pub mod metric_metadata {
    /// Justifications for why this metric is blocked.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum BlockedReason {
        /// Will never be specified in API response.
        Unspecified = 0,
        /// If present, your access is blocked to revenue related metrics for this
        /// property, and this metric is revenue related.
        NoRevenueMetrics = 1,
        /// If present, your access is blocked to cost related metrics for this
        /// property, and this metric is cost related.
        NoCostMetrics = 2,
    }
}
/// The compatibility for a single dimension.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DimensionCompatibility {
    /// The dimension metadata contains the API name for this compatibility
    /// information. The dimension metadata also contains other helpful information
    /// like the UI name and description.
    #[prost(message, optional, tag = "1")]
    pub dimension_metadata: ::core::option::Option<DimensionMetadata>,
    /// The compatibility of this dimension. If the compatibility is COMPATIBLE,
    /// this dimension can be successfully added to the report.
    #[prost(enumeration = "Compatibility", optional, tag = "2")]
    pub compatibility: ::core::option::Option<i32>,
}
/// The compatibility for a single metric.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MetricCompatibility {
    /// The metric metadata contains the API name for this compatibility
    /// information. The metric metadata also contains other helpful information
    /// like the UI name and description.
    #[prost(message, optional, tag = "1")]
    pub metric_metadata: ::core::option::Option<MetricMetadata>,
    /// The compatibility of this metric. If the compatibility is COMPATIBLE,
    /// this metric can be successfully added to the report.
    #[prost(enumeration = "Compatibility", optional, tag = "2")]
    pub compatibility: ::core::option::Option<i32>,
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
/// A metric's value type.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum MetricType {
    /// Unspecified type.
    Unspecified = 0,
    /// Integer type.
    TypeInteger = 1,
    /// Floating point type.
    TypeFloat = 2,
    /// A duration of seconds; a special floating point type.
    TypeSeconds = 4,
    /// A duration in milliseconds; a special floating point type.
    TypeMilliseconds = 5,
    /// A duration in minutes; a special floating point type.
    TypeMinutes = 6,
    /// A duration in hours; a special floating point type.
    TypeHours = 7,
    /// A custom metric of standard type; a special floating point type.
    TypeStandard = 8,
    /// An amount of money; a special floating point type.
    TypeCurrency = 9,
    /// A length in feet; a special floating point type.
    TypeFeet = 10,
    /// A length in miles; a special floating point type.
    TypeMiles = 11,
    /// A length in meters; a special floating point type.
    TypeMeters = 12,
    /// A length in kilometers; a special floating point type.
    TypeKilometers = 13,
}
/// Categories of data that you may be restricted from viewing on certain GA4
/// properties.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum RestrictedMetricType {
    /// Unspecified type.
    Unspecified = 0,
    /// Cost metrics such as `adCost`.
    CostData = 1,
    /// Revenue metrics such as `purchaseRevenue`.
    RevenueData = 2,
}
/// The compatibility types for a single dimension or metric.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Compatibility {
    /// Unspecified compatibility.
    Unspecified = 0,
    /// The dimension or metric is compatible. This dimension or metric can be
    /// successfully added to a report.
    Compatible = 1,
    /// The dimension or metric is incompatible. This dimension or metric cannot be
    /// successfully added to a report.
    Incompatible = 2,
}
/// The request for compatibility information for a report's dimensions and
/// metrics. Check compatibility provides a preview of the compatibility of a
/// report; fields shared with the `runReport` request should be the same values
/// as in your `runReport` request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckCompatibilityRequest {
    /// A Google Analytics GA4 property identifier whose events are tracked. To
    /// learn more, see [where to find your Property
    /// ID](<https://developers.google.com/analytics/devguides/reporting/data/v1/property-id>).
    /// `property` should be the same value as in your `runReport` request.
    ///
    /// Example: properties/1234
    ///
    /// Set the Property ID to 0 for compatibility checking on dimensions and
    /// metrics common to all properties. In this special mode, this method will
    /// not return custom dimensions and metrics.
    #[prost(string, tag = "1")]
    pub property: ::prost::alloc::string::String,
    /// The dimensions in this report. `dimensions` should be the same value as in
    /// your `runReport` request.
    #[prost(message, repeated, tag = "2")]
    pub dimensions: ::prost::alloc::vec::Vec<Dimension>,
    /// The metrics in this report. `metrics` should be the same value as in your
    /// `runReport` request.
    #[prost(message, repeated, tag = "3")]
    pub metrics: ::prost::alloc::vec::Vec<Metric>,
    /// The filter clause of dimensions. `dimensionFilter` should be the same value
    /// as in your `runReport` request.
    #[prost(message, optional, tag = "4")]
    pub dimension_filter: ::core::option::Option<FilterExpression>,
    /// The filter clause of metrics. `metricFilter` should be the same value as in
    /// your `runReport` request
    #[prost(message, optional, tag = "5")]
    pub metric_filter: ::core::option::Option<FilterExpression>,
    /// Filters the dimensions and metrics in the response to just this
    /// compatibility. Commonly used as `”compatibilityFilter”: “COMPATIBLE”`
    /// to only return compatible dimensions & metrics.
    #[prost(enumeration = "Compatibility", tag = "6")]
    pub compatibility_filter: i32,
}
/// The compatibility response with the compatibility of each dimension & metric.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckCompatibilityResponse {
    /// The compatibility of each dimension.
    #[prost(message, repeated, tag = "1")]
    pub dimension_compatibilities: ::prost::alloc::vec::Vec<DimensionCompatibility>,
    /// The compatibility of each metric.
    #[prost(message, repeated, tag = "2")]
    pub metric_compatibilities: ::prost::alloc::vec::Vec<MetricCompatibility>,
}
/// The dimensions and metrics currently accepted in reporting methods.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Metadata {
    /// Resource name of this metadata.
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
    /// The dimension descriptions.
    #[prost(message, repeated, tag = "1")]
    pub dimensions: ::prost::alloc::vec::Vec<DimensionMetadata>,
    /// The metric descriptions.
    #[prost(message, repeated, tag = "2")]
    pub metrics: ::prost::alloc::vec::Vec<MetricMetadata>,
}
/// The request to generate a report.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RunReportRequest {
    /// A Google Analytics GA4 property identifier whose events are tracked.
    /// Specified in the URL path and not the body. To learn more, see [where to
    /// find your Property
    /// ID](<https://developers.google.com/analytics/devguides/reporting/data/v1/property-id>).
    /// Within a batch request, this property should either be unspecified or
    /// consistent with the batch-level property.
    ///
    /// Example: properties/1234
    #[prost(string, tag = "1")]
    pub property: ::prost::alloc::string::String,
    /// The dimensions requested and displayed.
    #[prost(message, repeated, tag = "2")]
    pub dimensions: ::prost::alloc::vec::Vec<Dimension>,
    /// The metrics requested and displayed.
    #[prost(message, repeated, tag = "3")]
    pub metrics: ::prost::alloc::vec::Vec<Metric>,
    /// Date ranges of data to read. If multiple date ranges are requested, each
    /// response row will contain a zero based date range index. If two date
    /// ranges overlap, the event data for the overlapping days is included in the
    /// response rows for both date ranges. In a cohort request, this `dateRanges`
    /// must be unspecified.
    #[prost(message, repeated, tag = "4")]
    pub date_ranges: ::prost::alloc::vec::Vec<DateRange>,
    /// Dimension filters allow you to ask for only specific dimension values in
    /// the report. To learn more, see [Fundamentals of Dimension
    /// Filters](<https://developers.google.com/analytics/devguides/reporting/data/v1/basics#dimension_filters>)
    /// for examples. Metrics cannot be used in this filter.
    #[prost(message, optional, tag = "5")]
    pub dimension_filter: ::core::option::Option<FilterExpression>,
    /// The filter clause of metrics. Applied at post aggregation phase, similar to
    /// SQL having-clause. Dimensions cannot be used in this filter.
    #[prost(message, optional, tag = "6")]
    pub metric_filter: ::core::option::Option<FilterExpression>,
    /// The row count of the start row. The first row is counted as row 0.
    ///
    /// When paging, the first request does not specify offset; or equivalently,
    /// sets offset to 0; the first request returns the first `limit` of rows. The
    /// second request sets offset to the `limit` of the first request; the second
    /// request returns the second `limit` of rows.
    ///
    /// To learn more about this pagination parameter, see
    /// \[Pagination\](<https://developers.google.com/analytics/devguides/reporting/data/v1/basics#pagination>).
    #[prost(int64, tag = "7")]
    pub offset: i64,
    /// The number of rows to return. If unspecified, 10,000 rows are returned. The
    /// API returns a maximum of 100,000 rows per request, no matter how many you
    /// ask for. `limit` must be positive.
    ///
    /// The API can also return fewer rows than the requested `limit`, if there
    /// aren't as many dimension values as the `limit`. For instance, there are
    /// fewer than 300 possible values for the dimension `country`, so when
    /// reporting on only `country`, you can't get more than 300 rows, even if you
    /// set `limit` to a higher value.
    ///
    /// To learn more about this pagination parameter, see
    /// \[Pagination\](<https://developers.google.com/analytics/devguides/reporting/data/v1/basics#pagination>).
    #[prost(int64, tag = "8")]
    pub limit: i64,
    /// Aggregation of metrics. Aggregated metric values will be shown in rows
    /// where the dimension_values are set to "RESERVED_(MetricAggregation)".
    #[prost(enumeration = "MetricAggregation", repeated, tag = "9")]
    pub metric_aggregations: ::prost::alloc::vec::Vec<i32>,
    /// Specifies how rows are ordered in the response.
    #[prost(message, repeated, tag = "10")]
    pub order_bys: ::prost::alloc::vec::Vec<OrderBy>,
    /// A currency code in ISO4217 format, such as "AED", "USD", "JPY".
    /// If the field is empty, the report uses the property's default currency.
    #[prost(string, tag = "11")]
    pub currency_code: ::prost::alloc::string::String,
    /// Cohort group associated with this request. If there is a cohort group
    /// in the request the 'cohort' dimension must be present.
    #[prost(message, optional, tag = "12")]
    pub cohort_spec: ::core::option::Option<CohortSpec>,
    /// If false or unspecified, each row with all metrics equal to 0 will not be
    /// returned. If true, these rows will be returned if they are not separately
    /// removed by a filter.
    #[prost(bool, tag = "13")]
    pub keep_empty_rows: bool,
    /// Toggles whether to return the current state of this Analytics Property's
    /// quota. Quota is returned in \[PropertyQuota\](#PropertyQuota).
    #[prost(bool, tag = "14")]
    pub return_property_quota: bool,
}
/// The response report table corresponding to a request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RunReportResponse {
    /// Describes dimension columns. The number of DimensionHeaders and ordering of
    /// DimensionHeaders matches the dimensions present in rows.
    #[prost(message, repeated, tag = "1")]
    pub dimension_headers: ::prost::alloc::vec::Vec<DimensionHeader>,
    /// Describes metric columns. The number of MetricHeaders and ordering of
    /// MetricHeaders matches the metrics present in rows.
    #[prost(message, repeated, tag = "2")]
    pub metric_headers: ::prost::alloc::vec::Vec<MetricHeader>,
    /// Rows of dimension value combinations and metric values in the report.
    #[prost(message, repeated, tag = "3")]
    pub rows: ::prost::alloc::vec::Vec<Row>,
    /// If requested, the totaled values of metrics.
    #[prost(message, repeated, tag = "4")]
    pub totals: ::prost::alloc::vec::Vec<Row>,
    /// If requested, the maximum values of metrics.
    #[prost(message, repeated, tag = "5")]
    pub maximums: ::prost::alloc::vec::Vec<Row>,
    /// If requested, the minimum values of metrics.
    #[prost(message, repeated, tag = "6")]
    pub minimums: ::prost::alloc::vec::Vec<Row>,
    /// The total number of rows in the query result. `rowCount` is independent of
    /// the number of rows returned in the response, the `limit` request
    /// parameter, and the `offset` request parameter. For example if a query
    /// returns 175 rows and includes `limit` of 50 in the API request, the
    /// response will contain `rowCount` of 175 but only 50 rows.
    ///
    /// To learn more about this pagination parameter, see
    /// \[Pagination\](<https://developers.google.com/analytics/devguides/reporting/data/v1/basics#pagination>).
    #[prost(int32, tag = "7")]
    pub row_count: i32,
    /// Metadata for the report.
    #[prost(message, optional, tag = "8")]
    pub metadata: ::core::option::Option<ResponseMetaData>,
    /// This Analytics Property's quota state including this request.
    #[prost(message, optional, tag = "9")]
    pub property_quota: ::core::option::Option<PropertyQuota>,
    /// Identifies what kind of resource this message is. This `kind` is always the
    /// fixed string "analyticsData#runReport". Useful to distinguish between
    /// response types in JSON.
    #[prost(string, tag = "10")]
    pub kind: ::prost::alloc::string::String,
}
/// The request to generate a pivot report.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RunPivotReportRequest {
    /// A Google Analytics GA4 property identifier whose events are tracked.
    /// Specified in the URL path and not the body. To learn more, see [where to
    /// find your Property
    /// ID](<https://developers.google.com/analytics/devguides/reporting/data/v1/property-id>).
    /// Within a batch request, this property should either be unspecified or
    /// consistent with the batch-level property.
    ///
    /// Example: properties/1234
    #[prost(string, tag = "1")]
    pub property: ::prost::alloc::string::String,
    /// The dimensions requested. All defined dimensions must be used by one of the
    /// following: dimension_expression, dimension_filter, pivots, order_bys.
    #[prost(message, repeated, tag = "2")]
    pub dimensions: ::prost::alloc::vec::Vec<Dimension>,
    /// The metrics requested, at least one metric needs to be specified. All
    /// defined metrics must be used by one of the following: metric_expression,
    /// metric_filter, order_bys.
    #[prost(message, repeated, tag = "3")]
    pub metrics: ::prost::alloc::vec::Vec<Metric>,
    /// The date range to retrieve event data for the report. If multiple date
    /// ranges are specified, event data from each date range is used in the
    /// report. A special dimension with field name "dateRange" can be included in
    /// a Pivot's field names; if included, the report compares between date
    /// ranges. In a cohort request, this `dateRanges` must be unspecified.
    #[prost(message, repeated, tag = "4")]
    pub date_ranges: ::prost::alloc::vec::Vec<DateRange>,
    /// Describes the visual format of the report's dimensions in columns or rows.
    /// The union of the fieldNames (dimension names) in all pivots must be a
    /// subset of dimension names defined in Dimensions. No two pivots can share a
    /// dimension. A dimension is only visible if it appears in a pivot.
    #[prost(message, repeated, tag = "5")]
    pub pivots: ::prost::alloc::vec::Vec<Pivot>,
    /// The filter clause of dimensions. Dimensions must be requested to be used in
    /// this filter. Metrics cannot be used in this filter.
    #[prost(message, optional, tag = "6")]
    pub dimension_filter: ::core::option::Option<FilterExpression>,
    /// The filter clause of metrics. Applied at post aggregation phase, similar to
    /// SQL having-clause. Metrics must be requested to be used in this filter.
    /// Dimensions cannot be used in this filter.
    #[prost(message, optional, tag = "7")]
    pub metric_filter: ::core::option::Option<FilterExpression>,
    /// A currency code in ISO4217 format, such as "AED", "USD", "JPY".
    /// If the field is empty, the report uses the property's default currency.
    #[prost(string, tag = "8")]
    pub currency_code: ::prost::alloc::string::String,
    /// Cohort group associated with this request. If there is a cohort group
    /// in the request the 'cohort' dimension must be present.
    #[prost(message, optional, tag = "9")]
    pub cohort_spec: ::core::option::Option<CohortSpec>,
    /// If false or unspecified, each row with all metrics equal to 0 will not be
    /// returned. If true, these rows will be returned if they are not separately
    /// removed by a filter.
    #[prost(bool, tag = "10")]
    pub keep_empty_rows: bool,
    /// Toggles whether to return the current state of this Analytics Property's
    /// quota. Quota is returned in \[PropertyQuota\](#PropertyQuota).
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
    pub pivot_headers: ::prost::alloc::vec::Vec<PivotHeader>,
    /// Describes dimension columns. The number of DimensionHeaders and ordering of
    /// DimensionHeaders matches the dimensions present in rows.
    #[prost(message, repeated, tag = "2")]
    pub dimension_headers: ::prost::alloc::vec::Vec<DimensionHeader>,
    /// Describes metric columns. The number of MetricHeaders and ordering of
    /// MetricHeaders matches the metrics present in rows.
    #[prost(message, repeated, tag = "3")]
    pub metric_headers: ::prost::alloc::vec::Vec<MetricHeader>,
    /// Rows of dimension value combinations and metric values in the report.
    #[prost(message, repeated, tag = "4")]
    pub rows: ::prost::alloc::vec::Vec<Row>,
    /// Aggregation of metric values. Can be totals, minimums, or maximums. The
    /// returned aggregations are controlled by the metric_aggregations in the
    /// pivot. The type of aggregation returned in each row is shown by the
    /// dimension_values which are set to "RESERVED_<MetricAggregation>".
    #[prost(message, repeated, tag = "5")]
    pub aggregates: ::prost::alloc::vec::Vec<Row>,
    /// Metadata for the report.
    #[prost(message, optional, tag = "6")]
    pub metadata: ::core::option::Option<ResponseMetaData>,
    /// This Analytics Property's quota state including this request.
    #[prost(message, optional, tag = "7")]
    pub property_quota: ::core::option::Option<PropertyQuota>,
    /// Identifies what kind of resource this message is. This `kind` is always the
    /// fixed string "analyticsData#runPivotReport". Useful to distinguish between
    /// response types in JSON.
    #[prost(string, tag = "8")]
    pub kind: ::prost::alloc::string::String,
}
/// The batch request containing multiple report requests.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchRunReportsRequest {
    /// A Google Analytics GA4 property identifier whose events are tracked.
    /// Specified in the URL path and not the body. To learn more, see [where to
    /// find your Property
    /// ID](<https://developers.google.com/analytics/devguides/reporting/data/v1/property-id>).
    /// This property must be specified for the batch. The property within
    /// RunReportRequest may either be unspecified or consistent with this
    /// property.
    ///
    /// Example: properties/1234
    #[prost(string, tag = "1")]
    pub property: ::prost::alloc::string::String,
    /// Individual requests. Each request has a separate report response. Each
    /// batch request is allowed up to 5 requests.
    #[prost(message, repeated, tag = "2")]
    pub requests: ::prost::alloc::vec::Vec<RunReportRequest>,
}
/// The batch response containing multiple reports.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchRunReportsResponse {
    /// Individual responses. Each response has a separate report request.
    #[prost(message, repeated, tag = "1")]
    pub reports: ::prost::alloc::vec::Vec<RunReportResponse>,
    /// Identifies what kind of resource this message is. This `kind` is always the
    /// fixed string "analyticsData#batchRunReports". Useful to distinguish between
    /// response types in JSON.
    #[prost(string, tag = "2")]
    pub kind: ::prost::alloc::string::String,
}
/// The batch request containing multiple pivot report requests.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchRunPivotReportsRequest {
    /// A Google Analytics GA4 property identifier whose events are tracked.
    /// Specified in the URL path and not the body. To learn more, see [where to
    /// find your Property
    /// ID](<https://developers.google.com/analytics/devguides/reporting/data/v1/property-id>).
    /// This property must be specified for the batch. The property within
    /// RunPivotReportRequest may either be unspecified or consistent with this
    /// property.
    ///
    /// Example: properties/1234
    #[prost(string, tag = "1")]
    pub property: ::prost::alloc::string::String,
    /// Individual requests. Each request has a separate pivot report response.
    /// Each batch request is allowed up to 5 requests.
    #[prost(message, repeated, tag = "2")]
    pub requests: ::prost::alloc::vec::Vec<RunPivotReportRequest>,
}
/// The batch response containing multiple pivot reports.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchRunPivotReportsResponse {
    /// Individual responses. Each response has a separate pivot report request.
    #[prost(message, repeated, tag = "1")]
    pub pivot_reports: ::prost::alloc::vec::Vec<RunPivotReportResponse>,
    /// Identifies what kind of resource this message is. This `kind` is always the
    /// fixed string "analyticsData#batchRunPivotReports". Useful to distinguish
    /// between response types in JSON.
    #[prost(string, tag = "2")]
    pub kind: ::prost::alloc::string::String,
}
/// Request for a property's dimension and metric metadata.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMetadataRequest {
    /// Required. The resource name of the metadata to retrieve. This name field is
    /// specified in the URL path and not URL parameters. Property is a numeric
    /// Google Analytics GA4 Property identifier. To learn more, see [where to find
    /// your Property
    /// ID](<https://developers.google.com/analytics/devguides/reporting/data/v1/property-id>).
    ///
    /// Example: properties/1234/metadata
    ///
    /// Set the Property ID to 0 for dimensions and metrics common to all
    /// properties. In this special mode, this method will not return custom
    /// dimensions and metrics.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// The request to generate a realtime report.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RunRealtimeReportRequest {
    /// A Google Analytics GA4 property identifier whose events are tracked.
    /// Specified in the URL path and not the body. To learn more, see [where to
    /// find your Property
    /// ID](<https://developers.google.com/analytics/devguides/reporting/data/v1/property-id>).
    ///
    /// Example: properties/1234
    #[prost(string, tag = "1")]
    pub property: ::prost::alloc::string::String,
    /// The dimensions requested and displayed.
    #[prost(message, repeated, tag = "2")]
    pub dimensions: ::prost::alloc::vec::Vec<Dimension>,
    /// The metrics requested and displayed.
    #[prost(message, repeated, tag = "3")]
    pub metrics: ::prost::alloc::vec::Vec<Metric>,
    /// The filter clause of dimensions. Dimensions must be requested to be used in
    /// this filter. Metrics cannot be used in this filter.
    #[prost(message, optional, tag = "4")]
    pub dimension_filter: ::core::option::Option<FilterExpression>,
    /// The filter clause of metrics. Applied at post aggregation phase, similar to
    /// SQL having-clause. Metrics must be requested to be used in this filter.
    /// Dimensions cannot be used in this filter.
    #[prost(message, optional, tag = "5")]
    pub metric_filter: ::core::option::Option<FilterExpression>,
    /// The number of rows to return. If unspecified, 10,000 rows are returned. The
    /// API returns a maximum of 100,000 rows per request, no matter how many you
    /// ask for. `limit` must be positive.
    ///
    /// The API can also return fewer rows than the requested `limit`, if there
    /// aren't as many dimension values as the `limit`. For instance, there are
    /// fewer than 300 possible values for the dimension `country`, so when
    /// reporting on only `country`, you can't get more than 300 rows, even if you
    /// set `limit` to a higher value.
    #[prost(int64, tag = "6")]
    pub limit: i64,
    /// Aggregation of metrics. Aggregated metric values will be shown in rows
    /// where the dimension_values are set to "RESERVED_(MetricAggregation)".
    #[prost(enumeration = "MetricAggregation", repeated, tag = "7")]
    pub metric_aggregations: ::prost::alloc::vec::Vec<i32>,
    /// Specifies how rows are ordered in the response.
    #[prost(message, repeated, tag = "8")]
    pub order_bys: ::prost::alloc::vec::Vec<OrderBy>,
    /// Toggles whether to return the current state of this Analytics Property's
    /// Realtime quota. Quota is returned in \[PropertyQuota\](#PropertyQuota).
    #[prost(bool, tag = "9")]
    pub return_property_quota: bool,
    /// The minute ranges of event data to read. If unspecified, one minute range
    /// for the last 30 minutes will be used. If multiple minute ranges are
    /// requested, each response row will contain a zero based minute range index.
    /// If two minute ranges overlap, the event data for the overlapping minutes is
    /// included in the response rows for both minute ranges.
    #[prost(message, repeated, tag = "10")]
    pub minute_ranges: ::prost::alloc::vec::Vec<MinuteRange>,
}
/// The response realtime report table corresponding to a request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RunRealtimeReportResponse {
    /// Describes dimension columns. The number of DimensionHeaders and ordering of
    /// DimensionHeaders matches the dimensions present in rows.
    #[prost(message, repeated, tag = "1")]
    pub dimension_headers: ::prost::alloc::vec::Vec<DimensionHeader>,
    /// Describes metric columns. The number of MetricHeaders and ordering of
    /// MetricHeaders matches the metrics present in rows.
    #[prost(message, repeated, tag = "2")]
    pub metric_headers: ::prost::alloc::vec::Vec<MetricHeader>,
    /// Rows of dimension value combinations and metric values in the report.
    #[prost(message, repeated, tag = "3")]
    pub rows: ::prost::alloc::vec::Vec<Row>,
    /// If requested, the totaled values of metrics.
    #[prost(message, repeated, tag = "4")]
    pub totals: ::prost::alloc::vec::Vec<Row>,
    /// If requested, the maximum values of metrics.
    #[prost(message, repeated, tag = "5")]
    pub maximums: ::prost::alloc::vec::Vec<Row>,
    /// If requested, the minimum values of metrics.
    #[prost(message, repeated, tag = "6")]
    pub minimums: ::prost::alloc::vec::Vec<Row>,
    /// The total number of rows in the query result. `rowCount` is independent of
    /// the number of rows returned in the response and the `limit` request
    /// parameter. For example if a query returns 175 rows and includes `limit`
    /// of 50 in the API request, the response will contain `rowCount` of 175 but
    /// only 50 rows.
    #[prost(int32, tag = "7")]
    pub row_count: i32,
    /// This Analytics Property's Realtime quota state including this request.
    #[prost(message, optional, tag = "8")]
    pub property_quota: ::core::option::Option<PropertyQuota>,
    /// Identifies what kind of resource this message is. This `kind` is always the
    /// fixed string "analyticsData#runRealtimeReport". Useful to distinguish
    /// between response types in JSON.
    #[prost(string, tag = "9")]
    pub kind: ::prost::alloc::string::String,
}
#[doc = r" Generated client implementations."]
pub mod beta_analytics_data_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Google Analytics reporting data service."]
    #[derive(Debug, Clone)]
    pub struct BetaAnalyticsDataClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> BetaAnalyticsDataClient<T>
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
        ) -> BetaAnalyticsDataClient<InterceptedService<T, F>>
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
            BetaAnalyticsDataClient::new(InterceptedService::new(inner, interceptor))
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
                "/google.analytics.data.v1beta.BetaAnalyticsData/RunReport",
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
                "/google.analytics.data.v1beta.BetaAnalyticsData/RunPivotReport",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns multiple reports in a batch. All reports must be for the same"]
        #[doc = " GA4 Property."]
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
                "/google.analytics.data.v1beta.BetaAnalyticsData/BatchRunReports",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns multiple pivot reports in a batch. All reports must be for the same"]
        #[doc = " GA4 Property."]
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
                "/google.analytics.data.v1beta.BetaAnalyticsData/BatchRunPivotReports",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns metadata for dimensions and metrics available in reporting methods."]
        #[doc = " Used to explore the dimensions and metrics. In this method, a Google"]
        #[doc = " Analytics GA4 Property Identifier is specified in the request, and"]
        #[doc = " the metadata response includes Custom dimensions and metrics as well as"]
        #[doc = " Universal metadata."]
        #[doc = ""]
        #[doc = " For example if a custom metric with parameter name `levels_unlocked` is"]
        #[doc = " registered to a property, the Metadata response will contain"]
        #[doc = " `customEvent:levels_unlocked`. Universal metadata are dimensions and"]
        #[doc = " metrics applicable to any property such as `country` and `totalUsers`."]
        pub async fn get_metadata(
            &mut self,
            request: impl tonic::IntoRequest<super::GetMetadataRequest>,
        ) -> Result<tonic::Response<super::Metadata>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.data.v1beta.BetaAnalyticsData/GetMetadata",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " The Google Analytics Realtime API returns a customized report of realtime"]
        #[doc = " event data for your property. These reports show events and usage from the"]
        #[doc = " last 30 minutes."]
        pub async fn run_realtime_report(
            &mut self,
            request: impl tonic::IntoRequest<super::RunRealtimeReportRequest>,
        ) -> Result<tonic::Response<super::RunRealtimeReportResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.data.v1beta.BetaAnalyticsData/RunRealtimeReport",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " This compatibility method lists dimensions and metrics that can be added to"]
        #[doc = " a report request and maintain compatibility. This method fails if the"]
        #[doc = " request's dimensions and metrics are incompatible."]
        #[doc = ""]
        #[doc = " In Google Analytics, reports fail if they request incompatible dimensions"]
        #[doc = " and/or metrics; in that case, you will need to remove dimensions and/or"]
        #[doc = " metrics from the incompatible report until the report is compatible."]
        #[doc = ""]
        #[doc = " The Realtime and Core reports have different compatibility rules. This"]
        #[doc = " method checks compatibility for Core reports."]
        pub async fn check_compatibility(
            &mut self,
            request: impl tonic::IntoRequest<super::CheckCompatibilityRequest>,
        ) -> Result<tonic::Response<super::CheckCompatibilityResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.data.v1beta.BetaAnalyticsData/CheckCompatibility",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
