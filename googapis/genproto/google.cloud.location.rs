/// The request message for
/// [Locations.ListLocations][google.cloud.location.Locations.ListLocations].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListLocationsRequest {
    /// The resource that owns the locations collection, if applicable.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// The standard list filter.
    #[prost(string, tag = "2")]
    pub filter: std::string::String,
    /// The standard list page size.
    #[prost(int32, tag = "3")]
    pub page_size: i32,
    /// The standard list page token.
    #[prost(string, tag = "4")]
    pub page_token: std::string::String,
}
/// The response message for
/// [Locations.ListLocations][google.cloud.location.Locations.ListLocations].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListLocationsResponse {
    /// A list of locations that matches the specified filter in the request.
    #[prost(message, repeated, tag = "1")]
    pub locations: ::std::vec::Vec<Location>,
    /// The standard List next-page token.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
}
/// The request message for
/// [Locations.GetLocation][google.cloud.location.Locations.GetLocation].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLocationRequest {
    /// Resource name for the location.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// A resource that represents Google Cloud Platform location.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Location {
    /// Resource name for the location, which may vary between implementations.
    /// For example: `"projects/example-project/locations/us-east1"`
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// The canonical id for this location. For example: `"us-east1"`.
    #[prost(string, tag = "4")]
    pub location_id: std::string::String,
    /// The friendly name for this location, typically a nearby city name.
    /// For example, "Tokyo".
    #[prost(string, tag = "5")]
    pub display_name: std::string::String,
    /// Cross-service attributes for the location. For example
    ///
    ///     {"cloud.googleapis.com/region": "us-east1"}
    #[prost(map = "string, string", tag = "2")]
    pub labels: ::std::collections::HashMap<std::string::String, std::string::String>,
    /// Service-specific metadata. For example the available capacity at the given
    /// location.
    #[prost(message, optional, tag = "3")]
    pub metadata: ::std::option::Option<::prost_types::Any>,
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
#[doc = r" Generated server implementations."]
pub mod locations_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with LocationsServer."]
    #[async_trait]
    pub trait Locations: Send + Sync + 'static {
        #[doc = " Lists information about the supported locations for this service."]
        async fn list_locations(
            &self,
            request: tonic::Request<super::ListLocationsRequest>,
        ) -> Result<tonic::Response<super::ListLocationsResponse>, tonic::Status>;
        #[doc = " Gets information about a location."]
        async fn get_location(
            &self,
            request: tonic::Request<super::GetLocationRequest>,
        ) -> Result<tonic::Response<super::Location>, tonic::Status>;
    }
    #[doc = " An abstract interface that provides location-related information for"]
    #[doc = " a service. Service-specific metadata is provided through the"]
    #[doc = " [Location.metadata][google.cloud.location.Location.metadata] field."]
    #[derive(Debug)]
    pub struct LocationsServer<T: Locations> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: Locations> LocationsServer<T> {
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
    impl<T, B> Service<http::Request<B>> for LocationsServer<T>
    where
        T: Locations,
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
                "/google.cloud.location.Locations/ListLocations" => {
                    #[allow(non_camel_case_types)]
                    struct ListLocationsSvc<T: Locations>(pub Arc<T>);
                    impl<T: Locations> tonic::server::UnaryService<super::ListLocationsRequest>
                        for ListLocationsSvc<T>
                    {
                        type Response = super::ListLocationsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListLocationsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).list_locations(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ListLocationsSvc(inner);
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
                "/google.cloud.location.Locations/GetLocation" => {
                    #[allow(non_camel_case_types)]
                    struct GetLocationSvc<T: Locations>(pub Arc<T>);
                    impl<T: Locations> tonic::server::UnaryService<super::GetLocationRequest> for GetLocationSvc<T> {
                        type Response = super::Location;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetLocationRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_location(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetLocationSvc(inner);
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
    impl<T: Locations> Clone for LocationsServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: Locations> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
}
