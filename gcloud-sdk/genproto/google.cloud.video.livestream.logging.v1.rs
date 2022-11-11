/// Logs of activities related to the Channels.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChannelActivity {
    /// Message is for more details of the log and instructions to users.
    #[prost(string, tag = "1")]
    pub message: ::prost::alloc::string::String,
    /// Different types of the logs.
    #[prost(oneof = "channel_activity::ActivityType", tags = "2, 3, 4, 5, 6")]
    pub activity_type: ::core::option::Option<channel_activity::ActivityType>,
}
/// Nested message and enum types in `ChannelActivity`.
pub mod channel_activity {
    /// Different types of the logs.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ActivityType {
        /// The channel streaming state changes.
        #[prost(message, tag = "2")]
        StreamingStateChange(super::StreamingStateChange),
        /// An error happens with the video pipeline.
        #[prost(message, tag = "3")]
        StreamingError(super::StreamingError),
        /// The channel has accepted an input stream.
        #[prost(message, tag = "4")]
        InputAccept(super::InputAccept),
        /// An error happens with the input stream.
        #[prost(message, tag = "5")]
        InputError(super::InputError),
        /// An input stream disconnects.
        #[prost(message, tag = "6")]
        InputDisconnect(super::InputDisconnect),
    }
}
/// StreamingStateChange records when the channel streaming state changes.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamingStateChange {
    /// New streaming state of the channel.
    #[prost(enumeration = "super::super::v1::channel::StreamingState", tag = "1")]
    pub new_state: i32,
    /// Previous streaming state of the channel.
    #[prost(enumeration = "super::super::v1::channel::StreamingState", tag = "2")]
    pub previous_state: i32,
}
/// StreamingError records when an error happens with the video pipeline.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamingError {
    /// A description of the reason for the streaming error.
    #[prost(message, optional, tag = "1")]
    pub error: ::core::option::Option<super::super::super::super::super::rpc::Status>,
}
/// InputAccept records when the channel has accepted an input stream.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InputAccept {
    /// ID of the input stream.
    #[prost(string, tag = "1")]
    pub stream_id: ::prost::alloc::string::String,
    /// The user-defined key for the input attachment.
    #[prost(string, tag = "2")]
    pub input_attachment: ::prost::alloc::string::String,
    /// Properties of the input stream.
    #[prost(message, optional, tag = "3")]
    pub input_stream_property: ::core::option::Option<InputStreamProperty>,
}
/// InputError records when an error happens with the input stream.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InputError {
    /// ID of the input stream.
    #[prost(string, tag = "1")]
    pub stream_id: ::prost::alloc::string::String,
    /// The user-defined key for the input attachment. If the stream doesn’t belong
    /// to any input attachment, this field is empty.
    #[prost(string, tag = "2")]
    pub input_attachment: ::prost::alloc::string::String,
    /// Properties of the input stream.
    #[prost(message, optional, tag = "3")]
    pub input_stream_property: ::core::option::Option<InputStreamProperty>,
    /// A description of the reason for the error with the input stream.
    #[prost(message, optional, tag = "4")]
    pub error: ::core::option::Option<super::super::super::super::super::rpc::Status>,
}
/// Properties of the input stream.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InputStreamProperty {
    /// Properties of the video streams.
    #[prost(message, repeated, tag = "1")]
    pub video_streams: ::prost::alloc::vec::Vec<VideoStream>,
    /// Properties of the audio streams.
    #[prost(message, repeated, tag = "2")]
    pub audio_streams: ::prost::alloc::vec::Vec<AudioStream>,
}
/// Properties of the video stream.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VideoStream {
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
pub struct AudioStream {
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
/// InputDisconnect records when an input stream disconnects.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InputDisconnect {
    /// ID of the input stream.
    #[prost(string, tag = "1")]
    pub stream_id: ::prost::alloc::string::String,
    /// The user-defined key for the input attachment.
    #[prost(string, tag = "2")]
    pub input_attachment: ::prost::alloc::string::String,
}
