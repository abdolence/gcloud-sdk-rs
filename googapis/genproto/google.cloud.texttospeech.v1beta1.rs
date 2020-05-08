/// The top-level message sent by the client for the `ListVoices` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListVoicesRequest {
    /// Optional. Recommended.
    /// [BCP-47](https://www.rfc-editor.org/rfc/bcp/bcp47.txt) language tag. If
    /// specified, the ListVoices call will only return voices that can be used to
    /// synthesize this language_code. E.g. when specifying "en-NZ", you will get
    /// supported "en-*" voices; when specifying "no", you will get supported
    /// "no-*" (Norwegian) and "nb-*" (Norwegian Bokmal) voices; specifying "zh"
    /// will also get supported "cmn-*" voices; specifying "zh-hk" will also get
    /// supported "yue-*" voices.
    #[prost(string, tag = "1")]
    pub language_code: std::string::String,
}
/// The message returned to the client by the `ListVoices` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListVoicesResponse {
    /// The list of voices.
    #[prost(message, repeated, tag = "1")]
    pub voices: ::std::vec::Vec<Voice>,
}
/// Description of a voice supported by the TTS service.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Voice {
    /// The languages that this voice supports, expressed as
    /// [BCP-47](https://www.rfc-editor.org/rfc/bcp/bcp47.txt) language tags (e.g.
    /// "en-US", "es-419", "cmn-tw").
    #[prost(string, repeated, tag = "1")]
    pub language_codes: ::std::vec::Vec<std::string::String>,
    /// The name of this voice.  Each distinct voice has a unique name.
    #[prost(string, tag = "2")]
    pub name: std::string::String,
    /// The gender of this voice.
    #[prost(enumeration = "SsmlVoiceGender", tag = "3")]
    pub ssml_gender: i32,
    /// The natural sample rate (in hertz) for this voice.
    #[prost(int32, tag = "4")]
    pub natural_sample_rate_hertz: i32,
}
/// The top-level message sent by the client for the `SynthesizeSpeech` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SynthesizeSpeechRequest {
    /// Required. The Synthesizer requires either plain text or SSML as input.
    #[prost(message, optional, tag = "1")]
    pub input: ::std::option::Option<SynthesisInput>,
    /// Required. The desired voice of the synthesized audio.
    #[prost(message, optional, tag = "2")]
    pub voice: ::std::option::Option<VoiceSelectionParams>,
    /// Required. The configuration of the synthesized audio.
    #[prost(message, optional, tag = "3")]
    pub audio_config: ::std::option::Option<AudioConfig>,
}
/// Contains text input to be synthesized. Either `text` or `ssml` must be
/// supplied. Supplying both or neither returns
/// [google.rpc.Code.INVALID_ARGUMENT][]. The input size is limited to 5000
/// characters.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SynthesisInput {
    /// The input source, which is either plain text or SSML.
    #[prost(oneof = "synthesis_input::InputSource", tags = "1, 2")]
    pub input_source: ::std::option::Option<synthesis_input::InputSource>,
}
pub mod synthesis_input {
    /// The input source, which is either plain text or SSML.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum InputSource {
        /// The raw text to be synthesized.
        #[prost(string, tag = "1")]
        Text(std::string::String),
        /// The SSML document to be synthesized. The SSML document must be valid
        /// and well-formed. Otherwise the RPC will fail and return
        /// [google.rpc.Code.INVALID_ARGUMENT][]. For more information, see
        /// [SSML](/speech/text-to-speech/docs/ssml).
        #[prost(string, tag = "2")]
        Ssml(std::string::String),
    }
}
/// Description of which voice to use for a synthesis request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VoiceSelectionParams {
    /// Required. The language (and potentially also the region) of the voice expressed as a
    /// [BCP-47](https://www.rfc-editor.org/rfc/bcp/bcp47.txt) language tag, e.g.
    /// "en-US". This should not include a script tag (e.g. use
    /// "cmn-cn" rather than "cmn-Hant-cn"), because the script will be inferred
    /// from the input provided in the SynthesisInput.  The TTS service
    /// will use this parameter to help choose an appropriate voice.  Note that
    /// the TTS service may choose a voice with a slightly different language code
    /// than the one selected; it may substitute a different region
    /// (e.g. using en-US rather than en-CA if there isn't a Canadian voice
    /// available), or even a different language, e.g. using "nb" (Norwegian
    /// Bokmal) instead of "no" (Norwegian)".
    #[prost(string, tag = "1")]
    pub language_code: std::string::String,
    /// The name of the voice. If not set, the service will choose a
    /// voice based on the other parameters such as language_code and gender.
    #[prost(string, tag = "2")]
    pub name: std::string::String,
    /// The preferred gender of the voice. If not set, the service will
    /// choose a voice based on the other parameters such as language_code and
    /// name. Note that this is only a preference, not requirement; if a
    /// voice of the appropriate gender is not available, the synthesizer should
    /// substitute a voice with a different gender rather than failing the request.
    #[prost(enumeration = "SsmlVoiceGender", tag = "3")]
    pub ssml_gender: i32,
}
/// Description of audio data to be synthesized.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AudioConfig {
    /// Required. The format of the audio byte stream.
    #[prost(enumeration = "AudioEncoding", tag = "1")]
    pub audio_encoding: i32,
    /// Optional. Input only. Speaking rate/speed, in the range [0.25, 4.0]. 1.0 is
    /// the normal native speed supported by the specific voice. 2.0 is twice as
    /// fast, and 0.5 is half as fast. If unset(0.0), defaults to the native 1.0
    /// speed. Any other values < 0.25 or > 4.0 will return an error.
    #[prost(double, tag = "2")]
    pub speaking_rate: f64,
    /// Optional. Input only. Speaking pitch, in the range [-20.0, 20.0]. 20 means
    /// increase 20 semitones from the original pitch. -20 means decrease 20
    /// semitones from the original pitch.
    #[prost(double, tag = "3")]
    pub pitch: f64,
    /// Optional. Input only. Volume gain (in dB) of the normal native volume
    /// supported by the specific voice, in the range [-96.0, 16.0]. If unset, or
    /// set to a value of 0.0 (dB), will play at normal native signal amplitude. A
    /// value of -6.0 (dB) will play at approximately half the amplitude of the
    /// normal native signal amplitude. A value of +6.0 (dB) will play at
    /// approximately twice the amplitude of the normal native signal amplitude.
    /// Strongly recommend not to exceed +10 (dB) as there's usually no effective
    /// increase in loudness for any value greater than that.
    #[prost(double, tag = "4")]
    pub volume_gain_db: f64,
    /// Optional. The synthesis sample rate (in hertz) for this audio. When this is
    /// specified in SynthesizeSpeechRequest, if this is different from the voice's
    /// natural sample rate, then the synthesizer will honor this request by
    /// converting to the desired sample rate (which might result in worse audio
    /// quality), unless the specified sample rate is not supported for the
    /// encoding chosen, in which case it will fail the request and return
    /// [google.rpc.Code.INVALID_ARGUMENT][].
    #[prost(int32, tag = "5")]
    pub sample_rate_hertz: i32,
    /// Optional. Input only. An identifier which selects 'audio effects' profiles
    /// that are applied on (post synthesized) text to speech. Effects are applied
    /// on top of each other in the order they are given. See
    /// [audio
    /// profiles](https://cloud.google.com/text-to-speech/docs/audio-profiles) for
    /// current supported profile ids.
    #[prost(string, repeated, tag = "6")]
    pub effects_profile_id: ::std::vec::Vec<std::string::String>,
}
/// The message returned to the client by the `SynthesizeSpeech` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SynthesizeSpeechResponse {
    /// The audio data bytes encoded as specified in the request, including the
    /// header for encodings that are wrapped in containers (e.g. MP3, OGG_OPUS).
    /// For LINEAR16 audio, we include the WAV header. Note: as
    /// with all bytes fields, protobuffers use a pure binary representation,
    /// whereas JSON representations use base64.
    #[prost(bytes, tag = "1")]
    pub audio_content: std::vec::Vec<u8>,
}
/// Gender of the voice as described in
/// [SSML voice element](https://www.w3.org/TR/speech-synthesis11/#edef_voice).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SsmlVoiceGender {
    /// An unspecified gender.
    /// In VoiceSelectionParams, this means that the client doesn't care which
    /// gender the selected voice will have. In the Voice field of
    /// ListVoicesResponse, this may mean that the voice doesn't fit any of the
    /// other categories in this enum, or that the gender of the voice isn't known.
    Unspecified = 0,
    /// A male voice.
    Male = 1,
    /// A female voice.
    Female = 2,
    /// A gender-neutral voice.
    Neutral = 3,
}
/// Configuration to set up audio encoder. The encoding determines the output
/// audio format that we'd like.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AudioEncoding {
    /// Not specified. Will return result [google.rpc.Code.INVALID_ARGUMENT][].
    Unspecified = 0,
    /// Uncompressed 16-bit signed little-endian samples (Linear PCM).
    /// Audio content returned as LINEAR16 also contains a WAV header.
    Linear16 = 1,
    /// MP3 audio at 32kbps.
    Mp3 = 2,
    /// Opus encoded audio wrapped in an ogg container. The result will be a
    /// file which can be played natively on Android, and in browsers (at least
    /// Chrome and Firefox). The quality of the encoding is considerably higher
    /// than MP3 while using approximately the same bitrate.
    OggOpus = 3,
}
#[doc = r" Generated client implementations."]
pub mod text_to_speech_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Service that implements Google Cloud Text-to-Speech API."]
    pub struct TextToSpeechClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl TextToSpeechClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> TextToSpeechClient<T>
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
        #[doc = " Returns a list of Voice supported for synthesis."]
        pub async fn list_voices(
            &mut self,
            request: impl tonic::IntoRequest<super::ListVoicesRequest>,
        ) -> Result<tonic::Response<super::ListVoicesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.texttospeech.v1beta1.TextToSpeech/ListVoices",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Synthesizes speech synchronously: receive results after all text input"]
        #[doc = " has been processed."]
        pub async fn synthesize_speech(
            &mut self,
            request: impl tonic::IntoRequest<super::SynthesizeSpeechRequest>,
        ) -> Result<tonic::Response<super::SynthesizeSpeechResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.texttospeech.v1beta1.TextToSpeech/SynthesizeSpeech",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for TextToSpeechClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for TextToSpeechClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "TextToSpeechClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod text_to_speech_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with TextToSpeechServer."]
    #[async_trait]
    pub trait TextToSpeech: Send + Sync + 'static {
        #[doc = " Returns a list of Voice supported for synthesis."]
        async fn list_voices(
            &self,
            request: tonic::Request<super::ListVoicesRequest>,
        ) -> Result<tonic::Response<super::ListVoicesResponse>, tonic::Status>;
        #[doc = " Synthesizes speech synchronously: receive results after all text input"]
        #[doc = " has been processed."]
        async fn synthesize_speech(
            &self,
            request: tonic::Request<super::SynthesizeSpeechRequest>,
        ) -> Result<tonic::Response<super::SynthesizeSpeechResponse>, tonic::Status>;
    }
    #[doc = " Service that implements Google Cloud Text-to-Speech API."]
    #[derive(Debug)]
    #[doc(hidden)]
    pub struct TextToSpeechServer<T: TextToSpeech> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: TextToSpeech> TextToSpeechServer<T> {
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
    impl<T, B> Service<http::Request<B>> for TextToSpeechServer<T>
    where
        T: TextToSpeech,
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
                "/google.cloud.texttospeech.v1beta1.TextToSpeech/ListVoices" => {
                    #[allow(non_camel_case_types)]
                    struct ListVoicesSvc<T: TextToSpeech>(pub Arc<T>);
                    impl<T: TextToSpeech> tonic::server::UnaryService<super::ListVoicesRequest> for ListVoicesSvc<T> {
                        type Response = super::ListVoicesResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListVoicesRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.list_voices(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ListVoicesSvc(inner);
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
                "/google.cloud.texttospeech.v1beta1.TextToSpeech/SynthesizeSpeech" => {
                    #[allow(non_camel_case_types)]
                    struct SynthesizeSpeechSvc<T: TextToSpeech>(pub Arc<T>);
                    impl<T: TextToSpeech>
                        tonic::server::UnaryService<super::SynthesizeSpeechRequest>
                        for SynthesizeSpeechSvc<T>
                    {
                        type Response = super::SynthesizeSpeechResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SynthesizeSpeechRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.synthesize_speech(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = SynthesizeSpeechSvc(inner);
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
    impl<T: TextToSpeech> Clone for TextToSpeechServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: TextToSpeech> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: TextToSpeech> tonic::transport::NamedService for TextToSpeechServer<T> {
        const NAME: &'static str = "google.cloud.texttospeech.v1beta1.TextToSpeech";
    }
}
