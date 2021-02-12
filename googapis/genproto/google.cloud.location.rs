/// The request message for [Locations.ListLocations][google.cloud.location.Locations.ListLocations].
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
/// The response message for [Locations.ListLocations][google.cloud.location.Locations.ListLocations].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListLocationsResponse {
    /// A list of locations that matches the specified filter in the request.
    #[prost(message, repeated, tag = "1")]
    pub locations: ::prost::alloc::vec::Vec<Location>,
    /// The standard List next-page token.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// The request message for [Locations.GetLocation][google.cloud.location.Locations.GetLocation].
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
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " An abstract interface that provides location-related information for"]
    #[doc = " a service. Service-specific metadata is provided through the"]
    #[doc = " [Location.metadata][google.cloud.location.Location.metadata] field."]
    pub struct LocationsClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> LocationsClient<T>
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
    impl<T: Clone> Clone for LocationsClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for LocationsClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "LocationsClient {{ ... }}")
        }
    }
}
