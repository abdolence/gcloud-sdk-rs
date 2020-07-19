/// A trace describes how long it takes for an application to perform an
/// operation. It consists of a set of spans, each of which represent a single
/// timed event within the operation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Trace {
    /// Project ID of the Cloud project where the trace data is stored.
    #[prost(string, tag = "1")]
    pub project_id: std::string::String,
    /// Globally unique identifier for the trace. This identifier is a 128-bit
    /// numeric value formatted as a 32-byte hex string. For example,
    /// `382d4f4c6b7bb2f4a972559d9085001d`.
    #[prost(string, tag = "2")]
    pub trace_id: std::string::String,
    /// Collection of spans in the trace.
    #[prost(message, repeated, tag = "3")]
    pub spans: ::std::vec::Vec<TraceSpan>,
}
/// List of new or updated traces.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Traces {
    /// List of traces.
    #[prost(message, repeated, tag = "1")]
    pub traces: ::std::vec::Vec<Trace>,
}
/// A span represents a single timed event within a trace. Spans can be nested
/// and form a trace tree. Often, a trace contains a root span that describes the
/// end-to-end latency of an operation and, optionally, one or more subspans for
/// its suboperations. Spans do not need to be contiguous. There may be gaps
/// between spans in a trace.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TraceSpan {
    /// Identifier for the span. Must be a 64-bit integer other than 0 and
    /// unique within a trace. For example, `2205310701640571284`.
    #[prost(fixed64, tag = "1")]
    pub span_id: u64,
    /// Distinguishes between spans generated in a particular context. For example,
    /// two spans with the same name may be distinguished using `RPC_CLIENT`
    /// and `RPC_SERVER` to identify queueing latency associated with the span.
    #[prost(enumeration = "trace_span::SpanKind", tag = "2")]
    pub kind: i32,
    /// Name of the span. Must be less than 128 bytes. The span name is sanitized
    /// and displayed in the Stackdriver Trace tool in the
    /// Google Cloud Platform Console.
    /// The name may be a method name or some other per-call site name.
    /// For the same executable and the same call point, a best practice is
    /// to use a consistent name, which makes it easier to correlate
    /// cross-trace spans.
    #[prost(string, tag = "3")]
    pub name: std::string::String,
    /// Start time of the span in nanoseconds from the UNIX epoch.
    #[prost(message, optional, tag = "4")]
    pub start_time: ::std::option::Option<::prost_types::Timestamp>,
    /// End time of the span in nanoseconds from the UNIX epoch.
    #[prost(message, optional, tag = "5")]
    pub end_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Optional. ID of the parent span, if any.
    #[prost(fixed64, tag = "6")]
    pub parent_span_id: u64,
    /// Collection of labels associated with the span. Label keys must be less than
    /// 128 bytes. Label values must be less than 16 kilobytes (10MB for
    /// `/stacktrace` values).
    ///
    /// Some predefined label keys exist, or you may create your own. When creating
    /// your own, we recommend the following formats:
    ///
    /// * `/category/product/key` for agents of well-known products (e.g.
    ///   `/db/mongodb/read_size`).
    /// * `short_host/path/key` for domain-specific keys (e.g.
    ///   `foo.com/myproduct/bar`)
    ///
    /// Predefined labels include:
    ///
    /// *   `/agent`
    /// *   `/component`
    /// *   `/error/message`
    /// *   `/error/name`
    /// *   `/http/client_city`
    /// *   `/http/client_country`
    /// *   `/http/client_protocol`
    /// *   `/http/client_region`
    /// *   `/http/host`
    /// *   `/http/method`
    /// *   `/http/path`
    /// *   `/http/redirected_url`
    /// *   `/http/request/size`
    /// *   `/http/response/size`
    /// *   `/http/route`
    /// *   `/http/status_code`
    /// *   `/http/url`
    /// *   `/http/user_agent`
    /// *   `/pid`
    /// *   `/stacktrace`
    /// *   `/tid`
    #[prost(map = "string, string", tag = "7")]
    pub labels: ::std::collections::HashMap<std::string::String, std::string::String>,
}
pub mod trace_span {
    /// Type of span. Can be used to specify additional relationships between spans
    /// in addition to a parent/child relationship.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum SpanKind {
        /// Unspecified.
        Unspecified = 0,
        /// Indicates that the span covers server-side handling of an RPC or other
        /// remote network request.
        RpcServer = 1,
        /// Indicates that the span covers the client-side wrapper around an RPC or
        /// other remote request.
        RpcClient = 2,
    }
}
/// The request message for the `ListTraces` method. All fields are required
/// unless specified.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTracesRequest {
    /// Required. ID of the Cloud project where the trace data is stored.
    #[prost(string, tag = "1")]
    pub project_id: std::string::String,
    /// Optional. Type of data returned for traces in the list. Default is
    /// `MINIMAL`.
    #[prost(enumeration = "list_traces_request::ViewType", tag = "2")]
    pub view: i32,
    /// Optional. Maximum number of traces to return. If not specified or <= 0, the
    /// implementation selects a reasonable value.  The implementation may
    /// return fewer traces than the requested page size.
    #[prost(int32, tag = "3")]
    pub page_size: i32,
    /// Token identifying the page of results to return. If provided, use the
    /// value of the `next_page_token` field from a previous request.
    #[prost(string, tag = "4")]
    pub page_token: std::string::String,
    /// Start of the time interval (inclusive) during which the trace data was
    /// collected from the application.
    #[prost(message, optional, tag = "5")]
    pub start_time: ::std::option::Option<::prost_types::Timestamp>,
    /// End of the time interval (inclusive) during which the trace data was
    /// collected from the application.
    #[prost(message, optional, tag = "6")]
    pub end_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Optional. A filter against labels for the request.
    ///
    /// By default, searches use prefix matching. To specify exact match, prepend
    /// a plus symbol (`+`) to the search term.
    /// Multiple terms are ANDed. Syntax:
    ///
    /// *   `root:NAME_PREFIX` or `NAME_PREFIX`: Return traces where any root
    ///     span starts with `NAME_PREFIX`.
    /// *   `+root:NAME` or `+NAME`: Return traces where any root span's name is
    ///     exactly `NAME`.
    /// *   `span:NAME_PREFIX`: Return traces where any span starts with
    ///     `NAME_PREFIX`.
    /// *   `+span:NAME`: Return traces where any span's name is exactly
    ///     `NAME`.
    /// *   `latency:DURATION`: Return traces whose overall latency is
    ///     greater or equal to than `DURATION`. Accepted units are nanoseconds
    ///     (`ns`), milliseconds (`ms`), and seconds (`s`). Default is `ms`. For
    ///     example, `latency:24ms` returns traces whose overall latency
    ///     is greater than or equal to 24 milliseconds.
    /// *   `label:LABEL_KEY`: Return all traces containing the specified
    ///     label key (exact match, case-sensitive) regardless of the key:value
    ///     pair's value (including empty values).
    /// *   `LABEL_KEY:VALUE_PREFIX`: Return all traces containing the specified
    ///     label key (exact match, case-sensitive) whose value starts with
    ///     `VALUE_PREFIX`. Both a key and a value must be specified.
    /// *   `+LABEL_KEY:VALUE`: Return all traces containing a key:value pair
    ///     exactly matching the specified text. Both a key and a value must be
    ///     specified.
    /// *   `method:VALUE`: Equivalent to `/http/method:VALUE`.
    /// *   `url:VALUE`: Equivalent to `/http/url:VALUE`.
    #[prost(string, tag = "7")]
    pub filter: std::string::String,
    /// Optional. Field used to sort the returned traces.
    /// Can be one of the following:
    ///
    /// *   `trace_id`
    /// *   `name` (`name` field of root span in the trace)
    /// *   `duration` (difference between `end_time` and `start_time` fields of
    ///      the root span)
    /// *   `start` (`start_time` field of the root span)
    ///
    /// Descending order can be specified by appending `desc` to the sort field
    /// (for example, `name desc`).
    ///
    /// Only one sort field is permitted.
    #[prost(string, tag = "8")]
    pub order_by: std::string::String,
}
pub mod list_traces_request {
    /// Type of data returned for traces in the list.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ViewType {
        /// Default is `MINIMAL` if unspecified.
        Unspecified = 0,
        /// Minimal view of the trace record that contains only the project
        /// and trace IDs.
        Minimal = 1,
        /// Root span view of the trace record that returns the root spans along
        /// with the minimal trace data.
        Rootspan = 2,
        /// Complete view of the trace record that contains the actual trace data.
        /// This is equivalent to calling the REST `get` or RPC `GetTrace` method
        /// using the ID of each listed trace.
        Complete = 3,
    }
}
/// The response message for the `ListTraces` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTracesResponse {
    /// List of trace records as specified by the view parameter.
    #[prost(message, repeated, tag = "1")]
    pub traces: ::std::vec::Vec<Trace>,
    /// If defined, indicates that there are more traces that match the request
    /// and that this value should be passed to the next request to continue
    /// retrieving additional traces.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
}
/// The request message for the `GetTrace` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTraceRequest {
    /// Required. ID of the Cloud project where the trace data is stored.
    #[prost(string, tag = "1")]
    pub project_id: std::string::String,
    /// Required. ID of the trace to return.
    #[prost(string, tag = "2")]
    pub trace_id: std::string::String,
}
/// The request message for the `PatchTraces` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PatchTracesRequest {
    /// Required. ID of the Cloud project where the trace data is stored.
    #[prost(string, tag = "1")]
    pub project_id: std::string::String,
    /// Required. The body of the message.
    #[prost(message, optional, tag = "2")]
    pub traces: ::std::option::Option<Traces>,
}
#[doc = r" Generated client implementations."]
pub mod trace_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " This file describes an API for collecting and viewing traces and spans"]
    #[doc = " within a trace.  A Trace is a collection of spans corresponding to a single"]
    #[doc = " operation or set of operations for an application. A span is an individual"]
    #[doc = " timed event which forms a node of the trace tree. Spans for a single trace"]
    #[doc = " may span multiple services."]
    pub struct TraceServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> TraceServiceClient<T>
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
        #[doc = " Returns of a list of traces that match the specified filter conditions."]
        pub async fn list_traces(
            &mut self,
            request: impl tonic::IntoRequest<super::ListTracesRequest>,
        ) -> Result<tonic::Response<super::ListTracesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.devtools.cloudtrace.v1.TraceService/ListTraces",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets a single trace by its ID."]
        pub async fn get_trace(
            &mut self,
            request: impl tonic::IntoRequest<super::GetTraceRequest>,
        ) -> Result<tonic::Response<super::Trace>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.devtools.cloudtrace.v1.TraceService/GetTrace",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Sends new traces to Stackdriver Trace or updates existing traces. If the ID"]
        #[doc = " of a trace that you send matches that of an existing trace, any fields"]
        #[doc = " in the existing trace and its spans are overwritten by the provided values,"]
        #[doc = " and any new fields provided are merged with the existing trace data. If the"]
        #[doc = " ID does not match, a new trace is created."]
        pub async fn patch_traces(
            &mut self,
            request: impl tonic::IntoRequest<super::PatchTracesRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.devtools.cloudtrace.v1.TraceService/PatchTraces",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for TraceServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for TraceServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "TraceServiceClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod trace_service_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with TraceServiceServer."]
    #[async_trait]
    pub trait TraceService: Send + Sync + 'static {
        #[doc = " Returns of a list of traces that match the specified filter conditions."]
        async fn list_traces(
            &self,
            request: tonic::Request<super::ListTracesRequest>,
        ) -> Result<tonic::Response<super::ListTracesResponse>, tonic::Status>;
        #[doc = " Gets a single trace by its ID."]
        async fn get_trace(
            &self,
            request: tonic::Request<super::GetTraceRequest>,
        ) -> Result<tonic::Response<super::Trace>, tonic::Status>;
        #[doc = " Sends new traces to Stackdriver Trace or updates existing traces. If the ID"]
        #[doc = " of a trace that you send matches that of an existing trace, any fields"]
        #[doc = " in the existing trace and its spans are overwritten by the provided values,"]
        #[doc = " and any new fields provided are merged with the existing trace data. If the"]
        #[doc = " ID does not match, a new trace is created."]
        async fn patch_traces(
            &self,
            request: tonic::Request<super::PatchTracesRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status>;
    }
    #[doc = " This file describes an API for collecting and viewing traces and spans"]
    #[doc = " within a trace.  A Trace is a collection of spans corresponding to a single"]
    #[doc = " operation or set of operations for an application. A span is an individual"]
    #[doc = " timed event which forms a node of the trace tree. Spans for a single trace"]
    #[doc = " may span multiple services."]
    #[derive(Debug)]
    pub struct TraceServiceServer<T: TraceService> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: TraceService> TraceServiceServer<T> {
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
    impl<T, B> Service<http::Request<B>> for TraceServiceServer<T>
    where
        T: TraceService,
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
                "/google.devtools.cloudtrace.v1.TraceService/ListTraces" => {
                    #[allow(non_camel_case_types)]
                    struct ListTracesSvc<T: TraceService>(pub Arc<T>);
                    impl<T: TraceService> tonic::server::UnaryService<super::ListTracesRequest> for ListTracesSvc<T> {
                        type Response = super::ListTracesResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListTracesRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).list_traces(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ListTracesSvc(inner);
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
                "/google.devtools.cloudtrace.v1.TraceService/GetTrace" => {
                    #[allow(non_camel_case_types)]
                    struct GetTraceSvc<T: TraceService>(pub Arc<T>);
                    impl<T: TraceService> tonic::server::UnaryService<super::GetTraceRequest> for GetTraceSvc<T> {
                        type Response = super::Trace;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetTraceRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_trace(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetTraceSvc(inner);
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
                "/google.devtools.cloudtrace.v1.TraceService/PatchTraces" => {
                    #[allow(non_camel_case_types)]
                    struct PatchTracesSvc<T: TraceService>(pub Arc<T>);
                    impl<T: TraceService> tonic::server::UnaryService<super::PatchTracesRequest> for PatchTracesSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PatchTracesRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).patch_traces(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = PatchTracesSvc(inner);
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
    impl<T: TraceService> Clone for TraceServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: TraceService> Clone for _Inner<T> {
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
