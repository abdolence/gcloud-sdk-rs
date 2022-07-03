// Defines protos for runtime related platform logs

/// Log content of an event related to a runtime.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RuntimeEvent {
    /// Required. Type of event.
    #[prost(enumeration = "runtime_event::EventType", tag = "1")]
    pub r#type: i32,
    /// Optional. Additional metadata for the event.
    #[prost(map = "string, string", tag = "2")]
    pub details:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
}
/// Nested message and enum types in `RuntimeEvent`.
pub mod runtime_event {
    /// Defines event type.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum EventType {
        /// Event is not specified.
        Unspecified = 0,
        /// Runtime state has been updated.
        RuntimeStateChangeEvent = 1,
    }
}
