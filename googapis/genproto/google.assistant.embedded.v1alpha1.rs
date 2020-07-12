/// Specifies how to process the `ConverseRequest` messages.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConverseConfig {
    /// *Required* Specifies how to process the subsequent incoming audio.
    #[prost(message, optional, tag = "1")]
    pub audio_in_config: ::std::option::Option<AudioInConfig>,
    /// *Required* Specifies how to format the audio that will be returned.
    #[prost(message, optional, tag = "2")]
    pub audio_out_config: ::std::option::Option<AudioOutConfig>,
    /// *Required* Represents the current dialog state.
    #[prost(message, optional, tag = "3")]
    pub converse_state: ::std::option::Option<ConverseState>,
}
/// Specifies how to process the `audio_in` data that will be provided in
/// subsequent requests. For recommended settings, see the Google Assistant SDK
/// [best
/// practices](https://developers.google.com/assistant/sdk/develop/grpc/best-practices/audio).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AudioInConfig {
    /// *Required* Encoding of audio data sent in all `audio_in` messages.
    #[prost(enumeration = "audio_in_config::Encoding", tag = "1")]
    pub encoding: i32,
    /// *Required* Sample rate (in Hertz) of the audio data sent in all `audio_in`
    /// messages. Valid values are from 16000-24000, but 16000 is optimal.
    /// For best results, set the sampling rate of the audio source to 16000 Hz.
    /// If that's not possible, use the native sample rate of the audio source
    /// (instead of re-sampling).
    #[prost(int32, tag = "2")]
    pub sample_rate_hertz: i32,
}
pub mod audio_in_config {
    /// Audio encoding of the data sent in the audio message.
    /// Audio must be one-channel (mono). The only language supported is "en-US".
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Encoding {
        /// Not specified. Will return result [google.rpc.Code.INVALID_ARGUMENT][].
        Unspecified = 0,
        /// Uncompressed 16-bit signed little-endian samples (Linear PCM).
        /// This encoding includes no header, only the raw audio bytes.
        Linear16 = 1,
        /// [`FLAC`](https://xiph.org/flac/documentation.html) (Free Lossless Audio
        /// Codec) is the recommended encoding because it is
        /// lossless--therefore recognition is not compromised--and
        /// requires only about half the bandwidth of `LINEAR16`. This encoding
        /// includes the `FLAC` stream header followed by audio data. It supports
        /// 16-bit and 24-bit samples, however, not all fields in `STREAMINFO` are
        /// supported.
        Flac = 2,
    }
}
/// Specifies the desired format for the server to use when it returns
/// `audio_out` messages.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AudioOutConfig {
    /// *Required* The encoding of audio data to be returned in all `audio_out`
    /// messages.
    #[prost(enumeration = "audio_out_config::Encoding", tag = "1")]
    pub encoding: i32,
    /// *Required* The sample rate in Hertz of the audio data returned in
    /// `audio_out` messages. Valid values are: 16000-24000.
    #[prost(int32, tag = "2")]
    pub sample_rate_hertz: i32,
    /// *Required* Current volume setting of the device's audio output.
    /// Valid values are 1 to 100 (corresponding to 1% to 100%).
    #[prost(int32, tag = "3")]
    pub volume_percentage: i32,
}
pub mod audio_out_config {
    /// Audio encoding of the data returned in the audio message. All encodings are
    /// raw audio bytes with no header, except as indicated below.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Encoding {
        /// Not specified. Will return result [google.rpc.Code.INVALID_ARGUMENT][].
        Unspecified = 0,
        /// Uncompressed 16-bit signed little-endian samples (Linear PCM).
        Linear16 = 1,
        /// MP3 audio encoding. The sample rate is encoded in the payload.
        Mp3 = 2,
        /// Opus-encoded audio wrapped in an ogg container. The result will be a
        /// file which can be played natively on Android and in some browsers (such
        /// as Chrome). The quality of the encoding is considerably higher than MP3
        /// while using the same bitrate. The sample rate is encoded in the payload.
        OpusInOgg = 3,
    }
}
/// Provides information about the current dialog state.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConverseState {
    /// *Required* The `conversation_state` value returned in the prior
    /// `ConverseResponse`. Omit (do not set the field) if there was no prior
    /// `ConverseResponse`. If there was a prior `ConverseResponse`, do not omit
    /// this field; doing so will end that conversation (and this new request will
    /// start a new conversation).
    #[prost(bytes, tag = "1")]
    pub conversation_state: std::vec::Vec<u8>,
}
/// The audio containing the assistant's response to the query. Sequential chunks
/// of audio data are received in sequential `ConverseResponse` messages.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AudioOut {
    /// *Output-only* The audio data containing the assistant's response to the
    /// query. Sequential chunks of audio data are received in sequential
    /// `ConverseResponse` messages.
    #[prost(bytes, tag = "1")]
    pub audio_data: std::vec::Vec<u8>,
}
/// The semantic result for the user's spoken query.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConverseResult {
    /// *Output-only* The recognized transcript of what the user said.
    #[prost(string, tag = "1")]
    pub spoken_request_text: std::string::String,
    /// *Output-only* The text of the assistant's spoken response. This is only
    /// returned for an IFTTT action.
    #[prost(string, tag = "2")]
    pub spoken_response_text: std::string::String,
    /// *Output-only* State information for subsequent `ConverseRequest`. This
    /// value should be saved in the client and returned in the
    /// `conversation_state` with the next `ConverseRequest`. (The client does not
    /// need to interpret or otherwise use this value.) There is no need to save
    /// this information across device restarts.
    #[prost(bytes, tag = "3")]
    pub conversation_state: std::vec::Vec<u8>,
    /// *Output-only* Specifies the mode of the microphone after this `Converse`
    /// RPC is processed.
    #[prost(enumeration = "converse_result::MicrophoneMode", tag = "4")]
    pub microphone_mode: i32,
    /// *Output-only* Updated volume level. The value will be 0 or omitted
    /// (indicating no change) unless a voice command such as "Increase the volume"
    /// or "Set volume level 4" was recognized, in which case the value will be
    /// between 1 and 100 (corresponding to the new volume level of 1% to 100%).
    /// Typically, a client should use this volume level when playing the
    /// `audio_out` data, and retain this value as the current volume level and
    /// supply it in the `AudioOutConfig` of the next `ConverseRequest`. (Some
    /// clients may also implement other ways to allow the current volume level to
    /// be changed, for example, by providing a knob that the user can turn.)
    #[prost(int32, tag = "5")]
    pub volume_percentage: i32,
}
pub mod converse_result {
    /// Possible states of the microphone after a `Converse` RPC completes.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum MicrophoneMode {
        /// No mode specified.
        Unspecified = 0,
        /// The service is not expecting a follow-on question from the user.
        /// The microphone should remain off until the user re-activates it.
        CloseMicrophone = 1,
        /// The service is expecting a follow-on question from the user. The
        /// microphone should be re-opened when the `AudioOut` playback completes
        /// (by starting a new `Converse` RPC call to send the new audio).
        DialogFollowOn = 2,
    }
}
/// The top-level message sent by the client. Clients must send at least two, and
/// typically numerous `ConverseRequest` messages. The first message must
/// contain a `config` message and must not contain `audio_in` data. All
/// subsequent messages must contain `audio_in` data and must not contain a
/// `config` message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConverseRequest {
    /// Exactly one of these fields must be specified in each `ConverseRequest`.
    #[prost(oneof = "converse_request::ConverseRequest", tags = "1, 2")]
    pub converse_request: ::std::option::Option<converse_request::ConverseRequest>,
}
pub mod converse_request {
    /// Exactly one of these fields must be specified in each `ConverseRequest`.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ConverseRequest {
        /// The `config` message provides information to the recognizer that
        /// specifies how to process the request.
        /// The first `ConverseRequest` message must contain a `config` message.
        #[prost(message, tag = "1")]
        Config(super::ConverseConfig),
        /// The audio data to be recognized. Sequential chunks of audio data are sent
        /// in sequential `ConverseRequest` messages. The first `ConverseRequest`
        /// message must not contain `audio_in` data and all subsequent
        /// `ConverseRequest` messages must contain `audio_in` data. The audio bytes
        /// must be encoded as specified in `AudioInConfig`.
        /// Audio must be sent at approximately real-time (16000 samples per second).
        /// An error will be returned if audio is sent significantly faster or
        /// slower.
        #[prost(bytes, tag = "2")]
        AudioIn(std::vec::Vec<u8>),
    }
}
/// The top-level message received by the client. A series of one or more
/// `ConverseResponse` messages are streamed back to the client.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConverseResponse {
    /// Exactly one of these fields will be populated in each `ConverseResponse`.
    #[prost(oneof = "converse_response::ConverseResponse", tags = "1, 2, 3, 5")]
    pub converse_response: ::std::option::Option<converse_response::ConverseResponse>,
}
pub mod converse_response {
    /// Indicates the type of event.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum EventType {
        /// No event specified.
        Unspecified = 0,
        /// This event indicates that the server has detected the end of the user's
        /// speech utterance and expects no additional speech. Therefore, the server
        /// will not process additional audio (although it may subsequently return
        /// additional results). The client should stop sending additional audio
        /// data, half-close the gRPC connection, and wait for any additional results
        /// until the server closes the gRPC connection.
        EndOfUtterance = 1,
    }
    /// Exactly one of these fields will be populated in each `ConverseResponse`.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ConverseResponse {
        /// *Output-only* If set, returns a [google.rpc.Status][google.rpc.Status]
        /// message that specifies the error for the operation. If an error occurs
        /// during processing, this message will be set and there will be no further
        /// messages sent.
        #[prost(message, tag = "1")]
        Error(super::super::super::super::rpc::Status),
        /// *Output-only* Indicates the type of event.
        #[prost(enumeration = "EventType", tag = "2")]
        EventType(i32),
        /// *Output-only* The audio containing the assistant's response to the query.
        #[prost(message, tag = "3")]
        AudioOut(super::AudioOut),
        /// *Output-only* The semantic result for the user's spoken query.
        #[prost(message, tag = "5")]
        Result(super::ConverseResult),
    }
}
#[doc = r" Generated client implementations."]
pub mod embedded_assistant_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Service that implements Google Assistant API."]
    pub struct EmbeddedAssistantClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> EmbeddedAssistantClient<T>
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
        #[doc = " Initiates or continues a conversation with the embedded assistant service."]
        #[doc = " Each call performs one round-trip, sending an audio request to the service"]
        #[doc = " and receiving the audio response. Uses bidirectional streaming to receive"]
        #[doc = " results, such as the `END_OF_UTTERANCE` event, while sending audio."]
        #[doc = ""]
        #[doc = " A conversation is one or more gRPC connections, each consisting of several"]
        #[doc = " streamed requests and responses."]
        #[doc = " For example, the user says *Add to my shopping list* and the assistant"]
        #[doc = " responds *What do you want to add?*. The sequence of streamed requests and"]
        #[doc = " responses in the first gRPC message could be:"]
        #[doc = ""]
        #[doc = " *   ConverseRequest.config"]
        #[doc = " *   ConverseRequest.audio_in"]
        #[doc = " *   ConverseRequest.audio_in"]
        #[doc = " *   ConverseRequest.audio_in"]
        #[doc = " *   ConverseRequest.audio_in"]
        #[doc = " *   ConverseResponse.event_type.END_OF_UTTERANCE"]
        #[doc = " *   ConverseResponse.result.microphone_mode.DIALOG_FOLLOW_ON"]
        #[doc = " *   ConverseResponse.audio_out"]
        #[doc = " *   ConverseResponse.audio_out"]
        #[doc = " *   ConverseResponse.audio_out"]
        #[doc = ""]
        #[doc = " The user then says *bagels* and the assistant responds"]
        #[doc = " *OK, I've added bagels to your shopping list*. This is sent as another gRPC"]
        #[doc = " connection call to the `Converse` method, again with streamed requests and"]
        #[doc = " responses, such as:"]
        #[doc = ""]
        #[doc = " *   ConverseRequest.config"]
        #[doc = " *   ConverseRequest.audio_in"]
        #[doc = " *   ConverseRequest.audio_in"]
        #[doc = " *   ConverseRequest.audio_in"]
        #[doc = " *   ConverseResponse.event_type.END_OF_UTTERANCE"]
        #[doc = " *   ConverseResponse.result.microphone_mode.CLOSE_MICROPHONE"]
        #[doc = " *   ConverseResponse.audio_out"]
        #[doc = " *   ConverseResponse.audio_out"]
        #[doc = " *   ConverseResponse.audio_out"]
        #[doc = " *   ConverseResponse.audio_out"]
        #[doc = ""]
        #[doc = " Although the precise order of responses is not guaranteed, sequential"]
        #[doc = " ConverseResponse.audio_out messages will always contain sequential portions"]
        #[doc = " of audio."]
        pub async fn converse(
            &mut self,
            request: impl tonic::IntoStreamingRequest<Message = super::ConverseRequest>,
        ) -> Result<tonic::Response<tonic::codec::Streaming<super::ConverseResponse>>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.assistant.embedded.v1alpha1.EmbeddedAssistant/Converse",
            );
            self.inner
                .streaming(request.into_streaming_request(), path, codec)
                .await
        }
    }
    impl<T: Clone> Clone for EmbeddedAssistantClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for EmbeddedAssistantClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "EmbeddedAssistantClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod embedded_assistant_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with EmbeddedAssistantServer."]
    #[async_trait]
    pub trait EmbeddedAssistant: Send + Sync + 'static {
        #[doc = "Server streaming response type for the Converse method."]
        type ConverseStream: Stream<Item = Result<super::ConverseResponse, tonic::Status>>
            + Send
            + Sync
            + 'static;
        #[doc = " Initiates or continues a conversation with the embedded assistant service."]
        #[doc = " Each call performs one round-trip, sending an audio request to the service"]
        #[doc = " and receiving the audio response. Uses bidirectional streaming to receive"]
        #[doc = " results, such as the `END_OF_UTTERANCE` event, while sending audio."]
        #[doc = ""]
        #[doc = " A conversation is one or more gRPC connections, each consisting of several"]
        #[doc = " streamed requests and responses."]
        #[doc = " For example, the user says *Add to my shopping list* and the assistant"]
        #[doc = " responds *What do you want to add?*. The sequence of streamed requests and"]
        #[doc = " responses in the first gRPC message could be:"]
        #[doc = ""]
        #[doc = " *   ConverseRequest.config"]
        #[doc = " *   ConverseRequest.audio_in"]
        #[doc = " *   ConverseRequest.audio_in"]
        #[doc = " *   ConverseRequest.audio_in"]
        #[doc = " *   ConverseRequest.audio_in"]
        #[doc = " *   ConverseResponse.event_type.END_OF_UTTERANCE"]
        #[doc = " *   ConverseResponse.result.microphone_mode.DIALOG_FOLLOW_ON"]
        #[doc = " *   ConverseResponse.audio_out"]
        #[doc = " *   ConverseResponse.audio_out"]
        #[doc = " *   ConverseResponse.audio_out"]
        #[doc = ""]
        #[doc = " The user then says *bagels* and the assistant responds"]
        #[doc = " *OK, I've added bagels to your shopping list*. This is sent as another gRPC"]
        #[doc = " connection call to the `Converse` method, again with streamed requests and"]
        #[doc = " responses, such as:"]
        #[doc = ""]
        #[doc = " *   ConverseRequest.config"]
        #[doc = " *   ConverseRequest.audio_in"]
        #[doc = " *   ConverseRequest.audio_in"]
        #[doc = " *   ConverseRequest.audio_in"]
        #[doc = " *   ConverseResponse.event_type.END_OF_UTTERANCE"]
        #[doc = " *   ConverseResponse.result.microphone_mode.CLOSE_MICROPHONE"]
        #[doc = " *   ConverseResponse.audio_out"]
        #[doc = " *   ConverseResponse.audio_out"]
        #[doc = " *   ConverseResponse.audio_out"]
        #[doc = " *   ConverseResponse.audio_out"]
        #[doc = ""]
        #[doc = " Although the precise order of responses is not guaranteed, sequential"]
        #[doc = " ConverseResponse.audio_out messages will always contain sequential portions"]
        #[doc = " of audio."]
        async fn converse(
            &self,
            request: tonic::Request<tonic::Streaming<super::ConverseRequest>>,
        ) -> Result<tonic::Response<Self::ConverseStream>, tonic::Status>;
    }
    #[doc = " Service that implements Google Assistant API."]
    #[derive(Debug)]
    #[doc(hidden)]
    pub struct EmbeddedAssistantServer<T: EmbeddedAssistant> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: EmbeddedAssistant> EmbeddedAssistantServer<T> {
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
    impl<T, B> Service<http::Request<B>> for EmbeddedAssistantServer<T>
    where
        T: EmbeddedAssistant,
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
                "/google.assistant.embedded.v1alpha1.EmbeddedAssistant/Converse" => {
                    #[allow(non_camel_case_types)]
                    struct ConverseSvc<T: EmbeddedAssistant>(pub Arc<T>);
                    impl<T: EmbeddedAssistant>
                        tonic::server::StreamingService<super::ConverseRequest> for ConverseSvc<T>
                    {
                        type Response = super::ConverseResponse;
                        type ResponseStream = T::ConverseStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<tonic::Streaming<super::ConverseRequest>>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.converse(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1;
                        let inner = inner.0;
                        let method = ConverseSvc(inner);
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
    impl<T: EmbeddedAssistant> Clone for EmbeddedAssistantServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: EmbeddedAssistant> Clone for _Inner<T> {
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
