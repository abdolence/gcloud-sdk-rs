/// Configures what level the product should be uploaded with regards to
/// how users will be send events and how predictions will be made.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProductLevelConfig {
    /// The level of a [Catalog][google.cloud.retail.v2alpha.Catalog] at which the
    /// [UserEvent][google.cloud.retail.v2alpha.UserEvent]s are uploaded.
    /// Acceptable values are:
    ///   * `primary`
    ///   * `variant`
    ///
    /// If this field is set to an invalid value other than these, an
    /// INVALID_ARGUMENT error is returned.
    ///
    /// If this field is `primary` and
    /// [predict_product_level][google.cloud.retail.v2alpha.ProductLevelConfig.predict_product_level]
    /// is `variant`, an INVALID_ARGUMENT error is returned.
    ///
    /// See
    /// https://cloud.google.com/recommendations-ai/docs/catalog#catalog-levels
    /// for more details.
    #[prost(string, tag = "1")]
    pub event_product_level: std::string::String,
    /// The level of a [Catalog][google.cloud.retail.v2alpha.Catalog] at which the
    /// [PredictionService.Predict][google.cloud.retail.v2alpha.PredictionService.Predict]
    /// is called. Acceptable values are:
    ///   * `primary`
    ///   * `variant`
    ///
    /// If this field is set to an invalid value other than these, an
    /// INVALID_ARGUMENT error is returned.
    ///
    /// If this field is `variant` and
    /// [event_product_level][google.cloud.retail.v2alpha.ProductLevelConfig.event_product_level]
    /// is `primary`, an INVALID_ARGUMENT error is returned.
    ///
    /// See
    /// https://cloud.google.com/recommendations-ai/docs/catalog#catalog-levels
    /// for more details.
    #[prost(string, tag = "2")]
    pub predict_product_level: std::string::String,
}
/// The catalog configuration.
/// Next ID: 5.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Catalog {
    /// Required. Immutable. The fully qualified resource name of the catalog.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Required. Immutable. The catalog display name.
    ///
    /// This field must be a UTF-8 encoded string with a length limit of 128 bytes.
    /// Otherwise, an INVALID_ARGUMENT error is returned.
    #[prost(string, tag = "2")]
    pub display_name: std::string::String,
    /// Required. The product level configuration.
    #[prost(message, optional, tag = "4")]
    pub product_level_config: ::std::option::Option<ProductLevelConfig>,
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
    pub parent: std::string::String,
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
    pub page_token: std::string::String,
}
/// Response for
/// [CatalogService.ListCatalogs][google.cloud.retail.v2alpha.CatalogService.ListCatalogs]
/// method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCatalogsResponse {
    /// All the customer's [Catalog][google.cloud.retail.v2alpha.Catalog]s.
    #[prost(message, repeated, tag = "1")]
    pub catalogs: ::std::vec::Vec<Catalog>,
    /// A token that can be sent as
    /// [ListCatalogsRequest.page_token][google.cloud.retail.v2alpha.ListCatalogsRequest.page_token]
    /// to retrieve the next page. If this field is omitted, there are no
    /// subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
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
    pub catalog: ::std::option::Option<Catalog>,
    /// Indicates which fields in the provided
    /// [Catalog][google.cloud.retail.v2alpha.Catalog] to update. If not set, will
    /// only update the
    /// [Catalog.product_level_config][google.cloud.retail.v2alpha.Catalog.product_level_config]
    /// field, which is also the only currently supported field to update.
    ///
    /// If an unsupported or unknown field is provided, an INVALID_ARGUMENT error
    /// is returned.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
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
/// [Product][google.cloud.retail.v2alpha.Product]].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomAttribute {
    /// The textual values of this custom attribute. For example, `["yellow",
    /// "green"]` when the key is "color".
    ///
    /// At most 400 values are allowed. Empty values are not allowed. Each value
    /// must be a UTF-8 encoded string with a length limit of 256 bytes. Otherwise,
    /// an INVALID_ARGUMENT error is returned.
    ///
    /// Exactly one of [text][google.cloud.retail.v2alpha.CustomAttribute.text] or
    /// [numbers][google.cloud.retail.v2alpha.CustomAttribute.numbers] should be
    /// set. Otherwise, a FAILED_PRECONDITION error is returned.
    #[prost(string, repeated, tag = "1")]
    pub text: ::std::vec::Vec<std::string::String>,
    /// The numerical values of this custom attribute. For example, `[2.3, 15.4]`
    /// when the key is "lengths_cm".
    ///
    /// At most 400 values are allowed.Otherwise, an INVALID_ARGUMENT error is
    /// returned.
    ///
    /// Exactly one of [text][google.cloud.retail.v2alpha.CustomAttribute.text] or
    /// [numbers][google.cloud.retail.v2alpha.CustomAttribute.numbers] should be
    /// set. Otherwise, a FAILED_PRECONDITION error is returned.
    #[prost(double, repeated, tag = "2")]
    pub numbers: ::std::vec::Vec<f64>,
}
/// [Product][google.cloud.retail.v2alpha.Product] thumbnail/detail image.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Image {
    /// Required. URI of the image.
    ///
    /// This field must be a valid UTF-8 encoded URI with a length limit of 5 KiB.
    /// Otherwise, an INVALID_ARGUMENT error is returned.
    ///
    /// Google Merchant Center property
    /// [image_link](https://support.google.com/merchants/answer/6324350).
    /// Schema.org property [Product.image](http://schema.org/image).
    #[prost(string, tag = "1")]
    pub uri: std::string::String,
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
/// The price information of a [Product][google.cloud.retail.v2alpha.Product].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PriceInfo {
    /// The 3-letter currency code defined in [ISO
    /// 4217][https://www.iso.org/iso-4217-currency-codes.html].
    ///
    /// If this field is an unrecognizable currency code, an INVALID_ARGUMENT
    /// error is returned.
    #[prost(string, tag = "1")]
    pub currency_code: std::string::String,
    /// Price of the product.
    ///
    /// Google Merchant Center property
    /// [price](https://support.google.com/merchants/answer/6324371).
    /// Schema.org property
    /// [Offer.priceSpecification](https://schema.org/priceSpecification).
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
    /// [cost_of_goods_sold](https://support.google.com/merchants/answer/9017895)
    #[prost(float, tag = "4")]
    pub cost: f32,
}
/// Information of an end user.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserInfo {
    /// Highly recommended for logged-in users. Unique identifier for logged-in
    /// user, such as a user name.
    ///
    /// The field must be a UTF-8 encoded string with a length limit of 128 bytes.
    /// Otherwise, an INVALID_ARGUMENT error is returned.
    #[prost(string, tag = "1")]
    pub user_id: std::string::String,
    /// The end user's IP address. This field is used to extract location
    /// information for personalization.
    ///
    /// This field must be either an IPv4 address (e.g. "104.133.9.80") or an IPv6
    /// address (e.g. "2001:0db8:85a3:0000:0000:8a2e:0370:7334"). Otherwise, an
    /// INVALID_ARGUMENT error is returned.
    ///
    /// This should not be set when using the JavaScript tag in
    /// [UserEventService.CollectUserEvent][google.cloud.retail.v2alpha.UserEventService.CollectUserEvent]
    /// or if
    /// [direct_user_request][google.cloud.retail.v2alpha.UserInfo.direct_user_request]
    /// is set. Otherwise, a FAILED_PRECONDITION error is returned.
    #[prost(string, tag = "2")]
    pub ip_address: std::string::String,
    /// User agent as included in the HTTP header.
    ///
    /// The field must be a UTF-8 encoded string with a length limit of 1 KiB.
    /// Otherwise, an INVALID_ARGUMENT error is returned.
    ///
    /// This should not be set when using the client side event reporting with
    /// GTM or JavaScript tag in
    /// [UserEventService.CollectUserEvent][google.cloud.retail.v2alpha.UserEventService.CollectUserEvent]
    /// or if
    /// [direct_user_request][google.cloud.retail.v2alpha.UserInfo.direct_user_request]
    /// is set. Otherwise, a FAILED_PRECONDITION error is returned.
    #[prost(string, tag = "3")]
    pub user_agent: std::string::String,
    /// True if the request is made directly from the end user, in which case the
    /// [ip_address][google.cloud.retail.v2alpha.UserInfo.ip_address] and
    /// [user_agent][google.cloud.retail.v2alpha.UserInfo.user_agent] can be
    /// populated from the HTTP request. This flag should be set only if the API
    /// request is made directly from the end user such as a mobile app (and not if
    /// a gateway or a server is processing and pushing the user events).
    ///
    /// This should not be set when using the JavaScript tag in
    /// [UserEventService.CollectUserEvent][google.cloud.retail.v2alpha.UserEventService.CollectUserEvent].
    /// Otherwise, a FAILED_PRECONDITION error is returned.
    #[prost(bool, tag = "4")]
    pub direct_user_request: bool,
}
/// The output configuration setting.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OutputConfig {
    /// The configuration of destination for holding output data.
    #[prost(oneof = "output_config::Destination", tags = "1, 2")]
    pub destination: ::std::option::Option<output_config::Destination>,
}
pub mod output_config {
    /// The Google Cloud Storage output destination configuration.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GcsDestination {
        /// Required. The output uri prefix for saving output data to json files.
        /// Some mapping examples are as follows:
        /// output_uri_prefix         sample output(assuming the object is foo.json)
        /// ========================  =============================================
        /// gs://bucket/              gs://bucket/foo.json
        /// gs://bucket/folder/       gs://bucket/folder/foo.json
        /// gs://bucket/folder/item_  gs://bucket/folder/item_foo.json
        #[prost(string, tag = "1")]
        pub output_uri_prefix: std::string::String,
    }
    /// The BigQuery output destination configuration.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct BigQueryDestination {
        /// The ID of a BigQuery Dataset.
        #[prost(string, tag = "1")]
        pub dataset_id: std::string::String,
    }
    /// The configuration of destination for holding output data.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Destination {
        /// The Google Cloud Storage location where the output is to be written to.
        #[prost(message, tag = "1")]
        GcsDestination(GcsDestination),
        /// The BigQuery location where the output is to be written to.
        #[prost(message, tag = "2")]
        BigqueryDestination(BigQueryDestination),
    }
}
/// Configuration of destination for Export related errors.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportErrorsConfig {
    /// Required. Errors destination.
    #[prost(oneof = "export_errors_config::Destination", tags = "1")]
    pub destination: ::std::option::Option<export_errors_config::Destination>,
}
pub mod export_errors_config {
    /// Required. Errors destination.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Destination {
        /// Google Cloud Storage path for import errors. This must be an empty,
        /// existing Cloud Storage bucket. Export errors will be written to a file in
        /// this bucket, one per line, as a JSON-encoded
        /// `google.rpc.Status` message.
        #[prost(string, tag = "1")]
        GcsPrefix(std::string::String),
    }
}
/// Request message for ExportProducts method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportProductsRequest {
    /// Required.
    /// "projects/1234/locations/global/catalogs/default_catalog/branches/default_branch"
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Unique identifier provided by client, within the ancestor
    /// dataset scope. Ensures idempotency for expensive long running operations.
    /// Server-generated if unspecified. Up to 128 characters long. This is
    /// returned as google.longrunning.Operation.name in the response.
    #[prost(string, tag = "2")]
    pub request_id: std::string::String,
    /// Required. The desired output location of the data.
    #[prost(message, optional, tag = "3")]
    pub output_config: ::std::option::Option<OutputConfig>,
    /// Filtering expression to specify restrictions over
    /// returned products. This is a sequence of terms, where each term applies
    /// some kind of a restriction to the returned products. Use this expression to
    /// restrict results to a specific time range, tag, stock state or filter
    /// products by product type.
    ///    eg: lastModifiedTime > "2012-04-23T18:25:43.511Z"
    ///    lastModifiedTime<"2012-04-23T18:25:43.511Z" productType=primary
    ///
    ///   We expect only 4 types of fields:
    ///
    ///    * lastModifiedTime: this can be specified a maximum of 2 times, once
    ///    with a
    ///      less than operator and once with a greater than operator. The
    ///      lastModifiedTime restrict should result in one contiguous valid last
    ///      modified time range.
    ///
    ///    * productType: supported values are 'primary' and 'variant'. Boolean
    ///    operators `OR` and `NOT` are supported if the
    ///      expression is enclosed in parentheses, and must be separated from the
    ///      productType values by a space.
    ///
    ///    * availability: supported values are IN_STOCK, OUT_OF_STOCK,
    ///    PREORDER and BACKORDER. Boolean operators `OR` and `NOT` are
    ///    supported if the
    ///      expression is enclosed in parentheses, and must be separated from the
    ///      availability values by a space.
    ///
    ///    * Tag expressions. Restricts output to products that match all of the
    ///      specified tags. Boolean operators `OR` and `NOT` are supported if the
    ///      expression is enclosed in parentheses, and must be separated from the
    ///      tag values by a space. `-"tagA"` is also supported and is equivalent
    ///      to `NOT "tagA"`. Tag values must be double quoted UTF-8 encoded
    ///      strings with a size limit of 1 KiB.
    ///
    ///   Some examples of valid filters expressions:
    ///
    ///   * Example 1: lastModifiedTime > "2012-04-23T18:25:43.511Z"
    ///             lastModifiedTime < "2012-04-23T18:30:43.511Z"
    ///   * Example 2: lastModifiedTime > "2012-04-23T18:25:43.511Z"
    ///             productType = "variant"
    ///   * Example 3: tag=("Red" OR "Blue") tag="New-Arrival"
    ///             tag=(NOT "promotional")
    ///             productType = "primary" lastModifiedTime <
    ///             "2018-04-23T18:30:43.511Z"
    ///   * Example 4: lastModifiedTime > "2012-04-23T18:25:43.511Z"
    ///   * Example 5: availability = (IN_STOCK OR BACKORDER)
    #[prost(string, tag = "4")]
    pub filter: std::string::String,
}
/// Request message for ExportUserEvents method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportUserEventsRequest {
    /// Required. "projects/1234/locations/global/catalogs/default_catalog"
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Unique identifier provided by client, within the ancestor
    /// dataset scope. Ensures idempotency for expensive long running operations.
    /// Server-generated if unspecified. Up to 128 characters long. This is
    /// returned as google.longrunning.Operation.name in the response.
    #[prost(string, tag = "2")]
    pub request_id: std::string::String,
    /// Required. The desired output location of the data.
    #[prost(message, optional, tag = "3")]
    pub output_config: ::std::option::Option<OutputConfig>,
    /// Filtering expression to specify restrictions over
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
    ///    * eventType: Boolean operators `OR` and `NOT` are supported if the
    ///      expression is enclosed in parentheses, and must be separated from the
    ///      tag values by a space.
    ///
    ///    * eventsMissingCatalogItems: specifying this will restrict results
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
    ///             eventType = (NOT search) eventTime < "2018-04-23T18:30:43.511Z"
    ///   * Example 4: eventTime > "2012-04-23T18:25:43.511Z"
    ///   * Example 5: eventType = (search OR impression)
    ///   * Example 6: eventsMissingCatalogItems
    #[prost(string, tag = "4")]
    pub filter: std::string::String,
}
/// Metadata related to the progress of the Export operation. This will be
/// returned by the google.longrunning.Operation.metadata field.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportMetadata {
    /// Name of the operation.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Id of the request / operation. This is parroting back the requestId that
    /// was passed in the request.
    #[prost(string, tag = "2")]
    pub request_id: std::string::String,
    /// Operation create time.
    #[prost(message, optional, tag = "3")]
    pub create_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Operation last update time. If the operation is done, this is also the
    /// finish time.
    #[prost(message, optional, tag = "4")]
    pub update_time: ::std::option::Option<::prost_types::Timestamp>,
}
/// Response of the ExportProductsRequest. If the long running
/// operation is done, then this message is returned by the
/// google.longrunning.Operations.response field if the operation was successful.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportProductsResponse {
    /// A sample of errors encountered while processing the request.
    #[prost(message, repeated, tag = "1")]
    pub error_samples: ::std::vec::Vec<super::super::super::rpc::Status>,
    /// Echoes the destination for the complete errors in the request if set.
    #[prost(message, optional, tag = "2")]
    pub errors_config: ::std::option::Option<ExportErrorsConfig>,
}
/// Response of the ExportUserEventsRequest. If the long running
/// operation was successful, then this message is returned by the
/// google.longrunning.Operations.response field if the operation was successful.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportUserEventsResponse {
    /// A sample of errors encountered while processing the request.
    #[prost(message, repeated, tag = "1")]
    pub error_samples: ::std::vec::Vec<super::super::super::rpc::Status>,
    /// Echoes the destination for the complete errors if this field was set in
    /// the request.
    #[prost(message, optional, tag = "2")]
    pub errors_config: ::std::option::Option<ExportErrorsConfig>,
}
/// Product captures all metadata information of items to be recommended or
/// searched.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Product {
    /// Immutable. Full resource name of the product, such as
    ///
    /// "projects/*/locations/global/catalogs/default_catalog/branches/default_branch/products/product_id".
    ///
    /// The branch ID must be "default_branch".
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Immutable. [Product][google.cloud.retail.v2alpha.Product] identifier, which
    /// is the final component of [name][google.cloud.retail.v2alpha.Product.name].
    /// For example, this field is "id_1", if
    /// [name][google.cloud.retail.v2alpha.Product.name] is
    ///
    /// "projects/*/locations/global/catalogs/default_catalog/branches/default_branch/products/id_1".
    ///
    /// This field must be a UTF-8 encoded string with a length limit of 128 bytes.
    /// Otherwise, an INVALID_ARGUMENT error is returned.
    ///
    /// Google Merchant Center property
    /// [id](https://support.google.com/merchants/answer/6324405).
    /// schema.org Property [Product.sku](https://schema.org/sku).
    #[prost(string, tag = "2")]
    pub id: std::string::String,
    /// Variant group identifier. Must be an
    /// [id][google.cloud.retail.v2alpha.Product.id], with the same parent branch
    /// with this product. Otherwise, an error is thrown.
    ///
    /// The primary product may be empty during the creation, but cannot be updated
    /// from a non-empty string to an empty one. Otherwise an INVALID_ARGUMENT
    /// error is returned.
    ///
    /// Should only be set for
    /// [Type.VARIANT][google.cloud.retail.v2alpha.Product.Type.VARIANT]
    /// [Product][google.cloud.retail.v2alpha.Product]s. A maximum of 1000 products
    /// are allowed to share the same
    /// [Type.PRIMARY][google.cloud.retail.v2alpha.Product.Type.PRIMARY]
    /// [Product][google.cloud.retail.v2alpha.Product]. Otherwise, an
    /// INVALID_ARGUMENT error is returned.
    ///
    /// Google Merchant Center Property
    /// [item_group_id](https://support.google.com/merchants/answer/6324507).
    /// schema.org Property
    /// [Product.inProductGroupWithID](https://schema.org/inProductGroupWithID).
    ///
    /// This field must be enabled before it can be used. [Learn
    /// more](/recommendations-ai/docs/catalog#item-group-id).
    #[prost(string, tag = "4")]
    pub primary_product_id: std::string::String,
    /// Required. Product categories. This field is repeated for supporting one
    /// product belonging to several parallel categories. Each value is either the
    /// full path of the category, or the [category
    ///
    /// ID](https:
    /// //www.google.com/basepages/producttype/taxonomy-with-ids.en-US.txt).
    /// Strongly recommended using the full path for better search / recommendation
    /// quality.
    ///
    /// To represent full path of category, use '>' sign to separate different
    /// hierarchies. If '>' is part of the category name, you should escape it with
    /// '\x3E'.
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
    /// At most 250 values are allowed per
    /// [Product][google.cloud.retail.v2alpha.Product]. Empty values are not
    /// allowed. Each value must be a UTF-8 encoded string with a length limit of 5
    /// KiB. Otherwise, an INVALID_ARGUMENT error is returned.
    ///
    /// Google Merchant Center property
    ///
    /// [google_product_category](https:
    /// //support.google.com/merchants/answer/6324436).
    /// Schema.org property [Product.category] (https://schema.org/category).
    #[prost(string, repeated, tag = "7")]
    pub categories: ::std::vec::Vec<std::string::String>,
    /// Required. Product title.
    ///
    /// This field must be a UTF-8 encoded string with a length limit of 128 bytes.
    /// Otherwise, an INVALID_ARGUMENT error is returned.
    ///
    /// Google Merchant Center property
    /// [title](https://support.google.com/merchants/answer/6324415). Schema.org
    /// property [Product.name](https://schema.org/name).
    #[prost(string, tag = "8")]
    pub title: std::string::String,
    /// Product description.
    ///
    /// This field must be a UTF-8 encoded string with a length limit of 5 KiB.
    /// Otherwise, an INVALID_ARGUMENT error is returned.
    ///
    /// Google Merchant Center property
    /// [description](https://support.google.com/merchants/answer/6324468).
    /// schema.org property [Product.description](https://schema.org/description).
    #[prost(string, tag = "10")]
    pub description: std::string::String,
    /// Language of the title/description and other string attributes. Use language
    /// tags defined by [BCP 47][https://www.rfc-editor.org/rfc/bcp/bcp47.txt].
    ///
    /// The model automatically detects the text language. The
    /// [Product][google.cloud.retail.v2alpha.Product] can include text in
    /// different languages, but duplicating
    /// [Product][google.cloud.retail.v2alpha.Product]s to provide text in multiple
    /// languages can result in degraded model performance.
    ///
    /// Currently, recommendation supports all language codes, while the only
    /// supported language code for search is "en-US".
    #[prost(string, tag = "11")]
    pub language_code: std::string::String,
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
    /// For example: { "vendor": {"text": ["vendor123", "vendor456"]},
    /// "lengths_cm": {"numbers":[2.3, 15.4]}, "heights_cm": {"numbers":[8.1, 6.4]}
    /// }.
    ///
    /// A maximum of 150 attributes are allowed. Otherwise, an INVALID_ARGUMENT
    /// error is returned.
    #[prost(map = "string, message", tag = "12")]
    pub attributes: ::std::collections::HashMap<std::string::String, CustomAttribute>,
    /// Custom tags associated with the product.
    ///
    /// At most 250 values are allowed per
    /// [Product][google.cloud.retail.v2alpha.Product]. This value must be a UTF-8
    /// encoded string with a length limit of 1 KiB. Otherwise, an INVALID_ARGUMENT
    /// error is returned.
    ///
    /// This tag can be used for filtering recommendation results by passing the
    /// tag as part of the
    /// [PredictRequest.filter][google.cloud.retail.v2alpha.PredictRequest.filter].
    ///
    /// Google Merchant Center property
    /// [custom_label_0–4](https://support.google.com/merchants/answer/6324473).
    #[prost(string, repeated, tag = "13")]
    pub tags: ::std::vec::Vec<std::string::String>,
    /// Product price and cost information.
    ///
    /// Google Merchant Center property
    /// [price](https://support.google.com/merchants/answer/6324371).
    #[prost(message, optional, tag = "14")]
    pub price_info: ::std::option::Option<PriceInfo>,
    /// The timestamp when this [Product][google.cloud.retail.v2alpha.Product]
    /// becomes available recommendation and search.
    #[prost(message, optional, tag = "18")]
    pub available_time: ::std::option::Option<::prost_types::Timestamp>,
    /// The online availability of the
    /// [Product][google.cloud.retail.v2alpha.Product], which is parallel to and
    /// independent of
    /// [fulfillment_info][google.cloud.retail.v2alpha.Product.fulfillment_info].
    /// Default is
    /// [Availability.IN_STOCK][google.cloud.retail.v2alpha.Product.Availability.IN_STOCK].
    ///
    /// Google Merchant Center Property
    /// [availability](https://support.google.com/merchants/answer/6324448).
    /// schema.org Property [Offer.availability](https://schema.org/availability).
    #[prost(enumeration = "product::Availability", tag = "19")]
    pub availability: i32,
    /// The available quantity of the item.
    #[prost(message, optional, tag = "20")]
    pub available_quantity: ::std::option::Option<i32>,
    /// Canonical URL directly linking to the product detail page.
    ///
    /// This field must be a UTF-8 encoded string with a length limit of 5 KiB.
    /// Otherwise, an INVALID_ARGUMENT error is returned.
    ///
    /// Google Merchant Center property
    /// [link](https://support.google.com/merchants/answer/6324416).
    /// Schema.org property [Offer.url](https://schema.org/url).
    #[prost(string, tag = "22")]
    pub uri: std::string::String,
    /// Product images for the product.
    ///
    /// Google Merchant Center property
    /// [image_link](https://support.google.com/merchants/answer/6324350).
    /// Schema.org property [Product.image](https://schema.org/image).
    #[prost(message, repeated, tag = "23")]
    pub images: ::std::vec::Vec<Image>,
    /// The material of the product. For example, "leather", "wooden".
    ///
    /// A maximum of 5 values are allowed. Each value must be a UTF-8 encoded
    /// string with a length limit of 128 bytes. Otherwise, an INVALID_ARGUMENT
    /// error is returned.
    ///
    /// Google Merchant Center property
    /// [material](https://support.google.com/merchants/answer/6324410). Schema.org
    /// property [Product.material](https://schema.org/material).
    #[prost(string, repeated, tag = "27")]
    pub materials: ::std::vec::Vec<std::string::String>,
    /// Indicates which fields in the
    /// [variants][google.cloud.retail.v2alpha.Product.variants] are retrievable in
    /// [Search][]. If not set or empty, the following fields are returned:
    ///
    /// * [name][google.cloud.retail.v2alpha.Product.name]
    /// * [availability][google.cloud.retail.v2alpha.Product.availability]
    /// * [color_info][google.cloud.retail.v2alpha.Product.color_info]
    ///
    /// Supported fields:
    ///
    /// * [name][google.cloud.retail.v2alpha.Product.name]
    /// * [availability][google.cloud.retail.v2alpha.Product.availability]
    /// * [color_info][google.cloud.retail.v2alpha.Product.color_info]
    /// * [gtin][google.cloud.retail.v2alpha.Product.gtin]
    /// * [price_info][google.cloud.retail.v2alpha.Product.price_info]
    /// * [sizes][google.cloud.retail.v2alpha.Product.sizes]
    /// * [materials][google.cloud.retail.v2alpha.Product.materials]
    /// * [patterns][google.cloud.retail.v2alpha.Product.patterns]
    /// * [conditions][google.cloud.retail.v2alpha.Product.conditions]
    /// * [images][google.cloud.retail.v2alpha.Product.images]
    /// * [attributes][google.cloud.retail.v2alpha.Product.attributes]
    ///
    /// To mark custom attributes as retrievable, include paths of the form
    /// "attributes.key" where "key" is the key of a custom attribute, as
    /// specified in [attributes][google.cloud.retail.v2alpha.Product.attributes].
    ///
    /// Maximum number of paths is 10. Otherwise, an INVALID_ARGUMENT error is
    /// returned.
    #[prost(message, optional, tag = "30")]
    pub stocking_unit_retrievable_fields: ::std::option::Option<::prost_types::FieldMask>,
}
pub mod product {
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
}
/// UserEvent captures all metadata information recommendation engine needs to
/// know about how end users interact with customers' website.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserEvent {
    /// Required. User event type. Allowed values are:
    ///
    /// * `add-to-cart` Products being added to cart.
    /// * `category-page-view` Special pages such as sale or promotion pages
    ///   viewed.
    /// * `detail-page-view` Products detail page viewed.
    /// * `home-page-view` Homepage viewed.
    /// * `purchase-complete` User finishing a purchase.
    /// * `search`
    /// * `shopping-cart-page-view` User viewing a shopping cart.
    #[prost(string, tag = "1")]
    pub event_type: std::string::String,
    /// Required. A unique identifier for tracking visitors. For example, this
    /// could be implemented with a http cookie, which should be able to uniquely
    /// identify a visitor on a single device. This unique identifier should not
    /// change if the visitor log in/out of the website.
    ///
    /// The field must be a UTF-8 encoded string with a length limit of 128 bytes.
    /// Otherwise, an INVALID_ARGUMENT error is returned.
    #[prost(string, tag = "2")]
    pub visitor_id: std::string::String,
    /// Only required for
    /// [UserEventService.ImportUserEvents][google.cloud.retail.v2alpha.UserEventService.ImportUserEvents]
    /// method. Timestamp of when the user event happened.
    #[prost(message, optional, tag = "3")]
    pub event_time: ::std::option::Option<::prost_types::Timestamp>,
    /// A list of identifiers for the independent experiment groups
    /// this user event belongs to. This is used to distinguish between user events
    /// associated with different experiment setups (e.g. using Recommendations AI,
    /// using different recommendation models).
    #[prost(string, repeated, tag = "4")]
    pub experiment_ids: ::std::vec::Vec<std::string::String>,
    /// Highly recommended for user events that are the result of
    /// [PredictionService.Predict][google.cloud.retail.v2alpha.PredictionService.Predict].
    /// This field enables accurate attribution of recommendation model
    /// performance.
    ///
    /// The value must be a valid
    /// [PredictResponse.attribute_token][] for user events that are the result of
    /// [PredictionService.Predict][google.cloud.retail.v2alpha.PredictionService.Predict].
    ///
    /// This token enables us to accurately attribute page view or purchase back to
    /// the event and the particular predict response containing this
    /// clicked/purchased product. If user clicks on product K in the
    /// recommendation results, pass [PredictResponse.attribute_token][] as a url
    /// parameter to product K's page. When recording events on product K's page,
    /// log the [PredictResponse.attribute_token][] to this field.
    #[prost(string, tag = "5")]
    pub attribution_token: std::string::String,
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
    /// pagination/filtering/ordering even for the same query, a new SEARCH event
    /// with different
    /// [product_details][google.cloud.retail.v2alpha.UserEvent.product_details] is
    /// desired. The end user may have not finished broswing the whole page yet.
    #[prost(message, repeated, tag = "6")]
    pub product_details: ::std::vec::Vec<ProductDetail>,
    /// Extra user event features to include in the recommendation model.
    ///
    /// For product recommendation, an example of extra user information is
    /// traffic_channel, i.e. how user arrives at the site. Users can arrive
    /// at the site by coming to the site directly, or coming through Google
    /// search, and etc.
    #[prost(map = "string, message", tag = "7")]
    pub attributes: ::std::collections::HashMap<std::string::String, CustomAttribute>,
    /// The id or name of the associated shopping cart. This id is used
    /// to associate multiple items added or present in the cart before purchase.
    ///
    /// This can only be set for `add-to-cart`, `remove-from-cart`,
    /// `checkout-start`, `purchase-complete`, or `shopping-cart-page-view` events.
    #[prost(string, tag = "8")]
    pub cart_id: std::string::String,
    /// A transaction represents the entire purchase transaction.
    ///
    /// Required for `purchase-complete` events. Optional for `checkout-start`
    /// events. Other event types should not set this field. Otherwise, an
    /// INVALID_ARGUMENT error is returned.
    #[prost(message, optional, tag = "9")]
    pub purchase_transaction: ::std::option::Option<PurchaseTransaction>,
    /// The user's search query.
    ///
    /// The value must be a UTF-8 encoded string with a length limit of 5 KiB.
    /// Otherwise, an INVALID_ARGUMENT error is returned.
    ///
    /// Required for `search` events. Other event types should not set this field.
    /// Otherwise, an INVALID_ARGUMENT error is returned.
    #[prost(string, tag = "10")]
    pub search_query: std::string::String,
    /// The categories associated with a category page.
    ///
    /// To represent full path of category, use '>' sign to separate different
    /// hierarchies. If '>' is part of the category name, you should escape it with
    /// '\x3E'.
    ///
    /// Category pages include special pages such as sales or promotions. For
    /// instance, a special sale page may have the category hierarchy:
    /// "pageCategories" : ["Sales > 2017 Black Friday Deals"].
    ///
    /// Required for `category-page-view` events. Other event types should not set
    /// this field. Otherwise, an INVALID_ARGUMENT error is returned.
    #[prost(string, repeated, tag = "11")]
    pub page_categories: ::std::vec::Vec<std::string::String>,
    /// User information.
    #[prost(message, optional, tag = "12")]
    pub user_info: ::std::option::Option<UserInfo>,
    /// Complete url (window.location.href) of the user's current page.
    /// When using the client side event reporting with JavaScript pixel and Google
    /// Tag Manager, this value is filled in automatically. Maximum length 5KB.
    #[prost(string, tag = "13")]
    pub uri: std::string::String,
    /// The referrer url of the current page. When using
    /// the client side event reporting with JavaScript pixel and Google Tag
    /// Manager, this value is filled in automatically.
    #[prost(string, tag = "14")]
    pub referrer_uri: std::string::String,
    /// A unique id of a web page view.
    /// This should be kept the same for all user events triggered from the same
    /// pageview. For example, an item detail page view could trigger multiple
    /// events as the user is browsing the page.
    /// The `pageViewId` property should be kept the same for all these events so
    /// that they can be grouped together properly. This `pageViewId` will be
    /// automatically generated if using the client side event reporting with
    /// JavaScript pixel and Google Tag Manager.
    #[prost(string, tag = "15")]
    pub page_view_id: std::string::String,
    /// User event source.
    /// Acceptable values are:
    ///
    /// * `client_tag` if the event is ingested via a JavaScript tag or
    ///   Recommendations AI Tag through automl datalayer or JS Macros.
    /// * `client_tag_ecommerce` if the event is ingested via Recommendations AI
    /// Tag through
    ///   Enhanced Ecommerce datalayer.
    /// * 'batch_upload' if the event is ingested via ImportUserEvents API.
    ///
    /// This field should *not* be set when using client side event reporting with
    /// JavaScript pixel and Google Tag Manager or the Recommendations AI Tag.
    #[prost(string, tag = "16")]
    pub event_source: std::string::String,
}
/// Detailed product information associated with a user event.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProductDetail {
    /// Required. [Product][google.cloud.retail.v2alpha.Product] information.
    /// Only [Product][id] field must to be set.
    #[prost(message, optional, tag = "1")]
    pub product: ::std::option::Option<Product>,
    /// Quantity of the product associated with the user event. For
    /// example, this field will be 2 if two products are added to the shopping
    /// cart for `purchase-complete` event. Required for `add-to-cart` and
    /// `purchase-complete` event types.
    #[prost(message, optional, tag = "2")]
    pub quantity: ::std::option::Option<i32>,
}
/// A transaction represents the entire purchase transaction.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PurchaseTransaction {
    /// The transaction ID with a length limit of 128 bytes.
    #[prost(string, tag = "1")]
    pub id: std::string::String,
    /// Required. Total revenue or grand total associated with the transaction.
    /// This value include shipping, tax, or other adjustments to total revenue
    /// that you want to include as part of your revenue calculations. This field
    /// is not required if the event type is `refund`.
    #[prost(float, tag = "2")]
    pub revenue: f32,
    /// All the taxes associated with the transaction.
    #[prost(float, tag = "3")]
    pub tax: f32,
    /// All the costs associated with the product. These can be
    /// manufacturing costs, shipping expenses not borne by the end user, or any
    /// other costs.
    ///
    /// Total product cost such that
    ///   profit = revenue - tax + [Product][pricing][cost]
    /// If product_cost is not set, then
    ///   profit = revenue - tax - [Product][pricing][cost].
    ///
    /// If [Product][pricing][cost] is not specified for one of the products,
    /// [Product][pricing][cost] based profit *cannot* be calculated for this
    /// Transaction.
    #[prost(float, tag = "4")]
    pub cost: f32,
    /// Required. Currency code. Use three-character ISO-4217 code. This field
    /// is not required if the event type is `refund`.
    #[prost(string, tag = "5")]
    pub currency_code: std::string::String,
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
    /// [Importing product information](/recommendations-ai/docs/upload-catalog)
    /// for the expected file format and setup instructions.
    #[prost(string, repeated, tag = "1")]
    pub input_uris: ::std::vec::Vec<std::string::String>,
    /// The schema to use when parsing the data from the source.
    ///
    /// Supported values for product imports:
    ///
    ///  1: "product" using
    ///    https://cloud.google.com/recommendations-ai/docs/upload-catalog#json
    ///    (Default for products.import)
    ///
    ///  2: "product_merchant_center" using
    ///    https://cloud.google.com/recommendations-ai/docs/upload-catalog#mc
    ///
    /// Supported values for user events imports:
    ///
    ///  1: "user_event" using
    ///  https://cloud.google.com/recommendations-ai/docs/manage-user-events#import
    ///  (Default for userEvents.import)
    ///
    ///  2. "user_event_ga360" using
    ///  https://support.google.com/analytics/answer/3437719?hl=en
    #[prost(string, tag = "2")]
    pub data_schema: std::string::String,
}
/// BigQuery source import data from.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BigQuerySource {
    /// The project id (can be project # or id) that the BigQuery source is in. If
    /// not specified, inherits the project id from the parent request.
    #[prost(string, tag = "5")]
    pub project_id: std::string::String,
    /// Required. The BigQuery data set to copy the data from.
    #[prost(string, tag = "1")]
    pub dataset_id: std::string::String,
    /// Required. The BigQuery table to copy the data from.
    #[prost(string, tag = "2")]
    pub table_id: std::string::String,
    /// Intermediate Cloud Storage directory used for the import. Can be specified
    /// if one wants to have the BigQuery export to a specific Cloud Storage
    /// directory.
    #[prost(string, tag = "3")]
    pub gcs_staging_dir: std::string::String,
    /// The schema to use when parsing the data from the source.
    ///
    /// Supported values for catalog imports:
    ///
    ///  1: "product" using
    ///    https://cloud.google.com/recommendations-ai/docs/upload-catalog#json
    ///    (Default for products.import)
    ///
    ///  2: "product_merchant_center" using
    ///    https://cloud.google.com/recommendations-ai/docs/upload-catalog#mc
    ///
    /// Supported values for user event imports:
    ///
    ///  1: "user_event" using
    ///  https://cloud.google.com/recommendations-ai/docs/manage-user-events#import
    ///  (Default for userEvents.import)
    ///
    ///  2. "user_event_ga360" using
    ///  https://support.google.com/analytics/answer/3437719?hl=en
    #[prost(string, tag = "4")]
    pub data_schema: std::string::String,
}
/// The inline source for the input config for ImportProducts method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProductInlineSource {
    /// A list of products to update/create. Recommended max of 10k items.
    #[prost(message, repeated, tag = "1")]
    pub products: ::std::vec::Vec<Product>,
}
/// The inline source for the input config for ImportUserEvents method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserEventInlineSource {
    /// A list of user events to import. Recommended max of 10k items.
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
pub struct ImportProductsRequest {
    /// Required.
    /// "projects/1234/locations/global/catalogs/default_catalog/branches/default_branch"
    ///
    /// If no updateMask is specified, requires products.create permission.
    /// If updateMask is specified, requires products.update permission.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Required. The desired input location of the data.
    #[prost(message, optional, tag = "2")]
    pub input_config: ::std::option::Option<ProductInputConfig>,
    /// The desired location of errors incurred during the Import.
    #[prost(message, optional, tag = "3")]
    pub errors_config: ::std::option::Option<ImportErrorsConfig>,
    /// Indicates which fields in the provided imported 'products' to update. If
    /// not set, will by default update all fields.
    #[prost(message, optional, tag = "4")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
}
/// Request message for the ImportUserEvents request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportUserEventsRequest {
    /// Required. "projects/1234/locations/global/catalogs/default_catalog"
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Required. The desired input location of the data.
    #[prost(message, optional, tag = "2")]
    pub input_config: ::std::option::Option<UserEventInputConfig>,
    /// The desired location of errors incurred during the Import.
    #[prost(message, optional, tag = "3")]
    pub errors_config: ::std::option::Option<ImportErrorsConfig>,
}
/// The input config source for products.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProductInputConfig {
    /// Required. The source of the input.
    #[prost(oneof = "product_input_config::Source", tags = "1, 2, 3")]
    pub source: ::std::option::Option<product_input_config::Source>,
}
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
    pub source: ::std::option::Option<user_event_input_config::Source>,
}
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
    /// Name of the operation.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Operation create time.
    #[prost(message, optional, tag = "2")]
    pub create_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Operation last update time. If the operation is done, this is also the
    /// finish time.
    #[prost(message, optional, tag = "3")]
    pub update_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Count of entries that were processed successfully.
    #[prost(int64, tag = "4")]
    pub success_count: i64,
    /// Count of entries that encountered errors while processing.
    #[prost(int64, tag = "5")]
    pub failure_count: i64,
}
/// Response of the
/// [ImportProductsRequest][google.cloud.retail.v2alpha.ImportProductsRequest].
/// If the long running operation is done, then this message is returned by the
/// google.longrunning.Operations.response field if the operation was successful.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportProductsResponse {
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
    ///
    /// https:
    /// //console.cloud.google.com/recommendatio
    /// // n/datafeeds/default_catalog/dashboard
    #[prost(string, tag = "1")]
    pub placement: std::string::String,
    /// Required. Context about the user, what they are looking at and what action
    /// they took to trigger the predict request. Note that this user event detail
    /// won't be ingested to userEvent logs. Thus, a separate userEvent write
    /// request is required for event logging.
    #[prost(message, optional, tag = "2")]
    pub user_event: ::std::option::Option<UserEvent>,
    /// Maximum number of results to return per page. Set this property
    /// to the number of prediction results needed. If zero, the service will
    /// choose a reasonable default. The maximum allowed value is 100. Values
    /// above 100 will be coerced to 100.
    #[prost(int32, tag = "3")]
    pub page_size: i32,
    /// The previous PredictResponse.next_page_token.
    #[prost(string, tag = "4")]
    pub page_token: std::string::String,
    /// Filter for restricting prediction results. Accepts values for
    /// tags and the `filterOutOfStockItems` flag.
    ///
    ///  * Tag expressions. Restricts predictions to products that match all of the
    ///    specified tags. Boolean operators `OR` and `NOT` are supported if the
    ///    expression is enclosed in parentheses, and must be separated from the
    ///    tag values by a space. `-"tagA"` is also supported and is equivalent to
    ///    `NOT "tagA"`. Tag values must be double quoted UTF-8 encoded strings
    ///    with a size limit of 1 KiB.
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
    pub filter: std::string::String,
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
    pub params: ::std::collections::HashMap<std::string::String, ::prost_types::Value>,
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
    pub labels: ::std::collections::HashMap<std::string::String, std::string::String>,
}
/// Response message for predict method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PredictResponse {
    /// A list of recommended products. The order represents the ranking (from the
    /// most relevant product to the least).
    #[prost(message, repeated, tag = "1")]
    pub results: ::std::vec::Vec<predict_response::PredictionResult>,
    /// A unique attribution token. This should be included in the
    /// [UserEvent][google.cloud.retail.v2alpha.UserEvent] logs resulting from this
    /// recommendation, which enables accurate attribution of recommendation model
    /// performance.
    #[prost(string, tag = "2")]
    pub attribution_token: std::string::String,
    /// IDs of products in the request that were missing from the inventory.
    #[prost(string, repeated, tag = "3")]
    pub missing_ids: ::std::vec::Vec<std::string::String>,
    /// True if the validateOnly property was set in the request.
    #[prost(bool, tag = "4")]
    pub validate_only: bool,
}
pub mod predict_response {
    /// PredictionResult represents the recommendation prediction results.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PredictionResult {
        /// ID of the recommended product
        #[prost(string, tag = "1")]
        pub id: std::string::String,
        /// Additional product metadata / annotations.
        ///
        /// Possible values:
        ///
        /// * `product`: JSON representation of the product. Will be set if
        ///   `returnProduct` is set to true in `PredictRequest.params`.
        /// * `score`: Prediction score in double value. Will be set if
        ///   `returnScore` is set to true in `PredictRequest.params`.
        #[prost(map = "string, message", tag = "2")]
        pub metadata: ::std::collections::HashMap<std::string::String, ::prost_types::Value>,
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
                "/google.cloud.retail.v2alpha.PredictionService/Predict",
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
pub struct PurgeMetadata {
    /// The ID of the request / operation.
    #[prost(string, tag = "1")]
    pub operation: std::string::String,
    /// Operation create time.
    #[prost(message, optional, tag = "2")]
    pub create_time: ::std::option::Option<::prost_types::Timestamp>,
}
/// Request message for PurgeProducts method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PurgeProductsRequest {
    /// Required. The resource name of the catalog under which the products are
    /// created. The format is
    /// "projects/${projectId}/locations/global/catalogs/${catalogId}"
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Required. Filter matching the products to be purged. Only supported value
    /// at the moment is "*" (all items).
    #[prost(string, tag = "2")]
    pub filter: std::string::String,
    /// The default value is false. Override this flag to true to
    /// actually perform the purge. If the field is not set to true, a sampling of
    /// products to be deleted will be returned.
    #[prost(bool, tag = "3")]
    pub force: bool,
}
/// Response of the PurgeProductsRequest. If the long running operation is
/// successfully done, then this message is returned by the
/// google.longrunning.Operations.response field.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PurgeProductsResponse {
    /// The total count of products purged as a result of the operation.
    #[prost(int64, tag = "1")]
    pub purged_count: i64,
    /// A random sampling of products deleted (or will be deleted) depending
    /// on the `force` property in the request. Max of 500 items will be returned.
    /// Currently, this is only populated if force=false.
    #[prost(message, repeated, tag = "2")]
    pub products_sample: ::std::vec::Vec<Product>,
}
/// Request message for PurgeUserEvents method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PurgeUserEventsRequest {
    /// Required. The resource name of the event_store under which the events are
    /// created. The format is
    ///
    /// "projects/${projectId}/locations/global/catalogs/${catalogId}/eventStores/${eventStoreId}"
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Required. The filter string to specify the events to be deleted. Empty
    /// string filter is not allowed. The eligible fields
    /// for filtering are:
    ///
    /// * `eventType`: UserEvent.eventType field of type string.
    /// * `eventTime`: in ISO 8601 "zulu" format.
    /// * `visitorId`: field of type string. Specifying this will delete all
    ///   events associated with a visitor.
    /// * `userId`: field of type string. Specifying this will delete all events
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
    pub filter: std::string::String,
    /// The default value is false. Override this flag to true to
    /// actually perform the purge. If the field is not set to true, a sampling of
    /// events to be deleted will be returned.
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
    /// A sampling of events deleted (or will be deleted) depending on the `force`
    /// property in the request. Max of 500 items will be returned.
    #[prost(message, repeated, tag = "2")]
    pub user_events_sample: ::std::vec::Vec<UserEvent>,
}
/// Request message for [CreateProduct][] method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateProductRequest {
    /// Required. The parent catalog resource name, such as
    ///
    /// "projects/*/locations/global/catalogs/default_catalog/branches/default_branch".
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Required. The [Product][google.cloud.retail.v2alpha.Product] to create.
    #[prost(message, optional, tag = "2")]
    pub product: ::std::option::Option<Product>,
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
    /// This field must be a UTF-8 encoded string with a length limit of 128 bytes.
    /// Otherwise, an INVALID_ARGUMENT error is returned.
    #[prost(string, tag = "3")]
    pub product_id: std::string::String,
}
/// Request message for [GetProduct][] method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetProductRequest {
    /// Required. Full resource name of
    /// [Product][google.cloud.retail.v2alpha.Product], such as
    ///
    /// "projects/*/locations/global/catalogs/default_catalog/branches/default_branch/products/some_product_id".
    ///
    /// If the caller does not have permission to access the
    /// [Product][google.cloud.retail.v2alpha.Product], regardless of whether or
    /// not it exists, a PERMISSION_DENIED error is returned.
    ///
    /// If the requested [Product][google.cloud.retail.v2alpha.Product] does not
    /// exist, a NOT_FOUND error is returned.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
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
    /// exist, a NOT_FOUND error is returned.
    #[prost(message, optional, tag = "1")]
    pub product: ::std::option::Option<Product>,
    /// Indicates which fields in the provided
    /// [Product][google.cloud.retail.v2alpha.Product] to update. The immutable and
    /// output only fields are NOT supported. If not set, all supported fields (the
    /// fields that are neither immutable nor output only) are updated.
    ///
    /// If an unsupported or unknown field is provided, an INVALID_ARGUMENT error
    /// is returned.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
}
/// Request message for [DeleteProduct][] method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteProductRequest {
    /// Required. Full resource name of
    /// [Product][google.cloud.retail.v2alpha.Product], such as
    ///
    /// "projects/*/locations/global/catalogs/default_catalog/branches/default_branch/products/some_product_id".
    ///
    /// If the caller does not have permission to delete the
    /// [Product][google.cloud.retail.v2alpha.Product], regardless of whether or
    /// not it exists, a PERMISSION_DENIED error is returned.
    ///
    /// If the [Product][google.cloud.retail.v2alpha.Product] to delete does not
    /// exist, a NOT_FOUND error is returned.
    ///
    /// The [Product][google.cloud.retail.v2alpha.Product] to delete can neither be
    /// a non-empty
    /// [Product.Type.COLLECTION][google.cloud.retail.v2alpha.Product.Type.COLLECTION]
    /// [Product][google.cloud.retail.v2alpha.Product] nor a
    /// [Product.Type.PRIMARY][google.cloud.retail.v2alpha.Product.Type.PRIMARY]
    /// [Product][google.cloud.retail.v2alpha.Product] with more than one
    /// [variants][google.cloud.retail.v2alpha.Product.Type.VARIANT]. Otherwise, a
    /// FAILED_PRECONDITION error is returned.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
#[doc = r" Generated client implementations."]
pub mod product_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Service for ingesting [Product][google.cloud.retail.v2alpha.Product]"]
    #[doc = " information of the customer's website."]
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
        #[doc = " Updates a [Product][google.cloud.retail.v2alpha.Product]. Non-existing"]
        #[doc = " items will be created."]
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
        #[doc = " Permanently deletes all [Product][google.cloud.retail.v2alpha.Product]s"]
        #[doc = " under a branch."]
        #[doc = ""]
        #[doc = " Depending on the number of [Product][google.cloud.retail.v2alpha.Product]s,"]
        #[doc = " this operation could take hours to complete. To get a sample of"]
        #[doc = " [Product][google.cloud.retail.v2alpha.Product]s that would be deleted, set"]
        #[doc = " [PurgeProductsRequest.force][google.cloud.retail.v2alpha.PurgeProductsRequest.force]"]
        #[doc = " to false."]
        pub async fn purge_products(
            &mut self,
            request: impl tonic::IntoRequest<super::PurgeProductsRequest>,
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
                "/google.cloud.retail.v2alpha.ProductService/PurgeProducts",
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
        #[doc = " Export of multiple [Product][google.cloud.retail.v2alpha.Product]s."]
        pub async fn export_products(
            &mut self,
            request: impl tonic::IntoRequest<super::ExportProductsRequest>,
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
                "/google.cloud.retail.v2alpha.ProductService/ExportProducts",
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
    /// Required. The parent eventStore resource name, such as
    /// "projects/1234/locations/global/catalogs/default_catalog".
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
    /// "projects/1234/locations/global/catalogs/default_catalog".
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Required. URL encoded UserEvent proto.
    #[prost(string, tag = "2")]
    pub user_event: std::string::String,
    /// The url including cgi-parameters but excluding the hash fragment. The URL
    /// must be truncated to 1.5K bytes to conservatively be under the 2K bytes.
    /// This is often more useful than the referer url, because many browsers only
    /// send the domain for 3rd party requests.
    #[prost(string, tag = "3")]
    pub uri: std::string::String,
    /// The event timestamp in milliseconds. This prevents browser caching of
    /// otherwise identical get requests. The name is abbreviated to reduce the
    /// payload bytes.
    #[prost(int64, tag = "4")]
    pub ets: i64,
}
/// Request message for CatalogRejoin method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RejoinUserEventsRequest {
    /// Required. Full resource name of user event, such as
    /// "projects/*/locations/*/catalogs/default_catalog".
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Required. The type of the catalog rejoin to define the scope and range of
    /// the user events to be rejoined with catalog items.
    #[prost(
        enumeration = "rejoin_user_events_request::UserEventRejoinScope",
        tag = "2"
    )]
    pub user_event_rejoin_scope: i32,
}
pub mod rejoin_user_events_request {
    /// The scope of events to be rejoined with latest catalog. If the rejoining
    /// aims at reducing number of unjoined events, set UserEventRejoinScope to
    /// UNJOINED_EVENTS. If the rejoining aims at correcting catalog information
    /// in joined_events, set UserEventRejoinScope to JOINED_EVENTS. If all events
    /// needs to be rejoined, set UserEventRejoinScope to
    /// USER_EVENT_REJOIN_SCOPE_UNSPECIFIED.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum UserEventRejoinScope {
        /// Rejoin catalogs with all events including both joined events and
        /// unjoined events.
        Unspecified = 0,
        /// Only rejoin catalogs with joined events.
        JoinedEvents = 1,
        /// Only rejoin catalogs with unjoined events.
        UnjoinedEvents = 2,
    }
}
/// Response message for RejoinUserEvents method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RejoinUserEventsResponse {
    /// Number of user events that were joined with latest catalog items.
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
                "/google.cloud.retail.v2alpha.UserEventService/WriteUserEvent",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Writes a single user event from the browser. This uses a GET request to"]
        #[doc = " due to browser restriction of POST-ing to a 3rd party domain."]
        #[doc = ""]
        #[doc = " This method is used only by the Recommendations AI JavaScript pixel and"]
        #[doc = " Google Tag Manager. Users should not call this method directly."]
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
        #[doc = " Export of user events."]
        #[doc = ""]
        #[doc = " Operation.response is of type ExportResponse."]
        #[doc = " Operation.metadata is of type ExportMetadata."]
        pub async fn export_user_events(
            &mut self,
            request: impl tonic::IntoRequest<super::ExportUserEventsRequest>,
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
                "/google.cloud.retail.v2alpha.UserEventService/ExportUserEvents",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Triggers a user event rejoin operation with latest catalog data. Events"]
        #[doc = " will not be annotated with detailed catalog information if catalog item is"]
        #[doc = " missing at the time the user event is ingested, and these events are stored"]
        #[doc = " as unjoined events with a limited usage on training and serving. This API"]
        #[doc = " can be used to trigger a 'join' operation on specified events with latest"]
        #[doc = " version of catalog items. It can also be used to correct events joined with"]
        #[doc = " wrong catalog items."]
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
