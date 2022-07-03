// Proto file describing AccessInvitation errors.

/// Container for enum describing possible AccessInvitation errors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccessInvitationErrorEnum {}
/// Nested message and enum types in `AccessInvitationErrorEnum`.
pub mod access_invitation_error_enum {
    /// Enum describing possible AccessInvitation errors.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum AccessInvitationError {
        /// Enum unspecified.
        Unspecified = 0,
        /// The received error code is not known in this version.
        Unknown = 1,
        /// The email address is invalid for sending an invitation.
        InvalidEmailAddress = 2,
        /// Email address already has access to this customer.
        EmailAddressAlreadyHasAccess = 3,
        /// Invalid invitation status for the operation.
        InvalidInvitationStatus = 4,
        /// Email address cannot be like abc+foo@google.com.
        GoogleConsumerAccountNotAllowed = 5,
        /// Invalid invitation id.
        InvalidInvitationId = 6,
        /// Email address already has a pending invitation.
        EmailAddressAlreadyHasPendingInvitation = 7,
        /// Pending invitation limit exceeded for the customer.
        PendingInvitationsLimitExceeded = 8,
        /// Email address doesn't conform to the email domain policy. Please see
        /// <https://support.google.com/google-ads/answer/2375456>
        EmailDomainPolicyViolated = 9,
    }
}
// Proto file describing account budget proposal errors.

/// Container for enum describing possible account budget proposal errors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountBudgetProposalErrorEnum {}
/// Nested message and enum types in `AccountBudgetProposalErrorEnum`.
pub mod account_budget_proposal_error_enum {
    /// Enum describing possible account budget proposal errors.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum AccountBudgetProposalError {
        /// Enum unspecified.
        Unspecified = 0,
        /// The received error code is not known in this version.
        Unknown = 1,
        /// The field mask must be empty for create/end/remove proposals.
        FieldMaskNotAllowed = 2,
        /// The field cannot be set because of the proposal type.
        ImmutableField = 3,
        /// The field is required because of the proposal type.
        RequiredFieldMissing = 4,
        /// Proposals that have been approved cannot be cancelled.
        CannotCancelApprovedProposal = 5,
        /// Budgets that haven't been approved cannot be removed.
        CannotRemoveUnapprovedBudget = 6,
        /// Budgets that are currently running cannot be removed.
        CannotRemoveRunningBudget = 7,
        /// Budgets that haven't been approved cannot be truncated.
        CannotEndUnapprovedBudget = 8,
        /// Only budgets that are currently running can be truncated.
        CannotEndInactiveBudget = 9,
        /// All budgets must have names.
        BudgetNameRequired = 10,
        /// Expired budgets cannot be edited after a sufficient amount of time has
        /// passed.
        CannotUpdateOldBudget = 11,
        /// It is not permissible a propose a new budget that ends in the past.
        CannotEndInPast = 12,
        /// An expired budget cannot be extended to overlap with the running budget.
        CannotExtendEndTime = 13,
        /// A purchase order number is required.
        PurchaseOrderNumberRequired = 14,
        /// Budgets that have a pending update cannot be updated.
        PendingUpdateProposalExists = 15,
        /// Cannot propose more than one budget when the corresponding billing setup
        /// hasn't been approved.
        MultipleBudgetsNotAllowedForUnapprovedBillingSetup = 16,
        /// Cannot update the start time of a budget that has already started.
        CannotUpdateStartTimeForStartedBudget = 17,
        /// Cannot update the spending limit of a budget with an amount lower than
        /// what has already been spent.
        SpendingLimitLowerThanAccruedCostNotAllowed = 18,
        /// Cannot propose a budget update without actually changing any fields.
        UpdateIsNoOp = 19,
        /// The end time must come after the start time.
        EndTimeMustFollowStartTime = 20,
        /// The budget's date range must fall within the date range of its billing
        /// setup.
        BudgetDateRangeIncompatibleWithBillingSetup = 21,
        /// The user is not authorized to mutate budgets for the given billing setup.
        NotAuthorized = 22,
        /// Mutates are not allowed for the given billing setup.
        InvalidBillingSetup = 23,
        /// Budget creation failed as it overlaps with an pending budget proposal
        /// or an approved budget.
        OverlapsExistingBudget = 24,
        /// The control setting in user's payments profile doesn't allow budget
        /// creation through API. Log in to Google Ads to create budget.
        CannotCreateBudgetThroughApi = 25,
    }
}
// Proto file describing AccountLink errors.

/// Container for enum describing possible account link errors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountLinkErrorEnum {}
/// Nested message and enum types in `AccountLinkErrorEnum`.
pub mod account_link_error_enum {
    /// Enum describing possible account link errors.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum AccountLinkError {
        /// Enum unspecified.
        Unspecified = 0,
        /// The received error code is not known in this version.
        Unknown = 1,
        /// The new link status is invalid.
        InvalidStatus = 2,
    }
}
// Proto file describing ad customizer errors.

/// Container for enum describing possible ad customizer errors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdCustomizerErrorEnum {}
/// Nested message and enum types in `AdCustomizerErrorEnum`.
pub mod ad_customizer_error_enum {
    /// Enum describing possible ad customizer errors.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum AdCustomizerError {
        /// Enum unspecified.
        Unspecified = 0,
        /// The received error code is not known in this version.
        Unknown = 1,
        /// Invalid date argument in countdown function.
        CountdownInvalidDateFormat = 2,
        /// Countdown end date is in the past.
        CountdownDateInPast = 3,
        /// Invalid locale string in countdown function.
        CountdownInvalidLocale = 4,
        /// Days-before argument to countdown function is not positive.
        CountdownInvalidStartDaysBefore = 5,
        /// A user list referenced in an IF function does not exist.
        UnknownUserList = 6,
    }
}
// Proto file describing ad errors.

/// Container for enum describing possible ad errors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdErrorEnum {}
/// Nested message and enum types in `AdErrorEnum`.
pub mod ad_error_enum {
    /// Enum describing possible ad errors.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum AdError {
        /// Enum unspecified.
        Unspecified = 0,
        /// The received error code is not known in this version.
        Unknown = 1,
        /// Ad customizers are not supported for ad type.
        AdCustomizersNotSupportedForAdType = 2,
        /// Estimating character sizes the string is too long.
        ApproximatelyTooLong = 3,
        /// Estimating character sizes the string is too short.
        ApproximatelyTooShort = 4,
        /// There is a problem with the snippet.
        BadSnippet = 5,
        /// Cannot modify an ad.
        CannotModifyAd = 6,
        /// business name and url cannot be set at the same time
        CannotSetBusinessNameIfUrlSet = 7,
        /// The specified field is incompatible with this ad's type or settings.
        CannotSetField = 8,
        /// Cannot set field when originAdId is set.
        CannotSetFieldWithOriginAdIdSet = 9,
        /// Cannot set field when an existing ad id is set for sharing.
        CannotSetFieldWithAdIdSetForSharing = 10,
        /// Cannot set allowFlexibleColor false if no color is provided by user.
        CannotSetAllowFlexibleColorFalse = 11,
        /// When user select native, no color control is allowed because we will
        /// always respect publisher color for native format serving.
        CannotSetColorControlWhenNativeFormatSetting = 12,
        /// Cannot specify a url for the ad type
        CannotSetUrl = 13,
        /// Cannot specify a tracking or mobile url without also setting final urls
        CannotSetWithoutFinalUrls = 14,
        /// Cannot specify a legacy url and a final url simultaneously
        CannotSetWithFinalUrls = 15,
        /// Cannot specify a urls in UrlData and in template fields simultaneously.
        CannotSetWithUrlData = 17,
        /// This operator cannot be used with a subclass of Ad.
        CannotUseAdSubclassForOperator = 18,
        /// Customer is not approved for mobile ads.
        CustomerNotApprovedMobileads = 19,
        /// Customer is not approved for 3PAS richmedia ads.
        CustomerNotApprovedThirdpartyAds = 20,
        /// Customer is not approved for 3PAS redirect richmedia (Ad Exchange) ads.
        CustomerNotApprovedThirdpartyRedirectAds = 21,
        /// Not an eligible customer
        CustomerNotEligible = 22,
        /// Customer is not eligible for updating beacon url
        CustomerNotEligibleForUpdatingBeaconUrl = 23,
        /// There already exists an ad with the same dimensions in the union.
        DimensionAlreadyInUnion = 24,
        /// Ad's dimension must be set before setting union dimension.
        DimensionMustBeSet = 25,
        /// Ad's dimension must be included in the union dimensions.
        DimensionNotInUnion = 26,
        /// Display Url cannot be specified (applies to Ad Exchange Ads)
        DisplayUrlCannotBeSpecified = 27,
        /// Telephone number contains invalid characters or invalid format. Please
        /// re-enter your number using digits (0-9), dashes (-), and parentheses
        /// only.
        DomesticPhoneNumberFormat = 28,
        /// Emergency telephone numbers are not allowed. Please enter a valid
        /// domestic phone number to connect customers to your business.
        EmergencyPhoneNumber = 29,
        /// A required field was not specified or is an empty string.
        EmptyField = 30,
        /// A feed attribute referenced in an ad customizer tag is not in the ad
        /// customizer mapping for the feed.
        FeedAttributeMustHaveMappingForTypeId = 31,
        /// The ad customizer field mapping for the feed attribute does not match the
        /// expected field type.
        FeedAttributeMappingTypeMismatch = 32,
        /// The use of ad customizer tags in the ad text is disallowed. Details in
        /// trigger.
        IllegalAdCustomizerTagUse = 33,
        /// Tags of the form {PH_x}, where x is a number, are disallowed in ad text.
        IllegalTagUse = 34,
        /// The dimensions of the ad are specified or derived in multiple ways and
        /// are not consistent.
        InconsistentDimensions = 35,
        /// The status cannot differ among template ads of the same union.
        InconsistentStatusInTemplateUnion = 36,
        /// The length of the string is not valid.
        IncorrectLength = 37,
        /// The ad is ineligible for upgrade.
        IneligibleForUpgrade = 38,
        /// User cannot create mobile ad for countries targeted in specified
        /// campaign.
        InvalidAdAddressCampaignTarget = 39,
        /// Invalid Ad type. A specific type of Ad is required.
        InvalidAdType = 40,
        /// Headline, description or phone cannot be present when creating mobile
        /// image ad.
        InvalidAttributesForMobileImage = 41,
        /// Image cannot be present when creating mobile text ad.
        InvalidAttributesForMobileText = 42,
        /// Invalid call to action text.
        InvalidCallToActionText = 43,
        /// Invalid character in URL.
        InvalidCharacterForUrl = 44,
        /// Creative's country code is not valid.
        InvalidCountryCode = 45,
        /// Invalid use of Expanded Dynamic Search Ads tags ({lpurl} etc.)
        InvalidExpandedDynamicSearchAdTag = 47,
        /// An input error whose real reason was not properly mapped (should not
        /// happen).
        InvalidInput = 48,
        /// An invalid markup language was entered.
        InvalidMarkupLanguage = 49,
        /// An invalid mobile carrier was entered.
        InvalidMobileCarrier = 50,
        /// Specified mobile carriers target a country not targeted by the campaign.
        InvalidMobileCarrierTarget = 51,
        /// Wrong number of elements for given element type
        InvalidNumberOfElements = 52,
        /// The format of the telephone number is incorrect. Please re-enter the
        /// number using the correct format.
        InvalidPhoneNumberFormat = 53,
        /// The certified vendor format id is incorrect.
        InvalidRichMediaCertifiedVendorFormatId = 54,
        /// The template ad data contains validation errors.
        InvalidTemplateData = 55,
        /// The template field doesn't have have the correct type.
        InvalidTemplateElementFieldType = 56,
        /// Invalid template id.
        InvalidTemplateId = 57,
        /// After substituting replacement strings, the line is too wide.
        LineTooWide = 58,
        /// The feed referenced must have ad customizer mapping to be used in a
        /// customizer tag.
        MissingAdCustomizerMapping = 59,
        /// Missing address component in template element address field.
        MissingAddressComponent = 60,
        /// An ad name must be entered.
        MissingAdvertisementName = 61,
        /// Business name must be entered.
        MissingBusinessName = 62,
        /// Description (line 2) must be entered.
        MissingDescription1 = 63,
        /// Description (line 3) must be entered.
        MissingDescription2 = 64,
        /// The destination url must contain at least one tag (e.g. {lpurl})
        MissingDestinationUrlTag = 65,
        /// The tracking url template of ExpandedDynamicSearchAd must contain at
        /// least one tag. (e.g. {lpurl})
        MissingLandingPageUrlTag = 66,
        /// A valid dimension must be specified for this ad.
        MissingDimension = 67,
        /// A display URL must be entered.
        MissingDisplayUrl = 68,
        /// Headline must be entered.
        MissingHeadline = 69,
        /// A height must be entered.
        MissingHeight = 70,
        /// An image must be entered.
        MissingImage = 71,
        /// Marketing image or product videos are required.
        MissingMarketingImageOrProductVideos = 72,
        /// The markup language in which your site is written must be entered.
        MissingMarkupLanguages = 73,
        /// A mobile carrier must be entered.
        MissingMobileCarrier = 74,
        /// Phone number must be entered.
        MissingPhone = 75,
        /// Missing required template fields
        MissingRequiredTemplateFields = 76,
        /// Missing a required field value
        MissingTemplateFieldValue = 77,
        /// The ad must have text.
        MissingText = 78,
        /// A visible URL must be entered.
        MissingVisibleUrl = 79,
        /// A width must be entered.
        MissingWidth = 80,
        /// Only 1 feed can be used as the source of ad customizer substitutions in a
        /// single ad.
        MultipleDistinctFeedsUnsupported = 81,
        /// TempAdUnionId must be use when adding template ads.
        MustUseTempAdUnionIdOnAdd = 82,
        /// The string has too many characters.
        TooLong = 83,
        /// The string has too few characters.
        TooShort = 84,
        /// Ad union dimensions cannot change for saved ads.
        UnionDimensionsCannotChange = 85,
        /// Address component is not {country, lat, lng}.
        UnknownAddressComponent = 86,
        /// Unknown unique field name
        UnknownFieldName = 87,
        /// Unknown unique name (template element type specifier)
        UnknownUniqueName = 88,
        /// Unsupported ad dimension
        UnsupportedDimensions = 89,
        /// URL starts with an invalid scheme.
        UrlInvalidScheme = 90,
        /// URL ends with an invalid top-level domain name.
        UrlInvalidTopLevelDomain = 91,
        /// URL contains illegal characters.
        UrlMalformed = 92,
        /// URL must contain a host name.
        UrlNoHost = 93,
        /// URL not equivalent during upgrade.
        UrlNotEquivalent = 94,
        /// URL host name too long to be stored as visible URL (applies to Ad
        /// Exchange ads)
        UrlHostNameTooLong = 95,
        /// URL must start with a scheme.
        UrlNoScheme = 96,
        /// URL should end in a valid domain extension, such as .com or .net.
        UrlNoTopLevelDomain = 97,
        /// URL must not end with a path.
        UrlPathNotAllowed = 98,
        /// URL must not specify a port.
        UrlPortNotAllowed = 99,
        /// URL must not contain a query.
        UrlQueryNotAllowed = 100,
        /// A url scheme is not allowed in front of tag in tracking url template
        /// (e.g. <http://{lpurl}>)
        UrlSchemeBeforeExpandedDynamicSearchAdTag = 102,
        /// The user does not have permissions to create a template ad for the given
        /// template.
        UserDoesNotHaveAccessToTemplate = 103,
        /// Expandable setting is inconsistent/wrong. For example, an AdX ad is
        /// invalid if it has a expandable vendor format but no expanding directions
        /// specified, or expanding directions is specified, but the vendor format is
        /// not expandable.
        InconsistentExpandableSettings = 104,
        /// Format is invalid
        InvalidFormat = 105,
        /// The text of this field did not match a pattern of allowed values.
        InvalidFieldText = 106,
        /// Template element is mising
        ElementNotPresent = 107,
        /// Error occurred during image processing
        ImageError = 108,
        /// The value is not within the valid range
        ValueNotInRange = 109,
        /// Template element field is not present
        FieldNotPresent = 110,
        /// Address is incomplete
        AddressNotComplete = 111,
        /// Invalid address
        AddressInvalid = 112,
        /// Error retrieving specified video
        VideoRetrievalError = 113,
        /// Error processing audio
        AudioError = 114,
        /// Display URL is incorrect for YouTube PYV ads
        InvalidYoutubeDisplayUrl = 115,
        /// Too many product Images in GmailAd
        TooManyProductImages = 116,
        /// Too many product Videos in GmailAd
        TooManyProductVideos = 117,
        /// The device preference is not compatible with the ad type
        IncompatibleAdTypeAndDevicePreference = 118,
        /// Call tracking is not supported for specified country.
        CalltrackingNotSupportedForCountry = 119,
        /// Carrier specific short number is not allowed.
        CarrierSpecificShortNumberNotAllowed = 120,
        /// Specified phone number type is disallowed.
        DisallowedNumberType = 121,
        /// Phone number not supported for country.
        PhoneNumberNotSupportedForCountry = 122,
        /// Phone number not supported with call tracking enabled for country.
        PhoneNumberNotSupportedWithCalltrackingForCountry = 123,
        /// Premium rate phone number is not allowed.
        PremiumRateNumberNotAllowed = 124,
        /// Vanity phone number is not allowed.
        VanityPhoneNumberNotAllowed = 125,
        /// Invalid call conversion type id.
        InvalidCallConversionTypeId = 126,
        /// Cannot disable call conversion and set conversion type id.
        CannotDisableCallConversionAndSetConversionTypeId = 127,
        /// Cannot set path2 without path1.
        CannotSetPath2WithoutPath1 = 128,
        /// Missing domain name in campaign setting when adding expanded dynamic
        /// search ad.
        MissingDynamicSearchAdsSettingDomainName = 129,
        /// The associated ad is not compatible with restriction type.
        IncompatibleWithRestrictionType = 130,
        /// Consent for call recording is required for creating/updating call only
        /// ads. Please see <https://support.google.com/google-ads/answer/7412639.>
        CustomerConsentForCallRecordingRequired = 131,
        /// Either an image or a media bundle is required in a display upload ad.
        MissingImageOrMediaBundle = 132,
        /// The display upload product type is not supported in this campaign.
        ProductTypeNotSupportedInThisCampaign = 133,
        /// The default value of an ad placeholder can not be the empty string.
        PlaceholderCannotHaveEmptyDefaultValue = 134,
        /// Ad placeholders with countdown functions must not have a default value.
        PlaceholderCountdownFunctionCannotHaveDefaultValue = 135,
        /// A previous ad placeholder that had a default value was found which means
        /// that all (non-countdown) placeholders must have a default value. This
        /// ad placeholder does not have a default value.
        PlaceholderDefaultValueMissing = 136,
        /// A previous ad placeholder that did not have a default value was found
        /// which means that no placeholders may have a default value. This
        /// ad placeholder does have a default value.
        UnexpectedPlaceholderDefaultValue = 137,
        /// Two ad customizers may not be directly adjacent in an ad text. They must
        /// be separated by at least one character.
        AdCustomizersMayNotBeAdjacent = 138,
        /// The ad is not associated with any enabled AdGroupAd, and cannot be
        /// updated.
        UpdatingAdWithNoEnabledAssociation = 139,
        /// Call Ad verification url and final url don't have same domain.
        CallAdVerificationUrlFinalUrlDoesNotHaveSameDomain = 140,
        /// Final url and verification url cannot both be empty for call ads.
        CallAdFinalUrlAndVerificationUrlCannotBothBeEmpty = 154,
        /// Too many ad customizers in one asset.
        TooManyAdCustomizers = 141,
        /// The ad customizer tag is recognized, but the format is invalid.
        InvalidAdCustomizerFormat = 142,
        /// Customizer tags cannot be nested.
        NestedAdCustomizerSyntax = 143,
        /// The ad customizer syntax used in the ad is not supported.
        UnsupportedAdCustomizerSyntax = 144,
        /// There exists unpaired brace in the ad customizer tag.
        UnpairedBraceInAdCustomizerTag = 145,
        /// More than one type of countdown tag exists among all text lines.
        MoreThanOneCountdownTagTypeExists = 146,
        /// Date time in the countdown tag is invalid.
        DateTimeInCountdownTagIsInvalid = 147,
        /// Date time in the countdown tag is in the past.
        DateTimeInCountdownTagIsPast = 148,
        /// Cannot recognize the ad customizer tag.
        UnrecognizedAdCustomizerTagFound = 149,
        /// Customizer type forbidden for this field.
        CustomizerTypeForbiddenForField = 150,
        /// Customizer attribute name is invalid.
        InvalidCustomizerAttributeName = 151,
        /// App store value does not match the value of the app store in the app
        /// specified in the campaign.
        StoreMismatch = 152,
        /// Missing required image aspect ratio.
        MissingRequiredImageAspectRatio = 153,
        /// Aspect ratios mismatch between different assets.
        MismatchedAspectRatios = 155,
        /// Images must be unique between different carousel card assets.
        DuplicateImageAcrossCarouselCards = 156,
    }
}
// Proto file describing ad group ad errors.

/// Container for enum describing possible ad group ad errors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdGroupAdErrorEnum {}
/// Nested message and enum types in `AdGroupAdErrorEnum`.
pub mod ad_group_ad_error_enum {
    /// Enum describing possible ad group ad errors.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum AdGroupAdError {
        /// Enum unspecified.
        Unspecified = 0,
        /// The received error code is not known in this version.
        Unknown = 1,
        /// No link found between the adgroup ad and the label.
        AdGroupAdLabelDoesNotExist = 2,
        /// The label has already been attached to the adgroup ad.
        AdGroupAdLabelAlreadyExists = 3,
        /// The specified ad was not found in the adgroup
        AdNotUnderAdgroup = 4,
        /// Removed ads may not be modified
        CannotOperateOnRemovedAdgroupad = 5,
        /// An ad of this type is deprecated and cannot be created. Only deletions
        /// are permitted.
        CannotCreateDeprecatedAds = 6,
        /// Text ads are deprecated and cannot be created. Use expanded text ads
        /// instead.
        CannotCreateTextAds = 7,
        /// A required field was not specified or is an empty string.
        EmptyField = 8,
        /// An ad may only be modified once per call
        ResourceReferencedInMultipleOps = 9,
        /// AdGroupAds with the given ad type cannot be paused.
        AdTypeCannotBePaused = 10,
        /// AdGroupAds with the given ad type cannot be removed.
        AdTypeCannotBeRemoved = 11,
        /// An ad of this type is deprecated and cannot be updated. Only removals
        /// are permitted.
        CannotUpdateDeprecatedAds = 12,
    }
}
// Proto file describing ad group bid modifier errors.

/// Container for enum describing possible ad group bid modifier errors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdGroupBidModifierErrorEnum {}
/// Nested message and enum types in `AdGroupBidModifierErrorEnum`.
pub mod ad_group_bid_modifier_error_enum {
    /// Enum describing possible ad group bid modifier errors.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum AdGroupBidModifierError {
        /// Enum unspecified.
        Unspecified = 0,
        /// The received error code is not known in this version.
        Unknown = 1,
        /// The criterion ID does not support bid modification.
        CriterionIdNotSupported = 2,
        /// Cannot override the bid modifier for the given criterion ID if the parent
        /// campaign is opted out of the same criterion.
        CannotOverrideOptedOutCampaignCriterionBidModifier = 3,
    }
}
// Proto file describing ad group criterion customizer errors.

/// Container for enum describing possible ad group criterion customizer errors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdGroupCriterionCustomizerErrorEnum {}
/// Nested message and enum types in `AdGroupCriterionCustomizerErrorEnum`.
pub mod ad_group_criterion_customizer_error_enum {
    /// Enum describing possible ad group criterion customizer errors.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum AdGroupCriterionCustomizerError {
        /// Enum unspecified.
        Unspecified = 0,
        /// The received error code is not known in this version.
        Unknown = 1,
        /// Only keyword type criterion is allowed to link customizer attribute.
        CriterionIsNotKeyword = 2,
    }
}
// Proto file describing ad group criterion errors.

/// Container for enum describing possible ad group criterion errors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdGroupCriterionErrorEnum {}
/// Nested message and enum types in `AdGroupCriterionErrorEnum`.
pub mod ad_group_criterion_error_enum {
    /// Enum describing possible ad group criterion errors.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum AdGroupCriterionError {
        /// Enum unspecified.
        Unspecified = 0,
        /// The received error code is not known in this version.
        Unknown = 1,
        /// No link found between the AdGroupCriterion and the label.
        AdGroupCriterionLabelDoesNotExist = 2,
        /// The label has already been attached to the AdGroupCriterion.
        AdGroupCriterionLabelAlreadyExists = 3,
        /// Negative AdGroupCriterion cannot have labels.
        CannotAddLabelToNegativeCriterion = 4,
        /// Too many operations for a single call.
        TooManyOperations = 5,
        /// Negative ad group criteria are not updateable.
        CantUpdateNegative = 6,
        /// Concrete type of criterion (keyword v.s. placement) is required for ADD
        /// and SET operations.
        ConcreteTypeRequired = 7,
        /// Bid is incompatible with ad group's bidding settings.
        BidIncompatibleWithAdgroup = 8,
        /// Cannot target and exclude the same criterion at once.
        CannotTargetAndExclude = 9,
        /// The URL of a placement is invalid.
        IllegalUrl = 10,
        /// Keyword text was invalid.
        InvalidKeywordText = 11,
        /// Destination URL was invalid.
        InvalidDestinationUrl = 12,
        /// The destination url must contain at least one tag (e.g. {lpurl})
        MissingDestinationUrlTag = 13,
        /// Keyword-level cpm bid is not supported
        KeywordLevelBidNotSupportedForManualcpm = 14,
        /// For example, cannot add a biddable ad group criterion that had been
        /// removed.
        InvalidUserStatus = 15,
        /// Criteria type cannot be targeted for the ad group. Either the account is
        /// restricted to keywords only, the criteria type is incompatible with the
        /// campaign's bidding strategy, or the criteria type can only be applied to
        /// campaigns.
        CannotAddCriteriaType = 16,
        /// Criteria type cannot be excluded for the ad group. Refer to the
        /// documentation for a specific criterion to check if it is excludable.
        CannotExcludeCriteriaType = 17,
        /// Partial failure is not supported for shopping campaign mutate operations.
        CampaignTypeNotCompatibleWithPartialFailure = 27,
        /// Operations in the mutate request changes too many shopping ad groups.
        /// Please split requests for multiple shopping ad groups across multiple
        /// requests.
        OperationsForTooManyShoppingAdgroups = 28,
        /// Not allowed to modify url fields of an ad group criterion if there are
        /// duplicate elements for that ad group criterion in the request.
        CannotModifyUrlFieldsWithDuplicateElements = 29,
        /// Cannot set url fields without also setting final urls.
        CannotSetWithoutFinalUrls = 30,
        /// Cannot clear final urls if final mobile urls exist.
        CannotClearFinalUrlsIfFinalMobileUrlsExist = 31,
        /// Cannot clear final urls if final app urls exist.
        CannotClearFinalUrlsIfFinalAppUrlsExist = 32,
        /// Cannot clear final urls if tracking url template exists.
        CannotClearFinalUrlsIfTrackingUrlTemplateExists = 33,
        /// Cannot clear final urls if url custom parameters exist.
        CannotClearFinalUrlsIfUrlCustomParametersExist = 34,
        /// Cannot set both destination url and final urls.
        CannotSetBothDestinationUrlAndFinalUrls = 35,
        /// Cannot set both destination url and tracking url template.
        CannotSetBothDestinationUrlAndTrackingUrlTemplate = 36,
        /// Final urls are not supported for this criterion type.
        FinalUrlsNotSupportedForCriterionType = 37,
        /// Final mobile urls are not supported for this criterion type.
        FinalMobileUrlsNotSupportedForCriterionType = 38,
    }
}
// Proto file describing ad group customizer errors.

/// Container for enum describing possible ad group customizer errors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdGroupCustomizerErrorEnum {}
/// Nested message and enum types in `AdGroupCustomizerErrorEnum`.
pub mod ad_group_customizer_error_enum {
    /// Enum describing possible ad group customizer errors.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum AdGroupCustomizerError {
        /// Enum unspecified.
        Unspecified = 0,
        /// The received error code is not known in this version.
        Unknown = 1,
    }
}
// Proto file describing ad group errors.

/// Container for enum describing possible ad group errors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdGroupErrorEnum {}
/// Nested message and enum types in `AdGroupErrorEnum`.
pub mod ad_group_error_enum {
    /// Enum describing possible ad group errors.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum AdGroupError {
        /// Enum unspecified.
        Unspecified = 0,
        /// The received error code is not known in this version.
        Unknown = 1,
        /// AdGroup with the same name already exists for the campaign.
        DuplicateAdgroupName = 2,
        /// AdGroup name is not valid.
        InvalidAdgroupName = 3,
        /// Advertiser is not allowed to target sites or set site bids that are not
        /// on the Google Search Network.
        AdvertiserNotOnContentNetwork = 5,
        /// Bid amount is too big.
        BidTooBig = 6,
        /// AdGroup bid does not match the campaign's bidding strategy.
        BidTypeAndBiddingStrategyMismatch = 7,
        /// AdGroup name is required for Add.
        MissingAdgroupName = 8,
        /// No link found between the ad group and the label.
        AdgroupLabelDoesNotExist = 9,
        /// The label has already been attached to the ad group.
        AdgroupLabelAlreadyExists = 10,
        /// The CriterionTypeGroup is not supported for the content bid dimension.
        InvalidContentBidCriterionTypeGroup = 11,
        /// The ad group type is not compatible with the campaign channel type.
        AdGroupTypeNotValidForAdvertisingChannelType = 12,
        /// The ad group type is not supported in the country of sale of the
        /// campaign.
        AdgroupTypeNotSupportedForCampaignSalesCountry = 13,
        /// Ad groups of AdGroupType.SEARCH_DYNAMIC_ADS can only be added to
        /// campaigns that have DynamicSearchAdsSetting attached.
        CannotAddAdgroupOfTypeDsaToCampaignWithoutDsaSetting = 14,
        /// Promoted hotels ad groups are only available to customers on the
        /// allow-list.
        PromotedHotelAdGroupsNotAvailableForCustomer = 15,
        /// The field type cannot be excluded because an active ad group-asset link
        /// of this type exists.
        InvalidExcludedParentAssetFieldType = 16,
    }
}
// Proto file describing ad group feed errors.

/// Container for enum describing possible ad group feed errors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdGroupFeedErrorEnum {}
/// Nested message and enum types in `AdGroupFeedErrorEnum`.
pub mod ad_group_feed_error_enum {
    /// Enum describing possible ad group feed errors.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum AdGroupFeedError {
        /// Enum unspecified.
        Unspecified = 0,
        /// The received error code is not known in this version.
        Unknown = 1,
        /// An active feed already exists for this ad group and place holder type.
        FeedAlreadyExistsForPlaceholderType = 2,
        /// The specified feed is removed.
        CannotCreateForRemovedFeed = 3,
        /// The AdGroupFeed already exists. UPDATE operation should be used to modify
        /// the existing AdGroupFeed.
        AdgroupFeedAlreadyExists = 4,
        /// Cannot operate on removed AdGroupFeed.
        CannotOperateOnRemovedAdgroupFeed = 5,
        /// Invalid placeholder type.
        InvalidPlaceholderType = 6,
        /// Feed mapping for this placeholder type does not exist.
        MissingFeedmappingForPlaceholderType = 7,
        /// Location AdGroupFeeds cannot be created unless there is a location
        /// CustomerFeed for the specified feed.
        NoExistingLocationCustomerFeed = 8,
    }
}
// Proto file describing ad parameter errors.

/// Container for enum describing possible ad parameter errors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdParameterErrorEnum {}
/// Nested message and enum types in `AdParameterErrorEnum`.
pub mod ad_parameter_error_enum {
    /// Enum describing possible ad parameter errors.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum AdParameterError {
        /// Enum unspecified.
        Unspecified = 0,
        /// The received error code is not known in this version.
        Unknown = 1,
        /// The ad group criterion must be a keyword criterion.
        AdGroupCriterionMustBeKeyword = 2,
        /// The insertion text is invalid.
        InvalidInsertionTextFormat = 3,
    }
}
// Proto file describing ad sharing errors.

/// Container for enum describing possible ad sharing errors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdSharingErrorEnum {}
/// Nested message and enum types in `AdSharingErrorEnum`.
pub mod ad_sharing_error_enum {
    /// Enum describing possible ad sharing errors.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum AdSharingError {
        /// Enum unspecified.
        Unspecified = 0,
        /// The received error code is not known in this version.
        Unknown = 1,
        /// Error resulting in attempting to add an Ad to an AdGroup that already
        /// contains the Ad.
        AdGroupAlreadyContainsAd = 2,
        /// Ad is not compatible with the AdGroup it is being shared with.
        IncompatibleAdUnderAdGroup = 3,
        /// Cannot add AdGroupAd on inactive Ad.
        CannotShareInactiveAd = 4,
    }
}
// Proto file describing adx errors.

/// Container for enum describing possible adx errors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdxErrorEnum {}
/// Nested message and enum types in `AdxErrorEnum`.
pub mod adx_error_enum {
    /// Enum describing possible adx errors.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum AdxError {
        /// Enum unspecified.
        Unspecified = 0,
        /// The received error code is not known in this version.
        Unknown = 1,
        /// Attempt to use non-AdX feature by AdX customer.
        UnsupportedFeature = 2,
    }
}
// Proto file describing asset errors.

/// Container for enum describing possible asset errors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetErrorEnum {}
/// Nested message and enum types in `AssetErrorEnum`.
pub mod asset_error_enum {
    /// Enum describing possible asset errors.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum AssetError {
        /// Enum unspecified.
        Unspecified = 0,
        /// The received error code is not known in this version.
        Unknown = 1,
        /// The customer is not is not on the allow-list for this asset type.
        CustomerNotOnAllowlistForAssetType = 13,
        /// Assets are duplicated across operations.
        DuplicateAsset = 3,
        /// The asset name is duplicated, either across operations or with an
        /// existing asset.
        DuplicateAssetName = 4,
        /// The Asset.asset_data oneof is empty.
        AssetDataIsMissing = 5,
        /// The asset has a name which is different from an existing duplicate that
        /// represents the same content.
        CannotModifyAssetName = 6,
        /// The field cannot be set for this asset type.
        FieldIncompatibleWithAssetType = 7,
        /// Call to action must come from the list of supported values.
        InvalidCallToActionText = 8,
        /// A lead form asset is created with an invalid combination of input fields.
        LeadFormInvalidFieldsCombination = 9,
        /// Lead forms require that the Terms of Service have been agreed to before
        /// mutates can be executed.
        LeadFormMissingAgreement = 10,
        /// Asset status is invalid in this operation.
        InvalidAssetStatus = 11,
        /// The field cannot be modified by this asset type.
        FieldCannotBeModifiedForAssetType = 12,
        /// Ad schedules for the same asset cannot overlap.
        SchedulesCannotOverlap = 14,
        /// Cannot set both percent off and money amount off fields of promotion
        /// asset.
        PromotionCannotSetPercentOffAndMoneyAmountOff = 15,
        /// Cannot set both promotion code and orders over amount fields of promotion
        /// asset.
        PromotionCannotSetPromotionCodeAndOrdersOverAmount = 16,
        /// The field has too many decimal places specified.
        TooManyDecimalPlacesSpecified = 17,
        /// Duplicate assets across operations, which have identical Asset.asset_data
        /// oneof, cannot have different asset level fields for asset types which are
        /// deduped.
        DuplicateAssetsWithDifferentFieldValue = 18,
        /// Carrier specific short number is not allowed.
        CallCarrierSpecificShortNumberNotAllowed = 19,
        /// Customer consent required for call recording Terms of Service.
        CallCustomerConsentForCallRecordingRequired = 20,
        /// The type of the specified phone number is not allowed.
        CallDisallowedNumberType = 21,
        /// If the default call_conversion_action is not used, the customer must have
        /// a ConversionAction with the same id and the ConversionAction must be call
        /// conversion type.
        CallInvalidConversionAction = 22,
        /// The country code of the phone number is invalid.
        CallInvalidCountryCode = 23,
        /// The format of the phone number is incorrect.
        CallInvalidDomesticPhoneNumberFormat = 24,
        /// The input phone number is not a valid phone number.
        CallInvalidPhoneNumber = 25,
        /// The phone number is not supported for country.
        CallPhoneNumberNotSupportedForCountry = 26,
        /// Premium rate phone number is not allowed.
        CallPremiumRateNumberNotAllowed = 27,
        /// Vanity phone number is not allowed.
        CallVanityPhoneNumberNotAllowed = 28,
        /// PriceOffering cannot have the same value for header and description.
        PriceHeaderSameAsDescription = 29,
        /// AppId is invalid.
        MobileAppInvalidAppId = 30,
        /// Invalid App download URL in final URLs.
        MobileAppInvalidFinalUrlForAppDownloadUrl = 31,
        /// Asset name is required for the asset type.
        NameRequiredForAssetType = 32,
        /// Legacy qualifying questions cannot be in the same Lead Form as
        /// custom questions.
        LeadFormLegacyQualifyingQuestionsDisallowed = 33,
        /// Unique name is required for this asset type.
        NameConflictForAssetType = 34,
    }
}
// Proto file describing asset group asset errors.

/// Container for enum describing possible asset group asset errors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetGroupAssetErrorEnum {}
/// Nested message and enum types in `AssetGroupAssetErrorEnum`.
pub mod asset_group_asset_error_enum {
    /// Enum describing possible asset group asset errors.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum AssetGroupAssetError {
        /// Enum unspecified.
        Unspecified = 0,
        /// The received error code is not known in this version.
        Unknown = 1,
        /// Cannot add duplicated asset group asset.
        DuplicateResource = 2,
        /// Expandable tags are not allowed in description assets.
        ExpandableTagsNotAllowedInDescription = 3,
        /// Ad customizers are not supported in assetgroup's text assets.
        AdCustomizerNotSupported = 4,
    }
}
// Proto file describing asset group errors.

/// Container for enum describing possible asset group errors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetGroupErrorEnum {}
/// Nested message and enum types in `AssetGroupErrorEnum`.
pub mod asset_group_error_enum {
    /// Enum describing possible asset group errors.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum AssetGroupError {
        /// Enum unspecified.
        Unspecified = 0,
        /// The received error code is not known in this version.
        Unknown = 1,
        /// Each asset group in a single campaign must have a unique name.
        DuplicateName = 2,
        /// Cannot add asset group for the campaign type.
        CannotAddAssetGroupForCampaignType = 3,
        /// Not enough headline asset for a valid asset group.
        NotEnoughHeadlineAsset = 4,
        /// Not enough long headline asset for a valid asset group.
        NotEnoughLongHeadlineAsset = 5,
        /// Not enough description headline asset for a valid asset group.
        NotEnoughDescriptionAsset = 6,
        /// Not enough business name asset for a valid asset group.
        NotEnoughBusinessNameAsset = 7,
        /// Not enough marketing image asset for a valid asset group.
        NotEnoughMarketingImageAsset = 8,
        /// Not enough square marketing image asset for a valid asset group.
        NotEnoughSquareMarketingImageAsset = 9,
        /// Not enough logo asset for a valid asset group.
        NotEnoughLogoAsset = 10,
        /// Final url and shopping merchant url does not have the same domain.
        FinalUrlShoppingMerchantHomePageUrlDomainsDiffer = 11,
    }
}
// Proto file describing asset group asset errors.

/// Container for enum describing possible asset group listing group filter
/// errors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetGroupListingGroupFilterErrorEnum {}
/// Nested message and enum types in `AssetGroupListingGroupFilterErrorEnum`.
pub mod asset_group_listing_group_filter_error_enum {
    /// Enum describing possible asset group listing group filter errors.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum AssetGroupListingGroupFilterError {
        /// Enum unspecified.
        Unspecified = 0,
        /// The received error code is not known in this version.
        Unknown = 1,
        /// Listing group tree is too deep.
        TreeTooDeep = 2,
        /// Listing Group UNIT node cannot have children.
        UnitCannotHaveChildren = 3,
        /// Listing Group SUBDIVISION node must have everything else child.
        SubdivisionMustHaveEverythingElseChild = 4,
        /// Dimension type of Listing Group must be the same as that of its siblings.
        DifferentDimensionTypeBetweenSiblings = 5,
        /// The sibling Listing Groups target exactly the same dimension value.
        SameDimensionValueBetweenSiblings = 6,
        /// The dimension type is the same as one of the ancestor Listing Groups.
        SameDimensionTypeBetweenAncestors = 7,
        /// Each Listing Group tree must have a single root.
        MultipleRoots = 8,
        /// Invalid Listing Group dimension value.
        InvalidDimensionValue = 9,
        /// Hierarchical dimension must refine a dimension of the same type.
        MustRefineHierarchicalParentType = 10,
        /// Invalid Product Bidding Category.
        InvalidProductBiddingCategory = 11,
        /// Modifying case value is allowed only while updating the entire subtree at
        /// the same time.
        ChangingCaseValueWithChildren = 12,
        /// Subdivision node has children which must be removed first.
        SubdivisionHasChildren = 13,
        /// Dimension can't subdivide everything-else node in its own hierarchy.
        CannotRefineHierarchicalEverythingElse = 14,
    }
}
// Proto file describing asset link errors.

/// Container for enum describing possible asset link errors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetLinkErrorEnum {}
/// Nested message and enum types in `AssetLinkErrorEnum`.
pub mod asset_link_error_enum {
    /// Enum describing possible asset link errors.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum AssetLinkError {
        /// Enum unspecified.
        Unspecified = 0,
        /// The received error code is not known in this version.
        Unknown = 1,
        /// Pinning is not supported for the given asset link field.
        PinningUnsupported = 2,
        /// The given field type is not supported to be added directly via asset
        /// links.
        UnsupportedFieldType = 3,
        /// The given asset's type and the specified field type are incompatible.
        FieldTypeIncompatibleWithAssetType = 4,
        /// The specified field type is incompatible with the given campaign type.
        FieldTypeIncompatibleWithCampaignType = 5,
        /// The campaign advertising channel type cannot be associated with the given
        /// asset due to channel-based restrictions on the asset's fields.
        IncompatibleAdvertisingChannelType = 6,
        /// The image asset provided is not within the dimension constraints
        /// specified for the submitted asset field.
        ImageNotWithinSpecifiedDimensionRange = 7,
        /// The pinned field is not valid for the submitted asset field.
        InvalidPinnedField = 8,
        /// The media bundle asset provided is too large for the submitted asset
        /// field.
        MediaBundleAssetFileSizeTooLarge = 9,
        /// Not enough assets are available for use with other fields since other
        /// assets are pinned to specific fields.
        NotEnoughAvailableAssetLinksForValidCombination = 10,
        /// Not enough assets with fallback are available. When validating the
        /// minimum number of assets, assets without fallback (e.g. assets that
        /// contain location tag without default value "{LOCATION(City)}") will not
        /// be counted.
        NotEnoughAvailableAssetLinksWithFallback = 11,
        /// This is a combination of the
        /// NOT_ENOUGH_AVAILABLE_ASSET_LINKS_FOR_VALID_COMBINATION and
        /// NOT_ENOUGH_AVAILABLE_ASSET_LINKS_WITH_FALLBACK errors. Not enough assets
        /// with fallback are available since some assets are pinned.
        NotEnoughAvailableAssetLinksWithFallbackForValidCombination = 12,
        /// The YouTube video referenced in the provided asset has been removed.
        YoutubeVideoRemoved = 13,
        /// The YouTube video referenced in the provided asset is too long for the
        /// field submitted.
        YoutubeVideoTooLong = 14,
        /// The YouTube video referenced in the provided asset is too short for the
        /// field submitted.
        YoutubeVideoTooShort = 15,
        /// The status is invalid for the operation specified.
        InvalidStatus = 17,
        /// The YouTube video referenced in the provided asset has unknown duration.
        /// This might be the case for a livestream video or a video being currently
        /// uploaded to YouTube. In both cases, the video duration should eventually
        /// get resolved.
        YoutubeVideoDurationNotDefined = 18,
    }
}
// Proto file describing asset set asset errors.

/// Container for enum describing possible asset set asset errors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetSetAssetErrorEnum {}
/// Nested message and enum types in `AssetSetAssetErrorEnum`.
pub mod asset_set_asset_error_enum {
    /// Enum describing possible asset set asset errors.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum AssetSetAssetError {
        /// Enum unspecified.
        Unspecified = 0,
        /// The received error code is not known in this version.
        Unknown = 1,
        /// The asset type is not eligible to be linked to the specific type of asset
        /// set.
        InvalidAssetType = 2,
        /// The asset set type is not eligible to contain the specified type of
        /// assets.
        InvalidAssetSetType = 3,
        /// The asset contains duplicate external key with another asset in the asset
        /// set.
        DuplicateExternalKey = 4,
    }
}
// Proto file describing asset set errors.

/// Container for enum describing possible asset set errors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetSetErrorEnum {}
/// Nested message and enum types in `AssetSetErrorEnum`.
pub mod asset_set_error_enum {
    /// Enum describing possible asset set errors.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum AssetSetError {
        /// Enum unspecified.
        Unspecified = 0,
        /// The received error code is not known in this version.
        Unknown = 1,
        /// The asset set name matches that of another enabled asset set.
        DuplicateAssetSetName = 2,
    }
}
// Proto file describing asset set link errors.

/// Container for enum describing possible asset set link errors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetSetLinkErrorEnum {}
/// Nested message and enum types in `AssetSetLinkErrorEnum`.
pub mod asset_set_link_error_enum {
    /// Enum describing possible asset set link errors.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum AssetSetLinkError {
        /// Enum unspecified.
        Unspecified = 0,
        /// The received error code is not known in this version.
        Unknown = 1,
        /// Advertising channel type cannot be attached to the asset set due to
        /// channel-based restrictions.
        IncompatibleAdvertisingChannelType = 2,
        /// For this asset set type, only one campaign to feed linkage is allowed.
        DuplicateFeedLink = 3,
        /// The asset set type and campaign type are incompatible.
        IncompatibleAssetSetTypeWithCampaignType = 4,
        /// Cannot link duplicate asset sets to the same campaign.
        DuplicateAssetSetLink = 5,
        /// Cannot remove the asset set link. If a campaign is linked with only one
        /// asset set and you attempt to unlink them, this error will be triggered.
        AssetSetLinkCannotBeRemoved = 6,
    }
}
// Proto file describing audience errors.

/// Container for enum describing possible audience errors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AudienceErrorEnum {}
/// Nested message and enum types in `AudienceErrorEnum`.
pub mod audience_error_enum {
    /// Enum describing possible audience errors.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum AudienceError {
        /// Enum unspecified.
        Unspecified = 0,
        /// The received error code is not known in this version.
        Unknown = 1,
        /// An audience with this name already exists.
        NameAlreadyInUse = 2,
        /// A dimension within the audience definition is not valid.
        DimensionInvalid = 3,
        /// One of the audience segment added is not found.
        AudienceSegmentNotFound = 4,
        /// One of the audience segment type is not supported.
        AudienceSegmentTypeNotSupported = 5,
        /// The same segment already exists in this audience.
        DuplicateAudienceSegment = 6,
        /// Audience can't have more than allowed number segments.
        TooManySegments = 7,
        /// Audience can't have multiple dimensions of same type.
        TooManyDimensionsOfSameType = 8,
        /// The audience cannot be removed, because it is currently used in an
        /// ad group criterion or asset group signal in an (enabled or paused)
        /// ad group or campaign.
        InUse = 9,
    }
}
// Proto file describing authentication errors.

/// Container for enum describing possible authentication errors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthenticationErrorEnum {}
/// Nested message and enum types in `AuthenticationErrorEnum`.
pub mod authentication_error_enum {
    /// Enum describing possible authentication errors.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum AuthenticationError {
        /// Enum unspecified.
        Unspecified = 0,
        /// The received error code is not known in this version.
        Unknown = 1,
        /// Authentication of the request failed.
        AuthenticationError = 2,
        /// Client Customer ID is not a number.
        ClientCustomerIdInvalid = 5,
        /// No customer found for the provided customer ID.
        CustomerNotFound = 8,
        /// Client's Google Account is deleted.
        GoogleAccountDeleted = 9,
        /// Google account login token in the cookie is invalid.
        GoogleAccountCookieInvalid = 10,
        /// A problem occurred during Google account authentication.
        GoogleAccountAuthenticationFailed = 25,
        /// The user in the Google account login token does not match the user ID in
        /// the cookie.
        GoogleAccountUserAndAdsUserMismatch = 12,
        /// Login cookie is required for authentication.
        LoginCookieRequired = 13,
        /// User in the cookie is not a valid Ads user.
        NotAdsUser = 14,
        /// Oauth token in the header is not valid.
        OauthTokenInvalid = 15,
        /// Oauth token in the header has expired.
        OauthTokenExpired = 16,
        /// Oauth token in the header has been disabled.
        OauthTokenDisabled = 17,
        /// Oauth token in the header has been revoked.
        OauthTokenRevoked = 18,
        /// Oauth token HTTP header is malformed.
        OauthTokenHeaderInvalid = 19,
        /// Login cookie is not valid.
        LoginCookieInvalid = 20,
        /// User Id in the header is not a valid id.
        UserIdInvalid = 22,
        /// An account administrator changed this account's authentication settings.
        /// To access this Google Ads account, enable 2-Step Verification in your
        /// Google account at <https://www.google.com/landing/2step.>
        TwoStepVerificationNotEnrolled = 23,
        /// An account administrator changed this account's authentication settings.
        /// To access this Google Ads account, enable Advanced Protection in your
        /// Google account at <https://landing.google.com/advancedprotection.>
        AdvancedProtectionNotEnrolled = 24,
    }
}
// Proto file describing authorization errors.

/// Container for enum describing possible authorization errors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthorizationErrorEnum {}
/// Nested message and enum types in `AuthorizationErrorEnum`.
pub mod authorization_error_enum {
    /// Enum describing possible authorization errors.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum AuthorizationError {
        /// Enum unspecified.
        Unspecified = 0,
        /// The received error code is not known in this version.
        Unknown = 1,
        /// User doesn't have permission to access customer. Note: If you're
        /// accessing a client customer, the manager's customer ID must be set in the
        /// `login-customer-id` header. Learn more at
        /// <https://developers.google.com/google-ads/api/docs/concepts/call-structure#cid>
        UserPermissionDenied = 2,
        /// The developer token is not on the allow-list.
        DeveloperTokenNotOnAllowlist = 13,
        /// The developer token is not allowed with the project sent in the request.
        DeveloperTokenProhibited = 4,
        /// The Google Cloud project sent in the request does not have permission to
        /// access the api.
        ProjectDisabled = 5,
        /// Authorization of the client failed.
        AuthorizationError = 6,
        /// The user does not have permission to perform this action
        /// (e.g., ADD, UPDATE, REMOVE) on the resource or call a method.
        ActionNotPermitted = 7,
        /// Signup not complete.
        IncompleteSignup = 8,
        /// The customer can't be used because it isn't enabled.
        CustomerNotEnabled = 24,
        /// The developer must sign the terms of service. They can be found here:
        /// ads.google.com/aw/apicenter
        MissingTos = 9,
        /// The developer token is not approved. Non-approved developer tokens can
        /// only be used with test accounts.
        DeveloperTokenNotApproved = 10,
        /// The login customer specified does not have access to the account
        /// specified, so the request is invalid.
        InvalidLoginCustomerIdServingCustomerIdCombination = 11,
        /// The developer specified does not have access to the service.
        ServiceAccessDenied = 12,
        /// The customer (or login customer) isn't in Google Ads. It belongs to
        /// another ads system.
        AccessDeniedForAccountType = 25,
        /// The developer does not have access to the metrics queried.
        MetricAccessDenied = 26,
    }
}
// Proto file describing batch job errors.

/// Container for enum describing possible batch job errors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchJobErrorEnum {}
/// Nested message and enum types in `BatchJobErrorEnum`.
pub mod batch_job_error_enum {
    /// Enum describing possible request errors.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum BatchJobError {
        /// Enum unspecified.
        Unspecified = 0,
        /// The received error code is not known in this version.
        Unknown = 1,
        /// The batch job cannot add more operations or run after it has started
        /// running.
        CannotModifyJobAfterJobStartsRunning = 2,
        /// The operations for an AddBatchJobOperations request were empty.
        EmptyOperations = 3,
        /// The sequence token for an AddBatchJobOperations request was invalid.
        InvalidSequenceToken = 4,
        /// Batch job results can only be retrieved once the job is finished.
        ResultsNotReady = 5,
        /// The page size for ListBatchJobResults was invalid.
        InvalidPageSize = 6,
        /// The batch job cannot be removed because it has started running.
        CanOnlyRemovePendingJob = 7,
    }
}
// Proto file describing bidding errors.

/// Container for enum describing possible bidding errors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BiddingErrorEnum {}
/// Nested message and enum types in `BiddingErrorEnum`.
pub mod bidding_error_enum {
    /// Enum describing possible bidding errors.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum BiddingError {
        /// Enum unspecified.
        Unspecified = 0,
        /// The received error code is not known in this version.
        Unknown = 1,
        /// Cannot transition to new bidding strategy.
        BiddingStrategyTransitionNotAllowed = 2,
        /// Cannot attach bidding strategy to campaign.
        CannotAttachBiddingStrategyToCampaign = 7,
        /// Bidding strategy is not supported or cannot be used as anonymous.
        InvalidAnonymousBiddingStrategyType = 10,
        /// The type does not match the named strategy's type.
        InvalidBiddingStrategyType = 14,
        /// The bid is invalid.
        InvalidBid = 17,
        /// Bidding strategy is not available for the account type.
        BiddingStrategyNotAvailableForAccountType = 18,
        /// Conversion tracking is not enabled in the campaign that has value-based
        /// bidding transitions.
        ConversionTrackingNotEnabled = 19,
        /// Not enough conversions tracked for value-based bidding transitions.
        NotEnoughConversions = 20,
        /// Campaign can not be created with given bidding strategy. It can be
        /// transitioned to the strategy, once eligible.
        CannotCreateCampaignWithBiddingStrategy = 21,
        /// Cannot target content network only as campaign uses Page One Promoted
        /// bidding strategy.
        CannotTargetContentNetworkOnlyWithCampaignLevelPopBiddingStrategy = 23,
        /// Budget Optimizer and Target Spend bidding strategies are not supported
        /// for campaigns with AdSchedule targeting.
        BiddingStrategyNotSupportedWithAdSchedule = 24,
        /// Pay per conversion is not available to all the customer, only few
        /// customers on the allow-list can use this.
        PayPerConversionNotAvailableForCustomer = 25,
        /// Pay per conversion is not allowed with Target CPA.
        PayPerConversionNotAllowedWithTargetCpa = 26,
        /// Cannot set bidding strategy to Manual CPM for search network only
        /// campaigns.
        BiddingStrategyNotAllowedForSearchOnlyCampaigns = 27,
        /// The bidding strategy is not supported for use in drafts or experiments.
        BiddingStrategyNotSupportedInDraftsOrExperiments = 28,
        /// Bidding strategy type does not support product type ad group criterion.
        BiddingStrategyTypeDoesNotSupportProductTypeAdgroupCriterion = 29,
        /// Bid amount is too small.
        BidTooSmall = 30,
        /// Bid amount is too big.
        BidTooBig = 31,
        /// Bid has too many fractional digit precision.
        BidTooManyFractionalDigits = 32,
        /// Invalid domain name specified.
        InvalidDomainName = 33,
        /// The field is not compatible with the payment mode.
        NotCompatibleWithPaymentMode = 34,
        /// The field is not compatible with the budget type.
        NotCompatibleWithBudgetType = 35,
        /// The field is not compatible with the bidding strategy type.
        NotCompatibleWithBiddingStrategyType = 36,
        /// Bidding strategy type is incompatible with shared budget.
        BiddingStrategyTypeIncompatibleWithSharedBudget = 37,
        /// The attached bidding strategy and budget must be aligned with each other
        /// if alignment is specified on either entity.
        BiddingStrategyAndBudgetMustBeAligned = 38,
        /// The attached bidding strategy and budget must be attached to the same
        /// campaigns to become aligned.
        BiddingStrategyAndBudgetMustBeAttachedToTheSameCampaignsToAlign = 39,
        /// The aligned bidding strategy and budget must be removed at the same time.
        BiddingStrategyAndBudgetMustBeRemovedTogether = 40,
    }
}
// Proto file describing bidding strategy errors.

/// Container for enum describing possible bidding strategy errors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BiddingStrategyErrorEnum {}
/// Nested message and enum types in `BiddingStrategyErrorEnum`.
pub mod bidding_strategy_error_enum {
    /// Enum describing possible bidding strategy errors.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum BiddingStrategyError {
        /// Enum unspecified.
        Unspecified = 0,
        /// The received error code is not known in this version.
        Unknown = 1,
        /// Each bidding strategy must have a unique name.
        DuplicateName = 2,
        /// Bidding strategy type is immutable.
        CannotChangeBiddingStrategyType = 3,
        /// Only bidding strategies not linked to campaigns, adgroups or adgroup
        /// criteria can be removed.
        CannotRemoveAssociatedStrategy = 4,
        /// The specified bidding strategy is not supported.
        BiddingStrategyNotSupported = 5,
        /// The bidding strategy is incompatible with the campaign's bidding
        /// strategy goal type.
        IncompatibleBiddingStrategyAndBiddingStrategyGoalType = 6,
    }
}
// Proto file describing billing setup errors.

/// Container for enum describing possible billing setup errors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BillingSetupErrorEnum {}
/// Nested message and enum types in `BillingSetupErrorEnum`.
pub mod billing_setup_error_enum {
    /// Enum describing possible billing setup errors.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum BillingSetupError {
        /// Enum unspecified.
        Unspecified = 0,
        /// The received error code is not known in this version.
        Unknown = 1,
        /// Cannot specify both an existing payments account and a new payments
        /// account when setting up billing.
        CannotUseExistingAndNewAccount = 2,
        /// Cannot cancel an approved billing setup whose start time has passed.
        CannotRemoveStartedBillingSetup = 3,
        /// Cannot perform a Change of Bill-To (CBT) to the same payments account.
        CannotChangeBillingToSamePaymentsAccount = 4,
        /// Billing setups can only be used by customers with ENABLED or DRAFT
        /// status.
        BillingSetupNotPermittedForCustomerStatus = 5,
        /// Billing setups must either include a correctly formatted existing
        /// payments account id, or a non-empty new payments account name.
        InvalidPaymentsAccount = 6,
        /// Only billable and third-party customers can create billing setups.
        BillingSetupNotPermittedForCustomerCategory = 7,
        /// Billing setup creations can only use NOW for start time type.
        InvalidStartTimeType = 8,
        /// Billing setups can only be created for a third-party customer if they do
        /// not already have a setup.
        ThirdPartyAlreadyHasBilling = 9,
        /// Billing setups cannot be created if there is already a pending billing in
        /// progress.
        BillingSetupInProgress = 10,
        /// Billing setups can only be created by customers who have permission to
        /// setup billings. Users can contact a representative for help setting up
        /// permissions.
        NoSignupPermission = 11,
        /// Billing setups cannot be created if there is already a future-approved
        /// billing.
        ChangeOfBillToInProgress = 12,
        /// Requested payments profile not found.
        PaymentsProfileNotFound = 13,
        /// Requested payments account not found.
        PaymentsAccountNotFound = 14,
        /// Billing setup creation failed because the payments profile is ineligible.
        PaymentsProfileIneligible = 15,
        /// Billing setup creation failed because the payments account is ineligible.
        PaymentsAccountIneligible = 16,
        /// Billing setup creation failed because the payments profile needs internal
        /// approval.
        CustomerNeedsInternalApproval = 17,
        /// Payments account has different currency code than the current customer
        /// and hence cannot be used to setup billing.
        PaymentsAccountIneligibleCurrencyCodeMismatch = 19,
        /// A start time in the future cannot be used because there is currently no
        /// active billing setup for this customer.
        FutureStartTimeProhibited = 20,
    }
}
// Proto file describing campaign budget errors.

/// Container for enum describing possible campaign budget errors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CampaignBudgetErrorEnum {}
/// Nested message and enum types in `CampaignBudgetErrorEnum`.
pub mod campaign_budget_error_enum {
    /// Enum describing possible campaign budget errors.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum CampaignBudgetError {
        /// Enum unspecified.
        Unspecified = 0,
        /// The received error code is not known in this version.
        Unknown = 1,
        /// The campaign budget cannot be shared.
        CampaignBudgetCannotBeShared = 17,
        /// The requested campaign budget no longer exists.
        CampaignBudgetRemoved = 2,
        /// The campaign budget is associated with at least one campaign, and so the
        /// campaign budget cannot be removed.
        CampaignBudgetInUse = 3,
        /// Customer is not on the allow-list for this campaign budget period.
        CampaignBudgetPeriodNotAvailable = 4,
        /// This field is not mutable on implicitly shared campaign budgets
        CannotModifyFieldOfImplicitlySharedCampaignBudget = 6,
        /// Cannot change explicitly shared campaign budgets back to implicitly
        /// shared ones.
        CannotUpdateCampaignBudgetToImplicitlyShared = 7,
        /// An implicit campaign budget without a name cannot be changed to
        /// explicitly shared campaign budget.
        CannotUpdateCampaignBudgetToExplicitlySharedWithoutName = 8,
        /// Cannot change an implicitly shared campaign budget to an explicitly
        /// shared one.
        CannotUpdateCampaignBudgetToExplicitlyShared = 9,
        /// Only explicitly shared campaign budgets can be used with multiple
        /// campaigns.
        CannotUseImplicitlySharedCampaignBudgetWithMultipleCampaigns = 10,
        /// A campaign budget with this name already exists.
        DuplicateName = 11,
        /// A money amount was not in the expected currency.
        MoneyAmountInWrongCurrency = 12,
        /// A money amount was less than the minimum CPC for currency.
        MoneyAmountLessThanCurrencyMinimumCpc = 13,
        /// A money amount was greater than the maximum allowed.
        MoneyAmountTooLarge = 14,
        /// A money amount was negative.
        NegativeMoneyAmount = 15,
        /// A money amount was not a multiple of a minimum unit.
        NonMultipleOfMinimumCurrencyUnit = 16,
        /// Total budget amount must be unset when BudgetPeriod is DAILY.
        TotalBudgetAmountMustBeUnsetForBudgetPeriodDaily = 18,
        /// The period of the budget is not allowed.
        InvalidPeriod = 19,
    }
}
// Proto file describing campaign conversion goal errors.

/// Container for enum describing possible campaign conversion goal errors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CampaignConversionGoalErrorEnum {}
/// Nested message and enum types in `CampaignConversionGoalErrorEnum`.
pub mod campaign_conversion_goal_error_enum {
    /// Enum describing possible campaign conversion goal errors.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum CampaignConversionGoalError {
        /// Enum unspecified.
        Unspecified = 0,
        /// The received error code is not known in this version.
        Unknown = 1,
        /// Campaign is managed by Search Ads 360 but uses Unified Goal.
        CannotUseCampaignGoalForSearchAds360ManagedCampaign = 2,
    }
}
// Proto file describing campaign criterion errors.

/// Container for enum describing possible campaign criterion errors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CampaignCriterionErrorEnum {}
/// Nested message and enum types in `CampaignCriterionErrorEnum`.
pub mod campaign_criterion_error_enum {
    /// Enum describing possible campaign criterion errors.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum CampaignCriterionError {
        /// Enum unspecified.
        Unspecified = 0,
        /// The received error code is not known in this version.
        Unknown = 1,
        /// Concrete type of criterion (keyword v.s. placement) is required for
        /// CREATE and UPDATE operations.
        ConcreteTypeRequired = 2,
        /// Invalid placement URL.
        InvalidPlacementUrl = 3,
        /// Criteria type can not be excluded for the campaign by the customer. like
        /// AOL account type cannot target site type criteria
        CannotExcludeCriteriaType = 4,
        /// Cannot set the campaign criterion status for this criteria type.
        CannotSetStatusForCriteriaType = 5,
        /// Cannot set the campaign criterion status for an excluded criteria.
        CannotSetStatusForExcludedCriteria = 6,
        /// Cannot target and exclude the same criterion.
        CannotTargetAndExclude = 7,
        /// The mutate contained too many operations.
        TooManyOperations = 8,
        /// This operator cannot be applied to a criterion of this type.
        OperatorNotSupportedForCriterionType = 9,
        /// The Shopping campaign sales country is not supported for
        /// ProductSalesChannel targeting.
        ShoppingCampaignSalesCountryNotSupportedForSalesChannel = 10,
        /// The existing field can't be updated with CREATE operation. It can be
        /// updated with UPDATE operation only.
        CannotAddExistingField = 11,
        /// Negative criteria are immutable, so updates are not allowed.
        CannotUpdateNegativeCriterion = 12,
        /// Only free form names are allowed for negative Smart campaign keyword
        /// theme.
        CannotSetNegativeKeywordThemeConstantCriterion = 13,
        /// Invalid Smart campaign keyword theme constant criterion.
        InvalidKeywordThemeConstant = 14,
        /// A Smart campaign keyword theme constant or free-form Smart campaign
        /// keyword theme is required.
        MissingKeywordThemeConstantOrFreeFormKeywordTheme = 15,
        /// A Smart campaign may not target proximity and location criteria
        /// simultaneously.
        CannotTargetBothProximityAndLocationCriteriaForSmartCampaign = 16,
        /// A Smart campaign may not target multiple proximity criteria.
        CannotTargetMultipleProximityCriteriaForSmartCampaign = 17,
    }
}
// Proto file describing campaign customizer errors.

/// Container for enum describing possible campaign customizer errors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CampaignCustomizerErrorEnum {}
/// Nested message and enum types in `CampaignCustomizerErrorEnum`.
pub mod campaign_customizer_error_enum {
    /// Enum describing possible campaign customizer errors.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum CampaignCustomizerError {
        /// Enum unspecified.
        Unspecified = 0,
        /// The received error code is not known in this version.
        Unknown = 1,
    }
}
// Proto file describing campaign draft errors.

/// Container for enum describing possible campaign draft errors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CampaignDraftErrorEnum {}
/// Nested message and enum types in `CampaignDraftErrorEnum`.
pub mod campaign_draft_error_enum {
    /// Enum describing possible campaign draft errors.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum CampaignDraftError {
        /// Enum unspecified.
        Unspecified = 0,
        /// The received error code is not known in this version.
        Unknown = 1,
        /// A draft with this name already exists for this campaign.
        DuplicateDraftName = 2,
        /// The draft is removed and cannot be transitioned to another status.
        InvalidStatusTransitionFromRemoved = 3,
        /// The draft has been promoted and cannot be transitioned to the specified
        /// status.
        InvalidStatusTransitionFromPromoted = 4,
        /// The draft has failed to be promoted and cannot be transitioned to the
        /// specified status.
        InvalidStatusTransitionFromPromoteFailed = 5,
        /// This customer is not allowed to create drafts.
        CustomerCannotCreateDraft = 6,
        /// This campaign is not allowed to create drafts.
        CampaignCannotCreateDraft = 7,
        /// This modification cannot be made on a draft.
        InvalidDraftChange = 8,
        /// The draft cannot be transitioned to the specified status from its
        /// current status.
        InvalidStatusTransition = 9,
        /// The campaign has reached the maximum number of drafts that can be created
        /// for a campaign throughout its lifetime. No additional drafts can be
        /// created for this campaign. Removed drafts also count towards this limit.
        MaxNumberOfDraftsPerCampaignReached = 10,
        /// ListAsyncErrors was called without first promoting the draft.
        ListErrorsForPromotedDraftOnly = 11,
    }
}
// Proto file describing campaign errors.

/// Container for enum describing possible campaign errors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CampaignErrorEnum {}
/// Nested message and enum types in `CampaignErrorEnum`.
pub mod campaign_error_enum {
    /// Enum describing possible campaign errors.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum CampaignError {
        /// Enum unspecified.
        Unspecified = 0,
        /// The received error code is not known in this version.
        Unknown = 1,
        /// Cannot target content network.
        CannotTargetContentNetwork = 3,
        /// Cannot target search network.
        CannotTargetSearchNetwork = 4,
        /// Cannot cover search network without google search network.
        CannotTargetSearchNetworkWithoutGoogleSearch = 5,
        /// Cannot target Google Search network for a CPM campaign.
        CannotTargetGoogleSearchForCpmCampaign = 6,
        /// Must target at least one network.
        CampaignMustTargetAtLeastOneNetwork = 7,
        /// Only some Google partners are allowed to target partner search network.
        CannotTargetPartnerSearchNetwork = 8,
        /// Cannot target content network only as campaign has criteria-level bidding
        /// strategy.
        CannotTargetContentNetworkOnlyWithCriteriaLevelBiddingStrategy = 9,
        /// Cannot modify the start or end date such that the campaign duration would
        /// not contain the durations of all runnable trials.
        CampaignDurationMustContainAllRunnableTrials = 10,
        /// Cannot modify dates, budget or status of a trial campaign.
        CannotModifyForTrialCampaign = 11,
        /// Trying to modify the name of an active or paused campaign, where the name
        /// is already assigned to another active or paused campaign.
        DuplicateCampaignName = 12,
        /// Two fields are in conflicting modes.
        IncompatibleCampaignField = 13,
        /// Campaign name cannot be used.
        InvalidCampaignName = 14,
        /// Given status is invalid.
        InvalidAdServingOptimizationStatus = 15,
        /// Error in the campaign level tracking URL.
        InvalidTrackingUrl = 16,
        /// Cannot set both tracking URL template and tracking setting. A user has
        /// to clear legacy tracking setting in order to add tracking URL template.
        CannotSetBothTrackingUrlTemplateAndTrackingSetting = 17,
        /// The maximum number of impressions for Frequency Cap should be an integer
        /// greater than 0.
        MaxImpressionsNotInRange = 18,
        /// Only the Day, Week and Month time units are supported.
        TimeUnitNotSupported = 19,
        /// Operation not allowed on a campaign whose serving status has ended
        InvalidOperationIfServingStatusHasEnded = 20,
        /// This budget is exclusively linked to a Campaign that is using experiments
        /// so it cannot be shared.
        BudgetCannotBeShared = 21,
        /// Campaigns using experiments cannot use a shared budget.
        CampaignCannotUseSharedBudget = 22,
        /// A different budget cannot be assigned to a campaign when there are
        /// running or scheduled trials.
        CannotChangeBudgetOnCampaignWithTrials = 23,
        /// No link found between the campaign and the label.
        CampaignLabelDoesNotExist = 24,
        /// The label has already been attached to the campaign.
        CampaignLabelAlreadyExists = 25,
        /// A ShoppingSetting was not found when creating a shopping campaign.
        MissingShoppingSetting = 26,
        /// The country in shopping setting is not an allowed country.
        InvalidShoppingSalesCountry = 27,
        /// The requested channel type is not available according to the customer's
        /// account setting.
        AdvertisingChannelTypeNotAvailableForAccountType = 31,
        /// The AdvertisingChannelSubType is not a valid subtype of the primary
        /// channel type.
        InvalidAdvertisingChannelSubType = 32,
        /// At least one conversion must be selected.
        AtLeastOneConversionMustBeSelected = 33,
        /// Setting ad rotation mode for a campaign is not allowed. Ad rotation mode
        /// at campaign is deprecated.
        CannotSetAdRotationMode = 34,
        /// Trying to change start date on a campaign that has started.
        CannotModifyStartDateIfAlreadyStarted = 35,
        /// Trying to modify a date into the past.
        CannotSetDateToPast = 36,
        /// Hotel center id in the hotel setting does not match any customer links.
        MissingHotelCustomerLink = 37,
        /// Hotel center id in the hotel setting must match an active customer link.
        InvalidHotelCustomerLink = 38,
        /// Hotel setting was not found when creating a hotel ads campaign.
        MissingHotelSetting = 39,
        /// A Campaign cannot use shared campaign budgets and be part of a campaign
        /// group.
        CannotUseSharedCampaignBudgetWhilePartOfCampaignGroup = 40,
        /// The app ID was not found.
        AppNotFound = 41,
        /// Campaign.shopping_setting.enable_local is not supported for the specified
        /// campaign type.
        ShoppingEnableLocalNotSupportedForCampaignType = 42,
        /// The merchant does not support the creation of campaigns for Shopping
        /// Comparison Listing Ads.
        MerchantNotAllowedForComparisonListingAds = 43,
        /// The App campaign for engagement cannot be created because there aren't
        /// enough installs.
        InsufficientAppInstallsCount = 44,
        /// The App campaign for engagement cannot be created because the app is
        /// sensitive.
        SensitiveCategoryApp = 45,
        /// Customers with Housing, Employment, or Credit ads must accept updated
        /// personalized ads policy to continue creating campaigns.
        HecAgreementRequired = 46,
        /// The field is not compatible with view through conversion optimization.
        NotCompatibleWithViewThroughConversionOptimization = 49,
        /// The field type cannot be excluded because an active campaign-asset link
        /// of this type exists.
        InvalidExcludedParentAssetFieldType = 48,
        /// The app pre-registration campaign cannot be created for non-Android
        /// applications.
        CannotCreateAppPreRegistrationForNonAndroidApp = 50,
        /// The campaign cannot be created since the app is not available for
        /// pre-registration in any country.
        AppNotAvailableToCreateAppPreRegistrationCampaign = 51,
        /// The type of the Budget is not compatible with this Campaign.
        IncompatibleBudgetType = 52,
    }
}
// Proto file describing campaign experiment errors.

/// Container for enum describing possible campaign experiment errors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CampaignExperimentErrorEnum {}
/// Nested message and enum types in `CampaignExperimentErrorEnum`.
pub mod campaign_experiment_error_enum {
    /// Enum describing possible campaign experiment errors.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum CampaignExperimentError {
        /// Enum unspecified.
        Unspecified = 0,
        /// The received error code is not known in this version.
        Unknown = 1,
        /// An active campaign or experiment with this name already exists.
        DuplicateName = 2,
        /// Experiment cannot be updated from the current state to the
        /// requested target state. For example, an experiment can only graduate
        /// if its status is ENABLED.
        InvalidTransition = 3,
        /// Cannot create an experiment from a campaign using an explicitly shared
        /// budget.
        CannotCreateExperimentWithSharedBudget = 4,
        /// Cannot create an experiment for a removed base campaign.
        CannotCreateExperimentForRemovedBaseCampaign = 5,
        /// Cannot create an experiment from a draft, which has a status other than
        /// proposed.
        CannotCreateExperimentForNonProposedDraft = 6,
        /// This customer is not allowed to create an experiment.
        CustomerCannotCreateExperiment = 7,
        /// This campaign is not allowed to create an experiment.
        CampaignCannotCreateExperiment = 8,
        /// Trying to set an experiment duration which overlaps with another
        /// experiment.
        ExperimentDurationsMustNotOverlap = 9,
        /// All non-removed experiments must start and end within their campaign's
        /// duration.
        ExperimentDurationMustBeWithinCampaignDuration = 10,
        /// The experiment cannot be modified because its status is in a terminal
        /// state, such as REMOVED.
        CannotMutateExperimentDueToStatus = 11,
    }
}
// Proto file describing campaign feed errors.

/// Container for enum describing possible campaign feed errors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CampaignFeedErrorEnum {}
/// Nested message and enum types in `CampaignFeedErrorEnum`.
pub mod campaign_feed_error_enum {
    /// Enum describing possible campaign feed errors.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum CampaignFeedError {
        /// Enum unspecified.
        Unspecified = 0,
        /// The received error code is not known in this version.
        Unknown = 1,
        /// An active feed already exists for this campaign and placeholder type.
        FeedAlreadyExistsForPlaceholderType = 2,
        /// The specified feed is removed.
        CannotCreateForRemovedFeed = 4,
        /// The CampaignFeed already exists. UPDATE should be used to modify the
        /// existing CampaignFeed.
        CannotCreateAlreadyExistingCampaignFeed = 5,
        /// Cannot update removed campaign feed.
        CannotModifyRemovedCampaignFeed = 6,
        /// Invalid placeholder type.
        InvalidPlaceholderType = 7,
        /// Feed mapping for this placeholder type does not exist.
        MissingFeedmappingForPlaceholderType = 8,
        /// Location CampaignFeeds cannot be created unless there is a location
        /// CustomerFeed for the specified feed.
        NoExistingLocationCustomerFeed = 9,
    }
}
// Proto file describing campaign shared set errors.

/// Container for enum describing possible campaign shared set errors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CampaignSharedSetErrorEnum {}
/// Nested message and enum types in `CampaignSharedSetErrorEnum`.
pub mod campaign_shared_set_error_enum {
    /// Enum describing possible campaign shared set errors.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum CampaignSharedSetError {
        /// Enum unspecified.
        Unspecified = 0,
        /// The received error code is not known in this version.
        Unknown = 1,
        /// The shared set belongs to another customer and permission isn't granted.
        SharedSetAccessDenied = 2,
    }
}
// Proto file describing change event errors.

/// Container for enum describing possible change event errors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangeEventErrorEnum {}
/// Nested message and enum types in `ChangeEventErrorEnum`.
pub mod change_event_error_enum {
    /// Enum describing possible change event errors.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ChangeEventError {
        /// Enum unspecified.
        Unspecified = 0,
        /// The received error code is not known in this version.
        Unknown = 1,
        /// The requested start date is too old. It cannot be older than 30 days.
        StartDateTooOld = 2,
        /// The change_event search request must specify a finite range filter
        /// on change_date_time.
        ChangeDateRangeInfinite = 3,
        /// The change event search request has specified invalid date time filters
        /// that can never logically produce any valid results (for example, start
        /// time after end time).
        ChangeDateRangeNegative = 4,
        /// The change_event search request must specify a LIMIT.
        LimitNotSpecified = 5,
        /// The LIMIT specified by change_event request should be less than or equal
        /// to 10K.
        InvalidLimitClause = 6,
    }
}
// Proto file describing change status errors.

/// Container for enum describing possible change status errors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangeStatusErrorEnum {}
/// Nested message and enum types in `ChangeStatusErrorEnum`.
pub mod change_status_error_enum {
    /// Enum describing possible change status errors.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ChangeStatusError {
        /// Enum unspecified.
        Unspecified = 0,
        /// The received error code is not known in this version.
        Unknown = 1,
        /// The requested start date is too old.
        StartDateTooOld = 3,
        /// The change_status search request must specify a finite range filter
        /// on last_change_date_time.
        ChangeDateRangeInfinite = 4,
        /// The change status search request has specified invalid date time filters
        /// that can never logically produce any valid results (for example, start
        /// time after end time).
        ChangeDateRangeNegative = 5,
        /// The change_status search request must specify a LIMIT.
        LimitNotSpecified = 6,
        /// The LIMIT specified by change_status request should be less than or equal
        /// to 10K.
        InvalidLimitClause = 7,
    }
}
// Proto file describing collection size errors.

/// Container for enum describing possible collection size errors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CollectionSizeErrorEnum {}
/// Nested message and enum types in `CollectionSizeErrorEnum`.
pub mod collection_size_error_enum {
    /// Enum describing possible collection size errors.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum CollectionSizeError {
        /// Enum unspecified.
        Unspecified = 0,
        /// The received error code is not known in this version.
        Unknown = 1,
        /// Too few.
        TooFew = 2,
        /// Too many.
        TooMany = 3,
    }
}
// Proto file describing context errors.

/// Container for enum describing possible context errors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContextErrorEnum {}
/// Nested message and enum types in `ContextErrorEnum`.
pub mod context_error_enum {
    /// Enum describing possible context errors.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ContextError {
        /// Enum unspecified.
        Unspecified = 0,
        /// The received error code is not known in this version.
        Unknown = 1,
        /// The operation is not allowed for the given context.
        OperationNotPermittedForContext = 2,
        /// The operation is not allowed for removed resources.
        OperationNotPermittedForRemovedResource = 3,
    }
}
// Proto file describing conversion action errors.

/// Container for enum describing possible conversion action errors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConversionActionErrorEnum {}
/// Nested message and enum types in `ConversionActionErrorEnum`.
pub mod conversion_action_error_enum {
    /// Enum describing possible conversion action errors.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ConversionActionError {
        /// Enum unspecified.
        Unspecified = 0,
        /// The received error code is not known in this version.
        Unknown = 1,
        /// The specified conversion action name already exists.
        DuplicateName = 2,
        /// Another conversion action with the specified app id already exists.
        DuplicateAppId = 3,
        /// Android first open action conflicts with Google play codeless download
        /// action tracking the same app.
        TwoConversionActionsBiddingOnSameAppDownload = 4,
        /// Android first open action conflicts with Google play codeless download
        /// action tracking the same app.
        BiddingOnSameAppDownloadAsGlobalAction = 5,
        /// The attribution model cannot be set to DATA_DRIVEN because a data-driven
        /// model has never been generated.
        DataDrivenModelWasNeverGenerated = 6,
        /// The attribution model cannot be set to DATA_DRIVEN because the
        /// data-driven model is expired.
        DataDrivenModelExpired = 7,
        /// The attribution model cannot be set to DATA_DRIVEN because the
        /// data-driven model is stale.
        DataDrivenModelStale = 8,
        /// The attribution model cannot be set to DATA_DRIVEN because the
        /// data-driven model is unavailable or the conversion action was newly
        /// added.
        DataDrivenModelUnknown = 9,
        /// Creation of this conversion action type isn't supported by Google
        /// Ads API.
        CreationNotSupported = 10,
        /// Update of this conversion action isn't supported by Google Ads API.
        UpdateNotSupported = 11,
    }
}
// Proto file describing conversion adjustment upload errors.

/// Container for enum describing possible conversion adjustment upload errors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConversionAdjustmentUploadErrorEnum {}
/// Nested message and enum types in `ConversionAdjustmentUploadErrorEnum`.
pub mod conversion_adjustment_upload_error_enum {
    /// Enum describing possible conversion adjustment upload errors.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ConversionAdjustmentUploadError {
        /// Not specified.
        Unspecified = 0,
        /// The received error code is not known in this version.
        Unknown = 1,
        /// The specified conversion action was created too recently.
        /// Please try the upload again after 4-6 hours have passed since the
        /// conversion action was created.
        TooRecentConversionAction = 2,
        /// No conversion action of a supported ConversionActionType that matches the
        /// provided information can be found for the customer.
        InvalidConversionAction = 3,
        /// A retraction was already reported for this conversion.
        ConversionAlreadyRetracted = 4,
        /// A conversion for the supplied combination of conversion
        /// action and conversion identifier could not be found.
        ConversionNotFound = 5,
        /// The specified conversion has already expired. Conversions expire after 55
        /// days, after which adjustments cannot be reported against them.
        ConversionExpired = 6,
        /// The supplied adjustment date time precedes that of the original
        /// conversion.
        AdjustmentPrecedesConversion = 7,
        /// A restatement with a more recent adjustment date time was already
        /// reported for this conversion.
        MoreRecentRestatementFound = 8,
        /// The conversion was created too recently.
        TooRecentConversion = 9,
        /// Restatements cannot be reported for a conversion action that always uses
        /// the default value.
        CannotRestateConversionActionThatAlwaysUsesDefaultConversionValue = 10,
        /// The request contained more than 2000 adjustments.
        TooManyAdjustmentsInRequest = 11,
        /// The conversion has been adjusted too many times.
        TooManyAdjustments = 12,
        /// A restatement with this timestamp already exists for this conversion. To
        /// upload another adjustment, please use a different timestamp.
        RestatementAlreadyExists = 13,
        /// This adjustment has the same timestamp as another adjustment in the
        /// request for this conversion. To upload another adjustment, please use a
        /// different timestamp.
        DuplicateAdjustmentInRequest = 14,
        /// The customer has not accepted the customer data terms in the conversion
        /// settings page.
        CustomerNotAcceptedCustomerDataTerms = 15,
        /// The enhanced conversion settings of the conversion action supplied is
        /// not eligible for enhancements.
        ConversionActionNotEligibleForEnhancement = 16,
        /// The provided user identifier is not a SHA-256 hash. It is either unhashed
        /// or hashed using a different hash function.
        InvalidUserIdentifier = 17,
        /// The provided user identifier is not supported.
        /// ConversionAdjustmentUploadService only supports hashed_email,
        /// hashed_phone_number, and address_info.
        UnsupportedUserIdentifier = 18,
        /// Cannot set both gclid_date_time_pair and order_id.
        GclidDateTimePairAndOrderIdBothSet = 20,
        /// An enhancement with this conversion action and order_id already exists
        /// for this conversion.
        ConversionAlreadyEnhanced = 21,
        /// This enhancement has the same conversion action and order_id as
        /// another enhancement in the request.
        DuplicateEnhancementInRequest = 22,
        /// Per our customer data policies, enhancement has been prohibited in your
        /// account. If you have any questions, please contact your Google
        /// representative.
        CustomerDataPolicyProhibitsEnhancement = 23,
        /// The conversion adjustment is for a conversion action of type WEBPAGE, but
        /// does not have an order_id. The order_id is required for an adjustment for
        /// a WEBPAGE conversion action.
        MissingOrderIdForWebpage = 24,
    }
}
// Proto file describing conversion custom variable errors.

/// Container for enum describing possible conversion custom variable errors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConversionCustomVariableErrorEnum {}
/// Nested message and enum types in `ConversionCustomVariableErrorEnum`.
pub mod conversion_custom_variable_error_enum {
    /// Enum describing possible conversion custom variable errors.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ConversionCustomVariableError {
        /// Enum unspecified.
        Unspecified = 0,
        /// The received error code is not known in this version.
        Unknown = 1,
        /// A conversion custom variable with the specified name already exists.
        DuplicateName = 2,
        /// A conversion custom variable with the specified tag already exists.
        DuplicateTag = 3,
        /// A conversion custom variable with the specified tag is reserved for other
        /// uses.
        ReservedTag = 4,
    }
}
// Proto file describing conversion goal campaign config errors.

/// Container for enum describing possible conversion goal campaign config
/// errors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConversionGoalCampaignConfigErrorEnum {}
/// Nested message and enum types in `ConversionGoalCampaignConfigErrorEnum`.
pub mod conversion_goal_campaign_config_error_enum {
    /// Enum describing possible conversion goal campaign config errors.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ConversionGoalCampaignConfigError {
        /// Enum unspecified.
        Unspecified = 0,
        /// The received error code is not known in this version.
        Unknown = 1,
        /// Campaign is managed by Search Ads 360 but uses Unified Goal.
        CannotUseCampaignGoalForSearchAds360ManagedCampaign = 2,
        /// The campaign is using a custom goal that does not belong to its Google
        /// Ads conversion customer (conversion tracking customer).
        CustomGoalDoesNotBelongToGoogleAdsConversionCustomer = 3,
        /// The campaign is not allowed to use unified goals.
        CampaignCannotUseUnifiedGoals = 4,
    }
}
// Proto file describing conversion upload errors.

/// Container for enum describing possible conversion upload errors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConversionUploadErrorEnum {}
/// Nested message and enum types in `ConversionUploadErrorEnum`.
pub mod conversion_upload_error_enum {
    /// Enum describing possible conversion upload errors.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ConversionUploadError {
        /// Enum unspecified.
        Unspecified = 0,
        /// The received error code is not known in this version.
        Unknown = 1,
        /// The request contained more than 2000 conversions.
        TooManyConversionsInRequest = 2,
        /// The specified gclid could not be decoded.
        UnparseableGclid = 3,
        /// The specified conversion_date_time is before the event time
        /// associated with the given identifier or iOS URL parameter.
        ConversionPrecedesEvent = 42,
        /// The click associated with the given identifier or iOS URL parameter is
        /// either too old to be imported or occurred outside of the click through
        /// lookback window for the specified conversion action.
        ExpiredEvent = 43,
        /// The click associated with the given identifier or iOS URL parameter
        /// occurred too recently. Please try uploading again after 6 hours have
        /// passed since the click occurred.
        TooRecentEvent = 44,
        /// The click associated with the given identifier or iOS URL parameter could
        /// not be found in the system. This can happen if the identifier or iOS URL
        /// parameter are collected for non Google Ads clicks.
        EventNotFound = 45,
        /// The click associated with the given identifier or iOS URL parameter is
        /// owned by a customer account that the uploading customer does not manage.
        UnauthorizedCustomer = 8,
        /// No upload eligible conversion action that matches the provided
        /// information can be found for the customer.
        InvalidConversionAction = 9,
        /// The specified conversion action was created too recently.
        /// Please try the upload again after 4-6 hours have passed since the
        /// conversion action was created.
        TooRecentConversionAction = 10,
        /// The click associated with the given identifier does not contain
        /// conversion tracking information.
        ConversionTrackingNotEnabledAtImpressionTime = 11,
        /// The specified conversion action does not use an external attribution
        /// model, but external_attribution_data was set.
        ExternalAttributionDataSetForNonExternallyAttributedConversionAction = 12,
        /// The specified conversion action uses an external attribution model, but
        /// external_attribution_data or one of its contained fields was not set.
        /// Both external_attribution_credit and external_attribution_model must be
        /// set for externally attributed conversion actions.
        ExternalAttributionDataNotSetForExternallyAttributedConversionAction = 13,
        /// Order IDs are not supported for conversion actions which use an external
        /// attribution model.
        OrderIdNotPermittedForExternallyAttributedConversionAction = 14,
        /// A conversion with the same order id and conversion action combination
        /// already exists in our system.
        OrderIdAlreadyInUse = 15,
        /// The request contained two or more conversions with the same order id and
        /// conversion action combination.
        DuplicateOrderId = 16,
        /// The call occurred too recently. Please try uploading again after 12 hours
        /// have passed since the call occurred.
        TooRecentCall = 17,
        /// The click that initiated the call is too old for this conversion to be
        /// imported.
        ExpiredCall = 18,
        /// The call or the click leading to the call was not found.
        CallNotFound = 19,
        /// The specified conversion_date_time is before the call_start_date_time.
        ConversionPrecedesCall = 20,
        /// The click associated with the call does not contain conversion tracking
        /// information.
        ConversionTrackingNotEnabledAtCallTime = 21,
        /// The caller's phone number cannot be parsed. It should be formatted either
        /// as E.164 "+16502531234", International "+64 3-331 6005" or US national
        /// number "6502531234".
        UnparseableCallersPhoneNumber = 22,
        /// A conversion with this timestamp already exists for this click. To upload
        /// another conversion, please use a different timestamp.
        ClickConversionAlreadyExists = 23,
        /// A conversion with this timestamp already exists for this call. To upload
        /// another conversion, please use a different timestamp.
        CallConversionAlreadyExists = 24,
        /// This conversion has the same click and timestamp as another conversion in
        /// the request. To upload another conversion for this click, please use a
        /// different timestamp.
        DuplicateClickConversionInRequest = 25,
        /// This conversion has the same call and timestamp as another conversion in
        /// the request. To upload another conversion for this call, please use a
        /// different timestamp.
        DuplicateCallConversionInRequest = 26,
        /// The custom variable is not enabled.
        CustomVariableNotEnabled = 28,
        /// The value of the custom variable contains personally identifiable
        /// information (PII), such as an email address or phone number.
        CustomVariableValueContainsPii = 29,
        /// The click associated with the given identifier or iOS URL parameter isn't
        /// from the account where conversion tracking is set up.
        InvalidCustomerForClick = 30,
        /// The click associated with the given call isn't from the account where
        /// conversion tracking is set up.
        InvalidCustomerForCall = 31,
        /// The conversion can't be uploaded because the conversion source didn't
        /// comply with the App Tracking Transparency (ATT) policy or the person who
        /// converted didn't consent to tracking.
        ConversionNotCompliantWithAttPolicy = 32,
        /// No click was found for the provided user identifiers that could be
        /// applied to the specified conversion action.
        ClickNotFound = 33,
        /// The provided user identifier is not a SHA-256 hash. It is either unhashed
        /// or hashed using a different hash function.
        InvalidUserIdentifier = 34,
        /// Conversion actions which use an external attribution model cannot be used
        /// with UserIdentifier.
        ExternallyAttributedConversionActionNotPermittedWithUserIdentifier = 35,
        /// The provided user identifier is not supported. ConversionUploadService
        /// only supports hashed_email and hashed_phone_number.
        UnsupportedUserIdentifier = 36,
        /// gbraid and wbraid are both set in the request. Only one is allowed.
        GbraidWbraidBothSet = 38,
        /// The specified wbraid could not be decoded.
        UnparseableWbraid = 39,
        /// The specified gbraid could not be decoded.
        UnparseableGbraid = 40,
        /// Conversion types which use an external attribution model cannot be used
        /// with gbraid or wbraid.
        ExternallyAttributedConversionTypeNotPermittedWithBraid = 41,
        /// Conversion actions which use the one-per-click counting type cannot be
        /// used with gbraid or wbraid.
        OnePerClickConversionActionNotPermittedWithBraid = 46,
        /// Per our customer data policies, enhanced conversions have been prohibited
        /// in your account. If you have any questions, please contact your Google
        /// representative.
        CustomerDataPolicyProhibitsEnhancedConversions = 47,
        /// The customer has not accepted the customer data terms in the conversion
        /// settings page.
        CustomerNotAcceptedCustomerDataTerms = 48,
    }
}
// Proto file describing conversion value rule errors.

/// Container for enum describing possible conversion value rule errors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConversionValueRuleErrorEnum {}
/// Nested message and enum types in `ConversionValueRuleErrorEnum`.
pub mod conversion_value_rule_error_enum {
    /// Enum describing possible conversion value rule errors.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ConversionValueRuleError {
        /// Enum unspecified.
        Unspecified = 0,
        /// The received error code is not known in this version.
        Unknown = 1,
        /// The value rule's geo location condition contains invalid geo target
        /// constant(s), i.e. there's no matching geo target.
        InvalidGeoTargetConstant = 2,
        /// The value rule's geo location condition contains conflicting included and
        /// excluded geo targets. Specifically, some of the excluded geo target(s)
        /// are the same as or contain some of the included geo target(s). For
        /// example, the geo location condition includes California but excludes U.S.
        ConflictingIncludedAndExcludedGeoTarget = 3,
        /// User specified conflicting conditions for two value rules in the same
        /// value rule set.
        ConflictingConditions = 4,
        /// The value rule cannot be removed because it's still included in some
        /// value rule set.
        CannotRemoveIfIncludedInValueRuleSet = 5,
        /// The value rule contains a condition that's not allowed by the value rule
        /// set including this value rule.
        ConditionNotAllowed = 6,
        /// The value rule contains a field that should be unset.
        FieldMustBeUnset = 7,
        /// Pausing the value rule requires pausing the value rule set because the
        /// value rule is (one of) the last enabled in the value rule set.
        CannotPauseUnlessValueRuleSetIsPaused = 8,
        /// The value rule's geo location condition contains untargetable geo target
        /// constant(s).
        UntargetableGeoTarget = 9,
        /// The value rule's audience condition contains invalid user list(s). In
        /// another word, there's no matching user list.
        InvalidAudienceUserList = 10,
        /// The value rule's audience condition contains inaccessible user list(s).
        InaccessibleUserList = 11,
        /// The value rule's audience condition contains invalid user_interest(s).
        /// This might be because there is no matching user interest, or the user
        /// interest is not visible.
        InvalidAudienceUserInterest = 12,
        /// When a value rule is created, it shouldn't have REMOVED status.
        CannotAddRuleWithStatusRemoved = 13,
    }
}
// Proto file describing conversion value rule set errors.

/// Container for enum describing possible conversion value rule set errors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConversionValueRuleSetErrorEnum {}
/// Nested message and enum types in `ConversionValueRuleSetErrorEnum`.
pub mod conversion_value_rule_set_error_enum {
    /// Enum describing possible conversion value rule set errors.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ConversionValueRuleSetError {
        /// Enum unspecified.
        Unspecified = 0,
        /// The received error code is not known in this version.
        Unknown = 1,
        /// Two value rules in this value rule set contain conflicting conditions.
        ConflictingValueRuleConditions = 2,
        /// This value rule set includes a value rule that cannot be found, has been
        /// permanently removed or belongs to a different customer.
        InvalidValueRule = 3,
        /// An error that's thrown when a mutate operation is trying to
        /// replace/remove some existing elements in the dimensions field. In other
        /// words, ADD op is always fine and UPDATE op is fine if it's only appending
        /// new elements into dimensions list.
        DimensionsUpdateOnlyAllowAppend = 4,
        /// An error that's thrown when a mutate is adding new value rule(s) into a
        /// value rule set and the added value rule(s) include conditions that are
        /// not specified in the dimensions of the value rule set.
        ConditionTypeNotAllowed = 5,
        /// The dimensions field contains duplicate elements.
        DuplicateDimensions = 6,
        /// This value rule set is attached to an invalid campaign id. Either a
        /// campaign with this campaign id doesn't exist or it belongs to a different
        /// customer.
        InvalidCampaignId = 7,
        /// When a mutate request tries to pause a value rule set, the enabled
        /// value rules in this set must be paused in the same command, or this error
        /// will be thrown.
        CannotPauseUnlessAllValueRulesArePaused = 8,
        /// When a mutate request tries to pause all the value rules in a value rule
        /// set, the value rule set must be paused, or this error will be thrown.
        ShouldPauseWhenAllValueRulesArePaused = 9,
        /// This value rule set is attached to a campaign that does not support value
        /// rules. Currently, campaign level value rule sets can only be created on
        /// Search, or Display campaigns.
        ValueRulesNotSupportedForCampaignType = 10,
        /// To add a value rule set that applies on Store Visits/Store Sales
        /// conversion action categories, the customer must have valid Store Visits/
        /// Store Sales conversion actions.
        IneligibleConversionActionCategories = 11,
        /// If NO_CONDITION is used as a dimension of a value rule set, it must be
        /// the only dimension.
        DimensionNoConditionUsedWithOtherDimensions = 12,
        /// Dimension NO_CONDITION can only be used by Store Visits/Store Sales value
        /// rule set.
        DimensionNoConditionNotAllowed = 13,
        /// Value rule sets defined on the specified conversion action categories are
        /// not supported. The list of conversion action categories must be an empty
        /// list, only STORE_VISIT, or only STORE_SALE.
        UnsupportedConversionActionCategories = 14,
    }
}
// Proto file describing country code errors.

/// Container for enum describing country code errors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CountryCodeErrorEnum {}
/// Nested message and enum types in `CountryCodeErrorEnum`.
pub mod country_code_error_enum {
    /// Enum describing country code errors.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum CountryCodeError {
        /// Enum unspecified.
        Unspecified = 0,
        /// The received error code is not known in this version.
        Unknown = 1,
        /// The country code is invalid.
        InvalidCountryCode = 2,
    }
}
// Proto file describing criterion errors.

/// Container for enum describing possible criterion errors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CriterionErrorEnum {}
/// Nested message and enum types in `CriterionErrorEnum`.
pub mod criterion_error_enum {
    /// Enum describing possible criterion errors.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum CriterionError {
        /// Enum unspecified.
        Unspecified = 0,
        /// The received error code is not known in this version.
        Unknown = 1,
        /// Concrete type of criterion is required for CREATE and UPDATE operations.
        ConcreteTypeRequired = 2,
        /// The category requested for exclusion is invalid.
        InvalidExcludedCategory = 3,
        /// Invalid keyword criteria text.
        InvalidKeywordText = 4,
        /// Keyword text should be less than 80 chars.
        KeywordTextTooLong = 5,
        /// Keyword text has too many words.
        KeywordHasTooManyWords = 6,
        /// Keyword text has invalid characters or symbols.
        KeywordHasInvalidChars = 7,
        /// Invalid placement URL.
        InvalidPlacementUrl = 8,
        /// Invalid user list criterion.
        InvalidUserList = 9,
        /// Invalid user interest criterion.
        InvalidUserInterest = 10,
        /// Placement URL has wrong format.
        InvalidFormatForPlacementUrl = 11,
        /// Placement URL is too long.
        PlacementUrlIsTooLong = 12,
        /// Indicates the URL contains an illegal character.
        PlacementUrlHasIllegalChar = 13,
        /// Indicates the URL contains multiple comma separated URLs.
        PlacementUrlHasMultipleSitesInLine = 14,
        /// Indicates the domain is blocked.
        PlacementIsNotAvailableForTargetingOrExclusion = 15,
        /// Invalid topic path.
        InvalidTopicPath = 16,
        /// The YouTube Channel Id is invalid.
        InvalidYoutubeChannelId = 17,
        /// The YouTube Video Id is invalid.
        InvalidYoutubeVideoId = 18,
        /// Indicates the placement is a YouTube vertical channel, which is no longer
        /// supported.
        YoutubeVerticalChannelDeprecated = 19,
        /// Indicates the placement is a YouTube demographic channel, which is no
        /// longer supported.
        YoutubeDemographicChannelDeprecated = 20,
        /// YouTube urls are not supported in Placement criterion. Use YouTubeChannel
        /// and YouTubeVideo criterion instead.
        YoutubeUrlUnsupported = 21,
        /// Criteria type can not be excluded by the customer, like AOL account type
        /// cannot target site type criteria.
        CannotExcludeCriteriaType = 22,
        /// Criteria type can not be targeted.
        CannotAddCriteriaType = 23,
        /// Not allowed to exclude similar user list.
        CannotExcludeSimilarUserList = 26,
        /// Not allowed to target a closed user list.
        CannotAddClosedUserList = 27,
        /// Not allowed to add display only UserLists to search only campaigns.
        CannotAddDisplayOnlyListsToSearchOnlyCampaigns = 28,
        /// Not allowed to add display only UserLists to search plus campaigns.
        CannotAddDisplayOnlyListsToSearchCampaigns = 29,
        /// Not allowed to add display only UserLists to shopping campaigns.
        CannotAddDisplayOnlyListsToShoppingCampaigns = 30,
        /// Not allowed to add User interests to search only campaigns.
        CannotAddUserInterestsToSearchCampaigns = 31,
        /// Not allowed to set bids for this criterion type in search campaigns
        CannotSetBidsOnCriterionTypeInSearchCampaigns = 32,
        /// Final URLs, URL Templates and CustomParameters cannot be set for the
        /// criterion types of Gender, AgeRange, UserList, Placement, MobileApp, and
        /// MobileAppCategory in search campaigns and shopping campaigns.
        CannotAddUrlsToCriterionTypeForCampaignType = 33,
        /// Invalid combined audience criterion.
        InvalidCombinedAudience = 122,
        /// Invalid custom affinity criterion.
        InvalidCustomAffinity = 96,
        /// Invalid custom intent criterion.
        InvalidCustomIntent = 97,
        /// Invalid custom audience criterion.
        InvalidCustomAudience = 121,
        /// IP address is not valid.
        InvalidIpAddress = 34,
        /// IP format is not valid.
        InvalidIpFormat = 35,
        /// Mobile application is not valid.
        InvalidMobileApp = 36,
        /// Mobile application category is not valid.
        InvalidMobileAppCategory = 37,
        /// The CriterionId does not exist or is of the incorrect type.
        InvalidCriterionId = 38,
        /// The Criterion is not allowed to be targeted.
        CannotTargetCriterion = 39,
        /// The criterion is not allowed to be targeted as it is deprecated.
        CannotTargetObsoleteCriterion = 40,
        /// The CriterionId is not valid for the type.
        CriterionIdAndTypeMismatch = 41,
        /// Distance for the radius for the proximity criterion is invalid.
        InvalidProximityRadius = 42,
        /// Units for the distance for the radius for the proximity criterion is
        /// invalid.
        InvalidProximityRadiusUnits = 43,
        /// Street address in the address is not valid.
        InvalidStreetaddressLength = 44,
        /// City name in the address is not valid.
        InvalidCitynameLength = 45,
        /// Region code in the address is not valid.
        InvalidRegioncodeLength = 46,
        /// Region name in the address is not valid.
        InvalidRegionnameLength = 47,
        /// Postal code in the address is not valid.
        InvalidPostalcodeLength = 48,
        /// Country code in the address is not valid.
        InvalidCountryCode = 49,
        /// Latitude for the GeoPoint is not valid.
        InvalidLatitude = 50,
        /// Longitude for the GeoPoint is not valid.
        InvalidLongitude = 51,
        /// The Proximity input is not valid. Both address and geoPoint cannot be
        /// null.
        ProximityGeopointAndAddressBothCannotBeNull = 52,
        /// The Proximity address cannot be geocoded to a valid lat/long.
        InvalidProximityAddress = 53,
        /// User domain name is not valid.
        InvalidUserDomainName = 54,
        /// Length of serialized criterion parameter exceeded size limit.
        CriterionParameterTooLong = 55,
        /// Time interval in the AdSchedule overlaps with another AdSchedule.
        AdScheduleTimeIntervalsOverlap = 56,
        /// AdSchedule time interval cannot span multiple days.
        AdScheduleIntervalCannotSpanMultipleDays = 57,
        /// AdSchedule time interval specified is invalid, endTime cannot be earlier
        /// than startTime.
        AdScheduleInvalidTimeInterval = 58,
        /// The number of AdSchedule entries in a day exceeds the limit.
        AdScheduleExceededIntervalsPerDayLimit = 59,
        /// CriteriaId does not match the interval of the AdSchedule specified.
        AdScheduleCriterionIdMismatchingFields = 60,
        /// Cannot set bid modifier for this criterion type.
        CannotBidModifyCriterionType = 61,
        /// Cannot bid modify criterion, since it is opted out of the campaign.
        CannotBidModifyCriterionCampaignOptedOut = 62,
        /// Cannot set bid modifier for a negative criterion.
        CannotBidModifyNegativeCriterion = 63,
        /// Bid Modifier already exists. Use SET operation to update.
        BidModifierAlreadyExists = 64,
        /// Feed Id is not allowed in these Location Groups.
        FeedIdNotAllowed = 65,
        /// The account may not use the requested criteria type. For example, some
        /// accounts are restricted to keywords only.
        AccountIneligibleForCriteriaType = 66,
        /// The requested criteria type cannot be used with campaign or ad group
        /// bidding strategy.
        CriteriaTypeInvalidForBiddingStrategy = 67,
        /// The Criterion is not allowed to be excluded.
        CannotExcludeCriterion = 68,
        /// The criterion is not allowed to be removed. For example, we cannot remove
        /// any of the device criterion.
        CannotRemoveCriterion = 69,
        /// Bidding categories do not form a valid path in the Shopping bidding
        /// category taxonomy.
        InvalidProductBiddingCategory = 76,
        /// ShoppingSetting must be added to the campaign before ProductScope
        /// criteria can be added.
        MissingShoppingSetting = 77,
        /// Matching function is invalid.
        InvalidMatchingFunction = 78,
        /// Filter parameters not allowed for location groups targeting.
        LocationFilterNotAllowed = 79,
        /// Feed not found, or the feed is not an enabled location feed.
        InvalidFeedForLocationFilter = 98,
        /// Given location filter parameter is invalid for location groups targeting.
        LocationFilterInvalid = 80,
        /// Cannot set geo target constants and feed item sets at the same time.
        CannotSetGeoTargetConstantsWithFeedItemSets = 123,
        /// The location group radius is in the range but not at the valid increment.
        InvalidLocationGroupRadius = 124,
        /// The location group radius unit is invalid.
        InvalidLocationGroupRadiusUnit = 125,
        /// Criteria type cannot be associated with a campaign and its ad group(s)
        /// simultaneously.
        CannotAttachCriteriaAtCampaignAndAdgroup = 81,
        /// Range represented by hotel length of stay's min nights and max nights
        /// overlaps with an existing criterion.
        HotelLengthOfStayOverlapsWithExistingCriterion = 82,
        /// Range represented by hotel advance booking window's min days and max days
        /// overlaps with an existing criterion.
        HotelAdvanceBookingWindowOverlapsWithExistingCriterion = 83,
        /// The field is not allowed to be set when the negative field is set to
        /// true, e.g. we don't allow bids in negative ad group or campaign criteria.
        FieldIncompatibleWithNegativeTargeting = 84,
        /// The combination of operand and operator in webpage condition is invalid.
        InvalidWebpageCondition = 85,
        /// The URL of webpage condition is invalid.
        InvalidWebpageConditionUrl = 86,
        /// The URL of webpage condition cannot be empty or contain white space.
        WebpageConditionUrlCannotBeEmpty = 87,
        /// The URL of webpage condition contains an unsupported protocol.
        WebpageConditionUrlUnsupportedProtocol = 88,
        /// The URL of webpage condition cannot be an IP address.
        WebpageConditionUrlCannotBeIpAddress = 89,
        /// The domain of the URL is not consistent with the domain in campaign
        /// setting.
        WebpageConditionUrlDomainNotConsistentWithCampaignSetting = 90,
        /// The URL of webpage condition cannot be a public suffix itself.
        WebpageConditionUrlCannotBePublicSuffix = 91,
        /// The URL of webpage condition has an invalid public suffix.
        WebpageConditionUrlInvalidPublicSuffix = 92,
        /// Value track parameter is not supported in webpage condition URL.
        WebpageConditionUrlValueTrackValueNotSupported = 93,
        /// Only one URL-EQUALS webpage condition is allowed in a webpage
        /// criterion and it cannot be combined with other conditions.
        WebpageCriterionUrlEqualsCanHaveOnlyOneCondition = 94,
        /// A webpage criterion cannot be added to a non-DSA ad group.
        WebpageCriterionNotSupportedOnNonDsaAdGroup = 95,
        /// Cannot add positive user list criteria in Smart Display campaigns.
        CannotTargetUserListForSmartDisplayCampaigns = 99,
        /// Cannot add positive placement criterion types in search campaigns.
        CannotTargetPlacementsForSearchCampaigns = 126,
        /// Listing scope contains too many dimension types.
        ListingScopeTooManyDimensionTypes = 100,
        /// Listing scope has too many IN operators.
        ListingScopeTooManyInOperators = 101,
        /// Listing scope contains IN operator on an unsupported dimension type.
        ListingScopeInOperatorNotSupported = 102,
        /// There are dimensions with duplicate dimension type.
        DuplicateListingDimensionType = 103,
        /// There are dimensions with duplicate dimension value.
        DuplicateListingDimensionValue = 104,
        /// Listing group SUBDIVISION nodes cannot have bids.
        CannotSetBidsOnListingGroupSubdivision = 105,
        /// Ad group is invalid due to the listing groups it contains.
        InvalidListingGroupHierarchy = 106,
        /// Listing group unit cannot have children.
        ListingGroupUnitCannotHaveChildren = 107,
        /// Subdivided listing groups must have an "others" case.
        ListingGroupSubdivisionRequiresOthersCase = 108,
        /// Dimension type of listing group must be the same as that of its siblings.
        ListingGroupRequiresSameDimensionTypeAsSiblings = 109,
        /// Listing group cannot be added to the ad group because it already exists.
        ListingGroupAlreadyExists = 110,
        /// Listing group referenced in the operation was not found in the ad group.
        ListingGroupDoesNotExist = 111,
        /// Recursive removal failed because listing group subdivision is being
        /// created or modified in this request.
        ListingGroupCannotBeRemoved = 112,
        /// Listing group type is not allowed for specified ad group criterion type.
        InvalidListingGroupType = 113,
        /// Listing group in an ADD operation specifies a non temporary criterion id.
        ListingGroupAddMayOnlyUseTempId = 114,
        /// The combined length of dimension values of the Listing scope criterion
        /// is too long.
        ListingScopeTooLong = 115,
        /// Listing scope contains too many dimensions.
        ListingScopeTooManyDimensions = 116,
        /// The combined length of dimension values of the Listing group criterion is
        /// too long.
        ListingGroupTooLong = 117,
        /// Listing group tree is too deep.
        ListingGroupTreeTooDeep = 118,
        /// Listing dimension is invalid (e.g. dimension contains illegal value,
        /// dimension type is represented with wrong class, etc). Listing dimension
        /// value can not contain "==" or "&+".
        InvalidListingDimension = 119,
        /// Listing dimension type is either invalid for campaigns of this type or
        /// cannot be used in the current context. BIDDING_CATEGORY_Lx and
        /// PRODUCT_TYPE_Lx dimensions must be used in ascending order of their
        /// levels: L1, L2, L3, L4, L5... The levels must be specified sequentially
        /// and start from L1. Furthermore, an "others" Listing group cannot be
        /// subdivided with a dimension of the same type but of a higher level
        /// ("others" BIDDING_CATEGORY_L3 can be subdivided with BRAND but not with
        /// BIDDING_CATEGORY_L4).
        InvalidListingDimensionType = 120,
        /// Customer is not on allowlist for composite audience in display campaigns.
        AdvertiserNotOnAllowlistForCombinedAudienceOnDisplay = 127,
        /// Cannot target on a removed combined audience.
        CannotTargetRemovedCombinedAudience = 128,
        /// Combined audience ID is invalid.
        InvalidCombinedAudienceId = 129,
        /// Can not target removed combined audience.
        CannotTargetRemovedCustomAudience = 130,
        /// Range represented by hotel check-in date's start date and end date
        /// overlaps with an existing criterion.
        HotelCheckInDateRangeOverlapsWithExistingCriterion = 131,
        /// Start date is earlier than earliest allowed value of yesterday UTC.
        HotelCheckInDateRangeStartDateTooEarly = 132,
        /// End date later is than latest allowed day of 330 days in the future UTC.
        HotelCheckInDateRangeEndDateTooLate = 133,
        /// Start date is after end date.
        HotelCheckInDateRangeReversed = 134,
        /// Broad match modifier (BMM) keywords can no longer be created. Please see
        /// <https://ads-developers.googleblog.com/2021/06/broad-match-modifier-upcoming-changes.html.>
        BroadMatchModifierKeywordNotAllowed = 135,
        /// Only one audience is allowed in an asset group.
        OneAudienceAllowedPerAssetGroup = 136,
        /// Audience is not supported for the specified campaign type.
        AudienceNotEligibleForCampaignType = 137,
        /// Audience is not allowed to attach when use_audience_grouped bit is set to
        /// false.
        AudienceNotAllowedToAttachWhenAudienceGroupedSetToFalse = 138,
        /// Targeting is not allowed for Customer Match lists as per Customer Match
        /// policy. Please see
        /// <https://support.google.com/google-ads/answer/6299717.>
        CannotTargetCustomerMatchUserList = 139,
    }
}
// Proto file describing currency code errors.

/// Container for enum describing possible currency code errors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CurrencyCodeErrorEnum {}
/// Nested message and enum types in `CurrencyCodeErrorEnum`.
pub mod currency_code_error_enum {
    /// Enum describing possible currency code errors.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum CurrencyCodeError {
        /// Enum unspecified.
        Unspecified = 0,
        /// The received error code is not known in this version.
        Unknown = 1,
        /// The currency code is not supported.
        Unsupported = 2,
    }
}
// Proto file describing custom audience errors.

/// Container for enum describing possible custom audience errors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomAudienceErrorEnum {}
/// Nested message and enum types in `CustomAudienceErrorEnum`.
pub mod custom_audience_error_enum {
    /// Enum describing possible custom audience errors.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum CustomAudienceError {
        /// Enum unspecified.
        Unspecified = 0,
        /// The received error code is not known in this version.
        Unknown = 1,
        /// New name in the custom audience is duplicated ignoring cases.
        NameAlreadyUsed = 2,
        /// Cannot remove a custom audience while it's still being used as targeting.
        CannotRemoveWhileInUse = 3,
        /// Cannot update or remove a custom audience that is already removed.
        ResourceAlreadyRemoved = 4,
        /// The pair of [type, value] already exists in members.
        MemberTypeAndParameterAlreadyExisted = 5,
        /// Member type is invalid.
        InvalidMemberType = 6,
        /// Member type does not have associated value.
        MemberTypeAndValueDoesNotMatch = 7,
        /// Custom audience contains a member that violates policy.
        PolicyViolation = 8,
        /// Change in custom audience type is not allowed.
        InvalidTypeChange = 9,
    }
}
// Proto file describing CustomConversionGoal errors.

/// Container for enum describing possible custom conversion goal errors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomConversionGoalErrorEnum {}
/// Nested message and enum types in `CustomConversionGoalErrorEnum`.
pub mod custom_conversion_goal_error_enum {
    /// Enum describing possible custom conversion goal errors.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum CustomConversionGoalError {
        /// Enum unspecified.
        Unspecified = 0,
        /// The received error code is not known in this version.
        Unknown = 1,
        /// Cannot find a conversion action with the specified id.
        InvalidConversionAction = 2,
        /// The conversion action is not enabled so it cannot be included in a custom
        /// conversion goal.
        ConversionActionNotEnabled = 3,
        /// The custom conversion goal cannot be removed because it's linked to a
        /// campaign.
        CannotRemoveLinkedCustomConversionGoal = 4,
        /// Custom goal with the same name already exists.
        CustomGoalDuplicateName = 5,
        /// Custom goal with the same conversion action list already exists.
        DuplicateConversionActionList = 6,
    }
}
// Proto file describing custom interest errors.

/// Container for enum describing possible custom interest errors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomInterestErrorEnum {}
/// Nested message and enum types in `CustomInterestErrorEnum`.
pub mod custom_interest_error_enum {
    /// Enum describing possible custom interest errors.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum CustomInterestError {
        /// Enum unspecified.
        Unspecified = 0,
        /// The received error code is not known in this version.
        Unknown = 1,
        /// Duplicate custom interest name ignoring case.
        NameAlreadyUsed = 2,
        /// In the remove custom interest member operation, both member ID and
        /// pair [type, parameter] are not present.
        CustomInterestMemberIdAndTypeParameterNotPresentInRemove = 3,
        /// The pair of [type, parameter] does not exist.
        TypeAndParameterNotFound = 4,
        /// The pair of [type, parameter] already exists.
        TypeAndParameterAlreadyExisted = 5,
        /// Unsupported custom interest member type.
        InvalidCustomInterestMemberType = 6,
        /// Cannot remove a custom interest while it's still being targeted.
        CannotRemoveWhileInUse = 7,
        /// Cannot mutate custom interest type.
        CannotChangeType = 8,
    }
}
// Proto file describing CustomerClientLink errors.

/// Container for enum describing possible CustomeClientLink errors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomerClientLinkErrorEnum {}
/// Nested message and enum types in `CustomerClientLinkErrorEnum`.
pub mod customer_client_link_error_enum {
    /// Enum describing possible CustomerClientLink errors.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum CustomerClientLinkError {
        /// Enum unspecified.
        Unspecified = 0,
        /// The received error code is not known in this version.
        Unknown = 1,
        /// Trying to manage a client that already in being managed by customer.
        ClientAlreadyInvitedByThisManager = 2,
        /// Already managed by some other manager in the hierarchy.
        ClientAlreadyManagedInHierarchy = 3,
        /// Attempt to create a cycle in the hierarchy.
        CyclicLinkNotAllowed = 4,
        /// Managed accounts has the maximum number of linked accounts.
        CustomerHasTooManyAccounts = 5,
        /// Invitor has the maximum pending invitations.
        ClientHasTooManyInvitations = 6,
        /// Attempt to change hidden status of a link that is not active.
        CannotHideOrUnhideManagerAccounts = 7,
        /// Parent manager account has the maximum number of linked accounts.
        CustomerHasTooManyAccountsAtManager = 8,
        /// Client has too many managers.
        ClientHasTooManyManagers = 9,
    }
}
// Proto file describing customer customizer errors.

/// Container for enum describing possible customer customizer errors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomerCustomizerErrorEnum {}
/// Nested message and enum types in `CustomerCustomizerErrorEnum`.
pub mod customer_customizer_error_enum {
    /// Enum describing possible customer customizer errors.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum CustomerCustomizerError {
        /// Enum unspecified.
        Unspecified = 0,
        /// The received error code is not known in this version.
        Unknown = 1,
    }
}
/// Container for enum describing possible customer errors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomerErrorEnum {}
/// Nested message and enum types in `CustomerErrorEnum`.
pub mod customer_error_enum {
    /// Set of errors that are related to requests dealing with Customer.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum CustomerError {
        /// Enum unspecified.
        Unspecified = 0,
        /// The received error code is not known in this version.
        Unknown = 1,
        /// Customer status is not allowed to be changed from DRAFT and CLOSED.
        /// Currency code and at least one of country code and time zone needs to be
        /// set when status is changed to ENABLED.
        StatusChangeDisallowed = 2,
        /// CustomerService cannot get a customer that has not been fully set up.
        AccountNotSetUp = 3,
    }
}
// Proto file describing customer feed errors.

/// Container for enum describing possible customer feed errors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomerFeedErrorEnum {}
/// Nested message and enum types in `CustomerFeedErrorEnum`.
pub mod customer_feed_error_enum {
    /// Enum describing possible customer feed errors.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum CustomerFeedError {
        /// Enum unspecified.
        Unspecified = 0,
        /// The received error code is not known in this version.
        Unknown = 1,
        /// An active feed already exists for this customer and place holder type.
        FeedAlreadyExistsForPlaceholderType = 2,
        /// The specified feed is removed.
        CannotCreateForRemovedFeed = 3,
        /// The CustomerFeed already exists. Update should be used to modify the
        /// existing CustomerFeed.
        CannotCreateAlreadyExistingCustomerFeed = 4,
        /// Cannot update removed customer feed.
        CannotModifyRemovedCustomerFeed = 5,
        /// Invalid placeholder type.
        InvalidPlaceholderType = 6,
        /// Feed mapping for this placeholder type does not exist.
        MissingFeedmappingForPlaceholderType = 7,
        /// Placeholder not allowed at the account level.
        PlaceholderTypeNotAllowedOnCustomerFeed = 8,
    }
}
// Proto file describing CustomerManagerLink errors.

/// Container for enum describing possible CustomerManagerLink errors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomerManagerLinkErrorEnum {}
/// Nested message and enum types in `CustomerManagerLinkErrorEnum`.
pub mod customer_manager_link_error_enum {
    /// Enum describing possible CustomerManagerLink errors.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum CustomerManagerLinkError {
        /// Enum unspecified.
        Unspecified = 0,
        /// The received error code is not known in this version.
        Unknown = 1,
        /// No pending invitation.
        NoPendingInvite = 2,
        /// Attempt to operate on the same client more than once in the same call.
        SameClientMoreThanOncePerCall = 3,
        /// Manager account has the maximum number of linked accounts.
        ManagerHasMaxNumberOfLinkedAccounts = 4,
        /// If no active user on account it cannot be unlinked from its manager.
        CannotUnlinkAccountWithoutActiveUser = 5,
        /// Account should have at least one active owner on it before being
        /// unlinked.
        CannotRemoveLastClientAccountOwner = 6,
        /// Only account owners may change their permission role.
        CannotChangeRoleByNonAccountOwner = 7,
        /// When a client's link to its manager is not active, the link role cannot
        /// be changed.
        CannotChangeRoleForNonActiveLinkAccount = 8,
        /// Attempt to link a child to a parent that contains or will contain
        /// duplicate children.
        DuplicateChildFound = 9,
        /// The authorized customer is a test account. It can add no more than the
        /// allowed number of accounts
        TestAccountLinksTooManyChildAccounts = 10,
    }
}
// Proto file describing CustomerUserAccess errors.

/// Container for enum describing possible CustomerUserAccess errors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomerUserAccessErrorEnum {}
/// Nested message and enum types in `CustomerUserAccessErrorEnum`.
pub mod customer_user_access_error_enum {
    /// Enum describing possible customer user access errors.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum CustomerUserAccessError {
        /// Enum unspecified.
        Unspecified = 0,
        /// The received error code is not known in this version.
        Unknown = 1,
        /// There is no user associated with the user id specified.
        InvalidUserId = 2,
        /// Unable to remove the access between the user and customer.
        RemovalDisallowed = 3,
        /// Unable to add or update the access role as specified.
        DisallowedAccessRole = 4,
        /// The user can't remove itself from an active serving customer if it's the
        /// last admin user and the customer doesn't have any owner manager
        LastAdminUserOfServingCustomer = 5,
        /// Last admin user cannot be removed from a manager.
        LastAdminUserOfManager = 6,
    }
}
// Proto file describing customizer attribute errors.

/// Container for enum describing possible customizer attribute errors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomizerAttributeErrorEnum {}
/// Nested message and enum types in `CustomizerAttributeErrorEnum`.
pub mod customizer_attribute_error_enum {
    /// Enum describing possible customizer attribute errors.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum CustomizerAttributeError {
        /// Enum unspecified.
        Unspecified = 0,
        /// The received error code is not known in this version.
        Unknown = 1,
        /// CustomizerAttribute name matches that of another active
        /// CustomizerAttribute.
        DuplicateCustomizerAttributeName = 2,
    }
}
// Proto file describing database errors.

/// Container for enum describing possible database errors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DatabaseErrorEnum {}
/// Nested message and enum types in `DatabaseErrorEnum`.
pub mod database_error_enum {
    /// Enum describing possible database errors.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum DatabaseError {
        /// Enum unspecified.
        Unspecified = 0,
        /// The received error code is not known in this version.
        Unknown = 1,
        /// Multiple requests were attempting to modify the same resource at once.
        /// Please retry the request.
        ConcurrentModification = 2,
        /// The request conflicted with existing data. This error will usually be
        /// replaced with a more specific error if the request is retried.
        DataConstraintViolation = 3,
        /// The data written is too large. Please split the request into smaller
        /// requests.
        RequestTooLarge = 4,
    }
}
// Proto file describing date errors.

/// Container for enum describing possible date errors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DateErrorEnum {}
/// Nested message and enum types in `DateErrorEnum`.
pub mod date_error_enum {
    /// Enum describing possible date errors.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum DateError {
        /// Enum unspecified.
        Unspecified = 0,
        /// The received error code is not known in this version.
        Unknown = 1,
        /// Given field values do not correspond to a valid date.
        InvalidFieldValuesInDate = 2,
        /// Given field values do not correspond to a valid date time.
        InvalidFieldValuesInDateTime = 3,
        /// The string date's format should be yyyy-mm-dd.
        InvalidStringDate = 4,
        /// The string date time's format should be yyyy-mm-dd hh:mm:ss.ssssss.
        InvalidStringDateTimeMicros = 6,
        /// The string date time's format should be yyyy-mm-dd hh:mm:ss.
        InvalidStringDateTimeSeconds = 11,
        /// The string date time's format should be yyyy-mm-dd hh:mm:ss+|-hh:mm.
        InvalidStringDateTimeSecondsWithOffset = 12,
        /// Date is before allowed minimum.
        EarlierThanMinimumDate = 7,
        /// Date is after allowed maximum.
        LaterThanMaximumDate = 8,
        /// Date range bounds are not in order.
        DateRangeMinimumDateLaterThanMaximumDate = 9,
        /// Both dates in range are null.
        DateRangeMinimumAndMaximumDatesBothNull = 10,
    }
}
// Proto file describing date range errors.

/// Container for enum describing possible date range errors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DateRangeErrorEnum {}
/// Nested message and enum types in `DateRangeErrorEnum`.
pub mod date_range_error_enum {
    /// Enum describing possible date range errors.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum DateRangeError {
        /// Enum unspecified.
        Unspecified = 0,
        /// The received error code is not known in this version.
        Unknown = 1,
        /// Invalid date.
        InvalidDate = 2,
        /// The start date was after the end date.
        StartDateAfterEndDate = 3,
        /// Cannot set date to past time
        CannotSetDateToPast = 4,
        /// A date was used that is past the system "last" date.
        AfterMaximumAllowableDate = 5,
        /// Trying to change start date on a resource that has started.
        CannotModifyStartDateIfAlreadyStarted = 6,
    }
}
// Proto file describing distinct errors.

/// Container for enum describing possible distinct errors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DistinctErrorEnum {}
/// Nested message and enum types in `DistinctErrorEnum`.
pub mod distinct_error_enum {
    /// Enum describing possible distinct errors.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum DistinctError {
        /// Enum unspecified.
        Unspecified = 0,
        /// The received error code is not known in this version.
        Unknown = 1,
        /// Duplicate element.
        DuplicateElement = 2,
        /// Duplicate type.
        DuplicateType = 3,
    }
}
// Proto file describing enum errors.

/// Container for enum describing possible enum errors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EnumErrorEnum {}
/// Nested message and enum types in `EnumErrorEnum`.
pub mod enum_error_enum {
    /// Enum describing possible enum errors.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum EnumError {
        /// Enum unspecified.
        Unspecified = 0,
        /// The received error code is not known in this version.
        Unknown = 1,
        /// The enum value is not permitted.
        EnumValueNotPermitted = 3,
    }
}
// Proto file describing experiment arm errors.

/// Container for enum describing possible experiment arm error.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExperimentArmErrorEnum {}
/// Nested message and enum types in `ExperimentArmErrorEnum`.
pub mod experiment_arm_error_enum {
    /// Enum describing possible experiment arm errors.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ExperimentArmError {
        /// Enum unspecified.
        Unspecified = 0,
        /// The received error code is not known in this version.
        Unknown = 1,
        /// Number of experiment arms is above limit.
        ExperimentArmCountLimitExceeded = 2,
        /// Cannot add campaign with invalid status to the experiment arm.
        InvalidCampaignStatus = 3,
        /// Cannot add duplicate experiment arm name in one experiment.
        DuplicateExperimentArmName = 4,
        /// Cannot set campaigns of treatment experiment arm.
        CannotSetTreatmentArmCampaign = 5,
        /// Cannot edit campaign ids in trial arms in non SETUP experiment.
        CannotModifyCampaignIds = 6,
        /// Cannot modify the campaigns in the control arm
        /// if there is not a suffix set in the trial.
        CannotModifyCampaignWithoutSuffixSet = 7,
        /// Traffic split related settings (like traffic share bounds) can't be
        /// modified after the trial has started.
        CannotMutateTrafficSplitAfterStart = 8,
        /// Cannot use shared budget on experiment's control campaign.
        CannotAddCampaignWithSharedBudget = 9,
        /// Cannot use custom budget on experiment's control campaigns.
        CannotAddCampaignWithCustomBudget = 10,
        /// Cannot have enable_dynamic_assets turned on in experiment's campaigns.
        CannotAddCampaignsWithDynamicAssetsEnabled = 11,
        /// Cannot use campaign's advertising channel sub type in experiment.
        UnsupportedCampaignAdvertisingChannelSubType = 12,
        /// Experiment date range must be within base campaign's date range.
        CannotAddBaseCampaignWithDateRange = 13,
        /// Bidding strategy is not supported in experiments.
        BiddingStrategyNotSupportedInExperiments = 14,
        /// Traffic split is not supported for some channel types.
        TrafficSplitNotSupportedForChannelType = 15,
    }
}
// Proto file describing experiment errors.

/// Container for enum describing possible experiment error.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExperimentErrorEnum {}
/// Nested message and enum types in `ExperimentErrorEnum`.
pub mod experiment_error_enum {
    /// Enum describing possible experiment errors.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ExperimentError {
        /// Enum unspecified.
        Unspecified = 0,
        /// The received error code is not known in this version.
        Unknown = 1,
        /// The start date of an experiment cannot be set in the past.
        /// Please use a start date in the future.
        CannotSetStartDateInPast = 2,
        /// The end date of an experiment is before its start date.
        /// Please use an end date after the start date.
        EndDateBeforeStartDate = 3,
        /// The start date of an experiment is too far in the future.
        /// Please use a start date no more than 1 year in the future.
        StartDateTooFarInFuture = 4,
        /// The experiment has the same name as an existing active experiment.
        DuplicateExperimentName = 5,
        /// Experiments can only be modified when they are ENABLED.
        CannotModifyRemovedExperiment = 6,
        /// The start date of an experiment cannot be modified if the existing start
        /// date has already passed.
        StartDateAlreadyPassed = 7,
        /// The end date of an experiment cannot be set in the past.
        CannotSetEndDateInPast = 8,
        /// The status of an experiment cannot be set to REMOVED.
        CannotSetStatusToRemoved = 9,
        /// The end date of an expired experiment cannot be modified.
        CannotModifyPastEndDate = 10,
        /// The status is invalid.
        InvalidStatus = 11,
        /// Experiment arm contains campaigns with invalid advertising channel type.
        InvalidCampaignChannelType = 12,
        /// A pair of trials share members and have overlapping date ranges.
        OverlappingMembersAndDateRange = 13,
        /// Experiment arm contains invalid traffic split.
        InvalidTrialArmTrafficSplit = 14,
        /// Experiment contains trial arms with overlapping traffic split.
        TrafficSplitOverlapping = 15,
        /// The total traffic split of trial arms is not equal to 100.
        SumTrialArmTrafficUnequalsToTrialTrafficSplitDenominator = 16,
        /// Traffic split related settings (like traffic share bounds) can't be
        /// modified after the experiment has started.
        CannotModifyTrafficSplitAfterStart = 17,
        /// The experiment could not be found.
        ExperimentNotFound = 18,
        /// Experiment has not begun.
        ExperimentNotYetStarted = 19,
        /// The experiment cannot have more than one control arm.
        CannotHaveMultipleControlArms = 20,
        /// The experiment doesn't set in-design campaigns.
        InDesignCampaignsNotSet = 21,
        /// Clients must use the graduate action to graduate experiments and cannot
        /// set the status to GRADUATED directly.
        CannotSetStatusToGraduated = 22,
        /// Cannot use shared budget on base campaign when scheduling an experiment.
        CannotCreateExperimentCampaignWithSharedBudget = 23,
        /// Cannot use custom budget on base campaign when scheduling an experiment.
        CannotCreateExperimentCampaignWithCustomBudget = 24,
        /// Invalid status transition.
        StatusTransitionInvalid = 25,
    }
}
// Proto file describing extension feed item errors.

/// Container for enum describing possible extension feed item error.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExtensionFeedItemErrorEnum {}
/// Nested message and enum types in `ExtensionFeedItemErrorEnum`.
pub mod extension_feed_item_error_enum {
    /// Enum describing possible extension feed item errors.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ExtensionFeedItemError {
        /// Enum unspecified.
        Unspecified = 0,
        /// The received error code is not known in this version.
        Unknown = 1,
        /// Value is not within the accepted range.
        ValueOutOfRange = 2,
        /// Url list is too long.
        UrlListTooLong = 3,
        /// Cannot have a geo targeting restriction without having geo targeting.
        CannotHaveRestrictionOnEmptyGeoTargeting = 4,
        /// Cannot simultaneously set sitelink field with final urls.
        CannotSetWithFinalUrls = 5,
        /// Must set field with final urls.
        CannotSetWithoutFinalUrls = 6,
        /// Phone number for a call extension is invalid.
        InvalidPhoneNumber = 7,
        /// Phone number for a call extension is not supported for the given country
        /// code.
        PhoneNumberNotSupportedForCountry = 8,
        /// A carrier specific number in short format is not allowed for call
        /// extensions.
        CarrierSpecificShortNumberNotAllowed = 9,
        /// Premium rate numbers are not allowed for call extensions.
        PremiumRateNumberNotAllowed = 10,
        /// Phone number type for a call extension is not allowed.
        /// For example, personal number is not allowed for a call extension in
        /// most regions.
        DisallowedNumberType = 11,
        /// Phone number for a call extension does not meet domestic format
        /// requirements.
        InvalidDomesticPhoneNumberFormat = 12,
        /// Vanity phone numbers (i.e. those including letters) are not allowed for
        /// call extensions.
        VanityPhoneNumberNotAllowed = 13,
        /// Call conversion action provided for a call extension is invalid.
        InvalidCallConversionAction = 14,
        /// For a call extension, the customer is not on the allow-list for call
        /// tracking.
        CustomerNotOnAllowlistForCalltracking = 47,
        /// Call tracking is not supported for the given country for a call
        /// extension.
        CalltrackingNotSupportedForCountry = 16,
        /// Customer hasn't consented for call recording, which is required for
        /// creating/updating call feed items. Please see
        /// <https://support.google.com/google-ads/answer/7412639.>
        CustomerConsentForCallRecordingRequired = 17,
        /// App id provided for an app extension is invalid.
        InvalidAppId = 18,
        /// Quotation marks present in the review text for a review extension.
        QuotesInReviewExtensionSnippet = 19,
        /// Hyphen character present in the review text for a review extension.
        HyphensInReviewExtensionSnippet = 20,
        /// A denylisted review source name or url was provided for a review
        /// extension.
        ReviewExtensionSourceIneligible = 21,
        /// Review source name should not be found in the review text.
        SourceNameInReviewExtensionText = 22,
        /// Inconsistent currency codes.
        InconsistentCurrencyCodes = 23,
        /// Price extension cannot have duplicated headers.
        PriceExtensionHasDuplicatedHeaders = 24,
        /// Price item cannot have duplicated header and description.
        PriceItemHasDuplicatedHeaderAndDescription = 25,
        /// Price extension has too few items.
        PriceExtensionHasTooFewItems = 26,
        /// Price extension has too many items.
        PriceExtensionHasTooManyItems = 27,
        /// The input value is not currently supported.
        UnsupportedValue = 28,
        /// The input value is not currently supported in the selected language of an
        /// extension.
        UnsupportedValueInSelectedLanguage = 29,
        /// Unknown or unsupported device preference.
        InvalidDevicePreference = 30,
        /// Invalid feed item schedule end time (i.e., endHour = 24 and endMinute !=
        /// 0).
        InvalidScheduleEnd = 31,
        /// Date time zone does not match the account's time zone.
        DateTimeMustBeInAccountTimeZone = 32,
        /// Invalid structured snippet header.
        InvalidSnippetsHeader = 33,
        /// Cannot operate on removed feed item.
        CannotOperateOnRemovedFeedItem = 34,
        /// Phone number not supported when call tracking enabled for country.
        PhoneNumberNotSupportedWithCalltrackingForCountry = 35,
        /// Cannot set call_conversion_action while call_conversion_tracking_enabled
        /// is set to true.
        ConflictingCallConversionSettings = 36,
        /// The type of the input extension feed item doesn't match the existing
        /// extension feed item.
        ExtensionTypeMismatch = 37,
        /// The oneof field extension i.e. subtype of extension feed item is
        /// required.
        ExtensionSubtypeRequired = 38,
        /// The referenced feed item is not mapped to a supported extension type.
        ExtensionTypeUnsupported = 39,
        /// Cannot operate on a Feed with more than one active FeedMapping.
        CannotOperateOnFeedWithMultipleMappings = 40,
        /// Cannot operate on a Feed that has key attributes.
        CannotOperateOnFeedWithKeyAttributes = 41,
        /// Input price is not in a valid format.
        InvalidPriceFormat = 42,
        /// The promotion time is invalid.
        PromotionInvalidTime = 43,
        /// This field has too many decimal places specified.
        TooManyDecimalPlacesSpecified = 44,
        /// Concrete sub type of ExtensionFeedItem is required for this operation.
        ConcreteExtensionTypeRequired = 45,
        /// Feed item schedule end time must be after start time.
        ScheduleEndNotAfterStart = 46,
    }
}
// Proto file describing extension setting validation errors.

/// Container for enum describing validation errors of extension settings.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExtensionSettingErrorEnum {}
/// Nested message and enum types in `ExtensionSettingErrorEnum`.
pub mod extension_setting_error_enum {
    /// Enum describing possible extension setting errors.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ExtensionSettingError {
        /// Enum unspecified.
        Unspecified = 0,
        /// The received error code is not known in this version.
        Unknown = 1,
        /// A platform restriction was provided without input extensions or existing
        /// extensions.
        ExtensionsRequired = 2,
        /// The provided feed type does not correspond to the provided extensions.
        FeedTypeExtensionTypeMismatch = 3,
        /// The provided feed type cannot be used.
        InvalidFeedType = 4,
        /// The provided feed type cannot be used at the customer level.
        InvalidFeedTypeForCustomerExtensionSetting = 5,
        /// Cannot change a feed item field on a CREATE operation.
        CannotChangeFeedItemOnCreate = 6,
        /// Cannot update an extension that is not already in this setting.
        CannotUpdateNewlyCreatedExtension = 7,
        /// There is no existing AdGroupExtensionSetting for this type.
        NoExistingAdGroupExtensionSettingForType = 8,
        /// There is no existing CampaignExtensionSetting for this type.
        NoExistingCampaignExtensionSettingForType = 9,
        /// There is no existing CustomerExtensionSetting for this type.
        NoExistingCustomerExtensionSettingForType = 10,
        /// The AdGroupExtensionSetting already exists. UPDATE should be used to
        /// modify the existing AdGroupExtensionSetting.
        AdGroupExtensionSettingAlreadyExists = 11,
        /// The CampaignExtensionSetting already exists. UPDATE should be used to
        /// modify the existing CampaignExtensionSetting.
        CampaignExtensionSettingAlreadyExists = 12,
        /// The CustomerExtensionSetting already exists. UPDATE should be used to
        /// modify the existing CustomerExtensionSetting.
        CustomerExtensionSettingAlreadyExists = 13,
        /// An active ad group feed already exists for this place holder type.
        AdGroupFeedAlreadyExistsForPlaceholderType = 14,
        /// An active campaign feed already exists for this place holder type.
        CampaignFeedAlreadyExistsForPlaceholderType = 15,
        /// An active customer feed already exists for this place holder type.
        CustomerFeedAlreadyExistsForPlaceholderType = 16,
        /// Value is not within the accepted range.
        ValueOutOfRange = 17,
        /// Cannot simultaneously set specified field with final urls.
        CannotSetFieldWithFinalUrls = 18,
        /// Must set field with final urls.
        FinalUrlsNotSet = 19,
        /// Phone number for a call extension is invalid.
        InvalidPhoneNumber = 20,
        /// Phone number for a call extension is not supported for the given country
        /// code.
        PhoneNumberNotSupportedForCountry = 21,
        /// A carrier specific number in short format is not allowed for call
        /// extensions.
        CarrierSpecificShortNumberNotAllowed = 22,
        /// Premium rate numbers are not allowed for call extensions.
        PremiumRateNumberNotAllowed = 23,
        /// Phone number type for a call extension is not allowed.
        DisallowedNumberType = 24,
        /// Phone number for a call extension does not meet domestic format
        /// requirements.
        InvalidDomesticPhoneNumberFormat = 25,
        /// Vanity phone numbers (i.e. those including letters) are not allowed for
        /// call extensions.
        VanityPhoneNumberNotAllowed = 26,
        /// Country code provided for a call extension is invalid.
        InvalidCountryCode = 27,
        /// Call conversion type id provided for a call extension is invalid.
        InvalidCallConversionTypeId = 28,
        /// For a call extension, the customer is not on the allow-list for call
        /// tracking.
        CustomerNotInAllowlistForCalltracking = 69,
        /// Call tracking is not supported for the given country for a call
        /// extension.
        CalltrackingNotSupportedForCountry = 30,
        /// App id provided for an app extension is invalid.
        InvalidAppId = 31,
        /// Quotation marks present in the review text for a review extension.
        QuotesInReviewExtensionSnippet = 32,
        /// Hyphen character present in the review text for a review extension.
        HyphensInReviewExtensionSnippet = 33,
        /// A blocked review source name or url was provided for a review
        /// extension.
        ReviewExtensionSourceNotEligible = 34,
        /// Review source name should not be found in the review text.
        SourceNameInReviewExtensionText = 35,
        /// Field must be set.
        MissingField = 36,
        /// Inconsistent currency codes.
        InconsistentCurrencyCodes = 37,
        /// Price extension cannot have duplicated headers.
        PriceExtensionHasDuplicatedHeaders = 38,
        /// Price item cannot have duplicated header and description.
        PriceItemHasDuplicatedHeaderAndDescription = 39,
        /// Price extension has too few items
        PriceExtensionHasTooFewItems = 40,
        /// Price extension has too many items
        PriceExtensionHasTooManyItems = 41,
        /// The input value is not currently supported.
        UnsupportedValue = 42,
        /// Unknown or unsupported device preference.
        InvalidDevicePreference = 43,
        /// Invalid feed item schedule end time (i.e., endHour = 24 and
        /// endMinute != 0).
        InvalidScheduleEnd = 45,
        /// Date time zone does not match the account's time zone.
        DateTimeMustBeInAccountTimeZone = 47,
        /// Overlapping feed item schedule times (e.g., 7-10AM and 8-11AM) are not
        /// allowed.
        OverlappingSchedulesNotAllowed = 48,
        /// Feed item schedule end time must be after start time.
        ScheduleEndNotAfterStart = 49,
        /// There are too many feed item schedules per day.
        TooManySchedulesPerDay = 50,
        /// Cannot edit the same extension feed item more than once in the same
        /// request.
        DuplicateExtensionFeedItemEdit = 51,
        /// Invalid structured snippet header.
        InvalidSnippetsHeader = 52,
        /// Phone number with call tracking enabled is not supported for the
        /// specified country.
        PhoneNumberNotSupportedWithCalltrackingForCountry = 53,
        /// The targeted adgroup must belong to the targeted campaign.
        CampaignTargetingMismatch = 54,
        /// The feed used by the ExtensionSetting is removed and cannot be operated
        /// on. Remove the ExtensionSetting to allow a new one to be created using
        /// an active feed.
        CannotOperateOnRemovedFeed = 55,
        /// The ExtensionFeedItem type is required for this operation.
        ExtensionTypeRequired = 56,
        /// The matching function that links the extension feed to the customer,
        /// campaign, or ad group is not compatible with the ExtensionSetting
        /// services.
        IncompatibleUnderlyingMatchingFunction = 57,
        /// Start date must be before end date.
        StartDateAfterEndDate = 58,
        /// Input price is not in a valid format.
        InvalidPriceFormat = 59,
        /// The promotion time is invalid.
        PromotionInvalidTime = 60,
        /// Cannot set both percent discount and money discount fields.
        PromotionCannotSetPercentDiscountAndMoneyDiscount = 61,
        /// Cannot set both promotion code and orders over amount fields.
        PromotionCannotSetPromotionCodeAndOrdersOverAmount = 62,
        /// This field has too many decimal places specified.
        TooManyDecimalPlacesSpecified = 63,
        /// The language code is not valid.
        InvalidLanguageCode = 64,
        /// The language is not supported.
        UnsupportedLanguage = 65,
        /// Customer hasn't consented for call recording, which is required for
        /// adding/updating call extensions. Please see
        /// <https://support.google.com/google-ads/answer/7412639.>
        CustomerConsentForCallRecordingRequired = 66,
        /// The UPDATE operation does not specify any fields other than the resource
        /// name in the update mask.
        ExtensionSettingUpdateIsANoop = 67,
        /// The extension contains text which has been prohibited on policy grounds.
        DisallowedText = 68,
    }
}
// Proto file describing feed attribute reference errors.

/// Container for enum describing possible feed attribute reference errors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeedAttributeReferenceErrorEnum {}
/// Nested message and enum types in `FeedAttributeReferenceErrorEnum`.
pub mod feed_attribute_reference_error_enum {
    /// Enum describing possible feed attribute reference errors.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum FeedAttributeReferenceError {
        /// Enum unspecified.
        Unspecified = 0,
        /// The received error code is not known in this version.
        Unknown = 1,
        /// A feed referenced by ID has been removed.
        CannotReferenceRemovedFeed = 2,
        /// There is no enabled feed with the given name.
        InvalidFeedName = 3,
        /// There is no feed attribute in an enabled feed with the given name.
        InvalidFeedAttributeName = 4,
    }
}
// Proto file describing feed errors.

/// Container for enum describing possible feed errors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeedErrorEnum {}
/// Nested message and enum types in `FeedErrorEnum`.
pub mod feed_error_enum {
    /// Enum describing possible feed errors.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum FeedError {
        /// Enum unspecified.
        Unspecified = 0,
        /// The received error code is not known in this version.
        Unknown = 1,
        /// The names of the FeedAttributes must be unique.
        AttributeNamesNotUnique = 2,
        /// The attribute list must be an exact copy of the existing list if the
        /// attribute ID's are present.
        AttributesDoNotMatchExistingAttributes = 3,
        /// Cannot specify USER origin for a system generated feed.
        CannotSpecifyUserOriginForSystemFeed = 4,
        /// Cannot specify GOOGLE origin for a non-system generated feed.
        CannotSpecifyGoogleOriginForNonSystemFeed = 5,
        /// Cannot specify feed attributes for system feed.
        CannotSpecifyFeedAttributesForSystemFeed = 6,
        /// Cannot update FeedAttributes on feed with origin GOOGLE.
        CannotUpdateFeedAttributesWithOriginGoogle = 7,
        /// The given ID refers to a removed Feed. Removed Feeds are immutable.
        FeedRemoved = 8,
        /// The origin of the feed is not valid for the client.
        InvalidOriginValue = 9,
        /// A user can only create and modify feeds with USER origin.
        FeedOriginIsNotUser = 10,
        /// Invalid auth token for the given email.
        InvalidAuthTokenForEmail = 11,
        /// Invalid email specified.
        InvalidEmail = 12,
        /// Feed name matches that of another active Feed.
        DuplicateFeedName = 13,
        /// Name of feed is not allowed.
        InvalidFeedName = 14,
        /// Missing OAuthInfo.
        MissingOauthInfo = 15,
        /// New FeedAttributes must not affect the unique key.
        NewAttributeCannotBePartOfUniqueKey = 16,
        /// Too many FeedAttributes for a Feed.
        TooManyAttributes = 17,
        /// The business account is not valid.
        InvalidBusinessAccount = 18,
        /// Business account cannot access Business Profile.
        BusinessAccountCannotAccessLocationAccount = 19,
        /// Invalid chain ID provided for affiliate location feed.
        InvalidAffiliateChainId = 20,
        /// There is already a feed with the given system feed generation data.
        DuplicateSystemFeed = 21,
        /// An error occurred accessing Business Profile.
        GmbAccessError = 22,
        /// A customer cannot have both LOCATION and AFFILIATE_LOCATION feeds.
        CannotHaveLocationAndAffiliateLocationFeeds = 23,
        /// Feed-based extension is read-only for this extension type.
        LegacyExtensionTypeReadOnly = 24,
    }
}
// Proto file describing feed item errors.

/// Container for enum describing possible feed item errors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeedItemErrorEnum {}
/// Nested message and enum types in `FeedItemErrorEnum`.
pub mod feed_item_error_enum {
    /// Enum describing possible feed item errors.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum FeedItemError {
        /// Enum unspecified.
        Unspecified = 0,
        /// The received error code is not known in this version.
        Unknown = 1,
        /// Cannot convert the feed attribute value from string to its real type.
        CannotConvertAttributeValueFromString = 2,
        /// Cannot operate on removed feed item.
        CannotOperateOnRemovedFeedItem = 3,
        /// Date time zone does not match the account's time zone.
        DateTimeMustBeInAccountTimeZone = 4,
        /// Feed item with the key attributes could not be found.
        KeyAttributesNotFound = 5,
        /// Url feed attribute value is not valid.
        InvalidUrl = 6,
        /// Some key attributes are missing.
        MissingKeyAttributes = 7,
        /// Feed item has same key attributes as another feed item.
        KeyAttributesNotUnique = 8,
        /// Cannot modify key attributes on an existing feed item.
        CannotModifyKeyAttributeValue = 9,
        /// The feed attribute value is too large.
        SizeTooLargeForMultiValueAttribute = 10,
    }
}
// Proto file describing feed item set errors.

/// Container for enum describing possible feed item set errors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeedItemSetErrorEnum {}
/// Nested message and enum types in `FeedItemSetErrorEnum`.
pub mod feed_item_set_error_enum {
    /// Enum describing possible feed item set errors.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum FeedItemSetError {
        /// Enum unspecified.
        Unspecified = 0,
        /// The received error code is not known in this version.
        Unknown = 1,
        /// The given ID refers to a removed FeedItemSet.
        FeedItemSetRemoved = 2,
        /// The dynamic filter of a feed item set cannot be cleared on UPDATE if it
        /// exists. A set is either static or dynamic once added, and that cannot
        /// change.
        CannotClearDynamicFilter = 3,
        /// The dynamic filter of a feed item set cannot be created on UPDATE if it
        /// does not exist. A set is either static or dynamic once added, and that
        /// cannot change.
        CannotCreateDynamicFilter = 4,
        /// FeedItemSets can only be made for location or affiliate location feeds.
        InvalidFeedType = 5,
        /// FeedItemSets duplicate name. Name should be unique within an account.
        DuplicateName = 6,
        /// The feed type of the parent Feed is not compatible with the type of
        /// dynamic filter being set. For example, you can only set
        /// dynamic_location_set_filter for LOCATION feed item sets.
        WrongDynamicFilterForFeedType = 7,
        /// Chain ID specified for AffiliateLocationFeedData is invalid.
        DynamicFilterInvalidChainIds = 8,
    }
}
// Proto file describing feed item set link errors.

/// Container for enum describing possible feed item set link errors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeedItemSetLinkErrorEnum {}
/// Nested message and enum types in `FeedItemSetLinkErrorEnum`.
pub mod feed_item_set_link_error_enum {
    /// Enum describing possible feed item set link errors.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum FeedItemSetLinkError {
        /// Enum unspecified.
        Unspecified = 0,
        /// The received error code is not known in this version.
        Unknown = 1,
        /// The feed IDs of the FeedItemSet and FeedItem do not match. Only FeedItems
        /// in a given Feed can be linked to a FeedItemSet in that Feed.
        FeedIdMismatch = 2,
        /// Cannot add or remove links to a dynamic set.
        NoMutateAllowedForDynamicSet = 3,
    }
}
// Proto file describing feed item target errors.

/// Container for enum describing possible feed item target errors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeedItemTargetErrorEnum {}
/// Nested message and enum types in `FeedItemTargetErrorEnum`.
pub mod feed_item_target_error_enum {
    /// Enum describing possible feed item target errors.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum FeedItemTargetError {
        /// Enum unspecified.
        Unspecified = 0,
        /// The received error code is not known in this version.
        Unknown = 1,
        /// On CREATE, the FeedItemTarget must have a populated field in the oneof
        /// target.
        MustSetTargetOneofOnCreate = 2,
        /// The specified feed item target already exists, so it cannot be added.
        FeedItemTargetAlreadyExists = 3,
        /// The schedules for a given feed item cannot overlap.
        FeedItemSchedulesCannotOverlap = 4,
        /// Too many targets of a given type were added for a single feed item.
        TargetLimitExceededForGivenType = 5,
        /// Too many AdSchedules are enabled for the feed item for the given day.
        TooManySchedulesPerDay = 6,
        /// A feed item may either have an enabled campaign target or an enabled ad
        /// group target.
        CannotHaveEnabledCampaignAndEnabledAdGroupTargets = 7,
        /// Duplicate ad schedules aren't allowed.
        DuplicateAdSchedule = 8,
        /// Duplicate keywords aren't allowed.
        DuplicateKeyword = 9,
    }
}
// Proto file describing feed item validation errors.

/// Container for enum describing possible validation errors of a feed item.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeedItemValidationErrorEnum {}
/// Nested message and enum types in `FeedItemValidationErrorEnum`.
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
        /// Customer is not on the allow-list for call tracking.
        CustomerNotInAllowlistForCalltracking = 99,
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
        /// <https://support.google.com/google-ads/answer/7412639.>
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
        /// The specified asset ID does not exist.
        InvalidAssetId = 100,
        /// The asset type cannot be set for the field.
        IncompatibleAssetType = 101,
        /// The image has unexpected size.
        ImageErrorUnexpectedSize = 102,
        /// The image aspect ratio is not allowed.
        ImageErrorAspectRatioNotAllowed = 103,
        /// The image file is too large.
        ImageErrorFileTooLarge = 104,
        /// The image format is unsupported.
        ImageErrorFormatNotAllowed = 105,
        /// Image violates constraints without more details.
        ImageErrorConstraintsViolated = 106,
        /// An error occurred when validating image.
        ImageErrorServerError = 107,
    }
}
// Proto file describing feed item errors.

/// Container for enum describing possible feed item errors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeedMappingErrorEnum {}
/// Nested message and enum types in `FeedMappingErrorEnum`.
pub mod feed_mapping_error_enum {
    /// Enum describing possible feed item errors.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum FeedMappingError {
        /// Enum unspecified.
        Unspecified = 0,
        /// The received error code is not known in this version.
        Unknown = 1,
        /// The given placeholder field does not exist.
        InvalidPlaceholderField = 2,
        /// The given criterion field does not exist.
        InvalidCriterionField = 3,
        /// The given placeholder type does not exist.
        InvalidPlaceholderType = 4,
        /// The given criterion type does not exist.
        InvalidCriterionType = 5,
        /// A feed mapping must contain at least one attribute field mapping.
        NoAttributeFieldMappings = 7,
        /// The type of the feed attribute referenced in the attribute field mapping
        /// must match the type of the placeholder field.
        FeedAttributeTypeMismatch = 8,
        /// A feed mapping for a system generated feed cannot be operated on.
        CannotOperateOnMappingsForSystemGeneratedFeed = 9,
        /// Only one feed mapping for a placeholder type is allowed per feed or
        /// customer (depending on the placeholder type).
        MultipleMappingsForPlaceholderType = 10,
        /// Only one feed mapping for a criterion type is allowed per customer.
        MultipleMappingsForCriterionType = 11,
        /// Only one feed attribute mapping for a placeholder field is allowed
        /// (depending on the placeholder type).
        MultipleMappingsForPlaceholderField = 12,
        /// Only one feed attribute mapping for a criterion field is allowed
        /// (depending on the criterion type).
        MultipleMappingsForCriterionField = 13,
        /// This feed mapping may not contain any explicit attribute field mappings.
        UnexpectedAttributeFieldMappings = 14,
        /// Location placeholder feed mappings can only be created for Places feeds.
        LocationPlaceholderOnlyForPlacesFeeds = 15,
        /// Mappings for typed feeds cannot be modified.
        CannotModifyMappingsForTypedFeed = 16,
        /// The given placeholder type can only be mapped to system generated feeds.
        InvalidPlaceholderTypeForNonSystemGeneratedFeed = 17,
        /// The given placeholder type cannot be mapped to a system generated feed
        /// with the given type.
        InvalidPlaceholderTypeForSystemGeneratedFeedType = 18,
        /// The "field" oneof was not set in an AttributeFieldMapping.
        AttributeFieldMappingMissingField = 19,
    }
}
// Proto file describing field errors.

/// Container for enum describing possible field errors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FieldErrorEnum {}
/// Nested message and enum types in `FieldErrorEnum`.
pub mod field_error_enum {
    /// Enum describing possible field errors.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum FieldError {
        /// Enum unspecified.
        Unspecified = 0,
        /// The received error code is not known in this version.
        Unknown = 1,
        /// The required field was not present.
        Required = 2,
        /// The field attempted to be mutated is immutable.
        ImmutableField = 3,
        /// The field's value is invalid.
        InvalidValue = 4,
        /// The field cannot be set.
        ValueMustBeUnset = 5,
        /// The required repeated field was empty.
        RequiredNonemptyList = 6,
        /// The field cannot be cleared.
        FieldCannotBeCleared = 7,
        /// The field's value is on a deny-list for this field.
        BlockedValue = 9,
    }
}
// Proto file describing field mask errors.

/// Container for enum describing possible field mask errors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FieldMaskErrorEnum {}
/// Nested message and enum types in `FieldMaskErrorEnum`.
pub mod field_mask_error_enum {
    /// Enum describing possible field mask errors.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum FieldMaskError {
        /// Enum unspecified.
        Unspecified = 0,
        /// The received error code is not known in this version.
        Unknown = 1,
        /// The field mask must be provided for update operations.
        FieldMaskMissing = 5,
        /// The field mask must be empty for create and remove operations.
        FieldMaskNotAllowed = 4,
        /// The field mask contained an invalid field.
        FieldNotFound = 2,
        /// The field mask updated a field with subfields. Fields with subfields may
        /// be cleared, but not updated. To fix this, the field mask should select
        /// all the subfields of the invalid field.
        FieldHasSubfields = 3,
    }
}
// Proto file describing function errors.

/// Container for enum describing possible function errors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FunctionErrorEnum {}
/// Nested message and enum types in `FunctionErrorEnum`.
pub mod function_error_enum {
    /// Enum describing possible function errors.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum FunctionError {
        /// Enum unspecified.
        Unspecified = 0,
        /// The received error code is not known in this version.
        Unknown = 1,
        /// The format of the function is not recognized as a supported function
        /// format.
        InvalidFunctionFormat = 2,
        /// Operand data types do not match.
        DataTypeMismatch = 3,
        /// The operands cannot be used together in a conjunction.
        InvalidConjunctionOperands = 4,
        /// Invalid numer of Operands.
        InvalidNumberOfOperands = 5,
        /// Operand Type not supported.
        InvalidOperandType = 6,
        /// Operator not supported.
        InvalidOperator = 7,
        /// Request context type not supported.
        InvalidRequestContextType = 8,
        /// The matching function is not allowed for call placeholders
        InvalidFunctionForCallPlaceholder = 9,
        /// The matching function is not allowed for the specified placeholder
        InvalidFunctionForPlaceholder = 10,
        /// Invalid operand.
        InvalidOperand = 11,
        /// Missing value for the constant operand.
        MissingConstantOperandValue = 12,
        /// The value of the constant operand is invalid.
        InvalidConstantOperandValue = 13,
        /// Invalid function nesting.
        InvalidNesting = 14,
        /// The Feed ID was different from another Feed ID in the same function.
        MultipleFeedIdsNotSupported = 15,
        /// The matching function is invalid for use with a feed with a fixed schema.
        InvalidFunctionForFeedWithFixedSchema = 16,
        /// Invalid attribute name.
        InvalidAttributeName = 17,
    }
}
// Proto file describing function parsing errors.

/// Container for enum describing possible function parsing errors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FunctionParsingErrorEnum {}
/// Nested message and enum types in `FunctionParsingErrorEnum`.
pub mod function_parsing_error_enum {
    /// Enum describing possible function parsing errors.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum FunctionParsingError {
        /// Enum unspecified.
        Unspecified = 0,
        /// The received error code is not known in this version.
        Unknown = 1,
        /// Unexpected end of function string.
        NoMoreInput = 2,
        /// Could not find an expected character.
        ExpectedCharacter = 3,
        /// Unexpected separator character.
        UnexpectedSeparator = 4,
        /// Unmatched left bracket or parenthesis.
        UnmatchedLeftBracket = 5,
        /// Unmatched right bracket or parenthesis.
        UnmatchedRightBracket = 6,
        /// Functions are nested too deeply.
        TooManyNestedFunctions = 7,
        /// Missing right-hand-side operand.
        MissingRightHandOperand = 8,
        /// Invalid operator/function name.
        InvalidOperatorName = 9,
        /// Feed attribute operand's argument is not an integer.
        FeedAttributeOperandArgumentNotInteger = 10,
        /// Missing function operands.
        NoOperands = 11,
        /// Function had too many operands.
        TooManyOperands = 12,
    }
}
/// Container for enum describing possible geo target constant suggestion errors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GeoTargetConstantSuggestionErrorEnum {}
/// Nested message and enum types in `GeoTargetConstantSuggestionErrorEnum`.
pub mod geo_target_constant_suggestion_error_enum {
    /// Enum describing possible geo target constant suggestion errors.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum GeoTargetConstantSuggestionError {
        /// Enum unspecified.
        Unspecified = 0,
        /// The received error code is not known in this version.
        Unknown = 1,
        /// A location name cannot be greater than 300 characters.
        LocationNameSizeLimit = 2,
        /// At most 25 location names can be specified in a SuggestGeoTargetConstants
        /// method.
        LocationNameLimit = 3,
        /// The country code is invalid.
        InvalidCountryCode = 4,
        /// Geo target constant resource names or location names must be provided in
        /// the request.
        RequestParametersUnset = 5,
    }
}
// Proto file describing header errors.

/// Container for enum describing possible header errors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HeaderErrorEnum {}
/// Nested message and enum types in `HeaderErrorEnum`.
pub mod header_error_enum {
    /// Enum describing possible header errors.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum HeaderError {
        /// Enum unspecified.
        Unspecified = 0,
        /// The received error code is not known in this version.
        Unknown = 1,
        /// The login customer ID could not be validated.
        InvalidLoginCustomerId = 3,
        /// The linked customer ID could not be validated.
        InvalidLinkedCustomerId = 7,
    }
}
// Proto file describing id errors.

/// Container for enum describing possible id errors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IdErrorEnum {}
/// Nested message and enum types in `IdErrorEnum`.
pub mod id_error_enum {
    /// Enum describing possible id errors.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum IdError {
        /// Enum unspecified.
        Unspecified = 0,
        /// The received error code is not known in this version.
        Unknown = 1,
        /// Id not found
        NotFound = 2,
    }
}
// Proto file describing image errors.

/// Container for enum describing possible image errors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImageErrorEnum {}
/// Nested message and enum types in `ImageErrorEnum`.
pub mod image_error_enum {
    /// Enum describing possible image errors.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ImageError {
        /// Enum unspecified.
        Unspecified = 0,
        /// The received error code is not known in this version.
        Unknown = 1,
        /// The image is not valid.
        InvalidImage = 2,
        /// The image could not be stored.
        StorageError = 3,
        /// There was a problem with the request.
        BadRequest = 4,
        /// The image is not of legal dimensions.
        UnexpectedSize = 5,
        /// Animated image are not permitted.
        AnimatedNotAllowed = 6,
        /// Animation is too long.
        AnimationTooLong = 7,
        /// There was an error on the server.
        ServerError = 8,
        /// Image cannot be in CMYK color format.
        CmykJpegNotAllowed = 9,
        /// Flash images are not permitted.
        FlashNotAllowed = 10,
        /// Flash images must support clickTag.
        FlashWithoutClicktag = 11,
        /// A flash error has occurred after fixing the click tag.
        FlashErrorAfterFixingClickTag = 12,
        /// Unacceptable visual effects.
        AnimatedVisualEffect = 13,
        /// There was a problem with the flash image.
        FlashError = 14,
        /// Incorrect image layout.
        LayoutProblem = 15,
        /// There was a problem reading the image file.
        ProblemReadingImageFile = 16,
        /// There was an error storing the image.
        ErrorStoringImage = 17,
        /// The aspect ratio of the image is not allowed.
        AspectRatioNotAllowed = 18,
        /// Flash cannot have network objects.
        FlashHasNetworkObjects = 19,
        /// Flash cannot have network methods.
        FlashHasNetworkMethods = 20,
        /// Flash cannot have a Url.
        FlashHasUrl = 21,
        /// Flash cannot use mouse tracking.
        FlashHasMouseTracking = 22,
        /// Flash cannot have a random number.
        FlashHasRandomNum = 23,
        /// Ad click target cannot be '_self'.
        FlashSelfTargets = 24,
        /// GetUrl method should only use '_blank'.
        FlashBadGeturlTarget = 25,
        /// Flash version is not supported.
        FlashVersionNotSupported = 26,
        /// Flash movies need to have hard coded click URL or clickTAG
        FlashWithoutHardCodedClickUrl = 27,
        /// Uploaded flash file is corrupted.
        InvalidFlashFile = 28,
        /// Uploaded flash file can be parsed, but the click tag can not be fixed
        /// properly.
        FailedToFixClickTagInFlash = 29,
        /// Flash movie accesses network resources
        FlashAccessesNetworkResources = 30,
        /// Flash movie attempts to call external javascript code
        FlashExternalJsCall = 31,
        /// Flash movie attempts to call flash system commands
        FlashExternalFsCall = 32,
        /// Image file is too large.
        FileTooLarge = 33,
        /// Image data is too large.
        ImageDataTooLarge = 34,
        /// Error while processing the image.
        ImageProcessingError = 35,
        /// Image is too small.
        ImageTooSmall = 36,
        /// Input was invalid.
        InvalidInput = 37,
        /// There was a problem reading the image file.
        ProblemReadingFile = 38,
        /// Image constraints are violated, but details like ASPECT_RATIO_NOT_ALLOWED
        /// can't be provided. This happens when asset spec contains more than one
        /// constraint and different criteria of different constraints are violated.
        ImageConstraintsViolated = 39,
        /// Image format is not allowed.
        FormatNotAllowed = 40,
    }
}
// Proto file describing internal errors.

/// Container for enum describing possible internal errors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InternalErrorEnum {}
/// Nested message and enum types in `InternalErrorEnum`.
pub mod internal_error_enum {
    /// Enum describing possible internal errors.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum InternalError {
        /// Enum unspecified.
        Unspecified = 0,
        /// The received error code is not known in this version.
        Unknown = 1,
        /// Google Ads API encountered unexpected internal error.
        InternalError = 2,
        /// The intended error code doesn't exist in specified API version. It will
        /// be released in a future API version.
        ErrorCodeNotPublished = 3,
        /// Google Ads API encountered an unexpected transient error. The user
        /// should retry their request in these cases.
        TransientError = 4,
        /// The request took longer than a deadline.
        DeadlineExceeded = 5,
    }
}
// Proto file describing invoice errors.

/// Container for enum describing possible invoice errors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InvoiceErrorEnum {}
/// Nested message and enum types in `InvoiceErrorEnum`.
pub mod invoice_error_enum {
    /// Enum describing possible invoice errors.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum InvoiceError {
        /// Enum unspecified.
        Unspecified = 0,
        /// The received error code is not known in this version.
        Unknown = 1,
        /// Cannot request invoices issued before 2019-01-01.
        YearMonthTooOld = 2,
        /// Cannot request invoices for customer who doesn't receive invoices.
        NotInvoicedCustomer = 3,
        /// Cannot request invoices for a non approved billing setup.
        BillingSetupNotApproved = 4,
        /// Cannot request invoices for a billing setup that is not on monthly
        /// invoicing.
        BillingSetupNotOnMonthlyInvoicing = 5,
        /// Cannot request invoices for a non serving customer.
        NonServingCustomer = 6,
    }
}
// Proto file describing errors from applying a keyword plan ad group.

/// Container for enum describing possible errors from applying a keyword plan
/// ad group.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeywordPlanAdGroupErrorEnum {}
/// Nested message and enum types in `KeywordPlanAdGroupErrorEnum`.
pub mod keyword_plan_ad_group_error_enum {
    /// Enum describing possible errors from applying a keyword plan ad group.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum KeywordPlanAdGroupError {
        /// Enum unspecified.
        Unspecified = 0,
        /// The received error code is not known in this version.
        Unknown = 1,
        /// The keyword plan ad group name is missing, empty, longer than allowed
        /// limit or contains invalid chars.
        InvalidName = 2,
        /// The keyword plan ad group name is duplicate to an existing keyword plan
        /// AdGroup name or other keyword plan AdGroup name in the request.
        DuplicateName = 3,
    }
}
// Proto file describing errors from applying a keyword plan ad group keyword or
// keyword plan campaign keyword.

/// Container for enum describing possible errors from applying an ad group
/// keyword or a campaign keyword from a keyword plan.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeywordPlanAdGroupKeywordErrorEnum {}
/// Nested message and enum types in `KeywordPlanAdGroupKeywordErrorEnum`.
pub mod keyword_plan_ad_group_keyword_error_enum {
    /// Enum describing possible errors from applying a keyword plan ad group
    /// keyword or keyword plan campaign keyword.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum KeywordPlanAdGroupKeywordError {
        /// Enum unspecified.
        Unspecified = 0,
        /// The received error code is not known in this version.
        Unknown = 1,
        /// A keyword or negative keyword has invalid match type.
        InvalidKeywordMatchType = 2,
        /// A keyword or negative keyword with same text and match type already
        /// exists.
        DuplicateKeyword = 3,
        /// Keyword or negative keyword text exceeds the allowed limit.
        KeywordTextTooLong = 4,
        /// Keyword or negative keyword text has invalid characters or symbols.
        KeywordHasInvalidChars = 5,
        /// Keyword or negative keyword text has too many words.
        KeywordHasTooManyWords = 6,
        /// Keyword or negative keyword has invalid text.
        InvalidKeywordText = 7,
        /// Cpc Bid set for negative keyword.
        NegativeKeywordHasCpcBid = 8,
        /// New broad match modifier (BMM) KpAdGroupKeywords are not allowed.
        NewBmmKeywordsNotAllowed = 9,
    }
}
// Proto file describing errors from applying a keyword plan campaign.

/// Container for enum describing possible errors from applying a keyword plan
/// campaign.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeywordPlanCampaignErrorEnum {}
/// Nested message and enum types in `KeywordPlanCampaignErrorEnum`.
pub mod keyword_plan_campaign_error_enum {
    /// Enum describing possible errors from applying a keyword plan campaign.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum KeywordPlanCampaignError {
        /// Enum unspecified.
        Unspecified = 0,
        /// The received error code is not known in this version.
        Unknown = 1,
        /// A keyword plan campaign name is missing, empty, longer than allowed limit
        /// or contains invalid chars.
        InvalidName = 2,
        /// A keyword plan campaign contains one or more untargetable languages.
        InvalidLanguages = 3,
        /// A keyword plan campaign contains one or more invalid geo targets.
        InvalidGeos = 4,
        /// The keyword plan campaign name is duplicate to an existing keyword plan
        /// campaign name or other keyword plan campaign name in the request.
        DuplicateName = 5,
        /// The number of geo targets in the keyword plan campaign exceeds limits.
        MaxGeosExceeded = 6,
        /// The number of languages in the keyword plan campaign exceeds limits.
        MaxLanguagesExceeded = 7,
    }
}
// Proto file describing errors from applying a keyword plan campaign keyword.

/// Container for enum describing possible errors from applying a keyword plan
/// campaign keyword.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeywordPlanCampaignKeywordErrorEnum {}
/// Nested message and enum types in `KeywordPlanCampaignKeywordErrorEnum`.
pub mod keyword_plan_campaign_keyword_error_enum {
    /// Enum describing possible errors from applying a keyword plan campaign
    /// keyword.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum KeywordPlanCampaignKeywordError {
        /// Enum unspecified.
        Unspecified = 0,
        /// The received error code is not known in this version.
        Unknown = 1,
        /// Keyword plan campaign keyword is positive.
        CampaignKeywordIsPositive = 8,
    }
}
// Proto file describing errors from applying keyword plan resources (keyword
// plan, keyword plan campaign, keyword plan ad group or keyword plan keyword)
// or KeywordPlanService RPC.

/// Container for enum describing possible errors from applying a keyword plan
/// resource (keyword plan, keyword plan campaign, keyword plan ad group or
/// keyword plan keyword) or KeywordPlanService RPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeywordPlanErrorEnum {}
/// Nested message and enum types in `KeywordPlanErrorEnum`.
pub mod keyword_plan_error_enum {
    /// Enum describing possible errors from applying a keyword plan.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum KeywordPlanError {
        /// Enum unspecified.
        Unspecified = 0,
        /// The received error code is not known in this version.
        Unknown = 1,
        /// The plan's bid multiplier value is outside the valid range.
        BidMultiplierOutOfRange = 2,
        /// The plan's bid value is too high.
        BidTooHigh = 3,
        /// The plan's bid value is too low.
        BidTooLow = 4,
        /// The plan's cpc bid is not a multiple of the minimum billable unit.
        BidTooManyFractionalDigits = 5,
        /// The plan's daily budget value is too low.
        DailyBudgetTooLow = 6,
        /// The plan's daily budget is not a multiple of the minimum billable unit.
        DailyBudgetTooManyFractionalDigits = 7,
        /// The input has an invalid value.
        InvalidValue = 8,
        /// The plan has no keyword.
        KeywordPlanHasNoKeywords = 9,
        /// The plan is not enabled and API cannot provide mutation, forecast or
        /// stats.
        KeywordPlanNotEnabled = 10,
        /// The requested plan cannot be found for providing forecast or stats.
        KeywordPlanNotFound = 11,
        /// The plan is missing a cpc bid.
        MissingBid = 13,
        /// The plan is missing required forecast_period field.
        MissingForecastPeriod = 14,
        /// The plan's forecast_period has invalid forecast date range.
        InvalidForecastDateRange = 15,
        /// The plan's name is invalid.
        InvalidName = 16,
    }
}
// Proto file describing errors from KeywordPlanIdeaService.

/// Container for enum describing possible errors from KeywordPlanIdeaService.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeywordPlanIdeaErrorEnum {}
/// Nested message and enum types in `KeywordPlanIdeaErrorEnum`.
pub mod keyword_plan_idea_error_enum {
    /// Enum describing possible errors from KeywordPlanIdeaService.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum KeywordPlanIdeaError {
        /// Enum unspecified.
        Unspecified = 0,
        /// The received error code is not known in this version.
        Unknown = 1,
        /// Error when crawling the input URL.
        UrlCrawlError = 2,
        /// The input has an invalid value.
        InvalidValue = 3,
    }
}
// Proto file describing label errors.

/// Container for enum describing possible label errors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LabelErrorEnum {}
/// Nested message and enum types in `LabelErrorEnum`.
pub mod label_error_enum {
    /// Enum describing possible label errors.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum LabelError {
        /// Enum unspecified.
        Unspecified = 0,
        /// The received error code is not known in this version.
        Unknown = 1,
        /// An inactive label cannot be applied.
        CannotApplyInactiveLabel = 2,
        /// A label cannot be applied to a disabled ad group criterion.
        CannotApplyLabelToDisabledAdGroupCriterion = 3,
        /// A label cannot be applied to a negative ad group criterion.
        CannotApplyLabelToNegativeAdGroupCriterion = 4,
        /// Cannot apply more than 50 labels per resource.
        ExceededLabelLimitPerType = 5,
        /// Labels from a manager account cannot be applied to campaign, ad group,
        /// ad group ad, or ad group criterion resources.
        InvalidResourceForManagerLabel = 6,
        /// Label names must be unique.
        DuplicateName = 7,
        /// Label names cannot be empty.
        InvalidLabelName = 8,
        /// Labels cannot be applied to a draft.
        CannotAttachLabelToDraft = 9,
        /// Labels not from a manager account cannot be applied to the customer
        /// resource.
        CannotAttachNonManagerLabelToCustomer = 10,
    }
}
// Proto file describing language code errors.

/// Container for enum describing language code errors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LanguageCodeErrorEnum {}
/// Nested message and enum types in `LanguageCodeErrorEnum`.
pub mod language_code_error_enum {
    /// Enum describing language code errors.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum LanguageCodeError {
        /// Enum unspecified.
        Unspecified = 0,
        /// The received error code is not known in this version.
        Unknown = 1,
        /// The input language code is not recognized.
        LanguageCodeNotFound = 2,
        /// The language code is not supported.
        InvalidLanguageCode = 3,
    }
}
// Proto file describing list operation errors.

/// Container for enum describing possible list operation errors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListOperationErrorEnum {}
/// Nested message and enum types in `ListOperationErrorEnum`.
pub mod list_operation_error_enum {
    /// Enum describing possible list operation errors.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ListOperationError {
        /// Enum unspecified.
        Unspecified = 0,
        /// The received error code is not known in this version.
        Unknown = 1,
        /// Field required in value is missing.
        RequiredFieldMissing = 7,
        /// Duplicate or identical value is sent in multiple list operations.
        DuplicateValues = 8,
    }
}
// Proto file describing ManagerLink errors.

/// Container for enum describing possible ManagerLink errors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ManagerLinkErrorEnum {}
/// Nested message and enum types in `ManagerLinkErrorEnum`.
pub mod manager_link_error_enum {
    /// Enum describing possible ManagerLink errors.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ManagerLinkError {
        /// Enum unspecified.
        Unspecified = 0,
        /// The received error code is not known in this version.
        Unknown = 1,
        /// The manager and client have incompatible account types.
        AccountsNotCompatibleForLinking = 2,
        /// Client is already linked to too many managers.
        TooManyManagers = 3,
        /// Manager has too many pending invitations.
        TooManyInvites = 4,
        /// Client is already invited by this manager.
        AlreadyInvitedByThisManager = 5,
        /// The client is already managed by this manager.
        AlreadyManagedByThisManager = 6,
        /// Client is already managed in hierarchy.
        AlreadyManagedInHierarchy = 7,
        /// Manager and sub-manager to be linked have duplicate client.
        DuplicateChildFound = 8,
        /// Client has no active user that can access the client account.
        ClientHasNoAdminUser = 9,
        /// Adding this link would exceed the maximum hierarchy depth.
        MaxDepthExceeded = 10,
        /// Adding this link will create a cycle.
        CycleNotAllowed = 11,
        /// Manager account has the maximum number of linked clients.
        TooManyAccounts = 12,
        /// Parent manager account has the maximum number of linked clients.
        TooManyAccountsAtManager = 13,
        /// The account is not authorized owner.
        NonOwnerUserCannotModifyLink = 14,
        /// Your manager account is suspended, and you are no longer allowed to link
        /// to clients.
        SuspendedAccountCannotAddClients = 15,
        /// You are not allowed to move a client to a manager that is not under your
        /// current hierarchy.
        ClientOutsideTree = 16,
        /// The changed status for mutate link is invalid.
        InvalidStatusChange = 17,
        /// The change for mutate link is invalid.
        InvalidChange = 18,
        /// You are not allowed to link a manager account to itself.
        CustomerCannotManageSelf = 19,
        /// The link was created with status ACTIVE and not PENDING.
        CreatingEnabledLinkNotAllowed = 20,
    }
}
// Proto file describing media bundle errors.

/// Container for enum describing possible media bundle errors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MediaBundleErrorEnum {}
/// Nested message and enum types in `MediaBundleErrorEnum`.
pub mod media_bundle_error_enum {
    /// Enum describing possible media bundle errors.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum MediaBundleError {
        /// Enum unspecified.
        Unspecified = 0,
        /// The received error code is not known in this version.
        Unknown = 1,
        /// There was a problem with the request.
        BadRequest = 3,
        /// HTML5 ads using DoubleClick Studio created ZIP files are not supported.
        DoubleclickBundleNotAllowed = 4,
        /// Cannot reference URL external to the media bundle.
        ExternalUrlNotAllowed = 5,
        /// Media bundle file is too large.
        FileTooLarge = 6,
        /// ZIP file from Google Web Designer is not published.
        GoogleWebDesignerZipFileNotPublished = 7,
        /// Input was invalid.
        InvalidInput = 8,
        /// There was a problem with the media bundle.
        InvalidMediaBundle = 9,
        /// There was a problem with one or more of the media bundle entries.
        InvalidMediaBundleEntry = 10,
        /// The media bundle contains a file with an unknown mime type
        InvalidMimeType = 11,
        /// The media bundle contain an invalid asset path.
        InvalidPath = 12,
        /// HTML5 ad is trying to reference an asset not in .ZIP file
        InvalidUrlReference = 13,
        /// Media data is too large.
        MediaDataTooLarge = 14,
        /// The media bundle contains no primary entry.
        MissingPrimaryMediaBundleEntry = 15,
        /// There was an error on the server.
        ServerError = 16,
        /// The image could not be stored.
        StorageError = 17,
        /// Media bundle created with the Swiffy tool is not allowed.
        SwiffyBundleNotAllowed = 18,
        /// The media bundle contains too many files.
        TooManyFiles = 19,
        /// The media bundle is not of legal dimensions.
        UnexpectedSize = 20,
        /// Google Web Designer not created for "Google Ads" environment.
        UnsupportedGoogleWebDesignerEnvironment = 21,
        /// Unsupported HTML5 feature in HTML5 asset.
        UnsupportedHtml5Feature = 22,
        /// URL in HTML5 entry is not ssl compliant.
        UrlInMediaBundleNotSslCompliant = 23,
        /// Custom exits not allowed in HTML5 entry.
        CustomExitNotAllowed = 24,
    }
}
// Proto file describing media file errors.

/// Container for enum describing possible media file errors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MediaFileErrorEnum {}
/// Nested message and enum types in `MediaFileErrorEnum`.
pub mod media_file_error_enum {
    /// Enum describing possible media file errors.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum MediaFileError {
        /// Enum unspecified.
        Unspecified = 0,
        /// The received error code is not known in this version.
        Unknown = 1,
        /// Cannot create a standard icon type.
        CannotCreateStandardIcon = 2,
        /// May only select Standard Icons alone.
        CannotSelectStandardIconWithOtherTypes = 3,
        /// Image contains both a media file ID and data.
        CannotSpecifyMediaFileIdAndData = 4,
        /// A media file with given type and reference ID already exists.
        DuplicateMedia = 5,
        /// A required field was not specified or is an empty string.
        EmptyField = 6,
        /// A media file may only be modified once per call.
        ResourceReferencedInMultipleOps = 7,
        /// Field is not supported for the media sub type.
        FieldNotSupportedForMediaSubType = 8,
        /// The media file ID is invalid.
        InvalidMediaFileId = 9,
        /// The media subtype is invalid.
        InvalidMediaSubType = 10,
        /// The media file type is invalid.
        InvalidMediaFileType = 11,
        /// The mimetype is invalid.
        InvalidMimeType = 12,
        /// The media reference ID is invalid.
        InvalidReferenceId = 13,
        /// The YouTube video ID is invalid.
        InvalidYouTubeId = 14,
        /// Media file has failed transcoding
        MediaFileFailedTranscoding = 15,
        /// Media file has not been transcoded.
        MediaNotTranscoded = 16,
        /// The media type does not match the actual media file's type.
        MediaTypeDoesNotMatchMediaFileType = 17,
        /// None of the fields have been specified.
        NoFieldsSpecified = 18,
        /// One of reference ID or media file ID must be specified.
        NullReferenceIdAndMediaId = 19,
        /// The string has too many characters.
        TooLong = 20,
        /// The specified type is not supported.
        UnsupportedType = 21,
        /// YouTube is unavailable for requesting video data.
        YouTubeServiceUnavailable = 22,
        /// The YouTube video has a non positive duration.
        YouTubeVideoHasNonPositiveDuration = 23,
        /// The YouTube video ID is syntactically valid but the video was not found.
        YouTubeVideoNotFound = 24,
    }
}
// Proto file describing media uploading errors.

/// Container for enum describing possible media uploading errors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MediaUploadErrorEnum {}
/// Nested message and enum types in `MediaUploadErrorEnum`.
pub mod media_upload_error_enum {
    /// Enum describing possible media uploading errors.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum MediaUploadError {
        /// Enum unspecified.
        Unspecified = 0,
        /// The received error code is not known in this version.
        Unknown = 1,
        /// The uploaded file is too big.
        FileTooBig = 2,
        /// Image data is unparseable.
        UnparseableImage = 3,
        /// Animated images are not allowed.
        AnimatedImageNotAllowed = 4,
        /// The image or media bundle format is not allowed.
        FormatNotAllowed = 5,
        /// Cannot reference URL external to the media bundle.
        ExternalUrlNotAllowed = 6,
        /// HTML5 ad is trying to reference an asset not in .ZIP file.
        InvalidUrlReference = 7,
        /// The media bundle contains no primary entry.
        MissingPrimaryMediaBundleEntry = 8,
        /// Animation has disallowed visual effects.
        AnimatedVisualEffect = 9,
        /// Animation longer than the allowed 30 second limit.
        AnimationTooLong = 10,
        /// The aspect ratio of the image does not match the expected aspect ratios
        /// provided in the asset spec.
        AspectRatioNotAllowed = 11,
        /// Audio files are not allowed in bundle.
        AudioNotAllowedInMediaBundle = 12,
        /// CMYK jpegs are not supported.
        CmykJpegNotAllowed = 13,
        /// Flash movies are not allowed.
        FlashNotAllowed = 14,
        /// The frame rate of the video is higher than the allowed 5fps.
        FrameRateTooHigh = 15,
        /// ZIP file from Google Web Designer is not published.
        GoogleWebDesignerZipFileNotPublished = 16,
        /// Image constraints are violated, but more details (like
        /// DIMENSIONS_NOT_ALLOWED or ASPECT_RATIO_NOT_ALLOWED) can not be provided.
        /// This happens when asset spec contains more than one constraint and
        /// criteria of different constraints are violated.
        ImageConstraintsViolated = 17,
        /// Media bundle data is unrecognizable.
        InvalidMediaBundle = 18,
        /// There was a problem with one or more of the media bundle entries.
        InvalidMediaBundleEntry = 19,
        /// The asset has an invalid mime type.
        InvalidMimeType = 20,
        /// The media bundle contains an invalid asset path.
        InvalidPath = 21,
        /// Image has layout problem.
        LayoutProblem = 22,
        /// An asset had a URL reference that is malformed per RFC 1738 convention.
        MalformedUrl = 23,
        /// The uploaded media bundle format is not allowed.
        MediaBundleNotAllowed = 24,
        /// The media bundle is not compatible with the asset spec product type.
        /// (E.g. Gmail, dynamic remarketing, etc.)
        MediaBundleNotCompatibleToProductType = 25,
        /// A bundle being uploaded that is incompatible with multiple assets for
        /// different reasons.
        MediaBundleRejectedByMultipleAssetSpecs = 26,
        /// The media bundle contains too many files.
        TooManyFilesInMediaBundle = 27,
        /// Google Web Designer not created for "Google Ads" environment.
        UnsupportedGoogleWebDesignerEnvironment = 28,
        /// Unsupported HTML5 feature in HTML5 asset.
        UnsupportedHtml5Feature = 29,
        /// URL in HTML5 entry is not SSL compliant.
        UrlInMediaBundleNotSslCompliant = 30,
        /// Video file name is longer than the 50 allowed characters.
        VideoFileNameTooLong = 31,
        /// Multiple videos with same name in a bundle.
        VideoMultipleFilesWithSameName = 32,
        /// Videos are not allowed in media bundle.
        VideoNotAllowedInMediaBundle = 33,
        /// This type of media cannot be uploaded through the Google Ads API.
        CannotUploadMediaTypeThroughApi = 34,
        /// The dimensions of the image are not allowed.
        DimensionsNotAllowed = 35,
    }
}
// Proto file describing merchant center errors.

/// Container for enum describing possible merchant center errors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MerchantCenterErrorEnum {}
/// Nested message and enum types in `MerchantCenterErrorEnum`.
pub mod merchant_center_error_enum {
    /// Enum describing Merchant Center errors.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum MerchantCenterError {
        /// Enum unspecified.
        Unspecified = 0,
        /// The received error code is not known in this version.
        Unknown = 1,
        /// Merchant ID is either not found or not linked to the Google Ads customer.
        MerchantIdCannotBeAccessed = 2,
        /// Customer not allowlisted for Shopping in Performance Max Campaign.
        CustomerNotAllowedForShoppingPerformanceMax = 3,
    }
}
// Proto file describing multiplier errors.

/// Container for enum describing possible multiplier errors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MultiplierErrorEnum {}
/// Nested message and enum types in `MultiplierErrorEnum`.
pub mod multiplier_error_enum {
    /// Enum describing possible multiplier errors.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum MultiplierError {
        /// Enum unspecified.
        Unspecified = 0,
        /// The received error code is not known in this version.
        Unknown = 1,
        /// Multiplier value is too high
        MultiplierTooHigh = 2,
        /// Multiplier value is too low
        MultiplierTooLow = 3,
        /// Too many fractional digits
        TooManyFractionalDigits = 4,
        /// A multiplier cannot be set for this bidding strategy
        MultiplierNotAllowedForBiddingStrategy = 5,
        /// A multiplier cannot be set when there is no base bid (e.g., content max
        /// cpc)
        MultiplierNotAllowedWhenBaseBidIsMissing = 6,
        /// A bid multiplier must be specified
        NoMultiplierSpecified = 7,
        /// Multiplier causes bid to exceed daily budget
        MultiplierCausesBidToExceedDailyBudget = 8,
        /// Multiplier causes bid to exceed monthly budget
        MultiplierCausesBidToExceedMonthlyBudget = 9,
        /// Multiplier causes bid to exceed custom budget
        MultiplierCausesBidToExceedCustomBudget = 10,
        /// Multiplier causes bid to exceed maximum allowed bid
        MultiplierCausesBidToExceedMaxAllowedBid = 11,
        /// Multiplier causes bid to become less than the minimum bid allowed
        BidLessThanMinAllowedBidWithMultiplier = 12,
        /// Multiplier type (cpc vs. cpm) needs to match campaign's bidding strategy
        MultiplierAndBiddingStrategyTypeMismatch = 13,
    }
}
// Proto file describing mutate errors.

/// Container for enum describing possible mutate errors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateErrorEnum {}
/// Nested message and enum types in `MutateErrorEnum`.
pub mod mutate_error_enum {
    /// Enum describing possible mutate errors.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum MutateError {
        /// Enum unspecified.
        Unspecified = 0,
        /// The received error code is not known in this version.
        Unknown = 1,
        /// Requested resource was not found.
        ResourceNotFound = 3,
        /// Cannot mutate the same resource twice in one request.
        IdExistsInMultipleMutates = 7,
        /// The field's contents don't match another field that represents the same
        /// data.
        InconsistentFieldValues = 8,
        /// Mutates are not allowed for the requested resource.
        MutateNotAllowed = 9,
        /// The resource isn't in Google Ads. It belongs to another ads system.
        ResourceNotInGoogleAds = 10,
        /// The resource being created already exists.
        ResourceAlreadyExists = 11,
        /// This resource cannot be used with "validate_only".
        ResourceDoesNotSupportValidateOnly = 12,
        /// This operation cannot be used with "partial_failure".
        OperationDoesNotSupportPartialFailure = 16,
        /// Attempt to write to read-only fields.
        ResourceReadOnly = 13,
    }
}
// Proto file describing new resource creation errors.

/// Container for enum describing possible new resource creation errors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewResourceCreationErrorEnum {}
/// Nested message and enum types in `NewResourceCreationErrorEnum`.
pub mod new_resource_creation_error_enum {
    /// Enum describing possible new resource creation errors.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum NewResourceCreationError {
        /// Enum unspecified.
        Unspecified = 0,
        /// The received error code is not known in this version.
        Unknown = 1,
        /// Do not set the id field while creating new resources.
        CannotSetIdForCreate = 2,
        /// Creating more than one resource with the same temp ID is not allowed.
        DuplicateTempIds = 3,
        /// Parent resource with specified temp ID failed validation, so no
        /// validation will be done for this child resource.
        TempIdResourceHadErrors = 4,
    }
}
// Proto file describing not allowlisted errors.

/// Container for enum describing possible not allowlisted errors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NotAllowlistedErrorEnum {}
/// Nested message and enum types in `NotAllowlistedErrorEnum`.
pub mod not_allowlisted_error_enum {
    /// Enum describing possible not allowlisted errors.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum NotAllowlistedError {
        /// Enum unspecified.
        Unspecified = 0,
        /// The received error code is not known in this version.
        Unknown = 1,
        /// Customer is not allowlisted for accessing this feature.
        CustomerNotAllowlistedForThisFeature = 2,
    }
}
// Proto file describing not empty errors.

/// Container for enum describing possible not empty errors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NotEmptyErrorEnum {}
/// Nested message and enum types in `NotEmptyErrorEnum`.
pub mod not_empty_error_enum {
    /// Enum describing possible not empty errors.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum NotEmptyError {
        /// Enum unspecified.
        Unspecified = 0,
        /// The received error code is not known in this version.
        Unknown = 1,
        /// Empty list.
        EmptyList = 2,
    }
}
// Proto file describing null errors.

/// Container for enum describing possible null errors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NullErrorEnum {}
/// Nested message and enum types in `NullErrorEnum`.
pub mod null_error_enum {
    /// Enum describing possible null errors.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum NullError {
        /// Enum unspecified.
        Unspecified = 0,
        /// The received error code is not known in this version.
        Unknown = 1,
        /// Specified list/container must not contain any null elements
        NullContent = 2,
    }
}
// Proto file describing offline user data job errors.

/// Container for enum describing possible offline user data job errors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OfflineUserDataJobErrorEnum {}
/// Nested message and enum types in `OfflineUserDataJobErrorEnum`.
pub mod offline_user_data_job_error_enum {
    /// Enum describing possible request errors.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum OfflineUserDataJobError {
        /// Enum unspecified.
        Unspecified = 0,
        /// The received error code is not known in this version.
        Unknown = 1,
        /// The user list ID provided for the job is invalid.
        InvalidUserListId = 3,
        /// Type of the user list is not applicable for the job.
        InvalidUserListType = 4,
        /// Customer is not allowisted for using user ID in upload data.
        NotOnAllowlistForUserId = 33,
        /// Upload data is not compatible with the upload key type of the associated
        /// user list.
        IncompatibleUploadKeyType = 6,
        /// The user identifier is missing valid data.
        MissingUserIdentifier = 7,
        /// The mobile ID is malformed.
        InvalidMobileIdFormat = 8,
        /// Maximum number of user identifiers allowed per request is 100,000 and per
        /// operation is 20.
        TooManyUserIdentifiers = 9,
        /// Customer is not on the allow-list for store sales direct data.
        NotOnAllowlistForStoreSalesDirect = 31,
        /// Customer is not on the allow-list for unified store sales data.
        NotOnAllowlistForUnifiedStoreSales = 32,
        /// The partner ID in store sales direct metadata is invalid.
        InvalidPartnerId = 11,
        /// The data in user identifier should not be encoded.
        InvalidEncoding = 12,
        /// The country code is invalid.
        InvalidCountryCode = 13,
        /// Incompatible user identifier when using third_party_user_id for store
        /// sales direct first party data or not using third_party_user_id for store
        /// sales third party data.
        IncompatibleUserIdentifier = 14,
        /// A transaction time in the future is not allowed.
        FutureTransactionTime = 15,
        /// The conversion_action specified in transaction_attributes is used to
        /// report conversions to a conversion action configured in Google Ads. This
        /// error indicates there is no such conversion action in the account.
        InvalidConversionAction = 16,
        /// Mobile ID is not supported for store sales direct data.
        MobileIdNotSupported = 17,
        /// When a remove-all operation is provided, it has to be the first operation
        /// of the operation list.
        InvalidOperationOrder = 18,
        /// Mixing creation and removal of offline data in the same job is not
        /// allowed.
        ConflictingOperation = 19,
        /// The external update ID already exists.
        ExternalUpdateIdAlreadyExists = 21,
        /// Once the upload job is started, new operations cannot be added.
        JobAlreadyStarted = 22,
        /// Remove operation is not allowed for store sales direct updates.
        RemoveNotSupported = 23,
        /// Remove-all is not supported for certain offline user data job types.
        RemoveAllNotSupported = 24,
        /// The SHA256 encoded value is malformed.
        InvalidSha256Format = 25,
        /// The custom key specified is not enabled for the unified store sales
        /// upload.
        CustomKeyDisabled = 26,
        /// The custom key specified is not predefined through the Google Ads UI.
        CustomKeyNotPredefined = 27,
        /// The custom key specified is not set in the upload.
        CustomKeyNotSet = 29,
        /// The customer has not accepted the customer data terms in the conversion
        /// settings page.
        CustomerNotAcceptedCustomerDataTerms = 30,
        /// User attributes cannot be uploaded into a user list.
        AttributesNotApplicableForCustomerMatchUserList = 34,
        /// Lifetime value bucket must be a number from 1-10, except for remove
        /// operation where 0 will be accepted.
        LifetimeValueBucketNotInRange = 35,
        /// Identifiers not supported for Customer Match attributes. User attributes
        /// can only be provided with contact info (email, phone, address) user
        /// identifiers.
        IncompatibleUserIdentifierForAttributes = 36,
        /// A time in the future is not allowed.
        FutureTimeNotAllowed = 37,
        /// Last purchase date time cannot be less than acquisition date time.
        LastPurchaseTimeLessThanAcquisitionTime = 38,
        /// Only emails are accepted as user identifiers for shopping loyalty match.
        /// {-- api.dev/not-precedent: The identifier is not limited to ids, but
        /// also include other user info eg. phone numbers.}
        CustomerIdentifierNotAllowed = 39,
        /// Provided item ID is invalid.
        InvalidItemId = 40,
    }
}
// Proto file describing operation access denied errors.

/// Container for enum describing possible operation access denied errors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationAccessDeniedErrorEnum {}
/// Nested message and enum types in `OperationAccessDeniedErrorEnum`.
pub mod operation_access_denied_error_enum {
    /// Enum describing possible operation access denied errors.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum OperationAccessDeniedError {
        /// Enum unspecified.
        Unspecified = 0,
        /// The received error code is not known in this version.
        Unknown = 1,
        /// Unauthorized invocation of a service's method (get, mutate, etc.)
        ActionNotPermitted = 2,
        /// Unauthorized CREATE operation in invoking a service's mutate method.
        CreateOperationNotPermitted = 3,
        /// Unauthorized REMOVE operation in invoking a service's mutate method.
        RemoveOperationNotPermitted = 4,
        /// Unauthorized UPDATE operation in invoking a service's mutate method.
        UpdateOperationNotPermitted = 5,
        /// A mutate action is not allowed on this resource, from this client.
        MutateActionNotPermittedForClient = 6,
        /// This operation is not permitted on this campaign type
        OperationNotPermittedForCampaignType = 7,
        /// A CREATE operation may not set status to REMOVED.
        CreateAsRemovedNotPermitted = 8,
        /// This operation is not allowed because the resource is removed.
        OperationNotPermittedForRemovedResource = 9,
        /// This operation is not permitted on this ad group type.
        OperationNotPermittedForAdGroupType = 10,
        /// The mutate is not allowed for this customer.
        MutateNotPermittedForCustomer = 11,
    }
}
// Proto file describing operator errors.

/// Container for enum describing possible operator errors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperatorErrorEnum {}
/// Nested message and enum types in `OperatorErrorEnum`.
pub mod operator_error_enum {
    /// Enum describing possible operator errors.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum OperatorError {
        /// Enum unspecified.
        Unspecified = 0,
        /// The received error code is not known in this version.
        Unknown = 1,
        /// Operator not supported.
        OperatorNotSupported = 2,
    }
}
// Proto file describing partial failure errors.

/// Container for enum describing possible partial failure errors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PartialFailureErrorEnum {}
/// Nested message and enum types in `PartialFailureErrorEnum`.
pub mod partial_failure_error_enum {
    /// Enum describing possible partial failure errors.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum PartialFailureError {
        /// Enum unspecified.
        Unspecified = 0,
        /// The received error code is not known in this version.
        Unknown = 1,
        /// The partial failure field was false in the request.
        /// This method requires this field be set to true.
        PartialFailureModeRequired = 2,
    }
}
// Proto file describing payments account service errors.

/// Container for enum describing possible errors in payments account service.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PaymentsAccountErrorEnum {}
/// Nested message and enum types in `PaymentsAccountErrorEnum`.
pub mod payments_account_error_enum {
    /// Enum describing possible errors in payments account service.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum PaymentsAccountError {
        /// Enum unspecified.
        Unspecified = 0,
        /// The received error code is not known in this version.
        Unknown = 1,
        /// Manager customers are not supported for payments account service.
        NotSupportedForManagerCustomer = 2,
    }
}
// Proto file describing policy finding errors.

/// Container for enum describing possible policy finding errors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PolicyFindingErrorEnum {}
/// Nested message and enum types in `PolicyFindingErrorEnum`.
pub mod policy_finding_error_enum {
    /// Enum describing possible policy finding errors.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum PolicyFindingError {
        /// Enum unspecified.
        Unspecified = 0,
        /// The received error code is not known in this version.
        Unknown = 1,
        /// The resource has been disapproved since the policy summary includes
        /// policy topics of type PROHIBITED.
        PolicyFinding = 2,
        /// The given policy topic does not exist.
        PolicyTopicNotFound = 3,
    }
}
// Proto file describing policy validation parameter errors.

/// Container for enum describing possible policy validation parameter errors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PolicyValidationParameterErrorEnum {}
/// Nested message and enum types in `PolicyValidationParameterErrorEnum`.
pub mod policy_validation_parameter_error_enum {
    /// Enum describing possible policy validation parameter errors.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum PolicyValidationParameterError {
        /// Enum unspecified.
        Unspecified = 0,
        /// The received error code is not known in this version.
        Unknown = 1,
        /// Ignorable policy topics are not supported for the ad type.
        UnsupportedAdTypeForIgnorablePolicyTopics = 2,
        /// Exempt policy violation keys are not supported for the ad type.
        UnsupportedAdTypeForExemptPolicyViolationKeys = 3,
        /// Cannot set ignorable policy topics and exempt policy violation keys in
        /// the same policy violation parameter.
        CannotSetBothIgnorablePolicyTopicsAndExemptPolicyViolationKeys = 4,
    }
}
// Proto file describing policy violation errors.

/// Container for enum describing possible policy violation errors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PolicyViolationErrorEnum {}
/// Nested message and enum types in `PolicyViolationErrorEnum`.
pub mod policy_violation_error_enum {
    /// Enum describing possible policy violation errors.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum PolicyViolationError {
        /// Enum unspecified.
        Unspecified = 0,
        /// The received error code is not known in this version.
        Unknown = 1,
        /// A policy was violated. See PolicyViolationDetails for more detail.
        PolicyError = 2,
    }
}
// Proto file describing query errors.

/// Container for enum describing possible query errors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryErrorEnum {}
/// Nested message and enum types in `QueryErrorEnum`.
pub mod query_error_enum {
    /// Enum describing possible query errors.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum QueryError {
        /// Name unspecified.
        Unspecified = 0,
        /// The received error code is not known in this version.
        Unknown = 1,
        /// Returned if all other query error reasons are not applicable.
        QueryError = 50,
        /// A condition used in the query references an invalid enum constant.
        BadEnumConstant = 18,
        /// Query contains an invalid escape sequence.
        BadEscapeSequence = 7,
        /// Field name is invalid.
        BadFieldName = 12,
        /// Limit value is invalid (i.e. not a number)
        BadLimitValue = 15,
        /// Encountered number can not be parsed.
        BadNumber = 5,
        /// Invalid operator encountered.
        BadOperator = 3,
        /// Parameter unknown or not supported.
        BadParameterName = 61,
        /// Parameter have invalid value.
        BadParameterValue = 62,
        /// Invalid resource type was specified in the FROM clause.
        BadResourceTypeInFromClause = 45,
        /// Non-ASCII symbol encountered outside of strings.
        BadSymbol = 2,
        /// Value is invalid.
        BadValue = 4,
        /// Date filters fail to restrict date to a range smaller than 31 days.
        /// Applicable if the query is segmented by date.
        DateRangeTooWide = 36,
        /// Filters on date/week/month/quarter have a start date after
        /// end date.
        DateRangeTooNarrow = 60,
        /// Expected AND between values with BETWEEN operator.
        ExpectedAnd = 30,
        /// Expecting ORDER BY to have BY.
        ExpectedBy = 14,
        /// There was no dimension field selected.
        ExpectedDimensionFieldInSelectClause = 37,
        /// Missing filters on date related fields.
        ExpectedFiltersOnDateRange = 55,
        /// Missing FROM clause.
        ExpectedFrom = 44,
        /// The operator used in the conditions requires the value to be a list.
        ExpectedList = 41,
        /// Fields used in WHERE or ORDER BY clauses are missing from the SELECT
        /// clause.
        ExpectedReferencedFieldInSelectClause = 16,
        /// SELECT is missing at the beginning of query.
        ExpectedSelect = 13,
        /// A list was passed as a value to a condition whose operator expects a
        /// single value.
        ExpectedSingleValue = 42,
        /// Missing one or both values with BETWEEN operator.
        ExpectedValueWithBetweenOperator = 29,
        /// Invalid date format. Expected 'YYYY-MM-DD'.
        InvalidDateFormat = 38,
        /// Misaligned date value for the filter. The date should be the start of a
        /// week/month/quarter if the filtered field is
        /// segments.week/segments.month/segments.quarter.
        MisalignedDateForFilter = 64,
        /// Value passed was not a string when it should have been. I.e., it was a
        /// number or unquoted literal.
        InvalidStringValue = 57,
        /// A String value passed to the BETWEEN operator does not parse as a date.
        InvalidValueWithBetweenOperator = 26,
        /// The value passed to the DURING operator is not a Date range literal
        InvalidValueWithDuringOperator = 22,
        /// An invalid value was passed to the LIKE operator.
        InvalidValueWithLikeOperator = 56,
        /// An operator was provided that is inapplicable to the field being
        /// filtered.
        OperatorFieldMismatch = 35,
        /// A Condition was found with an empty list.
        ProhibitedEmptyListInCondition = 28,
        /// A condition used in the query references an unsupported enum constant.
        ProhibitedEnumConstant = 54,
        /// Fields that are not allowed to be selected together were included in
        /// the SELECT clause.
        ProhibitedFieldCombinationInSelectClause = 31,
        /// A field that is not orderable was included in the ORDER BY clause.
        ProhibitedFieldInOrderByClause = 40,
        /// A field that is not selectable was included in the SELECT clause.
        ProhibitedFieldInSelectClause = 23,
        /// A field that is not filterable was included in the WHERE clause.
        ProhibitedFieldInWhereClause = 24,
        /// Resource type specified in the FROM clause is not supported by this
        /// service.
        ProhibitedResourceTypeInFromClause = 43,
        /// A field that comes from an incompatible resource was included in the
        /// SELECT clause.
        ProhibitedResourceTypeInSelectClause = 48,
        /// A field that comes from an incompatible resource was included in the
        /// WHERE clause.
        ProhibitedResourceTypeInWhereClause = 58,
        /// A metric incompatible with the main resource or other selected
        /// segmenting resources was included in the SELECT or WHERE clause.
        ProhibitedMetricInSelectOrWhereClause = 49,
        /// A segment incompatible with the main resource or other selected
        /// segmenting resources was included in the SELECT or WHERE clause.
        ProhibitedSegmentInSelectOrWhereClause = 51,
        /// A segment in the SELECT clause is incompatible with a metric in the
        /// SELECT or WHERE clause.
        ProhibitedSegmentWithMetricInSelectOrWhereClause = 53,
        /// The value passed to the limit clause is too low.
        LimitValueTooLow = 25,
        /// Query has a string containing a newline character.
        ProhibitedNewlineInString = 8,
        /// List contains values of different types.
        ProhibitedValueCombinationInList = 10,
        /// The values passed to the BETWEEN operator are not of the same type.
        ProhibitedValueCombinationWithBetweenOperator = 21,
        /// Query contains unterminated string.
        StringNotTerminated = 6,
        /// Too many segments are specified in SELECT clause.
        TooManySegments = 34,
        /// Query is incomplete and cannot be parsed.
        UnexpectedEndOfQuery = 9,
        /// FROM clause cannot be specified in this query.
        UnexpectedFromClause = 47,
        /// Query contains one or more unrecognized fields.
        UnrecognizedField = 32,
        /// Query has an unexpected extra part.
        UnexpectedInput = 11,
        /// Metrics cannot be requested for a manager account. To retrieve metrics,
        /// issue separate requests against each client account under the manager
        /// account.
        RequestedMetricsForManager = 59,
        /// The number of values (right-hand-side operands) in a filter exceeds the
        /// limit.
        FilterHasTooManyValues = 63,
    }
}
// Proto file describing quota errors.

/// Container for enum describing possible quota errors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuotaErrorEnum {}
/// Nested message and enum types in `QuotaErrorEnum`.
pub mod quota_error_enum {
    /// Enum describing possible quota errors.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum QuotaError {
        /// Enum unspecified.
        Unspecified = 0,
        /// The received error code is not known in this version.
        Unknown = 1,
        /// Too many requests.
        ResourceExhausted = 2,
        /// Access is prohibited.
        AccessProhibited = 3,
        /// Too many requests in a short amount of time.
        ResourceTemporarilyExhausted = 4,
    }
}
// Proto file describing range errors.

/// Container for enum describing possible range errors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RangeErrorEnum {}
/// Nested message and enum types in `RangeErrorEnum`.
pub mod range_error_enum {
    /// Enum describing possible range errors.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum RangeError {
        /// Enum unspecified.
        Unspecified = 0,
        /// The received error code is not known in this version.
        Unknown = 1,
        /// Too low.
        TooLow = 2,
        /// Too high.
        TooHigh = 3,
    }
}
// Proto file describing errors generated from ReachPlanService.

/// Container for enum describing possible errors returned from
/// the ReachPlanService.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReachPlanErrorEnum {}
/// Nested message and enum types in `ReachPlanErrorEnum`.
pub mod reach_plan_error_enum {
    /// Enum describing possible errors from ReachPlanService.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ReachPlanError {
        /// Enum unspecified.
        Unspecified = 0,
        /// The received error code is not known in this version.
        Unknown = 1,
        /// Not forecastable due to missing rate card data.
        NotForecastableMissingRate = 2,
    }
}
// Proto file describing errors from applying a recommendation.

/// Container for enum describing possible errors from applying a recommendation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RecommendationErrorEnum {}
/// Nested message and enum types in `RecommendationErrorEnum`.
pub mod recommendation_error_enum {
    /// Enum describing possible errors from applying a recommendation.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum RecommendationError {
        /// Enum unspecified.
        Unspecified = 0,
        /// The received error code is not known in this version.
        Unknown = 1,
        /// The specified budget amount is too low e.g. lower than minimum currency
        /// unit or lower than ad group minimum cost-per-click.
        BudgetAmountTooSmall = 2,
        /// The specified budget amount is too large.
        BudgetAmountTooLarge = 3,
        /// The specified budget amount is not a valid amount. e.g. not a multiple
        /// of minimum currency unit.
        InvalidBudgetAmount = 4,
        /// The specified keyword or ad violates ad policy.
        PolicyError = 5,
        /// The specified bid amount is not valid. e.g. too many fractional digits,
        /// or negative amount.
        InvalidBidAmount = 6,
        /// The number of keywords in ad group have reached the maximum allowed.
        AdgroupKeywordLimit = 7,
        /// The recommendation requested to apply has already been applied.
        RecommendationAlreadyApplied = 8,
        /// The recommendation requested to apply has been invalidated.
        RecommendationInvalidated = 9,
        /// The number of operations in a single request exceeds the maximum allowed.
        TooManyOperations = 10,
        /// There are no operations in the request.
        NoOperations = 11,
        /// Operations with multiple recommendation types are not supported when
        /// partial failure mode is not enabled.
        DifferentTypesNotSupported = 12,
        /// Request contains multiple operations with the same resource_name.
        DuplicateResourceName = 13,
        /// The recommendation requested to dismiss has already been dismissed.
        RecommendationAlreadyDismissed = 14,
        /// The recommendation apply request was malformed and invalid.
        InvalidApplyRequest = 15,
    }
}
// Proto file describing region code errors.

/// Container for enum describing possible region code errors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegionCodeErrorEnum {}
/// Nested message and enum types in `RegionCodeErrorEnum`.
pub mod region_code_error_enum {
    /// Enum describing possible region code errors.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum RegionCodeError {
        /// Enum unspecified.
        Unspecified = 0,
        /// The received error code is not known in this version.
        Unknown = 1,
        /// Invalid region code.
        InvalidRegionCode = 2,
    }
}
// Proto file describing request errors.

/// Container for enum describing possible request errors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestErrorEnum {}
/// Nested message and enum types in `RequestErrorEnum`.
pub mod request_error_enum {
    /// Enum describing possible request errors.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum RequestError {
        /// Enum unspecified.
        Unspecified = 0,
        /// The received error code is not known in this version.
        Unknown = 1,
        /// Resource name is required for this request.
        ResourceNameMissing = 3,
        /// Resource name provided is malformed.
        ResourceNameMalformed = 4,
        /// Resource name provided is malformed.
        BadResourceId = 17,
        /// Customer ID is invalid.
        InvalidCustomerId = 16,
        /// Mutate operation should have either create, update, or remove specified.
        OperationRequired = 5,
        /// Requested resource not found.
        ResourceNotFound = 6,
        /// Next page token specified in user request is invalid.
        InvalidPageToken = 7,
        /// Next page token specified in user request has expired.
        ExpiredPageToken = 8,
        /// Page size specified in user request is invalid.
        InvalidPageSize = 22,
        /// Required field is missing.
        RequiredFieldMissing = 9,
        /// The field cannot be modified because it's immutable. It's also possible
        /// that the field can be modified using 'create' operation but not 'update'.
        ImmutableField = 11,
        /// Received too many entries in request.
        TooManyMutateOperations = 13,
        /// Request cannot be executed by a manager account.
        CannotBeExecutedByManagerAccount = 14,
        /// Mutate request was attempting to modify a readonly field.
        /// For instance, Budget fields can be requested for Ad Group,
        /// but are read-only for adGroups:mutate.
        CannotModifyForeignField = 15,
        /// Enum value is not permitted.
        InvalidEnumValue = 18,
        /// The developer-token parameter is required for all requests.
        DeveloperTokenParameterMissing = 19,
        /// The login-customer-id parameter is required for this request.
        LoginCustomerIdParameterMissing = 20,
        /// page_token is set in the validate only request
        ValidateOnlyRequestHasPageToken = 21,
        /// return_summary_row cannot be enabled if request did not select any
        /// metrics field.
        CannotReturnSummaryRowForRequestWithoutMetrics = 29,
        /// return_summary_row should not be enabled for validate only requests.
        CannotReturnSummaryRowForValidateOnlyRequests = 30,
        /// return_summary_row parameter value should be the same between requests
        /// with page_token field set and their original request.
        InconsistentReturnSummaryRowValue = 31,
        /// The total results count cannot be returned if it was not requested in the
        /// original request.
        TotalResultsCountNotOriginallyRequested = 32,
        /// Deadline specified by the client was too short.
        RpcDeadlineTooShort = 33,
    }
}
// Proto file describing resource access denied errors.

/// Container for enum describing possible resource access denied errors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResourceAccessDeniedErrorEnum {}
/// Nested message and enum types in `ResourceAccessDeniedErrorEnum`.
pub mod resource_access_denied_error_enum {
    /// Enum describing possible resource access denied errors.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ResourceAccessDeniedError {
        /// Enum unspecified.
        Unspecified = 0,
        /// The received error code is not known in this version.
        Unknown = 1,
        /// User did not have write access.
        WriteAccessDenied = 3,
    }
}
// Proto file describing resource count limit exceeded errors.

/// Container for enum describing possible resource count limit exceeded errors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResourceCountLimitExceededErrorEnum {}
/// Nested message and enum types in `ResourceCountLimitExceededErrorEnum`.
pub mod resource_count_limit_exceeded_error_enum {
    /// Enum describing possible resource count limit exceeded errors.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ResourceCountLimitExceededError {
        /// Enum unspecified.
        Unspecified = 0,
        /// The received error code is not known in this version.
        Unknown = 1,
        /// Indicates that this request would exceed the number of allowed resources
        /// for the Google Ads account. The exact resource type and limit being
        /// checked can be inferred from accountLimitType.
        AccountLimit = 2,
        /// Indicates that this request would exceed the number of allowed resources
        /// in a Campaign. The exact resource type and limit being checked can be
        /// inferred from accountLimitType, and the numeric id of the
        /// Campaign involved is given by enclosingId.
        CampaignLimit = 3,
        /// Indicates that this request would exceed the number of allowed resources
        /// in an ad group. The exact resource type and limit being checked can be
        /// inferred from accountLimitType, and the numeric id of the
        /// ad group involved is given by enclosingId.
        AdgroupLimit = 4,
        /// Indicates that this request would exceed the number of allowed resources
        /// in an ad group ad. The exact resource type and limit being checked can
        /// be inferred from accountLimitType, and the enclosingId
        /// contains the ad group id followed by the ad id, separated by a single
        /// comma (,).
        AdGroupAdLimit = 5,
        /// Indicates that this request would exceed the number of allowed resources
        /// in an ad group criterion. The exact resource type and limit being checked
        /// can be inferred from accountLimitType, and the
        /// enclosingId contains the ad group id followed by the
        /// criterion id, separated by a single comma (,).
        AdGroupCriterionLimit = 6,
        /// Indicates that this request would exceed the number of allowed resources
        /// in this shared set. The exact resource type and limit being checked can
        /// be inferred from accountLimitType, and the numeric id of the
        /// shared set involved is given by enclosingId.
        SharedSetLimit = 7,
        /// Exceeds a limit related to a matching function.
        MatchingFunctionLimit = 8,
        /// The response for this request would exceed the maximum number of rows
        /// that can be returned.
        ResponseRowLimitExceeded = 9,
        /// This request would exceed a limit on the number of allowed resources.
        /// The details of which type of limit was exceeded will eventually be
        /// returned in ErrorDetails.
        ResourceLimit = 10,
    }
}
// Proto file describing setting errors.

/// Container for enum describing possible setting errors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SettingErrorEnum {}
/// Nested message and enum types in `SettingErrorEnum`.
pub mod setting_error_enum {
    /// Enum describing possible setting errors.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum SettingError {
        /// Enum unspecified.
        Unspecified = 0,
        /// The received error code is not known in this version.
        Unknown = 1,
        /// The campaign setting is not available for this Google Ads account.
        SettingTypeIsNotAvailable = 3,
        /// The setting is not compatible with the campaign.
        SettingTypeIsNotCompatibleWithCampaign = 4,
        /// The supplied TargetingSetting contains an invalid CriterionTypeGroup. See
        /// CriterionTypeGroup documentation for CriterionTypeGroups allowed
        /// in Campaign or AdGroup TargetingSettings.
        TargetingSettingContainsInvalidCriterionTypeGroup = 5,
        /// TargetingSetting must not explicitly
        /// set any of the Demographic CriterionTypeGroups (AGE_RANGE, GENDER,
        /// PARENT, INCOME_RANGE) to false (it's okay to not set them at all, in
        /// which case the system will set them to true automatically).
        TargetingSettingDemographicCriterionTypeGroupsMustBeSetToTargetAll = 6,
        /// TargetingSetting cannot change any of
        /// the Demographic CriterionTypeGroups (AGE_RANGE, GENDER, PARENT,
        /// INCOME_RANGE) from true to false.
        TargetingSettingCannotChangeTargetAllToFalseForDemographicCriterionTypeGroup = 7,
        /// At least one feed id should be present.
        DynamicSearchAdsSettingAtLeastOneFeedIdMustBePresent = 8,
        /// The supplied DynamicSearchAdsSetting contains an invalid domain name.
        DynamicSearchAdsSettingContainsInvalidDomainName = 9,
        /// The supplied DynamicSearchAdsSetting contains a subdomain name.
        DynamicSearchAdsSettingContainsSubdomainName = 10,
        /// The supplied DynamicSearchAdsSetting contains an invalid language code.
        DynamicSearchAdsSettingContainsInvalidLanguageCode = 11,
        /// TargetingSettings in search campaigns should not have
        /// CriterionTypeGroup.PLACEMENT set to targetAll.
        TargetAllIsNotAllowedForPlacementInSearchCampaign = 12,
        /// The setting value is not compatible with the campaign type.
        SettingValueNotCompatibleWithCampaign = 20,
    }
}
// Proto file describing shared criterion errors.

/// Container for enum describing possible shared criterion errors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SharedCriterionErrorEnum {}
/// Nested message and enum types in `SharedCriterionErrorEnum`.
pub mod shared_criterion_error_enum {
    /// Enum describing possible shared criterion errors.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum SharedCriterionError {
        /// Enum unspecified.
        Unspecified = 0,
        /// The received error code is not known in this version.
        Unknown = 1,
        /// The criterion is not appropriate for the shared set type.
        CriterionTypeNotAllowedForSharedSetType = 2,
    }
}
// Proto file describing shared set errors.

/// Container for enum describing possible shared set errors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SharedSetErrorEnum {}
/// Nested message and enum types in `SharedSetErrorEnum`.
pub mod shared_set_error_enum {
    /// Enum describing possible shared set errors.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum SharedSetError {
        /// Enum unspecified.
        Unspecified = 0,
        /// The received error code is not known in this version.
        Unknown = 1,
        /// The customer cannot create this type of shared set.
        CustomerCannotCreateSharedSetOfThisType = 2,
        /// A shared set with this name already exists.
        DuplicateName = 3,
        /// Removed shared sets cannot be mutated.
        SharedSetRemoved = 4,
        /// The shared set cannot be removed because it is in use.
        SharedSetInUse = 5,
    }
}
// Proto file describing size limit errors.

/// Container for enum describing possible size limit errors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SizeLimitErrorEnum {}
/// Nested message and enum types in `SizeLimitErrorEnum`.
pub mod size_limit_error_enum {
    /// Enum describing possible size limit errors.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum SizeLimitError {
        /// Enum unspecified.
        Unspecified = 0,
        /// The received error code is not known in this version.
        Unknown = 1,
        /// The number of entries in the request exceeds the system limit.
        RequestSizeLimitExceeded = 2,
        /// The number of entries in the response exceeds the system limit.
        ResponseSizeLimitExceeded = 3,
    }
}
// Proto file describing string format errors.

/// Container for enum describing possible string format errors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StringFormatErrorEnum {}
/// Nested message and enum types in `StringFormatErrorEnum`.
pub mod string_format_error_enum {
    /// Enum describing possible string format errors.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum StringFormatError {
        /// Enum unspecified.
        Unspecified = 0,
        /// The received error code is not known in this version.
        Unknown = 1,
        /// The input string value contains disallowed characters.
        IllegalChars = 2,
        /// The input string value is invalid for the associated field.
        InvalidFormat = 3,
    }
}
// Proto file describing string length errors.

/// Container for enum describing possible string length errors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StringLengthErrorEnum {}
/// Nested message and enum types in `StringLengthErrorEnum`.
pub mod string_length_error_enum {
    /// Enum describing possible string length errors.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum StringLengthError {
        /// Enum unspecified.
        Unspecified = 0,
        /// The received error code is not known in this version.
        Unknown = 1,
        /// The specified field should have a least one non-whitespace character in
        /// it.
        Empty = 4,
        /// Too short.
        TooShort = 2,
        /// Too long.
        TooLong = 3,
    }
}
// Proto file describing ThirdPartyAppAnalyticsLink errors.

/// Container for enum describing possible third party app analytics link errors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ThirdPartyAppAnalyticsLinkErrorEnum {}
/// Nested message and enum types in `ThirdPartyAppAnalyticsLinkErrorEnum`.
pub mod third_party_app_analytics_link_error_enum {
    /// Enum describing possible third party app analytics link errors.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ThirdPartyAppAnalyticsLinkError {
        /// Enum unspecified.
        Unspecified = 0,
        /// The received error code is not known in this version.
        Unknown = 1,
        /// The provided analytics provider ID is invalid.
        InvalidAnalyticsProviderId = 2,
        /// The provided mobile app ID is invalid.
        InvalidMobileAppId = 3,
        /// The mobile app corresponding to the provided app ID is not
        /// active/enabled.
        MobileAppIsNotEnabled = 4,
        /// Regenerating shareable link ID is only allowed on active links
        CannotRegenerateShareableLinkIdForRemovedLink = 5,
    }
}
// Proto file describing time zone errors.

/// Container for enum describing possible time zone errors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TimeZoneErrorEnum {}
/// Nested message and enum types in `TimeZoneErrorEnum`.
pub mod time_zone_error_enum {
    /// Enum describing possible currency code errors.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum TimeZoneError {
        /// Enum unspecified.
        Unspecified = 0,
        /// The received error code is not known in this version.
        Unknown = 1,
        /// Time zone is not valid.
        InvalidTimeZone = 5,
    }
}
// Proto file describing url field errors.

/// Container for enum describing possible url field errors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UrlFieldErrorEnum {}
/// Nested message and enum types in `UrlFieldErrorEnum`.
pub mod url_field_error_enum {
    /// Enum describing possible url field errors.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum UrlFieldError {
        /// Enum unspecified.
        Unspecified = 0,
        /// The received error code is not known in this version.
        Unknown = 1,
        /// The tracking url template is invalid.
        InvalidTrackingUrlTemplate = 2,
        /// The tracking url template contains invalid tag.
        InvalidTagInTrackingUrlTemplate = 3,
        /// The tracking url template must contain at least one tag (e.g. {lpurl}),
        /// This applies only to tracking url template associated with website ads or
        /// product ads.
        MissingTrackingUrlTemplateTag = 4,
        /// The tracking url template must start with a valid protocol (or lpurl
        /// tag).
        MissingProtocolInTrackingUrlTemplate = 5,
        /// The tracking url template starts with an invalid protocol.
        InvalidProtocolInTrackingUrlTemplate = 6,
        /// The tracking url template contains illegal characters.
        MalformedTrackingUrlTemplate = 7,
        /// The tracking url template must contain a host name (or lpurl tag).
        MissingHostInTrackingUrlTemplate = 8,
        /// The tracking url template has an invalid or missing top level domain
        /// extension.
        InvalidTldInTrackingUrlTemplate = 9,
        /// The tracking url template contains nested occurrences of the same
        /// conditional tag (i.e. {ifmobile:{ifmobile:x}}).
        RedundantNestedTrackingUrlTemplateTag = 10,
        /// The final url is invalid.
        InvalidFinalUrl = 11,
        /// The final url contains invalid tag.
        InvalidTagInFinalUrl = 12,
        /// The final url contains nested occurrences of the same conditional tag
        /// (i.e. {ifmobile:{ifmobile:x}}).
        RedundantNestedFinalUrlTag = 13,
        /// The final url must start with a valid protocol.
        MissingProtocolInFinalUrl = 14,
        /// The final url starts with an invalid protocol.
        InvalidProtocolInFinalUrl = 15,
        /// The final url contains illegal characters.
        MalformedFinalUrl = 16,
        /// The final url must contain a host name.
        MissingHostInFinalUrl = 17,
        /// The tracking url template has an invalid or missing top level domain
        /// extension.
        InvalidTldInFinalUrl = 18,
        /// The final mobile url is invalid.
        InvalidFinalMobileUrl = 19,
        /// The final mobile url contains invalid tag.
        InvalidTagInFinalMobileUrl = 20,
        /// The final mobile url contains nested occurrences of the same conditional
        /// tag (i.e. {ifmobile:{ifmobile:x}}).
        RedundantNestedFinalMobileUrlTag = 21,
        /// The final mobile url must start with a valid protocol.
        MissingProtocolInFinalMobileUrl = 22,
        /// The final mobile url starts with an invalid protocol.
        InvalidProtocolInFinalMobileUrl = 23,
        /// The final mobile url contains illegal characters.
        MalformedFinalMobileUrl = 24,
        /// The final mobile url must contain a host name.
        MissingHostInFinalMobileUrl = 25,
        /// The tracking url template has an invalid or missing top level domain
        /// extension.
        InvalidTldInFinalMobileUrl = 26,
        /// The final app url is invalid.
        InvalidFinalAppUrl = 27,
        /// The final app url contains invalid tag.
        InvalidTagInFinalAppUrl = 28,
        /// The final app url contains nested occurrences of the same conditional tag
        /// (i.e. {ifmobile:{ifmobile:x}}).
        RedundantNestedFinalAppUrlTag = 29,
        /// More than one app url found for the same OS type.
        MultipleAppUrlsForOstype = 30,
        /// The OS type given for an app url is not valid.
        InvalidOstype = 31,
        /// The protocol given for an app url is not valid. (E.g. "android-app://")
        InvalidProtocolForAppUrl = 32,
        /// The package id (app id) given for an app url is not valid.
        InvalidPackageIdForAppUrl = 33,
        /// The number of url custom parameters for an resource exceeds the maximum
        /// limit allowed.
        UrlCustomParametersCountExceedsLimit = 34,
        /// An invalid character appears in the parameter key.
        InvalidCharactersInUrlCustomParameterKey = 39,
        /// An invalid character appears in the parameter value.
        InvalidCharactersInUrlCustomParameterValue = 40,
        /// The url custom parameter value fails url tag validation.
        InvalidTagInUrlCustomParameterValue = 41,
        /// The custom parameter contains nested occurrences of the same conditional
        /// tag (i.e. {ifmobile:{ifmobile:x}}).
        RedundantNestedUrlCustomParameterTag = 42,
        /// The protocol (http:// or https://) is missing.
        MissingProtocol = 43,
        /// Unsupported protocol in URL. Only http and https are supported.
        InvalidProtocol = 52,
        /// The url is invalid.
        InvalidUrl = 44,
        /// Destination Url is deprecated.
        DestinationUrlDeprecated = 45,
        /// The url contains invalid tag.
        InvalidTagInUrl = 46,
        /// The url must contain at least one tag (e.g. {lpurl}).
        MissingUrlTag = 47,
        /// Duplicate url id.
        DuplicateUrlId = 48,
        /// Invalid url id.
        InvalidUrlId = 49,
        /// The final url suffix cannot begin with '?' or '&' characters and must be
        /// a valid query string.
        FinalUrlSuffixMalformed = 50,
        /// The final url suffix cannot contain {lpurl} related or {ignore} tags.
        InvalidTagInFinalUrlSuffix = 51,
        /// The top level domain is invalid, e.g. not a public top level domain
        /// listed in publicsuffix.org.
        InvalidTopLevelDomain = 53,
        /// Malformed top level domain in URL.
        MalformedTopLevelDomain = 54,
        /// Malformed URL.
        MalformedUrl = 55,
        /// No host found in URL.
        MissingHost = 56,
        /// Custom parameter value cannot be null.
        NullCustomParameterValue = 57,
        /// Track parameter is not supported.
        ValueTrackParameterNotSupported = 58,
    }
}
// Proto file describing user data errors.

/// Container for enum describing possible user data errors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserDataErrorEnum {}
/// Nested message and enum types in `UserDataErrorEnum`.
pub mod user_data_error_enum {
    /// Enum describing possible request errors.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum UserDataError {
        /// Enum unspecified.
        Unspecified = 0,
        /// The received error code is not known in this version.
        Unknown = 1,
        /// Customer is not allowed to perform operations related to Customer Match.
        OperationsForCustomerMatchNotAllowed = 2,
        /// Maximum number of user identifiers allowed for each request is 100 and
        /// for each operation is 20.
        TooManyUserIdentifiers = 3,
        /// Current user list is not applicable for the given customer.
        UserListNotApplicable = 4,
    }
}
// Proto file describing user list errors.

/// Container for enum describing possible user list errors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserListErrorEnum {}
/// Nested message and enum types in `UserListErrorEnum`.
pub mod user_list_error_enum {
    /// Enum describing possible user list errors.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum UserListError {
        /// Enum unspecified.
        Unspecified = 0,
        /// The received error code is not known in this version.
        Unknown = 1,
        /// Creating and updating external remarketing user lists is not supported.
        ExternalRemarketingUserListMutateNotSupported = 2,
        /// Concrete type of user list is required.
        ConcreteTypeRequired = 3,
        /// Creating/updating user list conversion types requires specifying the
        /// conversion type Id.
        ConversionTypeIdRequired = 4,
        /// Remarketing user list cannot have duplicate conversion types.
        DuplicateConversionTypes = 5,
        /// Conversion type is invalid/unknown.
        InvalidConversionType = 6,
        /// User list description is empty or invalid.
        InvalidDescription = 7,
        /// User list name is empty or invalid.
        InvalidName = 8,
        /// Type of the UserList does not match.
        InvalidType = 9,
        /// Embedded logical user lists are not allowed.
        CanNotAddLogicalListAsLogicalListOperand = 10,
        /// User list rule operand is invalid.
        InvalidUserListLogicalRuleOperand = 11,
        /// Name is already being used for another user list for the account.
        NameAlreadyUsed = 12,
        /// Name is required when creating a new conversion type.
        NewConversionTypeNameRequired = 13,
        /// The given conversion type name has been used.
        ConversionTypeNameAlreadyUsed = 14,
        /// Only an owner account may edit a user list.
        OwnershipRequiredForSet = 15,
        /// Creating user list without setting type in oneof user_list field, or
        /// creating/updating read-only user list types is not allowed.
        UserListMutateNotSupported = 16,
        /// Rule is invalid.
        InvalidRule = 17,
        /// The specified date range is empty.
        InvalidDateRange = 27,
        /// A UserList which is privacy sensitive or legal rejected cannot be mutated
        /// by external users.
        CanNotMutateSensitiveUserlist = 28,
        /// Maximum number of rulebased user lists a customer can have.
        MaxNumRulebasedUserlists = 29,
        /// BasicUserList's billable record field cannot be modified once it is set.
        CannotModifyBillableRecordCount = 30,
        /// crm_based_user_list.app_id field must be set when upload_key_type is
        /// MOBILE_ADVERTISING_ID.
        AppIdNotSet = 31,
        /// Name of the user list is reserved for system generated lists and cannot
        /// be used.
        UserlistNameIsReservedForSystemList = 32,
        /// Advertiser needs to be on the allow-list to use remarketing lists created
        /// from advertiser uploaded data (e.g., Customer Match lists).
        AdvertiserNotOnAllowlistForUsingUploadedData = 37,
        /// The provided rule_type is not supported for the user list.
        RuleTypeIsNotSupported = 34,
        /// Similar user list cannot be used as a logical user list operand.
        CanNotAddASimilarUserlistAsLogicalListOperand = 35,
        /// Logical user list should not have a mix of CRM based user list and other
        /// types of lists in its rules.
        CanNotMixCrmBasedInLogicalListWithOtherLists = 36,
    }
}
// Proto file describing YouTube video registration errors.

/// Container for enum describing YouTube video registration errors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct YoutubeVideoRegistrationErrorEnum {}
/// Nested message and enum types in `YoutubeVideoRegistrationErrorEnum`.
pub mod youtube_video_registration_error_enum {
    /// Enum describing YouTube video registration errors.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum YoutubeVideoRegistrationError {
        /// Enum unspecified.
        Unspecified = 0,
        /// The received error code is not known in this version.
        Unknown = 1,
        /// Video to be registered wasn't found.
        VideoNotFound = 2,
        /// Video to be registered is not accessible (e.g. private).
        VideoNotAccessible = 3,
        /// Video to be registered is not eligible (e.g. mature content).
        VideoNotEligible = 4,
    }
}
// Proto file describing the common error protos

/// Describes how a GoogleAds API call failed. It's returned inside
/// google.rpc.Status.details when a call fails.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GoogleAdsFailure {
    /// The list of errors that occurred.
    #[prost(message, repeated, tag = "1")]
    pub errors: ::prost::alloc::vec::Vec<GoogleAdsError>,
    /// The unique ID of the request that is used for debugging purposes.
    #[prost(string, tag = "2")]
    pub request_id: ::prost::alloc::string::String,
}
/// GoogleAds-specific error.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GoogleAdsError {
    /// An enum value that indicates which error occurred.
    #[prost(message, optional, tag = "1")]
    pub error_code: ::core::option::Option<ErrorCode>,
    /// A human-readable description of the error.
    #[prost(string, tag = "2")]
    pub message: ::prost::alloc::string::String,
    /// The value that triggered the error.
    #[prost(message, optional, tag = "3")]
    pub trigger: ::core::option::Option<super::common::Value>,
    /// Describes the part of the request proto that caused the error.
    #[prost(message, optional, tag = "4")]
    pub location: ::core::option::Option<ErrorLocation>,
    /// Additional error details, which are returned by certain error codes. Most
    /// error codes do not include details.
    #[prost(message, optional, tag = "5")]
    pub details: ::core::option::Option<ErrorDetails>,
}
/// The error reason represented by type and enum.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ErrorCode {
    /// The list of error enums
    #[prost(
        oneof = "error_code::ErrorCode",
        tags = "1, 2, 3, 4, 5, 7, 8, 9, 10, 11, 12, 13, 14, 15, 17, 161, 18, 159, 19, 21, 24, 25, 107, 149, 155, 148, 153, 154, 152, 26, 29, 166, 160, 31, 165, 109, 32, 150, 158, 90, 151, 33, 34, 35, 36, 37, 38, 39, 40, 110, 42, 116, 86, 162, 44, 45, 46, 47, 48, 49, 58, 51, 52, 53, 54, 55, 56, 57, 117, 59, 60, 61, 62, 63, 64, 65, 115, 143, 111, 145, 146, 66, 67, 68, 70, 71, 72, 132, 74, 133, 76, 77, 78, 136, 79, 80, 81, 82, 83, 84, 87, 88, 91, 92, 93, 94, 96, 97, 98, 100, 101, 102, 103, 140, 141, 104, 105, 112, 114, 118, 119, 137, 121, 122, 123, 124, 125, 126, 127, 128, 129, 130, 131, 134, 135, 138, 139, 164, 156"
    )]
    pub error_code: ::core::option::Option<error_code::ErrorCode>,
}
/// Nested message and enum types in `ErrorCode`.
pub mod error_code {
    /// The list of error enums
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ErrorCode {
        /// An error caused by the request
        #[prost(enumeration = "super::request_error_enum::RequestError", tag = "1")]
        RequestError(i32),
        /// An error with a Bidding Strategy mutate.
        #[prost(
            enumeration = "super::bidding_strategy_error_enum::BiddingStrategyError",
            tag = "2"
        )]
        BiddingStrategyError(i32),
        /// An error with a URL field mutate.
        #[prost(enumeration = "super::url_field_error_enum::UrlFieldError", tag = "3")]
        UrlFieldError(i32),
        /// An error with a list operation.
        #[prost(
            enumeration = "super::list_operation_error_enum::ListOperationError",
            tag = "4"
        )]
        ListOperationError(i32),
        /// An error with an AWQL query
        #[prost(enumeration = "super::query_error_enum::QueryError", tag = "5")]
        QueryError(i32),
        /// An error with a mutate
        #[prost(enumeration = "super::mutate_error_enum::MutateError", tag = "7")]
        MutateError(i32),
        /// An error with a field mask
        #[prost(
            enumeration = "super::field_mask_error_enum::FieldMaskError",
            tag = "8"
        )]
        FieldMaskError(i32),
        /// An error encountered when trying to authorize a user.
        #[prost(
            enumeration = "super::authorization_error_enum::AuthorizationError",
            tag = "9"
        )]
        AuthorizationError(i32),
        /// An unexpected server-side error.
        #[prost(enumeration = "super::internal_error_enum::InternalError", tag = "10")]
        InternalError(i32),
        /// An error with the amonut of quota remaining.
        #[prost(enumeration = "super::quota_error_enum::QuotaError", tag = "11")]
        QuotaError(i32),
        /// An error with an Ad Group Ad mutate.
        #[prost(enumeration = "super::ad_error_enum::AdError", tag = "12")]
        AdError(i32),
        /// An error with an Ad Group mutate.
        #[prost(enumeration = "super::ad_group_error_enum::AdGroupError", tag = "13")]
        AdGroupError(i32),
        /// An error with a Campaign Budget mutate.
        #[prost(
            enumeration = "super::campaign_budget_error_enum::CampaignBudgetError",
            tag = "14"
        )]
        CampaignBudgetError(i32),
        /// An error with a Campaign mutate.
        #[prost(enumeration = "super::campaign_error_enum::CampaignError", tag = "15")]
        CampaignError(i32),
        /// Indicates failure to properly authenticate user.
        #[prost(
            enumeration = "super::authentication_error_enum::AuthenticationError",
            tag = "17"
        )]
        AuthenticationError(i32),
        /// The reasons for the ad group criterion customizer error.
        #[prost(
            enumeration = "super::ad_group_criterion_customizer_error_enum::AdGroupCriterionCustomizerError",
            tag = "161"
        )]
        AdGroupCriterionCustomizerError(i32),
        /// Indicates failure to properly authenticate user.
        #[prost(
            enumeration = "super::ad_group_criterion_error_enum::AdGroupCriterionError",
            tag = "18"
        )]
        AdGroupCriterionError(i32),
        /// The reasons for the ad group customizer error.
        #[prost(
            enumeration = "super::ad_group_customizer_error_enum::AdGroupCustomizerError",
            tag = "159"
        )]
        AdGroupCustomizerError(i32),
        /// The reasons for the ad customizer error
        #[prost(
            enumeration = "super::ad_customizer_error_enum::AdCustomizerError",
            tag = "19"
        )]
        AdCustomizerError(i32),
        /// The reasons for the ad group ad error
        #[prost(
            enumeration = "super::ad_group_ad_error_enum::AdGroupAdError",
            tag = "21"
        )]
        AdGroupAdError(i32),
        /// The reasons for the ad sharing error
        #[prost(
            enumeration = "super::ad_sharing_error_enum::AdSharingError",
            tag = "24"
        )]
        AdSharingError(i32),
        /// The reasons for the adx error
        #[prost(enumeration = "super::adx_error_enum::AdxError", tag = "25")]
        AdxError(i32),
        /// The reasons for the asset error
        #[prost(enumeration = "super::asset_error_enum::AssetError", tag = "107")]
        AssetError(i32),
        /// The reasons for the asset group asset error
        #[prost(
            enumeration = "super::asset_group_asset_error_enum::AssetGroupAssetError",
            tag = "149"
        )]
        AssetGroupAssetError(i32),
        /// The reasons for the asset group listing group filter error
        #[prost(
            enumeration = "super::asset_group_listing_group_filter_error_enum::AssetGroupListingGroupFilterError",
            tag = "155"
        )]
        AssetGroupListingGroupFilterError(i32),
        /// The reasons for the asset group error
        #[prost(
            enumeration = "super::asset_group_error_enum::AssetGroupError",
            tag = "148"
        )]
        AssetGroupError(i32),
        /// The reasons for the asset set asset error
        #[prost(
            enumeration = "super::asset_set_asset_error_enum::AssetSetAssetError",
            tag = "153"
        )]
        AssetSetAssetError(i32),
        /// The reasons for the asset set link error
        #[prost(
            enumeration = "super::asset_set_link_error_enum::AssetSetLinkError",
            tag = "154"
        )]
        AssetSetLinkError(i32),
        /// The reasons for the asset set error
        #[prost(
            enumeration = "super::asset_set_error_enum::AssetSetError",
            tag = "152"
        )]
        AssetSetError(i32),
        /// The reasons for the bidding errors
        #[prost(enumeration = "super::bidding_error_enum::BiddingError", tag = "26")]
        BiddingError(i32),
        /// The reasons for the campaign criterion error
        #[prost(
            enumeration = "super::campaign_criterion_error_enum::CampaignCriterionError",
            tag = "29"
        )]
        CampaignCriterionError(i32),
        /// The reasons for the campaign conversion goal error
        #[prost(
            enumeration = "super::campaign_conversion_goal_error_enum::CampaignConversionGoalError",
            tag = "166"
        )]
        CampaignConversionGoalError(i32),
        /// The reasons for the campaign customizer error.
        #[prost(
            enumeration = "super::campaign_customizer_error_enum::CampaignCustomizerError",
            tag = "160"
        )]
        CampaignCustomizerError(i32),
        /// The reasons for the collection size error
        #[prost(
            enumeration = "super::collection_size_error_enum::CollectionSizeError",
            tag = "31"
        )]
        CollectionSizeError(i32),
        /// The reasons for the conversion goal campaign config error
        #[prost(
            enumeration = "super::conversion_goal_campaign_config_error_enum::ConversionGoalCampaignConfigError",
            tag = "165"
        )]
        ConversionGoalCampaignConfigError(i32),
        /// The reasons for the country code error
        #[prost(
            enumeration = "super::country_code_error_enum::CountryCodeError",
            tag = "109"
        )]
        CountryCodeError(i32),
        /// The reasons for the criterion error
        #[prost(
            enumeration = "super::criterion_error_enum::CriterionError",
            tag = "32"
        )]
        CriterionError(i32),
        /// The reasons for the custom conversion goal error
        #[prost(
            enumeration = "super::custom_conversion_goal_error_enum::CustomConversionGoalError",
            tag = "150"
        )]
        CustomConversionGoalError(i32),
        /// The reasons for the customer customizer error.
        #[prost(
            enumeration = "super::customer_customizer_error_enum::CustomerCustomizerError",
            tag = "158"
        )]
        CustomerCustomizerError(i32),
        /// The reasons for the customer error
        #[prost(enumeration = "super::customer_error_enum::CustomerError", tag = "90")]
        CustomerError(i32),
        /// The reasons for the customizer attribute error.
        #[prost(
            enumeration = "super::customizer_attribute_error_enum::CustomizerAttributeError",
            tag = "151"
        )]
        CustomizerAttributeError(i32),
        /// The reasons for the date error
        #[prost(enumeration = "super::date_error_enum::DateError", tag = "33")]
        DateError(i32),
        /// The reasons for the date range error
        #[prost(
            enumeration = "super::date_range_error_enum::DateRangeError",
            tag = "34"
        )]
        DateRangeError(i32),
        /// The reasons for the distinct error
        #[prost(enumeration = "super::distinct_error_enum::DistinctError", tag = "35")]
        DistinctError(i32),
        /// The reasons for the feed attribute reference error
        #[prost(
            enumeration = "super::feed_attribute_reference_error_enum::FeedAttributeReferenceError",
            tag = "36"
        )]
        FeedAttributeReferenceError(i32),
        /// The reasons for the function error
        #[prost(enumeration = "super::function_error_enum::FunctionError", tag = "37")]
        FunctionError(i32),
        /// The reasons for the function parsing error
        #[prost(
            enumeration = "super::function_parsing_error_enum::FunctionParsingError",
            tag = "38"
        )]
        FunctionParsingError(i32),
        /// The reasons for the id error
        #[prost(enumeration = "super::id_error_enum::IdError", tag = "39")]
        IdError(i32),
        /// The reasons for the image error
        #[prost(enumeration = "super::image_error_enum::ImageError", tag = "40")]
        ImageError(i32),
        /// The reasons for the language code error
        #[prost(
            enumeration = "super::language_code_error_enum::LanguageCodeError",
            tag = "110"
        )]
        LanguageCodeError(i32),
        /// The reasons for the media bundle error
        #[prost(
            enumeration = "super::media_bundle_error_enum::MediaBundleError",
            tag = "42"
        )]
        MediaBundleError(i32),
        /// The reasons for media uploading errors.
        #[prost(
            enumeration = "super::media_upload_error_enum::MediaUploadError",
            tag = "116"
        )]
        MediaUploadError(i32),
        /// The reasons for the media file error
        #[prost(
            enumeration = "super::media_file_error_enum::MediaFileError",
            tag = "86"
        )]
        MediaFileError(i32),
        /// Container for enum describing possible merchant center errors.
        #[prost(
            enumeration = "super::merchant_center_error_enum::MerchantCenterError",
            tag = "162"
        )]
        MerchantCenterError(i32),
        /// The reasons for the multiplier error
        #[prost(
            enumeration = "super::multiplier_error_enum::MultiplierError",
            tag = "44"
        )]
        MultiplierError(i32),
        /// The reasons for the new resource creation error
        #[prost(
            enumeration = "super::new_resource_creation_error_enum::NewResourceCreationError",
            tag = "45"
        )]
        NewResourceCreationError(i32),
        /// The reasons for the not empty error
        #[prost(enumeration = "super::not_empty_error_enum::NotEmptyError", tag = "46")]
        NotEmptyError(i32),
        /// The reasons for the null error
        #[prost(enumeration = "super::null_error_enum::NullError", tag = "47")]
        NullError(i32),
        /// The reasons for the operator error
        #[prost(enumeration = "super::operator_error_enum::OperatorError", tag = "48")]
        OperatorError(i32),
        /// The reasons for the range error
        #[prost(enumeration = "super::range_error_enum::RangeError", tag = "49")]
        RangeError(i32),
        /// The reasons for error in applying a recommendation
        #[prost(
            enumeration = "super::recommendation_error_enum::RecommendationError",
            tag = "58"
        )]
        RecommendationError(i32),
        /// The reasons for the region code error
        #[prost(
            enumeration = "super::region_code_error_enum::RegionCodeError",
            tag = "51"
        )]
        RegionCodeError(i32),
        /// The reasons for the setting error
        #[prost(enumeration = "super::setting_error_enum::SettingError", tag = "52")]
        SettingError(i32),
        /// The reasons for the string format error
        #[prost(
            enumeration = "super::string_format_error_enum::StringFormatError",
            tag = "53"
        )]
        StringFormatError(i32),
        /// The reasons for the string length error
        #[prost(
            enumeration = "super::string_length_error_enum::StringLengthError",
            tag = "54"
        )]
        StringLengthError(i32),
        /// The reasons for the operation access denied error
        #[prost(
            enumeration = "super::operation_access_denied_error_enum::OperationAccessDeniedError",
            tag = "55"
        )]
        OperationAccessDeniedError(i32),
        /// The reasons for the resource access denied error
        #[prost(
            enumeration = "super::resource_access_denied_error_enum::ResourceAccessDeniedError",
            tag = "56"
        )]
        ResourceAccessDeniedError(i32),
        /// The reasons for the resource count limit exceeded error
        #[prost(
            enumeration = "super::resource_count_limit_exceeded_error_enum::ResourceCountLimitExceededError",
            tag = "57"
        )]
        ResourceCountLimitExceededError(i32),
        /// The reasons for YouTube video registration errors.
        #[prost(
            enumeration = "super::youtube_video_registration_error_enum::YoutubeVideoRegistrationError",
            tag = "117"
        )]
        YoutubeVideoRegistrationError(i32),
        /// The reasons for the ad group bid modifier error
        #[prost(
            enumeration = "super::ad_group_bid_modifier_error_enum::AdGroupBidModifierError",
            tag = "59"
        )]
        AdGroupBidModifierError(i32),
        /// The reasons for the context error
        #[prost(enumeration = "super::context_error_enum::ContextError", tag = "60")]
        ContextError(i32),
        /// The reasons for the field error
        #[prost(enumeration = "super::field_error_enum::FieldError", tag = "61")]
        FieldError(i32),
        /// The reasons for the shared set error
        #[prost(
            enumeration = "super::shared_set_error_enum::SharedSetError",
            tag = "62"
        )]
        SharedSetError(i32),
        /// The reasons for the shared criterion error
        #[prost(
            enumeration = "super::shared_criterion_error_enum::SharedCriterionError",
            tag = "63"
        )]
        SharedCriterionError(i32),
        /// The reasons for the campaign shared set error
        #[prost(
            enumeration = "super::campaign_shared_set_error_enum::CampaignSharedSetError",
            tag = "64"
        )]
        CampaignSharedSetError(i32),
        /// The reasons for the conversion action error
        #[prost(
            enumeration = "super::conversion_action_error_enum::ConversionActionError",
            tag = "65"
        )]
        ConversionActionError(i32),
        /// The reasons for the conversion adjustment upload error
        #[prost(
            enumeration = "super::conversion_adjustment_upload_error_enum::ConversionAdjustmentUploadError",
            tag = "115"
        )]
        ConversionAdjustmentUploadError(i32),
        /// The reasons for the conversion custom variable error
        #[prost(
            enumeration = "super::conversion_custom_variable_error_enum::ConversionCustomVariableError",
            tag = "143"
        )]
        ConversionCustomVariableError(i32),
        /// The reasons for the conversion upload error
        #[prost(
            enumeration = "super::conversion_upload_error_enum::ConversionUploadError",
            tag = "111"
        )]
        ConversionUploadError(i32),
        /// The reasons for the conversion value rule error
        #[prost(
            enumeration = "super::conversion_value_rule_error_enum::ConversionValueRuleError",
            tag = "145"
        )]
        ConversionValueRuleError(i32),
        /// The reasons for the conversion value rule set error
        #[prost(
            enumeration = "super::conversion_value_rule_set_error_enum::ConversionValueRuleSetError",
            tag = "146"
        )]
        ConversionValueRuleSetError(i32),
        /// The reasons for the header error.
        #[prost(enumeration = "super::header_error_enum::HeaderError", tag = "66")]
        HeaderError(i32),
        /// The reasons for the database error.
        #[prost(enumeration = "super::database_error_enum::DatabaseError", tag = "67")]
        DatabaseError(i32),
        /// The reasons for the policy finding error.
        #[prost(
            enumeration = "super::policy_finding_error_enum::PolicyFindingError",
            tag = "68"
        )]
        PolicyFindingError(i32),
        /// The reason for enum error.
        #[prost(enumeration = "super::enum_error_enum::EnumError", tag = "70")]
        EnumError(i32),
        /// The reason for keyword plan error.
        #[prost(
            enumeration = "super::keyword_plan_error_enum::KeywordPlanError",
            tag = "71"
        )]
        KeywordPlanError(i32),
        /// The reason for keyword plan campaign error.
        #[prost(
            enumeration = "super::keyword_plan_campaign_error_enum::KeywordPlanCampaignError",
            tag = "72"
        )]
        KeywordPlanCampaignError(i32),
        /// The reason for keyword plan campaign keyword error.
        #[prost(
            enumeration = "super::keyword_plan_campaign_keyword_error_enum::KeywordPlanCampaignKeywordError",
            tag = "132"
        )]
        KeywordPlanCampaignKeywordError(i32),
        /// The reason for keyword plan ad group error.
        #[prost(
            enumeration = "super::keyword_plan_ad_group_error_enum::KeywordPlanAdGroupError",
            tag = "74"
        )]
        KeywordPlanAdGroupError(i32),
        /// The reason for keyword plan ad group keyword error.
        #[prost(
            enumeration = "super::keyword_plan_ad_group_keyword_error_enum::KeywordPlanAdGroupKeywordError",
            tag = "133"
        )]
        KeywordPlanAdGroupKeywordError(i32),
        /// The reason for keyword idea error.
        #[prost(
            enumeration = "super::keyword_plan_idea_error_enum::KeywordPlanIdeaError",
            tag = "76"
        )]
        KeywordPlanIdeaError(i32),
        /// The reasons for account budget proposal errors.
        #[prost(
            enumeration = "super::account_budget_proposal_error_enum::AccountBudgetProposalError",
            tag = "77"
        )]
        AccountBudgetProposalError(i32),
        /// The reasons for the user list error
        #[prost(enumeration = "super::user_list_error_enum::UserListError", tag = "78")]
        UserListError(i32),
        /// The reasons for the change event error
        #[prost(
            enumeration = "super::change_event_error_enum::ChangeEventError",
            tag = "136"
        )]
        ChangeEventError(i32),
        /// The reasons for the change status error
        #[prost(
            enumeration = "super::change_status_error_enum::ChangeStatusError",
            tag = "79"
        )]
        ChangeStatusError(i32),
        /// The reasons for the feed error
        #[prost(enumeration = "super::feed_error_enum::FeedError", tag = "80")]
        FeedError(i32),
        /// The reasons for the geo target constant suggestion error.
        #[prost(
            enumeration = "super::geo_target_constant_suggestion_error_enum::GeoTargetConstantSuggestionError",
            tag = "81"
        )]
        GeoTargetConstantSuggestionError(i32),
        /// The reasons for the campaign draft error
        #[prost(
            enumeration = "super::campaign_draft_error_enum::CampaignDraftError",
            tag = "82"
        )]
        CampaignDraftError(i32),
        /// The reasons for the feed item error
        #[prost(enumeration = "super::feed_item_error_enum::FeedItemError", tag = "83")]
        FeedItemError(i32),
        /// The reason for the label error.
        #[prost(enumeration = "super::label_error_enum::LabelError", tag = "84")]
        LabelError(i32),
        /// The reasons for the billing setup error
        #[prost(
            enumeration = "super::billing_setup_error_enum::BillingSetupError",
            tag = "87"
        )]
        BillingSetupError(i32),
        /// The reasons for the customer client link error
        #[prost(
            enumeration = "super::customer_client_link_error_enum::CustomerClientLinkError",
            tag = "88"
        )]
        CustomerClientLinkError(i32),
        /// The reasons for the customer manager link error
        #[prost(
            enumeration = "super::customer_manager_link_error_enum::CustomerManagerLinkError",
            tag = "91"
        )]
        CustomerManagerLinkError(i32),
        /// The reasons for the feed mapping error
        #[prost(
            enumeration = "super::feed_mapping_error_enum::FeedMappingError",
            tag = "92"
        )]
        FeedMappingError(i32),
        /// The reasons for the customer feed error
        #[prost(
            enumeration = "super::customer_feed_error_enum::CustomerFeedError",
            tag = "93"
        )]
        CustomerFeedError(i32),
        /// The reasons for the ad group feed error
        #[prost(
            enumeration = "super::ad_group_feed_error_enum::AdGroupFeedError",
            tag = "94"
        )]
        AdGroupFeedError(i32),
        /// The reasons for the campaign feed error
        #[prost(
            enumeration = "super::campaign_feed_error_enum::CampaignFeedError",
            tag = "96"
        )]
        CampaignFeedError(i32),
        /// The reasons for the custom interest error
        #[prost(
            enumeration = "super::custom_interest_error_enum::CustomInterestError",
            tag = "97"
        )]
        CustomInterestError(i32),
        /// The reasons for the campaign experiment error
        #[prost(
            enumeration = "super::campaign_experiment_error_enum::CampaignExperimentError",
            tag = "98"
        )]
        CampaignExperimentError(i32),
        /// The reasons for the extension feed item error
        #[prost(
            enumeration = "super::extension_feed_item_error_enum::ExtensionFeedItemError",
            tag = "100"
        )]
        ExtensionFeedItemError(i32),
        /// The reasons for the ad parameter error
        #[prost(
            enumeration = "super::ad_parameter_error_enum::AdParameterError",
            tag = "101"
        )]
        AdParameterError(i32),
        /// The reasons for the feed item validation error
        #[prost(
            enumeration = "super::feed_item_validation_error_enum::FeedItemValidationError",
            tag = "102"
        )]
        FeedItemValidationError(i32),
        /// The reasons for the extension setting error
        #[prost(
            enumeration = "super::extension_setting_error_enum::ExtensionSettingError",
            tag = "103"
        )]
        ExtensionSettingError(i32),
        /// The reasons for the feed item set error
        #[prost(
            enumeration = "super::feed_item_set_error_enum::FeedItemSetError",
            tag = "140"
        )]
        FeedItemSetError(i32),
        /// The reasons for the feed item set link error
        #[prost(
            enumeration = "super::feed_item_set_link_error_enum::FeedItemSetLinkError",
            tag = "141"
        )]
        FeedItemSetLinkError(i32),
        /// The reasons for the feed item target error
        #[prost(
            enumeration = "super::feed_item_target_error_enum::FeedItemTargetError",
            tag = "104"
        )]
        FeedItemTargetError(i32),
        /// The reasons for the policy violation error
        #[prost(
            enumeration = "super::policy_violation_error_enum::PolicyViolationError",
            tag = "105"
        )]
        PolicyViolationError(i32),
        /// The reasons for the mutate job error
        #[prost(
            enumeration = "super::partial_failure_error_enum::PartialFailureError",
            tag = "112"
        )]
        PartialFailureError(i32),
        /// The reasons for the policy validation parameter error
        #[prost(
            enumeration = "super::policy_validation_parameter_error_enum::PolicyValidationParameterError",
            tag = "114"
        )]
        PolicyValidationParameterError(i32),
        /// The reasons for the size limit error
        #[prost(
            enumeration = "super::size_limit_error_enum::SizeLimitError",
            tag = "118"
        )]
        SizeLimitError(i32),
        /// The reasons for the offline user data job error.
        #[prost(
            enumeration = "super::offline_user_data_job_error_enum::OfflineUserDataJobError",
            tag = "119"
        )]
        OfflineUserDataJobError(i32),
        /// The reasons for the not allowlisted error
        #[prost(
            enumeration = "super::not_allowlisted_error_enum::NotAllowlistedError",
            tag = "137"
        )]
        NotAllowlistedError(i32),
        /// The reasons for the manager link error
        #[prost(
            enumeration = "super::manager_link_error_enum::ManagerLinkError",
            tag = "121"
        )]
        ManagerLinkError(i32),
        /// The reasons for the currency code error
        #[prost(
            enumeration = "super::currency_code_error_enum::CurrencyCodeError",
            tag = "122"
        )]
        CurrencyCodeError(i32),
        /// The reasons for the experiment error
        #[prost(
            enumeration = "super::experiment_error_enum::ExperimentError",
            tag = "123"
        )]
        ExperimentError(i32),
        /// The reasons for the access invitation error
        #[prost(
            enumeration = "super::access_invitation_error_enum::AccessInvitationError",
            tag = "124"
        )]
        AccessInvitationError(i32),
        /// The reasons for the reach plan error
        #[prost(
            enumeration = "super::reach_plan_error_enum::ReachPlanError",
            tag = "125"
        )]
        ReachPlanError(i32),
        /// The reasons for the invoice error
        #[prost(enumeration = "super::invoice_error_enum::InvoiceError", tag = "126")]
        InvoiceError(i32),
        /// The reasons for errors in payments accounts service
        #[prost(
            enumeration = "super::payments_account_error_enum::PaymentsAccountError",
            tag = "127"
        )]
        PaymentsAccountError(i32),
        /// The reasons for the time zone error
        #[prost(
            enumeration = "super::time_zone_error_enum::TimeZoneError",
            tag = "128"
        )]
        TimeZoneError(i32),
        /// The reasons for the asset link error
        #[prost(
            enumeration = "super::asset_link_error_enum::AssetLinkError",
            tag = "129"
        )]
        AssetLinkError(i32),
        /// The reasons for the user data error.
        #[prost(
            enumeration = "super::user_data_error_enum::UserDataError",
            tag = "130"
        )]
        UserDataError(i32),
        /// The reasons for the batch job error
        #[prost(
            enumeration = "super::batch_job_error_enum::BatchJobError",
            tag = "131"
        )]
        BatchJobError(i32),
        /// The reasons for the account link status change error
        #[prost(
            enumeration = "super::account_link_error_enum::AccountLinkError",
            tag = "134"
        )]
        AccountLinkError(i32),
        /// The reasons for the third party app analytics link mutate error
        #[prost(
            enumeration = "super::third_party_app_analytics_link_error_enum::ThirdPartyAppAnalyticsLinkError",
            tag = "135"
        )]
        ThirdPartyAppAnalyticsLinkError(i32),
        /// The reasons for the customer user access mutate error
        #[prost(
            enumeration = "super::customer_user_access_error_enum::CustomerUserAccessError",
            tag = "138"
        )]
        CustomerUserAccessError(i32),
        /// The reasons for the custom audience error
        #[prost(
            enumeration = "super::custom_audience_error_enum::CustomAudienceError",
            tag = "139"
        )]
        CustomAudienceError(i32),
        /// The reasons for the audience error
        #[prost(enumeration = "super::audience_error_enum::AudienceError", tag = "164")]
        AudienceError(i32),
        /// The reasons for the experiment arm error
        #[prost(
            enumeration = "super::experiment_arm_error_enum::ExperimentArmError",
            tag = "156"
        )]
        ExperimentArmError(i32),
    }
}
/// Describes the part of the request proto that caused the error.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ErrorLocation {
    /// A field path that indicates which field was invalid in the request.
    #[prost(message, repeated, tag = "2")]
    pub field_path_elements: ::prost::alloc::vec::Vec<error_location::FieldPathElement>,
}
/// Nested message and enum types in `ErrorLocation`.
pub mod error_location {
    /// A part of a field path.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct FieldPathElement {
        /// The name of a field or a oneof
        #[prost(string, tag = "1")]
        pub field_name: ::prost::alloc::string::String,
        /// If field_name is a repeated field, this is the element that failed
        #[prost(int32, optional, tag = "3")]
        pub index: ::core::option::Option<i32>,
    }
}
/// Additional error details.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ErrorDetails {
    /// The error code that should have been returned, but wasn't. This is used
    /// when the error code is not published in the client specified version.
    #[prost(string, tag = "1")]
    pub unpublished_error_code: ::prost::alloc::string::String,
    /// Describes an ad policy violation.
    #[prost(message, optional, tag = "2")]
    pub policy_violation_details: ::core::option::Option<PolicyViolationDetails>,
    /// Describes policy violation findings.
    #[prost(message, optional, tag = "3")]
    pub policy_finding_details: ::core::option::Option<PolicyFindingDetails>,
    /// Details on the quota error, including the scope (account or developer), the
    /// rate bucket name and the retry delay.
    #[prost(message, optional, tag = "4")]
    pub quota_error_details: ::core::option::Option<QuotaErrorDetails>,
    /// Details for a resource count limit exceeded error.
    #[prost(message, optional, tag = "5")]
    pub resource_count_details: ::core::option::Option<ResourceCountDetails>,
}
/// Error returned as part of a mutate response.
/// This error indicates single policy violation by some text
/// in one of the fields.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PolicyViolationDetails {
    /// Human readable description of policy violation.
    #[prost(string, tag = "2")]
    pub external_policy_description: ::prost::alloc::string::String,
    /// Unique identifier for this violation.
    /// If policy is exemptible, this key may be used to request exemption.
    #[prost(message, optional, tag = "4")]
    pub key: ::core::option::Option<super::common::PolicyViolationKey>,
    /// Human readable name of the policy.
    #[prost(string, tag = "5")]
    pub external_policy_name: ::prost::alloc::string::String,
    /// Whether user can file an exemption request for this violation.
    #[prost(bool, tag = "6")]
    pub is_exemptible: bool,
}
/// Error returned as part of a mutate response.
/// This error indicates one or more policy findings in the fields of a
/// resource.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PolicyFindingDetails {
    /// The list of policy topics for the resource. Contains the PROHIBITED or
    /// FULLY_LIMITED policy topic entries that prevented the resource from being
    /// saved (among any other entries the resource may also have).
    #[prost(message, repeated, tag = "1")]
    pub policy_topic_entries: ::prost::alloc::vec::Vec<super::common::PolicyTopicEntry>,
}
/// Additional quota error details when there is QuotaError.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuotaErrorDetails {
    /// The rate scope of the quota limit.
    #[prost(enumeration = "quota_error_details::QuotaRateScope", tag = "1")]
    pub rate_scope: i32,
    /// The high level description of the quota bucket.
    /// Examples are "Get requests for standard access" or "Requests per account".
    #[prost(string, tag = "2")]
    pub rate_name: ::prost::alloc::string::String,
    /// Backoff period that customers should wait before sending next request.
    #[prost(message, optional, tag = "3")]
    pub retry_delay: ::core::option::Option<::prost_types::Duration>,
}
/// Nested message and enum types in `QuotaErrorDetails`.
pub mod quota_error_details {
    /// Enum of possible scopes that quota buckets belong to.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum QuotaRateScope {
        /// Unspecified enum
        Unspecified = 0,
        /// Used for return value only. Represents value unknown in this version.
        Unknown = 1,
        /// Per customer account quota
        Account = 2,
        /// Per project or DevToken quota
        Developer = 3,
    }
}
/// Error details returned when an resource count limit was exceeded.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResourceCountDetails {
    /// The ID of the resource whose limit was exceeded.
    /// External customer ID if the limit is for a customer.
    #[prost(string, tag = "1")]
    pub enclosing_id: ::prost::alloc::string::String,
    /// The name of the resource (Customer, Campaign etc.) whose limit was
    /// exceeded.
    #[prost(string, tag = "5")]
    pub enclosing_resource: ::prost::alloc::string::String,
    /// The limit which was exceeded.
    #[prost(int32, tag = "2")]
    pub limit: i32,
    /// The resource limit type which was exceeded.
    #[prost(
        enumeration = "super::enums::resource_limit_type_enum::ResourceLimitType",
        tag = "3"
    )]
    pub limit_type: i32,
    /// The count of existing entities.
    #[prost(int32, tag = "4")]
    pub existing_count: i32,
}
