// Proto file describing feed item validation errors.

/// Container for enum describing possible validation errors of a feed item.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeedItemValidationErrorEnum {}
pub mod feed_item_validation_error_enum {
    /// The possible validation errors of a feed item.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum FeedItemValidationError {
        /// No value has been specified.
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// String is too short.
        StringTooShort = 2,
        /// String is too long.
        StringTooLong = 3,
        /// Value is not provided.
        ValueNotSpecified = 4,
        /// Phone number format is invalid for region.
        InvalidDomesticPhoneNumberFormat = 5,
        /// String does not represent a phone number.
        InvalidPhoneNumber = 6,
        /// Phone number format is not compatible with country code.
        PhoneNumberNotSupportedForCountry = 7,
        /// Premium rate number is not allowed.
        PremiumRateNumberNotAllowed = 8,
        /// Phone number type is not allowed.
        DisallowedNumberType = 9,
        /// Specified value is outside of the valid range.
        ValueOutOfRange = 10,
        /// Call tracking is not supported in the selected country.
        CalltrackingNotSupportedForCountry = 11,
        /// Customer is not whitelisted for call tracking.
        CustomerNotWhitelistedForCalltracking = 12,
        /// Country code is invalid.
        InvalidCountryCode = 13,
        /// The specified mobile app id is invalid.
        InvalidAppId = 14,
        /// Some required field attributes are missing.
        MissingAttributesForFields = 15,
        /// Invalid email button type for email extension.
        InvalidTypeId = 16,
        /// Email address is invalid.
        InvalidEmailAddress = 17,
        /// The HTTPS URL in email extension is invalid.
        InvalidHttpsUrl = 18,
        /// Delivery address is missing from email extension.
        MissingDeliveryAddress = 19,
        /// FeedItem scheduling start date comes after end date.
        StartDateAfterEndDate = 20,
        /// FeedItem scheduling start time is missing.
        MissingFeedItemStartTime = 21,
        /// FeedItem scheduling end time is missing.
        MissingFeedItemEndTime = 22,
        /// Cannot compute system attributes on a FeedItem that has no FeedItemId.
        MissingFeedItemId = 23,
        /// Call extension vanity phone numbers are not supported.
        VanityPhoneNumberNotAllowed = 24,
        /// Invalid review text.
        InvalidReviewExtensionSnippet = 25,
        /// Invalid format for numeric value in ad parameter.
        InvalidNumberFormat = 26,
        /// Invalid format for date value in ad parameter.
        InvalidDateFormat = 27,
        /// Invalid format for price value in ad parameter.
        InvalidPriceFormat = 28,
        /// Unrecognized type given for value in ad parameter.
        UnknownPlaceholderField = 29,
        /// Enhanced sitelinks must have both description lines specified.
        MissingEnhancedSitelinkDescriptionLine = 30,
        /// Review source is ineligible.
        ReviewExtensionSourceIneligible = 31,
        /// Review text cannot contain hyphens or dashes.
        HyphensInReviewExtensionSnippet = 32,
        /// Review text cannot contain double quote characters.
        DoubleQuotesInReviewExtensionSnippet = 33,
        /// Review text cannot contain quote characters.
        QuotesInReviewExtensionSnippet = 34,
        /// Parameters are encoded in the wrong format.
        InvalidFormEncodedParams = 35,
        /// URL parameter name must contain only letters, numbers, underscores, and
        /// dashes.
        InvalidUrlParameterName = 36,
        /// Cannot find address location.
        NoGeocodingResult = 37,
        /// Review extension text has source name.
        SourceNameInReviewExtensionText = 38,
        /// Some phone numbers can be shorter than usual. Some of these short numbers
        /// are carrier-specific, and we disallow those in ad extensions because they
        /// will not be available to all users.
        CarrierSpecificShortNumberNotAllowed = 39,
        /// Triggered when a request references a placeholder field id that does not
        /// exist.
        InvalidPlaceholderFieldId = 40,
        /// URL contains invalid ValueTrack tags or format.
        InvalidUrlTag = 41,
        /// Provided list exceeds acceptable size.
        ListTooLong = 42,
        /// Certain combinations of attributes aren't allowed to be specified in the
        /// same feed item.
        InvalidAttributesCombination = 43,
        /// An attribute has the same value repeatedly.
        DuplicateValues = 44,
        /// Advertisers can link a conversion action with a phone number to indicate
        /// that sufficiently long calls forwarded to that phone number should be
        /// counted as conversions of the specified type.  This is an error message
        /// indicating that the conversion action specified is invalid (e.g., the
        /// conversion action does not exist within the appropriate Google Ads
        /// account, or it is a type of conversion not appropriate to phone call
        /// conversions).
        InvalidCallConversionActionId = 45,
        /// Tracking template requires final url to be set.
        CannotSetWithoutFinalUrls = 46,
        /// An app id was provided that doesn't exist in the given app store.
        AppIdDoesntExistInAppStore = 47,
        /// Invalid U2 final url.
        InvalidFinalUrl = 48,
        /// Invalid U2 tracking url.
        InvalidTrackingUrl = 49,
        /// Final URL should start from App download URL.
        InvalidFinalUrlForAppDownloadUrl = 50,
        /// List provided is too short.
        ListTooShort = 51,
        /// User Action field has invalid value.
        InvalidUserAction = 52,
        /// Type field has invalid value.
        InvalidTypeName = 53,
        /// Change status for event is invalid.
        InvalidEventChangeStatus = 54,
        /// The header of a structured snippets extension is not one of the valid
        /// headers.
        InvalidSnippetsHeader = 55,
        /// Android app link is not formatted correctly
        InvalidAndroidAppLink = 56,
        /// Phone number incompatible with call tracking for country.
        NumberTypeWithCalltrackingNotSupportedForCountry = 57,
        /// The input is identical to a reserved keyword
        ReservedKeywordOther = 58,
        /// Each option label in the message extension must be unique.
        DuplicateOptionLabels = 59,
        /// Each option prefill in the message extension must be unique.
        DuplicateOptionPrefills = 60,
        /// In message extensions, the number of optional labels and optional
        /// prefills must be the same.
        UnequalListLengths = 61,
        /// All currency codes in an ad extension must be the same.
        InconsistentCurrencyCodes = 62,
        /// Headers in price extension are not unique.
        PriceExtensionHasDuplicatedHeaders = 63,
        /// Header and description in an item are the same.
        ItemHasDuplicatedHeaderAndDescription = 64,
        /// Price extension has too few items.
        PriceExtensionHasTooFewItems = 65,
        /// The given value is not supported.
        UnsupportedValue = 66,
        /// Invalid final mobile url.
        InvalidFinalMobileUrl = 67,
        /// The given string value of Label contains invalid characters
        InvalidKeywordlessAdRuleLabel = 68,
        /// The given URL contains value track parameters.
        ValueTrackParameterNotSupported = 69,
        /// The given value is not supported in the selected language of an
        /// extension.
        UnsupportedValueInSelectedLanguage = 70,
        /// The iOS app link is not formatted correctly.
        InvalidIosAppLink = 71,
        /// iOS app link or iOS app store id is missing.
        MissingIosAppLinkOrIosAppStoreId = 72,
        /// Promotion time is invalid.
        PromotionInvalidTime = 73,
        /// Both the percent off and money amount off fields are set.
        PromotionCannotSetPercentOffAndMoneyAmountOff = 74,
        /// Both the promotion code and orders over amount fields are set.
        PromotionCannotSetPromotionCodeAndOrdersOverAmount = 75,
        /// Too many decimal places are specified.
        TooManyDecimalPlacesSpecified = 76,
        /// Ad Customizers are present and not allowed.
        AdCustomizersNotAllowed = 77,
        /// Language code is not valid.
        InvalidLanguageCode = 78,
        /// Language is not supported.
        UnsupportedLanguage = 79,
        /// IF Function is present and not allowed.
        IfFunctionNotAllowed = 80,
        /// Final url suffix is not valid.
        InvalidFinalUrlSuffix = 81,
        /// Final url suffix contains an invalid tag.
        InvalidTagInFinalUrlSuffix = 82,
        /// Final url suffix is formatted incorrectly.
        InvalidFinalUrlSuffixFormat = 83,
        /// Consent for call recording, which is required for the use of call
        /// extensions, was not provided by the advertiser. Please see
        /// https://support.google.com/google-ads/answer/7412639.
        CustomerConsentForCallRecordingRequired = 84,
        /// Multiple message delivery options are set.
        OnlyOneDeliveryOptionIsAllowed = 85,
        /// No message delivery option is set.
        NoDeliveryOptionIsSet = 86,
        /// String value of conversion reporting state field is not valid.
        InvalidConversionReportingState = 87,
        /// Image size is not right.
        ImageSizeWrong = 88,
        /// Email delivery is not supported in the country specified in the country
        /// code field.
        EmailDeliveryNotAvailableInCountry = 89,
        /// Auto reply is not supported in the country specified in the country code
        /// field.
        AutoReplyNotAvailableInCountry = 90,
        /// Invalid value specified for latitude.
        InvalidLatitudeValue = 91,
        /// Invalid value specified for longitude.
        InvalidLongitudeValue = 92,
        /// Too many label fields provided.
        TooManyLabels = 93,
        /// Invalid image url.
        InvalidImageUrl = 94,
        /// Latitude value is missing.
        MissingLatitudeValue = 95,
        /// Longitude value is missing.
        MissingLongitudeValue = 96,
        /// Unable to find address.
        AddressNotFound = 97,
        /// Cannot target provided address.
        AddressNotTargetable = 98,
    }
}
