/// Logged during a workflow execution if the customer has requested call
/// logging.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EngineCallLog {
    /// The execution ID of the execution where the call occurred.
    #[prost(string, tag = "1")]
    pub execution_id: ::prost::alloc::string::String,
    /// The point in time when the activity occurred.
    #[prost(message, optional, tag = "2")]
    pub activity_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The state of the function execution.
    #[prost(enumeration = "engine_call_log::State", tag = "3")]
    pub state: i32,
    /// The name of the step in which the call took place, truncated if necessary.
    #[prost(string, tag = "4")]
    pub step: ::prost::alloc::string::String,
    /// The name of the target of the call, truncated if necessary.
    #[prost(string, tag = "5")]
    pub callee: ::prost::alloc::string::String,
    #[prost(oneof = "engine_call_log::Details", tags = "6, 7, 8")]
    pub details: ::core::option::Option<engine_call_log::Details>,
}
/// Nested message and enum types in `EngineCallLog`.
pub mod engine_call_log {
    /// Information about an argument to a called function.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CallArg {
        /// A function argument, serialized to a string. This may be truncated for
        /// size reasons.
        #[prost(string, tag = "1")]
        pub argument: ::prost::alloc::string::String,
    }
    /// Information about the start of a call.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Begun {
        /// The arguments passed to the function.
        #[prost(message, repeated, tag = "1")]
        pub args: ::prost::alloc::vec::Vec<CallArg>,
    }
    /// Information about the end of a successful call.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Succeeded {
        /// The time when the call started.
        #[prost(message, optional, tag = "1")]
        pub call_start_time: ::core::option::Option<::prost_types::Timestamp>,
        /// The result of the call, if the call succeeded, serialized to a string.
        /// This may be truncated for size reasons.
        #[prost(string, tag = "2")]
        pub response: ::prost::alloc::string::String,
    }
    /// Information about the end of a failed call.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ExceptionRaised {
        /// The time when the call started.
        #[prost(message, optional, tag = "1")]
        pub call_start_time: ::core::option::Option<::prost_types::Timestamp>,
        /// The exception message which terminated the call, truncated if necessary.
        #[prost(string, tag = "2")]
        pub exception: ::prost::alloc::string::String,
        /// The name of the step where the failure originates, if known. Truncated
        /// if necessary.
        #[prost(string, tag = "3")]
        pub origin: ::prost::alloc::string::String,
    }
    /// The state of a function call.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// Function call state is unspecified or unknown.
        Unspecified = 0,
        /// Function call is starting.
        Begun = 1,
        /// Function call has completed successfully.
        Succeeded = 2,
        /// Function call did not succeed because an exception was raised.
        ExceptionRaised = 3,
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Details {
        /// Appears at the start of a call.
        #[prost(message, tag = "6")]
        Begun(Begun),
        /// Appears when a call returns successfully.
        #[prost(message, tag = "7")]
        Succeeded(Succeeded),
        /// Appears when a call returns because an exception was raised.
        #[prost(message, tag = "8")]
        ExceptionRaised(ExceptionRaised),
    }
}
/// Logged during the lifetime of Workflow Execution.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecutionsSystemLog {
    /// Human readable contents of the log in English. The size limit is 5 kB.
    #[prost(string, tag = "1")]
    pub message: ::prost::alloc::string::String,
    /// The absolute point in time when the activity happened.
    #[prost(message, optional, tag = "2")]
    pub activity_time: ::core::option::Option<::prost_types::Timestamp>,
    /// State of the execution when the log was created.
    #[prost(enumeration = "executions_system_log::State", tag = "3")]
    pub state: i32,
    /// Detailed log information.
    #[prost(oneof = "executions_system_log::Details", tags = "4, 5, 6")]
    pub details: ::core::option::Option<executions_system_log::Details>,
}
/// Nested message and enum types in `ExecutionsSystemLog`.
pub mod executions_system_log {
    /// Detailed information about the start of the execution.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Start {
        /// The execution input argument.
        #[prost(string, tag = "2")]
        pub argument: ::prost::alloc::string::String,
    }
    /// Detailed information about the successful finish of the execution.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Success {
        /// The final result of the execution.
        #[prost(string, tag = "2")]
        pub result: ::prost::alloc::string::String,
    }
    /// Detailed information about the execution failure.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Failure {
        /// The exception message, e.g. "division by zero". The size limit is 1 kB.
        #[prost(string, tag = "1")]
        pub exception: ::prost::alloc::string::String,
        /// The code location of the statement that has created the log. For example,
        /// a log created in subworkflow 'Foo' in step 'bar' will have its source
        /// equal to 'Foo.bar'. The size limit is 1 kB.
        #[prost(string, tag = "2")]
        pub source: ::prost::alloc::string::String,
    }
    /// Possible states of the execution. There could be more states in the future.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// Invalid state.
        Unspecified = 0,
        /// The Workflow Execution is in progress.
        Active = 1,
        /// The Workflow Execution has finished successfully.
        Succeeded = 2,
        /// The Workflow Execution failed with an error.
        Failed = 3,
        /// The Workflow Execution has been stopped intentionally.
        Cancelled = 4,
    }
    /// Detailed log information.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Details {
        /// Appears only in the log created when the execution has started.
        #[prost(message, tag = "4")]
        Start(Start),
        /// Appears only in the log created when the execution has finished
        /// successfully.
        #[prost(message, tag = "5")]
        Success(Success),
        /// Appears only in the log created when the execution has failed.
        #[prost(message, tag = "6")]
        Failure(Failure),
    }
}
