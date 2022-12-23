/// ReplicationCycle contains information about the current replication cycle
/// status.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReplicationCycle {
    /// The time the replication cycle has started.
    #[prost(message, optional, tag = "1")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The current progress in percentage of this cycle.
    #[prost(int32, tag = "5")]
    pub progress_percent: i32,
}
/// ReplicationSync contain information about the last replica sync to the cloud.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReplicationSync {
    /// The most updated snapshot created time in the source that finished
    /// replication.
    #[prost(message, optional, tag = "1")]
    pub last_sync_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// MigratingVm describes the VM that will be migrated from a Source environment
/// and its replication state.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MigratingVm {
    /// Output only. The identifier of the MigratingVm.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The unique ID of the VM in the source.
    /// The VM's name in vSphere can be changed, so this is not the VM's name but
    /// rather its moRef id. This id is of the form vm-<num>.
    #[prost(string, tag = "2")]
    pub source_vm_id: ::prost::alloc::string::String,
    /// The display name attached to the MigratingVm by the user.
    #[prost(string, tag = "18")]
    pub display_name: ::prost::alloc::string::String,
    /// The description attached to the migrating VM by the user.
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    /// The replication schedule policy.
    #[prost(message, optional, tag = "8")]
    pub policy: ::core::option::Option<SchedulePolicy>,
    /// Output only. The time the migrating VM was created (this refers to this
    /// resource and not to the time it was installed in the source).
    #[prost(message, optional, tag = "9")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The last time the migrating VM resource was updated.
    #[prost(message, optional, tag = "10")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The most updated snapshot created time in the source that
    /// finished replication.
    #[prost(message, optional, tag = "11")]
    pub last_sync: ::core::option::Option<ReplicationSync>,
    /// Output only. State of the MigratingVm.
    #[prost(enumeration = "migrating_vm::State", tag = "23")]
    pub state: i32,
    /// Output only. The last time the migrating VM state was updated.
    #[prost(message, optional, tag = "22")]
    pub state_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The percentage progress of the current running replication
    /// cycle.
    #[prost(message, optional, tag = "13")]
    pub current_sync_info: ::core::option::Option<ReplicationCycle>,
    /// Output only. The group this migrating vm is included in, if any. The group
    /// is represented by the full path of the appropriate
    /// \[Group][google.cloud.vmmigration.v1.Group\] resource.
    #[prost(string, tag = "15")]
    pub group: ::prost::alloc::string::String,
    /// The labels of the migrating VM.
    #[prost(map = "string, string", tag = "16")]
    pub labels: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Output only. The recent [clone jobs]\[google.cloud.vmmigration.v1.CloneJob\]
    /// performed on the migrating VM. This field holds the vm's last completed
    /// clone job and the vm's running clone job, if one exists.
    /// Note: To have this field populated you need to explicitly request it via
    /// the "view" parameter of the Get/List request.
    #[prost(message, repeated, tag = "17")]
    pub recent_clone_jobs: ::prost::alloc::vec::Vec<CloneJob>,
    /// Output only. Provides details on the state of the Migrating VM in case of
    /// an error in replication.
    #[prost(message, optional, tag = "19")]
    pub error: ::core::option::Option<super::super::super::rpc::Status>,
    /// Output only. The recent cutover jobs performed on the migrating VM.
    /// This field holds the vm's last completed cutover job and the vm's
    /// running cutover job, if one exists.
    /// Note: To have this field populated you need to explicitly request it via
    /// the "view" parameter of the Get/List request.
    #[prost(message, repeated, tag = "20")]
    pub recent_cutover_jobs: ::prost::alloc::vec::Vec<CutoverJob>,
    /// The default configuration of the target VM that will be created in GCP as a
    /// result of the migration.
    #[prost(oneof = "migrating_vm::TargetVmDefaults", tags = "26")]
    pub target_vm_defaults: ::core::option::Option<migrating_vm::TargetVmDefaults>,
}
/// Nested message and enum types in `MigratingVm`.
pub mod migrating_vm {
    /// The possible values of the state/health of source VM.
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
        /// The state was not sampled by the health checks yet.
        Unspecified = 0,
        /// The VM in the source is being verified.
        Pending = 1,
        /// The source VM was verified, and it's ready to start replication.
        Ready = 2,
        /// Migration is going through the first sync cycle.
        FirstSync = 3,
        /// The replication is active, and it's running or scheduled to run.
        Active = 4,
        /// The source VM is being turned off, and a final replication is currently
        /// running.
        CuttingOver = 7,
        /// The source VM was stopped and replicated. The replication is currently
        /// paused.
        Cutover = 8,
        /// A cutover job is active and replication cycle is running the final sync.
        FinalSync = 9,
        /// The replication was paused by the user and no cycles are scheduled to
        /// run.
        Paused = 10,
        /// The migrating VM is being finalized and migration resources are being
        /// removed.
        Finalizing = 11,
        /// The replication process is done. The migrating VM is finalized and no
        /// longer consumes billable resources.
        Finalized = 12,
        /// The replication process encountered an unrecoverable error and was
        /// aborted.
        Error = 13,
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
                State::Ready => "READY",
                State::FirstSync => "FIRST_SYNC",
                State::Active => "ACTIVE",
                State::CuttingOver => "CUTTING_OVER",
                State::Cutover => "CUTOVER",
                State::FinalSync => "FINAL_SYNC",
                State::Paused => "PAUSED",
                State::Finalizing => "FINALIZING",
                State::Finalized => "FINALIZED",
                State::Error => "ERROR",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "PENDING" => Some(Self::Pending),
                "READY" => Some(Self::Ready),
                "FIRST_SYNC" => Some(Self::FirstSync),
                "ACTIVE" => Some(Self::Active),
                "CUTTING_OVER" => Some(Self::CuttingOver),
                "CUTOVER" => Some(Self::Cutover),
                "FINAL_SYNC" => Some(Self::FinalSync),
                "PAUSED" => Some(Self::Paused),
                "FINALIZING" => Some(Self::Finalizing),
                "FINALIZED" => Some(Self::Finalized),
                "ERROR" => Some(Self::Error),
                _ => None,
            }
        }
    }
    /// The default configuration of the target VM that will be created in GCP as a
    /// result of the migration.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum TargetVmDefaults {
        /// Details of the target VM in Compute Engine.
        #[prost(message, tag = "26")]
        ComputeEngineTargetDefaults(super::ComputeEngineTargetDefaults),
    }
}
/// CloneJob describes the process of creating a clone of a
/// \[MigratingVM][google.cloud.vmmigration.v1.MigratingVm\] to the
/// requested target based on the latest successful uploaded snapshots.
/// While the migration cycles of a MigratingVm take place, it is possible to
/// verify the uploaded VM can be started in the cloud, by creating a clone. The
/// clone can be created without any downtime, and it is created using the latest
/// snapshots which are already in the cloud. The cloneJob is only responsible
/// for its work, not its products, which means once it is finished, it will
/// never touch the instance it created. It will only delete it in case of the
/// CloneJob being cancelled or upon failure to clone.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CloneJob {
    /// Output only. The time the clone job was created (as an API call, not when
    /// it was actually created in the target).
    #[prost(message, optional, tag = "1")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The time the clone job was ended.
    #[prost(message, optional, tag = "22")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The name of the clone.
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
    /// Output only. State of the clone job.
    #[prost(enumeration = "clone_job::State", tag = "12")]
    pub state: i32,
    /// Output only. The time the state was last updated.
    #[prost(message, optional, tag = "14")]
    pub state_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Provides details for the errors that led to the Clone Job's
    /// state.
    #[prost(message, optional, tag = "17")]
    pub error: ::core::option::Option<super::super::super::rpc::Status>,
    /// Details of the VM to create as the target of this clone job.
    #[prost(oneof = "clone_job::TargetVmDetails", tags = "20")]
    pub target_vm_details: ::core::option::Option<clone_job::TargetVmDetails>,
}
/// Nested message and enum types in `CloneJob`.
pub mod clone_job {
    /// Possible states of the clone job.
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
        /// The state is unknown. This is used for API compatibility only and is not
        /// used by the system.
        Unspecified = 0,
        /// The clone job has not yet started.
        Pending = 1,
        /// The clone job is active and running.
        Active = 2,
        /// The clone job finished with errors.
        Failed = 3,
        /// The clone job finished successfully.
        Succeeded = 4,
        /// The clone job was cancelled.
        Cancelled = 5,
        /// The clone job is being cancelled.
        Cancelling = 6,
        /// OS adaptation is running as part of the clone job to generate license.
        AdaptingOs = 7,
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
                State::Active => "ACTIVE",
                State::Failed => "FAILED",
                State::Succeeded => "SUCCEEDED",
                State::Cancelled => "CANCELLED",
                State::Cancelling => "CANCELLING",
                State::AdaptingOs => "ADAPTING_OS",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "PENDING" => Some(Self::Pending),
                "ACTIVE" => Some(Self::Active),
                "FAILED" => Some(Self::Failed),
                "SUCCEEDED" => Some(Self::Succeeded),
                "CANCELLED" => Some(Self::Cancelled),
                "CANCELLING" => Some(Self::Cancelling),
                "ADAPTING_OS" => Some(Self::AdaptingOs),
                _ => None,
            }
        }
    }
    /// Details of the VM to create as the target of this clone job.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum TargetVmDetails {
        /// Output only. Details of the target VM in Compute Engine.
        #[prost(message, tag = "20")]
        ComputeEngineTargetDetails(super::ComputeEngineTargetDetails),
    }
}
/// CutoverJob message describes a cutover of a migrating VM. The CutoverJob is
/// the operation of shutting down the VM, creating a snapshot and
/// clonning the VM using the replicated snapshot.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CutoverJob {
    /// Output only. The time the cutover job was created (as an API call, not when
    /// it was actually created in the target).
    #[prost(message, optional, tag = "1")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The time the cutover job had finished.
    #[prost(message, optional, tag = "16")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The name of the cutover job.
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
    /// Output only. State of the cutover job.
    #[prost(enumeration = "cutover_job::State", tag = "5")]
    pub state: i32,
    /// Output only. The time the state was last updated.
    #[prost(message, optional, tag = "6")]
    pub state_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The current progress in percentage of the cutover job.
    #[prost(int32, tag = "13")]
    pub progress_percent: i32,
    /// Output only. Provides details for the errors that led to the Cutover Job's
    /// state.
    #[prost(message, optional, tag = "9")]
    pub error: ::core::option::Option<super::super::super::rpc::Status>,
    /// Output only. A message providing possible extra details about the current
    /// state.
    #[prost(string, tag = "10")]
    pub state_message: ::prost::alloc::string::String,
    /// Details of the VM to create as the target of this cutover job.
    #[prost(oneof = "cutover_job::TargetVmDetails", tags = "14")]
    pub target_vm_details: ::core::option::Option<cutover_job::TargetVmDetails>,
}
/// Nested message and enum types in `CutoverJob`.
pub mod cutover_job {
    /// Possible states of the cutover job.
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
        /// The state is unknown. This is used for API compatibility only and is not
        /// used by the system.
        Unspecified = 0,
        /// The cutover job has not yet started.
        Pending = 1,
        /// The cutover job finished with errors.
        Failed = 2,
        /// The cutover job finished successfully.
        Succeeded = 3,
        /// The cutover job was cancelled.
        Cancelled = 4,
        /// The cutover job is being cancelled.
        Cancelling = 5,
        /// The cutover job is active and running.
        Active = 6,
        /// OS adaptation is running as part of the cutover job to generate license.
        AdaptingOs = 7,
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
                State::Failed => "FAILED",
                State::Succeeded => "SUCCEEDED",
                State::Cancelled => "CANCELLED",
                State::Cancelling => "CANCELLING",
                State::Active => "ACTIVE",
                State::AdaptingOs => "ADAPTING_OS",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "PENDING" => Some(Self::Pending),
                "FAILED" => Some(Self::Failed),
                "SUCCEEDED" => Some(Self::Succeeded),
                "CANCELLED" => Some(Self::Cancelled),
                "CANCELLING" => Some(Self::Cancelling),
                "ACTIVE" => Some(Self::Active),
                "ADAPTING_OS" => Some(Self::AdaptingOs),
                _ => None,
            }
        }
    }
    /// Details of the VM to create as the target of this cutover job.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum TargetVmDetails {
        /// Output only. Details of the target VM in Compute Engine.
        #[prost(message, tag = "14")]
        ComputeEngineTargetDetails(super::ComputeEngineTargetDetails),
    }
}
/// Request message for 'CreateCloneJob' request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateCloneJobRequest {
    /// Required. The Clone's parent.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The clone job identifier.
    #[prost(string, tag = "2")]
    pub clone_job_id: ::prost::alloc::string::String,
    /// Required. The clone request body.
    #[prost(message, optional, tag = "3")]
    pub clone_job: ::core::option::Option<CloneJob>,
    /// A request ID to identify requests. Specify a unique request ID
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
/// Request message for 'CancelCloneJob' request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CancelCloneJobRequest {
    /// Required. The clone job id
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Response message for 'CancelCloneJob' request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CancelCloneJobResponse {}
/// Request message for 'ListCloneJobsRequest' request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCloneJobsRequest {
    /// Required. The parent, which owns this collection of source VMs.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. The maximum number of clone jobs to return. The service may
    /// return fewer than this value. If unspecified, at most 500 clone jobs will
    /// be returned. The maximum value is 1000; values above 1000 will be coerced
    /// to 1000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Required. A page token, received from a previous `ListCloneJobs` call.
    /// Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to `ListCloneJobs` must
    /// match the call that provided the page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. The filter request.
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Optional. the order by fields for the result.
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
/// Response message for 'ListCloneJobs' request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCloneJobsResponse {
    /// Output only. The list of clone jobs response.
    #[prost(message, repeated, tag = "1")]
    pub clone_jobs: ::prost::alloc::vec::Vec<CloneJob>,
    /// Output only. A token, which can be sent as `page_token` to retrieve the
    /// next page. If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Output only. Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Request message for 'GetCloneJob' request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCloneJobRequest {
    /// Required. The name of the CloneJob.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Source message describes a specific vm migration Source resource. It contains
/// the source environment information.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Source {
    /// Output only. The Source name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. The create time timestamp.
    #[prost(message, optional, tag = "2")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The update time timestamp.
    #[prost(message, optional, tag = "3")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The labels of the source.
    #[prost(map = "string, string", tag = "4")]
    pub labels: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// User-provided description of the source.
    #[prost(string, tag = "6")]
    pub description: ::prost::alloc::string::String,
    #[prost(oneof = "source::SourceDetails", tags = "10")]
    pub source_details: ::core::option::Option<source::SourceDetails>,
}
/// Nested message and enum types in `Source`.
pub mod source {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum SourceDetails {
        /// Vmware type source details.
        #[prost(message, tag = "10")]
        Vmware(super::VmwareSourceDetails),
    }
}
/// VmwareSourceDetails message describes a specific source details for the
/// vmware source type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VmwareSourceDetails {
    /// The credentials username.
    #[prost(string, tag = "1")]
    pub username: ::prost::alloc::string::String,
    /// Input only. The credentials password. This is write only and can not be
    /// read in a GET operation.
    #[prost(string, tag = "2")]
    pub password: ::prost::alloc::string::String,
    /// The ip address of the vcenter this Source represents.
    #[prost(string, tag = "3")]
    pub vcenter_ip: ::prost::alloc::string::String,
    /// The thumbprint representing the certificate for the vcenter.
    #[prost(string, tag = "4")]
    pub thumbprint: ::prost::alloc::string::String,
}
/// DatacenterConnector message describes a connector between the Source and GCP,
/// which is installed on a vmware datacenter (an OVA vm installed by the user)
/// to connect the Datacenter to GCP and support vm migration data transfer.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DatacenterConnector {
    /// Output only. The time the connector was created (as an API call, not when
    /// it was actually installed).
    #[prost(message, optional, tag = "1")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The last time the connector was updated with an API call.
    #[prost(message, optional, tag = "2")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The connector's name.
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
    /// Immutable. A unique key for this connector. This key is internal to the OVA
    /// connector and is supplied with its creation during the registration process
    /// and can not be modified.
    #[prost(string, tag = "12")]
    pub registration_id: ::prost::alloc::string::String,
    /// The service account to use in the connector when communicating with the
    /// cloud.
    #[prost(string, tag = "5")]
    pub service_account: ::prost::alloc::string::String,
    /// The version running in the DatacenterConnector. This is supplied by the OVA
    /// connector during the registration process and can not be modified.
    #[prost(string, tag = "6")]
    pub version: ::prost::alloc::string::String,
    /// Output only. The communication channel between the datacenter connector and
    /// GCP.
    #[prost(string, tag = "10")]
    pub bucket: ::prost::alloc::string::String,
    /// Output only. State of the DatacenterConnector, as determined by the health
    /// checks.
    #[prost(enumeration = "datacenter_connector::State", tag = "7")]
    pub state: i32,
    /// Output only. The time the state was last set.
    #[prost(message, optional, tag = "8")]
    pub state_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Provides details on the state of the Datacenter Connector in
    /// case of an error.
    #[prost(message, optional, tag = "11")]
    pub error: ::core::option::Option<super::super::super::rpc::Status>,
    /// Output only. Appliance OVA version.
    /// This is the OVA which is manually installed by the user and contains the
    /// infrastructure for the automatically updatable components on the appliance.
    #[prost(string, tag = "13")]
    pub appliance_infrastructure_version: ::prost::alloc::string::String,
    /// Output only. Appliance last installed update bundle version.
    /// This is the version of the automatically updatable components on the
    /// appliance.
    #[prost(string, tag = "14")]
    pub appliance_software_version: ::prost::alloc::string::String,
    /// Output only. The available versions for updating this appliance.
    #[prost(message, optional, tag = "15")]
    pub available_versions: ::core::option::Option<AvailableUpdates>,
    /// Output only. The status of the current / last upgradeAppliance operation.
    #[prost(message, optional, tag = "16")]
    pub upgrade_status: ::core::option::Option<UpgradeStatus>,
}
/// Nested message and enum types in `DatacenterConnector`.
pub mod datacenter_connector {
    /// The possible values of the state.
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
        /// The state is unknown. This is used for API compatibility only and is not
        /// used by the system.
        Unspecified = 0,
        /// The state was not sampled by the health checks yet.
        Pending = 1,
        /// The source was sampled by health checks and is not available.
        Offline = 2,
        /// The source is available but might not be usable yet due to unvalidated
        /// credentials or another reason. The credentials referred to are the ones
        /// to the Source. The error message will contain further details.
        Failed = 3,
        /// The source exists and its credentials were verified.
        Active = 4,
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
                State::Offline => "OFFLINE",
                State::Failed => "FAILED",
                State::Active => "ACTIVE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "PENDING" => Some(Self::Pending),
                "OFFLINE" => Some(Self::Offline),
                "FAILED" => Some(Self::Failed),
                "ACTIVE" => Some(Self::Active),
                _ => None,
            }
        }
    }
}
/// UpgradeStatus contains information about upgradeAppliance operation.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpgradeStatus {
    /// The version to upgrade to.
    #[prost(string, tag = "1")]
    pub version: ::prost::alloc::string::String,
    /// The state of the upgradeAppliance operation.
    #[prost(enumeration = "upgrade_status::State", tag = "2")]
    pub state: i32,
    /// Provides details on the state of the upgrade operation in case of an error.
    #[prost(message, optional, tag = "3")]
    pub error: ::core::option::Option<super::super::super::rpc::Status>,
    /// The time the operation was started.
    #[prost(message, optional, tag = "4")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The version from which we upgraded.
    #[prost(string, tag = "5")]
    pub previous_version: ::prost::alloc::string::String,
}
/// Nested message and enum types in `UpgradeStatus`.
pub mod upgrade_status {
    /// The possible values of the state.
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
        /// The state was not sampled by the health checks yet.
        Unspecified = 0,
        /// The upgrade has started.
        Running = 1,
        /// The upgrade failed.
        Failed = 2,
        /// The upgrade finished successfully.
        Succeeded = 3,
    }
    impl State {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                State::Unspecified => "STATE_UNSPECIFIED",
                State::Running => "RUNNING",
                State::Failed => "FAILED",
                State::Succeeded => "SUCCEEDED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "RUNNING" => Some(Self::Running),
                "FAILED" => Some(Self::Failed),
                "SUCCEEDED" => Some(Self::Succeeded),
                _ => None,
            }
        }
    }
}
/// Holds informatiom about the available versions for upgrade.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AvailableUpdates {
    /// The newest deployable version of the appliance.
    /// The current appliance can't be updated into this version, and the owner
    /// must manually deploy this OVA to a new appliance.
    #[prost(message, optional, tag = "1")]
    pub new_deployable_appliance: ::core::option::Option<ApplianceVersion>,
    /// The latest version for in place update.
    /// The current appliance can be updated to this version using the API or m4c
    /// CLI.
    #[prost(message, optional, tag = "2")]
    pub in_place_update: ::core::option::Option<ApplianceVersion>,
}
/// Describes an appliance version.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApplianceVersion {
    /// The appliance version.
    #[prost(string, tag = "1")]
    pub version: ::prost::alloc::string::String,
    /// A link for downloading the version.
    #[prost(string, tag = "2")]
    pub uri: ::prost::alloc::string::String,
    /// Determine whether it's critical to upgrade the appliance to this version.
    #[prost(bool, tag = "3")]
    pub critical: bool,
    /// Link to a page that contains the version release notes.
    #[prost(string, tag = "4")]
    pub release_notes_uri: ::prost::alloc::string::String,
}
/// Request message for 'ListSources' request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSourcesRequest {
    /// Required. The parent, which owns this collection of sources.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. The maximum number of sources to return. The service may return
    /// fewer than this value. If unspecified, at most 500 sources will be
    /// returned. The maximum value is 1000; values above 1000 will be coerced to
    /// 1000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Required. A page token, received from a previous `ListSources` call.
    /// Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to `ListSources` must
    /// match the call that provided the page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. The filter request.
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Optional. the order by fields for the result.
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
/// Response message for 'ListSources' request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSourcesResponse {
    /// Output only. The list of sources response.
    #[prost(message, repeated, tag = "1")]
    pub sources: ::prost::alloc::vec::Vec<Source>,
    /// Output only. A token, which can be sent as `page_token` to retrieve the
    /// next page. If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Output only. Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Request message for 'GetSource' request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSourceRequest {
    /// Required. The Source name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for 'CreateSource' request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateSourceRequest {
    /// Required. The Source's parent.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The source identifier.
    #[prost(string, tag = "2")]
    pub source_id: ::prost::alloc::string::String,
    /// Required. The create request body.
    #[prost(message, optional, tag = "3")]
    pub source: ::core::option::Option<Source>,
    /// A request ID to identify requests. Specify a unique request ID
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
/// Update message for 'UpdateSources' request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateSourceRequest {
    /// Field mask is used to specify the fields to be overwritten in the
    /// Source resource by the update.
    /// The fields specified in the update_mask are relative to the resource, not
    /// the full request. A field will be overwritten if it is in the mask. If the
    /// user does not provide a mask then all fields will be overwritten.
    #[prost(message, optional, tag = "1")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Required. The update request body.
    #[prost(message, optional, tag = "2")]
    pub source: ::core::option::Option<Source>,
    /// A request ID to identify requests. Specify a unique request ID
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
    #[prost(string, tag = "3")]
    pub request_id: ::prost::alloc::string::String,
}
/// Request message for 'DeleteSource' request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteSourceRequest {
    /// Required. The Source name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. A request ID to identify requests. Specify a unique request ID
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
    #[prost(string, tag = "2")]
    pub request_id: ::prost::alloc::string::String,
}
/// Request message for
/// \[fetchInventory][google.cloud.vmmigration.v1.VmMigration.FetchInventory\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FetchInventoryRequest {
    /// Required. The name of the Source.
    #[prost(string, tag = "1")]
    pub source: ::prost::alloc::string::String,
    /// If this flag is set to true, the source will be queried instead of using
    /// cached results. Using this flag will make the call slower.
    #[prost(bool, tag = "2")]
    pub force_refresh: bool,
}
/// VmwareVmDetails describes a VM in vCenter.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VmwareVmDetails {
    /// The VM's id in the source (note that this is not the MigratingVm's id).
    /// This is the moref id of the VM.
    #[prost(string, tag = "1")]
    pub vm_id: ::prost::alloc::string::String,
    /// The id of the vCenter's datacenter this VM is contained in.
    #[prost(string, tag = "2")]
    pub datacenter_id: ::prost::alloc::string::String,
    /// The descriptive name of the vCenter's datacenter this VM is contained in.
    #[prost(string, tag = "3")]
    pub datacenter_description: ::prost::alloc::string::String,
    /// The unique identifier of the VM in vCenter.
    #[prost(string, tag = "4")]
    pub uuid: ::prost::alloc::string::String,
    /// The display name of the VM. Note that this is not necessarily unique.
    #[prost(string, tag = "5")]
    pub display_name: ::prost::alloc::string::String,
    /// The power state of the VM at the moment list was taken.
    #[prost(enumeration = "vmware_vm_details::PowerState", tag = "6")]
    pub power_state: i32,
    /// The number of cpus in the VM.
    #[prost(int32, tag = "7")]
    pub cpu_count: i32,
    /// The size of the memory of the VM in MB.
    #[prost(int32, tag = "8")]
    pub memory_mb: i32,
    /// The number of disks the VM has.
    #[prost(int32, tag = "9")]
    pub disk_count: i32,
    /// The total size of the storage allocated to the VM in MB.
    #[prost(int64, tag = "12")]
    pub committed_storage_mb: i64,
    /// The VM's OS. See for example
    /// <https://vdc-repo.vmware.com/vmwb-repository/dcr-public/da47f910-60ac-438b-8b9b-6122f4d14524/16b7274a-bf8b-4b4c-a05e-746f2aa93c8c/doc/vim.vm.GuestOsDescriptor.GuestOsIdentifier.html>
    /// for types of strings this might hold.
    #[prost(string, tag = "11")]
    pub guest_description: ::prost::alloc::string::String,
    /// Output only. The VM Boot Option.
    #[prost(enumeration = "vmware_vm_details::BootOption", tag = "13")]
    pub boot_option: i32,
}
/// Nested message and enum types in `VmwareVmDetails`.
pub mod vmware_vm_details {
    /// Possible values for the power state of the VM.
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
    pub enum PowerState {
        /// Power state is not specified.
        Unspecified = 0,
        /// The VM is turned ON.
        On = 1,
        /// The VM is turned OFF.
        Off = 2,
        /// The VM is suspended. This is similar to hibernation or sleep mode.
        Suspended = 3,
    }
    impl PowerState {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                PowerState::Unspecified => "POWER_STATE_UNSPECIFIED",
                PowerState::On => "ON",
                PowerState::Off => "OFF",
                PowerState::Suspended => "SUSPENDED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "POWER_STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "ON" => Some(Self::On),
                "OFF" => Some(Self::Off),
                "SUSPENDED" => Some(Self::Suspended),
                _ => None,
            }
        }
    }
    /// Possible values for vm boot option.
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
    pub enum BootOption {
        /// The boot option is unknown.
        Unspecified = 0,
        /// The boot option is EFI.
        Efi = 1,
        /// The boot option is BIOS.
        Bios = 2,
    }
    impl BootOption {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                BootOption::Unspecified => "BOOT_OPTION_UNSPECIFIED",
                BootOption::Efi => "EFI",
                BootOption::Bios => "BIOS",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "BOOT_OPTION_UNSPECIFIED" => Some(Self::Unspecified),
                "EFI" => Some(Self::Efi),
                "BIOS" => Some(Self::Bios),
                _ => None,
            }
        }
    }
}
/// VmwareVmsDetails describes VMs in vCenter.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VmwareVmsDetails {
    /// The details of the vmware VMs.
    #[prost(message, repeated, tag = "1")]
    pub details: ::prost::alloc::vec::Vec<VmwareVmDetails>,
}
/// Response message for
/// \[fetchInventory][google.cloud.vmmigration.v1.VmMigration.FetchInventory\].
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FetchInventoryResponse {
    /// Output only. The timestamp when the source was last queried (if the result
    /// is from the cache).
    #[prost(message, optional, tag = "2")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(oneof = "fetch_inventory_response::SourceVms", tags = "1")]
    pub source_vms: ::core::option::Option<fetch_inventory_response::SourceVms>,
}
/// Nested message and enum types in `FetchInventoryResponse`.
pub mod fetch_inventory_response {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum SourceVms {
        /// The description of the VMs in a Source of type Vmware.
        #[prost(message, tag = "1")]
        VmwareVms(super::VmwareVmsDetails),
    }
}
/// Utilization report details the utilization (CPU, memory, etc.) of selected
/// source VMs.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UtilizationReport {
    /// Output only. The report unique name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The report display name, as assigned by the user.
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    /// Output only. Current state of the report.
    #[prost(enumeration = "utilization_report::State", tag = "3")]
    pub state: i32,
    /// Output only. The time the state was last set.
    #[prost(message, optional, tag = "4")]
    pub state_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Provides details on the state of the report in case of an
    /// error.
    #[prost(message, optional, tag = "5")]
    pub error: ::core::option::Option<super::super::super::rpc::Status>,
    /// Output only. The time the report was created (this refers to the time of
    /// the request, not the time the report creation completed).
    #[prost(message, optional, tag = "6")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Time frame of the report.
    #[prost(enumeration = "utilization_report::TimeFrame", tag = "7")]
    pub time_frame: i32,
    /// Output only. The point in time when the time frame ends. Notice that the
    /// time frame is counted backwards. For instance if the "frame_end_time" value
    /// is 2021/01/20 and the time frame is WEEK then the report covers the week
    /// between 2021/01/20 and 2021/01/14.
    #[prost(message, optional, tag = "8")]
    pub frame_end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Total number of VMs included in the report.
    #[prost(int32, tag = "9")]
    pub vm_count: i32,
    /// List of utilization information per VM.
    /// When sent as part of the request, the "vm_id" field is used in order to
    /// specify which VMs to include in the report. In that case all other fields
    /// are ignored.
    #[prost(message, repeated, tag = "10")]
    pub vms: ::prost::alloc::vec::Vec<VmUtilizationInfo>,
}
/// Nested message and enum types in `UtilizationReport`.
pub mod utilization_report {
    /// Utilization report state.
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
        /// The state is unknown. This value is not in use.
        Unspecified = 0,
        /// The report is in the making.
        Creating = 1,
        /// Report creation completed successfully.
        Succeeded = 2,
        /// Report creation failed.
        Failed = 3,
    }
    impl State {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                State::Unspecified => "STATE_UNSPECIFIED",
                State::Creating => "CREATING",
                State::Succeeded => "SUCCEEDED",
                State::Failed => "FAILED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "CREATING" => Some(Self::Creating),
                "SUCCEEDED" => Some(Self::Succeeded),
                "FAILED" => Some(Self::Failed),
                _ => None,
            }
        }
    }
    /// Report time frame options.
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
    pub enum TimeFrame {
        /// The time frame was not specified and will default to WEEK.
        Unspecified = 0,
        /// One week.
        Week = 1,
        /// One month.
        Month = 2,
        /// One year.
        Year = 3,
    }
    impl TimeFrame {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                TimeFrame::Unspecified => "TIME_FRAME_UNSPECIFIED",
                TimeFrame::Week => "WEEK",
                TimeFrame::Month => "MONTH",
                TimeFrame::Year => "YEAR",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "TIME_FRAME_UNSPECIFIED" => Some(Self::Unspecified),
                "WEEK" => Some(Self::Week),
                "MONTH" => Some(Self::Month),
                "YEAR" => Some(Self::Year),
                _ => None,
            }
        }
    }
}
/// Utilization information of a single VM.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VmUtilizationInfo {
    /// The VM's ID in the source.
    #[prost(string, tag = "3")]
    pub vm_id: ::prost::alloc::string::String,
    /// Utilization metrics for this VM.
    #[prost(message, optional, tag = "2")]
    pub utilization: ::core::option::Option<VmUtilizationMetrics>,
    #[prost(oneof = "vm_utilization_info::VmDetails", tags = "1")]
    pub vm_details: ::core::option::Option<vm_utilization_info::VmDetails>,
}
/// Nested message and enum types in `VmUtilizationInfo`.
pub mod vm_utilization_info {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum VmDetails {
        /// The description of the VM in a Source of type Vmware.
        #[prost(message, tag = "1")]
        VmwareVmDetails(super::VmwareVmDetails),
    }
}
/// Utilization metrics values for a single VM.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VmUtilizationMetrics {
    /// Max CPU usage, percent.
    #[prost(int32, tag = "9")]
    pub cpu_max_percent: i32,
    /// Average CPU usage, percent.
    #[prost(int32, tag = "10")]
    pub cpu_average_percent: i32,
    /// Max memory usage, percent.
    #[prost(int32, tag = "11")]
    pub memory_max_percent: i32,
    /// Average memory usage, percent.
    #[prost(int32, tag = "12")]
    pub memory_average_percent: i32,
    /// Max disk IO rate, in kilobytes per second.
    #[prost(int64, tag = "13")]
    pub disk_io_rate_max_kbps: i64,
    /// Average disk IO rate, in kilobytes per second.
    #[prost(int64, tag = "14")]
    pub disk_io_rate_average_kbps: i64,
    /// Max network throughput (combined transmit-rates and receive-rates), in
    /// kilobytes per second.
    #[prost(int64, tag = "15")]
    pub network_throughput_max_kbps: i64,
    /// Average network throughput (combined transmit-rates and receive-rates), in
    /// kilobytes per second.
    #[prost(int64, tag = "16")]
    pub network_throughput_average_kbps: i64,
}
/// Request message for 'ListUtilizationReports' request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListUtilizationReportsRequest {
    /// Required. The Utilization Reports parent.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. The level of details of each report.
    /// Defaults to BASIC.
    #[prost(enumeration = "UtilizationReportView", tag = "2")]
    pub view: i32,
    /// Optional. The maximum number of reports to return. The service may return
    /// fewer than this value. If unspecified, at most 500 reports will be
    /// returned. The maximum value is 1000; values above 1000 will be coerced to
    /// 1000.
    #[prost(int32, tag = "3")]
    pub page_size: i32,
    /// Required. A page token, received from a previous `ListUtilizationReports`
    /// call. Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to `ListUtilizationReports`
    /// must match the call that provided the page token.
    #[prost(string, tag = "4")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. The filter request.
    #[prost(string, tag = "5")]
    pub filter: ::prost::alloc::string::String,
    /// Optional. the order by fields for the result.
    #[prost(string, tag = "6")]
    pub order_by: ::prost::alloc::string::String,
}
/// Response message for 'ListUtilizationReports' request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListUtilizationReportsResponse {
    /// Output only. The list of reports.
    #[prost(message, repeated, tag = "1")]
    pub utilization_reports: ::prost::alloc::vec::Vec<UtilizationReport>,
    /// Output only. A token, which can be sent as `page_token` to retrieve the
    /// next page. If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Output only. Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Request message for 'GetUtilizationReport' request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetUtilizationReportRequest {
    /// Required. The Utilization Report name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. The level of details of the report.
    /// Defaults to FULL
    #[prost(enumeration = "UtilizationReportView", tag = "2")]
    pub view: i32,
}
/// Request message for 'CreateUtilizationReport' request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateUtilizationReportRequest {
    /// Required. The Utilization Report's parent.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The report to create.
    #[prost(message, optional, tag = "2")]
    pub utilization_report: ::core::option::Option<UtilizationReport>,
    /// Required. The ID to use for the report, which will become the final
    /// component of the reports's resource name.
    ///
    /// This value maximum length is 63 characters, and valid characters
    /// are /\[a-z][0-9\]-/. It must start with an english letter and must not
    /// end with a hyphen.
    #[prost(string, tag = "3")]
    pub utilization_report_id: ::prost::alloc::string::String,
    /// A request ID to identify requests. Specify a unique request ID
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
/// Request message for 'DeleteUtilizationReport' request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteUtilizationReportRequest {
    /// Required. The Utilization Report name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. A request ID to identify requests. Specify a unique request ID
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
    #[prost(string, tag = "2")]
    pub request_id: ::prost::alloc::string::String,
}
/// Response message for 'ListDatacenterConnectors' request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDatacenterConnectorsResponse {
    /// Output only. The list of sources response.
    #[prost(message, repeated, tag = "1")]
    pub datacenter_connectors: ::prost::alloc::vec::Vec<DatacenterConnector>,
    /// Output only. A token, which can be sent as `page_token` to retrieve the
    /// next page. If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Output only. Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Request message for 'GetDatacenterConnector' request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDatacenterConnectorRequest {
    /// Required. The name of the DatacenterConnector.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for 'CreateDatacenterConnector' request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateDatacenterConnectorRequest {
    /// Required. The DatacenterConnector's parent.
    /// Required. The Source in where the new DatacenterConnector will be created.
    /// For example:
    /// `projects/my-project/locations/us-central1/sources/my-source`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The datacenterConnector identifier.
    #[prost(string, tag = "2")]
    pub datacenter_connector_id: ::prost::alloc::string::String,
    /// Required. The create request body.
    #[prost(message, optional, tag = "3")]
    pub datacenter_connector: ::core::option::Option<DatacenterConnector>,
    /// A request ID to identify requests. Specify a unique request ID
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
/// Request message for 'DeleteDatacenterConnector' request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteDatacenterConnectorRequest {
    /// Required. The DatacenterConnector name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// A request ID to identify requests. Specify a unique request ID
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
    #[prost(string, tag = "2")]
    pub request_id: ::prost::alloc::string::String,
}
/// Request message for 'UpgradeAppliance' request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpgradeApplianceRequest {
    /// Required. The DatacenterConnector name.
    #[prost(string, tag = "1")]
    pub datacenter_connector: ::prost::alloc::string::String,
    /// A request ID to identify requests. Specify a unique request ID
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
    #[prost(string, tag = "2")]
    pub request_id: ::prost::alloc::string::String,
}
/// Response message for 'UpgradeAppliance' request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpgradeApplianceResponse {}
/// Request message for 'ListDatacenterConnectors' request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDatacenterConnectorsRequest {
    /// Required. The parent, which owns this collection of connectors.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. The maximum number of connectors to return. The service may
    /// return fewer than this value. If unspecified, at most 500 sources will be
    /// returned. The maximum value is 1000; values above 1000 will be coerced to
    /// 1000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Required. A page token, received from a previous `ListDatacenterConnectors`
    /// call. Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to
    /// `ListDatacenterConnectors` must match the call that provided the page
    /// token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. The filter request.
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Optional. the order by fields for the result.
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
/// ComputeEngineTargetDefaults is a collection of details for creating a VM in a
/// target Compute Engine project.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ComputeEngineTargetDefaults {
    /// The name of the VM to create.
    #[prost(string, tag = "1")]
    pub vm_name: ::prost::alloc::string::String,
    /// The full path of the resource of type TargetProject which represents the
    /// Compute Engine project in which to create this VM.
    #[prost(string, tag = "2")]
    pub target_project: ::prost::alloc::string::String,
    /// The zone in which to create the VM.
    #[prost(string, tag = "3")]
    pub zone: ::prost::alloc::string::String,
    /// The machine type series to create the VM with.
    #[prost(string, tag = "4")]
    pub machine_type_series: ::prost::alloc::string::String,
    /// The machine type to create the VM with.
    #[prost(string, tag = "5")]
    pub machine_type: ::prost::alloc::string::String,
    /// A map of network tags to associate with the VM.
    #[prost(string, repeated, tag = "6")]
    pub network_tags: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// List of NICs connected to this VM.
    #[prost(message, repeated, tag = "7")]
    pub network_interfaces: ::prost::alloc::vec::Vec<NetworkInterface>,
    /// The service account to associate the VM with.
    #[prost(string, tag = "8")]
    pub service_account: ::prost::alloc::string::String,
    /// The disk type to use in the VM.
    #[prost(enumeration = "ComputeEngineDiskType", tag = "9")]
    pub disk_type: i32,
    /// A map of labels to associate with the VM.
    #[prost(map = "string, string", tag = "10")]
    pub labels: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// The license type to use in OS adaptation.
    #[prost(enumeration = "ComputeEngineLicenseType", tag = "11")]
    pub license_type: i32,
    /// Output only. The OS license returned from the adaptation module report.
    #[prost(message, optional, tag = "12")]
    pub applied_license: ::core::option::Option<AppliedLicense>,
    /// Compute instance scheduling information (if empty default is used).
    #[prost(message, optional, tag = "13")]
    pub compute_scheduling: ::core::option::Option<ComputeScheduling>,
    /// Defines whether the instance has Secure Boot enabled.
    /// This can be set to true only if the vm boot option is EFI.
    #[prost(bool, tag = "14")]
    pub secure_boot: bool,
    /// Output only. The VM Boot Option, as set in the source vm.
    #[prost(enumeration = "ComputeEngineBootOption", tag = "15")]
    pub boot_option: i32,
    /// The metadata key/value pairs to assign to the VM.
    #[prost(map = "string, string", tag = "16")]
    pub metadata: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Additional licenses to assign to the VM.
    #[prost(string, repeated, tag = "17")]
    pub additional_licenses: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The hostname to assign to the VM.
    #[prost(string, tag = "18")]
    pub hostname: ::prost::alloc::string::String,
}
/// ComputeEngineTargetDetails is a collection of details for creating a VM in a
/// target Compute Engine project.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ComputeEngineTargetDetails {
    /// The name of the VM to create.
    #[prost(string, tag = "1")]
    pub vm_name: ::prost::alloc::string::String,
    /// The GCP target project ID or project name.
    #[prost(string, tag = "2")]
    pub project: ::prost::alloc::string::String,
    /// The zone in which to create the VM.
    #[prost(string, tag = "3")]
    pub zone: ::prost::alloc::string::String,
    /// The machine type series to create the VM with.
    #[prost(string, tag = "4")]
    pub machine_type_series: ::prost::alloc::string::String,
    /// The machine type to create the VM with.
    #[prost(string, tag = "5")]
    pub machine_type: ::prost::alloc::string::String,
    /// A map of network tags to associate with the VM.
    #[prost(string, repeated, tag = "6")]
    pub network_tags: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// List of NICs connected to this VM.
    #[prost(message, repeated, tag = "7")]
    pub network_interfaces: ::prost::alloc::vec::Vec<NetworkInterface>,
    /// The service account to associate the VM with.
    #[prost(string, tag = "8")]
    pub service_account: ::prost::alloc::string::String,
    /// The disk type to use in the VM.
    #[prost(enumeration = "ComputeEngineDiskType", tag = "9")]
    pub disk_type: i32,
    /// A map of labels to associate with the VM.
    #[prost(map = "string, string", tag = "10")]
    pub labels: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// The license type to use in OS adaptation.
    #[prost(enumeration = "ComputeEngineLicenseType", tag = "11")]
    pub license_type: i32,
    /// The OS license returned from the adaptation module report.
    #[prost(message, optional, tag = "12")]
    pub applied_license: ::core::option::Option<AppliedLicense>,
    /// Compute instance scheduling information (if empty default is used).
    #[prost(message, optional, tag = "13")]
    pub compute_scheduling: ::core::option::Option<ComputeScheduling>,
    /// Defines whether the instance has Secure Boot enabled.
    /// This can be set to true only if the vm boot option is EFI.
    #[prost(bool, tag = "14")]
    pub secure_boot: bool,
    /// The VM Boot Option, as set in the source vm.
    #[prost(enumeration = "ComputeEngineBootOption", tag = "15")]
    pub boot_option: i32,
    /// The metadata key/value pairs to assign to the VM.
    #[prost(map = "string, string", tag = "16")]
    pub metadata: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Additional licenses to assign to the VM.
    #[prost(string, repeated, tag = "17")]
    pub additional_licenses: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The hostname to assign to the VM.
    #[prost(string, tag = "18")]
    pub hostname: ::prost::alloc::string::String,
}
/// NetworkInterface represents a NIC of a VM.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NetworkInterface {
    /// The network to connect the NIC to.
    #[prost(string, tag = "1")]
    pub network: ::prost::alloc::string::String,
    /// The subnetwork to connect the NIC to.
    #[prost(string, tag = "2")]
    pub subnetwork: ::prost::alloc::string::String,
    /// The internal IP to define in the NIC.
    /// The formats accepted are: `ephemeral` \ ipv4 address \ a named address
    /// resource full path.
    #[prost(string, tag = "3")]
    pub internal_ip: ::prost::alloc::string::String,
    /// The external IP to define in the NIC.
    #[prost(string, tag = "4")]
    pub external_ip: ::prost::alloc::string::String,
}
/// AppliedLicense holds the license data returned by adaptation module report.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AppliedLicense {
    /// The license type that was used in OS adaptation.
    #[prost(enumeration = "applied_license::Type", tag = "1")]
    pub r#type: i32,
    /// The OS license returned from the adaptation module's report.
    #[prost(string, tag = "2")]
    pub os_license: ::prost::alloc::string::String,
}
/// Nested message and enum types in `AppliedLicense`.
pub mod applied_license {
    /// License types used in OS adaptation.
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
        /// Unspecified license for the OS.
        Unspecified = 0,
        /// No license available for the OS.
        None = 1,
        /// The license type is Pay As You Go license type.
        Payg = 2,
        /// The license type is is Bring Your Own License type.
        Byol = 3,
    }
    impl Type {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Type::Unspecified => "TYPE_UNSPECIFIED",
                Type::None => "NONE",
                Type::Payg => "PAYG",
                Type::Byol => "BYOL",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "NONE" => Some(Self::None),
                "PAYG" => Some(Self::Payg),
                "BYOL" => Some(Self::Byol),
                _ => None,
            }
        }
    }
}
/// Node Affinity: the configuration of desired nodes onto which this Instance
/// could be scheduled. Based on
/// <https://cloud.google.com/compute/docs/reference/rest/v1/instances/setScheduling>
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SchedulingNodeAffinity {
    /// The label key of Node resource to reference.
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
    /// The operator to use for the node resources specified in the `values`
    /// parameter.
    #[prost(enumeration = "scheduling_node_affinity::Operator", tag = "2")]
    pub operator: i32,
    /// Corresponds to the label values of Node resource.
    #[prost(string, repeated, tag = "3")]
    pub values: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Nested message and enum types in `SchedulingNodeAffinity`.
pub mod scheduling_node_affinity {
    /// Possible types of node selection operators. Valid operators are IN for
    /// affinity and NOT_IN for anti-affinity.
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
    pub enum Operator {
        /// An unknown, unexpected behavior.
        Unspecified = 0,
        /// The node resource group should be in these resources affinity.
        In = 1,
        /// The node resource group should not be in these resources affinity.
        NotIn = 2,
    }
    impl Operator {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Operator::Unspecified => "OPERATOR_UNSPECIFIED",
                Operator::In => "IN",
                Operator::NotIn => "NOT_IN",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "OPERATOR_UNSPECIFIED" => Some(Self::Unspecified),
                "IN" => Some(Self::In),
                "NOT_IN" => Some(Self::NotIn),
                _ => None,
            }
        }
    }
}
/// Scheduling information for VM on maintenance/restart behaviour and
/// node allocation in sole tenant nodes.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ComputeScheduling {
    /// How the instance should behave when the host machine undergoes
    /// maintenance that may temporarily impact instance performance.
    #[prost(enumeration = "compute_scheduling::OnHostMaintenance", tag = "1")]
    pub on_host_maintenance: i32,
    /// Whether the Instance should be automatically restarted whenever it is
    /// terminated by Compute Engine (not terminated by user).
    /// This configuration is identical to `automaticRestart` field in Compute
    /// Engine create instance under scheduling.
    /// It was changed to an enum (instead of a boolean) to match the default
    /// value in Compute Engine which is automatic restart.
    #[prost(enumeration = "compute_scheduling::RestartType", tag = "5")]
    pub restart_type: i32,
    /// A set of node affinity and anti-affinity configurations for sole tenant
    /// nodes.
    #[prost(message, repeated, tag = "3")]
    pub node_affinities: ::prost::alloc::vec::Vec<SchedulingNodeAffinity>,
    /// The minimum number of virtual CPUs this instance will consume when
    /// running on a sole-tenant node. Ignored if no node_affinites are
    /// configured.
    #[prost(int32, tag = "4")]
    pub min_node_cpus: i32,
}
/// Nested message and enum types in `ComputeScheduling`.
pub mod compute_scheduling {
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
    pub enum OnHostMaintenance {
        /// An unknown, unexpected behavior.
        Unspecified = 0,
        /// Terminate the instance when the host machine undergoes maintenance.
        Terminate = 1,
        /// Migrate the instance when the host machine undergoes maintenance.
        Migrate = 2,
    }
    impl OnHostMaintenance {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                OnHostMaintenance::Unspecified => "ON_HOST_MAINTENANCE_UNSPECIFIED",
                OnHostMaintenance::Terminate => "TERMINATE",
                OnHostMaintenance::Migrate => "MIGRATE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "ON_HOST_MAINTENANCE_UNSPECIFIED" => Some(Self::Unspecified),
                "TERMINATE" => Some(Self::Terminate),
                "MIGRATE" => Some(Self::Migrate),
                _ => None,
            }
        }
    }
    /// Defines whether the Instance should be automatically restarted whenever
    /// it is terminated by Compute Engine (not terminated by user).
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
    pub enum RestartType {
        /// Unspecified behavior. This will use the default.
        Unspecified = 0,
        /// The Instance should be automatically restarted whenever it is
        /// terminated by Compute Engine.
        AutomaticRestart = 1,
        /// The Instance isn't automatically restarted whenever it is
        /// terminated by Compute Engine.
        NoAutomaticRestart = 2,
    }
    impl RestartType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                RestartType::Unspecified => "RESTART_TYPE_UNSPECIFIED",
                RestartType::AutomaticRestart => "AUTOMATIC_RESTART",
                RestartType::NoAutomaticRestart => "NO_AUTOMATIC_RESTART",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "RESTART_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "AUTOMATIC_RESTART" => Some(Self::AutomaticRestart),
                "NO_AUTOMATIC_RESTART" => Some(Self::NoAutomaticRestart),
                _ => None,
            }
        }
    }
}
/// A policy for scheduling replications.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SchedulePolicy {
    /// The idle duration between replication stages.
    #[prost(message, optional, tag = "1")]
    pub idle_duration: ::core::option::Option<::prost_types::Duration>,
    /// A flag to indicate whether to skip OS adaptation during the replication
    /// sync. OS adaptation is a process where the VM's operating system undergoes
    /// changes and adaptations to fully function on Compute Engine.
    #[prost(bool, tag = "2")]
    pub skip_os_adaptation: bool,
}
/// Request message for 'CreateMigratingVm' request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateMigratingVmRequest {
    /// Required. The MigratingVm's parent.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The migratingVm identifier.
    #[prost(string, tag = "2")]
    pub migrating_vm_id: ::prost::alloc::string::String,
    /// Required. The create request body.
    #[prost(message, optional, tag = "3")]
    pub migrating_vm: ::core::option::Option<MigratingVm>,
    /// A request ID to identify requests. Specify a unique request ID
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
/// Request message for 'LisMigratingVmsRequest' request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListMigratingVmsRequest {
    /// Required. The parent, which owns this collection of MigratingVms.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. The maximum number of migrating VMs to return. The service may
    /// return fewer than this value. If unspecified, at most 500 migrating VMs
    /// will be returned. The maximum value is 1000; values above 1000 will be
    /// coerced to 1000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Required. A page token, received from a previous `ListMigratingVms` call.
    /// Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to `ListMigratingVms`
    /// must match the call that provided the page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. The filter request.
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Optional. the order by fields for the result.
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
    /// Optional. The level of details of each migrating VM.
    #[prost(enumeration = "MigratingVmView", tag = "6")]
    pub view: i32,
}
/// Response message for 'ListMigratingVms' request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListMigratingVmsResponse {
    /// Output only. The list of Migrating VMs response.
    #[prost(message, repeated, tag = "1")]
    pub migrating_vms: ::prost::alloc::vec::Vec<MigratingVm>,
    /// Output only. A token, which can be sent as `page_token` to retrieve the
    /// next page. If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Output only. Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Request message for 'GetMigratingVm' request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMigratingVmRequest {
    /// Required. The name of the MigratingVm.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. The level of details of the migrating VM.
    #[prost(enumeration = "MigratingVmView", tag = "2")]
    pub view: i32,
}
/// Request message for 'UpdateMigratingVm' request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateMigratingVmRequest {
    /// Field mask is used to specify the fields to be overwritten in the
    /// MigratingVm resource by the update.
    /// The fields specified in the update_mask are relative to the resource, not
    /// the full request. A field will be overwritten if it is in the mask. If the
    /// user does not provide a mask then all fields will be overwritten.
    #[prost(message, optional, tag = "1")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Required. The update request body.
    #[prost(message, optional, tag = "2")]
    pub migrating_vm: ::core::option::Option<MigratingVm>,
    /// A request ID to identify requests. Specify a unique request ID
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
    #[prost(string, tag = "3")]
    pub request_id: ::prost::alloc::string::String,
}
/// Request message for 'DeleteMigratingVm' request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteMigratingVmRequest {
    /// Required. The name of the MigratingVm.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for 'StartMigrationRequest' request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StartMigrationRequest {
    /// Required. The name of the MigratingVm.
    #[prost(string, tag = "1")]
    pub migrating_vm: ::prost::alloc::string::String,
}
/// Response message for 'StartMigration' request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StartMigrationResponse {}
/// Request message for 'PauseMigration' request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PauseMigrationRequest {
    /// Required. The name of the MigratingVm.
    #[prost(string, tag = "1")]
    pub migrating_vm: ::prost::alloc::string::String,
}
/// Response message for 'PauseMigration' request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PauseMigrationResponse {}
/// Request message for 'ResumeMigration' request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResumeMigrationRequest {
    /// Required. The name of the MigratingVm.
    #[prost(string, tag = "1")]
    pub migrating_vm: ::prost::alloc::string::String,
}
/// Response message for 'ResumeMigration' request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResumeMigrationResponse {}
/// Request message for 'FinalizeMigration' request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FinalizeMigrationRequest {
    /// Required. The name of the MigratingVm.
    #[prost(string, tag = "1")]
    pub migrating_vm: ::prost::alloc::string::String,
}
/// Response message for 'FinalizeMigration' request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FinalizeMigrationResponse {}
/// TargetProject message represents a target Compute Engine project for a
/// migration or a clone.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TargetProject {
    /// Output only. The name of the target project.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The target project ID (number) or project name.
    #[prost(string, tag = "2")]
    pub project: ::prost::alloc::string::String,
    /// The target project's description.
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    /// Output only. The time this target project resource was created (not related
    /// to when the Compute Engine project it points to was created).
    #[prost(message, optional, tag = "4")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The last time the target project resource was updated.
    #[prost(message, optional, tag = "5")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Request message for 'GetTargetProject' call.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTargetProjectRequest {
    /// Required. The TargetProject name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for 'ListTargetProjects' call.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTargetProjectsRequest {
    /// Required. The parent, which owns this collection of targets.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. The maximum number of targets to return. The service may return
    /// fewer than this value. If unspecified, at most 500 targets will be
    /// returned. The maximum value is 1000; values above 1000 will be coerced to
    /// 1000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Required. A page token, received from a previous `ListTargets` call.
    /// Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to `ListTargets` must
    /// match the call that provided the page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. The filter request.
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Optional. the order by fields for the result.
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
/// Response message for 'ListTargetProjects' call.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTargetProjectsResponse {
    /// Output only. The list of target response.
    #[prost(message, repeated, tag = "1")]
    pub target_projects: ::prost::alloc::vec::Vec<TargetProject>,
    /// Output only. A token, which can be sent as `page_token` to retrieve the
    /// next page. If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Output only. Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Request message for 'CreateTargetProject' request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateTargetProjectRequest {
    /// Required. The TargetProject's parent.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The target_project identifier.
    #[prost(string, tag = "2")]
    pub target_project_id: ::prost::alloc::string::String,
    /// Required. The create request body.
    #[prost(message, optional, tag = "3")]
    pub target_project: ::core::option::Option<TargetProject>,
    /// A request ID to identify requests. Specify a unique request ID
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
/// Update message for 'UpdateTargetProject' request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateTargetProjectRequest {
    /// Field mask is used to specify the fields to be overwritten in the
    /// TargetProject resource by the update.
    /// The fields specified in the update_mask are relative to the resource, not
    /// the full request. A field will be overwritten if it is in the mask. If the
    /// user does not provide a mask then all fields will be overwritten.
    #[prost(message, optional, tag = "1")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Required. The update request body.
    #[prost(message, optional, tag = "2")]
    pub target_project: ::core::option::Option<TargetProject>,
    /// A request ID to identify requests. Specify a unique request ID
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
    #[prost(string, tag = "3")]
    pub request_id: ::prost::alloc::string::String,
}
/// Request message for 'DeleteTargetProject' request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteTargetProjectRequest {
    /// Required. The TargetProject name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. A request ID to identify requests. Specify a unique request ID
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
    #[prost(string, tag = "2")]
    pub request_id: ::prost::alloc::string::String,
}
/// Describes message for 'Group' resource. The Group is a collections of several
/// MigratingVms.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Group {
    /// Output only. The Group name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. The create time timestamp.
    #[prost(message, optional, tag = "2")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The update time timestamp.
    #[prost(message, optional, tag = "3")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// User-provided description of the group.
    #[prost(string, tag = "4")]
    pub description: ::prost::alloc::string::String,
    /// Display name is a user defined name for this group which can be updated.
    #[prost(string, tag = "5")]
    pub display_name: ::prost::alloc::string::String,
}
/// Request message for 'ListGroups' request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListGroupsRequest {
    /// Required. The parent, which owns this collection of groups.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. The maximum number of groups to return. The service may return
    /// fewer than this value. If unspecified, at most 500 groups will be
    /// returned. The maximum value is 1000; values above 1000 will be coerced to
    /// 1000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Required. A page token, received from a previous `ListGroups` call.
    /// Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to `ListGroups` must
    /// match the call that provided the page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. The filter request.
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Optional. the order by fields for the result.
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
/// Response message for 'ListGroups' request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListGroupsResponse {
    /// Output only. The list of groups response.
    #[prost(message, repeated, tag = "1")]
    pub groups: ::prost::alloc::vec::Vec<Group>,
    /// Output only. A token, which can be sent as `page_token` to retrieve the
    /// next page. If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Output only. Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Request message for 'GetGroup' request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetGroupRequest {
    /// Required. The group name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for 'CreateGroup' request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateGroupRequest {
    /// Required. The Group's parent.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The group identifier.
    #[prost(string, tag = "2")]
    pub group_id: ::prost::alloc::string::String,
    /// Required. The create request body.
    #[prost(message, optional, tag = "3")]
    pub group: ::core::option::Option<Group>,
    /// A request ID to identify requests. Specify a unique request ID
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
/// Update message for 'UpdateGroups' request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateGroupRequest {
    /// Field mask is used to specify the fields to be overwritten in the
    /// Group resource by the update.
    /// The fields specified in the update_mask are relative to the resource, not
    /// the full request. A field will be overwritten if it is in the mask. If the
    /// user does not provide a mask then all fields will be overwritten.
    #[prost(message, optional, tag = "1")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Required. The update request body.
    #[prost(message, optional, tag = "2")]
    pub group: ::core::option::Option<Group>,
    /// A request ID to identify requests. Specify a unique request ID
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
    #[prost(string, tag = "3")]
    pub request_id: ::prost::alloc::string::String,
}
/// Request message for 'DeleteGroup' request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteGroupRequest {
    /// Required. The Group name.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. A request ID to identify requests. Specify a unique request ID
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
    #[prost(string, tag = "2")]
    pub request_id: ::prost::alloc::string::String,
}
/// Request message for 'AddGroupMigration' request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddGroupMigrationRequest {
    /// Required. The full path name of the Group to add to.
    #[prost(string, tag = "1")]
    pub group: ::prost::alloc::string::String,
    /// The full path name of the MigratingVm to add.
    #[prost(string, tag = "2")]
    pub migrating_vm: ::prost::alloc::string::String,
}
/// Response message for 'AddGroupMigration' request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddGroupMigrationResponse {}
/// Request message for 'RemoveMigration' request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveGroupMigrationRequest {
    /// Required. The name of the Group.
    #[prost(string, tag = "1")]
    pub group: ::prost::alloc::string::String,
    /// The MigratingVm to remove.
    #[prost(string, tag = "2")]
    pub migrating_vm: ::prost::alloc::string::String,
}
/// Response message for 'RemoveMigration' request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveGroupMigrationResponse {}
/// Request message for 'CreateCutoverJob' request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateCutoverJobRequest {
    /// Required. The Cutover's parent.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The cutover job identifier.
    #[prost(string, tag = "2")]
    pub cutover_job_id: ::prost::alloc::string::String,
    /// Required. The cutover request body.
    #[prost(message, optional, tag = "3")]
    pub cutover_job: ::core::option::Option<CutoverJob>,
    /// A request ID to identify requests. Specify a unique request ID
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
/// Request message for 'CancelCutoverJob' request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CancelCutoverJobRequest {
    /// Required. The cutover job id
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Response message for 'CancelCutoverJob' request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CancelCutoverJobResponse {}
/// Request message for 'ListCutoverJobsRequest' request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCutoverJobsRequest {
    /// Required. The parent, which owns this collection of migrating VMs.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. The maximum number of cutover jobs to return. The service may
    /// return fewer than this value. If unspecified, at most 500 cutover jobs will
    /// be returned. The maximum value is 1000; values above 1000 will be coerced
    /// to 1000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Required. A page token, received from a previous `ListCutoverJobs` call.
    /// Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to `ListCutoverJobs` must
    /// match the call that provided the page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. The filter request.
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Optional. the order by fields for the result.
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
/// Response message for 'ListCutoverJobs' request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCutoverJobsResponse {
    /// Output only. The list of cutover jobs response.
    #[prost(message, repeated, tag = "1")]
    pub cutover_jobs: ::prost::alloc::vec::Vec<CutoverJob>,
    /// Output only. A token, which can be sent as `page_token` to retrieve the
    /// next page. If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Output only. Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Request message for 'GetCutoverJob' request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCutoverJobRequest {
    /// Required. The name of the CutoverJob.
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
/// Represents migration resource error information that can be used with
/// google.rpc.Status message. MigrationError is used to present the user with
/// error information in migration operations.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MigrationError {
    /// Output only. The error code.
    #[prost(enumeration = "migration_error::ErrorCode", tag = "1")]
    pub code: i32,
    /// Output only. The localized error message.
    #[prost(message, optional, tag = "2")]
    pub error_message: ::core::option::Option<
        super::super::super::rpc::LocalizedMessage,
    >,
    /// Output only. Suggested action for solving the error.
    #[prost(message, optional, tag = "3")]
    pub action_item: ::core::option::Option<super::super::super::rpc::LocalizedMessage>,
    /// Output only. URL(s) pointing to additional information on handling the
    /// current error.
    #[prost(message, repeated, tag = "4")]
    pub help_links: ::prost::alloc::vec::Vec<super::super::super::rpc::help::Link>,
    /// Output only. The time the error occurred.
    #[prost(message, optional, tag = "5")]
    pub error_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Nested message and enum types in `MigrationError`.
pub mod migration_error {
    /// Represents resource error codes.
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
    pub enum ErrorCode {
        /// Default value. This value is not used.
        Unspecified = 0,
        /// Migrate for Compute encountered an unknown error.
        UnknownError = 1,
        /// Migrate for Compute encountered an error while validating replication
        /// source health.
        SourceValidationError = 2,
        /// Migrate for Compute encountered an error during source data operation.
        SourceReplicationError = 3,
        /// Migrate for Compute encountered an error during target data operation.
        TargetReplicationError = 4,
        /// Migrate for Compute encountered an error during OS adaptation.
        OsAdaptationError = 5,
        /// Migrate for Compute encountered an error in clone operation.
        CloneError = 6,
        /// Migrate for Compute encountered an error in cutover operation.
        CutoverError = 7,
        /// Migrate for Compute encountered an error during utilization report
        /// creation.
        UtilizationReportError = 8,
        /// Migrate for Compute encountered an error during appliance upgrade.
        ApplianceUpgradeError = 9,
    }
    impl ErrorCode {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ErrorCode::Unspecified => "ERROR_CODE_UNSPECIFIED",
                ErrorCode::UnknownError => "UNKNOWN_ERROR",
                ErrorCode::SourceValidationError => "SOURCE_VALIDATION_ERROR",
                ErrorCode::SourceReplicationError => "SOURCE_REPLICATION_ERROR",
                ErrorCode::TargetReplicationError => "TARGET_REPLICATION_ERROR",
                ErrorCode::OsAdaptationError => "OS_ADAPTATION_ERROR",
                ErrorCode::CloneError => "CLONE_ERROR",
                ErrorCode::CutoverError => "CUTOVER_ERROR",
                ErrorCode::UtilizationReportError => "UTILIZATION_REPORT_ERROR",
                ErrorCode::ApplianceUpgradeError => "APPLIANCE_UPGRADE_ERROR",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "ERROR_CODE_UNSPECIFIED" => Some(Self::Unspecified),
                "UNKNOWN_ERROR" => Some(Self::UnknownError),
                "SOURCE_VALIDATION_ERROR" => Some(Self::SourceValidationError),
                "SOURCE_REPLICATION_ERROR" => Some(Self::SourceReplicationError),
                "TARGET_REPLICATION_ERROR" => Some(Self::TargetReplicationError),
                "OS_ADAPTATION_ERROR" => Some(Self::OsAdaptationError),
                "CLONE_ERROR" => Some(Self::CloneError),
                "CUTOVER_ERROR" => Some(Self::CutoverError),
                "UTILIZATION_REPORT_ERROR" => Some(Self::UtilizationReportError),
                "APPLIANCE_UPGRADE_ERROR" => Some(Self::ApplianceUpgradeError),
                _ => None,
            }
        }
    }
}
/// Controls the level of details of a Utilization Report.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum UtilizationReportView {
    /// The default / unset value.
    /// The API will default to FULL on single report request and BASIC for
    /// multiple reports request.
    Unspecified = 0,
    /// Get the report metadata, without the list of VMs and their utilization
    /// info.
    Basic = 1,
    /// Include everything.
    Full = 2,
}
impl UtilizationReportView {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            UtilizationReportView::Unspecified => "UTILIZATION_REPORT_VIEW_UNSPECIFIED",
            UtilizationReportView::Basic => "BASIC",
            UtilizationReportView::Full => "FULL",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "UTILIZATION_REPORT_VIEW_UNSPECIFIED" => Some(Self::Unspecified),
            "BASIC" => Some(Self::Basic),
            "FULL" => Some(Self::Full),
            _ => None,
        }
    }
}
/// Controls the level of details of a Migrating VM.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum MigratingVmView {
    /// View is unspecified. The API will fallback to the default value.
    Unspecified = 0,
    /// Get the migrating VM basic details.
    /// The basic details do not include the recent clone jobs and recent cutover
    /// jobs lists.
    Basic = 1,
    /// Include everything.
    Full = 2,
}
impl MigratingVmView {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            MigratingVmView::Unspecified => "MIGRATING_VM_VIEW_UNSPECIFIED",
            MigratingVmView::Basic => "MIGRATING_VM_VIEW_BASIC",
            MigratingVmView::Full => "MIGRATING_VM_VIEW_FULL",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "MIGRATING_VM_VIEW_UNSPECIFIED" => Some(Self::Unspecified),
            "MIGRATING_VM_VIEW_BASIC" => Some(Self::Basic),
            "MIGRATING_VM_VIEW_FULL" => Some(Self::Full),
            _ => None,
        }
    }
}
/// Types of disks supported for Compute Engine VM.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ComputeEngineDiskType {
    /// An unspecified disk type. Will be used as STANDARD.
    Unspecified = 0,
    /// A Standard disk type.
    Standard = 1,
    /// SSD hard disk type.
    Ssd = 2,
    /// An alternative to SSD persistent disks that balance performance and
    /// cost.
    Balanced = 3,
}
impl ComputeEngineDiskType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ComputeEngineDiskType::Unspecified => "COMPUTE_ENGINE_DISK_TYPE_UNSPECIFIED",
            ComputeEngineDiskType::Standard => "COMPUTE_ENGINE_DISK_TYPE_STANDARD",
            ComputeEngineDiskType::Ssd => "COMPUTE_ENGINE_DISK_TYPE_SSD",
            ComputeEngineDiskType::Balanced => "COMPUTE_ENGINE_DISK_TYPE_BALANCED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "COMPUTE_ENGINE_DISK_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "COMPUTE_ENGINE_DISK_TYPE_STANDARD" => Some(Self::Standard),
            "COMPUTE_ENGINE_DISK_TYPE_SSD" => Some(Self::Ssd),
            "COMPUTE_ENGINE_DISK_TYPE_BALANCED" => Some(Self::Balanced),
            _ => None,
        }
    }
}
/// Types of licenses used in OS adaptation.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ComputeEngineLicenseType {
    /// The license type is the default for the OS.
    Default = 0,
    /// The license type is Pay As You Go license type.
    Payg = 1,
    /// The license type is Bring Your Own License type.
    Byol = 2,
}
impl ComputeEngineLicenseType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ComputeEngineLicenseType::Default => "COMPUTE_ENGINE_LICENSE_TYPE_DEFAULT",
            ComputeEngineLicenseType::Payg => "COMPUTE_ENGINE_LICENSE_TYPE_PAYG",
            ComputeEngineLicenseType::Byol => "COMPUTE_ENGINE_LICENSE_TYPE_BYOL",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "COMPUTE_ENGINE_LICENSE_TYPE_DEFAULT" => Some(Self::Default),
            "COMPUTE_ENGINE_LICENSE_TYPE_PAYG" => Some(Self::Payg),
            "COMPUTE_ENGINE_LICENSE_TYPE_BYOL" => Some(Self::Byol),
            _ => None,
        }
    }
}
/// Possible values for vm boot option.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ComputeEngineBootOption {
    /// The boot option is unknown.
    Unspecified = 0,
    /// The boot option is EFI.
    Efi = 1,
    /// The boot option is BIOS.
    Bios = 2,
}
impl ComputeEngineBootOption {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ComputeEngineBootOption::Unspecified => {
                "COMPUTE_ENGINE_BOOT_OPTION_UNSPECIFIED"
            }
            ComputeEngineBootOption::Efi => "COMPUTE_ENGINE_BOOT_OPTION_EFI",
            ComputeEngineBootOption::Bios => "COMPUTE_ENGINE_BOOT_OPTION_BIOS",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "COMPUTE_ENGINE_BOOT_OPTION_UNSPECIFIED" => Some(Self::Unspecified),
            "COMPUTE_ENGINE_BOOT_OPTION_EFI" => Some(Self::Efi),
            "COMPUTE_ENGINE_BOOT_OPTION_BIOS" => Some(Self::Bios),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod vm_migration_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// VM Migration Service
    #[derive(Debug, Clone)]
    pub struct VmMigrationClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl VmMigrationClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> VmMigrationClient<T>
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
        ) -> VmMigrationClient<InterceptedService<T, F>>
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
            VmMigrationClient::new(InterceptedService::new(inner, interceptor))
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
        /// Lists Sources in a given project and location.
        pub async fn list_sources(
            &mut self,
            request: impl tonic::IntoRequest<super::ListSourcesRequest>,
        ) -> Result<tonic::Response<super::ListSourcesResponse>, tonic::Status> {
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
                "/google.cloud.vmmigration.v1.VmMigration/ListSources",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets details of a single Source.
        pub async fn get_source(
            &mut self,
            request: impl tonic::IntoRequest<super::GetSourceRequest>,
        ) -> Result<tonic::Response<super::Source>, tonic::Status> {
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
                "/google.cloud.vmmigration.v1.VmMigration/GetSource",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates a new Source in a given project and location.
        pub async fn create_source(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateSourceRequest>,
        ) -> Result<
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
                "/google.cloud.vmmigration.v1.VmMigration/CreateSource",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates the parameters of a single Source.
        pub async fn update_source(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateSourceRequest>,
        ) -> Result<
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
                "/google.cloud.vmmigration.v1.VmMigration/UpdateSource",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes a single Source.
        pub async fn delete_source(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteSourceRequest>,
        ) -> Result<
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
                "/google.cloud.vmmigration.v1.VmMigration/DeleteSource",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// List remote source's inventory of VMs.
        /// The remote source is the onprem vCenter (remote in the sense it's not in
        /// Compute Engine). The inventory describes the list of existing VMs in that
        /// source. Note that this operation lists the VMs on the remote source, as
        /// opposed to listing the MigratingVms resources in the vmmigration service.
        pub async fn fetch_inventory(
            &mut self,
            request: impl tonic::IntoRequest<super::FetchInventoryRequest>,
        ) -> Result<tonic::Response<super::FetchInventoryResponse>, tonic::Status> {
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
                "/google.cloud.vmmigration.v1.VmMigration/FetchInventory",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists Utilization Reports of the given Source.
        pub async fn list_utilization_reports(
            &mut self,
            request: impl tonic::IntoRequest<super::ListUtilizationReportsRequest>,
        ) -> Result<
            tonic::Response<super::ListUtilizationReportsResponse>,
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
                "/google.cloud.vmmigration.v1.VmMigration/ListUtilizationReports",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets a single Utilization Report.
        pub async fn get_utilization_report(
            &mut self,
            request: impl tonic::IntoRequest<super::GetUtilizationReportRequest>,
        ) -> Result<tonic::Response<super::UtilizationReport>, tonic::Status> {
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
                "/google.cloud.vmmigration.v1.VmMigration/GetUtilizationReport",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates a new UtilizationReport.
        pub async fn create_utilization_report(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateUtilizationReportRequest>,
        ) -> Result<
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
                "/google.cloud.vmmigration.v1.VmMigration/CreateUtilizationReport",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes a single Utilization Report.
        pub async fn delete_utilization_report(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteUtilizationReportRequest>,
        ) -> Result<
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
                "/google.cloud.vmmigration.v1.VmMigration/DeleteUtilizationReport",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists DatacenterConnectors in a given Source.
        pub async fn list_datacenter_connectors(
            &mut self,
            request: impl tonic::IntoRequest<super::ListDatacenterConnectorsRequest>,
        ) -> Result<
            tonic::Response<super::ListDatacenterConnectorsResponse>,
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
                "/google.cloud.vmmigration.v1.VmMigration/ListDatacenterConnectors",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets details of a single DatacenterConnector.
        pub async fn get_datacenter_connector(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDatacenterConnectorRequest>,
        ) -> Result<tonic::Response<super::DatacenterConnector>, tonic::Status> {
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
                "/google.cloud.vmmigration.v1.VmMigration/GetDatacenterConnector",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates a new DatacenterConnector in a given Source.
        pub async fn create_datacenter_connector(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateDatacenterConnectorRequest>,
        ) -> Result<
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
                "/google.cloud.vmmigration.v1.VmMigration/CreateDatacenterConnector",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes a single DatacenterConnector.
        pub async fn delete_datacenter_connector(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteDatacenterConnectorRequest>,
        ) -> Result<
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
                "/google.cloud.vmmigration.v1.VmMigration/DeleteDatacenterConnector",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Upgrades the appliance relate to this DatacenterConnector to the in-place
        /// updateable version.
        pub async fn upgrade_appliance(
            &mut self,
            request: impl tonic::IntoRequest<super::UpgradeApplianceRequest>,
        ) -> Result<
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
                "/google.cloud.vmmigration.v1.VmMigration/UpgradeAppliance",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates a new MigratingVm in a given Source.
        pub async fn create_migrating_vm(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateMigratingVmRequest>,
        ) -> Result<
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
                "/google.cloud.vmmigration.v1.VmMigration/CreateMigratingVm",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists MigratingVms in a given Source.
        pub async fn list_migrating_vms(
            &mut self,
            request: impl tonic::IntoRequest<super::ListMigratingVmsRequest>,
        ) -> Result<tonic::Response<super::ListMigratingVmsResponse>, tonic::Status> {
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
                "/google.cloud.vmmigration.v1.VmMigration/ListMigratingVms",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets details of a single MigratingVm.
        pub async fn get_migrating_vm(
            &mut self,
            request: impl tonic::IntoRequest<super::GetMigratingVmRequest>,
        ) -> Result<tonic::Response<super::MigratingVm>, tonic::Status> {
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
                "/google.cloud.vmmigration.v1.VmMigration/GetMigratingVm",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates the parameters of a single MigratingVm.
        pub async fn update_migrating_vm(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateMigratingVmRequest>,
        ) -> Result<
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
                "/google.cloud.vmmigration.v1.VmMigration/UpdateMigratingVm",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes a single MigratingVm.
        pub async fn delete_migrating_vm(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteMigratingVmRequest>,
        ) -> Result<
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
                "/google.cloud.vmmigration.v1.VmMigration/DeleteMigratingVm",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Starts migration for a VM. Starts the process of uploading
        /// data and creating snapshots, in replication cycles scheduled by the policy.
        pub async fn start_migration(
            &mut self,
            request: impl tonic::IntoRequest<super::StartMigrationRequest>,
        ) -> Result<
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
                "/google.cloud.vmmigration.v1.VmMigration/StartMigration",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Resumes a migration for a VM. When called on a paused migration, will start
        /// the process of uploading data and creating snapshots; when called on a
        /// completed cut-over migration, will update the migration to active state and
        /// start the process of uploading data and creating snapshots.
        pub async fn resume_migration(
            &mut self,
            request: impl tonic::IntoRequest<super::ResumeMigrationRequest>,
        ) -> Result<
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
                "/google.cloud.vmmigration.v1.VmMigration/ResumeMigration",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Pauses a migration for a VM. If cycle tasks are running they will be
        /// cancelled, preserving source task data. Further replication cycles will not
        /// be triggered while the VM is paused.
        pub async fn pause_migration(
            &mut self,
            request: impl tonic::IntoRequest<super::PauseMigrationRequest>,
        ) -> Result<
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
                "/google.cloud.vmmigration.v1.VmMigration/PauseMigration",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Marks a migration as completed, deleting migration resources that are no
        /// longer being used. Only applicable after cutover is done.
        pub async fn finalize_migration(
            &mut self,
            request: impl tonic::IntoRequest<super::FinalizeMigrationRequest>,
        ) -> Result<
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
                "/google.cloud.vmmigration.v1.VmMigration/FinalizeMigration",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Initiates a Clone of a specific migrating VM.
        pub async fn create_clone_job(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateCloneJobRequest>,
        ) -> Result<
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
                "/google.cloud.vmmigration.v1.VmMigration/CreateCloneJob",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Initiates the cancellation of a running clone job.
        pub async fn cancel_clone_job(
            &mut self,
            request: impl tonic::IntoRequest<super::CancelCloneJobRequest>,
        ) -> Result<
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
                "/google.cloud.vmmigration.v1.VmMigration/CancelCloneJob",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists CloneJobs of a given migrating VM.
        pub async fn list_clone_jobs(
            &mut self,
            request: impl tonic::IntoRequest<super::ListCloneJobsRequest>,
        ) -> Result<tonic::Response<super::ListCloneJobsResponse>, tonic::Status> {
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
                "/google.cloud.vmmigration.v1.VmMigration/ListCloneJobs",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets details of a single CloneJob.
        pub async fn get_clone_job(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCloneJobRequest>,
        ) -> Result<tonic::Response<super::CloneJob>, tonic::Status> {
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
                "/google.cloud.vmmigration.v1.VmMigration/GetCloneJob",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Initiates a Cutover of a specific migrating VM.
        /// The returned LRO is completed when the cutover job resource is created
        /// and the job is initiated.
        pub async fn create_cutover_job(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateCutoverJobRequest>,
        ) -> Result<
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
                "/google.cloud.vmmigration.v1.VmMigration/CreateCutoverJob",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Initiates the cancellation of a running cutover job.
        pub async fn cancel_cutover_job(
            &mut self,
            request: impl tonic::IntoRequest<super::CancelCutoverJobRequest>,
        ) -> Result<
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
                "/google.cloud.vmmigration.v1.VmMigration/CancelCutoverJob",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists CutoverJobs of a given migrating VM.
        pub async fn list_cutover_jobs(
            &mut self,
            request: impl tonic::IntoRequest<super::ListCutoverJobsRequest>,
        ) -> Result<tonic::Response<super::ListCutoverJobsResponse>, tonic::Status> {
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
                "/google.cloud.vmmigration.v1.VmMigration/ListCutoverJobs",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets details of a single CutoverJob.
        pub async fn get_cutover_job(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCutoverJobRequest>,
        ) -> Result<tonic::Response<super::CutoverJob>, tonic::Status> {
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
                "/google.cloud.vmmigration.v1.VmMigration/GetCutoverJob",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists Groups in a given project and location.
        pub async fn list_groups(
            &mut self,
            request: impl tonic::IntoRequest<super::ListGroupsRequest>,
        ) -> Result<tonic::Response<super::ListGroupsResponse>, tonic::Status> {
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
                "/google.cloud.vmmigration.v1.VmMigration/ListGroups",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets details of a single Group.
        pub async fn get_group(
            &mut self,
            request: impl tonic::IntoRequest<super::GetGroupRequest>,
        ) -> Result<tonic::Response<super::Group>, tonic::Status> {
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
                "/google.cloud.vmmigration.v1.VmMigration/GetGroup",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates a new Group in a given project and location.
        pub async fn create_group(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateGroupRequest>,
        ) -> Result<
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
                "/google.cloud.vmmigration.v1.VmMigration/CreateGroup",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates the parameters of a single Group.
        pub async fn update_group(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateGroupRequest>,
        ) -> Result<
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
                "/google.cloud.vmmigration.v1.VmMigration/UpdateGroup",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes a single Group.
        pub async fn delete_group(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteGroupRequest>,
        ) -> Result<
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
                "/google.cloud.vmmigration.v1.VmMigration/DeleteGroup",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Adds a MigratingVm to a Group.
        pub async fn add_group_migration(
            &mut self,
            request: impl tonic::IntoRequest<super::AddGroupMigrationRequest>,
        ) -> Result<
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
                "/google.cloud.vmmigration.v1.VmMigration/AddGroupMigration",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Removes a MigratingVm from a Group.
        pub async fn remove_group_migration(
            &mut self,
            request: impl tonic::IntoRequest<super::RemoveGroupMigrationRequest>,
        ) -> Result<
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
                "/google.cloud.vmmigration.v1.VmMigration/RemoveGroupMigration",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists TargetProjects in a given project.
        ///
        /// NOTE: TargetProject is a global resource; hence the only supported value
        /// for location is `global`.
        pub async fn list_target_projects(
            &mut self,
            request: impl tonic::IntoRequest<super::ListTargetProjectsRequest>,
        ) -> Result<tonic::Response<super::ListTargetProjectsResponse>, tonic::Status> {
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
                "/google.cloud.vmmigration.v1.VmMigration/ListTargetProjects",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets details of a single TargetProject.
        ///
        /// NOTE: TargetProject is a global resource; hence the only supported value
        /// for location is `global`.
        pub async fn get_target_project(
            &mut self,
            request: impl tonic::IntoRequest<super::GetTargetProjectRequest>,
        ) -> Result<tonic::Response<super::TargetProject>, tonic::Status> {
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
                "/google.cloud.vmmigration.v1.VmMigration/GetTargetProject",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates a new TargetProject in a given project.
        ///
        /// NOTE: TargetProject is a global resource; hence the only supported value
        /// for location is `global`.
        pub async fn create_target_project(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateTargetProjectRequest>,
        ) -> Result<
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
                "/google.cloud.vmmigration.v1.VmMigration/CreateTargetProject",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates the parameters of a single TargetProject.
        ///
        /// NOTE: TargetProject is a global resource; hence the only supported value
        /// for location is `global`.
        pub async fn update_target_project(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateTargetProjectRequest>,
        ) -> Result<
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
                "/google.cloud.vmmigration.v1.VmMigration/UpdateTargetProject",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes a single TargetProject.
        ///
        /// NOTE: TargetProject is a global resource; hence the only supported value
        /// for location is `global`.
        pub async fn delete_target_project(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteTargetProjectRequest>,
        ) -> Result<
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
                "/google.cloud.vmmigration.v1.VmMigration/DeleteTargetProject",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
