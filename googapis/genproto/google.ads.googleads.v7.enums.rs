// Proto file describing policy topic entry types.

/// Container for enum describing possible policy topic entry types.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PolicyTopicEntryTypeEnum {}
/// Nested message and enum types in `PolicyTopicEntryTypeEnum`.
pub mod policy_topic_entry_type_enum {
    /// The possible policy topic entry types.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum PolicyTopicEntryType {
        /// No value has been specified.
        Unspecified = 0,
        /// The received value is not known in this version.
        ///
        /// This is a response-only value.
        Unknown = 1,
        /// The resource will not be served.
        Prohibited = 2,
        /// The resource will not be served under some circumstances.
        Limited = 4,
        /// The resource cannot serve at all because of the current targeting
        /// criteria.
        FullyLimited = 8,
        /// May be of interest, but does not limit how the resource is served.
        Descriptive = 5,
        /// Could increase coverage beyond normal.
        Broadening = 6,
        /// Constrained for all targeted countries, but may serve in other countries
        /// through area of interest.
        AreaOfInterestOnly = 7,
    }
}
// Proto file describing policy topic evidence destination mismatch url types.

/// Container for enum describing possible policy topic evidence destination
/// mismatch url types.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PolicyTopicEvidenceDestinationMismatchUrlTypeEnum {}
/// Nested message and enum types in `PolicyTopicEvidenceDestinationMismatchUrlTypeEnum`.
pub mod policy_topic_evidence_destination_mismatch_url_type_enum {
    /// The possible policy topic evidence destination mismatch url types.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum PolicyTopicEvidenceDestinationMismatchUrlType {
        /// No value has been specified.
        Unspecified = 0,
        /// The received value is not known in this version.
        ///
        /// This is a response-only value.
        Unknown = 1,
        /// The display url.
        DisplayUrl = 2,
        /// The final url.
        FinalUrl = 3,
        /// The final mobile url.
        FinalMobileUrl = 4,
        /// The tracking url template, with substituted desktop url.
        TrackingUrl = 5,
        /// The tracking url template, with substituted mobile url.
        MobileTrackingUrl = 6,
    }
}
// Proto file describing device of destination not working policy topic
// evidence.

/// Container for enum describing possible policy topic evidence destination not
/// working devices.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PolicyTopicEvidenceDestinationNotWorkingDeviceEnum {}
/// Nested message and enum types in `PolicyTopicEvidenceDestinationNotWorkingDeviceEnum`.
pub mod policy_topic_evidence_destination_not_working_device_enum {
    /// The possible policy topic evidence destination not working devices.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum PolicyTopicEvidenceDestinationNotWorkingDevice {
        /// No value has been specified.
        Unspecified = 0,
        /// The received value is not known in this version.
        ///
        /// This is a response-only value.
        Unknown = 1,
        /// Landing page doesn't work on desktop device.
        Desktop = 2,
        /// Landing page doesn't work on Android device.
        Android = 3,
        /// Landing page doesn't work on iOS device.
        Ios = 4,
    }
}
// Proto file describing DNS error types of destination not working policy topic
// evidence.

/// Container for enum describing possible policy topic evidence destination not
/// working DNS error types.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PolicyTopicEvidenceDestinationNotWorkingDnsErrorTypeEnum {}
/// Nested message and enum types in `PolicyTopicEvidenceDestinationNotWorkingDnsErrorTypeEnum`.
pub mod policy_topic_evidence_destination_not_working_dns_error_type_enum {
    /// The possible policy topic evidence destination not working DNS error types.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum PolicyTopicEvidenceDestinationNotWorkingDnsErrorType {
        /// No value has been specified.
        Unspecified = 0,
        /// The received value is not known in this version.
        ///
        /// This is a response-only value.
        Unknown = 1,
        /// Host name not found in DNS when fetching landing page.
        HostnameNotFound = 2,
        /// Google internal crawler issue when communicating with DNS. This error
        /// doesn't mean the landing page doesn't work. Google will recrawl the
        /// landing page.
        GoogleCrawlerDnsIssue = 3,
    }
}
// Proto file describing policy approval statuses.

/// Container for enum describing possible policy approval statuses.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PolicyApprovalStatusEnum {}
/// Nested message and enum types in `PolicyApprovalStatusEnum`.
pub mod policy_approval_status_enum {
    /// The possible policy approval statuses. When there are several approval
    /// statuses available the most severe one will be used. The order of severity
    /// is DISAPPROVED, AREA_OF_INTEREST_ONLY, APPROVED_LIMITED and APPROVED.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum PolicyApprovalStatus {
        /// No value has been specified.
        Unspecified = 0,
        /// The received value is not known in this version.
        ///
        /// This is a response-only value.
        Unknown = 1,
        /// Will not serve.
        Disapproved = 2,
        /// Serves with restrictions.
        ApprovedLimited = 3,
        /// Serves without restrictions.
        Approved = 4,
        /// Will not serve in targeted countries, but may serve for users who are
        /// searching for information about the targeted countries.
        AreaOfInterestOnly = 5,
    }
}
// Proto file describing policy review statuses.

/// Container for enum describing possible policy review statuses.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PolicyReviewStatusEnum {}
/// Nested message and enum types in `PolicyReviewStatusEnum`.
pub mod policy_review_status_enum {
    /// The possible policy review statuses.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum PolicyReviewStatus {
        /// No value has been specified.
        Unspecified = 0,
        /// The received value is not known in this version.
        ///
        /// This is a response-only value.
        Unknown = 1,
        /// Currently under review.
        ReviewInProgress = 2,
        /// Primary review complete. Other reviews may be continuing.
        Reviewed = 3,
        /// The resource has been resubmitted for approval or its policy decision has
        /// been appealed.
        UnderAppeal = 4,
        /// The resource is eligible and may be serving but could still undergo
        /// further review.
        EligibleMayServe = 5,
    }
}
// Proto file describing the performance label of an asset.

/// Container for enum describing the performance label of an asset.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetPerformanceLabelEnum {}
/// Nested message and enum types in `AssetPerformanceLabelEnum`.
pub mod asset_performance_label_enum {
    /// Enum describing the possible performance labels of an asset, usually
    /// computed in the context of a linkage.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum AssetPerformanceLabel {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// This asset does not yet have any performance informantion. This may be
        /// because it is still under review.
        Pending = 2,
        /// The asset has started getting impressions but the stats are not
        /// statistically significant enough to get an asset performance label.
        Learning = 3,
        /// Worst performing assets.
        Low = 4,
        /// Good performing assets.
        Good = 5,
        /// Best performing assets.
        Best = 6,
    }
}
// Proto file describing policy review statuses.

/// Container for enum describing possible asset field types.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServedAssetFieldTypeEnum {}
/// Nested message and enum types in `ServedAssetFieldTypeEnum`.
pub mod served_asset_field_type_enum {
    /// The possible asset field types.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ServedAssetFieldType {
        /// No value has been specified.
        Unspecified = 0,
        /// The received value is not known in this version.
        ///
        /// This is a response-only value.
        Unknown = 1,
        /// The asset is used in headline 1.
        Headline1 = 2,
        /// The asset is used in headline 2.
        Headline2 = 3,
        /// The asset is used in headline 3.
        Headline3 = 4,
        /// The asset is used in description 1.
        Description1 = 5,
        /// The asset is used in description 2.
        Description2 = 6,
    }
}
// Proto file describing call conversion reporting state.

/// Container for enum describing possible data types for call conversion
/// reporting state.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CallConversionReportingStateEnum {}
/// Nested message and enum types in `CallConversionReportingStateEnum`.
pub mod call_conversion_reporting_state_enum {
    /// Possible data types for a call conversion action state.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum CallConversionReportingState {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Call conversion action is disabled.
        Disabled = 2,
        /// Call conversion action will use call conversion type set at the
        /// account level.
        UseAccountLevelCallConversionAction = 3,
        /// Call conversion action will use call conversion type set at the resource
        /// (call only ads/call extensions) level.
        UseResourceLevelCallConversionAction = 4,
    }
}
// Proto file describing display ad format settings.

/// Container for display ad format settings.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DisplayAdFormatSettingEnum {}
/// Nested message and enum types in `DisplayAdFormatSettingEnum`.
pub mod display_ad_format_setting_enum {
    /// Enumerates display ad format settings.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum DisplayAdFormatSetting {
        /// Not specified.
        Unspecified = 0,
        /// The value is unknown in this version.
        Unknown = 1,
        /// Text, image and native formats.
        AllFormats = 2,
        /// Text and image formats.
        NonNative = 3,
        /// Native format, i.e. the format rendering is controlled by the publisher
        /// and not by Google.
        Native = 4,
    }
}
// Proto file describing display upload product types.

/// Container for display upload product types. Product types that have the word
/// "DYNAMIC" in them must be associated with a campaign that has a dynamic
/// remarketing feed. See <https://support.google.com/google-ads/answer/6053288>
/// for more info about dynamic remarketing. Other product types are regarded
/// as "static" and do not have this requirement.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DisplayUploadProductTypeEnum {}
/// Nested message and enum types in `DisplayUploadProductTypeEnum`.
pub mod display_upload_product_type_enum {
    /// Enumerates display upload product types.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum DisplayUploadProductType {
        /// Not specified.
        Unspecified = 0,
        /// The value is unknown in this version.
        Unknown = 1,
        /// HTML5 upload ad. This product type requires the upload_media_bundle
        /// field in DisplayUploadAdInfo to be set.
        Html5UploadAd = 2,
        /// Dynamic HTML5 education ad. This product type requires the
        /// upload_media_bundle field in DisplayUploadAdInfo to be set. Can only be
        /// used in an education campaign.
        DynamicHtml5EducationAd = 3,
        /// Dynamic HTML5 flight ad. This product type requires the
        /// upload_media_bundle field in DisplayUploadAdInfo to be set. Can only be
        /// used in a flight campaign.
        DynamicHtml5FlightAd = 4,
        /// Dynamic HTML5 hotel and rental ad. This product type requires the
        /// upload_media_bundle field in DisplayUploadAdInfo to be set. Can only be
        /// used in a hotel campaign.
        DynamicHtml5HotelRentalAd = 5,
        /// Dynamic HTML5 job ad. This product type requires the
        /// upload_media_bundle field in DisplayUploadAdInfo to be set. Can only be
        /// used in a job campaign.
        DynamicHtml5JobAd = 6,
        /// Dynamic HTML5 local ad. This product type requires the
        /// upload_media_bundle field in DisplayUploadAdInfo to be set. Can only be
        /// used in a local campaign.
        DynamicHtml5LocalAd = 7,
        /// Dynamic HTML5 real estate ad. This product type requires the
        /// upload_media_bundle field in DisplayUploadAdInfo to be set. Can only be
        /// used in a real estate campaign.
        DynamicHtml5RealEstateAd = 8,
        /// Dynamic HTML5 custom ad. This product type requires the
        /// upload_media_bundle field in DisplayUploadAdInfo to be set. Can only be
        /// used in a custom campaign.
        DynamicHtml5CustomAd = 9,
        /// Dynamic HTML5 travel ad. This product type requires the
        /// upload_media_bundle field in DisplayUploadAdInfo to be set. Can only be
        /// used in a travel campaign.
        DynamicHtml5TravelAd = 10,
        /// Dynamic HTML5 hotel ad. This product type requires the
        /// upload_media_bundle field in DisplayUploadAdInfo to be set. Can only be
        /// used in a hotel campaign.
        DynamicHtml5HotelAd = 11,
    }
}
// Proto file describing app store types for a legacy app install ad.

/// Container for enum describing app store type in a legacy app install ad.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LegacyAppInstallAdAppStoreEnum {}
/// Nested message and enum types in `LegacyAppInstallAdAppStoreEnum`.
pub mod legacy_app_install_ad_app_store_enum {
    /// App store type in a legacy app install ad.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum LegacyAppInstallAdAppStore {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Apple iTunes.
        AppleAppStore = 2,
        /// Google Play.
        GooglePlay = 3,
        /// Windows Store.
        WindowsStore = 4,
        /// Windows Phone Store.
        WindowsPhoneStore = 5,
        /// The app is hosted in a Chinese app store.
        CnAppStore = 6,
    }
}
// Proto file describing mime types.

/// Container for enum describing the mime types.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MimeTypeEnum {}
/// Nested message and enum types in `MimeTypeEnum`.
pub mod mime_type_enum {
    /// The mime type
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum MimeType {
        /// The mime type has not been specified.
        Unspecified = 0,
        /// The received value is not known in this version.
        ///
        /// This is a response-only value.
        Unknown = 1,
        /// MIME type of image/jpeg.
        ImageJpeg = 2,
        /// MIME type of image/gif.
        ImageGif = 3,
        /// MIME type of image/png.
        ImagePng = 4,
        /// MIME type of application/x-shockwave-flash.
        Flash = 5,
        /// MIME type of text/html.
        TextHtml = 6,
        /// MIME type of application/pdf.
        Pdf = 7,
        /// MIME type of application/msword.
        Msword = 8,
        /// MIME type of application/vnd.ms-excel.
        Msexcel = 9,
        /// MIME type of application/rtf.
        Rtf = 10,
        /// MIME type of audio/wav.
        AudioWav = 11,
        /// MIME type of audio/mp3.
        AudioMp3 = 12,
        /// MIME type of application/x-html5-ad-zip.
        Html5AdZip = 13,
    }
}
// Proto file describing age range types.

/// Container for enum describing the type of demographic age ranges.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AgeRangeTypeEnum {}
/// Nested message and enum types in `AgeRangeTypeEnum`.
pub mod age_range_type_enum {
    /// The type of demographic age ranges (e.g. between 18 and 24 years old).
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum AgeRangeType {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Between 18 and 24 years old.
        AgeRange1824 = 503001,
        /// Between 25 and 34 years old.
        AgeRange2534 = 503002,
        /// Between 35 and 44 years old.
        AgeRange3544 = 503003,
        /// Between 45 and 54 years old.
        AgeRange4554 = 503004,
        /// Between 55 and 64 years old.
        AgeRange5564 = 503005,
        /// 65 years old and beyond.
        AgeRange65Up = 503006,
        /// Undetermined age range.
        AgeRangeUndetermined = 503999,
    }
}
// Proto file describing criteria types.

/// Represents a criterion for targeting paid apps.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AppPaymentModelTypeEnum {}
/// Nested message and enum types in `AppPaymentModelTypeEnum`.
pub mod app_payment_model_type_enum {
    /// Enum describing possible app payment models.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum AppPaymentModelType {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Represents paid-for apps.
        Paid = 30,
    }
}
// Proto file describing content label types.

/// Container for enum describing content label types in ContentLabel.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContentLabelTypeEnum {}
/// Nested message and enum types in `ContentLabelTypeEnum`.
pub mod content_label_type_enum {
    /// Enum listing the content label types supported by ContentLabel criterion.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ContentLabelType {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Sexually suggestive content.
        SexuallySuggestive = 2,
        /// Below the fold placement.
        BelowTheFold = 3,
        /// Parked domain.
        ParkedDomain = 4,
        /// Juvenile, gross & bizarre content.
        Juvenile = 6,
        /// Profanity & rough language.
        Profanity = 7,
        /// Death & tragedy.
        Tragedy = 8,
        /// Video.
        Video = 9,
        /// Content rating: G.
        VideoRatingDvG = 10,
        /// Content rating: PG.
        VideoRatingDvPg = 11,
        /// Content rating: T.
        VideoRatingDvT = 12,
        /// Content rating: MA.
        VideoRatingDvMa = 13,
        /// Content rating: not yet rated.
        VideoNotYetRated = 14,
        /// Embedded video.
        EmbeddedVideo = 15,
        /// Live streaming video.
        LiveStreamingVideo = 16,
        /// Sensitive social issues.
        SocialIssues = 17,
    }
}
// Proto file describing days of week.

/// Container for enumeration of days of the week, e.g., "Monday".
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DayOfWeekEnum {}
/// Nested message and enum types in `DayOfWeekEnum`.
pub mod day_of_week_enum {
    /// Enumerates days of the week, e.g., "Monday".
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum DayOfWeek {
        /// Not specified.
        Unspecified = 0,
        /// The value is unknown in this version.
        Unknown = 1,
        /// Monday.
        Monday = 2,
        /// Tuesday.
        Tuesday = 3,
        /// Wednesday.
        Wednesday = 4,
        /// Thursday.
        Thursday = 5,
        /// Friday.
        Friday = 6,
        /// Saturday.
        Saturday = 7,
        /// Sunday.
        Sunday = 8,
    }
}
// Proto file describing devices.

/// Container for enumeration of Google Ads devices available for targeting.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeviceEnum {}
/// Nested message and enum types in `DeviceEnum`.
pub mod device_enum {
    /// Enumerates Google Ads devices available for targeting.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Device {
        /// Not specified.
        Unspecified = 0,
        /// The value is unknown in this version.
        Unknown = 1,
        /// Mobile devices with full browsers.
        Mobile = 2,
        /// Tablets with full browsers.
        Tablet = 3,
        /// Computers.
        Desktop = 4,
        /// Smart TVs and game consoles.
        ConnectedTv = 6,
        /// Other device types.
        Other = 5,
    }
}
// Proto file describing gender types.

/// Container for enum describing the type of demographic genders.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenderTypeEnum {}
/// Nested message and enum types in `GenderTypeEnum`.
pub mod gender_type_enum {
    /// The type of demographic genders (e.g. female).
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum GenderType {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Male.
        Male = 10,
        /// Female.
        Female = 11,
        /// Undetermined gender.
        Undetermined = 20,
    }
}
// Proto file describing hotel date selection types.

/// Container for enum describing possible hotel date selection types
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HotelDateSelectionTypeEnum {}
/// Nested message and enum types in `HotelDateSelectionTypeEnum`.
pub mod hotel_date_selection_type_enum {
    /// Enum describing possible hotel date selection types.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum HotelDateSelectionType {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Dates selected by default.
        DefaultSelection = 50,
        /// Dates selected by the user.
        UserSelected = 51,
    }
}
// Proto file describing income range types.

/// Container for enum describing the type of demographic income ranges.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IncomeRangeTypeEnum {}
/// Nested message and enum types in `IncomeRangeTypeEnum`.
pub mod income_range_type_enum {
    /// The type of demographic income ranges (e.g. between 0% to 50%).
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum IncomeRangeType {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// 0%-50%.
        IncomeRange050 = 510001,
        /// 50% to 60%.
        IncomeRange5060 = 510002,
        /// 60% to 70%.
        IncomeRange6070 = 510003,
        /// 70% to 80%.
        IncomeRange7080 = 510004,
        /// 80% to 90%.
        IncomeRange8090 = 510005,
        /// Greater than 90%.
        IncomeRange90Up = 510006,
        /// Undetermined income range.
        IncomeRangeUndetermined = 510000,
    }
}
// Proto file describing interaction types.

/// Container for enum describing possible interaction types.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InteractionTypeEnum {}
/// Nested message and enum types in `InteractionTypeEnum`.
pub mod interaction_type_enum {
    /// Enum describing possible interaction types.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum InteractionType {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Calls.
        Calls = 8000,
    }
}
// Proto file describing Keyword match types.

/// Message describing Keyword match types.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeywordMatchTypeEnum {}
/// Nested message and enum types in `KeywordMatchTypeEnum`.
pub mod keyword_match_type_enum {
    /// Possible Keyword match types.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum KeywordMatchType {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Exact match.
        Exact = 2,
        /// Phrase match.
        Phrase = 3,
        /// Broad match.
        Broad = 4,
    }
}
// Proto file describing listing groups.

/// Container for enum describing the type of the listing group.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListingGroupTypeEnum {}
/// Nested message and enum types in `ListingGroupTypeEnum`.
pub mod listing_group_type_enum {
    /// The type of the listing group.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ListingGroupType {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Subdivision of products along some listing dimension. These nodes
        /// are not used by serving to target listing entries, but is purely
        /// to define the structure of the tree.
        Subdivision = 2,
        /// Listing group unit that defines a bid.
        Unit = 3,
    }
}
// Proto file describing location group radius units.

/// Container for enum describing unit of radius in location group.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LocationGroupRadiusUnitsEnum {}
/// Nested message and enum types in `LocationGroupRadiusUnitsEnum`.
pub mod location_group_radius_units_enum {
    /// The unit of radius distance in location group (e.g. MILES)
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum LocationGroupRadiusUnits {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Meters
        Meters = 2,
        /// Miles
        Miles = 3,
        /// Milli Miles
        MilliMiles = 4,
    }
}
// Proto file describing days of week.

/// Container for enumeration of quarter-hours.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MinuteOfHourEnum {}
/// Nested message and enum types in `MinuteOfHourEnum`.
pub mod minute_of_hour_enum {
    /// Enumerates of quarter-hours. E.g. "FIFTEEN"
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum MinuteOfHour {
        /// Not specified.
        Unspecified = 0,
        /// The value is unknown in this version.
        Unknown = 1,
        /// Zero minutes past the hour.
        Zero = 2,
        /// Fifteen minutes past the hour.
        Fifteen = 3,
        /// Thirty minutes past the hour.
        Thirty = 4,
        /// Forty-five minutes past the hour.
        FortyFive = 5,
    }
}
// Proto file describing parenal status types.

/// Container for enum describing the type of demographic parental statuses.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ParentalStatusTypeEnum {}
/// Nested message and enum types in `ParentalStatusTypeEnum`.
pub mod parental_status_type_enum {
    /// The type of parental statuses (e.g. not a parent).
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ParentalStatusType {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Parent.
        Parent = 300,
        /// Not a parent.
        NotAParent = 301,
        /// Undetermined parental status.
        Undetermined = 302,
    }
}
// Proto file describing preferred content criterion type.

/// Container for enumeration of preferred content criterion type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PreferredContentTypeEnum {}
/// Nested message and enum types in `PreferredContentTypeEnum`.
pub mod preferred_content_type_enum {
    /// Enumerates preferred content criterion type.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum PreferredContentType {
        /// Not specified.
        Unspecified = 0,
        /// The value is unknown in this version.
        Unknown = 1,
        /// Represents top content on YouTube.
        YoutubeTopContent = 400,
    }
}
/// Level of a product bidding category.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProductBiddingCategoryLevelEnum {}
/// Nested message and enum types in `ProductBiddingCategoryLevelEnum`.
pub mod product_bidding_category_level_enum {
    /// Enum describing the level of the product bidding category.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ProductBiddingCategoryLevel {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Level 1.
        Level1 = 2,
        /// Level 2.
        Level2 = 3,
        /// Level 3.
        Level3 = 4,
        /// Level 4.
        Level4 = 5,
        /// Level 5.
        Level5 = 6,
    }
}
// Proto file describing bidding schemes.

/// Locality of a product offer.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProductChannelEnum {}
/// Nested message and enum types in `ProductChannelEnum`.
pub mod product_channel_enum {
    /// Enum describing the locality of a product offer.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ProductChannel {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// The item is sold online.
        Online = 2,
        /// The item is sold in local stores.
        Local = 3,
    }
}
// Proto file describing bidding schemes.

/// Availability of a product offer.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProductChannelExclusivityEnum {}
/// Nested message and enum types in `ProductChannelExclusivityEnum`.
pub mod product_channel_exclusivity_enum {
    /// Enum describing the availability of a product offer.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ProductChannelExclusivity {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// The item is sold through one channel only, either local stores or online
        /// as indicated by its ProductChannel.
        SingleChannel = 2,
        /// The item is matched to its online or local stores counterpart, indicating
        /// it is available for purchase in both ShoppingProductChannels.
        MultiChannel = 3,
    }
}
// Proto file describing bidding schemes.

/// Condition of a product offer.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProductConditionEnum {}
/// Nested message and enum types in `ProductConditionEnum`.
pub mod product_condition_enum {
    /// Enum describing the condition of a product offer.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ProductCondition {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// The product condition is new.
        New = 3,
        /// The product condition is refurbished.
        Refurbished = 4,
        /// The product condition is used.
        Used = 5,
    }
}
// Proto file describing product custom attributes.

/// Container for enum describing the index of the product custom attribute.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProductCustomAttributeIndexEnum {}
/// Nested message and enum types in `ProductCustomAttributeIndexEnum`.
pub mod product_custom_attribute_index_enum {
    /// The index of the product custom attribute.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ProductCustomAttributeIndex {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// First product custom attribute.
        Index0 = 7,
        /// Second product custom attribute.
        Index1 = 8,
        /// Third product custom attribute.
        Index2 = 9,
        /// Fourth product custom attribute.
        Index3 = 10,
        /// Fifth product custom attribute.
        Index4 = 11,
    }
}
// Proto file describing bidding schemes.

/// Level of the type of a product offer.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProductTypeLevelEnum {}
/// Nested message and enum types in `ProductTypeLevelEnum`.
pub mod product_type_level_enum {
    /// Enum describing the level of the type of a product offer.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ProductTypeLevel {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Level 1.
        Level1 = 7,
        /// Level 2.
        Level2 = 8,
        /// Level 3.
        Level3 = 9,
        /// Level 4.
        Level4 = 10,
        /// Level 5.
        Level5 = 11,
    }
}
// Proto file describing proximity radius units.

/// Container for enum describing unit of radius in proximity.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProximityRadiusUnitsEnum {}
/// Nested message and enum types in `ProximityRadiusUnitsEnum`.
pub mod proximity_radius_units_enum {
    /// The unit of radius distance in proximity (e.g. MILES)
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ProximityRadiusUnits {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Miles
        Miles = 2,
        /// Kilometers
        Kilometers = 3,
    }
}
// Proto file describing webpage condition operand.

/// Container for enum describing webpage condition operand in webpage criterion.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WebpageConditionOperandEnum {}
/// Nested message and enum types in `WebpageConditionOperandEnum`.
pub mod webpage_condition_operand_enum {
    /// The webpage condition operand in webpage criterion.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum WebpageConditionOperand {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Operand denoting a webpage URL targeting condition.
        Url = 2,
        /// Operand denoting a webpage category targeting condition.
        Category = 3,
        /// Operand denoting a webpage title targeting condition.
        PageTitle = 4,
        /// Operand denoting a webpage content targeting condition.
        PageContent = 5,
        /// Operand denoting a webpage custom label targeting condition.
        CustomLabel = 6,
    }
}
// Proto file describing webpage condition operator.

/// Container for enum describing webpage condition operator in webpage
/// criterion.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WebpageConditionOperatorEnum {}
/// Nested message and enum types in `WebpageConditionOperatorEnum`.
pub mod webpage_condition_operator_enum {
    /// The webpage condition operator in webpage criterion.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum WebpageConditionOperator {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// The argument web condition is equal to the compared web condition.
        Equals = 2,
        /// The argument web condition is part of the compared web condition.
        Contains = 3,
    }
}
/// Describes the type of call-to-action phrases in a lead form.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LeadFormCallToActionTypeEnum {}
/// Nested message and enum types in `LeadFormCallToActionTypeEnum`.
pub mod lead_form_call_to_action_type_enum {
    /// Enum describing the type of call-to-action phrases in a lead form.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum LeadFormCallToActionType {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Learn more.
        LearnMore = 2,
        /// Get quote.
        GetQuote = 3,
        /// Apply now.
        ApplyNow = 4,
        /// Sign Up.
        SignUp = 5,
        /// Contact us.
        ContactUs = 6,
        /// Subscribe.
        Subscribe = 7,
        /// Download.
        Download = 8,
        /// Book now.
        BookNow = 9,
        /// Get offer.
        GetOffer = 10,
        /// Register.
        Register = 11,
        /// Get info.
        GetInfo = 12,
        /// Request a demo.
        RequestDemo = 13,
        /// Join now.
        JoinNow = 14,
        /// Get started.
        GetStarted = 15,
    }
}
/// Describes the desired level of intent of generated leads.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LeadFormDesiredIntentEnum {}
/// Nested message and enum types in `LeadFormDesiredIntentEnum`.
pub mod lead_form_desired_intent_enum {
    /// Enum describing the desired level of intent of generated leads.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum LeadFormDesiredIntent {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Deliver more leads at a potentially lower quality.
        LowIntent = 2,
        /// Only leads with a high level of intent are desired.
        HighIntent = 3,
    }
}
/// Describes the input type of a lead form field.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LeadFormFieldUserInputTypeEnum {}
/// Nested message and enum types in `LeadFormFieldUserInputTypeEnum`.
pub mod lead_form_field_user_input_type_enum {
    /// Enum describing the input type of a lead form field.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum LeadFormFieldUserInputType {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// The user will be asked to fill in their given and family name. This field
        /// cannot be set at the same time as GIVEN_NAME or FAMILY_NAME.
        FullName = 2,
        /// The user will be asked to fill in their given name (first name). This
        /// field can not be set at the same time as FULL_NAME.
        GivenName = 6,
        /// The user will be asked to fill in their family name (last name). This
        /// field can not be set at the same time as FULL_NAME.
        FamilyName = 7,
        /// The user will be asked to fill in their email address.
        Email = 3,
        /// The user will be asked to fill in their phone number.
        PhoneNumber = 4,
        /// The user will be asked to fill in their zip code.
        PostalCode = 5,
        /// The user will be asked to fill in their city.
        City = 9,
        /// The user will be asked to fill in their region part of the address (e.g.
        /// state for US, province for Canada).
        Region = 10,
        /// The user will be asked to fill in their country.
        Country = 11,
        /// The user will be asked to fill in their work email address.
        WorkEmail = 12,
        /// The user will be asked to fill in their company name.
        CompanyName = 13,
        /// The user will be asked to fill in their work phone.
        WorkPhone = 14,
        /// The user will be asked to fill in their job title.
        JobTitle = 15,
        /// Question: "Which model are you interested in?"
        /// Category: "Auto"
        VehicleModel = 1001,
        /// Question: "Which type of vehicle are you interested in?"
        /// Category: "Auto"
        VehicleType = 1002,
        /// Question: "What is your preferred dealership?"
        /// Category: "Auto"
        PreferredDealership = 1003,
        /// Question: "When do you plan on purchasing a vehicle?"
        /// Category: "Auto"
        VehiclePurchaseTimeline = 1004,
        /// Question: "Do you own a vehicle?"
        /// Category: "Auto"
        VehicleOwnership = 1005,
        /// Question: "What vehicle ownership option are you interested in?"
        /// Category: "Auto"
        VehiclePaymentType = 1009,
        /// Question: "What type of vehicle condition are you interested in?"
        /// Category: "Auto"
        VehicleCondition = 1010,
        /// Question: "What size is your company?"
        /// Category: "Business"
        CompanySize = 1006,
        /// Question: "What is your annual sales volume?"
        /// Category: "Business"
        AnnualSales = 1007,
        /// Question: "How many years have you been in business?"
        /// Category: "Business"
        YearsInBusiness = 1008,
        /// Question: "What is your job department?"
        /// Category: "Business"
        JobDepartment = 1011,
        /// Question: "What is your job role?"
        /// Category: "Business"
        JobRole = 1012,
        /// Question: "Which program are you interested in?"
        /// Category: "Education"
        EducationProgram = 1013,
        /// Question: "Which course are you interested in?"
        /// Category: "Education"
        EducationCourse = 1014,
        /// Question: "Which product are you interested in?"
        /// Category: "General"
        Product = 1016,
        /// Question: "Which service are you interested in?"
        /// Category: "General"
        Service = 1017,
        /// Question: "Which offer are you interested in?"
        /// Category: "General"
        Offer = 1018,
        /// Question: "Which category are you interested in?"
        /// Category: "General"
        Category = 1019,
        /// Question: "What is your preferred method of contact?"
        /// Category: "General"
        PreferredContactMethod = 1020,
        /// Question: "What is your preferred location?"
        /// Category: "General"
        PreferredLocation = 1021,
        /// Question: "What is the best time to contact you?"
        /// Category: "General"
        PreferredContactTime = 1022,
        /// Question: "When are you looking to make a purchase?"
        /// Category: "General"
        PurchaseTimeline = 1023,
        /// Question: "How many years of work experience do you have?"
        /// Category: "Jobs"
        YearsOfExperience = 1048,
        /// Question: "What industry do you work in?"
        /// Category: "Jobs"
        JobIndustry = 1049,
        /// Question: "What is your highest level of education?"
        /// Category: "Jobs"
        LevelOfEducation = 1050,
        /// Question: "What type of property are you looking for?"
        /// Category: "Real Estate"
        PropertyType = 1024,
        /// Question: "What do you need a realtor's help with?"
        /// Category: "Real Estate"
        RealtorHelpGoal = 1025,
        /// Question: "What neighborhood are you interested in?"
        /// Category: "Real Estate"
        PropertyCommunity = 1026,
        /// Question: "What price range are you looking for?"
        /// Category: "Real Estate"
        PriceRange = 1027,
        /// Question: "How many bedrooms are you looking for?"
        /// Category: "Real Estate"
        NumberOfBedrooms = 1028,
        /// Question: "Are you looking for a fully furnished property?"
        /// Category: "Real Estate"
        FurnishedProperty = 1029,
        /// Question: "Are you looking for properties that allow pets?"
        /// Category: "Real Estate"
        PetsAllowedProperty = 1030,
        /// Question: "What is the next product you plan to purchase?"
        /// Category: "Retail"
        NextPlannedPurchase = 1031,
        /// Question: "Would you like to sign up for an event?"
        /// Category: "Retail"
        EventSignupInterest = 1033,
        /// Question: "Where are you interested in shopping?"
        /// Category: "Retail"
        PreferredShoppingPlaces = 1034,
        /// Question: "What is your favorite brand?"
        /// Category: "Retail"
        FavoriteBrand = 1035,
        /// Question: "Which type of valid commercial license do you have?"
        /// Category: "Transportation"
        TransportationCommercialLicenseType = 1036,
        /// Question: "Interested in booking an event?"
        /// Category: "Travel"
        EventBookingInterest = 1038,
        /// Question: "What is your destination country?"
        /// Category: "Travel"
        DestinationCountry = 1039,
        /// Question: "What is your destination city?"
        /// Category: "Travel"
        DestinationCity = 1040,
        /// Question: "What is your departure country?"
        /// Category: "Travel"
        DepartureCountry = 1041,
        /// Question: "What is your departure city?"
        /// Category: "Travel"
        DepartureCity = 1042,
        /// Question: "What is your departure date?"
        /// Category: "Travel"
        DepartureDate = 1043,
        /// Question: "What is your return date?"
        /// Category: "Travel"
        ReturnDate = 1044,
        /// Question: "How many people are you traveling with?"
        /// Category: "Travel"
        NumberOfTravelers = 1045,
        /// Question: "What is your travel budget?"
        /// Category: "Travel"
        TravelBudget = 1046,
        /// Question: "Where do you want to stay during your travel?"
        /// Category: "Travel"
        TravelAccommodation = 1047,
    }
}
/// Describes the type of post-submit call-to-action phrases for a lead form.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LeadFormPostSubmitCallToActionTypeEnum {}
/// Nested message and enum types in `LeadFormPostSubmitCallToActionTypeEnum`.
pub mod lead_form_post_submit_call_to_action_type_enum {
    /// Enum describing the type of post-submit call-to-action phrases for a lead
    /// form.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum LeadFormPostSubmitCallToActionType {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Visit site.
        VisitSite = 2,
        /// Download.
        Download = 3,
        /// Learn more.
        LearnMore = 4,
        /// Shop now.
        ShopNow = 5,
    }
}
// Proto file describing promotion extension discount modifier.

/// Container for enum describing possible a promotion extension
/// discount modifier.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PromotionExtensionDiscountModifierEnum {}
/// Nested message and enum types in `PromotionExtensionDiscountModifierEnum`.
pub mod promotion_extension_discount_modifier_enum {
    /// A promotion extension discount modifier.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum PromotionExtensionDiscountModifier {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// 'Up to'.
        UpTo = 2,
    }
}
// Proto file describing promotion extension occasion.

/// Container for enum describing a promotion extension occasion.
/// For more information about the occasions please check:
/// <https://support.google.com/google-ads/answer/7367521>
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PromotionExtensionOccasionEnum {}
/// Nested message and enum types in `PromotionExtensionOccasionEnum`.
pub mod promotion_extension_occasion_enum {
    /// A promotion extension occasion.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum PromotionExtensionOccasion {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// New Year's.
        NewYears = 2,
        /// Chinese New Year.
        ChineseNewYear = 3,
        /// Valentine's Day.
        ValentinesDay = 4,
        /// Easter.
        Easter = 5,
        /// Mother's Day.
        MothersDay = 6,
        /// Father's Day.
        FathersDay = 7,
        /// Labor Day.
        LaborDay = 8,
        /// Back To School.
        BackToSchool = 9,
        /// Halloween.
        Halloween = 10,
        /// Black Friday.
        BlackFriday = 11,
        /// Cyber Monday.
        CyberMonday = 12,
        /// Christmas.
        Christmas = 13,
        /// Boxing Day.
        BoxingDay = 14,
        /// Independence Day in any country.
        IndependenceDay = 15,
        /// National Day in any country.
        NationalDay = 16,
        /// End of any season.
        EndOfSeason = 17,
        /// Winter Sale.
        WinterSale = 18,
        /// Summer sale.
        SummerSale = 19,
        /// Fall Sale.
        FallSale = 20,
        /// Spring Sale.
        SpringSale = 21,
        /// Ramadan.
        Ramadan = 22,
        /// Eid al-Fitr.
        EidAlFitr = 23,
        /// Eid al-Adha.
        EidAlAdha = 24,
        /// Singles Day.
        SinglesDay = 25,
        /// Women's Day.
        WomensDay = 26,
        /// Holi.
        Holi = 27,
        /// Parent's Day.
        ParentsDay = 28,
        /// St. Nicholas Day.
        StNicholasDay = 29,
        /// Carnival.
        Carnival = 30,
        /// Epiphany, also known as Three Kings' Day.
        Epiphany = 31,
        /// Rosh Hashanah.
        RoshHashanah = 32,
        /// Passover.
        Passover = 33,
        /// Hanukkah.
        Hanukkah = 34,
        /// Diwali.
        Diwali = 35,
        /// Navratri.
        Navratri = 36,
        /// Available in Thai: Songkran.
        Songkran = 37,
        /// Available in Japanese: Year-end Gift.
        YearEndGift = 38,
    }
}
// Proto file describing target impression share goal.

/// Container for enum describing where on the first search results page the
/// automated bidding system should target impressions for the
/// TargetImpressionShare bidding strategy.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TargetImpressionShareLocationEnum {}
/// Nested message and enum types in `TargetImpressionShareLocationEnum`.
pub mod target_impression_share_location_enum {
    /// Enum describing possible goals.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum TargetImpressionShareLocation {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Any location on the web page.
        AnywhereOnPage = 2,
        /// Top box of ads.
        TopOfPage = 3,
        /// Top slot in the top box of ads.
        AbsoluteTopOfPage = 4,
    }
}
// Proto file describing advertising channel subtypes.

/// An immutable specialization of an Advertising Channel.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdvertisingChannelSubTypeEnum {}
/// Nested message and enum types in `AdvertisingChannelSubTypeEnum`.
pub mod advertising_channel_sub_type_enum {
    /// Enum describing the different channel subtypes.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum AdvertisingChannelSubType {
        /// Not specified.
        Unspecified = 0,
        /// Used as a return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Mobile app campaigns for Search.
        SearchMobileApp = 2,
        /// Mobile app campaigns for Display.
        DisplayMobileApp = 3,
        /// AdWords express campaigns for search.
        SearchExpress = 4,
        /// AdWords Express campaigns for display.
        DisplayExpress = 5,
        /// Smart Shopping campaigns.
        ShoppingSmartAds = 6,
        /// Gmail Ad campaigns.
        DisplayGmailAd = 7,
        /// Smart display campaigns.
        DisplaySmartCampaign = 8,
        /// Video Outstream campaigns.
        VideoOutstream = 9,
        /// Video TrueView for Action campaigns.
        VideoAction = 10,
        /// Video campaigns with non-skippable video ads.
        VideoNonSkippable = 11,
        /// App Campaign that allows you to easily promote your Android or iOS app
        /// across Google's top properties including Search, Play, YouTube, and the
        /// Google Display Network.
        AppCampaign = 12,
        /// App Campaign for engagement, focused on driving re-engagement with the
        /// app across several of Google’s top properties including Search, YouTube,
        /// and the Google Display Network.
        AppCampaignForEngagement = 13,
        /// Campaigns specialized for local advertising.
        LocalCampaign = 14,
        /// Shopping Comparison Listing campaigns.
        ShoppingComparisonListingAds = 15,
        /// Standard Smart campaigns.
        SmartCampaign = 16,
        /// Video campaigns with sequence video ads.
        VideoSequence = 17,
    }
}
// Proto file describing advertising channel types

/// The channel type a campaign may target to serve on.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdvertisingChannelTypeEnum {}
/// Nested message and enum types in `AdvertisingChannelTypeEnum`.
pub mod advertising_channel_type_enum {
    /// Enum describing the various advertising channel types.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum AdvertisingChannelType {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Search Network. Includes display bundled, and Search+ campaigns.
        Search = 2,
        /// Google Display Network only.
        Display = 3,
        /// Shopping campaigns serve on the shopping property
        /// and on google.com search results.
        Shopping = 4,
        /// Hotel Ads campaigns.
        Hotel = 5,
        /// Video campaigns.
        Video = 6,
        /// App Campaigns, and App Campaigns for Engagement, that run
        /// across multiple channels.
        MultiChannel = 7,
        /// Local ads campaigns.
        Local = 8,
        /// Smart campaigns.
        Smart = 9,
    }
}
// Proto file describing the criterion category channel availability mode.

/// Describes channel availability mode for a criterion availability - whether
/// the availability is meant to include all advertising channels, or a
/// particular channel with all its channel subtypes, or a channel with a certain
/// subset of channel subtypes.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CriterionCategoryChannelAvailabilityModeEnum {}
/// Nested message and enum types in `CriterionCategoryChannelAvailabilityModeEnum`.
pub mod criterion_category_channel_availability_mode_enum {
    /// Enum containing the possible CriterionCategoryChannelAvailabilityMode.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum CriterionCategoryChannelAvailabilityMode {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// The category is available to campaigns of all channel types and subtypes.
        AllChannels = 2,
        /// The category is available to campaigns of a specific channel type,
        /// including all subtypes under it.
        ChannelTypeAndAllSubtypes = 3,
        /// The category is available to campaigns of a specific channel type and
        /// subtype(s).
        ChannelTypeAndSubsetSubtypes = 4,
    }
}
// Proto file describing the criterion category locale availability mode.

/// Describes locale availability mode for a criterion availability - whether
/// it's available globally, or a particular country with all languages, or a
/// particular language with all countries, or a country-language pair.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CriterionCategoryLocaleAvailabilityModeEnum {}
/// Nested message and enum types in `CriterionCategoryLocaleAvailabilityModeEnum`.
pub mod criterion_category_locale_availability_mode_enum {
    /// Enum containing the possible CriterionCategoryLocaleAvailabilityMode.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum CriterionCategoryLocaleAvailabilityMode {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// The category is available to campaigns of all locales.
        AllLocales = 2,
        /// The category is available to campaigns within a list of countries,
        /// regardless of language.
        CountryAndAllLanguages = 3,
        /// The category is available to campaigns within a list of languages,
        /// regardless of country.
        LanguageAndAllCountries = 4,
        /// The category is available to campaigns within a list of country, language
        /// pairs.
        CountryAndLanguage = 5,
    }
}
// Proto file describing days of week.

/// Container for enumeration of months of the year, e.g., "January".
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MonthOfYearEnum {}
/// Nested message and enum types in `MonthOfYearEnum`.
pub mod month_of_year_enum {
    /// Enumerates months of the year, e.g., "January".
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum MonthOfYear {
        /// Not specified.
        Unspecified = 0,
        /// The value is unknown in this version.
        Unknown = 1,
        /// January.
        January = 2,
        /// February.
        February = 3,
        /// March.
        March = 4,
        /// April.
        April = 5,
        /// May.
        May = 6,
        /// June.
        June = 7,
        /// July.
        July = 8,
        /// August.
        August = 9,
        /// September.
        September = 10,
        /// October.
        October = 11,
        /// November.
        November = 12,
        /// December.
        December = 13,
    }
}
// Proto file describing app store types for an app extension.

/// Container for enum describing app store type in an app extension.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AppStoreEnum {}
/// Nested message and enum types in `AppStoreEnum`.
pub mod app_store_enum {
    /// App store type in an app extension.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum AppStore {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Apple iTunes.
        AppleItunes = 2,
        /// Google Play.
        GooglePlay = 3,
    }
}
// Proto file describing price extension price qualifier type.

/// Container for enum describing a price extension price qualifier.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PriceExtensionPriceQualifierEnum {}
/// Nested message and enum types in `PriceExtensionPriceQualifierEnum`.
pub mod price_extension_price_qualifier_enum {
    /// Enums of price extension price qualifier.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum PriceExtensionPriceQualifier {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// 'From' qualifier for the price.
        From = 2,
        /// 'Up to' qualifier for the price.
        UpTo = 3,
        /// 'Average' qualifier for the price.
        Average = 4,
    }
}
// Proto file describing price extension price unit.

/// Container for enum describing price extension price unit.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PriceExtensionPriceUnitEnum {}
/// Nested message and enum types in `PriceExtensionPriceUnitEnum`.
pub mod price_extension_price_unit_enum {
    /// Price extension price unit.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum PriceExtensionPriceUnit {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Per hour.
        PerHour = 2,
        /// Per day.
        PerDay = 3,
        /// Per week.
        PerWeek = 4,
        /// Per month.
        PerMonth = 5,
        /// Per year.
        PerYear = 6,
        /// Per night.
        PerNight = 7,
    }
}
// Proto file describing price extension type.

/// Container for enum describing types for a price extension.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PriceExtensionTypeEnum {}
/// Nested message and enum types in `PriceExtensionTypeEnum`.
pub mod price_extension_type_enum {
    /// Price extension type.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum PriceExtensionType {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// The type for showing a list of brands.
        Brands = 2,
        /// The type for showing a list of events.
        Events = 3,
        /// The type for showing locations relevant to your business.
        Locations = 4,
        /// The type for showing sub-regions or districts within a city or region.
        Neighborhoods = 5,
        /// The type for showing a collection of product categories.
        ProductCategories = 6,
        /// The type for showing a collection of related product tiers.
        ProductTiers = 7,
        /// The type for showing a collection of services offered by your business.
        Services = 8,
        /// The type for showing a collection of service categories.
        ServiceCategories = 9,
        /// The type for showing a collection of related service tiers.
        ServiceTiers = 10,
    }
}
/// The type of string matching to be used for a dynamic FeedItemSet filter.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeedItemSetStringFilterTypeEnum {}
/// Nested message and enum types in `FeedItemSetStringFilterTypeEnum`.
pub mod feed_item_set_string_filter_type_enum {
    /// describe the possible types for a FeedItemSetStringFilter.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum FeedItemSetStringFilterType {
        /// Not specified.
        Unspecified = 0,
        /// The received error code is not known in this version.
        Unknown = 1,
        /// The dynamic set filter will use exact string matching.
        Exact = 2,
    }
}
// Proto file describing operating system for a deeplink app URL.

/// The possible OS types for a deeplink AppUrl.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AppUrlOperatingSystemTypeEnum {}
/// Nested message and enum types in `AppUrlOperatingSystemTypeEnum`.
pub mod app_url_operating_system_type_enum {
    /// Operating System
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum AppUrlOperatingSystemType {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// The Apple IOS operating system.
        Ios = 2,
        /// The Android operating system.
        Android = 3,
    }
}
// Proto file describing frequency caps.

/// Container for enum describing the type of event that the cap applies to.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FrequencyCapEventTypeEnum {}
/// Nested message and enum types in `FrequencyCapEventTypeEnum`.
pub mod frequency_cap_event_type_enum {
    /// The type of event that the cap applies to (e.g. impression).
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum FrequencyCapEventType {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// The cap applies on ad impressions.
        Impression = 2,
        /// The cap applies on video ad views.
        VideoView = 3,
    }
}
// Proto file describing frequency caps.

/// Container for enum describing the level on which the cap is to be applied.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FrequencyCapLevelEnum {}
/// Nested message and enum types in `FrequencyCapLevelEnum`.
pub mod frequency_cap_level_enum {
    /// The level on which the cap is to be applied (e.g ad group ad, ad group).
    /// Cap is applied to all the resources of this level.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum FrequencyCapLevel {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// The cap is applied at the ad group ad level.
        AdGroupAd = 2,
        /// The cap is applied at the ad group level.
        AdGroup = 3,
        /// The cap is applied at the campaign level.
        Campaign = 4,
    }
}
// Proto file describing frequency caps.

/// Container for enum describing the unit of time the cap is defined at.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FrequencyCapTimeUnitEnum {}
/// Nested message and enum types in `FrequencyCapTimeUnitEnum`.
pub mod frequency_cap_time_unit_enum {
    /// Unit of time the cap is defined at (e.g. day, week).
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum FrequencyCapTimeUnit {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// The cap would define limit per one day.
        Day = 2,
        /// The cap would define limit per one week.
        Week = 3,
        /// The cap would define limit per one month.
        Month = 4,
    }
}
// Proto file describing keyword plan aggregate metric types.

/// The enumeration of keyword plan aggregate metric types.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeywordPlanAggregateMetricTypeEnum {}
/// Nested message and enum types in `KeywordPlanAggregateMetricTypeEnum`.
pub mod keyword_plan_aggregate_metric_type_enum {
    /// Aggregate fields.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum KeywordPlanAggregateMetricType {
        /// Not specified.
        Unspecified = 0,
        /// The value is unknown in this version.
        Unknown = 1,
        /// The device breakdown of aggregate search volume.
        Device = 2,
    }
}
// Proto file describing Keyword Planner competition levels.

/// Container for enumeration of keyword competition levels. The competition
/// level indicates how competitive ad placement is for a keyword and
/// is determined by the number of advertisers bidding on that keyword relative
/// to all keywords across Google. The competition level can depend on the
/// location and Search Network targeting options you've selected.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeywordPlanCompetitionLevelEnum {}
/// Nested message and enum types in `KeywordPlanCompetitionLevelEnum`.
pub mod keyword_plan_competition_level_enum {
    /// Competition level of a keyword.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum KeywordPlanCompetitionLevel {
        /// Not specified.
        Unspecified = 0,
        /// The value is unknown in this version.
        Unknown = 1,
        /// Low competition. The Competition Index range for this is [0, 33].
        Low = 2,
        /// Medium competition. The Competition Index range for this is [34, 66].
        Medium = 3,
        /// High competition. The Competition Index range for this is [67, 100].
        High = 4,
    }
}
// Proto file describing Keyword Planner Concept Group types.

/// Container for enumeration of keyword plan concept group types.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeywordPlanConceptGroupTypeEnum {}
/// Nested message and enum types in `KeywordPlanConceptGroupTypeEnum`.
pub mod keyword_plan_concept_group_type_enum {
    /// Enumerates keyword plan concept group types.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum KeywordPlanConceptGroupType {
        /// The concept group classification different from brand/non-brand.
        /// This is a catch all bucket for all classifications that are none of the
        /// below.
        Unspecified = 0,
        /// The value is unknown in this version.
        Unknown = 1,
        /// The concept group classification is based on BRAND.
        Brand = 2,
        /// The concept group classification based on BRAND, that didn't fit well
        /// with the BRAND classifications. These are generally outliers and can have
        /// very few keywords in this type of classification.
        OtherBrands = 3,
        /// These concept group classification is not based on BRAND. This is
        /// returned for generic keywords that don't have a brand association.
        NonBrand = 4,
    }
}
// Proto file describing matching function context types.

/// Container for context types for an operand in a matching function.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MatchingFunctionContextTypeEnum {}
/// Nested message and enum types in `MatchingFunctionContextTypeEnum`.
pub mod matching_function_context_type_enum {
    /// Possible context types for an operand in a matching function.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum MatchingFunctionContextType {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Feed item id in the request context.
        FeedItemId = 2,
        /// The device being used (possible values are 'Desktop' or 'Mobile').
        DeviceName = 3,
        /// Feed item set id in the request context.
        FeedItemSetId = 4,
    }
}
// Proto file describing matching function operators.

/// Container for enum describing matching function operator.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MatchingFunctionOperatorEnum {}
/// Nested message and enum types in `MatchingFunctionOperatorEnum`.
pub mod matching_function_operator_enum {
    /// Possible operators in a matching function.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum MatchingFunctionOperator {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// The IN operator.
        In = 2,
        /// The IDENTITY operator.
        Identity = 3,
        /// The EQUALS operator
        Equals = 4,
        /// Operator that takes two or more operands that are of type
        /// FunctionOperand and checks that all the operands evaluate to true.
        /// For functions related to ad formats, all the operands must be in
        /// left_operands.
        And = 5,
        /// Operator that returns true if the elements in left_operands contain any
        /// of the elements in right_operands. Otherwise, return false. The
        /// right_operands must contain at least 1 and no more than 3
        /// ConstantOperands.
        ContainsAny = 6,
    }
}
// Proto file describing types of payable and free interactions.

/// Container for enum describing types of payable and free interactions.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InteractionEventTypeEnum {}
/// Nested message and enum types in `InteractionEventTypeEnum`.
pub mod interaction_event_type_enum {
    /// Enum describing possible types of payable and free interactions.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum InteractionEventType {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Click to site. In most cases, this interaction navigates to an external
        /// location, usually the advertiser's landing page. This is also the default
        /// InteractionEventType for click events.
        Click = 2,
        /// The user's expressed intent to engage with the ad in-place.
        Engagement = 3,
        /// User viewed a video ad.
        VideoView = 4,
        /// The default InteractionEventType for ad conversion events.
        /// This is used when an ad conversion row does NOT indicate
        /// that the free interactions (i.e., the ad conversions)
        /// should be 'promoted' and reported as part of the core metrics.
        /// These are simply other (ad) conversions.
        None = 5,
    }
}
// Proto file describing quality score buckets.

/// The relative performance compared to other advertisers.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QualityScoreBucketEnum {}
/// Nested message and enum types in `QualityScoreBucketEnum`.
pub mod quality_score_bucket_enum {
    /// Enum listing the possible quality score buckets.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum QualityScoreBucket {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Quality of the creative is below average.
        BelowAverage = 2,
        /// Quality of the creative is average.
        Average = 3,
        /// Quality of the creative is above average.
        AboveAverage = 4,
    }
}
// Proto file describing user identifier source

/// Container for enum describing the source of the user identifier for offline
/// Store Sales third party uploads.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserIdentifierSourceEnum {}
/// Nested message and enum types in `UserIdentifierSourceEnum`.
pub mod user_identifier_source_enum {
    /// The type of user identifier source for offline Store Sales third party
    /// uploads.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum UserIdentifierSource {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version
        Unknown = 1,
        /// Indicates that the user identifier was provided by the first party
        /// (advertiser).
        FirstParty = 2,
        /// Indicates that the user identifier was provided by the third party
        /// (partner).
        ThirdParty = 3,
    }
}
// Proto file describing ad destination types.

/// Container for enumeration of Google Ads destination types.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdDestinationTypeEnum {}
/// Nested message and enum types in `AdDestinationTypeEnum`.
pub mod ad_destination_type_enum {
    /// Enumerates Google Ads destination types
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum AdDestinationType {
        /// Not specified.
        Unspecified = 0,
        /// The value is unknown in this version.
        Unknown = 1,
        /// Ads that don't intend to drive users off from ads to other destinations
        NotApplicable = 2,
        /// Website
        Website = 3,
        /// App Deep Link
        AppDeepLink = 4,
        /// iOS App Store or Play Store
        AppStore = 5,
        /// Call Dialer
        PhoneCall = 6,
        /// Map App
        MapDirections = 7,
        /// Location Dedicated Page
        LocationListing = 8,
        /// Text Message
        Message = 9,
        /// Lead Generation Form
        LeadForm = 10,
        /// YouTube
        Youtube = 11,
        /// Ad Destination for Conversions with keys unknown
        UnmodeledForConversions = 12,
    }
}
// Proto file describing ad network types.

/// Container for enumeration of Google Ads network types.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdNetworkTypeEnum {}
/// Nested message and enum types in `AdNetworkTypeEnum`.
pub mod ad_network_type_enum {
    /// Enumerates Google Ads network types.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum AdNetworkType {
        /// Not specified.
        Unspecified = 0,
        /// The value is unknown in this version.
        Unknown = 1,
        /// Google search.
        Search = 2,
        /// Search partners.
        SearchPartners = 3,
        /// Display Network.
        Content = 4,
        /// YouTube Search.
        YoutubeSearch = 5,
        /// YouTube Videos
        YoutubeWatch = 6,
        /// Cross-network.
        Mixed = 7,
    }
}
// Proto file describing Budget and Campaign association status.

/// Message describing the status of the association between the Budget and the
/// Campaign.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BudgetCampaignAssociationStatusEnum {}
/// Nested message and enum types in `BudgetCampaignAssociationStatusEnum`.
pub mod budget_campaign_association_status_enum {
    /// Possible statuses of the association between the Budget and the Campaign.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum BudgetCampaignAssociationStatus {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// The campaign is currently using the budget.
        Enabled = 2,
        /// The campaign is no longer using the budget.
        Removed = 3,
    }
}
// Proto file describing click types.

/// Container for enumeration of Google Ads click types.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClickTypeEnum {}
/// Nested message and enum types in `ClickTypeEnum`.
pub mod click_type_enum {
    /// Enumerates Google Ads click types.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ClickType {
        /// Not specified.
        Unspecified = 0,
        /// The value is unknown in this version.
        Unknown = 1,
        /// App engagement ad deep link.
        AppDeeplink = 2,
        /// Breadcrumbs.
        Breadcrumbs = 3,
        /// Broadband Plan.
        BroadbandPlan = 4,
        /// Manually dialed phone calls.
        CallTracking = 5,
        /// Phone calls.
        Calls = 6,
        /// Click on engagement ad.
        ClickOnEngagementAd = 7,
        /// Driving direction.
        GetDirections = 8,
        /// Get location details.
        LocationExpansion = 9,
        /// Call.
        LocationFormatCall = 10,
        /// Directions.
        LocationFormatDirections = 11,
        /// Image(s).
        LocationFormatImage = 12,
        /// Go to landing page.
        LocationFormatLandingPage = 13,
        /// Map.
        LocationFormatMap = 14,
        /// Go to store info.
        LocationFormatStoreInfo = 15,
        /// Text.
        LocationFormatText = 16,
        /// Mobile phone calls.
        MobileCallTracking = 17,
        /// Print offer.
        OfferPrints = 18,
        /// Other.
        Other = 19,
        /// Product plusbox offer.
        ProductExtensionClicks = 20,
        /// Shopping - Product - Online.
        ProductListingAdClicks = 21,
        /// Sitelink.
        Sitelinks = 22,
        /// Show nearby locations.
        StoreLocator = 23,
        /// Headline.
        UrlClicks = 25,
        /// App store.
        VideoAppStoreClicks = 26,
        /// Call-to-Action overlay.
        VideoCallToActionClicks = 27,
        /// Cards.
        VideoCardActionHeadlineClicks = 28,
        /// End cap.
        VideoEndCapClicks = 29,
        /// Website.
        VideoWebsiteClicks = 30,
        /// Visual Sitelinks.
        VisualSitelinks = 31,
        /// Wireless Plan.
        WirelessPlan = 32,
        /// Shopping - Product - Local.
        ProductListingAdLocal = 33,
        /// Shopping - Product - MultiChannel Local.
        ProductListingAdMultichannelLocal = 34,
        /// Shopping - Product - MultiChannel Online.
        ProductListingAdMultichannelOnline = 35,
        /// Shopping - Product - Coupon.
        ProductListingAdsCoupon = 36,
        /// Shopping - Product - Sell on Google.
        ProductListingAdTransactable = 37,
        /// Shopping - Product - App engagement ad deep link.
        ProductAdAppDeeplink = 38,
        /// Shopping - Showcase - Category.
        ShowcaseAdCategoryLink = 39,
        /// Shopping - Showcase - Local storefront.
        ShowcaseAdLocalStorefrontLink = 40,
        /// Shopping - Showcase - Online product.
        ShowcaseAdOnlineProductLink = 42,
        /// Shopping - Showcase - Local product.
        ShowcaseAdLocalProductLink = 43,
        /// Promotion Extension.
        PromotionExtension = 44,
        /// Ad Headline.
        SwipeableGalleryAdHeadline = 45,
        /// Swipes.
        SwipeableGalleryAdSwipes = 46,
        /// See More.
        SwipeableGalleryAdSeeMore = 47,
        /// Sitelink 1.
        SwipeableGalleryAdSitelinkOne = 48,
        /// Sitelink 2.
        SwipeableGalleryAdSitelinkTwo = 49,
        /// Sitelink 3.
        SwipeableGalleryAdSitelinkThree = 50,
        /// Sitelink 4.
        SwipeableGalleryAdSitelinkFour = 51,
        /// Sitelink 5.
        SwipeableGalleryAdSitelinkFive = 52,
        /// Hotel price.
        HotelPrice = 53,
        /// Price Extension.
        PriceExtension = 54,
        /// Book on Google hotel room selection.
        HotelBookOnGoogleRoomSelection = 55,
        /// Shopping - Comparison Listing.
        ShoppingComparisonListing = 56,
    }
}
/// Container for enum describing the category of conversions that are associated
/// with a ConversionAction.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConversionActionCategoryEnum {}
/// Nested message and enum types in `ConversionActionCategoryEnum`.
pub mod conversion_action_category_enum {
    /// The category of conversions that are associated with a ConversionAction.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ConversionActionCategory {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Default category.
        Default = 2,
        /// User visiting a page.
        PageView = 3,
        /// Purchase, sales, or "order placed" event.
        Purchase = 4,
        /// Signup user action.
        Signup = 5,
        /// Lead-generating action.
        Lead = 6,
        /// Software download action (as for an app).
        Download = 7,
        /// The addition of items to a shopping cart or bag on an advertiser site.
        AddToCart = 8,
        /// When someone enters the checkout flow on an advertiser site.
        BeginCheckout = 9,
        /// The start of a paid subscription for a product or service.
        SubscribePaid = 10,
        /// A call to indicate interest in an advertiser's offering.
        PhoneCallLead = 11,
        /// A lead conversion imported from an external source into Google Ads.
        ImportedLead = 12,
        /// A submission of a form on an advertiser site indicating business
        /// interest.
        SubmitLeadForm = 13,
        /// A booking of an appointment with an advertiser's business.
        BookAppointment = 14,
        /// A quote or price estimate request.
        RequestQuote = 15,
        /// A search for an advertiser's business location with intention to visit.
        GetDirections = 16,
        /// A click to an advertiser's partner's site.
        OutboundClick = 17,
        /// A call, SMS, email, chat or other type of contact to an advertiser.
        Contact = 18,
        /// A website engagement event such as long site time or a Google Analytics
        /// (GA) Smart Goal. Intended to be used for GA, Firebase, GA Gold goal
        /// imports.
        Engagement = 19,
        /// A visit to a physical store location.
        StoreVisit = 20,
        /// A sale occurring in a physical store.
        StoreSale = 21,
    }
}
/// Container for enum indicating the event type the conversion is attributed to.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConversionAttributionEventTypeEnum {}
/// Nested message and enum types in `ConversionAttributionEventTypeEnum`.
pub mod conversion_attribution_event_type_enum {
    /// The event type of conversions that are attributed to.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ConversionAttributionEventType {
        /// Not specified.
        Unspecified = 0,
        /// Represents value unknown in this version.
        Unknown = 1,
        /// The conversion is attributed to an impression.
        Impression = 2,
        /// The conversion is attributed to an interaction.
        Interaction = 3,
    }
}
/// Container for enum representing the number of days between impression and
/// conversion.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConversionLagBucketEnum {}
/// Nested message and enum types in `ConversionLagBucketEnum`.
pub mod conversion_lag_bucket_enum {
    /// Enum representing the number of days between impression and conversion.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ConversionLagBucket {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Conversion lag bucket from 0 to 1 day. 0 day is included, 1 day is not.
        LessThanOneDay = 2,
        /// Conversion lag bucket from 1 to 2 days. 1 day is included, 2 days is not.
        OneToTwoDays = 3,
        /// Conversion lag bucket from 2 to 3 days. 2 days is included,
        /// 3 days is not.
        TwoToThreeDays = 4,
        /// Conversion lag bucket from 3 to 4 days. 3 days is included,
        /// 4 days is not.
        ThreeToFourDays = 5,
        /// Conversion lag bucket from 4 to 5 days. 4 days is included,
        /// 5 days is not.
        FourToFiveDays = 6,
        /// Conversion lag bucket from 5 to 6 days. 5 days is included,
        /// 6 days is not.
        FiveToSixDays = 7,
        /// Conversion lag bucket from 6 to 7 days. 6 days is included,
        /// 7 days is not.
        SixToSevenDays = 8,
        /// Conversion lag bucket from 7 to 8 days. 7 days is included,
        /// 8 days is not.
        SevenToEightDays = 9,
        /// Conversion lag bucket from 8 to 9 days. 8 days is included,
        /// 9 days is not.
        EightToNineDays = 10,
        /// Conversion lag bucket from 9 to 10 days. 9 days is included,
        /// 10 days is not.
        NineToTenDays = 11,
        /// Conversion lag bucket from 10 to 11 days. 10 days is included,
        /// 11 days is not.
        TenToElevenDays = 12,
        /// Conversion lag bucket from 11 to 12 days. 11 days is included,
        /// 12 days is not.
        ElevenToTwelveDays = 13,
        /// Conversion lag bucket from 12 to 13 days. 12 days is included,
        /// 13 days is not.
        TwelveToThirteenDays = 14,
        /// Conversion lag bucket from 13 to 14 days. 13 days is included,
        /// 14 days is not.
        ThirteenToFourteenDays = 15,
        /// Conversion lag bucket from 14 to 21 days. 14 days is included,
        /// 21 days is not.
        FourteenToTwentyOneDays = 16,
        /// Conversion lag bucket from 21 to 30 days. 21 days is included,
        /// 30 days is not.
        TwentyOneToThirtyDays = 17,
        /// Conversion lag bucket from 30 to 45 days. 30 days is included,
        /// 45 days is not.
        ThirtyToFortyFiveDays = 18,
        /// Conversion lag bucket from 45 to 60 days. 45 days is included,
        /// 60 days is not.
        FortyFiveToSixtyDays = 19,
        /// Conversion lag bucket from 60 to 90 days. 60 days is included,
        /// 90 days is not.
        SixtyToNinetyDays = 20,
    }
}
/// Container for enum representing the number of days between the impression and
/// the conversion or between the impression and adjustments to the conversion.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConversionOrAdjustmentLagBucketEnum {}
/// Nested message and enum types in `ConversionOrAdjustmentLagBucketEnum`.
pub mod conversion_or_adjustment_lag_bucket_enum {
    /// Enum representing the number of days between the impression and the
    /// conversion or between the impression and adjustments to the conversion.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ConversionOrAdjustmentLagBucket {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Conversion lag bucket from 0 to 1 day. 0 day is included, 1 day is not.
        ConversionLessThanOneDay = 2,
        /// Conversion lag bucket from 1 to 2 days. 1 day is included, 2 days is not.
        ConversionOneToTwoDays = 3,
        /// Conversion lag bucket from 2 to 3 days. 2 days is included,
        /// 3 days is not.
        ConversionTwoToThreeDays = 4,
        /// Conversion lag bucket from 3 to 4 days. 3 days is included,
        /// 4 days is not.
        ConversionThreeToFourDays = 5,
        /// Conversion lag bucket from 4 to 5 days. 4 days is included,
        /// 5 days is not.
        ConversionFourToFiveDays = 6,
        /// Conversion lag bucket from 5 to 6 days. 5 days is included,
        /// 6 days is not.
        ConversionFiveToSixDays = 7,
        /// Conversion lag bucket from 6 to 7 days. 6 days is included,
        /// 7 days is not.
        ConversionSixToSevenDays = 8,
        /// Conversion lag bucket from 7 to 8 days. 7 days is included,
        /// 8 days is not.
        ConversionSevenToEightDays = 9,
        /// Conversion lag bucket from 8 to 9 days. 8 days is included,
        /// 9 days is not.
        ConversionEightToNineDays = 10,
        /// Conversion lag bucket from 9 to 10 days. 9 days is included,
        /// 10 days is not.
        ConversionNineToTenDays = 11,
        /// Conversion lag bucket from 10 to 11 days. 10 days is included,
        /// 11 days is not.
        ConversionTenToElevenDays = 12,
        /// Conversion lag bucket from 11 to 12 days. 11 days is included,
        /// 12 days is not.
        ConversionElevenToTwelveDays = 13,
        /// Conversion lag bucket from 12 to 13 days. 12 days is included,
        /// 13 days is not.
        ConversionTwelveToThirteenDays = 14,
        /// Conversion lag bucket from 13 to 14 days. 13 days is included,
        /// 14 days is not.
        ConversionThirteenToFourteenDays = 15,
        /// Conversion lag bucket from 14 to 21 days. 14 days is included,
        /// 21 days is not.
        ConversionFourteenToTwentyOneDays = 16,
        /// Conversion lag bucket from 21 to 30 days. 21 days is included,
        /// 30 days is not.
        ConversionTwentyOneToThirtyDays = 17,
        /// Conversion lag bucket from 30 to 45 days. 30 days is included,
        /// 45 days is not.
        ConversionThirtyToFortyFiveDays = 18,
        /// Conversion lag bucket from 45 to 60 days. 45 days is included,
        /// 60 days is not.
        ConversionFortyFiveToSixtyDays = 19,
        /// Conversion lag bucket from 60 to 90 days. 60 days is included,
        /// 90 days is not.
        ConversionSixtyToNinetyDays = 20,
        /// Conversion adjustment lag bucket from 0 to 1 day. 0 day is included,
        /// 1 day is not.
        AdjustmentLessThanOneDay = 21,
        /// Conversion adjustment lag bucket from 1 to 2 days. 1 day is included,
        /// 2 days is not.
        AdjustmentOneToTwoDays = 22,
        /// Conversion adjustment lag bucket from 2 to 3 days. 2 days is included,
        /// 3 days is not.
        AdjustmentTwoToThreeDays = 23,
        /// Conversion adjustment lag bucket from 3 to 4 days. 3 days is included,
        /// 4 days is not.
        AdjustmentThreeToFourDays = 24,
        /// Conversion adjustment lag bucket from 4 to 5 days. 4 days is included,
        /// 5 days is not.
        AdjustmentFourToFiveDays = 25,
        /// Conversion adjustment lag bucket from 5 to 6 days. 5 days is included,
        /// 6 days is not.
        AdjustmentFiveToSixDays = 26,
        /// Conversion adjustment lag bucket from 6 to 7 days. 6 days is included,
        /// 7 days is not.
        AdjustmentSixToSevenDays = 27,
        /// Conversion adjustment lag bucket from 7 to 8 days. 7 days is included,
        /// 8 days is not.
        AdjustmentSevenToEightDays = 28,
        /// Conversion adjustment lag bucket from 8 to 9 days. 8 days is included,
        /// 9 days is not.
        AdjustmentEightToNineDays = 29,
        /// Conversion adjustment lag bucket from 9 to 10 days. 9 days is included,
        /// 10 days is not.
        AdjustmentNineToTenDays = 30,
        /// Conversion adjustment lag bucket from 10 to 11 days. 10 days is included,
        /// 11 days is not.
        AdjustmentTenToElevenDays = 31,
        /// Conversion adjustment lag bucket from 11 to 12 days. 11 days is included,
        /// 12 days is not.
        AdjustmentElevenToTwelveDays = 32,
        /// Conversion adjustment lag bucket from 12 to 13 days. 12 days is included,
        /// 13 days is not.
        AdjustmentTwelveToThirteenDays = 33,
        /// Conversion adjustment lag bucket from 13 to 14 days. 13 days is included,
        /// 14 days is not.
        AdjustmentThirteenToFourteenDays = 34,
        /// Conversion adjustment lag bucket from 14 to 21 days. 14 days is included,
        /// 21 days is not.
        AdjustmentFourteenToTwentyOneDays = 35,
        /// Conversion adjustment lag bucket from 21 to 30 days. 21 days is included,
        /// 30 days is not.
        AdjustmentTwentyOneToThirtyDays = 36,
        /// Conversion adjustment lag bucket from 30 to 45 days. 30 days is included,
        /// 45 days is not.
        AdjustmentThirtyToFortyFiveDays = 37,
        /// Conversion adjustment lag bucket from 45 to 60 days. 45 days is included,
        /// 60 days is not.
        AdjustmentFortyFiveToSixtyDays = 38,
        /// Conversion adjustment lag bucket from 60 to 90 days. 60 days is included,
        /// 90 days is not.
        AdjustmentSixtyToNinetyDays = 39,
        /// Conversion adjustment lag bucket from 90 to 145 days. 90 days is
        /// included, 145 days is not.
        AdjustmentNinetyToOneHundredAndFortyFiveDays = 40,
        /// Conversion lag bucket UNKNOWN. This is for dates before conversion lag
        /// bucket was available in Google Ads.
        ConversionUnknown = 41,
        /// Conversion adjustment lag bucket UNKNOWN. This is for dates before
        /// conversion adjustment lag bucket was available in Google Ads.
        AdjustmentUnknown = 42,
    }
}
/// Container for enum describing the external conversion source that is
/// associated with a ConversionAction.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExternalConversionSourceEnum {}
/// Nested message and enum types in `ExternalConversionSourceEnum`.
pub mod external_conversion_source_enum {
    /// The external conversion source that is associated with a ConversionAction.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ExternalConversionSource {
        /// Not specified.
        Unspecified = 0,
        /// Represents value unknown in this version.
        Unknown = 1,
        /// Conversion that occurs when a user navigates to a particular webpage
        /// after viewing an ad; Displayed in Google Ads UI as 'Website'.
        Webpage = 2,
        /// Conversion that comes from linked Google Analytics goal or transaction;
        /// Displayed in Google Ads UI as 'Analytics'.
        Analytics = 3,
        /// Website conversion that is uploaded through ConversionUploadService;
        /// Displayed in Google Ads UI as 'Import from clicks'.
        Upload = 4,
        /// Conversion that occurs when a user clicks on a call extension directly on
        /// an ad; Displayed in Google Ads UI as 'Calls from ads'.
        AdCallMetrics = 5,
        /// Conversion that occurs when a user calls a dynamically-generated phone
        /// number (by installed javascript) from an advertiser's website after
        /// clicking on an ad; Displayed in Google Ads UI as 'Calls from website'.
        WebsiteCallMetrics = 6,
        /// Conversion that occurs when a user visits an advertiser's retail store
        /// after clicking on a Google ad;
        /// Displayed in Google Ads UI as 'Store visits'.
        StoreVisits = 7,
        /// Conversion that occurs when a user takes an in-app action such as a
        /// purchase in an Android app;
        /// Displayed in Google Ads UI as 'Android in-app action'.
        AndroidInApp = 8,
        /// Conversion that occurs when a user takes an in-app action such as a
        /// purchase in an iOS app;
        /// Displayed in Google Ads UI as 'iOS in-app action'.
        IosInApp = 9,
        /// Conversion that occurs when a user opens an iOS app for the first time;
        /// Displayed in Google Ads UI as 'iOS app install (first open)'.
        IosFirstOpen = 10,
        /// Legacy app conversions that do not have an AppPlatform provided;
        /// Displayed in Google Ads UI as 'Mobile app'.
        AppUnspecified = 11,
        /// Conversion that occurs when a user opens an Android app for the first
        /// time; Displayed in Google Ads UI as 'Android app install (first open)'.
        AndroidFirstOpen = 12,
        /// Call conversion that is uploaded through ConversionUploadService;
        /// Displayed in Google Ads UI as 'Import from calls'.
        UploadCalls = 13,
        /// Conversion that comes from a linked Firebase event;
        /// Displayed in Google Ads UI as 'Firebase'.
        Firebase = 14,
        /// Conversion that occurs when a user clicks on a mobile phone number;
        /// Displayed in Google Ads UI as 'Phone number clicks'.
        ClickToCall = 15,
        /// Conversion that comes from Salesforce;
        /// Displayed in Google Ads UI as 'Salesforce.com'.
        Salesforce = 16,
        /// Conversion that comes from in-store purchases recorded by CRM;
        /// Displayed in Google Ads UI as 'Store sales (data partner)'.
        StoreSalesCrm = 17,
        /// Conversion that comes from in-store purchases from payment network;
        /// Displayed in Google Ads UI as 'Store sales (payment network)'.
        StoreSalesPaymentNetwork = 18,
        /// Codeless Google Play conversion;
        /// Displayed in Google Ads UI as 'Google Play'.
        GooglePlay = 19,
        /// Conversion that comes from a linked third-party app analytics event;
        /// Displayed in Google Ads UI as 'Third-party app analytics'.
        ThirdPartyAppAnalytics = 20,
        /// Conversion that is controlled by Google Attribution.
        GoogleAttribution = 21,
        /// Store Sales conversion based on first-party or third-party merchant data
        /// uploads. Displayed in Google Ads UI as 'Store sales (direct upload)'.
        StoreSalesDirectUpload = 23,
        /// Store Sales conversion based on first-party or third-party merchant
        /// data uploads and/or from in-store purchases using cards from payment
        /// networks. Displayed in Google Ads UI as 'Store sales'.
        StoreSales = 24,
        /// Conversions imported from Search Ads 360 Floodlight data.
        SearchAds360 = 25,
        /// Conversions that track local actions from Google's products and services
        /// after interacting with an ad.
        GoogleHosted = 27,
        /// Conversions reported by Floodlight tags.
        Floodlight = 29,
        /// Conversions that come from Google Analytics specifically for Search Ads
        /// 360. Displayed in Google Ads UI as Analytics (SA360).
        AnalyticsSearchAds360 = 31,
        /// Conversion that comes from a linked Firebase event for Search Ads 360.
        FirebaseSearchAds360 = 33,
    }
}
// Proto file describing hotel price buckets.

/// Container for enum describing hotel price bucket for a hotel itinerary.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HotelPriceBucketEnum {}
/// Nested message and enum types in `HotelPriceBucketEnum`.
pub mod hotel_price_bucket_enum {
    /// Enum describing possible hotel price buckets.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum HotelPriceBucket {
        /// Not specified.
        Unspecified = 0,
        /// The value is unknown in this version.
        Unknown = 1,
        /// Uniquely lowest price. Partner has the lowest price, and no other
        /// partners are within a small variance of that price.
        LowestUnique = 2,
        /// Tied for lowest price. Partner is within a small variance of the lowest
        /// price.
        LowestTied = 3,
        /// Not lowest price. Partner is not within a small variance of the lowest
        /// price.
        NotLowest = 4,
        /// Partner was the only one shown.
        OnlyPartnerShown = 5,
    }
}
// Proto file describing hotel rate types.

/// Container for enum describing possible hotel rate types.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HotelRateTypeEnum {}
/// Nested message and enum types in `HotelRateTypeEnum`.
pub mod hotel_rate_type_enum {
    /// Enum describing possible hotel rate types.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum HotelRateType {
        /// Not specified.
        Unspecified = 0,
        /// The value is unknown in this version.
        Unknown = 1,
        /// Rate type information is unavailable.
        Unavailable = 2,
        /// Rates available to everyone.
        PublicRate = 3,
        /// A membership program rate is available and satisfies basic requirements
        /// like having a public rate available. UI treatment will strikethrough the
        /// public rate and indicate that a discount is available to the user. For
        /// more on Qualified Rates, visit
        /// <https://developers.google.com/hotels/hotel-ads/dev-guide/qualified-rates>
        QualifiedRate = 4,
        /// Rates available to users that satisfy some eligibility criteria. e.g.
        /// all signed-in users, 20% of mobile users, all mobile users in Canada,
        /// etc.
        PrivateRate = 5,
    }
}
// Proto file describing feed placeholder types.

/// Container for enum describing possible placeholder types for a feed mapping.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlaceholderTypeEnum {}
/// Nested message and enum types in `PlaceholderTypeEnum`.
pub mod placeholder_type_enum {
    /// Possible placeholder types for a feed mapping.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum PlaceholderType {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Lets you show links in your ad to pages from your website, including the
        /// main landing page.
        Sitelink = 2,
        /// Lets you attach a phone number to an ad, allowing customers to call
        /// directly from the ad.
        Call = 3,
        /// Lets you provide users with a link that points to a mobile app in
        /// addition to a website.
        App = 4,
        /// Lets you show locations of businesses from your Google My Business
        /// account in your ad. This helps people find your locations by showing your
        /// ads with your address, a map to your location, or the distance to your
        /// business. This extension type is useful to draw customers to your
        /// brick-and-mortar location.
        Location = 5,
        /// If you sell your product through retail chains, affiliate location
        /// extensions let you show nearby stores that carry your products.
        AffiliateLocation = 6,
        /// Lets you include additional text with your search ads that provide
        /// detailed information about your business, including products and services
        /// you offer. Callouts appear in ads at the top and bottom of Google search
        /// results.
        Callout = 7,
        /// Lets you add more info to your ad, specific to some predefined categories
        /// such as types, brands, styles, etc. A minimum of 3 text (SNIPPETS) values
        /// are required.
        StructuredSnippet = 8,
        /// Allows users to see your ad, click an icon, and contact you directly by
        /// text message. With one tap on your ad, people can contact you to book an
        /// appointment, get a quote, ask for information, or request a service.
        Message = 9,
        /// Lets you display prices for a list of items along with your ads. A price
        /// feed is composed of three to eight price table rows.
        Price = 10,
        /// Allows you to highlight sales and other promotions that let users see how
        /// they can save by buying now.
        Promotion = 11,
        /// Lets you dynamically inject custom data into the title and description
        /// of your ads.
        AdCustomizer = 12,
        /// Indicates that this feed is for education dynamic remarketing.
        DynamicEducation = 13,
        /// Indicates that this feed is for flight dynamic remarketing.
        DynamicFlight = 14,
        /// Indicates that this feed is for a custom dynamic remarketing type. Use
        /// this only if the other business types don't apply to your products or
        /// services.
        DynamicCustom = 15,
        /// Indicates that this feed is for hotels and rentals dynamic remarketing.
        DynamicHotel = 16,
        /// Indicates that this feed is for real estate dynamic remarketing.
        DynamicRealEstate = 17,
        /// Indicates that this feed is for travel dynamic remarketing.
        DynamicTravel = 18,
        /// Indicates that this feed is for local deals dynamic remarketing.
        DynamicLocal = 19,
        /// Indicates that this feed is for job dynamic remarketing.
        DynamicJob = 20,
        /// Lets you attach an image to an ad.
        Image = 21,
    }
}
// Proto file describing search engine results page types.

/// The type of the search engine results page.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchEngineResultsPageTypeEnum {}
/// Nested message and enum types in `SearchEngineResultsPageTypeEnum`.
pub mod search_engine_results_page_type_enum {
    /// The type of the search engine results page.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum SearchEngineResultsPageType {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Only ads were contained in the search engine results page.
        AdsOnly = 2,
        /// Only organic results were contained in the search engine results page.
        OrganicOnly = 3,
        /// Both ads and organic results were contained in the search engine results
        /// page.
        AdsAndOrganic = 4,
    }
}
// Proto file describing search term match types.

/// Container for enum describing match types for a keyword triggering an ad.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchTermMatchTypeEnum {}
/// Nested message and enum types in `SearchTermMatchTypeEnum`.
pub mod search_term_match_type_enum {
    /// Possible match types for a keyword triggering an ad, including variants.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum SearchTermMatchType {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Broad match.
        Broad = 2,
        /// Exact match.
        Exact = 3,
        /// Phrase match.
        Phrase = 4,
        /// Exact match (close variant).
        NearExact = 5,
        /// Phrase match (close variant).
        NearPhrase = 6,
    }
}
// Proto file describing slots.

/// Container for enumeration of possible positions of the Ad.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SlotEnum {}
/// Nested message and enum types in `SlotEnum`.
pub mod slot_enum {
    /// Enumerates possible positions of the Ad.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Slot {
        /// Not specified.
        Unspecified = 0,
        /// The value is unknown in this version.
        Unknown = 1,
        /// Google search: Side.
        SearchSide = 2,
        /// Google search: Top.
        SearchTop = 3,
        /// Google search: Other.
        SearchOther = 4,
        /// Google Display Network.
        Content = 5,
        /// Search partners: Top.
        SearchPartnerTop = 6,
        /// Search partners: Other.
        SearchPartnerOther = 7,
        /// Cross-network.
        Mixed = 8,
    }
}
/// Container for enum describing the format of the web page where the tracking
/// tag and snippet will be installed.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TrackingCodePageFormatEnum {}
/// Nested message and enum types in `TrackingCodePageFormatEnum`.
pub mod tracking_code_page_format_enum {
    /// The format of the web page where the tracking tag and snippet will be
    /// installed.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum TrackingCodePageFormat {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Standard HTML page format.
        Html = 2,
        /// Google AMP page format.
        Amp = 3,
    }
}
/// Container for enum describing the type of the generated tag snippets for
/// tracking conversions.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TrackingCodeTypeEnum {}
/// Nested message and enum types in `TrackingCodeTypeEnum`.
pub mod tracking_code_type_enum {
    /// The type of the generated tag snippets for tracking conversions.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum TrackingCodeType {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// The snippet that is fired as a result of a website page loading.
        Webpage = 2,
        /// The snippet contains a JavaScript function which fires the tag. This
        /// function is typically called from an onClick handler added to a link or
        /// button element on the page.
        WebpageOnclick = 3,
        /// For embedding on a mobile webpage. The snippet contains a JavaScript
        /// function which fires the tag.
        ClickToCall = 4,
        /// The snippet that is used to replace the phone number on your website with
        /// a Google forwarding number for call tracking purposes.
        WebsiteCall = 5,
    }
}
// Proto file describing criteria types.

/// The dimensions that can be targeted.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TargetingDimensionEnum {}
/// Nested message and enum types in `TargetingDimensionEnum`.
pub mod targeting_dimension_enum {
    /// Enum describing possible targeting dimensions.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum TargetingDimension {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Keyword criteria, e.g. 'mars cruise'. KEYWORD may be used as a custom bid
        /// dimension. Keywords are always a targeting dimension, so may not be set
        /// as a target "ALL" dimension with TargetRestriction.
        Keyword = 2,
        /// Audience criteria, which include user list, user interest, custom
        /// affinity,  and custom in market.
        Audience = 3,
        /// Topic criteria for targeting categories of content, e.g.
        /// 'category::Animals>Pets' Used for Display and Video targeting.
        Topic = 4,
        /// Criteria for targeting gender.
        Gender = 5,
        /// Criteria for targeting age ranges.
        AgeRange = 6,
        /// Placement criteria, which include websites like 'www.flowers4sale.com',
        /// as well as mobile applications, mobile app categories, YouTube videos,
        /// and YouTube channels.
        Placement = 7,
        /// Criteria for parental status targeting.
        ParentalStatus = 8,
        /// Criteria for income range targeting.
        IncomeRange = 9,
    }
}
/// Indicates what type of data are the user list's members matched from.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomerMatchUploadKeyTypeEnum {}
/// Nested message and enum types in `CustomerMatchUploadKeyTypeEnum`.
pub mod customer_match_upload_key_type_enum {
    /// Enum describing possible customer match upload key types.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum CustomerMatchUploadKeyType {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Members are matched from customer info such as email address, phone
        /// number or physical address.
        ContactInfo = 2,
        /// Members are matched from a user id generated and assigned by the
        /// advertiser.
        CrmId = 3,
        /// Members are matched from mobile advertising ids.
        MobileAdvertisingId = 4,
    }
}
/// Logical operator connecting two rules.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserListCombinedRuleOperatorEnum {}
/// Nested message and enum types in `UserListCombinedRuleOperatorEnum`.
pub mod user_list_combined_rule_operator_enum {
    /// Enum describing possible user list combined rule operators.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum UserListCombinedRuleOperator {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// A AND B.
        And = 2,
        /// A AND NOT B.
        AndNot = 3,
    }
}
/// Indicates source of Crm upload data.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserListCrmDataSourceTypeEnum {}
/// Nested message and enum types in `UserListCrmDataSourceTypeEnum`.
pub mod user_list_crm_data_source_type_enum {
    /// Enum describing possible user list crm data source type.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum UserListCrmDataSourceType {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// The uploaded data is first-party data.
        FirstParty = 2,
        /// The uploaded data is from a third-party credit bureau.
        ThirdPartyCreditBureau = 3,
        /// The uploaded data is from a third-party voter file.
        ThirdPartyVoterFile = 4,
    }
}
/// Supported rule operator for date type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserListDateRuleItemOperatorEnum {}
/// Nested message and enum types in `UserListDateRuleItemOperatorEnum`.
pub mod user_list_date_rule_item_operator_enum {
    /// Enum describing possible user list date rule item operators.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum UserListDateRuleItemOperator {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Equals.
        Equals = 2,
        /// Not Equals.
        NotEquals = 3,
        /// Before.
        Before = 4,
        /// After.
        After = 5,
    }
}
/// The logical operator of the rule.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserListLogicalRuleOperatorEnum {}
/// Nested message and enum types in `UserListLogicalRuleOperatorEnum`.
pub mod user_list_logical_rule_operator_enum {
    /// Enum describing possible user list logical rule operators.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum UserListLogicalRuleOperator {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// And - all of the operands.
        All = 2,
        /// Or - at least one of the operands.
        Any = 3,
        /// Not - none of the operands.
        None = 4,
    }
}
/// Supported rule operator for number type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserListNumberRuleItemOperatorEnum {}
/// Nested message and enum types in `UserListNumberRuleItemOperatorEnum`.
pub mod user_list_number_rule_item_operator_enum {
    /// Enum describing possible user list number rule item operators.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum UserListNumberRuleItemOperator {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Greater than.
        GreaterThan = 2,
        /// Greater than or equal.
        GreaterThanOrEqual = 3,
        /// Equals.
        Equals = 4,
        /// Not equals.
        NotEquals = 5,
        /// Less than.
        LessThan = 6,
        /// Less than or equal.
        LessThanOrEqual = 7,
    }
}
/// Indicates status of prepopulation based on the rule.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserListPrepopulationStatusEnum {}
/// Nested message and enum types in `UserListPrepopulationStatusEnum`.
pub mod user_list_prepopulation_status_enum {
    /// Enum describing possible user list prepopulation status.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum UserListPrepopulationStatus {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Prepopoulation is being requested.
        Requested = 2,
        /// Prepopulation is finished.
        Finished = 3,
        /// Prepopulation failed.
        Failed = 4,
    }
}
/// Rule based user list rule type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserListRuleTypeEnum {}
/// Nested message and enum types in `UserListRuleTypeEnum`.
pub mod user_list_rule_type_enum {
    /// Enum describing possible user list rule types.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum UserListRuleType {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Conjunctive normal form.
        AndOfOrs = 2,
        /// Disjunctive normal form.
        OrOfAnds = 3,
    }
}
/// Supported rule operator for string type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserListStringRuleItemOperatorEnum {}
/// Nested message and enum types in `UserListStringRuleItemOperatorEnum`.
pub mod user_list_string_rule_item_operator_enum {
    /// Enum describing possible user list string rule item operators.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum UserListStringRuleItemOperator {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Contains.
        Contains = 2,
        /// Equals.
        Equals = 3,
        /// Starts with.
        StartsWith = 4,
        /// Ends with.
        EndsWith = 5,
        /// Not equals.
        NotEquals = 6,
        /// Not contains.
        NotContains = 7,
        /// Not starts with.
        NotStartsWith = 8,
        /// Not ends with.
        NotEndsWith = 9,
    }
}
// Proto file describing AccessInvitationStatus enum.

/// Container for enum for identifying the status of access invitation
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccessInvitationStatusEnum {}
/// Nested message and enum types in `AccessInvitationStatusEnum`.
pub mod access_invitation_status_enum {
    /// Possible access invitation status of a user
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum AccessInvitationStatus {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// The initial state of an invitation, before being acted upon by anyone.
        Pending = 2,
        /// Invitation process was terminated by the email recipient. No new user was
        /// created.
        Declined = 3,
        /// Invitation URLs expired without being acted upon. No new user can be
        /// created.  Invitations expire 20 days after creation.
        Expired = 4,
    }
}
/// Indicates the way the resource such as user list is related to a user.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccessReasonEnum {}
/// Nested message and enum types in `AccessReasonEnum`.
pub mod access_reason_enum {
    /// Enum describing possible access reasons.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum AccessReason {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// The resource is owned by the user.
        Owned = 2,
        /// The resource is shared to the user.
        Shared = 3,
        /// The resource is licensed to the user.
        Licensed = 4,
        /// The user subscribed to the resource.
        Subscribed = 5,
        /// The resource is accessible to the user.
        Affiliated = 6,
    }
}
/// Container for enum describing possible access role for user.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccessRoleEnum {}
/// Nested message and enum types in `AccessRoleEnum`.
pub mod access_role_enum {
    /// Possible access role of a user.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum AccessRole {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Owns its account and can control the addition of other users.
        Admin = 2,
        /// Can modify campaigns, but can't affect other users.
        Standard = 3,
        /// Can view campaigns and account changes, but cannot make edits.
        ReadOnly = 4,
        /// Role for \"email only\" access. Represents an email recipient rather than
        /// a true User entity.
        EmailOnly = 5,
    }
}
// Proto file describing AccountBudgetProposal statuses.

/// Message describing AccountBudgetProposal statuses.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountBudgetProposalStatusEnum {}
/// Nested message and enum types in `AccountBudgetProposalStatusEnum`.
pub mod account_budget_proposal_status_enum {
    /// The possible statuses of an AccountBudgetProposal.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum AccountBudgetProposalStatus {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// The proposal is pending approval.
        Pending = 2,
        /// The proposal has been approved but the corresponding billing setup
        /// has not.  This can occur for proposals that set up the first budget
        /// when signing up for billing or when performing a change of bill-to
        /// operation.
        ApprovedHeld = 3,
        /// The proposal has been approved.
        Approved = 4,
        /// The proposal has been cancelled by the user.
        Cancelled = 5,
        /// The proposal has been rejected by the user, e.g. by rejecting an
        /// acceptance email.
        Rejected = 6,
    }
}
// Proto file describing AccountBudgetProposal types.

/// Message describing AccountBudgetProposal types.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountBudgetProposalTypeEnum {}
/// Nested message and enum types in `AccountBudgetProposalTypeEnum`.
pub mod account_budget_proposal_type_enum {
    /// The possible types of an AccountBudgetProposal.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum AccountBudgetProposalType {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Identifies a request to create a new budget.
        Create = 2,
        /// Identifies a request to edit an existing budget.
        Update = 3,
        /// Identifies a request to end a budget that has already started.
        End = 4,
        /// Identifies a request to remove a budget that hasn't started yet.
        Remove = 5,
    }
}
// Proto file describing AccountBudget statuses.

/// Message describing AccountBudget statuses.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountBudgetStatusEnum {}
/// Nested message and enum types in `AccountBudgetStatusEnum`.
pub mod account_budget_status_enum {
    /// The possible statuses of an AccountBudget.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum AccountBudgetStatus {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// The account budget is pending approval.
        Pending = 2,
        /// The account budget has been approved.
        Approved = 3,
        /// The account budget has been cancelled by the user.
        Cancelled = 4,
    }
}
/// Container for enum describing possible statuses of an account link.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountLinkStatusEnum {}
/// Nested message and enum types in `AccountLinkStatusEnum`.
pub mod account_link_status_enum {
    /// Describes the possible statuses for a link between a Google Ads customer
    /// and another account.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum AccountLinkStatus {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// The link is enabled.
        Enabled = 2,
        /// The link is removed/disabled.
        Removed = 3,
        /// The link to the other account has been requested. A user on the other
        /// account may now approve the link by setting the status to ENABLED.
        Requested = 4,
        /// This link has been requested by a user on the other account. It may be
        /// approved by a user on this account by setting the status to ENABLED.
        PendingApproval = 5,
        /// The link is rejected by the approver.
        Rejected = 6,
        /// The link is revoked by the user who requested the link.
        Revoked = 7,
    }
}
// Proto file describing Ad Customizer placeholder fields.

/// Values for Ad Customizer placeholder fields.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdCustomizerPlaceholderFieldEnum {}
/// Nested message and enum types in `AdCustomizerPlaceholderFieldEnum`.
pub mod ad_customizer_placeholder_field_enum {
    /// Possible values for Ad Customizers placeholder fields.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum AdCustomizerPlaceholderField {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Data Type: INT64. Integer value to be inserted.
        Integer = 2,
        /// Data Type: STRING. Price value to be inserted.
        Price = 3,
        /// Data Type: DATE_TIME. Date value to be inserted.
        Date = 4,
        /// Data Type: STRING. String value to be inserted.
        String = 5,
    }
}
// Proto file describing ad group ad rotation mode.

/// Container for enum describing possible ad rotation modes of ads within an
/// ad group.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdGroupAdRotationModeEnum {}
/// Nested message and enum types in `AdGroupAdRotationModeEnum`.
pub mod ad_group_ad_rotation_mode_enum {
    /// The possible ad rotation modes of an ad group.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum AdGroupAdRotationMode {
        /// The ad rotation mode has not been specified.
        Unspecified = 0,
        /// The received value is not known in this version.
        ///
        /// This is a response-only value.
        Unknown = 1,
        /// Optimize ad group ads based on clicks or conversions.
        Optimize = 2,
        /// Rotate evenly forever.
        RotateForever = 3,
    }
}
// Proto file describing ad group status.

/// Container for enum describing possible statuses of an AdGroupAd.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdGroupAdStatusEnum {}
/// Nested message and enum types in `AdGroupAdStatusEnum`.
pub mod ad_group_ad_status_enum {
    /// The possible statuses of an AdGroupAd.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum AdGroupAdStatus {
        /// No value has been specified.
        Unspecified = 0,
        /// The received value is not known in this version.
        ///
        /// This is a response-only value.
        Unknown = 1,
        /// The ad group ad is enabled.
        Enabled = 2,
        /// The ad group ad is paused.
        Paused = 3,
        /// The ad group ad is removed.
        Removed = 4,
    }
}
// Proto file describing approval status for the criterion.

/// Container for enum describing possible AdGroupCriterion approval statuses.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdGroupCriterionApprovalStatusEnum {}
/// Nested message and enum types in `AdGroupCriterionApprovalStatusEnum`.
pub mod ad_group_criterion_approval_status_enum {
    /// Enumerates AdGroupCriterion approval statuses.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum AdGroupCriterionApprovalStatus {
        /// Not specified.
        Unspecified = 0,
        /// The value is unknown in this version.
        Unknown = 1,
        /// Approved.
        Approved = 2,
        /// Disapproved.
        Disapproved = 3,
        /// Pending Review.
        PendingReview = 4,
        /// Under review.
        UnderReview = 5,
    }
}
// Proto file describing AdGroupCriterion statuses.

/// Message describing AdGroupCriterion statuses.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdGroupCriterionStatusEnum {}
/// Nested message and enum types in `AdGroupCriterionStatusEnum`.
pub mod ad_group_criterion_status_enum {
    /// The possible statuses of an AdGroupCriterion.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum AdGroupCriterionStatus {
        /// No value has been specified.
        Unspecified = 0,
        /// The received value is not known in this version.
        ///
        /// This is a response-only value.
        Unknown = 1,
        /// The ad group criterion is enabled.
        Enabled = 2,
        /// The ad group criterion is paused.
        Paused = 3,
        /// The ad group criterion is removed.
        Removed = 4,
    }
}
// Proto file describing ad group status.

/// Container for enum describing possible statuses of an ad group.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdGroupStatusEnum {}
/// Nested message and enum types in `AdGroupStatusEnum`.
pub mod ad_group_status_enum {
    /// The possible statuses of an ad group.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum AdGroupStatus {
        /// The status has not been specified.
        Unspecified = 0,
        /// The received value is not known in this version.
        ///
        /// This is a response-only value.
        Unknown = 1,
        /// The ad group is enabled.
        Enabled = 2,
        /// The ad group is paused.
        Paused = 3,
        /// The ad group is removed.
        Removed = 4,
    }
}
// Proto file describing ad group types.

/// Defines types of an ad group, specific to a particular campaign channel
/// type. This type drives validations that restrict which entities can be
/// added to the ad group.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdGroupTypeEnum {}
/// Nested message and enum types in `AdGroupTypeEnum`.
pub mod ad_group_type_enum {
    /// Enum listing the possible types of an ad group.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum AdGroupType {
        /// The type has not been specified.
        Unspecified = 0,
        /// The received value is not known in this version.
        ///
        /// This is a response-only value.
        Unknown = 1,
        /// The default ad group type for Search campaigns.
        SearchStandard = 2,
        /// The default ad group type for Display campaigns.
        DisplayStandard = 3,
        /// The ad group type for Shopping campaigns serving standard product ads.
        ShoppingProductAds = 4,
        /// The default ad group type for Hotel campaigns.
        HotelAds = 6,
        /// The type for ad groups in Smart Shopping campaigns.
        ShoppingSmartAds = 7,
        /// Short unskippable in-stream video ads.
        VideoBumper = 8,
        /// TrueView (skippable) in-stream video ads.
        VideoTrueViewInStream = 9,
        /// TrueView in-display video ads.
        VideoTrueViewInDisplay = 10,
        /// Unskippable in-stream video ads.
        VideoNonSkippableInStream = 11,
        /// Outstream video ads.
        VideoOutstream = 12,
        /// Ad group type for Dynamic Search Ads ad groups.
        SearchDynamicAds = 13,
        /// The type for ad groups in Shopping Comparison Listing campaigns.
        ShoppingComparisonListingAds = 14,
        /// The ad group type for Promoted Hotel ad groups.
        PromotedHotelAds = 15,
        /// Video responsive ad groups.
        VideoResponsive = 16,
        /// Video efficient reach ad groups.
        VideoEfficientReach = 17,
    }
}
// Proto file describing ad serving statuses.

/// Possible ad serving statuses of a campaign.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdServingOptimizationStatusEnum {}
/// Nested message and enum types in `AdServingOptimizationStatusEnum`.
pub mod ad_serving_optimization_status_enum {
    /// Enum describing possible serving statuses.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum AdServingOptimizationStatus {
        /// No value has been specified.
        Unspecified = 0,
        /// The received value is not known in this version.
        ///
        /// This is a response-only value.
        Unknown = 1,
        /// Ad serving is optimized based on CTR for the campaign.
        Optimize = 2,
        /// Ad serving is optimized based on CTR * Conversion for the campaign. If
        /// the campaign is not in the conversion optimizer bidding strategy, it will
        /// default to OPTIMIZED.
        ConversionOptimize = 3,
        /// Ads are rotated evenly for 90 days, then optimized for clicks.
        Rotate = 4,
        /// Show lower performing ads more evenly with higher performing ads, and do
        /// not optimize.
        RotateIndefinitely = 5,
        /// Ad serving optimization status is not available.
        Unavailable = 6,
    }
}
// Proto file describing ad strengths.

/// Container for enum describing possible ad strengths.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdStrengthEnum {}
/// Nested message and enum types in `AdStrengthEnum`.
pub mod ad_strength_enum {
    /// Enum listing the possible ad strengths.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum AdStrength {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// The ad strength is currently pending.
        Pending = 2,
        /// No ads could be generated.
        NoAds = 3,
        /// Poor strength.
        Poor = 4,
        /// Average strength.
        Average = 5,
        /// Good strength.
        Good = 6,
        /// Excellent strength.
        Excellent = 7,
    }
}
// Proto file describing the ad type.

/// Container for enum describing possible types of an ad.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdTypeEnum {}
/// Nested message and enum types in `AdTypeEnum`.
pub mod ad_type_enum {
    /// The possible types of an ad.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum AdType {
        /// No value has been specified.
        Unspecified = 0,
        /// The received value is not known in this version.
        ///
        /// This is a response-only value.
        Unknown = 1,
        /// The ad is a text ad.
        TextAd = 2,
        /// The ad is an expanded text ad.
        ExpandedTextAd = 3,
        /// The ad is a call only ad.
        CallOnlyAd = 6,
        /// The ad is an expanded dynamic search ad.
        ExpandedDynamicSearchAd = 7,
        /// The ad is a hotel ad.
        HotelAd = 8,
        /// The ad is a Smart Shopping ad.
        ShoppingSmartAd = 9,
        /// The ad is a standard Shopping ad.
        ShoppingProductAd = 10,
        /// The ad is a video ad.
        VideoAd = 12,
        /// This ad is a Gmail ad.
        GmailAd = 13,
        /// This ad is an Image ad.
        ImageAd = 14,
        /// The ad is a responsive search ad.
        ResponsiveSearchAd = 15,
        /// The ad is a legacy responsive display ad.
        LegacyResponsiveDisplayAd = 16,
        /// The ad is an app ad.
        AppAd = 17,
        /// The ad is a legacy app install ad.
        LegacyAppInstallAd = 18,
        /// The ad is a responsive display ad.
        ResponsiveDisplayAd = 19,
        /// The ad is a local ad.
        LocalAd = 20,
        /// The ad is a display upload ad with the HTML5_UPLOAD_AD product type.
        Html5UploadAd = 21,
        /// The ad is a display upload ad with one of the DYNAMIC_HTML5_* product
        /// types.
        DynamicHtml5Ad = 22,
        /// The ad is an app engagement ad.
        AppEngagementAd = 23,
        /// The ad is a Shopping Comparison Listing ad.
        ShoppingComparisonListingAd = 24,
        /// Video bumper ad.
        VideoBumperAd = 25,
        /// Video non-skippable in-stream ad.
        VideoNonSkippableInStreamAd = 26,
        /// Video outstream ad.
        VideoOutstreamAd = 27,
        /// Video TrueView in-display ad.
        VideoTrueviewDiscoveryAd = 28,
        /// Video TrueView in-stream ad.
        VideoTrueviewInStreamAd = 29,
        /// Video responsive ad.
        VideoResponsiveAd = 30,
    }
}
// Proto file describing relation type for affiliate location feeds.

/// Container for enum describing possible values for a relationship type for
/// an affiliate location feed.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AffiliateLocationFeedRelationshipTypeEnum {}
/// Nested message and enum types in `AffiliateLocationFeedRelationshipTypeEnum`.
pub mod affiliate_location_feed_relationship_type_enum {
    /// Possible values for a relationship type for an affiliate location feed.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum AffiliateLocationFeedRelationshipType {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// General retailer relationship.
        GeneralRetailer = 2,
    }
}
// Proto file describing Affiliate Location placeholder fields.

/// Values for Affiliate Location placeholder fields.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AffiliateLocationPlaceholderFieldEnum {}
/// Nested message and enum types in `AffiliateLocationPlaceholderFieldEnum`.
pub mod affiliate_location_placeholder_field_enum {
    /// Possible values for Affiliate Location placeholder fields.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum AffiliateLocationPlaceholderField {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Data Type: STRING. The name of the business.
        BusinessName = 2,
        /// Data Type: STRING. Line 1 of the business address.
        AddressLine1 = 3,
        /// Data Type: STRING. Line 2 of the business address.
        AddressLine2 = 4,
        /// Data Type: STRING. City of the business address.
        City = 5,
        /// Data Type: STRING. Province of the business address.
        Province = 6,
        /// Data Type: STRING. Postal code of the business address.
        PostalCode = 7,
        /// Data Type: STRING. Country code of the business address.
        CountryCode = 8,
        /// Data Type: STRING. Phone number of the business.
        PhoneNumber = 9,
        /// Data Type: STRING. Language code of the business.
        LanguageCode = 10,
        /// Data Type: INT64. ID of the chain.
        ChainId = 11,
        /// Data Type: STRING. Name of the chain.
        ChainName = 12,
    }
}
// Proto file describing App Campaign app store.

/// The application store that distributes mobile applications.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AppCampaignAppStoreEnum {}
/// Nested message and enum types in `AppCampaignAppStoreEnum`.
pub mod app_campaign_app_store_enum {
    /// Enum describing app campaign app store.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum AppCampaignAppStore {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Apple app store.
        AppleAppStore = 2,
        /// Google play.
        GoogleAppStore = 3,
    }
}
// Proto file describing App Campaign bidding strategy goal types.

/// Container for enum describing goal towards which the bidding strategy of an
/// app campaign should optimize for.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AppCampaignBiddingStrategyGoalTypeEnum {}
/// Nested message and enum types in `AppCampaignBiddingStrategyGoalTypeEnum`.
pub mod app_campaign_bidding_strategy_goal_type_enum {
    /// Goal type of App campaign BiddingStrategy.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum AppCampaignBiddingStrategyGoalType {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Aim to maximize the number of app installs. The cpa bid is the
        /// target cost per install.
        OptimizeInstallsTargetInstallCost = 2,
        /// Aim to maximize the long term number of selected in-app conversions from
        /// app installs. The cpa bid is the target cost per install.
        OptimizeInAppConversionsTargetInstallCost = 3,
        /// Aim to maximize the long term number of selected in-app conversions from
        /// app installs. The cpa bid is the target cost per in-app conversion. Note
        /// that the actual cpa may seem higher than the target cpa at first, since
        /// the long term conversions haven’t happened yet.
        OptimizeInAppConversionsTargetConversionCost = 4,
        /// Aim to maximize all conversions' value, i.e. install + selected in-app
        /// conversions while achieving or exceeding target return on advertising
        /// spend.
        OptimizeReturnOnAdvertisingSpend = 5,
    }
}
// Proto file describing App placeholder fields.

/// Values for App placeholder fields.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AppPlaceholderFieldEnum {}
/// Nested message and enum types in `AppPlaceholderFieldEnum`.
pub mod app_placeholder_field_enum {
    /// Possible values for App placeholder fields.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum AppPlaceholderField {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Data Type: INT64. The application store that the target application
        /// belongs to. Valid values are: 1 = Apple iTunes Store; 2 = Google Play
        /// Store.
        Store = 2,
        /// Data Type: STRING. The store-specific ID for the target application.
        Id = 3,
        /// Data Type: STRING. The visible text displayed when the link is rendered
        /// in an ad.
        LinkText = 4,
        /// Data Type: STRING. The destination URL of the in-app link.
        Url = 5,
        /// Data Type: URL_LIST. Final URLs for the in-app link when using Upgraded
        /// URLs.
        FinalUrls = 6,
        /// Data Type: URL_LIST. Final Mobile URLs for the in-app link when using
        /// Upgraded URLs.
        FinalMobileUrls = 7,
        /// Data Type: URL. Tracking template for the in-app link when using Upgraded
        /// URLs.
        TrackingUrl = 8,
        /// Data Type: STRING. Final URL suffix for the in-app link when using
        /// parallel tracking.
        FinalUrlSuffix = 9,
    }
}
// Proto file describing asset type.

/// Container for enum describing the possible placements of an asset.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetFieldTypeEnum {}
/// Nested message and enum types in `AssetFieldTypeEnum`.
pub mod asset_field_type_enum {
    /// Enum describing the possible placements of an asset.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum AssetFieldType {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// The asset is linked for use as a headline.
        Headline = 2,
        /// The asset is linked for use as a description.
        Description = 3,
        /// The asset is linked for use as mandatory ad text.
        MandatoryAdText = 4,
        /// The asset is linked for use as a marketing image.
        MarketingImage = 5,
        /// The asset is linked for use as a media bundle.
        MediaBundle = 6,
        /// The asset is linked for use as a YouTube video.
        YoutubeVideo = 7,
        /// The asset is linked to indicate that a hotels campaign is "Book on
        /// Google" enabled.
        BookOnGoogle = 8,
        /// The asset is linked for use as a Lead Form extension.
        LeadForm = 9,
        /// The asset is linked for use as a Promotion extension.
        Promotion = 10,
        /// The asset is linked for use as a Callout extension.
        Callout = 11,
        /// The asset is linked for use as a Structured Snippet extension.
        StructuredSnippet = 12,
        /// The asset is linked for use as a Sitelink extension.
        Sitelink = 13,
    }
}
// Proto file describing status of an asset link.

/// Container for enum describing possible statuses of an asset link.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetLinkStatusEnum {}
/// Nested message and enum types in `AssetLinkStatusEnum`.
pub mod asset_link_status_enum {
    /// Enum describing statuses of an asset link.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum AssetLinkStatus {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Asset link is enabled.
        Enabled = 2,
        /// Asset link has been removed.
        Removed = 3,
        /// Asset link is paused.
        Paused = 4,
    }
}
// Proto file describing asset type.

/// Container for enum describing the types of asset.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetTypeEnum {}
/// Nested message and enum types in `AssetTypeEnum`.
pub mod asset_type_enum {
    /// Enum describing possible types of asset.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum AssetType {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// YouTube video asset.
        YoutubeVideo = 2,
        /// Media bundle asset.
        MediaBundle = 3,
        /// Image asset.
        Image = 4,
        /// Text asset.
        Text = 5,
        /// Lead form asset.
        LeadForm = 6,
        /// Book on Google asset.
        BookOnGoogle = 7,
        /// Promotion asset.
        Promotion = 8,
        /// Callout asset.
        Callout = 9,
        /// Structured Snippet asset.
        StructuredSnippet = 10,
        /// Sitelink asset.
        Sitelink = 11,
    }
}
/// Container for enum representing the attribution model that describes how to
/// distribute credit for a particular conversion across potentially many prior
/// interactions.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AttributionModelEnum {}
/// Nested message and enum types in `AttributionModelEnum`.
pub mod attribution_model_enum {
    /// The attribution model that describes how to distribute credit for a
    /// particular conversion across potentially many prior interactions.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum AttributionModel {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Uses external attribution.
        External = 100,
        /// Attributes all credit for a conversion to its last click.
        GoogleAdsLastClick = 101,
        /// Attributes all credit for a conversion to its first click using Google
        /// Search attribution.
        GoogleSearchAttributionFirstClick = 102,
        /// Attributes credit for a conversion equally across all of its clicks using
        /// Google Search attribution.
        GoogleSearchAttributionLinear = 103,
        /// Attributes exponentially more credit for a conversion to its more recent
        /// clicks using Google Search attribution (half-life is 1 week).
        GoogleSearchAttributionTimeDecay = 104,
        /// Attributes 40% of the credit for a conversion to its first and last
        /// clicks. Remaining 20% is evenly distributed across all other clicks. This
        /// uses Google Search attribution.
        GoogleSearchAttributionPositionBased = 105,
        /// Flexible model that uses machine learning to determine the appropriate
        /// distribution of credit among clicks using Google Search attribution.
        GoogleSearchAttributionDataDriven = 106,
    }
}
// Proto file describing batch job statuses.

/// Container for enum describing possible batch job statuses.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchJobStatusEnum {}
/// Nested message and enum types in `BatchJobStatusEnum`.
pub mod batch_job_status_enum {
    /// The batch job statuses.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum BatchJobStatus {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// The job is not currently running.
        Pending = 2,
        /// The job is running.
        Running = 3,
        /// The job is done.
        Done = 4,
    }
}
// Proto file describing bid modifier source.

/// Container for enum describing possible bid modifier sources.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BidModifierSourceEnum {}
/// Nested message and enum types in `BidModifierSourceEnum`.
pub mod bid_modifier_source_enum {
    /// Enum describing possible bid modifier sources.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum BidModifierSource {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// The bid modifier is specified at the campaign level, on the campaign
        /// level criterion.
        Campaign = 2,
        /// The bid modifier is specified (overridden) at the ad group level.
        AdGroup = 3,
    }
}
// Proto file describing bidding sources.

/// Container for enum describing possible bidding sources.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BiddingSourceEnum {}
/// Nested message and enum types in `BiddingSourceEnum`.
pub mod bidding_source_enum {
    /// Indicates where a bid or target is defined. For example, an ad group
    /// criterion may define a cpc bid directly, or it can inherit its cpc bid from
    /// the ad group.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum BiddingSource {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Effective bid or target is inherited from campaign bidding strategy.
        CampaignBiddingStrategy = 5,
        /// The bid or target is defined on the ad group.
        AdGroup = 6,
        /// The bid or target is defined on the ad group criterion.
        AdGroupCriterion = 7,
    }
}
// Proto file describing BiddingStrategy statuses.

/// Message describing BiddingStrategy statuses.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BiddingStrategyStatusEnum {}
/// Nested message and enum types in `BiddingStrategyStatusEnum`.
pub mod bidding_strategy_status_enum {
    /// The possible statuses of a BiddingStrategy.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum BiddingStrategyStatus {
        /// No value has been specified.
        Unspecified = 0,
        /// The received value is not known in this version.
        ///
        /// This is a response-only value.
        Unknown = 1,
        /// The bidding strategy is enabled.
        Enabled = 2,
        /// The bidding strategy is removed.
        Removed = 4,
    }
}
// Proto file describing bidding schemes.

/// Container for enum describing possible bidding strategy types.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BiddingStrategyTypeEnum {}
/// Nested message and enum types in `BiddingStrategyTypeEnum`.
pub mod bidding_strategy_type_enum {
    /// Enum describing possible bidding strategy types.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum BiddingStrategyType {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Commission is an automatic bidding strategy in which the advertiser pays
        /// a certain portion of the conversion value.
        Commission = 16,
        /// Enhanced CPC is a bidding strategy that raises bids for clicks
        /// that seem more likely to lead to a conversion and lowers
        /// them for clicks where they seem less likely.
        EnhancedCpc = 2,
        /// Manual click based bidding where user pays per click.
        ManualCpc = 3,
        /// Manual impression based bidding
        /// where user pays per thousand impressions.
        ManualCpm = 4,
        /// A bidding strategy that pays a configurable amount per video view.
        ManualCpv = 13,
        /// A bidding strategy that automatically maximizes number of conversions
        /// given a daily budget.
        MaximizeConversions = 10,
        /// An automated bidding strategy that automatically sets bids to maximize
        /// revenue while spending your budget.
        MaximizeConversionValue = 11,
        /// Page-One Promoted bidding scheme, which sets max cpc bids to
        /// target impressions on page one or page one promoted slots on google.com.
        /// This enum value is deprecated.
        PageOnePromoted = 5,
        /// Percent Cpc is bidding strategy where bids are a fraction of the
        /// advertised price for some good or service.
        PercentCpc = 12,
        /// Target CPA is an automated bid strategy that sets bids
        /// to help get as many conversions as possible
        /// at the target cost-per-acquisition (CPA) you set.
        TargetCpa = 6,
        /// Target CPM is an automated bid strategy that sets bids to help get
        /// as many impressions as possible at the target cost per one thousand
        /// impressions (CPM) you set.
        TargetCpm = 14,
        /// An automated bidding strategy that sets bids so that a certain percentage
        /// of search ads are shown at the top of the first page (or other targeted
        /// location).
        TargetImpressionShare = 15,
        /// Target Outrank Share is an automated bidding strategy that sets bids
        /// based on the target fraction of auctions where the advertiser
        /// should outrank a specific competitor.
        /// This enum value is deprecated.
        TargetOutrankShare = 7,
        /// Target ROAS is an automated bidding strategy
        /// that helps you maximize revenue while averaging
        /// a specific target Return On Average Spend (ROAS).
        TargetRoas = 8,
        /// Target Spend is an automated bid strategy that sets your bids
        /// to help get as many clicks as possible within your budget.
        TargetSpend = 9,
    }
}
// Proto file describing BillingSetup statuses.

/// Message describing BillingSetup statuses.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BillingSetupStatusEnum {}
/// Nested message and enum types in `BillingSetupStatusEnum`.
pub mod billing_setup_status_enum {
    /// The possible statuses of a BillingSetup.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum BillingSetupStatus {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// The billing setup is pending approval.
        Pending = 2,
        /// The billing setup has been approved but the corresponding first budget
        /// has not.  This can only occur for billing setups configured for monthly
        /// invoicing.
        ApprovedHeld = 3,
        /// The billing setup has been approved.
        Approved = 4,
        /// The billing setup was cancelled by the user prior to approval.
        Cancelled = 5,
    }
}
// Proto file describing brand safety suitability settings.

/// Container for enum with 3-Tier brand safety suitability control.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BrandSafetySuitabilityEnum {}
/// Nested message and enum types in `BrandSafetySuitabilityEnum`.
pub mod brand_safety_suitability_enum {
    /// 3-Tier brand safety suitability control.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum BrandSafetySuitability {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// This option lets you show ads across all inventory on YouTube and video
        /// partners that meet our standards for monetization. This option may be an
        /// appropriate choice for brands that want maximum access to the full
        /// breadth of videos eligible for ads, including, for example, videos that
        /// have strong profanity in the context of comedy or a documentary, or
        /// excessive violence as featured in video games.
        ExpandedInventory = 2,
        /// This option lets you show ads across a wide range of content that's
        /// appropriate for most brands, such as popular music videos, documentaries,
        /// and movie trailers. The content you can show ads on is based on YouTube's
        /// advertiser-friendly content guidelines that take into account, for
        /// example, the strength or frequency of profanity, or the appropriateness
        /// of subject matter like sensitive events. Ads won't show, for example, on
        /// content with repeated strong profanity, strong sexual content, or graphic
        /// violence.
        StandardInventory = 3,
        /// This option lets you show ads on a reduced range of content that's
        /// appropriate for brands with particularly strict guidelines around
        /// inappropriate language and sexual suggestiveness; above and beyond what
        /// YouTube's advertiser-friendly content guidelines address. The videos
        /// accessible in this sensitive category meet heightened requirements,
        /// especially for inappropriate language and sexual suggestiveness. For
        /// example, your ads will be excluded from showing on some of YouTube's most
        /// popular music videos and other pop culture content across YouTube and
        /// Google video partners.
        LimitedInventory = 4,
    }
}
// Proto file describing Budget delivery methods.

/// Message describing Budget delivery methods. A delivery method determines the
/// rate at which the Budget is spent.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BudgetDeliveryMethodEnum {}
/// Nested message and enum types in `BudgetDeliveryMethodEnum`.
pub mod budget_delivery_method_enum {
    /// Possible delivery methods of a Budget.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum BudgetDeliveryMethod {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// The budget server will throttle serving evenly across
        /// the entire time period.
        Standard = 2,
        /// The budget server will not throttle serving,
        /// and ads will serve as fast as possible.
        Accelerated = 3,
    }
}
// Proto file describing Budget delivery methods.

/// Message describing Budget period.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BudgetPeriodEnum {}
/// Nested message and enum types in `BudgetPeriodEnum`.
pub mod budget_period_enum {
    /// Possible period of a Budget.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum BudgetPeriod {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Daily budget.
        Daily = 2,
        /// Custom budget, added back in V5.
        /// Custom bugdet can be used with total_amount to specify lifetime budget
        /// limit. See: <https://support.google.com/google-ads/answer/6385083> for more
        /// info.
        CustomPeriod = 5,
    }
}
// Proto file describing Budget statuses

/// Message describing a Budget status
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BudgetStatusEnum {}
/// Nested message and enum types in `BudgetStatusEnum`.
pub mod budget_status_enum {
    /// Possible statuses of a Budget.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum BudgetStatus {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Budget is enabled.
        Enabled = 2,
        /// Budget is removed.
        Removed = 3,
    }
}
// Proto file describing Budget types.

/// Describes Budget types.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BudgetTypeEnum {}
/// Nested message and enum types in `BudgetTypeEnum`.
pub mod budget_type_enum {
    /// Possible Budget types.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum BudgetType {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Budget type for standard Google Ads usage.
        /// Caps daily spend at two times the specified budget amount.
        /// Full details: <https://support.google.com/google-ads/answer/6385083>
        Standard = 2,
        /// Budget type for Hotels Ads commission program.
        /// Full details: <https://support.google.com/google-ads/answer/9243945>
        ///
        /// This type is only supported by campaigns with
        /// AdvertisingChannelType.HOTEL, BiddingStrategyType.COMMISSION and
        /// PaymentMode.CONVERSION_VALUE.
        HotelAdsCommission = 3,
        /// Budget type with a fixed cost-per-acquisition (conversion).
        /// Full details: <https://support.google.com/google-ads/answer/7528254>
        ///
        /// This type is only supported by campaigns with
        /// AdvertisingChannelType.DISPLAY (excluding
        /// AdvertisingChannelSubType.DISPLAY_GMAIL),
        /// BiddingStrategyType.TARGET_CPA and PaymentMode.CONVERSIONS.
        FixedCpa = 4,
    }
}
// Proto file describing Call placeholder fields.

/// Values for Call placeholder fields.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CallPlaceholderFieldEnum {}
/// Nested message and enum types in `CallPlaceholderFieldEnum`.
pub mod call_placeholder_field_enum {
    /// Possible values for Call placeholder fields.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum CallPlaceholderField {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Data Type: STRING. The advertiser's phone number to append to the ad.
        PhoneNumber = 2,
        /// Data Type: STRING. Uppercase two-letter country code of the advertiser's
        /// phone number.
        CountryCode = 3,
        /// Data Type: BOOLEAN. Indicates whether call tracking is enabled. Default:
        /// true.
        Tracked = 4,
        /// Data Type: INT64. The ID of an AdCallMetricsConversion object. This
        /// object contains the phoneCallDurationfield which is the minimum duration
        /// (in seconds) of a call to be considered a conversion.
        ConversionTypeId = 5,
        /// Data Type: STRING. Indicates whether this call extension uses its own
        /// call conversion setting or follows the account level setting.
        /// Valid values are: USE_ACCOUNT_LEVEL_CALL_CONVERSION_ACTION and
        /// USE_RESOURCE_LEVEL_CALL_CONVERSION_ACTION.
        ConversionReportingState = 6,
    }
}
// Proto file describing call tracking display location.

/// Container for enum describing possible call tracking display locations.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CallTrackingDisplayLocationEnum {}
/// Nested message and enum types in `CallTrackingDisplayLocationEnum`.
pub mod call_tracking_display_location_enum {
    /// Possible call tracking display locations.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum CallTrackingDisplayLocation {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// The phone call placed from the ad.
        Ad = 2,
        /// The phone call placed from the landing page ad points to.
        LandingPage = 3,
    }
}
// Proto file describing call types.

/// Container for enum describing possible types of property from where the call
/// was made.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CallTypeEnum {}
/// Nested message and enum types in `CallTypeEnum`.
pub mod call_type_enum {
    /// Possible types of property from where the call was made.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum CallType {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// The phone call was manually dialed.
        ManuallyDialed = 2,
        /// The phone call was a mobile click-to-call.
        HighEndMobileSearch = 3,
    }
}
// Proto file describing Callout placeholder fields.

/// Values for Callout placeholder fields.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CalloutPlaceholderFieldEnum {}
/// Nested message and enum types in `CalloutPlaceholderFieldEnum`.
pub mod callout_placeholder_field_enum {
    /// Possible values for Callout placeholder fields.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum CalloutPlaceholderField {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Data Type: STRING. Callout text.
        CalloutText = 2,
    }
}
// Proto file describing CampaignCriterion statuses.

/// Message describing CampaignCriterion statuses.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CampaignCriterionStatusEnum {}
/// Nested message and enum types in `CampaignCriterionStatusEnum`.
pub mod campaign_criterion_status_enum {
    /// The possible statuses of a CampaignCriterion.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum CampaignCriterionStatus {
        /// No value has been specified.
        Unspecified = 0,
        /// The received value is not known in this version.
        ///
        /// This is a response-only value.
        Unknown = 1,
        /// The campaign criterion is enabled.
        Enabled = 2,
        /// The campaign criterion is paused.
        Paused = 3,
        /// The campaign criterion is removed.
        Removed = 4,
    }
}
// Proto file describing campaign draft status.

/// Container for enum describing possible statuses of a campaign draft.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CampaignDraftStatusEnum {}
/// Nested message and enum types in `CampaignDraftStatusEnum`.
pub mod campaign_draft_status_enum {
    /// Possible statuses of a campaign draft.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum CampaignDraftStatus {
        /// The status has not been specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Initial state of the draft, the advertiser can start adding changes with
        /// no effect on serving.
        Proposed = 2,
        /// The campaign draft is removed.
        Removed = 3,
        /// Advertiser requested to promote draft's changes back into the original
        /// campaign. Advertiser can poll the long running operation returned by
        /// the promote action to see the status of the promotion.
        Promoting = 5,
        /// The process to merge changes in the draft back to the original campaign
        /// has completed successfully.
        Promoted = 4,
        /// The promotion failed after it was partially applied. Promote cannot be
        /// attempted again safely, so the issue must be corrected in the original
        /// campaign.
        PromoteFailed = 6,
    }
}
// Proto file describing campaign experiment status.

/// Container for enum describing possible statuses of a campaign experiment.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CampaignExperimentStatusEnum {}
/// Nested message and enum types in `CampaignExperimentStatusEnum`.
pub mod campaign_experiment_status_enum {
    /// Possible statuses of a campaign experiment.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum CampaignExperimentStatus {
        /// The status has not been specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// The experiment campaign is being initialized.
        Initializing = 2,
        /// Initialization of the experiment campaign failed.
        InitializationFailed = 8,
        /// The experiment campaign is fully initialized. The experiment is currently
        /// running, scheduled to run in the future or has ended based on its
        /// end date. An experiment with the status INITIALIZING will be updated to
        /// ENABLED when it is fully created.
        Enabled = 3,
        /// The experiment campaign was graduated to a stand-alone
        /// campaign, existing independently of the experiment.
        Graduated = 4,
        /// The experiment is removed.
        Removed = 5,
        /// The experiment's changes are being applied to the original campaign.
        /// The long running operation returned by the promote method can be polled
        /// to see the status of the promotion.
        Promoting = 6,
        /// Promote of the experiment campaign failed.
        PromotionFailed = 9,
        /// The changes of the experiment are promoted to their original campaign.
        Promoted = 7,
        /// The experiment was ended manually. It did not end based on its end date.
        EndedManually = 10,
    }
}
// Proto file describing campaign experiment traffic split type.

/// Container for enum describing campaign experiment traffic split type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CampaignExperimentTrafficSplitTypeEnum {}
/// Nested message and enum types in `CampaignExperimentTrafficSplitTypeEnum`.
pub mod campaign_experiment_traffic_split_type_enum {
    /// Enum of strategies for splitting traffic between base and experiment
    /// campaigns in campaign experiment.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum CampaignExperimentTrafficSplitType {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Traffic is randomly assigned to the base or experiment arm for each
        /// query, independent of previous assignments for the same user.
        RandomQuery = 2,
        /// Traffic is split using cookies to keep users in the same arm (base or
        /// experiment) of the experiment.
        Cookie = 3,
    }
}
// Proto file describing campaign experiment type.

/// Container for enum describing campaign experiment type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CampaignExperimentTypeEnum {}
/// Nested message and enum types in `CampaignExperimentTypeEnum`.
pub mod campaign_experiment_type_enum {
    /// Indicates if this campaign is a normal campaign,
    /// a draft campaign, or an experiment campaign.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum CampaignExperimentType {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// This is a regular campaign.
        Base = 2,
        /// This is a draft version of a campaign.
        /// It has some modifications from a base campaign,
        /// but it does not serve or accrue metrics.
        Draft = 3,
        /// This is an experiment version of a campaign.
        /// It has some modifications from a base campaign,
        /// and a percentage of traffic is being diverted
        /// from the BASE campaign to this experiment campaign.
        Experiment = 4,
    }
}
// Proto file describing Campaign serving statuses.

/// Message describing Campaign serving statuses.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CampaignServingStatusEnum {}
/// Nested message and enum types in `CampaignServingStatusEnum`.
pub mod campaign_serving_status_enum {
    /// Possible serving statuses of a campaign.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum CampaignServingStatus {
        /// No value has been specified.
        Unspecified = 0,
        /// The received value is not known in this version.
        ///
        /// This is a response-only value.
        Unknown = 1,
        /// Serving.
        Serving = 2,
        /// None.
        None = 3,
        /// Ended.
        Ended = 4,
        /// Pending.
        Pending = 5,
        /// Suspended.
        Suspended = 6,
    }
}
// Proto file describing campaign shared set statuses.

/// Container for enum describing types of campaign shared set statuses.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CampaignSharedSetStatusEnum {}
/// Nested message and enum types in `CampaignSharedSetStatusEnum`.
pub mod campaign_shared_set_status_enum {
    /// Enum listing the possible campaign shared set statuses.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum CampaignSharedSetStatus {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// The campaign shared set is enabled.
        Enabled = 2,
        /// The campaign shared set is removed and can no longer be used.
        Removed = 3,
    }
}
// Proto file describing campaign status.

/// Container for enum describing possible statuses of a campaign.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CampaignStatusEnum {}
/// Nested message and enum types in `CampaignStatusEnum`.
pub mod campaign_status_enum {
    /// Possible statuses of a campaign.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum CampaignStatus {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Campaign is currently serving ads depending on budget information.
        Enabled = 2,
        /// Campaign has been paused by the user.
        Paused = 3,
        /// Campaign has been removed.
        Removed = 4,
    }
}
// Proto file describing the sources that the change event resource was
// made through.

/// Container for enum describing the sources that the change event resource
/// was made through.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangeClientTypeEnum {}
/// Nested message and enum types in `ChangeClientTypeEnum`.
pub mod change_client_type_enum {
    /// The source that the change_event resource was made through.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ChangeClientType {
        /// No value has been specified.
        Unspecified = 0,
        /// Used for return value only. Represents an unclassified client type
        /// unknown in this version.
        Unknown = 1,
        /// Changes made through the "ads.google.com".
        /// For example, changes made through campaign management.
        GoogleAdsWebClient = 2,
        /// Changes made through Google Ads automated rules.
        GoogleAdsAutomatedRule = 3,
        /// Changes made through Google Ads scripts.
        GoogleAdsScripts = 4,
        /// Changes made by Google Ads bulk upload.
        GoogleAdsBulkUpload = 5,
        /// Changes made by Google Ads API.
        GoogleAdsApi = 6,
        /// Changes made by Google Ads Editor.
        GoogleAdsEditor = 7,
        /// Changes made by Google Ads mobile app.
        GoogleAdsMobileApp = 8,
        /// Changes made through Google Ads recommendations.
        GoogleAdsRecommendations = 9,
        /// Changes made through Search Ads 360 Sync.
        SearchAds360Sync = 10,
        /// Changes made through Search Ads 360 Post.
        SearchAds360Post = 11,
        /// Changes made through internal tools.
        /// For example, when a user sets a URL template on an entity like a
        /// Campaign, it's automatically wrapped with the SA360 Clickserver URL.
        InternalTool = 12,
        /// Types of changes that are not categorized, for example,
        /// changes made by coupon redemption through Google Ads.
        Other = 13,
    }
}
// Proto file describing the resource types the ChangeEvent resource supports.

/// Container for enum describing supported resource types for the ChangeEvent
/// resource.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangeEventResourceTypeEnum {}
/// Nested message and enum types in `ChangeEventResourceTypeEnum`.
pub mod change_event_resource_type_enum {
    /// Enum listing the resource types support by the ChangeEvent resource.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ChangeEventResourceType {
        /// No value has been specified.
        Unspecified = 0,
        /// Used for return value only. Represents an unclassified resource unknown
        /// in this version.
        Unknown = 1,
        /// An Ad resource change.
        Ad = 2,
        /// An AdGroup resource change.
        AdGroup = 3,
        /// An AdGroupCriterion resource change.
        AdGroupCriterion = 4,
        /// A Campaign resource change.
        Campaign = 5,
        /// A CampaignBudget resource change.
        CampaignBudget = 6,
        /// An AdGroupBidModifier resource change.
        AdGroupBidModifier = 7,
        /// A CampaignCriterion resource change.
        CampaignCriterion = 8,
        /// A Feed resource change.
        Feed = 9,
        /// A FeedItem resource change.
        FeedItem = 10,
        /// A CampaignFeed resource change.
        CampaignFeed = 11,
        /// An AdGroupFeed resource change.
        AdGroupFeed = 12,
        /// An AdGroupAd resource change.
        AdGroupAd = 13,
    }
}
// Proto file describing the change status operations.

/// Container for enum describing operations for the ChangeStatus resource.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangeStatusOperationEnum {}
/// Nested message and enum types in `ChangeStatusOperationEnum`.
pub mod change_status_operation_enum {
    /// Status of the changed resource
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ChangeStatusOperation {
        /// No value has been specified.
        Unspecified = 0,
        /// Used for return value only. Represents an unclassified resource unknown
        /// in this version.
        Unknown = 1,
        /// The resource was created.
        Added = 2,
        /// The resource was modified.
        Changed = 3,
        /// The resource was removed.
        Removed = 4,
    }
}
// Proto file describing the resource types the ChangeStatus resource supports.

/// Container for enum describing supported resource types for the ChangeStatus
/// resource.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangeStatusResourceTypeEnum {}
/// Nested message and enum types in `ChangeStatusResourceTypeEnum`.
pub mod change_status_resource_type_enum {
    /// Enum listing the resource types support by the ChangeStatus resource.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ChangeStatusResourceType {
        /// No value has been specified.
        Unspecified = 0,
        /// Used for return value only. Represents an unclassified resource unknown
        /// in this version.
        Unknown = 1,
        /// An AdGroup resource change.
        AdGroup = 3,
        /// An AdGroupAd resource change.
        AdGroupAd = 4,
        /// An AdGroupCriterion resource change.
        AdGroupCriterion = 5,
        /// A Campaign resource change.
        Campaign = 6,
        /// A CampaignCriterion resource change.
        CampaignCriterion = 7,
        /// A Feed resource change.
        Feed = 9,
        /// A FeedItem resource change.
        FeedItem = 10,
        /// An AdGroupFeed resource change.
        AdGroupFeed = 11,
        /// A CampaignFeed resource change.
        CampaignFeed = 12,
        /// An AdGroupBidModifier resource change.
        AdGroupBidModifier = 13,
    }
}
// Proto file describing combined audience status.

/// The status of combined audience.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CombinedAudienceStatusEnum {}
/// Nested message and enum types in `CombinedAudienceStatusEnum`.
pub mod combined_audience_status_enum {
    /// Enum containing possible combined audience status types.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum CombinedAudienceStatus {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Enabled status - combined audience is enabled and can be targeted.
        Enabled = 2,
        /// Removed status - combined audience is removed and cannot be used for
        /// targeting.
        Removed = 3,
    }
}
// Proto file describing conversion action counting type.

/// Container for enum describing the conversion deduplication mode for
/// conversion optimizer.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConversionActionCountingTypeEnum {}
/// Nested message and enum types in `ConversionActionCountingTypeEnum`.
pub mod conversion_action_counting_type_enum {
    /// Indicates how conversions for this action will be counted. For more
    /// information, see <https://support.google.com/google-ads/answer/3438531.>
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ConversionActionCountingType {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Count only one conversion per click.
        OnePerClick = 2,
        /// Count all conversions per click.
        ManyPerClick = 3,
    }
}
// Proto file describing conversion action status.

/// Container for enum describing possible statuses of a conversion action.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConversionActionStatusEnum {}
/// Nested message and enum types in `ConversionActionStatusEnum`.
pub mod conversion_action_status_enum {
    /// Possible statuses of a conversion action.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ConversionActionStatus {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Conversions will be recorded.
        Enabled = 2,
        /// Conversions will not be recorded.
        Removed = 3,
        /// Conversions will not be recorded and the conversion action will not
        /// appear in the UI.
        Hidden = 4,
    }
}
// Proto file describing conversion action type.

/// Container for enum describing possible types of a conversion action.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConversionActionTypeEnum {}
/// Nested message and enum types in `ConversionActionTypeEnum`.
pub mod conversion_action_type_enum {
    /// Possible types of a conversion action.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ConversionActionType {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Conversions that occur when a user clicks on an ad's call extension.
        AdCall = 2,
        /// Conversions that occur when a user on a mobile device clicks a phone
        /// number.
        ClickToCall = 3,
        /// Conversions that occur when a user downloads a mobile app from the Google
        /// Play Store.
        GooglePlayDownload = 4,
        /// Conversions that occur when a user makes a purchase in an app through
        /// Android billing.
        GooglePlayInAppPurchase = 5,
        /// Call conversions that are tracked by the advertiser and uploaded.
        UploadCalls = 6,
        /// Conversions that are tracked by the advertiser and uploaded with
        /// attributed clicks.
        UploadClicks = 7,
        /// Conversions that occur on a webpage.
        Webpage = 8,
        /// Conversions that occur when a user calls a dynamically-generated phone
        /// number from an advertiser's website.
        WebsiteCall = 9,
        /// Store Sales conversion based on first-party or third-party merchant
        /// data uploads.
        /// Only customers on the allowlist can use store sales direct upload types.
        StoreSalesDirectUpload = 10,
        /// Store Sales conversion based on first-party or third-party merchant
        /// data uploads and/or from in-store purchases using cards from payment
        /// networks.
        /// Only customers on the allowlist can use store sales types.
        /// Read only.
        StoreSales = 11,
        /// Android app first open conversions tracked via Firebase.
        FirebaseAndroidFirstOpen = 12,
        /// Android app in app purchase conversions tracked via Firebase.
        FirebaseAndroidInAppPurchase = 13,
        /// Android app custom conversions tracked via Firebase.
        FirebaseAndroidCustom = 14,
        /// iOS app first open conversions tracked via Firebase.
        FirebaseIosFirstOpen = 15,
        /// iOS app in app purchase conversions tracked via Firebase.
        FirebaseIosInAppPurchase = 16,
        /// iOS app custom conversions tracked via Firebase.
        FirebaseIosCustom = 17,
        /// Android app first open conversions tracked via Third Party App Analytics.
        ThirdPartyAppAnalyticsAndroidFirstOpen = 18,
        /// Android app in app purchase conversions tracked via Third Party App
        /// Analytics.
        ThirdPartyAppAnalyticsAndroidInAppPurchase = 19,
        /// Android app custom conversions tracked via Third Party App Analytics.
        ThirdPartyAppAnalyticsAndroidCustom = 20,
        /// iOS app first open conversions tracked via Third Party App Analytics.
        ThirdPartyAppAnalyticsIosFirstOpen = 21,
        /// iOS app in app purchase conversions tracked via Third Party App
        /// Analytics.
        ThirdPartyAppAnalyticsIosInAppPurchase = 22,
        /// iOS app custom conversions tracked via Third Party App Analytics.
        ThirdPartyAppAnalyticsIosCustom = 23,
        /// Conversions that occur when a user pre-registers a mobile app from the
        /// Google Play Store. Read only.
        AndroidAppPreRegistration = 24,
        /// Conversions that track all Google Play downloads which aren't tracked
        /// by an app-specific type. Read only.
        AndroidInstallsAllOtherApps = 25,
        /// Floodlight activity that counts the number of times that users have
        /// visited a particular webpage after seeing or clicking on one of
        /// an advertiser's ads. Read only.
        FloodlightAction = 26,
        /// Floodlight activity that tracks the number of sales made or the number
        /// of items purchased. Can also capture the total value of each sale.
        /// Read only.
        FloodlightTransaction = 27,
        /// Conversions that track local actions from Google's products and
        /// services after interacting with an ad. Read only.
        GoogleHosted = 28,
        /// Conversions reported when a user submits a lead form. Read only.
        LeadFormSubmit = 29,
        /// Conversions that come from Salesforce. Read only.
        Salesforce = 30,
        /// Conversions imported from Search Ads 360 Floodlight data. Read only.
        SearchAds360 = 31,
        /// Call conversions that occur on Smart campaign Ads without call tracking
        /// setup, using Smart campaign custom criteria. Read only.
        SmartCampaignAdClicksToCall = 32,
        /// The user clicks on a call element within Google Maps. Smart campaign
        /// only. Read only.
        SmartCampaignMapClicksToCall = 33,
        /// The user requests directions to a business location within Google Maps.
        /// Smart campaign only. Read only.
        SmartCampaignMapDirections = 34,
        /// Call conversions that occur on Smart campaign Ads with call tracking
        /// setup, using Smart campaign custom criteria. Read only.
        SmartCampaignTrackedCalls = 35,
        /// Conversions that occur when a user visits an advertiser's retail store.
        /// Read only.
        StoreVisits = 36,
    }
}
// Proto file describing conversion adjustment type.

/// Container for enum describing conversion adjustment types.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConversionAdjustmentTypeEnum {}
/// Nested message and enum types in `ConversionAdjustmentTypeEnum`.
pub mod conversion_adjustment_type_enum {
    /// The different actions advertisers can take to adjust the conversions that
    /// they already reported. Retractions negate a conversion. Restatements change
    /// the value of a conversion.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ConversionAdjustmentType {
        /// Not specified.
        Unspecified = 0,
        /// Represents value unknown in this version.
        Unknown = 1,
        /// Negates a conversion so that its total value and count are both zero.
        Retraction = 2,
        /// Changes the value of a conversion.
        Restatement = 3,
    }
}
// Proto file describing conversion custom variable status.

/// Container for enum describing possible statuses of a conversion custom
/// variable.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConversionCustomVariableStatusEnum {}
/// Nested message and enum types in `ConversionCustomVariableStatusEnum`.
pub mod conversion_custom_variable_status_enum {
    /// Possible statuses of a conversion custom variable.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ConversionCustomVariableStatus {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// The conversion custom variable is pending activation and will not
        /// accrue stats until set to ENABLED.
        ///
        /// This status can't be used in CREATE and UPDATE requests.
        ActivationNeeded = 2,
        /// The conversion custom variable is enabled and will accrue stats.
        Enabled = 3,
        /// The conversion custom variable is paused and will not accrue stats
        /// until set to ENABLED again.
        Paused = 4,
    }
}
// Proto file describing approval status for the criterion.

/// Container for enum describing possible criterion system serving statuses.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CriterionSystemServingStatusEnum {}
/// Nested message and enum types in `CriterionSystemServingStatusEnum`.
pub mod criterion_system_serving_status_enum {
    /// Enumerates criterion system serving statuses.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum CriterionSystemServingStatus {
        /// Not specified.
        Unspecified = 0,
        /// The value is unknown in this version.
        Unknown = 1,
        /// Eligible.
        Eligible = 2,
        /// Low search volume.
        RarelyServed = 3,
    }
}
// Proto file describing criteria types.

/// The possible types of a criterion.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CriterionTypeEnum {}
/// Nested message and enum types in `CriterionTypeEnum`.
pub mod criterion_type_enum {
    /// Enum describing possible criterion types.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum CriterionType {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Keyword. e.g. 'mars cruise'.
        Keyword = 2,
        /// Placement, aka Website. e.g. 'www.flowers4sale.com'
        Placement = 3,
        /// Mobile application categories to target.
        MobileAppCategory = 4,
        /// Mobile applications to target.
        MobileApplication = 5,
        /// Devices to target.
        Device = 6,
        /// Locations to target.
        Location = 7,
        /// Listing groups to target.
        ListingGroup = 8,
        /// Ad Schedule.
        AdSchedule = 9,
        /// Age range.
        AgeRange = 10,
        /// Gender.
        Gender = 11,
        /// Income Range.
        IncomeRange = 12,
        /// Parental status.
        ParentalStatus = 13,
        /// YouTube Video.
        YoutubeVideo = 14,
        /// YouTube Channel.
        YoutubeChannel = 15,
        /// User list.
        UserList = 16,
        /// Proximity.
        Proximity = 17,
        /// A topic target on the display network (e.g. "Pets & Animals").
        Topic = 18,
        /// Listing scope to target.
        ListingScope = 19,
        /// Language.
        Language = 20,
        /// IpBlock.
        IpBlock = 21,
        /// Content Label for category exclusion.
        ContentLabel = 22,
        /// Carrier.
        Carrier = 23,
        /// A category the user is interested in.
        UserInterest = 24,
        /// Webpage criterion for dynamic search ads.
        Webpage = 25,
        /// Operating system version.
        OperatingSystemVersion = 26,
        /// App payment model.
        AppPaymentModel = 27,
        /// Mobile device.
        MobileDevice = 28,
        /// Custom affinity.
        CustomAffinity = 29,
        /// Custom intent.
        CustomIntent = 30,
        /// Location group.
        LocationGroup = 31,
        /// Custom audience
        CustomAudience = 32,
        /// Combined audience
        CombinedAudience = 33,
    }
}
// Proto file describing custom audience member type.

/// The type of custom audience member.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomAudienceMemberTypeEnum {}
/// Nested message and enum types in `CustomAudienceMemberTypeEnum`.
pub mod custom_audience_member_type_enum {
    /// Enum containing possible custom audience member types.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum CustomAudienceMemberType {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Users whose interests or actions are described by a keyword.
        Keyword = 2,
        /// Users who have interests related to the website's content.
        Url = 3,
        /// Users who visit place types described by a place category.
        PlaceCategory = 4,
        /// Users who have installed a mobile app.
        App = 5,
    }
}
// Proto file describing custom audience status.

/// The status of custom audience.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomAudienceStatusEnum {}
/// Nested message and enum types in `CustomAudienceStatusEnum`.
pub mod custom_audience_status_enum {
    /// Enum containing possible custom audience statuses.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum CustomAudienceStatus {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Enabled status - custom audience is enabled and can be targeted.
        Enabled = 2,
        /// Removed status - custom audience is removed and cannot be used for
        /// targeting.
        Removed = 3,
    }
}
// Proto file describing custom audience type.

/// The types of custom audience.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomAudienceTypeEnum {}
/// Nested message and enum types in `CustomAudienceTypeEnum`.
pub mod custom_audience_type_enum {
    /// Enum containing possible custom audience types.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum CustomAudienceType {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Google Ads will auto-select the best interpretation at serving
        /// time.
        Auto = 2,
        /// Matches users by their interests.
        Interest = 3,
        /// Matches users by topics they are researching or products they are
        /// considering for purchase.
        PurchaseIntent = 4,
        /// Matches users by what they searched on Google Search.
        Search = 5,
    }
}
// Proto file describing custom interest member type.

/// The types of custom interest member, either KEYWORD or URL.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomInterestMemberTypeEnum {}
/// Nested message and enum types in `CustomInterestMemberTypeEnum`.
pub mod custom_interest_member_type_enum {
    /// Enum containing possible custom interest member types.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum CustomInterestMemberType {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Custom interest member type KEYWORD.
        Keyword = 2,
        /// Custom interest member type URL.
        Url = 3,
    }
}
// Proto file describing custom interest status.

/// The status of custom interest.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomInterestStatusEnum {}
/// Nested message and enum types in `CustomInterestStatusEnum`.
pub mod custom_interest_status_enum {
    /// Enum containing possible custom interest types.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum CustomInterestStatus {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Enabled status - custom interest is enabled and can be targeted to.
        Enabled = 2,
        /// Removed status - custom interest is removed and cannot be used for
        /// targeting.
        Removed = 3,
    }
}
// Proto file describing custom interest type.

/// The types of custom interest.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomInterestTypeEnum {}
/// Nested message and enum types in `CustomInterestTypeEnum`.
pub mod custom_interest_type_enum {
    /// Enum containing possible custom interest types.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum CustomInterestType {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Allows brand advertisers to define custom affinity audience lists.
        CustomAffinity = 2,
        /// Allows advertisers to define custom intent audience lists.
        CustomIntent = 3,
    }
}
// Proto file describing Custom placeholder fields.

/// Values for Custom placeholder fields.
/// For more information about dynamic remarketing feeds, see
/// <https://support.google.com/google-ads/answer/6053288.>
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomPlaceholderFieldEnum {}
/// Nested message and enum types in `CustomPlaceholderFieldEnum`.
pub mod custom_placeholder_field_enum {
    /// Possible values for Custom placeholder fields.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum CustomPlaceholderField {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Data Type: STRING. Required. Combination ID and ID2 must be unique per
        /// offer.
        Id = 2,
        /// Data Type: STRING. Combination ID and ID2 must be unique per offer.
        Id2 = 3,
        /// Data Type: STRING. Required. Main headline with product name to be shown
        /// in dynamic ad.
        ItemTitle = 4,
        /// Data Type: STRING. Optional text to be shown in the image ad.
        ItemSubtitle = 5,
        /// Data Type: STRING. Optional description of the product to be shown in the
        /// ad.
        ItemDescription = 6,
        /// Data Type: STRING. Full address of your offer or service, including
        /// postal code. This will be used to identify the closest product to the
        /// user when there are multiple offers in the feed that are relevant to the
        /// user.
        ItemAddress = 7,
        /// Data Type: STRING. Price to be shown in the ad.
        /// Example: "100.00 USD"
        Price = 8,
        /// Data Type: STRING. Formatted price to be shown in the ad.
        /// Example: "Starting at $100.00 USD", "$80 - $100"
        FormattedPrice = 9,
        /// Data Type: STRING. Sale price to be shown in the ad.
        /// Example: "80.00 USD"
        SalePrice = 10,
        /// Data Type: STRING. Formatted sale price to be shown in the ad.
        /// Example: "On sale for $80.00", "$60 - $80"
        FormattedSalePrice = 11,
        /// Data Type: URL. Image to be displayed in the ad. Highly recommended for
        /// image ads.
        ImageUrl = 12,
        /// Data Type: STRING. Used as a recommendation engine signal to serve items
        /// in the same category.
        ItemCategory = 13,
        /// Data Type: URL_LIST. Final URLs for the ad when using Upgraded
        /// URLs. User will be redirected to these URLs when they click on an ad, or
        /// when they click on a specific product for ads that have multiple
        /// products.
        FinalUrls = 14,
        /// Data Type: URL_LIST. Final mobile URLs for the ad when using Upgraded
        /// URLs.
        FinalMobileUrls = 15,
        /// Data Type: URL. Tracking template for the ad when using Upgraded URLs.
        TrackingUrl = 16,
        /// Data Type: STRING_LIST. Keywords used for product retrieval.
        ContextualKeywords = 17,
        /// Data Type: STRING. Android app link. Must be formatted as:
        /// android-app://{package_id}/{scheme}/{host_path}.
        /// The components are defined as follows:
        /// package_id: app ID as specified in Google Play.
        /// scheme: the scheme to pass to the application. Can be HTTP, or a custom
        ///   scheme.
        /// host_path: identifies the specific content within your application.
        AndroidAppLink = 18,
        /// Data Type: STRING_LIST. List of recommended IDs to show together with
        /// this item.
        SimilarIds = 19,
        /// Data Type: STRING. iOS app link.
        IosAppLink = 20,
        /// Data Type: INT64. iOS app store ID.
        IosAppStoreId = 21,
    }
}
// Proto file describing pay per conversion eligibility failure reasons.

/// Container for enum describing reasons why a customer is not eligible to use
/// PaymentMode.CONVERSIONS.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomerPayPerConversionEligibilityFailureReasonEnum {}
/// Nested message and enum types in `CustomerPayPerConversionEligibilityFailureReasonEnum`.
pub mod customer_pay_per_conversion_eligibility_failure_reason_enum {
    /// Enum describing possible reasons a customer is not eligible to use
    /// PaymentMode.CONVERSIONS.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum CustomerPayPerConversionEligibilityFailureReason {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Customer does not have enough conversions.
        NotEnoughConversions = 2,
        /// Customer's conversion lag is too high.
        ConversionLagTooHigh = 3,
        /// Customer uses shared budgets.
        HasCampaignWithSharedBudget = 4,
        /// Customer has conversions with ConversionActionType.UPLOAD_CLICKS.
        HasUploadClicksConversion = 5,
        /// Customer's average daily spend is too high.
        AverageDailySpendTooHigh = 6,
        /// Customer's eligibility has not yet been calculated by the Google Ads
        /// backend. Check back soon.
        AnalysisNotComplete = 7,
        /// Customer is not eligible due to other reasons.
        Other = 8,
    }
}
// Proto file describing data-driven model status.

/// Container for enum indicating data driven model status.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DataDrivenModelStatusEnum {}
/// Nested message and enum types in `DataDrivenModelStatusEnum`.
pub mod data_driven_model_status_enum {
    /// Enumerates data driven model statuses.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum DataDrivenModelStatus {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// The data driven model is available.
        Available = 2,
        /// The data driven model is stale. It hasn't been updated for at least 7
        /// days. It is still being used, but will become expired if it does not get
        /// updated for 30 days.
        Stale = 3,
        /// The data driven model expired. It hasn't been updated for at least 30
        /// days and cannot be used. Most commonly this is because there hasn't been
        /// the required number of events in a recent 30-day period.
        Expired = 4,
        /// The data driven model has never been generated. Most commonly this is
        /// because there has never been the required number of events in any 30-day
        /// period.
        NeverGenerated = 5,
    }
}
// Proto file describing distance buckets.

/// Container for distance buckets of a user’s distance from an advertiser’s
/// location extension.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DistanceBucketEnum {}
/// Nested message and enum types in `DistanceBucketEnum`.
pub mod distance_bucket_enum {
    /// The distance bucket for a user’s distance from an advertiser’s location
    /// extension.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum DistanceBucket {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// User was within 700m of the location.
        Within700m = 2,
        /// User was within 1KM of the location.
        Within1km = 3,
        /// User was within 5KM of the location.
        Within5km = 4,
        /// User was within 10KM of the location.
        Within10km = 5,
        /// User was within 15KM of the location.
        Within15km = 6,
        /// User was within 20KM of the location.
        Within20km = 7,
        /// User was within 25KM of the location.
        Within25km = 8,
        /// User was within 30KM of the location.
        Within30km = 9,
        /// User was within 35KM of the location.
        Within35km = 10,
        /// User was within 40KM of the location.
        Within40km = 11,
        /// User was within 45KM of the location.
        Within45km = 12,
        /// User was within 50KM of the location.
        Within50km = 13,
        /// User was within 55KM of the location.
        Within55km = 14,
        /// User was within 60KM of the location.
        Within60km = 15,
        /// User was within 65KM of the location.
        Within65km = 16,
        /// User was beyond 65KM of the location.
        Beyond65km = 17,
        /// User was within 0.7 miles of the location.
        Within07miles = 18,
        /// User was within 1 mile of the location.
        Within1mile = 19,
        /// User was within 5 miles of the location.
        Within5miles = 20,
        /// User was within 10 miles of the location.
        Within10miles = 21,
        /// User was within 15 miles of the location.
        Within15miles = 22,
        /// User was within 20 miles of the location.
        Within20miles = 23,
        /// User was within 25 miles of the location.
        Within25miles = 24,
        /// User was within 30 miles of the location.
        Within30miles = 25,
        /// User was within 35 miles of the location.
        Within35miles = 26,
        /// User was within 40 miles of the location.
        Within40miles = 27,
        /// User was beyond 40 miles of the location.
        Beyond40miles = 28,
    }
}
// Proto file describing Dynamic Search Ad Page Feed criterion fields.

/// Values for Dynamic Search Ad Page Feed criterion fields.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DsaPageFeedCriterionFieldEnum {}
/// Nested message and enum types in `DsaPageFeedCriterionFieldEnum`.
pub mod dsa_page_feed_criterion_field_enum {
    /// Possible values for Dynamic Search Ad Page Feed criterion fields.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum DsaPageFeedCriterionField {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Data Type: URL or URL_LIST. URL of the web page you want to target.
        PageUrl = 2,
        /// Data Type: STRING_LIST. The labels that will help you target ads within
        /// your page feed.
        Label = 3,
    }
}
// Proto file describing Education placeholder fields.

/// Values for Education placeholder fields.
/// For more information about dynamic remarketing feeds, see
/// <https://support.google.com/google-ads/answer/6053288.>
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EducationPlaceholderFieldEnum {}
/// Nested message and enum types in `EducationPlaceholderFieldEnum`.
pub mod education_placeholder_field_enum {
    /// Possible values for Education placeholder fields.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum EducationPlaceholderField {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Data Type: STRING. Required. Combination of PROGRAM ID and LOCATION ID
        /// must be unique per offer.
        ProgramId = 2,
        /// Data Type: STRING. Combination of PROGRAM ID and LOCATION ID must be
        /// unique per offer.
        LocationId = 3,
        /// Data Type: STRING. Required. Main headline with program name to be shown
        /// in dynamic ad.
        ProgramName = 4,
        /// Data Type: STRING. Area of study that can be shown in dynamic ad.
        AreaOfStudy = 5,
        /// Data Type: STRING. Description of program that can be shown in dynamic
        /// ad.
        ProgramDescription = 6,
        /// Data Type: STRING. Name of school that can be shown in dynamic ad.
        SchoolName = 7,
        /// Data Type: STRING. Complete school address, including postal code.
        Address = 8,
        /// Data Type: URL. Image to be displayed in ads.
        ThumbnailImageUrl = 9,
        /// Data Type: URL. Alternative hosted file of image to be used in the ad.
        AlternativeThumbnailImageUrl = 10,
        /// Data Type: URL_LIST. Required. Final URLs to be used in ad when using
        /// Upgraded URLs; the more specific the better (e.g. the individual URL of a
        /// specific program and its location).
        FinalUrls = 11,
        /// Data Type: URL_LIST. Final mobile URLs for the ad when using Upgraded
        /// URLs.
        FinalMobileUrls = 12,
        /// Data Type: URL. Tracking template for the ad when using Upgraded URLs.
        TrackingUrl = 13,
        /// Data Type: STRING_LIST. Keywords used for product retrieval.
        ContextualKeywords = 14,
        /// Data Type: STRING. Android app link. Must be formatted as:
        /// android-app://{package_id}/{scheme}/{host_path}.
        /// The components are defined as follows:
        /// package_id: app ID as specified in Google Play.
        /// scheme: the scheme to pass to the application. Can be HTTP, or a custom
        ///   scheme.
        /// host_path: identifies the specific content within your application.
        AndroidAppLink = 15,
        /// Data Type: STRING_LIST. List of recommended program IDs to show together
        /// with this item.
        SimilarProgramIds = 16,
        /// Data Type: STRING. iOS app link.
        IosAppLink = 17,
        /// Data Type: INT64. iOS app store ID.
        IosAppStoreId = 18,
    }
}
// Proto file describing extension setting device type.

/// Container for enum describing extension setting device types.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExtensionSettingDeviceEnum {}
/// Nested message and enum types in `ExtensionSettingDeviceEnum`.
pub mod extension_setting_device_enum {
    /// Possible device types for an extension setting.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ExtensionSettingDevice {
        /// Not specified.
        Unspecified = 0,
        /// The value is unknown in this version.
        Unknown = 1,
        /// Mobile. The extensions in the extension setting will only serve on
        /// mobile devices.
        Mobile = 2,
        /// Desktop. The extensions in the extension setting will only serve on
        /// desktop devices.
        Desktop = 3,
    }
}
// Proto file describing extension type.

/// Container for enum describing possible data types for an extension in an
/// extension setting.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExtensionTypeEnum {}
/// Nested message and enum types in `ExtensionTypeEnum`.
pub mod extension_type_enum {
    /// Possible data types for an extension in an extension setting.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ExtensionType {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// None.
        None = 2,
        /// App.
        App = 3,
        /// Call.
        Call = 4,
        /// Callout.
        Callout = 5,
        /// Message.
        Message = 6,
        /// Price.
        Price = 7,
        /// Promotion.
        Promotion = 8,
        /// Sitelink.
        Sitelink = 10,
        /// Structured snippet.
        StructuredSnippet = 11,
        /// Location.
        Location = 12,
        /// Affiliate location.
        AffiliateLocation = 13,
        /// Hotel callout
        HotelCallout = 15,
        /// Image.
        Image = 16,
    }
}
// Proto file describing feed attribute type.

/// Container for enum describing possible data types for a feed attribute.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeedAttributeTypeEnum {}
/// Nested message and enum types in `FeedAttributeTypeEnum`.
pub mod feed_attribute_type_enum {
    /// Possible data types for a feed attribute.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum FeedAttributeType {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Int64.
        Int64 = 2,
        /// Double.
        Double = 3,
        /// String.
        String = 4,
        /// Boolean.
        Boolean = 5,
        /// Url.
        Url = 6,
        /// Datetime.
        DateTime = 7,
        /// Int64 list.
        Int64List = 8,
        /// Double (8 bytes) list.
        DoubleList = 9,
        /// String list.
        StringList = 10,
        /// Boolean list.
        BooleanList = 11,
        /// Url list.
        UrlList = 12,
        /// Datetime list.
        DateTimeList = 13,
        /// Price.
        Price = 14,
    }
}
// Proto file describing feed item quality evaluation approval statuses.

/// Container for enum describing possible quality evaluation approval statuses
/// of a feed item.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeedItemQualityApprovalStatusEnum {}
/// Nested message and enum types in `FeedItemQualityApprovalStatusEnum`.
pub mod feed_item_quality_approval_status_enum {
    /// The possible quality evaluation approval statuses of a feed item.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum FeedItemQualityApprovalStatus {
        /// No value has been specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Meets all quality expectations.
        Approved = 2,
        /// Does not meet some quality expectations. The specific reason is found in
        /// the quality_disapproval_reasons field.
        Disapproved = 3,
    }
}
// Proto file describing feed item quality disapproval reasons.

/// Container for enum describing possible quality evaluation disapproval reasons
/// of a feed item.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeedItemQualityDisapprovalReasonEnum {}
/// Nested message and enum types in `FeedItemQualityDisapprovalReasonEnum`.
pub mod feed_item_quality_disapproval_reason_enum {
    /// The possible quality evaluation disapproval reasons of a feed item.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum FeedItemQualityDisapprovalReason {
        /// No value has been specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Price contains repetitive headers.
        PriceTableRepetitiveHeaders = 2,
        /// Price contains repetitive description.
        PriceTableRepetitiveDescription = 3,
        /// Price contains inconsistent items.
        PriceTableInconsistentRows = 4,
        /// Price contains qualifiers in description.
        PriceDescriptionHasPriceQualifiers = 5,
        /// Price contains an unsupported language.
        PriceUnsupportedLanguage = 6,
        /// Price item header is not relevant to the price type.
        PriceTableRowHeaderTableTypeMismatch = 7,
        /// Price item header has promotional text.
        PriceTableRowHeaderHasPromotionalText = 8,
        /// Price item description is not relevant to the item header.
        PriceTableRowDescriptionNotRelevant = 9,
        /// Price item description contains promotional text.
        PriceTableRowDescriptionHasPromotionalText = 10,
        /// Price item header and description are repetitive.
        PriceTableRowHeaderDescriptionRepetitive = 11,
        /// Price item is in a foreign language, nonsense, or can't be rated.
        PriceTableRowUnrateable = 12,
        /// Price item price is invalid or inaccurate.
        PriceTableRowPriceInvalid = 13,
        /// Price item URL is invalid or irrelevant.
        PriceTableRowUrlInvalid = 14,
        /// Price item header or description has price.
        PriceHeaderOrDescriptionHasPrice = 15,
        /// Structured snippet values do not match the header.
        StructuredSnippetsHeaderPolicyViolated = 16,
        /// Structured snippet values are repeated.
        StructuredSnippetsRepeatedValues = 17,
        /// Structured snippet values violate editorial guidelines like punctuation.
        StructuredSnippetsEditorialGuidelines = 18,
        /// Structured snippet contain promotional text.
        StructuredSnippetsHasPromotionalText = 19,
    }
}
// Proto file describing feed item set status.

/// Container for enum describing possible statuses of a feed item set.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeedItemSetStatusEnum {}
/// Nested message and enum types in `FeedItemSetStatusEnum`.
pub mod feed_item_set_status_enum {
    /// Possible statuses of a feed item set.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum FeedItemSetStatus {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Feed item set is enabled.
        Enabled = 2,
        /// Feed item set has been removed.
        Removed = 3,
    }
}
// Proto file describing feed item status.

/// Container for enum describing possible statuses of a feed item.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeedItemStatusEnum {}
/// Nested message and enum types in `FeedItemStatusEnum`.
pub mod feed_item_status_enum {
    /// Possible statuses of a feed item.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum FeedItemStatus {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Feed item is enabled.
        Enabled = 2,
        /// Feed item has been removed.
        Removed = 3,
    }
}
// Proto file describing feed item target device type.

/// Container for enum describing possible data types for a feed item target
/// device.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeedItemTargetDeviceEnum {}
/// Nested message and enum types in `FeedItemTargetDeviceEnum`.
pub mod feed_item_target_device_enum {
    /// Possible data types for a feed item target device.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum FeedItemTargetDevice {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Mobile.
        Mobile = 2,
    }
}
// Proto file describing feed item target status.

/// Container for enum describing possible statuses of a feed item target.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeedItemTargetStatusEnum {}
/// Nested message and enum types in `FeedItemTargetStatusEnum`.
pub mod feed_item_target_status_enum {
    /// Possible statuses of a feed item target.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum FeedItemTargetStatus {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Feed item target is enabled.
        Enabled = 2,
        /// Feed item target has been removed.
        Removed = 3,
    }
}
// Proto file describing feed item target type status.

/// Container for enum describing possible types of a feed item target.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeedItemTargetTypeEnum {}
/// Nested message and enum types in `FeedItemTargetTypeEnum`.
pub mod feed_item_target_type_enum {
    /// Possible type of a feed item target.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum FeedItemTargetType {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Feed item targets a campaign.
        Campaign = 2,
        /// Feed item targets an ad group.
        AdGroup = 3,
        /// Feed item targets a criterion.
        Criterion = 4,
    }
}
// Proto file describing feed item validation statuses.

/// Container for enum describing possible validation statuses of a feed item.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeedItemValidationStatusEnum {}
/// Nested message and enum types in `FeedItemValidationStatusEnum`.
pub mod feed_item_validation_status_enum {
    /// The possible validation statuses of a feed item.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum FeedItemValidationStatus {
        /// No value has been specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Validation pending.
        Pending = 2,
        /// An error was found.
        Invalid = 3,
        /// Feed item is semantically well-formed.
        Valid = 4,
    }
}
// Proto file describing status of a feed link.

/// Container for an enum describing possible statuses of a feed link.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeedLinkStatusEnum {}
/// Nested message and enum types in `FeedLinkStatusEnum`.
pub mod feed_link_status_enum {
    /// Possible statuses of a feed link.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum FeedLinkStatus {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Feed link is enabled.
        Enabled = 2,
        /// Feed link has been removed.
        Removed = 3,
    }
}
// Proto file describing criterion types for feed mappings.

/// Container for enum describing possible criterion types for a feed mapping.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeedMappingCriterionTypeEnum {}
/// Nested message and enum types in `FeedMappingCriterionTypeEnum`.
pub mod feed_mapping_criterion_type_enum {
    /// Possible placeholder types for a feed mapping.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum FeedMappingCriterionType {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Allows campaign targeting at locations within a location feed.
        LocationExtensionTargeting = 4,
        /// Allows url targeting for your dynamic search ads within a page feed.
        DsaPageFeed = 3,
    }
}
// Proto file describing feed mapping status.

/// Container for enum describing possible statuses of a feed mapping.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeedMappingStatusEnum {}
/// Nested message and enum types in `FeedMappingStatusEnum`.
pub mod feed_mapping_status_enum {
    /// Possible statuses of a feed mapping.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum FeedMappingStatus {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Feed mapping is enabled.
        Enabled = 2,
        /// Feed mapping has been removed.
        Removed = 3,
    }
}
// Proto file describing feed origin.

/// Container for enum describing possible values for a feed origin.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeedOriginEnum {}
/// Nested message and enum types in `FeedOriginEnum`.
pub mod feed_origin_enum {
    /// Possible values for a feed origin.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum FeedOrigin {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// The FeedAttributes for this Feed are managed by the
        /// user. Users can add FeedAttributes to this Feed.
        User = 2,
        /// The FeedAttributes for an GOOGLE Feed are created by Google. A feed of
        /// this type is maintained by Google and will have the correct attributes
        /// for the placeholder type of the feed.
        Google = 3,
    }
}
// Proto file describing feed status.

/// Container for enum describing possible statuses of a feed.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeedStatusEnum {}
/// Nested message and enum types in `FeedStatusEnum`.
pub mod feed_status_enum {
    /// Possible statuses of a feed.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum FeedStatus {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Feed is enabled.
        Enabled = 2,
        /// Feed has been removed.
        Removed = 3,
    }
}
// Proto file describing Flight placeholder fields.

/// Values for Flight placeholder fields.
/// For more information about dynamic remarketing feeds, see
/// <https://support.google.com/google-ads/answer/6053288.>
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FlightPlaceholderFieldEnum {}
/// Nested message and enum types in `FlightPlaceholderFieldEnum`.
pub mod flight_placeholder_field_enum {
    /// Possible values for Flight placeholder fields.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum FlightPlaceholderField {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Data Type: STRING. Required. Destination id. Example: PAR, LON.
        /// For feed items that only have destination id, destination id must be a
        /// unique key. For feed items that have both destination id and origin id,
        /// then the combination must be a unique key.
        DestinationId = 2,
        /// Data Type: STRING. Origin id. Example: PAR, LON.
        /// Optional. Combination of destination id and origin id must be unique per
        /// offer.
        OriginId = 3,
        /// Data Type: STRING. Required. Main headline with product name to be shown
        /// in dynamic ad.
        FlightDescription = 4,
        /// Data Type: STRING. Shorter names are recommended.
        OriginName = 5,
        /// Data Type: STRING. Shorter names are recommended.
        DestinationName = 6,
        /// Data Type: STRING. Price to be shown in the ad.
        /// Example: "100.00 USD"
        FlightPrice = 7,
        /// Data Type: STRING. Formatted price to be shown in the ad.
        /// Example: "Starting at $100.00 USD", "$80 - $100"
        FormattedPrice = 8,
        /// Data Type: STRING. Sale price to be shown in the ad.
        /// Example: "80.00 USD"
        FlightSalePrice = 9,
        /// Data Type: STRING. Formatted sale price to be shown in the ad.
        /// Example: "On sale for $80.00", "$60 - $80"
        FormattedSalePrice = 10,
        /// Data Type: URL. Image to be displayed in the ad.
        ImageUrl = 11,
        /// Data Type: URL_LIST. Required. Final URLs for the ad when using Upgraded
        /// URLs. User will be redirected to these URLs when they click on an ad, or
        /// when they click on a specific flight for ads that show multiple
        /// flights.
        FinalUrls = 12,
        /// Data Type: URL_LIST. Final mobile URLs for the ad when using Upgraded
        /// URLs.
        FinalMobileUrls = 13,
        /// Data Type: URL. Tracking template for the ad when using Upgraded URLs.
        TrackingUrl = 14,
        /// Data Type: STRING. Android app link. Must be formatted as:
        /// android-app://{package_id}/{scheme}/{host_path}.
        /// The components are defined as follows:
        /// package_id: app ID as specified in Google Play.
        /// scheme: the scheme to pass to the application. Can be HTTP, or a custom
        ///   scheme.
        /// host_path: identifies the specific content within your application.
        AndroidAppLink = 15,
        /// Data Type: STRING_LIST. List of recommended destination IDs to show
        /// together with this item.
        SimilarDestinationIds = 16,
        /// Data Type: STRING. iOS app link.
        IosAppLink = 17,
        /// Data Type: INT64. iOS app store ID.
        IosAppStoreId = 18,
    }
}
// Proto file describing geo target constant statuses.

/// Container for describing the status of a geo target constant.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GeoTargetConstantStatusEnum {}
/// Nested message and enum types in `GeoTargetConstantStatusEnum`.
pub mod geo_target_constant_status_enum {
    /// The possible statuses of a geo target constant.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum GeoTargetConstantStatus {
        /// No value has been specified.
        Unspecified = 0,
        /// The received value is not known in this version.
        ///
        /// This is a response-only value.
        Unknown = 1,
        /// The geo target constant is valid.
        Enabled = 2,
        /// The geo target constant is obsolete and will be removed.
        RemovalPlanned = 3,
    }
}
// Proto file describing GeoTargetingRestriction.

/// Message describing feed item geo targeting restriction.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GeoTargetingRestrictionEnum {}
/// Nested message and enum types in `GeoTargetingRestrictionEnum`.
pub mod geo_targeting_restriction_enum {
    /// A restriction used to determine if the request context's
    /// geo should be matched.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum GeoTargetingRestriction {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Indicates that request context should match the physical location of
        /// the user.
        LocationOfPresence = 2,
    }
}
// Proto file describing geo targeting types.

/// Container for enum describing possible geo targeting types.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GeoTargetingTypeEnum {}
/// Nested message and enum types in `GeoTargetingTypeEnum`.
pub mod geo_targeting_type_enum {
    /// The possible geo targeting types.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum GeoTargetingType {
        /// Not specified.
        Unspecified = 0,
        /// The value is unknown in this version.
        Unknown = 1,
        /// Location the user is interested in while making the query.
        AreaOfInterest = 2,
        /// Location of the user issuing the query.
        LocationOfPresence = 3,
    }
}
// Proto file describing GoogleAdsField categories

/// Container for enum that determines if the described artifact is a resource
/// or a field, and if it is a field, when it segments search queries.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GoogleAdsFieldCategoryEnum {}
/// Nested message and enum types in `GoogleAdsFieldCategoryEnum`.
pub mod google_ads_field_category_enum {
    /// The category of the artifact.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum GoogleAdsFieldCategory {
        /// Unspecified
        Unspecified = 0,
        /// Unknown
        Unknown = 1,
        /// The described artifact is a resource.
        Resource = 2,
        /// The described artifact is a field and is an attribute of a resource.
        /// Including a resource attribute field in a query may segment the query if
        /// the resource to which it is attributed segments the resource found in
        /// the FROM clause.
        Attribute = 3,
        /// The described artifact is a field and always segments search queries.
        Segment = 5,
        /// The described artifact is a field and is a metric. It never segments
        /// search queries.
        Metric = 6,
    }
}
// Proto file describing GoogleAdsField data types

/// Container holding the various data types.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GoogleAdsFieldDataTypeEnum {}
/// Nested message and enum types in `GoogleAdsFieldDataTypeEnum`.
pub mod google_ads_field_data_type_enum {
    /// These are the various types a GoogleAdsService artifact may take on.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum GoogleAdsFieldDataType {
        /// Unspecified
        Unspecified = 0,
        /// Unknown
        Unknown = 1,
        /// Maps to google.protobuf.BoolValue
        ///
        /// Applicable operators:  =, !=
        Boolean = 2,
        /// Maps to google.protobuf.StringValue. It can be compared using the set of
        /// operators specific to dates however.
        ///
        /// Applicable operators:  =, <, >, <=, >=, BETWEEN, DURING, and IN
        Date = 3,
        /// Maps to google.protobuf.DoubleValue
        ///
        /// Applicable operators:  =, !=, <, >, IN, NOT IN
        Double = 4,
        /// Maps to an enum. It's specific definition can be found at type_url.
        ///
        /// Applicable operators:  =, !=, IN, NOT IN
        Enum = 5,
        /// Maps to google.protobuf.FloatValue
        ///
        /// Applicable operators:  =, !=, <, >, IN, NOT IN
        Float = 6,
        /// Maps to google.protobuf.Int32Value
        ///
        /// Applicable operators:  =, !=, <, >, <=, >=, BETWEEN, IN, NOT IN
        Int32 = 7,
        /// Maps to google.protobuf.Int64Value
        ///
        /// Applicable operators:  =, !=, <, >, <=, >=, BETWEEN, IN, NOT IN
        Int64 = 8,
        /// Maps to a protocol buffer message type. The data type's details can be
        /// found in type_url.
        ///
        /// No operators work with MESSAGE fields.
        Message = 9,
        /// Maps to google.protobuf.StringValue. Represents the resource name
        /// (unique id) of a resource or one of its foreign keys.
        ///
        /// No operators work with RESOURCE_NAME fields.
        ResourceName = 10,
        /// Maps to google.protobuf.StringValue.
        ///
        /// Applicable operators:  =, !=, LIKE, NOT LIKE, IN, NOT IN
        String = 11,
        /// Maps to google.protobuf.UInt64Value
        ///
        /// Applicable operators:  =, !=, <, >, <=, >=, BETWEEN, IN, NOT IN
        Uint64 = 12,
    }
}
// Proto file describing google voice call status.

/// Container for enum describing possible statuses of a google voice call.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GoogleVoiceCallStatusEnum {}
/// Nested message and enum types in `GoogleVoiceCallStatusEnum`.
pub mod google_voice_call_status_enum {
    /// Possible statuses of a google voice call.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum GoogleVoiceCallStatus {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// The call was missed.
        Missed = 2,
        /// The call was received.
        Received = 3,
    }
}
// Proto file describing Hotel placeholder fields.

/// Values for Hotel placeholder fields.
/// For more information about dynamic remarketing feeds, see
/// <https://support.google.com/google-ads/answer/6053288.>
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HotelPlaceholderFieldEnum {}
/// Nested message and enum types in `HotelPlaceholderFieldEnum`.
pub mod hotel_placeholder_field_enum {
    /// Possible values for Hotel placeholder fields.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum HotelPlaceholderField {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Data Type: STRING. Required. Unique ID.
        PropertyId = 2,
        /// Data Type: STRING. Required. Main headline with property name to be shown
        /// in dynamic ad.
        PropertyName = 3,
        /// Data Type: STRING. Name of destination to be shown in dynamic ad.
        DestinationName = 4,
        /// Data Type: STRING. Description of destination to be shown in dynamic ad.
        Description = 5,
        /// Data Type: STRING. Complete property address, including postal code.
        Address = 6,
        /// Data Type: STRING. Price to be shown in the ad.
        /// Example: "100.00 USD"
        Price = 7,
        /// Data Type: STRING. Formatted price to be shown in the ad.
        /// Example: "Starting at $100.00 USD", "$80 - $100"
        FormattedPrice = 8,
        /// Data Type: STRING. Sale price to be shown in the ad.
        /// Example: "80.00 USD"
        SalePrice = 9,
        /// Data Type: STRING. Formatted sale price to be shown in the ad.
        /// Example: "On sale for $80.00", "$60 - $80"
        FormattedSalePrice = 10,
        /// Data Type: URL. Image to be displayed in the ad.
        ImageUrl = 11,
        /// Data Type: STRING. Category of property used to group like items together
        /// for recommendation engine.
        Category = 12,
        /// Data Type: INT64. Star rating (1 to 5) used to group like items
        /// together for recommendation engine.
        StarRating = 13,
        /// Data Type: STRING_LIST. Keywords used for product retrieval.
        ContextualKeywords = 14,
        /// Data Type: URL_LIST. Required. Final URLs for the ad when using Upgraded
        /// URLs. User will be redirected to these URLs when they click on an ad, or
        /// when they click on a specific flight for ads that show multiple
        /// flights.
        FinalUrls = 15,
        /// Data Type: URL_LIST. Final mobile URLs for the ad when using Upgraded
        /// URLs.
        FinalMobileUrls = 16,
        /// Data Type: URL. Tracking template for the ad when using Upgraded URLs.
        TrackingUrl = 17,
        /// Data Type: STRING. Android app link. Must be formatted as:
        /// android-app://{package_id}/{scheme}/{host_path}.
        /// The components are defined as follows:
        /// package_id: app ID as specified in Google Play.
        /// scheme: the scheme to pass to the application. Can be HTTP, or a custom
        ///   scheme.
        /// host_path: identifies the specific content within your application.
        AndroidAppLink = 18,
        /// Data Type: STRING_LIST. List of recommended property IDs to show together
        /// with this item.
        SimilarPropertyIds = 19,
        /// Data Type: STRING. iOS app link.
        IosAppLink = 20,
        /// Data Type: INT64. iOS app store ID.
        IosAppStoreId = 21,
    }
}
// Proto file describing Advertiser Provided Image placeholder fields.

/// Values for Advertiser Provided Image placeholder fields.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImagePlaceholderFieldEnum {}
/// Nested message and enum types in `ImagePlaceholderFieldEnum`.
pub mod image_placeholder_field_enum {
    /// Possible values for Advertiser Provided Image placeholder fields.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ImagePlaceholderField {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Data Type: INT64. The asset ID of the image.
        AssetId = 2,
    }
}
// Proto file describing invoice types.

/// Container for enum describing the type of invoices.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InvoiceTypeEnum {}
/// Nested message and enum types in `InvoiceTypeEnum`.
pub mod invoice_type_enum {
    /// The possible type of invoices.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum InvoiceType {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// An invoice with a negative amount. The account receives a credit.
        CreditMemo = 2,
        /// An invoice with a positive amount. The account owes a balance.
        Invoice = 3,
    }
}
// Proto file describing Job placeholder fields.

/// Values for Job placeholder fields.
/// For more information about dynamic remarketing feeds, see
/// <https://support.google.com/google-ads/answer/6053288.>
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct JobPlaceholderFieldEnum {}
/// Nested message and enum types in `JobPlaceholderFieldEnum`.
pub mod job_placeholder_field_enum {
    /// Possible values for Job placeholder fields.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum JobPlaceholderField {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Data Type: STRING. Required. If only JOB_ID is specified, then it must be
        /// unique. If both JOB_ID and LOCATION_ID are specified, then the
        /// pair must be unique.
        /// ID) pair must be unique.
        JobId = 2,
        /// Data Type: STRING. Combination of JOB_ID and LOCATION_ID must be unique
        /// per offer.
        LocationId = 3,
        /// Data Type: STRING. Required. Main headline with job title to be shown in
        /// dynamic ad.
        Title = 4,
        /// Data Type: STRING. Job subtitle to be shown in dynamic ad.
        Subtitle = 5,
        /// Data Type: STRING. Description of job to be shown in dynamic ad.
        Description = 6,
        /// Data Type: URL. Image to be displayed in the ad. Highly recommended for
        /// image ads.
        ImageUrl = 7,
        /// Data Type: STRING. Category of property used to group like items together
        /// for recommendation engine.
        Category = 8,
        /// Data Type: STRING_LIST. Keywords used for product retrieval.
        ContextualKeywords = 9,
        /// Data Type: STRING. Complete property address, including postal code.
        Address = 10,
        /// Data Type: STRING. Salary or salary range of job to be shown in dynamic
        /// ad.
        Salary = 11,
        /// Data Type: URL_LIST. Required. Final URLs to be used in ad when using
        /// Upgraded URLs; the more specific the better (e.g. the individual URL of a
        /// specific job and its location).
        FinalUrls = 12,
        /// Data Type: URL_LIST. Final mobile URLs for the ad when using Upgraded
        /// URLs.
        FinalMobileUrls = 14,
        /// Data Type: URL. Tracking template for the ad when using Upgraded URLs.
        TrackingUrl = 15,
        /// Data Type: STRING. Android app link. Must be formatted as:
        /// android-app://{package_id}/{scheme}/{host_path}.
        /// The components are defined as follows:
        /// package_id: app ID as specified in Google Play.
        /// scheme: the scheme to pass to the application. Can be HTTP, or a custom
        ///   scheme.
        /// host_path: identifies the specific content within your application.
        AndroidAppLink = 16,
        /// Data Type: STRING_LIST. List of recommended job IDs to show together with
        /// this item.
        SimilarJobIds = 17,
        /// Data Type: STRING. iOS app link.
        IosAppLink = 18,
        /// Data Type: INT64. iOS app store ID.
        IosAppStoreId = 19,
    }
}
// Proto file describing keyword plan forecast intervals.

/// Container for enumeration of forecast intervals.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeywordPlanForecastIntervalEnum {}
/// Nested message and enum types in `KeywordPlanForecastIntervalEnum`.
pub mod keyword_plan_forecast_interval_enum {
    /// Forecast intervals.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum KeywordPlanForecastInterval {
        /// Not specified.
        Unspecified = 0,
        /// The value is unknown in this version.
        Unknown = 1,
        /// The next week date range for keyword plan. The next week is based
        /// on the default locale of the user's account and is mostly SUN-SAT or
        /// MON-SUN.
        /// This can be different from next-7 days.
        NextWeek = 3,
        /// The next month date range for keyword plan.
        NextMonth = 4,
        /// The next quarter date range for keyword plan.
        NextQuarter = 5,
    }
}
// Proto file describing Keyword Planner Keyword annotation types.

/// Container for enumeration of keyword plan keyword annotations.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeywordPlanKeywordAnnotationEnum {}
/// Nested message and enum types in `KeywordPlanKeywordAnnotationEnum`.
pub mod keyword_plan_keyword_annotation_enum {
    /// Enumerates keyword plan annotations that can be requested.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum KeywordPlanKeywordAnnotation {
        /// Not specified.
        Unspecified = 0,
        /// The value is unknown in this version.
        Unknown = 1,
        /// Return the keyword concept and concept group data.
        KeywordConcept = 2,
    }
}
// Proto file describing Keyword Planner forecastable network types.

/// Container for enumeration of keyword plan forecastable network types.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeywordPlanNetworkEnum {}
/// Nested message and enum types in `KeywordPlanNetworkEnum`.
pub mod keyword_plan_network_enum {
    /// Enumerates keyword plan forecastable network types.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum KeywordPlanNetwork {
        /// Not specified.
        Unspecified = 0,
        /// The value is unknown in this version.
        Unknown = 1,
        /// Google Search.
        GoogleSearch = 2,
        /// Google Search + Search partners.
        GoogleSearchAndPartners = 3,
    }
}
/// Container for enum describing possible status of a label.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LabelStatusEnum {}
/// Nested message and enum types in `LabelStatusEnum`.
pub mod label_status_enum {
    /// Possible statuses of a label.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum LabelStatus {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Label is enabled.
        Enabled = 2,
        /// Label is removed.
        Removed = 3,
    }
}
/// Container for enum describing different types of Linked accounts.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LinkedAccountTypeEnum {}
/// Nested message and enum types in `LinkedAccountTypeEnum`.
pub mod linked_account_type_enum {
    /// Describes the possible link types between a Google Ads customer
    /// and another account.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum LinkedAccountType {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// A link to provide third party app analytics data.
        ThirdPartyAppAnalytics = 2,
        /// A link to Data partner.
        DataPartner = 3,
        /// A link to Google Ads.
        GoogleAds = 4,
    }
}
// Proto file describing Local placeholder fields.

/// Values for Local placeholder fields.
/// For more information about dynamic remarketing feeds, see
/// <https://support.google.com/google-ads/answer/6053288.>
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LocalPlaceholderFieldEnum {}
/// Nested message and enum types in `LocalPlaceholderFieldEnum`.
pub mod local_placeholder_field_enum {
    /// Possible values for Local placeholder fields.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum LocalPlaceholderField {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Data Type: STRING. Required. Unique ID.
        DealId = 2,
        /// Data Type: STRING. Required. Main headline with local deal title to be
        /// shown in dynamic ad.
        DealName = 3,
        /// Data Type: STRING. Local deal subtitle to be shown in dynamic ad.
        Subtitle = 4,
        /// Data Type: STRING. Description of local deal to be shown in dynamic ad.
        Description = 5,
        /// Data Type: STRING. Price to be shown in the ad. Highly recommended for
        /// dynamic ads. Example: "100.00 USD"
        Price = 6,
        /// Data Type: STRING. Formatted price to be shown in the ad.
        /// Example: "Starting at $100.00 USD", "$80 - $100"
        FormattedPrice = 7,
        /// Data Type: STRING. Sale price to be shown in the ad.
        /// Example: "80.00 USD"
        SalePrice = 8,
        /// Data Type: STRING. Formatted sale price to be shown in the ad.
        /// Example: "On sale for $80.00", "$60 - $80"
        FormattedSalePrice = 9,
        /// Data Type: URL. Image to be displayed in the ad.
        ImageUrl = 10,
        /// Data Type: STRING. Complete property address, including postal code.
        Address = 11,
        /// Data Type: STRING. Category of local deal used to group like items
        /// together for recommendation engine.
        Category = 12,
        /// Data Type: STRING_LIST. Keywords used for product retrieval.
        ContextualKeywords = 13,
        /// Data Type: URL_LIST. Required. Final URLs to be used in ad when using
        /// Upgraded URLs; the more specific the better (e.g. the individual URL of a
        /// specific local deal and its location).
        FinalUrls = 14,
        /// Data Type: URL_LIST. Final mobile URLs for the ad when using Upgraded
        /// URLs.
        FinalMobileUrls = 15,
        /// Data Type: URL. Tracking template for the ad when using Upgraded URLs.
        TrackingUrl = 16,
        /// Data Type: STRING. Android app link. Must be formatted as:
        /// android-app://{package_id}/{scheme}/{host_path}.
        /// The components are defined as follows:
        /// package_id: app ID as specified in Google Play.
        /// scheme: the scheme to pass to the application. Can be HTTP, or a custom
        ///   scheme.
        /// host_path: identifies the specific content within your application.
        AndroidAppLink = 17,
        /// Data Type: STRING_LIST. List of recommended local deal IDs to show
        /// together with this item.
        SimilarDealIds = 18,
        /// Data Type: STRING. iOS app link.
        IosAppLink = 19,
        /// Data Type: INT64. iOS app store ID.
        IosAppStoreId = 20,
    }
}
// Proto file describing Location Extension Targeting criterion fields.

/// Values for Location Extension Targeting criterion fields.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LocationExtensionTargetingCriterionFieldEnum {}
/// Nested message and enum types in `LocationExtensionTargetingCriterionFieldEnum`.
pub mod location_extension_targeting_criterion_field_enum {
    /// Possible values for Location Extension Targeting criterion fields.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum LocationExtensionTargetingCriterionField {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Data Type: STRING. Line 1 of the business address.
        AddressLine1 = 2,
        /// Data Type: STRING. Line 2 of the business address.
        AddressLine2 = 3,
        /// Data Type: STRING. City of the business address.
        City = 4,
        /// Data Type: STRING. Province of the business address.
        Province = 5,
        /// Data Type: STRING. Postal code of the business address.
        PostalCode = 6,
        /// Data Type: STRING. Country code of the business address.
        CountryCode = 7,
    }
}
// Proto file describing Location placeholder fields.

/// Values for Location placeholder fields.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LocationPlaceholderFieldEnum {}
/// Nested message and enum types in `LocationPlaceholderFieldEnum`.
pub mod location_placeholder_field_enum {
    /// Possible values for Location placeholder fields.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum LocationPlaceholderField {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Data Type: STRING. The name of the business.
        BusinessName = 2,
        /// Data Type: STRING. Line 1 of the business address.
        AddressLine1 = 3,
        /// Data Type: STRING. Line 2 of the business address.
        AddressLine2 = 4,
        /// Data Type: STRING. City of the business address.
        City = 5,
        /// Data Type: STRING. Province of the business address.
        Province = 6,
        /// Data Type: STRING. Postal code of the business address.
        PostalCode = 7,
        /// Data Type: STRING. Country code of the business address.
        CountryCode = 8,
        /// Data Type: STRING. Phone number of the business.
        PhoneNumber = 9,
    }
}
// Proto file describing location source types.

/// Used to distinguish the location source type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LocationSourceTypeEnum {}
/// Nested message and enum types in `LocationSourceTypeEnum`.
pub mod location_source_type_enum {
    /// The possible types of a location source.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum LocationSourceType {
        /// No value has been specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Locations associated with the customer's linked Google My Business
        /// account.
        GoogleMyBusiness = 2,
        /// Affiliate (chain) store locations. For example, Best Buy store locations.
        Affiliate = 3,
    }
}
/// Container for enum describing possible status of a manager and client link.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ManagerLinkStatusEnum {}
/// Nested message and enum types in `ManagerLinkStatusEnum`.
pub mod manager_link_status_enum {
    /// Possible statuses of a link.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ManagerLinkStatus {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Indicates current in-effect relationship
        Active = 2,
        /// Indicates terminated relationship
        Inactive = 3,
        /// Indicates relationship has been requested by manager, but the client
        /// hasn't accepted yet.
        Pending = 4,
        /// Relationship was requested by the manager, but the client has refused.
        Refused = 5,
        /// Indicates relationship has been requested by manager, but manager
        /// canceled it.
        Canceled = 6,
    }
}
// Proto file describing media types.

/// Container for enum describing the types of media.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MediaTypeEnum {}
/// Nested message and enum types in `MediaTypeEnum`.
pub mod media_type_enum {
    /// The type of media.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum MediaType {
        /// The media type has not been specified.
        Unspecified = 0,
        /// The received value is not known in this version.
        ///
        /// This is a response-only value.
        Unknown = 1,
        /// Static image, used for image ad.
        Image = 2,
        /// Small image, used for map ad.
        Icon = 3,
        /// ZIP file, used in fields of template ads.
        MediaBundle = 4,
        /// Audio file.
        Audio = 5,
        /// Video file.
        Video = 6,
        /// Animated image, such as animated GIF.
        DynamicImage = 7,
    }
}
// Proto file describing Merchant Center link statuses.

/// Container for enum describing possible statuses of a Google Merchant Center
/// link.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MerchantCenterLinkStatusEnum {}
/// Nested message and enum types in `MerchantCenterLinkStatusEnum`.
pub mod merchant_center_link_status_enum {
    /// Describes the possible statuses for a link between a Google Ads customer
    /// and a Google Merchant Center account.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum MerchantCenterLinkStatus {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// The link is enabled.
        Enabled = 2,
        /// The link has no effect. It was proposed by the Merchant Center Account
        /// owner and hasn't been confirmed by the customer.
        Pending = 3,
    }
}
// Proto file describing Message placeholder fields.

/// Values for Message placeholder fields.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MessagePlaceholderFieldEnum {}
/// Nested message and enum types in `MessagePlaceholderFieldEnum`.
pub mod message_placeholder_field_enum {
    /// Possible values for Message placeholder fields.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum MessagePlaceholderField {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Data Type: STRING. The name of your business.
        BusinessName = 2,
        /// Data Type: STRING. Country code of phone number.
        CountryCode = 3,
        /// Data Type: STRING. A phone number that's capable of sending and receiving
        /// text messages.
        PhoneNumber = 4,
        /// Data Type: STRING. The text that will go in your click-to-message ad.
        MessageExtensionText = 5,
        /// Data Type: STRING. The message text automatically shows in people's
        /// messaging apps when they tap to send you a message.
        MessageText = 6,
    }
}
/// Container for enum describing different types of mobile app vendors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MobileAppVendorEnum {}
/// Nested message and enum types in `MobileAppVendorEnum`.
pub mod mobile_app_vendor_enum {
    /// The type of mobile app vendor
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum MobileAppVendor {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Mobile app vendor for Apple app store.
        AppleAppStore = 2,
        /// Mobile app vendor for Google app store.
        GoogleAppStore = 3,
    }
}
// Proto file describing mobile device types.

/// Container for enum describing the types of mobile device.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MobileDeviceTypeEnum {}
/// Nested message and enum types in `MobileDeviceTypeEnum`.
pub mod mobile_device_type_enum {
    /// The type of mobile device.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum MobileDeviceType {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Mobile phones.
        Mobile = 2,
        /// Tablets.
        Tablet = 3,
    }
}
// Proto file describing negative geo target types.

/// Container for enum describing possible negative geo target types.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NegativeGeoTargetTypeEnum {}
/// Nested message and enum types in `NegativeGeoTargetTypeEnum`.
pub mod negative_geo_target_type_enum {
    /// The possible negative geo target types.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum NegativeGeoTargetType {
        /// Not specified.
        Unspecified = 0,
        /// The value is unknown in this version.
        Unknown = 1,
        /// Specifies that a user is excluded from seeing the ad if they
        /// are in, or show interest in, advertiser's excluded locations.
        PresenceOrInterest = 4,
        /// Specifies that a user is excluded from seeing the ad if they
        /// are in advertiser's excluded locations.
        Presence = 5,
    }
}
// Proto file describing offline user data job failure reasons.

/// Container for enum describing reasons why an offline user data job
/// failed to be processed.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OfflineUserDataJobFailureReasonEnum {}
/// Nested message and enum types in `OfflineUserDataJobFailureReasonEnum`.
pub mod offline_user_data_job_failure_reason_enum {
    /// The failure reason of an offline user data job.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum OfflineUserDataJobFailureReason {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// The matched transactions are insufficient.
        InsufficientMatchedTransactions = 2,
        /// The uploaded transactions are insufficient.
        InsufficientTransactions = 3,
    }
}
// Proto file describing offline user data job status.

/// Container for enum describing status of an offline user data job.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OfflineUserDataJobStatusEnum {}
/// Nested message and enum types in `OfflineUserDataJobStatusEnum`.
pub mod offline_user_data_job_status_enum {
    /// The status of an offline user data job.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum OfflineUserDataJobStatus {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// The job has been successfully created and pending for uploading.
        Pending = 2,
        /// Upload(s) have been accepted and data is being processed.
        Running = 3,
        /// Uploaded data has been successfully processed.
        Success = 4,
        /// Uploaded data has failed to be processed.
        Failed = 5,
    }
}
// Proto file describing offline user data job types.

/// Container for enum describing types of an offline user data job.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OfflineUserDataJobTypeEnum {}
/// Nested message and enum types in `OfflineUserDataJobTypeEnum`.
pub mod offline_user_data_job_type_enum {
    /// The type of an offline user data job.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum OfflineUserDataJobType {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Store Sales Direct data for self service.
        StoreSalesUploadFirstParty = 2,
        /// Store Sales Direct data for third party.
        StoreSalesUploadThirdParty = 3,
        /// Customer Match user list data.
        CustomerMatchUserList = 4,
        /// Customer Match with attribute data.
        CustomerMatchWithAttributes = 5,
    }
}
// Proto file describing operating system version operator types.

/// Container for enum describing the type of OS operators.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperatingSystemVersionOperatorTypeEnum {}
/// Nested message and enum types in `OperatingSystemVersionOperatorTypeEnum`.
pub mod operating_system_version_operator_type_enum {
    /// The type of operating system version.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum OperatingSystemVersionOperatorType {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Equals to the specified version.
        EqualsTo = 2,
        /// Greater than or equals to the specified version.
        GreaterThanEqualsTo = 4,
    }
}
// Proto file describing optimization goal type.

/// Container for enum describing the type of optimization goal.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OptimizationGoalTypeEnum {}
/// Nested message and enum types in `OptimizationGoalTypeEnum`.
pub mod optimization_goal_type_enum {
    /// The type of optimization goal
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum OptimizationGoalType {
        /// Not specified.
        Unspecified = 0,
        /// Used as a return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Optimize for call clicks. Call click conversions are times people
        /// selected 'Call' to contact a store after viewing an ad.
        CallClicks = 2,
        /// Optimize for driving directions. Driving directions conversions are
        /// times people selected 'Get directions' to navigate to a store after
        /// viewing an ad.
        DrivingDirections = 3,
    }
}
// Proto file describing payment modes.

/// Container for enum describing possible payment modes.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PaymentModeEnum {}
/// Nested message and enum types in `PaymentModeEnum`.
pub mod payment_mode_enum {
    /// Enum describing possible payment modes.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum PaymentMode {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Pay per click.
        Clicks = 4,
        /// Pay per conversion value. This mode is only supported by campaigns with
        /// AdvertisingChannelType.HOTEL, BiddingStrategyType.COMMISSION, and
        /// BudgetType.HOTEL_ADS_COMMISSION.
        ConversionValue = 5,
        /// Pay per conversion. This mode is only supported by campaigns with
        /// AdvertisingChannelType.DISPLAY (excluding
        /// AdvertisingChannelSubType.DISPLAY_GMAIL), BiddingStrategyType.TARGET_CPA,
        /// and BudgetType.FIXED_CPA. The customer must also be eligible for this
        /// mode. See Customer.eligibility_failure_reasons for details.
        Conversions = 6,
        /// Pay per guest stay value. This mode is only supported by campaigns with
        /// AdvertisingChannelType.HOTEL, BiddingStrategyType.COMMISSION, and
        /// BudgetType.STANDARD.
        GuestStay = 7,
    }
}
// Proto file describing placement types.

/// Container for enum describing possible placement types.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlacementTypeEnum {}
/// Nested message and enum types in `PlacementTypeEnum`.
pub mod placement_type_enum {
    /// Possible placement types for a feed mapping.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum PlacementType {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Websites(e.g. 'www.flowers4sale.com').
        Website = 2,
        /// Mobile application categories(e.g. 'Games').
        MobileAppCategory = 3,
        /// mobile applications(e.g. 'mobileapp::2-com.whatsthewordanswers').
        MobileApplication = 4,
        /// YouTube videos(e.g. 'youtube.com/video/wtLJPvx7-ys').
        YoutubeVideo = 5,
        /// YouTube channels(e.g. 'youtube.com::L8ZULXASCc1I_oaOT0NaOQ').
        YoutubeChannel = 6,
    }
}
// Proto file describing positive geo target types.

/// Container for enum describing possible positive geo target types.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PositiveGeoTargetTypeEnum {}
/// Nested message and enum types in `PositiveGeoTargetTypeEnum`.
pub mod positive_geo_target_type_enum {
    /// The possible positive geo target types.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum PositiveGeoTargetType {
        /// Not specified.
        Unspecified = 0,
        /// The value is unknown in this version.
        Unknown = 1,
        /// Specifies that an ad is triggered if the user is in,
        /// or shows interest in, advertiser's targeted locations.
        PresenceOrInterest = 5,
        /// Specifies that an ad is triggered if the user
        /// searches for advertiser's targeted locations.
        /// This can only be used with Search and standard
        /// Shopping campaigns.
        SearchInterest = 6,
        /// Specifies that an ad is triggered if the user is in
        /// or regularly in advertiser's targeted locations.
        Presence = 7,
    }
}
// Proto file describing Price placeholder fields.

/// Values for Price placeholder fields.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PricePlaceholderFieldEnum {}
/// Nested message and enum types in `PricePlaceholderFieldEnum`.
pub mod price_placeholder_field_enum {
    /// Possible values for Price placeholder fields.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum PricePlaceholderField {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Data Type: STRING. The type of your price feed. Must match one of the
        /// predefined price feed type exactly.
        Type = 2,
        /// Data Type: STRING. The qualifier of each price. Must match one of the
        /// predefined price qualifiers exactly.
        PriceQualifier = 3,
        /// Data Type: URL. Tracking template for the price feed when using Upgraded
        /// URLs.
        TrackingTemplate = 4,
        /// Data Type: STRING. Language of the price feed. Must match one of the
        /// available available locale codes exactly.
        Language = 5,
        /// Data Type: STRING. Final URL suffix for the price feed when using
        /// parallel tracking.
        FinalUrlSuffix = 6,
        /// Data Type: STRING. The header of item 1 of the table.
        Item1Header = 100,
        /// Data Type: STRING. The description of item 1 of the table.
        Item1Description = 101,
        /// Data Type: MONEY. The price (money with currency) of item 1 of the table,
        /// e.g., 30 USD. The currency must match one of the available currencies.
        Item1Price = 102,
        /// Data Type: STRING. The price unit of item 1 of the table. Must match one
        /// of the predefined price units.
        Item1Unit = 103,
        /// Data Type: URL_LIST. The final URLs of item 1 of the table when using
        /// Upgraded URLs.
        Item1FinalUrls = 104,
        /// Data Type: URL_LIST. The final mobile URLs of item 1 of the table when
        /// using Upgraded URLs.
        Item1FinalMobileUrls = 105,
        /// Data Type: STRING. The header of item 2 of the table.
        Item2Header = 200,
        /// Data Type: STRING. The description of item 2 of the table.
        Item2Description = 201,
        /// Data Type: MONEY. The price (money with currency) of item 2 of the table,
        /// e.g., 30 USD. The currency must match one of the available currencies.
        Item2Price = 202,
        /// Data Type: STRING. The price unit of item 2 of the table. Must match one
        /// of the predefined price units.
        Item2Unit = 203,
        /// Data Type: URL_LIST. The final URLs of item 2 of the table when using
        /// Upgraded URLs.
        Item2FinalUrls = 204,
        /// Data Type: URL_LIST. The final mobile URLs of item 2 of the table when
        /// using Upgraded URLs.
        Item2FinalMobileUrls = 205,
        /// Data Type: STRING. The header of item 3 of the table.
        Item3Header = 300,
        /// Data Type: STRING. The description of item 3 of the table.
        Item3Description = 301,
        /// Data Type: MONEY. The price (money with currency) of item 3 of the table,
        /// e.g., 30 USD. The currency must match one of the available currencies.
        Item3Price = 302,
        /// Data Type: STRING. The price unit of item 3 of the table. Must match one
        /// of the predefined price units.
        Item3Unit = 303,
        /// Data Type: URL_LIST. The final URLs of item 3 of the table when using
        /// Upgraded URLs.
        Item3FinalUrls = 304,
        /// Data Type: URL_LIST. The final mobile URLs of item 3 of the table when
        /// using Upgraded URLs.
        Item3FinalMobileUrls = 305,
        /// Data Type: STRING. The header of item 4 of the table.
        Item4Header = 400,
        /// Data Type: STRING. The description of item 4 of the table.
        Item4Description = 401,
        /// Data Type: MONEY. The price (money with currency) of item 4 of the table,
        /// e.g., 30 USD. The currency must match one of the available currencies.
        Item4Price = 402,
        /// Data Type: STRING. The price unit of item 4 of the table. Must match one
        /// of the predefined price units.
        Item4Unit = 403,
        /// Data Type: URL_LIST. The final URLs of item 4 of the table when using
        /// Upgraded URLs.
        Item4FinalUrls = 404,
        /// Data Type: URL_LIST. The final mobile URLs of item 4 of the table when
        /// using Upgraded URLs.
        Item4FinalMobileUrls = 405,
        /// Data Type: STRING. The header of item 5 of the table.
        Item5Header = 500,
        /// Data Type: STRING. The description of item 5 of the table.
        Item5Description = 501,
        /// Data Type: MONEY. The price (money with currency) of item 5 of the table,
        /// e.g., 30 USD. The currency must match one of the available currencies.
        Item5Price = 502,
        /// Data Type: STRING. The price unit of item 5 of the table. Must match one
        /// of the predefined price units.
        Item5Unit = 503,
        /// Data Type: URL_LIST. The final URLs of item 5 of the table when using
        /// Upgraded URLs.
        Item5FinalUrls = 504,
        /// Data Type: URL_LIST. The final mobile URLs of item 5 of the table when
        /// using Upgraded URLs.
        Item5FinalMobileUrls = 505,
        /// Data Type: STRING. The header of item 6 of the table.
        Item6Header = 600,
        /// Data Type: STRING. The description of item 6 of the table.
        Item6Description = 601,
        /// Data Type: MONEY. The price (money with currency) of item 6 of the table,
        /// e.g., 30 USD. The currency must match one of the available currencies.
        Item6Price = 602,
        /// Data Type: STRING. The price unit of item 6 of the table. Must match one
        /// of the predefined price units.
        Item6Unit = 603,
        /// Data Type: URL_LIST. The final URLs of item 6 of the table when using
        /// Upgraded URLs.
        Item6FinalUrls = 604,
        /// Data Type: URL_LIST. The final mobile URLs of item 6 of the table when
        /// using Upgraded URLs.
        Item6FinalMobileUrls = 605,
        /// Data Type: STRING. The header of item 7 of the table.
        Item7Header = 700,
        /// Data Type: STRING. The description of item 7 of the table.
        Item7Description = 701,
        /// Data Type: MONEY. The price (money with currency) of item 7 of the table,
        /// e.g., 30 USD. The currency must match one of the available currencies.
        Item7Price = 702,
        /// Data Type: STRING. The price unit of item 7 of the table. Must match one
        /// of the predefined price units.
        Item7Unit = 703,
        /// Data Type: URL_LIST. The final URLs of item 7 of the table when using
        /// Upgraded URLs.
        Item7FinalUrls = 704,
        /// Data Type: URL_LIST. The final mobile URLs of item 7 of the table when
        /// using Upgraded URLs.
        Item7FinalMobileUrls = 705,
        /// Data Type: STRING. The header of item 8 of the table.
        Item8Header = 800,
        /// Data Type: STRING. The description of item 8 of the table.
        Item8Description = 801,
        /// Data Type: MONEY. The price (money with currency) of item 8 of the table,
        /// e.g., 30 USD. The currency must match one of the available currencies.
        Item8Price = 802,
        /// Data Type: STRING. The price unit of item 8 of the table. Must match one
        /// of the predefined price units.
        Item8Unit = 803,
        /// Data Type: URL_LIST. The final URLs of item 8 of the table when using
        /// Upgraded URLs.
        Item8FinalUrls = 804,
        /// Data Type: URL_LIST. The final mobile URLs of item 8 of the table when
        /// using Upgraded URLs.
        Item8FinalMobileUrls = 805,
    }
}
// Proto file describing bidding schemes.

/// Status of the product bidding category.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProductBiddingCategoryStatusEnum {}
/// Nested message and enum types in `ProductBiddingCategoryStatusEnum`.
pub mod product_bidding_category_status_enum {
    /// Enum describing the status of the product bidding category.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ProductBiddingCategoryStatus {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// The category is active and can be used for bidding.
        Active = 2,
        /// The category is obsolete. Used only for reporting purposes.
        Obsolete = 3,
    }
}
// Proto file describing Promotion placeholder fields.

/// Values for Promotion placeholder fields.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PromotionPlaceholderFieldEnum {}
/// Nested message and enum types in `PromotionPlaceholderFieldEnum`.
pub mod promotion_placeholder_field_enum {
    /// Possible values for Promotion placeholder fields.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum PromotionPlaceholderField {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Data Type: STRING. The text that appears on the ad when the extension is
        /// shown.
        PromotionTarget = 2,
        /// Data Type: STRING. Allows you to add "up to" phrase to the promotion,
        /// in case you have variable promotion rates.
        DiscountModifier = 3,
        /// Data Type: INT64. Takes a value in micros, where 1 million micros
        /// represents 1%, and is shown as a percentage when rendered.
        PercentOff = 4,
        /// Data Type: MONEY. Requires a currency and an amount of money.
        MoneyAmountOff = 5,
        /// Data Type: STRING. A string that the user enters to get the discount.
        PromotionCode = 6,
        /// Data Type: MONEY. A minimum spend before the user qualifies for the
        /// promotion.
        OrdersOverAmount = 7,
        /// Data Type: DATE. The start date of the promotion.
        PromotionStart = 8,
        /// Data Type: DATE. The end date of the promotion.
        PromotionEnd = 9,
        /// Data Type: STRING. Describes the associated event for the promotion using
        /// one of the PromotionExtensionOccasion enum values, for example NEW_YEARS.
        Occasion = 10,
        /// Data Type: URL_LIST. Final URLs to be used in the ad when using Upgraded
        /// URLs.
        FinalUrls = 11,
        /// Data Type: URL_LIST. Final mobile URLs for the ad when using Upgraded
        /// URLs.
        FinalMobileUrls = 12,
        /// Data Type: URL. Tracking template for the ad when using Upgraded URLs.
        TrackingUrl = 13,
        /// Data Type: STRING. A string represented by a language code for the
        /// promotion.
        Language = 14,
        /// Data Type: STRING. Final URL suffix for the ad when using parallel
        /// tracking.
        FinalUrlSuffix = 15,
    }
}
// Proto file describing ad lengths of a plannable video ad.

/// Message describing length of a plannable video ad.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReachPlanAdLengthEnum {}
/// Nested message and enum types in `ReachPlanAdLengthEnum`.
pub mod reach_plan_ad_length_enum {
    /// Possible ad length values.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ReachPlanAdLength {
        /// Not specified.
        Unspecified = 0,
        /// The value is unknown in this version.
        Unknown = 1,
        /// 6 seconds long ad.
        SixSeconds = 2,
        /// 15 or 20 seconds long ad.
        FifteenOrTwentySeconds = 3,
        /// More than 20 seconds long ad.
        TwentySecondsOrMore = 4,
    }
}
// Proto file describing a plannable age range.

/// Message describing plannable age ranges.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReachPlanAgeRangeEnum {}
/// Nested message and enum types in `ReachPlanAgeRangeEnum`.
pub mod reach_plan_age_range_enum {
    /// Possible plannable age range values.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ReachPlanAgeRange {
        /// Not specified.
        Unspecified = 0,
        /// The value is unknown in this version.
        Unknown = 1,
        /// Between 18 and 24 years old.
        AgeRange1824 = 503001,
        /// Between 18 and 34 years old.
        AgeRange1834 = 2,
        /// Between 18 and 44 years old.
        AgeRange1844 = 3,
        /// Between 18 and 49 years old.
        AgeRange1849 = 4,
        /// Between 18 and 54 years old.
        AgeRange1854 = 5,
        /// Between 18 and 64 years old.
        AgeRange1864 = 6,
        /// Between 18 and 65+ years old.
        AgeRange1865Up = 7,
        /// Between 21 and 34 years old.
        AgeRange2134 = 8,
        /// Between 25 and 34 years old.
        AgeRange2534 = 503002,
        /// Between 25 and 44 years old.
        AgeRange2544 = 9,
        /// Between 25 and 49 years old.
        AgeRange2549 = 10,
        /// Between 25 and 54 years old.
        AgeRange2554 = 11,
        /// Between 25 and 64 years old.
        AgeRange2564 = 12,
        /// Between 25 and 65+ years old.
        AgeRange2565Up = 13,
        /// Between 35 and 44 years old.
        AgeRange3544 = 503003,
        /// Between 35 and 49 years old.
        AgeRange3549 = 14,
        /// Between 35 and 54 years old.
        AgeRange3554 = 15,
        /// Between 35 and 64 years old.
        AgeRange3564 = 16,
        /// Between 35 and 65+ years old.
        AgeRange3565Up = 17,
        /// Between 45 and 54 years old.
        AgeRange4554 = 503004,
        /// Between 45 and 64 years old.
        AgeRange4564 = 18,
        /// Between 45 and 65+ years old.
        AgeRange4565Up = 19,
        /// Between 50 and 65+ years old.
        AgeRange5065Up = 20,
        /// Between 55 and 64 years old.
        AgeRange5564 = 503005,
        /// Between 55 and 65+ years old.
        AgeRange5565Up = 21,
        /// 65 years old and beyond.
        AgeRange65Up = 503006,
    }
}
// Proto file describing a plannable network.

/// Container for enum describing plannable networks.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReachPlanNetworkEnum {}
/// Nested message and enum types in `ReachPlanNetworkEnum`.
pub mod reach_plan_network_enum {
    /// Possible plannable network values.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ReachPlanNetwork {
        /// Not specified.
        Unspecified = 0,
        /// Used as a return value only. Represents value unknown in this version.
        Unknown = 1,
        /// YouTube network.
        Youtube = 2,
        /// Google Video Partners (GVP) network.
        GoogleVideoPartners = 3,
        /// A combination of the YouTube network and the Google Video Partners
        /// network.
        YoutubeAndGoogleVideoPartners = 4,
    }
}
// Proto file describing Real Estate placeholder fields.

/// Values for Real Estate placeholder fields.
/// For more information about dynamic remarketing feeds, see
/// <https://support.google.com/google-ads/answer/6053288.>
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RealEstatePlaceholderFieldEnum {}
/// Nested message and enum types in `RealEstatePlaceholderFieldEnum`.
pub mod real_estate_placeholder_field_enum {
    /// Possible values for Real Estate placeholder fields.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum RealEstatePlaceholderField {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Data Type: STRING. Unique ID.
        ListingId = 2,
        /// Data Type: STRING. Main headline with listing name to be shown in dynamic
        /// ad.
        ListingName = 3,
        /// Data Type: STRING. City name to be shown in dynamic ad.
        CityName = 4,
        /// Data Type: STRING. Description of listing to be shown in dynamic ad.
        Description = 5,
        /// Data Type: STRING. Complete listing address, including postal code.
        Address = 6,
        /// Data Type: STRING. Price to be shown in the ad.
        /// Example: "100.00 USD"
        Price = 7,
        /// Data Type: STRING. Formatted price to be shown in the ad.
        /// Example: "Starting at $100.00 USD", "$80 - $100"
        FormattedPrice = 8,
        /// Data Type: URL. Image to be displayed in the ad.
        ImageUrl = 9,
        /// Data Type: STRING. Type of property (house, condo, apartment, etc.) used
        /// to group like items together for recommendation engine.
        PropertyType = 10,
        /// Data Type: STRING. Type of listing (resale, rental, foreclosure, etc.)
        /// used to group like items together for recommendation engine.
        ListingType = 11,
        /// Data Type: STRING_LIST. Keywords used for product retrieval.
        ContextualKeywords = 12,
        /// Data Type: URL_LIST. Final URLs to be used in ad when using Upgraded
        /// URLs; the more specific the better (e.g. the individual URL of a specific
        /// listing and its location).
        FinalUrls = 13,
        /// Data Type: URL_LIST. Final mobile URLs for the ad when using Upgraded
        /// URLs.
        FinalMobileUrls = 14,
        /// Data Type: URL. Tracking template for the ad when using Upgraded URLs.
        TrackingUrl = 15,
        /// Data Type: STRING. Android app link. Must be formatted as:
        /// android-app://{package_id}/{scheme}/{host_path}.
        /// The components are defined as follows:
        /// package_id: app ID as specified in Google Play.
        /// scheme: the scheme to pass to the application. Can be HTTP, or a custom
        ///   scheme.
        /// host_path: identifies the specific content within your application.
        AndroidAppLink = 16,
        /// Data Type: STRING_LIST. List of recommended listing IDs to show together
        /// with this item.
        SimilarListingIds = 17,
        /// Data Type: STRING. iOS app link.
        IosAppLink = 18,
        /// Data Type: INT64. iOS app store ID.
        IosAppStoreId = 19,
    }
}
// Proto file describing Recommendation types.

/// Container for enum describing types of recommendations.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RecommendationTypeEnum {}
/// Nested message and enum types in `RecommendationTypeEnum`.
pub mod recommendation_type_enum {
    /// Types of recommendations.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum RecommendationType {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Budget recommendation for campaigns that are currently budget-constrained
        /// (as opposed to the FORECASTING_CAMPAIGN_BUDGET recommendation, which
        /// applies to campaigns that are expected to become budget-constrained in
        /// the future).
        CampaignBudget = 2,
        /// Keyword recommendation.
        Keyword = 3,
        /// Recommendation to add a new text ad.
        TextAd = 4,
        /// Recommendation to update a campaign to use a Target CPA bidding strategy.
        TargetCpaOptIn = 5,
        /// Recommendation to update a campaign to use the Maximize Conversions
        /// bidding strategy.
        MaximizeConversionsOptIn = 6,
        /// Recommendation to enable Enhanced Cost Per Click for a campaign.
        EnhancedCpcOptIn = 7,
        /// Recommendation to start showing your campaign's ads on Google Search
        /// Partners Websites.
        SearchPartnersOptIn = 8,
        /// Recommendation to update a campaign to use a Maximize Clicks bidding
        /// strategy.
        MaximizeClicksOptIn = 9,
        /// Recommendation to start using the "Optimize" ad rotation setting for the
        /// given ad group.
        OptimizeAdRotation = 10,
        /// Recommendation to add callout extensions to a campaign.
        CalloutExtension = 11,
        /// Recommendation to add sitelink extensions to a campaign.
        SitelinkExtension = 12,
        /// Recommendation to add call extensions to a campaign.
        CallExtension = 13,
        /// Recommendation to change an existing keyword from one match type to a
        /// broader match type.
        KeywordMatchType = 14,
        /// Recommendation to move unused budget from one budget to a constrained
        /// budget.
        MoveUnusedBudget = 15,
        /// Budget recommendation for campaigns that are expected to become
        /// budget-constrained in the future (as opposed to the CAMPAIGN_BUDGET
        /// recommendation, which applies to campaigns that are currently
        /// budget-constrained).
        ForecastingCampaignBudget = 16,
        /// Recommendation to update a campaign to use a Target ROAS bidding
        /// strategy.
        TargetRoasOptIn = 17,
        /// Recommendation to add a new responsive search ad.
        ResponsiveSearchAd = 18,
        /// Budget recommendation for campaigns whose ROI is predicted to increase
        /// with a budget adjustment.
        MarginalRoiCampaignBudget = 19,
    }
}
// Proto file describing the resource change operations in change event.

/// Container for enum describing resource change operations
/// in the ChangeEvent resource.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResourceChangeOperationEnum {}
/// Nested message and enum types in `ResourceChangeOperationEnum`.
pub mod resource_change_operation_enum {
    /// The operation on the changed resource in change_event resource.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ResourceChangeOperation {
        /// No value has been specified.
        Unspecified = 0,
        /// Used for return value only. Represents an unclassified operation unknown
        /// in this version.
        Unknown = 1,
        /// The resource was created.
        Create = 2,
        /// The resource was modified.
        Update = 3,
        /// The resource was removed.
        Remove = 4,
    }
}
/// Container for enum describing possible resource limit types.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResourceLimitTypeEnum {}
/// Nested message and enum types in `ResourceLimitTypeEnum`.
pub mod resource_limit_type_enum {
    /// Resource limit type.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ResourceLimitType {
        /// No value has been specified.
        Unspecified = 0,
        /// Used for return value only. Represents an unclassified operation unknown
        /// in this version.
        Unknown = 1,
        /// Number of ENABLED and PAUSED campaigns per customer.
        CampaignsPerCustomer = 2,
        /// Number of ENABLED and PAUSED base campaigns per customer.
        BaseCampaignsPerCustomer = 3,
        /// Number of ENABLED and PAUSED experiment campaigns per customer.
        ExperimentCampaignsPerCustomer = 105,
        /// Number of ENABLED and PAUSED Hotel campaigns per customer.
        HotelCampaignsPerCustomer = 4,
        /// Number of ENABLED and PAUSED Smart Shopping campaigns per customer.
        SmartShoppingCampaignsPerCustomer = 5,
        /// Number of ENABLED ad groups per campaign.
        AdGroupsPerCampaign = 6,
        /// Number of ENABLED ad groups per Shopping campaign.
        AdGroupsPerShoppingCampaign = 8,
        /// Number of ENABLED ad groups per Hotel campaign.
        AdGroupsPerHotelCampaign = 9,
        /// Number of ENABLED reporting ad groups per local campaign.
        ReportingAdGroupsPerLocalCampaign = 10,
        /// Number of ENABLED reporting ad groups per App campaign. It includes app
        /// campaign and app campaign for engagement.
        ReportingAdGroupsPerAppCampaign = 11,
        /// Number of ENABLED managed ad groups per smart campaign.
        ManagedAdGroupsPerSmartCampaign = 52,
        /// Number of ENABLED ad group criteria per customer.
        /// An ad group criterion is considered as ENABLED if:
        /// 1. it's not REMOVED
        /// 2. its ad group is not REMOVED
        /// 3. its campaign is not REMOVED.
        AdGroupCriteriaPerCustomer = 12,
        /// Number of ad group criteria across all base campaigns for a customer.
        BaseAdGroupCriteriaPerCustomer = 13,
        /// Number of ad group criteria across all experiment campaigns for a
        /// customer.
        ExperimentAdGroupCriteriaPerCustomer = 107,
        /// Number of ENABLED ad group criteria per campaign.
        /// An ad group criterion is considered as ENABLED if:
        /// 1. it's not REMOVED
        /// 2. its ad group is not REMOVED.
        AdGroupCriteriaPerCampaign = 14,
        /// Number of ENABLED campaign criteria per customer.
        CampaignCriteriaPerCustomer = 15,
        /// Number of ENABLED campaign criteria across all base campaigns for a
        /// customer.
        BaseCampaignCriteriaPerCustomer = 16,
        /// Number of ENABLED campaign criteria across all experiment campaigns for a
        /// customer.
        ExperimentCampaignCriteriaPerCustomer = 108,
        /// Number of ENABLED webpage criteria per customer, including
        /// campaign level and ad group level.
        WebpageCriteriaPerCustomer = 17,
        /// Number of ENABLED webpage criteria across all base campaigns for
        /// a customer.
        BaseWebpageCriteriaPerCustomer = 18,
        /// Meximum number of ENABLED webpage criteria across all experiment
        /// campaigns for a customer.
        ExperimentWebpageCriteriaPerCustomer = 19,
        /// Number of combined audience criteria per ad group.
        CombinedAudienceCriteriaPerAdGroup = 20,
        /// Limit for placement criterion type group in customer negative criterion.
        CustomerNegativePlacementCriteriaPerCustomer = 21,
        /// Limit for YouTube TV channels in customer negative criterion.
        CustomerNegativeYoutubeChannelCriteriaPerCustomer = 22,
        /// Number of ENABLED criteria per ad group.
        CriteriaPerAdGroup = 23,
        /// Number of listing group criteria per ad group.
        ListingGroupsPerAdGroup = 24,
        /// Number of ENABLED explicitly shared budgets per customer.
        ExplicitlySharedBudgetsPerCustomer = 25,
        /// Number of ENABLED implicitly shared budgets per customer.
        ImplicitlySharedBudgetsPerCustomer = 26,
        /// Number of combined audience criteria per campaign.
        CombinedAudienceCriteriaPerCampaign = 27,
        /// Number of negative keywords per campaign.
        NegativeKeywordsPerCampaign = 28,
        /// Number of excluded campaign criteria in placement dimension, e.g.
        /// placement, mobile application, YouTube channel, etc. The API criterion
        /// type is NOT limited to placement only, and this does not include
        /// exclusions at the ad group or other levels.
        NegativePlacementsPerCampaign = 29,
        /// Number of geo targets per campaign.
        GeoTargetsPerCampaign = 30,
        /// Number of negative IP blocks per campaign.
        NegativeIpBlocksPerCampaign = 32,
        /// Number of proximity targets per campaign.
        ProximitiesPerCampaign = 33,
        /// Number of listing scopes per Shopping campaign.
        ListingScopesPerShoppingCampaign = 34,
        /// Number of listing scopes per non-Shopping campaign.
        ListingScopesPerNonShoppingCampaign = 35,
        /// Number of criteria per negative keyword shared set.
        NegativeKeywordsPerSharedSet = 36,
        /// Number of criteria per negative placement shared set.
        NegativePlacementsPerSharedSet = 37,
        /// Default number of shared sets allowed per type per customer.
        SharedSetsPerCustomerForTypeDefault = 40,
        /// Number of shared sets of negative placement list type for a
        /// manager customer.
        SharedSetsPerCustomerForNegativePlacementListLower = 41,
        /// Number of hotel_advance_booking_window bid modifiers per ad group.
        HotelAdvanceBookingWindowBidModifiersPerAdGroup = 44,
        /// Number of ENABLED shared bidding strategies per customer.
        BiddingStrategiesPerCustomer = 45,
        /// Number of open basic user lists per customer.
        BasicUserListsPerCustomer = 47,
        /// Number of open logical user lists per customer.
        LogicalUserListsPerCustomer = 48,
        /// Number of ENABLED and PAUSED ad group ads across all base campaigns for a
        /// customer.
        BaseAdGroupAdsPerCustomer = 53,
        /// Number of ENABLED and PAUSED ad group ads across all experiment campaigns
        /// for a customer.
        ExperimentAdGroupAdsPerCustomer = 54,
        /// Number of ENABLED and PAUSED ad group ads per campaign.
        AdGroupAdsPerCampaign = 55,
        /// Number of ENABLED ads per ad group that do not fall in to other buckets.
        /// Includes text and many other types.
        TextAndOtherAdsPerAdGroup = 56,
        /// Number of ENABLED image ads per ad group.
        ImageAdsPerAdGroup = 57,
        /// Number of ENABLED shopping smart ads per ad group.
        ShoppingSmartAdsPerAdGroup = 58,
        /// Number of ENABLED responsive search ads per ad group.
        ResponsiveSearchAdsPerAdGroup = 59,
        /// Number of ENABLED app ads per ad group.
        AppAdsPerAdGroup = 60,
        /// Number of ENABLED app engagement ads per ad group.
        AppEngagementAdsPerAdGroup = 61,
        /// Number of ENABLED local ads per ad group.
        LocalAdsPerAdGroup = 62,
        /// Number of ENABLED video ads per ad group.
        VideoAdsPerAdGroup = 63,
        /// Number of ENABLED lead form asset links per campaign.
        LeadFormAssetLinksPerCampaign = 68,
        /// Number of versions per ad.
        VersionsPerAd = 82,
        /// Number of ENABLED user feeds per customer.
        UserFeedsPerCustomer = 90,
        /// Number of ENABLED system feeds per customer.
        SystemFeedsPerCustomer = 91,
        /// Number of feed attributes per feed.
        FeedAttributesPerFeed = 92,
        /// Number of ENABLED feed items per customer.
        FeedItemsPerCustomer = 94,
        /// Number of ENABLED campaign feeds per customer.
        CampaignFeedsPerCustomer = 95,
        /// Number of ENABLED campaign feeds across all base campaigns for a
        /// customer.
        BaseCampaignFeedsPerCustomer = 96,
        /// Number of ENABLED campaign feeds across all experiment campaigns for a
        /// customer.
        ExperimentCampaignFeedsPerCustomer = 109,
        /// Number of ENABLED ad group feeds per customer.
        AdGroupFeedsPerCustomer = 97,
        /// Number of ENABLED ad group feeds across all base campaigns for a
        /// customer.
        BaseAdGroupFeedsPerCustomer = 98,
        /// Number of ENABLED ad group feeds across all experiment campaigns for a
        /// customer.
        ExperimentAdGroupFeedsPerCustomer = 110,
        /// Number of ENABLED ad group feeds per campaign.
        AdGroupFeedsPerCampaign = 99,
        /// Number of ENABLED feed items per customer.
        FeedItemSetsPerCustomer = 100,
        /// Number of feed items per feed item set.
        FeedItemsPerFeedItemSet = 101,
        /// Number of ENABLED campaign experiments per customer.
        CampaignExperimentsPerCustomer = 112,
        /// Number of video experiment arms per experiment.
        ExperimentArmsPerVideoExperiment = 113,
        /// Number of owned labels per customer.
        OwnedLabelsPerCustomer = 115,
        /// Number of applied labels per campaign.
        LabelsPerCampaign = 117,
        /// Number of applied labels per ad group.
        LabelsPerAdGroup = 118,
        /// Number of applied labels per ad group ad.
        LabelsPerAdGroupAd = 119,
        /// Number of applied labels per ad group criterion.
        LabelsPerAdGroupCriterion = 120,
        /// Number of customers with a single label applied.
        TargetCustomersPerLabel = 121,
        /// Number of ENABLED keyword plans per user per customer.
        /// The limit is applied per <user, customer> pair because by default a plan
        /// is private to a user of a customer. Each user of a customer has his or
        /// her own independent limit.
        KeywordPlansPerUserPerCustomer = 122,
        /// Number of keyword plan ad group keywords per keyword plan.
        KeywordPlanAdGroupKeywordsPerKeywordPlan = 123,
        /// Number of keyword plan ad groups per keyword plan.
        KeywordPlanAdGroupsPerKeywordPlan = 124,
        /// Number of keyword plan negative keywords (both campaign and ad group) per
        /// keyword plan.
        KeywordPlanNegativeKeywordsPerKeywordPlan = 125,
        /// Number of keyword plan campaigns per keyword plan.
        KeywordPlanCampaignsPerKeywordPlan = 126,
        /// Number of ENABLED conversion actions per customer.
        ConversionActionsPerCustomer = 128,
        /// Number of operations in a single batch job.
        BatchJobOperationsPerJob = 130,
        /// Number of PENDING or ENABLED batch jobs per customer.
        BatchJobsPerCustomer = 131,
        /// Number of hotel check-in date range bid modifiers per ad agroup.
        HotelCheckInDateRangeBidModifiersPerAdGroup = 132,
    }
}
// Proto file describing the response content types used in mutate operations.

/// Container for possible response content types.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseContentTypeEnum {}
/// Nested message and enum types in `ResponseContentTypeEnum`.
pub mod response_content_type_enum {
    /// Possible response content types.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ResponseContentType {
        /// Not specified. Will return the resource name only in the response.
        Unspecified = 0,
        /// The mutate response will be the resource name.
        ResourceNameOnly = 1,
        /// The mutate response will be the resource name and the resource with
        /// all mutable fields.
        MutableResource = 2,
    }
}
// Proto file describing search term targeting statuses.

/// Container for enum indicating whether a search term is one of your targeted
/// or excluded keywords.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchTermTargetingStatusEnum {}
/// Nested message and enum types in `SearchTermTargetingStatusEnum`.
pub mod search_term_targeting_status_enum {
    /// Indicates whether the search term is one of your targeted or excluded
    /// keywords.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum SearchTermTargetingStatus {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Search term is added to targeted keywords.
        Added = 2,
        /// Search term matches a negative keyword.
        Excluded = 3,
        /// Search term has been both added and excluded.
        AddedExcluded = 4,
        /// Search term is neither targeted nor excluded.
        None = 5,
    }
}
// Proto file describing shared set statuses.

/// Container for enum describing types of shared set statuses.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SharedSetStatusEnum {}
/// Nested message and enum types in `SharedSetStatusEnum`.
pub mod shared_set_status_enum {
    /// Enum listing the possible shared set statuses.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum SharedSetStatus {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// The shared set is enabled.
        Enabled = 2,
        /// The shared set is removed and can no longer be used.
        Removed = 3,
    }
}
// Proto file describing shared set types.

/// Container for enum describing types of shared sets.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SharedSetTypeEnum {}
/// Nested message and enum types in `SharedSetTypeEnum`.
pub mod shared_set_type_enum {
    /// Enum listing the possible shared set types.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum SharedSetType {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// A set of keywords that can be excluded from targeting.
        NegativeKeywords = 2,
        /// A set of placements that can be excluded from targeting.
        NegativePlacements = 3,
    }
}
// Proto file describing simulation modification methods.

/// Container for enum describing the method by which a simulation modifies
/// a field.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SimulationModificationMethodEnum {}
/// Nested message and enum types in `SimulationModificationMethodEnum`.
pub mod simulation_modification_method_enum {
    /// Enum describing the method by which a simulation modifies a field.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum SimulationModificationMethod {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// The values in a simulation were applied to all children of a given
        /// resource uniformly. Overrides on child resources were not respected.
        Uniform = 2,
        /// The values in a simulation were applied to the given resource.
        /// Overrides on child resources were respected, and traffic estimates
        /// do not include these resources.
        Default = 3,
        /// The values in a simulation were all scaled by the same factor.
        /// For example, in a simulated TargetCpa campaign, the campaign target and
        /// all ad group targets were scaled by a factor of X.
        Scaling = 4,
    }
}
// Proto file describing simulation types.

/// Container for enum describing the field a simulation modifies.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SimulationTypeEnum {}
/// Nested message and enum types in `SimulationTypeEnum`.
pub mod simulation_type_enum {
    /// Enum describing the field a simulation modifies.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum SimulationType {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// The simulation is for a CPC bid.
        CpcBid = 2,
        /// The simulation is for a CPV bid.
        CpvBid = 3,
        /// The simulation is for a CPA target.
        TargetCpa = 4,
        /// The simulation is for a bid modifier.
        BidModifier = 5,
        /// The simulation is for a ROAS target.
        TargetRoas = 6,
        /// The simulation is for a percent CPC bid.
        PercentCpcBid = 7,
        /// The simulation is for an impression share target.
        TargetImpressionShare = 8,
        /// The simulation is for a budget.
        Budget = 9,
    }
}
// Proto file describing Sitelink placeholder fields.

/// Values for Sitelink placeholder fields.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SitelinkPlaceholderFieldEnum {}
/// Nested message and enum types in `SitelinkPlaceholderFieldEnum`.
pub mod sitelink_placeholder_field_enum {
    /// Possible values for Sitelink placeholder fields.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum SitelinkPlaceholderField {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Data Type: STRING. The link text for your sitelink.
        Text = 2,
        /// Data Type: STRING. First line of the sitelink description.
        Line1 = 3,
        /// Data Type: STRING. Second line of the sitelink description.
        Line2 = 4,
        /// Data Type: URL_LIST. Final URLs for the sitelink when using Upgraded
        /// URLs.
        FinalUrls = 5,
        /// Data Type: URL_LIST. Final Mobile URLs for the sitelink when using
        /// Upgraded URLs.
        FinalMobileUrls = 6,
        /// Data Type: URL. Tracking template for the sitelink when using Upgraded
        /// URLs.
        TrackingUrl = 7,
        /// Data Type: STRING. Final URL suffix for sitelink when using parallel
        /// tracking.
        FinalUrlSuffix = 8,
    }
}
// Proto file describing SpendingLimit types.

/// Message describing spending limit types.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpendingLimitTypeEnum {}
/// Nested message and enum types in `SpendingLimitTypeEnum`.
pub mod spending_limit_type_enum {
    /// The possible spending limit types used by certain resources as an
    /// alternative to absolute money values in micros.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum SpendingLimitType {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Infinite, indicates unlimited spending power.
        Infinite = 2,
    }
}
// Proto file describing Structured Snippet placeholder fields.

/// Values for Structured Snippet placeholder fields.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StructuredSnippetPlaceholderFieldEnum {}
/// Nested message and enum types in `StructuredSnippetPlaceholderFieldEnum`.
pub mod structured_snippet_placeholder_field_enum {
    /// Possible values for Structured Snippet placeholder fields.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum StructuredSnippetPlaceholderField {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Data Type: STRING. The category of snippet of your products/services.
        /// Must match exactly one of the predefined structured snippets headers.
        /// For a list, visit
        /// <https://developers.google.com/adwords/api/docs/appendix/structured-snippet-headers>
        Header = 2,
        /// Data Type: STRING_LIST. Text values that describe your products/services.
        /// All text must be family safe. Special or non-ASCII characters are not
        /// permitted. A snippet can be at most 25 characters.
        Snippets = 3,
    }
}
// Proto file describing summary row setting.

/// Indicates summary row setting in request parameter.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SummaryRowSettingEnum {}
/// Nested message and enum types in `SummaryRowSettingEnum`.
pub mod summary_row_setting_enum {
    /// Enum describing return summary row settings.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum SummaryRowSetting {
        /// Not specified.
        Unspecified = 0,
        /// Represent unknown values of return summary row.
        Unknown = 1,
        /// Do not return summary row.
        NoSummaryRow = 2,
        /// Return summary row along with results. The summary row will be returned
        /// in the last batch alone (last batch will contain no results).
        SummaryRowWithResults = 3,
        /// Return summary row only and return no results.
        SummaryRowOnly = 4,
    }
}
// Proto file describing system managed entity sources.

/// Container for enum describing possible system managed entity sources.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SystemManagedResourceSourceEnum {}
/// Nested message and enum types in `SystemManagedResourceSourceEnum`.
pub mod system_managed_resource_source_enum {
    /// Enum listing the possible system managed entity sources.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum SystemManagedResourceSource {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Generated ad variations experiment ad.
        AdVariations = 2,
    }
}
// Proto file describing TargetCpaOptIn recommendation goals.

/// Container for enum describing goals for TargetCpaOptIn recommendation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TargetCpaOptInRecommendationGoalEnum {}
/// Nested message and enum types in `TargetCpaOptInRecommendationGoalEnum`.
pub mod target_cpa_opt_in_recommendation_goal_enum {
    /// Goal of TargetCpaOptIn recommendation.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum TargetCpaOptInRecommendationGoal {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Recommendation to set Target CPA to maintain the same cost.
        SameCost = 2,
        /// Recommendation to set Target CPA to maintain the same conversions.
        SameConversions = 3,
        /// Recommendation to set Target CPA to maintain the same CPA.
        SameCpa = 4,
        /// Recommendation to set Target CPA to a value that is as close as possible
        /// to, yet lower than, the actual CPA (computed for past 28 days).
        ClosestCpa = 5,
    }
}
// Proto file describing TimeType types.

/// Message describing time types.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TimeTypeEnum {}
/// Nested message and enum types in `TimeTypeEnum`.
pub mod time_type_enum {
    /// The possible time types used by certain resources as an alternative to
    /// absolute timestamps.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum TimeType {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// As soon as possible.
        Now = 2,
        /// An infinite point in the future.
        Forever = 3,
    }
}
// Proto file describing Travel placeholder fields.

/// Values for Travel placeholder fields.
/// For more information about dynamic remarketing feeds, see
/// <https://support.google.com/google-ads/answer/6053288.>
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TravelPlaceholderFieldEnum {}
/// Nested message and enum types in `TravelPlaceholderFieldEnum`.
pub mod travel_placeholder_field_enum {
    /// Possible values for Travel placeholder fields.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum TravelPlaceholderField {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Data Type: STRING. Required. Destination id. Example: PAR, LON.
        /// For feed items that only have destination id, destination id must be a
        /// unique key. For feed items that have both destination id and origin id,
        /// then the combination must be a unique key.
        DestinationId = 2,
        /// Data Type: STRING. Origin id. Example: PAR, LON.
        /// Combination of DESTINATION_ID and ORIGIN_ID must be
        /// unique per offer.
        OriginId = 3,
        /// Data Type: STRING. Required. Main headline with name to be shown in
        /// dynamic ad.
        Title = 4,
        /// Data Type: STRING. The destination name. Shorter names are recommended.
        DestinationName = 5,
        /// Data Type: STRING. Origin name. Shorter names are recommended.
        OriginName = 6,
        /// Data Type: STRING. Price to be shown in the ad. Highly recommended for
        /// dynamic ads.
        /// Example: "100.00 USD"
        Price = 7,
        /// Data Type: STRING. Formatted price to be shown in the ad.
        /// Example: "Starting at $100.00 USD", "$80 - $100"
        FormattedPrice = 8,
        /// Data Type: STRING. Sale price to be shown in the ad.
        /// Example: "80.00 USD"
        SalePrice = 9,
        /// Data Type: STRING. Formatted sale price to be shown in the ad.
        /// Example: "On sale for $80.00", "$60 - $80"
        FormattedSalePrice = 10,
        /// Data Type: URL. Image to be displayed in the ad.
        ImageUrl = 11,
        /// Data Type: STRING. Category of travel offer used to group like items
        /// together for recommendation engine.
        Category = 12,
        /// Data Type: STRING_LIST. Keywords used for product retrieval.
        ContextualKeywords = 13,
        /// Data Type: STRING. Address of travel offer, including postal code.
        DestinationAddress = 14,
        /// Data Type: URL_LIST. Required. Final URLs to be used in ad, when using
        /// Upgraded URLs; the more specific the better (e.g. the individual URL of a
        /// specific travel offer and its location).
        FinalUrl = 15,
        /// Data Type: URL_LIST. Final mobile URLs for the ad when using Upgraded
        /// URLs.
        FinalMobileUrls = 16,
        /// Data Type: URL. Tracking template for the ad when using Upgraded URLs.
        TrackingUrl = 17,
        /// Data Type: STRING. Android app link. Must be formatted as:
        /// android-app://{package_id}/{scheme}/{host_path}.
        /// The components are defined as follows:
        /// package_id: app ID as specified in Google Play.
        /// scheme: the scheme to pass to the application. Can be HTTP, or a custom
        ///   scheme.
        /// host_path: identifies the specific content within your application.
        AndroidAppLink = 18,
        /// Data Type: STRING_LIST. List of recommended destination IDs to show
        /// together with this item.
        SimilarDestinationIds = 19,
        /// Data Type: STRING. iOS app link.
        IosAppLink = 20,
        /// Data Type: INT64. iOS app store ID.
        IosAppStoreId = 21,
    }
}
// Proto file describing the UserInterest taxonomy type

/// Message describing a UserInterestTaxonomyType.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserInterestTaxonomyTypeEnum {}
/// Nested message and enum types in `UserInterestTaxonomyTypeEnum`.
pub mod user_interest_taxonomy_type_enum {
    /// Enum containing the possible UserInterestTaxonomyTypes.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum UserInterestTaxonomyType {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// The affinity for this user interest.
        Affinity = 2,
        /// The market for this user interest.
        InMarket = 3,
        /// Users known to have installed applications in the specified categories.
        MobileAppInstallUser = 4,
        /// The geographical location of the interest-based vertical.
        VerticalGeo = 5,
        /// User interest criteria for new smart phone users.
        NewSmartPhoneUser = 6,
    }
}
// Proto file describing user list access status.

/// Indicates if this client still has access to the list.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserListAccessStatusEnum {}
/// Nested message and enum types in `UserListAccessStatusEnum`.
pub mod user_list_access_status_enum {
    /// Enum containing possible user list access statuses.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum UserListAccessStatus {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// The access is enabled.
        Enabled = 2,
        /// The access is disabled.
        Disabled = 3,
    }
}
// Proto file describing user list closing reason.

/// Indicates the reason why the userlist was closed.
/// This enum is only used when a list is auto-closed by the system.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserListClosingReasonEnum {}
/// Nested message and enum types in `UserListClosingReasonEnum`.
pub mod user_list_closing_reason_enum {
    /// Enum describing possible user list closing reasons.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum UserListClosingReason {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// The userlist was closed because of not being used for over one year.
        Unused = 2,
    }
}
// Proto file describing user list membership status.

/// Membership status of this user list. Indicates whether a user list is open
/// or active. Only open user lists can accumulate more users and can be used for
/// targeting.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserListMembershipStatusEnum {}
/// Nested message and enum types in `UserListMembershipStatusEnum`.
pub mod user_list_membership_status_enum {
    /// Enum containing possible user list membership statuses.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum UserListMembershipStatus {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Open status - List is accruing members and can be targeted to.
        Open = 2,
        /// Closed status - No new members being added. Cannot be used for targeting.
        Closed = 3,
    }
}
// Proto file describing user list size range.

/// Size range in terms of number of users of a UserList.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserListSizeRangeEnum {}
/// Nested message and enum types in `UserListSizeRangeEnum`.
pub mod user_list_size_range_enum {
    /// Enum containing possible user list size ranges.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum UserListSizeRange {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// User list has less than 500 users.
        LessThanFiveHundred = 2,
        /// User list has number of users in range of 500 to 1000.
        LessThanOneThousand = 3,
        /// User list has number of users in range of 1000 to 10000.
        OneThousandToTenThousand = 4,
        /// User list has number of users in range of 10000 to 50000.
        TenThousandToFiftyThousand = 5,
        /// User list has number of users in range of 50000 to 100000.
        FiftyThousandToOneHundredThousand = 6,
        /// User list has number of users in range of 100000 to 300000.
        OneHundredThousandToThreeHundredThousand = 7,
        /// User list has number of users in range of 300000 to 500000.
        ThreeHundredThousandToFiveHundredThousand = 8,
        /// User list has number of users in range of 500000 to 1 million.
        FiveHundredThousandToOneMillion = 9,
        /// User list has number of users in range of 1 to 2 millions.
        OneMillionToTwoMillion = 10,
        /// User list has number of users in range of 2 to 3 millions.
        TwoMillionToThreeMillion = 11,
        /// User list has number of users in range of 3 to 5 millions.
        ThreeMillionToFiveMillion = 12,
        /// User list has number of users in range of 5 to 10 millions.
        FiveMillionToTenMillion = 13,
        /// User list has number of users in range of 10 to 20 millions.
        TenMillionToTwentyMillion = 14,
        /// User list has number of users in range of 20 to 30 millions.
        TwentyMillionToThirtyMillion = 15,
        /// User list has number of users in range of 30 to 50 millions.
        ThirtyMillionToFiftyMillion = 16,
        /// User list has over 50 million users.
        OverFiftyMillion = 17,
    }
}
// Proto file describing user list type.

/// The user list types.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserListTypeEnum {}
/// Nested message and enum types in `UserListTypeEnum`.
pub mod user_list_type_enum {
    /// Enum containing possible user list types.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum UserListType {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// UserList represented as a collection of conversion types.
        Remarketing = 2,
        /// UserList represented as a combination of other user lists/interests.
        Logical = 3,
        /// UserList created in the Google Ad Manager platform.
        ExternalRemarketing = 4,
        /// UserList associated with a rule.
        RuleBased = 5,
        /// UserList with users similar to users of another UserList.
        Similar = 6,
        /// UserList of first-party CRM data provided by advertiser in the form of
        /// emails or other formats.
        CrmBased = 7,
    }
}
// Proto file describing vanity pharma display url modes.

/// The display mode for vanity pharma URLs.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VanityPharmaDisplayUrlModeEnum {}
/// Nested message and enum types in `VanityPharmaDisplayUrlModeEnum`.
pub mod vanity_pharma_display_url_mode_enum {
    /// Enum describing possible display modes for vanity pharma URLs.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum VanityPharmaDisplayUrlMode {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Replace vanity pharma URL with manufacturer website url.
        ManufacturerWebsiteUrl = 2,
        /// Replace vanity pharma URL with description of the website.
        WebsiteDescription = 3,
    }
}
// Proto file describing vanity pharma texts.

/// The text that will be displayed in display URL of the text ad when website
/// description is the selected display mode for vanity pharma URLs.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VanityPharmaTextEnum {}
/// Nested message and enum types in `VanityPharmaTextEnum`.
pub mod vanity_pharma_text_enum {
    /// Enum describing possible text.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum VanityPharmaText {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Prescription treatment website with website content in English.
        PrescriptionTreatmentWebsiteEn = 2,
        /// Prescription treatment website with website content in Spanish
        /// (Sitio de tratamientos con receta).
        PrescriptionTreatmentWebsiteEs = 3,
        /// Prescription device website with website content in English.
        PrescriptionDeviceWebsiteEn = 4,
        /// Prescription device website with website content in Spanish (Sitio de
        /// dispositivos con receta).
        PrescriptionDeviceWebsiteEs = 5,
        /// Medical device website with website content in English.
        MedicalDeviceWebsiteEn = 6,
        /// Medical device website with website content in Spanish (Sitio de
        /// dispositivos médicos).
        MedicalDeviceWebsiteEs = 7,
        /// Preventative treatment website with website content in English.
        PreventativeTreatmentWebsiteEn = 8,
        /// Preventative treatment website with website content in Spanish (Sitio de
        /// tratamientos preventivos).
        PreventativeTreatmentWebsiteEs = 9,
        /// Prescription contraception website with website content in English.
        PrescriptionContraceptionWebsiteEn = 10,
        /// Prescription contraception website with website content in Spanish (Sitio
        /// de anticonceptivos con receta).
        PrescriptionContraceptionWebsiteEs = 11,
        /// Prescription vaccine website with website content in English.
        PrescriptionVaccineWebsiteEn = 12,
        /// Prescription vaccine website with website content in Spanish (Sitio de
        /// vacunas con receta).
        PrescriptionVaccineWebsiteEs = 13,
    }
}
