/// Definition of a software environment that is used to start a notebook
/// instance.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Environment {
    /// Output only. Name of this environment.
    /// Format:
    /// `projects/{project_id}/locations/{location}/environments/{environment_id}`
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Display name of this environment for the UI.
    #[prost(string, tag = "2")]
    pub display_name: std::string::String,
    /// A brief description of this environment.
    #[prost(string, tag = "3")]
    pub description: std::string::String,
    /// Path to a Bash script that automatically runs after a notebook instance
    /// fully boots up. The path must be a URL or
    /// Cloud Storage path. Example: `"gs://path-to-file/file-name"`
    #[prost(string, tag = "8")]
    pub post_startup_script: std::string::String,
    /// Output only. The time at which this environment was created.
    #[prost(message, optional, tag = "9")]
    pub create_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Type of the environment; can be one of VM image, or container image.
    #[prost(oneof = "environment::ImageType", tags = "6, 7")]
    pub image_type: ::std::option::Option<environment::ImageType>,
}
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
    pub project: std::string::String,
    /// The reference to an external Compute Engine VM image.
    #[prost(oneof = "vm_image::Image", tags = "2, 3")]
    pub image: ::std::option::Option<vm_image::Image>,
}
pub mod vm_image {
    /// The reference to an external Compute Engine VM image.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Image {
        /// Use VM image name to find the image.
        #[prost(string, tag = "2")]
        ImageName(std::string::String),
        /// Use this VM image family to find the image; the newest image in this
        /// family will be used.
        #[prost(string, tag = "3")]
        ImageFamily(std::string::String),
    }
}
/// Definition of a container image for starting a notebook instance with the
/// environment installed in a container.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContainerImage {
    /// Required. The path to the container image repository. For example:
    /// `gcr.io/{project_id}/{image_name}`
    #[prost(string, tag = "1")]
    pub repository: std::string::String,
    /// The tag of the container image. If not specified, this defaults
    /// to the latest tag.
    #[prost(string, tag = "2")]
    pub tag: std::string::String,
}
/// The definition of a notebook instance.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Instance {
    /// Output only. The name of this notebook instance. Format:
    /// `projects/{project_id}/locations/{location}/instances/{instance_id}`
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Path to a Bash script that automatically runs after a notebook instance
    /// fully boots up. The path must be a URL or
    /// Cloud Storage path (`gs://path-to-file/file-name`).
    #[prost(string, tag = "4")]
    pub post_startup_script: std::string::String,
    /// Output only. The proxy endpoint that is used to access the Jupyter notebook.
    #[prost(string, tag = "5")]
    pub proxy_uri: std::string::String,
    /// Input only. The owner of this instance after creation. Format: `alias@example.com`
    ///
    /// Currently supports one owner only. If not specified, all of the service
    /// account users of your VM instance's service account can use
    /// the instance.
    #[prost(string, repeated, tag = "6")]
    pub instance_owners: ::std::vec::Vec<std::string::String>,
    /// The service account on this instance, giving access to other Google
    /// Cloud services.
    /// You can use any service account within the same project, but you
    /// must have the service account user permission to use the instance.
    ///
    /// If not specified, the [Compute Engine default service
    /// account](https://cloud.google.com/compute/docs/access/service-accounts#default_service_account)
    /// is used.
    #[prost(string, tag = "7")]
    pub service_account: std::string::String,
    /// Required. The [Compute Engine machine type](https://cloud.google.com/compute/docs/machine-types) of this
    /// instance.
    #[prost(string, tag = "8")]
    pub machine_type: std::string::String,
    /// The hardware accelerator used on this instance. If you use
    /// accelerators, make sure that your configuration has
    /// [enough vCPUs and memory to support the `machine_type` you
    /// have selected](https://cloud.google.com/compute/docs/gpus/#gpus-list).
    #[prost(message, optional, tag = "9")]
    pub accelerator_config: ::std::option::Option<instance::AcceleratorConfig>,
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
    pub custom_gpu_driver_path: std::string::String,
    /// Input only. The type of the boot disk attached to this instance, defaults to
    /// standard persistent disk (`PD_STANDARD`).
    #[prost(enumeration = "instance::DiskType", tag = "13")]
    pub boot_disk_type: i32,
    /// Input only. The size of the boot disk in GB attached to this instance, up to a maximum
    /// of 64000&nbsp;GB (64&nbsp;TB). The minimum recommended value is
    /// 100&nbsp;GB. If not specified, this defaults to 100.
    #[prost(int64, tag = "14")]
    pub boot_disk_size_gb: i64,
    /// Input only. The type of the data disk attached to this instance, defaults to
    /// standard persistent disk (`PD_STANDARD`).
    #[prost(enumeration = "instance::DiskType", tag = "25")]
    pub data_disk_type: i32,
    /// Input only. The size of the data disk in GB attached to this instance, up to a maximum
    /// of 64000&nbsp;GB (64&nbsp;TB). You can choose the size of the data disk
    /// based on how big your notebooks and data are. If not specified, this
    /// defaults to 100.
    #[prost(int64, tag = "26")]
    pub data_disk_size_gb: i64,
    /// Input only. If true, the data disk will not be auto deleted when deleting the instance.
    #[prost(bool, tag = "27")]
    pub no_remove_data_disk: bool,
    /// Input only. Disk encryption method used on the boot and data disks, defaults to GMEK.
    #[prost(enumeration = "instance::DiskEncryption", tag = "15")]
    pub disk_encryption: i32,
    /// Input only. The KMS key used to encrypt the disks, only applicable if disk_encryption
    /// is CMEK.
    /// Format:
    /// `projects/{project_id}/locations/{location}/keyRings/{key_ring_id}/cryptoKeys/{key_id}`
    ///
    /// Learn more about [using your own encryption keys]( https://cloud.google.com/kms/docs/quickstart).
    #[prost(string, tag = "16")]
    pub kms_key: std::string::String,
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
    pub network: std::string::String,
    /// The name of the subnet that this instance is in.
    /// Format:
    /// `projects/{project_id}/regions/{region}/subnetworks/{subnetwork_id}`
    #[prost(string, tag = "20")]
    pub subnet: std::string::String,
    /// Labels to apply to this instance.
    /// These can be later modified by the setLabels method.
    #[prost(map = "string, string", tag = "21")]
    pub labels: ::std::collections::HashMap<std::string::String, std::string::String>,
    /// Custom metadata to apply to this instance.
    #[prost(map = "string, string", tag = "22")]
    pub metadata: ::std::collections::HashMap<std::string::String, std::string::String>,
    /// Output only. Instance creation time.
    #[prost(message, optional, tag = "23")]
    pub create_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Output only. Instance update time.
    #[prost(message, optional, tag = "24")]
    pub update_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Type of the environment; can be one of VM image, or container image.
    #[prost(oneof = "instance::Environment", tags = "2, 3")]
    pub environment: ::std::option::Option<instance::Environment>,
}
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
    pub create_time: ::std::option::Option<::prost_types::Timestamp>,
    /// The time the operation finished running.
    #[prost(message, optional, tag = "2")]
    pub end_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Server-defined resource path for the target of the operation.
    #[prost(string, tag = "3")]
    pub target: std::string::String,
    /// Name of the verb executed by the operation.
    #[prost(string, tag = "4")]
    pub verb: std::string::String,
    /// Human-readable status of the operation, if any.
    #[prost(string, tag = "5")]
    pub status_message: std::string::String,
    /// Identifies whether the user has requested cancellation
    /// of the operation. Operations that have successfully been cancelled
    /// have [Operation.error][] value with a
    /// [google.rpc.Status.code][google.rpc.Status.code] of 1, corresponding to `Code.CANCELLED`.
    #[prost(bool, tag = "6")]
    pub requested_cancellation: bool,
    /// API version used to start the operation.
    #[prost(string, tag = "7")]
    pub api_version: std::string::String,
}
/// Request for listing notebook instances.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListInstancesRequest {
    /// Required. Format:
    /// `parent=projects/{project_id}/locations/{location}`
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Maximum return size of the list call.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A previous returned page token that can be used to continue listing
    /// from the last result.
    #[prost(string, tag = "3")]
    pub page_token: std::string::String,
}
/// Response for listing notebook instances.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListInstancesResponse {
    /// A list of returned instances.
    #[prost(message, repeated, tag = "1")]
    pub instances: ::std::vec::Vec<Instance>,
    /// Page token that can be used to continue listing from the last result in the
    /// next list call.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
    /// Locations that could not be reached. For example,
    /// ['us-west1-a', 'us-central1-b'].
    /// A ListInstancesResponse will only contain either instances or unreachables,
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::std::vec::Vec<std::string::String>,
}
/// Request for getting a notebook instance.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetInstanceRequest {
    /// Required. Format:
    /// `projects/{project_id}/locations/{location}/instances/{instance_id}`
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Request for creating a notebook instance.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateInstanceRequest {
    /// Required. Format:
    /// `parent=projects/{project_id}/locations/{location}`
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Required. User-defined unique ID of this instance.
    #[prost(string, tag = "2")]
    pub instance_id: std::string::String,
    /// Required. The instance to be created.
    #[prost(message, optional, tag = "3")]
    pub instance: ::std::option::Option<Instance>,
}
/// Request for registering a notebook instance.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisterInstanceRequest {
    /// Required. Format:
    /// `parent=projects/{project_id}/locations/{location}`
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Required. User defined unique ID of this instance. The `instance_id` must
    /// be 1 to 63 characters long and contain only lowercase letters,
    /// numeric characters, and dashes. The first character must be a lowercase
    /// letter and the last character cannot be a dash.
    #[prost(string, tag = "2")]
    pub instance_id: std::string::String,
}
/// Request for setting instance accelerator.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetInstanceAcceleratorRequest {
    /// Required. Format:
    /// `projects/{project_id}/locations/{location}/instances/{instance_id}`
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Required. Type of this accelerator.
    #[prost(enumeration = "instance::AcceleratorType", tag = "2")]
    pub r#type: i32,
    /// Required. Count of cores of this accelerator. Note that not all combinations
    /// of `type` and `core_count` are valid. Check [GPUs on
    /// Compute Engine](/compute/docs/gpus/#gpus-list) to find a valid
    /// combination. TPUs are not supported.
    #[prost(int64, tag = "3")]
    pub core_count: i64,
}
/// Request for setting instance machine type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetInstanceMachineTypeRequest {
    /// Required. Format:
    /// `projects/{project_id}/locations/{location}/instances/{instance_id}`
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Required. The [Compute Engine machine type](/compute/docs/machine-types).
    #[prost(string, tag = "2")]
    pub machine_type: std::string::String,
}
/// Request for setting instance labels.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetInstanceLabelsRequest {
    /// Required. Format:
    /// `projects/{project_id}/locations/{location}/instances/{instance_id}`
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Labels to apply to this instance.
    /// These can be later modified by the setLabels method
    #[prost(map = "string, string", tag = "2")]
    pub labels: ::std::collections::HashMap<std::string::String, std::string::String>,
}
/// Request for deleting a notebook instance.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteInstanceRequest {
    /// Required. Format:
    /// `projects/{project_id}/locations/{location}/instances/{instance_id}`
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Request for starting a notebook instance
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StartInstanceRequest {
    /// Required. Format:
    /// `projects/{project_id}/locations/{location}/instances/{instance_id}`
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Request for stopping a notebook instance
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StopInstanceRequest {
    /// Required. Format:
    /// `projects/{project_id}/locations/{location}/instances/{instance_id}`
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Request for reseting a notebook instance
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResetInstanceRequest {
    /// Required. Format:
    /// `projects/{project_id}/locations/{location}/instances/{instance_id}`
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Request for notebook instances to report information to Notebooks API.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReportInstanceInfoRequest {
    /// Required. Format:
    /// `projects/{project_id}/locations/{location}/instances/{instance_id}`
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Required. The VM hardware token for authenticating the VM.
    /// https://cloud.google.com/compute/docs/instances/verifying-instance-identity
    #[prost(string, tag = "2")]
    pub vm_id: std::string::String,
    /// The metadata reported to Notebooks API. This will be merged to the instance
    /// metadata store
    #[prost(map = "string, string", tag = "3")]
    pub metadata: ::std::collections::HashMap<std::string::String, std::string::String>,
}
/// Request for checking if a notebook instance is upgradeable.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IsInstanceUpgradeableRequest {
    /// Required. Format:
    /// `projects/{project_id}/locations/{location}/instances/{instance_id}`
    #[prost(string, tag = "1")]
    pub notebook_instance: std::string::String,
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
    pub upgrade_version: std::string::String,
}
/// Request for upgrading a notebook instance
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpgradeInstanceRequest {
    /// Required. Format:
    /// `projects/{project_id}/locations/{location}/instances/{instance_id}`
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Request for upgrading a notebook instance from within the VM
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpgradeInstanceInternalRequest {
    /// Required. Format:
    /// `projects/{project_id}/locations/{location}/instances/{instance_id}`
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Required. The VM hardware token for authenticating the VM.
    /// https://cloud.google.com/compute/docs/instances/verifying-instance-identity
    #[prost(string, tag = "2")]
    pub vm_id: std::string::String,
}
/// Request for listing environments.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListEnvironmentsRequest {
    /// Required. Format: `projects/{project_id}/locations/{location}`
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Maximum return size of the list call.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A previous returned page token that can be used to continue listing from
    /// the last result.
    #[prost(string, tag = "3")]
    pub page_token: std::string::String,
}
/// Response for listing environments.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListEnvironmentsResponse {
    /// A list of returned environments.
    #[prost(message, repeated, tag = "1")]
    pub environments: ::std::vec::Vec<Environment>,
    /// A page token that can be used to continue listing from the last result
    /// in the next list call.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::std::vec::Vec<std::string::String>,
}
/// Request for getting a notebook environment.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetEnvironmentRequest {
    /// Required. Format:
    /// `projects/{project_id}/locations/{location}/environments/{environment_id}`
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Request for creating a notebook environment.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateEnvironmentRequest {
    /// Required. Format: `projects/{project_id}/locations/{location}`
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Required. User-defined unique ID of this environment. The `environment_id` must
    /// be 1 to 63 characters long and contain only lowercase letters,
    /// numeric characters, and dashes. The first character must be a lowercase
    /// letter and the last character cannot be a dash.
    #[prost(string, tag = "2")]
    pub environment_id: std::string::String,
    /// Required. The environment to be created.
    #[prost(message, optional, tag = "3")]
    pub environment: ::std::option::Option<Environment>,
}
/// Request for deleting a notebook environment.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteEnvironmentRequest {
    /// Required. Format:
    /// `projects/{project_id}/locations/{location}/environments/{environment_id}`
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
#[doc = r" Generated client implementations."]
pub mod notebook_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " API service for Cloud AI Platform Notebooks."]
    pub struct NotebookServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> NotebookServiceClient<T>
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
    impl<T: Clone> Clone for NotebookServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for NotebookServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "NotebookServiceClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod notebook_service_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with NotebookServiceServer."]
    #[async_trait]
    pub trait NotebookService: Send + Sync + 'static {
        #[doc = " Lists instances in a given project and location."]
        async fn list_instances(
            &self,
            request: tonic::Request<super::ListInstancesRequest>,
        ) -> Result<tonic::Response<super::ListInstancesResponse>, tonic::Status>;
        #[doc = " Gets details of a single Instance."]
        async fn get_instance(
            &self,
            request: tonic::Request<super::GetInstanceRequest>,
        ) -> Result<tonic::Response<super::Instance>, tonic::Status>;
        #[doc = " Creates a new Instance in a given project and location."]
        async fn create_instance(
            &self,
            request: tonic::Request<super::CreateInstanceRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        >;
        #[doc = " Registers an existing legacy notebook instance to the Notebooks API server."]
        #[doc = " Legacy instances are instances created with the legacy Compute Engine"]
        #[doc = " calls. They are not manageable by the Notebooks API out of the box. This"]
        #[doc = " call makes these instances manageable by the Notebooks API."]
        async fn register_instance(
            &self,
            request: tonic::Request<super::RegisterInstanceRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        >;
        #[doc = " Updates the guest accelerators of a single Instance."]
        async fn set_instance_accelerator(
            &self,
            request: tonic::Request<super::SetInstanceAcceleratorRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        >;
        #[doc = " Updates the machine type of a single Instance."]
        async fn set_instance_machine_type(
            &self,
            request: tonic::Request<super::SetInstanceMachineTypeRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        >;
        #[doc = " Updates the labels of an Instance."]
        async fn set_instance_labels(
            &self,
            request: tonic::Request<super::SetInstanceLabelsRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        >;
        #[doc = " Deletes a single Instance."]
        async fn delete_instance(
            &self,
            request: tonic::Request<super::DeleteInstanceRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        >;
        #[doc = " Starts a notebook instance."]
        async fn start_instance(
            &self,
            request: tonic::Request<super::StartInstanceRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        >;
        #[doc = " Stops a notebook instance."]
        async fn stop_instance(
            &self,
            request: tonic::Request<super::StopInstanceRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        >;
        #[doc = " Resets a notebook instance."]
        async fn reset_instance(
            &self,
            request: tonic::Request<super::ResetInstanceRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        >;
        #[doc = " Allows notebook instances to"]
        #[doc = " report their latest instance information to the Notebooks"]
        #[doc = " API server. The server will merge the reported information to"]
        #[doc = " the instance metadata store. Do not use this method directly."]
        async fn report_instance_info(
            &self,
            request: tonic::Request<super::ReportInstanceInfoRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        >;
        #[doc = " Check if a notebook instance is upgradable."]
        async fn is_instance_upgradeable(
            &self,
            request: tonic::Request<super::IsInstanceUpgradeableRequest>,
        ) -> Result<tonic::Response<super::IsInstanceUpgradeableResponse>, tonic::Status>;
        #[doc = " Upgrades a notebook instance to the latest version."]
        async fn upgrade_instance(
            &self,
            request: tonic::Request<super::UpgradeInstanceRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        >;
        #[doc = " Allows notebook instances to"]
        #[doc = " call this endpoint to upgrade themselves. Do not use this method directly."]
        async fn upgrade_instance_internal(
            &self,
            request: tonic::Request<super::UpgradeInstanceInternalRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        >;
        #[doc = " Lists environments in a project."]
        async fn list_environments(
            &self,
            request: tonic::Request<super::ListEnvironmentsRequest>,
        ) -> Result<tonic::Response<super::ListEnvironmentsResponse>, tonic::Status>;
        #[doc = " Gets details of a single Environment."]
        async fn get_environment(
            &self,
            request: tonic::Request<super::GetEnvironmentRequest>,
        ) -> Result<tonic::Response<super::Environment>, tonic::Status>;
        #[doc = " Creates a new Environment."]
        async fn create_environment(
            &self,
            request: tonic::Request<super::CreateEnvironmentRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        >;
        #[doc = " Deletes a single Environment."]
        async fn delete_environment(
            &self,
            request: tonic::Request<super::DeleteEnvironmentRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        >;
    }
    #[doc = " API service for Cloud AI Platform Notebooks."]
    #[derive(Debug)]
    pub struct NotebookServiceServer<T: NotebookService> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: NotebookService> NotebookServiceServer<T> {
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
    impl<T, B> Service<http::Request<B>> for NotebookServiceServer<T>
    where
        T: NotebookService,
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
                "/google.cloud.notebooks.v1beta1.NotebookService/ListInstances" => {
                    #[allow(non_camel_case_types)]
                    struct ListInstancesSvc<T: NotebookService>(pub Arc<T>);
                    impl<T: NotebookService>
                        tonic::server::UnaryService<super::ListInstancesRequest>
                        for ListInstancesSvc<T>
                    {
                        type Response = super::ListInstancesResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListInstancesRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).list_instances(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ListInstancesSvc(inner);
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
                "/google.cloud.notebooks.v1beta1.NotebookService/GetInstance" => {
                    #[allow(non_camel_case_types)]
                    struct GetInstanceSvc<T: NotebookService>(pub Arc<T>);
                    impl<T: NotebookService> tonic::server::UnaryService<super::GetInstanceRequest>
                        for GetInstanceSvc<T>
                    {
                        type Response = super::Instance;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetInstanceRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_instance(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetInstanceSvc(inner);
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
                "/google.cloud.notebooks.v1beta1.NotebookService/CreateInstance" => {
                    #[allow(non_camel_case_types)]
                    struct CreateInstanceSvc<T: NotebookService>(pub Arc<T>);
                    impl<T: NotebookService>
                        tonic::server::UnaryService<super::CreateInstanceRequest>
                        for CreateInstanceSvc<T>
                    {
                        type Response = super::super::super::super::longrunning::Operation;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateInstanceRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).create_instance(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = CreateInstanceSvc(inner);
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
                "/google.cloud.notebooks.v1beta1.NotebookService/RegisterInstance" => {
                    #[allow(non_camel_case_types)]
                    struct RegisterInstanceSvc<T: NotebookService>(pub Arc<T>);
                    impl<T: NotebookService>
                        tonic::server::UnaryService<super::RegisterInstanceRequest>
                        for RegisterInstanceSvc<T>
                    {
                        type Response = super::super::super::super::longrunning::Operation;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RegisterInstanceRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).register_instance(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = RegisterInstanceSvc(inner);
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
                "/google.cloud.notebooks.v1beta1.NotebookService/SetInstanceAccelerator" => {
                    #[allow(non_camel_case_types)]
                    struct SetInstanceAcceleratorSvc<T: NotebookService>(pub Arc<T>);
                    impl<T: NotebookService>
                        tonic::server::UnaryService<super::SetInstanceAcceleratorRequest>
                        for SetInstanceAcceleratorSvc<T>
                    {
                        type Response = super::super::super::super::longrunning::Operation;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SetInstanceAcceleratorRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).set_instance_accelerator(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = SetInstanceAcceleratorSvc(inner);
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
                "/google.cloud.notebooks.v1beta1.NotebookService/SetInstanceMachineType" => {
                    #[allow(non_camel_case_types)]
                    struct SetInstanceMachineTypeSvc<T: NotebookService>(pub Arc<T>);
                    impl<T: NotebookService>
                        tonic::server::UnaryService<super::SetInstanceMachineTypeRequest>
                        for SetInstanceMachineTypeSvc<T>
                    {
                        type Response = super::super::super::super::longrunning::Operation;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SetInstanceMachineTypeRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).set_instance_machine_type(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = SetInstanceMachineTypeSvc(inner);
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
                "/google.cloud.notebooks.v1beta1.NotebookService/SetInstanceLabels" => {
                    #[allow(non_camel_case_types)]
                    struct SetInstanceLabelsSvc<T: NotebookService>(pub Arc<T>);
                    impl<T: NotebookService>
                        tonic::server::UnaryService<super::SetInstanceLabelsRequest>
                        for SetInstanceLabelsSvc<T>
                    {
                        type Response = super::super::super::super::longrunning::Operation;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SetInstanceLabelsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).set_instance_labels(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = SetInstanceLabelsSvc(inner);
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
                "/google.cloud.notebooks.v1beta1.NotebookService/DeleteInstance" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteInstanceSvc<T: NotebookService>(pub Arc<T>);
                    impl<T: NotebookService>
                        tonic::server::UnaryService<super::DeleteInstanceRequest>
                        for DeleteInstanceSvc<T>
                    {
                        type Response = super::super::super::super::longrunning::Operation;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteInstanceRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).delete_instance(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = DeleteInstanceSvc(inner);
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
                "/google.cloud.notebooks.v1beta1.NotebookService/StartInstance" => {
                    #[allow(non_camel_case_types)]
                    struct StartInstanceSvc<T: NotebookService>(pub Arc<T>);
                    impl<T: NotebookService>
                        tonic::server::UnaryService<super::StartInstanceRequest>
                        for StartInstanceSvc<T>
                    {
                        type Response = super::super::super::super::longrunning::Operation;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::StartInstanceRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).start_instance(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = StartInstanceSvc(inner);
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
                "/google.cloud.notebooks.v1beta1.NotebookService/StopInstance" => {
                    #[allow(non_camel_case_types)]
                    struct StopInstanceSvc<T: NotebookService>(pub Arc<T>);
                    impl<T: NotebookService> tonic::server::UnaryService<super::StopInstanceRequest>
                        for StopInstanceSvc<T>
                    {
                        type Response = super::super::super::super::longrunning::Operation;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::StopInstanceRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).stop_instance(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = StopInstanceSvc(inner);
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
                "/google.cloud.notebooks.v1beta1.NotebookService/ResetInstance" => {
                    #[allow(non_camel_case_types)]
                    struct ResetInstanceSvc<T: NotebookService>(pub Arc<T>);
                    impl<T: NotebookService>
                        tonic::server::UnaryService<super::ResetInstanceRequest>
                        for ResetInstanceSvc<T>
                    {
                        type Response = super::super::super::super::longrunning::Operation;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ResetInstanceRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).reset_instance(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ResetInstanceSvc(inner);
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
                "/google.cloud.notebooks.v1beta1.NotebookService/ReportInstanceInfo" => {
                    #[allow(non_camel_case_types)]
                    struct ReportInstanceInfoSvc<T: NotebookService>(pub Arc<T>);
                    impl<T: NotebookService>
                        tonic::server::UnaryService<super::ReportInstanceInfoRequest>
                        for ReportInstanceInfoSvc<T>
                    {
                        type Response = super::super::super::super::longrunning::Operation;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ReportInstanceInfoRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).report_instance_info(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ReportInstanceInfoSvc(inner);
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
                "/google.cloud.notebooks.v1beta1.NotebookService/IsInstanceUpgradeable" => {
                    #[allow(non_camel_case_types)]
                    struct IsInstanceUpgradeableSvc<T: NotebookService>(pub Arc<T>);
                    impl<T: NotebookService>
                        tonic::server::UnaryService<super::IsInstanceUpgradeableRequest>
                        for IsInstanceUpgradeableSvc<T>
                    {
                        type Response = super::IsInstanceUpgradeableResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::IsInstanceUpgradeableRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).is_instance_upgradeable(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = IsInstanceUpgradeableSvc(inner);
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
                "/google.cloud.notebooks.v1beta1.NotebookService/UpgradeInstance" => {
                    #[allow(non_camel_case_types)]
                    struct UpgradeInstanceSvc<T: NotebookService>(pub Arc<T>);
                    impl<T: NotebookService>
                        tonic::server::UnaryService<super::UpgradeInstanceRequest>
                        for UpgradeInstanceSvc<T>
                    {
                        type Response = super::super::super::super::longrunning::Operation;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpgradeInstanceRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).upgrade_instance(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = UpgradeInstanceSvc(inner);
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
                "/google.cloud.notebooks.v1beta1.NotebookService/UpgradeInstanceInternal" => {
                    #[allow(non_camel_case_types)]
                    struct UpgradeInstanceInternalSvc<T: NotebookService>(pub Arc<T>);
                    impl<T: NotebookService>
                        tonic::server::UnaryService<super::UpgradeInstanceInternalRequest>
                        for UpgradeInstanceInternalSvc<T>
                    {
                        type Response = super::super::super::super::longrunning::Operation;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpgradeInstanceInternalRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).upgrade_instance_internal(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = UpgradeInstanceInternalSvc(inner);
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
                "/google.cloud.notebooks.v1beta1.NotebookService/ListEnvironments" => {
                    #[allow(non_camel_case_types)]
                    struct ListEnvironmentsSvc<T: NotebookService>(pub Arc<T>);
                    impl<T: NotebookService>
                        tonic::server::UnaryService<super::ListEnvironmentsRequest>
                        for ListEnvironmentsSvc<T>
                    {
                        type Response = super::ListEnvironmentsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListEnvironmentsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).list_environments(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ListEnvironmentsSvc(inner);
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
                "/google.cloud.notebooks.v1beta1.NotebookService/GetEnvironment" => {
                    #[allow(non_camel_case_types)]
                    struct GetEnvironmentSvc<T: NotebookService>(pub Arc<T>);
                    impl<T: NotebookService>
                        tonic::server::UnaryService<super::GetEnvironmentRequest>
                        for GetEnvironmentSvc<T>
                    {
                        type Response = super::Environment;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetEnvironmentRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_environment(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetEnvironmentSvc(inner);
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
                "/google.cloud.notebooks.v1beta1.NotebookService/CreateEnvironment" => {
                    #[allow(non_camel_case_types)]
                    struct CreateEnvironmentSvc<T: NotebookService>(pub Arc<T>);
                    impl<T: NotebookService>
                        tonic::server::UnaryService<super::CreateEnvironmentRequest>
                        for CreateEnvironmentSvc<T>
                    {
                        type Response = super::super::super::super::longrunning::Operation;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateEnvironmentRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).create_environment(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = CreateEnvironmentSvc(inner);
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
                "/google.cloud.notebooks.v1beta1.NotebookService/DeleteEnvironment" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteEnvironmentSvc<T: NotebookService>(pub Arc<T>);
                    impl<T: NotebookService>
                        tonic::server::UnaryService<super::DeleteEnvironmentRequest>
                        for DeleteEnvironmentSvc<T>
                    {
                        type Response = super::super::super::super::longrunning::Operation;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteEnvironmentRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).delete_environment(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = DeleteEnvironmentSvc(inner);
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
    impl<T: NotebookService> Clone for NotebookServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: NotebookService> Clone for _Inner<T> {
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
