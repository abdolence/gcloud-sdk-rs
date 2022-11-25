/// This is an event
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Event {
    /// The event_time field displays the time when the event was reported
    #[prost(message, optional, tag = "1")]
    pub event_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The srcid field displays unique id of the event occurred in the backup
    /// appliance
    #[prost(int64, tag = "2")]
    pub srcid: i64,
    /// The error_message field describes the detailed error associated with the
    /// event
    #[prost(string, tag = "3")]
    pub error_message: ::prost::alloc::string::String,
    /// The event_id field displays unique id associated with the error
    #[prost(int32, tag = "4")]
    pub event_id: i32,
    /// The component field displays the source of the event
    #[prost(string, tag = "5")]
    pub component: ::prost::alloc::string::String,
    /// The appliance_id field displays unique id of the appliance on which event
    /// occurred
    #[prost(int64, tag = "6")]
    pub appliance_id: i64,
    /// The appliance_name field displays name of the appliance on which event
    /// occurred
    #[prost(string, tag = "7")]
    pub appliance_name: ::prost::alloc::string::String,
    /// The source_event_time field displays the time when the event occurred at
    /// appliance
    #[prost(message, optional, tag = "8")]
    pub source_event_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The app_name field displays name of the application associated with the
    /// event
    #[prost(string, tag = "9")]
    pub app_name: ::prost::alloc::string::String,
    /// The app_type field displays type of the application associated with the
    /// event
    #[prost(string, tag = "10")]
    pub app_type: ::prost::alloc::string::String,
    /// The job_name field displays name of the job associated with the event
    #[prost(string, tag = "11")]
    pub job_name: ::prost::alloc::string::String,
}
