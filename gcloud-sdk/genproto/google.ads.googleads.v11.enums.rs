/// Container for enum describing possible policy topic entry types.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PolicyTopicEntryTypeEnum {}
/// Nested message and enum types in `PolicyTopicEntryTypeEnum`.
pub mod policy_topic_entry_type_enum {
    /// The possible policy topic entry types.
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
    impl PolicyTopicEntryType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                PolicyTopicEntryType::Unspecified => "UNSPECIFIED",
                PolicyTopicEntryType::Unknown => "UNKNOWN",
                PolicyTopicEntryType::Prohibited => "PROHIBITED",
                PolicyTopicEntryType::Limited => "LIMITED",
                PolicyTopicEntryType::FullyLimited => "FULLY_LIMITED",
                PolicyTopicEntryType::Descriptive => "DESCRIPTIVE",
                PolicyTopicEntryType::Broadening => "BROADENING",
                PolicyTopicEntryType::AreaOfInterestOnly => "AREA_OF_INTEREST_ONLY",
            }
        }
    }
}
/// Container for enum describing possible policy topic evidence destination
/// mismatch url types.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PolicyTopicEvidenceDestinationMismatchUrlTypeEnum {}
/// Nested message and enum types in `PolicyTopicEvidenceDestinationMismatchUrlTypeEnum`.
pub mod policy_topic_evidence_destination_mismatch_url_type_enum {
    /// The possible policy topic evidence destination mismatch url types.
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
    impl PolicyTopicEvidenceDestinationMismatchUrlType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                PolicyTopicEvidenceDestinationMismatchUrlType::Unspecified => {
                    "UNSPECIFIED"
                }
                PolicyTopicEvidenceDestinationMismatchUrlType::Unknown => "UNKNOWN",
                PolicyTopicEvidenceDestinationMismatchUrlType::DisplayUrl => {
                    "DISPLAY_URL"
                }
                PolicyTopicEvidenceDestinationMismatchUrlType::FinalUrl => "FINAL_URL",
                PolicyTopicEvidenceDestinationMismatchUrlType::FinalMobileUrl => {
                    "FINAL_MOBILE_URL"
                }
                PolicyTopicEvidenceDestinationMismatchUrlType::TrackingUrl => {
                    "TRACKING_URL"
                }
                PolicyTopicEvidenceDestinationMismatchUrlType::MobileTrackingUrl => {
                    "MOBILE_TRACKING_URL"
                }
            }
        }
    }
}
/// Container for enum describing possible policy topic evidence destination not
/// working devices.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PolicyTopicEvidenceDestinationNotWorkingDeviceEnum {}
/// Nested message and enum types in `PolicyTopicEvidenceDestinationNotWorkingDeviceEnum`.
pub mod policy_topic_evidence_destination_not_working_device_enum {
    /// The possible policy topic evidence destination not working devices.
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
    impl PolicyTopicEvidenceDestinationNotWorkingDevice {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                PolicyTopicEvidenceDestinationNotWorkingDevice::Unspecified => {
                    "UNSPECIFIED"
                }
                PolicyTopicEvidenceDestinationNotWorkingDevice::Unknown => "UNKNOWN",
                PolicyTopicEvidenceDestinationNotWorkingDevice::Desktop => "DESKTOP",
                PolicyTopicEvidenceDestinationNotWorkingDevice::Android => "ANDROID",
                PolicyTopicEvidenceDestinationNotWorkingDevice::Ios => "IOS",
            }
        }
    }
}
/// Container for enum describing possible policy topic evidence destination not
/// working DNS error types.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PolicyTopicEvidenceDestinationNotWorkingDnsErrorTypeEnum {}
/// Nested message and enum types in `PolicyTopicEvidenceDestinationNotWorkingDnsErrorTypeEnum`.
pub mod policy_topic_evidence_destination_not_working_dns_error_type_enum {
    /// The possible policy topic evidence destination not working DNS error types.
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
    impl PolicyTopicEvidenceDestinationNotWorkingDnsErrorType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                PolicyTopicEvidenceDestinationNotWorkingDnsErrorType::Unspecified => {
                    "UNSPECIFIED"
                }
                PolicyTopicEvidenceDestinationNotWorkingDnsErrorType::Unknown => {
                    "UNKNOWN"
                }
                PolicyTopicEvidenceDestinationNotWorkingDnsErrorType::HostnameNotFound => {
                    "HOSTNAME_NOT_FOUND"
                }
                PolicyTopicEvidenceDestinationNotWorkingDnsErrorType::GoogleCrawlerDnsIssue => {
                    "GOOGLE_CRAWLER_DNS_ISSUE"
                }
            }
        }
    }
}
/// Container for enum describing possible policy approval statuses.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PolicyApprovalStatusEnum {}
/// Nested message and enum types in `PolicyApprovalStatusEnum`.
pub mod policy_approval_status_enum {
    /// The possible policy approval statuses. When there are several approval
    /// statuses available the most severe one will be used. The order of severity
    /// is DISAPPROVED, AREA_OF_INTEREST_ONLY, APPROVED_LIMITED and APPROVED.
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
    impl PolicyApprovalStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                PolicyApprovalStatus::Unspecified => "UNSPECIFIED",
                PolicyApprovalStatus::Unknown => "UNKNOWN",
                PolicyApprovalStatus::Disapproved => "DISAPPROVED",
                PolicyApprovalStatus::ApprovedLimited => "APPROVED_LIMITED",
                PolicyApprovalStatus::Approved => "APPROVED",
                PolicyApprovalStatus::AreaOfInterestOnly => "AREA_OF_INTEREST_ONLY",
            }
        }
    }
}
/// Container for enum describing possible policy review statuses.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PolicyReviewStatusEnum {}
/// Nested message and enum types in `PolicyReviewStatusEnum`.
pub mod policy_review_status_enum {
    /// The possible policy review statuses.
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
    impl PolicyReviewStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                PolicyReviewStatus::Unspecified => "UNSPECIFIED",
                PolicyReviewStatus::Unknown => "UNKNOWN",
                PolicyReviewStatus::ReviewInProgress => "REVIEW_IN_PROGRESS",
                PolicyReviewStatus::Reviewed => "REVIEWED",
                PolicyReviewStatus::UnderAppeal => "UNDER_APPEAL",
                PolicyReviewStatus::EligibleMayServe => "ELIGIBLE_MAY_SERVE",
            }
        }
    }
}
/// Container for enum describing the performance label of an asset.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetPerformanceLabelEnum {}
/// Nested message and enum types in `AssetPerformanceLabelEnum`.
pub mod asset_performance_label_enum {
    /// Enum describing the possible performance labels of an asset, usually
    /// computed in the context of a linkage.
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
    impl AssetPerformanceLabel {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                AssetPerformanceLabel::Unspecified => "UNSPECIFIED",
                AssetPerformanceLabel::Unknown => "UNKNOWN",
                AssetPerformanceLabel::Pending => "PENDING",
                AssetPerformanceLabel::Learning => "LEARNING",
                AssetPerformanceLabel::Low => "LOW",
                AssetPerformanceLabel::Good => "GOOD",
                AssetPerformanceLabel::Best => "BEST",
            }
        }
    }
}
/// Container for enum describing possible asset field types.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServedAssetFieldTypeEnum {}
/// Nested message and enum types in `ServedAssetFieldTypeEnum`.
pub mod served_asset_field_type_enum {
    /// The possible asset field types.
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
    impl ServedAssetFieldType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ServedAssetFieldType::Unspecified => "UNSPECIFIED",
                ServedAssetFieldType::Unknown => "UNKNOWN",
                ServedAssetFieldType::Headline1 => "HEADLINE_1",
                ServedAssetFieldType::Headline2 => "HEADLINE_2",
                ServedAssetFieldType::Headline3 => "HEADLINE_3",
                ServedAssetFieldType::Description1 => "DESCRIPTION_1",
                ServedAssetFieldType::Description2 => "DESCRIPTION_2",
            }
        }
    }
}
/// Container for enum describing possible data types for call conversion
/// reporting state.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CallConversionReportingStateEnum {}
/// Nested message and enum types in `CallConversionReportingStateEnum`.
pub mod call_conversion_reporting_state_enum {
    /// Possible data types for a call conversion action state.
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
    impl CallConversionReportingState {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                CallConversionReportingState::Unspecified => "UNSPECIFIED",
                CallConversionReportingState::Unknown => "UNKNOWN",
                CallConversionReportingState::Disabled => "DISABLED",
                CallConversionReportingState::UseAccountLevelCallConversionAction => {
                    "USE_ACCOUNT_LEVEL_CALL_CONVERSION_ACTION"
                }
                CallConversionReportingState::UseResourceLevelCallConversionAction => {
                    "USE_RESOURCE_LEVEL_CALL_CONVERSION_ACTION"
                }
            }
        }
    }
}
/// Container for display ad format settings.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DisplayAdFormatSettingEnum {}
/// Nested message and enum types in `DisplayAdFormatSettingEnum`.
pub mod display_ad_format_setting_enum {
    /// Enumerates display ad format settings.
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
    pub enum DisplayAdFormatSetting {
        /// Not specified.
        Unspecified = 0,
        /// The value is unknown in this version.
        Unknown = 1,
        /// Text, image and native formats.
        AllFormats = 2,
        /// Text and image formats.
        NonNative = 3,
        /// Native format, for example, the format rendering is controlled by the
        /// publisher and not by Google.
        Native = 4,
    }
    impl DisplayAdFormatSetting {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                DisplayAdFormatSetting::Unspecified => "UNSPECIFIED",
                DisplayAdFormatSetting::Unknown => "UNKNOWN",
                DisplayAdFormatSetting::AllFormats => "ALL_FORMATS",
                DisplayAdFormatSetting::NonNative => "NON_NATIVE",
                DisplayAdFormatSetting::Native => "NATIVE",
            }
        }
    }
}
/// Container for display upload product types. Product types that have the word
/// "DYNAMIC" in them must be associated with a campaign that has a dynamic
/// remarketing feed. See <https://support.google.com/google-ads/answer/6053288>
/// for more info about dynamic remarketing. Other product types are regarded
/// as "static" and do not have this requirement.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DisplayUploadProductTypeEnum {}
/// Nested message and enum types in `DisplayUploadProductTypeEnum`.
pub mod display_upload_product_type_enum {
    /// Enumerates display upload product types.
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
    impl DisplayUploadProductType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                DisplayUploadProductType::Unspecified => "UNSPECIFIED",
                DisplayUploadProductType::Unknown => "UNKNOWN",
                DisplayUploadProductType::Html5UploadAd => "HTML5_UPLOAD_AD",
                DisplayUploadProductType::DynamicHtml5EducationAd => {
                    "DYNAMIC_HTML5_EDUCATION_AD"
                }
                DisplayUploadProductType::DynamicHtml5FlightAd => {
                    "DYNAMIC_HTML5_FLIGHT_AD"
                }
                DisplayUploadProductType::DynamicHtml5HotelRentalAd => {
                    "DYNAMIC_HTML5_HOTEL_RENTAL_AD"
                }
                DisplayUploadProductType::DynamicHtml5JobAd => "DYNAMIC_HTML5_JOB_AD",
                DisplayUploadProductType::DynamicHtml5LocalAd => "DYNAMIC_HTML5_LOCAL_AD",
                DisplayUploadProductType::DynamicHtml5RealEstateAd => {
                    "DYNAMIC_HTML5_REAL_ESTATE_AD"
                }
                DisplayUploadProductType::DynamicHtml5CustomAd => {
                    "DYNAMIC_HTML5_CUSTOM_AD"
                }
                DisplayUploadProductType::DynamicHtml5TravelAd => {
                    "DYNAMIC_HTML5_TRAVEL_AD"
                }
                DisplayUploadProductType::DynamicHtml5HotelAd => "DYNAMIC_HTML5_HOTEL_AD",
            }
        }
    }
}
/// Container for enum describing app store type in a legacy app install ad.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LegacyAppInstallAdAppStoreEnum {}
/// Nested message and enum types in `LegacyAppInstallAdAppStoreEnum`.
pub mod legacy_app_install_ad_app_store_enum {
    /// App store type in a legacy app install ad.
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
    impl LegacyAppInstallAdAppStore {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                LegacyAppInstallAdAppStore::Unspecified => "UNSPECIFIED",
                LegacyAppInstallAdAppStore::Unknown => "UNKNOWN",
                LegacyAppInstallAdAppStore::AppleAppStore => "APPLE_APP_STORE",
                LegacyAppInstallAdAppStore::GooglePlay => "GOOGLE_PLAY",
                LegacyAppInstallAdAppStore::WindowsStore => "WINDOWS_STORE",
                LegacyAppInstallAdAppStore::WindowsPhoneStore => "WINDOWS_PHONE_STORE",
                LegacyAppInstallAdAppStore::CnAppStore => "CN_APP_STORE",
            }
        }
    }
}
/// Container for enum describing the mime types.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MimeTypeEnum {}
/// Nested message and enum types in `MimeTypeEnum`.
pub mod mime_type_enum {
    /// The mime type
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
    impl MimeType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                MimeType::Unspecified => "UNSPECIFIED",
                MimeType::Unknown => "UNKNOWN",
                MimeType::ImageJpeg => "IMAGE_JPEG",
                MimeType::ImageGif => "IMAGE_GIF",
                MimeType::ImagePng => "IMAGE_PNG",
                MimeType::Flash => "FLASH",
                MimeType::TextHtml => "TEXT_HTML",
                MimeType::Pdf => "PDF",
                MimeType::Msword => "MSWORD",
                MimeType::Msexcel => "MSEXCEL",
                MimeType::Rtf => "RTF",
                MimeType::AudioWav => "AUDIO_WAV",
                MimeType::AudioMp3 => "AUDIO_MP3",
                MimeType::Html5AdZip => "HTML5_AD_ZIP",
            }
        }
    }
}
/// Defines the thumbnail to use for In-Display video ads. Note that
/// DEFAULT_THUMBNAIL may have been uploaded by the user while thumbnails 1-3 are
/// auto-generated from the video.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VideoThumbnailEnum {}
/// Nested message and enum types in `VideoThumbnailEnum`.
pub mod video_thumbnail_enum {
    /// Enum listing the possible types of a video thumbnail.
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
    pub enum VideoThumbnail {
        /// The type has not been specified.
        Unspecified = 0,
        /// The received value is not known in this version.
        /// This is a response-only value.
        Unknown = 1,
        /// The default thumbnail. Can be auto-generated or user-uploaded.
        DefaultThumbnail = 2,
        /// Thumbnail 1, generated from the video.
        Thumbnail1 = 3,
        /// Thumbnail 2, generated from the video.
        Thumbnail2 = 4,
        /// Thumbnail 3, generated from the video.
        Thumbnail3 = 5,
    }
    impl VideoThumbnail {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                VideoThumbnail::Unspecified => "UNSPECIFIED",
                VideoThumbnail::Unknown => "UNKNOWN",
                VideoThumbnail::DefaultThumbnail => "DEFAULT_THUMBNAIL",
                VideoThumbnail::Thumbnail1 => "THUMBNAIL_1",
                VideoThumbnail::Thumbnail2 => "THUMBNAIL_2",
                VideoThumbnail::Thumbnail3 => "THUMBNAIL_3",
            }
        }
    }
}
/// Container for enum describing the type of demographic age ranges.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AgeRangeTypeEnum {}
/// Nested message and enum types in `AgeRangeTypeEnum`.
pub mod age_range_type_enum {
    /// The type of demographic age ranges (for example, between 18 and 24 years
    /// old).
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
    impl AgeRangeType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                AgeRangeType::Unspecified => "UNSPECIFIED",
                AgeRangeType::Unknown => "UNKNOWN",
                AgeRangeType::AgeRange1824 => "AGE_RANGE_18_24",
                AgeRangeType::AgeRange2534 => "AGE_RANGE_25_34",
                AgeRangeType::AgeRange3544 => "AGE_RANGE_35_44",
                AgeRangeType::AgeRange4554 => "AGE_RANGE_45_54",
                AgeRangeType::AgeRange5564 => "AGE_RANGE_55_64",
                AgeRangeType::AgeRange65Up => "AGE_RANGE_65_UP",
                AgeRangeType::AgeRangeUndetermined => "AGE_RANGE_UNDETERMINED",
            }
        }
    }
}
/// Represents a criterion for targeting paid apps.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AppPaymentModelTypeEnum {}
/// Nested message and enum types in `AppPaymentModelTypeEnum`.
pub mod app_payment_model_type_enum {
    /// Enum describing possible app payment models.
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
    pub enum AppPaymentModelType {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Represents paid-for apps.
        Paid = 30,
    }
    impl AppPaymentModelType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                AppPaymentModelType::Unspecified => "UNSPECIFIED",
                AppPaymentModelType::Unknown => "UNKNOWN",
                AppPaymentModelType::Paid => "PAID",
            }
        }
    }
}
/// Container for enum describing content label types in ContentLabel.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContentLabelTypeEnum {}
/// Nested message and enum types in `ContentLabelTypeEnum`.
pub mod content_label_type_enum {
    /// Enum listing the content label types supported by ContentLabel criterion.
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
    impl ContentLabelType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ContentLabelType::Unspecified => "UNSPECIFIED",
                ContentLabelType::Unknown => "UNKNOWN",
                ContentLabelType::SexuallySuggestive => "SEXUALLY_SUGGESTIVE",
                ContentLabelType::BelowTheFold => "BELOW_THE_FOLD",
                ContentLabelType::ParkedDomain => "PARKED_DOMAIN",
                ContentLabelType::Juvenile => "JUVENILE",
                ContentLabelType::Profanity => "PROFANITY",
                ContentLabelType::Tragedy => "TRAGEDY",
                ContentLabelType::Video => "VIDEO",
                ContentLabelType::VideoRatingDvG => "VIDEO_RATING_DV_G",
                ContentLabelType::VideoRatingDvPg => "VIDEO_RATING_DV_PG",
                ContentLabelType::VideoRatingDvT => "VIDEO_RATING_DV_T",
                ContentLabelType::VideoRatingDvMa => "VIDEO_RATING_DV_MA",
                ContentLabelType::VideoNotYetRated => "VIDEO_NOT_YET_RATED",
                ContentLabelType::EmbeddedVideo => "EMBEDDED_VIDEO",
                ContentLabelType::LiveStreamingVideo => "LIVE_STREAMING_VIDEO",
                ContentLabelType::SocialIssues => "SOCIAL_ISSUES",
            }
        }
    }
}
/// Container for enumeration of days of the week, for example, "Monday".
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DayOfWeekEnum {}
/// Nested message and enum types in `DayOfWeekEnum`.
pub mod day_of_week_enum {
    /// Enumerates days of the week, for example, "Monday".
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
    impl DayOfWeek {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                DayOfWeek::Unspecified => "UNSPECIFIED",
                DayOfWeek::Unknown => "UNKNOWN",
                DayOfWeek::Monday => "MONDAY",
                DayOfWeek::Tuesday => "TUESDAY",
                DayOfWeek::Wednesday => "WEDNESDAY",
                DayOfWeek::Thursday => "THURSDAY",
                DayOfWeek::Friday => "FRIDAY",
                DayOfWeek::Saturday => "SATURDAY",
                DayOfWeek::Sunday => "SUNDAY",
            }
        }
    }
}
/// Container for enumeration of Google Ads devices available for targeting.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeviceEnum {}
/// Nested message and enum types in `DeviceEnum`.
pub mod device_enum {
    /// Enumerates Google Ads devices available for targeting.
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
    impl Device {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Device::Unspecified => "UNSPECIFIED",
                Device::Unknown => "UNKNOWN",
                Device::Mobile => "MOBILE",
                Device::Tablet => "TABLET",
                Device::Desktop => "DESKTOP",
                Device::ConnectedTv => "CONNECTED_TV",
                Device::Other => "OTHER",
            }
        }
    }
}
/// Container for enum describing the type of demographic genders.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenderTypeEnum {}
/// Nested message and enum types in `GenderTypeEnum`.
pub mod gender_type_enum {
    /// The type of demographic genders (for example, female).
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
    impl GenderType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                GenderType::Unspecified => "UNSPECIFIED",
                GenderType::Unknown => "UNKNOWN",
                GenderType::Male => "MALE",
                GenderType::Female => "FEMALE",
                GenderType::Undetermined => "UNDETERMINED",
            }
        }
    }
}
/// Container for enum describing possible hotel date selection types
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HotelDateSelectionTypeEnum {}
/// Nested message and enum types in `HotelDateSelectionTypeEnum`.
pub mod hotel_date_selection_type_enum {
    /// Enum describing possible hotel date selection types.
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
    impl HotelDateSelectionType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                HotelDateSelectionType::Unspecified => "UNSPECIFIED",
                HotelDateSelectionType::Unknown => "UNKNOWN",
                HotelDateSelectionType::DefaultSelection => "DEFAULT_SELECTION",
                HotelDateSelectionType::UserSelected => "USER_SELECTED",
            }
        }
    }
}
/// Container for enum describing the type of demographic income ranges.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IncomeRangeTypeEnum {}
/// Nested message and enum types in `IncomeRangeTypeEnum`.
pub mod income_range_type_enum {
    /// The type of demographic income ranges (for example, between 0% to 50%).
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
    impl IncomeRangeType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                IncomeRangeType::Unspecified => "UNSPECIFIED",
                IncomeRangeType::Unknown => "UNKNOWN",
                IncomeRangeType::IncomeRange050 => "INCOME_RANGE_0_50",
                IncomeRangeType::IncomeRange5060 => "INCOME_RANGE_50_60",
                IncomeRangeType::IncomeRange6070 => "INCOME_RANGE_60_70",
                IncomeRangeType::IncomeRange7080 => "INCOME_RANGE_70_80",
                IncomeRangeType::IncomeRange8090 => "INCOME_RANGE_80_90",
                IncomeRangeType::IncomeRange90Up => "INCOME_RANGE_90_UP",
                IncomeRangeType::IncomeRangeUndetermined => "INCOME_RANGE_UNDETERMINED",
            }
        }
    }
}
/// Container for enum describing possible interaction types.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InteractionTypeEnum {}
/// Nested message and enum types in `InteractionTypeEnum`.
pub mod interaction_type_enum {
    /// Enum describing possible interaction types.
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
    pub enum InteractionType {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Calls.
        Calls = 8000,
    }
    impl InteractionType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                InteractionType::Unspecified => "UNSPECIFIED",
                InteractionType::Unknown => "UNKNOWN",
                InteractionType::Calls => "CALLS",
            }
        }
    }
}
/// Message describing Keyword match types.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeywordMatchTypeEnum {}
/// Nested message and enum types in `KeywordMatchTypeEnum`.
pub mod keyword_match_type_enum {
    /// Possible Keyword match types.
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
    impl KeywordMatchType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                KeywordMatchType::Unspecified => "UNSPECIFIED",
                KeywordMatchType::Unknown => "UNKNOWN",
                KeywordMatchType::Exact => "EXACT",
                KeywordMatchType::Phrase => "PHRASE",
                KeywordMatchType::Broad => "BROAD",
            }
        }
    }
}
/// Container for enum describing the type of the listing group.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListingGroupTypeEnum {}
/// Nested message and enum types in `ListingGroupTypeEnum`.
pub mod listing_group_type_enum {
    /// The type of the listing group.
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
    impl ListingGroupType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ListingGroupType::Unspecified => "UNSPECIFIED",
                ListingGroupType::Unknown => "UNKNOWN",
                ListingGroupType::Subdivision => "SUBDIVISION",
                ListingGroupType::Unit => "UNIT",
            }
        }
    }
}
/// Container for enum describing unit of radius in location group.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LocationGroupRadiusUnitsEnum {}
/// Nested message and enum types in `LocationGroupRadiusUnitsEnum`.
pub mod location_group_radius_units_enum {
    /// The unit of radius distance in location group (for example, MILES)
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
    impl LocationGroupRadiusUnits {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                LocationGroupRadiusUnits::Unspecified => "UNSPECIFIED",
                LocationGroupRadiusUnits::Unknown => "UNKNOWN",
                LocationGroupRadiusUnits::Meters => "METERS",
                LocationGroupRadiusUnits::Miles => "MILES",
                LocationGroupRadiusUnits::MilliMiles => "MILLI_MILES",
            }
        }
    }
}
/// Container for enumeration of quarter-hours.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MinuteOfHourEnum {}
/// Nested message and enum types in `MinuteOfHourEnum`.
pub mod minute_of_hour_enum {
    /// Enumerates of quarter-hours. For example, "FIFTEEN"
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
    impl MinuteOfHour {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                MinuteOfHour::Unspecified => "UNSPECIFIED",
                MinuteOfHour::Unknown => "UNKNOWN",
                MinuteOfHour::Zero => "ZERO",
                MinuteOfHour::Fifteen => "FIFTEEN",
                MinuteOfHour::Thirty => "THIRTY",
                MinuteOfHour::FortyFive => "FORTY_FIVE",
            }
        }
    }
}
/// Container for enum describing the type of demographic parental statuses.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ParentalStatusTypeEnum {}
/// Nested message and enum types in `ParentalStatusTypeEnum`.
pub mod parental_status_type_enum {
    /// The type of parental statuses (for example, not a parent).
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
    impl ParentalStatusType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ParentalStatusType::Unspecified => "UNSPECIFIED",
                ParentalStatusType::Unknown => "UNKNOWN",
                ParentalStatusType::Parent => "PARENT",
                ParentalStatusType::NotAParent => "NOT_A_PARENT",
                ParentalStatusType::Undetermined => "UNDETERMINED",
            }
        }
    }
}
/// Container for enumeration of preferred content criterion type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PreferredContentTypeEnum {}
/// Nested message and enum types in `PreferredContentTypeEnum`.
pub mod preferred_content_type_enum {
    /// Enumerates preferred content criterion type.
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
    pub enum PreferredContentType {
        /// Not specified.
        Unspecified = 0,
        /// The value is unknown in this version.
        Unknown = 1,
        /// Represents top content on YouTube.
        YoutubeTopContent = 400,
    }
    impl PreferredContentType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                PreferredContentType::Unspecified => "UNSPECIFIED",
                PreferredContentType::Unknown => "UNKNOWN",
                PreferredContentType::YoutubeTopContent => "YOUTUBE_TOP_CONTENT",
            }
        }
    }
}
/// Level of a product bidding category.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProductBiddingCategoryLevelEnum {}
/// Nested message and enum types in `ProductBiddingCategoryLevelEnum`.
pub mod product_bidding_category_level_enum {
    /// Enum describing the level of the product bidding category.
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
    impl ProductBiddingCategoryLevel {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ProductBiddingCategoryLevel::Unspecified => "UNSPECIFIED",
                ProductBiddingCategoryLevel::Unknown => "UNKNOWN",
                ProductBiddingCategoryLevel::Level1 => "LEVEL1",
                ProductBiddingCategoryLevel::Level2 => "LEVEL2",
                ProductBiddingCategoryLevel::Level3 => "LEVEL3",
                ProductBiddingCategoryLevel::Level4 => "LEVEL4",
                ProductBiddingCategoryLevel::Level5 => "LEVEL5",
            }
        }
    }
}
/// Locality of a product offer.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProductChannelEnum {}
/// Nested message and enum types in `ProductChannelEnum`.
pub mod product_channel_enum {
    /// Enum describing the locality of a product offer.
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
    impl ProductChannel {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ProductChannel::Unspecified => "UNSPECIFIED",
                ProductChannel::Unknown => "UNKNOWN",
                ProductChannel::Online => "ONLINE",
                ProductChannel::Local => "LOCAL",
            }
        }
    }
}
/// Availability of a product offer.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProductChannelExclusivityEnum {}
/// Nested message and enum types in `ProductChannelExclusivityEnum`.
pub mod product_channel_exclusivity_enum {
    /// Enum describing the availability of a product offer.
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
    impl ProductChannelExclusivity {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ProductChannelExclusivity::Unspecified => "UNSPECIFIED",
                ProductChannelExclusivity::Unknown => "UNKNOWN",
                ProductChannelExclusivity::SingleChannel => "SINGLE_CHANNEL",
                ProductChannelExclusivity::MultiChannel => "MULTI_CHANNEL",
            }
        }
    }
}
/// Condition of a product offer.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProductConditionEnum {}
/// Nested message and enum types in `ProductConditionEnum`.
pub mod product_condition_enum {
    /// Enum describing the condition of a product offer.
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
    impl ProductCondition {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ProductCondition::Unspecified => "UNSPECIFIED",
                ProductCondition::Unknown => "UNKNOWN",
                ProductCondition::New => "NEW",
                ProductCondition::Refurbished => "REFURBISHED",
                ProductCondition::Used => "USED",
            }
        }
    }
}
/// Container for enum describing the index of the product custom attribute.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProductCustomAttributeIndexEnum {}
/// Nested message and enum types in `ProductCustomAttributeIndexEnum`.
pub mod product_custom_attribute_index_enum {
    /// The index of the product custom attribute.
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
    impl ProductCustomAttributeIndex {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ProductCustomAttributeIndex::Unspecified => "UNSPECIFIED",
                ProductCustomAttributeIndex::Unknown => "UNKNOWN",
                ProductCustomAttributeIndex::Index0 => "INDEX0",
                ProductCustomAttributeIndex::Index1 => "INDEX1",
                ProductCustomAttributeIndex::Index2 => "INDEX2",
                ProductCustomAttributeIndex::Index3 => "INDEX3",
                ProductCustomAttributeIndex::Index4 => "INDEX4",
            }
        }
    }
}
/// Level of the type of a product offer.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProductTypeLevelEnum {}
/// Nested message and enum types in `ProductTypeLevelEnum`.
pub mod product_type_level_enum {
    /// Enum describing the level of the type of a product offer.
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
    impl ProductTypeLevel {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ProductTypeLevel::Unspecified => "UNSPECIFIED",
                ProductTypeLevel::Unknown => "UNKNOWN",
                ProductTypeLevel::Level1 => "LEVEL1",
                ProductTypeLevel::Level2 => "LEVEL2",
                ProductTypeLevel::Level3 => "LEVEL3",
                ProductTypeLevel::Level4 => "LEVEL4",
                ProductTypeLevel::Level5 => "LEVEL5",
            }
        }
    }
}
/// Container for enum describing unit of radius in proximity.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProximityRadiusUnitsEnum {}
/// Nested message and enum types in `ProximityRadiusUnitsEnum`.
pub mod proximity_radius_units_enum {
    /// The unit of radius distance in proximity (for example, MILES)
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
    impl ProximityRadiusUnits {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ProximityRadiusUnits::Unspecified => "UNSPECIFIED",
                ProximityRadiusUnits::Unknown => "UNKNOWN",
                ProximityRadiusUnits::Miles => "MILES",
                ProximityRadiusUnits::Kilometers => "KILOMETERS",
            }
        }
    }
}
/// Container for enum describing webpage condition operand in webpage criterion.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WebpageConditionOperandEnum {}
/// Nested message and enum types in `WebpageConditionOperandEnum`.
pub mod webpage_condition_operand_enum {
    /// The webpage condition operand in webpage criterion.
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
    impl WebpageConditionOperand {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                WebpageConditionOperand::Unspecified => "UNSPECIFIED",
                WebpageConditionOperand::Unknown => "UNKNOWN",
                WebpageConditionOperand::Url => "URL",
                WebpageConditionOperand::Category => "CATEGORY",
                WebpageConditionOperand::PageTitle => "PAGE_TITLE",
                WebpageConditionOperand::PageContent => "PAGE_CONTENT",
                WebpageConditionOperand::CustomLabel => "CUSTOM_LABEL",
            }
        }
    }
}
/// Container for enum describing webpage condition operator in webpage
/// criterion.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WebpageConditionOperatorEnum {}
/// Nested message and enum types in `WebpageConditionOperatorEnum`.
pub mod webpage_condition_operator_enum {
    /// The webpage condition operator in webpage criterion.
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
    impl WebpageConditionOperator {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                WebpageConditionOperator::Unspecified => "UNSPECIFIED",
                WebpageConditionOperator::Unknown => "UNKNOWN",
                WebpageConditionOperator::Equals => "EQUALS",
                WebpageConditionOperator::Contains => "CONTAINS",
            }
        }
    }
}
/// Container for enum describing the call to action types.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CallToActionTypeEnum {}
/// Nested message and enum types in `CallToActionTypeEnum`.
pub mod call_to_action_type_enum {
    /// Enum describing possible types of call to action.
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
    pub enum CallToActionType {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// The call to action type is learn more.
        LearnMore = 2,
        /// The call to action type is get quote.
        GetQuote = 3,
        /// The call to action type is apply now.
        ApplyNow = 4,
        /// The call to action type is sign up.
        SignUp = 5,
        /// The call to action type is contact us.
        ContactUs = 6,
        /// The call to action type is subscribe.
        Subscribe = 7,
        /// The call to action type is download.
        Download = 8,
        /// The call to action type is book now.
        BookNow = 9,
        /// The call to action type is shop now.
        ShopNow = 10,
    }
    impl CallToActionType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                CallToActionType::Unspecified => "UNSPECIFIED",
                CallToActionType::Unknown => "UNKNOWN",
                CallToActionType::LearnMore => "LEARN_MORE",
                CallToActionType::GetQuote => "GET_QUOTE",
                CallToActionType::ApplyNow => "APPLY_NOW",
                CallToActionType::SignUp => "SIGN_UP",
                CallToActionType::ContactUs => "CONTACT_US",
                CallToActionType::Subscribe => "SUBSCRIBE",
                CallToActionType::Download => "DOWNLOAD",
                CallToActionType::BookNow => "BOOK_NOW",
                CallToActionType::ShopNow => "SHOP_NOW",
            }
        }
    }
}
/// Describes the type of call-to-action phrases in a lead form.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LeadFormCallToActionTypeEnum {}
/// Nested message and enum types in `LeadFormCallToActionTypeEnum`.
pub mod lead_form_call_to_action_type_enum {
    /// Enum describing the type of call-to-action phrases in a lead form.
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
    impl LeadFormCallToActionType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                LeadFormCallToActionType::Unspecified => "UNSPECIFIED",
                LeadFormCallToActionType::Unknown => "UNKNOWN",
                LeadFormCallToActionType::LearnMore => "LEARN_MORE",
                LeadFormCallToActionType::GetQuote => "GET_QUOTE",
                LeadFormCallToActionType::ApplyNow => "APPLY_NOW",
                LeadFormCallToActionType::SignUp => "SIGN_UP",
                LeadFormCallToActionType::ContactUs => "CONTACT_US",
                LeadFormCallToActionType::Subscribe => "SUBSCRIBE",
                LeadFormCallToActionType::Download => "DOWNLOAD",
                LeadFormCallToActionType::BookNow => "BOOK_NOW",
                LeadFormCallToActionType::GetOffer => "GET_OFFER",
                LeadFormCallToActionType::Register => "REGISTER",
                LeadFormCallToActionType::GetInfo => "GET_INFO",
                LeadFormCallToActionType::RequestDemo => "REQUEST_DEMO",
                LeadFormCallToActionType::JoinNow => "JOIN_NOW",
                LeadFormCallToActionType::GetStarted => "GET_STARTED",
            }
        }
    }
}
/// Describes the chosen level of intent of generated leads.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LeadFormDesiredIntentEnum {}
/// Nested message and enum types in `LeadFormDesiredIntentEnum`.
pub mod lead_form_desired_intent_enum {
    /// Enum describing the chosen level of intent of generated leads.
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
    pub enum LeadFormDesiredIntent {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Deliver more leads at a potentially lower quality.
        LowIntent = 2,
        /// Deliver leads that are more qualified.
        HighIntent = 3,
    }
    impl LeadFormDesiredIntent {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                LeadFormDesiredIntent::Unspecified => "UNSPECIFIED",
                LeadFormDesiredIntent::Unknown => "UNKNOWN",
                LeadFormDesiredIntent::LowIntent => "LOW_INTENT",
                LeadFormDesiredIntent::HighIntent => "HIGH_INTENT",
            }
        }
    }
}
/// Describes the input type of a lead form field.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LeadFormFieldUserInputTypeEnum {}
/// Nested message and enum types in `LeadFormFieldUserInputTypeEnum`.
pub mod lead_form_field_user_input_type_enum {
    /// Enum describing the input type of a lead form field.
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
    pub enum LeadFormFieldUserInputType {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// The user will be asked to fill in their given and family name. This field
        /// cannot be set at the same time as GIVEN_NAME or FAMILY_NAME.
        FullName = 2,
        /// The user will be asked to fill in their email address.
        Email = 3,
        /// The user will be asked to fill in their phone number.
        PhoneNumber = 4,
        /// The user will be asked to fill in their zip code.
        PostalCode = 5,
        /// The user will be asked to fill in their street address.
        StreetAddress = 8,
        /// The user will be asked to fill in their city.
        City = 9,
        /// The user will be asked to fill in their region part of the address (for
        /// example, state for US, province for Canada).
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
        /// The user will be asked to fill in their CPF for Brazil users.
        GovernmentIssuedIdCpfBr = 16,
        /// The user will be asked to fill in their DNI for Argentina users.
        GovernmentIssuedIdDniAr = 17,
        /// The user will be asked to fill in their DNI for Peru users.
        GovernmentIssuedIdDniPe = 18,
        /// The user will be asked to fill in their RUT for Chile users.
        GovernmentIssuedIdRutCl = 19,
        /// The user will be asked to fill in their CC for Colombia users.
        GovernmentIssuedIdCcCo = 20,
        /// The user will be asked to fill in their CI for Ecuador users.
        GovernmentIssuedIdCiEc = 21,
        /// The user will be asked to fill in their RFC for Mexico users.
        GovernmentIssuedIdRfcMx = 22,
        /// The user will be asked to fill in their first name. This
        /// field can not be set at the same time as FULL_NAME.
        FirstName = 23,
        /// The user will be asked to fill in their last name. This
        /// field can not be set at the same time as FULL_NAME.
        LastName = 24,
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
        /// Question: "Are you over 18 years of age?"
        /// Category: "Demographics"
        Over18Age = 1078,
        /// Question: "Are you over 19 years of age?"
        /// Category: "Demographics"
        Over19Age = 1079,
        /// Question: "Are you over 20 years of age?"
        /// Category: "Demographics"
        Over20Age = 1080,
        /// Question: "Are you over 21 years of age?"
        /// Category: "Demographics"
        Over21Age = 1081,
        /// Question: "Are you over 22 years of age?"
        /// Category: "Demographics"
        Over22Age = 1082,
        /// Question: "Are you over 23 years of age?"
        /// Category: "Demographics"
        Over23Age = 1083,
        /// Question: "Are you over 24 years of age?"
        /// Category: "Demographics"
        Over24Age = 1084,
        /// Question: "Are you over 25 years of age?"
        /// Category: "Demographics"
        Over25Age = 1085,
        /// Question: "Are you over 26 years of age?"
        /// Category: "Demographics"
        Over26Age = 1086,
        /// Question: "Are you over 27 years of age?"
        /// Category: "Demographics"
        Over27Age = 1087,
        /// Question: "Are you over 28 years of age?"
        /// Category: "Demographics"
        Over28Age = 1088,
        /// Question: "Are you over 29 years of age?"
        /// Category: "Demographics"
        Over29Age = 1089,
        /// Question: "Are you over 30 years of age?"
        /// Category: "Demographics"
        Over30Age = 1090,
        /// Question: "Are you over 31 years of age?"
        /// Category: "Demographics"
        Over31Age = 1091,
        /// Question: "Are you over 32 years of age?"
        /// Category: "Demographics"
        Over32Age = 1092,
        /// Question: "Are you over 33 years of age?"
        /// Category: "Demographics"
        Over33Age = 1093,
        /// Question: "Are you over 34 years of age?"
        /// Category: "Demographics"
        Over34Age = 1094,
        /// Question: "Are you over 35 years of age?"
        /// Category: "Demographics"
        Over35Age = 1095,
        /// Question: "Are you over 36 years of age?"
        /// Category: "Demographics"
        Over36Age = 1096,
        /// Question: "Are you over 37 years of age?"
        /// Category: "Demographics"
        Over37Age = 1097,
        /// Question: "Are you over 38 years of age?"
        /// Category: "Demographics"
        Over38Age = 1098,
        /// Question: "Are you over 39 years of age?"
        /// Category: "Demographics"
        Over39Age = 1099,
        /// Question: "Are you over 40 years of age?"
        /// Category: "Demographics"
        Over40Age = 1100,
        /// Question: "Are you over 41 years of age?"
        /// Category: "Demographics"
        Over41Age = 1101,
        /// Question: "Are you over 42 years of age?"
        /// Category: "Demographics"
        Over42Age = 1102,
        /// Question: "Are you over 43 years of age?"
        /// Category: "Demographics"
        Over43Age = 1103,
        /// Question: "Are you over 44 years of age?"
        /// Category: "Demographics"
        Over44Age = 1104,
        /// Question: "Are you over 45 years of age?"
        /// Category: "Demographics"
        Over45Age = 1105,
        /// Question: "Are you over 46 years of age?"
        /// Category: "Demographics"
        Over46Age = 1106,
        /// Question: "Are you over 47 years of age?"
        /// Category: "Demographics"
        Over47Age = 1107,
        /// Question: "Are you over 48 years of age?"
        /// Category: "Demographics"
        Over48Age = 1108,
        /// Question: "Are you over 49 years of age?"
        /// Category: "Demographics"
        Over49Age = 1109,
        /// Question: "Are you over 50 years of age?"
        /// Category: "Demographics"
        Over50Age = 1110,
        /// Question: "Are you over 51 years of age?"
        /// Category: "Demographics"
        Over51Age = 1111,
        /// Question: "Are you over 52 years of age?"
        /// Category: "Demographics"
        Over52Age = 1112,
        /// Question: "Are you over 53 years of age?"
        /// Category: "Demographics"
        Over53Age = 1113,
        /// Question: "Are you over 54 years of age?"
        /// Category: "Demographics"
        Over54Age = 1114,
        /// Question: "Are you over 55 years of age?"
        /// Category: "Demographics"
        Over55Age = 1115,
        /// Question: "Are you over 56 years of age?"
        /// Category: "Demographics"
        Over56Age = 1116,
        /// Question: "Are you over 57 years of age?"
        /// Category: "Demographics"
        Over57Age = 1117,
        /// Question: "Are you over 58 years of age?"
        /// Category: "Demographics"
        Over58Age = 1118,
        /// Question: "Are you over 59 years of age?"
        /// Category: "Demographics"
        Over59Age = 1119,
        /// Question: "Are you over 60 years of age?"
        /// Category: "Demographics"
        Over60Age = 1120,
        /// Question: "Are you over 61 years of age?"
        /// Category: "Demographics"
        Over61Age = 1121,
        /// Question: "Are you over 62 years of age?"
        /// Category: "Demographics"
        Over62Age = 1122,
        /// Question: "Are you over 63 years of age?"
        /// Category: "Demographics"
        Over63Age = 1123,
        /// Question: "Are you over 64 years of age?"
        /// Category: "Demographics"
        Over64Age = 1124,
        /// Question: "Are you over 65 years of age?"
        /// Category: "Demographics"
        Over65Age = 1125,
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
    impl LeadFormFieldUserInputType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                LeadFormFieldUserInputType::Unspecified => "UNSPECIFIED",
                LeadFormFieldUserInputType::Unknown => "UNKNOWN",
                LeadFormFieldUserInputType::FullName => "FULL_NAME",
                LeadFormFieldUserInputType::Email => "EMAIL",
                LeadFormFieldUserInputType::PhoneNumber => "PHONE_NUMBER",
                LeadFormFieldUserInputType::PostalCode => "POSTAL_CODE",
                LeadFormFieldUserInputType::StreetAddress => "STREET_ADDRESS",
                LeadFormFieldUserInputType::City => "CITY",
                LeadFormFieldUserInputType::Region => "REGION",
                LeadFormFieldUserInputType::Country => "COUNTRY",
                LeadFormFieldUserInputType::WorkEmail => "WORK_EMAIL",
                LeadFormFieldUserInputType::CompanyName => "COMPANY_NAME",
                LeadFormFieldUserInputType::WorkPhone => "WORK_PHONE",
                LeadFormFieldUserInputType::JobTitle => "JOB_TITLE",
                LeadFormFieldUserInputType::GovernmentIssuedIdCpfBr => {
                    "GOVERNMENT_ISSUED_ID_CPF_BR"
                }
                LeadFormFieldUserInputType::GovernmentIssuedIdDniAr => {
                    "GOVERNMENT_ISSUED_ID_DNI_AR"
                }
                LeadFormFieldUserInputType::GovernmentIssuedIdDniPe => {
                    "GOVERNMENT_ISSUED_ID_DNI_PE"
                }
                LeadFormFieldUserInputType::GovernmentIssuedIdRutCl => {
                    "GOVERNMENT_ISSUED_ID_RUT_CL"
                }
                LeadFormFieldUserInputType::GovernmentIssuedIdCcCo => {
                    "GOVERNMENT_ISSUED_ID_CC_CO"
                }
                LeadFormFieldUserInputType::GovernmentIssuedIdCiEc => {
                    "GOVERNMENT_ISSUED_ID_CI_EC"
                }
                LeadFormFieldUserInputType::GovernmentIssuedIdRfcMx => {
                    "GOVERNMENT_ISSUED_ID_RFC_MX"
                }
                LeadFormFieldUserInputType::FirstName => "FIRST_NAME",
                LeadFormFieldUserInputType::LastName => "LAST_NAME",
                LeadFormFieldUserInputType::VehicleModel => "VEHICLE_MODEL",
                LeadFormFieldUserInputType::VehicleType => "VEHICLE_TYPE",
                LeadFormFieldUserInputType::PreferredDealership => "PREFERRED_DEALERSHIP",
                LeadFormFieldUserInputType::VehiclePurchaseTimeline => {
                    "VEHICLE_PURCHASE_TIMELINE"
                }
                LeadFormFieldUserInputType::VehicleOwnership => "VEHICLE_OWNERSHIP",
                LeadFormFieldUserInputType::VehiclePaymentType => "VEHICLE_PAYMENT_TYPE",
                LeadFormFieldUserInputType::VehicleCondition => "VEHICLE_CONDITION",
                LeadFormFieldUserInputType::CompanySize => "COMPANY_SIZE",
                LeadFormFieldUserInputType::AnnualSales => "ANNUAL_SALES",
                LeadFormFieldUserInputType::YearsInBusiness => "YEARS_IN_BUSINESS",
                LeadFormFieldUserInputType::JobDepartment => "JOB_DEPARTMENT",
                LeadFormFieldUserInputType::JobRole => "JOB_ROLE",
                LeadFormFieldUserInputType::Over18Age => "OVER_18_AGE",
                LeadFormFieldUserInputType::Over19Age => "OVER_19_AGE",
                LeadFormFieldUserInputType::Over20Age => "OVER_20_AGE",
                LeadFormFieldUserInputType::Over21Age => "OVER_21_AGE",
                LeadFormFieldUserInputType::Over22Age => "OVER_22_AGE",
                LeadFormFieldUserInputType::Over23Age => "OVER_23_AGE",
                LeadFormFieldUserInputType::Over24Age => "OVER_24_AGE",
                LeadFormFieldUserInputType::Over25Age => "OVER_25_AGE",
                LeadFormFieldUserInputType::Over26Age => "OVER_26_AGE",
                LeadFormFieldUserInputType::Over27Age => "OVER_27_AGE",
                LeadFormFieldUserInputType::Over28Age => "OVER_28_AGE",
                LeadFormFieldUserInputType::Over29Age => "OVER_29_AGE",
                LeadFormFieldUserInputType::Over30Age => "OVER_30_AGE",
                LeadFormFieldUserInputType::Over31Age => "OVER_31_AGE",
                LeadFormFieldUserInputType::Over32Age => "OVER_32_AGE",
                LeadFormFieldUserInputType::Over33Age => "OVER_33_AGE",
                LeadFormFieldUserInputType::Over34Age => "OVER_34_AGE",
                LeadFormFieldUserInputType::Over35Age => "OVER_35_AGE",
                LeadFormFieldUserInputType::Over36Age => "OVER_36_AGE",
                LeadFormFieldUserInputType::Over37Age => "OVER_37_AGE",
                LeadFormFieldUserInputType::Over38Age => "OVER_38_AGE",
                LeadFormFieldUserInputType::Over39Age => "OVER_39_AGE",
                LeadFormFieldUserInputType::Over40Age => "OVER_40_AGE",
                LeadFormFieldUserInputType::Over41Age => "OVER_41_AGE",
                LeadFormFieldUserInputType::Over42Age => "OVER_42_AGE",
                LeadFormFieldUserInputType::Over43Age => "OVER_43_AGE",
                LeadFormFieldUserInputType::Over44Age => "OVER_44_AGE",
                LeadFormFieldUserInputType::Over45Age => "OVER_45_AGE",
                LeadFormFieldUserInputType::Over46Age => "OVER_46_AGE",
                LeadFormFieldUserInputType::Over47Age => "OVER_47_AGE",
                LeadFormFieldUserInputType::Over48Age => "OVER_48_AGE",
                LeadFormFieldUserInputType::Over49Age => "OVER_49_AGE",
                LeadFormFieldUserInputType::Over50Age => "OVER_50_AGE",
                LeadFormFieldUserInputType::Over51Age => "OVER_51_AGE",
                LeadFormFieldUserInputType::Over52Age => "OVER_52_AGE",
                LeadFormFieldUserInputType::Over53Age => "OVER_53_AGE",
                LeadFormFieldUserInputType::Over54Age => "OVER_54_AGE",
                LeadFormFieldUserInputType::Over55Age => "OVER_55_AGE",
                LeadFormFieldUserInputType::Over56Age => "OVER_56_AGE",
                LeadFormFieldUserInputType::Over57Age => "OVER_57_AGE",
                LeadFormFieldUserInputType::Over58Age => "OVER_58_AGE",
                LeadFormFieldUserInputType::Over59Age => "OVER_59_AGE",
                LeadFormFieldUserInputType::Over60Age => "OVER_60_AGE",
                LeadFormFieldUserInputType::Over61Age => "OVER_61_AGE",
                LeadFormFieldUserInputType::Over62Age => "OVER_62_AGE",
                LeadFormFieldUserInputType::Over63Age => "OVER_63_AGE",
                LeadFormFieldUserInputType::Over64Age => "OVER_64_AGE",
                LeadFormFieldUserInputType::Over65Age => "OVER_65_AGE",
                LeadFormFieldUserInputType::EducationProgram => "EDUCATION_PROGRAM",
                LeadFormFieldUserInputType::EducationCourse => "EDUCATION_COURSE",
                LeadFormFieldUserInputType::Product => "PRODUCT",
                LeadFormFieldUserInputType::Service => "SERVICE",
                LeadFormFieldUserInputType::Offer => "OFFER",
                LeadFormFieldUserInputType::Category => "CATEGORY",
                LeadFormFieldUserInputType::PreferredContactMethod => {
                    "PREFERRED_CONTACT_METHOD"
                }
                LeadFormFieldUserInputType::PreferredLocation => "PREFERRED_LOCATION",
                LeadFormFieldUserInputType::PreferredContactTime => {
                    "PREFERRED_CONTACT_TIME"
                }
                LeadFormFieldUserInputType::PurchaseTimeline => "PURCHASE_TIMELINE",
                LeadFormFieldUserInputType::YearsOfExperience => "YEARS_OF_EXPERIENCE",
                LeadFormFieldUserInputType::JobIndustry => "JOB_INDUSTRY",
                LeadFormFieldUserInputType::LevelOfEducation => "LEVEL_OF_EDUCATION",
                LeadFormFieldUserInputType::PropertyType => "PROPERTY_TYPE",
                LeadFormFieldUserInputType::RealtorHelpGoal => "REALTOR_HELP_GOAL",
                LeadFormFieldUserInputType::PropertyCommunity => "PROPERTY_COMMUNITY",
                LeadFormFieldUserInputType::PriceRange => "PRICE_RANGE",
                LeadFormFieldUserInputType::NumberOfBedrooms => "NUMBER_OF_BEDROOMS",
                LeadFormFieldUserInputType::FurnishedProperty => "FURNISHED_PROPERTY",
                LeadFormFieldUserInputType::PetsAllowedProperty => {
                    "PETS_ALLOWED_PROPERTY"
                }
                LeadFormFieldUserInputType::NextPlannedPurchase => {
                    "NEXT_PLANNED_PURCHASE"
                }
                LeadFormFieldUserInputType::EventSignupInterest => {
                    "EVENT_SIGNUP_INTEREST"
                }
                LeadFormFieldUserInputType::PreferredShoppingPlaces => {
                    "PREFERRED_SHOPPING_PLACES"
                }
                LeadFormFieldUserInputType::FavoriteBrand => "FAVORITE_BRAND",
                LeadFormFieldUserInputType::TransportationCommercialLicenseType => {
                    "TRANSPORTATION_COMMERCIAL_LICENSE_TYPE"
                }
                LeadFormFieldUserInputType::EventBookingInterest => {
                    "EVENT_BOOKING_INTEREST"
                }
                LeadFormFieldUserInputType::DestinationCountry => "DESTINATION_COUNTRY",
                LeadFormFieldUserInputType::DestinationCity => "DESTINATION_CITY",
                LeadFormFieldUserInputType::DepartureCountry => "DEPARTURE_COUNTRY",
                LeadFormFieldUserInputType::DepartureCity => "DEPARTURE_CITY",
                LeadFormFieldUserInputType::DepartureDate => "DEPARTURE_DATE",
                LeadFormFieldUserInputType::ReturnDate => "RETURN_DATE",
                LeadFormFieldUserInputType::NumberOfTravelers => "NUMBER_OF_TRAVELERS",
                LeadFormFieldUserInputType::TravelBudget => "TRAVEL_BUDGET",
                LeadFormFieldUserInputType::TravelAccommodation => "TRAVEL_ACCOMMODATION",
            }
        }
    }
}
/// Describes the type of post-submit call-to-action phrases for a lead form.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LeadFormPostSubmitCallToActionTypeEnum {}
/// Nested message and enum types in `LeadFormPostSubmitCallToActionTypeEnum`.
pub mod lead_form_post_submit_call_to_action_type_enum {
    /// Enum describing the type of post-submit call-to-action phrases for a lead
    /// form.
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
    impl LeadFormPostSubmitCallToActionType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                LeadFormPostSubmitCallToActionType::Unspecified => "UNSPECIFIED",
                LeadFormPostSubmitCallToActionType::Unknown => "UNKNOWN",
                LeadFormPostSubmitCallToActionType::VisitSite => "VISIT_SITE",
                LeadFormPostSubmitCallToActionType::Download => "DOWNLOAD",
                LeadFormPostSubmitCallToActionType::LearnMore => "LEARN_MORE",
                LeadFormPostSubmitCallToActionType::ShopNow => "SHOP_NOW",
            }
        }
    }
}
/// Container for enum describing different types of mobile app vendors.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MobileAppVendorEnum {}
/// Nested message and enum types in `MobileAppVendorEnum`.
pub mod mobile_app_vendor_enum {
    /// The type of mobile app vendor
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
    impl MobileAppVendor {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                MobileAppVendor::Unspecified => "UNSPECIFIED",
                MobileAppVendor::Unknown => "UNKNOWN",
                MobileAppVendor::AppleAppStore => "APPLE_APP_STORE",
                MobileAppVendor::GoogleAppStore => "GOOGLE_APP_STORE",
            }
        }
    }
}
/// Container for enum describing a price extension price qualifier.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PriceExtensionPriceQualifierEnum {}
/// Nested message and enum types in `PriceExtensionPriceQualifierEnum`.
pub mod price_extension_price_qualifier_enum {
    /// Enums of price extension price qualifier.
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
    impl PriceExtensionPriceQualifier {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                PriceExtensionPriceQualifier::Unspecified => "UNSPECIFIED",
                PriceExtensionPriceQualifier::Unknown => "UNKNOWN",
                PriceExtensionPriceQualifier::From => "FROM",
                PriceExtensionPriceQualifier::UpTo => "UP_TO",
                PriceExtensionPriceQualifier::Average => "AVERAGE",
            }
        }
    }
}
/// Container for enum describing price extension price unit.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PriceExtensionPriceUnitEnum {}
/// Nested message and enum types in `PriceExtensionPriceUnitEnum`.
pub mod price_extension_price_unit_enum {
    /// Price extension price unit.
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
    impl PriceExtensionPriceUnit {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                PriceExtensionPriceUnit::Unspecified => "UNSPECIFIED",
                PriceExtensionPriceUnit::Unknown => "UNKNOWN",
                PriceExtensionPriceUnit::PerHour => "PER_HOUR",
                PriceExtensionPriceUnit::PerDay => "PER_DAY",
                PriceExtensionPriceUnit::PerWeek => "PER_WEEK",
                PriceExtensionPriceUnit::PerMonth => "PER_MONTH",
                PriceExtensionPriceUnit::PerYear => "PER_YEAR",
                PriceExtensionPriceUnit::PerNight => "PER_NIGHT",
            }
        }
    }
}
/// Container for enum describing types for a price extension.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PriceExtensionTypeEnum {}
/// Nested message and enum types in `PriceExtensionTypeEnum`.
pub mod price_extension_type_enum {
    /// Price extension type.
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
    impl PriceExtensionType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                PriceExtensionType::Unspecified => "UNSPECIFIED",
                PriceExtensionType::Unknown => "UNKNOWN",
                PriceExtensionType::Brands => "BRANDS",
                PriceExtensionType::Events => "EVENTS",
                PriceExtensionType::Locations => "LOCATIONS",
                PriceExtensionType::Neighborhoods => "NEIGHBORHOODS",
                PriceExtensionType::ProductCategories => "PRODUCT_CATEGORIES",
                PriceExtensionType::ProductTiers => "PRODUCT_TIERS",
                PriceExtensionType::Services => "SERVICES",
                PriceExtensionType::ServiceCategories => "SERVICE_CATEGORIES",
                PriceExtensionType::ServiceTiers => "SERVICE_TIERS",
            }
        }
    }
}
/// Container for enum describing possible a promotion extension
/// discount modifier.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PromotionExtensionDiscountModifierEnum {}
/// Nested message and enum types in `PromotionExtensionDiscountModifierEnum`.
pub mod promotion_extension_discount_modifier_enum {
    /// A promotion extension discount modifier.
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
    pub enum PromotionExtensionDiscountModifier {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// 'Up to'.
        UpTo = 2,
    }
    impl PromotionExtensionDiscountModifier {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                PromotionExtensionDiscountModifier::Unspecified => "UNSPECIFIED",
                PromotionExtensionDiscountModifier::Unknown => "UNKNOWN",
                PromotionExtensionDiscountModifier::UpTo => "UP_TO",
            }
        }
    }
}
/// Container for enum describing a promotion extension occasion.
/// For more information about the occasions  check:
/// <https://support.google.com/google-ads/answer/7367521>
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PromotionExtensionOccasionEnum {}
/// Nested message and enum types in `PromotionExtensionOccasionEnum`.
pub mod promotion_extension_occasion_enum {
    /// A promotion extension occasion.
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
    impl PromotionExtensionOccasion {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                PromotionExtensionOccasion::Unspecified => "UNSPECIFIED",
                PromotionExtensionOccasion::Unknown => "UNKNOWN",
                PromotionExtensionOccasion::NewYears => "NEW_YEARS",
                PromotionExtensionOccasion::ChineseNewYear => "CHINESE_NEW_YEAR",
                PromotionExtensionOccasion::ValentinesDay => "VALENTINES_DAY",
                PromotionExtensionOccasion::Easter => "EASTER",
                PromotionExtensionOccasion::MothersDay => "MOTHERS_DAY",
                PromotionExtensionOccasion::FathersDay => "FATHERS_DAY",
                PromotionExtensionOccasion::LaborDay => "LABOR_DAY",
                PromotionExtensionOccasion::BackToSchool => "BACK_TO_SCHOOL",
                PromotionExtensionOccasion::Halloween => "HALLOWEEN",
                PromotionExtensionOccasion::BlackFriday => "BLACK_FRIDAY",
                PromotionExtensionOccasion::CyberMonday => "CYBER_MONDAY",
                PromotionExtensionOccasion::Christmas => "CHRISTMAS",
                PromotionExtensionOccasion::BoxingDay => "BOXING_DAY",
                PromotionExtensionOccasion::IndependenceDay => "INDEPENDENCE_DAY",
                PromotionExtensionOccasion::NationalDay => "NATIONAL_DAY",
                PromotionExtensionOccasion::EndOfSeason => "END_OF_SEASON",
                PromotionExtensionOccasion::WinterSale => "WINTER_SALE",
                PromotionExtensionOccasion::SummerSale => "SUMMER_SALE",
                PromotionExtensionOccasion::FallSale => "FALL_SALE",
                PromotionExtensionOccasion::SpringSale => "SPRING_SALE",
                PromotionExtensionOccasion::Ramadan => "RAMADAN",
                PromotionExtensionOccasion::EidAlFitr => "EID_AL_FITR",
                PromotionExtensionOccasion::EidAlAdha => "EID_AL_ADHA",
                PromotionExtensionOccasion::SinglesDay => "SINGLES_DAY",
                PromotionExtensionOccasion::WomensDay => "WOMENS_DAY",
                PromotionExtensionOccasion::Holi => "HOLI",
                PromotionExtensionOccasion::ParentsDay => "PARENTS_DAY",
                PromotionExtensionOccasion::StNicholasDay => "ST_NICHOLAS_DAY",
                PromotionExtensionOccasion::Carnival => "CARNIVAL",
                PromotionExtensionOccasion::Epiphany => "EPIPHANY",
                PromotionExtensionOccasion::RoshHashanah => "ROSH_HASHANAH",
                PromotionExtensionOccasion::Passover => "PASSOVER",
                PromotionExtensionOccasion::Hanukkah => "HANUKKAH",
                PromotionExtensionOccasion::Diwali => "DIWALI",
                PromotionExtensionOccasion::Navratri => "NAVRATRI",
                PromotionExtensionOccasion::Songkran => "SONGKRAN",
                PromotionExtensionOccasion::YearEndGift => "YEAR_END_GIFT",
            }
        }
    }
}
/// Container for enum describing where on the first search results page the
/// automated bidding system should target impressions for the
/// TargetImpressionShare bidding strategy.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TargetImpressionShareLocationEnum {}
/// Nested message and enum types in `TargetImpressionShareLocationEnum`.
pub mod target_impression_share_location_enum {
    /// Enum describing possible goals.
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
    impl TargetImpressionShareLocation {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                TargetImpressionShareLocation::Unspecified => "UNSPECIFIED",
                TargetImpressionShareLocation::Unknown => "UNKNOWN",
                TargetImpressionShareLocation::AnywhereOnPage => "ANYWHERE_ON_PAGE",
                TargetImpressionShareLocation::TopOfPage => "TOP_OF_PAGE",
                TargetImpressionShareLocation::AbsoluteTopOfPage => {
                    "ABSOLUTE_TOP_OF_PAGE"
                }
            }
        }
    }
}
/// An immutable specialization of an Advertising Channel.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdvertisingChannelSubTypeEnum {}
/// Nested message and enum types in `AdvertisingChannelSubTypeEnum`.
pub mod advertising_channel_sub_type_enum {
    /// Enum describing the different channel subtypes.
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
        /// App Campaign that lets you easily promote your Android or iOS app
        /// across Google's top properties including Search, Play, YouTube, and the
        /// Google Display Network.
        AppCampaign = 12,
        /// App Campaign for engagement, focused on driving re-engagement with the
        /// app across several of Google's top properties including Search, YouTube,
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
        /// App Campaign for pre registration, specialized for advertising mobile
        /// app pre-registration, that targets multiple advertising channels across
        /// Google Play, YouTube and Display Network. See
        /// <https://support.google.com/google-ads/answer/9441344> to learn more.
        AppCampaignForPreRegistration = 18,
    }
    impl AdvertisingChannelSubType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                AdvertisingChannelSubType::Unspecified => "UNSPECIFIED",
                AdvertisingChannelSubType::Unknown => "UNKNOWN",
                AdvertisingChannelSubType::SearchMobileApp => "SEARCH_MOBILE_APP",
                AdvertisingChannelSubType::DisplayMobileApp => "DISPLAY_MOBILE_APP",
                AdvertisingChannelSubType::SearchExpress => "SEARCH_EXPRESS",
                AdvertisingChannelSubType::DisplayExpress => "DISPLAY_EXPRESS",
                AdvertisingChannelSubType::ShoppingSmartAds => "SHOPPING_SMART_ADS",
                AdvertisingChannelSubType::DisplayGmailAd => "DISPLAY_GMAIL_AD",
                AdvertisingChannelSubType::DisplaySmartCampaign => {
                    "DISPLAY_SMART_CAMPAIGN"
                }
                AdvertisingChannelSubType::VideoOutstream => "VIDEO_OUTSTREAM",
                AdvertisingChannelSubType::VideoAction => "VIDEO_ACTION",
                AdvertisingChannelSubType::VideoNonSkippable => "VIDEO_NON_SKIPPABLE",
                AdvertisingChannelSubType::AppCampaign => "APP_CAMPAIGN",
                AdvertisingChannelSubType::AppCampaignForEngagement => {
                    "APP_CAMPAIGN_FOR_ENGAGEMENT"
                }
                AdvertisingChannelSubType::LocalCampaign => "LOCAL_CAMPAIGN",
                AdvertisingChannelSubType::ShoppingComparisonListingAds => {
                    "SHOPPING_COMPARISON_LISTING_ADS"
                }
                AdvertisingChannelSubType::SmartCampaign => "SMART_CAMPAIGN",
                AdvertisingChannelSubType::VideoSequence => "VIDEO_SEQUENCE",
                AdvertisingChannelSubType::AppCampaignForPreRegistration => {
                    "APP_CAMPAIGN_FOR_PRE_REGISTRATION"
                }
            }
        }
    }
}
/// The channel type a campaign may target to serve on.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdvertisingChannelTypeEnum {}
/// Nested message and enum types in `AdvertisingChannelTypeEnum`.
pub mod advertising_channel_type_enum {
    /// Enum describing the various advertising channel types.
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
        /// Performance Max campaigns.
        PerformanceMax = 10,
        /// Local services campaigns.
        LocalServices = 11,
        /// Discovery campaigns.
        Discovery = 12,
    }
    impl AdvertisingChannelType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                AdvertisingChannelType::Unspecified => "UNSPECIFIED",
                AdvertisingChannelType::Unknown => "UNKNOWN",
                AdvertisingChannelType::Search => "SEARCH",
                AdvertisingChannelType::Display => "DISPLAY",
                AdvertisingChannelType::Shopping => "SHOPPING",
                AdvertisingChannelType::Hotel => "HOTEL",
                AdvertisingChannelType::Video => "VIDEO",
                AdvertisingChannelType::MultiChannel => "MULTI_CHANNEL",
                AdvertisingChannelType::Local => "LOCAL",
                AdvertisingChannelType::Smart => "SMART",
                AdvertisingChannelType::PerformanceMax => "PERFORMANCE_MAX",
                AdvertisingChannelType::LocalServices => "LOCAL_SERVICES",
                AdvertisingChannelType::Discovery => "DISCOVERY",
            }
        }
    }
}
/// Describes channel availability mode for a criterion availability - whether
/// the availability is meant to include all advertising channels, or a
/// particular channel with all its channel subtypes, or a channel with a certain
/// subset of channel subtypes.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CriterionCategoryChannelAvailabilityModeEnum {}
/// Nested message and enum types in `CriterionCategoryChannelAvailabilityModeEnum`.
pub mod criterion_category_channel_availability_mode_enum {
    /// Enum containing the possible CriterionCategoryChannelAvailabilityMode.
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
    impl CriterionCategoryChannelAvailabilityMode {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                CriterionCategoryChannelAvailabilityMode::Unspecified => "UNSPECIFIED",
                CriterionCategoryChannelAvailabilityMode::Unknown => "UNKNOWN",
                CriterionCategoryChannelAvailabilityMode::AllChannels => "ALL_CHANNELS",
                CriterionCategoryChannelAvailabilityMode::ChannelTypeAndAllSubtypes => {
                    "CHANNEL_TYPE_AND_ALL_SUBTYPES"
                }
                CriterionCategoryChannelAvailabilityMode::ChannelTypeAndSubsetSubtypes => {
                    "CHANNEL_TYPE_AND_SUBSET_SUBTYPES"
                }
            }
        }
    }
}
/// Describes locale availability mode for a criterion availability - whether
/// it's available globally, or a particular country with all languages, or a
/// particular language with all countries, or a country-language pair.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CriterionCategoryLocaleAvailabilityModeEnum {}
/// Nested message and enum types in `CriterionCategoryLocaleAvailabilityModeEnum`.
pub mod criterion_category_locale_availability_mode_enum {
    /// Enum containing the possible CriterionCategoryLocaleAvailabilityMode.
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
    impl CriterionCategoryLocaleAvailabilityMode {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                CriterionCategoryLocaleAvailabilityMode::Unspecified => "UNSPECIFIED",
                CriterionCategoryLocaleAvailabilityMode::Unknown => "UNKNOWN",
                CriterionCategoryLocaleAvailabilityMode::AllLocales => "ALL_LOCALES",
                CriterionCategoryLocaleAvailabilityMode::CountryAndAllLanguages => {
                    "COUNTRY_AND_ALL_LANGUAGES"
                }
                CriterionCategoryLocaleAvailabilityMode::LanguageAndAllCountries => {
                    "LANGUAGE_AND_ALL_COUNTRIES"
                }
                CriterionCategoryLocaleAvailabilityMode::CountryAndLanguage => {
                    "COUNTRY_AND_LANGUAGE"
                }
            }
        }
    }
}
/// Container for enum describing possible types of a customizer attribute.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomizerAttributeTypeEnum {}
/// Nested message and enum types in `CustomizerAttributeTypeEnum`.
pub mod customizer_attribute_type_enum {
    /// The possible types of a customizer attribute.
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
    pub enum CustomizerAttributeType {
        /// The status has not been specified.
        Unspecified = 0,
        /// The received value is not known in this version.
        Unknown = 1,
        /// Text customizer.
        Text = 2,
        /// Number customizer.
        Number = 3,
        /// Price customizer consisting of a number and a currency.
        Price = 4,
        /// Percentage customizer consisting of a number and a '%'.
        Percent = 5,
    }
    impl CustomizerAttributeType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                CustomizerAttributeType::Unspecified => "UNSPECIFIED",
                CustomizerAttributeType::Unknown => "UNKNOWN",
                CustomizerAttributeType::Text => "TEXT",
                CustomizerAttributeType::Number => "NUMBER",
                CustomizerAttributeType::Price => "PRICE",
                CustomizerAttributeType::Percent => "PERCENT",
            }
        }
    }
}
/// Container for enumeration of months of the year, for example, "January".
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MonthOfYearEnum {}
/// Nested message and enum types in `MonthOfYearEnum`.
pub mod month_of_year_enum {
    /// Enumerates months of the year, for example, "January".
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
    impl MonthOfYear {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                MonthOfYear::Unspecified => "UNSPECIFIED",
                MonthOfYear::Unknown => "UNKNOWN",
                MonthOfYear::January => "JANUARY",
                MonthOfYear::February => "FEBRUARY",
                MonthOfYear::March => "MARCH",
                MonthOfYear::April => "APRIL",
                MonthOfYear::May => "MAY",
                MonthOfYear::June => "JUNE",
                MonthOfYear::July => "JULY",
                MonthOfYear::August => "AUGUST",
                MonthOfYear::September => "SEPTEMBER",
                MonthOfYear::October => "OCTOBER",
                MonthOfYear::November => "NOVEMBER",
                MonthOfYear::December => "DECEMBER",
            }
        }
    }
}
/// Container for enum describing app store type in an app extension.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AppStoreEnum {}
/// Nested message and enum types in `AppStoreEnum`.
pub mod app_store_enum {
    /// App store type in an app extension.
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
    impl AppStore {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                AppStore::Unspecified => "UNSPECIFIED",
                AppStore::Unknown => "UNKNOWN",
                AppStore::AppleItunes => "APPLE_ITUNES",
                AppStore::GooglePlay => "GOOGLE_PLAY",
            }
        }
    }
}
/// The type of string matching to be used for a dynamic FeedItemSet filter.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeedItemSetStringFilterTypeEnum {}
/// Nested message and enum types in `FeedItemSetStringFilterTypeEnum`.
pub mod feed_item_set_string_filter_type_enum {
    /// describe the possible types for a FeedItemSetStringFilter.
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
    pub enum FeedItemSetStringFilterType {
        /// Not specified.
        Unspecified = 0,
        /// The received error code is not known in this version.
        Unknown = 1,
        /// The dynamic set filter will use exact string matching.
        Exact = 2,
    }
    impl FeedItemSetStringFilterType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                FeedItemSetStringFilterType::Unspecified => "UNSPECIFIED",
                FeedItemSetStringFilterType::Unknown => "UNKNOWN",
                FeedItemSetStringFilterType::Exact => "EXACT",
            }
        }
    }
}
/// The possible OS types for a deeplink AppUrl.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AppUrlOperatingSystemTypeEnum {}
/// Nested message and enum types in `AppUrlOperatingSystemTypeEnum`.
pub mod app_url_operating_system_type_enum {
    /// Operating System
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
    impl AppUrlOperatingSystemType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                AppUrlOperatingSystemType::Unspecified => "UNSPECIFIED",
                AppUrlOperatingSystemType::Unknown => "UNKNOWN",
                AppUrlOperatingSystemType::Ios => "IOS",
                AppUrlOperatingSystemType::Android => "ANDROID",
            }
        }
    }
}
/// Container for enum describing the type of event that the cap applies to.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FrequencyCapEventTypeEnum {}
/// Nested message and enum types in `FrequencyCapEventTypeEnum`.
pub mod frequency_cap_event_type_enum {
    /// The type of event that the cap applies to (for example, impression).
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
    impl FrequencyCapEventType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                FrequencyCapEventType::Unspecified => "UNSPECIFIED",
                FrequencyCapEventType::Unknown => "UNKNOWN",
                FrequencyCapEventType::Impression => "IMPRESSION",
                FrequencyCapEventType::VideoView => "VIDEO_VIEW",
            }
        }
    }
}
/// Container for enum describing the level on which the cap is to be applied.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FrequencyCapLevelEnum {}
/// Nested message and enum types in `FrequencyCapLevelEnum`.
pub mod frequency_cap_level_enum {
    /// The level on which the cap is to be applied (e.g ad group ad, ad group).
    /// Cap is applied to all the resources of this level.
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
    impl FrequencyCapLevel {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                FrequencyCapLevel::Unspecified => "UNSPECIFIED",
                FrequencyCapLevel::Unknown => "UNKNOWN",
                FrequencyCapLevel::AdGroupAd => "AD_GROUP_AD",
                FrequencyCapLevel::AdGroup => "AD_GROUP",
                FrequencyCapLevel::Campaign => "CAMPAIGN",
            }
        }
    }
}
/// Container for enum describing the unit of time the cap is defined at.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FrequencyCapTimeUnitEnum {}
/// Nested message and enum types in `FrequencyCapTimeUnitEnum`.
pub mod frequency_cap_time_unit_enum {
    /// Unit of time the cap is defined at (for example, day, week).
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
    impl FrequencyCapTimeUnit {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                FrequencyCapTimeUnit::Unspecified => "UNSPECIFIED",
                FrequencyCapTimeUnit::Unknown => "UNKNOWN",
                FrequencyCapTimeUnit::Day => "DAY",
                FrequencyCapTimeUnit::Week => "WEEK",
                FrequencyCapTimeUnit::Month => "MONTH",
            }
        }
    }
}
/// The enumeration of keyword plan aggregate metric types.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeywordPlanAggregateMetricTypeEnum {}
/// Nested message and enum types in `KeywordPlanAggregateMetricTypeEnum`.
pub mod keyword_plan_aggregate_metric_type_enum {
    /// Aggregate fields.
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
    pub enum KeywordPlanAggregateMetricType {
        /// Not specified.
        Unspecified = 0,
        /// The value is unknown in this version.
        Unknown = 1,
        /// The device breakdown of aggregate search volume.
        Device = 2,
    }
    impl KeywordPlanAggregateMetricType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                KeywordPlanAggregateMetricType::Unspecified => "UNSPECIFIED",
                KeywordPlanAggregateMetricType::Unknown => "UNKNOWN",
                KeywordPlanAggregateMetricType::Device => "DEVICE",
            }
        }
    }
}
/// Container for enumeration of keyword competition levels. The competition
/// level indicates how competitive ad placement is for a keyword and
/// is determined by the number of advertisers bidding on that keyword relative
/// to all keywords across Google. The competition level can depend on the
/// location and Search Network targeting options you've selected.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeywordPlanCompetitionLevelEnum {}
/// Nested message and enum types in `KeywordPlanCompetitionLevelEnum`.
pub mod keyword_plan_competition_level_enum {
    /// Competition level of a keyword.
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
    impl KeywordPlanCompetitionLevel {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                KeywordPlanCompetitionLevel::Unspecified => "UNSPECIFIED",
                KeywordPlanCompetitionLevel::Unknown => "UNKNOWN",
                KeywordPlanCompetitionLevel::Low => "LOW",
                KeywordPlanCompetitionLevel::Medium => "MEDIUM",
                KeywordPlanCompetitionLevel::High => "HIGH",
            }
        }
    }
}
/// Container for enumeration of keyword plan concept group types.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeywordPlanConceptGroupTypeEnum {}
/// Nested message and enum types in `KeywordPlanConceptGroupTypeEnum`.
pub mod keyword_plan_concept_group_type_enum {
    /// Enumerates keyword plan concept group types.
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
    impl KeywordPlanConceptGroupType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                KeywordPlanConceptGroupType::Unspecified => "UNSPECIFIED",
                KeywordPlanConceptGroupType::Unknown => "UNKNOWN",
                KeywordPlanConceptGroupType::Brand => "BRAND",
                KeywordPlanConceptGroupType::OtherBrands => "OTHER_BRANDS",
                KeywordPlanConceptGroupType::NonBrand => "NON_BRAND",
            }
        }
    }
}
/// Container for context types for an operand in a matching function.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MatchingFunctionContextTypeEnum {}
/// Nested message and enum types in `MatchingFunctionContextTypeEnum`.
pub mod matching_function_context_type_enum {
    /// Possible context types for an operand in a matching function.
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
    impl MatchingFunctionContextType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                MatchingFunctionContextType::Unspecified => "UNSPECIFIED",
                MatchingFunctionContextType::Unknown => "UNKNOWN",
                MatchingFunctionContextType::FeedItemId => "FEED_ITEM_ID",
                MatchingFunctionContextType::DeviceName => "DEVICE_NAME",
                MatchingFunctionContextType::FeedItemSetId => "FEED_ITEM_SET_ID",
            }
        }
    }
}
/// Container for enum describing matching function operator.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MatchingFunctionOperatorEnum {}
/// Nested message and enum types in `MatchingFunctionOperatorEnum`.
pub mod matching_function_operator_enum {
    /// Possible operators in a matching function.
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
    impl MatchingFunctionOperator {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                MatchingFunctionOperator::Unspecified => "UNSPECIFIED",
                MatchingFunctionOperator::Unknown => "UNKNOWN",
                MatchingFunctionOperator::In => "IN",
                MatchingFunctionOperator::Identity => "IDENTITY",
                MatchingFunctionOperator::Equals => "EQUALS",
                MatchingFunctionOperator::And => "AND",
                MatchingFunctionOperator::ContainsAny => "CONTAINS_ANY",
            }
        }
    }
}
/// Container for enum describing the type of experiment metric.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExperimentMetricEnum {}
/// Nested message and enum types in `ExperimentMetricEnum`.
pub mod experiment_metric_enum {
    /// The type of experiment metric.
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
    pub enum ExperimentMetric {
        /// Not specified.
        Unspecified = 0,
        /// The value is unknown in this version.
        Unknown = 1,
        /// The goal of the experiment is clicks.
        Clicks = 2,
        /// The goal of the experiment is impressions.
        Impressions = 3,
        /// The goal of the experiment is cost.
        Cost = 4,
        /// The goal of the experiment is conversion rate.
        ConversionsPerInteractionRate = 5,
        /// The goal of the experiment is cost per conversion.
        CostPerConversion = 6,
        /// The goal of the experiment is conversion value per cost.
        ConversionsValuePerCost = 7,
        /// The goal of the experiment is avg cpc.
        AverageCpc = 8,
        /// The goal of the experiment is ctr.
        Ctr = 9,
        /// The goal of the experiment is incremental conversions.
        IncrementalConversions = 10,
        /// The goal of the experiment is completed video views.
        CompletedVideoViews = 11,
        /// The goal of the experiment is custom algorithms.
        CustomAlgorithms = 12,
        /// The goal of the experiment is conversions.
        Conversions = 13,
        /// The goal of the experiment is conversion value.
        ConversionValue = 14,
    }
    impl ExperimentMetric {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ExperimentMetric::Unspecified => "UNSPECIFIED",
                ExperimentMetric::Unknown => "UNKNOWN",
                ExperimentMetric::Clicks => "CLICKS",
                ExperimentMetric::Impressions => "IMPRESSIONS",
                ExperimentMetric::Cost => "COST",
                ExperimentMetric::ConversionsPerInteractionRate => {
                    "CONVERSIONS_PER_INTERACTION_RATE"
                }
                ExperimentMetric::CostPerConversion => "COST_PER_CONVERSION",
                ExperimentMetric::ConversionsValuePerCost => "CONVERSIONS_VALUE_PER_COST",
                ExperimentMetric::AverageCpc => "AVERAGE_CPC",
                ExperimentMetric::Ctr => "CTR",
                ExperimentMetric::IncrementalConversions => "INCREMENTAL_CONVERSIONS",
                ExperimentMetric::CompletedVideoViews => "COMPLETED_VIDEO_VIEWS",
                ExperimentMetric::CustomAlgorithms => "CUSTOM_ALGORITHMS",
                ExperimentMetric::Conversions => "CONVERSIONS",
                ExperimentMetric::ConversionValue => "CONVERSION_VALUE",
            }
        }
    }
}
/// Container for enum describing the type of experiment metric direction.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExperimentMetricDirectionEnum {}
/// Nested message and enum types in `ExperimentMetricDirectionEnum`.
pub mod experiment_metric_direction_enum {
    /// The type of experiment metric direction.
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
    pub enum ExperimentMetricDirection {
        /// Not specified.
        Unspecified = 0,
        /// The value is unknown in this version.
        Unknown = 1,
        /// The goal of the experiment is to not change the metric.
        NoChange = 2,
        /// The goal of the experiment is to increate the metric.
        Increase = 3,
        /// The goal of the experiment is to decrease the metric.
        Decrease = 4,
        /// The goal of the experiment is to either not change or increase the
        /// metric.
        NoChangeOrIncrease = 5,
        /// The goal of the experiment is to either not change or decrease the
        /// metric.
        NoChangeOrDecrease = 6,
    }
    impl ExperimentMetricDirection {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ExperimentMetricDirection::Unspecified => "UNSPECIFIED",
                ExperimentMetricDirection::Unknown => "UNKNOWN",
                ExperimentMetricDirection::NoChange => "NO_CHANGE",
                ExperimentMetricDirection::Increase => "INCREASE",
                ExperimentMetricDirection::Decrease => "DECREASE",
                ExperimentMetricDirection::NoChangeOrIncrease => "NO_CHANGE_OR_INCREASE",
                ExperimentMetricDirection::NoChangeOrDecrease => "NO_CHANGE_OR_DECREASE",
            }
        }
    }
}
/// Container for enum describing types of payable and free interactions.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InteractionEventTypeEnum {}
/// Nested message and enum types in `InteractionEventTypeEnum`.
pub mod interaction_event_type_enum {
    /// Enum describing possible types of payable and free interactions.
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
        /// that the free interactions (for example, the ad conversions)
        /// should be 'promoted' and reported as part of the core metrics.
        /// These are simply other (ad) conversions.
        None = 5,
    }
    impl InteractionEventType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                InteractionEventType::Unspecified => "UNSPECIFIED",
                InteractionEventType::Unknown => "UNKNOWN",
                InteractionEventType::Click => "CLICK",
                InteractionEventType::Engagement => "ENGAGEMENT",
                InteractionEventType::VideoView => "VIDEO_VIEW",
                InteractionEventType::None => "NONE",
            }
        }
    }
}
/// The relative performance compared to other advertisers.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QualityScoreBucketEnum {}
/// Nested message and enum types in `QualityScoreBucketEnum`.
pub mod quality_score_bucket_enum {
    /// Enum listing the possible quality score buckets.
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
    impl QualityScoreBucket {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                QualityScoreBucket::Unspecified => "UNSPECIFIED",
                QualityScoreBucket::Unknown => "UNKNOWN",
                QualityScoreBucket::BelowAverage => "BELOW_AVERAGE",
                QualityScoreBucket::Average => "AVERAGE",
                QualityScoreBucket::AboveAverage => "ABOVE_AVERAGE",
            }
        }
    }
}
/// Container for enum describing the source of the user identifier for offline
/// Store Sales, click conversion, and conversion adjustment uploads.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserIdentifierSourceEnum {}
/// Nested message and enum types in `UserIdentifierSourceEnum`.
pub mod user_identifier_source_enum {
    /// The type of user identifier source for offline Store Sales, click
    /// conversion, and conversion adjustment uploads.
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
    impl UserIdentifierSource {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                UserIdentifierSource::Unspecified => "UNSPECIFIED",
                UserIdentifierSource::Unknown => "UNKNOWN",
                UserIdentifierSource::FirstParty => "FIRST_PARTY",
                UserIdentifierSource::ThirdParty => "THIRD_PARTY",
            }
        }
    }
}
/// Container for enumeration of Google Ads destination types.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdDestinationTypeEnum {}
/// Nested message and enum types in `AdDestinationTypeEnum`.
pub mod ad_destination_type_enum {
    /// Enumerates Google Ads destination types
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
    impl AdDestinationType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                AdDestinationType::Unspecified => "UNSPECIFIED",
                AdDestinationType::Unknown => "UNKNOWN",
                AdDestinationType::NotApplicable => "NOT_APPLICABLE",
                AdDestinationType::Website => "WEBSITE",
                AdDestinationType::AppDeepLink => "APP_DEEP_LINK",
                AdDestinationType::AppStore => "APP_STORE",
                AdDestinationType::PhoneCall => "PHONE_CALL",
                AdDestinationType::MapDirections => "MAP_DIRECTIONS",
                AdDestinationType::LocationListing => "LOCATION_LISTING",
                AdDestinationType::Message => "MESSAGE",
                AdDestinationType::LeadForm => "LEAD_FORM",
                AdDestinationType::Youtube => "YOUTUBE",
                AdDestinationType::UnmodeledForConversions => "UNMODELED_FOR_CONVERSIONS",
            }
        }
    }
}
/// Container for enumeration of Google Ads network types.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdNetworkTypeEnum {}
/// Nested message and enum types in `AdNetworkTypeEnum`.
pub mod ad_network_type_enum {
    /// Enumerates Google Ads network types.
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
    impl AdNetworkType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                AdNetworkType::Unspecified => "UNSPECIFIED",
                AdNetworkType::Unknown => "UNKNOWN",
                AdNetworkType::Search => "SEARCH",
                AdNetworkType::SearchPartners => "SEARCH_PARTNERS",
                AdNetworkType::Content => "CONTENT",
                AdNetworkType::YoutubeSearch => "YOUTUBE_SEARCH",
                AdNetworkType::YoutubeWatch => "YOUTUBE_WATCH",
                AdNetworkType::Mixed => "MIXED",
            }
        }
    }
}
/// Message describing the status of the association between the Budget and the
/// Campaign.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BudgetCampaignAssociationStatusEnum {}
/// Nested message and enum types in `BudgetCampaignAssociationStatusEnum`.
pub mod budget_campaign_association_status_enum {
    /// Possible statuses of the association between the Budget and the Campaign.
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
    impl BudgetCampaignAssociationStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                BudgetCampaignAssociationStatus::Unspecified => "UNSPECIFIED",
                BudgetCampaignAssociationStatus::Unknown => "UNKNOWN",
                BudgetCampaignAssociationStatus::Enabled => "ENABLED",
                BudgetCampaignAssociationStatus::Removed => "REMOVED",
            }
        }
    }
}
/// Container for enumeration of Google Ads click types.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClickTypeEnum {}
/// Nested message and enum types in `ClickTypeEnum`.
pub mod click_type_enum {
    /// Enumerates Google Ads click types.
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
        /// Cross-network. From Performance Max and Discovery Campaigns.
        CrossNetwork = 57,
    }
    impl ClickType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ClickType::Unspecified => "UNSPECIFIED",
                ClickType::Unknown => "UNKNOWN",
                ClickType::AppDeeplink => "APP_DEEPLINK",
                ClickType::Breadcrumbs => "BREADCRUMBS",
                ClickType::BroadbandPlan => "BROADBAND_PLAN",
                ClickType::CallTracking => "CALL_TRACKING",
                ClickType::Calls => "CALLS",
                ClickType::ClickOnEngagementAd => "CLICK_ON_ENGAGEMENT_AD",
                ClickType::GetDirections => "GET_DIRECTIONS",
                ClickType::LocationExpansion => "LOCATION_EXPANSION",
                ClickType::LocationFormatCall => "LOCATION_FORMAT_CALL",
                ClickType::LocationFormatDirections => "LOCATION_FORMAT_DIRECTIONS",
                ClickType::LocationFormatImage => "LOCATION_FORMAT_IMAGE",
                ClickType::LocationFormatLandingPage => "LOCATION_FORMAT_LANDING_PAGE",
                ClickType::LocationFormatMap => "LOCATION_FORMAT_MAP",
                ClickType::LocationFormatStoreInfo => "LOCATION_FORMAT_STORE_INFO",
                ClickType::LocationFormatText => "LOCATION_FORMAT_TEXT",
                ClickType::MobileCallTracking => "MOBILE_CALL_TRACKING",
                ClickType::OfferPrints => "OFFER_PRINTS",
                ClickType::Other => "OTHER",
                ClickType::ProductExtensionClicks => "PRODUCT_EXTENSION_CLICKS",
                ClickType::ProductListingAdClicks => "PRODUCT_LISTING_AD_CLICKS",
                ClickType::Sitelinks => "SITELINKS",
                ClickType::StoreLocator => "STORE_LOCATOR",
                ClickType::UrlClicks => "URL_CLICKS",
                ClickType::VideoAppStoreClicks => "VIDEO_APP_STORE_CLICKS",
                ClickType::VideoCallToActionClicks => "VIDEO_CALL_TO_ACTION_CLICKS",
                ClickType::VideoCardActionHeadlineClicks => {
                    "VIDEO_CARD_ACTION_HEADLINE_CLICKS"
                }
                ClickType::VideoEndCapClicks => "VIDEO_END_CAP_CLICKS",
                ClickType::VideoWebsiteClicks => "VIDEO_WEBSITE_CLICKS",
                ClickType::VisualSitelinks => "VISUAL_SITELINKS",
                ClickType::WirelessPlan => "WIRELESS_PLAN",
                ClickType::ProductListingAdLocal => "PRODUCT_LISTING_AD_LOCAL",
                ClickType::ProductListingAdMultichannelLocal => {
                    "PRODUCT_LISTING_AD_MULTICHANNEL_LOCAL"
                }
                ClickType::ProductListingAdMultichannelOnline => {
                    "PRODUCT_LISTING_AD_MULTICHANNEL_ONLINE"
                }
                ClickType::ProductListingAdsCoupon => "PRODUCT_LISTING_ADS_COUPON",
                ClickType::ProductListingAdTransactable => {
                    "PRODUCT_LISTING_AD_TRANSACTABLE"
                }
                ClickType::ProductAdAppDeeplink => "PRODUCT_AD_APP_DEEPLINK",
                ClickType::ShowcaseAdCategoryLink => "SHOWCASE_AD_CATEGORY_LINK",
                ClickType::ShowcaseAdLocalStorefrontLink => {
                    "SHOWCASE_AD_LOCAL_STOREFRONT_LINK"
                }
                ClickType::ShowcaseAdOnlineProductLink => {
                    "SHOWCASE_AD_ONLINE_PRODUCT_LINK"
                }
                ClickType::ShowcaseAdLocalProductLink => "SHOWCASE_AD_LOCAL_PRODUCT_LINK",
                ClickType::PromotionExtension => "PROMOTION_EXTENSION",
                ClickType::SwipeableGalleryAdHeadline => "SWIPEABLE_GALLERY_AD_HEADLINE",
                ClickType::SwipeableGalleryAdSwipes => "SWIPEABLE_GALLERY_AD_SWIPES",
                ClickType::SwipeableGalleryAdSeeMore => "SWIPEABLE_GALLERY_AD_SEE_MORE",
                ClickType::SwipeableGalleryAdSitelinkOne => {
                    "SWIPEABLE_GALLERY_AD_SITELINK_ONE"
                }
                ClickType::SwipeableGalleryAdSitelinkTwo => {
                    "SWIPEABLE_GALLERY_AD_SITELINK_TWO"
                }
                ClickType::SwipeableGalleryAdSitelinkThree => {
                    "SWIPEABLE_GALLERY_AD_SITELINK_THREE"
                }
                ClickType::SwipeableGalleryAdSitelinkFour => {
                    "SWIPEABLE_GALLERY_AD_SITELINK_FOUR"
                }
                ClickType::SwipeableGalleryAdSitelinkFive => {
                    "SWIPEABLE_GALLERY_AD_SITELINK_FIVE"
                }
                ClickType::HotelPrice => "HOTEL_PRICE",
                ClickType::PriceExtension => "PRICE_EXTENSION",
                ClickType::HotelBookOnGoogleRoomSelection => {
                    "HOTEL_BOOK_ON_GOOGLE_ROOM_SELECTION"
                }
                ClickType::ShoppingComparisonListing => "SHOPPING_COMPARISON_LISTING",
                ClickType::CrossNetwork => "CROSS_NETWORK",
            }
        }
    }
}
/// Container for enum describing the category of conversions that are associated
/// with a ConversionAction.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConversionActionCategoryEnum {}
/// Nested message and enum types in `ConversionActionCategoryEnum`.
pub mod conversion_action_category_enum {
    /// The category of conversions that are associated with a ConversionAction.
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
        /// A lead conversion imported from an external source into Google Ads,
        /// that has been further qualified by the advertiser (marketing/sales team).
        /// In the lead-to-sale journey, advertisers get leads, then act on them
        /// by reaching out to the consumer. If the consumer is interested and
        /// may end up buying their product, the advertiser marks such leads as
        /// "qualified leads".
        QualifiedLead = 22,
        /// A lead conversion imported from an external source into Google Ads, that
        /// has further completed a chosen stage as defined by the lead gen
        /// advertiser.
        ConvertedLead = 23,
    }
    impl ConversionActionCategory {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ConversionActionCategory::Unspecified => "UNSPECIFIED",
                ConversionActionCategory::Unknown => "UNKNOWN",
                ConversionActionCategory::Default => "DEFAULT",
                ConversionActionCategory::PageView => "PAGE_VIEW",
                ConversionActionCategory::Purchase => "PURCHASE",
                ConversionActionCategory::Signup => "SIGNUP",
                ConversionActionCategory::Download => "DOWNLOAD",
                ConversionActionCategory::AddToCart => "ADD_TO_CART",
                ConversionActionCategory::BeginCheckout => "BEGIN_CHECKOUT",
                ConversionActionCategory::SubscribePaid => "SUBSCRIBE_PAID",
                ConversionActionCategory::PhoneCallLead => "PHONE_CALL_LEAD",
                ConversionActionCategory::ImportedLead => "IMPORTED_LEAD",
                ConversionActionCategory::SubmitLeadForm => "SUBMIT_LEAD_FORM",
                ConversionActionCategory::BookAppointment => "BOOK_APPOINTMENT",
                ConversionActionCategory::RequestQuote => "REQUEST_QUOTE",
                ConversionActionCategory::GetDirections => "GET_DIRECTIONS",
                ConversionActionCategory::OutboundClick => "OUTBOUND_CLICK",
                ConversionActionCategory::Contact => "CONTACT",
                ConversionActionCategory::Engagement => "ENGAGEMENT",
                ConversionActionCategory::StoreVisit => "STORE_VISIT",
                ConversionActionCategory::StoreSale => "STORE_SALE",
                ConversionActionCategory::QualifiedLead => "QUALIFIED_LEAD",
                ConversionActionCategory::ConvertedLead => "CONVERTED_LEAD",
            }
        }
    }
}
/// Container for enum indicating the event type the conversion is attributed to.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConversionAttributionEventTypeEnum {}
/// Nested message and enum types in `ConversionAttributionEventTypeEnum`.
pub mod conversion_attribution_event_type_enum {
    /// The event type of conversions that are attributed to.
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
    impl ConversionAttributionEventType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ConversionAttributionEventType::Unspecified => "UNSPECIFIED",
                ConversionAttributionEventType::Unknown => "UNKNOWN",
                ConversionAttributionEventType::Impression => "IMPRESSION",
                ConversionAttributionEventType::Interaction => "INTERACTION",
            }
        }
    }
}
/// Container for enum representing the number of days between impression and
/// conversion.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConversionLagBucketEnum {}
/// Nested message and enum types in `ConversionLagBucketEnum`.
pub mod conversion_lag_bucket_enum {
    /// Enum representing the number of days between impression and conversion.
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
    impl ConversionLagBucket {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ConversionLagBucket::Unspecified => "UNSPECIFIED",
                ConversionLagBucket::Unknown => "UNKNOWN",
                ConversionLagBucket::LessThanOneDay => "LESS_THAN_ONE_DAY",
                ConversionLagBucket::OneToTwoDays => "ONE_TO_TWO_DAYS",
                ConversionLagBucket::TwoToThreeDays => "TWO_TO_THREE_DAYS",
                ConversionLagBucket::ThreeToFourDays => "THREE_TO_FOUR_DAYS",
                ConversionLagBucket::FourToFiveDays => "FOUR_TO_FIVE_DAYS",
                ConversionLagBucket::FiveToSixDays => "FIVE_TO_SIX_DAYS",
                ConversionLagBucket::SixToSevenDays => "SIX_TO_SEVEN_DAYS",
                ConversionLagBucket::SevenToEightDays => "SEVEN_TO_EIGHT_DAYS",
                ConversionLagBucket::EightToNineDays => "EIGHT_TO_NINE_DAYS",
                ConversionLagBucket::NineToTenDays => "NINE_TO_TEN_DAYS",
                ConversionLagBucket::TenToElevenDays => "TEN_TO_ELEVEN_DAYS",
                ConversionLagBucket::ElevenToTwelveDays => "ELEVEN_TO_TWELVE_DAYS",
                ConversionLagBucket::TwelveToThirteenDays => "TWELVE_TO_THIRTEEN_DAYS",
                ConversionLagBucket::ThirteenToFourteenDays => {
                    "THIRTEEN_TO_FOURTEEN_DAYS"
                }
                ConversionLagBucket::FourteenToTwentyOneDays => {
                    "FOURTEEN_TO_TWENTY_ONE_DAYS"
                }
                ConversionLagBucket::TwentyOneToThirtyDays => "TWENTY_ONE_TO_THIRTY_DAYS",
                ConversionLagBucket::ThirtyToFortyFiveDays => "THIRTY_TO_FORTY_FIVE_DAYS",
                ConversionLagBucket::FortyFiveToSixtyDays => "FORTY_FIVE_TO_SIXTY_DAYS",
                ConversionLagBucket::SixtyToNinetyDays => "SIXTY_TO_NINETY_DAYS",
            }
        }
    }
}
/// Container for enum representing the number of days between the impression and
/// the conversion or between the impression and adjustments to the conversion.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConversionOrAdjustmentLagBucketEnum {}
/// Nested message and enum types in `ConversionOrAdjustmentLagBucketEnum`.
pub mod conversion_or_adjustment_lag_bucket_enum {
    /// Enum representing the number of days between the impression and the
    /// conversion or between the impression and adjustments to the conversion.
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
    impl ConversionOrAdjustmentLagBucket {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ConversionOrAdjustmentLagBucket::Unspecified => "UNSPECIFIED",
                ConversionOrAdjustmentLagBucket::Unknown => "UNKNOWN",
                ConversionOrAdjustmentLagBucket::ConversionLessThanOneDay => {
                    "CONVERSION_LESS_THAN_ONE_DAY"
                }
                ConversionOrAdjustmentLagBucket::ConversionOneToTwoDays => {
                    "CONVERSION_ONE_TO_TWO_DAYS"
                }
                ConversionOrAdjustmentLagBucket::ConversionTwoToThreeDays => {
                    "CONVERSION_TWO_TO_THREE_DAYS"
                }
                ConversionOrAdjustmentLagBucket::ConversionThreeToFourDays => {
                    "CONVERSION_THREE_TO_FOUR_DAYS"
                }
                ConversionOrAdjustmentLagBucket::ConversionFourToFiveDays => {
                    "CONVERSION_FOUR_TO_FIVE_DAYS"
                }
                ConversionOrAdjustmentLagBucket::ConversionFiveToSixDays => {
                    "CONVERSION_FIVE_TO_SIX_DAYS"
                }
                ConversionOrAdjustmentLagBucket::ConversionSixToSevenDays => {
                    "CONVERSION_SIX_TO_SEVEN_DAYS"
                }
                ConversionOrAdjustmentLagBucket::ConversionSevenToEightDays => {
                    "CONVERSION_SEVEN_TO_EIGHT_DAYS"
                }
                ConversionOrAdjustmentLagBucket::ConversionEightToNineDays => {
                    "CONVERSION_EIGHT_TO_NINE_DAYS"
                }
                ConversionOrAdjustmentLagBucket::ConversionNineToTenDays => {
                    "CONVERSION_NINE_TO_TEN_DAYS"
                }
                ConversionOrAdjustmentLagBucket::ConversionTenToElevenDays => {
                    "CONVERSION_TEN_TO_ELEVEN_DAYS"
                }
                ConversionOrAdjustmentLagBucket::ConversionElevenToTwelveDays => {
                    "CONVERSION_ELEVEN_TO_TWELVE_DAYS"
                }
                ConversionOrAdjustmentLagBucket::ConversionTwelveToThirteenDays => {
                    "CONVERSION_TWELVE_TO_THIRTEEN_DAYS"
                }
                ConversionOrAdjustmentLagBucket::ConversionThirteenToFourteenDays => {
                    "CONVERSION_THIRTEEN_TO_FOURTEEN_DAYS"
                }
                ConversionOrAdjustmentLagBucket::ConversionFourteenToTwentyOneDays => {
                    "CONVERSION_FOURTEEN_TO_TWENTY_ONE_DAYS"
                }
                ConversionOrAdjustmentLagBucket::ConversionTwentyOneToThirtyDays => {
                    "CONVERSION_TWENTY_ONE_TO_THIRTY_DAYS"
                }
                ConversionOrAdjustmentLagBucket::ConversionThirtyToFortyFiveDays => {
                    "CONVERSION_THIRTY_TO_FORTY_FIVE_DAYS"
                }
                ConversionOrAdjustmentLagBucket::ConversionFortyFiveToSixtyDays => {
                    "CONVERSION_FORTY_FIVE_TO_SIXTY_DAYS"
                }
                ConversionOrAdjustmentLagBucket::ConversionSixtyToNinetyDays => {
                    "CONVERSION_SIXTY_TO_NINETY_DAYS"
                }
                ConversionOrAdjustmentLagBucket::AdjustmentLessThanOneDay => {
                    "ADJUSTMENT_LESS_THAN_ONE_DAY"
                }
                ConversionOrAdjustmentLagBucket::AdjustmentOneToTwoDays => {
                    "ADJUSTMENT_ONE_TO_TWO_DAYS"
                }
                ConversionOrAdjustmentLagBucket::AdjustmentTwoToThreeDays => {
                    "ADJUSTMENT_TWO_TO_THREE_DAYS"
                }
                ConversionOrAdjustmentLagBucket::AdjustmentThreeToFourDays => {
                    "ADJUSTMENT_THREE_TO_FOUR_DAYS"
                }
                ConversionOrAdjustmentLagBucket::AdjustmentFourToFiveDays => {
                    "ADJUSTMENT_FOUR_TO_FIVE_DAYS"
                }
                ConversionOrAdjustmentLagBucket::AdjustmentFiveToSixDays => {
                    "ADJUSTMENT_FIVE_TO_SIX_DAYS"
                }
                ConversionOrAdjustmentLagBucket::AdjustmentSixToSevenDays => {
                    "ADJUSTMENT_SIX_TO_SEVEN_DAYS"
                }
                ConversionOrAdjustmentLagBucket::AdjustmentSevenToEightDays => {
                    "ADJUSTMENT_SEVEN_TO_EIGHT_DAYS"
                }
                ConversionOrAdjustmentLagBucket::AdjustmentEightToNineDays => {
                    "ADJUSTMENT_EIGHT_TO_NINE_DAYS"
                }
                ConversionOrAdjustmentLagBucket::AdjustmentNineToTenDays => {
                    "ADJUSTMENT_NINE_TO_TEN_DAYS"
                }
                ConversionOrAdjustmentLagBucket::AdjustmentTenToElevenDays => {
                    "ADJUSTMENT_TEN_TO_ELEVEN_DAYS"
                }
                ConversionOrAdjustmentLagBucket::AdjustmentElevenToTwelveDays => {
                    "ADJUSTMENT_ELEVEN_TO_TWELVE_DAYS"
                }
                ConversionOrAdjustmentLagBucket::AdjustmentTwelveToThirteenDays => {
                    "ADJUSTMENT_TWELVE_TO_THIRTEEN_DAYS"
                }
                ConversionOrAdjustmentLagBucket::AdjustmentThirteenToFourteenDays => {
                    "ADJUSTMENT_THIRTEEN_TO_FOURTEEN_DAYS"
                }
                ConversionOrAdjustmentLagBucket::AdjustmentFourteenToTwentyOneDays => {
                    "ADJUSTMENT_FOURTEEN_TO_TWENTY_ONE_DAYS"
                }
                ConversionOrAdjustmentLagBucket::AdjustmentTwentyOneToThirtyDays => {
                    "ADJUSTMENT_TWENTY_ONE_TO_THIRTY_DAYS"
                }
                ConversionOrAdjustmentLagBucket::AdjustmentThirtyToFortyFiveDays => {
                    "ADJUSTMENT_THIRTY_TO_FORTY_FIVE_DAYS"
                }
                ConversionOrAdjustmentLagBucket::AdjustmentFortyFiveToSixtyDays => {
                    "ADJUSTMENT_FORTY_FIVE_TO_SIXTY_DAYS"
                }
                ConversionOrAdjustmentLagBucket::AdjustmentSixtyToNinetyDays => {
                    "ADJUSTMENT_SIXTY_TO_NINETY_DAYS"
                }
                ConversionOrAdjustmentLagBucket::AdjustmentNinetyToOneHundredAndFortyFiveDays => {
                    "ADJUSTMENT_NINETY_TO_ONE_HUNDRED_AND_FORTY_FIVE_DAYS"
                }
                ConversionOrAdjustmentLagBucket::ConversionUnknown => {
                    "CONVERSION_UNKNOWN"
                }
                ConversionOrAdjustmentLagBucket::AdjustmentUnknown => {
                    "ADJUSTMENT_UNKNOWN"
                }
            }
        }
    }
}
/// Container for enum describing value rule primary dimension for stats.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConversionValueRulePrimaryDimensionEnum {}
/// Nested message and enum types in `ConversionValueRulePrimaryDimensionEnum`.
pub mod conversion_value_rule_primary_dimension_enum {
    /// Identifies the primary dimension for conversion value rule stats.
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
    pub enum ConversionValueRulePrimaryDimension {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// For no-value-rule-applied conversions after value rule is enabled.
        NoRuleApplied = 2,
        /// Below are for value-rule-applied conversions:
        /// The original stats.
        Original = 3,
        /// When a new or returning customer condition is satisfied.
        NewVsReturningUser = 4,
        /// When a query-time Geo location condition is satisfied.
        GeoLocation = 5,
        /// When a query-time browsing device condition is satisfied.
        Device = 6,
        /// When a query-time audience condition is satisfied.
        Audience = 7,
        /// When multiple rules are applied.
        Multiple = 8,
    }
    impl ConversionValueRulePrimaryDimension {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ConversionValueRulePrimaryDimension::Unspecified => "UNSPECIFIED",
                ConversionValueRulePrimaryDimension::Unknown => "UNKNOWN",
                ConversionValueRulePrimaryDimension::NoRuleApplied => "NO_RULE_APPLIED",
                ConversionValueRulePrimaryDimension::Original => "ORIGINAL",
                ConversionValueRulePrimaryDimension::NewVsReturningUser => {
                    "NEW_VS_RETURNING_USER"
                }
                ConversionValueRulePrimaryDimension::GeoLocation => "GEO_LOCATION",
                ConversionValueRulePrimaryDimension::Device => "DEVICE",
                ConversionValueRulePrimaryDimension::Audience => "AUDIENCE",
                ConversionValueRulePrimaryDimension::Multiple => "MULTIPLE",
            }
        }
    }
}
/// Container for enum describing the external conversion source that is
/// associated with a ConversionAction.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExternalConversionSourceEnum {}
/// Nested message and enum types in `ExternalConversionSourceEnum`.
pub mod external_conversion_source_enum {
    /// The external conversion source that is associated with a ConversionAction.
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
        /// Conversion that is reported by Floodlight for DV360.
        DisplayAndVideo360Floodlight = 34,
    }
    impl ExternalConversionSource {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ExternalConversionSource::Unspecified => "UNSPECIFIED",
                ExternalConversionSource::Unknown => "UNKNOWN",
                ExternalConversionSource::Webpage => "WEBPAGE",
                ExternalConversionSource::Analytics => "ANALYTICS",
                ExternalConversionSource::Upload => "UPLOAD",
                ExternalConversionSource::AdCallMetrics => "AD_CALL_METRICS",
                ExternalConversionSource::WebsiteCallMetrics => "WEBSITE_CALL_METRICS",
                ExternalConversionSource::StoreVisits => "STORE_VISITS",
                ExternalConversionSource::AndroidInApp => "ANDROID_IN_APP",
                ExternalConversionSource::IosInApp => "IOS_IN_APP",
                ExternalConversionSource::IosFirstOpen => "IOS_FIRST_OPEN",
                ExternalConversionSource::AppUnspecified => "APP_UNSPECIFIED",
                ExternalConversionSource::AndroidFirstOpen => "ANDROID_FIRST_OPEN",
                ExternalConversionSource::UploadCalls => "UPLOAD_CALLS",
                ExternalConversionSource::Firebase => "FIREBASE",
                ExternalConversionSource::ClickToCall => "CLICK_TO_CALL",
                ExternalConversionSource::Salesforce => "SALESFORCE",
                ExternalConversionSource::StoreSalesCrm => "STORE_SALES_CRM",
                ExternalConversionSource::StoreSalesPaymentNetwork => {
                    "STORE_SALES_PAYMENT_NETWORK"
                }
                ExternalConversionSource::GooglePlay => "GOOGLE_PLAY",
                ExternalConversionSource::ThirdPartyAppAnalytics => {
                    "THIRD_PARTY_APP_ANALYTICS"
                }
                ExternalConversionSource::GoogleAttribution => "GOOGLE_ATTRIBUTION",
                ExternalConversionSource::StoreSalesDirectUpload => {
                    "STORE_SALES_DIRECT_UPLOAD"
                }
                ExternalConversionSource::StoreSales => "STORE_SALES",
                ExternalConversionSource::SearchAds360 => "SEARCH_ADS_360",
                ExternalConversionSource::GoogleHosted => "GOOGLE_HOSTED",
                ExternalConversionSource::Floodlight => "FLOODLIGHT",
                ExternalConversionSource::AnalyticsSearchAds360 => {
                    "ANALYTICS_SEARCH_ADS_360"
                }
                ExternalConversionSource::FirebaseSearchAds360 => {
                    "FIREBASE_SEARCH_ADS_360"
                }
                ExternalConversionSource::DisplayAndVideo360Floodlight => {
                    "DISPLAY_AND_VIDEO_360_FLOODLIGHT"
                }
            }
        }
    }
}
/// Container for enum describing hotel price bucket for a hotel itinerary.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HotelPriceBucketEnum {}
/// Nested message and enum types in `HotelPriceBucketEnum`.
pub mod hotel_price_bucket_enum {
    /// Enum describing possible hotel price buckets.
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
    impl HotelPriceBucket {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                HotelPriceBucket::Unspecified => "UNSPECIFIED",
                HotelPriceBucket::Unknown => "UNKNOWN",
                HotelPriceBucket::LowestUnique => "LOWEST_UNIQUE",
                HotelPriceBucket::LowestTied => "LOWEST_TIED",
                HotelPriceBucket::NotLowest => "NOT_LOWEST",
                HotelPriceBucket::OnlyPartnerShown => "ONLY_PARTNER_SHOWN",
            }
        }
    }
}
/// Container for enum describing possible hotel rate types.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HotelRateTypeEnum {}
/// Nested message and enum types in `HotelRateTypeEnum`.
pub mod hotel_rate_type_enum {
    /// Enum describing possible hotel rate types.
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
        /// Rates available to users that satisfy some eligibility criteria, for
        /// example, all signed-in users, 20% of mobile users, all mobile users in
        /// Canada, etc.
        PrivateRate = 5,
    }
    impl HotelRateType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                HotelRateType::Unspecified => "UNSPECIFIED",
                HotelRateType::Unknown => "UNKNOWN",
                HotelRateType::Unavailable => "UNAVAILABLE",
                HotelRateType::PublicRate => "PUBLIC_RATE",
                HotelRateType::QualifiedRate => "QUALIFIED_RATE",
                HotelRateType::PrivateRate => "PRIVATE_RATE",
            }
        }
    }
}
/// Container for enum describing possible placeholder types for a feed mapping.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlaceholderTypeEnum {}
/// Nested message and enum types in `PlaceholderTypeEnum`.
pub mod placeholder_type_enum {
    /// Possible placeholder types for a feed mapping.
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
        /// Lets you show locations of businesses from your Business Profile
        /// in your ad. This helps people find your locations by showing your
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
        /// Lets you highlight sales and other promotions that let users see how
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
    impl PlaceholderType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                PlaceholderType::Unspecified => "UNSPECIFIED",
                PlaceholderType::Unknown => "UNKNOWN",
                PlaceholderType::Sitelink => "SITELINK",
                PlaceholderType::Call => "CALL",
                PlaceholderType::App => "APP",
                PlaceholderType::Location => "LOCATION",
                PlaceholderType::AffiliateLocation => "AFFILIATE_LOCATION",
                PlaceholderType::Callout => "CALLOUT",
                PlaceholderType::StructuredSnippet => "STRUCTURED_SNIPPET",
                PlaceholderType::Message => "MESSAGE",
                PlaceholderType::Price => "PRICE",
                PlaceholderType::Promotion => "PROMOTION",
                PlaceholderType::AdCustomizer => "AD_CUSTOMIZER",
                PlaceholderType::DynamicEducation => "DYNAMIC_EDUCATION",
                PlaceholderType::DynamicFlight => "DYNAMIC_FLIGHT",
                PlaceholderType::DynamicCustom => "DYNAMIC_CUSTOM",
                PlaceholderType::DynamicHotel => "DYNAMIC_HOTEL",
                PlaceholderType::DynamicRealEstate => "DYNAMIC_REAL_ESTATE",
                PlaceholderType::DynamicTravel => "DYNAMIC_TRAVEL",
                PlaceholderType::DynamicLocal => "DYNAMIC_LOCAL",
                PlaceholderType::DynamicJob => "DYNAMIC_JOB",
                PlaceholderType::Image => "IMAGE",
            }
        }
    }
}
/// Container for enum describing types of recommendations.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RecommendationTypeEnum {}
/// Nested message and enum types in `RecommendationTypeEnum`.
pub mod recommendation_type_enum {
    /// Types of recommendations.
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
        /// Recommendation to expand keywords to broad match for fully automated
        /// conversion-based bidding campaigns.
        UseBroadMatchKeyword = 20,
        /// Recommendation to add new responsive search ad assets.
        ResponsiveSearchAdAsset = 21,
        /// Recommendation to upgrade a Smart Shopping campaign to a Performance Max
        /// campaign.
        UpgradeSmartShoppingCampaignToPerformanceMax = 22,
        /// Recommendation to improve strength of responsive search ad.
        ResponsiveSearchAdImproveAdStrength = 23,
        /// Recommendation to update a campaign to use Display Expansion.
        DisplayExpansionOptIn = 24,
        /// Recommendation to upgrade a Local campaign to a Performance Max
        /// campaign.
        UpgradeLocalCampaignToPerformanceMax = 25,
    }
    impl RecommendationType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                RecommendationType::Unspecified => "UNSPECIFIED",
                RecommendationType::Unknown => "UNKNOWN",
                RecommendationType::CampaignBudget => "CAMPAIGN_BUDGET",
                RecommendationType::Keyword => "KEYWORD",
                RecommendationType::TextAd => "TEXT_AD",
                RecommendationType::TargetCpaOptIn => "TARGET_CPA_OPT_IN",
                RecommendationType::MaximizeConversionsOptIn => {
                    "MAXIMIZE_CONVERSIONS_OPT_IN"
                }
                RecommendationType::EnhancedCpcOptIn => "ENHANCED_CPC_OPT_IN",
                RecommendationType::SearchPartnersOptIn => "SEARCH_PARTNERS_OPT_IN",
                RecommendationType::MaximizeClicksOptIn => "MAXIMIZE_CLICKS_OPT_IN",
                RecommendationType::OptimizeAdRotation => "OPTIMIZE_AD_ROTATION",
                RecommendationType::CalloutExtension => "CALLOUT_EXTENSION",
                RecommendationType::SitelinkExtension => "SITELINK_EXTENSION",
                RecommendationType::CallExtension => "CALL_EXTENSION",
                RecommendationType::KeywordMatchType => "KEYWORD_MATCH_TYPE",
                RecommendationType::MoveUnusedBudget => "MOVE_UNUSED_BUDGET",
                RecommendationType::ForecastingCampaignBudget => {
                    "FORECASTING_CAMPAIGN_BUDGET"
                }
                RecommendationType::TargetRoasOptIn => "TARGET_ROAS_OPT_IN",
                RecommendationType::ResponsiveSearchAd => "RESPONSIVE_SEARCH_AD",
                RecommendationType::MarginalRoiCampaignBudget => {
                    "MARGINAL_ROI_CAMPAIGN_BUDGET"
                }
                RecommendationType::UseBroadMatchKeyword => "USE_BROAD_MATCH_KEYWORD",
                RecommendationType::ResponsiveSearchAdAsset => {
                    "RESPONSIVE_SEARCH_AD_ASSET"
                }
                RecommendationType::UpgradeSmartShoppingCampaignToPerformanceMax => {
                    "UPGRADE_SMART_SHOPPING_CAMPAIGN_TO_PERFORMANCE_MAX"
                }
                RecommendationType::ResponsiveSearchAdImproveAdStrength => {
                    "RESPONSIVE_SEARCH_AD_IMPROVE_AD_STRENGTH"
                }
                RecommendationType::DisplayExpansionOptIn => "DISPLAY_EXPANSION_OPT_IN",
                RecommendationType::UpgradeLocalCampaignToPerformanceMax => {
                    "UPGRADE_LOCAL_CAMPAIGN_TO_PERFORMANCE_MAX"
                }
            }
        }
    }
}
/// The type of the search engine results page.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchEngineResultsPageTypeEnum {}
/// Nested message and enum types in `SearchEngineResultsPageTypeEnum`.
pub mod search_engine_results_page_type_enum {
    /// The type of the search engine results page.
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
    impl SearchEngineResultsPageType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                SearchEngineResultsPageType::Unspecified => "UNSPECIFIED",
                SearchEngineResultsPageType::Unknown => "UNKNOWN",
                SearchEngineResultsPageType::AdsOnly => "ADS_ONLY",
                SearchEngineResultsPageType::OrganicOnly => "ORGANIC_ONLY",
                SearchEngineResultsPageType::AdsAndOrganic => "ADS_AND_ORGANIC",
            }
        }
    }
}
/// Container for enum describing match types for a keyword triggering an ad.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchTermMatchTypeEnum {}
/// Nested message and enum types in `SearchTermMatchTypeEnum`.
pub mod search_term_match_type_enum {
    /// Possible match types for a keyword triggering an ad, including variants.
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
    impl SearchTermMatchType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                SearchTermMatchType::Unspecified => "UNSPECIFIED",
                SearchTermMatchType::Unknown => "UNKNOWN",
                SearchTermMatchType::Broad => "BROAD",
                SearchTermMatchType::Exact => "EXACT",
                SearchTermMatchType::Phrase => "PHRASE",
                SearchTermMatchType::NearExact => "NEAR_EXACT",
                SearchTermMatchType::NearPhrase => "NEAR_PHRASE",
            }
        }
    }
}
/// Container for enumeration of SkAdNetwork ad event types.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SkAdNetworkAdEventTypeEnum {}
/// Nested message and enum types in `SkAdNetworkAdEventTypeEnum`.
pub mod sk_ad_network_ad_event_type_enum {
    /// Enumerates SkAdNetwork ad event types
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
    pub enum SkAdNetworkAdEventType {
        /// Not specified.
        Unspecified = 0,
        /// The value is unknown in this version.
        Unknown = 1,
        /// The value was not present in the postback or we do not have this data for
        /// other reasons.
        Unavailable = 2,
        /// The user interacted with the ad.
        Interaction = 3,
        /// The user viewed the ad.
        View = 4,
    }
    impl SkAdNetworkAdEventType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                SkAdNetworkAdEventType::Unspecified => "UNSPECIFIED",
                SkAdNetworkAdEventType::Unknown => "UNKNOWN",
                SkAdNetworkAdEventType::Unavailable => "UNAVAILABLE",
                SkAdNetworkAdEventType::Interaction => "INTERACTION",
                SkAdNetworkAdEventType::View => "VIEW",
            }
        }
    }
}
/// Container for enumeration of SkAdNetwork attribution credits.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SkAdNetworkAttributionCreditEnum {}
/// Nested message and enum types in `SkAdNetworkAttributionCreditEnum`.
pub mod sk_ad_network_attribution_credit_enum {
    /// Enumerates SkAdNetwork attribution credits.
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
    pub enum SkAdNetworkAttributionCredit {
        /// Default value. This value is equivalent to null.
        Unspecified = 0,
        /// The value is unknown in this API version. The true enum value cannot be
        /// returned in this API version or is not supported yet.
        Unknown = 1,
        /// The value was not present in the postback or we do not have this data for
        /// other reasons.
        Unavailable = 2,
        /// Google was the ad network that won ad attribution.
        Won = 3,
        /// Google qualified for attribution, but didn't win.
        Contributed = 4,
    }
    impl SkAdNetworkAttributionCredit {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                SkAdNetworkAttributionCredit::Unspecified => "UNSPECIFIED",
                SkAdNetworkAttributionCredit::Unknown => "UNKNOWN",
                SkAdNetworkAttributionCredit::Unavailable => "UNAVAILABLE",
                SkAdNetworkAttributionCredit::Won => "WON",
                SkAdNetworkAttributionCredit::Contributed => "CONTRIBUTED",
            }
        }
    }
}
/// Container for enumeration of SkAdNetwork user types.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SkAdNetworkUserTypeEnum {}
/// Nested message and enum types in `SkAdNetworkUserTypeEnum`.
pub mod sk_ad_network_user_type_enum {
    /// Enumerates SkAdNetwork user types
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
    pub enum SkAdNetworkUserType {
        /// Not specified.
        Unspecified = 0,
        /// The value is unknown in this version.
        Unknown = 1,
        /// The value was not present in the postback or we do not have this data for
        /// other reasons.
        Unavailable = 2,
        /// The user installed the app for the first time.
        NewInstaller = 3,
        /// The user has previously installed the app.
        Reinstaller = 4,
    }
    impl SkAdNetworkUserType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                SkAdNetworkUserType::Unspecified => "UNSPECIFIED",
                SkAdNetworkUserType::Unknown => "UNKNOWN",
                SkAdNetworkUserType::Unavailable => "UNAVAILABLE",
                SkAdNetworkUserType::NewInstaller => "NEW_INSTALLER",
                SkAdNetworkUserType::Reinstaller => "REINSTALLER",
            }
        }
    }
}
/// Container for enumeration of possible positions of the Ad.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SlotEnum {}
/// Nested message and enum types in `SlotEnum`.
pub mod slot_enum {
    /// Enumerates possible positions of the Ad.
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
    impl Slot {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Slot::Unspecified => "UNSPECIFIED",
                Slot::Unknown => "UNKNOWN",
                Slot::SearchSide => "SEARCH_SIDE",
                Slot::SearchTop => "SEARCH_TOP",
                Slot::SearchOther => "SEARCH_OTHER",
                Slot::Content => "CONTENT",
                Slot::SearchPartnerTop => "SEARCH_PARTNER_TOP",
                Slot::SearchPartnerOther => "SEARCH_PARTNER_OTHER",
                Slot::Mixed => "MIXED",
            }
        }
    }
}
/// Container for enum describing the format of the web page where the tracking
/// tag and snippet will be installed.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TrackingCodePageFormatEnum {}
/// Nested message and enum types in `TrackingCodePageFormatEnum`.
pub mod tracking_code_page_format_enum {
    /// The format of the web page where the tracking tag and snippet will be
    /// installed.
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
    impl TrackingCodePageFormat {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                TrackingCodePageFormat::Unspecified => "UNSPECIFIED",
                TrackingCodePageFormat::Unknown => "UNKNOWN",
                TrackingCodePageFormat::Html => "HTML",
                TrackingCodePageFormat::Amp => "AMP",
            }
        }
    }
}
/// Container for enum describing the type of the generated tag snippets for
/// tracking conversions.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TrackingCodeTypeEnum {}
/// Nested message and enum types in `TrackingCodeTypeEnum`.
pub mod tracking_code_type_enum {
    /// The type of the generated tag snippets for tracking conversions.
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
    impl TrackingCodeType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                TrackingCodeType::Unspecified => "UNSPECIFIED",
                TrackingCodeType::Unknown => "UNKNOWN",
                TrackingCodeType::Webpage => "WEBPAGE",
                TrackingCodeType::WebpageOnclick => "WEBPAGE_ONCLICK",
                TrackingCodeType::ClickToCall => "CLICK_TO_CALL",
                TrackingCodeType::WebsiteCall => "WEBSITE_CALL",
            }
        }
    }
}
/// The dimensions that can be targeted.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TargetingDimensionEnum {}
/// Nested message and enum types in `TargetingDimensionEnum`.
pub mod targeting_dimension_enum {
    /// Enum describing possible targeting dimensions.
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
    pub enum TargetingDimension {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Keyword criteria, for example, 'mars cruise'. KEYWORD may be used as a
        /// custom bid dimension. Keywords are always a targeting dimension, so may
        /// not be set as a target "ALL" dimension with TargetRestriction.
        Keyword = 2,
        /// Audience criteria, which include user list, user interest, custom
        /// affinity,  and custom in market.
        Audience = 3,
        /// Topic criteria for targeting categories of content, for example,
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
    impl TargetingDimension {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                TargetingDimension::Unspecified => "UNSPECIFIED",
                TargetingDimension::Unknown => "UNKNOWN",
                TargetingDimension::Keyword => "KEYWORD",
                TargetingDimension::Audience => "AUDIENCE",
                TargetingDimension::Topic => "TOPIC",
                TargetingDimension::Gender => "GENDER",
                TargetingDimension::AgeRange => "AGE_RANGE",
                TargetingDimension::Placement => "PLACEMENT",
                TargetingDimension::ParentalStatus => "PARENTAL_STATUS",
                TargetingDimension::IncomeRange => "INCOME_RANGE",
            }
        }
    }
}
/// Indicates what type of data are the user list's members matched from.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomerMatchUploadKeyTypeEnum {}
/// Nested message and enum types in `CustomerMatchUploadKeyTypeEnum`.
pub mod customer_match_upload_key_type_enum {
    /// Enum describing possible customer match upload key types.
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
    impl CustomerMatchUploadKeyType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                CustomerMatchUploadKeyType::Unspecified => "UNSPECIFIED",
                CustomerMatchUploadKeyType::Unknown => "UNKNOWN",
                CustomerMatchUploadKeyType::ContactInfo => "CONTACT_INFO",
                CustomerMatchUploadKeyType::CrmId => "CRM_ID",
                CustomerMatchUploadKeyType::MobileAdvertisingId => {
                    "MOBILE_ADVERTISING_ID"
                }
            }
        }
    }
}
/// Logical operator connecting two rules.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserListCombinedRuleOperatorEnum {}
/// Nested message and enum types in `UserListCombinedRuleOperatorEnum`.
pub mod user_list_combined_rule_operator_enum {
    /// Enum describing possible user list combined rule operators.
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
    impl UserListCombinedRuleOperator {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                UserListCombinedRuleOperator::Unspecified => "UNSPECIFIED",
                UserListCombinedRuleOperator::Unknown => "UNKNOWN",
                UserListCombinedRuleOperator::And => "AND",
                UserListCombinedRuleOperator::AndNot => "AND_NOT",
            }
        }
    }
}
/// Indicates source of Crm upload data.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserListCrmDataSourceTypeEnum {}
/// Nested message and enum types in `UserListCrmDataSourceTypeEnum`.
pub mod user_list_crm_data_source_type_enum {
    /// Enum describing possible user list crm data source type.
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
    impl UserListCrmDataSourceType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                UserListCrmDataSourceType::Unspecified => "UNSPECIFIED",
                UserListCrmDataSourceType::Unknown => "UNKNOWN",
                UserListCrmDataSourceType::FirstParty => "FIRST_PARTY",
                UserListCrmDataSourceType::ThirdPartyCreditBureau => {
                    "THIRD_PARTY_CREDIT_BUREAU"
                }
                UserListCrmDataSourceType::ThirdPartyVoterFile => {
                    "THIRD_PARTY_VOTER_FILE"
                }
            }
        }
    }
}
/// Supported rule operator for date type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserListDateRuleItemOperatorEnum {}
/// Nested message and enum types in `UserListDateRuleItemOperatorEnum`.
pub mod user_list_date_rule_item_operator_enum {
    /// Enum describing possible user list date rule item operators.
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
    impl UserListDateRuleItemOperator {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                UserListDateRuleItemOperator::Unspecified => "UNSPECIFIED",
                UserListDateRuleItemOperator::Unknown => "UNKNOWN",
                UserListDateRuleItemOperator::Equals => "EQUALS",
                UserListDateRuleItemOperator::NotEquals => "NOT_EQUALS",
                UserListDateRuleItemOperator::Before => "BEFORE",
                UserListDateRuleItemOperator::After => "AFTER",
            }
        }
    }
}
/// Logical operator connecting two rules.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserListFlexibleRuleOperatorEnum {}
/// Nested message and enum types in `UserListFlexibleRuleOperatorEnum`.
pub mod user_list_flexible_rule_operator_enum {
    /// Enum describing possible user list combined rule operators.
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
    pub enum UserListFlexibleRuleOperator {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// A AND B.
        And = 2,
        /// A OR B.
        Or = 3,
    }
    impl UserListFlexibleRuleOperator {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                UserListFlexibleRuleOperator::Unspecified => "UNSPECIFIED",
                UserListFlexibleRuleOperator::Unknown => "UNKNOWN",
                UserListFlexibleRuleOperator::And => "AND",
                UserListFlexibleRuleOperator::Or => "OR",
            }
        }
    }
}
/// The logical operator of the rule.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserListLogicalRuleOperatorEnum {}
/// Nested message and enum types in `UserListLogicalRuleOperatorEnum`.
pub mod user_list_logical_rule_operator_enum {
    /// Enum describing possible user list logical rule operators.
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
    impl UserListLogicalRuleOperator {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                UserListLogicalRuleOperator::Unspecified => "UNSPECIFIED",
                UserListLogicalRuleOperator::Unknown => "UNKNOWN",
                UserListLogicalRuleOperator::All => "ALL",
                UserListLogicalRuleOperator::Any => "ANY",
                UserListLogicalRuleOperator::None => "NONE",
            }
        }
    }
}
/// Supported rule operator for number type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserListNumberRuleItemOperatorEnum {}
/// Nested message and enum types in `UserListNumberRuleItemOperatorEnum`.
pub mod user_list_number_rule_item_operator_enum {
    /// Enum describing possible user list number rule item operators.
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
    impl UserListNumberRuleItemOperator {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                UserListNumberRuleItemOperator::Unspecified => "UNSPECIFIED",
                UserListNumberRuleItemOperator::Unknown => "UNKNOWN",
                UserListNumberRuleItemOperator::GreaterThan => "GREATER_THAN",
                UserListNumberRuleItemOperator::GreaterThanOrEqual => {
                    "GREATER_THAN_OR_EQUAL"
                }
                UserListNumberRuleItemOperator::Equals => "EQUALS",
                UserListNumberRuleItemOperator::NotEquals => "NOT_EQUALS",
                UserListNumberRuleItemOperator::LessThan => "LESS_THAN",
                UserListNumberRuleItemOperator::LessThanOrEqual => "LESS_THAN_OR_EQUAL",
            }
        }
    }
}
/// Indicates status of prepopulation based on the rule.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserListPrepopulationStatusEnum {}
/// Nested message and enum types in `UserListPrepopulationStatusEnum`.
pub mod user_list_prepopulation_status_enum {
    /// Enum describing possible user list prepopulation status.
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
    impl UserListPrepopulationStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                UserListPrepopulationStatus::Unspecified => "UNSPECIFIED",
                UserListPrepopulationStatus::Unknown => "UNKNOWN",
                UserListPrepopulationStatus::Requested => "REQUESTED",
                UserListPrepopulationStatus::Finished => "FINISHED",
                UserListPrepopulationStatus::Failed => "FAILED",
            }
        }
    }
}
/// Rule based user list rule type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserListRuleTypeEnum {}
/// Nested message and enum types in `UserListRuleTypeEnum`.
pub mod user_list_rule_type_enum {
    /// Enum describing possible user list rule types.
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
    impl UserListRuleType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                UserListRuleType::Unspecified => "UNSPECIFIED",
                UserListRuleType::Unknown => "UNKNOWN",
                UserListRuleType::AndOfOrs => "AND_OF_ORS",
                UserListRuleType::OrOfAnds => "OR_OF_ANDS",
            }
        }
    }
}
/// Supported rule operator for string type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserListStringRuleItemOperatorEnum {}
/// Nested message and enum types in `UserListStringRuleItemOperatorEnum`.
pub mod user_list_string_rule_item_operator_enum {
    /// Enum describing possible user list string rule item operators.
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
    impl UserListStringRuleItemOperator {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                UserListStringRuleItemOperator::Unspecified => "UNSPECIFIED",
                UserListStringRuleItemOperator::Unknown => "UNKNOWN",
                UserListStringRuleItemOperator::Contains => "CONTAINS",
                UserListStringRuleItemOperator::Equals => "EQUALS",
                UserListStringRuleItemOperator::StartsWith => "STARTS_WITH",
                UserListStringRuleItemOperator::EndsWith => "ENDS_WITH",
                UserListStringRuleItemOperator::NotEquals => "NOT_EQUALS",
                UserListStringRuleItemOperator::NotContains => "NOT_CONTAINS",
                UserListStringRuleItemOperator::NotStartsWith => "NOT_STARTS_WITH",
                UserListStringRuleItemOperator::NotEndsWith => "NOT_ENDS_WITH",
            }
        }
    }
}
/// Container for enum for identifying the status of access invitation
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccessInvitationStatusEnum {}
/// Nested message and enum types in `AccessInvitationStatusEnum`.
pub mod access_invitation_status_enum {
    /// Possible access invitation status of a user
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
    impl AccessInvitationStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                AccessInvitationStatus::Unspecified => "UNSPECIFIED",
                AccessInvitationStatus::Unknown => "UNKNOWN",
                AccessInvitationStatus::Pending => "PENDING",
                AccessInvitationStatus::Declined => "DECLINED",
                AccessInvitationStatus::Expired => "EXPIRED",
            }
        }
    }
}
/// Indicates the way the resource such as user list is related to a user.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccessReasonEnum {}
/// Nested message and enum types in `AccessReasonEnum`.
pub mod access_reason_enum {
    /// Enum describing possible access reasons.
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
    impl AccessReason {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                AccessReason::Unspecified => "UNSPECIFIED",
                AccessReason::Unknown => "UNKNOWN",
                AccessReason::Owned => "OWNED",
                AccessReason::Shared => "SHARED",
                AccessReason::Licensed => "LICENSED",
                AccessReason::Subscribed => "SUBSCRIBED",
                AccessReason::Affiliated => "AFFILIATED",
            }
        }
    }
}
/// Container for enum describing possible access role for user.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccessRoleEnum {}
/// Nested message and enum types in `AccessRoleEnum`.
pub mod access_role_enum {
    /// Possible access role of a user.
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
    impl AccessRole {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                AccessRole::Unspecified => "UNSPECIFIED",
                AccessRole::Unknown => "UNKNOWN",
                AccessRole::Admin => "ADMIN",
                AccessRole::Standard => "STANDARD",
                AccessRole::ReadOnly => "READ_ONLY",
                AccessRole::EmailOnly => "EMAIL_ONLY",
            }
        }
    }
}
/// Message describing AccountBudgetProposal statuses.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountBudgetProposalStatusEnum {}
/// Nested message and enum types in `AccountBudgetProposalStatusEnum`.
pub mod account_budget_proposal_status_enum {
    /// The possible statuses of an AccountBudgetProposal.
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
        /// The proposal has been rejected by the user, for example, by rejecting an
        /// acceptance email.
        Rejected = 6,
    }
    impl AccountBudgetProposalStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                AccountBudgetProposalStatus::Unspecified => "UNSPECIFIED",
                AccountBudgetProposalStatus::Unknown => "UNKNOWN",
                AccountBudgetProposalStatus::Pending => "PENDING",
                AccountBudgetProposalStatus::ApprovedHeld => "APPROVED_HELD",
                AccountBudgetProposalStatus::Approved => "APPROVED",
                AccountBudgetProposalStatus::Cancelled => "CANCELLED",
                AccountBudgetProposalStatus::Rejected => "REJECTED",
            }
        }
    }
}
/// Message describing AccountBudgetProposal types.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountBudgetProposalTypeEnum {}
/// Nested message and enum types in `AccountBudgetProposalTypeEnum`.
pub mod account_budget_proposal_type_enum {
    /// The possible types of an AccountBudgetProposal.
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
    impl AccountBudgetProposalType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                AccountBudgetProposalType::Unspecified => "UNSPECIFIED",
                AccountBudgetProposalType::Unknown => "UNKNOWN",
                AccountBudgetProposalType::Create => "CREATE",
                AccountBudgetProposalType::Update => "UPDATE",
                AccountBudgetProposalType::End => "END",
                AccountBudgetProposalType::Remove => "REMOVE",
            }
        }
    }
}
/// Message describing AccountBudget statuses.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountBudgetStatusEnum {}
/// Nested message and enum types in `AccountBudgetStatusEnum`.
pub mod account_budget_status_enum {
    /// The possible statuses of an AccountBudget.
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
    impl AccountBudgetStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                AccountBudgetStatus::Unspecified => "UNSPECIFIED",
                AccountBudgetStatus::Unknown => "UNKNOWN",
                AccountBudgetStatus::Pending => "PENDING",
                AccountBudgetStatus::Approved => "APPROVED",
                AccountBudgetStatus::Cancelled => "CANCELLED",
            }
        }
    }
}
/// Container for enum describing possible statuses of an account link.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountLinkStatusEnum {}
/// Nested message and enum types in `AccountLinkStatusEnum`.
pub mod account_link_status_enum {
    /// Describes the possible statuses for a link between a Google Ads customer
    /// and another account.
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
    impl AccountLinkStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                AccountLinkStatus::Unspecified => "UNSPECIFIED",
                AccountLinkStatus::Unknown => "UNKNOWN",
                AccountLinkStatus::Enabled => "ENABLED",
                AccountLinkStatus::Removed => "REMOVED",
                AccountLinkStatus::Requested => "REQUESTED",
                AccountLinkStatus::PendingApproval => "PENDING_APPROVAL",
                AccountLinkStatus::Rejected => "REJECTED",
                AccountLinkStatus::Revoked => "REVOKED",
            }
        }
    }
}
/// Values for Ad Customizer placeholder fields.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdCustomizerPlaceholderFieldEnum {}
/// Nested message and enum types in `AdCustomizerPlaceholderFieldEnum`.
pub mod ad_customizer_placeholder_field_enum {
    /// Possible values for Ad Customizers placeholder fields.
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
    impl AdCustomizerPlaceholderField {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                AdCustomizerPlaceholderField::Unspecified => "UNSPECIFIED",
                AdCustomizerPlaceholderField::Unknown => "UNKNOWN",
                AdCustomizerPlaceholderField::Integer => "INTEGER",
                AdCustomizerPlaceholderField::Price => "PRICE",
                AdCustomizerPlaceholderField::Date => "DATE",
                AdCustomizerPlaceholderField::String => "STRING",
            }
        }
    }
}
/// Container for enum describing possible ad rotation modes of ads within an
/// ad group.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdGroupAdRotationModeEnum {}
/// Nested message and enum types in `AdGroupAdRotationModeEnum`.
pub mod ad_group_ad_rotation_mode_enum {
    /// The possible ad rotation modes of an ad group.
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
    impl AdGroupAdRotationMode {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                AdGroupAdRotationMode::Unspecified => "UNSPECIFIED",
                AdGroupAdRotationMode::Unknown => "UNKNOWN",
                AdGroupAdRotationMode::Optimize => "OPTIMIZE",
                AdGroupAdRotationMode::RotateForever => "ROTATE_FOREVER",
            }
        }
    }
}
/// Container for enum describing possible statuses of an AdGroupAd.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdGroupAdStatusEnum {}
/// Nested message and enum types in `AdGroupAdStatusEnum`.
pub mod ad_group_ad_status_enum {
    /// The possible statuses of an AdGroupAd.
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
    impl AdGroupAdStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                AdGroupAdStatus::Unspecified => "UNSPECIFIED",
                AdGroupAdStatus::Unknown => "UNKNOWN",
                AdGroupAdStatus::Enabled => "ENABLED",
                AdGroupAdStatus::Paused => "PAUSED",
                AdGroupAdStatus::Removed => "REMOVED",
            }
        }
    }
}
/// Container for enum describing possible AdGroupCriterion approval statuses.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdGroupCriterionApprovalStatusEnum {}
/// Nested message and enum types in `AdGroupCriterionApprovalStatusEnum`.
pub mod ad_group_criterion_approval_status_enum {
    /// Enumerates AdGroupCriterion approval statuses.
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
    impl AdGroupCriterionApprovalStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                AdGroupCriterionApprovalStatus::Unspecified => "UNSPECIFIED",
                AdGroupCriterionApprovalStatus::Unknown => "UNKNOWN",
                AdGroupCriterionApprovalStatus::Approved => "APPROVED",
                AdGroupCriterionApprovalStatus::Disapproved => "DISAPPROVED",
                AdGroupCriterionApprovalStatus::PendingReview => "PENDING_REVIEW",
                AdGroupCriterionApprovalStatus::UnderReview => "UNDER_REVIEW",
            }
        }
    }
}
/// Message describing AdGroupCriterion statuses.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdGroupCriterionStatusEnum {}
/// Nested message and enum types in `AdGroupCriterionStatusEnum`.
pub mod ad_group_criterion_status_enum {
    /// The possible statuses of an AdGroupCriterion.
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
    impl AdGroupCriterionStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                AdGroupCriterionStatus::Unspecified => "UNSPECIFIED",
                AdGroupCriterionStatus::Unknown => "UNKNOWN",
                AdGroupCriterionStatus::Enabled => "ENABLED",
                AdGroupCriterionStatus::Paused => "PAUSED",
                AdGroupCriterionStatus::Removed => "REMOVED",
            }
        }
    }
}
/// Container for enum describing possible statuses of an ad group.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdGroupStatusEnum {}
/// Nested message and enum types in `AdGroupStatusEnum`.
pub mod ad_group_status_enum {
    /// The possible statuses of an ad group.
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
    impl AdGroupStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                AdGroupStatus::Unspecified => "UNSPECIFIED",
                AdGroupStatus::Unknown => "UNKNOWN",
                AdGroupStatus::Enabled => "ENABLED",
                AdGroupStatus::Paused => "PAUSED",
                AdGroupStatus::Removed => "REMOVED",
            }
        }
    }
}
/// Defines types of an ad group, specific to a particular campaign channel
/// type. This type drives validations that restrict which entities can be
/// added to the ad group.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdGroupTypeEnum {}
/// Nested message and enum types in `AdGroupTypeEnum`.
pub mod ad_group_type_enum {
    /// Enum listing the possible types of an ad group.
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
        /// Ad group type for Smart campaigns.
        SmartCampaignAds = 18,
    }
    impl AdGroupType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                AdGroupType::Unspecified => "UNSPECIFIED",
                AdGroupType::Unknown => "UNKNOWN",
                AdGroupType::SearchStandard => "SEARCH_STANDARD",
                AdGroupType::DisplayStandard => "DISPLAY_STANDARD",
                AdGroupType::ShoppingProductAds => "SHOPPING_PRODUCT_ADS",
                AdGroupType::HotelAds => "HOTEL_ADS",
                AdGroupType::ShoppingSmartAds => "SHOPPING_SMART_ADS",
                AdGroupType::VideoBumper => "VIDEO_BUMPER",
                AdGroupType::VideoTrueViewInStream => "VIDEO_TRUE_VIEW_IN_STREAM",
                AdGroupType::VideoTrueViewInDisplay => "VIDEO_TRUE_VIEW_IN_DISPLAY",
                AdGroupType::VideoNonSkippableInStream => "VIDEO_NON_SKIPPABLE_IN_STREAM",
                AdGroupType::VideoOutstream => "VIDEO_OUTSTREAM",
                AdGroupType::SearchDynamicAds => "SEARCH_DYNAMIC_ADS",
                AdGroupType::ShoppingComparisonListingAds => {
                    "SHOPPING_COMPARISON_LISTING_ADS"
                }
                AdGroupType::PromotedHotelAds => "PROMOTED_HOTEL_ADS",
                AdGroupType::VideoResponsive => "VIDEO_RESPONSIVE",
                AdGroupType::VideoEfficientReach => "VIDEO_EFFICIENT_REACH",
                AdGroupType::SmartCampaignAds => "SMART_CAMPAIGN_ADS",
            }
        }
    }
}
/// Possible ad serving statuses of a campaign.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdServingOptimizationStatusEnum {}
/// Nested message and enum types in `AdServingOptimizationStatusEnum`.
pub mod ad_serving_optimization_status_enum {
    /// Enum describing possible serving statuses.
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
    impl AdServingOptimizationStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                AdServingOptimizationStatus::Unspecified => "UNSPECIFIED",
                AdServingOptimizationStatus::Unknown => "UNKNOWN",
                AdServingOptimizationStatus::Optimize => "OPTIMIZE",
                AdServingOptimizationStatus::ConversionOptimize => "CONVERSION_OPTIMIZE",
                AdServingOptimizationStatus::Rotate => "ROTATE",
                AdServingOptimizationStatus::RotateIndefinitely => "ROTATE_INDEFINITELY",
                AdServingOptimizationStatus::Unavailable => "UNAVAILABLE",
            }
        }
    }
}
/// Container for enum describing possible ad strengths.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdStrengthEnum {}
/// Nested message and enum types in `AdStrengthEnum`.
pub mod ad_strength_enum {
    /// Enum listing the possible ad strengths.
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
    impl AdStrength {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                AdStrength::Unspecified => "UNSPECIFIED",
                AdStrength::Unknown => "UNKNOWN",
                AdStrength::Pending => "PENDING",
                AdStrength::NoAds => "NO_ADS",
                AdStrength::Poor => "POOR",
                AdStrength::Average => "AVERAGE",
                AdStrength::Good => "GOOD",
                AdStrength::Excellent => "EXCELLENT",
            }
        }
    }
}
/// Container for enum describing possible types of an ad.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdTypeEnum {}
/// Nested message and enum types in `AdTypeEnum`.
pub mod ad_type_enum {
    /// The possible types of an ad.
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
        /// Video TrueView in-stream ad.
        VideoTrueviewInStreamAd = 29,
        /// Video responsive ad.
        VideoResponsiveAd = 30,
        /// Smart campaign ad.
        SmartCampaignAd = 31,
        /// Call ad.
        CallAd = 32,
        /// Universal app pre-registration ad.
        AppPreRegistrationAd = 33,
        /// In-feed video ad.
        InFeedVideoAd = 34,
        /// Discovery multi asset ad.
        DiscoveryMultiAssetAd = 35,
        /// Discovery carousel ad.
        DiscoveryCarouselAd = 36,
    }
    impl AdType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                AdType::Unspecified => "UNSPECIFIED",
                AdType::Unknown => "UNKNOWN",
                AdType::TextAd => "TEXT_AD",
                AdType::ExpandedTextAd => "EXPANDED_TEXT_AD",
                AdType::ExpandedDynamicSearchAd => "EXPANDED_DYNAMIC_SEARCH_AD",
                AdType::HotelAd => "HOTEL_AD",
                AdType::ShoppingSmartAd => "SHOPPING_SMART_AD",
                AdType::ShoppingProductAd => "SHOPPING_PRODUCT_AD",
                AdType::VideoAd => "VIDEO_AD",
                AdType::GmailAd => "GMAIL_AD",
                AdType::ImageAd => "IMAGE_AD",
                AdType::ResponsiveSearchAd => "RESPONSIVE_SEARCH_AD",
                AdType::LegacyResponsiveDisplayAd => "LEGACY_RESPONSIVE_DISPLAY_AD",
                AdType::AppAd => "APP_AD",
                AdType::LegacyAppInstallAd => "LEGACY_APP_INSTALL_AD",
                AdType::ResponsiveDisplayAd => "RESPONSIVE_DISPLAY_AD",
                AdType::LocalAd => "LOCAL_AD",
                AdType::Html5UploadAd => "HTML5_UPLOAD_AD",
                AdType::DynamicHtml5Ad => "DYNAMIC_HTML5_AD",
                AdType::AppEngagementAd => "APP_ENGAGEMENT_AD",
                AdType::ShoppingComparisonListingAd => "SHOPPING_COMPARISON_LISTING_AD",
                AdType::VideoBumperAd => "VIDEO_BUMPER_AD",
                AdType::VideoNonSkippableInStreamAd => "VIDEO_NON_SKIPPABLE_IN_STREAM_AD",
                AdType::VideoOutstreamAd => "VIDEO_OUTSTREAM_AD",
                AdType::VideoTrueviewInStreamAd => "VIDEO_TRUEVIEW_IN_STREAM_AD",
                AdType::VideoResponsiveAd => "VIDEO_RESPONSIVE_AD",
                AdType::SmartCampaignAd => "SMART_CAMPAIGN_AD",
                AdType::CallAd => "CALL_AD",
                AdType::AppPreRegistrationAd => "APP_PRE_REGISTRATION_AD",
                AdType::InFeedVideoAd => "IN_FEED_VIDEO_AD",
                AdType::DiscoveryMultiAssetAd => "DISCOVERY_MULTI_ASSET_AD",
                AdType::DiscoveryCarouselAd => "DISCOVERY_CAROUSEL_AD",
            }
        }
    }
}
/// Container for enum describing possible values for a relationship type for
/// an affiliate location feed.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AffiliateLocationFeedRelationshipTypeEnum {}
/// Nested message and enum types in `AffiliateLocationFeedRelationshipTypeEnum`.
pub mod affiliate_location_feed_relationship_type_enum {
    /// Possible values for a relationship type for an affiliate location feed.
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
    pub enum AffiliateLocationFeedRelationshipType {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// General retailer relationship.
        GeneralRetailer = 2,
    }
    impl AffiliateLocationFeedRelationshipType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                AffiliateLocationFeedRelationshipType::Unspecified => "UNSPECIFIED",
                AffiliateLocationFeedRelationshipType::Unknown => "UNKNOWN",
                AffiliateLocationFeedRelationshipType::GeneralRetailer => {
                    "GENERAL_RETAILER"
                }
            }
        }
    }
}
/// Values for Affiliate Location placeholder fields.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AffiliateLocationPlaceholderFieldEnum {}
/// Nested message and enum types in `AffiliateLocationPlaceholderFieldEnum`.
pub mod affiliate_location_placeholder_field_enum {
    /// Possible values for Affiliate Location placeholder fields.
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
    impl AffiliateLocationPlaceholderField {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                AffiliateLocationPlaceholderField::Unspecified => "UNSPECIFIED",
                AffiliateLocationPlaceholderField::Unknown => "UNKNOWN",
                AffiliateLocationPlaceholderField::BusinessName => "BUSINESS_NAME",
                AffiliateLocationPlaceholderField::AddressLine1 => "ADDRESS_LINE_1",
                AffiliateLocationPlaceholderField::AddressLine2 => "ADDRESS_LINE_2",
                AffiliateLocationPlaceholderField::City => "CITY",
                AffiliateLocationPlaceholderField::Province => "PROVINCE",
                AffiliateLocationPlaceholderField::PostalCode => "POSTAL_CODE",
                AffiliateLocationPlaceholderField::CountryCode => "COUNTRY_CODE",
                AffiliateLocationPlaceholderField::PhoneNumber => "PHONE_NUMBER",
                AffiliateLocationPlaceholderField::LanguageCode => "LANGUAGE_CODE",
                AffiliateLocationPlaceholderField::ChainId => "CHAIN_ID",
                AffiliateLocationPlaceholderField::ChainName => "CHAIN_NAME",
            }
        }
    }
}
/// The application store that distributes mobile applications.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AppCampaignAppStoreEnum {}
/// Nested message and enum types in `AppCampaignAppStoreEnum`.
pub mod app_campaign_app_store_enum {
    /// Enum describing app campaign app store.
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
    impl AppCampaignAppStore {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                AppCampaignAppStore::Unspecified => "UNSPECIFIED",
                AppCampaignAppStore::Unknown => "UNKNOWN",
                AppCampaignAppStore::AppleAppStore => "APPLE_APP_STORE",
                AppCampaignAppStore::GoogleAppStore => "GOOGLE_APP_STORE",
            }
        }
    }
}
/// Container for enum describing goal towards which the bidding strategy of an
/// app campaign should optimize for.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AppCampaignBiddingStrategyGoalTypeEnum {}
/// Nested message and enum types in `AppCampaignBiddingStrategyGoalTypeEnum`.
pub mod app_campaign_bidding_strategy_goal_type_enum {
    /// Goal type of App campaign BiddingStrategy.
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
        /// the long term conversions haven't happened yet.
        OptimizeInAppConversionsTargetConversionCost = 4,
        /// Aim to maximize all conversions' value, for example, install + selected
        /// in-app conversions while achieving or exceeding target return on
        /// advertising spend.
        OptimizeReturnOnAdvertisingSpend = 5,
        /// Aim to maximize the pre-registration of the app.
        OptimizePreRegistrationConversionVolume = 6,
        /// Aim to maximize installation of the app without target cost-per-install.
        OptimizeInstallsWithoutTargetInstallCost = 7,
    }
    impl AppCampaignBiddingStrategyGoalType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                AppCampaignBiddingStrategyGoalType::Unspecified => "UNSPECIFIED",
                AppCampaignBiddingStrategyGoalType::Unknown => "UNKNOWN",
                AppCampaignBiddingStrategyGoalType::OptimizeInstallsTargetInstallCost => {
                    "OPTIMIZE_INSTALLS_TARGET_INSTALL_COST"
                }
                AppCampaignBiddingStrategyGoalType::OptimizeInAppConversionsTargetInstallCost => {
                    "OPTIMIZE_IN_APP_CONVERSIONS_TARGET_INSTALL_COST"
                }
                AppCampaignBiddingStrategyGoalType::OptimizeInAppConversionsTargetConversionCost => {
                    "OPTIMIZE_IN_APP_CONVERSIONS_TARGET_CONVERSION_COST"
                }
                AppCampaignBiddingStrategyGoalType::OptimizeReturnOnAdvertisingSpend => {
                    "OPTIMIZE_RETURN_ON_ADVERTISING_SPEND"
                }
                AppCampaignBiddingStrategyGoalType::OptimizePreRegistrationConversionVolume => {
                    "OPTIMIZE_PRE_REGISTRATION_CONVERSION_VOLUME"
                }
                AppCampaignBiddingStrategyGoalType::OptimizeInstallsWithoutTargetInstallCost => {
                    "OPTIMIZE_INSTALLS_WITHOUT_TARGET_INSTALL_COST"
                }
            }
        }
    }
}
/// Values for App placeholder fields.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AppPlaceholderFieldEnum {}
/// Nested message and enum types in `AppPlaceholderFieldEnum`.
pub mod app_placeholder_field_enum {
    /// Possible values for App placeholder fields.
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
    impl AppPlaceholderField {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                AppPlaceholderField::Unspecified => "UNSPECIFIED",
                AppPlaceholderField::Unknown => "UNKNOWN",
                AppPlaceholderField::Store => "STORE",
                AppPlaceholderField::Id => "ID",
                AppPlaceholderField::LinkText => "LINK_TEXT",
                AppPlaceholderField::Url => "URL",
                AppPlaceholderField::FinalUrls => "FINAL_URLS",
                AppPlaceholderField::FinalMobileUrls => "FINAL_MOBILE_URLS",
                AppPlaceholderField::TrackingUrl => "TRACKING_URL",
                AppPlaceholderField::FinalUrlSuffix => "FINAL_URL_SUFFIX",
            }
        }
    }
}
/// Container for enum describing the possible placements of an asset.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetFieldTypeEnum {}
/// Nested message and enum types in `AssetFieldTypeEnum`.
pub mod asset_field_type_enum {
    /// Enum describing the possible placements of an asset.
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
        /// The asset is linked for use as a Mobile App extension.
        MobileApp = 14,
        /// The asset is linked for use as a Hotel Callout extension.
        HotelCallout = 15,
        /// The asset is linked for use as a Call extension.
        Call = 16,
        /// The asset is linked for use as a Price extension.
        Price = 24,
        /// The asset is linked for use as a long headline.
        LongHeadline = 17,
        /// The asset is linked for use as a business name.
        BusinessName = 18,
        /// The asset is linked for use as a square marketing image.
        SquareMarketingImage = 19,
        /// The asset is linked for use as a portrait marketing image.
        PortraitMarketingImage = 20,
        /// The asset is linked for use as a logo.
        Logo = 21,
        /// The asset is linked for use as a landscape logo.
        LandscapeLogo = 22,
        /// The asset is linked for use as a non YouTube logo.
        Video = 23,
        /// The asset is linked for use to select a call-to-action.
        CallToActionSelection = 25,
    }
    impl AssetFieldType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                AssetFieldType::Unspecified => "UNSPECIFIED",
                AssetFieldType::Unknown => "UNKNOWN",
                AssetFieldType::Headline => "HEADLINE",
                AssetFieldType::Description => "DESCRIPTION",
                AssetFieldType::MandatoryAdText => "MANDATORY_AD_TEXT",
                AssetFieldType::MarketingImage => "MARKETING_IMAGE",
                AssetFieldType::MediaBundle => "MEDIA_BUNDLE",
                AssetFieldType::YoutubeVideo => "YOUTUBE_VIDEO",
                AssetFieldType::BookOnGoogle => "BOOK_ON_GOOGLE",
                AssetFieldType::LeadForm => "LEAD_FORM",
                AssetFieldType::Promotion => "PROMOTION",
                AssetFieldType::Callout => "CALLOUT",
                AssetFieldType::StructuredSnippet => "STRUCTURED_SNIPPET",
                AssetFieldType::Sitelink => "SITELINK",
                AssetFieldType::MobileApp => "MOBILE_APP",
                AssetFieldType::HotelCallout => "HOTEL_CALLOUT",
                AssetFieldType::Call => "CALL",
                AssetFieldType::Price => "PRICE",
                AssetFieldType::LongHeadline => "LONG_HEADLINE",
                AssetFieldType::BusinessName => "BUSINESS_NAME",
                AssetFieldType::SquareMarketingImage => "SQUARE_MARKETING_IMAGE",
                AssetFieldType::PortraitMarketingImage => "PORTRAIT_MARKETING_IMAGE",
                AssetFieldType::Logo => "LOGO",
                AssetFieldType::LandscapeLogo => "LANDSCAPE_LOGO",
                AssetFieldType::Video => "VIDEO",
                AssetFieldType::CallToActionSelection => "CALL_TO_ACTION_SELECTION",
            }
        }
    }
}
/// Container for enum describing possible statuses of an asset group.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetGroupStatusEnum {}
/// Nested message and enum types in `AssetGroupStatusEnum`.
pub mod asset_group_status_enum {
    /// The possible statuses of an asset group.
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
    pub enum AssetGroupStatus {
        /// The status has not been specified.
        Unspecified = 0,
        /// The received value is not known in this version.
        Unknown = 1,
        /// The asset group is enabled.
        Enabled = 2,
        /// The asset group is paused.
        Paused = 3,
        /// The asset group is removed.
        Removed = 4,
    }
    impl AssetGroupStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                AssetGroupStatus::Unspecified => "UNSPECIFIED",
                AssetGroupStatus::Unknown => "UNKNOWN",
                AssetGroupStatus::Enabled => "ENABLED",
                AssetGroupStatus::Paused => "PAUSED",
                AssetGroupStatus::Removed => "REMOVED",
            }
        }
    }
}
/// Container for enum describing possible statuses of an asset link.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetLinkStatusEnum {}
/// Nested message and enum types in `AssetLinkStatusEnum`.
pub mod asset_link_status_enum {
    /// Enum describing statuses of an asset link.
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
    impl AssetLinkStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                AssetLinkStatus::Unspecified => "UNSPECIFIED",
                AssetLinkStatus::Unknown => "UNKNOWN",
                AssetLinkStatus::Enabled => "ENABLED",
                AssetLinkStatus::Removed => "REMOVED",
                AssetLinkStatus::Paused => "PAUSED",
            }
        }
    }
}
/// Container for enum describing possible statuses of an asset set asset.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetSetAssetStatusEnum {}
/// Nested message and enum types in `AssetSetAssetStatusEnum`.
pub mod asset_set_asset_status_enum {
    /// The possible statuses of an asset set asset.
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
    pub enum AssetSetAssetStatus {
        /// The status has not been specified.
        Unspecified = 0,
        /// The received value is not known in this version.
        /// This is a response-only value.
        Unknown = 1,
        /// The asset set asset is enabled.
        Enabled = 2,
        /// The asset set asset is removed.
        Removed = 3,
    }
    impl AssetSetAssetStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                AssetSetAssetStatus::Unspecified => "UNSPECIFIED",
                AssetSetAssetStatus::Unknown => "UNKNOWN",
                AssetSetAssetStatus::Enabled => "ENABLED",
                AssetSetAssetStatus::Removed => "REMOVED",
            }
        }
    }
}
/// Container for enum describing possible statuses of the linkage between asset
/// set and its container.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetSetLinkStatusEnum {}
/// Nested message and enum types in `AssetSetLinkStatusEnum`.
pub mod asset_set_link_status_enum {
    /// The possible statuses of the linkage between asset set and its container.
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
    pub enum AssetSetLinkStatus {
        /// The status has not been specified.
        Unspecified = 0,
        /// The received value is not known in this version.
        /// This is a response-only value.
        Unknown = 1,
        /// The linkage between asset set and its container is enabled.
        Enabled = 2,
        /// The linkage between asset set and its container is removed.
        Removed = 3,
    }
    impl AssetSetLinkStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                AssetSetLinkStatus::Unspecified => "UNSPECIFIED",
                AssetSetLinkStatus::Unknown => "UNKNOWN",
                AssetSetLinkStatus::Enabled => "ENABLED",
                AssetSetLinkStatus::Removed => "REMOVED",
            }
        }
    }
}
/// Container for enum describing possible statuses of an asset set.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetSetStatusEnum {}
/// Nested message and enum types in `AssetSetStatusEnum`.
pub mod asset_set_status_enum {
    /// The possible statuses of an asset set.
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
    pub enum AssetSetStatus {
        /// The status has not been specified.
        Unspecified = 0,
        /// The received value is not known in this version.
        /// This is a response-only value.
        Unknown = 1,
        /// The asset set is enabled.
        Enabled = 2,
        /// The asset set is removed.
        Removed = 3,
    }
    impl AssetSetStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                AssetSetStatus::Unspecified => "UNSPECIFIED",
                AssetSetStatus::Unknown => "UNKNOWN",
                AssetSetStatus::Enabled => "ENABLED",
                AssetSetStatus::Removed => "REMOVED",
            }
        }
    }
}
/// Container for enum describing possible types of an asset set.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetSetTypeEnum {}
/// Nested message and enum types in `AssetSetTypeEnum`.
pub mod asset_set_type_enum {
    /// Possible types of an asset set.
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
    pub enum AssetSetType {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Page asset set.
        PageFeed = 2,
        /// Dynamic education asset set.
        DynamicEducation = 3,
        /// Google Merchant Center asset set.
        MerchantCenterFeed = 4,
        /// Dynamic real estate asset set.
        DynamicRealEstate = 5,
        /// Dynamic custom asset set.
        DynamicCustom = 6,
        /// Dynamic hotels and rentals asset set.
        DynamicHotelsAndRentals = 7,
        /// Dynamic flights asset set.
        DynamicFlights = 8,
        /// Dynamic travel asset set.
        DynamicTravel = 9,
        /// Dynamic local asset set.
        DynamicLocal = 10,
        /// Dynamic jobs asset set.
        DynamicJobs = 11,
    }
    impl AssetSetType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                AssetSetType::Unspecified => "UNSPECIFIED",
                AssetSetType::Unknown => "UNKNOWN",
                AssetSetType::PageFeed => "PAGE_FEED",
                AssetSetType::DynamicEducation => "DYNAMIC_EDUCATION",
                AssetSetType::MerchantCenterFeed => "MERCHANT_CENTER_FEED",
                AssetSetType::DynamicRealEstate => "DYNAMIC_REAL_ESTATE",
                AssetSetType::DynamicCustom => "DYNAMIC_CUSTOM",
                AssetSetType::DynamicHotelsAndRentals => "DYNAMIC_HOTELS_AND_RENTALS",
                AssetSetType::DynamicFlights => "DYNAMIC_FLIGHTS",
                AssetSetType::DynamicTravel => "DYNAMIC_TRAVEL",
                AssetSetType::DynamicLocal => "DYNAMIC_LOCAL",
                AssetSetType::DynamicJobs => "DYNAMIC_JOBS",
            }
        }
    }
}
/// Source of the asset or asset link for who generated the entity.
/// For example, advertiser or automatically created.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetSourceEnum {}
/// Nested message and enum types in `AssetSourceEnum`.
pub mod asset_source_enum {
    /// Enum describing possible source of asset.
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
    pub enum AssetSource {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// The asset or asset link is provided by advertiser.
        Advertiser = 2,
        /// The asset or asset link is generated by Google.
        AutomaticallyCreated = 3,
    }
    impl AssetSource {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                AssetSource::Unspecified => "UNSPECIFIED",
                AssetSource::Unknown => "UNKNOWN",
                AssetSource::Advertiser => "ADVERTISER",
                AssetSource::AutomaticallyCreated => "AUTOMATICALLY_CREATED",
            }
        }
    }
}
/// Container for enum describing the types of asset.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetTypeEnum {}
/// Nested message and enum types in `AssetTypeEnum`.
pub mod asset_type_enum {
    /// Enum describing possible types of asset.
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
        /// Page Feed asset.
        PageFeed = 12,
        /// Dynamic Education asset.
        DynamicEducation = 13,
        /// Mobile app asset.
        MobileApp = 14,
        /// Hotel callout asset.
        HotelCallout = 15,
        /// Call asset.
        Call = 16,
        /// Price asset.
        Price = 17,
        /// Call to action asset.
        CallToAction = 18,
        /// Dynamic real estate asset.
        DynamicRealEstate = 19,
        /// Dynamic custom asset.
        DynamicCustom = 20,
        /// Dynamic hotels and rentals asset.
        DynamicHotelsAndRentals = 21,
        /// Dynamic flights asset.
        DynamicFlights = 22,
        /// Discovery Carousel Card asset.
        DiscoveryCarouselCard = 23,
        /// Dynamic travel asset.
        DynamicTravel = 24,
        /// Dynamic local asset.
        DynamicLocal = 25,
        /// Dynamic jobs asset.
        DynamicJobs = 26,
    }
    impl AssetType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                AssetType::Unspecified => "UNSPECIFIED",
                AssetType::Unknown => "UNKNOWN",
                AssetType::YoutubeVideo => "YOUTUBE_VIDEO",
                AssetType::MediaBundle => "MEDIA_BUNDLE",
                AssetType::Image => "IMAGE",
                AssetType::Text => "TEXT",
                AssetType::LeadForm => "LEAD_FORM",
                AssetType::BookOnGoogle => "BOOK_ON_GOOGLE",
                AssetType::Promotion => "PROMOTION",
                AssetType::Callout => "CALLOUT",
                AssetType::StructuredSnippet => "STRUCTURED_SNIPPET",
                AssetType::Sitelink => "SITELINK",
                AssetType::PageFeed => "PAGE_FEED",
                AssetType::DynamicEducation => "DYNAMIC_EDUCATION",
                AssetType::MobileApp => "MOBILE_APP",
                AssetType::HotelCallout => "HOTEL_CALLOUT",
                AssetType::Call => "CALL",
                AssetType::Price => "PRICE",
                AssetType::CallToAction => "CALL_TO_ACTION",
                AssetType::DynamicRealEstate => "DYNAMIC_REAL_ESTATE",
                AssetType::DynamicCustom => "DYNAMIC_CUSTOM",
                AssetType::DynamicHotelsAndRentals => "DYNAMIC_HOTELS_AND_RENTALS",
                AssetType::DynamicFlights => "DYNAMIC_FLIGHTS",
                AssetType::DiscoveryCarouselCard => "DISCOVERY_CAROUSEL_CARD",
                AssetType::DynamicTravel => "DYNAMIC_TRAVEL",
                AssetType::DynamicLocal => "DYNAMIC_LOCAL",
                AssetType::DynamicJobs => "DYNAMIC_JOBS",
            }
        }
    }
}
/// Container for enum describing the experiment async action status.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AsyncActionStatusEnum {}
/// Nested message and enum types in `AsyncActionStatusEnum`.
pub mod async_action_status_enum {
    /// The async action status of the experiment.
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
    pub enum AsyncActionStatus {
        /// Not specified.
        Unspecified = 0,
        /// The value is unknown in this version.
        Unknown = 1,
        /// Action has not started.
        NotStarted = 2,
        /// Action is in progress.
        InProgress = 3,
        /// Action has completed successfully.
        Completed = 4,
        /// Action has failed.
        Failed = 5,
        /// Action has completed successfully with warnings.
        CompletedWithWarning = 6,
    }
    impl AsyncActionStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                AsyncActionStatus::Unspecified => "UNSPECIFIED",
                AsyncActionStatus::Unknown => "UNKNOWN",
                AsyncActionStatus::NotStarted => "NOT_STARTED",
                AsyncActionStatus::InProgress => "IN_PROGRESS",
                AsyncActionStatus::Completed => "COMPLETED",
                AsyncActionStatus::Failed => "FAILED",
                AsyncActionStatus::CompletedWithWarning => "COMPLETED_WITH_WARNING",
            }
        }
    }
}
/// Container for enum representing the attribution model that describes how to
/// distribute credit for a particular conversion across potentially many prior
/// interactions.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AttributionModelEnum {}
/// Nested message and enum types in `AttributionModelEnum`.
pub mod attribution_model_enum {
    /// The attribution model that describes how to distribute credit for a
    /// particular conversion across potentially many prior interactions.
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
    impl AttributionModel {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                AttributionModel::Unspecified => "UNSPECIFIED",
                AttributionModel::Unknown => "UNKNOWN",
                AttributionModel::External => "EXTERNAL",
                AttributionModel::GoogleAdsLastClick => "GOOGLE_ADS_LAST_CLICK",
                AttributionModel::GoogleSearchAttributionFirstClick => {
                    "GOOGLE_SEARCH_ATTRIBUTION_FIRST_CLICK"
                }
                AttributionModel::GoogleSearchAttributionLinear => {
                    "GOOGLE_SEARCH_ATTRIBUTION_LINEAR"
                }
                AttributionModel::GoogleSearchAttributionTimeDecay => {
                    "GOOGLE_SEARCH_ATTRIBUTION_TIME_DECAY"
                }
                AttributionModel::GoogleSearchAttributionPositionBased => {
                    "GOOGLE_SEARCH_ATTRIBUTION_POSITION_BASED"
                }
                AttributionModel::GoogleSearchAttributionDataDriven => {
                    "GOOGLE_SEARCH_ATTRIBUTION_DATA_DRIVEN"
                }
            }
        }
    }
}
/// Container for enum describing audience insights dimensions.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AudienceInsightsDimensionEnum {}
/// Nested message and enum types in `AudienceInsightsDimensionEnum`.
pub mod audience_insights_dimension_enum {
    /// Possible audience dimensions for use in generating insights.
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
    pub enum AudienceInsightsDimension {
        /// Not specified.
        Unspecified = 0,
        /// The value is unknown in this version.
        Unknown = 1,
        /// A Product & Service category.
        Category = 2,
        /// A Knowledge Graph entity.
        KnowledgeGraph = 3,
        /// A country, represented by a geo target.
        GeoTargetCountry = 4,
        /// A geographic location within a country.
        SubCountryLocation = 5,
        /// A YouTube channel.
        YoutubeChannel = 6,
        /// A YouTube Dynamic Lineup.
        YoutubeDynamicLineup = 7,
        /// An Affinity UserInterest.
        AffinityUserInterest = 8,
        /// An In-Market UserInterest.
        InMarketUserInterest = 9,
        /// A Parental Status value (parent, or not a parent).
        ParentalStatus = 10,
        /// A household income percentile range.
        IncomeRange = 11,
        /// An age range.
        AgeRange = 12,
        /// A gender.
        Gender = 13,
    }
    impl AudienceInsightsDimension {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                AudienceInsightsDimension::Unspecified => "UNSPECIFIED",
                AudienceInsightsDimension::Unknown => "UNKNOWN",
                AudienceInsightsDimension::Category => "CATEGORY",
                AudienceInsightsDimension::KnowledgeGraph => "KNOWLEDGE_GRAPH",
                AudienceInsightsDimension::GeoTargetCountry => "GEO_TARGET_COUNTRY",
                AudienceInsightsDimension::SubCountryLocation => "SUB_COUNTRY_LOCATION",
                AudienceInsightsDimension::YoutubeChannel => "YOUTUBE_CHANNEL",
                AudienceInsightsDimension::YoutubeDynamicLineup => {
                    "YOUTUBE_DYNAMIC_LINEUP"
                }
                AudienceInsightsDimension::AffinityUserInterest => {
                    "AFFINITY_USER_INTEREST"
                }
                AudienceInsightsDimension::InMarketUserInterest => {
                    "IN_MARKET_USER_INTEREST"
                }
                AudienceInsightsDimension::ParentalStatus => "PARENTAL_STATUS",
                AudienceInsightsDimension::IncomeRange => "INCOME_RANGE",
                AudienceInsightsDimension::AgeRange => "AGE_RANGE",
                AudienceInsightsDimension::Gender => "GENDER",
            }
        }
    }
}
/// The status of audience.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AudienceStatusEnum {}
/// Nested message and enum types in `AudienceStatusEnum`.
pub mod audience_status_enum {
    /// Enum containing possible audience status types.
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
    pub enum AudienceStatus {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Enabled status - audience is enabled and can be targeted.
        Enabled = 2,
        /// Removed status - audience is removed and cannot be used for
        /// targeting.
        Removed = 3,
    }
    impl AudienceStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                AudienceStatus::Unspecified => "UNSPECIFIED",
                AudienceStatus::Unknown => "UNKNOWN",
                AudienceStatus::Enabled => "ENABLED",
                AudienceStatus::Removed => "REMOVED",
            }
        }
    }
}
/// Container for enum describing possible batch job statuses.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchJobStatusEnum {}
/// Nested message and enum types in `BatchJobStatusEnum`.
pub mod batch_job_status_enum {
    /// The batch job statuses.
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
    impl BatchJobStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                BatchJobStatus::Unspecified => "UNSPECIFIED",
                BatchJobStatus::Unknown => "UNKNOWN",
                BatchJobStatus::Pending => "PENDING",
                BatchJobStatus::Running => "RUNNING",
                BatchJobStatus::Done => "DONE",
            }
        }
    }
}
/// Container for enum describing possible bid modifier sources.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BidModifierSourceEnum {}
/// Nested message and enum types in `BidModifierSourceEnum`.
pub mod bid_modifier_source_enum {
    /// Enum describing possible bid modifier sources.
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
    impl BidModifierSource {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                BidModifierSource::Unspecified => "UNSPECIFIED",
                BidModifierSource::Unknown => "UNKNOWN",
                BidModifierSource::Campaign => "CAMPAIGN",
                BidModifierSource::AdGroup => "AD_GROUP",
            }
        }
    }
}
/// Container for enum describing possible bidding sources.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BiddingSourceEnum {}
/// Nested message and enum types in `BiddingSourceEnum`.
pub mod bidding_source_enum {
    /// Indicates where a bid or target is defined. For example, an ad group
    /// criterion may define a cpc bid directly, or it can inherit its cpc bid from
    /// the ad group.
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
    impl BiddingSource {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                BiddingSource::Unspecified => "UNSPECIFIED",
                BiddingSource::Unknown => "UNKNOWN",
                BiddingSource::CampaignBiddingStrategy => "CAMPAIGN_BIDDING_STRATEGY",
                BiddingSource::AdGroup => "AD_GROUP",
                BiddingSource::AdGroupCriterion => "AD_GROUP_CRITERION",
            }
        }
    }
}
/// Message describing BiddingStrategy statuses.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BiddingStrategyStatusEnum {}
/// Nested message and enum types in `BiddingStrategyStatusEnum`.
pub mod bidding_strategy_status_enum {
    /// The possible statuses of a BiddingStrategy.
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
    impl BiddingStrategyStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                BiddingStrategyStatus::Unspecified => "UNSPECIFIED",
                BiddingStrategyStatus::Unknown => "UNKNOWN",
                BiddingStrategyStatus::Enabled => "ENABLED",
                BiddingStrategyStatus::Removed => "REMOVED",
            }
        }
    }
}
/// Message describing BiddingStrategy system statuses.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BiddingStrategySystemStatusEnum {}
/// Nested message and enum types in `BiddingStrategySystemStatusEnum`.
pub mod bidding_strategy_system_status_enum {
    /// The possible system statuses of a BiddingStrategy.
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
    pub enum BiddingStrategySystemStatus {
        /// Signals that an unexpected error occurred, for example, no bidding
        /// strategy type was found, or no status information was found.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// The bid strategy is active, and AdWords cannot find any specific issues
        /// with the strategy.
        Enabled = 2,
        /// The bid strategy is learning because it has been recently created or
        /// recently reactivated.
        LearningNew = 3,
        /// The bid strategy is learning because of a recent setting change.
        LearningSettingChange = 4,
        /// The bid strategy is learning because of a recent budget change.
        LearningBudgetChange = 5,
        /// The bid strategy is learning because of recent change in number of
        /// campaigns, ad groups or keywords attached to it.
        LearningCompositionChange = 6,
        /// The bid strategy depends on conversion reporting and the customer
        /// recently modified conversion types that were relevant to the
        /// bid strategy.
        LearningConversionTypeChange = 7,
        /// The bid strategy depends on conversion reporting and the customer
        /// recently changed their conversion settings.
        LearningConversionSettingChange = 8,
        /// The bid strategy is limited by its bid ceiling.
        LimitedByCpcBidCeiling = 9,
        /// The bid strategy is limited by its bid floor.
        LimitedByCpcBidFloor = 10,
        /// The bid strategy is limited because there was not enough conversion
        /// traffic over the past weeks.
        LimitedByData = 11,
        /// A significant fraction of keywords in this bid strategy are limited by
        /// budget.
        LimitedByBudget = 12,
        /// The bid strategy cannot reach its target spend because its spend has
        /// been de-prioritized.
        LimitedByLowPrioritySpend = 13,
        /// A significant fraction of keywords in this bid strategy have a low
        /// Quality Score.
        LimitedByLowQuality = 14,
        /// The bid strategy cannot fully spend its budget because of narrow
        /// targeting.
        LimitedByInventory = 15,
        /// Missing conversion tracking (no pings present) and/or remarketing lists
        /// for SSC.
        MisconfiguredZeroEligibility = 16,
        /// The bid strategy depends on conversion reporting and the customer is
        /// lacking conversion types that might be reported against this strategy.
        MisconfiguredConversionTypes = 17,
        /// The bid strategy depends on conversion reporting and the customer's
        /// conversion settings are misconfigured.
        MisconfiguredConversionSettings = 18,
        /// There are campaigns outside the bid strategy that share budgets with
        /// campaigns included in the strategy.
        MisconfiguredSharedBudget = 19,
        /// The campaign has an invalid strategy type and is not serving.
        MisconfiguredStrategyType = 20,
        /// The bid strategy is not active. Either there are no active campaigns,
        /// ad groups or keywords attached to the bid strategy. Or there are no
        /// active budgets connected to the bid strategy.
        Paused = 21,
        /// This bid strategy currently does not support status reporting.
        Unavailable = 22,
        /// There were multiple LEARNING_* system statuses for this bid strategy
        /// during the time in question.
        MultipleLearning = 23,
        /// There were multiple LIMITED_* system statuses for this bid strategy
        /// during the time in question.
        MultipleLimited = 24,
        /// There were multiple MISCONFIGURED_* system statuses for this bid strategy
        /// during the time in question.
        MultipleMisconfigured = 25,
        /// There were multiple system statuses for this bid strategy during the
        /// time in question.
        Multiple = 26,
    }
    impl BiddingStrategySystemStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                BiddingStrategySystemStatus::Unspecified => "UNSPECIFIED",
                BiddingStrategySystemStatus::Unknown => "UNKNOWN",
                BiddingStrategySystemStatus::Enabled => "ENABLED",
                BiddingStrategySystemStatus::LearningNew => "LEARNING_NEW",
                BiddingStrategySystemStatus::LearningSettingChange => {
                    "LEARNING_SETTING_CHANGE"
                }
                BiddingStrategySystemStatus::LearningBudgetChange => {
                    "LEARNING_BUDGET_CHANGE"
                }
                BiddingStrategySystemStatus::LearningCompositionChange => {
                    "LEARNING_COMPOSITION_CHANGE"
                }
                BiddingStrategySystemStatus::LearningConversionTypeChange => {
                    "LEARNING_CONVERSION_TYPE_CHANGE"
                }
                BiddingStrategySystemStatus::LearningConversionSettingChange => {
                    "LEARNING_CONVERSION_SETTING_CHANGE"
                }
                BiddingStrategySystemStatus::LimitedByCpcBidCeiling => {
                    "LIMITED_BY_CPC_BID_CEILING"
                }
                BiddingStrategySystemStatus::LimitedByCpcBidFloor => {
                    "LIMITED_BY_CPC_BID_FLOOR"
                }
                BiddingStrategySystemStatus::LimitedByData => "LIMITED_BY_DATA",
                BiddingStrategySystemStatus::LimitedByBudget => "LIMITED_BY_BUDGET",
                BiddingStrategySystemStatus::LimitedByLowPrioritySpend => {
                    "LIMITED_BY_LOW_PRIORITY_SPEND"
                }
                BiddingStrategySystemStatus::LimitedByLowQuality => {
                    "LIMITED_BY_LOW_QUALITY"
                }
                BiddingStrategySystemStatus::LimitedByInventory => "LIMITED_BY_INVENTORY",
                BiddingStrategySystemStatus::MisconfiguredZeroEligibility => {
                    "MISCONFIGURED_ZERO_ELIGIBILITY"
                }
                BiddingStrategySystemStatus::MisconfiguredConversionTypes => {
                    "MISCONFIGURED_CONVERSION_TYPES"
                }
                BiddingStrategySystemStatus::MisconfiguredConversionSettings => {
                    "MISCONFIGURED_CONVERSION_SETTINGS"
                }
                BiddingStrategySystemStatus::MisconfiguredSharedBudget => {
                    "MISCONFIGURED_SHARED_BUDGET"
                }
                BiddingStrategySystemStatus::MisconfiguredStrategyType => {
                    "MISCONFIGURED_STRATEGY_TYPE"
                }
                BiddingStrategySystemStatus::Paused => "PAUSED",
                BiddingStrategySystemStatus::Unavailable => "UNAVAILABLE",
                BiddingStrategySystemStatus::MultipleLearning => "MULTIPLE_LEARNING",
                BiddingStrategySystemStatus::MultipleLimited => "MULTIPLE_LIMITED",
                BiddingStrategySystemStatus::MultipleMisconfigured => {
                    "MULTIPLE_MISCONFIGURED"
                }
                BiddingStrategySystemStatus::Multiple => "MULTIPLE",
            }
        }
    }
}
/// Container for enum describing possible bidding strategy types.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BiddingStrategyTypeEnum {}
/// Nested message and enum types in `BiddingStrategyTypeEnum`.
pub mod bidding_strategy_type_enum {
    /// Enum describing possible bidding strategy types.
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
        /// Used for return value only. Indicates that a campaign does not have a
        /// bidding strategy. This prevents the campaign from serving. For example,
        /// a campaign may be attached to a manager bidding strategy and the serving
        /// account is subsequently unlinked from the manager account. In this case
        /// the campaign will automatically be detached from the now inaccessible
        /// manager bidding strategy and transition to the INVALID bidding strategy
        /// type.
        Invalid = 17,
        /// Manual bidding strategy that allows advertiser to set the bid per
        /// advertiser-specified action.
        ManualCpa = 18,
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
    impl BiddingStrategyType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                BiddingStrategyType::Unspecified => "UNSPECIFIED",
                BiddingStrategyType::Unknown => "UNKNOWN",
                BiddingStrategyType::Commission => "COMMISSION",
                BiddingStrategyType::EnhancedCpc => "ENHANCED_CPC",
                BiddingStrategyType::Invalid => "INVALID",
                BiddingStrategyType::ManualCpa => "MANUAL_CPA",
                BiddingStrategyType::ManualCpc => "MANUAL_CPC",
                BiddingStrategyType::ManualCpm => "MANUAL_CPM",
                BiddingStrategyType::ManualCpv => "MANUAL_CPV",
                BiddingStrategyType::MaximizeConversions => "MAXIMIZE_CONVERSIONS",
                BiddingStrategyType::MaximizeConversionValue => {
                    "MAXIMIZE_CONVERSION_VALUE"
                }
                BiddingStrategyType::PageOnePromoted => "PAGE_ONE_PROMOTED",
                BiddingStrategyType::PercentCpc => "PERCENT_CPC",
                BiddingStrategyType::TargetCpa => "TARGET_CPA",
                BiddingStrategyType::TargetCpm => "TARGET_CPM",
                BiddingStrategyType::TargetImpressionShare => "TARGET_IMPRESSION_SHARE",
                BiddingStrategyType::TargetOutrankShare => "TARGET_OUTRANK_SHARE",
                BiddingStrategyType::TargetRoas => "TARGET_ROAS",
                BiddingStrategyType::TargetSpend => "TARGET_SPEND",
            }
        }
    }
}
/// Message describing BillingSetup statuses.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BillingSetupStatusEnum {}
/// Nested message and enum types in `BillingSetupStatusEnum`.
pub mod billing_setup_status_enum {
    /// The possible statuses of a BillingSetup.
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
    impl BillingSetupStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                BillingSetupStatus::Unspecified => "UNSPECIFIED",
                BillingSetupStatus::Unknown => "UNKNOWN",
                BillingSetupStatus::Pending => "PENDING",
                BillingSetupStatus::ApprovedHeld => "APPROVED_HELD",
                BillingSetupStatus::Approved => "APPROVED",
                BillingSetupStatus::Cancelled => "CANCELLED",
            }
        }
    }
}
/// Container for enum with 3-Tier brand safety suitability control.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BrandSafetySuitabilityEnum {}
/// Nested message and enum types in `BrandSafetySuitabilityEnum`.
pub mod brand_safety_suitability_enum {
    /// 3-Tier brand safety suitability control.
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
    impl BrandSafetySuitability {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                BrandSafetySuitability::Unspecified => "UNSPECIFIED",
                BrandSafetySuitability::Unknown => "UNKNOWN",
                BrandSafetySuitability::ExpandedInventory => "EXPANDED_INVENTORY",
                BrandSafetySuitability::StandardInventory => "STANDARD_INVENTORY",
                BrandSafetySuitability::LimitedInventory => "LIMITED_INVENTORY",
            }
        }
    }
}
/// Message describing Budget delivery methods. A delivery method determines the
/// rate at which the Budget is spent.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BudgetDeliveryMethodEnum {}
/// Nested message and enum types in `BudgetDeliveryMethodEnum`.
pub mod budget_delivery_method_enum {
    /// Possible delivery methods of a Budget.
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
    impl BudgetDeliveryMethod {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                BudgetDeliveryMethod::Unspecified => "UNSPECIFIED",
                BudgetDeliveryMethod::Unknown => "UNKNOWN",
                BudgetDeliveryMethod::Standard => "STANDARD",
                BudgetDeliveryMethod::Accelerated => "ACCELERATED",
            }
        }
    }
}
/// Message describing Budget period.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BudgetPeriodEnum {}
/// Nested message and enum types in `BudgetPeriodEnum`.
pub mod budget_period_enum {
    /// Possible period of a Budget.
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
    impl BudgetPeriod {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                BudgetPeriod::Unspecified => "UNSPECIFIED",
                BudgetPeriod::Unknown => "UNKNOWN",
                BudgetPeriod::Daily => "DAILY",
                BudgetPeriod::CustomPeriod => "CUSTOM_PERIOD",
            }
        }
    }
}
/// Message describing a Budget status
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BudgetStatusEnum {}
/// Nested message and enum types in `BudgetStatusEnum`.
pub mod budget_status_enum {
    /// Possible statuses of a Budget.
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
    impl BudgetStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                BudgetStatus::Unspecified => "UNSPECIFIED",
                BudgetStatus::Unknown => "UNKNOWN",
                BudgetStatus::Enabled => "ENABLED",
                BudgetStatus::Removed => "REMOVED",
            }
        }
    }
}
/// Describes Budget types.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BudgetTypeEnum {}
/// Nested message and enum types in `BudgetTypeEnum`.
pub mod budget_type_enum {
    /// Possible Budget types.
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
    pub enum BudgetType {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Budget type for standard Google Ads usage.
        /// Caps daily spend at two times the specified budget amount.
        /// Full details: <https://support.google.com/google-ads/answer/6385083>
        Standard = 2,
        /// Budget type with a fixed cost-per-acquisition (conversion).
        /// Full details: <https://support.google.com/google-ads/answer/7528254>
        ///
        /// This type is only supported by campaigns with
        /// AdvertisingChannelType.DISPLAY (excluding
        /// AdvertisingChannelSubType.DISPLAY_GMAIL),
        /// BiddingStrategyType.TARGET_CPA and PaymentMode.CONVERSIONS.
        FixedCpa = 4,
        /// Budget type for Smart Campaign.
        /// Full details: <https://support.google.com/google-ads/answer/7653509>
        ///
        /// This type is only supported by campaigns with
        /// AdvertisingChannelType.SMART and
        /// AdvertisingChannelSubType.SMART_CAMPAIGN.
        SmartCampaign = 5,
        /// Budget type for Local Services Campaign.
        /// Full details: <https://support.google.com/localservices/answer/7434558>
        ///
        /// This type is only supported by campaigns with
        /// AdvertisingChannelType.LOCAL_SERVICES.
        LocalServices = 6,
    }
    impl BudgetType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                BudgetType::Unspecified => "UNSPECIFIED",
                BudgetType::Unknown => "UNKNOWN",
                BudgetType::Standard => "STANDARD",
                BudgetType::FixedCpa => "FIXED_CPA",
                BudgetType::SmartCampaign => "SMART_CAMPAIGN",
                BudgetType::LocalServices => "LOCAL_SERVICES",
            }
        }
    }
}
/// Values for Call placeholder fields.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CallPlaceholderFieldEnum {}
/// Nested message and enum types in `CallPlaceholderFieldEnum`.
pub mod call_placeholder_field_enum {
    /// Possible values for Call placeholder fields.
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
    impl CallPlaceholderField {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                CallPlaceholderField::Unspecified => "UNSPECIFIED",
                CallPlaceholderField::Unknown => "UNKNOWN",
                CallPlaceholderField::PhoneNumber => "PHONE_NUMBER",
                CallPlaceholderField::CountryCode => "COUNTRY_CODE",
                CallPlaceholderField::Tracked => "TRACKED",
                CallPlaceholderField::ConversionTypeId => "CONVERSION_TYPE_ID",
                CallPlaceholderField::ConversionReportingState => {
                    "CONVERSION_REPORTING_STATE"
                }
            }
        }
    }
}
/// Container for enum describing possible call tracking display locations.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CallTrackingDisplayLocationEnum {}
/// Nested message and enum types in `CallTrackingDisplayLocationEnum`.
pub mod call_tracking_display_location_enum {
    /// Possible call tracking display locations.
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
    impl CallTrackingDisplayLocation {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                CallTrackingDisplayLocation::Unspecified => "UNSPECIFIED",
                CallTrackingDisplayLocation::Unknown => "UNKNOWN",
                CallTrackingDisplayLocation::Ad => "AD",
                CallTrackingDisplayLocation::LandingPage => "LANDING_PAGE",
            }
        }
    }
}
/// Container for enum describing possible types of property from where the call
/// was made.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CallTypeEnum {}
/// Nested message and enum types in `CallTypeEnum`.
pub mod call_type_enum {
    /// Possible types of property from where the call was made.
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
    impl CallType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                CallType::Unspecified => "UNSPECIFIED",
                CallType::Unknown => "UNKNOWN",
                CallType::ManuallyDialed => "MANUALLY_DIALED",
                CallType::HighEndMobileSearch => "HIGH_END_MOBILE_SEARCH",
            }
        }
    }
}
/// Values for Callout placeholder fields.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CalloutPlaceholderFieldEnum {}
/// Nested message and enum types in `CalloutPlaceholderFieldEnum`.
pub mod callout_placeholder_field_enum {
    /// Possible values for Callout placeholder fields.
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
    pub enum CalloutPlaceholderField {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Data Type: STRING. Callout text.
        CalloutText = 2,
    }
    impl CalloutPlaceholderField {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                CalloutPlaceholderField::Unspecified => "UNSPECIFIED",
                CalloutPlaceholderField::Unknown => "UNKNOWN",
                CalloutPlaceholderField::CalloutText => "CALLOUT_TEXT",
            }
        }
    }
}
/// Message describing CampaignCriterion statuses.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CampaignCriterionStatusEnum {}
/// Nested message and enum types in `CampaignCriterionStatusEnum`.
pub mod campaign_criterion_status_enum {
    /// The possible statuses of a CampaignCriterion.
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
    impl CampaignCriterionStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                CampaignCriterionStatus::Unspecified => "UNSPECIFIED",
                CampaignCriterionStatus::Unknown => "UNKNOWN",
                CampaignCriterionStatus::Enabled => "ENABLED",
                CampaignCriterionStatus::Paused => "PAUSED",
                CampaignCriterionStatus::Removed => "REMOVED",
            }
        }
    }
}
/// Container for enum describing possible statuses of a campaign draft.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CampaignDraftStatusEnum {}
/// Nested message and enum types in `CampaignDraftStatusEnum`.
pub mod campaign_draft_status_enum {
    /// Possible statuses of a campaign draft.
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
    impl CampaignDraftStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                CampaignDraftStatus::Unspecified => "UNSPECIFIED",
                CampaignDraftStatus::Unknown => "UNKNOWN",
                CampaignDraftStatus::Proposed => "PROPOSED",
                CampaignDraftStatus::Removed => "REMOVED",
                CampaignDraftStatus::Promoting => "PROMOTING",
                CampaignDraftStatus::Promoted => "PROMOTED",
                CampaignDraftStatus::PromoteFailed => "PROMOTE_FAILED",
            }
        }
    }
}
/// Container for enum describing possible statuses of a campaign experiment.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CampaignExperimentStatusEnum {}
/// Nested message and enum types in `CampaignExperimentStatusEnum`.
pub mod campaign_experiment_status_enum {
    /// Possible statuses of a campaign experiment.
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
    impl CampaignExperimentStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                CampaignExperimentStatus::Unspecified => "UNSPECIFIED",
                CampaignExperimentStatus::Unknown => "UNKNOWN",
                CampaignExperimentStatus::Initializing => "INITIALIZING",
                CampaignExperimentStatus::InitializationFailed => "INITIALIZATION_FAILED",
                CampaignExperimentStatus::Enabled => "ENABLED",
                CampaignExperimentStatus::Graduated => "GRADUATED",
                CampaignExperimentStatus::Removed => "REMOVED",
                CampaignExperimentStatus::Promoting => "PROMOTING",
                CampaignExperimentStatus::PromotionFailed => "PROMOTION_FAILED",
                CampaignExperimentStatus::Promoted => "PROMOTED",
                CampaignExperimentStatus::EndedManually => "ENDED_MANUALLY",
            }
        }
    }
}
/// Container for enum describing campaign experiment traffic split type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CampaignExperimentTrafficSplitTypeEnum {}
/// Nested message and enum types in `CampaignExperimentTrafficSplitTypeEnum`.
pub mod campaign_experiment_traffic_split_type_enum {
    /// Enum of strategies for splitting traffic between base and experiment
    /// campaigns in campaign experiment.
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
    impl CampaignExperimentTrafficSplitType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                CampaignExperimentTrafficSplitType::Unspecified => "UNSPECIFIED",
                CampaignExperimentTrafficSplitType::Unknown => "UNKNOWN",
                CampaignExperimentTrafficSplitType::RandomQuery => "RANDOM_QUERY",
                CampaignExperimentTrafficSplitType::Cookie => "COOKIE",
            }
        }
    }
}
/// Container for enum describing campaign experiment type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CampaignExperimentTypeEnum {}
/// Nested message and enum types in `CampaignExperimentTypeEnum`.
pub mod campaign_experiment_type_enum {
    /// Indicates if this campaign is a normal campaign,
    /// a draft campaign, or an experiment campaign.
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
    impl CampaignExperimentType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                CampaignExperimentType::Unspecified => "UNSPECIFIED",
                CampaignExperimentType::Unknown => "UNKNOWN",
                CampaignExperimentType::Base => "BASE",
                CampaignExperimentType::Draft => "DRAFT",
                CampaignExperimentType::Experiment => "EXPERIMENT",
            }
        }
    }
}
/// Message describing CampaignGroup statuses.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CampaignGroupStatusEnum {}
/// Nested message and enum types in `CampaignGroupStatusEnum`.
pub mod campaign_group_status_enum {
    /// Possible statuses of a CampaignGroup.
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
    pub enum CampaignGroupStatus {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// The campaign group is active.
        Enabled = 2,
        /// The campaign group has been removed.
        Removed = 3,
    }
    impl CampaignGroupStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                CampaignGroupStatus::Unspecified => "UNSPECIFIED",
                CampaignGroupStatus::Unknown => "UNKNOWN",
                CampaignGroupStatus::Enabled => "ENABLED",
                CampaignGroupStatus::Removed => "REMOVED",
            }
        }
    }
}
/// Message describing Campaign serving statuses.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CampaignServingStatusEnum {}
/// Nested message and enum types in `CampaignServingStatusEnum`.
pub mod campaign_serving_status_enum {
    /// Possible serving statuses of a campaign.
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
    impl CampaignServingStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                CampaignServingStatus::Unspecified => "UNSPECIFIED",
                CampaignServingStatus::Unknown => "UNKNOWN",
                CampaignServingStatus::Serving => "SERVING",
                CampaignServingStatus::None => "NONE",
                CampaignServingStatus::Ended => "ENDED",
                CampaignServingStatus::Pending => "PENDING",
                CampaignServingStatus::Suspended => "SUSPENDED",
            }
        }
    }
}
/// Container for enum describing types of campaign shared set statuses.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CampaignSharedSetStatusEnum {}
/// Nested message and enum types in `CampaignSharedSetStatusEnum`.
pub mod campaign_shared_set_status_enum {
    /// Enum listing the possible campaign shared set statuses.
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
    impl CampaignSharedSetStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                CampaignSharedSetStatus::Unspecified => "UNSPECIFIED",
                CampaignSharedSetStatus::Unknown => "UNKNOWN",
                CampaignSharedSetStatus::Enabled => "ENABLED",
                CampaignSharedSetStatus::Removed => "REMOVED",
            }
        }
    }
}
/// Container for enum describing possible statuses of a campaign.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CampaignStatusEnum {}
/// Nested message and enum types in `CampaignStatusEnum`.
pub mod campaign_status_enum {
    /// Possible statuses of a campaign.
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
    impl CampaignStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                CampaignStatus::Unspecified => "UNSPECIFIED",
                CampaignStatus::Unknown => "UNKNOWN",
                CampaignStatus::Enabled => "ENABLED",
                CampaignStatus::Paused => "PAUSED",
                CampaignStatus::Removed => "REMOVED",
            }
        }
    }
}
/// Container for enum describing the sources that the change event resource
/// was made through.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangeClientTypeEnum {}
/// Nested message and enum types in `ChangeClientTypeEnum`.
pub mod change_client_type_enum {
    /// The source that the change_event resource was made through.
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
    impl ChangeClientType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ChangeClientType::Unspecified => "UNSPECIFIED",
                ChangeClientType::Unknown => "UNKNOWN",
                ChangeClientType::GoogleAdsWebClient => "GOOGLE_ADS_WEB_CLIENT",
                ChangeClientType::GoogleAdsAutomatedRule => "GOOGLE_ADS_AUTOMATED_RULE",
                ChangeClientType::GoogleAdsScripts => "GOOGLE_ADS_SCRIPTS",
                ChangeClientType::GoogleAdsBulkUpload => "GOOGLE_ADS_BULK_UPLOAD",
                ChangeClientType::GoogleAdsApi => "GOOGLE_ADS_API",
                ChangeClientType::GoogleAdsEditor => "GOOGLE_ADS_EDITOR",
                ChangeClientType::GoogleAdsMobileApp => "GOOGLE_ADS_MOBILE_APP",
                ChangeClientType::GoogleAdsRecommendations => {
                    "GOOGLE_ADS_RECOMMENDATIONS"
                }
                ChangeClientType::SearchAds360Sync => "SEARCH_ADS_360_SYNC",
                ChangeClientType::SearchAds360Post => "SEARCH_ADS_360_POST",
                ChangeClientType::InternalTool => "INTERNAL_TOOL",
                ChangeClientType::Other => "OTHER",
            }
        }
    }
}
/// Container for enum describing supported resource types for the ChangeEvent
/// resource.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangeEventResourceTypeEnum {}
/// Nested message and enum types in `ChangeEventResourceTypeEnum`.
pub mod change_event_resource_type_enum {
    /// Enum listing the resource types support by the ChangeEvent resource.
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
        /// An Asset resource change.
        Asset = 14,
        /// A CustomerAsset resource change.
        CustomerAsset = 15,
        /// A CampaignAsset resource change.
        CampaignAsset = 16,
        /// An AdGroupAsset resource change.
        AdGroupAsset = 17,
        /// An AssetSet resource change.
        AssetSet = 18,
        /// An AssetSetAsset resource change.
        AssetSetAsset = 19,
        /// A CampaignAssetSet resource change.
        CampaignAssetSet = 20,
    }
    impl ChangeEventResourceType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ChangeEventResourceType::Unspecified => "UNSPECIFIED",
                ChangeEventResourceType::Unknown => "UNKNOWN",
                ChangeEventResourceType::Ad => "AD",
                ChangeEventResourceType::AdGroup => "AD_GROUP",
                ChangeEventResourceType::AdGroupCriterion => "AD_GROUP_CRITERION",
                ChangeEventResourceType::Campaign => "CAMPAIGN",
                ChangeEventResourceType::CampaignBudget => "CAMPAIGN_BUDGET",
                ChangeEventResourceType::AdGroupBidModifier => "AD_GROUP_BID_MODIFIER",
                ChangeEventResourceType::CampaignCriterion => "CAMPAIGN_CRITERION",
                ChangeEventResourceType::Feed => "FEED",
                ChangeEventResourceType::FeedItem => "FEED_ITEM",
                ChangeEventResourceType::CampaignFeed => "CAMPAIGN_FEED",
                ChangeEventResourceType::AdGroupFeed => "AD_GROUP_FEED",
                ChangeEventResourceType::AdGroupAd => "AD_GROUP_AD",
                ChangeEventResourceType::Asset => "ASSET",
                ChangeEventResourceType::CustomerAsset => "CUSTOMER_ASSET",
                ChangeEventResourceType::CampaignAsset => "CAMPAIGN_ASSET",
                ChangeEventResourceType::AdGroupAsset => "AD_GROUP_ASSET",
                ChangeEventResourceType::AssetSet => "ASSET_SET",
                ChangeEventResourceType::AssetSetAsset => "ASSET_SET_ASSET",
                ChangeEventResourceType::CampaignAssetSet => "CAMPAIGN_ASSET_SET",
            }
        }
    }
}
/// Container for enum describing operations for the ChangeStatus resource.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangeStatusOperationEnum {}
/// Nested message and enum types in `ChangeStatusOperationEnum`.
pub mod change_status_operation_enum {
    /// Status of the changed resource
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
    impl ChangeStatusOperation {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ChangeStatusOperation::Unspecified => "UNSPECIFIED",
                ChangeStatusOperation::Unknown => "UNKNOWN",
                ChangeStatusOperation::Added => "ADDED",
                ChangeStatusOperation::Changed => "CHANGED",
                ChangeStatusOperation::Removed => "REMOVED",
            }
        }
    }
}
/// Container for enum describing supported resource types for the ChangeStatus
/// resource.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangeStatusResourceTypeEnum {}
/// Nested message and enum types in `ChangeStatusResourceTypeEnum`.
pub mod change_status_resource_type_enum {
    /// Enum listing the resource types support by the ChangeStatus resource.
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
        /// A SharedSet resource change.
        SharedSet = 14,
        /// A CampaignSharedSet resource change.
        CampaignSharedSet = 15,
        /// An Asset resource change.
        Asset = 16,
        /// A CustomerAsset resource change.
        CustomerAsset = 17,
        /// A CampaignAsset resource change.
        CampaignAsset = 18,
        /// An AdGroupAsset resource change.
        AdGroupAsset = 19,
    }
    impl ChangeStatusResourceType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ChangeStatusResourceType::Unspecified => "UNSPECIFIED",
                ChangeStatusResourceType::Unknown => "UNKNOWN",
                ChangeStatusResourceType::AdGroup => "AD_GROUP",
                ChangeStatusResourceType::AdGroupAd => "AD_GROUP_AD",
                ChangeStatusResourceType::AdGroupCriterion => "AD_GROUP_CRITERION",
                ChangeStatusResourceType::Campaign => "CAMPAIGN",
                ChangeStatusResourceType::CampaignCriterion => "CAMPAIGN_CRITERION",
                ChangeStatusResourceType::Feed => "FEED",
                ChangeStatusResourceType::FeedItem => "FEED_ITEM",
                ChangeStatusResourceType::AdGroupFeed => "AD_GROUP_FEED",
                ChangeStatusResourceType::CampaignFeed => "CAMPAIGN_FEED",
                ChangeStatusResourceType::AdGroupBidModifier => "AD_GROUP_BID_MODIFIER",
                ChangeStatusResourceType::SharedSet => "SHARED_SET",
                ChangeStatusResourceType::CampaignSharedSet => "CAMPAIGN_SHARED_SET",
                ChangeStatusResourceType::Asset => "ASSET",
                ChangeStatusResourceType::CustomerAsset => "CUSTOMER_ASSET",
                ChangeStatusResourceType::CampaignAsset => "CAMPAIGN_ASSET",
                ChangeStatusResourceType::AdGroupAsset => "AD_GROUP_ASSET",
            }
        }
    }
}
/// The status of combined audience.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CombinedAudienceStatusEnum {}
/// Nested message and enum types in `CombinedAudienceStatusEnum`.
pub mod combined_audience_status_enum {
    /// Enum containing possible combined audience status types.
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
    impl CombinedAudienceStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                CombinedAudienceStatus::Unspecified => "UNSPECIFIED",
                CombinedAudienceStatus::Unknown => "UNKNOWN",
                CombinedAudienceStatus::Enabled => "ENABLED",
                CombinedAudienceStatus::Removed => "REMOVED",
            }
        }
    }
}
/// Container for enum describing the conversion deduplication mode for
/// conversion optimizer.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConversionActionCountingTypeEnum {}
/// Nested message and enum types in `ConversionActionCountingTypeEnum`.
pub mod conversion_action_counting_type_enum {
    /// Indicates how conversions for this action will be counted. For more
    /// information, see <https://support.google.com/google-ads/answer/3438531.>
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
    impl ConversionActionCountingType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ConversionActionCountingType::Unspecified => "UNSPECIFIED",
                ConversionActionCountingType::Unknown => "UNKNOWN",
                ConversionActionCountingType::OnePerClick => "ONE_PER_CLICK",
                ConversionActionCountingType::ManyPerClick => "MANY_PER_CLICK",
            }
        }
    }
}
/// Container for enum describing possible statuses of a conversion action.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConversionActionStatusEnum {}
/// Nested message and enum types in `ConversionActionStatusEnum`.
pub mod conversion_action_status_enum {
    /// Possible statuses of a conversion action.
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
    impl ConversionActionStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ConversionActionStatus::Unspecified => "UNSPECIFIED",
                ConversionActionStatus::Unknown => "UNKNOWN",
                ConversionActionStatus::Enabled => "ENABLED",
                ConversionActionStatus::Removed => "REMOVED",
                ConversionActionStatus::Hidden => "HIDDEN",
            }
        }
    }
}
/// Container for enum describing possible types of a conversion action.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConversionActionTypeEnum {}
/// Nested message and enum types in `ConversionActionTypeEnum`.
pub mod conversion_action_type_enum {
    /// Possible types of a conversion action.
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
        /// Android app first open conversions tracked through Firebase.
        FirebaseAndroidFirstOpen = 12,
        /// Android app in app purchase conversions tracked through Firebase.
        FirebaseAndroidInAppPurchase = 13,
        /// Android app custom conversions tracked through Firebase.
        FirebaseAndroidCustom = 14,
        /// iOS app first open conversions tracked through Firebase.
        FirebaseIosFirstOpen = 15,
        /// iOS app in app purchase conversions tracked through Firebase.
        FirebaseIosInAppPurchase = 16,
        /// iOS app custom conversions tracked through Firebase.
        FirebaseIosCustom = 17,
        /// Android app first open conversions tracked through Third Party App
        /// Analytics.
        ThirdPartyAppAnalyticsAndroidFirstOpen = 18,
        /// Android app in app purchase conversions tracked through Third Party App
        /// Analytics.
        ThirdPartyAppAnalyticsAndroidInAppPurchase = 19,
        /// Android app custom conversions tracked through Third Party App Analytics.
        ThirdPartyAppAnalyticsAndroidCustom = 20,
        /// iOS app first open conversions tracked through Third Party App Analytics.
        ThirdPartyAppAnalyticsIosFirstOpen = 21,
        /// iOS app in app purchase conversions tracked through Third Party App
        /// Analytics.
        ThirdPartyAppAnalyticsIosInAppPurchase = 22,
        /// iOS app custom conversions tracked through Third Party App Analytics.
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
    impl ConversionActionType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ConversionActionType::Unspecified => "UNSPECIFIED",
                ConversionActionType::Unknown => "UNKNOWN",
                ConversionActionType::AdCall => "AD_CALL",
                ConversionActionType::ClickToCall => "CLICK_TO_CALL",
                ConversionActionType::GooglePlayDownload => "GOOGLE_PLAY_DOWNLOAD",
                ConversionActionType::GooglePlayInAppPurchase => {
                    "GOOGLE_PLAY_IN_APP_PURCHASE"
                }
                ConversionActionType::UploadCalls => "UPLOAD_CALLS",
                ConversionActionType::UploadClicks => "UPLOAD_CLICKS",
                ConversionActionType::Webpage => "WEBPAGE",
                ConversionActionType::WebsiteCall => "WEBSITE_CALL",
                ConversionActionType::StoreSalesDirectUpload => {
                    "STORE_SALES_DIRECT_UPLOAD"
                }
                ConversionActionType::StoreSales => "STORE_SALES",
                ConversionActionType::FirebaseAndroidFirstOpen => {
                    "FIREBASE_ANDROID_FIRST_OPEN"
                }
                ConversionActionType::FirebaseAndroidInAppPurchase => {
                    "FIREBASE_ANDROID_IN_APP_PURCHASE"
                }
                ConversionActionType::FirebaseAndroidCustom => "FIREBASE_ANDROID_CUSTOM",
                ConversionActionType::FirebaseIosFirstOpen => "FIREBASE_IOS_FIRST_OPEN",
                ConversionActionType::FirebaseIosInAppPurchase => {
                    "FIREBASE_IOS_IN_APP_PURCHASE"
                }
                ConversionActionType::FirebaseIosCustom => "FIREBASE_IOS_CUSTOM",
                ConversionActionType::ThirdPartyAppAnalyticsAndroidFirstOpen => {
                    "THIRD_PARTY_APP_ANALYTICS_ANDROID_FIRST_OPEN"
                }
                ConversionActionType::ThirdPartyAppAnalyticsAndroidInAppPurchase => {
                    "THIRD_PARTY_APP_ANALYTICS_ANDROID_IN_APP_PURCHASE"
                }
                ConversionActionType::ThirdPartyAppAnalyticsAndroidCustom => {
                    "THIRD_PARTY_APP_ANALYTICS_ANDROID_CUSTOM"
                }
                ConversionActionType::ThirdPartyAppAnalyticsIosFirstOpen => {
                    "THIRD_PARTY_APP_ANALYTICS_IOS_FIRST_OPEN"
                }
                ConversionActionType::ThirdPartyAppAnalyticsIosInAppPurchase => {
                    "THIRD_PARTY_APP_ANALYTICS_IOS_IN_APP_PURCHASE"
                }
                ConversionActionType::ThirdPartyAppAnalyticsIosCustom => {
                    "THIRD_PARTY_APP_ANALYTICS_IOS_CUSTOM"
                }
                ConversionActionType::AndroidAppPreRegistration => {
                    "ANDROID_APP_PRE_REGISTRATION"
                }
                ConversionActionType::AndroidInstallsAllOtherApps => {
                    "ANDROID_INSTALLS_ALL_OTHER_APPS"
                }
                ConversionActionType::FloodlightAction => "FLOODLIGHT_ACTION",
                ConversionActionType::FloodlightTransaction => "FLOODLIGHT_TRANSACTION",
                ConversionActionType::GoogleHosted => "GOOGLE_HOSTED",
                ConversionActionType::LeadFormSubmit => "LEAD_FORM_SUBMIT",
                ConversionActionType::Salesforce => "SALESFORCE",
                ConversionActionType::SearchAds360 => "SEARCH_ADS_360",
                ConversionActionType::SmartCampaignAdClicksToCall => {
                    "SMART_CAMPAIGN_AD_CLICKS_TO_CALL"
                }
                ConversionActionType::SmartCampaignMapClicksToCall => {
                    "SMART_CAMPAIGN_MAP_CLICKS_TO_CALL"
                }
                ConversionActionType::SmartCampaignMapDirections => {
                    "SMART_CAMPAIGN_MAP_DIRECTIONS"
                }
                ConversionActionType::SmartCampaignTrackedCalls => {
                    "SMART_CAMPAIGN_TRACKED_CALLS"
                }
                ConversionActionType::StoreVisits => "STORE_VISITS",
            }
        }
    }
}
/// Container for enum describing conversion adjustment types.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConversionAdjustmentTypeEnum {}
/// Nested message and enum types in `ConversionAdjustmentTypeEnum`.
pub mod conversion_adjustment_type_enum {
    /// The different actions advertisers can take to adjust the conversions that
    /// they already reported. Retractions negate a conversion. Restatements change
    /// the value of a conversion.
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
    pub enum ConversionAdjustmentType {
        /// Not specified.
        Unspecified = 0,
        /// Represents value unknown in this version.
        Unknown = 1,
        /// Negates a conversion so that its total value and count are both zero.
        Retraction = 2,
        /// Changes the value of a conversion.
        Restatement = 3,
        /// Supplements an existing conversion with provided user identifiers and
        /// user agent, which can be used by Google to enhance the conversion count.
        Enhancement = 4,
    }
    impl ConversionAdjustmentType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ConversionAdjustmentType::Unspecified => "UNSPECIFIED",
                ConversionAdjustmentType::Unknown => "UNKNOWN",
                ConversionAdjustmentType::Retraction => "RETRACTION",
                ConversionAdjustmentType::Restatement => "RESTATEMENT",
                ConversionAdjustmentType::Enhancement => "ENHANCEMENT",
            }
        }
    }
}
/// Container for enum describing possible statuses of a conversion custom
/// variable.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConversionCustomVariableStatusEnum {}
/// Nested message and enum types in `ConversionCustomVariableStatusEnum`.
pub mod conversion_custom_variable_status_enum {
    /// Possible statuses of a conversion custom variable.
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
    impl ConversionCustomVariableStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ConversionCustomVariableStatus::Unspecified => "UNSPECIFIED",
                ConversionCustomVariableStatus::Unknown => "UNKNOWN",
                ConversionCustomVariableStatus::ActivationNeeded => "ACTIVATION_NEEDED",
                ConversionCustomVariableStatus::Enabled => "ENABLED",
                ConversionCustomVariableStatus::Paused => "PAUSED",
            }
        }
    }
}
/// Container for enum representing the conversion environment an uploaded
/// conversion was recorded on, for example, App or Web.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConversionEnvironmentEnum {}
/// Nested message and enum types in `ConversionEnvironmentEnum`.
pub mod conversion_environment_enum {
    /// Conversion environment of the uploaded conversion.
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
    pub enum ConversionEnvironment {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// The conversion was recorded on an app.
        App = 2,
        /// The conversion was recorded on a website.
        Web = 3,
    }
    impl ConversionEnvironment {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ConversionEnvironment::Unspecified => "UNSPECIFIED",
                ConversionEnvironment::Unknown => "UNKNOWN",
                ConversionEnvironment::App => "APP",
                ConversionEnvironment::Web => "WEB",
            }
        }
    }
}
/// Container for enum describing possible conversion origins.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConversionOriginEnum {}
/// Nested message and enum types in `ConversionOriginEnum`.
pub mod conversion_origin_enum {
    /// The possible places where a conversion can occur.
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
    pub enum ConversionOrigin {
        /// The conversion origin has not been specified.
        Unspecified = 0,
        /// The conversion origin is not known in this version.
        Unknown = 1,
        /// Conversion that occurs when a user visits a website or takes an action
        /// there after viewing an ad.
        Website = 2,
        /// Conversions reported by an offline pipeline which collects local actions
        /// from Google-hosted pages (for example, Google Maps, Google Place Page,
        /// etc) and attributes them to relevant ad events.
        GoogleHosted = 3,
        /// Conversion that occurs when a user performs an action through any app
        /// platforms.
        App = 4,
        /// Conversion that occurs when a user makes a call from ads.
        CallFromAds = 5,
        /// Conversion that occurs when a user visits or makes a purchase at a
        /// physical store.
        Store = 6,
        /// Conversion that occurs on YouTube.
        YoutubeHosted = 7,
    }
    impl ConversionOrigin {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ConversionOrigin::Unspecified => "UNSPECIFIED",
                ConversionOrigin::Unknown => "UNKNOWN",
                ConversionOrigin::Website => "WEBSITE",
                ConversionOrigin::GoogleHosted => "GOOGLE_HOSTED",
                ConversionOrigin::App => "APP",
                ConversionOrigin::CallFromAds => "CALL_FROM_ADS",
                ConversionOrigin::Store => "STORE",
                ConversionOrigin::YoutubeHosted => "YOUTUBE_HOSTED",
            }
        }
    }
}
/// Container for enum representing the conversion tracking status of the
/// customer.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConversionTrackingStatusEnum {}
/// Nested message and enum types in `ConversionTrackingStatusEnum`.
pub mod conversion_tracking_status_enum {
    /// Conversion Tracking status of the customer.
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
    pub enum ConversionTrackingStatus {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Customer does not use any conversion tracking.
        NotConversionTracked = 2,
        /// The conversion actions are created and managed by this customer.
        ConversionTrackingManagedBySelf = 3,
        /// The conversion actions are created and managed by the manager specified
        /// in the request's `login-customer-id`.
        ConversionTrackingManagedByThisManager = 4,
        /// The conversion actions are created and managed by a manager different
        /// from the customer or manager specified in the request's
        /// `login-customer-id`.
        ConversionTrackingManagedByAnotherManager = 5,
    }
    impl ConversionTrackingStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ConversionTrackingStatus::Unspecified => "UNSPECIFIED",
                ConversionTrackingStatus::Unknown => "UNKNOWN",
                ConversionTrackingStatus::NotConversionTracked => {
                    "NOT_CONVERSION_TRACKED"
                }
                ConversionTrackingStatus::ConversionTrackingManagedBySelf => {
                    "CONVERSION_TRACKING_MANAGED_BY_SELF"
                }
                ConversionTrackingStatus::ConversionTrackingManagedByThisManager => {
                    "CONVERSION_TRACKING_MANAGED_BY_THIS_MANAGER"
                }
                ConversionTrackingStatus::ConversionTrackingManagedByAnotherManager => {
                    "CONVERSION_TRACKING_MANAGED_BY_ANOTHER_MANAGER"
                }
            }
        }
    }
}
/// Container for enum describing possible statuses of a conversion value rule
/// set.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConversionValueRuleSetStatusEnum {}
/// Nested message and enum types in `ConversionValueRuleSetStatusEnum`.
pub mod conversion_value_rule_set_status_enum {
    /// Possible statuses of a conversion value rule set.
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
    pub enum ConversionValueRuleSetStatus {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Conversion Value Rule Set is enabled and can be applied.
        Enabled = 2,
        /// Conversion Value Rule Set is permanently deleted and can't be applied.
        Removed = 3,
        /// Conversion Value Rule Set is paused and won't be applied. It can be
        /// enabled again.
        Paused = 4,
    }
    impl ConversionValueRuleSetStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ConversionValueRuleSetStatus::Unspecified => "UNSPECIFIED",
                ConversionValueRuleSetStatus::Unknown => "UNKNOWN",
                ConversionValueRuleSetStatus::Enabled => "ENABLED",
                ConversionValueRuleSetStatus::Removed => "REMOVED",
                ConversionValueRuleSetStatus::Paused => "PAUSED",
            }
        }
    }
}
/// Container for enum describing possible statuses of a conversion value rule.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConversionValueRuleStatusEnum {}
/// Nested message and enum types in `ConversionValueRuleStatusEnum`.
pub mod conversion_value_rule_status_enum {
    /// Possible statuses of a conversion value rule.
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
    pub enum ConversionValueRuleStatus {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Conversion Value Rule is enabled and can be applied.
        Enabled = 2,
        /// Conversion Value Rule is permanently deleted and can't be applied.
        Removed = 3,
        /// Conversion Value Rule is paused, but can be re-enabled.
        Paused = 4,
    }
    impl ConversionValueRuleStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ConversionValueRuleStatus::Unspecified => "UNSPECIFIED",
                ConversionValueRuleStatus::Unknown => "UNKNOWN",
                ConversionValueRuleStatus::Enabled => "ENABLED",
                ConversionValueRuleStatus::Removed => "REMOVED",
                ConversionValueRuleStatus::Paused => "PAUSED",
            }
        }
    }
}
/// Container for enum describing possible criterion system serving statuses.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CriterionSystemServingStatusEnum {}
/// Nested message and enum types in `CriterionSystemServingStatusEnum`.
pub mod criterion_system_serving_status_enum {
    /// Enumerates criterion system serving statuses.
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
    impl CriterionSystemServingStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                CriterionSystemServingStatus::Unspecified => "UNSPECIFIED",
                CriterionSystemServingStatus::Unknown => "UNKNOWN",
                CriterionSystemServingStatus::Eligible => "ELIGIBLE",
                CriterionSystemServingStatus::RarelyServed => "RARELY_SERVED",
            }
        }
    }
}
/// The possible types of a criterion.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CriterionTypeEnum {}
/// Nested message and enum types in `CriterionTypeEnum`.
pub mod criterion_type_enum {
    /// Enum describing possible criterion types.
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
    pub enum CriterionType {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Keyword, for example, 'mars cruise'.
        Keyword = 2,
        /// Placement, also known as Website, for example, 'www.flowers4sale.com'
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
        /// A topic target on the display network (for example, "Pets & Animals").
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
        /// Smart Campaign keyword theme
        KeywordTheme = 34,
        /// Audience
        Audience = 35,
    }
    impl CriterionType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                CriterionType::Unspecified => "UNSPECIFIED",
                CriterionType::Unknown => "UNKNOWN",
                CriterionType::Keyword => "KEYWORD",
                CriterionType::Placement => "PLACEMENT",
                CriterionType::MobileAppCategory => "MOBILE_APP_CATEGORY",
                CriterionType::MobileApplication => "MOBILE_APPLICATION",
                CriterionType::Device => "DEVICE",
                CriterionType::Location => "LOCATION",
                CriterionType::ListingGroup => "LISTING_GROUP",
                CriterionType::AdSchedule => "AD_SCHEDULE",
                CriterionType::AgeRange => "AGE_RANGE",
                CriterionType::Gender => "GENDER",
                CriterionType::IncomeRange => "INCOME_RANGE",
                CriterionType::ParentalStatus => "PARENTAL_STATUS",
                CriterionType::YoutubeVideo => "YOUTUBE_VIDEO",
                CriterionType::YoutubeChannel => "YOUTUBE_CHANNEL",
                CriterionType::UserList => "USER_LIST",
                CriterionType::Proximity => "PROXIMITY",
                CriterionType::Topic => "TOPIC",
                CriterionType::ListingScope => "LISTING_SCOPE",
                CriterionType::Language => "LANGUAGE",
                CriterionType::IpBlock => "IP_BLOCK",
                CriterionType::ContentLabel => "CONTENT_LABEL",
                CriterionType::Carrier => "CARRIER",
                CriterionType::UserInterest => "USER_INTEREST",
                CriterionType::Webpage => "WEBPAGE",
                CriterionType::OperatingSystemVersion => "OPERATING_SYSTEM_VERSION",
                CriterionType::AppPaymentModel => "APP_PAYMENT_MODEL",
                CriterionType::MobileDevice => "MOBILE_DEVICE",
                CriterionType::CustomAffinity => "CUSTOM_AFFINITY",
                CriterionType::CustomIntent => "CUSTOM_INTENT",
                CriterionType::LocationGroup => "LOCATION_GROUP",
                CriterionType::CustomAudience => "CUSTOM_AUDIENCE",
                CriterionType::CombinedAudience => "COMBINED_AUDIENCE",
                CriterionType::KeywordTheme => "KEYWORD_THEME",
                CriterionType::Audience => "AUDIENCE",
            }
        }
    }
}
/// The type of custom audience member.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomAudienceMemberTypeEnum {}
/// Nested message and enum types in `CustomAudienceMemberTypeEnum`.
pub mod custom_audience_member_type_enum {
    /// Enum containing possible custom audience member types.
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
    impl CustomAudienceMemberType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                CustomAudienceMemberType::Unspecified => "UNSPECIFIED",
                CustomAudienceMemberType::Unknown => "UNKNOWN",
                CustomAudienceMemberType::Keyword => "KEYWORD",
                CustomAudienceMemberType::Url => "URL",
                CustomAudienceMemberType::PlaceCategory => "PLACE_CATEGORY",
                CustomAudienceMemberType::App => "APP",
            }
        }
    }
}
/// The status of custom audience.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomAudienceStatusEnum {}
/// Nested message and enum types in `CustomAudienceStatusEnum`.
pub mod custom_audience_status_enum {
    /// Enum containing possible custom audience statuses.
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
    impl CustomAudienceStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                CustomAudienceStatus::Unspecified => "UNSPECIFIED",
                CustomAudienceStatus::Unknown => "UNKNOWN",
                CustomAudienceStatus::Enabled => "ENABLED",
                CustomAudienceStatus::Removed => "REMOVED",
            }
        }
    }
}
/// The types of custom audience.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomAudienceTypeEnum {}
/// Nested message and enum types in `CustomAudienceTypeEnum`.
pub mod custom_audience_type_enum {
    /// Enum containing possible custom audience types.
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
    impl CustomAudienceType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                CustomAudienceType::Unspecified => "UNSPECIFIED",
                CustomAudienceType::Unknown => "UNKNOWN",
                CustomAudienceType::Auto => "AUTO",
                CustomAudienceType::Interest => "INTEREST",
                CustomAudienceType::PurchaseIntent => "PURCHASE_INTENT",
                CustomAudienceType::Search => "SEARCH",
            }
        }
    }
}
/// Container for enum describing possible statuses of a custom conversion goal.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomConversionGoalStatusEnum {}
/// Nested message and enum types in `CustomConversionGoalStatusEnum`.
pub mod custom_conversion_goal_status_enum {
    /// The possible statuses of a custom conversion goal.
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
    pub enum CustomConversionGoalStatus {
        /// The status has not been specified.
        Unspecified = 0,
        /// The received value is not known in this version.
        Unknown = 1,
        /// The custom conversion goal is enabled.
        Enabled = 2,
        /// The custom conversion goal is removed.
        Removed = 3,
    }
    impl CustomConversionGoalStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                CustomConversionGoalStatus::Unspecified => "UNSPECIFIED",
                CustomConversionGoalStatus::Unknown => "UNKNOWN",
                CustomConversionGoalStatus::Enabled => "ENABLED",
                CustomConversionGoalStatus::Removed => "REMOVED",
            }
        }
    }
}
/// The types of custom interest member, either KEYWORD or URL.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomInterestMemberTypeEnum {}
/// Nested message and enum types in `CustomInterestMemberTypeEnum`.
pub mod custom_interest_member_type_enum {
    /// Enum containing possible custom interest member types.
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
    impl CustomInterestMemberType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                CustomInterestMemberType::Unspecified => "UNSPECIFIED",
                CustomInterestMemberType::Unknown => "UNKNOWN",
                CustomInterestMemberType::Keyword => "KEYWORD",
                CustomInterestMemberType::Url => "URL",
            }
        }
    }
}
/// The status of custom interest.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomInterestStatusEnum {}
/// Nested message and enum types in `CustomInterestStatusEnum`.
pub mod custom_interest_status_enum {
    /// Enum containing possible custom interest types.
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
    impl CustomInterestStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                CustomInterestStatus::Unspecified => "UNSPECIFIED",
                CustomInterestStatus::Unknown => "UNKNOWN",
                CustomInterestStatus::Enabled => "ENABLED",
                CustomInterestStatus::Removed => "REMOVED",
            }
        }
    }
}
/// The types of custom interest.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomInterestTypeEnum {}
/// Nested message and enum types in `CustomInterestTypeEnum`.
pub mod custom_interest_type_enum {
    /// Enum containing possible custom interest types.
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
    impl CustomInterestType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                CustomInterestType::Unspecified => "UNSPECIFIED",
                CustomInterestType::Unknown => "UNKNOWN",
                CustomInterestType::CustomAffinity => "CUSTOM_AFFINITY",
                CustomInterestType::CustomIntent => "CUSTOM_INTENT",
            }
        }
    }
}
/// Values for Custom placeholder fields.
/// For more information about dynamic remarketing feeds, see
/// <https://support.google.com/google-ads/answer/6053288.>
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomPlaceholderFieldEnum {}
/// Nested message and enum types in `CustomPlaceholderFieldEnum`.
pub mod custom_placeholder_field_enum {
    /// Possible values for Custom placeholder fields.
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
        ///    scheme.
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
    impl CustomPlaceholderField {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                CustomPlaceholderField::Unspecified => "UNSPECIFIED",
                CustomPlaceholderField::Unknown => "UNKNOWN",
                CustomPlaceholderField::Id => "ID",
                CustomPlaceholderField::Id2 => "ID2",
                CustomPlaceholderField::ItemTitle => "ITEM_TITLE",
                CustomPlaceholderField::ItemSubtitle => "ITEM_SUBTITLE",
                CustomPlaceholderField::ItemDescription => "ITEM_DESCRIPTION",
                CustomPlaceholderField::ItemAddress => "ITEM_ADDRESS",
                CustomPlaceholderField::Price => "PRICE",
                CustomPlaceholderField::FormattedPrice => "FORMATTED_PRICE",
                CustomPlaceholderField::SalePrice => "SALE_PRICE",
                CustomPlaceholderField::FormattedSalePrice => "FORMATTED_SALE_PRICE",
                CustomPlaceholderField::ImageUrl => "IMAGE_URL",
                CustomPlaceholderField::ItemCategory => "ITEM_CATEGORY",
                CustomPlaceholderField::FinalUrls => "FINAL_URLS",
                CustomPlaceholderField::FinalMobileUrls => "FINAL_MOBILE_URLS",
                CustomPlaceholderField::TrackingUrl => "TRACKING_URL",
                CustomPlaceholderField::ContextualKeywords => "CONTEXTUAL_KEYWORDS",
                CustomPlaceholderField::AndroidAppLink => "ANDROID_APP_LINK",
                CustomPlaceholderField::SimilarIds => "SIMILAR_IDS",
                CustomPlaceholderField::IosAppLink => "IOS_APP_LINK",
                CustomPlaceholderField::IosAppStoreId => "IOS_APP_STORE_ID",
            }
        }
    }
}
/// Container for enum describing reasons why a customer is not eligible to use
/// PaymentMode.CONVERSIONS.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomerPayPerConversionEligibilityFailureReasonEnum {}
/// Nested message and enum types in `CustomerPayPerConversionEligibilityFailureReasonEnum`.
pub mod customer_pay_per_conversion_eligibility_failure_reason_enum {
    /// Enum describing possible reasons a customer is not eligible to use
    /// PaymentMode.CONVERSIONS.
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
    impl CustomerPayPerConversionEligibilityFailureReason {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                CustomerPayPerConversionEligibilityFailureReason::Unspecified => {
                    "UNSPECIFIED"
                }
                CustomerPayPerConversionEligibilityFailureReason::Unknown => "UNKNOWN",
                CustomerPayPerConversionEligibilityFailureReason::NotEnoughConversions => {
                    "NOT_ENOUGH_CONVERSIONS"
                }
                CustomerPayPerConversionEligibilityFailureReason::ConversionLagTooHigh => {
                    "CONVERSION_LAG_TOO_HIGH"
                }
                CustomerPayPerConversionEligibilityFailureReason::HasCampaignWithSharedBudget => {
                    "HAS_CAMPAIGN_WITH_SHARED_BUDGET"
                }
                CustomerPayPerConversionEligibilityFailureReason::HasUploadClicksConversion => {
                    "HAS_UPLOAD_CLICKS_CONVERSION"
                }
                CustomerPayPerConversionEligibilityFailureReason::AverageDailySpendTooHigh => {
                    "AVERAGE_DAILY_SPEND_TOO_HIGH"
                }
                CustomerPayPerConversionEligibilityFailureReason::AnalysisNotComplete => {
                    "ANALYSIS_NOT_COMPLETE"
                }
                CustomerPayPerConversionEligibilityFailureReason::Other => "OTHER",
            }
        }
    }
}
/// Container for enum describing possible statuses of a customer.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomerStatusEnum {}
/// Nested message and enum types in `CustomerStatusEnum`.
pub mod customer_status_enum {
    /// Possible statuses of a customer.
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
    pub enum CustomerStatus {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Indicates an active account able to serve ads.
        Enabled = 2,
        /// Indicates a canceled account unable to serve ads.
        /// Can be reactivated by an admin user.
        Canceled = 3,
        /// Indicates a suspended account unable to serve ads.
        /// May only be activated by Google support.
        Suspended = 4,
        /// Indicates a closed account unable to serve ads.
        /// Test account will also have CLOSED status.
        /// Status is permanent and may not be reopened.
        Closed = 5,
    }
    impl CustomerStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                CustomerStatus::Unspecified => "UNSPECIFIED",
                CustomerStatus::Unknown => "UNKNOWN",
                CustomerStatus::Enabled => "ENABLED",
                CustomerStatus::Canceled => "CANCELED",
                CustomerStatus::Suspended => "SUSPENDED",
                CustomerStatus::Closed => "CLOSED",
            }
        }
    }
}
/// Container for enum describing possible statuses of a customizer attribute.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomizerAttributeStatusEnum {}
/// Nested message and enum types in `CustomizerAttributeStatusEnum`.
pub mod customizer_attribute_status_enum {
    /// The possible statuses of a customizer attribute.
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
    pub enum CustomizerAttributeStatus {
        /// The status has not been specified.
        Unspecified = 0,
        /// The received value is not known in this version.
        Unknown = 1,
        /// The customizer attribute is enabled.
        Enabled = 2,
        /// The customizer attribute is removed.
        Removed = 3,
    }
    impl CustomizerAttributeStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                CustomizerAttributeStatus::Unspecified => "UNSPECIFIED",
                CustomizerAttributeStatus::Unknown => "UNKNOWN",
                CustomizerAttributeStatus::Enabled => "ENABLED",
                CustomizerAttributeStatus::Removed => "REMOVED",
            }
        }
    }
}
/// Container for enum describing possible statuses of a customizer value.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomizerValueStatusEnum {}
/// Nested message and enum types in `CustomizerValueStatusEnum`.
pub mod customizer_value_status_enum {
    /// The possible statuses of a customizer value.
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
    pub enum CustomizerValueStatus {
        /// The status has not been specified.
        Unspecified = 0,
        /// The received value is not known in this version.
        Unknown = 1,
        /// The customizer value is enabled.
        Enabled = 2,
        /// The customizer value is removed.
        Removed = 3,
    }
    impl CustomizerValueStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                CustomizerValueStatus::Unspecified => "UNSPECIFIED",
                CustomizerValueStatus::Unknown => "UNKNOWN",
                CustomizerValueStatus::Enabled => "ENABLED",
                CustomizerValueStatus::Removed => "REMOVED",
            }
        }
    }
}
/// Container for enum indicating data driven model status.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DataDrivenModelStatusEnum {}
/// Nested message and enum types in `DataDrivenModelStatusEnum`.
pub mod data_driven_model_status_enum {
    /// Enumerates data driven model statuses.
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
    impl DataDrivenModelStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                DataDrivenModelStatus::Unspecified => "UNSPECIFIED",
                DataDrivenModelStatus::Unknown => "UNKNOWN",
                DataDrivenModelStatus::Available => "AVAILABLE",
                DataDrivenModelStatus::Stale => "STALE",
                DataDrivenModelStatus::Expired => "EXPIRED",
                DataDrivenModelStatus::NeverGenerated => "NEVER_GENERATED",
            }
        }
    }
}
/// Container for distance buckets of a user's distance from an advertiser's
/// location extension.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DistanceBucketEnum {}
/// Nested message and enum types in `DistanceBucketEnum`.
pub mod distance_bucket_enum {
    /// The distance bucket for a user's distance from an advertiser's location
    /// extension.
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
    impl DistanceBucket {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                DistanceBucket::Unspecified => "UNSPECIFIED",
                DistanceBucket::Unknown => "UNKNOWN",
                DistanceBucket::Within700m => "WITHIN_700M",
                DistanceBucket::Within1km => "WITHIN_1KM",
                DistanceBucket::Within5km => "WITHIN_5KM",
                DistanceBucket::Within10km => "WITHIN_10KM",
                DistanceBucket::Within15km => "WITHIN_15KM",
                DistanceBucket::Within20km => "WITHIN_20KM",
                DistanceBucket::Within25km => "WITHIN_25KM",
                DistanceBucket::Within30km => "WITHIN_30KM",
                DistanceBucket::Within35km => "WITHIN_35KM",
                DistanceBucket::Within40km => "WITHIN_40KM",
                DistanceBucket::Within45km => "WITHIN_45KM",
                DistanceBucket::Within50km => "WITHIN_50KM",
                DistanceBucket::Within55km => "WITHIN_55KM",
                DistanceBucket::Within60km => "WITHIN_60KM",
                DistanceBucket::Within65km => "WITHIN_65KM",
                DistanceBucket::Beyond65km => "BEYOND_65KM",
                DistanceBucket::Within07miles => "WITHIN_0_7MILES",
                DistanceBucket::Within1mile => "WITHIN_1MILE",
                DistanceBucket::Within5miles => "WITHIN_5MILES",
                DistanceBucket::Within10miles => "WITHIN_10MILES",
                DistanceBucket::Within15miles => "WITHIN_15MILES",
                DistanceBucket::Within20miles => "WITHIN_20MILES",
                DistanceBucket::Within25miles => "WITHIN_25MILES",
                DistanceBucket::Within30miles => "WITHIN_30MILES",
                DistanceBucket::Within35miles => "WITHIN_35MILES",
                DistanceBucket::Within40miles => "WITHIN_40MILES",
                DistanceBucket::Beyond40miles => "BEYOND_40MILES",
            }
        }
    }
}
/// Values for Dynamic Search Ad Page Feed criterion fields.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DsaPageFeedCriterionFieldEnum {}
/// Nested message and enum types in `DsaPageFeedCriterionFieldEnum`.
pub mod dsa_page_feed_criterion_field_enum {
    /// Possible values for Dynamic Search Ad Page Feed criterion fields.
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
    impl DsaPageFeedCriterionField {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                DsaPageFeedCriterionField::Unspecified => "UNSPECIFIED",
                DsaPageFeedCriterionField::Unknown => "UNKNOWN",
                DsaPageFeedCriterionField::PageUrl => "PAGE_URL",
                DsaPageFeedCriterionField::Label => "LABEL",
            }
        }
    }
}
/// Values for Education placeholder fields.
/// For more information about dynamic remarketing feeds, see
/// <https://support.google.com/google-ads/answer/6053288.>
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EducationPlaceholderFieldEnum {}
/// Nested message and enum types in `EducationPlaceholderFieldEnum`.
pub mod education_placeholder_field_enum {
    /// Possible values for Education placeholder fields.
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
        /// Upgraded URLs; the more specific the better (for example, the individual
        /// URL of a specific program and its location).
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
        ///    scheme.
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
    impl EducationPlaceholderField {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                EducationPlaceholderField::Unspecified => "UNSPECIFIED",
                EducationPlaceholderField::Unknown => "UNKNOWN",
                EducationPlaceholderField::ProgramId => "PROGRAM_ID",
                EducationPlaceholderField::LocationId => "LOCATION_ID",
                EducationPlaceholderField::ProgramName => "PROGRAM_NAME",
                EducationPlaceholderField::AreaOfStudy => "AREA_OF_STUDY",
                EducationPlaceholderField::ProgramDescription => "PROGRAM_DESCRIPTION",
                EducationPlaceholderField::SchoolName => "SCHOOL_NAME",
                EducationPlaceholderField::Address => "ADDRESS",
                EducationPlaceholderField::ThumbnailImageUrl => "THUMBNAIL_IMAGE_URL",
                EducationPlaceholderField::AlternativeThumbnailImageUrl => {
                    "ALTERNATIVE_THUMBNAIL_IMAGE_URL"
                }
                EducationPlaceholderField::FinalUrls => "FINAL_URLS",
                EducationPlaceholderField::FinalMobileUrls => "FINAL_MOBILE_URLS",
                EducationPlaceholderField::TrackingUrl => "TRACKING_URL",
                EducationPlaceholderField::ContextualKeywords => "CONTEXTUAL_KEYWORDS",
                EducationPlaceholderField::AndroidAppLink => "ANDROID_APP_LINK",
                EducationPlaceholderField::SimilarProgramIds => "SIMILAR_PROGRAM_IDS",
                EducationPlaceholderField::IosAppLink => "IOS_APP_LINK",
                EducationPlaceholderField::IosAppStoreId => "IOS_APP_STORE_ID",
            }
        }
    }
}
/// Container for enum describing the experiment status.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExperimentStatusEnum {}
/// Nested message and enum types in `ExperimentStatusEnum`.
pub mod experiment_status_enum {
    /// The status of the experiment.
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
    pub enum ExperimentStatus {
        /// Not specified.
        Unspecified = 0,
        /// The value is unknown in this version.
        Unknown = 1,
        /// The experiment is enabled.
        Enabled = 2,
        /// The experiment has been removed.
        Removed = 3,
        /// The experiment has been halted.
        /// This status can be set from ENABLED status through API.
        Halted = 4,
        /// The experiment will be promoted out of experimental status.
        Promoted = 5,
        /// Initial status of the experiment.
        Setup = 6,
        /// The experiment's campaigns are pending materialization.
        /// This status can be set from SETUP status through API.
        Initiated = 7,
        /// The experiment has been graduated.
        Graduated = 8,
    }
    impl ExperimentStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ExperimentStatus::Unspecified => "UNSPECIFIED",
                ExperimentStatus::Unknown => "UNKNOWN",
                ExperimentStatus::Enabled => "ENABLED",
                ExperimentStatus::Removed => "REMOVED",
                ExperimentStatus::Halted => "HALTED",
                ExperimentStatus::Promoted => "PROMOTED",
                ExperimentStatus::Setup => "SETUP",
                ExperimentStatus::Initiated => "INITIATED",
                ExperimentStatus::Graduated => "GRADUATED",
            }
        }
    }
}
/// Container for enum describing the type of experiment.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExperimentTypeEnum {}
/// Nested message and enum types in `ExperimentTypeEnum`.
pub mod experiment_type_enum {
    /// The type of the experiment.
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
    pub enum ExperimentType {
        /// Not specified.
        Unspecified = 0,
        /// The value is unknown in this version.
        Unknown = 1,
        /// This is a DISPLAY_AND_VIDEO_360 experiment.
        DisplayAndVideo360 = 2,
        /// This is an ad variation experiment.
        AdVariation = 3,
        /// A custom experiment consisting of Video campaigns.
        YoutubeCustom = 5,
        /// A custom experiment consisting of display campaigns.
        DisplayCustom = 6,
        /// A custom experiment consisting of search campaigns.
        SearchCustom = 7,
        /// An experiment that compares bidding strategies for display campaigns.
        DisplayAutomatedBiddingStrategy = 8,
        /// An experiment that compares bidding strategies for search campaigns."
        SearchAutomatedBiddingStrategy = 9,
        /// An experiment that compares bidding strategies for shopping campaigns.
        ShoppingAutomatedBiddingStrategy = 10,
        /// DEPRECATED. A smart matching experiment with search campaigns.
        SmartMatching = 11,
        /// A custom experiment consisting of hotel campaigns.
        HotelCustom = 12,
    }
    impl ExperimentType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ExperimentType::Unspecified => "UNSPECIFIED",
                ExperimentType::Unknown => "UNKNOWN",
                ExperimentType::DisplayAndVideo360 => "DISPLAY_AND_VIDEO_360",
                ExperimentType::AdVariation => "AD_VARIATION",
                ExperimentType::YoutubeCustom => "YOUTUBE_CUSTOM",
                ExperimentType::DisplayCustom => "DISPLAY_CUSTOM",
                ExperimentType::SearchCustom => "SEARCH_CUSTOM",
                ExperimentType::DisplayAutomatedBiddingStrategy => {
                    "DISPLAY_AUTOMATED_BIDDING_STRATEGY"
                }
                ExperimentType::SearchAutomatedBiddingStrategy => {
                    "SEARCH_AUTOMATED_BIDDING_STRATEGY"
                }
                ExperimentType::ShoppingAutomatedBiddingStrategy => {
                    "SHOPPING_AUTOMATED_BIDDING_STRATEGY"
                }
                ExperimentType::SmartMatching => "SMART_MATCHING",
                ExperimentType::HotelCustom => "HOTEL_CUSTOM",
            }
        }
    }
}
/// Container for enum describing extension setting device types.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExtensionSettingDeviceEnum {}
/// Nested message and enum types in `ExtensionSettingDeviceEnum`.
pub mod extension_setting_device_enum {
    /// Possible device types for an extension setting.
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
    impl ExtensionSettingDevice {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ExtensionSettingDevice::Unspecified => "UNSPECIFIED",
                ExtensionSettingDevice::Unknown => "UNKNOWN",
                ExtensionSettingDevice::Mobile => "MOBILE",
                ExtensionSettingDevice::Desktop => "DESKTOP",
            }
        }
    }
}
/// Container for enum describing possible data types for an extension in an
/// extension setting.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExtensionTypeEnum {}
/// Nested message and enum types in `ExtensionTypeEnum`.
pub mod extension_type_enum {
    /// Possible data types for an extension in an extension setting.
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
    impl ExtensionType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ExtensionType::Unspecified => "UNSPECIFIED",
                ExtensionType::Unknown => "UNKNOWN",
                ExtensionType::None => "NONE",
                ExtensionType::App => "APP",
                ExtensionType::Call => "CALL",
                ExtensionType::Callout => "CALLOUT",
                ExtensionType::Message => "MESSAGE",
                ExtensionType::Price => "PRICE",
                ExtensionType::Promotion => "PROMOTION",
                ExtensionType::Sitelink => "SITELINK",
                ExtensionType::StructuredSnippet => "STRUCTURED_SNIPPET",
                ExtensionType::Location => "LOCATION",
                ExtensionType::AffiliateLocation => "AFFILIATE_LOCATION",
                ExtensionType::HotelCallout => "HOTEL_CALLOUT",
                ExtensionType::Image => "IMAGE",
            }
        }
    }
}
/// Container for enum describing possible data types for a feed attribute.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeedAttributeTypeEnum {}
/// Nested message and enum types in `FeedAttributeTypeEnum`.
pub mod feed_attribute_type_enum {
    /// Possible data types for a feed attribute.
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
    impl FeedAttributeType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                FeedAttributeType::Unspecified => "UNSPECIFIED",
                FeedAttributeType::Unknown => "UNKNOWN",
                FeedAttributeType::Int64 => "INT64",
                FeedAttributeType::Double => "DOUBLE",
                FeedAttributeType::String => "STRING",
                FeedAttributeType::Boolean => "BOOLEAN",
                FeedAttributeType::Url => "URL",
                FeedAttributeType::DateTime => "DATE_TIME",
                FeedAttributeType::Int64List => "INT64_LIST",
                FeedAttributeType::DoubleList => "DOUBLE_LIST",
                FeedAttributeType::StringList => "STRING_LIST",
                FeedAttributeType::BooleanList => "BOOLEAN_LIST",
                FeedAttributeType::UrlList => "URL_LIST",
                FeedAttributeType::DateTimeList => "DATE_TIME_LIST",
                FeedAttributeType::Price => "PRICE",
            }
        }
    }
}
/// Container for enum describing possible quality evaluation approval statuses
/// of a feed item.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeedItemQualityApprovalStatusEnum {}
/// Nested message and enum types in `FeedItemQualityApprovalStatusEnum`.
pub mod feed_item_quality_approval_status_enum {
    /// The possible quality evaluation approval statuses of a feed item.
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
    impl FeedItemQualityApprovalStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                FeedItemQualityApprovalStatus::Unspecified => "UNSPECIFIED",
                FeedItemQualityApprovalStatus::Unknown => "UNKNOWN",
                FeedItemQualityApprovalStatus::Approved => "APPROVED",
                FeedItemQualityApprovalStatus::Disapproved => "DISAPPROVED",
            }
        }
    }
}
/// Container for enum describing possible quality evaluation disapproval reasons
/// of a feed item.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeedItemQualityDisapprovalReasonEnum {}
/// Nested message and enum types in `FeedItemQualityDisapprovalReasonEnum`.
pub mod feed_item_quality_disapproval_reason_enum {
    /// The possible quality evaluation disapproval reasons of a feed item.
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
    impl FeedItemQualityDisapprovalReason {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                FeedItemQualityDisapprovalReason::Unspecified => "UNSPECIFIED",
                FeedItemQualityDisapprovalReason::Unknown => "UNKNOWN",
                FeedItemQualityDisapprovalReason::PriceTableRepetitiveHeaders => {
                    "PRICE_TABLE_REPETITIVE_HEADERS"
                }
                FeedItemQualityDisapprovalReason::PriceTableRepetitiveDescription => {
                    "PRICE_TABLE_REPETITIVE_DESCRIPTION"
                }
                FeedItemQualityDisapprovalReason::PriceTableInconsistentRows => {
                    "PRICE_TABLE_INCONSISTENT_ROWS"
                }
                FeedItemQualityDisapprovalReason::PriceDescriptionHasPriceQualifiers => {
                    "PRICE_DESCRIPTION_HAS_PRICE_QUALIFIERS"
                }
                FeedItemQualityDisapprovalReason::PriceUnsupportedLanguage => {
                    "PRICE_UNSUPPORTED_LANGUAGE"
                }
                FeedItemQualityDisapprovalReason::PriceTableRowHeaderTableTypeMismatch => {
                    "PRICE_TABLE_ROW_HEADER_TABLE_TYPE_MISMATCH"
                }
                FeedItemQualityDisapprovalReason::PriceTableRowHeaderHasPromotionalText => {
                    "PRICE_TABLE_ROW_HEADER_HAS_PROMOTIONAL_TEXT"
                }
                FeedItemQualityDisapprovalReason::PriceTableRowDescriptionNotRelevant => {
                    "PRICE_TABLE_ROW_DESCRIPTION_NOT_RELEVANT"
                }
                FeedItemQualityDisapprovalReason::PriceTableRowDescriptionHasPromotionalText => {
                    "PRICE_TABLE_ROW_DESCRIPTION_HAS_PROMOTIONAL_TEXT"
                }
                FeedItemQualityDisapprovalReason::PriceTableRowHeaderDescriptionRepetitive => {
                    "PRICE_TABLE_ROW_HEADER_DESCRIPTION_REPETITIVE"
                }
                FeedItemQualityDisapprovalReason::PriceTableRowUnrateable => {
                    "PRICE_TABLE_ROW_UNRATEABLE"
                }
                FeedItemQualityDisapprovalReason::PriceTableRowPriceInvalid => {
                    "PRICE_TABLE_ROW_PRICE_INVALID"
                }
                FeedItemQualityDisapprovalReason::PriceTableRowUrlInvalid => {
                    "PRICE_TABLE_ROW_URL_INVALID"
                }
                FeedItemQualityDisapprovalReason::PriceHeaderOrDescriptionHasPrice => {
                    "PRICE_HEADER_OR_DESCRIPTION_HAS_PRICE"
                }
                FeedItemQualityDisapprovalReason::StructuredSnippetsHeaderPolicyViolated => {
                    "STRUCTURED_SNIPPETS_HEADER_POLICY_VIOLATED"
                }
                FeedItemQualityDisapprovalReason::StructuredSnippetsRepeatedValues => {
                    "STRUCTURED_SNIPPETS_REPEATED_VALUES"
                }
                FeedItemQualityDisapprovalReason::StructuredSnippetsEditorialGuidelines => {
                    "STRUCTURED_SNIPPETS_EDITORIAL_GUIDELINES"
                }
                FeedItemQualityDisapprovalReason::StructuredSnippetsHasPromotionalText => {
                    "STRUCTURED_SNIPPETS_HAS_PROMOTIONAL_TEXT"
                }
            }
        }
    }
}
/// Container for enum describing possible statuses of a feed item set.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeedItemSetStatusEnum {}
/// Nested message and enum types in `FeedItemSetStatusEnum`.
pub mod feed_item_set_status_enum {
    /// Possible statuses of a feed item set.
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
    impl FeedItemSetStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                FeedItemSetStatus::Unspecified => "UNSPECIFIED",
                FeedItemSetStatus::Unknown => "UNKNOWN",
                FeedItemSetStatus::Enabled => "ENABLED",
                FeedItemSetStatus::Removed => "REMOVED",
            }
        }
    }
}
/// Container for enum describing possible statuses of a feed item.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeedItemStatusEnum {}
/// Nested message and enum types in `FeedItemStatusEnum`.
pub mod feed_item_status_enum {
    /// Possible statuses of a feed item.
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
    impl FeedItemStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                FeedItemStatus::Unspecified => "UNSPECIFIED",
                FeedItemStatus::Unknown => "UNKNOWN",
                FeedItemStatus::Enabled => "ENABLED",
                FeedItemStatus::Removed => "REMOVED",
            }
        }
    }
}
/// Container for enum describing possible data types for a feed item target
/// device.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeedItemTargetDeviceEnum {}
/// Nested message and enum types in `FeedItemTargetDeviceEnum`.
pub mod feed_item_target_device_enum {
    /// Possible data types for a feed item target device.
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
    pub enum FeedItemTargetDevice {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Mobile.
        Mobile = 2,
    }
    impl FeedItemTargetDevice {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                FeedItemTargetDevice::Unspecified => "UNSPECIFIED",
                FeedItemTargetDevice::Unknown => "UNKNOWN",
                FeedItemTargetDevice::Mobile => "MOBILE",
            }
        }
    }
}
/// Container for enum describing possible statuses of a feed item target.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeedItemTargetStatusEnum {}
/// Nested message and enum types in `FeedItemTargetStatusEnum`.
pub mod feed_item_target_status_enum {
    /// Possible statuses of a feed item target.
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
    impl FeedItemTargetStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                FeedItemTargetStatus::Unspecified => "UNSPECIFIED",
                FeedItemTargetStatus::Unknown => "UNKNOWN",
                FeedItemTargetStatus::Enabled => "ENABLED",
                FeedItemTargetStatus::Removed => "REMOVED",
            }
        }
    }
}
/// Container for enum describing possible types of a feed item target.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeedItemTargetTypeEnum {}
/// Nested message and enum types in `FeedItemTargetTypeEnum`.
pub mod feed_item_target_type_enum {
    /// Possible type of a feed item target.
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
    impl FeedItemTargetType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                FeedItemTargetType::Unspecified => "UNSPECIFIED",
                FeedItemTargetType::Unknown => "UNKNOWN",
                FeedItemTargetType::Campaign => "CAMPAIGN",
                FeedItemTargetType::AdGroup => "AD_GROUP",
                FeedItemTargetType::Criterion => "CRITERION",
            }
        }
    }
}
/// Container for enum describing possible validation statuses of a feed item.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeedItemValidationStatusEnum {}
/// Nested message and enum types in `FeedItemValidationStatusEnum`.
pub mod feed_item_validation_status_enum {
    /// The possible validation statuses of a feed item.
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
    impl FeedItemValidationStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                FeedItemValidationStatus::Unspecified => "UNSPECIFIED",
                FeedItemValidationStatus::Unknown => "UNKNOWN",
                FeedItemValidationStatus::Pending => "PENDING",
                FeedItemValidationStatus::Invalid => "INVALID",
                FeedItemValidationStatus::Valid => "VALID",
            }
        }
    }
}
/// Container for an enum describing possible statuses of a feed link.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeedLinkStatusEnum {}
/// Nested message and enum types in `FeedLinkStatusEnum`.
pub mod feed_link_status_enum {
    /// Possible statuses of a feed link.
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
    impl FeedLinkStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                FeedLinkStatus::Unspecified => "UNSPECIFIED",
                FeedLinkStatus::Unknown => "UNKNOWN",
                FeedLinkStatus::Enabled => "ENABLED",
                FeedLinkStatus::Removed => "REMOVED",
            }
        }
    }
}
/// Container for enum describing possible criterion types for a feed mapping.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeedMappingCriterionTypeEnum {}
/// Nested message and enum types in `FeedMappingCriterionTypeEnum`.
pub mod feed_mapping_criterion_type_enum {
    /// Possible placeholder types for a feed mapping.
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
    impl FeedMappingCriterionType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                FeedMappingCriterionType::Unspecified => "UNSPECIFIED",
                FeedMappingCriterionType::Unknown => "UNKNOWN",
                FeedMappingCriterionType::LocationExtensionTargeting => {
                    "LOCATION_EXTENSION_TARGETING"
                }
                FeedMappingCriterionType::DsaPageFeed => "DSA_PAGE_FEED",
            }
        }
    }
}
/// Container for enum describing possible statuses of a feed mapping.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeedMappingStatusEnum {}
/// Nested message and enum types in `FeedMappingStatusEnum`.
pub mod feed_mapping_status_enum {
    /// Possible statuses of a feed mapping.
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
    impl FeedMappingStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                FeedMappingStatus::Unspecified => "UNSPECIFIED",
                FeedMappingStatus::Unknown => "UNKNOWN",
                FeedMappingStatus::Enabled => "ENABLED",
                FeedMappingStatus::Removed => "REMOVED",
            }
        }
    }
}
/// Container for enum describing possible values for a feed origin.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeedOriginEnum {}
/// Nested message and enum types in `FeedOriginEnum`.
pub mod feed_origin_enum {
    /// Possible values for a feed origin.
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
    impl FeedOrigin {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                FeedOrigin::Unspecified => "UNSPECIFIED",
                FeedOrigin::Unknown => "UNKNOWN",
                FeedOrigin::User => "USER",
                FeedOrigin::Google => "GOOGLE",
            }
        }
    }
}
/// Container for enum describing possible statuses of a feed.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeedStatusEnum {}
/// Nested message and enum types in `FeedStatusEnum`.
pub mod feed_status_enum {
    /// Possible statuses of a feed.
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
    impl FeedStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                FeedStatus::Unspecified => "UNSPECIFIED",
                FeedStatus::Unknown => "UNKNOWN",
                FeedStatus::Enabled => "ENABLED",
                FeedStatus::Removed => "REMOVED",
            }
        }
    }
}
/// Values for Flight placeholder fields.
/// For more information about dynamic remarketing feeds, see
/// <https://support.google.com/google-ads/answer/6053288.>
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FlightPlaceholderFieldEnum {}
/// Nested message and enum types in `FlightPlaceholderFieldEnum`.
pub mod flight_placeholder_field_enum {
    /// Possible values for Flight placeholder fields.
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
        ///    scheme.
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
    impl FlightPlaceholderField {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                FlightPlaceholderField::Unspecified => "UNSPECIFIED",
                FlightPlaceholderField::Unknown => "UNKNOWN",
                FlightPlaceholderField::DestinationId => "DESTINATION_ID",
                FlightPlaceholderField::OriginId => "ORIGIN_ID",
                FlightPlaceholderField::FlightDescription => "FLIGHT_DESCRIPTION",
                FlightPlaceholderField::OriginName => "ORIGIN_NAME",
                FlightPlaceholderField::DestinationName => "DESTINATION_NAME",
                FlightPlaceholderField::FlightPrice => "FLIGHT_PRICE",
                FlightPlaceholderField::FormattedPrice => "FORMATTED_PRICE",
                FlightPlaceholderField::FlightSalePrice => "FLIGHT_SALE_PRICE",
                FlightPlaceholderField::FormattedSalePrice => "FORMATTED_SALE_PRICE",
                FlightPlaceholderField::ImageUrl => "IMAGE_URL",
                FlightPlaceholderField::FinalUrls => "FINAL_URLS",
                FlightPlaceholderField::FinalMobileUrls => "FINAL_MOBILE_URLS",
                FlightPlaceholderField::TrackingUrl => "TRACKING_URL",
                FlightPlaceholderField::AndroidAppLink => "ANDROID_APP_LINK",
                FlightPlaceholderField::SimilarDestinationIds => {
                    "SIMILAR_DESTINATION_IDS"
                }
                FlightPlaceholderField::IosAppLink => "IOS_APP_LINK",
                FlightPlaceholderField::IosAppStoreId => "IOS_APP_STORE_ID",
            }
        }
    }
}
/// Container for describing the status of a geo target constant.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GeoTargetConstantStatusEnum {}
/// Nested message and enum types in `GeoTargetConstantStatusEnum`.
pub mod geo_target_constant_status_enum {
    /// The possible statuses of a geo target constant.
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
    impl GeoTargetConstantStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                GeoTargetConstantStatus::Unspecified => "UNSPECIFIED",
                GeoTargetConstantStatus::Unknown => "UNKNOWN",
                GeoTargetConstantStatus::Enabled => "ENABLED",
                GeoTargetConstantStatus::RemovalPlanned => "REMOVAL_PLANNED",
            }
        }
    }
}
/// Message describing feed item geo targeting restriction.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GeoTargetingRestrictionEnum {}
/// Nested message and enum types in `GeoTargetingRestrictionEnum`.
pub mod geo_targeting_restriction_enum {
    /// A restriction used to determine if the request context's
    /// geo should be matched.
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
    pub enum GeoTargetingRestriction {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Indicates that request context should match the physical location of
        /// the user.
        LocationOfPresence = 2,
    }
    impl GeoTargetingRestriction {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                GeoTargetingRestriction::Unspecified => "UNSPECIFIED",
                GeoTargetingRestriction::Unknown => "UNKNOWN",
                GeoTargetingRestriction::LocationOfPresence => "LOCATION_OF_PRESENCE",
            }
        }
    }
}
/// Container for enum describing possible geo targeting types.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GeoTargetingTypeEnum {}
/// Nested message and enum types in `GeoTargetingTypeEnum`.
pub mod geo_targeting_type_enum {
    /// The possible geo targeting types.
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
    impl GeoTargetingType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                GeoTargetingType::Unspecified => "UNSPECIFIED",
                GeoTargetingType::Unknown => "UNKNOWN",
                GeoTargetingType::AreaOfInterest => "AREA_OF_INTEREST",
                GeoTargetingType::LocationOfPresence => "LOCATION_OF_PRESENCE",
            }
        }
    }
}
/// Container for enum describing possible goal config levels.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GoalConfigLevelEnum {}
/// Nested message and enum types in `GoalConfigLevelEnum`.
pub mod goal_config_level_enum {
    /// The possible goal config levels. Campaigns automatically inherit the
    /// effective conversion account's customer goals unless they have been
    /// configured with their own set of campaign goals.
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
    pub enum GoalConfigLevel {
        /// The goal config level has not been specified.
        Unspecified = 0,
        /// The goal config level is not known in this version.
        Unknown = 1,
        /// The goal config is defined at the customer level.
        Customer = 2,
        /// The goal config is defined at the campaign level.
        Campaign = 3,
    }
    impl GoalConfigLevel {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                GoalConfigLevel::Unspecified => "UNSPECIFIED",
                GoalConfigLevel::Unknown => "UNKNOWN",
                GoalConfigLevel::Customer => "CUSTOMER",
                GoalConfigLevel::Campaign => "CAMPAIGN",
            }
        }
    }
}
/// Container for enum that determines if the described artifact is a resource
/// or a field, and if it is a field, when it segments search queries.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GoogleAdsFieldCategoryEnum {}
/// Nested message and enum types in `GoogleAdsFieldCategoryEnum`.
pub mod google_ads_field_category_enum {
    /// The category of the artifact.
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
    impl GoogleAdsFieldCategory {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                GoogleAdsFieldCategory::Unspecified => "UNSPECIFIED",
                GoogleAdsFieldCategory::Unknown => "UNKNOWN",
                GoogleAdsFieldCategory::Resource => "RESOURCE",
                GoogleAdsFieldCategory::Attribute => "ATTRIBUTE",
                GoogleAdsFieldCategory::Segment => "SEGMENT",
                GoogleAdsFieldCategory::Metric => "METRIC",
            }
        }
    }
}
/// Container holding the various data types.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GoogleAdsFieldDataTypeEnum {}
/// Nested message and enum types in `GoogleAdsFieldDataTypeEnum`.
pub mod google_ads_field_data_type_enum {
    /// These are the various types a GoogleAdsService artifact may take on.
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
    impl GoogleAdsFieldDataType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                GoogleAdsFieldDataType::Unspecified => "UNSPECIFIED",
                GoogleAdsFieldDataType::Unknown => "UNKNOWN",
                GoogleAdsFieldDataType::Boolean => "BOOLEAN",
                GoogleAdsFieldDataType::Date => "DATE",
                GoogleAdsFieldDataType::Double => "DOUBLE",
                GoogleAdsFieldDataType::Enum => "ENUM",
                GoogleAdsFieldDataType::Float => "FLOAT",
                GoogleAdsFieldDataType::Int32 => "INT32",
                GoogleAdsFieldDataType::Int64 => "INT64",
                GoogleAdsFieldDataType::Message => "MESSAGE",
                GoogleAdsFieldDataType::ResourceName => "RESOURCE_NAME",
                GoogleAdsFieldDataType::String => "STRING",
                GoogleAdsFieldDataType::Uint64 => "UINT64",
            }
        }
    }
}
/// Container for enum describing possible statuses of a google voice call.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GoogleVoiceCallStatusEnum {}
/// Nested message and enum types in `GoogleVoiceCallStatusEnum`.
pub mod google_voice_call_status_enum {
    /// Possible statuses of a google voice call.
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
    impl GoogleVoiceCallStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                GoogleVoiceCallStatus::Unspecified => "UNSPECIFIED",
                GoogleVoiceCallStatus::Unknown => "UNKNOWN",
                GoogleVoiceCallStatus::Missed => "MISSED",
                GoogleVoiceCallStatus::Received => "RECEIVED",
            }
        }
    }
}
/// Values for Hotel placeholder fields.
/// For more information about dynamic remarketing feeds, see
/// <https://support.google.com/google-ads/answer/6053288.>
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HotelPlaceholderFieldEnum {}
/// Nested message and enum types in `HotelPlaceholderFieldEnum`.
pub mod hotel_placeholder_field_enum {
    /// Possible values for Hotel placeholder fields.
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
        ///    scheme.
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
    impl HotelPlaceholderField {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                HotelPlaceholderField::Unspecified => "UNSPECIFIED",
                HotelPlaceholderField::Unknown => "UNKNOWN",
                HotelPlaceholderField::PropertyId => "PROPERTY_ID",
                HotelPlaceholderField::PropertyName => "PROPERTY_NAME",
                HotelPlaceholderField::DestinationName => "DESTINATION_NAME",
                HotelPlaceholderField::Description => "DESCRIPTION",
                HotelPlaceholderField::Address => "ADDRESS",
                HotelPlaceholderField::Price => "PRICE",
                HotelPlaceholderField::FormattedPrice => "FORMATTED_PRICE",
                HotelPlaceholderField::SalePrice => "SALE_PRICE",
                HotelPlaceholderField::FormattedSalePrice => "FORMATTED_SALE_PRICE",
                HotelPlaceholderField::ImageUrl => "IMAGE_URL",
                HotelPlaceholderField::Category => "CATEGORY",
                HotelPlaceholderField::StarRating => "STAR_RATING",
                HotelPlaceholderField::ContextualKeywords => "CONTEXTUAL_KEYWORDS",
                HotelPlaceholderField::FinalUrls => "FINAL_URLS",
                HotelPlaceholderField::FinalMobileUrls => "FINAL_MOBILE_URLS",
                HotelPlaceholderField::TrackingUrl => "TRACKING_URL",
                HotelPlaceholderField::AndroidAppLink => "ANDROID_APP_LINK",
                HotelPlaceholderField::SimilarPropertyIds => "SIMILAR_PROPERTY_IDS",
                HotelPlaceholderField::IosAppLink => "IOS_APP_LINK",
                HotelPlaceholderField::IosAppStoreId => "IOS_APP_STORE_ID",
            }
        }
    }
}
/// Container for HotelReconciliationStatus.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HotelReconciliationStatusEnum {}
/// Nested message and enum types in `HotelReconciliationStatusEnum`.
pub mod hotel_reconciliation_status_enum {
    /// Status of the hotel booking reconciliation.
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
    pub enum HotelReconciliationStatus {
        /// Not specified.
        Unspecified = 0,
        /// The value is unknown in this version.
        Unknown = 1,
        /// Bookings are for a future date, or a stay is underway but the check-out
        /// date hasn't passed. An active reservation can't be reconciled.
        ReservationEnabled = 2,
        /// Check-out has already taken place, or the booked dates have passed
        /// without cancellation. Bookings that are not reconciled within 45 days of
        /// the check-out date are billed based on the original booking price.
        ReconciliationNeeded = 3,
        /// These bookings have been reconciled. Reconciled bookings are billed 45
        /// days after the check-out date.
        Reconciled = 4,
        /// This booking was marked as canceled. Canceled stays with a value greater
        /// than zero (due to minimum stay rules or cancellation fees) are billed 45
        /// days after the check-out date.
        Canceled = 5,
    }
    impl HotelReconciliationStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                HotelReconciliationStatus::Unspecified => "UNSPECIFIED",
                HotelReconciliationStatus::Unknown => "UNKNOWN",
                HotelReconciliationStatus::ReservationEnabled => "RESERVATION_ENABLED",
                HotelReconciliationStatus::ReconciliationNeeded => {
                    "RECONCILIATION_NEEDED"
                }
                HotelReconciliationStatus::Reconciled => "RECONCILED",
                HotelReconciliationStatus::Canceled => "CANCELED",
            }
        }
    }
}
/// Values for Advertiser Provided Image placeholder fields.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImagePlaceholderFieldEnum {}
/// Nested message and enum types in `ImagePlaceholderFieldEnum`.
pub mod image_placeholder_field_enum {
    /// Possible values for Advertiser Provided Image placeholder fields.
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
    pub enum ImagePlaceholderField {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Data Type: INT64. The asset ID of the image.
        AssetId = 2,
    }
    impl ImagePlaceholderField {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ImagePlaceholderField::Unspecified => "UNSPECIFIED",
                ImagePlaceholderField::Unknown => "UNKNOWN",
                ImagePlaceholderField::AssetId => "ASSET_ID",
            }
        }
    }
}
/// Container for enum describing the type of invoices.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InvoiceTypeEnum {}
/// Nested message and enum types in `InvoiceTypeEnum`.
pub mod invoice_type_enum {
    /// The possible type of invoices.
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
    impl InvoiceType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                InvoiceType::Unspecified => "UNSPECIFIED",
                InvoiceType::Unknown => "UNKNOWN",
                InvoiceType::CreditMemo => "CREDIT_MEMO",
                InvoiceType::Invoice => "INVOICE",
            }
        }
    }
}
/// Values for Job placeholder fields.
/// For more information about dynamic remarketing feeds, see
/// <https://support.google.com/google-ads/answer/6053288.>
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct JobPlaceholderFieldEnum {}
/// Nested message and enum types in `JobPlaceholderFieldEnum`.
pub mod job_placeholder_field_enum {
    /// Possible values for Job placeholder fields.
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
        /// Upgraded URLs; the more specific the better (for example, the individual
        /// URL of a specific job and its location).
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
        ///    scheme.
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
    impl JobPlaceholderField {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                JobPlaceholderField::Unspecified => "UNSPECIFIED",
                JobPlaceholderField::Unknown => "UNKNOWN",
                JobPlaceholderField::JobId => "JOB_ID",
                JobPlaceholderField::LocationId => "LOCATION_ID",
                JobPlaceholderField::Title => "TITLE",
                JobPlaceholderField::Subtitle => "SUBTITLE",
                JobPlaceholderField::Description => "DESCRIPTION",
                JobPlaceholderField::ImageUrl => "IMAGE_URL",
                JobPlaceholderField::Category => "CATEGORY",
                JobPlaceholderField::ContextualKeywords => "CONTEXTUAL_KEYWORDS",
                JobPlaceholderField::Address => "ADDRESS",
                JobPlaceholderField::Salary => "SALARY",
                JobPlaceholderField::FinalUrls => "FINAL_URLS",
                JobPlaceholderField::FinalMobileUrls => "FINAL_MOBILE_URLS",
                JobPlaceholderField::TrackingUrl => "TRACKING_URL",
                JobPlaceholderField::AndroidAppLink => "ANDROID_APP_LINK",
                JobPlaceholderField::SimilarJobIds => "SIMILAR_JOB_IDS",
                JobPlaceholderField::IosAppLink => "IOS_APP_LINK",
                JobPlaceholderField::IosAppStoreId => "IOS_APP_STORE_ID",
            }
        }
    }
}
/// Container for enumeration of forecast intervals.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeywordPlanForecastIntervalEnum {}
/// Nested message and enum types in `KeywordPlanForecastIntervalEnum`.
pub mod keyword_plan_forecast_interval_enum {
    /// Forecast intervals.
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
    impl KeywordPlanForecastInterval {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                KeywordPlanForecastInterval::Unspecified => "UNSPECIFIED",
                KeywordPlanForecastInterval::Unknown => "UNKNOWN",
                KeywordPlanForecastInterval::NextWeek => "NEXT_WEEK",
                KeywordPlanForecastInterval::NextMonth => "NEXT_MONTH",
                KeywordPlanForecastInterval::NextQuarter => "NEXT_QUARTER",
            }
        }
    }
}
/// Container for enumeration of keyword plan keyword annotations.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeywordPlanKeywordAnnotationEnum {}
/// Nested message and enum types in `KeywordPlanKeywordAnnotationEnum`.
pub mod keyword_plan_keyword_annotation_enum {
    /// Enumerates keyword plan annotations that can be requested.
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
    pub enum KeywordPlanKeywordAnnotation {
        /// Not specified.
        Unspecified = 0,
        /// The value is unknown in this version.
        Unknown = 1,
        /// Return the keyword concept and concept group data.
        KeywordConcept = 2,
    }
    impl KeywordPlanKeywordAnnotation {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                KeywordPlanKeywordAnnotation::Unspecified => "UNSPECIFIED",
                KeywordPlanKeywordAnnotation::Unknown => "UNKNOWN",
                KeywordPlanKeywordAnnotation::KeywordConcept => "KEYWORD_CONCEPT",
            }
        }
    }
}
/// Container for enumeration of keyword plan forecastable network types.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeywordPlanNetworkEnum {}
/// Nested message and enum types in `KeywordPlanNetworkEnum`.
pub mod keyword_plan_network_enum {
    /// Enumerates keyword plan forecastable network types.
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
    impl KeywordPlanNetwork {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                KeywordPlanNetwork::Unspecified => "UNSPECIFIED",
                KeywordPlanNetwork::Unknown => "UNKNOWN",
                KeywordPlanNetwork::GoogleSearch => "GOOGLE_SEARCH",
                KeywordPlanNetwork::GoogleSearchAndPartners => {
                    "GOOGLE_SEARCH_AND_PARTNERS"
                }
            }
        }
    }
}
/// Container for enum describing possible status of a label.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LabelStatusEnum {}
/// Nested message and enum types in `LabelStatusEnum`.
pub mod label_status_enum {
    /// Possible statuses of a label.
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
    impl LabelStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                LabelStatus::Unspecified => "UNSPECIFIED",
                LabelStatus::Unknown => "UNKNOWN",
                LabelStatus::Enabled => "ENABLED",
                LabelStatus::Removed => "REMOVED",
            }
        }
    }
}
/// Container for enum describing different types of Linked accounts.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LinkedAccountTypeEnum {}
/// Nested message and enum types in `LinkedAccountTypeEnum`.
pub mod linked_account_type_enum {
    /// Describes the possible link types between a Google Ads customer
    /// and another account.
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
        /// A link to Hotel Center.
        HotelCenter = 5,
    }
    impl LinkedAccountType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                LinkedAccountType::Unspecified => "UNSPECIFIED",
                LinkedAccountType::Unknown => "UNKNOWN",
                LinkedAccountType::ThirdPartyAppAnalytics => "THIRD_PARTY_APP_ANALYTICS",
                LinkedAccountType::DataPartner => "DATA_PARTNER",
                LinkedAccountType::GoogleAds => "GOOGLE_ADS",
                LinkedAccountType::HotelCenter => "HOTEL_CENTER",
            }
        }
    }
}
/// Container for enum describing the levels of bidding category used in
/// ListingGroupFilterDimension.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListingGroupFilterBiddingCategoryLevelEnum {}
/// Nested message and enum types in `ListingGroupFilterBiddingCategoryLevelEnum`.
pub mod listing_group_filter_bidding_category_level_enum {
    /// The level of the listing group filter bidding category.
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
    pub enum ListingGroupFilterBiddingCategoryLevel {
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
    impl ListingGroupFilterBiddingCategoryLevel {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ListingGroupFilterBiddingCategoryLevel::Unspecified => "UNSPECIFIED",
                ListingGroupFilterBiddingCategoryLevel::Unknown => "UNKNOWN",
                ListingGroupFilterBiddingCategoryLevel::Level1 => "LEVEL1",
                ListingGroupFilterBiddingCategoryLevel::Level2 => "LEVEL2",
                ListingGroupFilterBiddingCategoryLevel::Level3 => "LEVEL3",
                ListingGroupFilterBiddingCategoryLevel::Level4 => "LEVEL4",
                ListingGroupFilterBiddingCategoryLevel::Level5 => "LEVEL5",
            }
        }
    }
}
/// Container for enum describing the indexes of custom attribute used in
/// ListingGroupFilterDimension.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListingGroupFilterCustomAttributeIndexEnum {}
/// Nested message and enum types in `ListingGroupFilterCustomAttributeIndexEnum`.
pub mod listing_group_filter_custom_attribute_index_enum {
    /// The index of customer attributes.
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
    pub enum ListingGroupFilterCustomAttributeIndex {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// First listing group filter custom attribute.
        Index0 = 2,
        /// Second listing group filter custom attribute.
        Index1 = 3,
        /// Third listing group filter custom attribute.
        Index2 = 4,
        /// Fourth listing group filter custom attribute.
        Index3 = 5,
        /// Fifth listing group filter custom attribute.
        Index4 = 6,
    }
    impl ListingGroupFilterCustomAttributeIndex {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ListingGroupFilterCustomAttributeIndex::Unspecified => "UNSPECIFIED",
                ListingGroupFilterCustomAttributeIndex::Unknown => "UNKNOWN",
                ListingGroupFilterCustomAttributeIndex::Index0 => "INDEX0",
                ListingGroupFilterCustomAttributeIndex::Index1 => "INDEX1",
                ListingGroupFilterCustomAttributeIndex::Index2 => "INDEX2",
                ListingGroupFilterCustomAttributeIndex::Index3 => "INDEX3",
                ListingGroupFilterCustomAttributeIndex::Index4 => "INDEX4",
            }
        }
    }
}
/// Locality of a product offer.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListingGroupFilterProductChannelEnum {}
/// Nested message and enum types in `ListingGroupFilterProductChannelEnum`.
pub mod listing_group_filter_product_channel_enum {
    /// Enum describing the locality of a product offer.
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
    pub enum ListingGroupFilterProductChannel {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// The item is sold online.
        Online = 2,
        /// The item is sold in local stores.
        Local = 3,
    }
    impl ListingGroupFilterProductChannel {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ListingGroupFilterProductChannel::Unspecified => "UNSPECIFIED",
                ListingGroupFilterProductChannel::Unknown => "UNKNOWN",
                ListingGroupFilterProductChannel::Online => "ONLINE",
                ListingGroupFilterProductChannel::Local => "LOCAL",
            }
        }
    }
}
/// Condition of a product offer.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListingGroupFilterProductConditionEnum {}
/// Nested message and enum types in `ListingGroupFilterProductConditionEnum`.
pub mod listing_group_filter_product_condition_enum {
    /// Enum describing the condition of a product offer.
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
    pub enum ListingGroupFilterProductCondition {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// The product condition is new.
        New = 2,
        /// The product condition is refurbished.
        Refurbished = 3,
        /// The product condition is used.
        Used = 4,
    }
    impl ListingGroupFilterProductCondition {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ListingGroupFilterProductCondition::Unspecified => "UNSPECIFIED",
                ListingGroupFilterProductCondition::Unknown => "UNKNOWN",
                ListingGroupFilterProductCondition::New => "NEW",
                ListingGroupFilterProductCondition::Refurbished => "REFURBISHED",
                ListingGroupFilterProductCondition::Used => "USED",
            }
        }
    }
}
/// Level of the type of a product offer.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListingGroupFilterProductTypeLevelEnum {}
/// Nested message and enum types in `ListingGroupFilterProductTypeLevelEnum`.
pub mod listing_group_filter_product_type_level_enum {
    /// Enum describing the level of the type of a product offer.
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
    pub enum ListingGroupFilterProductTypeLevel {
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
    impl ListingGroupFilterProductTypeLevel {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ListingGroupFilterProductTypeLevel::Unspecified => "UNSPECIFIED",
                ListingGroupFilterProductTypeLevel::Unknown => "UNKNOWN",
                ListingGroupFilterProductTypeLevel::Level1 => "LEVEL1",
                ListingGroupFilterProductTypeLevel::Level2 => "LEVEL2",
                ListingGroupFilterProductTypeLevel::Level3 => "LEVEL3",
                ListingGroupFilterProductTypeLevel::Level4 => "LEVEL4",
                ListingGroupFilterProductTypeLevel::Level5 => "LEVEL5",
            }
        }
    }
}
/// Container for enum describing the type of the listing group filter node.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListingGroupFilterTypeEnum {}
/// Nested message and enum types in `ListingGroupFilterTypeEnum`.
pub mod listing_group_filter_type_enum {
    /// The type of the listing group filter.
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
    pub enum ListingGroupFilterType {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Subdivision of products along some listing dimensions.
        Subdivision = 2,
        /// An included listing group filter leaf node.
        UnitIncluded = 3,
        /// An excluded listing group filter leaf node.
        UnitExcluded = 4,
    }
    impl ListingGroupFilterType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ListingGroupFilterType::Unspecified => "UNSPECIFIED",
                ListingGroupFilterType::Unknown => "UNKNOWN",
                ListingGroupFilterType::Subdivision => "SUBDIVISION",
                ListingGroupFilterType::UnitIncluded => "UNIT_INCLUDED",
                ListingGroupFilterType::UnitExcluded => "UNIT_EXCLUDED",
            }
        }
    }
}
/// Container for enum describing the type of the vertical a listing group filter
/// tree represents.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListingGroupFilterVerticalEnum {}
/// Nested message and enum types in `ListingGroupFilterVerticalEnum`.
pub mod listing_group_filter_vertical_enum {
    /// The type of the listing group filter vertical.
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
    pub enum ListingGroupFilterVertical {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Represents the shopping vertical.
        Shopping = 2,
    }
    impl ListingGroupFilterVertical {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ListingGroupFilterVertical::Unspecified => "UNSPECIFIED",
                ListingGroupFilterVertical::Unknown => "UNKNOWN",
                ListingGroupFilterVertical::Shopping => "SHOPPING",
            }
        }
    }
}
/// Values for Local placeholder fields.
/// For more information about dynamic remarketing feeds, see
/// <https://support.google.com/google-ads/answer/6053288.>
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LocalPlaceholderFieldEnum {}
/// Nested message and enum types in `LocalPlaceholderFieldEnum`.
pub mod local_placeholder_field_enum {
    /// Possible values for Local placeholder fields.
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
        /// Upgraded URLs; the more specific the better (for example, the individual
        /// URL of a specific local deal and its location).
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
        ///    scheme.
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
    impl LocalPlaceholderField {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                LocalPlaceholderField::Unspecified => "UNSPECIFIED",
                LocalPlaceholderField::Unknown => "UNKNOWN",
                LocalPlaceholderField::DealId => "DEAL_ID",
                LocalPlaceholderField::DealName => "DEAL_NAME",
                LocalPlaceholderField::Subtitle => "SUBTITLE",
                LocalPlaceholderField::Description => "DESCRIPTION",
                LocalPlaceholderField::Price => "PRICE",
                LocalPlaceholderField::FormattedPrice => "FORMATTED_PRICE",
                LocalPlaceholderField::SalePrice => "SALE_PRICE",
                LocalPlaceholderField::FormattedSalePrice => "FORMATTED_SALE_PRICE",
                LocalPlaceholderField::ImageUrl => "IMAGE_URL",
                LocalPlaceholderField::Address => "ADDRESS",
                LocalPlaceholderField::Category => "CATEGORY",
                LocalPlaceholderField::ContextualKeywords => "CONTEXTUAL_KEYWORDS",
                LocalPlaceholderField::FinalUrls => "FINAL_URLS",
                LocalPlaceholderField::FinalMobileUrls => "FINAL_MOBILE_URLS",
                LocalPlaceholderField::TrackingUrl => "TRACKING_URL",
                LocalPlaceholderField::AndroidAppLink => "ANDROID_APP_LINK",
                LocalPlaceholderField::SimilarDealIds => "SIMILAR_DEAL_IDS",
                LocalPlaceholderField::IosAppLink => "IOS_APP_LINK",
                LocalPlaceholderField::IosAppStoreId => "IOS_APP_STORE_ID",
            }
        }
    }
}
/// Values for Location Extension Targeting criterion fields.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LocationExtensionTargetingCriterionFieldEnum {}
/// Nested message and enum types in `LocationExtensionTargetingCriterionFieldEnum`.
pub mod location_extension_targeting_criterion_field_enum {
    /// Possible values for Location Extension Targeting criterion fields.
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
    impl LocationExtensionTargetingCriterionField {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                LocationExtensionTargetingCriterionField::Unspecified => "UNSPECIFIED",
                LocationExtensionTargetingCriterionField::Unknown => "UNKNOWN",
                LocationExtensionTargetingCriterionField::AddressLine1 => {
                    "ADDRESS_LINE_1"
                }
                LocationExtensionTargetingCriterionField::AddressLine2 => {
                    "ADDRESS_LINE_2"
                }
                LocationExtensionTargetingCriterionField::City => "CITY",
                LocationExtensionTargetingCriterionField::Province => "PROVINCE",
                LocationExtensionTargetingCriterionField::PostalCode => "POSTAL_CODE",
                LocationExtensionTargetingCriterionField::CountryCode => "COUNTRY_CODE",
            }
        }
    }
}
/// Values for Location placeholder fields.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LocationPlaceholderFieldEnum {}
/// Nested message and enum types in `LocationPlaceholderFieldEnum`.
pub mod location_placeholder_field_enum {
    /// Possible values for Location placeholder fields.
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
    impl LocationPlaceholderField {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                LocationPlaceholderField::Unspecified => "UNSPECIFIED",
                LocationPlaceholderField::Unknown => "UNKNOWN",
                LocationPlaceholderField::BusinessName => "BUSINESS_NAME",
                LocationPlaceholderField::AddressLine1 => "ADDRESS_LINE_1",
                LocationPlaceholderField::AddressLine2 => "ADDRESS_LINE_2",
                LocationPlaceholderField::City => "CITY",
                LocationPlaceholderField::Province => "PROVINCE",
                LocationPlaceholderField::PostalCode => "POSTAL_CODE",
                LocationPlaceholderField::CountryCode => "COUNTRY_CODE",
                LocationPlaceholderField::PhoneNumber => "PHONE_NUMBER",
            }
        }
    }
}
/// Used to distinguish the location source type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LocationSourceTypeEnum {}
/// Nested message and enum types in `LocationSourceTypeEnum`.
pub mod location_source_type_enum {
    /// The possible types of a location source.
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
    pub enum LocationSourceType {
        /// No value has been specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Locations associated with the customer's linked Business Profile.
        GoogleMyBusiness = 2,
        /// Affiliate (chain) store locations. For example, Best Buy store locations.
        Affiliate = 3,
    }
    impl LocationSourceType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                LocationSourceType::Unspecified => "UNSPECIFIED",
                LocationSourceType::Unknown => "UNKNOWN",
                LocationSourceType::GoogleMyBusiness => "GOOGLE_MY_BUSINESS",
                LocationSourceType::Affiliate => "AFFILIATE",
            }
        }
    }
}
/// Container for enum describing possible status of a manager and client link.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ManagerLinkStatusEnum {}
/// Nested message and enum types in `ManagerLinkStatusEnum`.
pub mod manager_link_status_enum {
    /// Possible statuses of a link.
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
    impl ManagerLinkStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ManagerLinkStatus::Unspecified => "UNSPECIFIED",
                ManagerLinkStatus::Unknown => "UNKNOWN",
                ManagerLinkStatus::Active => "ACTIVE",
                ManagerLinkStatus::Inactive => "INACTIVE",
                ManagerLinkStatus::Pending => "PENDING",
                ManagerLinkStatus::Refused => "REFUSED",
                ManagerLinkStatus::Canceled => "CANCELED",
            }
        }
    }
}
/// Container for enum describing the types of media.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MediaTypeEnum {}
/// Nested message and enum types in `MediaTypeEnum`.
pub mod media_type_enum {
    /// The type of media.
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
    impl MediaType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                MediaType::Unspecified => "UNSPECIFIED",
                MediaType::Unknown => "UNKNOWN",
                MediaType::Image => "IMAGE",
                MediaType::Icon => "ICON",
                MediaType::MediaBundle => "MEDIA_BUNDLE",
                MediaType::Audio => "AUDIO",
                MediaType::Video => "VIDEO",
                MediaType::DynamicImage => "DYNAMIC_IMAGE",
            }
        }
    }
}
/// Container for enum describing possible statuses of a Google Merchant Center
/// link.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MerchantCenterLinkStatusEnum {}
/// Nested message and enum types in `MerchantCenterLinkStatusEnum`.
pub mod merchant_center_link_status_enum {
    /// Describes the possible statuses for a link between a Google Ads customer
    /// and a Google Merchant Center account.
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
    impl MerchantCenterLinkStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                MerchantCenterLinkStatus::Unspecified => "UNSPECIFIED",
                MerchantCenterLinkStatus::Unknown => "UNKNOWN",
                MerchantCenterLinkStatus::Enabled => "ENABLED",
                MerchantCenterLinkStatus::Pending => "PENDING",
            }
        }
    }
}
/// Values for Message placeholder fields.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MessagePlaceholderFieldEnum {}
/// Nested message and enum types in `MessagePlaceholderFieldEnum`.
pub mod message_placeholder_field_enum {
    /// Possible values for Message placeholder fields.
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
    impl MessagePlaceholderField {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                MessagePlaceholderField::Unspecified => "UNSPECIFIED",
                MessagePlaceholderField::Unknown => "UNKNOWN",
                MessagePlaceholderField::BusinessName => "BUSINESS_NAME",
                MessagePlaceholderField::CountryCode => "COUNTRY_CODE",
                MessagePlaceholderField::PhoneNumber => "PHONE_NUMBER",
                MessagePlaceholderField::MessageExtensionText => "MESSAGE_EXTENSION_TEXT",
                MessagePlaceholderField::MessageText => "MESSAGE_TEXT",
            }
        }
    }
}
/// Container for enum describing the types of mobile device.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MobileDeviceTypeEnum {}
/// Nested message and enum types in `MobileDeviceTypeEnum`.
pub mod mobile_device_type_enum {
    /// The type of mobile device.
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
    impl MobileDeviceType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                MobileDeviceType::Unspecified => "UNSPECIFIED",
                MobileDeviceType::Unknown => "UNKNOWN",
                MobileDeviceType::Mobile => "MOBILE",
                MobileDeviceType::Tablet => "TABLET",
            }
        }
    }
}
/// Container for enum describing possible negative geo target types.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NegativeGeoTargetTypeEnum {}
/// Nested message and enum types in `NegativeGeoTargetTypeEnum`.
pub mod negative_geo_target_type_enum {
    /// The possible negative geo target types.
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
    impl NegativeGeoTargetType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                NegativeGeoTargetType::Unspecified => "UNSPECIFIED",
                NegativeGeoTargetType::Unknown => "UNKNOWN",
                NegativeGeoTargetType::PresenceOrInterest => "PRESENCE_OR_INTEREST",
                NegativeGeoTargetType::Presence => "PRESENCE",
            }
        }
    }
}
/// Container for enum describing reasons why an offline user data job
/// failed to be processed.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OfflineUserDataJobFailureReasonEnum {}
/// Nested message and enum types in `OfflineUserDataJobFailureReasonEnum`.
pub mod offline_user_data_job_failure_reason_enum {
    /// The failure reason of an offline user data job.
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
    impl OfflineUserDataJobFailureReason {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                OfflineUserDataJobFailureReason::Unspecified => "UNSPECIFIED",
                OfflineUserDataJobFailureReason::Unknown => "UNKNOWN",
                OfflineUserDataJobFailureReason::InsufficientMatchedTransactions => {
                    "INSUFFICIENT_MATCHED_TRANSACTIONS"
                }
                OfflineUserDataJobFailureReason::InsufficientTransactions => {
                    "INSUFFICIENT_TRANSACTIONS"
                }
            }
        }
    }
}
/// Container for enum describing reasons match rate ranges for a customer match
/// list upload.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OfflineUserDataJobMatchRateRangeEnum {}
/// Nested message and enum types in `OfflineUserDataJobMatchRateRangeEnum`.
pub mod offline_user_data_job_match_rate_range_enum {
    /// The match rate range of an offline user data job.
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
    pub enum OfflineUserDataJobMatchRateRange {
        /// Not specified.
        Unspecified = 0,
        /// Default value for match rate range.
        Unknown = 1,
        /// Match rate range for offline data upload entity is between 0% and 19%.
        MatchRangeLessThan20 = 2,
        /// Match rate range for offline data upload entity is between 20% and 30%.
        MatchRange20To30 = 3,
        /// Match rate range for offline data upload entity is between 31% and 40%.
        MatchRange31To40 = 4,
        /// Match rate range for offline data upload entity is between 41% and 50%.
        MatchRange41To50 = 5,
        /// Match rate range for offline data upload entity is between 51% and 60%.
        MatchRange51To60 = 6,
        /// Match rate range for offline data upload entity is between 61% and 70%.
        MatchRange61To70 = 7,
        /// Match rate range for offline data upload entity is between 71% and 80%.
        MatchRange71To80 = 8,
        /// Match rate range for offline data upload entity is between 81% and 90%.
        MatchRange81To90 = 9,
        /// Match rate range for offline data upload entity more than or equal to
        /// 91%.
        MatchRange91To100 = 10,
    }
    impl OfflineUserDataJobMatchRateRange {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                OfflineUserDataJobMatchRateRange::Unspecified => "UNSPECIFIED",
                OfflineUserDataJobMatchRateRange::Unknown => "UNKNOWN",
                OfflineUserDataJobMatchRateRange::MatchRangeLessThan20 => {
                    "MATCH_RANGE_LESS_THAN_20"
                }
                OfflineUserDataJobMatchRateRange::MatchRange20To30 => {
                    "MATCH_RANGE_20_TO_30"
                }
                OfflineUserDataJobMatchRateRange::MatchRange31To40 => {
                    "MATCH_RANGE_31_TO_40"
                }
                OfflineUserDataJobMatchRateRange::MatchRange41To50 => {
                    "MATCH_RANGE_41_TO_50"
                }
                OfflineUserDataJobMatchRateRange::MatchRange51To60 => {
                    "MATCH_RANGE_51_TO_60"
                }
                OfflineUserDataJobMatchRateRange::MatchRange61To70 => {
                    "MATCH_RANGE_61_TO_70"
                }
                OfflineUserDataJobMatchRateRange::MatchRange71To80 => {
                    "MATCH_RANGE_71_TO_80"
                }
                OfflineUserDataJobMatchRateRange::MatchRange81To90 => {
                    "MATCH_RANGE_81_TO_90"
                }
                OfflineUserDataJobMatchRateRange::MatchRange91To100 => {
                    "MATCH_RANGE_91_TO_100"
                }
            }
        }
    }
}
/// Container for enum describing status of an offline user data job.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OfflineUserDataJobStatusEnum {}
/// Nested message and enum types in `OfflineUserDataJobStatusEnum`.
pub mod offline_user_data_job_status_enum {
    /// The status of an offline user data job.
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
    impl OfflineUserDataJobStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                OfflineUserDataJobStatus::Unspecified => "UNSPECIFIED",
                OfflineUserDataJobStatus::Unknown => "UNKNOWN",
                OfflineUserDataJobStatus::Pending => "PENDING",
                OfflineUserDataJobStatus::Running => "RUNNING",
                OfflineUserDataJobStatus::Success => "SUCCESS",
                OfflineUserDataJobStatus::Failed => "FAILED",
            }
        }
    }
}
/// Container for enum describing types of an offline user data job.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OfflineUserDataJobTypeEnum {}
/// Nested message and enum types in `OfflineUserDataJobTypeEnum`.
pub mod offline_user_data_job_type_enum {
    /// The type of an offline user data job.
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
    impl OfflineUserDataJobType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                OfflineUserDataJobType::Unspecified => "UNSPECIFIED",
                OfflineUserDataJobType::Unknown => "UNKNOWN",
                OfflineUserDataJobType::StoreSalesUploadFirstParty => {
                    "STORE_SALES_UPLOAD_FIRST_PARTY"
                }
                OfflineUserDataJobType::StoreSalesUploadThirdParty => {
                    "STORE_SALES_UPLOAD_THIRD_PARTY"
                }
                OfflineUserDataJobType::CustomerMatchUserList => {
                    "CUSTOMER_MATCH_USER_LIST"
                }
                OfflineUserDataJobType::CustomerMatchWithAttributes => {
                    "CUSTOMER_MATCH_WITH_ATTRIBUTES"
                }
            }
        }
    }
}
/// Container for enum describing the type of OS operators.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperatingSystemVersionOperatorTypeEnum {}
/// Nested message and enum types in `OperatingSystemVersionOperatorTypeEnum`.
pub mod operating_system_version_operator_type_enum {
    /// The type of operating system version.
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
    impl OperatingSystemVersionOperatorType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                OperatingSystemVersionOperatorType::Unspecified => "UNSPECIFIED",
                OperatingSystemVersionOperatorType::Unknown => "UNKNOWN",
                OperatingSystemVersionOperatorType::EqualsTo => "EQUALS_TO",
                OperatingSystemVersionOperatorType::GreaterThanEqualsTo => {
                    "GREATER_THAN_EQUALS_TO"
                }
            }
        }
    }
}
/// Container for enum describing the type of optimization goal.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OptimizationGoalTypeEnum {}
/// Nested message and enum types in `OptimizationGoalTypeEnum`.
pub mod optimization_goal_type_enum {
    /// The type of optimization goal
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
        /// Optimize for pre-registration. Pre-registration conversions are the
        /// number of pre-registration signups to receive a notification when the app
        /// is released.
        AppPreRegistration = 4,
    }
    impl OptimizationGoalType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                OptimizationGoalType::Unspecified => "UNSPECIFIED",
                OptimizationGoalType::Unknown => "UNKNOWN",
                OptimizationGoalType::CallClicks => "CALL_CLICKS",
                OptimizationGoalType::DrivingDirections => "DRIVING_DIRECTIONS",
                OptimizationGoalType::AppPreRegistration => "APP_PRE_REGISTRATION",
            }
        }
    }
}
/// Container for enum describing possible payment modes.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PaymentModeEnum {}
/// Nested message and enum types in `PaymentModeEnum`.
pub mod payment_mode_enum {
    /// Enum describing possible payment modes.
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
    pub enum PaymentMode {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Pay per click.
        Clicks = 4,
        /// Pay per conversion value. This mode is only supported by campaigns with
        /// AdvertisingChannelType.HOTEL, BiddingStrategyType.COMMISSION, and
        /// BudgetType.STANDARD.
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
    impl PaymentMode {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                PaymentMode::Unspecified => "UNSPECIFIED",
                PaymentMode::Unknown => "UNKNOWN",
                PaymentMode::Clicks => "CLICKS",
                PaymentMode::ConversionValue => "CONVERSION_VALUE",
                PaymentMode::Conversions => "CONVERSIONS",
                PaymentMode::GuestStay => "GUEST_STAY",
            }
        }
    }
}
/// Performance Max Upgrade status for campaign.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PerformanceMaxUpgradeStatusEnum {}
/// Nested message and enum types in `PerformanceMaxUpgradeStatusEnum`.
pub mod performance_max_upgrade_status_enum {
    /// Performance Max Upgrade status enum for campaign.
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
    pub enum PerformanceMaxUpgradeStatus {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// The campaign is eligible for upgrade to a Performance Max campaign.
        UpgradeElibigle = 2,
        /// The upgrade to a Performance Max campaign is in progress.
        UpgradeInProgress = 3,
        /// The upgrade to a Performance Max campaign is complete.
        UpgradeComplete = 4,
        /// The upgrade to a Performance Max campaign failed.
        /// The campaign will still serve as it was before upgrade was attempted.
        UpgradeFailed = 5,
    }
    impl PerformanceMaxUpgradeStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                PerformanceMaxUpgradeStatus::Unspecified => "UNSPECIFIED",
                PerformanceMaxUpgradeStatus::Unknown => "UNKNOWN",
                PerformanceMaxUpgradeStatus::UpgradeElibigle => "UPGRADE_ELIBIGLE",
                PerformanceMaxUpgradeStatus::UpgradeInProgress => "UPGRADE_IN_PROGRESS",
                PerformanceMaxUpgradeStatus::UpgradeComplete => "UPGRADE_COMPLETE",
                PerformanceMaxUpgradeStatus::UpgradeFailed => "UPGRADE_FAILED",
            }
        }
    }
}
/// Container for enum describing possible placement types.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlacementTypeEnum {}
/// Nested message and enum types in `PlacementTypeEnum`.
pub mod placement_type_enum {
    /// Possible placement types for a feed mapping.
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
    pub enum PlacementType {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Websites(for example, 'www.flowers4sale.com').
        Website = 2,
        /// Mobile application categories(for example, 'Games').
        MobileAppCategory = 3,
        /// mobile applications(for example, 'mobileapp::2-com.whatsthewordanswers').
        MobileApplication = 4,
        /// YouTube videos(for example, 'youtube.com/video/wtLJPvx7-ys').
        YoutubeVideo = 5,
        /// YouTube channels(for example, 'youtube.com::L8ZULXASCc1I_oaOT0NaOQ').
        YoutubeChannel = 6,
    }
    impl PlacementType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                PlacementType::Unspecified => "UNSPECIFIED",
                PlacementType::Unknown => "UNKNOWN",
                PlacementType::Website => "WEBSITE",
                PlacementType::MobileAppCategory => "MOBILE_APP_CATEGORY",
                PlacementType::MobileApplication => "MOBILE_APPLICATION",
                PlacementType::YoutubeVideo => "YOUTUBE_VIDEO",
                PlacementType::YoutubeChannel => "YOUTUBE_CHANNEL",
            }
        }
    }
}
/// Container for enum describing possible positive geo target types.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PositiveGeoTargetTypeEnum {}
/// Nested message and enum types in `PositiveGeoTargetTypeEnum`.
pub mod positive_geo_target_type_enum {
    /// The possible positive geo target types.
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
    impl PositiveGeoTargetType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                PositiveGeoTargetType::Unspecified => "UNSPECIFIED",
                PositiveGeoTargetType::Unknown => "UNKNOWN",
                PositiveGeoTargetType::PresenceOrInterest => "PRESENCE_OR_INTEREST",
                PositiveGeoTargetType::SearchInterest => "SEARCH_INTEREST",
                PositiveGeoTargetType::Presence => "PRESENCE",
            }
        }
    }
}
/// Values for Price placeholder fields.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PricePlaceholderFieldEnum {}
/// Nested message and enum types in `PricePlaceholderFieldEnum`.
pub mod price_placeholder_field_enum {
    /// Possible values for Price placeholder fields.
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
        /// for example, 30 USD. The currency must match one of the available
        /// currencies.
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
        /// for example, 30 USD. The currency must match one of the available
        /// currencies.
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
        /// for example, 30 USD. The currency must match one of the available
        /// currencies.
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
        /// for example, 30 USD. The currency must match one of the available
        /// currencies.
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
        /// for example, 30 USD. The currency must match one of the available
        /// currencies.
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
        /// for example, 30 USD. The currency must match one of the available
        /// currencies.
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
        /// for example, 30 USD. The currency must match one of the available
        /// currencies.
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
        /// for example, 30 USD. The currency must match one of the available
        /// currencies.
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
    impl PricePlaceholderField {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                PricePlaceholderField::Unspecified => "UNSPECIFIED",
                PricePlaceholderField::Unknown => "UNKNOWN",
                PricePlaceholderField::Type => "TYPE",
                PricePlaceholderField::PriceQualifier => "PRICE_QUALIFIER",
                PricePlaceholderField::TrackingTemplate => "TRACKING_TEMPLATE",
                PricePlaceholderField::Language => "LANGUAGE",
                PricePlaceholderField::FinalUrlSuffix => "FINAL_URL_SUFFIX",
                PricePlaceholderField::Item1Header => "ITEM_1_HEADER",
                PricePlaceholderField::Item1Description => "ITEM_1_DESCRIPTION",
                PricePlaceholderField::Item1Price => "ITEM_1_PRICE",
                PricePlaceholderField::Item1Unit => "ITEM_1_UNIT",
                PricePlaceholderField::Item1FinalUrls => "ITEM_1_FINAL_URLS",
                PricePlaceholderField::Item1FinalMobileUrls => "ITEM_1_FINAL_MOBILE_URLS",
                PricePlaceholderField::Item2Header => "ITEM_2_HEADER",
                PricePlaceholderField::Item2Description => "ITEM_2_DESCRIPTION",
                PricePlaceholderField::Item2Price => "ITEM_2_PRICE",
                PricePlaceholderField::Item2Unit => "ITEM_2_UNIT",
                PricePlaceholderField::Item2FinalUrls => "ITEM_2_FINAL_URLS",
                PricePlaceholderField::Item2FinalMobileUrls => "ITEM_2_FINAL_MOBILE_URLS",
                PricePlaceholderField::Item3Header => "ITEM_3_HEADER",
                PricePlaceholderField::Item3Description => "ITEM_3_DESCRIPTION",
                PricePlaceholderField::Item3Price => "ITEM_3_PRICE",
                PricePlaceholderField::Item3Unit => "ITEM_3_UNIT",
                PricePlaceholderField::Item3FinalUrls => "ITEM_3_FINAL_URLS",
                PricePlaceholderField::Item3FinalMobileUrls => "ITEM_3_FINAL_MOBILE_URLS",
                PricePlaceholderField::Item4Header => "ITEM_4_HEADER",
                PricePlaceholderField::Item4Description => "ITEM_4_DESCRIPTION",
                PricePlaceholderField::Item4Price => "ITEM_4_PRICE",
                PricePlaceholderField::Item4Unit => "ITEM_4_UNIT",
                PricePlaceholderField::Item4FinalUrls => "ITEM_4_FINAL_URLS",
                PricePlaceholderField::Item4FinalMobileUrls => "ITEM_4_FINAL_MOBILE_URLS",
                PricePlaceholderField::Item5Header => "ITEM_5_HEADER",
                PricePlaceholderField::Item5Description => "ITEM_5_DESCRIPTION",
                PricePlaceholderField::Item5Price => "ITEM_5_PRICE",
                PricePlaceholderField::Item5Unit => "ITEM_5_UNIT",
                PricePlaceholderField::Item5FinalUrls => "ITEM_5_FINAL_URLS",
                PricePlaceholderField::Item5FinalMobileUrls => "ITEM_5_FINAL_MOBILE_URLS",
                PricePlaceholderField::Item6Header => "ITEM_6_HEADER",
                PricePlaceholderField::Item6Description => "ITEM_6_DESCRIPTION",
                PricePlaceholderField::Item6Price => "ITEM_6_PRICE",
                PricePlaceholderField::Item6Unit => "ITEM_6_UNIT",
                PricePlaceholderField::Item6FinalUrls => "ITEM_6_FINAL_URLS",
                PricePlaceholderField::Item6FinalMobileUrls => "ITEM_6_FINAL_MOBILE_URLS",
                PricePlaceholderField::Item7Header => "ITEM_7_HEADER",
                PricePlaceholderField::Item7Description => "ITEM_7_DESCRIPTION",
                PricePlaceholderField::Item7Price => "ITEM_7_PRICE",
                PricePlaceholderField::Item7Unit => "ITEM_7_UNIT",
                PricePlaceholderField::Item7FinalUrls => "ITEM_7_FINAL_URLS",
                PricePlaceholderField::Item7FinalMobileUrls => "ITEM_7_FINAL_MOBILE_URLS",
                PricePlaceholderField::Item8Header => "ITEM_8_HEADER",
                PricePlaceholderField::Item8Description => "ITEM_8_DESCRIPTION",
                PricePlaceholderField::Item8Price => "ITEM_8_PRICE",
                PricePlaceholderField::Item8Unit => "ITEM_8_UNIT",
                PricePlaceholderField::Item8FinalUrls => "ITEM_8_FINAL_URLS",
                PricePlaceholderField::Item8FinalMobileUrls => "ITEM_8_FINAL_MOBILE_URLS",
            }
        }
    }
}
/// Status of the product bidding category.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProductBiddingCategoryStatusEnum {}
/// Nested message and enum types in `ProductBiddingCategoryStatusEnum`.
pub mod product_bidding_category_status_enum {
    /// Enum describing the status of the product bidding category.
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
    impl ProductBiddingCategoryStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ProductBiddingCategoryStatus::Unspecified => "UNSPECIFIED",
                ProductBiddingCategoryStatus::Unknown => "UNKNOWN",
                ProductBiddingCategoryStatus::Active => "ACTIVE",
                ProductBiddingCategoryStatus::Obsolete => "OBSOLETE",
            }
        }
    }
}
/// Values for Promotion placeholder fields.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PromotionPlaceholderFieldEnum {}
/// Nested message and enum types in `PromotionPlaceholderFieldEnum`.
pub mod promotion_placeholder_field_enum {
    /// Possible values for Promotion placeholder fields.
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
    pub enum PromotionPlaceholderField {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Data Type: STRING. The text that appears on the ad when the extension is
        /// shown.
        PromotionTarget = 2,
        /// Data Type: STRING. Lets you add "up to" phrase to the promotion,
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
    impl PromotionPlaceholderField {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                PromotionPlaceholderField::Unspecified => "UNSPECIFIED",
                PromotionPlaceholderField::Unknown => "UNKNOWN",
                PromotionPlaceholderField::PromotionTarget => "PROMOTION_TARGET",
                PromotionPlaceholderField::DiscountModifier => "DISCOUNT_MODIFIER",
                PromotionPlaceholderField::PercentOff => "PERCENT_OFF",
                PromotionPlaceholderField::MoneyAmountOff => "MONEY_AMOUNT_OFF",
                PromotionPlaceholderField::PromotionCode => "PROMOTION_CODE",
                PromotionPlaceholderField::OrdersOverAmount => "ORDERS_OVER_AMOUNT",
                PromotionPlaceholderField::PromotionStart => "PROMOTION_START",
                PromotionPlaceholderField::PromotionEnd => "PROMOTION_END",
                PromotionPlaceholderField::Occasion => "OCCASION",
                PromotionPlaceholderField::FinalUrls => "FINAL_URLS",
                PromotionPlaceholderField::FinalMobileUrls => "FINAL_MOBILE_URLS",
                PromotionPlaceholderField::TrackingUrl => "TRACKING_URL",
                PromotionPlaceholderField::Language => "LANGUAGE",
                PromotionPlaceholderField::FinalUrlSuffix => "FINAL_URL_SUFFIX",
            }
        }
    }
}
/// Message describing length of a plannable video ad.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReachPlanAdLengthEnum {}
/// Nested message and enum types in `ReachPlanAdLengthEnum`.
pub mod reach_plan_ad_length_enum {
    /// Possible ad length values.
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
    impl ReachPlanAdLength {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ReachPlanAdLength::Unspecified => "UNSPECIFIED",
                ReachPlanAdLength::Unknown => "UNKNOWN",
                ReachPlanAdLength::SixSeconds => "SIX_SECONDS",
                ReachPlanAdLength::FifteenOrTwentySeconds => "FIFTEEN_OR_TWENTY_SECONDS",
                ReachPlanAdLength::TwentySecondsOrMore => "TWENTY_SECONDS_OR_MORE",
            }
        }
    }
}
/// Message describing plannable age ranges.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReachPlanAgeRangeEnum {}
/// Nested message and enum types in `ReachPlanAgeRangeEnum`.
pub mod reach_plan_age_range_enum {
    /// Possible plannable age range values.
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
    impl ReachPlanAgeRange {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ReachPlanAgeRange::Unspecified => "UNSPECIFIED",
                ReachPlanAgeRange::Unknown => "UNKNOWN",
                ReachPlanAgeRange::AgeRange1824 => "AGE_RANGE_18_24",
                ReachPlanAgeRange::AgeRange1834 => "AGE_RANGE_18_34",
                ReachPlanAgeRange::AgeRange1844 => "AGE_RANGE_18_44",
                ReachPlanAgeRange::AgeRange1849 => "AGE_RANGE_18_49",
                ReachPlanAgeRange::AgeRange1854 => "AGE_RANGE_18_54",
                ReachPlanAgeRange::AgeRange1864 => "AGE_RANGE_18_64",
                ReachPlanAgeRange::AgeRange1865Up => "AGE_RANGE_18_65_UP",
                ReachPlanAgeRange::AgeRange2134 => "AGE_RANGE_21_34",
                ReachPlanAgeRange::AgeRange2534 => "AGE_RANGE_25_34",
                ReachPlanAgeRange::AgeRange2544 => "AGE_RANGE_25_44",
                ReachPlanAgeRange::AgeRange2549 => "AGE_RANGE_25_49",
                ReachPlanAgeRange::AgeRange2554 => "AGE_RANGE_25_54",
                ReachPlanAgeRange::AgeRange2564 => "AGE_RANGE_25_64",
                ReachPlanAgeRange::AgeRange2565Up => "AGE_RANGE_25_65_UP",
                ReachPlanAgeRange::AgeRange3544 => "AGE_RANGE_35_44",
                ReachPlanAgeRange::AgeRange3549 => "AGE_RANGE_35_49",
                ReachPlanAgeRange::AgeRange3554 => "AGE_RANGE_35_54",
                ReachPlanAgeRange::AgeRange3564 => "AGE_RANGE_35_64",
                ReachPlanAgeRange::AgeRange3565Up => "AGE_RANGE_35_65_UP",
                ReachPlanAgeRange::AgeRange4554 => "AGE_RANGE_45_54",
                ReachPlanAgeRange::AgeRange4564 => "AGE_RANGE_45_64",
                ReachPlanAgeRange::AgeRange4565Up => "AGE_RANGE_45_65_UP",
                ReachPlanAgeRange::AgeRange5065Up => "AGE_RANGE_50_65_UP",
                ReachPlanAgeRange::AgeRange5564 => "AGE_RANGE_55_64",
                ReachPlanAgeRange::AgeRange5565Up => "AGE_RANGE_55_65_UP",
                ReachPlanAgeRange::AgeRange65Up => "AGE_RANGE_65_UP",
            }
        }
    }
}
/// Container for enum describing plannable networks.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReachPlanNetworkEnum {}
/// Nested message and enum types in `ReachPlanNetworkEnum`.
pub mod reach_plan_network_enum {
    /// Possible plannable network values.
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
    impl ReachPlanNetwork {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ReachPlanNetwork::Unspecified => "UNSPECIFIED",
                ReachPlanNetwork::Unknown => "UNKNOWN",
                ReachPlanNetwork::Youtube => "YOUTUBE",
                ReachPlanNetwork::GoogleVideoPartners => "GOOGLE_VIDEO_PARTNERS",
                ReachPlanNetwork::YoutubeAndGoogleVideoPartners => {
                    "YOUTUBE_AND_GOOGLE_VIDEO_PARTNERS"
                }
            }
        }
    }
}
/// Values for Real Estate placeholder fields.
/// For more information about dynamic remarketing feeds, see
/// <https://support.google.com/google-ads/answer/6053288.>
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RealEstatePlaceholderFieldEnum {}
/// Nested message and enum types in `RealEstatePlaceholderFieldEnum`.
pub mod real_estate_placeholder_field_enum {
    /// Possible values for Real Estate placeholder fields.
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
        /// URLs; the more specific the better (for example, the individual URL of a
        /// specific listing and its location).
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
        ///    scheme.
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
    impl RealEstatePlaceholderField {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                RealEstatePlaceholderField::Unspecified => "UNSPECIFIED",
                RealEstatePlaceholderField::Unknown => "UNKNOWN",
                RealEstatePlaceholderField::ListingId => "LISTING_ID",
                RealEstatePlaceholderField::ListingName => "LISTING_NAME",
                RealEstatePlaceholderField::CityName => "CITY_NAME",
                RealEstatePlaceholderField::Description => "DESCRIPTION",
                RealEstatePlaceholderField::Address => "ADDRESS",
                RealEstatePlaceholderField::Price => "PRICE",
                RealEstatePlaceholderField::FormattedPrice => "FORMATTED_PRICE",
                RealEstatePlaceholderField::ImageUrl => "IMAGE_URL",
                RealEstatePlaceholderField::PropertyType => "PROPERTY_TYPE",
                RealEstatePlaceholderField::ListingType => "LISTING_TYPE",
                RealEstatePlaceholderField::ContextualKeywords => "CONTEXTUAL_KEYWORDS",
                RealEstatePlaceholderField::FinalUrls => "FINAL_URLS",
                RealEstatePlaceholderField::FinalMobileUrls => "FINAL_MOBILE_URLS",
                RealEstatePlaceholderField::TrackingUrl => "TRACKING_URL",
                RealEstatePlaceholderField::AndroidAppLink => "ANDROID_APP_LINK",
                RealEstatePlaceholderField::SimilarListingIds => "SIMILAR_LISTING_IDS",
                RealEstatePlaceholderField::IosAppLink => "IOS_APP_LINK",
                RealEstatePlaceholderField::IosAppStoreId => "IOS_APP_STORE_ID",
            }
        }
    }
}
/// Container for enum describing resource change operations
/// in the ChangeEvent resource.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResourceChangeOperationEnum {}
/// Nested message and enum types in `ResourceChangeOperationEnum`.
pub mod resource_change_operation_enum {
    /// The operation on the changed resource in change_event resource.
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
    impl ResourceChangeOperation {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ResourceChangeOperation::Unspecified => "UNSPECIFIED",
                ResourceChangeOperation::Unknown => "UNKNOWN",
                ResourceChangeOperation::Create => "CREATE",
                ResourceChangeOperation::Update => "UPDATE",
                ResourceChangeOperation::Remove => "REMOVE",
            }
        }
    }
}
/// Container for enum describing possible resource limit types.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResourceLimitTypeEnum {}
/// Nested message and enum types in `ResourceLimitTypeEnum`.
pub mod resource_limit_type_enum {
    /// Resource limit type.
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
        /// Number of excluded campaign criteria in placement dimension, for example,
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
        /// Number of open rule based user lists per customer.
        RuleBasedUserListsPerCustomer = 153,
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
        /// Number of ENABLED lead form CampaignAssets per campaign.
        LeadFormCampaignAssetsPerCampaign = 143,
        /// Number of ENABLED promotion CustomerAssets per customer.
        PromotionCustomerAssetsPerCustomer = 79,
        /// Number of ENABLED promotion CampaignAssets per campaign.
        PromotionCampaignAssetsPerCampaign = 80,
        /// Number of ENABLED promotion AdGroupAssets per ad group.
        PromotionAdGroupAssetsPerAdGroup = 81,
        /// Number of ENABLED callout CustomerAssets per customer.
        CalloutCustomerAssetsPerCustomer = 134,
        /// Number of ENABLED callout CampaignAssets per campaign.
        CalloutCampaignAssetsPerCampaign = 135,
        /// Number of ENABLED callout AdGroupAssets per ad group.
        CalloutAdGroupAssetsPerAdGroup = 136,
        /// Number of ENABLED sitelink CustomerAssets per customer.
        SitelinkCustomerAssetsPerCustomer = 137,
        /// Number of ENABLED sitelink CampaignAssets per campaign.
        SitelinkCampaignAssetsPerCampaign = 138,
        /// Number of ENABLED sitelink AdGroupAssets per ad group.
        SitelinkAdGroupAssetsPerAdGroup = 139,
        /// Number of ENABLED structured snippet CustomerAssets per customer.
        StructuredSnippetCustomerAssetsPerCustomer = 140,
        /// Number of ENABLED structured snippet CampaignAssets per campaign.
        StructuredSnippetCampaignAssetsPerCampaign = 141,
        /// Number of ENABLED structured snippet AdGroupAssets per ad group.
        StructuredSnippetAdGroupAssetsPerAdGroup = 142,
        /// Number of ENABLED mobile app CustomerAssets per customer.
        MobileAppCustomerAssetsPerCustomer = 144,
        /// Number of ENABLED mobile app CampaignAssets per campaign.
        MobileAppCampaignAssetsPerCampaign = 145,
        /// Number of ENABLED mobile app AdGroupAssets per ad group.
        MobileAppAdGroupAssetsPerAdGroup = 146,
        /// Number of ENABLED hotel callout CustomerAssets per customer.
        HotelCalloutCustomerAssetsPerCustomer = 147,
        /// Number of ENABLED hotel callout CampaignAssets per campaign.
        HotelCalloutCampaignAssetsPerCampaign = 148,
        /// Number of ENABLED hotel callout AdGroupAssets per ad group.
        HotelCalloutAdGroupAssetsPerAdGroup = 149,
        /// Number of ENABLED call CustomerAssets per customer.
        CallCustomerAssetsPerCustomer = 150,
        /// Number of ENABLED call CampaignAssets per campaign.
        CallCampaignAssetsPerCampaign = 151,
        /// Number of ENABLED call AdGroupAssets per ad group.
        CallAdGroupAssetsPerAdGroup = 152,
        /// Number of ENABLED price CustomerAssets per customer.
        PriceCustomerAssetsPerCustomer = 154,
        /// Number of ENABLED price CampaignAssets per campaign.
        PriceCampaignAssetsPerCampaign = 155,
        /// Number of ENABLED price AdGroupAssets per ad group.
        PriceAdGroupAssetsPerAdGroup = 156,
        /// Number of ENABLED page feed asset sets per customer.
        PageFeedAssetSetsPerCustomer = 157,
        /// Number of ENABLED dynamic education feed asset sets per customer.
        DynamicEducationFeedAssetSetsPerCustomer = 158,
        /// Number of ENABLED assets per page feed asset set.
        AssetsPerPageFeedAssetSet = 159,
        /// Number of ENABLED assets per dynamic education asset set.
        AssetsPerDynamicEducationFeedAssetSet = 160,
        /// Number of ENABLED dynamic real estate asset sets per customer.
        DynamicRealEstateAssetSetsPerCustomer = 161,
        /// Number of ENABLED assets per dynamic real estate asset set.
        AssetsPerDynamicRealEstateAssetSet = 162,
        /// Number of ENABLED dynamic custom asset sets per customer.
        DynamicCustomAssetSetsPerCustomer = 163,
        /// Number of ENABLED assets per dynamic custom asset set.
        AssetsPerDynamicCustomAssetSet = 164,
        /// Number of ENABLED dynamic hotels and rentals asset sets per
        /// customer.
        DynamicHotelsAndRentalsAssetSetsPerCustomer = 165,
        /// Number of ENABLED assets per dynamic hotels and rentals asset set.
        AssetsPerDynamicHotelsAndRentalsAssetSet = 166,
        /// Number of ENABLED dynamic local asset sets per customer.
        DynamicLocalAssetSetsPerCustomer = 167,
        /// Number of ENABLED assets per dynamic local asset set.
        AssetsPerDynamicLocalAssetSet = 168,
        /// Number of ENABLED dynamic flights asset sets per customer.
        DynamicFlightsAssetSetsPerCustomer = 169,
        /// Number of ENABLED assets per dynamic flights asset set.
        AssetsPerDynamicFlightsAssetSet = 170,
        /// Number of ENABLED dynamic travel asset sets per customer.
        DynamicTravelAssetSetsPerCustomer = 171,
        /// Number of ENABLED assets per dynamic travel asset set.
        AssetsPerDynamicTravelAssetSet = 172,
        /// Number of ENABLED dynamic jobs asset sets per customer.
        DynamicJobsAssetSetsPerCustomer = 173,
        /// Number of ENABLED assets per dynamic jobs asset set.
        AssetsPerDynamicJobsAssetSet = 174,
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
        /// is private to a user of a customer. Each user of a customer has their own
        /// independent limit.
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
    impl ResourceLimitType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ResourceLimitType::Unspecified => "UNSPECIFIED",
                ResourceLimitType::Unknown => "UNKNOWN",
                ResourceLimitType::CampaignsPerCustomer => "CAMPAIGNS_PER_CUSTOMER",
                ResourceLimitType::BaseCampaignsPerCustomer => {
                    "BASE_CAMPAIGNS_PER_CUSTOMER"
                }
                ResourceLimitType::ExperimentCampaignsPerCustomer => {
                    "EXPERIMENT_CAMPAIGNS_PER_CUSTOMER"
                }
                ResourceLimitType::HotelCampaignsPerCustomer => {
                    "HOTEL_CAMPAIGNS_PER_CUSTOMER"
                }
                ResourceLimitType::SmartShoppingCampaignsPerCustomer => {
                    "SMART_SHOPPING_CAMPAIGNS_PER_CUSTOMER"
                }
                ResourceLimitType::AdGroupsPerCampaign => "AD_GROUPS_PER_CAMPAIGN",
                ResourceLimitType::AdGroupsPerShoppingCampaign => {
                    "AD_GROUPS_PER_SHOPPING_CAMPAIGN"
                }
                ResourceLimitType::AdGroupsPerHotelCampaign => {
                    "AD_GROUPS_PER_HOTEL_CAMPAIGN"
                }
                ResourceLimitType::ReportingAdGroupsPerLocalCampaign => {
                    "REPORTING_AD_GROUPS_PER_LOCAL_CAMPAIGN"
                }
                ResourceLimitType::ReportingAdGroupsPerAppCampaign => {
                    "REPORTING_AD_GROUPS_PER_APP_CAMPAIGN"
                }
                ResourceLimitType::ManagedAdGroupsPerSmartCampaign => {
                    "MANAGED_AD_GROUPS_PER_SMART_CAMPAIGN"
                }
                ResourceLimitType::AdGroupCriteriaPerCustomer => {
                    "AD_GROUP_CRITERIA_PER_CUSTOMER"
                }
                ResourceLimitType::BaseAdGroupCriteriaPerCustomer => {
                    "BASE_AD_GROUP_CRITERIA_PER_CUSTOMER"
                }
                ResourceLimitType::ExperimentAdGroupCriteriaPerCustomer => {
                    "EXPERIMENT_AD_GROUP_CRITERIA_PER_CUSTOMER"
                }
                ResourceLimitType::AdGroupCriteriaPerCampaign => {
                    "AD_GROUP_CRITERIA_PER_CAMPAIGN"
                }
                ResourceLimitType::CampaignCriteriaPerCustomer => {
                    "CAMPAIGN_CRITERIA_PER_CUSTOMER"
                }
                ResourceLimitType::BaseCampaignCriteriaPerCustomer => {
                    "BASE_CAMPAIGN_CRITERIA_PER_CUSTOMER"
                }
                ResourceLimitType::ExperimentCampaignCriteriaPerCustomer => {
                    "EXPERIMENT_CAMPAIGN_CRITERIA_PER_CUSTOMER"
                }
                ResourceLimitType::WebpageCriteriaPerCustomer => {
                    "WEBPAGE_CRITERIA_PER_CUSTOMER"
                }
                ResourceLimitType::BaseWebpageCriteriaPerCustomer => {
                    "BASE_WEBPAGE_CRITERIA_PER_CUSTOMER"
                }
                ResourceLimitType::ExperimentWebpageCriteriaPerCustomer => {
                    "EXPERIMENT_WEBPAGE_CRITERIA_PER_CUSTOMER"
                }
                ResourceLimitType::CombinedAudienceCriteriaPerAdGroup => {
                    "COMBINED_AUDIENCE_CRITERIA_PER_AD_GROUP"
                }
                ResourceLimitType::CustomerNegativePlacementCriteriaPerCustomer => {
                    "CUSTOMER_NEGATIVE_PLACEMENT_CRITERIA_PER_CUSTOMER"
                }
                ResourceLimitType::CustomerNegativeYoutubeChannelCriteriaPerCustomer => {
                    "CUSTOMER_NEGATIVE_YOUTUBE_CHANNEL_CRITERIA_PER_CUSTOMER"
                }
                ResourceLimitType::CriteriaPerAdGroup => "CRITERIA_PER_AD_GROUP",
                ResourceLimitType::ListingGroupsPerAdGroup => {
                    "LISTING_GROUPS_PER_AD_GROUP"
                }
                ResourceLimitType::ExplicitlySharedBudgetsPerCustomer => {
                    "EXPLICITLY_SHARED_BUDGETS_PER_CUSTOMER"
                }
                ResourceLimitType::ImplicitlySharedBudgetsPerCustomer => {
                    "IMPLICITLY_SHARED_BUDGETS_PER_CUSTOMER"
                }
                ResourceLimitType::CombinedAudienceCriteriaPerCampaign => {
                    "COMBINED_AUDIENCE_CRITERIA_PER_CAMPAIGN"
                }
                ResourceLimitType::NegativeKeywordsPerCampaign => {
                    "NEGATIVE_KEYWORDS_PER_CAMPAIGN"
                }
                ResourceLimitType::NegativePlacementsPerCampaign => {
                    "NEGATIVE_PLACEMENTS_PER_CAMPAIGN"
                }
                ResourceLimitType::GeoTargetsPerCampaign => "GEO_TARGETS_PER_CAMPAIGN",
                ResourceLimitType::NegativeIpBlocksPerCampaign => {
                    "NEGATIVE_IP_BLOCKS_PER_CAMPAIGN"
                }
                ResourceLimitType::ProximitiesPerCampaign => "PROXIMITIES_PER_CAMPAIGN",
                ResourceLimitType::ListingScopesPerShoppingCampaign => {
                    "LISTING_SCOPES_PER_SHOPPING_CAMPAIGN"
                }
                ResourceLimitType::ListingScopesPerNonShoppingCampaign => {
                    "LISTING_SCOPES_PER_NON_SHOPPING_CAMPAIGN"
                }
                ResourceLimitType::NegativeKeywordsPerSharedSet => {
                    "NEGATIVE_KEYWORDS_PER_SHARED_SET"
                }
                ResourceLimitType::NegativePlacementsPerSharedSet => {
                    "NEGATIVE_PLACEMENTS_PER_SHARED_SET"
                }
                ResourceLimitType::SharedSetsPerCustomerForTypeDefault => {
                    "SHARED_SETS_PER_CUSTOMER_FOR_TYPE_DEFAULT"
                }
                ResourceLimitType::SharedSetsPerCustomerForNegativePlacementListLower => {
                    "SHARED_SETS_PER_CUSTOMER_FOR_NEGATIVE_PLACEMENT_LIST_LOWER"
                }
                ResourceLimitType::HotelAdvanceBookingWindowBidModifiersPerAdGroup => {
                    "HOTEL_ADVANCE_BOOKING_WINDOW_BID_MODIFIERS_PER_AD_GROUP"
                }
                ResourceLimitType::BiddingStrategiesPerCustomer => {
                    "BIDDING_STRATEGIES_PER_CUSTOMER"
                }
                ResourceLimitType::BasicUserListsPerCustomer => {
                    "BASIC_USER_LISTS_PER_CUSTOMER"
                }
                ResourceLimitType::LogicalUserListsPerCustomer => {
                    "LOGICAL_USER_LISTS_PER_CUSTOMER"
                }
                ResourceLimitType::RuleBasedUserListsPerCustomer => {
                    "RULE_BASED_USER_LISTS_PER_CUSTOMER"
                }
                ResourceLimitType::BaseAdGroupAdsPerCustomer => {
                    "BASE_AD_GROUP_ADS_PER_CUSTOMER"
                }
                ResourceLimitType::ExperimentAdGroupAdsPerCustomer => {
                    "EXPERIMENT_AD_GROUP_ADS_PER_CUSTOMER"
                }
                ResourceLimitType::AdGroupAdsPerCampaign => "AD_GROUP_ADS_PER_CAMPAIGN",
                ResourceLimitType::TextAndOtherAdsPerAdGroup => {
                    "TEXT_AND_OTHER_ADS_PER_AD_GROUP"
                }
                ResourceLimitType::ImageAdsPerAdGroup => "IMAGE_ADS_PER_AD_GROUP",
                ResourceLimitType::ShoppingSmartAdsPerAdGroup => {
                    "SHOPPING_SMART_ADS_PER_AD_GROUP"
                }
                ResourceLimitType::ResponsiveSearchAdsPerAdGroup => {
                    "RESPONSIVE_SEARCH_ADS_PER_AD_GROUP"
                }
                ResourceLimitType::AppAdsPerAdGroup => "APP_ADS_PER_AD_GROUP",
                ResourceLimitType::AppEngagementAdsPerAdGroup => {
                    "APP_ENGAGEMENT_ADS_PER_AD_GROUP"
                }
                ResourceLimitType::LocalAdsPerAdGroup => "LOCAL_ADS_PER_AD_GROUP",
                ResourceLimitType::VideoAdsPerAdGroup => "VIDEO_ADS_PER_AD_GROUP",
                ResourceLimitType::LeadFormCampaignAssetsPerCampaign => {
                    "LEAD_FORM_CAMPAIGN_ASSETS_PER_CAMPAIGN"
                }
                ResourceLimitType::PromotionCustomerAssetsPerCustomer => {
                    "PROMOTION_CUSTOMER_ASSETS_PER_CUSTOMER"
                }
                ResourceLimitType::PromotionCampaignAssetsPerCampaign => {
                    "PROMOTION_CAMPAIGN_ASSETS_PER_CAMPAIGN"
                }
                ResourceLimitType::PromotionAdGroupAssetsPerAdGroup => {
                    "PROMOTION_AD_GROUP_ASSETS_PER_AD_GROUP"
                }
                ResourceLimitType::CalloutCustomerAssetsPerCustomer => {
                    "CALLOUT_CUSTOMER_ASSETS_PER_CUSTOMER"
                }
                ResourceLimitType::CalloutCampaignAssetsPerCampaign => {
                    "CALLOUT_CAMPAIGN_ASSETS_PER_CAMPAIGN"
                }
                ResourceLimitType::CalloutAdGroupAssetsPerAdGroup => {
                    "CALLOUT_AD_GROUP_ASSETS_PER_AD_GROUP"
                }
                ResourceLimitType::SitelinkCustomerAssetsPerCustomer => {
                    "SITELINK_CUSTOMER_ASSETS_PER_CUSTOMER"
                }
                ResourceLimitType::SitelinkCampaignAssetsPerCampaign => {
                    "SITELINK_CAMPAIGN_ASSETS_PER_CAMPAIGN"
                }
                ResourceLimitType::SitelinkAdGroupAssetsPerAdGroup => {
                    "SITELINK_AD_GROUP_ASSETS_PER_AD_GROUP"
                }
                ResourceLimitType::StructuredSnippetCustomerAssetsPerCustomer => {
                    "STRUCTURED_SNIPPET_CUSTOMER_ASSETS_PER_CUSTOMER"
                }
                ResourceLimitType::StructuredSnippetCampaignAssetsPerCampaign => {
                    "STRUCTURED_SNIPPET_CAMPAIGN_ASSETS_PER_CAMPAIGN"
                }
                ResourceLimitType::StructuredSnippetAdGroupAssetsPerAdGroup => {
                    "STRUCTURED_SNIPPET_AD_GROUP_ASSETS_PER_AD_GROUP"
                }
                ResourceLimitType::MobileAppCustomerAssetsPerCustomer => {
                    "MOBILE_APP_CUSTOMER_ASSETS_PER_CUSTOMER"
                }
                ResourceLimitType::MobileAppCampaignAssetsPerCampaign => {
                    "MOBILE_APP_CAMPAIGN_ASSETS_PER_CAMPAIGN"
                }
                ResourceLimitType::MobileAppAdGroupAssetsPerAdGroup => {
                    "MOBILE_APP_AD_GROUP_ASSETS_PER_AD_GROUP"
                }
                ResourceLimitType::HotelCalloutCustomerAssetsPerCustomer => {
                    "HOTEL_CALLOUT_CUSTOMER_ASSETS_PER_CUSTOMER"
                }
                ResourceLimitType::HotelCalloutCampaignAssetsPerCampaign => {
                    "HOTEL_CALLOUT_CAMPAIGN_ASSETS_PER_CAMPAIGN"
                }
                ResourceLimitType::HotelCalloutAdGroupAssetsPerAdGroup => {
                    "HOTEL_CALLOUT_AD_GROUP_ASSETS_PER_AD_GROUP"
                }
                ResourceLimitType::CallCustomerAssetsPerCustomer => {
                    "CALL_CUSTOMER_ASSETS_PER_CUSTOMER"
                }
                ResourceLimitType::CallCampaignAssetsPerCampaign => {
                    "CALL_CAMPAIGN_ASSETS_PER_CAMPAIGN"
                }
                ResourceLimitType::CallAdGroupAssetsPerAdGroup => {
                    "CALL_AD_GROUP_ASSETS_PER_AD_GROUP"
                }
                ResourceLimitType::PriceCustomerAssetsPerCustomer => {
                    "PRICE_CUSTOMER_ASSETS_PER_CUSTOMER"
                }
                ResourceLimitType::PriceCampaignAssetsPerCampaign => {
                    "PRICE_CAMPAIGN_ASSETS_PER_CAMPAIGN"
                }
                ResourceLimitType::PriceAdGroupAssetsPerAdGroup => {
                    "PRICE_AD_GROUP_ASSETS_PER_AD_GROUP"
                }
                ResourceLimitType::PageFeedAssetSetsPerCustomer => {
                    "PAGE_FEED_ASSET_SETS_PER_CUSTOMER"
                }
                ResourceLimitType::DynamicEducationFeedAssetSetsPerCustomer => {
                    "DYNAMIC_EDUCATION_FEED_ASSET_SETS_PER_CUSTOMER"
                }
                ResourceLimitType::AssetsPerPageFeedAssetSet => {
                    "ASSETS_PER_PAGE_FEED_ASSET_SET"
                }
                ResourceLimitType::AssetsPerDynamicEducationFeedAssetSet => {
                    "ASSETS_PER_DYNAMIC_EDUCATION_FEED_ASSET_SET"
                }
                ResourceLimitType::DynamicRealEstateAssetSetsPerCustomer => {
                    "DYNAMIC_REAL_ESTATE_ASSET_SETS_PER_CUSTOMER"
                }
                ResourceLimitType::AssetsPerDynamicRealEstateAssetSet => {
                    "ASSETS_PER_DYNAMIC_REAL_ESTATE_ASSET_SET"
                }
                ResourceLimitType::DynamicCustomAssetSetsPerCustomer => {
                    "DYNAMIC_CUSTOM_ASSET_SETS_PER_CUSTOMER"
                }
                ResourceLimitType::AssetsPerDynamicCustomAssetSet => {
                    "ASSETS_PER_DYNAMIC_CUSTOM_ASSET_SET"
                }
                ResourceLimitType::DynamicHotelsAndRentalsAssetSetsPerCustomer => {
                    "DYNAMIC_HOTELS_AND_RENTALS_ASSET_SETS_PER_CUSTOMER"
                }
                ResourceLimitType::AssetsPerDynamicHotelsAndRentalsAssetSet => {
                    "ASSETS_PER_DYNAMIC_HOTELS_AND_RENTALS_ASSET_SET"
                }
                ResourceLimitType::DynamicLocalAssetSetsPerCustomer => {
                    "DYNAMIC_LOCAL_ASSET_SETS_PER_CUSTOMER"
                }
                ResourceLimitType::AssetsPerDynamicLocalAssetSet => {
                    "ASSETS_PER_DYNAMIC_LOCAL_ASSET_SET"
                }
                ResourceLimitType::DynamicFlightsAssetSetsPerCustomer => {
                    "DYNAMIC_FLIGHTS_ASSET_SETS_PER_CUSTOMER"
                }
                ResourceLimitType::AssetsPerDynamicFlightsAssetSet => {
                    "ASSETS_PER_DYNAMIC_FLIGHTS_ASSET_SET"
                }
                ResourceLimitType::DynamicTravelAssetSetsPerCustomer => {
                    "DYNAMIC_TRAVEL_ASSET_SETS_PER_CUSTOMER"
                }
                ResourceLimitType::AssetsPerDynamicTravelAssetSet => {
                    "ASSETS_PER_DYNAMIC_TRAVEL_ASSET_SET"
                }
                ResourceLimitType::DynamicJobsAssetSetsPerCustomer => {
                    "DYNAMIC_JOBS_ASSET_SETS_PER_CUSTOMER"
                }
                ResourceLimitType::AssetsPerDynamicJobsAssetSet => {
                    "ASSETS_PER_DYNAMIC_JOBS_ASSET_SET"
                }
                ResourceLimitType::VersionsPerAd => "VERSIONS_PER_AD",
                ResourceLimitType::UserFeedsPerCustomer => "USER_FEEDS_PER_CUSTOMER",
                ResourceLimitType::SystemFeedsPerCustomer => "SYSTEM_FEEDS_PER_CUSTOMER",
                ResourceLimitType::FeedAttributesPerFeed => "FEED_ATTRIBUTES_PER_FEED",
                ResourceLimitType::FeedItemsPerCustomer => "FEED_ITEMS_PER_CUSTOMER",
                ResourceLimitType::CampaignFeedsPerCustomer => {
                    "CAMPAIGN_FEEDS_PER_CUSTOMER"
                }
                ResourceLimitType::BaseCampaignFeedsPerCustomer => {
                    "BASE_CAMPAIGN_FEEDS_PER_CUSTOMER"
                }
                ResourceLimitType::ExperimentCampaignFeedsPerCustomer => {
                    "EXPERIMENT_CAMPAIGN_FEEDS_PER_CUSTOMER"
                }
                ResourceLimitType::AdGroupFeedsPerCustomer => {
                    "AD_GROUP_FEEDS_PER_CUSTOMER"
                }
                ResourceLimitType::BaseAdGroupFeedsPerCustomer => {
                    "BASE_AD_GROUP_FEEDS_PER_CUSTOMER"
                }
                ResourceLimitType::ExperimentAdGroupFeedsPerCustomer => {
                    "EXPERIMENT_AD_GROUP_FEEDS_PER_CUSTOMER"
                }
                ResourceLimitType::AdGroupFeedsPerCampaign => {
                    "AD_GROUP_FEEDS_PER_CAMPAIGN"
                }
                ResourceLimitType::FeedItemSetsPerCustomer => {
                    "FEED_ITEM_SETS_PER_CUSTOMER"
                }
                ResourceLimitType::FeedItemsPerFeedItemSet => {
                    "FEED_ITEMS_PER_FEED_ITEM_SET"
                }
                ResourceLimitType::CampaignExperimentsPerCustomer => {
                    "CAMPAIGN_EXPERIMENTS_PER_CUSTOMER"
                }
                ResourceLimitType::ExperimentArmsPerVideoExperiment => {
                    "EXPERIMENT_ARMS_PER_VIDEO_EXPERIMENT"
                }
                ResourceLimitType::OwnedLabelsPerCustomer => "OWNED_LABELS_PER_CUSTOMER",
                ResourceLimitType::LabelsPerCampaign => "LABELS_PER_CAMPAIGN",
                ResourceLimitType::LabelsPerAdGroup => "LABELS_PER_AD_GROUP",
                ResourceLimitType::LabelsPerAdGroupAd => "LABELS_PER_AD_GROUP_AD",
                ResourceLimitType::LabelsPerAdGroupCriterion => {
                    "LABELS_PER_AD_GROUP_CRITERION"
                }
                ResourceLimitType::TargetCustomersPerLabel => {
                    "TARGET_CUSTOMERS_PER_LABEL"
                }
                ResourceLimitType::KeywordPlansPerUserPerCustomer => {
                    "KEYWORD_PLANS_PER_USER_PER_CUSTOMER"
                }
                ResourceLimitType::KeywordPlanAdGroupKeywordsPerKeywordPlan => {
                    "KEYWORD_PLAN_AD_GROUP_KEYWORDS_PER_KEYWORD_PLAN"
                }
                ResourceLimitType::KeywordPlanAdGroupsPerKeywordPlan => {
                    "KEYWORD_PLAN_AD_GROUPS_PER_KEYWORD_PLAN"
                }
                ResourceLimitType::KeywordPlanNegativeKeywordsPerKeywordPlan => {
                    "KEYWORD_PLAN_NEGATIVE_KEYWORDS_PER_KEYWORD_PLAN"
                }
                ResourceLimitType::KeywordPlanCampaignsPerKeywordPlan => {
                    "KEYWORD_PLAN_CAMPAIGNS_PER_KEYWORD_PLAN"
                }
                ResourceLimitType::ConversionActionsPerCustomer => {
                    "CONVERSION_ACTIONS_PER_CUSTOMER"
                }
                ResourceLimitType::BatchJobOperationsPerJob => {
                    "BATCH_JOB_OPERATIONS_PER_JOB"
                }
                ResourceLimitType::BatchJobsPerCustomer => "BATCH_JOBS_PER_CUSTOMER",
                ResourceLimitType::HotelCheckInDateRangeBidModifiersPerAdGroup => {
                    "HOTEL_CHECK_IN_DATE_RANGE_BID_MODIFIERS_PER_AD_GROUP"
                }
            }
        }
    }
}
/// Container for possible response content types.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseContentTypeEnum {}
/// Nested message and enum types in `ResponseContentTypeEnum`.
pub mod response_content_type_enum {
    /// Possible response content types.
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
    pub enum ResponseContentType {
        /// Not specified. Will return the resource name only in the response.
        Unspecified = 0,
        /// The mutate response will be the resource name.
        ResourceNameOnly = 1,
        /// The mutate response will be the resource name and the resource with
        /// all mutable fields.
        MutableResource = 2,
    }
    impl ResponseContentType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ResponseContentType::Unspecified => "UNSPECIFIED",
                ResponseContentType::ResourceNameOnly => "RESOURCE_NAME_ONLY",
                ResponseContentType::MutableResource => "MUTABLE_RESOURCE",
            }
        }
    }
}
/// Container for enum indicating whether a search term is one of your targeted
/// or excluded keywords.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchTermTargetingStatusEnum {}
/// Nested message and enum types in `SearchTermTargetingStatusEnum`.
pub mod search_term_targeting_status_enum {
    /// Indicates whether the search term is one of your targeted or excluded
    /// keywords.
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
    impl SearchTermTargetingStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                SearchTermTargetingStatus::Unspecified => "UNSPECIFIED",
                SearchTermTargetingStatus::Unknown => "UNKNOWN",
                SearchTermTargetingStatus::Added => "ADDED",
                SearchTermTargetingStatus::Excluded => "EXCLUDED",
                SearchTermTargetingStatus::AddedExcluded => "ADDED_EXCLUDED",
                SearchTermTargetingStatus::None => "NONE",
            }
        }
    }
}
/// Message describing seasonality event scopes. The two types of seasonality
/// events are BiddingSeasonalityAdjustments and BiddingDataExclusions.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SeasonalityEventScopeEnum {}
/// Nested message and enum types in `SeasonalityEventScopeEnum`.
pub mod seasonality_event_scope_enum {
    /// The possible scopes of a Seasonality Event.
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
    pub enum SeasonalityEventScope {
        /// No value has been specified.
        Unspecified = 0,
        /// The received value is not known in this version.
        ///
        /// This is a response-only value.
        Unknown = 1,
        /// The seasonality event is applied to all the customer's traffic for
        /// supported advertising channel types and device types. The CUSTOMER scope
        /// cannot be used in mutates.
        Customer = 2,
        /// The seasonality event is applied to all specified campaigns.
        Campaign = 4,
        /// The seasonality event is applied to all campaigns that belong to
        /// specified channel types.
        Channel = 5,
    }
    impl SeasonalityEventScope {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                SeasonalityEventScope::Unspecified => "UNSPECIFIED",
                SeasonalityEventScope::Unknown => "UNKNOWN",
                SeasonalityEventScope::Customer => "CUSTOMER",
                SeasonalityEventScope::Campaign => "CAMPAIGN",
                SeasonalityEventScope::Channel => "CHANNEL",
            }
        }
    }
}
/// Message describing seasonality event statuses. The two types of seasonality
/// events are BiddingSeasonalityAdjustments and BiddingDataExclusions.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SeasonalityEventStatusEnum {}
/// Nested message and enum types in `SeasonalityEventStatusEnum`.
pub mod seasonality_event_status_enum {
    /// The possible statuses of a Seasonality Event.
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
    pub enum SeasonalityEventStatus {
        /// No value has been specified.
        Unspecified = 0,
        /// The received value is not known in this version.
        ///
        /// This is a response-only value.
        Unknown = 1,
        /// The seasonality event is enabled.
        Enabled = 2,
        /// The seasonality event is removed.
        Removed = 4,
    }
    impl SeasonalityEventStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                SeasonalityEventStatus::Unspecified => "UNSPECIFIED",
                SeasonalityEventStatus::Unknown => "UNKNOWN",
                SeasonalityEventStatus::Enabled => "ENABLED",
                SeasonalityEventStatus::Removed => "REMOVED",
            }
        }
    }
}
/// Container for enum describing types of shared set statuses.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SharedSetStatusEnum {}
/// Nested message and enum types in `SharedSetStatusEnum`.
pub mod shared_set_status_enum {
    /// Enum listing the possible shared set statuses.
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
    impl SharedSetStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                SharedSetStatus::Unspecified => "UNSPECIFIED",
                SharedSetStatus::Unknown => "UNKNOWN",
                SharedSetStatus::Enabled => "ENABLED",
                SharedSetStatus::Removed => "REMOVED",
            }
        }
    }
}
/// Container for enum describing types of shared sets.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SharedSetTypeEnum {}
/// Nested message and enum types in `SharedSetTypeEnum`.
pub mod shared_set_type_enum {
    /// Enum listing the possible shared set types.
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
    impl SharedSetType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                SharedSetType::Unspecified => "UNSPECIFIED",
                SharedSetType::Unknown => "UNKNOWN",
                SharedSetType::NegativeKeywords => "NEGATIVE_KEYWORDS",
                SharedSetType::NegativePlacements => "NEGATIVE_PLACEMENTS",
            }
        }
    }
}
/// Container for enum describing the method by which a simulation modifies
/// a field.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SimulationModificationMethodEnum {}
/// Nested message and enum types in `SimulationModificationMethodEnum`.
pub mod simulation_modification_method_enum {
    /// Enum describing the method by which a simulation modifies a field.
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
    impl SimulationModificationMethod {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                SimulationModificationMethod::Unspecified => "UNSPECIFIED",
                SimulationModificationMethod::Unknown => "UNKNOWN",
                SimulationModificationMethod::Uniform => "UNIFORM",
                SimulationModificationMethod::Default => "DEFAULT",
                SimulationModificationMethod::Scaling => "SCALING",
            }
        }
    }
}
/// Container for enum describing the field a simulation modifies.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SimulationTypeEnum {}
/// Nested message and enum types in `SimulationTypeEnum`.
pub mod simulation_type_enum {
    /// Enum describing the field a simulation modifies.
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
    impl SimulationType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                SimulationType::Unspecified => "UNSPECIFIED",
                SimulationType::Unknown => "UNKNOWN",
                SimulationType::CpcBid => "CPC_BID",
                SimulationType::CpvBid => "CPV_BID",
                SimulationType::TargetCpa => "TARGET_CPA",
                SimulationType::BidModifier => "BID_MODIFIER",
                SimulationType::TargetRoas => "TARGET_ROAS",
                SimulationType::PercentCpcBid => "PERCENT_CPC_BID",
                SimulationType::TargetImpressionShare => "TARGET_IMPRESSION_SHARE",
                SimulationType::Budget => "BUDGET",
            }
        }
    }
}
/// Values for Sitelink placeholder fields.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SitelinkPlaceholderFieldEnum {}
/// Nested message and enum types in `SitelinkPlaceholderFieldEnum`.
pub mod sitelink_placeholder_field_enum {
    /// Possible values for Sitelink placeholder fields.
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
    impl SitelinkPlaceholderField {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                SitelinkPlaceholderField::Unspecified => "UNSPECIFIED",
                SitelinkPlaceholderField::Unknown => "UNKNOWN",
                SitelinkPlaceholderField::Text => "TEXT",
                SitelinkPlaceholderField::Line1 => "LINE_1",
                SitelinkPlaceholderField::Line2 => "LINE_2",
                SitelinkPlaceholderField::FinalUrls => "FINAL_URLS",
                SitelinkPlaceholderField::FinalMobileUrls => "FINAL_MOBILE_URLS",
                SitelinkPlaceholderField::TrackingUrl => "TRACKING_URL",
                SitelinkPlaceholderField::FinalUrlSuffix => "FINAL_URL_SUFFIX",
            }
        }
    }
}
/// Message describing spending limit types.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpendingLimitTypeEnum {}
/// Nested message and enum types in `SpendingLimitTypeEnum`.
pub mod spending_limit_type_enum {
    /// The possible spending limit types used by certain resources as an
    /// alternative to absolute money values in micros.
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
    pub enum SpendingLimitType {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Infinite, indicates unlimited spending power.
        Infinite = 2,
    }
    impl SpendingLimitType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                SpendingLimitType::Unspecified => "UNSPECIFIED",
                SpendingLimitType::Unknown => "UNKNOWN",
                SpendingLimitType::Infinite => "INFINITE",
            }
        }
    }
}
/// Values for Structured Snippet placeholder fields.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StructuredSnippetPlaceholderFieldEnum {}
/// Nested message and enum types in `StructuredSnippetPlaceholderFieldEnum`.
pub mod structured_snippet_placeholder_field_enum {
    /// Possible values for Structured Snippet placeholder fields.
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
    impl StructuredSnippetPlaceholderField {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                StructuredSnippetPlaceholderField::Unspecified => "UNSPECIFIED",
                StructuredSnippetPlaceholderField::Unknown => "UNKNOWN",
                StructuredSnippetPlaceholderField::Header => "HEADER",
                StructuredSnippetPlaceholderField::Snippets => "SNIPPETS",
            }
        }
    }
}
/// Indicates summary row setting in request parameter.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SummaryRowSettingEnum {}
/// Nested message and enum types in `SummaryRowSettingEnum`.
pub mod summary_row_setting_enum {
    /// Enum describing return summary row settings.
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
    impl SummaryRowSetting {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                SummaryRowSetting::Unspecified => "UNSPECIFIED",
                SummaryRowSetting::Unknown => "UNKNOWN",
                SummaryRowSetting::NoSummaryRow => "NO_SUMMARY_ROW",
                SummaryRowSetting::SummaryRowWithResults => "SUMMARY_ROW_WITH_RESULTS",
                SummaryRowSetting::SummaryRowOnly => "SUMMARY_ROW_ONLY",
            }
        }
    }
}
/// Container for enum describing possible system managed entity sources.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SystemManagedResourceSourceEnum {}
/// Nested message and enum types in `SystemManagedResourceSourceEnum`.
pub mod system_managed_resource_source_enum {
    /// Enum listing the possible system managed entity sources.
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
    pub enum SystemManagedResourceSource {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Generated ad variations experiment ad.
        AdVariations = 2,
    }
    impl SystemManagedResourceSource {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                SystemManagedResourceSource::Unspecified => "UNSPECIFIED",
                SystemManagedResourceSource::Unknown => "UNKNOWN",
                SystemManagedResourceSource::AdVariations => "AD_VARIATIONS",
            }
        }
    }
}
/// Container for enum describing goals for TargetCpaOptIn recommendation.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TargetCpaOptInRecommendationGoalEnum {}
/// Nested message and enum types in `TargetCpaOptInRecommendationGoalEnum`.
pub mod target_cpa_opt_in_recommendation_goal_enum {
    /// Goal of TargetCpaOptIn recommendation.
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
    impl TargetCpaOptInRecommendationGoal {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                TargetCpaOptInRecommendationGoal::Unspecified => "UNSPECIFIED",
                TargetCpaOptInRecommendationGoal::Unknown => "UNKNOWN",
                TargetCpaOptInRecommendationGoal::SameCost => "SAME_COST",
                TargetCpaOptInRecommendationGoal::SameConversions => "SAME_CONVERSIONS",
                TargetCpaOptInRecommendationGoal::SameCpa => "SAME_CPA",
                TargetCpaOptInRecommendationGoal::ClosestCpa => "CLOSEST_CPA",
            }
        }
    }
}
/// Message describing time types.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TimeTypeEnum {}
/// Nested message and enum types in `TimeTypeEnum`.
pub mod time_type_enum {
    /// The possible time types used by certain resources as an alternative to
    /// absolute timestamps.
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
    impl TimeType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                TimeType::Unspecified => "UNSPECIFIED",
                TimeType::Unknown => "UNKNOWN",
                TimeType::Now => "NOW",
                TimeType::Forever => "FOREVER",
            }
        }
    }
}
/// Values for Travel placeholder fields.
/// For more information about dynamic remarketing feeds, see
/// <https://support.google.com/google-ads/answer/6053288.>
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TravelPlaceholderFieldEnum {}
/// Nested message and enum types in `TravelPlaceholderFieldEnum`.
pub mod travel_placeholder_field_enum {
    /// Possible values for Travel placeholder fields.
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
        /// Upgraded URLs; the more specific the better (for example, the individual
        /// URL of a specific travel offer and its location).
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
        ///    scheme.
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
    impl TravelPlaceholderField {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                TravelPlaceholderField::Unspecified => "UNSPECIFIED",
                TravelPlaceholderField::Unknown => "UNKNOWN",
                TravelPlaceholderField::DestinationId => "DESTINATION_ID",
                TravelPlaceholderField::OriginId => "ORIGIN_ID",
                TravelPlaceholderField::Title => "TITLE",
                TravelPlaceholderField::DestinationName => "DESTINATION_NAME",
                TravelPlaceholderField::OriginName => "ORIGIN_NAME",
                TravelPlaceholderField::Price => "PRICE",
                TravelPlaceholderField::FormattedPrice => "FORMATTED_PRICE",
                TravelPlaceholderField::SalePrice => "SALE_PRICE",
                TravelPlaceholderField::FormattedSalePrice => "FORMATTED_SALE_PRICE",
                TravelPlaceholderField::ImageUrl => "IMAGE_URL",
                TravelPlaceholderField::Category => "CATEGORY",
                TravelPlaceholderField::ContextualKeywords => "CONTEXTUAL_KEYWORDS",
                TravelPlaceholderField::DestinationAddress => "DESTINATION_ADDRESS",
                TravelPlaceholderField::FinalUrl => "FINAL_URL",
                TravelPlaceholderField::FinalMobileUrls => "FINAL_MOBILE_URLS",
                TravelPlaceholderField::TrackingUrl => "TRACKING_URL",
                TravelPlaceholderField::AndroidAppLink => "ANDROID_APP_LINK",
                TravelPlaceholderField::SimilarDestinationIds => {
                    "SIMILAR_DESTINATION_IDS"
                }
                TravelPlaceholderField::IosAppLink => "IOS_APP_LINK",
                TravelPlaceholderField::IosAppStoreId => "IOS_APP_STORE_ID",
            }
        }
    }
}
/// Message describing a UserInterestTaxonomyType.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserInterestTaxonomyTypeEnum {}
/// Nested message and enum types in `UserInterestTaxonomyTypeEnum`.
pub mod user_interest_taxonomy_type_enum {
    /// Enum containing the possible UserInterestTaxonomyTypes.
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
    impl UserInterestTaxonomyType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                UserInterestTaxonomyType::Unspecified => "UNSPECIFIED",
                UserInterestTaxonomyType::Unknown => "UNKNOWN",
                UserInterestTaxonomyType::Affinity => "AFFINITY",
                UserInterestTaxonomyType::InMarket => "IN_MARKET",
                UserInterestTaxonomyType::MobileAppInstallUser => {
                    "MOBILE_APP_INSTALL_USER"
                }
                UserInterestTaxonomyType::VerticalGeo => "VERTICAL_GEO",
                UserInterestTaxonomyType::NewSmartPhoneUser => "NEW_SMART_PHONE_USER",
            }
        }
    }
}
/// Indicates if this client still has access to the list.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserListAccessStatusEnum {}
/// Nested message and enum types in `UserListAccessStatusEnum`.
pub mod user_list_access_status_enum {
    /// Enum containing possible user list access statuses.
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
    impl UserListAccessStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                UserListAccessStatus::Unspecified => "UNSPECIFIED",
                UserListAccessStatus::Unknown => "UNKNOWN",
                UserListAccessStatus::Enabled => "ENABLED",
                UserListAccessStatus::Disabled => "DISABLED",
            }
        }
    }
}
/// Indicates the reason why the userlist was closed.
/// This enum is only used when a list is auto-closed by the system.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserListClosingReasonEnum {}
/// Nested message and enum types in `UserListClosingReasonEnum`.
pub mod user_list_closing_reason_enum {
    /// Enum describing possible user list closing reasons.
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
    pub enum UserListClosingReason {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// The userlist was closed because of not being used for over one year.
        Unused = 2,
    }
    impl UserListClosingReason {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                UserListClosingReason::Unspecified => "UNSPECIFIED",
                UserListClosingReason::Unknown => "UNKNOWN",
                UserListClosingReason::Unused => "UNUSED",
            }
        }
    }
}
/// Membership status of this user list. Indicates whether a user list is open
/// or active. Only open user lists can accumulate more users and can be used for
/// targeting.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserListMembershipStatusEnum {}
/// Nested message and enum types in `UserListMembershipStatusEnum`.
pub mod user_list_membership_status_enum {
    /// Enum containing possible user list membership statuses.
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
    impl UserListMembershipStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                UserListMembershipStatus::Unspecified => "UNSPECIFIED",
                UserListMembershipStatus::Unknown => "UNKNOWN",
                UserListMembershipStatus::Open => "OPEN",
                UserListMembershipStatus::Closed => "CLOSED",
            }
        }
    }
}
/// Size range in terms of number of users of a UserList.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserListSizeRangeEnum {}
/// Nested message and enum types in `UserListSizeRangeEnum`.
pub mod user_list_size_range_enum {
    /// Enum containing possible user list size ranges.
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
    impl UserListSizeRange {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                UserListSizeRange::Unspecified => "UNSPECIFIED",
                UserListSizeRange::Unknown => "UNKNOWN",
                UserListSizeRange::LessThanFiveHundred => "LESS_THAN_FIVE_HUNDRED",
                UserListSizeRange::LessThanOneThousand => "LESS_THAN_ONE_THOUSAND",
                UserListSizeRange::OneThousandToTenThousand => {
                    "ONE_THOUSAND_TO_TEN_THOUSAND"
                }
                UserListSizeRange::TenThousandToFiftyThousand => {
                    "TEN_THOUSAND_TO_FIFTY_THOUSAND"
                }
                UserListSizeRange::FiftyThousandToOneHundredThousand => {
                    "FIFTY_THOUSAND_TO_ONE_HUNDRED_THOUSAND"
                }
                UserListSizeRange::OneHundredThousandToThreeHundredThousand => {
                    "ONE_HUNDRED_THOUSAND_TO_THREE_HUNDRED_THOUSAND"
                }
                UserListSizeRange::ThreeHundredThousandToFiveHundredThousand => {
                    "THREE_HUNDRED_THOUSAND_TO_FIVE_HUNDRED_THOUSAND"
                }
                UserListSizeRange::FiveHundredThousandToOneMillion => {
                    "FIVE_HUNDRED_THOUSAND_TO_ONE_MILLION"
                }
                UserListSizeRange::OneMillionToTwoMillion => "ONE_MILLION_TO_TWO_MILLION",
                UserListSizeRange::TwoMillionToThreeMillion => {
                    "TWO_MILLION_TO_THREE_MILLION"
                }
                UserListSizeRange::ThreeMillionToFiveMillion => {
                    "THREE_MILLION_TO_FIVE_MILLION"
                }
                UserListSizeRange::FiveMillionToTenMillion => {
                    "FIVE_MILLION_TO_TEN_MILLION"
                }
                UserListSizeRange::TenMillionToTwentyMillion => {
                    "TEN_MILLION_TO_TWENTY_MILLION"
                }
                UserListSizeRange::TwentyMillionToThirtyMillion => {
                    "TWENTY_MILLION_TO_THIRTY_MILLION"
                }
                UserListSizeRange::ThirtyMillionToFiftyMillion => {
                    "THIRTY_MILLION_TO_FIFTY_MILLION"
                }
                UserListSizeRange::OverFiftyMillion => "OVER_FIFTY_MILLION",
            }
        }
    }
}
/// The user list types.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserListTypeEnum {}
/// Nested message and enum types in `UserListTypeEnum`.
pub mod user_list_type_enum {
    /// Enum containing possible user list types.
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
    impl UserListType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                UserListType::Unspecified => "UNSPECIFIED",
                UserListType::Unknown => "UNKNOWN",
                UserListType::Remarketing => "REMARKETING",
                UserListType::Logical => "LOGICAL",
                UserListType::ExternalRemarketing => "EXTERNAL_REMARKETING",
                UserListType::RuleBased => "RULE_BASED",
                UserListType::Similar => "SIMILAR",
                UserListType::CrmBased => "CRM_BASED",
            }
        }
    }
}
/// Container for enum describing possible device types used in a conversion
/// value rule.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValueRuleDeviceTypeEnum {}
/// Nested message and enum types in `ValueRuleDeviceTypeEnum`.
pub mod value_rule_device_type_enum {
    /// Possible device types used in conversion value rule.
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
    pub enum ValueRuleDeviceType {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Mobile device.
        Mobile = 2,
        /// Desktop device.
        Desktop = 3,
        /// Tablet device.
        Tablet = 4,
    }
    impl ValueRuleDeviceType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ValueRuleDeviceType::Unspecified => "UNSPECIFIED",
                ValueRuleDeviceType::Unknown => "UNKNOWN",
                ValueRuleDeviceType::Mobile => "MOBILE",
                ValueRuleDeviceType::Desktop => "DESKTOP",
                ValueRuleDeviceType::Tablet => "TABLET",
            }
        }
    }
}
/// Container for enum describing possible geographic location matching types
/// used in a conversion value rule.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValueRuleGeoLocationMatchTypeEnum {}
/// Nested message and enum types in `ValueRuleGeoLocationMatchTypeEnum`.
pub mod value_rule_geo_location_match_type_enum {
    /// Possible geographic location matching types.
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
    pub enum ValueRuleGeoLocationMatchType {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Either Area of Interest or Location of Presence can be used to match.
        Any = 2,
        /// Only Location of Presence can be used to match.
        LocationOfPresence = 3,
    }
    impl ValueRuleGeoLocationMatchType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ValueRuleGeoLocationMatchType::Unspecified => "UNSPECIFIED",
                ValueRuleGeoLocationMatchType::Unknown => "UNKNOWN",
                ValueRuleGeoLocationMatchType::Any => "ANY",
                ValueRuleGeoLocationMatchType::LocationOfPresence => {
                    "LOCATION_OF_PRESENCE"
                }
            }
        }
    }
}
/// Container for enum describing possible operations for value rules which are
/// executed when rules are triggered.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValueRuleOperationEnum {}
/// Nested message and enum types in `ValueRuleOperationEnum`.
pub mod value_rule_operation_enum {
    /// Possible operations of the action of a conversion value rule.
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
    pub enum ValueRuleOperation {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Add provided value to conversion value.
        Add = 2,
        /// Multiply conversion value by provided value.
        Multiply = 3,
        /// Set conversion value to provided value.
        Set = 4,
    }
    impl ValueRuleOperation {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ValueRuleOperation::Unspecified => "UNSPECIFIED",
                ValueRuleOperation::Unknown => "UNKNOWN",
                ValueRuleOperation::Add => "ADD",
                ValueRuleOperation::Multiply => "MULTIPLY",
                ValueRuleOperation::Set => "SET",
            }
        }
    }
}
/// Container for enum describing where a value rule set is attached.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValueRuleSetAttachmentTypeEnum {}
/// Nested message and enum types in `ValueRuleSetAttachmentTypeEnum`.
pub mod value_rule_set_attachment_type_enum {
    /// Possible level where a value rule set is attached.
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
    pub enum ValueRuleSetAttachmentType {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Attached to the customer.
        Customer = 2,
        /// Attached to a campaign.
        Campaign = 3,
    }
    impl ValueRuleSetAttachmentType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ValueRuleSetAttachmentType::Unspecified => "UNSPECIFIED",
                ValueRuleSetAttachmentType::Unknown => "UNKNOWN",
                ValueRuleSetAttachmentType::Customer => "CUSTOMER",
                ValueRuleSetAttachmentType::Campaign => "CAMPAIGN",
            }
        }
    }
}
/// Container for enum describing possible dimensions of a conversion value rule
/// set.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValueRuleSetDimensionEnum {}
/// Nested message and enum types in `ValueRuleSetDimensionEnum`.
pub mod value_rule_set_dimension_enum {
    /// Possible dimensions of a conversion value rule set.
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
    pub enum ValueRuleSetDimension {
        /// Not specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Dimension for geo location.
        GeoLocation = 2,
        /// Dimension for device type.
        Device = 3,
        /// Dimension for audience.
        Audience = 4,
        /// This dimension implies the rule will always apply.
        NoCondition = 5,
    }
    impl ValueRuleSetDimension {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ValueRuleSetDimension::Unspecified => "UNSPECIFIED",
                ValueRuleSetDimension::Unknown => "UNKNOWN",
                ValueRuleSetDimension::GeoLocation => "GEO_LOCATION",
                ValueRuleSetDimension::Device => "DEVICE",
                ValueRuleSetDimension::Audience => "AUDIENCE",
                ValueRuleSetDimension::NoCondition => "NO_CONDITION",
            }
        }
    }
}
/// The display mode for vanity pharma URLs.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VanityPharmaDisplayUrlModeEnum {}
/// Nested message and enum types in `VanityPharmaDisplayUrlModeEnum`.
pub mod vanity_pharma_display_url_mode_enum {
    /// Enum describing possible display modes for vanity pharma URLs.
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
    impl VanityPharmaDisplayUrlMode {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                VanityPharmaDisplayUrlMode::Unspecified => "UNSPECIFIED",
                VanityPharmaDisplayUrlMode::Unknown => "UNKNOWN",
                VanityPharmaDisplayUrlMode::ManufacturerWebsiteUrl => {
                    "MANUFACTURER_WEBSITE_URL"
                }
                VanityPharmaDisplayUrlMode::WebsiteDescription => "WEBSITE_DESCRIPTION",
            }
        }
    }
}
/// The text that will be displayed in display URL of the text ad when website
/// description is the selected display mode for vanity pharma URLs.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VanityPharmaTextEnum {}
/// Nested message and enum types in `VanityPharmaTextEnum`.
pub mod vanity_pharma_text_enum {
    /// Enum describing possible text.
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
    impl VanityPharmaText {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                VanityPharmaText::Unspecified => "UNSPECIFIED",
                VanityPharmaText::Unknown => "UNKNOWN",
                VanityPharmaText::PrescriptionTreatmentWebsiteEn => {
                    "PRESCRIPTION_TREATMENT_WEBSITE_EN"
                }
                VanityPharmaText::PrescriptionTreatmentWebsiteEs => {
                    "PRESCRIPTION_TREATMENT_WEBSITE_ES"
                }
                VanityPharmaText::PrescriptionDeviceWebsiteEn => {
                    "PRESCRIPTION_DEVICE_WEBSITE_EN"
                }
                VanityPharmaText::PrescriptionDeviceWebsiteEs => {
                    "PRESCRIPTION_DEVICE_WEBSITE_ES"
                }
                VanityPharmaText::MedicalDeviceWebsiteEn => "MEDICAL_DEVICE_WEBSITE_EN",
                VanityPharmaText::MedicalDeviceWebsiteEs => "MEDICAL_DEVICE_WEBSITE_ES",
                VanityPharmaText::PreventativeTreatmentWebsiteEn => {
                    "PREVENTATIVE_TREATMENT_WEBSITE_EN"
                }
                VanityPharmaText::PreventativeTreatmentWebsiteEs => {
                    "PREVENTATIVE_TREATMENT_WEBSITE_ES"
                }
                VanityPharmaText::PrescriptionContraceptionWebsiteEn => {
                    "PRESCRIPTION_CONTRACEPTION_WEBSITE_EN"
                }
                VanityPharmaText::PrescriptionContraceptionWebsiteEs => {
                    "PRESCRIPTION_CONTRACEPTION_WEBSITE_ES"
                }
                VanityPharmaText::PrescriptionVaccineWebsiteEn => {
                    "PRESCRIPTION_VACCINE_WEBSITE_EN"
                }
                VanityPharmaText::PrescriptionVaccineWebsiteEs => {
                    "PRESCRIPTION_VACCINE_WEBSITE_ES"
                }
            }
        }
    }
}
