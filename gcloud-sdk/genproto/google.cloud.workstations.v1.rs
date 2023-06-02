/// A grouping of workstation configurations and the associated workstations
///   in that region.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WorkstationCluster {
    /// Full name of this resource.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Human-readable name for this resource.
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    /// Output only. A system-assigned unique identified for this resource.
    #[prost(string, tag = "3")]
    pub uid: ::prost::alloc::string::String,
    /// Output only. Indicates whether this resource is currently being updated to
    /// match its intended state.
    #[prost(bool, tag = "4")]
    pub reconciling: bool,
    /// Client-specified annotations.
    #[prost(map = "string, string", tag = "5")]
    pub annotations: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Client-specified labels that are applied to the resource and that are also
    /// propagated to the underlying Compute Engine resources.
    #[prost(map = "string, string", tag = "15")]
    pub labels: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Output only. Time when this resource was created.
    #[prost(message, optional, tag = "6")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Time when this resource was most recently updated.
    #[prost(message, optional, tag = "7")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Time when this resource was soft-deleted.
    #[prost(message, optional, tag = "8")]
    pub delete_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Checksum computed by the server. May be sent on update and delete requests
    /// to make sure that the client has an up-to-date value before proceeding.
    #[prost(string, tag = "9")]
    pub etag: ::prost::alloc::string::String,
    /// Immutable. Name of the Compute Engine network in which instances associated
    /// with this cluster will be created.
    #[prost(string, tag = "10")]
    pub network: ::prost::alloc::string::String,
    /// Immutable. Name of the Compute Engine subnetwork in which instances
    /// associated with this cluster will be created. Must be part of the
    /// subnetwork specified for this cluster.
    #[prost(string, tag = "11")]
    pub subnetwork: ::prost::alloc::string::String,
    /// Output only. The private IP address of the control plane for this cluster.
    /// Workstation VMs need access to this IP address to work with the service, so
    /// make sure that your firewall rules allow egress from the workstation VMs to
    /// this address.
    #[prost(string, tag = "16")]
    pub control_plane_ip: ::prost::alloc::string::String,
    /// Configuration for private cluster.
    #[prost(message, optional, tag = "12")]
    pub private_cluster_config: ::core::option::Option<
        workstation_cluster::PrivateClusterConfig,
    >,
    /// Output only. Whether this resource is in degraded mode, in which case it
    /// may require user action to restore full functionality. Details can be found
    /// in the `conditions` field.
    #[prost(bool, tag = "13")]
    pub degraded: bool,
    /// Output only. Status conditions describing the current resource state.
    #[prost(message, repeated, tag = "14")]
    pub conditions: ::prost::alloc::vec::Vec<super::super::super::rpc::Status>,
}
/// Nested message and enum types in `WorkstationCluster`.
pub mod workstation_cluster {
    /// Configuration options for private clusters.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PrivateClusterConfig {
        /// Immutable. Whether Workstations endpoint is private.
        #[prost(bool, tag = "1")]
        pub enable_private_endpoint: bool,
        /// Output only. Hostname for the workstation cluster. This field will be
        /// populated only when private endpoint is enabled. To access workstations
        /// in the cluster, create a new DNS zone mapping this domain name to an
        /// internal IP address and a forwarding rule mapping that address to the
        /// service attachment.
        #[prost(string, tag = "2")]
        pub cluster_hostname: ::prost::alloc::string::String,
        /// Output only. Service attachment URI for the workstation cluster. The
        /// service attachemnt is created when private endpoint is enabled. To access
        /// workstations in the cluster, configure access to the managed service
        /// using [Private Service
        /// Connect](<https://cloud.google.com/vpc/docs/configure-private-service-connect-services>).
        #[prost(string, tag = "3")]
        pub service_attachment_uri: ::prost::alloc::string::String,
        /// Additional projects that are allowed to attach to the workstation
        /// cluster's service attachment. By default, the workstation cluster's
        /// project and the VPC host project (if different) are allowed.
        #[prost(string, repeated, tag = "4")]
        pub allowed_projects: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
}
/// A set of configuration options describing how a workstation will be run.
/// Workstation configurations are intended to be shared across multiple
/// workstations.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WorkstationConfig {
    /// Full name of this resource.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Human-readable name for this resource.
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    /// Output only. A system-assigned unique identified for this resource.
    #[prost(string, tag = "3")]
    pub uid: ::prost::alloc::string::String,
    /// Output only. Indicates whether this resource is currently being updated to
    /// match its intended state.
    #[prost(bool, tag = "4")]
    pub reconciling: bool,
    /// Client-specified annotations.
    #[prost(map = "string, string", tag = "5")]
    pub annotations: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Client-specified labels that are applied to the resource and that are also
    /// propagated to the underlying Compute Engine resources.
    #[prost(map = "string, string", tag = "18")]
    pub labels: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Output only. Time when this resource was created.
    #[prost(message, optional, tag = "6")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Time when this resource was most recently updated.
    #[prost(message, optional, tag = "7")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Time when this resource was soft-deleted.
    #[prost(message, optional, tag = "8")]
    pub delete_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Checksum computed by the server. May be sent on update and delete requests
    /// to make sure that the client has an up-to-date value before proceeding.
    #[prost(string, tag = "9")]
    pub etag: ::prost::alloc::string::String,
    /// How long to wait before automatically stopping an instance that hasn't
    /// received any user traffic. A value of 0 indicates that this instance
    /// should never time out due to idleness. Defaults to 20 minutes.
    #[prost(message, optional, tag = "10")]
    pub idle_timeout: ::core::option::Option<::prost_types::Duration>,
    /// How long to wait before automatically stopping a workstation after it
    /// started. A value of 0 indicates that workstations using this configuration
    /// should never time out. Must be greater than 0 and less than 24 hours if
    /// encryption_key is set. Defaults to 12 hours.
    #[prost(message, optional, tag = "11")]
    pub running_timeout: ::core::option::Option<::prost_types::Duration>,
    /// Runtime host for the workstation.
    #[prost(message, optional, tag = "12")]
    pub host: ::core::option::Option<workstation_config::Host>,
    /// Directories to persist across workstation sessions.
    #[prost(message, repeated, tag = "13")]
    pub persistent_directories: ::prost::alloc::vec::Vec<
        workstation_config::PersistentDirectory,
    >,
    /// Container that will be run for each workstation using this configuration
    /// when that workstation is started.
    #[prost(message, optional, tag = "14")]
    pub container: ::core::option::Option<workstation_config::Container>,
    /// Immutable. Encrypts resources of this workstation configuration using a
    /// customer-managed encryption key.
    ///
    /// If specified, the boot disk of the Compute Engine instance and the
    /// persistent disk are encrypted using this encryption key. If
    /// this field is not set, the disks are encrypted using a generated
    /// key. Customer-managed encryption keys do not protect disk metadata.
    ///
    /// If the customer-managed encryption key is rotated, when the workstation
    /// instance is stopped, the system attempts to recreate the
    /// persistent disk with the new version of the key. Be sure to keep
    /// older versions of the key until the persistent disk is recreated.
    /// Otherwise, data on the persistent disk will be lost.
    ///
    /// If the encryption key is revoked, the workstation session will
    /// automatically be stopped within 7 hours.
    ///
    /// Immutable after the workstation configuration is created.
    #[prost(message, optional, tag = "17")]
    pub encryption_key: ::core::option::Option<
        workstation_config::CustomerEncryptionKey,
    >,
    /// Output only. Whether this resource is degraded, in which case it may
    /// require user action to restore full functionality. See also the
    /// `conditions` field.
    #[prost(bool, tag = "15")]
    pub degraded: bool,
    /// Output only. Status conditions describing the current resource state.
    #[prost(message, repeated, tag = "16")]
    pub conditions: ::prost::alloc::vec::Vec<super::super::super::rpc::Status>,
}
/// Nested message and enum types in `WorkstationConfig`.
pub mod workstation_config {
    /// Runtime host for a workstation.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Host {
        /// Type of host that will be used for the workstation's runtime.
        #[prost(oneof = "host::Config", tags = "1")]
        pub config: ::core::option::Option<host::Config>,
    }
    /// Nested message and enum types in `Host`.
    pub mod host {
        /// A runtime using a Compute Engine instance.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct GceInstance {
            /// The name of a Compute Engine machine type.
            #[prost(string, tag = "1")]
            pub machine_type: ::prost::alloc::string::String,
            /// Email address of the service account used on VM instances
            /// used to support this configuration. If not set, VMs run with a
            /// Google-managed service account. This service account must have
            /// permission to pull the specified container image; otherwise, the image
            /// must be publicly accessible.
            #[prost(string, tag = "2")]
            pub service_account: ::prost::alloc::string::String,
            /// Network tags to add to the Compute Engine machines backing the
            /// Workstations.
            #[prost(string, repeated, tag = "4")]
            pub tags: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
            /// Number of instances to pool for faster workstation startup.
            #[prost(int32, tag = "5")]
            pub pool_size: i32,
            /// Output only. Number of instances currently available in the pool for
            /// faster workstation startup.
            #[prost(int32, tag = "12")]
            pub pooled_instances: i32,
            /// Whether instances have no public IP address.
            #[prost(bool, tag = "6")]
            pub disable_public_ip_addresses: bool,
            /// Whether to enable nested virtualization on instances.
            #[prost(bool, tag = "7")]
            pub enable_nested_virtualization: bool,
            /// A set of Compute Engine Shielded instance options.
            #[prost(message, optional, tag = "8")]
            pub shielded_instance_config: ::core::option::Option<
                gce_instance::GceShieldedInstanceConfig,
            >,
            /// A set of Compute Engine Confidential VM instance options.
            #[prost(message, optional, tag = "10")]
            pub confidential_instance_config: ::core::option::Option<
                gce_instance::GceConfidentialInstanceConfig,
            >,
            /// Size of the boot disk in GB. Defaults to 50.
            #[prost(int32, tag = "9")]
            pub boot_disk_size_gb: i32,
        }
        /// Nested message and enum types in `GceInstance`.
        pub mod gce_instance {
            /// A set of Compute Engine Shielded instance options.
            #[allow(clippy::derive_partial_eq_without_eq)]
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct GceShieldedInstanceConfig {
                /// Whether the instance has Secure Boot enabled.
                #[prost(bool, tag = "1")]
                pub enable_secure_boot: bool,
                /// Whether the instance has the vTPM enabled.
                #[prost(bool, tag = "2")]
                pub enable_vtpm: bool,
                /// Whether the instance has integrity monitoring enabled.
                #[prost(bool, tag = "3")]
                pub enable_integrity_monitoring: bool,
            }
            /// A set of Compute Engine Confidential VM instance options.
            #[allow(clippy::derive_partial_eq_without_eq)]
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct GceConfidentialInstanceConfig {
                /// Whether the instance has confidential compute enabled.
                #[prost(bool, tag = "1")]
                pub enable_confidential_compute: bool,
            }
        }
        /// Type of host that will be used for the workstation's runtime.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Config {
            /// Specifies a Compute Engine instance as the host.
            #[prost(message, tag = "1")]
            GceInstance(GceInstance),
        }
    }
    /// A directory to persist across workstation sessions.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PersistentDirectory {
        /// Location of this directory in the running workstation.
        #[prost(string, tag = "1")]
        pub mount_path: ::prost::alloc::string::String,
        /// How a persistent directory should be implemented.
        #[prost(oneof = "persistent_directory::DirectoryType", tags = "2")]
        pub directory_type: ::core::option::Option<persistent_directory::DirectoryType>,
    }
    /// Nested message and enum types in `PersistentDirectory`.
    pub mod persistent_directory {
        /// A PersistentDirectory backed by a Compute Engine regional persistent
        /// disk.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct GceRegionalPersistentDisk {
            /// Size of the disk in GB. Must be empty if source_snapshot is set.
            /// Defaults to 200.
            #[prost(int32, tag = "1")]
            pub size_gb: i32,
            /// Type of file system that the disk should be formatted with. The
            /// workstation image must support this file system type. Must be empty
            /// if source_snapshot is set. Defaults to ext4.
            #[prost(string, tag = "2")]
            pub fs_type: ::prost::alloc::string::String,
            /// Type of the disk to use. Defaults to pd-standard.
            #[prost(string, tag = "3")]
            pub disk_type: ::prost::alloc::string::String,
            /// Name of the snapshot to use as the source for the disk. If set,
            /// size_gb and fs_type must be empty.
            #[prost(string, tag = "5")]
            pub source_snapshot: ::prost::alloc::string::String,
            /// What should happen to the disk after the workstation is deleted.
            /// Defaults to DELETE.
            #[prost(
                enumeration = "gce_regional_persistent_disk::ReclaimPolicy",
                tag = "4"
            )]
            pub reclaim_policy: i32,
        }
        /// Nested message and enum types in `GceRegionalPersistentDisk`.
        pub mod gce_regional_persistent_disk {
            /// Value representing what should happen to the disk after the workstation
            /// is deleted.
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
            pub enum ReclaimPolicy {
                /// Do not use.
                Unspecified = 0,
                /// The persistent disk will be deleted with the workstation.
                Delete = 1,
                /// The persistent disk will be remain after the workstation is deleted,
                /// and the administrator must manually delete the disk.
                Retain = 2,
            }
            impl ReclaimPolicy {
                /// String value of the enum field names used in the ProtoBuf definition.
                ///
                /// The values are not transformed in any way and thus are considered stable
                /// (if the ProtoBuf definition does not change) and safe for programmatic use.
                pub fn as_str_name(&self) -> &'static str {
                    match self {
                        ReclaimPolicy::Unspecified => "RECLAIM_POLICY_UNSPECIFIED",
                        ReclaimPolicy::Delete => "DELETE",
                        ReclaimPolicy::Retain => "RETAIN",
                    }
                }
                /// Creates an enum from field names used in the ProtoBuf definition.
                pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                    match value {
                        "RECLAIM_POLICY_UNSPECIFIED" => Some(Self::Unspecified),
                        "DELETE" => Some(Self::Delete),
                        "RETAIN" => Some(Self::Retain),
                        _ => None,
                    }
                }
            }
        }
        /// How a persistent directory should be implemented.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum DirectoryType {
            /// A PersistentDirectory backed by a Compute Engine persistent disk.
            #[prost(message, tag = "2")]
            GcePd(GceRegionalPersistentDisk),
        }
    }
    /// A Docker container.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Container {
        /// Docker image defining the container. This image must be accessible by the
        /// service account specified in the workstation configuration.
        #[prost(string, tag = "1")]
        pub image: ::prost::alloc::string::String,
        /// If set, overrides the default ENTRYPOINT specified by the image.
        #[prost(string, repeated, tag = "2")]
        pub command: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// Arguments passed to the entrypoint.
        #[prost(string, repeated, tag = "3")]
        pub args: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// Environment variables passed to the container's entrypoint.
        #[prost(map = "string, string", tag = "4")]
        pub env: ::std::collections::HashMap<
            ::prost::alloc::string::String,
            ::prost::alloc::string::String,
        >,
        /// If set, overrides the default DIR specified by the image.
        #[prost(string, tag = "5")]
        pub working_dir: ::prost::alloc::string::String,
        /// If set, overrides the USER specified in the image with the given uid.
        #[prost(int32, tag = "6")]
        pub run_as_user: i32,
    }
    /// A customer-managed encryption key for the Compute Engine resources
    /// of this workstation configuration.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CustomerEncryptionKey {
        /// Immutable. The name of the Google Cloud KMS encryption key. For example,
        /// `projects/PROJECT_ID/locations/REGION/keyRings/KEY_RING/cryptoKeys/KEY_NAME`.
        #[prost(string, tag = "1")]
        pub kms_key: ::prost::alloc::string::String,
        /// Immutable. The service account to use with the specified
        /// KMS key. We recommend that you use a separate service account
        /// and follow KMS best practices. For more information, see
        /// [Separation of
        /// duties](<https://cloud.google.com/kms/docs/separation-of-duties>) and
        /// `gcloud kms keys add-iam-policy-binding`
        /// \[`--member`\](<https://cloud.google.com/sdk/gcloud/reference/kms/keys/add-iam-policy-binding#--member>).
        #[prost(string, tag = "2")]
        pub kms_key_service_account: ::prost::alloc::string::String,
    }
}
/// A single instance of a developer workstation with its own persistent storage.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Workstation {
    /// Full name of this resource.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Human-readable name for this resource.
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    /// Output only. A system-assigned unique identified for this resource.
    #[prost(string, tag = "3")]
    pub uid: ::prost::alloc::string::String,
    /// Output only. Indicates whether this resource is currently being updated to
    /// match its intended state.
    #[prost(bool, tag = "4")]
    pub reconciling: bool,
    /// Client-specified annotations.
    #[prost(map = "string, string", tag = "5")]
    pub annotations: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Client-specified labels that are applied to the resource and that are also
    /// propagated to the underlying Compute Engine resources.
    #[prost(map = "string, string", tag = "13")]
    pub labels: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Output only. Time when this resource was created.
    #[prost(message, optional, tag = "6")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Time when this resource was most recently updated.
    #[prost(message, optional, tag = "7")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Time when this resource was soft-deleted.
    #[prost(message, optional, tag = "8")]
    pub delete_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Checksum computed by the server. May be sent on update and delete requests
    /// to make sure that the client has an up-to-date value before proceeding.
    #[prost(string, tag = "9")]
    pub etag: ::prost::alloc::string::String,
    /// Output only. Current state of the workstation.
    #[prost(enumeration = "workstation::State", tag = "10")]
    pub state: i32,
    /// Output only. Host to which clients can send HTTPS traffic that will be
    /// received by the workstation. Authorized traffic will be received to the
    /// workstation as HTTP on port 80. To send traffic to a different port,
    /// clients may prefix the host with the destination port in the format
    /// `{port}-{host}`.
    #[prost(string, tag = "11")]
    pub host: ::prost::alloc::string::String,
}
/// Nested message and enum types in `Workstation`.
pub mod workstation {
    /// Whether a workstation is running and ready to receive user requests.
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
        /// Do not use.
        Unspecified = 0,
        /// The workstation is not yet ready to accept requests from users but will
        /// be soon.
        Starting = 1,
        /// The workstation is ready to accept requests from users.
        Running = 2,
        /// The workstation is being stopped.
        Stopping = 3,
        /// The workstation is stopped and will not be able to receive requests until
        /// it is started.
        Stopped = 4,
    }
    impl State {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                State::Unspecified => "STATE_UNSPECIFIED",
                State::Starting => "STATE_STARTING",
                State::Running => "STATE_RUNNING",
                State::Stopping => "STATE_STOPPING",
                State::Stopped => "STATE_STOPPED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "STATE_UNSPECIFIED" => Some(Self::Unspecified),
                "STATE_STARTING" => Some(Self::Starting),
                "STATE_RUNNING" => Some(Self::Running),
                "STATE_STOPPING" => Some(Self::Stopping),
                "STATE_STOPPED" => Some(Self::Stopped),
                _ => None,
            }
        }
    }
}
/// Request message for GetWorkstationCluster.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetWorkstationClusterRequest {
    /// Required. Name of the requested resource.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for ListWorkstationClusters.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListWorkstationClustersRequest {
    /// Required. Parent resource name.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Maximum number of items to return.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// next_page_token value returned from a previous List request, if any.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for ListWorkstationClusters.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListWorkstationClustersResponse {
    /// The requested workstation clusters.
    #[prost(message, repeated, tag = "1")]
    pub workstation_clusters: ::prost::alloc::vec::Vec<WorkstationCluster>,
    /// Token to retrieve the next page of results, or empty if there are no more
    /// results in the list.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Unreachable resources.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Message for creating a CreateWorkstationCluster.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateWorkstationClusterRequest {
    /// Required. Parent resource name.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. ID to use for the workstation cluster.
    #[prost(string, tag = "2")]
    pub workstation_cluster_id: ::prost::alloc::string::String,
    /// Required. Workstation cluster to create.
    #[prost(message, optional, tag = "3")]
    pub workstation_cluster: ::core::option::Option<WorkstationCluster>,
    /// If set, validate the request and preview the review, but do not actually
    /// apply it.
    #[prost(bool, tag = "4")]
    pub validate_only: bool,
}
/// Request message for UpdateWorkstationCluster.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateWorkstationClusterRequest {
    /// Required. Workstation cluster to update.
    #[prost(message, optional, tag = "1")]
    pub workstation_cluster: ::core::option::Option<WorkstationCluster>,
    /// Required. Mask that specifies which fields in the workstation cluster
    /// should be updated.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// If set, validate the request and preview the review, but do not actually
    /// apply it.
    #[prost(bool, tag = "3")]
    pub validate_only: bool,
    /// If set, and the workstation cluster is not found, a new workstation
    /// cluster will be created. In this situation, update_mask is ignored.
    #[prost(bool, tag = "4")]
    pub allow_missing: bool,
}
/// Message for deleting a workstation cluster.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteWorkstationClusterRequest {
    /// Required. Name of the workstation cluster to delete.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// If set, validate the request and preview the review, but do not apply it.
    #[prost(bool, tag = "2")]
    pub validate_only: bool,
    /// If set, the request will be rejected if the latest version of the
    /// workstation cluster on the server does not have this ETag.
    #[prost(string, tag = "3")]
    pub etag: ::prost::alloc::string::String,
    /// If set, any workstation configurations and workstations in the
    /// workstation cluster are also deleted. Otherwise, the request only
    /// works if the workstation cluster has no configurations or workstations.
    #[prost(bool, tag = "4")]
    pub force: bool,
}
/// Request message for GetWorkstationConfig.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetWorkstationConfigRequest {
    /// Required. Name of the requested resource.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for ListWorkstationConfigs.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListWorkstationConfigsRequest {
    /// Required. Parent resource name.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Maximum number of items to return.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// next_page_token value returned from a previous List request, if any.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for ListWorkstationConfigs.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListWorkstationConfigsResponse {
    /// The requested configs.
    #[prost(message, repeated, tag = "1")]
    pub workstation_configs: ::prost::alloc::vec::Vec<WorkstationConfig>,
    /// Token to retrieve the next page of results, or empty if there are no more
    /// results in the list.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Unreachable resources.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Request message for ListUsableWorkstationConfigs.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListUsableWorkstationConfigsRequest {
    /// Required. Parent resource name.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Maximum number of items to return.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// next_page_token value returned from a previous List request, if any.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for ListUsableWorkstationConfigs.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListUsableWorkstationConfigsResponse {
    /// The requested configs.
    #[prost(message, repeated, tag = "1")]
    pub workstation_configs: ::prost::alloc::vec::Vec<WorkstationConfig>,
    /// Token to retrieve the next page of results, or empty if there are no more
    /// results in the list.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Unreachable resources.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Message for creating a CreateWorkstationConfig.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateWorkstationConfigRequest {
    /// Required. Parent resource name.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. ID to use for the workstation configuration.
    #[prost(string, tag = "2")]
    pub workstation_config_id: ::prost::alloc::string::String,
    /// Required. Config to create.
    #[prost(message, optional, tag = "3")]
    pub workstation_config: ::core::option::Option<WorkstationConfig>,
    /// If set, validate the request and preview the review, but do not actually
    /// apply it.
    #[prost(bool, tag = "4")]
    pub validate_only: bool,
}
/// Request message for UpdateWorkstationConfig.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateWorkstationConfigRequest {
    /// Required. Config to update.
    #[prost(message, optional, tag = "1")]
    pub workstation_config: ::core::option::Option<WorkstationConfig>,
    /// Required. Mask specifying which fields in the workstation configuration
    /// should be updated.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// If set, validate the request and preview the review, but do not actually
    /// apply it.
    #[prost(bool, tag = "3")]
    pub validate_only: bool,
    /// If set and the workstation configuration is not found, a new
    /// workstation configuration will be created. In this situation,
    /// update_mask is ignored.
    #[prost(bool, tag = "4")]
    pub allow_missing: bool,
}
/// Message for deleting a workstation configuration.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteWorkstationConfigRequest {
    /// Required. Name of the workstation configuration to delete.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// If set, validate the request and preview the review, but do not actually
    /// apply it.
    #[prost(bool, tag = "2")]
    pub validate_only: bool,
    /// If set, the request is rejected if the latest version of the
    /// workstation configuration on the server does not have this ETag.
    #[prost(string, tag = "3")]
    pub etag: ::prost::alloc::string::String,
    /// If set, any workstations in the workstation configuration are also deleted.
    /// Otherwise, the request works only if the workstation configuration has
    /// no workstations.
    #[prost(bool, tag = "4")]
    pub force: bool,
}
/// Request message for GetWorkstation.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetWorkstationRequest {
    /// Required. Name of the requested resource.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for ListWorkstations.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListWorkstationsRequest {
    /// Required. Parent resource name.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Maximum number of items to return.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// next_page_token value returned from a previous List request, if any.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for ListWorkstations.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListWorkstationsResponse {
    /// The requested workstations.
    #[prost(message, repeated, tag = "1")]
    pub workstations: ::prost::alloc::vec::Vec<Workstation>,
    /// Token to retrieve the next page of results, or empty if there are no more
    /// results in the list.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Unreachable resources.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Request message for ListUsableWorkstations.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListUsableWorkstationsRequest {
    /// Required. Parent resource name.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Maximum number of items to return.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// next_page_token value returned from a previous List request, if any.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for ListUsableWorkstations.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListUsableWorkstationsResponse {
    /// The requested workstations.
    #[prost(message, repeated, tag = "1")]
    pub workstations: ::prost::alloc::vec::Vec<Workstation>,
    /// Token to retrieve the next page of results, or empty if there are no more
    /// results in the list.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Unreachable resources.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Message for creating a CreateWorkstation.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateWorkstationRequest {
    /// Required. Parent resource name.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. ID to use for the workstation.
    #[prost(string, tag = "2")]
    pub workstation_id: ::prost::alloc::string::String,
    /// Required. Workstation to create.
    #[prost(message, optional, tag = "3")]
    pub workstation: ::core::option::Option<Workstation>,
    /// If set, validate the request and preview the review, but do not actually
    /// apply it.
    #[prost(bool, tag = "4")]
    pub validate_only: bool,
}
/// Request message for UpdateWorkstation.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateWorkstationRequest {
    /// Required. Workstation to update.
    #[prost(message, optional, tag = "1")]
    pub workstation: ::core::option::Option<Workstation>,
    /// Required. Mask specifying which fields in the workstation configuration
    /// should be updated.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// If set, validate the request and preview the review, but do not actually
    /// apply it.
    #[prost(bool, tag = "3")]
    pub validate_only: bool,
    /// If set and the workstation configuration is not found, a new
    /// workstation configuration is created. In this situation, update_mask
    /// is ignored.
    #[prost(bool, tag = "4")]
    pub allow_missing: bool,
}
/// Request message for DeleteWorkstation.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteWorkstationRequest {
    /// Required. Name of the workstation to delete.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// If set, validate the request and preview the review, but do not actually
    /// apply it.
    #[prost(bool, tag = "2")]
    pub validate_only: bool,
    /// If set, the request will be rejected if the latest version of the
    /// workstation on the server does not have this ETag.
    #[prost(string, tag = "3")]
    pub etag: ::prost::alloc::string::String,
}
/// Request message for StartWorkstation.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StartWorkstationRequest {
    /// Required. Name of the workstation to start.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// If set, validate the request and preview the review, but do not actually
    /// apply it.
    #[prost(bool, tag = "2")]
    pub validate_only: bool,
    /// If set, the request will be rejected if the latest version of the
    /// workstation on the server does not have this ETag.
    #[prost(string, tag = "3")]
    pub etag: ::prost::alloc::string::String,
}
/// Request message for StopWorkstation.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StopWorkstationRequest {
    /// Required. Name of the workstation to stop.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// If set, validate the request and preview the review, but do not actually
    /// apply it.
    #[prost(bool, tag = "2")]
    pub validate_only: bool,
    /// If set, the request will be rejected if the latest version of the
    /// workstation on the server does not have this ETag.
    #[prost(string, tag = "3")]
    pub etag: ::prost::alloc::string::String,
}
/// Request message for GenerateAccessToken.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateAccessTokenRequest {
    /// Required. Name of the workstation for which the access token should be
    /// generated.
    #[prost(string, tag = "1")]
    pub workstation: ::prost::alloc::string::String,
    /// Desired expiration or lifetime of the access token.
    #[prost(oneof = "generate_access_token_request::Expiration", tags = "2, 3")]
    pub expiration: ::core::option::Option<generate_access_token_request::Expiration>,
}
/// Nested message and enum types in `GenerateAccessTokenRequest`.
pub mod generate_access_token_request {
    /// Desired expiration or lifetime of the access token.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Expiration {
        /// Desired expiration time of the access token. This value must
        /// be at most 24 hours in the future. If a value is not specified, the
        /// token's expiration time will be set to a default value of 1 hour in the
        /// future.
        #[prost(message, tag = "2")]
        ExpireTime(::prost_types::Timestamp),
        /// Desired lifetime duration of the access token. This value must
        /// be at most 24 hours. If a value is not specified, the token's lifetime
        /// will be set to a default value of 1 hour.
        #[prost(message, tag = "3")]
        Ttl(::prost_types::Duration),
    }
}
/// Response message for GenerateAccessToken.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateAccessTokenResponse {
    /// The generated bearer access token. To use this token, include it in an
    /// Authorization header of an HTTP request sent to the associated
    /// workstation's hostname—for example, `Authorization: Bearer
    /// <access_token>`.
    #[prost(string, tag = "1")]
    pub access_token: ::prost::alloc::string::String,
    /// Time at which the generated token will expire.
    #[prost(message, optional, tag = "2")]
    pub expire_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Metadata for long-running operations.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationMetadata {
    /// Output only. Time that the operation was created.
    #[prost(message, optional, tag = "1")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Time that the operation finished running.
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
    /// of the operation.
    #[prost(bool, tag = "6")]
    pub requested_cancellation: bool,
    /// Output only. API version used to start the operation.
    #[prost(string, tag = "7")]
    pub api_version: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod workstations_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Service for interacting with Cloud Workstations.
    #[derive(Debug, Clone)]
    pub struct WorkstationsClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl WorkstationsClient<tonic::transport::Channel> {
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
    impl<T> WorkstationsClient<T>
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
        ) -> WorkstationsClient<InterceptedService<T, F>>
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
            WorkstationsClient::new(InterceptedService::new(inner, interceptor))
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
        /// Returns the requested workstation cluster.
        pub async fn get_workstation_cluster(
            &mut self,
            request: impl tonic::IntoRequest<super::GetWorkstationClusterRequest>,
        ) -> std::result::Result<
            tonic::Response<super::WorkstationCluster>,
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
                "/google.cloud.workstations.v1.Workstations/GetWorkstationCluster",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.workstations.v1.Workstations",
                        "GetWorkstationCluster",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Returns all workstation clusters in the specified location.
        pub async fn list_workstation_clusters(
            &mut self,
            request: impl tonic::IntoRequest<super::ListWorkstationClustersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListWorkstationClustersResponse>,
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
                "/google.cloud.workstations.v1.Workstations/ListWorkstationClusters",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.workstations.v1.Workstations",
                        "ListWorkstationClusters",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a new workstation cluster.
        pub async fn create_workstation_cluster(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateWorkstationClusterRequest>,
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
                "/google.cloud.workstations.v1.Workstations/CreateWorkstationCluster",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.workstations.v1.Workstations",
                        "CreateWorkstationCluster",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates an existing workstation cluster.
        pub async fn update_workstation_cluster(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateWorkstationClusterRequest>,
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
                "/google.cloud.workstations.v1.Workstations/UpdateWorkstationCluster",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.workstations.v1.Workstations",
                        "UpdateWorkstationCluster",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes the specified workstation cluster.
        pub async fn delete_workstation_cluster(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteWorkstationClusterRequest>,
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
                "/google.cloud.workstations.v1.Workstations/DeleteWorkstationCluster",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.workstations.v1.Workstations",
                        "DeleteWorkstationCluster",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Returns the requested workstation configuration.
        pub async fn get_workstation_config(
            &mut self,
            request: impl tonic::IntoRequest<super::GetWorkstationConfigRequest>,
        ) -> std::result::Result<
            tonic::Response<super::WorkstationConfig>,
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
                "/google.cloud.workstations.v1.Workstations/GetWorkstationConfig",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.workstations.v1.Workstations",
                        "GetWorkstationConfig",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Returns all workstation configurations in the specified cluster.
        pub async fn list_workstation_configs(
            &mut self,
            request: impl tonic::IntoRequest<super::ListWorkstationConfigsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListWorkstationConfigsResponse>,
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
                "/google.cloud.workstations.v1.Workstations/ListWorkstationConfigs",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.workstations.v1.Workstations",
                        "ListWorkstationConfigs",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Returns all workstation configurations in the specified cluster on which
        /// the caller has the "workstations.workstation.create" permission.
        pub async fn list_usable_workstation_configs(
            &mut self,
            request: impl tonic::IntoRequest<super::ListUsableWorkstationConfigsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListUsableWorkstationConfigsResponse>,
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
                "/google.cloud.workstations.v1.Workstations/ListUsableWorkstationConfigs",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.workstations.v1.Workstations",
                        "ListUsableWorkstationConfigs",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a new workstation configuration.
        pub async fn create_workstation_config(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateWorkstationConfigRequest>,
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
                "/google.cloud.workstations.v1.Workstations/CreateWorkstationConfig",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.workstations.v1.Workstations",
                        "CreateWorkstationConfig",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates an existing workstation configuration.
        pub async fn update_workstation_config(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateWorkstationConfigRequest>,
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
                "/google.cloud.workstations.v1.Workstations/UpdateWorkstationConfig",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.workstations.v1.Workstations",
                        "UpdateWorkstationConfig",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes the specified workstation configuration.
        pub async fn delete_workstation_config(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteWorkstationConfigRequest>,
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
                "/google.cloud.workstations.v1.Workstations/DeleteWorkstationConfig",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.workstations.v1.Workstations",
                        "DeleteWorkstationConfig",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Returns the requested workstation.
        pub async fn get_workstation(
            &mut self,
            request: impl tonic::IntoRequest<super::GetWorkstationRequest>,
        ) -> std::result::Result<tonic::Response<super::Workstation>, tonic::Status> {
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
                "/google.cloud.workstations.v1.Workstations/GetWorkstation",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.workstations.v1.Workstations",
                        "GetWorkstation",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Returns all Workstations using the specified workstation configuration.
        pub async fn list_workstations(
            &mut self,
            request: impl tonic::IntoRequest<super::ListWorkstationsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListWorkstationsResponse>,
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
                "/google.cloud.workstations.v1.Workstations/ListWorkstations",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.workstations.v1.Workstations",
                        "ListWorkstations",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Returns all workstations using the specified workstation configuration
        /// on which the caller has the "workstations.workstations.use" permission.
        pub async fn list_usable_workstations(
            &mut self,
            request: impl tonic::IntoRequest<super::ListUsableWorkstationsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListUsableWorkstationsResponse>,
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
                "/google.cloud.workstations.v1.Workstations/ListUsableWorkstations",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.workstations.v1.Workstations",
                        "ListUsableWorkstations",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Creates a new workstation.
        pub async fn create_workstation(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateWorkstationRequest>,
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
                "/google.cloud.workstations.v1.Workstations/CreateWorkstation",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.workstations.v1.Workstations",
                        "CreateWorkstation",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Updates an existing workstation.
        pub async fn update_workstation(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateWorkstationRequest>,
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
                "/google.cloud.workstations.v1.Workstations/UpdateWorkstation",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.workstations.v1.Workstations",
                        "UpdateWorkstation",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deletes the specified workstation.
        pub async fn delete_workstation(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteWorkstationRequest>,
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
                "/google.cloud.workstations.v1.Workstations/DeleteWorkstation",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.workstations.v1.Workstations",
                        "DeleteWorkstation",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Starts running a workstation so that users can connect to it.
        pub async fn start_workstation(
            &mut self,
            request: impl tonic::IntoRequest<super::StartWorkstationRequest>,
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
                "/google.cloud.workstations.v1.Workstations/StartWorkstation",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.workstations.v1.Workstations",
                        "StartWorkstation",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Stops running a workstation, reducing costs.
        pub async fn stop_workstation(
            &mut self,
            request: impl tonic::IntoRequest<super::StopWorkstationRequest>,
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
                "/google.cloud.workstations.v1.Workstations/StopWorkstation",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.workstations.v1.Workstations",
                        "StopWorkstation",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Returns a short-lived credential that can be used to send authenticated and
        /// authorized traffic to a workstation.
        pub async fn generate_access_token(
            &mut self,
            request: impl tonic::IntoRequest<super::GenerateAccessTokenRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GenerateAccessTokenResponse>,
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
                "/google.cloud.workstations.v1.Workstations/GenerateAccessToken",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "google.cloud.workstations.v1.Workstations",
                        "GenerateAccessToken",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
