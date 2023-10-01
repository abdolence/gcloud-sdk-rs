/// An ad.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Ad {
    /// Immutable. The resource name of the ad.
    /// Ad resource names have the form:
    ///
    /// `customers/{customer_id}/ads/{ad_id}`
    #[prost(string, tag = "37")]
    pub resource_name: ::prost::alloc::string::String,
    /// Output only. The ID of the ad.
    #[prost(int64, optional, tag = "40")]
    pub id: ::core::option::Option<i64>,
    /// The list of possible final URLs after all cross-domain redirects for the
    /// ad.
    #[prost(string, repeated, tag = "41")]
    pub final_urls: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The URL that appears in the ad description for some ad formats.
    #[prost(string, optional, tag = "45")]
    pub display_url: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. The type of ad.
    #[prost(enumeration = "super::enums::ad_type_enum::AdType", tag = "5")]
    pub r#type: i32,
    /// Immutable. The name of the ad. This is only used to be able to identify the
    /// ad. It does not need to be unique and does not affect the served ad. The
    /// name field is currently only supported for DisplayUploadAd, ImageAd,
    /// ShoppingComparisonListingAd and VideoAd.
    #[prost(string, optional, tag = "47")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    /// Details pertinent to the ad type. Exactly one value must be set.
    #[prost(oneof = "ad::AdData", tags = "55, 56, 57, 58, 59")]
    pub ad_data: ::core::option::Option<ad::AdData>,
}
/// Nested message and enum types in `Ad`.
pub mod ad {
    /// Details pertinent to the ad type. Exactly one value must be set.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum AdData {
        /// Immutable. Details pertaining to a text ad.
        #[prost(message, tag = "55")]
        TextAd(super::super::common::SearchAds360TextAdInfo),
        /// Immutable. Details pertaining to an expanded text ad.
        #[prost(message, tag = "56")]
        ExpandedTextAd(super::super::common::SearchAds360ExpandedTextAdInfo),
        /// Immutable. Details pertaining to a responsive search ad.
        #[prost(message, tag = "57")]
        ResponsiveSearchAd(super::super::common::SearchAds360ResponsiveSearchAdInfo),
        /// Immutable. Details pertaining to a product ad.
        #[prost(message, tag = "58")]
        ProductAd(super::super::common::SearchAds360ProductAdInfo),
        /// Immutable. Details pertaining to an expanded dynamic search ad.
        #[prost(message, tag = "59")]
        ExpandedDynamicSearchAd(
            super::super::common::SearchAds360ExpandedDynamicSearchAdInfo,
        ),
    }
}
/// An ad group.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdGroup {
    /// Immutable. The resource name of the ad group.
    /// Ad group resource names have the form:
    ///
    /// `customers/{customer_id}/adGroups/{ad_group_id}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Output only. The ID of the ad group.
    #[prost(int64, optional, tag = "34")]
    pub id: ::core::option::Option<i64>,
    /// The name of the ad group.
    ///
    /// This field is required and should not be empty when creating new ad
    /// groups.
    ///
    /// It must contain fewer than 255 UTF-8 full-width characters.
    ///
    /// It must not contain any null (code point 0x0), NL line feed
    /// (code point 0xA) or carriage return (code point 0xD) characters.
    #[prost(string, optional, tag = "35")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    /// The status of the ad group.
    #[prost(
        enumeration = "super::enums::ad_group_status_enum::AdGroupStatus",
        tag = "5"
    )]
    pub status: i32,
    /// Immutable. The type of the ad group.
    #[prost(enumeration = "super::enums::ad_group_type_enum::AdGroupType", tag = "12")]
    pub r#type: i32,
    /// The ad rotation mode of the ad group.
    #[prost(
        enumeration = "super::enums::ad_group_ad_rotation_mode_enum::AdGroupAdRotationMode",
        tag = "22"
    )]
    pub ad_rotation_mode: i32,
    /// The maximum CPC (cost-per-click) bid.
    #[prost(int64, optional, tag = "39")]
    pub cpc_bid_micros: ::core::option::Option<i64>,
    /// Output only. The timestamp when this ad_group was created. The timestamp is
    /// in the customer's time zone and in "yyyy-MM-dd HH:mm:ss" format.
    #[prost(string, tag = "60")]
    pub creation_time: ::prost::alloc::string::String,
    /// Output only. The Engine Status for ad group.
    #[prost(
        enumeration = "super::enums::ad_group_engine_status_enum::AdGroupEngineStatus",
        optional,
        tag = "61"
    )]
    pub engine_status: ::core::option::Option<i32>,
    /// Setting for targeting related features.
    #[prost(message, optional, tag = "25")]
    pub targeting_setting: ::core::option::Option<super::common::TargetingSetting>,
    /// Output only. The resource names of labels attached to this ad group.
    #[prost(string, repeated, tag = "49")]
    pub labels: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Output only. ID of the ad group in the external engine account. This field
    /// is for non-Google Ads account only, for example, Yahoo Japan, Microsoft,
    /// Baidu etc. For Google Ads entity, use "ad_group.id" instead.
    #[prost(string, tag = "50")]
    pub engine_id: ::prost::alloc::string::String,
    /// Output only. Date when this ad group starts serving ads. By default, the ad
    /// group starts now or the ad group's start date, whichever is later. If this
    /// field is set, then the ad group starts at the beginning of the specified
    /// date in the customer's time zone. This field is only available for
    /// Microsoft Advertising and Facebook gateway accounts.
    ///
    /// Format: YYYY-MM-DD
    /// Example: 2019-03-14
    #[prost(string, tag = "51")]
    pub start_date: ::prost::alloc::string::String,
    /// Output only. Date when the ad group ends serving ads. By default, the ad
    /// group ends on the ad group's end date. If this field is set, then the ad
    /// group ends at the end of the specified date in the customer's time zone.
    /// This field is only available for Microsoft Advertising and Facebook gateway
    /// accounts.
    ///
    /// Format: YYYY-MM-DD
    /// Example: 2019-03-14
    #[prost(string, tag = "52")]
    pub end_date: ::prost::alloc::string::String,
    /// Output only. The language of the ads and keywords in an ad group. This
    /// field is only available for Microsoft Advertising accounts. More details:
    /// <https://docs.microsoft.com/en-us/advertising/guides/ad-languages?view=bingads-13#adlanguage>
    #[prost(string, tag = "53")]
    pub language_code: ::prost::alloc::string::String,
    /// Output only. The datetime when this ad group was last modified. The
    /// datetime is in the customer's time zone and in "yyyy-MM-dd HH:mm:ss.ssssss"
    /// format.
    #[prost(string, tag = "55")]
    pub last_modified_time: ::prost::alloc::string::String,
}
/// An ad group ad.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdGroupAd {
    /// Immutable. The resource name of the ad.
    /// Ad group ad resource names have the form:
    ///
    /// `customers/{customer_id}/adGroupAds/{ad_group_id}~{ad_id}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// The status of the ad.
    #[prost(
        enumeration = "super::enums::ad_group_ad_status_enum::AdGroupAdStatus",
        tag = "3"
    )]
    pub status: i32,
    /// Immutable. The ad.
    #[prost(message, optional, tag = "5")]
    pub ad: ::core::option::Option<Ad>,
    /// Output only. The timestamp when this ad_group_ad was created. The datetime
    /// is in the customer's time zone and in "yyyy-MM-dd HH:mm:ss.ssssss" format.
    #[prost(string, tag = "14")]
    pub creation_time: ::prost::alloc::string::String,
    /// Output only. The resource names of labels attached to this ad group ad.
    #[prost(string, repeated, tag = "10")]
    pub labels: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Output only. ID of the ad in the external engine account. This field is for
    /// SearchAds 360 account only, for example, Yahoo Japan, Microsoft, Baidu etc.
    /// For non-SearchAds 360 entity, use "ad_group_ad.ad.id" instead.
    #[prost(string, tag = "11")]
    pub engine_id: ::prost::alloc::string::String,
    /// Output only. Additional status of the ad in the external engine account.
    /// Possible statuses (depending on the type of external account) include
    /// active, eligible, pending review, etc.
    #[prost(
        enumeration = "super::enums::ad_group_ad_engine_status_enum::AdGroupAdEngineStatus",
        tag = "15"
    )]
    pub engine_status: i32,
    /// Output only. The datetime when this ad group ad was last modified. The
    /// datetime is in the customer's time zone and in "yyyy-MM-dd HH:mm:ss.ssssss"
    /// format.
    #[prost(string, tag = "12")]
    pub last_modified_time: ::prost::alloc::string::String,
}
/// A relationship between an ad group ad and a label.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdGroupAdLabel {
    /// Immutable. The resource name of the ad group ad label.
    /// Ad group ad label resource names have the form:
    /// `customers/{customer_id}/adGroupAdLabels/{ad_group_id}~{ad_id}~{label_id}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Immutable. The ad group ad to which the label is attached.
    #[prost(string, optional, tag = "4")]
    pub ad_group_ad: ::core::option::Option<::prost::alloc::string::String>,
    /// Immutable. The label assigned to the ad group ad.
    #[prost(string, optional, tag = "5")]
    pub label: ::core::option::Option<::prost::alloc::string::String>,
}
/// A link between an ad group and an asset.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdGroupAsset {
    /// Immutable. The resource name of the ad group asset.
    /// AdGroupAsset resource names have the form:
    ///
    /// `customers/{customer_id}/adGroupAssets/{ad_group_id}~{asset_id}~{field_type}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Required. Immutable. The ad group to which the asset is linked.
    #[prost(string, tag = "2")]
    pub ad_group: ::prost::alloc::string::String,
    /// Required. Immutable. The asset which is linked to the ad group.
    #[prost(string, tag = "3")]
    pub asset: ::prost::alloc::string::String,
    /// Status of the ad group asset.
    #[prost(
        enumeration = "super::enums::asset_link_status_enum::AssetLinkStatus",
        tag = "5"
    )]
    pub status: i32,
}
/// AdGroupAssetSet is the linkage between an ad group and an asset set.
/// Creating an AdGroupAssetSet links an asset set with an ad group.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdGroupAssetSet {
    /// Immutable. The resource name of the ad group asset set.
    /// Ad group asset set resource names have the form:
    ///
    /// `customers/{customer_id}/adGroupAssetSets/{ad_group_id}~{asset_set_id}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Immutable. The ad group to which this asset set is linked.
    #[prost(string, tag = "2")]
    pub ad_group: ::prost::alloc::string::String,
    /// Immutable. The asset set which is linked to the ad group.
    #[prost(string, tag = "3")]
    pub asset_set: ::prost::alloc::string::String,
    /// Output only. The status of the ad group asset set. Read-only.
    #[prost(
        enumeration = "super::enums::asset_set_link_status_enum::AssetSetLinkStatus",
        tag = "4"
    )]
    pub status: i32,
}
/// An ad group audience view.
/// Includes performance data from interests and remarketing lists for Display
/// Network and YouTube Network ads, and remarketing lists for search ads (RLSA),
/// aggregated at the audience level.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdGroupAudienceView {
    /// Output only. The resource name of the ad group audience view.
    /// Ad group audience view resource names have the form:
    ///
    /// `customers/{customer_id}/adGroupAudienceViews/{ad_group_id}~{criterion_id}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
}
/// Represents an ad group bid modifier.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdGroupBidModifier {
    /// Immutable. The resource name of the ad group bid modifier.
    /// Ad group bid modifier resource names have the form:
    ///
    /// `customers/{customer_id}/adGroupBidModifiers/{ad_group_id}~{criterion_id}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// The modifier for the bid when the criterion matches. The modifier must be
    /// in the range: 0.1 - 10.0. The range is 1.0 - 6.0 for PreferredContent.
    /// Use 0 to opt out of a Device type.
    #[prost(double, optional, tag = "15")]
    pub bid_modifier: ::core::option::Option<f64>,
    /// The criterion of this ad group bid modifier.
    ///
    /// Required in create operations starting in V5.
    #[prost(oneof = "ad_group_bid_modifier::Criterion", tags = "11")]
    pub criterion: ::core::option::Option<ad_group_bid_modifier::Criterion>,
}
/// Nested message and enum types in `AdGroupBidModifier`.
pub mod ad_group_bid_modifier {
    /// The criterion of this ad group bid modifier.
    ///
    /// Required in create operations starting in V5.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Criterion {
        /// Immutable. A device criterion.
        #[prost(message, tag = "11")]
        Device(super::super::common::DeviceInfo),
    }
}
/// An ad group criterion.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdGroupCriterion {
    /// Immutable. The resource name of the ad group criterion.
    /// Ad group criterion resource names have the form:
    ///
    /// `customers/{customer_id}/adGroupCriteria/{ad_group_id}~{criterion_id}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Output only. The ID of the criterion.
    #[prost(int64, optional, tag = "56")]
    pub criterion_id: ::core::option::Option<i64>,
    /// Output only. The timestamp when this ad group criterion was created. The
    /// timestamp is in the customer's time zone and in "yyyy-MM-dd HH:mm:ss"
    /// format.
    #[prost(string, tag = "81")]
    pub creation_time: ::prost::alloc::string::String,
    /// The status of the criterion.
    ///
    /// This is the status of the ad group criterion entity, set by the client.
    /// Note: UI reports may incorporate additional information that affects
    /// whether a criterion is eligible to run. In some cases a criterion that's
    /// REMOVED in the API can still show as enabled in the UI.
    /// For example, campaigns by default show to users of all age ranges unless
    /// excluded. The UI will show each age range as "enabled", since they're
    /// eligible to see the ads; but AdGroupCriterion.status will show "removed",
    /// since no positive criterion was added.
    #[prost(
        enumeration = "super::enums::ad_group_criterion_status_enum::AdGroupCriterionStatus",
        tag = "3"
    )]
    pub status: i32,
    /// Output only. Information regarding the quality of the criterion.
    #[prost(message, optional, tag = "4")]
    pub quality_info: ::core::option::Option<ad_group_criterion::QualityInfo>,
    /// Immutable. The ad group to which the criterion belongs.
    #[prost(string, optional, tag = "57")]
    pub ad_group: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. The type of the criterion.
    #[prost(
        enumeration = "super::enums::criterion_type_enum::CriterionType",
        tag = "25"
    )]
    pub r#type: i32,
    /// Immutable. Whether to target (`false`) or exclude (`true`) the criterion.
    ///
    /// This field is immutable. To switch a criterion from positive to negative,
    /// remove then re-add it.
    #[prost(bool, optional, tag = "58")]
    pub negative: ::core::option::Option<bool>,
    /// Output only. The resource names of labels attached to this ad group
    /// criterion.
    #[prost(string, repeated, tag = "60")]
    pub labels: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The modifier for the bid when the criterion matches. The modifier must be
    /// in the range: 0.1 - 10.0. Most targetable criteria types support modifiers.
    #[prost(double, optional, tag = "61")]
    pub bid_modifier: ::core::option::Option<f64>,
    /// The CPC (cost-per-click) bid.
    #[prost(int64, optional, tag = "62")]
    pub cpc_bid_micros: ::core::option::Option<i64>,
    /// Output only. The effective CPC (cost-per-click) bid.
    #[prost(int64, optional, tag = "66")]
    pub effective_cpc_bid_micros: ::core::option::Option<i64>,
    /// Output only. Estimates for criterion bids at various positions.
    #[prost(message, optional, tag = "10")]
    pub position_estimates: ::core::option::Option<
        ad_group_criterion::PositionEstimates,
    >,
    /// The list of possible final URLs after all cross-domain redirects for the
    /// ad.
    #[prost(string, repeated, tag = "70")]
    pub final_urls: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Output only. The Engine Status for ad group criterion.
    #[prost(
        enumeration = "super::enums::ad_group_criterion_engine_status_enum::AdGroupCriterionEngineStatus",
        optional,
        tag = "80"
    )]
    pub engine_status: ::core::option::Option<i32>,
    /// URL template for appending params to final URL.
    #[prost(string, optional, tag = "72")]
    pub final_url_suffix: ::core::option::Option<::prost::alloc::string::String>,
    /// The URL template for constructing a tracking URL.
    #[prost(string, optional, tag = "73")]
    pub tracking_url_template: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. ID of the ad group criterion in the external engine account.
    /// This field is for non-Google Ads account only, for example, Yahoo Japan,
    /// Microsoft, Baidu etc. For Google Ads entity, use
    /// "ad_group_criterion.criterion_id" instead.
    #[prost(string, tag = "76")]
    pub engine_id: ::prost::alloc::string::String,
    /// Output only. The datetime when this ad group criterion was last modified.
    /// The datetime is in the customer's time zone and in "yyyy-MM-dd
    /// HH:mm:ss.ssssss" format.
    #[prost(string, tag = "78")]
    pub last_modified_time: ::prost::alloc::string::String,
    /// The ad group criterion.
    ///
    /// Exactly one must be set.
    #[prost(
        oneof = "ad_group_criterion::Criterion",
        tags = "27, 32, 36, 37, 42, 46, 82"
    )]
    pub criterion: ::core::option::Option<ad_group_criterion::Criterion>,
}
/// Nested message and enum types in `AdGroupCriterion`.
pub mod ad_group_criterion {
    /// A container for ad group criterion quality information.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct QualityInfo {
        /// Output only. The quality score.
        ///
        /// This field may not be populated if Google does not have enough
        /// information to determine a value.
        #[prost(int32, optional, tag = "5")]
        pub quality_score: ::core::option::Option<i32>,
    }
    /// Estimates for criterion bids at various positions.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PositionEstimates {
        /// Output only. The estimate of the CPC bid required for ad to be displayed
        /// at the top of the first page of search results.
        #[prost(int64, optional, tag = "8")]
        pub top_of_page_cpc_micros: ::core::option::Option<i64>,
    }
    /// The ad group criterion.
    ///
    /// Exactly one must be set.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Criterion {
        /// Immutable. Keyword.
        #[prost(message, tag = "27")]
        Keyword(super::super::common::KeywordInfo),
        /// Immutable. Listing group.
        #[prost(message, tag = "32")]
        ListingGroup(super::super::common::ListingGroupInfo),
        /// Immutable. Age range.
        #[prost(message, tag = "36")]
        AgeRange(super::super::common::AgeRangeInfo),
        /// Immutable. Gender.
        #[prost(message, tag = "37")]
        Gender(super::super::common::GenderInfo),
        /// Immutable. User List.
        /// The Similar Audiences sunset starts May 2023. Refer to
        /// <https://ads-developers.googleblog.com/2022/11/announcing-deprecation-and-sunset-of.html>
        /// for other options.
        #[prost(message, tag = "42")]
        UserList(super::super::common::UserListInfo),
        /// Immutable. Webpage
        #[prost(message, tag = "46")]
        Webpage(super::super::common::WebpageInfo),
        /// Immutable. Location.
        #[prost(message, tag = "82")]
        Location(super::super::common::LocationInfo),
    }
}
/// A relationship between an ad group criterion and a label.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdGroupCriterionLabel {
    /// Immutable. The resource name of the ad group criterion label.
    /// Ad group criterion label resource names have the form:
    /// `customers/{customer_id}/adGroupCriterionLabels/{ad_group_id}~{criterion_id}~{label_id}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Immutable. The ad group criterion to which the label is attached.
    #[prost(string, optional, tag = "4")]
    pub ad_group_criterion: ::core::option::Option<::prost::alloc::string::String>,
    /// Immutable. The label assigned to the ad group criterion.
    #[prost(string, optional, tag = "5")]
    pub label: ::core::option::Option<::prost::alloc::string::String>,
}
/// A relationship between an ad group and a label.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdGroupLabel {
    /// Immutable. The resource name of the ad group label.
    /// Ad group label resource names have the form:
    /// `customers/{customer_id}/adGroupLabels/{ad_group_id}~{label_id}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Immutable. The ad group to which the label is attached.
    #[prost(string, optional, tag = "4")]
    pub ad_group: ::core::option::Option<::prost::alloc::string::String>,
    /// Immutable. The label assigned to the ad group.
    #[prost(string, optional, tag = "5")]
    pub label: ::core::option::Option<::prost::alloc::string::String>,
}
/// An age range view.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AgeRangeView {
    /// Output only. The resource name of the age range view.
    /// Age range view resource names have the form:
    ///
    /// `customers/{customer_id}/ageRangeViews/{ad_group_id}~{criterion_id}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
}
/// Asset is a part of an ad which can be shared across multiple ads.
/// It can be an image (ImageAsset), a video (YoutubeVideoAsset), etc.
/// Assets are immutable and cannot be removed. To stop an asset from serving,
/// remove the asset from the entity that is using it.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Asset {
    /// Immutable. The resource name of the asset.
    /// Asset resource names have the form:
    ///
    /// `customers/{customer_id}/assets/{asset_id}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Output only. The ID of the asset.
    #[prost(int64, optional, tag = "11")]
    pub id: ::core::option::Option<i64>,
    /// Output only. Type of the asset.
    #[prost(enumeration = "super::enums::asset_type_enum::AssetType", tag = "4")]
    pub r#type: i32,
    /// A list of possible final URLs after all cross domain redirects.
    #[prost(string, repeated, tag = "14")]
    pub final_urls: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// URL template for constructing a tracking URL.
    #[prost(string, optional, tag = "17")]
    pub tracking_url_template: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. The status of the asset.
    #[prost(enumeration = "super::enums::asset_status_enum::AssetStatus", tag = "42")]
    pub status: i32,
    /// Output only. The timestamp when this asset was created. The timestamp is in
    /// the customer's time zone and in "yyyy-MM-dd HH:mm:ss" format.
    #[prost(string, tag = "43")]
    pub creation_time: ::prost::alloc::string::String,
    /// Output only. The datetime when this asset was last modified. The datetime
    /// is in the customer's time zone and in "yyyy-MM-dd HH:mm:ss.ssssss" format.
    #[prost(string, tag = "44")]
    pub last_modified_time: ::prost::alloc::string::String,
    /// Output only. The Engine Status for an asset.
    #[prost(
        enumeration = "super::enums::asset_engine_status_enum::AssetEngineStatus",
        optional,
        tag = "61"
    )]
    pub engine_status: ::core::option::Option<i32>,
    /// The specific type of the asset.
    #[prost(oneof = "asset::AssetData", tags = "48, 45, 46, 25, 47, 49")]
    pub asset_data: ::core::option::Option<asset::AssetData>,
}
/// Nested message and enum types in `Asset`.
pub mod asset {
    /// The specific type of the asset.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum AssetData {
        /// Output only. A unified callout asset.
        #[prost(message, tag = "48")]
        CalloutAsset(super::super::common::UnifiedCalloutAsset),
        /// Output only. A unified sitelink asset.
        #[prost(message, tag = "45")]
        SitelinkAsset(super::super::common::UnifiedSitelinkAsset),
        /// Output only. A unified page feed asset.
        #[prost(message, tag = "46")]
        PageFeedAsset(super::super::common::UnifiedPageFeedAsset),
        /// A mobile app asset.
        #[prost(message, tag = "25")]
        MobileAppAsset(super::super::common::MobileAppAsset),
        /// Output only. A unified call asset.
        #[prost(message, tag = "47")]
        CallAsset(super::super::common::UnifiedCallAsset),
        /// Output only. A unified location asset.
        #[prost(message, tag = "49")]
        LocationAsset(super::super::common::UnifiedLocationAsset),
    }
}
/// An asset set representing a collection of assets.
/// Use AssetSetAsset to link an asset to the asset set.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetSet {
    /// Output only. The ID of the asset set.
    #[prost(int64, tag = "6")]
    pub id: i64,
    /// Immutable. The resource name of the asset set.
    /// Asset set resource names have the form:
    ///
    /// `customers/{customer_id}/assetSets/{asset_set_id}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
}
/// AssetSetAsset is the link between an asset and an asset set.
/// Adding an AssetSetAsset links an asset with an asset set.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetSetAsset {
    /// Immutable. The resource name of the asset set asset.
    /// Asset set asset resource names have the form:
    ///
    /// `customers/{customer_id}/assetSetAssets/{asset_set_id}~{asset_id}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Immutable. The asset set which this asset set asset is linking to.
    #[prost(string, tag = "2")]
    pub asset_set: ::prost::alloc::string::String,
    /// Immutable. The asset which this asset set asset is linking to.
    #[prost(string, tag = "3")]
    pub asset: ::prost::alloc::string::String,
    /// Output only. The status of the asset set asset. Read-only.
    #[prost(
        enumeration = "super::enums::asset_set_asset_status_enum::AssetSetAssetStatus",
        tag = "4"
    )]
    pub status: i32,
}
/// A bidding strategy.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BiddingStrategy {
    /// Immutable. The resource name of the bidding strategy.
    /// Bidding strategy resource names have the form:
    ///
    /// `customers/{customer_id}/biddingStrategies/{bidding_strategy_id}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Output only. The ID of the bidding strategy.
    #[prost(int64, optional, tag = "16")]
    pub id: ::core::option::Option<i64>,
    /// The name of the bidding strategy.
    /// All bidding strategies within an account must be named distinctly.
    ///
    /// The length of this string should be between 1 and 255, inclusive,
    /// in UTF-8 bytes, (trimmed).
    #[prost(string, optional, tag = "17")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. The status of the bidding strategy.
    ///
    /// This field is read-only.
    #[prost(
        enumeration = "super::enums::bidding_strategy_status_enum::BiddingStrategyStatus",
        tag = "15"
    )]
    pub status: i32,
    /// Output only. The type of the bidding strategy.
    /// Create a bidding strategy by setting the bidding scheme.
    ///
    /// This field is read-only.
    #[prost(
        enumeration = "super::enums::bidding_strategy_type_enum::BiddingStrategyType",
        tag = "5"
    )]
    pub r#type: i32,
    /// Immutable. The currency used by the bidding strategy (ISO 4217 three-letter
    /// code).
    ///
    /// For bidding strategies in manager customers, this currency can be set on
    /// creation and defaults to the manager customer's currency. For serving
    /// customers, this field cannot be set; all strategies in a serving customer
    /// implicitly use the serving customer's currency. In all cases the
    /// effective_currency_code field returns the currency used by the strategy.
    #[prost(string, tag = "23")]
    pub currency_code: ::prost::alloc::string::String,
    /// Output only. The currency used by the bidding strategy (ISO 4217
    /// three-letter code).
    ///
    /// For bidding strategies in manager customers, this is the currency set by
    /// the advertiser when creating the strategy. For serving customers, this is
    /// the customer's currency_code.
    ///
    /// Bidding strategy metrics are reported in this currency.
    ///
    /// This field is read-only.
    #[prost(string, optional, tag = "20")]
    pub effective_currency_code: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. The number of campaigns attached to this bidding strategy.
    ///
    /// This field is read-only.
    #[prost(int64, optional, tag = "18")]
    pub campaign_count: ::core::option::Option<i64>,
    /// Output only. The number of non-removed campaigns attached to this bidding
    /// strategy.
    ///
    /// This field is read-only.
    #[prost(int64, optional, tag = "19")]
    pub non_removed_campaign_count: ::core::option::Option<i64>,
    /// The bidding scheme.
    ///
    /// Only one can be set.
    #[prost(oneof = "bidding_strategy::Scheme", tags = "7, 21, 22, 9, 48, 10, 11, 12")]
    pub scheme: ::core::option::Option<bidding_strategy::Scheme>,
}
/// Nested message and enum types in `BiddingStrategy`.
pub mod bidding_strategy {
    /// The bidding scheme.
    ///
    /// Only one can be set.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Scheme {
        /// A bidding strategy that raises bids for clicks that seem more likely to
        /// lead to a conversion and lowers them for clicks where they seem less
        /// likely.
        #[prost(message, tag = "7")]
        EnhancedCpc(super::super::common::EnhancedCpc),
        /// An automated bidding strategy to help get the most conversion value for
        /// your campaigns while spending your budget.
        #[prost(message, tag = "21")]
        MaximizeConversionValue(super::super::common::MaximizeConversionValue),
        /// An automated bidding strategy to help get the most conversions for your
        /// campaigns while spending your budget.
        #[prost(message, tag = "22")]
        MaximizeConversions(super::super::common::MaximizeConversions),
        /// A bidding strategy that sets bids to help get as many conversions as
        /// possible at the target cost-per-acquisition (CPA) you set.
        #[prost(message, tag = "9")]
        TargetCpa(super::super::common::TargetCpa),
        /// A bidding strategy that automatically optimizes towards a chosen
        /// percentage of impressions.
        #[prost(message, tag = "48")]
        TargetImpressionShare(super::super::common::TargetImpressionShare),
        /// A bidding strategy that sets bids based on the target fraction of
        /// auctions where the advertiser should outrank a specific competitor.
        /// This field is deprecated. Creating a new bidding strategy with this
        /// field or attaching bidding strategies with this field to a campaign will
        /// fail. Mutates to strategies that already have this scheme populated are
        /// allowed.
        #[prost(message, tag = "10")]
        TargetOutrankShare(super::super::common::TargetOutrankShare),
        /// A bidding strategy that helps you maximize revenue while averaging a
        /// specific target Return On Ad Spend (ROAS).
        #[prost(message, tag = "11")]
        TargetRoas(super::super::common::TargetRoas),
        /// A bid strategy that sets your bids to help get as many clicks as
        /// possible within your budget.
        #[prost(message, tag = "12")]
        TargetSpend(super::super::common::TargetSpend),
    }
}
/// A campaign.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Campaign {
    /// Immutable. The resource name of the campaign.
    /// Campaign resource names have the form:
    ///
    /// `customers/{customer_id}/campaigns/{campaign_id}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Output only. The ID of the campaign.
    #[prost(int64, optional, tag = "59")]
    pub id: ::core::option::Option<i64>,
    /// The name of the campaign.
    ///
    /// This field is required and should not be empty when creating new
    /// campaigns.
    ///
    /// It must not contain any null (code point 0x0), NL line feed
    /// (code point 0xA) or carriage return (code point 0xD) characters.
    #[prost(string, optional, tag = "58")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    /// The status of the campaign.
    ///
    /// When a new campaign is added, the status defaults to ENABLED.
    #[prost(
        enumeration = "super::enums::campaign_status_enum::CampaignStatus",
        tag = "5"
    )]
    pub status: i32,
    /// Output only. The ad serving status of the campaign.
    #[prost(
        enumeration = "super::enums::campaign_serving_status_enum::CampaignServingStatus",
        tag = "21"
    )]
    pub serving_status: i32,
    /// Output only. The system status of the campaign's bidding strategy.
    #[prost(
        enumeration = "super::enums::bidding_strategy_system_status_enum::BiddingStrategySystemStatus",
        tag = "78"
    )]
    pub bidding_strategy_system_status: i32,
    /// The ad serving optimization status of the campaign.
    #[prost(
        enumeration = "super::enums::ad_serving_optimization_status_enum::AdServingOptimizationStatus",
        tag = "8"
    )]
    pub ad_serving_optimization_status: i32,
    /// Immutable. The primary serving target for ads within the campaign.
    /// The targeting options can be refined in `network_settings`.
    ///
    /// This field is required and should not be empty when creating new
    /// campaigns.
    ///
    /// Can be set only when creating campaigns.
    /// After the campaign is created, the field can not be changed.
    #[prost(
        enumeration = "super::enums::advertising_channel_type_enum::AdvertisingChannelType",
        tag = "9"
    )]
    pub advertising_channel_type: i32,
    /// Immutable. Optional refinement to `advertising_channel_type`.
    /// Must be a valid sub-type of the parent channel type.
    ///
    /// Can be set only when creating campaigns.
    /// After campaign is created, the field can not be changed.
    #[prost(
        enumeration = "super::enums::advertising_channel_sub_type_enum::AdvertisingChannelSubType",
        tag = "10"
    )]
    pub advertising_channel_sub_type: i32,
    /// The URL template for constructing a tracking URL.
    #[prost(string, optional, tag = "60")]
    pub tracking_url_template: ::core::option::Option<::prost::alloc::string::String>,
    /// The list of mappings used to substitute custom parameter tags in a
    /// `tracking_url_template`, `final_urls`, or `mobile_final_urls`.
    #[prost(message, repeated, tag = "12")]
    pub url_custom_parameters: ::prost::alloc::vec::Vec<super::common::CustomParameter>,
    /// Settings for Real-Time Bidding, a feature only available for campaigns
    /// targeting the Ad Exchange network.
    #[prost(message, optional, tag = "39")]
    pub real_time_bidding_setting: ::core::option::Option<
        super::common::RealTimeBiddingSetting,
    >,
    /// The network settings for the campaign.
    #[prost(message, optional, tag = "14")]
    pub network_settings: ::core::option::Option<campaign::NetworkSettings>,
    /// The setting for controlling Dynamic Search Ads (DSA).
    #[prost(message, optional, tag = "33")]
    pub dynamic_search_ads_setting: ::core::option::Option<
        campaign::DynamicSearchAdsSetting,
    >,
    /// The setting for controlling Shopping campaigns.
    #[prost(message, optional, tag = "36")]
    pub shopping_setting: ::core::option::Option<campaign::ShoppingSetting>,
    /// The setting for ads geotargeting.
    #[prost(message, optional, tag = "47")]
    pub geo_target_type_setting: ::core::option::Option<campaign::GeoTargetTypeSetting>,
    /// Output only. The resource names of labels attached to this campaign.
    #[prost(string, repeated, tag = "61")]
    pub labels: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The budget of the campaign.
    #[prost(string, optional, tag = "62")]
    pub campaign_budget: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. The type of bidding strategy.
    ///
    /// A bidding strategy can be created by setting either the bidding scheme to
    /// create a standard bidding strategy or the `bidding_strategy` field to
    /// create a portfolio bidding strategy.
    ///
    /// This field is read-only.
    #[prost(
        enumeration = "super::enums::bidding_strategy_type_enum::BiddingStrategyType",
        tag = "22"
    )]
    pub bidding_strategy_type: i32,
    /// The date when campaign started in serving customer's timezone in YYYY-MM-DD
    /// format.
    #[prost(string, optional, tag = "63")]
    pub start_date: ::core::option::Option<::prost::alloc::string::String>,
    /// The last day of the campaign in serving customer's timezone in YYYY-MM-DD
    /// format. On create, defaults to 2037-12-30, which means the campaign will
    /// run indefinitely. To set an existing campaign to run indefinitely, set this
    /// field to 2037-12-30.
    #[prost(string, optional, tag = "64")]
    pub end_date: ::core::option::Option<::prost::alloc::string::String>,
    /// Suffix used to append query parameters to landing pages that are served
    /// with parallel tracking.
    #[prost(string, optional, tag = "65")]
    pub final_url_suffix: ::core::option::Option<::prost::alloc::string::String>,
    /// A list that limits how often each user will see this campaign's ads.
    #[prost(message, repeated, tag = "40")]
    pub frequency_caps: ::prost::alloc::vec::Vec<super::common::FrequencyCapEntry>,
    /// Selective optimization setting for this campaign, which includes a set of
    /// conversion actions to optimize this campaign towards.
    /// This feature only applies to app campaigns that use MULTI_CHANNEL as
    /// AdvertisingChannelType and APP_CAMPAIGN or APP_CAMPAIGN_FOR_ENGAGEMENT as
    /// AdvertisingChannelSubType.
    #[prost(message, optional, tag = "45")]
    pub selective_optimization: ::core::option::Option<campaign::SelectiveOptimization>,
    /// Optimization goal setting for this campaign, which includes a set of
    /// optimization goal types.
    #[prost(message, optional, tag = "54")]
    pub optimization_goal_setting: ::core::option::Option<
        campaign::OptimizationGoalSetting,
    >,
    /// Output only. Campaign-level settings for tracking information.
    #[prost(message, optional, tag = "46")]
    pub tracking_setting: ::core::option::Option<campaign::TrackingSetting>,
    /// Output only. ID of the campaign in the external engine account. This field
    /// is for non-Google Ads account only, for example, Yahoo Japan, Microsoft,
    /// Baidu etc. For Google Ads entity, use "campaign.id" instead.
    #[prost(string, tag = "68")]
    pub engine_id: ::prost::alloc::string::String,
    /// The asset field types that should be excluded from this campaign. Asset
    /// links with these field types will not be inherited by this campaign from
    /// the upper level.
    #[prost(
        enumeration = "super::enums::asset_field_type_enum::AssetFieldType",
        repeated,
        tag = "69"
    )]
    pub excluded_parent_asset_field_types: ::prost::alloc::vec::Vec<i32>,
    /// Output only. The timestamp when this campaign was created. The timestamp is
    /// in the customer's time zone and in "yyyy-MM-dd HH:mm:ss" format.
    /// create_time will be deprecated in v1. Use creation_time instead.
    #[prost(string, tag = "79")]
    pub create_time: ::prost::alloc::string::String,
    /// Output only. The timestamp when this campaign was created. The timestamp is
    /// in the customer's time zone and in "yyyy-MM-dd HH:mm:ss" format.
    #[prost(string, tag = "84")]
    pub creation_time: ::prost::alloc::string::String,
    /// Output only. The datetime when this campaign was last modified. The
    /// datetime is in the customer's time zone and in "yyyy-MM-dd HH:mm:ss.ssssss"
    /// format.
    #[prost(string, tag = "70")]
    pub last_modified_time: ::prost::alloc::string::String,
    /// Represents opting out of URL expansion to more targeted URLs. If opted out
    /// (true), only the final URLs in the asset group or URLs specified in the
    /// advertiser's Google Merchant Center or business data feeds are targeted.
    /// If opted in (false), the entire domain will be targeted. This field can
    /// only be set for Performance Max campaigns, where the default value is
    /// false.
    #[prost(bool, optional, tag = "72")]
    pub url_expansion_opt_out: ::core::option::Option<bool>,
    /// The bidding strategy for the campaign.
    ///
    /// Must be either portfolio (created through BiddingStrategy service) or
    /// standard, that is embedded into the campaign.
    #[prost(
        oneof = "campaign::CampaignBiddingStrategy",
        tags = "67, 74, 24, 25, 30, 31, 26, 48, 29, 27, 34, 41"
    )]
    pub campaign_bidding_strategy: ::core::option::Option<
        campaign::CampaignBiddingStrategy,
    >,
}
/// Nested message and enum types in `Campaign`.
pub mod campaign {
    /// The network settings for the campaign.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct NetworkSettings {
        /// Whether ads will be served with google.com search results.
        #[prost(bool, optional, tag = "5")]
        pub target_google_search: ::core::option::Option<bool>,
        /// Whether ads will be served on partner sites in the Google Search Network
        /// (requires `target_google_search` to also be `true`).
        #[prost(bool, optional, tag = "6")]
        pub target_search_network: ::core::option::Option<bool>,
        /// Whether ads will be served on specified placements in the Google Display
        /// Network. Placements are specified using the Placement criterion.
        #[prost(bool, optional, tag = "7")]
        pub target_content_network: ::core::option::Option<bool>,
        /// Whether ads will be served on the Google Partner Network.
        /// This is available only to some select Google partner accounts.
        #[prost(bool, optional, tag = "8")]
        pub target_partner_search_network: ::core::option::Option<bool>,
    }
    /// The setting for controlling Dynamic Search Ads (DSA).
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DynamicSearchAdsSetting {
        /// Required. The Internet domain name that this setting represents, for
        /// example, "google.com" or "www.google.com".
        #[prost(string, tag = "6")]
        pub domain_name: ::prost::alloc::string::String,
        /// Required. The language code specifying the language of the domain, for
        /// example, "en".
        #[prost(string, tag = "7")]
        pub language_code: ::prost::alloc::string::String,
        /// Whether the campaign uses advertiser supplied URLs exclusively.
        #[prost(bool, optional, tag = "8")]
        pub use_supplied_urls_only: ::core::option::Option<bool>,
    }
    /// The setting for Shopping campaigns. Defines the universe of products that
    /// can be advertised by the campaign, and how this campaign interacts with
    /// other Shopping campaigns.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ShoppingSetting {
        /// Immutable. ID of the Merchant Center account.
        /// This field is required for create operations. This field is immutable for
        /// Shopping campaigns.
        #[prost(int64, optional, tag = "5")]
        pub merchant_id: ::core::option::Option<i64>,
        /// Sales country of products to include in the campaign.
        ///
        #[prost(string, optional, tag = "6")]
        pub sales_country: ::core::option::Option<::prost::alloc::string::String>,
        /// Feed label of products to include in the campaign.
        /// Only one of feed_label or sales_country can be set.
        /// If used instead of sales_country, the feed_label field accepts country
        /// codes in the same format for example: 'XX'.
        /// Otherwise can be any string used for feed label in Google Merchant
        /// Center.
        #[prost(string, tag = "10")]
        pub feed_label: ::prost::alloc::string::String,
        /// Priority of the campaign. Campaigns with numerically higher priorities
        /// take precedence over those with lower priorities.
        /// This field is required for Shopping campaigns, with values between 0 and
        /// 2, inclusive.
        /// This field is optional for Smart Shopping campaigns, but must be equal to
        /// 3 if set.
        #[prost(int32, optional, tag = "7")]
        pub campaign_priority: ::core::option::Option<i32>,
        /// Whether to include local products.
        #[prost(bool, optional, tag = "8")]
        pub enable_local: ::core::option::Option<bool>,
        /// Immutable. Whether to target Vehicle Listing inventory.
        #[prost(bool, tag = "9")]
        pub use_vehicle_inventory: bool,
    }
    /// Campaign-level settings for tracking information.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TrackingSetting {
        /// Output only. The url used for dynamic tracking.
        #[prost(string, optional, tag = "2")]
        pub tracking_url: ::core::option::Option<::prost::alloc::string::String>,
    }
    /// Represents a collection of settings related to ads geotargeting.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GeoTargetTypeSetting {
        /// The setting used for positive geotargeting in this particular campaign.
        #[prost(
            enumeration = "super::super::enums::positive_geo_target_type_enum::PositiveGeoTargetType",
            tag = "1"
        )]
        pub positive_geo_target_type: i32,
        /// The setting used for negative geotargeting in this particular campaign.
        #[prost(
            enumeration = "super::super::enums::negative_geo_target_type_enum::NegativeGeoTargetType",
            tag = "2"
        )]
        pub negative_geo_target_type: i32,
    }
    /// Selective optimization setting for this campaign, which includes a set of
    /// conversion actions to optimize this campaign towards.
    /// This feature only applies to app campaigns that use MULTI_CHANNEL as
    /// AdvertisingChannelType and APP_CAMPAIGN or APP_CAMPAIGN_FOR_ENGAGEMENT as
    /// AdvertisingChannelSubType.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SelectiveOptimization {
        /// The selected set of conversion actions for optimizing this campaign.
        #[prost(string, repeated, tag = "2")]
        pub conversion_actions: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
    /// Optimization goal setting for this campaign, which includes a set of
    /// optimization goal types.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct OptimizationGoalSetting {
        /// The list of optimization goal types.
        #[prost(
            enumeration = "super::super::enums::optimization_goal_type_enum::OptimizationGoalType",
            repeated,
            tag = "1"
        )]
        pub optimization_goal_types: ::prost::alloc::vec::Vec<i32>,
    }
    /// The bidding strategy for the campaign.
    ///
    /// Must be either portfolio (created through BiddingStrategy service) or
    /// standard, that is embedded into the campaign.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum CampaignBiddingStrategy {
        /// Portfolio bidding strategy used by campaign.
        #[prost(string, tag = "67")]
        BiddingStrategy(::prost::alloc::string::String),
        /// Standard Manual CPA bidding strategy.
        /// Manual bidding strategy that allows advertiser to set the bid per
        /// advertiser-specified action. Supported only for Local Services campaigns.
        #[prost(message, tag = "74")]
        ManualCpa(super::super::common::ManualCpa),
        /// Standard Manual CPC bidding strategy.
        /// Manual click-based bidding where user pays per click.
        #[prost(message, tag = "24")]
        ManualCpc(super::super::common::ManualCpc),
        /// Standard Manual CPM bidding strategy.
        /// Manual impression-based bidding where user pays per thousand
        /// impressions.
        #[prost(message, tag = "25")]
        ManualCpm(super::super::common::ManualCpm),
        /// Standard Maximize Conversions bidding strategy that automatically
        /// maximizes number of conversions while spending your budget.
        #[prost(message, tag = "30")]
        MaximizeConversions(super::super::common::MaximizeConversions),
        /// Standard Maximize Conversion Value bidding strategy that automatically
        /// sets bids to maximize revenue while spending your budget.
        #[prost(message, tag = "31")]
        MaximizeConversionValue(super::super::common::MaximizeConversionValue),
        /// Standard Target CPA bidding strategy that automatically sets bids to
        /// help get as many conversions as possible at the target
        /// cost-per-acquisition (CPA) you set.
        #[prost(message, tag = "26")]
        TargetCpa(super::super::common::TargetCpa),
        /// Target Impression Share bidding strategy. An automated bidding strategy
        /// that sets bids to achieve a chosen percentage of impressions.
        #[prost(message, tag = "48")]
        TargetImpressionShare(super::super::common::TargetImpressionShare),
        /// Standard Target ROAS bidding strategy that automatically maximizes
        /// revenue while averaging a specific target return on ad spend (ROAS).
        #[prost(message, tag = "29")]
        TargetRoas(super::super::common::TargetRoas),
        /// Standard Target Spend bidding strategy that automatically sets your bids
        /// to help get as many clicks as possible within your budget.
        #[prost(message, tag = "27")]
        TargetSpend(super::super::common::TargetSpend),
        /// Standard Percent Cpc bidding strategy where bids are a fraction of the
        /// advertised price for some good or service.
        #[prost(message, tag = "34")]
        PercentCpc(super::super::common::PercentCpc),
        /// A bidding strategy that automatically optimizes cost per thousand
        /// impressions.
        #[prost(message, tag = "41")]
        TargetCpm(super::super::common::TargetCpm),
    }
}
/// A link between a Campaign and an Asset.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CampaignAsset {
    /// Immutable. The resource name of the campaign asset.
    /// CampaignAsset resource names have the form:
    ///
    /// `customers/{customer_id}/campaignAssets/{campaign_id}~{asset_id}~{field_type}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Immutable. The campaign to which the asset is linked.
    #[prost(string, optional, tag = "6")]
    pub campaign: ::core::option::Option<::prost::alloc::string::String>,
    /// Immutable. The asset which is linked to the campaign.
    #[prost(string, optional, tag = "7")]
    pub asset: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. Status of the campaign asset.
    #[prost(
        enumeration = "super::enums::asset_link_status_enum::AssetLinkStatus",
        tag = "5"
    )]
    pub status: i32,
}
/// CampaignAssetSet is the linkage between a campaign and an asset set.
/// Adding a CampaignAssetSet links an asset set with a campaign.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CampaignAssetSet {
    /// Immutable. The resource name of the campaign asset set.
    /// Asset set asset resource names have the form:
    ///
    /// `customers/{customer_id}/campaignAssetSets/{campaign_id}~{asset_set_id}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Immutable. The campaign to which this asset set is linked.
    #[prost(string, tag = "2")]
    pub campaign: ::prost::alloc::string::String,
    /// Immutable. The asset set which is linked to the campaign.
    #[prost(string, tag = "3")]
    pub asset_set: ::prost::alloc::string::String,
    /// Output only. The status of the campaign asset set asset. Read-only.
    #[prost(
        enumeration = "super::enums::asset_set_link_status_enum::AssetSetLinkStatus",
        tag = "4"
    )]
    pub status: i32,
}
/// A campaign audience view.
/// Includes performance data from interests and remarketing lists for Display
/// Network and YouTube Network ads, and remarketing lists for search ads (RLSA),
/// aggregated by campaign and audience criterion. This view only includes
/// audiences attached at the campaign level.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CampaignAudienceView {
    /// Output only. The resource name of the campaign audience view.
    /// Campaign audience view resource names have the form:
    ///
    /// `customers/{customer_id}/campaignAudienceViews/{campaign_id}~{criterion_id}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
}
/// A campaign budget.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CampaignBudget {
    /// Immutable. The resource name of the campaign budget.
    /// Campaign budget resource names have the form:
    ///
    /// `customers/{customer_id}/campaignBudgets/{campaign_budget_id}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// The amount of the budget, in the local currency for the account.
    /// Amount is specified in micros, where one million is equivalent to one
    /// currency unit. Monthly spend is capped at 30.4 times this amount.
    #[prost(int64, optional, tag = "21")]
    pub amount_micros: ::core::option::Option<i64>,
    /// The delivery method that determines the rate at which the campaign budget
    /// is spent.
    ///
    /// Defaults to STANDARD if unspecified in a create operation.
    #[prost(
        enumeration = "super::enums::budget_delivery_method_enum::BudgetDeliveryMethod",
        tag = "7"
    )]
    pub delivery_method: i32,
    /// Immutable. Period over which to spend the budget. Defaults to DAILY if not
    /// specified.
    #[prost(enumeration = "super::enums::budget_period_enum::BudgetPeriod", tag = "13")]
    pub period: i32,
}
/// A campaign criterion.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CampaignCriterion {
    /// Immutable. The resource name of the campaign criterion.
    /// Campaign criterion resource names have the form:
    ///
    /// `customers/{customer_id}/campaignCriteria/{campaign_id}~{criterion_id}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Output only. The ID of the criterion.
    ///
    /// This field is ignored during mutate.
    #[prost(int64, optional, tag = "38")]
    pub criterion_id: ::core::option::Option<i64>,
    /// Output only. The display name of the criterion.
    ///
    /// This field is ignored for mutates.
    #[prost(string, tag = "43")]
    pub display_name: ::prost::alloc::string::String,
    /// The modifier for the bids when the criterion matches. The modifier must be
    /// in the range: 0.1 - 10.0. Most targetable criteria types support modifiers.
    /// Use 0 to opt out of a Device type.
    #[prost(float, optional, tag = "39")]
    pub bid_modifier: ::core::option::Option<f32>,
    /// Immutable. Whether to target (`false`) or exclude (`true`) the criterion.
    #[prost(bool, optional, tag = "40")]
    pub negative: ::core::option::Option<bool>,
    /// Output only. The type of the criterion.
    #[prost(enumeration = "super::enums::criterion_type_enum::CriterionType", tag = "6")]
    pub r#type: i32,
    /// The status of the criterion.
    #[prost(
        enumeration = "super::enums::campaign_criterion_status_enum::CampaignCriterionStatus",
        tag = "35"
    )]
    pub status: i32,
    /// Output only. The datetime when this campaign criterion was last modified.
    /// The datetime is in the customer's time zone and in "yyyy-MM-dd
    /// HH:mm:ss.ssssss" format.
    #[prost(string, tag = "44")]
    pub last_modified_time: ::prost::alloc::string::String,
    /// The campaign criterion.
    ///
    /// Exactly one must be set.
    #[prost(
        oneof = "campaign_criterion::Criterion",
        tags = "8, 12, 13, 16, 17, 22, 26, 31, 34"
    )]
    pub criterion: ::core::option::Option<campaign_criterion::Criterion>,
}
/// Nested message and enum types in `CampaignCriterion`.
pub mod campaign_criterion {
    /// The campaign criterion.
    ///
    /// Exactly one must be set.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Criterion {
        /// Immutable. Keyword.
        #[prost(message, tag = "8")]
        Keyword(super::super::common::KeywordInfo),
        /// Immutable. Location.
        #[prost(message, tag = "12")]
        Location(super::super::common::LocationInfo),
        /// Immutable. Device.
        #[prost(message, tag = "13")]
        Device(super::super::common::DeviceInfo),
        /// Immutable. Age range.
        #[prost(message, tag = "16")]
        AgeRange(super::super::common::AgeRangeInfo),
        /// Immutable. Gender.
        #[prost(message, tag = "17")]
        Gender(super::super::common::GenderInfo),
        /// Immutable. User List.
        /// The Similar Audiences sunset starts May 2023. Refer to
        /// <https://ads-developers.googleblog.com/2022/11/announcing-deprecation-and-sunset-of.html>
        /// for other options.
        #[prost(message, tag = "22")]
        UserList(super::super::common::UserListInfo),
        /// Immutable. Language.
        #[prost(message, tag = "26")]
        Language(super::super::common::LanguageInfo),
        /// Immutable. Webpage.
        #[prost(message, tag = "31")]
        Webpage(super::super::common::WebpageInfo),
        /// Immutable. Location Group
        #[prost(message, tag = "34")]
        LocationGroup(super::super::common::LocationGroupInfo),
    }
}
/// Represents a relationship between a campaign and a label.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CampaignLabel {
    /// Immutable. Name of the resource.
    /// Campaign label resource names have the form:
    /// `customers/{customer_id}/campaignLabels/{campaign_id}~{label_id}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Immutable. The campaign to which the label is attached.
    #[prost(string, optional, tag = "4")]
    pub campaign: ::core::option::Option<::prost::alloc::string::String>,
    /// Immutable. The label assigned to the campaign.
    #[prost(string, optional, tag = "5")]
    pub label: ::core::option::Option<::prost::alloc::string::String>,
}
/// A conversion action.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConversionAction {
    /// Immutable. The resource name of the conversion action.
    /// Conversion action resource names have the form:
    ///
    /// `customers/{customer_id}/conversionActions/{conversion_action_id}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Output only. The ID of the conversion action.
    #[prost(int64, optional, tag = "21")]
    pub id: ::core::option::Option<i64>,
    /// The name of the conversion action.
    ///
    /// This field is required and should not be empty when creating new
    /// conversion actions.
    #[prost(string, optional, tag = "22")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. Timestamp of the Floodlight activity's creation, formatted in
    /// ISO 8601.
    #[prost(string, tag = "33")]
    pub creation_time: ::prost::alloc::string::String,
    /// The status of this conversion action for conversion event accrual.
    #[prost(
        enumeration = "super::enums::conversion_action_status_enum::ConversionActionStatus",
        tag = "4"
    )]
    pub status: i32,
    /// Immutable. The type of this conversion action.
    #[prost(
        enumeration = "super::enums::conversion_action_type_enum::ConversionActionType",
        tag = "5"
    )]
    pub r#type: i32,
    /// If a conversion action's primary_for_goal bit is false, the conversion
    /// action is non-biddable for all campaigns regardless of their customer
    /// conversion goal or campaign conversion goal.
    /// However, custom conversion goals do not respect primary_for_goal, so if
    /// a campaign has a custom conversion goal configured with a
    /// primary_for_goal = false conversion action, that conversion action is
    /// still biddable.
    /// By default, primary_for_goal will be true if not set. In V9,
    /// primary_for_goal can only be set to false after creation through an
    /// 'update' operation because it's not declared as optional.
    #[prost(bool, optional, tag = "31")]
    pub primary_for_goal: ::core::option::Option<bool>,
    /// The category of conversions reported for this conversion action.
    #[prost(
        enumeration = "super::enums::conversion_action_category_enum::ConversionActionCategory",
        tag = "6"
    )]
    pub category: i32,
    /// Output only. The resource name of the conversion action owner customer, or
    /// null if this is a system-defined conversion action.
    #[prost(string, optional, tag = "23")]
    pub owner_customer: ::core::option::Option<::prost::alloc::string::String>,
    /// Whether this conversion action should be included in the
    /// "client_account_conversions" metric.
    #[prost(bool, optional, tag = "24")]
    pub include_in_client_account_conversions_metric: ::core::option::Option<bool>,
    /// Output only. Whether this conversion action should be included in the
    /// "conversions" metric.
    #[prost(bool, optional, tag = "32")]
    pub include_in_conversions_metric: ::core::option::Option<bool>,
    /// The maximum number of days that may elapse between an interaction
    /// (for example, a click) and a conversion event.
    #[prost(int64, optional, tag = "25")]
    pub click_through_lookback_window_days: ::core::option::Option<i64>,
    /// Settings related to the value for conversion events associated with this
    /// conversion action.
    #[prost(message, optional, tag = "11")]
    pub value_settings: ::core::option::Option<conversion_action::ValueSettings>,
    /// Settings related to this conversion action's attribution model.
    #[prost(message, optional, tag = "13")]
    pub attribution_model_settings: ::core::option::Option<
        conversion_action::AttributionModelSettings,
    >,
    /// App ID for an app conversion action.
    #[prost(string, optional, tag = "28")]
    pub app_id: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. Floodlight settings for Floodlight conversion types.
    #[prost(message, optional, tag = "29")]
    pub floodlight_settings: ::core::option::Option<
        conversion_action::FloodlightSettings,
    >,
}
/// Nested message and enum types in `ConversionAction`.
pub mod conversion_action {
    /// Settings related to this conversion action's attribution model.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct AttributionModelSettings {
        /// The attribution model type of this conversion action.
        #[prost(
            enumeration = "super::super::enums::attribution_model_enum::AttributionModel",
            tag = "1"
        )]
        pub attribution_model: i32,
        /// Output only. The status of the data-driven attribution model for the
        /// conversion action.
        #[prost(
            enumeration = "super::super::enums::data_driven_model_status_enum::DataDrivenModelStatus",
            tag = "2"
        )]
        pub data_driven_model_status: i32,
    }
    /// Settings related to the value for conversion events associated with this
    /// conversion action.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ValueSettings {
        /// The value to use when conversion events for this conversion action are
        /// sent with an invalid, disallowed or missing value, or when
        /// this conversion action is configured to always use the default value.
        #[prost(double, optional, tag = "4")]
        pub default_value: ::core::option::Option<f64>,
        /// The currency code to use when conversion events for this conversion
        /// action are sent with an invalid or missing currency code, or when this
        /// conversion action is configured to always use the default value.
        #[prost(string, optional, tag = "5")]
        pub default_currency_code: ::core::option::Option<
            ::prost::alloc::string::String,
        >,
        /// Controls whether the default value and default currency code are used in
        /// place of the value and currency code specified in conversion events for
        /// this conversion action.
        #[prost(bool, optional, tag = "6")]
        pub always_use_default_value: ::core::option::Option<bool>,
    }
    /// Settings related to a Floodlight conversion action.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct FloodlightSettings {
        /// Output only. String used to identify a Floodlight activity group when
        /// reporting conversions.
        #[prost(string, tag = "1")]
        pub activity_group_tag: ::prost::alloc::string::String,
        /// Output only. String used to identify a Floodlight activity when reporting
        /// conversions.
        #[prost(string, tag = "2")]
        pub activity_tag: ::prost::alloc::string::String,
        /// Output only. ID of the Floodlight activity in DoubleClick Campaign
        /// Manager (DCM).
        #[prost(int64, tag = "3")]
        pub activity_id: i64,
    }
}
/// A custom column.
/// See Search Ads 360 custom column at
/// <https://support.google.com/sa360/answer/9633916>
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomColumn {
    /// Immutable. The resource name of the custom column.
    /// Custom column resource names have the form:
    ///
    /// `customers/{customer_id}/customColumns/{custom_column_id}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Output only. ID of the custom column.
    #[prost(int64, tag = "2")]
    pub id: i64,
    /// Output only. User-defined name of the custom column.
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
    /// Output only. User-defined description of the custom column.
    #[prost(string, tag = "4")]
    pub description: ::prost::alloc::string::String,
    /// Output only. The type of the result value of the custom column.
    #[prost(
        enumeration = "super::enums::custom_column_value_type_enum::CustomColumnValueType",
        tag = "5"
    )]
    pub value_type: i32,
    /// Output only. True when the custom column is referring to one or more
    /// attributes.
    #[prost(bool, tag = "6")]
    pub references_attributes: bool,
    /// Output only. True when the custom column is referring to one or more
    /// metrics.
    #[prost(bool, tag = "7")]
    pub references_metrics: bool,
    /// Output only. True when the custom column is available to be used in the
    /// query of SearchAds360Service.Search and SearchAds360Service.SearchStream.
    #[prost(bool, tag = "8")]
    pub queryable: bool,
    /// Output only. The list of the referenced system columns of this custom
    /// column. For example, A custom column "sum of impressions and clicks" has
    /// referenced system columns of {"metrics.clicks", "metrics.impressions"}.
    #[prost(string, repeated, tag = "9")]
    pub referenced_system_columns: ::prost::alloc::vec::Vec<
        ::prost::alloc::string::String,
    >,
}
/// A customer.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Customer {
    /// Immutable. The resource name of the customer.
    /// Customer resource names have the form:
    ///
    /// `customers/{customer_id}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Output only. The ID of the customer.
    #[prost(int64, optional, tag = "19")]
    pub id: ::core::option::Option<i64>,
    /// Optional, non-unique descriptive name of the customer.
    #[prost(string, optional, tag = "20")]
    pub descriptive_name: ::core::option::Option<::prost::alloc::string::String>,
    /// Immutable. The currency in which the account operates.
    /// A subset of the currency codes from the ISO 4217 standard is
    /// supported.
    #[prost(string, optional, tag = "21")]
    pub currency_code: ::core::option::Option<::prost::alloc::string::String>,
    /// Immutable. The local timezone ID of the customer.
    #[prost(string, optional, tag = "22")]
    pub time_zone: ::core::option::Option<::prost::alloc::string::String>,
    /// The URL template for constructing a tracking URL out of parameters.
    #[prost(string, optional, tag = "23")]
    pub tracking_url_template: ::core::option::Option<::prost::alloc::string::String>,
    /// The URL template for appending params to the final URL.
    #[prost(string, optional, tag = "24")]
    pub final_url_suffix: ::core::option::Option<::prost::alloc::string::String>,
    /// Whether auto-tagging is enabled for the customer.
    #[prost(bool, optional, tag = "25")]
    pub auto_tagging_enabled: ::core::option::Option<bool>,
    /// Output only. Whether the customer is a manager.
    #[prost(bool, optional, tag = "27")]
    pub manager: ::core::option::Option<bool>,
    /// Output only. Conversion tracking setting for a customer.
    #[prost(message, optional, tag = "14")]
    pub conversion_tracking_setting: ::core::option::Option<ConversionTrackingSetting>,
    /// Output only. Engine account type, for example, Google Ads, Microsoft
    /// Advertising, Yahoo Japan, Baidu, Facebook, Engine Track, etc.
    #[prost(enumeration = "super::enums::account_type_enum::AccountType", tag = "31")]
    pub account_type: i32,
    /// Output only. DoubleClick Campaign Manager (DCM) setting for a manager
    /// customer.
    #[prost(message, optional, tag = "32")]
    pub double_click_campaign_manager_setting: ::core::option::Option<
        DoubleClickCampaignManagerSetting,
    >,
    /// Output only. Account status, for example, Enabled, Paused, Removed, etc.
    #[prost(
        enumeration = "super::enums::account_status_enum::AccountStatus",
        tag = "33"
    )]
    pub account_status: i32,
    /// Output only. The datetime when this customer was last modified. The
    /// datetime is in the customer's time zone and in "yyyy-MM-dd HH:mm:ss.ssssss"
    /// format.
    #[prost(string, tag = "34")]
    pub last_modified_time: ::prost::alloc::string::String,
    /// Output only. ID of the account in the external engine account.
    #[prost(string, tag = "35")]
    pub engine_id: ::prost::alloc::string::String,
    /// Output only. The status of the customer.
    #[prost(
        enumeration = "super::enums::customer_status_enum::CustomerStatus",
        tag = "36"
    )]
    pub status: i32,
    /// Output only. The timestamp when this customer was created. The timestamp is
    /// in the customer's time zone and in "yyyy-MM-dd HH:mm:ss" format.
    #[prost(string, tag = "42")]
    pub creation_time: ::prost::alloc::string::String,
}
/// A collection of customer-wide settings related to Search Ads 360 Conversion
/// Tracking.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConversionTrackingSetting {
    /// Output only. The conversion tracking id used for this account. This id
    /// doesn't indicate whether the customer uses conversion tracking
    /// (conversion_tracking_status does). This field is read-only.
    #[prost(int64, optional, tag = "3")]
    pub conversion_tracking_id: ::core::option::Option<i64>,
    /// Output only. The conversion tracking id of the customer's manager. This is
    /// set when the customer is opted into  conversion tracking, and it overrides
    /// conversion_tracking_id. This field can only be managed through the Google
    /// Ads UI. This field is read-only.
    #[prost(int64, optional, tag = "4")]
    pub google_ads_cross_account_conversion_tracking_id: ::core::option::Option<i64>,
    /// Output only. The conversion tracking id of the customer's manager. This is
    /// set when the customer is opted into cross-account conversion tracking, and
    /// it overrides conversion_tracking_id.
    #[prost(int64, optional, tag = "37")]
    pub cross_account_conversion_tracking_id: ::core::option::Option<i64>,
    /// Output only. Whether the customer has accepted customer data terms. If
    /// using cross-account conversion tracking, this value is inherited from the
    /// manager. This field is read-only. For more
    /// information, see <https://support.google.com/adspolicy/answer/7475709.>
    #[prost(bool, tag = "5")]
    pub accepted_customer_data_terms: bool,
    /// Output only. Conversion tracking status. It indicates whether the customer
    /// is using conversion tracking, and who is the conversion tracking owner of
    /// this customer. If this customer is using cross-account conversion tracking,
    /// the value returned will differ based on the `login-customer-id` of the
    /// request.
    #[prost(
        enumeration = "super::enums::conversion_tracking_status_enum::ConversionTrackingStatus",
        tag = "6"
    )]
    pub conversion_tracking_status: i32,
    /// Output only. Whether the customer is opted-in for enhanced conversions
    /// for leads. If using cross-account conversion tracking, this value is
    /// inherited from the manager. This field is read-only.
    #[prost(bool, tag = "7")]
    pub enhanced_conversions_for_leads_enabled: bool,
    /// Output only. The resource name of the customer where conversions are
    /// created and managed. This field is read-only.
    #[prost(string, tag = "8")]
    pub google_ads_conversion_customer: ::prost::alloc::string::String,
}
/// DoubleClick Campaign Manager (DCM) setting for a manager customer.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DoubleClickCampaignManagerSetting {
    /// Output only. ID of the Campaign Manager advertiser associated with this
    /// customer.
    #[prost(int64, tag = "1")]
    pub advertiser_id: i64,
    /// Output only. ID of the Campaign Manager network associated with this
    /// customer.
    #[prost(int64, tag = "2")]
    pub network_id: i64,
    /// Output only. Time zone of the Campaign Manager network associated with this
    /// customer in IANA Time Zone Database format, such as America/New_York.
    #[prost(string, tag = "3")]
    pub time_zone: ::prost::alloc::string::String,
}
/// A link between a customer and an asset.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomerAsset {
    /// Immutable. The resource name of the customer asset.
    /// CustomerAsset resource names have the form:
    ///
    /// `customers/{customer_id}/customerAssets/{asset_id}~{field_type}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Required. Immutable. The asset which is linked to the customer.
    #[prost(string, tag = "2")]
    pub asset: ::prost::alloc::string::String,
    /// Status of the customer asset.
    #[prost(
        enumeration = "super::enums::asset_link_status_enum::AssetLinkStatus",
        tag = "4"
    )]
    pub status: i32,
}
/// CustomerAssetSet is the linkage between a customer and an asset set.
/// Adding a CustomerAssetSet links an asset set with a customer.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomerAssetSet {
    /// Immutable. The resource name of the customer asset set.
    /// Asset set asset resource names have the form:
    ///
    /// `customers/{customer_id}/customerAssetSets/{asset_set_id}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Immutable. The asset set which is linked to the customer.
    #[prost(string, tag = "2")]
    pub asset_set: ::prost::alloc::string::String,
    /// Immutable. The customer to which this asset set is linked.
    #[prost(string, tag = "3")]
    pub customer: ::prost::alloc::string::String,
    /// Output only. The status of the customer asset set asset. Read-only.
    #[prost(
        enumeration = "super::enums::asset_set_link_status_enum::AssetSetLinkStatus",
        tag = "4"
    )]
    pub status: i32,
}
/// A link between the given customer and a client customer. CustomerClients only
/// exist for manager customers. All direct and indirect client customers are
/// included, as well as the manager itself.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomerClient {
    /// Output only. The resource name of the customer client.
    /// CustomerClient resource names have the form:
    /// `customers/{customer_id}/customerClients/{client_customer_id}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Output only. The resource name of the client-customer which is linked to
    /// the given customer. Read only.
    #[prost(string, optional, tag = "12")]
    pub client_customer: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. Specifies whether this is a hidden account.
    ///
    /// Read only.
    #[prost(bool, optional, tag = "13")]
    pub hidden: ::core::option::Option<bool>,
    /// Output only. Distance between given customer and client. For self link, the
    /// level value will be 0. Read only.
    #[prost(int64, optional, tag = "14")]
    pub level: ::core::option::Option<i64>,
    /// Output only. Common Locale Data Repository (CLDR) string representation of
    /// the time zone of the client, for example, America/Los_Angeles. Read only.
    #[prost(string, optional, tag = "15")]
    pub time_zone: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. Identifies if the client is a test account. Read only.
    #[prost(bool, optional, tag = "16")]
    pub test_account: ::core::option::Option<bool>,
    /// Output only. Identifies if the client is a manager. Read only.
    #[prost(bool, optional, tag = "17")]
    pub manager: ::core::option::Option<bool>,
    /// Output only. Descriptive name for the client. Read only.
    #[prost(string, optional, tag = "18")]
    pub descriptive_name: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. Currency code (for example, 'USD', 'EUR') for the client. Read
    /// only.
    #[prost(string, optional, tag = "19")]
    pub currency_code: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. The ID of the client customer. Read only.
    #[prost(int64, optional, tag = "20")]
    pub id: ::core::option::Option<i64>,
    /// Output only. The resource names of the labels owned by the requesting
    /// customer that are applied to the client customer. Label resource names have
    /// the form:
    ///
    /// `customers/{customer_id}/labels/{label_id}`
    #[prost(string, repeated, tag = "21")]
    pub applied_labels: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Output only. The status of the client customer. Read only.
    #[prost(
        enumeration = "super::enums::customer_status_enum::CustomerStatus",
        tag = "22"
    )]
    pub status: i32,
}
/// Represents customer-manager link relationship.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomerManagerLink {
    /// Immutable. Name of the resource.
    /// CustomerManagerLink resource names have the form:
    /// `customers/{customer_id}/customerManagerLinks/{manager_customer_id}~{manager_link_id}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Output only. The manager customer linked to the customer.
    #[prost(string, optional, tag = "6")]
    pub manager_customer: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. ID of the customer-manager link. This field is read only.
    #[prost(int64, optional, tag = "7")]
    pub manager_link_id: ::core::option::Option<i64>,
    /// Status of the link between the customer and the manager.
    #[prost(
        enumeration = "super::enums::manager_link_status_enum::ManagerLinkStatus",
        tag = "5"
    )]
    pub status: i32,
}
/// A dynamic search ads search term view.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DynamicSearchAdsSearchTermView {
    /// Output only. The resource name of the dynamic search ads search term view.
    /// Dynamic search ads search term view resource names have the form:
    ///
    /// `customers/{customer_id}/dynamicSearchAdsSearchTermViews/{ad_group_id}~{search_term_fingerprint}~{headline_fingerprint}~{landing_page_fingerprint}~{page_url_fingerprint}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Output only. The dynamically selected landing page URL of the impression.
    ///
    /// This field is read-only.
    #[prost(string, optional, tag = "11")]
    pub landing_page: ::core::option::Option<::prost::alloc::string::String>,
}
/// A gender view.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenderView {
    /// Output only. The resource name of the gender view.
    /// Gender view resource names have the form:
    ///
    /// `customers/{customer_id}/genderViews/{ad_group_id}~{criterion_id}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
}
/// A geo target constant.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GeoTargetConstant {
    /// Output only. The resource name of the geo target constant.
    /// Geo target constant resource names have the form:
    ///
    /// `geoTargetConstants/{geo_target_constant_id}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Output only. The ID of the geo target constant.
    #[prost(int64, optional, tag = "10")]
    pub id: ::core::option::Option<i64>,
    /// Output only. Geo target constant English name.
    #[prost(string, optional, tag = "11")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. The ISO-3166-1 alpha-2 country code that is associated with
    /// the target.
    #[prost(string, optional, tag = "12")]
    pub country_code: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. Geo target constant target type.
    #[prost(string, optional, tag = "13")]
    pub target_type: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. Geo target constant status.
    #[prost(
        enumeration = "super::enums::geo_target_constant_status_enum::GeoTargetConstantStatus",
        tag = "7"
    )]
    pub status: i32,
    /// Output only. The fully qualified English name, consisting of the target's
    /// name and that of its parent and country.
    #[prost(string, optional, tag = "14")]
    pub canonical_name: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. The resource name of the parent geo target constant.
    /// Geo target constant resource names have the form:
    ///
    /// `geoTargetConstants/{parent_geo_target_constant_id}`
    #[prost(string, optional, tag = "9")]
    pub parent_geo_target: ::core::option::Option<::prost::alloc::string::String>,
}
/// A keyword view.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeywordView {
    /// Output only. The resource name of the keyword view.
    /// Keyword view resource names have the form:
    ///
    /// `customers/{customer_id}/keywordViews/{ad_group_id}~{criterion_id}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
}
/// A label.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Label {
    /// Immutable. Name of the resource.
    /// Label resource names have the form:
    /// `customers/{customer_id}/labels/{label_id}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Output only. ID of the label. Read only.
    #[prost(int64, optional, tag = "6")]
    pub id: ::core::option::Option<i64>,
    /// The name of the label.
    ///
    /// This field is required and should not be empty when creating a new label.
    ///
    /// The length of this string should be between 1 and 80, inclusive.
    #[prost(string, optional, tag = "7")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. Status of the label. Read only.
    #[prost(enumeration = "super::enums::label_status_enum::LabelStatus", tag = "4")]
    pub status: i32,
    /// A type of label displaying text on a colored background.
    #[prost(message, optional, tag = "5")]
    pub text_label: ::core::option::Option<super::common::TextLabel>,
}
/// A location view summarizes the performance of campaigns by
/// Location criteria.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LocationView {
    /// Output only. The resource name of the location view.
    /// Location view resource names have the form:
    ///
    /// `customers/{customer_id}/locationViews/{campaign_id}~{criterion_id}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
}
/// A product group view.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProductGroupView {
    /// Output only. The resource name of the product group view.
    /// Product group view resource names have the form:
    ///
    /// `customers/{customer_id}/productGroupViews/{ad_group_id}~{criterion_id}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
}
/// A field or resource (artifact) used by SearchAds360Service.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchAds360Field {
    /// Output only. The resource name of the artifact.
    /// Artifact resource names have the form:
    ///
    /// `SearchAds360Fields/{name}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Output only. The name of the artifact.
    #[prost(string, optional, tag = "21")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. The category of the artifact.
    #[prost(
        enumeration = "super::enums::search_ads360_field_category_enum::SearchAds360FieldCategory",
        tag = "3"
    )]
    pub category: i32,
    /// Output only. Whether the artifact can be used in a SELECT clause in search
    /// queries.
    #[prost(bool, optional, tag = "22")]
    pub selectable: ::core::option::Option<bool>,
    /// Output only. Whether the artifact can be used in a WHERE clause in search
    /// queries.
    #[prost(bool, optional, tag = "23")]
    pub filterable: ::core::option::Option<bool>,
    /// Output only. Whether the artifact can be used in a ORDER BY clause in
    /// search queries.
    #[prost(bool, optional, tag = "24")]
    pub sortable: ::core::option::Option<bool>,
    /// Output only. The names of all resources, segments, and metrics that are
    /// selectable with the described artifact.
    #[prost(string, repeated, tag = "25")]
    pub selectable_with: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Output only. The names of all resources that are selectable with the
    /// described artifact. Fields from these resources do not segment metrics when
    /// included in search queries.
    ///
    /// This field is only set for artifacts whose category is RESOURCE.
    #[prost(string, repeated, tag = "26")]
    pub attribute_resources: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Output only. This field lists the names of all metrics that are selectable
    /// with the described artifact when it is used in the FROM clause. It is only
    /// set for artifacts whose category is RESOURCE.
    #[prost(string, repeated, tag = "27")]
    pub metrics: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Output only. This field lists the names of all artifacts, whether a segment
    /// or another resource, that segment metrics when included in search queries
    /// and when the described artifact is used in the FROM clause. It is only set
    /// for artifacts whose category is RESOURCE.
    #[prost(string, repeated, tag = "28")]
    pub segments: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Output only. Values the artifact can assume if it is a field of type ENUM.
    ///
    /// This field is only set for artifacts of category SEGMENT or ATTRIBUTE.
    #[prost(string, repeated, tag = "29")]
    pub enum_values: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Output only. This field determines the operators that can be used with the
    /// artifact in WHERE clauses.
    #[prost(
        enumeration = "super::enums::search_ads360_field_data_type_enum::SearchAds360FieldDataType",
        tag = "12"
    )]
    pub data_type: i32,
    /// Output only. The URL of proto describing the artifact's data type.
    #[prost(string, optional, tag = "30")]
    pub type_url: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. Whether the field artifact is repeated.
    #[prost(bool, optional, tag = "31")]
    pub is_repeated: ::core::option::Option<bool>,
}
/// A user list. This is a list of users a customer may target.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserList {
    /// Immutable. The resource name of the user list.
    /// User list resource names have the form:
    ///
    /// `customers/{customer_id}/userLists/{user_list_id}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
    /// Output only. Id of the user list.
    #[prost(int64, optional, tag = "25")]
    pub id: ::core::option::Option<i64>,
    /// Name of this user list. Depending on its access_reason, the user list name
    /// may not be unique (for example, if access_reason=SHARED)
    #[prost(string, optional, tag = "27")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    /// Output only. Type of this list.
    ///
    /// This field is read-only.
    #[prost(enumeration = "super::enums::user_list_type_enum::UserListType", tag = "13")]
    pub r#type: i32,
}
/// A webpage view.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WebpageView {
    /// Output only. The resource name of the webpage view.
    /// Webpage view resource names have the form:
    ///
    /// `customers/{customer_id}/webpageViews/{ad_group_id}~{criterion_id}`
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
}
