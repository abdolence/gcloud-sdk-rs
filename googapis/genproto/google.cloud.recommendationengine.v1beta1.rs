/// FeatureMap represents extra features that customers want to include in the
/// recommendation model for catalogs/user events as categorical/numerical
/// features.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeatureMap {
    /// Categorical features that can take on one of a limited number of possible
    /// values. Some examples would be the brand/maker of a product, or country of
    /// a customer.
    ///
    /// Feature names and values must be UTF-8 encoded strings.
    ///
    /// For example: `{ "colors": {"value": ["yellow", "green"]},
    ///                 "sizes": {"value":["S", "M"]}`
    #[prost(map = "string, message", tag = "1")]
    pub categorical_features:
        ::std::collections::HashMap<std::string::String, feature_map::StringList>,
    /// Numerical features. Some examples would be the height/weight of a product,
    /// or age of a customer.
    ///
    /// Feature names must be UTF-8 encoded strings.
    ///
    /// For example: `{ "lengths_cm": {"value":[2.3, 15.4]},
    ///                 "heights_cm": {"value":[8.1, 6.4]} }`
    #[prost(map = "string, message", tag = "2")]
    pub numerical_features:
        ::std::collections::HashMap<std::string::String, feature_map::FloatList>,
}
pub mod feature_map {
    /// A list of string features.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct StringList {
        /// String feature value with a length limit of 128 bytes.
        #[prost(string, repeated, tag = "1")]
        pub value: ::std::vec::Vec<std::string::String>,
    }
    /// A list of float features.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct FloatList {
        /// Float feature value.
        #[prost(float, repeated, tag = "1")]
        pub value: ::std::vec::Vec<f32>,
    }
}
/// CatalogItem captures all metadata information of items to be recommended.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CatalogItem {
    /// Required. Catalog item identifier. UTF-8 encoded string with a length limit
    /// of 128 bytes.
    ///
    /// This id must be unique among all catalog items within the same catalog. It
    /// should also be used when logging user events in order for the user events
    /// to be joined with the Catalog.
    #[prost(string, tag = "1")]
    pub id: std::string::String,
    /// Required. Catalog item categories. This field is repeated for supporting
    /// one catalog item belonging to several parallel category hierarchies.
    ///
    /// For example, if a shoes product belongs to both
    /// ["Shoes & Accessories" -> "Shoes"] and
    /// ["Sports & Fitness" -> "Athletic Clothing" -> "Shoes"], it could be
    /// represented as:
    ///
    ///      "categoryHierarchies": [
    ///        { "categories": ["Shoes & Accessories", "Shoes"]},
    ///        { "categories": ["Sports & Fitness", "Athletic Clothing", "Shoes"] }
    ///      ]
    #[prost(message, repeated, tag = "2")]
    pub category_hierarchies: ::std::vec::Vec<catalog_item::CategoryHierarchy>,
    /// Required. Catalog item title. UTF-8 encoded string with a length limit of 1
    /// KiB.
    #[prost(string, tag = "3")]
    pub title: std::string::String,
    /// Optional. Catalog item description. UTF-8 encoded string with a length
    /// limit of 5 KiB.
    #[prost(string, tag = "4")]
    pub description: std::string::String,
    /// Optional. Highly encouraged. Extra catalog item attributes to be
    /// included in the recommendation model. For example, for retail products,
    /// this could include the store name, vendor, style, color, etc. These are
    /// very strong signals for recommendation model, thus we highly recommend
    /// providing the item attributes here.
    #[prost(message, optional, tag = "5")]
    pub item_attributes: ::std::option::Option<FeatureMap>,
    /// Optional. Language of the title/description/item_attributes. Use language
    /// tags defined by BCP 47. https://www.rfc-editor.org/rfc/bcp/bcp47.txt. Our
    /// supported language codes include 'en', 'es', 'fr', 'de', 'ar', 'fa', 'zh',
    /// 'ja', 'ko', 'sv', 'ro', 'nl'. For other languages, contact
    /// your Google account manager.
    #[prost(string, tag = "6")]
    pub language_code: std::string::String,
    /// Optional. Filtering tags associated with the catalog item. Each tag should
    /// be a UTF-8 encoded string with a length limit of 1 KiB.
    ///
    /// This tag can be used for filtering recommendation results by passing the
    /// tag as part of the predict request filter.
    #[prost(string, repeated, tag = "8")]
    pub tags: ::std::vec::Vec<std::string::String>,
    /// Optional. Variant group identifier for prediction results. UTF-8 encoded
    /// string with a length limit of 128 bytes.
    ///
    /// This field must be enabled before it can be used. [Learn
    /// more](/recommendations-ai/docs/catalog#item-group-id).
    #[prost(string, tag = "9")]
    pub item_group_id: std::string::String,
    /// Extra catalog item metadata for different recommendation types.
    #[prost(oneof = "catalog_item::RecommendationType", tags = "10")]
    pub recommendation_type: ::std::option::Option<catalog_item::RecommendationType>,
}
pub mod catalog_item {
    /// Category represents catalog item category hierarchy.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CategoryHierarchy {
        /// Required. Catalog item categories. Each category should be a UTF-8
        /// encoded string with a length limit of 2 KiB.
        ///
        /// Note that the order in the list denotes the specificity (from least to
        /// most specific).
        #[prost(string, repeated, tag = "1")]
        pub categories: ::std::vec::Vec<std::string::String>,
    }
    /// Extra catalog item metadata for different recommendation types.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum RecommendationType {
        /// Optional. Metadata specific to retail products.
        #[prost(message, tag = "10")]
        ProductMetadata(super::ProductCatalogItem),
    }
}
/// ProductCatalogItem captures item metadata specific to retail products.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProductCatalogItem {
    /// Optional. A map to pass the costs associated with the product.
    ///
    /// For example:
    /// {"manufacturing": 45.5} The profit of selling this item is computed like
    /// so:
    ///
    /// * If 'exactPrice' is provided, profit = displayPrice - sum(costs)
    /// * If 'priceRange' is provided, profit = minPrice - sum(costs)
    #[prost(map = "string, float", tag = "3")]
    pub costs: ::std::collections::HashMap<std::string::String, f32>,
    /// Optional. Only required if the price is set. Currency code for price/costs. Use
    /// three-character ISO-4217 code.
    #[prost(string, tag = "4")]
    pub currency_code: std::string::String,
    /// Optional. Online stock state of the catalog item. Default is `IN_STOCK`.
    #[prost(enumeration = "product_catalog_item::StockState", tag = "5")]
    pub stock_state: i32,
    /// Optional. The available quantity of the item.
    #[prost(int64, tag = "6")]
    pub available_quantity: i64,
    /// Optional. Canonical URL directly linking to the item detail page with a
    /// length limit of 5 KiB..
    #[prost(string, tag = "7")]
    pub canonical_product_uri: std::string::String,
    /// Optional. Product images for the catalog item.
    #[prost(message, repeated, tag = "8")]
    pub images: ::std::vec::Vec<Image>,
    /// Product price. Only one of 'exactPrice'/'priceRange' can be provided.
    #[prost(oneof = "product_catalog_item::Price", tags = "1, 2")]
    pub price: ::std::option::Option<product_catalog_item::Price>,
}
pub mod product_catalog_item {
    /// Exact product price.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ExactPrice {
        /// Optional. Display price of the product.
        #[prost(float, tag = "1")]
        pub display_price: f32,
        /// Optional. Price of the product without any discount. If zero, by default
        /// set to be the 'displayPrice'.
        #[prost(float, tag = "2")]
        pub original_price: f32,
    }
    /// Product price range when there are a range of prices for different
    /// variations of the same product.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PriceRange {
        /// Required. The minimum product price.
        #[prost(float, tag = "1")]
        pub min: f32,
        /// Required. The maximum product price.
        #[prost(float, tag = "2")]
        pub max: f32,
    }
    /// Item stock state. If this field is unspecified, the item is
    /// assumed to be in stock.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum StockState {
        /// Default item stock status. Should never be used.
        Unspecified = 0,
        /// Item out of stock.
        OutOfStock = 1,
        /// Item that is in pre-order state.
        Preorder = 2,
        /// Item that is back-ordered (i.e. temporarily out of stock).
        Backorder = 3,
    }
    /// Product price. Only one of 'exactPrice'/'priceRange' can be provided.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Price {
        /// Optional. The exact product price.
        #[prost(message, tag = "1")]
        ExactPrice(ExactPrice),
        /// Optional. The product price range.
        #[prost(message, tag = "2")]
        PriceRange(PriceRange),
    }
}
/// Catalog item thumbnail/detail image.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Image {
    /// Required. URL of the image with a length limit of 5 KiB.
    #[prost(string, tag = "1")]
    pub uri: std::string::String,
    /// Optional. Height of the image in number of pixels.
    #[prost(int32, tag = "2")]
    pub height: i32,
    /// Optional. Width of the image in number of pixels.
    #[prost(int32, tag = "3")]
    pub width: i32,
}
/// UserEvent captures all metadata information recommendation engine needs to
/// know about how end users interact with customers' website.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserEvent {
    /// Required. User event type. Allowed values are:
    ///
    /// * `add-to-cart` Products being added to cart.
    /// * `add-to-list` Items being added to a list (shopping list, favorites
    ///   etc).
    /// * `category-page-view` Special pages such as sale or promotion pages
    ///   viewed.
    /// * `checkout-start` User starting a checkout process.
    /// * `detail-page-view` Products detail page viewed.
    /// * `home-page-view` Homepage viewed.
    /// * `page-visit` Generic page visits not included in the event types above.
    /// * `purchase-complete` User finishing a purchase.
    /// * `refund` Purchased items being refunded or returned.
    /// * `remove-from-cart` Products being removed from cart.
    /// * `remove-from-list` Items being removed from a list.
    /// * `search` Product search.
    /// * `shopping-cart-page-view` User viewing a shopping cart.
    /// * `impression` List of items displayed. Used by Google Tag Manager.
    #[prost(string, tag = "1")]
    pub event_type: std::string::String,
    /// Required. User information.
    #[prost(message, optional, tag = "2")]
    pub user_info: ::std::option::Option<UserInfo>,
    /// Optional. User event detailed information common across different
    /// recommendation types.
    #[prost(message, optional, tag = "3")]
    pub event_detail: ::std::option::Option<EventDetail>,
    /// Optional. Retail product specific user event metadata.
    ///
    /// This field is required for the following event types:
    ///
    /// * `add-to-cart`
    /// * `add-to-list`
    /// * `category-page-view`
    /// * `checkout-start`
    /// * `detail-page-view`
    /// * `purchase-complete`
    /// * `refund`
    /// * `remove-from-cart`
    /// * `remove-from-list`
    /// * `search`
    ///
    /// This field is optional for the following event types:
    ///
    /// * `page-visit`
    /// * `shopping-cart-page-view` - note that 'product_event_detail' should be
    ///   set for this unless the shopping cart is empty.
    ///
    /// This field is not allowed for the following event types:
    ///
    /// * `home-page-view`
    #[prost(message, optional, tag = "4")]
    pub product_event_detail: ::std::option::Option<ProductEventDetail>,
    /// Optional. Only required for ImportUserEvents method. Timestamp of user
    /// event created.
    #[prost(message, optional, tag = "5")]
    pub event_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Optional. This field should *not* be set when using JavaScript pixel
    /// or the Recommendations AI Tag. Defaults to `EVENT_SOURCE_UNSPECIFIED`.
    #[prost(enumeration = "user_event::EventSource", tag = "6")]
    pub event_source: i32,
}
pub mod user_event {
    /// User event source.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum EventSource {
        /// Unspecified event source.
        Unspecified = 0,
        /// The event is ingested via a javascript pixel or Recommendations AI Tag
        /// through automl datalayer or JS Macros.
        Automl = 1,
        /// The event is ingested via Recommendations AI Tag through Enhanced
        /// Ecommerce datalayer.
        Ecommerce = 2,
        /// The event is ingested via Import user events API.
        BatchUpload = 3,
    }
}
/// Information of end users.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserInfo {
    /// Required. A unique identifier for tracking visitors with a length limit of
    /// 128 bytes.
    ///
    /// For example, this could be implemented with a http cookie, which should be
    /// able to uniquely identify a visitor on a single device. This unique
    /// identifier should not change if the visitor log in/out of the website.
    /// Maximum length 128 bytes. Cannot be empty.
    #[prost(string, tag = "1")]
    pub visitor_id: std::string::String,
    /// Optional. Unique identifier for logged-in user with a length limit of 128
    /// bytes. Required only for logged-in users.
    #[prost(string, tag = "2")]
    pub user_id: std::string::String,
    /// Optional. IP address of the user. This could be either IPv4 (e.g. 104.133.9.80) or
    /// IPv6 (e.g. 2001:0db8:85a3:0000:0000:8a2e:0370:7334). This should *not* be
    /// set when using the javascript pixel or if `direct_user_request` is set.
    /// Used to extract location information for personalization.
    #[prost(string, tag = "3")]
    pub ip_address: std::string::String,
    /// Optional. User agent as included in the HTTP header. UTF-8 encoded string
    /// with a length limit of 1 KiB.
    ///
    /// This should *not* be set when using the JavaScript pixel or if
    /// `directUserRequest` is set.
    #[prost(string, tag = "4")]
    pub user_agent: std::string::String,
    /// Optional. Indicates if the request is made directly from the end user
    /// in which case the user_agent and ip_address fields can be populated
    /// from the HTTP request. This should *not* be set when using the javascript
    /// pixel. This flag should be set only if the API request is made directly
    /// from the end user such as a mobile app (and not if a gateway or a server is
    /// processing and pushing the user events).
    #[prost(bool, tag = "5")]
    pub direct_user_request: bool,
}
/// User event details shared by all recommendation types.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventDetail {
    /// Optional. Complete url (window.location.href) of the user's current page.
    /// When using the JavaScript pixel, this value is filled in automatically.
    /// Maximum length 5KB.
    #[prost(string, tag = "1")]
    pub uri: std::string::String,
    /// Optional. The referrer url of the current page. When using
    /// the JavaScript pixel, this value is filled in automatically.
    #[prost(string, tag = "6")]
    pub referrer_uri: std::string::String,
    /// Optional. A unique id of a web page view.
    /// This should be kept the same for all user events triggered from the same
    /// pageview. For example, an item detail page view could trigger multiple
    /// events as the user is browsing the page.
    /// The `pageViewId` property should be kept the same for all these events so
    /// that they can be grouped together properly. This `pageViewId` will be
    /// automatically generated if using the JavaScript pixel.
    #[prost(string, tag = "2")]
    pub page_view_id: std::string::String,
    /// Optional. A list of identifiers for the independent experiment groups
    /// this user event belongs to. This is used to distinguish between user events
    /// associated with different experiment setups (e.g. using Recommendation
    /// Engine system, using different recommendation models).
    #[prost(string, repeated, tag = "3")]
    pub experiment_ids: ::std::vec::Vec<std::string::String>,
    /// Optional. Recommendation token included in the recommendation prediction
    /// response.
    ///
    /// This field enables accurate attribution of recommendation model
    /// performance.
    ///
    /// This token enables us to accurately attribute page view or purchase back to
    /// the event and the particular predict response containing this
    /// clicked/purchased item. If user clicks on product K in the recommendation
    /// results, pass the `PredictResponse.recommendationToken` property as a url
    /// parameter to product K's page. When recording events on product K's page,
    /// log the PredictResponse.recommendation_token to this field.
    ///
    /// Optional, but highly encouraged for user events that are the result of a
    /// recommendation prediction query.
    #[prost(string, tag = "4")]
    pub recommendation_token: std::string::String,
    /// Optional. Extra user event features to include in the recommendation
    /// model.
    ///
    /// For product recommendation, an example of extra user information is
    /// traffic_channel, i.e. how user arrives at the site. Users can arrive
    /// at the site by coming to the site directly, or coming through Google
    /// search, and etc.
    #[prost(message, optional, tag = "5")]
    pub event_attributes: ::std::option::Option<FeatureMap>,
}
/// ProductEventDetail captures user event information specific to retail
/// products.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProductEventDetail {
    /// Required for `search` events. Other event types should not set this field.
    /// The user's search query as UTF-8 encoded text with a length limit of 5 KiB.
    #[prost(string, tag = "1")]
    pub search_query: std::string::String,
    /// Required for `category-page-view` events. Other event types should not set
    /// this field.
    /// The categories associated with a category page.
    /// Category pages include special pages such as sales or promotions. For
    /// instance, a special sale page may have the category hierarchy:
    /// categories : ["Sales", "2017 Black Friday Deals"].
    #[prost(message, repeated, tag = "2")]
    pub page_categories: ::std::vec::Vec<catalog_item::CategoryHierarchy>,
    /// The main product details related to the event.
    ///
    /// This field is required for the following event types:
    ///
    /// * `add-to-cart`
    /// * `add-to-list`
    /// * `checkout-start`
    /// * `detail-page-view`
    /// * `purchase-complete`
    /// * `refund`
    /// * `remove-from-cart`
    /// * `remove-from-list`
    ///
    /// This field is optional for the following event types:
    ///
    /// * `page-visit`
    /// * `shopping-cart-page-view` - note that 'product_details' should be set for
    ///   this unless the shopping cart is empty.
    ///
    /// This field is not allowed for the following event types:
    ///
    /// * `category-page-view`
    /// * `home-page-view`
    /// * `search`
    #[prost(message, repeated, tag = "3")]
    pub product_details: ::std::vec::Vec<ProductDetail>,
    /// Required for `add-to-list` and `remove-from-list` events. The id or name of
    /// the list that the item is being added to or removed from. Other event types
    /// should not set this field.
    #[prost(string, tag = "4")]
    pub list_id: std::string::String,
    /// Optional. The id or name of the associated shopping cart. This id is used
    /// to associate multiple items added or present in the cart before purchase.
    ///
    /// This can only be set for `add-to-cart`, `remove-from-cart`,
    /// `checkout-start`, `purchase-complete`, or `shopping-cart-page-view` events.
    #[prost(string, tag = "5")]
    pub cart_id: std::string::String,
    /// Optional. A transaction represents the entire purchase transaction.
    /// Required for `purchase-complete` events. Optional for `checkout-start`
    /// events. Other event types should not set this field.
    #[prost(message, optional, tag = "6")]
    pub purchase_transaction: ::std::option::Option<PurchaseTransaction>,
}
/// A transaction represents the entire purchase transaction.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PurchaseTransaction {
    /// Optional. The transaction ID with a length limit of 128 bytes.
    #[prost(string, tag = "1")]
    pub id: std::string::String,
    /// Required. Total revenue or grand total associated with the transaction.
    /// This value include shipping, tax, or other adjustments to total revenue
    /// that you want to include as part of your revenue calculations. This field
    /// is not required if the event type is `refund`.
    #[prost(float, tag = "2")]
    pub revenue: f32,
    /// Optional. All the taxes associated with the transaction.
    #[prost(map = "string, float", tag = "3")]
    pub taxes: ::std::collections::HashMap<std::string::String, f32>,
    /// Optional. All the costs associated with the product. These can be
    /// manufacturing costs, shipping expenses not borne by the end user, or any
    /// other costs.
    ///
    /// Total product cost such that
    ///   profit = revenue - (sum(taxes) + sum(costs))
    /// If product_cost is not set, then
    ///   profit = revenue - tax - shipping - sum(CatalogItem.costs).
    ///
    /// If CatalogItem.cost is not specified for one of the items, CatalogItem.cost
    /// based profit *cannot* be calculated for this Transaction.
    #[prost(map = "string, float", tag = "4")]
    pub costs: ::std::collections::HashMap<std::string::String, f32>,
    /// Required. Currency code. Use three-character ISO-4217 code. This field
    /// is not required if the event type is `refund`.
    #[prost(string, tag = "6")]
    pub currency_code: std::string::String,
}
/// Detailed product information associated with a user event.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProductDetail {
    /// Required. Catalog item ID. UTF-8 encoded string with a length limit of 128
    /// characters.
    #[prost(string, tag = "1")]
    pub id: std::string::String,
    /// Optional. Currency code for price/costs. Use three-character ISO-4217
    /// code. Required only if originalPrice or displayPrice is set.
    #[prost(string, tag = "2")]
    pub currency_code: std::string::String,
    /// Optional. Original price of the product. If provided, this will override
    /// the original price in Catalog for this product.
    #[prost(float, tag = "3")]
    pub original_price: f32,
    /// Optional. Display price of the product (e.g. discounted price). If
    /// provided, this will override the display price in Catalog for this product.
    #[prost(float, tag = "4")]
    pub display_price: f32,
    /// Optional. Item stock state. If provided, this overrides the stock state
    /// in Catalog for items in this event.
    #[prost(enumeration = "product_catalog_item::StockState", tag = "5")]
    pub stock_state: i32,
    /// Optional. Quantity of the product associated with the user event. For
    /// example, this field will be 2 if two products are added to the shopping
    /// cart for `add-to-cart` event. Required for `add-to-cart`, `add-to-list`,
    /// `remove-from-cart`, `checkout-start`, `purchase-complete`, `refund` event
    /// types.
    #[prost(int32, tag = "6")]
    pub quantity: i32,
    /// Optional. Quantity of the products in stock when a user event happens.
    /// Optional. If provided, this overrides the available quantity in Catalog for
    /// this event. and can only be set if `stock_status` is set to `IN_STOCK`.
    ///
    /// Note that if an item is out of stock, you must set the `stock_state` field
    /// to be `OUT_OF_STOCK`. Leaving this field unspecified / as zero is not
    /// sufficient to mark the item out of stock.
    #[prost(int32, tag = "7")]
    pub available_quantity: i32,
    /// Optional. Extra features associated with a product in the user event.
    #[prost(message, optional, tag = "8")]
    pub item_attributes: ::std::option::Option<FeatureMap>,
}
/// Google Cloud Storage location for input content.
/// format.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GcsSource {
    /// Required. Google Cloud Storage URIs to input files. URI can be up to
    /// 2000 characters long. URIs can match the full object path (for example,
    /// gs://bucket/directory/object.json) or a pattern matching one or more
    /// files, such as gs://bucket/directory/*.json. A request can
    /// contain at most 100 files, and each file can be up to 2 GB. See
    /// [Importing catalog information](/recommendations-ai/docs/upload-catalog)
    /// for the expected file format and setup instructions.
    #[prost(string, repeated, tag = "1")]
    pub input_uris: ::std::vec::Vec<std::string::String>,
}
/// The inline source for the input config for ImportCatalogItems method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CatalogInlineSource {
    /// Optional. A list of catalog items to update/create. Recommended max of 10k
    /// items.
    #[prost(message, repeated, tag = "1")]
    pub catalog_items: ::std::vec::Vec<CatalogItem>,
}
/// The inline source for the input config for ImportUserEvents method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserEventInlineSource {
    /// Optional. A list of user events to import. Recommended max of 10k items.
    #[prost(message, repeated, tag = "1")]
    pub user_events: ::std::vec::Vec<UserEvent>,
}
/// Configuration of destination for Import related errors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportErrorsConfig {
    /// Required. Errors destination.
    #[prost(oneof = "import_errors_config::Destination", tags = "1")]
    pub destination: ::std::option::Option<import_errors_config::Destination>,
}
pub mod import_errors_config {
    /// Required. Errors destination.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Destination {
        /// Google Cloud Storage path for import errors. This must be an empty,
        /// existing Cloud Storage bucket. Import errors will be written to a file in
        /// this bucket, one per line, as a JSON-encoded
        /// `google.rpc.Status` message.
        #[prost(string, tag = "1")]
        GcsPrefix(std::string::String),
    }
}
/// Request message for Import methods.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportCatalogItemsRequest {
    /// Required. "projects/1234/locations/global/catalogs/default_catalog"
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Optional. Unique identifier provided by client, within the ancestor
    /// dataset scope. Ensures idempotency and used for request deduplication.
    /// Server-generated if unspecified. Up to 128 characters long. This is
    /// returned as google.longrunning.Operation.name in the response.
    #[prost(string, tag = "2")]
    pub request_id: std::string::String,
    /// Required. The desired input location of the data.
    #[prost(message, optional, tag = "3")]
    pub input_config: ::std::option::Option<InputConfig>,
    /// Optional. The desired location of errors incurred during the Import.
    #[prost(message, optional, tag = "4")]
    pub errors_config: ::std::option::Option<ImportErrorsConfig>,
}
/// Request message for the ImportUserEvents request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportUserEventsRequest {
    /// Required.
    /// "projects/1234/locations/global/catalogs/default_catalog/eventStores/default_event_store"
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Optional. Unique identifier provided by client, within the ancestor
    /// dataset scope. Ensures idempotency for expensive long running operations.
    /// Server-generated if unspecified. Up to 128 characters long. This is
    /// returned as google.longrunning.Operation.name in the response. Note that
    /// this field must not be set if the desired input config is
    /// catalog_inline_source.
    #[prost(string, tag = "2")]
    pub request_id: std::string::String,
    /// Required. The desired input location of the data.
    #[prost(message, optional, tag = "3")]
    pub input_config: ::std::option::Option<InputConfig>,
    /// Optional. The desired location of errors incurred during the Import.
    #[prost(message, optional, tag = "4")]
    pub errors_config: ::std::option::Option<ImportErrorsConfig>,
}
/// The input config source.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InputConfig {
    /// Required. The source of the input.
    #[prost(oneof = "input_config::Source", tags = "1, 2, 3")]
    pub source: ::std::option::Option<input_config::Source>,
}
pub mod input_config {
    /// Required. The source of the input.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Source {
        /// The Inline source for the input content for Catalog items.
        #[prost(message, tag = "1")]
        CatalogInlineSource(super::CatalogInlineSource),
        /// Google Cloud Storage location for the input content.
        #[prost(message, tag = "2")]
        GcsSource(super::GcsSource),
        /// The Inline source for the input content for UserEvents.
        #[prost(message, tag = "3")]
        UserEventInlineSource(super::UserEventInlineSource),
    }
}
/// Metadata related to the progress of the Import operation. This will be
/// returned by the google.longrunning.Operation.metadata field.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportMetadata {
    /// Name of the operation.
    #[prost(string, tag = "5")]
    pub operation_name: std::string::String,
    /// Id of the request / operation. This is parroting back the requestId that
    /// was passed in the request.
    #[prost(string, tag = "3")]
    pub request_id: std::string::String,
    /// Operation create time.
    #[prost(message, optional, tag = "4")]
    pub create_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Count of entries that were processed successfully.
    #[prost(int64, tag = "1")]
    pub success_count: i64,
    /// Count of entries that encountered errors while processing.
    #[prost(int64, tag = "2")]
    pub failure_count: i64,
    /// Operation last update time. If the operation is done, this is also the
    /// finish time.
    #[prost(message, optional, tag = "6")]
    pub update_time: ::std::option::Option<::prost_types::Timestamp>,
}
/// Response of the ImportCatalogItemsRequest. If the long running
/// operation is done, then this message is returned by the
/// google.longrunning.Operations.response field if the operation was successful.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportCatalogItemsResponse {
    /// A sample of errors encountered while processing the request.
    #[prost(message, repeated, tag = "1")]
    pub error_samples: ::std::vec::Vec<super::super::super::rpc::Status>,
    /// Echoes the destination for the complete errors in the request if set.
    #[prost(message, optional, tag = "2")]
    pub errors_config: ::std::option::Option<ImportErrorsConfig>,
}
/// Response of the ImportUserEventsRequest. If the long running
/// operation was successful, then this message is returned by the
/// google.longrunning.Operations.response field if the operation was successful.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportUserEventsResponse {
    /// A sample of errors encountered while processing the request.
    #[prost(message, repeated, tag = "1")]
    pub error_samples: ::std::vec::Vec<super::super::super::rpc::Status>,
    /// Echoes the destination for the complete errors if this field was set in
    /// the request.
    #[prost(message, optional, tag = "2")]
    pub errors_config: ::std::option::Option<ImportErrorsConfig>,
    /// Aggregated statistics of user event import status.
    #[prost(message, optional, tag = "3")]
    pub import_summary: ::std::option::Option<UserEventImportSummary>,
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
/// Request message for CreateCatalogItem method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateCatalogItemRequest {
    /// Required. The parent catalog resource name, such as
    /// "projects/*/locations/global/catalogs/default_catalog".
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Required. The catalog item to create.
    #[prost(message, optional, tag = "2")]
    pub catalog_item: ::std::option::Option<CatalogItem>,
}
/// Request message for GetCatalogItem method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCatalogItemRequest {
    /// Required. Full resource name of catalog item, such as
    /// "projects/*/locations/global/catalogs/default_catalog/catalogitems/some_catalog_item_id".
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Request message for ListCatalogItems method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCatalogItemsRequest {
    /// Required. The parent catalog resource name, such as
    /// "projects/*/locations/global/catalogs/default_catalog".
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Optional. Maximum number of results to return per page. If zero, the
    /// service will choose a reasonable default.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. The previous ListCatalogItemsResponse.next_page_token.
    #[prost(string, tag = "3")]
    pub page_token: std::string::String,
    /// Optional. A filter to apply on the list results.
    #[prost(string, tag = "4")]
    pub filter: std::string::String,
}
/// Response message for ListCatalogItems method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCatalogItemsResponse {
    /// The catalog items.
    #[prost(message, repeated, tag = "1")]
    pub catalog_items: ::std::vec::Vec<CatalogItem>,
    /// If empty, the list is complete. If nonempty, the token to pass to the next
    /// request's ListCatalogItemRequest.page_token.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
}
/// Request message for UpdateCatalogItem method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateCatalogItemRequest {
    /// Required. Full resource name of catalog item, such as
    /// "projects/*/locations/global/catalogs/default_catalog/catalogItems/some_catalog_item_id".
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Required. The catalog item to update/create. The 'catalog_item_id' field
    /// has to match that in the 'name'.
    #[prost(message, optional, tag = "2")]
    pub catalog_item: ::std::option::Option<CatalogItem>,
    /// Optional. Indicates which fields in the provided 'item' to update. If not
    /// set, will by default update all fields.
    #[prost(message, optional, tag = "3")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
}
/// Request message for DeleteCatalogItem method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteCatalogItemRequest {
    /// Required. Full resource name of catalog item, such as
    /// "projects/*/locations/global/catalogs/default_catalog/catalogItems/some_catalog_item_id".
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
#[doc = r" Generated client implementations."]
pub mod catalog_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Service for ingesting catalog information of the customer's website."]
    pub struct CatalogServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl CatalogServiceClient<tonic::transport::Channel> {
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
        #[doc = " Creates a catalog item."]
        pub async fn create_catalog_item(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateCatalogItemRequest>,
        ) -> Result<tonic::Response<super::CatalogItem>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.recommendationengine.v1beta1.CatalogService/CreateCatalogItem",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets a specific catalog item."]
        pub async fn get_catalog_item(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCatalogItemRequest>,
        ) -> Result<tonic::Response<super::CatalogItem>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.recommendationengine.v1beta1.CatalogService/GetCatalogItem",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets a list of catalog items."]
        pub async fn list_catalog_items(
            &mut self,
            request: impl tonic::IntoRequest<super::ListCatalogItemsRequest>,
        ) -> Result<tonic::Response<super::ListCatalogItemsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.recommendationengine.v1beta1.CatalogService/ListCatalogItems",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates a catalog item. Partial updating is supported. Non-existing"]
        #[doc = " items will be created."]
        pub async fn update_catalog_item(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateCatalogItemRequest>,
        ) -> Result<tonic::Response<super::CatalogItem>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.recommendationengine.v1beta1.CatalogService/UpdateCatalogItem",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes a catalog item."]
        pub async fn delete_catalog_item(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteCatalogItemRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.recommendationengine.v1beta1.CatalogService/DeleteCatalogItem",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Bulk import of multiple catalog items. Request processing may be"]
        #[doc = " synchronous. No partial updating supported. Non-existing items will be"]
        #[doc = " created."]
        #[doc = ""]
        #[doc = " Operation.response is of type ImportResponse. Note that it is"]
        #[doc = " possible for a subset of the items to be successfully updated."]
        pub async fn import_catalog_items(
            &mut self,
            request: impl tonic::IntoRequest<super::ImportCatalogItemsRequest>,
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
                "/google.cloud.recommendationengine.v1beta1.CatalogService/ImportCatalogItems",
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
#[doc = r" Generated server implementations."]
pub mod catalog_service_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with CatalogServiceServer."]
    #[async_trait]
    pub trait CatalogService: Send + Sync + 'static {
        #[doc = " Creates a catalog item."]
        async fn create_catalog_item(
            &self,
            request: tonic::Request<super::CreateCatalogItemRequest>,
        ) -> Result<tonic::Response<super::CatalogItem>, tonic::Status>;
        #[doc = " Gets a specific catalog item."]
        async fn get_catalog_item(
            &self,
            request: tonic::Request<super::GetCatalogItemRequest>,
        ) -> Result<tonic::Response<super::CatalogItem>, tonic::Status>;
        #[doc = " Gets a list of catalog items."]
        async fn list_catalog_items(
            &self,
            request: tonic::Request<super::ListCatalogItemsRequest>,
        ) -> Result<tonic::Response<super::ListCatalogItemsResponse>, tonic::Status>;
        #[doc = " Updates a catalog item. Partial updating is supported. Non-existing"]
        #[doc = " items will be created."]
        async fn update_catalog_item(
            &self,
            request: tonic::Request<super::UpdateCatalogItemRequest>,
        ) -> Result<tonic::Response<super::CatalogItem>, tonic::Status>;
        #[doc = " Deletes a catalog item."]
        async fn delete_catalog_item(
            &self,
            request: tonic::Request<super::DeleteCatalogItemRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status>;
        #[doc = " Bulk import of multiple catalog items. Request processing may be"]
        #[doc = " synchronous. No partial updating supported. Non-existing items will be"]
        #[doc = " created."]
        #[doc = ""]
        #[doc = " Operation.response is of type ImportResponse. Note that it is"]
        #[doc = " possible for a subset of the items to be successfully updated."]
        async fn import_catalog_items(
            &self,
            request: tonic::Request<super::ImportCatalogItemsRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        >;
    }
    #[doc = " Service for ingesting catalog information of the customer's website."]
    #[derive(Debug)]
    #[doc(hidden)]
    pub struct CatalogServiceServer<T: CatalogService> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: CatalogService> CatalogServiceServer<T> {
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
    impl<T, B> Service<http::Request<B>> for CatalogServiceServer<T>
    where
        T: CatalogService,
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
                "/google.cloud.recommendationengine.v1beta1.CatalogService/CreateCatalogItem" => {
                    #[allow(non_camel_case_types)]
                    struct CreateCatalogItemSvc<T: CatalogService>(pub Arc<T>);
                    impl<T: CatalogService>
                        tonic::server::UnaryService<super::CreateCatalogItemRequest>
                        for CreateCatalogItemSvc<T>
                    {
                        type Response = super::CatalogItem;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateCatalogItemRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.create_catalog_item(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = CreateCatalogItemSvc(inner);
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
                "/google.cloud.recommendationengine.v1beta1.CatalogService/GetCatalogItem" => {
                    #[allow(non_camel_case_types)]
                    struct GetCatalogItemSvc<T: CatalogService>(pub Arc<T>);
                    impl<T: CatalogService>
                        tonic::server::UnaryService<super::GetCatalogItemRequest>
                        for GetCatalogItemSvc<T>
                    {
                        type Response = super::CatalogItem;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetCatalogItemRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.get_catalog_item(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetCatalogItemSvc(inner);
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
                "/google.cloud.recommendationengine.v1beta1.CatalogService/ListCatalogItems" => {
                    #[allow(non_camel_case_types)]
                    struct ListCatalogItemsSvc<T: CatalogService>(pub Arc<T>);
                    impl<T: CatalogService>
                        tonic::server::UnaryService<super::ListCatalogItemsRequest>
                        for ListCatalogItemsSvc<T>
                    {
                        type Response = super::ListCatalogItemsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListCatalogItemsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.list_catalog_items(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ListCatalogItemsSvc(inner);
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
                "/google.cloud.recommendationengine.v1beta1.CatalogService/UpdateCatalogItem" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateCatalogItemSvc<T: CatalogService>(pub Arc<T>);
                    impl<T: CatalogService>
                        tonic::server::UnaryService<super::UpdateCatalogItemRequest>
                        for UpdateCatalogItemSvc<T>
                    {
                        type Response = super::CatalogItem;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateCatalogItemRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.update_catalog_item(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = UpdateCatalogItemSvc(inner);
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
                "/google.cloud.recommendationengine.v1beta1.CatalogService/DeleteCatalogItem" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteCatalogItemSvc<T: CatalogService>(pub Arc<T>);
                    impl<T: CatalogService>
                        tonic::server::UnaryService<super::DeleteCatalogItemRequest>
                        for DeleteCatalogItemSvc<T>
                    {
                        type Response = ();
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteCatalogItemRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.delete_catalog_item(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = DeleteCatalogItemSvc(inner);
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
                "/google.cloud.recommendationengine.v1beta1.CatalogService/ImportCatalogItems" => {
                    #[allow(non_camel_case_types)]
                    struct ImportCatalogItemsSvc<T: CatalogService>(pub Arc<T>);
                    impl<T: CatalogService>
                        tonic::server::UnaryService<super::ImportCatalogItemsRequest>
                        for ImportCatalogItemsSvc<T>
                    {
                        type Response = super::super::super::super::longrunning::Operation;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ImportCatalogItemsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.import_catalog_items(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ImportCatalogItemsSvc(inner);
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
    impl<T: CatalogService> Clone for CatalogServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: CatalogService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: CatalogService> tonic::transport::NamedService for CatalogServiceServer<T> {
        const NAME: &'static str = "google.cloud.recommendationengine.v1beta1.CatalogService";
    }
}
/// Registered Api Key.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PredictionApiKeyRegistration {
    /// The API key.
    #[prost(string, tag = "1")]
    pub api_key: std::string::String,
}
/// Request message for the `CreatePredictionApiKeyRegistration` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreatePredictionApiKeyRegistrationRequest {
    /// Required. The parent resource path.
    /// "projects/*/locations/global/catalogs/default_catalog/eventStores/default_event_store".
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Required. The prediction API key registration.
    #[prost(message, optional, tag = "2")]
    pub prediction_api_key_registration: ::std::option::Option<PredictionApiKeyRegistration>,
}
/// Request message for the `ListPredictionApiKeyRegistrations`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPredictionApiKeyRegistrationsRequest {
    /// Required. The parent placement resource name such as
    /// "projects/1234/locations/global/catalogs/default_catalog/eventStores/default_event_store"
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Optional. Maximum number of results to return per page. If unset, the
    /// service will choose a reasonable default.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. The previous `ListPredictionApiKeyRegistration.nextPageToken`.
    #[prost(string, tag = "3")]
    pub page_token: std::string::String,
}
/// Response message for the `ListPredictionApiKeyRegistrations`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPredictionApiKeyRegistrationsResponse {
    /// The list of registered API keys.
    #[prost(message, repeated, tag = "1")]
    pub prediction_api_key_registrations: ::std::vec::Vec<PredictionApiKeyRegistration>,
    /// If empty, the list is complete. If nonempty, pass the token to the next
    /// request's `ListPredictionApiKeysRegistrationsRequest.pageToken`.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
}
/// Request message for `DeletePredictionApiKeyRegistration` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeletePredictionApiKeyRegistrationRequest {
    /// Required. The API key to unregister including full resource path.
    /// "projects/*/locations/global/catalogs/default_catalog/eventStores/default_event_store/predictionApiKeyRegistrations/<YOUR_API_KEY>"
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
#[doc = r" Generated client implementations."]
pub mod prediction_api_key_registry_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Service for registering API keys for use with the `predict` method. If you"]
    #[doc = " use an API key to request predictions, you must first register the API key."]
    #[doc = " Otherwise, your prediction request is rejected. If you use OAuth to"]
    #[doc = " authenticate your `predict` method call, you do not need to register an API"]
    #[doc = " key. You can register up to 20 API keys per project."]
    pub struct PredictionApiKeyRegistryClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl PredictionApiKeyRegistryClient<tonic::transport::Channel> {
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
    impl<T> PredictionApiKeyRegistryClient<T>
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
        #[doc = " Register an API key for use with predict method."]
        pub async fn create_prediction_api_key_registration(
            &mut self,
            request: impl tonic::IntoRequest<super::CreatePredictionApiKeyRegistrationRequest>,
        ) -> Result<tonic::Response<super::PredictionApiKeyRegistration>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ( "/google.cloud.recommendationengine.v1beta1.PredictionApiKeyRegistry/CreatePredictionApiKeyRegistration" ) ;
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " List the registered apiKeys for use with predict method."]
        pub async fn list_prediction_api_key_registrations(
            &mut self,
            request: impl tonic::IntoRequest<super::ListPredictionApiKeyRegistrationsRequest>,
        ) -> Result<tonic::Response<super::ListPredictionApiKeyRegistrationsResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ( "/google.cloud.recommendationengine.v1beta1.PredictionApiKeyRegistry/ListPredictionApiKeyRegistrations" ) ;
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Unregister an apiKey from using for predict method."]
        pub async fn delete_prediction_api_key_registration(
            &mut self,
            request: impl tonic::IntoRequest<super::DeletePredictionApiKeyRegistrationRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ( "/google.cloud.recommendationengine.v1beta1.PredictionApiKeyRegistry/DeletePredictionApiKeyRegistration" ) ;
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for PredictionApiKeyRegistryClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for PredictionApiKeyRegistryClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "PredictionApiKeyRegistryClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod prediction_api_key_registry_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with PredictionApiKeyRegistryServer."]
    #[async_trait]
    pub trait PredictionApiKeyRegistry: Send + Sync + 'static {
        #[doc = " Register an API key for use with predict method."]
        async fn create_prediction_api_key_registration(
            &self,
            request: tonic::Request<super::CreatePredictionApiKeyRegistrationRequest>,
        ) -> Result<tonic::Response<super::PredictionApiKeyRegistration>, tonic::Status>;
        #[doc = " List the registered apiKeys for use with predict method."]
        async fn list_prediction_api_key_registrations(
            &self,
            request: tonic::Request<super::ListPredictionApiKeyRegistrationsRequest>,
        ) -> Result<tonic::Response<super::ListPredictionApiKeyRegistrationsResponse>, tonic::Status>;
        #[doc = " Unregister an apiKey from using for predict method."]
        async fn delete_prediction_api_key_registration(
            &self,
            request: tonic::Request<super::DeletePredictionApiKeyRegistrationRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status>;
    }
    #[doc = " Service for registering API keys for use with the `predict` method. If you"]
    #[doc = " use an API key to request predictions, you must first register the API key."]
    #[doc = " Otherwise, your prediction request is rejected. If you use OAuth to"]
    #[doc = " authenticate your `predict` method call, you do not need to register an API"]
    #[doc = " key. You can register up to 20 API keys per project."]
    #[derive(Debug)]
    #[doc(hidden)]
    pub struct PredictionApiKeyRegistryServer<T: PredictionApiKeyRegistry> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: PredictionApiKeyRegistry> PredictionApiKeyRegistryServer<T> {
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
    impl<T, B> Service<http::Request<B>> for PredictionApiKeyRegistryServer<T>
    where
        T: PredictionApiKeyRegistry,
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
            match req . uri ( ) . path ( ) { "/google.cloud.recommendationengine.v1beta1.PredictionApiKeyRegistry/CreatePredictionApiKeyRegistration" => { # [ allow ( non_camel_case_types ) ] struct CreatePredictionApiKeyRegistrationSvc < T : PredictionApiKeyRegistry > ( pub Arc < T > ) ; impl < T : PredictionApiKeyRegistry > tonic :: server :: UnaryService < super :: CreatePredictionApiKeyRegistrationRequest > for CreatePredictionApiKeyRegistrationSvc < T > { type Response = super :: PredictionApiKeyRegistration ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: CreatePredictionApiKeyRegistrationRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { inner . create_prediction_api_key_registration ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = CreatePredictionApiKeyRegistrationSvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } "/google.cloud.recommendationengine.v1beta1.PredictionApiKeyRegistry/ListPredictionApiKeyRegistrations" => { # [ allow ( non_camel_case_types ) ] struct ListPredictionApiKeyRegistrationsSvc < T : PredictionApiKeyRegistry > ( pub Arc < T > ) ; impl < T : PredictionApiKeyRegistry > tonic :: server :: UnaryService < super :: ListPredictionApiKeyRegistrationsRequest > for ListPredictionApiKeyRegistrationsSvc < T > { type Response = super :: ListPredictionApiKeyRegistrationsResponse ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: ListPredictionApiKeyRegistrationsRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { inner . list_prediction_api_key_registrations ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = ListPredictionApiKeyRegistrationsSvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } "/google.cloud.recommendationengine.v1beta1.PredictionApiKeyRegistry/DeletePredictionApiKeyRegistration" => { # [ allow ( non_camel_case_types ) ] struct DeletePredictionApiKeyRegistrationSvc < T : PredictionApiKeyRegistry > ( pub Arc < T > ) ; impl < T : PredictionApiKeyRegistry > tonic :: server :: UnaryService < super :: DeletePredictionApiKeyRegistrationRequest > for DeletePredictionApiKeyRegistrationSvc < T > { type Response = ( ) ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: DeletePredictionApiKeyRegistrationRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { inner . delete_prediction_api_key_registration ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = DeletePredictionApiKeyRegistrationSvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } _ => Box :: pin ( async move { Ok ( http :: Response :: builder ( ) . status ( 200 ) . header ( "grpc-status" , "12" ) . body ( tonic :: body :: BoxBody :: empty ( ) ) . unwrap ( ) ) } ) , }
        }
    }
    impl<T: PredictionApiKeyRegistry> Clone for PredictionApiKeyRegistryServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: PredictionApiKeyRegistry> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: PredictionApiKeyRegistry> tonic::transport::NamedService
        for PredictionApiKeyRegistryServer<T>
    {
        const NAME: &'static str =
            "google.cloud.recommendationengine.v1beta1.PredictionApiKeyRegistry";
    }
}
/// Request message for Predict method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PredictRequest {
    /// Required. Full resource name of the format:
    /// {name=projects/*/locations/global/catalogs/default_catalog/eventStores/default_event_store/placements/*}
    /// The id of the recommendation engine placement. This id is used to identify
    /// the set of models that will be used to make the prediction.
    ///
    /// We currently support three placements with the following IDs by default:
    ///
    /// * `shopping_cart`: Predicts items frequently bought together with one or
    ///   more catalog items in the same shopping session. Commonly displayed after
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
    ///   pages. For example - More items like this.
    ///
    /// * `recently_viewed_default`: Returns up to 75 items recently viewed by the
    ///   specified `userId` or `visitorId`, most recent ones first. Returns
    ///   nothing if neither of them has viewed any items yet. For example -
    ///   Recently viewed.
    ///
    /// The full list of available placements can be seen at
    /// https://console.cloud.google.com/recommendation/datafeeds/default_catalog/dashboard
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Required. Context about the user, what they are looking at and what action
    /// they took to trigger the predict request. Note that this user event detail
    /// won't be ingested to userEvent logs. Thus, a separate userEvent write
    /// request is required for event logging.
    #[prost(message, optional, tag = "2")]
    pub user_event: ::std::option::Option<UserEvent>,
    /// Optional. Maximum number of results to return per page. Set this property
    /// to the number of prediction results required. If zero, the service will
    /// choose a reasonable default.
    #[prost(int32, tag = "7")]
    pub page_size: i32,
    /// Optional. The previous PredictResponse.next_page_token.
    #[prost(string, tag = "8")]
    pub page_token: std::string::String,
    /// Optional. Filter for restricting prediction results. Accepts values for
    /// tags and the `filterOutOfStockItems` flag.
    ///
    ///  * Tag expressions. Restricts predictions to items that match all of the
    ///    specified tags. Boolean operators `OR` and `NOT` are supported if the
    ///    expression is enclosed in parentheses, and must be separated from the
    ///    tag values by a space. `-"tagA"` is also supported and is equivalent to
    ///    `NOT "tagA"`. Tag values must be double quoted UTF-8 encoded strings
    ///    with a size limit of 1 KiB.
    ///
    ///  * filterOutOfStockItems. Restricts predictions to items that do not have a
    ///    stockState value of OUT_OF_STOCK.
    ///
    /// Examples:
    ///
    ///  * tag=("Red" OR "Blue") tag="New-Arrival" tag=(NOT "promotional")
    ///  * filterOutOfStockItems  tag=(-"promotional")
    ///  * filterOutOfStockItems
    #[prost(string, tag = "3")]
    pub filter: std::string::String,
    /// Optional. Use dryRun mode for this prediction query. If set to true, a
    /// dummy model will be used that returns arbitrary catalog items.
    /// Note that the dryRun mode should only be used for testing the API, or if
    /// the model is not ready.
    #[prost(bool, tag = "4")]
    pub dry_run: bool,
    /// Optional. Additional domain specific parameters for the predictions.
    ///
    /// Allowed values:
    ///
    /// * `returnCatalogItem`: Boolean. If set to true, the associated catalogItem
    ///    object will be returned in the
    ///   `PredictResponse.PredictionResult.itemMetadata` object in the method
    ///    response.
    /// * `returnItemScore`: Boolean. If set to true, the prediction 'score'
    ///    corresponding to each returned item will be set in the `metadata`
    ///    field in the prediction response. The given 'score' indicates the
    ///    probability of an item being clicked/purchased given the user's context
    ///    and history.
    #[prost(map = "string, message", tag = "6")]
    pub params: ::std::collections::HashMap<std::string::String, ::prost_types::Value>,
    /// Optional. The labels for the predict request.
    ///
    ///  * Label keys can contain lowercase letters, digits and hyphens, must start
    ///    with a letter, and must end with a letter or digit.
    ///  * Non-zero label values can contain lowercase letters, digits and hyphens,
    ///    must start with a letter, and must end with a letter or digit.
    ///  * No more than 64 labels can be associated with a given request.
    ///
    /// See https://goo.gl/xmQnxf for more information on and examples of labels.
    #[prost(map = "string, string", tag = "9")]
    pub labels: ::std::collections::HashMap<std::string::String, std::string::String>,
}
/// Response message for predict method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PredictResponse {
    /// A list of recommended items. The order represents the ranking (from the
    /// most relevant item to the least).
    #[prost(message, repeated, tag = "1")]
    pub results: ::std::vec::Vec<predict_response::PredictionResult>,
    /// A unique recommendation token. This should be included in the user event
    /// logs resulting from this recommendation, which enables accurate attribution
    /// of recommendation model performance.
    #[prost(string, tag = "2")]
    pub recommendation_token: std::string::String,
    /// IDs of items in the request that were missing from the catalog.
    #[prost(string, repeated, tag = "3")]
    pub items_missing_in_catalog: ::std::vec::Vec<std::string::String>,
    /// True if the dryRun property was set in the request.
    #[prost(bool, tag = "4")]
    pub dry_run: bool,
    /// Additional domain specific prediction response metadata.
    #[prost(map = "string, message", tag = "5")]
    pub metadata: ::std::collections::HashMap<std::string::String, ::prost_types::Value>,
    /// If empty, the list is complete. If nonempty, the token to pass to the next
    /// request's PredictRequest.page_token.
    #[prost(string, tag = "6")]
    pub next_page_token: std::string::String,
}
pub mod predict_response {
    /// PredictionResult represents the recommendation prediction results.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PredictionResult {
        /// ID of the recommended catalog item
        #[prost(string, tag = "1")]
        pub id: std::string::String,
        /// Additional item metadata / annotations.
        ///
        /// Possible values:
        ///
        /// * `catalogItem`: JSON representation of the catalogItem. Will be set if
        ///   `returnCatalogItem` is set to true in `PredictRequest.params`.
        /// * `score`: Prediction score in double value. Will be set if
        ///   `returnItemScore` is set to true in `PredictRequest.params`.
        #[prost(map = "string, message", tag = "2")]
        pub item_metadata: ::std::collections::HashMap<std::string::String, ::prost_types::Value>,
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
    impl PredictionServiceClient<tonic::transport::Channel> {
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
        #[doc = " Makes a recommendation prediction. If using API Key based authentication,"]
        #[doc = " the API Key must be registered using the"]
        #[doc = " [PredictionApiKeyRegistry][google.cloud.recommendationengine.v1beta1.PredictionApiKeyRegistry]"]
        #[doc = " service. [Learn more](/recommendations-ai/docs/setting-up#register-key)."]
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
                "/google.cloud.recommendationengine.v1beta1.PredictionService/Predict",
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
#[doc = r" Generated server implementations."]
pub mod prediction_service_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with PredictionServiceServer."]
    #[async_trait]
    pub trait PredictionService: Send + Sync + 'static {
        #[doc = " Makes a recommendation prediction. If using API Key based authentication,"]
        #[doc = " the API Key must be registered using the"]
        #[doc = " [PredictionApiKeyRegistry][google.cloud.recommendationengine.v1beta1.PredictionApiKeyRegistry]"]
        #[doc = " service. [Learn more](/recommendations-ai/docs/setting-up#register-key)."]
        async fn predict(
            &self,
            request: tonic::Request<super::PredictRequest>,
        ) -> Result<tonic::Response<super::PredictResponse>, tonic::Status>;
    }
    #[doc = " Service for making recommendation prediction."]
    #[derive(Debug)]
    #[doc(hidden)]
    pub struct PredictionServiceServer<T: PredictionService> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: PredictionService> PredictionServiceServer<T> {
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
    impl<T, B> Service<http::Request<B>> for PredictionServiceServer<T>
    where
        T: PredictionService,
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
                "/google.cloud.recommendationengine.v1beta1.PredictionService/Predict" => {
                    #[allow(non_camel_case_types)]
                    struct PredictSvc<T: PredictionService>(pub Arc<T>);
                    impl<T: PredictionService> tonic::server::UnaryService<super::PredictRequest> for PredictSvc<T> {
                        type Response = super::PredictResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PredictRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.predict(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = PredictSvc(inner);
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
    impl<T: PredictionService> Clone for PredictionServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: PredictionService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: PredictionService> tonic::transport::NamedService for PredictionServiceServer<T> {
        const NAME: &'static str = "google.cloud.recommendationengine.v1beta1.PredictionService";
    }
}
/// Request message for PurgeUserEvents method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PurgeUserEventsRequest {
    /// Required. The resource name of the event_store under which the events are
    /// created. The format is
    /// "projects/${projectId}/locations/global/catalogs/${catalogId}/eventStores/${eventStoreId}"
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Required. The filter string to specify the events to be deleted. Empty
    /// string filter is not allowed. This filter can also be used with
    /// ListUserEvents API to list events that will be deleted. The eligible fields
    /// for filtering are:
    /// * eventType - UserEvent.eventType field of type string.
    /// * eventTime - in ISO 8601 "zulu" format.
    /// * visitorId - field of type string. Specifying this will delete all events
    /// associated with a visitor.
    /// * userId - field of type string. Specifying this will delete all events
    /// associated with a user.
    /// Example 1: Deleting all events in a time range.
    /// `eventTime > "2012-04-23T18:25:43.511Z" eventTime <
    /// "2012-04-23T18:30:43.511Z"`
    /// Example 2: Deleting specific eventType in time range.
    /// `eventTime > "2012-04-23T18:25:43.511Z" eventType = "detail-page-view"`
    /// Example 3: Deleting all events for a specific visitor
    /// `visitorId = visitor1024`
    /// The filtering fields are assumed to have an implicit AND.
    #[prost(string, tag = "2")]
    pub filter: std::string::String,
    /// Optional. The default value is false. Override this flag to true to
    /// actually perform the purge. If the field is not set to true, a sampling of
    /// events to be deleted will be returned.
    #[prost(bool, tag = "3")]
    pub force: bool,
}
/// Metadata related to the progress of the PurgeUserEvents operation.
/// This will be returned by the google.longrunning.Operation.metadata field.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PurgeUserEventsMetadata {
    /// The ID of the request / operation.
    #[prost(string, tag = "1")]
    pub operation_name: std::string::String,
    /// Operation create time.
    #[prost(message, optional, tag = "2")]
    pub create_time: ::std::option::Option<::prost_types::Timestamp>,
}
/// Response of the PurgeUserEventsRequest. If the long running operation is
/// successfully done, then this message is returned by the
/// google.longrunning.Operations.response field.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PurgeUserEventsResponse {
    /// The total count of events purged as a result of the operation.
    #[prost(int64, tag = "1")]
    pub purged_events_count: i64,
    /// A sampling of events deleted (or will be deleted) depending on the `force`
    /// property in the request. Max of 500 items will be returned.
    #[prost(message, repeated, tag = "2")]
    pub user_events_sample: ::std::vec::Vec<UserEvent>,
}
/// Request message for WriteUserEvent method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WriteUserEventRequest {
    /// Required. The parent eventStore resource name, such as
    /// "projects/1234/locations/global/catalogs/default_catalog/eventStores/default_event_store".
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Required. User event to write.
    #[prost(message, optional, tag = "2")]
    pub user_event: ::std::option::Option<UserEvent>,
}
/// Request message for CollectUserEvent method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CollectUserEventRequest {
    /// Required. The parent eventStore name, such as
    /// "projects/1234/locations/global/catalogs/default_catalog/eventStores/default_event_store".
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Required. URL encoded UserEvent proto.
    #[prost(string, tag = "2")]
    pub user_event: std::string::String,
    /// Optional. The url including cgi-parameters but excluding the hash fragment.
    /// The URL must be truncated to 1.5K bytes to conservatively be under the 2K
    /// bytes. This is often more useful than the referer url, because many
    /// browsers only send the domain for 3rd party requests.
    #[prost(string, tag = "3")]
    pub uri: std::string::String,
    /// Optional. The event timestamp in milliseconds. This prevents browser
    /// caching of otherwise identical get requests. The name is abbreviated to
    /// reduce the payload bytes.
    #[prost(int64, tag = "4")]
    pub ets: i64,
}
/// Request message for ListUserEvents method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListUserEventsRequest {
    /// Required. The parent eventStore resource name, such as
    /// "projects/*/locations/*/catalogs/default_catalog/eventStores/default_event_store".
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Optional. Maximum number of results to return per page. If zero, the
    /// service will choose a reasonable default.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. The previous ListUserEventsResponse.next_page_token.
    #[prost(string, tag = "3")]
    pub page_token: std::string::String,
    /// Optional. Filtering expression to specify restrictions over
    /// returned events. This is a sequence of terms, where each term applies some
    /// kind of a restriction to the returned user events. Use this expression to
    /// restrict results to a specific time range, or filter events by eventType.
    ///    eg: eventTime > "2012-04-23T18:25:43.511Z" eventsMissingCatalogItems
    ///    eventTime<"2012-04-23T18:25:43.511Z" eventType=search
    ///
    ///   We expect only 3 types of fields:
    ///
    ///    * eventTime: this can be specified a maximum of 2 times, once with a
    ///      less than operator and once with a greater than operator. The
    ///      eventTime restrict should result in one contiguous valid eventTime
    ///      range.
    ///
    ///    * eventType: only 1 eventType restriction can be specified.
    ///
    ///    * eventsMissingCatalogItems: specififying this will restrict results
    ///      to events for which catalog items were not found in the catalog. The
    ///      default behavior is to return only those events for which catalog
    ///      items were found.
    ///
    ///   Some examples of valid filters expressions:
    ///
    ///   * Example 1: eventTime > "2012-04-23T18:25:43.511Z"
    ///             eventTime < "2012-04-23T18:30:43.511Z"
    ///   * Example 2: eventTime > "2012-04-23T18:25:43.511Z"
    ///             eventType = detail-page-view
    ///   * Example 3: eventsMissingCatalogItems
    ///             eventType = search eventTime < "2018-04-23T18:30:43.511Z"
    ///   * Example 4: eventTime > "2012-04-23T18:25:43.511Z"
    ///   * Example 5: eventType = search
    ///   * Example 6: eventsMissingCatalogItems
    #[prost(string, tag = "4")]
    pub filter: std::string::String,
}
/// Response message for ListUserEvents method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListUserEventsResponse {
    /// The user events.
    #[prost(message, repeated, tag = "1")]
    pub user_events: ::std::vec::Vec<UserEvent>,
    /// If empty, the list is complete. If nonempty, the token to pass to the next
    /// request's ListUserEvents.page_token.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
}
#[doc = r" Generated client implementations."]
pub mod user_event_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Service for ingesting end user actions on the customer website."]
    pub struct UserEventServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl UserEventServiceClient<tonic::transport::Channel> {
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
                "/google.cloud.recommendationengine.v1beta1.UserEventService/WriteUserEvent",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Writes a single user event from the browser. This uses a GET request to"]
        #[doc = " due to browser restriction of POST-ing to a 3rd party domain."]
        #[doc = ""]
        #[doc = " This method is used only by the Recommendations AI JavaScript pixel."]
        #[doc = " Users should not call this method directly."]
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
                "/google.cloud.recommendationengine.v1beta1.UserEventService/CollectUserEvent",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets a list of user events within a time range, with potential filtering."]
        pub async fn list_user_events(
            &mut self,
            request: impl tonic::IntoRequest<super::ListUserEventsRequest>,
        ) -> Result<tonic::Response<super::ListUserEventsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.recommendationengine.v1beta1.UserEventService/ListUserEvents",
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
                "/google.cloud.recommendationengine.v1beta1.UserEventService/PurgeUserEvents",
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
                "/google.cloud.recommendationengine.v1beta1.UserEventService/ImportUserEvents",
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
#[doc = r" Generated server implementations."]
pub mod user_event_service_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with UserEventServiceServer."]
    #[async_trait]
    pub trait UserEventService: Send + Sync + 'static {
        #[doc = " Writes a single user event."]
        async fn write_user_event(
            &self,
            request: tonic::Request<super::WriteUserEventRequest>,
        ) -> Result<tonic::Response<super::UserEvent>, tonic::Status>;
        #[doc = " Writes a single user event from the browser. This uses a GET request to"]
        #[doc = " due to browser restriction of POST-ing to a 3rd party domain."]
        #[doc = ""]
        #[doc = " This method is used only by the Recommendations AI JavaScript pixel."]
        #[doc = " Users should not call this method directly."]
        async fn collect_user_event(
            &self,
            request: tonic::Request<super::CollectUserEventRequest>,
        ) -> Result<tonic::Response<super::super::super::super::api::HttpBody>, tonic::Status>;
        #[doc = " Gets a list of user events within a time range, with potential filtering."]
        async fn list_user_events(
            &self,
            request: tonic::Request<super::ListUserEventsRequest>,
        ) -> Result<tonic::Response<super::ListUserEventsResponse>, tonic::Status>;
        #[doc = " Deletes permanently all user events specified by the filter provided."]
        #[doc = " Depending on the number of events specified by the filter, this operation"]
        #[doc = " could take hours or days to complete. To test a filter, use the list"]
        #[doc = " command first."]
        async fn purge_user_events(
            &self,
            request: tonic::Request<super::PurgeUserEventsRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        >;
        #[doc = " Bulk import of User events. Request processing might be"]
        #[doc = " synchronous. Events that already exist are skipped."]
        #[doc = " Use this method for backfilling historical user events."]
        #[doc = ""]
        #[doc = " Operation.response is of type ImportResponse. Note that it is"]
        #[doc = " possible for a subset of the items to be successfully inserted."]
        #[doc = " Operation.metadata is of type ImportMetadata."]
        async fn import_user_events(
            &self,
            request: tonic::Request<super::ImportUserEventsRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        >;
    }
    #[doc = " Service for ingesting end user actions on the customer website."]
    #[derive(Debug)]
    #[doc(hidden)]
    pub struct UserEventServiceServer<T: UserEventService> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: UserEventService> UserEventServiceServer<T> {
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
    impl<T, B> Service<http::Request<B>> for UserEventServiceServer<T>
    where
        T: UserEventService,
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
                "/google.cloud.recommendationengine.v1beta1.UserEventService/WriteUserEvent" => {
                    #[allow(non_camel_case_types)]
                    struct WriteUserEventSvc<T: UserEventService>(pub Arc<T>);
                    impl<T: UserEventService>
                        tonic::server::UnaryService<super::WriteUserEventRequest>
                        for WriteUserEventSvc<T>
                    {
                        type Response = super::UserEvent;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::WriteUserEventRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.write_user_event(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = WriteUserEventSvc(inner);
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
                "/google.cloud.recommendationengine.v1beta1.UserEventService/CollectUserEvent" => {
                    #[allow(non_camel_case_types)]
                    struct CollectUserEventSvc<T: UserEventService>(pub Arc<T>);
                    impl<T: UserEventService>
                        tonic::server::UnaryService<super::CollectUserEventRequest>
                        for CollectUserEventSvc<T>
                    {
                        type Response = super::super::super::super::api::HttpBody;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CollectUserEventRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.collect_user_event(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = CollectUserEventSvc(inner);
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
                "/google.cloud.recommendationengine.v1beta1.UserEventService/ListUserEvents" => {
                    #[allow(non_camel_case_types)]
                    struct ListUserEventsSvc<T: UserEventService>(pub Arc<T>);
                    impl<T: UserEventService>
                        tonic::server::UnaryService<super::ListUserEventsRequest>
                        for ListUserEventsSvc<T>
                    {
                        type Response = super::ListUserEventsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListUserEventsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.list_user_events(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ListUserEventsSvc(inner);
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
                "/google.cloud.recommendationengine.v1beta1.UserEventService/PurgeUserEvents" => {
                    #[allow(non_camel_case_types)]
                    struct PurgeUserEventsSvc<T: UserEventService>(pub Arc<T>);
                    impl<T: UserEventService>
                        tonic::server::UnaryService<super::PurgeUserEventsRequest>
                        for PurgeUserEventsSvc<T>
                    {
                        type Response = super::super::super::super::longrunning::Operation;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PurgeUserEventsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.purge_user_events(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = PurgeUserEventsSvc(inner);
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
                "/google.cloud.recommendationengine.v1beta1.UserEventService/ImportUserEvents" => {
                    #[allow(non_camel_case_types)]
                    struct ImportUserEventsSvc<T: UserEventService>(pub Arc<T>);
                    impl<T: UserEventService>
                        tonic::server::UnaryService<super::ImportUserEventsRequest>
                        for ImportUserEventsSvc<T>
                    {
                        type Response = super::super::super::super::longrunning::Operation;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ImportUserEventsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.import_user_events(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ImportUserEventsSvc(inner);
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
    impl<T: UserEventService> Clone for UserEventServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: UserEventService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: UserEventService> tonic::transport::NamedService for UserEventServiceServer<T> {
        const NAME: &'static str = "google.cloud.recommendationengine.v1beta1.UserEventService";
    }
}
