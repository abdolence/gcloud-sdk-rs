/// The price represented as a number and currency.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Price {
    /// The price represented as a number in micros (1 million micros is an
    /// equivalent to one's currency standard unit, for example, 1 USD = 1000000
    /// micros).
    #[prost(int64, optional, tag = "1")]
    pub amount_micros: ::core::option::Option<i64>,
    /// The currency of the price using three-letter acronyms according to [ISO
    /// 4217](<http://en.wikipedia.org/wiki/ISO_4217>).
    #[prost(string, optional, tag = "2")]
    pub currency_code: ::core::option::Option<::prost::alloc::string::String>,
}
/// A message that represents custom attributes. Exactly one of `value` or
/// `group_values` must not be empty.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomAttribute {
    /// The name of the attribute.
    #[prost(string, optional, tag = "1")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    /// The value of the attribute. If `value` is not empty, `group_values` must be
    /// empty.
    #[prost(string, optional, tag = "2")]
    pub value: ::core::option::Option<::prost::alloc::string::String>,
    /// Subattributes within this attribute group.  If
    /// `group_values` is not empty, `value` must be empty.
    #[prost(message, repeated, tag = "3")]
    pub group_values: ::prost::alloc::vec::Vec<CustomAttribute>,
}
/// Destinations available for a product.
///
/// Destinations are used in Merchant Center to allow you to control where the
/// products from your data feed should be displayed.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Destination {
    /// Not specified.
    Unspecified = 0,
    /// Shopping ads.
    ShoppingAds = 1,
    /// Local inventory ads.
    LocalInventoryAds = 2,
    /// Free listings.
    FreeListings = 3,
    /// Free local product listings.
    FreeLocalListings = 4,
}
impl Destination {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Destination::Unspecified => "DESTINATION_UNSPECIFIED",
            Destination::ShoppingAds => "SHOPPING_ADS",
            Destination::LocalInventoryAds => "LOCAL_INVENTORY_ADS",
            Destination::FreeListings => "FREE_LISTINGS",
            Destination::FreeLocalListings => "FREE_LOCAL_LISTINGS",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "DESTINATION_UNSPECIFIED" => Some(Self::Unspecified),
            "SHOPPING_ADS" => Some(Self::ShoppingAds),
            "LOCAL_INVENTORY_ADS" => Some(Self::LocalInventoryAds),
            "FREE_LISTINGS" => Some(Self::FreeListings),
            "FREE_LOCAL_LISTINGS" => Some(Self::FreeLocalListings),
            _ => None,
        }
    }
}
