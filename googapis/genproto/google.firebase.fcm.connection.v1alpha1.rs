/// Request sent to FCM from the connected client.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpstreamRequest {
    /// The type of request the client is making to FCM.
    #[prost(oneof = "upstream_request::RequestType", tags = "1")]
    pub request_type: ::std::option::Option<upstream_request::RequestType>,
}
pub mod upstream_request {
    /// The type of request the client is making to FCM.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum RequestType {
        /// Message acknowledgement.
        #[prost(message, tag = "1")]
        Ack(super::Ack),
    }
}
/// Response sent to the connected client from FCM.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DownstreamResponse {
    /// The type of response FCM is sending to the client.
    #[prost(oneof = "downstream_response::ResponseType", tags = "1")]
    pub response_type: ::std::option::Option<downstream_response::ResponseType>,
}
pub mod downstream_response {
    /// The type of response FCM is sending to the client.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ResponseType {
        /// Message sent to FCM via the [Send
        /// API](https://firebase.google.com/docs/cloud-messaging/send-message)
        /// targeting this client.
        #[prost(message, tag = "1")]
        Message(super::Message),
    }
}
/// Acknowledgement to indicate a client successfully received an FCM message.
///
/// If a message is not acked, FCM will continously resend the message until
/// it expires. Duplicate delivery in this case is working as intended.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Ack {
    /// Id of message being acknowledged
    #[prost(string, tag = "1")]
    pub message_id: std::string::String,
}
/// Message created through the [Send
/// API](https://firebase.google.com/docs/reference/fcm/rest/v1/projects.messages#resource-message).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Message {
    /// The identifier of the message. Used to ack the message.
    #[prost(string, tag = "1")]
    pub message_id: std::string::String,
    /// Time the message was received in FCM.
    #[prost(message, optional, tag = "2")]
    pub create_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Expiry time of the message. Currently it is always 4 weeks.
    #[prost(message, optional, tag = "3")]
    pub expire_time: ::std::option::Option<::prost_types::Timestamp>,
    /// The arbitrary payload set in the [Send
    /// API](https://firebase.google.com/docs/reference/fcm/rest/v1/projects.messages#resource-message).
    #[prost(map = "string, string", tag = "4")]
    pub data: ::std::collections::HashMap<std::string::String, std::string::String>,
}
#[doc = r" Generated client implementations."]
pub mod connection_api_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " FCM's service to create client connections to send/receive messages."]
    pub struct ConnectionApiClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> ConnectionApiClient<T>
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
        #[doc = " Creates a streaming connection with FCM to send messages and their"]
        #[doc = " respective ACKs."]
        #[doc = ""]
        #[doc = " The client credentials need to be passed in the [gRPC"]
        #[doc = " Metadata](https://grpc.io/docs/guides/concepts.html#metadata). The Format"]
        #[doc = " of the header is:"]
        #[doc = "   Key: \"authorization\""]
        #[doc = "   Value: \"Checkin [client_id:secret]\""]
        #[doc = ""]
        #[doc = ""]
        #[doc = " The project's API key also needs to be sent to authorize the project."]
        #[doc = " That can be set in the X-Goog-Api-Key Metadata header."]
        pub async fn connect(
            &mut self,
            request: impl tonic::IntoStreamingRequest<Message = super::UpstreamRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::DownstreamResponse>>,
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
                "/google.firebase.fcm.connection.v1alpha1.ConnectionApi/Connect",
            );
            self.inner
                .streaming(request.into_streaming_request(), path, codec)
                .await
        }
    }
    impl<T: Clone> Clone for ConnectionApiClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for ConnectionApiClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "ConnectionApiClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod connection_api_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with ConnectionApiServer."]
    #[async_trait]
    pub trait ConnectionApi: Send + Sync + 'static {
        #[doc = "Server streaming response type for the Connect method."]
        type ConnectStream: Stream<Item = Result<super::DownstreamResponse, tonic::Status>>
            + Send
            + Sync
            + 'static;
        #[doc = " Creates a streaming connection with FCM to send messages and their"]
        #[doc = " respective ACKs."]
        #[doc = ""]
        #[doc = " The client credentials need to be passed in the [gRPC"]
        #[doc = " Metadata](https://grpc.io/docs/guides/concepts.html#metadata). The Format"]
        #[doc = " of the header is:"]
        #[doc = "   Key: \"authorization\""]
        #[doc = "   Value: \"Checkin [client_id:secret]\""]
        #[doc = ""]
        #[doc = ""]
        #[doc = " The project's API key also needs to be sent to authorize the project."]
        #[doc = " That can be set in the X-Goog-Api-Key Metadata header."]
        async fn connect(
            &self,
            request: tonic::Request<tonic::Streaming<super::UpstreamRequest>>,
        ) -> Result<tonic::Response<Self::ConnectStream>, tonic::Status>;
    }
    #[doc = " FCM's service to create client connections to send/receive messages."]
    #[derive(Debug)]
    pub struct ConnectionApiServer<T: ConnectionApi> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: ConnectionApi> ConnectionApiServer<T> {
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
    impl<T, B> Service<http::Request<B>> for ConnectionApiServer<T>
    where
        T: ConnectionApi,
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
                "/google.firebase.fcm.connection.v1alpha1.ConnectionApi/Connect" => {
                    #[allow(non_camel_case_types)]
                    struct ConnectSvc<T: ConnectionApi>(pub Arc<T>);
                    impl<T: ConnectionApi> tonic::server::StreamingService<super::UpstreamRequest> for ConnectSvc<T> {
                        type Response = super::DownstreamResponse;
                        type ResponseStream = T::ConnectStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<tonic::Streaming<super::UpstreamRequest>>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).connect(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1;
                        let inner = inner.0;
                        let method = ConnectSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.streaming(method, req).await;
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
    impl<T: ConnectionApi> Clone for ConnectionApiServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: ConnectionApi> Clone for _Inner<T> {
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
