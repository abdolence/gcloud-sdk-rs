/// A report submitted by a player about a playable location that is considered
/// inappropriate for use in the game.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerReport {
    /// Required. The name of the playable location.
    #[prost(string, tag = "1")]
    pub location_name: std::string::String,
    /// Required. One or more reasons why this playable location is considered bad.
    #[prost(
        enumeration = "player_report::BadLocationReason",
        repeated,
        packed = "false",
        tag = "2"
    )]
    pub reasons: ::std::vec::Vec<i32>,
    /// Required. A free-form description detailing why the playable location is
    /// considered bad.
    #[prost(string, tag = "3")]
    pub reason_details: std::string::String,
    /// Language code (in BCP-47 format) indicating the language of the freeform
    /// description provided in `reason_details`. Examples are "en", "en-US" or
    /// "ja-Latn". For more information, see
    /// http://www.unicode.org/reports/tr35/#Unicode_locale_identifier.
    #[prost(string, tag = "4")]
    pub language_code: std::string::String,
}
pub mod player_report {
    /// The reason why the playable location is considered bad.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum BadLocationReason {
        /// Unspecified reason. Do not use.
        Unspecified = 0,
        /// The reason isn't one of the reasons in this enumeration.
        Other = 1,
        /// The playable location isn't accessible to pedestrians. For example, if
        /// it's in the middle of a highway.
        NotPedestrianAccessible = 2,
        /// The playable location isn't open to the public. For example, a private
        /// office building.
        NotOpenToPublic = 4,
        /// The playable location is permanently closed. For example, when a business
        /// has been shut down.
        PermanentlyClosed = 5,
        /// The playable location is temporarily inaccessible. For example, when a
        /// business has closed for renovations.
        TemporarilyInaccessible = 6,
    }
}
/// Encapsulates impression event details.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Impression {
    /// Required. The name of the playable location.
    #[prost(string, tag = "1")]
    pub location_name: std::string::String,
    /// Required. The type of impression event.
    #[prost(enumeration = "impression::ImpressionType", tag = "2")]
    pub impression_type: i32,
    /// An arbitrary, developer-defined type identifier for each type of game
    /// object used in your game.
    ///
    /// Since players interact with differ types of game objects in different ways,
    /// this field allows you to segregate impression data by type for analysis.
    ///
    /// You should assign a unique `game_object_type` ID to represent a distinct
    /// type of game object in your game.
    ///
    /// For example, 1=monster location, 2=powerup location.
    #[prost(int32, tag = "4")]
    pub game_object_type: i32,
}
pub mod impression {
    /// The type of impression event.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ImpressionType {
        /// Unspecified type. Do not use.
        Unspecified = 0,
        /// The playable location was presented to a player.
        Presented = 1,
        /// A player interacted with the playable location.
        Interacted = 2,
    }
}
///
/// Life of a query:
///
/// - When a game starts in a new location, your game server issues a
/// [SamplePlayableLocations][google.maps.playablelocations.v3.PlayableLocations.SamplePlayableLocations]
/// request. The request specifies the S2 cell, and contains one or more
/// "criteria" for filtering:
///
/// - Criterion 0: i locations for long-lived bases, or level 0 monsters, or...
/// - Criterion 1: j locations for short-lived bases, or level 1 monsters, ...
/// - Criterion 2: k locations for random objects.
/// - etc (up to 5 criterion may be specified).
///
/// `PlayableLocationList` will then contain mutually
/// exclusive lists of `PlayableLocation` objects that satisfy each of
/// the criteria. Think of it as a collection of real-world locations that you
/// can then associate with your game state.
///
/// Note: These points are impermanent in nature. E.g, parks can close, and
/// places can be removed.
///
/// The response specifies how long you can expect the playable locations to
/// last. Once they expire, you should query the `samplePlayableLocations` API
/// again to get a fresh view of the real world.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SamplePlayableLocationsRequest {
    /// Required. Specifies the area to search within for playable locations.
    #[prost(message, optional, tag = "1")]
    pub area_filter: ::std::option::Option<sample::AreaFilter>,
    /// Required. Specifies one or more (up to 5) criteria for filtering the
    /// returned playable locations.
    #[prost(message, repeated, tag = "2")]
    pub criteria: ::std::vec::Vec<sample::Criterion>,
}
///
/// Response for the
/// [SamplePlayableLocations][google.maps.playablelocations.v3.PlayableLocations.SamplePlayableLocations]
/// method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SamplePlayableLocationsResponse {
    /// Each PlayableLocation object corresponds to a game_object_type specified
    /// in the request.
    #[prost(map = "int32, message", tag = "1")]
    pub locations_per_game_object_type:
        ::std::collections::HashMap<i32, sample::PlayableLocationList>,
    /// Required. Specifies the "time-to-live" for the set of playable locations. You can use
    /// this value to determine how long to cache the set of playable locations.
    /// After this length of time, your back-end game server should issue a new
    /// [SamplePlayableLocations][google.maps.playablelocations.v3.PlayableLocations.SamplePlayableLocations]
    /// request to get a fresh set of playable locations (because for example, they
    /// might have been removed, a park might have closed for the day, a
    /// business might have closed permanently).
    #[prost(message, optional, tag = "9")]
    pub ttl: ::std::option::Option<::prost_types::Duration>,
}
/// A request for logging your player's bad location reports.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LogPlayerReportsRequest {
    /// Required. Player reports. The maximum number of player reports that you can log at
    /// once is 50.
    #[prost(message, repeated, tag = "1")]
    pub player_reports: ::std::vec::Vec<PlayerReport>,
    /// Required. A string that uniquely identifies the log player reports request. This
    /// allows you to detect duplicate requests. We recommend that you use UUIDs
    /// for this value. The value must not exceed 50 characters.
    ///
    /// You should reuse the `request_id` only when retrying a request in the case
    /// of a failure. In that case, the request must be identical to the one that
    /// failed.
    #[prost(string, tag = "2")]
    pub request_id: std::string::String,
    /// Required. Information about the client device (for example, device model and
    /// operating system).
    #[prost(message, optional, tag = "3")]
    pub client_info: ::std::option::Option<super::super::unity::ClientInfo>,
}
/// A response for the [LogPlayerReports][google.maps.playablelocations.v3.PlayableLocations.LogPlayerReports]
/// method.
///
/// This method returns no data upon success.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LogPlayerReportsResponse {}
/// A request for logging impressions.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LogImpressionsRequest {
    /// Required. Impression event details. The maximum number of impression reports that you
    /// can log at once is 50.
    #[prost(message, repeated, tag = "1")]
    pub impressions: ::std::vec::Vec<Impression>,
    /// Required. A string that uniquely identifies the log impressions request. This allows
    /// you to detect duplicate requests. We recommend that you use UUIDs for this
    /// value. The value must not exceed 50 characters.
    ///
    /// You should reuse the `request_id` only when retrying a request in case of
    /// failure. In this case, the request must be identical to the one that
    /// failed.
    #[prost(string, tag = "2")]
    pub request_id: std::string::String,
    /// Required. Information about the client device. For example, device model and
    /// operating system.
    #[prost(message, optional, tag = "3")]
    pub client_info: ::std::option::Option<super::super::unity::ClientInfo>,
}
/// A response for the [LogImpressions][google.maps.playablelocations.v3.PlayableLocations.LogImpressions] method.
/// This method returns no data upon success.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LogImpressionsResponse {}
#[doc = r" Generated client implementations."]
pub mod playable_locations_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " The Playable Locations API for v3."]
    pub struct PlayableLocationsClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> PlayableLocationsClient<T>
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
        #[doc = " Returns a set of playable locations that lie within a specified area,"]
        #[doc = " that satisfy optional filter criteria."]
        #[doc = ""]
        #[doc = " Note: Identical `SamplePlayableLocations` requests can return different"]
        #[doc = " results as the state of the world changes over time."]
        pub async fn sample_playable_locations(
            &mut self,
            request: impl tonic::IntoRequest<super::SamplePlayableLocationsRequest>,
        ) -> Result<tonic::Response<super::SamplePlayableLocationsResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.maps.playablelocations.v3.PlayableLocations/SamplePlayableLocations",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Logs bad playable location reports submitted by players."]
        #[doc = ""]
        #[doc = " Reports are not partially saved; either all reports are saved and this"]
        #[doc = " request succeeds, or no reports are saved, and this request fails."]
        pub async fn log_player_reports(
            &mut self,
            request: impl tonic::IntoRequest<super::LogPlayerReportsRequest>,
        ) -> Result<tonic::Response<super::LogPlayerReportsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.maps.playablelocations.v3.PlayableLocations/LogPlayerReports",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Logs new events when playable locations are displayed, and when they are"]
        #[doc = " interacted with."]
        #[doc = ""]
        #[doc = " Impressions are not partially saved; either all impressions are saved and"]
        #[doc = " this request succeeds, or no impressions are saved, and this request fails."]
        pub async fn log_impressions(
            &mut self,
            request: impl tonic::IntoRequest<super::LogImpressionsRequest>,
        ) -> Result<tonic::Response<super::LogImpressionsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.maps.playablelocations.v3.PlayableLocations/LogImpressions",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for PlayableLocationsClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for PlayableLocationsClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "PlayableLocationsClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod playable_locations_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with PlayableLocationsServer."]
    #[async_trait]
    pub trait PlayableLocations: Send + Sync + 'static {
        #[doc = " Returns a set of playable locations that lie within a specified area,"]
        #[doc = " that satisfy optional filter criteria."]
        #[doc = ""]
        #[doc = " Note: Identical `SamplePlayableLocations` requests can return different"]
        #[doc = " results as the state of the world changes over time."]
        async fn sample_playable_locations(
            &self,
            request: tonic::Request<super::SamplePlayableLocationsRequest>,
        ) -> Result<tonic::Response<super::SamplePlayableLocationsResponse>, tonic::Status>;
        #[doc = " Logs bad playable location reports submitted by players."]
        #[doc = ""]
        #[doc = " Reports are not partially saved; either all reports are saved and this"]
        #[doc = " request succeeds, or no reports are saved, and this request fails."]
        async fn log_player_reports(
            &self,
            request: tonic::Request<super::LogPlayerReportsRequest>,
        ) -> Result<tonic::Response<super::LogPlayerReportsResponse>, tonic::Status>;
        #[doc = " Logs new events when playable locations are displayed, and when they are"]
        #[doc = " interacted with."]
        #[doc = ""]
        #[doc = " Impressions are not partially saved; either all impressions are saved and"]
        #[doc = " this request succeeds, or no impressions are saved, and this request fails."]
        async fn log_impressions(
            &self,
            request: tonic::Request<super::LogImpressionsRequest>,
        ) -> Result<tonic::Response<super::LogImpressionsResponse>, tonic::Status>;
    }
    #[doc = " The Playable Locations API for v3."]
    #[derive(Debug)]
    #[doc(hidden)]
    pub struct PlayableLocationsServer<T: PlayableLocations> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: PlayableLocations> PlayableLocationsServer<T> {
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
    impl<T, B> Service<http::Request<B>> for PlayableLocationsServer<T>
    where
        T: PlayableLocations,
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
                "/google.maps.playablelocations.v3.PlayableLocations/SamplePlayableLocations" => {
                    #[allow(non_camel_case_types)]
                    struct SamplePlayableLocationsSvc<T: PlayableLocations>(pub Arc<T>);
                    impl<T: PlayableLocations>
                        tonic::server::UnaryService<super::SamplePlayableLocationsRequest>
                        for SamplePlayableLocationsSvc<T>
                    {
                        type Response = super::SamplePlayableLocationsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SamplePlayableLocationsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.sample_playable_locations(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = SamplePlayableLocationsSvc(inner);
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
                "/google.maps.playablelocations.v3.PlayableLocations/LogPlayerReports" => {
                    #[allow(non_camel_case_types)]
                    struct LogPlayerReportsSvc<T: PlayableLocations>(pub Arc<T>);
                    impl<T: PlayableLocations>
                        tonic::server::UnaryService<super::LogPlayerReportsRequest>
                        for LogPlayerReportsSvc<T>
                    {
                        type Response = super::LogPlayerReportsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::LogPlayerReportsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.log_player_reports(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = LogPlayerReportsSvc(inner);
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
                "/google.maps.playablelocations.v3.PlayableLocations/LogImpressions" => {
                    #[allow(non_camel_case_types)]
                    struct LogImpressionsSvc<T: PlayableLocations>(pub Arc<T>);
                    impl<T: PlayableLocations>
                        tonic::server::UnaryService<super::LogImpressionsRequest>
                        for LogImpressionsSvc<T>
                    {
                        type Response = super::LogImpressionsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::LogImpressionsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.log_impressions(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = LogImpressionsSvc(inner);
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
    impl<T: PlayableLocations> Clone for PlayableLocationsServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: PlayableLocations> Clone for _Inner<T> {
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
