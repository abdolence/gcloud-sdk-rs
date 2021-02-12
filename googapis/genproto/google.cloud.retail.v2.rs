/// Configures what level the product should be uploaded with regards to
/// how users will be send events and how predictions will be made.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProductLevelConfig {
    /// The type of [Product][google.cloud.retail.v2.Product]s allowed to be
    /// ingested into the catalog. Acceptable values are:
    ///
    /// * `primary` (default): You can only ingest
    /// [Product.Type.PRIMARY][google.cloud.retail.v2.Product.Type.PRIMARY]
    ///   [Product][google.cloud.retail.v2.Product]s. This means
    ///   [Product.primary_product_id][google.cloud.retail.v2.Product.primary_product_id]
    ///   can only be empty or set to the same value as
    ///   [Product.id][google.cloud.retail.v2.Product.id].
    /// * `variant`: You can only ingest
    /// [Product.Type.VARIANT][google.cloud.retail.v2.Product.Type.VARIANT]
    /// [Product][google.cloud.retail.v2.Product]s.
    ///   This means
    ///   [Product.primary_product_id][google.cloud.retail.v2.Product.primary_product_id]
    ///   cannot be empty.
    ///
    /// If this field is set to an invalid value other than these, an
    /// INVALID_ARGUMENT error is returned.
    ///
    /// If this field is `variant` and
    /// [merchant_center_product_id_field][google.cloud.retail.v2.ProductLevelConfig.merchant_center_product_id_field]
    /// is `itemGroupId`, an INVALID_ARGUMENT error is returned.
    ///
    /// See [Using catalog
    /// levels](/retail/recommendations-ai/docs/catalog#catalog-levels) for more
    /// details.
    #[prost(string, tag = "1")]
    pub ingestion_product_type: ::prost::alloc::string::String,
    /// Which field of [Merchant Center
    /// Product](/bigquery-transfer/docs/merchant-center-products-schema) should be
    /// imported as [Product.id][google.cloud.retail.v2.Product.id]. Acceptable
    /// values are:
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
    /// [ingestion_product_type][google.cloud.retail.v2.ProductLevelConfig.ingestion_product_type]
    /// is `variant`, an INVALID_ARGUMENT error is returned.
    ///
    /// See [Using catalog
    /// levels](/retail/recommendations-ai/docs/catalog#catalog-levels) for more
    /// details.
    #[prost(string, tag = "2")]
    pub merchant_center_product_id_field: ::prost::alloc::string::String,
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
}
/// Request for
/// [CatalogService.ListCatalogs][google.cloud.retail.v2.CatalogService.ListCatalogs]
/// method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCatalogsRequest {
    /// Required. The account resource name with an associated location.
    ///
    /// If the caller does not have permission to list
    /// [Catalog][google.cloud.retail.v2.Catalog]s under this location, regardless
    /// of whether or not this location exists, a PERMISSION_DENIED error is
    /// returned.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Maximum number of [Catalog][google.cloud.retail.v2.Catalog]s to return. If
    /// unspecified, defaults to 50. The maximum allowed value is 1000. Values
    /// above 1000 will be coerced to 1000.
    ///
    /// If this field is negative, an INVALID_ARGUMENT is returned.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A page token
    /// [ListCatalogsResponse.next_page_token][google.cloud.retail.v2.ListCatalogsResponse.next_page_token],
    /// received from a previous
    /// [CatalogService.ListCatalogs][google.cloud.retail.v2.CatalogService.ListCatalogs]
    /// call. Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to
    /// [CatalogService.ListCatalogs][google.cloud.retail.v2.CatalogService.ListCatalogs]
    /// must match the call that provided the page token. Otherwise, an
    /// INVALID_ARGUMENT error is returned.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response for
/// [CatalogService.ListCatalogs][google.cloud.retail.v2.CatalogService.ListCatalogs]
/// method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCatalogsResponse {
    /// All the customer's [Catalog][google.cloud.retail.v2.Catalog]s.
    #[prost(message, repeated, tag = "1")]
    pub catalogs: ::prost::alloc::vec::Vec<Catalog>,
    /// A token that can be sent as
    /// [ListCatalogsRequest.page_token][google.cloud.retail.v2.ListCatalogsRequest.page_token]
    /// to retrieve the next page. If this field is omitted, there are no
    /// subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request for
/// [CatalogService.UpdateCatalog][google.cloud.retail.v2.CatalogService.UpdateCatalog]
/// method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateCatalogRequest {
    /// Required. The [Catalog][google.cloud.retail.v2.Catalog] to update.
    ///
    /// If the caller does not have permission to update the
    /// [Catalog][google.cloud.retail.v2.Catalog], regardless of whether or not it
    /// exists, a PERMISSION_DENIED error is returned.
    ///
    /// If the [Catalog][google.cloud.retail.v2.Catalog] to update does not exist,
    /// a NOT_FOUND error is returned.
    #[prost(message, optional, tag = "1")]
    pub catalog: ::core::option::Option<Catalog>,
    /// Indicates which fields in the provided
    /// [Catalog][google.cloud.retail.v2.Catalog] to update. If not set, will only
    /// update the
    /// [Catalog.product_level_config][google.cloud.retail.v2.Catalog.product_level_config]
    /// field, which is also the only currently supported field to update.
    ///
    /// If an unsupported or unknown field is provided, an INVALID_ARGUMENT error
    /// is returned.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
#[doc = r" Generated client implementations."]
pub mod catalog_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Service for managing catalog configuration."]
    pub struct CatalogServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> CatalogServiceClient<T>
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
        #[doc = " Lists all the [Catalog][google.cloud.retail.v2.Catalog]s associated with"]
        #[doc = " the project."]
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
                "/google.cloud.retail.v2.CatalogService/ListCatalogs",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates the [Catalog][google.cloud.retail.v2.Catalog]s."]
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
                "/google.cloud.retail.v2.CatalogService/UpdateCatalog",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for CatalogServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for CatalogServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "CatalogServiceClient {{ ... }}")
        }
    }
}
/// A custom attribute that is not explicitly modeled in
/// [Product][google.cloud.retail.v2.Product]].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomAttribute {
    /// The textual values of this custom attribute. For example, `["yellow",
    /// "green"]` when the key is "color".
    ///
    /// At most 400 values are allowed. Empty values are not allowed. Each value
    /// must be a UTF-8 encoded string with a length limit of 256 characters.
    /// Otherwise, an INVALID_ARGUMENT error is returned.
    ///
    /// Exactly one of [text][google.cloud.retail.v2.CustomAttribute.text] or
    /// [numbers][google.cloud.retail.v2.CustomAttribute.numbers] should be set.
    /// Otherwise, an INVALID_ARGUMENT error is returned.
    #[prost(string, repeated, tag = "1")]
    pub text: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The numerical values of this custom attribute. For example, `[2.3, 15.4]`
    /// when the key is "lengths_cm".
    ///
    /// At most 400 values are allowed.Otherwise, an INVALID_ARGUMENT error is
    /// returned.
    ///
    /// Exactly one of [text][google.cloud.retail.v2.CustomAttribute.text] or
    /// [numbers][google.cloud.retail.v2.CustomAttribute.numbers] should be set.
    /// Otherwise, an INVALID_ARGUMENT error is returned.
    #[prost(double, repeated, tag = "2")]
    pub numbers: ::prost::alloc::vec::Vec<f64>,
}
/// [Product][google.cloud.retail.v2.Product] thumbnail/detail image.
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
/// The price information of a [Product][google.cloud.retail.v2.Product].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PriceInfo {
    /// The 3-letter currency code defined in [ISO
    /// 4217](https://www.iso.org/iso-4217-currency-codes.html).
    ///
    /// If this field is an unrecognizable currency code, an INVALID_ARGUMENT
    /// error is returned.
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
    /// the [price][google.cloud.retail.v2.PriceInfo.price].
    #[prost(float, tag = "3")]
    pub original_price: f32,
    /// The costs associated with the sale of a particular product. Used for gross
    /// profit reporting.
    ///
    /// * Profit = [price][google.cloud.retail.v2.PriceInfo.price] -
    /// [cost][google.cloud.retail.v2.PriceInfo.cost]
    ///
    /// Google Merchant Center property
    /// [cost_of_goods_sold](https://support.google.com/merchants/answer/9017895).
    #[prost(float, tag = "4")]
    pub cost: f32,
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
    /// The end user's IP address. This field is used to extract location
    /// information for personalization.
    ///
    /// This field must be either an IPv4 address (e.g. "104.133.9.80") or an IPv6
    /// address (e.g. "2001:0db8:85a3:0000:0000:8a2e:0370:7334"). Otherwise, an
    /// INVALID_ARGUMENT error is returned.
    ///
    /// This should not be set when using the JavaScript tag in
    /// [UserEventService.CollectUserEvent][google.cloud.retail.v2.UserEventService.CollectUserEvent]
    /// or if
    /// [direct_user_request][google.cloud.retail.v2.UserInfo.direct_user_request]
    /// is set.
    #[prost(string, tag = "2")]
    pub ip_address: ::prost::alloc::string::String,
    /// User agent as included in the HTTP header.
    ///
    /// The field must be a UTF-8 encoded string with a length limit of 1,000
    /// characters. Otherwise, an INVALID_ARGUMENT error is returned.
    ///
    /// This should not be set when using the client side event reporting with
    /// GTM or JavaScript tag in
    /// [UserEventService.CollectUserEvent][google.cloud.retail.v2.UserEventService.CollectUserEvent]
    /// or if
    /// [direct_user_request][google.cloud.retail.v2.UserInfo.direct_user_request]
    /// is set.
    #[prost(string, tag = "3")]
    pub user_agent: ::prost::alloc::string::String,
    /// True if the request is made directly from the end user, in which case the
    /// [ip_address][google.cloud.retail.v2.UserInfo.ip_address] and
    /// [user_agent][google.cloud.retail.v2.UserInfo.user_agent] can be populated
    /// from the HTTP request. This flag should be set only if the API request is
    /// made directly from the end user such as a mobile app (and not if a gateway
    /// or a server is processing and pushing the user events).
    ///
    /// This should not be set when using the JavaScript tag in
    /// [UserEventService.CollectUserEvent][google.cloud.retail.v2.UserEventService.CollectUserEvent].
    #[prost(bool, tag = "4")]
    pub direct_user_request: bool,
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
    /// Immutable. [Product][google.cloud.retail.v2.Product] identifier, which is
    /// the final component of [name][google.cloud.retail.v2.Product.name]. For
    /// example, this field is "id_1", if
    /// [name][google.cloud.retail.v2.Product.name] is
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
    /// Immutable. The type of the product. This field is output-only.
    #[prost(enumeration = "product::Type", tag = "3")]
    pub r#type: i32,
    /// Variant group identifier. Must be an
    /// [id][google.cloud.retail.v2.Product.id], with the same parent branch with
    /// this product. Otherwise, an error is thrown.
    ///
    /// For [Type.PRIMARY][google.cloud.retail.v2.Product.Type.PRIMARY]
    /// [Product][google.cloud.retail.v2.Product]s, this field can only be empty or
    /// set to the same value as [id][google.cloud.retail.v2.Product.id].
    ///
    /// For VARIANT [Product][google.cloud.retail.v2.Product]s, this field cannot
    /// be empty. A maximum of 2,000 products are allowed to share the same
    /// [Type.PRIMARY][google.cloud.retail.v2.Product.Type.PRIMARY]
    /// [Product][google.cloud.retail.v2.Product]. Otherwise, an INVALID_ARGUMENT
    /// error is returned.
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
    /// Must be set for [Type.PRIMARY][google.cloud.retail.v2.Product.Type.PRIMARY]
    /// [Product][google.cloud.retail.v2.Product] otherwise an INVALID_ARGUMENT
    /// error is returned.
    ///
    /// At most 250 values are allowed per
    /// [Product][google.cloud.retail.v2.Product]. Empty values are not allowed.
    /// Each value must be a UTF-8 encoded string with a length limit of 5,000
    /// characters. Otherwise, an INVALID_ARGUMENT error is returned.
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
    /// This field must be a UTF-8 encoded string with a length limit of 128
    /// characters. Otherwise, an INVALID_ARGUMENT error is returned.
    ///
    /// Google Merchant Center property
    /// [title](https://support.google.com/merchants/answer/6324415). Schema.org
    /// property [Product.name](https://schema.org/name).
    #[prost(string, tag = "8")]
    pub title: ::prost::alloc::string::String,
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
    /// A maximum of 150 attributes are allowed. Otherwise, an INVALID_ARGUMENT
    /// error is returned.
    ///
    /// The key must be a UTF-8 encoded string with a length limit of 5,000
    /// characters. Otherwise, an INVALID_ARGUMENT error is returned.
    #[prost(map = "string, message", tag = "12")]
    pub attributes: ::std::collections::HashMap<::prost::alloc::string::String, CustomAttribute>,
    /// Custom tags associated with the product.
    ///
    /// At most 250 values are allowed per
    /// [Product][google.cloud.retail.v2.Product]. This value must be a UTF-8
    /// encoded string with a length limit of 1,000 characters. Otherwise, an
    /// INVALID_ARGUMENT error is returned.
    ///
    /// This tag can be used for filtering recommendation results by passing the
    /// tag as part of the
    /// [PredictRequest.filter][google.cloud.retail.v2.PredictRequest.filter].
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
    /// The timestamp when this [Product][google.cloud.retail.v2.Product] becomes
    /// available recommendation and search.
    #[prost(message, optional, tag = "18")]
    pub available_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The online availability of the [Product][google.cloud.retail.v2.Product].
    /// Default to
    /// [Availability.IN_STOCK][google.cloud.retail.v2.Product.Availability.IN_STOCK].
    ///
    /// Google Merchant Center Property
    /// [availability](https://support.google.com/merchants/answer/6324448).
    /// Schema.org Property [Offer.availability](https://schema.org/availability).
    #[prost(enumeration = "product::Availability", tag = "19")]
    pub availability: i32,
    /// The available quantity of the item.
    #[prost(message, optional, tag = "20")]
    pub available_quantity: ::core::option::Option<i32>,
    /// Canonical URL directly linking to the product detail page.
    ///
    /// This field must be a UTF-8 encoded string with a length limit of 5,000
    /// characters. Otherwise, an INVALID_ARGUMENT error is returned.
    ///
    /// Google Merchant Center property
    /// [link](https://support.google.com/merchants/answer/6324416). Schema.org
    /// property [Offer.url](https://schema.org/url).
    #[prost(string, tag = "22")]
    pub uri: ::prost::alloc::string::String,
    /// Product images for the product.
    ///
    /// A maximum of 300 images are allowed.
    ///
    /// Google Merchant Center property
    /// [image_link](https://support.google.com/merchants/answer/6324350).
    /// Schema.org property [Product.image](https://schema.org/image).
    #[prost(message, repeated, tag = "23")]
    pub images: ::prost::alloc::vec::Vec<Image>,
}
/// Nested message and enum types in `Product`.
pub mod product {
    /// The type of this product.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Type {
        /// Default value. Default to
        /// [Type.PRIMARY][google.cloud.retail.v2.Product.Type.PRIMARY] if unset.
        Unspecified = 0,
        /// The primary type.
        ///
        /// As the primary unit for predicting, indexing and search serving, a
        /// [Type.PRIMARY][google.cloud.retail.v2.Product.Type.PRIMARY]
        /// [Product][google.cloud.retail.v2.Product] is grouped with multiple
        /// [Type.VARIANT][google.cloud.retail.v2.Product.Type.VARIANT]
        /// [Product][google.cloud.retail.v2.Product]s.
        Primary = 1,
        /// The variant type.
        ///
        /// [Type.VARIANT][google.cloud.retail.v2.Product.Type.VARIANT]
        /// [Product][google.cloud.retail.v2.Product]s usually share some common
        /// attributes on the same
        /// [Type.PRIMARY][google.cloud.retail.v2.Product.Type.PRIMARY]
        /// [Product][google.cloud.retail.v2.Product]s, but they have variant
        /// attributes like different colors, sizes and prices, etc.
        Variant = 2,
        /// The collection type. Collection products are bundled
        /// [Type.PRIMARY][google.cloud.retail.v2.Product.Type.PRIMARY]
        /// [Product][google.cloud.retail.v2.Product]s or
        /// [Type.VARIANT][google.cloud.retail.v2.Product.Type.VARIANT]
        /// [Product][google.cloud.retail.v2.Product]s that are sold together, such
        /// as a jewelry set with necklaces, earrings and rings, etc.
        Collection = 3,
    }
    /// Product availability. If this field is unspecified, the product is
    /// assumed to be in stock.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Availability {
        /// Default product availability. Default to
        /// [Availability.IN_STOCK][google.cloud.retail.v2.Product.Availability.IN_STOCK]
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
    /// * `detail-page-view`: Products detail page viewed.
    /// * `home-page-view`: Homepage viewed.
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
    #[prost(string, tag = "2")]
    pub visitor_id: ::prost::alloc::string::String,
    /// Only required for
    /// [UserEventService.ImportUserEvents][google.cloud.retail.v2.UserEventService.ImportUserEvents]
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
    /// [PredictionService.Predict][google.cloud.retail.v2.PredictionService.Predict].
    /// This field enables accurate attribution of recommendation model
    /// performance.
    ///
    /// The value must be a valid
    /// [PredictResponse.attribution_token][google.cloud.retail.v2.PredictResponse.attribution_token]
    /// for user events that are the result of
    /// [PredictionService.Predict][google.cloud.retail.v2.PredictionService.Predict].
    ///
    /// This token enables us to accurately attribute page view or purchase back to
    /// the event and the particular predict response containing this
    /// clicked/purchased product. If user clicks on product K in the
    /// recommendation results, pass
    /// [PredictResponse.attribution_token][google.cloud.retail.v2.PredictResponse.attribution_token]
    /// as a URL parameter to product K's page. When recording events on product
    /// K's page, log the
    /// [PredictResponse.attribution_token][google.cloud.retail.v2.PredictResponse.attribution_token]
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
    /// [product_details][google.cloud.retail.v2.UserEvent.product_details] is
    /// desired. The end user may have not finished broswing the whole page yet.
    #[prost(message, repeated, tag = "6")]
    pub product_details: ::prost::alloc::vec::Vec<ProductDetail>,
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
    /// The value must be a UTF-8 encoded string with a length limit of 5,000
    /// characters. Otherwise, an INVALID_ARGUMENT error is returned.
    ///
    /// Required for `search` events. Other event types should not set this field.
    /// Otherwise, an INVALID_ARGUMENT error is returned.
    #[prost(string, tag = "10")]
    pub search_query: ::prost::alloc::string::String,
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
    /// Required for `category-page-view` events. Other event types should not set
    /// this field. Otherwise, an INVALID_ARGUMENT error is returned.
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
    /// Required. [Product][google.cloud.retail.v2.Product] information.
    ///
    /// Only [Product.id][google.cloud.retail.v2.Product.id] field is used when
    /// ingesting an event, all other product fields are ignored as we will look
    /// them up from the catalog.
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
    /// * Profit = [revenue][google.cloud.retail.v2.PurchaseTransaction.revenue] -
    /// [tax][google.cloud.retail.v2.PurchaseTransaction.tax] -
    /// [cost][google.cloud.retail.v2.PurchaseTransaction.cost]
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
    /// information](https://cloud.google.com/recommendations-ai/docs/upload-catalog)
    /// for the expected file format and setup instructions.
    #[prost(string, repeated, tag = "1")]
    pub input_uris: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The schema to use when parsing the data from the source.
    ///
    /// Supported values for product imports:
    ///
    /// * `product` (default): One JSON [Product][google.cloud.retail.v2.Product]
    /// per line. Each product must
    ///   have a valid [Product.id][google.cloud.retail.v2.Product.id].
    /// * `product_merchant_center`: See [Importing catalog data from Merchant
    ///   Center](https://cloud.google.com/retail/recommendations-ai/docs/upload-catalog#mc).
    ///
    /// Supported values for user events imports:
    ///
    /// * `user_event` (default): One JSON
    /// [UserEvent][google.cloud.retail.v2.UserEvent] per line.
    /// * `user_event_ga360`: Using
    ///   https://support.google.com/analytics/answer/3437719?hl=en.
    #[prost(string, tag = "2")]
    pub data_schema: ::prost::alloc::string::String,
}
/// BigQuery source import data from.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BigQuerySource {
    /// The project id (can be project # or id) that the BigQuery source is in with
    /// a length limit of 128 characters. If not specified, inherits the project
    /// id from the parent request.
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
    /// * `product` (default): One JSON [Product][google.cloud.retail.v2.Product]
    /// per line. Each product must
    ///   have a valid [Product.id][google.cloud.retail.v2.Product.id].
    /// * `product_merchant_center`: See [Importing catalog data from Merchant
    ///   Center](https://cloud.google.com/retail/recommendations-ai/docs/upload-catalog#mc).
    ///
    /// Supported values for user events imports:
    ///
    /// * `user_event` (default): One JSON
    /// [UserEvent][google.cloud.retail.v2.UserEvent] per line.
    /// * `user_event_ga360`: Using
    ///   https://support.google.com/analytics/answer/3437719?hl=en.
    #[prost(string, tag = "4")]
    pub data_schema: ::prost::alloc::string::String,
}
/// The inline source for the input config for ImportProducts method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProductInlineSource {
    /// Required. A list of products to update/create. Each product must have a
    /// valid [Product.id][google.cloud.retail.v2.Product.id]. Recommended max of
    /// 10k items.
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
}
/// Response of the
/// [ImportProductsRequest][google.cloud.retail.v2.ImportProductsRequest]. If the
/// long running operation is done, then this message is returned by the
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
/// Request message for Predict method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PredictRequest {
    /// Required. Full resource name of the format:
    /// {name=projects/*/locations/global/catalogs/default_catalog/placements/*}
    /// The id of the recommendation engine placement. This id is used to identify
    /// the set of models that will be used to make the prediction.
    ///
    /// We currently support three placements with the following IDs by default:
    ///
    /// * `shopping_cart`: Predicts products frequently bought together with one or
    ///   more  products in the same shopping session. Commonly displayed after
    ///   `add-to-cart` events, on product detail pages, or on the shopping cart
    ///   page.
    ///
    /// * `home_page`: Predicts the next product that a user will most likely
    ///   engage with or purchase based on the shopping or viewing history of the
    ///   specified `userId` or `visitorId`. For example - Recommendations for you.
    ///
    /// * `product_detail`: Predicts the next product that a user will most likely
    ///   engage with or purchase. The prediction is based on the shopping or
    ///   viewing history of the specified `userId` or `visitorId` and its
    ///   relevance to a specified `CatalogItem`. Typically used on product detail
    ///   pages. For example - More products like this.
    ///
    /// * `recently_viewed_default`: Returns up to 75 products recently viewed by
    ///   the specified `userId` or `visitorId`, most recent ones first. Returns
    ///   nothing if neither of them has viewed any products yet. For example -
    ///   Recently viewed.
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
    #[prost(map = "string, message", tag = "7")]
    pub params: ::std::collections::HashMap<::prost::alloc::string::String, ::prost_types::Value>,
    /// The labels for the predict request.
    ///
    ///  * Label keys can contain lowercase letters, digits and hyphens, must start
    ///    with a letter, and must end with a letter or digit.
    ///  * Non-zero label values can contain lowercase letters, digits and hyphens,
    ///    must start with a letter, and must end with a letter or digit.
    ///  * No more than 64 labels can be associated with a given request.
    ///
    /// See https://goo.gl/xmQnxf for more information on and examples of labels.
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
    /// [UserEvent][google.cloud.retail.v2.UserEvent] logs resulting from this
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
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Service for making recommendation prediction."]
    pub struct PredictionServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> PredictionServiceClient<T>
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
                "/google.cloud.retail.v2.PredictionService/Predict",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for PredictionServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for PredictionServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "PredictionServiceClient {{ ... }}")
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
    /// "projects/${projectId}/locations/global/catalogs/${catalogId}"
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The filter string to specify the events to be deleted with a
    /// length limit of 5,000 characters. Empty string filter is not allowed. The
    /// eligible fields for filtering are:
    ///
    /// * `eventType`: Double quoted
    /// [UserEvent.event_type][google.cloud.retail.v2.UserEvent.event_type] string.
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
    /// Required. The [Product][google.cloud.retail.v2.Product] to create.
    #[prost(message, optional, tag = "2")]
    pub product: ::core::option::Option<Product>,
    /// Required. The ID to use for the [Product][google.cloud.retail.v2.Product],
    /// which will become the final component of the
    /// [Product.name][google.cloud.retail.v2.Product.name].
    ///
    /// If the caller does not have permission to create the
    /// [Product][google.cloud.retail.v2.Product], regardless of whether or not it
    /// exists, a PERMISSION_DENIED error is returned.
    ///
    /// This field must be unique among all
    /// [Product][google.cloud.retail.v2.Product]s with the same
    /// [parent][google.cloud.retail.v2.CreateProductRequest.parent]. Otherwise, an
    /// ALREADY_EXISTS error is returned.
    ///
    /// This field must be a UTF-8 encoded string with a length limit of 128
    /// characters. Otherwise, an INVALID_ARGUMENT error is returned.
    #[prost(string, tag = "3")]
    pub product_id: ::prost::alloc::string::String,
}
/// Request message for [GetProduct][] method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetProductRequest {
    /// Required. Full resource name of [Product][google.cloud.retail.v2.Product],
    /// such as
    /// `projects/*/locations/global/catalogs/default_catalog/branches/default_branch/products/some_product_id`.
    ///
    /// If the caller does not have permission to access the
    /// [Product][google.cloud.retail.v2.Product], regardless of whether or not it
    /// exists, a PERMISSION_DENIED error is returned.
    ///
    /// If the requested [Product][google.cloud.retail.v2.Product] does not exist,
    /// a NOT_FOUND error is returned.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for [UpdateProduct][] method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateProductRequest {
    /// Required. The product to update/create.
    ///
    /// If the caller does not have permission to update the
    /// [Product][google.cloud.retail.v2.Product], regardless of whether or not it
    /// exists, a PERMISSION_DENIED error is returned.
    ///
    /// If the [Product][google.cloud.retail.v2.Product] to update does not exist,
    /// a NOT_FOUND error is returned.
    #[prost(message, optional, tag = "1")]
    pub product: ::core::option::Option<Product>,
    /// Indicates which fields in the provided
    /// [Product][google.cloud.retail.v2.Product] to update. The immutable and
    /// output only fields are NOT supported. If not set, all supported fields (the
    /// fields that are neither immutable nor output only) are updated.
    ///
    /// If an unsupported or unknown field is provided, an INVALID_ARGUMENT error
    /// is returned.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for [DeleteProduct][] method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteProductRequest {
    /// Required. Full resource name of [Product][google.cloud.retail.v2.Product],
    /// such as
    /// `projects/*/locations/global/catalogs/default_catalog/branches/default_branch/products/some_product_id`.
    ///
    /// If the caller does not have permission to delete the
    /// [Product][google.cloud.retail.v2.Product], regardless of whether or not it
    /// exists, a PERMISSION_DENIED error is returned.
    ///
    /// If the [Product][google.cloud.retail.v2.Product] to delete does not exist,
    /// a NOT_FOUND error is returned.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
#[doc = r" Generated client implementations."]
pub mod product_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Service for ingesting [Product][google.cloud.retail.v2.Product] information"]
    #[doc = " of the customer's website."]
    pub struct ProductServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> ProductServiceClient<T>
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
        #[doc = " Creates a [Product][google.cloud.retail.v2.Product]."]
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
                "/google.cloud.retail.v2.ProductService/CreateProduct",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets a [Product][google.cloud.retail.v2.Product]."]
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
                "/google.cloud.retail.v2.ProductService/GetProduct",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates a [Product][google.cloud.retail.v2.Product]."]
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
                "/google.cloud.retail.v2.ProductService/UpdateProduct",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes a [Product][google.cloud.retail.v2.Product]."]
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
                "/google.cloud.retail.v2.ProductService/DeleteProduct",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Bulk import of multiple [Product][google.cloud.retail.v2.Product]s."]
        #[doc = ""]
        #[doc = " Request processing may be synchronous. No partial updating is supported."]
        #[doc = " Non-existing items are created."]
        #[doc = ""]
        #[doc = " Note that it is possible for a subset of the"]
        #[doc = " [Product][google.cloud.retail.v2.Product]s to be successfully updated."]
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
                "/google.cloud.retail.v2.ProductService/ImportProducts",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for ProductServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for ProductServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "ProductServiceClient {{ ... }}")
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
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Service for ingesting end user actions on the customer website."]
    pub struct UserEventServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> UserEventServiceClient<T>
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
                "/google.cloud.retail.v2.UserEventService/WriteUserEvent",
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
                "/google.cloud.retail.v2.UserEventService/CollectUserEvent",
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
                "/google.cloud.retail.v2.UserEventService/PurgeUserEvents",
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
                "/google.cloud.retail.v2.UserEventService/ImportUserEvents",
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
                "/google.cloud.retail.v2.UserEventService/RejoinUserEvents",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for UserEventServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for UserEventServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "UserEventServiceClient {{ ... }}")
        }
    }
}
