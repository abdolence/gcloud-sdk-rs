// OS Config Inventory is a service for collecting and reporting operating
// system and package information on VM instances.

/// The inventory details of a VM.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Inventory {
    /// Base level operating system information for the VM.
    #[prost(message, optional, tag = "1")]
    pub os_info: ::std::option::Option<inventory::OsInfo>,
    /// A list of installed packages currently on the VM.
    #[prost(message, repeated, tag = "2")]
    pub installed_packages: ::std::vec::Vec<inventory::SoftwarePackage>,
    /// A list of software updates available for the VM as reported by the update
    /// managers.
    #[prost(message, repeated, tag = "3")]
    pub available_packages: ::std::vec::Vec<inventory::SoftwarePackage>,
}
pub mod inventory {
    /// Operating system information for the VM.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct OsInfo {
        /// The VM hostname.
        #[prost(string, tag = "1")]
        pub hostname: std::string::String,
        /// The operating system long name.
        /// For example 'Debian GNU/Linux 9' or 'Microsoft Window Server 2019
        /// Datacenter'.
        #[prost(string, tag = "2")]
        pub long_name: std::string::String,
        /// The operating system short name.
        /// For example, 'windows' or 'debian'.
        #[prost(string, tag = "3")]
        pub short_name: std::string::String,
        /// The version of the operating system.
        #[prost(string, tag = "4")]
        pub version: std::string::String,
        /// The system architecture of the operating system.
        #[prost(string, tag = "5")]
        pub architecture: std::string::String,
        /// The kernel version of the operating system.
        #[prost(string, tag = "6")]
        pub kernel_version: std::string::String,
        /// The kernel release of the operating system.
        #[prost(string, tag = "7")]
        pub kernel_release: std::string::String,
        /// The current version of the OS Config agent running on the VM.
        #[prost(string, tag = "8")]
        pub osconfig_agent_version: std::string::String,
    }
    /// Software package information of the operating system.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SoftwarePackage {
        /// Information about the different types of software packages.
        #[prost(oneof = "software_package::Details", tags = "1, 2, 3, 4, 5, 6, 7")]
        pub details: ::std::option::Option<software_package::Details>,
    }
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
        }
    }
    /// Information related to the a standard versioned package.  This includes
    /// package info for APT, Yum, Zypper, and Googet package managers.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct VersionedPackage {
        /// The name of the package.
        #[prost(string, tag = "1")]
        pub package_name: std::string::String,
        /// The system architecture this package is intended for.
        #[prost(string, tag = "2")]
        pub architecture: std::string::String,
        /// The version of the package.
        #[prost(string, tag = "3")]
        pub version: std::string::String,
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
        pub title: std::string::String,
        /// The localized description of the update package.
        #[prost(string, tag = "2")]
        pub description: std::string::String,
        /// The categories that are associated with this update package.
        #[prost(message, repeated, tag = "3")]
        pub categories: ::std::vec::Vec<windows_update_package::WindowsUpdateCategory>,
        /// A collection of Microsoft Knowledge Base article IDs that are associated
        /// with the update package.
        #[prost(string, repeated, tag = "4")]
        pub kb_article_ids: ::std::vec::Vec<std::string::String>,
        /// A hyperlink to the language-specific support information for the update.
        #[prost(string, tag = "5")]
        pub support_url: std::string::String,
        /// A collection of URLs that provide more information about the update
        /// package.
        #[prost(string, repeated, tag = "6")]
        pub more_info_urls: ::std::vec::Vec<std::string::String>,
        /// Gets the identifier of an update package.  Stays the same across
        /// revisions.
        #[prost(string, tag = "7")]
        pub update_id: std::string::String,
        /// The revision number of this update package.
        #[prost(int32, tag = "8")]
        pub revision_number: i32,
        /// The last published date of the update, in (UTC) date and time.
        #[prost(message, optional, tag = "9")]
        pub last_deployment_change_time: ::std::option::Option<::prost_types::Timestamp>,
    }
    pub mod windows_update_package {
        /// Categories specified by the Windows Update.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct WindowsUpdateCategory {
            /// The identifier of the windows update category.
            #[prost(string, tag = "1")]
            pub id: std::string::String,
            /// The name of the windows update category.
            #[prost(string, tag = "2")]
            pub name: std::string::String,
        }
    }
    /// Details related to a Zypper Patch.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ZypperPatch {
        /// The name of the patch.
        #[prost(string, tag = "1")]
        pub patch_name: std::string::String,
        /// The category of the patch.
        #[prost(string, tag = "2")]
        pub category: std::string::String,
        /// The severity specified for this patch
        #[prost(string, tag = "3")]
        pub severity: std::string::String,
        /// Any summary information provided about this patch.
        #[prost(string, tag = "4")]
        pub summary: std::string::String,
    }
    /// Information related to a Quick Fix Engineering package.
    /// Fields are taken from Windows QuickFixEngineering Interface and match
    /// the source names:
    /// https://docs.microsoft.com/en-us/windows/win32/cimwin32prov/win32-quickfixengineering
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct WindowsQuickFixEngineeringPackage {
        /// A short textual description of the QFE update.
        #[prost(string, tag = "1")]
        pub caption: std::string::String,
        /// A textual description of the QFE update.
        #[prost(string, tag = "2")]
        pub description: std::string::String,
        /// Unique identifier associated with a particular QFE update.
        #[prost(string, tag = "3")]
        pub hot_fix_id: std::string::String,
        /// Date that the QFE update was installed.  Mapped from installed_on field.
        #[prost(message, optional, tag = "4")]
        pub install_time: ::std::option::Option<::prost_types::Timestamp>,
    }
}
/// Patch configuration specifications. Contains details on how to
/// apply patches to a VM instance.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PatchConfig {
    /// Post-patch reboot settings.
    #[prost(enumeration = "patch_config::RebootConfig", tag = "1")]
    pub reboot_config: i32,
    /// Retry strategy can be defined to have the agent retry patching
    /// during the window if patching fails. If omitted, the agent will use its
    /// default retry strategy.
    #[prost(message, optional, tag = "2")]
    pub retry_strategy: ::std::option::Option<RetryStrategy>,
    /// Apt update settings. Use this override the default apt patch rules.
    #[prost(message, optional, tag = "3")]
    pub apt: ::std::option::Option<AptSettings>,
    /// Yum update settings. Use this override the default yum patch rules.
    #[prost(message, optional, tag = "4")]
    pub yum: ::std::option::Option<YumSettings>,
    /// Goo update settings. Use this override the default goo patch rules.
    #[prost(message, optional, tag = "5")]
    pub goo: ::std::option::Option<GooSettings>,
    /// Zypper update settings. Use this override the default zypper patch rules.
    #[prost(message, optional, tag = "6")]
    pub zypper: ::std::option::Option<ZypperSettings>,
    /// Windows update settings. Use this override the default windows patch rules.
    #[prost(message, optional, tag = "7")]
    pub windows_update: ::std::option::Option<WindowsUpdateSettings>,
    /// The ExecStep to run before the patch update.
    #[prost(message, optional, tag = "8")]
    pub pre_step: ::std::option::Option<ExecStep>,
    /// The ExecStep to run after the patch update.
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
        /// The agent decides if a reboot is necessary by checking
        /// signals such as registry keys on Windows or `/var/run/reboot-required` on
        /// APT based systems. On RPM based systems, a set of core system package
        /// install times are compared with system boot time.
        Default = 1,
        /// Always reboot the machine after the update completes.
        Always = 2,
        /// Never reboot the machine after the update completes.
        Never = 3,
    }
}
/// Apt patching will be performed by executing `apt-get update && apt-get
/// upgrade`. Additional options can be set to control how this is executed.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AptSettings {
    /// By changing the type to DIST, the patching will be performed
    /// using `apt-get dist-upgrade` instead.
    #[prost(enumeration = "apt_settings::Type", tag = "1")]
    pub r#type: i32,
    /// List of packages to exclude from update.
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
/// Yum patching will be performed by executing `yum update`. Additional options
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
    /// List of packages to exclude from update. These packages will be excluded by
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
    /// updates will be applied.
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
        /// to a product’s definition database. Definition databases are often used
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
/// The strategy for retrying failed patches during the patch window.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RetryStrategy {
    /// If true, the agent will continue to try and patch until the window has
    /// ended.
    #[prost(bool, tag = "1")]
    pub enabled: bool,
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
    /// only succeed for scripts with shebang lines.
    /// [Wikipedia shebang](https://en.wikipedia.org/wiki/Shebang_(Unix)).
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
        /// Indicates that the script will be run with /bin/sh on Linux and cmd
        /// on windows.
        Shell = 1,
        /// Indicates that the file will be run with PowerShell.
        Powershell = 2,
    }
    /// Location of the executable.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Executable {
        /// An absolute path to the executable on the VM.
        #[prost(string, tag = "1")]
        LocalPath(std::string::String),
        /// A GCS object containing the executable.
        #[prost(message, tag = "2")]
        GcsObject(super::GcsObject),
    }
}
/// GCS object representation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GcsObject {
    /// Bucket of the GCS object.
    #[prost(string, tag = "1")]
    pub bucket: std::string::String,
    /// Name of the GCS object.
    #[prost(string, tag = "2")]
    pub object: std::string::String,
    /// Generation number of the GCS object. This is used to ensure that the
    /// ExecStep specified by this PatchJob does not change.
    #[prost(int64, tag = "3")]
    pub generation_number: i64,
}
/// A unit of work to be performed by the agent.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Task {
    /// Unique task id.
    #[prost(string, tag = "1")]
    pub task_id: std::string::String,
    /// The type of task to perform.
    ///
    /// Task details must include the appropriate message based on this enum as
    /// specified below:
    /// APPLY_PATCHES = ApplyPatchesTask
    /// EXEC_STEP = ExecStepTask;
    #[prost(enumeration = "TaskType", tag = "2")]
    pub task_type: i32,
    /// Current directive to the agent.
    #[prost(enumeration = "TaskDirective", tag = "3")]
    pub task_directive: i32,
    /// Labels describing the task.  Used for logging by the agent.
    #[prost(map = "string, string", tag = "6")]
    pub service_labels: ::std::collections::HashMap<std::string::String, std::string::String>,
    /// Specific details about the current task to perform.
    #[prost(oneof = "task::TaskDetails", tags = "4, 5")]
    pub task_details: ::std::option::Option<task::TaskDetails>,
}
pub mod task {
    /// Specific details about the current task to perform.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum TaskDetails {
        /// Details about the apply patches task to perform.
        #[prost(message, tag = "4")]
        ApplyPatchesTask(super::ApplyPatchesTask),
        /// Details about the exec step task to perform.
        #[prost(message, tag = "5")]
        ExecStepTask(super::ExecStepTask),
    }
}
/// Message which instructs agent to apply patches.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApplyPatchesTask {
    /// Specific information about how patches should be applied.
    #[prost(message, optional, tag = "1")]
    pub patch_config: ::std::option::Option<PatchConfig>,
    /// If true, the agent will report its status as it goes through the motions
    /// but won't actually run any updates or perform any reboots.
    #[prost(bool, tag = "3")]
    pub dry_run: bool,
}
/// Information reported from the agent about applying patches execution.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApplyPatchesTaskProgress {
    /// Required. The current state of this patch execution.
    #[prost(enumeration = "apply_patches_task_progress::State", tag = "1")]
    pub state: i32,
}
pub mod apply_patches_task_progress {
    /// The intermediate states of applying patches.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// Unspecified is invalid.
        Unspecified = 0,
        /// The agent has started the patch task.
        Started = 4,
        /// The agent is currently downloading patches.
        DownloadingPatches = 1,
        /// The agent is currently applying patches.
        ApplyingPatches = 2,
        /// The agent is currently rebooting the VM instance.
        Rebooting = 3,
    }
}
/// Information reported from the agent about applying patches execution.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApplyPatchesTaskOutput {
    /// Required. The final state of this task.
    #[prost(enumeration = "apply_patches_task_output::State", tag = "1")]
    pub state: i32,
}
pub mod apply_patches_task_output {
    /// The final states of applying patches.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// Unspecified is invalid.
        Unspecified = 0,
        /// Applying patches completed successfully.
        Succeeded = 1,
        /// Applying patches completed successfully, but a reboot is required.
        SucceededRebootRequired = 2,
        /// Applying patches failed.
        Failed = 3,
    }
}
/// Message which instructs agent to execute the following command.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecStepTask {
    /// Details of the exec step to run.
    #[prost(message, optional, tag = "1")]
    pub exec_step: ::std::option::Option<ExecStep>,
}
/// Information reported from the agent about the exec step execution.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecStepTaskProgress {
    /// Required. The current state of this exec step.
    #[prost(enumeration = "exec_step_task_progress::State", tag = "1")]
    pub state: i32,
}
pub mod exec_step_task_progress {
    /// The intermediate states of exec steps.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// Unspecified is invalid.
        Unspecified = 0,
        /// The agent has started the exec step task.
        Started = 1,
    }
}
/// Information reported from the agent about the exec step execution.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecStepTaskOutput {
    /// Required. The final state of the exec step.
    #[prost(enumeration = "exec_step_task_output::State", tag = "1")]
    pub state: i32,
    /// Required. The exit code received from the script which ran as part of the exec step.
    #[prost(int32, tag = "2")]
    pub exit_code: i32,
}
pub mod exec_step_task_output {
    /// The final states of exec steps.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// Unspecified is invalid.
        Unspecified = 0,
        /// The exec step completed normally.
        Completed = 1,
        /// The exec step was terminated because it took too long.
        TimedOut = 2,
        /// The exec step task was cancelled before it started.
        Cancelled = 3,
    }
}
/// Specifies the current agent behavior.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TaskDirective {
    /// Unspecified is invalid.
    Unspecified = 0,
    /// The task should continue to progress.
    Continue = 1,
    /// Task should not be started, or if already in progress, should stop
    /// at first safe stopping point.  Task should be considered done and will
    /// never repeat.
    Stop = 2,
}
/// Specifies the type of task to perform.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TaskType {
    /// Unspecified is invalid.
    Unspecified = 0,
    /// The apply patches task.
    ApplyPatches = 1,
    /// The exec step task.
    ExecStepTask = 2,
}
/// A request message to receive task notifications.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReceiveTaskNotificationRequest {
    /// Required. This is the Compute Engine instance identity token described in
    /// https://cloud.google.com/compute/docs/instances/verifying-instance-identity
    /// where the audience is 'osconfig.googleapis.com' and the format is 'full'.
    #[prost(string, tag = "1")]
    pub instance_id_token: std::string::String,
    /// Required. The version of the agent making the request.
    #[prost(string, tag = "2")]
    pub agent_version: std::string::String,
}
/// The streaming rpc message that notifies the agent when it has a task
/// that it needs to perform on the VM instance.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReceiveTaskNotificationResponse {}
/// A request message for signaling the start of a task execution.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StartNextTaskRequest {
    /// Required. This is the Compute Engine instance identity token described in
    /// https://cloud.google.com/compute/docs/instances/verifying-instance-identity
    /// where the audience is 'osconfig.googleapis.com' and the format is 'full'.
    #[prost(string, tag = "1")]
    pub instance_id_token: std::string::String,
}
/// A response message that contains the details of the task to work on.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StartNextTaskResponse {
    /// The details of the task that should be worked on.  Can be empty if there
    /// is no new task to work on.
    #[prost(message, optional, tag = "1")]
    pub task: ::std::option::Option<Task>,
}
/// A request message for reporting the progress of current task.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReportTaskProgressRequest {
    /// Required. This is the Compute Engine instance identity token described in
    /// https://cloud.google.com/compute/docs/instances/verifying-instance-identity
    /// where the audience is 'osconfig.googleapis.com' and the format is 'full'.
    #[prost(string, tag = "1")]
    pub instance_id_token: std::string::String,
    /// Required. Unique identifier of the task this applies to.
    #[prost(string, tag = "2")]
    pub task_id: std::string::String,
    /// Required. The type of task to report progress on.
    ///
    /// Progress must include the appropriate message based on this enum as
    /// specified below:
    /// APPLY_PATCHES = ApplyPatchesTaskProgress
    /// EXEC_STEP = Progress not supported for this type.
    #[prost(enumeration = "TaskType", tag = "3")]
    pub task_type: i32,
    /// Intermediate progress of the current task.
    #[prost(oneof = "report_task_progress_request::Progress", tags = "4, 5")]
    pub progress: ::std::option::Option<report_task_progress_request::Progress>,
}
pub mod report_task_progress_request {
    /// Intermediate progress of the current task.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Progress {
        /// Details about the progress of the apply patches task.
        #[prost(message, tag = "4")]
        ApplyPatchesTaskProgress(super::ApplyPatchesTaskProgress),
        /// Details about the progress of the exec step task.
        #[prost(message, tag = "5")]
        ExecStepTaskProgress(super::ExecStepTaskProgress),
    }
}
/// The response message after the agent reported the current task progress.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReportTaskProgressResponse {
    /// Instructs agent to continue or not.
    #[prost(enumeration = "TaskDirective", tag = "1")]
    pub task_directive: i32,
}
/// A request message for signaling the completion of a task execution.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReportTaskCompleteRequest {
    /// Required. This is the Compute Engine instance identity token described in
    /// https://cloud.google.com/compute/docs/instances/verifying-instance-identity
    /// where the audience is 'osconfig.googleapis.com' and the format is 'full'.
    #[prost(string, tag = "1")]
    pub instance_id_token: std::string::String,
    /// Required. Unique identifier of the task this applies to.
    #[prost(string, tag = "2")]
    pub task_id: std::string::String,
    /// Required. The type of task to report completed.
    ///
    /// The output must include the appropriate message based on the following
    /// enum values:
    /// APPLY_PATCHES = ApplyPatchesTaskOutput
    /// EXEC_STEP = ExecStepTaskOutput
    #[prost(enumeration = "TaskType", tag = "3")]
    pub task_type: i32,
    /// Descriptive error message if the task execution ended in error.
    #[prost(string, tag = "4")]
    pub error_message: std::string::String,
    /// Final output details of the current task.
    #[prost(oneof = "report_task_complete_request::Output", tags = "5, 6")]
    pub output: ::std::option::Option<report_task_complete_request::Output>,
}
pub mod report_task_complete_request {
    /// Final output details of the current task.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Output {
        /// Final output details of the apply patches task;
        #[prost(message, tag = "5")]
        ApplyPatchesTaskOutput(super::ApplyPatchesTaskOutput),
        /// Final output details of the exec step task;
        #[prost(message, tag = "6")]
        ExecStepTaskOutput(super::ExecStepTaskOutput),
    }
}
/// The response message after the agent signaled the current task complete.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReportTaskCompleteResponse {}
/// The request message for registering the agent.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisterAgentRequest {
    /// Required. This is the Compute Engine instance identity token described in
    /// https://cloud.google.com/compute/docs/instances/verifying-instance-identity
    /// where the audience is 'osconfig.googleapis.com' and the format is 'full'.
    #[prost(string, tag = "1")]
    pub instance_id_token: std::string::String,
    /// Required. The version of the agent.
    #[prost(string, tag = "2")]
    pub agent_version: std::string::String,
    /// Required. The capabilities supported by the agent. Supported values are:
    /// PATCH_GA
    /// GUEST_POLICY_BETA
    #[prost(string, repeated, tag = "3")]
    pub supported_capabilities: ::std::vec::Vec<std::string::String>,
}
/// The response message after the agent registered.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisterAgentResponse {}
/// The request message for having the agent report inventory.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReportInventoryRequest {
    /// Required. This is the Compute Engine instance identity token described in
    /// https://cloud.google.com/compute/docs/instances/verifying-instance-identity
    /// where the audience is 'osconfig.googleapis.com' and the format is 'full'.
    #[prost(string, tag = "1")]
    pub instance_id_token: std::string::String,
    /// Required. This is a client created checksum that should be generated based on the
    /// contents of the reported inventory.  This will be used by the service to
    /// determine if it has the latest version of inventory.
    #[prost(string, tag = "2")]
    pub inventory_checksum: std::string::String,
    /// Optional. This is the details of the inventory.  Should only be provided if the
    /// inventory has changed since the last report, or if instructed by the
    /// service to provide full inventory.
    #[prost(message, optional, tag = "3")]
    pub inventory: ::std::option::Option<Inventory>,
}
/// The response message after the agent has reported inventory.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReportInventoryResponse {
    /// If true, the full inventory should be reported back to the server.
    #[prost(bool, tag = "1")]
    pub report_full_inventory: bool,
}
#[doc = r" Generated client implementations."]
pub mod agent_endpoint_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " OS Config agent endpoint API."]
    pub struct AgentEndpointServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> AgentEndpointServiceClient<T>
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
        #[doc = " Stream established by client to receive Task notifications."]
        pub async fn receive_task_notification(
            &mut self,
            request: impl tonic::IntoRequest<super::ReceiveTaskNotificationRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::ReceiveTaskNotificationResponse>>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ( "/google.cloud.osconfig.agentendpoint.v1.AgentEndpointService/ReceiveTaskNotification" ) ;
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        #[doc = " Signals the start of a task execution and returns the task info."]
        pub async fn start_next_task(
            &mut self,
            request: impl tonic::IntoRequest<super::StartNextTaskRequest>,
        ) -> Result<tonic::Response<super::StartNextTaskResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.osconfig.agentendpoint.v1.AgentEndpointService/StartNextTask",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Signals an intermediary progress checkpoint in task execution."]
        pub async fn report_task_progress(
            &mut self,
            request: impl tonic::IntoRequest<super::ReportTaskProgressRequest>,
        ) -> Result<tonic::Response<super::ReportTaskProgressResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.osconfig.agentendpoint.v1.AgentEndpointService/ReportTaskProgress",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Signals that the task execution is complete and optionally returns the next"]
        #[doc = " task."]
        pub async fn report_task_complete(
            &mut self,
            request: impl tonic::IntoRequest<super::ReportTaskCompleteRequest>,
        ) -> Result<tonic::Response<super::ReportTaskCompleteResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.osconfig.agentendpoint.v1.AgentEndpointService/ReportTaskComplete",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Registers the agent running on the VM."]
        pub async fn register_agent(
            &mut self,
            request: impl tonic::IntoRequest<super::RegisterAgentRequest>,
        ) -> Result<tonic::Response<super::RegisterAgentResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.osconfig.agentendpoint.v1.AgentEndpointService/RegisterAgent",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Reports the VMs current inventory."]
        pub async fn report_inventory(
            &mut self,
            request: impl tonic::IntoRequest<super::ReportInventoryRequest>,
        ) -> Result<tonic::Response<super::ReportInventoryResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.osconfig.agentendpoint.v1.AgentEndpointService/ReportInventory",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for AgentEndpointServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for AgentEndpointServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "AgentEndpointServiceClient {{ ... }}")
        }
    }
}
