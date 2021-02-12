// Proto file describing assets used inside an ad.

/// A text asset used inside an ad.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdTextAsset {
    /// Asset text.
    #[prost(message, optional, tag = "1")]
    pub text: ::core::option::Option<::prost::alloc::string::String>,
    /// The pinned field of the asset. This restricts the asset to only serve
    /// within this field. Multiple assets can be pinned to the same field. An
    /// asset that is unpinned or pinned to a different field will not serve in a
    /// field where some other asset has been pinned.
    #[prost(
        enumeration = "super::enums::served_asset_field_type_enum::ServedAssetFieldType",
        tag = "2"
    )]
    pub pinned_field: i32,
}
/// An image asset used inside an ad.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdImageAsset {
    /// The Asset resource name of this image.
    #[prost(message, optional, tag = "1")]
    pub asset: ::core::option::Option<::prost::alloc::string::String>,
}
/// A video asset used inside an ad.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdVideoAsset {
    /// The Asset resource name of this video.
    #[prost(message, optional, tag = "1")]
    pub asset: ::core::option::Option<::prost::alloc::string::String>,
}
/// A media bundle asset used inside an ad.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdMediaBundleAsset {
    /// The Asset resource name of this media bundle.
    #[prost(message, optional, tag = "1")]
    pub asset: ::core::option::Option<::prost::alloc::string::String>,
}
// Proto file containing info messages for specific ad types.

/// A text ad.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TextAdInfo {
    /// The headline of the ad.
    #[prost(message, optional, tag = "1")]
    pub headline: ::core::option::Option<::prost::alloc::string::String>,
    /// The first line of the ad's description.
    #[prost(message, optional, tag = "2")]
    pub description1: ::core::option::Option<::prost::alloc::string::String>,
    /// The second line of the ad's description.
    #[prost(message, optional, tag = "3")]
    pub description2: ::core::option::Option<::prost::alloc::string::String>,
}
/// An expanded text ad.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExpandedTextAdInfo {
    /// The first part of the ad's headline.
    #[prost(message, optional, tag = "1")]
    pub headline_part1: ::core::option::Option<::prost::alloc::string::String>,
    /// The second part of the ad's headline.
    #[prost(message, optional, tag = "2")]
    pub headline_part2: ::core::option::Option<::prost::alloc::string::String>,
    /// The third part of the ad's headline.
    #[prost(message, optional, tag = "6")]
    pub headline_part3: ::core::option::Option<::prost::alloc::string::String>,
    /// The description of the ad.
    #[prost(message, optional, tag = "3")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
    /// The second description of the ad.
    #[prost(message, optional, tag = "7")]
    pub description2: ::core::option::Option<::prost::alloc::string::String>,
    /// The text that can appear alongside the ad's displayed URL.
    #[prost(message, optional, tag = "4")]
    pub path1: ::core::option::Option<::prost::alloc::string::String>,
    /// Additional text that can appear alongside the ad's displayed URL.
    #[prost(message, optional, tag = "5")]
    pub path2: ::core::option::Option<::prost::alloc::string::String>,
}
/// A call-only ad.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CallOnlyAdInfo {
    /// The country code in the ad.
    #[prost(message, optional, tag = "1")]
    pub country_code: ::core::option::Option<::prost::alloc::string::String>,
    /// The phone number in the ad.
    #[prost(message, optional, tag = "2")]
    pub phone_number: ::core::option::Option<::prost::alloc::string::String>,
    /// The business name in the ad.
    #[prost(message, optional, tag = "3")]
    pub business_name: ::core::option::Option<::prost::alloc::string::String>,
    /// First headline in the ad.
    #[prost(message, optional, tag = "11")]
    pub headline1: ::core::option::Option<::prost::alloc::string::String>,
    /// Second headline in the ad.
    #[prost(message, optional, tag = "12")]
    pub headline2: ::core::option::Option<::prost::alloc::string::String>,
    /// The first line of the ad's description.
    #[prost(message, optional, tag = "4")]
    pub description1: ::core::option::Option<::prost::alloc::string::String>,
    /// The second line of the ad's description.
    #[prost(message, optional, tag = "5")]
    pub description2: ::core::option::Option<::prost::alloc::string::String>,
    /// Whether to enable call tracking for the creative. Enabling call
    /// tracking also enables call conversions.
    #[prost(message, optional, tag = "6")]
    pub call_tracked: ::core::option::Option<bool>,
    /// Whether to disable call conversion for the creative.
    /// If set to `true`, disables call conversions even when `call_tracked` is
    /// `true`.
    /// If `call_tracked` is `false`, this field is ignored.
    #[prost(message, optional, tag = "7")]
    pub disable_call_conversion: ::core::option::Option<bool>,
    /// The URL to be used for phone number verification.
    #[prost(message, optional, tag = "8")]
    pub phone_number_verification_url: ::core::option::Option<::prost::alloc::string::String>,
    /// The conversion action to attribute a call conversion to. If not set a
    /// default conversion action is used. This field only has effect if
    /// call_tracked is set to true. Otherwise this field is ignored.
    #[prost(message, optional, tag = "9")]
    pub conversion_action: ::core::option::Option<::prost::alloc::string::String>,
    /// The call conversion behavior of this call only ad. It can use its own call
    /// conversion setting, inherit the account level setting, or be disabled.
    #[prost(
        enumeration = "super::enums::call_conversion_reporting_state_enum::CallConversionReportingState",
        tag = "10"
    )]
    pub conversion_reporting_state: i32,
}
/// An expanded dynamic search ad.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExpandedDynamicSearchAdInfo {
    /// The description of the ad.
    #[prost(message, optional, tag = "1")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
    /// The second description of the ad.
    #[prost(message, optional, tag = "2")]
    pub description2: ::core::option::Option<::prost::alloc::string::String>,
}
/// A hotel ad.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HotelAdInfo {}
/// A Smart Shopping ad.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShoppingSmartAdInfo {}
/// A standard Shopping ad.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShoppingProductAdInfo {}
/// A Shopping Comparison Listing ad.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShoppingComparisonListingAdInfo {
    /// Headline of the ad. This field is required. Allowed length is between 25
    /// and 45 characters.
    #[prost(message, optional, tag = "1")]
    pub headline: ::core::option::Option<::prost::alloc::string::String>,
}
/// A Gmail ad.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GmailAdInfo {
    /// The Gmail teaser.
    #[prost(message, optional, tag = "1")]
    pub teaser: ::core::option::Option<GmailTeaser>,
    /// The MediaFile resource name of the header image. Valid image types are GIF,
    /// JPEG and PNG. The minimum size is 300x100 pixels and the aspect ratio must
    /// be between 3:1 and 5:1 (+-1%).
    #[prost(message, optional, tag = "2")]
    pub header_image: ::core::option::Option<::prost::alloc::string::String>,
    /// The MediaFile resource name of the marketing image. Valid image types are
    /// GIF, JPEG and PNG. The image must either be landscape with a minimum size
    /// of 600x314 pixels and aspect ratio of 600:314 (+-1%) or square with a
    /// minimum size of 300x300 pixels and aspect ratio of 1:1 (+-1%)
    #[prost(message, optional, tag = "3")]
    pub marketing_image: ::core::option::Option<::prost::alloc::string::String>,
    /// Headline of the marketing image.
    #[prost(message, optional, tag = "4")]
    pub marketing_image_headline: ::core::option::Option<::prost::alloc::string::String>,
    /// Description of the marketing image.
    #[prost(message, optional, tag = "5")]
    pub marketing_image_description: ::core::option::Option<::prost::alloc::string::String>,
    /// Display-call-to-action of the marketing image.
    #[prost(message, optional, tag = "6")]
    pub marketing_image_display_call_to_action: ::core::option::Option<DisplayCallToAction>,
    /// Product images. Up to 15 images are supported.
    #[prost(message, repeated, tag = "7")]
    pub product_images: ::prost::alloc::vec::Vec<ProductImage>,
    /// Product videos. Up to 7 videos are supported. At least one product video
    /// or a marketing image must be specified.
    #[prost(message, repeated, tag = "8")]
    pub product_videos: ::prost::alloc::vec::Vec<ProductVideo>,
}
/// Gmail teaser data. The teaser is a small header that acts as an invitation
/// to view the rest of the ad (the body).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GmailTeaser {
    /// Headline of the teaser.
    #[prost(message, optional, tag = "1")]
    pub headline: ::core::option::Option<::prost::alloc::string::String>,
    /// Description of the teaser.
    #[prost(message, optional, tag = "2")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
    /// Business name of the advertiser.
    #[prost(message, optional, tag = "3")]
    pub business_name: ::core::option::Option<::prost::alloc::string::String>,
    /// The MediaFile resource name of the logo image. Valid image types are GIF,
    /// JPEG and PNG. The minimum size is 144x144 pixels and the aspect ratio must
    /// be 1:1 (+-1%).
    #[prost(message, optional, tag = "4")]
    pub logo_image: ::core::option::Option<::prost::alloc::string::String>,
}
/// Data for display call to action. The call to action is a piece of the ad
/// that prompts the user to do something. Like clicking a link or making a phone
/// call.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DisplayCallToAction {
    /// Text for the display-call-to-action.
    #[prost(message, optional, tag = "1")]
    pub text: ::core::option::Option<::prost::alloc::string::String>,
    /// Text color for the display-call-to-action in hexadecimal, e.g. #ffffff for
    /// white.
    #[prost(message, optional, tag = "2")]
    pub text_color: ::core::option::Option<::prost::alloc::string::String>,
    /// Identifies the url collection in the ad.url_collections field. If not set
    /// the url defaults to final_url.
    #[prost(message, optional, tag = "3")]
    pub url_collection_id: ::core::option::Option<::prost::alloc::string::String>,
}
/// Product image specific data.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProductImage {
    /// The MediaFile resource name of the product image. Valid image types are
    /// GIF, JPEG and PNG. The minimum size is 300x300 pixels and the aspect ratio
    /// must be 1:1 (+-1%).
    #[prost(message, optional, tag = "1")]
    pub product_image: ::core::option::Option<::prost::alloc::string::String>,
    /// Description of the product.
    #[prost(message, optional, tag = "2")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
    /// Display-call-to-action of the product image.
    #[prost(message, optional, tag = "3")]
    pub display_call_to_action: ::core::option::Option<DisplayCallToAction>,
}
/// Product video specific data.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProductVideo {
    /// The MediaFile resource name of a video which must be hosted on YouTube.
    #[prost(message, optional, tag = "1")]
    pub product_video: ::core::option::Option<::prost::alloc::string::String>,
}
/// An image ad.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImageAdInfo {
    /// Width in pixels of the full size image.
    #[prost(message, optional, tag = "4")]
    pub pixel_width: ::core::option::Option<i64>,
    /// Height in pixels of the full size image.
    #[prost(message, optional, tag = "5")]
    pub pixel_height: ::core::option::Option<i64>,
    /// URL of the full size image.
    #[prost(message, optional, tag = "6")]
    pub image_url: ::core::option::Option<::prost::alloc::string::String>,
    /// Width in pixels of the preview size image.
    #[prost(message, optional, tag = "7")]
    pub preview_pixel_width: ::core::option::Option<i64>,
    /// Height in pixels of the preview size image.
    #[prost(message, optional, tag = "8")]
    pub preview_pixel_height: ::core::option::Option<i64>,
    /// URL of the preview size image.
    #[prost(message, optional, tag = "9")]
    pub preview_image_url: ::core::option::Option<::prost::alloc::string::String>,
    /// The mime type of the image.
    #[prost(enumeration = "super::enums::mime_type_enum::MimeType", tag = "10")]
    pub mime_type: i32,
    /// The name of the image. If the image was created from a MediaFile, this is
    /// the MediaFile's name. If the image was created from bytes, this is empty.
    #[prost(message, optional, tag = "11")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    /// The image to create the ImageAd from. This can be specified in one of
    /// two ways.
    /// 1. An existing MediaFile resource.
    /// 2. The raw image data as bytes.
    #[prost(oneof = "image_ad_info::Image", tags = "1, 2, 3")]
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
        #[prost(message, tag = "1")]
        MediaFile(::prost::alloc::string::String),
        /// Raw image data as bytes.
        #[prost(message, tag = "2")]
        Data(::prost::alloc::vec::Vec<u8>),
        /// An ad ID to copy the image from.
        #[prost(message, tag = "3")]
        AdIdToCopyImageFrom(i64),
    }
}
/// Representation of video bumper in-stream ad format (very short in-stream
/// non-skippable video ad).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VideoBumperInStreamAdInfo {
    /// The MediaFile resource name of the companion banner used with the ad.
    #[prost(message, optional, tag = "1")]
    pub companion_banner: ::core::option::Option<::prost::alloc::string::String>,
}
/// Representation of video non-skippable in-stream ad format (15 second
/// in-stream non-skippable video ad).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VideoNonSkippableInStreamAdInfo {
    /// The MediaFile resource name of the companion banner used with the ad.
    #[prost(message, optional, tag = "1")]
    pub companion_banner: ::core::option::Option<::prost::alloc::string::String>,
}
/// Representation of video TrueView in-stream ad format (ad shown during video
/// playback, often at beginning, which displays a skip button a few seconds into
/// the video).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VideoTrueViewInStreamAdInfo {
    /// Label on the CTA (call-to-action) button taking the user to the video ad's
    /// final URL.
    /// Required for TrueView for action campaigns, optional otherwise.
    #[prost(message, optional, tag = "1")]
    pub action_button_label: ::core::option::Option<::prost::alloc::string::String>,
    /// Additional text displayed with the CTA (call-to-action) button to give
    /// context and encourage clicking on the button.
    #[prost(message, optional, tag = "2")]
    pub action_headline: ::core::option::Option<::prost::alloc::string::String>,
    /// The MediaFile resource name of the companion banner used with the ad.
    #[prost(message, optional, tag = "3")]
    pub companion_banner: ::core::option::Option<::prost::alloc::string::String>,
}
/// Representation of video out-stream ad format (ad shown alongside a feed
/// with automatic playback, without sound).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VideoOutstreamAdInfo {
    /// The headline of the ad.
    #[prost(message, optional, tag = "1")]
    pub headline: ::core::option::Option<::prost::alloc::string::String>,
    /// The description line.
    #[prost(message, optional, tag = "2")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
}
/// Representation of video TrueView discovery ad format.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VideoTrueViewDiscoveryAdInfo {
    /// The headline of the ad.
    #[prost(message, optional, tag = "1")]
    pub headline: ::core::option::Option<::prost::alloc::string::String>,
    /// First text line for a TrueView video discovery ad.
    #[prost(message, optional, tag = "2")]
    pub description1: ::core::option::Option<::prost::alloc::string::String>,
    /// Second text line for a TrueView video discovery ad.
    #[prost(message, optional, tag = "3")]
    pub description2: ::core::option::Option<::prost::alloc::string::String>,
}
/// A video ad.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VideoAdInfo {
    /// The MediaFile resource to use for the video.
    #[prost(message, optional, tag = "1")]
    pub media_file: ::core::option::Option<::prost::alloc::string::String>,
    /// Format-specific schema for the different video formats.
    #[prost(oneof = "video_ad_info::Format", tags = "2, 3, 4, 5, 6")]
    pub format: ::core::option::Option<video_ad_info::Format>,
}
/// Nested message and enum types in `VideoAdInfo`.
pub mod video_ad_info {
    /// Format-specific schema for the different video formats.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Format {
        /// Video TrueView in-stream ad format.
        #[prost(message, tag = "2")]
        InStream(super::VideoTrueViewInStreamAdInfo),
        /// Video bumper in-stream ad format.
        #[prost(message, tag = "3")]
        Bumper(super::VideoBumperInStreamAdInfo),
        /// Video out-stream ad format.
        #[prost(message, tag = "4")]
        OutStream(super::VideoOutstreamAdInfo),
        /// Video non-skippable in-stream ad format.
        #[prost(message, tag = "5")]
        NonSkippable(super::VideoNonSkippableInStreamAdInfo),
        /// Video TrueView discovery ad format.
        #[prost(message, tag = "6")]
        Discovery(super::VideoTrueViewDiscoveryAdInfo),
    }
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
/// More information at https://support.google.com/google-ads/answer/7684791
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponsiveSearchAdInfo {
    /// List of text assets for headlines. When the ad serves the headlines will
    /// be selected from this list.
    #[prost(message, repeated, tag = "1")]
    pub headlines: ::prost::alloc::vec::Vec<AdTextAsset>,
    /// List of text assets for descriptions. When the ad serves the descriptions
    /// will be selected from this list.
    #[prost(message, repeated, tag = "2")]
    pub descriptions: ::prost::alloc::vec::Vec<AdTextAsset>,
    /// First part of text that may appear appended to the url displayed in the ad.
    #[prost(message, optional, tag = "3")]
    pub path1: ::core::option::Option<::prost::alloc::string::String>,
    /// Second part of text that may appear appended to the url displayed in the
    /// ad. This field can only be set when path1 is also set.
    #[prost(message, optional, tag = "4")]
    pub path2: ::core::option::Option<::prost::alloc::string::String>,
}
/// A legacy responsive display ad. Ads of this type are labeled 'Responsive ads'
/// in the Google Ads UI.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LegacyResponsiveDisplayAdInfo {
    /// The short version of the ad's headline.
    #[prost(message, optional, tag = "1")]
    pub short_headline: ::core::option::Option<::prost::alloc::string::String>,
    /// The long version of the ad's headline.
    #[prost(message, optional, tag = "2")]
    pub long_headline: ::core::option::Option<::prost::alloc::string::String>,
    /// The description of the ad.
    #[prost(message, optional, tag = "3")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
    /// The business name in the ad.
    #[prost(message, optional, tag = "4")]
    pub business_name: ::core::option::Option<::prost::alloc::string::String>,
    /// Advertiser's consent to allow flexible color. When true, the ad may be
    /// served with different color if necessary. When false, the ad will be served
    /// with the specified colors or a neutral color.
    /// The default value is true.
    /// Must be true if main_color and accent_color are not set.
    #[prost(message, optional, tag = "5")]
    pub allow_flexible_color: ::core::option::Option<bool>,
    /// The accent color of the ad in hexadecimal, e.g. #ffffff for white.
    /// If one of main_color and accent_color is set, the other is required as
    /// well.
    #[prost(message, optional, tag = "6")]
    pub accent_color: ::core::option::Option<::prost::alloc::string::String>,
    /// The main color of the ad in hexadecimal, e.g. #ffffff for white.
    /// If one of main_color and accent_color is set, the other is required as
    /// well.
    #[prost(message, optional, tag = "7")]
    pub main_color: ::core::option::Option<::prost::alloc::string::String>,
    /// The call-to-action text for the ad.
    #[prost(message, optional, tag = "8")]
    pub call_to_action_text: ::core::option::Option<::prost::alloc::string::String>,
    /// The MediaFile resource name of the logo image used in the ad.
    #[prost(message, optional, tag = "9")]
    pub logo_image: ::core::option::Option<::prost::alloc::string::String>,
    /// The MediaFile resource name of the square logo image used in the ad.
    #[prost(message, optional, tag = "10")]
    pub square_logo_image: ::core::option::Option<::prost::alloc::string::String>,
    /// The MediaFile resource name of the marketing image used in the ad.
    #[prost(message, optional, tag = "11")]
    pub marketing_image: ::core::option::Option<::prost::alloc::string::String>,
    /// The MediaFile resource name of the square marketing image used in the ad.
    #[prost(message, optional, tag = "12")]
    pub square_marketing_image: ::core::option::Option<::prost::alloc::string::String>,
    /// Specifies which format the ad will be served in. Default is ALL_FORMATS.
    #[prost(
        enumeration = "super::enums::display_ad_format_setting_enum::DisplayAdFormatSetting",
        tag = "13"
    )]
    pub format_setting: i32,
    /// Prefix before price. E.g. 'as low as'.
    #[prost(message, optional, tag = "14")]
    pub price_prefix: ::core::option::Option<::prost::alloc::string::String>,
    /// Promotion text used for dyanmic formats of responsive ads. For example
    /// 'Free two-day shipping'.
    #[prost(message, optional, tag = "15")]
    pub promo_text: ::core::option::Option<::prost::alloc::string::String>,
}
/// An app ad.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AppAdInfo {
    /// Mandatory ad text.
    #[prost(message, optional, tag = "1")]
    pub mandatory_ad_text: ::core::option::Option<AdTextAsset>,
    /// List of text assets for headlines. When the ad serves the headlines will
    /// be selected from this list.
    #[prost(message, repeated, tag = "2")]
    pub headlines: ::prost::alloc::vec::Vec<AdTextAsset>,
    /// List of text assets for descriptions. When the ad serves the descriptions
    /// will be selected from this list.
    #[prost(message, repeated, tag = "3")]
    pub descriptions: ::prost::alloc::vec::Vec<AdTextAsset>,
    /// List of image assets that may be displayed with the ad.
    #[prost(message, repeated, tag = "4")]
    pub images: ::prost::alloc::vec::Vec<AdImageAsset>,
    /// List of YouTube video assets that may be displayed with the ad.
    #[prost(message, repeated, tag = "5")]
    pub youtube_videos: ::prost::alloc::vec::Vec<AdVideoAsset>,
    /// List of media bundle assets that may be used with the ad.
    #[prost(message, repeated, tag = "6")]
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
    #[prost(message, repeated, tag = "1")]
    pub headlines: ::prost::alloc::vec::Vec<AdTextAsset>,
    /// List of text assets for descriptions. When the ad serves the descriptions
    /// will be selected from this list.
    #[prost(message, repeated, tag = "2")]
    pub descriptions: ::prost::alloc::vec::Vec<AdTextAsset>,
    /// List of image assets that may be displayed with the ad.
    #[prost(message, repeated, tag = "3")]
    pub images: ::prost::alloc::vec::Vec<AdImageAsset>,
    /// List of video assets that may be displayed with the ad.
    #[prost(message, repeated, tag = "4")]
    pub videos: ::prost::alloc::vec::Vec<AdVideoAsset>,
}
/// A legacy app install ad that only can be used by a few select customers.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LegacyAppInstallAdInfo {
    /// The id of the mobile app.
    #[prost(message, optional, tag = "1")]
    pub app_id: ::core::option::Option<::prost::alloc::string::String>,
    /// The app store the mobile app is available in.
    #[prost(
        enumeration = "super::enums::legacy_app_install_ad_app_store_enum::LegacyAppInstallAdAppStore",
        tag = "2"
    )]
    pub app_store: i32,
    /// The headline of the ad.
    #[prost(message, optional, tag = "3")]
    pub headline: ::core::option::Option<::prost::alloc::string::String>,
    /// The first description line of the ad.
    #[prost(message, optional, tag = "4")]
    pub description1: ::core::option::Option<::prost::alloc::string::String>,
    /// The second description line of the ad.
    #[prost(message, optional, tag = "5")]
    pub description2: ::core::option::Option<::prost::alloc::string::String>,
}
/// A responsive display ad.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponsiveDisplayAdInfo {
    /// Marketing images to be used in the ad. Valid image types are GIF,
    /// JPEG, and PNG. The minimum size is 600x314 and the aspect ratio must
    /// be 1.91:1 (+-1%). At least one marketing_image is required. Combined with
    /// square_marketing_images the maximum is 15.
    #[prost(message, repeated, tag = "1")]
    pub marketing_images: ::prost::alloc::vec::Vec<AdImageAsset>,
    /// Square marketing images to be used in the ad. Valid image types are GIF,
    /// JPEG, and PNG. The minimum size is 300x300 and the aspect ratio must
    /// be 1:1 (+-1%). At least one square marketing_image is required. Combined
    /// with marketing_images the maximum is 15.
    #[prost(message, repeated, tag = "2")]
    pub square_marketing_images: ::prost::alloc::vec::Vec<AdImageAsset>,
    /// Logo images to be used in the ad. Valid image types are GIF,
    /// JPEG, and PNG. The minimum size is 512x128 and the aspect ratio must
    /// be 4:1 (+-1%). Combined with square_logo_images the maximum is 5.
    #[prost(message, repeated, tag = "3")]
    pub logo_images: ::prost::alloc::vec::Vec<AdImageAsset>,
    /// Square logo images to be used in the ad. Valid image types are GIF,
    /// JPEG, and PNG. The minimum size is 128x128 and the aspect ratio must
    /// be 1:1 (+-1%). Combined with square_logo_images the maximum is 5.
    #[prost(message, repeated, tag = "4")]
    pub square_logo_images: ::prost::alloc::vec::Vec<AdImageAsset>,
    /// Short format headlines for the ad. The maximum length is 30 characters.
    /// At least 1 and max 5 headlines can be specified.
    #[prost(message, repeated, tag = "5")]
    pub headlines: ::prost::alloc::vec::Vec<AdTextAsset>,
    /// A required long format headline. The maximum length is 90 characters.
    #[prost(message, optional, tag = "6")]
    pub long_headline: ::core::option::Option<AdTextAsset>,
    /// Descriptive texts for the ad. The maximum length is 90 characters. At
    /// least 1 and max 5 headlines can be specified.
    #[prost(message, repeated, tag = "7")]
    pub descriptions: ::prost::alloc::vec::Vec<AdTextAsset>,
    /// Optional YouTube videos for the ad. A maximum of 5 videos can be specified.
    #[prost(message, repeated, tag = "8")]
    pub youtube_videos: ::prost::alloc::vec::Vec<AdVideoAsset>,
    /// The advertiser/brand name. Maximum display width is 25.
    #[prost(message, optional, tag = "9")]
    pub business_name: ::core::option::Option<::prost::alloc::string::String>,
    /// The main color of the ad in hexadecimal, e.g. #ffffff for white.
    /// If one of main_color and accent_color is set, the other is required as
    /// well.
    #[prost(message, optional, tag = "10")]
    pub main_color: ::core::option::Option<::prost::alloc::string::String>,
    /// The accent color of the ad in hexadecimal, e.g. #ffffff for white.
    /// If one of main_color and accent_color is set, the other is required as
    /// well.
    #[prost(message, optional, tag = "11")]
    pub accent_color: ::core::option::Option<::prost::alloc::string::String>,
    /// Advertiser's consent to allow flexible color. When true, the ad may be
    /// served with different color if necessary. When false, the ad will be served
    /// with the specified colors or a neutral color.
    /// The default value is true.
    /// Must be true if main_color and accent_color are not set.
    #[prost(message, optional, tag = "12")]
    pub allow_flexible_color: ::core::option::Option<bool>,
    /// The call-to-action text for the ad. Maximum display width is 30.
    #[prost(message, optional, tag = "13")]
    pub call_to_action_text: ::core::option::Option<::prost::alloc::string::String>,
    /// Prefix before price. E.g. 'as low as'.
    #[prost(message, optional, tag = "14")]
    pub price_prefix: ::core::option::Option<::prost::alloc::string::String>,
    /// Promotion text used for dyanmic formats of responsive ads. For example
    /// 'Free two-day shipping'.
    #[prost(message, optional, tag = "15")]
    pub promo_text: ::core::option::Option<::prost::alloc::string::String>,
    /// Specifies which format the ad will be served in. Default is ALL_FORMATS.
    #[prost(
        enumeration = "super::enums::display_ad_format_setting_enum::DisplayAdFormatSetting",
        tag = "16"
    )]
    pub format_setting: i32,
}
/// A local ad.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LocalAdInfo {
    /// List of text assets for headlines. When the ad serves the headlines will
    /// be selected from this list. At least 1 and at most 5 headlines must be
    /// specified.
    #[prost(message, repeated, tag = "1")]
    pub headlines: ::prost::alloc::vec::Vec<AdTextAsset>,
    /// List of text assets for descriptions. When the ad serves the descriptions
    /// will be selected from this list. At least 1 and at most 5 descriptions must
    /// be specified.
    #[prost(message, repeated, tag = "2")]
    pub descriptions: ::prost::alloc::vec::Vec<AdTextAsset>,
    /// List of text assets for call-to-actions. When the ad serves the
    /// call-to-actions will be selected from this list. Call-to-actions are
    /// optional and at most 5 can be specified.
    #[prost(message, repeated, tag = "3")]
    pub call_to_actions: ::prost::alloc::vec::Vec<AdTextAsset>,
    /// List of marketing image assets that may be displayed with the ad. The
    /// images must be 314x600 pixels or 320x320 pixels. At least 1 and at most
    /// 20 image assets must be specified.
    #[prost(message, repeated, tag = "4")]
    pub marketing_images: ::prost::alloc::vec::Vec<AdImageAsset>,
    /// List of logo image assets that may be displayed with the ad. The images
    /// must be 128x128 pixels and not larger than 120KB. At least 1 and at most 5
    /// image assets must be specified.
    #[prost(message, repeated, tag = "5")]
    pub logo_images: ::prost::alloc::vec::Vec<AdImageAsset>,
    /// List of YouTube video assets that may be displayed with the ad. Videos
    /// are optional and at most 20 can be specified.
    #[prost(message, repeated, tag = "6")]
    pub videos: ::prost::alloc::vec::Vec<AdVideoAsset>,
    /// First part of optional text that may appear appended to the url displayed
    /// in the ad.
    #[prost(message, optional, tag = "7")]
    pub path1: ::core::option::Option<::prost::alloc::string::String>,
    /// Second part of optional text that may appear appended to the url displayed
    /// in the ad. This field can only be set when path1 is also set.
    #[prost(message, optional, tag = "8")]
    pub path2: ::core::option::Option<::prost::alloc::string::String>,
}
/// A generic type of display ad. The exact ad format is controlled by the
/// display_upload_product_type field, which determines what kinds of data
/// need to be included with the ad.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DisplayUploadAdInfo {
    /// The product type of this ad. See comments on the enum for details.
    #[prost(
        enumeration = "super::enums::display_upload_product_type_enum::DisplayUploadProductType",
        tag = "1"
    )]
    pub display_upload_product_type: i32,
    /// The asset data that makes up the ad.
    #[prost(oneof = "display_upload_ad_info::MediaAsset", tags = "2")]
    pub media_asset: ::core::option::Option<display_upload_ad_info::MediaAsset>,
}
/// Nested message and enum types in `DisplayUploadAdInfo`.
pub mod display_upload_ad_info {
    /// The asset data that makes up the ad.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum MediaAsset {
        /// A media bundle asset to be used in the ad. For information about the
        /// media bundle for HTML5_UPLOAD_AD see
        /// https://support.google.com/google-ads/answer/1722096
        /// Media bundles that are part of dynamic product types use a special format
        /// that needs to be created through the Google Web Designer. See
        /// https://support.google.com/webdesigner/answer/7543898 for more
        /// information.
        #[prost(message, tag = "2")]
        MediaBundle(super::AdMediaBundleAsset),
    }
}
// Proto file containing info messages for specific asset types.

/// A YouTube asset.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct YoutubeVideoAsset {
    /// YouTube video id. This is the 11 character string value used in the
    /// YouTube video URL.
    #[prost(message, optional, tag = "1")]
    pub youtube_video_id: ::core::option::Option<::prost::alloc::string::String>,
}
/// A MediaBundle asset.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MediaBundleAsset {
    /// Media bundle (ZIP file) asset data. The format of the uploaded ZIP file
    /// depends on the ad field where it will be used. For more information on the
    /// format, see the documentation of the ad field where you plan on using the
    /// MediaBundleAsset. This field is mutate only.
    #[prost(message, optional, tag = "1")]
    pub data: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
/// An Image asset.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImageAsset {
    /// The raw bytes data of an image. This field is mutate only.
    #[prost(message, optional, tag = "1")]
    pub data: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    /// File size of the image asset in bytes.
    #[prost(message, optional, tag = "2")]
    pub file_size: ::core::option::Option<i64>,
    /// MIME type of the image asset.
    #[prost(enumeration = "super::enums::mime_type_enum::MimeType", tag = "3")]
    pub mime_type: i32,
    /// Metadata for this image at its original size.
    #[prost(message, optional, tag = "4")]
    pub full_size: ::core::option::Option<ImageDimension>,
}
/// Metadata for an image at a certain size, either original or resized.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImageDimension {
    /// Height of the image.
    #[prost(message, optional, tag = "1")]
    pub height_pixels: ::core::option::Option<i64>,
    /// Width of the image.
    #[prost(message, optional, tag = "2")]
    pub width_pixels: ::core::option::Option<i64>,
    /// A URL that returns the image with this height and width.
    #[prost(message, optional, tag = "3")]
    pub url: ::core::option::Option<::prost::alloc::string::String>,
}
/// A Text asset.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TextAsset {
    /// Text content of the text asset.
    #[prost(message, optional, tag = "1")]
    pub text: ::core::option::Option<::prost::alloc::string::String>,
}
/// A Book on Google asset. Used to redirect user to book through Google.
/// Book on Google will change the redirect url to book directly through
/// Google.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BookOnGoogleAsset {}
// Proto file describing bidding schemes.

/// Commission is an automatic bidding strategy in which the advertiser pays a
/// certain portion of the conversion value.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Commission {
    /// Commission rate defines the portion of the conversion value that the
    /// advertiser will be billed. A commission rate of x should be passed into
    /// this field as (x * 1,000,000). For example, 106,000 represents a commission
    /// rate of 0.106 (10.6%).
    #[prost(message, optional, tag = "1")]
    pub commission_rate_micros: ::core::option::Option<i64>,
}
/// An automated bidding strategy that raises bids for clicks
/// that seem more likely to lead to a conversion and lowers
/// them for clicks where they seem less likely.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EnhancedCpc {}
/// Manual click-based bidding where user pays per click.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ManualCpc {
    /// Whether bids are to be enhanced based on conversion optimizer data.
    #[prost(message, optional, tag = "1")]
    pub enhanced_cpc_enabled: ::core::option::Option<bool>,
}
/// Manual impression-based bidding where user pays per thousand impressions.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ManualCpm {}
/// View based bidding where user pays per video view.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ManualCpv {}
/// An automated bidding strategy to help get the most conversions for your
/// campaigns while spending your budget.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MaximizeConversions {}
/// An automated bidding strategy to help get the most conversion value for your
/// campaigns while spending your budget.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MaximizeConversionValue {
    /// The target return on ad spend (ROAS) option. If set, the bid strategy will
    /// maximize revenue while averaging the target return on ad spend. If the
    /// target ROAS is high, the bid strategy may not be able to spend the full
    /// budget. If the target ROAS is not set, the bid strategy will aim to
    /// achieve the highest possible ROAS for the budget.
    #[prost(message, optional, tag = "1")]
    pub target_roas: ::core::option::Option<f64>,
}
/// An automated bid strategy that sets bids to help get as many conversions as
/// possible at the target cost-per-acquisition (CPA) you set.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TargetCpa {
    /// Average CPA target.
    /// This target should be greater than or equal to minimum billable unit based
    /// on the currency for the account.
    #[prost(message, optional, tag = "1")]
    pub target_cpa_micros: ::core::option::Option<i64>,
    /// Maximum bid limit that can be set by the bid strategy.
    /// The limit applies to all keywords managed by the strategy.
    #[prost(message, optional, tag = "2")]
    pub cpc_bid_ceiling_micros: ::core::option::Option<i64>,
    /// Minimum bid limit that can be set by the bid strategy.
    /// The limit applies to all keywords managed by the strategy.
    #[prost(message, optional, tag = "3")]
    pub cpc_bid_floor_micros: ::core::option::Option<i64>,
}
/// Target CPM (cost per thousand impressions) is an automated bidding strategy
/// that sets bids to optimize performance given the target CPM you set.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TargetCpm {}
/// An automated bidding strategy that sets bids so that a certain percentage of
/// search ads are shown at the top of the first page (or other targeted
/// location).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TargetImpressionShare {
    /// The targeted location on the search results page.
    #[prost(
        enumeration = "super::enums::target_impression_share_location_enum::TargetImpressionShareLocation",
        tag = "1"
    )]
    pub location: i32,
    /// The desired fraction of ads to be shown in the targeted location in micros.
    /// E.g. 1% equals 10,000.
    #[prost(message, optional, tag = "2")]
    pub location_fraction_micros: ::core::option::Option<i64>,
    /// The highest CPC bid the automated bidding system is permitted to specify.
    /// This is a required field entered by the advertiser that sets the ceiling
    /// and specified in local micros.
    #[prost(message, optional, tag = "3")]
    pub cpc_bid_ceiling_micros: ::core::option::Option<i64>,
}
/// An automated bidding strategy that helps you maximize revenue while
/// averaging a specific target return on ad spend (ROAS).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TargetRoas {
    /// Required. The desired revenue (based on conversion data) per unit of spend.
    /// Value must be between 0.01 and 1000.0, inclusive.
    #[prost(message, optional, tag = "1")]
    pub target_roas: ::core::option::Option<f64>,
    /// Maximum bid limit that can be set by the bid strategy.
    /// The limit applies to all keywords managed by the strategy.
    #[prost(message, optional, tag = "2")]
    pub cpc_bid_ceiling_micros: ::core::option::Option<i64>,
    /// Minimum bid limit that can be set by the bid strategy.
    /// The limit applies to all keywords managed by the strategy.
    #[prost(message, optional, tag = "3")]
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
    /// https://ads-developers.googleblog.com/2020/05/reminder-about-sunset-creation-of.html
    /// for details.
    #[deprecated]
    #[prost(message, optional, tag = "1")]
    pub target_spend_micros: ::core::option::Option<i64>,
    /// Maximum bid limit that can be set by the bid strategy.
    /// The limit applies to all keywords managed by the strategy.
    #[prost(message, optional, tag = "2")]
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
    #[prost(message, optional, tag = "1")]
    pub cpc_bid_ceiling_micros: ::core::option::Option<i64>,
    /// Adjusts the bid for each auction upward or downward, depending on the
    /// likelihood of a conversion. Individual bids may exceed
    /// cpc_bid_ceiling_micros, but the average bid amount for a campaign should
    /// not.
    #[prost(message, optional, tag = "2")]
    pub enhanced_cpc_enabled: ::core::option::Option<bool>,
}
// Proto file describing a ClickLocation.

/// Location criteria associated with a click.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClickLocation {
    /// The city location criterion associated with the impression.
    #[prost(message, optional, tag = "1")]
    pub city: ::core::option::Option<::prost::alloc::string::String>,
    /// The country location criterion associated with the impression.
    #[prost(message, optional, tag = "2")]
    pub country: ::core::option::Option<::prost::alloc::string::String>,
    /// The metro location criterion associated with the impression.
    #[prost(message, optional, tag = "3")]
    pub metro: ::core::option::Option<::prost::alloc::string::String>,
    /// The most specific location criterion associated with the impression.
    #[prost(message, optional, tag = "4")]
    pub most_specific: ::core::option::Option<::prost::alloc::string::String>,
    /// The region location criterion associated with the impression.
    #[prost(message, optional, tag = "5")]
    pub region: ::core::option::Option<::prost::alloc::string::String>,
}
// Proto file describing criteria types.

/// A keyword criterion.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeywordInfo {
    /// The text of the keyword (at most 80 characters and 10 words).
    #[prost(message, optional, tag = "1")]
    pub text: ::core::option::Option<::prost::alloc::string::String>,
    /// The match type of the keyword.
    #[prost(
        enumeration = "super::enums::keyword_match_type_enum::KeywordMatchType",
        tag = "2"
    )]
    pub match_type: i32,
}
/// A placement criterion. This can be used to modify bids for sites when
/// targeting the content network.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlacementInfo {
    /// URL of the placement.
    ///
    /// For example, "http://www.domain.com".
    #[prost(message, optional, tag = "1")]
    pub url: ::core::option::Option<::prost::alloc::string::String>,
}
/// A mobile app category criterion.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MobileAppCategoryInfo {
    /// The mobile app category constant resource name.
    #[prost(message, optional, tag = "1")]
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
    /// Store link is "http://itunes.apple.com/us/app/flood-it!-2/id476943146").
    /// For Android, this native identifier is the application's package name
    /// (e.g., "com.labpixies.colordrips" for "Color Drips" given Google Play link
    /// "https://play.google.com/store/apps/details?id=com.labpixies.colordrips").
    /// A well formed app id for Google Ads API would thus be "1-476943146" for iOS
    /// and "2-com.labpixies.colordrips" for Android.
    /// This field is required and must be set in CREATE operations.
    #[prost(message, optional, tag = "2")]
    pub app_id: ::core::option::Option<::prost::alloc::string::String>,
    /// Name of this mobile application.
    #[prost(message, optional, tag = "3")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
}
/// A location criterion.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LocationInfo {
    /// The geo target constant resource name.
    #[prost(message, optional, tag = "1")]
    pub geo_target_constant: ::core::option::Option<::prost::alloc::string::String>,
}
/// A device criterion.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeviceInfo {
    /// Type of the device.
    #[prost(enumeration = "super::enums::device_enum::Device", tag = "1")]
    pub r#type: i32,
}
/// A preferred content criterion.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PreferredContentInfo {
    /// Type of the preferred content.
    #[prost(
        enumeration = "super::enums::preferred_content_type_enum::PreferredContentType",
        tag = "2"
    )]
    pub r#type: i32,
}
/// A listing group criterion.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListingGroupInfo {
    /// Type of the listing group.
    #[prost(
        enumeration = "super::enums::listing_group_type_enum::ListingGroupType",
        tag = "1"
    )]
    pub r#type: i32,
    /// Dimension value with which this listing group is refining its parent.
    /// Undefined for the root group.
    #[prost(message, optional, tag = "2")]
    pub case_value: ::core::option::Option<ListingDimensionInfo>,
    /// Resource name of ad group criterion which is the parent listing group
    /// subdivision. Null for the root group.
    #[prost(message, optional, tag = "3")]
    pub parent_ad_group_criterion: ::core::option::Option<::prost::alloc::string::String>,
}
/// A listing scope criterion.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListingScopeInfo {
    /// Scope of the campaign criterion.
    #[prost(message, repeated, tag = "2")]
    pub dimensions: ::prost::alloc::vec::Vec<ListingDimensionInfo>,
}
/// Listing dimensions for listing group criterion.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListingDimensionInfo {
    /// Dimension of one of the types below is always present.
    #[prost(
        oneof = "listing_dimension_info::Dimension",
        tags = "2, 3, 4, 5, 6, 13, 15, 8, 9, 10, 16, 11, 12, 14"
    )]
    pub dimension: ::core::option::Option<listing_dimension_info::Dimension>,
}
/// Nested message and enum types in `ListingDimensionInfo`.
pub mod listing_dimension_info {
    /// Dimension of one of the types below is always present.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Dimension {
        /// Advertiser-specific hotel ID.
        #[prost(message, tag = "2")]
        HotelId(super::HotelIdInfo),
        /// Class of the hotel as a number of stars 1 to 5.
        #[prost(message, tag = "3")]
        HotelClass(super::HotelClassInfo),
        /// Country or Region the hotel is located in.
        #[prost(message, tag = "4")]
        HotelCountryRegion(super::HotelCountryRegionInfo),
        /// State the hotel is located in.
        #[prost(message, tag = "5")]
        HotelState(super::HotelStateInfo),
        /// City the hotel is located in.
        #[prost(message, tag = "6")]
        HotelCity(super::HotelCityInfo),
        /// Bidding category of a product offer.
        #[prost(message, tag = "13")]
        ProductBiddingCategory(super::ProductBiddingCategoryInfo),
        /// Brand of a product offer.
        #[prost(message, tag = "15")]
        ProductBrand(super::ProductBrandInfo),
        /// Locality of a product offer.
        #[prost(message, tag = "8")]
        ProductChannel(super::ProductChannelInfo),
        /// Availability of a product offer.
        #[prost(message, tag = "9")]
        ProductChannelExclusivity(super::ProductChannelExclusivityInfo),
        /// Condition of a product offer.
        #[prost(message, tag = "10")]
        ProductCondition(super::ProductConditionInfo),
        /// Custom attribute of a product offer.
        #[prost(message, tag = "16")]
        ProductCustomAttribute(super::ProductCustomAttributeInfo),
        /// Item id of a product offer.
        #[prost(message, tag = "11")]
        ProductItemId(super::ProductItemIdInfo),
        /// Type of a product offer.
        #[prost(message, tag = "12")]
        ProductType(super::ProductTypeInfo),
        /// Unknown dimension. Set when no other listing dimension is set.
        #[prost(message, tag = "14")]
        UnknownListingDimension(super::UnknownListingDimensionInfo),
    }
}
/// Advertiser-specific hotel ID.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HotelIdInfo {
    /// String value of the hotel ID.
    #[prost(message, optional, tag = "1")]
    pub value: ::core::option::Option<::prost::alloc::string::String>,
}
/// Class of the hotel as a number of stars 1 to 5.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HotelClassInfo {
    /// Long value of the hotel class.
    #[prost(message, optional, tag = "1")]
    pub value: ::core::option::Option<i64>,
}
/// Country or Region the hotel is located in.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HotelCountryRegionInfo {
    /// The Geo Target Constant resource name.
    #[prost(message, optional, tag = "1")]
    pub country_region_criterion: ::core::option::Option<::prost::alloc::string::String>,
}
/// State the hotel is located in.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HotelStateInfo {
    /// The Geo Target Constant resource name.
    #[prost(message, optional, tag = "1")]
    pub state_criterion: ::core::option::Option<::prost::alloc::string::String>,
}
/// City the hotel is located in.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HotelCityInfo {
    /// The Geo Target Constant resource name.
    #[prost(message, optional, tag = "1")]
    pub city_criterion: ::core::option::Option<::prost::alloc::string::String>,
}
/// Bidding category of a product offer.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProductBiddingCategoryInfo {
    /// ID of the product bidding category.
    ///
    /// This ID is equivalent to the google_product_category ID as described in
    /// this article: https://support.google.com/merchants/answer/6324436
    #[prost(message, optional, tag = "1")]
    pub id: ::core::option::Option<i64>,
    /// Two-letter upper-case country code of the product bidding category. It must
    /// match the campaign.shopping_setting.sales_country field.
    #[prost(message, optional, tag = "2")]
    pub country_code: ::core::option::Option<::prost::alloc::string::String>,
    /// Level of the product bidding category.
    #[prost(
        enumeration = "super::enums::product_bidding_category_level_enum::ProductBiddingCategoryLevel",
        tag = "3"
    )]
    pub level: i32,
}
/// Brand of the product.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProductBrandInfo {
    /// String value of the product brand.
    #[prost(message, optional, tag = "1")]
    pub value: ::core::option::Option<::prost::alloc::string::String>,
}
/// Locality of a product offer.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProductChannelInfo {
    /// Value of the locality.
    #[prost(
        enumeration = "super::enums::product_channel_enum::ProductChannel",
        tag = "1"
    )]
    pub channel: i32,
}
/// Availability of a product offer.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProductChannelExclusivityInfo {
    /// Value of the availability.
    #[prost(
        enumeration = "super::enums::product_channel_exclusivity_enum::ProductChannelExclusivity",
        tag = "1"
    )]
    pub channel_exclusivity: i32,
}
/// Condition of a product offer.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProductConditionInfo {
    /// Value of the condition.
    #[prost(
        enumeration = "super::enums::product_condition_enum::ProductCondition",
        tag = "1"
    )]
    pub condition: i32,
}
/// Custom attribute of a product offer.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProductCustomAttributeInfo {
    /// String value of the product custom attribute.
    #[prost(message, optional, tag = "1")]
    pub value: ::core::option::Option<::prost::alloc::string::String>,
    /// Indicates the index of the custom attribute.
    #[prost(
        enumeration = "super::enums::product_custom_attribute_index_enum::ProductCustomAttributeIndex",
        tag = "2"
    )]
    pub index: i32,
}
/// Item id of a product offer.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProductItemIdInfo {
    /// Value of the id.
    #[prost(message, optional, tag = "1")]
    pub value: ::core::option::Option<::prost::alloc::string::String>,
}
/// Type of a product offer.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProductTypeInfo {
    /// Value of the type.
    #[prost(message, optional, tag = "1")]
    pub value: ::core::option::Option<::prost::alloc::string::String>,
    /// Level of the type.
    #[prost(
        enumeration = "super::enums::product_type_level_enum::ProductTypeLevel",
        tag = "2"
    )]
    pub level: i32,
}
/// Unknown listing dimension.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnknownListingDimensionInfo {}
/// Criterion for hotel date selection (default dates vs. user selected).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HotelDateSelectionTypeInfo {
    /// Type of the hotel date selection
    #[prost(
        enumeration = "super::enums::hotel_date_selection_type_enum::HotelDateSelectionType",
        tag = "1"
    )]
    pub r#type: i32,
}
/// Criterion for number of days prior to the stay the booking is being made.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HotelAdvanceBookingWindowInfo {
    /// Low end of the number of days prior to the stay.
    #[prost(message, optional, tag = "1")]
    pub min_days: ::core::option::Option<i64>,
    /// High end of the number of days prior to the stay.
    #[prost(message, optional, tag = "2")]
    pub max_days: ::core::option::Option<i64>,
}
/// Criterion for length of hotel stay in nights.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HotelLengthOfStayInfo {
    /// Low end of the number of nights in the stay.
    #[prost(message, optional, tag = "1")]
    pub min_nights: ::core::option::Option<i64>,
    /// High end of the number of nights in the stay.
    #[prost(message, optional, tag = "2")]
    pub max_nights: ::core::option::Option<i64>,
}
/// Criterion for day of the week the booking is for.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HotelCheckInDayInfo {
    /// The day of the week.
    #[prost(enumeration = "super::enums::day_of_week_enum::DayOfWeek", tag = "1")]
    pub day_of_week: i32,
}
/// Criterion for Interaction Type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InteractionTypeInfo {
    /// The interaction type.
    #[prost(
        enumeration = "super::enums::interaction_type_enum::InteractionType",
        tag = "1"
    )]
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
    #[prost(
        enumeration = "super::enums::minute_of_hour_enum::MinuteOfHour",
        tag = "1"
    )]
    pub start_minute: i32,
    /// Minutes after the end hour at which this schedule ends. The schedule is
    /// exclusive of the end minute.
    ///
    /// This field is required for CREATE operations and is prohibited on UPDATE
    /// operations.
    #[prost(
        enumeration = "super::enums::minute_of_hour_enum::MinuteOfHour",
        tag = "2"
    )]
    pub end_minute: i32,
    /// Starting hour in 24 hour time.
    /// This field must be between 0 and 23, inclusive.
    ///
    /// This field is required for CREATE operations and is prohibited on UPDATE
    /// operations.
    #[prost(message, optional, tag = "3")]
    pub start_hour: ::core::option::Option<i32>,
    /// Ending hour in 24 hour time; 24 signifies end of the day.
    /// This field must be between 0 and 24, inclusive.
    ///
    /// This field is required for CREATE operations and is prohibited on UPDATE
    /// operations.
    #[prost(message, optional, tag = "4")]
    pub end_hour: ::core::option::Option<i32>,
    /// Day of the week the schedule applies to.
    ///
    /// This field is required for CREATE operations and is prohibited on UPDATE
    /// operations.
    #[prost(enumeration = "super::enums::day_of_week_enum::DayOfWeek", tag = "5")]
    pub day_of_week: i32,
}
/// An age range criterion.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AgeRangeInfo {
    /// Type of the age range.
    #[prost(
        enumeration = "super::enums::age_range_type_enum::AgeRangeType",
        tag = "1"
    )]
    pub r#type: i32,
}
/// A gender criterion.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenderInfo {
    /// Type of the gender.
    #[prost(enumeration = "super::enums::gender_type_enum::GenderType", tag = "1")]
    pub r#type: i32,
}
/// An income range criterion.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IncomeRangeInfo {
    /// Type of the income range.
    #[prost(
        enumeration = "super::enums::income_range_type_enum::IncomeRangeType",
        tag = "1"
    )]
    pub r#type: i32,
}
/// A parental status criterion.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ParentalStatusInfo {
    /// Type of the parental status.
    #[prost(
        enumeration = "super::enums::parental_status_type_enum::ParentalStatusType",
        tag = "1"
    )]
    pub r#type: i32,
}
/// A YouTube Video criterion.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct YouTubeVideoInfo {
    /// YouTube video id as it appears on the YouTube watch page.
    #[prost(message, optional, tag = "1")]
    pub video_id: ::core::option::Option<::prost::alloc::string::String>,
}
/// A YouTube Channel criterion.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct YouTubeChannelInfo {
    /// The YouTube uploader channel id or the channel code of a YouTube channel.
    #[prost(message, optional, tag = "1")]
    pub channel_id: ::core::option::Option<::prost::alloc::string::String>,
}
/// A User List criterion. Represents a user list that is defined by the
/// advertiser to be targeted.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserListInfo {
    /// The User List resource name.
    #[prost(message, optional, tag = "1")]
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
    #[prost(message, optional, tag = "1")]
    pub geo_point: ::core::option::Option<GeoPointInfo>,
    /// The radius of the proximity.
    #[prost(message, optional, tag = "2")]
    pub radius: ::core::option::Option<f64>,
    /// The unit of measurement of the radius. Default is KILOMETERS.
    #[prost(
        enumeration = "super::enums::proximity_radius_units_enum::ProximityRadiusUnits",
        tag = "3"
    )]
    pub radius_units: i32,
    /// Full address.
    #[prost(message, optional, tag = "4")]
    pub address: ::core::option::Option<AddressInfo>,
}
/// Geo point for proximity criterion.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GeoPointInfo {
    /// Micro degrees for the longitude.
    #[prost(message, optional, tag = "1")]
    pub longitude_in_micro_degrees: ::core::option::Option<i32>,
    /// Micro degrees for the latitude.
    #[prost(message, optional, tag = "2")]
    pub latitude_in_micro_degrees: ::core::option::Option<i32>,
}
/// Address for proximity criterion.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddressInfo {
    /// Postal code.
    #[prost(message, optional, tag = "1")]
    pub postal_code: ::core::option::Option<::prost::alloc::string::String>,
    /// Province or state code.
    #[prost(message, optional, tag = "2")]
    pub province_code: ::core::option::Option<::prost::alloc::string::String>,
    /// Country code.
    #[prost(message, optional, tag = "3")]
    pub country_code: ::core::option::Option<::prost::alloc::string::String>,
    /// Province or state name.
    #[prost(message, optional, tag = "4")]
    pub province_name: ::core::option::Option<::prost::alloc::string::String>,
    /// Street address line 1.
    #[prost(message, optional, tag = "5")]
    pub street_address: ::core::option::Option<::prost::alloc::string::String>,
    /// Street address line 2. This field is write-only. It is only used for
    /// calculating the longitude and latitude of an address when geo_point is
    /// empty.
    #[prost(message, optional, tag = "6")]
    pub street_address2: ::core::option::Option<::prost::alloc::string::String>,
    /// Name of the city.
    #[prost(message, optional, tag = "7")]
    pub city_name: ::core::option::Option<::prost::alloc::string::String>,
}
/// A topic criterion. Use topics to target or exclude placements in the
/// Google Display Network based on the category into which the placement falls
/// (for example, "Pets & Animals/Pets/Dogs").
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TopicInfo {
    /// The Topic Constant resource name.
    #[prost(message, optional, tag = "1")]
    pub topic_constant: ::core::option::Option<::prost::alloc::string::String>,
    /// The category to target or exclude. Each subsequent element in the array
    /// describes a more specific sub-category. For example,
    /// "Pets & Animals", "Pets", "Dogs" represents the "Pets & Animals/Pets/Dogs"
    /// category.
    #[prost(message, repeated, tag = "2")]
    pub path: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// A language criterion.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LanguageInfo {
    /// The language constant resource name.
    #[prost(message, optional, tag = "1")]
    pub language_constant: ::core::option::Option<::prost::alloc::string::String>,
}
/// An IpBlock criterion used for IP exclusions. We allow:
///  - IPv4 and IPv6 addresses
///  - individual addresses (192.168.0.1)
///  - masks for individual addresses (192.168.0.1/32)
///  - masks for Class C networks (192.168.0.1/24)
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IpBlockInfo {
    /// The IP address of this IP block.
    #[prost(message, optional, tag = "1")]
    pub ip_address: ::core::option::Option<::prost::alloc::string::String>,
}
/// Content Label for category exclusion.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContentLabelInfo {
    /// Content label type, required for CREATE operations.
    #[prost(
        enumeration = "super::enums::content_label_type_enum::ContentLabelType",
        tag = "1"
    )]
    pub r#type: i32,
}
/// Represents a Carrier Criterion.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CarrierInfo {
    /// The Carrier constant resource name.
    #[prost(message, optional, tag = "1")]
    pub carrier_constant: ::core::option::Option<::prost::alloc::string::String>,
}
/// Represents a particular interest-based topic to be targeted.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserInterestInfo {
    /// The UserInterest resource name.
    #[prost(message, optional, tag = "1")]
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
    #[prost(message, optional, tag = "1")]
    pub criterion_name: ::core::option::Option<::prost::alloc::string::String>,
    /// Conditions, or logical expressions, for webpage targeting. The list of
    /// webpage targeting conditions are and-ed together when evaluated
    /// for targeting.
    ///
    /// This field is required for CREATE operations and is prohibited on UPDATE
    /// operations.
    #[prost(message, repeated, tag = "2")]
    pub conditions: ::prost::alloc::vec::Vec<WebpageConditionInfo>,
}
/// Logical expression for targeting webpages of an advertiser's website.
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
    #[prost(message, optional, tag = "3")]
    pub argument: ::core::option::Option<::prost::alloc::string::String>,
}
/// Represents an operating system version to be targeted.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperatingSystemVersionInfo {
    /// The operating system version constant resource name.
    #[prost(message, optional, tag = "1")]
    pub operating_system_version_constant: ::core::option::Option<::prost::alloc::string::String>,
}
/// An app payment model criterion.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AppPaymentModelInfo {
    /// Type of the app payment model.
    #[prost(
        enumeration = "super::enums::app_payment_model_type_enum::AppPaymentModelType",
        tag = "1"
    )]
    pub r#type: i32,
}
/// A mobile device criterion.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MobileDeviceInfo {
    /// The mobile device constant resource name.
    #[prost(message, optional, tag = "1")]
    pub mobile_device_constant: ::core::option::Option<::prost::alloc::string::String>,
}
/// A custom affinity criterion.
/// A criterion of this type is only targetable.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomAffinityInfo {
    /// The CustomInterest resource name.
    #[prost(message, optional, tag = "1")]
    pub custom_affinity: ::core::option::Option<::prost::alloc::string::String>,
}
/// A custom intent criterion.
/// A criterion of this type is only targetable.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomIntentInfo {
    /// The CustomInterest resource name.
    #[prost(message, optional, tag = "1")]
    pub custom_intent: ::core::option::Option<::prost::alloc::string::String>,
}
/// A radius around a list of locations specified via a feed.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LocationGroupInfo {
    /// Feed specifying locations for targeting.
    /// This is required and must be set in CREATE operations.
    #[prost(message, optional, tag = "1")]
    pub feed: ::core::option::Option<::prost::alloc::string::String>,
    /// Geo target constant(s) restricting the scope of the geographic area within
    /// the feed. Currently only one geo target constant is allowed.
    #[prost(message, repeated, tag = "2")]
    pub geo_target_constants: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Distance in units specifying the radius around targeted locations.
    /// This is required and must be set in CREATE operations.
    #[prost(message, optional, tag = "3")]
    pub radius: ::core::option::Option<i64>,
    /// Unit of the radius. Miles and meters are supported for geo target
    /// constants. Milli miles and meters are supported for feed item sets.
    /// This is required and must be set in CREATE operations.
    #[prost(
        enumeration = "super::enums::location_group_radius_units_enum::LocationGroupRadiusUnits",
        tag = "4"
    )]
    pub radius_units: i32,
}
// Proto file describing criterion category availability information.

/// Information of category availability, per advertising channel.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CriterionCategoryAvailability {
    /// Channel types and subtypes that are available to the category.
    #[prost(message, optional, tag = "1")]
    pub channel: ::core::option::Option<CriterionCategoryChannelAvailability>,
    /// Locales that are available to the category for the channel.
    #[prost(message, repeated, tag = "2")]
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
    #[prost(
        enumeration = "super::enums::criterion_category_channel_availability_mode_enum::CriterionCategoryChannelAvailabilityMode",
        tag = "1"
    )]
    pub availability_mode: i32,
    /// Channel type the category is available to.
    #[prost(
        enumeration = "super::enums::advertising_channel_type_enum::AdvertisingChannelType",
        tag = "2"
    )]
    pub advertising_channel_type: i32,
    /// Channel subtypes under the channel type the category is available to.
    #[prost(
        enumeration = "super::enums::advertising_channel_sub_type_enum::AdvertisingChannelSubType",
        repeated,
        tag = "3"
    )]
    pub advertising_channel_sub_type: ::prost::alloc::vec::Vec<i32>,
    /// Whether default channel sub type is included. For example,
    /// advertising_channel_type being DISPLAY and include_default_channel_sub_type
    /// being false means that the default display campaign where channel sub type
    /// is not set is not included in this availability configuration.
    #[prost(message, optional, tag = "4")]
    pub include_default_channel_sub_type: ::core::option::Option<bool>,
}
/// Information about which locales a category is available in.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CriterionCategoryLocaleAvailability {
    /// Format of the locale availability. Can be LAUNCHED_TO_ALL (both country and
    /// language will be empty), COUNTRY (only country will be set), LANGUAGE (only
    /// language wil be set), COUNTRY_AND_LANGUAGE (both country and language will
    /// be set).
    #[prost(
        enumeration = "super::enums::criterion_category_locale_availability_mode_enum::CriterionCategoryLocaleAvailabilityMode",
        tag = "1"
    )]
    pub availability_mode: i32,
    /// Code of the country.
    #[prost(message, optional, tag = "2")]
    pub country_code: ::core::option::Option<::prost::alloc::string::String>,
    /// Code of the language.
    #[prost(message, optional, tag = "3")]
    pub language_code: ::core::option::Option<::prost::alloc::string::String>,
}
// Proto file describing CustomParameter and operation

/// A mapping that can be used by custom parameter tags in a
/// `tracking_url_template`, `final_urls`, or `mobile_final_urls`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomParameter {
    /// The key matching the parameter tag name.
    #[prost(message, optional, tag = "1")]
    pub key: ::core::option::Option<::prost::alloc::string::String>,
    /// The value to be substituted.
    #[prost(message, optional, tag = "2")]
    pub value: ::core::option::Option<::prost::alloc::string::String>,
}
// Proto file describing date range message.

/// A date range.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DateRange {
    /// The start date, in yyyy-mm-dd format. This date is inclusive.
    #[prost(message, optional, tag = "1")]
    pub start_date: ::core::option::Option<::prost::alloc::string::String>,
    /// The end date, in yyyy-mm-dd format. This date is inclusive.
    #[prost(message, optional, tag = "2")]
    pub end_date: ::core::option::Option<::prost::alloc::string::String>,
}
// Proto file describing ExplorerAutoOptimizerSetting

/// Settings for the Display Campaign Optimizer, initially named "Explorer".
/// Learn more about
/// [automatic targeting](https://support.google.com/google-ads/answer/190596).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExplorerAutoOptimizerSetting {
    /// Indicates whether the optimizer is turned on.
    #[prost(message, optional, tag = "1")]
    pub opt_in: ::core::option::Option<bool>,
}
// Proto file describing common feed proto messages.

/// Represents a price in a particular currency.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Money {
    /// Three-character ISO 4217 currency code.
    #[prost(message, optional, tag = "1")]
    pub currency_code: ::core::option::Option<::prost::alloc::string::String>,
    /// Amount in micros. One million is equivalent to one unit.
    #[prost(message, optional, tag = "2")]
    pub amount_micros: ::core::option::Option<i64>,
}
// Proto file describing extension types.

/// Represents an App extension.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AppFeedItem {
    /// The visible text displayed when the link is rendered in an ad.
    /// This string must not be empty, and the length of this string should
    /// be between 1 and 25, inclusive.
    #[prost(message, optional, tag = "1")]
    pub link_text: ::core::option::Option<::prost::alloc::string::String>,
    /// The store-specific ID for the target application.
    /// This string must not be empty.
    #[prost(message, optional, tag = "2")]
    pub app_id: ::core::option::Option<::prost::alloc::string::String>,
    /// The application store that the target application belongs to.
    /// This field is required.
    #[prost(enumeration = "super::enums::app_store_enum::AppStore", tag = "3")]
    pub app_store: i32,
    /// A list of possible final URLs after all cross domain redirects.
    /// This list must not be empty.
    #[prost(message, repeated, tag = "4")]
    pub final_urls: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// A list of possible final mobile URLs after all cross domain redirects.
    #[prost(message, repeated, tag = "5")]
    pub final_mobile_urls: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// URL template for constructing a tracking URL. Default value is "{lpurl}".
    #[prost(message, optional, tag = "6")]
    pub tracking_url_template: ::core::option::Option<::prost::alloc::string::String>,
    /// A list of mappings to be used for substituting URL custom parameter tags in
    /// the tracking_url_template, final_urls, and/or final_mobile_urls.
    #[prost(message, repeated, tag = "7")]
    pub url_custom_parameters: ::prost::alloc::vec::Vec<CustomParameter>,
    /// URL template for appending params to landing page URLs served with parallel
    /// tracking.
    #[prost(message, optional, tag = "8")]
    pub final_url_suffix: ::core::option::Option<::prost::alloc::string::String>,
}
/// Represents a Call extension.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CallFeedItem {
    /// The advertiser's phone number to append to the ad.
    /// This string must not be empty.
    #[prost(message, optional, tag = "1")]
    pub phone_number: ::core::option::Option<::prost::alloc::string::String>,
    /// Uppercase two-letter country code of the advertiser's phone number.
    /// This string must not be empty.
    #[prost(message, optional, tag = "2")]
    pub country_code: ::core::option::Option<::prost::alloc::string::String>,
    /// Indicates whether call tracking is enabled. By default, call tracking is
    /// not enabled.
    #[prost(message, optional, tag = "3")]
    pub call_tracking_enabled: ::core::option::Option<bool>,
    /// The conversion action to attribute a call conversion to. If not set a
    /// default conversion action is used. This field only has effect if
    /// call_tracking_enabled is set to true. Otherwise this field is ignored.
    #[prost(message, optional, tag = "4")]
    pub call_conversion_action: ::core::option::Option<::prost::alloc::string::String>,
    /// If true, disable call conversion tracking. call_conversion_action should
    /// not be set if this is true. Optional.
    #[prost(message, optional, tag = "5")]
    pub call_conversion_tracking_disabled: ::core::option::Option<bool>,
    /// Enum value that indicates whether this call extension uses its own call
    /// conversion setting (or just have call conversion disabled), or following
    /// the account level setting.
    #[prost(
        enumeration = "super::enums::call_conversion_reporting_state_enum::CallConversionReportingState",
        tag = "6"
    )]
    pub call_conversion_reporting_state: i32,
}
/// Represents a callout extension.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CalloutFeedItem {
    /// The callout text.
    /// The length of this string should be between 1 and 25, inclusive.
    #[prost(message, optional, tag = "1")]
    pub callout_text: ::core::option::Option<::prost::alloc::string::String>,
}
/// Represents a location extension.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LocationFeedItem {
    /// The name of the business.
    #[prost(message, optional, tag = "1")]
    pub business_name: ::core::option::Option<::prost::alloc::string::String>,
    /// Line 1 of the business address.
    #[prost(message, optional, tag = "2")]
    pub address_line_1: ::core::option::Option<::prost::alloc::string::String>,
    /// Line 2 of the business address.
    #[prost(message, optional, tag = "3")]
    pub address_line_2: ::core::option::Option<::prost::alloc::string::String>,
    /// City of the business address.
    #[prost(message, optional, tag = "4")]
    pub city: ::core::option::Option<::prost::alloc::string::String>,
    /// Province of the business address.
    #[prost(message, optional, tag = "5")]
    pub province: ::core::option::Option<::prost::alloc::string::String>,
    /// Postal code of the business address.
    #[prost(message, optional, tag = "6")]
    pub postal_code: ::core::option::Option<::prost::alloc::string::String>,
    /// Country code of the business address.
    #[prost(message, optional, tag = "7")]
    pub country_code: ::core::option::Option<::prost::alloc::string::String>,
    /// Phone number of the business.
    #[prost(message, optional, tag = "8")]
    pub phone_number: ::core::option::Option<::prost::alloc::string::String>,
}
/// Represents an affiliate location extension.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AffiliateLocationFeedItem {
    /// The name of the business.
    #[prost(message, optional, tag = "1")]
    pub business_name: ::core::option::Option<::prost::alloc::string::String>,
    /// Line 1 of the business address.
    #[prost(message, optional, tag = "2")]
    pub address_line_1: ::core::option::Option<::prost::alloc::string::String>,
    /// Line 2 of the business address.
    #[prost(message, optional, tag = "3")]
    pub address_line_2: ::core::option::Option<::prost::alloc::string::String>,
    /// City of the business address.
    #[prost(message, optional, tag = "4")]
    pub city: ::core::option::Option<::prost::alloc::string::String>,
    /// Province of the business address.
    #[prost(message, optional, tag = "5")]
    pub province: ::core::option::Option<::prost::alloc::string::String>,
    /// Postal code of the business address.
    #[prost(message, optional, tag = "6")]
    pub postal_code: ::core::option::Option<::prost::alloc::string::String>,
    /// Country code of the business address.
    #[prost(message, optional, tag = "7")]
    pub country_code: ::core::option::Option<::prost::alloc::string::String>,
    /// Phone number of the business.
    #[prost(message, optional, tag = "8")]
    pub phone_number: ::core::option::Option<::prost::alloc::string::String>,
    /// Id of the retail chain that is advertised as a seller of your product.
    #[prost(message, optional, tag = "9")]
    pub chain_id: ::core::option::Option<i64>,
    /// Name of chain.
    #[prost(message, optional, tag = "10")]
    pub chain_name: ::core::option::Option<::prost::alloc::string::String>,
}
/// An extension that users can click on to send a text message to the
/// advertiser.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TextMessageFeedItem {
    /// The business name to prepend to the message text.
    /// This field is required.
    #[prost(message, optional, tag = "1")]
    pub business_name: ::core::option::Option<::prost::alloc::string::String>,
    /// Uppercase two-letter country code of the advertiser's phone number.
    /// This field is required.
    #[prost(message, optional, tag = "2")]
    pub country_code: ::core::option::Option<::prost::alloc::string::String>,
    /// The advertiser's phone number the message will be sent to. Required.
    #[prost(message, optional, tag = "3")]
    pub phone_number: ::core::option::Option<::prost::alloc::string::String>,
    /// The text to show in the ad.
    /// This field is required.
    #[prost(message, optional, tag = "4")]
    pub text: ::core::option::Option<::prost::alloc::string::String>,
    /// The message text populated in the messaging app.
    #[prost(message, optional, tag = "5")]
    pub extension_text: ::core::option::Option<::prost::alloc::string::String>,
}
/// Represents a Price extension.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PriceFeedItem {
    /// Price extension type of this extension.
    #[prost(
        enumeration = "super::enums::price_extension_type_enum::PriceExtensionType",
        tag = "1"
    )]
    pub r#type: i32,
    /// Price qualifier for all offers of this price extension.
    #[prost(
        enumeration = "super::enums::price_extension_price_qualifier_enum::PriceExtensionPriceQualifier",
        tag = "2"
    )]
    pub price_qualifier: i32,
    /// Tracking URL template for all offers of this price extension.
    #[prost(message, optional, tag = "3")]
    pub tracking_url_template: ::core::option::Option<::prost::alloc::string::String>,
    /// The code of the language used for this price extension.
    #[prost(message, optional, tag = "4")]
    pub language_code: ::core::option::Option<::prost::alloc::string::String>,
    /// The price offerings in this price extension.
    #[prost(message, repeated, tag = "5")]
    pub price_offerings: ::prost::alloc::vec::Vec<PriceOffer>,
    /// URL template for appending params to landing page URLs served with parallel
    /// tracking.
    #[prost(message, optional, tag = "6")]
    pub final_url_suffix: ::core::option::Option<::prost::alloc::string::String>,
}
/// Represents one price offer in a price extension.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PriceOffer {
    /// Header text of this offer.
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<::prost::alloc::string::String>,
    /// Description text of this offer.
    #[prost(message, optional, tag = "2")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
    /// Price value of this offer.
    #[prost(message, optional, tag = "3")]
    pub price: ::core::option::Option<Money>,
    /// Price unit for this offer.
    #[prost(
        enumeration = "super::enums::price_extension_price_unit_enum::PriceExtensionPriceUnit",
        tag = "4"
    )]
    pub unit: i32,
    /// A list of possible final URLs after all cross domain redirects.
    #[prost(message, repeated, tag = "5")]
    pub final_urls: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// A list of possible final mobile URLs after all cross domain redirects.
    #[prost(message, repeated, tag = "6")]
    pub final_mobile_urls: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Represents a Promotion extension.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PromotionFeedItem {
    /// A freeform description of what the promotion is targeting.
    /// This field is required.
    #[prost(message, optional, tag = "1")]
    pub promotion_target: ::core::option::Option<::prost::alloc::string::String>,
    /// Enum that modifies the qualification of the discount.
    #[prost(
        enumeration = "super::enums::promotion_extension_discount_modifier_enum::PromotionExtensionDiscountModifier",
        tag = "2"
    )]
    pub discount_modifier: i32,
    /// Start date of when the promotion is eligible to be redeemed.
    #[prost(message, optional, tag = "7")]
    pub promotion_start_date: ::core::option::Option<::prost::alloc::string::String>,
    /// End date of when the promotion is eligible to be redeemed.
    #[prost(message, optional, tag = "8")]
    pub promotion_end_date: ::core::option::Option<::prost::alloc::string::String>,
    /// The occasion the promotion was intended for.
    /// If an occasion is set, the redemption window will need to fall within
    /// the date range associated with the occasion.
    #[prost(
        enumeration = "super::enums::promotion_extension_occasion_enum::PromotionExtensionOccasion",
        tag = "9"
    )]
    pub occasion: i32,
    /// A list of possible final URLs after all cross domain redirects.
    /// This field is required.
    #[prost(message, repeated, tag = "10")]
    pub final_urls: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// A list of possible final mobile URLs after all cross domain redirects.
    #[prost(message, repeated, tag = "11")]
    pub final_mobile_urls: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// URL template for constructing a tracking URL.
    #[prost(message, optional, tag = "12")]
    pub tracking_url_template: ::core::option::Option<::prost::alloc::string::String>,
    /// A list of mappings to be used for substituting URL custom parameter tags in
    /// the tracking_url_template, final_urls, and/or final_mobile_urls.
    #[prost(message, repeated, tag = "13")]
    pub url_custom_parameters: ::prost::alloc::vec::Vec<CustomParameter>,
    /// URL template for appending params to landing page URLs served with parallel
    /// tracking.
    #[prost(message, optional, tag = "14")]
    pub final_url_suffix: ::core::option::Option<::prost::alloc::string::String>,
    /// The language of the promotion.
    /// Represented as BCP 47 language tag.
    #[prost(message, optional, tag = "15")]
    pub language_code: ::core::option::Option<::prost::alloc::string::String>,
    /// Discount type, can be percentage off or amount off.
    #[prost(oneof = "promotion_feed_item::DiscountType", tags = "3, 4")]
    pub discount_type: ::core::option::Option<promotion_feed_item::DiscountType>,
    /// Promotion trigger. Can be by promotion code or promo by eligible order
    /// amount.
    #[prost(oneof = "promotion_feed_item::PromotionTrigger", tags = "5, 6")]
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
        #[prost(message, tag = "3")]
        PercentOff(i64),
        /// Money amount off for discount in the promotion.
        /// Either this or percent_off is required.
        #[prost(message, tag = "4")]
        MoneyAmountOff(super::Money),
    }
    /// Promotion trigger. Can be by promotion code or promo by eligible order
    /// amount.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum PromotionTrigger {
        /// A code the user should use in order to be eligible for the promotion.
        #[prost(message, tag = "5")]
        PromotionCode(::prost::alloc::string::String),
        /// The amount the total order needs to be for the user to be eligible for
        /// the promotion.
        #[prost(message, tag = "6")]
        OrdersOverAmount(super::Money),
    }
}
/// Represents a structured snippet extension.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StructuredSnippetFeedItem {
    /// The header of the snippet.
    /// This string must not be empty.
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<::prost::alloc::string::String>,
    /// The values in the snippet.
    /// The maximum size of this collection is 10.
    #[prost(message, repeated, tag = "2")]
    pub values: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Represents a sitelink extension.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SitelinkFeedItem {
    /// URL display text for the sitelink.
    /// The length of this string should be between 1 and 25, inclusive.
    #[prost(message, optional, tag = "1")]
    pub link_text: ::core::option::Option<::prost::alloc::string::String>,
    /// First line of the description for the sitelink.
    /// If this value is set, line2 must also be set.
    /// The length of this string should be between 0 and 35, inclusive.
    #[prost(message, optional, tag = "2")]
    pub line1: ::core::option::Option<::prost::alloc::string::String>,
    /// Second line of the description for the sitelink.
    /// If this value is set, line1 must also be set.
    /// The length of this string should be between 0 and 35, inclusive.
    #[prost(message, optional, tag = "3")]
    pub line2: ::core::option::Option<::prost::alloc::string::String>,
    /// A list of possible final URLs after all cross domain redirects.
    #[prost(message, repeated, tag = "4")]
    pub final_urls: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// A list of possible final mobile URLs after all cross domain redirects.
    #[prost(message, repeated, tag = "5")]
    pub final_mobile_urls: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// URL template for constructing a tracking URL.
    #[prost(message, optional, tag = "6")]
    pub tracking_url_template: ::core::option::Option<::prost::alloc::string::String>,
    /// A list of mappings to be used for substituting URL custom parameter tags in
    /// the tracking_url_template, final_urls, and/or final_mobile_urls.
    #[prost(message, repeated, tag = "7")]
    pub url_custom_parameters: ::prost::alloc::vec::Vec<CustomParameter>,
    /// Final URL suffix to be appended to landing page URLs served with
    /// parallel tracking.
    #[prost(message, optional, tag = "8")]
    pub final_url_suffix: ::core::option::Option<::prost::alloc::string::String>,
}
/// Represents a hotel callout extension.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HotelCalloutFeedItem {
    /// The callout text.
    /// The length of this string should be between 1 and 25, inclusive.
    #[prost(message, optional, tag = "1")]
    pub text: ::core::option::Option<::prost::alloc::string::String>,
    /// The language of the hotel callout text.
    /// IETF BCP 47 compliant language code.
    #[prost(message, optional, tag = "2")]
    pub language_code: ::core::option::Option<::prost::alloc::string::String>,
}
// Proto file FinalAppUrl type.

/// A URL for deep linking into an app for the given operating system.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FinalAppUrl {
    /// The operating system targeted by this URL. Required.
    #[prost(
        enumeration = "super::enums::app_url_operating_system_type_enum::AppUrlOperatingSystemType",
        tag = "1"
    )]
    pub os_type: i32,
    /// The app deep link URL. Deep links specify a location in an app that
    /// corresponds to the content you'd like to show, and should be of the form
    /// {scheme}://{host_path}
    /// The scheme identifies which app to open. For your app, you can use a custom
    /// scheme that starts with the app's name. The host and path specify the
    /// unique location in the app where your content exists.
    /// Example: "exampleapp://productid_1234". Required.
    #[prost(message, optional, tag = "2")]
    pub url: ::core::option::Option<::prost::alloc::string::String>,
}
// Proto file describing frequency caps.

/// A rule specifying the maximum number of times an ad (or some set of ads) can
/// be shown to a user over a particular time period.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FrequencyCapEntry {
    /// The key of a particular frequency cap. There can be no more
    /// than one frequency cap with the same key.
    #[prost(message, optional, tag = "1")]
    pub key: ::core::option::Option<FrequencyCapKey>,
    /// Maximum number of events allowed during the time range by this cap.
    #[prost(message, optional, tag = "2")]
    pub cap: ::core::option::Option<i32>,
}
/// A group of fields used as keys for a frequency cap.
/// There can be no more than one frequency cap with the same key.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FrequencyCapKey {
    /// The level on which the cap is to be applied (e.g. ad group ad, ad group).
    /// The cap is applied to all the entities of this level.
    #[prost(
        enumeration = "super::enums::frequency_cap_level_enum::FrequencyCapLevel",
        tag = "1"
    )]
    pub level: i32,
    /// The type of event that the cap applies to (e.g. impression).
    #[prost(
        enumeration = "super::enums::frequency_cap_event_type_enum::FrequencyCapEventType",
        tag = "3"
    )]
    pub event_type: i32,
    /// Unit of time the cap is defined at (e.g. day, week).
    #[prost(
        enumeration = "super::enums::frequency_cap_time_unit_enum::FrequencyCapTimeUnit",
        tag = "2"
    )]
    pub time_unit: i32,
    /// Number of time units the cap lasts.
    #[prost(message, optional, tag = "4")]
    pub time_length: ::core::option::Option<i32>,
}
// Proto file describing Keyword Planner messages.

/// Historical metrics specific to the targeting options selected.
/// Targeting options include geographies, network, etc.
/// Refer to https://support.google.com/google-ads/answer/3022575 for more
/// details.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeywordPlanHistoricalMetrics {
    /// Approximate number of monthly searches on this query averaged
    /// for the past 12 months.
    #[prost(message, optional, tag = "1")]
    pub avg_monthly_searches: ::core::option::Option<i64>,
    /// Approximate number of searches on this query for the past twelve months.
    #[prost(message, repeated, tag = "6")]
    pub monthly_search_volumes: ::prost::alloc::vec::Vec<MonthlySearchVolume>,
    /// The competition level for the query.
    #[prost(
        enumeration = "super::enums::keyword_plan_competition_level_enum::KeywordPlanCompetitionLevel",
        tag = "2"
    )]
    pub competition: i32,
    /// The competition index for the query in the range [0, 100].
    /// Shows how competitive ad placement is for a keyword.
    /// The level of competition from 0-100 is determined by the number of ad slots
    /// filled divided by the total number of ad slots available. If not enough
    /// data is available, null is returned.
    #[prost(message, optional, tag = "3")]
    pub competition_index: ::core::option::Option<i64>,
    /// Top of page bid low range (20th percentile) in micros for the keyword.
    #[prost(message, optional, tag = "4")]
    pub low_top_of_page_bid_micros: ::core::option::Option<i64>,
    /// Top of page bid high range (80th percentile) in micros for the keyword.
    #[prost(message, optional, tag = "5")]
    pub high_top_of_page_bid_micros: ::core::option::Option<i64>,
}
/// Monthly search volume.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MonthlySearchVolume {
    /// The year of the search volume (e.g. 2020).
    #[prost(message, optional, tag = "1")]
    pub year: ::core::option::Option<i64>,
    /// The month of the search volume.
    #[prost(
        enumeration = "super::enums::month_of_year_enum::MonthOfYear",
        tag = "2"
    )]
    pub month: i32,
    /// Approximate number of searches for the month.
    /// A null value indicates the search volume is unavailable for
    /// that month.
    #[prost(message, optional, tag = "3")]
    pub monthly_searches: ::core::option::Option<i64>,
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
    /// 4. CONTAINS_ANY(FeedAttribute[12345678,0],{"Mars cruise","Venus cruise"})
    /// 5. AND(IN(FEED_ITEM_ID,{10001,10002}),EQUALS(CONTEXT.DEVICE,"Mobile"))
    ///
    /// For more details, visit
    /// https://developers.google.com/adwords/api/docs/guides/feed-matching-functions
    ///
    /// Note that because multiple strings may represent the same underlying
    /// function (whitespace and single versus double quotation marks, for
    /// example), the value returned may not be identical to the string sent in a
    /// mutate request.
    #[prost(message, optional, tag = "1")]
    pub function_string: ::core::option::Option<::prost::alloc::string::String>,
    /// Operator for a function.
    #[prost(
        enumeration = "super::enums::matching_function_operator_enum::MatchingFunctionOperator",
        tag = "4"
    )]
    pub operator: i32,
    /// The operands on the left hand side of the equation. This is also the
    /// operand to be used for single operand expressions such as NOT.
    #[prost(message, repeated, tag = "2")]
    pub left_operands: ::prost::alloc::vec::Vec<Operand>,
    /// The operands on the right hand side of the equation.
    #[prost(message, repeated, tag = "3")]
    pub right_operands: ::prost::alloc::vec::Vec<Operand>,
}
/// An operand in a matching function.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Operand {
    /// Different operands that can be used in a matching function. Required.
    #[prost(oneof = "operand::FunctionArgumentOperand", tags = "1, 2, 3, 4")]
    pub function_argument_operand: ::core::option::Option<operand::FunctionArgumentOperand>,
}
/// Nested message and enum types in `Operand`.
pub mod operand {
    /// A constant operand in a matching function.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ConstantOperand {
        /// Constant operand values. Required.
        #[prost(oneof = "constant_operand::ConstantOperandValue", tags = "1, 2, 3, 4")]
        pub constant_operand_value: ::core::option::Option<constant_operand::ConstantOperandValue>,
    }
    /// Nested message and enum types in `ConstantOperand`.
    pub mod constant_operand {
        /// Constant operand values. Required.
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum ConstantOperandValue {
            /// String value of the operand if it is a string type.
            #[prost(message, tag = "1")]
            StringValue(::prost::alloc::string::String),
            /// Int64 value of the operand if it is a int64 type.
            #[prost(message, tag = "2")]
            LongValue(i64),
            /// Boolean value of the operand if it is a boolean type.
            #[prost(message, tag = "3")]
            BooleanValue(bool),
            /// Double value of the operand if it is a double type.
            #[prost(message, tag = "4")]
            DoubleValue(f64),
        }
    }
    /// A feed attribute operand in a matching function.
    /// Used to represent a feed attribute in feed.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct FeedAttributeOperand {
        /// The associated feed. Required.
        #[prost(message, optional, tag = "1")]
        pub feed_id: ::core::option::Option<i64>,
        /// Id of the referenced feed attribute. Required.
        #[prost(message, optional, tag = "2")]
        pub feed_attribute_id: ::core::option::Option<i64>,
    }
    /// A function operand in a matching function.
    /// Used to represent nested functions.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct FunctionOperand {
        /// The matching function held in this operand.
        #[prost(message, optional, tag = "1")]
        pub matching_function: ::core::option::Option<super::MatchingFunction>,
    }
    /// An operand in a function referring to a value in the request context.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct RequestContextOperand {
        /// Type of value to be referred in the request context.
        #[prost(
            enumeration = "super::super::enums::matching_function_context_type_enum::MatchingFunctionContextType",
            tag = "1"
        )]
        pub context_type: i32,
    }
    /// Different operands that can be used in a matching function. Required.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum FunctionArgumentOperand {
        /// A constant operand in a matching function.
        #[prost(message, tag = "1")]
        ConstantOperand(ConstantOperand),
        /// This operand specifies a feed attribute in feed.
        #[prost(message, tag = "2")]
        FeedAttributeOperand(FeedAttributeOperand),
        /// A function operand in a matching function.
        /// Used to represent nested functions.
        #[prost(message, tag = "3")]
        FunctionOperand(FunctionOperand),
        /// An operand in a function referring to a value in the request context.
        #[prost(message, tag = "4")]
        RequestContextOperand(RequestContextOperand),
    }
}
// Proto file describing metrics.

/// Metrics data.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Metrics {
    /// The percent of your ad impressions that are shown as the very first ad
    /// above the organic search results.
    #[prost(message, optional, tag = "95")]
    pub absolute_top_impression_percentage: ::core::option::Option<f64>,
    /// Average cost of viewable impressions (`active_view_impressions`).
    #[prost(message, optional, tag = "1")]
    pub active_view_cpm: ::core::option::Option<f64>,
    /// Active view measurable clicks divided by active view viewable impressions.
    /// This metric is reported only for display network.
    #[prost(message, optional, tag = "79")]
    pub active_view_ctr: ::core::option::Option<f64>,
    /// A measurement of how often your ad has become viewable on a Display
    /// Network site.
    #[prost(message, optional, tag = "2")]
    pub active_view_impressions: ::core::option::Option<i64>,
    /// The ratio of impressions that could be measured by Active View over the
    /// number of served impressions.
    #[prost(message, optional, tag = "96")]
    pub active_view_measurability: ::core::option::Option<f64>,
    /// The cost of the impressions you received that were measurable by Active
    /// View.
    #[prost(message, optional, tag = "3")]
    pub active_view_measurable_cost_micros: ::core::option::Option<i64>,
    /// The number of times your ads are appearing on placements in positions
    /// where they can be seen.
    #[prost(message, optional, tag = "4")]
    pub active_view_measurable_impressions: ::core::option::Option<i64>,
    /// The percentage of time when your ad appeared on an Active View enabled site
    /// (measurable impressions) and was viewable (viewable impressions).
    #[prost(message, optional, tag = "97")]
    pub active_view_viewability: ::core::option::Option<f64>,
    /// All conversions from interactions (as oppose to view through conversions)
    /// divided by the number of ad interactions.
    #[prost(message, optional, tag = "65")]
    pub all_conversions_from_interactions_rate: ::core::option::Option<f64>,
    /// The value of all conversions.
    #[prost(message, optional, tag = "66")]
    pub all_conversions_value: ::core::option::Option<f64>,
    /// The total number of conversions. This includes all conversions regardless
    /// of the value of include_in_conversions_metric.
    #[prost(message, optional, tag = "7")]
    pub all_conversions: ::core::option::Option<f64>,
    /// The value of all conversions divided by the total cost of ad interactions
    /// (such as clicks for text ads or views for video ads).
    #[prost(message, optional, tag = "62")]
    pub all_conversions_value_per_cost: ::core::option::Option<f64>,
    /// The number of times people clicked the "Call" button to call a store during
    /// or after clicking an ad. This number doesn't include whether or not calls
    /// were connected, or the duration of any calls.
    /// This metric applies to feed items only.
    #[prost(message, optional, tag = "118")]
    pub all_conversions_from_click_to_call: ::core::option::Option<f64>,
    /// The number of times people clicked a "Get directions" button to navigate to
    /// a store after clicking an ad.
    /// This metric applies to feed items only.
    #[prost(message, optional, tag = "119")]
    pub all_conversions_from_directions: ::core::option::Option<f64>,
    /// The value of all conversions from interactions divided by the total number
    /// of interactions.
    #[prost(message, optional, tag = "67")]
    pub all_conversions_from_interactions_value_per_interaction: ::core::option::Option<f64>,
    /// The number of times people clicked a link to view a store's menu after
    /// clicking an ad.
    /// This metric applies to feed items only.
    #[prost(message, optional, tag = "120")]
    pub all_conversions_from_menu: ::core::option::Option<f64>,
    /// The number of times people placed an order at a store after clicking an ad.
    /// This metric applies to feed items only.
    #[prost(message, optional, tag = "121")]
    pub all_conversions_from_order: ::core::option::Option<f64>,
    /// The number of other conversions (for example, posting a review or saving a
    /// location for a store) that occurred after people clicked an ad.
    /// This metric applies to feed items only.
    #[prost(message, optional, tag = "122")]
    pub all_conversions_from_other_engagement: ::core::option::Option<f64>,
    /// Estimated number of times people visited a store after clicking an ad.
    /// This metric applies to feed items only.
    #[prost(message, optional, tag = "123")]
    pub all_conversions_from_store_visit: ::core::option::Option<f64>,
    /// The number of times that people were taken to a store's URL after clicking
    /// an ad.
    /// This metric applies to feed items only.
    #[prost(message, optional, tag = "124")]
    pub all_conversions_from_store_website: ::core::option::Option<f64>,
    /// The average amount you pay per interaction. This amount is the total cost
    /// of your ads divided by the total number of interactions.
    #[prost(message, optional, tag = "8")]
    pub average_cost: ::core::option::Option<f64>,
    /// The total cost of all clicks divided by the total number of clicks
    /// received.
    #[prost(message, optional, tag = "9")]
    pub average_cpc: ::core::option::Option<f64>,
    /// The average amount that you've been charged for an ad engagement. This
    /// amount is the total cost of all ad engagements divided by the total number
    /// of ad engagements.
    #[prost(message, optional, tag = "98")]
    pub average_cpe: ::core::option::Option<f64>,
    /// Average cost-per-thousand impressions (CPM).
    #[prost(message, optional, tag = "10")]
    pub average_cpm: ::core::option::Option<f64>,
    /// The average amount you pay each time someone views your ad.
    /// The average CPV is defined by the total cost of all ad views divided by
    /// the number of views.
    #[prost(message, optional, tag = "11")]
    pub average_cpv: ::core::option::Option<f64>,
    /// Average number of pages viewed per session.
    #[prost(message, optional, tag = "99")]
    pub average_page_views: ::core::option::Option<f64>,
    /// Total duration of all sessions (in seconds) / number of sessions. Imported
    /// from Google Analytics.
    #[prost(message, optional, tag = "84")]
    pub average_time_on_site: ::core::option::Option<f64>,
    /// An indication of how other advertisers are bidding on similar products.
    #[prost(message, optional, tag = "14")]
    pub benchmark_average_max_cpc: ::core::option::Option<f64>,
    /// An indication on how other advertisers' Shopping ads for similar products
    /// are performing based on how often people who see their ad click on it.
    #[prost(message, optional, tag = "77")]
    pub benchmark_ctr: ::core::option::Option<f64>,
    /// Percentage of clicks where the user only visited a single page on your
    /// site. Imported from Google Analytics.
    #[prost(message, optional, tag = "15")]
    pub bounce_rate: ::core::option::Option<f64>,
    /// The number of clicks.
    #[prost(message, optional, tag = "19")]
    pub clicks: ::core::option::Option<i64>,
    /// The number of times your ad or your site's listing in the unpaid
    /// results was clicked. See the help page at
    /// https://support.google.com/google-ads/answer/3097241 for details.
    #[prost(message, optional, tag = "115")]
    pub combined_clicks: ::core::option::Option<i64>,
    /// The number of times your ad or your site's listing in the unpaid
    /// results was clicked (combined_clicks) divided by combined_queries. See the
    /// help page at https://support.google.com/google-ads/answer/3097241 for
    /// details.
    #[prost(message, optional, tag = "116")]
    pub combined_clicks_per_query: ::core::option::Option<f64>,
    /// The number of searches that returned pages from your site in the unpaid
    /// results or showed one of your text ads. See the help page at
    /// https://support.google.com/google-ads/answer/3097241 for details.
    #[prost(message, optional, tag = "117")]
    pub combined_queries: ::core::option::Option<i64>,
    /// The estimated percent of times that your ad was eligible to show
    /// on the Display Network but didn't because your budget was too low.
    /// Note: Content budget lost impression share is reported in the range of 0
    /// to 0.9. Any value above 0.9 is reported as 0.9001.
    #[prost(message, optional, tag = "20")]
    pub content_budget_lost_impression_share: ::core::option::Option<f64>,
    /// The impressions you've received on the Display Network divided
    /// by the estimated number of impressions you were eligible to receive.
    /// Note: Content impression share is reported in the range of 0.1 to 1. Any
    /// value below 0.1 is reported as 0.0999.
    #[prost(message, optional, tag = "21")]
    pub content_impression_share: ::core::option::Option<f64>,
    /// The last date/time a conversion tag for this conversion action successfully
    /// fired and was seen by Google Ads. This firing event may not have been the
    /// result of an attributable conversion (e.g. because the tag was fired from a
    /// browser that did not previously click an ad from an appropriate
    /// advertiser). The date/time is in the customer's time zone.
    #[prost(message, optional, tag = "73")]
    pub conversion_last_received_request_date_time:
        ::core::option::Option<::prost::alloc::string::String>,
    /// The date of the most recent conversion for this conversion action. The date
    /// is in the customer's time zone.
    #[prost(message, optional, tag = "74")]
    pub conversion_last_conversion_date: ::core::option::Option<::prost::alloc::string::String>,
    /// The estimated percentage of impressions on the Display Network
    /// that your ads didn't receive due to poor Ad Rank.
    /// Note: Content rank lost impression share is reported in the range of 0
    /// to 0.9. Any value above 0.9 is reported as 0.9001.
    #[prost(message, optional, tag = "22")]
    pub content_rank_lost_impression_share: ::core::option::Option<f64>,
    /// Conversions from interactions divided by the number of ad interactions
    /// (such as clicks for text ads or views for video ads). This only includes
    /// conversion actions which include_in_conversions_metric attribute is set to
    /// true. If you use conversion-based bidding, your bid strategies will
    /// optimize for these conversions.
    #[prost(message, optional, tag = "69")]
    pub conversions_from_interactions_rate: ::core::option::Option<f64>,
    /// The value of conversions. This only includes conversion actions which
    /// include_in_conversions_metric attribute is set to true. If you use
    /// conversion-based bidding, your bid strategies will optimize for these
    /// conversions.
    #[prost(message, optional, tag = "70")]
    pub conversions_value: ::core::option::Option<f64>,
    /// The value of conversions divided by the cost of ad interactions. This only
    /// includes conversion actions which include_in_conversions_metric attribute
    /// is set to true. If you use conversion-based bidding, your bid strategies
    /// will optimize for these conversions.
    #[prost(message, optional, tag = "71")]
    pub conversions_value_per_cost: ::core::option::Option<f64>,
    /// The value of conversions from interactions divided by the number of ad
    /// interactions. This only includes conversion actions which
    /// include_in_conversions_metric attribute is set to true. If you use
    /// conversion-based bidding, your bid strategies will optimize for these
    /// conversions.
    #[prost(message, optional, tag = "72")]
    pub conversions_from_interactions_value_per_interaction: ::core::option::Option<f64>,
    /// The number of conversions. This only includes conversion actions which
    /// include_in_conversions_metric attribute is set to true. If you use
    /// conversion-based bidding, your bid strategies will optimize for these
    /// conversions.
    #[prost(message, optional, tag = "25")]
    pub conversions: ::core::option::Option<f64>,
    /// The sum of your cost-per-click (CPC) and cost-per-thousand impressions
    /// (CPM) costs during this period.
    #[prost(message, optional, tag = "26")]
    pub cost_micros: ::core::option::Option<i64>,
    /// The cost of ad interactions divided by all conversions.
    #[prost(message, optional, tag = "68")]
    pub cost_per_all_conversions: ::core::option::Option<f64>,
    /// The cost of ad interactions divided by conversions. This only includes
    /// conversion actions which include_in_conversions_metric attribute is set to
    /// true. If you use conversion-based bidding, your bid strategies will
    /// optimize for these conversions.
    #[prost(message, optional, tag = "28")]
    pub cost_per_conversion: ::core::option::Option<f64>,
    /// The cost of ad interactions divided by current model attributed
    /// conversions. This only includes conversion actions which
    /// include_in_conversions_metric attribute is set to true. If you use
    /// conversion-based bidding, your bid strategies will optimize for these
    /// conversions.
    #[prost(message, optional, tag = "106")]
    pub cost_per_current_model_attributed_conversion: ::core::option::Option<f64>,
    /// Conversions from when a customer clicks on a Google Ads ad on one device,
    /// then converts on a different device or browser.
    /// Cross-device conversions are already included in all_conversions.
    #[prost(message, optional, tag = "29")]
    pub cross_device_conversions: ::core::option::Option<f64>,
    /// The number of clicks your ad receives (Clicks) divided by the number
    /// of times your ad is shown (Impressions).
    #[prost(message, optional, tag = "30")]
    pub ctr: ::core::option::Option<f64>,
    /// Shows how your historic conversions data would look under the attribution
    /// model you've currently selected. This only includes conversion actions
    /// which include_in_conversions_metric attribute is set to true. If you use
    /// conversion-based bidding, your bid strategies will optimize for these
    /// conversions.
    #[prost(message, optional, tag = "101")]
    pub current_model_attributed_conversions: ::core::option::Option<f64>,
    /// Current model attributed conversions from interactions divided by the
    /// number of ad interactions (such as clicks for text ads or views for video
    /// ads). This only includes conversion actions which
    /// include_in_conversions_metric attribute is set to true. If you use
    /// conversion-based bidding, your bid strategies will optimize for these
    /// conversions.
    #[prost(message, optional, tag = "102")]
    pub current_model_attributed_conversions_from_interactions_rate: ::core::option::Option<f64>,
    /// The value of current model attributed conversions from interactions divided
    /// by the number of ad interactions. This only includes conversion actions
    /// which include_in_conversions_metric attribute is set to true. If you use
    /// conversion-based bidding, your bid strategies will optimize for these
    /// conversions.
    #[prost(message, optional, tag = "103")]
    pub current_model_attributed_conversions_from_interactions_value_per_interaction:
        ::core::option::Option<f64>,
    /// The value of current model attributed conversions. This only includes
    /// conversion actions which include_in_conversions_metric attribute is set to
    /// true. If you use conversion-based bidding, your bid strategies will
    /// optimize for these conversions.
    #[prost(message, optional, tag = "104")]
    pub current_model_attributed_conversions_value: ::core::option::Option<f64>,
    /// The value of current model attributed conversions divided by the cost of ad
    /// interactions. This only includes conversion actions which
    /// include_in_conversions_metric attribute is set to true. If you use
    /// conversion-based bidding, your bid strategies will optimize for these
    /// conversions.
    #[prost(message, optional, tag = "105")]
    pub current_model_attributed_conversions_value_per_cost: ::core::option::Option<f64>,
    /// How often people engage with your ad after it's shown to them. This is the
    /// number of ad expansions divided by the number of times your ad is shown.
    #[prost(message, optional, tag = "31")]
    pub engagement_rate: ::core::option::Option<f64>,
    /// The number of engagements.
    /// An engagement occurs when a viewer expands your Lightbox ad. Also, in the
    /// future, other ad types may support engagement metrics.
    #[prost(message, optional, tag = "32")]
    pub engagements: ::core::option::Option<i64>,
    /// Average lead value based on clicks.
    #[prost(message, optional, tag = "75")]
    pub hotel_average_lead_value_micros: ::core::option::Option<f64>,
    /// The average price difference between the price offered by reporting hotel
    /// advertiser and the cheapest price offered by the competing advertiser.
    #[prost(message, optional, tag = "129")]
    pub hotel_price_difference_percentage: ::core::option::Option<f64>,
    /// The number of impressions that hotel partners could have had given their
    /// feed performance.
    #[prost(message, optional, tag = "130")]
    pub hotel_eligible_impressions: ::core::option::Option<i64>,
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
    #[prost(message, optional, tag = "82")]
    pub historical_quality_score: ::core::option::Option<i64>,
    /// The historical search predicted click through rate (CTR).
    #[prost(
        enumeration = "super::enums::quality_score_bucket_enum::QualityScoreBucket",
        tag = "83"
    )]
    pub historical_search_predicted_ctr: i32,
    /// The number of times the ad was forwarded to someone else as a message.
    #[prost(message, optional, tag = "85")]
    pub gmail_forwards: ::core::option::Option<i64>,
    /// The number of times someone has saved your Gmail ad to their inbox as a
    /// message.
    #[prost(message, optional, tag = "86")]
    pub gmail_saves: ::core::option::Option<i64>,
    /// The number of clicks to the landing page on the expanded state of Gmail
    /// ads.
    #[prost(message, optional, tag = "87")]
    pub gmail_secondary_clicks: ::core::option::Option<i64>,
    /// The number of times a store's location-based ad was shown.
    /// This metric applies to feed items only.
    #[prost(message, optional, tag = "125")]
    pub impressions_from_store_reach: ::core::option::Option<i64>,
    /// Count of how often your ad has appeared on a search results page or
    /// website on the Google Network.
    #[prost(message, optional, tag = "37")]
    pub impressions: ::core::option::Option<i64>,
    /// How often people interact with your ad after it is shown to them.
    /// This is the number of interactions divided by the number of times your ad
    /// is shown.
    #[prost(message, optional, tag = "38")]
    pub interaction_rate: ::core::option::Option<f64>,
    /// The number of interactions.
    /// An interaction is the main user action associated with an ad format-clicks
    /// for text and shopping ads, views for video ads, and so on.
    #[prost(message, optional, tag = "39")]
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
    #[prost(message, optional, tag = "40")]
    pub invalid_click_rate: ::core::option::Option<f64>,
    /// Number of clicks Google considers illegitimate and doesn't charge you for.
    #[prost(message, optional, tag = "41")]
    pub invalid_clicks: ::core::option::Option<i64>,
    /// Number of message chats initiated for Click To Message impressions that
    /// were message tracking eligible.
    #[prost(message, optional, tag = "126")]
    pub message_chats: ::core::option::Option<i64>,
    /// Number of Click To Message impressions that were message tracking eligible.
    #[prost(message, optional, tag = "127")]
    pub message_impressions: ::core::option::Option<i64>,
    /// Number of message chats initiated (message_chats) divided by the number
    /// of message impressions (message_impressions).
    /// Rate at which a user initiates a message chat from an ad impression with
    /// a messaging option and message tracking enabled.
    /// Note that this rate can be more than 1.0 for a given message impression.
    #[prost(message, optional, tag = "128")]
    pub message_chat_rate: ::core::option::Option<f64>,
    /// The percentage of mobile clicks that go to a mobile-friendly page.
    #[prost(message, optional, tag = "109")]
    pub mobile_friendly_clicks_percentage: ::core::option::Option<f64>,
    /// The number of times someone clicked your site's listing in the unpaid
    /// results for a particular query. See the help page at
    /// https://support.google.com/google-ads/answer/3097241 for details.
    #[prost(message, optional, tag = "110")]
    pub organic_clicks: ::core::option::Option<i64>,
    /// The number of times someone clicked your site's listing in the unpaid
    /// results (organic_clicks) divided by the total number of searches that
    /// returned pages from your site (organic_queries). See the help page at
    /// https://support.google.com/google-ads/answer/3097241 for details.
    #[prost(message, optional, tag = "111")]
    pub organic_clicks_per_query: ::core::option::Option<f64>,
    /// The number of listings for your site in the unpaid search results. See the
    /// help page at https://support.google.com/google-ads/answer/3097241 for
    /// details.
    #[prost(message, optional, tag = "112")]
    pub organic_impressions: ::core::option::Option<i64>,
    /// The number of times a page from your site was listed in the unpaid search
    /// results (organic_impressions) divided by the number of searches returning
    /// your site's listing in the unpaid results (organic_queries). See the help
    /// page at https://support.google.com/google-ads/answer/3097241 for details.
    #[prost(message, optional, tag = "113")]
    pub organic_impressions_per_query: ::core::option::Option<f64>,
    /// The total number of searches that returned your site's listing in the
    /// unpaid results. See the help page at
    /// https://support.google.com/google-ads/answer/3097241 for details.
    #[prost(message, optional, tag = "114")]
    pub organic_queries: ::core::option::Option<i64>,
    /// Percentage of first-time sessions (from people who had never visited your
    /// site before). Imported from Google Analytics.
    #[prost(message, optional, tag = "42")]
    pub percent_new_visitors: ::core::option::Option<f64>,
    /// Number of offline phone calls.
    #[prost(message, optional, tag = "43")]
    pub phone_calls: ::core::option::Option<i64>,
    /// Number of offline phone impressions.
    #[prost(message, optional, tag = "44")]
    pub phone_impressions: ::core::option::Option<i64>,
    /// Number of phone calls received (phone_calls) divided by the number of
    /// times your phone number is shown (phone_impressions).
    #[prost(message, optional, tag = "45")]
    pub phone_through_rate: ::core::option::Option<f64>,
    /// Your clickthrough rate (Ctr) divided by the average clickthrough rate of
    /// all advertisers on the websites that show your ads. Measures how your ads
    /// perform on Display Network sites compared to other ads on the same sites.
    #[prost(message, optional, tag = "46")]
    pub relative_ctr: ::core::option::Option<f64>,
    /// The percentage of the customer's Shopping or Search ad impressions that are
    /// shown in the most prominent Shopping position. See
    /// https://support.google.com/google-ads/answer/7501826
    /// for details. Any value below 0.1 is reported as 0.0999.
    #[prost(message, optional, tag = "78")]
    pub search_absolute_top_impression_share: ::core::option::Option<f64>,
    /// The number estimating how often your ad wasn't the very first ad above the
    /// organic search results due to a low budget. Note: Search
    /// budget lost absolute top impression share is reported in the range of 0 to
    /// 0.9. Any value above 0.9 is reported as 0.9001.
    #[prost(message, optional, tag = "88")]
    pub search_budget_lost_absolute_top_impression_share: ::core::option::Option<f64>,
    /// The estimated percent of times that your ad was eligible to show on the
    /// Search Network but didn't because your budget was too low. Note: Search
    /// budget lost impression share is reported in the range of 0 to 0.9. Any
    /// value above 0.9 is reported as 0.9001.
    #[prost(message, optional, tag = "47")]
    pub search_budget_lost_impression_share: ::core::option::Option<f64>,
    /// The number estimating how often your ad didn't show anywhere above the
    /// organic search results due to a low budget. Note: Search
    /// budget lost top impression share is reported in the range of 0 to 0.9. Any
    /// value above 0.9 is reported as 0.9001.
    #[prost(message, optional, tag = "89")]
    pub search_budget_lost_top_impression_share: ::core::option::Option<f64>,
    /// The number of clicks you've received on the Search Network
    /// divided by the estimated number of clicks you were eligible to receive.
    /// Note: Search click share is reported in the range of 0.1 to 1. Any value
    /// below 0.1 is reported as 0.0999.
    #[prost(message, optional, tag = "48")]
    pub search_click_share: ::core::option::Option<f64>,
    /// The impressions you've received divided by the estimated number of
    /// impressions you were eligible to receive on the Search Network for search
    /// terms that matched your keywords exactly (or were close variants of your
    /// keyword), regardless of your keyword match types. Note: Search exact match
    /// impression share is reported in the range of 0.1 to 1. Any value below 0.1
    /// is reported as 0.0999.
    #[prost(message, optional, tag = "49")]
    pub search_exact_match_impression_share: ::core::option::Option<f64>,
    /// The impressions you've received on the Search Network divided
    /// by the estimated number of impressions you were eligible to receive.
    /// Note: Search impression share is reported in the range of 0.1 to 1. Any
    /// value below 0.1 is reported as 0.0999.
    #[prost(message, optional, tag = "50")]
    pub search_impression_share: ::core::option::Option<f64>,
    /// The number estimating how often your ad wasn't the very first ad above the
    /// organic search results due to poor Ad Rank.
    /// Note: Search rank lost absolute top impression share is reported in the
    /// range of 0 to 0.9. Any value above 0.9 is reported as 0.9001.
    #[prost(message, optional, tag = "90")]
    pub search_rank_lost_absolute_top_impression_share: ::core::option::Option<f64>,
    /// The estimated percentage of impressions on the Search Network
    /// that your ads didn't receive due to poor Ad Rank.
    /// Note: Search rank lost impression share is reported in the range of 0 to
    /// 0.9. Any value above 0.9 is reported as 0.9001.
    #[prost(message, optional, tag = "51")]
    pub search_rank_lost_impression_share: ::core::option::Option<f64>,
    /// The number estimating how often your ad didn't show anywhere above the
    /// organic search results due to poor Ad Rank.
    /// Note: Search rank lost top impression share is reported in the range of 0
    /// to 0.9. Any value above 0.9 is reported as 0.9001.
    #[prost(message, optional, tag = "91")]
    pub search_rank_lost_top_impression_share: ::core::option::Option<f64>,
    /// The impressions you've received in the top location (anywhere above the
    /// organic search results) compared to the estimated number of impressions you
    /// were eligible to receive in the top location.
    /// Note: Search top impression share is reported in the range of 0.1 to 1. Any
    /// value below 0.1 is reported as 0.0999.
    #[prost(message, optional, tag = "92")]
    pub search_top_impression_share: ::core::option::Option<f64>,
    /// A measure of how quickly your page loads after clicks on your mobile ads.
    /// The score is a range from 1 to 10, 10 being the fastest.
    #[prost(message, optional, tag = "107")]
    pub speed_score: ::core::option::Option<i64>,
    /// The percent of your ad impressions that are shown anywhere above the
    /// organic search results.
    #[prost(message, optional, tag = "93")]
    pub top_impression_percentage: ::core::option::Option<f64>,
    /// The percentage of ad clicks to Accelerated Mobile Pages (AMP) landing pages
    /// that reach a valid AMP page.
    #[prost(message, optional, tag = "108")]
    pub valid_accelerated_mobile_pages_clicks_percentage: ::core::option::Option<f64>,
    /// The value of all conversions divided by the number of all conversions.
    #[prost(message, optional, tag = "52")]
    pub value_per_all_conversions: ::core::option::Option<f64>,
    /// The value of conversions divided by the number of conversions. This only
    /// includes conversion actions which include_in_conversions_metric attribute
    /// is set to true. If you use conversion-based bidding, your bid strategies
    /// will optimize for these conversions.
    #[prost(message, optional, tag = "53")]
    pub value_per_conversion: ::core::option::Option<f64>,
    /// The value of current model attributed conversions divided by the number of
    /// the conversions. This only includes conversion actions which
    /// include_in_conversions_metric attribute is set to true. If you use
    /// conversion-based bidding, your bid strategies will optimize for these
    /// conversions.
    #[prost(message, optional, tag = "94")]
    pub value_per_current_model_attributed_conversion: ::core::option::Option<f64>,
    /// Percentage of impressions where the viewer watched all of your video.
    #[prost(message, optional, tag = "54")]
    pub video_quartile_100_rate: ::core::option::Option<f64>,
    /// Percentage of impressions where the viewer watched 25% of your video.
    #[prost(message, optional, tag = "55")]
    pub video_quartile_25_rate: ::core::option::Option<f64>,
    /// Percentage of impressions where the viewer watched 50% of your video.
    #[prost(message, optional, tag = "56")]
    pub video_quartile_50_rate: ::core::option::Option<f64>,
    /// Percentage of impressions where the viewer watched 75% of your video.
    #[prost(message, optional, tag = "57")]
    pub video_quartile_75_rate: ::core::option::Option<f64>,
    /// The number of views your TrueView video ad receives divided by its number
    /// of impressions, including thumbnail impressions for TrueView in-display
    /// ads.
    #[prost(message, optional, tag = "58")]
    pub video_view_rate: ::core::option::Option<f64>,
    /// The number of times your video ads were viewed.
    #[prost(message, optional, tag = "59")]
    pub video_views: ::core::option::Option<i64>,
    /// The total number of view-through conversions.
    /// These happen when a customer sees an image or rich media ad, then later
    /// completes a conversion on your site without interacting with (e.g.,
    /// clicking on) another ad.
    #[prost(message, optional, tag = "60")]
    pub view_through_conversions: ::core::option::Option<i64>,
}
// Proto file describing offline user data.

/// Address identifier of offline data.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OfflineUserAddressInfo {
    /// First name of the user, which is hashed as SHA-256 after normalized
    /// (Lowercase all characters; Remove any extra spaces before, after, and in
    /// between).
    #[prost(message, optional, tag = "1")]
    pub hashed_first_name: ::core::option::Option<::prost::alloc::string::String>,
    /// Last name of the user, which is hashed as SHA-256 after normalized (lower
    /// case only and no punctuation).
    #[prost(message, optional, tag = "2")]
    pub hashed_last_name: ::core::option::Option<::prost::alloc::string::String>,
    /// City of the address. Only accepted for Store Sales Direct data.
    #[prost(message, optional, tag = "3")]
    pub city: ::core::option::Option<::prost::alloc::string::String>,
    /// State code of the address. Only accepted for Store Sales Direct data.
    #[prost(message, optional, tag = "4")]
    pub state: ::core::option::Option<::prost::alloc::string::String>,
    /// 2-letter country code in ISO-3166-1 alpha-2 of the user's address.
    #[prost(message, optional, tag = "5")]
    pub country_code: ::core::option::Option<::prost::alloc::string::String>,
    /// Postal code of the user's address.
    #[prost(message, optional, tag = "6")]
    pub postal_code: ::core::option::Option<::prost::alloc::string::String>,
}
/// Hashed user identifying information.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserIdentifier {
    /// Exactly one must be specified.
    #[prost(oneof = "user_identifier::Identifier", tags = "1, 2, 3, 4, 5")]
    pub identifier: ::core::option::Option<user_identifier::Identifier>,
}
/// Nested message and enum types in `UserIdentifier`.
pub mod user_identifier {
    /// Exactly one must be specified.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Identifier {
        /// Hashed email address using SHA-256 hash function after normalization.
        #[prost(message, tag = "1")]
        HashedEmail(::prost::alloc::string::String),
        /// Hashed phone number using SHA-256 hash function after normalization
        /// (E164 standard).
        #[prost(message, tag = "2")]
        HashedPhoneNumber(::prost::alloc::string::String),
        /// Mobile device ID (advertising ID/IDFA).
        #[prost(message, tag = "3")]
        MobileId(::prost::alloc::string::String),
        /// Advertiser-assigned user ID for Customer Match upload, or
        /// third-party-assigned user ID for SSD.
        #[prost(message, tag = "4")]
        ThirdPartyUserId(::prost::alloc::string::String),
        /// Address information.
        #[prost(message, tag = "5")]
        AddressInfo(super::OfflineUserAddressInfo),
    }
}
/// Attribute of the store sales transaction.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionAttribute {
    /// Timestamp when transaction occurred. Required.
    /// The format is "YYYY-MM-DD HH:MM:SS".
    /// Examples: "2018-03-05 09:15:00" or "2018-02-01 14:34:30"
    #[prost(message, optional, tag = "1")]
    pub transaction_date_time: ::core::option::Option<::prost::alloc::string::String>,
    /// Transaction amount in micros. Required.
    #[prost(message, optional, tag = "2")]
    pub transaction_amount_micros: ::core::option::Option<f64>,
    /// Transaction currency code. ISO 4217 three-letter code is used. Required.
    #[prost(message, optional, tag = "3")]
    pub currency_code: ::core::option::Option<::prost::alloc::string::String>,
    /// The resource name of conversion action to report conversions to.
    /// Required.
    #[prost(message, optional, tag = "4")]
    pub conversion_action: ::core::option::Option<::prost::alloc::string::String>,
    /// Transaction order id.
    /// Accessible only to customers on the allow-list.
    #[prost(message, optional, tag = "5")]
    pub order_id: ::core::option::Option<::prost::alloc::string::String>,
    /// Store attributes of the transaction.
    /// Accessible only to customers on the allow-list.
    #[prost(message, optional, tag = "6")]
    pub store_attribute: ::core::option::Option<StoreAttribute>,
    /// Value of the custom variable for each transaction.
    /// Accessible only to customers on the allow-list.
    #[prost(message, optional, tag = "7")]
    pub custom_value: ::core::option::Option<::prost::alloc::string::String>,
}
/// Store attributes of the transaction.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StoreAttribute {
    /// Store code from
    /// https://support.google.com/business/answer/3370250#storecode
    #[prost(message, optional, tag = "1")]
    pub store_code: ::core::option::Option<::prost::alloc::string::String>,
}
/// User data holding user identifiers and attributes.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserData {
    /// User identification info. Required.
    #[prost(message, repeated, tag = "1")]
    pub user_identifiers: ::prost::alloc::vec::Vec<UserIdentifier>,
    /// Additional transactions/attributes associated with the user.
    /// Required when updating store sales data.
    #[prost(message, optional, tag = "2")]
    pub transaction_attribute: ::core::option::Option<TransactionAttribute>,
}
/// Metadata for customer match user list.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomerMatchUserListMetadata {
    /// The resource name of remarketing list to update data.
    /// Required for job of CUSTOMER_MATCH_USER_LIST type.
    #[prost(message, optional, tag = "1")]
    pub user_list: ::core::option::Option<::prost::alloc::string::String>,
}
/// Metadata for Store Sales Direct.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StoreSalesMetadata {
    /// This is the fraction of all transactions that are identifiable (i.e.,
    /// associated with any form of customer information).
    /// Required.
    /// The fraction needs to be between 0 and 1 (excluding 0).
    #[prost(message, optional, tag = "1")]
    pub loyalty_fraction: ::core::option::Option<f64>,
    /// This is the ratio of sales being uploaded compared to the overall sales
    /// that can be associated with a customer. Required.
    /// The fraction needs to be between 0 and 1 (excluding 0). For example, if you
    /// upload half the sales that you are able to associate with a customer, this
    /// would be 0.5.
    #[prost(message, optional, tag = "2")]
    pub transaction_upload_fraction: ::core::option::Option<f64>,
    /// Name of the store sales custom variable key. A predefined key that
    /// can be applied to the transaction and then later used for custom
    /// segmentation in reporting.
    /// Accessible only to customers on the allow-list.
    #[prost(message, optional, tag = "4")]
    pub custom_key: ::core::option::Option<::prost::alloc::string::String>,
    /// Metadata for a third party Store Sales upload.
    #[prost(message, optional, tag = "3")]
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
    #[prost(message, optional, tag = "1")]
    pub advertiser_upload_date_time: ::core::option::Option<::prost::alloc::string::String>,
    /// The fraction of transactions that are valid. Invalid transactions may
    /// include invalid formats or values.
    /// Required.
    /// The fraction needs to be between 0 and 1 (excluding 0).
    #[prost(message, optional, tag = "2")]
    pub valid_transaction_fraction: ::core::option::Option<f64>,
    /// The fraction of valid transactions that are matched to a third party
    /// assigned user ID on the partner side.
    /// Required.
    /// The fraction needs to be between 0 and 1 (excluding 0).
    #[prost(message, optional, tag = "3")]
    pub partner_match_fraction: ::core::option::Option<f64>,
    /// The fraction of valid transactions that are uploaded by the partner to
    /// Google.
    /// Required.
    /// The fraction needs to be between 0 and 1 (excluding 0).
    #[prost(message, optional, tag = "4")]
    pub partner_upload_fraction: ::core::option::Option<f64>,
    /// Version of partner IDs to be used for uploads. Required.
    #[prost(message, optional, tag = "5")]
    pub bridge_map_version_id: ::core::option::Option<::prost::alloc::string::String>,
    /// ID of the third party partner updating the transaction feed.
    #[prost(message, optional, tag = "6")]
    pub partner_id: ::core::option::Option<i64>,
}
// Proto file describing policy information.

/// Key of the violation. The key is used for referring to a violation
/// when filing an exemption request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PolicyViolationKey {
    /// Unique ID of the violated policy.
    #[prost(message, optional, tag = "1")]
    pub policy_name: ::core::option::Option<::prost::alloc::string::String>,
    /// The text that violates the policy if specified.
    /// Otherwise, refers to the policy in general
    /// (e.g., when requesting to be exempt from the whole policy).
    /// If not specified for criterion exemptions, the whole policy is implied.
    /// Must be specified for ad exemptions.
    #[prost(message, optional, tag = "2")]
    pub violating_text: ::core::option::Option<::prost::alloc::string::String>,
}
/// Parameter for controlling how policy exemption is done. Ignorable policy
/// topics are only usable with expanded text ads and responsive search ads. All
/// other ad types must use policy violation keys.
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
    #[prost(message, repeated, tag = "1")]
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
    #[prost(message, repeated, tag = "2")]
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
    #[prost(message, optional, tag = "1")]
    pub topic: ::core::option::Option<::prost::alloc::string::String>,
    /// Describes the negative or positive effect this policy will have on serving.
    #[prost(
        enumeration = "super::enums::policy_topic_entry_type_enum::PolicyTopicEntryType",
        tag = "2"
    )]
    pub r#type: i32,
    /// Additional information that explains policy finding
    /// (e.g. the brand name for a trademark finding).
    #[prost(message, repeated, tag = "3")]
    pub evidences: ::prost::alloc::vec::Vec<PolicyTopicEvidence>,
    /// Indicates how serving of this resource may be affected (e.g. not serving
    /// in a country).
    #[prost(message, repeated, tag = "4")]
    pub constraints: ::prost::alloc::vec::Vec<PolicyTopicConstraint>,
}
/// Additional information that explains a policy finding.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PolicyTopicEvidence {
    /// Specific evidence information depending on the evidence type.
    #[prost(oneof = "policy_topic_evidence::Value", tags = "3, 4, 5, 6, 7, 8")]
    pub value: ::core::option::Option<policy_topic_evidence::Value>,
}
/// Nested message and enum types in `PolicyTopicEvidence`.
pub mod policy_topic_evidence {
    /// A list of fragments of text that violated a policy.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TextList {
        /// The fragments of text from the resource that caused the policy finding.
        #[prost(message, repeated, tag = "1")]
        pub texts: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
    /// A list of websites that caused a policy finding. Used for
    /// ONE_WEBSITE_PER_AD_GROUP policy topic, for example. In case there are more
    /// than five websites, only the top five (those that appear in resources the
    /// most) will be listed here.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct WebsiteList {
        /// Websites that caused the policy finding.
        #[prost(message, repeated, tag = "1")]
        pub websites: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
    /// A list of strings found in a destination page that caused a policy
    /// finding.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DestinationTextList {
        /// List of text found in the resource's destination page.
        #[prost(message, repeated, tag = "1")]
        pub destination_texts: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
    /// Evidence of mismatches between the URLs of a resource.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DestinationMismatch {
        /// The set of URLs that did not match each other.
        #[prost(
            enumeration = "super::super::enums::policy_topic_evidence_destination_mismatch_url_type_enum::PolicyTopicEvidenceDestinationMismatchUrlType",
            repeated,
            tag = "1"
        )]
        pub url_types: ::prost::alloc::vec::Vec<i32>,
    }
    /// Evidence details when the destination is returning an HTTP error
    /// code or isn't functional in all locations for commonly used devices.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DestinationNotWorking {
        /// The full URL that didn't work.
        #[prost(message, optional, tag = "3")]
        pub expanded_url: ::core::option::Option<::prost::alloc::string::String>,
        /// The type of device that failed to load the URL.
        #[prost(
            enumeration = "super::super::enums::policy_topic_evidence_destination_not_working_device_enum::PolicyTopicEvidenceDestinationNotWorkingDevice",
            tag = "4"
        )]
        pub device: i32,
        /// The time the URL was last checked.
        /// The format is "YYYY-MM-DD HH:MM:SS".
        /// Examples: "2018-03-05 09:15:00" or "2018-02-01 14:34:30"
        #[prost(message, optional, tag = "5")]
        pub last_checked_date_time: ::core::option::Option<::prost::alloc::string::String>,
        /// Indicates the reason of the DESTINATION_NOT_WORKING policy finding.
        #[prost(oneof = "destination_not_working::Reason", tags = "1, 2")]
        pub reason: ::core::option::Option<destination_not_working::Reason>,
    }
    /// Nested message and enum types in `DestinationNotWorking`.
    pub mod destination_not_working {
        /// Indicates the reason of the DESTINATION_NOT_WORKING policy finding.
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Reason {
            /// The type of DNS error.
            #[prost(
                enumeration = "super::super::super::enums::policy_topic_evidence_destination_not_working_dns_error_type_enum::PolicyTopicEvidenceDestinationNotWorkingDnsErrorType",
                tag = "1"
            )]
            DnsErrorType(i32),
            /// The HTTP error code.
            #[prost(message, tag = "2")]
            HttpErrorCode(i64),
        }
    }
    /// Specific evidence information depending on the evidence type.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Value {
        /// List of websites linked with this resource.
        #[prost(message, tag = "3")]
        WebsiteList(WebsiteList),
        /// List of evidence found in the text of a resource.
        #[prost(message, tag = "4")]
        TextList(TextList),
        /// The language the resource was detected to be written in.
        /// This is an IETF language tag such as "en-US".
        #[prost(message, tag = "5")]
        LanguageCode(::prost::alloc::string::String),
        /// The text in the destination of the resource that is causing a policy
        /// finding.
        #[prost(message, tag = "6")]
        DestinationTextList(DestinationTextList),
        /// Mismatch between the destinations of a resource's URLs.
        #[prost(message, tag = "7")]
        DestinationMismatch(DestinationMismatch),
        /// Details when the destination is returning an HTTP error code or isn't
        /// functional in all locations for commonly used devices.
        #[prost(message, tag = "8")]
        DestinationNotWorking(DestinationNotWorking),
    }
}
/// Describes the effect on serving that a policy topic entry will have.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PolicyTopicConstraint {
    /// Specific information about the constraint.
    #[prost(oneof = "policy_topic_constraint::Value", tags = "1, 2, 3, 4")]
    pub value: ::core::option::Option<policy_topic_constraint::Value>,
}
/// Nested message and enum types in `PolicyTopicConstraint`.
pub mod policy_topic_constraint {
    /// A list of countries where a resource's serving is constrained.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CountryConstraintList {
        /// Total number of countries targeted by the resource.
        #[prost(message, optional, tag = "1")]
        pub total_targeted_countries: ::core::option::Option<i32>,
        /// Countries in which serving is restricted.
        #[prost(message, repeated, tag = "2")]
        pub countries: ::prost::alloc::vec::Vec<CountryConstraint>,
    }
    /// Indicates that a policy topic was constrained due to disapproval of the
    /// website for reseller purposes.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ResellerConstraint {}
    /// Indicates that a resource's ability to serve in a particular country is
    /// constrained.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CountryConstraint {
        /// Geo target constant resource name of the country in which serving is
        /// constrained.
        #[prost(message, optional, tag = "1")]
        pub country_criterion: ::core::option::Option<::prost::alloc::string::String>,
    }
    /// Specific information about the constraint.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Value {
        /// Countries where the resource cannot serve.
        #[prost(message, tag = "1")]
        CountryConstraintList(CountryConstraintList),
        /// Reseller constraint.
        #[prost(message, tag = "2")]
        ResellerConstraint(ResellerConstraint),
        /// Countries where a certificate is required for serving.
        #[prost(message, tag = "3")]
        CertificateMissingInCountryList(CountryConstraintList),
        /// Countries where the resource's domain is not covered by the
        /// certificates associated with it.
        #[prost(message, tag = "4")]
        CertificateDomainMismatchInCountryList(CountryConstraintList),
    }
}
// Proto file describing RealTimeBiddingSetting

/// Settings for Real-Time Bidding, a feature only available for campaigns
/// targeting the Ad Exchange network.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RealTimeBiddingSetting {
    /// Whether the campaign is opted in to real-time bidding.
    #[prost(message, optional, tag = "1")]
    pub opt_in: ::core::option::Option<bool>,
}
// Proto file describing segment only fields.

/// Segment only fields.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Segments {
    /// Ad network type.
    #[prost(
        enumeration = "super::enums::ad_network_type_enum::AdNetworkType",
        tag = "3"
    )]
    pub ad_network_type: i32,
    /// Click type.
    #[prost(enumeration = "super::enums::click_type_enum::ClickType", tag = "26")]
    pub click_type: i32,
    /// Resource name of the conversion action.
    #[prost(message, optional, tag = "52")]
    pub conversion_action: ::core::option::Option<::prost::alloc::string::String>,
    /// Conversion action category.
    #[prost(
        enumeration = "super::enums::conversion_action_category_enum::ConversionActionCategory",
        tag = "53"
    )]
    pub conversion_action_category: i32,
    /// Conversion action name.
    #[prost(message, optional, tag = "54")]
    pub conversion_action_name: ::core::option::Option<::prost::alloc::string::String>,
    /// This segments your conversion columns by the original conversion and
    /// conversion value vs. the delta if conversions were adjusted. False row has
    /// the data as originally stated; While true row has the delta between data
    /// now and the data as originally stated. Summing the two together results
    /// post-adjustment data.
    #[prost(message, optional, tag = "27")]
    pub conversion_adjustment: ::core::option::Option<bool>,
    /// Conversion attribution event type.
    #[prost(
        enumeration = "super::enums::conversion_attribution_event_type_enum::ConversionAttributionEventType",
        tag = "2"
    )]
    pub conversion_attribution_event_type: i32,
    /// An enum value representing the number of days between the impression and
    /// the conversion.
    #[prost(
        enumeration = "super::enums::conversion_lag_bucket_enum::ConversionLagBucket",
        tag = "50"
    )]
    pub conversion_lag_bucket: i32,
    /// An enum value representing the number of days between the impression and
    /// the conversion or between the impression and adjustments to the conversion.
    #[prost(
        enumeration = "super::enums::conversion_or_adjustment_lag_bucket_enum::ConversionOrAdjustmentLagBucket",
        tag = "51"
    )]
    pub conversion_or_adjustment_lag_bucket: i32,
    /// Date to which metrics apply.
    /// yyyy-MM-dd format, e.g., 2018-04-17.
    #[prost(message, optional, tag = "4")]
    pub date: ::core::option::Option<::prost::alloc::string::String>,
    /// Day of the week, e.g., MONDAY.
    #[prost(enumeration = "super::enums::day_of_week_enum::DayOfWeek", tag = "5")]
    pub day_of_week: i32,
    /// Device to which metrics apply.
    #[prost(enumeration = "super::enums::device_enum::Device", tag = "1")]
    pub device: i32,
    /// External conversion source.
    #[prost(
        enumeration = "super::enums::external_conversion_source_enum::ExternalConversionSource",
        tag = "55"
    )]
    pub external_conversion_source: i32,
    /// Resource name of the geo target constant that represents an airport.
    #[prost(message, optional, tag = "65")]
    pub geo_target_airport: ::core::option::Option<::prost::alloc::string::String>,
    /// Resource name of the geo target constant that represents a canton.
    #[prost(message, optional, tag = "76")]
    pub geo_target_canton: ::core::option::Option<::prost::alloc::string::String>,
    /// Resource name of the geo target constant that represents a city.
    #[prost(message, optional, tag = "62")]
    pub geo_target_city: ::core::option::Option<::prost::alloc::string::String>,
    /// Resource name of the geo target constant that represents a country.
    #[prost(message, optional, tag = "77")]
    pub geo_target_country: ::core::option::Option<::prost::alloc::string::String>,
    /// Resource name of the geo target constant that represents a county.
    #[prost(message, optional, tag = "68")]
    pub geo_target_county: ::core::option::Option<::prost::alloc::string::String>,
    /// Resource name of the geo target constant that represents a district.
    #[prost(message, optional, tag = "69")]
    pub geo_target_district: ::core::option::Option<::prost::alloc::string::String>,
    /// Resource name of the geo target constant that represents a metro.
    #[prost(message, optional, tag = "63")]
    pub geo_target_metro: ::core::option::Option<::prost::alloc::string::String>,
    /// Resource name of the geo target constant that represents the most
    /// specific location.
    #[prost(message, optional, tag = "72")]
    pub geo_target_most_specific_location: ::core::option::Option<::prost::alloc::string::String>,
    /// Resource name of the geo target constant that represents a postal code.
    #[prost(message, optional, tag = "71")]
    pub geo_target_postal_code: ::core::option::Option<::prost::alloc::string::String>,
    /// Resource name of the geo target constant that represents a province.
    #[prost(message, optional, tag = "75")]
    pub geo_target_province: ::core::option::Option<::prost::alloc::string::String>,
    /// Resource name of the geo target constant that represents a region.
    #[prost(message, optional, tag = "64")]
    pub geo_target_region: ::core::option::Option<::prost::alloc::string::String>,
    /// Resource name of the geo target constant that represents a state.
    #[prost(message, optional, tag = "67")]
    pub geo_target_state: ::core::option::Option<::prost::alloc::string::String>,
    /// Hotel booking window in days.
    #[prost(message, optional, tag = "6")]
    pub hotel_booking_window_days: ::core::option::Option<i64>,
    /// Hotel center ID.
    #[prost(message, optional, tag = "7")]
    pub hotel_center_id: ::core::option::Option<i64>,
    /// Hotel check-in date. Formatted as yyyy-MM-dd.
    #[prost(message, optional, tag = "8")]
    pub hotel_check_in_date: ::core::option::Option<::prost::alloc::string::String>,
    /// Hotel check-in day of week.
    #[prost(enumeration = "super::enums::day_of_week_enum::DayOfWeek", tag = "9")]
    pub hotel_check_in_day_of_week: i32,
    /// Hotel city.
    #[prost(message, optional, tag = "10")]
    pub hotel_city: ::core::option::Option<::prost::alloc::string::String>,
    /// Hotel class.
    #[prost(message, optional, tag = "11")]
    pub hotel_class: ::core::option::Option<i32>,
    /// Hotel country.
    #[prost(message, optional, tag = "12")]
    pub hotel_country: ::core::option::Option<::prost::alloc::string::String>,
    /// Hotel date selection type.
    #[prost(
        enumeration = "super::enums::hotel_date_selection_type_enum::HotelDateSelectionType",
        tag = "13"
    )]
    pub hotel_date_selection_type: i32,
    /// Hotel length of stay.
    #[prost(message, optional, tag = "14")]
    pub hotel_length_of_stay: ::core::option::Option<i32>,
    /// Hotel rate rule ID.
    #[prost(message, optional, tag = "73")]
    pub hotel_rate_rule_id: ::core::option::Option<::prost::alloc::string::String>,
    /// Hotel rate type.
    #[prost(
        enumeration = "super::enums::hotel_rate_type_enum::HotelRateType",
        tag = "74"
    )]
    pub hotel_rate_type: i32,
    /// Hotel price bucket.
    #[prost(
        enumeration = "super::enums::hotel_price_bucket_enum::HotelPriceBucket",
        tag = "78"
    )]
    pub hotel_price_bucket: i32,
    /// Hotel state.
    #[prost(message, optional, tag = "15")]
    pub hotel_state: ::core::option::Option<::prost::alloc::string::String>,
    /// Hour of day as a number between 0 and 23, inclusive.
    #[prost(message, optional, tag = "16")]
    pub hour: ::core::option::Option<i32>,
    /// Only used with feed item metrics.
    /// Indicates whether the interaction metrics occurred on the feed item itself
    /// or a different extension or ad unit.
    #[prost(message, optional, tag = "49")]
    pub interaction_on_this_extension: ::core::option::Option<bool>,
    /// Keyword criterion.
    #[prost(message, optional, tag = "61")]
    pub keyword: ::core::option::Option<Keyword>,
    /// Month as represented by the date of the first day of a month. Formatted as
    /// yyyy-MM-dd.
    #[prost(message, optional, tag = "17")]
    pub month: ::core::option::Option<::prost::alloc::string::String>,
    /// Month of the year, e.g., January.
    #[prost(
        enumeration = "super::enums::month_of_year_enum::MonthOfYear",
        tag = "18"
    )]
    pub month_of_year: i32,
    /// Partner hotel ID.
    #[prost(message, optional, tag = "19")]
    pub partner_hotel_id: ::core::option::Option<::prost::alloc::string::String>,
    /// Placeholder type. This is only used with feed item metrics.
    #[prost(
        enumeration = "super::enums::placeholder_type_enum::PlaceholderType",
        tag = "20"
    )]
    pub placeholder_type: i32,
    /// Aggregator ID of the product.
    #[prost(message, optional, tag = "28")]
    pub product_aggregator_id: ::core::option::Option<u64>,
    /// Bidding category (level 1) of the product.
    #[prost(message, optional, tag = "56")]
    pub product_bidding_category_level1: ::core::option::Option<::prost::alloc::string::String>,
    /// Bidding category (level 2) of the product.
    #[prost(message, optional, tag = "57")]
    pub product_bidding_category_level2: ::core::option::Option<::prost::alloc::string::String>,
    /// Bidding category (level 3) of the product.
    #[prost(message, optional, tag = "58")]
    pub product_bidding_category_level3: ::core::option::Option<::prost::alloc::string::String>,
    /// Bidding category (level 4) of the product.
    #[prost(message, optional, tag = "59")]
    pub product_bidding_category_level4: ::core::option::Option<::prost::alloc::string::String>,
    /// Bidding category (level 5) of the product.
    #[prost(message, optional, tag = "60")]
    pub product_bidding_category_level5: ::core::option::Option<::prost::alloc::string::String>,
    /// Brand of the product.
    #[prost(message, optional, tag = "29")]
    pub product_brand: ::core::option::Option<::prost::alloc::string::String>,
    /// Channel of the product.
    #[prost(
        enumeration = "super::enums::product_channel_enum::ProductChannel",
        tag = "30"
    )]
    pub product_channel: i32,
    /// Channel exclusivity of the product.
    #[prost(
        enumeration = "super::enums::product_channel_exclusivity_enum::ProductChannelExclusivity",
        tag = "31"
    )]
    pub product_channel_exclusivity: i32,
    /// Condition of the product.
    #[prost(
        enumeration = "super::enums::product_condition_enum::ProductCondition",
        tag = "32"
    )]
    pub product_condition: i32,
    /// Resource name of the geo target constant for the country of sale of the
    /// product.
    #[prost(message, optional, tag = "33")]
    pub product_country: ::core::option::Option<::prost::alloc::string::String>,
    /// Custom attribute 0 of the product.
    #[prost(message, optional, tag = "34")]
    pub product_custom_attribute0: ::core::option::Option<::prost::alloc::string::String>,
    /// Custom attribute 1 of the product.
    #[prost(message, optional, tag = "35")]
    pub product_custom_attribute1: ::core::option::Option<::prost::alloc::string::String>,
    /// Custom attribute 2 of the product.
    #[prost(message, optional, tag = "36")]
    pub product_custom_attribute2: ::core::option::Option<::prost::alloc::string::String>,
    /// Custom attribute 3 of the product.
    #[prost(message, optional, tag = "37")]
    pub product_custom_attribute3: ::core::option::Option<::prost::alloc::string::String>,
    /// Custom attribute 4 of the product.
    #[prost(message, optional, tag = "38")]
    pub product_custom_attribute4: ::core::option::Option<::prost::alloc::string::String>,
    /// Item ID of the product.
    #[prost(message, optional, tag = "39")]
    pub product_item_id: ::core::option::Option<::prost::alloc::string::String>,
    /// Resource name of the language constant for the language of the product.
    #[prost(message, optional, tag = "40")]
    pub product_language: ::core::option::Option<::prost::alloc::string::String>,
    /// Merchant ID of the product.
    #[prost(message, optional, tag = "41")]
    pub product_merchant_id: ::core::option::Option<u64>,
    /// Store ID of the product.
    #[prost(message, optional, tag = "42")]
    pub product_store_id: ::core::option::Option<::prost::alloc::string::String>,
    /// Title of the product.
    #[prost(message, optional, tag = "43")]
    pub product_title: ::core::option::Option<::prost::alloc::string::String>,
    /// Type (level 1) of the product.
    #[prost(message, optional, tag = "44")]
    pub product_type_l1: ::core::option::Option<::prost::alloc::string::String>,
    /// Type (level 2) of the product.
    #[prost(message, optional, tag = "45")]
    pub product_type_l2: ::core::option::Option<::prost::alloc::string::String>,
    /// Type (level 3) of the product.
    #[prost(message, optional, tag = "46")]
    pub product_type_l3: ::core::option::Option<::prost::alloc::string::String>,
    /// Type (level 4) of the product.
    #[prost(message, optional, tag = "47")]
    pub product_type_l4: ::core::option::Option<::prost::alloc::string::String>,
    /// Type (level 5) of the product.
    #[prost(message, optional, tag = "48")]
    pub product_type_l5: ::core::option::Option<::prost::alloc::string::String>,
    /// Quarter as represented by the date of the first day of a quarter.
    /// Uses the calendar year for quarters, e.g., the second quarter of 2018
    /// starts on 2018-04-01. Formatted as yyyy-MM-dd.
    #[prost(message, optional, tag = "21")]
    pub quarter: ::core::option::Option<::prost::alloc::string::String>,
    /// Type of the search engine results page.
    #[prost(
        enumeration = "super::enums::search_engine_results_page_type_enum::SearchEngineResultsPageType",
        tag = "70"
    )]
    pub search_engine_results_page_type: i32,
    /// Match type of the keyword that triggered the ad, including variants.
    #[prost(
        enumeration = "super::enums::search_term_match_type_enum::SearchTermMatchType",
        tag = "22"
    )]
    pub search_term_match_type: i32,
    /// Position of the ad.
    #[prost(enumeration = "super::enums::slot_enum::Slot", tag = "23")]
    pub slot: i32,
    /// Resource name of the ad group criterion that represents webpage criterion.
    #[prost(message, optional, tag = "66")]
    pub webpage: ::core::option::Option<::prost::alloc::string::String>,
    /// Week as defined as Monday through Sunday, and represented by the date of
    /// Monday. Formatted as yyyy-MM-dd.
    #[prost(message, optional, tag = "24")]
    pub week: ::core::option::Option<::prost::alloc::string::String>,
    /// Year, formatted as yyyy.
    #[prost(message, optional, tag = "25")]
    pub year: ::core::option::Option<i32>,
}
/// A Keyword criterion segment.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Keyword {
    /// The AdGroupCriterion resource name.
    #[prost(message, optional, tag = "1")]
    pub ad_group_criterion: ::core::option::Option<::prost::alloc::string::String>,
    /// Keyword info.
    #[prost(message, optional, tag = "2")]
    pub info: ::core::option::Option<KeywordInfo>,
}
// Proto file describing simulation points.

/// A container for simulation points for simulations of type BID_MODIFIER.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BidModifierSimulationPointList {
    /// Projected metrics for a series of bid modifier amounts.
    #[prost(message, repeated, tag = "1")]
    pub points: ::prost::alloc::vec::Vec<BidModifierSimulationPoint>,
}
/// A container for simulation points for simulations of type CPC_BID.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CpcBidSimulationPointList {
    /// Projected metrics for a series of CPC bid amounts.
    #[prost(message, repeated, tag = "1")]
    pub points: ::prost::alloc::vec::Vec<CpcBidSimulationPoint>,
}
/// A container for simulation points for simulations of type CPV_BID.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CpvBidSimulationPointList {
    /// Projected metrics for a series of CPV bid amounts.
    #[prost(message, repeated, tag = "1")]
    pub points: ::prost::alloc::vec::Vec<CpvBidSimulationPoint>,
}
/// A container for simulation points for simulations of type TARGET_CPA.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TargetCpaSimulationPointList {
    /// Projected metrics for a series of target CPA amounts.
    #[prost(message, repeated, tag = "1")]
    pub points: ::prost::alloc::vec::Vec<TargetCpaSimulationPoint>,
}
/// A container for simulation points for simulations of type TARGET_ROAS.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TargetRoasSimulationPointList {
    /// Projected metrics for a series of target ROAS amounts.
    #[prost(message, repeated, tag = "1")]
    pub points: ::prost::alloc::vec::Vec<TargetRoasSimulationPoint>,
}
/// Projected metrics for a specific bid modifier amount.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BidModifierSimulationPoint {
    /// The simulated bid modifier upon which projected metrics are based.
    #[prost(message, optional, tag = "1")]
    pub bid_modifier: ::core::option::Option<f64>,
    /// Projected number of biddable conversions.
    /// Only search advertising channel type supports this field.
    #[prost(message, optional, tag = "2")]
    pub biddable_conversions: ::core::option::Option<f64>,
    /// Projected total value of biddable conversions.
    /// Only search advertising channel type supports this field.
    #[prost(message, optional, tag = "3")]
    pub biddable_conversions_value: ::core::option::Option<f64>,
    /// Projected number of clicks.
    #[prost(message, optional, tag = "4")]
    pub clicks: ::core::option::Option<i64>,
    /// Projected cost in micros.
    #[prost(message, optional, tag = "5")]
    pub cost_micros: ::core::option::Option<i64>,
    /// Projected number of impressions.
    #[prost(message, optional, tag = "6")]
    pub impressions: ::core::option::Option<i64>,
    /// Projected number of top slot impressions.
    /// Only search advertising channel type supports this field.
    #[prost(message, optional, tag = "7")]
    pub top_slot_impressions: ::core::option::Option<i64>,
    /// Projected number of biddable conversions for the parent resource.
    /// Only search advertising channel type supports this field.
    #[prost(message, optional, tag = "8")]
    pub parent_biddable_conversions: ::core::option::Option<f64>,
    /// Projected total value of biddable conversions for the parent resource.
    /// Only search advertising channel type supports this field.
    #[prost(message, optional, tag = "9")]
    pub parent_biddable_conversions_value: ::core::option::Option<f64>,
    /// Projected number of clicks for the parent resource.
    #[prost(message, optional, tag = "10")]
    pub parent_clicks: ::core::option::Option<i64>,
    /// Projected cost in micros for the parent resource.
    #[prost(message, optional, tag = "11")]
    pub parent_cost_micros: ::core::option::Option<i64>,
    /// Projected number of impressions for the parent resource.
    #[prost(message, optional, tag = "12")]
    pub parent_impressions: ::core::option::Option<i64>,
    /// Projected number of top slot impressions for the parent resource.
    /// Only search advertising channel type supports this field.
    #[prost(message, optional, tag = "13")]
    pub parent_top_slot_impressions: ::core::option::Option<i64>,
    /// Projected minimum daily budget that must be available to the parent
    /// resource to realize this simulation.
    #[prost(message, optional, tag = "14")]
    pub parent_required_budget_micros: ::core::option::Option<i64>,
}
/// Projected metrics for a specific CPC bid amount.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CpcBidSimulationPoint {
    /// The simulated CPC bid upon which projected metrics are based.
    #[prost(message, optional, tag = "1")]
    pub cpc_bid_micros: ::core::option::Option<i64>,
    /// Projected number of biddable conversions.
    #[prost(message, optional, tag = "2")]
    pub biddable_conversions: ::core::option::Option<f64>,
    /// Projected total value of biddable conversions.
    #[prost(message, optional, tag = "3")]
    pub biddable_conversions_value: ::core::option::Option<f64>,
    /// Projected number of clicks.
    #[prost(message, optional, tag = "4")]
    pub clicks: ::core::option::Option<i64>,
    /// Projected cost in micros.
    #[prost(message, optional, tag = "5")]
    pub cost_micros: ::core::option::Option<i64>,
    /// Projected number of impressions.
    #[prost(message, optional, tag = "6")]
    pub impressions: ::core::option::Option<i64>,
    /// Projected number of top slot impressions.
    /// Only search advertising channel type supports this field.
    #[prost(message, optional, tag = "7")]
    pub top_slot_impressions: ::core::option::Option<i64>,
}
/// Projected metrics for a specific CPV bid amount.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CpvBidSimulationPoint {
    /// The simulated CPV bid upon which projected metrics are based.
    #[prost(message, optional, tag = "1")]
    pub cpv_bid_micros: ::core::option::Option<i64>,
    /// Projected cost in micros.
    #[prost(message, optional, tag = "2")]
    pub cost_micros: ::core::option::Option<i64>,
    /// Projected number of impressions.
    #[prost(message, optional, tag = "3")]
    pub impressions: ::core::option::Option<i64>,
    /// Projected number of views.
    #[prost(message, optional, tag = "4")]
    pub views: ::core::option::Option<i64>,
}
/// Projected metrics for a specific target CPA amount.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TargetCpaSimulationPoint {
    /// The simulated target CPA upon which projected metrics are based.
    #[prost(message, optional, tag = "1")]
    pub target_cpa_micros: ::core::option::Option<i64>,
    /// Projected number of biddable conversions.
    #[prost(message, optional, tag = "2")]
    pub biddable_conversions: ::core::option::Option<f64>,
    /// Projected total value of biddable conversions.
    #[prost(message, optional, tag = "3")]
    pub biddable_conversions_value: ::core::option::Option<f64>,
    /// Projected number of clicks.
    #[prost(message, optional, tag = "4")]
    pub clicks: ::core::option::Option<i64>,
    /// Projected cost in micros.
    #[prost(message, optional, tag = "5")]
    pub cost_micros: ::core::option::Option<i64>,
    /// Projected number of impressions.
    #[prost(message, optional, tag = "6")]
    pub impressions: ::core::option::Option<i64>,
    /// Projected number of top slot impressions.
    /// Only search advertising channel type supports this field.
    #[prost(message, optional, tag = "7")]
    pub top_slot_impressions: ::core::option::Option<i64>,
}
/// Projected metrics for a specific target ROAS amount.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TargetRoasSimulationPoint {
    /// The simulated target ROAS upon which projected metrics are based.
    #[prost(message, optional, tag = "1")]
    pub target_roas: ::core::option::Option<f64>,
    /// Projected number of biddable conversions.
    #[prost(message, optional, tag = "2")]
    pub biddable_conversions: ::core::option::Option<f64>,
    /// Projected total value of biddable conversions.
    #[prost(message, optional, tag = "3")]
    pub biddable_conversions_value: ::core::option::Option<f64>,
    /// Projected number of clicks.
    #[prost(message, optional, tag = "4")]
    pub clicks: ::core::option::Option<i64>,
    /// Projected cost in micros.
    #[prost(message, optional, tag = "5")]
    pub cost_micros: ::core::option::Option<i64>,
    /// Projected number of impressions.
    #[prost(message, optional, tag = "6")]
    pub impressions: ::core::option::Option<i64>,
    /// Projected number of top slot impressions.
    /// Only Search advertising channel type supports this field.
    #[prost(message, optional, tag = "7")]
    pub top_slot_impressions: ::core::option::Option<i64>,
}
// Proto file describing TagSnippet

/// The site tag and event snippet pair for a TrackingCodeType.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TagSnippet {
    /// The type of the generated tag snippets for tracking conversions.
    #[prost(
        enumeration = "super::enums::tracking_code_type_enum::TrackingCodeType",
        tag = "1"
    )]
    pub r#type: i32,
    /// The format of the web page where the tracking tag and snippet will be
    /// installed, e.g. HTML.
    #[prost(
        enumeration = "super::enums::tracking_code_page_format_enum::TrackingCodePageFormat",
        tag = "2"
    )]
    pub page_format: i32,
    /// The site tag that adds visitors to your basic remarketing lists and sets
    /// new cookies on your domain.
    #[prost(message, optional, tag = "3")]
    pub global_site_tag: ::core::option::Option<::prost::alloc::string::String>,
    /// The event snippet that works with the site tag to track actions that
    /// should be counted as conversions.
    #[prost(message, optional, tag = "4")]
    pub event_snippet: ::core::option::Option<::prost::alloc::string::String>,
}
// Proto file describing TargetingSetting

/// Settings for the targeting-related features, at the campaign and ad group
/// levels. For more details about the targeting setting, visit
/// https://support.google.com/google-ads/answer/7365594
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TargetingSetting {
    /// The per-targeting-dimension setting to restrict the reach of your campaign
    /// or ad group.
    #[prost(message, repeated, tag = "1")]
    pub target_restrictions: ::prost::alloc::vec::Vec<TargetRestriction>,
    /// The list of operations changing the target restrictions.
    ///
    /// Adding a target restriction with a targeting dimension that already exists
    /// causes the existing target restriction to be replaced with the new value.
    #[prost(message, repeated, tag = "2")]
    pub target_restriction_operations: ::prost::alloc::vec::Vec<TargetRestrictionOperation>,
}
/// The list of per-targeting-dimension targeting settings.
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
    #[prost(message, optional, tag = "2")]
    pub bid_only: ::core::option::Option<bool>,
}
/// Operation to be performed on a target restriction list in a mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TargetRestrictionOperation {
    /// Type of list operation to perform.
    #[prost(enumeration = "target_restriction_operation::Operator", tag = "1")]
    pub operator: i32,
    /// The target restriction being added to or removed from the list.
    #[prost(message, optional, tag = "2")]
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
}
/// A type of label displaying text on a colored background.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TextLabel {
    /// Background color of the label in RGB format. This string must match the
    /// regular expression '^\#([a-fA-F0-9]{6}|[a-fA-F0-9]{3})$'.
    /// Note: The background color may not be visible for manager accounts.
    #[prost(message, optional, tag = "1")]
    pub background_color: ::core::option::Option<::prost::alloc::string::String>,
    /// A short description of the label. The length must be no more than 200
    /// characters.
    #[prost(message, optional, tag = "2")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
}
// Proto file UrlCollection type.

/// Collection of urls that is tagged with a unique identifier.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UrlCollection {
    /// Unique identifier for this UrlCollection instance.
    #[prost(message, optional, tag = "1")]
    pub url_collection_id: ::core::option::Option<::prost::alloc::string::String>,
    /// A list of possible final URLs.
    #[prost(message, repeated, tag = "2")]
    pub final_urls: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// A list of possible final mobile URLs.
    #[prost(message, repeated, tag = "3")]
    pub final_mobile_urls: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// URL template for constructing a tracking URL.
    #[prost(message, optional, tag = "4")]
    pub tracking_url_template: ::core::option::Option<::prost::alloc::string::String>,
}
// Proto file describing user list types.

/// SimilarUserList is a list of users which are similar to users from another
/// UserList. These lists are read-only and automatically created by Google.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SimilarUserListInfo {
    /// Seed UserList from which this list is derived.
    #[prost(message, optional, tag = "1")]
    pub seed_user_list: ::core::option::Option<::prost::alloc::string::String>,
}
/// UserList of CRM users provided by the advertiser.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CrmBasedUserListInfo {
    /// A string that uniquely identifies a mobile application from which the data
    /// was collected to the Google Ads API.
    /// For iOS, the ID string is the 9 digit string that appears at the end of an
    /// App Store URL (e.g., "476943146" for "Flood-It! 2" whose App Store link is
    /// http://itunes.apple.com/us/app/flood-it!-2/id476943146).
    /// For Android, the ID string is the application's package name
    /// (e.g., "com.labpixies.colordrips" for "Color Drips" given Google Play link
    /// https://play.google.com/store/apps/details?id=com.labpixies.colordrips).
    /// Required when creating CrmBasedUserList for uploading mobile advertising
    /// IDs.
    #[prost(message, optional, tag = "1")]
    pub app_id: ::core::option::Option<::prost::alloc::string::String>,
    /// Matching key type of the list.
    /// Mixed data types are not allowed on the same list.
    /// This field is required for an ADD operation.
    #[prost(
        enumeration = "super::enums::customer_match_upload_key_type_enum::CustomerMatchUploadKeyType",
        tag = "2"
    )]
    pub upload_key_type: i32,
    /// Data source of the list. Default value is FIRST_PARTY.
    /// Only customers on the allow-list can create third-party sourced CRM lists.
    #[prost(
        enumeration = "super::enums::user_list_crm_data_source_type_enum::UserListCrmDataSourceType",
        tag = "3"
    )]
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
    #[prost(
        enumeration = "super::enums::user_list_rule_type_enum::UserListRuleType",
        tag = "1"
    )]
    pub rule_type: i32,
    /// List of rule item groups that defines this rule.
    /// Rule item groups are grouped together based on rule_type.
    #[prost(message, repeated, tag = "2")]
    pub rule_item_groups: ::prost::alloc::vec::Vec<UserListRuleItemGroupInfo>,
}
/// A group of rule items.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserListRuleItemGroupInfo {
    /// Rule items that will be grouped together based on rule_type.
    #[prost(message, repeated, tag = "1")]
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
    #[prost(message, optional, tag = "1")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    /// An atomic rule item.
    #[prost(oneof = "user_list_rule_item_info::RuleItem", tags = "2, 3, 4")]
    pub rule_item: ::core::option::Option<user_list_rule_item_info::RuleItem>,
}
/// Nested message and enum types in `UserListRuleItemInfo`.
pub mod user_list_rule_item_info {
    /// An atomic rule item.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum RuleItem {
        /// An atomic rule item composed of a number operation.
        #[prost(message, tag = "2")]
        NumberRuleItem(super::UserListNumberRuleItemInfo),
        /// An atomic rule item composed of a string operation.
        #[prost(message, tag = "3")]
        StringRuleItem(super::UserListStringRuleItemInfo),
        /// An atomic rule item composed of a date operation.
        #[prost(message, tag = "4")]
        DateRuleItem(super::UserListDateRuleItemInfo),
    }
}
/// A rule item composed of a date operation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserListDateRuleItemInfo {
    /// Date comparison operator.
    /// This field is required and must be populated when creating new date
    /// rule item.
    #[prost(
        enumeration = "super::enums::user_list_date_rule_item_operator_enum::UserListDateRuleItemOperator",
        tag = "1"
    )]
    pub operator: i32,
    /// String representing date value to be compared with the rule variable.
    /// Supported date format is YYYY-MM-DD.
    /// Times are reported in the customer's time zone.
    #[prost(message, optional, tag = "2")]
    pub value: ::core::option::Option<::prost::alloc::string::String>,
    /// The relative date value of the right hand side denoted by number of days
    /// offset from now. The value field will override this field when both are
    /// present.
    #[prost(message, optional, tag = "3")]
    pub offset_in_days: ::core::option::Option<i64>,
}
/// A rule item composed of a number operation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserListNumberRuleItemInfo {
    /// Number comparison operator.
    /// This field is required and must be populated when creating a new number
    /// rule item.
    #[prost(
        enumeration = "super::enums::user_list_number_rule_item_operator_enum::UserListNumberRuleItemOperator",
        tag = "1"
    )]
    pub operator: i32,
    /// Number value to be compared with the variable.
    /// This field is required and must be populated when creating a new number
    /// rule item.
    #[prost(message, optional, tag = "2")]
    pub value: ::core::option::Option<f64>,
}
/// A rule item composed of a string operation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserListStringRuleItemInfo {
    /// String comparison operator.
    /// This field is required and must be populated when creating a new string
    /// rule item.
    #[prost(
        enumeration = "super::enums::user_list_string_rule_item_operator_enum::UserListStringRuleItemOperator",
        tag = "1"
    )]
    pub operator: i32,
    /// The right hand side of the string rule item. For URLs or referrer URLs,
    /// the value can not contain illegal URL chars such as newlines, quotes,
    /// tabs, or parentheses. This field is required and must be populated when
    /// creating a new string rule item.
    #[prost(message, optional, tag = "2")]
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
    #[prost(message, optional, tag = "1")]
    pub left_operand: ::core::option::Option<UserListRuleInfo>,
    /// Right operand of the combined rule.
    /// This field is required and must be populated when creating new combined
    /// rule based user list.
    #[prost(message, optional, tag = "2")]
    pub right_operand: ::core::option::Option<UserListRuleInfo>,
    /// Operator to connect the two operands.
    ///
    /// Required for creating a combined rule user list.
    #[prost(
        enumeration = "super::enums::user_list_combined_rule_operator_enum::UserListCombinedRuleOperator",
        tag = "3"
    )]
    pub rule_operator: i32,
}
/// Visitors of a page during specific dates.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DateSpecificRuleUserListInfo {
    /// Boolean rule that defines visitor of a page.
    ///
    /// Required for creating a date specific rule user list.
    #[prost(message, optional, tag = "1")]
    pub rule: ::core::option::Option<UserListRuleInfo>,
    /// Start date of users visit. If set to 2000-01-01, then the list includes all
    /// users before end_date. The date's format should be YYYY-MM-DD.
    ///
    /// Required for creating a data specific rule user list.
    #[prost(message, optional, tag = "2")]
    pub start_date: ::core::option::Option<::prost::alloc::string::String>,
    /// End date of users visit. If set to 2037-12-30, then the list includes all
    /// users after start_date. The date's format should be YYYY-MM-DD.
    ///
    /// Required for creating a data specific rule user list.
    #[prost(message, optional, tag = "3")]
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
    #[prost(message, optional, tag = "1")]
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
    #[prost(
        enumeration = "super::enums::user_list_prepopulation_status_enum::UserListPrepopulationStatus",
        tag = "1"
    )]
    pub prepopulation_status: i32,
    /// Subtypes of rule based user lists.
    #[prost(
        oneof = "rule_based_user_list_info::RuleBasedUserList",
        tags = "2, 3, 4"
    )]
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
        #[prost(message, tag = "2")]
        CombinedRuleUserList(super::CombinedRuleUserListInfo),
        /// Visitors of a page during specific dates. The visiting periods are
        /// defined as follows:
        /// Between start_date (inclusive) and end_date (inclusive);
        /// Before end_date (exclusive) with start_date = 2000-01-01;
        /// After start_date (exclusive) with end_date = 2037-12-30.
        #[prost(message, tag = "3")]
        DateSpecificRuleUserList(super::DateSpecificRuleUserListInfo),
        /// Visitors of a page. The page visit is defined by one boolean rule
        /// expression.
        #[prost(message, tag = "4")]
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
    #[prost(message, repeated, tag = "1")]
    pub rules: ::prost::alloc::vec::Vec<UserListLogicalRuleInfo>,
}
/// A user list logical rule. A rule has a logical operator (and/or/not) and a
/// list of user lists as operands.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserListLogicalRuleInfo {
    /// The logical operator of the rule.
    #[prost(
        enumeration = "super::enums::user_list_logical_rule_operator_enum::UserListLogicalRuleOperator",
        tag = "1"
    )]
    pub operator: i32,
    /// The list of operands of the rule.
    #[prost(message, repeated, tag = "2")]
    pub rule_operands: ::prost::alloc::vec::Vec<LogicalUserListOperandInfo>,
}
/// Operand of logical user list that consists of a user list.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LogicalUserListOperandInfo {
    /// Resource name of a user list as an operand.
    #[prost(message, optional, tag = "1")]
    pub user_list: ::core::option::Option<::prost::alloc::string::String>,
}
/// User list targeting as a collection of conversions or remarketing actions.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BasicUserListInfo {
    /// Actions associated with this user list.
    #[prost(message, repeated, tag = "1")]
    pub actions: ::prost::alloc::vec::Vec<UserListActionInfo>,
}
/// Represents an action type used for building remarketing user lists.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserListActionInfo {
    /// Subtypes of user list action.
    #[prost(oneof = "user_list_action_info::UserListAction", tags = "1, 2")]
    pub user_list_action: ::core::option::Option<user_list_action_info::UserListAction>,
}
/// Nested message and enum types in `UserListActionInfo`.
pub mod user_list_action_info {
    /// Subtypes of user list action.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum UserListAction {
        /// A conversion action that's not generated from remarketing.
        #[prost(message, tag = "1")]
        ConversionAction(::prost::alloc::string::String),
        /// A remarketing action.
        #[prost(message, tag = "2")]
        RemarketingAction(::prost::alloc::string::String),
    }
}
// Proto file describing value types

/// A generic data container.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Value {
    /// A value.
    #[prost(oneof = "value::Value", tags = "1, 2, 3, 4, 5")]
    pub value: ::core::option::Option<value::Value>,
}
/// Nested message and enum types in `Value`.
pub mod value {
    /// A value.
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
