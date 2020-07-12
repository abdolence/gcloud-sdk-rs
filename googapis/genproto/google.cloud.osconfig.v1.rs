/// A request message to initiate patching across Compute Engine
/// instances.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecutePatchJobRequest {
    /// Required. The project in which to run this patch in the form `projects/*`
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Description of the patch job. Length of the description is limited
    /// to 1024 characters.
    #[prost(string, tag = "2")]
    pub description: std::string::String,
    /// Required. Instances to patch, either explicitly or filtered by some criteria such
    /// as zone or labels.
    #[prost(message, optional, tag = "7")]
    pub instance_filter: ::std::option::Option<PatchInstanceFilter>,
    /// Patch configuration being applied. If omitted, instances are
    /// patched using the default configurations.
    #[prost(message, optional, tag = "4")]
    pub patch_config: ::std::option::Option<PatchConfig>,
    /// Duration of the patch job. After the duration ends, the patch job
    /// times out.
    #[prost(message, optional, tag = "5")]
    pub duration: ::std::option::Option<::prost_types::Duration>,
    /// If this patch is a dry-run only, instances are contacted but
    /// will do nothing.
    #[prost(bool, tag = "6")]
    pub dry_run: bool,
    /// Display name for this patch job. This does not have to be unique.
    #[prost(string, tag = "8")]
    pub display_name: std::string::String,
}
/// Request to get an active or completed patch job.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPatchJobRequest {
    /// Required. Name of the patch in the form `projects/*/patchJobs/*`
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Request to list details for all instances that are part of a patch job.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPatchJobInstanceDetailsRequest {
    /// Required. The parent for the instances are in the form of `projects/*/patchJobs/*`.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// The maximum number of instance details records to return.  Default is 100.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A pagination token returned from a previous call
    /// that indicates where this listing should continue from.
    #[prost(string, tag = "3")]
    pub page_token: std::string::String,
    /// A filter expression that filters results listed in the response. This
    /// field supports filtering results by instance zone, name, state, or
    /// `failure_reason`.
    #[prost(string, tag = "4")]
    pub filter: std::string::String,
}
/// A response message for listing the instances details for a patch job.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPatchJobInstanceDetailsResponse {
    /// A list of instance status.
    #[prost(message, repeated, tag = "1")]
    pub patch_job_instance_details: ::std::vec::Vec<PatchJobInstanceDetails>,
    /// A pagination token that can be used to get the next page of results.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
}
/// Patch details for a VM instance. For more information about reviewing VM
/// instance details, see
/// [Listing all VM instance details for a specific patch
/// job](https://cloud.google.com/compute/docs/os-patch-management/manage-patch-jobs#list-instance-details).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PatchJobInstanceDetails {
    /// The instance name in the form `projects/*/zones/*/instances/*`
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// The unique identifier for the instance. This identifier is
    /// defined by the server.
    #[prost(string, tag = "2")]
    pub instance_system_id: std::string::String,
    /// Current state of instance patch.
    #[prost(enumeration = "instance::PatchState", tag = "3")]
    pub state: i32,
    /// If the patch fails, this field provides the reason.
    #[prost(string, tag = "4")]
    pub failure_reason: std::string::String,
    /// The number of times the agent that the agent attempts to apply the patch.
    #[prost(int64, tag = "5")]
    pub attempt_count: i64,
}
/// A request message for listing patch jobs.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPatchJobsRequest {
    /// Required. In the form of `projects/*`
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// The maximum number of instance status to return.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A pagination token returned from a previous call
    /// that indicates where this listing should continue from.
    #[prost(string, tag = "3")]
    pub page_token: std::string::String,
    /// If provided, this field specifies the criteria that must be met by patch
    /// jobs to be included in the response.
    /// Currently, filtering is only available on the patch_deployment field.
    #[prost(string, tag = "4")]
    pub filter: std::string::String,
}
/// A response message for listing patch jobs.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPatchJobsResponse {
    /// The list of patch jobs.
    #[prost(message, repeated, tag = "1")]
    pub patch_jobs: ::std::vec::Vec<PatchJob>,
    /// A pagination token that can be used to get the next page of results.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
}
/// A high level representation of a patch job that is either in progress
/// or has completed.
///
/// Instances details are not included in the job. To paginate through instance
/// details, use ListPatchJobInstanceDetails.
///
/// For more information about patch jobs, see
/// [Creating patch
/// jobs](https://cloud.google.com/compute/docs/os-patch-management/create-patch-job).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PatchJob {
    /// Unique identifier for this patch job in the form
    /// `projects/*/patchJobs/*`
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Display name for this patch job. This is not a unique identifier.
    #[prost(string, tag = "14")]
    pub display_name: std::string::String,
    /// Description of the patch job. Length of the description is limited
    /// to 1024 characters.
    #[prost(string, tag = "2")]
    pub description: std::string::String,
    /// Time this patch job was created.
    #[prost(message, optional, tag = "3")]
    pub create_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Last time this patch job was updated.
    #[prost(message, optional, tag = "4")]
    pub update_time: ::std::option::Option<::prost_types::Timestamp>,
    /// The current state of the PatchJob .
    #[prost(enumeration = "patch_job::State", tag = "5")]
    pub state: i32,
    /// Instances to patch.
    #[prost(message, optional, tag = "13")]
    pub instance_filter: ::std::option::Option<PatchInstanceFilter>,
    /// Patch configuration being applied.
    #[prost(message, optional, tag = "7")]
    pub patch_config: ::std::option::Option<PatchConfig>,
    /// Duration of the patch job. After the duration ends, the
    /// patch job times out.
    #[prost(message, optional, tag = "8")]
    pub duration: ::std::option::Option<::prost_types::Duration>,
    /// Summary of instance details.
    #[prost(message, optional, tag = "9")]
    pub instance_details_summary: ::std::option::Option<patch_job::InstanceDetailsSummary>,
    /// If this patch job is a dry run, the agent reports that it has
    /// finished without running any updates on the VM instance.
    #[prost(bool, tag = "10")]
    pub dry_run: bool,
    /// If this patch job failed, this message provides information about the
    /// failure.
    #[prost(string, tag = "11")]
    pub error_message: std::string::String,
    /// Reflects the overall progress of the patch job in the range of
    /// 0.0 being no progress to 100.0 being complete.
    #[prost(double, tag = "12")]
    pub percent_complete: f64,
    /// Output only. Name of the patch deployment that created this patch job.
    #[prost(string, tag = "15")]
    pub patch_deployment: std::string::String,
}
pub mod patch_job {
    /// A summary of the current patch state across all instances that this patch
    /// job affects. Contains counts of instances in different states. These states
    /// map to `InstancePatchState`. List patch job instance details to see the
    /// specific states of each instance.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct InstanceDetailsSummary {
        /// Number of instances pending patch job.
        #[prost(int64, tag = "1")]
        pub pending_instance_count: i64,
        /// Number of instances that are inactive.
        #[prost(int64, tag = "2")]
        pub inactive_instance_count: i64,
        /// Number of instances notified about patch job.
        #[prost(int64, tag = "3")]
        pub notified_instance_count: i64,
        /// Number of instances that have started.
        #[prost(int64, tag = "4")]
        pub started_instance_count: i64,
        /// Number of instances that are downloading patches.
        #[prost(int64, tag = "5")]
        pub downloading_patches_instance_count: i64,
        /// Number of instances that are applying patches.
        #[prost(int64, tag = "6")]
        pub applying_patches_instance_count: i64,
        /// Number of instances rebooting.
        #[prost(int64, tag = "7")]
        pub rebooting_instance_count: i64,
        /// Number of instances that have completed successfully.
        #[prost(int64, tag = "8")]
        pub succeeded_instance_count: i64,
        /// Number of instances that require reboot.
        #[prost(int64, tag = "9")]
        pub succeeded_reboot_required_instance_count: i64,
        /// Number of instances that failed.
        #[prost(int64, tag = "10")]
        pub failed_instance_count: i64,
        /// Number of instances that have acked and will start shortly.
        #[prost(int64, tag = "11")]
        pub acked_instance_count: i64,
        /// Number of instances that exceeded the time out while applying the patch.
        #[prost(int64, tag = "12")]
        pub timed_out_instance_count: i64,
        /// Number of instances that are running the pre-patch step.
        #[prost(int64, tag = "13")]
        pub pre_patch_step_instance_count: i64,
        /// Number of instances that are running the post-patch step.
        #[prost(int64, tag = "14")]
        pub post_patch_step_instance_count: i64,
        /// Number of instances that do not appear to be running the agent. Check to
        /// ensure that the agent is installed, running, and able to communicate with
        /// the service.
        #[prost(int64, tag = "15")]
        pub no_agent_detected_instance_count: i64,
    }
    /// Enumeration of the various states a patch job passes through as it
    /// executes.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// State must be specified.
        Unspecified = 0,
        /// The patch job was successfully initiated.
        Started = 1,
        /// The patch job is looking up instances to run the patch on.
        InstanceLookup = 2,
        /// Instances are being patched.
        Patching = 3,
        /// Patch job completed successfully.
        Succeeded = 4,
        /// Patch job completed but there were errors.
        CompletedWithErrors = 5,
        /// The patch job was canceled.
        Canceled = 6,
        /// The patch job timed out.
        TimedOut = 7,
    }
}
/// Patch configuration specifications. Contains details on how to apply the
/// patch(es) to a VM instance.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PatchConfig {
    /// Post-patch reboot settings.
    #[prost(enumeration = "patch_config::RebootConfig", tag = "1")]
    pub reboot_config: i32,
    /// Apt update settings. Use this setting to override the default `apt` patch
    /// rules.
    #[prost(message, optional, tag = "3")]
    pub apt: ::std::option::Option<AptSettings>,
    /// Yum update settings. Use this setting to override the default `yum` patch
    /// rules.
    #[prost(message, optional, tag = "4")]
    pub yum: ::std::option::Option<YumSettings>,
    /// Goo update settings. Use this setting to override the default `goo` patch
    /// rules.
    #[prost(message, optional, tag = "5")]
    pub goo: ::std::option::Option<GooSettings>,
    /// Zypper update settings. Use this setting to override the default `zypper`
    /// patch rules.
    #[prost(message, optional, tag = "6")]
    pub zypper: ::std::option::Option<ZypperSettings>,
    /// Windows update settings. Use this override the default windows patch rules.
    #[prost(message, optional, tag = "7")]
    pub windows_update: ::std::option::Option<WindowsUpdateSettings>,
    /// The `ExecStep` to run before the patch update.
    #[prost(message, optional, tag = "8")]
    pub pre_step: ::std::option::Option<ExecStep>,
    /// The `ExecStep` to run after the patch update.
    #[prost(message, optional, tag = "9")]
    pub post_step: ::std::option::Option<ExecStep>,
}
pub mod patch_config {
    /// Post-patch reboot settings.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum RebootConfig {
        /// The default behavior is DEFAULT.
        Unspecified = 0,
        /// The agent decides if a reboot is necessary by checking signals such as
        /// registry keys on Windows or `/var/run/reboot-required` on APT based
        /// systems. On RPM based systems, a set of core system package install times
        /// are compared with system boot time.
        Default = 1,
        /// Always reboot the machine after the update completes.
        Always = 2,
        /// Never reboot the machine after the update completes.
        Never = 3,
    }
}
/// Namespace for instance state enums.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Instance {}
pub mod instance {
    /// Patch state of an instance.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum PatchState {
        /// Unspecified.
        Unspecified = 0,
        /// The instance is not yet notified.
        Pending = 1,
        /// Instance is inactive and cannot be patched.
        Inactive = 2,
        /// The instance is notified that it should be patched.
        Notified = 3,
        /// The instance has started the patching process.
        Started = 4,
        /// The instance is downloading patches.
        DownloadingPatches = 5,
        /// The instance is applying patches.
        ApplyingPatches = 6,
        /// The instance is rebooting.
        Rebooting = 7,
        /// The instance has completed applying patches.
        Succeeded = 8,
        /// The instance has completed applying patches but a reboot is required.
        SucceededRebootRequired = 9,
        /// The instance has failed to apply the patch.
        Failed = 10,
        /// The instance acked the notification and will start shortly.
        Acked = 11,
        /// The instance exceeded the time out while applying the patch.
        TimedOut = 12,
        /// The instance is running the pre-patch step.
        RunningPrePatchStep = 13,
        /// The instance is running the post-patch step.
        RunningPostPatchStep = 14,
        /// The service could not detect the presence of the agent. Check to ensure
        /// that the agent is installed, running, and able to communicate with the
        /// service.
        NoAgentDetected = 15,
    }
}
/// Message for canceling a patch job.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CancelPatchJobRequest {
    /// Required. Name of the patch in the form `projects/*/patchJobs/*`
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Apt patching is completed by executing `apt-get update && apt-get
/// upgrade`. Additional options can be set to control how this is executed.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AptSettings {
    /// By changing the type to DIST, the patching is performed
    /// using `apt-get dist-upgrade` instead.
    #[prost(enumeration = "apt_settings::Type", tag = "1")]
    pub r#type: i32,
    /// List of packages to exclude from update. These packages will be excluded
    #[prost(string, repeated, tag = "2")]
    pub excludes: ::std::vec::Vec<std::string::String>,
    /// An exclusive list of packages to be updated. These are the only packages
    /// that will be updated. If these packages are not installed, they will be
    /// ignored. This field cannot be specified with any other patch configuration
    /// fields.
    #[prost(string, repeated, tag = "3")]
    pub exclusive_packages: ::std::vec::Vec<std::string::String>,
}
pub mod apt_settings {
    /// Apt patch type.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Type {
        /// By default, upgrade will be performed.
        Unspecified = 0,
        /// Runs `apt-get dist-upgrade`.
        Dist = 1,
        /// Runs `apt-get upgrade`.
        Upgrade = 2,
    }
}
/// Yum patching is performed by executing `yum update`. Additional options
/// can be set to control how this is executed.
///
/// Note that not all settings are supported on all platforms.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct YumSettings {
    /// Adds the `--security` flag to `yum update`. Not supported on
    /// all platforms.
    #[prost(bool, tag = "1")]
    pub security: bool,
    /// Will cause patch to run `yum update-minimal` instead.
    #[prost(bool, tag = "2")]
    pub minimal: bool,
    /// List of packages to exclude from update. These packages are excluded by
    /// using the yum `--exclude` flag.
    #[prost(string, repeated, tag = "3")]
    pub excludes: ::std::vec::Vec<std::string::String>,
    /// An exclusive list of packages to be updated. These are the only packages
    /// that will be updated. If these packages are not installed, they will be
    /// ignored. This field must not be specified with any other patch
    /// configuration fields.
    #[prost(string, repeated, tag = "4")]
    pub exclusive_packages: ::std::vec::Vec<std::string::String>,
}
/// Googet patching is performed by running `googet update`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GooSettings {}
/// Zypper patching is performed by running `zypper patch`.
/// See also https://en.opensuse.org/SDB:Zypper_manual.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ZypperSettings {
    /// Adds the `--with-optional` flag to `zypper patch`.
    #[prost(bool, tag = "1")]
    pub with_optional: bool,
    /// Adds the `--with-update` flag, to `zypper patch`.
    #[prost(bool, tag = "2")]
    pub with_update: bool,
    /// Install only patches with these categories.
    /// Common categories include security, recommended, and feature.
    #[prost(string, repeated, tag = "3")]
    pub categories: ::std::vec::Vec<std::string::String>,
    /// Install only patches with these severities.
    /// Common severities include critical, important, moderate, and low.
    #[prost(string, repeated, tag = "4")]
    pub severities: ::std::vec::Vec<std::string::String>,
    /// List of patches to exclude from update.
    #[prost(string, repeated, tag = "5")]
    pub excludes: ::std::vec::Vec<std::string::String>,
    /// An exclusive list of patches to be updated. These are the only patches
    /// that will be installed using 'zypper patch patch:<patch_name>' command.
    /// This field must not be used with any other patch configuration fields.
    #[prost(string, repeated, tag = "6")]
    pub exclusive_patches: ::std::vec::Vec<std::string::String>,
}
/// Windows patching is performed using the Windows Update Agent.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WindowsUpdateSettings {
    /// Only apply updates of these windows update classifications. If empty, all
    /// updates are applied.
    #[prost(
        enumeration = "windows_update_settings::Classification",
        repeated,
        tag = "1"
    )]
    pub classifications: ::std::vec::Vec<i32>,
    /// List of KBs to exclude from update.
    #[prost(string, repeated, tag = "2")]
    pub excludes: ::std::vec::Vec<std::string::String>,
    /// An exclusive list of kbs to be updated. These are the only patches
    /// that will be updated. This field must not be used with other
    /// patch configurations.
    #[prost(string, repeated, tag = "3")]
    pub exclusive_patches: ::std::vec::Vec<std::string::String>,
}
pub mod windows_update_settings {
    /// Microsoft Windows update classifications as defined in
    /// [1]
    /// https://support.microsoft.com/en-us/help/824684/description-of-the-standard-terminology-that-is-used-to-describe-micro
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Classification {
        /// Invalid. If classifications are included, they must be specified.
        Unspecified = 0,
        /// "A widely released fix for a specific problem that addresses a critical,
        /// non-security-related bug." [1]
        Critical = 1,
        /// "A widely released fix for a product-specific, security-related
        /// vulnerability. Security vulnerabilities are rated by their severity. The
        /// severity rating is indicated in the Microsoft security bulletin as
        /// critical, important, moderate, or low." [1]
        Security = 2,
        /// "A widely released and frequent software update that contains additions
        /// to a product's definition database. Definition databases are often used
        /// to detect objects that have specific attributes, such as malicious code,
        /// phishing websites, or junk mail." [1]
        Definition = 3,
        /// "Software that controls the input and output of a device." [1]
        Driver = 4,
        /// "New product functionality that is first distributed outside the context
        /// of a product release and that is typically included in the next full
        /// product release." [1]
        FeaturePack = 5,
        /// "A tested, cumulative set of all hotfixes, security updates, critical
        /// updates, and updates. Additionally, service packs may contain additional
        /// fixes for problems that are found internally since the release of the
        /// product. Service packs my also contain a limited number of
        /// customer-requested design changes or features." [1]
        ServicePack = 6,
        /// "A utility or feature that helps complete a task or set of tasks." [1]
        Tool = 7,
        /// "A tested, cumulative set of hotfixes, security updates, critical
        /// updates, and updates that are packaged together for easy deployment. A
        /// rollup generally targets a specific area, such as security, or a
        /// component of a product, such as Internet Information Services (IIS)." [1]
        UpdateRollup = 8,
        /// "A widely released fix for a specific problem. An update addresses a
        /// noncritical, non-security-related bug." [1]
        Update = 9,
    }
}
/// A step that runs an executable for a PatchJob.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecStep {
    /// The ExecStepConfig for all Linux VMs targeted by the PatchJob.
    #[prost(message, optional, tag = "1")]
    pub linux_exec_step_config: ::std::option::Option<ExecStepConfig>,
    /// The ExecStepConfig for all Windows VMs targeted by the PatchJob.
    #[prost(message, optional, tag = "2")]
    pub windows_exec_step_config: ::std::option::Option<ExecStepConfig>,
}
/// Common configurations for an ExecStep.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecStepConfig {
    /// Defaults to [0]. A list of possible return values that the
    /// execution can return to indicate a success.
    #[prost(int32, repeated, tag = "3")]
    pub allowed_success_codes: ::std::vec::Vec<i32>,
    /// The script interpreter to use to run the script. If no interpreter is
    /// specified the script will be executed directly, which will likely
    /// only succeed for scripts with [shebang lines]
    /// (https://en.wikipedia.org/wiki/Shebang_\(Unix\)).
    #[prost(enumeration = "exec_step_config::Interpreter", tag = "4")]
    pub interpreter: i32,
    /// Location of the executable.
    #[prost(oneof = "exec_step_config::Executable", tags = "1, 2")]
    pub executable: ::std::option::Option<exec_step_config::Executable>,
}
pub mod exec_step_config {
    /// The interpreter used to execute the a file.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Interpreter {
        /// Invalid for a Windows ExecStepConfig. For a Linux ExecStepConfig, the
        /// interpreter will be parsed from the shebang line of the script if
        /// unspecified.
        Unspecified = 0,
        /// Indicates that the script is run with `/bin/sh` on Linux and `cmd`
        /// on Windows.
        Shell = 1,
        /// Indicates that the file is run with PowerShell flags
        /// `-NonInteractive`, `-NoProfile`, and `-ExecutionPolicy Bypass`.
        Powershell = 2,
    }
    /// Location of the executable.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Executable {
        /// An absolute path to the executable on the VM.
        #[prost(string, tag = "1")]
        LocalPath(std::string::String),
        /// A Cloud Storage object containing the executable.
        #[prost(message, tag = "2")]
        GcsObject(super::GcsObject),
    }
}
/// Cloud Storage object representation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GcsObject {
    /// Required. Bucket of the Cloud Storage object.
    #[prost(string, tag = "1")]
    pub bucket: std::string::String,
    /// Required. Name of the Cloud Storage object.
    #[prost(string, tag = "2")]
    pub object: std::string::String,
    /// Required. Generation number of the Cloud Storage object. This is used to
    /// ensure that the ExecStep specified by this PatchJob does not change.
    #[prost(int64, tag = "3")]
    pub generation_number: i64,
}
/// A filter to target VM instances for patching. The targeted
/// VMs must meet all criteria specified. So if both labels and zones are
/// specified, the patch job targets only VMs with those labels and in those
/// zones.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PatchInstanceFilter {
    /// Target all VM instances in the project. If true, no other criteria is
    /// permitted.
    #[prost(bool, tag = "1")]
    pub all: bool,
    /// Targets VM instances matching ANY of these GroupLabels. This allows
    /// targeting of disparate groups of VM instances.
    #[prost(message, repeated, tag = "2")]
    pub group_labels: ::std::vec::Vec<patch_instance_filter::GroupLabel>,
    /// Targets VM instances in ANY of these zones. Leave empty to target VM
    /// instances in any zone.
    #[prost(string, repeated, tag = "3")]
    pub zones: ::std::vec::Vec<std::string::String>,
    /// Targets any of the VM instances specified. Instances are specified by their
    /// URI in the form `zones/[ZONE]/instances/[INSTANCE_NAME]`,
    /// `projects/[PROJECT_ID]/zones/[ZONE]/instances/[INSTANCE_NAME]`, or
    /// `https://www.googleapis.com/compute/v1/projects/[PROJECT_ID]/zones/[ZONE]/instances/[INSTANCE_NAME]`
    #[prost(string, repeated, tag = "4")]
    pub instances: ::std::vec::Vec<std::string::String>,
    /// Targets VMs whose name starts with one of these prefixes. Similar to
    /// labels, this is another way to group VMs when targeting configs, for
    /// example prefix="prod-".
    #[prost(string, repeated, tag = "5")]
    pub instance_name_prefixes: ::std::vec::Vec<std::string::String>,
}
pub mod patch_instance_filter {
    /// Targets a group of VM instances by using their [assigned
    /// labels](https://cloud.google.com/compute/docs/labeling-resources). Labels
    /// are key-value pairs. A `GroupLabel` is a combination of labels
    /// that is used to target VMs for a patch job.
    ///
    /// For example, a patch job can target VMs that have the following
    /// `GroupLabel`: `{"env":"test", "app":"web"}`. This means that the patch job
    /// is applied to VMs that have both the labels `env=test` and `app=web`.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GroupLabel {
        /// Compute Engine instance labels that must be present for a VM
        /// instance to be targeted by this filter.
        #[prost(map = "string, string", tag = "1")]
        pub labels: ::std::collections::HashMap<std::string::String, std::string::String>,
    }
}
/// Patch deployments are configurations that individual patch jobs use to
/// complete a patch. These configurations include instance filter, package
/// repository settings, and a schedule. For more information about creating and
/// managing patch deployments, see [Scheduling patch
/// jobs](https://cloud.google.com/compute/docs/os-patch-management/schedule-patch-jobs).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PatchDeployment {
    /// Unique name for the patch deployment resource in a project. The patch
    /// deployment name is in the form:
    /// `projects/{project_id}/patchDeployments/{patch_deployment_id}`.
    /// This field is ignored when you create a new patch deployment.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Optional. Description of the patch deployment. Length of the description is limited
    /// to 1024 characters.
    #[prost(string, tag = "2")]
    pub description: std::string::String,
    /// Required. VM instances to patch.
    #[prost(message, optional, tag = "3")]
    pub instance_filter: ::std::option::Option<PatchInstanceFilter>,
    /// Optional. Patch configuration that is applied.
    #[prost(message, optional, tag = "4")]
    pub patch_config: ::std::option::Option<PatchConfig>,
    /// Optional. Duration of the patch. After the duration ends, the patch times out.
    #[prost(message, optional, tag = "5")]
    pub duration: ::std::option::Option<::prost_types::Duration>,
    /// Output only. Time the patch deployment was created. Timestamp is in
    /// [RFC3339](https://www.ietf.org/rfc/rfc3339.txt) text format.
    #[prost(message, optional, tag = "8")]
    pub create_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Output only. Time the patch deployment was last updated. Timestamp is in
    /// [RFC3339](https://www.ietf.org/rfc/rfc3339.txt) text format.
    #[prost(message, optional, tag = "9")]
    pub update_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Output only. The last time a patch job was started by this deployment.
    /// Timestamp is in [RFC3339](https://www.ietf.org/rfc/rfc3339.txt) text
    /// format.
    #[prost(message, optional, tag = "10")]
    pub last_execute_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Schedule for the patch.
    #[prost(oneof = "patch_deployment::Schedule", tags = "6, 7")]
    pub schedule: ::std::option::Option<patch_deployment::Schedule>,
}
pub mod patch_deployment {
    /// Schedule for the patch.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Schedule {
        /// Required. Schedule a one-time execution.
        #[prost(message, tag = "6")]
        OneTimeSchedule(super::OneTimeSchedule),
        /// Required. Schedule recurring executions.
        #[prost(message, tag = "7")]
        RecurringSchedule(super::RecurringSchedule),
    }
}
/// Sets the time for a one time patch deployment. Timestamp is in
/// [RFC3339](https://www.ietf.org/rfc/rfc3339.txt) text format.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OneTimeSchedule {
    /// Required. The desired patch job execution time.
    #[prost(message, optional, tag = "1")]
    pub execute_time: ::std::option::Option<::prost_types::Timestamp>,
}
/// Sets the time for recurring patch deployments.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RecurringSchedule {
    /// Required. Defines the time zone that `time_of_day` is relative to.
    /// The rules for daylight saving time are determined by the chosen time zone.
    #[prost(message, optional, tag = "1")]
    pub time_zone: ::std::option::Option<super::super::super::r#type::TimeZone>,
    /// Optional. The time that the recurring schedule becomes effective.
    /// Defaults to `create_time` of the patch deployment.
    #[prost(message, optional, tag = "2")]
    pub start_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Optional. The end time at which a recurring patch deployment schedule is no longer
    /// active.
    #[prost(message, optional, tag = "3")]
    pub end_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Required. Time of the day to run a recurring deployment.
    #[prost(message, optional, tag = "4")]
    pub time_of_day: ::std::option::Option<super::super::super::r#type::TimeOfDay>,
    /// Required. The frequency unit of this recurring schedule.
    #[prost(enumeration = "recurring_schedule::Frequency", tag = "5")]
    pub frequency: i32,
    /// Output only. The time the last patch job ran successfully.
    #[prost(message, optional, tag = "9")]
    pub last_execute_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Output only. The time the next patch job is scheduled to run.
    #[prost(message, optional, tag = "10")]
    pub next_execute_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Configurations for this recurring schedule.
    /// Configurations must match frequency.
    #[prost(oneof = "recurring_schedule::ScheduleConfig", tags = "6, 7")]
    pub schedule_config: ::std::option::Option<recurring_schedule::ScheduleConfig>,
}
pub mod recurring_schedule {
    /// Specifies the frequency of the recurring patch deployments.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Frequency {
        /// Invalid. A frequency must be specified.
        Unspecified = 0,
        /// Indicates that the frequency should be expressed in terms of
        /// weeks.
        Weekly = 1,
        /// Indicates that the frequency should be expressed in terms of
        /// months.
        Monthly = 2,
    }
    /// Configurations for this recurring schedule.
    /// Configurations must match frequency.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ScheduleConfig {
        /// Required. Schedule with weekly executions.
        #[prost(message, tag = "6")]
        Weekly(super::WeeklySchedule),
        /// Required. Schedule with monthly executions.
        #[prost(message, tag = "7")]
        Monthly(super::MonthlySchedule),
    }
}
/// Represents a weekly schedule.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WeeklySchedule {
    /// Required. Day of the week.
    #[prost(enumeration = "super::super::super::r#type::DayOfWeek", tag = "1")]
    pub day_of_week: i32,
}
/// Represents a monthly schedule. An example of a valid monthly schedule is
/// "on the third Tuesday of the month" or "on the 15th of the month".
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MonthlySchedule {
    /// One day in a month.
    #[prost(oneof = "monthly_schedule::DayOfMonth", tags = "1, 2")]
    pub day_of_month: ::std::option::Option<monthly_schedule::DayOfMonth>,
}
pub mod monthly_schedule {
    /// One day in a month.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum DayOfMonth {
        /// Required. Week day in a month.
        #[prost(message, tag = "1")]
        WeekDayOfMonth(super::WeekDayOfMonth),
        /// Required. One day of the month. 1-31 indicates the 1st to the 31st day. -1
        /// indicates the last day of the month.
        /// Months without the target day will be skipped. For example, a schedule to
        /// run "every month on the 31st" will not run in February, April, June, etc.
        #[prost(int32, tag = "2")]
        MonthDay(i32),
    }
}
/// Represents one week day in a month. An example is "the 4th Sunday".
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WeekDayOfMonth {
    /// Required. Week number in a month. 1-4 indicates the 1st to 4th week of the month. -1
    /// indicates the last week of the month.
    #[prost(int32, tag = "1")]
    pub week_ordinal: i32,
    /// Required. A day of the week.
    #[prost(enumeration = "super::super::super::r#type::DayOfWeek", tag = "2")]
    pub day_of_week: i32,
}
/// A request message for creating a patch deployment.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreatePatchDeploymentRequest {
    /// Required. The project to apply this patch deployment to in the form `projects/*`.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Required. A name for the patch deployment in the project. When creating a name
    /// the following rules apply:
    /// * Must contain only lowercase letters, numbers, and hyphens.
    /// * Must start with a letter.
    /// * Must be between 1-63 characters.
    /// * Must end with a number or a letter.
    /// * Must be unique within the project.
    #[prost(string, tag = "2")]
    pub patch_deployment_id: std::string::String,
    /// Required. The patch deployment to create.
    #[prost(message, optional, tag = "3")]
    pub patch_deployment: ::std::option::Option<PatchDeployment>,
}
/// A request message for retrieving a patch deployment.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPatchDeploymentRequest {
    /// Required. The resource name of the patch deployment in the form
    /// `projects/*/patchDeployments/*`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// A request message for listing patch deployments.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPatchDeploymentsRequest {
    /// Required. The resource name of the parent in the form `projects/*`.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Optional. The maximum number of patch deployments to return. Default is 100.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. A pagination token returned from a previous call to ListPatchDeployments
    /// that indicates where this listing should continue from.
    #[prost(string, tag = "3")]
    pub page_token: std::string::String,
}
/// A response message for listing patch deployments.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPatchDeploymentsResponse {
    /// The list of patch deployments.
    #[prost(message, repeated, tag = "1")]
    pub patch_deployments: ::std::vec::Vec<PatchDeployment>,
    /// A pagination token that can be used to get the next page of patch
    /// deployments.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
}
/// A request message for deleting a patch deployment.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeletePatchDeploymentRequest {
    /// Required. The resource name of the patch deployment in the form
    /// `projects/*/patchDeployments/*`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
#[doc = r" Generated client implementations."]
pub mod os_config_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " OS Config API"]
    #[doc = ""]
    #[doc = " The OS Config service is a server-side component that you can use to"]
    #[doc = " manage package installations and patch jobs for virtual machine instances."]
    pub struct OsConfigServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> OsConfigServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + HttpBody + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as HttpBody>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = tonic::client::Grpc::with_interceptor(inner, interceptor);
            Self { inner }
        }
        #[doc = " Patch VM instances by creating and running a patch job."]
        pub async fn execute_patch_job(
            &mut self,
            request: impl tonic::IntoRequest<super::ExecutePatchJobRequest>,
        ) -> Result<tonic::Response<super::PatchJob>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.osconfig.v1.OsConfigService/ExecutePatchJob",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Get the patch job. This can be used to track the progress of an"]
        #[doc = " ongoing patch job or review the details of completed jobs."]
        pub async fn get_patch_job(
            &mut self,
            request: impl tonic::IntoRequest<super::GetPatchJobRequest>,
        ) -> Result<tonic::Response<super::PatchJob>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.osconfig.v1.OsConfigService/GetPatchJob",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Cancel a patch job. The patch job must be active. Canceled patch jobs"]
        #[doc = " cannot be restarted."]
        pub async fn cancel_patch_job(
            &mut self,
            request: impl tonic::IntoRequest<super::CancelPatchJobRequest>,
        ) -> Result<tonic::Response<super::PatchJob>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.osconfig.v1.OsConfigService/CancelPatchJob",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Get a list of patch jobs."]
        pub async fn list_patch_jobs(
            &mut self,
            request: impl tonic::IntoRequest<super::ListPatchJobsRequest>,
        ) -> Result<tonic::Response<super::ListPatchJobsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.osconfig.v1.OsConfigService/ListPatchJobs",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Get a list of instance details for a given patch job."]
        pub async fn list_patch_job_instance_details(
            &mut self,
            request: impl tonic::IntoRequest<super::ListPatchJobInstanceDetailsRequest>,
        ) -> Result<tonic::Response<super::ListPatchJobInstanceDetailsResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.osconfig.v1.OsConfigService/ListPatchJobInstanceDetails",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Create an OS Config patch deployment."]
        pub async fn create_patch_deployment(
            &mut self,
            request: impl tonic::IntoRequest<super::CreatePatchDeploymentRequest>,
        ) -> Result<tonic::Response<super::PatchDeployment>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.osconfig.v1.OsConfigService/CreatePatchDeployment",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Get an OS Config patch deployment."]
        pub async fn get_patch_deployment(
            &mut self,
            request: impl tonic::IntoRequest<super::GetPatchDeploymentRequest>,
        ) -> Result<tonic::Response<super::PatchDeployment>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.osconfig.v1.OsConfigService/GetPatchDeployment",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Get a page of OS Config patch deployments."]
        pub async fn list_patch_deployments(
            &mut self,
            request: impl tonic::IntoRequest<super::ListPatchDeploymentsRequest>,
        ) -> Result<tonic::Response<super::ListPatchDeploymentsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.osconfig.v1.OsConfigService/ListPatchDeployments",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Delete an OS Config patch deployment."]
        pub async fn delete_patch_deployment(
            &mut self,
            request: impl tonic::IntoRequest<super::DeletePatchDeploymentRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.osconfig.v1.OsConfigService/DeletePatchDeployment",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for OsConfigServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for OsConfigServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "OsConfigServiceClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod os_config_service_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with OsConfigServiceServer."]
    #[async_trait]
    pub trait OsConfigService: Send + Sync + 'static {
        #[doc = " Patch VM instances by creating and running a patch job."]
        async fn execute_patch_job(
            &self,
            request: tonic::Request<super::ExecutePatchJobRequest>,
        ) -> Result<tonic::Response<super::PatchJob>, tonic::Status>;
        #[doc = " Get the patch job. This can be used to track the progress of an"]
        #[doc = " ongoing patch job or review the details of completed jobs."]
        async fn get_patch_job(
            &self,
            request: tonic::Request<super::GetPatchJobRequest>,
        ) -> Result<tonic::Response<super::PatchJob>, tonic::Status>;
        #[doc = " Cancel a patch job. The patch job must be active. Canceled patch jobs"]
        #[doc = " cannot be restarted."]
        async fn cancel_patch_job(
            &self,
            request: tonic::Request<super::CancelPatchJobRequest>,
        ) -> Result<tonic::Response<super::PatchJob>, tonic::Status>;
        #[doc = " Get a list of patch jobs."]
        async fn list_patch_jobs(
            &self,
            request: tonic::Request<super::ListPatchJobsRequest>,
        ) -> Result<tonic::Response<super::ListPatchJobsResponse>, tonic::Status>;
        #[doc = " Get a list of instance details for a given patch job."]
        async fn list_patch_job_instance_details(
            &self,
            request: tonic::Request<super::ListPatchJobInstanceDetailsRequest>,
        ) -> Result<tonic::Response<super::ListPatchJobInstanceDetailsResponse>, tonic::Status>;
        #[doc = " Create an OS Config patch deployment."]
        async fn create_patch_deployment(
            &self,
            request: tonic::Request<super::CreatePatchDeploymentRequest>,
        ) -> Result<tonic::Response<super::PatchDeployment>, tonic::Status>;
        #[doc = " Get an OS Config patch deployment."]
        async fn get_patch_deployment(
            &self,
            request: tonic::Request<super::GetPatchDeploymentRequest>,
        ) -> Result<tonic::Response<super::PatchDeployment>, tonic::Status>;
        #[doc = " Get a page of OS Config patch deployments."]
        async fn list_patch_deployments(
            &self,
            request: tonic::Request<super::ListPatchDeploymentsRequest>,
        ) -> Result<tonic::Response<super::ListPatchDeploymentsResponse>, tonic::Status>;
        #[doc = " Delete an OS Config patch deployment."]
        async fn delete_patch_deployment(
            &self,
            request: tonic::Request<super::DeletePatchDeploymentRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status>;
    }
    #[doc = " OS Config API"]
    #[doc = ""]
    #[doc = " The OS Config service is a server-side component that you can use to"]
    #[doc = " manage package installations and patch jobs for virtual machine instances."]
    #[derive(Debug)]
    #[doc(hidden)]
    pub struct OsConfigServiceServer<T: OsConfigService> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: OsConfigService> OsConfigServiceServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, None);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, Some(interceptor.into()));
            Self { inner }
        }
    }
    impl<T, B> Service<http::Request<B>> for OsConfigServiceServer<T>
    where
        T: OsConfigService,
        B: HttpBody + Send + Sync + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/google.cloud.osconfig.v1.OsConfigService/ExecutePatchJob" => {
                    #[allow(non_camel_case_types)]
                    struct ExecutePatchJobSvc<T: OsConfigService>(pub Arc<T>);
                    impl<T: OsConfigService>
                        tonic::server::UnaryService<super::ExecutePatchJobRequest>
                        for ExecutePatchJobSvc<T>
                    {
                        type Response = super::PatchJob;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ExecutePatchJobRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.execute_patch_job(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ExecutePatchJobSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/google.cloud.osconfig.v1.OsConfigService/GetPatchJob" => {
                    #[allow(non_camel_case_types)]
                    struct GetPatchJobSvc<T: OsConfigService>(pub Arc<T>);
                    impl<T: OsConfigService> tonic::server::UnaryService<super::GetPatchJobRequest>
                        for GetPatchJobSvc<T>
                    {
                        type Response = super::PatchJob;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetPatchJobRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.get_patch_job(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetPatchJobSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/google.cloud.osconfig.v1.OsConfigService/CancelPatchJob" => {
                    #[allow(non_camel_case_types)]
                    struct CancelPatchJobSvc<T: OsConfigService>(pub Arc<T>);
                    impl<T: OsConfigService>
                        tonic::server::UnaryService<super::CancelPatchJobRequest>
                        for CancelPatchJobSvc<T>
                    {
                        type Response = super::PatchJob;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CancelPatchJobRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.cancel_patch_job(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = CancelPatchJobSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/google.cloud.osconfig.v1.OsConfigService/ListPatchJobs" => {
                    #[allow(non_camel_case_types)]
                    struct ListPatchJobsSvc<T: OsConfigService>(pub Arc<T>);
                    impl<T: OsConfigService>
                        tonic::server::UnaryService<super::ListPatchJobsRequest>
                        for ListPatchJobsSvc<T>
                    {
                        type Response = super::ListPatchJobsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListPatchJobsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.list_patch_jobs(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ListPatchJobsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/google.cloud.osconfig.v1.OsConfigService/ListPatchJobInstanceDetails" => {
                    #[allow(non_camel_case_types)]
                    struct ListPatchJobInstanceDetailsSvc<T: OsConfigService>(pub Arc<T>);
                    impl<T: OsConfigService>
                        tonic::server::UnaryService<super::ListPatchJobInstanceDetailsRequest>
                        for ListPatchJobInstanceDetailsSvc<T>
                    {
                        type Response = super::ListPatchJobInstanceDetailsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListPatchJobInstanceDetailsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { inner.list_patch_job_instance_details(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ListPatchJobInstanceDetailsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/google.cloud.osconfig.v1.OsConfigService/CreatePatchDeployment" => {
                    #[allow(non_camel_case_types)]
                    struct CreatePatchDeploymentSvc<T: OsConfigService>(pub Arc<T>);
                    impl<T: OsConfigService>
                        tonic::server::UnaryService<super::CreatePatchDeploymentRequest>
                        for CreatePatchDeploymentSvc<T>
                    {
                        type Response = super::PatchDeployment;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreatePatchDeploymentRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.create_patch_deployment(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = CreatePatchDeploymentSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/google.cloud.osconfig.v1.OsConfigService/GetPatchDeployment" => {
                    #[allow(non_camel_case_types)]
                    struct GetPatchDeploymentSvc<T: OsConfigService>(pub Arc<T>);
                    impl<T: OsConfigService>
                        tonic::server::UnaryService<super::GetPatchDeploymentRequest>
                        for GetPatchDeploymentSvc<T>
                    {
                        type Response = super::PatchDeployment;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetPatchDeploymentRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.get_patch_deployment(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetPatchDeploymentSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/google.cloud.osconfig.v1.OsConfigService/ListPatchDeployments" => {
                    #[allow(non_camel_case_types)]
                    struct ListPatchDeploymentsSvc<T: OsConfigService>(pub Arc<T>);
                    impl<T: OsConfigService>
                        tonic::server::UnaryService<super::ListPatchDeploymentsRequest>
                        for ListPatchDeploymentsSvc<T>
                    {
                        type Response = super::ListPatchDeploymentsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListPatchDeploymentsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.list_patch_deployments(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ListPatchDeploymentsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/google.cloud.osconfig.v1.OsConfigService/DeletePatchDeployment" => {
                    #[allow(non_camel_case_types)]
                    struct DeletePatchDeploymentSvc<T: OsConfigService>(pub Arc<T>);
                    impl<T: OsConfigService>
                        tonic::server::UnaryService<super::DeletePatchDeploymentRequest>
                        for DeletePatchDeploymentSvc<T>
                    {
                        type Response = ();
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeletePatchDeploymentRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.delete_patch_deployment(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = DeletePatchDeploymentSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .body(tonic::body::BoxBody::empty())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: OsConfigService> Clone for OsConfigServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: OsConfigService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
}
