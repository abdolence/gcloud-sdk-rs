/// This is an event
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Event {
    /// The event_time field displays the time when the event was reported
    #[prost(string, tag = "1")]
    pub event_time: ::prost::alloc::string::String,
    /// The srcid field displays unique id of the event occurred in the backup
    /// appliance
    #[prost(int64, tag = "2")]
    pub srcid: i64,
    /// The errormessage field describes the detailed error associated with the
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
    pub appliance_name: i64,
    /// The appname field displays name of the application associated with the
    /// event
    #[prost(string, tag = "7")]
    pub app_name: ::prost::alloc::string::String,
    /// The apptype field displays type of the application associated with the
    /// event
    #[prost(string, tag = "8")]
    pub app_type: ::prost::alloc::string::String,
    /// The jobname field displays name of the job associated with the event
    #[prost(string, tag = "9")]
    pub job_name: ::prost::alloc::string::String,
}
