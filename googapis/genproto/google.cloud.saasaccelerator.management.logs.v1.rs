#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InstanceEvent {
    /// The type of the event, e.g. Create, Update, etc.
    #[prost(string, tag = "1")]
    pub verb: std::string::String,
    /// The state of the instance, e.g. "RETRYING_CREATE_INSTANCE".
    #[prost(string, tag = "2")]
    pub stage: std::string::String,
    /// A human-readable log message, e.g. "error in stage: CREATING, err: location
    /// not available".
    #[prost(string, tag = "3")]
    pub msg: std::string::String,
    /// The ID to uniquely locate all logs associated with a given request.
    #[prost(string, tag = "4")]
    pub trace_id: std::string::String,
    /// The instance node which is the subject of the operation, if known.
    /// Currently unused as tf actuation does not manage nodes.
    #[prost(string, tag = "5")]
    pub node_id: std::string::String,
}
