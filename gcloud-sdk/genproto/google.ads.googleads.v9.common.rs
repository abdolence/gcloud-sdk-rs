// Proto file describing policy information.

/// Key of the violation. The key is used for referring to a violation
/// when filing an exemption request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PolicyViolationKey {
    /// Unique ID of the violated policy.
    #[prost(string, optional, tag="3")]
    pub policy_name: ::core::option::Option<::prost::alloc::string::String>,
    /// The text that violates the policy if specified.
    /// Otherwise, refers to the policy in general
    /// (e.g., when requesting to be exempt from the whole policy).
    /// If not specified for criterion exemptions, the whole policy is implied.
    /// Must be specified for ad exemptions.
    #[prost(string, optional, tag="4")]
    pub violating_text: ::core::option::Option<::prost::alloc::string::String>,
}
/// Parameter for controlling how policy exemption is done.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PolicyValidationParameter {
    /// The list of policy topics that should not cause a PolicyFindingError to
    /// be reported. This field is currently only compatible with Enhanced Text Ad.
    /// It corresponds to the PolicyTopicEntry.topic field.
    ///
    /// Resources violating these policies will be saved, but will not be eligible
    /// to serve. They may begin serving at a later time due to a change in
    /// policies, re-review of the resource, or a change in advertiser
    /// certificates.
    #[prost(string, repeated, tag="3")]
    pub ignorable_policy_topics: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The list of policy violation keys that should not cause a
    /// PolicyViolationError to be reported. Not all policy violations are
    /// exemptable, please refer to the is_exemptible field in the returned
    /// PolicyViolationError.
    ///
    /// Resources violating these polices will be saved, but will not be eligible
    /// to serve. They may begin serving at a later time due to a change in
    /// policies, re-review of the resource, or a change in advertiser
    /// certificates.
    #[prost(message, repeated, tag="2")]
    pub exempt_policy_violation_keys: ::prost::alloc::vec::Vec<PolicyViolationKey>,
}
/// Policy finding attached to a resource (e.g. alcohol policy associated with
/// a site that sells alcohol).
///
/// Each PolicyTopicEntry has a topic that indicates the specific ads policy
/// the entry is about and a type to indicate the effect that the entry will have
/// on serving. It may optionally have one or more evidences that indicate the
/// reason for the finding. It may also optionally have one or more constraints
/// that provide details about how serving may be restricted.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PolicyTopicEntry {
    /// Policy topic this finding refers to. For example, "ALCOHOL",
    /// "TRADEMARKS_IN_AD_TEXT", or "DESTINATION_NOT_WORKING". The set of possible
    /// policy topics is not fixed for a particular API version and may change
    /// at any time.
    #[prost(string, optional, tag="5")]
    pub topic: ::core::option::Option<::prost::alloc::string::String>,
    /// Describes the negative or positive effect this policy will have on serving.
    #[prost(enumeration="super::enums::policy_topic_entry_type_enum::PolicyTopicEntryType", tag="2")]
    pub r#type: i32,
    /// Additional information that explains policy finding
    /// (e.g. the brand name for a trademark finding).
    #[prost(message, repeated, tag="3")]
    pub evidences: ::prost::alloc::vec::Vec<PolicyTopicEvidence>,
    /// Indicates how serving of this resource may be affected (e.g. not serving
    /// in a country).
    #[prost(message, repeated, tag="4")]
    pub constraints: ::prost::alloc::vec::Vec<PolicyTopicConstraint>,
}
/// Additional information that explains a policy finding.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PolicyTopicEvidence {
    /// Specific evidence information depending on the evidence type.
    #[prost(oneof="policy_topic_evidence::Value", tags="3, 4, 9, 6, 7, 8")]
    pub value: ::core::option::Option<policy_topic_evidence::Value>,
}
/// Nested message and enum types in `PolicyTopicEvidence`.
pub mod policy_topic_evidence {
    /// A list of fragments of text that violated a policy.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TextList {
        /// The fragments of text from the resource that caused the policy finding.
        #[prost(string, repeated, tag="2")]
        pub texts: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
    /// A list of websites that caused a policy finding. Used for
    /// ONE_WEBSITE_PER_AD_GROUP policy topic, for example. In case there are more
    /// than five websites, only the top five (those that appear in resources the
    /// most) will be listed here.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct WebsiteList {
        /// Websites that caused the policy finding.
        #[prost(string, repeated, tag="2")]
        pub websites: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
    /// A list of strings found in a destination page that caused a policy
    /// finding.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DestinationTextList {
        /// List of text found in the resource's destination page.
        #[prost(string, repeated, tag="2")]
        pub destination_texts: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
    /// Evidence of mismatches between the URLs of a resource.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DestinationMismatch {
        /// The set of URLs that did not match each other.
        #[prost(enumeration="super::super::enums::policy_topic_evidence_destination_mismatch_url_type_enum::PolicyTopicEvidenceDestinationMismatchUrlType", repeated, tag="1")]
        pub url_types: ::prost::alloc::vec::Vec<i32>,
    }
    /// Evidence details when the destination is returning an HTTP error
    /// code or isn't functional in all locations for commonly used devices.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DestinationNotWorking {
        /// The full URL that didn't work.
        #[prost(string, optional, tag="7")]
        pub expanded_url: ::core::option::Option<::prost::alloc::string::String>,
        /// The type of device that failed to load the URL.
        #[prost(enumeration="super::super::enums::policy_topic_evidence_destination_not_working_device_enum::PolicyTopicEvidenceDestinationNotWorkingDevice", tag="4")]
        pub device: i32,
        /// The time the URL was last checked.
        /// The format is "YYYY-MM-DD HH:MM:SS".
        /// Examples: "2018-03-05 09:15:00" or "2018-02-01 14:34:30"
        #[prost(string, optional, tag="8")]
        pub last_checked_date_time: ::core::option::Option<::prost::alloc::string::String>,
        /// Indicates the reason of the DESTINATION_NOT_WORKING policy finding.
        #[prost(oneof="destination_not_working::Reason", tags="1, 6")]
        pub reason: ::core::option::Option<destination_not_working::Reason>,
    }
    /// Nested message and enum types in `DestinationNotWorking`.
    pub mod destination_not_working {
        /// Indicates the reason of the DESTINATION_NOT_WORKING policy finding.
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Reason {
            /// The type of DNS error.
            #[prost(enumeration="super::super::super::enums::policy_topic_evidence_destination_not_working_dns_error_type_enum::PolicyTopicEvidenceDestinationNotWorkingDnsErrorType", tag="1")]
            DnsErrorType(i32),
            /// The HTTP error code.
            #[prost(int64, tag="6")]
            HttpErrorCode(i64),
        }
    }
    /// Specific evidence information depending on the evidence type.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Value {
        /// List of websites linked with this resource.
        #[prost(message, tag="3")]
        WebsiteList(WebsiteList),
        /// List of evidence found in the text of a resource.
        #[prost(message, tag="4")]
        TextList(TextList),
        /// The language the resource was detected to be written in.
        /// This is an IETF language tag such as "en-US".
        #[prost(string, tag="9")]
        LanguageCode(::prost::alloc::string::String),
        /// The text in the destination of the resource that is causing a policy
        /// finding.
        #[prost(message, tag="6")]
        DestinationTextList(DestinationTextList),
        /// Mismatch between the destinations of a resource's URLs.
        #[prost(message, tag="7")]
        DestinationMismatch(DestinationMismatch),
        /// Details when the destination is returning an HTTP error code or isn't
        /// functional in all locations for commonly used devices.
        #[prost(message, tag="8")]
        DestinationNotWorking(DestinationNotWorking),
    }
}
/// Describes the effect on serving that a policy topic entry will have.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PolicyTopicConstraint {
    /// Specific information about the constraint.
    #[prost(oneof="policy_topic_constraint::Value", tags="1, 2, 3, 4")]
    pub value: ::core::option::Option<policy_topic_constraint::Value>,
}
/// Nested message and enum types in `PolicyTopicConstraint`.
pub mod policy_topic_constraint {
    /// A list of countries where a resource's serving is constrained.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CountryConstraintList {
        /// Total number of countries targeted by the resource.
        #[prost(int32, optional, tag="3")]
        pub total_targeted_countries: ::core::option::Option<i32>,
        /// Countries in which serving is restricted.
        #[prost(message, repeated, tag="2")]
        pub countries: ::prost::alloc::vec::Vec<CountryConstraint>,
    }
    /// Indicates that a policy topic was constrained due to disapproval of the
    /// website for reseller purposes.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ResellerConstraint {
    }
    /// Indicates that a resource's ability to serve in a particular country is
    /// constrained.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CountryConstraint {
        /// Geo target constant resource name of the country in which serving is
        /// constrained.
        #[prost(string, optional, tag="2")]
        pub country_criterion: ::core::option::Option<::prost::alloc::string::String>,
    }
    /// Specific information about the constraint.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Value {
        /// Countries where the resource cannot serve.
        #[prost(message, tag="1")]
        CountryConstraintList(CountryConstraintList),
        /// Reseller constraint.
        #[prost(message, tag="2")]
        ResellerConstraint(ResellerConstraint),
        /// Countries where a certificate is required for serving.
        #[prost(message, tag="3")]
        CertificateMissingInCountryList(CountryConstraintList),
        /// Countries where the resource's domain is not covered by the
        /// certificates associated with it.
        #[prost(message, tag="4")]
        CertificateDomainMismatchInCountryList(CountryConstraintList),
    }
}
// Proto file describing asset policies.

/// Contains policy information for an asset inside an ad.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdAssetPolicySummary {
    /// The list of policy findings for this asset.
    #[prost(message, repeated, tag="1")]
    pub policy_topic_entries: ::prost::alloc::vec::Vec<PolicyTopicEntry>,
    /// Where in the review process this asset.
    #[prost(enumeration="super::enums::policy_review_status_enum::PolicyReviewStatus", tag="2")]
    pub review_status: i32,
    /// The overall approval status of this asset, which is calculated based on
    /// the status of its individual policy topic entries.
    #[prost(enumeration="super::enums::policy_approval_status_enum::PolicyApprovalStatus", tag="3")]
    pub approval_status: i32,
}
// Proto file describing assets used inside an ad.

/// A text asset used inside an ad.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdTextAsset {
    /// Asset text.
    #[prost(string, optional, tag="4")]
    pub text: ::core::option::Option<::prost::alloc::string::String>,
    /// The pinned field of the asset. This restricts the asset to only serve
    /// within this field. Multiple assets can be pinned to the same field. An
    /// asset that is unpinned or pinned to a different field will not serve in a
    /// field where some other asset has been pinned.
    #[prost(enumeration="super::enums::served_asset_field_type_enum::ServedAssetFieldType", tag="2")]
    pub pinned_field: i32,
    /// The performance label of this text asset.
    #[prost(enumeration="super::enums::asset_performance_label_enum::AssetPerformanceLabel", tag="5")]
    pub asset_performance_label: i32,
    /// The policy summary of this text asset.
    #[prost(message, optional, tag="6")]
    pub policy_summary_info: ::core::option::Option<AdAssetPolicySummary>,
}
/// An image asset used inside an ad.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdImageAsset {
    /// The Asset resource name of this image.
    #[prost(string, optional, tag="2")]
    pub asset: ::core::option::Option<::prost::alloc::string::String>,
}
/// A video asset used inside an ad.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdVideoAsset {
    /// The Asset resource name of this video.
    #[prost(string, optional, tag="2")]
    pub asset: ::core::option::Option<::prost::alloc::string::String>,
}
/// A media bundle asset used inside an ad.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdMediaBundleAsset {
    /// The Asset resource name of this media bundle.
    #[prost(string, optional, tag="2")]
    pub asset: ::core::option::Option<::prost::alloc::string::String>,
}
// Proto file containing info messages for specific ad types.

/// A text ad.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TextAdInfo {
    /// The headline of the ad.
    #[prost(string, optional, tag="4")]
    pub headline: ::core::option::Option<::prost::alloc::string::String>,
    /// The first line of the ad's description.
    #[prost(string, optional, tag="5")]
    pub description1: ::core::option::Option<::prost::alloc::string::String>,
    /// The second line of the ad's description.
    #[prost(string, optional, tag="6")]
    pub description2: ::core::option::Option<::prost::alloc::string::String>,
}
/// An expanded text ad.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExpandedTextAdInfo {
    /// The first part of the ad's headline.
    #[prost(string, optional, tag="8")]
    pub headline_part1: ::core::option::Option<::prost::alloc::string::String>,
    /// The second part of the ad's headline.
    #[prost(string, optional, tag="9")]
    pub headline_part2: ::core::option::Option<::prost::alloc::string::String>,
    /// The third part of the ad's headline.
    #[prost(string, optional, tag="10")]
    pub headline_part3: ::core::option::Option<::prost::alloc::string::String>,
    /// The description of the ad.
    #[prost(string, optional, tag="11")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
    /// The second description of the ad.
    #[prost(string, optional, tag="12")]
    pub description2: ::core::option::Option<::prost::alloc::string::String>,
    /// The text that can appear alongside the ad's displayed URL.
    #[prost(string, optional, tag="13")]
    pub path1: ::core::option::Option<::prost::alloc::string::String>,
    /// Additional text that can appear alongside the ad's displayed URL.
    #[prost(string, optional, tag="14")]
    pub path2: ::core::option::Option<::prost::alloc::string::String>,
}
/// An expanded dynamic search ad.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExpandedDynamicSearchAdInfo {
    /// The description of the ad.
    #[prost(string, optional, tag="3")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
    /// The second description of the ad.
    #[prost(string, optional, tag="4")]
    pub description2: ::core::option::Option<::prost::alloc::string::String>,
}
/// A hotel ad.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HotelAdInfo {
}
/// A Smart Shopping ad.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShoppingSmartAdInfo {
}
/// A standard Shopping ad.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShoppingProductAdInfo {
}
/// A Shopping Comparison Listing ad.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShoppingComparisonListingAdInfo {
    /// Headline of the ad. This field is required. Allowed length is between 25
    /// and 45 characters.
    #[prost(string, optional, tag="2")]
    pub headline: ::core::option::Option<::prost::alloc::string::String>,
}
/// A Gmail ad.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GmailAdInfo {
    /// The Gmail teaser.
    #[prost(message, optional, tag="1")]
    pub teaser: ::core::option::Option<GmailTeaser>,
    /// The MediaFile resource name of the header image. Valid image types are GIF,
    /// JPEG and PNG. The minimum size is 300x100 pixels and the aspect ratio must
    /// be between 3:1 and 5:1 (+-1%).
    #[prost(string, optional, tag="10")]
    pub header_image: ::core::option::Option<::prost::alloc::string::String>,
    /// The MediaFile resource name of the marketing image. Valid image types are
    /// GIF, JPEG and PNG. The image must either be landscape with a minimum size
    /// of 600x314 pixels and aspect ratio of 600:314 (+-1%) or square with a
    /// minimum size of 300x300 pixels and aspect ratio of 1:1 (+-1%)
    #[prost(string, optional, tag="11")]
    pub marketing_image: ::core::option::Option<::prost::alloc::string::String>,
    /// Headline of the marketing image.
    #[prost(string, optional, tag="12")]
    pub marketing_image_headline: ::core::option::Option<::prost::alloc::string::String>,
    /// Description of the marketing image.
    #[prost(string, optional, tag="13")]
    pub marketing_image_description: ::core::option::Option<::prost::alloc::string::String>,
    /// Display-call-to-action of the marketing image.
    #[prost(message, optional, tag="6")]
    pub marketing_image_display_call_to_action: ::core::option::Option<DisplayCallToAction>,
    /// Product images. Up to 15 images are supported.
    #[prost(message, repeated, tag="7")]
    pub product_images: ::prost::alloc::vec::Vec<ProductImage>,
    /// Product videos. Up to 7 videos are supported. At least one product video
    /// or a marketing image must be specified.
    #[prost(message, repeated, tag="8")]
    pub product_videos: ::prost::alloc::vec::Vec<ProductVideo>,
}
/// Gmail teaser data. The teaser is a small header that acts as an invitation
/// to view the rest of the ad (the body).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GmailTeaser {
    /// Headline of the teaser.
    #[prost(string, optional, tag="5")]
    pub headline: ::core::option::Option<::prost::alloc::string::String>,
    /// Description of the teaser.
    #[prost(string, optional, tag="6")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
    /// Business name of the advertiser.
    #[prost(string, optional, tag="7")]
    pub business_name: ::core::option::Option<::prost::alloc::string::String>,
    /// The MediaFile resource name of the logo image. Valid image types are GIF,
    /// JPEG and PNG. The minimum size is 144x144 pixels and the aspect ratio must
    /// be 1:1 (+-1%).
    #[prost(string, optional, tag="8")]
    pub logo_image: ::core::option::Option<::prost::alloc::string::String>,
}
/// Data for display call to action. The call to action is a piece of the ad
/// that prompts the user to do something. Like clicking a link or making a phone
/// call.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DisplayCallToAction {
    /// Text for the display-call-to-action.
    #[prost(string, optional, tag="5")]
    pub text: ::core::option::Option<::prost::alloc::string::String>,
    /// Text color for the display-call-to-action in hexadecimal, e.g. #ffffff for
    /// white.
    #[prost(string, optional, tag="6")]
    pub text_color: ::core::option::Option<::prost::alloc::string::String>,
    /// Identifies the URL collection in the `ad.url_collections` field. If not
    /// set, the URL defaults to `final_url`.
    #[prost(string, optional, tag="7")]
    pub url_collection_id: ::core::option::Option<::prost::alloc::string::String>,
}
/// Product image specific data.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProductImage {
    /// The MediaFile resource name of the product image. Valid image types are
    /// GIF, JPEG and PNG. The minimum size is 300x300 pixels and the aspect ratio
    /// must be 1:1 (+-1%).
    #[prost(string, optional, tag="4")]
    pub product_image: ::core::option::Option<::prost::alloc::string::String>,
    /// Description of the product.
    #[prost(string, optional, tag="5")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
    /// Display-call-to-action of the product image.
    #[prost(message, optional, tag="3")]
    pub display_call_to_action: ::core::option::Option<DisplayCallToAction>,
}
/// Product video specific data.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProductVideo {
    /// The MediaFile resource name of a video which must be hosted on YouTube.
    #[prost(string, optional, tag="2")]
    pub product_video: ::core::option::Option<::prost::alloc::string::String>,
}
/// An image ad.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImageAdInfo {
    /// Width in pixels of the full size image.
    #[prost(int64, optional, tag="15")]
    pub pixel_width: ::core::option::Option<i64>,
    /// Height in pixels of the full size image.
    #[prost(int64, optional, tag="16")]
    pub pixel_height: ::core::option::Option<i64>,
    /// URL of the full size image.
    #[prost(string, optional, tag="17")]
    pub image_url: ::core::option::Option<::prost::alloc::string::String>,
    /// Width in pixels of the preview size image.
    #[prost(int64, optional, tag="18")]
    pub preview_pixel_width: ::core::option::Option<i64>,
    /// Height in pixels of the preview size image.
    #[prost(int64, optional, tag="19")]
    pub preview_pixel_height: ::core::option::Option<i64>,
    /// URL of the preview size image.
    #[prost(string, optional, tag="20")]
    pub preview_image_url: ::core::option::Option<::prost::alloc::string::String>,
    /// The mime type of the image.
    #[prost(enumeration="super::enums::mime_type_enum::MimeType", tag="10")]
    pub mime_type: i32,
    /// The name of the image. If the image was created from a MediaFile, this is
    /// the MediaFile's name. If the image was created from bytes, this is empty.
    #[prost(string, optional, tag="21")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    /// The image to create the ImageAd from. This can be specified in one of
    /// two ways.
    /// 1. An existing MediaFile resource.
    /// 2. The raw image data as bytes.
    #[prost(oneof="image_ad_info::Image", tags="12, 13, 14")]
    pub image: ::core::option::Option<image_ad_info::Image>,
}
/// Nested message and enum types in `ImageAdInfo`.
pub mod image_ad_info {
    /// The image to create the ImageAd from. This can be specified in one of
    /// two ways.
    /// 1. An existing MediaFile resource.
    /// 2. The raw image data as bytes.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Image {
        /// The MediaFile resource to use for the image.
        #[prost(string, tag="12")]
        MediaFile(::prost::alloc::string::String),
        /// Raw image data as bytes.
        #[prost(bytes, tag="13")]
        Data(::prost::alloc::vec::Vec<u8>),
        /// An ad ID to copy the image from.
        #[prost(int64, tag="14")]
        AdIdToCopyImageFrom(i64),
    }
}
/// Representation of video bumper in-stream ad format (very short in-stream
/// non-skippable video ad).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VideoBumperInStreamAdInfo {
    /// The image assets of the companion banner used with the ad.
    #[prost(message, optional, tag="3")]
    pub companion_banner: ::core::option::Option<AdImageAsset>,
}
/// Representation of video non-skippable in-stream ad format (15 second
/// in-stream non-skippable video ad).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VideoNonSkippableInStreamAdInfo {
    /// The image assets of the companion banner used with the ad.
    #[prost(message, optional, tag="5")]
    pub companion_banner: ::core::option::Option<AdImageAsset>,
    /// Label on the "Call To Action" button taking the user to the video ad's
    /// final URL.
    #[prost(string, tag="3")]
    pub action_button_label: ::prost::alloc::string::String,
    /// Additional text displayed with the "Call To Action" button to give
    /// context and encourage clicking on the button.
    #[prost(string, tag="4")]
    pub action_headline: ::prost::alloc::string::String,
}
/// Representation of video TrueView in-stream ad format (ad shown during video
/// playback, often at beginning, which displays a skip button a few seconds into
/// the video).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VideoTrueViewInStreamAdInfo {
    /// Label on the CTA (call-to-action) button taking the user to the video ad's
    /// final URL.
    /// Required for TrueView for action campaigns, optional otherwise.
    #[prost(string, tag="4")]
    pub action_button_label: ::prost::alloc::string::String,
    /// Additional text displayed with the CTA (call-to-action) button to give
    /// context and encourage clicking on the button.
    #[prost(string, tag="5")]
    pub action_headline: ::prost::alloc::string::String,
    /// The image assets of the companion banner used with the ad.
    #[prost(message, optional, tag="7")]
    pub companion_banner: ::core::option::Option<AdImageAsset>,
}
/// Representation of video out-stream ad format (ad shown alongside a feed
/// with automatic playback, without sound).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VideoOutstreamAdInfo {
    /// The headline of the ad.
    #[prost(string, tag="3")]
    pub headline: ::prost::alloc::string::String,
    /// The description line.
    #[prost(string, tag="4")]
    pub description: ::prost::alloc::string::String,
}
/// Representation of video TrueView discovery ad format.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VideoTrueViewDiscoveryAdInfo {
    /// The headline of the ad.
    #[prost(string, tag="4")]
    pub headline: ::prost::alloc::string::String,
    /// First text line for a TrueView video discovery ad.
    #[prost(string, tag="5")]
    pub description1: ::prost::alloc::string::String,
    /// Second text line for a TrueView video discovery ad.
    #[prost(string, tag="6")]
    pub description2: ::prost::alloc::string::String,
    /// Video thumbnail image to use.
    #[prost(enumeration="super::enums::video_thumbnail_enum::VideoThumbnail", tag="7")]
    pub thumbnail: i32,
}
/// A video ad.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VideoAdInfo {
    /// The YouTube video assets used for the ad.
    #[prost(message, optional, tag="8")]
    pub video: ::core::option::Option<AdVideoAsset>,
    /// Format-specific schema for the different video formats.
    #[prost(oneof="video_ad_info::Format", tags="2, 3, 4, 5, 6")]
    pub format: ::core::option::Option<video_ad_info::Format>,
}
/// Nested message and enum types in `VideoAdInfo`.
pub mod video_ad_info {
    /// Format-specific schema for the different video formats.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Format {
        /// Video TrueView in-stream ad format.
        #[prost(message, tag="2")]
        InStream(super::VideoTrueViewInStreamAdInfo),
        /// Video bumper in-stream ad format.
        #[prost(message, tag="3")]
        Bumper(super::VideoBumperInStreamAdInfo),
        /// Video out-stream ad format.
        #[prost(message, tag="4")]
        OutStream(super::VideoOutstreamAdInfo),
        /// Video non-skippable in-stream ad format.
        #[prost(message, tag="5")]
        NonSkippable(super::VideoNonSkippableInStreamAdInfo),
        /// Video TrueView discovery ad format.
        #[prost(message, tag="6")]
        Discovery(super::VideoTrueViewDiscoveryAdInfo),
    }
}
/// A video responsive ad.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VideoResponsiveAdInfo {
    /// List of text assets used for the short headline, e.g. the "Call To Action"
    /// banner. Currently, only a single value for the short headline is supported.
    #[prost(message, repeated, tag="1")]
    pub headlines: ::prost::alloc::vec::Vec<AdTextAsset>,
    /// List of text assets used for the long headline.
    /// Currently, only a single value for the long headline is supported.
    #[prost(message, repeated, tag="2")]
    pub long_headlines: ::prost::alloc::vec::Vec<AdTextAsset>,
    /// List of text assets used for the description.
    /// Currently, only a single value for the description is supported.
    #[prost(message, repeated, tag="3")]
    pub descriptions: ::prost::alloc::vec::Vec<AdTextAsset>,
    /// List of text assets used for the button, e.g. the "Call To Action" button.
    /// Currently, only a single value for the button is supported.
    #[prost(message, repeated, tag="4")]
    pub call_to_actions: ::prost::alloc::vec::Vec<AdTextAsset>,
    /// List of YouTube video assets used for the ad.
    /// Currently, only a single value for the YouTube video asset is supported.
    #[prost(message, repeated, tag="5")]
    pub videos: ::prost::alloc::vec::Vec<AdVideoAsset>,
    /// List of image assets used for the companion banner.
    /// Currently, only a single value for the companion banner asset is supported.
    #[prost(message, repeated, tag="6")]
    pub companion_banners: ::prost::alloc::vec::Vec<AdImageAsset>,
}
/// A responsive search ad.
///
/// Responsive search ads let you create an ad that adapts to show more text, and
/// more relevant messages, to your customers. Enter multiple headlines and
/// descriptions when creating a responsive search ad, and over time, Google Ads
/// will automatically test different combinations and learn which combinations
/// perform best. By adapting your ad's content to more closely match potential
/// customers' search terms, responsive search ads may improve your campaign's
/// performance.
///
/// More information at <https://support.google.com/google-ads/answer/7684791>
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponsiveSearchAdInfo {
    /// List of text assets for headlines. When the ad serves the headlines will
    /// be selected from this list.
    #[prost(message, repeated, tag="1")]
    pub headlines: ::prost::alloc::vec::Vec<AdTextAsset>,
    /// List of text assets for descriptions. When the ad serves the descriptions
    /// will be selected from this list.
    #[prost(message, repeated, tag="2")]
    pub descriptions: ::prost::alloc::vec::Vec<AdTextAsset>,
    /// First part of text that can be appended to the URL in the ad.
    #[prost(string, optional, tag="5")]
    pub path1: ::core::option::Option<::prost::alloc::string::String>,
    /// Second part of text that can be appended to the URL in the ad. This field
    /// can only be set when `path1` is also set.
    #[prost(string, optional, tag="6")]
    pub path2: ::core::option::Option<::prost::alloc::string::String>,
}
/// A legacy responsive display ad. Ads of this type are labeled 'Responsive ads'
/// in the Google Ads UI.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LegacyResponsiveDisplayAdInfo {
    /// The short version of the ad's headline.
    #[prost(string, optional, tag="16")]
    pub short_headline: ::core::option::Option<::prost::alloc::string::String>,
    /// The long version of the ad's headline.
    #[prost(string, optional, tag="17")]
    pub long_headline: ::core::option::Option<::prost::alloc::string::String>,
    /// The description of the ad.
    #[prost(string, optional, tag="18")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
    /// The business name in the ad.
    #[prost(string, optional, tag="19")]
    pub business_name: ::core::option::Option<::prost::alloc::string::String>,
    /// Advertiser's consent to allow flexible color. When true, the ad may be
    /// served with different color if necessary. When false, the ad will be served
    /// with the specified colors or a neutral color.
    /// The default value is `true`.
    /// Must be true if `main_color` and `accent_color` are not set.
    #[prost(bool, optional, tag="20")]
    pub allow_flexible_color: ::core::option::Option<bool>,
    /// The accent color of the ad in hexadecimal, e.g. #ffffff for white.
    /// If one of `main_color` and `accent_color` is set, the other is required as
    /// well.
    #[prost(string, optional, tag="21")]
    pub accent_color: ::core::option::Option<::prost::alloc::string::String>,
    /// The main color of the ad in hexadecimal, e.g. #ffffff for white.
    /// If one of `main_color` and `accent_color` is set, the other is required as
    /// well.
    #[prost(string, optional, tag="22")]
    pub main_color: ::core::option::Option<::prost::alloc::string::String>,
    /// The call-to-action text for the ad.
    #[prost(string, optional, tag="23")]
    pub call_to_action_text: ::core::option::Option<::prost::alloc::string::String>,
    /// The MediaFile resource name of the logo image used in the ad.
    #[prost(string, optional, tag="24")]
    pub logo_image: ::core::option::Option<::prost::alloc::string::String>,
    /// The MediaFile resource name of the square logo image used in the ad.
    #[prost(string, optional, tag="25")]
    pub square_logo_image: ::core::option::Option<::prost::alloc::string::String>,
    /// The MediaFile resource name of the marketing image used in the ad.
    #[prost(string, optional, tag="26")]
    pub marketing_image: ::core::option::Option<::prost::alloc::string::String>,
    /// The MediaFile resource name of the square marketing image used in the ad.
    #[prost(string, optional, tag="27")]
    pub square_marketing_image: ::core::option::Option<::prost::alloc::string::String>,
    /// Specifies which format the ad will be served in. Default is ALL_FORMATS.
    #[prost(enumeration="super::enums::display_ad_format_setting_enum::DisplayAdFormatSetting", tag="13")]
    pub format_setting: i32,
    /// Prefix before price. E.g. 'as low as'.
    #[prost(string, optional, tag="28")]
    pub price_prefix: ::core::option::Option<::prost::alloc::string::String>,
    /// Promotion text used for dynamic formats of responsive ads. For example
    /// 'Free two-day shipping'.
    #[prost(string, optional, tag="29")]
    pub promo_text: ::core::option::Option<::prost::alloc::string::String>,
}
/// An app ad.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AppAdInfo {
    /// Mandatory ad text.
    #[prost(message, optional, tag="1")]
    pub mandatory_ad_text: ::core::option::Option<AdTextAsset>,
    /// List of text assets for headlines. When the ad serves the headlines will
    /// be selected from this list.
    #[prost(message, repeated, tag="2")]
    pub headlines: ::prost::alloc::vec::Vec<AdTextAsset>,
    /// List of text assets for descriptions. When the ad serves the descriptions
    /// will be selected from this list.
    #[prost(message, repeated, tag="3")]
    pub descriptions: ::prost::alloc::vec::Vec<AdTextAsset>,
    /// List of image assets that may be displayed with the ad.
    #[prost(message, repeated, tag="4")]
    pub images: ::prost::alloc::vec::Vec<AdImageAsset>,
    /// List of YouTube video assets that may be displayed with the ad.
    #[prost(message, repeated, tag="5")]
    pub youtube_videos: ::prost::alloc::vec::Vec<AdVideoAsset>,
    /// List of media bundle assets that may be used with the ad.
    #[prost(message, repeated, tag="6")]
    pub html5_media_bundles: ::prost::alloc::vec::Vec<AdMediaBundleAsset>,
}
/// App engagement ads allow you to write text encouraging a specific action in
/// the app, like checking in, making a purchase, or booking a flight.
/// They allow you to send users to a specific part of your app where they can
/// find what they're looking for easier and faster.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AppEngagementAdInfo {
    /// List of text assets for headlines. When the ad serves the headlines will
    /// be selected from this list.
    #[prost(message, repeated, tag="1")]
    pub headlines: ::prost::alloc::vec::Vec<AdTextAsset>,
    /// List of text assets for descriptions. When the ad serves the descriptions
    /// will be selected from this list.
    #[prost(message, repeated, tag="2")]
    pub descriptions: ::prost::alloc::vec::Vec<AdTextAsset>,
    /// List of image assets that may be displayed with the ad.
    #[prost(message, repeated, tag="3")]
    pub images: ::prost::alloc::vec::Vec<AdImageAsset>,
    /// List of video assets that may be displayed with the ad.
    #[prost(message, repeated, tag="4")]
    pub videos: ::prost::alloc::vec::Vec<AdVideoAsset>,
}
/// App pre-registration ads link to your app or game listing on Google Play, and
/// can run on Google Play, on YouTube (in-stream only), and within other apps
/// and mobile websites on the Display Network. It will help capture people’s
/// interest in your app or game and generate an early install base for your app
/// or game before a launch.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AppPreRegistrationAdInfo {
    /// List of text assets for headlines. When the ad serves the headlines will
    /// be selected from this list.
    #[prost(message, repeated, tag="1")]
    pub headlines: ::prost::alloc::vec::Vec<AdTextAsset>,
    /// List of text assets for descriptions. When the ad serves the descriptions
    /// will be selected from this list.
    #[prost(message, repeated, tag="2")]
    pub descriptions: ::prost::alloc::vec::Vec<AdTextAsset>,
    /// List of image asset IDs whose images may be displayed with the ad.
    #[prost(message, repeated, tag="3")]
    pub images: ::prost::alloc::vec::Vec<AdImageAsset>,
    /// List of YouTube video asset IDs whose videos may be displayed with the ad.
    #[prost(message, repeated, tag="4")]
    pub youtube_videos: ::prost::alloc::vec::Vec<AdVideoAsset>,
}
/// A legacy app install ad that only can be used by a few select customers.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LegacyAppInstallAdInfo {
    /// The ID of the mobile app.
    #[prost(string, optional, tag="6")]
    pub app_id: ::core::option::Option<::prost::alloc::string::String>,
    /// The app store the mobile app is available in.
    #[prost(enumeration="super::enums::legacy_app_install_ad_app_store_enum::LegacyAppInstallAdAppStore", tag="2")]
    pub app_store: i32,
    /// The headline of the ad.
    #[prost(string, optional, tag="7")]
    pub headline: ::core::option::Option<::prost::alloc::string::String>,
    /// The first description line of the ad.
    #[prost(string, optional, tag="8")]
    pub description1: ::core::option::Option<::prost::alloc::string::String>,
    /// The second description line of the ad.
    #[prost(string, optional, tag="9")]
    pub description2: ::core::option::Option<::prost::alloc::string::String>,
}
/// A responsive display ad.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponsiveDisplayAdInfo {
    /// Marketing images to be used in the ad. Valid image types are GIF,
    /// JPEG, and PNG. The minimum size is 600x314 and the aspect ratio must
    /// be 1.91:1 (+-1%). At least one `marketing_image` is required. Combined
    /// with `square_marketing_images`, the maximum is 15.
    #[prost(message, repeated, tag="1")]
    pub marketing_images: ::prost::alloc::vec::Vec<AdImageAsset>,
    /// Square marketing images to be used in the ad. Valid image types are GIF,
    /// JPEG, and PNG. The minimum size is 300x300 and the aspect ratio must
    /// be 1:1 (+-1%). At least one square `marketing_image` is required. Combined
    /// with `marketing_images`, the maximum is 15.
    #[prost(message, repeated, tag="2")]
    pub square_marketing_images: ::prost::alloc::vec::Vec<AdImageAsset>,
    /// Logo images to be used in the ad. Valid image types are GIF,
    /// JPEG, and PNG. The minimum size is 512x128 and the aspect ratio must
    /// be 4:1 (+-1%). Combined with `square_logo_images`, the maximum is 5.
    #[prost(message, repeated, tag="3")]
    pub logo_images: ::prost::alloc::vec::Vec<AdImageAsset>,
    /// Square logo images to be used in the ad. Valid image types are GIF,
    /// JPEG, and PNG. The minimum size is 128x128 and the aspect ratio must
    /// be 1:1 (+-1%). Combined with `square_logo_images`, the maximum is 5.
    #[prost(message, repeated, tag="4")]
    pub square_logo_images: ::prost::alloc::vec::Vec<AdImageAsset>,
    /// Short format headlines for the ad. The maximum length is 30 characters.
    /// At least 1 and max 5 headlines can be specified.
    #[prost(message, repeated, tag="5")]
    pub headlines: ::prost::alloc::vec::Vec<AdTextAsset>,
    /// A required long format headline. The maximum length is 90 characters.
    #[prost(message, optional, tag="6")]
    pub long_headline: ::core::option::Option<AdTextAsset>,
    /// Descriptive texts for the ad. The maximum length is 90 characters. At
    /// least 1 and max 5 headlines can be specified.
    #[prost(message, repeated, tag="7")]
    pub descriptions: ::prost::alloc::vec::Vec<AdTextAsset>,
    /// Optional YouTube videos for the ad. A maximum of 5 videos can be specified.
    #[prost(message, repeated, tag="8")]
    pub youtube_videos: ::prost::alloc::vec::Vec<AdVideoAsset>,
    /// The advertiser/brand name. Maximum display width is 25.
    #[prost(string, optional, tag="17")]
    pub business_name: ::core::option::Option<::prost::alloc::string::String>,
    /// The main color of the ad in hexadecimal, e.g. #ffffff for white.
    /// If one of `main_color` and `accent_color` is set, the other is required as
    /// well.
    #[prost(string, optional, tag="18")]
    pub main_color: ::core::option::Option<::prost::alloc::string::String>,
    /// The accent color of the ad in hexadecimal, e.g. #ffffff for white.
    /// If one of `main_color` and `accent_color` is set, the other is required as
    /// well.
    #[prost(string, optional, tag="19")]
    pub accent_color: ::core::option::Option<::prost::alloc::string::String>,
    /// Advertiser's consent to allow flexible color. When true, the ad may be
    /// served with different color if necessary. When false, the ad will be served
    /// with the specified colors or a neutral color.
    /// The default value is `true`.
    /// Must be true if `main_color` and `accent_color` are not set.
    #[prost(bool, optional, tag="20")]
    pub allow_flexible_color: ::core::option::Option<bool>,
    /// The call-to-action text for the ad. Maximum display width is 30.
    #[prost(string, optional, tag="21")]
    pub call_to_action_text: ::core::option::Option<::prost::alloc::string::String>,
    /// Prefix before price. E.g. 'as low as'.
    #[prost(string, optional, tag="22")]
    pub price_prefix: ::core::option::Option<::prost::alloc::string::String>,
    /// Promotion text used for dynamic formats of responsive ads. For example
    /// 'Free two-day shipping'.
    #[prost(string, optional, tag="23")]
    pub promo_text: ::core::option::Option<::prost::alloc::string::String>,
    /// Specifies which format the ad will be served in. Default is ALL_FORMATS.
    #[prost(enumeration="super::enums::display_ad_format_setting_enum::DisplayAdFormatSetting", tag="16")]
    pub format_setting: i32,
    /// Specification for various creative controls.
    #[prost(message, optional, tag="24")]
    pub control_spec: ::core::option::Option<ResponsiveDisplayAdControlSpec>,
}
/// A local ad.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LocalAdInfo {
    /// List of text assets for headlines. When the ad serves the headlines will
    /// be selected from this list. At least 1 and at most 5 headlines must be
    /// specified.
    #[prost(message, repeated, tag="1")]
    pub headlines: ::prost::alloc::vec::Vec<AdTextAsset>,
    /// List of text assets for descriptions. When the ad serves the descriptions
    /// will be selected from this list. At least 1 and at most 5 descriptions must
    /// be specified.
    #[prost(message, repeated, tag="2")]
    pub descriptions: ::prost::alloc::vec::Vec<AdTextAsset>,
    /// List of text assets for call-to-actions. When the ad serves the
    /// call-to-actions will be selected from this list. Call-to-actions are
    /// optional and at most 5 can be specified.
    #[prost(message, repeated, tag="3")]
    pub call_to_actions: ::prost::alloc::vec::Vec<AdTextAsset>,
    /// List of marketing image assets that may be displayed with the ad. The
    /// images must be 314x600 pixels or 320x320 pixels. At least 1 and at most
    /// 20 image assets must be specified.
    #[prost(message, repeated, tag="4")]
    pub marketing_images: ::prost::alloc::vec::Vec<AdImageAsset>,
    /// List of logo image assets that may be displayed with the ad. The images
    /// must be 128x128 pixels and not larger than 120KB. At least 1 and at most 5
    /// image assets must be specified.
    #[prost(message, repeated, tag="5")]
    pub logo_images: ::prost::alloc::vec::Vec<AdImageAsset>,
    /// List of YouTube video assets that may be displayed with the ad. Videos
    /// are optional and at most 20 can be specified.
    #[prost(message, repeated, tag="6")]
    pub videos: ::prost::alloc::vec::Vec<AdVideoAsset>,
    /// First part of optional text that can be appended to the URL in the ad.
    #[prost(string, optional, tag="9")]
    pub path1: ::core::option::Option<::prost::alloc::string::String>,
    /// Second part of optional text that can be appended to the URL in the ad.
    /// This field can only be set when `path1` is also set.
    #[prost(string, optional, tag="10")]
    pub path2: ::core::option::Option<::prost::alloc::string::String>,
}
/// A generic type of display ad. The exact ad format is controlled by the
/// `display_upload_product_type` field, which determines what kinds of data
/// need to be included with the ad.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DisplayUploadAdInfo {
    /// The product type of this ad. See comments on the enum for details.
    #[prost(enumeration="super::enums::display_upload_product_type_enum::DisplayUploadProductType", tag="1")]
    pub display_upload_product_type: i32,
    /// The asset data that makes up the ad.
    #[prost(oneof="display_upload_ad_info::MediaAsset", tags="2")]
    pub media_asset: ::core::option::Option<display_upload_ad_info::MediaAsset>,
}
/// Nested message and enum types in `DisplayUploadAdInfo`.
pub mod display_upload_ad_info {
    /// The asset data that makes up the ad.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum MediaAsset {
        /// A media bundle asset to be used in the ad. For information about the
        /// media bundle for HTML5_UPLOAD_AD, see
        /// <https://support.google.com/google-ads/answer/1722096>
        /// Media bundles that are part of dynamic product types use a special format
        /// that needs to be created through the Google Web Designer. See
        /// <https://support.google.com/webdesigner/answer/7543898> for more
        /// information.
        #[prost(message, tag="2")]
        MediaBundle(super::AdMediaBundleAsset),
    }
}
/// Specification for various creative controls for a responsive display ad.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponsiveDisplayAdControlSpec {
    /// Whether the advertiser has opted into the asset enhancements feature.
    #[prost(bool, tag="1")]
    pub enable_asset_enhancements: bool,
    /// Whether the advertiser has opted into auto-gen video feature.
    #[prost(bool, tag="2")]
    pub enable_autogen_video: bool,
}
/// A Smart campaign ad.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SmartCampaignAdInfo {
    /// List of text assets for headlines. When the ad serves the headlines will
    /// be selected from this list. 3 headlines must be specified.
    #[prost(message, repeated, tag="1")]
    pub headlines: ::prost::alloc::vec::Vec<AdTextAsset>,
    /// List of text assets for descriptions. When the ad serves the descriptions
    /// will be selected from this list. 2 descriptions must be specified.
    #[prost(message, repeated, tag="2")]
    pub descriptions: ::prost::alloc::vec::Vec<AdTextAsset>,
}
/// A call ad.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CallAdInfo {
    /// The country code in the ad.
    #[prost(string, tag="1")]
    pub country_code: ::prost::alloc::string::String,
    /// The phone number in the ad.
    #[prost(string, tag="2")]
    pub phone_number: ::prost::alloc::string::String,
    /// The business name in the ad.
    #[prost(string, tag="3")]
    pub business_name: ::prost::alloc::string::String,
    /// First headline in the ad.
    #[prost(string, tag="11")]
    pub headline1: ::prost::alloc::string::String,
    /// Second headline in the ad.
    #[prost(string, tag="12")]
    pub headline2: ::prost::alloc::string::String,
    /// The first line of the ad's description.
    #[prost(string, tag="4")]
    pub description1: ::prost::alloc::string::String,
    /// The second line of the ad's description.
    #[prost(string, tag="5")]
    pub description2: ::prost::alloc::string::String,
    /// Whether to enable call tracking for the creative. Enabling call
    /// tracking also enables call conversions.
    #[prost(bool, tag="6")]
    pub call_tracked: bool,
    /// Whether to disable call conversion for the creative.
    /// If set to `true`, disables call conversions even when `call_tracked` is
    /// `true`.
    /// If `call_tracked` is `false`, this field is ignored.
    #[prost(bool, tag="7")]
    pub disable_call_conversion: bool,
    /// The URL to be used for phone number verification.
    #[prost(string, tag="8")]
    pub phone_number_verification_url: ::prost::alloc::string::String,
    /// The conversion action to attribute a call conversion to. If not set a
    /// default conversion action is used. This field only has effect if
    /// `call_tracked` is set to `true`. Otherwise this field is ignored.
    #[prost(string, tag="9")]
    pub conversion_action: ::prost::alloc::string::String,
    /// The call conversion behavior of this call ad. It can use its own call
    /// conversion setting, inherit the account level setting, or be disabled.
    #[prost(enumeration="super::enums::call_conversion_reporting_state_enum::CallConversionReportingState", tag="10")]
    pub conversion_reporting_state: i32,
    /// First part of text that can be appended to the URL in the ad. Optional.
    #[prost(string, tag="13")]
    pub path1: ::prost::alloc::string::String,
    /// Second part of text that can be appended to the URL in the ad. This field
    /// can only be set when `path1` is also set. Optional.
    #[prost(string, tag="14")]
    pub path2: ::prost::alloc::string::String,
}
// Proto file describing criteria types.

/// A keyword criterion.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeywordInfo {
    /// The text of the keyword (at most 80 characters and 10 words).
    #[prost(string, optional, tag="3")]
    pub text: ::core::option::Option<::prost::alloc::string::String>,
    /// The match type of the keyword.
    #[prost(enumeration="super::enums::keyword_match_type_enum::KeywordMatchType", tag="2")]
    pub match_type: i32,
}
/// A placement criterion. This can be used to modify bids for sites when
/// targeting the content network.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlacementInfo {
    /// URL of the placement.
    ///
    /// For example, "<http://www.domain.com".>
    #[prost(string, optional, tag="2")]
    pub url: ::core::option::Option<::prost::alloc::string::String>,
}
/// A mobile app category criterion.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MobileAppCategoryInfo {
    /// The mobile app category constant resource name.
    #[prost(string, optional, tag="2")]
    pub mobile_app_category_constant: ::core::option::Option<::prost::alloc::string::String>,
}
/// A mobile application criterion.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MobileApplicationInfo {
    /// A string that uniquely identifies a mobile application to Google Ads API.
    /// The format of this string is "{platform}-{platform_native_id}", where
    /// platform is "1" for iOS apps and "2" for Android apps, and where
    /// platform_native_id is the mobile application identifier native to the
    /// corresponding platform.
    /// For iOS, this native identifier is the 9 digit string that appears at the
    /// end of an App Store URL (e.g., "476943146" for "Flood-It! 2" whose App
    /// Store link is "<http://itunes.apple.com/us/app/flood-it!-2/id476943146">).
    /// For Android, this native identifier is the application's package name
    /// (e.g., "com.labpixies.colordrips" for "Color Drips" given Google Play link
    /// "<https://play.google.com/store/apps/details?id=com.labpixies.colordrips">).
    /// A well formed app id for Google Ads API would thus be "1-476943146" for iOS
    /// and "2-com.labpixies.colordrips" for Android.
    /// This field is required and must be set in CREATE operations.
    #[prost(string, optional, tag="4")]
    pub app_id: ::core::option::Option<::prost::alloc::string::String>,
    /// Name of this mobile application.
    #[prost(string, optional, tag="5")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
}
/// A location criterion.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LocationInfo {
    /// The geo target constant resource name.
    #[prost(string, optional, tag="2")]
    pub geo_target_constant: ::core::option::Option<::prost::alloc::string::String>,
}
/// A device criterion.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeviceInfo {
    /// Type of the device.
    #[prost(enumeration="super::enums::device_enum::Device", tag="1")]
    pub r#type: i32,
}
/// A preferred content criterion.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PreferredContentInfo {
    /// Type of the preferred content.
    #[prost(enumeration="super::enums::preferred_content_type_enum::PreferredContentType", tag="2")]
    pub r#type: i32,
}
/// A listing group criterion.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListingGroupInfo {
    /// Type of the listing group.
    #[prost(enumeration="super::enums::listing_group_type_enum::ListingGroupType", tag="1")]
    pub r#type: i32,
    /// Dimension value with which this listing group is refining its parent.
    /// Undefined for the root group.
    #[prost(message, optional, tag="2")]
    pub case_value: ::core::option::Option<ListingDimensionInfo>,
    /// Resource name of ad group criterion which is the parent listing group
    /// subdivision. Null for the root group.
    #[prost(string, optional, tag="4")]
    pub parent_ad_group_criterion: ::core::option::Option<::prost::alloc::string::String>,
}
/// A listing scope criterion.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListingScopeInfo {
    /// Scope of the campaign criterion.
    #[prost(message, repeated, tag="2")]
    pub dimensions: ::prost::alloc::vec::Vec<ListingDimensionInfo>,
}
/// Listing dimensions for listing group criterion.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListingDimensionInfo {
    /// Dimension of one of the types below is always present.
    #[prost(oneof="listing_dimension_info::Dimension", tags="2, 3, 4, 5, 6, 13, 15, 8, 9, 10, 16, 11, 12, 14")]
    pub dimension: ::core::option::Option<listing_dimension_info::Dimension>,
}
/// Nested message and enum types in `ListingDimensionInfo`.
pub mod listing_dimension_info {
    /// Dimension of one of the types below is always present.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Dimension {
        /// Advertiser-specific hotel ID.
        #[prost(message, tag="2")]
        HotelId(super::HotelIdInfo),
        /// Class of the hotel as a number of stars 1 to 5.
        #[prost(message, tag="3")]
        HotelClass(super::HotelClassInfo),
        /// Country or Region the hotel is located in.
        #[prost(message, tag="4")]
        HotelCountryRegion(super::HotelCountryRegionInfo),
        /// State the hotel is located in.
        #[prost(message, tag="5")]
        HotelState(super::HotelStateInfo),
        /// City the hotel is located in.
        #[prost(message, tag="6")]
        HotelCity(super::HotelCityInfo),
        /// Bidding category of a product offer.
        #[prost(message, tag="13")]
        ProductBiddingCategory(super::ProductBiddingCategoryInfo),
        /// Brand of a product offer.
        #[prost(message, tag="15")]
        ProductBrand(super::ProductBrandInfo),
        /// Locality of a product offer.
        #[prost(message, tag="8")]
        ProductChannel(super::ProductChannelInfo),
        /// Availability of a product offer.
        #[prost(message, tag="9")]
        ProductChannelExclusivity(super::ProductChannelExclusivityInfo),
        /// Condition of a product offer.
        #[prost(message, tag="10")]
        ProductCondition(super::ProductConditionInfo),
        /// Custom attribute of a product offer.
        #[prost(message, tag="16")]
        ProductCustomAttribute(super::ProductCustomAttributeInfo),
        /// Item id of a product offer.
        #[prost(message, tag="11")]
        ProductItemId(super::ProductItemIdInfo),
        /// Type of a product offer.
        #[prost(message, tag="12")]
        ProductType(super::ProductTypeInfo),
        /// Unknown dimension. Set when no other listing dimension is set.
        #[prost(message, tag="14")]
        UnknownListingDimension(super::UnknownListingDimensionInfo),
    }
}
/// Advertiser-specific hotel ID.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HotelIdInfo {
    /// String value of the hotel ID.
    #[prost(string, optional, tag="2")]
    pub value: ::core::option::Option<::prost::alloc::string::String>,
}
/// Class of the hotel as a number of stars 1 to 5.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HotelClassInfo {
    /// Long value of the hotel class.
    #[prost(int64, optional, tag="2")]
    pub value: ::core::option::Option<i64>,
}
/// Country or Region the hotel is located in.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HotelCountryRegionInfo {
    /// The Geo Target Constant resource name.
    #[prost(string, optional, tag="2")]
    pub country_region_criterion: ::core::option::Option<::prost::alloc::string::String>,
}
/// State the hotel is located in.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HotelStateInfo {
    /// The Geo Target Constant resource name.
    #[prost(string, optional, tag="2")]
    pub state_criterion: ::core::option::Option<::prost::alloc::string::String>,
}
/// City the hotel is located in.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HotelCityInfo {
    /// The Geo Target Constant resource name.
    #[prost(string, optional, tag="2")]
    pub city_criterion: ::core::option::Option<::prost::alloc::string::String>,
}
/// Bidding category of a product offer.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProductBiddingCategoryInfo {
    /// ID of the product bidding category.
    ///
    /// This ID is equivalent to the google_product_category ID as described in
    /// this article: <https://support.google.com/merchants/answer/6324436>
    #[prost(int64, optional, tag="4")]
    pub id: ::core::option::Option<i64>,
    /// Two-letter upper-case country code of the product bidding category. It must
    /// match the campaign.shopping_setting.sales_country field.
    #[prost(string, optional, tag="5")]
    pub country_code: ::core::option::Option<::prost::alloc::string::String>,
    /// Level of the product bidding category.
    #[prost(enumeration="super::enums::product_bidding_category_level_enum::ProductBiddingCategoryLevel", tag="3")]
    pub level: i32,
}
/// Brand of the product.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProductBrandInfo {
    /// String value of the product brand.
    #[prost(string, optional, tag="2")]
    pub value: ::core::option::Option<::prost::alloc::string::String>,
}
/// Locality of a product offer.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProductChannelInfo {
    /// Value of the locality.
    #[prost(enumeration="super::enums::product_channel_enum::ProductChannel", tag="1")]
    pub channel: i32,
}
/// Availability of a product offer.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProductChannelExclusivityInfo {
    /// Value of the availability.
    #[prost(enumeration="super::enums::product_channel_exclusivity_enum::ProductChannelExclusivity", tag="1")]
    pub channel_exclusivity: i32,
}
/// Condition of a product offer.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProductConditionInfo {
    /// Value of the condition.
    #[prost(enumeration="super::enums::product_condition_enum::ProductCondition", tag="1")]
    pub condition: i32,
}
/// Custom attribute of a product offer.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProductCustomAttributeInfo {
    /// String value of the product custom attribute.
    #[prost(string, optional, tag="3")]
    pub value: ::core::option::Option<::prost::alloc::string::String>,
    /// Indicates the index of the custom attribute.
    #[prost(enumeration="super::enums::product_custom_attribute_index_enum::ProductCustomAttributeIndex", tag="2")]
    pub index: i32,
}
/// Item id of a product offer.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProductItemIdInfo {
    /// Value of the id.
    #[prost(string, optional, tag="2")]
    pub value: ::core::option::Option<::prost::alloc::string::String>,
}
/// Type of a product offer.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProductTypeInfo {
    /// Value of the type.
    #[prost(string, optional, tag="3")]
    pub value: ::core::option::Option<::prost::alloc::string::String>,
    /// Level of the type.
    #[prost(enumeration="super::enums::product_type_level_enum::ProductTypeLevel", tag="2")]
    pub level: i32,
}
/// Unknown listing dimension.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnknownListingDimensionInfo {
}
/// Criterion for hotel date selection (default dates vs. user selected).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HotelDateSelectionTypeInfo {
    /// Type of the hotel date selection
    #[prost(enumeration="super::enums::hotel_date_selection_type_enum::HotelDateSelectionType", tag="1")]
    pub r#type: i32,
}
/// Criterion for number of days prior to the stay the booking is being made.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HotelAdvanceBookingWindowInfo {
    /// Low end of the number of days prior to the stay.
    #[prost(int64, optional, tag="3")]
    pub min_days: ::core::option::Option<i64>,
    /// High end of the number of days prior to the stay.
    #[prost(int64, optional, tag="4")]
    pub max_days: ::core::option::Option<i64>,
}
/// Criterion for length of hotel stay in nights.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HotelLengthOfStayInfo {
    /// Low end of the number of nights in the stay.
    #[prost(int64, optional, tag="3")]
    pub min_nights: ::core::option::Option<i64>,
    /// High end of the number of nights in the stay.
    #[prost(int64, optional, tag="4")]
    pub max_nights: ::core::option::Option<i64>,
}
/// Criterion for a check-in date range.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HotelCheckInDateRangeInfo {
    /// Start date in the YYYY-MM-DD format.
    #[prost(string, tag="1")]
    pub start_date: ::prost::alloc::string::String,
    /// End date in the YYYY-MM-DD format.
    #[prost(string, tag="2")]
    pub end_date: ::prost::alloc::string::String,
}
/// Criterion for day of the week the booking is for.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HotelCheckInDayInfo {
    /// The day of the week.
    #[prost(enumeration="super::enums::day_of_week_enum::DayOfWeek", tag="1")]
    pub day_of_week: i32,
}
/// Criterion for Interaction Type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InteractionTypeInfo {
    /// The interaction type.
    #[prost(enumeration="super::enums::interaction_type_enum::InteractionType", tag="1")]
    pub r#type: i32,
}
/// Represents an AdSchedule criterion.
///
/// AdSchedule is specified as the day of the week and a time interval
/// within which ads will be shown.
///
/// No more than six AdSchedules can be added for the same day.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdScheduleInfo {
    /// Minutes after the start hour at which this schedule starts.
    ///
    /// This field is required for CREATE operations and is prohibited on UPDATE
    /// operations.
    #[prost(enumeration="super::enums::minute_of_hour_enum::MinuteOfHour", tag="1")]
    pub start_minute: i32,
    /// Minutes after the end hour at which this schedule ends. The schedule is
    /// exclusive of the end minute.
    ///
    /// This field is required for CREATE operations and is prohibited on UPDATE
    /// operations.
    #[prost(enumeration="super::enums::minute_of_hour_enum::MinuteOfHour", tag="2")]
    pub end_minute: i32,
    /// Starting hour in 24 hour time.
    /// This field must be between 0 and 23, inclusive.
    ///
    /// This field is required for CREATE operations and is prohibited on UPDATE
    /// operations.
    #[prost(int32, optional, tag="6")]
    pub start_hour: ::core::option::Option<i32>,
    /// Ending hour in 24 hour time; 24 signifies end of the day.
    /// This field must be between 0 and 24, inclusive.
    ///
    /// This field is required for CREATE operations and is prohibited on UPDATE
    /// operations.
    #[prost(int32, optional, tag="7")]
    pub end_hour: ::core::option::Option<i32>,
    /// Day of the week the schedule applies to.
    ///
    /// This field is required for CREATE operations and is prohibited on UPDATE
    /// operations.
    #[prost(enumeration="super::enums::day_of_week_enum::DayOfWeek", tag="5")]
    pub day_of_week: i32,
}
/// An age range criterion.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AgeRangeInfo {
    /// Type of the age range.
    #[prost(enumeration="super::enums::age_range_type_enum::AgeRangeType", tag="1")]
    pub r#type: i32,
}
/// A gender criterion.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenderInfo {
    /// Type of the gender.
    #[prost(enumeration="super::enums::gender_type_enum::GenderType", tag="1")]
    pub r#type: i32,
}
/// An income range criterion.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IncomeRangeInfo {
    /// Type of the income range.
    #[prost(enumeration="super::enums::income_range_type_enum::IncomeRangeType", tag="1")]
    pub r#type: i32,
}
/// A parental status criterion.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ParentalStatusInfo {
    /// Type of the parental status.
    #[prost(enumeration="super::enums::parental_status_type_enum::ParentalStatusType", tag="1")]
    pub r#type: i32,
}
/// A YouTube Video criterion.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct YouTubeVideoInfo {
    /// YouTube video id as it appears on the YouTube watch page.
    #[prost(string, optional, tag="2")]
    pub video_id: ::core::option::Option<::prost::alloc::string::String>,
}
/// A YouTube Channel criterion.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct YouTubeChannelInfo {
    /// The YouTube uploader channel id or the channel code of a YouTube channel.
    #[prost(string, optional, tag="2")]
    pub channel_id: ::core::option::Option<::prost::alloc::string::String>,
}
/// A User List criterion. Represents a user list that is defined by the
/// advertiser to be targeted.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserListInfo {
    /// The User List resource name.
    #[prost(string, optional, tag="2")]
    pub user_list: ::core::option::Option<::prost::alloc::string::String>,
}
/// A Proximity criterion. The geo point and radius determine what geographical
/// area is included. The address is a description of the geo point that does
/// not affect ad serving.
///
/// There are two ways to create a proximity. First, by setting an address
/// and radius. The geo point will be automatically computed. Second, by
/// setting a geo point and radius. The address is an optional label that won't
/// be validated.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProximityInfo {
    /// Latitude and longitude.
    #[prost(message, optional, tag="1")]
    pub geo_point: ::core::option::Option<GeoPointInfo>,
    /// The radius of the proximity.
    #[prost(double, optional, tag="5")]
    pub radius: ::core::option::Option<f64>,
    /// The unit of measurement of the radius. Default is KILOMETERS.
    #[prost(enumeration="super::enums::proximity_radius_units_enum::ProximityRadiusUnits", tag="3")]
    pub radius_units: i32,
    /// Full address.
    #[prost(message, optional, tag="4")]
    pub address: ::core::option::Option<AddressInfo>,
}
/// Geo point for proximity criterion.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GeoPointInfo {
    /// Micro degrees for the longitude.
    #[prost(int32, optional, tag="3")]
    pub longitude_in_micro_degrees: ::core::option::Option<i32>,
    /// Micro degrees for the latitude.
    #[prost(int32, optional, tag="4")]
    pub latitude_in_micro_degrees: ::core::option::Option<i32>,
}
/// Address for proximity criterion.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddressInfo {
    /// Postal code.
    #[prost(string, optional, tag="8")]
    pub postal_code: ::core::option::Option<::prost::alloc::string::String>,
    /// Province or state code.
    #[prost(string, optional, tag="9")]
    pub province_code: ::core::option::Option<::prost::alloc::string::String>,
    /// Country code.
    #[prost(string, optional, tag="10")]
    pub country_code: ::core::option::Option<::prost::alloc::string::String>,
    /// Province or state name.
    #[prost(string, optional, tag="11")]
    pub province_name: ::core::option::Option<::prost::alloc::string::String>,
    /// Street address line 1.
    #[prost(string, optional, tag="12")]
    pub street_address: ::core::option::Option<::prost::alloc::string::String>,
    /// Street address line 2. This field is write-only. It is only used for
    /// calculating the longitude and latitude of an address when geo_point is
    /// empty.
    #[prost(string, optional, tag="13")]
    pub street_address2: ::core::option::Option<::prost::alloc::string::String>,
    /// Name of the city.
    #[prost(string, optional, tag="14")]
    pub city_name: ::core::option::Option<::prost::alloc::string::String>,
}
/// A topic criterion. Use topics to target or exclude placements in the
/// Google Display Network based on the category into which the placement falls
/// (for example, "Pets & Animals/Pets/Dogs").
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TopicInfo {
    /// The Topic Constant resource name.
    #[prost(string, optional, tag="3")]
    pub topic_constant: ::core::option::Option<::prost::alloc::string::String>,
    /// The category to target or exclude. Each subsequent element in the array
    /// describes a more specific sub-category. For example,
    /// "Pets & Animals", "Pets", "Dogs" represents the "Pets & Animals/Pets/Dogs"
    /// category.
    #[prost(string, repeated, tag="4")]
    pub path: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// A language criterion.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LanguageInfo {
    /// The language constant resource name.
    #[prost(string, optional, tag="2")]
    pub language_constant: ::core::option::Option<::prost::alloc::string::String>,
}
/// An IpBlock criterion used for IP exclusions. We allow:
///   - IPv4 and IPv6 addresses
///   - individual addresses (192.168.0.1)
///   - masks for individual addresses (192.168.0.1/32)
///   - masks for Class C networks (192.168.0.1/24)
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IpBlockInfo {
    /// The IP address of this IP block.
    #[prost(string, optional, tag="2")]
    pub ip_address: ::core::option::Option<::prost::alloc::string::String>,
}
/// Content Label for category exclusion.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContentLabelInfo {
    /// Content label type, required for CREATE operations.
    #[prost(enumeration="super::enums::content_label_type_enum::ContentLabelType", tag="1")]
    pub r#type: i32,
}
/// Represents a Carrier Criterion.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CarrierInfo {
    /// The Carrier constant resource name.
    #[prost(string, optional, tag="2")]
    pub carrier_constant: ::core::option::Option<::prost::alloc::string::String>,
}
/// Represents a particular interest-based topic to be targeted.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserInterestInfo {
    /// The UserInterest resource name.
    #[prost(string, optional, tag="2")]
    pub user_interest_category: ::core::option::Option<::prost::alloc::string::String>,
}
/// Represents a criterion for targeting webpages of an advertiser's website.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WebpageInfo {
    /// The name of the criterion that is defined by this parameter. The name value
    /// will be used for identifying, sorting and filtering criteria with this type
    /// of parameters.
    ///
    /// This field is required for CREATE operations and is prohibited on UPDATE
    /// operations.
    #[prost(string, optional, tag="3")]
    pub criterion_name: ::core::option::Option<::prost::alloc::string::String>,
    /// Conditions, or logical expressions, for webpage targeting. The list of
    /// webpage targeting conditions are and-ed together when evaluated
    /// for targeting.
    ///
    /// This field is required for CREATE operations and is prohibited on UPDATE
    /// operations.
    #[prost(message, repeated, tag="2")]
    pub conditions: ::prost::alloc::vec::Vec<WebpageConditionInfo>,
    /// Website criteria coverage percentage. This is the computed percentage
    /// of website coverage based on the website target, negative website target
    /// and negative keywords in the ad group and campaign. For instance, when
    /// coverage returns as 1, it indicates it has 100% coverage. This field is
    /// read-only.
    #[prost(double, tag="4")]
    pub coverage_percentage: f64,
    /// List of sample urls that match the website target. This field is read-only.
    #[prost(message, optional, tag="5")]
    pub sample: ::core::option::Option<WebpageSampleInfo>,
}
/// Logical expression for targeting webpages of an advertiser's website.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WebpageConditionInfo {
    /// Operand of webpage targeting condition.
    #[prost(enumeration="super::enums::webpage_condition_operand_enum::WebpageConditionOperand", tag="1")]
    pub operand: i32,
    /// Operator of webpage targeting condition.
    #[prost(enumeration="super::enums::webpage_condition_operator_enum::WebpageConditionOperator", tag="2")]
    pub operator: i32,
    /// Argument of webpage targeting condition.
    #[prost(string, optional, tag="4")]
    pub argument: ::core::option::Option<::prost::alloc::string::String>,
}
/// List of sample urls that match the website target
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WebpageSampleInfo {
    /// Webpage sample urls
    #[prost(string, repeated, tag="1")]
    pub sample_urls: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Represents an operating system version to be targeted.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperatingSystemVersionInfo {
    /// The operating system version constant resource name.
    #[prost(string, optional, tag="2")]
    pub operating_system_version_constant: ::core::option::Option<::prost::alloc::string::String>,
}
/// An app payment model criterion.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AppPaymentModelInfo {
    /// Type of the app payment model.
    #[prost(enumeration="super::enums::app_payment_model_type_enum::AppPaymentModelType", tag="1")]
    pub r#type: i32,
}
/// A mobile device criterion.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MobileDeviceInfo {
    /// The mobile device constant resource name.
    #[prost(string, optional, tag="2")]
    pub mobile_device_constant: ::core::option::Option<::prost::alloc::string::String>,
}
/// A custom affinity criterion.
/// A criterion of this type is only targetable.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomAffinityInfo {
    /// The CustomInterest resource name.
    #[prost(string, optional, tag="2")]
    pub custom_affinity: ::core::option::Option<::prost::alloc::string::String>,
}
/// A custom intent criterion.
/// A criterion of this type is only targetable.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomIntentInfo {
    /// The CustomInterest resource name.
    #[prost(string, optional, tag="2")]
    pub custom_intent: ::core::option::Option<::prost::alloc::string::String>,
}
/// A radius around a list of locations specified via a feed.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LocationGroupInfo {
    /// Feed specifying locations for targeting.
    /// This is required and must be set in CREATE operations.
    #[prost(string, optional, tag="5")]
    pub feed: ::core::option::Option<::prost::alloc::string::String>,
    /// Geo target constant(s) restricting the scope of the geographic area within
    /// the feed. Currently only one geo target constant is allowed.
    #[prost(string, repeated, tag="6")]
    pub geo_target_constants: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Distance in units specifying the radius around targeted locations.
    /// This is required and must be set in CREATE operations.
    #[prost(int64, optional, tag="7")]
    pub radius: ::core::option::Option<i64>,
    /// Unit of the radius. Miles and meters are supported for geo target
    /// constants. Milli miles and meters are supported for feed item sets.
    /// This is required and must be set in CREATE operations.
    #[prost(enumeration="super::enums::location_group_radius_units_enum::LocationGroupRadiusUnits", tag="4")]
    pub radius_units: i32,
    /// FeedItemSets whose FeedItems are targeted. If multiple IDs are specified,
    /// then all items that appear in at least one set are targeted. This field
    /// cannot be used with geo_target_constants. This is optional and can only be
    /// set in CREATE operations.
    #[prost(string, repeated, tag="8")]
    pub feed_item_sets: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// A custom audience criterion.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomAudienceInfo {
    /// The CustomAudience resource name.
    #[prost(string, tag="1")]
    pub custom_audience: ::prost::alloc::string::String,
}
/// A combined audience criterion.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CombinedAudienceInfo {
    /// The CombinedAudience resource name.
    #[prost(string, tag="1")]
    pub combined_audience: ::prost::alloc::string::String,
}
/// A Smart Campaign keyword theme.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeywordThemeInfo {
    /// Either a predefined keyword theme constant or free-form text may be
    /// specified.
    #[prost(oneof="keyword_theme_info::KeywordTheme", tags="1, 2")]
    pub keyword_theme: ::core::option::Option<keyword_theme_info::KeywordTheme>,
}
/// Nested message and enum types in `KeywordThemeInfo`.
pub mod keyword_theme_info {
    /// Either a predefined keyword theme constant or free-form text may be
    /// specified.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum KeywordTheme {
        /// The resource name of a Smart Campaign keyword theme constant.
        /// `keywordThemeConstants/{keyword_theme_id}~{sub_keyword_theme_id}`
        #[prost(string, tag="1")]
        KeywordThemeConstant(::prost::alloc::string::String),
        /// Free-form text to be matched to a Smart Campaign keyword theme constant
        /// on a best-effort basis.
        #[prost(string, tag="2")]
        FreeFormKeywordTheme(::prost::alloc::string::String),
    }
}
// Proto file describing common feed proto messages.

/// Represents a price in a particular currency.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Money {
    /// Three-character ISO 4217 currency code.
    #[prost(string, optional, tag="3")]
    pub currency_code: ::core::option::Option<::prost::alloc::string::String>,
    /// Amount in micros. One million is equivalent to one unit.
    #[prost(int64, optional, tag="4")]
    pub amount_micros: ::core::option::Option<i64>,
}
// Proto file containing info messages for specific asset types.

/// A YouTube asset.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct YoutubeVideoAsset {
    /// YouTube video id. This is the 11 character string value used in the
    /// YouTube video URL.
    #[prost(string, optional, tag="2")]
    pub youtube_video_id: ::core::option::Option<::prost::alloc::string::String>,
    /// YouTube video title.
    #[prost(string, tag="3")]
    pub youtube_video_title: ::prost::alloc::string::String,
}
/// A MediaBundle asset.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MediaBundleAsset {
    /// Media bundle (ZIP file) asset data. The format of the uploaded ZIP file
    /// depends on the ad field where it will be used. For more information on the
    /// format, see the documentation of the ad field where you plan on using the
    /// MediaBundleAsset. This field is mutate only.
    #[prost(bytes="vec", optional, tag="2")]
    pub data: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
/// An Image asset.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImageAsset {
    /// The raw bytes data of an image. This field is mutate only.
    #[prost(bytes="vec", optional, tag="5")]
    pub data: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    /// File size of the image asset in bytes.
    #[prost(int64, optional, tag="6")]
    pub file_size: ::core::option::Option<i64>,
    /// MIME type of the image asset.
    #[prost(enumeration="super::enums::mime_type_enum::MimeType", tag="3")]
    pub mime_type: i32,
    /// Metadata for this image at its original size.
    #[prost(message, optional, tag="4")]
    pub full_size: ::core::option::Option<ImageDimension>,
}
/// Metadata for an image at a certain size, either original or resized.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImageDimension {
    /// Height of the image.
    #[prost(int64, optional, tag="4")]
    pub height_pixels: ::core::option::Option<i64>,
    /// Width of the image.
    #[prost(int64, optional, tag="5")]
    pub width_pixels: ::core::option::Option<i64>,
    /// A URL that returns the image with this height and width.
    #[prost(string, optional, tag="6")]
    pub url: ::core::option::Option<::prost::alloc::string::String>,
}
/// A Text asset.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TextAsset {
    /// Text content of the text asset.
    #[prost(string, optional, tag="2")]
    pub text: ::core::option::Option<::prost::alloc::string::String>,
}
/// A Lead Form asset.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LeadFormAsset {
    /// Required. The name of the business being advertised.
    #[prost(string, tag="10")]
    pub business_name: ::prost::alloc::string::String,
    /// Required. Pre-defined display text that encourages user to expand the form.
    #[prost(enumeration="super::enums::lead_form_call_to_action_type_enum::LeadFormCallToActionType", tag="17")]
    pub call_to_action_type: i32,
    /// Required. Text giving a clear value proposition of what users expect once they expand
    /// the form.
    #[prost(string, tag="18")]
    pub call_to_action_description: ::prost::alloc::string::String,
    /// Required. Headline of the expanded form to describe what the form is asking for or
    /// facilitating.
    #[prost(string, tag="12")]
    pub headline: ::prost::alloc::string::String,
    /// Required. Detailed description of the expanded form to describe what the form is
    /// asking for or facilitating.
    #[prost(string, tag="13")]
    pub description: ::prost::alloc::string::String,
    /// Required. Link to a page describing the policy on how the collected data is handled
    /// by the advertiser/business.
    #[prost(string, tag="14")]
    pub privacy_policy_url: ::prost::alloc::string::String,
    /// Headline of text shown after form submission that describes how the
    /// advertiser will follow up with the user.
    #[prost(string, optional, tag="15")]
    pub post_submit_headline: ::core::option::Option<::prost::alloc::string::String>,
    /// Detailed description shown after form submission that describes how the
    /// advertiser will follow up with the user.
    #[prost(string, optional, tag="16")]
    pub post_submit_description: ::core::option::Option<::prost::alloc::string::String>,
    /// Ordered list of input fields.
    #[prost(message, repeated, tag="8")]
    pub fields: ::prost::alloc::vec::Vec<LeadFormField>,
    /// Configured methods for collected lead data to be delivered to advertiser.
    /// Only one method typed as WebhookDelivery can be configured.
    #[prost(message, repeated, tag="9")]
    pub delivery_methods: ::prost::alloc::vec::Vec<LeadFormDeliveryMethod>,
    /// Pre-defined display text that encourages user action after the form is
    /// submitted.
    #[prost(enumeration="super::enums::lead_form_post_submit_call_to_action_type_enum::LeadFormPostSubmitCallToActionType", tag="19")]
    pub post_submit_call_to_action_type: i32,
    /// Asset resource name of the background image. The minimum size is 600x314
    /// and the aspect ratio must be 1.91:1 (+-1%).
    #[prost(string, optional, tag="20")]
    pub background_image_asset: ::core::option::Option<::prost::alloc::string::String>,
    /// Desired intent for the lead form, e.g. more volume or more qualified.
    #[prost(enumeration="super::enums::lead_form_desired_intent_enum::LeadFormDesiredIntent", tag="21")]
    pub desired_intent: i32,
    /// Custom disclosure shown along with Google disclaimer on the lead form.
    /// Accessible to allowed customers only.
    #[prost(string, optional, tag="22")]
    pub custom_disclosure: ::core::option::Option<::prost::alloc::string::String>,
}
/// One input field instance within a form.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LeadFormField {
    /// Describes the input type, which may be a predefined type such as
    /// "full name" or a pre-vetted question like "Do you own a car?".
    #[prost(enumeration="super::enums::lead_form_field_user_input_type_enum::LeadFormFieldUserInputType", tag="1")]
    pub input_type: i32,
    /// Defines answer configuration that this form field accepts. If oneof is not
    /// set, this is a free-text answer.
    #[prost(oneof="lead_form_field::Answers", tags="2")]
    pub answers: ::core::option::Option<lead_form_field::Answers>,
}
/// Nested message and enum types in `LeadFormField`.
pub mod lead_form_field {
    /// Defines answer configuration that this form field accepts. If oneof is not
    /// set, this is a free-text answer.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Answers {
        /// Answer configuration for a single choice question. Can be set only for
        /// pre-vetted question fields. Minimum of 2 answers required and maximum of
        /// 12 allowed.
        #[prost(message, tag="2")]
        SingleChoiceAnswers(super::LeadFormSingleChoiceAnswers),
    }
}
/// Defines possible answers for a single choice question, usually presented as
/// a single-choice drop-down list.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LeadFormSingleChoiceAnswers {
    /// List of choices for a single question field. The order of entries defines
    /// UI order. Minimum of 2 answers required and maximum of 12 allowed.
    #[prost(string, repeated, tag="1")]
    pub answers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// A configuration of how leads are delivered to the advertiser.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LeadFormDeliveryMethod {
    /// Various subtypes of delivery.
    #[prost(oneof="lead_form_delivery_method::DeliveryDetails", tags="1")]
    pub delivery_details: ::core::option::Option<lead_form_delivery_method::DeliveryDetails>,
}
/// Nested message and enum types in `LeadFormDeliveryMethod`.
pub mod lead_form_delivery_method {
    /// Various subtypes of delivery.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum DeliveryDetails {
        /// Webhook method of delivery.
        #[prost(message, tag="1")]
        Webhook(super::WebhookDelivery),
    }
}
/// Google notifies the advertiser of leads by making HTTP calls to an
/// endpoint they specify. The requests contain JSON matching a schema that
/// Google publishes as part of form ads documentation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WebhookDelivery {
    /// Webhook url specified by advertiser to send the lead.
    #[prost(string, optional, tag="4")]
    pub advertiser_webhook_url: ::core::option::Option<::prost::alloc::string::String>,
    /// Anti-spoofing secret set by the advertiser as part of the webhook payload.
    #[prost(string, optional, tag="5")]
    pub google_secret: ::core::option::Option<::prost::alloc::string::String>,
    /// The schema version that this delivery instance will use.
    #[prost(int64, optional, tag="6")]
    pub payload_schema_version: ::core::option::Option<i64>,
}
/// A Book on Google asset. Used to redirect user to book through Google.
/// Book on Google will change the redirect url to book directly through
/// Google.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BookOnGoogleAsset {
}
/// A Promotion asset.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PromotionAsset {
    /// Required. A freeform description of what the promotion is targeting.
    #[prost(string, tag="1")]
    pub promotion_target: ::prost::alloc::string::String,
    /// A modifier for qualification of the discount.
    #[prost(enumeration="super::enums::promotion_extension_discount_modifier_enum::PromotionExtensionDiscountModifier", tag="2")]
    pub discount_modifier: i32,
    /// Start date of when the promotion is eligible to be redeemed, in yyyy-MM-dd
    /// format.
    #[prost(string, tag="7")]
    pub redemption_start_date: ::prost::alloc::string::String,
    /// Last date of when the promotion is eligible to be redeemed, in yyyy-MM-dd
    /// format.
    #[prost(string, tag="8")]
    pub redemption_end_date: ::prost::alloc::string::String,
    /// The occasion the promotion was intended for.
    /// If an occasion is set, the redemption window will need to fall within the
    /// date range associated with the occasion.
    #[prost(enumeration="super::enums::promotion_extension_occasion_enum::PromotionExtensionOccasion", tag="9")]
    pub occasion: i32,
    /// The language of the promotion.
    /// Represented as BCP 47 language tag.
    #[prost(string, tag="10")]
    pub language_code: ::prost::alloc::string::String,
    /// Start date of when this asset is effective and can begin serving, in
    /// yyyy-MM-dd format.
    #[prost(string, tag="11")]
    pub start_date: ::prost::alloc::string::String,
    /// Last date of when this asset is effective and still serving, in yyyy-MM-dd
    /// format.
    #[prost(string, tag="12")]
    pub end_date: ::prost::alloc::string::String,
    /// List of non-overlapping schedules specifying all time intervals for which
    /// the asset may serve. There can be a maximum of 6 schedules per day, 42 in
    /// total.
    #[prost(message, repeated, tag="13")]
    pub ad_schedule_targets: ::prost::alloc::vec::Vec<AdScheduleInfo>,
    /// Discount type, can be percentage off or amount off.
    #[prost(oneof="promotion_asset::DiscountType", tags="3, 4")]
    pub discount_type: ::core::option::Option<promotion_asset::DiscountType>,
    /// Promotion trigger. Can be by promotion code or promo by eligible order
    /// amount.
    #[prost(oneof="promotion_asset::PromotionTrigger", tags="5, 6")]
    pub promotion_trigger: ::core::option::Option<promotion_asset::PromotionTrigger>,
}
/// Nested message and enum types in `PromotionAsset`.
pub mod promotion_asset {
    /// Discount type, can be percentage off or amount off.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum DiscountType {
        /// Percentage off discount in the promotion. 1,000,000 = 100%.
        /// Either this or money_amount_off is required.
        #[prost(int64, tag="3")]
        PercentOff(i64),
        /// Money amount off for discount in the promotion.
        /// Either this or percent_off is required.
        #[prost(message, tag="4")]
        MoneyAmountOff(super::Money),
    }
    /// Promotion trigger. Can be by promotion code or promo by eligible order
    /// amount.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum PromotionTrigger {
        /// A code the user should use in order to be eligible for the promotion.
        #[prost(string, tag="5")]
        PromotionCode(::prost::alloc::string::String),
        /// The amount the total order needs to be for the user to be eligible for
        /// the promotion.
        #[prost(message, tag="6")]
        OrdersOverAmount(super::Money),
    }
}
/// A Callout asset.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CalloutAsset {
    /// Required. The callout text.
    /// The length of this string should be between 1 and 25, inclusive.
    #[prost(string, tag="1")]
    pub callout_text: ::prost::alloc::string::String,
    /// Start date of when this asset is effective and can begin serving, in
    /// yyyy-MM-dd format.
    #[prost(string, tag="2")]
    pub start_date: ::prost::alloc::string::String,
    /// Last date of when this asset is effective and still serving, in yyyy-MM-dd
    /// format.
    #[prost(string, tag="3")]
    pub end_date: ::prost::alloc::string::String,
    /// List of non-overlapping schedules specifying all time intervals for which
    /// the asset may serve. There can be a maximum of 6 schedules per day, 42 in
    /// total.
    #[prost(message, repeated, tag="4")]
    pub ad_schedule_targets: ::prost::alloc::vec::Vec<AdScheduleInfo>,
}
/// A Structured Snippet asset.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StructuredSnippetAsset {
    /// Required. The header of the snippet.
    /// This string should be one of the predefined values at
    /// <https://developers.google.com/google-ads/api/reference/data/structured-snippet-headers>
    #[prost(string, tag="1")]
    pub header: ::prost::alloc::string::String,
    /// Required. The values in the snippet.
    /// The size of this collection should be between 3 and 10, inclusive.
    /// The length of each value should be between 1 and 25 characters, inclusive.
    #[prost(string, repeated, tag="2")]
    pub values: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// A Sitelink asset.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SitelinkAsset {
    /// Required. URL display text for the sitelink.
    /// The length of this string should be between 1 and 25, inclusive.
    #[prost(string, tag="1")]
    pub link_text: ::prost::alloc::string::String,
    /// First line of the description for the sitelink.
    /// If set, the length should be between 1 and 35, inclusive, and description2
    /// must also be set.
    #[prost(string, tag="2")]
    pub description1: ::prost::alloc::string::String,
    /// Second line of the description for the sitelink.
    /// If set, the length should be between 1 and 35, inclusive, and description1
    /// must also be set.
    #[prost(string, tag="3")]
    pub description2: ::prost::alloc::string::String,
    /// Start date of when this asset is effective and can begin serving, in
    /// yyyy-MM-dd format.
    #[prost(string, tag="4")]
    pub start_date: ::prost::alloc::string::String,
    /// Last date of when this asset is effective and still serving, in yyyy-MM-dd
    /// format.
    #[prost(string, tag="5")]
    pub end_date: ::prost::alloc::string::String,
    /// List of non-overlapping schedules specifying all time intervals for which
    /// the asset may serve. There can be a maximum of 6 schedules per day, 42 in
    /// total.
    #[prost(message, repeated, tag="6")]
    pub ad_schedule_targets: ::prost::alloc::vec::Vec<AdScheduleInfo>,
}
/// A Page Feed asset.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PageFeedAsset {
    /// Required. The webpage that advertisers want to target.
    #[prost(string, tag="1")]
    pub page_url: ::prost::alloc::string::String,
    /// Labels used to group the page urls.
    #[prost(string, repeated, tag="2")]
    pub labels: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// A Dynamic Education asset.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DynamicEducationAsset {
    /// Required. Program ID which can be any sequence of letters and digits, and must be
    /// unique and match the values of remarketing tag. Required.
    #[prost(string, tag="1")]
    pub program_id: ::prost::alloc::string::String,
    /// Location ID which can be any sequence of letters and digits and must be
    /// unique.
    #[prost(string, tag="2")]
    pub location_id: ::prost::alloc::string::String,
    /// Required. Program name, e.g. Nursing. Required.
    #[prost(string, tag="3")]
    pub program_name: ::prost::alloc::string::String,
    /// Subject of study, e.g. Health.
    #[prost(string, tag="4")]
    pub subject: ::prost::alloc::string::String,
    /// Program description, e.g. Nursing Certification.
    #[prost(string, tag="5")]
    pub program_description: ::prost::alloc::string::String,
    /// School name, e.g. Mountain View School of Nursing.
    #[prost(string, tag="6")]
    pub school_name: ::prost::alloc::string::String,
    /// School address which can be specified in one of the following formats.
    /// (1) City, state, code, country, e.g. Mountain View, CA, USA.
    /// (2) Full address, e.g. 123 Boulevard St, Mountain View, CA 94043.
    /// (3) Latitude-longitude in the DDD format, e.g. 41.40338, 2.17403
    #[prost(string, tag="7")]
    pub address: ::prost::alloc::string::String,
    /// Contextual keywords, e.g. Nursing certification, Health, Mountain View.
    #[prost(string, repeated, tag="8")]
    pub contextual_keywords: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Android deep link, e.g.
    /// android-app://com.example.android/http/example.com/gizmos?1234.
    #[prost(string, tag="9")]
    pub android_app_link: ::prost::alloc::string::String,
    /// Similar program IDs.
    #[prost(string, repeated, tag="10")]
    pub similar_program_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// iOS deep link, e.g. exampleApp://content/page.
    #[prost(string, tag="11")]
    pub ios_app_link: ::prost::alloc::string::String,
    /// iOS app store ID. This is used to check if the user has the app installed
    /// on their device before deep linking. If this field is set, then the
    /// ios_app_link field must also be present.
    #[prost(int64, tag="12")]
    pub ios_app_store_id: i64,
    /// Thumbnail image url, e.g. <http://www.example.com/thumbnail.png.> The
    /// thumbnail image will not be uploaded as image asset.
    #[prost(string, tag="13")]
    pub thumbnail_image_url: ::prost::alloc::string::String,
    /// Image url, e.g. <http://www.example.com/image.png.> The image will not be
    /// uploaded as image asset.
    #[prost(string, tag="14")]
    pub image_url: ::prost::alloc::string::String,
}
/// An asset representing a mobile app.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MobileAppAsset {
    /// Required. A string that uniquely identifies a mobile application. It should just
    /// contain the platform native id, like "com.android.ebay" for Android or
    /// "12345689" for iOS.
    #[prost(string, tag="1")]
    pub app_id: ::prost::alloc::string::String,
    /// Required. The application store that distributes this specific app.
    #[prost(enumeration="super::enums::mobile_app_vendor_enum::MobileAppVendor", tag="2")]
    pub app_store: i32,
    /// Required. The visible text displayed when the link is rendered in an ad.
    /// The length of this string should be between 1 and 25, inclusive.
    #[prost(string, tag="3")]
    pub link_text: ::prost::alloc::string::String,
    /// Start date of when this asset is effective and can begin serving, in
    /// yyyy-MM-dd format.
    #[prost(string, tag="4")]
    pub start_date: ::prost::alloc::string::String,
    /// Last date of when this asset is effective and still serving, in yyyy-MM-dd
    /// format.
    #[prost(string, tag="5")]
    pub end_date: ::prost::alloc::string::String,
}
/// An asset representing a hotel callout.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HotelCalloutAsset {
    /// Required. The text of the hotel callout asset.
    /// The length of this string should be between 1 and 25, inclusive.
    #[prost(string, tag="1")]
    pub text: ::prost::alloc::string::String,
    /// Required. The language of the hotel callout.
    /// Represented as BCP 47 language tag.
    #[prost(string, tag="2")]
    pub language_code: ::prost::alloc::string::String,
}
/// A Call asset.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CallAsset {
    /// Required. Two-letter country code of the phone number. Examples: 'US', 'us'.
    #[prost(string, tag="1")]
    pub country_code: ::prost::alloc::string::String,
    /// Required. The advertiser's raw phone number. Examples: '1234567890', '(123)456-7890'
    #[prost(string, tag="2")]
    pub phone_number: ::prost::alloc::string::String,
    /// Indicates whether this CallAsset should use its own call conversion
    /// setting, follow the account level setting, or disable call conversion.
    #[prost(enumeration="super::enums::call_conversion_reporting_state_enum::CallConversionReportingState", tag="3")]
    pub call_conversion_reporting_state: i32,
    /// The conversion action to attribute a call conversion to. If not set, the
    /// default conversion action is used. This field only has effect if
    /// call_conversion_reporting_state is set to
    /// USE_RESOURCE_LEVEL_CALL_CONVERSION_ACTION.
    #[prost(string, tag="4")]
    pub call_conversion_action: ::prost::alloc::string::String,
    /// List of non-overlapping schedules specifying all time intervals for which
    /// the asset may serve. There can be a maximum of 6 schedules per day, 42 in
    /// total.
    #[prost(message, repeated, tag="5")]
    pub ad_schedule_targets: ::prost::alloc::vec::Vec<AdScheduleInfo>,
}
/// An asset representing a list of price offers.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PriceAsset {
    /// Required. The type of the price asset.
    #[prost(enumeration="super::enums::price_extension_type_enum::PriceExtensionType", tag="1")]
    pub r#type: i32,
    /// The price qualifier of the price asset.
    #[prost(enumeration="super::enums::price_extension_price_qualifier_enum::PriceExtensionPriceQualifier", tag="2")]
    pub price_qualifier: i32,
    /// Required. The language of the price asset.
    /// Represented as BCP 47 language tag.
    #[prost(string, tag="3")]
    pub language_code: ::prost::alloc::string::String,
    /// The price offerings of the price asset.
    /// The size of this collection should be between 3 and 8, inclusive.
    #[prost(message, repeated, tag="4")]
    pub price_offerings: ::prost::alloc::vec::Vec<PriceOffering>,
}
/// A single price offering within a PriceAsset.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PriceOffering {
    /// Required. The header of the price offering.
    /// The length of this string should be between 1 and 25, inclusive.
    #[prost(string, tag="1")]
    pub header: ::prost::alloc::string::String,
    /// Required. The description of the price offering.
    /// The length of this string should be between 1 and 25, inclusive.
    #[prost(string, tag="2")]
    pub description: ::prost::alloc::string::String,
    /// Required. The price value of the price offering.
    #[prost(message, optional, tag="3")]
    pub price: ::core::option::Option<Money>,
    /// The price unit of the price offering.
    #[prost(enumeration="super::enums::price_extension_price_unit_enum::PriceExtensionPriceUnit", tag="4")]
    pub unit: i32,
    /// Required. The final URL after all cross domain redirects.
    #[prost(string, tag="5")]
    pub final_url: ::prost::alloc::string::String,
    /// The final mobile URL after all cross domain redirects.
    #[prost(string, tag="6")]
    pub final_mobile_url: ::prost::alloc::string::String,
}
/// A call to action asset.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CallToActionAsset {
    /// Call to action.
    #[prost(enumeration="super::enums::call_to_action_type_enum::CallToActionType", tag="1")]
    pub call_to_action: i32,
}
// Proto file describing bidding schemes.

/// Commission is an automatic bidding strategy in which the advertiser pays a
/// certain portion of the conversion value.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Commission {
    /// Commission rate defines the portion of the conversion value that the
    /// advertiser will be billed. A commission rate of x should be passed into
    /// this field as (x * 1,000,000). For example, 106,000 represents a commission
    /// rate of 0.106 (10.6%).
    #[prost(int64, optional, tag="2")]
    pub commission_rate_micros: ::core::option::Option<i64>,
}
/// An automated bidding strategy that raises bids for clicks
/// that seem more likely to lead to a conversion and lowers
/// them for clicks where they seem less likely.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EnhancedCpc {
}
/// Manual click-based bidding where user pays per click.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ManualCpc {
    /// Whether bids are to be enhanced based on conversion optimizer data.
    #[prost(bool, optional, tag="2")]
    pub enhanced_cpc_enabled: ::core::option::Option<bool>,
}
/// Manual impression-based bidding where user pays per thousand impressions.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ManualCpm {
}
/// View based bidding where user pays per video view.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ManualCpv {
}
/// An automated bidding strategy to help get the most conversions for your
/// campaigns while spending your budget.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MaximizeConversions {
    /// The target cost-per-action (CPA) option. This is the average amount that
    /// you would like to spend per conversion action. If set, the bid strategy
    /// will get as many conversions as possible at or below the target
    /// cost-per-action. If the target CPA is not set, the bid strategy will
    /// aim to achieve the lowest possible CPA given the budget.
    #[prost(int64, tag="1")]
    pub target_cpa: i64,
    /// Maximum bid limit that can be set by the bid strategy.
    /// The limit applies to all keywords managed by the strategy.
    /// Mutable for portfolio bidding strategies only.
    #[prost(int64, tag="2")]
    pub cpc_bid_ceiling_micros: i64,
    /// Minimum bid limit that can be set by the bid strategy.
    /// The limit applies to all keywords managed by the strategy.
    /// Mutable for portfolio bidding strategies only.
    #[prost(int64, tag="3")]
    pub cpc_bid_floor_micros: i64,
}
/// An automated bidding strategy to help get the most conversion value for your
/// campaigns while spending your budget.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MaximizeConversionValue {
    /// The target return on ad spend (ROAS) option. If set, the bid strategy will
    /// maximize revenue while averaging the target return on ad spend. If the
    /// target ROAS is high, the bid strategy may not be able to spend the full
    /// budget. If the target ROAS is not set, the bid strategy will aim to
    /// achieve the highest possible ROAS for the budget.
    #[prost(double, tag="2")]
    pub target_roas: f64,
    /// Maximum bid limit that can be set by the bid strategy.
    /// The limit applies to all keywords managed by the strategy.
    /// Mutable for portfolio bidding strategies only.
    #[prost(int64, tag="3")]
    pub cpc_bid_ceiling_micros: i64,
    /// Minimum bid limit that can be set by the bid strategy.
    /// The limit applies to all keywords managed by the strategy.
    /// Mutable for portfolio bidding strategies only.
    #[prost(int64, tag="4")]
    pub cpc_bid_floor_micros: i64,
}
/// An automated bid strategy that sets bids to help get as many conversions as
/// possible at the target cost-per-acquisition (CPA) you set.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TargetCpa {
    /// Average CPA target.
    /// This target should be greater than or equal to minimum billable unit based
    /// on the currency for the account.
    #[prost(int64, optional, tag="4")]
    pub target_cpa_micros: ::core::option::Option<i64>,
    /// Maximum bid limit that can be set by the bid strategy.
    /// The limit applies to all keywords managed by the strategy.
    /// This should only be set for portfolio bid strategies.
    #[prost(int64, optional, tag="5")]
    pub cpc_bid_ceiling_micros: ::core::option::Option<i64>,
    /// Minimum bid limit that can be set by the bid strategy.
    /// The limit applies to all keywords managed by the strategy.
    /// This should only be set for portfolio bid strategies.
    #[prost(int64, optional, tag="6")]
    pub cpc_bid_floor_micros: ::core::option::Option<i64>,
}
/// Target CPM (cost per thousand impressions) is an automated bidding strategy
/// that sets bids to optimize performance given the target CPM you set.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TargetCpm {
}
/// An automated bidding strategy that sets bids so that a certain percentage of
/// search ads are shown at the top of the first page (or other targeted
/// location).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TargetImpressionShare {
    /// The targeted location on the search results page.
    #[prost(enumeration="super::enums::target_impression_share_location_enum::TargetImpressionShareLocation", tag="1")]
    pub location: i32,
    /// The desired fraction of ads to be shown in the targeted location in micros.
    /// E.g. 1% equals 10,000.
    #[prost(int64, optional, tag="4")]
    pub location_fraction_micros: ::core::option::Option<i64>,
    /// The highest CPC bid the automated bidding system is permitted to specify.
    /// This is a required field entered by the advertiser that sets the ceiling
    /// and specified in local micros.
    #[prost(int64, optional, tag="5")]
    pub cpc_bid_ceiling_micros: ::core::option::Option<i64>,
}
/// An automated bidding strategy that helps you maximize revenue while
/// averaging a specific target return on ad spend (ROAS).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TargetRoas {
    /// Required. The desired revenue (based on conversion data) per unit of spend.
    /// Value must be between 0.01 and 1000.0, inclusive.
    #[prost(double, optional, tag="4")]
    pub target_roas: ::core::option::Option<f64>,
    /// Maximum bid limit that can be set by the bid strategy.
    /// The limit applies to all keywords managed by the strategy.
    /// This should only be set for portfolio bid strategies.
    #[prost(int64, optional, tag="5")]
    pub cpc_bid_ceiling_micros: ::core::option::Option<i64>,
    /// Minimum bid limit that can be set by the bid strategy.
    /// The limit applies to all keywords managed by the strategy.
    /// This should only be set for portfolio bid strategies.
    #[prost(int64, optional, tag="6")]
    pub cpc_bid_floor_micros: ::core::option::Option<i64>,
}
/// An automated bid strategy that sets your bids to help get as many clicks
/// as possible within your budget.
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
    #[prost(int64, optional, tag="3")]
    pub target_spend_micros: ::core::option::Option<i64>,
    /// Maximum bid limit that can be set by the bid strategy.
    /// The limit applies to all keywords managed by the strategy.
    #[prost(int64, optional, tag="4")]
    pub cpc_bid_ceiling_micros: ::core::option::Option<i64>,
}
/// A bidding strategy where bids are a fraction of the advertised price for
/// some good or service.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PercentCpc {
    /// Maximum bid limit that can be set by the bid strategy. This is
    /// an optional field entered by the advertiser and specified in local micros.
    /// Note: A zero value is interpreted in the same way as having bid_ceiling
    /// undefined.
    #[prost(int64, optional, tag="3")]
    pub cpc_bid_ceiling_micros: ::core::option::Option<i64>,
    /// Adjusts the bid for each auction upward or downward, depending on the
    /// likelihood of a conversion. Individual bids may exceed
    /// cpc_bid_ceiling_micros, but the average bid amount for a campaign should
    /// not.
    #[prost(bool, optional, tag="4")]
    pub enhanced_cpc_enabled: ::core::option::Option<bool>,
}
// Proto file describing a ClickLocation.

/// Location criteria associated with a click.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClickLocation {
    /// The city location criterion associated with the impression.
    #[prost(string, optional, tag="6")]
    pub city: ::core::option::Option<::prost::alloc::string::String>,
    /// The country location criterion associated with the impression.
    #[prost(string, optional, tag="7")]
    pub country: ::core::option::Option<::prost::alloc::string::String>,
    /// The metro location criterion associated with the impression.
    #[prost(string, optional, tag="8")]
    pub metro: ::core::option::Option<::prost::alloc::string::String>,
    /// The most specific location criterion associated with the impression.
    #[prost(string, optional, tag="9")]
    pub most_specific: ::core::option::Option<::prost::alloc::string::String>,
    /// The region location criterion associated with the impression.
    #[prost(string, optional, tag="10")]
    pub region: ::core::option::Option<::prost::alloc::string::String>,
}
// Proto file describing criterion category availability information.

/// Information of category availability, per advertising channel.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CriterionCategoryAvailability {
    /// Channel types and subtypes that are available to the category.
    #[prost(message, optional, tag="1")]
    pub channel: ::core::option::Option<CriterionCategoryChannelAvailability>,
    /// Locales that are available to the category for the channel.
    #[prost(message, repeated, tag="2")]
    pub locale: ::prost::alloc::vec::Vec<CriterionCategoryLocaleAvailability>,
}
/// Information of advertising channel type and subtypes a category is available
/// in.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CriterionCategoryChannelAvailability {
    /// Format of the channel availability. Can be ALL_CHANNELS (the rest of the
    /// fields will not be set), CHANNEL_TYPE (only advertising_channel_type type
    /// will be set, the category is available to all sub types under it) or
    /// CHANNEL_TYPE_AND_SUBTYPES (advertising_channel_type,
    /// advertising_channel_sub_type, and include_default_channel_sub_type will all
    /// be set).
    #[prost(enumeration="super::enums::criterion_category_channel_availability_mode_enum::CriterionCategoryChannelAvailabilityMode", tag="1")]
    pub availability_mode: i32,
    /// Channel type the category is available to.
    #[prost(enumeration="super::enums::advertising_channel_type_enum::AdvertisingChannelType", tag="2")]
    pub advertising_channel_type: i32,
    /// Channel subtypes under the channel type the category is available to.
    #[prost(enumeration="super::enums::advertising_channel_sub_type_enum::AdvertisingChannelSubType", repeated, tag="3")]
    pub advertising_channel_sub_type: ::prost::alloc::vec::Vec<i32>,
    /// Whether default channel sub type is included. For example,
    /// advertising_channel_type being DISPLAY and include_default_channel_sub_type
    /// being false means that the default display campaign where channel sub type
    /// is not set is not included in this availability configuration.
    #[prost(bool, optional, tag="5")]
    pub include_default_channel_sub_type: ::core::option::Option<bool>,
}
/// Information about which locales a category is available in.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CriterionCategoryLocaleAvailability {
    /// Format of the locale availability. Can be LAUNCHED_TO_ALL (both country and
    /// language will be empty), COUNTRY (only country will be set), LANGUAGE (only
    /// language wil be set), COUNTRY_AND_LANGUAGE (both country and language will
    /// be set).
    #[prost(enumeration="super::enums::criterion_category_locale_availability_mode_enum::CriterionCategoryLocaleAvailabilityMode", tag="1")]
    pub availability_mode: i32,
    /// Code of the country.
    #[prost(string, optional, tag="4")]
    pub country_code: ::core::option::Option<::prost::alloc::string::String>,
    /// Code of the language.
    #[prost(string, optional, tag="5")]
    pub language_code: ::core::option::Option<::prost::alloc::string::String>,
}
// Proto file describing CustomParameter and operation

/// A mapping that can be used by custom parameter tags in a
/// `tracking_url_template`, `final_urls`, or `mobile_final_urls`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomParameter {
    /// The key matching the parameter tag name.
    #[prost(string, optional, tag="3")]
    pub key: ::core::option::Option<::prost::alloc::string::String>,
    /// The value to be substituted.
    #[prost(string, optional, tag="4")]
    pub value: ::core::option::Option<::prost::alloc::string::String>,
}
// Proto file describing common customizer value proto messages.

/// A customizer value that is referenced in customizer linkage entities
/// like CustomerCustomizer, CampaignCustomizer, etc.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomizerValue {
    /// Required. The data type for the customizer value. It must match the attribute type.
    /// The string_value content must match the constraints associated with the
    /// type.
    #[prost(enumeration="super::enums::customizer_attribute_type_enum::CustomizerAttributeType", tag="1")]
    pub r#type: i32,
    /// Required. Value to insert in creative text. Customizer values of all types are stored
    /// as string to make formatting unambiguous.
    #[prost(string, tag="2")]
    pub string_value: ::prost::alloc::string::String,
}
// Proto file describing date range message.

/// A date range.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DateRange {
    /// The start date, in yyyy-mm-dd format. This date is inclusive.
    #[prost(string, optional, tag="3")]
    pub start_date: ::core::option::Option<::prost::alloc::string::String>,
    /// The end date, in yyyy-mm-dd format. This date is inclusive.
    #[prost(string, optional, tag="4")]
    pub end_date: ::core::option::Option<::prost::alloc::string::String>,
}
/// The year month range inclusive of the start and end months.
/// Eg: A year month range to represent Jan 2020 would be: (Jan 2020, Jan 2020).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct YearMonthRange {
    /// The inclusive start year month.
    #[prost(message, optional, tag="1")]
    pub start: ::core::option::Option<YearMonth>,
    /// The inclusive end year month.
    #[prost(message, optional, tag="2")]
    pub end: ::core::option::Option<YearMonth>,
}
/// Year month.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct YearMonth {
    /// The year (e.g. 2020).
    #[prost(int64, tag="1")]
    pub year: i64,
    /// The month of the year. (e.g. FEBRUARY).
    #[prost(enumeration="super::enums::month_of_year_enum::MonthOfYear", tag="2")]
    pub month: i32,
}
// Proto file describing ExplorerAutoOptimizerSetting

/// Settings for the Display Campaign Optimizer, initially named "Explorer".
/// Learn more about
/// [automatic targeting](<https://support.google.com/google-ads/answer/190596>).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExplorerAutoOptimizerSetting {
    /// Indicates whether the optimizer is turned on.
    #[prost(bool, optional, tag="2")]
    pub opt_in: ::core::option::Option<bool>,
}
// Proto file describing extension types.

/// Represents an App extension.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AppFeedItem {
    /// The visible text displayed when the link is rendered in an ad.
    /// This string must not be empty, and the length of this string should
    /// be between 1 and 25, inclusive.
    #[prost(string, optional, tag="9")]
    pub link_text: ::core::option::Option<::prost::alloc::string::String>,
    /// The store-specific ID for the target application.
    /// This string must not be empty.
    #[prost(string, optional, tag="10")]
    pub app_id: ::core::option::Option<::prost::alloc::string::String>,
    /// The application store that the target application belongs to.
    /// This field is required.
    #[prost(enumeration="super::enums::app_store_enum::AppStore", tag="3")]
    pub app_store: i32,
    /// A list of possible final URLs after all cross domain redirects.
    /// This list must not be empty.
    #[prost(string, repeated, tag="11")]
    pub final_urls: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// A list of possible final mobile URLs after all cross domain redirects.
    #[prost(string, repeated, tag="12")]
    pub final_mobile_urls: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// URL template for constructing a tracking URL. Default value is "{lpurl}".
    #[prost(string, optional, tag="13")]
    pub tracking_url_template: ::core::option::Option<::prost::alloc::string::String>,
    /// A list of mappings to be used for substituting URL custom parameter tags in
    /// the tracking_url_template, final_urls, and/or final_mobile_urls.
    #[prost(message, repeated, tag="7")]
    pub url_custom_parameters: ::prost::alloc::vec::Vec<CustomParameter>,
    /// URL template for appending params to landing page URLs served with parallel
    /// tracking.
    #[prost(string, optional, tag="14")]
    pub final_url_suffix: ::core::option::Option<::prost::alloc::string::String>,
}
/// Represents a Call extension.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CallFeedItem {
    /// The advertiser's phone number to append to the ad.
    /// This string must not be empty.
    #[prost(string, optional, tag="7")]
    pub phone_number: ::core::option::Option<::prost::alloc::string::String>,
    /// Uppercase two-letter country code of the advertiser's phone number.
    /// This string must not be empty.
    #[prost(string, optional, tag="8")]
    pub country_code: ::core::option::Option<::prost::alloc::string::String>,
    /// Indicates whether call tracking is enabled. By default, call tracking is
    /// not enabled.
    #[prost(bool, optional, tag="9")]
    pub call_tracking_enabled: ::core::option::Option<bool>,
    /// The conversion action to attribute a call conversion to. If not set a
    /// default conversion action is used. This field only has effect if
    /// call_tracking_enabled is set to true. Otherwise this field is ignored.
    #[prost(string, optional, tag="10")]
    pub call_conversion_action: ::core::option::Option<::prost::alloc::string::String>,
    /// If true, disable call conversion tracking. call_conversion_action should
    /// not be set if this is true. Optional.
    #[prost(bool, optional, tag="11")]
    pub call_conversion_tracking_disabled: ::core::option::Option<bool>,
    /// Enum value that indicates whether this call extension uses its own call
    /// conversion setting (or just have call conversion disabled), or following
    /// the account level setting.
    #[prost(enumeration="super::enums::call_conversion_reporting_state_enum::CallConversionReportingState", tag="6")]
    pub call_conversion_reporting_state: i32,
}
/// Represents a callout extension.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CalloutFeedItem {
    /// The callout text.
    /// The length of this string should be between 1 and 25, inclusive.
    #[prost(string, optional, tag="2")]
    pub callout_text: ::core::option::Option<::prost::alloc::string::String>,
}
/// Represents a location extension.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LocationFeedItem {
    /// The name of the business.
    #[prost(string, optional, tag="9")]
    pub business_name: ::core::option::Option<::prost::alloc::string::String>,
    /// Line 1 of the business address.
    #[prost(string, optional, tag="10")]
    pub address_line_1: ::core::option::Option<::prost::alloc::string::String>,
    /// Line 2 of the business address.
    #[prost(string, optional, tag="11")]
    pub address_line_2: ::core::option::Option<::prost::alloc::string::String>,
    /// City of the business address.
    #[prost(string, optional, tag="12")]
    pub city: ::core::option::Option<::prost::alloc::string::String>,
    /// Province of the business address.
    #[prost(string, optional, tag="13")]
    pub province: ::core::option::Option<::prost::alloc::string::String>,
    /// Postal code of the business address.
    #[prost(string, optional, tag="14")]
    pub postal_code: ::core::option::Option<::prost::alloc::string::String>,
    /// Country code of the business address.
    #[prost(string, optional, tag="15")]
    pub country_code: ::core::option::Option<::prost::alloc::string::String>,
    /// Phone number of the business.
    #[prost(string, optional, tag="16")]
    pub phone_number: ::core::option::Option<::prost::alloc::string::String>,
}
/// Represents an affiliate location extension.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AffiliateLocationFeedItem {
    /// The name of the business.
    #[prost(string, optional, tag="11")]
    pub business_name: ::core::option::Option<::prost::alloc::string::String>,
    /// Line 1 of the business address.
    #[prost(string, optional, tag="12")]
    pub address_line_1: ::core::option::Option<::prost::alloc::string::String>,
    /// Line 2 of the business address.
    #[prost(string, optional, tag="13")]
    pub address_line_2: ::core::option::Option<::prost::alloc::string::String>,
    /// City of the business address.
    #[prost(string, optional, tag="14")]
    pub city: ::core::option::Option<::prost::alloc::string::String>,
    /// Province of the business address.
    #[prost(string, optional, tag="15")]
    pub province: ::core::option::Option<::prost::alloc::string::String>,
    /// Postal code of the business address.
    #[prost(string, optional, tag="16")]
    pub postal_code: ::core::option::Option<::prost::alloc::string::String>,
    /// Country code of the business address.
    #[prost(string, optional, tag="17")]
    pub country_code: ::core::option::Option<::prost::alloc::string::String>,
    /// Phone number of the business.
    #[prost(string, optional, tag="18")]
    pub phone_number: ::core::option::Option<::prost::alloc::string::String>,
    /// Id of the retail chain that is advertised as a seller of your product.
    #[prost(int64, optional, tag="19")]
    pub chain_id: ::core::option::Option<i64>,
    /// Name of chain.
    #[prost(string, optional, tag="20")]
    pub chain_name: ::core::option::Option<::prost::alloc::string::String>,
}
/// An extension that users can click on to send a text message to the
/// advertiser.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TextMessageFeedItem {
    /// The business name to prepend to the message text.
    /// This field is required.
    #[prost(string, optional, tag="6")]
    pub business_name: ::core::option::Option<::prost::alloc::string::String>,
    /// Uppercase two-letter country code of the advertiser's phone number.
    /// This field is required.
    #[prost(string, optional, tag="7")]
    pub country_code: ::core::option::Option<::prost::alloc::string::String>,
    /// The advertiser's phone number the message will be sent to. Required.
    #[prost(string, optional, tag="8")]
    pub phone_number: ::core::option::Option<::prost::alloc::string::String>,
    /// The text to show in the ad.
    /// This field is required.
    #[prost(string, optional, tag="9")]
    pub text: ::core::option::Option<::prost::alloc::string::String>,
    /// The message extension_text populated in the messaging app.
    #[prost(string, optional, tag="10")]
    pub extension_text: ::core::option::Option<::prost::alloc::string::String>,
}
/// Represents a Price extension.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PriceFeedItem {
    /// Price extension type of this extension.
    #[prost(enumeration="super::enums::price_extension_type_enum::PriceExtensionType", tag="1")]
    pub r#type: i32,
    /// Price qualifier for all offers of this price extension.
    #[prost(enumeration="super::enums::price_extension_price_qualifier_enum::PriceExtensionPriceQualifier", tag="2")]
    pub price_qualifier: i32,
    /// Tracking URL template for all offers of this price extension.
    #[prost(string, optional, tag="7")]
    pub tracking_url_template: ::core::option::Option<::prost::alloc::string::String>,
    /// The code of the language used for this price extension.
    #[prost(string, optional, tag="8")]
    pub language_code: ::core::option::Option<::prost::alloc::string::String>,
    /// The price offerings in this price extension.
    #[prost(message, repeated, tag="5")]
    pub price_offerings: ::prost::alloc::vec::Vec<PriceOffer>,
    /// Tracking URL template for all offers of this price extension.
    #[prost(string, optional, tag="9")]
    pub final_url_suffix: ::core::option::Option<::prost::alloc::string::String>,
}
/// Represents one price offer in a price extension.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PriceOffer {
    /// Header text of this offer.
    #[prost(string, optional, tag="7")]
    pub header: ::core::option::Option<::prost::alloc::string::String>,
    /// Description text of this offer.
    #[prost(string, optional, tag="8")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
    /// Price value of this offer.
    #[prost(message, optional, tag="3")]
    pub price: ::core::option::Option<Money>,
    /// Price unit for this offer.
    #[prost(enumeration="super::enums::price_extension_price_unit_enum::PriceExtensionPriceUnit", tag="4")]
    pub unit: i32,
    /// A list of possible final URLs after all cross domain redirects.
    #[prost(string, repeated, tag="9")]
    pub final_urls: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// A list of possible final mobile URLs after all cross domain redirects.
    #[prost(string, repeated, tag="10")]
    pub final_mobile_urls: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Represents a Promotion extension.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PromotionFeedItem {
    /// A freeform description of what the promotion is targeting.
    /// This field is required.
    #[prost(string, optional, tag="16")]
    pub promotion_target: ::core::option::Option<::prost::alloc::string::String>,
    /// Enum that modifies the qualification of the discount.
    #[prost(enumeration="super::enums::promotion_extension_discount_modifier_enum::PromotionExtensionDiscountModifier", tag="2")]
    pub discount_modifier: i32,
    /// Start date of when the promotion is eligible to be redeemed.
    #[prost(string, optional, tag="19")]
    pub promotion_start_date: ::core::option::Option<::prost::alloc::string::String>,
    /// Last date when the promotion is eligible to be redeemed.
    #[prost(string, optional, tag="20")]
    pub promotion_end_date: ::core::option::Option<::prost::alloc::string::String>,
    /// The occasion the promotion was intended for.
    /// If an occasion is set, the redemption window will need to fall within
    /// the date range associated with the occasion.
    #[prost(enumeration="super::enums::promotion_extension_occasion_enum::PromotionExtensionOccasion", tag="9")]
    pub occasion: i32,
    /// A list of possible final URLs after all cross domain redirects.
    /// This field is required.
    #[prost(string, repeated, tag="21")]
    pub final_urls: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// A list of possible final mobile URLs after all cross domain redirects.
    #[prost(string, repeated, tag="22")]
    pub final_mobile_urls: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// URL template for constructing a tracking URL.
    #[prost(string, optional, tag="23")]
    pub tracking_url_template: ::core::option::Option<::prost::alloc::string::String>,
    /// A list of mappings to be used for substituting URL custom parameter tags in
    /// the tracking_url_template, final_urls, and/or final_mobile_urls.
    #[prost(message, repeated, tag="13")]
    pub url_custom_parameters: ::prost::alloc::vec::Vec<CustomParameter>,
    /// URL template for appending params to landing page URLs served with parallel
    /// tracking.
    #[prost(string, optional, tag="24")]
    pub final_url_suffix: ::core::option::Option<::prost::alloc::string::String>,
    /// The language of the promotion.
    /// Represented as BCP 47 language tag.
    #[prost(string, optional, tag="25")]
    pub language_code: ::core::option::Option<::prost::alloc::string::String>,
    /// Discount type, can be percentage off or amount off.
    #[prost(oneof="promotion_feed_item::DiscountType", tags="17, 4")]
    pub discount_type: ::core::option::Option<promotion_feed_item::DiscountType>,
    /// Promotion trigger. Can be by promotion code or promo by eligible order
    /// amount.
    #[prost(oneof="promotion_feed_item::PromotionTrigger", tags="18, 6")]
    pub promotion_trigger: ::core::option::Option<promotion_feed_item::PromotionTrigger>,
}
/// Nested message and enum types in `PromotionFeedItem`.
pub mod promotion_feed_item {
    /// Discount type, can be percentage off or amount off.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum DiscountType {
        /// Percentage off discount in the promotion in micros.
        /// One million is equivalent to one percent.
        /// Either this or money_off_amount is required.
        #[prost(int64, tag="17")]
        PercentOff(i64),
        /// Money amount off for discount in the promotion.
        /// Either this or percent_off is required.
        #[prost(message, tag="4")]
        MoneyAmountOff(super::Money),
    }
    /// Promotion trigger. Can be by promotion code or promo by eligible order
    /// amount.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum PromotionTrigger {
        /// A code the user should use in order to be eligible for the promotion.
        #[prost(string, tag="18")]
        PromotionCode(::prost::alloc::string::String),
        /// The amount the total order needs to be for the user to be eligible for
        /// the promotion.
        #[prost(message, tag="6")]
        OrdersOverAmount(super::Money),
    }
}
/// Represents a structured snippet extension.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StructuredSnippetFeedItem {
    /// The header of the snippet.
    /// This string must not be empty.
    #[prost(string, optional, tag="3")]
    pub header: ::core::option::Option<::prost::alloc::string::String>,
    /// The values in the snippet.
    /// The maximum size of this collection is 10.
    #[prost(string, repeated, tag="4")]
    pub values: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Represents a sitelink extension.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SitelinkFeedItem {
    /// URL display text for the sitelink.
    /// The length of this string should be between 1 and 25, inclusive.
    #[prost(string, optional, tag="9")]
    pub link_text: ::core::option::Option<::prost::alloc::string::String>,
    /// First line of the description for the sitelink.
    /// If this value is set, line2 must also be set.
    /// The length of this string should be between 0 and 35, inclusive.
    #[prost(string, optional, tag="10")]
    pub line1: ::core::option::Option<::prost::alloc::string::String>,
    /// Second line of the description for the sitelink.
    /// If this value is set, line1 must also be set.
    /// The length of this string should be between 0 and 35, inclusive.
    #[prost(string, optional, tag="11")]
    pub line2: ::core::option::Option<::prost::alloc::string::String>,
    /// A list of possible final URLs after all cross domain redirects.
    #[prost(string, repeated, tag="12")]
    pub final_urls: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// A list of possible final mobile URLs after all cross domain redirects.
    #[prost(string, repeated, tag="13")]
    pub final_mobile_urls: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// URL template for constructing a tracking URL.
    #[prost(string, optional, tag="14")]
    pub tracking_url_template: ::core::option::Option<::prost::alloc::string::String>,
    /// A list of mappings to be used for substituting URL custom parameter tags in
    /// the tracking_url_template, final_urls, and/or final_mobile_urls.
    #[prost(message, repeated, tag="7")]
    pub url_custom_parameters: ::prost::alloc::vec::Vec<CustomParameter>,
    /// Final URL suffix to be appended to landing page URLs served with
    /// parallel tracking.
    #[prost(string, optional, tag="15")]
    pub final_url_suffix: ::core::option::Option<::prost::alloc::string::String>,
}
/// Represents a hotel callout extension.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HotelCalloutFeedItem {
    /// The callout text.
    /// The length of this string should be between 1 and 25, inclusive.
    #[prost(string, optional, tag="3")]
    pub text: ::core::option::Option<::prost::alloc::string::String>,
    /// The language of the hotel callout text.
    /// IETF BCP 47 compliant language code.
    #[prost(string, optional, tag="4")]
    pub language_code: ::core::option::Option<::prost::alloc::string::String>,
}
/// Represents an advertiser provided image extension.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImageFeedItem {
    /// Required. Resource name of the image asset.
    #[prost(string, tag="1")]
    pub image_asset: ::prost::alloc::string::String,
}
/// Represents a filter on locations in a feed item set.
/// Only applicable if the parent Feed of the FeedItemSet is a LOCATION feed.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DynamicLocationSetFilter {
    /// If multiple labels are set, then only feeditems marked with all the labels
    /// will be added to the FeedItemSet.
    #[prost(string, repeated, tag="1")]
    pub labels: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Business name filter.
    #[prost(message, optional, tag="2")]
    pub business_name_filter: ::core::option::Option<BusinessNameFilter>,
}
/// Represents a business name filter on locations in a FeedItemSet.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BusinessNameFilter {
    /// Business name string to use for filtering.
    #[prost(string, tag="1")]
    pub business_name: ::prost::alloc::string::String,
    /// The type of string matching to use when filtering with business_name.
    #[prost(enumeration="super::enums::feed_item_set_string_filter_type_enum::FeedItemSetStringFilterType", tag="2")]
    pub filter_type: i32,
}
/// Represents a filter on affiliate locations in a FeedItemSet.
/// Only applicable if the parent Feed of the FeedItemSet is an
/// AFFILIATE_LOCATION feed.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DynamicAffiliateLocationSetFilter {
    /// Used to filter affiliate locations by chain ids. Only affiliate locations
    /// that belong to the specified chain(s) will be added to the FeedItemSet.
    #[prost(int64, repeated, tag="1")]
    pub chain_ids: ::prost::alloc::vec::Vec<i64>,
}
// Proto file FinalAppUrl type.

/// A URL for deep linking into an app for the given operating system.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FinalAppUrl {
    /// The operating system targeted by this URL. Required.
    #[prost(enumeration="super::enums::app_url_operating_system_type_enum::AppUrlOperatingSystemType", tag="1")]
    pub os_type: i32,
    /// The app deep link URL. Deep links specify a location in an app that
    /// corresponds to the content you'd like to show, and should be of the form
    /// {scheme}://{host_path}
    /// The scheme identifies which app to open. For your app, you can use a custom
    /// scheme that starts with the app's name. The host and path specify the
    /// unique location in the app where your content exists.
    /// Example: "exampleapp://productid_1234". Required.
    #[prost(string, optional, tag="3")]
    pub url: ::core::option::Option<::prost::alloc::string::String>,
}
// Proto file describing frequency caps.

/// A rule specifying the maximum number of times an ad (or some set of ads) can
/// be shown to a user over a particular time period.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FrequencyCapEntry {
    /// The key of a particular frequency cap. There can be no more
    /// than one frequency cap with the same key.
    #[prost(message, optional, tag="1")]
    pub key: ::core::option::Option<FrequencyCapKey>,
    /// Maximum number of events allowed during the time range by this cap.
    #[prost(int32, optional, tag="3")]
    pub cap: ::core::option::Option<i32>,
}
/// A group of fields used as keys for a frequency cap.
/// There can be no more than one frequency cap with the same key.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FrequencyCapKey {
    /// The level on which the cap is to be applied (e.g. ad group ad, ad group).
    /// The cap is applied to all the entities of this level.
    #[prost(enumeration="super::enums::frequency_cap_level_enum::FrequencyCapLevel", tag="1")]
    pub level: i32,
    /// The type of event that the cap applies to (e.g. impression).
    #[prost(enumeration="super::enums::frequency_cap_event_type_enum::FrequencyCapEventType", tag="3")]
    pub event_type: i32,
    /// Unit of time the cap is defined at (e.g. day, week).
    #[prost(enumeration="super::enums::frequency_cap_time_unit_enum::FrequencyCapTimeUnit", tag="2")]
    pub time_unit: i32,
    /// Number of time units the cap lasts.
    #[prost(int32, optional, tag="5")]
    pub time_length: ::core::option::Option<i32>,
}
// Proto file describing Keyword Planner messages.

/// Historical metrics specific to the targeting options selected.
/// Targeting options include geographies, network, etc.
/// Refer to <https://support.google.com/google-ads/answer/3022575> for more
/// details.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeywordPlanHistoricalMetrics {
    /// Approximate number of monthly searches on this query averaged
    /// for the past 12 months.
    #[prost(int64, optional, tag="7")]
    pub avg_monthly_searches: ::core::option::Option<i64>,
    /// Approximate number of searches on this query for the past twelve months.
    #[prost(message, repeated, tag="6")]
    pub monthly_search_volumes: ::prost::alloc::vec::Vec<MonthlySearchVolume>,
    /// The competition level for the query.
    #[prost(enumeration="super::enums::keyword_plan_competition_level_enum::KeywordPlanCompetitionLevel", tag="2")]
    pub competition: i32,
    /// The competition index for the query in the range [0, 100].
    /// Shows how competitive ad placement is for a keyword.
    /// The level of competition from 0-100 is determined by the number of ad slots
    /// filled divided by the total number of ad slots available. If not enough
    /// data is available, null is returned.
    #[prost(int64, optional, tag="8")]
    pub competition_index: ::core::option::Option<i64>,
    /// Top of page bid low range (20th percentile) in micros for the keyword.
    #[prost(int64, optional, tag="9")]
    pub low_top_of_page_bid_micros: ::core::option::Option<i64>,
    /// Top of page bid high range (80th percentile) in micros for the keyword.
    #[prost(int64, optional, tag="10")]
    pub high_top_of_page_bid_micros: ::core::option::Option<i64>,
}
/// Historical metrics options.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HistoricalMetricsOptions {
    /// The year month range for historical metrics. If not specified the searches
    /// will be returned for past 12 months.
    /// Searches data is available for the past 4 years. If the search volume is
    /// not available for the entire year_month_range provided, the subset of the
    /// year month range for which search volume is available will be returned.
    #[prost(message, optional, tag="1")]
    pub year_month_range: ::core::option::Option<YearMonthRange>,
}
/// Monthly search volume.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MonthlySearchVolume {
    /// The year of the search volume (e.g. 2020).
    #[prost(int64, optional, tag="4")]
    pub year: ::core::option::Option<i64>,
    /// The month of the search volume.
    #[prost(enumeration="super::enums::month_of_year_enum::MonthOfYear", tag="2")]
    pub month: i32,
    /// Approximate number of searches for the month.
    /// A null value indicates the search volume is unavailable for
    /// that month.
    #[prost(int64, optional, tag="5")]
    pub monthly_searches: ::core::option::Option<i64>,
}
/// The aggregate metrics specification of the request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeywordPlanAggregateMetrics {
    /// The list of aggregate metrics to fetch data.
    #[prost(enumeration="super::enums::keyword_plan_aggregate_metric_type_enum::KeywordPlanAggregateMetricType", repeated, tag="1")]
    pub aggregate_metric_types: ::prost::alloc::vec::Vec<i32>,
}
/// The aggregated historical metrics for keyword plan keywords.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeywordPlanAggregateMetricResults {
    /// The aggregate searches for all the keywords segmented by device
    /// for the specified time.
    /// Supports the following device types: MOBILE, TABLET, DESKTOP.
    ///
    /// This is only set when KeywordPlanAggregateMetricTypeEnum.DEVICE is set
    /// in the KeywordPlanAggregateMetrics field in the request.
    #[prost(message, repeated, tag="1")]
    pub device_searches: ::prost::alloc::vec::Vec<KeywordPlanDeviceSearches>,
}
/// The total searches for the device type during the specified time period.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeywordPlanDeviceSearches {
    /// The device type.
    #[prost(enumeration="super::enums::device_enum::Device", tag="1")]
    pub device: i32,
    /// The total searches for the device.
    #[prost(int64, optional, tag="2")]
    pub search_count: ::core::option::Option<i64>,
}
/// The Annotations for the Keyword plan keywords.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeywordAnnotations {
    /// The list of concepts for the keyword.
    #[prost(message, repeated, tag="1")]
    pub concepts: ::prost::alloc::vec::Vec<KeywordConcept>,
}
/// The concept for the keyword.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeywordConcept {
    /// The concept name for the keyword in the concept_group.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// The concept group of the concept details.
    #[prost(message, optional, tag="2")]
    pub concept_group: ::core::option::Option<ConceptGroup>,
}
/// The concept group for the keyword concept.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConceptGroup {
    /// The concept group name.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// The concept group type.
    #[prost(enumeration="super::enums::keyword_plan_concept_group_type_enum::KeywordPlanConceptGroupType", tag="2")]
    pub r#type: i32,
}
// Proto file describing a matching function.

/// Matching function associated with a
/// CustomerFeed, CampaignFeed, or AdGroupFeed. The matching function is used
/// to filter the set of feed items selected.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MatchingFunction {
    /// String representation of the Function.
    ///
    /// Examples:
    ///
    /// 1. IDENTITY(true) or IDENTITY(false). All or no feed items served.
    /// 2. EQUALS(CONTEXT.DEVICE,"Mobile")
    /// 3. IN(FEED_ITEM_ID,{1000001,1000002,1000003})
    /// 4. CONTAINS_ANY(FeedAttribute\[12345678,0\],{"Mars cruise","Venus cruise"})
    /// 5. AND(IN(FEED_ITEM_ID,{10001,10002}),EQUALS(CONTEXT.DEVICE,"Mobile"))
    ///
    /// For more details, visit
    /// <https://developers.google.com/adwords/api/docs/guides/feed-matching-functions>
    ///
    /// Note that because multiple strings may represent the same underlying
    /// function (whitespace and single versus double quotation marks, for
    /// example), the value returned may not be identical to the string sent in a
    /// mutate request.
    #[prost(string, optional, tag="5")]
    pub function_string: ::core::option::Option<::prost::alloc::string::String>,
    /// Operator for a function.
    #[prost(enumeration="super::enums::matching_function_operator_enum::MatchingFunctionOperator", tag="4")]
    pub operator: i32,
    /// The operands on the left hand side of the equation. This is also the
    /// operand to be used for single operand expressions such as NOT.
    #[prost(message, repeated, tag="2")]
    pub left_operands: ::prost::alloc::vec::Vec<Operand>,
    /// The operands on the right hand side of the equation.
    #[prost(message, repeated, tag="3")]
    pub right_operands: ::prost::alloc::vec::Vec<Operand>,
}
/// An operand in a matching function.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Operand {
    /// Different operands that can be used in a matching function. Required.
    #[prost(oneof="operand::FunctionArgumentOperand", tags="1, 2, 3, 4")]
    pub function_argument_operand: ::core::option::Option<operand::FunctionArgumentOperand>,
}
/// Nested message and enum types in `Operand`.
pub mod operand {
    /// A constant operand in a matching function.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ConstantOperand {
        /// Constant operand values. Required.
        #[prost(oneof="constant_operand::ConstantOperandValue", tags="5, 6, 7, 8")]
        pub constant_operand_value: ::core::option::Option<constant_operand::ConstantOperandValue>,
    }
    /// Nested message and enum types in `ConstantOperand`.
    pub mod constant_operand {
        /// Constant operand values. Required.
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum ConstantOperandValue {
            /// String value of the operand if it is a string type.
            #[prost(string, tag="5")]
            StringValue(::prost::alloc::string::String),
            /// Int64 value of the operand if it is a int64 type.
            #[prost(int64, tag="6")]
            LongValue(i64),
            /// Boolean value of the operand if it is a boolean type.
            #[prost(bool, tag="7")]
            BooleanValue(bool),
            /// Double value of the operand if it is a double type.
            #[prost(double, tag="8")]
            DoubleValue(f64),
        }
    }
    /// A feed attribute operand in a matching function.
    /// Used to represent a feed attribute in feed.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct FeedAttributeOperand {
        /// The associated feed. Required.
        #[prost(int64, optional, tag="3")]
        pub feed_id: ::core::option::Option<i64>,
        /// Id of the referenced feed attribute. Required.
        #[prost(int64, optional, tag="4")]
        pub feed_attribute_id: ::core::option::Option<i64>,
    }
    /// A function operand in a matching function.
    /// Used to represent nested functions.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct FunctionOperand {
        /// The matching function held in this operand.
        #[prost(message, optional, tag="1")]
        pub matching_function: ::core::option::Option<super::MatchingFunction>,
    }
    /// An operand in a function referring to a value in the request context.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct RequestContextOperand {
        /// Type of value to be referred in the request context.
        #[prost(enumeration="super::super::enums::matching_function_context_type_enum::MatchingFunctionContextType", tag="1")]
        pub context_type: i32,
    }
    /// Different operands that can be used in a matching function. Required.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum FunctionArgumentOperand {
        /// A constant operand in a matching function.
        #[prost(message, tag="1")]
        ConstantOperand(ConstantOperand),
        /// This operand specifies a feed attribute in feed.
        #[prost(message, tag="2")]
        FeedAttributeOperand(FeedAttributeOperand),
        /// A function operand in a matching function.
        /// Used to represent nested functions.
        #[prost(message, tag="3")]
        FunctionOperand(FunctionOperand),
        /// An operand in a function referring to a value in the request context.
        #[prost(message, tag="4")]
        RequestContextOperand(RequestContextOperand),
    }
}
// Proto file describing metrics.

/// Metrics data.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Metrics {
    /// The percent of your ad impressions that are shown as the very first ad
    /// above the organic search results.
    #[prost(double, optional, tag="183")]
    pub absolute_top_impression_percentage: ::core::option::Option<f64>,
    /// Average cost of viewable impressions (`active_view_impressions`).
    #[prost(double, optional, tag="184")]
    pub active_view_cpm: ::core::option::Option<f64>,
    /// Active view measurable clicks divided by active view viewable impressions.
    /// This metric is reported only for display network.
    #[prost(double, optional, tag="185")]
    pub active_view_ctr: ::core::option::Option<f64>,
    /// A measurement of how often your ad has become viewable on a Display
    /// Network site.
    #[prost(int64, optional, tag="186")]
    pub active_view_impressions: ::core::option::Option<i64>,
    /// The ratio of impressions that could be measured by Active View over the
    /// number of served impressions.
    #[prost(double, optional, tag="187")]
    pub active_view_measurability: ::core::option::Option<f64>,
    /// The cost of the impressions you received that were measurable by Active
    /// View.
    #[prost(int64, optional, tag="188")]
    pub active_view_measurable_cost_micros: ::core::option::Option<i64>,
    /// The number of times your ads are appearing on placements in positions
    /// where they can be seen.
    #[prost(int64, optional, tag="189")]
    pub active_view_measurable_impressions: ::core::option::Option<i64>,
    /// The percentage of time when your ad appeared on an Active View enabled site
    /// (measurable impressions) and was viewable (viewable impressions).
    #[prost(double, optional, tag="190")]
    pub active_view_viewability: ::core::option::Option<f64>,
    /// All conversions from interactions (as oppose to view through conversions)
    /// divided by the number of ad interactions.
    #[prost(double, optional, tag="191")]
    pub all_conversions_from_interactions_rate: ::core::option::Option<f64>,
    /// The value of all conversions.
    #[prost(double, optional, tag="192")]
    pub all_conversions_value: ::core::option::Option<f64>,
    /// The value of all conversions. When this column is selected with date, the
    /// values in date column means the conversion date. Details for the
    /// by_conversion_date columns are available at
    /// <https://support.google.com/google-ads/answer/9549009.>
    #[prost(double, tag="240")]
    pub all_conversions_value_by_conversion_date: f64,
    /// The total number of conversions. This includes all conversions regardless
    /// of the value of include_in_conversions_metric.
    #[prost(double, optional, tag="193")]
    pub all_conversions: ::core::option::Option<f64>,
    /// The total number of conversions. This includes all conversions regardless
    /// of the value of include_in_conversions_metric. When this column is selected
    /// with date, the values in date column means the conversion date. Details for
    /// the by_conversion_date columns are available at
    /// <https://support.google.com/google-ads/answer/9549009.>
    #[prost(double, tag="241")]
    pub all_conversions_by_conversion_date: f64,
    /// The value of all conversions divided by the total cost of ad interactions
    /// (such as clicks for text ads or views for video ads).
    #[prost(double, optional, tag="194")]
    pub all_conversions_value_per_cost: ::core::option::Option<f64>,
    /// The number of times people clicked the "Call" button to call a store during
    /// or after clicking an ad. This number doesn't include whether or not calls
    /// were connected, or the duration of any calls.
    /// This metric applies to feed items only.
    #[prost(double, optional, tag="195")]
    pub all_conversions_from_click_to_call: ::core::option::Option<f64>,
    /// The number of times people clicked a "Get directions" button to navigate to
    /// a store after clicking an ad.
    /// This metric applies to feed items only.
    #[prost(double, optional, tag="196")]
    pub all_conversions_from_directions: ::core::option::Option<f64>,
    /// The value of all conversions from interactions divided by the total number
    /// of interactions.
    #[prost(double, optional, tag="197")]
    pub all_conversions_from_interactions_value_per_interaction: ::core::option::Option<f64>,
    /// The number of times people clicked a link to view a store's menu after
    /// clicking an ad.
    /// This metric applies to feed items only.
    #[prost(double, optional, tag="198")]
    pub all_conversions_from_menu: ::core::option::Option<f64>,
    /// The number of times people placed an order at a store after clicking an ad.
    /// This metric applies to feed items only.
    #[prost(double, optional, tag="199")]
    pub all_conversions_from_order: ::core::option::Option<f64>,
    /// The number of other conversions (for example, posting a review or saving a
    /// location for a store) that occurred after people clicked an ad.
    /// This metric applies to feed items only.
    #[prost(double, optional, tag="200")]
    pub all_conversions_from_other_engagement: ::core::option::Option<f64>,
    /// Estimated number of times people visited a store after clicking an ad.
    /// This metric applies to feed items only.
    #[prost(double, optional, tag="201")]
    pub all_conversions_from_store_visit: ::core::option::Option<f64>,
    /// The number of times that people were taken to a store's URL after clicking
    /// an ad.
    /// This metric applies to feed items only.
    #[prost(double, optional, tag="202")]
    pub all_conversions_from_store_website: ::core::option::Option<f64>,
    /// The average amount you pay per interaction. This amount is the total cost
    /// of your ads divided by the total number of interactions.
    #[prost(double, optional, tag="203")]
    pub average_cost: ::core::option::Option<f64>,
    /// The total cost of all clicks divided by the total number of clicks
    /// received.
    #[prost(double, optional, tag="204")]
    pub average_cpc: ::core::option::Option<f64>,
    /// The average amount that you've been charged for an ad engagement. This
    /// amount is the total cost of all ad engagements divided by the total number
    /// of ad engagements.
    #[prost(double, optional, tag="205")]
    pub average_cpe: ::core::option::Option<f64>,
    /// Average cost-per-thousand impressions (CPM).
    #[prost(double, optional, tag="206")]
    pub average_cpm: ::core::option::Option<f64>,
    /// The average amount you pay each time someone views your ad.
    /// The average CPV is defined by the total cost of all ad views divided by
    /// the number of views.
    #[prost(double, optional, tag="207")]
    pub average_cpv: ::core::option::Option<f64>,
    /// Average number of pages viewed per session.
    #[prost(double, optional, tag="208")]
    pub average_page_views: ::core::option::Option<f64>,
    /// Total duration of all sessions (in seconds) / number of sessions. Imported
    /// from Google Analytics.
    #[prost(double, optional, tag="209")]
    pub average_time_on_site: ::core::option::Option<f64>,
    /// An indication of how other advertisers are bidding on similar products.
    #[prost(double, optional, tag="210")]
    pub benchmark_average_max_cpc: ::core::option::Option<f64>,
    /// An indication on how other advertisers' Shopping ads for similar products
    /// are performing based on how often people who see their ad click on it.
    #[prost(double, optional, tag="211")]
    pub benchmark_ctr: ::core::option::Option<f64>,
    /// Percentage of clicks where the user only visited a single page on your
    /// site. Imported from Google Analytics.
    #[prost(double, optional, tag="212")]
    pub bounce_rate: ::core::option::Option<f64>,
    /// The number of clicks.
    #[prost(int64, optional, tag="131")]
    pub clicks: ::core::option::Option<i64>,
    /// The number of times your ad or your site's listing in the unpaid
    /// results was clicked. See the help page at
    /// <https://support.google.com/google-ads/answer/3097241> for details.
    #[prost(int64, optional, tag="156")]
    pub combined_clicks: ::core::option::Option<i64>,
    /// The number of times your ad or your site's listing in the unpaid
    /// results was clicked (combined_clicks) divided by combined_queries. See the
    /// help page at <https://support.google.com/google-ads/answer/3097241> for
    /// details.
    #[prost(double, optional, tag="157")]
    pub combined_clicks_per_query: ::core::option::Option<f64>,
    /// The number of searches that returned pages from your site in the unpaid
    /// results or showed one of your text ads. See the help page at
    /// <https://support.google.com/google-ads/answer/3097241> for details.
    #[prost(int64, optional, tag="158")]
    pub combined_queries: ::core::option::Option<i64>,
    /// The estimated percent of times that your ad was eligible to show
    /// on the Display Network but didn't because your budget was too low.
    /// Note: Content budget lost impression share is reported in the range of 0
    /// to 0.9. Any value above 0.9 is reported as 0.9001.
    #[prost(double, optional, tag="159")]
    pub content_budget_lost_impression_share: ::core::option::Option<f64>,
    /// The impressions you've received on the Display Network divided
    /// by the estimated number of impressions you were eligible to receive.
    /// Note: Content impression share is reported in the range of 0.1 to 1. Any
    /// value below 0.1 is reported as 0.0999.
    #[prost(double, optional, tag="160")]
    pub content_impression_share: ::core::option::Option<f64>,
    /// The last date/time a conversion tag for this conversion action successfully
    /// fired and was seen by Google Ads. This firing event may not have been the
    /// result of an attributable conversion (e.g. because the tag was fired from a
    /// browser that did not previously click an ad from an appropriate
    /// advertiser). The date/time is in the customer's time zone.
    #[prost(string, optional, tag="161")]
    pub conversion_last_received_request_date_time: ::core::option::Option<::prost::alloc::string::String>,
    /// The date of the most recent conversion for this conversion action. The date
    /// is in the customer's time zone.
    #[prost(string, optional, tag="162")]
    pub conversion_last_conversion_date: ::core::option::Option<::prost::alloc::string::String>,
    /// The estimated percentage of impressions on the Display Network
    /// that your ads didn't receive due to poor Ad Rank.
    /// Note: Content rank lost impression share is reported in the range of 0
    /// to 0.9. Any value above 0.9 is reported as 0.9001.
    #[prost(double, optional, tag="163")]
    pub content_rank_lost_impression_share: ::core::option::Option<f64>,
    /// Conversions from interactions divided by the number of ad interactions
    /// (such as clicks for text ads or views for video ads). This only includes
    /// conversion actions which include_in_conversions_metric attribute is set to
    /// true. If you use conversion-based bidding, your bid strategies will
    /// optimize for these conversions.
    #[prost(double, optional, tag="164")]
    pub conversions_from_interactions_rate: ::core::option::Option<f64>,
    /// The value of conversions. This only includes conversion actions which
    /// include_in_conversions_metric attribute is set to true. If you use
    /// conversion-based bidding, your bid strategies will optimize for these
    /// conversions.
    #[prost(double, optional, tag="165")]
    pub conversions_value: ::core::option::Option<f64>,
    /// The value of conversions. This only includes conversion actions which
    /// include_in_conversions_metric attribute is set to true. If you use
    /// conversion-based bidding, your bid strategies will optimize for these
    /// conversions. When this column is selected with date, the values in date
    /// column means the conversion date. Details for the by_conversion_date
    /// columns are available at
    /// <https://support.google.com/google-ads/answer/9549009.>
    #[prost(double, tag="242")]
    pub conversions_value_by_conversion_date: f64,
    /// The value of conversions divided by the cost of ad interactions. This only
    /// includes conversion actions which include_in_conversions_metric attribute
    /// is set to true. If you use conversion-based bidding, your bid strategies
    /// will optimize for these conversions.
    #[prost(double, optional, tag="166")]
    pub conversions_value_per_cost: ::core::option::Option<f64>,
    /// The value of conversions from interactions divided by the number of ad
    /// interactions. This only includes conversion actions which
    /// include_in_conversions_metric attribute is set to true. If you use
    /// conversion-based bidding, your bid strategies will optimize for these
    /// conversions.
    #[prost(double, optional, tag="167")]
    pub conversions_from_interactions_value_per_interaction: ::core::option::Option<f64>,
    /// The number of conversions. This only includes conversion actions which
    /// include_in_conversions_metric attribute is set to true. If you use
    /// conversion-based bidding, your bid strategies will optimize for these
    /// conversions.
    #[prost(double, optional, tag="168")]
    pub conversions: ::core::option::Option<f64>,
    /// The number of conversions. This only includes conversion actions which
    /// include_in_conversions_metric attribute is set to true. If you use
    /// conversion-based bidding, your bid strategies will optimize for these
    /// conversions. When this column is selected with date, the values in date
    /// column means the conversion date. Details for the by_conversion_date
    /// columns are available at
    /// <https://support.google.com/google-ads/answer/9549009.>
    #[prost(double, tag="243")]
    pub conversions_by_conversion_date: f64,
    /// The sum of your cost-per-click (CPC) and cost-per-thousand impressions
    /// (CPM) costs during this period.
    #[prost(int64, optional, tag="169")]
    pub cost_micros: ::core::option::Option<i64>,
    /// The cost of ad interactions divided by all conversions.
    #[prost(double, optional, tag="170")]
    pub cost_per_all_conversions: ::core::option::Option<f64>,
    /// The cost of ad interactions divided by conversions. This only includes
    /// conversion actions which include_in_conversions_metric attribute is set to
    /// true. If you use conversion-based bidding, your bid strategies will
    /// optimize for these conversions.
    #[prost(double, optional, tag="171")]
    pub cost_per_conversion: ::core::option::Option<f64>,
    /// The cost of ad interactions divided by current model attributed
    /// conversions. This only includes conversion actions which
    /// include_in_conversions_metric attribute is set to true. If you use
    /// conversion-based bidding, your bid strategies will optimize for these
    /// conversions.
    #[prost(double, optional, tag="172")]
    pub cost_per_current_model_attributed_conversion: ::core::option::Option<f64>,
    /// Conversions from when a customer clicks on a Google Ads ad on one device,
    /// then converts on a different device or browser.
    /// Cross-device conversions are already included in all_conversions.
    #[prost(double, optional, tag="173")]
    pub cross_device_conversions: ::core::option::Option<f64>,
    /// The number of clicks your ad receives (Clicks) divided by the number
    /// of times your ad is shown (Impressions).
    #[prost(double, optional, tag="174")]
    pub ctr: ::core::option::Option<f64>,
    /// Shows how your historic conversions data would look under the attribution
    /// model you've currently selected. This only includes conversion actions
    /// which include_in_conversions_metric attribute is set to true. If you use
    /// conversion-based bidding, your bid strategies will optimize for these
    /// conversions.
    #[prost(double, optional, tag="175")]
    pub current_model_attributed_conversions: ::core::option::Option<f64>,
    /// Current model attributed conversions from interactions divided by the
    /// number of ad interactions (such as clicks for text ads or views for video
    /// ads). This only includes conversion actions which
    /// include_in_conversions_metric attribute is set to true. If you use
    /// conversion-based bidding, your bid strategies will optimize for these
    /// conversions.
    #[prost(double, optional, tag="176")]
    pub current_model_attributed_conversions_from_interactions_rate: ::core::option::Option<f64>,
    /// The value of current model attributed conversions from interactions divided
    /// by the number of ad interactions. This only includes conversion actions
    /// which include_in_conversions_metric attribute is set to true. If you use
    /// conversion-based bidding, your bid strategies will optimize for these
    /// conversions.
    #[prost(double, optional, tag="177")]
    pub current_model_attributed_conversions_from_interactions_value_per_interaction: ::core::option::Option<f64>,
    /// The value of current model attributed conversions. This only includes
    /// conversion actions which include_in_conversions_metric attribute is set to
    /// true. If you use conversion-based bidding, your bid strategies will
    /// optimize for these conversions.
    #[prost(double, optional, tag="178")]
    pub current_model_attributed_conversions_value: ::core::option::Option<f64>,
    /// The value of current model attributed conversions divided by the cost of ad
    /// interactions. This only includes conversion actions which
    /// include_in_conversions_metric attribute is set to true. If you use
    /// conversion-based bidding, your bid strategies will optimize for these
    /// conversions.
    #[prost(double, optional, tag="179")]
    pub current_model_attributed_conversions_value_per_cost: ::core::option::Option<f64>,
    /// How often people engage with your ad after it's shown to them. This is the
    /// number of ad expansions divided by the number of times your ad is shown.
    #[prost(double, optional, tag="180")]
    pub engagement_rate: ::core::option::Option<f64>,
    /// The number of engagements.
    /// An engagement occurs when a viewer expands your Lightbox ad. Also, in the
    /// future, other ad types may support engagement metrics.
    #[prost(int64, optional, tag="181")]
    pub engagements: ::core::option::Option<i64>,
    /// Average lead value based on clicks.
    #[prost(double, optional, tag="213")]
    pub hotel_average_lead_value_micros: ::core::option::Option<f64>,
    /// The average price difference between the price offered by reporting hotel
    /// advertiser and the cheapest price offered by the competing advertiser.
    #[prost(double, optional, tag="214")]
    pub hotel_price_difference_percentage: ::core::option::Option<f64>,
    /// The number of impressions that hotel partners could have had given their
    /// feed performance.
    #[prost(int64, optional, tag="215")]
    pub hotel_eligible_impressions: ::core::option::Option<i64>,
    /// The creative historical quality score.
    #[prost(enumeration="super::enums::quality_score_bucket_enum::QualityScoreBucket", tag="80")]
    pub historical_creative_quality_score: i32,
    /// The quality of historical landing page experience.
    #[prost(enumeration="super::enums::quality_score_bucket_enum::QualityScoreBucket", tag="81")]
    pub historical_landing_page_quality_score: i32,
    /// The historical quality score.
    #[prost(int64, optional, tag="216")]
    pub historical_quality_score: ::core::option::Option<i64>,
    /// The historical search predicted click through rate (CTR).
    #[prost(enumeration="super::enums::quality_score_bucket_enum::QualityScoreBucket", tag="83")]
    pub historical_search_predicted_ctr: i32,
    /// The number of times the ad was forwarded to someone else as a message.
    #[prost(int64, optional, tag="217")]
    pub gmail_forwards: ::core::option::Option<i64>,
    /// The number of times someone has saved your Gmail ad to their inbox as a
    /// message.
    #[prost(int64, optional, tag="218")]
    pub gmail_saves: ::core::option::Option<i64>,
    /// The number of clicks to the landing page on the expanded state of Gmail
    /// ads.
    #[prost(int64, optional, tag="219")]
    pub gmail_secondary_clicks: ::core::option::Option<i64>,
    /// The number of times a store's location-based ad was shown.
    /// This metric applies to feed items only.
    #[prost(int64, optional, tag="220")]
    pub impressions_from_store_reach: ::core::option::Option<i64>,
    /// Count of how often your ad has appeared on a search results page or
    /// website on the Google Network.
    #[prost(int64, optional, tag="221")]
    pub impressions: ::core::option::Option<i64>,
    /// How often people interact with your ad after it is shown to them.
    /// This is the number of interactions divided by the number of times your ad
    /// is shown.
    #[prost(double, optional, tag="222")]
    pub interaction_rate: ::core::option::Option<f64>,
    /// The number of interactions.
    /// An interaction is the main user action associated with an ad format-clicks
    /// for text and shopping ads, views for video ads, and so on.
    #[prost(int64, optional, tag="223")]
    pub interactions: ::core::option::Option<i64>,
    /// The types of payable and free interactions.
    #[prost(enumeration="super::enums::interaction_event_type_enum::InteractionEventType", repeated, tag="100")]
    pub interaction_event_types: ::prost::alloc::vec::Vec<i32>,
    /// The percentage of clicks filtered out of your total number of clicks
    /// (filtered + non-filtered clicks) during the reporting period.
    #[prost(double, optional, tag="224")]
    pub invalid_click_rate: ::core::option::Option<f64>,
    /// Number of clicks Google considers illegitimate and doesn't charge you for.
    #[prost(int64, optional, tag="225")]
    pub invalid_clicks: ::core::option::Option<i64>,
    /// Number of message chats initiated for Click To Message impressions that
    /// were message tracking eligible.
    #[prost(int64, optional, tag="226")]
    pub message_chats: ::core::option::Option<i64>,
    /// Number of Click To Message impressions that were message tracking eligible.
    #[prost(int64, optional, tag="227")]
    pub message_impressions: ::core::option::Option<i64>,
    /// Number of message chats initiated (message_chats) divided by the number
    /// of message impressions (message_impressions).
    /// Rate at which a user initiates a message chat from an ad impression with
    /// a messaging option and message tracking enabled.
    /// Note that this rate can be more than 1.0 for a given message impression.
    #[prost(double, optional, tag="228")]
    pub message_chat_rate: ::core::option::Option<f64>,
    /// The percentage of mobile clicks that go to a mobile-friendly page.
    #[prost(double, optional, tag="229")]
    pub mobile_friendly_clicks_percentage: ::core::option::Option<f64>,
    /// Total optimization score uplift of all recommendations.
    #[prost(double, optional, tag="247")]
    pub optimization_score_uplift: ::core::option::Option<f64>,
    /// URL for the optimization score page in the Google Ads web interface.
    /// This metric can be selected from `customer` or `campaign`, and can be
    /// segmented by `segments.recommendation_type`. For example, `SELECT
    /// metrics.optimization_score_url, segments.recommendation_type FROM
    /// customer` will return a URL for each unique (customer, recommendation_type)
    /// combination.
    #[prost(string, optional, tag="248")]
    pub optimization_score_url: ::core::option::Option<::prost::alloc::string::String>,
    /// The number of times someone clicked your site's listing in the unpaid
    /// results for a particular query. See the help page at
    /// <https://support.google.com/google-ads/answer/3097241> for details.
    #[prost(int64, optional, tag="230")]
    pub organic_clicks: ::core::option::Option<i64>,
    /// The number of times someone clicked your site's listing in the unpaid
    /// results (organic_clicks) divided by the total number of searches that
    /// returned pages from your site (organic_queries). See the help page at
    /// <https://support.google.com/google-ads/answer/3097241> for details.
    #[prost(double, optional, tag="231")]
    pub organic_clicks_per_query: ::core::option::Option<f64>,
    /// The number of listings for your site in the unpaid search results. See the
    /// help page at <https://support.google.com/google-ads/answer/3097241> for
    /// details.
    #[prost(int64, optional, tag="232")]
    pub organic_impressions: ::core::option::Option<i64>,
    /// The number of times a page from your site was listed in the unpaid search
    /// results (organic_impressions) divided by the number of searches returning
    /// your site's listing in the unpaid results (organic_queries). See the help
    /// page at <https://support.google.com/google-ads/answer/3097241> for details.
    #[prost(double, optional, tag="233")]
    pub organic_impressions_per_query: ::core::option::Option<f64>,
    /// The total number of searches that returned your site's listing in the
    /// unpaid results. See the help page at
    /// <https://support.google.com/google-ads/answer/3097241> for details.
    #[prost(int64, optional, tag="234")]
    pub organic_queries: ::core::option::Option<i64>,
    /// Percentage of first-time sessions (from people who had never visited your
    /// site before). Imported from Google Analytics.
    #[prost(double, optional, tag="235")]
    pub percent_new_visitors: ::core::option::Option<f64>,
    /// Number of offline phone calls.
    #[prost(int64, optional, tag="236")]
    pub phone_calls: ::core::option::Option<i64>,
    /// Number of offline phone impressions.
    #[prost(int64, optional, tag="237")]
    pub phone_impressions: ::core::option::Option<i64>,
    /// Number of phone calls received (phone_calls) divided by the number of
    /// times your phone number is shown (phone_impressions).
    #[prost(double, optional, tag="238")]
    pub phone_through_rate: ::core::option::Option<f64>,
    /// Your clickthrough rate (Ctr) divided by the average clickthrough rate of
    /// all advertisers on the websites that show your ads. Measures how your ads
    /// perform on Display Network sites compared to other ads on the same sites.
    #[prost(double, optional, tag="239")]
    pub relative_ctr: ::core::option::Option<f64>,
    /// The percentage of the customer's Shopping or Search ad impressions that are
    /// shown in the most prominent Shopping position. See
    /// <https://support.google.com/google-ads/answer/7501826>
    /// for details. Any value below 0.1 is reported as 0.0999.
    #[prost(double, optional, tag="136")]
    pub search_absolute_top_impression_share: ::core::option::Option<f64>,
    /// The number estimating how often your ad wasn't the very first ad above the
    /// organic search results due to a low budget. Note: Search
    /// budget lost absolute top impression share is reported in the range of 0 to
    /// 0.9. Any value above 0.9 is reported as 0.9001.
    #[prost(double, optional, tag="137")]
    pub search_budget_lost_absolute_top_impression_share: ::core::option::Option<f64>,
    /// The estimated percent of times that your ad was eligible to show on the
    /// Search Network but didn't because your budget was too low. Note: Search
    /// budget lost impression share is reported in the range of 0 to 0.9. Any
    /// value above 0.9 is reported as 0.9001.
    #[prost(double, optional, tag="138")]
    pub search_budget_lost_impression_share: ::core::option::Option<f64>,
    /// The number estimating how often your ad didn't show anywhere above the
    /// organic search results due to a low budget. Note: Search
    /// budget lost top impression share is reported in the range of 0 to 0.9. Any
    /// value above 0.9 is reported as 0.9001.
    #[prost(double, optional, tag="139")]
    pub search_budget_lost_top_impression_share: ::core::option::Option<f64>,
    /// The number of clicks you've received on the Search Network
    /// divided by the estimated number of clicks you were eligible to receive.
    /// Note: Search click share is reported in the range of 0.1 to 1. Any value
    /// below 0.1 is reported as 0.0999.
    #[prost(double, optional, tag="140")]
    pub search_click_share: ::core::option::Option<f64>,
    /// The impressions you've received divided by the estimated number of
    /// impressions you were eligible to receive on the Search Network for search
    /// terms that matched your keywords exactly (or were close variants of your
    /// keyword), regardless of your keyword match types. Note: Search exact match
    /// impression share is reported in the range of 0.1 to 1. Any value below 0.1
    /// is reported as 0.0999.
    #[prost(double, optional, tag="141")]
    pub search_exact_match_impression_share: ::core::option::Option<f64>,
    /// The impressions you've received on the Search Network divided
    /// by the estimated number of impressions you were eligible to receive.
    /// Note: Search impression share is reported in the range of 0.1 to 1. Any
    /// value below 0.1 is reported as 0.0999.
    #[prost(double, optional, tag="142")]
    pub search_impression_share: ::core::option::Option<f64>,
    /// The number estimating how often your ad wasn't the very first ad above the
    /// organic search results due to poor Ad Rank.
    /// Note: Search rank lost absolute top impression share is reported in the
    /// range of 0 to 0.9. Any value above 0.9 is reported as 0.9001.
    #[prost(double, optional, tag="143")]
    pub search_rank_lost_absolute_top_impression_share: ::core::option::Option<f64>,
    /// The estimated percentage of impressions on the Search Network
    /// that your ads didn't receive due to poor Ad Rank.
    /// Note: Search rank lost impression share is reported in the range of 0 to
    /// 0.9. Any value above 0.9 is reported as 0.9001.
    #[prost(double, optional, tag="144")]
    pub search_rank_lost_impression_share: ::core::option::Option<f64>,
    /// The number estimating how often your ad didn't show anywhere above the
    /// organic search results due to poor Ad Rank.
    /// Note: Search rank lost top impression share is reported in the range of 0
    /// to 0.9. Any value above 0.9 is reported as 0.9001.
    #[prost(double, optional, tag="145")]
    pub search_rank_lost_top_impression_share: ::core::option::Option<f64>,
    /// The impressions you've received in the top location (anywhere above the
    /// organic search results) compared to the estimated number of impressions you
    /// were eligible to receive in the top location.
    /// Note: Search top impression share is reported in the range of 0.1 to 1. Any
    /// value below 0.1 is reported as 0.0999.
    #[prost(double, optional, tag="146")]
    pub search_top_impression_share: ::core::option::Option<f64>,
    /// A measure of how quickly your page loads after clicks on your mobile ads.
    /// The score is a range from 1 to 10, 10 being the fastest.
    #[prost(int64, optional, tag="147")]
    pub speed_score: ::core::option::Option<i64>,
    /// The percent of your ad impressions that are shown anywhere above the
    /// organic search results.
    #[prost(double, optional, tag="148")]
    pub top_impression_percentage: ::core::option::Option<f64>,
    /// The percentage of ad clicks to Accelerated Mobile Pages (AMP) landing pages
    /// that reach a valid AMP page.
    #[prost(double, optional, tag="149")]
    pub valid_accelerated_mobile_pages_clicks_percentage: ::core::option::Option<f64>,
    /// The value of all conversions divided by the number of all conversions.
    #[prost(double, optional, tag="150")]
    pub value_per_all_conversions: ::core::option::Option<f64>,
    /// The value of all conversions divided by the number of all conversions. When
    /// this column is selected with date, the values in date column means the
    /// conversion date. Details for the by_conversion_date columns are available
    /// at <https://support.google.com/google-ads/answer/9549009.>
    #[prost(double, optional, tag="244")]
    pub value_per_all_conversions_by_conversion_date: ::core::option::Option<f64>,
    /// The value of conversions divided by the number of conversions. This only
    /// includes conversion actions which include_in_conversions_metric attribute
    /// is set to true. If you use conversion-based bidding, your bid strategies
    /// will optimize for these conversions.
    #[prost(double, optional, tag="151")]
    pub value_per_conversion: ::core::option::Option<f64>,
    /// The value of conversions divided by the number of conversions. This only
    /// includes conversion actions which include_in_conversions_metric attribute
    /// is set to true. If you use conversion-based bidding, your bid strategies
    /// will optimize for these conversions. When this column is selected with
    /// date, the values in date column means the conversion date. Details for the
    /// by_conversion_date columns are available at
    /// <https://support.google.com/google-ads/answer/9549009.>
    #[prost(double, optional, tag="245")]
    pub value_per_conversions_by_conversion_date: ::core::option::Option<f64>,
    /// The value of current model attributed conversions divided by the number of
    /// the conversions. This only includes conversion actions which
    /// include_in_conversions_metric attribute is set to true. If you use
    /// conversion-based bidding, your bid strategies will optimize for these
    /// conversions.
    #[prost(double, optional, tag="152")]
    pub value_per_current_model_attributed_conversion: ::core::option::Option<f64>,
    /// Percentage of impressions where the viewer watched all of your video.
    #[prost(double, optional, tag="132")]
    pub video_quartile_p100_rate: ::core::option::Option<f64>,
    /// Percentage of impressions where the viewer watched 25% of your video.
    #[prost(double, optional, tag="133")]
    pub video_quartile_p25_rate: ::core::option::Option<f64>,
    /// Percentage of impressions where the viewer watched 50% of your video.
    #[prost(double, optional, tag="134")]
    pub video_quartile_p50_rate: ::core::option::Option<f64>,
    /// Percentage of impressions where the viewer watched 75% of your video.
    #[prost(double, optional, tag="135")]
    pub video_quartile_p75_rate: ::core::option::Option<f64>,
    /// The number of views your TrueView video ad receives divided by its number
    /// of impressions, including thumbnail impressions for TrueView in-display
    /// ads.
    #[prost(double, optional, tag="153")]
    pub video_view_rate: ::core::option::Option<f64>,
    /// The number of times your video ads were viewed.
    #[prost(int64, optional, tag="154")]
    pub video_views: ::core::option::Option<i64>,
    /// The total number of view-through conversions.
    /// These happen when a customer sees an image or rich media ad, then later
    /// completes a conversion on your site without interacting with (e.g.,
    /// clicking on) another ad.
    #[prost(int64, optional, tag="155")]
    pub view_through_conversions: ::core::option::Option<i64>,
    /// The number of iOS Store Kit Ad Network conversions.
    #[prost(int64, tag="246")]
    pub sk_ad_network_conversions: i64,
}
// Proto file describing offline user data.

/// Address identifier of offline data.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OfflineUserAddressInfo {
    /// First name of the user, which is hashed as SHA-256 after normalized
    /// (Lowercase all characters; Remove any extra spaces before, after, and in
    /// between).
    #[prost(string, optional, tag="7")]
    pub hashed_first_name: ::core::option::Option<::prost::alloc::string::String>,
    /// Last name of the user, which is hashed as SHA-256 after normalized (lower
    /// case only and no punctuation).
    #[prost(string, optional, tag="8")]
    pub hashed_last_name: ::core::option::Option<::prost::alloc::string::String>,
    /// City of the address. Only accepted for Store Sales and
    /// ConversionAdjustmentUploadService.
    #[prost(string, optional, tag="9")]
    pub city: ::core::option::Option<::prost::alloc::string::String>,
    /// State code of the address. Only accepted for Store Sales and
    /// ConversionAdjustmentUploadService.
    #[prost(string, optional, tag="10")]
    pub state: ::core::option::Option<::prost::alloc::string::String>,
    /// 2-letter country code in ISO-3166-1 alpha-2 of the user's address.
    #[prost(string, optional, tag="11")]
    pub country_code: ::core::option::Option<::prost::alloc::string::String>,
    /// Postal code of the user's address.
    #[prost(string, optional, tag="12")]
    pub postal_code: ::core::option::Option<::prost::alloc::string::String>,
    /// The street address of the user hashed using SHA-256 hash function after
    /// normalization (lower case only). Only accepted for
    /// ConversionAdjustmentUploadService.
    #[prost(string, optional, tag="13")]
    pub hashed_street_address: ::core::option::Option<::prost::alloc::string::String>,
}
/// User identifying information.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserIdentifier {
    /// Source of the user identifier when the upload is from Store Sales,
    /// ConversionUploadService, or ConversionAdjustmentUploadService.
    #[prost(enumeration="super::enums::user_identifier_source_enum::UserIdentifierSource", tag="6")]
    pub user_identifier_source: i32,
    /// Exactly one must be specified. For OfflineUserDataJobService, Customer
    /// Match accepts hashed_email, hashed_phone_number, mobile_id,
    /// third_party_user_id, and address_info; Store Sales accepts hashed_email,
    /// hashed_phone_number, third_party_user_id, and address_info.
    /// ConversionUploadService accepts hashed_email and hashed_phone_number.
    /// ConversionAdjustmentUploadService accepts hashed_email,
    /// hashed_phone_number, and address_info.
    #[prost(oneof="user_identifier::Identifier", tags="7, 8, 9, 10, 5")]
    pub identifier: ::core::option::Option<user_identifier::Identifier>,
}
/// Nested message and enum types in `UserIdentifier`.
pub mod user_identifier {
    /// Exactly one must be specified. For OfflineUserDataJobService, Customer
    /// Match accepts hashed_email, hashed_phone_number, mobile_id,
    /// third_party_user_id, and address_info; Store Sales accepts hashed_email,
    /// hashed_phone_number, third_party_user_id, and address_info.
    /// ConversionUploadService accepts hashed_email and hashed_phone_number.
    /// ConversionAdjustmentUploadService accepts hashed_email,
    /// hashed_phone_number, and address_info.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Identifier {
        /// Hashed email address using SHA-256 hash function after normalization.
        /// Accepted for Customer Match, Store Sales, ConversionUploadService, and
        /// ConversionAdjustmentUploadService.
        #[prost(string, tag="7")]
        HashedEmail(::prost::alloc::string::String),
        /// Hashed phone number using SHA-256 hash function after normalization
        /// (E164 standard). Accepted for Customer Match, Store Sales,
        /// ConversionUploadService, and ConversionAdjustmentUploadService.
        #[prost(string, tag="8")]
        HashedPhoneNumber(::prost::alloc::string::String),
        /// Mobile device ID (advertising ID/IDFA). Accepted only for Customer Match.
        #[prost(string, tag="9")]
        MobileId(::prost::alloc::string::String),
        /// Advertiser-assigned user ID for Customer Match upload, or
        /// third-party-assigned user ID for Store Sales. Accepted only for Customer
        /// Match and Store Sales.
        #[prost(string, tag="10")]
        ThirdPartyUserId(::prost::alloc::string::String),
        /// Address information. Accepted only for Customer Match, Store Sales, and
        /// ConversionAdjustmentUploadService.
        #[prost(message, tag="5")]
        AddressInfo(super::OfflineUserAddressInfo),
    }
}
/// Attribute of the store sales transaction.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionAttribute {
    /// Timestamp when transaction occurred. Required.
    /// The format is "YYYY-MM-DD HH:MM:SS\[+/-HH:MM\]", where \[+/-HH:MM\] is an
    /// optional timezone offset from UTC. If the offset is absent, the API will
    /// use the account's timezone as default.
    /// Examples: "2018-03-05 09:15:00" or "2018-02-01 14:34:30+03:00"
    #[prost(string, optional, tag="8")]
    pub transaction_date_time: ::core::option::Option<::prost::alloc::string::String>,
    /// Transaction amount in micros. Required.
    /// Transaction amount in micros needs to be greater than 1000.
    /// If item Attributes are provided, it represents the total value of the
    /// items, after multiplying the unit price per item by the quantity provided
    /// in the ItemAttributes.
    #[prost(double, optional, tag="9")]
    pub transaction_amount_micros: ::core::option::Option<f64>,
    /// Transaction currency code. ISO 4217 three-letter code is used. Required.
    #[prost(string, optional, tag="10")]
    pub currency_code: ::core::option::Option<::prost::alloc::string::String>,
    /// The resource name of conversion action to report conversions to.
    /// Required.
    #[prost(string, optional, tag="11")]
    pub conversion_action: ::core::option::Option<::prost::alloc::string::String>,
    /// Transaction order id.
    /// Accessible only to customers on the allow-list.
    #[prost(string, optional, tag="12")]
    pub order_id: ::core::option::Option<::prost::alloc::string::String>,
    /// Store attributes of the transaction.
    /// Accessible only to customers on the allow-list.
    #[prost(message, optional, tag="6")]
    pub store_attribute: ::core::option::Option<StoreAttribute>,
    /// Value of the custom variable for each transaction.
    /// Accessible only to customers on the allow-list.
    #[prost(string, optional, tag="13")]
    pub custom_value: ::core::option::Option<::prost::alloc::string::String>,
    /// Item attributes of the transaction.
    #[prost(message, optional, tag="14")]
    pub item_attribute: ::core::option::Option<ItemAttribute>,
}
/// Store attributes of the transaction.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StoreAttribute {
    /// Store code from
    /// <https://support.google.com/business/answer/3370250#storecode>
    #[prost(string, optional, tag="2")]
    pub store_code: ::core::option::Option<::prost::alloc::string::String>,
}
/// Item attributes of the transaction.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ItemAttribute {
    /// A unique identifier of a product. It can be either the Merchant Center Item
    /// ID or GTIN (Global Trade Item Number).
    #[prost(string, tag="1")]
    pub item_id: ::prost::alloc::string::String,
    /// ID of the Merchant Center Account.
    #[prost(int64, optional, tag="2")]
    pub merchant_id: ::core::option::Option<i64>,
    /// Common Locale Data Repository (CLDR) territory code of the country
    /// associated with the feed where your items are uploaded. See
    /// <https://developers.google.com/google-ads/api/reference/data/codes-formats#country-codes>
    /// for more information.
    #[prost(string, tag="3")]
    pub country_code: ::prost::alloc::string::String,
    /// ISO 639-1 code of the language associated with the feed where your items
    /// are uploaded
    #[prost(string, tag="4")]
    pub language_code: ::prost::alloc::string::String,
    /// The number of items sold. Defaults to 1 if not set.
    #[prost(int64, tag="5")]
    pub quantity: i64,
}
/// User data holding user identifiers and attributes.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserData {
    /// User identification info. Required.
    #[prost(message, repeated, tag="1")]
    pub user_identifiers: ::prost::alloc::vec::Vec<UserIdentifier>,
    /// Additional transactions/attributes associated with the user.
    /// Required when updating store sales data.
    #[prost(message, optional, tag="2")]
    pub transaction_attribute: ::core::option::Option<TransactionAttribute>,
    /// Additional attributes associated with the user. Required when updating
    /// customer match attributes. These have an expiration of 540 days.
    #[prost(message, optional, tag="3")]
    pub user_attribute: ::core::option::Option<UserAttribute>,
}
/// User attribute, can only be used with CUSTOMER_MATCH_WITH_ATTRIBUTES job
/// type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserAttribute {
    /// Advertiser defined lifetime value for the user.
    #[prost(int64, optional, tag="1")]
    pub lifetime_value_micros: ::core::option::Option<i64>,
    /// Advertiser defined lifetime value bucket for the user. The valid range for
    /// a lifetime value bucket is from 1 (low) to 10 (high), except for remove
    /// operation where 0 will also be accepted.
    #[prost(int32, optional, tag="2")]
    pub lifetime_value_bucket: ::core::option::Option<i32>,
    /// Timestamp of the last purchase made by the user.
    /// The format is YYYY-MM-DD HH:MM:SS\[+/-HH:MM\], where \[+/-HH:MM\] is an
    /// optional timezone offset from UTC. If the offset is absent, the API will
    /// use the account's timezone as default.
    #[prost(string, tag="3")]
    pub last_purchase_date_time: ::prost::alloc::string::String,
    /// Advertiser defined average number of purchases that are made by the user in
    /// a 30 day period.
    #[prost(int32, tag="4")]
    pub average_purchase_count: i32,
    /// Advertiser defined average purchase value in micros for the user.
    #[prost(int64, tag="5")]
    pub average_purchase_value_micros: i64,
    /// Timestamp when the user was acquired.
    /// The format is YYYY-MM-DD HH:MM:SS\[+/-HH:MM\], where \[+/-HH:MM\] is an
    /// optional timezone offset from UTC. If the offset is absent, the API will
    /// use the account's timezone as default.
    #[prost(string, tag="6")]
    pub acquisition_date_time: ::prost::alloc::string::String,
    /// The shopping loyalty related data. Shopping utilizes this data to provide
    /// users with a better experience. Accessible only to merchants on the
    /// allow-list with users’ consent.
    #[prost(message, optional, tag="7")]
    pub shopping_loyalty: ::core::option::Option<ShoppingLoyalty>,
}
/// The shopping loyalty related data. Shopping utilizes this data to provide
/// users with a better experience.
/// Accessible only to merchants on the allow-list.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShoppingLoyalty {
    /// The membership tier. It is a free-form string as each merchant may have
    /// their own loyalty system. For example, it could be a number from 1 to 10,
    /// or a string such as "Golden" or "Silver", or even empty string "".
    #[prost(string, optional, tag="1")]
    pub loyalty_tier: ::core::option::Option<::prost::alloc::string::String>,
}
/// Metadata for customer match user list.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomerMatchUserListMetadata {
    /// The resource name of remarketing list to update data.
    /// Required for job of CUSTOMER_MATCH_USER_LIST type.
    #[prost(string, optional, tag="2")]
    pub user_list: ::core::option::Option<::prost::alloc::string::String>,
}
/// Metadata for Store Sales Direct.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StoreSalesMetadata {
    /// This is the fraction of all transactions that are identifiable (i.e.,
    /// associated with any form of customer information).
    /// Required.
    /// The fraction needs to be between 0 and 1 (excluding 0).
    #[prost(double, optional, tag="5")]
    pub loyalty_fraction: ::core::option::Option<f64>,
    /// This is the ratio of sales being uploaded compared to the overall sales
    /// that can be associated with a customer. Required.
    /// The fraction needs to be between 0 and 1 (excluding 0). For example, if you
    /// upload half the sales that you are able to associate with a customer, this
    /// would be 0.5.
    #[prost(double, optional, tag="6")]
    pub transaction_upload_fraction: ::core::option::Option<f64>,
    /// Name of the store sales custom variable key. A predefined key that
    /// can be applied to the transaction and then later used for custom
    /// segmentation in reporting.
    /// Accessible only to customers on the allow-list.
    #[prost(string, optional, tag="7")]
    pub custom_key: ::core::option::Option<::prost::alloc::string::String>,
    /// Metadata for a third party Store Sales upload.
    #[prost(message, optional, tag="3")]
    pub third_party_metadata: ::core::option::Option<StoreSalesThirdPartyMetadata>,
}
/// Metadata for a third party Store Sales.
/// This product is only for customers on the allow-list. Please contact your
/// Google business development representative for details on the upload
/// configuration.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StoreSalesThirdPartyMetadata {
    /// Time the advertiser uploaded the data to the partner. Required.
    /// The format is "YYYY-MM-DD HH:MM:SS".
    /// Examples: "2018-03-05 09:15:00" or "2018-02-01 14:34:30"
    #[prost(string, optional, tag="7")]
    pub advertiser_upload_date_time: ::core::option::Option<::prost::alloc::string::String>,
    /// The fraction of transactions that are valid. Invalid transactions may
    /// include invalid formats or values.
    /// Required.
    /// The fraction needs to be between 0 and 1 (excluding 0).
    #[prost(double, optional, tag="8")]
    pub valid_transaction_fraction: ::core::option::Option<f64>,
    /// The fraction of valid transactions that are matched to a third party
    /// assigned user ID on the partner side.
    /// Required.
    /// The fraction needs to be between 0 and 1 (excluding 0).
    #[prost(double, optional, tag="9")]
    pub partner_match_fraction: ::core::option::Option<f64>,
    /// The fraction of valid transactions that are uploaded by the partner to
    /// Google.
    /// Required.
    /// The fraction needs to be between 0 and 1 (excluding 0).
    #[prost(double, optional, tag="10")]
    pub partner_upload_fraction: ::core::option::Option<f64>,
    /// Version of partner IDs to be used for uploads. Required.
    #[prost(string, optional, tag="11")]
    pub bridge_map_version_id: ::core::option::Option<::prost::alloc::string::String>,
    /// ID of the third party partner updating the transaction feed.
    #[prost(int64, optional, tag="12")]
    pub partner_id: ::core::option::Option<i64>,
}
// Proto file describing policy summary.

/// Contains policy summary information.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PolicySummary {
    /// The list of policy findings.
    #[prost(message, repeated, tag="1")]
    pub policy_topic_entries: ::prost::alloc::vec::Vec<PolicyTopicEntry>,
    /// Where in the review process the resource is.
    #[prost(enumeration="super::enums::policy_review_status_enum::PolicyReviewStatus", tag="2")]
    pub review_status: i32,
    /// The overall approval status, which is calculated based on
    /// the status of its individual policy topic entries.
    #[prost(enumeration="super::enums::policy_approval_status_enum::PolicyApprovalStatus", tag="3")]
    pub approval_status: i32,
}
// Proto file describing RealTimeBiddingSetting

/// Settings for Real-Time Bidding, a feature only available for campaigns
/// targeting the Ad Exchange network.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RealTimeBiddingSetting {
    /// Whether the campaign is opted in to real-time bidding.
    #[prost(bool, optional, tag="2")]
    pub opt_in: ::core::option::Option<bool>,
}
// Proto file describing segment only fields.

/// Segment only fields.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Segments {
    /// Ad Destination type.
    #[prost(enumeration="super::enums::ad_destination_type_enum::AdDestinationType", tag="136")]
    pub ad_destination_type: i32,
    /// Ad network type.
    #[prost(enumeration="super::enums::ad_network_type_enum::AdNetworkType", tag="3")]
    pub ad_network_type: i32,
    /// Budget campaign association status.
    #[prost(message, optional, tag="134")]
    pub budget_campaign_association_status: ::core::option::Option<BudgetCampaignAssociationStatus>,
    /// Click type.
    #[prost(enumeration="super::enums::click_type_enum::ClickType", tag="26")]
    pub click_type: i32,
    /// Resource name of the conversion action.
    #[prost(string, optional, tag="113")]
    pub conversion_action: ::core::option::Option<::prost::alloc::string::String>,
    /// Conversion action category.
    #[prost(enumeration="super::enums::conversion_action_category_enum::ConversionActionCategory", tag="53")]
    pub conversion_action_category: i32,
    /// Conversion action name.
    #[prost(string, optional, tag="114")]
    pub conversion_action_name: ::core::option::Option<::prost::alloc::string::String>,
    /// This segments your conversion columns by the original conversion and
    /// conversion value vs. the delta if conversions were adjusted. False row has
    /// the data as originally stated; While true row has the delta between data
    /// now and the data as originally stated. Summing the two together results
    /// post-adjustment data.
    #[prost(bool, optional, tag="115")]
    pub conversion_adjustment: ::core::option::Option<bool>,
    /// Conversion attribution event type.
    #[prost(enumeration="super::enums::conversion_attribution_event_type_enum::ConversionAttributionEventType", tag="2")]
    pub conversion_attribution_event_type: i32,
    /// An enum value representing the number of days between the impression and
    /// the conversion.
    #[prost(enumeration="super::enums::conversion_lag_bucket_enum::ConversionLagBucket", tag="50")]
    pub conversion_lag_bucket: i32,
    /// An enum value representing the number of days between the impression and
    /// the conversion or between the impression and adjustments to the conversion.
    #[prost(enumeration="super::enums::conversion_or_adjustment_lag_bucket_enum::ConversionOrAdjustmentLagBucket", tag="51")]
    pub conversion_or_adjustment_lag_bucket: i32,
    /// Date to which metrics apply.
    /// yyyy-MM-dd format, e.g., 2018-04-17.
    #[prost(string, optional, tag="79")]
    pub date: ::core::option::Option<::prost::alloc::string::String>,
    /// Day of the week, e.g., MONDAY.
    #[prost(enumeration="super::enums::day_of_week_enum::DayOfWeek", tag="5")]
    pub day_of_week: i32,
    /// Device to which metrics apply.
    #[prost(enumeration="super::enums::device_enum::Device", tag="1")]
    pub device: i32,
    /// External conversion source.
    #[prost(enumeration="super::enums::external_conversion_source_enum::ExternalConversionSource", tag="55")]
    pub external_conversion_source: i32,
    /// Resource name of the geo target constant that represents an airport.
    #[prost(string, optional, tag="116")]
    pub geo_target_airport: ::core::option::Option<::prost::alloc::string::String>,
    /// Resource name of the geo target constant that represents a canton.
    #[prost(string, optional, tag="117")]
    pub geo_target_canton: ::core::option::Option<::prost::alloc::string::String>,
    /// Resource name of the geo target constant that represents a city.
    #[prost(string, optional, tag="118")]
    pub geo_target_city: ::core::option::Option<::prost::alloc::string::String>,
    /// Resource name of the geo target constant that represents a country.
    #[prost(string, optional, tag="119")]
    pub geo_target_country: ::core::option::Option<::prost::alloc::string::String>,
    /// Resource name of the geo target constant that represents a county.
    #[prost(string, optional, tag="120")]
    pub geo_target_county: ::core::option::Option<::prost::alloc::string::String>,
    /// Resource name of the geo target constant that represents a district.
    #[prost(string, optional, tag="121")]
    pub geo_target_district: ::core::option::Option<::prost::alloc::string::String>,
    /// Resource name of the geo target constant that represents a metro.
    #[prost(string, optional, tag="122")]
    pub geo_target_metro: ::core::option::Option<::prost::alloc::string::String>,
    /// Resource name of the geo target constant that represents the most
    /// specific location.
    #[prost(string, optional, tag="123")]
    pub geo_target_most_specific_location: ::core::option::Option<::prost::alloc::string::String>,
    /// Resource name of the geo target constant that represents a postal code.
    #[prost(string, optional, tag="124")]
    pub geo_target_postal_code: ::core::option::Option<::prost::alloc::string::String>,
    /// Resource name of the geo target constant that represents a province.
    #[prost(string, optional, tag="125")]
    pub geo_target_province: ::core::option::Option<::prost::alloc::string::String>,
    /// Resource name of the geo target constant that represents a region.
    #[prost(string, optional, tag="126")]
    pub geo_target_region: ::core::option::Option<::prost::alloc::string::String>,
    /// Resource name of the geo target constant that represents a state.
    #[prost(string, optional, tag="127")]
    pub geo_target_state: ::core::option::Option<::prost::alloc::string::String>,
    /// Hotel booking window in days.
    #[prost(int64, optional, tag="135")]
    pub hotel_booking_window_days: ::core::option::Option<i64>,
    /// Hotel center ID.
    #[prost(int64, optional, tag="80")]
    pub hotel_center_id: ::core::option::Option<i64>,
    /// Hotel check-in date. Formatted as yyyy-MM-dd.
    #[prost(string, optional, tag="81")]
    pub hotel_check_in_date: ::core::option::Option<::prost::alloc::string::String>,
    /// Hotel check-in day of week.
    #[prost(enumeration="super::enums::day_of_week_enum::DayOfWeek", tag="9")]
    pub hotel_check_in_day_of_week: i32,
    /// Hotel city.
    #[prost(string, optional, tag="82")]
    pub hotel_city: ::core::option::Option<::prost::alloc::string::String>,
    /// Hotel class.
    #[prost(int32, optional, tag="83")]
    pub hotel_class: ::core::option::Option<i32>,
    /// Hotel country.
    #[prost(string, optional, tag="84")]
    pub hotel_country: ::core::option::Option<::prost::alloc::string::String>,
    /// Hotel date selection type.
    #[prost(enumeration="super::enums::hotel_date_selection_type_enum::HotelDateSelectionType", tag="13")]
    pub hotel_date_selection_type: i32,
    /// Hotel length of stay.
    #[prost(int32, optional, tag="85")]
    pub hotel_length_of_stay: ::core::option::Option<i32>,
    /// Hotel rate rule ID.
    #[prost(string, optional, tag="86")]
    pub hotel_rate_rule_id: ::core::option::Option<::prost::alloc::string::String>,
    /// Hotel rate type.
    #[prost(enumeration="super::enums::hotel_rate_type_enum::HotelRateType", tag="74")]
    pub hotel_rate_type: i32,
    /// Hotel price bucket.
    #[prost(enumeration="super::enums::hotel_price_bucket_enum::HotelPriceBucket", tag="78")]
    pub hotel_price_bucket: i32,
    /// Hotel state.
    #[prost(string, optional, tag="87")]
    pub hotel_state: ::core::option::Option<::prost::alloc::string::String>,
    /// Hour of day as a number between 0 and 23, inclusive.
    #[prost(int32, optional, tag="88")]
    pub hour: ::core::option::Option<i32>,
    /// Only used with feed item metrics.
    /// Indicates whether the interaction metrics occurred on the feed item itself
    /// or a different extension or ad unit.
    #[prost(bool, optional, tag="89")]
    pub interaction_on_this_extension: ::core::option::Option<bool>,
    /// Keyword criterion.
    #[prost(message, optional, tag="61")]
    pub keyword: ::core::option::Option<Keyword>,
    /// Month as represented by the date of the first day of a month. Formatted as
    /// yyyy-MM-dd.
    #[prost(string, optional, tag="90")]
    pub month: ::core::option::Option<::prost::alloc::string::String>,
    /// Month of the year, e.g., January.
    #[prost(enumeration="super::enums::month_of_year_enum::MonthOfYear", tag="18")]
    pub month_of_year: i32,
    /// Partner hotel ID.
    #[prost(string, optional, tag="91")]
    pub partner_hotel_id: ::core::option::Option<::prost::alloc::string::String>,
    /// Placeholder type. This is only used with feed item metrics.
    #[prost(enumeration="super::enums::placeholder_type_enum::PlaceholderType", tag="20")]
    pub placeholder_type: i32,
    /// Aggregator ID of the product.
    #[prost(int64, optional, tag="132")]
    pub product_aggregator_id: ::core::option::Option<i64>,
    /// Bidding category (level 1) of the product.
    #[prost(string, optional, tag="92")]
    pub product_bidding_category_level1: ::core::option::Option<::prost::alloc::string::String>,
    /// Bidding category (level 2) of the product.
    #[prost(string, optional, tag="93")]
    pub product_bidding_category_level2: ::core::option::Option<::prost::alloc::string::String>,
    /// Bidding category (level 3) of the product.
    #[prost(string, optional, tag="94")]
    pub product_bidding_category_level3: ::core::option::Option<::prost::alloc::string::String>,
    /// Bidding category (level 4) of the product.
    #[prost(string, optional, tag="95")]
    pub product_bidding_category_level4: ::core::option::Option<::prost::alloc::string::String>,
    /// Bidding category (level 5) of the product.
    #[prost(string, optional, tag="96")]
    pub product_bidding_category_level5: ::core::option::Option<::prost::alloc::string::String>,
    /// Brand of the product.
    #[prost(string, optional, tag="97")]
    pub product_brand: ::core::option::Option<::prost::alloc::string::String>,
    /// Channel of the product.
    #[prost(enumeration="super::enums::product_channel_enum::ProductChannel", tag="30")]
    pub product_channel: i32,
    /// Channel exclusivity of the product.
    #[prost(enumeration="super::enums::product_channel_exclusivity_enum::ProductChannelExclusivity", tag="31")]
    pub product_channel_exclusivity: i32,
    /// Condition of the product.
    #[prost(enumeration="super::enums::product_condition_enum::ProductCondition", tag="32")]
    pub product_condition: i32,
    /// Resource name of the geo target constant for the country of sale of the
    /// product.
    #[prost(string, optional, tag="98")]
    pub product_country: ::core::option::Option<::prost::alloc::string::String>,
    /// Custom attribute 0 of the product.
    #[prost(string, optional, tag="99")]
    pub product_custom_attribute0: ::core::option::Option<::prost::alloc::string::String>,
    /// Custom attribute 1 of the product.
    #[prost(string, optional, tag="100")]
    pub product_custom_attribute1: ::core::option::Option<::prost::alloc::string::String>,
    /// Custom attribute 2 of the product.
    #[prost(string, optional, tag="101")]
    pub product_custom_attribute2: ::core::option::Option<::prost::alloc::string::String>,
    /// Custom attribute 3 of the product.
    #[prost(string, optional, tag="102")]
    pub product_custom_attribute3: ::core::option::Option<::prost::alloc::string::String>,
    /// Custom attribute 4 of the product.
    #[prost(string, optional, tag="103")]
    pub product_custom_attribute4: ::core::option::Option<::prost::alloc::string::String>,
    /// Item ID of the product.
    #[prost(string, optional, tag="104")]
    pub product_item_id: ::core::option::Option<::prost::alloc::string::String>,
    /// Resource name of the language constant for the language of the product.
    #[prost(string, optional, tag="105")]
    pub product_language: ::core::option::Option<::prost::alloc::string::String>,
    /// Merchant ID of the product.
    #[prost(int64, optional, tag="133")]
    pub product_merchant_id: ::core::option::Option<i64>,
    /// Store ID of the product.
    #[prost(string, optional, tag="106")]
    pub product_store_id: ::core::option::Option<::prost::alloc::string::String>,
    /// Title of the product.
    #[prost(string, optional, tag="107")]
    pub product_title: ::core::option::Option<::prost::alloc::string::String>,
    /// Type (level 1) of the product.
    #[prost(string, optional, tag="108")]
    pub product_type_l1: ::core::option::Option<::prost::alloc::string::String>,
    /// Type (level 2) of the product.
    #[prost(string, optional, tag="109")]
    pub product_type_l2: ::core::option::Option<::prost::alloc::string::String>,
    /// Type (level 3) of the product.
    #[prost(string, optional, tag="110")]
    pub product_type_l3: ::core::option::Option<::prost::alloc::string::String>,
    /// Type (level 4) of the product.
    #[prost(string, optional, tag="111")]
    pub product_type_l4: ::core::option::Option<::prost::alloc::string::String>,
    /// Type (level 5) of the product.
    #[prost(string, optional, tag="112")]
    pub product_type_l5: ::core::option::Option<::prost::alloc::string::String>,
    /// Quarter as represented by the date of the first day of a quarter.
    /// Uses the calendar year for quarters, e.g., the second quarter of 2018
    /// starts on 2018-04-01. Formatted as yyyy-MM-dd.
    #[prost(string, optional, tag="128")]
    pub quarter: ::core::option::Option<::prost::alloc::string::String>,
    /// Recommendation type.
    #[prost(enumeration="super::enums::recommendation_type_enum::RecommendationType", tag="140")]
    pub recommendation_type: i32,
    /// Type of the search engine results page.
    #[prost(enumeration="super::enums::search_engine_results_page_type_enum::SearchEngineResultsPageType", tag="70")]
    pub search_engine_results_page_type: i32,
    /// Match type of the keyword that triggered the ad, including variants.
    #[prost(enumeration="super::enums::search_term_match_type_enum::SearchTermMatchType", tag="22")]
    pub search_term_match_type: i32,
    /// Position of the ad.
    #[prost(enumeration="super::enums::slot_enum::Slot", tag="23")]
    pub slot: i32,
    /// Primary dimension of applied conversion value rules.
    /// NO_RULE_APPLIED shows the total recorded value of conversions that
    /// do not have a value rule applied.
    /// ORIGINAL shows the original value of conversions to which a value rule
    /// has been applied.
    /// GEO_LOCATION, DEVICE, AUDIENCE show the net adjustment after value
    /// rules were applied.
    #[prost(enumeration="super::enums::conversion_value_rule_primary_dimension_enum::ConversionValueRulePrimaryDimension", tag="138")]
    pub conversion_value_rule_primary_dimension: i32,
    /// Resource name of the ad group criterion that represents webpage criterion.
    #[prost(string, optional, tag="129")]
    pub webpage: ::core::option::Option<::prost::alloc::string::String>,
    /// Week as defined as Monday through Sunday, and represented by the date of
    /// Monday. Formatted as yyyy-MM-dd.
    #[prost(string, optional, tag="130")]
    pub week: ::core::option::Option<::prost::alloc::string::String>,
    /// Year, formatted as yyyy.
    #[prost(int32, optional, tag="131")]
    pub year: ::core::option::Option<i32>,
    /// iOS Store Kit Ad Network conversion value.
    /// Null value means this segment is not applicable, e.g. non-iOS campaign.
    #[prost(int64, optional, tag="137")]
    pub sk_ad_network_conversion_value: ::core::option::Option<i64>,
    /// Only used with CustomerAsset, CampaignAsset and AdGroupAsset metrics.
    /// Indicates whether the interaction metrics occurred on the asset itself
    /// or a different asset or ad unit.
    /// Interactions (e.g. clicks) are counted across all the parts of the served
    /// ad (e.g. Ad itself and other components like Sitelinks) when they are
    /// served together. When interaction_on_this_asset is true, it means the
    /// interactions are on this specific asset and when interaction_on_this_asset
    /// is false, it means the interactions is not on this specific asset but on
    /// other parts of the served ad this asset is served with.
    #[prost(message, optional, tag="139")]
    pub asset_interaction_target: ::core::option::Option<AssetInteractionTarget>,
}
/// A Keyword criterion segment.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Keyword {
    /// The AdGroupCriterion resource name.
    #[prost(string, optional, tag="3")]
    pub ad_group_criterion: ::core::option::Option<::prost::alloc::string::String>,
    /// Keyword info.
    #[prost(message, optional, tag="2")]
    pub info: ::core::option::Option<KeywordInfo>,
}
/// A BudgetCampaignAssociationStatus segment.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BudgetCampaignAssociationStatus {
    /// The campaign resource name.
    #[prost(string, optional, tag="1")]
    pub campaign: ::core::option::Option<::prost::alloc::string::String>,
    /// Budget campaign association status.
    #[prost(enumeration="super::enums::budget_campaign_association_status_enum::BudgetCampaignAssociationStatus", tag="2")]
    pub status: i32,
}
/// An AssetInteractionTarget segment.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetInteractionTarget {
    /// The asset resource name.
    #[prost(string, tag="1")]
    pub asset: ::prost::alloc::string::String,
    /// Only used with CustomerAsset, CampaignAsset and AdGroupAsset metrics.
    /// Indicates whether the interaction metrics occurred on the asset itself or a
    /// different asset or ad unit.
    #[prost(bool, tag="2")]
    pub interaction_on_this_asset: bool,
}
// Proto file describing simulation points.

/// A container for simulation points for simulations of type BID_MODIFIER.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BidModifierSimulationPointList {
    /// Projected metrics for a series of bid modifier amounts.
    #[prost(message, repeated, tag="1")]
    pub points: ::prost::alloc::vec::Vec<BidModifierSimulationPoint>,
}
/// A container for simulation points for simulations of type CPC_BID.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CpcBidSimulationPointList {
    /// Projected metrics for a series of CPC bid amounts.
    #[prost(message, repeated, tag="1")]
    pub points: ::prost::alloc::vec::Vec<CpcBidSimulationPoint>,
}
/// A container for simulation points for simulations of type CPV_BID.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CpvBidSimulationPointList {
    /// Projected metrics for a series of CPV bid amounts.
    #[prost(message, repeated, tag="1")]
    pub points: ::prost::alloc::vec::Vec<CpvBidSimulationPoint>,
}
/// A container for simulation points for simulations of type TARGET_CPA.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TargetCpaSimulationPointList {
    /// Projected metrics for a series of target CPA amounts.
    #[prost(message, repeated, tag="1")]
    pub points: ::prost::alloc::vec::Vec<TargetCpaSimulationPoint>,
}
/// A container for simulation points for simulations of type TARGET_ROAS.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TargetRoasSimulationPointList {
    /// Projected metrics for a series of target ROAS amounts.
    #[prost(message, repeated, tag="1")]
    pub points: ::prost::alloc::vec::Vec<TargetRoasSimulationPoint>,
}
/// A container for simulation points for simulations of type PERCENT_CPC_BID.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PercentCpcBidSimulationPointList {
    /// Projected metrics for a series of percent CPC bid amounts.
    #[prost(message, repeated, tag="1")]
    pub points: ::prost::alloc::vec::Vec<PercentCpcBidSimulationPoint>,
}
/// A container for simulation points for simulations of type BUDGET.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BudgetSimulationPointList {
    /// Projected metrics for a series of budget amounts.
    #[prost(message, repeated, tag="1")]
    pub points: ::prost::alloc::vec::Vec<BudgetSimulationPoint>,
}
/// A container for simulation points for simulations of type
/// TARGET_IMPRESSION_SHARE.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TargetImpressionShareSimulationPointList {
    /// Projected metrics for a specific target impression share value.
    #[prost(message, repeated, tag="1")]
    pub points: ::prost::alloc::vec::Vec<TargetImpressionShareSimulationPoint>,
}
/// Projected metrics for a specific bid modifier amount.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BidModifierSimulationPoint {
    /// The simulated bid modifier upon which projected metrics are based.
    #[prost(double, optional, tag="15")]
    pub bid_modifier: ::core::option::Option<f64>,
    /// Projected number of biddable conversions.
    /// Only search advertising channel type supports this field.
    #[prost(double, optional, tag="16")]
    pub biddable_conversions: ::core::option::Option<f64>,
    /// Projected total value of biddable conversions.
    /// Only search advertising channel type supports this field.
    #[prost(double, optional, tag="17")]
    pub biddable_conversions_value: ::core::option::Option<f64>,
    /// Projected number of clicks.
    #[prost(int64, optional, tag="18")]
    pub clicks: ::core::option::Option<i64>,
    /// Projected cost in micros.
    #[prost(int64, optional, tag="19")]
    pub cost_micros: ::core::option::Option<i64>,
    /// Projected number of impressions.
    #[prost(int64, optional, tag="20")]
    pub impressions: ::core::option::Option<i64>,
    /// Projected number of top slot impressions.
    /// Only search advertising channel type supports this field.
    #[prost(int64, optional, tag="21")]
    pub top_slot_impressions: ::core::option::Option<i64>,
    /// Projected number of biddable conversions for the parent resource.
    /// Only search advertising channel type supports this field.
    #[prost(double, optional, tag="22")]
    pub parent_biddable_conversions: ::core::option::Option<f64>,
    /// Projected total value of biddable conversions for the parent resource.
    /// Only search advertising channel type supports this field.
    #[prost(double, optional, tag="23")]
    pub parent_biddable_conversions_value: ::core::option::Option<f64>,
    /// Projected number of clicks for the parent resource.
    #[prost(int64, optional, tag="24")]
    pub parent_clicks: ::core::option::Option<i64>,
    /// Projected cost in micros for the parent resource.
    #[prost(int64, optional, tag="25")]
    pub parent_cost_micros: ::core::option::Option<i64>,
    /// Projected number of impressions for the parent resource.
    #[prost(int64, optional, tag="26")]
    pub parent_impressions: ::core::option::Option<i64>,
    /// Projected number of top slot impressions for the parent resource.
    /// Only search advertising channel type supports this field.
    #[prost(int64, optional, tag="27")]
    pub parent_top_slot_impressions: ::core::option::Option<i64>,
    /// Projected minimum daily budget that must be available to the parent
    /// resource to realize this simulation.
    #[prost(int64, optional, tag="28")]
    pub parent_required_budget_micros: ::core::option::Option<i64>,
}
/// Projected metrics for a specific CPC bid amount.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CpcBidSimulationPoint {
    /// Projected required daily budget that the advertiser must set in order to
    /// receive the estimated traffic, in micros of advertiser currency.
    #[prost(int64, tag="17")]
    pub required_budget_amount_micros: i64,
    /// Projected number of biddable conversions.
    #[prost(double, optional, tag="9")]
    pub biddable_conversions: ::core::option::Option<f64>,
    /// Projected total value of biddable conversions.
    #[prost(double, optional, tag="10")]
    pub biddable_conversions_value: ::core::option::Option<f64>,
    /// Projected number of clicks.
    #[prost(int64, optional, tag="11")]
    pub clicks: ::core::option::Option<i64>,
    /// Projected cost in micros.
    #[prost(int64, optional, tag="12")]
    pub cost_micros: ::core::option::Option<i64>,
    /// Projected number of impressions.
    #[prost(int64, optional, tag="13")]
    pub impressions: ::core::option::Option<i64>,
    /// Projected number of top slot impressions.
    /// Only search advertising channel type supports this field.
    #[prost(int64, optional, tag="14")]
    pub top_slot_impressions: ::core::option::Option<i64>,
    /// When SimulationModificationMethod = UNIFORM or DEFAULT,
    /// cpc_bid_micros is set.
    /// When SimulationModificationMethod = SCALING,
    /// cpc_bid_scaling_modifier is set.
    #[prost(oneof="cpc_bid_simulation_point::CpcSimulationKeyValue", tags="15, 16")]
    pub cpc_simulation_key_value: ::core::option::Option<cpc_bid_simulation_point::CpcSimulationKeyValue>,
}
/// Nested message and enum types in `CpcBidSimulationPoint`.
pub mod cpc_bid_simulation_point {
    /// When SimulationModificationMethod = UNIFORM or DEFAULT,
    /// cpc_bid_micros is set.
    /// When SimulationModificationMethod = SCALING,
    /// cpc_bid_scaling_modifier is set.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum CpcSimulationKeyValue {
        /// The simulated CPC bid upon which projected metrics are based.
        #[prost(int64, tag="15")]
        CpcBidMicros(i64),
        /// The simulated scaling modifier upon which projected metrics are based.
        /// All CPC bids relevant to the simulated entity are scaled by this
        /// modifier.
        #[prost(double, tag="16")]
        CpcBidScalingModifier(f64),
    }
}
/// Projected metrics for a specific CPV bid amount.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CpvBidSimulationPoint {
    /// The simulated CPV bid upon which projected metrics are based.
    #[prost(int64, optional, tag="5")]
    pub cpv_bid_micros: ::core::option::Option<i64>,
    /// Projected cost in micros.
    #[prost(int64, optional, tag="6")]
    pub cost_micros: ::core::option::Option<i64>,
    /// Projected number of impressions.
    #[prost(int64, optional, tag="7")]
    pub impressions: ::core::option::Option<i64>,
    /// Projected number of views.
    #[prost(int64, optional, tag="8")]
    pub views: ::core::option::Option<i64>,
}
/// Projected metrics for a specific target CPA amount.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TargetCpaSimulationPoint {
    /// Projected required daily budget that the advertiser must set in order to
    /// receive the estimated traffic, in micros of advertiser currency.
    #[prost(int64, tag="19")]
    pub required_budget_amount_micros: i64,
    /// Projected number of biddable conversions.
    #[prost(double, optional, tag="9")]
    pub biddable_conversions: ::core::option::Option<f64>,
    /// Projected total value of biddable conversions.
    #[prost(double, optional, tag="10")]
    pub biddable_conversions_value: ::core::option::Option<f64>,
    /// Projected number of app installs.
    #[prost(double, tag="15")]
    pub app_installs: f64,
    /// Projected number of in-app actions.
    #[prost(double, tag="16")]
    pub in_app_actions: f64,
    /// Projected number of clicks.
    #[prost(int64, optional, tag="11")]
    pub clicks: ::core::option::Option<i64>,
    /// Projected cost in micros.
    #[prost(int64, optional, tag="12")]
    pub cost_micros: ::core::option::Option<i64>,
    /// Projected number of impressions.
    #[prost(int64, optional, tag="13")]
    pub impressions: ::core::option::Option<i64>,
    /// Projected number of top slot impressions.
    /// Only search advertising channel type supports this field.
    #[prost(int64, optional, tag="14")]
    pub top_slot_impressions: ::core::option::Option<i64>,
    /// When SimulationModificationMethod = UNIFORM or DEFAULT,
    /// target_cpa_micros is set.
    /// When SimulationModificationMethod = SCALING,
    /// target_cpa_scaling_modifier is set.
    #[prost(oneof="target_cpa_simulation_point::TargetCpaSimulationKeyValue", tags="17, 18")]
    pub target_cpa_simulation_key_value: ::core::option::Option<target_cpa_simulation_point::TargetCpaSimulationKeyValue>,
}
/// Nested message and enum types in `TargetCpaSimulationPoint`.
pub mod target_cpa_simulation_point {
    /// When SimulationModificationMethod = UNIFORM or DEFAULT,
    /// target_cpa_micros is set.
    /// When SimulationModificationMethod = SCALING,
    /// target_cpa_scaling_modifier is set.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum TargetCpaSimulationKeyValue {
        /// The simulated target CPA upon which projected metrics are based.
        #[prost(int64, tag="17")]
        TargetCpaMicros(i64),
        /// The simulated scaling modifier upon which projected metrics are based.
        /// All CPA targets relevant to the simulated entity are scaled by this
        /// modifier.
        #[prost(double, tag="18")]
        TargetCpaScalingModifier(f64),
    }
}
/// Projected metrics for a specific target ROAS amount.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TargetRoasSimulationPoint {
    /// The simulated target ROAS upon which projected metrics are based.
    #[prost(double, optional, tag="8")]
    pub target_roas: ::core::option::Option<f64>,
    /// Projected required daily budget that the advertiser must set in order to
    /// receive the estimated traffic, in micros of advertiser currency.
    #[prost(int64, tag="15")]
    pub required_budget_amount_micros: i64,
    /// Projected number of biddable conversions.
    #[prost(double, optional, tag="9")]
    pub biddable_conversions: ::core::option::Option<f64>,
    /// Projected total value of biddable conversions.
    #[prost(double, optional, tag="10")]
    pub biddable_conversions_value: ::core::option::Option<f64>,
    /// Projected number of clicks.
    #[prost(int64, optional, tag="11")]
    pub clicks: ::core::option::Option<i64>,
    /// Projected cost in micros.
    #[prost(int64, optional, tag="12")]
    pub cost_micros: ::core::option::Option<i64>,
    /// Projected number of impressions.
    #[prost(int64, optional, tag="13")]
    pub impressions: ::core::option::Option<i64>,
    /// Projected number of top slot impressions.
    /// Only Search advertising channel type supports this field.
    #[prost(int64, optional, tag="14")]
    pub top_slot_impressions: ::core::option::Option<i64>,
}
/// Projected metrics for a specific percent CPC amount. Only Hotel advertising
/// channel type supports this field.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PercentCpcBidSimulationPoint {
    /// The simulated percent CPC upon which projected metrics are based. Percent
    /// CPC expressed as fraction of the advertised price for some good or service.
    /// The value stored here is 1,000,000 * \[fraction\].
    #[prost(int64, optional, tag="1")]
    pub percent_cpc_bid_micros: ::core::option::Option<i64>,
    /// Projected number of biddable conversions.
    #[prost(double, optional, tag="2")]
    pub biddable_conversions: ::core::option::Option<f64>,
    /// Projected total value of biddable conversions in local currency.
    #[prost(double, optional, tag="3")]
    pub biddable_conversions_value: ::core::option::Option<f64>,
    /// Projected number of clicks.
    #[prost(int64, optional, tag="4")]
    pub clicks: ::core::option::Option<i64>,
    /// Projected cost in micros.
    #[prost(int64, optional, tag="5")]
    pub cost_micros: ::core::option::Option<i64>,
    /// Projected number of impressions.
    #[prost(int64, optional, tag="6")]
    pub impressions: ::core::option::Option<i64>,
    /// Projected number of top slot impressions.
    #[prost(int64, optional, tag="7")]
    pub top_slot_impressions: ::core::option::Option<i64>,
}
/// Projected metrics for a specific budget amount.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BudgetSimulationPoint {
    /// The simulated budget upon which projected metrics are based.
    #[prost(int64, tag="1")]
    pub budget_amount_micros: i64,
    /// Projected required daily cpc bid ceiling that the advertiser must set to
    /// realize this simulation, in micros of the advertiser currency.
    /// Only campaigns with the Target Spend bidding strategy support this field.
    #[prost(int64, tag="2")]
    pub required_cpc_bid_ceiling_micros: i64,
    /// Projected number of biddable conversions.
    #[prost(double, tag="3")]
    pub biddable_conversions: f64,
    /// Projected total value of biddable conversions.
    #[prost(double, tag="4")]
    pub biddable_conversions_value: f64,
    /// Projected number of clicks.
    #[prost(int64, tag="5")]
    pub clicks: i64,
    /// Projected cost in micros.
    #[prost(int64, tag="6")]
    pub cost_micros: i64,
    /// Projected number of impressions.
    #[prost(int64, tag="7")]
    pub impressions: i64,
    /// Projected number of top slot impressions.
    /// Only search advertising channel type supports this field.
    #[prost(int64, tag="8")]
    pub top_slot_impressions: i64,
}
/// Projected metrics for a specific target impression share value.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TargetImpressionShareSimulationPoint {
    /// The simulated target impression share value (in micros) upon which
    /// projected metrics are based.
    /// E.g. 10% impression share, which is equal to 0.1, is stored as 100_000.
    /// This value is validated and will not exceed 1M (100%).
    #[prost(int64, tag="1")]
    pub target_impression_share_micros: i64,
    /// Projected required daily cpc bid ceiling that the advertiser must set to
    /// realize this simulation, in micros of the advertiser currency.
    #[prost(int64, tag="2")]
    pub required_cpc_bid_ceiling_micros: i64,
    /// Projected required daily budget that the advertiser must set in order to
    /// receive the estimated traffic, in micros of advertiser currency.
    #[prost(int64, tag="3")]
    pub required_budget_amount_micros: i64,
    /// Projected number of biddable conversions.
    #[prost(double, tag="4")]
    pub biddable_conversions: f64,
    /// Projected total value of biddable conversions.
    #[prost(double, tag="5")]
    pub biddable_conversions_value: f64,
    /// Projected number of clicks.
    #[prost(int64, tag="6")]
    pub clicks: i64,
    /// Projected cost in micros.
    #[prost(int64, tag="7")]
    pub cost_micros: i64,
    /// Projected number of impressions.
    #[prost(int64, tag="8")]
    pub impressions: i64,
    /// Projected number of top slot impressions.
    /// Only search advertising channel type supports this field.
    #[prost(int64, tag="9")]
    pub top_slot_impressions: i64,
    /// Projected number of absolute top impressions.
    /// Only search advertising channel type supports this field.
    #[prost(int64, tag="10")]
    pub absolute_top_impressions: i64,
}
// Proto file describing TagSnippet

/// The site tag and event snippet pair for a TrackingCodeType.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TagSnippet {
    /// The type of the generated tag snippets for tracking conversions.
    #[prost(enumeration="super::enums::tracking_code_type_enum::TrackingCodeType", tag="1")]
    pub r#type: i32,
    /// The format of the web page where the tracking tag and snippet will be
    /// installed, e.g. HTML.
    #[prost(enumeration="super::enums::tracking_code_page_format_enum::TrackingCodePageFormat", tag="2")]
    pub page_format: i32,
    /// The site tag that adds visitors to your basic remarketing lists and sets
    /// new cookies on your domain.
    #[prost(string, optional, tag="5")]
    pub global_site_tag: ::core::option::Option<::prost::alloc::string::String>,
    /// The event snippet that works with the site tag to track actions that
    /// should be counted as conversions.
    #[prost(string, optional, tag="6")]
    pub event_snippet: ::core::option::Option<::prost::alloc::string::String>,
}
// Proto file describing TargetingSetting

/// Settings for the targeting-related features, at the campaign and ad group
/// levels. For more details about the targeting setting, visit
/// <https://support.google.com/google-ads/answer/7365594>
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TargetingSetting {
    /// The per-targeting-dimension setting to restrict the reach of your campaign
    /// or ad group.
    #[prost(message, repeated, tag="1")]
    pub target_restrictions: ::prost::alloc::vec::Vec<TargetRestriction>,
    /// The list of operations changing the target restrictions.
    ///
    /// Adding a target restriction with a targeting dimension that already exists
    /// causes the existing target restriction to be replaced with the new value.
    #[prost(message, repeated, tag="2")]
    pub target_restriction_operations: ::prost::alloc::vec::Vec<TargetRestrictionOperation>,
}
/// The list of per-targeting-dimension targeting settings.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TargetRestriction {
    /// The targeting dimension that these settings apply to.
    #[prost(enumeration="super::enums::targeting_dimension_enum::TargetingDimension", tag="1")]
    pub targeting_dimension: i32,
    /// Indicates whether to restrict your ads to show only for the criteria you
    /// have selected for this targeting_dimension, or to target all values for
    /// this targeting_dimension and show ads based on your targeting in other
    /// TargetingDimensions. A value of `true` means that these criteria will only
    /// apply bid modifiers, and not affect targeting. A value of `false` means
    /// that these criteria will restrict targeting as well as applying bid
    /// modifiers.
    #[prost(bool, optional, tag="3")]
    pub bid_only: ::core::option::Option<bool>,
}
/// Operation to be performed on a target restriction list in a mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TargetRestrictionOperation {
    /// Type of list operation to perform.
    #[prost(enumeration="target_restriction_operation::Operator", tag="1")]
    pub operator: i32,
    /// The target restriction being added to or removed from the list.
    #[prost(message, optional, tag="2")]
    pub value: ::core::option::Option<TargetRestriction>,
}
/// Nested message and enum types in `TargetRestrictionOperation`.
pub mod target_restriction_operation {
    /// The operator.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Operator {
        /// Unspecified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Add the restriction to the existing restrictions.
        Add = 2,
        /// Remove the restriction from the existing restrictions.
        Remove = 3,
    }
    impl Operator {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Operator::Unspecified => "UNSPECIFIED",
                Operator::Unknown => "UNKNOWN",
                Operator::Add => "ADD",
                Operator::Remove => "REMOVE",
            }
        }
    }
}
/// A type of label displaying text on a colored background.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TextLabel {
    /// Background color of the label in RGB format. This string must match the
    /// regular expression '^\#(\[a-fA-F0-9]{6}|[a-fA-F0-9\]{3})$'.
    /// Note: The background color may not be visible for manager accounts.
    #[prost(string, optional, tag="3")]
    pub background_color: ::core::option::Option<::prost::alloc::string::String>,
    /// A short description of the label. The length must be no more than 200
    /// characters.
    #[prost(string, optional, tag="4")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
}
// Proto file UrlCollection type.

/// Collection of urls that is tagged with a unique identifier.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UrlCollection {
    /// Unique identifier for this UrlCollection instance.
    #[prost(string, optional, tag="5")]
    pub url_collection_id: ::core::option::Option<::prost::alloc::string::String>,
    /// A list of possible final URLs.
    #[prost(string, repeated, tag="6")]
    pub final_urls: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// A list of possible final mobile URLs.
    #[prost(string, repeated, tag="7")]
    pub final_mobile_urls: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// URL template for constructing a tracking URL.
    #[prost(string, optional, tag="8")]
    pub tracking_url_template: ::core::option::Option<::prost::alloc::string::String>,
}
// Proto file describing user list types.

/// SimilarUserList is a list of users which are similar to users from another
/// UserList. These lists are read-only and automatically created by Google.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SimilarUserListInfo {
    /// Seed UserList from which this list is derived.
    #[prost(string, optional, tag="2")]
    pub seed_user_list: ::core::option::Option<::prost::alloc::string::String>,
}
/// UserList of CRM users provided by the advertiser.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CrmBasedUserListInfo {
    /// A string that uniquely identifies a mobile application from which the data
    /// was collected.
    /// For iOS, the ID string is the 9 digit string that appears at the end of an
    /// App Store URL (e.g., "476943146" for "Flood-It! 2" whose App Store link is
    /// <http://itunes.apple.com/us/app/flood-it!-2/id476943146>).
    /// For Android, the ID string is the application's package name
    /// (e.g., "com.labpixies.colordrips" for "Color Drips" given Google Play link
    /// <https://play.google.com/store/apps/details?id=com.labpixies.colordrips>).
    /// Required when creating CrmBasedUserList for uploading mobile advertising
    /// IDs.
    #[prost(string, optional, tag="4")]
    pub app_id: ::core::option::Option<::prost::alloc::string::String>,
    /// Matching key type of the list.
    /// Mixed data types are not allowed on the same list.
    /// This field is required for an ADD operation.
    #[prost(enumeration="super::enums::customer_match_upload_key_type_enum::CustomerMatchUploadKeyType", tag="2")]
    pub upload_key_type: i32,
    /// Data source of the list. Default value is FIRST_PARTY.
    /// Only customers on the allow-list can create third-party sourced CRM lists.
    #[prost(enumeration="super::enums::user_list_crm_data_source_type_enum::UserListCrmDataSourceType", tag="3")]
    pub data_source_type: i32,
}
/// A client defined rule based on custom parameters sent by web sites or
/// uploaded by the advertiser.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserListRuleInfo {
    /// Rule type is used to determine how to group rule items.
    ///
    /// The default is OR of ANDs (disjunctive normal form).
    /// That is, rule items will be ANDed together within rule item groups and the
    /// groups themselves will be ORed together.
    ///
    /// Currently AND of ORs (conjunctive normal form) is only supported for
    /// ExpressionRuleUserList.
    #[prost(enumeration="super::enums::user_list_rule_type_enum::UserListRuleType", tag="1")]
    pub rule_type: i32,
    /// List of rule item groups that defines this rule.
    /// Rule item groups are grouped together based on rule_type.
    #[prost(message, repeated, tag="2")]
    pub rule_item_groups: ::prost::alloc::vec::Vec<UserListRuleItemGroupInfo>,
}
/// A group of rule items.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserListRuleItemGroupInfo {
    /// Rule items that will be grouped together based on rule_type.
    #[prost(message, repeated, tag="1")]
    pub rule_items: ::prost::alloc::vec::Vec<UserListRuleItemInfo>,
}
/// An atomic rule item.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserListRuleItemInfo {
    /// Rule variable name. It should match the corresponding key name fired
    /// by the pixel.
    /// A name must begin with US-ascii letters or underscore or UTF8 code that is
    /// greater than 127 and consist of US-ascii letters or digits or underscore or
    /// UTF8 code that is greater than 127.
    /// For websites, there are two built-in variable URL (name = 'url__') and
    /// referrer URL (name = 'ref_url__').
    /// This field must be populated when creating a new rule item.
    #[prost(string, optional, tag="5")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    /// An atomic rule item.
    #[prost(oneof="user_list_rule_item_info::RuleItem", tags="2, 3, 4")]
    pub rule_item: ::core::option::Option<user_list_rule_item_info::RuleItem>,
}
/// Nested message and enum types in `UserListRuleItemInfo`.
pub mod user_list_rule_item_info {
    /// An atomic rule item.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum RuleItem {
        /// An atomic rule item composed of a number operation.
        #[prost(message, tag="2")]
        NumberRuleItem(super::UserListNumberRuleItemInfo),
        /// An atomic rule item composed of a string operation.
        #[prost(message, tag="3")]
        StringRuleItem(super::UserListStringRuleItemInfo),
        /// An atomic rule item composed of a date operation.
        #[prost(message, tag="4")]
        DateRuleItem(super::UserListDateRuleItemInfo),
    }
}
/// A rule item composed of a date operation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserListDateRuleItemInfo {
    /// Date comparison operator.
    /// This field is required and must be populated when creating new date
    /// rule item.
    #[prost(enumeration="super::enums::user_list_date_rule_item_operator_enum::UserListDateRuleItemOperator", tag="1")]
    pub operator: i32,
    /// String representing date value to be compared with the rule variable.
    /// Supported date format is YYYY-MM-DD.
    /// Times are reported in the customer's time zone.
    #[prost(string, optional, tag="4")]
    pub value: ::core::option::Option<::prost::alloc::string::String>,
    /// The relative date value of the right hand side denoted by number of days
    /// offset from now. The value field will override this field when both are
    /// present.
    #[prost(int64, optional, tag="5")]
    pub offset_in_days: ::core::option::Option<i64>,
}
/// A rule item composed of a number operation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserListNumberRuleItemInfo {
    /// Number comparison operator.
    /// This field is required and must be populated when creating a new number
    /// rule item.
    #[prost(enumeration="super::enums::user_list_number_rule_item_operator_enum::UserListNumberRuleItemOperator", tag="1")]
    pub operator: i32,
    /// Number value to be compared with the variable.
    /// This field is required and must be populated when creating a new number
    /// rule item.
    #[prost(double, optional, tag="3")]
    pub value: ::core::option::Option<f64>,
}
/// A rule item composed of a string operation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserListStringRuleItemInfo {
    /// String comparison operator.
    /// This field is required and must be populated when creating a new string
    /// rule item.
    #[prost(enumeration="super::enums::user_list_string_rule_item_operator_enum::UserListStringRuleItemOperator", tag="1")]
    pub operator: i32,
    /// The right hand side of the string rule item. For URLs or referrer URLs,
    /// the value can not contain illegal URL chars such as newlines, quotes,
    /// tabs, or parentheses. This field is required and must be populated when
    /// creating a new string rule item.
    #[prost(string, optional, tag="3")]
    pub value: ::core::option::Option<::prost::alloc::string::String>,
}
/// User lists defined by combining two rules, left operand and right operand.
/// There are two operators: AND where left operand and right operand have to be
/// true; AND_NOT where left operand is true but right operand is false.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CombinedRuleUserListInfo {
    /// Left operand of the combined rule.
    /// This field is required and must be populated when creating new combined
    /// rule based user list.
    #[prost(message, optional, tag="1")]
    pub left_operand: ::core::option::Option<UserListRuleInfo>,
    /// Right operand of the combined rule.
    /// This field is required and must be populated when creating new combined
    /// rule based user list.
    #[prost(message, optional, tag="2")]
    pub right_operand: ::core::option::Option<UserListRuleInfo>,
    /// Operator to connect the two operands.
    ///
    /// Required for creating a combined rule user list.
    #[prost(enumeration="super::enums::user_list_combined_rule_operator_enum::UserListCombinedRuleOperator", tag="3")]
    pub rule_operator: i32,
}
/// Visitors of a page during specific dates.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DateSpecificRuleUserListInfo {
    /// Boolean rule that defines visitor of a page.
    ///
    /// Required for creating a date specific rule user list.
    #[prost(message, optional, tag="1")]
    pub rule: ::core::option::Option<UserListRuleInfo>,
    /// Start date of users visit. If set to 2000-01-01, then the list includes all
    /// users before end_date. The date's format should be YYYY-MM-DD.
    ///
    /// Required for creating a data specific rule user list.
    #[prost(string, optional, tag="4")]
    pub start_date: ::core::option::Option<::prost::alloc::string::String>,
    /// Last date of users visit. If set to 2037-12-30, then the list includes all
    /// users after start_date. The date's format should be YYYY-MM-DD.
    ///
    /// Required for creating a data specific rule user list.
    #[prost(string, optional, tag="5")]
    pub end_date: ::core::option::Option<::prost::alloc::string::String>,
}
/// Visitors of a page. The page visit is defined by one boolean rule expression.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExpressionRuleUserListInfo {
    /// Boolean rule that defines this user list. The rule consists of a list of
    /// rule item groups and each rule item group consists of a list of rule items.
    /// All the rule item groups are ORed or ANDed together for evaluation based on
    /// rule.rule_type.
    ///
    /// Required for creating an expression rule user list.
    #[prost(message, optional, tag="1")]
    pub rule: ::core::option::Option<UserListRuleInfo>,
}
/// Representation of a userlist that is generated by a rule.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RuleBasedUserListInfo {
    /// The status of pre-population. The field is default to NONE if not set which
    /// means the previous users will not be considered. If set to REQUESTED, past
    /// site visitors or app users who match the list definition will be included
    /// in the list (works on the Display Network only). This will only
    /// add past users from within the last 30 days, depending on the
    /// list's membership duration and the date when the remarketing tag is added.
    /// The status will be updated to FINISHED once request is processed, or FAILED
    /// if the request fails.
    #[prost(enumeration="super::enums::user_list_prepopulation_status_enum::UserListPrepopulationStatus", tag="1")]
    pub prepopulation_status: i32,
    /// Subtypes of rule based user lists.
    #[prost(oneof="rule_based_user_list_info::RuleBasedUserList", tags="2, 3, 4")]
    pub rule_based_user_list: ::core::option::Option<rule_based_user_list_info::RuleBasedUserList>,
}
/// Nested message and enum types in `RuleBasedUserListInfo`.
pub mod rule_based_user_list_info {
    /// Subtypes of rule based user lists.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum RuleBasedUserList {
        /// User lists defined by combining two rules.
        /// There are two operators: AND, where the left and right operands have to
        /// be true; AND_NOT where left operand is true but right operand is false.
        #[prost(message, tag="2")]
        CombinedRuleUserList(super::CombinedRuleUserListInfo),
        /// Visitors of a page during specific dates. The visiting periods are
        /// defined as follows:
        /// Between start_date (inclusive) and end_date (inclusive);
        /// Before end_date (exclusive) with start_date = 2000-01-01;
        /// After start_date (exclusive) with end_date = 2037-12-30.
        #[prost(message, tag="3")]
        DateSpecificRuleUserList(super::DateSpecificRuleUserListInfo),
        /// Visitors of a page. The page visit is defined by one boolean rule
        /// expression.
        #[prost(message, tag="4")]
        ExpressionRuleUserList(super::ExpressionRuleUserListInfo),
    }
}
/// Represents a user list that is a custom combination of user lists.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LogicalUserListInfo {
    /// Logical list rules that define this user list. The rules are defined as a
    /// logical operator (ALL/ANY/NONE) and a list of user lists. All the rules are
    /// ANDed when they are evaluated.
    ///
    /// Required for creating a logical user list.
    #[prost(message, repeated, tag="1")]
    pub rules: ::prost::alloc::vec::Vec<UserListLogicalRuleInfo>,
}
/// A user list logical rule. A rule has a logical operator (and/or/not) and a
/// list of user lists as operands.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserListLogicalRuleInfo {
    /// The logical operator of the rule.
    #[prost(enumeration="super::enums::user_list_logical_rule_operator_enum::UserListLogicalRuleOperator", tag="1")]
    pub operator: i32,
    /// The list of operands of the rule.
    #[prost(message, repeated, tag="2")]
    pub rule_operands: ::prost::alloc::vec::Vec<LogicalUserListOperandInfo>,
}
/// Operand of logical user list that consists of a user list.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LogicalUserListOperandInfo {
    /// Resource name of a user list as an operand.
    #[prost(string, optional, tag="2")]
    pub user_list: ::core::option::Option<::prost::alloc::string::String>,
}
/// User list targeting as a collection of conversions or remarketing actions.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BasicUserListInfo {
    /// Actions associated with this user list.
    #[prost(message, repeated, tag="1")]
    pub actions: ::prost::alloc::vec::Vec<UserListActionInfo>,
}
/// Represents an action type used for building remarketing user lists.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserListActionInfo {
    /// Subtypes of user list action.
    #[prost(oneof="user_list_action_info::UserListAction", tags="3, 4")]
    pub user_list_action: ::core::option::Option<user_list_action_info::UserListAction>,
}
/// Nested message and enum types in `UserListActionInfo`.
pub mod user_list_action_info {
    /// Subtypes of user list action.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum UserListAction {
        /// A conversion action that's not generated from remarketing.
        #[prost(string, tag="3")]
        ConversionAction(::prost::alloc::string::String),
        /// A remarketing action.
        #[prost(string, tag="4")]
        RemarketingAction(::prost::alloc::string::String),
    }
}
// Proto file describing value types.

/// A generic data container.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Value {
    /// A value.
    #[prost(oneof="value::Value", tags="1, 2, 3, 4, 5")]
    pub value: ::core::option::Option<value::Value>,
}
/// Nested message and enum types in `Value`.
pub mod value {
    /// A value.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Value {
        /// A boolean.
        #[prost(bool, tag="1")]
        BooleanValue(bool),
        /// An int64.
        #[prost(int64, tag="2")]
        Int64Value(i64),
        /// A float.
        #[prost(float, tag="3")]
        FloatValue(f32),
        /// A double.
        #[prost(double, tag="4")]
        DoubleValue(f64),
        /// A string.
        #[prost(string, tag="5")]
        StringValue(::prost::alloc::string::String),
    }
}
