/// Volume describes a volume and parameters for it to be mounted to a VM.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Volume {
    /// The mount path for the volume, e.g. /mnt/disks/share.
    #[prost(string, tag = "4")]
    pub mount_path: ::prost::alloc::string::String,
    /// For Google Cloud Storage (GCS), mount options are the options supported by
    /// the gcsfuse tool (<https://github.com/GoogleCloudPlatform/gcsfuse>).
    /// For existing persistent disks, mount options provided by the
    /// mount command (<https://man7.org/linux/man-pages/man8/mount.8.html>) except
    /// writing are supported. This is due to restrictions of multi-writer mode
    /// (<https://cloud.google.com/compute/docs/disks/sharing-disks-between-vms>).
    /// For other attached disks and Network File System (NFS), mount options are
    /// these supported by the mount command
    /// (<https://man7.org/linux/man-pages/man8/mount.8.html>).
    #[prost(string, repeated, tag = "5")]
    pub mount_options: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The source for the volume.
    #[prost(oneof = "volume::Source", tags = "1, 3, 6")]
    pub source: ::core::option::Option<volume::Source>,
}
/// Nested message and enum types in `Volume`.
pub mod volume {
    /// The source for the volume.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Source {
        /// A Network File System (NFS) volume. For example, a
        /// Filestore file share.
        #[prost(message, tag = "1")]
        Nfs(super::Nfs),
        /// A Google Cloud Storage (GCS) volume.
        #[prost(message, tag = "3")]
        Gcs(super::Gcs),
        /// Device name of an attached disk volume, which should align with a
        /// device_name specified by
        /// job.allocation_policy.instances\[0].policy.disks[i\].device_name or
        /// defined by the given instance template in
        /// job.allocation_policy.instances\[0\].instance_template.
        #[prost(string, tag = "6")]
        DeviceName(::prost::alloc::string::String),
    }
}
/// Represents an NFS volume.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Nfs {
    /// The IP address of the NFS.
    #[prost(string, tag = "1")]
    pub server: ::prost::alloc::string::String,
    /// Remote source path exported from the NFS, e.g., "/share".
    #[prost(string, tag = "2")]
    pub remote_path: ::prost::alloc::string::String,
}
/// Represents a Google Cloud Storage volume.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Gcs {
    /// Remote path, either a bucket name or a subdirectory of a bucket, e.g.:
    /// bucket_name, bucket_name/subdirectory/
    #[prost(string, tag = "1")]
    pub remote_path: ::prost::alloc::string::String,
}
/// Compute resource requirements
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ComputeResource {
    /// The milliCPU count.
    #[prost(int64, tag = "1")]
    pub cpu_milli: i64,
    /// Memory in MiB.
    #[prost(int64, tag = "2")]
    pub memory_mib: i64,
    /// Extra boot disk size in MiB for each task.
    #[prost(int64, tag = "4")]
    pub boot_disk_mib: i64,
}
/// Status event
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StatusEvent {
    /// Type of the event.
    #[prost(string, tag = "3")]
    pub r#type: ::prost::alloc::string::String,
    /// Description of the event.
    #[prost(string, tag = "1")]
    pub description: ::prost::alloc::string::String,
    /// The time this event occurred.
    #[prost(message, optional, tag = "2")]
    pub event_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Task Execution
    #[prost(message, optional, tag = "4")]
    pub task_execution: ::core::option::Option<TaskExecution>,
    /// Task State
    #[prost(enumeration = "task_status::State", tag = "5")]
    pub task_state: i32,
}
/// This Task Execution field includes detail information for
/// task execution procedures, based on StatusEvent types.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TaskExecution {
    /// When task is completed as the status of FAILED or SUCCEEDED,
    /// exit code is for one task execution result, default is 0 as success.
    #[prost(int32, tag = "1")]
    pub exit_code: i32,
}
/// Status of a task
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TaskStatus {
    /// Task state
    #[prost(enumeration = "task_status::State", tag = "1")]
    pub state: i32,
    /// Detailed info about why the state is reached.
    #[prost(message, repeated, tag = "2")]
    pub status_events: ::prost::alloc::vec::Vec<StatusEvent>,
}
/// Nested message and enum types in `TaskStatus`.
pub mod task_status {
    /// Task states.
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum State {
        /// Unknown state.
        Unspecified = 0,
        /// The Task is created and waiting for resources.
        Pending = 1,
        /// The Task is assigned to at least one VM.
        Assigned = 2,
        /// The Task is running.
        Running = 3,
        /// The Task has failed.
        Failed = 4,
        /// The Task has succeeded.
        Succeeded = 5,
        /// The Task has not been executed when the Job finishes.
        Unexecuted = 6,
    }
    impl State {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                State::Unspecified => "STATE_UNSPECIFIED",
                State::Pending => "PENDING",
                State::Assigned => "ASSIGNED",
                State::Running => "RUNNING",
                State::Failed => "FAILED",
                State::Succeeded => "SUCCEEDED",
                State::Unexecuted => "UNEXECUTED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "PENDING" => Some(Self::Pending),
                "ASSIGNED" => Some(Self::Assigned),
                "RUNNING" => Some(Self::Running),
                "FAILED" => Some(Self::Failed),
                "SUCCEEDED" => Some(Self::Succeeded),
                "UNEXECUTED" => Some(Self::Unexecuted),
                _ => None,
            }
        }
    }
}
/// Runnable describes instructions for executing a specific script or container
/// as part of a Task.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Runnable {
    /// Normally, a non-zero exit status causes the Task to fail. This flag allows
    /// execution of other Runnables to continue instead.
    #[prost(bool, tag = "3")]
    pub ignore_exit_status: bool,
    /// This flag allows a Runnable to continue running in the background while the
    /// Task executes subsequent Runnables. This is useful to provide services to
    /// other Runnables (or to provide debugging support tools like SSH servers).
    #[prost(bool, tag = "4")]
    pub background: bool,
    /// By default, after a Runnable fails, no further Runnable are executed. This
    /// flag indicates that this Runnable must be run even if the Task has already
    /// failed. This is useful for Runnables that copy output files off of the VM
    /// or for debugging.
    ///
    /// The always_run flag does not override the Task's overall max_run_duration.
    /// If the max_run_duration has expired then no further Runnables will execute,
    /// not even always_run Runnables.
    #[prost(bool, tag = "5")]
    pub always_run: bool,
    /// Environment variables for this Runnable (overrides variables set for the
    /// whole Task or TaskGroup).
    #[prost(message, optional, tag = "7")]
    pub environment: ::core::option::Option<Environment>,
    /// Timeout for this Runnable.
    #[prost(message, optional, tag = "8")]
    pub timeout: ::core::option::Option<::prost_types::Duration>,
    /// Labels for this Runnable.
    #[prost(map = "string, string", tag = "9")]
    pub labels: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// The script or container to run.
    #[prost(oneof = "runnable::Executable", tags = "1, 2, 6")]
    pub executable: ::core::option::Option<runnable::Executable>,
}
/// Nested message and enum types in `Runnable`.
pub mod runnable {
    /// Container runnable.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Container {
        /// The URI to pull the container image from.
        #[prost(string, tag = "1")]
        pub image_uri: ::prost::alloc::string::String,
        /// Overrides the `CMD` specified in the container. If there is an ENTRYPOINT
        /// (either in the container image or with the entrypoint field below) then
        /// commands are appended as arguments to the ENTRYPOINT.
        #[prost(string, repeated, tag = "2")]
        pub commands: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// Overrides the `ENTRYPOINT` specified in the container.
        #[prost(string, tag = "3")]
        pub entrypoint: ::prost::alloc::string::String,
        /// Volumes to mount (bind mount) from the host machine files or directories
        /// into the container, formatted to match docker run's --volume option,
        /// e.g. /foo:/bar, or /foo:/bar:ro
        #[prost(string, repeated, tag = "7")]
        pub volumes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// Arbitrary additional options to include in the "docker run" command when
        /// running this container, e.g. "--network host".
        #[prost(string, tag = "8")]
        pub options: ::prost::alloc::string::String,
        /// If set to true, external network access to and from container will be
        /// blocked, containers that are with block_external_network as true can
        /// still communicate with each other, network cannot be specified in the
        /// `container.options` field.
        #[prost(bool, tag = "9")]
        pub block_external_network: bool,
        /// Optional username for logging in to a docker registry. If username
        /// matches `projects/*/secrets/*/versions/*` then Batch will read the
        /// username from the Secret Manager.
        #[prost(string, tag = "10")]
        pub username: ::prost::alloc::string::String,
        /// Optional password for logging in to a docker registry. If password
        /// matches `projects/*/secrets/*/versions/*` then Batch will read the
        /// password from the Secret Manager;
        #[prost(string, tag = "11")]
        pub password: ::prost::alloc::string::String,
    }
    /// Script runnable.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Script {
        #[prost(oneof = "script::Command", tags = "1, 2")]
        pub command: ::core::option::Option<script::Command>,
    }
    /// Nested message and enum types in `Script`.
    pub mod script {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Command {
            /// Script file path on the host VM.
            ///
            /// To specify an interpreter, please add a `#!<interpreter>`(also known as
            /// [shebang line](<https://en.wikipedia.org/wiki/Shebang_(Unix>))) as the
            /// first line of the file.(For example, to execute the script using bash,
            /// `#!/bin/bash` should be the first line of the file. To execute the
            /// script using`Python3`, `#!/usr/bin/env python3` should be the first
            /// line of the file.) Otherwise, the file will by default be excuted by
            /// `/bin/sh`.
            #[prost(string, tag = "1")]
            Path(::prost::alloc::string::String),
            /// Shell script text.
            ///
            /// To specify an interpreter, please add a `#!<interpreter>\n` at the
            /// beginning of the text.(For example, to execute the script using bash,
            /// `#!/bin/bash\n` should be added. To execute the script using`Python3`,
            /// `#!/usr/bin/env python3\n` should be added.) Otherwise, the script will
            /// by default be excuted by `/bin/sh`.
            #[prost(string, tag = "2")]
            Text(::prost::alloc::string::String),
        }
    }
    /// Barrier runnable blocks until all tasks in a taskgroup reach it.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Barrier {
        /// Barriers are identified by their index in runnable list.
        /// Names are not required, but if present should be an identifier.
        #[prost(string, tag = "1")]
        pub name: ::prost::alloc::string::String,
    }
    /// The script or container to run.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Executable {
        /// Container runnable.
        #[prost(message, tag = "1")]
        Container(Container),
        /// Script runnable.
        #[prost(message, tag = "2")]
        Script(Script),
        /// Barrier runnable.
        #[prost(message, tag = "6")]
        Barrier(Barrier),
    }
}
/// Spec of a task
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TaskSpec {
    /// The sequence of scripts or containers to run for this Task. Each Task using
    /// this TaskSpec executes its list of runnables in order. The Task succeeds if
    /// all of its runnables either exit with a zero status or any that exit with a
    /// non-zero status have the ignore_exit_status flag.
    ///
    /// Background runnables are killed automatically (if they have not already
    /// exited) a short time after all foreground runnables have completed. Even
    /// though this is likely to result in a non-zero exit status for the
    /// background runnable, these automatic kills are not treated as Task
    /// failures.
    #[prost(message, repeated, tag = "8")]
    pub runnables: ::prost::alloc::vec::Vec<Runnable>,
    /// ComputeResource requirements.
    #[prost(message, optional, tag = "3")]
    pub compute_resource: ::core::option::Option<ComputeResource>,
    /// Maximum duration the task should run.
    /// The task will be killed and marked as FAILED if over this limit.
    #[prost(message, optional, tag = "4")]
    pub max_run_duration: ::core::option::Option<::prost_types::Duration>,
    /// Maximum number of retries on failures.
    /// The default, 0, which means never retry.
    /// The valid value range is [0, 10].
    #[prost(int32, tag = "5")]
    pub max_retry_count: i32,
    /// Lifecycle management schema when any task in a task group is failed.
    /// Currently we only support one lifecycle policy.
    /// When the lifecycle policy condition is met,
    /// the action in the policy will execute.
    /// If task execution result does not meet with the defined lifecycle
    /// policy, we consider it as the default policy.
    /// Default policy means if the exit code is 0, exit task.
    /// If task ends with non-zero exit code, retry the task with max_retry_count.
    #[prost(message, repeated, tag = "9")]
    pub lifecycle_policies: ::prost::alloc::vec::Vec<LifecyclePolicy>,
    /// Deprecated: please use environment(non-plural) instead.
    #[prost(map = "string, string", tag = "6")]
    pub environments: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Volumes to mount before running Tasks using this TaskSpec.
    #[prost(message, repeated, tag = "7")]
    pub volumes: ::prost::alloc::vec::Vec<Volume>,
    /// Environment variables to set before running the Task.
    #[prost(message, optional, tag = "10")]
    pub environment: ::core::option::Option<Environment>,
}
/// LifecyclePolicy describes how to deal with task failures
/// based on different conditions.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LifecyclePolicy {
    /// Action to execute when ActionCondition is true.
    /// When RETRY_TASK is specified, we will retry failed tasks
    /// if we notice any exit code match and fail tasks if no match is found.
    /// Likewise, when FAIL_TASK is specified, we will fail tasks
    /// if we notice any exit code match and retry tasks if no match is found.
    #[prost(enumeration = "lifecycle_policy::Action", tag = "1")]
    pub action: i32,
    /// Conditions that decide why a task failure is dealt with a specific action.
    #[prost(message, optional, tag = "2")]
    pub action_condition: ::core::option::Option<lifecycle_policy::ActionCondition>,
}
/// Nested message and enum types in `LifecyclePolicy`.
pub mod lifecycle_policy {
    /// Conditions for actions to deal with task failures.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ActionCondition {
        /// Exit codes of a task execution.
        /// If there are more than 1 exit codes,
        /// when task executes with any of the exit code in the list,
        /// the condition is met and the action will be executed.
        #[prost(int32, repeated, tag = "1")]
        pub exit_codes: ::prost::alloc::vec::Vec<i32>,
    }
    /// Action on task failures based on different conditions.
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum Action {
        /// Action unspecified.
        Unspecified = 0,
        /// Action that tasks in the group will be scheduled to re-execute.
        RetryTask = 1,
        /// Action that tasks in the group will be stopped immediately.
        FailTask = 2,
    }
    impl Action {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Action::Unspecified => "ACTION_UNSPECIFIED",
                Action::RetryTask => "RETRY_TASK",
                Action::FailTask => "FAIL_TASK",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "ACTION_UNSPECIFIED" => Some(Self::Unspecified),
                "RETRY_TASK" => Some(Self::RetryTask),
                "FAIL_TASK" => Some(Self::FailTask),
                _ => None,
            }
        }
    }
}
/// A Cloud Batch task.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Task {
    /// Task name.
    /// The name is generated from the parent TaskGroup name and 'id' field.
    /// For example:
    /// "projects/123456/locations/us-west1/jobs/job01/taskGroups/group01/tasks/task01".
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Task Status.
    #[prost(message, optional, tag = "2")]
    pub status: ::core::option::Option<TaskStatus>,
}
/// An Environment describes a collection of environment variables to set when
/// executing Tasks.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Environment {
    /// A map of environment variable names to values.
    #[prost(map = "string, string", tag = "1")]
    pub variables: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// A map of environment variable names to Secret Manager secret names.
    /// The VM will access the named secrets to set the value of each environment
    /// variable.
    #[prost(map = "string, string", tag = "2")]
    pub secret_variables: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// An encrypted JSON dictionary where the key/value pairs correspond to
    /// environment variable names and their values.
    #[prost(message, optional, tag = "3")]
    pub encrypted_variables: ::core::option::Option<environment::KmsEnvMap>,
}
/// Nested message and enum types in `Environment`.
pub mod environment {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct KmsEnvMap {
        /// The name of the KMS key that will be used to decrypt the cipher text.
        #[prost(string, tag = "1")]
        pub key_name: ::prost::alloc::string::String,
        /// The value of the cipherText response from the `encrypt` method.
        #[prost(string, tag = "2")]
        pub cipher_text: ::prost::alloc::string::String,
    }
}
/// The Cloud Batch Job description.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Job {
    /// Output only. Job name.
    /// For example: "projects/123456/locations/us-central1/jobs/job01".
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. A system generated unique ID (in UUID4 format) for the Job.
    #[prost(string, tag = "2")]
    pub uid: ::prost::alloc::string::String,
    /// Priority of the Job.
    /// The valid value range is [0, 100). Default value is 0.
    /// Higher value indicates higher priority.
    /// A job with higher priority value is more likely to run earlier if all other
    /// requirements are satisfied.
    #[prost(int64, tag = "3")]
    pub priority: i64,
    /// Required. TaskGroups in the Job. Only one TaskGroup is supported now.
    #[prost(message, repeated, tag = "4")]
    pub task_groups: ::prost::alloc::vec::Vec<TaskGroup>,
    /// Compute resource allocation for all TaskGroups in the Job.
    #[prost(message, optional, tag = "7")]
    pub allocation_policy: ::core::option::Option<AllocationPolicy>,
    /// Labels for the Job. Labels could be user provided or system generated.
    /// For example,
    /// "labels": {
    ///     "department": "finance",
    ///     "environment": "test"
    ///   }
    /// You can assign up to 64 labels.  [Google Compute Engine label
    /// restrictions](<https://cloud.google.com/compute/docs/labeling-resources#restrictions>)
    /// apply.
    /// Label names that start with "goog-" or "google-" are reserved.
    #[prost(map = "string, string", tag = "8")]
    pub labels: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Output only. Job status. It is read only for users.
    #[prost(message, optional, tag = "9")]
    pub status: ::core::option::Option<JobStatus>,
    /// Output only. When the Job was created.
    #[prost(message, optional, tag = "11")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The last time the Job was updated.
    #[prost(message, optional, tag = "12")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Log preservation policy for the Job.
    #[prost(message, optional, tag = "13")]
    pub logs_policy: ::core::option::Option<LogsPolicy>,
    /// Notification configurations.
    #[prost(message, repeated, tag = "14")]
    pub notifications: ::prost::alloc::vec::Vec<JobNotification>,
}
/// LogsPolicy describes how outputs from a Job's Tasks (stdout/stderr) will be
/// preserved.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LogsPolicy {
    /// Where logs should be saved.
    #[prost(enumeration = "logs_policy::Destination", tag = "1")]
    pub destination: i32,
    /// The path to which logs are saved when the destination = PATH. This can be a
    /// local file path on the VM, or under the mount point of a Persistent Disk or
    /// Filestore, or a Cloud Storage path.
    #[prost(string, tag = "2")]
    pub logs_path: ::prost::alloc::string::String,
}
/// Nested message and enum types in `LogsPolicy`.
pub mod logs_policy {
    /// The destination (if any) for logs.
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum Destination {
        /// Logs are not preserved.
        Unspecified = 0,
        /// Logs are streamed to Cloud Logging.
        CloudLogging = 1,
        /// Logs are saved to a file path.
        Path = 2,
    }
    impl Destination {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Destination::Unspecified => "DESTINATION_UNSPECIFIED",
                Destination::CloudLogging => "CLOUD_LOGGING",
                Destination::Path => "PATH",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "DESTINATION_UNSPECIFIED" => Some(Self::Unspecified),
                "CLOUD_LOGGING" => Some(Self::CloudLogging),
                "PATH" => Some(Self::Path),
                _ => None,
            }
        }
    }
}
/// Job status.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct JobStatus {
    /// Job state
    #[prost(enumeration = "job_status::State", tag = "1")]
    pub state: i32,
    /// Job status events
    #[prost(message, repeated, tag = "2")]
    pub status_events: ::prost::alloc::vec::Vec<StatusEvent>,
    /// Aggregated task status for each TaskGroup in the Job.
    /// The map key is TaskGroup ID.
    #[prost(map = "string, message", tag = "4")]
    pub task_groups: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        job_status::TaskGroupStatus,
    >,
    /// The duration of time that the Job spent in status RUNNING.
    #[prost(message, optional, tag = "5")]
    pub run_duration: ::core::option::Option<::prost_types::Duration>,
}
/// Nested message and enum types in `JobStatus`.
pub mod job_status {
    /// VM instance status.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct InstanceStatus {
        /// The Compute Engine machine type.
        #[prost(string, tag = "1")]
        pub machine_type: ::prost::alloc::string::String,
        /// The VM instance provisioning model.
        #[prost(enumeration = "super::allocation_policy::ProvisioningModel", tag = "2")]
        pub provisioning_model: i32,
        /// The max number of tasks can be assigned to this instance type.
        #[prost(int64, tag = "3")]
        pub task_pack: i64,
        /// The VM boot disk.
        #[prost(message, optional, tag = "4")]
        pub boot_disk: ::core::option::Option<super::allocation_policy::Disk>,
    }
    /// Aggregated task status for a TaskGroup.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TaskGroupStatus {
        /// Count of task in each state in the TaskGroup.
        /// The map key is task state name.
        #[prost(map = "string, int64", tag = "1")]
        pub counts: ::std::collections::HashMap<::prost::alloc::string::String, i64>,
        /// Status of instances allocated for the TaskGroup.
        #[prost(message, repeated, tag = "2")]
        pub instances: ::prost::alloc::vec::Vec<InstanceStatus>,
    }
    /// Valid Job states.
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum State {
        Unspecified = 0,
        /// Job is admitted (validated and persisted) and waiting for resources.
        Queued = 1,
        /// Job is scheduled to run as soon as resource allocation is ready.
        /// The resource allocation may happen at a later time but with a high
        /// chance to succeed.
        Scheduled = 2,
        /// Resource allocation has been successful. At least one Task in the Job is
        /// RUNNING.
        Running = 3,
        /// All Tasks in the Job have finished successfully.
        Succeeded = 4,
        /// At least one Task in the Job has failed.
        Failed = 5,
        /// The Job will be deleted, but has not been deleted yet. Typically this is
        /// because resources used by the Job are still being cleaned up.
        DeletionInProgress = 6,
    }
    impl State {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                State::Unspecified => "STATE_UNSPECIFIED",
                State::Queued => "QUEUED",
                State::Scheduled => "SCHEDULED",
                State::Running => "RUNNING",
                State::Succeeded => "SUCCEEDED",
                State::Failed => "FAILED",
                State::DeletionInProgress => "DELETION_IN_PROGRESS",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "QUEUED" => Some(Self::Queued),
                "SCHEDULED" => Some(Self::Scheduled),
                "RUNNING" => Some(Self::Running),
                "SUCCEEDED" => Some(Self::Succeeded),
                "FAILED" => Some(Self::Failed),
                "DELETION_IN_PROGRESS" => Some(Self::DeletionInProgress),
                _ => None,
            }
        }
    }
}
/// Notification configurations.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct JobNotification {
    /// The Pub/Sub topic where notifications like the job state changes
    /// will be published. This topic exist in the same project as the job
    /// and billings will be charged to this project.
    /// If not specified, no Pub/Sub messages will be sent.
    /// Topic format: `projects/{project}/topics/{topic}`.
    #[prost(string, tag = "1")]
    pub pubsub_topic: ::prost::alloc::string::String,
    /// The attribute requirements of messages to be sent to this Pub/Sub topic.
    /// Without this field, no message will be sent.
    #[prost(message, optional, tag = "2")]
    pub message: ::core::option::Option<job_notification::Message>,
}
/// Nested message and enum types in `JobNotification`.
pub mod job_notification {
    /// Message details.
    /// Describe the attribute that a message should have.
    /// Without specified message attributes, no message will be sent by default.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Message {
        /// The message type.
        #[prost(enumeration = "Type", tag = "1")]
        pub r#type: i32,
        /// The new job state.
        #[prost(enumeration = "super::job_status::State", tag = "2")]
        pub new_job_state: i32,
        /// The new task state.
        #[prost(enumeration = "super::task_status::State", tag = "3")]
        pub new_task_state: i32,
    }
    /// The message type.
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum Type {
        /// Unspecified.
        Unspecified = 0,
        /// Notify users that the job state has changed.
        JobStateChanged = 1,
        /// Notify users that the task state has changed.
        TaskStateChanged = 2,
    }
    impl Type {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Type::Unspecified => "TYPE_UNSPECIFIED",
                Type::JobStateChanged => "JOB_STATE_CHANGED",
                Type::TaskStateChanged => "TASK_STATE_CHANGED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "JOB_STATE_CHANGED" => Some(Self::JobStateChanged),
                "TASK_STATE_CHANGED" => Some(Self::TaskStateChanged),
                _ => None,
            }
        }
    }
}
/// A Job's resource allocation policy describes when, where, and how compute
/// resources should be allocated for the Job.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AllocationPolicy {
    /// Location where compute resources should be allocated for the Job.
    #[prost(message, optional, tag = "1")]
    pub location: ::core::option::Option<allocation_policy::LocationPolicy>,
    /// Describe instances that can be created by this AllocationPolicy.
    /// Only instances\[0\] is supported now.
    #[prost(message, repeated, tag = "8")]
    pub instances: ::prost::alloc::vec::Vec<allocation_policy::InstancePolicyOrTemplate>,
    /// Service account that VMs will run as.
    #[prost(message, optional, tag = "9")]
    pub service_account: ::core::option::Option<ServiceAccount>,
    /// Labels applied to all VM instances and other resources
    /// created by AllocationPolicy.
    /// Labels could be user provided or system generated.
    /// You can assign up to 64 labels. [Google Compute Engine label
    /// restrictions](<https://cloud.google.com/compute/docs/labeling-resources#restrictions>)
    /// apply.
    /// Label names that start with "goog-" or "google-" are reserved.
    #[prost(map = "string, string", tag = "6")]
    pub labels: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// The network policy.
    #[prost(message, optional, tag = "7")]
    pub network: ::core::option::Option<allocation_policy::NetworkPolicy>,
    /// The placement policy.
    #[prost(message, optional, tag = "10")]
    pub placement: ::core::option::Option<allocation_policy::PlacementPolicy>,
}
/// Nested message and enum types in `AllocationPolicy`.
pub mod allocation_policy {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct LocationPolicy {
        /// A list of allowed location names represented by internal URLs.
        ///
        /// Each location can be a region or a zone.
        /// Only one region or multiple zones in one region is supported now.
        /// For example,
        /// \["regions/us-central1"\] allow VMs in any zones in region us-central1.
        /// ["zones/us-central1-a", "zones/us-central1-c"] only allow VMs
        /// in zones us-central1-a and us-central1-c.
        ///
        /// All locations end up in different regions would cause errors.
        /// For example,
        /// ["regions/us-central1", "zones/us-central1-a", "zones/us-central1-b",
        /// "zones/us-west1-a"] contains 2 regions "us-central1" and
        /// "us-west1". An error is expected in this case.
        #[prost(string, repeated, tag = "1")]
        pub allowed_locations: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
    /// A new persistent disk or a local ssd.
    /// A VM can only have one local SSD setting but multiple local SSD partitions.
    /// See <https://cloud.google.com/compute/docs/disks#pdspecs> and
    /// <https://cloud.google.com/compute/docs/disks#localssds.>
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Disk {
        /// Disk type as shown in `gcloud compute disk-types list`.
        /// For example, local SSD uses type "local-ssd".
        /// Persistent disks and boot disks use "pd-balanced", "pd-extreme", "pd-ssd"
        /// or "pd-standard".
        #[prost(string, tag = "1")]
        pub r#type: ::prost::alloc::string::String,
        /// Disk size in GB.
        ///
        /// For persistent disk, this field is ignored if `data_source` is `image` or
        /// `snapshot`.
        /// For local SSD, size_gb should be a multiple of 375GB,
        /// otherwise, the final size will be the next greater multiple of 375 GB.
        /// For boot disk, Batch will calculate the boot disk size based on source
        /// image and task requirements if you do not speicify the size.
        /// If both this field and the boot_disk_mib field in task spec's
        /// compute_resource are defined, Batch will only honor this field.
        #[prost(int64, tag = "2")]
        pub size_gb: i64,
        /// Local SSDs are available through both "SCSI" and "NVMe" interfaces.
        /// If not indicated, "NVMe" will be the default one for local ssds.
        /// We only support "SCSI" for persistent disks now.
        #[prost(string, tag = "6")]
        pub disk_interface: ::prost::alloc::string::String,
        /// A data source from which a PD will be created.
        #[prost(oneof = "disk::DataSource", tags = "4, 5")]
        pub data_source: ::core::option::Option<disk::DataSource>,
    }
    /// Nested message and enum types in `Disk`.
    pub mod disk {
        /// A data source from which a PD will be created.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum DataSource {
            /// Name of a public or custom image used as the data source.
            /// For example, the following are all valid URLs:
            ///
            /// * Specify the image by its family name:
            /// projects/{project}/global/images/family/{image_family}
            /// * Specify the image version:
            /// projects/{project}/global/images/{image_version}
            ///
            /// You can also use Batch customized image in short names.
            /// The following image values are supported for a boot disk:
            ///
            /// * "batch-debian": use Batch Debian images.
            /// * "batch-centos": use Batch CentOS images.
            /// * "batch-cos": use Batch Container-Optimized images.
            #[prost(string, tag = "4")]
            Image(::prost::alloc::string::String),
            /// Name of a snapshot used as the data source.
            /// Snapshot is not supported as boot disk now.
            #[prost(string, tag = "5")]
            Snapshot(::prost::alloc::string::String),
        }
    }
    /// A new or an existing persistent disk (PD) or a local ssd attached to a VM
    /// instance.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct AttachedDisk {
        /// Device name that the guest operating system will see.
        /// It is used by Runnable.volumes field to mount disks. So please specify
        /// the device_name if you want Batch to help mount the disk, and it should
        /// match the device_name field in volumes.
        #[prost(string, tag = "3")]
        pub device_name: ::prost::alloc::string::String,
        #[prost(oneof = "attached_disk::Attached", tags = "1, 2")]
        pub attached: ::core::option::Option<attached_disk::Attached>,
    }
    /// Nested message and enum types in `AttachedDisk`.
    pub mod attached_disk {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Attached {
            #[prost(message, tag = "1")]
            NewDisk(super::Disk),
            /// Name of an existing PD.
            #[prost(string, tag = "2")]
            ExistingDisk(::prost::alloc::string::String),
        }
    }
    /// Accelerator describes Compute Engine accelerators to be attached to the VM.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Accelerator {
        /// The accelerator type. For example, "nvidia-tesla-t4".
        /// See `gcloud compute accelerator-types list`.
        #[prost(string, tag = "1")]
        pub r#type: ::prost::alloc::string::String,
        /// The number of accelerators of this type.
        #[prost(int64, tag = "2")]
        pub count: i64,
        /// Deprecated: please use instances\[0\].install_gpu_drivers instead.
        #[deprecated]
        #[prost(bool, tag = "3")]
        pub install_gpu_drivers: bool,
    }
    /// InstancePolicy describes an instance type and resources attached to each VM
    /// created by this InstancePolicy.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct InstancePolicy {
        /// The Compute Engine machine type.
        #[prost(string, tag = "2")]
        pub machine_type: ::prost::alloc::string::String,
        /// The minimum CPU platform.
        /// See
        /// <https://cloud.google.com/compute/docs/instances/specify-min-cpu-platform.>
        /// Not yet implemented.
        #[prost(string, tag = "3")]
        pub min_cpu_platform: ::prost::alloc::string::String,
        /// The provisioning model.
        #[prost(enumeration = "ProvisioningModel", tag = "4")]
        pub provisioning_model: i32,
        /// The accelerators attached to each VM instance.
        #[prost(message, repeated, tag = "5")]
        pub accelerators: ::prost::alloc::vec::Vec<Accelerator>,
        /// Boot disk to be created and attached to each VM by this InstancePolicy.
        /// Boot disk will be deleted when the VM is deleted.
        /// Batch API now only supports booting from image.
        #[prost(message, optional, tag = "8")]
        pub boot_disk: ::core::option::Option<Disk>,
        /// Non-boot disks to be attached for each VM created by this InstancePolicy.
        /// New disks will be deleted when the VM is deleted.
        #[prost(message, repeated, tag = "6")]
        pub disks: ::prost::alloc::vec::Vec<AttachedDisk>,
    }
    /// Either an InstancePolicy or an instance template.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct InstancePolicyOrTemplate {
        /// Set this field true if users want Batch to help fetch drivers from a
        /// third party location and install them for GPUs specified in
        /// policy.accelerators or instance_template on their behalf. Default is
        /// false.
        #[prost(bool, tag = "3")]
        pub install_gpu_drivers: bool,
        #[prost(oneof = "instance_policy_or_template::PolicyTemplate", tags = "1, 2")]
        pub policy_template: ::core::option::Option<
            instance_policy_or_template::PolicyTemplate,
        >,
    }
    /// Nested message and enum types in `InstancePolicyOrTemplate`.
    pub mod instance_policy_or_template {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum PolicyTemplate {
            /// InstancePolicy.
            #[prost(message, tag = "1")]
            Policy(super::InstancePolicy),
            /// Name of an instance template used to create VMs.
            /// Named the field as 'instance_template' instead of 'template' to avoid
            /// c++ keyword conflict.
            #[prost(string, tag = "2")]
            InstanceTemplate(::prost::alloc::string::String),
        }
    }
    /// A network interface.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct NetworkInterface {
        /// The URL of an existing network resource.
        /// You can specify the network as a full or partial URL.
        ///
        /// For example, the following are all valid URLs:
        ///
        /// * <https://www.googleapis.com/compute/v1/projects/{project}/global/networks/{network}>
        /// * projects/{project}/global/networks/{network}
        /// * global/networks/{network}
        #[prost(string, tag = "1")]
        pub network: ::prost::alloc::string::String,
        /// The URL of an existing subnetwork resource in the network.
        /// You can specify the subnetwork as a full or partial URL.
        ///
        /// For example, the following are all valid URLs:
        ///
        /// * <https://www.googleapis.com/compute/v1/projects/{project}/regions/{region}/subnetworks/{subnetwork}>
        /// * projects/{project}/regions/{region}/subnetworks/{subnetwork}
        /// * regions/{region}/subnetworks/{subnetwork}
        #[prost(string, tag = "2")]
        pub subnetwork: ::prost::alloc::string::String,
        /// Default is false (with an external IP address). Required if
        /// no external public IP address is attached to the VM. If no external
        /// public IP address, additional configuration is required to allow the VM
        /// to access Google Services. See
        /// <https://cloud.google.com/vpc/docs/configure-private-google-access> and
        /// <https://cloud.google.com/nat/docs/gce-example#create-nat> for more
        /// information.
        #[prost(bool, tag = "3")]
        pub no_external_ip_address: bool,
    }
    /// NetworkPolicy describes VM instance network configurations.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct NetworkPolicy {
        /// Network configurations.
        #[prost(message, repeated, tag = "1")]
        pub network_interfaces: ::prost::alloc::vec::Vec<NetworkInterface>,
    }
    /// PlacementPolicy describes a group placement policy for the VMs controlled
    /// by this AllocationPolicy.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PlacementPolicy {
        /// UNSPECIFIED vs. COLLOCATED (default UNSPECIFIED). Use COLLOCATED when you
        /// want VMs to be located close to each other for low network latency
        /// between the VMs. No placement policy will be generated when collocation
        /// is UNSPECIFIED.
        #[prost(string, tag = "1")]
        pub collocation: ::prost::alloc::string::String,
        /// When specified, causes the job to fail if more than max_distance logical
        /// switches are required between VMs. Batch uses the most compact possible
        /// placement of VMs even when max_distance is not specified. An explicit
        /// max_distance makes that level of compactness a strict requirement.
        /// Not yet implemented
        #[prost(int64, tag = "2")]
        pub max_distance: i64,
    }
    /// Compute Engine VM instance provisioning model.
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum ProvisioningModel {
        /// Unspecified.
        Unspecified = 0,
        /// Standard VM.
        Standard = 1,
        /// SPOT VM.
        Spot = 2,
        /// Preemptible VM (PVM).
        ///
        /// Above SPOT VM is the preferable model for preemptible VM instances: the
        /// old preemptible VM model (indicated by this field) is the older model,
        /// and has been migrated to use the SPOT model as the underlying technology.
        /// This old model will still be supported.
        Preemptible = 3,
    }
    impl ProvisioningModel {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ProvisioningModel::Unspecified => "PROVISIONING_MODEL_UNSPECIFIED",
                ProvisioningModel::Standard => "STANDARD",
                ProvisioningModel::Spot => "SPOT",
                ProvisioningModel::Preemptible => "PREEMPTIBLE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "PROVISIONING_MODEL_UNSPECIFIED" => Some(Self::Unspecified),
                "STANDARD" => Some(Self::Standard),
                "SPOT" => Some(Self::Spot),
                "PREEMPTIBLE" => Some(Self::Preemptible),
                _ => None,
            }
        }
    }
}
/// A TaskGroup contains one or multiple Tasks that share the same
/// Runnable but with different runtime parameters.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TaskGroup {
    /// Output only. TaskGroup name.
    /// The system generates this field based on parent Job name.
    /// For example:
    /// "projects/123456/locations/us-west1/jobs/job01/taskGroups/group01".
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. Tasks in the group share the same task spec.
    #[prost(message, optional, tag = "3")]
    pub task_spec: ::core::option::Option<TaskSpec>,
    /// Number of Tasks in the TaskGroup.
    /// Default is 1.
    #[prost(int64, tag = "4")]
    pub task_count: i64,
    /// Max number of tasks that can run in parallel.
    /// Default to min(task_count, 1000).
    /// Field parallelism must be 1 if the scheduling_policy is IN_ORDER.
    #[prost(int64, tag = "5")]
    pub parallelism: i64,
    /// An array of environment variable mappings, which are passed to Tasks with
    /// matching indices. If task_environments is used then task_count should
    /// not be specified in the request (and will be ignored). Task count will be
    /// the length of task_environments.
    ///
    /// Tasks get a BATCH_TASK_INDEX and BATCH_TASK_COUNT environment variable, in
    /// addition to any environment variables set in task_environments, specifying
    /// the number of Tasks in the Task's parent TaskGroup, and the specific Task's
    /// index in the TaskGroup (0 through BATCH_TASK_COUNT - 1).
    ///
    /// task_environments supports up to 200 entries.
    #[prost(message, repeated, tag = "9")]
    pub task_environments: ::prost::alloc::vec::Vec<Environment>,
    /// Max number of tasks that can be run on a VM at the same time.
    /// If not specified, the system will decide a value based on available
    /// compute resources on a VM and task requirements.
    #[prost(int64, tag = "10")]
    pub task_count_per_node: i64,
    /// When true, Batch will populate a file with a list of all VMs assigned to
    /// the TaskGroup and set the BATCH_HOSTS_FILE environment variable to the path
    /// of that file. Defaults to false.
    #[prost(bool, tag = "11")]
    pub require_hosts_file: bool,
    /// When true, Batch will configure SSH to allow passwordless login between
    /// VMs running the Batch tasks in the same TaskGroup.
    #[prost(bool, tag = "12")]
    pub permissive_ssh: bool,
}
/// Carries information about a Google Cloud service account.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServiceAccount {
    /// Email address of the service account. If not specified, the default
    /// Compute Engine service account for the project will be used. If instance
    /// template is being used, the service account has to be specified in the
    /// instance template and it has to match the email field here.
    #[prost(string, tag = "1")]
    pub email: ::prost::alloc::string::String,
    /// List of scopes to be enabled for this service account on the VM, in
    /// addition to the cloud-platform API scope that will be added by default.
    #[prost(string, repeated, tag = "2")]
    pub scopes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// CreateJob Request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateJobRequest {
    /// Required. The parent resource name where the Job will be created.
    /// Pattern: "projects/{project}/locations/{location}"
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// ID used to uniquely identify the Job within its parent scope.
    /// This field should contain at most 63 characters and must start with
    /// lowercase characters.
    /// Only lowercase characters, numbers and '-' are accepted.
    /// The '-' character cannot be the first or the last one.
    /// A system generated ID will be used if the field is not set.
    ///
    /// The job.name field in the request will be ignored and the created resource
    /// name of the Job will be "{parent}/jobs/{job_id}".
    #[prost(string, tag = "2")]
    pub job_id: ::prost::alloc::string::String,
    /// Required. The Job to create.
    #[prost(message, optional, tag = "3")]
    pub job: ::core::option::Option<Job>,
    /// Optional. An optional request ID to identify requests. Specify a unique
    /// request ID so that if you must retry your request, the server will know to
    /// ignore the request if it has already been completed. The server will
    /// guarantee that for at least 60 minutes since the first request.
    ///
    /// For example, consider a situation where you make an initial request and
    /// the request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "4")]
    pub request_id: ::prost::alloc::string::String,
}
/// GetJob Request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetJobRequest {
    /// Required. Job name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// DeleteJob Request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteJobRequest {
    /// Job name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. Reason for this deletion.
    #[prost(string, tag = "2")]
    pub reason: ::prost::alloc::string::String,
    /// Optional. An optional request ID to identify requests. Specify a unique
    /// request ID so that if you must retry your request, the server will know to
    /// ignore the request if it has already been completed. The server will
    /// guarantee that for at least 60 minutes after the first request.
    ///
    /// For example, consider a situation where you make an initial request and
    /// the request times out. If you make the request again with the same request
    /// ID, the server can check if original operation with the same request ID
    /// was received, and if so, will ignore the second request. This prevents
    /// clients from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "4")]
    pub request_id: ::prost::alloc::string::String,
}
/// ListJob Request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListJobsRequest {
    /// Parent path.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// List filter.
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Page size.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// ListJob Response.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListJobsResponse {
    /// Jobs.
    #[prost(message, repeated, tag = "1")]
    pub jobs: ::prost::alloc::vec::Vec<Job>,
    /// Next page token.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// ListTasks Request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTasksRequest {
    /// Required. Name of a TaskGroup from which Tasks are being requested.
    /// Pattern:
    /// "projects/{project}/locations/{location}/jobs/{job}/taskGroups/{task_group}"
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Task filter, null filter matches all Tasks.
    /// Filter string should be of the format State=TaskStatus.State e.g.
    /// State=RUNNING
    #[prost(string, tag = "2")]
    pub filter: ::prost::alloc::string::String,
    /// Page size.
    #[prost(int32, tag = "3")]
    pub page_size: i32,
    /// Page token.
    #[prost(string, tag = "4")]
    pub page_token: ::prost::alloc::string::String,
}
/// ListTasks Response.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTasksResponse {
    /// Tasks.
    #[prost(message, repeated, tag = "1")]
    pub tasks: ::prost::alloc::vec::Vec<Task>,
    /// Next page token.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Request for a single Task by name.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTaskRequest {
    /// Required. Task name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Represents the metadata of the long-running operation.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationMetadata {
    /// Output only. The time the operation was created.
    #[prost(message, optional, tag = "1")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The time the operation finished running.
    #[prost(message, optional, tag = "2")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Server-defined resource path for the target of the operation.
    #[prost(string, tag = "3")]
    pub target: ::prost::alloc::string::String,
    /// Output only. Name of the verb executed by the operation.
    #[prost(string, tag = "4")]
    pub verb: ::prost::alloc::string::String,
    /// Output only. Human-readable status of the operation, if any.
    #[prost(string, tag = "5")]
    pub status_message: ::prost::alloc::string::String,
    /// Output only. Identifies whether the user has requested cancellation
    /// of the operation. Operations that have successfully been cancelled
    /// have \[Operation.error][\] value with a
    /// \[google.rpc.Status.code][google.rpc.Status.code\] of 1, corresponding to
    /// `Code.CANCELLED`.
    #[prost(bool, tag = "6")]
    pub requested_cancellation: bool,
    /// Output only. API version used to start the operation.
    #[prost(string, tag = "7")]
    pub api_version: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod batch_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Google Batch Service.
    /// The service manages user submitted batch jobs and allocates Google Compute
    /// Engine VM instances to run the jobs.
    #[derive(Debug, Clone)]
    pub struct BatchServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl BatchServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> BatchServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> BatchServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            BatchServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        /// Create a Job.
        pub async fn create_job(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateJobRequest>,
        ) -> std::result::Result<tonic::Response<super::Job>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.batch.v1.BatchService/CreateJob",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.batch.v1.BatchService", "CreateJob"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Get a Job specified by its resource name.
        pub async fn get_job(
            &mut self,
            request: impl tonic::IntoRequest<super::GetJobRequest>,
        ) -> std::result::Result<tonic::Response<super::Job>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.batch.v1.BatchService/GetJob",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("google.cloud.batch.v1.BatchService", "GetJob"));
            self.inner.unary(req, path, codec).await
        }
        /// Delete a Job.
        pub async fn delete_job(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteJobRequest>,
        ) -> std::result::Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.batch.v1.BatchService/DeleteJob",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.batch.v1.BatchService", "DeleteJob"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// List all Jobs for a project within a region.
        pub async fn list_jobs(
            &mut self,
            request: impl tonic::IntoRequest<super::ListJobsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListJobsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.batch.v1.BatchService/ListJobs",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.batch.v1.BatchService", "ListJobs"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Return a single Task.
        pub async fn get_task(
            &mut self,
            request: impl tonic::IntoRequest<super::GetTaskRequest>,
        ) -> std::result::Result<tonic::Response<super::Task>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.batch.v1.BatchService/GetTask",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.batch.v1.BatchService", "GetTask"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// List Tasks associated with a job.
        pub async fn list_tasks(
            &mut self,
            request: impl tonic::IntoRequest<super::ListTasksRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListTasksResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.batch.v1.BatchService/ListTasks",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("google.cloud.batch.v1.BatchService", "ListTasks"),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
