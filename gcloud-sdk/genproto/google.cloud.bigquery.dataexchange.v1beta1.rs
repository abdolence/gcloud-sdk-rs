///  A data exchange is a container that enables data sharing.
///  It contains a set of listings of the data sources along with descriptive
///  information of the data exchange.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DataExchange {
    ///  Output only. The resource name of the data exchange.
    ///  e.g. `projects/myproject/locations/US/dataExchanges/123`.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    ///  Required. Human-readable display name of the data exchange. The display name must
    ///  contain only Unicode letters, numbers (0-9), underscores (_), dashes (-),
    ///  spaces ( ), and can't start or end with spaces.
    ///  Default value is an empty string.
    ///  Max length: 63 bytes.
    #[prost(string, tag="2")]
    pub display_name: ::prost::alloc::string::String,
    ///  Optional. Short description of the data exchange that can consist of sentences
    ///  or paragraphs. The description must not contain Unicode non-characters as
    ///  well as C0 and C1 control codes except tabs (HT), new lines (LF), carriage
    ///  returns (CR), and page breaks (FF).
    ///  Default value is an empty string.
    ///  Max length: 2000 bytes.
    #[prost(string, tag="3")]
    pub description: ::prost::alloc::string::String,
    ///  Optional. Email, URL or other reference of the primary point of contact of the data
    ///  exchange
    ///  Max Length: 1000 bytes.
    #[prost(string, tag="4")]
    pub primary_contact: ::prost::alloc::string::String,
    ///  Optional. Documentation describing the data exchange.
    #[prost(string, tag="5")]
    pub documentation: ::prost::alloc::string::String,
    ///  Output only. Number of listings contained in the data exchange.
    #[prost(int32, tag="6")]
    pub listing_count: i32,
    ///  Optional. Base64 encoded image representing the data exchange. Max Size: 3.0MiB
    ///  Expected image dimensions are 512x512 pixels, however the API only
    ///  performs validation on size of the encoded data.
    ///  Note: For byte fields, the contents of the field are base64-encoded (which
    ///  increases the size of the data by 33-36%) when using JSON on the wire.
    #[prost(bytes="vec", tag="7")]
    pub icon: ::prost::alloc::vec::Vec<u8>,
}
///  Contains details of the Data Provider.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DataProvider {
    ///  Optional. Name of the Data Provider.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    ///  Optional. Email or URL of the Data Provider.
    ///  Max Length: 1000 bytes.
    #[prost(string, tag="2")]
    pub primary_contact: ::prost::alloc::string::String,
}
///  Contains details of the Publisher.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Publisher {
    ///  Optional. Name of the listing Publisher.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    ///  Optional. Email or URL of the listing Publisher.
    ///  Max Length: 1000 bytes.
    #[prost(string, tag="2")]
    pub primary_contact: ::prost::alloc::string::String,
}
///  Defines the Destination BigQuery Dataset Reference.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DestinationDatasetReference {
    ///  Required. A unique ID for this dataset, without the project name. The ID
    ///  must contain only letters (a-z, A-Z), numbers (0-9), or underscores (_).
    ///  The maximum length is 1,024 characters.
    #[prost(string, tag="1")]
    pub dataset_id: ::prost::alloc::string::String,
    ///  Required. The ID of the project containing this dataset.
    #[prost(string, tag="2")]
    pub project_id: ::prost::alloc::string::String,
}
///  Defines the Destination BigQuery Dataset.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DestinationDataset {
    ///  Required. A reference that identifies the destination dataset.
    #[prost(message, optional, tag="1")]
    pub dataset_reference: ::core::option::Option<DestinationDatasetReference>,
    ///  Optional. A descriptive name for the dataset.
    #[prost(message, optional, tag="2")]
    pub friendly_name: ::core::option::Option<::prost::alloc::string::String>,
    ///  Optional. A user-friendly description of the dataset.
    #[prost(message, optional, tag="3")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
    ///  Optional. The labels associated with this dataset. You can use these
    ///  to organize and group your datasets.
    ///  You can set this property when inserting or updating a dataset.
    ///  See <https://cloud.google.com/resource-manager/docs/creating-managing-labels>
    ///  for more information.
    #[prost(map="string, string", tag="4")]
    pub labels: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    ///  Required. The geographic location where the dataset should reside. See
    ///  <https://cloud.google.com/bigquery/docs/locations> for supported
    ///  locations.
    #[prost(string, tag="5")]
    pub location: ::prost::alloc::string::String,
}
///  A listing is what gets published into a data exchange that a subscriber can
///  subscribe to. It contains a reference to the data source along with
///  descriptive information that will help subscribers find and subscribe the
///  data.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Listing {
    ///  Output only. The resource name of the listing.
    ///  e.g. `projects/myproject/locations/US/dataExchanges/123/listings/456`
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    ///  Required. Human-readable display name of the listing. The display name must contain
    ///  only Unicode letters, numbers (0-9), underscores (_), dashes (-), spaces
    ///  ( ), and can't start or end with spaces.
    ///  Default value is an empty string.
    ///  Max length: 63 bytes.
    #[prost(string, tag="2")]
    pub display_name: ::prost::alloc::string::String,
    ///  Optional. Short description of the listing that can consist of sentences or
    ///  paragraphs. The description must not contain Unicode non-characters as
    ///  well as C0 and C1 control codes except tabs (HT), new lines (LF), carriage
    ///  returns (CR), and page breaks (FF).
    ///  Default value is an empty string.
    ///  Max length: 2000 bytes.
    #[prost(string, tag="3")]
    pub description: ::prost::alloc::string::String,
    ///  Optional. Email or URL of the primary point of contact of the listing.
    ///  Max Length: 1000 bytes.
    #[prost(string, tag="4")]
    pub primary_contact: ::prost::alloc::string::String,
    ///  Optional. Documentation describing the listing.
    #[prost(string, tag="5")]
    pub documentation: ::prost::alloc::string::String,
    ///  Output only. Current state of the Listing.
    #[prost(enumeration="listing::State", tag="7")]
    pub state: i32,
    ///  Optional. Base64 encoded image representing the listing. Max Size: 3.0MiB
    ///  Expected image dimensions are 512x512 pixels, however the API only
    ///  performs validation on size of the encoded data.
    ///  Note: For byte fields, the contents of the field are base64-encoded (which
    ///  increases the size of the data by 33-36%) when using JSON on the wire.
    #[prost(bytes="vec", tag="8")]
    pub icon: ::prost::alloc::vec::Vec<u8>,
    ///  Optional. The details of the Data Provider who owns the source data.
    #[prost(message, optional, tag="9")]
    pub data_provider: ::core::option::Option<DataProvider>,
    ///  Optional. Categories of the Listing. Up to two categories are allowed.
    #[prost(enumeration="super::common::Category", repeated, packed="false", tag="10")]
    pub categories: ::prost::alloc::vec::Vec<i32>,
    ///  Optional. The details of the Publisher who owns the listing and has rights to share
    ///  the source data.
    #[prost(message, optional, tag="11")]
    pub publisher: ::core::option::Option<Publisher>,
    ///  Optional. Email or URL of the request access of the listing.
    ///  Subscribers can use this reference to request access.
    ///  Max Length: 1000 bytes.
    #[prost(string, tag="12")]
    pub request_access: ::prost::alloc::string::String,
    ///  Listing source.
    #[prost(oneof="listing::Source", tags="6")]
    pub source: ::core::option::Option<listing::Source>,
}
/// Nested message and enum types in `Listing`.
pub mod listing {
    ///  A reference to a Shared dataset. It's an existing BigQuery dataset with a
    ///  collection of objects, such as tables and views, that you want to share
    ///  with subscribers.
    ///  Upon subscription to a Listing, Data Exchange creates a Linked dataset in
    ///  the subscriber's project. A Linked dataset is an opaque, read-only BigQuery
    ///  dataset that serves as a "symbolic link" to a shared dataset.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct BigQueryDatasetSource {
        ///  Resource name of the dataset source for this listing.
        ///  e.g. `projects/myproject/datasets/123`
        #[prost(string, tag="1")]
        pub dataset: ::prost::alloc::string::String,
    }
    ///  State of the Listing
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        ///  Default value. This value is unused.
        Unspecified = 0,
        ///  Subscribable state. Users with dataexchange.listings.subscribe permission
        ///  can subscribe to this Listing.
        Active = 1,
    }
    impl State {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                State::Unspecified => "STATE_UNSPECIFIED",
                State::Active => "ACTIVE",
            }
        }
    }
    ///  Listing source.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Source {
        ///  Required. Shared dataset i.e. BigQuery dataset source.
        #[prost(message, tag="6")]
        BigqueryDataset(BigQueryDatasetSource),
    }
}
///  Message for requesting list of DataExchanges.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDataExchangesRequest {
    ///  Required. The parent resource path of the DataExchanges.
    ///  e.g. `projects/myproject/locations/US`.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    ///  The maximum number of results to return in a single response page. Leverage
    ///  the page tokens to iterate through the entire collection.
    #[prost(int32, tag="2")]
    pub page_size: i32,
    ///  Page token, returned by a previous call, to request the next page of
    ///  results.
    #[prost(string, tag="3")]
    pub page_token: ::prost::alloc::string::String,
}
///  Message for response to listing DataExchanges.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDataExchangesResponse {
    ///  The list of DataExchange.
    #[prost(message, repeated, tag="1")]
    pub data_exchanges: ::prost::alloc::vec::Vec<DataExchange>,
    ///  A token to request the next page of results.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
///  Message for requesting list of DataExchanges from projects in an organization
///  and location.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListOrgDataExchangesRequest {
    ///  Required. The organization resource path of the projects containing DataExchanges.
    ///  e.g. `organizations/myorg/locations/US`.
    #[prost(string, tag="1")]
    pub organization: ::prost::alloc::string::String,
    ///  The maximum number of results to return in a single response page. Leverage
    ///  the page tokens to iterate through the entire collection.
    #[prost(int32, tag="2")]
    pub page_size: i32,
    ///  Page token, returned by a previous call, to request the next page of
    ///  results.
    #[prost(string, tag="3")]
    pub page_token: ::prost::alloc::string::String,
}
///  Message for response to listing DataExchanges in an organization and
///  location.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListOrgDataExchangesResponse {
    ///  The list of DataExchange.
    #[prost(message, repeated, tag="1")]
    pub data_exchanges: ::prost::alloc::vec::Vec<DataExchange>,
    ///  A token to request the next page of results.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
///  Message for getting a DataExchange.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDataExchangeRequest {
    ///  Required. The resource name of the DataExchange.
    ///  e.g. `projects/myproject/locations/US/dataExchanges/123`.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
///  Message for creating a DataExchange.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateDataExchangeRequest {
    ///  Required. The parent resource path of the DataExchange.
    ///  e.g. `projects/myproject/locations/US`.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    ///  Required. The ID of the DataExchange to create.
    ///  Must contain only Unicode letters, numbers (0-9), underscores (_).
    ///  Should not use characters that require URL-escaping, or characters
    ///  outside of ASCII, spaces.
    ///  Max length: 100 bytes.
    #[prost(string, tag="2")]
    pub data_exchange_id: ::prost::alloc::string::String,
    ///  Required. The DataExchange to create.
    #[prost(message, optional, tag="3")]
    pub data_exchange: ::core::option::Option<DataExchange>,
}
///  Message for updating a DataExchange.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateDataExchangeRequest {
    ///  Required. Field mask is used to specify the fields to be overwritten in the
    ///  DataExchange resource by the update.
    ///  The fields specified in the update_mask are relative to the resource, not
    ///  the full request.
    #[prost(message, optional, tag="1")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    ///  Required. The DataExchange to update.
    #[prost(message, optional, tag="2")]
    pub data_exchange: ::core::option::Option<DataExchange>,
}
///  Message for deleting a DataExchange.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteDataExchangeRequest {
    ///  Required. Resource name of the DataExchange to delete.
    ///  e.g. `projects/myproject/locations/US/dataExchanges/123`.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
///  Message for requesting list of Listings.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListListingsRequest {
    ///  Required. The parent resource path of the listing.
    ///  e.g. `projects/myproject/locations/US/dataExchanges/123`.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    ///  The maximum number of results to return in a single response page. Leverage
    ///  the page tokens to iterate through the entire collection.
    #[prost(int32, tag="2")]
    pub page_size: i32,
    ///  Page token, returned by a previous call, to request the next page of
    ///  results.
    #[prost(string, tag="3")]
    pub page_token: ::prost::alloc::string::String,
}
///  Message for response to listing Listings.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListListingsResponse {
    ///  The list of Listing.
    #[prost(message, repeated, tag="1")]
    pub listings: ::prost::alloc::vec::Vec<Listing>,
    ///  A token to request the next page of results.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
///  Message for getting a Listing.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetListingRequest {
    ///  Required. The resource name of the listing.
    ///  e.g. `projects/myproject/locations/US/dataExchanges/123/listings/456`.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
///  Message for creating a Listing.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateListingRequest {
    ///  Required. The parent resource path of the listing.
    ///  e.g. `projects/myproject/locations/US/dataExchanges/123`.
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    ///  Required. The ID of the Listing to create.
    ///  Must contain only Unicode letters, numbers (0-9), underscores (_).
    ///  Should not use characters that require URL-escaping, or characters
    ///  outside of ASCII, spaces.
    ///  Max length: 100 bytes.
    #[prost(string, tag="2")]
    pub listing_id: ::prost::alloc::string::String,
    ///  Required. The listing to create.
    #[prost(message, optional, tag="3")]
    pub listing: ::core::option::Option<Listing>,
}
///  Message for updating a Listing.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateListingRequest {
    ///  Required. Field mask is used to specify the fields to be overwritten in the
    ///  Listing resource by the update.
    ///  The fields specified in the update_mask are relative to the resource, not
    ///  the full request.
    #[prost(message, optional, tag="1")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    ///  Required. The listing to update.
    #[prost(message, optional, tag="2")]
    pub listing: ::core::option::Option<Listing>,
}
///  Message for deleting a Listing.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteListingRequest {
    ///  Required. Resource name of the listing to delete.
    ///  e.g. `projects/myproject/locations/US/dataExchanges/123/listings/456`.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
///  Message for subscribing a Listing.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeListingRequest {
    ///  Required. Resource name of the listing to subscribe to.
    ///  e.g. `projects/myproject/locations/US/dataExchanges/123/listings/456`.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    ///  Resulting destination of the listing subscribed to.
    #[prost(oneof="subscribe_listing_request::Destination", tags="3")]
    pub destination: ::core::option::Option<subscribe_listing_request::Destination>,
}
/// Nested message and enum types in `SubscribeListingRequest`.
pub mod subscribe_listing_request {
    ///  Resulting destination of the listing subscribed to.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Destination {
        ///  BigQuery destination dataset to create for the subscriber.
        #[prost(message, tag="3")]
        DestinationDataset(super::DestinationDataset),
    }
}
///  Message for response to subscribing a Listing.
///  Empty for now.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeListingResponse {
}
/// Generated client implementations.
pub mod analytics_hub_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// The AnalyticsHubService API facilitates data sharing within and across
    /// organizations. It allows data providers to publish Listings --- a
    /// discoverable and searchable SKU representing a dataset. Data consumers can
    /// subscribe to Listings. Upon subscription, AnalyticsHub provisions a "Linked
    /// Datasets" surfacing the data in the consumer's project.
    #[derive(Debug, Clone)]
    pub struct AnalyticsHubServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl AnalyticsHubServiceClient<tonic::transport::Channel> {
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
    impl<T> AnalyticsHubServiceClient<T>
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
        ) -> AnalyticsHubServiceClient<InterceptedService<T, F>>
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
            AnalyticsHubServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Lists DataExchanges in a given project and location.
        pub async fn list_data_exchanges(
            &mut self,
            request: impl tonic::IntoRequest<super::ListDataExchangesRequest>,
        ) -> Result<tonic::Response<super::ListDataExchangesResponse>, tonic::Status> {
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
                "/google.cloud.bigquery.dataexchange.v1beta1.AnalyticsHubService/ListDataExchanges",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists DataExchanges from projects in a given organization and location.
        pub async fn list_org_data_exchanges(
            &mut self,
            request: impl tonic::IntoRequest<super::ListOrgDataExchangesRequest>,
        ) -> Result<
            tonic::Response<super::ListOrgDataExchangesResponse>,
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
                "/google.cloud.bigquery.dataexchange.v1beta1.AnalyticsHubService/ListOrgDataExchanges",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets details of a single DataExchange.
        pub async fn get_data_exchange(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDataExchangeRequest>,
        ) -> Result<tonic::Response<super::DataExchange>, tonic::Status> {
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
                "/google.cloud.bigquery.dataexchange.v1beta1.AnalyticsHubService/GetDataExchange",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates a new DataExchange in a given project and location.
        pub async fn create_data_exchange(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateDataExchangeRequest>,
        ) -> Result<tonic::Response<super::DataExchange>, tonic::Status> {
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
                "/google.cloud.bigquery.dataexchange.v1beta1.AnalyticsHubService/CreateDataExchange",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates the parameters of a single DataExchange.
        pub async fn update_data_exchange(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateDataExchangeRequest>,
        ) -> Result<tonic::Response<super::DataExchange>, tonic::Status> {
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
                "/google.cloud.bigquery.dataexchange.v1beta1.AnalyticsHubService/UpdateDataExchange",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes a single DataExchange.
        pub async fn delete_data_exchange(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteDataExchangeRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
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
                "/google.cloud.bigquery.dataexchange.v1beta1.AnalyticsHubService/DeleteDataExchange",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists Listings in a given project and location.
        pub async fn list_listings(
            &mut self,
            request: impl tonic::IntoRequest<super::ListListingsRequest>,
        ) -> Result<tonic::Response<super::ListListingsResponse>, tonic::Status> {
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
                "/google.cloud.bigquery.dataexchange.v1beta1.AnalyticsHubService/ListListings",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets details of a single Listing.
        pub async fn get_listing(
            &mut self,
            request: impl tonic::IntoRequest<super::GetListingRequest>,
        ) -> Result<tonic::Response<super::Listing>, tonic::Status> {
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
                "/google.cloud.bigquery.dataexchange.v1beta1.AnalyticsHubService/GetListing",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates a new Listing in a given project and location.
        pub async fn create_listing(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateListingRequest>,
        ) -> Result<tonic::Response<super::Listing>, tonic::Status> {
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
                "/google.cloud.bigquery.dataexchange.v1beta1.AnalyticsHubService/CreateListing",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates the parameters of a single Listing.
        pub async fn update_listing(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateListingRequest>,
        ) -> Result<tonic::Response<super::Listing>, tonic::Status> {
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
                "/google.cloud.bigquery.dataexchange.v1beta1.AnalyticsHubService/UpdateListing",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes a single Listing, as long as there are no subscriptions
        /// associated with the source of this Listing.
        pub async fn delete_listing(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteListingRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
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
                "/google.cloud.bigquery.dataexchange.v1beta1.AnalyticsHubService/DeleteListing",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Subscribes to a single Listing.
        ///
        /// Data Exchange currently supports one type of Listing: a BigQuery dataset.
        /// Upon subscription to a Listing for a BigQuery dataset, Data Exchange
        /// creates a linked dataset in the subscriber's project.
        pub async fn subscribe_listing(
            &mut self,
            request: impl tonic::IntoRequest<super::SubscribeListingRequest>,
        ) -> Result<tonic::Response<super::SubscribeListingResponse>, tonic::Status> {
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
                "/google.cloud.bigquery.dataexchange.v1beta1.AnalyticsHubService/SubscribeListing",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets the IAM policy for a dataExchange or a listing.
        pub async fn get_iam_policy(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::super::iam::v1::GetIamPolicyRequest,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::super::iam::v1::Policy>,
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
                "/google.cloud.bigquery.dataexchange.v1beta1.AnalyticsHubService/GetIamPolicy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Sets the IAM policy for a dataExchange or a listing.
        pub async fn set_iam_policy(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::super::iam::v1::SetIamPolicyRequest,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::super::iam::v1::Policy>,
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
                "/google.cloud.bigquery.dataexchange.v1beta1.AnalyticsHubService/SetIamPolicy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Returns the permissions that a caller has on a specified dataExchange or
        /// listing.
        pub async fn test_iam_permissions(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::super::iam::v1::TestIamPermissionsRequest,
            >,
        ) -> Result<
            tonic::Response<
                super::super::super::super::super::iam::v1::TestIamPermissionsResponse,
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
                "/google.cloud.bigquery.dataexchange.v1beta1.AnalyticsHubService/TestIamPermissions",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
