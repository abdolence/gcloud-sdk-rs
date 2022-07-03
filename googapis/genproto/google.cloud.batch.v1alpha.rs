/// Volume and mount parameters to be associated with a TaskSpec. A TaskSpec
/// might describe zero, one, or multiple volumes to be mounted as part of the
/// task.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Volume {
    /// Mount path for the volume, e.g. /mnt/share
    #[prost(string, tag = "4")]
    pub mount_path: ::prost::alloc::string::String,
    /// Mount options
    /// For Google Cloud Storage, mount options are the global options supported by
    /// gcsfuse tool. Batch will use them to mount the volume with the following
    /// command:
    /// "gcsfuse [global options] bucket mountpoint".
    /// For PD, NFS, mount options are these supported by /etc/fstab. Batch will
    /// use Fstab to mount such volumes.
    /// <https://help.ubuntu.com/community/Fstab>
    #[prost(string, repeated, tag = "5")]
    pub mount_options: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The source for the volume.
    #[prost(oneof = "volume::Source", tags = "1, 2, 3, 6")]
    pub source: ::core::option::Option<volume::Source>,
}
/// Nested message and enum types in `Volume`.
pub mod volume {
    /// The source for the volume.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Source {
        /// An NFS source for the volume (could be a Filestore, for example).
        #[prost(message, tag = "1")]
        Nfs(super::Nfs),
        /// A persistent disk source for the volume.
        #[prost(message, tag = "2")]
        Pd(super::Pd),
        /// A Google Cloud Storage source for the volume.
        #[prost(message, tag = "3")]
        Gcs(super::Gcs),
        /// Device name of an attached disk
        #[prost(string, tag = "6")]
        DeviceName(::prost::alloc::string::String),
    }
}
/// Represents an NFS server and remote path: <server>:<remote_path>
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Nfs {
    /// URI of the NFS server, e.g. an IP address.
    #[prost(string, tag = "1")]
    pub server: ::prost::alloc::string::String,
    /// Remote source path exported from NFS, e.g., "/share".
    #[prost(string, tag = "2")]
    pub remote_path: ::prost::alloc::string::String,
}
/// Represents a GCP persistent disk
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Pd {
    /// PD disk name, e.g. pd-1.
    #[prost(string, tag = "1")]
    pub disk: ::prost::alloc::string::String,
    /// PD device name, e.g. persistent-disk-1.
    #[prost(string, tag = "2")]
    pub device: ::prost::alloc::string::String,
    /// Whether this is an existing PD. Default is false. If false, i.e., new
    /// PD, we will format it into ext4 and mount to the given path. If true, i.e.,
    /// existing PD, it should be in ext4 format and we will mount it to the given
    /// path.
    #[deprecated]
    #[prost(bool, tag = "3")]
    pub existing: bool,
}
/// Represents a Google Cloud Storage volume source config.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Gcs {
    /// Remote path, either a bucket name or a subdirectory of a bucket, e.g.:
    /// bucket_name, bucket_name/subdirectory/
    #[prost(string, tag = "1")]
    pub remote_path: ::prost::alloc::string::String,
}
/// Compute resource requirements
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ComputeResource {
    /// The milliCPU count.
    #[prost(int64, tag = "1")]
    pub cpu_milli: i64,
    /// Memory in MiB.
    #[prost(int64, tag = "2")]
    pub memory_mib: i64,
    /// The GPU count.
    ///
    /// Not yet implemented.
    #[prost(int64, tag = "3")]
    pub gpu_count: i64,
    /// Extra boot disk size in MiB for each task.
    #[prost(int64, tag = "4")]
    pub boot_disk_mib: i64,
}
/// Status event
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
}
/// This Task Execution field includes detail information for
/// task execution procedures, based on StatusEvent types.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TaskExecution {
    /// When task is completed as the status of FAILED or SUCCEEDED,
    /// exit code is for one task execution result, default is 0 as success.
    #[prost(int32, tag = "1")]
    pub exit_code: i32,
}
/// Status of a task
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
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// unknown state
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
    }
}
/// Runnable describes instructions for executing a specific script or container
/// as part of a Task.
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
    pub labels:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// The script or container to run.
    #[prost(oneof = "runnable::Executable", tags = "1, 2, 6")]
    pub executable: ::core::option::Option<runnable::Executable>,
}
/// Nested message and enum types in `Runnable`.
pub mod runnable {
    /// Container runnable.
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
        /// blocked. The container will use the default internal network
        /// 'goog-internal'.
        #[prost(bool, tag = "9")]
        pub block_external_network: bool,
        /// Optional username for logging in to a docker registry. If username
        /// matches "projects/*/secrets/*/versions/*" then Batch will read the
        /// username from the Secret Manager.
        #[prost(string, tag = "10")]
        pub username: ::prost::alloc::string::String,
        /// Optional password for logging in to a docker registry. If password
        /// matches "projects/*/secrets/*/versions/*" then Batch will read the
        /// password from the Secret Manager;
        #[prost(string, tag = "11")]
        pub password: ::prost::alloc::string::String,
    }
    /// Script runnable.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Script {
        #[prost(oneof = "script::Command", tags = "1, 2")]
        pub command: ::core::option::Option<script::Command>,
    }
    /// Nested message and enum types in `Script`.
    pub mod script {
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Command {
            /// Script file path on the host VM.
            #[prost(string, tag = "1")]
            Path(::prost::alloc::string::String),
            /// Shell script text.
            #[prost(string, tag = "2")]
            Text(::prost::alloc::string::String),
        }
    }
    /// Barrier runnable blocks until all tasks in a taskgroup reach it.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Barrier {
        /// Barriers are identified by their index in runnable list.
        /// Names are not required, but if present should be an identifier.
        #[prost(string, tag = "1")]
        pub name: ::prost::alloc::string::String,
    }
    /// The script or container to run.
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
    /// The valid size of lifecycle policies are [0, 10].
    /// For each lifecycle policy, when the condition is met,
    /// the action in that policy will execute.
    /// If there are multiple policies that the task execution result matches,
    /// we use the action from the first matched policy. If task execution result
    /// does not meet with any of the defined lifecycle policy, we consider it as
    /// the default policy. Default policy means if the exit code is 0, exit task.
    /// If task ends with non-zero exit code, retry the task with max_retry_count.
    #[prost(message, repeated, tag = "9")]
    pub lifecycle_policies: ::prost::alloc::vec::Vec<LifecyclePolicy>,
    /// Environment variables to set before running the Task.
    /// You can set up to 100 environments.
    #[prost(map = "string, string", tag = "6")]
    pub environments:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Volumes to mount before running Tasks using this TaskSpec.
    #[prost(message, repeated, tag = "7")]
    pub volumes: ::prost::alloc::vec::Vec<Volume>,
    /// Environment variables to set before running the Task.
    #[prost(message, optional, tag = "10")]
    pub environment: ::core::option::Option<Environment>,
}
/// LifecyclePolicy describes how to deal with task failures
/// based on different conditions.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LifecyclePolicy {
    /// Action to execute when ActionCondition is true.
    #[prost(enumeration = "lifecycle_policy::Action", tag = "1")]
    pub action: i32,
    /// Conditions that decide why a task failure is dealt with a specific action.
    #[prost(message, optional, tag = "2")]
    pub action_condition: ::core::option::Option<lifecycle_policy::ActionCondition>,
}
/// Nested message and enum types in `LifecyclePolicy`.
pub mod lifecycle_policy {
    /// Conditions for actions to deal with task failures.
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
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Action {
        /// Action unspecified.
        Unspecified = 0,
        /// Action that tasks in the group will be scheduled to re-execute.
        RetryTask = 1,
        /// Action that tasks in the group will be stopped immediately.
        FailTask = 2,
    }
}
/// A Cloud Batch task.
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
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Environment {
    /// A map of environment variable names to values.
    #[prost(map = "string, string", tag = "1")]
    pub variables:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// A map of environment variable names to Secret Manager secret names.
    /// The VM will access the named secrets to set the value of each environment
    /// variable.
    #[prost(map = "string, string", tag = "2")]
    pub secret_variables:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// An encrypted JSON dictionary where the key/value pairs correspond to
    /// environment variable names and their values.
    #[prost(message, optional, tag = "3")]
    pub encrypted_variables: ::core::option::Option<environment::KmsEnvMap>,
}
/// Nested message and enum types in `Environment`.
pub mod environment {
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
    /// The valid value range is [0, 100).
    /// A job with higher priority value is more likely to run earlier if all other
    /// requirements are satisfied.
    #[prost(int64, tag = "3")]
    pub priority: i64,
    /// Required. TaskGroups in the Job. Only one TaskGroup is supported now.
    #[prost(message, repeated, tag = "4")]
    pub task_groups: ::prost::alloc::vec::Vec<TaskGroup>,
    /// Scheduling policy for TaskGroups in the job.
    #[prost(enumeration = "job::SchedulingPolicy", tag = "5")]
    pub scheduling_policy: i32,
    /// At least one of the dependencies must be satisfied before the Job is
    /// scheduled to run.
    /// Only one JobDependency is supported now.
    /// Not yet implemented.
    #[prost(message, repeated, tag = "6")]
    pub dependencies: ::prost::alloc::vec::Vec<JobDependency>,
    /// Compute resource allocation for all TaskGroups in the Job.
    #[prost(message, optional, tag = "7")]
    pub allocation_policy: ::core::option::Option<AllocationPolicy>,
    /// Labels for the Job. Labels could be user provided or system generated.
    /// For example,
    /// "labels": {
    ///    "department": "finance",
    ///    "environment": "test"
    ///  }
    /// You can assign up to 64 labels.  [Google Compute Engine label
    /// restrictions](<https://cloud.google.com/compute/docs/labeling-resources#restrictions>)
    /// apply.
    /// Label names that start with "goog-" or "google-" are reserved.
    #[prost(map = "string, string", tag = "8")]
    pub labels:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Output only. Job status. It is read only for users.
    #[prost(message, optional, tag = "9")]
    pub status: ::core::option::Option<JobStatus>,
    /// Job notification.
    #[deprecated]
    #[prost(message, optional, tag = "10")]
    pub notification: ::core::option::Option<JobNotification>,
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
/// Nested message and enum types in `Job`.
pub mod job {
    /// The order that TaskGroups are scheduled relative to each other.
    ///
    /// Not yet implemented.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum SchedulingPolicy {
        /// Unspecified.
        Unspecified = 0,
        /// Run all TaskGroups as soon as possible.
        AsSoonAsPossible = 1,
    }
}
/// LogsPolicy describes how outputs from a Job's Tasks (stdout/stderr) will be
/// preserved.
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
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Destination {
        /// Logs are not preserved.
        Unspecified = 0,
        /// Logs are streamed to Cloud Logging.
        CloudLogging = 1,
        /// Logs are saved to a file path.
        Path = 2,
    }
}
/// JobDependency describes the state of other Jobs that the start of this Job
/// depends on.
/// All dependent Jobs must have been submitted in the same region.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct JobDependency {
    /// Each item maps a Job name to a Type.
    /// All items must be satisfied for the JobDependency to be satisfied (the AND
    /// operation).
    /// Once a condition for one item becomes true, it won't go back to false
    /// even the dependent Job state changes again.
    #[prost(map = "string, enumeration(job_dependency::Type)", tag = "1")]
    pub items: ::std::collections::HashMap<::prost::alloc::string::String, i32>,
}
/// Nested message and enum types in `JobDependency`.
pub mod job_dependency {
    /// Dependency type.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Type {
        /// Unspecified.
        Unspecified = 0,
        /// The dependent Job has succeeded.
        Succeeded = 1,
        /// The dependent Job has failed.
        Failed = 2,
        /// SUCCEEDED or FAILED.
        Finished = 3,
    }
}
/// Job status.
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
    pub task_groups:
        ::std::collections::HashMap<::prost::alloc::string::String, job_status::TaskGroupStatus>,
    /// The duration of time that the Job spent in status RUNNING.
    #[prost(message, optional, tag = "5")]
    pub run_duration: ::core::option::Option<::prost_types::Duration>,
}
/// Nested message and enum types in `JobStatus`.
pub mod job_status {
    /// VM instance status.
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
    }
    /// Aggregated task status for a TaskGroup.
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
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
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
}
/// Notification configurations.
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
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Type {
        /// Unspecified.
        Unspecified = 0,
        /// Notify users that the job state has changed.
        JobStateChanged = 1,
        /// Notify users that the task state has changed.
        TaskStateChanged = 2,
    }
}
/// A Job's resource allocation policy describes when, where, and how compute
/// resources should be allocated for the Job.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AllocationPolicy {
    /// Location where compute resources should be allocated for the Job.
    #[prost(message, optional, tag = "1")]
    pub location: ::core::option::Option<allocation_policy::LocationPolicy>,
    /// Create only instances allowed by this policy.
    #[deprecated]
    #[prost(message, optional, tag = "2")]
    pub instance: ::core::option::Option<allocation_policy::InstancePolicy>,
    /// Describe instances that can be created by this AllocationPolicy.
    /// Only instances\[0\] is supported now.
    #[prost(message, repeated, tag = "8")]
    pub instances: ::prost::alloc::vec::Vec<allocation_policy::InstancePolicyOrTemplate>,
    /// Instance templates that are used to VMs.
    /// If specified, only instance_templates\[0\] is used.
    #[deprecated]
    #[prost(string, repeated, tag = "3")]
    pub instance_templates: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Create only instances in the listed provisiong models.
    /// Default to allow all.
    ///
    /// Currently only the first model of the provisioning_models list will be
    /// considered; specifying additional models (e.g., 2nd, 3rd, etc.) is a no-op.
    #[deprecated]
    #[prost(
        enumeration = "allocation_policy::ProvisioningModel",
        repeated,
        packed = "false",
        tag = "4"
    )]
    pub provisioning_models: ::prost::alloc::vec::Vec<i32>,
    /// Email of the service account that VMs will run as.
    #[prost(string, tag = "5")]
    pub service_account_email: ::prost::alloc::string::String,
    /// Service account that VMs will run as.
    /// Not yet implemented.
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
    pub labels:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// The network policy.
    #[prost(message, optional, tag = "7")]
    pub network: ::core::option::Option<allocation_policy::NetworkPolicy>,
}
/// Nested message and enum types in `AllocationPolicy`.
pub mod allocation_policy {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct LocationPolicy {
        /// A list of allowed location names represented by internal URLs,
        /// First location in the list must be a region.
        /// for example,
        /// \["regions/us-central1"\] allow VMs in region us-central1,
        /// ["regions/us-central1", "zones/us-central1-a"] only allow VMs in zone
        /// us-central1-a.
        #[prost(string, repeated, tag = "1")]
        pub allowed_locations: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// A list of denied location names.
        ///
        /// Not yet implemented.
        #[prost(string, repeated, tag = "2")]
        pub denied_locations: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
    /// A new persistent disk.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Disk {
        /// Disk type as shown in `gcloud compute disk-types list`
        /// For example, "pd-ssd", "pd-standard", "pd-balanced".
        #[prost(string, tag = "1")]
        pub r#type: ::prost::alloc::string::String,
        /// Disk size in GB.
        /// This field is ignored if `data_source` is `disk` or `image`.
        #[prost(int64, tag = "2")]
        pub size_gb: i64,
        /// A data source from which a PD will be created.
        #[prost(oneof = "disk::DataSource", tags = "4, 5")]
        pub data_source: ::core::option::Option<disk::DataSource>,
    }
    /// Nested message and enum types in `Disk`.
    pub mod disk {
        /// A data source from which a PD will be created.
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum DataSource {
            /// Name of a public or custom image used as the data source.
            #[prost(string, tag = "4")]
            Image(::prost::alloc::string::String),
            /// Name of a snapshot used as the data source.
            #[prost(string, tag = "5")]
            Snapshot(::prost::alloc::string::String),
        }
    }
    /// A new or an existing persistent disk attached to a VM instance.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct AttachedDisk {
        /// Device name that the guest operating system will see.
        /// If not specified, this is default to the disk name.
        #[prost(string, tag = "3")]
        pub device_name: ::prost::alloc::string::String,
        #[prost(oneof = "attached_disk::Attached", tags = "1, 2")]
        pub attached: ::core::option::Option<attached_disk::Attached>,
    }
    /// Nested message and enum types in `AttachedDisk`.
    pub mod attached_disk {
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Attached {
            #[prost(message, tag = "1")]
            NewDisk(super::Disk),
            /// Name of an existing PD.
            #[prost(string, tag = "2")]
            ExistingDisk(::prost::alloc::string::String),
        }
    }
    /// Accelerator describes Compute Engine accelerators to be attached to VMs.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Accelerator {
        /// The accelerator type. For example, "nvidia-tesla-t4".
        /// See `gcloud compute accelerator-types list`.
        #[prost(string, tag = "1")]
        pub r#type: ::prost::alloc::string::String,
        /// The number of accelerators of this type.
        #[prost(int64, tag = "2")]
        pub count: i64,
    }
    /// InstancePolicy describes an instance type and resources attached to each VM
    /// created by this InstancePolicy.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct InstancePolicy {
        #[deprecated]
        #[prost(string, repeated, tag = "1")]
        pub allowed_machine_types: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// The Compute Engine machine type.
        #[prost(string, tag = "2")]
        pub machine_type: ::prost::alloc::string::String,
        /// The minimum CPU platform.
        /// See
        /// `<https://cloud.google.com/compute/docs/instances/specify-min-cpu-platform`.>
        /// Not yet implemented.
        #[prost(string, tag = "3")]
        pub min_cpu_platform: ::prost::alloc::string::String,
        /// The provisioning model.
        #[prost(enumeration = "ProvisioningModel", tag = "4")]
        pub provisioning_model: i32,
        /// The accelerators attached to each VM instance.
        /// Not yet implemented.
        #[prost(message, repeated, tag = "5")]
        pub accelerators: ::prost::alloc::vec::Vec<Accelerator>,
        /// Non-boot disks to be attached for each VM created by this InstancePolicy.
        /// New disks will be deleted when the attached VM is deleted.
        #[prost(message, repeated, tag = "6")]
        pub disks: ::prost::alloc::vec::Vec<AttachedDisk>,
    }
    /// Either an InstancePolicy or an instance template.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct InstancePolicyOrTemplate {
        #[prost(oneof = "instance_policy_or_template::PolicyTemplate", tags = "1, 2")]
        pub policy_template: ::core::option::Option<instance_policy_or_template::PolicyTemplate>,
    }
    /// Nested message and enum types in `InstancePolicyOrTemplate`.
    pub mod instance_policy_or_template {
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
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct NetworkInterface {
        /// The URL of the network resource.
        #[prost(string, tag = "1")]
        pub network: ::prost::alloc::string::String,
        /// The URL of the Subnetwork resource.
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
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct NetworkPolicy {
        /// Network configurations.
        #[prost(message, repeated, tag = "1")]
        pub network_interfaces: ::prost::alloc::vec::Vec<NetworkInterface>,
    }
    /// Compute Engine VM instance provisioning model.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
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
}
/// A TaskGroup contains one or multiple Tasks that share the same
/// Runnable but with different runtime parameters.
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
    /// default is 1
    #[prost(int64, tag = "4")]
    pub task_count: i64,
    /// Max number of tasks that can run in parallel.
    /// Default to min(task_count, 1000).
    #[prost(int64, tag = "5")]
    pub parallelism: i64,
    /// Scheduling policy for Tasks in the TaskGroup.
    #[prost(enumeration = "task_group::SchedulingPolicy", tag = "6")]
    pub scheduling_policy: i32,
    /// Compute resource allocation for the TaskGroup.
    /// If specified, it overrides resources in Job.
    #[prost(message, optional, tag = "7")]
    pub allocation_policy: ::core::option::Option<AllocationPolicy>,
    /// Labels for the TaskGroup.
    /// Labels could be user provided or system generated.
    /// You can assign up to 64 labels.  [Google Compute Engine label
    /// restrictions](<https://cloud.google.com/compute/docs/labeling-resources#restrictions>)
    /// apply.
    /// Label names that start with "goog-" or "google-" are reserved.
    #[prost(map = "string, string", tag = "8")]
    pub labels:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
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
/// Nested message and enum types in `TaskGroup`.
pub mod task_group {
    /// How Tasks in the TaskGroup should be scheduled relative to each other.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum SchedulingPolicy {
        /// Unspecified.
        Unspecified = 0,
        /// Run Tasks as soon as resources are available.
        AsSoonAsPossible = 1,
    }
}
/// Carries information about a Google Cloud service account.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServiceAccount {
    /// Email address of the service account. If not specified, the default
    /// Compute Engine service account for the project will be used.
    #[prost(string, tag = "1")]
    pub email: ::prost::alloc::string::String,
    /// List of scopes to be enabled for this service account on the VM, in
    /// addition to the cloud-platform API scope that will be added by default.
    #[prost(string, repeated, tag = "2")]
    pub scopes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// CreateJob Request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateJobRequest {
    /// Required. The parent resource name where the Job will be created.
    /// Pattern: "projects/{project}/locations/{location}"
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// ID used to uniquely identify the Job within its parent scope.
    /// This field should contain at most 63 characters.
    /// Only alphanumeric characters or '-' are accepted.
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
    /// Optional. An optional request ID to identify requests. Specify a unique request ID
    /// so that if you must retry your request, the server will know to ignore
    /// the request if it has already been completed. The server will guarantee
    /// that for at least 60 minutes since the first request.
    ///
    /// For example, consider a situation where you make an initial request and t
    /// he request times out. If you make the request again with the same request
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
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetJobRequest {
    /// Required. Job name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// DeleteJob Request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteJobRequest {
    /// Job name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. Reason for this deletion.
    #[prost(string, tag = "2")]
    pub reason: ::prost::alloc::string::String,
    /// Optional. An optional request ID to identify requests. Specify a unique request ID
    /// so that if you must retry your request, the server will know to ignore
    /// the request if it has already been completed. The server will guarantee
    /// that for at least 60 minutes after the first request.
    ///
    /// For example, consider a situation where you make an initial request and t
    /// he request times out. If you make the request again with the same request
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
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTaskRequest {
    /// Required. Task name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Represents the metadata of the long-running operation.
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
    /// have \[Operation.error][\] value with a \[google.rpc.Status.code][google.rpc.Status.code\] of 1,
    /// corresponding to `Code.CANCELLED`.
    #[prost(bool, tag = "6")]
    pub requested_cancellation: bool,
    /// Output only. API version used to start the operation.
    #[prost(string, tag = "7")]
    pub api_version: ::prost::alloc::string::String,
}
#[doc = r" Generated client implementations."]
pub mod batch_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Google Batch Service."]
    #[doc = " The service manages user submitted batch jobs and allocates Google Compute"]
    #[doc = " Engine VM instances to run the jobs."]
    #[derive(Debug, Clone)]
    pub struct BatchServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> BatchServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> BatchServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            BatchServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        #[doc = r" Compress requests with `gzip`."]
        #[doc = r""]
        #[doc = r" This requires the server to support it otherwise it might respond with an"]
        #[doc = r" error."]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        #[doc = r" Enable decompressing responses with `gzip`."]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        #[doc = " Create a Job."]
        pub async fn create_job(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateJobRequest>,
        ) -> Result<tonic::Response<super::Job>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.batch.v1alpha.BatchService/CreateJob",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Get a Job specified by its resource name."]
        pub async fn get_job(
            &mut self,
            request: impl tonic::IntoRequest<super::GetJobRequest>,
        ) -> Result<tonic::Response<super::Job>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.batch.v1alpha.BatchService/GetJob",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Delete a Job."]
        pub async fn delete_job(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteJobRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.batch.v1alpha.BatchService/DeleteJob",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " List all Jobs for a project within a region."]
        pub async fn list_jobs(
            &mut self,
            request: impl tonic::IntoRequest<super::ListJobsRequest>,
        ) -> Result<tonic::Response<super::ListJobsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.batch.v1alpha.BatchService/ListJobs",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Return a single Task."]
        pub async fn get_task(
            &mut self,
            request: impl tonic::IntoRequest<super::GetTaskRequest>,
        ) -> Result<tonic::Response<super::Task>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.batch.v1alpha.BatchService/GetTask",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " List Tasks associated with a job."]
        pub async fn list_tasks(
            &mut self,
            request: impl tonic::IntoRequest<super::ListTasksRequest>,
        ) -> Result<tonic::Response<super::ListTasksResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.batch.v1alpha.BatchService/ListTasks",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
