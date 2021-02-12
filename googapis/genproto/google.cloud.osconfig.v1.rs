// OS Config Inventory is a service for collecting and reporting operating
// system and package information on VM instances.

/// The inventory details of a VM.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Inventory {
    /// Base level operating system information for the VM.
    #[prost(message, optional, tag = "1")]
    pub os_info: ::core::option::Option<inventory::OsInfo>,
    /// Inventory items related to the VM keyed by an opaque unique identifier for
    /// each inventory item.  The identifier is unique to each distinct and
    /// addressable inventory item and will change, when there is a new package
    /// version.
    #[prost(map = "string, message", tag = "2")]
    pub items: ::std::collections::HashMap<::prost::alloc::string::String, inventory::Item>,
}
/// Nested message and enum types in `Inventory`.
pub mod inventory {
    /// Operating system information for the VM.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct OsInfo {
        /// The VM hostname.
        #[prost(string, tag = "9")]
        pub hostname: ::prost::alloc::string::String,
        /// The operating system long name.
        /// For example 'Debian GNU/Linux 9' or 'Microsoft Window Server 2019
        /// Datacenter'.
        #[prost(string, tag = "2")]
        pub long_name: ::prost::alloc::string::String,
        /// The operating system short name.
        /// For example, 'windows' or 'debian'.
        #[prost(string, tag = "3")]
        pub short_name: ::prost::alloc::string::String,
        /// The version of the operating system.
        #[prost(string, tag = "4")]
        pub version: ::prost::alloc::string::String,
        /// The system architecture of the operating system.
        #[prost(string, tag = "5")]
        pub architecture: ::prost::alloc::string::String,
        /// The kernel version of the operating system.
        #[prost(string, tag = "6")]
        pub kernel_version: ::prost::alloc::string::String,
        /// The kernel release of the operating system.
        #[prost(string, tag = "7")]
        pub kernel_release: ::prost::alloc::string::String,
        /// The current version of the OS Config agent running on the VM.
        #[prost(string, tag = "8")]
        pub osconfig_agent_version: ::prost::alloc::string::String,
    }
    /// A single piece of inventory on a VM.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Item {
        /// Identifier for this item, unique across items for this VM.
        #[prost(string, tag = "1")]
        pub id: ::prost::alloc::string::String,
        /// The origin of this inventory item.
        #[prost(enumeration = "item::OriginType", tag = "2")]
        pub origin_type: i32,
        /// When this inventory item was first detected.
        #[prost(message, optional, tag = "8")]
        pub create_time: ::core::option::Option<::prost_types::Timestamp>,
        /// When this inventory item was last modified.
        #[prost(message, optional, tag = "9")]
        pub update_time: ::core::option::Option<::prost_types::Timestamp>,
        /// The specific type of inventory, correlating to its specific details.
        #[prost(enumeration = "item::Type", tag = "5")]
        pub r#type: i32,
        /// Specific details of this inventory item based on its type.
        #[prost(oneof = "item::Details", tags = "6, 7")]
        pub details: ::core::option::Option<item::Details>,
    }
    /// Nested message and enum types in `Item`.
    pub mod item {
        /// The origin of a specific inventory item.
        #[derive(
            Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration,
        )]
        #[repr(i32)]
        pub enum OriginType {
            /// Invalid. An origin type must be specified.
            Unspecified = 0,
            /// This inventory item was discovered as the result of the agent
            /// reporting inventory via the reporting API.
            InventoryReport = 1,
        }
        /// The different types of inventory that are tracked on a VM.
        #[derive(
            Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration,
        )]
        #[repr(i32)]
        pub enum Type {
            /// Invalid. An type must be specified.
            Unspecified = 0,
            /// This represents a package that is installed on the VM.
            InstalledPackage = 1,
            /// This represents an update that is available for a package.
            AvailablePackage = 2,
        }
        /// Specific details of this inventory item based on its type.
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Details {
            /// Software package present on the VM instance.
            #[prost(message, tag = "6")]
            InstalledPackage(super::SoftwarePackage),
            /// Software package available to be installed on the VM instance.
            #[prost(message, tag = "7")]
            AvailablePackage(super::SoftwarePackage),
        }
    }
    /// Software package information of the operating system.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SoftwarePackage {
        /// Information about the different types of software packages.
        #[prost(oneof = "software_package::Details", tags = "1, 2, 3, 4, 5, 6, 7, 8")]
        pub details: ::core::option::Option<software_package::Details>,
    }
    /// Nested message and enum types in `SoftwarePackage`.
    pub mod software_package {
        /// Information about the different types of software packages.
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Details {
            /// Yum package info.
            /// For details about the yum package manager, see
            /// https://access.redhat.com/documentation/en-us/red_hat_enterprise_linux/6/html/deployment_guide/ch-yum.
            #[prost(message, tag = "1")]
            YumPackage(super::VersionedPackage),
            /// Details of an APT package.
            /// For details about the apt package manager, see
            /// https://wiki.debian.org/Apt.
            #[prost(message, tag = "2")]
            AptPackage(super::VersionedPackage),
            /// Details of a Zypper package.
            /// For details about the Zypper package manager, see
            /// https://en.opensuse.org/SDB:Zypper_manual.
            #[prost(message, tag = "3")]
            ZypperPackage(super::VersionedPackage),
            /// Details of a Googet package.
            ///  For details about the googet package manager, see
            ///  https://github.com/google/googet.
            #[prost(message, tag = "4")]
            GoogetPackage(super::VersionedPackage),
            /// Details of a Zypper patch.
            /// For details about the Zypper package manager, see
            /// https://en.opensuse.org/SDB:Zypper_manual.
            #[prost(message, tag = "5")]
            ZypperPatch(super::ZypperPatch),
            /// Details of a Windows Update package.
            /// See https://docs.microsoft.com/en-us/windows/win32/api/_wua/ for
            /// information about Windows Update.
            #[prost(message, tag = "6")]
            WuaPackage(super::WindowsUpdatePackage),
            /// Details of a Windows Quick Fix engineering package.
            /// See
            /// https://docs.microsoft.com/en-us/windows/win32/cimwin32prov/win32-quickfixengineering
            /// for info in Windows Quick Fix Engineering.
            #[prost(message, tag = "7")]
            QfePackage(super::WindowsQuickFixEngineeringPackage),
            /// Details of a COS package.
            #[prost(message, tag = "8")]
            CosPackage(super::VersionedPackage),
        }
    }
    /// Information related to the a standard versioned package.  This includes
    /// package info for APT, Yum, Zypper, and Googet package managers.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct VersionedPackage {
        /// The name of the package.
        #[prost(string, tag = "4")]
        pub package_name: ::prost::alloc::string::String,
        /// The system architecture this package is intended for.
        #[prost(string, tag = "2")]
        pub architecture: ::prost::alloc::string::String,
        /// The version of the package.
        #[prost(string, tag = "3")]
        pub version: ::prost::alloc::string::String,
    }
    /// Details related to a Windows Update package.
    /// Field data and names are taken from Windows Update API IUpdate Interface:
    /// https://docs.microsoft.com/en-us/windows/win32/api/_wua/
    /// Descriptive fields like title, and description are localized based on
    /// the locale of the VM being updated.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct WindowsUpdatePackage {
        /// The localized title of the update package.
        #[prost(string, tag = "1")]
        pub title: ::prost::alloc::string::String,
        /// The localized description of the update package.
        #[prost(string, tag = "2")]
        pub description: ::prost::alloc::string::String,
        /// The categories that are associated with this update package.
        #[prost(message, repeated, tag = "3")]
        pub categories: ::prost::alloc::vec::Vec<windows_update_package::WindowsUpdateCategory>,
        /// A collection of Microsoft Knowledge Base article IDs that are associated
        /// with the update package.
        #[prost(string, repeated, tag = "4")]
        pub kb_article_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// A hyperlink to the language-specific support information for the update.
        #[prost(string, tag = "11")]
        pub support_url: ::prost::alloc::string::String,
        /// A collection of URLs that provide more information about the update
        /// package.
        #[prost(string, repeated, tag = "5")]
        pub more_info_urls: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// Gets the identifier of an update package.  Stays the same across
        /// revisions.
        #[prost(string, tag = "6")]
        pub update_id: ::prost::alloc::string::String,
        /// The revision number of this update package.
        #[prost(int32, tag = "7")]
        pub revision_number: i32,
        /// The last published date of the update, in (UTC) date and time.
        #[prost(message, optional, tag = "10")]
        pub last_deployment_change_time: ::core::option::Option<::prost_types::Timestamp>,
    }
    /// Nested message and enum types in `WindowsUpdatePackage`.
    pub mod windows_update_package {
        /// Categories specified by the Windows Update.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct WindowsUpdateCategory {
            /// The identifier of the windows update category.
            #[prost(string, tag = "1")]
            pub id: ::prost::alloc::string::String,
            /// The name of the windows update category.
            #[prost(string, tag = "2")]
            pub name: ::prost::alloc::string::String,
        }
    }
    /// Details related to a Zypper Patch.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ZypperPatch {
        /// The name of the patch.
        #[prost(string, tag = "5")]
        pub patch_name: ::prost::alloc::string::String,
        /// The category of the patch.
        #[prost(string, tag = "2")]
        pub category: ::prost::alloc::string::String,
        /// The severity specified for this patch
        #[prost(string, tag = "3")]
        pub severity: ::prost::alloc::string::String,
        /// Any summary information provided about this patch.
        #[prost(string, tag = "4")]
        pub summary: ::prost::alloc::string::String,
    }
    /// Information related to a Quick Fix Engineering package.
    /// Fields are taken from Windows QuickFixEngineering Interface and match
    /// the source names:
    /// https://docs.microsoft.com/en-us/windows/win32/cimwin32prov/win32-quickfixengineering
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct WindowsQuickFixEngineeringPackage {
        /// A short textual description of the QFE update.
        #[prost(string, tag = "1")]
        pub caption: ::prost::alloc::string::String,
        /// A textual description of the QFE update.
        #[prost(string, tag = "2")]
        pub description: ::prost::alloc::string::String,
        /// Unique identifier associated with a particular QFE update.
        #[prost(string, tag = "3")]
        pub hot_fix_id: ::prost::alloc::string::String,
        /// Date that the QFE update was installed.  Mapped from installed_on field.
        #[prost(message, optional, tag = "5")]
        pub install_time: ::core::option::Option<::prost_types::Timestamp>,
    }
}
/// Message encapsulating a value that can be either absolute ("fixed") or
/// relative ("percent") to a value.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FixedOrPercent {
    /// Type of the value.
    #[prost(oneof = "fixed_or_percent::Mode", tags = "1, 2")]
    pub mode: ::core::option::Option<fixed_or_percent::Mode>,
}
/// Nested message and enum types in `FixedOrPercent`.
pub mod fixed_or_percent {
    /// Type of the value.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Mode {
        /// Specifies a fixed value.
        #[prost(int32, tag = "1")]
        Fixed(i32),
        /// Specifies the relative value defined as a percentage, which will be
        /// multiplied by a reference value.
        #[prost(int32, tag = "2")]
        Percent(i32),
    }
}
/// A request message to initiate patching across Compute Engine
/// instances.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecutePatchJobRequest {
    /// Required. The project in which to run this patch in the form `projects/*`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Description of the patch job. Length of the description is limited
    /// to 1024 characters.
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// Required. Instances to patch, either explicitly or filtered by some
    /// criteria such as zone or labels.
    #[prost(message, optional, tag = "7")]
    pub instance_filter: ::core::option::Option<PatchInstanceFilter>,
    /// Patch configuration being applied. If omitted, instances are
    /// patched using the default configurations.
    #[prost(message, optional, tag = "4")]
    pub patch_config: ::core::option::Option<PatchConfig>,
    /// Duration of the patch job. After the duration ends, the patch job
    /// times out.
    #[prost(message, optional, tag = "5")]
    pub duration: ::core::option::Option<::prost_types::Duration>,
    /// If this patch is a dry-run only, instances are contacted but
    /// will do nothing.
    #[prost(bool, tag = "6")]
    pub dry_run: bool,
    /// Display name for this patch job. This does not have to be unique.
    #[prost(string, tag = "8")]
    pub display_name: ::prost::alloc::string::String,
    /// Rollout strategy of the patch job.
    #[prost(message, optional, tag = "9")]
    pub rollout: ::core::option::Option<PatchRollout>,
}
/// Request to get an active or completed patch job.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPatchJobRequest {
    /// Required. Name of the patch in the form `projects/*/patchJobs/*`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request to list details for all instances that are part of a patch job.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPatchJobInstanceDetailsRequest {
    /// Required. The parent for the instances are in the form of
    /// `projects/*/patchJobs/*`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of instance details records to return.  Default is 100.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A pagination token returned from a previous call
    /// that indicates where this listing should continue from.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// A filter expression that filters results listed in the response. This
    /// field supports filtering results by instance zone, name, state, or
    /// `failure_reason`.
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
}
/// A response message for listing the instances details for a patch job.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPatchJobInstanceDetailsResponse {
    /// A list of instance status.
    #[prost(message, repeated, tag = "1")]
    pub patch_job_instance_details: ::prost::alloc::vec::Vec<PatchJobInstanceDetails>,
    /// A pagination token that can be used to get the next page of results.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Patch details for a VM instance. For more information about reviewing VM
/// instance details, see
/// [Listing all VM instance details for a specific patch
/// job](https://cloud.google.com/compute/docs/os-patch-management/manage-patch-jobs#list-instance-details).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PatchJobInstanceDetails {
    /// The instance name in the form `projects/*/zones/*/instances/*`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The unique identifier for the instance. This identifier is
    /// defined by the server.
    #[prost(string, tag = "2")]
    pub instance_system_id: ::prost::alloc::string::String,
    /// Current state of instance patch.
    #[prost(enumeration = "instance::PatchState", tag = "3")]
    pub state: i32,
    /// If the patch fails, this field provides the reason.
    #[prost(string, tag = "4")]
    pub failure_reason: ::prost::alloc::string::String,
    /// The number of times the agent that the agent attempts to apply the patch.
    #[prost(int64, tag = "5")]
    pub attempt_count: i64,
}
/// A request message for listing patch jobs.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPatchJobsRequest {
    /// Required. In the form of `projects/*`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of instance status to return.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A pagination token returned from a previous call
    /// that indicates where this listing should continue from.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// If provided, this field specifies the criteria that must be met by patch
    /// jobs to be included in the response.
    /// Currently, filtering is only available on the patch_deployment field.
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
}
/// A response message for listing patch jobs.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPatchJobsResponse {
    /// The list of patch jobs.
    #[prost(message, repeated, tag = "1")]
    pub patch_jobs: ::prost::alloc::vec::Vec<PatchJob>,
    /// A pagination token that can be used to get the next page of results.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// A high level representation of a patch job that is either in progress
/// or has completed.
///
/// Instance details are not included in the job. To paginate through instance
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
    pub name: ::prost::alloc::string::String,
    /// Display name for this patch job. This is not a unique identifier.
    #[prost(string, tag = "14")]
    pub display_name: ::prost::alloc::string::String,
    /// Description of the patch job. Length of the description is limited
    /// to 1024 characters.
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// Time this patch job was created.
    #[prost(message, optional, tag = "3")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Last time this patch job was updated.
    #[prost(message, optional, tag = "4")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The current state of the PatchJob.
    #[prost(enumeration = "patch_job::State", tag = "5")]
    pub state: i32,
    /// Instances to patch.
    #[prost(message, optional, tag = "13")]
    pub instance_filter: ::core::option::Option<PatchInstanceFilter>,
    /// Patch configuration being applied.
    #[prost(message, optional, tag = "7")]
    pub patch_config: ::core::option::Option<PatchConfig>,
    /// Duration of the patch job. After the duration ends, the
    /// patch job times out.
    #[prost(message, optional, tag = "8")]
    pub duration: ::core::option::Option<::prost_types::Duration>,
    /// Summary of instance details.
    #[prost(message, optional, tag = "9")]
    pub instance_details_summary: ::core::option::Option<patch_job::InstanceDetailsSummary>,
    /// If this patch job is a dry run, the agent reports that it has
    /// finished without running any updates on the VM instance.
    #[prost(bool, tag = "10")]
    pub dry_run: bool,
    /// If this patch job failed, this message provides information about the
    /// failure.
    #[prost(string, tag = "11")]
    pub error_message: ::prost::alloc::string::String,
    /// Reflects the overall progress of the patch job in the range of
    /// 0.0 being no progress to 100.0 being complete.
    #[prost(double, tag = "12")]
    pub percent_complete: f64,
    /// Output only. Name of the patch deployment that created this patch job.
    #[prost(string, tag = "15")]
    pub patch_deployment: ::prost::alloc::string::String,
    /// Rollout strategy being applied.
    #[prost(message, optional, tag = "16")]
    pub rollout: ::core::option::Option<PatchRollout>,
}
/// Nested message and enum types in `PatchJob`.
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
    pub apt: ::core::option::Option<AptSettings>,
    /// Yum update settings. Use this setting to override the default `yum` patch
    /// rules.
    #[prost(message, optional, tag = "4")]
    pub yum: ::core::option::Option<YumSettings>,
    /// Goo update settings. Use this setting to override the default `goo` patch
    /// rules.
    #[prost(message, optional, tag = "5")]
    pub goo: ::core::option::Option<GooSettings>,
    /// Zypper update settings. Use this setting to override the default `zypper`
    /// patch rules.
    #[prost(message, optional, tag = "6")]
    pub zypper: ::core::option::Option<ZypperSettings>,
    /// Windows update settings. Use this override the default windows patch rules.
    #[prost(message, optional, tag = "7")]
    pub windows_update: ::core::option::Option<WindowsUpdateSettings>,
    /// The `ExecStep` to run before the patch update.
    #[prost(message, optional, tag = "8")]
    pub pre_step: ::core::option::Option<ExecStep>,
    /// The `ExecStep` to run after the patch update.
    #[prost(message, optional, tag = "9")]
    pub post_step: ::core::option::Option<ExecStep>,
}
/// Nested message and enum types in `PatchConfig`.
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
/// Nested message and enum types in `Instance`.
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
    pub name: ::prost::alloc::string::String,
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
    pub excludes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// An exclusive list of packages to be updated. These are the only packages
    /// that will be updated. If these packages are not installed, they will be
    /// ignored. This field cannot be specified with any other patch configuration
    /// fields.
    #[prost(string, repeated, tag = "3")]
    pub exclusive_packages: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Nested message and enum types in `AptSettings`.
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
    pub excludes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// An exclusive list of packages to be updated. These are the only packages
    /// that will be updated. If these packages are not installed, they will be
    /// ignored. This field must not be specified with any other patch
    /// configuration fields.
    #[prost(string, repeated, tag = "4")]
    pub exclusive_packages: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
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
    pub categories: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Install only patches with these severities.
    /// Common severities include critical, important, moderate, and low.
    #[prost(string, repeated, tag = "4")]
    pub severities: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// List of patches to exclude from update.
    #[prost(string, repeated, tag = "5")]
    pub excludes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// An exclusive list of patches to be updated. These are the only patches
    /// that will be installed using 'zypper patch patch:<patch_name>' command.
    /// This field must not be used with any other patch configuration fields.
    #[prost(string, repeated, tag = "6")]
    pub exclusive_patches: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
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
    pub classifications: ::prost::alloc::vec::Vec<i32>,
    /// List of KBs to exclude from update.
    #[prost(string, repeated, tag = "2")]
    pub excludes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// An exclusive list of kbs to be updated. These are the only patches
    /// that will be updated. This field must not be used with other
    /// patch configurations.
    #[prost(string, repeated, tag = "3")]
    pub exclusive_patches: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Nested message and enum types in `WindowsUpdateSettings`.
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
    pub linux_exec_step_config: ::core::option::Option<ExecStepConfig>,
    /// The ExecStepConfig for all Windows VMs targeted by the PatchJob.
    #[prost(message, optional, tag = "2")]
    pub windows_exec_step_config: ::core::option::Option<ExecStepConfig>,
}
/// Common configurations for an ExecStep.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecStepConfig {
    /// Defaults to [0]. A list of possible return values that the
    /// execution can return to indicate a success.
    #[prost(int32, repeated, tag = "3")]
    pub allowed_success_codes: ::prost::alloc::vec::Vec<i32>,
    /// The script interpreter to use to run the script. If no interpreter is
    /// specified the script will be executed directly, which will likely
    /// only succeed for scripts with [shebang lines]
    /// (https://en.wikipedia.org/wiki/Shebang_\(Unix\)).
    #[prost(enumeration = "exec_step_config::Interpreter", tag = "4")]
    pub interpreter: i32,
    /// Location of the executable.
    #[prost(oneof = "exec_step_config::Executable", tags = "1, 2")]
    pub executable: ::core::option::Option<exec_step_config::Executable>,
}
/// Nested message and enum types in `ExecStepConfig`.
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
        LocalPath(::prost::alloc::string::String),
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
    pub bucket: ::prost::alloc::string::String,
    /// Required. Name of the Cloud Storage object.
    #[prost(string, tag = "2")]
    pub object: ::prost::alloc::string::String,
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
    pub group_labels: ::prost::alloc::vec::Vec<patch_instance_filter::GroupLabel>,
    /// Targets VM instances in ANY of these zones. Leave empty to target VM
    /// instances in any zone.
    #[prost(string, repeated, tag = "3")]
    pub zones: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Targets any of the VM instances specified. Instances are specified by their
    /// URI in the form `zones/[ZONE]/instances/[INSTANCE_NAME]`,
    /// `projects/[PROJECT_ID]/zones/[ZONE]/instances/[INSTANCE_NAME]`, or
    /// `https://www.googleapis.com/compute/v1/projects/[PROJECT_ID]/zones/[ZONE]/instances/[INSTANCE_NAME]`
    #[prost(string, repeated, tag = "4")]
    pub instances: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Targets VMs whose name starts with one of these prefixes. Similar to
    /// labels, this is another way to group VMs when targeting configs, for
    /// example prefix="prod-".
    #[prost(string, repeated, tag = "5")]
    pub instance_name_prefixes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Nested message and enum types in `PatchInstanceFilter`.
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
        pub labels: ::std::collections::HashMap<
            ::prost::alloc::string::String,
            ::prost::alloc::string::String,
        >,
    }
}
/// Patch rollout configuration specifications. Contains details on the
/// concurrency control when applying patch(es) to all targeted VMs.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PatchRollout {
    /// Mode of the patch rollout.
    #[prost(enumeration = "patch_rollout::Mode", tag = "1")]
    pub mode: i32,
    /// The maximum number (or percentage) of VMs per zone to disrupt at any given
    /// moment. The number of VMs calculated from multiplying the percentage by the
    /// total number of VMs in a zone is rounded up.
    ///
    /// During patching, a VM is considered disrupted from the time the agent is
    /// notified to begin until patching has completed. This disruption time
    /// includes the time to complete reboot and any post-patch steps.
    ///
    /// A VM contributes to the disruption budget if its patching operation fails
    /// either when applying the patches, running pre or post patch steps, or if it
    /// fails to respond with a success notification before timing out. VMs that
    /// are not running or do not have an active agent do not count toward this
    /// disruption budget.
    ///
    /// For zone-by-zone rollouts, if the disruption budget in a zone is exceeded,
    /// the patch job stops, because continuing to the next zone requires
    /// completion of the patch process in the previous zone.
    ///
    /// For example, if the disruption budget has a fixed value of `10`, and 8 VMs
    /// fail to patch in the current zone, the patch job continues to patch 2 VMs
    /// at a time until the zone is completed. When that zone is completed
    /// successfully, patching begins with 10 VMs at a time in the next zone. If 10
    /// VMs in the next zone fail to patch, the patch job stops.
    #[prost(message, optional, tag = "2")]
    pub disruption_budget: ::core::option::Option<FixedOrPercent>,
}
/// Nested message and enum types in `PatchRollout`.
pub mod patch_rollout {
    /// Type of the rollout.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Mode {
        /// Mode must be specified.
        Unspecified = 0,
        /// Patches are applied one zone at a time. The patch job begins in the
        /// region with the lowest number of targeted VMs. Within the region,
        /// patching begins in the zone with the lowest number of targeted VMs. If
        /// multiple regions (or zones within a region) have the same number of
        /// targeted VMs, a tie-breaker is achieved by sorting the regions or zones
        /// in alphabetical order.
        ZoneByZone = 1,
        /// Patches are applied to VMs in all zones at the same time.
        ConcurrentZones = 2,
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
    pub name: ::prost::alloc::string::String,
    /// Optional. Description of the patch deployment. Length of the description is
    /// limited to 1024 characters.
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// Required. VM instances to patch.
    #[prost(message, optional, tag = "3")]
    pub instance_filter: ::core::option::Option<PatchInstanceFilter>,
    /// Optional. Patch configuration that is applied.
    #[prost(message, optional, tag = "4")]
    pub patch_config: ::core::option::Option<PatchConfig>,
    /// Optional. Duration of the patch. After the duration ends, the patch times
    /// out.
    #[prost(message, optional, tag = "5")]
    pub duration: ::core::option::Option<::prost_types::Duration>,
    /// Output only. Time the patch deployment was created. Timestamp is in
    /// [RFC3339](https://www.ietf.org/rfc/rfc3339.txt) text format.
    #[prost(message, optional, tag = "8")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Time the patch deployment was last updated. Timestamp is in
    /// [RFC3339](https://www.ietf.org/rfc/rfc3339.txt) text format.
    #[prost(message, optional, tag = "9")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The last time a patch job was started by this deployment.
    /// Timestamp is in [RFC3339](https://www.ietf.org/rfc/rfc3339.txt) text
    /// format.
    #[prost(message, optional, tag = "10")]
    pub last_execute_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Optional. Rollout strategy of the patch job.
    #[prost(message, optional, tag = "11")]
    pub rollout: ::core::option::Option<PatchRollout>,
    /// Schedule for the patch.
    #[prost(oneof = "patch_deployment::Schedule", tags = "6, 7")]
    pub schedule: ::core::option::Option<patch_deployment::Schedule>,
}
/// Nested message and enum types in `PatchDeployment`.
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
    pub execute_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Sets the time for recurring patch deployments.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RecurringSchedule {
    /// Required. Defines the time zone that `time_of_day` is relative to.
    /// The rules for daylight saving time are determined by the chosen time zone.
    #[prost(message, optional, tag = "1")]
    pub time_zone: ::core::option::Option<super::super::super::r#type::TimeZone>,
    /// Optional. The time that the recurring schedule becomes effective.
    /// Defaults to `create_time` of the patch deployment.
    #[prost(message, optional, tag = "2")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Optional. The end time at which a recurring patch deployment schedule is no
    /// longer active.
    #[prost(message, optional, tag = "3")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Required. Time of the day to run a recurring deployment.
    #[prost(message, optional, tag = "4")]
    pub time_of_day: ::core::option::Option<super::super::super::r#type::TimeOfDay>,
    /// Required. The frequency unit of this recurring schedule.
    #[prost(enumeration = "recurring_schedule::Frequency", tag = "5")]
    pub frequency: i32,
    /// Output only. The time the last patch job ran successfully.
    #[prost(message, optional, tag = "9")]
    pub last_execute_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The time the next patch job is scheduled to run.
    #[prost(message, optional, tag = "10")]
    pub next_execute_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Configurations for this recurring schedule.
    /// Configurations must match frequency.
    #[prost(oneof = "recurring_schedule::ScheduleConfig", tags = "6, 7")]
    pub schedule_config: ::core::option::Option<recurring_schedule::ScheduleConfig>,
}
/// Nested message and enum types in `RecurringSchedule`.
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
    pub day_of_month: ::core::option::Option<monthly_schedule::DayOfMonth>,
}
/// Nested message and enum types in `MonthlySchedule`.
pub mod monthly_schedule {
    /// One day in a month.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum DayOfMonth {
        /// Required. Week day in a month.
        #[prost(message, tag = "1")]
        WeekDayOfMonth(super::WeekDayOfMonth),
        /// Required. One day of the month. 1-31 indicates the 1st to the 31st day.
        /// -1 indicates the last day of the month. Months without the target day
        /// will be skipped. For example, a schedule to run "every month on the 31st"
        /// will not run in February, April, June, etc.
        #[prost(int32, tag = "2")]
        MonthDay(i32),
    }
}
/// Represents one week day in a month. An example is "the 4th Sunday".
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WeekDayOfMonth {
    /// Required. Week number in a month. 1-4 indicates the 1st to 4th week of the
    /// month. -1 indicates the last week of the month.
    #[prost(int32, tag = "1")]
    pub week_ordinal: i32,
    /// Required. A day of the week.
    #[prost(enumeration = "super::super::super::r#type::DayOfWeek", tag = "2")]
    pub day_of_week: i32,
}
/// A request message for creating a patch deployment.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreatePatchDeploymentRequest {
    /// Required. The project to apply this patch deployment to in the form
    /// `projects/*`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. A name for the patch deployment in the project. When creating a
    /// name the following rules apply:
    /// * Must contain only lowercase letters, numbers, and hyphens.
    /// * Must start with a letter.
    /// * Must be between 1-63 characters.
    /// * Must end with a number or a letter.
    /// * Must be unique within the project.
    #[prost(string, tag = "2")]
    pub patch_deployment_id: ::prost::alloc::string::String,
    /// Required. The patch deployment to create.
    #[prost(message, optional, tag = "3")]
    pub patch_deployment: ::core::option::Option<PatchDeployment>,
}
/// A request message for retrieving a patch deployment.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPatchDeploymentRequest {
    /// Required. The resource name of the patch deployment in the form
    /// `projects/*/patchDeployments/*`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// A request message for listing patch deployments.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPatchDeploymentsRequest {
    /// Required. The resource name of the parent in the form `projects/*`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. The maximum number of patch deployments to return. Default is
    /// 100.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. A pagination token returned from a previous call to
    /// ListPatchDeployments that indicates where this listing should continue
    /// from.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// A response message for listing patch deployments.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPatchDeploymentsResponse {
    /// The list of patch deployments.
    #[prost(message, repeated, tag = "1")]
    pub patch_deployments: ::prost::alloc::vec::Vec<PatchDeployment>,
    /// A pagination token that can be used to get the next page of patch
    /// deployments.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// A request message for deleting a patch deployment.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeletePatchDeploymentRequest {
    /// Required. The resource name of the patch deployment in the form
    /// `projects/*/patchDeployments/*`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
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
