/// An automated bidding strategy that raises bids for clicks
/// that seem more likely to lead to a conversion and lowers
/// them for clicks where they seem less likely.
///
/// This bidding strategy is deprecated and cannot be created anymore. Use
/// ManualCpc with enhanced_cpc_enabled set to true for equivalent functionality.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EnhancedCpc {}
/// Manual bidding strategy that allows advertiser to set the bid per
/// advertiser-specified action.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ManualCpa {}
/// Manual click-based bidding where user pays per click.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ManualCpc {
    /// Whether bids are to be enhanced based on conversion optimizer data.
    #[prost(bool, optional, tag = "2")]
    pub enhanced_cpc_enabled: ::core::option::Option<bool>,
}
/// Manual impression-based bidding where user pays per thousand impressions.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ManualCpm {}
/// An automated bidding strategy to help get the most conversions for your
/// campaigns while spending your budget.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MaximizeConversions {
    /// Maximum bid limit that can be set by the bid strategy.
    /// The limit applies to all keywords managed by the strategy.
    /// Mutable for portfolio bidding strategies only.
    #[prost(int64, tag = "2")]
    pub cpc_bid_ceiling_micros: i64,
    /// Minimum bid limit that can be set by the bid strategy.
    /// The limit applies to all keywords managed by the strategy.
    /// Mutable for portfolio bidding strategies only.
    #[prost(int64, tag = "3")]
    pub cpc_bid_floor_micros: i64,
    /// The target cost-per-action (CPA) option. This is the average amount that
    /// you would like to spend per conversion action specified in micro units of
    /// the bidding strategy's currency. If set, the bid strategy will get as many
    /// conversions as possible at or below the target cost-per-action. If the
    /// target CPA is not set, the bid strategy will aim to achieve the lowest
    /// possible CPA given the budget.
    #[prost(int64, tag = "4")]
    pub target_cpa_micros: i64,
}
/// An automated bidding strategy to help get the most conversion value for your
/// campaigns while spending your budget.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MaximizeConversionValue {
    /// The target return on ad spend (ROAS) option. If set, the bid strategy will
    /// maximize revenue while averaging the target return on ad spend. If the
    /// target ROAS is high, the bid strategy may not be able to spend the full
    /// budget. If the target ROAS is not set, the bid strategy will aim to
    /// achieve the highest possible ROAS for the budget.
    #[prost(double, optional, tag = "2")]
    pub target_roas: ::core::option::Option<f64>,
    /// Maximum bid limit that can be set by the bid strategy.
    /// The limit applies to all keywords managed by the strategy.
    /// Mutable for portfolio bidding strategies only.
    #[prost(int64, tag = "3")]
    pub cpc_bid_ceiling_micros: i64,
    /// Minimum bid limit that can be set by the bid strategy.
    /// The limit applies to all keywords managed by the strategy.
    /// Mutable for portfolio bidding strategies only.
    #[prost(int64, tag = "4")]
    pub cpc_bid_floor_micros: i64,
}
/// An automated bid strategy that sets bids to help get as many conversions as
/// possible at the target cost-per-acquisition (CPA) you set.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TargetCpa {
    /// Average CPA target.
    /// This target should be greater than or equal to minimum billable unit based
    /// on the currency for the account.
    #[prost(int64, optional, tag = "4")]
    pub target_cpa_micros: ::core::option::Option<i64>,
    /// Maximum bid limit that can be set by the bid strategy.
    /// The limit applies to all keywords managed by the strategy.
    /// This should only be set for portfolio bid strategies.
    #[prost(int64, optional, tag = "5")]
    pub cpc_bid_ceiling_micros: ::core::option::Option<i64>,
    /// Minimum bid limit that can be set by the bid strategy.
    /// The limit applies to all keywords managed by the strategy.
    /// This should only be set for portfolio bid strategies.
    #[prost(int64, optional, tag = "6")]
    pub cpc_bid_floor_micros: ::core::option::Option<i64>,
}
/// Target CPM (cost per thousand impressions) is an automated bidding strategy
/// that sets bids to optimize performance given the target CPM you set.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TargetCpm {}
/// An automated bidding strategy that sets bids so that a certain percentage of
/// search ads are shown at the top of the first page (or other targeted
/// location).
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TargetImpressionShare {
    /// The targeted location on the search results page.
    #[prost(
        enumeration = "super::enums::target_impression_share_location_enum::TargetImpressionShareLocation",
        tag = "1"
    )]
    pub location: i32,
    /// The chosen fraction of ads to be shown in the targeted location in micros.
    /// For example, 1% equals 10,000.
    #[prost(int64, optional, tag = "4")]
    pub location_fraction_micros: ::core::option::Option<i64>,
    /// The highest CPC bid the automated bidding system is permitted to specify.
    /// This is a required field entered by the advertiser that sets the ceiling
    /// and specified in local micros.
    #[prost(int64, optional, tag = "5")]
    pub cpc_bid_ceiling_micros: ::core::option::Option<i64>,
}
/// An automated bidding strategy that sets bids based on the target fraction of
/// auctions where the advertiser should outrank a specific competitor.
/// This strategy is deprecated.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TargetOutrankShare {
    /// Maximum bid limit that can be set by the bid strategy.
    /// The limit applies to all keywords managed by the strategy.
    #[prost(message, optional, tag = "3")]
    pub cpc_bid_ceiling_micros: ::core::option::Option<i64>,
}
/// An automated bidding strategy that helps you maximize revenue while
/// averaging a specific target return on ad spend (ROAS).
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TargetRoas {
    /// Required. The chosen revenue (based on conversion data) per unit of spend.
    /// Value must be between 0.01 and 1000.0, inclusive.
    #[prost(double, optional, tag = "4")]
    pub target_roas: ::core::option::Option<f64>,
    /// Maximum bid limit that can be set by the bid strategy.
    /// The limit applies to all keywords managed by the strategy.
    /// This should only be set for portfolio bid strategies.
    #[prost(int64, optional, tag = "5")]
    pub cpc_bid_ceiling_micros: ::core::option::Option<i64>,
    /// Minimum bid limit that can be set by the bid strategy.
    /// The limit applies to all keywords managed by the strategy.
    /// This should only be set for portfolio bid strategies.
    #[prost(int64, optional, tag = "6")]
    pub cpc_bid_floor_micros: ::core::option::Option<i64>,
}
/// An automated bid strategy that sets your bids to help get as many clicks
/// as possible within your budget.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TargetSpend {
    /// The spend target under which to maximize clicks.
    /// A TargetSpend bidder will attempt to spend the smaller of this value
    /// or the natural throttling spend amount.
    /// If not specified, the budget is used as the spend target.
    /// This field is deprecated and should no longer be used. See
    /// <https://ads-developers.googleblog.com/2020/05/reminder-about-sunset-creation-of.html>
    /// for details.
    #[deprecated]
    #[prost(int64, optional, tag = "3")]
    pub target_spend_micros: ::core::option::Option<i64>,
    /// Maximum bid limit that can be set by the bid strategy.
    /// The limit applies to all keywords managed by the strategy.
    #[prost(int64, optional, tag = "4")]
    pub cpc_bid_ceiling_micros: ::core::option::Option<i64>,
}
/// A bidding strategy where bids are a fraction of the advertised price for
/// some good or service.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PercentCpc {
    /// Maximum bid limit that can be set by the bid strategy. This is
    /// an optional field entered by the advertiser and specified in local micros.
    /// Note: A zero value is interpreted in the same way as having bid_ceiling
    /// undefined.
    #[prost(int64, optional, tag = "3")]
    pub cpc_bid_ceiling_micros: ::core::option::Option<i64>,
    /// Adjusts the bid for each auction upward or downward, depending on the
    /// likelihood of a conversion. Individual bids may exceed
    /// cpc_bid_ceiling_micros, but the average bid amount for a campaign should
    /// not.
    #[prost(bool, optional, tag = "4")]
    pub enhanced_cpc_enabled: ::core::option::Option<bool>,
}
/// A keyword criterion.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeywordInfo {
    /// The text of the keyword (at most 80 characters and 10 words).
    #[prost(string, optional, tag = "3")]
    pub text: ::core::option::Option<::prost::alloc::string::String>,
    /// The match type of the keyword.
    #[prost(
        enumeration = "super::enums::keyword_match_type_enum::KeywordMatchType",
        tag = "2"
    )]
    pub match_type: i32,
}
/// A location criterion.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LocationInfo {
    /// The geo target constant resource name.
    #[prost(string, optional, tag = "2")]
    pub geo_target_constant: ::core::option::Option<::prost::alloc::string::String>,
}
/// A device criterion.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeviceInfo {
    /// Type of the device.
    #[prost(enumeration = "super::enums::device_enum::Device", tag = "1")]
    pub r#type: i32,
}
/// A listing group criterion.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListingGroupInfo {
    /// Type of the listing group.
    #[prost(
        enumeration = "super::enums::listing_group_type_enum::ListingGroupType",
        tag = "1"
    )]
    pub r#type: i32,
}
/// An age range criterion.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AgeRangeInfo {
    /// Type of the age range.
    #[prost(enumeration = "super::enums::age_range_type_enum::AgeRangeType", tag = "1")]
    pub r#type: i32,
}
/// A gender criterion.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenderInfo {
    /// Type of the gender.
    #[prost(enumeration = "super::enums::gender_type_enum::GenderType", tag = "1")]
    pub r#type: i32,
}
/// A User List criterion. Represents a user list that is defined by the
/// advertiser to be targeted.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserListInfo {
    /// The User List resource name.
    #[prost(string, optional, tag = "2")]
    pub user_list: ::core::option::Option<::prost::alloc::string::String>,
}
/// A language criterion.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LanguageInfo {
    /// The language constant resource name.
    #[prost(string, optional, tag = "2")]
    pub language_constant: ::core::option::Option<::prost::alloc::string::String>,
}
/// Represents a criterion for targeting webpages of an advertiser's website.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WebpageInfo {
    /// The name of the criterion that is defined by this parameter. The name value
    /// will be used for identifying, sorting and filtering criteria with this type
    /// of parameters.
    ///
    /// This field is required for CREATE operations and is prohibited on UPDATE
    /// operations.
    #[prost(string, optional, tag = "3")]
    pub criterion_name: ::core::option::Option<::prost::alloc::string::String>,
    /// Conditions, or logical expressions, for webpage targeting. The list of
    /// webpage targeting conditions are and-ed together when evaluated
    /// for targeting. An empty list of conditions indicates all pages of the
    /// campaign's website are targeted.
    ///
    /// This field is required for CREATE operations and is prohibited on UPDATE
    /// operations.
    #[prost(message, repeated, tag = "2")]
    pub conditions: ::prost::alloc::vec::Vec<WebpageConditionInfo>,
    /// Website criteria coverage percentage. This is the computed percentage
    /// of website coverage based on the website target, negative website target
    /// and negative keywords in the ad group and campaign. For instance, when
    /// coverage returns as 1, it indicates it has 100% coverage. This field is
    /// read-only.
    #[prost(double, tag = "4")]
    pub coverage_percentage: f64,
}
/// Logical expression for targeting webpages of an advertiser's website.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WebpageConditionInfo {
    /// Operand of webpage targeting condition.
    #[prost(
        enumeration = "super::enums::webpage_condition_operand_enum::WebpageConditionOperand",
        tag = "1"
    )]
    pub operand: i32,
    /// Operator of webpage targeting condition.
    #[prost(
        enumeration = "super::enums::webpage_condition_operator_enum::WebpageConditionOperator",
        tag = "2"
    )]
    pub operator: i32,
    /// Argument of webpage targeting condition.
    #[prost(string, optional, tag = "4")]
    pub argument: ::core::option::Option<::prost::alloc::string::String>,
}
/// A radius around a list of locations specified through a feed.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LocationGroupInfo {
    /// Geo target constant(s) restricting the scope of the geographic area within
    /// the feed. Currently only one geo target constant is allowed.
    #[prost(string, repeated, tag = "6")]
    pub geo_target_constants: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Distance in units specifying the radius around targeted locations.
    /// This is required and must be set in CREATE operations.
    #[prost(int64, optional, tag = "7")]
    pub radius: ::core::option::Option<i64>,
    /// Unit of the radius. Miles and meters are supported for geo target
    /// constants. Milli miles and meters are supported for feed item sets.
    /// This is required and must be set in CREATE operations.
    #[prost(
        enumeration = "super::enums::location_group_radius_units_enum::LocationGroupRadiusUnits",
        tag = "4"
    )]
    pub radius_units: i32,
    /// FeedItemSets whose FeedItems are targeted. If multiple IDs are specified,
    /// then all items that appear in at least one set are targeted. This field
    /// cannot be used with geo_target_constants. This is optional and can only be
    /// set in CREATE operations.
    #[prost(string, repeated, tag = "8")]
    pub feed_item_sets: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// A mapping that can be used by custom parameter tags in a
/// `tracking_url_template`, `final_urls`, or `mobile_final_urls`.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomParameter {
    /// The key matching the parameter tag name.
    #[prost(string, optional, tag = "3")]
    pub key: ::core::option::Option<::prost::alloc::string::String>,
    /// The value to be substituted.
    #[prost(string, optional, tag = "4")]
    pub value: ::core::option::Option<::prost::alloc::string::String>,
}
/// A rule specifying the maximum number of times an ad (or some set of ads) can
/// be shown to a user over a particular time period.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FrequencyCapEntry {}
/// Metrics data.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Metrics {
    /// The percent of your ad impressions that are shown as the very first ad
    /// above the organic search results.
    #[prost(double, optional, tag = "183")]
    pub absolute_top_impression_percentage: ::core::option::Option<f64>,
    /// All conversions from interactions (as oppose to view through conversions)
    /// divided by the number of ad interactions.
    #[prost(double, optional, tag = "191")]
    pub all_conversions_from_interactions_rate: ::core::option::Option<f64>,
    /// The value of all conversions.
    #[prost(double, optional, tag = "192")]
    pub all_conversions_value: ::core::option::Option<f64>,
    /// The value of all conversions. When this column is selected with date, the
    /// values in date column means the conversion date. Details for the
    /// by_conversion_date columns are available at
    /// <https://support.google.com/sa360/answer/9250611.>
    #[prost(double, tag = "240")]
    pub all_conversions_value_by_conversion_date: f64,
    /// The total number of conversions. This includes all conversions regardless
    /// of the value of include_in_conversions_metric.
    #[prost(double, optional, tag = "193")]
    pub all_conversions: ::core::option::Option<f64>,
    /// The total number of conversions. This includes all conversions regardless
    /// of the value of include_in_conversions_metric. When this column is selected
    /// with date, the values in date column means the conversion date. Details for
    /// the by_conversion_date columns are available at
    /// <https://support.google.com/sa360/answer/9250611.>
    #[prost(double, tag = "241")]
    pub all_conversions_by_conversion_date: f64,
    /// The value of all conversions divided by the total cost of ad interactions
    /// (such as clicks for text ads or views for video ads).
    #[prost(double, optional, tag = "194")]
    pub all_conversions_value_per_cost: ::core::option::Option<f64>,
    /// The number of times people clicked the "Call" button to call a store during
    /// or after clicking an ad. This number doesn't include whether or not calls
    /// were connected, or the duration of any calls.
    ///
    /// This metric applies to feed items only.
    #[prost(double, optional, tag = "195")]
    pub all_conversions_from_click_to_call: ::core::option::Option<f64>,
    /// The number of times people clicked a "Get directions" button to navigate to
    /// a store after clicking an ad.
    ///
    /// This metric applies to feed items only.
    #[prost(double, optional, tag = "196")]
    pub all_conversions_from_directions: ::core::option::Option<f64>,
    /// The value of all conversions from interactions divided by the total number
    /// of interactions.
    #[prost(double, optional, tag = "197")]
    pub all_conversions_from_interactions_value_per_interaction: ::core::option::Option<
        f64,
    >,
    /// The number of times people clicked a link to view a store's menu after
    /// clicking an ad.
    ///
    /// This metric applies to feed items only.
    #[prost(double, optional, tag = "198")]
    pub all_conversions_from_menu: ::core::option::Option<f64>,
    /// The number of times people placed an order at a store after clicking an ad.
    ///
    /// This metric applies to feed items only.
    #[prost(double, optional, tag = "199")]
    pub all_conversions_from_order: ::core::option::Option<f64>,
    /// The number of other conversions (for example, posting a review or saving a
    /// location for a store) that occurred after people clicked an ad.
    ///
    /// This metric applies to feed items only.
    #[prost(double, optional, tag = "200")]
    pub all_conversions_from_other_engagement: ::core::option::Option<f64>,
    /// Estimated number of times people visited a store after clicking an ad.
    ///
    /// This metric applies to feed items only.
    #[prost(double, optional, tag = "201")]
    pub all_conversions_from_store_visit: ::core::option::Option<f64>,
    /// Clicks that Search Ads 360 has successfully recorded and forwarded to an
    /// advertiser's landing page.
    #[prost(double, optional, tag = "289")]
    pub visits: ::core::option::Option<f64>,
    /// The number of times that people were taken to a store's URL after clicking
    /// an ad.
    ///
    /// This metric applies to feed items only.
    #[prost(double, optional, tag = "202")]
    pub all_conversions_from_store_website: ::core::option::Option<f64>,
    /// The average amount you pay per interaction. This amount is the total cost
    /// of your ads divided by the total number of interactions.
    #[prost(double, optional, tag = "203")]
    pub average_cost: ::core::option::Option<f64>,
    /// The total cost of all clicks divided by the total number of clicks
    /// received.
    #[prost(double, optional, tag = "204")]
    pub average_cpc: ::core::option::Option<f64>,
    /// Average cost-per-thousand impressions (CPM).
    #[prost(double, optional, tag = "206")]
    pub average_cpm: ::core::option::Option<f64>,
    /// The number of clicks.
    #[prost(int64, optional, tag = "131")]
    pub clicks: ::core::option::Option<i64>,
    /// The estimated percent of times that your ad was eligible to show
    /// on the Display Network but didn't because your budget was too low.
    /// Note: Content budget lost impression share is reported in the range of 0
    /// to 0.9. Any value above 0.9 is reported as 0.9001.
    #[prost(double, optional, tag = "159")]
    pub content_budget_lost_impression_share: ::core::option::Option<f64>,
    /// The impressions you've received on the Display Network divided
    /// by the estimated number of impressions you were eligible to receive.
    /// Note: Content impression share is reported in the range of 0.1 to 1. Any
    /// value below 0.1 is reported as 0.0999.
    #[prost(double, optional, tag = "160")]
    pub content_impression_share: ::core::option::Option<f64>,
    /// The estimated percentage of impressions on the Display Network
    /// that your ads didn't receive due to poor Ad Rank.
    /// Note: Content rank lost impression share is reported in the range of 0
    /// to 0.9. Any value above 0.9 is reported as 0.9001.
    #[prost(double, optional, tag = "163")]
    pub content_rank_lost_impression_share: ::core::option::Option<f64>,
    /// Average biddable conversions (from interaction) per conversion eligible
    /// interaction. Shows how often, on average, an ad interaction leads to a
    /// biddable conversion.
    #[prost(double, optional, tag = "284")]
    pub conversions_from_interactions_rate: ::core::option::Option<f64>,
    /// The value of client account conversions. This only
    /// includes conversion actions which
    /// include_in_client_account_conversions_metric attribute is set to true. If
    /// you use conversion-based bidding, your bid strategies will optimize for
    /// these conversions.
    #[prost(double, optional, tag = "165")]
    pub client_account_conversions_value: ::core::option::Option<f64>,
    /// The sum of biddable conversions value by conversion date. When this
    /// column is selected with date, the values in date column means the
    /// conversion date.
    #[prost(double, tag = "283")]
    pub conversions_value_by_conversion_date: f64,
    /// The value of biddable conversion divided by the total cost of conversion
    /// eligible interactions.
    #[prost(double, optional, tag = "288")]
    pub conversions_value_per_cost: ::core::option::Option<f64>,
    /// The value of conversions from interactions divided by the number of ad
    /// interactions. This only includes conversion actions which
    /// include_in_conversions_metric attribute is set to true. If you use
    /// conversion-based bidding, your bid strategies will optimize for these
    /// conversions.
    #[prost(double, optional, tag = "167")]
    pub conversions_from_interactions_value_per_interaction: ::core::option::Option<f64>,
    /// The number of client account conversions. This only
    /// includes conversion actions which
    /// include_in_client_account_conversions_metric attribute is set to true. If
    /// you use conversion-based bidding, your bid strategies will optimize for
    /// these conversions.
    #[prost(double, optional, tag = "168")]
    pub client_account_conversions: ::core::option::Option<f64>,
    /// The sum of conversions by conversion date for biddable conversion types.
    /// Can be fractional due to attribution modeling. When this column is selected
    /// with date, the values in date column means the conversion date.
    #[prost(double, tag = "282")]
    pub conversions_by_conversion_date: f64,
    /// The sum of your cost-per-click (CPC) and cost-per-thousand impressions
    /// (CPM) costs during this period.
    #[prost(int64, optional, tag = "169")]
    pub cost_micros: ::core::option::Option<i64>,
    /// The cost of ad interactions divided by all conversions.
    #[prost(double, optional, tag = "170")]
    pub cost_per_all_conversions: ::core::option::Option<f64>,
    /// Average conversion eligible cost per biddable conversion.
    #[prost(double, optional, tag = "286")]
    pub cost_per_conversion: ::core::option::Option<f64>,
    /// The cost of ad interactions divided by current model attributed
    /// conversions. This only includes conversion actions which
    /// include_in_conversions_metric attribute is set to true. If you use
    /// conversion-based bidding, your bid strategies will optimize for these
    /// conversions.
    #[prost(double, optional, tag = "172")]
    pub cost_per_current_model_attributed_conversion: ::core::option::Option<f64>,
    /// Conversions from when a customer clicks on an ad on one device,
    /// then converts on a different device or browser.
    /// Cross-device conversions are already included in all_conversions.
    #[prost(double, optional, tag = "173")]
    pub cross_device_conversions: ::core::option::Option<f64>,
    /// The sum of the value of cross-device conversions.
    #[prost(double, optional, tag = "253")]
    pub cross_device_conversions_value: ::core::option::Option<f64>,
    /// The number of clicks your ad receives (Clicks) divided by the number
    /// of times your ad is shown (Impressions).
    #[prost(double, optional, tag = "174")]
    pub ctr: ::core::option::Option<f64>,
    /// The number of conversions. This only includes conversion actions which
    /// include_in_conversions_metric attribute is set to true. If you use
    /// conversion-based bidding, your bid strategies will optimize for these
    /// conversions.
    #[prost(double, optional, tag = "251")]
    pub conversions: ::core::option::Option<f64>,
    /// The sum of conversion values for the conversions included in the
    /// "conversions" field. This metric is useful only if you entered a value for
    /// your conversion actions.
    #[prost(double, optional, tag = "252")]
    pub conversions_value: ::core::option::Option<f64>,
    /// The creative historical quality score.
    #[prost(
        enumeration = "super::enums::quality_score_bucket_enum::QualityScoreBucket",
        tag = "80"
    )]
    pub historical_creative_quality_score: i32,
    /// The quality of historical landing page experience.
    #[prost(
        enumeration = "super::enums::quality_score_bucket_enum::QualityScoreBucket",
        tag = "81"
    )]
    pub historical_landing_page_quality_score: i32,
    /// The historical quality score.
    #[prost(int64, optional, tag = "216")]
    pub historical_quality_score: ::core::option::Option<i64>,
    /// The historical search predicted click through rate (CTR).
    #[prost(
        enumeration = "super::enums::quality_score_bucket_enum::QualityScoreBucket",
        tag = "83"
    )]
    pub historical_search_predicted_ctr: i32,
    /// Count of how often your ad has appeared on a search results page or
    /// website on the Google Network.
    #[prost(int64, optional, tag = "221")]
    pub impressions: ::core::option::Option<i64>,
    /// How often people interact with your ad after it is shown to them.
    /// This is the number of interactions divided by the number of times your ad
    /// is shown.
    #[prost(double, optional, tag = "222")]
    pub interaction_rate: ::core::option::Option<f64>,
    /// The number of interactions.
    /// An interaction is the main user action associated with an ad format-clicks
    /// for text and shopping ads, views for video ads, and so on.
    #[prost(int64, optional, tag = "223")]
    pub interactions: ::core::option::Option<i64>,
    /// The types of payable and free interactions.
    #[prost(
        enumeration = "super::enums::interaction_event_type_enum::InteractionEventType",
        repeated,
        tag = "100"
    )]
    pub interaction_event_types: ::prost::alloc::vec::Vec<i32>,
    /// The percentage of clicks filtered out of your total number of clicks
    /// (filtered + non-filtered clicks) during the reporting period.
    #[prost(double, optional, tag = "224")]
    pub invalid_click_rate: ::core::option::Option<f64>,
    /// Number of clicks Google considers illegitimate and doesn't charge you for.
    #[prost(int64, optional, tag = "225")]
    pub invalid_clicks: ::core::option::Option<i64>,
    /// The percentage of mobile clicks that go to a mobile-friendly page.
    #[prost(double, optional, tag = "229")]
    pub mobile_friendly_clicks_percentage: ::core::option::Option<f64>,
    /// The percentage of the customer's Shopping or Search ad impressions that are
    /// shown in the most prominent Shopping position. See
    /// <https://support.google.com/sa360/answer/9566729>
    /// for details. Any value below 0.1 is reported as 0.0999.
    #[prost(double, optional, tag = "136")]
    pub search_absolute_top_impression_share: ::core::option::Option<f64>,
    /// The number estimating how often your ad wasn't the very first ad above the
    /// organic search results due to a low budget. Note: Search
    /// budget lost absolute top impression share is reported in the range of 0 to
    /// 0.9. Any value above 0.9 is reported as 0.9001.
    #[prost(double, optional, tag = "137")]
    pub search_budget_lost_absolute_top_impression_share: ::core::option::Option<f64>,
    /// The estimated percent of times that your ad was eligible to show on the
    /// Search Network but didn't because your budget was too low. Note: Search
    /// budget lost impression share is reported in the range of 0 to 0.9. Any
    /// value above 0.9 is reported as 0.9001.
    #[prost(double, optional, tag = "138")]
    pub search_budget_lost_impression_share: ::core::option::Option<f64>,
    /// The number estimating how often your ad didn't show anywhere above the
    /// organic search results due to a low budget. Note: Search
    /// budget lost top impression share is reported in the range of 0 to 0.9. Any
    /// value above 0.9 is reported as 0.9001.
    #[prost(double, optional, tag = "139")]
    pub search_budget_lost_top_impression_share: ::core::option::Option<f64>,
    /// The number of clicks you've received on the Search Network
    /// divided by the estimated number of clicks you were eligible to receive.
    /// Note: Search click share is reported in the range of 0.1 to 1. Any value
    /// below 0.1 is reported as 0.0999.
    #[prost(double, optional, tag = "140")]
    pub search_click_share: ::core::option::Option<f64>,
    /// The impressions you've received divided by the estimated number of
    /// impressions you were eligible to receive on the Search Network for search
    /// terms that matched your keywords exactly (or were close variants of your
    /// keyword), regardless of your keyword match types. Note: Search exact match
    /// impression share is reported in the range of 0.1 to 1. Any value below 0.1
    /// is reported as 0.0999.
    #[prost(double, optional, tag = "141")]
    pub search_exact_match_impression_share: ::core::option::Option<f64>,
    /// The impressions you've received on the Search Network divided
    /// by the estimated number of impressions you were eligible to receive.
    /// Note: Search impression share is reported in the range of 0.1 to 1. Any
    /// value below 0.1 is reported as 0.0999.
    #[prost(double, optional, tag = "142")]
    pub search_impression_share: ::core::option::Option<f64>,
    /// The number estimating how often your ad wasn't the very first ad above the
    /// organic search results due to poor Ad Rank.
    /// Note: Search rank lost absolute top impression share is reported in the
    /// range of 0 to 0.9. Any value above 0.9 is reported as 0.9001.
    #[prost(double, optional, tag = "143")]
    pub search_rank_lost_absolute_top_impression_share: ::core::option::Option<f64>,
    /// The estimated percentage of impressions on the Search Network
    /// that your ads didn't receive due to poor Ad Rank.
    /// Note: Search rank lost impression share is reported in the range of 0 to
    /// 0.9. Any value above 0.9 is reported as 0.9001.
    #[prost(double, optional, tag = "144")]
    pub search_rank_lost_impression_share: ::core::option::Option<f64>,
    /// The number estimating how often your ad didn't show anywhere above the
    /// organic search results due to poor Ad Rank.
    /// Note: Search rank lost top impression share is reported in the range of 0
    /// to 0.9. Any value above 0.9 is reported as 0.9001.
    #[prost(double, optional, tag = "145")]
    pub search_rank_lost_top_impression_share: ::core::option::Option<f64>,
    /// The impressions you've received in the top location (anywhere above the
    /// organic search results) compared to the estimated number of impressions you
    /// were eligible to receive in the top location.
    /// Note: Search top impression share is reported in the range of 0.1 to 1. Any
    /// value below 0.1 is reported as 0.0999.
    #[prost(double, optional, tag = "146")]
    pub search_top_impression_share: ::core::option::Option<f64>,
    /// The percent of your ad impressions that are shown anywhere above the
    /// organic search results.
    #[prost(double, optional, tag = "148")]
    pub top_impression_percentage: ::core::option::Option<f64>,
    /// The value of all conversions divided by the number of all conversions.
    #[prost(double, optional, tag = "150")]
    pub value_per_all_conversions: ::core::option::Option<f64>,
    /// The value of all conversions divided by the number of all conversions. When
    /// this column is selected with date, the values in date column means the
    /// conversion date. Details for the by_conversion_date columns are available
    /// at <https://support.google.com/sa360/answer/9250611.>
    #[prost(double, optional, tag = "244")]
    pub value_per_all_conversions_by_conversion_date: ::core::option::Option<f64>,
    /// The value of biddable conversion divided by the number of biddable
    /// conversions. Shows how much, on average, each of the biddable conversions
    /// is worth.
    #[prost(double, optional, tag = "287")]
    pub value_per_conversion: ::core::option::Option<f64>,
    /// Biddable conversions value by conversion date divided by biddable
    /// conversions by conversion date. Shows how much, on average, each of the
    /// biddable conversions is worth (by conversion date). When this column is
    /// selected with date, the values in date column means the conversion date.
    #[prost(double, optional, tag = "285")]
    pub value_per_conversions_by_conversion_date: ::core::option::Option<f64>,
    /// The total number of view-through conversions.
    /// These happen when a customer sees an image or rich media ad, then later
    /// completes a conversion on your site without interacting with (for example,
    /// clicking on) another ad.
    #[prost(int64, optional, tag = "155")]
    pub client_account_view_through_conversions: ::core::option::Option<i64>,
}
/// Settings for Real-Time Bidding, a feature only available for campaigns
/// targeting the Ad Exchange network.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RealTimeBiddingSetting {
    /// Whether the campaign is opted in to real-time bidding.
    #[prost(bool, optional, tag = "2")]
    pub opt_in: ::core::option::Option<bool>,
}
/// Segment only fields.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Segments {
    /// Resource name of the conversion action.
    #[prost(string, optional, tag = "146")]
    pub conversion_action: ::core::option::Option<::prost::alloc::string::String>,
    /// Conversion action category.
    #[prost(
        enumeration = "super::enums::conversion_action_category_enum::ConversionActionCategory",
        tag = "53"
    )]
    pub conversion_action_category: i32,
    /// Conversion action name.
    #[prost(string, optional, tag = "114")]
    pub conversion_action_name: ::core::option::Option<::prost::alloc::string::String>,
    /// Date to which metrics apply.
    /// yyyy-MM-dd format, for example, 2018-04-17.
    #[prost(string, optional, tag = "79")]
    pub date: ::core::option::Option<::prost::alloc::string::String>,
    /// Day of the week, for example, MONDAY.
    #[prost(enumeration = "super::enums::day_of_week_enum::DayOfWeek", tag = "5")]
    pub day_of_week: i32,
    /// Device to which metrics apply.
    #[prost(enumeration = "super::enums::device_enum::Device", tag = "1")]
    pub device: i32,
    /// Month as represented by the date of the first day of a month. Formatted as
    /// yyyy-MM-dd.
    #[prost(string, optional, tag = "90")]
    pub month: ::core::option::Option<::prost::alloc::string::String>,
    /// Quarter as represented by the date of the first day of a quarter.
    /// Uses the calendar year for quarters, for example, the second quarter of
    /// 2018 starts on 2018-04-01. Formatted as yyyy-MM-dd.
    #[prost(string, optional, tag = "128")]
    pub quarter: ::core::option::Option<::prost::alloc::string::String>,
    /// Week as defined as Monday through Sunday, and represented by the date of
    /// Monday. Formatted as yyyy-MM-dd.
    #[prost(string, optional, tag = "130")]
    pub week: ::core::option::Option<::prost::alloc::string::String>,
    /// Year, formatted as yyyy.
    #[prost(int32, optional, tag = "131")]
    pub year: ::core::option::Option<i32>,
}
/// Settings for the targeting-related features, at the campaign and ad group
/// levels. For more details about the targeting setting, visit
/// <https://support.google.com/google-ads/answer/7365594>
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TargetingSetting {
    /// The per-targeting-dimension setting to restrict the reach of your campaign
    /// or ad group.
    #[prost(message, repeated, tag = "1")]
    pub target_restrictions: ::prost::alloc::vec::Vec<TargetRestriction>,
}
/// The list of per-targeting-dimension targeting settings.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TargetRestriction {
    /// The targeting dimension that these settings apply to.
    #[prost(
        enumeration = "super::enums::targeting_dimension_enum::TargetingDimension",
        tag = "1"
    )]
    pub targeting_dimension: i32,
    /// Indicates whether to restrict your ads to show only for the criteria you
    /// have selected for this targeting_dimension, or to target all values for
    /// this targeting_dimension and show ads based on your targeting in other
    /// TargetingDimensions. A value of `true` means that these criteria will only
    /// apply bid modifiers, and not affect targeting. A value of `false` means
    /// that these criteria will restrict targeting as well as applying bid
    /// modifiers.
    #[prost(bool, optional, tag = "3")]
    pub bid_only: ::core::option::Option<bool>,
}
/// A type of label displaying text on a colored background.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TextLabel {
    /// Background color of the label in RGB format. This string must match the
    /// regular expression '^\#(\[a-fA-F0-9]{6}|[a-fA-F0-9\]{3})$'.
    /// Note: The background color may not be visible for manager accounts.
    #[prost(string, optional, tag = "3")]
    pub background_color: ::core::option::Option<::prost::alloc::string::String>,
    /// A short description of the label. The length must be no more than 200
    /// characters.
    #[prost(string, optional, tag = "4")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
}
/// A generic data container.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Value {
    /// A value.
    #[prost(oneof = "value::Value", tags = "1, 2, 3, 4, 5")]
    pub value: ::core::option::Option<value::Value>,
}
/// Nested message and enum types in `Value`.
pub mod value {
    /// A value.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Value {
        /// A boolean.
        #[prost(bool, tag = "1")]
        BooleanValue(bool),
        /// An int64.
        #[prost(int64, tag = "2")]
        Int64Value(i64),
        /// A float.
        #[prost(float, tag = "3")]
        FloatValue(f32),
        /// A double.
        #[prost(double, tag = "4")]
        DoubleValue(f64),
        /// A string.
        #[prost(string, tag = "5")]
        StringValue(::prost::alloc::string::String),
    }
}
