/// An intended audience of the [Product][google.cloud.retail.v2alpha.Product]
/// for whom it's sold.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Audience {
    /// The genders of the audience. Strongly encouraged to use the standard
    /// values: "male", "female", "unisex".
    ///
    /// At most 5 values are allowed. Each value must be a UTF-8 encoded string
    /// with a length limit of 128 characters. Otherwise, an INVALID_ARGUMENT error
    /// is returned.
    ///
    /// Google Merchant Center property
    /// [gender](https://support.google.com/merchants/answer/6324479). Schema.org
    /// property
    /// [Product.audience.suggestedGender](https://schema.org/suggestedGender).
    #[prost(string, repeated, tag = "1")]
    pub genders: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The age groups of the audience. Strongly encouraged to use the standard
    /// values: "newborn" (up to 3 months old), "infant" (3–12 months old),
    /// "toddler" (1–5 years old), "kids" (5–13 years old), "adult" (typically
    /// teens or older).
    ///
    /// At most 5 values are allowed. Each value must be a UTF-8 encoded string
    /// with a length limit of 128 characters. Otherwise, an INVALID_ARGUMENT error
    /// is returned.
    ///
    /// Google Merchant Center property
    /// [age_group](https://support.google.com/merchants/answer/6324463).
    /// Schema.org property
    /// [Product.audience.suggestedMinAge](https://schema.org/suggestedMinAge) and
    /// [Product.audience.suggestedMaxAge](https://schema.org/suggestedMaxAge).
    #[prost(string, repeated, tag = "2")]
    pub age_groups: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// The color information of a [Product][google.cloud.retail.v2alpha.Product].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ColorInfo {
    /// The standard color families. Strongly recommended to use the following
    /// standard color groups: "Red", "Pink", "Orange", "Yellow", "Purple",
    /// "Green", "Cyan", "Blue", "Brown", "White", "Gray", "Black" and
    /// "Mixed". Normally it is expected to have only 1 color family. May consider
    /// using single "Mixed" instead of multiple values.
    ///
    /// A maximum of 5 values are allowed. Each value must be a UTF-8 encoded
    /// string with a length limit of 128 characters. Otherwise, an
    /// INVALID_ARGUMENT error is returned.
    ///
    /// Google Merchant Center property
    /// [color](https://support.google.com/merchants/answer/6324487). Schema.org
    /// property [Product.color](https://schema.org/color).
    #[prost(string, repeated, tag = "1")]
    pub color_families: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The color display names, which may be different from standard color family
    /// names, such as the color aliases used in the website frontend. Normally
    /// it is expected to have only 1 color. May consider using single "Mixed"
    /// instead of multiple values.
    ///
    /// A maximum of 5 colors are allowed. Each value must be a UTF-8 encoded
    /// string with a length limit of 128 characters. Otherwise, an
    /// INVALID_ARGUMENT error is returned.
    ///
    /// Google Merchant Center property
    /// [color](https://support.google.com/merchants/answer/6324487). Schema.org
    /// property [Product.color](https://schema.org/color).
    #[prost(string, repeated, tag = "2")]
    pub colors: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// A custom attribute that is not explicitly modeled in
/// [Product][google.cloud.retail.v2alpha.Product].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomAttribute {
    /// The textual values of this custom attribute. For example, `["yellow",
    /// "green"]` when the key is "color".
    ///
    /// At most 400 values are allowed. Empty values are not allowed. Each value
    /// must be a UTF-8 encoded string with a length limit of 256 characters.
    /// Otherwise, an INVALID_ARGUMENT error is returned.
    ///
    /// Exactly one of [text][google.cloud.retail.v2alpha.CustomAttribute.text] or
    /// [numbers][google.cloud.retail.v2alpha.CustomAttribute.numbers] should be
    /// set. Otherwise, an INVALID_ARGUMENT error is returned.
    #[prost(string, repeated, tag = "1")]
    pub text: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The numerical values of this custom attribute. For example, `[2.3, 15.4]`
    /// when the key is "lengths_cm".
    ///
    /// At most 400 values are allowed.Otherwise, an INVALID_ARGUMENT error is
    /// returned.
    ///
    /// Exactly one of [text][google.cloud.retail.v2alpha.CustomAttribute.text] or
    /// [numbers][google.cloud.retail.v2alpha.CustomAttribute.numbers] should be
    /// set. Otherwise, an INVALID_ARGUMENT error is returned.
    #[prost(double, repeated, tag = "2")]
    pub numbers: ::prost::alloc::vec::Vec<f64>,
    /// If true, custom attribute values are searchable by text queries in
    /// [SearchService.Search][google.cloud.retail.v2alpha.SearchService.Search].
    ///
    /// This field is ignored in a
    /// [UserEvent][google.cloud.retail.v2alpha.UserEvent].
    ///
    /// Only set if type [text][google.cloud.retail.v2alpha.CustomAttribute.text]
    /// is set. Otherwise, a INVALID_ARGUMENT error is returned.
    #[prost(bool, optional, tag = "3")]
    pub searchable: ::core::option::Option<bool>,
    /// If true, custom attribute values are indexed, so that it can be filtered,
    /// faceted or boosted in
    /// [SearchService.Search][google.cloud.retail.v2alpha.SearchService.Search].
    ///
    /// This field is ignored in a
    /// [UserEvent][google.cloud.retail.v2alpha.UserEvent].
    ///
    /// See
    /// [SearchRequest.filter][google.cloud.retail.v2alpha.SearchRequest.filter],
    /// [SearchRequest.facet_specs][google.cloud.retail.v2alpha.SearchRequest.facet_specs]
    /// and
    /// [SearchRequest.boost_spec][google.cloud.retail.v2alpha.SearchRequest.boost_spec]
    /// for more details.
    #[prost(bool, optional, tag = "4")]
    pub indexable: ::core::option::Option<bool>,
}
/// Fulfillment information, such as the store IDs for in-store pickup or region
/// IDs for different shipping methods.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FulfillmentInfo {
    /// The fulfillment type, including commonly used types (such as pickup in
    /// store and same day delivery), and custom types. Customers have to map
    /// custom types to their display names before rendering UI.
    ///
    /// Supported values:
    ///
    /// * "pickup-in-store"
    /// * "ship-to-store"
    /// * "same-day-delivery"
    /// * "next-day-delivery"
    /// * "custom-type-1"
    /// * "custom-type-2"
    /// * "custom-type-3"
    /// * "custom-type-4"
    /// * "custom-type-5"
    ///
    /// If this field is set to an invalid value other than these, an
    /// INVALID_ARGUMENT error is returned.
    #[prost(string, tag = "1")]
    pub r#type: ::prost::alloc::string::String,
    /// The IDs for this [type][google.cloud.retail.v2alpha.FulfillmentInfo.type],
    /// such as the store IDs for
    /// [FulfillmentInfo.type.pickup-in-store][google.cloud.retail.v2alpha.FulfillmentInfo.type]
    /// or the region IDs for
    /// [FulfillmentInfo.type.same-day-delivery][google.cloud.retail.v2alpha.FulfillmentInfo.type].
    ///
    /// A maximum of 2000 values are allowed. Each value must be a string with a
    /// length limit of 10 characters, matching the pattern [a-zA-Z0-9_-]+, such as
    /// "store1" or "REGION-2". Otherwise, an INVALID_ARGUMENT error is returned.
    #[prost(string, repeated, tag = "2")]
    pub place_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// [Product][google.cloud.retail.v2alpha.Product] thumbnail/detail image.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Image {
    /// Required. URI of the image.
    ///
    /// This field must be a valid UTF-8 encoded URI with a length limit of 5,000
    /// characters. Otherwise, an INVALID_ARGUMENT error is returned.
    ///
    /// Google Merchant Center property
    /// [image_link](https://support.google.com/merchants/answer/6324350).
    /// Schema.org property [Product.image](https://schema.org/image).
    #[prost(string, tag = "1")]
    pub uri: ::prost::alloc::string::String,
    /// Height of the image in number of pixels.
    ///
    /// This field must be nonnegative. Otherwise, an INVALID_ARGUMENT error is
    /// returned.
    #[prost(int32, tag = "2")]
    pub height: i32,
    /// Width of the image in number of pixels.
    ///
    /// This field must be nonnegative. Otherwise, an INVALID_ARGUMENT error is
    /// returned.
    #[prost(int32, tag = "3")]
    pub width: i32,
}
/// A floating point interval.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Interval {
    /// The lower bound of the interval. If neither of the min fields are set, then
    /// the lower bound is negative infinity.
    ///
    /// This field must be not larger than
    /// [max][google.cloud.retail.v2alpha.Interval.max]. Otherwise, an
    /// INVALID_ARGUMENT error is returned.
    #[prost(oneof = "interval::Min", tags = "1, 2")]
    pub min: ::core::option::Option<interval::Min>,
    /// The upper bound of the interval. If neither of the max fields are set, then
    /// the upper bound is positive infinity.
    ///
    /// This field must be not smaller than
    /// [min][google.cloud.retail.v2alpha.Interval.min]. Otherwise, an
    /// INVALID_ARGUMENT error is returned.
    #[prost(oneof = "interval::Max", tags = "3, 4")]
    pub max: ::core::option::Option<interval::Max>,
}
/// Nested message and enum types in `Interval`.
pub mod interval {
    /// The lower bound of the interval. If neither of the min fields are set, then
    /// the lower bound is negative infinity.
    ///
    /// This field must be not larger than
    /// [max][google.cloud.retail.v2alpha.Interval.max]. Otherwise, an
    /// INVALID_ARGUMENT error is returned.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Min {
        /// Inclusive lower bound.
        #[prost(double, tag = "1")]
        Minimum(f64),
        /// Exclusive lower bound.
        #[prost(double, tag = "2")]
        ExclusiveMinimum(f64),
    }
    /// The upper bound of the interval. If neither of the max fields are set, then
    /// the upper bound is positive infinity.
    ///
    /// This field must be not smaller than
    /// [min][google.cloud.retail.v2alpha.Interval.min]. Otherwise, an
    /// INVALID_ARGUMENT error is returned.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Max {
        /// Inclusive upper bound.
        #[prost(double, tag = "3")]
        Maximum(f64),
        /// Exclusive upper bound.
        #[prost(double, tag = "4")]
        ExclusiveMaximum(f64),
    }
}
/// The price information of a [Product][google.cloud.retail.v2alpha.Product].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PriceInfo {
    /// The 3-letter currency code defined in [ISO
    /// 4217](https://www.iso.org/iso-4217-currency-codes.html).
    ///
    /// If this field is an unrecognizable currency code, an INVALID_ARGUMENT
    /// error is returned.
    ///
    /// The
    /// [Product.Type.VARIANT][google.cloud.retail.v2alpha.Product.Type.VARIANT]
    /// [Product][google.cloud.retail.v2alpha.Product]s with the same
    /// [Product.primary_product_id][google.cloud.retail.v2alpha.Product.primary_product_id]
    /// must share the same
    /// [currency_code][google.cloud.retail.v2alpha.PriceInfo.currency_code].
    /// Otherwise, a FAILED_PRECONDITION error is returned.
    #[prost(string, tag = "1")]
    pub currency_code: ::prost::alloc::string::String,
    /// Price of the product.
    ///
    /// Google Merchant Center property
    /// [price](https://support.google.com/merchants/answer/6324371). Schema.org
    /// property [Offer.priceSpecification](https://schema.org/priceSpecification).
    #[prost(float, tag = "2")]
    pub price: f32,
    /// Price of the product without any discount. If zero, by default set to be
    /// the [price][google.cloud.retail.v2alpha.PriceInfo.price].
    #[prost(float, tag = "3")]
    pub original_price: f32,
    /// The costs associated with the sale of a particular product. Used for gross
    /// profit reporting.
    ///
    /// * Profit = [price][google.cloud.retail.v2alpha.PriceInfo.price] -
    /// [cost][google.cloud.retail.v2alpha.PriceInfo.cost]
    ///
    /// Google Merchant Center property
    /// [cost_of_goods_sold](https://support.google.com/merchants/answer/9017895).
    #[prost(float, tag = "4")]
    pub cost: f32,
    /// The timestamp when the [price][google.cloud.retail.v2alpha.PriceInfo.price]
    /// starts to be effective. This can be set as a future timestamp, and the
    /// [price][google.cloud.retail.v2alpha.PriceInfo.price] is only used for
    /// search after
    /// [price_effective_time][google.cloud.retail.v2alpha.PriceInfo.price_effective_time].
    /// If so, the
    /// [original_price][google.cloud.retail.v2alpha.PriceInfo.original_price] must
    /// be set and
    /// [original_price][google.cloud.retail.v2alpha.PriceInfo.original_price] is
    /// used before
    /// [price_effective_time][google.cloud.retail.v2alpha.PriceInfo.price_effective_time].
    ///
    /// Do not set if [price][google.cloud.retail.v2alpha.PriceInfo.price] is
    /// always effective because it will cause additional latency during search.
    #[prost(message, optional, tag = "5")]
    pub price_effective_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The timestamp when the [price][google.cloud.retail.v2alpha.PriceInfo.price]
    /// stops to be effective. The
    /// [price][google.cloud.retail.v2alpha.PriceInfo.price] is used for search
    /// before
    /// [price_expire_time][google.cloud.retail.v2alpha.PriceInfo.price_expire_time].
    /// If this field is set, the
    /// [original_price][google.cloud.retail.v2alpha.PriceInfo.original_price] must
    /// be set and
    /// [original_price][google.cloud.retail.v2alpha.PriceInfo.original_price] is
    /// used after
    /// [price_expire_time][google.cloud.retail.v2alpha.PriceInfo.price_expire_time].
    ///
    /// Do not set if [price][google.cloud.retail.v2alpha.PriceInfo.price] is
    /// always effective because it will cause additional latency during search.
    #[prost(message, optional, tag = "6")]
    pub price_expire_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The price range of all the child
    /// [Product.Type.VARIANT][google.cloud.retail.v2alpha.Product.Type.VARIANT]
    /// [Product][google.cloud.retail.v2alpha.Product]s grouped together on the
    /// [Product.Type.PRIMARY][google.cloud.retail.v2alpha.Product.Type.PRIMARY]
    /// [Product][google.cloud.retail.v2alpha.Product]. Only populated for
    /// [Product.Type.PRIMARY][google.cloud.retail.v2alpha.Product.Type.PRIMARY]
    /// [Product][google.cloud.retail.v2alpha.Product]s.
    ///
    /// Note: This field is OUTPUT_ONLY for
    /// [ProductService.GetProduct][google.cloud.retail.v2alpha.ProductService.GetProduct].
    /// Do not set this field in API requests.
    #[prost(message, optional, tag = "7")]
    pub price_range: ::core::option::Option<price_info::PriceRange>,
}
/// Nested message and enum types in `PriceInfo`.
pub mod price_info {
    /// The price range of all
    /// [variant][google.cloud.retail.v2alpha.Product.Type.VARIANT]
    /// [Product][google.cloud.retail.v2alpha.Product] having the same
    /// [Product.primary_product_id][google.cloud.retail.v2alpha.Product.primary_product_id].
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PriceRange {
        /// The inclusive
        /// [Product.pricing_info.price][google.cloud.retail.v2alpha.PriceInfo.price]
        /// interval of all
        /// [variant][google.cloud.retail.v2alpha.Product.Type.VARIANT]
        /// [Product][google.cloud.retail.v2alpha.Product] having the same
        /// [Product.primary_product_id][google.cloud.retail.v2alpha.Product.primary_product_id].
        #[prost(message, optional, tag = "1")]
        pub price: ::core::option::Option<super::Interval>,
        /// The inclusive
        /// [Product.pricing_info.original_price][google.cloud.retail.v2alpha.PriceInfo.original_price]
        /// internal of all
        /// [variant][google.cloud.retail.v2alpha.Product.Type.VARIANT]
        /// [Product][google.cloud.retail.v2alpha.Product] having the same
        /// [Product.primary_product_id][google.cloud.retail.v2alpha.Product.primary_product_id].
        #[prost(message, optional, tag = "2")]
        pub original_price: ::core::option::Option<super::Interval>,
    }
}
/// The rating of a [Product][google.cloud.retail.v2alpha.Product].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Rating {
    /// The total number of ratings. This value is independent of the value of
    /// [rating_histogram][google.cloud.retail.v2alpha.Rating.rating_histogram].
    ///
    /// This value must be nonnegative. Otherwise, an INVALID_ARGUMENT error is
    /// returned.
    #[prost(int32, tag = "1")]
    pub rating_count: i32,
    /// The average rating of the [Product][google.cloud.retail.v2alpha.Product].
    ///
    /// The rating is scaled at 1-5. Otherwise, an INVALID_ARGUMENT error is
    /// returned.
    #[prost(float, tag = "2")]
    pub average_rating: f32,
    /// List of rating counts per rating value (index = rating - 1). The list is
    /// empty if there is no rating. If the list is non-empty, its size is
    /// always 5. Otherwise, an INVALID_ARGUMENT error is returned.
    ///
    /// For example, [41, 14, 13, 47, 303]. It means that the
    /// [Product][google.cloud.retail.v2alpha.Product] got 41 ratings with 1 star,
    /// 14 ratings with 2 star, and so on.
    #[prost(int32, repeated, tag = "3")]
    pub rating_histogram: ::prost::alloc::vec::Vec<i32>,
}
/// Information of an end user.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserInfo {
    /// Highly recommended for logged-in users. Unique identifier for logged-in
    /// user, such as a user name.
    ///
    /// The field must be a UTF-8 encoded string with a length limit of 128
    /// characters. Otherwise, an INVALID_ARGUMENT error is returned.
    #[prost(string, tag = "1")]
    pub user_id: ::prost::alloc::string::String,
    /// The end user's IP address. Required for getting
    /// [SearchResponse.sponsored_results][google.cloud.retail.v2alpha.SearchResponse.sponsored_results].
    /// This field is used to extract location information for personalization.
    ///
    /// This field must be either an IPv4 address (e.g. "104.133.9.80") or an IPv6
    /// address (e.g. "2001:0db8:85a3:0000:0000:8a2e:0370:7334"). Otherwise, an
    /// INVALID_ARGUMENT error is returned.
    ///
    /// This should not be set when using the JavaScript tag in
    /// [UserEventService.CollectUserEvent][google.cloud.retail.v2alpha.UserEventService.CollectUserEvent]
    /// or if
    /// [direct_user_request][google.cloud.retail.v2alpha.UserInfo.direct_user_request]
    /// is set.
    #[prost(string, tag = "2")]
    pub ip_address: ::prost::alloc::string::String,
    /// User agent as included in the HTTP header. Required for getting
    /// [SearchResponse.sponsored_results][google.cloud.retail.v2alpha.SearchResponse.sponsored_results].
    ///
    /// The field must be a UTF-8 encoded string with a length limit of 1,000
    /// characters. Otherwise, an INVALID_ARGUMENT error is returned.
    ///
    /// This should not be set when using the client side event reporting with
    /// GTM or JavaScript tag in
    /// [UserEventService.CollectUserEvent][google.cloud.retail.v2alpha.UserEventService.CollectUserEvent]
    /// or if
    /// [direct_user_request][google.cloud.retail.v2alpha.UserInfo.direct_user_request]
    /// is set.
    #[prost(string, tag = "3")]
    pub user_agent: ::prost::alloc::string::String,
    /// True if the request is made directly from the end user, in which case the
    /// [ip_address][google.cloud.retail.v2alpha.UserInfo.ip_address] and
    /// [user_agent][google.cloud.retail.v2alpha.UserInfo.user_agent] can be
    /// populated from the HTTP request. This flag should be set only if the API
    /// request is made directly from the end user such as a mobile app (and not if
    /// a gateway or a server is processing and pushing the user events).
    ///
    /// This should not be set when using the JavaScript tag in
    /// [UserEventService.CollectUserEvent][google.cloud.retail.v2alpha.UserEventService.CollectUserEvent].
    #[prost(bool, tag = "4")]
    pub direct_user_request: bool,
}
/// Promotion information.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Promotion {
    /// ID of the promotion. For example, "free gift".
    ///
    /// The value value must be a UTF-8 encoded string with a length limit of 128
    /// characters, and match the pattern: [a-zA-Z][a-zA-Z0-9_]*. For example,
    /// id0LikeThis or ID_1_LIKE_THIS. Otherwise, an INVALID_ARGUMENT error is
    /// returned.
    ///
    /// Google Merchant Center property
    /// [promotion](https://support.google.com/merchants/answer/7050148).
    #[prost(string, tag = "1")]
    pub promotion_id: ::prost::alloc::string::String,
}
/// Product captures all metadata information of items to be recommended or
/// searched.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Product {
    /// Immutable. Full resource name of the product, such as
    /// `projects/*/locations/global/catalogs/default_catalog/branches/default_branch/products/product_id`.
    ///
    /// The branch ID must be "default_branch".
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Immutable. [Product][google.cloud.retail.v2alpha.Product] identifier, which
    /// is the final component of [name][google.cloud.retail.v2alpha.Product.name].
    /// For example, this field is "id_1", if
    /// [name][google.cloud.retail.v2alpha.Product.name] is
    /// `projects/*/locations/global/catalogs/default_catalog/branches/default_branch/products/id_1`.
    ///
    /// This field must be a UTF-8 encoded string with a length limit of 128
    /// characters. Otherwise, an INVALID_ARGUMENT error is returned.
    ///
    /// Google Merchant Center property
    /// [id](https://support.google.com/merchants/answer/6324405). Schema.org
    /// Property [Product.sku](https://schema.org/sku).
    #[prost(string, tag = "2")]
    pub id: ::prost::alloc::string::String,
    /// Immutable. The type of the product. Default to
    /// [Catalog.product_level_config.ingestion_product_type][google.cloud.retail.v2alpha.ProductLevelConfig.ingestion_product_type]
    /// if unset.
    #[prost(enumeration = "product::Type", tag = "3")]
    pub r#type: i32,
    /// Variant group identifier. Must be an
    /// [id][google.cloud.retail.v2alpha.Product.id], with the same parent branch
    /// with this product. Otherwise, an error is thrown.
    ///
    /// For [Type.PRIMARY][google.cloud.retail.v2alpha.Product.Type.PRIMARY]
    /// [Product][google.cloud.retail.v2alpha.Product]s, this field can only be
    /// empty or set to the same value as
    /// [id][google.cloud.retail.v2alpha.Product.id].
    ///
    /// For VARIANT [Product][google.cloud.retail.v2alpha.Product]s, this field
    /// cannot be empty. A maximum of 2,000 products are allowed to share the same
    /// [Type.PRIMARY][google.cloud.retail.v2alpha.Product.Type.PRIMARY]
    /// [Product][google.cloud.retail.v2alpha.Product]. Otherwise, an
    /// INVALID_ARGUMENT error is returned.
    ///
    /// Google Merchant Center Property
    /// [item_group_id](https://support.google.com/merchants/answer/6324507).
    /// Schema.org Property
    /// [Product.inProductGroupWithID](https://schema.org/inProductGroupWithID).
    ///
    /// This field must be enabled before it can be used. [Learn
    /// more](/recommendations-ai/docs/catalog#item-group-id).
    #[prost(string, tag = "4")]
    pub primary_product_id: ::prost::alloc::string::String,
    /// The [id][google.cloud.retail.v2alpha.Product.id] of the collection members
    /// when [type][google.cloud.retail.v2alpha.Product.type] is
    /// [Type.COLLECTION][google.cloud.retail.v2alpha.Product.Type.COLLECTION].
    ///
    /// Should not set it for other types. A maximum of 1000 values are allowed.
    /// Otherwise, an INVALID_ARGUMENT error is return.
    #[prost(string, repeated, tag = "5")]
    pub collection_member_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The Global Trade Item Number (GTIN) of the product.
    ///
    /// This field must be a UTF-8 encoded string with a length limit of 128
    /// characters. Otherwise, an INVALID_ARGUMENT error is returned.
    ///
    /// Google Merchant Center property
    /// [gtin](https://support.google.com/merchants/answer/6324461).
    /// Schema.org property
    /// [Product.isbn](https://schema.org/isbn) or
    /// [Product.gtin8](https://schema.org/gtin8) or
    /// [Product.gtin12](https://schema.org/gtin12) or
    /// [Product.gtin13](https://schema.org/gtin13) or
    /// [Product.gtin14](https://schema.org/gtin14).
    ///
    /// If the value is not a valid GTIN, an INVALID_ARGUMENT error is returned.
    #[prost(string, tag = "6")]
    pub gtin: ::prost::alloc::string::String,
    /// Product categories. This field is repeated for supporting one product
    /// belonging to several parallel categories. Strongly recommended using the
    /// full path for better search / recommendation quality.
    ///
    ///
    /// To represent full path of category, use '>' sign to separate different
    /// hierarchies. If '>' is part of the category name, please replace it with
    /// other character(s).
    ///
    /// For example, if a shoes product belongs to both
    /// ["Shoes & Accessories" -> "Shoes"] and
    /// ["Sports & Fitness" -> "Athletic Clothing" -> "Shoes"], it could be
    /// represented as:
    ///
    ///      "categories": [
    ///        "Shoes & Accessories > Shoes",
    ///        "Sports & Fitness > Athletic Clothing > Shoes"
    ///      ]
    ///
    /// Must be set for
    /// [Type.PRIMARY][google.cloud.retail.v2alpha.Product.Type.PRIMARY]
    /// [Product][google.cloud.retail.v2alpha.Product] otherwise an
    /// INVALID_ARGUMENT error is returned.
    ///
    /// At most 250 values are allowed per
    /// [Product][google.cloud.retail.v2alpha.Product]. Empty values are not
    /// allowed. Each value must be a UTF-8 encoded string with a length limit of
    /// 5,000 characters. Otherwise, an INVALID_ARGUMENT error is returned.
    ///
    /// Google Merchant Center property
    /// [google_product_category][mc_google_product_category]. Schema.org property
    /// [Product.category] (https://schema.org/category).
    ///
    /// [mc_google_product_category]:
    /// https://support.google.com/merchants/answer/6324436
    #[prost(string, repeated, tag = "7")]
    pub categories: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Required. Product title.
    ///
    /// This field must be a UTF-8 encoded string with a length limit of 1,000
    /// characters. Otherwise, an INVALID_ARGUMENT error is returned.
    ///
    /// Google Merchant Center property
    /// [title](https://support.google.com/merchants/answer/6324415). Schema.org
    /// property [Product.name](https://schema.org/name).
    #[prost(string, tag = "8")]
    pub title: ::prost::alloc::string::String,
    /// The brands of the product.
    ///
    /// A maximum of 30 brands are allowed. Each brand must be a UTF-8 encoded
    /// string with a length limit of 1,000 characters. Otherwise, an
    /// INVALID_ARGUMENT error is returned.
    ///
    /// Google Merchant Center property
    /// [brand](https://support.google.com/merchants/answer/6324351). Schema.org
    /// property [Product.brand](https://schema.org/brand).
    #[prost(string, repeated, tag = "9")]
    pub brands: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Product description.
    ///
    /// This field must be a UTF-8 encoded string with a length limit of 5,000
    /// characters. Otherwise, an INVALID_ARGUMENT error is returned.
    ///
    /// Google Merchant Center property
    /// [description](https://support.google.com/merchants/answer/6324468).
    /// schema.org property [Product.description](https://schema.org/description).
    #[prost(string, tag = "10")]
    pub description: ::prost::alloc::string::String,
    /// Language of the title/description and other string attributes. Use language
    /// tags defined by [BCP 47][https://www.rfc-editor.org/rfc/bcp/bcp47.txt].
    ///
    /// For product prediction, this field is ignored and the model automatically
    /// detects the text language. The
    /// [Product][google.cloud.retail.v2alpha.Product] can include text in
    /// different languages, but duplicating
    /// [Product][google.cloud.retail.v2alpha.Product]s to provide text in multiple
    /// languages can result in degraded model performance.
    ///
    /// For product search this field is in use. It defaults to "en-US" if unset.
    #[prost(string, tag = "11")]
    pub language_code: ::prost::alloc::string::String,
    /// Highly encouraged. Extra product attributes to be included. For example,
    /// for products, this could include the store name, vendor, style, color, etc.
    /// These are very strong signals for recommendation model, thus we highly
    /// recommend providing the attributes here.
    ///
    /// Features that can take on one of a limited number of possible values. Two
    /// types of features can be set are:
    ///
    /// Textual features. some examples would be the brand/maker of a product, or
    /// country of a customer. Numerical features. Some examples would be the
    /// height/weight of a product, or age of a customer.
    ///
    /// For example: `{ "vendor": {"text": ["vendor123", "vendor456"]},
    /// "lengths_cm": {"numbers":[2.3, 15.4]}, "heights_cm": {"numbers":[8.1, 6.4]}
    /// }`.
    ///
    /// This field needs to pass all below criteria, otherwise an INVALID_ARGUMENT
    /// error is returned:
    ///
    /// * Max entries count: 200 by default; 100 for
    /// [Type.VARIANT][google.cloud.retail.v2alpha.Product.Type.VARIANT].
    /// * The key must be a UTF-8 encoded string with a length limit of 128
    ///   characters.
    /// * Max indexable entries count: 200 by default; 40 for
    /// [Type.VARIANT][google.cloud.retail.v2alpha.Product.Type.VARIANT].
    /// * Max searchable entries count: 30.
    /// * For indexable attribute, the key must match the pattern:
    ///   [a-zA-Z0-9][a-zA-Z0-9_]*. For example, key0LikeThis or KEY_1_LIKE_THIS.
    #[prost(map = "string, message", tag = "12")]
    pub attributes: ::std::collections::HashMap<::prost::alloc::string::String, CustomAttribute>,
    /// Custom tags associated with the product.
    ///
    /// At most 250 values are allowed per
    /// [Product][google.cloud.retail.v2alpha.Product]. This value must be a UTF-8
    /// encoded string with a length limit of 1,000 characters. Otherwise, an
    /// INVALID_ARGUMENT error is returned.
    ///
    /// This tag can be used for filtering recommendation results by passing the
    /// tag as part of the
    /// [PredictRequest.filter][google.cloud.retail.v2alpha.PredictRequest.filter].
    ///
    /// Google Merchant Center property
    /// [custom_label_0–4](https://support.google.com/merchants/answer/6324473).
    #[prost(string, repeated, tag = "13")]
    pub tags: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Product price and cost information.
    ///
    /// Google Merchant Center property
    /// [price](https://support.google.com/merchants/answer/6324371).
    #[prost(message, optional, tag = "14")]
    pub price_info: ::core::option::Option<PriceInfo>,
    /// The rating of this product.
    #[prost(message, optional, tag = "15")]
    pub rating: ::core::option::Option<Rating>,
    /// The timestamp when this [Product][google.cloud.retail.v2alpha.Product]
    /// becomes available for
    /// [SearchService.Search][google.cloud.retail.v2alpha.SearchService.Search].
    #[prost(message, optional, tag = "18")]
    pub available_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The online availability of the
    /// [Product][google.cloud.retail.v2alpha.Product]. Default to
    /// [Availability.IN_STOCK][google.cloud.retail.v2alpha.Product.Availability.IN_STOCK].
    ///
    /// Google Merchant Center Property
    /// [availability](https://support.google.com/merchants/answer/6324448).
    /// Schema.org Property [Offer.availability](https://schema.org/availability).
    #[prost(enumeration = "product::Availability", tag = "19")]
    pub availability: i32,
    /// The available quantity of the item.
    #[prost(message, optional, tag = "20")]
    pub available_quantity: ::core::option::Option<i32>,
    /// Fulfillment information, such as the store IDs for in-store pickup or
    /// region IDs for different shipping methods.
    ///
    /// All the elements must have distinct
    /// [FulfillmentInfo.type][google.cloud.retail.v2alpha.FulfillmentInfo.type].
    /// Otherwise, an INVALID_ARGUMENT error is returned.
    #[prost(message, repeated, tag = "21")]
    pub fulfillment_info: ::prost::alloc::vec::Vec<FulfillmentInfo>,
    /// Canonical URL directly linking to the product detail page.
    ///
    /// It is strongly recommended to provide a valid uri for the product,
    /// otherwise the service performance could be significantly degraded.
    ///
    /// This field must be a UTF-8 encoded string with a length limit of 5,000
    /// characters. Otherwise, an INVALID_ARGUMENT error is returned.
    ///
    /// Google Merchant Center property
    /// [link](https://support.google.com/merchants/answer/6324416). Schema.org
    /// property [Offer.url](https://schema.org/url).
    #[prost(string, tag = "22")]
    pub uri: ::prost::alloc::string::String,
    /// Product images for the product.Highly recommended to put the main image
    /// to the first.
    ///
    /// A maximum of 300 images are allowed.
    ///
    /// Google Merchant Center property
    /// [image_link](https://support.google.com/merchants/answer/6324350).
    /// Schema.org property [Product.image](https://schema.org/image).
    #[prost(message, repeated, tag = "23")]
    pub images: ::prost::alloc::vec::Vec<Image>,
    /// The target group associated with a given audience (e.g. male, veterans,
    /// car owners, musicians, etc.) of the product.
    #[prost(message, optional, tag = "24")]
    pub audience: ::core::option::Option<Audience>,
    /// The color of the product.
    ///
    /// Google Merchant Center property
    /// [color](https://support.google.com/merchants/answer/6324487). Schema.org
    /// property [Product.color](https://schema.org/color).
    #[prost(message, optional, tag = "25")]
    pub color_info: ::core::option::Option<ColorInfo>,
    /// The size of the product. To represent different size systems or size types,
    /// consider using this format: [[[size_system:]size_type:]size_value].
    ///
    /// For example, in "US:MENS:M", "US" represents size system; "MENS" represents
    /// size type; "M" represents size value. In "GIRLS:27", size system is empty;
    /// "GIRLS" represents size type; "27" represents size value. In "32 inches",
    /// both size system and size type are empty, while size value is "32 inches".
    ///
    /// A maximum of 20 values are allowed per
    /// [Product][google.cloud.retail.v2alpha.Product]. Each value must be a UTF-8
    /// encoded string with a length limit of 128 characters. Otherwise, an
    /// INVALID_ARGUMENT error is returned.
    ///
    /// Google Merchant Center property
    /// [size](https://support.google.com/merchants/answer/6324492),
    /// [size_type](https://support.google.com/merchants/answer/6324497) and
    /// [size_system](https://support.google.com/merchants/answer/6324502).
    /// Schema.org property [Product.size](https://schema.org/size).
    #[prost(string, repeated, tag = "26")]
    pub sizes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The material of the product. For example, "leather", "wooden".
    ///
    /// A maximum of 5 values are allowed. Each value must be a UTF-8 encoded
    /// string with a length limit of 128 characters. Otherwise, an
    /// INVALID_ARGUMENT error is returned.
    ///
    /// Google Merchant Center property
    /// [material](https://support.google.com/merchants/answer/6324410). Schema.org
    /// property [Product.material](https://schema.org/material).
    #[prost(string, repeated, tag = "27")]
    pub materials: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The pattern or graphic print of the product. For example, "striped", "polka
    /// dot", "paisley".
    ///
    /// A maximum of 5 values are allowed per
    /// [Product][google.cloud.retail.v2alpha.Product]. Each value must be a UTF-8
    /// encoded string with a length limit of 128 characters. Otherwise, an
    /// INVALID_ARGUMENT error is returned.
    ///
    /// Google Merchant Center property
    /// [pattern](https://support.google.com/merchants/answer/6324483). Schema.org
    /// property [Product.pattern](https://schema.org/pattern).
    #[prost(string, repeated, tag = "28")]
    pub patterns: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The condition of the product. Strongly encouraged to use the standard
    /// values: "new", "refurbished", "used".
    ///
    /// A maximum of 5 values are allowed per
    /// [Product][google.cloud.retail.v2alpha.Product]. Each value must be a UTF-8
    /// encoded string with a length limit of 128 characters. Otherwise, an
    /// INVALID_ARGUMENT error is returned.
    ///
    /// Google Merchant Center property
    /// [condition](https://support.google.com/merchants/answer/6324469).
    /// Schema.org property
    /// [Offer.itemCondition](https://schema.org/itemCondition).
    #[prost(string, repeated, tag = "29")]
    pub conditions: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The promotions applied to the product. A maximum of 10 values are allowed
    /// per [Product][google.cloud.retail.v2alpha.Product].
    #[prost(message, repeated, tag = "34")]
    pub promotions: ::prost::alloc::vec::Vec<Promotion>,
    /// The timestamp when the product is published by the retailer for the first
    /// time, which indicates the freshness of the products. Note that this field
    /// is different from
    /// [available_time][google.cloud.retail.v2alpha.Product.available_time], given
    /// it purely describes product freshness regardless of when it is available on
    /// search and recommendation.
    #[prost(message, optional, tag = "33")]
    pub publish_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Indicates which fields in the
    /// [Product][google.cloud.retail.v2alpha.Product]s are returned in
    /// [SearchResponse][google.cloud.retail.v2alpha.SearchResponse].
    ///
    /// Supported fields for all [type][google.cloud.retail.v2alpha.Product.type]s:
    ///
    /// * [audience][google.cloud.retail.v2alpha.Product.audience]
    /// * [availability][google.cloud.retail.v2alpha.Product.availability]
    /// * [brands][google.cloud.retail.v2alpha.Product.brands]
    /// * [color_info][google.cloud.retail.v2alpha.Product.color_info]
    /// * [conditions][google.cloud.retail.v2alpha.Product.conditions]
    /// * [gtin][google.cloud.retail.v2alpha.Product.gtin]
    /// * [materials][google.cloud.retail.v2alpha.Product.materials]
    /// * [name][google.cloud.retail.v2alpha.Product.name]
    /// * [patterns][google.cloud.retail.v2alpha.Product.patterns]
    /// * [price_info][google.cloud.retail.v2alpha.Product.price_info]
    /// * [rating][google.cloud.retail.v2alpha.Product.rating]
    /// * [sizes][google.cloud.retail.v2alpha.Product.sizes]
    /// * [title][google.cloud.retail.v2alpha.Product.title]
    /// * [uri][google.cloud.retail.v2alpha.Product.uri]
    ///
    /// Supported fields only for
    /// [Type.PRIMARY][google.cloud.retail.v2alpha.Product.Type.PRIMARY] and
    /// [Type.COLLECTION][google.cloud.retail.v2alpha.Product.Type.COLLECTION]:
    ///
    /// * [categories][google.cloud.retail.v2alpha.Product.categories]
    /// * [description][google.cloud.retail.v2alpha.Product.description]
    /// * [images][google.cloud.retail.v2alpha.Product.images]
    ///
    /// Supported fields only for
    /// [Type.VARIANT][google.cloud.retail.v2alpha.Product.Type.VARIANT]:
    ///
    /// * Only the first image in
    /// [images][google.cloud.retail.v2alpha.Product.images]
    ///
    /// To mark [attributes][google.cloud.retail.v2alpha.Product.attributes] as
    /// retrievable, include paths of the form "attributes.key" where "key" is the
    /// key of a custom attribute, as specified in
    /// [attributes][google.cloud.retail.v2alpha.Product.attributes].
    ///
    /// For [Type.PRIMARY][google.cloud.retail.v2alpha.Product.Type.PRIMARY] and
    /// [Type.COLLECTION][google.cloud.retail.v2alpha.Product.Type.COLLECTION], the
    /// following fields are always returned in
    /// [SearchResponse][google.cloud.retail.v2alpha.SearchResponse] by default:
    ///
    /// * [name][google.cloud.retail.v2alpha.Product.name]
    ///
    /// For [Type.VARIANT][google.cloud.retail.v2alpha.Product.Type.VARIANT], the
    /// following fields are always returned in by default:
    ///
    /// * [name][google.cloud.retail.v2alpha.Product.name]
    /// * [color_info][google.cloud.retail.v2alpha.Product.color_info]
    ///
    /// Maximum number of paths is 20. Otherwise, an INVALID_ARGUMENT error is
    /// returned.
    #[prost(message, optional, tag = "30")]
    pub retrievable_fields: ::core::option::Option<::prost_types::FieldMask>,
    /// Output only. Product variants grouped together on primary product which
    /// share similar product attributes. It's automatically grouped by
    /// [primary_product_id][google.cloud.retail.v2alpha.Product.primary_product_id]
    /// for all the product variants. Only populated for
    /// [Type.PRIMARY][google.cloud.retail.v2alpha.Product.Type.PRIMARY]
    /// [Product][google.cloud.retail.v2alpha.Product]s.
    ///
    /// Note: This field is OUTPUT_ONLY for
    /// [ProductService.GetProduct][google.cloud.retail.v2alpha.ProductService.GetProduct].
    /// Do not set this field in API requests.
    #[prost(message, repeated, tag = "31")]
    pub variants: ::prost::alloc::vec::Vec<Product>,
    #[prost(oneof = "product::Expiration", tags = "16, 17")]
    pub expiration: ::core::option::Option<product::Expiration>,
}
/// Nested message and enum types in `Product`.
pub mod product {
    /// The type of this product.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Type {
        /// Default value. Default to
        /// [Catalog.product_level_config.ingestion_product_type][google.cloud.retail.v2alpha.ProductLevelConfig.ingestion_product_type]
        /// if unset.
        Unspecified = 0,
        /// The primary type.
        ///
        /// As the primary unit for predicting, indexing and search serving, a
        /// [Type.PRIMARY][google.cloud.retail.v2alpha.Product.Type.PRIMARY]
        /// [Product][google.cloud.retail.v2alpha.Product] is grouped with multiple
        /// [Type.VARIANT][google.cloud.retail.v2alpha.Product.Type.VARIANT]
        /// [Product][google.cloud.retail.v2alpha.Product]s.
        Primary = 1,
        /// The variant type.
        ///
        /// [Type.VARIANT][google.cloud.retail.v2alpha.Product.Type.VARIANT]
        /// [Product][google.cloud.retail.v2alpha.Product]s usually share some common
        /// attributes on the same
        /// [Type.PRIMARY][google.cloud.retail.v2alpha.Product.Type.PRIMARY]
        /// [Product][google.cloud.retail.v2alpha.Product]s, but they have variant
        /// attributes like different colors, sizes and prices, etc.
        Variant = 2,
        /// The collection type. Collection products are bundled
        /// [Type.PRIMARY][google.cloud.retail.v2alpha.Product.Type.PRIMARY]
        /// [Product][google.cloud.retail.v2alpha.Product]s or
        /// [Type.VARIANT][google.cloud.retail.v2alpha.Product.Type.VARIANT]
        /// [Product][google.cloud.retail.v2alpha.Product]s that are sold together,
        /// such as a jewelry set with necklaces, earrings and rings, etc.
        Collection = 3,
    }
    /// Product availability. If this field is unspecified, the product is
    /// assumed to be in stock.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Availability {
        /// Default product availability. Default to
        /// [Availability.IN_STOCK][google.cloud.retail.v2alpha.Product.Availability.IN_STOCK]
        /// if unset.
        Unspecified = 0,
        /// Product in stock.
        InStock = 1,
        /// Product out of stock.
        OutOfStock = 2,
        /// Product that is in pre-order state.
        Preorder = 3,
        /// Product that is back-ordered (i.e. temporarily out of stock).
        Backorder = 4,
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Expiration {
        /// The timestamp when this product becomes unavailable for
        /// [SearchService.Search][google.cloud.retail.v2alpha.SearchService.Search].
        ///
        /// If it is set, the [Product][google.cloud.retail.v2alpha.Product] is not
        /// available for
        /// [SearchService.Search][google.cloud.retail.v2alpha.SearchService.Search]
        /// after [expire_time][google.cloud.retail.v2alpha.Product.expire_time].
        /// However, the product can still be retrieved by
        /// [ProductService.GetProduct][google.cloud.retail.v2alpha.ProductService.GetProduct]
        /// and
        /// [ProductService.ListProducts][google.cloud.retail.v2alpha.ProductService.ListProducts].
        ///
        /// Google Merchant Center property
        /// [expiration_date](https://support.google.com/merchants/answer/6324499).
        #[prost(message, tag = "16")]
        ExpireTime(::prost_types::Timestamp),
        /// Input only. The TTL (time to live) of the product.
        ///
        /// If it is set,
        /// [expire_time][google.cloud.retail.v2alpha.Product.expire_time] is set as
        /// current timestamp plus [ttl][google.cloud.retail.v2alpha.Product.ttl].
        /// The derived
        /// [expire_time][google.cloud.retail.v2alpha.Product.expire_time] is
        /// returned in the output and [ttl][google.cloud.retail.v2alpha.Product.ttl]
        /// is left blank when retrieving the
        /// [Product][google.cloud.retail.v2alpha.Product].
        ///
        /// If it is set, the product is not available for
        /// [SearchService.Search][google.cloud.retail.v2alpha.SearchService.Search]
        /// after current timestamp plus
        /// [ttl][google.cloud.retail.v2alpha.Product.ttl]. However, the product can
        /// still be retrieved by
        /// [ProductService.GetProduct][google.cloud.retail.v2alpha.ProductService.GetProduct]
        /// and
        /// [ProductService.ListProducts][google.cloud.retail.v2alpha.ProductService.ListProducts].
        #[prost(message, tag = "17")]
        Ttl(::prost_types::Duration),
    }
}
/// UserEvent captures all metadata information Retail API needs to know about
/// how end users interact with customers' website.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserEvent {
    /// Required. User event type. Allowed values are:
    ///
    /// * `add-to-cart`: Products being added to cart.
    /// * `category-page-view`: Special pages such as sale or promotion pages
    ///   viewed.
    /// * `completion`: Completion query result showed/clicked.
    /// * `detail-page-view`: Products detail page viewed.
    /// * `home-page-view`: Homepage viewed.
    /// * `promotion-offered`: Promotion is offered to a user.
    /// * `promotion-not-offered`: Promotion is not offered to a user.
    /// * `purchase-complete`: User finishing a purchase.
    /// * `search`: Product search.
    /// * `shopping-cart-page-view`: User viewing a shopping cart.
    #[prost(string, tag = "1")]
    pub event_type: ::prost::alloc::string::String,
    /// Required. A unique identifier for tracking visitors.
    ///
    /// For example, this could be implemented with an HTTP cookie, which should be
    /// able to uniquely identify a visitor on a single device. This unique
    /// identifier should not change if the visitor log in/out of the website.
    ///
    /// The field must be a UTF-8 encoded string with a length limit of 128
    /// characters. Otherwise, an INVALID_ARGUMENT error is returned.
    ///
    /// The field should not contain PII or user-data. We recommend to use Google
    /// Analystics [Client
    /// ID](https://developers.google.com/analytics/devguides/collection/analyticsjs/field-reference#clientId)
    /// for this field.
    #[prost(string, tag = "2")]
    pub visitor_id: ::prost::alloc::string::String,
    /// A unique identifier for tracking a visitor session with a length limit of
    /// 128 bytes. A session is an aggregation of an end user behavior in a time
    /// span.
    ///
    /// A general guideline to populate the sesion_id:
    /// 1. If user has no activity for 30 min, a new session_id should be assigned.
    /// 2. The session_id should be unique across users, suggest use uuid or add
    /// visitor_id as prefix.
    #[prost(string, tag = "21")]
    pub session_id: ::prost::alloc::string::String,
    /// Only required for
    /// [UserEventService.ImportUserEvents][google.cloud.retail.v2alpha.UserEventService.ImportUserEvents]
    /// method. Timestamp of when the user event happened.
    #[prost(message, optional, tag = "3")]
    pub event_time: ::core::option::Option<::prost_types::Timestamp>,
    /// A list of identifiers for the independent experiment groups this user event
    /// belongs to. This is used to distinguish between user events associated with
    /// different experiment setups (e.g. using Retail API, using different
    /// recommendation models).
    #[prost(string, repeated, tag = "4")]
    pub experiment_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Highly recommended for user events that are the result of
    /// [PredictionService.Predict][google.cloud.retail.v2alpha.PredictionService.Predict].
    /// This field enables accurate attribution of recommendation model
    /// performance.
    ///
    /// The value must be a valid
    /// [PredictResponse.attribution_token][google.cloud.retail.v2alpha.PredictResponse.attribution_token]
    /// for user events that are the result of
    /// [PredictionService.Predict][google.cloud.retail.v2alpha.PredictionService.Predict].
    /// The value must be a valid
    /// [SearchResponse.attribution_token][google.cloud.retail.v2alpha.SearchResponse.attribution_token]
    /// for user events that are the result of
    /// [SearchService.Search][google.cloud.retail.v2alpha.SearchService.Search].
    ///
    /// This token enables us to accurately attribute page view or purchase back to
    /// the event and the particular predict response containing this
    /// clicked/purchased product. If user clicks on product K in the
    /// recommendation results, pass
    /// [PredictResponse.attribution_token][google.cloud.retail.v2alpha.PredictResponse.attribution_token]
    /// as a URL parameter to product K's page. When recording events on product
    /// K's page, log the
    /// [PredictResponse.attribution_token][google.cloud.retail.v2alpha.PredictResponse.attribution_token]
    /// to this field.
    #[prost(string, tag = "5")]
    pub attribution_token: ::prost::alloc::string::String,
    /// The main product details related to the event.
    ///
    /// This field is required for the following event types:
    ///
    /// * `add-to-cart`
    /// * `detail-page-view`
    /// * `purchase-complete`
    ///
    /// In a `search` event, this field represents the products returned to the end
    /// user on the current page (the end user may have not finished broswing the
    /// whole page yet). When a new page is returned to the end user, after
    /// pagination/filtering/ordering even for the same query, a new `search` event
    /// with different
    /// [product_details][google.cloud.retail.v2alpha.UserEvent.product_details] is
    /// desired. The end user may have not finished broswing the whole page yet.
    #[prost(message, repeated, tag = "6")]
    pub product_details: ::prost::alloc::vec::Vec<ProductDetail>,
    /// The main completion details related to the event.
    ///
    /// In a `completion` event, this field represents the completions returned to
    /// the end user and the clicked completion by the end user. In a `search`
    /// event, it represents the search event happens after clicking completion.
    #[prost(message, optional, tag = "22")]
    pub completion_detail: ::core::option::Option<CompletionDetail>,
    /// Extra user event features to include in the recommendation model.
    ///
    /// The key must be a UTF-8 encoded string with a length limit of 5,000
    /// characters. Otherwise, an INVALID_ARGUMENT error is returned.
    ///
    /// For product recommendation, an example of extra user information is
    /// traffic_channel, i.e. how user arrives at the site. Users can arrive
    /// at the site by coming to the site directly, or coming through Google
    /// search, and etc.
    #[prost(map = "string, message", tag = "7")]
    pub attributes: ::std::collections::HashMap<::prost::alloc::string::String, CustomAttribute>,
    /// The id or name of the associated shopping cart. This id is used
    /// to associate multiple items added or present in the cart before purchase.
    ///
    /// This can only be set for `add-to-cart`, `purchase-complete`, or
    /// `shopping-cart-page-view` events.
    #[prost(string, tag = "8")]
    pub cart_id: ::prost::alloc::string::String,
    /// A transaction represents the entire purchase transaction.
    ///
    /// Required for `purchase-complete` events. Other event types should not set
    /// this field. Otherwise, an INVALID_ARGUMENT error is returned.
    #[prost(message, optional, tag = "9")]
    pub purchase_transaction: ::core::option::Option<PurchaseTransaction>,
    /// The user's search query.
    ///
    /// See [SearchRequest.query][google.cloud.retail.v2alpha.SearchRequest.query]
    /// for definition.
    ///
    /// The value must be a UTF-8 encoded string with a length limit of 5,000
    /// characters. Otherwise, an INVALID_ARGUMENT error is returned.
    ///
    /// At least one of
    /// [search_query][google.cloud.retail.v2alpha.UserEvent.search_query] or
    /// [page_categories][google.cloud.retail.v2alpha.UserEvent.page_categories] is
    /// required for `search` events. Other event types should not set this field.
    /// Otherwise, an INVALID_ARGUMENT error is returned.
    #[prost(string, tag = "10")]
    pub search_query: ::prost::alloc::string::String,
    /// The filter syntax consists of an expression language for constructing a
    /// predicate from one or more fields of the products being filtered.
    ///
    /// See
    /// [SearchRequest.filter][google.cloud.retail.v2alpha.SearchRequest.filter]
    /// for definition and syntax.
    ///
    /// The value must be a UTF-8 encoded string with a length limit of 1,000
    /// characters. Otherwise, an INVALID_ARGUMENT error is returned.
    #[prost(string, tag = "16")]
    pub filter: ::prost::alloc::string::String,
    /// The order in which products are returned.
    ///
    /// See
    /// [SearchRequest.order_by][google.cloud.retail.v2alpha.SearchRequest.order_by]
    /// for definition and syntax.
    ///
    /// The value must be a UTF-8 encoded string with a length limit of 1,000
    /// characters. Otherwise, an INVALID_ARGUMENT error is returned.
    ///
    /// This can only be set for `search` events. Other event types should not set
    /// this field. Otherwise, an INVALID_ARGUMENT error is returned.
    #[prost(string, tag = "17")]
    pub order_by: ::prost::alloc::string::String,
    /// An integer that specifies the current offset for pagination (the 0-indexed
    /// starting location, amongst the products deemed by the API as relevant).
    ///
    /// See
    /// [SearchRequest.offset][google.cloud.retail.v2alpha.SearchRequest.offset]
    /// for definition.
    ///
    /// If this field is negative, an INVALID_ARGUMENT is returned.
    ///
    /// This can only be set for `search` events. Other event types should not set
    /// this field. Otherwise, an INVALID_ARGUMENT error is returned.
    #[prost(int32, tag = "18")]
    pub offset: i32,
    /// The categories associated with a category page.
    ///
    /// To represent full path of category, use '>' sign to separate different
    /// hierarchies. If '>' is part of the category name, please replace it with
    /// other character(s).
    ///
    /// Category pages include special pages such as sales or promotions. For
    /// instance, a special sale page may have the category hierarchy:
    /// "pageCategories" : ["Sales > 2017 Black Friday Deals"].
    ///
    /// Required for `category-page-view` events. At least one of
    /// [search_query][google.cloud.retail.v2alpha.UserEvent.search_query] or
    /// [page_categories][google.cloud.retail.v2alpha.UserEvent.page_categories] is
    /// required for `search` events. Other event types should not set this field.
    /// Otherwise, an INVALID_ARGUMENT error is returned.
    #[prost(string, repeated, tag = "11")]
    pub page_categories: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// User information.
    #[prost(message, optional, tag = "12")]
    pub user_info: ::core::option::Option<UserInfo>,
    /// Complete URL (window.location.href) of the user's current page.
    ///
    /// When using the client side event reporting with JavaScript pixel and Google
    /// Tag Manager, this value is filled in automatically. Maximum length 5,000
    /// characters.
    #[prost(string, tag = "13")]
    pub uri: ::prost::alloc::string::String,
    /// The referrer URL of the current page.
    ///
    /// When using the client side event reporting with JavaScript pixel and Google
    /// Tag Manager, this value is filled in automatically.
    #[prost(string, tag = "14")]
    pub referrer_uri: ::prost::alloc::string::String,
    /// A unique id of a web page view.
    ///
    /// This should be kept the same for all user events triggered from the same
    /// pageview. For example, an item detail page view could trigger multiple
    /// events as the user is browsing the page. The `pageViewId` property should
    /// be kept the same for all these events so that they can be grouped together
    /// properly.
    ///
    /// When using the client side event reporting with JavaScript pixel and Google
    /// Tag Manager, this value is filled in automatically.
    #[prost(string, tag = "15")]
    pub page_view_id: ::prost::alloc::string::String,
}
/// Detailed product information associated with a user event.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProductDetail {
    /// Required. [Product][google.cloud.retail.v2alpha.Product] information.
    ///
    /// Only [Product.id][google.cloud.retail.v2alpha.Product.id] field is used
    /// when ingesting an event, all other product fields are ignored as we will
    /// look them up from the catalog.
    #[prost(message, optional, tag = "1")]
    pub product: ::core::option::Option<Product>,
    /// Quantity of the product associated with the user event.
    ///
    /// For example, this field will be 2 if two products are added to the shopping
    /// cart for `purchase-complete` event. Required for `add-to-cart` and
    /// `purchase-complete` event types.
    #[prost(message, optional, tag = "2")]
    pub quantity: ::core::option::Option<i32>,
}
/// Detailed completion information including completion attribution token and
/// clicked completion info.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CompletionDetail {
    /// Completion attribution token in
    /// [CompleteQueryResponse.attribution_token][google.cloud.retail.v2alpha.CompleteQueryResponse.attribution_token].
    #[prost(string, tag = "1")]
    pub completion_attribution_token: ::prost::alloc::string::String,
    /// End user selected
    /// [CompleteQueryResponse.CompletionResult.suggestion][google.cloud.retail.v2alpha.CompleteQueryResponse.CompletionResult.suggestion].
    #[prost(string, tag = "2")]
    pub selected_suggestion: ::prost::alloc::string::String,
    /// End user selected
    /// [CompleteQueryResponse.CompletionResult.suggestion][google.cloud.retail.v2alpha.CompleteQueryResponse.CompletionResult.suggestion]
    /// position, starting from 0.
    #[prost(int32, tag = "3")]
    pub selected_position: i32,
}
/// A transaction represents the entire purchase transaction.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PurchaseTransaction {
    /// The transaction ID with a length limit of 128 characters.
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// Required. Total non-zero revenue or grand total associated with the
    /// transaction. This value include shipping, tax, or other adjustments to
    /// total revenue that you want to include as part of your revenue
    /// calculations.
    #[prost(float, tag = "2")]
    pub revenue: f32,
    /// All the taxes associated with the transaction.
    #[prost(float, tag = "3")]
    pub tax: f32,
    /// All the costs associated with the products. These can be manufacturing
    /// costs, shipping expenses not borne by the end user, or any other costs,
    /// such that:
    ///
    /// * Profit =
    /// [revenue][google.cloud.retail.v2alpha.PurchaseTransaction.revenue] -
    /// [tax][google.cloud.retail.v2alpha.PurchaseTransaction.tax] -
    /// [cost][google.cloud.retail.v2alpha.PurchaseTransaction.cost]
    #[prost(float, tag = "4")]
    pub cost: f32,
    /// Required. Currency code. Use three-character ISO-4217 code.
    #[prost(string, tag = "5")]
    pub currency_code: ::prost::alloc::string::String,
}
/// Google Cloud Storage location for input content.
/// format.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GcsSource {
    /// Required. Google Cloud Storage URIs to input files. URI can be up to
    /// 2000 characters long. URIs can match the full object path (for example,
    /// `gs://bucket/directory/object.json`) or a pattern matching one or more
    /// files, such as `gs://bucket/directory/*.json`. A request can
    /// contain at most 100 files, and each file can be up to 2 GB. See
    /// [Importing product
    /// information](https://cloud.google.com/retail/recommendations-ai/docs/upload-catalog)
    /// for the expected file format and setup instructions.
    #[prost(string, repeated, tag = "1")]
    pub input_uris: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The schema to use when parsing the data from the source.
    ///
    /// Supported values for product imports:
    ///
    /// * `product` (default): One JSON
    /// [Product][google.cloud.retail.v2alpha.Product] per line. Each product must
    ///   have a valid [Product.id][google.cloud.retail.v2alpha.Product.id].
    /// * `product_merchant_center`: See [Importing catalog data from Merchant
    ///   Center](https://cloud.google.com/retail/recommendations-ai/docs/upload-catalog#mc).
    ///
    /// Supported values for user events imports:
    ///
    /// * `user_event` (default): One JSON
    /// [UserEvent][google.cloud.retail.v2alpha.UserEvent] per line.
    /// * `user_event_ga360`: Using
    ///   https://support.google.com/analytics/answer/3437719.
    #[prost(string, tag = "2")]
    pub data_schema: ::prost::alloc::string::String,
}
/// BigQuery source import data from.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BigQuerySource {
    /// The project ID (can be project # or ID) that the BigQuery source is in with
    /// a length limit of 128 characters. If not specified, inherits the project
    /// ID from the parent request.
    #[prost(string, tag = "5")]
    pub project_id: ::prost::alloc::string::String,
    /// Required. The BigQuery data set to copy the data from with a length limit
    /// of 1,024 characters.
    #[prost(string, tag = "1")]
    pub dataset_id: ::prost::alloc::string::String,
    /// Required. The BigQuery table to copy the data from with a length limit of
    /// 1,024 characters.
    #[prost(string, tag = "2")]
    pub table_id: ::prost::alloc::string::String,
    /// Intermediate Cloud Storage directory used for the import with a length
    /// limit of 2,000 characters. Can be specified if one wants to have the
    /// BigQuery export to a specific Cloud Storage directory.
    #[prost(string, tag = "3")]
    pub gcs_staging_dir: ::prost::alloc::string::String,
    /// The schema to use when parsing the data from the source.
    ///
    /// Supported values for product imports:
    ///
    /// * `product` (default): One JSON
    /// [Product][google.cloud.retail.v2alpha.Product] per line. Each product must
    ///   have a valid [Product.id][google.cloud.retail.v2alpha.Product.id].
    /// * `product_merchant_center`: See [Importing catalog data from Merchant
    ///   Center](https://cloud.google.com/retail/recommendations-ai/docs/upload-catalog#mc).
    ///
    /// Supported values for user events imports:
    ///
    /// * `user_event` (default): One JSON
    /// [UserEvent][google.cloud.retail.v2alpha.UserEvent] per line.
    /// * `user_event_ga360`: Using
    ///   https://support.google.com/analytics/answer/3437719.
    #[prost(string, tag = "4")]
    pub data_schema: ::prost::alloc::string::String,
    /// BigQuery table partition info. Leave this empty if the BigQuery table
    /// is not partitioned.
    #[prost(oneof = "big_query_source::Partition", tags = "6")]
    pub partition: ::core::option::Option<big_query_source::Partition>,
}
/// Nested message and enum types in `BigQuerySource`.
pub mod big_query_source {
    /// BigQuery table partition info. Leave this empty if the BigQuery table
    /// is not partitioned.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Partition {
        /// BigQuery time partitioned table's _PARTITIONDATE in YYYY-MM-DD format.
        ///
        /// Only supported when
        /// [ImportProductsRequest.reconciliation_mode][google.cloud.retail.v2alpha.ImportProductsRequest.reconciliation_mode]
        /// is set to `FULL`.
        #[prost(message, tag = "6")]
        PartitionDate(super::super::super::super::r#type::Date),
    }
}
/// The inline source for the input config for ImportProducts method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProductInlineSource {
    /// Required. A list of products to update/create. Each product must have a
    /// valid [Product.id][google.cloud.retail.v2alpha.Product.id]. Recommended max
    /// of 100 items.
    #[prost(message, repeated, tag = "1")]
    pub products: ::prost::alloc::vec::Vec<Product>,
}
/// The inline source for the input config for ImportUserEvents method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserEventInlineSource {
    /// Required. A list of user events to import. Recommended max of 10k items.
    #[prost(message, repeated, tag = "1")]
    pub user_events: ::prost::alloc::vec::Vec<UserEvent>,
}
/// Configuration of destination for Import related errors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportErrorsConfig {
    /// Required. Errors destination.
    #[prost(oneof = "import_errors_config::Destination", tags = "1")]
    pub destination: ::core::option::Option<import_errors_config::Destination>,
}
/// Nested message and enum types in `ImportErrorsConfig`.
pub mod import_errors_config {
    /// Required. Errors destination.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Destination {
        /// Google Cloud Storage path for import errors. This must be an empty,
        /// existing Cloud Storage bucket. Import errors will be written to a file in
        /// this bucket, one per line, as a JSON-encoded
        /// `google.rpc.Status` message.
        #[prost(string, tag = "1")]
        GcsPrefix(::prost::alloc::string::String),
    }
}
/// Request message for Import methods.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportProductsRequest {
    /// Required.
    /// `projects/1234/locations/global/catalogs/default_catalog/branches/default_branch`
    ///
    /// If no updateMask is specified, requires products.create permission.
    /// If updateMask is specified, requires products.update permission.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Unique identifier provided by client, within the ancestor
    /// dataset scope. Ensures idempotency and used for request deduplication.
    /// Server-generated if unspecified. Up to 128 characters long and must match
    /// the pattern: "[a-zA-Z0-9_]+". This is returned as [Operation.name][] in
    /// [ImportMetadata][google.cloud.retail.v2alpha.ImportMetadata].
    ///
    /// Only supported when
    /// [ImportProductsRequest.reconciliation_mode][google.cloud.retail.v2alpha.ImportProductsRequest.reconciliation_mode]
    /// is set to `FULL`.
    #[prost(string, tag = "6")]
    pub request_id: ::prost::alloc::string::String,
    /// Required. The desired input location of the data.
    #[prost(message, optional, tag = "2")]
    pub input_config: ::core::option::Option<ProductInputConfig>,
    /// The desired location of errors incurred during the Import.
    #[prost(message, optional, tag = "3")]
    pub errors_config: ::core::option::Option<ImportErrorsConfig>,
    /// Indicates which fields in the provided imported 'products' to update. If
    /// not set, will by default update all fields.
    #[prost(message, optional, tag = "4")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// The mode of reconciliation between existing products and the products to be
    /// imported. Defaults to
    /// [ReconciliationMode.INCREMENTAL][google.cloud.retail.v2alpha.ImportProductsRequest.ReconciliationMode.INCREMENTAL].
    #[prost(enumeration = "import_products_request::ReconciliationMode", tag = "5")]
    pub reconciliation_mode: i32,
    /// Pub/Sub topic for receiving notification. If this field is set,
    /// when the import is finished, a notification will be sent to
    /// specified Pub/Sub topic. The message data will be JSON string of a
    /// [Operation][google.longrunning.Operation].
    /// Format of the Pub/Sub topic is `projects/{project}/topics/{topic}`.
    ///
    /// Only supported when
    /// [ImportProductsRequest.reconciliation_mode][google.cloud.retail.v2alpha.ImportProductsRequest.reconciliation_mode]
    /// is set to `FULL`.
    #[prost(string, tag = "7")]
    pub notification_pubsub_topic: ::prost::alloc::string::String,
}
/// Nested message and enum types in `ImportProductsRequest`.
pub mod import_products_request {
    /// Indicates how imported products are reconciled with the existing products
    /// created or imported before.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ReconciliationMode {
        /// Defaults to INCREMENTAL.
        Unspecified = 0,
        /// Inserts new products or updates existing products.
        Incremental = 1,
        /// Calculates diff and replaces the entire product dataset. Existing
        /// products may be deleted if they are not present in the source location.
        ///
        /// Can only be while using
        /// [BigQuerySource][google.cloud.retail.v2alpha.BigQuerySource].
        ///
        /// Add the IAM permission “BigQuery Data Viewer” for
        /// cloud-retail-customer-data-access@system.gserviceaccount.com before
        /// using this feature otherwise an error is thrown.
        ///
        /// This feature is only available for users who have Retail Search enabled.
        /// Contact Retail Support (retail-search-support@google.com) if you are
        /// interested in using Retail Search.
        Full = 2,
    }
}
/// Request message for the ImportUserEvents request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportUserEventsRequest {
    /// Required. `projects/1234/locations/global/catalogs/default_catalog`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The desired input location of the data.
    #[prost(message, optional, tag = "2")]
    pub input_config: ::core::option::Option<UserEventInputConfig>,
    /// The desired location of errors incurred during the Import. Cannot be set
    /// for inline user event imports.
    #[prost(message, optional, tag = "3")]
    pub errors_config: ::core::option::Option<ImportErrorsConfig>,
}
/// Request message for ImportCompletionData methods.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportCompletionDataRequest {
    /// Required. The catalog which the suggestions dataset belongs to.
    ///
    /// Format: `projects/1234/locations/global/catalogs/default_catalog`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The desired input location of the data.
    #[prost(message, optional, tag = "2")]
    pub input_config: ::core::option::Option<CompletionDataInputConfig>,
    /// Pub/Sub topic for receiving notification. If this field is set,
    /// when the import is finished, a notification will be sent to
    /// specified Pub/Sub topic. The message data will be JSON string of a
    /// [Operation][google.longrunning.Operation].
    /// Format of the Pub/Sub topic is `projects/{project}/topics/{topic}`.
    #[prost(string, tag = "3")]
    pub notification_pubsub_topic: ::prost::alloc::string::String,
}
/// The input config source for products.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProductInputConfig {
    /// Required. The source of the input.
    #[prost(oneof = "product_input_config::Source", tags = "1, 2, 3")]
    pub source: ::core::option::Option<product_input_config::Source>,
}
/// Nested message and enum types in `ProductInputConfig`.
pub mod product_input_config {
    /// Required. The source of the input.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Source {
        /// The Inline source for the input content for products.
        #[prost(message, tag = "1")]
        ProductInlineSource(super::ProductInlineSource),
        /// Google Cloud Storage location for the input content.
        #[prost(message, tag = "2")]
        GcsSource(super::GcsSource),
        /// BigQuery input source.
        #[prost(message, tag = "3")]
        BigQuerySource(super::BigQuerySource),
    }
}
/// The input config source for user events.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserEventInputConfig {
    /// The source of the input.
    #[prost(oneof = "user_event_input_config::Source", tags = "1, 2, 3")]
    pub source: ::core::option::Option<user_event_input_config::Source>,
}
/// Nested message and enum types in `UserEventInputConfig`.
pub mod user_event_input_config {
    /// The source of the input.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Source {
        /// Required. The Inline source for the input content for UserEvents.
        #[prost(message, tag = "1")]
        UserEventInlineSource(super::UserEventInlineSource),
        /// Required. Google Cloud Storage location for the input content.
        #[prost(message, tag = "2")]
        GcsSource(super::GcsSource),
        /// Required. BigQuery input source.
        #[prost(message, tag = "3")]
        BigQuerySource(super::BigQuerySource),
    }
}
/// The input config source for completion data.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CompletionDataInputConfig {
    /// The source of the input.
    ///
    /// Supported
    /// [BigQuerySource.data_schema][google.cloud.retail.v2alpha.BigQuerySource.data_schema]
    /// values for suggestions imports:
    ///
    /// * `suggestions` (default): One JSON completion suggestion per line.
    /// * `denylist`:  One JSON deny suggestion per line.
    /// * `allowlist`:  One JSON allow suggestion per line.
    #[prost(oneof = "completion_data_input_config::Source", tags = "1")]
    pub source: ::core::option::Option<completion_data_input_config::Source>,
}
/// Nested message and enum types in `CompletionDataInputConfig`.
pub mod completion_data_input_config {
    /// The source of the input.
    ///
    /// Supported
    /// [BigQuerySource.data_schema][google.cloud.retail.v2alpha.BigQuerySource.data_schema]
    /// values for suggestions imports:
    ///
    /// * `suggestions` (default): One JSON completion suggestion per line.
    /// * `denylist`:  One JSON deny suggestion per line.
    /// * `allowlist`:  One JSON allow suggestion per line.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Source {
        /// Required. BigQuery input source.
        ///
        /// Add the IAM permission “BigQuery Data Viewer” for
        /// cloud-retail-customer-data-access@system.gserviceaccount.com before
        /// using this feature otherwise an error is thrown.
        #[prost(message, tag = "1")]
        BigQuerySource(super::BigQuerySource),
    }
}
/// Metadata related to the progress of the Import operation. This will be
/// returned by the google.longrunning.Operation.metadata field.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportMetadata {
    /// Operation create time.
    #[prost(message, optional, tag = "1")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Operation last update time. If the operation is done, this is also the
    /// finish time.
    #[prost(message, optional, tag = "2")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Count of entries that were processed successfully.
    #[prost(int64, tag = "3")]
    pub success_count: i64,
    /// Count of entries that encountered errors while processing.
    #[prost(int64, tag = "4")]
    pub failure_count: i64,
    /// Id of the request / operation. This is parroting back the requestId
    /// that was passed in the request.
    #[prost(string, tag = "5")]
    pub request_id: ::prost::alloc::string::String,
    /// Pub/Sub topic for receiving notification. If this field is set,
    /// when the import is finished, a notification will be sent to
    /// specified Pub/Sub topic. The message data will be JSON string of a
    /// [Operation][google.longrunning.Operation].
    /// Format of the Pub/Sub topic is `projects/{project}/topics/{topic}`.
    #[prost(string, tag = "6")]
    pub notification_pubsub_topic: ::prost::alloc::string::String,
}
/// Response of the
/// [ImportProductsRequest][google.cloud.retail.v2alpha.ImportProductsRequest].
/// If the long running operation is done, then this message is returned by the
/// google.longrunning.Operations.response field if the operation was successful.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportProductsResponse {
    /// A sample of errors encountered while processing the request.
    #[prost(message, repeated, tag = "1")]
    pub error_samples: ::prost::alloc::vec::Vec<super::super::super::rpc::Status>,
    /// Echoes the destination for the complete errors in the request if set.
    #[prost(message, optional, tag = "2")]
    pub errors_config: ::core::option::Option<ImportErrorsConfig>,
}
/// Response of the ImportUserEventsRequest. If the long running
/// operation was successful, then this message is returned by the
/// google.longrunning.Operations.response field if the operation was successful.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportUserEventsResponse {
    /// A sample of errors encountered while processing the request.
    #[prost(message, repeated, tag = "1")]
    pub error_samples: ::prost::alloc::vec::Vec<super::super::super::rpc::Status>,
    /// Echoes the destination for the complete errors if this field was set in
    /// the request.
    #[prost(message, optional, tag = "2")]
    pub errors_config: ::core::option::Option<ImportErrorsConfig>,
    /// Aggregated statistics of user event import status.
    #[prost(message, optional, tag = "3")]
    pub import_summary: ::core::option::Option<UserEventImportSummary>,
}
/// A summary of import result. The UserEventImportSummary summarizes
/// the import status for user events.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserEventImportSummary {
    /// Count of user events imported with complete existing catalog information.
    #[prost(int64, tag = "1")]
    pub joined_events_count: i64,
    /// Count of user events imported, but with catalog information not found
    /// in the imported catalog.
    #[prost(int64, tag = "2")]
    pub unjoined_events_count: i64,
}
/// Response of the
/// [ImportCompletionDataRequest][google.cloud.retail.v2alpha.ImportCompletionDataRequest].
/// If the long running operation is done, this message is returned by the
/// google.longrunning.Operations.response field if the operation is successful.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportCompletionDataResponse {
    /// A sample of errors encountered while processing the request.
    #[prost(message, repeated, tag = "1")]
    pub error_samples: ::prost::alloc::vec::Vec<super::super::super::rpc::Status>,
}
/// Configures what level the product should be uploaded with regards to
/// how users will be send events and how predictions will be made.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProductLevelConfig {
    /// The type of [Product][google.cloud.retail.v2alpha.Product]s allowed to be
    /// ingested into the catalog. Acceptable values are:
    ///
    /// * `primary` (default): You can only ingest
    /// [Product.Type.PRIMARY][google.cloud.retail.v2alpha.Product.Type.PRIMARY]
    ///   [Product][google.cloud.retail.v2alpha.Product]s. This means
    ///   [Product.primary_product_id][google.cloud.retail.v2alpha.Product.primary_product_id]
    ///   can only be empty or set to the same value as
    ///   [Product.id][google.cloud.retail.v2alpha.Product.id].
    /// * `variant`: You can only ingest
    /// [Product.Type.VARIANT][google.cloud.retail.v2alpha.Product.Type.VARIANT]
    /// [Product][google.cloud.retail.v2alpha.Product]s.
    ///   This means
    ///   [Product.primary_product_id][google.cloud.retail.v2alpha.Product.primary_product_id]
    ///   cannot be empty.
    ///
    /// If this field is set to an invalid value other than these, an
    /// INVALID_ARGUMENT error is returned.
    ///
    /// If this field is `variant` and
    /// [merchant_center_product_id_field][google.cloud.retail.v2alpha.ProductLevelConfig.merchant_center_product_id_field]
    /// is `itemGroupId`, an INVALID_ARGUMENT error is returned.
    ///
    /// See [Using product
    /// levels](https://cloud.google.com/retail/recommendations-ai/docs/catalog#product-levels)
    /// for more details.
    #[prost(string, tag = "1")]
    pub ingestion_product_type: ::prost::alloc::string::String,
    /// Which field of [Merchant Center
    /// Product](/bigquery-transfer/docs/merchant-center-products-schema) should be
    /// imported as [Product.id][google.cloud.retail.v2alpha.Product.id].
    /// Acceptable values are:
    ///
    /// * `offerId` (default): Import `offerId` as the product ID.
    /// * `itemGroupId`: Import `itemGroupId` as the product ID. Notice that Retail
    ///   API will choose one item from the ones with the same `itemGroupId`, and
    ///   use it to represent the item group.
    ///
    /// If this field is set to an invalid value other than these, an
    /// INVALID_ARGUMENT error is returned.
    ///
    /// If this field is `itemGroupId` and
    /// [ingestion_product_type][google.cloud.retail.v2alpha.ProductLevelConfig.ingestion_product_type]
    /// is `variant`, an INVALID_ARGUMENT error is returned.
    ///
    /// See [Using product
    /// levels](https://cloud.google.com/retail/recommendations-ai/docs/catalog#product-levels)
    /// for more details.
    #[prost(string, tag = "2")]
    pub merchant_center_product_id_field: ::prost::alloc::string::String,
}
/// Represents a link between a Merchant Center account and a branch.
/// Once a link is established, products from the linked merchant center account
/// will be streamed to the linked branch.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MerchantCenterLink {
    /// Required. The linked [Merchant center account
    /// id](https://developers.google.com/shopping-content/guides/accountstatuses).
    /// The account must be a standalone account or a sub-account of a MCA.
    #[prost(int64, tag = "1")]
    pub merchant_center_account_id: i64,
    /// The branch id (e.g. 0/1/2) within this catalog that products from
    /// merchant_center_account_id are streamed to. When updating this field, an
    /// empty value will use the currently configured default branch. However,
    /// changing the default branch later on won't change the linked branch here.
    ///
    /// A single branch id can only have one linked merchant center account id.
    #[prost(string, tag = "2")]
    pub branch_id: ::prost::alloc::string::String,
    /// String representing the destination to import for, all if left empty.
    /// List of possible values can be found here.
    /// [https://support.google.com/merchants/answer/7501026?hl=en]
    /// List of allowed string values:
    /// "shopping-ads", "buy-on-google-listings", "display-ads", "local-inventory
    /// -ads", "free-listings", "free-local-listings"
    /// NOTE: The string values are case sensitive.
    #[prost(string, repeated, tag = "3")]
    pub destinations: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Configures Merchant Center linking.
/// Links contained in the config will be used to sync data from a Merchant
/// Center account to a Cloud Retail branch.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MerchantCenterLinkingConfig {
    /// Links between Merchant Center accounts and branches.
    #[prost(message, repeated, tag = "1")]
    pub links: ::prost::alloc::vec::Vec<MerchantCenterLink>,
}
/// The catalog configuration.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Catalog {
    /// Required. Immutable. The fully qualified resource name of the catalog.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. Immutable. The catalog display name.
    ///
    /// This field must be a UTF-8 encoded string with a length limit of 128
    /// characters. Otherwise, an INVALID_ARGUMENT error is returned.
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    /// Required. The product level configuration.
    #[prost(message, optional, tag = "4")]
    pub product_level_config: ::core::option::Option<ProductLevelConfig>,
    /// The Merchant Center linking configuration.
    /// Once a link is added, the data stream from Merchant Center to Cloud Retail
    /// will be enabled automatically. The requester must have access to the
    /// merchant center account in order to make changes to this field.
    #[prost(message, optional, tag = "6")]
    pub merchant_center_linking_config: ::core::option::Option<MerchantCenterLinkingConfig>,
}
/// Request for
/// [CatalogService.ListCatalogs][google.cloud.retail.v2alpha.CatalogService.ListCatalogs]
/// method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCatalogsRequest {
    /// Required. The account resource name with an associated location.
    ///
    /// If the caller does not have permission to list
    /// [Catalog][google.cloud.retail.v2alpha.Catalog]s under this location,
    /// regardless of whether or not this location exists, a PERMISSION_DENIED
    /// error is returned.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Maximum number of [Catalog][google.cloud.retail.v2alpha.Catalog]s to
    /// return. If unspecified, defaults to 50. The maximum allowed value is 1000.
    /// Values above 1000 will be coerced to 1000.
    ///
    /// If this field is negative, an INVALID_ARGUMENT is returned.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A page token
    /// [ListCatalogsResponse.next_page_token][google.cloud.retail.v2alpha.ListCatalogsResponse.next_page_token],
    /// received from a previous
    /// [CatalogService.ListCatalogs][google.cloud.retail.v2alpha.CatalogService.ListCatalogs]
    /// call. Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to
    /// [CatalogService.ListCatalogs][google.cloud.retail.v2alpha.CatalogService.ListCatalogs]
    /// must match the call that provided the page token. Otherwise, an
    /// INVALID_ARGUMENT error is returned.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response for
/// [CatalogService.ListCatalogs][google.cloud.retail.v2alpha.CatalogService.ListCatalogs]
/// method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCatalogsResponse {
    /// All the customer's [Catalog][google.cloud.retail.v2alpha.Catalog]s.
    #[prost(message, repeated, tag = "1")]
    pub catalogs: ::prost::alloc::vec::Vec<Catalog>,
    /// A token that can be sent as
    /// [ListCatalogsRequest.page_token][google.cloud.retail.v2alpha.ListCatalogsRequest.page_token]
    /// to retrieve the next page. If this field is omitted, there are no
    /// subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request for
/// [CatalogService.UpdateCatalog][google.cloud.retail.v2alpha.CatalogService.UpdateCatalog]
/// method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateCatalogRequest {
    /// Required. The [Catalog][google.cloud.retail.v2alpha.Catalog] to update.
    ///
    /// If the caller does not have permission to update the
    /// [Catalog][google.cloud.retail.v2alpha.Catalog], regardless of whether or
    /// not it exists, a PERMISSION_DENIED error is returned.
    ///
    /// If the [Catalog][google.cloud.retail.v2alpha.Catalog] to update does not
    /// exist, a NOT_FOUND error is returned.
    #[prost(message, optional, tag = "1")]
    pub catalog: ::core::option::Option<Catalog>,
    /// Indicates which fields in the provided
    /// [Catalog][google.cloud.retail.v2alpha.Catalog] to update.
    ///
    /// If an unsupported or unknown field is provided, an INVALID_ARGUMENT error
    /// is returned.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message to set a specified branch as new default_branch.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetDefaultBranchRequest {
    /// Full resource name of the catalog, such as
    /// `projects/*/locations/global/catalogs/default_catalog`.
    #[prost(string, tag = "1")]
    pub catalog: ::prost::alloc::string::String,
    /// The final component of the resource name of a branch.
    ///
    /// This field must be one of "0", "1" or "2". Otherwise, an INVALID_ARGUMENT
    /// error is returned.
    #[prost(string, tag = "2")]
    pub branch_id: ::prost::alloc::string::String,
    /// Some note on this request, this can be retrieved by
    /// [CatalogService.GetDefaultBranch][google.cloud.retail.v2alpha.CatalogService.GetDefaultBranch]
    /// before next valid default branch set occurs.
    ///
    /// This field must be a UTF-8 encoded string with a length limit of 1,000
    /// characters. Otherwise, an INVALID_ARGUMENT error is returned.
    #[prost(string, tag = "3")]
    pub note: ::prost::alloc::string::String,
}
/// Request message to show which branch is currently the default branch.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDefaultBranchRequest {
    /// The parent catalog resource name, such as
    /// `projects/*/locations/global/catalogs/default_catalog`.
    #[prost(string, tag = "1")]
    pub catalog: ::prost::alloc::string::String,
}
/// Response message of
/// [CatalogService.GetDefaultBranch][google.cloud.retail.v2alpha.CatalogService.GetDefaultBranch].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDefaultBranchResponse {
    /// Full resource name of the branch id currently set as default branch.
    #[prost(string, tag = "1")]
    pub branch: ::prost::alloc::string::String,
    /// The time when this branch is set to default.
    #[prost(message, optional, tag = "2")]
    pub set_time: ::core::option::Option<::prost_types::Timestamp>,
    /// This corresponds to
    /// [SetDefaultBranchRequest.note][google.cloud.retail.v2alpha.SetDefaultBranchRequest.note]
    /// field, when this branch was set as default.
    #[prost(string, tag = "3")]
    pub note: ::prost::alloc::string::String,
}
#[doc = r" Generated client implementations."]
pub mod catalog_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Service for managing catalog configuration."]
    #[derive(Debug, Clone)]
    pub struct CatalogServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> CatalogServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + Send + Sync + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> CatalogServiceClient<InterceptedService<T, F>>
        where
            F: FnMut(tonic::Request<()>) -> Result<tonic::Request<()>, tonic::Status>,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            CatalogServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        #[doc = r" Compress requests with `gzip`."]
        #[doc = r""]
        #[doc = r" This requires the server to support it otherwise it might respond with an"]
        #[doc = r" error."]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        #[doc = r" Enable decompressing responses with `gzip`."]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        #[doc = " Lists all the [Catalog][google.cloud.retail.v2alpha.Catalog]s associated"]
        #[doc = " with the project."]
        pub async fn list_catalogs(
            &mut self,
            request: impl tonic::IntoRequest<super::ListCatalogsRequest>,
        ) -> Result<tonic::Response<super::ListCatalogsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.retail.v2alpha.CatalogService/ListCatalogs",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates the [Catalog][google.cloud.retail.v2alpha.Catalog]s."]
        pub async fn update_catalog(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateCatalogRequest>,
        ) -> Result<tonic::Response<super::Catalog>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.retail.v2alpha.CatalogService/UpdateCatalog",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Set a specified branch id as default branch. API methods such as"]
        #[doc = " [SearchService.Search][google.cloud.retail.v2alpha.SearchService.Search],"]
        #[doc = " [ProductService.GetProduct][google.cloud.retail.v2alpha.ProductService.GetProduct],"]
        #[doc = " [ProductService.ListProducts][google.cloud.retail.v2alpha.ProductService.ListProducts]"]
        #[doc = " will treat requests using \"default_branch\" to the actual branch id set as"]
        #[doc = " default."]
        #[doc = ""]
        #[doc = " For example, if `projects/*/locations/*/catalogs/*/branches/1` is set as"]
        #[doc = " default, setting"]
        #[doc = " [SearchRequest.branch][google.cloud.retail.v2alpha.SearchRequest.branch] to"]
        #[doc = " `projects/*/locations/*/catalogs/*/branches/default_branch` is equivalent"]
        #[doc = " to setting"]
        #[doc = " [SearchRequest.branch][google.cloud.retail.v2alpha.SearchRequest.branch] to"]
        #[doc = " `projects/*/locations/*/catalogs/*/branches/1`."]
        #[doc = ""]
        #[doc = " Using multiple branches can be useful when developers would like"]
        #[doc = " to have a staging branch to test and verify for future usage. When it"]
        #[doc = " becomes ready, developers switch on the staging branch using this API while"]
        #[doc = " keeping using `projects/*/locations/*/catalogs/*/branches/default_branch`"]
        #[doc = " as [SearchRequest.branch][google.cloud.retail.v2alpha.SearchRequest.branch]"]
        #[doc = " to route the traffic to this staging branch."]
        #[doc = ""]
        #[doc = " CAUTION: If you have live predict/search traffic, switching the default"]
        #[doc = " branch could potentially cause outages if the ID space of the new branch is"]
        #[doc = " very different from the old one."]
        #[doc = ""]
        #[doc = " More specifically:"]
        #[doc = ""]
        #[doc = " * PredictionService will only return product IDs from branch {newBranch}."]
        #[doc = " * SearchService will only return product IDs from branch {newBranch}"]
        #[doc = "   (if branch is not explicitly set)."]
        #[doc = " * UserEventService will only join events with products from branch"]
        #[doc = "   {newBranch}."]
        #[doc = ""]
        #[doc = " This feature is only available for users who have Retail Search enabled."]
        #[doc = " Contact Retail Support (retail-search-support@google.com) if you are"]
        #[doc = " interested in using Retail Search."]
        pub async fn set_default_branch(
            &mut self,
            request: impl tonic::IntoRequest<super::SetDefaultBranchRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.retail.v2alpha.CatalogService/SetDefaultBranch",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Get which branch is currently default branch set by"]
        #[doc = " [CatalogService.SetDefaultBranch][google.cloud.retail.v2alpha.CatalogService.SetDefaultBranch]"]
        #[doc = " method under a specified parent catalog."]
        #[doc = ""]
        #[doc = " This feature is only available for users who have Retail Search enabled."]
        #[doc = " Contact Retail Support (retail-search-support@google.com) if you are"]
        #[doc = " interested in using Retail Search."]
        pub async fn get_default_branch(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDefaultBranchRequest>,
        ) -> Result<tonic::Response<super::GetDefaultBranchResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.retail.v2alpha.CatalogService/GetDefaultBranch",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Auto-complete parameters.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CompleteQueryRequest {
    /// Required. Catalog for which the completion is performed.
    ///
    /// Full resource name of catalog, such as
    /// `projects/*/locations/global/catalogs/default_catalog`.
    #[prost(string, tag = "1")]
    pub catalog: ::prost::alloc::string::String,
    /// Required. The query used to generate suggestions.
    ///
    /// The maximum number of allowed characters is 255.
    #[prost(string, tag = "2")]
    pub query: ::prost::alloc::string::String,
    /// A unique identifier for tracking visitors. For example, this could be
    /// implemented with an HTTP cookie, which should be able to uniquely identify
    /// a visitor on a single device. This unique identifier should not change if
    /// the visitor logs in or out of the website.
    ///
    /// The field must be a UTF-8 encoded string with a length limit of 128
    /// characters. Otherwise, an INVALID_ARGUMENT error is returned.
    #[prost(string, tag = "7")]
    pub visitor_id: ::prost::alloc::string::String,
    /// The list of languages of the query. This is
    /// the BCP-47 language code, such as "en-US" or "sr-Latn".
    /// For more information, see
    /// [Tags for Identifying Languages](https://tools.ietf.org/html/bcp47).
    ///
    /// The maximum number of allowed characters is 255.
    /// Only "en-US" is currently supported.
    #[prost(string, repeated, tag = "3")]
    pub language_codes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The device type context for completion suggestions.
    /// It is useful to apply different suggestions on different device types, e.g.
    /// DESKTOP, MOBILE. If it is empty, the suggestions are across all device
    /// types.
    ///
    /// Supported formats:
    ///
    /// * UNKNOWN_DEVICE_TYPE
    ///
    /// * DESKTOP
    ///
    /// * MOBILE
    ///
    /// * A customized string starts with OTHER_, e.g. OTHER_IPHONE.
    #[prost(string, tag = "4")]
    pub device_type: ::prost::alloc::string::String,
    /// Determines which dataset to use for fetching completion. "user-data" will
    /// use the imported dataset through [ImportCompletionData][]. "cloud-retail"
    /// will use the dataset generated by cloud retail based on user events. If
    /// leave empty, it will use the "user-data".
    ///
    /// Current supported values:
    ///
    /// * user-data
    ///
    /// * cloud-retail
    ///   This option is not automatically enabled. Before using cloud-retail,
    ///   contact retail-search-support@google.com first.
    #[prost(string, tag = "6")]
    pub dataset: ::prost::alloc::string::String,
    /// Completion max suggestions.
    ///
    /// The maximum allowed max suggestions is 20. The default value is 20.
    #[prost(int32, tag = "5")]
    pub max_suggestions: i32,
}
/// Response of the auto-complete query.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CompleteQueryResponse {
    /// Results of the matching suggestions. The result list is ordered and the
    /// first result is top suggestion.
    #[prost(message, repeated, tag = "1")]
    pub completion_results: ::prost::alloc::vec::Vec<complete_query_response::CompletionResult>,
    /// A unique complete token. This should be included in the
    /// [SearchRequest][google.cloud.retail.v2alpha.SearchRequest] resulting from
    /// this completion, which enables accurate attribution of complete model
    /// performance.
    #[prost(string, tag = "2")]
    pub attribution_token: ::prost::alloc::string::String,
    /// Matched recent searches of this user. This field is a restricted feature.
    /// Contact Retail Support (retail-search-support@google.com) if you are
    /// interested in enabling it.
    ///
    /// This feature is only available when
    /// [CompleteQueryRequest.visitor_id][google.cloud.retail.v2alpha.CompleteQueryRequest.visitor_id]
    /// field is set and [UserEvent][google.cloud.retail.v2alpha.UserEvent] is
    /// imported. The recent searches satisfy the follow rules:
    ///  * They are ordered from latest to oldest.
    ///  * They are matched with
    ///  [CompleteQueryRequest.query][google.cloud.retail.v2alpha.CompleteQueryRequest.query]
    ///  case insensitively.
    ///  * They are transformed to lower cases.
    ///  * They are UTF-8 safe.
    ///
    /// Recent searches are deduplicated. More recent searches will be reserved
    /// when duplication happens.
    #[prost(message, repeated, tag = "3")]
    pub recent_search_results:
        ::prost::alloc::vec::Vec<complete_query_response::RecentSearchResult>,
}
/// Nested message and enum types in `CompleteQueryResponse`.
pub mod complete_query_response {
    /// Resource that represents completion results.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CompletionResult {
        /// The suggestion for the query.
        #[prost(string, tag = "1")]
        pub suggestion: ::prost::alloc::string::String,
        /// Additional custom attributes ingested through BigQuery.
        #[prost(map = "string, message", tag = "2")]
        pub attributes:
            ::std::collections::HashMap<::prost::alloc::string::String, super::CustomAttribute>,
    }
    /// Recent search of this user.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct RecentSearchResult {
        /// The recent search query.
        #[prost(string, tag = "1")]
        pub recent_search: ::prost::alloc::string::String,
    }
}
#[doc = r" Generated client implementations."]
pub mod completion_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Auto-completion service for retail."]
    #[doc = ""]
    #[doc = " This feature is only available for users who have Retail Search enabled."]
    #[doc = " Contact Retail Support (retail-search-support@google.com) if you are"]
    #[doc = " interested in using Retail Search."]
    #[derive(Debug, Clone)]
    pub struct CompletionServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> CompletionServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + Send + Sync + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> CompletionServiceClient<InterceptedService<T, F>>
        where
            F: FnMut(tonic::Request<()>) -> Result<tonic::Request<()>, tonic::Status>,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            CompletionServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        #[doc = r" Compress requests with `gzip`."]
        #[doc = r""]
        #[doc = r" This requires the server to support it otherwise it might respond with an"]
        #[doc = r" error."]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        #[doc = r" Enable decompressing responses with `gzip`."]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        #[doc = " Completes the specified prefix with keyword suggestions."]
        #[doc = ""]
        #[doc = " This feature is only available for users who have Retail Search enabled."]
        #[doc = " Contact Retail Support (retail-search-support@google.com) if you are"]
        #[doc = " interested in using Retail Search."]
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
                "/google.cloud.retail.v2alpha.CompletionService/CompleteQuery",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Bulk import of processed completion dataset."]
        #[doc = ""]
        #[doc = " Request processing may be synchronous. Partial updating is not supported."]
        #[doc = ""]
        #[doc = " This feature is only available for users who have Retail Search enabled."]
        #[doc = " Contact Retail Support (retail-search-support@google.com) if you are"]
        #[doc = " interested in using Retail Search."]
        pub async fn import_completion_data(
            &mut self,
            request: impl tonic::IntoRequest<super::ImportCompletionDataRequest>,
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
                "/google.cloud.retail.v2alpha.CompletionService/ImportCompletionData",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Configuration of destination for Export related errors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportErrorsConfig {
    /// Required. Errors destination.
    #[prost(oneof = "export_errors_config::Destination", tags = "1")]
    pub destination: ::core::option::Option<export_errors_config::Destination>,
}
/// Nested message and enum types in `ExportErrorsConfig`.
pub mod export_errors_config {
    /// Required. Errors destination.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Destination {
        /// Google Cloud Storage path for import errors. This must be an empty,
        /// existing Cloud Storage bucket. Export errors will be written to a file in
        /// this bucket, one per line, as a JSON-encoded
        /// `google.rpc.Status` message.
        #[prost(string, tag = "1")]
        GcsPrefix(::prost::alloc::string::String),
    }
}
/// Metadata related to the progress of the Export operation. This will be
/// returned by the google.longrunning.Operation.metadata field.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportMetadata {
    /// Operation create time.
    #[prost(message, optional, tag = "1")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Operation last update time. If the operation is done, this is also the
    /// finish time.
    #[prost(message, optional, tag = "2")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Response of the ExportProductsRequest. If the long running
/// operation is done, then this message is returned by the
/// google.longrunning.Operations.response field if the operation was successful.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportProductsResponse {
    /// A sample of errors encountered while processing the request.
    #[prost(message, repeated, tag = "1")]
    pub error_samples: ::prost::alloc::vec::Vec<super::super::super::rpc::Status>,
    /// Echoes the destination for the complete errors in the request if set.
    #[prost(message, optional, tag = "2")]
    pub errors_config: ::core::option::Option<ExportErrorsConfig>,
}
/// Response of the ExportUserEventsRequest. If the long running
/// operation was successful, then this message is returned by the
/// google.longrunning.Operations.response field if the operation was successful.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportUserEventsResponse {
    /// A sample of errors encountered while processing the request.
    #[prost(message, repeated, tag = "1")]
    pub error_samples: ::prost::alloc::vec::Vec<super::super::super::rpc::Status>,
    /// Echoes the destination for the complete errors if this field was set in
    /// the request.
    #[prost(message, optional, tag = "2")]
    pub errors_config: ::core::option::Option<ExportErrorsConfig>,
}
/// Request message for Predict method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PredictRequest {
    /// Required. Full resource name of the format:
    /// {name=projects/*/locations/global/catalogs/default_catalog/placements/*}
    /// The ID of the Recommendations AI placement. Before you can request
    /// predictions from your model, you must create at least one placement for it.
    /// For more information, see [Managing
    /// placements](https://cloud.google.com/retail/recommendations-ai/docs/manage-placements).
    ///
    /// The full list of available placements can be seen at
    /// https://console.cloud.google.com/recommendation/catalogs/default_catalog/placements
    #[prost(string, tag = "1")]
    pub placement: ::prost::alloc::string::String,
    /// Required. Context about the user, what they are looking at and what action
    /// they took to trigger the predict request. Note that this user event detail
    /// won't be ingested to userEvent logs. Thus, a separate userEvent write
    /// request is required for event logging.
    #[prost(message, optional, tag = "2")]
    pub user_event: ::core::option::Option<UserEvent>,
    /// Maximum number of results to return per page. Set this property
    /// to the number of prediction results needed. If zero, the service will
    /// choose a reasonable default. The maximum allowed value is 100. Values
    /// above 100 will be coerced to 100.
    #[prost(int32, tag = "3")]
    pub page_size: i32,
    /// The previous PredictResponse.next_page_token.
    #[prost(string, tag = "4")]
    pub page_token: ::prost::alloc::string::String,
    /// Filter for restricting prediction results with a length limit of 5,000
    /// characters. Accepts values for tags and the `filterOutOfStockItems` flag.
    ///
    ///  * Tag expressions. Restricts predictions to products that match all of the
    ///    specified tags. Boolean operators `OR` and `NOT` are supported if the
    ///    expression is enclosed in parentheses, and must be separated from the
    ///    tag values by a space. `-"tagA"` is also supported and is equivalent to
    ///    `NOT "tagA"`. Tag values must be double quoted UTF-8 encoded strings
    ///    with a size limit of 1,000 characters.
    ///
    ///    Note: "Recently viewed" models don't support tag filtering at the
    ///    moment.
    ///
    ///  * filterOutOfStockItems. Restricts predictions to products that do not
    ///  have a
    ///    stockState value of OUT_OF_STOCK.
    ///
    /// Examples:
    ///
    ///  * tag=("Red" OR "Blue") tag="New-Arrival" tag=(NOT "promotional")
    ///  * filterOutOfStockItems  tag=(-"promotional")
    ///  * filterOutOfStockItems
    ///
    /// If your filter blocks all prediction results, nothing will be returned. If
    /// you want generic (unfiltered) popular products to be returned instead, set
    /// `strictFiltering` to false in `PredictRequest.params`.
    #[prost(string, tag = "5")]
    pub filter: ::prost::alloc::string::String,
    /// Use validate only mode for this prediction query. If set to true, a
    /// dummy model will be used that returns arbitrary products.
    /// Note that the validate only mode should only be used for testing the API,
    /// or if the model is not ready.
    #[prost(bool, tag = "6")]
    pub validate_only: bool,
    /// Additional domain specific parameters for the predictions.
    ///
    /// Allowed values:
    ///
    /// * `returnProduct`: Boolean. If set to true, the associated product
    ///    object will be returned in the `results.metadata` field in the
    ///    prediction response.
    /// * `returnScore`: Boolean. If set to true, the prediction 'score'
    ///    corresponding to each returned product will be set in the
    ///    `results.metadata` field in the prediction response. The given
    ///    'score' indicates the probability of an product being clicked/purchased
    ///    given the user's context and history.
    /// * `strictFiltering`: Boolean. True by default. If set to false, the service
    ///    will return generic (unfiltered) popular products instead of empty if
    ///    your filter blocks all prediction results.
    /// * `priceRerankLevel`: String. Default empty. If set to be non-empty, then
    ///    it needs to be one of {'no-price-reranking', 'low-price-reranking',
    ///    'medium-price-reranking', 'high-price-reranking'}. This gives
    ///    request-level control and adjusts prediction results based on product
    ///    price.
    /// * `diversityLevel`: String. Default empty. If set to be non-empty, then
    ///    it needs to be one of {'no-diversity', 'low-diversity',
    ///    'medium-diversity', 'high-diversity', 'auto-diversity'}. This gives
    ///    request-level control and adjusts prediction results based on product
    ///    category.
    #[prost(map = "string, message", tag = "7")]
    pub params: ::std::collections::HashMap<::prost::alloc::string::String, ::prost_types::Value>,
    /// The labels applied to a resource must meet the following requirements:
    ///
    /// * Each resource can have multiple labels, up to a maximum of 64.
    /// * Each label must be a key-value pair.
    /// * Keys have a minimum length of 1 character and a maximum length of 63
    ///   characters, and cannot be empty. Values can be empty, and have a maximum
    ///   length of 63 characters.
    /// * Keys and values can contain only lowercase letters, numeric characters,
    ///   underscores, and dashes. All characters must use UTF-8 encoding, and
    ///   international characters are allowed.
    /// * The key portion of a label must be unique. However, you can use the same
    ///   key with multiple resources.
    /// * Keys must start with a lowercase letter or international character.
    ///
    /// See [Google Cloud
    /// Document](https://cloud.google.com/resource-manager/docs/creating-managing-labels#requirements)
    /// for more details.
    #[prost(map = "string, string", tag = "8")]
    pub labels:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
}
/// Response message for predict method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PredictResponse {
    /// A list of recommended products. The order represents the ranking (from the
    /// most relevant product to the least).
    #[prost(message, repeated, tag = "1")]
    pub results: ::prost::alloc::vec::Vec<predict_response::PredictionResult>,
    /// A unique attribution token. This should be included in the
    /// [UserEvent][google.cloud.retail.v2alpha.UserEvent] logs resulting from this
    /// recommendation, which enables accurate attribution of recommendation model
    /// performance.
    #[prost(string, tag = "2")]
    pub attribution_token: ::prost::alloc::string::String,
    /// IDs of products in the request that were missing from the inventory.
    #[prost(string, repeated, tag = "3")]
    pub missing_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// True if the validateOnly property was set in the request.
    #[prost(bool, tag = "4")]
    pub validate_only: bool,
}
/// Nested message and enum types in `PredictResponse`.
pub mod predict_response {
    /// PredictionResult represents the recommendation prediction results.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PredictionResult {
        /// ID of the recommended product
        #[prost(string, tag = "1")]
        pub id: ::prost::alloc::string::String,
        /// Additional product metadata / annotations.
        ///
        /// Possible values:
        ///
        /// * `product`: JSON representation of the product. Will be set if
        ///   `returnProduct` is set to true in `PredictRequest.params`.
        /// * `score`: Prediction score in double value. Will be set if
        ///   `returnScore` is set to true in `PredictRequest.params`.
        #[prost(map = "string, message", tag = "2")]
        pub metadata:
            ::std::collections::HashMap<::prost::alloc::string::String, ::prost_types::Value>,
    }
}
#[doc = r" Generated client implementations."]
pub mod prediction_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Service for making recommendation prediction."]
    #[derive(Debug, Clone)]
    pub struct PredictionServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> PredictionServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + Send + Sync + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> PredictionServiceClient<InterceptedService<T, F>>
        where
            F: FnMut(tonic::Request<()>) -> Result<tonic::Request<()>, tonic::Status>,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            PredictionServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        #[doc = r" Compress requests with `gzip`."]
        #[doc = r""]
        #[doc = r" This requires the server to support it otherwise it might respond with an"]
        #[doc = r" error."]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        #[doc = r" Enable decompressing responses with `gzip`."]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        #[doc = " Makes a recommendation prediction."]
        pub async fn predict(
            &mut self,
            request: impl tonic::IntoRequest<super::PredictRequest>,
        ) -> Result<tonic::Response<super::PredictResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.retail.v2alpha.PredictionService/Predict",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Metadata related to the progress of the Purge operation.
/// This will be returned by the google.longrunning.Operation.metadata field.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PurgeMetadata {}
/// Request message for PurgeUserEvents method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PurgeUserEventsRequest {
    /// Required. The resource name of the catalog under which the events are
    /// created. The format is
    /// `projects/${projectId}/locations/global/catalogs/${catalogId}`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The filter string to specify the events to be deleted with a
    /// length limit of 5,000 characters. Empty string filter is not allowed. The
    /// eligible fields for filtering are:
    ///
    /// * `eventType`: Double quoted
    /// [UserEvent.event_type][google.cloud.retail.v2alpha.UserEvent.event_type]
    /// string.
    /// * `eventTime`: in ISO 8601 "zulu" format.
    /// * `visitorId`: Double quoted string. Specifying this will delete all
    ///   events associated with a visitor.
    /// * `userId`: Double quoted string. Specifying this will delete all events
    ///   associated with a user.
    ///
    /// Examples:
    ///
    /// * Deleting all events in a time range:
    ///   `eventTime > "2012-04-23T18:25:43.511Z"
    ///   eventTime < "2012-04-23T18:30:43.511Z"`
    /// * Deleting specific eventType in time range:
    ///   `eventTime > "2012-04-23T18:25:43.511Z" eventType = "detail-page-view"`
    /// * Deleting all events for a specific visitor:
    ///   `visitorId = "visitor1024"`
    ///
    /// The filtering fields are assumed to have an implicit AND.
    #[prost(string, tag = "2")]
    pub filter: ::prost::alloc::string::String,
    /// Actually perform the purge.
    /// If `force` is set to false, the method will return the expected purge count
    /// without deleting any user events.
    #[prost(bool, tag = "3")]
    pub force: bool,
}
/// Response of the PurgeUserEventsRequest. If the long running operation is
/// successfully done, then this message is returned by the
/// google.longrunning.Operations.response field.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PurgeUserEventsResponse {
    /// The total count of events purged as a result of the operation.
    #[prost(int64, tag = "1")]
    pub purged_events_count: i64,
}
/// Request message for [CreateProduct][] method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateProductRequest {
    /// Required. The parent catalog resource name, such as
    /// `projects/*/locations/global/catalogs/default_catalog/branches/default_branch`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The [Product][google.cloud.retail.v2alpha.Product] to create.
    #[prost(message, optional, tag = "2")]
    pub product: ::core::option::Option<Product>,
    /// Required. The ID to use for the
    /// [Product][google.cloud.retail.v2alpha.Product], which will become the final
    /// component of the [Product.name][google.cloud.retail.v2alpha.Product.name].
    ///
    /// If the caller does not have permission to create the
    /// [Product][google.cloud.retail.v2alpha.Product], regardless of whether or
    /// not it exists, a PERMISSION_DENIED error is returned.
    ///
    /// This field must be unique among all
    /// [Product][google.cloud.retail.v2alpha.Product]s with the same
    /// [parent][google.cloud.retail.v2alpha.CreateProductRequest.parent].
    /// Otherwise, an ALREADY_EXISTS error is returned.
    ///
    /// This field must be a UTF-8 encoded string with a length limit of 128
    /// characters. Otherwise, an INVALID_ARGUMENT error is returned.
    #[prost(string, tag = "3")]
    pub product_id: ::prost::alloc::string::String,
}
/// Request message for [GetProduct][] method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetProductRequest {
    /// Required. Full resource name of
    /// [Product][google.cloud.retail.v2alpha.Product], such as
    /// `projects/*/locations/global/catalogs/default_catalog/branches/default_branch/products/some_product_id`.
    ///
    /// If the caller does not have permission to access the
    /// [Product][google.cloud.retail.v2alpha.Product], regardless of whether or
    /// not it exists, a PERMISSION_DENIED error is returned.
    ///
    /// If the requested [Product][google.cloud.retail.v2alpha.Product] does not
    /// exist, a NOT_FOUND error is returned.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for [UpdateProduct][] method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateProductRequest {
    /// Required. The product to update/create.
    ///
    /// If the caller does not have permission to update the
    /// [Product][google.cloud.retail.v2alpha.Product], regardless of whether or
    /// not it exists, a PERMISSION_DENIED error is returned.
    ///
    /// If the [Product][google.cloud.retail.v2alpha.Product] to update does not
    /// exist and
    /// [allow_missing][google.cloud.retail.v2alpha.UpdateProductRequest.allow_missing]
    /// is not set, a NOT_FOUND error is returned.
    #[prost(message, optional, tag = "1")]
    pub product: ::core::option::Option<Product>,
    /// Indicates which fields in the provided
    /// [Product][google.cloud.retail.v2alpha.Product] to update. The immutable and
    /// output only fields are NOT supported. If not set, all supported fields (the
    /// fields that are neither immutable nor output only) are updated.
    ///
    /// If an unsupported or unknown field is provided, an INVALID_ARGUMENT error
    /// is returned.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// If set to true, and the [Product][google.cloud.retail.v2alpha.Product] is
    /// not found, a new [Product][google.cloud.retail.v2alpha.Product] will be
    /// created. In this situation, `update_mask` is ignored.
    #[prost(bool, tag = "3")]
    pub allow_missing: bool,
}
/// Request message for [DeleteProduct][] method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteProductRequest {
    /// Required. Full resource name of
    /// [Product][google.cloud.retail.v2alpha.Product], such as
    /// `projects/*/locations/global/catalogs/default_catalog/branches/default_branch/products/some_product_id`.
    ///
    /// If the caller does not have permission to delete the
    /// [Product][google.cloud.retail.v2alpha.Product], regardless of whether or
    /// not it exists, a PERMISSION_DENIED error is returned.
    ///
    /// If the [Product][google.cloud.retail.v2alpha.Product] to delete does not
    /// exist, a NOT_FOUND error is returned.
    ///
    /// The [Product][google.cloud.retail.v2alpha.Product] to delete can neither be
    /// a
    /// [Product.Type.COLLECTION][google.cloud.retail.v2alpha.Product.Type.COLLECTION]
    /// [Product][google.cloud.retail.v2alpha.Product] member nor a
    /// [Product.Type.PRIMARY][google.cloud.retail.v2alpha.Product.Type.PRIMARY]
    /// [Product][google.cloud.retail.v2alpha.Product] with more than one
    /// [variants][google.cloud.retail.v2alpha.Product.Type.VARIANT]. Otherwise, an
    /// INVALID_ARGUMENT error is returned.
    ///
    /// All inventory information for the named
    /// [Product][google.cloud.retail.v2alpha.Product] will be deleted.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for
/// [ProductService.ListProducts][google.cloud.retail.v2alpha.ProductService.ListProducts]
/// method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListProductsRequest {
    /// Required. The parent branch resource name, such as
    /// `projects/*/locations/global/catalogs/default_catalog/branches/0`. Use
    /// `default_branch` as the branch ID, to list products under the default
    /// branch.
    ///
    /// If the caller does not have permission to list
    /// [Product][google.cloud.retail.v2alpha.Product]s under this branch,
    /// regardless of whether or not this branch exists, a PERMISSION_DENIED error
    /// is returned.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Maximum number of [Product][google.cloud.retail.v2alpha.Product]s to
    /// return. If unspecified, defaults to 100. The maximum allowed value is 1000.
    /// Values above 1000 will be coerced to 1000.
    ///
    /// If this field is negative, an INVALID_ARGUMENT error is returned.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A page token
    /// [ListProductsResponse.next_page_token][google.cloud.retail.v2alpha.ListProductsResponse.next_page_token],
    /// received from a previous
    /// [ProductService.ListProducts][google.cloud.retail.v2alpha.ProductService.ListProducts]
    /// call. Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to
    /// [ProductService.ListProducts][google.cloud.retail.v2alpha.ProductService.ListProducts]
    /// must match the call that provided the page token. Otherwise, an
    /// INVALID_ARGUMENT error is returned.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// A filter to apply on the list results. Supported features:
    ///
    /// * List all the products under the parent branch if
    /// [filter][google.cloud.retail.v2alpha.ListProductsRequest.filter] is unset.
    /// * List
    /// [Product.Type.VARIANT][google.cloud.retail.v2alpha.Product.Type.VARIANT]
    /// [Product][google.cloud.retail.v2alpha.Product]s sharing the same
    ///   [Product.Type.PRIMARY][google.cloud.retail.v2alpha.Product.Type.PRIMARY]
    ///   [Product][google.cloud.retail.v2alpha.Product]. For example:
    ///     `primary_product_id = "some_product_id"`
    /// * List [Product][google.cloud.retail.v2alpha.Product]s bundled in a
    /// [Product.Type.COLLECTION][google.cloud.retail.v2alpha.Product.Type.COLLECTION]
    /// [Product][google.cloud.retail.v2alpha.Product].
    ///   For example:
    ///     `collection_product_id = "some_product_id"`
    /// * List [Product][google.cloud.retail.v2alpha.Product]s with a partibular
    /// type. For example:
    ///     `type = "PRIMARY"`
    ///     `type = "VARIANT"`
    ///     `type = "COLLECTION"`
    ///
    /// If the field is unrecognizable, an INVALID_ARGUMENT error is returned.
    ///
    /// If the specified
    /// [Product.Type.PRIMARY][google.cloud.retail.v2alpha.Product.Type.PRIMARY]
    /// [Product][google.cloud.retail.v2alpha.Product] or
    /// [Product.Type.COLLECTION][google.cloud.retail.v2alpha.Product.Type.COLLECTION]
    /// [Product][google.cloud.retail.v2alpha.Product] does not exist, a NOT_FOUND
    /// error is returned.
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// The fields of [Product][google.cloud.retail.v2alpha.Product] to return in
    /// the responses. If not set or empty, the following fields are returned:
    ///
    /// * [Product.name][google.cloud.retail.v2alpha.Product.name]
    /// * [Product.id][google.cloud.retail.v2alpha.Product.id]
    /// * [Product.title][google.cloud.retail.v2alpha.Product.title]
    /// * [Product.uri][google.cloud.retail.v2alpha.Product.uri]
    /// * [Product.images][google.cloud.retail.v2alpha.Product.images]
    /// * [Product.price_info][google.cloud.retail.v2alpha.Product.price_info]
    /// * [Product.brands][google.cloud.retail.v2alpha.Product.brands]
    ///
    /// If "*" is provided, all fields are returned.
    /// [Product.name][google.cloud.retail.v2alpha.Product.name] is always returned
    /// no matter what mask is set.
    ///
    /// If an unsupported or unknown field is provided, an INVALID_ARGUMENT error
    /// is returned.
    #[prost(message, optional, tag = "5")]
    pub read_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// If true and
    /// [page_token][google.cloud.retail.v2alpha.ListProductsRequest.page_token] is
    /// empty,
    /// [ListProductsResponse.total_size][google.cloud.retail.v2alpha.ListProductsResponse.total_size]
    /// is set to the total count of matched items irrespective of pagination.
    ///
    /// Notice that setting this field to true affects the performance.
    #[prost(bool, tag = "6")]
    pub require_total_size: bool,
}
/// Response message for
/// [ProductService.ListProducts][google.cloud.retail.v2alpha.ProductService.ListProducts]
/// method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListProductsResponse {
    /// The [Product][google.cloud.retail.v2alpha.Product]s.
    #[prost(message, repeated, tag = "1")]
    pub products: ::prost::alloc::vec::Vec<Product>,
    /// A token that can be sent as
    /// [ListProductsRequest.page_token][google.cloud.retail.v2alpha.ListProductsRequest.page_token]
    /// to retrieve the next page. If this field is omitted, there are no
    /// subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// The total count of matched [Product][google.cloud.retail.v2alpha.Product]s
    /// irrespective of pagination. The total number of
    /// [Product][google.cloud.retail.v2alpha.Product]s returned by pagination may
    /// be less than the
    /// [total_size][google.cloud.retail.v2alpha.ListProductsResponse.total_size]
    /// that matches.
    ///
    /// This field is ignored if
    /// [ListProductsRequest.require_total_size][google.cloud.retail.v2alpha.ListProductsRequest.require_total_size]
    /// is not set or
    /// [ListProductsRequest.page_token][google.cloud.retail.v2alpha.ListProductsRequest.page_token]
    /// is not empty.
    #[prost(int32, tag = "3")]
    pub total_size: i32,
}
/// Request message for [SetInventory][] method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetInventoryRequest {
    /// Required. The inventory information to update. The allowable fields to
    /// update are:
    /// * [Product.price_info][google.cloud.retail.v2alpha.Product.price_info]
    /// * [Product.availability][google.cloud.retail.v2alpha.Product.availability]
    /// * [Product.available_quantity][google.cloud.retail.v2alpha.Product.available_quantity]
    /// * [Product.fulfillment_info][google.cloud.retail.v2alpha.Product.fulfillment_info]
    /// The updated inventory fields must be specified in
    /// [SetInventoryRequest.set_mask][google.cloud.retail.v2alpha.SetInventoryRequest.set_mask].
    ///
    /// If [SetInventoryRequest.inventory.name][] is empty or invalid, an
    /// INVALID_ARGUMENT error is returned.
    ///
    /// If the caller does not have permission to update the
    /// [Product][google.cloud.retail.v2alpha.Product] named in
    /// [Product.name][google.cloud.retail.v2alpha.Product.name], regardless of
    /// whether or not it exists, a PERMISSION_DENIED error is returned.
    ///
    /// If the [Product][google.cloud.retail.v2alpha.Product] to update does not
    /// have existing inventory information, the provided inventory information
    /// will be inserted.
    ///
    /// If the [Product][google.cloud.retail.v2alpha.Product] to update has
    /// existing inventory information, the provided inventory information will be
    /// merged while respecting the last update time for each inventory field,
    /// using the provided or default value for
    /// [SetInventoryRequest.set_time][google.cloud.retail.v2alpha.SetInventoryRequest.set_time].
    ///
    /// The last update time is recorded for the following inventory fields:
    /// * [Product.price_info][google.cloud.retail.v2alpha.Product.price_info]
    /// * [Product.availability][google.cloud.retail.v2alpha.Product.availability]
    /// * [Product.available_quantity][google.cloud.retail.v2alpha.Product.available_quantity]
    /// * [Product.fulfillment_info][google.cloud.retail.v2alpha.Product.fulfillment_info]
    ///
    /// If a full overwrite of inventory information while ignoring timestamps is
    /// needed, [UpdateProduct][] should be invoked instead.
    #[prost(message, optional, tag = "1")]
    pub inventory: ::core::option::Option<Product>,
    /// Indicates which inventory fields in the provided
    /// [Product][google.cloud.retail.v2alpha.Product] to update. If not set or set
    /// with empty paths, all inventory fields will be updated.
    ///
    /// If an unsupported or unknown field is provided, an INVALID_ARGUMENT error
    /// is returned and the entire update will be ignored.
    #[prost(message, optional, tag = "2")]
    pub set_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// The time when the request is issued, used to prevent
    /// out-of-order updates on inventory fields with the last update time
    /// recorded. If not provided, the internal system time will be used.
    #[prost(message, optional, tag = "3")]
    pub set_time: ::core::option::Option<::prost_types::Timestamp>,
    /// If set to true, and the [Product][google.cloud.retail.v2alpha.Product] with
    /// name [Product.name][google.cloud.retail.v2alpha.Product.name] is not found,
    /// the inventory update will still be processed and retained for at most 1 day
    /// until the [Product][google.cloud.retail.v2alpha.Product] is created. If set
    /// to false, an INVALID_ARGUMENT error is returned if the
    /// [Product][google.cloud.retail.v2alpha.Product] is not found.
    #[prost(bool, tag = "4")]
    pub allow_missing: bool,
}
/// Metadata related to the progress of the SetInventory operation.
/// Currently empty because there is no meaningful metadata populated from the
/// [SetInventory][] method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetInventoryMetadata {}
/// Response of the SetInventoryRequest.  Currently empty because
/// there is no meaningful response populated from the [SetInventory][]
/// method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetInventoryResponse {}
/// Request message for [AddFulfillmentPlaces][] method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddFulfillmentPlacesRequest {
    /// Required. Full resource name of
    /// [Product][google.cloud.retail.v2alpha.Product], such as
    /// `projects/*/locations/global/catalogs/default_catalog/branches/default_branch/products/some_product_id`.
    ///
    /// If the caller does not have permission to access the
    /// [Product][google.cloud.retail.v2alpha.Product], regardless of whether or
    /// not it exists, a PERMISSION_DENIED error is returned.
    #[prost(string, tag = "1")]
    pub product: ::prost::alloc::string::String,
    /// Required. The fulfillment type, including commonly used types (such as
    /// pickup in store and same day delivery), and custom types.
    ///
    /// Supported values:
    ///
    /// * "pickup-in-store"
    /// * "ship-to-store"
    /// * "same-day-delivery"
    /// * "next-day-delivery"
    /// * "custom-type-1"
    /// * "custom-type-2"
    /// * "custom-type-3"
    /// * "custom-type-4"
    /// * "custom-type-5"
    ///
    /// If this field is set to an invalid value other than these, an
    /// INVALID_ARGUMENT error is returned.
    ///
    /// This field directly corresponds to [Product.fulfillment_info.type][].
    #[prost(string, tag = "2")]
    pub r#type: ::prost::alloc::string::String,
    /// Required. The IDs for this
    /// [type][google.cloud.retail.v2alpha.AddFulfillmentPlacesRequest.type], such
    /// as the store IDs for "pickup-in-store" or the region IDs for
    /// "same-day-delivery" to be added for this
    /// [type][google.cloud.retail.v2alpha.AddFulfillmentPlacesRequest.type].
    /// Duplicate IDs will be automatically ignored.
    ///
    /// At least 1 value is required, and a maximum of 2000 values are allowed.
    /// Each value must be a string with a length limit of 10 characters, matching
    /// the pattern [a-zA-Z0-9_-]+, such as "store1" or "REGION-2". Otherwise, an
    /// INVALID_ARGUMENT error is returned.
    ///
    /// If the total number of place IDs exceeds 2000 for this
    /// [type][google.cloud.retail.v2alpha.AddFulfillmentPlacesRequest.type] after
    /// adding, then the update will be rejected.
    #[prost(string, repeated, tag = "3")]
    pub place_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The time when the fulfillment updates are issued, used to prevent
    /// out-of-order updates on fulfillment information. If not provided, the
    /// internal system time will be used.
    #[prost(message, optional, tag = "4")]
    pub add_time: ::core::option::Option<::prost_types::Timestamp>,
    /// If set to true, and the [Product][google.cloud.retail.v2alpha.Product] is
    /// not found, the fulfillment information will still be processed and retained
    /// for at most 1 day and processed once the
    /// [Product][google.cloud.retail.v2alpha.Product] is created. If set to false,
    /// an INVALID_ARGUMENT error is returned if the
    /// [Product][google.cloud.retail.v2alpha.Product] is not found.
    #[prost(bool, tag = "5")]
    pub allow_missing: bool,
}
/// Metadata related to the progress of the AddFulfillmentPlaces operation.
/// Currently empty because there is no meaningful metadata populated from the
/// [AddFulfillmentPlaces][] method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddFulfillmentPlacesMetadata {}
/// Response of the RemoveFulfillmentPlacesRequest.  Currently empty because
/// there is no meaningful response populated from the [AddFulfillmentPlaces][]
/// method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddFulfillmentPlacesResponse {}
/// Request message for [RemoveFulfillmentPlaces][] method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveFulfillmentPlacesRequest {
    /// Required. Full resource name of
    /// [Product][google.cloud.retail.v2alpha.Product], such as
    /// `projects/*/locations/global/catalogs/default_catalog/branches/default_branch/products/some_product_id`.
    ///
    /// If the caller does not have permission to access the
    /// [Product][google.cloud.retail.v2alpha.Product], regardless of whether or
    /// not it exists, a PERMISSION_DENIED error is returned.
    #[prost(string, tag = "1")]
    pub product: ::prost::alloc::string::String,
    /// Required. The fulfillment type, including commonly used types (such as
    /// pickup in store and same day delivery), and custom types.
    ///
    /// Supported values:
    ///
    /// * "pickup-in-store"
    /// * "ship-to-store"
    /// * "same-day-delivery"
    /// * "next-day-delivery"
    /// * "custom-type-1"
    /// * "custom-type-2"
    /// * "custom-type-3"
    /// * "custom-type-4"
    /// * "custom-type-5"
    ///
    /// If this field is set to an invalid value other than these, an
    /// INVALID_ARGUMENT error is returned.
    ///
    /// This field directly corresponds to [Product.fulfillment_info.type][].
    #[prost(string, tag = "2")]
    pub r#type: ::prost::alloc::string::String,
    /// Required. The IDs for this
    /// [type][google.cloud.retail.v2alpha.RemoveFulfillmentPlacesRequest.type],
    /// such as the store IDs for "pickup-in-store" or the region IDs for
    /// "same-day-delivery", to be removed for this
    /// [type][google.cloud.retail.v2alpha.RemoveFulfillmentPlacesRequest.type].
    ///
    /// At least 1 value is required, and a maximum of 2000 values are allowed.
    /// Each value must be a string with a length limit of 10 characters, matching
    /// the pattern [a-zA-Z0-9_-]+, such as "store1" or "REGION-2". Otherwise, an
    /// INVALID_ARGUMENT error is returned.
    #[prost(string, repeated, tag = "3")]
    pub place_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The time when the fulfillment updates are issued, used to prevent
    /// out-of-order updates on fulfillment information. If not provided, the
    /// internal system time will be used.
    #[prost(message, optional, tag = "4")]
    pub remove_time: ::core::option::Option<::prost_types::Timestamp>,
    /// If set to true, and the [Product][google.cloud.retail.v2alpha.Product] is
    /// not found, the fulfillment information will still be processed and retained
    /// for at most 1 day and processed once the
    /// [Product][google.cloud.retail.v2alpha.Product] is created. If set to false,
    /// an INVALID_ARGUMENT error is returned if the
    /// [Product][google.cloud.retail.v2alpha.Product] is not found.
    #[prost(bool, tag = "5")]
    pub allow_missing: bool,
}
/// Metadata related to the progress of the RemoveFulfillmentPlaces operation.
/// Currently empty because there is no meaningful metadata populated from the
/// [RemoveFulfillmentPlaces][] method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveFulfillmentPlacesMetadata {}
/// Response of the RemoveFulfillmentPlacesRequest. Currently empty because there
/// is no meaningful response populated from the [RemoveFulfillmentPlaces][]
/// method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveFulfillmentPlacesResponse {}
#[doc = r" Generated client implementations."]
pub mod product_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Service for ingesting [Product][google.cloud.retail.v2alpha.Product]"]
    #[doc = " information of the customer's website."]
    #[derive(Debug, Clone)]
    pub struct ProductServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> ProductServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + Send + Sync + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> ProductServiceClient<InterceptedService<T, F>>
        where
            F: FnMut(tonic::Request<()>) -> Result<tonic::Request<()>, tonic::Status>,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            ProductServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        #[doc = r" Compress requests with `gzip`."]
        #[doc = r""]
        #[doc = r" This requires the server to support it otherwise it might respond with an"]
        #[doc = r" error."]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        #[doc = r" Enable decompressing responses with `gzip`."]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        #[doc = " Creates a [Product][google.cloud.retail.v2alpha.Product]."]
        pub async fn create_product(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateProductRequest>,
        ) -> Result<tonic::Response<super::Product>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.retail.v2alpha.ProductService/CreateProduct",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets a [Product][google.cloud.retail.v2alpha.Product]."]
        pub async fn get_product(
            &mut self,
            request: impl tonic::IntoRequest<super::GetProductRequest>,
        ) -> Result<tonic::Response<super::Product>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.retail.v2alpha.ProductService/GetProduct",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets a list of [Product][google.cloud.retail.v2alpha.Product]s."]
        pub async fn list_products(
            &mut self,
            request: impl tonic::IntoRequest<super::ListProductsRequest>,
        ) -> Result<tonic::Response<super::ListProductsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.retail.v2alpha.ProductService/ListProducts",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates a [Product][google.cloud.retail.v2alpha.Product]."]
        pub async fn update_product(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateProductRequest>,
        ) -> Result<tonic::Response<super::Product>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.retail.v2alpha.ProductService/UpdateProduct",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes a [Product][google.cloud.retail.v2alpha.Product]."]
        pub async fn delete_product(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteProductRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.retail.v2alpha.ProductService/DeleteProduct",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Bulk import of multiple [Product][google.cloud.retail.v2alpha.Product]s."]
        #[doc = ""]
        #[doc = " Request processing may be synchronous. No partial updating is supported."]
        #[doc = " Non-existing items are created."]
        #[doc = ""]
        #[doc = " Note that it is possible for a subset of the"]
        #[doc = " [Product][google.cloud.retail.v2alpha.Product]s to be successfully updated."]
        pub async fn import_products(
            &mut self,
            request: impl tonic::IntoRequest<super::ImportProductsRequest>,
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
                "/google.cloud.retail.v2alpha.ProductService/ImportProducts",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates inventory information for a"]
        #[doc = " [Product][google.cloud.retail.v2alpha.Product] while respecting the last"]
        #[doc = " update timestamps of each inventory field."]
        #[doc = ""]
        #[doc = " This process is asynchronous and does not require the"]
        #[doc = " [Product][google.cloud.retail.v2alpha.Product] to exist before updating"]
        #[doc = " fulfillment information. If the request is valid, the update will be"]
        #[doc = " enqueued and processed downstream. As a consequence, when a response is"]
        #[doc = " returned, updates are not immediately manifested in the"]
        #[doc = " [Product][google.cloud.retail.v2alpha.Product] queried by"]
        #[doc = " [GetProduct][google.cloud.retail.v2alpha.ProductService.GetProduct] or"]
        #[doc = " [ListProducts][google.cloud.retail.v2alpha.ProductService.ListProducts]."]
        #[doc = ""]
        #[doc = " When inventory is updated with"]
        #[doc = " [CreateProduct][google.cloud.retail.v2alpha.ProductService.CreateProduct]"]
        #[doc = " and"]
        #[doc = " [UpdateProduct][google.cloud.retail.v2alpha.ProductService.UpdateProduct],"]
        #[doc = " the specified inventory field value(s) will overwrite any existing value(s)"]
        #[doc = " while ignoring the last update time for this field. Furthermore, the last"]
        #[doc = " update time for the specified inventory fields will be overwritten to the"]
        #[doc = " time of the"]
        #[doc = " [CreateProduct][google.cloud.retail.v2alpha.ProductService.CreateProduct]"]
        #[doc = " or"]
        #[doc = " [UpdateProduct][google.cloud.retail.v2alpha.ProductService.UpdateProduct]"]
        #[doc = " request."]
        #[doc = ""]
        #[doc = " If no inventory fields are set in"]
        #[doc = " [CreateProductRequest.product][google.cloud.retail.v2alpha.CreateProductRequest.product],"]
        #[doc = " then any pre-existing inventory information for this product will be used."]
        #[doc = ""]
        #[doc = " If no inventory fields are set in [UpdateProductRequest.set_mask][],"]
        #[doc = " then any existing inventory information will be preserved."]
        #[doc = ""]
        #[doc = " Pre-existing inventory information can only be updated with"]
        #[doc = " [SetInventory][google.cloud.retail.v2alpha.ProductService.SetInventory],"]
        #[doc = " [AddFulfillmentPlaces][google.cloud.retail.v2alpha.ProductService.AddFulfillmentPlaces],"]
        #[doc = " and"]
        #[doc = " [RemoveFulfillmentPlaces][google.cloud.retail.v2alpha.ProductService.RemoveFulfillmentPlaces]."]
        #[doc = ""]
        #[doc = " This feature is only available for users who have Retail Search enabled."]
        #[doc = " Contact Retail Support (retail-search-support@google.com) if you are"]
        #[doc = " interested in using Retail Search."]
        pub async fn set_inventory(
            &mut self,
            request: impl tonic::IntoRequest<super::SetInventoryRequest>,
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
                "/google.cloud.retail.v2alpha.ProductService/SetInventory",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Incrementally adds place IDs to"]
        #[doc = " [Product.fulfillment_info.place_ids][google.cloud.retail.v2alpha.FulfillmentInfo.place_ids]."]
        #[doc = ""]
        #[doc = " This process is asynchronous and does not require the"]
        #[doc = " [Product][google.cloud.retail.v2alpha.Product] to exist before updating"]
        #[doc = " fulfillment information. If the request is valid, the update will be"]
        #[doc = " enqueued and processed downstream. As a consequence, when a response is"]
        #[doc = " returned, the added place IDs are not immediately manifested in the"]
        #[doc = " [Product][google.cloud.retail.v2alpha.Product] queried by"]
        #[doc = " [GetProduct][google.cloud.retail.v2alpha.ProductService.GetProduct] or"]
        #[doc = " [ListProducts][google.cloud.retail.v2alpha.ProductService.ListProducts]."]
        #[doc = ""]
        #[doc = " This feature is only available for users who have Retail Search enabled."]
        #[doc = " Contact Retail Support (retail-search-support@google.com) if you are"]
        #[doc = " interested in using Retail Search."]
        pub async fn add_fulfillment_places(
            &mut self,
            request: impl tonic::IntoRequest<super::AddFulfillmentPlacesRequest>,
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
                "/google.cloud.retail.v2alpha.ProductService/AddFulfillmentPlaces",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Incrementally removes place IDs from a"]
        #[doc = " [Product.fulfillment_info.place_ids][google.cloud.retail.v2alpha.FulfillmentInfo.place_ids]."]
        #[doc = ""]
        #[doc = " This process is asynchronous and does not require the"]
        #[doc = " [Product][google.cloud.retail.v2alpha.Product] to exist before updating"]
        #[doc = " fulfillment information. If the request is valid, the update will be"]
        #[doc = " enqueued and processed downstream. As a consequence, when a response is"]
        #[doc = " returned, the removed place IDs are not immediately manifested in the"]
        #[doc = " [Product][google.cloud.retail.v2alpha.Product] queried by"]
        #[doc = " [GetProduct][google.cloud.retail.v2alpha.ProductService.GetProduct] or"]
        #[doc = " [ListProducts][google.cloud.retail.v2alpha.ProductService.ListProducts]."]
        #[doc = ""]
        #[doc = " This feature is only available for users who have Retail Search enabled."]
        #[doc = " Contact Retail Support (retail-search-support@google.com) if you are"]
        #[doc = " interested in using Retail Search."]
        pub async fn remove_fulfillment_places(
            &mut self,
            request: impl tonic::IntoRequest<super::RemoveFulfillmentPlacesRequest>,
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
                "/google.cloud.retail.v2alpha.ProductService/RemoveFulfillmentPlaces",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Request message for
/// [SearchService.Search][google.cloud.retail.v2alpha.SearchService.Search]
/// method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchRequest {
    /// Required. The resource name of the search engine placement, such as
    /// `projects/*/locations/global/catalogs/default_catalog/placements/default_search`.
    /// This field is used to identify the set of models that will be used to make
    /// the search.
    ///
    /// We currently support one placement with the following ID:
    ///
    /// * `default_search`.
    #[prost(string, tag = "1")]
    pub placement: ::prost::alloc::string::String,
    /// The branch resource name, such as
    /// `projects/*/locations/global/catalogs/default_catalog/branches/0`.
    ///
    /// Use "default_branch" as the branch ID or leave this field empty, to search
    /// products under the default branch.
    #[prost(string, tag = "2")]
    pub branch: ::prost::alloc::string::String,
    /// Raw search query.
    #[prost(string, tag = "3")]
    pub query: ::prost::alloc::string::String,
    /// Required. A unique identifier for tracking visitors. For example, this
    /// could be implemented with an HTTP cookie, which should be able to uniquely
    /// identify a visitor on a single device. This unique identifier should not
    /// change if the visitor logs in or out of the website.
    ///
    /// The field must be a UTF-8 encoded string with a length limit of 128
    /// characters. Otherwise, an INVALID_ARGUMENT error is returned.
    #[prost(string, tag = "4")]
    pub visitor_id: ::prost::alloc::string::String,
    /// User information.
    #[prost(message, optional, tag = "5")]
    pub user_info: ::core::option::Option<UserInfo>,
    /// Maximum number of [Product][google.cloud.retail.v2alpha.Product]s to
    /// return. If unspecified, defaults to a reasonable value. The maximum allowed
    /// value is 120. Values above 120 will be coerced to 120.
    ///
    /// If this field is negative, an INVALID_ARGUMENT is returned.
    #[prost(int32, tag = "7")]
    pub page_size: i32,
    /// A page token
    /// [SearchResponse.next_page_token][google.cloud.retail.v2alpha.SearchResponse.next_page_token],
    /// received from a previous
    /// [SearchService.Search][google.cloud.retail.v2alpha.SearchService.Search]
    /// call. Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to
    /// [SearchService.Search][google.cloud.retail.v2alpha.SearchService.Search]
    /// must match the call that provided the page token. Otherwise, an
    /// INVALID_ARGUMENT error is returned.
    #[prost(string, tag = "8")]
    pub page_token: ::prost::alloc::string::String,
    /// A 0-indexed integer that specifies the current offset (that is, starting
    /// result location, amongst the
    /// [Product][google.cloud.retail.v2alpha.Product]s deemed by the API as
    /// relevant) in search results. This field is only considered if
    /// [page_token][google.cloud.retail.v2alpha.SearchRequest.page_token] is
    /// unset.
    ///
    /// If this field is negative, an INVALID_ARGUMENT is returned.
    #[prost(int32, tag = "9")]
    pub offset: i32,
    /// The filter syntax consists of an expression language for constructing a
    /// predicate from one or more fields of the products being filtered. Filter
    /// expression is case-sensitive. See more details at this [user
    /// guide](/retail/private/docs/filter-and-order#filter).
    ///
    /// If this field is unrecognizable, an INVALID_ARGUMENT is returned.
    #[prost(string, tag = "10")]
    pub filter: ::prost::alloc::string::String,
    /// The filter applied to every search request when quality improvement such as
    /// query expansion is needed. For example, if a query does not have enough
    /// results, an expanded query with
    /// [SearchRequest.canonical_filter][google.cloud.retail.v2alpha.SearchRequest.canonical_filter]
    /// will be returned as a supplement of the original query. This field is
    /// strongly recommended to achieve high search quality.
    ///
    /// See
    /// [SearchRequest.filter][google.cloud.retail.v2alpha.SearchRequest.filter]
    /// for more details about filter syntax.
    #[prost(string, tag = "28")]
    pub canonical_filter: ::prost::alloc::string::String,
    /// The order in which products are returned. Products can be ordered by
    /// a field in an [Product][google.cloud.retail.v2alpha.Product] object. Leave
    /// it unset if ordered by relevance. OrderBy expression is case-sensitive. See
    /// more details at this [user
    /// guide](/retail/private/docs/filter-and-order#order).
    ///
    /// If this field is unrecognizable, an INVALID_ARGUMENT is returned.
    #[prost(string, tag = "11")]
    pub order_by: ::prost::alloc::string::String,
    /// Facet specifications for faceted search. If empty, no facets are returned.
    ///
    /// A maximum of 100 values are allowed. Otherwise, an INVALID_ARGUMENT error
    /// is returned.
    #[prost(message, repeated, tag = "12")]
    pub facet_specs: ::prost::alloc::vec::Vec<search_request::FacetSpec>,
    /// The specification for dynamically generated facets. Notice that only
    /// textual facets can be dynamically generated.
    ///
    /// This feature requires additional allowlisting. Contact Retail Support
    /// (retail-search-support@google.com) if you are interested in using dynamic
    /// facet feature.
    #[prost(message, optional, tag = "21")]
    pub dynamic_facet_spec: ::core::option::Option<search_request::DynamicFacetSpec>,
    /// Boost specification to boost certain products. See more details at this
    /// [user guide](/retail/private/docs/boosting).
    #[prost(message, optional, tag = "13")]
    pub boost_spec: ::core::option::Option<search_request::BoostSpec>,
    /// The query expansion specification that specifies the conditions under which
    /// query expansion will occur. See more details at this [user
    /// guide](/retail/private/docs/result-size#query_expansion).
    #[prost(message, optional, tag = "14")]
    pub query_expansion_spec: ::core::option::Option<search_request::QueryExpansionSpec>,
    /// The relevance threshold of the search results.
    ///
    /// Defaults to
    /// [RelevanceThreshold.HIGH][google.cloud.retail.v2alpha.SearchRequest.RelevanceThreshold.HIGH],
    /// which means only the most relevant results are shown, and the least number
    /// of results are returned. See more details at this [user
    /// guide](/retail/private/docs/result-size#relevance_thresholding).
    #[prost(enumeration = "search_request::RelevanceThreshold", tag = "15")]
    pub relevance_threshold: i32,
    /// The keys to fetch and rollup the matching
    /// [variant][google.cloud.retail.v2alpha.Product.Type.VARIANT]
    /// [Product][google.cloud.retail.v2alpha.Product]s attributes. The attributes
    /// from all the matching
    /// [variant][google.cloud.retail.v2alpha.Product.Type.VARIANT]
    /// [Product][google.cloud.retail.v2alpha.Product]s are merged and
    /// de-duplicated. Notice that rollup
    /// [variant][google.cloud.retail.v2alpha.Product.Type.VARIANT]
    /// [Product][google.cloud.retail.v2alpha.Product]s attributes will lead to
    /// extra query latency. Maximum number of keys is 10.
    ///
    /// For
    /// [Product.fulfillment_info][google.cloud.retail.v2alpha.Product.fulfillment_info],
    /// a fulfillment type and a fulfillment ID must be provided in the format of
    /// "fulfillmentType.filfillmentId". E.g., in "pickupInStore.store123",
    /// "pickupInStore" is fulfillment type and "store123" is the store ID.
    ///
    /// Supported keys are:
    ///
    /// * colorFamilies
    /// * price
    /// * originalPrice
    /// * discount
    /// * attributes.key, where key is any key in the
    ///   [Product.attributes][google.cloud.retail.v2alpha.Product.attributes] map.
    /// * pickupInStore.id, where id is any [FulfillmentInfo.ids][] for type
    ///   [FulfillmentInfo.Type.PICKUP_IN_STORE][].
    /// * shipToStore.id, where id is any [FulfillmentInfo.ids][] for type
    ///   [FulfillmentInfo.Type.SHIP_TO_STORE][].
    /// * sameDayDelivery.id, where id is any [FulfillmentInfo.ids][] for type
    ///   [FulfillmentInfo.Type.SAME_DAY_DELIVERY][].
    /// * nextDayDelivery.id, where id is any [FulfillmentInfo.ids][] for type
    ///   [FulfillmentInfo.Type.NEXT_DAY_DELIVERY][].
    /// * customFulfillment1.id, where id is any [FulfillmentInfo.ids][] for type
    ///   [FulfillmentInfo.Type.CUSTOM_TYPE_1][].
    /// * customFulfillment2.id, where id is any [FulfillmentInfo.ids][] for type
    ///   [FulfillmentInfo.Type.CUSTOM_TYPE_2][].
    /// * customFulfillment3.id, where id is any [FulfillmentInfo.ids][] for type
    ///   [FulfillmentInfo.Type.CUSTOM_TYPE_3][].
    /// * customFulfillment4.id, where id is any [FulfillmentInfo.ids][] for type
    ///   [FulfillmentInfo.Type.CUSTOM_TYPE_4][].
    /// * customFulfillment5.id, where id is any [FulfillmentInfo.ids][] for type
    ///   [FulfillmentInfo.Type.CUSTOM_TYPE_5][].
    ///
    /// If this field is set to an invalid value other than these, an
    /// INVALID_ARGUMENT error is returned.
    #[prost(string, repeated, tag = "17")]
    pub variant_rollup_keys: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The categories associated with a category page. Required for category
    /// navigation queries to achieve good search quality. The format should be
    /// the same as
    /// [UserEvent.page_categories][google.cloud.retail.v2alpha.UserEvent.page_categories];
    ///
    /// To represent full path of category, use '>' sign to separate different
    /// hierarchies. If '>' is part of the category name, please replace it with
    /// other character(s).
    ///
    /// Category pages include special pages such as sales or promotions. For
    /// instance, a special sale page may have the category hierarchy:
    /// "pageCategories" : ["Sales > 2017 Black Friday Deals"].
    #[prost(string, repeated, tag = "23")]
    pub page_categories: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Nested message and enum types in `SearchRequest`.
pub mod search_request {
    /// A facet specification to perform faceted search.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct FacetSpec {
        /// Required. The facet key specification.
        #[prost(message, optional, tag = "1")]
        pub facet_key: ::core::option::Option<facet_spec::FacetKey>,
        /// Maximum of facet values that should be returned for this facet. If
        /// unspecified, defaults to 20. The maximum allowed value is 300. Values
        /// above 300 will be coerced to 300.
        ///
        /// If this field is negative, an INVALID_ARGUMENT is returned.
        #[prost(int32, tag = "2")]
        pub limit: i32,
        /// List of keys to exclude when faceting.
        ///
        /// By default,
        /// [FacetKey.key][google.cloud.retail.v2alpha.SearchRequest.FacetSpec.FacetKey.key]
        /// is not excluded from the filter unless it is listed in this field.
        ///
        /// For example, suppose there are 100 products with color facet "Red" and
        /// 200 products with color facet "Blue". A query containing the filter
        /// "colorFamilies:ANY("Red")" and have "colorFamilies" as
        /// [FacetKey.key][google.cloud.retail.v2alpha.SearchRequest.FacetSpec.FacetKey.key]
        /// will by default return the "Red" with count 100.
        ///
        /// If this field contains "colorFamilies", then the query returns both the
        /// "Red" with count 100 and "Blue" with count 200, because the
        /// "colorFamilies" key is now excluded from the filter.
        ///
        /// A maximum of 100 values are allowed. Otherwise, an INVALID_ARGUMENT error
        /// is returned.
        #[prost(string, repeated, tag = "3")]
        pub excluded_filter_keys: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// Enables dynamic position for this facet. If set to true, the position of
        /// this facet among all facets in the response is determined by Google
        /// Retail Search. It will be ordered together with dynamic facets if dynamic
        /// facets is enabled. If set to false, the position of this facet in the
        /// response will be the same as in the request, and it will be ranked before
        /// the facets with dynamic position enable and all dynamic facets.
        ///
        /// For example, you may always want to have rating facet returned in
        /// the response, but it's not necessarily to always display the rating facet
        /// at the top. In that case, you can set enable_dynamic_position to true so
        /// that the position of rating facet in response will be determined by
        /// Google Retail Search.
        ///
        /// Another example, assuming you have the following facets in the request:
        ///
        /// * "rating", enable_dynamic_position = true
        ///
        /// * "price", enable_dynamic_position = false
        ///
        /// * "brands", enable_dynamic_position = false
        ///
        /// And also you have a dynamic facets enable, which will generate a facet
        /// 'gender'. Then the final order of the facets in the response can be
        /// ("price", "brands", "rating", "gender") or ("price", "brands", "gender",
        /// "rating") depends on how Google Retail Search orders "gender" and
        /// "rating" facets. However, notice that "price" and "brands" will always be
        /// ranked at 1st and 2nd position since their enable_dynamic_position are
        /// false.
        #[prost(bool, tag = "4")]
        pub enable_dynamic_position: bool,
    }
    /// Nested message and enum types in `FacetSpec`.
    pub mod facet_spec {
        /// Specifies how a facet is computed.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct FacetKey {
            /// Required. Supported textual and numerical facet keys in
            /// [Product][google.cloud.retail.v2alpha.Product] object, over which the
            /// facet values are computed. Facet key is case-sensitive.
            ///
            /// Allowed facet keys when
            /// [FacetKey.query][google.cloud.retail.v2alpha.SearchRequest.FacetSpec.FacetKey.query]
            /// is not specified:
            ///
            /// * textual_field =<br>
            ///     <font color='grey'>
            ///     *# The
            ///     [Product.brands][google.cloud.retail.v2alpha.Product.brands].<br>*
            ///     </font>
            ///     "brands";
            ///     <br>
            ///     <font color='categories'>
            ///     *# The
            ///     [Product.categories][google.cloud.retail.v2alpha.Product.categories].<br>*
            ///     </font>
            ///     "categories";
            ///     <br>
            ///     <font color='grey'>
            ///     *# The
            ///     [Audience.genders][google.cloud.retail.v2alpha.Audience.genders].<br>*
            ///     </font>
            ///     | "genders";
            ///     <br>
            ///     <font color='grey'>
            ///     *# The
            ///     [Audience.age_groups][google.cloud.retail.v2alpha.Audience.age_groups].<br>*
            ///     </font>
            ///     | "ageGroups";
            ///     <br>
            ///     <font color='grey'>
            ///     *# The
            ///     [Product.availability][google.cloud.retail.v2alpha.Product.availability].
            ///     Value is one of<br>*
            ///     *# "IN_STOCK", "OUT_OF_STOCK", PREORDER", "BACKORDER".<br>*
            ///     </font>
            ///     | "availability";
            ///     <br>
            ///     <font color='grey'>
            ///     *# The
            ///     [ColorInfo.color_families][google.cloud.retail.v2alpha.ColorInfo.color_families].<br>*
            ///     </font>
            ///     | "colorFamilies";
            ///     <br>
            ///     <font color='grey'>
            ///     *# The
            ///     [ColorInfo.colors][google.cloud.retail.v2alpha.ColorInfo.colors].<br>*
            ///     </font>
            ///     | "colors";
            ///     <br>
            ///     <font color='grey'>
            ///     *# The
            ///     [Product.sizes][google.cloud.retail.v2alpha.Product.sizes].<br>*
            ///     </font>
            ///     | "sizes";
            ///     <br>
            ///     <font color='grey'>
            ///     *# The
            ///     [Product.materials][google.cloud.retail.v2alpha.Product.materials].<br>*
            ///     </font>
            ///     | "materials";
            ///     <br>
            ///     <font color='grey'>
            ///     *# The
            ///     [Product.patterns][google.cloud.retail.v2alpha.Product.patterns].<br>*
            ///     </font>
            ///     | "patterns";
            ///     <br>
            ///     <font color='grey'>
            ///     *# The
            ///     [Product.conditions][google.cloud.retail.v2alpha.Product.conditions].<br>*
            ///     </font>
            ///     | "conditions";
            ///     <br>
            ///     <font color='grey'>
            ///     *# The textual custom attribute in
            ///     [Product][google.cloud.retail.v2alpha.Product] object. Key can<br>*
            ///     *# be any key in the
            ///     [Product.attributes][google.cloud.retail.v2alpha.Product.attributes]
            ///     map<br>*
            ///     *# if the attribute values are textual.<br>*
            ///     *# map.<br>*
            ///     </font>
            ///     | "attributes.key";
            ///     <br>
            ///     <font color='grey'>
            ///     *# The [FulfillmentInfo.ids][] for type
            ///     *# [FulfillmentInfo.Type.PICKUP_IN_STORE][].<br>*
            ///     </font>
            ///     | "pickupInStore";
            ///     <br>
            ///     <font color='grey'>
            ///     *# The [FulfillmentInfo.ids][] for type
            ///     *# [FulfillmentInfo.Type.SHIP_TO_STORE][].<br>*
            ///     </font>
            ///     | "shipToStore";
            ///     <br>
            ///     <font color='grey'>
            ///     *# The [FulfillmentInfo.ids][] for type
            ///     *# [FulfillmentInfo.Type.SAME_DAY_DELIVERY][].<br>*
            ///     </font>
            ///     | "sameDayDelivery";
            ///     <br>
            ///     <font color='grey'>
            ///     *# The [FulfillmentInfo.ids][] for type
            ///     *# [FulfillmentInfo.Type.NEXT_DAY_DELIVERY][].<br>*
            ///     </font>
            ///     | "nextDayDelivery";
            ///     <br>
            ///     <font color='grey'>
            ///     *# The [FulfillmentInfo.ids][] for type
            ///     *# [FulfillmentInfo.Type.CUSTOM_TYPE_1][].<br>*
            ///     </font>
            ///     | "customFulfillment1";
            ///     <br>
            ///     <font color='grey'>
            ///     *# The [FulfillmentInfo.ids][] for type
            ///     *# [FulfillmentInfo.Type.CUSTOM_TYPE_2][].<br>*
            ///     </font>
            ///     | "customFulfillment2";
            ///     <br>
            ///     <font color='grey'>
            ///     *# The [FulfillmentInfo.ids][] for type
            ///     *# [FulfillmentInfo.Type.CUSTOM_TYPE_3][].<br>*
            ///     </font>
            ///     | "customFulfillment3";
            ///     <br>
            ///     <font color='grey'>
            ///     *# The [FulfillmentInfo.ids][] for type
            ///     *# [FulfillmentInfo.Type.CUSTOM_TYPE_4][].<br>*
            ///     </font>
            ///     | "customFulfillment4";
            ///     <br>
            ///     <font color='grey'>
            ///     *# The [FulfillmentInfo.ids][] for type
            ///     *# [FulfillmentInfo.Type.CUSTOM_TYPE_5][].<br>*
            ///     </font>
            ///     | "customFulfillment5";
            ///
            /// * numerical_field =<br>
            ///     <font color='grey'>
            ///     *# The
            ///     [PriceInfo.price][google.cloud.retail.v2alpha.PriceInfo.price].<br>*
            ///     </font>
            ///     "price";
            ///     <br>
            ///     <font color='grey'>
            ///     *# The discount. Computed by (original_price-price)/price <br>*
            ///     </font>
            ///     "discount";
            ///     <br>
            ///     <font color='grey'>
            ///     *# The
            ///     [Rating.average_rating][google.cloud.retail.v2alpha.Rating.average_rating].<br>*
            ///     </font>
            ///     "rating";
            ///     <br>
            ///     <font color='grey'>
            ///     *# The
            ///     [Rating.rating_count][google.cloud.retail.v2alpha.Rating.rating_count].<br>*
            ///     </font>
            ///     "ratingCount";
            ///     <br>
            ///     <font color='grey'>
            ///     *# The numerical custom attribute in
            ///     [Product][google.cloud.retail.v2alpha.Product] object. Key can<br>*
            ///     *# be any key in the
            ///     [Product.attributes][google.cloud.retail.v2alpha.Product.attributes]
            ///     map<br>*
            ///     *# if the attribute values are numerical.<br>*
            ///     </font>
            ///     | "attributes.key";
            #[prost(string, tag = "1")]
            pub key: ::prost::alloc::string::String,
            /// Set only if values should be bucketized into intervals. Must be set
            /// for facets with numerical values. Must not be set for facet with text
            /// values. Maximum number of intervals is 30.
            #[prost(message, repeated, tag = "2")]
            pub intervals: ::prost::alloc::vec::Vec<super::super::Interval>,
            /// Only get facet for the given restricted values. For example, when using
            /// "pickupInStore" as key and set restricted values to
            /// ["store123", "store456"], only facets for "store123" and "store456" are
            /// returned. Only supported on textual fields and fulfillments.
            /// Maximum is 20.
            ///
            /// Must be set for the fulfillment facet keys:
            ///
            /// * pickupInStore
            ///
            /// * shipToStore
            ///
            /// * sameDayDelivery
            ///
            /// * nextDayDelivery
            ///
            /// * customFulfillment1
            ///
            /// * customFulfillment2
            ///
            /// * customFulfillment3
            ///
            /// * customFulfillment4
            ///
            /// * customFulfillment5
            #[prost(string, repeated, tag = "3")]
            pub restricted_values: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
            /// Only get facet values that start with the given string prefix. For
            /// example, suppose "categories" has three values "Women > Shoe",
            /// "Women > Dress" and "Men > Shoe". If set "prefixes" to "Women", the
            /// "categories" facet will give only "Women > Shoe" and "Women > Dress".
            /// Only supported on textual fields. Maximum is 10.
            #[prost(string, repeated, tag = "8")]
            pub prefixes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
            /// Only get facet values that contains the given strings. For example,
            /// suppose "categories" has three values "Women > Shoe",
            /// "Women > Dress" and "Men > Shoe". If set "contains" to "Shoe", the
            /// "categories" facet will give only "Women > Shoe" and "Men > Shoe".
            /// Only supported on textual fields. Maximum is 10.
            #[prost(string, repeated, tag = "9")]
            pub contains: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
            /// The order in which [Facet.values][] are returned.
            ///
            /// Allowed values are:
            ///
            /// * "count desc", which means order by [Facet.FacetValue.count][]
            /// descending.
            ///
            /// * "value desc", which means order by [Facet.FacetValue.value][]
            /// descending.
            ///   Only applies to textual facets.
            ///
            /// If not set, textual values are sorted in [natural
            /// order](https://en.wikipedia.org/wiki/Natural_sort_order); numerical
            /// intervals are sorted in the order given by
            /// [FacetSpec.FacetKey.intervals][google.cloud.retail.v2alpha.SearchRequest.FacetSpec.FacetKey.intervals];
            /// [FulfillmentInfo.ids][] are sorted in the order given by
            /// [FacetSpec.FacetKey.restricted_values][google.cloud.retail.v2alpha.SearchRequest.FacetSpec.FacetKey.restricted_values].
            #[prost(string, tag = "4")]
            pub order_by: ::prost::alloc::string::String,
            /// The query that is used to compute facet for the given facet key.
            /// When provided, it will override the default behavior of facet
            /// computation. The query syntax is the same as a filter expression. See
            /// [SearchRequest.filter][google.cloud.retail.v2alpha.SearchRequest.filter]
            /// for detail syntax and limitations. Notice that there is no limitation
            /// on
            /// [FacetKey.key][google.cloud.retail.v2alpha.SearchRequest.FacetSpec.FacetKey.key]
            /// when query is specified.
            ///
            /// In the response, [FacetValue.value][] will be always "1" and
            /// [FacetValue.count][] will be the number of results that matches the
            /// query.
            ///
            /// For example, you can set a customized facet for "shipToStore",
            /// where
            /// [FacetKey.key][google.cloud.retail.v2alpha.SearchRequest.FacetSpec.FacetKey.key]
            /// is "customizedShipToStore", and
            /// [FacetKey.query][google.cloud.retail.v2alpha.SearchRequest.FacetSpec.FacetKey.query]
            /// is "availability: ANY(\"IN_STOCK\") AND shipToStore: ANY(\"123\")".
            /// Then the facet will count the products that are both in stock and ship
            /// to store "123".
            #[prost(string, tag = "5")]
            pub query: ::prost::alloc::string::String,
        }
    }
    /// The specifications of dynamically generated facets.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DynamicFacetSpec {
        /// Mode of the DynamicFacet feature.
        /// Defaults to
        /// [Mode.DISABLED][google.cloud.retail.v2alpha.SearchRequest.DynamicFacetSpec.Mode.DISABLED]
        /// if it's unset.
        #[prost(enumeration = "dynamic_facet_spec::Mode", tag = "1")]
        pub mode: i32,
    }
    /// Nested message and enum types in `DynamicFacetSpec`.
    pub mod dynamic_facet_spec {
        /// Enum to control DynamicFacet mode
        #[derive(
            Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration,
        )]
        #[repr(i32)]
        pub enum Mode {
            /// Default value.
            Unspecified = 0,
            /// Disable Dynamic Facet.
            Disabled = 1,
            /// Automatic mode built by Google Retail Search.
            Enabled = 2,
        }
    }
    /// Boost specification to boost certain items.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct BoostSpec {
        /// Condition boost specifications. If a product matches multiple conditions
        /// in the specifictions, boost scores from these specifications are all
        /// applied and combined in a non-linear way. Maximum number of
        /// specifications is 10.
        #[prost(message, repeated, tag = "1")]
        pub condition_boost_specs: ::prost::alloc::vec::Vec<boost_spec::ConditionBoostSpec>,
    }
    /// Nested message and enum types in `BoostSpec`.
    pub mod boost_spec {
        /// Boost applies to products which match a condition.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ConditionBoostSpec {
            /// An expression which specifies a boost condition. The syntax and
            /// supported fields are the same as a filter expression. See
            /// [SearchRequest.filter][google.cloud.retail.v2alpha.SearchRequest.filter]
            /// for detail syntax and limitations.
            ///
            /// Examples:
            ///
            /// * To boost products with product ID "product_1" or "product_2", and
            /// color
            ///   "Red" or "Blue":<br>
            ///   *(id: ANY("product_1", "product_2"))<br>*
            ///   *AND<br>*
            ///   *(colorFamilies: ANY("Red", "Blue"))<br>*
            #[prost(string, tag = "1")]
            pub condition: ::prost::alloc::string::String,
            /// Strength of the condition boost, which should be in [-1, 1]. Negative
            /// boost means demotion. Default is 0.0.
            ///
            /// Setting to 1.0 gives the item a big promotion. However, it does not
            /// necessarily mean that the boosted item will be the top result at all
            /// times, nor that other items will be excluded. Results could still be
            /// shown even when none of them matches the condition. And results that
            /// are significantly more relevant to the search query can still trump
            /// your heavily favored but irrelevant items.
            ///
            /// Setting to -1.0 gives the item a big demotion. However, results that
            /// are deeply relevant might still be shown. The item will have an
            /// upstream battle to get a fairly high ranking, but it is not blocked out
            /// completely.
            ///
            /// Setting to 0.0 means no boost applied. The boosting condition is
            /// ignored.
            #[prost(float, tag = "2")]
            pub boost: f32,
        }
    }
    /// Specification to determine under which conditions query expansion should
    /// occur.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct QueryExpansionSpec {
        /// The condition under which query expansion should occur. Default to
        /// [Condition.DISABLED][google.cloud.retail.v2alpha.SearchRequest.QueryExpansionSpec.Condition.DISABLED].
        #[prost(enumeration = "query_expansion_spec::Condition", tag = "1")]
        pub condition: i32,
    }
    /// Nested message and enum types in `QueryExpansionSpec`.
    pub mod query_expansion_spec {
        /// Enum describing under which condition query expansion should occur.
        #[derive(
            Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration,
        )]
        #[repr(i32)]
        pub enum Condition {
            /// Unspecified query expansion condition. This defaults to
            /// [Condition.DISABLED][google.cloud.retail.v2alpha.SearchRequest.QueryExpansionSpec.Condition.DISABLED].
            Unspecified = 0,
            /// Disabled query expansion. Only the exact search query is used, even if
            /// [SearchResponse.total_size][google.cloud.retail.v2alpha.SearchResponse.total_size]
            /// is zero.
            Disabled = 1,
            /// Automatic query expansion built by Google Retail Search.
            Auto = 3,
        }
    }
    /// The relevance threshold of the search results. The higher relevance
    /// threshold is, the higher relevant results are shown and the less number of
    /// results are returned.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum RelevanceThreshold {
        /// Default value. Defaults to
        /// [RelevanceThreshold.HIGH][google.cloud.retail.v2alpha.SearchRequest.RelevanceThreshold.HIGH].
        Unspecified = 0,
        /// High relevance threshold.
        High = 1,
        /// Medium relevance threshold.
        Medium = 2,
        /// Low relevance threshold.
        Low = 3,
        /// Lowest relevance threshold.
        Lowest = 4,
    }
}
/// Response message for
/// [SearchService.Search][google.cloud.retail.v2alpha.SearchService.Search]
/// method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchResponse {
    /// A list of matched items. The order represents the ranking.
    #[prost(message, repeated, tag = "1")]
    pub results: ::prost::alloc::vec::Vec<search_response::SearchResult>,
    /// Results of facets requested by user.
    #[prost(message, repeated, tag = "2")]
    pub facets: ::prost::alloc::vec::Vec<search_response::Facet>,
    /// The estimated total count of matched items irrespective of pagination. The
    /// count of [results][google.cloud.retail.v2alpha.SearchResponse.results]
    /// returned by pagination may be less than the
    /// [total_size][google.cloud.retail.v2alpha.SearchResponse.total_size] that
    /// matches.
    #[prost(int32, tag = "3")]
    pub total_size: i32,
    /// If spell correction applies, the corrected query. Otherwise, empty.
    #[prost(string, tag = "4")]
    pub corrected_query: ::prost::alloc::string::String,
    /// A unique search token. This should be included in the
    /// [UserEvent][google.cloud.retail.v2alpha.UserEvent] logs resulting from this
    /// search, which enables accurate attribution of search model performance.
    #[prost(string, tag = "5")]
    pub attribution_token: ::prost::alloc::string::String,
    /// A token that can be sent as
    /// [SearchRequest.page_token][google.cloud.retail.v2alpha.SearchRequest.page_token]
    /// to retrieve the next page. If this field is omitted, there are no
    /// subsequent pages.
    #[prost(string, tag = "6")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Query expansion information for the returned results.
    #[prost(message, optional, tag = "7")]
    pub query_expansion_info: ::core::option::Option<search_response::QueryExpansionInfo>,
    /// The URI of a customer-defined redirect page. If redirect action is
    /// triggered, no search will be performed, and only
    /// [redirect_uri][google.cloud.retail.v2alpha.SearchResponse.redirect_uri] and
    /// [attribution_token][google.cloud.retail.v2alpha.SearchResponse.attribution_token]
    /// will be set in the response.
    #[prost(string, tag = "10")]
    pub redirect_uri: ::prost::alloc::string::String,
}
/// Nested message and enum types in `SearchResponse`.
pub mod search_response {
    /// Represents the search results.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SearchResult {
        /// [Product.id][google.cloud.retail.v2alpha.Product.id] of the searched
        /// [Product][google.cloud.retail.v2alpha.Product].
        #[prost(string, tag = "1")]
        pub id: ::prost::alloc::string::String,
        /// The product data snippet in the search response. Only
        /// [Product.name][google.cloud.retail.v2alpha.Product.name] is guaranteed to
        /// be populated.
        ///
        /// [Product.variants][google.cloud.retail.v2alpha.Product.variants] contains
        /// the product variants that match the search query. If there are multiple
        /// product variants matching the query, top 5 most relevant product variants
        /// are returned and ordered by relevancy.
        ///
        /// If relevancy can be deternmined, use
        /// [matching_variant_fields][google.cloud.retail.v2alpha.SearchResponse.SearchResult.matching_variant_fields]
        /// to look up matched product variants fields. If relevancy cannot be
        /// determined, e.g. when searching "shoe" all products in a shoe product can
        /// be a match, 5 product variants are returned but order is meaningless.
        #[prost(message, optional, tag = "2")]
        pub product: ::core::option::Option<super::Product>,
        /// The count of matched
        /// [variant][google.cloud.retail.v2alpha.Product.Type.VARIANT]
        /// [Product][google.cloud.retail.v2alpha.Product]s.
        #[prost(int32, tag = "3")]
        pub matching_variant_count: i32,
        /// If a [variant][google.cloud.retail.v2alpha.Product.Type.VARIANT]
        /// [Product][google.cloud.retail.v2alpha.Product] matches the search query,
        /// this map indicates which [Product][google.cloud.retail.v2alpha.Product]
        /// fields are matched. The key is the
        /// [Product.name][google.cloud.retail.v2alpha.Product.name], the value is a
        /// field mask of the matched [Product][google.cloud.retail.v2alpha.Product]
        /// fields. If matched attributes cannot be determined, this map will be
        /// empty.
        ///
        /// For example, a key "sku1" with field mask
        /// "products.color_info" indicates there is a match between
        /// "sku1" [ColorInfo][google.cloud.retail.v2alpha.ColorInfo] and the query.
        #[prost(map = "string, message", tag = "4")]
        pub matching_variant_fields:
            ::std::collections::HashMap<::prost::alloc::string::String, ::prost_types::FieldMask>,
        /// The rollup matching
        /// [variant][google.cloud.retail.v2alpha.Product.Type.VARIANT]
        /// [Product][google.cloud.retail.v2alpha.Product] attributes. The key is one
        /// of the
        /// [SearchRequest.variant_rollup_keys][google.cloud.retail.v2alpha.SearchRequest.variant_rollup_keys].
        /// The values are the merged and de-duplicated
        /// [Product][google.cloud.retail.v2alpha.Product] attributes. Notice that
        /// the rollup values are respect filter. For example, when filtering by
        /// "colorFamilies:ANY(\"red\")" and rollup "colorFamilies", only "red" is
        /// returned.
        ///
        /// For textual and numerical attributes, the rollup values is a list of
        /// string or double values with type
        /// [google.protobuf.ListValue][google.protobuf.ListValue]. For example, if
        /// there are two variants with colors "red" and "blue", the rollup values
        /// are { key: "colorFamilies"
        ///   value {
        ///     list_value {
        ///       values { string_value: "red" }
        ///       values { string_value: "blue" }
        ///      }
        ///   }
        /// }
        ///
        /// For
        /// [Product.fulfillment_info][google.cloud.retail.v2alpha.Product.fulfillment_info],
        /// the rollup values is a double value with type
        /// [google.protobuf.Value][google.protobuf.Value]. For example, {key:
        /// "pickupInStore.store1" value { number_value: 10 }} means a there are 10
        /// variants in this product are available in the store "store1".
        #[prost(map = "string, message", tag = "5")]
        pub variant_rollup_values:
            ::std::collections::HashMap<::prost::alloc::string::String, ::prost_types::Value>,
    }
    /// A facet result.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Facet {
        /// The key for this facet. E.g., "colorFamilies" or "price" or
        /// "attributes.attr1".
        #[prost(string, tag = "1")]
        pub key: ::prost::alloc::string::String,
        /// The facet values for this field.
        #[prost(message, repeated, tag = "2")]
        pub values: ::prost::alloc::vec::Vec<facet::FacetValue>,
        /// Whether the facet is dynamically generated.
        #[prost(bool, tag = "3")]
        pub dynamic_facet: bool,
    }
    /// Nested message and enum types in `Facet`.
    pub mod facet {
        /// A facet value which contains value names and their count.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct FacetValue {
            /// Number of items that have this facet value.
            #[prost(int64, tag = "3")]
            pub count: i64,
            /// A facet value which contains values.
            #[prost(oneof = "facet_value::FacetValue", tags = "1, 2")]
            pub facet_value: ::core::option::Option<facet_value::FacetValue>,
        }
        /// Nested message and enum types in `FacetValue`.
        pub mod facet_value {
            /// A facet value which contains values.
            #[derive(Clone, PartialEq, ::prost::Oneof)]
            pub enum FacetValue {
                /// Text value of a facet, such as "Black" for facet "colorFamilies".
                #[prost(string, tag = "1")]
                Value(::prost::alloc::string::String),
                /// Interval value for a facet, such as [10, 20) for facet "price".
                #[prost(message, tag = "2")]
                Interval(super::super::super::Interval),
            }
        }
    }
    /// Information describing query expansion including whether expansion has
    /// occurred.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct QueryExpansionInfo {
        /// Bool describing whether query expansion has occurred.
        #[prost(bool, tag = "1")]
        pub expanded_query: bool,
    }
}
#[doc = r" Generated client implementations."]
pub mod search_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Service for search."]
    #[doc = ""]
    #[doc = " This feature is only available for users who have Retail Search enabled."]
    #[doc = " Contact Retail Support (retail-search-support@google.com) if you are"]
    #[doc = " interested in using Retail Search."]
    #[derive(Debug, Clone)]
    pub struct SearchServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> SearchServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + Send + Sync + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> SearchServiceClient<InterceptedService<T, F>>
        where
            F: FnMut(tonic::Request<()>) -> Result<tonic::Request<()>, tonic::Status>,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            SearchServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        #[doc = r" Compress requests with `gzip`."]
        #[doc = r""]
        #[doc = r" This requires the server to support it otherwise it might respond with an"]
        #[doc = r" error."]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        #[doc = r" Enable decompressing responses with `gzip`."]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        #[doc = " Performs a search."]
        #[doc = ""]
        #[doc = " This feature is only available for users who have Retail Search enabled."]
        #[doc = " Contact Retail Support (retail-search-support@google.com) if you are"]
        #[doc = " interested in using Retail Search."]
        pub async fn search(
            &mut self,
            request: impl tonic::IntoRequest<super::SearchRequest>,
        ) -> Result<tonic::Response<super::SearchResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.retail.v2alpha.SearchService/Search",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Request message for WriteUserEvent method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WriteUserEventRequest {
    /// Required. The parent catalog resource name, such as
    /// `projects/1234/locations/global/catalogs/default_catalog`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. User event to write.
    #[prost(message, optional, tag = "2")]
    pub user_event: ::core::option::Option<UserEvent>,
}
/// Request message for CollectUserEvent method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CollectUserEventRequest {
    /// Required. The parent catalog name, such as
    /// `projects/1234/locations/global/catalogs/default_catalog`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. URL encoded UserEvent proto with a length limit of 2,000,000
    /// characters.
    #[prost(string, tag = "2")]
    pub user_event: ::prost::alloc::string::String,
    /// The URL including cgi-parameters but excluding the hash fragment with a
    /// length limit of 5,000 characters. This is often more useful than the
    /// referer URL, because many browsers only send the domain for 3rd party
    /// requests.
    #[prost(string, tag = "3")]
    pub uri: ::prost::alloc::string::String,
    /// The event timestamp in milliseconds. This prevents browser caching of
    /// otherwise identical get requests. The name is abbreviated to reduce the
    /// payload bytes.
    #[prost(int64, tag = "4")]
    pub ets: i64,
}
/// Request message for RejoinUserEvents method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RejoinUserEventsRequest {
    /// Required. The parent catalog resource name, such as
    /// `projects/1234/locations/global/catalogs/default_catalog`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The type of the user event rejoin to define the scope and range of the user
    /// events to be rejoined with the latest product catalog. Defaults to
    /// USER_EVENT_REJOIN_SCOPE_UNSPECIFIED if this field is not set, or set to an
    /// invalid integer value.
    #[prost(
        enumeration = "rejoin_user_events_request::UserEventRejoinScope",
        tag = "2"
    )]
    pub user_event_rejoin_scope: i32,
}
/// Nested message and enum types in `RejoinUserEventsRequest`.
pub mod rejoin_user_events_request {
    /// The scope of user events to be rejoined with the latest product catalog.
    /// If the rejoining aims at reducing number of unjoined events, set
    /// UserEventRejoinScope to UNJOINED_EVENTS.
    /// If the rejoining aims at correcting product catalog information in joined
    /// events, set UserEventRejoinScope to JOINED_EVENTS.
    /// If all events needs to be rejoined, set UserEventRejoinScope to
    /// USER_EVENT_REJOIN_SCOPE_UNSPECIFIED.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum UserEventRejoinScope {
        /// Rejoin all events with the latest product catalog, including both joined
        /// events and unjoined events.
        Unspecified = 0,
        /// Only rejoin joined events with the latest product catalog.
        JoinedEvents = 1,
        /// Only rejoin unjoined events with the latest product catalog.
        UnjoinedEvents = 2,
    }
}
/// Response message for RejoinUserEvents method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RejoinUserEventsResponse {
    /// Number of user events that were joined with latest product catalog.
    #[prost(int64, tag = "1")]
    pub rejoined_user_events_count: i64,
}
/// Metadata for RejoinUserEvents method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RejoinUserEventsMetadata {}
#[doc = r" Generated client implementations."]
pub mod user_event_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Service for ingesting end user actions on the customer website."]
    #[derive(Debug, Clone)]
    pub struct UserEventServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> UserEventServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + Send + Sync + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> UserEventServiceClient<InterceptedService<T, F>>
        where
            F: FnMut(tonic::Request<()>) -> Result<tonic::Request<()>, tonic::Status>,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            UserEventServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        #[doc = r" Compress requests with `gzip`."]
        #[doc = r""]
        #[doc = r" This requires the server to support it otherwise it might respond with an"]
        #[doc = r" error."]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        #[doc = r" Enable decompressing responses with `gzip`."]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        #[doc = " Writes a single user event."]
        pub async fn write_user_event(
            &mut self,
            request: impl tonic::IntoRequest<super::WriteUserEventRequest>,
        ) -> Result<tonic::Response<super::UserEvent>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.retail.v2alpha.UserEventService/WriteUserEvent",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Writes a single user event from the browser. This uses a GET request to"]
        #[doc = " due to browser restriction of POST-ing to a 3rd party domain."]
        #[doc = ""]
        #[doc = " This method is used only by the Retail API JavaScript pixel and Google Tag"]
        #[doc = " Manager. Users should not call this method directly."]
        pub async fn collect_user_event(
            &mut self,
            request: impl tonic::IntoRequest<super::CollectUserEventRequest>,
        ) -> Result<tonic::Response<super::super::super::super::api::HttpBody>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.retail.v2alpha.UserEventService/CollectUserEvent",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes permanently all user events specified by the filter provided."]
        #[doc = " Depending on the number of events specified by the filter, this operation"]
        #[doc = " could take hours or days to complete. To test a filter, use the list"]
        #[doc = " command first."]
        pub async fn purge_user_events(
            &mut self,
            request: impl tonic::IntoRequest<super::PurgeUserEventsRequest>,
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
                "/google.cloud.retail.v2alpha.UserEventService/PurgeUserEvents",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Bulk import of User events. Request processing might be"]
        #[doc = " synchronous. Events that already exist are skipped."]
        #[doc = " Use this method for backfilling historical user events."]
        #[doc = ""]
        #[doc = " Operation.response is of type ImportResponse. Note that it is"]
        #[doc = " possible for a subset of the items to be successfully inserted."]
        #[doc = " Operation.metadata is of type ImportMetadata."]
        pub async fn import_user_events(
            &mut self,
            request: impl tonic::IntoRequest<super::ImportUserEventsRequest>,
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
                "/google.cloud.retail.v2alpha.UserEventService/ImportUserEvents",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Triggers a user event rejoin operation with latest product catalog. Events"]
        #[doc = " will not be annotated with detailed product information if product is"]
        #[doc = " missing from the catalog at the time the user event is ingested, and these"]
        #[doc = " events are stored as unjoined events with a limited usage on training and"]
        #[doc = " serving. This API can be used to trigger a 'join' operation on specified"]
        #[doc = " events with latest version of product catalog. It can also be used to"]
        #[doc = " correct events joined with wrong product catalog."]
        pub async fn rejoin_user_events(
            &mut self,
            request: impl tonic::IntoRequest<super::RejoinUserEventsRequest>,
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
                "/google.cloud.retail.v2alpha.UserEventService/RejoinUserEvents",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
