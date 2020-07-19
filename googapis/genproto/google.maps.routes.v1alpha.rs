#[doc = r" Generated client implementations."]
pub mod routes_alpha_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " The Routes Preferred API."]
    pub struct RoutesAlphaClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> RoutesAlphaClient<T>
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
        #[doc = " Returns the primary route along with optional alternate routes, given a set"]
        #[doc = " of terminal and intermediate waypoints."]
        #[doc = ""]
        #[doc = " **NOTE:** This method requires that you specify a response field mask in"]
        #[doc = " the input. You can provide the response field mask by using the URL"]
        #[doc = " parameter `$fields` or `fields`, or by using the HTTP/gRPC header"]
        #[doc = " `X-Goog-FieldMask` (see the [available URL parameters and"]
        #[doc = " headers](https://cloud.google.com/apis/docs/system-parameters). The value"]
        #[doc = " is a comma separated list of field paths. See this detailed documentation"]
        #[doc = " about [how to construct the field"]
        #[doc = " paths](https://github.com/protocolbuffers/protobuf/blob/master/src/google/protobuf/field_mask.proto)."]
        #[doc = ""]
        #[doc = " For example, in this method:"]
        #[doc = ""]
        #[doc = " * Field mask of all available fields (for manual inspection):"]
        #[doc = "   `X-Goog-FieldMask: *`"]
        #[doc = " * Field mask of Route-level duration, distance, and polyline (an example"]
        #[doc = " production setup):"]
        #[doc = "   `X-Goog-FieldMask:"]
        #[doc = "   routes.duration,routes.distanceMeters,routes.polyline.encodedPolyline`"]
        #[doc = ""]
        #[doc = " Google discourages the use of the wildcard (`*`) response field mask, or"]
        #[doc = " specifying the field mask at the top level (`routes`), because:"]
        #[doc = ""]
        #[doc = " * Selecting only the fields that you need helps our server save computation"]
        #[doc = " cycles, allowing us to return the result to you with a lower latency."]
        #[doc = " * Selecting only the fields that you need in your production job ensures"]
        #[doc = " stable latency performance. We might add more response fields in the"]
        #[doc = " future, and those new fields might require extra computation time. If you"]
        #[doc = " select all fields, or if you select all fields at the top level, then you"]
        #[doc = " might experience performance degradation because any new field we add will"]
        #[doc = " be automatically included in the response."]
        #[doc = " * Selecting only the fields that you need results in a smaller response"]
        #[doc = " size, and thus higher network throughput."]
        pub async fn compute_routes(
            &mut self,
            request: impl tonic::IntoRequest<super::super::v1::ComputeRoutesRequest>,
        ) -> Result<tonic::Response<super::super::v1::ComputeRoutesResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.maps.routes.v1alpha.RoutesAlpha/ComputeRoutes",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Takes in a list of origins and destinations and returns a stream containing"]
        #[doc = " route information for each combination of origin and destination."]
        #[doc = ""]
        #[doc = " **NOTE:** This method requires that you specify a response field mask in"]
        #[doc = " the input. You can provide the response field mask by using the URL"]
        #[doc = " parameter `$fields` or `fields`, or by using the HTTP/gRPC header"]
        #[doc = " `X-Goog-FieldMask` (see the [available URL parameters and"]
        #[doc = " headers](https://cloud.google.com/apis/docs/system-parameters). The value"]
        #[doc = " is a comma separated list of field paths. See this detailed documentation"]
        #[doc = " about [how to construct the field"]
        #[doc = " paths](https://github.com/protocolbuffers/protobuf/blob/master/src/google/protobuf/field_mask.proto)."]
        #[doc = ""]
        #[doc = " For example, in this method:"]
        #[doc = ""]
        #[doc = " * Field mask of all available fields (for manual inspection):"]
        #[doc = "   `X-Goog-FieldMask: *`"]
        #[doc = " * Field mask of route durations, distances, element status, and element"]
        #[doc = "   indices (an example production setup):"]
        #[doc = "   `X-Goog-FieldMask:"]
        #[doc = "   originIndex,destinationIndex,status,distanceMeters,duration`"]
        #[doc = ""]
        #[doc = " Google discourages the use of the wildcard (`*`) response field mask,"]
        #[doc = " because:"]
        #[doc = ""]
        #[doc = " * Selecting only the fields that you need helps our server save computation"]
        #[doc = " cycles, allowing us to return the result to you with a lower latency."]
        #[doc = " * Selecting only the fields that you need in your production job ensures"]
        #[doc = " stable latency performance. We might add more response fields in the"]
        #[doc = " future, and those new fields might require extra computation time. If you"]
        #[doc = " select all fields, or if you select all fields at the top level, then you"]
        #[doc = " might experience performance degradation because any new field we add will"]
        #[doc = " be automatically included in the response."]
        #[doc = " * Selecting only the fields that you need results in a smaller response"]
        #[doc = " size, and thus higher network throughput."]
        pub async fn compute_route_matrix(
            &mut self,
            request: impl tonic::IntoRequest<super::super::v1::ComputeRouteMatrixRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::super::v1::RouteMatrixElement>>,
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
                "/google.maps.routes.v1alpha.RoutesAlpha/ComputeRouteMatrix",
            );
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
    }
    impl<T: Clone> Clone for RoutesAlphaClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for RoutesAlphaClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "RoutesAlphaClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod routes_alpha_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with RoutesAlphaServer."]
    #[async_trait]
    pub trait RoutesAlpha: Send + Sync + 'static {
        #[doc = " Returns the primary route along with optional alternate routes, given a set"]
        #[doc = " of terminal and intermediate waypoints."]
        #[doc = ""]
        #[doc = " **NOTE:** This method requires that you specify a response field mask in"]
        #[doc = " the input. You can provide the response field mask by using the URL"]
        #[doc = " parameter `$fields` or `fields`, or by using the HTTP/gRPC header"]
        #[doc = " `X-Goog-FieldMask` (see the [available URL parameters and"]
        #[doc = " headers](https://cloud.google.com/apis/docs/system-parameters). The value"]
        #[doc = " is a comma separated list of field paths. See this detailed documentation"]
        #[doc = " about [how to construct the field"]
        #[doc = " paths](https://github.com/protocolbuffers/protobuf/blob/master/src/google/protobuf/field_mask.proto)."]
        #[doc = ""]
        #[doc = " For example, in this method:"]
        #[doc = ""]
        #[doc = " * Field mask of all available fields (for manual inspection):"]
        #[doc = "   `X-Goog-FieldMask: *`"]
        #[doc = " * Field mask of Route-level duration, distance, and polyline (an example"]
        #[doc = " production setup):"]
        #[doc = "   `X-Goog-FieldMask:"]
        #[doc = "   routes.duration,routes.distanceMeters,routes.polyline.encodedPolyline`"]
        #[doc = ""]
        #[doc = " Google discourages the use of the wildcard (`*`) response field mask, or"]
        #[doc = " specifying the field mask at the top level (`routes`), because:"]
        #[doc = ""]
        #[doc = " * Selecting only the fields that you need helps our server save computation"]
        #[doc = " cycles, allowing us to return the result to you with a lower latency."]
        #[doc = " * Selecting only the fields that you need in your production job ensures"]
        #[doc = " stable latency performance. We might add more response fields in the"]
        #[doc = " future, and those new fields might require extra computation time. If you"]
        #[doc = " select all fields, or if you select all fields at the top level, then you"]
        #[doc = " might experience performance degradation because any new field we add will"]
        #[doc = " be automatically included in the response."]
        #[doc = " * Selecting only the fields that you need results in a smaller response"]
        #[doc = " size, and thus higher network throughput."]
        async fn compute_routes(
            &self,
            request: tonic::Request<super::super::v1::ComputeRoutesRequest>,
        ) -> Result<tonic::Response<super::super::v1::ComputeRoutesResponse>, tonic::Status>;
        #[doc = "Server streaming response type for the ComputeRouteMatrix method."]
        type ComputeRouteMatrixStream: Stream<Item = Result<super::super::v1::RouteMatrixElement, tonic::Status>>
            + Send
            + Sync
            + 'static;
        #[doc = " Takes in a list of origins and destinations and returns a stream containing"]
        #[doc = " route information for each combination of origin and destination."]
        #[doc = ""]
        #[doc = " **NOTE:** This method requires that you specify a response field mask in"]
        #[doc = " the input. You can provide the response field mask by using the URL"]
        #[doc = " parameter `$fields` or `fields`, or by using the HTTP/gRPC header"]
        #[doc = " `X-Goog-FieldMask` (see the [available URL parameters and"]
        #[doc = " headers](https://cloud.google.com/apis/docs/system-parameters). The value"]
        #[doc = " is a comma separated list of field paths. See this detailed documentation"]
        #[doc = " about [how to construct the field"]
        #[doc = " paths](https://github.com/protocolbuffers/protobuf/blob/master/src/google/protobuf/field_mask.proto)."]
        #[doc = ""]
        #[doc = " For example, in this method:"]
        #[doc = ""]
        #[doc = " * Field mask of all available fields (for manual inspection):"]
        #[doc = "   `X-Goog-FieldMask: *`"]
        #[doc = " * Field mask of route durations, distances, element status, and element"]
        #[doc = "   indices (an example production setup):"]
        #[doc = "   `X-Goog-FieldMask:"]
        #[doc = "   originIndex,destinationIndex,status,distanceMeters,duration`"]
        #[doc = ""]
        #[doc = " Google discourages the use of the wildcard (`*`) response field mask,"]
        #[doc = " because:"]
        #[doc = ""]
        #[doc = " * Selecting only the fields that you need helps our server save computation"]
        #[doc = " cycles, allowing us to return the result to you with a lower latency."]
        #[doc = " * Selecting only the fields that you need in your production job ensures"]
        #[doc = " stable latency performance. We might add more response fields in the"]
        #[doc = " future, and those new fields might require extra computation time. If you"]
        #[doc = " select all fields, or if you select all fields at the top level, then you"]
        #[doc = " might experience performance degradation because any new field we add will"]
        #[doc = " be automatically included in the response."]
        #[doc = " * Selecting only the fields that you need results in a smaller response"]
        #[doc = " size, and thus higher network throughput."]
        async fn compute_route_matrix(
            &self,
            request: tonic::Request<super::super::v1::ComputeRouteMatrixRequest>,
        ) -> Result<tonic::Response<Self::ComputeRouteMatrixStream>, tonic::Status>;
    }
    #[doc = " The Routes Preferred API."]
    #[derive(Debug)]
    pub struct RoutesAlphaServer<T: RoutesAlpha> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: RoutesAlpha> RoutesAlphaServer<T> {
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
    impl<T, B> Service<http::Request<B>> for RoutesAlphaServer<T>
    where
        T: RoutesAlpha,
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
                "/google.maps.routes.v1alpha.RoutesAlpha/ComputeRoutes" => {
                    #[allow(non_camel_case_types)]
                    struct ComputeRoutesSvc<T: RoutesAlpha>(pub Arc<T>);
                    impl<T: RoutesAlpha>
                        tonic::server::UnaryService<super::super::v1::ComputeRoutesRequest>
                        for ComputeRoutesSvc<T>
                    {
                        type Response = super::super::v1::ComputeRoutesResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::v1::ComputeRoutesRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).compute_routes(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ComputeRoutesSvc(inner);
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
                "/google.maps.routes.v1alpha.RoutesAlpha/ComputeRouteMatrix" => {
                    #[allow(non_camel_case_types)]
                    struct ComputeRouteMatrixSvc<T: RoutesAlpha>(pub Arc<T>);
                    impl<T: RoutesAlpha>
                        tonic::server::ServerStreamingService<
                            super::super::v1::ComputeRouteMatrixRequest,
                        > for ComputeRouteMatrixSvc<T>
                    {
                        type Response = super::super::v1::RouteMatrixElement;
                        type ResponseStream = T::ComputeRouteMatrixStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::super::v1::ComputeRouteMatrixRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).compute_route_matrix(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1;
                        let inner = inner.0;
                        let method = ComputeRouteMatrixSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.server_streaming(method, req).await;
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
    impl<T: RoutesAlpha> Clone for RoutesAlphaServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: RoutesAlpha> Clone for _Inner<T> {
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
