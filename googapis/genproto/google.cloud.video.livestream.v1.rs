/// Encoding of an input element such as an audio, video, or text track.
/// Elementary streams must be packaged before mapping and sharing between
/// different output formats.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ElementaryStream {
    /// A unique key for this elementary stream.
    #[prost(string, tag = "4")]
    pub key: ::prost::alloc::string::String,
    /// Required. Encoding of an audio, video, or text track.
    #[prost(oneof = "elementary_stream::ElementaryStream", tags = "1, 2, 3")]
    pub elementary_stream: ::core::option::Option<elementary_stream::ElementaryStream>,
}
/// Nested message and enum types in `ElementaryStream`.
pub mod elementary_stream {
    /// Required. Encoding of an audio, video, or text track.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ElementaryStream {
        /// Encoding of a video stream.
        #[prost(message, tag = "1")]
        VideoStream(super::VideoStream),
        /// Encoding of an audio stream.
        #[prost(message, tag = "2")]
        AudioStream(super::AudioStream),
        /// Encoding of a text stream. For example, closed captions or subtitles.
        #[prost(message, tag = "3")]
        TextStream(super::TextStream),
    }
}
/// Multiplexing settings for output stream.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MuxStream {
    /// A unique key for this multiplexed stream.
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
    /// The container format. The default is `fmp4`.
    ///
    /// Supported container formats:
    ///
    /// - `fmp4` - the corresponding file extension is `.m4s`
    /// - `ts` - the corresponding file extension is `.ts`
    #[prost(string, tag = "3")]
    pub container: ::prost::alloc::string::String,
    /// List of `ElementaryStream` \[key][google.cloud.video.livestream.v1.ElementaryStream.key\]s multiplexed in this
    /// stream.
    ///
    /// - For `fmp4` container, must contain either one video or one audio stream.
    /// - For `ts` container, must contain exactly one audio stream and up to one
    /// video stream.
    #[prost(string, repeated, tag = "4")]
    pub elementary_streams: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Segment settings for `fmp4` and `ts`.
    #[prost(message, optional, tag = "5")]
    pub segment_settings: ::core::option::Option<SegmentSettings>,
}
/// Manifest configuration.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Manifest {
    /// The name of the generated file. The default is `manifest` with the
    /// extension suffix corresponding to the `Manifest` \[type][google.cloud.video.livestream.v1.Manifest.type\]. If multiple
    /// manifests are added to the channel, each must have a unique file name.
    #[prost(string, tag = "1")]
    pub file_name: ::prost::alloc::string::String,
    /// Required. Type of the manifest, can be `HLS` or `DASH`.
    #[prost(enumeration = "manifest::ManifestType", tag = "2")]
    pub r#type: i32,
    /// Required. List of `MuxStream` \[key][google.cloud.video.livestream.v1.MuxStream.key\]s that should appear in this
    /// manifest.
    ///
    /// - For HLS, either `fmp4` or `ts` mux streams can be specified but not
    /// mixed.
    /// - For DASH, only `fmp4` mux streams can be specified.
    #[prost(string, repeated, tag = "3")]
    pub mux_streams: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Maximum number of segments that this manifest holds. Once the manifest
    /// reaches this maximum number of segments, whenever a new segment is added to
    /// the manifest, the oldest segment will be removed from the manifest.
    /// The minimum value is 3 and the default value is 5.
    #[prost(int32, tag = "4")]
    pub max_segment_count: i32,
    /// How long to keep a segment on the output Google Cloud Storage bucket after
    /// it is removed from the manifest. This field should be large enough to cover
    /// the manifest propagation delay. Otherwise, a player could receive 404
    /// errors while accessing segments which are listed in the manifest that the
    /// player has, but were already deleted from the output Google Cloud Storage
    /// bucket. Default value is `60s`.
    #[prost(message, optional, tag = "5")]
    pub segment_keep_duration: ::core::option::Option<::prost_types::Duration>,
}
/// Nested message and enum types in `Manifest`.
pub mod manifest {
    /// The manifest type can be either `HLS` or `DASH`.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ManifestType {
        /// The manifest type is not specified.
        Unspecified = 0,
        /// Create an `HLS` manifest. The corresponding file extension is `.m3u8`.
        Hls = 1,
        /// Create a `DASH` manifest. The corresponding file extension is `.mpd`.
        Dash = 2,
    }
}
/// Sprite sheet configuration.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpriteSheet {
    /// Format type. The default is `jpeg`.
    ///
    /// Supported formats:
    ///
    /// - `jpeg`
    #[prost(string, tag = "1")]
    pub format: ::prost::alloc::string::String,
    /// Required. File name prefix for the generated sprite sheets. If multiple
    /// sprite sheets are added to the channel, each must have a unique file
    /// prefix.
    /// Each sprite sheet has an incremental 10-digit zero-padded suffix starting
    /// from 0 before the extension, such as `sprite_sheet0000000123.jpeg`.
    #[prost(string, tag = "2")]
    pub file_prefix: ::prost::alloc::string::String,
    /// Required. The width of the sprite in pixels. Must be an even integer.
    #[prost(int32, tag = "3")]
    pub sprite_width_pixels: i32,
    /// Required. The height of the sprite in pixels. Must be an even integer.
    #[prost(int32, tag = "4")]
    pub sprite_height_pixels: i32,
    /// The maximum number of sprites per row in a sprite sheet. Valid range is
    /// [1, 10] and the default value is 1.
    #[prost(int32, tag = "5")]
    pub column_count: i32,
    /// The maximum number of rows per sprite sheet. When the sprite sheet is full,
    /// a new sprite sheet is created. Valid range is [1, 10] and the default value
    /// is 1.
    #[prost(int32, tag = "6")]
    pub row_count: i32,
    /// Create sprites at regular intervals. Valid range is [1 second, 1 hour] and
    /// the default value is `10s`.
    #[prost(message, optional, tag = "7")]
    pub interval: ::core::option::Option<::prost_types::Duration>,
    /// The quality of the generated sprite sheet. Enter a value between 1
    /// and 100, where 1 is the lowest quality and 100 is the highest quality.
    /// The default is 100. A high quality value corresponds to a low image data
    /// compression ratio.
    #[prost(int32, tag = "8")]
    pub quality: i32,
}
/// Preprocessing configurations.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PreprocessingConfig {
    /// Specify the video cropping configuration.
    #[prost(message, optional, tag = "2")]
    pub crop: ::core::option::Option<preprocessing_config::Crop>,
    /// Specify the video pad filter configuration.
    #[prost(message, optional, tag = "3")]
    pub pad: ::core::option::Option<preprocessing_config::Pad>,
}
/// Nested message and enum types in `PreprocessingConfig`.
pub mod preprocessing_config {
    /// Video cropping configuration for the input video. The cropped input video
    /// is scaled to match the output resolution.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Crop {
        /// The number of pixels to crop from the top. The default is 0.
        #[prost(int32, tag = "1")]
        pub top_pixels: i32,
        /// The number of pixels to crop from the bottom. The default is 0.
        #[prost(int32, tag = "2")]
        pub bottom_pixels: i32,
        /// The number of pixels to crop from the left. The default is 0.
        #[prost(int32, tag = "3")]
        pub left_pixels: i32,
        /// The number of pixels to crop from the right. The default is 0.
        #[prost(int32, tag = "4")]
        pub right_pixels: i32,
    }
    /// Pad filter configuration for the input video. The padded input video
    /// is scaled after padding with black to match the output resolution.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Pad {
        /// The number of pixels to add to the top. The default is 0.
        #[prost(int32, tag = "1")]
        pub top_pixels: i32,
        /// The number of pixels to add to the bottom. The default is 0.
        #[prost(int32, tag = "2")]
        pub bottom_pixels: i32,
        /// The number of pixels to add to the left. The default is 0.
        #[prost(int32, tag = "3")]
        pub left_pixels: i32,
        /// The number of pixels to add to the right. The default is 0.
        #[prost(int32, tag = "4")]
        pub right_pixels: i32,
    }
}
/// Video stream resource.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VideoStream {
    /// Codec settings.
    #[prost(oneof = "video_stream::CodecSettings", tags = "20")]
    pub codec_settings: ::core::option::Option<video_stream::CodecSettings>,
}
/// Nested message and enum types in `VideoStream`.
pub mod video_stream {
    /// H264 codec settings.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct H264CodecSettings {
        /// Required. The width of the video in pixels. Must be an even integer.
        /// Valid range is [320, 1920].
        #[prost(int32, tag = "1")]
        pub width_pixels: i32,
        /// Required. The height of the video in pixels. Must be an even integer.
        /// Valid range is [180, 1080].
        #[prost(int32, tag = "2")]
        pub height_pixels: i32,
        /// Required. The target video frame rate in frames per second (FPS). Must be less
        /// than or equal to 60. Will default to the input frame rate if larger
        /// than the input frame rate. The API will generate an output FPS that is
        /// divisible by the input FPS, and smaller or equal to the target FPS. See
        /// [Calculating frame
        /// rate](<https://cloud.google.com/transcoder/docs/concepts/frame-rate>) for
        /// more information.
        #[prost(double, tag = "3")]
        pub frame_rate: f64,
        /// Required. The video bitrate in bits per second. Minimum value is 10,000.
        ///
        /// - For SD resolution (< 720p), must be <= 3,000,000 (3 Mbps).
        /// - For HD resolution (<= 1080p), must be <= 15,000,000 (15 Mbps).
        #[prost(int32, tag = "4")]
        pub bitrate_bps: i32,
        /// Specifies whether an open Group of Pictures (GOP) structure should be
        /// allowed or not. The default is `false`.
        #[prost(bool, tag = "6")]
        pub allow_open_gop: bool,
        /// Size of the Video Buffering Verifier (VBV) buffer in bits. Must be
        /// greater than zero. The default is equal to \[bitrate_bps][google.cloud.video.livestream.v1.VideoStream.H264CodecSettings.bitrate_bps\].
        #[prost(int32, tag = "9")]
        pub vbv_size_bits: i32,
        /// Initial fullness of the Video Buffering Verifier (VBV) buffer in bits.
        /// Must be greater than zero. The default is equal to 90% of
        /// \[vbv_size_bits][google.cloud.video.livestream.v1.VideoStream.H264CodecSettings.vbv_size_bits\].
        #[prost(int32, tag = "10")]
        pub vbv_fullness_bits: i32,
        /// The entropy coder to use. The default is `cabac`.
        ///
        /// Supported entropy coders:
        ///
        /// - `cavlc`
        /// - `cabac`
        #[prost(string, tag = "11")]
        pub entropy_coder: ::prost::alloc::string::String,
        /// Allow B-pyramid for reference frame selection. This may not be supported
        /// on all decoders. The default is `false`.
        #[prost(bool, tag = "12")]
        pub b_pyramid: bool,
        /// The number of consecutive B-frames. Must be greater than or equal to
        /// zero. Must be less than \[gop_frame_count][google.cloud.video.livestream.v1.VideoStream.H264CodecSettings.gop_frame_count\] if set. The default
        /// is 0.
        #[prost(int32, tag = "13")]
        pub b_frame_count: i32,
        /// Specify the intensity of the adaptive quantizer (AQ). Must be between 0
        /// and 1, where 0 disables the quantizer and 1 maximizes the quantizer. A
        /// higher value equals a lower bitrate but smoother image. The default is 0.
        #[prost(double, tag = "14")]
        pub aq_strength: f64,
        /// Enforces the specified codec profile. The following profiles are
        /// supported:
        ///
        /// *   `baseline`
        /// *   `main` (default)
        /// *   `high`
        ///
        /// The available options are [FFmpeg-compatible Profile
        /// Options](<https://trac.ffmpeg.org/wiki/Encode/H.264#Profile>).
        /// Note that certain values for this field may cause the
        /// transcoder to override other fields you set in the \[H264CodecSettings][google.cloud.video.livestream.v1.VideoStream.H264CodecSettings\]
        /// message.
        #[prost(string, tag = "15")]
        pub profile: ::prost::alloc::string::String,
        /// Enforces the specified codec tune. The available options are
        /// [FFmpeg-compatible Encode
        /// Options](<https://trac.ffmpeg.org/wiki/Encode/H.264#Tune>)
        /// Note that certain values for this field may cause the transcoder to
        /// override other fields you set in the \[H264CodecSettings][google.cloud.video.livestream.v1.VideoStream.H264CodecSettings\] message.
        #[prost(string, tag = "16")]
        pub tune: ::prost::alloc::string::String,
        /// GOP mode can be either by frame count or duration.
        #[prost(oneof = "h264_codec_settings::GopMode", tags = "7, 8")]
        pub gop_mode: ::core::option::Option<h264_codec_settings::GopMode>,
    }
    /// Nested message and enum types in `H264CodecSettings`.
    pub mod h264_codec_settings {
        /// GOP mode can be either by frame count or duration.
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum GopMode {
            /// Select the GOP size based on the specified frame count.
            /// If GOP frame count is set instead of GOP duration, GOP duration will be
            /// calculated by `gopFrameCount`/`frameRate`. The calculated GOP duration
            /// must satisfy the limitations on `gopDuration` as well.
            /// Valid range is [60, 600].
            #[prost(int32, tag = "7")]
            GopFrameCount(i32),
            /// Select the GOP size based on the specified duration. The default is
            /// `2s`. Note that `gopDuration` must be less than or equal to
            /// \[segment_duration][google.cloud.video.livestream.v1.SegmentSettings.segment_duration\], and
            /// \[segment_duration][google.cloud.video.livestream.v1.SegmentSettings.segment_duration\] must be divisible
            /// by `gopDuration`. Valid range is [2s, 20s].
            ///
            /// All video streams in the same channel must have the same GOP size.
            #[prost(message, tag = "8")]
            GopDuration(::prost_types::Duration),
        }
    }
    /// Codec settings.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum CodecSettings {
        /// H264 codec settings.
        #[prost(message, tag = "20")]
        H264(H264CodecSettings),
    }
}
/// Audio stream resource.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AudioStream {
    /// Specifies whether pass through (transmuxing) is enabled or not.
    /// If set to `true`, the rest of the settings, other than `mapping`, will be
    /// ignored. The default is `false`.
    #[prost(bool, tag = "8")]
    pub transmux: bool,
    /// The codec for this audio stream. The default is `aac`.
    ///
    /// Supported audio codecs:
    ///
    /// - `aac`
    #[prost(string, tag = "1")]
    pub codec: ::prost::alloc::string::String,
    /// Required. Audio bitrate in bits per second. Must be between 1 and 10,000,000.
    #[prost(int32, tag = "2")]
    pub bitrate_bps: i32,
    /// Number of audio channels. Must be between 1 and 6. The default is 2.
    #[prost(int32, tag = "3")]
    pub channel_count: i32,
    /// A list of channel names specifying layout of the audio channels.
    /// This only affects the metadata embedded in the container headers, if
    /// supported by the specified format. The default is `[fl, fr]`.
    ///
    /// Supported channel names:
    ///
    /// - `fl` - Front left channel
    /// - `fr` - Front right channel
    /// - `sl` - Side left channel
    /// - `sr` - Side right channel
    /// - `fc` - Front center channel
    /// - `lfe` - Low frequency
    #[prost(string, repeated, tag = "4")]
    pub channel_layout: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The mapping for the input streams and audio channels.
    #[prost(message, repeated, tag = "5")]
    pub mapping: ::prost::alloc::vec::Vec<audio_stream::AudioMapping>,
    /// The audio sample rate in Hertz. The default is 48000 Hertz.
    #[prost(int32, tag = "6")]
    pub sample_rate_hertz: i32,
}
/// Nested message and enum types in `AudioStream`.
pub mod audio_stream {
    /// The mapping for the input streams and audio channels.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct AudioMapping {
        /// Required. The `Channel` \[InputAttachment.key][google.cloud.video.livestream.v1.InputAttachment.key\] that identifies the input that this
        /// audio mapping applies to. If an active input doesn't have an audio
        /// mapping, the primary audio track in the input stream will be selected.
        #[prost(string, tag = "6")]
        pub input_key: ::prost::alloc::string::String,
        /// Required. The zero-based index of the track in the input stream.
        /// All \[mapping][google.cloud.video.livestream.v1.AudioStream.mapping\]s in the same \[AudioStream][google.cloud.video.livestream.v1.AudioStream\]
        /// must have the same input track.
        #[prost(int32, tag = "2")]
        pub input_track: i32,
        /// Required. The zero-based index of the channel in the input stream.
        #[prost(int32, tag = "3")]
        pub input_channel: i32,
        /// Required. The zero-based index of the channel in the output audio stream.
        /// Must be consistent with the \[input_channel][google.cloud.video.livestream.v1.AudioStream.AudioMapping.input_channel\].
        #[prost(int32, tag = "4")]
        pub output_channel: i32,
    }
}
/// Encoding of a text stream. For example, closed captions or subtitles.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TextStream {
    /// Required. The codec for this text stream.
    ///
    /// Supported text codecs:
    ///
    /// - `cea608`
    /// - `cea708`
    #[prost(string, tag = "1")]
    pub codec: ::prost::alloc::string::String,
}
/// Segment settings for `fmp4` and `ts`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SegmentSettings {
    /// Duration of the segments in seconds. The default is `6s`. Note that
    /// `segmentDuration` must be greater than or equal to
    /// \[gop_duration][google.cloud.video.livestream.v1.VideoStream.H264CodecSettings.gop_duration\], and
    /// `segmentDuration` must be divisible by
    /// \[gop_duration][google.cloud.video.livestream.v1.VideoStream.H264CodecSettings.gop_duration\].
    /// Valid range is [2s, 20s].
    ///
    /// All \[mux_streams][google.cloud.video.livestream.v1.Manifest.mux_streams\] in the same manifest must have the
    /// same segment duration.
    #[prost(message, optional, tag = "1")]
    pub segment_duration: ::core::option::Option<::prost_types::Duration>,
}
/// Input resource represents the endpoint from which the channel ingests
/// the input stream.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Input {
    /// The resource name of the input, in the form of:
    /// `projects/{project}/locations/{location}/inputs/{inputId}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. The creation time.
    #[prost(message, optional, tag = "2")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The update time.
    #[prost(message, optional, tag = "3")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// User-defined key/value metadata.
    #[prost(map = "string, string", tag = "4")]
    pub labels:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Source type.
    #[prost(enumeration = "input::Type", tag = "5")]
    pub r#type: i32,
    /// Tier defines the maximum input specification that is accepted by the
    /// video pipeline. The billing is charged based on the tier specified here.
    /// See \[Pricing\](<https://cloud.google.com/livestream/pricing>) for more detail.
    /// The default is `HD`.
    #[prost(enumeration = "input::Tier", tag = "14")]
    pub tier: i32,
    /// Output only. URI to push the input stream to.
    /// Its format depends on the input \[type][google.cloud.video.livestream.v1.Input.type\], for example:
    ///
    /// *  `RTMP_PUSH`: `rtmp://1.2.3.4/live/{STREAM-ID}`
    /// *  `SRT_PUSH`: `srt://1.2.3.4:4201?streamid={STREAM-ID}`
    #[prost(string, tag = "6")]
    pub uri: ::prost::alloc::string::String,
    /// Preprocessing configurations.
    #[prost(message, optional, tag = "9")]
    pub preprocessing_config: ::core::option::Option<PreprocessingConfig>,
    /// Security rule for access control.
    #[prost(message, optional, tag = "12")]
    pub security_rules: ::core::option::Option<input::SecurityRule>,
    /// Output only. The information for the input stream. This field will be present only when
    /// this input receives the input stream.
    #[prost(message, optional, tag = "15")]
    pub input_stream_property: ::core::option::Option<InputStreamProperty>,
}
/// Nested message and enum types in `Input`.
pub mod input {
    /// Security rules for access control. Each field represents one security rule.
    /// Only when the source of the input stream satisfies all the fields, this
    /// input stream can be accepted.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SecurityRule {
        /// At least one ip range must match unless none specified. The IP range is
        /// defined by CIDR block: for example, `192.0.1.0/24` for a range and
        /// `192.0.1.0/32` for a single IP address.
        #[prost(string, repeated, tag = "1")]
        pub ip_ranges: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
    /// The type of the input.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Type {
        /// Input type is not specified.
        Unspecified = 0,
        /// Input will take an rtmp input stream.
        RtmpPush = 1,
        /// Input will take an srt (Secure Reliable Transport) input stream.
        SrtPush = 2,
    }
    /// Tier of the input specification.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Tier {
        /// Tier is not specified.
        Unspecified = 0,
        /// Resolution < 1280x720. Bitrate <= 6 Mbps. FPS <= 60.
        Sd = 1,
        /// Resolution <= 1920x1080. Bitrate <= 25 Mbps. FPS <= 60.
        Hd = 2,
        /// Resolution <= 4096x2160. Not supported yet.
        Uhd = 3,
    }
}
/// Channel resource represents the processor that does a user-defined
/// "streaming" operation, which includes getting an input stream through an
/// input, transcoding it to multiple renditions, and publishing output live
/// streams in certain formats (for example, HLS or DASH) to the specified
/// location.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Channel {
    /// The resource name of the channel, in the form of:
    /// `projects/{project}/locations/{location}/channels/{channelId}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. The creation time.
    #[prost(message, optional, tag = "2")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The update time.
    #[prost(message, optional, tag = "3")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// User-defined key/value metadata.
    #[prost(map = "string, string", tag = "4")]
    pub labels:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// A list of input attachments that this channel uses.
    /// One channel can have multiple inputs as the input sources. Only one
    /// input can be selected as the input source at one time.
    #[prost(message, repeated, tag = "16")]
    pub input_attachments: ::prost::alloc::vec::Vec<InputAttachment>,
    /// Output only. The \[InputAttachment.key][google.cloud.video.livestream.v1.InputAttachment.key\] that serves as the current input source. The
    /// first input in the \[input_attachments][google.cloud.video.livestream.v1.Channel.input_attachments\] is the initial input source.
    #[prost(string, tag = "6")]
    pub active_input: ::prost::alloc::string::String,
    /// Required. Information about the output (that is, the Cloud Storage bucket to store
    /// the generated live stream).
    #[prost(message, optional, tag = "9")]
    pub output: ::core::option::Option<channel::Output>,
    /// List of elementary streams.
    #[prost(message, repeated, tag = "10")]
    pub elementary_streams: ::prost::alloc::vec::Vec<ElementaryStream>,
    /// List of multiplexing settings for output streams.
    #[prost(message, repeated, tag = "11")]
    pub mux_streams: ::prost::alloc::vec::Vec<MuxStream>,
    /// List of output manifests.
    #[prost(message, repeated, tag = "12")]
    pub manifests: ::prost::alloc::vec::Vec<Manifest>,
    /// List of output sprite sheets.
    #[prost(message, repeated, tag = "13")]
    pub sprite_sheets: ::prost::alloc::vec::Vec<SpriteSheet>,
    /// Output only. State of the streaming operation.
    #[prost(enumeration = "channel::StreamingState", tag = "14")]
    pub streaming_state: i32,
    /// Output only. A description of the reason for the streaming error. This property is
    /// always present when \[streaming_state][google.cloud.video.livestream.v1.Channel.streaming_state\] is
    /// \[STREAMING_ERROR][google.cloud.video.livestream.v1.Channel.StreamingState.STREAMING_ERROR\].
    #[prost(message, optional, tag = "18")]
    pub streaming_error: ::core::option::Option<super::super::super::super::rpc::Status>,
    /// Configuration of platform logs for this channel.
    #[prost(message, optional, tag = "19")]
    pub log_config: ::core::option::Option<LogConfig>,
}
/// Nested message and enum types in `Channel`.
pub mod channel {
    /// Location of output file(s) in a Google Cloud Storage bucket.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Output {
        /// URI for the output file(s). For example, `gs://my-bucket/outputs/`.
        #[prost(string, tag = "1")]
        pub uri: ::prost::alloc::string::String,
    }
    /// State of streaming operation that the channel is running.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum StreamingState {
        /// Streaming state is not specified.
        Unspecified = 0,
        /// Channel is getting the input stream, generating the live streams to the
        /// specified output location.
        Streaming = 1,
        /// Channel is waiting for the input stream through the input.
        AwaitingInput = 2,
        /// Channel is running, but has trouble publishing the live streams onto the
        /// specified output location (for example, the specified Cloud Storage
        /// bucket is not writable).
        StreamingError = 4,
        /// Channel is generating live streams with no input stream. Live streams are
        /// filled out with black screen, while input stream is missing.
        /// Not supported yet.
        StreamingNoInput = 5,
        /// Channel is stopped, finishing live streams.
        Stopped = 6,
        /// Channel is starting.
        Starting = 7,
        /// Channel is stopping.
        Stopping = 8,
    }
}
/// Configuration of platform logs.
/// See [Using and managing platform
/// logs](<https://cloud.google.com/logging/docs/api/platform-logs#managing-logs>)
/// for more information about how to view platform logs through Cloud Logging.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LogConfig {
    /// The severity level of platform logging for this resource.
    #[prost(enumeration = "log_config::LogSeverity", tag = "1")]
    pub log_severity: i32,
}
/// Nested message and enum types in `LogConfig`.
pub mod log_config {
    /// The severity level of platform logging for this channel. Logs with a
    /// severity level higher than or equal to the chosen severity level will be
    /// logged and can be viewed through Cloud Logging.
    /// The severity level of a log is ranked as followed from low to high: DEBUG <
    /// INFO < NOTICE < WARNING < ERROR < CRITICAL < ALERT < EMERGENCY.
    /// See
    /// \[LogSeverity\](<https://cloud.google.com/logging/docs/reference/v2/rest/v2/LogEntry#logseverity>)
    /// for more information.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum LogSeverity {
        /// Log severity is not specified. This is the same as log severity is OFF.
        Unspecified = 0,
        /// Log is turned off.
        Off = 1,
        /// Log with severity higher than or equal to DEBUG are logged.
        Debug = 100,
        /// Logs with severity higher than or equal to INFO are logged.
        Info = 200,
        /// Logs with severity higher than or equal to WARNING are logged.
        Warning = 400,
        /// Logs with severity higher than or equal to ERROR are logged.
        Error = 500,
    }
}
/// Properties of the input stream.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InputStreamProperty {
    /// The time that the current input stream is accepted and the connection is
    /// established.
    #[prost(message, optional, tag = "1")]
    pub last_establish_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Properties of the video streams.
    #[prost(message, repeated, tag = "2")]
    pub video_streams: ::prost::alloc::vec::Vec<VideoStreamProperty>,
    /// Properties of the audio streams.
    #[prost(message, repeated, tag = "3")]
    pub audio_streams: ::prost::alloc::vec::Vec<AudioStreamProperty>,
}
/// Properties of the video stream.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VideoStreamProperty {
    /// Index of this video stream.
    #[prost(int32, tag = "1")]
    pub index: i32,
    /// Properties of the video format.
    #[prost(message, optional, tag = "2")]
    pub video_format: ::core::option::Option<VideoFormat>,
}
/// Properties of the video format.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VideoFormat {
    /// Video codec used in this video stream.
    #[prost(string, tag = "1")]
    pub codec: ::prost::alloc::string::String,
    /// The width of the video stream in pixels.
    #[prost(int32, tag = "2")]
    pub width_pixels: i32,
    /// The height of the video stream in pixels.
    #[prost(int32, tag = "3")]
    pub height_pixels: i32,
    /// The frame rate of the input video stream.
    #[prost(double, tag = "4")]
    pub frame_rate: f64,
}
/// Properties of the audio stream.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AudioStreamProperty {
    /// Index of this audio stream.
    #[prost(int32, tag = "1")]
    pub index: i32,
    /// Properties of the audio format.
    #[prost(message, optional, tag = "2")]
    pub audio_format: ::core::option::Option<AudioFormat>,
}
/// Properties of the audio format.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AudioFormat {
    /// Audio codec used in this audio stream.
    #[prost(string, tag = "1")]
    pub codec: ::prost::alloc::string::String,
    /// The number of audio channels.
    #[prost(int32, tag = "2")]
    pub channel_count: i32,
    /// A list of channel names specifying the layout of the audio channels.
    #[prost(string, repeated, tag = "3")]
    pub channel_layout: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// A group of information for attaching an input resource to this channel.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InputAttachment {
    /// A unique key for this input attachment.
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
    /// The resource name of an existing input, in the form of:
    /// `projects/{project}/locations/{location}/inputs/{inputId}`.
    #[prost(string, tag = "2")]
    pub input: ::prost::alloc::string::String,
    /// Automatic failover configurations.
    #[prost(message, optional, tag = "3")]
    pub automatic_failover: ::core::option::Option<input_attachment::AutomaticFailover>,
}
/// Nested message and enum types in `InputAttachment`.
pub mod input_attachment {
    /// Configurations to follow when automatic failover happens.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct AutomaticFailover {
        /// The \[InputAttachment.key][google.cloud.video.livestream.v1.InputAttachment.key\]s of inputs to failover to when this input is
        /// disconnected. Currently, only up to one backup input is supported.
        #[prost(string, repeated, tag = "1")]
        pub input_keys: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
}
/// Event is a sub-resource of a channel, which can be scheduled by the user to
/// execute operations on a channel resource without having to stop the channel.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Event {
    /// The resource name of the event, in the form of:
    /// `projects/{project}/locations/{location}/channels/{channelId}/events/{eventId}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. The creation time.
    #[prost(message, optional, tag = "2")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The update time.
    #[prost(message, optional, tag = "3")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// User-defined key/value metadata.
    #[prost(map = "string, string", tag = "4")]
    pub labels:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// When this field is set to true, the event will be executed at the earliest
    /// time that the server can schedule the event and \[execution_time][google.cloud.video.livestream.v1.Event.execution_time\] will be
    /// populated with the time that the server actually schedules the event.
    #[prost(bool, tag = "9")]
    pub execute_now: bool,
    /// The time when the event should be executed. When \[execute_now][google.cloud.video.livestream.v1.Event.execute_now\] is set to
    /// `true`, this field should not be set in `CreateEvent` request and will be
    /// populated with the time that the server schedules the event.
    #[prost(message, optional, tag = "10")]
    pub execution_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The state of the event.
    #[prost(enumeration = "event::State", tag = "11")]
    pub state: i32,
    /// Output only. An error object that describes the reason for the failure.
    /// This property is always present when `state` is `FAILED`.
    #[prost(message, optional, tag = "12")]
    pub error: ::core::option::Option<super::super::super::super::rpc::Status>,
    /// Required. Operation to be executed by this event.
    #[prost(oneof = "event::Task", tags = "6")]
    pub task: ::core::option::Option<event::Task>,
}
/// Nested message and enum types in `Event`.
pub mod event {
    /// Inserts a new ad opportunity.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct AdBreakTask {
        /// Duration of an ad opportunity. Must be greater than 0.
        #[prost(message, optional, tag = "1")]
        pub duration: ::core::option::Option<::prost_types::Duration>,
    }
    /// State of the event
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// Event state is not specified.
        Unspecified = 0,
        /// Event is scheduled but not executed yet.
        Scheduled = 1,
        /// Event is being executed.
        Running = 2,
        /// Event has been successfully executed.
        Succeeded = 3,
        /// Event fails to be executed.
        Failed = 4,
        /// Event has been created but not scheduled yet.
        Pending = 5,
        /// Event was stopped before running for its full duration.
        Stopped = 6,
    }
    /// Required. Operation to be executed by this event.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Task {
        /// Required. Inserts a new ad opportunity.
        #[prost(message, tag = "6")]
        AdBreak(AdBreakTask),
    }
}
/// Request message for "LivestreamService.CreateChannel".
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateChannelRequest {
    /// Required. The parent location for the resource, in the form of:
    /// `projects/{project}/locations/{location}`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The channel resource to be created.
    #[prost(message, optional, tag = "2")]
    pub channel: ::core::option::Option<Channel>,
    /// Required. The ID of the channel resource to be created.
    /// This value must be 1-63 characters, begin and end with `\[a-z0-9\]`,
    /// could contain dashes (-) in between.
    #[prost(string, tag = "3")]
    pub channel_id: ::prost::alloc::string::String,
    /// A request ID to identify requests. Specify a unique request ID
    /// so that if you must retry your request, the server will know to ignore
    /// the request if it has already been completed. The server will guarantee
    /// that for at least 60 minutes since the first request.
    ///
    /// For example, consider a situation where you make an initial request and the
    /// request times out. If you make the request again with the same request ID,
    /// the server can check if original operation with the same request ID was
    /// received, and if so, will ignore the second request. This prevents clients
    /// from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported `(00000000-0000-0000-0000-000000000000)`.
    #[prost(string, tag = "4")]
    pub request_id: ::prost::alloc::string::String,
}
/// Request message for "LivestreamService.ListChannels".
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListChannelsRequest {
    /// Required. The parent location for the resource, in the form of:
    /// `projects/{project}/locations/{location}`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of items to return. If unspecified, server
    /// will pick an appropriate default. Server may return fewer items than
    /// requested. A caller should only rely on response's
    /// \[next_page_token][google.cloud.video.livestream.v1.ListChannelsResponse.next_page_token\] to
    /// determine if there are more items left to be queried.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The next_page_token value returned from a previous List request, if any.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// The filter to apply to list results.
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Specifies the ordering of results following syntax at
    /// <https://cloud.google.com/apis/design/design_patterns#sorting_order.>
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
/// Response message for "LivestreamService.ListChannels".
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListChannelsResponse {
    /// A list of channels.
    #[prost(message, repeated, tag = "1")]
    pub channels: ::prost::alloc::vec::Vec<Channel>,
    /// Token to retrieve the next page of results, or empty if there are no more
    /// results in the list.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Request message for "LivestreamService.GetChannel".
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetChannelRequest {
    /// Required. The name of the channel resource, in the form of:
    /// `projects/{project}/locations/{location}/channels/{channelId}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for "LivestreamService.DeleteChannel".
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteChannelRequest {
    /// Required. The name of the channel resource, in the form of:
    /// `projects/{project}/locations/{location}/channels/{channelId}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// A request ID to identify requests. Specify a unique request ID
    /// so that if you must retry your request, the server will know to ignore
    /// the request if it has already been completed. The server will guarantee
    /// that for at least 60 minutes after the first request.
    ///
    /// For example, consider a situation where you make an initial request and the
    /// request times out. If you make the request again with the same request ID,
    /// the server can check if original operation with the same request ID was
    /// received, and if so, will ignore the second request. This prevents clients
    /// from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported `(00000000-0000-0000-0000-000000000000)`.
    #[prost(string, tag = "2")]
    pub request_id: ::prost::alloc::string::String,
    /// If the `force` field is set to the default value of `false`, you must
    /// delete all of a channel's events before you can delete the channel itself.
    /// If the field is set to `true`, requests to delete a channel also delete
    /// associated channel events.
    #[prost(bool, tag = "3")]
    pub force: bool,
}
/// Request message for "LivestreamService.UpdateChannel".
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateChannelRequest {
    /// Field mask is used to specify the fields to be overwritten in the Channel
    /// resource by the update. You can only update the following fields:
    ///
    /// * \[`inputAttachments`\](<https://cloud.google.com/livestream/docs/reference/rest/v1/projects.locations.channels#inputattachment>)
    /// * \[`output`\](<https://cloud.google.com/livestream/docs/reference/rest/v1/projects.locations.channels#output>)
    /// * \[`elementaryStreams`\](<https://cloud.google.com/livestream/docs/reference/rest/v1/projects.locations.channels#ElementaryStream>)
    /// * \[`muxStreams`\](<https://cloud.google.com/livestream/docs/reference/rest/v1/projects.locations.channels#muxstream>)
    /// * \[`manifests`\](<https://cloud.google.com/livestream/docs/reference/rest/v1/projects.locations.channels#Manifest>)
    /// * \[`spritesheets`\](<https://cloud.google.com/livestream/docs/reference/rest/v1/projects.locations.channels#spritesheet>)
    ///
    /// The fields specified in the update_mask are relative to the resource, not
    /// the full request. A field will be overwritten if it is in the mask.
    #[prost(message, optional, tag = "1")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Required. The channel resource to be updated.
    #[prost(message, optional, tag = "2")]
    pub channel: ::core::option::Option<Channel>,
    /// A request ID to identify requests. Specify a unique request ID
    /// so that if you must retry your request, the server will know to ignore
    /// the request if it has already been completed. The server will guarantee
    /// that for at least 60 minutes since the first request.
    ///
    /// For example, consider a situation where you make an initial request and the
    /// request times out. If you make the request again with the same request ID,
    /// the server can check if original operation with the same request ID was
    /// received, and if so, will ignore the second request. This prevents clients
    /// from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported `(00000000-0000-0000-0000-000000000000)`.
    #[prost(string, tag = "3")]
    pub request_id: ::prost::alloc::string::String,
}
/// Request message for "LivestreamService.StartChannel".
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StartChannelRequest {
    /// Required. The name of the channel resource, in the form of:
    /// `projects/{project}/locations/{location}/channels/{channelId}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// A request ID to identify requests. Specify a unique request ID
    /// so that if you must retry your request, the server will know to ignore
    /// the request if it has already been completed. The server will guarantee
    /// that for at least 60 minutes since the first request.
    ///
    /// For example, consider a situation where you make an initial request and the
    /// request times out. If you make the request again with the same request ID,
    /// the server can check if original operation with the same request ID was
    /// received, and if so, will ignore the second request. This prevents clients
    /// from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported `(00000000-0000-0000-0000-000000000000)`.
    #[prost(string, tag = "2")]
    pub request_id: ::prost::alloc::string::String,
}
/// Request message for "LivestreamService.StopChannel".
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StopChannelRequest {
    /// Required. The name of the channel resource, in the form of:
    /// `projects/{project}/locations/{location}/channels/{channelId}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// A request ID to identify requests. Specify a unique request ID
    /// so that if you must retry your request, the server will know to ignore
    /// the request if it has already been completed. The server will guarantee
    /// that for at least 60 minutes since the first request.
    ///
    /// For example, consider a situation where you make an initial request and the
    /// request times out. If you make the request again with the same request ID,
    /// the server can check if original operation with the same request ID was
    /// received, and if so, will ignore the second request. This prevents clients
    /// from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported `(00000000-0000-0000-0000-000000000000)`.
    #[prost(string, tag = "2")]
    pub request_id: ::prost::alloc::string::String,
}
/// Request message for "LivestreamService.CreateInput".
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateInputRequest {
    /// Required. The parent location for the resource, in the form of:
    /// `projects/{project}/locations/{location}`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The input resource to be created.
    #[prost(message, optional, tag = "2")]
    pub input: ::core::option::Option<Input>,
    /// Required. The ID of the input resource to be created.
    /// This value must be 1-63 characters, begin and end with `\[a-z0-9\]`,
    /// could contain dashes (-) in between.
    #[prost(string, tag = "3")]
    pub input_id: ::prost::alloc::string::String,
    /// A request ID to identify requests. Specify a unique request ID
    /// so that if you must retry your request, the server will know to ignore
    /// the request if it has already been completed. The server will guarantee
    /// that for at least 60 minutes since the first request.
    ///
    /// For example, consider a situation where you make an initial request and the
    /// request times out. If you make the request again with the same request ID,
    /// the server can check if original operation with the same request ID was
    /// received, and if so, will ignore the second request. This prevents clients
    /// from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported `(00000000-0000-0000-0000-000000000000)`.
    #[prost(string, tag = "4")]
    pub request_id: ::prost::alloc::string::String,
}
/// Request message for "LivestreamService.ListInputs".
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListInputsRequest {
    /// Required. The parent location for the resource, in the form of:
    /// `projects/{project}/locations/{location}`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of items to return. If unspecified, server
    /// will pick an appropriate default. Server may return fewer items than
    /// requested. A caller should only rely on response's
    /// \[next_page_token][google.cloud.video.livestream.v1.ListInputsResponse.next_page_token\] to
    /// determine if there are more items left to be queried.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The next_page_token value returned from a previous List request, if any.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// The filter to apply to list results.
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Specifies the ordering of results following syntax at [Sorting
    /// Order](<https://cloud.google.com/apis/design/design_patterns#sorting_order>).
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
/// Response message for "LivestreamService.ListInputs".
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListInputsResponse {
    /// A list of inputs.
    #[prost(message, repeated, tag = "1")]
    pub inputs: ::prost::alloc::vec::Vec<Input>,
    /// Token to retrieve the next page of results, or empty if there are no more
    /// results in the list.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Request message for "LivestreamService.GetInput".
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetInputRequest {
    /// Required. The name of the input resource, in the form of:
    /// `projects/{project}/locations/{location}/inputs/{inputId}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for "LivestreamService.DeleteInput".
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteInputRequest {
    /// Required. The name of the input resource, in the form of:
    /// `projects/{project}/locations/{location}/inputs/{inputId}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// A request ID to identify requests. Specify a unique request ID
    /// so that if you must retry your request, the server will know to ignore
    /// the request if it has already been completed. The server will guarantee
    /// that for at least 60 minutes since the first request.
    ///
    /// For example, consider a situation where you make an initial request and the
    /// request times out. If you make the request again with the same request ID,
    /// the server can check if original operation with the same request ID was
    /// received, and if so, will ignore the second request. This prevents clients
    /// from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported `(00000000-0000-0000-0000-000000000000)`.
    #[prost(string, tag = "2")]
    pub request_id: ::prost::alloc::string::String,
}
/// Request message for "LivestreamService.UpdateInput".
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateInputRequest {
    /// Field mask is used to specify the fields to be overwritten in the Input
    /// resource by the update. You can only update the following fields:
    ///
    /// * \[`preprocessingConfig`\](<https://cloud.google.com/livestream/docs/reference/rest/v1/projects.locations.inputs#PreprocessingConfig>)
    /// * \[`securityRules`\](<https://cloud.google.com/livestream/docs/reference/rest/v1/projects.locations.inputs#SecurityRule>)
    ///
    /// The fields specified in the update_mask are relative to the resource, not
    /// the full request. A field will be overwritten if it is in the mask.
    #[prost(message, optional, tag = "1")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Required. The input resource to be updated.
    #[prost(message, optional, tag = "2")]
    pub input: ::core::option::Option<Input>,
    /// A request ID to identify requests. Specify a unique request ID
    /// so that if you must retry your request, the server will know to ignore
    /// the request if it has already been completed. The server will guarantee
    /// that for at least 60 minutes since the first request.
    ///
    /// For example, consider a situation where you make an initial request and the
    /// request times out. If you make the request again with the same request ID,
    /// the server can check if original operation with the same request ID was
    /// received, and if so, will ignore the second request. This prevents clients
    /// from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported `(00000000-0000-0000-0000-000000000000)`.
    #[prost(string, tag = "3")]
    pub request_id: ::prost::alloc::string::String,
}
/// Request message for "LivestreamService.CreateEvent".
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateEventRequest {
    /// Required. The parent channel for the resource, in the form of:
    /// `projects/{project}/locations/{location}/channels/{channelId}`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The event resource to be created.
    #[prost(message, optional, tag = "2")]
    pub event: ::core::option::Option<Event>,
    /// Required. The ID of the event resource to be created.
    /// This value must be 1-63 characters, begin and end with `\[a-z0-9\]`,
    /// could contain dashes (-) in between.
    #[prost(string, tag = "3")]
    pub event_id: ::prost::alloc::string::String,
    /// A request ID to identify requests. Specify a unique request ID
    /// so that if you must retry your request, the server will know to ignore
    /// the request if it has already been completed. The server will guarantee
    /// that for at least 60 minutes since the first request.
    ///
    /// For example, consider a situation where you make an initial request and the
    /// request times out. If you make the request again with the same request ID,
    /// the server can check if original operation with the same request ID was
    /// received, and if so, will ignore the second request. This prevents clients
    /// from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported `(00000000-0000-0000-0000-000000000000)`.
    #[prost(string, tag = "4")]
    pub request_id: ::prost::alloc::string::String,
}
/// Request message for "LivestreamService.ListEvents".
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListEventsRequest {
    /// Required. The parent channel for the resource, in the form of:
    /// `projects/{project}/locations/{location}/channels/{channelId}`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of items to return. If unspecified, server
    /// will pick an appropriate default. Server may return fewer items than
    /// requested. A caller should only rely on response's
    /// \[next_page_token][google.cloud.video.livestream.v1.ListEventsResponse.next_page_token\] to
    /// determine if there are more items left to be queried.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The next_page_token value returned from a previous List request, if any.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// The filter to apply to list results.
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Specifies the ordering of results following syntax at
    /// <https://cloud.google.com/apis/design/design_patterns#sorting_order.>
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
/// Response message for "LivestreamService.ListEvents".
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListEventsResponse {
    /// A list of events.
    #[prost(message, repeated, tag = "1")]
    pub events: ::prost::alloc::vec::Vec<Event>,
    /// Token to retrieve the next page of results, or empty if there are no more
    /// results in the list.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Request message for "LivestreamService.GetEvent".
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetEventRequest {
    /// Required. The name of the event resource, in the form of:
    /// `projects/{project}/locations/{location}/channels/{channelId}/events/{eventId}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for "LivestreamService.DeleteEvent".
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteEventRequest {
    /// Required. The name of the event resource, in the form of:
    /// `projects/{project}/locations/{location}/channels/{channelId}/events/{eventId}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// A request ID to identify requests. Specify a unique request ID
    /// so that if you must retry your request, the server will know to ignore
    /// the request if it has already been completed. The server will guarantee
    /// that for at least 60 minutes since the first request.
    ///
    /// For example, consider a situation where you make an initial request and the
    /// request times out. If you make the request again with the same request ID,
    /// the server can check if original operation with the same request ID was
    /// received, and if so, will ignore the second request. This prevents clients
    /// from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported `(00000000-0000-0000-0000-000000000000)`.
    #[prost(string, tag = "2")]
    pub request_id: ::prost::alloc::string::String,
}
/// Response message for Start/Stop Channel long-running operations.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelOperationResponse {}
/// Represents the metadata of the long-running operation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationMetadata {
    /// Output only. The time the operation was created.
    #[prost(message, optional, tag = "1")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The time the operation finished running.
    #[prost(message, optional, tag = "2")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Server-defined resource path for the target of the operation.
    #[prost(string, tag = "3")]
    pub target: ::prost::alloc::string::String,
    /// Output only. Name of the verb executed by the operation.
    #[prost(string, tag = "4")]
    pub verb: ::prost::alloc::string::String,
    /// Output only. Identifies whether the user has requested cancellation
    /// of the operation. Operations that have successfully been cancelled
    /// have \[Operation.error][\] value with a
    /// \[google.rpc.Status.code][google.rpc.Status.code\] of 1, corresponding to
    /// `Code.CANCELLED`.
    #[prost(bool, tag = "5")]
    pub requested_cancellation: bool,
    /// Output only. API version used to start the operation.
    #[prost(string, tag = "6")]
    pub api_version: ::prost::alloc::string::String,
}
#[doc = r" Generated client implementations."]
pub mod livestream_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Using Live Stream API, you can generate live streams in the various"]
    #[doc = " renditions and streaming formats. The streaming format include HTTP Live"]
    #[doc = " Streaming (HLS) and Dynamic Adaptive Streaming over HTTP (DASH). You can send"]
    #[doc = " a source stream in the various ways, including Real-Time Messaging"]
    #[doc = " Protocol (RTMP) and Secure Reliable Transport (SRT)."]
    #[derive(Debug, Clone)]
    pub struct LivestreamServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> LivestreamServiceClient<T>
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
        ) -> LivestreamServiceClient<InterceptedService<T, F>>
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
            LivestreamServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Creates a channel with the provided unique ID in the specified"]
        #[doc = " region."]
        pub async fn create_channel(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateChannelRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.video.livestream.v1.LivestreamService/CreateChannel",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns a list of all channels in the specified region."]
        pub async fn list_channels(
            &mut self,
            request: impl tonic::IntoRequest<super::ListChannelsRequest>,
        ) -> Result<tonic::Response<super::ListChannelsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.video.livestream.v1.LivestreamService/ListChannels",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns the specified channel."]
        pub async fn get_channel(
            &mut self,
            request: impl tonic::IntoRequest<super::GetChannelRequest>,
        ) -> Result<tonic::Response<super::Channel>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.video.livestream.v1.LivestreamService/GetChannel",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes the specified channel."]
        pub async fn delete_channel(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteChannelRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.video.livestream.v1.LivestreamService/DeleteChannel",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates the specified channel."]
        pub async fn update_channel(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateChannelRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.video.livestream.v1.LivestreamService/UpdateChannel",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Starts the specified channel. Part of the video pipeline will be created"]
        #[doc = " only when the StartChannel request is received by the server."]
        pub async fn start_channel(
            &mut self,
            request: impl tonic::IntoRequest<super::StartChannelRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.video.livestream.v1.LivestreamService/StartChannel",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Stops the specified channel. Part of the video pipeline will be released"]
        #[doc = " when the StopChannel request is received by the server."]
        pub async fn stop_channel(
            &mut self,
            request: impl tonic::IntoRequest<super::StopChannelRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.video.livestream.v1.LivestreamService/StopChannel",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates an input with the provided unique ID in the specified region."]
        pub async fn create_input(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateInputRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.video.livestream.v1.LivestreamService/CreateInput",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns a list of all inputs in the specified region."]
        pub async fn list_inputs(
            &mut self,
            request: impl tonic::IntoRequest<super::ListInputsRequest>,
        ) -> Result<tonic::Response<super::ListInputsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.video.livestream.v1.LivestreamService/ListInputs",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns the specified input."]
        pub async fn get_input(
            &mut self,
            request: impl tonic::IntoRequest<super::GetInputRequest>,
        ) -> Result<tonic::Response<super::Input>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.video.livestream.v1.LivestreamService/GetInput",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes the specified input."]
        pub async fn delete_input(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteInputRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.video.livestream.v1.LivestreamService/DeleteInput",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates the specified input."]
        pub async fn update_input(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateInputRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.video.livestream.v1.LivestreamService/UpdateInput",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates an event with the provided unique ID in the specified channel."]
        pub async fn create_event(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateEventRequest>,
        ) -> Result<tonic::Response<super::Event>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.video.livestream.v1.LivestreamService/CreateEvent",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns a list of all events in the specified channel."]
        pub async fn list_events(
            &mut self,
            request: impl tonic::IntoRequest<super::ListEventsRequest>,
        ) -> Result<tonic::Response<super::ListEventsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.video.livestream.v1.LivestreamService/ListEvents",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns the specified event."]
        pub async fn get_event(
            &mut self,
            request: impl tonic::IntoRequest<super::GetEventRequest>,
        ) -> Result<tonic::Response<super::Event>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.video.livestream.v1.LivestreamService/GetEvent",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes the specified event."]
        pub async fn delete_event(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteEventRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.video.livestream.v1.LivestreamService/DeleteEvent",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
