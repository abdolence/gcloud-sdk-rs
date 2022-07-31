///  Listing categories.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Category {
    Unspecified = 0,
    Others = 1,
    AdvertisingAndMarketing = 2,
    Commerce = 3,
    ClimateAndEnvironment = 4,
    Demographics = 5,
    Economics = 6,
    Education = 7,
    Energy = 8,
    Financial = 9,
    Gaming = 10,
    Geospatial = 11,
    HealthcareAndLifeScience = 12,
    Media = 13,
    PublicSector = 14,
    Retail = 15,
    Sports = 16,
    ScienceAndResearch = 17,
    TransportationAndLogistics = 18,
    TravelAndTourism = 19,
}
impl Category {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Category::Unspecified => "CATEGORY_UNSPECIFIED",
            Category::Others => "CATEGORY_OTHERS",
            Category::AdvertisingAndMarketing => "CATEGORY_ADVERTISING_AND_MARKETING",
            Category::Commerce => "CATEGORY_COMMERCE",
            Category::ClimateAndEnvironment => "CATEGORY_CLIMATE_AND_ENVIRONMENT",
            Category::Demographics => "CATEGORY_DEMOGRAPHICS",
            Category::Economics => "CATEGORY_ECONOMICS",
            Category::Education => "CATEGORY_EDUCATION",
            Category::Energy => "CATEGORY_ENERGY",
            Category::Financial => "CATEGORY_FINANCIAL",
            Category::Gaming => "CATEGORY_GAMING",
            Category::Geospatial => "CATEGORY_GEOSPATIAL",
            Category::HealthcareAndLifeScience => "CATEGORY_HEALTHCARE_AND_LIFE_SCIENCE",
            Category::Media => "CATEGORY_MEDIA",
            Category::PublicSector => "CATEGORY_PUBLIC_SECTOR",
            Category::Retail => "CATEGORY_RETAIL",
            Category::Sports => "CATEGORY_SPORTS",
            Category::ScienceAndResearch => "CATEGORY_SCIENCE_AND_RESEARCH",
            Category::TransportationAndLogistics => "CATEGORY_TRANSPORTATION_AND_LOGISTICS",
            Category::TravelAndTourism => "CATEGORY_TRAVEL_AND_TOURISM",
        }
    }
}
