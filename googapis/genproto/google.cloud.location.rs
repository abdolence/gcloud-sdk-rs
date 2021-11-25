/// The request message for \[Locations.ListLocations][google.cloud.location.Locations.ListLocations\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListLocationsRequest {
    /// The resource that owns the locations collection, if applicable.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The standard list filter.
    #[prost(string, tag = "2")]
    pub filter: ::prost::alloc::string::String,
    /// The standard list page size.
    #[prost(int32, tag = "3")]
    pub page_size: i32,
    /// The standard list page token.
    #[prost(string, tag = "4")]
    pub page_token: ::prost::alloc::string::String,
}
/// The response message for \[Locations.ListLocations][google.cloud.location.Locations.ListLocations\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListLocationsResponse {
    /// A list of locations that matches the specified filter in the request.
    #[prost(message, repeated, tag = "1")]
    pub locations: ::prost::alloc::vec::Vec<Location>,
    /// The standard List next-page token.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// The request message for \[Locations.GetLocation][google.cloud.location.Locations.GetLocation\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLocationRequest {
    /// Resource name for the location.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// A resource that represents Google Cloud Platform location.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Location {
    /// Resource name for the location, which may vary between implementations.
    /// For example: `"projects/example-project/locations/us-east1"`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The canonical id for this location. For example: `"us-east1"`.
    #[prost(string, tag = "4")]
    pub location_id: ::prost::alloc::string::String,
    /// The friendly name for this location, typically a nearby city name.
    /// For example, "Tokyo".
    #[prost(string, tag = "5")]
    pub display_name: ::prost::alloc::string::String,
    /// Cross-service attributes for the location. For example
    ///
    ///     {"cloud.googleapis.com/region": "us-east1"}
    #[prost(map = "string, string", tag = "2")]
    pub labels:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Service-specific metadata. For example the available capacity at the given
    /// location.
    #[prost(message, optional, tag = "3")]
    pub metadata: ::core::option::Option<::prost_types::Any>,
}
#[doc = r" Generated client implementations."]
pub mod locations_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " An abstract interface that provides location-related information for"]
    #[doc = " a service. Service-specific metadata is provided through the"]
    #[doc = " [Location.metadata][google.cloud.location.Location.metadata] field."]
    #[derive(Debug, Clone)]
    pub struct LocationsClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> LocationsClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + Send + 'static,
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
        ) -> LocationsClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            LocationsClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Lists information about the supported locations for this service."]
        pub async fn list_locations(
            &mut self,
            request: impl tonic::IntoRequest<super::ListLocationsRequest>,
        ) -> Result<tonic::Response<super::ListLocationsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.location.Locations/ListLocations",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets information about a location."]
        pub async fn get_location(
            &mut self,
            request: impl tonic::IntoRequest<super::GetLocationRequest>,
        ) -> Result<tonic::Response<super::Location>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.location.Locations/GetLocation",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
