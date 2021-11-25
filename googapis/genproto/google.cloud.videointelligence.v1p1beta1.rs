/// Video annotation request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnnotateVideoRequest {
    /// Input video location. Currently, only
    /// [Google Cloud Storage](<https://cloud.google.com/storage/>) URIs are
    /// supported, which must be specified in the following format:
    /// `gs://bucket-id/object-id` (other URI formats return
    /// \[google.rpc.Code.INVALID_ARGUMENT][google.rpc.Code.INVALID_ARGUMENT\]). For
    /// more information, see [Request
    /// URIs](<https://cloud.google.com/storage/docs/request-endpoints>). A video URI
    /// may include wildcards in `object-id`, and thus identify multiple videos.
    /// Supported wildcards: '*' to match 0 or more characters;
    /// '?' to match 1 character. If unset, the input video should be embedded
    /// in the request as `input_content`. If set, `input_content` should be unset.
    #[prost(string, tag = "1")]
    pub input_uri: ::prost::alloc::string::String,
    /// The video data bytes.
    /// If unset, the input video(s) should be specified via `input_uri`.
    /// If set, `input_uri` should be unset.
    #[prost(bytes = "vec", tag = "6")]
    pub input_content: ::prost::alloc::vec::Vec<u8>,
    /// Required. Requested video annotation features.
    #[prost(enumeration = "Feature", repeated, packed = "false", tag = "2")]
    pub features: ::prost::alloc::vec::Vec<i32>,
    /// Additional video context and/or feature-specific parameters.
    #[prost(message, optional, tag = "3")]
    pub video_context: ::core::option::Option<VideoContext>,
    /// Optional. Location where the output (in JSON format) should be stored.
    /// Currently, only [Google Cloud Storage](<https://cloud.google.com/storage/>)
    /// URIs are supported, which must be specified in the following format:
    /// `gs://bucket-id/object-id` (other URI formats return
    /// \[google.rpc.Code.INVALID_ARGUMENT][google.rpc.Code.INVALID_ARGUMENT\]). For
    /// more information, see [Request
    /// URIs](<https://cloud.google.com/storage/docs/request-endpoints>).
    #[prost(string, tag = "4")]
    pub output_uri: ::prost::alloc::string::String,
    /// Optional. Cloud region where annotation should take place. Supported cloud
    /// regions: `us-east1`, `us-west1`, `europe-west1`, `asia-east1`. If no region
    /// is specified, a region will be determined based on video file location.
    #[prost(string, tag = "5")]
    pub location_id: ::prost::alloc::string::String,
}
/// Video context and/or feature-specific parameters.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VideoContext {
    /// Video segments to annotate. The segments may overlap and are not required
    /// to be contiguous or span the whole video. If unspecified, each video is
    /// treated as a single segment.
    #[prost(message, repeated, tag = "1")]
    pub segments: ::prost::alloc::vec::Vec<VideoSegment>,
    /// Config for LABEL_DETECTION.
    #[prost(message, optional, tag = "2")]
    pub label_detection_config: ::core::option::Option<LabelDetectionConfig>,
    /// Config for SHOT_CHANGE_DETECTION.
    #[prost(message, optional, tag = "3")]
    pub shot_change_detection_config: ::core::option::Option<ShotChangeDetectionConfig>,
    /// Config for EXPLICIT_CONTENT_DETECTION.
    #[prost(message, optional, tag = "4")]
    pub explicit_content_detection_config: ::core::option::Option<ExplicitContentDetectionConfig>,
    /// Config for SPEECH_TRANSCRIPTION.
    #[prost(message, optional, tag = "6")]
    pub speech_transcription_config: ::core::option::Option<SpeechTranscriptionConfig>,
}
/// Config for LABEL_DETECTION.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LabelDetectionConfig {
    /// What labels should be detected with LABEL_DETECTION, in addition to
    /// video-level labels or segment-level labels.
    /// If unspecified, defaults to `SHOT_MODE`.
    #[prost(enumeration = "LabelDetectionMode", tag = "1")]
    pub label_detection_mode: i32,
    /// Whether the video has been shot from a stationary (i.e. non-moving) camera.
    /// When set to true, might improve detection accuracy for moving objects.
    /// Should be used with `SHOT_AND_FRAME_MODE` enabled.
    #[prost(bool, tag = "2")]
    pub stationary_camera: bool,
    /// Model to use for label detection.
    /// Supported values: "builtin/stable" (the default if unset) and
    /// "builtin/latest".
    #[prost(string, tag = "3")]
    pub model: ::prost::alloc::string::String,
}
/// Config for SHOT_CHANGE_DETECTION.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShotChangeDetectionConfig {
    /// Model to use for shot change detection.
    /// Supported values: "builtin/stable" (the default if unset) and
    /// "builtin/latest".
    #[prost(string, tag = "1")]
    pub model: ::prost::alloc::string::String,
}
/// Config for EXPLICIT_CONTENT_DETECTION.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExplicitContentDetectionConfig {
    /// Model to use for explicit content detection.
    /// Supported values: "builtin/stable" (the default if unset) and
    /// "builtin/latest".
    #[prost(string, tag = "1")]
    pub model: ::prost::alloc::string::String,
}
/// Video segment.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VideoSegment {
    /// Time-offset, relative to the beginning of the video,
    /// corresponding to the start of the segment (inclusive).
    #[prost(message, optional, tag = "1")]
    pub start_time_offset: ::core::option::Option<::prost_types::Duration>,
    /// Time-offset, relative to the beginning of the video,
    /// corresponding to the end of the segment (inclusive).
    #[prost(message, optional, tag = "2")]
    pub end_time_offset: ::core::option::Option<::prost_types::Duration>,
}
/// Video segment level annotation results for label detection.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LabelSegment {
    /// Video segment where a label was detected.
    #[prost(message, optional, tag = "1")]
    pub segment: ::core::option::Option<VideoSegment>,
    /// Confidence that the label is accurate. Range: [0, 1].
    #[prost(float, tag = "2")]
    pub confidence: f32,
}
/// Video frame level annotation results for label detection.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LabelFrame {
    /// Time-offset, relative to the beginning of the video, corresponding to the
    /// video frame for this location.
    #[prost(message, optional, tag = "1")]
    pub time_offset: ::core::option::Option<::prost_types::Duration>,
    /// Confidence that the label is accurate. Range: [0, 1].
    #[prost(float, tag = "2")]
    pub confidence: f32,
}
/// Detected entity from video analysis.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Entity {
    /// Opaque entity ID. Some IDs may be available in
    /// [Google Knowledge Graph Search
    /// API](<https://developers.google.com/knowledge-graph/>).
    #[prost(string, tag = "1")]
    pub entity_id: ::prost::alloc::string::String,
    /// Textual description, e.g. `Fixed-gear bicycle`.
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// Language code for `description` in BCP-47 format.
    #[prost(string, tag = "3")]
    pub language_code: ::prost::alloc::string::String,
}
/// Label annotation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LabelAnnotation {
    /// Detected entity.
    #[prost(message, optional, tag = "1")]
    pub entity: ::core::option::Option<Entity>,
    /// Common categories for the detected entity.
    /// E.g. when the label is `Terrier` the category is likely `dog`. And in some
    /// cases there might be more than one categories e.g. `Terrier` could also be
    /// a `pet`.
    #[prost(message, repeated, tag = "2")]
    pub category_entities: ::prost::alloc::vec::Vec<Entity>,
    /// All video segments where a label was detected.
    #[prost(message, repeated, tag = "3")]
    pub segments: ::prost::alloc::vec::Vec<LabelSegment>,
    /// All video frames where a label was detected.
    #[prost(message, repeated, tag = "4")]
    pub frames: ::prost::alloc::vec::Vec<LabelFrame>,
}
/// Video frame level annotation results for explicit content.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExplicitContentFrame {
    /// Time-offset, relative to the beginning of the video, corresponding to the
    /// video frame for this location.
    #[prost(message, optional, tag = "1")]
    pub time_offset: ::core::option::Option<::prost_types::Duration>,
    /// Likelihood of the pornography content..
    #[prost(enumeration = "Likelihood", tag = "2")]
    pub pornography_likelihood: i32,
}
/// Explicit content annotation (based on per-frame visual signals only).
/// If no explicit content has been detected in a frame, no annotations are
/// present for that frame.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExplicitContentAnnotation {
    /// All video frames where explicit content was detected.
    #[prost(message, repeated, tag = "1")]
    pub frames: ::prost::alloc::vec::Vec<ExplicitContentFrame>,
}
/// Annotation results for a single video.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VideoAnnotationResults {
    /// Output only. Video file location in
    /// [Google Cloud Storage](<https://cloud.google.com/storage/>).
    #[prost(string, tag = "1")]
    pub input_uri: ::prost::alloc::string::String,
    /// Label annotations on video level or user specified segment level.
    /// There is exactly one element for each unique label.
    #[prost(message, repeated, tag = "2")]
    pub segment_label_annotations: ::prost::alloc::vec::Vec<LabelAnnotation>,
    /// Label annotations on shot level.
    /// There is exactly one element for each unique label.
    #[prost(message, repeated, tag = "3")]
    pub shot_label_annotations: ::prost::alloc::vec::Vec<LabelAnnotation>,
    /// Label annotations on frame level.
    /// There is exactly one element for each unique label.
    #[prost(message, repeated, tag = "4")]
    pub frame_label_annotations: ::prost::alloc::vec::Vec<LabelAnnotation>,
    /// Shot annotations. Each shot is represented as a video segment.
    #[prost(message, repeated, tag = "6")]
    pub shot_annotations: ::prost::alloc::vec::Vec<VideoSegment>,
    /// Explicit content annotation.
    #[prost(message, optional, tag = "7")]
    pub explicit_annotation: ::core::option::Option<ExplicitContentAnnotation>,
    /// Speech transcription.
    #[prost(message, repeated, tag = "11")]
    pub speech_transcriptions: ::prost::alloc::vec::Vec<SpeechTranscription>,
    /// Output only. If set, indicates an error. Note that for a single
    /// `AnnotateVideoRequest` some videos may succeed and some may fail.
    #[prost(message, optional, tag = "9")]
    pub error: ::core::option::Option<super::super::super::rpc::Status>,
}
/// Video annotation response. Included in the `response`
/// field of the `Operation` returned by the `GetOperation`
/// call of the `google::longrunning::Operations` service.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnnotateVideoResponse {
    /// Annotation results for all videos specified in `AnnotateVideoRequest`.
    #[prost(message, repeated, tag = "1")]
    pub annotation_results: ::prost::alloc::vec::Vec<VideoAnnotationResults>,
}
/// Annotation progress for a single video.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VideoAnnotationProgress {
    /// Output only. Video file location in
    /// [Google Cloud Storage](<https://cloud.google.com/storage/>).
    #[prost(string, tag = "1")]
    pub input_uri: ::prost::alloc::string::String,
    /// Output only. Approximate percentage processed thus far. Guaranteed to be
    /// 100 when fully processed.
    #[prost(int32, tag = "2")]
    pub progress_percent: i32,
    /// Output only. Time when the request was received.
    #[prost(message, optional, tag = "3")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Time of the most recent update.
    #[prost(message, optional, tag = "4")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Video annotation progress. Included in the `metadata`
/// field of the `Operation` returned by the `GetOperation`
/// call of the `google::longrunning::Operations` service.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnnotateVideoProgress {
    /// Progress metadata for all videos specified in `AnnotateVideoRequest`.
    #[prost(message, repeated, tag = "1")]
    pub annotation_progress: ::prost::alloc::vec::Vec<VideoAnnotationProgress>,
}
/// Config for SPEECH_TRANSCRIPTION.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpeechTranscriptionConfig {
    /// Required. *Required* The language of the supplied audio as a
    /// \[BCP-47\](<https://www.rfc-editor.org/rfc/bcp/bcp47.txt>) language tag.
    /// Example: "en-US".
    /// See [Language Support](<https://cloud.google.com/speech/docs/languages>)
    /// for a list of the currently supported language codes.
    #[prost(string, tag = "1")]
    pub language_code: ::prost::alloc::string::String,
    /// Optional. Maximum number of recognition hypotheses to be returned.
    /// Specifically, the maximum number of `SpeechRecognitionAlternative` messages
    /// within each `SpeechTranscription`. The server may return fewer than
    /// `max_alternatives`. Valid values are `0`-`30`. A value of `0` or `1` will
    /// return a maximum of one. If omitted, will return a maximum of one.
    #[prost(int32, tag = "2")]
    pub max_alternatives: i32,
    /// Optional. If set to `true`, the server will attempt to filter out
    /// profanities, replacing all but the initial character in each filtered word
    /// with asterisks, e.g. "f***". If set to `false` or omitted, profanities
    /// won't be filtered out.
    #[prost(bool, tag = "3")]
    pub filter_profanity: bool,
    /// Optional. A means to provide context to assist the speech recognition.
    #[prost(message, repeated, tag = "4")]
    pub speech_contexts: ::prost::alloc::vec::Vec<SpeechContext>,
    /// Optional. If 'true', adds punctuation to recognition result hypotheses.
    /// This feature is only available in select languages. Setting this for
    /// requests in other languages has no effect at all. The default 'false' value
    /// does not add punctuation to result hypotheses. NOTE: "This is currently
    /// offered as an experimental service, complimentary to all users. In the
    /// future this may be exclusively available as a premium feature."
    #[prost(bool, tag = "5")]
    pub enable_automatic_punctuation: bool,
    /// Optional. For file formats, such as MXF or MKV, supporting multiple audio
    /// tracks, specify up to two tracks. Default: track 0.
    #[prost(int32, repeated, packed = "false", tag = "6")]
    pub audio_tracks: ::prost::alloc::vec::Vec<i32>,
}
/// Provides "hints" to the speech recognizer to favor specific words and phrases
/// in the results.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpeechContext {
    /// Optional. A list of strings containing words and phrases "hints" so that
    /// the speech recognition is more likely to recognize them. This can be used
    /// to improve the accuracy for specific words and phrases, for example, if
    /// specific commands are typically spoken by the user. This can also be used
    /// to add additional words to the vocabulary of the recognizer. See
    /// [usage limits](<https://cloud.google.com/speech/limits#content>).
    #[prost(string, repeated, tag = "1")]
    pub phrases: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// A speech recognition result corresponding to a portion of the audio.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpeechTranscription {
    /// May contain one or more recognition hypotheses (up to the maximum specified
    /// in `max_alternatives`).  These alternatives are ordered in terms of
    /// accuracy, with the top (first) alternative being the most probable, as
    /// ranked by the recognizer.
    #[prost(message, repeated, tag = "1")]
    pub alternatives: ::prost::alloc::vec::Vec<SpeechRecognitionAlternative>,
}
/// Alternative hypotheses (a.k.a. n-best list).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpeechRecognitionAlternative {
    /// Output only. Transcript text representing the words that the user spoke.
    #[prost(string, tag = "1")]
    pub transcript: ::prost::alloc::string::String,
    /// Output only. The confidence estimate between 0.0 and 1.0. A higher number
    /// indicates an estimated greater likelihood that the recognized words are
    /// correct. This field is set only for the top alternative.
    /// This field is not guaranteed to be accurate and users should not rely on it
    /// to be always provided.
    /// The default of 0.0 is a sentinel value indicating `confidence` was not set.
    #[prost(float, tag = "2")]
    pub confidence: f32,
    /// Output only. A list of word-specific information for each recognized word.
    #[prost(message, repeated, tag = "3")]
    pub words: ::prost::alloc::vec::Vec<WordInfo>,
}
/// Word-specific information for recognized words. Word information is only
/// included in the response when certain request parameters are set, such
/// as `enable_word_time_offsets`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WordInfo {
    /// Output only. Time offset relative to the beginning of the audio, and
    /// corresponding to the start of the spoken word. This field is only set if
    /// `enable_word_time_offsets=true` and only in the top hypothesis. This is an
    /// experimental feature and the accuracy of the time offset can vary.
    #[prost(message, optional, tag = "1")]
    pub start_time: ::core::option::Option<::prost_types::Duration>,
    /// Output only. Time offset relative to the beginning of the audio, and
    /// corresponding to the end of the spoken word. This field is only set if
    /// `enable_word_time_offsets=true` and only in the top hypothesis. This is an
    /// experimental feature and the accuracy of the time offset can vary.
    #[prost(message, optional, tag = "2")]
    pub end_time: ::core::option::Option<::prost_types::Duration>,
    /// Output only. The word corresponding to this set of information.
    #[prost(string, tag = "3")]
    pub word: ::prost::alloc::string::String,
}
/// Video annotation feature.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Feature {
    /// Unspecified.
    Unspecified = 0,
    /// Label detection. Detect objects, such as dog or flower.
    LabelDetection = 1,
    /// Shot change detection.
    ShotChangeDetection = 2,
    /// Explicit content detection.
    ExplicitContentDetection = 3,
    /// Speech transcription.
    SpeechTranscription = 6,
}
/// Label detection mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum LabelDetectionMode {
    /// Unspecified.
    Unspecified = 0,
    /// Detect shot-level labels.
    ShotMode = 1,
    /// Detect frame-level labels.
    FrameMode = 2,
    /// Detect both shot-level and frame-level labels.
    ShotAndFrameMode = 3,
}
/// Bucketized representation of likelihood.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Likelihood {
    /// Unspecified likelihood.
    Unspecified = 0,
    /// Very unlikely.
    VeryUnlikely = 1,
    /// Unlikely.
    Unlikely = 2,
    /// Possible.
    Possible = 3,
    /// Likely.
    Likely = 4,
    /// Very likely.
    VeryLikely = 5,
}
#[doc = r" Generated client implementations."]
pub mod video_intelligence_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Service that implements Google Cloud Video Intelligence API."]
    #[derive(Debug, Clone)]
    pub struct VideoIntelligenceServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> VideoIntelligenceServiceClient<T>
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
        ) -> VideoIntelligenceServiceClient<InterceptedService<T, F>>
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
            VideoIntelligenceServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Performs asynchronous video annotation. Progress and results can be"]
        #[doc = " retrieved through the `google.longrunning.Operations` interface."]
        #[doc = " `Operation.metadata` contains `AnnotateVideoProgress` (progress)."]
        #[doc = " `Operation.response` contains `AnnotateVideoResponse` (results)."]
        pub async fn annotate_video(
            &mut self,
            request: impl tonic::IntoRequest<super::AnnotateVideoRequest>,
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
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.videointelligence.v1p1beta1.VideoIntelligenceService/AnnotateVideo",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
