/// Request message for \[CustomColumnService.GetCustomColumn][google.ads.searchads360.v0.services.CustomColumnService.GetCustomColumn\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCustomColumnRequest {
    /// Required. The resource name of the custom column to fetch.
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
}
/// Request message for \[CustomColumnService.ListCustomColumns][google.ads.searchads360.v0.services.CustomColumnService.ListCustomColumns\]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCustomColumnsRequest {
    /// Required. The ID of the customer to apply the CustomColumn list operation to.
    #[prost(string, tag = "1")]
    pub customer_id: ::prost::alloc::string::String,
}
/// Response message for fetching all custom columns associated with a customer.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCustomColumnsResponse {
    /// The CustomColumns owned by the provided customer.
    #[prost(message, repeated, tag = "1")]
    pub custom_columns: ::prost::alloc::vec::Vec<super::resources::CustomColumn>,
}
/// Generated client implementations.
pub mod custom_column_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Service to manage custom columns.
    #[derive(Debug, Clone)]
    pub struct CustomColumnServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl CustomColumnServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> CustomColumnServiceClient<T>
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
        ) -> CustomColumnServiceClient<InterceptedService<T, F>>
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
            CustomColumnServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Returns the requested custom column in full detail.
        pub async fn get_custom_column(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCustomColumnRequest>,
        ) -> Result<
            tonic::Response<super::super::resources::CustomColumn>,
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
                "/google.ads.searchads360.v0.services.CustomColumnService/GetCustomColumn",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Returns all the custom columns associated with the customer in full detail.
        pub async fn list_custom_columns(
            &mut self,
            request: impl tonic::IntoRequest<super::ListCustomColumnsRequest>,
        ) -> Result<tonic::Response<super::ListCustomColumnsResponse>, tonic::Status> {
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
                "/google.ads.searchads360.v0.services.CustomColumnService/ListCustomColumns",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Request message for \[SearchAds360FieldService.GetSearchAds360Field][google.ads.searchads360.v0.services.SearchAds360FieldService.GetSearchAds360Field\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSearchAds360FieldRequest {
    /// Required. The resource name of the field to get.
    #[prost(string, tag = "1")]
    pub resource_name: ::prost::alloc::string::String,
}
/// Request message for \[SearchAds360FieldService.SearchSearchAds360Fields][google.ads.searchads360.v0.services.SearchAds360FieldService.SearchSearchAds360Fields\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchSearchAds360FieldsRequest {
    /// Required. The query string.
    #[prost(string, tag = "1")]
    pub query: ::prost::alloc::string::String,
    /// Token of the page to retrieve. If not specified, the first page of
    /// results will be returned. Use the value obtained from `next_page_token`
    /// in the previous response in order to request the next page of results.
    #[prost(string, tag = "2")]
    pub page_token: ::prost::alloc::string::String,
    /// Number of elements to retrieve in a single page.
    /// When too large a page is requested, the server may decide to further
    /// limit the number of returned resources.
    #[prost(int32, tag = "3")]
    pub page_size: i32,
}
/// Response message for \[SearchAds360FieldService.SearchSearchAds360Fields][google.ads.searchads360.v0.services.SearchAds360FieldService.SearchSearchAds360Fields\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchSearchAds360FieldsResponse {
    /// The list of fields that matched the query.
    #[prost(message, repeated, tag = "1")]
    pub results: ::prost::alloc::vec::Vec<super::resources::SearchAds360Field>,
    /// Pagination token used to retrieve the next page of results. Pass the
    /// content of this string as the `page_token` attribute of the next request.
    /// `next_page_token` is not returned for the last page.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Total number of results that match the query ignoring the LIMIT clause.
    #[prost(int64, tag = "3")]
    pub total_results_count: i64,
}
/// Generated client implementations.
pub mod search_ads360_field_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Service to fetch Search Ads 360 API fields.
    #[derive(Debug, Clone)]
    pub struct SearchAds360FieldServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl SearchAds360FieldServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> SearchAds360FieldServiceClient<T>
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
        ) -> SearchAds360FieldServiceClient<InterceptedService<T, F>>
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
            SearchAds360FieldServiceClient::new(
                InterceptedService::new(inner, interceptor),
            )
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
        /// Returns just the requested field.
        ///
        /// List of thrown errors:
        ///   [AuthenticationError]()
        ///   [AuthorizationError]()
        ///   [HeaderError]()
        ///   [InternalError]()
        ///   [QuotaError]()
        ///   [RequestError]()
        pub async fn get_search_ads360_field(
            &mut self,
            request: impl tonic::IntoRequest<super::GetSearchAds360FieldRequest>,
        ) -> Result<
            tonic::Response<super::super::resources::SearchAds360Field>,
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
                "/google.ads.searchads360.v0.services.SearchAds360FieldService/GetSearchAds360Field",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Returns all fields that match the search query.
        ///
        /// List of thrown errors:
        ///   [AuthenticationError]()
        ///   [AuthorizationError]()
        ///   [HeaderError]()
        ///   [InternalError]()
        ///   [QueryError]()
        ///   [QuotaError]()
        ///   [RequestError]()
        pub async fn search_search_ads360_fields(
            &mut self,
            request: impl tonic::IntoRequest<super::SearchSearchAds360FieldsRequest>,
        ) -> Result<
            tonic::Response<super::SearchSearchAds360FieldsResponse>,
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
                "/google.ads.searchads360.v0.services.SearchAds360FieldService/SearchSearchAds360Fields",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Request message for \[SearchAds360Service.Search][google.ads.searchads360.v0.services.SearchAds360Service.Search\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchSearchAds360Request {
    /// Required. The ID of the customer being queried.
    #[prost(string, tag = "1")]
    pub customer_id: ::prost::alloc::string::String,
    /// Required. The query string.
    #[prost(string, tag = "2")]
    pub query: ::prost::alloc::string::String,
    /// Token of the page to retrieve. If not specified, the first
    /// page of results will be returned. Use the value obtained from
    /// `next_page_token` in the previous response in order to request
    /// the next page of results.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Number of elements to retrieve in a single page.
    /// When too large a page is requested, the server may decide to
    /// further limit the number of returned resources.
    #[prost(int32, tag = "4")]
    pub page_size: i32,
    /// If true, the request is validated but not executed.
    #[prost(bool, tag = "5")]
    pub validate_only: bool,
    /// If true, the total number of results that match the query ignoring the
    /// LIMIT clause will be included in the response.
    /// Default is false.
    #[prost(bool, tag = "7")]
    pub return_total_results_count: bool,
    /// Determines whether a summary row will be returned. By default, summary row
    /// is not returned. If requested, the summary row will be sent in a response
    /// by itself after all other query results are returned.
    #[prost(
        enumeration = "super::enums::summary_row_setting_enum::SummaryRowSetting",
        tag = "8"
    )]
    pub summary_row_setting: i32,
}
/// Response message for \[SearchAds360Service.Search][google.ads.searchads360.v0.services.SearchAds360Service.Search\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchSearchAds360Response {
    /// The list of rows that matched the query.
    #[prost(message, repeated, tag = "1")]
    pub results: ::prost::alloc::vec::Vec<SearchAds360Row>,
    /// Pagination token used to retrieve the next page of results.
    /// Pass the content of this string as the `page_token` attribute of
    /// the next request. `next_page_token` is not returned for the last
    /// page.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Total number of results that match the query ignoring the LIMIT
    /// clause.
    #[prost(int64, tag = "3")]
    pub total_results_count: i64,
    /// FieldMask that represents what fields were requested by the user.
    #[prost(message, optional, tag = "5")]
    pub field_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Summary row that contains summary of metrics in results.
    /// Summary of metrics means aggregation of metrics across all results,
    /// here aggregation could be sum, average, rate, etc.
    #[prost(message, optional, tag = "6")]
    pub summary_row: ::core::option::Option<SearchAds360Row>,
    /// The headers of the custom columns in the results.
    #[prost(message, repeated, tag = "7")]
    pub custom_column_headers: ::prost::alloc::vec::Vec<CustomColumnHeader>,
}
/// Request message for \[SearchAds360Service.SearchStream][google.ads.searchads360.v0.services.SearchAds360Service.SearchStream\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchSearchAds360StreamRequest {
    /// Required. The ID of the customer being queried.
    #[prost(string, tag = "1")]
    pub customer_id: ::prost::alloc::string::String,
    /// Required. The query string.
    #[prost(string, tag = "2")]
    pub query: ::prost::alloc::string::String,
    /// The number of rows that are returned in each stream response batch.
    /// When too large batch is requested, the server may decide to further limit
    /// the number of returned rows.
    #[prost(int32, tag = "4")]
    pub batch_size: i32,
    /// Determines whether a summary row will be returned. By default, summary row
    /// is not returned. If requested, the summary row will be sent in a response
    /// by itself after all other query results are returned.
    #[prost(
        enumeration = "super::enums::summary_row_setting_enum::SummaryRowSetting",
        tag = "3"
    )]
    pub summary_row_setting: i32,
}
/// Response message for \[SearchAds360Service.SearchStream][google.ads.searchads360.v0.services.SearchAds360Service.SearchStream\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchSearchAds360StreamResponse {
    /// The list of rows that matched the query.
    #[prost(message, repeated, tag = "1")]
    pub results: ::prost::alloc::vec::Vec<SearchAds360Row>,
    /// FieldMask that represents what fields were requested by the user.
    #[prost(message, optional, tag = "2")]
    pub field_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Summary row that contains summary of metrics in results.
    /// Summary of metrics means aggregation of metrics across all results,
    /// here aggregation could be sum, average, rate, etc.
    #[prost(message, optional, tag = "3")]
    pub summary_row: ::core::option::Option<SearchAds360Row>,
    /// The headers of the custom columns in the results.
    #[prost(message, repeated, tag = "5")]
    pub custom_column_headers: ::prost::alloc::vec::Vec<CustomColumnHeader>,
    /// The unique id of the request that is used for debugging purposes.
    #[prost(string, tag = "4")]
    pub request_id: ::prost::alloc::string::String,
}
/// A returned row from the query.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchAds360Row {
    /// The ad group referenced in the query.
    #[prost(message, optional, tag = "3")]
    pub ad_group: ::core::option::Option<super::resources::AdGroup>,
    /// The bid modifier referenced in the query.
    #[prost(message, optional, tag = "24")]
    pub ad_group_bid_modifier: ::core::option::Option<
        super::resources::AdGroupBidModifier,
    >,
    /// The criterion referenced in the query.
    #[prost(message, optional, tag = "17")]
    pub ad_group_criterion: ::core::option::Option<super::resources::AdGroupCriterion>,
    /// The bidding strategy referenced in the query.
    #[prost(message, optional, tag = "18")]
    pub bidding_strategy: ::core::option::Option<super::resources::BiddingStrategy>,
    /// The campaign budget referenced in the query.
    #[prost(message, optional, tag = "19")]
    pub campaign_budget: ::core::option::Option<super::resources::CampaignBudget>,
    /// The campaign referenced in the query.
    #[prost(message, optional, tag = "2")]
    pub campaign: ::core::option::Option<super::resources::Campaign>,
    /// The campaign criterion referenced in the query.
    #[prost(message, optional, tag = "20")]
    pub campaign_criterion: ::core::option::Option<super::resources::CampaignCriterion>,
    /// The conversion action referenced in the query.
    #[prost(message, optional, tag = "103")]
    pub conversion_action: ::core::option::Option<super::resources::ConversionAction>,
    /// The customer referenced in the query.
    #[prost(message, optional, tag = "1")]
    pub customer: ::core::option::Option<super::resources::Customer>,
    /// The CustomerManagerLink referenced in the query.
    #[prost(message, optional, tag = "61")]
    pub customer_manager_link: ::core::option::Option<
        super::resources::CustomerManagerLink,
    >,
    /// The CustomerClient referenced in the query.
    #[prost(message, optional, tag = "70")]
    pub customer_client: ::core::option::Option<super::resources::CustomerClient>,
    /// The metrics.
    #[prost(message, optional, tag = "4")]
    pub metrics: ::core::option::Option<super::common::Metrics>,
    /// The segments.
    #[prost(message, optional, tag = "102")]
    pub segments: ::core::option::Option<super::common::Segments>,
    /// The custom columns.
    #[prost(message, repeated, tag = "156")]
    pub custom_columns: ::prost::alloc::vec::Vec<super::common::Value>,
}
/// Message for custom column header.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomColumnHeader {
    /// The custom column ID.
    #[prost(int64, tag = "1")]
    pub id: i64,
    /// The user defined name of the custom column.
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    /// True when the custom column references metrics.
    #[prost(bool, tag = "3")]
    pub references_metrics: bool,
}
/// Generated client implementations.
pub mod search_ads360_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Service to fetch data and metrics across resources.
    #[derive(Debug, Clone)]
    pub struct SearchAds360ServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl SearchAds360ServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> SearchAds360ServiceClient<T>
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
        ) -> SearchAds360ServiceClient<InterceptedService<T, F>>
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
            SearchAds360ServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Returns all rows that match the search query.
        ///
        /// List of thrown errors:
        ///   [AuthenticationError]()
        ///   [AuthorizationError]()
        ///   [HeaderError]()
        ///   [InternalError]()
        ///   [QueryError]()
        ///   [QuotaError]()
        ///   [RequestError]()
        pub async fn search(
            &mut self,
            request: impl tonic::IntoRequest<super::SearchSearchAds360Request>,
        ) -> Result<tonic::Response<super::SearchSearchAds360Response>, tonic::Status> {
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
                "/google.ads.searchads360.v0.services.SearchAds360Service/Search",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Returns all rows that match the search stream query.
        ///
        /// List of thrown errors:
        ///   [AuthenticationError]()
        ///   [AuthorizationError]()
        ///   [HeaderError]()
        ///   [InternalError]()
        ///   [QueryError]()
        ///   [QuotaError]()
        ///   [RequestError]()
        pub async fn search_stream(
            &mut self,
            request: impl tonic::IntoRequest<super::SearchSearchAds360StreamRequest>,
        ) -> Result<
            tonic::Response<
                tonic::codec::Streaming<super::SearchSearchAds360StreamResponse>,
            >,
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
                "/google.ads.searchads360.v0.services.SearchAds360Service/SearchStream",
            );
            self.inner.server_streaming(request.into_request(), path, codec).await
        }
    }
}
