// Proto file describing range errors.

/// Container for enum describing possible range errors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RangeErrorEnum {}
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
pub mod reach_plan_error_enum {
    /// Enum describing possible errors from ReachPlanService.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ReachPlanError {
        /// Enum unspecified.
        Unspecified = 0,
        /// The received error code is not known in this version.
        Unknown = 1,
    }
}
// Proto file describing errors from applying a recommendation.

/// Container for enum describing possible errors from applying a recommendation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RecommendationErrorEnum {}
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
    }
}
// Proto file describing resource access denied errors.

/// Container for enum describing possible resource access denied errors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResourceAccessDeniedErrorEnum {}
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
// Proto file describing time zone errors.

/// Container for enum describing possible time zone errors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TimeZoneErrorEnum {}
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
        /// The url must contain at least one tag (e.g. {lpurl}), This applies only
        /// to urls associated with website ads or product ads.
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
        /// The top level domain is invalid, e.g, not a public top level domain
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
    }
}
// Proto file describing user data errors.

/// Container for enum describing possible user data errors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserDataErrorEnum {}
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
        /// Maximum number of user identifiers allowed for each mutate is 100.
        TooManyUserIdentifiers = 3,
        /// Current user list is not applicable for the given customer.
        UserListNotApplicable = 4,
    }
}
// Proto file describing user list errors.

/// Container for enum describing possible user list errors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserListErrorEnum {}
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
        /// Advertiser needs to be whitelisted to use remarketing lists created from
        /// advertiser uploaded data (e.g., Customer Match lists).
        AdvertiserNotWhitelistedForUsingUploadedData = 33,
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
