/// A contiguous set of days: `startDate`, `startDate + 1`, ..., `endDate`.
/// Requests are allowed up to 4 date ranges.
#[allow(clippy::derive_partial_eq_without_eq)]
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
/// Dimensions are attributes of your data. For example, the dimension city
/// indicates the city from which an event originates. Dimension values in report
/// responses are strings; for example, the city could be "Paris" or "New York".
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Dimension {
    /// The name of the dimension. See the [API
    /// Dimensions](<https://developers.google.com/analytics/devguides/reporting/data/v1/api-schema#dimensions>)
    /// for the list of dimension names supported by core reporting methods such
    /// as `runReport` and `batchRunReports`. See
    /// [Realtime
    /// Dimensions](<https://developers.google.com/analytics/devguides/reporting/data/v1/realtime-api-schema#dimensions>)
    /// for the list of dimension names supported by the `runRealtimeReport`
    /// method. See
    /// [Funnel
    /// Dimensions](<https://developers.google.com/analytics/devguides/reporting/data/v1/exploration-api-schema#dimensions>)
    /// for the list of dimension names supported by the `runFunnelReport`
    /// method.
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DimensionExpression {
    /// Specify one type of dimension expression for `DimensionExpression`.
    #[prost(oneof = "dimension_expression::OneExpression", tags = "4, 5, 6")]
    pub one_expression: ::core::option::Option<dimension_expression::OneExpression>,
}
/// Nested message and enum types in `DimensionExpression`.
pub mod dimension_expression {
    /// Used to convert a dimension value to a single case.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CaseExpression {
        /// Name of a dimension. The name must refer back to a name in dimensions
        /// field of the request.
        #[prost(string, tag = "1")]
        pub dimension_name: ::prost::alloc::string::String,
    }
    /// Used to combine dimension values to a single dimension.
    #[allow(clippy::derive_partial_eq_without_eq)]
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
    #[allow(clippy::derive_partial_eq_without_eq)]
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
/// To express dimension or metric filters. The fields in the same
/// FilterExpression need to be either all dimensions or all metrics.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FilterExpression {
    /// Specify one type of filter expression for `FilterExpression`.
    #[prost(oneof = "filter_expression::Expr", tags = "1, 2, 3, 4")]
    pub expr: ::core::option::Option<filter_expression::Expr>,
}
/// Nested message and enum types in `FilterExpression`.
pub mod filter_expression {
    /// Specify one type of filter expression for `FilterExpression`.
    #[allow(clippy::derive_partial_eq_without_eq)]
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
        /// A primitive filter. In the same FilterExpression, all of the filter's
        /// field names need to be either all dimensions or all metrics.
        #[prost(message, tag = "4")]
        Filter(super::Filter),
    }
}
/// A list of filter expressions.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FilterExpressionList {
    /// A list of filter expressions.
    #[prost(message, repeated, tag = "1")]
    pub expressions: ::prost::alloc::vec::Vec<FilterExpression>,
}
/// An expression to filter dimension or metric values.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Filter {
    /// The dimension name or metric name. Must be a name defined in dimensions
    /// or metrics.
    #[prost(string, tag = "1")]
    pub field_name: ::prost::alloc::string::String,
    /// Specify one type of filter for `Filter`.
    #[prost(oneof = "filter::OneFilter", tags = "2, 3, 4, 5")]
    pub one_filter: ::core::option::Option<filter::OneFilter>,
}
/// Nested message and enum types in `Filter`.
pub mod filter {
    /// Specify one type of filter for `Filter`.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum OneFilter {
        /// Strings related filter.
        #[prost(message, tag = "2")]
        StringFilter(super::StringFilter),
        /// A filter for in list values.
        #[prost(message, tag = "3")]
        InListFilter(super::InListFilter),
        /// A filter for numeric or date values.
        #[prost(message, tag = "4")]
        NumericFilter(super::NumericFilter),
        /// A filter for between two values.
        #[prost(message, tag = "5")]
        BetweenFilter(super::BetweenFilter),
    }
}
/// The filter for string
#[allow(clippy::derive_partial_eq_without_eq)]
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
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
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
        /// Full match for the regular expression with the string value.
        FullRegexp = 5,
        /// Partial match for the regular expression with the string value.
        PartialRegexp = 6,
    }
    impl MatchType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                MatchType::Unspecified => "MATCH_TYPE_UNSPECIFIED",
                MatchType::Exact => "EXACT",
                MatchType::BeginsWith => "BEGINS_WITH",
                MatchType::EndsWith => "ENDS_WITH",
                MatchType::Contains => "CONTAINS",
                MatchType::FullRegexp => "FULL_REGEXP",
                MatchType::PartialRegexp => "PARTIAL_REGEXP",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "MATCH_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "EXACT" => Some(Self::Exact),
                "BEGINS_WITH" => Some(Self::BeginsWith),
                "ENDS_WITH" => Some(Self::EndsWith),
                "CONTAINS" => Some(Self::Contains),
                "FULL_REGEXP" => Some(Self::FullRegexp),
                "PARTIAL_REGEXP" => Some(Self::PartialRegexp),
                _ => None,
            }
        }
    }
}
/// The result needs to be in a list of string values.
#[allow(clippy::derive_partial_eq_without_eq)]
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NumericFilter {
    /// The operation type for this filter.
    #[prost(enumeration = "numeric_filter::Operation", tag = "1")]
    pub operation: i32,
    /// A numeric value or a date value.
    #[prost(message, optional, tag = "2")]
    pub value: ::core::option::Option<NumericValue>,
}
/// Nested message and enum types in `NumericFilter`.
pub mod numeric_filter {
    /// The operation applied to a numeric filter
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
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
    impl Operation {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Operation::Unspecified => "OPERATION_UNSPECIFIED",
                Operation::Equal => "EQUAL",
                Operation::LessThan => "LESS_THAN",
                Operation::LessThanOrEqual => "LESS_THAN_OR_EQUAL",
                Operation::GreaterThan => "GREATER_THAN",
                Operation::GreaterThanOrEqual => "GREATER_THAN_OR_EQUAL",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "OPERATION_UNSPECIFIED" => Some(Self::Unspecified),
                "EQUAL" => Some(Self::Equal),
                "LESS_THAN" => Some(Self::LessThan),
                "LESS_THAN_OR_EQUAL" => Some(Self::LessThanOrEqual),
                "GREATER_THAN" => Some(Self::GreaterThan),
                "GREATER_THAN_OR_EQUAL" => Some(Self::GreaterThanOrEqual),
                _ => None,
            }
        }
    }
}
/// To express that the result needs to be between two numbers (inclusive).
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BetweenFilter {
    /// Begins with this number.
    #[prost(message, optional, tag = "1")]
    pub from_value: ::core::option::Option<NumericValue>,
    /// Ends with this number.
    #[prost(message, optional, tag = "2")]
    pub to_value: ::core::option::Option<NumericValue>,
}
/// To represent a number.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NumericValue {
    /// One of a numeric value
    #[prost(oneof = "numeric_value::OneValue", tags = "1, 2")]
    pub one_value: ::core::option::Option<numeric_value::OneValue>,
}
/// Nested message and enum types in `NumericValue`.
pub mod numeric_value {
    /// One of a numeric value
    #[allow(clippy::derive_partial_eq_without_eq)]
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
/// Describes a dimension column in the report. Dimensions requested in a report
/// produce column entries within rows and DimensionHeaders. However, dimensions
/// used exclusively within filters or expressions do not produce columns in a
/// report; correspondingly, those dimensions do not produce headers.
#[allow(clippy::derive_partial_eq_without_eq)]
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MetricHeader {
    /// The metric's name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The metric's data type.
    #[prost(enumeration = "MetricType", tag = "2")]
    pub r#type: i32,
}
/// Report data for each row.
/// For example if RunReportRequest contains:
///
/// ```none
/// "dimensions": [
///    {
///      "name": "eventName"
///    },
///    {
///      "name": "countryId"
///    }
/// ],
/// "metrics": [
///    {
///      "name": "eventCount"
///    }
/// ]
/// ```
///
/// One row with 'in_app_purchase' as the eventName, 'JP' as the countryId, and
/// 15 as the eventCount, would be:
///
/// ```none
/// "dimensionValues": [
///    {
///      "value": "in_app_purchase"
///    },
///    {
///      "value": "JP"
///    }
/// ],
/// "metricValues": [
///    {
///      "value": "15"
///    }
/// ]
/// ```
#[allow(clippy::derive_partial_eq_without_eq)]
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DimensionValue {
    /// One kind of dimension value
    #[prost(oneof = "dimension_value::OneValue", tags = "1")]
    pub one_value: ::core::option::Option<dimension_value::OneValue>,
}
/// Nested message and enum types in `DimensionValue`.
pub mod dimension_value {
    /// One kind of dimension value
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum OneValue {
        /// Value as a string if the dimension type is a string.
        #[prost(string, tag = "1")]
        Value(::prost::alloc::string::String),
    }
}
/// The value of a metric.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MetricValue {
    /// One of metric value
    #[prost(oneof = "metric_value::OneValue", tags = "4")]
    pub one_value: ::core::option::Option<metric_value::OneValue>,
}
/// Nested message and enum types in `MetricValue`.
pub mod metric_value {
    /// One of metric value
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum OneValue {
        /// Measurement value. See MetricHeader for type.
        #[prost(string, tag = "4")]
        Value(::prost::alloc::string::String),
    }
}
/// Current state of all quotas for this Analytics Property. If any quota for a
/// property is exhausted, all requests to that property will return Resource
/// Exhausted errors.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PropertyQuota {
    /// Standard Analytics Properties can use up to 200,000 tokens per day;
    /// Analytics 360 Properties can use 2,000,000 tokens per day. Most requests
    /// consume fewer than 10 tokens.
    #[prost(message, optional, tag = "1")]
    pub tokens_per_day: ::core::option::Option<QuotaStatus>,
    /// Standard Analytics Properties can use up to 40,000 tokens per hour;
    /// Analytics 360 Properties can use 400,000 tokens per hour. An API request
    /// consumes a single number of tokens, and that number is deducted from all of
    /// the hourly, daily, and per project hourly quotas.
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
    /// Analytics Properties can use up to 35% of their tokens per project per
    /// hour. This amounts to standard Analytics Properties can use up to 14,000
    /// tokens per project per hour, and Analytics 360 Properties can use 140,000
    /// tokens per project per hour. An API request consumes a single number of
    /// tokens, and that number is deducted from all of the hourly, daily, and per
    /// project hourly quotas.
    #[prost(message, optional, tag = "6")]
    pub tokens_per_project_per_hour: ::core::option::Option<QuotaStatus>,
}
/// Current state for a particular quota group.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuotaStatus {
    /// Quota consumed by this request.
    #[prost(int32, tag = "1")]
    pub consumed: i32,
    /// Quota remaining after this request.
    #[prost(int32, tag = "2")]
    pub remaining: i32,
}
/// Breakdowns add a dimension to the funnel table sub report response.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FunnelBreakdown {
    /// The dimension column added to the funnel table sub report response. The
    /// breakdown dimension breaks down each funnel step. A valid
    /// `breakdownDimension` is required if `funnelBreakdown` is specified.
    #[prost(message, optional, tag = "1")]
    pub breakdown_dimension: ::core::option::Option<Dimension>,
    /// The maximum number of distinct values of the breakdown dimension to return
    /// in the response. A `limit` of `5` is used if limit is not specified. Limit
    /// must exceed zero and cannot exceed 15.
    #[prost(int64, optional, tag = "2")]
    pub limit: ::core::option::Option<i64>,
}
/// Next actions state the value for a dimension after the user has achieved
/// a step but before the same user has achieved the next step. For example if
/// the `nextActionDimension` is `eventName`, then `nextActionDimension` in the
/// `i`th funnel step row will return first event after the event that qualified
/// the user into the `i`th funnel step but before the user achieved the `i+1`th
/// funnel step.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FunnelNextAction {
    /// The dimension column added to the funnel visualization sub report response.
    /// The next action dimension returns the next dimension value of this
    /// dimension after the user has attained the `i`th funnel step.
    ///
    /// `nextActionDimension` currently only supports `eventName` and most Page /
    /// Screen dimensions like `pageTitle` and `pagePath`. `nextActionDimension`
    /// cannot be a dimension expression.
    #[prost(message, optional, tag = "1")]
    pub next_action_dimension: ::core::option::Option<Dimension>,
    /// The maximum number of distinct values of the breakdown dimension to return
    /// in the response. A `limit` of `5` is used if limit is not specified. Limit
    /// must exceed zero and cannot exceed 5.
    #[prost(int64, optional, tag = "2")]
    pub limit: ::core::option::Option<i64>,
}
/// Configures the funnel in a funnel report request. A funnel reports on users
/// as they pass through a sequence of steps.
///
/// Funnel exploration lets you visualize the steps your users take to complete a
/// task and quickly see how well they are succeeding or failing at each step.
/// For example, how do prospects become shoppers and then become buyers? How do
/// one time buyers become repeat buyers? With this information, you can improve
/// inefficient or abandoned customer journeys.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Funnel {
    /// In an open funnel, users can enter the funnel in any step, and in a closed
    /// funnel, users must enter the funnel in the first step. Optional. If
    /// unspecified, a closed funnel is used.
    #[prost(bool, tag = "1")]
    pub is_open_funnel: bool,
    /// The sequential steps of this funnel.
    #[prost(message, repeated, tag = "2")]
    pub steps: ::prost::alloc::vec::Vec<FunnelStep>,
}
/// Steps define the user journey you want to measure. Steps contain one or
/// more conditions that your users must meet to be included in that step of
/// the funnel journey.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FunnelStep {
    /// The distinctive name for this step. If unspecified, steps will be named
    /// by a 1 based indexed name (for example "0. ", "1. ", etc.). This name
    /// defines string value returned by the `funnelStepName` dimension. For
    /// example, specifying `name = Purchase` in the request's third funnel step
    /// will produce `3. Purchase` in the funnel report response.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// If true, this step must directly follow the previous step. If false,
    /// there can be events between the previous step and this step. If
    /// unspecified, `isDirectlyFollowedBy` is treated as false.
    #[prost(bool, tag = "2")]
    pub is_directly_followed_by: bool,
    /// If specified, this step must complete within this duration of the
    /// completion of the prior step. `withinDurationFromPriorStep` is inclusive
    /// of the endpoint at the microsecond granularity. For example a duration of
    /// 5 seconds can be completed at 4.9 or 5.0 seconds, but not 5 seconds and 1
    /// microsecond.
    ///
    /// `withinDurationFromPriorStep` is optional, and if unspecified, steps may
    /// be separated by any time duration.
    #[prost(message, optional, tag = "3")]
    pub within_duration_from_prior_step: ::core::option::Option<::prost_types::Duration>,
    /// The condition that your users must meet to be included in this step of
    /// the funnel journey.
    #[prost(message, optional, tag = "4")]
    pub filter_expression: ::core::option::Option<FunnelFilterExpression>,
}
/// Funnel sub reports contain the dimension and metric data values. For example,
/// 12 users reached the second step of the funnel.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FunnelSubReport {
    /// Describes dimension columns. Funnel reports always include the funnel step
    /// dimension in sub report responses. Additional dimensions like breakdowns,
    /// dates, and next actions may be present in the response if requested.
    #[prost(message, repeated, tag = "1")]
    pub dimension_headers: ::prost::alloc::vec::Vec<DimensionHeader>,
    /// Describes metric columns. Funnel reports always include active users in sub
    /// report responses. The funnel table includes additional metrics like
    /// completion rate, abandonments, and abandonments rate.
    #[prost(message, repeated, tag = "2")]
    pub metric_headers: ::prost::alloc::vec::Vec<MetricHeader>,
    /// Rows of dimension value combinations and metric values in the report.
    #[prost(message, repeated, tag = "3")]
    pub rows: ::prost::alloc::vec::Vec<Row>,
    /// Metadata for the funnel report.
    #[prost(message, optional, tag = "4")]
    pub metadata: ::core::option::Option<FunnelResponseMetadata>,
}
/// User segments are subsets of users who engaged with your site or app. For
/// example, users who have previously purchased; users who added items to their
/// shopping carts, but didn’t complete a purchase.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserSegment {
    /// Defines which users are included in this segment. Optional.
    #[prost(message, optional, tag = "1")]
    pub user_inclusion_criteria: ::core::option::Option<UserSegmentCriteria>,
    /// Defines which users are excluded in this segment. Optional.
    #[prost(message, optional, tag = "2")]
    pub exclusion: ::core::option::Option<UserSegmentExclusion>,
}
/// A user matches a criteria if the user's events meet the conditions in the
/// criteria.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserSegmentCriteria {
    /// A user matches this criteria if the user matches each of these
    /// `andConditionGroups` and each of the `andSequenceGroups`.
    /// `andConditionGroups` may be empty if `andSequenceGroups` are specified.
    #[prost(message, repeated, tag = "1")]
    pub and_condition_groups: ::prost::alloc::vec::Vec<UserSegmentConditionGroup>,
    /// A user matches this criteria if the user matches each of these
    /// `andSequenceGroups` and each of the `andConditionGroups`.
    /// `andSequenceGroups` may be empty if `andConditionGroups` are specified.
    #[prost(message, repeated, tag = "2")]
    pub and_sequence_groups: ::prost::alloc::vec::Vec<UserSegmentSequenceGroup>,
}
/// Conditions tell Analytics what data to include in or exclude from the
/// segment.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserSegmentConditionGroup {
    /// Data is included or excluded from the segment based on if it matches
    /// the condition group. This scoping defines how many events the
    /// `segmentFilterExpression` is evaluated on before the condition group
    /// is determined to be matched or not. For example if `conditionScoping =
    /// USER_CRITERIA_WITHIN_SAME_SESSION`, the expression is evaluated on all
    /// events in a session, and then, the condition group is determined to be
    /// matched or not for this user. For example if `conditionScoping =
    /// USER_CRITERIA_WITHIN_SAME_EVENT`, the expression is evaluated on a single
    /// event, and then, the condition group is determined to be matched or not for
    /// this user.
    ///
    /// Optional. If unspecified, `conditionScoping = ACROSS_ALL_SESSIONS` is
    /// used.
    #[prost(enumeration = "UserCriteriaScoping", tag = "1")]
    pub condition_scoping: i32,
    /// Data is included or excluded from the segment based on if it matches
    /// this expression. Expressions express criteria on dimension, metrics,
    /// and/or parameters.
    #[prost(message, optional, tag = "2")]
    pub segment_filter_expression: ::core::option::Option<SegmentFilterExpression>,
}
/// Define conditions that must occur in a specific order for the user to be
/// a member of the segment.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserSegmentSequenceGroup {
    /// All sequence steps must be satisfied in the scoping for the user to
    /// match the sequence. For example if `sequenceScoping =
    /// USER_CRITERIA_WITHIN_SAME_SESSION`, all sequence steps must complete within
    /// one session for the user to match the sequence. `sequenceScoping =
    /// USER_CRITERIA_WITHIN_SAME_EVENT` is not supported.
    ///
    /// Optional. If unspecified, `conditionScoping = ACROSS_ALL_SESSIONS` is
    /// used.
    #[prost(enumeration = "UserCriteriaScoping", tag = "1")]
    pub sequence_scoping: i32,
    /// Defines the time period in which the whole sequence must occur; for
    /// example, 30 Minutes. `sequenceMaximumDuration` is inclusive
    /// of the endpoint at the microsecond granularity. For example a sequence
    /// with a maximum duration of 5 seconds can be completed at 4.9 or 5.0
    /// seconds, but not 5 seconds and 1 microsecond.
    ///
    /// `sequenceMaximumDuration` is optional, and if unspecified, sequences can
    /// be completed in any time duration.
    #[prost(message, optional, tag = "2")]
    pub sequence_maximum_duration: ::core::option::Option<::prost_types::Duration>,
    /// An ordered sequence of condition steps. A user's events must complete
    /// each step in order for the user to match the
    /// `UserSegmentSequenceGroup`.
    #[prost(message, repeated, tag = "3")]
    pub user_sequence_steps: ::prost::alloc::vec::Vec<UserSequenceStep>,
}
/// A condition that must occur in the specified step order for this user
/// to match the sequence.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserSequenceStep {
    /// If true, the event satisfying this step must be the very next event
    /// after the event satifying the last step. If false, this step indirectly
    /// follows the prior step; for example, there may be events between the
    /// prior step and this step. `isDirectlyFollowedBy` must be false for
    /// the first step.
    #[prost(bool, tag = "1")]
    pub is_directly_followed_by: bool,
    /// This sequence step must be satisfied in the scoping for the user to
    /// match the sequence. For example if `sequenceScoping =
    /// WITHIN_SAME_SESSION`, this sequence steps must complete within one
    /// session for the user to match the sequence. `stepScoping =
    /// ACROSS_ALL_SESSIONS` is only allowed if the `sequenceScoping =
    /// ACROSS_ALL_SESSIONS`.
    ///
    /// Optional. If unspecified, `stepScoping` uses the same
    /// `UserCriteriaScoping` as the `sequenceScoping`.
    #[prost(enumeration = "UserCriteriaScoping", tag = "2")]
    pub step_scoping: i32,
    /// A user matches this sequence step if their events match this
    /// expression. Expressions express criteria on dimension, metrics,
    /// and/or parameters.
    #[prost(message, optional, tag = "3")]
    pub segment_filter_expression: ::core::option::Option<SegmentFilterExpression>,
}
/// Specifies which users are excluded in this segment.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserSegmentExclusion {
    /// Specifies how long an exclusion will last if a user matches the
    /// `userExclusionCriteria`.
    ///
    /// Optional. If unspecified, `userExclusionDuration` of
    /// `USER_EXCLUSION_TEMPORARY` is used.
    #[prost(enumeration = "UserExclusionDuration", tag = "1")]
    pub user_exclusion_duration: i32,
    /// If a user meets this condition, the user is excluded from membership in
    /// the segment for the `userExclusionDuration`.
    #[prost(message, optional, tag = "2")]
    pub user_exclusion_criteria: ::core::option::Option<UserSegmentCriteria>,
}
/// Session segments are subsets of the sessions that occurred on your site or
/// app: for example, all the sessions that originated from a particular
/// advertising campaign.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SessionSegment {
    /// Defines which sessions are included in this segment. Optional.
    #[prost(message, optional, tag = "1")]
    pub session_inclusion_criteria: ::core::option::Option<SessionSegmentCriteria>,
    /// Defines which sessions are excluded in this segment. Optional.
    #[prost(message, optional, tag = "2")]
    pub exclusion: ::core::option::Option<SessionSegmentExclusion>,
}
/// A session matches a criteria if the session's events meet the conditions in
/// the criteria.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SessionSegmentCriteria {
    /// A session matches this criteria if the session matches each of these
    /// `andConditionGroups`.
    #[prost(message, repeated, tag = "1")]
    pub and_condition_groups: ::prost::alloc::vec::Vec<SessionSegmentConditionGroup>,
}
/// Conditions tell Analytics what data to include in or exclude from the
/// segment.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SessionSegmentConditionGroup {
    /// Data is included or excluded from the segment based on if it matches
    /// the condition group. This scoping defines how many events the
    /// `segmentFilterExpression` is evaluated on before the condition group
    /// is determined to be matched or not. For example if `conditionScoping =
    /// SESSION_CRITERIA_WITHIN_SAME_SESSION`, the expression is evaluated on all
    /// events in a session, and then, the condition group is determined to be
    /// matched or not for this session. For example if `conditionScoping =
    /// SESSION_CRITERIA_WITHIN_SAME_EVENT`, the expression is evaluated on a
    /// single event, and then, the condition group is determined to be matched or
    /// not for this session.
    ///
    /// Optional. If unspecified, a `conditionScoping` of `WITHIN_SAME_SESSION`
    /// is used.
    #[prost(enumeration = "SessionCriteriaScoping", tag = "1")]
    pub condition_scoping: i32,
    /// Data is included or excluded from the segment based on if it matches
    /// this expression. Expressions express criteria on dimension, metrics,
    /// and/or parameters.
    #[prost(message, optional, tag = "2")]
    pub segment_filter_expression: ::core::option::Option<SegmentFilterExpression>,
}
/// Specifies which sessions are excluded in this segment.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SessionSegmentExclusion {
    /// Specifies how long an exclusion will last if a session matches the
    /// `sessionExclusionCriteria`.
    ///
    /// Optional. If unspecified, a `sessionExclusionDuration` of
    /// `SESSION_EXCLUSION_TEMPORARY` is used.
    #[prost(enumeration = "SessionExclusionDuration", tag = "1")]
    pub session_exclusion_duration: i32,
    /// If a session meets this condition, the session is excluded from
    /// membership in the segment for the `sessionExclusionDuration`.
    #[prost(message, optional, tag = "2")]
    pub session_exclusion_criteria: ::core::option::Option<SessionSegmentCriteria>,
}
/// Event segments are subsets of events that were triggered on your site or app.
/// for example, all purchase events made in a particular location; app_exception
/// events that occurred on a specific operating system.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventSegment {
    /// Defines which events are included in this segment. Optional.
    #[prost(message, optional, tag = "1")]
    pub event_inclusion_criteria: ::core::option::Option<EventSegmentCriteria>,
    /// Defines which events are excluded in this segment. Optional.
    #[prost(message, optional, tag = "2")]
    pub exclusion: ::core::option::Option<EventSegmentExclusion>,
}
/// An event matches a criteria if the event meet the conditions in the
/// criteria.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventSegmentCriteria {
    /// An event matches this criteria if the event matches each of these
    /// `andConditionGroups`.
    #[prost(message, repeated, tag = "1")]
    pub and_condition_groups: ::prost::alloc::vec::Vec<EventSegmentConditionGroup>,
}
/// Conditions tell Analytics what data to include in or exclude from the
/// segment.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventSegmentConditionGroup {
    /// `conditionScoping` should always be `EVENT_CRITERIA_WITHIN_SAME_EVENT`.
    ///
    /// Optional. If unspecified, a `conditionScoping` of
    /// `EVENT_CRITERIA_WITHIN_SAME_EVENT` is used.
    #[prost(enumeration = "EventCriteriaScoping", tag = "1")]
    pub condition_scoping: i32,
    /// Data is included or excluded from the segment based on if it matches
    /// this expression. Expressions express criteria on dimension, metrics,
    /// and/or parameters.
    #[prost(message, optional, tag = "2")]
    pub segment_filter_expression: ::core::option::Option<SegmentFilterExpression>,
}
/// Specifies which events are excluded in this segment.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventSegmentExclusion {
    /// `eventExclusionDuration` should always be `PERMANENTLY_EXCLUDE`.
    ///
    /// Optional. If unspecified, an `eventExclusionDuration` of
    /// `EVENT_EXCLUSION_PERMANENT` is used.
    #[prost(enumeration = "EventExclusionDuration", tag = "1")]
    pub event_exclusion_duration: i32,
    /// If an event meets this condition, the event is excluded from membership
    /// in the segment for the `eventExclusionDuration`.
    #[prost(message, optional, tag = "2")]
    pub event_exclusion_criteria: ::core::option::Option<EventSegmentCriteria>,
}
/// A segment is a subset of your Analytics data. For example, of your entire set
/// of users, one segment might be users from a particular country or city.
/// Another segment might be users who purchase a particular line of products or
/// who visit a specific part of your site or trigger certain events in your app.
///
/// To learn more, see [GA4 Segment
/// Builder](<https://support.google.com/analytics/answer/9304353>).
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Segment {
    /// The name for this segment. If unspecified, segments are named "Segment".
    /// This name defines string value returned by the `segment` dimension. The
    /// `segment` dimension prefixes segment names by the 1-based index number of
    /// the segment in the request (for example "1. Segment", "2. Segment", etc.).
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// A segment is specified in one scope.
    #[prost(oneof = "segment::OneSegmentScope", tags = "2, 3, 4")]
    pub one_segment_scope: ::core::option::Option<segment::OneSegmentScope>,
}
/// Nested message and enum types in `Segment`.
pub mod segment {
    /// A segment is specified in one scope.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum OneSegmentScope {
        /// User segments are subsets of users who engaged with your site or app.
        #[prost(message, tag = "2")]
        UserSegment(super::UserSegment),
        /// Session segments are subsets of the sessions that occurred on your site
        /// or app.
        #[prost(message, tag = "3")]
        SessionSegment(super::SessionSegment),
        /// Event segments are subsets of events that were triggered on your site or
        /// app.
        #[prost(message, tag = "4")]
        EventSegment(super::EventSegment),
    }
}
/// Expresses combinations of segment filters.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SegmentFilterExpression {
    /// Specify one type of filter for `SegmentFilterExpression`.
    #[prost(oneof = "segment_filter_expression::Expr", tags = "1, 2, 3, 4, 5")]
    pub expr: ::core::option::Option<segment_filter_expression::Expr>,
}
/// Nested message and enum types in `SegmentFilterExpression`.
pub mod segment_filter_expression {
    /// Specify one type of filter for `SegmentFilterExpression`.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Expr {
        /// The SegmentFilterExpression in `andGroup` have an AND relationship.
        #[prost(message, tag = "1")]
        AndGroup(super::SegmentFilterExpressionList),
        /// The SegmentFilterExpression in `orGroup` have an OR relationship.
        #[prost(message, tag = "2")]
        OrGroup(super::SegmentFilterExpressionList),
        /// The SegmentFilterExpression is NOT of `notExpression`.
        #[prost(message, tag = "3")]
        NotExpression(::prost::alloc::boxed::Box<super::SegmentFilterExpression>),
        /// A primitive segment filter.
        #[prost(message, tag = "4")]
        SegmentFilter(super::SegmentFilter),
        /// Creates a filter that matches events of a single event name. If a
        /// parameter filter expression is specified, only the subset of events that
        /// match both the single event name and the parameter filter expressions
        /// match this event filter.
        #[prost(message, tag = "5")]
        SegmentEventFilter(super::SegmentEventFilter),
    }
}
/// A list of segment filter expressions.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SegmentFilterExpressionList {
    /// The list of segment filter expressions
    #[prost(message, repeated, tag = "1")]
    pub expressions: ::prost::alloc::vec::Vec<SegmentFilterExpression>,
}
/// An expression to filter dimension or metric values.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SegmentFilter {
    /// The dimension name or metric name.
    #[prost(string, tag = "1")]
    pub field_name: ::prost::alloc::string::String,
    /// Specifies the scope for the filter.
    #[prost(message, optional, tag = "8")]
    pub filter_scoping: ::core::option::Option<SegmentFilterScoping>,
    /// Specify one type of filter for `Filter`.
    #[prost(oneof = "segment_filter::OneFilter", tags = "4, 5, 6, 7")]
    pub one_filter: ::core::option::Option<segment_filter::OneFilter>,
}
/// Nested message and enum types in `SegmentFilter`.
pub mod segment_filter {
    /// Specify one type of filter for `Filter`.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum OneFilter {
        /// Strings related filter.
        #[prost(message, tag = "4")]
        StringFilter(super::StringFilter),
        /// A filter for in list values.
        #[prost(message, tag = "5")]
        InListFilter(super::InListFilter),
        /// A filter for numeric or date values.
        #[prost(message, tag = "6")]
        NumericFilter(super::NumericFilter),
        /// A filter for between two values.
        #[prost(message, tag = "7")]
        BetweenFilter(super::BetweenFilter),
    }
}
/// Scopings specify how the dimensions & metrics of multiple events
/// should be considered when evaluating a segment filter.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SegmentFilterScoping {
    /// If `atAnyPointInTime` is true, this filter evaluates to true for all
    /// events if it evaluates to true for any event in the date range of the
    /// request.
    ///
    /// This `atAnyPointInTime` parameter does not extend the date range of
    /// events in the report. If `atAnyPointInTime` is true, only events within
    /// the report's date range are considered when evaluating this filter.
    ///
    /// This `atAnyPointInTime` is only able to be specified if the criteria
    /// scoping is `ACROSS_ALL_SESSIONS` and is not able to be specified in
    /// sequences.
    ///
    /// If the criteria scoping is `ACROSS_ALL_SESSIONS`, `atAnyPointInTime` =
    /// false is used if unspecified.
    #[prost(bool, optional, tag = "1")]
    pub at_any_point_in_time: ::core::option::Option<bool>,
}
/// Creates a filter that matches events of a single event name. If a parameter
/// filter expression is specified, only the subset of events that match both the
/// single event name and the parameter filter expressions match this event
/// filter.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SegmentEventFilter {
    /// This filter matches events of this single event name. Event name is
    /// required.
    #[prost(string, optional, tag = "1")]
    pub event_name: ::core::option::Option<::prost::alloc::string::String>,
    /// If specified, this filter matches events that match both the single event
    /// name and the parameter filter expressions.
    ///
    /// Inside the parameter filter expression, only parameter filters are
    /// available.
    #[prost(message, optional, tag = "2")]
    pub segment_parameter_filter_expression: ::core::option::Option<
        SegmentParameterFilterExpression,
    >,
}
/// Expresses combinations of segment filter on parameters.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SegmentParameterFilterExpression {
    /// Specify one type of filter for `SegmentParameterFilterExpression`.
    #[prost(oneof = "segment_parameter_filter_expression::Expr", tags = "1, 2, 3, 4")]
    pub expr: ::core::option::Option<segment_parameter_filter_expression::Expr>,
}
/// Nested message and enum types in `SegmentParameterFilterExpression`.
pub mod segment_parameter_filter_expression {
    /// Specify one type of filter for `SegmentParameterFilterExpression`.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Expr {
        /// The SegmentParameterFilterExpression in `andGroup` have an AND
        /// relationship.
        #[prost(message, tag = "1")]
        AndGroup(super::SegmentParameterFilterExpressionList),
        /// The SegmentParameterFilterExpression in `orGroup` have an OR
        /// relationship.
        #[prost(message, tag = "2")]
        OrGroup(super::SegmentParameterFilterExpressionList),
        /// The SegmentParameterFilterExpression is NOT of `notExpression`.
        #[prost(message, tag = "3")]
        NotExpression(
            ::prost::alloc::boxed::Box<super::SegmentParameterFilterExpression>,
        ),
        /// A primitive segment parameter filter.
        #[prost(message, tag = "4")]
        SegmentParameterFilter(super::SegmentParameterFilter),
    }
}
/// A list of segment parameter filter expressions.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SegmentParameterFilterExpressionList {
    /// The list of segment parameter filter expressions.
    #[prost(message, repeated, tag = "1")]
    pub expressions: ::prost::alloc::vec::Vec<SegmentParameterFilterExpression>,
}
/// An expression to filter parameter values in a segment.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SegmentParameterFilter {
    /// Specifies the scope for the filter.
    #[prost(message, optional, tag = "8")]
    pub filter_scoping: ::core::option::Option<SegmentParameterFilterScoping>,
    /// The field that is being filtered.
    #[prost(oneof = "segment_parameter_filter::OneParameter", tags = "1, 2")]
    pub one_parameter: ::core::option::Option<segment_parameter_filter::OneParameter>,
    /// Specify one type of filter.
    #[prost(oneof = "segment_parameter_filter::OneFilter", tags = "4, 5, 6, 7")]
    pub one_filter: ::core::option::Option<segment_parameter_filter::OneFilter>,
}
/// Nested message and enum types in `SegmentParameterFilter`.
pub mod segment_parameter_filter {
    /// The field that is being filtered.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum OneParameter {
        /// This filter will be evaluated on the specified event parameter. Event
        /// parameters are logged as parameters of the event. Event parameters
        /// include fields like "firebase_screen" & "currency".
        ///
        /// Event parameters can only be used in segments & funnels and can only be
        /// used in a descendent filter from an EventFilter. In a descendent filter
        /// from an EventFilter either event or item parameters should be used.
        #[prost(string, tag = "1")]
        EventParameterName(::prost::alloc::string::String),
        /// This filter will be evaluated on the specified item parameter. Item
        /// parameters are logged as parameters in the item array. Item parameters
        /// include fields like "item_name" & "item_category".
        ///
        /// Item parameters can only be used in segments & funnels and can only be
        /// used in a descendent filter from an EventFilter. In a descendent filter
        /// from an EventFilter either event or item parameters should be used.
        ///
        /// Item parameters are only available in ecommerce events. To learn more
        /// about ecommerce events, see the \[Measure ecommerce\]
        /// (<https://developers.google.com/analytics/devguides/collection/ga4/ecommerce>)
        /// guide.
        #[prost(string, tag = "2")]
        ItemParameterName(::prost::alloc::string::String),
    }
    /// Specify one type of filter.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum OneFilter {
        /// Strings related filter.
        #[prost(message, tag = "4")]
        StringFilter(super::StringFilter),
        /// A filter for in list values.
        #[prost(message, tag = "5")]
        InListFilter(super::InListFilter),
        /// A filter for numeric or date values.
        #[prost(message, tag = "6")]
        NumericFilter(super::NumericFilter),
        /// A filter for between two values.
        #[prost(message, tag = "7")]
        BetweenFilter(super::BetweenFilter),
    }
}
/// Scopings specify how multiple events should be considered when evaluating a
/// segment parameter filter.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SegmentParameterFilterScoping {
    /// Accumulates the parameter over the specified period of days before
    /// applying the filter. Only supported if criteria scoping is
    /// `ACROSS_ALL_SESSIONS` or `WITHIN_SAME_SESSION`. Only supported if the
    /// parameter is `event_count`.
    ///
    /// For example if `inAnyNDayPeriod` is 3, the event_name is "purchase",
    /// the event parameter is "event_count", and the Filter's criteria is
    /// greater than 5, this filter will accumulate the event count of purchase
    /// events over every 3 consecutive day period in the report's date range; a
    /// user will pass this Filter's criteria to be included in this segment if
    /// their count of purchase events exceeds 5 in any 3 consecutive day period.
    /// For example, the periods 2021-11-01 to 2021-11-03, 2021-11-02 to
    /// 2021-11-04, 2021-11-03 to 2021-11-05, and etc. will be considered.
    ///
    /// The date range is not extended for the purpose of having a full N day
    /// window near the start of the date range. For example if a report is for
    /// 2021-11-01 to 2021-11-10 and `inAnyNDayPeriod` = 3, the first two day
    /// period will be effectively shortened because no event data outside the
    /// report's date range will be read. For example, the first four periods
    /// will effectively be: 2021-11-01 to 2021-11-01, 2021-11-01 to 2021-11-02,
    /// 2021-11-01 to 2021-11-03, and 2021-11-02 to 2021-11-04.
    ///
    /// `inAnyNDayPeriod` is optional. If not specified, the
    /// `segmentParameterFilter` is applied to each event individually.
    #[prost(int64, optional, tag = "1")]
    pub in_any_n_day_period: ::core::option::Option<i64>,
}
/// Expresses combinations of funnel filters.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FunnelFilterExpression {
    /// Specify one type of filter for `FunnelFilterExpression`.
    #[prost(oneof = "funnel_filter_expression::Expr", tags = "1, 2, 3, 4, 5")]
    pub expr: ::core::option::Option<funnel_filter_expression::Expr>,
}
/// Nested message and enum types in `FunnelFilterExpression`.
pub mod funnel_filter_expression {
    /// Specify one type of filter for `FunnelFilterExpression`.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Expr {
        /// The FunnelFilterExpression in `andGroup` have an AND relationship.
        #[prost(message, tag = "1")]
        AndGroup(super::FunnelFilterExpressionList),
        /// The FunnelFilterExpression in `orGroup` have an OR relationship.
        #[prost(message, tag = "2")]
        OrGroup(super::FunnelFilterExpressionList),
        /// The FunnelFilterExpression is NOT of `notExpression`.
        #[prost(message, tag = "3")]
        NotExpression(::prost::alloc::boxed::Box<super::FunnelFilterExpression>),
        /// A funnel filter for a dimension or metric.
        #[prost(message, tag = "4")]
        FunnelFieldFilter(super::FunnelFieldFilter),
        /// Creates a filter that matches events of a single event name. If a
        /// parameter filter expression is specified, only the subset of events that
        /// match both the single event name and the parameter filter expressions
        /// match this event filter.
        #[prost(message, tag = "5")]
        FunnelEventFilter(super::FunnelEventFilter),
    }
}
/// A list of funnel filter expressions.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FunnelFilterExpressionList {
    /// The list of funnel filter expressions.
    #[prost(message, repeated, tag = "1")]
    pub expressions: ::prost::alloc::vec::Vec<FunnelFilterExpression>,
}
/// An expression to filter dimension or metric values.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FunnelFieldFilter {
    /// The dimension name or metric name.
    #[prost(string, tag = "1")]
    pub field_name: ::prost::alloc::string::String,
    /// Specify one type of filter.
    #[prost(oneof = "funnel_field_filter::OneFilter", tags = "4, 5, 6, 7")]
    pub one_filter: ::core::option::Option<funnel_field_filter::OneFilter>,
}
/// Nested message and enum types in `FunnelFieldFilter`.
pub mod funnel_field_filter {
    /// Specify one type of filter.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum OneFilter {
        /// Strings related filter.
        #[prost(message, tag = "4")]
        StringFilter(super::StringFilter),
        /// A filter for in list values.
        #[prost(message, tag = "5")]
        InListFilter(super::InListFilter),
        /// A filter for numeric or date values.
        #[prost(message, tag = "6")]
        NumericFilter(super::NumericFilter),
        /// A filter for between two values.
        #[prost(message, tag = "7")]
        BetweenFilter(super::BetweenFilter),
    }
}
/// Creates a filter that matches events of a single event name. If a parameter
/// filter expression is specified, only the subset of events that match both the
/// single event name and the parameter filter expressions match this event
/// filter.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FunnelEventFilter {
    /// This filter matches events of this single event name. Event name is
    /// required.
    #[prost(string, optional, tag = "1")]
    pub event_name: ::core::option::Option<::prost::alloc::string::String>,
    /// If specified, this filter matches events that match both the single event
    /// name and the parameter filter expressions.
    ///
    /// Inside the parameter filter expression, only parameter filters are
    /// available.
    #[prost(message, optional, tag = "2")]
    pub funnel_parameter_filter_expression: ::core::option::Option<
        FunnelParameterFilterExpression,
    >,
}
/// Expresses combinations of funnel filters on parameters.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FunnelParameterFilterExpression {
    /// Specify one type of filter for `FunnelParameterFilterExpression`.
    #[prost(oneof = "funnel_parameter_filter_expression::Expr", tags = "1, 2, 3, 4")]
    pub expr: ::core::option::Option<funnel_parameter_filter_expression::Expr>,
}
/// Nested message and enum types in `FunnelParameterFilterExpression`.
pub mod funnel_parameter_filter_expression {
    /// Specify one type of filter for `FunnelParameterFilterExpression`.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Expr {
        /// The FunnelParameterFilterExpression in `andGroup` have an AND
        /// relationship.
        #[prost(message, tag = "1")]
        AndGroup(super::FunnelParameterFilterExpressionList),
        /// The FunnelParameterFilterExpression in `orGroup` have an OR
        /// relationship.
        #[prost(message, tag = "2")]
        OrGroup(super::FunnelParameterFilterExpressionList),
        /// The FunnelParameterFilterExpression is NOT of `notExpression`.
        #[prost(message, tag = "3")]
        NotExpression(
            ::prost::alloc::boxed::Box<super::FunnelParameterFilterExpression>,
        ),
        /// A primitive funnel parameter filter.
        #[prost(message, tag = "4")]
        FunnelParameterFilter(super::FunnelParameterFilter),
    }
}
/// A list of funnel parameter filter expressions.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FunnelParameterFilterExpressionList {
    /// The list of funnel parameter filter expressions.
    #[prost(message, repeated, tag = "1")]
    pub expressions: ::prost::alloc::vec::Vec<FunnelParameterFilterExpression>,
}
/// An expression to filter parameter values in a funnel.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FunnelParameterFilter {
    /// The field that is being filtered.
    #[prost(oneof = "funnel_parameter_filter::OneParameter", tags = "1, 2")]
    pub one_parameter: ::core::option::Option<funnel_parameter_filter::OneParameter>,
    /// Specify one type of filter.
    #[prost(oneof = "funnel_parameter_filter::OneFilter", tags = "4, 5, 6, 7")]
    pub one_filter: ::core::option::Option<funnel_parameter_filter::OneFilter>,
}
/// Nested message and enum types in `FunnelParameterFilter`.
pub mod funnel_parameter_filter {
    /// The field that is being filtered.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum OneParameter {
        /// This filter will be evaluated on the specified event parameter. Event
        /// parameters are logged as parameters of the event. Event parameters
        /// include fields like "firebase_screen" & "currency".
        ///
        /// Event parameters can only be used in segments & funnels and can only be
        /// used in a descendent filter from an EventFilter. In a descendent filter
        /// from an EventFilter either event or item parameters should be used.
        #[prost(string, tag = "1")]
        EventParameterName(::prost::alloc::string::String),
        /// This filter will be evaluated on the specified item parameter. Item
        /// parameters are logged as parameters in the item array. Item parameters
        /// include fields like "item_name" & "item_category".
        ///
        /// Item parameters can only be used in segments & funnels and can only be
        /// used in a descendent filter from an EventFilter. In a descendent filter
        /// from an EventFilter either event or item parameters should be used.
        ///
        /// Item parameters are only available in ecommerce events. To learn more
        /// about ecommerce events, see the \[Measure ecommerce\]
        /// (<https://developers.google.com/analytics/devguides/collection/ga4/ecommerce>)
        /// guide.
        #[prost(string, tag = "2")]
        ItemParameterName(::prost::alloc::string::String),
    }
    /// Specify one type of filter.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum OneFilter {
        /// Strings related filter.
        #[prost(message, tag = "4")]
        StringFilter(super::StringFilter),
        /// A filter for in list values.
        #[prost(message, tag = "5")]
        InListFilter(super::InListFilter),
        /// A filter for numeric or date values.
        #[prost(message, tag = "6")]
        NumericFilter(super::NumericFilter),
        /// A filter for between two values.
        #[prost(message, tag = "7")]
        BetweenFilter(super::BetweenFilter),
    }
}
/// The funnel report's response metadata carries additional information about
/// the funnel report.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FunnelResponseMetadata {
    /// If funnel report results are
    /// [sampled](<https://support.google.com/analytics/answer/13331292>), this
    /// describes what percentage of events were used in this funnel report. One
    /// `samplingMetadatas` is populated for each date range. Each
    /// `samplingMetadatas` corresponds to a date range in order that date ranges
    /// were specified in the request.
    ///
    /// However if the results are not sampled, this field will not be defined.
    #[prost(message, repeated, tag = "1")]
    pub sampling_metadatas: ::prost::alloc::vec::Vec<SamplingMetadata>,
}
/// If funnel report results are
/// [sampled](<https://support.google.com/analytics/answer/13331292>), this
/// metadata describes what percentage of events were used in this funnel
/// report for a date range. Sampling is the practice of analyzing a subset of
/// all data in order to uncover the meaningful information in the larger data
/// set.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SamplingMetadata {
    /// The total number of events read in this sampled report for a date range.
    /// This is the size of the subset this property's data that was analyzed in
    /// this funnel report.
    #[prost(int64, tag = "1")]
    pub samples_read_count: i64,
    /// The total number of events present in this property's data that could
    /// have been analyzed in this funnel report for a date range. Sampling
    /// uncovers the meaningful information about the larger data set, and this
    /// is the size of the larger data set.
    ///
    /// To calculate the percentage of available data that was used in this
    /// funnel report, compute `samplesReadCount/samplingSpaceSize`.
    #[prost(int64, tag = "2")]
    pub sampling_space_size: i64,
}
/// Scoping specifies which events are considered when evaluating if a user
/// meets a criteria.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum UserCriteriaScoping {
    /// Unspecified criteria scoping. Do not specify.
    Unspecified = 0,
    /// If the criteria is satisfied within one event, the user matches the
    /// criteria.
    UserCriteriaWithinSameEvent = 1,
    /// If the criteria is satisfied within one session, the user matches the
    /// criteria.
    UserCriteriaWithinSameSession = 2,
    /// If the criteria is satisfied by any events for the user, the user
    /// matches the criteria.
    UserCriteriaAcrossAllSessions = 3,
}
impl UserCriteriaScoping {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            UserCriteriaScoping::Unspecified => "USER_CRITERIA_SCOPING_UNSPECIFIED",
            UserCriteriaScoping::UserCriteriaWithinSameEvent => {
                "USER_CRITERIA_WITHIN_SAME_EVENT"
            }
            UserCriteriaScoping::UserCriteriaWithinSameSession => {
                "USER_CRITERIA_WITHIN_SAME_SESSION"
            }
            UserCriteriaScoping::UserCriteriaAcrossAllSessions => {
                "USER_CRITERIA_ACROSS_ALL_SESSIONS"
            }
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "USER_CRITERIA_SCOPING_UNSPECIFIED" => Some(Self::Unspecified),
            "USER_CRITERIA_WITHIN_SAME_EVENT" => Some(Self::UserCriteriaWithinSameEvent),
            "USER_CRITERIA_WITHIN_SAME_SESSION" => {
                Some(Self::UserCriteriaWithinSameSession)
            }
            "USER_CRITERIA_ACROSS_ALL_SESSIONS" => {
                Some(Self::UserCriteriaAcrossAllSessions)
            }
            _ => None,
        }
    }
}
/// Enumerates options for how long an exclusion will last if a user matches
/// the `userExclusionCriteria`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum UserExclusionDuration {
    /// Unspecified exclusion duration. Do not specify.
    Unspecified = 0,
    /// Temporarily exclude users from the segment during periods when the
    /// user meets the `userExclusionCriteria` condition.
    UserExclusionTemporary = 1,
    /// Permanently exclude users from the segment if the user ever meets the
    /// `userExclusionCriteria` condition.
    UserExclusionPermanent = 2,
}
impl UserExclusionDuration {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            UserExclusionDuration::Unspecified => "USER_EXCLUSION_DURATION_UNSPECIFIED",
            UserExclusionDuration::UserExclusionTemporary => "USER_EXCLUSION_TEMPORARY",
            UserExclusionDuration::UserExclusionPermanent => "USER_EXCLUSION_PERMANENT",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "USER_EXCLUSION_DURATION_UNSPECIFIED" => Some(Self::Unspecified),
            "USER_EXCLUSION_TEMPORARY" => Some(Self::UserExclusionTemporary),
            "USER_EXCLUSION_PERMANENT" => Some(Self::UserExclusionPermanent),
            _ => None,
        }
    }
}
/// Scoping specifies which events are considered when evaluating if a
/// session meets a criteria.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SessionCriteriaScoping {
    /// Unspecified criteria scoping. Do not specify.
    Unspecified = 0,
    /// If the criteria is satisfied within one event, the session matches the
    /// criteria.
    SessionCriteriaWithinSameEvent = 1,
    /// If the criteria is satisfied within one session, the session matches
    /// the criteria.
    SessionCriteriaWithinSameSession = 2,
}
impl SessionCriteriaScoping {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SessionCriteriaScoping::Unspecified => "SESSION_CRITERIA_SCOPING_UNSPECIFIED",
            SessionCriteriaScoping::SessionCriteriaWithinSameEvent => {
                "SESSION_CRITERIA_WITHIN_SAME_EVENT"
            }
            SessionCriteriaScoping::SessionCriteriaWithinSameSession => {
                "SESSION_CRITERIA_WITHIN_SAME_SESSION"
            }
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SESSION_CRITERIA_SCOPING_UNSPECIFIED" => Some(Self::Unspecified),
            "SESSION_CRITERIA_WITHIN_SAME_EVENT" => {
                Some(Self::SessionCriteriaWithinSameEvent)
            }
            "SESSION_CRITERIA_WITHIN_SAME_SESSION" => {
                Some(Self::SessionCriteriaWithinSameSession)
            }
            _ => None,
        }
    }
}
/// Enumerates options for how long an exclusion will last if a session
/// matches the `sessionExclusionCriteria`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SessionExclusionDuration {
    /// Unspecified exclusion duration. Do not specify.
    Unspecified = 0,
    /// Temporarily exclude sessions from the segment during periods when the
    /// session meets the `sessionExclusionCriteria` condition.
    SessionExclusionTemporary = 1,
    /// Permanently exclude sessions from the segment if the session ever meets
    /// the `sessionExclusionCriteria` condition.
    SessionExclusionPermanent = 2,
}
impl SessionExclusionDuration {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SessionExclusionDuration::Unspecified => {
                "SESSION_EXCLUSION_DURATION_UNSPECIFIED"
            }
            SessionExclusionDuration::SessionExclusionTemporary => {
                "SESSION_EXCLUSION_TEMPORARY"
            }
            SessionExclusionDuration::SessionExclusionPermanent => {
                "SESSION_EXCLUSION_PERMANENT"
            }
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SESSION_EXCLUSION_DURATION_UNSPECIFIED" => Some(Self::Unspecified),
            "SESSION_EXCLUSION_TEMPORARY" => Some(Self::SessionExclusionTemporary),
            "SESSION_EXCLUSION_PERMANENT" => Some(Self::SessionExclusionPermanent),
            _ => None,
        }
    }
}
/// Scoping specifies which events are considered when evaluating if an event
/// meets a criteria.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum EventCriteriaScoping {
    /// Unspecified criteria scoping. Do not specify.
    Unspecified = 0,
    /// If the criteria is satisfied within one event, the event matches the
    /// criteria.
    EventCriteriaWithinSameEvent = 1,
}
impl EventCriteriaScoping {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            EventCriteriaScoping::Unspecified => "EVENT_CRITERIA_SCOPING_UNSPECIFIED",
            EventCriteriaScoping::EventCriteriaWithinSameEvent => {
                "EVENT_CRITERIA_WITHIN_SAME_EVENT"
            }
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "EVENT_CRITERIA_SCOPING_UNSPECIFIED" => Some(Self::Unspecified),
            "EVENT_CRITERIA_WITHIN_SAME_EVENT" => {
                Some(Self::EventCriteriaWithinSameEvent)
            }
            _ => None,
        }
    }
}
/// Enumerates options for how long an exclusion will last if an event
/// matches the `eventExclusionCriteria`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum EventExclusionDuration {
    /// Unspecified exclusion duration. Do not specify.
    Unspecified = 0,
    /// Permanently exclude events from the segment if the event ever meets
    /// the `eventExclusionCriteria` condition.
    EventExclusionPermanent = 1,
}
impl EventExclusionDuration {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            EventExclusionDuration::Unspecified => "EVENT_EXCLUSION_DURATION_UNSPECIFIED",
            EventExclusionDuration::EventExclusionPermanent => {
                "EVENT_EXCLUSION_PERMANENT"
            }
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "EVENT_EXCLUSION_DURATION_UNSPECIFIED" => Some(Self::Unspecified),
            "EVENT_EXCLUSION_PERMANENT" => Some(Self::EventExclusionPermanent),
            _ => None,
        }
    }
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
impl MetricType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            MetricType::Unspecified => "METRIC_TYPE_UNSPECIFIED",
            MetricType::TypeInteger => "TYPE_INTEGER",
            MetricType::TypeFloat => "TYPE_FLOAT",
            MetricType::TypeSeconds => "TYPE_SECONDS",
            MetricType::TypeMilliseconds => "TYPE_MILLISECONDS",
            MetricType::TypeMinutes => "TYPE_MINUTES",
            MetricType::TypeHours => "TYPE_HOURS",
            MetricType::TypeStandard => "TYPE_STANDARD",
            MetricType::TypeCurrency => "TYPE_CURRENCY",
            MetricType::TypeFeet => "TYPE_FEET",
            MetricType::TypeMiles => "TYPE_MILES",
            MetricType::TypeMeters => "TYPE_METERS",
            MetricType::TypeKilometers => "TYPE_KILOMETERS",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "METRIC_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "TYPE_INTEGER" => Some(Self::TypeInteger),
            "TYPE_FLOAT" => Some(Self::TypeFloat),
            "TYPE_SECONDS" => Some(Self::TypeSeconds),
            "TYPE_MILLISECONDS" => Some(Self::TypeMilliseconds),
            "TYPE_MINUTES" => Some(Self::TypeMinutes),
            "TYPE_HOURS" => Some(Self::TypeHours),
            "TYPE_STANDARD" => Some(Self::TypeStandard),
            "TYPE_CURRENCY" => Some(Self::TypeCurrency),
            "TYPE_FEET" => Some(Self::TypeFeet),
            "TYPE_MILES" => Some(Self::TypeMiles),
            "TYPE_METERS" => Some(Self::TypeMeters),
            "TYPE_KILOMETERS" => Some(Self::TypeKilometers),
            _ => None,
        }
    }
}
/// A request to create a new recurring audience list.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateRecurringAudienceListRequest {
    /// Required. The parent resource where this recurring audience list will be
    /// created. Format: `properties/{property}`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The recurring audience list to create.
    #[prost(message, optional, tag = "2")]
    pub recurring_audience_list: ::core::option::Option<RecurringAudienceList>,
}
/// A recurring audience list produces new audience lists each day. Audience
/// lists are users in an audience at the time of the list's creation. A
/// recurring audience list ensures that you have audience list based on the most
/// recent data available for use each day.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RecurringAudienceList {
    /// Output only. Identifier. The recurring audience list resource name assigned
    /// during creation. This resource name identifies this
    /// `RecurringAudienceList`.
    ///
    /// Format:
    /// `properties/{property}/recurringAudienceLists/{recurring_audience_list}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The audience resource name. This resource name identifies the
    /// audience being listed and is shared between the Analytics Data & Admin
    /// APIs.
    ///
    /// Format: `properties/{property}/audiences/{audience}`
    #[prost(string, tag = "2")]
    pub audience: ::prost::alloc::string::String,
    /// Output only. The descriptive display name for this audience. For example,
    /// "Purchasers".
    #[prost(string, tag = "3")]
    pub audience_display_name: ::prost::alloc::string::String,
    /// Required. The dimensions requested and displayed in the audience list
    /// response.
    #[prost(message, repeated, tag = "4")]
    pub dimensions: ::prost::alloc::vec::Vec<AudienceDimension>,
    /// Optional. The number of remaining days that a recurring audience export
    /// will produce an audience list instance. This counter decreases by one each
    /// day, and when it reaches zero, no new audience lists will be created.
    ///
    /// Recurring audience list request for Analytics 360 properties default to 180
    /// days and have a maximum of 365 days. Requests for standard Analytics
    /// properties default to 14 days and have a maximum of 30 days.
    ///
    /// The minimum value allowed during creation is 1. Requests above their
    /// respective maximum will be coerced to their maximum.
    #[prost(int32, optional, tag = "5")]
    pub active_days_remaining: ::core::option::Option<i32>,
    /// Output only. Audience list resource names for audience list instances
    /// created for this recurring audience list. One audience list is created for
    /// each day, and the audience list will be listed here.
    ///
    /// This list is ordered with the most recently created audience list first.
    #[prost(string, repeated, tag = "6")]
    pub audience_lists: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// A request to retrieve configuration metadata about a specific recurring
/// audience list.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRecurringAudienceListRequest {
    /// Required. The recurring audience list resource name.
    /// Format:
    /// `properties/{property}/recurringAudienceLists/{recurring_audience_list}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// A request to list all recurring audience lists for a property.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListRecurringAudienceListsRequest {
    /// Required. All recurring audience lists for this property will be listed in
    /// the response. Format: `properties/{property}`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. The maximum number of recurring audience lists to return. The
    /// service may return fewer than this value. If unspecified, at most 200
    /// recurring audience lists will be returned. The maximum value is 1000
    /// (higher values will be coerced to the maximum).
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. A page token, received from a previous
    /// `ListRecurringAudienceLists` call. Provide this to retrieve the subsequent
    /// page.
    ///
    /// When paginating, all other parameters provided to
    /// `ListRecurringAudienceLists` must match the call that provided the page
    /// token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// A list of all recurring audience lists for a property.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListRecurringAudienceListsResponse {
    /// Each recurring audience list for a property.
    #[prost(message, repeated, tag = "1")]
    pub recurring_audience_lists: ::prost::alloc::vec::Vec<RecurringAudienceList>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, optional, tag = "2")]
    pub next_page_token: ::core::option::Option<::prost::alloc::string::String>,
}
/// A request to retrieve configuration metadata about a specific audience list.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAudienceListRequest {
    /// Required. The audience list resource name.
    /// Format: `properties/{property}/audienceLists/{audience_list}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// A request to list all audience lists for a property.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAudienceListsRequest {
    /// Required. All audience lists for this property will be listed in the
    /// response. Format: `properties/{property}`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. The maximum number of audience lists to return. The service may
    /// return fewer than this value. If unspecified, at most 200 audience lists
    /// will be returned. The maximum value is 1000 (higher values will be coerced
    /// to the maximum).
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. A page token, received from a previous `ListAudienceLists` call.
    /// Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to `ListAudienceLists` must
    /// match the call that provided the page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// A list of all audience lists for a property.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAudienceListsResponse {
    /// Each audience list for a property.
    #[prost(message, repeated, tag = "1")]
    pub audience_lists: ::prost::alloc::vec::Vec<AudienceList>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, optional, tag = "2")]
    pub next_page_token: ::core::option::Option<::prost::alloc::string::String>,
}
/// A request to create a new audience list.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateAudienceListRequest {
    /// Required. The parent resource where this audience list will be created.
    /// Format: `properties/{property}`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The audience list to create.
    #[prost(message, optional, tag = "2")]
    pub audience_list: ::core::option::Option<AudienceList>,
}
/// An audience list is a list of users in an audience at the time of the list's
/// creation. One audience may have multiple audience lists created for different
/// days.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AudienceList {
    /// Output only. Identifier. The audience list resource name assigned during
    /// creation. This resource name identifies this `AudienceList`.
    ///
    /// Format: `properties/{property}/audienceLists/{audience_list}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The audience resource name. This resource name identifies the
    /// audience being listed and is shared between the Analytics Data & Admin
    /// APIs.
    ///
    /// Format: `properties/{property}/audiences/{audience}`
    #[prost(string, tag = "2")]
    pub audience: ::prost::alloc::string::String,
    /// Output only. The descriptive display name for this audience. For example,
    /// "Purchasers".
    #[prost(string, tag = "3")]
    pub audience_display_name: ::prost::alloc::string::String,
    /// Required. The dimensions requested and displayed in the query response.
    #[prost(message, repeated, tag = "4")]
    pub dimensions: ::prost::alloc::vec::Vec<AudienceDimension>,
    /// Output only. The current state for this AudienceList.
    #[prost(enumeration = "audience_list::State", optional, tag = "5")]
    pub state: ::core::option::Option<i32>,
    /// Output only. The time when CreateAudienceList was called and the
    /// AudienceList began the `CREATING` state.
    #[prost(message, optional, tag = "6")]
    pub begin_creating_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The total quota tokens charged during creation of the
    /// AudienceList. Because this token count is based on activity from the
    /// `CREATING` state, this tokens charged will be fixed once an AudienceList
    /// enters the `ACTIVE` or `FAILED` states.
    #[prost(int32, tag = "7")]
    pub creation_quota_tokens_charged: i32,
    /// Output only. The total number of rows in the AudienceList result.
    #[prost(int32, optional, tag = "8")]
    pub row_count: ::core::option::Option<i32>,
    /// Output only. Error message is populated when an audience list fails during
    /// creation. A common reason for such a failure is quota exhaustion.
    #[prost(string, optional, tag = "9")]
    pub error_message: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. The percentage completed for this audience export ranging
    /// between 0 to 100.
    #[prost(double, optional, tag = "11")]
    pub percentage_completed: ::core::option::Option<f64>,
    /// Output only. The recurring audience list that created this audience list.
    /// Recurring audience lists create audience lists daily.
    ///
    /// If audience lists are created directly, they will have no associated
    /// recurring audience list, and this field will be blank.
    #[prost(string, optional, tag = "12")]
    pub recurring_audience_list: ::core::option::Option<::prost::alloc::string::String>,
}
/// Nested message and enum types in `AudienceList`.
pub mod audience_list {
    /// The AudienceList currently exists in this state.
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum State {
        /// Unspecified state will never be used.
        Unspecified = 0,
        /// The AudienceList is currently creating and will be available in the
        /// future. Creating occurs immediately after the CreateAudienceList call.
        Creating = 1,
        /// The AudienceList is fully created and ready for querying. An AudienceList
        /// is updated to active asynchronously from a request; this occurs some
        /// time (for example 15 minutes) after the initial create call.
        Active = 2,
        /// The AudienceList failed to be created. It is possible that re-requesting
        /// this audience list will succeed.
        Failed = 3,
    }
    impl State {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                State::Unspecified => "STATE_UNSPECIFIED",
                State::Creating => "CREATING",
                State::Active => "ACTIVE",
                State::Failed => "FAILED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "CREATING" => Some(Self::Creating),
                "ACTIVE" => Some(Self::Active),
                "FAILED" => Some(Self::Failed),
                _ => None,
            }
        }
    }
}
/// This metadata is currently blank.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AudienceListMetadata {}
/// A request to list users in an audience list.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAudienceListRequest {
    /// Required. The name of the audience list to retrieve users from.
    /// Format: `properties/{property}/audienceLists/{audience_list}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. The row count of the start row. The first row is counted as row
    /// 0.
    ///
    /// When paging, the first request does not specify offset; or equivalently,
    /// sets offset to 0; the first request returns the first `limit` of rows. The
    /// second request sets offset to the `limit` of the first request; the second
    /// request returns the second `limit` of rows.
    ///
    /// To learn more about this pagination parameter, see
    /// [Pagination](<https://developers.google.com/analytics/devguides/reporting/data/v1/basics#pagination>).
    #[prost(int64, tag = "2")]
    pub offset: i64,
    /// Optional. The number of rows to return. If unspecified, 10,000 rows are
    /// returned. The API returns a maximum of 250,000 rows per request, no matter
    /// how many you ask for. `limit` must be positive.
    ///
    /// The API can also return fewer rows than the requested `limit`, if there
    /// aren't as many dimension values as the `limit`.
    ///
    /// To learn more about this pagination parameter, see
    /// [Pagination](<https://developers.google.com/analytics/devguides/reporting/data/v1/basics#pagination>).
    #[prost(int64, tag = "3")]
    pub limit: i64,
}
/// A list of users in an audience list.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAudienceListResponse {
    /// Configuration data about AudienceList being queried. Returned to help
    /// interpret the audience rows in this response. For example, the dimensions
    /// in this AudienceList correspond to the columns in the AudienceRows.
    #[prost(message, optional, tag = "1")]
    pub audience_list: ::core::option::Option<AudienceList>,
    /// Rows for each user in an audience list. The number of rows in this
    /// response will be less than or equal to request's page size.
    #[prost(message, repeated, tag = "2")]
    pub audience_rows: ::prost::alloc::vec::Vec<AudienceRow>,
    /// The total number of rows in the AudienceList result. `rowCount` is
    /// independent of the number of rows returned in the response, the `limit`
    /// request parameter, and the `offset` request parameter. For example if a
    /// query returns 175 rows and includes `limit` of 50 in the API request, the
    /// response will contain `rowCount` of 175 but only 50 rows.
    ///
    /// To learn more about this pagination parameter, see
    /// [Pagination](<https://developers.google.com/analytics/devguides/reporting/data/v1/basics#pagination>).
    #[prost(int32, optional, tag = "3")]
    pub row_count: ::core::option::Option<i32>,
}
/// A request to export users in an audience list to a Google Sheet.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SheetExportAudienceListRequest {
    /// Required. The name of the audience list to retrieve users from.
    /// Format: `properties/{property}/audienceLists/{audience_list}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. The row count of the start row. The first row is counted as row
    /// 0.
    ///
    /// When paging, the first request does not specify offset; or equivalently,
    /// sets offset to 0; the first request returns the first `limit` of rows. The
    /// second request sets offset to the `limit` of the first request; the second
    /// request returns the second `limit` of rows.
    ///
    /// To learn more about this pagination parameter, see
    /// [Pagination](<https://developers.google.com/analytics/devguides/reporting/data/v1/basics#pagination>).
    #[prost(int64, tag = "2")]
    pub offset: i64,
    /// Optional. The number of rows to return. If unspecified, 10,000 rows are
    /// returned. The API returns a maximum of 250,000 rows per request, no matter
    /// how many you ask for. `limit` must be positive.
    ///
    /// The API can also return fewer rows than the requested `limit`, if there
    /// aren't as many dimension values as the `limit`.
    ///
    /// To learn more about this pagination parameter, see
    /// [Pagination](<https://developers.google.com/analytics/devguides/reporting/data/v1/basics#pagination>).
    #[prost(int64, tag = "3")]
    pub limit: i64,
}
/// The created Google Sheet with the list of users in an audience list.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SheetExportAudienceListResponse {
    /// A uri for you to visit in your browser to view the Google Sheet.
    #[prost(string, optional, tag = "1")]
    pub spreadsheet_uri: ::core::option::Option<::prost::alloc::string::String>,
    /// An ID that identifies the created Google Sheet resource.
    #[prost(string, optional, tag = "2")]
    pub spreadsheet_id: ::core::option::Option<::prost::alloc::string::String>,
    /// The total number of rows in the AudienceList result. `rowCount` is
    /// independent of the number of rows returned in the response, the `limit`
    /// request parameter, and the `offset` request parameter. For example if a
    /// query returns 175 rows and includes `limit` of 50 in the API request, the
    /// response will contain `rowCount` of 175 but only 50 rows.
    ///
    /// To learn more about this pagination parameter, see
    /// [Pagination](<https://developers.google.com/analytics/devguides/reporting/data/v1/basics#pagination>).
    #[prost(int32, optional, tag = "3")]
    pub row_count: ::core::option::Option<i32>,
    /// Configuration data about AudienceList being exported. Returned to help
    /// interpret the AudienceList in the Google Sheet of this response.
    ///
    /// For example, the AudienceList may have more rows than are present in the
    /// Google Sheet, and in that case, you may want to send an additional sheet
    /// export request with a different `offset` value to retrieve the next page of
    /// rows in an additional Google Sheet.
    #[prost(message, optional, tag = "4")]
    pub audience_list: ::core::option::Option<AudienceList>,
}
/// Dimension value attributes for the audience user row.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AudienceRow {
    /// Each dimension value attribute for an audience user. One dimension value
    /// will be added for each dimension column requested.
    #[prost(message, repeated, tag = "1")]
    pub dimension_values: ::prost::alloc::vec::Vec<AudienceDimensionValue>,
}
/// An audience dimension is a user attribute. Specific user attributed are
/// requested and then later returned in the `QueryAudienceListResponse`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AudienceDimension {
    /// Optional. The API name of the dimension. See the [API
    /// Dimensions](<https://developers.google.com/analytics/devguides/reporting/data/v1/audience-list-api-schema#dimensions>)
    /// for the list of dimension names.
    #[prost(string, tag = "1")]
    pub dimension_name: ::prost::alloc::string::String,
}
/// The value of a dimension.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AudienceDimensionValue {
    /// One kind of dimension value.
    #[prost(oneof = "audience_dimension_value::OneValue", tags = "1")]
    pub one_value: ::core::option::Option<audience_dimension_value::OneValue>,
}
/// Nested message and enum types in `AudienceDimensionValue`.
pub mod audience_dimension_value {
    /// One kind of dimension value.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum OneValue {
        /// Value as a string if the dimension type is a string.
        #[prost(string, tag = "1")]
        Value(::prost::alloc::string::String),
    }
}
/// The request for a funnel report.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RunFunnelReportRequest {
    /// Optional. A Google Analytics GA4 property identifier whose events are
    /// tracked. Specified in the URL path and not the body. To learn more, see
    /// [where to find your Property
    /// ID](<https://developers.google.com/analytics/devguides/reporting/data/v1/property-id>).
    /// Within a batch request, this property should either be unspecified or
    /// consistent with the batch-level property.
    ///
    /// Example: properties/1234
    #[prost(string, tag = "1")]
    pub property: ::prost::alloc::string::String,
    /// Optional. Date ranges of data to read. If multiple date ranges are
    /// requested, each response row will contain a zero based date range index. If
    /// two date ranges overlap, the event data for the overlapping days is
    /// included in the response rows for both date ranges.
    #[prost(message, repeated, tag = "2")]
    pub date_ranges: ::prost::alloc::vec::Vec<DateRange>,
    /// Optional. The configuration of this request's funnel. This funnel
    /// configuration is required.
    #[prost(message, optional, tag = "3")]
    pub funnel: ::core::option::Option<Funnel>,
    /// Optional. If specified, this breakdown adds a dimension to the funnel table
    /// sub report response. This breakdown dimension expands each funnel step to
    /// the unique values of the breakdown dimension. For example, a breakdown by
    /// the `deviceCategory` dimension will create rows for `mobile`, `tablet`,
    /// `desktop`, and the total.
    #[prost(message, optional, tag = "4")]
    pub funnel_breakdown: ::core::option::Option<FunnelBreakdown>,
    /// Optional. If specified, next action adds a dimension to the funnel
    /// visualization sub report response. This next action dimension expands each
    /// funnel step to the unique values of the next action. For example a next
    /// action of the `eventName` dimension will create rows for several events
    /// (for example `session_start` & `click`) and the total.
    ///
    /// Next action only supports `eventName` and most Page / Screen dimensions
    /// like `pageTitle` and `pagePath`.
    #[prost(message, optional, tag = "5")]
    pub funnel_next_action: ::core::option::Option<FunnelNextAction>,
    /// Optional. The funnel visualization type controls the dimensions present in
    /// the funnel visualization sub report response. If not specified,
    /// `STANDARD_FUNNEL` is used.
    #[prost(
        enumeration = "run_funnel_report_request::FunnelVisualizationType",
        tag = "6"
    )]
    pub funnel_visualization_type: i32,
    /// Optional. The configurations of segments. Segments are subsets of a
    /// property's data. In a funnel report with segments, the funnel is evaluated
    /// in each segment.
    ///
    /// Each segment specified in this request
    /// produces a separate row in the response; in the response, each segment
    /// identified by its name.
    ///
    /// The segments parameter is optional. Requests are limited to 4 segments.
    #[prost(message, repeated, tag = "7")]
    pub segments: ::prost::alloc::vec::Vec<Segment>,
    /// Optional. The number of rows to return. If unspecified, 10,000 rows are
    /// returned. The API returns a maximum of 250,000 rows per request, no matter
    /// how many you ask for. `limit` must be positive.
    ///
    /// The API can also return fewer rows than the requested `limit`, if there
    /// aren't as many dimension values as the `limit`.
    #[prost(int64, tag = "9")]
    pub limit: i64,
    /// Optional. Dimension filters allow you to ask for only specific dimension
    /// values in the report. To learn more, see [Creating a Report: Dimension
    /// Filters](<https://developers.google.com/analytics/devguides/reporting/data/v1/basics#dimension_filters>)
    /// for examples. Metrics cannot be used in this filter.
    #[prost(message, optional, tag = "10")]
    pub dimension_filter: ::core::option::Option<FilterExpression>,
    /// Optional. Toggles whether to return the current state of this Analytics
    /// Property's quota. Quota is returned in [PropertyQuota](#PropertyQuota).
    #[prost(bool, tag = "12")]
    pub return_property_quota: bool,
}
/// Nested message and enum types in `RunFunnelReportRequest`.
pub mod run_funnel_report_request {
    /// Controls the dimensions present in the funnel visualization sub report
    /// response.
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum FunnelVisualizationType {
        /// Unspecified type.
        Unspecified = 0,
        /// A standard (stepped) funnel. The funnel visualization sub report in the
        /// response will not contain date.
        StandardFunnel = 1,
        /// A trended (line chart) funnel. The funnel visualization sub report in the
        /// response will contain the date dimension.
        TrendedFunnel = 2,
    }
    impl FunnelVisualizationType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                FunnelVisualizationType::Unspecified => {
                    "FUNNEL_VISUALIZATION_TYPE_UNSPECIFIED"
                }
                FunnelVisualizationType::StandardFunnel => "STANDARD_FUNNEL",
                FunnelVisualizationType::TrendedFunnel => "TRENDED_FUNNEL",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "FUNNEL_VISUALIZATION_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "STANDARD_FUNNEL" => Some(Self::StandardFunnel),
                "TRENDED_FUNNEL" => Some(Self::TrendedFunnel),
                _ => None,
            }
        }
    }
}
/// The funnel report response contains two sub reports. The two sub reports are
/// different combinations of dimensions and metrics.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RunFunnelReportResponse {
    /// The funnel table is a report with the funnel step, segment, breakdown
    /// dimension, active users, completion rate, abandonments, and abandonments
    /// rate.
    ///
    /// The segment dimension is only present in this response if a segment was
    /// requested. The breakdown dimension is only present in this response if it
    /// was requested.
    #[prost(message, optional, tag = "1")]
    pub funnel_table: ::core::option::Option<FunnelSubReport>,
    /// The funnel visualization is a report with the funnel step, segment, date,
    /// next action dimension, and active users.
    ///
    /// The segment dimension is only present in this response if a segment was
    /// requested. The date dimension is only present in this response if it was
    /// requested through the `TRENDED_FUNNEL` funnel type. The next action
    /// dimension is only present in the response if it was requested.
    #[prost(message, optional, tag = "2")]
    pub funnel_visualization: ::core::option::Option<FunnelSubReport>,
    /// This Analytics Property's quota state including this request.
    #[prost(message, optional, tag = "3")]
    pub property_quota: ::core::option::Option<PropertyQuota>,
    /// Identifies what kind of resource this message is. This `kind` is always the
    /// fixed string "analyticsData#runFunnelReport". Useful to distinguish between
    /// response types in JSON.
    #[prost(string, tag = "4")]
    pub kind: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod alpha_analytics_data_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Google Analytics reporting data service.
    #[derive(Debug, Clone)]
    pub struct AlphaAnalyticsDataClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl AlphaAnalyticsDataClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> AlphaAnalyticsDataClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> AlphaAnalyticsDataClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            AlphaAnalyticsDataClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        /// Returns a customized funnel report of your Google Analytics event data. The
        /// data returned from the API is as a table with columns for the requested
        /// dimensions and metrics.
        ///
        /// Funnel exploration lets you visualize the steps your users take to complete
        /// a task and quickly see how well they are succeeding or failing at each
        /// step. For example, how do prospects become shoppers and then become buyers?
        /// How do one time buyers become repeat buyers? With this information, you can
        /// improve inefficient or abandoned customer journeys. To learn more, see [GA4
        /// Funnel Explorations](https://support.google.com/analytics/answer/9327974).
        ///
        /// This method is introduced at alpha stability with the intention of
        /// gathering feedback on syntax and capabilities before entering beta. To give
        /// your feedback on this API, complete the [Google Analytics Data API Funnel
        /// Reporting
        /// Feedback](https://docs.google.com/forms/d/e/1FAIpQLSdwOlQDJAUoBiIgUZZ3S_Lwi8gr7Bb0k1jhvc-DEg7Rol3UjA/viewform).
        pub async fn run_funnel_report(
            &mut self,
            request: impl tonic::IntoRequest<super::RunFunnelReportRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RunFunnelReportResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.data.v1alpha.AlphaAnalyticsData/RunFunnelReport",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.analytics.data.v1alpha.AlphaAnalyticsData",
                        "RunFunnelReport",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates an audience list for later retrieval. This method quickly returns
        /// the audience list's resource name and initiates a long running asynchronous
        /// request to form an audience list. To list the users in an audience list,
        /// first create the audience list through this method and then send the
        /// audience resource name to the `QueryAudienceList` method.
        ///
        /// See [Creating an Audience
        /// List](https://developers.google.com/analytics/devguides/reporting/data/v1/audience-list-basics)
        /// for an introduction to Audience Lists with examples.
        ///
        /// An audience list is a snapshot of the users currently in the audience at
        /// the time of audience list creation. Creating audience lists for one
        /// audience on different days will return different results as users enter and
        /// exit the audience.
        ///
        /// Audiences in Google Analytics 4 allow you to segment your users in the ways
        /// that are important to your business. To learn more, see
        /// https://support.google.com/analytics/answer/9267572. Audience lists contain
        /// the users in each audience.
        ///
        /// This method is introduced at alpha stability with the intention of
        /// gathering feedback on syntax and capabilities before entering beta. To give
        /// your feedback on this API, complete the
        /// [Google Analytics Audience Export API
        /// Feedback](https://forms.gle/EeA5u5LW6PEggtCEA) form.
        pub async fn create_audience_list(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateAudienceListRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.data.v1alpha.AlphaAnalyticsData/CreateAudienceList",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.analytics.data.v1alpha.AlphaAnalyticsData",
                        "CreateAudienceList",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Retrieves an audience list of users. After creating an audience, the users
        /// are not immediately available for listing. First, a request to
        /// `CreateAudienceList` is necessary to create an audience list of users, and
        /// then second, this method is used to retrieve the users in the audience
        /// list.
        ///
        /// See [Creating an Audience
        /// List](https://developers.google.com/analytics/devguides/reporting/data/v1/audience-list-basics)
        /// for an introduction to Audience Lists with examples.
        ///
        /// Audiences in Google Analytics 4 allow you to segment your users in the ways
        /// that are important to your business. To learn more, see
        /// https://support.google.com/analytics/answer/9267572.
        ///
        /// This method is introduced at alpha stability with the intention of
        /// gathering feedback on syntax and capabilities before entering beta. To give
        /// your feedback on this API, complete the
        /// [Google Analytics Audience Export API
        /// Feedback](https://forms.gle/EeA5u5LW6PEggtCEA) form.
        pub async fn query_audience_list(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryAudienceListRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryAudienceListResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.data.v1alpha.AlphaAnalyticsData/QueryAudienceList",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.analytics.data.v1alpha.AlphaAnalyticsData",
                        "QueryAudienceList",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Exports an audience list of users to a Google Sheet. After creating an
        /// audience, the users are not immediately available for listing. First, a
        /// request to `CreateAudienceList` is necessary to create an audience list of
        /// users, and then second, this method is used to export those users in the
        /// audience list to a Google Sheet.
        ///
        /// See [Creating an Audience
        /// List](https://developers.google.com/analytics/devguides/reporting/data/v1/audience-list-basics)
        /// for an introduction to Audience Lists with examples.
        ///
        /// Audiences in Google Analytics 4 allow you to segment your users in the ways
        /// that are important to your business. To learn more, see
        /// https://support.google.com/analytics/answer/9267572.
        ///
        /// This method is introduced at alpha stability with the intention of
        /// gathering feedback on syntax and capabilities before entering beta. To give
        /// your feedback on this API, complete the
        /// [Google Analytics Audience Export API
        /// Feedback](https://forms.gle/EeA5u5LW6PEggtCEA) form.
        pub async fn sheet_export_audience_list(
            &mut self,
            request: impl tonic::IntoRequest<super::SheetExportAudienceListRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SheetExportAudienceListResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.data.v1alpha.AlphaAnalyticsData/SheetExportAudienceList",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.analytics.data.v1alpha.AlphaAnalyticsData",
                        "SheetExportAudienceList",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets configuration metadata about a specific audience list. This method
        /// can be used to understand an audience list after it has been created.
        ///
        /// See [Creating an Audience
        /// List](https://developers.google.com/analytics/devguides/reporting/data/v1/audience-list-basics)
        /// for an introduction to Audience Lists with examples.
        ///
        /// This method is introduced at alpha stability with the intention of
        /// gathering feedback on syntax and capabilities before entering beta. To give
        /// your feedback on this API, complete the
        /// [Google Analytics Audience Export API
        /// Feedback](https://forms.gle/EeA5u5LW6PEggtCEA) form.
        pub async fn get_audience_list(
            &mut self,
            request: impl tonic::IntoRequest<super::GetAudienceListRequest>,
        ) -> std::result::Result<tonic::Response<super::AudienceList>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.data.v1alpha.AlphaAnalyticsData/GetAudienceList",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.analytics.data.v1alpha.AlphaAnalyticsData",
                        "GetAudienceList",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists all audience lists for a property. This method can be used for you to
        /// find and reuse existing audience lists rather than creating unnecessary new
        /// audience lists. The same audience can have multiple audience lists that
        /// represent the list of users that were in an audience on different days.
        ///
        /// See [Creating an Audience
        /// List](https://developers.google.com/analytics/devguides/reporting/data/v1/audience-list-basics)
        /// for an introduction to Audience Lists with examples.
        ///
        /// This method is introduced at alpha stability with the intention of
        /// gathering feedback on syntax and capabilities before entering beta. To give
        /// your feedback on this API, complete the
        /// [Google Analytics Audience Export API
        /// Feedback](https://forms.gle/EeA5u5LW6PEggtCEA) form.
        pub async fn list_audience_lists(
            &mut self,
            request: impl tonic::IntoRequest<super::ListAudienceListsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListAudienceListsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.data.v1alpha.AlphaAnalyticsData/ListAudienceLists",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.analytics.data.v1alpha.AlphaAnalyticsData",
                        "ListAudienceLists",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a recurring audience list. Recurring audience lists produces new
        /// audience lists each day. Audience lists are users in an audience at the
        /// time of the list's creation.
        ///
        /// A recurring audience list ensures that you have audience list based on the
        /// most recent data available for use each day. If you manually create
        /// audience list, you don't know when an audience list based on an additional
        /// day's data is available. This recurring audience list automates the
        /// creation of an audience list when an additional day's data is available.
        /// You will consume fewer quota tokens by using recurring audience list versus
        /// manually creating audience list at various times of day trying to guess
        /// when an additional day's data is ready.
        ///
        /// This method is introduced at alpha stability with the intention of
        /// gathering feedback on syntax and capabilities before entering beta. To give
        /// your feedback on this API, complete the
        /// [Google Analytics Audience Export API
        /// Feedback](https://forms.gle/EeA5u5LW6PEggtCEA) form.
        pub async fn create_recurring_audience_list(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateRecurringAudienceListRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RecurringAudienceList>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.data.v1alpha.AlphaAnalyticsData/CreateRecurringAudienceList",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.analytics.data.v1alpha.AlphaAnalyticsData",
                        "CreateRecurringAudienceList",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Gets configuration metadata about a specific recurring audience list. This
        /// method can be used to understand a recurring audience list's state after it
        /// has been created. For example, a recurring audience list resource will
        /// generate audience list instances for each day, and this method can be used
        /// to get the resource name of the most recent audience list instance.
        ///
        /// This method is introduced at alpha stability with the intention of
        /// gathering feedback on syntax and capabilities before entering beta. To give
        /// your feedback on this API, complete the
        /// [Google Analytics Audience Export API
        /// Feedback](https://forms.gle/EeA5u5LW6PEggtCEA) form.
        pub async fn get_recurring_audience_list(
            &mut self,
            request: impl tonic::IntoRequest<super::GetRecurringAudienceListRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RecurringAudienceList>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.data.v1alpha.AlphaAnalyticsData/GetRecurringAudienceList",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.analytics.data.v1alpha.AlphaAnalyticsData",
                        "GetRecurringAudienceList",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Lists all recurring audience lists for a property. This method can be used
        /// for you to find and reuse existing recurring audience lists rather than
        /// creating unnecessary new recurring audience lists. The same audience can
        /// have multiple recurring audience lists that represent different dimension
        /// combinations; for example, just the dimension `deviceId` or both the
        /// dimensions `deviceId` and `userId`.
        ///
        /// This method is introduced at alpha stability with the intention of
        /// gathering feedback on syntax and capabilities before entering beta. To give
        /// your feedback on this API, complete the
        /// [Google Analytics Audience Export API
        /// Feedback](https://forms.gle/EeA5u5LW6PEggtCEA) form.
        pub async fn list_recurring_audience_lists(
            &mut self,
            request: impl tonic::IntoRequest<super::ListRecurringAudienceListsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListRecurringAudienceListsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.data.v1alpha.AlphaAnalyticsData/ListRecurringAudienceLists",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.analytics.data.v1alpha.AlphaAnalyticsData",
                        "ListRecurringAudienceLists",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
