/// A request to the SnapToRoads method, requesting that a sequence of points be
/// snapped to road segments.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SnapToRoadsRequest {
    /// The path to be snapped as a series of lat, lng points. Specified as
    /// a string of the format: lat,lng|lat,lng|...
    #[prost(string, tag = "1")]
    pub path: std::string::String,
    /// Whether to interpolate the points to return full road geometry.
    #[prost(bool, tag = "2")]
    pub interpolate: bool,
    /// The asset ID of the asset to which this path relates. This is used for
    /// abuse detection purposes for clients with asset-based SKUs.
    #[prost(string, tag = "3")]
    pub asset_id: std::string::String,
    /// The type of travel being tracked. This will constrain the paths we snap to.
    #[prost(enumeration = "TravelMode", tag = "4")]
    pub travel_mode: i32,
}
/// A snapped point object, representing the result of snapping.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SnappedPoint {
    /// The lat,lng of the snapped location.
    #[prost(message, optional, tag = "1")]
    pub location: ::std::option::Option<super::super::super::r#type::LatLng>,
    /// The index into the original path of the equivalent pre-snapped point.
    /// This allows for identification of points which have been interpolated if
    /// this index is missing.
    #[prost(message, optional, tag = "2")]
    pub original_index: ::std::option::Option<u32>,
    /// The place ID for this snapped location (road segment). These are the same
    /// as are currently used by the Places API.
    #[prost(string, tag = "3")]
    pub place_id: std::string::String,
}
/// The response from the SnapToRoads method, returning a sequence of snapped
/// points.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SnapToRoadsResponse {
    /// A list of snapped points.
    #[prost(message, repeated, tag = "1")]
    pub snapped_points: ::std::vec::Vec<SnappedPoint>,
    /// User-visible warning message, if any, which can be shown alongside a valid
    /// result.
    #[prost(string, tag = "2")]
    pub warning_message: std::string::String,
}
/// A request to the ListNearestRoads method, requesting that a sequence of
/// points be snapped individually to the road segment that each is closest to.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListNearestRoadsRequest {
    /// The points to be snapped as a series of lat, lng points. Specified as
    /// a string of the format: lat,lng|lat,lng|...
    #[prost(string, tag = "1")]
    pub points: std::string::String,
    /// The type of travel being tracked. This will constrain the roads we snap to.
    #[prost(enumeration = "TravelMode", tag = "2")]
    pub travel_mode: i32,
}
/// The response from the ListNearestRoads method, returning a list of snapped
/// points.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListNearestRoadsResponse {
    /// A list of snapped points.
    #[prost(message, repeated, tag = "1")]
    pub snapped_points: ::std::vec::Vec<SnappedPoint>,
}
/// An enum representing the mode of travel used for snapping.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TravelMode {
    Unspecified = 0,
    Driving = 1,
    Cycling = 2,
    Walking = 3,
}
#[doc = r" Generated client implementations."]
pub mod roads_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    pub struct RoadsServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> RoadsServiceClient<T>
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
        #[doc = " This method takes a sequence of latitude,longitude points and snaps them to"]
        #[doc = " the most likely road segments. Optionally returns additional points giving"]
        #[doc = " the full road geometry. Also returns a place ID for each snapped point."]
        pub async fn snap_to_roads(
            &mut self,
            request: impl tonic::IntoRequest<super::SnapToRoadsRequest>,
        ) -> Result<tonic::Response<super::SnapToRoadsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.maps.roads.v1op.RoadsService/SnapToRoads",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " This method takes a list of latitude,longitude points and snaps them each"]
        #[doc = " to their nearest road. Also returns a place ID for each snapped point."]
        pub async fn list_nearest_roads(
            &mut self,
            request: impl tonic::IntoRequest<super::ListNearestRoadsRequest>,
        ) -> Result<tonic::Response<super::ListNearestRoadsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.maps.roads.v1op.RoadsService/ListNearestRoads",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for RoadsServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for RoadsServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "RoadsServiceClient {{ ... }}")
        }
    }
}
