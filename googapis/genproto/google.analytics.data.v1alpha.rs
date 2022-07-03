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
/// Dimensions are attributes of your data. For example, the dimension city
/// indicates the city from which an event originates. Dimension values in report
/// responses are strings; for example, the city could be "Paris" or "New York".
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
/// To express dimension or metric filters. The fields in the same
/// FilterExpression need to be either all dimensions or all metrics.
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
        /// A primitive filter. In the same FilterExpression, all of the filter's
        /// field names need to be either all dimensions or all metrics.
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
    #[prost(oneof = "filter::OneFilter", tags = "2, 3, 4, 5")]
    pub one_filter: ::core::option::Option<filter::OneFilter>,
}
/// Nested message and enum types in `Filter`.
pub mod filter {
    /// Specify one type of filter for `Filter`.
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
        /// Full match for the regular expression with the string value.
        FullRegexp = 5,
        /// Partial match for the regular expression with the string value.
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
    pub value: ::core::option::Option<NumericValue>,
}
/// Nested message and enum types in `NumericFilter`.
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
    pub from_value: ::core::option::Option<NumericValue>,
    /// Ends with this number.
    #[prost(message, optional, tag = "2")]
    pub to_value: ::core::option::Option<NumericValue>,
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
/// Breakdowns add a dimension to the funnel table sub report response.
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
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FunnelStep {
    /// The distinctive name for this step. If unspecified, steps will be named
    /// by a 1 based indexed name (i.e. "0. ", "1. ", etc.). This name defines
    /// string value returned by the `funnelStepName` dimension. For example,
    /// specifying `name = Purchase` in the request's third funnel step will
    /// produce `3. Purchase` in the funnel report response.
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
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SessionSegmentCriteria {
    /// A session matches this criteria if the session matches each of these
    /// `andConditionGroups`.
    #[prost(message, repeated, tag = "1")]
    pub and_condition_groups: ::prost::alloc::vec::Vec<SessionSegmentConditionGroup>,
}
/// Conditions tell Analytics what data to include in or exclude from the
/// segment.
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
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventSegmentCriteria {
    /// An event matches this criteria if the event matches each of these
    /// `andConditionGroups`.
    #[prost(message, repeated, tag = "1")]
    pub and_condition_groups: ::prost::alloc::vec::Vec<EventSegmentConditionGroup>,
}
/// Conditions tell Analytics what data to include in or exclude from the
/// segment.
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
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Segment {
    /// The name for this segment. If unspecified, segments are named "Segment".
    /// This name defines string value returned by the `segment` dimension. The
    /// `segment` dimension prefixes segment names by the 1-based index number of
    /// the segment in the request (i.e. "1. Segment", "2. Segment", etc.).
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// A segment is specified in one scope.
    #[prost(oneof = "segment::OneSegmentScope", tags = "2, 3, 4")]
    pub one_segment_scope: ::core::option::Option<segment::OneSegmentScope>,
}
/// Nested message and enum types in `Segment`.
pub mod segment {
    /// A segment is specified in one scope.
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
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SegmentFilterExpression {
    /// Specify one type of filter for `SegmentFilterExpression`.
    #[prost(oneof = "segment_filter_expression::Expr", tags = "1, 2, 3, 4, 5")]
    pub expr: ::core::option::Option<segment_filter_expression::Expr>,
}
/// Nested message and enum types in `SegmentFilterExpression`.
pub mod segment_filter_expression {
    /// Specify one type of filter for `SegmentFilterExpression`.
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
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SegmentFilterExpressionList {
    /// The list of segment filter expressions
    #[prost(message, repeated, tag = "1")]
    pub expressions: ::prost::alloc::vec::Vec<SegmentFilterExpression>,
}
/// An expression to filter dimension or metric values.
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
    pub segment_parameter_filter_expression:
        ::core::option::Option<SegmentParameterFilterExpression>,
}
/// Expresses combinations of segment filter on parameters.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SegmentParameterFilterExpression {
    /// Specify one type of filter for `SegmentParameterFilterExpression`.
    #[prost(
        oneof = "segment_parameter_filter_expression::Expr",
        tags = "1, 2, 3, 4"
    )]
    pub expr: ::core::option::Option<segment_parameter_filter_expression::Expr>,
}
/// Nested message and enum types in `SegmentParameterFilterExpression`.
pub mod segment_parameter_filter_expression {
    /// Specify one type of filter for `SegmentParameterFilterExpression`.
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
        NotExpression(::prost::alloc::boxed::Box<super::SegmentParameterFilterExpression>),
        /// A primitive segment parameter filter.
        #[prost(message, tag = "4")]
        SegmentParameterFilter(super::SegmentParameterFilter),
    }
}
/// A list of segment parameter filter expressions.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SegmentParameterFilterExpressionList {
    /// The list of segment parameter filter expressions.
    #[prost(message, repeated, tag = "1")]
    pub expressions: ::prost::alloc::vec::Vec<SegmentParameterFilterExpression>,
}
/// An expression to filter parameter values in a segment.
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
        /// about ecommerce events, see the [Measure ecommerce]
        /// (<https://developers.google.com/analytics/devguides/collection/ga4/ecommerce>)
        /// guide.
        #[prost(string, tag = "2")]
        ItemParameterName(::prost::alloc::string::String),
    }
    /// Specify one type of filter.
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
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FunnelFilterExpression {
    /// Specify one type of filter for `FunnelFilterExpression`.
    #[prost(oneof = "funnel_filter_expression::Expr", tags = "1, 2, 3, 4, 5")]
    pub expr: ::core::option::Option<funnel_filter_expression::Expr>,
}
/// Nested message and enum types in `FunnelFilterExpression`.
pub mod funnel_filter_expression {
    /// Specify one type of filter for `FunnelFilterExpression`.
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
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FunnelFilterExpressionList {
    /// The list of funnel filter expressions.
    #[prost(message, repeated, tag = "1")]
    pub expressions: ::prost::alloc::vec::Vec<FunnelFilterExpression>,
}
/// An expression to filter dimension or metric values.
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
    pub funnel_parameter_filter_expression: ::core::option::Option<FunnelParameterFilterExpression>,
}
/// Expresses combinations of funnel filters on parameters.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FunnelParameterFilterExpression {
    /// Specify one type of filter for `FunnelParameterFilterExpression`.
    #[prost(
        oneof = "funnel_parameter_filter_expression::Expr",
        tags = "1, 2, 3, 4"
    )]
    pub expr: ::core::option::Option<funnel_parameter_filter_expression::Expr>,
}
/// Nested message and enum types in `FunnelParameterFilterExpression`.
pub mod funnel_parameter_filter_expression {
    /// Specify one type of filter for `FunnelParameterFilterExpression`.
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
        NotExpression(::prost::alloc::boxed::Box<super::FunnelParameterFilterExpression>),
        /// A primitive funnel parameter filter.
        #[prost(message, tag = "4")]
        FunnelParameterFilter(super::FunnelParameterFilter),
    }
}
/// A list of funnel parameter filter expressions.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FunnelParameterFilterExpressionList {
    /// The list of funnel parameter filter expressions.
    #[prost(message, repeated, tag = "1")]
    pub expressions: ::prost::alloc::vec::Vec<FunnelParameterFilterExpression>,
}
/// An expression to filter parameter values in a funnel.
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
        /// about ecommerce events, see the [Measure ecommerce]
        /// (<https://developers.google.com/analytics/devguides/collection/ga4/ecommerce>)
        /// guide.
        #[prost(string, tag = "2")]
        ItemParameterName(::prost::alloc::string::String),
    }
    /// Specify one type of filter.
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
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FunnelResponseMetadata {
    /// If funnel report results are
    /// \[sampled\](<https://support.google.com/analytics/answer/2637192>), this
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
/// \[sampled\](<https://support.google.com/analytics/answer/2637192>), this
/// metadata describes what percentage of events were used in this funnel
/// report for a date range. Sampling is the practice of analyzing a subset of
/// all data in order to uncover the meaningful information in the larger data
/// set.
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
/// The request for a funnel report.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RunFunnelReportRequest {
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
    /// Date ranges of data to read. If multiple date ranges are requested, each
    /// response row will contain a zero based date range index. If two date
    /// ranges overlap, the event data for the overlapping days is included in the
    /// response rows for both date ranges.
    #[prost(message, repeated, tag = "2")]
    pub date_ranges: ::prost::alloc::vec::Vec<DateRange>,
    /// The configuration of this request's funnel. This funnel configuration is
    /// required.
    #[prost(message, optional, tag = "3")]
    pub funnel: ::core::option::Option<Funnel>,
    /// If specified, this breakdown adds a dimension to the funnel table sub
    /// report response. This breakdown dimension expands each funnel step to the
    /// unique values of the breakdown dimension. For example, a breakdown by the
    /// `deviceCategory` dimension will create rows for `mobile`, `tablet`,
    /// `desktop`, and the total.
    #[prost(message, optional, tag = "4")]
    pub funnel_breakdown: ::core::option::Option<FunnelBreakdown>,
    /// If specified, next action adds a dimension to the funnel visualization sub
    /// report response. This next action dimension expands each funnel step to the
    /// unique values of the next action. For example a next action of the
    /// `eventName` dimension will create rows for several events (i.e.
    /// `session_start` & `click`) and the total.
    ///
    /// Next action only supports `eventName` and most Page / Screen dimensions
    /// like `pageTitle` and `pagePath`.
    #[prost(message, optional, tag = "5")]
    pub funnel_next_action: ::core::option::Option<FunnelNextAction>,
    /// The funnel visualization type controls the dimensions present in the funnel
    /// visualization sub report response. If not specified, `STANDARD_FUNNEL` is
    /// used.
    #[prost(
        enumeration = "run_funnel_report_request::FunnelVisualizationType",
        tag = "6"
    )]
    pub funnel_visualization_type: i32,
    /// The configurations of segments. Segments are subsets of a property's data.
    /// In a funnel report with segments, the funnel is evaluated in each segment.
    ///
    /// Each segment specified in this request
    /// produces a separate row in the response; in the response, each segment
    /// identified by its name.
    ///
    /// The segments parameter is optional. Requests are limited to 4 segments.
    #[prost(message, repeated, tag = "7")]
    pub segments: ::prost::alloc::vec::Vec<Segment>,
    /// The number of rows to return. If unspecified, 10,000 rows are returned. The
    /// API returns a maximum of 100,000 rows per request, no matter how many you
    /// ask for. `limit` must be positive.
    ///
    /// The API can also return fewer rows than the requested `limit`, if there
    /// aren't as many dimension values as the `limit`.
    #[prost(int64, tag = "9")]
    pub limit: i64,
    /// Dimension filters allow you to ask for only specific dimension values in
    /// the report. To learn more, see [Creating a Report: Dimension
    /// Filters](<https://developers.google.com/analytics/devguides/reporting/data/v1/basics#dimension_filters>)
    /// for examples. Metrics cannot be used in this filter.
    #[prost(message, optional, tag = "10")]
    pub dimension_filter: ::core::option::Option<FilterExpression>,
    /// Toggles whether to return the current state of this Analytics Property's
    /// quota. Quota is returned in \[PropertyQuota\](#PropertyQuota).
    #[prost(bool, tag = "12")]
    pub return_property_quota: bool,
}
/// Nested message and enum types in `RunFunnelReportRequest`.
pub mod run_funnel_report_request {
    /// Controls the dimensions present in the funnel visualization sub report
    /// response.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
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
}
/// The funnel report response contains two sub reports. The two sub reports are
/// different combinations of dimensions and metrics.
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
    /// requested via the `TRENDED_FUNNEL` funnel type. The next action dimension
    /// is only present in the response if it was requested.
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
#[doc = r" Generated client implementations."]
pub mod alpha_analytics_data_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Google Analytics reporting data service."]
    #[derive(Debug, Clone)]
    pub struct AlphaAnalyticsDataClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> AlphaAnalyticsDataClient<T>
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
        ) -> AlphaAnalyticsDataClient<InterceptedService<T, F>>
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
            AlphaAnalyticsDataClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Returns a customized funnel report of your Google Analytics event data. The"]
        #[doc = " data returned from the API is as a table with columns for the requested"]
        #[doc = " dimensions and metrics."]
        #[doc = ""]
        #[doc = " Funnel exploration lets you visualize the steps your users take to complete"]
        #[doc = " a task and quickly see how well they are succeeding or failing at each"]
        #[doc = " step. For example, how do prospects become shoppers and then become buyers?"]
        #[doc = " How do one time buyers become repeat buyers? With this information, you can"]
        #[doc = " improve inefficient or abandoned customer journeys. To learn more, see [GA4"]
        #[doc = " Funnel Explorations](https://support.google.com/analytics/answer/9327974)."]
        pub async fn run_funnel_report(
            &mut self,
            request: impl tonic::IntoRequest<super::RunFunnelReportRequest>,
        ) -> Result<tonic::Response<super::RunFunnelReportResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.analytics.data.v1alpha.AlphaAnalyticsData/RunFunnelReport",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
