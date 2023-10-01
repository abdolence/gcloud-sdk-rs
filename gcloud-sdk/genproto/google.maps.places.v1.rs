/// Circle with a LatLng as center and radius.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Circle {
    /// Required. Center latitude and longitude.
    ///
    /// The range of latitude must be within `\[-90.0, 90.0\]`. The range of the
    /// longitude must be within `\[-180.0, 180.0\]`.
    #[prost(message, optional, tag = "1")]
    pub center: ::core::option::Option<super::super::super::r#type::LatLng>,
    /// Required. Radius measured in meters. The radius must be within `[0.0,
    /// 50000.0]`.
    #[prost(double, tag = "2")]
    pub radius: f64,
}
/// All the information representing a Place.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Place {
    /// Required. The unique identifier of a place.
    #[prost(string, tag = "2")]
    pub id: ::prost::alloc::string::String,
    /// The localized name of the place, suitable as a short human-readable
    /// description. For example, "Google Sydney", "Starbucks", "Pyrmont", etc.
    #[prost(message, optional, tag = "31")]
    pub display_name: ::core::option::Option<super::super::super::r#type::LocalizedText>,
    /// A set of type tags for this result. For example, "political" and
    /// "locality".
    #[prost(string, repeated, tag = "5")]
    pub types: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// A human-readable phone number for the place, in national format.
    #[prost(string, tag = "7")]
    pub national_phone_number: ::prost::alloc::string::String,
    /// A human-readable phone number for the place, in international format.
    #[prost(string, tag = "8")]
    pub international_phone_number: ::prost::alloc::string::String,
    /// A full, human-readable address for this place.
    #[prost(string, tag = "9")]
    pub formatted_address: ::prost::alloc::string::String,
    /// Repeated components for each locality level.
    #[prost(message, repeated, tag = "10")]
    pub address_components: ::prost::alloc::vec::Vec<place::AddressComponent>,
    /// Plus code of the place location lat/long.
    #[prost(message, optional, tag = "11")]
    pub plus_code: ::core::option::Option<place::PlusCode>,
    /// The position of this place.
    #[prost(message, optional, tag = "12")]
    pub location: ::core::option::Option<super::super::super::r#type::LatLng>,
    /// A viewport suitable for displaying the place on an average-sized map.
    #[prost(message, optional, tag = "13")]
    pub viewport: ::core::option::Option<super::super::super::geo::r#type::Viewport>,
    /// A rating between 1.0 and 5.0, based on user reviews of this place.
    #[prost(double, tag = "14")]
    pub rating: f64,
    /// A URL providing more information about this place.
    #[prost(string, tag = "15")]
    pub google_maps_uri: ::prost::alloc::string::String,
    /// The authoritative website for this place, e.g. a business' homepage.
    /// Note that for places that are part of a chain (e.g. an IKEA store), this
    /// will usually be the website for the individual store, not the overall
    /// chain.
    #[prost(string, tag = "16")]
    pub website_uri: ::prost::alloc::string::String,
    /// List of reviews about this place.
    #[prost(message, repeated, tag = "20")]
    pub reviews: ::prost::alloc::vec::Vec<place::Review>,
    /// The regular hours of operation.
    #[prost(message, optional, tag = "21")]
    pub opening_hours: ::core::option::Option<place::OpeningHours>,
    /// Number of minutes this place's timezone is currently offset from UTC.
    /// This is expressed in minutes to support timezones that are offset by
    /// fractions of an hour, e.g. X hours and 15 minutes.
    #[prost(int32, tag = "22")]
    pub utc_offset_minutes: i32,
    /// The place's address in adr microformat: <http://microformats.org/wiki/adr.>
    #[prost(string, tag = "24")]
    pub adr_format_address: ::prost::alloc::string::String,
    /// The business status for the place.
    #[prost(enumeration = "place::BusinessStatus", tag = "25")]
    pub business_status: i32,
    /// Price level of the place.
    #[prost(enumeration = "PriceLevel", tag = "26")]
    pub price_level: i32,
    /// A set of data provider that must be shown with this result.
    #[prost(message, repeated, tag = "27")]
    pub attributions: ::prost::alloc::vec::Vec<place::Attribution>,
    /// The total number of reviews (with or without text) for this place.
    #[prost(int32, tag = "28")]
    pub user_rating_count: i32,
    /// A truncated URL to an v2 icon mask. User can access different icon type by
    /// appending type suffix to the end (eg, ".svg" or ".png").
    #[prost(string, tag = "29")]
    pub icon_mask_base_uri: ::prost::alloc::string::String,
    /// Background color for icon_mask in hex format, e.g. #909CE1.
    #[prost(string, tag = "30")]
    pub icon_background_color: ::prost::alloc::string::String,
    /// Specifies if the business supports takeout.
    #[prost(bool, optional, tag = "33")]
    pub takeout: ::core::option::Option<bool>,
    /// Specifies if the business supports delivery.
    #[prost(bool, optional, tag = "34")]
    pub delivery: ::core::option::Option<bool>,
    /// Specifies if the business supports indoor or outdoor seating options.
    #[prost(bool, optional, tag = "35")]
    pub dine_in: ::core::option::Option<bool>,
    /// Specifies if the business supports curbside pickup.
    #[prost(bool, optional, tag = "36")]
    pub curbside_pickup: ::core::option::Option<bool>,
    /// Specifies if the place has an entrance that is wheelchair-accessible.
    #[prost(bool, optional, tag = "37")]
    pub wheelchair_accessible_entrance: ::core::option::Option<bool>,
    /// Specifies if the place supports reservations.
    #[prost(bool, optional, tag = "38")]
    pub reservable: ::core::option::Option<bool>,
    /// Specifies if the place serves breakfast.
    #[prost(bool, optional, tag = "39")]
    pub serves_breakfast: ::core::option::Option<bool>,
    /// Specifies if the place serves lunch.
    #[prost(bool, optional, tag = "40")]
    pub serves_lunch: ::core::option::Option<bool>,
    /// Specifies if the place serves dinner.
    #[prost(bool, optional, tag = "41")]
    pub serves_dinner: ::core::option::Option<bool>,
    /// Specifies if the place serves beer.
    #[prost(bool, optional, tag = "42")]
    pub serves_beer: ::core::option::Option<bool>,
    /// Specifies if the place serves wine.
    #[prost(bool, optional, tag = "43")]
    pub serves_wine: ::core::option::Option<bool>,
    /// Specifies if the place serves brunch.
    #[prost(bool, optional, tag = "44")]
    pub serves_brunch: ::core::option::Option<bool>,
    /// Specifies if the place serves vegetarian food.
    #[prost(bool, optional, tag = "45")]
    pub serves_vegetarian_food: ::core::option::Option<bool>,
    /// The hours of operation for the next seven days (including today). The time
    /// period starts at midnight on the date of the request and ends at 11:59 pm
    /// six days later. This field includes the special_days subfield of all hours,
    /// set for dates that have exceptional hours.
    #[prost(message, optional, tag = "46")]
    pub current_opening_hours: ::core::option::Option<place::OpeningHours>,
    /// Contains an array of entries for the next seven days including information
    /// about secondary hours of a business. Secondary hours are different from a
    /// business's main hours. For example, a restaurant can specify drive through
    /// hours or delivery hours as its secondary hours. This field populates the
    /// type subfield, which draws from a predefined list of opening hours types
    /// (such as DRIVE_THROUGH, PICKUP, or TAKEOUT) based on the types of the
    /// place. This field includes the special_days subfield of all hours, set for
    /// dates that have exceptional hours.
    #[prost(message, repeated, tag = "47")]
    pub current_secondary_opening_hours: ::prost::alloc::vec::Vec<place::OpeningHours>,
    /// Contains an array of entries for information about regular secondary hours
    /// of a business. Secondary hours are different from a business's main hours.
    /// For example, a restaurant can specify drive through hours or delivery hours
    /// as its secondary hours. This field populates the type subfield, which draws
    /// from a predefined list of opening hours types (such as DRIVE_THROUGH,
    /// PICKUP, or TAKEOUT) based on the types of the place.
    #[prost(message, repeated, tag = "49")]
    pub secondary_opening_hours: ::prost::alloc::vec::Vec<place::OpeningHours>,
    /// Contains a summary of the place. A summary is comprised of a textual
    /// overview, and also includes the language code for these if applicable.
    /// Summary text must be presented as-is and can not be modified or altered.
    #[prost(message, optional, tag = "48")]
    pub editorial_summary: ::core::option::Option<place::EditorialSummary>,
}
/// Nested message and enum types in `Place`.
pub mod place {
    /// The structured components that form the formatted address, if this
    /// information is available.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct AddressComponent {
        /// The full text description or name of the address component. For example,
        /// an address component for the country Australia may have a long_name of
        /// "Australia".
        #[prost(string, tag = "1")]
        pub long_text: ::prost::alloc::string::String,
        /// An abbreviated textual name for the address component, if available. For
        /// example, an address component for the country of Australia may have a
        /// short_name of "AU".
        #[prost(string, tag = "2")]
        pub short_text: ::prost::alloc::string::String,
        /// An array indicating the type(s) of the address component.
        #[prost(string, repeated, tag = "3")]
        pub types: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// The language used to format this components, in CLDR notation.
        #[prost(string, tag = "4")]
        pub language_code: ::prost::alloc::string::String,
    }
    /// Plus code (<http://plus.codes>) is a location reference with two formats:
    /// global code defining a 14mx14m (1/8000th of a degree) or smaller rectangle,
    /// and compound code, replacing the prefix with a reference location.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PlusCode {
        /// Place's global (full) code, such as `9FWM33GV+HQ`, representing an
        /// 1/8000 by 1/8000 degree area (~14 by 14 meters).
        #[prost(string, tag = "1")]
        pub global_code: ::prost::alloc::string::String,
        /// Place's compound code, such as `33GV+HQ, Ramberg, Norway`, containing
        /// the suffix of the global code and replacing the prefix with a formatted
        /// name of a reference entity.
        #[prost(string, tag = "2")]
        pub compound_code: ::prost::alloc::string::String,
    }
    /// Information about a review of the place.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Review {
        /// Timestamp for the review, expressed in seconds since epoch.
        #[prost(message, optional, tag = "1")]
        pub publish_time: ::core::option::Option<::prost_types::Timestamp>,
        /// A string of formatted recent time, expressing the review time relative
        /// to the current time in a form appropriate for the language and country.
        #[prost(string, tag = "2")]
        pub relative_publish_time_description: ::prost::alloc::string::String,
        /// The localized text of the review.
        #[prost(message, optional, tag = "9")]
        pub text: ::core::option::Option<
            super::super::super::super::r#type::LocalizedText,
        >,
        /// The name of the review author.
        #[prost(string, tag = "4")]
        pub author: ::prost::alloc::string::String,
        /// A link to the review author's profile.
        #[prost(string, tag = "5")]
        pub author_uri: ::prost::alloc::string::String,
        /// The author's profile photo.
        #[prost(string, tag = "6")]
        pub author_photo_uri: ::prost::alloc::string::String,
        /// A whole number between 1.0 and 5.0, a.k.a. the number of stars.
        #[prost(double, tag = "7")]
        pub rating: f64,
        /// A BCP-47 language code indicating the original language of the review.
        /// If the review has been translated, then original_language != language.
        /// This field contains the main language tag only, and not the secondary tag
        /// indicating country or region. For example, all the English reviews are
        /// tagged as 'en', and not 'en-AU' or 'en-UK' and so on.This field is empty
        /// if there is only a rating with no review text.
        #[prost(string, tag = "10")]
        pub original_language_code: ::prost::alloc::string::String,
        /// A boolean value indicating if the review was translated from the original
        /// language it was written in. If a review has been translated,
        /// corresponding to a value of true, Google recommends that you indicate
        /// this to your users. For example, you can add the following string,
        /// “Translated by Google”, to the review.
        #[prost(bool, tag = "11")]
        pub translated: bool,
    }
    /// Information about business hour of the place.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct OpeningHours {
        /// Is this place open right now?  Always present unless we lack time-of-day
        /// or timezone data for these opening hours.
        #[prost(bool, tag = "1")]
        pub open_now: bool,
        /// The periods that this place is open during the week. The periods are in
        /// chronological order, starting with Sunday in the place-local timezone. An
        /// empty (but not absent) value indicates a place that is never open, e.g.
        /// because it is closed temporarily for renovations.
        #[prost(message, repeated, tag = "2")]
        pub periods: ::prost::alloc::vec::Vec<opening_hours::OpeningHoursPeriod>,
        /// Localized strings describing the opening hours of this place, one string
        /// for each day of the week.  Will be empty if the hours are unknown or
        /// could not be converted to localized text. Example: "Sun: 18:00–06:00"
        #[prost(string, repeated, tag = "3")]
        pub weekday_descriptions: ::prost::alloc::vec::Vec<
            ::prost::alloc::string::String,
        >,
        /// A type string used to identify the type of secondary hours.
        #[prost(enumeration = "opening_hours::SecondaryHourType", tag = "4")]
        pub secondary_hour_type: i32,
        /// Structured information for special days that fall within the period that
        /// the returned opening hours cover. Special days are days that could impact
        /// the business hours of a place, e.g. Christmas day. Set for
        /// current_opening_hours and current_secondary_opening_hours if there are
        /// exceptional hours.
        #[prost(message, repeated, tag = "5")]
        pub special_days: ::prost::alloc::vec::Vec<opening_hours::SpecialDay>,
    }
    /// Nested message and enum types in `OpeningHours`.
    pub mod opening_hours {
        /// A period the place remains in open_now status.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct OpeningHoursPeriod {
            /// The time that the place starts to be open.
            #[prost(message, optional, tag = "1")]
            pub open: ::core::option::Option<opening_hours_period::OpeningHoursPoint>,
            /// The time that the place starts to be closed.
            #[prost(message, optional, tag = "2")]
            pub close: ::core::option::Option<opening_hours_period::OpeningHoursPoint>,
        }
        /// Nested message and enum types in `OpeningHoursPeriod`.
        pub mod opening_hours_period {
            /// Status changing points.
            #[allow(clippy::derive_partial_eq_without_eq)]
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct OpeningHoursPoint {
                /// A day of the week, as an integer in the range 0-6.  0 is Sunday, 1 is
                /// Monday, etc.
                #[prost(int32, optional, tag = "1")]
                pub day: ::core::option::Option<i32>,
                /// The hour in 2 digits. Ranges from 00 to 23.
                #[prost(int32, optional, tag = "2")]
                pub hour: ::core::option::Option<i32>,
                /// The minute in 2 digits. Ranges from 00 to 59.
                #[prost(int32, optional, tag = "3")]
                pub minute: ::core::option::Option<i32>,
                /// Date of the endpoint expressed in `RFC3339` format in the local
                /// timezone for the place. For example 2010-12-31.
                #[deprecated]
                #[prost(string, tag = "4")]
                pub date_deprecated: ::prost::alloc::string::String,
                /// Date in the local timezone for the place.
                #[prost(message, optional, tag = "6")]
                pub date: ::core::option::Option<
                    super::super::super::super::super::super::r#type::Date,
                >,
                /// Whether or not this endpoint was truncated. Truncation occurs when
                /// the real hours are outside the times we are willing to return hours
                /// between, so we truncate the hours back to these boundaries. This
                /// ensures that at most `24 * 7` hours from midnight of the day of the
                /// request are returned.
                #[prost(bool, tag = "5")]
                pub truncated: bool,
            }
        }
        /// Structured information for special days that fall within the period that
        /// the returned opening hours cover. Special days are days that could impact
        /// the business hours of a place, e.g. Christmas day.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct SpecialDay {
            /// The date of this special day.
            #[prost(message, optional, tag = "1")]
            pub date: ::core::option::Option<
                super::super::super::super::super::r#type::Date,
            >,
        }
        /// A type used to identify the type of secondary hours.
        #[derive(
            Clone,
            Copy,
            Debug,
            PartialEq,
            Eq,
            Hash,
            PartialOrd,
            Ord,
            ::prost::Enumeration
        )]
        #[repr(i32)]
        pub enum SecondaryHourType {
            /// Default value when secondary hour type is not specified.
            Unspecified = 0,
            /// The drive-through hour for banks, restaurants, or pharmacies.
            DriveThrough = 1,
            /// The happy hour.
            HappyHour = 2,
            /// The delivery hour.
            Delivery = 3,
            /// The takeout hour.
            Takeout = 4,
            /// The kitchen hour.
            Kitchen = 5,
            /// The breakfast hour.
            Breakfast = 6,
            /// The lunch hour.
            Lunch = 7,
            /// The dinner hour.
            Dinner = 8,
            /// The brunch hour.
            Brunch = 9,
            /// The pickup hour.
            Pickup = 10,
            /// The access hours for storage places.
            Access = 11,
            /// The special hours for seniors.
            SeniorHours = 12,
            /// The online service hours.
            OnlineServiceHours = 13,
        }
        impl SecondaryHourType {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    SecondaryHourType::Unspecified => "SECONDARY_HOUR_TYPE_UNSPECIFIED",
                    SecondaryHourType::DriveThrough => "DRIVE_THROUGH",
                    SecondaryHourType::HappyHour => "HAPPY_HOUR",
                    SecondaryHourType::Delivery => "DELIVERY",
                    SecondaryHourType::Takeout => "TAKEOUT",
                    SecondaryHourType::Kitchen => "KITCHEN",
                    SecondaryHourType::Breakfast => "BREAKFAST",
                    SecondaryHourType::Lunch => "LUNCH",
                    SecondaryHourType::Dinner => "DINNER",
                    SecondaryHourType::Brunch => "BRUNCH",
                    SecondaryHourType::Pickup => "PICKUP",
                    SecondaryHourType::Access => "ACCESS",
                    SecondaryHourType::SeniorHours => "SENIOR_HOURS",
                    SecondaryHourType::OnlineServiceHours => "ONLINE_SERVICE_HOURS",
                }
            }
            /// Creates an enum from field names used in the ProtoBuf definition.
            pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                match value {
                    "SECONDARY_HOUR_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                    "DRIVE_THROUGH" => Some(Self::DriveThrough),
                    "HAPPY_HOUR" => Some(Self::HappyHour),
                    "DELIVERY" => Some(Self::Delivery),
                    "TAKEOUT" => Some(Self::Takeout),
                    "KITCHEN" => Some(Self::Kitchen),
                    "BREAKFAST" => Some(Self::Breakfast),
                    "LUNCH" => Some(Self::Lunch),
                    "DINNER" => Some(Self::Dinner),
                    "BRUNCH" => Some(Self::Brunch),
                    "PICKUP" => Some(Self::Pickup),
                    "ACCESS" => Some(Self::Access),
                    "SENIOR_HOURS" => Some(Self::SeniorHours),
                    "ONLINE_SERVICE_HOURS" => Some(Self::OnlineServiceHours),
                    _ => None,
                }
            }
        }
    }
    /// Information about data providers of this place.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Attribution {
        /// Name of the Place's data provider.
        #[prost(string, tag = "1")]
        pub provider: ::prost::alloc::string::String,
        /// URI to the Place's data provider.
        #[prost(string, tag = "2")]
        pub provider_uri: ::prost::alloc::string::String,
    }
    /// Contains a summary of the place.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct EditorialSummary {
        /// A summary is comprised of a textual overview, and also includes the
        /// language code for these if applicable. Summary text must be presented
        /// as-is and can not be modified or altered.
        #[prost(message, optional, tag = "1")]
        pub overview: ::core::option::Option<
            super::super::super::super::r#type::LocalizedText,
        >,
    }
    /// Business status for the place.
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum BusinessStatus {
        /// Default value. This value is unused.
        Unspecified = 0,
        /// The establishment is operational, not necessarily open now.
        Operational = 1,
        /// The establishment is temporarily closed.
        ClosedTemporarily = 2,
        /// The establishment is permanently closed.
        ClosedPermanently = 3,
    }
    impl BusinessStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                BusinessStatus::Unspecified => "BUSINESS_STATUS_UNSPECIFIED",
                BusinessStatus::Operational => "OPERATIONAL",
                BusinessStatus::ClosedTemporarily => "CLOSED_TEMPORARILY",
                BusinessStatus::ClosedPermanently => "CLOSED_PERMANENTLY",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "BUSINESS_STATUS_UNSPECIFIED" => Some(Self::Unspecified),
                "OPERATIONAL" => Some(Self::Operational),
                "CLOSED_TEMPORARILY" => Some(Self::ClosedTemporarily),
                "CLOSED_PERMANENTLY" => Some(Self::ClosedPermanently),
                _ => None,
            }
        }
    }
}
/// Price level of the place.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PriceLevel {
    /// Place price level is unspecified or unknown.
    Unspecified = 0,
    Free = 1,
    /// Place provides inexpensive services.
    Inexpensive = 2,
    /// Place provides moderately priced services.
    Moderate = 3,
    /// Place provides expensive services.
    Expensive = 4,
    /// Place provides very expensive services.
    VeryExpensive = 5,
}
impl PriceLevel {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PriceLevel::Unspecified => "PRICE_LEVEL_UNSPECIFIED",
            PriceLevel::Free => "FREE",
            PriceLevel::Inexpensive => "INEXPENSIVE",
            PriceLevel::Moderate => "MODERATE",
            PriceLevel::Expensive => "EXPENSIVE",
            PriceLevel::VeryExpensive => "VERY_EXPENSIVE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "PRICE_LEVEL_UNSPECIFIED" => Some(Self::Unspecified),
            "FREE" => Some(Self::Free),
            "INEXPENSIVE" => Some(Self::Inexpensive),
            "MODERATE" => Some(Self::Moderate),
            "EXPENSIVE" => Some(Self::Expensive),
            "VERY_EXPENSIVE" => Some(Self::VeryExpensive),
            _ => None,
        }
    }
}
/// int 32 range. Both min and max are optional. If only min is set, then the
/// range only has a lower bound. If only max is set, then range only has an
/// upper bound. At least one of min and max must be set. Values are inclusive.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Int32Range {
    /// Lower bound. If unset, behavior is documented on the range field.
    #[prost(int32, optional, tag = "1")]
    pub min: ::core::option::Option<i32>,
    /// Upper bound. If unset, behavior is documented on the range field.
    #[prost(int32, optional, tag = "2")]
    pub max: ::core::option::Option<i32>,
}
/// Request data structure for SearchText.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchTextRequest {
    /// Required. The text query for textual search.
    #[prost(string, tag = "1")]
    pub text_query: ::prost::alloc::string::String,
    /// Place details will be displayed with the preferred language if available.
    /// If the language code is unspecified or unrecognized, place details of any
    /// language may be returned, with a preference for English if such details
    /// exist.
    ///
    /// Current list of supported languages:
    /// <https://developers.google.com/maps/faq#languagesupport.>
    #[prost(string, tag = "2")]
    pub language_code: ::prost::alloc::string::String,
    /// The Unicode country/region code (CLDR) of the location where the
    /// request is coming from. It is used to display the place details, like
    /// region-specific place name, if available.
    ///
    /// For more information, see
    /// <http://www.unicode.org/reports/tr35/#unicode_region_subtag.>
    ///
    ///
    /// Note that 3-digit region codes are not currently supported.
    #[prost(string, tag = "3")]
    pub region_code: ::prost::alloc::string::String,
    /// How results will be ranked in the response.
    #[prost(enumeration = "search_text_request::RankPreference", tag = "4")]
    pub rank_preference: i32,
    /// The region to search. Setting location would usually yields
    /// better results. Recommended to set. This location serves as a bias unless
    /// strict_restriction is set to true, which turns the location to a strict
    /// restriction.
    ///
    /// Deprecated.  Use LocationRestriction or LocationBias instead.
    #[deprecated]
    #[prost(message, optional, tag = "5")]
    pub location: ::core::option::Option<search_text_request::Location>,
    /// The requested place type. Full list of types supported:
    /// <https://developers.google.com/places/supported_types.> Only support one
    /// included type.
    #[prost(string, tag = "6")]
    pub included_type: ::prost::alloc::string::String,
    /// Used to restrict the search to places that are open at a specific time.
    /// open_now marks if a business is currently open.
    #[prost(bool, tag = "7")]
    pub open_now: bool,
    /// \[Deprecated!\]Used to restrict the search to places that are within a
    /// certain price range. This is on a scale of 0 to 4. Set a minimum of 0 or
    /// set a maximum of 4 has no effect on the search results. Min price is
    /// default to 0 and max price is default to 4. Default value will be used if
    /// either min or max is unset.
    #[deprecated]
    #[prost(message, optional, tag = "8")]
    pub price_range: ::core::option::Option<Int32Range>,
    /// Filter out results whose average user rating is strictly less than this
    /// limit. A valid value must be an float between 0 and 5 (inclusively) at a
    /// 0.5 cadence i.e. `\[0, 0.5, 1.0, ... , 5.0\]` inclusively. This is to keep
    /// parity with LocalRefinement_UserRating. The input rating will round up to
    /// the nearest 0.5(ceiling). For instance, a rating of 0.6 will eliminate all
    /// results with a less than 1.0 rating.
    #[prost(double, tag = "9")]
    pub min_rating: f64,
    /// Maximum number of results to return. It must be between 1 and 20,
    /// inclusively. If the number is unset, it falls back to the upper limit. If
    /// the number is set to negative or exceeds the upper limit, an
    /// INVALID_ARGUMENT error is returned.
    #[prost(int32, tag = "10")]
    pub max_result_count: i32,
    /// Used to restrict the search to places that are marked as certain price
    /// levels. Users can choose any combinations of price levels. Default to
    /// select all price levels.
    #[prost(enumeration = "PriceLevel", repeated, tag = "11")]
    pub price_levels: ::prost::alloc::vec::Vec<i32>,
    /// Used to set strict type filtering for included_type. If set to true, only
    /// results of the same type will be returned. Default to false.
    #[prost(bool, tag = "12")]
    pub strict_type_filtering: bool,
    /// The region to search. This location serves as a bias which means results
    /// around given location might be returned. Cannot be set along with
    /// location_restriction.
    #[prost(message, optional, tag = "13")]
    pub location_bias: ::core::option::Option<search_text_request::LocationBias>,
    /// The region to search. This location serves as a restriction which means
    /// results outside given location will not be returned. Cannot be set along
    /// with location_bias.
    #[prost(message, optional, tag = "14")]
    pub location_restriction: ::core::option::Option<
        search_text_request::LocationRestriction,
    >,
}
/// Nested message and enum types in `SearchTextRequest`.
pub mod search_text_request {
    /// The region to search.
    ///
    /// Deprecated. Use LocationRestriction or LocationBias instead.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Location {
        /// Make location field a strict restriction and filter out POIs outside of
        /// the given location. If location type field is unset this field will have
        /// no effect.
        #[prost(bool, tag = "2")]
        pub strict_restriction: bool,
        #[prost(oneof = "location::Type", tags = "1")]
        pub r#type: ::core::option::Option<location::Type>,
    }
    /// Nested message and enum types in `Location`.
    pub mod location {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Type {
            /// A rectangle box defined by northeast and southwest corner.
            #[prost(message, tag = "1")]
            Rectangle(super::super::super::super::super::geo::r#type::Viewport),
        }
    }
    /// The region to search. This location serves as a bias which means results
    /// around given location might be returned.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct LocationBias {
        #[prost(oneof = "location_bias::Type", tags = "1, 2")]
        pub r#type: ::core::option::Option<location_bias::Type>,
    }
    /// Nested message and enum types in `LocationBias`.
    pub mod location_bias {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Type {
            /// A rectangle box defined by northeast and southwest corner.
            #[prost(message, tag = "1")]
            Rectangle(super::super::super::super::super::geo::r#type::Viewport),
            /// A circle defined by center point and radius.
            #[prost(message, tag = "2")]
            Circle(super::super::Circle),
        }
    }
    /// The region to search. This location serves as a restriction which means
    /// results outside given location will not be returned.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct LocationRestriction {
        #[prost(oneof = "location_restriction::Type", tags = "1")]
        pub r#type: ::core::option::Option<location_restriction::Type>,
    }
    /// Nested message and enum types in `LocationRestriction`.
    pub mod location_restriction {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Type {
            /// A rectangle box defined by northeast and southwest corner.
            #[prost(message, tag = "1")]
            Rectangle(super::super::super::super::super::geo::r#type::Viewport),
        }
    }
    /// How results will be ranked in the response.
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum RankPreference {
        /// RankPreference value not set. Will default to DISTANCE.
        Unspecified = 0,
        /// Ranks results by distance.
        Distance = 1,
        /// Ranks results by relevance. Sort order determined by normal ranking
        /// stack. See SortRefinement::RELEVANCE.
        Relevance = 2,
    }
    impl RankPreference {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                RankPreference::Unspecified => "RANK_PREFERENCE_UNSPECIFIED",
                RankPreference::Distance => "DISTANCE",
                RankPreference::Relevance => "RELEVANCE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "RANK_PREFERENCE_UNSPECIFIED" => Some(Self::Unspecified),
                "DISTANCE" => Some(Self::Distance),
                "RELEVANCE" => Some(Self::Relevance),
                _ => None,
            }
        }
    }
}
/// Response proto for SearchText.
///
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchTextResponse {
    /// A list of places that meet the user's text search criteria.
    #[prost(message, repeated, tag = "1")]
    pub places: ::prost::alloc::vec::Vec<Place>,
}
/// Generated client implementations.
pub mod places_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Service definition for the Places API.
    #[derive(Debug, Clone)]
    pub struct PlacesClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl PlacesClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> PlacesClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> PlacesClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            PlacesClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        /// Text query based place search.
        pub async fn search_text(
            &mut self,
            request: impl tonic::IntoRequest<super::SearchTextRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SearchTextResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.maps.places.v1.Places/SearchText",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("google.maps.places.v1.Places", "SearchText"));
            self.inner.unary(req, path, codec).await
        }
    }
}
