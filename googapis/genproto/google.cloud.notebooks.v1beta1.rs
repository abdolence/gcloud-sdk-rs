/// Definition of a software environment that is used to start a notebook
/// instance.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Environment {
    /// Output only. Name of this environment.
    /// Format:
    /// `projects/{project_id}/locations/{location}/environments/{environment_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Display name of this environment for the UI.
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    /// A brief description of this environment.
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    /// Path to a Bash script that automatically runs after a notebook instance
    /// fully boots up. The path must be a URL or
    /// Cloud Storage path. Example: `"gs://path-to-file/file-name"`
    #[prost(string, tag = "8")]
    pub post_startup_script: ::prost::alloc::string::String,
    /// Output only. The time at which this environment was created.
    #[prost(message, optional, tag = "9")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Type of the environment; can be one of VM image, or container image.
    #[prost(oneof = "environment::ImageType", tags = "6, 7")]
    pub image_type: ::core::option::Option<environment::ImageType>,
}
/// Nested message and enum types in `Environment`.
pub mod environment {
    /// Type of the environment; can be one of VM image, or container image.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ImageType {
        /// Use a Compute Engine VM image to start the notebook instance.
        #[prost(message, tag = "6")]
        VmImage(super::VmImage),
        /// Use a container image to start the notebook instance.
        #[prost(message, tag = "7")]
        ContainerImage(super::ContainerImage),
    }
}
/// Definition of a custom Compute Engine virtual machine image for starting a
/// notebook instance with the environment installed directly on the VM.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VmImage {
    /// Required. The name of the Google Cloud project that this VM image belongs to.
    /// Format: `projects/{project_id}`
    #[prost(string, tag = "1")]
    pub project: ::prost::alloc::string::String,
    /// The reference to an external Compute Engine VM image.
    #[prost(oneof = "vm_image::Image", tags = "2, 3")]
    pub image: ::core::option::Option<vm_image::Image>,
}
/// Nested message and enum types in `VmImage`.
pub mod vm_image {
    /// The reference to an external Compute Engine VM image.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Image {
        /// Use VM image name to find the image.
        #[prost(string, tag = "2")]
        ImageName(::prost::alloc::string::String),
        /// Use this VM image family to find the image; the newest image in this
        /// family will be used.
        #[prost(string, tag = "3")]
        ImageFamily(::prost::alloc::string::String),
    }
}
/// Definition of a container image for starting a notebook instance with the
/// environment installed in a container.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContainerImage {
    /// Required. The path to the container image repository. For example:
    /// `gcr.io/{project_id}/{image_name}`
    #[prost(string, tag = "1")]
    pub repository: ::prost::alloc::string::String,
    /// The tag of the container image. If not specified, this defaults
    /// to the latest tag.
    #[prost(string, tag = "2")]
    pub tag: ::prost::alloc::string::String,
}
/// The definition of a notebook instance.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Instance {
    /// Output only. The name of this notebook instance. Format:
    /// `projects/{project_id}/locations/{location}/instances/{instance_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Path to a Bash script that automatically runs after a notebook instance
    /// fully boots up. The path must be a URL or
    /// Cloud Storage path (`gs://path-to-file/file-name`).
    #[prost(string, tag = "4")]
    pub post_startup_script: ::prost::alloc::string::String,
    /// Output only. The proxy endpoint that is used to access the Jupyter
    /// notebook.
    #[prost(string, tag = "5")]
    pub proxy_uri: ::prost::alloc::string::String,
    /// Input only. The owner of this instance after creation. Format:
    /// `alias@example.com`
    ///
    /// Currently supports one owner only. If not specified, all of the service
    /// account users of your VM instance's service account can use
    /// the instance.
    #[prost(string, repeated, tag = "6")]
    pub instance_owners: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The service account on this instance, giving access to other Google
    /// Cloud services.
    /// You can use any service account within the same project, but you
    /// must have the service account user permission to use the instance.
    ///
    /// If not specified, the [Compute Engine default service
    /// account](<https://cloud.google.com/compute/docs/access/service-accounts#default_service_account>)
    /// is used.
    #[prost(string, tag = "7")]
    pub service_account: ::prost::alloc::string::String,
    /// Required. The [Compute Engine machine
    /// type](<https://cloud.google.com/compute/docs/machine-types>) of this
    /// instance.
    #[prost(string, tag = "8")]
    pub machine_type: ::prost::alloc::string::String,
    /// The hardware accelerator used on this instance. If you use
    /// accelerators, make sure that your configuration has
    /// [enough vCPUs and memory to support the `machine_type` you
    /// have selected](<https://cloud.google.com/compute/docs/gpus/#gpus-list>).
    #[prost(message, optional, tag = "9")]
    pub accelerator_config: ::core::option::Option<instance::AcceleratorConfig>,
    /// Output only. The state of this instance.
    #[prost(enumeration = "instance::State", tag = "10")]
    pub state: i32,
    /// Whether the end user authorizes Google Cloud to install GPU driver
    /// on this instance.
    /// If this field is empty or set to false, the GPU driver won't be installed.
    /// Only applicable to instances with GPUs.
    #[prost(bool, tag = "11")]
    pub install_gpu_driver: bool,
    /// Specify a custom Cloud Storage path where the GPU driver is stored.
    /// If not specified, we'll automatically choose from official GPU drivers.
    #[prost(string, tag = "12")]
    pub custom_gpu_driver_path: ::prost::alloc::string::String,
    /// Input only. The type of the boot disk attached to this instance, defaults
    /// to standard persistent disk (`PD_STANDARD`).
    #[prost(enumeration = "instance::DiskType", tag = "13")]
    pub boot_disk_type: i32,
    /// Input only. The size of the boot disk in GB attached to this instance, up
    /// to a maximum of 64000&nbsp;GB (64&nbsp;TB). The minimum recommended value
    /// is 100&nbsp;GB. If not specified, this defaults to 100.
    #[prost(int64, tag = "14")]
    pub boot_disk_size_gb: i64,
    /// Input only. The type of the data disk attached to this instance, defaults
    /// to standard persistent disk (`PD_STANDARD`).
    #[prost(enumeration = "instance::DiskType", tag = "25")]
    pub data_disk_type: i32,
    /// Input only. The size of the data disk in GB attached to this instance, up
    /// to a maximum of 64000&nbsp;GB (64&nbsp;TB). You can choose the size of the
    /// data disk based on how big your notebooks and data are. If not specified,
    /// this defaults to 100.
    #[prost(int64, tag = "26")]
    pub data_disk_size_gb: i64,
    /// Input only. If true, the data disk will not be auto deleted when deleting
    /// the instance.
    #[prost(bool, tag = "27")]
    pub no_remove_data_disk: bool,
    /// Input only. Disk encryption method used on the boot and data disks,
    /// defaults to GMEK.
    #[prost(enumeration = "instance::DiskEncryption", tag = "15")]
    pub disk_encryption: i32,
    /// Input only. The KMS key used to encrypt the disks, only applicable if
    /// disk_encryption is CMEK. Format:
    /// `projects/{project_id}/locations/{location}/keyRings/{key_ring_id}/cryptoKeys/{key_id}`
    ///
    /// Learn more about [using your own encryption keys](
    /// <https://cloud.google.com/kms/docs/quickstart>).
    #[prost(string, tag = "16")]
    pub kms_key: ::prost::alloc::string::String,
    /// If true, no public IP will be assigned to this instance.
    #[prost(bool, tag = "17")]
    pub no_public_ip: bool,
    /// If true, the notebook instance will not register with the proxy.
    #[prost(bool, tag = "18")]
    pub no_proxy_access: bool,
    /// The name of the VPC that this instance is in.
    /// Format:
    /// `projects/{project_id}/global/networks/{network_id}`
    #[prost(string, tag = "19")]
    pub network: ::prost::alloc::string::String,
    /// The name of the subnet that this instance is in.
    /// Format:
    /// `projects/{project_id}/regions/{region}/subnetworks/{subnetwork_id}`
    #[prost(string, tag = "20")]
    pub subnet: ::prost::alloc::string::String,
    /// Labels to apply to this instance.
    /// These can be later modified by the setLabels method.
    #[prost(map = "string, string", tag = "21")]
    pub labels:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Custom metadata to apply to this instance.
    #[prost(map = "string, string", tag = "22")]
    pub metadata:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Output only. Instance creation time.
    #[prost(message, optional, tag = "23")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Instance update time.
    #[prost(message, optional, tag = "24")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Type of the environment; can be one of VM image, or container image.
    #[prost(oneof = "instance::Environment", tags = "2, 3")]
    pub environment: ::core::option::Option<instance::Environment>,
}
/// Nested message and enum types in `Instance`.
pub mod instance {
    /// Definition of a hardware accelerator. Note that not all combinations
    /// of `type` and `core_count` are valid. Check [GPUs on
    /// Compute Engine](/compute/docs/gpus/#gpus-list) to find a valid
    /// combination. TPUs are not supported.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct AcceleratorConfig {
        /// Type of this accelerator.
        #[prost(enumeration = "AcceleratorType", tag = "1")]
        pub r#type: i32,
        /// Count of cores of this accelerator.
        #[prost(int64, tag = "2")]
        pub core_count: i64,
    }
    /// Definition of the types of hardware accelerators that can be used on this
    /// instance.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum AcceleratorType {
        /// Accelerator type is not specified.
        Unspecified = 0,
        /// Accelerator type is Nvidia Tesla K80.
        NvidiaTeslaK80 = 1,
        /// Accelerator type is Nvidia Tesla P100.
        NvidiaTeslaP100 = 2,
        /// Accelerator type is Nvidia Tesla V100.
        NvidiaTeslaV100 = 3,
        /// Accelerator type is Nvidia Tesla P_4.
        NvidiaTeslaP4 = 4,
        /// Accelerator type is Nvidia Tesla T4.
        NvidiaTeslaT4 = 5,
        /// Accelerator type is NVIDIA Tesla T4 Virtual Workstations.
        NvidiaTeslaT4Vws = 8,
        /// Accelerator type is NVIDIA Tesla P100 Virtual Workstations.
        NvidiaTeslaP100Vws = 9,
        /// Accelerator type is NVIDIA Tesla P_4 Virtual Workstations.
        NvidiaTeslaP4Vws = 10,
        /// (Coming soon) Accelerator type is TPU V2.
        TpuV2 = 6,
        /// (Coming soon) Accelerator type is TPU V3.
        TpuV3 = 7,
    }
    /// The definition of the states of this instance.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// State is not specified.
        Unspecified = 0,
        /// The control logic is starting the instance.
        Starting = 1,
        /// The control logic is installing required frameworks and registering the
        /// instance with notebook proxy
        Provisioning = 2,
        /// The instance is running.
        Active = 3,
        /// The control logic is stopping the instance.
        Stopping = 4,
        /// The instance is stopped.
        Stopped = 5,
        /// The instance is deleted.
        Deleted = 6,
        /// The instance is upgrading.
        Upgrading = 7,
        /// The instance is being created.
        Initializing = 8,
        /// The instance is getting registered.
        Registering = 9,
    }
    /// Possible disk types for notebook instances.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum DiskType {
        /// Disk type not set.
        Unspecified = 0,
        /// Standard persistent disk type.
        PdStandard = 1,
        /// SSD persistent disk type.
        PdSsd = 2,
        /// Balanced persistent disk type.
        PdBalanced = 3,
    }
    /// Definition of the disk encryption options.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum DiskEncryption {
        /// Disk encryption is not specified.
        Unspecified = 0,
        /// Use Google managed encryption keys to encrypt the boot disk.
        Gmek = 1,
        /// Use customer managed encryption keys to encrypt the boot disk.
        Cmek = 2,
    }
    /// Type of the environment; can be one of VM image, or container image.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Environment {
        /// Use a Compute Engine VM image to start the notebook instance.
        #[prost(message, tag = "2")]
        VmImage(super::VmImage),
        /// Use a container image to start the notebook instance.
        #[prost(message, tag = "3")]
        ContainerImage(super::ContainerImage),
    }
}
/// Represents the metadata of the long-running operation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationMetadata {
    /// The time the operation was created.
    #[prost(message, optional, tag = "1")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The time the operation finished running.
    #[prost(message, optional, tag = "2")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Server-defined resource path for the target of the operation.
    #[prost(string, tag = "3")]
    pub target: ::prost::alloc::string::String,
    /// Name of the verb executed by the operation.
    #[prost(string, tag = "4")]
    pub verb: ::prost::alloc::string::String,
    /// Human-readable status of the operation, if any.
    #[prost(string, tag = "5")]
    pub status_message: ::prost::alloc::string::String,
    /// Identifies whether the user has requested cancellation
    /// of the operation. Operations that have successfully been cancelled
    /// have \[Operation.error][\] value with a
    /// \[google.rpc.Status.code][google.rpc.Status.code\] of 1, corresponding to
    /// `Code.CANCELLED`.
    #[prost(bool, tag = "6")]
    pub requested_cancellation: bool,
    /// API version used to start the operation.
    #[prost(string, tag = "7")]
    pub api_version: ::prost::alloc::string::String,
    /// API endpoint name of this operation.
    #[prost(string, tag = "8")]
    pub endpoint: ::prost::alloc::string::String,
}
/// Request for listing notebook instances.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListInstancesRequest {
    /// Required. Format:
    /// `parent=projects/{project_id}/locations/{location}`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Maximum return size of the list call.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A previous returned page token that can be used to continue listing
    /// from the last result.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response for listing notebook instances.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListInstancesResponse {
    /// A list of returned instances.
    #[prost(message, repeated, tag = "1")]
    pub instances: ::prost::alloc::vec::Vec<Instance>,
    /// Page token that can be used to continue listing from the last result in the
    /// next list call.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached. For example,
    /// ['us-west1-a', 'us-central1-b'].
    /// A ListInstancesResponse will only contain either instances or unreachables,
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Request for getting a notebook instance.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetInstanceRequest {
    /// Required. Format:
    /// `projects/{project_id}/locations/{location}/instances/{instance_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request for creating a notebook instance.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateInstanceRequest {
    /// Required. Format:
    /// `parent=projects/{project_id}/locations/{location}`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. User-defined unique ID of this instance.
    #[prost(string, tag = "2")]
    pub instance_id: ::prost::alloc::string::String,
    /// Required. The instance to be created.
    #[prost(message, optional, tag = "3")]
    pub instance: ::core::option::Option<Instance>,
}
/// Request for registering a notebook instance.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisterInstanceRequest {
    /// Required. Format:
    /// `parent=projects/{project_id}/locations/{location}`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. User defined unique ID of this instance. The `instance_id` must
    /// be 1 to 63 characters long and contain only lowercase letters,
    /// numeric characters, and dashes. The first character must be a lowercase
    /// letter and the last character cannot be a dash.
    #[prost(string, tag = "2")]
    pub instance_id: ::prost::alloc::string::String,
}
/// Request for setting instance accelerator.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetInstanceAcceleratorRequest {
    /// Required. Format:
    /// `projects/{project_id}/locations/{location}/instances/{instance_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. Type of this accelerator.
    #[prost(enumeration = "instance::AcceleratorType", tag = "2")]
    pub r#type: i32,
    /// Required. Count of cores of this accelerator. Note that not all
    /// combinations of `type` and `core_count` are valid. Check [GPUs on Compute
    /// Engine](<https://cloud.google.com/compute/docs/gpus/#gpus-list>) to find a
    /// valid combination. TPUs are not supported.
    #[prost(int64, tag = "3")]
    pub core_count: i64,
}
/// Request for setting instance machine type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetInstanceMachineTypeRequest {
    /// Required. Format:
    /// `projects/{project_id}/locations/{location}/instances/{instance_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The [Compute Engine machine
    /// type](<https://cloud.google.com/compute/docs/machine-types>).
    #[prost(string, tag = "2")]
    pub machine_type: ::prost::alloc::string::String,
}
/// Request for setting instance labels.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetInstanceLabelsRequest {
    /// Required. Format:
    /// `projects/{project_id}/locations/{location}/instances/{instance_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Labels to apply to this instance.
    /// These can be later modified by the setLabels method
    #[prost(map = "string, string", tag = "2")]
    pub labels:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
}
/// Request for deleting a notebook instance.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteInstanceRequest {
    /// Required. Format:
    /// `projects/{project_id}/locations/{location}/instances/{instance_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request for starting a notebook instance
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StartInstanceRequest {
    /// Required. Format:
    /// `projects/{project_id}/locations/{location}/instances/{instance_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request for stopping a notebook instance
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StopInstanceRequest {
    /// Required. Format:
    /// `projects/{project_id}/locations/{location}/instances/{instance_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request for reseting a notebook instance
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResetInstanceRequest {
    /// Required. Format:
    /// `projects/{project_id}/locations/{location}/instances/{instance_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request for notebook instances to report information to Notebooks API.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReportInstanceInfoRequest {
    /// Required. Format:
    /// `projects/{project_id}/locations/{location}/instances/{instance_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The VM hardware token for authenticating the VM.
    /// <https://cloud.google.com/compute/docs/instances/verifying-instance-identity>
    #[prost(string, tag = "2")]
    pub vm_id: ::prost::alloc::string::String,
    /// The metadata reported to Notebooks API. This will be merged to the instance
    /// metadata store
    #[prost(map = "string, string", tag = "3")]
    pub metadata:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
}
/// Request for checking if a notebook instance is upgradeable.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IsInstanceUpgradeableRequest {
    /// Required. Format:
    /// `projects/{project_id}/locations/{location}/instances/{instance_id}`
    #[prost(string, tag = "1")]
    pub notebook_instance: ::prost::alloc::string::String,
}
/// Response for checking if a notebook instance is upgradeable.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IsInstanceUpgradeableResponse {
    /// If an instance is upgradeable.
    #[prost(bool, tag = "1")]
    pub upgradeable: bool,
    /// The version this instance will be upgraded to if calling the upgrade
    /// endpoint. This field will only be populated if field upgradeable is true.
    #[prost(string, tag = "2")]
    pub upgrade_version: ::prost::alloc::string::String,
    /// Additional information about upgrade.
    #[prost(string, tag = "3")]
    pub upgrade_info: ::prost::alloc::string::String,
}
/// Request for upgrading a notebook instance
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpgradeInstanceRequest {
    /// Required. Format:
    /// `projects/{project_id}/locations/{location}/instances/{instance_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request for upgrading a notebook instance from within the VM
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpgradeInstanceInternalRequest {
    /// Required. Format:
    /// `projects/{project_id}/locations/{location}/instances/{instance_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The VM hardware token for authenticating the VM.
    /// <https://cloud.google.com/compute/docs/instances/verifying-instance-identity>
    #[prost(string, tag = "2")]
    pub vm_id: ::prost::alloc::string::String,
}
/// Request for listing environments.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListEnvironmentsRequest {
    /// Required. Format: `projects/{project_id}/locations/{location}`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Maximum return size of the list call.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A previous returned page token that can be used to continue listing from
    /// the last result.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response for listing environments.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListEnvironmentsResponse {
    /// A list of returned environments.
    #[prost(message, repeated, tag = "1")]
    pub environments: ::prost::alloc::vec::Vec<Environment>,
    /// A page token that can be used to continue listing from the last result
    /// in the next list call.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Request for getting a notebook environment.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetEnvironmentRequest {
    /// Required. Format:
    /// `projects/{project_id}/locations/{location}/environments/{environment_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request for creating a notebook environment.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateEnvironmentRequest {
    /// Required. Format: `projects/{project_id}/locations/{location}`
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. User-defined unique ID of this environment. The `environment_id`
    /// must be 1 to 63 characters long and contain only lowercase letters, numeric
    /// characters, and dashes. The first character must be a lowercase letter and
    /// the last character cannot be a dash.
    #[prost(string, tag = "2")]
    pub environment_id: ::prost::alloc::string::String,
    /// Required. The environment to be created.
    #[prost(message, optional, tag = "3")]
    pub environment: ::core::option::Option<Environment>,
}
/// Request for deleting a notebook environment.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteEnvironmentRequest {
    /// Required. Format:
    /// `projects/{project_id}/locations/{location}/environments/{environment_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
#[doc = r" Generated client implementations."]
pub mod notebook_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " API v1beta1 service for Cloud AI Platform Notebooks."]
    #[derive(Debug, Clone)]
    pub struct NotebookServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> NotebookServiceClient<T>
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
        ) -> NotebookServiceClient<InterceptedService<T, F>>
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
            NotebookServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Lists instances in a given project and location."]
        pub async fn list_instances(
            &mut self,
            request: impl tonic::IntoRequest<super::ListInstancesRequest>,
        ) -> Result<tonic::Response<super::ListInstancesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.notebooks.v1beta1.NotebookService/ListInstances",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets details of a single Instance."]
        pub async fn get_instance(
            &mut self,
            request: impl tonic::IntoRequest<super::GetInstanceRequest>,
        ) -> Result<tonic::Response<super::Instance>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.notebooks.v1beta1.NotebookService/GetInstance",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a new Instance in a given project and location."]
        pub async fn create_instance(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateInstanceRequest>,
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
                "/google.cloud.notebooks.v1beta1.NotebookService/CreateInstance",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Registers an existing legacy notebook instance to the Notebooks API server."]
        #[doc = " Legacy instances are instances created with the legacy Compute Engine"]
        #[doc = " calls. They are not manageable by the Notebooks API out of the box. This"]
        #[doc = " call makes these instances manageable by the Notebooks API."]
        pub async fn register_instance(
            &mut self,
            request: impl tonic::IntoRequest<super::RegisterInstanceRequest>,
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
                "/google.cloud.notebooks.v1beta1.NotebookService/RegisterInstance",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates the guest accelerators of a single Instance."]
        pub async fn set_instance_accelerator(
            &mut self,
            request: impl tonic::IntoRequest<super::SetInstanceAcceleratorRequest>,
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
                "/google.cloud.notebooks.v1beta1.NotebookService/SetInstanceAccelerator",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates the machine type of a single Instance."]
        pub async fn set_instance_machine_type(
            &mut self,
            request: impl tonic::IntoRequest<super::SetInstanceMachineTypeRequest>,
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
                "/google.cloud.notebooks.v1beta1.NotebookService/SetInstanceMachineType",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates the labels of an Instance."]
        pub async fn set_instance_labels(
            &mut self,
            request: impl tonic::IntoRequest<super::SetInstanceLabelsRequest>,
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
                "/google.cloud.notebooks.v1beta1.NotebookService/SetInstanceLabels",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes a single Instance."]
        pub async fn delete_instance(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteInstanceRequest>,
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
                "/google.cloud.notebooks.v1beta1.NotebookService/DeleteInstance",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Starts a notebook instance."]
        pub async fn start_instance(
            &mut self,
            request: impl tonic::IntoRequest<super::StartInstanceRequest>,
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
                "/google.cloud.notebooks.v1beta1.NotebookService/StartInstance",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Stops a notebook instance."]
        pub async fn stop_instance(
            &mut self,
            request: impl tonic::IntoRequest<super::StopInstanceRequest>,
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
                "/google.cloud.notebooks.v1beta1.NotebookService/StopInstance",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Resets a notebook instance."]
        pub async fn reset_instance(
            &mut self,
            request: impl tonic::IntoRequest<super::ResetInstanceRequest>,
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
                "/google.cloud.notebooks.v1beta1.NotebookService/ResetInstance",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Allows notebook instances to"]
        #[doc = " report their latest instance information to the Notebooks"]
        #[doc = " API server. The server will merge the reported information to"]
        #[doc = " the instance metadata store. Do not use this method directly."]
        pub async fn report_instance_info(
            &mut self,
            request: impl tonic::IntoRequest<super::ReportInstanceInfoRequest>,
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
                "/google.cloud.notebooks.v1beta1.NotebookService/ReportInstanceInfo",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Check if a notebook instance is upgradable."]
        pub async fn is_instance_upgradeable(
            &mut self,
            request: impl tonic::IntoRequest<super::IsInstanceUpgradeableRequest>,
        ) -> Result<tonic::Response<super::IsInstanceUpgradeableResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.notebooks.v1beta1.NotebookService/IsInstanceUpgradeable",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Upgrades a notebook instance to the latest version."]
        pub async fn upgrade_instance(
            &mut self,
            request: impl tonic::IntoRequest<super::UpgradeInstanceRequest>,
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
                "/google.cloud.notebooks.v1beta1.NotebookService/UpgradeInstance",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Allows notebook instances to"]
        #[doc = " call this endpoint to upgrade themselves. Do not use this method directly."]
        pub async fn upgrade_instance_internal(
            &mut self,
            request: impl tonic::IntoRequest<super::UpgradeInstanceInternalRequest>,
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
                "/google.cloud.notebooks.v1beta1.NotebookService/UpgradeInstanceInternal",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists environments in a project."]
        pub async fn list_environments(
            &mut self,
            request: impl tonic::IntoRequest<super::ListEnvironmentsRequest>,
        ) -> Result<tonic::Response<super::ListEnvironmentsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.notebooks.v1beta1.NotebookService/ListEnvironments",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets details of a single Environment."]
        pub async fn get_environment(
            &mut self,
            request: impl tonic::IntoRequest<super::GetEnvironmentRequest>,
        ) -> Result<tonic::Response<super::Environment>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.notebooks.v1beta1.NotebookService/GetEnvironment",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a new Environment."]
        pub async fn create_environment(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateEnvironmentRequest>,
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
                "/google.cloud.notebooks.v1beta1.NotebookService/CreateEnvironment",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes a single Environment."]
        pub async fn delete_environment(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteEnvironmentRequest>,
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
                "/google.cloud.notebooks.v1beta1.NotebookService/DeleteEnvironment",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
