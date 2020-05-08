/// Message representing a period of time between two timestamps.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TimestampRange {
    /// Begin of the period (inclusive).
    #[prost(message, optional, tag = "1")]
    pub start_time: ::std::option::Option<::prost_types::Timestamp>,
    /// End of the period (exclusive).
    #[prost(message, optional, tag = "2")]
    pub end_time: ::std::option::Option<::prost_types::Timestamp>,
}
/// A resource that represents a location with full geographic information.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Location {
    /// The type of a location, which corresponds to the address lines field of
    /// [google.type.PostalAddress][google.type.PostalAddress]. For example, "Downtown, Atlanta, GA, USA"
    /// has a type of [LocationType.NEIGHBORHOOD][google.cloud.talent.v4beta1.Location.LocationType.NEIGHBORHOOD], and "Kansas City, KS, USA"
    /// has a type of [LocationType.LOCALITY][google.cloud.talent.v4beta1.Location.LocationType.LOCALITY].
    #[prost(enumeration = "location::LocationType", tag = "1")]
    pub location_type: i32,
    /// Postal address of the location that includes human readable information,
    /// such as postal delivery and payments addresses. Given a postal address,
    /// a postal service can deliver items to a premises, P.O. Box, or other
    /// delivery location.
    #[prost(message, optional, tag = "2")]
    pub postal_address: ::std::option::Option<super::super::super::r#type::PostalAddress>,
    /// An object representing a latitude/longitude pair.
    #[prost(message, optional, tag = "3")]
    pub lat_lng: ::std::option::Option<super::super::super::r#type::LatLng>,
    /// Radius in miles of the job location. This value is derived from the
    /// location bounding box in which a circle with the specified radius
    /// centered from [google.type.LatLng][google.type.LatLng] covers the area associated with the
    /// job location.
    /// For example, currently, "Mountain View, CA, USA" has a radius of
    /// 6.17 miles.
    #[prost(double, tag = "4")]
    pub radius_miles: f64,
}
pub mod location {
    /// An enum which represents the type of a location.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum LocationType {
        /// Default value if the type isn't specified.
        Unspecified = 0,
        /// A country level location.
        Country = 1,
        /// A state or equivalent level location.
        AdministrativeArea = 2,
        /// A county or equivalent level location.
        SubAdministrativeArea = 3,
        /// A city or equivalent level location.
        Locality = 4,
        /// A postal code level location.
        PostalCode = 5,
        /// A sublocality is a subdivision of a locality, for example a city borough,
        /// ward, or arrondissement. Sublocalities are usually recognized by a local
        /// political authority. For example, Manhattan and Brooklyn are recognized
        /// as boroughs by the City of New York, and are therefore modeled as
        /// sublocalities.
        SubLocality = 6,
        /// A district or equivalent level location.
        SubLocality1 = 7,
        /// A smaller district or equivalent level display.
        SubLocality2 = 8,
        /// A neighborhood level location.
        Neighborhood = 9,
        /// A street address level location.
        StreetAddress = 10,
    }
}
/// Meta information related to the job searcher or entity
/// conducting the job search. This information is used to improve the
/// performance of the service.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestMetadata {
    /// Required if [allow_missing_ids][google.cloud.talent.v4beta1.RequestMetadata.allow_missing_ids] is unset or `false`.
    ///
    /// The client-defined scope or source of the service call, which typically
    /// is the domain on
    /// which the service has been implemented and is currently being run.
    ///
    /// For example, if the service is being run by client <em>Foo, Inc.</em>, on
    /// job board www.foo.com and career site www.bar.com, then this field is
    /// set to "foo.com" for use on the job board, and "bar.com" for use on the
    /// career site.
    ///
    /// Note that any improvements to the model for a particular tenant site rely
    /// on this field being set correctly to a unique domain.
    ///
    /// The maximum number of allowed characters is 255.
    #[prost(string, tag = "1")]
    pub domain: std::string::String,
    /// Required if [allow_missing_ids][google.cloud.talent.v4beta1.RequestMetadata.allow_missing_ids] is unset or `false`.
    ///
    /// A unique session identification string. A session is defined as the
    /// duration of an end user's interaction with the service over a certain
    /// period.
    /// Obfuscate this field for privacy concerns before
    /// providing it to the service.
    ///
    /// Note that any improvements to the model for a particular tenant site rely
    /// on this field being set correctly to a unique session ID.
    ///
    /// The maximum number of allowed characters is 255.
    #[prost(string, tag = "2")]
    pub session_id: std::string::String,
    /// Required if [allow_missing_ids][google.cloud.talent.v4beta1.RequestMetadata.allow_missing_ids] is unset or `false`.
    ///
    /// A unique user identification string, as determined by the client.
    /// To have the strongest positive impact on search quality
    /// make sure the client-level is unique.
    /// Obfuscate this field for privacy concerns before
    /// providing it to the service.
    ///
    /// Note that any improvements to the model for a particular tenant site rely
    /// on this field being set correctly to a unique user ID.
    ///
    /// The maximum number of allowed characters is 255.
    #[prost(string, tag = "3")]
    pub user_id: std::string::String,
    /// Only set when any of [domain][google.cloud.talent.v4beta1.RequestMetadata.domain], [session_id][google.cloud.talent.v4beta1.RequestMetadata.session_id] and [user_id][google.cloud.talent.v4beta1.RequestMetadata.user_id] isn't
    /// available for some reason. It is highly recommended not to set this field
    /// and provide accurate [domain][google.cloud.talent.v4beta1.RequestMetadata.domain], [session_id][google.cloud.talent.v4beta1.RequestMetadata.session_id] and [user_id][google.cloud.talent.v4beta1.RequestMetadata.user_id] for the
    /// best service experience.
    #[prost(bool, tag = "4")]
    pub allow_missing_ids: bool,
    /// The type of device used by the job seeker at the time of the call to the
    /// service.
    #[prost(message, optional, tag = "5")]
    pub device_info: ::std::option::Option<DeviceInfo>,
}
/// Additional information returned to client, such as debugging information.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseMetadata {
    /// A unique id associated with this call.
    /// This id is logged for tracking purposes.
    #[prost(string, tag = "1")]
    pub request_id: std::string::String,
}
/// Device information collected from the job seeker, candidate, or
/// other entity conducting the job search. Providing this information improves
/// the quality of the search results across devices.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeviceInfo {
    /// Type of the device.
    #[prost(enumeration = "device_info::DeviceType", tag = "1")]
    pub device_type: i32,
    /// A device-specific ID. The ID must be a unique identifier that
    /// distinguishes the device from other devices.
    #[prost(string, tag = "2")]
    pub id: std::string::String,
}
pub mod device_info {
    /// An enumeration describing an API access portal and exposure mechanism.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum DeviceType {
        /// The device type isn't specified.
        Unspecified = 0,
        /// A desktop web browser, such as, Chrome, Firefox, Safari, or Internet
        /// Explorer)
        Web = 1,
        /// A mobile device web browser, such as a phone or tablet with a Chrome
        /// browser.
        MobileWeb = 2,
        /// An Android device native application.
        Android = 3,
        /// An iOS device native application.
        Ios = 4,
        /// A bot, as opposed to a device operated by human beings, such as a web
        /// crawler.
        Bot = 5,
        /// Other devices types.
        Other = 6,
    }
}
/// Custom attribute values that are either filterable or non-filterable.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomAttribute {
    /// Exactly one of [string_values][google.cloud.talent.v4beta1.CustomAttribute.string_values] or [long_values][google.cloud.talent.v4beta1.CustomAttribute.long_values] must be specified.
    ///
    /// This field is used to perform a string match (`CASE_SENSITIVE_MATCH` or
    /// `CASE_INSENSITIVE_MATCH`) search.
    /// For filterable `string_value`s, a maximum total number of 200 values
    /// is allowed, with each `string_value` has a byte size of no more than
    /// 500B. For unfilterable `string_values`, the maximum total byte size of
    /// unfilterable `string_values` is 50KB.
    ///
    /// Empty string isn't allowed.
    #[prost(string, repeated, tag = "1")]
    pub string_values: ::std::vec::Vec<std::string::String>,
    /// Exactly one of [string_values][google.cloud.talent.v4beta1.CustomAttribute.string_values] or [long_values][google.cloud.talent.v4beta1.CustomAttribute.long_values] must be specified.
    ///
    /// This field is used to perform number range search.
    /// (`EQ`, `GT`, `GE`, `LE`, `LT`) over filterable `long_value`.
    ///
    /// Currently at most 1 [long_values][google.cloud.talent.v4beta1.CustomAttribute.long_values] is supported.
    #[prost(int64, repeated, tag = "2")]
    pub long_values: ::std::vec::Vec<i64>,
    /// If the `filterable` flag is true, custom field values are searchable.
    /// If false, values are not searchable.
    ///
    /// Default is false.
    #[prost(bool, tag = "3")]
    pub filterable: bool,
}
/// Spell check result.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpellingCorrection {
    /// Indicates if the query was corrected by the spell checker.
    #[prost(bool, tag = "1")]
    pub corrected: bool,
    /// Correction output consisting of the corrected keyword string.
    #[prost(string, tag = "2")]
    pub corrected_text: std::string::String,
    /// Corrected output with html tags to highlight the corrected words.
    /// Corrected words are called out with the "<b><i>...</i></b>" html tags.
    ///
    /// For example, the user input query is "software enginear", where the second
    /// word, "enginear," is incorrect. It should be "engineer". When spelling
    /// correction is enabled, this value is
    /// "software <b><i>engineer</i></b>".
    #[prost(string, tag = "3")]
    pub corrected_html: std::string::String,
}
/// Job compensation details.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CompensationInfo {
    /// Job compensation information.
    ///
    /// At most one entry can be of type
    /// [CompensationInfo.CompensationType.BASE][google.cloud.talent.v4beta1.CompensationInfo.CompensationType.BASE], which is
    /// referred as **base compensation entry** for the job.
    #[prost(message, repeated, tag = "1")]
    pub entries: ::std::vec::Vec<compensation_info::CompensationEntry>,
    /// Output only. Annualized base compensation range. Computed as base compensation entry's
    /// [CompensationEntry.amount][google.cloud.talent.v4beta1.CompensationInfo.CompensationEntry.amount] times
    /// [CompensationEntry.expected_units_per_year][google.cloud.talent.v4beta1.CompensationInfo.CompensationEntry.expected_units_per_year].
    ///
    /// See [CompensationEntry][google.cloud.talent.v4beta1.CompensationInfo.CompensationEntry] for explanation on compensation annualization.
    #[prost(message, optional, tag = "2")]
    pub annualized_base_compensation_range:
        ::std::option::Option<compensation_info::CompensationRange>,
    /// Output only. Annualized total compensation range. Computed as all compensation entries'
    /// [CompensationEntry.amount][google.cloud.talent.v4beta1.CompensationInfo.CompensationEntry.amount] times
    /// [CompensationEntry.expected_units_per_year][google.cloud.talent.v4beta1.CompensationInfo.CompensationEntry.expected_units_per_year].
    ///
    /// See [CompensationEntry][google.cloud.talent.v4beta1.CompensationInfo.CompensationEntry] for explanation on compensation annualization.
    #[prost(message, optional, tag = "3")]
    pub annualized_total_compensation_range:
        ::std::option::Option<compensation_info::CompensationRange>,
}
pub mod compensation_info {
    /// A compensation entry that represents one component of compensation, such
    /// as base pay, bonus, or other compensation type.
    ///
    /// Annualization: One compensation entry can be annualized if
    /// - it contains valid [amount][google.cloud.talent.v4beta1.CompensationInfo.CompensationEntry.amount] or [range][google.cloud.talent.v4beta1.CompensationInfo.CompensationEntry.range].
    /// - and its [expected_units_per_year][google.cloud.talent.v4beta1.CompensationInfo.CompensationEntry.expected_units_per_year] is set or can be derived.
    /// Its annualized range is determined as ([amount][google.cloud.talent.v4beta1.CompensationInfo.CompensationEntry.amount] or [range][google.cloud.talent.v4beta1.CompensationInfo.CompensationEntry.range]) times
    /// [expected_units_per_year][google.cloud.talent.v4beta1.CompensationInfo.CompensationEntry.expected_units_per_year].
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CompensationEntry {
        /// Compensation type.
        ///
        /// Default is [CompensationType.COMPENSATION_TYPE_UNSPECIFIED][google.cloud.talent.v4beta1.CompensationInfo.CompensationType.COMPENSATION_TYPE_UNSPECIFIED].
        #[prost(enumeration = "CompensationType", tag = "1")]
        pub r#type: i32,
        /// Frequency of the specified amount.
        ///
        /// Default is [CompensationUnit.COMPENSATION_UNIT_UNSPECIFIED][google.cloud.talent.v4beta1.CompensationInfo.CompensationUnit.COMPENSATION_UNIT_UNSPECIFIED].
        #[prost(enumeration = "CompensationUnit", tag = "2")]
        pub unit: i32,
        /// Compensation description.  For example, could
        /// indicate equity terms or provide additional context to an estimated
        /// bonus.
        #[prost(string, tag = "5")]
        pub description: std::string::String,
        /// Expected number of units paid each year. If not specified, when
        /// [Job.employment_types][google.cloud.talent.v4beta1.Job.employment_types] is FULLTIME, a default value is inferred
        /// based on [unit][google.cloud.talent.v4beta1.CompensationInfo.CompensationEntry.unit]. Default values:
        /// - HOURLY: 2080
        /// - DAILY: 260
        /// - WEEKLY: 52
        /// - MONTHLY: 12
        /// - ANNUAL: 1
        #[prost(message, optional, tag = "6")]
        pub expected_units_per_year: ::std::option::Option<f64>,
        /// Compensation amount. It could be a fixed amount or a floating range.
        #[prost(oneof = "compensation_entry::CompensationAmount", tags = "3, 4")]
        pub compensation_amount: ::std::option::Option<compensation_entry::CompensationAmount>,
    }
    pub mod compensation_entry {
        /// Compensation amount. It could be a fixed amount or a floating range.
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum CompensationAmount {
            /// Compensation amount.
            #[prost(message, tag = "3")]
            Amount(super::super::super::super::super::r#type::Money),
            /// Compensation range.
            #[prost(message, tag = "4")]
            Range(super::CompensationRange),
        }
    }
    /// Compensation range.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CompensationRange {
        /// The maximum amount of compensation. If left empty, the value is set
        /// to a maximal compensation value and the currency code is set to
        /// match the [currency code][google.type.Money.currency_code] of
        /// min_compensation.
        #[prost(message, optional, tag = "2")]
        pub max_compensation: ::std::option::Option<super::super::super::super::r#type::Money>,
        /// The minimum amount of compensation. If left empty, the value is set
        /// to zero and the currency code is set to match the
        /// [currency code][google.type.Money.currency_code] of max_compensation.
        #[prost(message, optional, tag = "1")]
        pub min_compensation: ::std::option::Option<super::super::super::super::r#type::Money>,
    }
    /// The type of compensation.
    ///
    /// For compensation amounts specified in non-monetary amounts,
    /// describe the compensation scheme in the [CompensationEntry.description][google.cloud.talent.v4beta1.CompensationInfo.CompensationEntry.description].
    ///
    /// For example, tipping format is described in
    /// [CompensationEntry.description][google.cloud.talent.v4beta1.CompensationInfo.CompensationEntry.description] (for example, "expect 15-20% tips based
    /// on customer bill.") and an estimate of the tips provided in
    /// [CompensationEntry.amount][google.cloud.talent.v4beta1.CompensationInfo.CompensationEntry.amount] or [CompensationEntry.range][google.cloud.talent.v4beta1.CompensationInfo.CompensationEntry.range] ($10 per hour).
    ///
    /// For example, equity is described in [CompensationEntry.description][google.cloud.talent.v4beta1.CompensationInfo.CompensationEntry.description]
    /// (for example, "1% - 2% equity vesting over 4 years, 1 year cliff") and
    /// value estimated in [CompensationEntry.amount][google.cloud.talent.v4beta1.CompensationInfo.CompensationEntry.amount] or
    /// [CompensationEntry.range][google.cloud.talent.v4beta1.CompensationInfo.CompensationEntry.range]. If no value estimate is possible, units are
    /// [CompensationUnit.COMPENSATION_UNIT_UNSPECIFIED][google.cloud.talent.v4beta1.CompensationInfo.CompensationUnit.COMPENSATION_UNIT_UNSPECIFIED] and then further
    /// clarified in [CompensationEntry.description][google.cloud.talent.v4beta1.CompensationInfo.CompensationEntry.description] field.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum CompensationType {
        /// Default value.
        Unspecified = 0,
        /// Base compensation: Refers to the fixed amount of money paid to an
        /// employee by an employer in return for work performed. Base compensation
        /// does not include benefits, bonuses or any other potential compensation
        /// from an employer.
        Base = 1,
        /// Bonus.
        Bonus = 2,
        /// Signing bonus.
        SigningBonus = 3,
        /// Equity.
        Equity = 4,
        /// Profit sharing.
        ProfitSharing = 5,
        /// Commission.
        Commissions = 6,
        /// Tips.
        Tips = 7,
        /// Other compensation type.
        OtherCompensationType = 8,
    }
    /// Pay frequency.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum CompensationUnit {
        /// Default value.
        Unspecified = 0,
        /// Hourly.
        Hourly = 1,
        /// Daily.
        Daily = 2,
        /// Weekly
        Weekly = 3,
        /// Monthly.
        Monthly = 4,
        /// Yearly.
        Yearly = 5,
        /// One time.
        OneTime = 6,
        /// Other compensation units.
        OtherCompensationUnit = 7,
    }
}
/// Resource that represents a license or certification.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Certification {
    /// Name of license or certification.
    ///
    /// Number of characters allowed is 100.
    #[prost(string, tag = "1")]
    pub display_name: std::string::String,
    /// Acquisition date or effective date of license or certification.
    #[prost(message, optional, tag = "2")]
    pub acquire_date: ::std::option::Option<super::super::super::r#type::Date>,
    /// Expiration date of license of certification.
    #[prost(message, optional, tag = "3")]
    pub expire_date: ::std::option::Option<super::super::super::r#type::Date>,
    /// Authority of license, such as government.
    ///
    /// Number of characters allowed is 100.
    #[prost(string, tag = "4")]
    pub authority: std::string::String,
    /// Description of license or certification.
    ///
    /// Number of characters allowed is 100,000.
    #[prost(string, tag = "5")]
    pub description: std::string::String,
}
/// Resource that represents a skill of a candidate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Skill {
    /// Skill display name.
    ///
    /// For example, "Java", "Python".
    ///
    /// Number of characters allowed is 100.
    #[prost(string, tag = "1")]
    pub display_name: std::string::String,
    /// The last time this skill was used.
    #[prost(message, optional, tag = "2")]
    pub last_used_date: ::std::option::Option<super::super::super::r#type::Date>,
    /// Skill proficiency level which indicates how proficient the candidate is at
    /// this skill.
    #[prost(enumeration = "SkillProficiencyLevel", tag = "3")]
    pub level: i32,
    /// A paragraph describes context of this skill.
    ///
    /// Number of characters allowed is 100,000.
    #[prost(string, tag = "4")]
    pub context: std::string::String,
    /// Output only. Skill name snippet shows how the [display_name][google.cloud.talent.v4beta1.Skill.display_name] is related to a search
    /// query. It's empty if the [display_name][google.cloud.talent.v4beta1.Skill.display_name] isn't related to the search
    /// query.
    #[prost(string, tag = "5")]
    pub skill_name_snippet: std::string::String,
}
/// Details of an interview.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Interview {
    /// The rating on this interview.
    #[prost(message, optional, tag = "6")]
    pub rating: ::std::option::Option<Rating>,
    /// Required. The overall decision resulting from this interview (positive, negative,
    /// nuetral).
    #[prost(enumeration = "Outcome", tag = "7")]
    pub outcome: i32,
}
/// The details of the score received for an assessment or interview.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Rating {
    /// Overall score.
    #[prost(double, tag = "1")]
    pub overall: f64,
    /// The minimum value for the score.
    #[prost(double, tag = "2")]
    pub min: f64,
    /// The maximum value for the score.
    #[prost(double, tag = "3")]
    pub max: f64,
    /// The steps within the score (for example, interval = 1 max = 5
    /// min = 1 indicates that the score can be 1, 2, 3, 4, or 5)
    #[prost(double, tag = "4")]
    pub interval: f64,
}
/// Metadata used for long running operations returned by CTS batch APIs.
/// It's used to replace [google.longrunning.Operation.metadata][google.longrunning.Operation.metadata].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchOperationMetadata {
    /// The state of a long running operation.
    #[prost(enumeration = "batch_operation_metadata::State", tag = "1")]
    pub state: i32,
    /// More detailed information about operation state.
    #[prost(string, tag = "2")]
    pub state_description: std::string::String,
    /// Count of successful item(s) inside an operation.
    #[prost(int32, tag = "3")]
    pub success_count: i32,
    /// Count of failed item(s) inside an operation.
    #[prost(int32, tag = "4")]
    pub failure_count: i32,
    /// Count of total item(s) inside an operation.
    #[prost(int32, tag = "5")]
    pub total_count: i32,
    /// The time when the batch operation is created.
    #[prost(message, optional, tag = "6")]
    pub create_time: ::std::option::Option<::prost_types::Timestamp>,
    /// The time when the batch operation status is updated. The metadata and the
    /// [update_time][google.cloud.talent.v4beta1.BatchOperationMetadata.update_time] is refreshed every minute otherwise cached data is
    /// returned.
    #[prost(message, optional, tag = "7")]
    pub update_time: ::std::option::Option<::prost_types::Timestamp>,
    /// The time when the batch operation is finished and
    /// [google.longrunning.Operation.done][google.longrunning.Operation.done] is set to `true`.
    #[prost(message, optional, tag = "8")]
    pub end_time: ::std::option::Option<::prost_types::Timestamp>,
}
pub mod batch_operation_metadata {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// Default value.
        Unspecified = 0,
        /// The batch operation is being prepared for processing.
        Initializing = 1,
        /// The batch operation is actively being processed.
        Processing = 2,
        /// The batch operation is processed, and at least one item has been
        /// successfully processed.
        Succeeded = 3,
        /// The batch operation is done and no item has been successfully processed.
        Failed = 4,
        /// The batch operation is in the process of cancelling after
        /// [google.longrunning.Operations.CancelOperation][google.longrunning.Operations.CancelOperation] is called.
        Cancelling = 5,
        /// The batch operation is done after
        /// [google.longrunning.Operations.CancelOperation][google.longrunning.Operations.CancelOperation] is called. Any items
        /// processed before cancelling are returned in the response.
        Cancelled = 6,
    }
}
/// An enum that represents the size of the company.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CompanySize {
    /// Default value if the size isn't specified.
    Unspecified = 0,
    /// The company has less than 50 employees.
    Mini = 1,
    /// The company has between 50 and 99 employees.
    Small = 2,
    /// The company has between 100 and 499 employees.
    Smedium = 3,
    /// The company has between 500 and 999 employees.
    Medium = 4,
    /// The company has between 1,000 and 4,999 employees.
    Big = 5,
    /// The company has between 5,000 and 9,999 employees.
    Bigger = 6,
    /// The company has 10,000 or more employees.
    Giant = 7,
}
/// An enum that represents employee benefits included with the job.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum JobBenefit {
    /// Default value if the type isn't specified.
    Unspecified = 0,
    /// The job includes access to programs that support child care, such
    /// as daycare.
    ChildCare = 1,
    /// The job includes dental services covered by a dental
    /// insurance plan.
    Dental = 2,
    /// The job offers specific benefits to domestic partners.
    DomesticPartner = 3,
    /// The job allows for a flexible work schedule.
    FlexibleHours = 4,
    /// The job includes health services covered by a medical insurance plan.
    Medical = 5,
    /// The job includes a life insurance plan provided by the employer or
    /// available for purchase by the employee.
    LifeInsurance = 6,
    /// The job allows for a leave of absence to a parent to care for a newborn
    /// child.
    ParentalLeave = 7,
    /// The job includes a workplace retirement plan provided by the
    /// employer or available for purchase by the employee.
    RetirementPlan = 8,
    /// The job allows for paid time off due to illness.
    SickDays = 9,
    /// The job includes paid time off for vacation.
    Vacation = 10,
    /// The job includes vision services covered by a vision
    /// insurance plan.
    Vision = 11,
}
/// Educational degree level defined in International Standard Classification
/// of Education (ISCED).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DegreeType {
    /// Default value. Represents no degree, or early childhood education.
    /// Maps to ISCED code 0.
    /// Ex) Kindergarten
    Unspecified = 0,
    /// Primary education which is typically the first stage of compulsory
    /// education. ISCED code 1.
    /// Ex) Elementary school
    PrimaryEducation = 1,
    /// Lower secondary education; First stage of secondary education building on
    /// primary education, typically with a more subject-oriented curriculum.
    /// ISCED code 2.
    /// Ex) Middle school
    LowerSecondaryEducation = 2,
    /// Middle education; Second/final stage of secondary education preparing for
    /// tertiary education and/or providing skills relevant to employment.
    /// Usually with an increased range of subject options and streams. ISCED
    /// code 3.
    /// Ex) High school
    UpperSecondaryEducation = 3,
    /// Adult Remedial Education; Programmes providing learning experiences that
    /// build on secondary education and prepare for labour market entry and/or
    /// tertiary education. The content is broader than secondary but not as
    /// complex as tertiary education. ISCED code 4.
    AdultRemedialEducation = 4,
    /// Associate's or equivalent; Short first tertiary programmes that are
    /// typically practically-based, occupationally-specific and prepare for
    /// labour market entry. These programmes may also provide a pathway to other
    /// tertiary programmes. ISCED code 5.
    AssociatesOrEquivalent = 5,
    /// Bachelor's or equivalent; Programmes designed to provide intermediate
    /// academic and/or professional knowledge, skills and competencies leading
    /// to a first tertiary degree or equivalent qualification. ISCED code 6.
    BachelorsOrEquivalent = 6,
    /// Master's or equivalent; Programmes designed to provide advanced academic
    /// and/or professional knowledge, skills and competencies leading to a
    /// second tertiary degree or equivalent qualification. ISCED code 7.
    MastersOrEquivalent = 7,
    /// Doctoral or equivalent; Programmes designed primarily to lead to an
    /// advanced research qualification, usually concluding with the submission
    /// and defense of a substantive dissertation of publishable quality based on
    /// original research. ISCED code 8.
    DoctoralOrEquivalent = 8,
}
/// An enum that represents the employment type of a job.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum EmploymentType {
    /// The default value if the employment type isn't specified.
    Unspecified = 0,
    /// The job requires working a number of hours that constitute full
    /// time employment, typically 40 or more hours per week.
    FullTime = 1,
    /// The job entails working fewer hours than a full time job,
    /// typically less than 40 hours a week.
    PartTime = 2,
    /// The job is offered as a contracted, as opposed to a salaried employee,
    /// position.
    Contractor = 3,
    /// The job is offered as a contracted position with the understanding
    /// that it's converted into a full-time position at the end of the
    /// contract. Jobs of this type are also returned by a search for
    /// [EmploymentType.CONTRACTOR][google.cloud.talent.v4beta1.EmploymentType.CONTRACTOR] jobs.
    ContractToHire = 4,
    /// The job is offered as a temporary employment opportunity, usually
    /// a short-term engagement.
    Temporary = 5,
    /// The job is a fixed-term opportunity for students or entry-level job
    /// seekers to obtain on-the-job training, typically offered as a summer
    /// position.
    Intern = 6,
    /// The is an opportunity for an individual to volunteer, where there's no
    /// expectation of compensation for the provided services.
    Volunteer = 7,
    /// The job requires an employee to work on an as-needed basis with a
    /// flexible schedule.
    PerDiem = 8,
    /// The job involves employing people in remote areas and flying them
    /// temporarily to the work site instead of relocating employees and their
    /// families permanently.
    FlyInFlyOut = 9,
    /// The job does not fit any of the other listed types.
    OtherEmploymentType = 10,
}
/// An enum that represents the required experience level required for the job.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum JobLevel {
    /// The default value if the level isn't specified.
    Unspecified = 0,
    /// Entry-level individual contributors, typically with less than 2 years of
    /// experience in a similar role. Includes interns.
    EntryLevel = 1,
    /// Experienced individual contributors, typically with 2+ years of
    /// experience in a similar role.
    Experienced = 2,
    /// Entry- to mid-level managers responsible for managing a team of people.
    Manager = 3,
    /// Senior-level managers responsible for managing teams of managers.
    Director = 4,
    /// Executive-level managers and above, including C-level positions.
    Executive = 5,
}
/// An enum that represents the categorization or primary focus of specific
/// role. This value is different than the "industry" associated with a role,
/// which is related to the categorization of the company listing the job.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum JobCategory {
    /// The default value if the category isn't specified.
    Unspecified = 0,
    /// An accounting and finance job, such as an Accountant.
    AccountingAndFinance = 1,
    /// An administrative and office job, such as an Administrative Assistant.
    AdministrativeAndOffice = 2,
    /// An advertising and marketing job, such as Marketing Manager.
    AdvertisingAndMarketing = 3,
    /// An animal care job, such as Veterinarian.
    AnimalCare = 4,
    /// An art, fashion, or design job, such as Designer.
    ArtFashionAndDesign = 5,
    /// A business operations job, such as Business Operations Manager.
    BusinessOperations = 6,
    /// A cleaning and facilities job, such as Custodial Staff.
    CleaningAndFacilities = 7,
    /// A computer and IT job, such as Systems Administrator.
    ComputerAndIt = 8,
    /// A construction job, such as General Laborer.
    Construction = 9,
    /// A customer service job, such s Cashier.
    CustomerService = 10,
    /// An education job, such as School Teacher.
    Education = 11,
    /// An entertainment and travel job, such as Flight Attendant.
    EntertainmentAndTravel = 12,
    /// A farming or outdoor job, such as Park Ranger.
    FarmingAndOutdoors = 13,
    /// A healthcare job, such as Registered Nurse.
    Healthcare = 14,
    /// A human resources job, such as Human Resources Director.
    HumanResources = 15,
    /// An installation, maintenance, or repair job, such as Electrician.
    InstallationMaintenanceAndRepair = 16,
    /// A legal job, such as Law Clerk.
    Legal = 17,
    /// A management job, often used in conjunction with another category,
    /// such as Store Manager.
    Management = 18,
    /// A manufacturing or warehouse job, such as Assembly Technician.
    ManufacturingAndWarehouse = 19,
    /// A media, communications, or writing job, such as Media Relations.
    MediaCommunicationsAndWriting = 20,
    /// An oil, gas or mining job, such as Offshore Driller.
    OilGasAndMining = 21,
    /// A personal care and services job, such as Hair Stylist.
    PersonalCareAndServices = 22,
    /// A protective services job, such as Security Guard.
    ProtectiveServices = 23,
    /// A real estate job, such as Buyer's Agent.
    RealEstate = 24,
    /// A restaurant and hospitality job, such as Restaurant Server.
    RestaurantAndHospitality = 25,
    /// A sales and/or retail job, such Sales Associate.
    SalesAndRetail = 26,
    /// A science and engineering job, such as Lab Technician.
    ScienceAndEngineering = 27,
    /// A social services or non-profit job, such as Case Worker.
    SocialServicesAndNonProfit = 28,
    /// A sports, fitness, or recreation job, such as Personal Trainer.
    SportsFitnessAndRecreation = 29,
    /// A transportation or logistics job, such as Truck Driver.
    TransportationAndLogistics = 30,
}
/// An enum that represents the job posting region. In most cases, job postings
/// don't need to specify a region. If a region is given, jobs are
/// eligible for searches in the specified region.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PostingRegion {
    /// If the region is unspecified, the job is only returned if it
    /// matches the [LocationFilter][google.cloud.talent.v4beta1.LocationFilter].
    Unspecified = 0,
    /// In addition to exact location matching, job posting is returned when the
    /// [LocationFilter][google.cloud.talent.v4beta1.LocationFilter] in the search query is in the same administrative area
    /// as the returned job posting. For example, if a `ADMINISTRATIVE_AREA` job
    /// is posted in "CA, USA", it's returned if [LocationFilter][google.cloud.talent.v4beta1.LocationFilter] has
    /// "Mountain View".
    ///
    /// Administrative area refers to top-level administrative subdivision of this
    /// country. For example, US state, IT region, UK constituent nation and
    /// JP prefecture.
    AdministrativeArea = 1,
    /// In addition to exact location matching, job is returned when
    /// [LocationFilter][google.cloud.talent.v4beta1.LocationFilter] in search query is in the same country as this job.
    /// For example, if a `NATION_WIDE` job is posted in "USA", it's
    /// returned if [LocationFilter][google.cloud.talent.v4beta1.LocationFilter] has 'Mountain View'.
    Nation = 2,
    /// Job allows employees to work remotely (telecommute).
    /// If locations are provided with this value, the job is
    /// considered as having a location, but telecommuting is allowed.
    Telecommute = 3,
}
/// Deprecated. All resources are only visible to the owner.
///
/// An enum that represents who has view access to the resource.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Visibility {
    /// Default value.
    Unspecified = 0,
    /// The resource is only visible to the GCP account who owns it.
    AccountOnly = 1,
    /// The resource is visible to the owner and may be visible to other
    /// applications and processes at Google.
    SharedWithGoogle = 2,
    /// The resource is visible to the owner and may be visible to all other API
    /// clients.
    SharedWithPublic = 3,
}
/// Enum that represents the usage of the contact information.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ContactInfoUsage {
    /// Default value.
    Unspecified = 0,
    /// Personal use.
    Personal = 1,
    /// Work use.
    Work = 2,
    /// School use.
    School = 3,
}
/// Option for HTML content sanitization on user input fields, for example, job
/// description. By setting this option, user can determine whether and how
/// sanitization is performed on these fields.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum HtmlSanitization {
    /// Default value.
    Unspecified = 0,
    /// Disables sanitization on HTML input.
    Disabled = 1,
    /// Sanitizes HTML input, only accepts bold, italic, ordered list, and
    /// unordered list markup tags.
    SimpleFormattingOnly = 2,
}
/// Method for commute.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CommuteMethod {
    /// Commute method isn't specified.
    Unspecified = 0,
    /// Commute time is calculated based on driving time.
    Driving = 1,
    /// Commute time is calculated based on public transit including bus, metro,
    /// subway, and so on.
    Transit = 2,
    /// Commute time is calculated based on walking time.
    Walking = 3,
    /// Commute time is calculated based on biking time.
    Cycling = 4,
}
/// Enum that represents the skill proficiency level.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SkillProficiencyLevel {
    /// Default value.
    Unspecified = 0,
    /// Lacks any proficiency in this skill.
    Unskilled = 6,
    /// Have a common knowledge or an understanding of basic techniques and
    /// concepts.
    FundamentalAwareness = 1,
    /// Have the level of experience gained in a classroom and/or experimental
    /// scenarios or as a trainee on-the-job.
    Novice = 2,
    /// Be able to successfully complete tasks in this skill as requested. Help
    /// from an expert may be required from time to time, but can usually perform
    /// skill independently.
    Intermediate = 3,
    /// Can perform the actions associated with this skill without assistance.
    Advanced = 4,
    /// Known as an expert in this area.
    Expert = 5,
}
/// The overall outcome /decision / result indicator.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Outcome {
    /// Default value.
    Unspecified = 0,
    /// A positive outcome / passing indicator (for example, candidate was
    /// recommended for hiring or to be moved forward in the hiring process,
    /// candidate passed a test).
    Positive = 1,
    /// A neutral outcome / no clear indicator (for example, no strong
    /// reccommendation either to move forward / not move forward, neutral score).
    Neutral = 2,
    /// A negative outcome / failing indicator (for example, candidate was
    /// recommended to NOT move forward in the hiring process, failed a test).
    Negative = 3,
    /// The assessment outcome is not available or otherwise unknown (for example,
    /// candidate did not complete assessment).
    NotAvailable = 4,
}
/// The type of candidate availability signal.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AvailabilitySignalType {
    /// Default value.
    Unspecified = 0,
    /// Job application signal.
    ///
    /// In the context of [Profile.availability_signals][google.cloud.talent.v4beta1.Profile.availability_signals], this signal is related
    /// to the candidate's most recent application.
    /// [last_update_time][google.cloud.talent.v4beta1.AvailabilitySignal.last_update_time] is
    /// calculated from max([Application.create_time][google.cloud.talent.v4beta1.Application.create_time]) from all [Application][google.cloud.talent.v4beta1.Application]
    /// records where [Application.source][google.cloud.talent.v4beta1.Application.source] is any of the following:
    ///  [APPLY_DIRECT_WEB][google.cloud.talent.v4beta1.Application.ApplicationSource.APPLY_DIRECT_WEB]
    ///  [APPLY_DIRECT_MOBILE_WEB][google.cloud.talent.v4beta1.Application.ApplicationSource.APPLY_DIRECT_MOBILE_WEB]
    ///  [APPLY_DIRECT_MOBILE_APP][google.cloud.talent.v4beta1.Application.ApplicationSource.APPLY_DIRECT_MOBILE_APP]
    ///  [APPLY_DIRECT_IN_PERSON][google.cloud.talent.v4beta1.Application.ApplicationSource.APPLY_DIRECT_IN_PERSON]
    ///  [APPLY_INDIRECT][google.cloud.talent.v4beta1.Application.ApplicationSource.APPLY_INDIRECT]
    ///
    /// In the context of [AvailabilityFilter][google.cloud.talent.v4beta1.AvailabilityFilter], the filter is applied on
    /// [Profile.availability_signals][google.cloud.talent.v4beta1.Profile.availability_signals] where
    /// [type][google.cloud.talent.v4beta1.AvailabilitySignal.type] is JOB_APPLICATION.
    JobApplication = 1,
    /// Resume update signal.
    ///
    /// In the context of [Profile.availability_signals][google.cloud.talent.v4beta1.Profile.availability_signals], this signal is related
    /// to the candidate's most recent update to their resume.
    /// For a [SummarizedProfile.summary][google.cloud.talent.v4beta1.SummarizedProfile.summary],
    /// [last_update_time][google.cloud.talent.v4beta1.AvailabilitySignal.last_update_time] is
    /// calculated from max([Profile.resume_update_time][google.cloud.talent.v4beta1.Profile.resume_update_time]) from all
    /// [SummarizedProfile.profiles][google.cloud.talent.v4beta1.SummarizedProfile.profiles].
    ///
    /// In the context of [AvailabilityFilter][google.cloud.talent.v4beta1.AvailabilityFilter], the filter is applied on
    /// [Profile.availability_signals][google.cloud.talent.v4beta1.Profile.availability_signals] where
    /// [type][google.cloud.talent.v4beta1.AvailabilitySignal.type] is RESUME_UPDATE.
    ResumeUpdate = 2,
    /// Candidate update signal.
    ///
    /// In the context of [Profile.availability_signals][google.cloud.talent.v4beta1.Profile.availability_signals], this signal is related
    /// to the candidate's most recent update to their profile.
    /// For a [SummarizedProfile.summary][google.cloud.talent.v4beta1.SummarizedProfile.summary],
    /// [last_update_time][google.cloud.talent.v4beta1.AvailabilitySignal.last_update_time] is
    /// calculated from max([Profile.candidate_update_time][google.cloud.talent.v4beta1.Profile.candidate_update_time]) from all
    /// [SummarizedProfile.profiles][google.cloud.talent.v4beta1.SummarizedProfile.profiles].
    ///
    /// In the context of [AvailabilityFilter][google.cloud.talent.v4beta1.AvailabilityFilter], the filter is applied on
    /// [Profile.availability_signals][google.cloud.talent.v4beta1.Profile.availability_signals] where
    /// [type][google.cloud.talent.v4beta1.AvailabilitySignal.type] is CANDIDATE_UPDATE.
    CandidateUpdate = 3,
    /// Client submission signal.
    ///
    /// In the context of [Profile.availability_signals][google.cloud.talent.v4beta1.Profile.availability_signals], this signal is related
    /// to the candidate's most recent submission.
    /// [last_update_time][google.cloud.talent.v4beta1.AvailabilitySignal.last_update_time] is
    /// calculated from max([Application.create_time][google.cloud.talent.v4beta1.Application.create_time]) from all [Application][google.cloud.talent.v4beta1.Application]
    /// records where [Application.stage][google.cloud.talent.v4beta1.Application.stage] is any of the following:
    ///  [HIRING_MANAGER_REVIEW][google.cloud.talent.v4beta1.Application.ApplicationStage.HIRING_MANAGER_REVIEW]
    ///  [INTERVIEW][google.cloud.talent.v4beta1.Application.ApplicationStage.INTERVIEW]
    ///  [OFFER_EXTENDED][google.cloud.talent.v4beta1.Application.ApplicationStage.OFFER_EXTENDED]
    ///  [OFFER_ACCEPTED][google.cloud.talent.v4beta1.Application.ApplicationStage.OFFER_ACCEPTED]
    ///  [STARTED][google.cloud.talent.v4beta1.Application.ApplicationStage.STARTED]
    ///
    /// In the context of [AvailabilityFilter][google.cloud.talent.v4beta1.AvailabilityFilter], the filter is applied on
    /// [Profile.availability_signals][google.cloud.talent.v4beta1.Profile.availability_signals] where
    /// [type][google.cloud.talent.v4beta1.AvailabilitySignal.type] is CLIENT_SUBMISSION.
    ClientSubmission = 4,
}
/// Resource that represents a job application record of a candidate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Application {
    /// Required during application update.
    ///
    /// Resource name assigned to an application by the API.
    ///
    /// The format is
    /// "projects/{project_id}/tenants/{tenant_id}/profiles/{profile_id}/applications/{application_id}".
    /// For example, "projects/foo/tenants/bar/profiles/baz/applications/qux".
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Required. Client side application identifier, used to uniquely identify the
    /// application.
    ///
    /// The maximum number of allowed characters is 255.
    #[prost(string, tag = "31")]
    pub external_id: std::string::String,
    /// Output only. Resource name of the candidate of this application.
    ///
    /// The format is
    /// "projects/{project_id}/tenants/{tenant_id}/profiles/{profile_id}".
    /// For example, "projects/foo/tenants/bar/profiles/baz".
    #[prost(string, tag = "2")]
    pub profile: std::string::String,
    /// Required. Resource name of the job which the candidate applied for.
    ///
    /// The format is
    /// "projects/{project_id}/tenants/{tenant_id}/jobs/{job_id}". For example,
    /// "projects/foo/tenants/bar/jobs/baz".
    #[prost(string, tag = "4")]
    pub job: std::string::String,
    /// Resource name of the company which the candidate applied for.
    ///
    /// The format is
    /// "projects/{project_id}/tenants/{tenant_id}/companies/{company_id}".
    /// For example, "projects/foo/tenants/bar/companies/baz".
    #[prost(string, tag = "5")]
    pub company: std::string::String,
    /// The application date.
    #[prost(message, optional, tag = "7")]
    pub application_date: ::std::option::Option<super::super::super::r#type::Date>,
    /// Required. What is the most recent stage of the application (that is, new,
    /// screen, send cv, hired, finished work)?  This field is intentionally not
    /// comprehensive of every possible status, but instead, represents statuses
    /// that would be used to indicate to the ML models good / bad matches.
    #[prost(enumeration = "application::ApplicationStage", tag = "11")]
    pub stage: i32,
    /// The application state.
    #[prost(enumeration = "application::ApplicationState", tag = "13")]
    pub state: i32,
    /// All interviews (screen, onsite, and so on) conducted as part of this
    /// application (includes details such as user conducting the interview,
    /// timestamp, feedback, and so on).
    #[prost(message, repeated, tag = "16")]
    pub interviews: ::std::vec::Vec<Interview>,
    /// If the candidate is referred by a employee.
    #[prost(message, optional, tag = "18")]
    pub referral: ::std::option::Option<bool>,
    /// Required. Reflects the time that the application was created.
    #[prost(message, optional, tag = "19")]
    pub create_time: ::std::option::Option<::prost_types::Timestamp>,
    /// The last update timestamp.
    #[prost(message, optional, tag = "20")]
    pub update_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Free text reason behind the recruitement outcome (for example, reason for
    /// withdraw / reject, reason for an unsuccessful finish, and so on).
    ///
    /// Number of characters allowed is 100.
    #[prost(string, tag = "21")]
    pub outcome_notes: std::string::String,
    /// Outcome positiveness shows how positive the outcome is.
    #[prost(enumeration = "Outcome", tag = "22")]
    pub outcome: i32,
    /// Output only. Indicates whether this job application is a match to
    /// application related filters. This value is only applicable in profile
    /// search response.
    #[prost(message, optional, tag = "28")]
    pub is_match: ::std::option::Option<bool>,
    /// Output only. Job title snippet shows how the job title is related to a
    /// search query. It's empty if the job title isn't related to the search
    /// query.
    #[prost(string, tag = "29")]
    pub job_title_snippet: std::string::String,
}
pub mod application {
    /// Enum that represents the application status.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ApplicationState {
        /// Default value.
        Unspecified = 0,
        /// The current stage is in progress or pending, for example, interviews in
        /// progress.
        InProgress = 1,
        /// The current stage was terminated by a candidate decision.
        CandidateWithdrew = 2,
        /// The current stage was terminated by an employer or agency decision.
        EmployerWithdrew = 3,
        /// The current stage is successfully completed, but the next stage (if
        /// applicable) has not begun.
        Completed = 4,
        /// The current stage was closed without an exception, or terminated for
        /// reasons unrealated to the candidate.
        Closed = 5,
    }
    /// The stage of the application.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ApplicationStage {
        /// Default value.
        Unspecified = 0,
        /// Candidate has applied or a recruiter put candidate into consideration but
        /// candidate is not yet screened / no decision has been made to move or not
        /// move the candidate to the next stage.
        New = 1,
        /// A recruiter decided to screen the candidate for this role.
        Screen = 2,
        /// Candidate is being / was sent to the customer / hiring manager for
        /// detailed review.
        HiringManagerReview = 3,
        /// Candidate was approved by the client / hiring manager and is being / was
        /// interviewed for the role.
        Interview = 4,
        /// Candidate will be / has been given an offer of employment.
        OfferExtended = 5,
        /// Candidate has accepted their offer of employment.
        OfferAccepted = 6,
        /// Candidate has begun (or completed) their employment or assignment with
        /// the employer.
        Started = 7,
    }
}
/// The Request of the CreateApplication method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateApplicationRequest {
    /// Required. Resource name of the profile under which the application is created.
    ///
    /// The format is
    /// "projects/{project_id}/tenants/{tenant_id}/profiles/{profile_id}".
    /// For example, "projects/foo/tenants/bar/profiles/baz".
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Required. The application to be created.
    #[prost(message, optional, tag = "2")]
    pub application: ::std::option::Option<Application>,
}
/// Request for getting a application by name.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetApplicationRequest {
    /// Required. The resource name of the application to be retrieved.
    ///
    /// The format is
    /// "projects/{project_id}/tenants/{tenant_id}/profiles/{profile_id}/applications/{application_id}".
    /// For example, "projects/foo/tenants/bar/profiles/baz/applications/qux".
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Request for updating a specified application.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateApplicationRequest {
    /// Required. The application resource to replace the current resource in the system.
    #[prost(message, optional, tag = "1")]
    pub application: ::std::option::Option<Application>,
    /// Strongly recommended for the best service experience.
    ///
    /// If [update_mask][google.cloud.talent.v4beta1.UpdateApplicationRequest.update_mask] is provided, only the specified fields in
    /// [application][google.cloud.talent.v4beta1.UpdateApplicationRequest.application] are updated. Otherwise all the fields are updated.
    ///
    /// A field mask to specify the application fields to be updated. Only
    /// top level fields of [Application][google.cloud.talent.v4beta1.Application] are supported.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
}
/// Request to delete a application.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteApplicationRequest {
    /// Required. The resource name of the application to be deleted.
    ///
    /// The format is
    /// "projects/{project_id}/tenants/{tenant_id}/profiles/{profile_id}/applications/{application_id}".
    /// For example, "projects/foo/tenants/bar/profiles/baz/applications/qux".
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// List applications for which the client has ACL visibility.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListApplicationsRequest {
    /// Required. Resource name of the profile under which the application is created.
    ///
    /// The format is
    /// "projects/{project_id}/tenants/{tenant_id}/profiles/{profile_id}", for
    /// example, "projects/foo/tenants/bar/profiles/baz".
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// The starting indicator from which to return results.
    #[prost(string, tag = "2")]
    pub page_token: std::string::String,
    /// The maximum number of applications to be returned, at most 100.
    /// Default is 100 if a non-positive number is provided.
    #[prost(int32, tag = "3")]
    pub page_size: i32,
}
/// The List applications response object.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListApplicationsResponse {
    /// Applications for the current client.
    #[prost(message, repeated, tag = "1")]
    pub applications: ::std::vec::Vec<Application>,
    /// A token to retrieve the next page of results.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
    /// Additional information for the API invocation, such as the request
    /// tracking id.
    #[prost(message, optional, tag = "3")]
    pub metadata: ::std::option::Option<ResponseMetadata>,
}
#[doc = r" Generated client implementations."]
pub mod application_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " A service that handles application management, including CRUD and"]
    #[doc = " enumeration."]
    pub struct ApplicationServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ApplicationServiceClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> ApplicationServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + HttpBody + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as HttpBody>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = tonic::client::Grpc::with_interceptor(inner, interceptor);
            Self { inner }
        }
        #[doc = " Creates a new application entity."]
        pub async fn create_application(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateApplicationRequest>,
        ) -> Result<tonic::Response<super::Application>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.talent.v4beta1.ApplicationService/CreateApplication",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Retrieves specified application."]
        pub async fn get_application(
            &mut self,
            request: impl tonic::IntoRequest<super::GetApplicationRequest>,
        ) -> Result<tonic::Response<super::Application>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.talent.v4beta1.ApplicationService/GetApplication",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates specified application."]
        pub async fn update_application(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateApplicationRequest>,
        ) -> Result<tonic::Response<super::Application>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.talent.v4beta1.ApplicationService/UpdateApplication",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes specified application."]
        pub async fn delete_application(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteApplicationRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.talent.v4beta1.ApplicationService/DeleteApplication",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists all applications associated with the profile."]
        pub async fn list_applications(
            &mut self,
            request: impl tonic::IntoRequest<super::ListApplicationsRequest>,
        ) -> Result<tonic::Response<super::ListApplicationsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.talent.v4beta1.ApplicationService/ListApplications",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for ApplicationServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for ApplicationServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "ApplicationServiceClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod application_service_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with ApplicationServiceServer."]
    #[async_trait]
    pub trait ApplicationService: Send + Sync + 'static {
        #[doc = " Creates a new application entity."]
        async fn create_application(
            &self,
            request: tonic::Request<super::CreateApplicationRequest>,
        ) -> Result<tonic::Response<super::Application>, tonic::Status>;
        #[doc = " Retrieves specified application."]
        async fn get_application(
            &self,
            request: tonic::Request<super::GetApplicationRequest>,
        ) -> Result<tonic::Response<super::Application>, tonic::Status>;
        #[doc = " Updates specified application."]
        async fn update_application(
            &self,
            request: tonic::Request<super::UpdateApplicationRequest>,
        ) -> Result<tonic::Response<super::Application>, tonic::Status>;
        #[doc = " Deletes specified application."]
        async fn delete_application(
            &self,
            request: tonic::Request<super::DeleteApplicationRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status>;
        #[doc = " Lists all applications associated with the profile."]
        async fn list_applications(
            &self,
            request: tonic::Request<super::ListApplicationsRequest>,
        ) -> Result<tonic::Response<super::ListApplicationsResponse>, tonic::Status>;
    }
    #[doc = " A service that handles application management, including CRUD and"]
    #[doc = " enumeration."]
    #[derive(Debug)]
    #[doc(hidden)]
    pub struct ApplicationServiceServer<T: ApplicationService> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: ApplicationService> ApplicationServiceServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, None);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, Some(interceptor.into()));
            Self { inner }
        }
    }
    impl<T, B> Service<http::Request<B>> for ApplicationServiceServer<T>
    where
        T: ApplicationService,
        B: HttpBody + Send + Sync + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/google.cloud.talent.v4beta1.ApplicationService/CreateApplication" => {
                    #[allow(non_camel_case_types)]
                    struct CreateApplicationSvc<T: ApplicationService>(pub Arc<T>);
                    impl<T: ApplicationService>
                        tonic::server::UnaryService<super::CreateApplicationRequest>
                        for CreateApplicationSvc<T>
                    {
                        type Response = super::Application;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateApplicationRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.create_application(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = CreateApplicationSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/google.cloud.talent.v4beta1.ApplicationService/GetApplication" => {
                    #[allow(non_camel_case_types)]
                    struct GetApplicationSvc<T: ApplicationService>(pub Arc<T>);
                    impl<T: ApplicationService>
                        tonic::server::UnaryService<super::GetApplicationRequest>
                        for GetApplicationSvc<T>
                    {
                        type Response = super::Application;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetApplicationRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.get_application(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetApplicationSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/google.cloud.talent.v4beta1.ApplicationService/UpdateApplication" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateApplicationSvc<T: ApplicationService>(pub Arc<T>);
                    impl<T: ApplicationService>
                        tonic::server::UnaryService<super::UpdateApplicationRequest>
                        for UpdateApplicationSvc<T>
                    {
                        type Response = super::Application;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateApplicationRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.update_application(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = UpdateApplicationSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/google.cloud.talent.v4beta1.ApplicationService/DeleteApplication" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteApplicationSvc<T: ApplicationService>(pub Arc<T>);
                    impl<T: ApplicationService>
                        tonic::server::UnaryService<super::DeleteApplicationRequest>
                        for DeleteApplicationSvc<T>
                    {
                        type Response = ();
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteApplicationRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.delete_application(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = DeleteApplicationSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/google.cloud.talent.v4beta1.ApplicationService/ListApplications" => {
                    #[allow(non_camel_case_types)]
                    struct ListApplicationsSvc<T: ApplicationService>(pub Arc<T>);
                    impl<T: ApplicationService>
                        tonic::server::UnaryService<super::ListApplicationsRequest>
                        for ListApplicationsSvc<T>
                    {
                        type Response = super::ListApplicationsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListApplicationsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.list_applications(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ListApplicationsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .body(tonic::body::BoxBody::empty())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: ApplicationService> Clone for ApplicationServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: ApplicationService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: ApplicationService> tonic::transport::NamedService for ApplicationServiceServer<T> {
        const NAME: &'static str = "google.cloud.talent.v4beta1.ApplicationService";
    }
}
/// A Company resource represents a company in the service. A company is the
/// entity that owns job postings, that is, the hiring entity responsible for
/// employing applicants for the job position.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Company {
    /// Required during company update.
    ///
    /// The resource name for a company. This is generated by the service when a
    /// company is created.
    ///
    /// The format is
    /// "projects/{project_id}/tenants/{tenant_id}/companies/{company_id}", for
    /// example, "projects/foo/tenants/bar/companies/baz".
    ///
    /// If tenant id is unspecified, the default tenant is used. For
    /// example, "projects/foo/companies/bar".
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Required. The display name of the company, for example, "Google LLC".
    #[prost(string, tag = "2")]
    pub display_name: std::string::String,
    /// Required. Client side company identifier, used to uniquely identify the
    /// company.
    ///
    /// The maximum number of allowed characters is 255.
    #[prost(string, tag = "3")]
    pub external_id: std::string::String,
    /// The employer's company size.
    #[prost(enumeration = "CompanySize", tag = "4")]
    pub size: i32,
    /// The street address of the company's main headquarters, which may be
    /// different from the job location. The service attempts
    /// to geolocate the provided address, and populates a more specific
    /// location wherever possible in [DerivedInfo.headquarters_location][google.cloud.talent.v4beta1.Company.DerivedInfo.headquarters_location].
    #[prost(string, tag = "5")]
    pub headquarters_address: std::string::String,
    /// Set to true if it is the hiring agency that post jobs for other
    /// employers.
    ///
    /// Defaults to false if not provided.
    #[prost(bool, tag = "6")]
    pub hiring_agency: bool,
    /// Equal Employment Opportunity legal disclaimer text to be
    /// associated with all jobs, and typically to be displayed in all
    /// roles.
    ///
    /// The maximum number of allowed characters is 500.
    #[prost(string, tag = "7")]
    pub eeo_text: std::string::String,
    /// The URI representing the company's primary web site or home page,
    /// for example, "https://www.google.com".
    ///
    /// The maximum number of allowed characters is 255.
    #[prost(string, tag = "8")]
    pub website_uri: std::string::String,
    /// The URI to employer's career site or careers page on the employer's web
    /// site, for example, "https://careers.google.com".
    #[prost(string, tag = "9")]
    pub career_site_uri: std::string::String,
    /// A URI that hosts the employer's company logo.
    #[prost(string, tag = "10")]
    pub image_uri: std::string::String,
    /// A list of keys of filterable [Job.custom_attributes][google.cloud.talent.v4beta1.Job.custom_attributes], whose
    /// corresponding `string_values` are used in keyword searches. Jobs with
    /// `string_values` under these specified field keys are returned if any
    /// of the values match the search keyword. Custom field values with
    /// parenthesis, brackets and special symbols are not searchable as-is,
    /// and those keyword queries must be surrounded by quotes.
    #[prost(string, repeated, tag = "11")]
    pub keyword_searchable_job_custom_attributes: ::std::vec::Vec<std::string::String>,
    /// Output only. Derived details about the company.
    #[prost(message, optional, tag = "12")]
    pub derived_info: ::std::option::Option<company::DerivedInfo>,
    /// Output only. Indicates whether a company is flagged to be suspended from
    /// public availability by the service when job content appears suspicious,
    /// abusive, or spammy.
    #[prost(bool, tag = "13")]
    pub suspended: bool,
}
pub mod company {
    /// Derived details about the company.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DerivedInfo {
        /// A structured headquarters location of the company, resolved from
        /// [Company.headquarters_address][google.cloud.talent.v4beta1.Company.headquarters_address] if provided.
        #[prost(message, optional, tag = "1")]
        pub headquarters_location: ::std::option::Option<super::Location>,
    }
}
/// The Request of the CreateCompany method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateCompanyRequest {
    /// Required. Resource name of the tenant under which the company is created.
    ///
    /// The format is "projects/{project_id}/tenants/{tenant_id}", for example,
    /// "projects/foo/tenant/bar". If tenant id is unspecified, a default tenant
    /// is created, for example, "projects/foo".
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Required. The company to be created.
    #[prost(message, optional, tag = "2")]
    pub company: ::std::option::Option<Company>,
}
/// Request for getting a company by name.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCompanyRequest {
    /// Required. The resource name of the company to be retrieved.
    ///
    /// The format is
    /// "projects/{project_id}/tenants/{tenant_id}/companies/{company_id}", for
    /// example, "projects/api-test-project/tenants/foo/companies/bar".
    ///
    /// If tenant id is unspecified, the default tenant is used, for
    /// example, "projects/api-test-project/companies/bar".
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Request for updating a specified company.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateCompanyRequest {
    /// Required. The company resource to replace the current resource in the system.
    #[prost(message, optional, tag = "1")]
    pub company: ::std::option::Option<Company>,
    /// Strongly recommended for the best service experience.
    ///
    /// If [update_mask][google.cloud.talent.v4beta1.UpdateCompanyRequest.update_mask] is provided, only the specified fields in
    /// [company][google.cloud.talent.v4beta1.UpdateCompanyRequest.company] are updated. Otherwise all the fields are updated.
    ///
    /// A field mask to specify the company fields to be updated. Only
    /// top level fields of [Company][google.cloud.talent.v4beta1.Company] are supported.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
}
/// Request to delete a company.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteCompanyRequest {
    /// Required. The resource name of the company to be deleted.
    ///
    /// The format is
    /// "projects/{project_id}/tenants/{tenant_id}/companies/{company_id}", for
    /// example, "projects/foo/tenants/bar/companies/baz".
    ///
    /// If tenant id is unspecified, the default tenant is used, for
    /// example, "projects/foo/companies/bar".
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// List companies for which the client has ACL visibility.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCompaniesRequest {
    /// Required. Resource name of the tenant under which the company is created.
    ///
    /// The format is "projects/{project_id}/tenants/{tenant_id}", for example,
    /// "projects/foo/tenant/bar".
    ///
    /// If tenant id is unspecified, the default tenant will be used, for
    /// example, "projects/foo".
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// The starting indicator from which to return results.
    #[prost(string, tag = "2")]
    pub page_token: std::string::String,
    /// The maximum number of companies to be returned, at most 100.
    /// Default is 100 if a non-positive number is provided.
    #[prost(int32, tag = "3")]
    pub page_size: i32,
    /// Set to true if the companies requested must have open jobs.
    ///
    /// Defaults to false.
    ///
    /// If true, at most [page_size][google.cloud.talent.v4beta1.ListCompaniesRequest.page_size] of companies are fetched, among which
    /// only those with open jobs are returned.
    #[prost(bool, tag = "4")]
    pub require_open_jobs: bool,
}
/// The List companies response object.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCompaniesResponse {
    /// Companies for the current client.
    #[prost(message, repeated, tag = "1")]
    pub companies: ::std::vec::Vec<Company>,
    /// A token to retrieve the next page of results.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
    /// Additional information for the API invocation, such as the request
    /// tracking id.
    #[prost(message, optional, tag = "3")]
    pub metadata: ::std::option::Option<ResponseMetadata>,
}
#[doc = r" Generated client implementations."]
pub mod company_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " A service that handles company management, including CRUD and enumeration."]
    pub struct CompanyServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl CompanyServiceClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> CompanyServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + HttpBody + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as HttpBody>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = tonic::client::Grpc::with_interceptor(inner, interceptor);
            Self { inner }
        }
        #[doc = " Creates a new company entity."]
        pub async fn create_company(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateCompanyRequest>,
        ) -> Result<tonic::Response<super::Company>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.talent.v4beta1.CompanyService/CreateCompany",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Retrieves specified company."]
        pub async fn get_company(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCompanyRequest>,
        ) -> Result<tonic::Response<super::Company>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.talent.v4beta1.CompanyService/GetCompany",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates specified company."]
        pub async fn update_company(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateCompanyRequest>,
        ) -> Result<tonic::Response<super::Company>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.talent.v4beta1.CompanyService/UpdateCompany",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes specified company."]
        #[doc = " Prerequisite: The company has no jobs associated with it."]
        pub async fn delete_company(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteCompanyRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.talent.v4beta1.CompanyService/DeleteCompany",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists all companies associated with the project."]
        pub async fn list_companies(
            &mut self,
            request: impl tonic::IntoRequest<super::ListCompaniesRequest>,
        ) -> Result<tonic::Response<super::ListCompaniesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.talent.v4beta1.CompanyService/ListCompanies",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for CompanyServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for CompanyServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "CompanyServiceClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod company_service_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with CompanyServiceServer."]
    #[async_trait]
    pub trait CompanyService: Send + Sync + 'static {
        #[doc = " Creates a new company entity."]
        async fn create_company(
            &self,
            request: tonic::Request<super::CreateCompanyRequest>,
        ) -> Result<tonic::Response<super::Company>, tonic::Status>;
        #[doc = " Retrieves specified company."]
        async fn get_company(
            &self,
            request: tonic::Request<super::GetCompanyRequest>,
        ) -> Result<tonic::Response<super::Company>, tonic::Status>;
        #[doc = " Updates specified company."]
        async fn update_company(
            &self,
            request: tonic::Request<super::UpdateCompanyRequest>,
        ) -> Result<tonic::Response<super::Company>, tonic::Status>;
        #[doc = " Deletes specified company."]
        #[doc = " Prerequisite: The company has no jobs associated with it."]
        async fn delete_company(
            &self,
            request: tonic::Request<super::DeleteCompanyRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status>;
        #[doc = " Lists all companies associated with the project."]
        async fn list_companies(
            &self,
            request: tonic::Request<super::ListCompaniesRequest>,
        ) -> Result<tonic::Response<super::ListCompaniesResponse>, tonic::Status>;
    }
    #[doc = " A service that handles company management, including CRUD and enumeration."]
    #[derive(Debug)]
    #[doc(hidden)]
    pub struct CompanyServiceServer<T: CompanyService> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: CompanyService> CompanyServiceServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, None);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, Some(interceptor.into()));
            Self { inner }
        }
    }
    impl<T, B> Service<http::Request<B>> for CompanyServiceServer<T>
    where
        T: CompanyService,
        B: HttpBody + Send + Sync + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/google.cloud.talent.v4beta1.CompanyService/CreateCompany" => {
                    #[allow(non_camel_case_types)]
                    struct CreateCompanySvc<T: CompanyService>(pub Arc<T>);
                    impl<T: CompanyService> tonic::server::UnaryService<super::CreateCompanyRequest>
                        for CreateCompanySvc<T>
                    {
                        type Response = super::Company;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateCompanyRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.create_company(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = CreateCompanySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/google.cloud.talent.v4beta1.CompanyService/GetCompany" => {
                    #[allow(non_camel_case_types)]
                    struct GetCompanySvc<T: CompanyService>(pub Arc<T>);
                    impl<T: CompanyService> tonic::server::UnaryService<super::GetCompanyRequest> for GetCompanySvc<T> {
                        type Response = super::Company;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetCompanyRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.get_company(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetCompanySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/google.cloud.talent.v4beta1.CompanyService/UpdateCompany" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateCompanySvc<T: CompanyService>(pub Arc<T>);
                    impl<T: CompanyService> tonic::server::UnaryService<super::UpdateCompanyRequest>
                        for UpdateCompanySvc<T>
                    {
                        type Response = super::Company;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateCompanyRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.update_company(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = UpdateCompanySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/google.cloud.talent.v4beta1.CompanyService/DeleteCompany" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteCompanySvc<T: CompanyService>(pub Arc<T>);
                    impl<T: CompanyService> tonic::server::UnaryService<super::DeleteCompanyRequest>
                        for DeleteCompanySvc<T>
                    {
                        type Response = ();
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteCompanyRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.delete_company(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = DeleteCompanySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/google.cloud.talent.v4beta1.CompanyService/ListCompanies" => {
                    #[allow(non_camel_case_types)]
                    struct ListCompaniesSvc<T: CompanyService>(pub Arc<T>);
                    impl<T: CompanyService> tonic::server::UnaryService<super::ListCompaniesRequest>
                        for ListCompaniesSvc<T>
                    {
                        type Response = super::ListCompaniesResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListCompaniesRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.list_companies(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ListCompaniesSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .body(tonic::body::BoxBody::empty())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: CompanyService> Clone for CompanyServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: CompanyService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: CompanyService> tonic::transport::NamedService for CompanyServiceServer<T> {
        const NAME: &'static str = "google.cloud.talent.v4beta1.CompanyService";
    }
}
/// Auto-complete parameters.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CompleteQueryRequest {
    /// Required. Resource name of tenant the completion is performed within.
    ///
    /// The format is "projects/{project_id}/tenants/{tenant_id}", for example,
    /// "projects/foo/tenant/bar".
    ///
    /// If tenant id is unspecified, the default tenant is used, for
    /// example, "projects/foo".
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Required. The query used to generate suggestions.
    ///
    /// The maximum number of allowed characters is 255.
    #[prost(string, tag = "2")]
    pub query: std::string::String,
    /// The list of languages of the query. This is
    /// the BCP-47 language code, such as "en-US" or "sr-Latn".
    /// For more information, see
    /// [Tags for Identifying Languages](https://tools.ietf.org/html/bcp47).
    ///
    /// The maximum number of allowed characters is 255.
    #[prost(string, repeated, tag = "3")]
    pub language_codes: ::std::vec::Vec<std::string::String>,
    /// Required. Completion result count.
    ///
    /// The maximum allowed page size is 10.
    #[prost(int32, tag = "4")]
    pub page_size: i32,
    /// If provided, restricts completion to specified company.
    ///
    /// The format is
    /// "projects/{project_id}/tenants/{tenant_id}/companies/{company_id}", for
    /// example, "projects/foo/tenants/bar/companies/baz".
    ///
    /// If tenant id is unspecified, the default tenant is used, for
    /// example, "projects/foo".
    #[prost(string, tag = "5")]
    pub company: std::string::String,
    /// The scope of the completion. The defaults is [CompletionScope.PUBLIC][google.cloud.talent.v4beta1.CompleteQueryRequest.CompletionScope.PUBLIC].
    #[prost(enumeration = "complete_query_request::CompletionScope", tag = "6")]
    pub scope: i32,
    /// The completion topic. The default is [CompletionType.COMBINED][google.cloud.talent.v4beta1.CompleteQueryRequest.CompletionType.COMBINED].
    #[prost(enumeration = "complete_query_request::CompletionType", tag = "7")]
    pub r#type: i32,
}
pub mod complete_query_request {
    /// Enum to specify the scope of completion.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum CompletionScope {
        /// Default value.
        Unspecified = 0,
        /// Suggestions are based only on the data provided by the client.
        Tenant = 1,
        /// Suggestions are based on all jobs data in the system that's visible to
        /// the client
        Public = 2,
    }
    /// Enum to specify auto-completion topics.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum CompletionType {
        /// Default value.
        Unspecified = 0,
        /// Suggest job titles for jobs autocomplete.
        ///
        /// For [CompletionType.JOB_TITLE][google.cloud.talent.v4beta1.CompleteQueryRequest.CompletionType.JOB_TITLE] type, only open jobs with the same
        /// [language_codes][google.cloud.talent.v4beta1.CompleteQueryRequest.language_codes] are returned.
        JobTitle = 1,
        /// Suggest company names for jobs autocomplete.
        ///
        /// For [CompletionType.COMPANY_NAME][google.cloud.talent.v4beta1.CompleteQueryRequest.CompletionType.COMPANY_NAME] type,
        /// only companies having open jobs with the same [language_codes][google.cloud.talent.v4beta1.CompleteQueryRequest.language_codes] are
        /// returned.
        CompanyName = 2,
        /// Suggest both job titles and company names for jobs autocomplete.
        ///
        /// For [CompletionType.COMBINED][google.cloud.talent.v4beta1.CompleteQueryRequest.CompletionType.COMBINED] type, only open jobs with the same
        /// [language_codes][google.cloud.talent.v4beta1.CompleteQueryRequest.language_codes] or companies having open jobs with the same
        /// [language_codes][google.cloud.talent.v4beta1.CompleteQueryRequest.language_codes] are returned.
        Combined = 3,
    }
}
/// Response of auto-complete query.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CompleteQueryResponse {
    /// Results of the matching job/company candidates.
    #[prost(message, repeated, tag = "1")]
    pub completion_results: ::std::vec::Vec<complete_query_response::CompletionResult>,
    /// Additional information for the API invocation, such as the request
    /// tracking id.
    #[prost(message, optional, tag = "2")]
    pub metadata: ::std::option::Option<ResponseMetadata>,
}
pub mod complete_query_response {
    /// Resource that represents completion results.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CompletionResult {
        /// The suggestion for the query.
        #[prost(string, tag = "1")]
        pub suggestion: std::string::String,
        /// The completion topic.
        #[prost(
            enumeration = "super::complete_query_request::CompletionType",
            tag = "2"
        )]
        pub r#type: i32,
        /// The URI of the company image for
        /// [COMPANY_NAME][google.cloud.talent.v4beta1.CompleteQueryRequest.CompletionType.COMPANY_NAME].
        #[prost(string, tag = "3")]
        pub image_uri: std::string::String,
    }
}
#[doc = r" Generated client implementations."]
pub mod completion_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " A service handles auto completion."]
    pub struct CompletionClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl CompletionClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> CompletionClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + HttpBody + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as HttpBody>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = tonic::client::Grpc::with_interceptor(inner, interceptor);
            Self { inner }
        }
        #[doc = " Completes the specified prefix with keyword suggestions."]
        #[doc = " Intended for use by a job search auto-complete search box."]
        pub async fn complete_query(
            &mut self,
            request: impl tonic::IntoRequest<super::CompleteQueryRequest>,
        ) -> Result<tonic::Response<super::CompleteQueryResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.talent.v4beta1.Completion/CompleteQuery",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for CompletionClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for CompletionClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "CompletionClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod completion_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with CompletionServer."]
    #[async_trait]
    pub trait Completion: Send + Sync + 'static {
        #[doc = " Completes the specified prefix with keyword suggestions."]
        #[doc = " Intended for use by a job search auto-complete search box."]
        async fn complete_query(
            &self,
            request: tonic::Request<super::CompleteQueryRequest>,
        ) -> Result<tonic::Response<super::CompleteQueryResponse>, tonic::Status>;
    }
    #[doc = " A service handles auto completion."]
    #[derive(Debug)]
    #[doc(hidden)]
    pub struct CompletionServer<T: Completion> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: Completion> CompletionServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, None);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, Some(interceptor.into()));
            Self { inner }
        }
    }
    impl<T, B> Service<http::Request<B>> for CompletionServer<T>
    where
        T: Completion,
        B: HttpBody + Send + Sync + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/google.cloud.talent.v4beta1.Completion/CompleteQuery" => {
                    #[allow(non_camel_case_types)]
                    struct CompleteQuerySvc<T: Completion>(pub Arc<T>);
                    impl<T: Completion> tonic::server::UnaryService<super::CompleteQueryRequest>
                        for CompleteQuerySvc<T>
                    {
                        type Response = super::CompleteQueryResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CompleteQueryRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.complete_query(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = CompleteQuerySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .body(tonic::body::BoxBody::empty())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: Completion> Clone for CompletionServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: Completion> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Completion> tonic::transport::NamedService for CompletionServer<T> {
        const NAME: &'static str = "google.cloud.talent.v4beta1.Completion";
    }
}
/// An event issued when an end user interacts with the application that
/// implements Cloud Talent Solution. Providing this information improves the
/// quality of results for the API clients, enabling the
/// service to perform optimally. The number of events sent must be consistent
/// with other calls, such as job searches, issued to the service by the client.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClientEvent {
    /// Strongly recommended for the best service experience.
    ///
    /// A unique ID generated in the API responses. It can be found in
    /// [ResponseMetadata.request_id][google.cloud.talent.v4beta1.ResponseMetadata.request_id].
    #[prost(string, tag = "1")]
    pub request_id: std::string::String,
    /// Required. A unique identifier, generated by the client application.
    #[prost(string, tag = "2")]
    pub event_id: std::string::String,
    /// Required. The timestamp of the event.
    #[prost(message, optional, tag = "4")]
    pub create_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Notes about the event provided by recruiters or other users, for example,
    /// feedback on why a profile was bookmarked.
    #[prost(string, tag = "9")]
    pub event_notes: std::string::String,
    /// Required.
    ///
    /// The detail information of a specific event type.
    #[prost(oneof = "client_event::Event", tags = "5, 6")]
    pub event: ::std::option::Option<client_event::Event>,
}
pub mod client_event {
    /// Required.
    ///
    /// The detail information of a specific event type.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Event {
        /// An event issued when a job seeker interacts with the application that
        /// implements Cloud Talent Solution.
        #[prost(message, tag = "5")]
        JobEvent(super::JobEvent),
        /// An event issued when a profile searcher interacts with the application
        /// that implements Cloud Talent Solution.
        #[prost(message, tag = "6")]
        ProfileEvent(super::ProfileEvent),
    }
}
/// An event issued when a job seeker interacts with the application that
/// implements Cloud Talent Solution.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct JobEvent {
    /// Required. The type of the event (see [JobEventType][google.cloud.talent.v4beta1.JobEvent.JobEventType]).
    #[prost(enumeration = "job_event::JobEventType", tag = "1")]
    pub r#type: i32,
    /// Required. The [job name(s)][google.cloud.talent.v4beta1.Job.name] associated with this event.
    /// For example, if this is an [impression][google.cloud.talent.v4beta1.JobEvent.JobEventType.IMPRESSION] event,
    /// this field contains the identifiers of all jobs shown to the job seeker.
    /// If this was a [view][google.cloud.talent.v4beta1.JobEvent.JobEventType.VIEW] event, this field contains the
    /// identifier of the viewed job.
    ///
    /// The format is
    /// "projects/{project_id}/tenants/{tenant_id}/jobs/{job_id}", for
    /// example, "projects/foo/tenants/bar/jobs/baz".
    #[prost(string, repeated, tag = "2")]
    pub jobs: ::std::vec::Vec<std::string::String>,
    /// The [profile name][google.cloud.talent.v4beta1.Profile.name] associated with this client event.
    ///
    /// The format is
    /// "projects/{project_id}/tenants/{tenant_id}/profiles/{profile_id}",
    /// for example, "projects/foo/tenants/bar/profiles/baz".
    #[prost(string, tag = "3")]
    pub profile: std::string::String,
}
pub mod job_event {
    /// An enumeration of an event attributed to the behavior of the end user,
    /// such as a job seeker.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum JobEventType {
        /// The event is unspecified by other provided values.
        Unspecified = 0,
        /// The job seeker or other entity interacting with the service has
        /// had a job rendered in their view, such as in a list of search results in
        /// a compressed or clipped format. This event is typically associated with
        /// the viewing of a jobs list on a single page by a job seeker.
        Impression = 1,
        /// The job seeker, or other entity interacting with the service, has
        /// viewed the details of a job, including the full description. This
        /// event doesn't apply to the viewing a snippet of a job appearing as a
        /// part of the job search results. Viewing a snippet is associated with an
        /// [impression][google.cloud.talent.v4beta1.JobEvent.JobEventType.IMPRESSION]).
        View = 2,
        /// The job seeker or other entity interacting with the service
        /// performed an action to view a job and was redirected to a different
        /// website for job.
        ViewRedirect = 3,
        /// The job seeker or other entity interacting with the service
        /// began the process or demonstrated the intention of applying for a job.
        ApplicationStart = 4,
        /// The job seeker or other entity interacting with the service
        /// submitted an application for a job.
        ApplicationFinish = 5,
        /// The job seeker or other entity interacting with the service
        /// submitted an application for a job with a single click without
        /// entering information. If a job seeker performs this action, send only
        /// this event to the service. Do not also send
        /// [JobEventType.APPLICATION_START][google.cloud.talent.v4beta1.JobEvent.JobEventType.APPLICATION_START] or [JobEventType.APPLICATION_FINISH][google.cloud.talent.v4beta1.JobEvent.JobEventType.APPLICATION_FINISH]
        /// events.
        ApplicationQuickSubmission = 6,
        /// The job seeker or other entity interacting with the service
        /// performed an action to apply to a job and was redirected to a different
        /// website to complete the application.
        ApplicationRedirect = 7,
        /// The job seeker or other entity interacting with the service began the
        /// process or demonstrated the intention of applying for a job from the
        /// search results page without viewing the details of the job posting.
        /// If sending this event, JobEventType.VIEW event shouldn't be sent.
        ApplicationStartFromSearch = 8,
        /// The job seeker, or other entity interacting with the service, performs an
        /// action with a single click from the search results page to apply to a job
        /// (without viewing the details of the job posting), and is redirected
        /// to a different website to complete the application. If a candidate
        /// performs this action, send only this event to the service. Do not also
        /// send [JobEventType.APPLICATION_START][google.cloud.talent.v4beta1.JobEvent.JobEventType.APPLICATION_START],
        /// [JobEventType.APPLICATION_FINISH][google.cloud.talent.v4beta1.JobEvent.JobEventType.APPLICATION_FINISH] or [JobEventType.VIEW][google.cloud.talent.v4beta1.JobEvent.JobEventType.VIEW] events.
        ApplicationRedirectFromSearch = 9,
        /// This event should be used when a company submits an application
        /// on behalf of a job seeker. This event is intended for use by staffing
        /// agencies attempting to place candidates.
        ApplicationCompanySubmit = 10,
        /// The job seeker or other entity interacting with the service demonstrated
        /// an interest in a job by bookmarking or saving it.
        Bookmark = 11,
        /// The job seeker or other entity interacting with the service was
        /// sent a notification, such as an email alert or device notification,
        /// containing one or more jobs listings generated by the service.
        Notification = 12,
        /// The job seeker or other entity interacting with the service was
        /// employed by the hiring entity (employer). Send this event
        /// only if the job seeker was hired through an application that was
        /// initiated by a search conducted through the Cloud Talent Solution
        /// service.
        Hired = 13,
        /// A recruiter or staffing agency submitted an application on behalf of the
        /// candidate after interacting with the service to identify a suitable job
        /// posting.
        SentCv = 14,
        /// The entity interacting with the service (for example, the job seeker),
        /// was granted an initial interview by the hiring entity (employer). This
        /// event should only be sent if the job seeker was granted an interview as
        /// part of an application that was initiated by a search conducted through /
        /// recommendation provided by the Cloud Talent Solution service.
        InterviewGranted = 15,
    }
}
/// An event issued when a profile searcher interacts with the application
/// that implements Cloud Talent Solution.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProfileEvent {
    /// Required. Type of event.
    #[prost(enumeration = "profile_event::ProfileEventType", tag = "1")]
    pub r#type: i32,
    /// Required. The [profile name(s)][google.cloud.talent.v4beta1.Profile.name] associated with this client event.
    ///
    /// The format is
    /// "projects/{project_id}/tenants/{tenant_id}/profiles/{profile_id}",
    /// for example, "projects/foo/tenants/bar/profiles/baz".
    #[prost(string, repeated, tag = "2")]
    pub profiles: ::std::vec::Vec<std::string::String>,
    /// The [job name(s)][google.cloud.talent.v4beta1.Job.name] associated with this client event. Leave it
    /// empty if the event isn't associated with a job.
    ///
    /// The format is
    /// "projects/{project_id}/tenants/{tenant_id}/jobs/{job_id}", for
    /// example, "projects/foo/tenants/bar/jobs/baz".
    #[prost(string, repeated, tag = "6")]
    pub jobs: ::std::vec::Vec<std::string::String>,
}
pub mod profile_event {
    /// The enum represents types of client events for a candidate profile.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ProfileEventType {
        /// Default value.
        Unspecified = 0,
        /// Send this event when a [ProfileEvent.profiles][google.cloud.talent.v4beta1.ProfileEvent.profiles] was sent as a part of
        /// a result set for a CTS API call and was rendered in the end user's UI
        /// (that is, the [ProfileEvent.recruiter][google.cloud.talent.v4beta1.ProfileEvent.recruiter]).
        Impression = 1,
        /// The VIEW event records the action of a candidate's profile being
        /// viewed by an end user. This is critical to tracking product metrics and
        /// should be sent for every profile VIEW that happens in your system,
        /// whether the event is associated with an API call (for example, a
        /// recruiter making a request for a result set and clicking on a profile)
        /// or not (a recruiter using the system to view profile details without
        /// making a request).
        ///
        /// For a VIEW events associated with API calls, the
        /// [ClientEvent.request_id][google.cloud.talent.v4beta1.ClientEvent.request_id] should be populated.  If the VIEW is not
        /// associated with an API call, [request_id][google.cloud.talent.v4beta1.ClientEvent.request_id] should
        /// not be populated.
        ///
        /// This event requires a valid recruiter and one valid ID in profiles.
        View = 2,
        /// The profile is bookmarked.
        Bookmark = 3,
    }
}
/// The report event request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateClientEventRequest {
    /// Required. Resource name of the tenant under which the event is created.
    ///
    /// The format is "projects/{project_id}/tenants/{tenant_id}", for example,
    /// "projects/foo/tenant/bar". If tenant id is unspecified, a default tenant
    /// is created, for example, "projects/foo".
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Required. Events issued when end user interacts with customer's application that
    /// uses Cloud Talent Solution.
    #[prost(message, optional, tag = "2")]
    pub client_event: ::std::option::Option<ClientEvent>,
}
#[doc = r" Generated client implementations."]
pub mod event_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " A service handles client event report."]
    pub struct EventServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl EventServiceClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> EventServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + HttpBody + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as HttpBody>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = tonic::client::Grpc::with_interceptor(inner, interceptor);
            Self { inner }
        }
        #[doc = " Report events issued when end user interacts with customer's application"]
        #[doc = " that uses Cloud Talent Solution. You may inspect the created events in"]
        #[doc = " [self service"]
        #[doc = " tools](https://console.cloud.google.com/talent-solution/overview)."]
        #[doc = " [Learn"]
        #[doc = " more](https://cloud.google.com/talent-solution/docs/management-tools)"]
        #[doc = " about self service tools."]
        pub async fn create_client_event(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateClientEventRequest>,
        ) -> Result<tonic::Response<super::ClientEvent>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.talent.v4beta1.EventService/CreateClientEvent",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for EventServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for EventServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "EventServiceClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod event_service_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with EventServiceServer."]
    #[async_trait]
    pub trait EventService: Send + Sync + 'static {
        #[doc = " Report events issued when end user interacts with customer's application"]
        #[doc = " that uses Cloud Talent Solution. You may inspect the created events in"]
        #[doc = " [self service"]
        #[doc = " tools](https://console.cloud.google.com/talent-solution/overview)."]
        #[doc = " [Learn"]
        #[doc = " more](https://cloud.google.com/talent-solution/docs/management-tools)"]
        #[doc = " about self service tools."]
        async fn create_client_event(
            &self,
            request: tonic::Request<super::CreateClientEventRequest>,
        ) -> Result<tonic::Response<super::ClientEvent>, tonic::Status>;
    }
    #[doc = " A service handles client event report."]
    #[derive(Debug)]
    #[doc(hidden)]
    pub struct EventServiceServer<T: EventService> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: EventService> EventServiceServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, None);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, Some(interceptor.into()));
            Self { inner }
        }
    }
    impl<T, B> Service<http::Request<B>> for EventServiceServer<T>
    where
        T: EventService,
        B: HttpBody + Send + Sync + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/google.cloud.talent.v4beta1.EventService/CreateClientEvent" => {
                    #[allow(non_camel_case_types)]
                    struct CreateClientEventSvc<T: EventService>(pub Arc<T>);
                    impl<T: EventService>
                        tonic::server::UnaryService<super::CreateClientEventRequest>
                        for CreateClientEventSvc<T>
                    {
                        type Response = super::ClientEvent;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateClientEventRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.create_client_event(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = CreateClientEventSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .body(tonic::body::BoxBody::empty())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: EventService> Clone for EventServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: EventService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: EventService> tonic::transport::NamedService for EventServiceServer<T> {
        const NAME: &'static str = "google.cloud.talent.v4beta1.EventService";
    }
}
/// The query required to perform a search query.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct JobQuery {
    /// The query string that matches against the job title, description, and
    /// location fields.
    ///
    /// The maximum number of allowed characters is 255.
    #[prost(string, tag = "1")]
    pub query: std::string::String,
    /// The language code of [query][google.cloud.talent.v4beta1.JobQuery.query]. For example, "en-US". This field helps to
    /// better interpret the query.
    ///
    /// If a value isn't specified, the query language code is automatically
    /// detected, which may not be accurate.
    ///
    /// Language code should be in BCP-47 format, such as "en-US" or "sr-Latn".
    /// For more information, see
    /// [Tags for Identifying Languages](https://tools.ietf.org/html/bcp47).
    #[prost(string, tag = "14")]
    pub query_language_code: std::string::String,
    /// This filter specifies the company entities to search against.
    ///
    /// If a value isn't specified, jobs are searched for against all
    /// companies.
    ///
    /// If multiple values are specified, jobs are searched against the
    /// companies specified.
    ///
    /// The format is
    /// "projects/{project_id}/tenants/{tenant_id}/companies/{company_id}". For
    /// example, "projects/foo/tenants/bar/companies/baz".
    ///
    /// If tenant id is unspecified, the default tenant is used. For
    /// example, "projects/foo/companies/bar".
    ///
    /// At most 20 company filters are allowed.
    #[prost(string, repeated, tag = "2")]
    pub companies: ::std::vec::Vec<std::string::String>,
    /// The location filter specifies geo-regions containing the jobs to
    /// search against. See [LocationFilter][google.cloud.talent.v4beta1.LocationFilter] for more information.
    ///
    /// If a location value isn't specified, jobs fitting the other search
    /// criteria are retrieved regardless of where they're located.
    ///
    /// If multiple values are specified, jobs are retrieved from any of the
    /// specified locations. If different values are specified for the
    /// [LocationFilter.distance_in_miles][google.cloud.talent.v4beta1.LocationFilter.distance_in_miles] parameter, the maximum provided
    /// distance is used for all locations.
    ///
    /// At most 5 location filters are allowed.
    #[prost(message, repeated, tag = "3")]
    pub location_filters: ::std::vec::Vec<LocationFilter>,
    /// The category filter specifies the categories of jobs to search against.
    /// See [JobCategory][google.cloud.talent.v4beta1.JobCategory] for more information.
    ///
    /// If a value isn't specified, jobs from any category are searched against.
    ///
    /// If multiple values are specified, jobs from any of the specified
    /// categories are searched against.
    #[prost(enumeration = "JobCategory", repeated, tag = "4")]
    pub job_categories: ::std::vec::Vec<i32>,
    /// Allows filtering jobs by commute time with different travel methods (for
    ///  example, driving or public transit).
    ///
    /// Note: This only works when you specify a [CommuteMethod][google.cloud.talent.v4beta1.CommuteMethod]. In this case,
    /// [location_filters][google.cloud.talent.v4beta1.JobQuery.location_filters] is ignored.
    ///
    ///  Currently we don't support sorting by commute time.
    #[prost(message, optional, tag = "5")]
    pub commute_filter: ::std::option::Option<CommuteFilter>,
    /// This filter specifies the exact company [Company.display_name][google.cloud.talent.v4beta1.Company.display_name]
    /// of the jobs to search against.
    ///
    /// If a value isn't specified, jobs within the search results are
    /// associated with any company.
    ///
    /// If multiple values are specified, jobs within the search results may be
    /// associated with any of the specified companies.
    ///
    /// At most 20 company display name filters are allowed.
    #[prost(string, repeated, tag = "6")]
    pub company_display_names: ::std::vec::Vec<std::string::String>,
    /// This search filter is applied only to
    /// [Job.compensation_info][google.cloud.talent.v4beta1.Job.compensation_info]. For example, if the filter is specified
    /// as "Hourly job with per-hour compensation > $15", only jobs meeting
    /// these criteria are searched. If a filter isn't defined, all open jobs
    /// are searched.
    #[prost(message, optional, tag = "7")]
    pub compensation_filter: ::std::option::Option<CompensationFilter>,
    /// This filter specifies a structured syntax to match against the
    /// [Job.custom_attributes][google.cloud.talent.v4beta1.Job.custom_attributes] marked as `filterable`.
    ///
    /// The syntax for this expression is a subset of SQL syntax.
    ///
    /// Supported operators are: `=`, `!=`, `<`, `<=`, `>`, and `>=` where the
    /// left of the operator is a custom field key and the right of the operator
    /// is a number or a quoted string. You must escape backslash (\\) and
    /// quote (\") characters.
    ///
    /// Supported functions are `LOWER([field_name])` to
    /// perform a case insensitive match and `EMPTY([field_name])` to filter on the
    /// existence of a key.
    ///
    /// Boolean expressions (AND/OR/NOT) are supported up to 3 levels of
    /// nesting (for example, "((A AND B AND C) OR NOT D) AND E"), a maximum of 100
    /// comparisons or functions are allowed in the expression. The expression
    /// must be < 6000 bytes in length.
    ///
    /// Sample Query:
    /// `(LOWER(driving_license)="class \"a\"" OR EMPTY(driving_license)) AND
    /// driving_years > 10`
    #[prost(string, tag = "8")]
    pub custom_attribute_filter: std::string::String,
    /// This flag controls the spell-check feature. If false, the
    /// service attempts to correct a misspelled query,
    /// for example, "enginee" is corrected to "engineer".
    ///
    /// Defaults to false: a spell check is performed.
    #[prost(bool, tag = "9")]
    pub disable_spell_check: bool,
    /// The employment type filter specifies the employment type of jobs to
    /// search against, such as [EmploymentType.FULL_TIME][google.cloud.talent.v4beta1.EmploymentType.FULL_TIME].
    ///
    /// If a value isn't specified, jobs in the search results includes any
    /// employment type.
    ///
    /// If multiple values are specified, jobs in the search results include
    /// any of the specified employment types.
    #[prost(enumeration = "EmploymentType", repeated, tag = "10")]
    pub employment_types: ::std::vec::Vec<i32>,
    /// This filter specifies the locale of jobs to search against,
    /// for example, "en-US".
    ///
    /// If a value isn't specified, the search results can contain jobs in any
    /// locale.
    ///
    ///
    /// Language codes should be in BCP-47 format, such as "en-US" or "sr-Latn".
    /// For more information, see
    /// [Tags for Identifying Languages](https://tools.ietf.org/html/bcp47).
    ///
    /// At most 10 language code filters are allowed.
    #[prost(string, repeated, tag = "11")]
    pub language_codes: ::std::vec::Vec<std::string::String>,
    /// Jobs published within a range specified by this filter are searched
    /// against.
    #[prost(message, optional, tag = "12")]
    pub publish_time_range: ::std::option::Option<TimestampRange>,
    /// This filter specifies a list of job names to be excluded during search.
    ///
    /// At most 400 excluded job names are allowed.
    #[prost(string, repeated, tag = "13")]
    pub excluded_jobs: ::std::vec::Vec<std::string::String>,
}
/// Filters to apply when performing the search query.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProfileQuery {
    /// Keywords to match any text fields of profiles.
    ///
    /// For example, "software engineer in Palo Alto".
    #[prost(string, tag = "1")]
    pub query: std::string::String,
    /// The location filter specifies geo-regions containing the profiles to
    /// search against.
    ///
    /// One of [LocationFilter.address][google.cloud.talent.v4beta1.LocationFilter.address] or [LocationFilter.lat_lng][google.cloud.talent.v4beta1.LocationFilter.lat_lng] must be
    /// provided or an error is thrown. If both [LocationFilter.address][google.cloud.talent.v4beta1.LocationFilter.address] and
    /// [LocationFilter.lat_lng][google.cloud.talent.v4beta1.LocationFilter.lat_lng] are provided, an error is thrown.
    ///
    /// The following logic is used to determine which locations in
    /// the profile to filter against:
    ///
    /// 1. All of the profile's geocoded [Profile.addresses][google.cloud.talent.v4beta1.Profile.addresses] where
    /// [Address.usage][google.cloud.talent.v4beta1.Address.usage] is PERSONAL and [Address.current][google.cloud.talent.v4beta1.Address.current] is true.
    ///
    /// 2. If the above set of locations is empty, all of the profile's geocoded
    /// [Profile.addresses][google.cloud.talent.v4beta1.Profile.addresses] where [Address.usage][google.cloud.talent.v4beta1.Address.usage] is
    /// CONTACT_INFO_USAGE_UNSPECIFIED and [Address.current][google.cloud.talent.v4beta1.Address.current] is true.
    ///
    /// 3. If the above set of locations is empty, all of the profile's geocoded
    /// [Profile.addresses][google.cloud.talent.v4beta1.Profile.addresses] where [Address.usage][google.cloud.talent.v4beta1.Address.usage] is PERSONAL or
    /// CONTACT_INFO_USAGE_UNSPECIFIED and [Address.current][google.cloud.talent.v4beta1.Address.current] is not set.
    ///
    /// This means that any profiles without any [Profile.addresses][google.cloud.talent.v4beta1.Profile.addresses] that match
    /// any of the above criteria will not be included in a search with location
    /// filter. Furthermore, any [Profile.addresses][google.cloud.talent.v4beta1.Profile.addresses] where [Address.usage][google.cloud.talent.v4beta1.Address.usage] is
    /// WORK or SCHOOL or where [Address.current][google.cloud.talent.v4beta1.Address.current] is false are not considered for
    /// location filter.
    ///
    /// If a location filter isn't specified, profiles fitting the other search
    /// criteria are retrieved regardless of where they're located.
    ///
    /// If [LocationFilter.negated][google.cloud.talent.v4beta1.LocationFilter.negated] is specified, the result doesn't contain
    /// profiles from that location.
    ///
    /// If [LocationFilter.address][google.cloud.talent.v4beta1.LocationFilter.address] is provided, the
    /// [LocationType][google.cloud.talent.v4beta1.Location.LocationType], center
    /// point (latitude and longitude), and radius are automatically detected by
    /// the Google Maps Geocoding API and included as well. If
    /// [LocationFilter.address][google.cloud.talent.v4beta1.LocationFilter.address] cannot be geocoded, the filter
    /// falls back to keyword search.
    ///
    /// If the detected
    /// [LocationType][google.cloud.talent.v4beta1.Location.LocationType] is
    /// [LocationType.SUB_ADMINISTRATIVE_AREA][google.cloud.talent.v4beta1.Location.LocationType.SUB_ADMINISTRATIVE_AREA],
    /// [LocationType.ADMINISTRATIVE_AREA][google.cloud.talent.v4beta1.Location.LocationType.ADMINISTRATIVE_AREA],
    /// or
    /// [LocationType.COUNTRY][google.cloud.talent.v4beta1.Location.LocationType.COUNTRY],
    /// the filter is performed against the detected location name (using exact
    /// text matching). Otherwise, the filter is performed against the detected
    /// center point and a radius of detected location radius +
    /// [LocationFilter.distance_in_miles][google.cloud.talent.v4beta1.LocationFilter.distance_in_miles].
    ///
    /// If [LocationFilter.address][google.cloud.talent.v4beta1.LocationFilter.address] is provided,
    /// [LocationFilter.distance_in_miles][google.cloud.talent.v4beta1.LocationFilter.distance_in_miles] is the additional radius on top of the
    /// radius of the location geocoded from [LocationFilter.address][google.cloud.talent.v4beta1.LocationFilter.address]. If
    /// [LocationFilter.lat_lng][google.cloud.talent.v4beta1.LocationFilter.lat_lng] is provided,
    /// [LocationFilter.distance_in_miles][google.cloud.talent.v4beta1.LocationFilter.distance_in_miles] is the only radius that is used.
    ///
    /// [LocationFilter.distance_in_miles][google.cloud.talent.v4beta1.LocationFilter.distance_in_miles] is 10 by default. Note that the value
    /// of [LocationFilter.distance_in_miles][google.cloud.talent.v4beta1.LocationFilter.distance_in_miles] is 0 if it is unset, so the server
    /// does not differentiate [LocationFilter.distance_in_miles][google.cloud.talent.v4beta1.LocationFilter.distance_in_miles] that is
    /// explicitly set to 0 and [LocationFilter.distance_in_miles][google.cloud.talent.v4beta1.LocationFilter.distance_in_miles] that is not
    /// set. Which means that if [LocationFilter.distance_in_miles][google.cloud.talent.v4beta1.LocationFilter.distance_in_miles] is explicitly
    /// set to 0, the server will use the default value of
    /// [LocationFilter.distance_in_miles][google.cloud.talent.v4beta1.LocationFilter.distance_in_miles] which is 10. To work around this and
    /// effectively set [LocationFilter.distance_in_miles][google.cloud.talent.v4beta1.LocationFilter.distance_in_miles] to 0, we recommend
    /// setting [LocationFilter.distance_in_miles][google.cloud.talent.v4beta1.LocationFilter.distance_in_miles] to a very small decimal number
    /// (such as 0.00001).
    ///
    /// If [LocationFilter.distance_in_miles][google.cloud.talent.v4beta1.LocationFilter.distance_in_miles] is negative, an error is thrown.
    #[prost(message, repeated, tag = "2")]
    pub location_filters: ::std::vec::Vec<LocationFilter>,
    /// Job title filter specifies job titles of profiles to match on.
    ///
    /// If a job title isn't specified, profiles with any titles are retrieved.
    ///
    /// If multiple values are specified, profiles are retrieved with any of the
    /// specified job titles.
    ///
    /// If [JobTitleFilter.negated][google.cloud.talent.v4beta1.JobTitleFilter.negated] is specified, the result won't contain
    /// profiles with the job titles.
    ///
    /// For example, search for profiles with a job title "Product Manager".
    #[prost(message, repeated, tag = "3")]
    pub job_title_filters: ::std::vec::Vec<JobTitleFilter>,
    /// Employer filter specifies employers of profiles to match on.
    ///
    /// If an employer filter isn't specified, profiles with any employers are
    /// retrieved.
    ///
    /// If multiple employer filters are specified, profiles with any matching
    /// employers are retrieved.
    ///
    /// If [EmployerFilter.negated][google.cloud.talent.v4beta1.EmployerFilter.negated] is specified, the result won't contain
    /// profiles that match the employers.
    ///
    /// For example, search for profiles that have working experience at "Google
    /// LLC".
    #[prost(message, repeated, tag = "4")]
    pub employer_filters: ::std::vec::Vec<EmployerFilter>,
    /// Education filter specifies education of profiles to match on.
    ///
    /// If an education filter isn't specified, profiles with any education are
    /// retrieved.
    ///
    /// If multiple education filters are specified, profiles that match any
    /// education filters are retrieved.
    ///
    /// If [EducationFilter.negated][google.cloud.talent.v4beta1.EducationFilter.negated] is specified, the result won't contain
    /// profiles that match the educations.
    ///
    /// For example, search for profiles with a master degree.
    #[prost(message, repeated, tag = "5")]
    pub education_filters: ::std::vec::Vec<EducationFilter>,
    /// Skill filter specifies skill of profiles to match on.
    ///
    /// If a skill filter isn't specified, profiles with any skills are retrieved.
    ///
    /// If multiple skill filters are specified, profiles that match any skill
    /// filters are retrieved.
    ///
    /// If [SkillFilter.negated][google.cloud.talent.v4beta1.SkillFilter.negated] is specified, the result won't contain profiles
    /// that match the skills.
    ///
    /// For example, search for profiles that have "Java" and "Python" in skill
    /// list.
    #[prost(message, repeated, tag = "6")]
    pub skill_filters: ::std::vec::Vec<SkillFilter>,
    /// Work experience filter specifies the total working experience of profiles
    /// to match on.
    ///
    /// If a work experience filter isn't specified, profiles with any
    /// professional experience are retrieved.
    ///
    /// If multiple work experience filters are specified, profiles that match any
    /// work experience filters are retrieved.
    ///
    /// For example, search for profiles with 10 years of work experience.
    #[prost(message, repeated, tag = "7")]
    pub work_experience_filter: ::std::vec::Vec<WorkExperienceFilter>,
    /// Time filter specifies the create/update timestamp of the profiles to match
    /// on.
    ///
    /// For example, search for profiles created since "2018-1-1".
    #[prost(message, repeated, tag = "8")]
    pub time_filters: ::std::vec::Vec<TimeFilter>,
    /// The hirable filter specifies the profile's hirable status to match on.
    #[prost(message, optional, tag = "9")]
    pub hirable_filter: ::std::option::Option<bool>,
    /// The application date filters specify application date ranges to match on.
    #[prost(message, repeated, tag = "10")]
    pub application_date_filters: ::std::vec::Vec<ApplicationDateFilter>,
    /// The application outcome notes filters specify the notes for the outcome of
    /// the job application.
    #[prost(message, repeated, tag = "11")]
    pub application_outcome_notes_filters: ::std::vec::Vec<ApplicationOutcomeNotesFilter>,
    /// The application job filters specify the job applied for in the application.
    #[prost(message, repeated, tag = "13")]
    pub application_job_filters: ::std::vec::Vec<ApplicationJobFilter>,
    /// This filter specifies a structured syntax to match against the
    /// [Profile.custom_attributes][google.cloud.talent.v4beta1.Profile.custom_attributes] that are marked as `filterable`.
    ///
    /// The syntax for this expression is a subset of Google SQL syntax.
    ///
    /// String custom attributes: supported operators are =, != where the left of
    /// the operator is a custom field key and the right of the operator is a
    /// string (surrounded by quotes) value.
    ///
    /// Numeric custom attributes: Supported operators are '>', '<' or '='
    /// operators where the left of the operator is a custom field key and the
    /// right of the operator is a numeric value.
    ///
    /// Supported functions are LOWER(<field_name>) to
    /// perform case insensitive match and EMPTY(<field_name>) to filter on the
    /// existence of a key.
    ///
    /// Boolean expressions (AND/OR/NOT) are supported up to 3 levels of
    /// nesting (for example "((A AND B AND C) OR NOT D) AND E"), and there can be
    /// a maximum of 50 comparisons/functions in the expression. The expression
    /// must be < 2000 characters in length.
    ///
    /// Sample Query:
    /// (key1 = "TEST" OR LOWER(key1)="test" OR NOT EMPTY(key1))
    #[prost(string, tag = "15")]
    pub custom_attribute_filter: std::string::String,
    /// Deprecated. Use availability_filters instead.
    ///
    /// The candidate availability filter which filters based on availability
    /// signals.
    ///
    /// Signal 1: Number of days since most recent job application.  See
    /// [Availability.JobApplicationAvailabilitySignal][google.cloud.talent.v4beta1.Availability.JobApplicationAvailabilitySignal] for the details of this
    /// signal.
    ///
    /// Signal 2: Number of days since last profile update. See
    /// [Availability.ProfileUpdateAvailabilitySignal][google.cloud.talent.v4beta1.Availability.ProfileUpdateAvailabilitySignal]
    /// for the details of this signal.
    ///
    /// The candidate availability filter helps a recruiter understand if a
    /// specific candidate is likely to be actively seeking new job opportunities
    /// based on an aggregated set of signals.  Specifically, the intent is NOT to
    /// indicate the candidate's potential qualification / interest / close ability
    /// for a specific job.
    #[prost(message, optional, tag = "16")]
    pub candidate_availability_filter: ::std::option::Option<CandidateAvailabilityFilter>,
    /// The availability filter which filters based on
    /// [Profile.availability_signals][google.cloud.talent.v4beta1.Profile.availability_signals].
    ///
    /// The availability filter helps a recruiter understand if a
    /// specific candidate is likely to be actively seeking new job opportunities
    /// based on an aggregated set of signals.  Specifically, the intent is NOT to
    /// indicate the candidate's potential qualification / interest / close ability
    /// for a specific job.
    ///
    /// There can be at most one [AvailabilityFilter][google.cloud.talent.v4beta1.AvailabilityFilter] per
    /// [signal_type][google.cloud.talent.v4beta1.AvailabilityFilter.signal_type]. If there are multiple
    /// [AvailabilityFilter][google.cloud.talent.v4beta1.AvailabilityFilter] for a [signal_type][google.cloud.talent.v4beta1.AvailabilityFilter.signal_type],
    /// an error is thrown.
    #[prost(message, repeated, tag = "18")]
    pub availability_filters: ::std::vec::Vec<AvailabilityFilter>,
    /// Person name filter specifies person name of profiles to match on.
    ///
    /// If multiple person name filters are specified, profiles that match any
    /// person name filters are retrieved.
    ///
    /// For example, search for profiles of candidates with name "John Smith".
    #[prost(message, repeated, tag = "17")]
    pub person_name_filters: ::std::vec::Vec<PersonNameFilter>,
}
/// Geographic region of the search.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LocationFilter {
    /// The address name, such as "Mountain View" or "Bay Area".
    #[prost(string, tag = "1")]
    pub address: std::string::String,
    /// CLDR region code of the country/region of the address. This is used
    /// to address ambiguity of the user-input location, for example, "Liverpool"
    /// against "Liverpool, NY, US" or "Liverpool, UK".
    ///
    /// Set this field to bias location resolution toward a specific country
    /// or territory. If this field is not set, application behavior is biased
    /// toward the United States by default.
    ///
    /// See https://cldr.unicode.org/ and
    /// https://www.unicode.org/cldr/charts/30/supplemental/territory_information.html
    /// for details. Example: "CH" for Switzerland.
    /// Note that this filter is not applicable for Profile Search related queries.
    #[prost(string, tag = "2")]
    pub region_code: std::string::String,
    /// The latitude and longitude of the geographic center to search from. This
    /// field is ignored if `address` is provided.
    #[prost(message, optional, tag = "3")]
    pub lat_lng: ::std::option::Option<super::super::super::r#type::LatLng>,
    /// The distance_in_miles is applied when the location being searched for is
    /// identified as a city or smaller. This field is ignored if the location
    /// being searched for is a state or larger.
    #[prost(double, tag = "4")]
    pub distance_in_miles: f64,
    /// Allows the client to return jobs without a
    /// set location, specifically, telecommuting jobs (telecommuting is considered
    /// by the service as a special location.
    /// [Job.posting_region][google.cloud.talent.v4beta1.Job.posting_region] indicates if a job permits telecommuting.
    /// If this field is set to [TelecommutePreference.TELECOMMUTE_ALLOWED][google.cloud.talent.v4beta1.LocationFilter.TelecommutePreference.TELECOMMUTE_ALLOWED],
    /// telecommuting jobs are searched, and [address][google.cloud.talent.v4beta1.LocationFilter.address] and [lat_lng][google.cloud.talent.v4beta1.LocationFilter.lat_lng] are
    /// ignored. If not set or set to
    /// [TelecommutePreference.TELECOMMUTE_EXCLUDED][google.cloud.talent.v4beta1.LocationFilter.TelecommutePreference.TELECOMMUTE_EXCLUDED], telecommute job are not
    /// searched.
    ///
    /// This filter can be used by itself to search exclusively for telecommuting
    /// jobs, or it can be combined with another location
    /// filter to search for a combination of job locations,
    /// such as "Mountain View" or "telecommuting" jobs. However, when used in
    /// combination with other location filters, telecommuting jobs can be
    /// treated as less relevant than other jobs in the search response.
    ///
    /// This field is only used for job search requests.
    #[prost(enumeration = "location_filter::TelecommutePreference", tag = "5")]
    pub telecommute_preference: i32,
    /// Whether to apply negation to the filter so profiles matching the filter
    /// are excluded.
    ///
    /// Currently only supported in profile search.
    #[prost(bool, tag = "6")]
    pub negated: bool,
}
pub mod location_filter {
    /// Specify whether to include telecommute jobs.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum TelecommutePreference {
        /// Default value if the telecommute preference isn't specified.
        Unspecified = 0,
        /// Exclude telecommute jobs.
        TelecommuteExcluded = 1,
        /// Allow telecommute jobs.
        TelecommuteAllowed = 2,
    }
}
/// Filter on job compensation type and amount.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CompensationFilter {
    /// Required. Type of filter.
    #[prost(enumeration = "compensation_filter::FilterType", tag = "1")]
    pub r#type: i32,
    /// Required. Specify desired `base compensation entry's`
    /// [CompensationInfo.CompensationUnit][google.cloud.talent.v4beta1.CompensationInfo.CompensationUnit].
    #[prost(
        enumeration = "compensation_info::CompensationUnit",
        repeated,
        packed = "false",
        tag = "2"
    )]
    pub units: ::std::vec::Vec<i32>,
    /// Compensation range.
    #[prost(message, optional, tag = "3")]
    pub range: ::std::option::Option<compensation_info::CompensationRange>,
    /// If set to true, jobs with unspecified compensation range fields are
    /// included.
    #[prost(bool, tag = "4")]
    pub include_jobs_with_unspecified_compensation_range: bool,
}
pub mod compensation_filter {
    /// Specify the type of filtering.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum FilterType {
        /// Filter type unspecified. Position holder, INVALID, should never be used.
        Unspecified = 0,
        /// Filter by `base compensation entry's` unit. A job is a match if and
        /// only if the job contains a base CompensationEntry and the base
        /// CompensationEntry's unit matches provided [units][google.cloud.talent.v4beta1.CompensationFilter.units].
        /// Populate one or more [units][google.cloud.talent.v4beta1.CompensationFilter.units].
        ///
        /// See [CompensationInfo.CompensationEntry][google.cloud.talent.v4beta1.CompensationInfo.CompensationEntry] for definition of
        /// base compensation entry.
        UnitOnly = 1,
        /// Filter by `base compensation entry's` unit and amount / range. A job
        /// is a match if and only if the job contains a base CompensationEntry, and
        /// the base entry's unit matches provided
        /// [CompensationUnit][google.cloud.talent.v4beta1.CompensationInfo.CompensationUnit] and
        /// amount or range overlaps with provided
        /// [CompensationRange][google.cloud.talent.v4beta1.CompensationInfo.CompensationRange].
        ///
        /// See [CompensationInfo.CompensationEntry][google.cloud.talent.v4beta1.CompensationInfo.CompensationEntry] for definition of
        /// base compensation entry.
        ///
        /// Set exactly one [units][google.cloud.talent.v4beta1.CompensationFilter.units] and populate [range][google.cloud.talent.v4beta1.CompensationFilter.range].
        UnitAndAmount = 2,
        /// Filter by annualized base compensation amount and `base compensation
        /// entry's` unit. Populate [range][google.cloud.talent.v4beta1.CompensationFilter.range] and zero or more [units][google.cloud.talent.v4beta1.CompensationFilter.units].
        AnnualizedBaseAmount = 3,
        /// Filter by annualized total compensation amount and `base compensation
        /// entry's` unit . Populate [range][google.cloud.talent.v4beta1.CompensationFilter.range] and zero or more [units][google.cloud.talent.v4beta1.CompensationFilter.units].
        AnnualizedTotalAmount = 4,
    }
}
/// Parameters needed for commute search.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommuteFilter {
    /// Required. The method of transportation to calculate the commute time for.
    #[prost(enumeration = "CommuteMethod", tag = "1")]
    pub commute_method: i32,
    /// Required. The latitude and longitude of the location to calculate the
    /// commute time from.
    #[prost(message, optional, tag = "2")]
    pub start_coordinates: ::std::option::Option<super::super::super::r#type::LatLng>,
    /// Required. The maximum travel time in seconds. The maximum allowed value is `3600s`
    /// (one hour). Format is `123s`.
    #[prost(message, optional, tag = "3")]
    pub travel_duration: ::std::option::Option<::prost_types::Duration>,
    /// If `true`, jobs without street level addresses may also be returned.
    /// For city level addresses, the city center is used. For state and coarser
    /// level addresses, text matching is used.
    /// If this field is set to `false` or isn't specified, only jobs that include
    /// street level addresses will be returned by commute search.
    #[prost(bool, tag = "4")]
    pub allow_imprecise_addresses: bool,
    /// Traffic factor to take into account while searching by commute.
    #[prost(oneof = "commute_filter::TrafficOption", tags = "5, 6")]
    pub traffic_option: ::std::option::Option<commute_filter::TrafficOption>,
}
pub mod commute_filter {
    /// The traffic density to use when calculating commute time.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum RoadTraffic {
        /// Road traffic situation isn't specified.
        Unspecified = 0,
        /// Optimal commute time without considering any traffic impact.
        TrafficFree = 1,
        /// Commute time calculation takes in account the peak traffic impact.
        BusyHour = 2,
    }
    /// Traffic factor to take into account while searching by commute.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum TrafficOption {
        /// Specifies the traffic density to use when calculating commute time.
        #[prost(enumeration = "RoadTraffic", tag = "5")]
        RoadTraffic(i32),
        /// The departure time used to calculate traffic impact, represented as
        /// [google.type.TimeOfDay][google.type.TimeOfDay] in local time zone.
        ///
        /// Currently traffic model is restricted to hour level resolution.
        #[prost(message, tag = "6")]
        DepartureTime(super::super::super::super::r#type::TimeOfDay),
    }
}
/// Job title of the search.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct JobTitleFilter {
    /// Required. The job title. For example, "Software engineer", or "Product manager".
    #[prost(string, tag = "1")]
    pub job_title: std::string::String,
    /// Whether to apply negation to the filter so profiles matching the filter
    /// are excluded.
    #[prost(bool, tag = "2")]
    pub negated: bool,
}
/// Skill filter of the search.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SkillFilter {
    /// Required. The skill name. For example, "java", "j2ee", and so on.
    #[prost(string, tag = "1")]
    pub skill: std::string::String,
    /// Whether to apply negation to the filter so profiles matching the filter
    /// are excluded.
    #[prost(bool, tag = "2")]
    pub negated: bool,
}
/// Employer filter of the search.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EmployerFilter {
    /// Required. The name of the employer, for example "Google", "Alphabet".
    #[prost(string, tag = "1")]
    pub employer: std::string::String,
    /// Define set of [EmploymentRecord][google.cloud.talent.v4beta1.EmploymentRecord]s to search against.
    ///
    /// Defaults to [EmployerFilterMode.ALL_EMPLOYMENT_RECORDS][google.cloud.talent.v4beta1.EmployerFilter.EmployerFilterMode.ALL_EMPLOYMENT_RECORDS].
    #[prost(enumeration = "employer_filter::EmployerFilterMode", tag = "2")]
    pub mode: i32,
    /// Whether to apply negation to the filter so profiles matching the filter
    /// is excluded.
    #[prost(bool, tag = "3")]
    pub negated: bool,
}
pub mod employer_filter {
    /// Enum indicating which set of [Profile.employment_records][google.cloud.talent.v4beta1.Profile.employment_records] to search
    /// against.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum EmployerFilterMode {
        /// Default value.
        Unspecified = 0,
        /// Apply to all employers in [Profile.employment_records][google.cloud.talent.v4beta1.Profile.employment_records].
        AllEmploymentRecords = 1,
        /// Apply only to current employer in [Profile.employment_records][google.cloud.talent.v4beta1.Profile.employment_records].
        CurrentEmploymentRecordsOnly = 2,
        /// Apply only to past (not current) employers in
        /// [Profile.employment_records][google.cloud.talent.v4beta1.Profile.employment_records].
        PastEmploymentRecordsOnly = 3,
    }
}
/// Education filter of the search.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EducationFilter {
    /// The school name. For example "MIT", "University of California, Berkeley".
    #[prost(string, tag = "1")]
    pub school: std::string::String,
    /// The field of study. This is to search against value provided in
    /// [Degree.fields_of_study][google.cloud.talent.v4beta1.Degree.fields_of_study].
    /// For example "Computer Science", "Mathematics".
    #[prost(string, tag = "2")]
    pub field_of_study: std::string::String,
    /// Education degree in ISCED code. Each value in degree covers a specific
    /// level of education, without any expansion to upper nor lower levels of
    /// education degree.
    #[prost(enumeration = "DegreeType", tag = "3")]
    pub degree_type: i32,
    /// Whether to apply negation to the filter so profiles matching the filter
    /// is excluded.
    #[prost(bool, tag = "6")]
    pub negated: bool,
}
/// Work experience filter.
///
/// This filter is used to search for profiles with working experience length
/// between [min_experience][google.cloud.talent.v4beta1.WorkExperienceFilter.min_experience] and [max_experience][google.cloud.talent.v4beta1.WorkExperienceFilter.max_experience].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WorkExperienceFilter {
    /// The minimum duration of the work experience (inclusive).
    #[prost(message, optional, tag = "1")]
    pub min_experience: ::std::option::Option<::prost_types::Duration>,
    /// The maximum duration of the work experience (exclusive).
    #[prost(message, optional, tag = "2")]
    pub max_experience: ::std::option::Option<::prost_types::Duration>,
}
/// Application Date Range Filter.
///
/// The API matches profiles with [Application.application_date][google.cloud.talent.v4beta1.Application.application_date] between
/// start date and end date (both boundaries are inclusive). The filter is
/// ignored if both [start_date][google.cloud.talent.v4beta1.ApplicationDateFilter.start_date] and [end_date][google.cloud.talent.v4beta1.ApplicationDateFilter.end_date] are missing.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApplicationDateFilter {
    /// Start date. If it's missing, The API matches profiles with application date
    /// not after the end date.
    #[prost(message, optional, tag = "1")]
    pub start_date: ::std::option::Option<super::super::super::r#type::Date>,
    /// End date. If it's missing, The API matches profiles with application date
    /// not before the start date.
    #[prost(message, optional, tag = "2")]
    pub end_date: ::std::option::Option<super::super::super::r#type::Date>,
}
/// Outcome Notes Filter.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApplicationOutcomeNotesFilter {
    /// Required. User entered or selected outcome reason. The API does an exact match on the
    /// [Application.outcome_notes][google.cloud.talent.v4beta1.Application.outcome_notes] in profiles.
    #[prost(string, tag = "1")]
    pub outcome_notes: std::string::String,
    /// If true, The API excludes all candidates with any
    /// [Application.outcome_notes][google.cloud.talent.v4beta1.Application.outcome_notes] matching the outcome reason specified in
    /// the filter.
    #[prost(bool, tag = "2")]
    pub negated: bool,
}
/// Filter on the job information of Application.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApplicationJobFilter {
    /// The job requisition id in the application. The API does an exact match on
    /// the [Job.requisition_id][google.cloud.talent.v4beta1.Job.requisition_id] of [Application.job][google.cloud.talent.v4beta1.Application.job] in profiles.
    #[prost(string, tag = "2")]
    pub job_requisition_id: std::string::String,
    /// The job title in the application. The API does an exact match on the
    /// [Job.title][google.cloud.talent.v4beta1.Job.title] of [Application.job][google.cloud.talent.v4beta1.Application.job] in profiles.
    #[prost(string, tag = "3")]
    pub job_title: std::string::String,
    /// If true, the API excludes all profiles with any [Application.job][google.cloud.talent.v4beta1.Application.job]
    /// matching the filters.
    #[prost(bool, tag = "4")]
    pub negated: bool,
}
/// Filter on create timestamp or update timestamp of profiles.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TimeFilter {
    /// Start timestamp, matching profiles with the start time. If this field
    /// missing, The API matches profiles with create / update timestamp before the
    /// end timestamp.
    #[prost(message, optional, tag = "1")]
    pub start_time: ::std::option::Option<::prost_types::Timestamp>,
    /// End timestamp, matching profiles with the end time. If this field
    /// missing, The API matches profiles with create / update timestamp after the
    /// start timestamp.
    #[prost(message, optional, tag = "2")]
    pub end_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Specifies which time field to filter profiles.
    ///
    /// Defaults to [TimeField.CREATE_TIME][google.cloud.talent.v4beta1.TimeFilter.TimeField.CREATE_TIME].
    #[prost(enumeration = "time_filter::TimeField", tag = "3")]
    pub time_field: i32,
}
pub mod time_filter {
    /// Time fields can be used in TimeFilter.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum TimeField {
        /// Default value.
        Unspecified = 0,
        /// Earliest profile create time.
        CreateTime = 1,
        /// Latest profile update time.
        UpdateTime = 2,
    }
}
/// Deprecated. Use AvailabilityFilter instead.
///
/// Filter on availability signals.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CandidateAvailabilityFilter {
    /// It is false by default. If true, API excludes all the potential available
    /// profiles.
    #[prost(bool, tag = "1")]
    pub negated: bool,
}
/// Filter on availability signals.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AvailabilityFilter {
    /// Required. Type of signal to apply filter on.
    #[prost(enumeration = "AvailabilitySignalType", tag = "1")]
    pub signal_type: i32,
    /// Required. Range of times to filter candidate signals by.
    #[prost(message, optional, tag = "2")]
    pub range: ::std::option::Option<TimestampRange>,
    /// If multiple [AvailabilityFilter][google.cloud.talent.v4beta1.AvailabilityFilter] are provided, the default
    /// behavior is to OR all filters, but if this field is set to true, this
    /// particular [AvailabilityFilter][google.cloud.talent.v4beta1.AvailabilityFilter] will be AND'ed against other
    /// [AvailabilityFilter][google.cloud.talent.v4beta1.AvailabilityFilter].
    #[prost(bool, tag = "3")]
    pub required: bool,
}
/// Filter on person name.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PersonNameFilter {
    /// Required. The person name. For example, "John Smith".
    ///
    /// Can be any combination of [PersonName.structured_name.given_name][],
    /// [PersonName.structured_name.middle_initial][],
    /// [PersonName.structured_name.family_name][], and
    /// [PersonName.formatted_name][google.cloud.talent.v4beta1.PersonName.formatted_name].
    #[prost(string, tag = "1")]
    pub person_name: std::string::String,
}
/// The histogram request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HistogramQuery {
    /// An expression specifies a histogram request against matching resources
    /// (for example, jobs, profiles) for searches.
    ///
    /// See [SearchJobsRequest.histogram_queries][google.cloud.talent.v4beta1.SearchJobsRequest.histogram_queries] and
    /// [SearchProfilesRequest.histogram_queries][google.cloud.talent.v4beta1.SearchProfilesRequest.histogram_queries] for details about syntax.
    #[prost(string, tag = "1")]
    pub histogram_query: std::string::String,
}
/// Histogram result that matches [HistogramQuery][google.cloud.talent.v4beta1.HistogramQuery] specified in searches.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HistogramQueryResult {
    /// Requested histogram expression.
    #[prost(string, tag = "1")]
    pub histogram_query: std::string::String,
    /// A map from the values of the facet associated with distinct values to the
    /// number of matching entries with corresponding value.
    ///
    /// The key format is:
    ///
    /// * (for string histogram) string values stored in the field.
    /// * (for named numeric bucket) name specified in `bucket()` function, like
    ///   for `bucket(0, MAX, "non-negative")`, the key will be `non-negative`.
    /// * (for anonymous numeric bucket) range formatted as `<low>-<high>`, for
    ///   example, `0-1000`, `MIN-0`, and `0-MAX`.
    #[prost(map = "string, int64", tag = "2")]
    pub histogram: ::std::collections::HashMap<std::string::String, i64>,
}
/// A Job resource represents a job posting (also referred to as a "job listing"
/// or "job requisition"). A job belongs to a [Company][google.cloud.talent.v4beta1.Company], which is the hiring
/// entity responsible for the job.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Job {
    /// Required during job update.
    ///
    /// The resource name for the job. This is generated by the service when a
    /// job is created.
    ///
    /// The format is
    /// "projects/{project_id}/tenants/{tenant_id}/jobs/{job_id}". For
    /// example, "projects/foo/tenants/bar/jobs/baz".
    ///
    /// If tenant id is unspecified, the default tenant is used. For
    /// example, "projects/foo/jobs/bar".
    ///
    /// Use of this field in job queries and API calls is preferred over the use of
    /// [requisition_id][google.cloud.talent.v4beta1.Job.requisition_id] since this value is unique.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Required. The resource name of the company listing the job.
    ///
    /// The format is
    /// "projects/{project_id}/tenants/{tenant_id}/companies/{company_id}". For
    /// example, "projects/foo/tenants/bar/companies/baz".
    ///
    /// If tenant id is unspecified, the default tenant is used. For
    /// example, "projects/foo/companies/bar".
    #[prost(string, tag = "2")]
    pub company: std::string::String,
    /// Required. The requisition ID, also referred to as the posting ID, is assigned by the
    /// client to identify a job. This field is intended to be used by clients
    /// for client identification and tracking of postings. A job isn't allowed
    /// to be created if there is another job with the same [company][google.cloud.talent.v4beta1.Job.name],
    /// [language_code][google.cloud.talent.v4beta1.Job.language_code] and [requisition_id][google.cloud.talent.v4beta1.Job.requisition_id].
    ///
    /// The maximum number of allowed characters is 255.
    #[prost(string, tag = "3")]
    pub requisition_id: std::string::String,
    /// Required. The title of the job, such as "Software Engineer"
    ///
    /// The maximum number of allowed characters is 500.
    #[prost(string, tag = "4")]
    pub title: std::string::String,
    /// Required. The description of the job, which typically includes a multi-paragraph
    /// description of the company and related information. Separate fields are
    /// provided on the job object for [responsibilities][google.cloud.talent.v4beta1.Job.responsibilities],
    /// [qualifications][google.cloud.talent.v4beta1.Job.qualifications], and other job characteristics. Use of
    /// these separate job fields is recommended.
    ///
    /// This field accepts and sanitizes HTML input, and also accepts
    /// bold, italic, ordered list, and unordered list markup tags.
    ///
    /// The maximum number of allowed characters is 100,000.
    #[prost(string, tag = "5")]
    pub description: std::string::String,
    /// Strongly recommended for the best service experience.
    ///
    /// Location(s) where the employer is looking to hire for this job posting.
    ///
    /// Specifying the full street address(es) of the hiring location enables
    /// better API results, especially job searches by commute time.
    ///
    /// At most 50 locations are allowed for best search performance. If a job has
    /// more locations, it is suggested to split it into multiple jobs with unique
    /// [requisition_id][google.cloud.talent.v4beta1.Job.requisition_id]s (e.g. 'ReqA' becomes 'ReqA-1', 'ReqA-2', and so on.) as
    /// multiple jobs with the same [company][google.cloud.talent.v4beta1.Job.company], [language_code][google.cloud.talent.v4beta1.Job.language_code] and
    /// [requisition_id][google.cloud.talent.v4beta1.Job.requisition_id] are not allowed. If the original [requisition_id][google.cloud.talent.v4beta1.Job.requisition_id] must
    /// be preserved, a custom field should be used for storage. It is also
    /// suggested to group the locations that close to each other in the same job
    /// for better search experience.
    ///
    /// The maximum number of allowed characters is 500.
    #[prost(string, repeated, tag = "6")]
    pub addresses: ::std::vec::Vec<std::string::String>,
    /// Job application information.
    #[prost(message, optional, tag = "7")]
    pub application_info: ::std::option::Option<job::ApplicationInfo>,
    /// The benefits included with the job.
    #[prost(enumeration = "JobBenefit", repeated, tag = "8")]
    pub job_benefits: ::std::vec::Vec<i32>,
    /// Job compensation information (a.k.a. "pay rate") i.e., the compensation
    /// that will paid to the employee.
    #[prost(message, optional, tag = "9")]
    pub compensation_info: ::std::option::Option<CompensationInfo>,
    /// A map of fields to hold both filterable and non-filterable custom job
    /// attributes that are not covered by the provided structured fields.
    ///
    /// The keys of the map are strings up to 64 bytes and must match the
    /// pattern: [a-zA-Z][a-zA-Z0-9_]*. For example, key0LikeThis or
    /// KEY_1_LIKE_THIS.
    ///
    /// At most 100 filterable and at most 100 unfilterable keys are supported.
    /// For filterable `string_values`, across all keys at most 200 values are
    /// allowed, with each string no more than 255 characters. For unfilterable
    /// `string_values`, the maximum total size of `string_values` across all keys
    /// is 50KB.
    #[prost(map = "string, message", tag = "10")]
    pub custom_attributes: ::std::collections::HashMap<std::string::String, CustomAttribute>,
    /// The desired education degrees for the job, such as Bachelors, Masters.
    #[prost(enumeration = "DegreeType", repeated, tag = "11")]
    pub degree_types: ::std::vec::Vec<i32>,
    /// The department or functional area within the company with the open
    /// position.
    ///
    /// The maximum number of allowed characters is 255.
    #[prost(string, tag = "12")]
    pub department: std::string::String,
    /// The employment type(s) of a job, for example,
    /// [full time][google.cloud.talent.v4beta1.EmploymentType.FULL_TIME] or
    /// [part time][google.cloud.talent.v4beta1.EmploymentType.PART_TIME].
    #[prost(enumeration = "EmploymentType", repeated, tag = "13")]
    pub employment_types: ::std::vec::Vec<i32>,
    /// A description of bonus, commission, and other compensation
    /// incentives associated with the job not including salary or pay.
    ///
    /// The maximum number of allowed characters is 10,000.
    #[prost(string, tag = "14")]
    pub incentives: std::string::String,
    /// The language of the posting. This field is distinct from
    /// any requirements for fluency that are associated with the job.
    ///
    /// Language codes must be in BCP-47 format, such as "en-US" or "sr-Latn".
    /// For more information, see
    /// [Tags for Identifying Languages](https://tools.ietf.org/html/bcp47){:
    /// class="external" target="_blank" }.
    ///
    /// If this field is unspecified and [Job.description][google.cloud.talent.v4beta1.Job.description] is present, detected
    /// language code based on [Job.description][google.cloud.talent.v4beta1.Job.description] is assigned, otherwise
    /// defaults to 'en_US'.
    #[prost(string, tag = "15")]
    pub language_code: std::string::String,
    /// The experience level associated with the job, such as "Entry Level".
    #[prost(enumeration = "JobLevel", tag = "16")]
    pub job_level: i32,
    /// A promotion value of the job, as determined by the client.
    /// The value determines the sort order of the jobs returned when searching for
    /// jobs using the featured jobs search call, with higher promotional values
    /// being returned first and ties being resolved by relevance sort. Only the
    /// jobs with a promotionValue >0 are returned in a FEATURED_JOB_SEARCH.
    ///
    /// Default value is 0, and negative values are treated as 0.
    #[prost(int32, tag = "17")]
    pub promotion_value: i32,
    /// A description of the qualifications required to perform the
    /// job. The use of this field is recommended
    /// as an alternative to using the more general [description][google.cloud.talent.v4beta1.Job.description] field.
    ///
    /// This field accepts and sanitizes HTML input, and also accepts
    /// bold, italic, ordered list, and unordered list markup tags.
    ///
    /// The maximum number of allowed characters is 10,000.
    #[prost(string, tag = "18")]
    pub qualifications: std::string::String,
    /// A description of job responsibilities. The use of this field is
    /// recommended as an alternative to using the more general [description][google.cloud.talent.v4beta1.Job.description]
    /// field.
    ///
    /// This field accepts and sanitizes HTML input, and also accepts
    /// bold, italic, ordered list, and unordered list markup tags.
    ///
    /// The maximum number of allowed characters is 10,000.
    #[prost(string, tag = "19")]
    pub responsibilities: std::string::String,
    /// The job [PostingRegion][google.cloud.talent.v4beta1.PostingRegion] (for example, state, country) throughout
    /// which the job is available. If this field is set, a [LocationFilter][google.cloud.talent.v4beta1.LocationFilter]
    /// in a search query within the job region finds this job posting if an
    /// exact location match isn't specified. If this field is set to
    /// [PostingRegion.NATION][google.cloud.talent.v4beta1.PostingRegion.NATION] or [PostingRegion.ADMINISTRATIVE_AREA][google.cloud.talent.v4beta1.PostingRegion.ADMINISTRATIVE_AREA],
    /// setting job [Job.addresses][google.cloud.talent.v4beta1.Job.addresses] to the same location level as this field
    /// is strongly recommended.
    #[prost(enumeration = "PostingRegion", tag = "20")]
    pub posting_region: i32,
    /// Deprecated. The job is only visible to the owner.
    ///
    /// The visibility of the job.
    ///
    /// Defaults to [Visibility.ACCOUNT_ONLY][google.cloud.talent.v4beta1.Visibility.ACCOUNT_ONLY] if not specified.
    #[prost(enumeration = "Visibility", tag = "21")]
    pub visibility: i32,
    /// The start timestamp of the job in UTC time zone. Typically this field
    /// is used for contracting engagements. Invalid timestamps are ignored.
    #[prost(message, optional, tag = "22")]
    pub job_start_time: ::std::option::Option<::prost_types::Timestamp>,
    /// The end timestamp of the job. Typically this field is used for contracting
    /// engagements. Invalid timestamps are ignored.
    #[prost(message, optional, tag = "23")]
    pub job_end_time: ::std::option::Option<::prost_types::Timestamp>,
    /// The timestamp this job posting was most recently published. The default
    /// value is the time the request arrives at the server. Invalid timestamps are
    /// ignored.
    #[prost(message, optional, tag = "24")]
    pub posting_publish_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Strongly recommended for the best service experience.
    ///
    /// The expiration timestamp of the job. After this timestamp, the
    /// job is marked as expired, and it no longer appears in search results. The
    /// expired job can't be listed by the [ListJobs][google.cloud.talent.v4beta1.JobService.ListJobs] API,
    /// but it can be retrieved with the [GetJob][google.cloud.talent.v4beta1.JobService.GetJob] API or
    /// updated with the [UpdateJob][google.cloud.talent.v4beta1.JobService.UpdateJob] API or deleted with
    /// the [DeleteJob][google.cloud.talent.v4beta1.JobService.DeleteJob] API. An expired job can
    /// be updated and opened again by using a future expiration timestamp.
    /// Updating an expired job fails if there is another existing open job with
    /// same [company][google.cloud.talent.v4beta1.Job.company], [language_code][google.cloud.talent.v4beta1.Job.language_code] and [requisition_id][google.cloud.talent.v4beta1.Job.requisition_id].
    ///
    /// The expired jobs are retained in our system for 90 days. However, the
    /// overall expired job count cannot exceed 3 times the maximum number of
    /// open jobs over previous 7 days. If this threshold is exceeded,
    /// expired jobs are cleaned out in order of earliest expire time.
    /// Expired jobs are no longer accessible after they are cleaned
    /// out.
    ///
    /// Invalid timestamps are ignored, and treated as expire time not provided.
    ///
    /// If the timestamp is before the instant request is made, the job
    /// is treated as expired immediately on creation. This kind of job can
    /// not be updated. And when creating a job with past timestamp, the
    /// [posting_publish_time][google.cloud.talent.v4beta1.Job.posting_publish_time] must be set before
    /// [posting_expire_time][google.cloud.talent.v4beta1.Job.posting_expire_time]. The purpose of this feature is
    /// to allow other objects, such as [Application][google.cloud.talent.v4beta1.Application], to refer a job
    /// that didn't exist in the system prior to becoming expired. If you
    /// want to modify a job that was expired on creation,
    /// delete it and create a new one.
    ///
    /// If this value isn't provided at the time of job creation or is invalid,
    /// the job posting expires after 30 days from the job's creation time. For
    /// example, if the job was created on 2017/01/01 13:00AM UTC with an
    /// unspecified expiration date, the job expires after 2017/01/31 13:00AM UTC.
    ///
    /// If this value isn't provided on job update, it depends on the field masks
    /// set by [UpdateJobRequest.update_mask][google.cloud.talent.v4beta1.UpdateJobRequest.update_mask]. If the field masks include
    /// [job_end_time][google.cloud.talent.v4beta1.Job.job_end_time], or the masks are empty meaning that every field is
    /// updated, the job posting expires after 30 days from the job's last
    /// update time. Otherwise the expiration date isn't updated.
    #[prost(message, optional, tag = "25")]
    pub posting_expire_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Output only. The timestamp when this job posting was created.
    #[prost(message, optional, tag = "26")]
    pub posting_create_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Output only. The timestamp when this job posting was last updated.
    #[prost(message, optional, tag = "27")]
    pub posting_update_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Output only. Display name of the company listing the job.
    #[prost(string, tag = "28")]
    pub company_display_name: std::string::String,
    /// Output only. Derived details about the job posting.
    #[prost(message, optional, tag = "29")]
    pub derived_info: ::std::option::Option<job::DerivedInfo>,
    /// Options for job processing.
    #[prost(message, optional, tag = "30")]
    pub processing_options: ::std::option::Option<job::ProcessingOptions>,
}
pub mod job {
    /// Application related details of a job posting.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ApplicationInfo {
        /// Use this field to specify email address(es) to which resumes or
        /// applications can be sent.
        ///
        /// The maximum number of allowed characters for each entry is 255.
        #[prost(string, repeated, tag = "1")]
        pub emails: ::std::vec::Vec<std::string::String>,
        /// Use this field to provide instructions, such as "Mail your application
        /// to ...", that a candidate can follow to apply for the job.
        ///
        /// This field accepts and sanitizes HTML input, and also accepts
        /// bold, italic, ordered list, and unordered list markup tags.
        ///
        /// The maximum number of allowed characters is 3,000.
        #[prost(string, tag = "2")]
        pub instruction: std::string::String,
        /// Use this URI field to direct an applicant to a website, for example to
        /// link to an online application form.
        ///
        /// The maximum number of allowed characters for each entry is 2,000.
        #[prost(string, repeated, tag = "3")]
        pub uris: ::std::vec::Vec<std::string::String>,
    }
    /// Derived details about the job posting.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DerivedInfo {
        /// Structured locations of the job, resolved from [Job.addresses][google.cloud.talent.v4beta1.Job.addresses].
        ///
        /// [locations][google.cloud.talent.v4beta1.Job.DerivedInfo.locations] are exactly matched to [Job.addresses][google.cloud.talent.v4beta1.Job.addresses] in the same
        /// order.
        #[prost(message, repeated, tag = "1")]
        pub locations: ::std::vec::Vec<super::Location>,
        /// Job categories derived from [Job.title][google.cloud.talent.v4beta1.Job.title] and [Job.description][google.cloud.talent.v4beta1.Job.description].
        #[prost(enumeration = "super::JobCategory", repeated, tag = "3")]
        pub job_categories: ::std::vec::Vec<i32>,
    }
    /// Options for job processing.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ProcessingOptions {
        /// If set to `true`, the service does not attempt to resolve a
        /// more precise address for the job.
        #[prost(bool, tag = "1")]
        pub disable_street_address_resolution: bool,
        /// Option for job HTML content sanitization. Applied fields are:
        ///
        /// * description
        /// * applicationInfo.instruction
        /// * incentives
        /// * qualifications
        /// * responsibilities
        ///
        /// HTML tags in these fields may be stripped if sanitiazation isn't
        /// disabled.
        ///
        /// Defaults to [HtmlSanitization.SIMPLE_FORMATTING_ONLY][google.cloud.talent.v4beta1.HtmlSanitization.SIMPLE_FORMATTING_ONLY].
        #[prost(enumeration = "super::HtmlSanitization", tag = "2")]
        pub html_sanitization: i32,
    }
}
/// Create job request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateJobRequest {
    /// Required. The resource name of the tenant under which the job is created.
    ///
    /// The format is "projects/{project_id}/tenants/{tenant_id}". For example,
    /// "projects/foo/tenant/bar". If tenant id is unspecified a default tenant
    /// is created. For example, "projects/foo".
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Required. The Job to be created.
    #[prost(message, optional, tag = "2")]
    pub job: ::std::option::Option<Job>,
}
/// Get job request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetJobRequest {
    /// Required. The resource name of the job to retrieve.
    ///
    /// The format is
    /// "projects/{project_id}/tenants/{tenant_id}/jobs/{job_id}". For
    /// example, "projects/foo/tenants/bar/jobs/baz".
    ///
    /// If tenant id is unspecified, the default tenant is used. For
    /// example, "projects/foo/jobs/bar".
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Update job request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateJobRequest {
    /// Required. The Job to be updated.
    #[prost(message, optional, tag = "1")]
    pub job: ::std::option::Option<Job>,
    /// Strongly recommended for the best service experience.
    ///
    /// If [update_mask][google.cloud.talent.v4beta1.UpdateJobRequest.update_mask] is provided, only the specified fields in
    /// [job][google.cloud.talent.v4beta1.UpdateJobRequest.job] are updated. Otherwise all the fields are updated.
    ///
    /// A field mask to restrict the fields that are updated. Only
    /// top level fields of [Job][google.cloud.talent.v4beta1.Job] are supported.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
}
/// Delete job request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteJobRequest {
    /// Required. The resource name of the job to be deleted.
    ///
    /// The format is
    /// "projects/{project_id}/tenants/{tenant_id}/jobs/{job_id}". For
    /// example, "projects/foo/tenants/bar/jobs/baz".
    ///
    /// If tenant id is unspecified, the default tenant is used. For
    /// example, "projects/foo/jobs/bar".
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Batch delete jobs request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchDeleteJobsRequest {
    /// Required. The resource name of the tenant under which the job is created.
    ///
    /// The format is "projects/{project_id}/tenants/{tenant_id}". For example,
    /// "projects/foo/tenant/bar". If tenant id is unspecified, a default tenant
    /// is created. For example, "projects/foo".
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Required. The filter string specifies the jobs to be deleted.
    ///
    /// Supported operator: =, AND
    ///
    /// The fields eligible for filtering are:
    ///
    /// * `companyName` (Required)
    /// * `requisitionId` (Required)
    ///
    /// Sample Query: companyName = "projects/foo/companies/bar" AND
    /// requisitionId = "req-1"
    #[prost(string, tag = "2")]
    pub filter: std::string::String,
}
/// List jobs request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListJobsRequest {
    /// Required. The resource name of the tenant under which the job is created.
    ///
    /// The format is "projects/{project_id}/tenants/{tenant_id}". For example,
    /// "projects/foo/tenant/bar". If tenant id is unspecified, a default tenant
    /// is created. For example, "projects/foo".
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Required. The filter string specifies the jobs to be enumerated.
    ///
    /// Supported operator: =, AND
    ///
    /// The fields eligible for filtering are:
    ///
    /// * `companyName` (Required)
    /// * `requisitionId`
    /// * `status` Available values: OPEN, EXPIRED, ALL. Defaults to
    /// OPEN if no value is specified.
    ///
    /// Sample Query:
    ///
    /// * companyName = "projects/foo/tenants/bar/companies/baz"
    /// * companyName = "projects/foo/tenants/bar/companies/baz" AND
    /// requisitionId = "req-1"
    /// * companyName = "projects/foo/tenants/bar/companies/baz" AND
    /// status = "EXPIRED"
    #[prost(string, tag = "2")]
    pub filter: std::string::String,
    /// The starting point of a query result.
    #[prost(string, tag = "3")]
    pub page_token: std::string::String,
    /// The maximum number of jobs to be returned per page of results.
    ///
    /// If [job_view][google.cloud.talent.v4beta1.ListJobsRequest.job_view] is set to [JobView.JOB_VIEW_ID_ONLY][google.cloud.talent.v4beta1.JobView.JOB_VIEW_ID_ONLY], the maximum allowed
    /// page size is 1000. Otherwise, the maximum allowed page size is 100.
    ///
    /// Default is 100 if empty or a number < 1 is specified.
    #[prost(int32, tag = "4")]
    pub page_size: i32,
    /// The desired job attributes returned for jobs in the
    /// search response. Defaults to [JobView.JOB_VIEW_FULL][google.cloud.talent.v4beta1.JobView.JOB_VIEW_FULL] if no value is
    /// specified.
    #[prost(enumeration = "JobView", tag = "5")]
    pub job_view: i32,
}
/// List jobs response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListJobsResponse {
    /// The Jobs for a given company.
    ///
    /// The maximum number of items returned is based on the limit field
    /// provided in the request.
    #[prost(message, repeated, tag = "1")]
    pub jobs: ::std::vec::Vec<Job>,
    /// A token to retrieve the next page of results.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
    /// Additional information for the API invocation, such as the request
    /// tracking id.
    #[prost(message, optional, tag = "3")]
    pub metadata: ::std::option::Option<ResponseMetadata>,
}
/// The Request body of the `SearchJobs` call.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchJobsRequest {
    /// Required. The resource name of the tenant to search within.
    ///
    /// The format is "projects/{project_id}/tenants/{tenant_id}". For example,
    /// "projects/foo/tenant/bar". If tenant id is unspecified, a default tenant
    /// is created. For example, "projects/foo".
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Mode of a search.
    ///
    /// Defaults to [SearchMode.JOB_SEARCH][google.cloud.talent.v4beta1.SearchJobsRequest.SearchMode.JOB_SEARCH].
    #[prost(enumeration = "search_jobs_request::SearchMode", tag = "2")]
    pub search_mode: i32,
    /// Required. The meta information collected about the job searcher, used to improve the
    /// search quality of the service. The identifiers (such as `user_id`) are
    /// provided by users, and must be unique and consistent.
    #[prost(message, optional, tag = "3")]
    pub request_metadata: ::std::option::Option<RequestMetadata>,
    /// Query used to search against jobs, such as keyword, location filters, etc.
    #[prost(message, optional, tag = "4")]
    pub job_query: ::std::option::Option<JobQuery>,
    /// Controls whether to broaden the search when it produces sparse results.
    /// Broadened queries append results to the end of the matching results
    /// list.
    ///
    /// Defaults to false.
    #[prost(bool, tag = "5")]
    pub enable_broadening: bool,
    /// Controls if the search job request requires the return of a precise
    /// count of the first 300 results. Setting this to `true` ensures
    /// consistency in the number of results per page. Best practice is to set this
    /// value to true if a client allows users to jump directly to a
    /// non-sequential search results page.
    ///
    /// Enabling this flag may adversely impact performance.
    ///
    /// Defaults to false.
    #[prost(bool, tag = "6")]
    pub require_precise_result_size: bool,
    /// An expression specifies a histogram request against matching jobs.
    ///
    /// Expression syntax is an aggregation function call with histogram facets and
    /// other options.
    ///
    /// Available aggregation function calls are:
    /// * `count(string_histogram_facet)`: Count the number of matching entities,
    /// for each distinct attribute value.
    /// * `count(numeric_histogram_facet, list of buckets)`: Count the number of
    /// matching entities within each bucket.
    ///
    /// Data types:
    ///
    /// * Histogram facet: facet names with format [a-zA-Z][a-zA-Z0-9_]+.
    /// * String: string like "any string with backslash escape for quote(\")."
    /// * Number: whole number and floating point number like 10, -1 and -0.01.
    /// * List: list of elements with comma(,) separator surrounded by square
    /// brackets, for example, [1, 2, 3] and ["one", "two", "three"].
    ///
    /// Built-in constants:
    ///
    /// * MIN (minimum number similar to java Double.MIN_VALUE)
    /// * MAX (maximum number similar to java Double.MAX_VALUE)
    ///
    /// Built-in functions:
    ///
    /// * bucket(start, end[, label]): bucket built-in function creates a bucket
    /// with range of [start, end). Note that the end is exclusive, for example,
    /// bucket(1, MAX, "positive number") or bucket(1, 10).
    ///
    /// Job histogram facets:
    ///
    /// * company_display_name: histogram by [Job.company_display_name][google.cloud.talent.v4beta1.Job.company_display_name].
    /// * employment_type: histogram by [Job.employment_types][google.cloud.talent.v4beta1.Job.employment_types], for example,
    ///   "FULL_TIME", "PART_TIME".
    /// * company_size: histogram by [CompanySize][google.cloud.talent.v4beta1.CompanySize], for example, "SMALL",
    /// "MEDIUM", "BIG".
    /// * publish_time_in_month: histogram by the [Job.posting_publish_time][google.cloud.talent.v4beta1.Job.posting_publish_time]
    ///   in months.
    ///   Must specify list of numeric buckets in spec.
    /// * publish_time_in_year: histogram by the [Job.posting_publish_time][google.cloud.talent.v4beta1.Job.posting_publish_time]
    ///   in years.
    ///   Must specify list of numeric buckets in spec.
    /// * degree_types: histogram by the [Job.degree_types][google.cloud.talent.v4beta1.Job.degree_types], for example,
    ///   "Bachelors", "Masters".
    /// * job_level: histogram by the [Job.job_level][google.cloud.talent.v4beta1.Job.job_level], for example, "Entry
    ///   Level".
    /// * country: histogram by the country code of jobs, for example, "US", "FR".
    /// * admin1: histogram by the admin1 code of jobs, which is a global
    ///   placeholder referring to the state, province, or the particular term a
    ///   country uses to define the geographic structure below the country level,
    ///   for example, "CA", "IL".
    /// * city: histogram by a combination of the "city name, admin1 code". For
    ///   example,  "Mountain View, CA", "New York, NY".
    /// * admin1_country: histogram by a combination of the "admin1 code, country",
    ///   for example, "CA, US", "IL, US".
    /// * city_coordinate: histogram by the city center's GPS coordinates (latitude
    ///   and longitude), for example, 37.4038522,-122.0987765. Since the
    ///   coordinates of a city center can change, customers may need to refresh
    ///   them periodically.
    /// * locale: histogram by the [Job.language_code][google.cloud.talent.v4beta1.Job.language_code], for example, "en-US",
    ///   "fr-FR".
    /// * language: histogram by the language subtag of the [Job.language_code][google.cloud.talent.v4beta1.Job.language_code],
    ///   for example, "en", "fr".
    /// * category: histogram by the [JobCategory][google.cloud.talent.v4beta1.JobCategory], for example,
    ///   "COMPUTER_AND_IT", "HEALTHCARE".
    /// * base_compensation_unit: histogram by the
    ///   [CompensationInfo.CompensationUnit][google.cloud.talent.v4beta1.CompensationInfo.CompensationUnit] of base
    ///   salary, for example, "WEEKLY", "MONTHLY".
    /// * base_compensation: histogram by the base salary. Must specify list of
    ///   numeric buckets to group results by.
    /// * annualized_base_compensation: histogram by the base annualized salary.
    ///   Must specify list of numeric buckets to group results by.
    /// * annualized_total_compensation: histogram by the total annualized salary.
    ///   Must specify list of numeric buckets to group results by.
    /// * string_custom_attribute: histogram by string [Job.custom_attributes][google.cloud.talent.v4beta1.Job.custom_attributes].
    ///   Values can be accessed via square bracket notations like
    ///   string_custom_attribute["key1"].
    /// * numeric_custom_attribute: histogram by numeric [Job.custom_attributes][google.cloud.talent.v4beta1.Job.custom_attributes].
    ///   Values can be accessed via square bracket notations like
    ///   numeric_custom_attribute["key1"]. Must specify list of numeric buckets to
    ///   group results by.
    ///
    /// Example expressions:
    ///
    /// * `count(admin1)`
    /// * `count(base_compensation, [bucket(1000, 10000), bucket(10000, 100000),
    /// bucket(100000, MAX)])`
    /// * `count(string_custom_attribute["some-string-custom-attribute"])`
    /// * `count(numeric_custom_attribute["some-numeric-custom-attribute"],
    ///   [bucket(MIN, 0, "negative"), bucket(0, MAX, "non-negative"])`
    #[prost(message, repeated, tag = "7")]
    pub histogram_queries: ::std::vec::Vec<HistogramQuery>,
    /// The desired job attributes returned for jobs in the search response.
    /// Defaults to [JobView.JOB_VIEW_SMALL][google.cloud.talent.v4beta1.JobView.JOB_VIEW_SMALL] if no value is specified.
    #[prost(enumeration = "JobView", tag = "8")]
    pub job_view: i32,
    /// An integer that specifies the current offset (that is, starting result
    /// location, amongst the jobs deemed by the API as relevant) in search
    /// results. This field is only considered if [page_token][google.cloud.talent.v4beta1.SearchJobsRequest.page_token] is unset.
    ///
    /// The maximum allowed value is 5000. Otherwise an error is thrown.
    ///
    /// For example, 0 means to  return results starting from the first matching
    /// job, and 10 means to return from the 11th job. This can be used for
    /// pagination, (for example, pageSize = 10 and offset = 10 means to return
    /// from the second page).
    #[prost(int32, tag = "9")]
    pub offset: i32,
    /// A limit on the number of jobs returned in the search results.
    /// Increasing this value above the default value of 10 can increase search
    /// response time. The value can be between 1 and 100.
    #[prost(int32, tag = "10")]
    pub page_size: i32,
    /// The token specifying the current offset within
    /// search results. See [SearchJobsResponse.next_page_token][google.cloud.talent.v4beta1.SearchJobsResponse.next_page_token] for
    /// an explanation of how to obtain the next set of query results.
    #[prost(string, tag = "11")]
    pub page_token: std::string::String,
    /// The criteria determining how search results are sorted. Default is
    /// `"relevance desc"`.
    ///
    /// Supported options are:
    ///
    /// * `"relevance desc"`: By relevance descending, as determined by the API
    ///   algorithms. Relevance thresholding of query results is only available
    ///   with this ordering.
    /// * `"posting_publish_time desc"`: By [Job.posting_publish_time][google.cloud.talent.v4beta1.Job.posting_publish_time]
    ///   descending.
    /// * `"posting_update_time desc"`: By [Job.posting_update_time][google.cloud.talent.v4beta1.Job.posting_update_time]
    ///   descending.
    /// * `"title"`: By [Job.title][google.cloud.talent.v4beta1.Job.title] ascending.
    /// * `"title desc"`: By [Job.title][google.cloud.talent.v4beta1.Job.title] descending.
    /// * `"annualized_base_compensation"`: By job's
    ///   [CompensationInfo.annualized_base_compensation_range][google.cloud.talent.v4beta1.CompensationInfo.annualized_base_compensation_range] ascending. Jobs
    ///   whose annualized base compensation is unspecified are put at the end of
    ///   search results.
    /// * `"annualized_base_compensation desc"`: By job's
    ///   [CompensationInfo.annualized_base_compensation_range][google.cloud.talent.v4beta1.CompensationInfo.annualized_base_compensation_range] descending. Jobs
    ///   whose annualized base compensation is unspecified are put at the end of
    ///   search results.
    /// * `"annualized_total_compensation"`: By job's
    ///   [CompensationInfo.annualized_total_compensation_range][google.cloud.talent.v4beta1.CompensationInfo.annualized_total_compensation_range] ascending. Jobs
    ///   whose annualized base compensation is unspecified are put at the end of
    ///   search results.
    /// * `"annualized_total_compensation desc"`: By job's
    ///   [CompensationInfo.annualized_total_compensation_range][google.cloud.talent.v4beta1.CompensationInfo.annualized_total_compensation_range] descending. Jobs
    ///   whose annualized base compensation is unspecified are put at the end of
    ///   search results.
    /// * `"custom_ranking desc"`: By the relevance score adjusted to the
    ///   [SearchJobsRequest.CustomRankingInfo.ranking_expression][google.cloud.talent.v4beta1.SearchJobsRequest.CustomRankingInfo.ranking_expression] with weight
    ///   factor assigned by
    ///   [SearchJobsRequest.CustomRankingInfo.importance_level][google.cloud.talent.v4beta1.SearchJobsRequest.CustomRankingInfo.importance_level] in descending
    ///   order.
    /// * Location sorting: Use the special syntax to order jobs by distance:<br>
    ///   `"distance_from('Hawaii')"`: Order by distance from Hawaii.<br>
    ///   `"distance_from(19.89, 155.5)"`: Order by distance from a coordinate.<br>
    ///   `"distance_from('Hawaii'), distance_from('Puerto Rico')"`: Order by
    ///   multiple locations. See details below.<br>
    ///   `"distance_from('Hawaii'), distance_from(19.89, 155.5)"`: Order by
    ///   multiple locations. See details below.<br>
    ///   The string can have a maximum of 256 characters. When multiple distance
    ///   centers are provided, a job that is close to any of the distance centers
    ///   would have a high rank. When a job has multiple locations, the job
    ///   location closest to one of the distance centers will be used. Jobs that
    ///   don't have locations will be ranked at the bottom. Distance is calculated
    ///   with a precision of 11.3 meters (37.4 feet). Diversification strategy is
    ///   still applied unless explicitly disabled in
    ///   [diversification_level][google.cloud.talent.v4beta1.SearchJobsRequest.diversification_level].
    #[prost(string, tag = "12")]
    pub order_by: std::string::String,
    /// Controls whether highly similar jobs are returned next to each other in
    /// the search results. Jobs are identified as highly similar based on
    /// their titles, job categories, and locations. Highly similar results are
    /// clustered so that only one representative job of the cluster is
    /// displayed to the job seeker higher up in the results, with the other jobs
    /// being displayed lower down in the results.
    ///
    /// Defaults to [DiversificationLevel.SIMPLE][google.cloud.talent.v4beta1.SearchJobsRequest.DiversificationLevel.SIMPLE] if no value
    /// is specified.
    #[prost(enumeration = "search_jobs_request::DiversificationLevel", tag = "13")]
    pub diversification_level: i32,
    /// Controls over how job documents get ranked on top of existing relevance
    /// score (determined by API algorithm).
    #[prost(message, optional, tag = "14")]
    pub custom_ranking_info: ::std::option::Option<search_jobs_request::CustomRankingInfo>,
    /// Controls whether to disable exact keyword match on [Job.title][google.cloud.talent.v4beta1.Job.title],
    /// [Job.description][google.cloud.talent.v4beta1.Job.description], [Job.company_display_name][google.cloud.talent.v4beta1.Job.company_display_name], [Job.addresses][google.cloud.talent.v4beta1.Job.addresses],
    /// [Job.qualifications][google.cloud.talent.v4beta1.Job.qualifications]. When disable keyword match is turned off, a
    /// keyword match returns jobs that do not match given category filters when
    /// there are matching keywords. For example, for the query "program manager,"
    /// a result is returned even if the job posting has the title "software
    /// developer," which doesn't fall into "program manager" ontology, but does
    /// have "program manager" appearing in its description.
    ///
    /// For queries like "cloud" that don't contain title or
    /// location specific ontology, jobs with "cloud" keyword matches are returned
    /// regardless of this flag's value.
    ///
    /// Use [Company.keyword_searchable_job_custom_attributes][google.cloud.talent.v4beta1.Company.keyword_searchable_job_custom_attributes] if
    /// company-specific globally matched custom field/attribute string values are
    /// needed. Enabling keyword match improves recall of subsequent search
    /// requests.
    ///
    /// Defaults to false.
    #[prost(bool, tag = "16")]
    pub disable_keyword_match: bool,
}
pub mod search_jobs_request {
    /// Custom ranking information for [SearchJobsRequest][google.cloud.talent.v4beta1.SearchJobsRequest].
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CustomRankingInfo {
        /// Required. Controls over how important the score of
        /// [CustomRankingInfo.ranking_expression][google.cloud.talent.v4beta1.SearchJobsRequest.CustomRankingInfo.ranking_expression] gets applied to job's final
        /// ranking position.
        ///
        /// An error is thrown if not specified.
        #[prost(enumeration = "custom_ranking_info::ImportanceLevel", tag = "1")]
        pub importance_level: i32,
        /// Required. Controls over how job documents get ranked on top of existing relevance
        /// score (determined by API algorithm). A combination of the ranking
        /// expression and relevance score is used to determine job's final ranking
        /// position.
        ///
        /// The syntax for this expression is a subset of Google SQL syntax.
        ///
        /// Supported operators are: +, -, *, /, where the left and right side of
        /// the operator is either a numeric [Job.custom_attributes][google.cloud.talent.v4beta1.Job.custom_attributes] key,
        /// integer/double value or an expression that can be evaluated to a number.
        ///
        /// Parenthesis are supported to adjust calculation precedence. The
        /// expression must be < 100 characters in length.
        ///
        /// The expression is considered invalid for a job if the expression
        /// references custom attributes that are not populated on the job or if the
        /// expression results in a divide by zero. If an expression is invalid for a
        /// job, that job is demoted to the end of the results.
        ///
        /// Sample ranking expression
        /// (year + 25) * 0.25 - (freshness / 0.5)
        #[prost(string, tag = "2")]
        pub ranking_expression: std::string::String,
    }
    pub mod custom_ranking_info {
        /// The importance level for [CustomRankingInfo.ranking_expression][google.cloud.talent.v4beta1.SearchJobsRequest.CustomRankingInfo.ranking_expression].
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
        #[repr(i32)]
        pub enum ImportanceLevel {
            /// Default value if the importance level isn't specified.
            Unspecified = 0,
            /// The given ranking expression is of None importance, existing relevance
            /// score (determined by API algorithm) dominates job's final ranking
            /// position.
            None = 1,
            /// The given ranking expression is of Low importance in terms of job's
            /// final ranking position compared to existing relevance
            /// score (determined by API algorithm).
            Low = 2,
            /// The given ranking expression is of Mild importance in terms of job's
            /// final ranking position compared to existing relevance
            /// score (determined by API algorithm).
            Mild = 3,
            /// The given ranking expression is of Medium importance in terms of job's
            /// final ranking position compared to existing relevance
            /// score (determined by API algorithm).
            Medium = 4,
            /// The given ranking expression is of High importance in terms of job's
            /// final ranking position compared to existing relevance
            /// score (determined by API algorithm).
            High = 5,
            /// The given ranking expression is of Extreme importance, and dominates
            /// job's final ranking position with existing relevance
            /// score (determined by API algorithm) ignored.
            Extreme = 6,
        }
    }
    /// A string-represented enumeration of the job search mode. The service
    /// operate differently for different modes of service.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum SearchMode {
        /// The mode of the search method isn't specified. The default search
        /// behavior is identical to JOB_SEARCH search behavior.
        Unspecified = 0,
        /// The job search matches against all jobs, and featured jobs
        /// (jobs with promotionValue > 0) are not specially handled.
        JobSearch = 1,
        /// The job search matches only against featured jobs (jobs with a
        /// promotionValue > 0). This method doesn't return any jobs having a
        /// promotionValue <= 0. The search results order is determined by the
        /// promotionValue (jobs with a higher promotionValue are returned higher up
        /// in the search results), with relevance being used as a tiebreaker.
        FeaturedJobSearch = 2,
    }
    /// Controls whether highly similar jobs are returned next to each other in
    /// the search results. Jobs are identified as highly similar based on
    /// their titles, job categories, and locations. Highly similar results are
    /// clustered so that only one representative job of the cluster is
    /// displayed to the job seeker higher up in the results, with the other jobs
    /// being displayed lower down in the results.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum DiversificationLevel {
        /// The diversification level isn't specified.
        Unspecified = 0,
        /// Disables diversification. Jobs that would normally be pushed to the last
        /// page would not have their positions altered. This may result in highly
        /// similar jobs appearing in sequence in the search results.
        Disabled = 1,
        /// Default diversifying behavior. The result list is ordered so that
        /// highly similar results are pushed to the end of the last page of search
        /// results. If you are using pageToken to page through the result set,
        /// latency might be lower but we can't guarantee that all results are
        /// returned. If you are using page offset, latency might be higher but all
        /// results are returned.
        Simple = 2,
    }
}
/// Response for SearchJob method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchJobsResponse {
    /// The Job entities that match the specified [SearchJobsRequest][google.cloud.talent.v4beta1.SearchJobsRequest].
    #[prost(message, repeated, tag = "1")]
    pub matching_jobs: ::std::vec::Vec<search_jobs_response::MatchingJob>,
    /// The histogram results that match with specified
    /// [SearchJobsRequest.histogram_queries][google.cloud.talent.v4beta1.SearchJobsRequest.histogram_queries].
    #[prost(message, repeated, tag = "2")]
    pub histogram_query_results: ::std::vec::Vec<HistogramQueryResult>,
    /// The token that specifies the starting position of the next page of results.
    /// This field is empty if there are no more results.
    #[prost(string, tag = "3")]
    pub next_page_token: std::string::String,
    /// The location filters that the service applied to the specified query. If
    /// any filters are lat-lng based, the [Location.location_type][google.cloud.talent.v4beta1.Location.location_type] is
    /// [Location.LocationType.LOCATION_TYPE_UNSPECIFIED][google.cloud.talent.v4beta1.Location.LocationType.LOCATION_TYPE_UNSPECIFIED].
    #[prost(message, repeated, tag = "4")]
    pub location_filters: ::std::vec::Vec<Location>,
    /// An estimation of the number of jobs that match the specified query.
    ///
    /// This number isn't guaranteed to be accurate. For accurate results,
    /// see [SearchJobsRequest.require_precise_result_size][google.cloud.talent.v4beta1.SearchJobsRequest.require_precise_result_size].
    #[prost(int32, tag = "5")]
    pub estimated_total_size: i32,
    /// The precise result count, which is available only if the client set
    /// [SearchJobsRequest.require_precise_result_size][google.cloud.talent.v4beta1.SearchJobsRequest.require_precise_result_size] to `true`, or if the
    /// response is the last page of results. Otherwise, the value is `-1`.
    #[prost(int32, tag = "6")]
    pub total_size: i32,
    /// Additional information for the API invocation, such as the request
    /// tracking id.
    #[prost(message, optional, tag = "7")]
    pub metadata: ::std::option::Option<ResponseMetadata>,
    /// If query broadening is enabled, we may append additional results from the
    /// broadened query. This number indicates how many of the jobs returned in the
    /// jobs field are from the broadened query. These results are always at the
    /// end of the jobs list. In particular, a value of 0, or if the field isn't
    /// set, all the jobs in the jobs list are from the original
    /// (without broadening) query. If this field is non-zero, subsequent requests
    /// with offset after this result set should contain all broadened results.
    #[prost(int32, tag = "8")]
    pub broadened_query_jobs_count: i32,
    /// The spell checking result, and correction.
    #[prost(message, optional, tag = "9")]
    pub spell_correction: ::std::option::Option<SpellingCorrection>,
}
pub mod search_jobs_response {
    /// Job entry with metadata inside [SearchJobsResponse][google.cloud.talent.v4beta1.SearchJobsResponse].
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct MatchingJob {
        /// Job resource that matches the specified [SearchJobsRequest][google.cloud.talent.v4beta1.SearchJobsRequest].
        #[prost(message, optional, tag = "1")]
        pub job: ::std::option::Option<super::Job>,
        /// A summary of the job with core information that's displayed on the search
        /// results listing page.
        #[prost(string, tag = "2")]
        pub job_summary: std::string::String,
        /// Contains snippets of text from the [Job.title][google.cloud.talent.v4beta1.Job.title] field most
        /// closely matching a search query's keywords, if available. The matching
        /// query keywords are enclosed in HTML bold tags.
        #[prost(string, tag = "3")]
        pub job_title_snippet: std::string::String,
        /// Contains snippets of text from the [Job.description][google.cloud.talent.v4beta1.Job.description] and similar
        /// fields that most closely match a search query's keywords, if available.
        /// All HTML tags in the original fields are stripped when returned in this
        /// field, and matching query keywords are enclosed in HTML bold tags.
        #[prost(string, tag = "4")]
        pub search_text_snippet: std::string::String,
        /// Commute information which is generated based on specified
        ///  [CommuteFilter][google.cloud.talent.v4beta1.CommuteFilter].
        #[prost(message, optional, tag = "5")]
        pub commute_info: ::std::option::Option<CommuteInfo>,
    }
    /// Commute details related to this job.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CommuteInfo {
        /// Location used as the destination in the commute calculation.
        #[prost(message, optional, tag = "1")]
        pub job_location: ::std::option::Option<super::Location>,
        /// The number of seconds required to travel to the job location from the
        /// query location. A duration of 0 seconds indicates that the job isn't
        /// reachable within the requested duration, but was returned as part of an
        /// expanded query.
        #[prost(message, optional, tag = "2")]
        pub travel_duration: ::std::option::Option<::prost_types::Duration>,
    }
}
/// Request to create a batch of jobs.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchCreateJobsRequest {
    /// Required. The resource name of the tenant under which the job is created.
    ///
    /// The format is "projects/{project_id}/tenants/{tenant_id}". For example,
    /// "projects/foo/tenant/bar". If tenant id is unspecified, a default tenant
    /// is created. For example, "projects/foo".
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Required. The jobs to be created.
    #[prost(message, repeated, tag = "2")]
    pub jobs: ::std::vec::Vec<Job>,
}
/// Request to update a batch of jobs.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchUpdateJobsRequest {
    /// Required. The resource name of the tenant under which the job is created.
    ///
    /// The format is "projects/{project_id}/tenants/{tenant_id}". For example,
    /// "projects/foo/tenant/bar". If tenant id is unspecified, a default tenant
    /// is created. For example, "projects/foo".
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Required. The jobs to be updated.
    #[prost(message, repeated, tag = "2")]
    pub jobs: ::std::vec::Vec<Job>,
    /// Strongly recommended for the best service experience. Be aware that it will
    /// also increase latency when checking the status of a batch operation.
    ///
    /// If [update_mask][google.cloud.talent.v4beta1.BatchUpdateJobsRequest.update_mask] is provided, only the specified fields in
    /// [Job][google.cloud.talent.v4beta1.Job] are updated. Otherwise all the fields are updated.
    ///
    /// A field mask to restrict the fields that are updated. Only
    /// top level fields of [Job][google.cloud.talent.v4beta1.Job] are supported.
    ///
    /// If [update_mask][google.cloud.talent.v4beta1.BatchUpdateJobsRequest.update_mask] is provided, The [Job][google.cloud.talent.v4beta1.Job] inside
    /// [JobResult][google.cloud.talent.v4beta1.JobOperationResult.JobResult]
    /// will only contains fields that is updated, plus the Id of the Job.
    /// Otherwise,  [Job][google.cloud.talent.v4beta1.Job] will include all fields, which can yield a very
    /// large response.
    #[prost(message, optional, tag = "3")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
}
/// The result of [JobService.BatchCreateJobs][google.cloud.talent.v4beta1.JobService.BatchCreateJobs] or
/// [JobService.BatchUpdateJobs][google.cloud.talent.v4beta1.JobService.BatchUpdateJobs] APIs. It's used to
/// replace [google.longrunning.Operation.response][google.longrunning.Operation.response] in case of success.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct JobOperationResult {
    /// List of job mutation results from a batch mutate operation. It can change
    /// until operation status is FINISHED, FAILED or CANCELLED.
    #[prost(message, repeated, tag = "1")]
    pub job_results: ::std::vec::Vec<job_operation_result::JobResult>,
}
pub mod job_operation_result {
    /// Mutation result of a job.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct JobResult {
        /// Here [Job][google.cloud.talent.v4beta1.Job] only contains basic information including [name][google.cloud.talent.v4beta1.Job.name],
        /// [company][google.cloud.talent.v4beta1.Job.company], [language_code][google.cloud.talent.v4beta1.Job.language_code]
        /// and [requisition_id][google.cloud.talent.v4beta1.Job.requisition_id], use getJob method to retrieve
        /// detailed information of the created/updated job.
        #[prost(message, optional, tag = "1")]
        pub job: ::std::option::Option<super::Job>,
        /// The status of the job processed. This field is populated if the
        /// processing of the [job][google.cloud.talent.v4beta1.JobOperationResult.JobResult.job] fails.
        #[prost(message, optional, tag = "2")]
        pub status: ::std::option::Option<super::super::super::super::rpc::Status>,
    }
}
/// An enum that specifies the job attributes that are returned in the
/// [MatchingJob.job][google.cloud.talent.v4beta1.SearchJobsResponse.MatchingJob.job] or
/// [ListJobsResponse.jobs][google.cloud.talent.v4beta1.ListJobsResponse.jobs] fields.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum JobView {
    /// Default value.
    Unspecified = 0,
    /// A ID only view of job, with following attributes:
    /// [Job.name][google.cloud.talent.v4beta1.Job.name], [Job.requisition_id][google.cloud.talent.v4beta1.Job.requisition_id], [Job.language_code][google.cloud.talent.v4beta1.Job.language_code].
    IdOnly = 1,
    /// A minimal view of the job, with the following attributes:
    /// [Job.name][google.cloud.talent.v4beta1.Job.name], [Job.requisition_id][google.cloud.talent.v4beta1.Job.requisition_id], [Job.title][google.cloud.talent.v4beta1.Job.title],
    /// [Job.company][google.cloud.talent.v4beta1.Job.company], [Job.DerivedInfo.locations][google.cloud.talent.v4beta1.Job.DerivedInfo.locations], [Job.language_code][google.cloud.talent.v4beta1.Job.language_code].
    Minimal = 2,
    /// A small view of the job, with the following attributes in the search
    /// results: [Job.name][google.cloud.talent.v4beta1.Job.name], [Job.requisition_id][google.cloud.talent.v4beta1.Job.requisition_id], [Job.title][google.cloud.talent.v4beta1.Job.title],
    /// [Job.company][google.cloud.talent.v4beta1.Job.company], [Job.DerivedInfo.locations][google.cloud.talent.v4beta1.Job.DerivedInfo.locations], [Job.visibility][google.cloud.talent.v4beta1.Job.visibility],
    /// [Job.language_code][google.cloud.talent.v4beta1.Job.language_code], [Job.description][google.cloud.talent.v4beta1.Job.description].
    Small = 3,
    /// All available attributes are included in the search results.
    Full = 4,
}
#[doc = r" Generated client implementations."]
pub mod job_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " A service handles job management, including job CRUD, enumeration and search."]
    pub struct JobServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl JobServiceClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> JobServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + HttpBody + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as HttpBody>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = tonic::client::Grpc::with_interceptor(inner, interceptor);
            Self { inner }
        }
        #[doc = " Creates a new job."]
        #[doc = ""]
        #[doc = " Typically, the job becomes searchable within 10 seconds, but it may take"]
        #[doc = " up to 5 minutes."]
        pub async fn create_job(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateJobRequest>,
        ) -> Result<tonic::Response<super::Job>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.talent.v4beta1.JobService/CreateJob",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Begins executing a batch create jobs operation."]
        pub async fn batch_create_jobs(
            &mut self,
            request: impl tonic::IntoRequest<super::BatchCreateJobsRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.talent.v4beta1.JobService/BatchCreateJobs",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Retrieves the specified job, whose status is OPEN or recently EXPIRED"]
        #[doc = " within the last 90 days."]
        pub async fn get_job(
            &mut self,
            request: impl tonic::IntoRequest<super::GetJobRequest>,
        ) -> Result<tonic::Response<super::Job>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.talent.v4beta1.JobService/GetJob",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates specified job."]
        #[doc = ""]
        #[doc = " Typically, updated contents become visible in search results within 10"]
        #[doc = " seconds, but it may take up to 5 minutes."]
        pub async fn update_job(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateJobRequest>,
        ) -> Result<tonic::Response<super::Job>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.talent.v4beta1.JobService/UpdateJob",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Begins executing a batch update jobs operation."]
        pub async fn batch_update_jobs(
            &mut self,
            request: impl tonic::IntoRequest<super::BatchUpdateJobsRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.talent.v4beta1.JobService/BatchUpdateJobs",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes the specified job."]
        #[doc = ""]
        #[doc = " Typically, the job becomes unsearchable within 10 seconds, but it may take"]
        #[doc = " up to 5 minutes."]
        pub async fn delete_job(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteJobRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.talent.v4beta1.JobService/DeleteJob",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes a list of [Job][google.cloud.talent.v4beta1.Job]s by filter."]
        pub async fn batch_delete_jobs(
            &mut self,
            request: impl tonic::IntoRequest<super::BatchDeleteJobsRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.talent.v4beta1.JobService/BatchDeleteJobs",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists jobs by filter."]
        pub async fn list_jobs(
            &mut self,
            request: impl tonic::IntoRequest<super::ListJobsRequest>,
        ) -> Result<tonic::Response<super::ListJobsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.talent.v4beta1.JobService/ListJobs",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Searches for jobs using the provided [SearchJobsRequest][google.cloud.talent.v4beta1.SearchJobsRequest]."]
        #[doc = ""]
        #[doc = " This call constrains the [visibility][google.cloud.talent.v4beta1.Job.visibility] of jobs"]
        #[doc = " present in the database, and only returns jobs that the caller has"]
        #[doc = " permission to search against."]
        pub async fn search_jobs(
            &mut self,
            request: impl tonic::IntoRequest<super::SearchJobsRequest>,
        ) -> Result<tonic::Response<super::SearchJobsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.talent.v4beta1.JobService/SearchJobs",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Searches for jobs using the provided [SearchJobsRequest][google.cloud.talent.v4beta1.SearchJobsRequest]."]
        #[doc = ""]
        #[doc = " This API call is intended for the use case of targeting passive job"]
        #[doc = " seekers (for example, job seekers who have signed up to receive email"]
        #[doc = " alerts about potential job opportunities), and has different algorithmic"]
        #[doc = " adjustments that are targeted to passive job seekers."]
        #[doc = ""]
        #[doc = " This call constrains the [visibility][google.cloud.talent.v4beta1.Job.visibility] of jobs"]
        #[doc = " present in the database, and only returns jobs the caller has"]
        #[doc = " permission to search against."]
        pub async fn search_jobs_for_alert(
            &mut self,
            request: impl tonic::IntoRequest<super::SearchJobsRequest>,
        ) -> Result<tonic::Response<super::SearchJobsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.talent.v4beta1.JobService/SearchJobsForAlert",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for JobServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for JobServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "JobServiceClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod job_service_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with JobServiceServer."]
    #[async_trait]
    pub trait JobService: Send + Sync + 'static {
        #[doc = " Creates a new job."]
        #[doc = ""]
        #[doc = " Typically, the job becomes searchable within 10 seconds, but it may take"]
        #[doc = " up to 5 minutes."]
        async fn create_job(
            &self,
            request: tonic::Request<super::CreateJobRequest>,
        ) -> Result<tonic::Response<super::Job>, tonic::Status>;
        #[doc = " Begins executing a batch create jobs operation."]
        async fn batch_create_jobs(
            &self,
            request: tonic::Request<super::BatchCreateJobsRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        >;
        #[doc = " Retrieves the specified job, whose status is OPEN or recently EXPIRED"]
        #[doc = " within the last 90 days."]
        async fn get_job(
            &self,
            request: tonic::Request<super::GetJobRequest>,
        ) -> Result<tonic::Response<super::Job>, tonic::Status>;
        #[doc = " Updates specified job."]
        #[doc = ""]
        #[doc = " Typically, updated contents become visible in search results within 10"]
        #[doc = " seconds, but it may take up to 5 minutes."]
        async fn update_job(
            &self,
            request: tonic::Request<super::UpdateJobRequest>,
        ) -> Result<tonic::Response<super::Job>, tonic::Status>;
        #[doc = " Begins executing a batch update jobs operation."]
        async fn batch_update_jobs(
            &self,
            request: tonic::Request<super::BatchUpdateJobsRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        >;
        #[doc = " Deletes the specified job."]
        #[doc = ""]
        #[doc = " Typically, the job becomes unsearchable within 10 seconds, but it may take"]
        #[doc = " up to 5 minutes."]
        async fn delete_job(
            &self,
            request: tonic::Request<super::DeleteJobRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status>;
        #[doc = " Deletes a list of [Job][google.cloud.talent.v4beta1.Job]s by filter."]
        async fn batch_delete_jobs(
            &self,
            request: tonic::Request<super::BatchDeleteJobsRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status>;
        #[doc = " Lists jobs by filter."]
        async fn list_jobs(
            &self,
            request: tonic::Request<super::ListJobsRequest>,
        ) -> Result<tonic::Response<super::ListJobsResponse>, tonic::Status>;
        #[doc = " Searches for jobs using the provided [SearchJobsRequest][google.cloud.talent.v4beta1.SearchJobsRequest]."]
        #[doc = ""]
        #[doc = " This call constrains the [visibility][google.cloud.talent.v4beta1.Job.visibility] of jobs"]
        #[doc = " present in the database, and only returns jobs that the caller has"]
        #[doc = " permission to search against."]
        async fn search_jobs(
            &self,
            request: tonic::Request<super::SearchJobsRequest>,
        ) -> Result<tonic::Response<super::SearchJobsResponse>, tonic::Status>;
        #[doc = " Searches for jobs using the provided [SearchJobsRequest][google.cloud.talent.v4beta1.SearchJobsRequest]."]
        #[doc = ""]
        #[doc = " This API call is intended for the use case of targeting passive job"]
        #[doc = " seekers (for example, job seekers who have signed up to receive email"]
        #[doc = " alerts about potential job opportunities), and has different algorithmic"]
        #[doc = " adjustments that are targeted to passive job seekers."]
        #[doc = ""]
        #[doc = " This call constrains the [visibility][google.cloud.talent.v4beta1.Job.visibility] of jobs"]
        #[doc = " present in the database, and only returns jobs the caller has"]
        #[doc = " permission to search against."]
        async fn search_jobs_for_alert(
            &self,
            request: tonic::Request<super::SearchJobsRequest>,
        ) -> Result<tonic::Response<super::SearchJobsResponse>, tonic::Status>;
    }
    #[doc = " A service handles job management, including job CRUD, enumeration and search."]
    #[derive(Debug)]
    #[doc(hidden)]
    pub struct JobServiceServer<T: JobService> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: JobService> JobServiceServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, None);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, Some(interceptor.into()));
            Self { inner }
        }
    }
    impl<T, B> Service<http::Request<B>> for JobServiceServer<T>
    where
        T: JobService,
        B: HttpBody + Send + Sync + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/google.cloud.talent.v4beta1.JobService/CreateJob" => {
                    #[allow(non_camel_case_types)]
                    struct CreateJobSvc<T: JobService>(pub Arc<T>);
                    impl<T: JobService> tonic::server::UnaryService<super::CreateJobRequest> for CreateJobSvc<T> {
                        type Response = super::Job;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateJobRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.create_job(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = CreateJobSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/google.cloud.talent.v4beta1.JobService/BatchCreateJobs" => {
                    #[allow(non_camel_case_types)]
                    struct BatchCreateJobsSvc<T: JobService>(pub Arc<T>);
                    impl<T: JobService> tonic::server::UnaryService<super::BatchCreateJobsRequest>
                        for BatchCreateJobsSvc<T>
                    {
                        type Response = super::super::super::super::longrunning::Operation;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BatchCreateJobsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.batch_create_jobs(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = BatchCreateJobsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/google.cloud.talent.v4beta1.JobService/GetJob" => {
                    #[allow(non_camel_case_types)]
                    struct GetJobSvc<T: JobService>(pub Arc<T>);
                    impl<T: JobService> tonic::server::UnaryService<super::GetJobRequest> for GetJobSvc<T> {
                        type Response = super::Job;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetJobRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.get_job(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetJobSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/google.cloud.talent.v4beta1.JobService/UpdateJob" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateJobSvc<T: JobService>(pub Arc<T>);
                    impl<T: JobService> tonic::server::UnaryService<super::UpdateJobRequest> for UpdateJobSvc<T> {
                        type Response = super::Job;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateJobRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.update_job(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = UpdateJobSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/google.cloud.talent.v4beta1.JobService/BatchUpdateJobs" => {
                    #[allow(non_camel_case_types)]
                    struct BatchUpdateJobsSvc<T: JobService>(pub Arc<T>);
                    impl<T: JobService> tonic::server::UnaryService<super::BatchUpdateJobsRequest>
                        for BatchUpdateJobsSvc<T>
                    {
                        type Response = super::super::super::super::longrunning::Operation;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BatchUpdateJobsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.batch_update_jobs(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = BatchUpdateJobsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/google.cloud.talent.v4beta1.JobService/DeleteJob" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteJobSvc<T: JobService>(pub Arc<T>);
                    impl<T: JobService> tonic::server::UnaryService<super::DeleteJobRequest> for DeleteJobSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteJobRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.delete_job(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = DeleteJobSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/google.cloud.talent.v4beta1.JobService/BatchDeleteJobs" => {
                    #[allow(non_camel_case_types)]
                    struct BatchDeleteJobsSvc<T: JobService>(pub Arc<T>);
                    impl<T: JobService> tonic::server::UnaryService<super::BatchDeleteJobsRequest>
                        for BatchDeleteJobsSvc<T>
                    {
                        type Response = ();
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BatchDeleteJobsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.batch_delete_jobs(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = BatchDeleteJobsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/google.cloud.talent.v4beta1.JobService/ListJobs" => {
                    #[allow(non_camel_case_types)]
                    struct ListJobsSvc<T: JobService>(pub Arc<T>);
                    impl<T: JobService> tonic::server::UnaryService<super::ListJobsRequest> for ListJobsSvc<T> {
                        type Response = super::ListJobsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListJobsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.list_jobs(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ListJobsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/google.cloud.talent.v4beta1.JobService/SearchJobs" => {
                    #[allow(non_camel_case_types)]
                    struct SearchJobsSvc<T: JobService>(pub Arc<T>);
                    impl<T: JobService> tonic::server::UnaryService<super::SearchJobsRequest> for SearchJobsSvc<T> {
                        type Response = super::SearchJobsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SearchJobsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.search_jobs(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = SearchJobsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/google.cloud.talent.v4beta1.JobService/SearchJobsForAlert" => {
                    #[allow(non_camel_case_types)]
                    struct SearchJobsForAlertSvc<T: JobService>(pub Arc<T>);
                    impl<T: JobService> tonic::server::UnaryService<super::SearchJobsRequest>
                        for SearchJobsForAlertSvc<T>
                    {
                        type Response = super::SearchJobsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SearchJobsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.search_jobs_for_alert(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = SearchJobsForAlertSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .body(tonic::body::BoxBody::empty())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: JobService> Clone for JobServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: JobService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: JobService> tonic::transport::NamedService for JobServiceServer<T> {
        const NAME: &'static str = "google.cloud.talent.v4beta1.JobService";
    }
} // Cloud Profile Discovery API definition

/// A resource that represents the profile for a job candidate (also referred to
/// as a "single-source profile").
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Profile {
    /// Required during profile update.
    ///
    /// Resource name assigned to a profile by the API.
    ///
    /// The format is
    /// "projects/{project_id}/tenants/{tenant_id}/profiles/{profile_id}",
    /// for example, "projects/foo/tenants/bar/profiles/baz".
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Profile's id in client system, if available. This value is unique for each
    /// profile inside a tenant. An error is thrown if another profile with the
    /// same external_id is created.
    ///
    /// The maximum number of bytes allowed is 100.
    #[prost(string, tag = "2")]
    pub external_id: std::string::String,
    /// The source description indicating where the profile is acquired.
    ///
    /// For example, if a candidate profile is acquired from a resume, the user can
    /// input "resume" here to indicate the source.
    ///
    /// The maximum number of bytes allowed is 100.
    #[prost(string, tag = "3")]
    pub source: std::string::String,
    /// The URI set by clients that links to this profile's client-side copy.
    ///
    /// The maximum number of bytes allowed is 4000.
    #[prost(string, tag = "4")]
    pub uri: std::string::String,
    /// The cluster id of the profile to associate with other profile(s) for the
    /// same candidate.
    ///
    /// This field should be generated by the customer. If a value is not provided,
    /// a random UUID is assigned to this field of the profile.
    ///
    /// This is used to link multiple profiles to the same candidate. For example,
    /// a client has a candidate with two profiles, where one was created recently
    /// and the other one was created 5 years ago. These two profiles may be very
    /// different. The clients can create the first profile and get a generated
    /// [group_id][google.cloud.talent.v4beta1.Profile.group_id], and assign it when the second profile is created,
    /// indicating these two profiles are referring to the same candidate.
    #[prost(string, tag = "5")]
    pub group_id: std::string::String,
    /// Indicates the hirable status of the candidate.
    #[prost(message, optional, tag = "6")]
    pub is_hirable: ::std::option::Option<bool>,
    /// The timestamp when the profile was first created at this source.
    #[prost(message, optional, tag = "7")]
    pub create_time: ::std::option::Option<::prost_types::Timestamp>,
    /// The timestamp when the profile was last updated at this source.
    #[prost(message, optional, tag = "8")]
    pub update_time: ::std::option::Option<::prost_types::Timestamp>,
    /// The timestamp when the profile was last updated as a result of a direct or
    /// indirect action by a candidate.
    ///
    /// These actions include:
    ///
    /// * Direct actions such as the candidate submitting a new resume as part of a
    /// job application to the agency, using a self-service tool such as a website
    /// to update their profile, and so on.
    /// * Indirect actions by the candidate such as uploading a resume to a job
    /// board that is collected by the agency through a feed, providing a resume to
    /// a recruiter who then uploads it into the ATS, and so on.
    /// * Updates made to the candidate's profile by the recruiter as a result of
    /// interacting with the candidate (for example adding a skill or work
    /// preference, and so on). Changes to [recruiting_notes][google.cloud.talent.v4beta1.Profile.recruiting_notes] are specifically
    /// excluded from this action type.
    ///
    /// Note: [candidate_update_time][google.cloud.talent.v4beta1.Profile.candidate_update_time] must be greater than or equal to
    /// [resume_update_time][google.cloud.talent.v4beta1.Profile.resume_update_time] or an error is thrown.
    #[prost(message, optional, tag = "67")]
    pub candidate_update_time: ::std::option::Option<::prost_types::Timestamp>,
    /// The timestamp when the candidate's resume was added or updated on the
    /// candidate's profile. Whether that resume was directly uploaded by a
    /// candidate, pulled from a 3rd party job board feed, added by a recruiter,
    /// and so on.
    ///
    /// If this field is updated, it's expected that [resume][google.cloud.talent.v4beta1.Profile.resume] is provided in
    /// the create or update calls.
    #[prost(message, optional, tag = "68")]
    pub resume_update_time: ::std::option::Option<::prost_types::Timestamp>,
    /// The resume representing this profile.
    #[prost(message, optional, tag = "53")]
    pub resume: ::std::option::Option<Resume>,
    /// The names of the candidate this profile references.
    ///
    /// Currently only one person name is supported.
    #[prost(message, repeated, tag = "11")]
    pub person_names: ::std::vec::Vec<PersonName>,
    /// The candidate's postal addresses. It's highly recommended to
    /// input this information as accurately as possible to help improve search
    /// quality. Here are some recommendations:
    ///
    /// * Provide [Address.usage][google.cloud.talent.v4beta1.Address.usage] if possible, especially if the address is
    /// PERSONAL. During a search only personal addresses are considered. If there
    /// is no such address, all addresses with unspecified usage are assumed to be
    /// personal.
    /// * Provide [Address.current][google.cloud.talent.v4beta1.Address.current] for the current address if possible. During
    /// a search, only current addresses are considered. If there is no such
    /// address, all addresses are assumed to be current.
    ///
    /// When displaying a candidate's addresses, it is sometimes desirable to limit
    /// the number of addresses shown. In these cases we recommend that you display
    /// the addresses in the following order of priority:
    /// 1. [Address.usage][google.cloud.talent.v4beta1.Address.usage] is PERSONAL and [Address.current][google.cloud.talent.v4beta1.Address.current] is true.
    /// 2. [Address.usage][google.cloud.talent.v4beta1.Address.usage] is PERSONAL and [Address.current][google.cloud.talent.v4beta1.Address.current] is false or not
    /// set.
    /// 3. [Address.usage][google.cloud.talent.v4beta1.Address.usage] is CONTACT_INFO_USAGE_UNSPECIFIED and
    /// [Address.current][google.cloud.talent.v4beta1.Address.current] is true.
    /// 4. [Address.usage][google.cloud.talent.v4beta1.Address.usage] is CONTACT_INFO_USAGE_UNSPECIFIED and
    /// [Address.current][google.cloud.talent.v4beta1.Address.current] is false or not set.
    #[prost(message, repeated, tag = "12")]
    pub addresses: ::std::vec::Vec<Address>,
    /// The candidate's email addresses.
    #[prost(message, repeated, tag = "13")]
    pub email_addresses: ::std::vec::Vec<Email>,
    /// The candidate's phone number(s).
    #[prost(message, repeated, tag = "14")]
    pub phone_numbers: ::std::vec::Vec<Phone>,
    /// The candidate's personal URIs.
    #[prost(message, repeated, tag = "15")]
    pub personal_uris: ::std::vec::Vec<PersonalUri>,
    /// Available contact information besides [addresses][google.cloud.talent.v4beta1.Profile.addresses], [email_addresses][google.cloud.talent.v4beta1.Profile.email_addresses],
    /// [phone_numbers][google.cloud.talent.v4beta1.Profile.phone_numbers] and [personal_uris][google.cloud.talent.v4beta1.Profile.personal_uris]. For example, Hang-out, Skype.
    #[prost(message, repeated, tag = "16")]
    pub additional_contact_info: ::std::vec::Vec<AdditionalContactInfo>,
    /// The employment history records of the candidate. It's highly recommended
    /// to input this information as accurately as possible to help improve search
    /// quality. Here are some recommendations:
    ///
    /// * Specify the start and end dates of the employment records.
    /// * List different employment types separately, no matter how minor the
    /// change is.
    /// For example, only job title is changed from "software engineer" to "senior
    /// software engineer".
    /// * Provide [EmploymentRecord.is_current][google.cloud.talent.v4beta1.EmploymentRecord.is_current] for the current employment if
    /// possible. If not, it's inferred from user inputs.
    ///
    /// The limitation for max number of employment records is 100.
    #[prost(message, repeated, tag = "17")]
    pub employment_records: ::std::vec::Vec<EmploymentRecord>,
    /// The education history record of the candidate. It's highly recommended to
    /// input this information as accurately as possible to help improve search
    /// quality. Here are some recommendations:
    ///
    /// * Specify the start and end dates of the education records.
    /// * List each education type separately, no matter how minor the change is.
    /// For example, the profile contains the education experience from the same
    /// school but different degrees.
    /// * Provide [EducationRecord.is_current][google.cloud.talent.v4beta1.EducationRecord.is_current] for the current education if
    /// possible. If not, it's inferred from user inputs.
    ///
    /// The limitation for max number of education records is 100.
    #[prost(message, repeated, tag = "18")]
    pub education_records: ::std::vec::Vec<EducationRecord>,
    /// The skill set of the candidate. It's highly recommended to provide as
    /// much information as possible to help improve the search quality.
    ///
    /// The limitation for max number of skills is 500.
    #[prost(message, repeated, tag = "19")]
    pub skills: ::std::vec::Vec<Skill>,
    /// The individual or collaborative activities which the candidate has
    /// participated in, for example, open-source projects, class assignments that
    /// aren't listed in [employment_records][google.cloud.talent.v4beta1.Profile.employment_records].
    ///
    /// The limitation for max number of activities is 50.
    #[prost(message, repeated, tag = "20")]
    pub activities: ::std::vec::Vec<Activity>,
    /// The publications published by the candidate.
    ///
    /// The limitation for max number of publications is 50.
    #[prost(message, repeated, tag = "21")]
    pub publications: ::std::vec::Vec<Publication>,
    /// The patents acquired by the candidate.
    #[prost(message, repeated, tag = "22")]
    pub patents: ::std::vec::Vec<Patent>,
    /// The certifications acquired by the candidate.
    #[prost(message, repeated, tag = "23")]
    pub certifications: ::std::vec::Vec<Certification>,
    /// Output only. The resource names of the candidate's applications.
    #[prost(string, repeated, tag = "47")]
    pub applications: ::std::vec::Vec<std::string::String>,
    /// Output only. The resource names of the candidate's assignments.
    #[prost(string, repeated, tag = "48")]
    pub assignments: ::std::vec::Vec<std::string::String>,
    /// A map of fields to hold both filterable and non-filterable custom profile
    /// attributes that aren't covered by the provided structured fields. See
    /// [CustomAttribute][google.cloud.talent.v4beta1.CustomAttribute] for more details.
    ///
    /// At most 100 filterable and at most 100 unfilterable keys are supported. If
    /// limit is exceeded, an error is thrown. Custom attributes are `unfilterable`
    /// by default. These are filterable when the `filterable` flag is set to
    /// `true`.
    ///
    /// Numeric custom attributes: each key can only map to one numeric value,
    /// otherwise an error is thrown. Client can also filter on numeric custom
    /// attributes using '>', '<' or '=' operators.
    ///
    /// String custom attributes: each key can map up to 50 string values. For
    /// filterable string value, each value has a byte size of no more than 256B.
    /// For unfilterable string values, the maximum byte size of a single key is
    /// 64B. An error is thrown for any request exceeding the limit.
    /// The maximum total byte size is 10KB.
    #[prost(map = "string, message", tag = "26")]
    pub custom_attributes: ::std::collections::HashMap<std::string::String, CustomAttribute>,
    /// Output only. Indicates if a summarized profile was created as part of the
    /// profile creation API call. This flag does not indicate whether a profile is
    /// searchable or not.
    #[prost(bool, tag = "27")]
    pub processed: bool,
    /// Output only. Keyword snippet shows how the search result is related to a
    /// search query.  This is only returned in [SearchProfilesResponse][google.cloud.talent.v4beta1.SearchProfilesResponse].
    #[prost(string, tag = "28")]
    pub keyword_snippet: std::string::String,
    /// Output only. Candidate's availability signals.
    #[prost(message, repeated, tag = "70")]
    pub availability_signals: ::std::vec::Vec<AvailabilitySignal>,
    /// Output only. Derived locations of the profile, resolved from [Profile.addresses][google.cloud.talent.v4beta1.Profile.addresses].
    ///
    /// [derived_addresses][google.cloud.talent.v4beta1.Profile.derived_addresses] are exactly matched to [Profile.addresses][google.cloud.talent.v4beta1.Profile.addresses] in the
    /// same order.
    #[prost(message, repeated, tag = "64")]
    pub derived_addresses: ::std::vec::Vec<Location>,
}
/// Candidate availability signal.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AvailabilitySignal {
    /// Type of signal.
    #[prost(enumeration = "AvailabilitySignalType", tag = "1")]
    pub r#type: i32,
    /// Timestamp of when the given availability activity last happened.
    #[prost(message, optional, tag = "2")]
    pub last_update_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Indicates if the [last_update_time][google.cloud.talent.v4beta1.AvailabilitySignal.last_update_time] is within
    /// [AvailabilityFilter.range][google.cloud.talent.v4beta1.AvailabilityFilter.range].
    ///
    /// Returned only in a search response when there is an [AvailabilityFilter][google.cloud.talent.v4beta1.AvailabilityFilter]
    /// in [ProfileQuery.availability_filters][google.cloud.talent.v4beta1.ProfileQuery.availability_filters] where
    /// [signal_type][google.cloud.talent.v4beta1.AvailabilityFilter.signal_type] matches [type][google.cloud.talent.v4beta1.AvailabilitySignal.type].
    #[prost(message, optional, tag = "3")]
    pub filter_satisfied: ::std::option::Option<bool>,
}
/// Resource that represents a resume.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Resume {
    /// Users can create a profile with only this field field, if [resume_type][google.cloud.talent.v4beta1.Resume.resume_type]
    /// is [HRXML][google.cloud.talent.v4beta1.Resume.ResumeType.HRXML]. For example, the API parses this field and
    /// creates a profile
    /// with all structured fields populated. [EmploymentRecord][google.cloud.talent.v4beta1.EmploymentRecord],
    /// [EducationRecord][google.cloud.talent.v4beta1.EducationRecord], and so on. An error is thrown if this field cannot be
    /// parsed.
    ///
    /// Note that the use of the functionality offered by this field to extract
    /// data from resumes is an Alpha feature and as such is not covered by any
    /// SLA.
    #[prost(string, tag = "1")]
    pub structured_resume: std::string::String,
    /// The format of [structured_resume][google.cloud.talent.v4beta1.Resume.structured_resume].
    #[prost(enumeration = "resume::ResumeType", tag = "2")]
    pub resume_type: i32,
}
pub mod resume {
    /// The format of a structured resume.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ResumeType {
        /// Default value.
        Unspecified = 0,
        /// The profile contents in HR-XML format.
        /// See https://schemas.liquid-technologies.com/hr-xml/2007-04-15/ for more
        /// information about Human Resources XML.
        Hrxml = 1,
        /// Resume type not specified.
        OtherResumeType = 2,
    }
}
/// Resource that represents the name of a person.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PersonName {
    /// Preferred name for the person. This field is ignored if [structured_name][google.cloud.talent.v4beta1.PersonName.structured_name]
    /// is provided.
    ///
    /// Number of characters allowed is 100.
    #[prost(string, tag = "3")]
    pub preferred_name: std::string::String,
    /// The name of a person. It can be one of
    /// [formatted_name][google.cloud.talent.v4beta1.PersonName.formatted_name] or
    /// [structured_name][google.cloud.talent.v4beta1.PersonName.structured_name].
    #[prost(oneof = "person_name::PersonName", tags = "1, 2")]
    pub person_name: ::std::option::Option<person_name::PersonName>,
}
pub mod person_name {
    /// Resource that represents a person's structured name.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PersonStructuredName {
        /// Given/first name.
        ///
        /// It's derived from [formatted_name][google.cloud.talent.v4beta1.PersonName.formatted_name] if not provided.
        ///
        /// Number of characters allowed is 100.
        #[prost(string, tag = "1")]
        pub given_name: std::string::String,
        /// Preferred given/first name or nickname.
        ///
        /// Number of characters allowed is 100.
        #[prost(string, tag = "6")]
        pub preferred_name: std::string::String,
        /// Middle initial.
        ///
        /// It's derived from [formatted_name][google.cloud.talent.v4beta1.PersonName.formatted_name] if not provided.
        ///
        /// Number of characters allowed is 20.
        #[prost(string, tag = "2")]
        pub middle_initial: std::string::String,
        /// Family/last name.
        ///
        /// It's derived from [formatted_name][google.cloud.talent.v4beta1.PersonName.formatted_name] if not provided.
        ///
        /// Number of characters allowed is 100.
        #[prost(string, tag = "3")]
        pub family_name: std::string::String,
        /// Suffixes.
        ///
        /// Number of characters allowed is 20.
        #[prost(string, repeated, tag = "4")]
        pub suffixes: ::std::vec::Vec<std::string::String>,
        /// Prefixes.
        ///
        /// Number of characters allowed is 20.
        #[prost(string, repeated, tag = "5")]
        pub prefixes: ::std::vec::Vec<std::string::String>,
    }
    /// The name of a person. It can be one of
    /// [formatted_name][google.cloud.talent.v4beta1.PersonName.formatted_name] or
    /// [structured_name][google.cloud.talent.v4beta1.PersonName.structured_name].
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum PersonName {
        /// A string represents a person's full name. For example, "Dr. John Smith".
        ///
        /// Number of characters allowed is 100.
        #[prost(string, tag = "1")]
        FormattedName(std::string::String),
        /// A person's name in a structured way (last name, first name, suffix, and
        /// so on.)
        #[prost(message, tag = "2")]
        StructuredName(PersonStructuredName),
    }
}
/// Resource that represents a address.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Address {
    /// The usage of the address. For example, SCHOOL, WORK, PERSONAL.
    #[prost(enumeration = "ContactInfoUsage", tag = "1")]
    pub usage: i32,
    /// Indicates if it's the person's current address.
    #[prost(message, optional, tag = "4")]
    pub current: ::std::option::Option<bool>,
    /// The address of a person. It can be one of
    /// [unstructured_address][google.cloud.talent.v4beta1.Address.unstructured_address] or
    /// [structured_address][google.cloud.talent.v4beta1.Address.structured_address].
    #[prost(oneof = "address::Address", tags = "2, 3")]
    pub address: ::std::option::Option<address::Address>,
}
pub mod address {
    /// The address of a person. It can be one of
    /// [unstructured_address][google.cloud.talent.v4beta1.Address.unstructured_address] or
    /// [structured_address][google.cloud.talent.v4beta1.Address.structured_address].
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Address {
        /// Unstructured address.
        ///
        /// For example, "1600 Amphitheatre Pkwy, Mountain View, CA 94043",
        /// "Sunnyvale, California".
        ///
        /// Number of characters allowed is 100.
        #[prost(string, tag = "2")]
        UnstructuredAddress(std::string::String),
        /// Structured address that contains street address, city, state, country,
        /// and so on.
        #[prost(message, tag = "3")]
        StructuredAddress(super::super::super::super::r#type::PostalAddress),
    }
}
/// Resource that represents a person's email address.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Email {
    /// The usage of the email address. For example, SCHOOL, WORK, PERSONAL.
    #[prost(enumeration = "ContactInfoUsage", tag = "1")]
    pub usage: i32,
    /// Email address.
    ///
    /// Number of characters allowed is 4,000.
    #[prost(string, tag = "2")]
    pub email_address: std::string::String,
}
/// Resource that represents a person's telephone number.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Phone {
    /// The usage of the phone. For example, SCHOOL, WORK, PERSONAL.
    #[prost(enumeration = "ContactInfoUsage", tag = "1")]
    pub usage: i32,
    /// The phone type. For example, LANDLINE, MOBILE, FAX.
    #[prost(enumeration = "phone::PhoneType", tag = "2")]
    pub r#type: i32,
    /// Phone number.
    ///
    /// Any phone formats are supported and only exact matches are performed on
    /// searches. For example, if a phone number in profile is provided in the
    /// format of "(xxx)xxx-xxxx", in profile searches the same phone format
    /// has to be provided.
    ///
    /// Number of characters allowed is 20.
    #[prost(string, tag = "3")]
    pub number: std::string::String,
    /// When this number is available. Any descriptive string is expected.
    ///
    /// Number of characters allowed is 100.
    #[prost(string, tag = "4")]
    pub when_available: std::string::String,
}
pub mod phone {
    /// Enum that represents the type of the telephone.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum PhoneType {
        /// Default value.
        Unspecified = 0,
        /// A landline.
        Landline = 1,
        /// A mobile.
        Mobile = 2,
        /// A fax.
        Fax = 3,
        /// A pager.
        Pager = 4,
        /// A TTY (test telephone) or TDD (telecommunication device for the deaf).
        TtyOrTdd = 5,
        /// A voicemail.
        Voicemail = 6,
        /// A virtual telephone number is a number that can be routed to another
        /// number and managed by the user via Web, SMS, IVR, and so on.  It is
        /// associated with a particular person, and may be routed to either a MOBILE
        /// or LANDLINE number. The [phone usage][google.cloud.talent.v4beta1.ContactInfoUsage] should
        /// be set to PERSONAL for these phone types. Some more information can be
        /// found here: https://en.wikipedia.org/wiki/Personal_Numbers
        Virtual = 7,
        /// Voice over IP numbers. This includes TSoIP (Telephony Service over IP).
        Voip = 8,
        /// In some regions (e.g. the USA), it is impossible to distinguish between
        /// fixed-line and mobile numbers by looking at the phone number itself.
        MobileOrLandline = 9,
    }
}
/// Resource that represents a valid URI for a personal use.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PersonalUri {
    /// The personal URI.
    ///
    /// Number of characters allowed is 4,000.
    #[prost(string, tag = "1")]
    pub uri: std::string::String,
}
/// Resource that represents contact information other than phone, email,
/// URI and addresses.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdditionalContactInfo {
    /// The usage of this contact method. For example, SCHOOL, WORK, PERSONAL.
    #[prost(enumeration = "ContactInfoUsage", tag = "1")]
    pub usage: i32,
    /// The name of the contact method.
    ///
    /// For example, "hangout", "skype".
    ///
    /// Number of characters allowed is 100.
    #[prost(string, tag = "2")]
    pub name: std::string::String,
    /// The contact id.
    ///
    /// Number of characters allowed is 100.
    #[prost(string, tag = "3")]
    pub contact_id: std::string::String,
}
/// Resource that represents an employment record of a candidate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EmploymentRecord {
    /// Start date of the employment.
    #[prost(message, optional, tag = "1")]
    pub start_date: ::std::option::Option<super::super::super::r#type::Date>,
    /// End date of the employment.
    #[prost(message, optional, tag = "2")]
    pub end_date: ::std::option::Option<super::super::super::r#type::Date>,
    /// The name of the employer company/organization.
    ///
    /// For example, "Google", "Alphabet", and so on.
    ///
    /// Number of characters allowed is 250.
    #[prost(string, tag = "3")]
    pub employer_name: std::string::String,
    /// The division name of the employment.
    ///
    /// For example, division, department, client, and so on.
    ///
    /// Number of characters allowed is 100.
    #[prost(string, tag = "4")]
    pub division_name: std::string::String,
    /// The physical address of the employer.
    #[prost(message, optional, tag = "5")]
    pub address: ::std::option::Option<Address>,
    /// The job title of the employment.
    ///
    /// For example, "Software Engineer", "Data Scientist", and so on.
    ///
    /// Number of characters allowed is 250.
    #[prost(string, tag = "6")]
    pub job_title: std::string::String,
    /// The description of job content.
    ///
    /// Number of characters allowed is 100,000.
    #[prost(string, tag = "7")]
    pub job_description: std::string::String,
    /// If the jobs is a supervisor position.
    #[prost(message, optional, tag = "8")]
    pub is_supervisor: ::std::option::Option<bool>,
    /// If this employment is self-employed.
    #[prost(message, optional, tag = "9")]
    pub is_self_employed: ::std::option::Option<bool>,
    /// If this employment is current.
    #[prost(message, optional, tag = "10")]
    pub is_current: ::std::option::Option<bool>,
    /// Output only. The job title snippet shows how the [job_title][google.cloud.talent.v4beta1.EmploymentRecord.job_title] is related
    /// to a search query. It's empty if the [job_title][google.cloud.talent.v4beta1.EmploymentRecord.job_title] isn't related to the
    /// search query.
    #[prost(string, tag = "11")]
    pub job_title_snippet: std::string::String,
    /// Output only. The job description snippet shows how the [job_description][google.cloud.talent.v4beta1.EmploymentRecord.job_description]
    /// is related to a search query. It's empty if the [job_description][google.cloud.talent.v4beta1.EmploymentRecord.job_description] isn't
    /// related to the search query.
    #[prost(string, tag = "12")]
    pub job_description_snippet: std::string::String,
    /// Output only. The employer name snippet shows how the [employer_name][google.cloud.talent.v4beta1.EmploymentRecord.employer_name] is
    /// related to a search query. It's empty if the [employer_name][google.cloud.talent.v4beta1.EmploymentRecord.employer_name] isn't
    /// related to the search query.
    #[prost(string, tag = "13")]
    pub employer_name_snippet: std::string::String,
}
/// Resource that represents an education record of a candidate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EducationRecord {
    /// The start date of the education.
    #[prost(message, optional, tag = "1")]
    pub start_date: ::std::option::Option<super::super::super::r#type::Date>,
    /// The end date of the education.
    #[prost(message, optional, tag = "2")]
    pub end_date: ::std::option::Option<super::super::super::r#type::Date>,
    /// The expected graduation date if currently pursuing a degree.
    #[prost(message, optional, tag = "3")]
    pub expected_graduation_date: ::std::option::Option<super::super::super::r#type::Date>,
    /// The name of the school or institution.
    ///
    /// For example, "Stanford University", "UC Berkeley", and so on.
    ///
    /// Number of characters allowed is 250.
    #[prost(string, tag = "4")]
    pub school_name: std::string::String,
    /// The physical address of the education institution.
    #[prost(message, optional, tag = "5")]
    pub address: ::std::option::Option<Address>,
    /// The description of the education.
    ///
    /// Number of characters allowed is 100,000.
    #[prost(string, tag = "8")]
    pub description: std::string::String,
    /// If this education is current.
    #[prost(message, optional, tag = "9")]
    pub is_current: ::std::option::Option<bool>,
    /// Output only. The school name snippet shows how the [school_name][google.cloud.talent.v4beta1.EducationRecord.school_name] is related to a
    /// search query in search result. It's empty if the [school_name][google.cloud.talent.v4beta1.EducationRecord.school_name] isn't
    /// related to the search query.
    #[prost(string, tag = "10")]
    pub school_name_snippet: std::string::String,
    /// Output only. The job description snippet shows how the [Degree][google.cloud.talent.v4beta1.Degree] is related to a search
    /// query in search result. It's empty if the [Degree][google.cloud.talent.v4beta1.Degree] isn't related to the
    /// search query.
    #[prost(string, tag = "11")]
    pub degree_snippet: std::string::String,
    /// The degree information. It can be one of
    /// [degree_description][google.cloud.talent.v4beta1.EducationRecord.degree_description] or
    /// [structured_degree][google.cloud.talent.v4beta1.EducationRecord.structured_degree].
    #[prost(oneof = "education_record::Degree", tags = "6, 7")]
    pub degree: ::std::option::Option<education_record::Degree>,
}
pub mod education_record {
    /// The degree information. It can be one of
    /// [degree_description][google.cloud.talent.v4beta1.EducationRecord.degree_description] or
    /// [structured_degree][google.cloud.talent.v4beta1.EducationRecord.structured_degree].
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Degree {
        /// The full description of the degree.
        ///
        /// For example, "Master of Science in Computer Science", "B.S in Math".
        ///
        /// Number of characters allowed is 100.
        #[prost(string, tag = "6")]
        DegreeDescription(std::string::String),
        /// The structured notation of the degree.
        #[prost(message, tag = "7")]
        StructuredDegree(super::Degree),
    }
}
/// Resource that represents a degree pursuing or acquired by a candidate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Degree {
    /// ISCED degree type.
    #[prost(enumeration = "DegreeType", tag = "1")]
    pub degree_type: i32,
    /// Full Degree name.
    ///
    /// For example, "B.S.", "Master of Arts", and so on.
    ///
    /// Number of characters allowed is 100.
    #[prost(string, tag = "2")]
    pub degree_name: std::string::String,
    /// Fields of study for the degree.
    ///
    /// For example, "Computer science", "engineering".
    ///
    /// Number of characters allowed is 100.
    #[prost(string, repeated, tag = "3")]
    pub fields_of_study: ::std::vec::Vec<std::string::String>,
}
/// Resource that represents an individual or collaborative activity participated
/// in by a candidate, for example, an open-source project, a class assignment,
/// and so on.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Activity {
    /// Activity display name.
    ///
    /// Number of characters allowed is 100.
    #[prost(string, tag = "1")]
    pub display_name: std::string::String,
    /// Activity description.
    ///
    /// Number of characters allowed is 100,000.
    #[prost(string, tag = "2")]
    pub description: std::string::String,
    /// Activity URI.
    ///
    /// Number of characters allowed is 4,000.
    #[prost(string, tag = "3")]
    pub uri: std::string::String,
    /// The first creation date of the activity.
    #[prost(message, optional, tag = "4")]
    pub create_date: ::std::option::Option<super::super::super::r#type::Date>,
    /// The last update date of the activity.
    #[prost(message, optional, tag = "5")]
    pub update_date: ::std::option::Option<super::super::super::r#type::Date>,
    /// A list of team members involved in this activity.
    ///
    /// Number of characters allowed is 100.
    ///
    /// The limitation for max number of team members is 50.
    #[prost(string, repeated, tag = "6")]
    pub team_members: ::std::vec::Vec<std::string::String>,
    /// A list of skills used in this activity.
    ///
    /// The limitation for max number of skills used is 50.
    #[prost(message, repeated, tag = "7")]
    pub skills_used: ::std::vec::Vec<Skill>,
    /// Output only. Activity name snippet shows how the [display_name][google.cloud.talent.v4beta1.Activity.display_name] is related to a search
    /// query. It's empty if the [display_name][google.cloud.talent.v4beta1.Activity.display_name] isn't related to the search
    /// query.
    #[prost(string, tag = "8")]
    pub activity_name_snippet: std::string::String,
    /// Output only. Activity description snippet shows how the
    /// [description][google.cloud.talent.v4beta1.Activity.description] is related to a search query. It's empty if the
    /// [description][google.cloud.talent.v4beta1.Activity.description] isn't related to the search query.
    #[prost(string, tag = "9")]
    pub activity_description_snippet: std::string::String,
    /// Output only. Skill used snippet shows how the corresponding
    /// [skills_used][google.cloud.talent.v4beta1.Activity.skills_used] are related to a search query. It's empty if the
    /// corresponding [skills_used][google.cloud.talent.v4beta1.Activity.skills_used] are not related to the search query.
    #[prost(string, repeated, tag = "10")]
    pub skills_used_snippet: ::std::vec::Vec<std::string::String>,
}
/// Resource that represents a publication resource of a candidate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Publication {
    /// A list of author names.
    ///
    /// Number of characters allowed is 100.
    #[prost(string, repeated, tag = "1")]
    pub authors: ::std::vec::Vec<std::string::String>,
    /// The title of the publication.
    ///
    /// Number of characters allowed is 100.
    #[prost(string, tag = "2")]
    pub title: std::string::String,
    /// The description of the publication.
    ///
    /// Number of characters allowed is 100,000.
    #[prost(string, tag = "3")]
    pub description: std::string::String,
    /// The journal name of the publication.
    ///
    /// Number of characters allowed is 100.
    #[prost(string, tag = "4")]
    pub journal: std::string::String,
    /// Volume number.
    ///
    /// Number of characters allowed is 100.
    #[prost(string, tag = "5")]
    pub volume: std::string::String,
    /// The publisher of the journal.
    ///
    /// Number of characters allowed is 100.
    #[prost(string, tag = "6")]
    pub publisher: std::string::String,
    /// The publication date.
    #[prost(message, optional, tag = "7")]
    pub publication_date: ::std::option::Option<super::super::super::r#type::Date>,
    /// The publication type.
    ///
    /// Number of characters allowed is 100.
    #[prost(string, tag = "8")]
    pub publication_type: std::string::String,
    /// ISBN number.
    ///
    /// Number of characters allowed is 100.
    #[prost(string, tag = "9")]
    pub isbn: std::string::String,
}
/// Resource that represents the patent acquired by a candidate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Patent {
    /// Name of the patent.
    ///
    /// Number of characters allowed is 100.
    #[prost(string, tag = "1")]
    pub display_name: std::string::String,
    /// A list of inventors' names.
    ///
    /// Number of characters allowed for each is 100.
    #[prost(string, repeated, tag = "2")]
    pub inventors: ::std::vec::Vec<std::string::String>,
    /// The status of the patent.
    ///
    /// Number of characters allowed is 100.
    #[prost(string, tag = "3")]
    pub patent_status: std::string::String,
    /// The date the last time the status of the patent was checked.
    #[prost(message, optional, tag = "4")]
    pub patent_status_date: ::std::option::Option<super::super::super::r#type::Date>,
    /// The date that the patent was filed.
    #[prost(message, optional, tag = "5")]
    pub patent_filing_date: ::std::option::Option<super::super::super::r#type::Date>,
    /// The name of the patent office.
    ///
    /// Number of characters allowed is 100.
    #[prost(string, tag = "6")]
    pub patent_office: std::string::String,
    /// The number of the patent.
    ///
    /// Number of characters allowed is 100.
    #[prost(string, tag = "7")]
    pub patent_number: std::string::String,
    /// The description of the patent.
    ///
    /// Number of characters allowed is 100,000.
    #[prost(string, tag = "8")]
    pub patent_description: std::string::String,
    /// The skills used in this patent.
    #[prost(message, repeated, tag = "9")]
    pub skills_used: ::std::vec::Vec<Skill>,
}
/// List profiles request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListProfilesRequest {
    /// Required. The resource name of the tenant under which the profile is created.
    ///
    /// The format is "projects/{project_id}/tenants/{tenant_id}". For example,
    /// "projects/foo/tenants/bar".
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// The filter string specifies the profiles to be enumerated.
    ///
    /// Supported operator: =, AND
    ///
    /// The field(s) eligible for filtering are:
    ///
    /// * `externalId`
    /// * `groupId`
    ///
    /// externalId and groupId cannot be specified at the same time. If both
    /// externalId and groupId are provided, the API will return a bad request
    /// error.
    ///
    /// Sample Query:
    ///
    /// * externalId = "externalId-1"
    /// * groupId = "groupId-1"
    #[prost(string, tag = "5")]
    pub filter: std::string::String,
    /// The token that specifies the current offset (that is, starting result).
    ///
    /// Please set the value to [ListProfilesResponse.next_page_token][google.cloud.talent.v4beta1.ListProfilesResponse.next_page_token] to
    /// continue the list.
    #[prost(string, tag = "2")]
    pub page_token: std::string::String,
    /// The maximum number of profiles to be returned, at most 100.
    ///
    /// Default is 100 unless a positive number smaller than 100 is specified.
    #[prost(int32, tag = "3")]
    pub page_size: i32,
    /// A field mask to specify the profile fields to be listed in response.
    /// All fields are listed if it is unset.
    ///
    /// Valid values are:
    ///
    /// * name
    #[prost(message, optional, tag = "4")]
    pub read_mask: ::std::option::Option<::prost_types::FieldMask>,
}
/// The List profiles response object.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListProfilesResponse {
    /// Profiles for the specific tenant.
    #[prost(message, repeated, tag = "1")]
    pub profiles: ::std::vec::Vec<Profile>,
    /// A token to retrieve the next page of results. This is empty if there are no
    /// more results.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
}
/// Create profile request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateProfileRequest {
    /// Required. The name of the tenant this profile belongs to.
    ///
    /// The format is "projects/{project_id}/tenants/{tenant_id}". For example,
    /// "projects/foo/tenants/bar".
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Required. The profile to be created.
    #[prost(message, optional, tag = "2")]
    pub profile: ::std::option::Option<Profile>,
}
/// Get profile request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetProfileRequest {
    /// Required. Resource name of the profile to get.
    ///
    /// The format is
    /// "projects/{project_id}/tenants/{tenant_id}/profiles/{profile_id}". For
    /// example, "projects/foo/tenants/bar/profiles/baz".
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Update profile request
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateProfileRequest {
    /// Required. Profile to be updated.
    #[prost(message, optional, tag = "1")]
    pub profile: ::std::option::Option<Profile>,
    /// A field mask to specify the profile fields to update.
    ///
    /// A full update is performed if it is unset.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
}
/// Delete profile request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteProfileRequest {
    /// Required. Resource name of the profile to be deleted.
    ///
    /// The format is
    /// "projects/{project_id}/tenants/{tenant_id}/profiles/{profile_id}". For
    /// example, "projects/foo/tenants/bar/profiles/baz".
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// The request body of the `SearchProfiles` call.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchProfilesRequest {
    /// Required. The resource name of the tenant to search within.
    ///
    /// The format is "projects/{project_id}/tenants/{tenant_id}". For example,
    /// "projects/foo/tenants/bar".
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Required. The meta information collected about the profile search user. This is used
    /// to improve the search quality of the service. These values are provided by
    /// users, and must be precise and consistent.
    #[prost(message, optional, tag = "2")]
    pub request_metadata: ::std::option::Option<RequestMetadata>,
    /// Search query to execute. See [ProfileQuery][google.cloud.talent.v4beta1.ProfileQuery] for more details.
    #[prost(message, optional, tag = "3")]
    pub profile_query: ::std::option::Option<ProfileQuery>,
    /// A limit on the number of profiles returned in the search results.
    /// A value above the default value 10 can increase search response time.
    ///
    /// The maximum value allowed is 100. Otherwise an error is thrown.
    #[prost(int32, tag = "4")]
    pub page_size: i32,
    /// The pageToken, similar to offset enables users of the API to paginate
    /// through the search results. To retrieve the first page of results, set the
    /// pageToken to empty. The search response includes a
    /// [nextPageToken][google.cloud.talent.v4beta1.SearchProfilesResponse.next_page_token] field that can be
    /// used to populate the pageToken field for the next page of results. Using
    /// pageToken instead of offset increases the performance of the API,
    /// especially compared to larger offset values.
    #[prost(string, tag = "5")]
    pub page_token: std::string::String,
    /// An integer that specifies the current offset (that is, starting result) in
    /// search results. This field is only considered if [page_token][google.cloud.talent.v4beta1.SearchProfilesRequest.page_token] is unset.
    ///
    /// The maximum allowed value is 5000. Otherwise an error is thrown.
    ///
    /// For example, 0 means to search from the first profile, and 10 means to
    /// search from the 11th profile. This can be used for pagination, for example
    /// pageSize = 10 and offset = 10 means to search from the second page.
    #[prost(int32, tag = "6")]
    pub offset: i32,
    /// This flag controls the spell-check feature. If `false`, the
    /// service attempts to correct a misspelled query.
    ///
    /// For example, "enginee" is corrected to "engineer".
    #[prost(bool, tag = "7")]
    pub disable_spell_check: bool,
    /// The criteria that determines how search results are sorted.
    /// Defaults is "relevance desc" if no value is specified.
    ///
    /// Supported options are:
    ///
    /// * "relevance desc": By descending relevance, as determined by the API
    ///    algorithms.
    /// * "update_date desc": Sort by [Profile.update_time][google.cloud.talent.v4beta1.Profile.update_time] in descending order
    ///   (recently updated profiles first).
    /// * "create_date desc": Sort by [Profile.create_time][google.cloud.talent.v4beta1.Profile.create_time] in descending order
    ///   (recently created profiles first).
    /// * "first_name": Sort by [PersonName.PersonStructuredName.given_name][google.cloud.talent.v4beta1.PersonName.PersonStructuredName.given_name] in
    ///   ascending order.
    /// * "first_name desc": Sort by [PersonName.PersonStructuredName.given_name][google.cloud.talent.v4beta1.PersonName.PersonStructuredName.given_name]
    ///   in descending order.
    /// * "last_name": Sort by [PersonName.PersonStructuredName.family_name][google.cloud.talent.v4beta1.PersonName.PersonStructuredName.family_name] in
    ///   ascending order.
    /// * "last_name desc": Sort by [PersonName.PersonStructuredName.family_name][google.cloud.talent.v4beta1.PersonName.PersonStructuredName.family_name]
    ///   in ascending order.
    #[prost(string, tag = "8")]
    pub order_by: std::string::String,
    /// When sort by field is based on alphabetical order, sort values case
    /// sensitively (based on ASCII) when the value is set to true. Default value
    /// is case in-sensitive sort (false).
    #[prost(bool, tag = "9")]
    pub case_sensitive_sort: bool,
    /// A list of expressions specifies histogram requests against matching
    /// profiles for [SearchProfilesRequest][google.cloud.talent.v4beta1.SearchProfilesRequest].
    ///
    /// The expression syntax looks like a function definition with parameters.
    ///
    /// Function syntax: function_name(histogram_facet[, list of buckets])
    ///
    /// Data types:
    ///
    /// * Histogram facet: facet names with format [a-zA-Z][a-zA-Z0-9_]+.
    /// * String: string like "any string with backslash escape for quote(\")."
    /// * Number: whole number and floating point number like 10, -1 and -0.01.
    /// * List: list of elements with comma(,) separator surrounded by square
    /// brackets. For example, [1, 2, 3] and ["one", "two", "three"].
    ///
    /// Built-in constants:
    ///
    /// * MIN (minimum number similar to java Double.MIN_VALUE)
    /// * MAX (maximum number similar to java Double.MAX_VALUE)
    ///
    /// Built-in functions:
    ///
    /// * bucket(start, end[, label])
    /// Bucket build-in function creates a bucket with range of [start, end). Note
    /// that the end is exclusive.
    /// For example, bucket(1, MAX, "positive number") or bucket(1, 10).
    ///
    /// Histogram Facets:
    ///
    /// * admin1: Admin1 is a global placeholder for referring to state, province,
    /// or the particular term a country uses to define the geographic structure
    /// below the country level. Examples include states codes such as "CA", "IL",
    /// "NY", and provinces, such as "BC".
    /// * locality: Locality is a global placeholder for referring to city, town,
    /// or the particular term a country uses to define the geographic structure
    /// below the admin1 level. Examples include city names such as
    /// "Mountain View" and "New York".
    /// * extended_locality: Extended locality is concatenated version of admin1
    /// and locality with comma separator. For example, "Mountain View, CA" and
    /// "New York, NY".
    /// * postal_code: Postal code of profile which follows locale code.
    /// * country: Country code (ISO-3166-1 alpha-2 code) of profile, such as US,
    ///  JP, GB.
    /// * job_title: Normalized job titles specified in EmploymentHistory.
    /// * company_name: Normalized company name of profiles to match on.
    /// * institution: The school name. For example, "MIT",
    /// "University of California, Berkeley"
    /// * degree: Highest education degree in ISCED code. Each value in degree
    /// covers a specific level of education, without any expansion to upper nor
    /// lower levels of education degree.
    /// * experience_in_months: experience in months. 0 means 0 month to 1 month
    /// (exclusive).
    /// * application_date: The application date specifies application start dates.
    /// See [ApplicationDateFilter][google.cloud.talent.v4beta1.ApplicationDateFilter] for more details.
    /// * application_outcome_notes: The application outcome reason specifies the
    /// reasons behind the outcome of the job application.
    /// See [ApplicationOutcomeNotesFilter][google.cloud.talent.v4beta1.ApplicationOutcomeNotesFilter] for more details.
    /// * application_job_title: The application job title specifies the job
    /// applied for in the application.
    /// See [ApplicationJobFilter][google.cloud.talent.v4beta1.ApplicationJobFilter] for more details.
    /// * hirable_status: Hirable status specifies the profile's hirable status.
    /// * string_custom_attribute: String custom attributes. Values can be accessed
    /// via square bracket notation like string_custom_attribute["key1"].
    /// * numeric_custom_attribute: Numeric custom attributes. Values can be
    /// accessed via square bracket notation like numeric_custom_attribute["key1"].
    ///
    /// Example expressions:
    ///
    /// * count(admin1)
    /// * count(experience_in_months, [bucket(0, 12, "1 year"),
    /// bucket(12, 36, "1-3 years"), bucket(36, MAX, "3+ years")])
    /// * count(string_custom_attribute["assigned_recruiter"])
    /// * count(numeric_custom_attribute["favorite_number"],
    /// [bucket(MIN, 0, "negative"), bucket(0, MAX, "non-negative")])
    #[prost(message, repeated, tag = "10")]
    pub histogram_queries: ::std::vec::Vec<HistogramQuery>,
    /// An id that uniquely identifies the result set of a
    /// [SearchProfiles][google.cloud.talent.v4beta1.ProfileService.SearchProfiles] call. The id should be
    /// retrieved from the
    /// [SearchProfilesResponse][google.cloud.talent.v4beta1.SearchProfilesResponse] message returned from a previous
    /// invocation of [SearchProfiles][google.cloud.talent.v4beta1.ProfileService.SearchProfiles].
    ///
    /// A result set is an ordered list of search results.
    ///
    /// If this field is not set, a new result set is computed based on the
    /// [profile_query][google.cloud.talent.v4beta1.SearchProfilesRequest.profile_query].  A new [result_set_id][google.cloud.talent.v4beta1.SearchProfilesRequest.result_set_id] is returned as a handle to
    /// access this result set.
    ///
    /// If this field is set, the service will ignore the resource and
    /// [profile_query][google.cloud.talent.v4beta1.SearchProfilesRequest.profile_query] values, and simply retrieve a page of results from the
    /// corresponding result set.  In this case, one and only one of [page_token][google.cloud.talent.v4beta1.SearchProfilesRequest.page_token]
    /// or [offset][google.cloud.talent.v4beta1.SearchProfilesRequest.offset] must be set.
    ///
    /// A typical use case is to invoke [SearchProfilesRequest][google.cloud.talent.v4beta1.SearchProfilesRequest] without this
    /// field, then use the resulting [result_set_id][google.cloud.talent.v4beta1.SearchProfilesRequest.result_set_id] in
    /// [SearchProfilesResponse][google.cloud.talent.v4beta1.SearchProfilesResponse] to page through the results.
    #[prost(string, tag = "12")]
    pub result_set_id: std::string::String,
    /// This flag is used to indicate whether the service will attempt to
    /// understand synonyms and terms related to the search query or treat the
    /// query "as is" when it generates a set of results. By default this flag is
    /// set to false, thus allowing expanded results to also be returned. For
    /// example a search for "software engineer" might also return candidates who
    /// have experience in jobs similar to software engineer positions. By setting
    /// this flag to true, the service will only attempt to deliver candidates has
    /// software engineer in his/her global fields by treating "software engineer"
    /// as a keyword.
    ///
    /// It is recommended to provide a feature in the UI (such as a checkbox) to
    /// allow recruiters to set this flag to true if they intend to search for
    /// longer boolean strings.
    #[prost(bool, tag = "13")]
    pub strict_keywords_search: bool,
}
/// Response of SearchProfiles method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchProfilesResponse {
    /// An estimation of the number of profiles that match the specified query.
    ///
    /// This number isn't guaranteed to be accurate.
    #[prost(int64, tag = "1")]
    pub estimated_total_size: i64,
    /// The spell checking result, and correction.
    #[prost(message, optional, tag = "2")]
    pub spell_correction: ::std::option::Option<SpellingCorrection>,
    /// Additional information for the API invocation, such as the request
    /// tracking id.
    #[prost(message, optional, tag = "3")]
    pub metadata: ::std::option::Option<ResponseMetadata>,
    /// A token to retrieve the next page of results. This is empty if there are no
    /// more results.
    #[prost(string, tag = "4")]
    pub next_page_token: std::string::String,
    /// The histogram results that match with specified
    /// [SearchProfilesRequest.histogram_queries][google.cloud.talent.v4beta1.SearchProfilesRequest.histogram_queries].
    #[prost(message, repeated, tag = "5")]
    pub histogram_query_results: ::std::vec::Vec<HistogramQueryResult>,
    /// The profile entities that match the specified [SearchProfilesRequest][google.cloud.talent.v4beta1.SearchProfilesRequest].
    #[prost(message, repeated, tag = "6")]
    pub summarized_profiles: ::std::vec::Vec<SummarizedProfile>,
    /// An id that uniquely identifies the result set of a
    /// [SearchProfiles][google.cloud.talent.v4beta1.ProfileService.SearchProfiles] call for consistent
    /// results.
    #[prost(string, tag = "7")]
    pub result_set_id: std::string::String,
}
/// Profile entry with metadata inside [SearchProfilesResponse][google.cloud.talent.v4beta1.SearchProfilesResponse].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SummarizedProfile {
    /// A list of profiles that are linked by [Profile.group_id][google.cloud.talent.v4beta1.Profile.group_id].
    #[prost(message, repeated, tag = "1")]
    pub profiles: ::std::vec::Vec<Profile>,
    /// A profile summary shows the profile summary and how the profile matches the
    /// search query.
    ///
    /// In profile summary, the profiles with the same [Profile.group_id][google.cloud.talent.v4beta1.Profile.group_id] are
    /// merged together. Among profiles, same education/employment records may be
    /// slightly different but they are merged into one with best efforts.
    ///
    /// For example, in one profile the school name is "UC Berkeley" and the field
    /// study is "Computer Science" and in another one the school name is
    /// "University of California at Berkeley" and the field study is "CS". The API
    /// merges these two inputs into one and selects one value for each field. For
    /// example, the school name in summary is set to "University of California at
    /// Berkeley" and the field of study is set to "Computer Science".
    #[prost(message, optional, tag = "2")]
    pub summary: ::std::option::Option<Profile>,
}
#[doc = r" Generated client implementations."]
pub mod profile_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " A service that handles profile management, including profile CRUD,"]
    #[doc = " enumeration and search."]
    pub struct ProfileServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ProfileServiceClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> ProfileServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + HttpBody + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as HttpBody>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = tonic::client::Grpc::with_interceptor(inner, interceptor);
            Self { inner }
        }
        #[doc = " Lists profiles by filter. The order is unspecified."]
        pub async fn list_profiles(
            &mut self,
            request: impl tonic::IntoRequest<super::ListProfilesRequest>,
        ) -> Result<tonic::Response<super::ListProfilesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.talent.v4beta1.ProfileService/ListProfiles",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates and returns a new profile."]
        pub async fn create_profile(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateProfileRequest>,
        ) -> Result<tonic::Response<super::Profile>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.talent.v4beta1.ProfileService/CreateProfile",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets the specified profile."]
        pub async fn get_profile(
            &mut self,
            request: impl tonic::IntoRequest<super::GetProfileRequest>,
        ) -> Result<tonic::Response<super::Profile>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.talent.v4beta1.ProfileService/GetProfile",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates the specified profile and returns the updated result."]
        pub async fn update_profile(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateProfileRequest>,
        ) -> Result<tonic::Response<super::Profile>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.talent.v4beta1.ProfileService/UpdateProfile",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes the specified profile."]
        #[doc = " Prerequisite: The profile has no associated applications or assignments"]
        #[doc = " associated."]
        pub async fn delete_profile(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteProfileRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.talent.v4beta1.ProfileService/DeleteProfile",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Searches for profiles within a tenant."]
        #[doc = ""]
        #[doc = " For example, search by raw queries \"software engineer in Mountain View\" or"]
        #[doc = " search by structured filters (location filter, education filter, etc.)."]
        #[doc = ""]
        #[doc = " See [SearchProfilesRequest][google.cloud.talent.v4beta1.SearchProfilesRequest] for more information."]
        pub async fn search_profiles(
            &mut self,
            request: impl tonic::IntoRequest<super::SearchProfilesRequest>,
        ) -> Result<tonic::Response<super::SearchProfilesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.talent.v4beta1.ProfileService/SearchProfiles",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for ProfileServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for ProfileServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "ProfileServiceClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod profile_service_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with ProfileServiceServer."]
    #[async_trait]
    pub trait ProfileService: Send + Sync + 'static {
        #[doc = " Lists profiles by filter. The order is unspecified."]
        async fn list_profiles(
            &self,
            request: tonic::Request<super::ListProfilesRequest>,
        ) -> Result<tonic::Response<super::ListProfilesResponse>, tonic::Status>;
        #[doc = " Creates and returns a new profile."]
        async fn create_profile(
            &self,
            request: tonic::Request<super::CreateProfileRequest>,
        ) -> Result<tonic::Response<super::Profile>, tonic::Status>;
        #[doc = " Gets the specified profile."]
        async fn get_profile(
            &self,
            request: tonic::Request<super::GetProfileRequest>,
        ) -> Result<tonic::Response<super::Profile>, tonic::Status>;
        #[doc = " Updates the specified profile and returns the updated result."]
        async fn update_profile(
            &self,
            request: tonic::Request<super::UpdateProfileRequest>,
        ) -> Result<tonic::Response<super::Profile>, tonic::Status>;
        #[doc = " Deletes the specified profile."]
        #[doc = " Prerequisite: The profile has no associated applications or assignments"]
        #[doc = " associated."]
        async fn delete_profile(
            &self,
            request: tonic::Request<super::DeleteProfileRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status>;
        #[doc = " Searches for profiles within a tenant."]
        #[doc = ""]
        #[doc = " For example, search by raw queries \"software engineer in Mountain View\" or"]
        #[doc = " search by structured filters (location filter, education filter, etc.)."]
        #[doc = ""]
        #[doc = " See [SearchProfilesRequest][google.cloud.talent.v4beta1.SearchProfilesRequest] for more information."]
        async fn search_profiles(
            &self,
            request: tonic::Request<super::SearchProfilesRequest>,
        ) -> Result<tonic::Response<super::SearchProfilesResponse>, tonic::Status>;
    }
    #[doc = " A service that handles profile management, including profile CRUD,"]
    #[doc = " enumeration and search."]
    #[derive(Debug)]
    #[doc(hidden)]
    pub struct ProfileServiceServer<T: ProfileService> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: ProfileService> ProfileServiceServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, None);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, Some(interceptor.into()));
            Self { inner }
        }
    }
    impl<T, B> Service<http::Request<B>> for ProfileServiceServer<T>
    where
        T: ProfileService,
        B: HttpBody + Send + Sync + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/google.cloud.talent.v4beta1.ProfileService/ListProfiles" => {
                    #[allow(non_camel_case_types)]
                    struct ListProfilesSvc<T: ProfileService>(pub Arc<T>);
                    impl<T: ProfileService> tonic::server::UnaryService<super::ListProfilesRequest>
                        for ListProfilesSvc<T>
                    {
                        type Response = super::ListProfilesResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListProfilesRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.list_profiles(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ListProfilesSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/google.cloud.talent.v4beta1.ProfileService/CreateProfile" => {
                    #[allow(non_camel_case_types)]
                    struct CreateProfileSvc<T: ProfileService>(pub Arc<T>);
                    impl<T: ProfileService> tonic::server::UnaryService<super::CreateProfileRequest>
                        for CreateProfileSvc<T>
                    {
                        type Response = super::Profile;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateProfileRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.create_profile(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = CreateProfileSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/google.cloud.talent.v4beta1.ProfileService/GetProfile" => {
                    #[allow(non_camel_case_types)]
                    struct GetProfileSvc<T: ProfileService>(pub Arc<T>);
                    impl<T: ProfileService> tonic::server::UnaryService<super::GetProfileRequest> for GetProfileSvc<T> {
                        type Response = super::Profile;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetProfileRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.get_profile(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetProfileSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/google.cloud.talent.v4beta1.ProfileService/UpdateProfile" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateProfileSvc<T: ProfileService>(pub Arc<T>);
                    impl<T: ProfileService> tonic::server::UnaryService<super::UpdateProfileRequest>
                        for UpdateProfileSvc<T>
                    {
                        type Response = super::Profile;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateProfileRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.update_profile(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = UpdateProfileSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/google.cloud.talent.v4beta1.ProfileService/DeleteProfile" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteProfileSvc<T: ProfileService>(pub Arc<T>);
                    impl<T: ProfileService> tonic::server::UnaryService<super::DeleteProfileRequest>
                        for DeleteProfileSvc<T>
                    {
                        type Response = ();
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteProfileRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.delete_profile(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = DeleteProfileSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/google.cloud.talent.v4beta1.ProfileService/SearchProfiles" => {
                    #[allow(non_camel_case_types)]
                    struct SearchProfilesSvc<T: ProfileService>(pub Arc<T>);
                    impl<T: ProfileService>
                        tonic::server::UnaryService<super::SearchProfilesRequest>
                        for SearchProfilesSvc<T>
                    {
                        type Response = super::SearchProfilesResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SearchProfilesRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.search_profiles(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = SearchProfilesSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .body(tonic::body::BoxBody::empty())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: ProfileService> Clone for ProfileServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: ProfileService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: ProfileService> tonic::transport::NamedService for ProfileServiceServer<T> {
        const NAME: &'static str = "google.cloud.talent.v4beta1.ProfileService";
    }
}
/// A Tenant resource represents a tenant in the service. A tenant is a group or
/// entity that shares common access with specific privileges for resources like
/// profiles. Customer may create multiple tenants to provide data isolation for
/// different groups.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Tenant {
    /// Required during tenant update.
    ///
    /// The resource name for a tenant. This is generated by the service when a
    /// tenant is created.
    ///
    /// The format is "projects/{project_id}/tenants/{tenant_id}", for example,
    /// "projects/foo/tenants/bar".
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Required. Client side tenant identifier, used to uniquely identify the tenant.
    ///
    /// The maximum number of allowed characters is 255.
    #[prost(string, tag = "2")]
    pub external_id: std::string::String,
    /// Indicates whether data owned by this tenant may be used to provide product
    /// improvements across other tenants.
    ///
    /// Defaults behavior is [DataUsageType.ISOLATED][google.cloud.talent.v4beta1.Tenant.DataUsageType.ISOLATED] if it's unset.
    #[prost(enumeration = "tenant::DataUsageType", tag = "3")]
    pub usage_type: i32,
    /// A list of keys of filterable [Profile.custom_attributes][google.cloud.talent.v4beta1.Profile.custom_attributes], whose
    /// corresponding `string_values` are used in keyword searches. Profiles with
    /// `string_values` under these specified field keys are returned if any
    /// of the values match the search keyword. Custom field values with
    /// parenthesis, brackets and special symbols are not searchable as-is,
    /// and must be surrounded by quotes.
    #[prost(string, repeated, tag = "4")]
    pub keyword_searchable_profile_custom_attributes: ::std::vec::Vec<std::string::String>,
}
pub mod tenant {
    /// Enum that represents how user data owned by the tenant is used.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum DataUsageType {
        /// Default value.
        Unspecified = 0,
        /// Data owned by this tenant is used to improve search/recommendation
        /// quality across tenants.
        Aggregated = 1,
        /// Data owned by this tenant is used to improve search/recommendation
        /// quality for this tenant only.
        Isolated = 2,
    }
}
/// The Request of the CreateTenant method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateTenantRequest {
    /// Required. Resource name of the project under which the tenant is created.
    ///
    /// The format is "projects/{project_id}", for example,
    /// "projects/foo".
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Required. The tenant to be created.
    #[prost(message, optional, tag = "2")]
    pub tenant: ::std::option::Option<Tenant>,
}
/// Request for getting a tenant by name.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTenantRequest {
    /// Required. The resource name of the tenant to be retrieved.
    ///
    /// The format is "projects/{project_id}/tenants/{tenant_id}", for example,
    /// "projects/foo/tenants/bar".
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Request for updating a specified tenant.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateTenantRequest {
    /// Required. The tenant resource to replace the current resource in the system.
    #[prost(message, optional, tag = "1")]
    pub tenant: ::std::option::Option<Tenant>,
    /// Strongly recommended for the best service experience.
    ///
    /// If [update_mask][google.cloud.talent.v4beta1.UpdateTenantRequest.update_mask] is provided, only the specified fields in
    /// [tenant][google.cloud.talent.v4beta1.UpdateTenantRequest.tenant] are updated. Otherwise all the fields are updated.
    ///
    /// A field mask to specify the tenant fields to be updated. Only
    /// top level fields of [Tenant][google.cloud.talent.v4beta1.Tenant] are supported.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
}
/// Request to delete a tenant.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteTenantRequest {
    /// Required. The resource name of the tenant to be deleted.
    ///
    /// The format is "projects/{project_id}/tenants/{tenant_id}", for example,
    /// "projects/foo/tenants/bar".
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// List tenants for which the client has ACL visibility.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTenantsRequest {
    /// Required. Resource name of the project under which the tenant is created.
    ///
    /// The format is "projects/{project_id}", for example,
    /// "projects/foo".
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// The starting indicator from which to return results.
    #[prost(string, tag = "2")]
    pub page_token: std::string::String,
    /// The maximum number of tenants to be returned, at most 100.
    /// Default is 100 if a non-positive number is provided.
    #[prost(int32, tag = "3")]
    pub page_size: i32,
}
/// The List tenants response object.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTenantsResponse {
    /// Tenants for the current client.
    #[prost(message, repeated, tag = "1")]
    pub tenants: ::std::vec::Vec<Tenant>,
    /// A token to retrieve the next page of results.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
    /// Additional information for the API invocation, such as the request
    /// tracking id.
    #[prost(message, optional, tag = "3")]
    pub metadata: ::std::option::Option<ResponseMetadata>,
}
#[doc = r" Generated client implementations."]
pub mod tenant_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " A service that handles tenant management, including CRUD and enumeration."]
    pub struct TenantServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl TenantServiceClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> TenantServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + HttpBody + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as HttpBody>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = tonic::client::Grpc::with_interceptor(inner, interceptor);
            Self { inner }
        }
        #[doc = " Creates a new tenant entity."]
        pub async fn create_tenant(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateTenantRequest>,
        ) -> Result<tonic::Response<super::Tenant>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.talent.v4beta1.TenantService/CreateTenant",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Retrieves specified tenant."]
        pub async fn get_tenant(
            &mut self,
            request: impl tonic::IntoRequest<super::GetTenantRequest>,
        ) -> Result<tonic::Response<super::Tenant>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.talent.v4beta1.TenantService/GetTenant",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates specified tenant."]
        pub async fn update_tenant(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateTenantRequest>,
        ) -> Result<tonic::Response<super::Tenant>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.talent.v4beta1.TenantService/UpdateTenant",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes specified tenant."]
        pub async fn delete_tenant(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteTenantRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.talent.v4beta1.TenantService/DeleteTenant",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists all tenants associated with the project."]
        pub async fn list_tenants(
            &mut self,
            request: impl tonic::IntoRequest<super::ListTenantsRequest>,
        ) -> Result<tonic::Response<super::ListTenantsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.talent.v4beta1.TenantService/ListTenants",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for TenantServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for TenantServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "TenantServiceClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod tenant_service_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with TenantServiceServer."]
    #[async_trait]
    pub trait TenantService: Send + Sync + 'static {
        #[doc = " Creates a new tenant entity."]
        async fn create_tenant(
            &self,
            request: tonic::Request<super::CreateTenantRequest>,
        ) -> Result<tonic::Response<super::Tenant>, tonic::Status>;
        #[doc = " Retrieves specified tenant."]
        async fn get_tenant(
            &self,
            request: tonic::Request<super::GetTenantRequest>,
        ) -> Result<tonic::Response<super::Tenant>, tonic::Status>;
        #[doc = " Updates specified tenant."]
        async fn update_tenant(
            &self,
            request: tonic::Request<super::UpdateTenantRequest>,
        ) -> Result<tonic::Response<super::Tenant>, tonic::Status>;
        #[doc = " Deletes specified tenant."]
        async fn delete_tenant(
            &self,
            request: tonic::Request<super::DeleteTenantRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status>;
        #[doc = " Lists all tenants associated with the project."]
        async fn list_tenants(
            &self,
            request: tonic::Request<super::ListTenantsRequest>,
        ) -> Result<tonic::Response<super::ListTenantsResponse>, tonic::Status>;
    }
    #[doc = " A service that handles tenant management, including CRUD and enumeration."]
    #[derive(Debug)]
    #[doc(hidden)]
    pub struct TenantServiceServer<T: TenantService> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: TenantService> TenantServiceServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, None);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, Some(interceptor.into()));
            Self { inner }
        }
    }
    impl<T, B> Service<http::Request<B>> for TenantServiceServer<T>
    where
        T: TenantService,
        B: HttpBody + Send + Sync + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/google.cloud.talent.v4beta1.TenantService/CreateTenant" => {
                    #[allow(non_camel_case_types)]
                    struct CreateTenantSvc<T: TenantService>(pub Arc<T>);
                    impl<T: TenantService> tonic::server::UnaryService<super::CreateTenantRequest>
                        for CreateTenantSvc<T>
                    {
                        type Response = super::Tenant;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateTenantRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.create_tenant(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = CreateTenantSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/google.cloud.talent.v4beta1.TenantService/GetTenant" => {
                    #[allow(non_camel_case_types)]
                    struct GetTenantSvc<T: TenantService>(pub Arc<T>);
                    impl<T: TenantService> tonic::server::UnaryService<super::GetTenantRequest> for GetTenantSvc<T> {
                        type Response = super::Tenant;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetTenantRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.get_tenant(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetTenantSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/google.cloud.talent.v4beta1.TenantService/UpdateTenant" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateTenantSvc<T: TenantService>(pub Arc<T>);
                    impl<T: TenantService> tonic::server::UnaryService<super::UpdateTenantRequest>
                        for UpdateTenantSvc<T>
                    {
                        type Response = super::Tenant;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateTenantRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.update_tenant(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = UpdateTenantSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/google.cloud.talent.v4beta1.TenantService/DeleteTenant" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteTenantSvc<T: TenantService>(pub Arc<T>);
                    impl<T: TenantService> tonic::server::UnaryService<super::DeleteTenantRequest>
                        for DeleteTenantSvc<T>
                    {
                        type Response = ();
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteTenantRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.delete_tenant(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = DeleteTenantSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/google.cloud.talent.v4beta1.TenantService/ListTenants" => {
                    #[allow(non_camel_case_types)]
                    struct ListTenantsSvc<T: TenantService>(pub Arc<T>);
                    impl<T: TenantService> tonic::server::UnaryService<super::ListTenantsRequest>
                        for ListTenantsSvc<T>
                    {
                        type Response = super::ListTenantsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListTenantsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.list_tenants(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ListTenantsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .body(tonic::body::BoxBody::empty())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: TenantService> Clone for TenantServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: TenantService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: TenantService> tonic::transport::NamedService for TenantServiceServer<T> {
        const NAME: &'static str = "google.cloud.talent.v4beta1.TenantService";
    }
}
