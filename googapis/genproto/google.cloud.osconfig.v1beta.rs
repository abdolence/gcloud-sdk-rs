/// An OS Config resource representing a guest configuration policy. These
/// policies represent the desired state for VM instance guest environments
/// including packages to install or remove, package repository configurations,
/// and software to install.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GuestPolicy {
    /// Required. Unique name of the resource in this project using one of the following
    /// forms:
    /// `projects/{project_number}/guestPolicies/{guest_policy_id}`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Description of the guest policy. Length of the description is limited
    /// to 1024 characters.
    #[prost(string, tag = "2")]
    pub description: std::string::String,
    /// Output only. Time this guest policy was created.
    #[prost(message, optional, tag = "3")]
    pub create_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Output only. Last time this guest policy was updated.
    #[prost(message, optional, tag = "4")]
    pub update_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Required. Specifies the VM instances that are assigned to this policy. This allows
    /// you to target sets or groups of VM instances by different parameters such
    /// as labels, names, OS, or zones.
    ///
    /// If left empty, all VM instances underneath this policy are targeted.
    ///
    /// At the same level in the resource hierarchy (that is within a project), the
    /// service prevents the creation of multiple policies that conflict with
    /// each other. For more information, see how the service [handles assignment
    /// conflicts](/compute/docs/os-config-management/create-guest-policy#handle-conflicts).
    #[prost(message, optional, tag = "6")]
    pub assignment: ::std::option::Option<Assignment>,
    /// The software packages to be managed by this policy.
    #[prost(message, repeated, tag = "7")]
    pub packages: ::std::vec::Vec<Package>,
    /// A list of package repositories to configure on the VM instance. This is
    /// done before any other configs are applied so they can use these repos.
    /// Package repositories are only configured if the corresponding package
    /// manager(s) are available.
    #[prost(message, repeated, tag = "8")]
    pub package_repositories: ::std::vec::Vec<PackageRepository>,
    /// A list of Recipes to install on the VM instance.
    #[prost(message, repeated, tag = "9")]
    pub recipes: ::std::vec::Vec<SoftwareRecipe>,
    /// The etag for this guest policy.
    /// If this is provided on update, it must match the server's etag.
    #[prost(string, tag = "10")]
    pub etag: std::string::String,
}
/// An assignment represents the group or groups of VM instances that the policy
/// applies to.
///
/// If an assignment is empty, it applies to all VM instances. Otherwise, the
/// targeted VM instances must meet all the criteria specified. So if both
/// labels and zones are specified, the policy applies to VM instances with those
/// labels and in those zones.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Assignment {
    /// Targets instances matching at least one of these label sets. This allows
    /// an assignment to target disparate groups, for example "env=prod or
    /// env=staging".
    #[prost(message, repeated, tag = "1")]
    pub group_labels: ::std::vec::Vec<assignment::GroupLabel>,
    /// Targets instances in any of these zones. Leave empty to target instances
    /// in any zone.
    ///
    /// Zonal targeting is uncommon and is supported to facilitate the management
    /// of changes by zone.
    #[prost(string, repeated, tag = "2")]
    pub zones: ::std::vec::Vec<std::string::String>,
    /// Targets any of the instances specified. Instances are specified by their
    /// URI in the form `zones/[ZONE]/instances/[INSTANCE_NAME]`.
    ///
    /// Instance targeting is uncommon and is supported to facilitate the
    /// management of changes by the instance or to target specific VM instances
    /// for development and testing.
    ///
    /// Only supported for project-level policies and must reference instances
    /// within this project.
    #[prost(string, repeated, tag = "3")]
    pub instances: ::std::vec::Vec<std::string::String>,
    /// Targets VM instances whose name starts with one of these prefixes.
    ///
    /// Like labels, this is another way to group VM instances when targeting
    /// configs, for example prefix="prod-".
    ///
    /// Only supported for project-level policies.
    #[prost(string, repeated, tag = "4")]
    pub instance_name_prefixes: ::std::vec::Vec<std::string::String>,
    /// Targets VM instances matching at least one of the following OS types.
    ///
    /// VM instances must match all supplied criteria for a given OsType to be
    /// included.
    #[prost(message, repeated, tag = "5")]
    pub os_types: ::std::vec::Vec<assignment::OsType>,
}
pub mod assignment {
    /// Represents a group of VM intances that can be identified as having all
    /// these labels, for example "env=prod and app=web".
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GroupLabel {
        /// Google Compute Engine instance labels that must be present for an
        /// instance to be included in this assignment group.
        #[prost(map = "string, string", tag = "1")]
        pub labels: ::std::collections::HashMap<std::string::String, std::string::String>,
    }
    /// Defines the criteria for selecting VM Instances by OS type.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct OsType {
        /// Targets VM instances with OS Inventory enabled and having the following
        /// OS short name, for example "debian" or "windows".
        #[prost(string, tag = "1")]
        pub os_short_name: std::string::String,
        /// Targets VM instances with OS Inventory enabled and having the following
        /// following OS version.
        #[prost(string, tag = "2")]
        pub os_version: std::string::String,
        /// Targets VM instances with OS Inventory enabled and having the following
        /// OS architecture.
        #[prost(string, tag = "3")]
        pub os_architecture: std::string::String,
    }
}
/// Package is a reference to the software package to be installed or removed.
/// The agent on the VM instance uses the system package manager to apply the
/// config.
///
///
/// These are the commands that the agent uses to install or remove
/// packages.
///
/// Apt
/// install: `apt-get update && apt-get -y install package1 package2 package3`
/// remove: `apt-get -y remove package1 package2 package3`
///
/// Yum
/// install: `yum -y install package1 package2 package3`
/// remove: `yum -y remove package1 package2 package3`
///
/// Zypper
/// install: `zypper install package1 package2 package3`
/// remove: `zypper rm package1 package2`
///
/// Googet
/// install: `googet -noconfirm install package1 package2 package3`
/// remove: `googet -noconfirm remove package1 package2 package3`
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Package {
    /// Required. The name of the package. A package is uniquely identified for conflict
    /// validation by checking the package name and the manager(s) that the
    /// package targets.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// The desired_state the agent should maintain for this package. The
    /// default is to ensure the package is installed.
    #[prost(enumeration = "DesiredState", tag = "2")]
    pub desired_state: i32,
    /// Type of package manager that can be used to install this package.
    /// If a system does not have the package manager, the package is not
    /// installed or removed no error message is returned. By default,
    /// or if you specify `ANY`,
    /// the agent attempts to install and remove this package using the default
    /// package manager. This is useful when creating a policy that applies to
    /// different types of systems.
    ///
    /// The default behavior is ANY.
    #[prost(enumeration = "package::Manager", tag = "3")]
    pub manager: i32,
}
pub mod package {
    /// Types of package managers that may be used to manage this package.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Manager {
        /// The default behavior is ANY.
        Unspecified = 0,
        /// Apply this package config using the default system package manager.
        Any = 1,
        /// Apply this package config only if Apt is available on the system.
        Apt = 2,
        /// Apply this package config only if Yum is available on the system.
        Yum = 3,
        /// Apply this package config only if Zypper is available on the system.
        Zypper = 4,
        /// Apply this package config only if GooGet is available on the system.
        Goo = 5,
    }
}
/// Represents a single Apt package repository. This repository is added to
/// a repo file that is stored at
/// `/etc/apt/sources.list.d/google_osconfig.list`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AptRepository {
    /// Type of archive files in this repository. The default behavior is DEB.
    #[prost(enumeration = "apt_repository::ArchiveType", tag = "1")]
    pub archive_type: i32,
    /// Required. URI for this repository.
    #[prost(string, tag = "2")]
    pub uri: std::string::String,
    /// Required. Distribution of this repository.
    #[prost(string, tag = "3")]
    pub distribution: std::string::String,
    /// Required. List of components for this repository. Must contain at least one item.
    #[prost(string, repeated, tag = "4")]
    pub components: ::std::vec::Vec<std::string::String>,
    /// URI of the key file for this repository. The agent maintains
    /// a keyring at `/etc/apt/trusted.gpg.d/osconfig_agent_managed.gpg` containing
    /// all the keys in any applied guest policy.
    #[prost(string, tag = "5")]
    pub gpg_key: std::string::String,
}
pub mod apt_repository {
    /// Type of archive.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ArchiveType {
        /// Unspecified.
        Unspecified = 0,
        /// DEB indicates that the archive contains binary files.
        Deb = 1,
        /// DEB_SRC indicates that the archive contains source files.
        DebSrc = 2,
    }
}
/// Represents a single Yum package repository. This repository is added to a
/// repo file that is stored at `/etc/yum.repos.d/google_osconfig.repo`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct YumRepository {
    /// Required. A one word, unique name for this repository. This is
    /// the `repo id` in the Yum config file and also the `display_name` if
    /// `display_name` is omitted. This id is also used as the unique identifier
    /// when checking for guest policy conflicts.
    #[prost(string, tag = "1")]
    pub id: std::string::String,
    /// The display name of the repository.
    #[prost(string, tag = "2")]
    pub display_name: std::string::String,
    /// Required. The location of the repository directory.
    #[prost(string, tag = "3")]
    pub base_url: std::string::String,
    /// URIs of GPG keys.
    #[prost(string, repeated, tag = "4")]
    pub gpg_keys: ::std::vec::Vec<std::string::String>,
}
/// Represents a single Zypper package repository. This repository is added to a
/// repo file that is stored at `/etc/zypp/repos.d/google_osconfig.repo`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ZypperRepository {
    /// Required. A one word, unique name for this repository. This is
    /// the `repo id` in the zypper config file and also the `display_name` if
    /// `display_name` is omitted. This id is also used as the unique identifier
    /// when checking for guest policy conflicts.
    #[prost(string, tag = "1")]
    pub id: std::string::String,
    /// The display name of the repository.
    #[prost(string, tag = "2")]
    pub display_name: std::string::String,
    /// Required. The location of the repository directory.
    #[prost(string, tag = "3")]
    pub base_url: std::string::String,
    /// URIs of GPG keys.
    #[prost(string, repeated, tag = "4")]
    pub gpg_keys: ::std::vec::Vec<std::string::String>,
}
/// Represents a Goo package repository. These is added to a repo file
/// that is stored at C:/ProgramData/GooGet/repos/google_osconfig.repo.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GooRepository {
    /// Required. The name of the repository.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Required. The url of the repository.
    #[prost(string, tag = "2")]
    pub url: std::string::String,
}
/// A package repository.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PackageRepository {
    /// A specific type of repository.
    #[prost(oneof = "package_repository::Repository", tags = "1, 2, 3, 4")]
    pub repository: ::std::option::Option<package_repository::Repository>,
}
pub mod package_repository {
    /// A specific type of repository.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Repository {
        /// An Apt Repository.
        #[prost(message, tag = "1")]
        Apt(super::AptRepository),
        /// A Yum Repository.
        #[prost(message, tag = "2")]
        Yum(super::YumRepository),
        /// A Zypper Repository.
        #[prost(message, tag = "3")]
        Zypper(super::ZypperRepository),
        /// A Goo Repository.
        #[prost(message, tag = "4")]
        Goo(super::GooRepository),
    }
}
/// A software recipe is a set of instructions for installing and configuring a
/// piece of software. It consists of a set of artifacts that are
/// downloaded, and a set of steps that install, configure, and/or update the
/// software.
///
/// Recipes support installing and updating software from artifacts in the
/// following formats:
/// Zip archive, Tar archive, Windows MSI, Debian package, and RPM package.
///
/// Additionally, recipes support executing a script (either defined in a file or
/// directly in this api) in bash, sh, cmd, and powershell.
///
/// Updating a software recipe
///
/// If a recipe is assigned to an instance and there is a recipe with the same
/// name but a lower version already installed and the assigned state
/// of the recipe is `INSTALLED_KEEP_UPDATED`, then the recipe is updated to
/// the new version.
///
/// Script Working Directories
///
/// Each script or execution step is run in its own temporary directory which
/// is deleted after completing the step.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SoftwareRecipe {
    /// Required. Unique identifier for the recipe. Only one recipe with a given name is
    /// installed on an instance.
    ///
    /// Names are also used to identify resources which helps to determine whether
    /// guest policies have conflicts. This means that requests to create multiple
    /// recipes with the same name and version are rejected since they
    /// could potentially have conflicting assignments.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// The version of this software recipe. Version can be up to 4 period
    /// separated numbers (e.g. 12.34.56.78).
    #[prost(string, tag = "2")]
    pub version: std::string::String,
    /// Resources available to be used in the steps in the recipe.
    #[prost(message, repeated, tag = "3")]
    pub artifacts: ::std::vec::Vec<software_recipe::Artifact>,
    /// Actions to be taken for installing this recipe. On failure it stops
    /// executing steps and does not attempt another installation. Any steps taken
    /// (including partially completed steps) are not rolled back.
    #[prost(message, repeated, tag = "4")]
    pub install_steps: ::std::vec::Vec<software_recipe::Step>,
    /// Actions to be taken for updating this recipe. On failure it stops
    /// executing steps and  does not attempt another update for this recipe. Any
    /// steps taken (including partially completed steps) are not rolled back.
    #[prost(message, repeated, tag = "5")]
    pub update_steps: ::std::vec::Vec<software_recipe::Step>,
    /// Default is INSTALLED. The desired state the agent should maintain for this
    /// recipe.
    ///
    /// INSTALLED: The software recipe is installed on the instance but
    ///            won't be updated to new versions.
    /// INSTALLED_KEEP_UPDATED: The software recipe is installed on the
    ///                         instance. The recipe is updated to a higher
    ///                         version, if a higher version of the recipe is
    ///                         assigned to this instance.
    /// REMOVE: Remove is unsupported for software recipes and attempts to
    ///         create or update a recipe to the REMOVE state is rejected.
    #[prost(enumeration = "DesiredState", tag = "6")]
    pub desired_state: i32,
}
pub mod software_recipe {
    /// Specifies a resource to be used in the recipe.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Artifact {
        /// Required. Id of the artifact, which the installation and update steps of this
        /// recipe can reference. Artifacts in a recipe cannot have the same id.
        #[prost(string, tag = "1")]
        pub id: std::string::String,
        /// Defaults to false. When false, recipes are subject to validations
        /// based on the artifact type:
        ///
        /// Remote: A checksum must be specified, and only protocols with
        /// transport-layer security are permitted.
        /// GCS:    An object generation number must be specified.
        #[prost(bool, tag = "4")]
        pub allow_insecure: bool,
        /// A specific type of artifact.
        #[prost(oneof = "artifact::Artifact", tags = "2, 3")]
        pub artifact: ::std::option::Option<artifact::Artifact>,
    }
    pub mod artifact {
        /// Specifies an artifact available via some URI.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Remote {
            /// URI from which to fetch the object. It should contain both the protocol
            /// and path following the format {protocol}://{location}.
            #[prost(string, tag = "1")]
            pub uri: std::string::String,
            /// Must be provided if `allow_insecure` is `false`.
            /// SHA256 checksum in hex format, to compare to the checksum of the
            /// artifact. If the checksum is not empty and it doesn't match the
            /// artifact then the recipe installation fails before running any of the
            /// steps.
            #[prost(string, tag = "2")]
            pub checksum: std::string::String,
        }
        /// Specifies an artifact available as a Google Cloud Storage object.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Gcs {
            /// Bucket of the Google Cloud Storage object.
            /// Given an example URL:
            /// `https://storage.googleapis.com/my-bucket/foo/bar#1234567`
            /// this value would be `my-bucket`.
            #[prost(string, tag = "1")]
            pub bucket: std::string::String,
            /// Name of the Google Cloud Storage object.
            /// As specified [here]
            /// (https://cloud.google.com/storage/docs/naming#objectnames)
            /// Given an example URL:
            /// `https://storage.googleapis.com/my-bucket/foo/bar#1234567`
            /// this value would be `foo/bar`.
            #[prost(string, tag = "2")]
            pub object: std::string::String,
            /// Must be provided if allow_insecure is false.
            /// Generation number of the Google Cloud Storage object.
            /// `https://storage.googleapis.com/my-bucket/foo/bar#1234567`
            /// this value would be `1234567`.
            #[prost(int64, tag = "3")]
            pub generation: i64,
        }
        /// A specific type of artifact.
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Artifact {
            /// A generic remote artifact.
            #[prost(message, tag = "2")]
            Remote(Remote),
            /// A Google Cloud Storage artifact.
            #[prost(message, tag = "3")]
            Gcs(Gcs),
        }
    }
    /// An action that can be taken as part of installing or updating a recipe.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Step {
        /// A specific type of step.
        #[prost(oneof = "step::Step", tags = "1, 2, 3, 4, 5, 6, 7")]
        pub step: ::std::option::Option<step::Step>,
    }
    pub mod step {
        /// Copies the artifact to the specified path on the instance.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct CopyFile {
            /// Required. The id of the relevant artifact in the recipe.
            #[prost(string, tag = "1")]
            pub artifact_id: std::string::String,
            /// Required. The absolute path on the instance to put the file.
            #[prost(string, tag = "2")]
            pub destination: std::string::String,
            /// Whether to allow this step to overwrite existing files. If this is
            /// false and the file already exists the file is not overwritten
            /// and the step is considered a success. Defaults to false.
            #[prost(bool, tag = "3")]
            pub overwrite: bool,
            /// Consists of three octal digits which represent, in
            /// order, the permissions of the owner, group, and other users for the
            /// file (similarly to the numeric mode used in the linux chmod utility).
            /// Each digit represents a three bit number with the 4 bit
            /// corresponding to the read permissions, the 2 bit corresponds to the
            /// write bit, and the one bit corresponds to the execute permission.
            /// Default behavior is 755.
            ///
            /// Below are some examples of permissions and their associated values:
            /// read, write, and execute: 7
            /// read and execute: 5
            /// read and write: 6
            /// read only: 4
            #[prost(string, tag = "4")]
            pub permissions: std::string::String,
        }
        /// Extracts an archive of the type specified in the specified directory.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ExtractArchive {
            /// Required. The id of the relevant artifact in the recipe.
            #[prost(string, tag = "1")]
            pub artifact_id: std::string::String,
            /// Directory to extract archive to.
            /// Defaults to `/` on Linux or `C:\` on Windows.
            #[prost(string, tag = "2")]
            pub destination: std::string::String,
            /// Required. The type of the archive to extract.
            #[prost(enumeration = "extract_archive::ArchiveType", tag = "3")]
            pub r#type: i32,
        }
        pub mod extract_archive {
            /// Specifying the type of archive.
            #[derive(
                Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration,
            )]
            #[repr(i32)]
            pub enum ArchiveType {
                /// Indicates that the archive type isn't specified.
                Unspecified = 0,
                /// Indicates that the archive is a tar archive with no encryption.
                Tar = 1,
                /// Indicates that the archive is a tar archive with gzip encryption.
                TarGzip = 2,
                /// Indicates that the archive is a tar archive with bzip encryption.
                TarBzip = 3,
                /// Indicates that the archive is a tar archive with lzma encryption.
                TarLzma = 4,
                /// Indicates that the archive is a tar archive with xz encryption.
                TarXz = 5,
                /// Indicates that the archive is a zip archive.
                Zip = 11,
            }
        }
        /// Installs an MSI file.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct InstallMsi {
            /// Required. The id of the relevant artifact in the recipe.
            #[prost(string, tag = "1")]
            pub artifact_id: std::string::String,
            /// The flags to use when installing the MSI
            /// defaults to ["/i"] (i.e. the install flag).
            #[prost(string, repeated, tag = "2")]
            pub flags: ::std::vec::Vec<std::string::String>,
            /// Return codes that indicate that the software installed or updated
            /// successfully. Behaviour defaults to [0]
            #[prost(int32, repeated, tag = "3")]
            pub allowed_exit_codes: ::std::vec::Vec<i32>,
        }
        /// Installs a deb via dpkg.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct InstallDpkg {
            /// Required. The id of the relevant artifact in the recipe.
            #[prost(string, tag = "1")]
            pub artifact_id: std::string::String,
        }
        /// Installs an rpm file via the rpm utility.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct InstallRpm {
            /// Required. The id of the relevant artifact in the recipe.
            #[prost(string, tag = "1")]
            pub artifact_id: std::string::String,
        }
        /// Executes an artifact or local file.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ExecFile {
            /// Arguments to be passed to the provided executable.
            #[prost(string, repeated, tag = "3")]
            pub args: ::std::vec::Vec<std::string::String>,
            /// Defaults to [0]. A list of possible return values that the program
            /// can return to indicate a success.
            #[prost(int32, repeated, tag = "4")]
            pub allowed_exit_codes: ::std::vec::Vec<i32>,
            /// Location of the file to execute.
            #[prost(oneof = "exec_file::LocationType", tags = "1, 2")]
            pub location_type: ::std::option::Option<exec_file::LocationType>,
        }
        pub mod exec_file {
            /// Location of the file to execute.
            #[derive(Clone, PartialEq, ::prost::Oneof)]
            pub enum LocationType {
                /// The id of the relevant artifact in the recipe.
                #[prost(string, tag = "1")]
                ArtifactId(std::string::String),
                /// The absolute path of the file on the local filesystem.
                #[prost(string, tag = "2")]
                LocalPath(std::string::String),
            }
        }
        /// Runs a script through an interpreter.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct RunScript {
            /// Required. The shell script to be executed.
            #[prost(string, tag = "1")]
            pub script: std::string::String,
            /// Return codes that indicate that the software installed or updated
            /// successfully. Behaviour defaults to [0]
            #[prost(int32, repeated, tag = "2")]
            pub allowed_exit_codes: ::std::vec::Vec<i32>,
            /// The script interpreter to use to run the script. If no interpreter is
            /// specified the script is executed directly, which likely
            /// only succeed for scripts with
            /// [shebang lines](https://en.wikipedia.org/wiki/Shebang_\(Unix\)).
            #[prost(enumeration = "run_script::Interpreter", tag = "3")]
            pub interpreter: i32,
        }
        pub mod run_script {
            /// The interpreter used to execute a script.
            #[derive(
                Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration,
            )]
            #[repr(i32)]
            pub enum Interpreter {
                /// Default value for ScriptType.
                Unspecified = 0,
                /// Indicates that the script is run with `/bin/sh` on Linux and `cmd`
                /// on windows.
                Shell = 1,
                /// Indicates that the script is run with powershell.
                Powershell = 3,
            }
        }
        /// A specific type of step.
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Step {
            /// Copies a file onto the instance.
            #[prost(message, tag = "1")]
            FileCopy(CopyFile),
            /// Extracts an archive into the specified directory.
            #[prost(message, tag = "2")]
            ArchiveExtraction(ExtractArchive),
            /// Installs an MSI file.
            #[prost(message, tag = "3")]
            MsiInstallation(InstallMsi),
            /// Installs a deb file via dpkg.
            #[prost(message, tag = "4")]
            DpkgInstallation(InstallDpkg),
            /// Installs an rpm file via the rpm utility.
            #[prost(message, tag = "5")]
            RpmInstallation(InstallRpm),
            /// Executes an artifact or local file.
            #[prost(message, tag = "6")]
            FileExec(ExecFile),
            /// Runs commands in a shell.
            #[prost(message, tag = "7")]
            ScriptRun(RunScript),
        }
    }
}
/// A request message for creating a guest policy.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateGuestPolicyRequest {
    /// Required. The resource name of the parent using one of the following forms:
    /// `projects/{project_number}`.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Required. The logical name of the guest policy in the project
    /// with the following restrictions:
    ///
    /// * Must contain only lowercase letters, numbers, and hyphens.
    /// * Must start with a letter.
    /// * Must be between 1-63 characters.
    /// * Must end with a number or a letter.
    /// * Must be unique within the project.
    #[prost(string, tag = "2")]
    pub guest_policy_id: std::string::String,
    /// Required. The GuestPolicy to create.
    #[prost(message, optional, tag = "3")]
    pub guest_policy: ::std::option::Option<GuestPolicy>,
}
/// A request message for retrieving a guest policy.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetGuestPolicyRequest {
    /// Required. The resource name of the guest policy using one of the following forms:
    /// `projects/{project_number}/guestPolicies/{guest_policy_id}`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// A request message for listing guest policies.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListGuestPoliciesRequest {
    /// Required. The resource name of the parent using one of the following forms:
    /// `projects/{project_number}`.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// The maximum number of guest policies to return.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A pagination token returned from a previous call to `ListGuestPolicies`
    /// that indicates where this listing should continue from.
    #[prost(string, tag = "3")]
    pub page_token: std::string::String,
}
/// A response message for listing guest policies.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListGuestPoliciesResponse {
    /// The list of GuestPolicies.
    #[prost(message, repeated, tag = "1")]
    pub guest_policies: ::std::vec::Vec<GuestPolicy>,
    /// A pagination token that can be used to get the next page
    /// of guest policies.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
}
/// A request message for updating a guest policy.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateGuestPolicyRequest {
    /// Required. The updated GuestPolicy.
    #[prost(message, optional, tag = "1")]
    pub guest_policy: ::std::option::Option<GuestPolicy>,
    /// Field mask that controls which fields of the guest policy should be
    /// updated.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
}
/// A request message for deleting a guest policy.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteGuestPolicyRequest {
    /// Required. The resource name of the guest policy  using one of the following forms:
    /// `projects/{project_number}/guestPolicies/{guest_policy_id}`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// A request message for getting the effective guest policy assigned to the
/// instance.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LookupEffectiveGuestPolicyRequest {
    /// Required. The VM instance whose policies are being looked up.
    #[prost(string, tag = "1")]
    pub instance: std::string::String,
    /// Short name of the OS running on the instance. The OS Config agent only
    /// provides this field for targeting if OS Inventory is enabled for that
    /// instance.
    #[prost(string, tag = "2")]
    pub os_short_name: std::string::String,
    /// Version of the OS running on the instance. The OS Config agent only
    /// provides this field for targeting if OS Inventory is enabled for that
    /// VM instance.
    #[prost(string, tag = "3")]
    pub os_version: std::string::String,
    /// Architecture of OS running on the instance. The OS Config agent only
    /// provides this field for targeting if OS Inventory is enabled for that
    /// instance.
    #[prost(string, tag = "4")]
    pub os_architecture: std::string::String,
}
/// The effective guest policy that applies to a VM instance.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EffectiveGuestPolicy {
    /// List of package configurations assigned to the VM instance.
    #[prost(message, repeated, tag = "1")]
    pub packages: ::std::vec::Vec<effective_guest_policy::SourcedPackage>,
    /// List of package repository configurations assigned to the VM instance.
    #[prost(message, repeated, tag = "2")]
    pub package_repositories: ::std::vec::Vec<effective_guest_policy::SourcedPackageRepository>,
    /// List of recipes assigned to the VM instance.
    #[prost(message, repeated, tag = "3")]
    pub software_recipes: ::std::vec::Vec<effective_guest_policy::SourcedSoftwareRecipe>,
}
pub mod effective_guest_policy {
    /// A guest policy package including its source.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SourcedPackage {
        /// Name of the guest policy providing this config.
        #[prost(string, tag = "1")]
        pub source: std::string::String,
        /// A software package to configure on the VM instance.
        #[prost(message, optional, tag = "2")]
        pub package: ::std::option::Option<super::Package>,
    }
    /// A guest policy package repository including its source.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SourcedPackageRepository {
        /// Name of the guest policy providing this config.
        #[prost(string, tag = "1")]
        pub source: std::string::String,
        /// A software package repository to configure on the VM instance.
        #[prost(message, optional, tag = "2")]
        pub package_repository: ::std::option::Option<super::PackageRepository>,
    }
    /// A guest policy recipe including its source.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SourcedSoftwareRecipe {
        /// Name of the guest policy providing this config.
        #[prost(string, tag = "1")]
        pub source: std::string::String,
        /// A software recipe to configure on the VM instance.
        #[prost(message, optional, tag = "2")]
        pub software_recipe: ::std::option::Option<super::SoftwareRecipe>,
    }
}
/// The desired state that the OS Config agent maintains on the VM instance.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DesiredState {
    /// The default is to ensure the package is installed.
    Unspecified = 0,
    /// The agent ensures that the package is installed.
    Installed = 1,
    /// The agent ensures that the package is installed and
    /// periodically checks for and install any updates.
    Updated = 2,
    /// The agent ensures that the package is not installed and uninstall it
    /// if detected.
    Removed = 3,
}
/// A request message to initiate patching across Google Compute Engine
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
/// job](/compute/docs/os-patch-management/manage-patch-jobs#list-instance-details).
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
/// [Creating patch jobs](/compute/docs/os-patch-management/create-patch-job).
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
        /// A Google Cloud Storage object containing the executable.
        #[prost(message, tag = "2")]
        GcsObject(super::GcsObject),
    }
}
/// Google Cloud Storage object representation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GcsObject {
    /// Required. Bucket of the Google Cloud Storage object.
    #[prost(string, tag = "1")]
    pub bucket: std::string::String,
    /// Required. Name of the Google Cloud Storage object.
    #[prost(string, tag = "2")]
    pub object: std::string::String,
    /// Required. Generation number of the Google Cloud Storage object. This is used to
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
    /// Targets VM instances matching at least one of these label sets. This allows
    /// targeting of disparate groups, for example "env=prod or env=staging".
    #[prost(message, repeated, tag = "2")]
    pub group_labels: ::std::vec::Vec<patch_instance_filter::GroupLabel>,
    /// Targets VM instances in ANY of these zones. Leave empty to target VM
    /// instances in any zone.
    #[prost(string, repeated, tag = "3")]
    pub zones: ::std::vec::Vec<std::string::String>,
    /// Targets any of the VM instances specified. Instances are specified by their
    /// URI in the form `zones/[ZONE]/instances/[INSTANCE_NAME],
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
    /// Represents a group of VMs that can be identified as having all these
    /// labels, for example "env=prod and app=web".
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GroupLabel {
        /// Google Compute Engine instance labels that must be present for a VM
        /// instance to be targeted by this filter.
        #[prost(map = "string, string", tag = "1")]
        pub labels: ::std::collections::HashMap<std::string::String, std::string::String>,
    }
}
/// Patch deployments are configurations that individual patch jobs use to
/// complete a patch. These configurations include instance filter, package
/// repository settings, and a schedule. For more information about creating and
/// managing patch deployments, see [Scheduling patch
/// jobs](/compute/docs/os-patch-management/schedule-patch-jobs).
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
    /// <a href="https://www.ietf.org/rfc/rfc3339.txt" target="_blank">RFC3339</a>
    /// text format.
    #[prost(message, optional, tag = "8")]
    pub create_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Output only. Time the patch deployment was last updated. Timestamp is in
    /// <a href="https://www.ietf.org/rfc/rfc3339.txt" target="_blank">RFC3339</a>
    /// text format.
    #[prost(message, optional, tag = "9")]
    pub update_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Output only. The last time a patch job was started by this deployment.
    /// Timestamp is in
    /// <a href="https://www.ietf.org/rfc/rfc3339.txt" target="_blank">RFC3339</a>
    /// text format.
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
/// <a href="https://www.ietf.org/rfc/rfc3339.txt" target="_blank">RFC3339</a>
/// text format.
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
    impl OsConfigServiceClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
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
                "/google.cloud.osconfig.v1beta.OsConfigService/ExecutePatchJob",
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
                "/google.cloud.osconfig.v1beta.OsConfigService/GetPatchJob",
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
                "/google.cloud.osconfig.v1beta.OsConfigService/CancelPatchJob",
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
                "/google.cloud.osconfig.v1beta.OsConfigService/ListPatchJobs",
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
                "/google.cloud.osconfig.v1beta.OsConfigService/ListPatchJobInstanceDetails",
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
                "/google.cloud.osconfig.v1beta.OsConfigService/CreatePatchDeployment",
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
                "/google.cloud.osconfig.v1beta.OsConfigService/GetPatchDeployment",
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
                "/google.cloud.osconfig.v1beta.OsConfigService/ListPatchDeployments",
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
                "/google.cloud.osconfig.v1beta.OsConfigService/DeletePatchDeployment",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Create an OS Config guest policy."]
        pub async fn create_guest_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateGuestPolicyRequest>,
        ) -> Result<tonic::Response<super::GuestPolicy>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.osconfig.v1beta.OsConfigService/CreateGuestPolicy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Get an OS Config guest policy."]
        pub async fn get_guest_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::GetGuestPolicyRequest>,
        ) -> Result<tonic::Response<super::GuestPolicy>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.osconfig.v1beta.OsConfigService/GetGuestPolicy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Get a page of OS Config guest policies."]
        pub async fn list_guest_policies(
            &mut self,
            request: impl tonic::IntoRequest<super::ListGuestPoliciesRequest>,
        ) -> Result<tonic::Response<super::ListGuestPoliciesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.osconfig.v1beta.OsConfigService/ListGuestPolicies",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Update an OS Config guest policy."]
        pub async fn update_guest_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateGuestPolicyRequest>,
        ) -> Result<tonic::Response<super::GuestPolicy>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.osconfig.v1beta.OsConfigService/UpdateGuestPolicy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Delete an OS Config guest policy."]
        pub async fn delete_guest_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteGuestPolicyRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.osconfig.v1beta.OsConfigService/DeleteGuestPolicy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lookup the effective guest policy that applies to a VM instance. This"]
        #[doc = " lookup merges all policies that are assigned to the instance ancestry."]
        pub async fn lookup_effective_guest_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::LookupEffectiveGuestPolicyRequest>,
        ) -> Result<tonic::Response<super::EffectiveGuestPolicy>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.osconfig.v1beta.OsConfigService/LookupEffectiveGuestPolicy",
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
        #[doc = " Create an OS Config guest policy."]
        async fn create_guest_policy(
            &self,
            request: tonic::Request<super::CreateGuestPolicyRequest>,
        ) -> Result<tonic::Response<super::GuestPolicy>, tonic::Status>;
        #[doc = " Get an OS Config guest policy."]
        async fn get_guest_policy(
            &self,
            request: tonic::Request<super::GetGuestPolicyRequest>,
        ) -> Result<tonic::Response<super::GuestPolicy>, tonic::Status>;
        #[doc = " Get a page of OS Config guest policies."]
        async fn list_guest_policies(
            &self,
            request: tonic::Request<super::ListGuestPoliciesRequest>,
        ) -> Result<tonic::Response<super::ListGuestPoliciesResponse>, tonic::Status>;
        #[doc = " Update an OS Config guest policy."]
        async fn update_guest_policy(
            &self,
            request: tonic::Request<super::UpdateGuestPolicyRequest>,
        ) -> Result<tonic::Response<super::GuestPolicy>, tonic::Status>;
        #[doc = " Delete an OS Config guest policy."]
        async fn delete_guest_policy(
            &self,
            request: tonic::Request<super::DeleteGuestPolicyRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status>;
        #[doc = " Lookup the effective guest policy that applies to a VM instance. This"]
        #[doc = " lookup merges all policies that are assigned to the instance ancestry."]
        async fn lookup_effective_guest_policy(
            &self,
            request: tonic::Request<super::LookupEffectiveGuestPolicyRequest>,
        ) -> Result<tonic::Response<super::EffectiveGuestPolicy>, tonic::Status>;
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
                "/google.cloud.osconfig.v1beta.OsConfigService/ExecutePatchJob" => {
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
                "/google.cloud.osconfig.v1beta.OsConfigService/GetPatchJob" => {
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
                "/google.cloud.osconfig.v1beta.OsConfigService/CancelPatchJob" => {
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
                "/google.cloud.osconfig.v1beta.OsConfigService/ListPatchJobs" => {
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
                "/google.cloud.osconfig.v1beta.OsConfigService/ListPatchJobInstanceDetails" => {
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
                "/google.cloud.osconfig.v1beta.OsConfigService/CreatePatchDeployment" => {
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
                "/google.cloud.osconfig.v1beta.OsConfigService/GetPatchDeployment" => {
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
                "/google.cloud.osconfig.v1beta.OsConfigService/ListPatchDeployments" => {
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
                "/google.cloud.osconfig.v1beta.OsConfigService/DeletePatchDeployment" => {
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
                "/google.cloud.osconfig.v1beta.OsConfigService/CreateGuestPolicy" => {
                    #[allow(non_camel_case_types)]
                    struct CreateGuestPolicySvc<T: OsConfigService>(pub Arc<T>);
                    impl<T: OsConfigService>
                        tonic::server::UnaryService<super::CreateGuestPolicyRequest>
                        for CreateGuestPolicySvc<T>
                    {
                        type Response = super::GuestPolicy;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateGuestPolicyRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.create_guest_policy(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = CreateGuestPolicySvc(inner);
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
                "/google.cloud.osconfig.v1beta.OsConfigService/GetGuestPolicy" => {
                    #[allow(non_camel_case_types)]
                    struct GetGuestPolicySvc<T: OsConfigService>(pub Arc<T>);
                    impl<T: OsConfigService>
                        tonic::server::UnaryService<super::GetGuestPolicyRequest>
                        for GetGuestPolicySvc<T>
                    {
                        type Response = super::GuestPolicy;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetGuestPolicyRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.get_guest_policy(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetGuestPolicySvc(inner);
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
                "/google.cloud.osconfig.v1beta.OsConfigService/ListGuestPolicies" => {
                    #[allow(non_camel_case_types)]
                    struct ListGuestPoliciesSvc<T: OsConfigService>(pub Arc<T>);
                    impl<T: OsConfigService>
                        tonic::server::UnaryService<super::ListGuestPoliciesRequest>
                        for ListGuestPoliciesSvc<T>
                    {
                        type Response = super::ListGuestPoliciesResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListGuestPoliciesRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.list_guest_policies(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ListGuestPoliciesSvc(inner);
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
                "/google.cloud.osconfig.v1beta.OsConfigService/UpdateGuestPolicy" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateGuestPolicySvc<T: OsConfigService>(pub Arc<T>);
                    impl<T: OsConfigService>
                        tonic::server::UnaryService<super::UpdateGuestPolicyRequest>
                        for UpdateGuestPolicySvc<T>
                    {
                        type Response = super::GuestPolicy;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateGuestPolicyRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.update_guest_policy(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = UpdateGuestPolicySvc(inner);
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
                "/google.cloud.osconfig.v1beta.OsConfigService/DeleteGuestPolicy" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteGuestPolicySvc<T: OsConfigService>(pub Arc<T>);
                    impl<T: OsConfigService>
                        tonic::server::UnaryService<super::DeleteGuestPolicyRequest>
                        for DeleteGuestPolicySvc<T>
                    {
                        type Response = ();
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteGuestPolicyRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.delete_guest_policy(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = DeleteGuestPolicySvc(inner);
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
                "/google.cloud.osconfig.v1beta.OsConfigService/LookupEffectiveGuestPolicy" => {
                    #[allow(non_camel_case_types)]
                    struct LookupEffectiveGuestPolicySvc<T: OsConfigService>(pub Arc<T>);
                    impl<T: OsConfigService>
                        tonic::server::UnaryService<super::LookupEffectiveGuestPolicyRequest>
                        for LookupEffectiveGuestPolicySvc<T>
                    {
                        type Response = super::EffectiveGuestPolicy;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::LookupEffectiveGuestPolicyRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { inner.lookup_effective_guest_policy(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = LookupEffectiveGuestPolicySvc(inner);
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
    impl<T: OsConfigService> tonic::transport::NamedService for OsConfigServiceServer<T> {
        const NAME: &'static str = "google.cloud.osconfig.v1beta.OsConfigService";
    }
}
