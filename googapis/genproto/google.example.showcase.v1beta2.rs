#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RepeatRequest {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub info: ::core::option::Option<ComplianceData>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RepeatResponse {
    #[prost(message, optional, tag = "1")]
    pub info: ::core::option::Option<ComplianceData>,
}
/// ComplianceData is a message used for testing REST transcoding of
/// different data types.
/// scalar types
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ComplianceData {
    #[prost(string, tag = "1")]
    pub f_string: ::prost::alloc::string::String,
    #[prost(int32, tag = "2")]
    pub f_int32: i32,
    #[prost(sint32, tag = "3")]
    pub f_sint32: i32,
    #[prost(sfixed32, tag = "4")]
    pub f_sfixed32: i32,
    #[prost(uint32, tag = "5")]
    pub f_uint32: u32,
    #[prost(fixed32, tag = "6")]
    pub f_fixed32: u32,
    #[prost(int64, tag = "7")]
    pub f_int64: i64,
    #[prost(sint64, tag = "8")]
    pub f_sint64: i64,
    #[prost(sfixed64, tag = "9")]
    pub f_sfixed64: i64,
    #[prost(uint64, tag = "10")]
    pub f_uint64: u64,
    #[prost(fixed64, tag = "11")]
    pub f_fixed64: u64,
    #[prost(double, tag = "12")]
    pub f_double: f64,
    #[prost(float, tag = "13")]
    pub f_float: f32,
    #[prost(bool, tag = "14")]
    pub f_bool: bool,
    #[prost(bytes = "vec", tag = "15")]
    pub f_bytes: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "16")]
    pub f_child: ::core::option::Option<ComplianceDataChild>,
    #[prost(string, optional, tag = "17")]
    pub p_string: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "18")]
    pub p_int32: ::core::option::Option<i32>,
    #[prost(double, optional, tag = "19")]
    pub p_double: ::core::option::Option<f64>,
    #[prost(bool, optional, tag = "20")]
    pub p_bool: ::core::option::Option<bool>,
    #[prost(message, optional, tag = "21")]
    pub p_child: ::core::option::Option<ComplianceDataChild>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ComplianceDataChild {
    #[prost(string, tag = "1")]
    pub f_string: ::prost::alloc::string::String,
    #[prost(float, tag = "2")]
    pub f_float: f32,
    #[prost(double, tag = "3")]
    pub f_double: f64,
    #[prost(bool, tag = "4")]
    pub f_bool: bool,
    #[prost(message, optional, tag = "5")]
    pub f_child: ::core::option::Option<ComplianceDataGrandchild>,
    #[prost(string, optional, tag = "6")]
    pub p_string: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(float, optional, tag = "7")]
    pub p_float: ::core::option::Option<f32>,
    #[prost(double, optional, tag = "8")]
    pub p_double: ::core::option::Option<f64>,
    #[prost(bool, optional, tag = "9")]
    pub p_bool: ::core::option::Option<bool>,
    #[prost(message, optional, tag = "10")]
    pub p_child: ::core::option::Option<ComplianceDataGrandchild>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ComplianceDataGrandchild {
    #[prost(string, tag = "1")]
    pub f_string: ::prost::alloc::string::String,
    #[prost(double, tag = "2")]
    pub f_double: f64,
    #[prost(bool, tag = "3")]
    pub f_bool: bool,
}
#[doc = r" Generated client implementations."]
pub mod compliance_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " This service is used to test that GAPICs can transcode proto3 requests to"]
    #[doc = " REST format correctly for various types of HTTP annotations..."]
    #[derive(Debug, Clone)]
    pub struct ComplianceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> ComplianceClient<T>
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
        ) -> ComplianceClient<InterceptedService<T, F>>
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
            ComplianceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " This method echoes the ComplianceData request. This method exercises"]
        #[doc = " sending the entire request object in the REST body."]
        pub async fn repeat_data_body(
            &mut self,
            request: impl tonic::IntoRequest<super::RepeatRequest>,
        ) -> Result<tonic::Response<super::RepeatResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.example.showcase.v1beta2.Compliance/RepeatDataBody",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " This method echoes the ComplianceData request. This method exercises"]
        #[doc = " sending the a message-type field in the REST body. Per AIP-127, only"]
        #[doc = " top-level, non-repeated fields can be sent this way."]
        pub async fn repeat_data_body_info(
            &mut self,
            request: impl tonic::IntoRequest<super::RepeatRequest>,
        ) -> Result<tonic::Response<super::RepeatResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.example.showcase.v1beta2.Compliance/RepeatDataBodyInfo",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " This method echoes the ComplianceData request. This method exercises"]
        #[doc = " sending all request fields as query parameters."]
        pub async fn repeat_data_query(
            &mut self,
            request: impl tonic::IntoRequest<super::RepeatRequest>,
        ) -> Result<tonic::Response<super::RepeatResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.example.showcase.v1beta2.Compliance/RepeatDataQuery",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " This method echoes the ComplianceData request. This method exercises"]
        #[doc = " sending some parameters as \"simple\" path variables (i.e., of the form"]
        #[doc = " \"/bar/{foo}\" rather than \"/{foo=bar/*}\"), and the rest as query parameters."]
        pub async fn repeat_data_simple_path(
            &mut self,
            request: impl tonic::IntoRequest<super::RepeatRequest>,
        ) -> Result<tonic::Response<super::RepeatResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.example.showcase.v1beta2.Compliance/RepeatDataSimplePath",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Same as RepeatDataSimplePath, but with a path resource."]
        pub async fn repeat_data_path_resource(
            &mut self,
            request: impl tonic::IntoRequest<super::RepeatRequest>,
        ) -> Result<tonic::Response<super::RepeatResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.example.showcase.v1beta2.Compliance/RepeatDataPathResource",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Same as RepeatDataSimplePath, but with a trailing resource."]
        pub async fn repeat_data_path_trailing_resource(
            &mut self,
            request: impl tonic::IntoRequest<super::RepeatRequest>,
        ) -> Result<tonic::Response<super::RepeatResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.example.showcase.v1beta2.Compliance/RepeatDataPathTrailingResource",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// The request message used for the Echo, Collect and Chat methods.
/// If content or opt are set in this message then the request will succeed.
/// If status is set in this message then the status will be returned as an
/// error.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EchoRequest {
    /// The severity to be echoed by the server.
    #[prost(enumeration = "Severity", tag = "3")]
    pub severity: i32,
    /// The response contents.
    #[prost(oneof = "echo_request::Response", tags = "1, 2")]
    pub response: ::core::option::Option<echo_request::Response>,
}
/// Nested message and enum types in `EchoRequest`.
pub mod echo_request {
    /// The response contents.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Response {
        /// The content to be echoed by the server.
        #[prost(string, tag = "1")]
        Content(::prost::alloc::string::String),
        /// The error to be thrown by the server.
        #[prost(message, tag = "2")]
        Error(super::super::super::super::rpc::Status),
    }
}
/// The response message for the Echo methods.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EchoResponse {
    /// The content specified in the request.
    #[prost(string, tag = "1")]
    pub content: ::prost::alloc::string::String,
    /// The severity specified in the request.
    #[prost(enumeration = "Severity", tag = "2")]
    pub severity: i32,
}
/// The request message for the Expand method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExpandRequest {
    /// The content that will be split into words and returned on the stream.
    #[prost(string, tag = "1")]
    pub content: ::prost::alloc::string::String,
    /// The error that is thrown after all words are sent on the stream.
    #[prost(message, optional, tag = "2")]
    pub error: ::core::option::Option<super::super::super::rpc::Status>,
}
/// The request for the PagedExpand method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PagedExpandRequest {
    /// Required. The string to expand.
    #[prost(string, tag = "1")]
    pub content: ::prost::alloc::string::String,
    /// The amount of words to returned in each page.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The position of the page to be returned.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// The response for the PagedExpand method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PagedExpandResponse {
    /// The words that were expanded.
    #[prost(message, repeated, tag = "1")]
    pub responses: ::prost::alloc::vec::Vec<EchoResponse>,
    /// The next page token.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// The request for Wait method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WaitRequest {
    /// The ending time or duration.
    #[prost(oneof = "wait_request::End", tags = "1, 4")]
    pub end: ::core::option::Option<wait_request::End>,
    /// The response.
    #[prost(oneof = "wait_request::Response", tags = "2, 3")]
    pub response: ::core::option::Option<wait_request::Response>,
}
/// Nested message and enum types in `WaitRequest`.
pub mod wait_request {
    /// The ending time or duration.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum End {
        /// The time that this operation will complete.
        #[prost(message, tag = "1")]
        EndTime(::prost_types::Timestamp),
        /// The duration of this operation.
        #[prost(message, tag = "4")]
        Ttl(::prost_types::Duration),
    }
    /// The response.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Response {
        /// The error that will be returned by the server. If this code is specified
        /// to be the OK rpc code, an empty response will be returned.
        #[prost(message, tag = "2")]
        Error(super::super::super::super::rpc::Status),
        /// The response to be returned on operation completion.
        #[prost(message, tag = "3")]
        Success(super::WaitResponse),
    }
}
/// The result of the Wait operation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WaitResponse {
    /// This content of the result.
    #[prost(string, tag = "1")]
    pub content: ::prost::alloc::string::String,
}
/// The request for Block method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockRequest {
    /// The amount of time to block before returning a response.
    #[prost(message, optional, tag = "1")]
    pub response_delay: ::core::option::Option<::prost_types::Duration>,
    /// The response.
    #[prost(oneof = "block_request::Response", tags = "2, 3")]
    pub response: ::core::option::Option<block_request::Response>,
}
/// Nested message and enum types in `BlockRequest`.
pub mod block_request {
    /// The response.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Response {
        /// The error that will be returned by the server. If this code is specified
        /// to be the OK rpc code, an empty response will be returned.
        #[prost(message, tag = "2")]
        Error(super::super::super::super::rpc::Status),
        /// The response to be returned that will signify successful method call.
        #[prost(message, tag = "3")]
        Success(super::BlockResponse),
    }
}
/// The response for Block method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockResponse {
    /// This content can contain anything, the server will not depend on a value
    /// here.
    #[prost(string, tag = "1")]
    pub content: ::prost::alloc::string::String,
}
/// A severity enum used to test enum capabilities in GAPIC surfaces.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Severity {
    /// The severity is unnecessary.
    Unnecessary = 0,
    /// The severity is necessary.
    Necessary = 1,
    /// Urgent.
    Urgent = 2,
    /// Critical.
    Critical = 3,
}
#[doc = r" Generated client implementations."]
pub mod echo_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " This service is used showcase the four main types of rpcs - unary, server"]
    #[doc = " side streaming, client side streaming, and bidirectional streaming. This"]
    #[doc = " service also exposes methods that explicitly implement server delay, and"]
    #[doc = " paginated calls. Set the 'showcase-trailer' metadata key on any method"]
    #[doc = " to have the values echoed in the response trailers."]
    #[derive(Debug, Clone)]
    pub struct EchoClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> EchoClient<T>
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
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> EchoClient<InterceptedService<T, F>>
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
            EchoClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " This method simply echoes the request. This method showcases unary RPCs."]
        pub async fn echo(
            &mut self,
            request: impl tonic::IntoRequest<super::EchoRequest>,
        ) -> Result<tonic::Response<super::EchoResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/google.example.showcase.v1beta2.Echo/Echo");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " This method splits the given content into words and will pass each word"]
        #[doc = " back through the stream. This method showcases server-side streaming RPCs."]
        pub async fn expand(
            &mut self,
            request: impl tonic::IntoRequest<super::ExpandRequest>,
        ) -> Result<tonic::Response<tonic::codec::Streaming<super::EchoResponse>>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.example.showcase.v1beta2.Echo/Expand",
            );
            self.inner.server_streaming(request.into_request(), path, codec).await
        }
        #[doc = " This method will collect the words given to it. When the stream is closed"]
        #[doc = " by the client, this method will return the a concatenation of the strings"]
        #[doc = " passed to it. This method showcases client-side streaming RPCs."]
        pub async fn collect(
            &mut self,
            request: impl tonic::IntoStreamingRequest<Message = super::EchoRequest>,
        ) -> Result<tonic::Response<super::EchoResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.example.showcase.v1beta2.Echo/Collect",
            );
            self.inner.client_streaming(request.into_streaming_request(), path, codec).await
        }
        #[doc = " This method, upon receiving a request on the stream, will pass the same"]
        #[doc = " content back on the stream. This method showcases bidirectional"]
        #[doc = " streaming RPCs."]
        pub async fn chat(
            &mut self,
            request: impl tonic::IntoStreamingRequest<Message = super::EchoRequest>,
        ) -> Result<tonic::Response<tonic::codec::Streaming<super::EchoResponse>>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/google.example.showcase.v1beta2.Echo/Chat");
            self.inner.streaming(request.into_streaming_request(), path, codec).await
        }
        #[doc = " This is similar to the Expand method but instead of returning a stream of"]
        #[doc = " expanded words, this method returns a paged list of expanded words."]
        pub async fn paged_expand(
            &mut self,
            request: impl tonic::IntoRequest<super::PagedExpandRequest>,
        ) -> Result<tonic::Response<super::PagedExpandResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.example.showcase.v1beta2.Echo/PagedExpand",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " This method will wait for the requested amount of time and then return."]
        #[doc = " This method showcases how a client handles a request timeout."]
        pub async fn wait(
            &mut self,
            request: impl tonic::IntoRequest<super::WaitRequest>,
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
            let path =
                http::uri::PathAndQuery::from_static("/google.example.showcase.v1beta2.Echo/Wait");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " This method will block (wait) for the requested amount of time"]
        #[doc = " and then return the response or error."]
        #[doc = " This method showcases how a client handles delays or retries."]
        pub async fn block(
            &mut self,
            request: impl tonic::IntoRequest<super::BlockRequest>,
        ) -> Result<tonic::Response<super::BlockResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/google.example.showcase.v1beta2.Echo/Block");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Sequence {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Sequence of responses to return in order for each attempt. If empty, the
    /// default response is an immediate OK.
    #[prost(message, repeated, tag = "2")]
    pub responses: ::prost::alloc::vec::Vec<sequence::Response>,
}
/// Nested message and enum types in `Sequence`.
pub mod sequence {
    /// A server response to an RPC Attempt in a sequence.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Response {
        /// The status to return for an individual attempt.
        #[prost(message, optional, tag = "1")]
        pub status: ::core::option::Option<super::super::super::super::rpc::Status>,
        /// The amount of time to delay sending the response.
        #[prost(message, optional, tag = "2")]
        pub delay: ::core::option::Option<::prost_types::Duration>,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SequenceReport {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The set of RPC attempts received by the server for a Sequence.
    #[prost(message, repeated, tag = "2")]
    pub attempts: ::prost::alloc::vec::Vec<sequence_report::Attempt>,
}
/// Nested message and enum types in `SequenceReport`.
pub mod sequence_report {
    /// Contains metrics on individual RPC Attempts in a sequence.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Attempt {
        /// The attempt number - starting at 0.
        #[prost(int32, tag = "1")]
        pub attempt_number: i32,
        /// The deadline dictated by the attempt to the server.
        #[prost(message, optional, tag = "2")]
        pub attempt_deadline: ::core::option::Option<::prost_types::Timestamp>,
        /// The time that the server responded to the RPC attempt. Used for
        /// calculating attempt_delay.
        #[prost(message, optional, tag = "3")]
        pub response_time: ::core::option::Option<::prost_types::Timestamp>,
        /// The server perceived delay between sending the last response and
        /// receiving this attempt. Used for validating attempt delay backoff.
        #[prost(message, optional, tag = "4")]
        pub attempt_delay: ::core::option::Option<::prost_types::Duration>,
        /// The status returned to the attempt.
        #[prost(message, optional, tag = "5")]
        pub status: ::core::option::Option<super::super::super::super::rpc::Status>,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateSequenceRequest {
    #[prost(message, optional, tag = "1")]
    pub sequence: ::core::option::Option<Sequence>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AttemptSequenceRequest {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSequenceReportRequest {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
#[doc = r" Generated client implementations."]
pub mod sequence_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct SequenceServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> SequenceServiceClient<T>
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
        ) -> SequenceServiceClient<InterceptedService<T, F>>
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
            SequenceServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Creates a sequence."]
        pub async fn create_sequence(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateSequenceRequest>,
        ) -> Result<tonic::Response<super::Sequence>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.example.showcase.v1beta2.SequenceService/CreateSequence",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Retrieves a sequence."]
        pub async fn get_sequence_report(
            &mut self,
            request: impl tonic::IntoRequest<super::GetSequenceReportRequest>,
        ) -> Result<tonic::Response<super::SequenceReport>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.example.showcase.v1beta2.SequenceService/GetSequenceReport",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Attempts a sequence."]
        pub async fn attempt_sequence(
            &mut self,
            request: impl tonic::IntoRequest<super::AttemptSequenceRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.example.showcase.v1beta2.SequenceService/AttemptSequence",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// A session is a suite of tests, generally being made in the context
/// of testing code generation.
///
/// A session defines tests it may expect, based on which version of the
/// code generation spec is in use.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Session {
    /// The name of the session. The ID must conform to ^\[a-z\]+$
    /// If this is not provided, Showcase chooses one at random.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The version this session is using.
    #[prost(enumeration = "session::Version", tag = "2")]
    pub version: i32,
}
/// Nested message and enum types in `Session`.
pub mod session {
    /// The specification versions understood by Showcase.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Version {
        /// Unspecified version. If passed on creation, the session will default
        /// to using the latest stable release.
        Unspecified = 0,
        /// The latest v1. Currently, this is v1.0.
        V1Latest = 1,
        /// v1.0. (Until the spec is "GA", this will be a moving target.)
        V10 = 2,
    }
}
/// The request for the CreateSession method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateSessionRequest {
    /// The session to be created.
    /// Sessions are immutable once they are created (although they can
    /// be deleted).
    #[prost(message, optional, tag = "1")]
    pub session: ::core::option::Option<Session>,
}
/// The request for the GetSession method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSessionRequest {
    /// The session to be retrieved.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// The request for the ListSessions method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSessionsRequest {
    /// The maximum number of sessions to return per page.
    #[prost(int32, tag = "1")]
    pub page_size: i32,
    /// The page token, for retrieving subsequent pages.
    #[prost(string, tag = "2")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response for the ListSessions method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSessionsResponse {
    /// The sessions being returned.
    #[prost(message, repeated, tag = "1")]
    pub sessions: ::prost::alloc::vec::Vec<Session>,
    /// The next page token, if any.
    /// An empty value here means the last page has been reached.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request for the DeleteSession method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteSessionRequest {
    /// The session to be deleted.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for reporting on a session.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReportSessionRequest {
    /// The session to be reported on.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Response message for reporting on a session.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReportSessionResponse {
    /// The state of the report.
    #[prost(enumeration = "report_session_response::Result", tag = "1")]
    pub result: i32,
    /// The test runs of this session.
    #[prost(message, repeated, tag = "2")]
    pub test_runs: ::prost::alloc::vec::Vec<TestRun>,
}
/// Nested message and enum types in `ReportSessionResponse`.
pub mod report_session_response {
    /// The topline state of the report.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Result {
        Unspecified = 0,
        /// The session is complete, and everything passed.
        Passed = 1,
        /// The session had an explicit failure.
        Failed = 2,
        /// The session is incomplete. This is a failure response.
        Incomplete = 3,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Test {
    /// The name of the test.
    /// The tests/* portion of the names are hard-coded, and do not change
    /// from session to session.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The expectation level for this test.
    #[prost(enumeration = "test::ExpectationLevel", tag = "2")]
    pub expectation_level: i32,
    /// A description of the test.
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    /// The blueprints that will satisfy this test. There may be multiple
    /// blueprints that can signal to the server that this test case is being
    /// exercised. Although multiple blueprints are specified, only a single
    /// blueprint needs to be run to signal that the test case was exercised.
    #[prost(message, repeated, tag = "4")]
    pub blueprints: ::prost::alloc::vec::Vec<test::Blueprint>,
}
/// Nested message and enum types in `Test`.
pub mod test {
    /// A blueprint is an explicit definition of methods and requests that are
    /// needed to be made to test this specific test case. Ideally this would be
    /// represented by something more robust like CEL, but as of writing this, I am
    /// unsure if CEL is ready.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Blueprint {
        /// The name of this blueprint.
        #[prost(string, tag = "1")]
        pub name: ::prost::alloc::string::String,
        /// A description of this blueprint.
        #[prost(string, tag = "2")]
        pub description: ::prost::alloc::string::String,
        /// The initial request to trigger this test.
        #[prost(message, optional, tag = "3")]
        pub request: ::core::option::Option<blueprint::Invocation>,
        /// An ordered list of method calls that can be called to trigger this test.
        #[prost(message, repeated, tag = "4")]
        pub additional_requests: ::prost::alloc::vec::Vec<blueprint::Invocation>,
    }
    /// Nested message and enum types in `Blueprint`.
    pub mod blueprint {
        /// A message representing a method invocation.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Invocation {
            /// The fully qualified name of the showcase method to be invoked.
            #[prost(string, tag = "1")]
            pub method: ::prost::alloc::string::String,
            /// The request to be made if a specific request is necessary.
            #[prost(bytes = "vec", tag = "2")]
            pub serialized_request: ::prost::alloc::vec::Vec<u8>,
        }
    }
    /// Whether or not a test is required, recommended, or optional.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ExpectationLevel {
        Unspecified = 0,
        /// This test is strictly required.
        Required = 1,
        /// This test is recommended.
        ///
        /// If a generator explicitly ignores a recommended test (see `DeleteTest`),
        /// then the report may still pass, but with a warning.
        ///
        /// If a generator skips a recommended test and does not explicitly
        /// express that intention, the report will fail.
        Recommended = 2,
        /// This test is optional.
        ///
        /// If a generator explicitly ignores an optional test (see `DeleteTest`),
        /// then the report may still pass, and no warning will be issued.
        ///
        /// If a generator skips an optional test and does not explicitly
        /// express that intention, the report may still pass, but with a
        /// warning.
        Optional = 3,
    }
}
/// An issue found in the test.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Issue {
    /// The type of the issue.
    #[prost(enumeration = "issue::Type", tag = "1")]
    pub r#type: i32,
    /// The severity of the issue.
    #[prost(enumeration = "issue::Severity", tag = "2")]
    pub severity: i32,
    /// A description of the issue.
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
}
/// Nested message and enum types in `Issue`.
pub mod issue {
    /// The different potential types of issues.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Type {
        Unspecified = 0,
        /// The test was never instrumented.
        Skipped = 1,
        /// The test was started but never confirmed.
        Pending = 2,
        /// The test was instrumented, but Showcase got an unexpected
        /// value when the generator tried to confirm success.
        IncorrectConfirmation = 3,
    }
    /// Severity levels.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Severity {
        Unspecified = 0,
        /// Errors.
        Error = 1,
        /// Warnings.
        Warning = 2,
    }
}
/// The request for the ListTests method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTestsRequest {
    /// The session.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of tests to return per page.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The page token, for retrieving subsequent pages.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// The response for the ListTests method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTestsResponse {
    /// The tests being returned.
    #[prost(message, repeated, tag = "1")]
    pub tests: ::prost::alloc::vec::Vec<Test>,
    /// The next page token, if any.
    /// An empty value here means the last page has been reached.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// A TestRun is the result of running a Test.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TestRun {
    /// The name of the test.
    /// The tests/* portion of the names are hard-coded, and do not change
    /// from session to session.
    #[prost(string, tag = "1")]
    pub test: ::prost::alloc::string::String,
    /// An issue found with the test run. If empty, this test run was successful.
    #[prost(message, optional, tag = "2")]
    pub issue: ::core::option::Option<Issue>,
}
/// Request message for deleting a test.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteTestRequest {
    /// The test to be deleted.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VerifyTestRequest {
    /// The test to have an answer registered to it.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The answer from the test.
    #[prost(bytes = "vec", tag = "2")]
    pub answer: ::prost::alloc::vec::Vec<u8>,
    /// The answers from the test if multiple are to be checked
    #[prost(bytes = "vec", repeated, tag = "3")]
    pub answers: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VerifyTestResponse {
    /// An issue if check answer was unsuccessful. This will be empty if the check
    /// answer succeeded.
    #[prost(message, optional, tag = "1")]
    pub issue: ::core::option::Option<Issue>,
}
#[doc = r" Generated client implementations."]
pub mod testing_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " A service to facilitate running discrete sets of tests"]
    #[doc = " against Showcase."]
    #[derive(Debug, Clone)]
    pub struct TestingClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> TestingClient<T>
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
        ) -> TestingClient<InterceptedService<T, F>>
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
            TestingClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Creates a new testing session."]
        pub async fn create_session(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateSessionRequest>,
        ) -> Result<tonic::Response<super::Session>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.example.showcase.v1beta2.Testing/CreateSession",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets a testing session."]
        pub async fn get_session(
            &mut self,
            request: impl tonic::IntoRequest<super::GetSessionRequest>,
        ) -> Result<tonic::Response<super::Session>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.example.showcase.v1beta2.Testing/GetSession",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists the current test sessions."]
        pub async fn list_sessions(
            &mut self,
            request: impl tonic::IntoRequest<super::ListSessionsRequest>,
        ) -> Result<tonic::Response<super::ListSessionsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.example.showcase.v1beta2.Testing/ListSessions",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Delete a test session."]
        pub async fn delete_session(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteSessionRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.example.showcase.v1beta2.Testing/DeleteSession",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Report on the status of a session."]
        #[doc = " This generates a report detailing which tests have been completed,"]
        #[doc = " and an overall rollup."]
        pub async fn report_session(
            &mut self,
            request: impl tonic::IntoRequest<super::ReportSessionRequest>,
        ) -> Result<tonic::Response<super::ReportSessionResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.example.showcase.v1beta2.Testing/ReportSession",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " List the tests of a sessesion."]
        pub async fn list_tests(
            &mut self,
            request: impl tonic::IntoRequest<super::ListTestsRequest>,
        ) -> Result<tonic::Response<super::ListTestsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.example.showcase.v1beta2.Testing/ListTests",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Explicitly decline to implement a test."]
        #[doc = ""]
        #[doc = " This removes the test from subsequent `ListTests` calls, and"]
        #[doc = " attempting to do the test will error."]
        #[doc = ""]
        #[doc = " This method will error if attempting to delete a required test."]
        pub async fn delete_test(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteTestRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.example.showcase.v1beta2.Testing/DeleteTest",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Register a response to a test."]
        #[doc = ""]
        #[doc = " In cases where a test involves registering a final answer at the"]
        #[doc = " end of the test, this method provides the means to do so."]
        pub async fn verify_test(
            &mut self,
            request: impl tonic::IntoRequest<super::VerifyTestRequest>,
        ) -> Result<tonic::Response<super::VerifyTestResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.example.showcase.v1beta2.Testing/VerifyTest",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
