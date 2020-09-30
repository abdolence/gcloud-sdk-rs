/// Transcoding job resource.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Job {
    /// The resource name of the job.
    /// Format: `projects/{project}/locations/{location}/jobs/{job}`
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Input only. Specify the `input_uri` to populate empty `uri` fields in each element of
    /// `Job.config.inputs` or `JobTemplate.config.inputs` when using template.
    /// URI of the media. It must be stored in Cloud Storage. For example,
    /// `gs://bucket/inputs/file.mp4`.
    #[prost(string, tag = "2")]
    pub input_uri: std::string::String,
    /// Input only. Specify the `output_uri` to populate an empty `Job.config.output.uri` or
    /// `JobTemplate.config.output.uri` when using template.
    /// URI for the output file(s). For example, `gs://my-bucket/outputs/`.
    #[prost(string, tag = "3")]
    pub output_uri: std::string::String,
    /// Specify the priority of the job. Enter a value between 0 and 100, where 0
    /// is the lowest priority and 100 is the highest priority. The default is 0.
    #[prost(int32, tag = "6")]
    pub priority: i32,
    /// Output only. The origin URI.
    #[prost(message, optional, tag = "7")]
    pub origin_uri: ::std::option::Option<job::OriginUri>,
    /// Output only. The current state of the job.
    #[prost(enumeration = "job::ProcessingState", tag = "8")]
    pub state: i32,
    /// Output only. Estimated fractional progress, from `0` to `1` for each
    /// step.
    #[prost(message, optional, tag = "9")]
    pub progress: ::std::option::Option<Progress>,
    /// Output only. A description of the reason for the failure. This property is
    /// always present when `state` is `FAILED`.
    #[prost(string, tag = "10")]
    pub failure_reason: std::string::String,
    /// Output only. List of failure details. This property may contain additional
    /// information about the failure when `failure_reason` is present.
    #[prost(message, repeated, tag = "11")]
    pub failure_details: ::std::vec::Vec<FailureDetail>,
    /// Specify the `job_config` for transcoding job. When you use a `template_id`
    /// to create a job, the `Job.config` is populated by the `JobTemplate.config`.
    #[prost(oneof = "job::JobConfig", tags = "4, 5")]
    pub job_config: ::std::option::Option<job::JobConfig>,
}
pub mod job {
    /// The origin URI.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct OriginUri {
        /// HLS master manifest URI. If multiple HLS master manifests are created
        /// only first one is listed.
        #[prost(string, tag = "1")]
        pub hls: std::string::String,
        /// Dash manifest URI. If multiple Dash manifests are created, only the first
        /// one is listed.
        #[prost(string, tag = "2")]
        pub dash: std::string::String,
    }
    /// The current state of the job.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ProcessingState {
        /// The processing state is not specified.
        Unspecified = 0,
        /// The job is enqueued and will be picked up for processing soon.
        Pending = 1,
        /// The job is being processed.
        Running = 2,
        /// The job has been completed successfully.
        Succeeded = 3,
        /// The job has failed. For additional information, see `failure_reason` and
        /// `failure_details`
        Failed = 4,
    }
    /// Specify the `job_config` for transcoding job. When you use a `template_id`
    /// to create a job, the `Job.config` is populated by the `JobTemplate.config`.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum JobConfig {
        /// Input only. Specify the `template_id` to use for populating `Job.config`. The default
        /// is `preset/web-hd`.
        ///
        /// Preset Transcoder templates:
        /// - `preset/{preset_id}`
        ///
        /// - User defined JobTemplate:
        ///   `{job_template_id}`
        #[prost(string, tag = "4")]
        TemplateId(std::string::String),
        /// The configuration for this job.
        #[prost(message, tag = "5")]
        Config(super::JobConfig),
    }
}
/// Transcoding job template resource.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct JobTemplate {
    /// The resource name of the job template.
    /// Format:
    /// `projects/{project}/locations/{location}/jobTemplates/{job_template}`
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// The configuration for this template.
    #[prost(message, optional, tag = "2")]
    pub config: ::std::option::Option<JobConfig>,
}
/// Job configuration
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct JobConfig {
    /// List of input assets stored in Cloud Storage.
    #[prost(message, repeated, tag = "1")]
    pub inputs: ::std::vec::Vec<Input>,
    /// List of `Edit atom`s. Defines the ultimate timeline of the resulting
    /// file or manifest.
    #[prost(message, repeated, tag = "2")]
    pub edit_list: ::std::vec::Vec<EditAtom>,
    /// List of elementary streams.
    #[prost(message, repeated, tag = "3")]
    pub elementary_streams: ::std::vec::Vec<ElementaryStream>,
    /// List of multiplexing settings for output streams.
    #[prost(message, repeated, tag = "4")]
    pub mux_streams: ::std::vec::Vec<MuxStream>,
    /// List of output manifests.
    #[prost(message, repeated, tag = "5")]
    pub manifests: ::std::vec::Vec<Manifest>,
    /// Output configuration.
    #[prost(message, optional, tag = "6")]
    pub output: ::std::option::Option<Output>,
    /// List of ad breaks. Specifies where to insert ad break tags in the output
    /// manifests.
    #[prost(message, repeated, tag = "7")]
    pub ad_breaks: ::std::vec::Vec<AdBreak>,
    /// Destination on Pub/Sub.
    #[prost(message, optional, tag = "8")]
    pub pubsub_destination: ::std::option::Option<PubsubDestination>,
    /// List of output sprite sheets.
    #[prost(message, repeated, tag = "9")]
    pub sprite_sheets: ::std::vec::Vec<SpriteSheet>,
    /// List of overlays on the output video, in descending Z-order.
    #[prost(message, repeated, tag = "10")]
    pub overlays: ::std::vec::Vec<Overlay>,
}
/// Input asset.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Input {
    /// A unique key for this input. Must be specified when using advanced
    /// mapping and edit lists.
    #[prost(string, tag = "1")]
    pub key: std::string::String,
    /// URI of the media. It must be stored in Cloud Storage. Example
    /// `gs://bucket/inputs/file.mp4`.
    /// If empty the value will be populated from `Job.input_uri`.
    #[prost(string, tag = "2")]
    pub uri: std::string::String,
    /// Preprocessing configurations.
    #[prost(message, optional, tag = "3")]
    pub preprocessing_config: ::std::option::Option<PreprocessingConfig>,
}
/// Location of output file(s) in a Cloud Storage bucket.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Output {
    /// URI for the output file(s). For example, `gs://my-bucket/outputs/`.
    /// If empty the value is populated from `Job.output_uri`.
    #[prost(string, tag = "1")]
    pub uri: std::string::String,
}
/// Edit atom.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EditAtom {
    /// A unique key for this atom. Must be specified when using advanced
    /// mapping.
    #[prost(string, tag = "1")]
    pub key: std::string::String,
    /// List of `Input.key`s identifying files that should be used in this atom.
    /// The listed `inputs` must have the same timeline.
    #[prost(string, repeated, tag = "2")]
    pub inputs: ::std::vec::Vec<std::string::String>,
    /// End time in seconds for the atom, relative to the input file timeline.
    /// When `end_time_offset` is not specified, the `inputs` are used until
    /// the end of the atom.
    #[prost(message, optional, tag = "3")]
    pub end_time_offset: ::std::option::Option<::prost_types::Duration>,
    /// Start time in seconds for the atom, relative to the input file timeline.
    /// The default is `0s`.
    #[prost(message, optional, tag = "4")]
    pub start_time_offset: ::std::option::Option<::prost_types::Duration>,
}
/// Ad break.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdBreak {
    /// Start time in seconds for the ad break, relative to the output file
    /// timeline. The default is `0s`.
    #[prost(message, optional, tag = "1")]
    pub start_time_offset: ::std::option::Option<::prost_types::Duration>,
}
/// Encoding of an input file such as an audio, video, or text track.
/// Elementary streams must be packaged before
/// mapping and sharing between different output formats.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ElementaryStream {
    /// A unique key for this elementary stream.
    #[prost(string, tag = "4")]
    pub key: std::string::String,
    /// Encoding of an audio, video, or text track.
    #[prost(oneof = "elementary_stream::ElementaryStream", tags = "1, 2, 3")]
    pub elementary_stream: ::std::option::Option<elementary_stream::ElementaryStream>,
}
pub mod elementary_stream {
    /// Encoding of an audio, video, or text track.
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
    /// A unique key for this multiplexed stream. HLS media manifests will be
    /// named `MuxStream.key` with the `".m3u8"` extension suffix.
    #[prost(string, tag = "1")]
    pub key: std::string::String,
    /// The name of the generated file. The default is `MuxStream.key` with the
    /// extension suffix corresponding to the `MuxStream.container`.
    ///
    /// Individual segments also have an incremental 10-digit zero-padded suffix
    /// starting from 0 before the extension, such as `"mux_stream0000000123.ts"`.
    #[prost(string, tag = "2")]
    pub file_name: std::string::String,
    /// The container format. The default is `"mp4"`
    ///
    /// Supported container formats:
    /// - 'ts'
    /// - 'fmp4'- the corresponding file extension is `".m4s"`
    /// - 'mp4'
    /// - 'vtt'
    #[prost(string, tag = "3")]
    pub container: std::string::String,
    /// List of `ElementaryStream.key`s multiplexed in this stream.
    #[prost(string, repeated, tag = "4")]
    pub elementary_streams: ::std::vec::Vec<std::string::String>,
    /// Segment settings for `"ts"`, `"fmp4"` and `"vtt"`.
    #[prost(message, optional, tag = "5")]
    pub segment_settings: ::std::option::Option<SegmentSettings>,
    /// Encryption settings.
    #[prost(message, optional, tag = "6")]
    pub encryption: ::std::option::Option<Encryption>,
}
/// Manifest configuration.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Manifest {
    /// The name of the generated file. The default is `"master"` with the
    /// extension suffix corresponding to the `Manifest.type`.
    #[prost(string, tag = "1")]
    pub file_name: std::string::String,
    /// Required. Type of the manifest, can be "HLS" or "DASH".
    #[prost(enumeration = "manifest::ManifestType", tag = "2")]
    pub r#type: i32,
    /// Required. List of user given `MuxStream.key`s that should appear in this manifest.
    ///
    /// When `Manifest.type` is `HLS`, a media manifest with name `MuxStream.key`
    /// and `.m3u8` extension is generated for each element of the
    /// `Manifest.mux_streams`.
    #[prost(string, repeated, tag = "3")]
    pub mux_streams: ::std::vec::Vec<std::string::String>,
}
pub mod manifest {
    /// The manifest type can be either `"HLS"` or `"DASH"`.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ManifestType {
        /// The manifest type is not specified.
        Unspecified = 0,
        /// Create `"HLS"` manifest. The corresponding file extension is `".m3u8"`.
        Hls = 1,
        /// Create `"DASH"` manifest. The corresponding file extension is `".mpd"`.
        Dash = 2,
    }
}
/// A Pub/Sub destination.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PubsubDestination {
    /// The name of the Pub/Sub topic to publish job completion notification
    /// to. For example: `projects/{project}/topics/{topic}`.
    #[prost(string, tag = "1")]
    pub topic: std::string::String,
}
/// Sprite sheet configuration.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpriteSheet {
    /// Format type. The default is `"jpeg"`.
    ///
    /// Supported formats:
    /// - 'jpeg'
    #[prost(string, tag = "1")]
    pub format: std::string::String,
    /// Required. File name prefix for the generated sprite sheets.
    ///
    /// Each sprite sheet has an incremental 10-digit zero-padded suffix starting
    /// from 0 before the extension, such as `"sprite_sheet0000000123.jpeg"`.
    #[prost(string, tag = "2")]
    pub file_prefix: std::string::String,
    /// Required. The width of sprite in pixels. Must be an even integer.
    #[prost(int32, tag = "3")]
    pub sprite_width_pixels: i32,
    /// Required. The height of sprite in pixels. Must be an even integer.
    #[prost(int32, tag = "4")]
    pub sprite_height_pixels: i32,
    /// The maximum number of sprites per row in a sprite sheet. The default is 0,
    /// which indicates no maximum limit.
    #[prost(int32, tag = "5")]
    pub column_count: i32,
    /// The maximum number of rows per sprite sheet. When the sprite sheet is full,
    /// a new sprite sheet is created. The default is 0, which indicates no maximum
    /// limit.
    #[prost(int32, tag = "6")]
    pub row_count: i32,
    /// Start time in seconds, relative to the output file timeline. Determines the
    /// first sprite to pick. The default is `0s`.
    #[prost(message, optional, tag = "7")]
    pub start_time_offset: ::std::option::Option<::prost_types::Duration>,
    /// End time in seconds, relative to the output file timeline. When
    /// `end_time_offset` is not specified, the sprites are generated until the end
    /// of the output file.
    #[prost(message, optional, tag = "8")]
    pub end_time_offset: ::std::option::Option<::prost_types::Duration>,
    /// Specify either total number of sprites or interval to create sprites.
    #[prost(oneof = "sprite_sheet::ExtractionStrategy", tags = "9, 10")]
    pub extraction_strategy: ::std::option::Option<sprite_sheet::ExtractionStrategy>,
}
pub mod sprite_sheet {
    /// Specify either total number of sprites or interval to create sprites.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ExtractionStrategy {
        /// Total number of sprites. Create the specified number of sprites
        /// distributed evenly across the timeline of the output media. The default
        /// is 100.
        #[prost(int32, tag = "9")]
        TotalCount(i32),
        /// Starting from `0s`, create sprites at regular intervals. Specify the
        /// interval value in seconds.
        #[prost(message, tag = "10")]
        Interval(::prost_types::Duration),
    }
}
/// Overlay configuration.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Overlay {
    /// Image overlay.
    #[prost(message, optional, tag = "1")]
    pub image: ::std::option::Option<overlay::Image>,
    /// List of Animations. The list should be chronological, without any time
    /// overlap.
    #[prost(message, repeated, tag = "2")]
    pub animations: ::std::vec::Vec<overlay::Animation>,
}
pub mod overlay {
    /// 2D normalized coordinates. Default: `{0.0, 0.0}`
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct NormalizedCoordinate {
        /// Normalized x coordinate.
        #[prost(double, tag = "1")]
        pub x: f64,
        /// Normalized y coordinate.
        #[prost(double, tag = "2")]
        pub y: f64,
    }
    /// Overlaid jpeg image.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Image {
        /// Required. URI of the image in Cloud Storage. For example,
        /// `gs://bucket/inputs/image.jpeg`.
        #[prost(string, tag = "1")]
        pub uri: std::string::String,
        /// Normalized image resolution, based on output video resolution. Valid
        /// values: `0.0`–`1.0`. To respect the original image aspect ratio, set
        /// either `x` or `y` to `0.0`. To use the original image resolution, set
        /// both `x` and `y` to `0.0`.
        #[prost(message, optional, tag = "2")]
        pub resolution: ::std::option::Option<NormalizedCoordinate>,
        /// Target image opacity. Valid values: `1` (solid, default),
        /// `0` (transparent).
        #[prost(double, tag = "3")]
        pub alpha: f64,
    }
    /// Display static overlay object.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct AnimationStatic {
        /// Normalized coordinates based on output video resolution. Valid
        /// values: `0.0`–`1.0`. `xy` is the upper-left coordinate of the overlay
        /// object.
        #[prost(message, optional, tag = "1")]
        pub xy: ::std::option::Option<NormalizedCoordinate>,
        /// The time to start displaying the overlay object, in seconds. Default: 0
        #[prost(message, optional, tag = "2")]
        pub start_time_offset: ::std::option::Option<::prost_types::Duration>,
    }
    /// Display overlay object with fade animation.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct AnimationFade {
        /// Required. Type of fade animation: `FADE_IN` or `FADE_OUT`.
        #[prost(enumeration = "FadeType", tag = "1")]
        pub fade_type: i32,
        /// Normalized coordinates based on output video resolution. Valid
        /// values: `0.0`–`1.0`. `xy` is the upper-left coordinate of the overlay
        /// object.
        #[prost(message, optional, tag = "2")]
        pub xy: ::std::option::Option<NormalizedCoordinate>,
        /// The time to start the fade animation, in seconds. Default: 0
        #[prost(message, optional, tag = "3")]
        pub start_time_offset: ::std::option::Option<::prost_types::Duration>,
        /// The time to end the fade animation, in seconds. Default:
        /// `start_time_offset` + 1s
        #[prost(message, optional, tag = "4")]
        pub end_time_offset: ::std::option::Option<::prost_types::Duration>,
    }
    /// End previous overlay animation from the video. Without AnimationEnd, the
    /// overlay object will keep the state of previous animation until the end of
    /// the video.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct AnimationEnd {
        /// The time to end overlay object, in seconds. Default: 0
        #[prost(message, optional, tag = "1")]
        pub start_time_offset: ::std::option::Option<::prost_types::Duration>,
    }
    /// Animation types.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Animation {
        /// Animations can be static or fade, or they can end the previous animation.
        #[prost(oneof = "animation::AnimationType", tags = "1, 2, 3")]
        pub animation_type: ::std::option::Option<animation::AnimationType>,
    }
    pub mod animation {
        /// Animations can be static or fade, or they can end the previous animation.
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum AnimationType {
            /// Display static overlay object.
            #[prost(message, tag = "1")]
            AnimationStatic(super::AnimationStatic),
            /// Display overlay object with fade animation.
            #[prost(message, tag = "2")]
            AnimationFade(super::AnimationFade),
            /// End previous animation.
            #[prost(message, tag = "3")]
            AnimationEnd(super::AnimationEnd),
        }
    }
    /// Fade type for the overlay: `FADE_IN` or `FADE_OUT`.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum FadeType {
        /// The fade type is not specified.
        Unspecified = 0,
        /// Fade the overlay object into view.
        FadeIn = 1,
        /// Fade the overlay object out of view.
        FadeOut = 2,
    }
}
/// Preprocessing configurations.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PreprocessingConfig {
    /// Color preprocessing configuration.
    #[prost(message, optional, tag = "1")]
    pub color: ::std::option::Option<preprocessing_config::Color>,
    /// Denoise preprocessing configuration.
    #[prost(message, optional, tag = "2")]
    pub denoise: ::std::option::Option<preprocessing_config::Denoise>,
    /// Deblock preprocessing configuration.
    #[prost(message, optional, tag = "3")]
    pub deblock: ::std::option::Option<preprocessing_config::Deblock>,
    /// Audio preprocessing configuration.
    #[prost(message, optional, tag = "4")]
    pub audio: ::std::option::Option<preprocessing_config::Audio>,
}
pub mod preprocessing_config {
    /// Color preprocessing configuration.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Color {
        /// Control color saturation of the video. Enter a value between -1 and 1,
        /// where -1 is fully desaturated and 1 is maximum saturation. 0 is no
        /// change. The default is 0.
        #[prost(double, tag = "1")]
        pub saturation: f64,
        /// Control black and white contrast of the video. Enter a value between -1
        /// and 1, where -1 is minimum contrast and 1 is maximum contrast. 0 is no
        /// change. The default is 0.
        #[prost(double, tag = "2")]
        pub contrast: f64,
        /// Control brightness of the video. Enter a value between -1 and 1, where -1
        /// is minimum brightness and 1 is maximum brightness. 0 is no change. The
        /// default is 0.
        #[prost(double, tag = "3")]
        pub brightness: f64,
    }
    /// Denoise preprocessing configuration.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Denoise {
        /// Set strength of the denoise. Enter a value between 0 and 1. The higher
        /// the value, the smoother the image. 0 is no denoising. The default is 0.
        #[prost(double, tag = "1")]
        pub strength: f64,
        /// Set the denoiser mode. The default is `"standard"`.
        ///
        /// Supported denoiser modes:
        /// - 'standard'
        /// - 'grain'
        #[prost(string, tag = "2")]
        pub tune: std::string::String,
    }
    /// Deblock preprocessing configuration.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Deblock {
        /// Set strength of the deblocker. Enter a value between 0 and 1. The higher
        /// the value, the stronger the block removal. 0 is no deblocking. The
        /// default is 0.
        #[prost(double, tag = "1")]
        pub strength: f64,
        /// Enable deblocker. The default is `false`.
        #[prost(bool, tag = "2")]
        pub enabled: bool,
    }
    /// Audio preprocessing configuration.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Audio {
        /// Specify audio loudness normalization in loudness units relative to full
        /// scale (LUFS). Enter a value between -24 and 0, where -24 is the Advanced
        /// Television Systems Committee (ATSC A/85), -23 is the EU R128 broadcast
        /// standard, -19 is the prior standard for online mono audio, -18 is the
        /// ReplayGain standard, -16 is the prior standard for stereo audio, -14 is
        /// the new online audio standard recommended by Spotify, as well as Amazon
        /// Echo, and 0 disables normalization. The default is 0.
        #[prost(double, tag = "1")]
        pub lufs: f64,
        /// Enable boosting high frequency components. The default is `false`.
        #[prost(bool, tag = "2")]
        pub high_boost: bool,
        /// Enable boosting low frequency components. The default is `false`.
        #[prost(bool, tag = "3")]
        pub low_boost: bool,
    }
}
/// Video stream resource.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VideoStream {
    /// Codec type. The default is `"h264"`.
    ///
    /// Supported codecs:
    /// - 'h264'
    /// - 'h265'
    /// - 'vp9'
    #[prost(string, tag = "1")]
    pub codec: std::string::String,
    /// Enforce specified codec profile. The default is `"high"`.
    ///
    /// Supported codec profiles:
    /// - 'baseline'
    /// - 'main'
    /// - 'high'
    #[prost(string, tag = "2")]
    pub profile: std::string::String,
    /// Enforce specified codec tune.
    #[prost(string, tag = "3")]
    pub tune: std::string::String,
    /// Enforce specified codec preset. The default is `"veryfast"`.
    #[prost(string, tag = "4")]
    pub preset: std::string::String,
    /// Required. The height of video in pixels. Must be an even integer.
    #[prost(int32, tag = "5")]
    pub height_pixels: i32,
    /// Required. The width of video in pixels. Must be an even integer.
    #[prost(int32, tag = "6")]
    pub width_pixels: i32,
    /// Pixel format to use. The default is `"yuv420p"`.
    ///
    /// Supported pixel formats:
    /// - 'yuv420p' pixel format.
    /// - 'yuv422p' pixel format.
    /// - 'yuv444p' pixel format.
    /// - 'yuv420p10' 10-bit HDR pixel format.
    /// - 'yuv422p10' 10-bit HDR pixel format.
    /// - 'yuv444p10' 10-bit HDR pixel format.
    /// - 'yuv420p12' 12-bit HDR pixel format.
    /// - 'yuv422p12' 12-bit HDR pixel format.
    /// - 'yuv444p12' 12-bit HDR pixel format.
    #[prost(string, tag = "7")]
    pub pixel_format: std::string::String,
    /// Required. The video bitrate in bits per second. Must be between 1 and 1,000,000,000.
    #[prost(int32, tag = "8")]
    pub bitrate_bps: i32,
    /// Specify the `rate_control_mode`. The default is `"vbr"`.
    ///
    /// Supported rate control modes:
    /// - 'vbr' - variable bitrate
    /// - 'crf' - constant rate factor
    #[prost(string, tag = "9")]
    pub rate_control_mode: std::string::String,
    /// Use two-pass encoding strategy to achieve better video quality.
    /// `VideoStream.rate_control_mode` must be `"vbr"`. The default is `false`.
    #[prost(bool, tag = "10")]
    pub enable_two_pass: bool,
    /// Target CRF level. Must be between 10 and 36, where 10 is the highest
    /// quality and 36 is the most efficient compression. The default is 21.
    #[prost(int32, tag = "11")]
    pub crf_level: i32,
    /// Size of the Video Buffering Verifier (VBV) buffer in bits. Must be greater
    /// than zero. The default is equal to `VideoStream.bitrate_bps`.
    #[prost(int32, tag = "12")]
    pub vbv_size_bits: i32,
    /// Initial fullness of the Video Buffering Verifier (VBV) buffer in bits. Must
    /// be greater than zero. The default is equal to 90% of
    /// `VideoStream.vbv_size_bits`.
    #[prost(int32, tag = "13")]
    pub vbv_fullness_bits: i32,
    /// Specifies whether an open Group of Pictures (GOP) structure should be
    /// allowed or not. The default is `false`.
    #[prost(bool, tag = "14")]
    pub allow_open_gop: bool,
    /// The entropy coder to use. The default is `"cabac"`.
    ///
    /// Supported entropy coders:
    /// - 'cavlc'
    /// - 'cabac'
    #[prost(string, tag = "17")]
    pub entropy_coder: std::string::String,
    /// Allow B-pyramid for reference frame selection. This may not be supported
    /// on all decoders. The default is `false`.
    #[prost(bool, tag = "18")]
    pub b_pyramid: bool,
    /// The number of consecutive B-frames. Must be greater than or equal to zero.
    /// Must be less than `VideoStream.gop_frame_count` if set. The default is 0.
    #[prost(int32, tag = "19")]
    pub b_frame_count: i32,
    /// Required. The video frame rate in frames per second. Must be less than or equal to
    /// 120. Will default to the input frame rate if larger than the input frame
    /// rate.
    #[prost(double, tag = "20")]
    pub frame_rate: f64,
    /// Specify the intensity of the adaptive quantizer (AQ). Must be between 0 and
    /// 1, where 0 disables the quantizer and 1 maximizes the quantizer. A
    /// higher value equals a lower bitrate but smoother image. The default is 0.
    #[prost(double, tag = "21")]
    pub aq_strength: f64,
    /// GOP mode can be either by frame count or duration.
    #[prost(oneof = "video_stream::GopMode", tags = "15, 16")]
    pub gop_mode: ::std::option::Option<video_stream::GopMode>,
}
pub mod video_stream {
    /// GOP mode can be either by frame count or duration.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum GopMode {
        /// Select the GOP size based on the specified frame count. Must be greater
        /// than zero.
        #[prost(int32, tag = "15")]
        GopFrameCount(i32),
        /// Select the GOP size based on the specified duration. The default is
        /// `"3s"`.
        #[prost(message, tag = "16")]
        GopDuration(::prost_types::Duration),
    }
}
/// Audio stream resource.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AudioStream {
    /// The codec for this audio stream. The default is `"aac"`.
    ///
    /// Supported audio codecs:
    /// - 'aac'
    /// - 'aac-he'
    /// - 'aac-he-v2'
    /// - 'mp3'
    /// - 'ac3'
    /// - 'eac3'
    #[prost(string, tag = "1")]
    pub codec: std::string::String,
    /// Required. Audio bitrate in bits per second. Must be between 1 and 10,000,000.
    #[prost(int32, tag = "2")]
    pub bitrate_bps: i32,
    /// Number of audio channels. Must be between 1 and 6. The default is 2.
    #[prost(int32, tag = "3")]
    pub channel_count: i32,
    /// A list of channel names specifying layout of the audio channels.
    /// This only affects the metadata embedded in the container headers, if
    /// supported by the specified format. The default is `["fl", "fr"]`.
    ///
    /// Supported channel names:
    /// - 'fl' - Front left channel
    /// - 'fr' - Front right channel
    /// - 'sl' - Side left channel
    /// - 'sr' - Side right channel
    /// - 'fc' - Front center channel
    /// - 'lfe' - Low frequency
    #[prost(string, repeated, tag = "4")]
    pub channel_layout: ::std::vec::Vec<std::string::String>,
    /// The mapping for the `Job.edit_list` atoms with audio `EditAtom.inputs`.
    #[prost(message, repeated, tag = "5")]
    pub mapping: ::std::vec::Vec<audio_stream::AudioAtom>,
    /// The audio sample rate in Hertz. The default is 48000 Hertz.
    #[prost(int32, tag = "6")]
    pub sample_rate_hertz: i32,
}
pub mod audio_stream {
    /// The mapping for the `Job.edit_list` atoms with audio `EditAtom.inputs`.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct AudioAtom {
        /// Required. The `EditAtom.key` that references the atom with audio inputs in the
        /// `Job.edit_list`.
        #[prost(string, tag = "1")]
        pub key: std::string::String,
        /// List of `Channel`s for this audio stream.
        /// for in-depth explanation.
        #[prost(message, repeated, tag = "2")]
        pub channels: ::std::vec::Vec<audio_atom::AudioChannel>,
    }
    pub mod audio_atom {
        /// The audio channel.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct AudioChannel {
            /// List of `Job.inputs` for this audio channel.
            #[prost(message, repeated, tag = "2")]
            pub inputs: ::std::vec::Vec<audio_channel::AudioChannelInput>,
        }
        pub mod audio_channel {
            /// Identifies which input file, track, and channel should be used.
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct AudioChannelInput {
                /// Required. The `Input.key` that identifies the input file.
                #[prost(string, tag = "1")]
                pub key: std::string::String,
                /// Required. The zero-based index of the track in the input file.
                #[prost(int32, tag = "2")]
                pub track: i32,
                /// Required. The zero-based index of the channel in the input file.
                #[prost(int32, tag = "3")]
                pub channel: i32,
                /// Audio volume control in dB. Negative values decrease volume,
                /// positive values increase. The default is 0.
                #[prost(double, tag = "4")]
                pub gain_db: f64,
            }
        }
    }
}
/// Encoding of a text stream. For example, closed captions or subtitles.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TextStream {
    /// The codec for this text stream. The default is `"webvtt"`.
    ///
    /// Supported text codecs:
    /// - 'srt'
    /// - 'ttml'
    /// - 'cea608'
    /// - 'cea708'
    /// - 'webvtt'
    #[prost(string, tag = "1")]
    pub codec: std::string::String,
    /// Required. The BCP-47 language code, such as `"en-US"` or `"sr-Latn"`. For more
    /// information, see
    /// https://www.unicode.org/reports/tr35/#Unicode_locale_identifier.
    #[prost(string, tag = "2")]
    pub language_code: std::string::String,
    /// The mapping for the `Job.edit_list` atoms with text `EditAtom.inputs`.
    #[prost(message, repeated, tag = "3")]
    pub mapping: ::std::vec::Vec<text_stream::TextAtom>,
}
pub mod text_stream {
    /// The mapping for the `Job.edit_list` atoms with text `EditAtom.inputs`.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TextAtom {
        /// Required. The `EditAtom.key` that references atom with text inputs in the
        /// `Job.edit_list`.
        #[prost(string, tag = "1")]
        pub key: std::string::String,
        /// List of `Job.inputs` that should be embedded in this atom. Only one
        /// input is supported.
        #[prost(message, repeated, tag = "2")]
        pub inputs: ::std::vec::Vec<text_atom::TextInput>,
    }
    pub mod text_atom {
        /// Identifies which input file and track should be used.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct TextInput {
            /// Required. The `Input.key` that identifies the input file.
            #[prost(string, tag = "1")]
            pub key: std::string::String,
            /// Required. The zero-based index of the track in the input file.
            #[prost(int32, tag = "2")]
            pub track: i32,
        }
    }
}
/// Segment settings for `"ts"`, `"fmp4"` and `"vtt"`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SegmentSettings {
    /// Duration of the segments in seconds. The default is `"6.0s"`.
    #[prost(message, optional, tag = "1")]
    pub segment_duration: ::std::option::Option<::prost_types::Duration>,
    /// Required. Create an individual segment file. The default is `false`.
    #[prost(bool, tag = "3")]
    pub individual_segments: bool,
}
/// Encryption settings.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Encryption {
    /// Required. 128 bit encryption key represented as lowercase hexadecimal digits.
    #[prost(string, tag = "1")]
    pub key: std::string::String,
    /// Required. 128 bit Initialization Vector (IV) represented as lowercase hexadecimal
    /// digits.
    #[prost(string, tag = "2")]
    pub iv: std::string::String,
    /// Encryption mode can be either `aes` or `cenc`.
    #[prost(oneof = "encryption::EncryptionMode", tags = "3, 4, 5")]
    pub encryption_mode: ::std::option::Option<encryption::EncryptionMode>,
}
pub mod encryption {
    /// Configuration for AES-128 encryption.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Aes128Encryption {
        /// Required. URI of the key delivery service. This URI is inserted into the M3U8
        /// header.
        #[prost(string, tag = "1")]
        pub key_uri: std::string::String,
    }
    /// Configuration for SAMPLE-AES encryption.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SampleAesEncryption {
        /// Required. URI of the key delivery service. This URI is inserted into the M3U8
        /// header.
        #[prost(string, tag = "1")]
        pub key_uri: std::string::String,
    }
    /// Configuration for MPEG Common Encryption (MPEG-CENC).
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct MpegCommonEncryption {
        /// Required. 128 bit Key ID represented as lowercase hexadecimal digits for use with
        /// common encryption.
        #[prost(string, tag = "1")]
        pub key_id: std::string::String,
        /// Required. Specify the encryption scheme.
        ///
        /// Supported encryption schemes:
        /// - 'cenc'
        /// - 'cbcs'
        #[prost(string, tag = "2")]
        pub scheme: std::string::String,
    }
    /// Encryption mode can be either `aes` or `cenc`.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum EncryptionMode {
        /// Configuration for AES-128 encryption.
        #[prost(message, tag = "3")]
        Aes128(Aes128Encryption),
        /// Configuration for SAMPLE-AES encryption.
        #[prost(message, tag = "4")]
        SampleAes(SampleAesEncryption),
        /// Configuration for MPEG Common Encryption (MPEG-CENC).
        #[prost(message, tag = "5")]
        MpegCenc(MpegCommonEncryption),
    }
}
/// Estimated fractional progress for each step, from `0` to `1`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Progress {
    /// Estimated fractional progress for `analyzing` step.
    #[prost(double, tag = "1")]
    pub analyzed: f64,
    /// Estimated fractional progress for `encoding` step.
    #[prost(double, tag = "2")]
    pub encoded: f64,
    /// Estimated fractional progress for `uploading` step.
    #[prost(double, tag = "3")]
    pub uploaded: f64,
    /// Estimated fractional progress for `notifying` step.
    #[prost(double, tag = "4")]
    pub notified: f64,
}
/// Additional information about the reasons for the failure.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FailureDetail {
    /// A description of the failure.
    #[prost(string, tag = "1")]
    pub description: std::string::String,
}
/// Request message for `TranscoderService.CreateJob`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateJobRequest {
    /// Required. The parent location to create and process this job.
    /// Format: `projects/{project}/locations/{location}`
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Required. Parameters for creating transcoding job.
    #[prost(message, optional, tag = "2")]
    pub job: ::std::option::Option<Job>,
}
/// Request message for `TranscoderService.ListJobs`.
/// The parent location from which to retrieve the collection of jobs.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListJobsRequest {
    /// Required. Format: `projects/{project}/locations/{location}`
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// The maximum number of items to return.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The `next_page_token` value returned from a previous List request, if
    /// any.
    #[prost(string, tag = "3")]
    pub page_token: std::string::String,
}
/// Request message for `TranscoderService.GetJob`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetJobRequest {
    /// Required. The name of the job to retrieve.
    /// Format: `projects/{project}/locations/{location}/jobs/{job}`
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Request message for `TranscoderService.DeleteJob`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteJobRequest {
    /// Required. The name of the job to delete.
    /// Format: `projects/{project}/locations/{location}/jobs/{job}`
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Response message for `TranscoderService.ListJobs`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListJobsResponse {
    /// List of jobs in the specified region.
    #[prost(message, repeated, tag = "1")]
    pub jobs: ::std::vec::Vec<Job>,
    /// The pagination token.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
}
/// Request message for `TranscoderService.CreateJobTemplate`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateJobTemplateRequest {
    /// Required. The parent location to create this job template.
    /// Format: `projects/{project}/locations/{location}`
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Required. Parameters for creating job template.
    #[prost(message, optional, tag = "2")]
    pub job_template: ::std::option::Option<JobTemplate>,
    /// Required. The ID to use for the job template, which will become the final component
    /// of the job template's resource name.
    ///
    /// This value should be 4-63 characters, and valid characters
    /// are `/[a-zA-Z0-9_-_]/`.
    #[prost(string, tag = "3")]
    pub job_template_id: std::string::String,
}
/// Request message for `TranscoderService.ListJobTemplates`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListJobTemplatesRequest {
    /// Required. The parent location from which to retrieve the collection of job templates.
    /// Format: `projects/{project}/locations/{location}`
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// The maximum number of items to return.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The `next_page_token` value returned from a previous List request, if
    /// any.
    #[prost(string, tag = "3")]
    pub page_token: std::string::String,
}
/// Request message for `TranscoderService.GetJobTemplate`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetJobTemplateRequest {
    /// Required. The name of the job template to retrieve.
    /// Format:
    /// `projects/{project}/locations/{location}/jobTemplates/{job_template}`
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Request message for `TranscoderService.DeleteJobTemplate`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteJobTemplateRequest {
    /// Required. The name of the job template to delete.
    /// `projects/{project}/locations/{location}/jobTemplates/{job_template}`
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Response message for `TranscoderService.ListJobTemplates`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListJobTemplatesResponse {
    /// List of job templates in the specified region.
    #[prost(message, repeated, tag = "1")]
    pub job_templates: ::std::vec::Vec<JobTemplate>,
    /// The pagination token.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
}
#[doc = r" Generated client implementations."]
pub mod transcoder_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Using the Transcoder API, you can queue asynchronous jobs for transcoding"]
    #[doc = " media into various output formats. Output formats may include different"]
    #[doc = " streaming standards such as HTTP Live Streaming (HLS) and Dynamic Adaptive"]
    #[doc = " Streaming over HTTP (DASH). You can also customize jobs using advanced"]
    #[doc = " features such as Digital Rights Management (DRM), audio equalization, content"]
    #[doc = " concatenation, and digital ad-stitch ready content generation."]
    pub struct TranscoderServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> TranscoderServiceClient<T>
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
        #[doc = " Creates a job in the specified region."]
        pub async fn create_job(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateJobRequest>,
        ) -> Result<tonic::Response<super::Job>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.video.transcoder.v1beta1.TranscoderService/CreateJob",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists jobs in the specified region."]
        pub async fn list_jobs(
            &mut self,
            request: impl tonic::IntoRequest<super::ListJobsRequest>,
        ) -> Result<tonic::Response<super::ListJobsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.video.transcoder.v1beta1.TranscoderService/ListJobs",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns the job data."]
        pub async fn get_job(
            &mut self,
            request: impl tonic::IntoRequest<super::GetJobRequest>,
        ) -> Result<tonic::Response<super::Job>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.video.transcoder.v1beta1.TranscoderService/GetJob",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes a job."]
        pub async fn delete_job(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteJobRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.video.transcoder.v1beta1.TranscoderService/DeleteJob",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a job template in the specified region."]
        pub async fn create_job_template(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateJobTemplateRequest>,
        ) -> Result<tonic::Response<super::JobTemplate>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.video.transcoder.v1beta1.TranscoderService/CreateJobTemplate",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists job templates in the specified region."]
        pub async fn list_job_templates(
            &mut self,
            request: impl tonic::IntoRequest<super::ListJobTemplatesRequest>,
        ) -> Result<tonic::Response<super::ListJobTemplatesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.video.transcoder.v1beta1.TranscoderService/ListJobTemplates",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns the job template data."]
        pub async fn get_job_template(
            &mut self,
            request: impl tonic::IntoRequest<super::GetJobTemplateRequest>,
        ) -> Result<tonic::Response<super::JobTemplate>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.video.transcoder.v1beta1.TranscoderService/GetJobTemplate",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes a job template."]
        pub async fn delete_job_template(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteJobTemplateRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.video.transcoder.v1beta1.TranscoderService/DeleteJobTemplate",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for TranscoderServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for TranscoderServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "TranscoderServiceClient {{ ... }}")
        }
    }
}
