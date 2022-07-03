/// The type of the parameter.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValueType {
    #[prost(oneof = "value_type::Value", tags = "1, 2, 3, 4, 5, 6, 7, 8, 9")]
    pub value: ::core::option::Option<value_type::Value>,
}
/// Nested message and enum types in `ValueType`.
pub mod value_type {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Value {
        /// String.
        #[prost(string, tag = "1")]
        StringValue(::prost::alloc::string::String),
        /// Integer.
        #[prost(int64, tag = "2")]
        IntValue(i64),
        /// Double Number.
        #[prost(double, tag = "3")]
        DoubleValue(f64),
        /// Boolean.
        #[prost(bool, tag = "4")]
        BooleanValue(bool),
        /// String Array.
        #[prost(message, tag = "5")]
        StringArray(super::StringParameterArray),
        /// Integer Array.
        #[prost(message, tag = "6")]
        IntArray(super::IntParameterArray),
        /// Double Number Array.
        #[prost(message, tag = "7")]
        DoubleArray(super::DoubleParameterArray),
        /// Boolean Array.
        #[prost(message, tag = "8")]
        BooleanArray(super::BooleanParameterArray),
        /// Json.
        #[prost(string, tag = "9")]
        JsonValue(::prost::alloc::string::String),
    }
}
/// This message only contains a field of string array.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StringParameterArray {
    /// String array.
    #[prost(string, repeated, tag = "1")]
    pub string_values: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// This message only contains a field of integer array.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IntParameterArray {
    /// Integer array.
    #[prost(int64, repeated, tag = "1")]
    pub int_values: ::prost::alloc::vec::Vec<i64>,
}
/// This message only contains a field of double number array.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DoubleParameterArray {
    /// Double number array.
    #[prost(double, repeated, tag = "1")]
    pub double_values: ::prost::alloc::vec::Vec<f64>,
}
/// This message only contains a field of boolean array.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BooleanParameterArray {
    /// Boolean array.
    #[prost(bool, repeated, tag = "1")]
    pub boolean_values: ::prost::alloc::vec::Vec<bool>,
}
/// This message is used for processing and persisting (when applicable) key
/// value pair parameters for each event in the event bus.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventParameter {
    /// Key is used to retrieve the corresponding parameter value. This should be
    /// unique for a given fired event. These parameters must be predefined in the
    /// integration definition.
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
    /// Values for the defined keys. Each value can either be string, int, double
    /// or any proto message.
    #[prost(message, optional, tag = "2")]
    pub value: ::core::option::Option<ValueType>,
}
/// Options for how to validate json schemas.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum JsonValidationOption {
    /// As per the default behavior, no validation will be run. Will not override
    /// any option set in a Task.
    Unspecified = 0,
    /// Do not run any validation against JSON schemas.
    Skip = 1,
    /// Validate all potential input JSON parameters against schemas specified in
    /// IntegrationParameter.
    PreExecution = 2,
    /// Validate all potential output JSON parameters against schemas specified in
    /// IntegrationParameter.
    PostExecution = 3,
    /// Perform both PRE_EXECUTION and POST_EXECUTION validations.
    PrePostExecution = 4,
}
/// Enum Product.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Product {
    /// Default value.
    Unspecified = 0,
    /// Integration Platform.
    Ip = 1,
    /// Apigee.
    Apigee = 2,
    /// Security Command Center.
    Security = 3,
}
/// The task configuration details. This is not the implementation of Task.
/// There might be multiple TaskConfigs for the same Task.
/// (-- Next available id: 12 --)
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TaskConfig {
    /// Optional. The name for the task.
    #[prost(string, tag = "1")]
    pub task: ::prost::alloc::string::String,
    /// Required. The identifier of this task within its parent event config,
    /// specified by the client. This should be unique among all the tasks belong
    /// to the same event config. We use this field as the identifier to
    /// find next tasks (via field `next_tasks.task_id`).
    #[prost(string, tag = "2")]
    pub task_id: ::prost::alloc::string::String,
    /// Optional. The customized parameters the user can pass to this task.
    #[prost(map = "string, message", tag = "3")]
    pub parameters: ::std::collections::HashMap<::prost::alloc::string::String, EventParameter>,
    /// Optional. Determines the number of times the
    /// task will be retried on failure and with what retry strategy.
    /// This is applicable for asynchronous calls to Eventbus alone (Post To
    /// Queue, Schedule etc.).
    #[prost(message, optional, tag = "4")]
    pub failure_policy: ::core::option::Option<FailurePolicy>,
    /// Optional. Determines the number of times the
    /// task will be retried on failure and with what retry strategy.
    /// This is applicable for synchronous calls to Eventbus alone (Post).
    #[prost(message, optional, tag = "5")]
    pub synchronous_call_failure_policy: ::core::option::Option<FailurePolicy>,
    /// Optional. The set of tasks that are next in line to be executed as per the
    /// execution graph defined for the parent event, specified by
    /// `event_config_id`. Each of these next tasks are executed
    /// only if the condition associated with them evaluates to true.
    #[prost(message, repeated, tag = "6")]
    pub next_tasks: ::prost::alloc::vec::Vec<NextTask>,
    /// Optional. The policy dictating the execution of the next set of tasks for the current
    /// task.
    #[prost(enumeration = "task_config::NextTasksExecutionPolicy", tag = "7")]
    pub next_tasks_execution_policy: i32,
    /// Optional. The policy dictating the execution strategy of this task.
    #[prost(enumeration = "task_config::TaskExecutionStrategy", tag = "8")]
    pub task_execution_strategy: i32,
    /// Optional. User-provided label that is attached to this TaskConfig in the UI.
    #[prost(string, tag = "9")]
    pub display_name: ::prost::alloc::string::String,
    /// Optional. Determines what action to take upon successful task completion.
    #[prost(message, optional, tag = "10")]
    pub success_policy: ::core::option::Option<SuccessPolicy>,
    /// Optional. If set, overrides the option configured in the Task implementation class.
    #[prost(enumeration = "JsonValidationOption", tag = "11")]
    pub json_validation_option: i32,
}
/// Nested message and enum types in `TaskConfig`.
pub mod task_config {
    /// Various policies for executing the next set of tasks.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum NextTasksExecutionPolicy {
        /// Default.
        Unspecified = 0,
        /// Execute all the tasks that satisfy their associated condition.
        RunAllMatch = 1,
        /// Execute the first task that satisfies the associated condition.
        RunFirstMatch = 2,
    }
    /// Various policies to trigger the execution of this task.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum TaskExecutionStrategy {
        /// Default. If the strategy is not set explicitly, it will default to
        /// `WHEN_ALL_SUCCEED`.
        Unspecified = 0,
        /// Wait until all of its previous tasks finished execution, then verify at
        /// least one of the edge conditions is met, and execute if possible. This
        /// should be considered as WHEN_ALL_TASKS_SUCCEED.
        WhenAllSucceed = 1,
        /// Start execution as long as any of its previous tasks finished execution
        /// and the corresponding edge condition is met (since we will execute if
        /// only that succeeding edge condition is met).
        WhenAnySucceed = 2,
        /// Wait until all of its previous tasks finished execution, then verify
        /// the all edge conditions are met and execute if possible.
        WhenAllTasksAndConditionsSucceed = 3,
    }
}
/// Policy that dictates the behavior for the task after it completes
/// successfully.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SuccessPolicy {
    /// State to which the execution snapshot status will be set if the task
    /// succeeds.
    #[prost(enumeration = "success_policy::FinalState", tag = "1")]
    pub final_state: i32,
}
/// Nested message and enum types in `SuccessPolicy`.
pub mod success_policy {
    /// The state of execution.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum FinalState {
        /// UNSPECIFIED.
        Unspecified = 0,
        /// The default behavior, where successful tasks will be marked as SUCCEEDED.
        Succeeded = 1,
        /// Sets the state to SUSPENDED after executing.  This is required for
        /// SuspensionTask; event execution will continue once the user calls
        /// ResolveSuspensions with the event_execution_info_id and the task number.
        Suspended = 2,
    }
}
/// Policy that defines the task retry logic and failure type. If no
/// FailurePolicy is defined for a task, all its dependent tasks will not be
/// executed (i.e, a `retry_strategy` of NONE will be applied).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FailurePolicy {
    /// Defines what happens to the task upon failure.
    #[prost(enumeration = "failure_policy::RetryStrategy", tag = "1")]
    pub retry_strategy: i32,
    /// Required if retry_strategy is FIXED_INTERVAL or
    /// LINEAR/EXPONENTIAL_BACKOFF/RESTART_INTEGRATION_WITH_BACKOFF. Defines the
    /// number of times the task will be retried if failed.
    #[prost(int32, tag = "2")]
    pub max_retries: i32,
    /// Required if retry_strategy is FIXED_INTERVAL or
    /// LINEAR/EXPONENTIAL_BACKOFF/RESTART_INTEGRATION_WITH_BACKOFF. Defines the
    /// initial interval in seconds for backoff.
    #[prost(message, optional, tag = "3")]
    pub interval_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Nested message and enum types in `FailurePolicy`.
pub mod failure_policy {
    /// The behavior when the taks failed.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum RetryStrategy {
        /// UNSPECIFIED.
        Unspecified = 0,
        /// Ignores the failure of this task. The rest of the integration will be
        /// executed Assuming this task succeeded.
        Ignore = 1,
        /// Causes a permanent failure of the task. However, if the last task(s)
        /// of event was successfully completed despite the failure of this task,
        /// it has no impact on the integration.
        None = 2,
        /// Causes a permanent failure of the event. It is different from NONE
        /// because this will mark the event as FAILED by shutting down the
        /// event execution.
        Fatal = 3,
        /// The task will be retried from the failed task onwards after a fixed
        /// delay. A max-retry count is required to be specified with this
        /// strategy. A jitter is added to each exponential interval so that
        /// concurrently failing tasks of the same type do not end up retrying
        /// after the exact same exponential interval. max_retries and
        /// interval_in_seconds must be specified.
        FixedInterval = 4,
        /// The task will be retried from the failed task onwards after a fixed
        /// delay that linearly increases with each retry attempt. A jitter is
        /// added to each exponential interval so that concurrently failing tasks
        /// of the same type do not end up retrying after the exact same
        /// exponential interval. A max-retry count is required to be specified
        /// with this strategy. max_retries and interval_in_seconds must be
        /// specified.
        LinearBackoff = 5,
        /// The task will be retried after an exponentially increasing period of
        /// time with each failure. A jitter is added to each exponential interval
        /// so that concurrently failing tasks of the same type do not end up
        /// retrying after the exact same exponential interval. A max-retry count
        /// is required to be specified with this strategy. `max_retries` and
        /// `interval_in_seconds` must be specified.
        ExponentialBackoff = 6,
        /// The entire integration will be restarted with the initial parameters that
        /// were set when the event was fired. A max-retry count is required to be
        /// specified with this strategy. `max_retries` and `interval_in_seconds`
        /// must be specified.
        RestartIntegrationWithBackoff = 7,
    }
}
/// The task that is next in line to be executed, if the
/// condition specified evaluated to true.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NextTask {
    /// ID of the next task.
    #[prost(string, tag = "1")]
    pub task_config_id: ::prost::alloc::string::String,
    /// Task number of the next task.
    #[prost(string, tag = "2")]
    pub task_id: ::prost::alloc::string::String,
    /// Standard filter expression for this task to become an eligible next task.
    #[prost(string, tag = "3")]
    pub condition: ::prost::alloc::string::String,
    /// User-provided label that is attached to this edge in the UI.
    #[prost(string, tag = "4")]
    pub display_name: ::prost::alloc::string::String,
}
/// Log entry to log execution info for the monitored resource
/// `integrations.googleapis.com/IntegrationVersion`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecutionInfo {
    /// Auto-generated primary key.
    #[prost(string, tag = "1")]
    pub event_execution_info_id: ::prost::alloc::string::String,
    /// Name of the integration.
    #[prost(string, tag = "2")]
    pub integration: ::prost::alloc::string::String,
    /// Pointer to the active version it is executing.
    #[prost(string, tag = "3")]
    pub integration_version: ::prost::alloc::string::String,
    /// The event data user sends as request.
    #[prost(string, tag = "4")]
    pub project_id: ::prost::alloc::string::String,
    /// The trigger id of the integration trigger config. If both trigger_id
    /// and client_id is present, the integration is executed from the start tasks
    /// provided by the matching trigger config otherwise it is executed from the
    /// default start tasks.
    #[prost(string, tag = "5")]
    pub trigger_id: ::prost::alloc::string::String,
    /// Event parameters come in as part of the request.
    #[prost(map = "string, message", tag = "6")]
    pub request_params: ::std::collections::HashMap<::prost::alloc::string::String, EventParameter>,
    /// Event parameters come out as part of the response.
    #[prost(map = "string, message", tag = "7")]
    pub response_params:
        ::std::collections::HashMap<::prost::alloc::string::String, EventParameter>,
    /// The ways user posts this event.
    #[prost(enumeration = "execution_info::PostMethod", tag = "8")]
    pub post_method: i32,
    /// The execution info about this event.
    #[prost(message, optional, tag = "9")]
    pub event_execution_details: ::core::option::Option<EventExecutionDetails>,
    /// Errors, warnings, and informationals associated with the workflow/task.
    /// The order in which the errors were added by the workflow/task is
    /// maintained.
    #[prost(message, repeated, tag = "10")]
    pub errors: ::prost::alloc::vec::Vec<ErrorDetail>,
    /// Which Google product the execution_info belongs to. If not set, the
    /// execution_info belongs to Integration Platform by default.
    #[prost(enumeration = "Product", tag = "11")]
    pub product: i32,
    /// This is used to de-dup incoming request.
    #[prost(string, tag = "12")]
    pub request_id: ::prost::alloc::string::String,
    /// The configuration details for a task.
    #[prost(message, repeated, tag = "13")]
    pub task_configs: ::prost::alloc::vec::Vec<TaskConfig>,
}
/// Nested message and enum types in `ExecutionInfo`.
pub mod execution_info {
    /// PostMethod Enum
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum PostMethod {
        /// Default value.
        Unspecified = 0,
        /// Sync post.
        Post = 1,
        /// Async post with schedule time.
        Schedule = 2,
    }
}
/// Contains the details of the execution info of this event: this includes
/// the tasks execution details plus the event execution statistics.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventExecutionDetails {
    /// The execution state of this event.
    #[prost(
        enumeration = "event_execution_details::EventExecutionState",
        tag = "1"
    )]
    pub event_execution_state: i32,
    /// After snapshot migration, this field will no longer be populated, but old
    /// execution snapshots will still be accessible.
    #[prost(message, repeated, tag = "2")]
    pub event_execution_snapshot: ::prost::alloc::vec::Vec<EventExecutionSnapshot>,
    /// Status for the current event execution attempt.
    #[prost(message, repeated, tag = "3")]
    pub event_attempt_stats: ::prost::alloc::vec::Vec<AttemptStats>,
    /// Next scheduled execution time in case the execution status was
    /// RETRY_ON_HOLD.
    #[prost(message, optional, tag = "4")]
    pub next_execution_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Indicates the number of times the execution has restarted from the
    /// beginning.
    #[prost(int32, tag = "5")]
    pub event_retries_count: i32,
}
/// Nested message and enum types in `EventExecutionDetails`.
pub mod event_execution_details {
    /// Enum EventExecutionState.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum EventExecutionState {
        /// Default value.
        Unspecified = 0,
        /// Event is received and waiting for the execution. This happens when
        /// firing the event via "postToQueue" or "schedule".
        OnHold = 1,
        /// Event is under processing.
        InProcess = 2,
        /// Event execution successfully finished. There's no more change after
        /// this state.
        Succeeded = 3,
        /// Event execution failed. There's no more change after this state.
        Failed = 4,
        /// Event execution canceled by user. There's no more change after this
        /// state.
        Cancelled = 5,
        /// Event execution failed and waiting for retry.
        RetryOnHold = 6,
        /// Event execution suspended and waiting for manual intervention.
        Suspended = 7,
    }
}
/// Contains the snapshot of the event execution for a given checkpoint.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventExecutionSnapshot {
    /// Indicates "right after which checkpoint task's execution" this snapshot
    /// is taken.
    #[prost(string, tag = "1")]
    pub checkpoint_task_number: ::prost::alloc::string::String,
    /// Indicates when this snapshot is taken.
    #[prost(message, optional, tag = "2")]
    pub snapshot_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Snapshot metadata.
    #[prost(message, optional, tag = "3")]
    pub event_execution_snapshot_metadata:
        ::core::option::Option<event_execution_snapshot::EventExecutionSnapshotMetadata>,
    /// All of the task execution details at the given point of time.
    #[prost(message, repeated, tag = "4")]
    pub task_execution_details: ::prost::alloc::vec::Vec<TaskExecutionDetails>,
    /// All of the computed conditions that been calculated.
    #[prost(message, repeated, tag = "5")]
    pub condition_results: ::prost::alloc::vec::Vec<ConditionResult>,
    /// The parameters in Event object.
    #[prost(map = "string, message", tag = "6")]
    pub event_params: ::std::collections::HashMap<::prost::alloc::string::String, EventParameter>,
    /// The parameters in Event object that differs from last snapshot.
    #[prost(map = "string, message", tag = "7")]
    pub diff_params: ::std::collections::HashMap<::prost::alloc::string::String, EventParameter>,
}
/// Nested message and enum types in `EventExecutionSnapshot`.
pub mod event_execution_snapshot {
    /// Metadata for the event/task retry.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct EventExecutionSnapshotMetadata {
        /// The task number associated with this snapshot. Could be empty.
        #[prost(string, tag = "1")]
        pub task_number: ::prost::alloc::string::String,
        /// the task name associated with this snapshot. Could be empty.
        #[prost(string, tag = "2")]
        pub task: ::prost::alloc::string::String,
        /// the event attempt number this snapshot belongs to.
        #[prost(int32, tag = "3")]
        pub event_attempt_num: i32,
        /// the task attempt number this snapshot belongs to. Could be empty.
        #[prost(int32, tag = "4")]
        pub task_attempt_num: i32,
    }
}
/// Contains the details of the execution of this task.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TaskExecutionDetails {
    /// Pointer to the task config it used for execution.
    #[prost(string, tag = "1")]
    pub task_number: ::prost::alloc::string::String,
    /// The execution state of this task.
    #[prost(enumeration = "task_execution_details::TaskExecutionState", tag = "2")]
    pub task_execution_state: i32,
    /// Status for the current task execution attempt.
    #[prost(message, repeated, tag = "3")]
    pub task_attempt_stats: ::prost::alloc::vec::Vec<AttemptStats>,
}
/// Nested message and enum types in `TaskExecutionDetails`.
pub mod task_execution_details {
    /// Enum TaskExecutionState.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum TaskExecutionState {
        /// Default value.
        Unspecified = 0,
        /// Task is waiting for its precondition tasks to finish to start the
        /// execution.
        PendingExecution = 1,
        /// Task is under processing.
        InProcess = 2,
        /// Task execution successfully finished. There's no more change after
        /// this state.
        Succeed = 3,
        /// Task execution failed. There's no more change after this state.
        Failed = 4,
        /// Task execution failed and cause the whole event execution to fail
        /// immediately. There's no more change after this state.
        Fatal = 5,
        /// Task execution failed and waiting for retry.
        RetryOnHold = 6,
        /// Task execution skipped. This happens when its precondition wasn't met,
        /// or the event execution been canceled before reach to the task.
        /// There's no more changes after this state.
        Skipped = 7,
        /// Task execution canceled when in progress. This happens when event
        /// execution been canceled or any other task fall in fatal state.
        Cancelled = 8,
        /// Task is waiting for its dependency tasks' rollback to finish to start
        /// its rollback.
        PendingRollback = 9,
        /// Task is rolling back.
        RollbackInProcess = 10,
        /// Task is rolled back. This is the state we will set regardless of
        /// rollback succeeding or failing.
        Rolledback = 11,
        /// Task is a SuspensionTask which has executed once, creating a pending
        /// suspension.
        Suspended = 12,
    }
}
/// Status for the execution attempt.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AttemptStats {
    /// The start time of the event execution for current attempt. This could be
    /// in the future if it's been scheduled.
    #[prost(message, optional, tag = "1")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The end time of the event execution for current attempt.
    #[prost(message, optional, tag = "2")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// An error, warning, or information message associated with an integration.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ErrorDetail {
    /// The full text of the error message, including any parameters that were
    /// thrown along with the exception.
    #[prost(string, tag = "1")]
    pub error_message: ::prost::alloc::string::String,
    /// The task try-number, in which, the error occurred.  If zero, the error
    /// happened at the event level.
    #[prost(int32, tag = "2")]
    pub task_number: i32,
}
/// Contains the combined condition calculation results.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConditionResult {
    /// the current task number.
    #[prost(string, tag = "1")]
    pub current_task_number: ::prost::alloc::string::String,
    /// the next task number.
    #[prost(string, tag = "2")]
    pub next_task_number: ::prost::alloc::string::String,
    /// the result comes out after evaluate the combined condition. True if there's
    /// no combined condition specified.
    #[prost(bool, tag = "3")]
    pub result: bool,
}
