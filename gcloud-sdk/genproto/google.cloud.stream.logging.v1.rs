/// OperationEventLog contains the time series data of the operations on the
/// stream resources. Internally, these logs represent events happening in
/// the control plane as result of API invocations against the stream resource
/// entities.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationEventLog {
    /// Enum for type of event being logged.
    #[prost(enumeration="OperationEventType", tag="1")]
    pub event_type: i32,
    /// Timestamp when the event occurred.
    #[prost(message, optional, tag="2")]
    pub event_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Operation resource identifier.
    #[prost(string, tag="3")]
    pub operation: ::prost::alloc::string::String,
    /// Artifact created as a result of the operation.
    #[prost(message, repeated, tag="4")]
    pub operation_artifacts: ::prost::alloc::vec::Vec<OperationArtifact>,
}
/// OperationArtifact contains the information about the artifact created as
/// result of the operation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationArtifact {
    /// Type of the artifact.
    #[prost(string, tag="1")]
    pub artifact_type: ::prost::alloc::string::String,
    /// Location of the artifact.
    #[prost(string, tag="2")]
    pub artifact_uri: ::prost::alloc::string::String,
}
/// SessionEventLog contains the time series data regarding the streaming session
/// to serve the end consumer. Internally, these logs represent events in the
/// data plane streamer instance as a result of end consumer interacting with the
/// stream resources.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SessionEventLog {
    /// Enum for type of event being logged.
    #[prost(enumeration="SessionEventType", tag="1")]
    pub event_type: i32,
    /// Timestamp when the event occurred.
    #[prost(message, optional, tag="2")]
    pub event_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Session identifier generated from the server.
    #[prost(string, tag="3")]
    pub session_id: ::prost::alloc::string::String,
}
/// OperationEventType is the enum value for the state of operation.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OperationEventType {
    /// Unspecified operation event type.
    Unspecified = 0,
    /// Operation for create stream content started.
    OperationEventCreateContentStarted = 1,
    /// Operation for create stream content ended.
    OperationEventCreateContentEnded = 2,
    /// Operation for build stream content started.
    OperationEventBuildContentStarted = 3,
    /// Operation for build stream content ended.
    OperationEventBuildContentEnded = 4,
    /// Operation for update stream content started.
    OperationEventUpdateContentStarted = 5,
    /// Operation for update stream content ended.
    OperationEventUpdateContentEnded = 6,
    /// Operation for delete stream content started.
    OperationEventDeleteContentStarted = 7,
    /// Operation for delete stream content ended.
    OperationEventDeleteContentEnded = 8,
    /// Operation for create stream instance started.
    OperationEventCreateInstanceStarted = 9,
    /// Operation for create stream instance ended.
    OperationEventCreateInstanceEnded = 10,
    /// Operation for update stream instance started.
    OperationEventUpdateInstanceStarted = 11,
    /// Operation for update stream instance ended.
    OperationEventUpdateInstanceEnded = 12,
    /// Operation for delete stream instance started.
    OperationEventDeleteInstanceStarted = 13,
    /// Operation for delete stream instance ended.
    OperationEventDeleteInstanceEnded = 14,
}
impl OperationEventType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            OperationEventType::Unspecified => "OPERATION_EVENT_TYPE_UNSPECIFIED",
            OperationEventType::OperationEventCreateContentStarted => "OPERATION_EVENT_CREATE_CONTENT_STARTED",
            OperationEventType::OperationEventCreateContentEnded => "OPERATION_EVENT_CREATE_CONTENT_ENDED",
            OperationEventType::OperationEventBuildContentStarted => "OPERATION_EVENT_BUILD_CONTENT_STARTED",
            OperationEventType::OperationEventBuildContentEnded => "OPERATION_EVENT_BUILD_CONTENT_ENDED",
            OperationEventType::OperationEventUpdateContentStarted => "OPERATION_EVENT_UPDATE_CONTENT_STARTED",
            OperationEventType::OperationEventUpdateContentEnded => "OPERATION_EVENT_UPDATE_CONTENT_ENDED",
            OperationEventType::OperationEventDeleteContentStarted => "OPERATION_EVENT_DELETE_CONTENT_STARTED",
            OperationEventType::OperationEventDeleteContentEnded => "OPERATION_EVENT_DELETE_CONTENT_ENDED",
            OperationEventType::OperationEventCreateInstanceStarted => "OPERATION_EVENT_CREATE_INSTANCE_STARTED",
            OperationEventType::OperationEventCreateInstanceEnded => "OPERATION_EVENT_CREATE_INSTANCE_ENDED",
            OperationEventType::OperationEventUpdateInstanceStarted => "OPERATION_EVENT_UPDATE_INSTANCE_STARTED",
            OperationEventType::OperationEventUpdateInstanceEnded => "OPERATION_EVENT_UPDATE_INSTANCE_ENDED",
            OperationEventType::OperationEventDeleteInstanceStarted => "OPERATION_EVENT_DELETE_INSTANCE_STARTED",
            OperationEventType::OperationEventDeleteInstanceEnded => "OPERATION_EVENT_DELETE_INSTANCE_ENDED",
        }
    }
}
/// SessionEventType is the enum value for the state of session.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SessionEventType {
    /// Unspecified session event type.
    Unspecified = 0,
    /// Session in streamer shutting down state.
    SessionEventServerStreamerShuttingDown = 1,
    /// Session in streamer ready state.
    SessionEventServerStreamerReady = 2,
    /// Session in streamer binary started state.
    SessionEventServerStreamerBinaryStarted = 3,
    /// Session in streamer read pod image names state.
    SessionEventServerStreamerReadPodImageNames = 4,
    /// Session in streamer connected to game state.
    SessionEventServerStreamerConnectedToGame = 5,
    /// Session in streamer connected to client state.
    SessionEventServerStreamerConnectedToClient = 6,
    /// Session in streamer disconnected from client state.
    SessionEventServerStreamerDisconnectedFromClient = 7,
    /// Session in streamer received create session request state.
    SessionEventServerStreamerReceivedCreateSessionRequest = 8,
    /// Session in streamer game message to stream closed state.
    SessionEventServerStreamerGameMessageStreamClosed = 9,
    /// Session in streamer game frame stream closed state.
    SessionEventServerStreamerGameFrameStreamClosed = 10,
    /// Session in streamer game message stream error state.
    SessionEventServerStreamerGameMessageStreamError = 11,
    /// Session in streamer game audio stream error state.
    SessionEventServerStreamerGameAudioStreamError = 12,
    /// Session in streamer game audio stream closed state.
    SessionEventServerStreamerGameAudioStreamClosed = 13,
    /// Session in streamer game frame stream error state.
    SessionEventServerStreamerGameFrameStreamError = 14,
    /// Session in game disconnecting after paused too long state.
    SessionEventServerGameDisconnectingAfterPausedTooLong = 15,
    /// Session in streamer received experiment configuration state.
    SessionEventServerStreamerReceivedExperimentConfiguration = 16,
    /// Session in game connected to logging service state.
    SessionEventServerGameConnectedToLoggingService = 17,
    /// Session in streamer determined session options state.
    SessionEventServerStreamerDeterminedSessionOptions = 18,
    /// Session in streamer killed in middle of session state.
    SessionEventServerStreamerKilledInMiddleOfSession = 19,
    /// Session in game updated frame pipeline state.
    SessionEventServerGameUpdatedFramePipeline = 20,
    /// Session in server error state.
    SessionEventServerError = 21,
}
impl SessionEventType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SessionEventType::Unspecified => "SESSION_EVENT_TYPE_UNSPECIFIED",
            SessionEventType::SessionEventServerStreamerShuttingDown => "SESSION_EVENT_SERVER_STREAMER_SHUTTING_DOWN",
            SessionEventType::SessionEventServerStreamerReady => "SESSION_EVENT_SERVER_STREAMER_READY",
            SessionEventType::SessionEventServerStreamerBinaryStarted => "SESSION_EVENT_SERVER_STREAMER_BINARY_STARTED",
            SessionEventType::SessionEventServerStreamerReadPodImageNames => "SESSION_EVENT_SERVER_STREAMER_READ_POD_IMAGE_NAMES",
            SessionEventType::SessionEventServerStreamerConnectedToGame => "SESSION_EVENT_SERVER_STREAMER_CONNECTED_TO_GAME",
            SessionEventType::SessionEventServerStreamerConnectedToClient => "SESSION_EVENT_SERVER_STREAMER_CONNECTED_TO_CLIENT",
            SessionEventType::SessionEventServerStreamerDisconnectedFromClient => "SESSION_EVENT_SERVER_STREAMER_DISCONNECTED_FROM_CLIENT",
            SessionEventType::SessionEventServerStreamerReceivedCreateSessionRequest => "SESSION_EVENT_SERVER_STREAMER_RECEIVED_CREATE_SESSION_REQUEST",
            SessionEventType::SessionEventServerStreamerGameMessageStreamClosed => "SESSION_EVENT_SERVER_STREAMER_GAME_MESSAGE_STREAM_CLOSED",
            SessionEventType::SessionEventServerStreamerGameFrameStreamClosed => "SESSION_EVENT_SERVER_STREAMER_GAME_FRAME_STREAM_CLOSED",
            SessionEventType::SessionEventServerStreamerGameMessageStreamError => "SESSION_EVENT_SERVER_STREAMER_GAME_MESSAGE_STREAM_ERROR",
            SessionEventType::SessionEventServerStreamerGameAudioStreamError => "SESSION_EVENT_SERVER_STREAMER_GAME_AUDIO_STREAM_ERROR",
            SessionEventType::SessionEventServerStreamerGameAudioStreamClosed => "SESSION_EVENT_SERVER_STREAMER_GAME_AUDIO_STREAM_CLOSED",
            SessionEventType::SessionEventServerStreamerGameFrameStreamError => "SESSION_EVENT_SERVER_STREAMER_GAME_FRAME_STREAM_ERROR",
            SessionEventType::SessionEventServerGameDisconnectingAfterPausedTooLong => "SESSION_EVENT_SERVER_GAME_DISCONNECTING_AFTER_PAUSED_TOO_LONG",
            SessionEventType::SessionEventServerStreamerReceivedExperimentConfiguration => "SESSION_EVENT_SERVER_STREAMER_RECEIVED_EXPERIMENT_CONFIGURATION",
            SessionEventType::SessionEventServerGameConnectedToLoggingService => "SESSION_EVENT_SERVER_GAME_CONNECTED_TO_LOGGING_SERVICE",
            SessionEventType::SessionEventServerStreamerDeterminedSessionOptions => "SESSION_EVENT_SERVER_STREAMER_DETERMINED_SESSION_OPTIONS",
            SessionEventType::SessionEventServerStreamerKilledInMiddleOfSession => "SESSION_EVENT_SERVER_STREAMER_KILLED_IN_MIDDLE_OF_SESSION",
            SessionEventType::SessionEventServerGameUpdatedFramePipeline => "SESSION_EVENT_SERVER_GAME_UPDATED_FRAME_PIPELINE",
            SessionEventType::SessionEventServerError => "SESSION_EVENT_SERVER_ERROR",
        }
    }
}
