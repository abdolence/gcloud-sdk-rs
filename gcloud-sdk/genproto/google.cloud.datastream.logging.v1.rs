/// Stackdriver structured-payload for events related to a connection.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConnectionActivityLogEntry {
    /// A code representing the event.
    #[prost(string, tag = "1")]
    pub event_code: ::prost::alloc::string::String,
    /// A free-text message describing the event.
    #[prost(string, tag = "2")]
    pub event_message: ::prost::alloc::string::String,
}
/// Stackdriver structured-payload for events related to a stream.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamActivityLogEntry {
    /// A code representing the event.
    #[prost(string, tag = "1")]
    pub event_code: ::prost::alloc::string::String,
    /// A free-text message describing the event.
    #[prost(string, tag = "2")]
    pub event_message: ::prost::alloc::string::String,
}
