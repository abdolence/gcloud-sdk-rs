/// Parameters that can be configured on Linux nodes.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LinuxNodeConfig {
    /// The Linux kernel parameters to be applied to the nodes and all pods running
    /// on the nodes.
    ///
    /// The following parameters are supported.
    ///
    /// net.core.busy_poll
    /// net.core.busy_read
    /// net.core.netdev_max_backlog
    /// net.core.rmem_max
    /// net.core.wmem_default
    /// net.core.wmem_max
    /// net.core.optmem_max
    /// net.core.somaxconn
    /// net.ipv4.tcp_rmem
    /// net.ipv4.tcp_wmem
    /// net.ipv4.tcp_tw_reuse
    #[prost(map = "string, string", tag = "1")]
    pub sysctls: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// cgroup_mode specifies the cgroup mode to be used on the node.
    #[prost(enumeration = "linux_node_config::CgroupMode", tag = "2")]
    pub cgroup_mode: i32,
}
/// Nested message and enum types in `LinuxNodeConfig`.
pub mod linux_node_config {
    /// Possible cgroup modes that can be used.
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
    pub enum CgroupMode {
        /// CGROUP_MODE_UNSPECIFIED is when unspecified cgroup configuration is used.
        /// The default for the GKE node OS image will be used.
        Unspecified = 0,
        /// CGROUP_MODE_V1 specifies to use cgroupv1 for the cgroup configuration on
        /// the node image.
        V1 = 1,
        /// CGROUP_MODE_V2 specifies to use cgroupv2 for the cgroup configuration on
        /// the node image.
        V2 = 2,
    }
    impl CgroupMode {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                CgroupMode::Unspecified => "CGROUP_MODE_UNSPECIFIED",
                CgroupMode::V1 => "CGROUP_MODE_V1",
                CgroupMode::V2 => "CGROUP_MODE_V2",
            }
        }
    }
}
/// Node kubelet configs.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NodeKubeletConfig {
    /// Control the CPU management policy on the node.
    /// See
    /// <https://kubernetes.io/docs/tasks/administer-cluster/cpu-management-policies/>
    ///
    /// The following values are allowed.
    /// * "none": the default, which represents the existing scheduling behavior.
    /// * "static": allows pods with certain resource characteristics to be granted
    /// increased CPU affinity and exclusivity on the node.
    /// The default value is 'none' if unspecified.
    #[prost(string, tag = "1")]
    pub cpu_manager_policy: ::prost::alloc::string::String,
    /// Enable CPU CFS quota enforcement for containers that specify CPU limits.
    ///
    /// This option is enabled by default which makes kubelet use CFS quota
    /// (<https://www.kernel.org/doc/Documentation/scheduler/sched-bwc.txt>) to
    /// enforce container CPU limits. Otherwise, CPU limits will not be enforced at
    /// all.
    ///
    /// Disable this option to mitigate CPU throttling problems while still having
    /// your pods to be in Guaranteed QoS class by specifying the CPU limits.
    ///
    /// The default value is 'true' if unspecified.
    #[prost(message, optional, tag = "2")]
    pub cpu_cfs_quota: ::core::option::Option<bool>,
    /// Set the CPU CFS quota period value 'cpu.cfs_period_us'.
    ///
    /// The string must be a sequence of decimal numbers, each with optional
    /// fraction and a unit suffix, such as "300ms".
    /// Valid time units are "ns", "us" (or "µs"), "ms", "s", "m", "h".
    /// The value must be a positive duration.
    #[prost(string, tag = "3")]
    pub cpu_cfs_quota_period: ::prost::alloc::string::String,
    /// Set the Pod PID limits. See
    /// <https://kubernetes.io/docs/concepts/policy/pid-limiting/#pod-pid-limits>
    ///
    /// Controls the maximum number of processes allowed to run in a pod. The value
    /// must be greater than or equal to 1024 and less than 4194304.
    #[prost(int64, tag = "4")]
    pub pod_pids_limit: i64,
}
/// Parameters that describe the nodes in a cluster.
///
/// GKE Autopilot clusters do not
/// recognize parameters in `NodeConfig`. Use
/// \[AutoprovisioningNodePoolDefaults][google.container.v1beta1.AutoprovisioningNodePoolDefaults\]
/// instead.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NodeConfig {
    /// The name of a Google Compute Engine [machine
    /// type](<https://cloud.google.com/compute/docs/machine-types>).
    ///
    /// If unspecified, the default machine type is
    /// `e2-medium`.
    #[prost(string, tag = "1")]
    pub machine_type: ::prost::alloc::string::String,
    /// Size of the disk attached to each node, specified in GB.
    /// The smallest allowed disk size is 10GB.
    ///
    /// If unspecified, the default disk size is 100GB.
    #[prost(int32, tag = "2")]
    pub disk_size_gb: i32,
    /// The set of Google API scopes to be made available on all of the
    /// node VMs under the "default" service account.
    ///
    /// The following scopes are recommended, but not required, and by default are
    /// not included:
    ///
    /// * `<https://www.googleapis.com/auth/compute`> is required for mounting
    /// persistent storage on your nodes.
    /// * `<https://www.googleapis.com/auth/devstorage.read_only`> is required for
    /// communicating with **gcr.io**
    /// (the [Google Container
    /// Registry](<https://cloud.google.com/container-registry/>)).
    ///
    /// If unspecified, no scopes are added, unless Cloud Logging or Cloud
    /// Monitoring are enabled, in which case their required scopes will be added.
    #[prost(string, repeated, tag = "3")]
    pub oauth_scopes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The Google Cloud Platform Service Account to be used by the node VMs.
    /// Specify the email address of the Service Account; otherwise, if no Service
    /// Account is specified, the "default" service account is used.
    #[prost(string, tag = "9")]
    pub service_account: ::prost::alloc::string::String,
    /// The metadata key/value pairs assigned to instances in the cluster.
    ///
    /// Keys must conform to the regexp `\[a-zA-Z0-9-_\]+` and be less than 128 bytes
    /// in length. These are reflected as part of a URL in the metadata server.
    /// Additionally, to avoid ambiguity, keys must not conflict with any other
    /// metadata keys for the project or be one of the reserved keys:
    ///
    ///   - "cluster-location"
    ///   - "cluster-name"
    ///   - "cluster-uid"
    ///   - "configure-sh"
    ///   - "containerd-configure-sh"
    ///   - "enable-oslogin"
    ///   - "gci-ensure-gke-docker"
    ///   - "gci-metrics-enabled"
    ///   - "gci-update-strategy"
    ///   - "instance-template"
    ///   - "kube-env"
    ///   - "startup-script"
    ///   - "user-data"
    ///   - "disable-address-manager"
    ///   - "windows-startup-script-ps1"
    ///   - "common-psm1"
    ///   - "k8s-node-setup-psm1"
    ///   - "install-ssh-psm1"
    ///   - "user-profile-psm1"
    ///
    /// Values are free-form strings, and only have meaning as interpreted by
    /// the image running in the instance. The only restriction placed on them is
    /// that each value's size must be less than or equal to 32 KB.
    ///
    /// The total size of all keys and values must be less than 512 KB.
    #[prost(map = "string, string", tag = "4")]
    pub metadata: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// The image type to use for this node. Note that for a given image type,
    /// the latest version of it will be used.
    #[prost(string, tag = "5")]
    pub image_type: ::prost::alloc::string::String,
    /// The map of Kubernetes labels (key/value pairs) to be applied to each node.
    /// These will added in addition to any default label(s) that
    /// Kubernetes may apply to the node.
    /// In case of conflict in label keys, the applied set may differ depending on
    /// the Kubernetes version -- it's best to assume the behavior is undefined
    /// and conflicts should be avoided.
    /// For more information, including usage and the valid values, see:
    /// <https://kubernetes.io/docs/concepts/overview/working-with-objects/labels/>
    #[prost(map = "string, string", tag = "6")]
    pub labels: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// The number of local SSD disks to be attached to the node.
    ///
    /// The limit for this value is dependent upon the maximum number of
    /// disks available on a machine per zone. See:
    /// <https://cloud.google.com/compute/docs/disks/local-ssd>
    /// for more information.
    #[prost(int32, tag = "7")]
    pub local_ssd_count: i32,
    /// The list of instance tags applied to all nodes. Tags are used to identify
    /// valid sources or targets for network firewalls and are specified by
    /// the client during cluster or node pool creation. Each tag within the list
    /// must comply with RFC1035.
    #[prost(string, repeated, tag = "8")]
    pub tags: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Whether the nodes are created as preemptible VM instances. See:
    /// <https://cloud.google.com/compute/docs/instances/preemptible> for more
    /// inforamtion about preemptible VM instances.
    #[prost(bool, tag = "10")]
    pub preemptible: bool,
    /// A list of hardware accelerators to be attached to each node.
    /// See <https://cloud.google.com/compute/docs/gpus> for more information about
    /// support for GPUs.
    #[prost(message, repeated, tag = "11")]
    pub accelerators: ::prost::alloc::vec::Vec<AcceleratorConfig>,
    /// Sandbox configuration for this node.
    #[prost(message, optional, tag = "17")]
    pub sandbox_config: ::core::option::Option<SandboxConfig>,
    /// Setting this field will assign instances of this
    /// pool to run on the specified node group. This is useful for running
    /// workloads on [sole tenant
    /// nodes](<https://cloud.google.com/compute/docs/nodes/sole-tenant-nodes>).
    #[prost(string, tag = "18")]
    pub node_group: ::prost::alloc::string::String,
    /// The optional reservation affinity. Setting this field will apply
    /// the specified [Zonal Compute
    /// Reservation](<https://cloud.google.com/compute/docs/instances/reserving-zonal-resources>)
    /// to this node pool.
    #[prost(message, optional, tag = "19")]
    pub reservation_affinity: ::core::option::Option<ReservationAffinity>,
    /// Type of the disk attached to each node (e.g. 'pd-standard', 'pd-ssd' or
    /// 'pd-balanced')
    ///
    /// If unspecified, the default disk type is 'pd-standard'
    #[prost(string, tag = "12")]
    pub disk_type: ::prost::alloc::string::String,
    /// Minimum CPU platform to be used by this instance. The instance may be
    /// scheduled on the specified or newer CPU platform. Applicable values are the
    /// friendly names of CPU platforms, such as
    /// `minCpuPlatform: "Intel Haswell"` or
    /// `minCpuPlatform: "Intel Sandy Bridge"`. For more
    /// information, read [how to specify min CPU
    /// platform](<https://cloud.google.com/compute/docs/instances/specify-min-cpu-platform>).
    #[prost(string, tag = "13")]
    pub min_cpu_platform: ::prost::alloc::string::String,
    /// The workload metadata configuration for this node.
    #[prost(message, optional, tag = "14")]
    pub workload_metadata_config: ::core::option::Option<WorkloadMetadataConfig>,
    /// List of kubernetes taints to be applied to each node.
    ///
    /// For more information, including usage and the valid values, see:
    /// <https://kubernetes.io/docs/concepts/configuration/taint-and-toleration/>
    #[prost(message, repeated, tag = "15")]
    pub taints: ::prost::alloc::vec::Vec<NodeTaint>,
    ///
    /// The Customer Managed Encryption Key used to encrypt the boot disk attached
    /// to each node in the node pool. This should be of the form
    /// projects/\[KEY_PROJECT_ID]/locations/[LOCATION]/keyRings/[RING_NAME]/cryptoKeys/[KEY_NAME\].
    /// For more information about protecting resources with Cloud KMS Keys please
    /// see:
    /// <https://cloud.google.com/compute/docs/disks/customer-managed-encryption>
    #[prost(string, tag = "23")]
    pub boot_disk_kms_key: ::prost::alloc::string::String,
    /// Shielded Instance options.
    #[prost(message, optional, tag = "20")]
    pub shielded_instance_config: ::core::option::Option<ShieldedInstanceConfig>,
    /// Parameters that can be configured on Linux nodes.
    #[prost(message, optional, tag = "21")]
    pub linux_node_config: ::core::option::Option<LinuxNodeConfig>,
    /// Node kubelet configs.
    #[prost(message, optional, tag = "22")]
    pub kubelet_config: ::core::option::Option<NodeKubeletConfig>,
    /// Parameters for the ephemeral storage filesystem.
    /// If unspecified, ephemeral storage is backed by the boot disk.
    #[prost(message, optional, tag = "24")]
    pub ephemeral_storage_config: ::core::option::Option<EphemeralStorageConfig>,
    /// GCFS (Google Container File System) configs.
    #[prost(message, optional, tag = "25")]
    pub gcfs_config: ::core::option::Option<GcfsConfig>,
    /// Advanced features for the Compute Engine VM.
    #[prost(message, optional, tag = "26")]
    pub advanced_machine_features: ::core::option::Option<AdvancedMachineFeatures>,
    /// Enable or disable gvnic on the node pool.
    #[prost(message, optional, tag = "29")]
    pub gvnic: ::core::option::Option<VirtualNic>,
    /// Spot flag for enabling Spot VM, which is a rebrand of
    /// the existing preemptible flag.
    #[prost(bool, tag = "32")]
    pub spot: bool,
    /// Confidential nodes config.
    /// All the nodes in the node pool will be Confidential VM once enabled.
    #[prost(message, optional, tag = "35")]
    pub confidential_nodes: ::core::option::Option<ConfidentialNodes>,
    /// Enable or disable NCCL fast socket for the node pool.
    #[prost(message, optional, tag = "36")]
    pub fast_socket: ::core::option::Option<FastSocket>,
    /// The resource labels for the node pool to use to annotate any related
    /// Google Compute Engine resources.
    #[prost(map = "string, string", tag = "37")]
    pub resource_labels: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Logging configuration.
    #[prost(message, optional, tag = "38")]
    pub logging_config: ::core::option::Option<NodePoolLoggingConfig>,
}
/// Specifies options for controlling advanced machine features.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdvancedMachineFeatures {
    /// The number of threads per physical core. To disable simultaneous
    /// multithreading (SMT) set this to 1. If unset, the maximum number of threads
    /// supported per core by the underlying processor is assumed.
    #[prost(int64, optional, tag = "1")]
    pub threads_per_core: ::core::option::Option<i64>,
}
/// Parameters for node pool-level network config.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NodeNetworkConfig {
    /// Input only. Whether to create a new range for pod IPs in this node pool.
    /// Defaults are provided for `pod_range` and `pod_ipv4_cidr_block` if they
    /// are not specified.
    ///
    /// If neither `create_pod_range` or `pod_range` are specified, the
    /// cluster-level default (`ip_allocation_policy.cluster_ipv4_cidr_block`) is
    /// used.
    ///
    /// Only applicable if `ip_allocation_policy.use_ip_aliases` is true.
    ///
    /// This field cannot be changed after the node pool has been created.
    #[prost(bool, tag = "4")]
    pub create_pod_range: bool,
    /// The ID of the secondary range for pod IPs.
    /// If `create_pod_range` is true, this ID is used for the new range.
    /// If `create_pod_range` is false, uses an existing secondary range with this
    /// ID.
    ///
    /// Only applicable if `ip_allocation_policy.use_ip_aliases` is true.
    ///
    /// This field cannot be changed after the node pool has been created.
    #[prost(string, tag = "5")]
    pub pod_range: ::prost::alloc::string::String,
    /// The IP address range for pod IPs in this node pool.
    ///
    /// Only applicable if `create_pod_range` is true.
    ///
    /// Set to blank to have a range chosen with the default size.
    ///
    /// Set to /netmask (e.g. `/14`) to have a range chosen with a specific
    /// netmask.
    ///
    /// Set to a
    /// \[CIDR\](<https://en.wikipedia.org/wiki/Classless_Inter-Domain_Routing>)
    /// notation (e.g. `10.96.0.0/14`) to pick a specific range to use.
    ///
    /// Only applicable if `ip_allocation_policy.use_ip_aliases` is true.
    ///
    /// This field cannot be changed after the node pool has been created.
    #[prost(string, tag = "6")]
    pub pod_ipv4_cidr_block: ::prost::alloc::string::String,
    /// Whether nodes have internal IP addresses only.
    /// If enable_private_nodes is not specified, then the value is derived from
    /// \[cluster.privateClusterConfig.enablePrivateNodes][google.container.v1beta1.PrivateClusterConfig.enablePrivateNodes\]
    #[prost(bool, optional, tag = "9")]
    pub enable_private_nodes: ::core::option::Option<bool>,
    /// Network bandwidth tier configuration.
    #[prost(message, optional, tag = "11")]
    pub network_performance_config: ::core::option::Option<
        node_network_config::NetworkPerformanceConfig,
    >,
}
/// Nested message and enum types in `NodeNetworkConfig`.
pub mod node_network_config {
    /// Configuration of all network bandwidth tiers
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct NetworkPerformanceConfig {
        /// Specifies the total network bandwidth tier for the NodePool.
        #[prost(enumeration = "network_performance_config::Tier", optional, tag = "1")]
        pub total_egress_bandwidth_tier: ::core::option::Option<i32>,
        /// Specifies the network bandwidth tier for the NodePool for traffic to
        /// external/public IP addresses.
        #[prost(enumeration = "network_performance_config::Tier", optional, tag = "2")]
        pub external_ip_egress_bandwidth_tier: ::core::option::Option<i32>,
    }
    /// Nested message and enum types in `NetworkPerformanceConfig`.
    pub mod network_performance_config {
        /// Node network tier
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
        pub enum Tier {
            /// Default value
            Unspecified = 0,
            /// Higher bandwidth, actual values based on VM size.
            Tier1 = 1,
        }
        impl Tier {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    Tier::Unspecified => "TIER_UNSPECIFIED",
                    Tier::Tier1 => "TIER_1",
                }
            }
        }
    }
}
/// A set of Shielded Instance options.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShieldedInstanceConfig {
    /// Defines whether the instance has Secure Boot enabled.
    ///
    /// Secure Boot helps ensure that the system only runs authentic software by
    /// verifying the digital signature of all boot components, and halting the
    /// boot process if signature verification fails.
    #[prost(bool, tag = "1")]
    pub enable_secure_boot: bool,
    /// Defines whether the instance has integrity monitoring enabled.
    ///
    /// Enables monitoring and attestation of the boot integrity of the instance.
    /// The attestation is performed against the integrity policy baseline. This
    /// baseline is initially derived from the implicitly trusted boot image when
    /// the instance is created.
    #[prost(bool, tag = "2")]
    pub enable_integrity_monitoring: bool,
}
/// SandboxConfig contains configurations of the sandbox to use for the node.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SandboxConfig {
    /// Type of the sandbox to use for the node (e.g. 'gvisor')
    #[deprecated]
    #[prost(string, tag = "1")]
    pub sandbox_type: ::prost::alloc::string::String,
    /// Type of the sandbox to use for the node.
    #[prost(enumeration = "sandbox_config::Type", tag = "2")]
    pub r#type: i32,
}
/// Nested message and enum types in `SandboxConfig`.
pub mod sandbox_config {
    /// Possible types of sandboxes.
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
        /// Default value. This should not be used.
        Unspecified = 0,
        /// Run sandbox using gvisor.
        Gvisor = 1,
    }
    impl Type {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Type::Unspecified => "UNSPECIFIED",
                Type::Gvisor => "GVISOR",
            }
        }
    }
}
/// EphemeralStorageConfig contains configuration for the ephemeral storage
/// filesystem.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EphemeralStorageConfig {
    /// Number of local SSDs to use to back ephemeral storage. Uses NVMe
    /// interfaces. Each local SSD is 375 GB in size.
    /// If zero, it means to disable using local SSDs as ephemeral storage.
    #[prost(int32, tag = "1")]
    pub local_ssd_count: i32,
}
/// GcfsConfig contains configurations of Google Container File System.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GcfsConfig {
    /// Whether to use GCFS.
    #[prost(bool, tag = "1")]
    pub enabled: bool,
}
/// \[ReservationAffinity\](<https://cloud.google.com/compute/docs/instances/reserving-zonal-resources>)
/// is the configuration of desired reservation which instances could take
/// capacity from.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReservationAffinity {
    /// Corresponds to the type of reservation consumption.
    #[prost(enumeration = "reservation_affinity::Type", tag = "1")]
    pub consume_reservation_type: i32,
    /// Corresponds to the label key of a reservation resource. To target a
    /// SPECIFIC_RESERVATION by name, specify
    /// "compute.googleapis.com/reservation-name" as the key and specify the name
    /// of your reservation as its value.
    #[prost(string, tag = "2")]
    pub key: ::prost::alloc::string::String,
    /// Corresponds to the label value(s) of reservation resource(s).
    #[prost(string, repeated, tag = "3")]
    pub values: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Nested message and enum types in `ReservationAffinity`.
pub mod reservation_affinity {
    /// Indicates whether to consume capacity from a reservation or not.
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
        /// Default value. This should not be used.
        Unspecified = 0,
        /// Do not consume from any reserved capacity.
        NoReservation = 1,
        /// Consume any reservation available.
        AnyReservation = 2,
        /// Must consume from a specific reservation. Must specify key value fields
        /// for specifying the reservations.
        SpecificReservation = 3,
    }
    impl Type {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Type::Unspecified => "UNSPECIFIED",
                Type::NoReservation => "NO_RESERVATION",
                Type::AnyReservation => "ANY_RESERVATION",
                Type::SpecificReservation => "SPECIFIC_RESERVATION",
            }
        }
    }
}
/// Kubernetes taint is comprised of three fields: key, value, and effect. Effect
/// can only be one of three types:  NoSchedule, PreferNoSchedule or NoExecute.
///
/// See
/// \[here\](<https://kubernetes.io/docs/concepts/configuration/taint-and-toleration>)
/// for more information, including usage and the valid values.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NodeTaint {
    /// Key for taint.
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
    /// Value for taint.
    #[prost(string, tag = "2")]
    pub value: ::prost::alloc::string::String,
    /// Effect for taint.
    #[prost(enumeration = "node_taint::Effect", tag = "3")]
    pub effect: i32,
}
/// Nested message and enum types in `NodeTaint`.
pub mod node_taint {
    /// Possible values for Effect in taint.
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
    pub enum Effect {
        /// Not set
        Unspecified = 0,
        /// NoSchedule
        NoSchedule = 1,
        /// PreferNoSchedule
        PreferNoSchedule = 2,
        /// NoExecute
        NoExecute = 3,
    }
    impl Effect {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Effect::Unspecified => "EFFECT_UNSPECIFIED",
                Effect::NoSchedule => "NO_SCHEDULE",
                Effect::PreferNoSchedule => "PREFER_NO_SCHEDULE",
                Effect::NoExecute => "NO_EXECUTE",
            }
        }
    }
}
/// Collection of Kubernetes [node
/// taints](<https://kubernetes.io/docs/concepts/configuration/taint-and-toleration>).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NodeTaints {
    /// List of node taints.
    #[prost(message, repeated, tag = "1")]
    pub taints: ::prost::alloc::vec::Vec<NodeTaint>,
}
/// Collection of node-level [Kubernetes
/// labels](<https://kubernetes.io/docs/concepts/overview/working-with-objects/labels>).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NodeLabels {
    /// Map of node label keys and node label values.
    #[prost(map = "string, string", tag = "1")]
    pub labels: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
}
/// Collection of [GCP
/// labels](<https://cloud.google.com/resource-manager/docs/creating-managing-labels>).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResourceLabels {
    /// Map of node label keys and node label values.
    #[prost(map = "string, string", tag = "1")]
    pub labels: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
}
/// Collection of Compute Engine network tags that can be applied to a node's
/// underlying VM instance. (See `tags` field in
/// \[`NodeConfig`\](/kubernetes-engine/docs/reference/rest/v1/NodeConfig)).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NetworkTags {
    /// List of network tags.
    #[prost(string, repeated, tag = "1")]
    pub tags: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// The authentication information for accessing the master endpoint.
/// Authentication can be done using HTTP basic auth or using client
/// certificates.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MasterAuth {
    /// The username to use for HTTP basic authentication to the master endpoint.
    /// For clusters v1.6.0 and later, basic authentication can be disabled by
    /// leaving username unspecified (or setting it to the empty string).
    ///
    /// Warning: basic authentication is deprecated, and will be removed in GKE
    /// control plane versions 1.19 and newer. For a list of recommended
    /// authentication methods, see:
    /// <https://cloud.google.com/kubernetes-engine/docs/how-to/api-server-authentication>
    #[deprecated]
    #[prost(string, tag = "1")]
    pub username: ::prost::alloc::string::String,
    /// The password to use for HTTP basic authentication to the master endpoint.
    /// Because the master endpoint is open to the Internet, you should create a
    /// strong password.  If a password is provided for cluster creation, username
    /// must be non-empty.
    ///
    /// Warning: basic authentication is deprecated, and will be removed in GKE
    /// control plane versions 1.19 and newer. For a list of recommended
    /// authentication methods, see:
    /// <https://cloud.google.com/kubernetes-engine/docs/how-to/api-server-authentication>
    #[deprecated]
    #[prost(string, tag = "2")]
    pub password: ::prost::alloc::string::String,
    /// Configuration for client certificate authentication on the cluster. For
    /// clusters before v1.12, if no configuration is specified, a client
    /// certificate is issued.
    #[prost(message, optional, tag = "3")]
    pub client_certificate_config: ::core::option::Option<ClientCertificateConfig>,
    #[prost(string, tag = "100")]
    pub cluster_ca_certificate: ::prost::alloc::string::String,
    /// [Output only] Base64-encoded public certificate used by clients to
    /// authenticate to the cluster endpoint.
    #[prost(string, tag = "101")]
    pub client_certificate: ::prost::alloc::string::String,
    /// [Output only] Base64-encoded private key used by clients to authenticate
    /// to the cluster endpoint.
    #[prost(string, tag = "102")]
    pub client_key: ::prost::alloc::string::String,
}
/// Configuration for client certificates on the cluster.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClientCertificateConfig {
    /// Issue a client certificate.
    #[prost(bool, tag = "1")]
    pub issue_client_certificate: bool,
}
/// Configuration for the addons that can be automatically spun up in the
/// cluster, enabling additional functionality.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddonsConfig {
    /// Configuration for the HTTP (L7) load balancing controller addon, which
    /// makes it easy to set up HTTP load balancers for services in a cluster.
    #[prost(message, optional, tag = "1")]
    pub http_load_balancing: ::core::option::Option<HttpLoadBalancing>,
    /// Configuration for the horizontal pod autoscaling feature, which
    /// increases or decreases the number of replica pods a replication controller
    /// has based on the resource usage of the existing pods.
    #[prost(message, optional, tag = "2")]
    pub horizontal_pod_autoscaling: ::core::option::Option<HorizontalPodAutoscaling>,
    /// Configuration for the Kubernetes Dashboard.
    /// This addon is deprecated, and will be disabled in 1.15. It is recommended
    /// to use the Cloud Console to manage and monitor your Kubernetes clusters,
    /// workloads and applications. For more information, see:
    /// <https://cloud.google.com/kubernetes-engine/docs/concepts/dashboards>
    #[deprecated]
    #[prost(message, optional, tag = "3")]
    pub kubernetes_dashboard: ::core::option::Option<KubernetesDashboard>,
    /// Configuration for NetworkPolicy. This only tracks whether the addon
    /// is enabled or not on the Master, it does not track whether network policy
    /// is enabled for the nodes.
    #[prost(message, optional, tag = "4")]
    pub network_policy_config: ::core::option::Option<NetworkPolicyConfig>,
    /// Configuration for Istio, an open platform to connect, manage, and secure
    /// microservices.
    #[deprecated]
    #[prost(message, optional, tag = "5")]
    pub istio_config: ::core::option::Option<IstioConfig>,
    /// Configuration for the Cloud Run addon. The `IstioConfig` addon must be
    /// enabled in order to enable Cloud Run addon. This option can only be enabled
    /// at cluster creation time.
    #[prost(message, optional, tag = "7")]
    pub cloud_run_config: ::core::option::Option<CloudRunConfig>,
    /// Configuration for NodeLocalDNS, a dns cache running on cluster nodes
    #[prost(message, optional, tag = "8")]
    pub dns_cache_config: ::core::option::Option<DnsCacheConfig>,
    /// Configuration for the ConfigConnector add-on, a Kubernetes
    /// extension to manage hosted GCP services through the Kubernetes API
    #[prost(message, optional, tag = "10")]
    pub config_connector_config: ::core::option::Option<ConfigConnectorConfig>,
    /// Configuration for the Compute Engine Persistent Disk CSI driver.
    #[prost(message, optional, tag = "11")]
    pub gce_persistent_disk_csi_driver_config: ::core::option::Option<
        GcePersistentDiskCsiDriverConfig,
    >,
    /// Configuration for the KALM addon, which manages the lifecycle of k8s
    /// applications.
    #[deprecated]
    #[prost(message, optional, tag = "12")]
    pub kalm_config: ::core::option::Option<KalmConfig>,
    /// Configuration for the GCP Filestore CSI driver.
    #[prost(message, optional, tag = "14")]
    pub gcp_filestore_csi_driver_config: ::core::option::Option<
        GcpFilestoreCsiDriverConfig,
    >,
    /// Configuration for the Backup for GKE agent addon.
    #[prost(message, optional, tag = "16")]
    pub gke_backup_agent_config: ::core::option::Option<GkeBackupAgentConfig>,
}
/// Configuration options for the HTTP (L7) load balancing controller addon,
/// which makes it easy to set up HTTP load balancers for services in a cluster.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HttpLoadBalancing {
    /// Whether the HTTP Load Balancing controller is enabled in the cluster.
    /// When enabled, it runs a small pod in the cluster that manages the load
    /// balancers.
    #[prost(bool, tag = "1")]
    pub disabled: bool,
}
/// Configuration options for the horizontal pod autoscaling feature, which
/// increases or decreases the number of replica pods a replication controller
/// has based on the resource usage of the existing pods.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HorizontalPodAutoscaling {
    /// Whether the Horizontal Pod Autoscaling feature is enabled in the cluster.
    /// When enabled, it ensures that metrics are collected into Stackdriver
    /// Monitoring.
    #[prost(bool, tag = "1")]
    pub disabled: bool,
}
/// Configuration for the Kubernetes Dashboard.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KubernetesDashboard {
    /// Whether the Kubernetes Dashboard is enabled for this cluster.
    #[prost(bool, tag = "1")]
    pub disabled: bool,
}
/// Configuration for NetworkPolicy. This only tracks whether the addon
/// is enabled or not on the Master, it does not track whether network policy
/// is enabled for the nodes.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NetworkPolicyConfig {
    /// Whether NetworkPolicy is enabled for this cluster.
    #[prost(bool, tag = "1")]
    pub disabled: bool,
}
/// Configuration for NodeLocal DNSCache
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DnsCacheConfig {
    /// Whether NodeLocal DNSCache is enabled for this cluster.
    #[prost(bool, tag = "1")]
    pub enabled: bool,
}
/// Configuration options for the KALM addon.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KalmConfig {
    /// Whether KALM is enabled for this cluster.
    #[deprecated]
    #[prost(bool, tag = "1")]
    pub enabled: bool,
}
/// Configuration for the Backup for GKE Agent.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GkeBackupAgentConfig {
    /// Whether the Backup for GKE agent is enabled for this cluster.
    #[prost(bool, tag = "1")]
    pub enabled: bool,
}
/// Configuration options for the Config Connector add-on.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConfigConnectorConfig {
    /// Whether Cloud Connector is enabled for this cluster.
    #[prost(bool, tag = "1")]
    pub enabled: bool,
}
/// Configuration for the Compute Engine PD CSI driver.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GcePersistentDiskCsiDriverConfig {
    /// Whether the Compute Engine PD CSI driver is enabled for this cluster.
    #[prost(bool, tag = "1")]
    pub enabled: bool,
}
/// Configuration for the GCP Filestore CSI driver.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GcpFilestoreCsiDriverConfig {
    /// Whether the GCP Filestore CSI driver is enabled for this cluster.
    #[prost(bool, tag = "1")]
    pub enabled: bool,
}
/// Configuration for controlling master global access settings.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrivateClusterMasterGlobalAccessConfig {
    /// Whenever master is accessible globally or not.
    #[prost(bool, tag = "1")]
    pub enabled: bool,
}
/// Configuration options for private clusters.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrivateClusterConfig {
    /// Whether nodes have internal IP addresses only. If enabled, all nodes are
    /// given only RFC 1918 private addresses and communicate with the master via
    /// private networking.
    #[prost(bool, tag = "1")]
    pub enable_private_nodes: bool,
    /// Whether the master's internal IP address is used as the cluster endpoint.
    #[prost(bool, tag = "2")]
    pub enable_private_endpoint: bool,
    /// The IP range in CIDR notation to use for the hosted master network. This
    /// range will be used for assigning internal IP addresses to the master or
    /// set of masters, as well as the ILB VIP. This range must not overlap with
    /// any other ranges in use within the cluster's network.
    #[prost(string, tag = "3")]
    pub master_ipv4_cidr_block: ::prost::alloc::string::String,
    /// Output only. The internal IP address of this cluster's master endpoint.
    #[prost(string, tag = "4")]
    pub private_endpoint: ::prost::alloc::string::String,
    /// Output only. The external IP address of this cluster's master endpoint.
    #[prost(string, tag = "5")]
    pub public_endpoint: ::prost::alloc::string::String,
    /// Output only. The peering name in the customer VPC used by this cluster.
    #[prost(string, tag = "7")]
    pub peering_name: ::prost::alloc::string::String,
    /// Controls master global access settings.
    #[prost(message, optional, tag = "8")]
    pub master_global_access_config: ::core::option::Option<
        PrivateClusterMasterGlobalAccessConfig,
    >,
    /// Subnet to provision the master's private endpoint during cluster creation.
    /// Specified in projects/*/regions/*/subnetworks/* format.
    #[prost(string, tag = "10")]
    pub private_endpoint_subnetwork: ::prost::alloc::string::String,
}
/// Configuration options for Istio addon.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IstioConfig {
    /// Whether Istio is enabled for this cluster.
    #[deprecated]
    #[prost(bool, tag = "1")]
    pub disabled: bool,
    /// The specified Istio auth mode, either none, or mutual TLS.
    #[deprecated]
    #[prost(enumeration = "istio_config::IstioAuthMode", tag = "2")]
    pub auth: i32,
}
/// Nested message and enum types in `IstioConfig`.
pub mod istio_config {
    /// Istio auth mode, <https://istio.io/docs/concepts/security/mutual-tls.html>
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
    pub enum IstioAuthMode {
        /// auth not enabled
        AuthNone = 0,
        /// auth mutual TLS enabled
        AuthMutualTls = 1,
    }
    impl IstioAuthMode {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                IstioAuthMode::AuthNone => "AUTH_NONE",
                IstioAuthMode::AuthMutualTls => "AUTH_MUTUAL_TLS",
            }
        }
    }
}
/// Configuration options for the Cloud Run feature.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CloudRunConfig {
    /// Whether Cloud Run addon is enabled for this cluster.
    #[prost(bool, tag = "1")]
    pub disabled: bool,
    /// Which load balancer type is installed for Cloud Run.
    #[prost(enumeration = "cloud_run_config::LoadBalancerType", tag = "3")]
    pub load_balancer_type: i32,
}
/// Nested message and enum types in `CloudRunConfig`.
pub mod cloud_run_config {
    /// Load balancer type of ingress service of Cloud Run.
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
    pub enum LoadBalancerType {
        /// Load balancer type for Cloud Run is unspecified.
        Unspecified = 0,
        /// Install external load balancer for Cloud Run.
        External = 1,
        /// Install internal load balancer for Cloud Run.
        Internal = 2,
    }
    impl LoadBalancerType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                LoadBalancerType::Unspecified => "LOAD_BALANCER_TYPE_UNSPECIFIED",
                LoadBalancerType::External => "LOAD_BALANCER_TYPE_EXTERNAL",
                LoadBalancerType::Internal => "LOAD_BALANCER_TYPE_INTERNAL",
            }
        }
    }
}
/// Configuration options for the master authorized networks feature. Enabled
/// master authorized networks will disallow all external traffic to access
/// Kubernetes master through HTTPS except traffic from the given CIDR blocks,
/// Google Compute Engine Public IPs and Google Prod IPs.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MasterAuthorizedNetworksConfig {
    /// Whether or not master authorized networks is enabled.
    #[prost(bool, tag = "1")]
    pub enabled: bool,
    /// cidr_blocks define up to 10 external networks that could access
    /// Kubernetes master through HTTPS.
    #[prost(message, repeated, tag = "2")]
    pub cidr_blocks: ::prost::alloc::vec::Vec<
        master_authorized_networks_config::CidrBlock,
    >,
    /// Whether master is accessbile via Google Compute Engine Public IP addresses.
    #[prost(bool, optional, tag = "3")]
    pub gcp_public_cidrs_access_enabled: ::core::option::Option<bool>,
}
/// Nested message and enum types in `MasterAuthorizedNetworksConfig`.
pub mod master_authorized_networks_config {
    /// CidrBlock contains an optional name and one CIDR block.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CidrBlock {
        /// display_name is an optional field for users to identify CIDR blocks.
        #[prost(string, tag = "1")]
        pub display_name: ::prost::alloc::string::String,
        /// cidr_block must be specified in CIDR notation.
        #[prost(string, tag = "2")]
        pub cidr_block: ::prost::alloc::string::String,
    }
}
/// Configuration for the legacy Attribute Based Access Control authorization
/// mode.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LegacyAbac {
    /// Whether the ABAC authorizer is enabled for this cluster. When enabled,
    /// identities in the system, including service accounts, nodes, and
    /// controllers, will have statically granted permissions beyond those
    /// provided by the RBAC configuration or IAM.
    #[prost(bool, tag = "1")]
    pub enabled: bool,
}
/// Configuration options for the NetworkPolicy feature.
/// <https://kubernetes.io/docs/concepts/services-networking/networkpolicies/>
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NetworkPolicy {
    /// The selected network policy provider.
    #[prost(enumeration = "network_policy::Provider", tag = "1")]
    pub provider: i32,
    /// Whether network policy is enabled on the cluster.
    #[prost(bool, tag = "2")]
    pub enabled: bool,
}
/// Nested message and enum types in `NetworkPolicy`.
pub mod network_policy {
    /// Allowed Network Policy providers.
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
    pub enum Provider {
        /// Not set
        Unspecified = 0,
        /// Tigera (Calico Felix).
        Calico = 1,
    }
    impl Provider {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Provider::Unspecified => "PROVIDER_UNSPECIFIED",
                Provider::Calico => "CALICO",
            }
        }
    }
}
/// Configuration for controlling how IPs are allocated in the cluster.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IpAllocationPolicy {
    /// Whether alias IPs will be used for pod IPs in the cluster.
    /// This is used in conjunction with use_routes. It cannot
    /// be true if use_routes is true. If both use_ip_aliases and use_routes are
    /// false, then the server picks the default IP allocation mode
    #[prost(bool, tag = "1")]
    pub use_ip_aliases: bool,
    /// Whether a new subnetwork will be created automatically for the cluster.
    ///
    /// This field is only applicable when `use_ip_aliases` is true.
    #[prost(bool, tag = "2")]
    pub create_subnetwork: bool,
    /// A custom subnetwork name to be used if `create_subnetwork` is true.  If
    /// this field is empty, then an automatic name will be chosen for the new
    /// subnetwork.
    #[prost(string, tag = "3")]
    pub subnetwork_name: ::prost::alloc::string::String,
    /// This field is deprecated, use cluster_ipv4_cidr_block.
    #[deprecated]
    #[prost(string, tag = "4")]
    pub cluster_ipv4_cidr: ::prost::alloc::string::String,
    /// This field is deprecated, use node_ipv4_cidr_block.
    #[deprecated]
    #[prost(string, tag = "5")]
    pub node_ipv4_cidr: ::prost::alloc::string::String,
    /// This field is deprecated, use services_ipv4_cidr_block.
    #[deprecated]
    #[prost(string, tag = "6")]
    pub services_ipv4_cidr: ::prost::alloc::string::String,
    /// The name of the secondary range to be used for the cluster CIDR
    /// block.  The secondary range will be used for pod IP
    /// addresses. This must be an existing secondary range associated
    /// with the cluster subnetwork.
    ///
    /// This field is only applicable with use_ip_aliases and
    /// create_subnetwork is false.
    #[prost(string, tag = "7")]
    pub cluster_secondary_range_name: ::prost::alloc::string::String,
    /// The name of the secondary range to be used as for the services
    /// CIDR block.  The secondary range will be used for service
    /// ClusterIPs. This must be an existing secondary range associated
    /// with the cluster subnetwork.
    ///
    /// This field is only applicable with use_ip_aliases and
    /// create_subnetwork is false.
    #[prost(string, tag = "8")]
    pub services_secondary_range_name: ::prost::alloc::string::String,
    /// The IP address range for the cluster pod IPs. If this field is set, then
    /// `cluster.cluster_ipv4_cidr` must be left blank.
    ///
    /// This field is only applicable when `use_ip_aliases` is true.
    ///
    /// Set to blank to have a range chosen with the default size.
    ///
    /// Set to /netmask (e.g. `/14`) to have a range chosen with a specific
    /// netmask.
    ///
    /// Set to a
    /// \[CIDR\](<http://en.wikipedia.org/wiki/Classless_Inter-Domain_Routing>)
    /// notation (e.g. `10.96.0.0/14`) from the RFC-1918 private networks (e.g.
    /// `10.0.0.0/8`, `172.16.0.0/12`, `192.168.0.0/16`) to pick a specific range
    /// to use.
    #[prost(string, tag = "9")]
    pub cluster_ipv4_cidr_block: ::prost::alloc::string::String,
    /// The IP address range of the instance IPs in this cluster.
    ///
    /// This is applicable only if `create_subnetwork` is true.
    ///
    /// Set to blank to have a range chosen with the default size.
    ///
    /// Set to /netmask (e.g. `/14`) to have a range chosen with a specific
    /// netmask.
    ///
    /// Set to a
    /// \[CIDR\](<http://en.wikipedia.org/wiki/Classless_Inter-Domain_Routing>)
    /// notation (e.g. `10.96.0.0/14`) from the RFC-1918 private networks (e.g.
    /// `10.0.0.0/8`, `172.16.0.0/12`, `192.168.0.0/16`) to pick a specific range
    /// to use.
    #[prost(string, tag = "10")]
    pub node_ipv4_cidr_block: ::prost::alloc::string::String,
    /// The IP address range of the services IPs in this cluster. If blank, a range
    /// will be automatically chosen with the default size.
    ///
    /// This field is only applicable when `use_ip_aliases` is true.
    ///
    /// Set to blank to have a range chosen with the default size.
    ///
    /// Set to /netmask (e.g. `/14`) to have a range chosen with a specific
    /// netmask.
    ///
    /// Set to a
    /// \[CIDR\](<http://en.wikipedia.org/wiki/Classless_Inter-Domain_Routing>)
    /// notation (e.g. `10.96.0.0/14`) from the RFC-1918 private networks (e.g.
    /// `10.0.0.0/8`, `172.16.0.0/12`, `192.168.0.0/16`) to pick a specific range
    /// to use.
    #[prost(string, tag = "11")]
    pub services_ipv4_cidr_block: ::prost::alloc::string::String,
    /// If true, allow allocation of cluster CIDR ranges that overlap with certain
    /// kinds of network routes. By default we do not allow cluster CIDR ranges to
    /// intersect with any user declared routes. With allow_route_overlap == true,
    /// we allow overlapping with CIDR ranges that are larger than the cluster CIDR
    /// range.
    ///
    /// If this field is set to true, then cluster and services CIDRs must be
    /// fully-specified (e.g. `10.96.0.0/14`, but not `/14`), which means:
    /// 1) When `use_ip_aliases` is true, `cluster_ipv4_cidr_block` and
    ///     `services_ipv4_cidr_block` must be fully-specified.
    /// 2) When `use_ip_aliases` is false, `cluster.cluster_ipv4_cidr` muse be
    ///     fully-specified.
    #[prost(bool, tag = "12")]
    pub allow_route_overlap: bool,
    /// The IP address range of the Cloud TPUs in this cluster. If unspecified, a
    /// range will be automatically chosen with the default size.
    ///
    /// This field is only applicable when `use_ip_aliases` is true.
    ///
    /// If unspecified, the range will use the default size.
    ///
    /// Set to /netmask (e.g. `/14`) to have a range chosen with a specific
    /// netmask.
    ///
    /// Set to a
    /// \[CIDR\](<http://en.wikipedia.org/wiki/Classless_Inter-Domain_Routing>)
    /// notation (e.g. `10.96.0.0/14`) from the RFC-1918 private networks (e.g.
    /// `10.0.0.0/8`, `172.16.0.0/12`, `192.168.0.0/16`) to pick a specific range
    /// to use.
    /// This field is deprecated, use cluster.tpu_config.ipv4_cidr_block instead.
    #[prost(string, tag = "13")]
    pub tpu_ipv4_cidr_block: ::prost::alloc::string::String,
    /// Whether routes will be used for pod IPs in the cluster.
    /// This is used in conjunction with use_ip_aliases. It cannot be true if
    /// use_ip_aliases is true. If both use_ip_aliases and use_routes are false,
    /// then the server picks the default IP allocation mode
    #[prost(bool, tag = "15")]
    pub use_routes: bool,
    /// IP stack type
    #[prost(enumeration = "ip_allocation_policy::StackType", tag = "16")]
    pub stack_type: i32,
    /// The ipv6 access type (internal or external) when create_subnetwork is true
    #[prost(enumeration = "ip_allocation_policy::IPv6AccessType", tag = "17")]
    pub ipv6_access_type: i32,
    /// Output only. [Output only] The subnet's IPv6 CIDR block used by nodes and pods.
    #[prost(string, tag = "22")]
    pub subnet_ipv6_cidr_block: ::prost::alloc::string::String,
    /// Output only. [Output only] The services IPv6 CIDR block for the cluster.
    #[prost(string, tag = "23")]
    pub services_ipv6_cidr_block: ::prost::alloc::string::String,
}
/// Nested message and enum types in `IPAllocationPolicy`.
pub mod ip_allocation_policy {
    /// Possible values for IP stack type
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
    pub enum StackType {
        /// By default, the clusters will be IPV4 only
        Unspecified = 0,
        /// The value used if the cluster is a IPV4 only
        Ipv4 = 1,
        /// The value used if the cluster is a dual stack cluster
        Ipv4Ipv6 = 2,
    }
    impl StackType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                StackType::Unspecified => "STACK_TYPE_UNSPECIFIED",
                StackType::Ipv4 => "IPV4",
                StackType::Ipv4Ipv6 => "IPV4_IPV6",
            }
        }
    }
    /// IPv6 access type
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
    pub enum IPv6AccessType {
        /// Default value, will be defaulted as type external.
        Ipv6AccessTypeUnspecified = 0,
        /// Access type internal (all v6 addresses are internal IPs)
        Internal = 1,
        /// Access type external (all v6 addresses are external IPs)
        External = 2,
    }
    impl IPv6AccessType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                IPv6AccessType::Ipv6AccessTypeUnspecified => {
                    "IPV6_ACCESS_TYPE_UNSPECIFIED"
                }
                IPv6AccessType::Internal => "INTERNAL",
                IPv6AccessType::External => "EXTERNAL",
            }
        }
    }
}
/// Configuration for Binary Authorization.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BinaryAuthorization {
    /// This field is deprecated. Leave this unset and instead configure
    /// BinaryAuthorization using evaluation_mode. If evaluation_mode is set to
    /// anything other than EVALUATION_MODE_UNSPECIFIED, this field is ignored.
    #[deprecated]
    #[prost(bool, tag = "1")]
    pub enabled: bool,
    /// Mode of operation for binauthz policy evaluation. If unspecified, defaults
    /// to DISABLED.
    #[prost(enumeration = "binary_authorization::EvaluationMode", tag = "2")]
    pub evaluation_mode: i32,
}
/// Nested message and enum types in `BinaryAuthorization`.
pub mod binary_authorization {
    /// Binary Authorization mode of operation.
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
    pub enum EvaluationMode {
        /// Default value
        Unspecified = 0,
        /// Disable BinaryAuthorization
        Disabled = 1,
        /// Enforce Kubernetes admission requests with BinaryAuthorization using the
        /// project's singleton policy. This is equivalent to setting the
        /// enabled boolean to true.
        ProjectSingletonPolicyEnforce = 2,
    }
    impl EvaluationMode {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                EvaluationMode::Unspecified => "EVALUATION_MODE_UNSPECIFIED",
                EvaluationMode::Disabled => "DISABLED",
                EvaluationMode::ProjectSingletonPolicyEnforce => {
                    "PROJECT_SINGLETON_POLICY_ENFORCE"
                }
            }
        }
    }
}
/// Configuration for the PodSecurityPolicy feature.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PodSecurityPolicyConfig {
    /// Enable the PodSecurityPolicy controller for this cluster. If enabled, pods
    /// must be valid under a PodSecurityPolicy to be created.
    #[prost(bool, tag = "1")]
    pub enabled: bool,
}
/// Configuration for returning group information from authenticators.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthenticatorGroupsConfig {
    /// Whether this cluster should return group membership lookups
    /// during authentication using a group of security groups.
    #[prost(bool, tag = "1")]
    pub enabled: bool,
    /// The name of the security group-of-groups to be used. Only relevant
    /// if enabled = true.
    #[prost(string, tag = "2")]
    pub security_group: ::prost::alloc::string::String,
}
/// Telemetry integration for the cluster.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClusterTelemetry {
    /// Type of the integration.
    #[prost(enumeration = "cluster_telemetry::Type", tag = "1")]
    pub r#type: i32,
}
/// Nested message and enum types in `ClusterTelemetry`.
pub mod cluster_telemetry {
    /// Type of the integration.
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
        /// Not set.
        Unspecified = 0,
        /// Monitoring integration is disabled.
        Disabled = 1,
        /// Monitoring integration is enabled.
        Enabled = 2,
        /// Only system components are monitored and logged.
        SystemOnly = 3,
    }
    impl Type {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Type::Unspecified => "UNSPECIFIED",
                Type::Disabled => "DISABLED",
                Type::Enabled => "ENABLED",
                Type::SystemOnly => "SYSTEM_ONLY",
            }
        }
    }
}
/// A Google Kubernetes Engine cluster.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Cluster {
    /// The name of this cluster. The name must be unique within this project
    /// and location (e.g. zone or region), and can be up to 40 characters with
    /// the following restrictions:
    ///
    /// * Lowercase letters, numbers, and hyphens only.
    /// * Must start with a letter.
    /// * Must end with a number or a letter.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// An optional description of this cluster.
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// The number of nodes to create in this cluster. You must ensure that your
    /// Compute Engine [resource quota](<https://cloud.google.com/compute/quotas>)
    /// is sufficient for this number of instances. You must also have available
    /// firewall and routes quota.
    /// For requests, this field should only be used in lieu of a
    /// "node_pool" object, since this configuration (along with the
    /// "node_config") will be used to create a "NodePool" object with an
    /// auto-generated name. Do not use this and a node_pool at the same time.
    ///
    /// This field is deprecated, use node_pool.initial_node_count instead.
    #[deprecated]
    #[prost(int32, tag = "3")]
    pub initial_node_count: i32,
    /// Parameters used in creating the cluster's nodes.
    /// For requests, this field should only be used in lieu of a
    /// "node_pool" object, since this configuration (along with the
    /// "initial_node_count") will be used to create a "NodePool" object with an
    /// auto-generated name. Do not use this and a node_pool at the same time.
    /// For responses, this field will be populated with the node configuration of
    /// the first node pool. (For configuration of each node pool, see
    /// `node_pool.config`)
    ///
    /// If unspecified, the defaults are used.
    /// This field is deprecated, use node_pool.config instead.
    #[deprecated]
    #[prost(message, optional, tag = "4")]
    pub node_config: ::core::option::Option<NodeConfig>,
    /// The authentication information for accessing the master endpoint.
    /// If unspecified, the defaults are used:
    /// For clusters before v1.12, if master_auth is unspecified, `username` will
    /// be set to "admin", a random password will be generated, and a client
    /// certificate will be issued.
    #[prost(message, optional, tag = "5")]
    pub master_auth: ::core::option::Option<MasterAuth>,
    /// The logging service the cluster should use to write logs.
    /// Currently available options:
    ///
    /// * `logging.googleapis.com/kubernetes` - The Cloud Logging
    /// service with a Kubernetes-native resource model
    /// * `logging.googleapis.com` - The legacy Cloud Logging service (no longer
    ///    available as of GKE 1.15).
    /// * `none` - no logs will be exported from the cluster.
    ///
    /// If left as an empty string,`logging.googleapis.com/kubernetes` will be
    /// used for GKE 1.14+ or `logging.googleapis.com` for earlier versions.
    #[prost(string, tag = "6")]
    pub logging_service: ::prost::alloc::string::String,
    /// The monitoring service the cluster should use to write metrics.
    /// Currently available options:
    ///
    /// * "monitoring.googleapis.com/kubernetes" - The Cloud Monitoring
    /// service with a Kubernetes-native resource model
    /// * `monitoring.googleapis.com` - The legacy Cloud Monitoring service (no
    ///    longer available as of GKE 1.15).
    /// * `none` - No metrics will be exported from the cluster.
    ///
    /// If left as an empty string,`monitoring.googleapis.com/kubernetes` will be
    /// used for GKE 1.14+ or `monitoring.googleapis.com` for earlier versions.
    #[prost(string, tag = "7")]
    pub monitoring_service: ::prost::alloc::string::String,
    /// The name of the Google Compute Engine
    /// \[network\](<https://cloud.google.com/compute/docs/networks-and-firewalls#networks>)
    /// to which the cluster is connected. If left unspecified, the `default`
    /// network will be used. On output this shows the network ID instead of the
    /// name.
    #[prost(string, tag = "8")]
    pub network: ::prost::alloc::string::String,
    /// The IP address range of the container pods in this cluster, in
    /// \[CIDR\](<http://en.wikipedia.org/wiki/Classless_Inter-Domain_Routing>)
    /// notation (e.g. `10.96.0.0/14`). Leave blank to have
    /// one automatically chosen or specify a `/14` block in `10.0.0.0/8`.
    #[prost(string, tag = "9")]
    pub cluster_ipv4_cidr: ::prost::alloc::string::String,
    /// Configurations for the various addons available to run in the cluster.
    #[prost(message, optional, tag = "10")]
    pub addons_config: ::core::option::Option<AddonsConfig>,
    /// The name of the Google Compute Engine
    /// \[subnetwork\](<https://cloud.google.com/compute/docs/subnetworks>) to which
    /// the cluster is connected. On output this shows the subnetwork ID instead of
    /// the name.
    #[prost(string, tag = "11")]
    pub subnetwork: ::prost::alloc::string::String,
    /// The node pools associated with this cluster.
    /// This field should not be set if "node_config" or "initial_node_count" are
    /// specified.
    #[prost(message, repeated, tag = "12")]
    pub node_pools: ::prost::alloc::vec::Vec<NodePool>,
    /// The list of Google Compute Engine
    /// \[zones\](<https://cloud.google.com/compute/docs/zones#available>) in which the
    /// cluster's nodes should be located.
    ///
    /// This field provides a default value if
    /// \[NodePool.Locations\](<https://cloud.google.com/kubernetes-engine/docs/reference/rest/v1/projects.locations.clusters.nodePools#NodePool.FIELDS.locations>)
    /// are not specified during node pool creation.
    ///
    /// Warning: changing cluster locations will update the
    /// \[NodePool.Locations\](<https://cloud.google.com/kubernetes-engine/docs/reference/rest/v1/projects.locations.clusters.nodePools#NodePool.FIELDS.locations>)
    /// of all node pools and will result in nodes being added and/or removed.
    #[prost(string, repeated, tag = "13")]
    pub locations: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Kubernetes alpha features are enabled on this cluster. This includes alpha
    /// API groups (e.g. v1beta1) and features that may not be production ready in
    /// the kubernetes version of the master and nodes.
    /// The cluster has no SLA for uptime and master/node upgrades are disabled.
    /// Alpha enabled clusters are automatically deleted thirty days after
    /// creation.
    #[prost(bool, tag = "14")]
    pub enable_kubernetes_alpha: bool,
    /// The resource labels for the cluster to use to annotate any related
    /// Google Compute Engine resources.
    #[prost(map = "string, string", tag = "15")]
    pub resource_labels: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// The fingerprint of the set of labels for this cluster.
    #[prost(string, tag = "16")]
    pub label_fingerprint: ::prost::alloc::string::String,
    /// Configuration for the legacy ABAC authorization mode.
    #[prost(message, optional, tag = "18")]
    pub legacy_abac: ::core::option::Option<LegacyAbac>,
    /// Configuration options for the NetworkPolicy feature.
    #[prost(message, optional, tag = "19")]
    pub network_policy: ::core::option::Option<NetworkPolicy>,
    /// Configuration for cluster IP allocation.
    #[prost(message, optional, tag = "20")]
    pub ip_allocation_policy: ::core::option::Option<IpAllocationPolicy>,
    /// The configuration options for master authorized networks feature.
    #[prost(message, optional, tag = "22")]
    pub master_authorized_networks_config: ::core::option::Option<
        MasterAuthorizedNetworksConfig,
    >,
    /// Configure the maintenance policy for this cluster.
    #[prost(message, optional, tag = "23")]
    pub maintenance_policy: ::core::option::Option<MaintenancePolicy>,
    /// Configuration for Binary Authorization.
    #[prost(message, optional, tag = "24")]
    pub binary_authorization: ::core::option::Option<BinaryAuthorization>,
    /// Configuration for the PodSecurityPolicy feature.
    #[prost(message, optional, tag = "25")]
    pub pod_security_policy_config: ::core::option::Option<PodSecurityPolicyConfig>,
    /// Cluster-level autoscaling configuration.
    #[prost(message, optional, tag = "26")]
    pub autoscaling: ::core::option::Option<ClusterAutoscaling>,
    /// Configuration for cluster networking.
    #[prost(message, optional, tag = "27")]
    pub network_config: ::core::option::Option<NetworkConfig>,
    /// If this is a private cluster setup. Private clusters are clusters that, by
    /// default have no external IP addresses on the nodes and where nodes and the
    /// master communicate over private IP addresses.
    /// This field is deprecated, use private_cluster_config.enable_private_nodes
    /// instead.
    #[deprecated]
    #[prost(bool, tag = "28")]
    pub private_cluster: bool,
    /// The IP prefix in CIDR notation to use for the hosted master network.
    /// This prefix will be used for assigning private IP addresses to the
    /// master or set of masters, as well as the ILB VIP.
    /// This field is deprecated, use
    /// private_cluster_config.master_ipv4_cidr_block instead.
    #[deprecated]
    #[prost(string, tag = "29")]
    pub master_ipv4_cidr_block: ::prost::alloc::string::String,
    /// The default constraint on the maximum number of pods that can be run
    /// simultaneously on a node in the node pool of this cluster. Only honored
    /// if cluster created with IP Alias support.
    #[prost(message, optional, tag = "30")]
    pub default_max_pods_constraint: ::core::option::Option<MaxPodsConstraint>,
    /// Configuration for exporting resource usages. Resource usage export is
    /// disabled when this config unspecified.
    #[prost(message, optional, tag = "33")]
    pub resource_usage_export_config: ::core::option::Option<ResourceUsageExportConfig>,
    /// Configuration controlling RBAC group membership information.
    #[prost(message, optional, tag = "34")]
    pub authenticator_groups_config: ::core::option::Option<AuthenticatorGroupsConfig>,
    /// Configuration for private cluster.
    #[prost(message, optional, tag = "37")]
    pub private_cluster_config: ::core::option::Option<PrivateClusterConfig>,
    /// Cluster-level Vertical Pod Autoscaling configuration.
    #[prost(message, optional, tag = "39")]
    pub vertical_pod_autoscaling: ::core::option::Option<VerticalPodAutoscaling>,
    /// Shielded Nodes configuration.
    #[prost(message, optional, tag = "40")]
    pub shielded_nodes: ::core::option::Option<ShieldedNodes>,
    /// Release channel configuration.
    #[prost(message, optional, tag = "41")]
    pub release_channel: ::core::option::Option<ReleaseChannel>,
    /// Configuration for the use of Kubernetes Service Accounts in GCP IAM
    /// policies.
    #[prost(message, optional, tag = "43")]
    pub workload_identity_config: ::core::option::Option<WorkloadIdentityConfig>,
    /// Configuration for issuance of mTLS keys and certificates to Kubernetes
    /// pods.
    #[prost(message, optional, tag = "52")]
    pub workload_certificates: ::core::option::Option<WorkloadCertificates>,
    /// Configuration for issuance of mTLS keys and certificates to Kubernetes
    /// pods.
    #[prost(message, optional, tag = "67")]
    pub mesh_certificates: ::core::option::Option<MeshCertificates>,
    /// Configuration for direct-path (via ALTS) with workload identity.
    #[prost(message, optional, tag = "53")]
    pub workload_alts_config: ::core::option::Option<WorkloadAltsConfig>,
    /// Configuration for the fine-grained cost management feature.
    #[prost(message, optional, tag = "45")]
    pub cost_management_config: ::core::option::Option<CostManagementConfig>,
    /// Telemetry integration for the cluster.
    #[prost(message, optional, tag = "46")]
    pub cluster_telemetry: ::core::option::Option<ClusterTelemetry>,
    /// Configuration for Cloud TPU support;
    #[prost(message, optional, tag = "47")]
    pub tpu_config: ::core::option::Option<TpuConfig>,
    /// Notification configuration of the cluster.
    #[prost(message, optional, tag = "49")]
    pub notification_config: ::core::option::Option<NotificationConfig>,
    /// Configuration of Confidential Nodes.
    /// All the nodes in the cluster will be Confidential VM once enabled.
    #[prost(message, optional, tag = "50")]
    pub confidential_nodes: ::core::option::Option<ConfidentialNodes>,
    /// Configuration for Identity Service component.
    #[prost(message, optional, tag = "54")]
    pub identity_service_config: ::core::option::Option<IdentityServiceConfig>,
    /// [Output only] Server-defined URL for the resource.
    #[prost(string, tag = "100")]
    pub self_link: ::prost::alloc::string::String,
    /// [Output only] The name of the Google Compute Engine
    /// \[zone\](<https://cloud.google.com/compute/docs/zones#available>) in which the
    /// cluster resides. This field is deprecated, use location instead.
    #[deprecated]
    #[prost(string, tag = "101")]
    pub zone: ::prost::alloc::string::String,
    /// [Output only] The IP address of this cluster's master endpoint.
    /// The endpoint can be accessed from the internet at
    /// `<https://username:password@endpoint/`.>
    ///
    /// See the `masterAuth` property of this resource for username and
    /// password information.
    #[prost(string, tag = "102")]
    pub endpoint: ::prost::alloc::string::String,
    /// The initial Kubernetes version for this cluster.  Valid versions are those
    /// found in validMasterVersions returned by getServerConfig.  The version can
    /// be upgraded over time; such upgrades are reflected in
    /// currentMasterVersion and currentNodeVersion.
    ///
    /// Users may specify either explicit versions offered by
    /// Kubernetes Engine or version aliases, which have the following behavior:
    ///
    /// - "latest": picks the highest valid Kubernetes version
    /// - "1.X": picks the highest valid patch+gke.N patch in the 1.X version
    /// - "1.X.Y": picks the highest valid gke.N patch in the 1.X.Y version
    /// - "1.X.Y-gke.N": picks an explicit Kubernetes version
    /// - "","-": picks the default Kubernetes version
    #[prost(string, tag = "103")]
    pub initial_cluster_version: ::prost::alloc::string::String,
    /// [Output only] The current software version of the master endpoint.
    #[prost(string, tag = "104")]
    pub current_master_version: ::prost::alloc::string::String,
    /// [Output only] Deprecated, use
    /// \[NodePool.version\](<https://cloud.google.com/kubernetes-engine/docs/reference/rest/v1beta1/projects.locations.clusters.nodePools>)
    /// instead. The current version of the node software components.
    /// If they are currently at multiple versions because they're in the process
    /// of being upgraded, this reflects the minimum version of all nodes.
    #[deprecated]
    #[prost(string, tag = "105")]
    pub current_node_version: ::prost::alloc::string::String,
    /// [Output only] The time the cluster was created, in
    /// \[RFC3339\](<https://www.ietf.org/rfc/rfc3339.txt>) text format.
    #[prost(string, tag = "106")]
    pub create_time: ::prost::alloc::string::String,
    /// [Output only] The current status of this cluster.
    #[prost(enumeration = "cluster::Status", tag = "107")]
    pub status: i32,
    /// [Output only] Deprecated. Use conditions instead.
    /// Additional information about the current status of this
    /// cluster, if available.
    #[deprecated]
    #[prost(string, tag = "108")]
    pub status_message: ::prost::alloc::string::String,
    /// [Output only] The size of the address space on each node for hosting
    /// containers. This is provisioned from within the `container_ipv4_cidr`
    /// range. This field will only be set when cluster is in route-based network
    /// mode.
    #[prost(int32, tag = "109")]
    pub node_ipv4_cidr_size: i32,
    /// [Output only] The IP address range of the Kubernetes services in
    /// this cluster, in
    /// \[CIDR\](<http://en.wikipedia.org/wiki/Classless_Inter-Domain_Routing>)
    /// notation (e.g. `1.2.3.4/29`). Service addresses are
    /// typically put in the last `/16` from the container CIDR.
    #[prost(string, tag = "110")]
    pub services_ipv4_cidr: ::prost::alloc::string::String,
    /// Deprecated. Use node_pools.instance_group_urls.
    #[deprecated]
    #[prost(string, repeated, tag = "111")]
    pub instance_group_urls: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// [Output only]  The number of nodes currently in the cluster. Deprecated.
    /// Call Kubernetes API directly to retrieve node information.
    #[deprecated]
    #[prost(int32, tag = "112")]
    pub current_node_count: i32,
    /// [Output only] The time the cluster will be automatically
    /// deleted in \[RFC3339\](<https://www.ietf.org/rfc/rfc3339.txt>) text format.
    #[prost(string, tag = "113")]
    pub expire_time: ::prost::alloc::string::String,
    /// [Output only] The name of the Google Compute Engine
    /// \[zone\](<https://cloud.google.com/compute/docs/regions-zones/regions-zones#available>)
    /// or
    /// \[region\](<https://cloud.google.com/compute/docs/regions-zones/regions-zones#available>)
    /// in which the cluster resides.
    #[prost(string, tag = "114")]
    pub location: ::prost::alloc::string::String,
    /// Enable the ability to use Cloud TPUs in this cluster.
    /// This field is deprecated, use tpu_config.enabled instead.
    #[prost(bool, tag = "115")]
    pub enable_tpu: bool,
    /// [Output only] The IP address range of the Cloud TPUs in this cluster, in
    /// \[CIDR\](<http://en.wikipedia.org/wiki/Classless_Inter-Domain_Routing>)
    /// notation (e.g. `1.2.3.4/29`).
    #[prost(string, tag = "116")]
    pub tpu_ipv4_cidr_block: ::prost::alloc::string::String,
    /// Configuration of etcd encryption.
    #[prost(message, optional, tag = "38")]
    pub database_encryption: ::core::option::Option<DatabaseEncryption>,
    /// Which conditions caused the current cluster state.
    #[prost(message, repeated, tag = "118")]
    pub conditions: ::prost::alloc::vec::Vec<StatusCondition>,
    /// Configuration for master components.
    #[prost(message, optional, tag = "124")]
    pub master: ::core::option::Option<Master>,
    /// Autopilot configuration for the cluster.
    #[prost(message, optional, tag = "128")]
    pub autopilot: ::core::option::Option<Autopilot>,
    /// Output only. Unique id for the cluster.
    #[prost(string, tag = "129")]
    pub id: ::prost::alloc::string::String,
    /// Default NodePool settings for the entire cluster. These settings are
    /// overridden if specified on the specific NodePool object.
    #[prost(message, optional, tag = "131")]
    pub node_pool_defaults: ::core::option::Option<NodePoolDefaults>,
    /// Logging configuration for the cluster.
    #[prost(message, optional, tag = "132")]
    pub logging_config: ::core::option::Option<LoggingConfig>,
    /// Monitoring configuration for the cluster.
    #[prost(message, optional, tag = "133")]
    pub monitoring_config: ::core::option::Option<MonitoringConfig>,
    /// Node pool configs that apply to all auto-provisioned node pools
    /// in autopilot clusters and node auto-provisioning enabled clusters.
    #[prost(message, optional, tag = "136")]
    pub node_pool_auto_config: ::core::option::Option<NodePoolAutoConfig>,
    /// Enable/Disable Protect API features for the cluster.
    #[prost(message, optional, tag = "137")]
    pub protect_config: ::core::option::Option<ProtectConfig>,
}
/// Nested message and enum types in `Cluster`.
pub mod cluster {
    /// The current status of the cluster.
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
    pub enum Status {
        /// Not set.
        Unspecified = 0,
        /// The PROVISIONING state indicates the cluster is being created.
        Provisioning = 1,
        /// The RUNNING state indicates the cluster has been created and is fully
        /// usable.
        Running = 2,
        /// The RECONCILING state indicates that some work is actively being done on
        /// the cluster, such as upgrading the master or node software. Details can
        /// be found in the `statusMessage` field.
        Reconciling = 3,
        /// The STOPPING state indicates the cluster is being deleted.
        Stopping = 4,
        /// The ERROR state indicates the cluster may be unusable. Details
        /// can be found in the `statusMessage` field.
        Error = 5,
        /// The DEGRADED state indicates the cluster requires user action to restore
        /// full functionality. Details can be found in the `statusMessage` field.
        Degraded = 6,
    }
    impl Status {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Status::Unspecified => "STATUS_UNSPECIFIED",
                Status::Provisioning => "PROVISIONING",
                Status::Running => "RUNNING",
                Status::Reconciling => "RECONCILING",
                Status::Stopping => "STOPPING",
                Status::Error => "ERROR",
                Status::Degraded => "DEGRADED",
            }
        }
    }
}
/// WorkloadConfig defines the flags to enable or disable the
/// workload configurations for the cluster.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WorkloadConfig {
    /// Sets which mode of auditing should be used for the cluster's workloads.
    #[prost(enumeration = "workload_config::Mode", optional, tag = "1")]
    pub audit_mode: ::core::option::Option<i32>,
}
/// Nested message and enum types in `WorkloadConfig`.
pub mod workload_config {
    /// Mode defines how to audit the workload configs.
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
    pub enum Mode {
        /// Default value meaning that no mode has been specified.
        Unspecified = 0,
        /// This disables Workload Configuration auditing on the cluster,
        /// meaning that nothing is surfaced.
        Disabled = 1,
        /// Applies the default set of policy auditing to a cluster's workloads.
        Basic = 4,
        /// Surfaces configurations that are not in line with the
        /// Pod Security Standard Baseline policy.
        Baseline = 2,
        /// Surfaces configurations that are not in line with the
        /// Pod Security Standard Restricted policy.
        Restricted = 3,
    }
    impl Mode {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Mode::Unspecified => "MODE_UNSPECIFIED",
                Mode::Disabled => "DISABLED",
                Mode::Basic => "BASIC",
                Mode::Baseline => "BASELINE",
                Mode::Restricted => "RESTRICTED",
            }
        }
    }
}
/// ProtectConfig defines the flags needed to enable/disable features for the
/// Protect API.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtectConfig {
    /// WorkloadConfig defines which actions are enabled for a cluster's workload
    /// configurations.
    #[prost(message, optional, tag = "1")]
    pub workload_config: ::core::option::Option<WorkloadConfig>,
    /// Sets which mode to use for Protect workload vulnerability scanning feature.
    #[prost(
        enumeration = "protect_config::WorkloadVulnerabilityMode",
        optional,
        tag = "2"
    )]
    pub workload_vulnerability_mode: ::core::option::Option<i32>,
}
/// Nested message and enum types in `ProtectConfig`.
pub mod protect_config {
    /// WorkloadVulnerabilityMode defines mode to perform vulnerability scanning.
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
    pub enum WorkloadVulnerabilityMode {
        /// Default value not specified.
        Unspecified = 0,
        /// Disables Workload Vulnerability Scanning feature on the cluster.
        Disabled = 1,
        /// Applies basic vulnerability scanning settings for cluster workloads.
        Basic = 2,
    }
    impl WorkloadVulnerabilityMode {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                WorkloadVulnerabilityMode::Unspecified => {
                    "WORKLOAD_VULNERABILITY_MODE_UNSPECIFIED"
                }
                WorkloadVulnerabilityMode::Disabled => "DISABLED",
                WorkloadVulnerabilityMode::Basic => "BASIC",
            }
        }
    }
}
/// Subset of Nodepool message that has defaults.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NodePoolDefaults {
    /// Subset of NodeConfig message that has defaults.
    #[prost(message, optional, tag = "1")]
    pub node_config_defaults: ::core::option::Option<NodeConfigDefaults>,
}
/// Subset of NodeConfig message that has defaults.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NodeConfigDefaults {
    /// GCFS (Google Container File System, also known as Riptide) options.
    #[prost(message, optional, tag = "1")]
    pub gcfs_config: ::core::option::Option<GcfsConfig>,
    /// Logging configuration for node pools.
    #[prost(message, optional, tag = "3")]
    pub logging_config: ::core::option::Option<NodePoolLoggingConfig>,
}
/// node pool configs that apply to all auto-provisioned node pools
/// in autopilot clusters and node auto-provisioning enabled clusters
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NodePoolAutoConfig {
    /// The list of instance tags applied to all nodes. Tags are used to identify
    /// valid sources or targets for network firewalls and are specified by
    /// the client during cluster creation. Each tag within the list
    /// must comply with RFC1035.
    #[prost(message, optional, tag = "1")]
    pub network_tags: ::core::option::Option<NetworkTags>,
}
/// ClusterUpdate describes an update to the cluster. Exactly one update can
/// be applied to a cluster with each request, so at most one field can be
/// provided.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClusterUpdate {
    /// The Kubernetes version to change the nodes to (typically an
    /// upgrade).
    ///
    /// Users may specify either explicit versions offered by
    /// Kubernetes Engine or version aliases, which have the following behavior:
    ///
    /// - "latest": picks the highest valid Kubernetes version
    /// - "1.X": picks the highest valid patch+gke.N patch in the 1.X version
    /// - "1.X.Y": picks the highest valid gke.N patch in the 1.X.Y version
    /// - "1.X.Y-gke.N": picks an explicit Kubernetes version
    /// - "-": picks the Kubernetes master version
    #[prost(string, tag = "4")]
    pub desired_node_version: ::prost::alloc::string::String,
    /// The monitoring service the cluster should use to write metrics.
    /// Currently available options:
    ///
    /// * "monitoring.googleapis.com/kubernetes" - The Cloud Monitoring
    /// service with a Kubernetes-native resource model
    /// * `monitoring.googleapis.com` - The legacy Cloud Monitoring service (no
    ///    longer available as of GKE 1.15).
    /// * `none` - No metrics will be exported from the cluster.
    ///
    /// If left as an empty string,`monitoring.googleapis.com/kubernetes` will be
    /// used for GKE 1.14+ or `monitoring.googleapis.com` for earlier versions.
    #[prost(string, tag = "5")]
    pub desired_monitoring_service: ::prost::alloc::string::String,
    /// Configurations for the various addons available to run in the cluster.
    #[prost(message, optional, tag = "6")]
    pub desired_addons_config: ::core::option::Option<AddonsConfig>,
    /// The node pool to be upgraded. This field is mandatory if
    /// "desired_node_version", "desired_image_family",
    /// "desired_node_pool_autoscaling", or "desired_workload_metadata_config"
    /// is specified and there is more than one node pool on the cluster.
    #[prost(string, tag = "7")]
    pub desired_node_pool_id: ::prost::alloc::string::String,
    /// The desired image type for the node pool.
    /// NOTE: Set the "desired_node_pool" field as well.
    #[prost(string, tag = "8")]
    pub desired_image_type: ::prost::alloc::string::String,
    /// Autoscaler configuration for the node pool specified in
    /// desired_node_pool_id. If there is only one pool in the
    /// cluster and desired_node_pool_id is not provided then
    /// the change applies to that single node pool.
    #[prost(message, optional, tag = "9")]
    pub desired_node_pool_autoscaling: ::core::option::Option<NodePoolAutoscaling>,
    /// The desired list of Google Compute Engine
    /// \[zones\](<https://cloud.google.com/compute/docs/zones#available>) in which the
    /// cluster's nodes should be located.
    ///
    /// This list must always include the cluster's primary zone.
    ///
    /// Warning: changing cluster locations will update the locations of all node
    /// pools and will result in nodes being added and/or removed.
    #[prost(string, repeated, tag = "10")]
    pub desired_locations: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The desired configuration options for master authorized networks feature.
    #[prost(message, optional, tag = "12")]
    pub desired_master_authorized_networks_config: ::core::option::Option<
        MasterAuthorizedNetworksConfig,
    >,
    /// The desired configuration options for the PodSecurityPolicy feature.
    #[prost(message, optional, tag = "14")]
    pub desired_pod_security_policy_config: ::core::option::Option<
        PodSecurityPolicyConfig,
    >,
    /// Cluster-level autoscaling configuration.
    #[prost(message, optional, tag = "15")]
    pub desired_cluster_autoscaling: ::core::option::Option<ClusterAutoscaling>,
    /// The desired configuration options for the Binary Authorization feature.
    #[prost(message, optional, tag = "16")]
    pub desired_binary_authorization: ::core::option::Option<BinaryAuthorization>,
    /// The logging service the cluster should use to write logs.
    /// Currently available options:
    ///
    /// * `logging.googleapis.com/kubernetes` - The Cloud Logging
    /// service with a Kubernetes-native resource model
    /// * `logging.googleapis.com` - The legacy Cloud Logging service (no longer
    ///    available as of GKE 1.15).
    /// * `none` - no logs will be exported from the cluster.
    ///
    /// If left as an empty string,`logging.googleapis.com/kubernetes` will be
    /// used for GKE 1.14+ or `logging.googleapis.com` for earlier versions.
    #[prost(string, tag = "19")]
    pub desired_logging_service: ::prost::alloc::string::String,
    /// The desired configuration for exporting resource usage.
    #[prost(message, optional, tag = "21")]
    pub desired_resource_usage_export_config: ::core::option::Option<
        ResourceUsageExportConfig,
    >,
    /// Cluster-level Vertical Pod Autoscaling configuration.
    #[prost(message, optional, tag = "22")]
    pub desired_vertical_pod_autoscaling: ::core::option::Option<VerticalPodAutoscaling>,
    /// The desired private cluster configuration.
    #[prost(message, optional, tag = "25")]
    pub desired_private_cluster_config: ::core::option::Option<PrivateClusterConfig>,
    /// The desired config of Intra-node visibility.
    #[prost(message, optional, tag = "26")]
    pub desired_intra_node_visibility_config: ::core::option::Option<
        IntraNodeVisibilityConfig,
    >,
    /// The desired status of whether to disable default sNAT for this cluster.
    #[prost(message, optional, tag = "28")]
    pub desired_default_snat_status: ::core::option::Option<DefaultSnatStatus>,
    /// The desired telemetry integration for the cluster.
    #[prost(message, optional, tag = "30")]
    pub desired_cluster_telemetry: ::core::option::Option<ClusterTelemetry>,
    /// The desired release channel configuration.
    #[prost(message, optional, tag = "31")]
    pub desired_release_channel: ::core::option::Option<ReleaseChannel>,
    /// The desired Cloud TPU configuration.
    #[prost(message, optional, tag = "38")]
    pub desired_tpu_config: ::core::option::Option<TpuConfig>,
    /// The desired L4 Internal Load Balancer Subsetting configuration.
    #[prost(message, optional, tag = "39")]
    pub desired_l4ilb_subsetting_config: ::core::option::Option<IlbSubsettingConfig>,
    /// The desired datapath provider for the cluster.
    #[prost(enumeration = "DatapathProvider", tag = "50")]
    pub desired_datapath_provider: i32,
    /// The desired state of IPv6 connectivity to Google Services.
    #[prost(enumeration = "PrivateIPv6GoogleAccess", tag = "51")]
    pub desired_private_ipv6_google_access: i32,
    /// The desired notification configuration.
    #[prost(message, optional, tag = "55")]
    pub desired_notification_config: ::core::option::Option<NotificationConfig>,
    /// The Kubernetes version to change the master to. The only valid value is the
    /// latest supported version.
    ///
    /// Users may specify either explicit versions offered by
    /// Kubernetes Engine or version aliases, which have the following behavior:
    ///
    /// - "latest": picks the highest valid Kubernetes version
    /// - "1.X": picks the highest valid patch+gke.N patch in the 1.X version
    /// - "1.X.Y": picks the highest valid gke.N patch in the 1.X.Y version
    /// - "1.X.Y-gke.N": picks an explicit Kubernetes version
    /// - "-": picks the default Kubernetes version
    #[prost(string, tag = "100")]
    pub desired_master_version: ::prost::alloc::string::String,
    /// The desired GCFS config for the cluster.
    #[prost(message, optional, tag = "109")]
    pub desired_gcfs_config: ::core::option::Option<GcfsConfig>,
    /// Configuration of etcd encryption.
    #[prost(message, optional, tag = "46")]
    pub desired_database_encryption: ::core::option::Option<DatabaseEncryption>,
    /// Configuration for Workload Identity.
    #[prost(message, optional, tag = "47")]
    pub desired_workload_identity_config: ::core::option::Option<WorkloadIdentityConfig>,
    /// Configuration for issuance of mTLS keys and certificates to Kubernetes
    /// pods.
    #[prost(message, optional, tag = "61")]
    pub desired_workload_certificates: ::core::option::Option<WorkloadCertificates>,
    /// Configuration for issuance of mTLS keys and certificates to Kubernetes
    /// pods.
    #[prost(message, optional, tag = "67")]
    pub desired_mesh_certificates: ::core::option::Option<MeshCertificates>,
    /// Configuration for direct-path (via ALTS) with workload identity.
    #[prost(message, optional, tag = "62")]
    pub desired_workload_alts_config: ::core::option::Option<WorkloadAltsConfig>,
    /// Configuration for Shielded Nodes.
    #[prost(message, optional, tag = "48")]
    pub desired_shielded_nodes: ::core::option::Option<ShieldedNodes>,
    /// The desired configuration for the fine-grained cost management feature.
    #[prost(message, optional, tag = "49")]
    pub desired_cost_management_config: ::core::option::Option<CostManagementConfig>,
    /// Configuration for master components.
    #[prost(message, optional, tag = "52")]
    pub desired_master: ::core::option::Option<Master>,
    /// DNSConfig contains clusterDNS config for this cluster.
    #[prost(message, optional, tag = "53")]
    pub desired_dns_config: ::core::option::Option<DnsConfig>,
    /// ServiceExternalIPsConfig specifies the config for the use of Services with
    /// ExternalIPs field.
    #[prost(message, optional, tag = "60")]
    pub desired_service_external_ips_config: ::core::option::Option<
        ServiceExternalIPsConfig,
    >,
    /// AuthenticatorGroupsConfig specifies the config for the cluster security
    /// groups settings.
    #[prost(message, optional, tag = "63")]
    pub desired_authenticator_groups_config: ::core::option::Option<
        AuthenticatorGroupsConfig,
    >,
    /// The desired logging configuration.
    #[prost(message, optional, tag = "64")]
    pub desired_logging_config: ::core::option::Option<LoggingConfig>,
    /// The desired monitoring configuration.
    #[prost(message, optional, tag = "65")]
    pub desired_monitoring_config: ::core::option::Option<MonitoringConfig>,
    /// The desired Identity Service component configuration.
    #[prost(message, optional, tag = "66")]
    pub desired_identity_service_config: ::core::option::Option<IdentityServiceConfig>,
    /// Enable/Disable private endpoint for the cluster's master.
    #[prost(bool, optional, tag = "71")]
    pub desired_enable_private_endpoint: ::core::option::Option<bool>,
    /// The desired network tags that apply to all auto-provisioned node pools
    /// in autopilot clusters and node auto-provisioning enabled clusters.
    #[prost(message, optional, tag = "110")]
    pub desired_node_pool_auto_config_network_tags: ::core::option::Option<NetworkTags>,
    /// Enable/Disable Protect API features for the cluster.
    #[prost(message, optional, tag = "112")]
    pub desired_protect_config: ::core::option::Option<ProtectConfig>,
    /// The desired config of Gateway API on this cluster.
    #[prost(message, optional, tag = "114")]
    pub desired_gateway_api_config: ::core::option::Option<GatewayApiConfig>,
    /// The desired node pool logging configuration defaults for the cluster.
    #[prost(message, optional, tag = "116")]
    pub desired_node_pool_logging_config: ::core::option::Option<NodePoolLoggingConfig>,
}
/// This operation resource represents operations that may have happened or are
/// happening on the cluster. All fields are output only.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Operation {
    /// The server-assigned ID for the operation.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The name of the Google Compute Engine
    /// \[zone\](<https://cloud.google.com/compute/docs/zones#available>) in which the
    /// operation is taking place. This field is deprecated, use location instead.
    #[deprecated]
    #[prost(string, tag = "2")]
    pub zone: ::prost::alloc::string::String,
    /// The operation type.
    #[prost(enumeration = "operation::Type", tag = "3")]
    pub operation_type: i32,
    /// The current status of the operation.
    #[prost(enumeration = "operation::Status", tag = "4")]
    pub status: i32,
    /// Detailed operation progress, if available.
    #[prost(string, tag = "8")]
    pub detail: ::prost::alloc::string::String,
    /// Output only. If an error has occurred, a textual description of the error.
    /// Deprecated. Use field error instead.
    #[deprecated]
    #[prost(string, tag = "5")]
    pub status_message: ::prost::alloc::string::String,
    /// Server-defined URL for the resource.
    #[prost(string, tag = "6")]
    pub self_link: ::prost::alloc::string::String,
    /// Server-defined URL for the target of the operation.
    #[prost(string, tag = "7")]
    pub target_link: ::prost::alloc::string::String,
    /// [Output only] The name of the Google Compute Engine
    /// \[zone\](<https://cloud.google.com/compute/docs/regions-zones/regions-zones#available>)
    /// or
    /// \[region\](<https://cloud.google.com/compute/docs/regions-zones/regions-zones#available>)
    /// in which the cluster resides.
    #[prost(string, tag = "9")]
    pub location: ::prost::alloc::string::String,
    /// [Output only] The time the operation started, in
    /// \[RFC3339\](<https://www.ietf.org/rfc/rfc3339.txt>) text format.
    #[prost(string, tag = "10")]
    pub start_time: ::prost::alloc::string::String,
    /// [Output only] The time the operation completed, in
    /// \[RFC3339\](<https://www.ietf.org/rfc/rfc3339.txt>) text format.
    #[prost(string, tag = "11")]
    pub end_time: ::prost::alloc::string::String,
    /// Output only. [Output only] Progress information for an operation.
    #[prost(message, optional, tag = "12")]
    pub progress: ::core::option::Option<OperationProgress>,
    /// Which conditions caused the current cluster state.
    /// Deprecated. Use field error instead.
    #[deprecated]
    #[prost(message, repeated, tag = "13")]
    pub cluster_conditions: ::prost::alloc::vec::Vec<StatusCondition>,
    /// Which conditions caused the current node pool state.
    /// Deprecated. Use field error instead.
    #[deprecated]
    #[prost(message, repeated, tag = "14")]
    pub nodepool_conditions: ::prost::alloc::vec::Vec<StatusCondition>,
    /// The error result of the operation in case of failure.
    #[prost(message, optional, tag = "15")]
    pub error: ::core::option::Option<super::super::rpc::Status>,
}
/// Nested message and enum types in `Operation`.
pub mod operation {
    /// Current status of the operation.
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
    pub enum Status {
        /// Not set.
        Unspecified = 0,
        /// The operation has been created.
        Pending = 1,
        /// The operation is currently running.
        Running = 2,
        /// The operation is done, either cancelled or completed.
        Done = 3,
        /// The operation is aborting.
        Aborting = 4,
    }
    impl Status {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Status::Unspecified => "STATUS_UNSPECIFIED",
                Status::Pending => "PENDING",
                Status::Running => "RUNNING",
                Status::Done => "DONE",
                Status::Aborting => "ABORTING",
            }
        }
    }
    /// Operation type.
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
        /// Not set.
        Unspecified = 0,
        /// Cluster create.
        CreateCluster = 1,
        /// Cluster delete.
        DeleteCluster = 2,
        /// A master upgrade.
        UpgradeMaster = 3,
        /// A node upgrade.
        UpgradeNodes = 4,
        /// Cluster repair.
        RepairCluster = 5,
        /// Cluster update.
        UpdateCluster = 6,
        /// Node pool create.
        CreateNodePool = 7,
        /// Node pool delete.
        DeleteNodePool = 8,
        /// Set node pool management.
        SetNodePoolManagement = 9,
        /// Automatic node pool repair.
        AutoRepairNodes = 10,
        /// Automatic node upgrade.
        AutoUpgradeNodes = 11,
        /// Set labels.
        SetLabels = 12,
        /// Set/generate master auth materials
        SetMasterAuth = 13,
        /// Set node pool size.
        SetNodePoolSize = 14,
        /// Updates network policy for a cluster.
        SetNetworkPolicy = 15,
        /// Set the maintenance policy.
        SetMaintenancePolicy = 16,
    }
    impl Type {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Type::Unspecified => "TYPE_UNSPECIFIED",
                Type::CreateCluster => "CREATE_CLUSTER",
                Type::DeleteCluster => "DELETE_CLUSTER",
                Type::UpgradeMaster => "UPGRADE_MASTER",
                Type::UpgradeNodes => "UPGRADE_NODES",
                Type::RepairCluster => "REPAIR_CLUSTER",
                Type::UpdateCluster => "UPDATE_CLUSTER",
                Type::CreateNodePool => "CREATE_NODE_POOL",
                Type::DeleteNodePool => "DELETE_NODE_POOL",
                Type::SetNodePoolManagement => "SET_NODE_POOL_MANAGEMENT",
                Type::AutoRepairNodes => "AUTO_REPAIR_NODES",
                Type::AutoUpgradeNodes => "AUTO_UPGRADE_NODES",
                Type::SetLabels => "SET_LABELS",
                Type::SetMasterAuth => "SET_MASTER_AUTH",
                Type::SetNodePoolSize => "SET_NODE_POOL_SIZE",
                Type::SetNetworkPolicy => "SET_NETWORK_POLICY",
                Type::SetMaintenancePolicy => "SET_MAINTENANCE_POLICY",
            }
        }
    }
}
/// Information about operation (or operation stage) progress.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationProgress {
    /// A non-parameterized string describing an operation stage.
    /// Unset for single-stage operations.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Status of an operation stage.
    /// Unset for single-stage operations.
    #[prost(enumeration = "operation::Status", tag = "2")]
    pub status: i32,
    /// Progress metric bundle, for example:
    ///    metrics: [{name: "nodes done",     int_value: 15},
    ///              {name: "nodes total",    int_value: 32}]
    /// or
    ///    metrics: [{name: "progress",       double_value: 0.56},
    ///              {name: "progress scale", double_value: 1.0}]
    #[prost(message, repeated, tag = "3")]
    pub metrics: ::prost::alloc::vec::Vec<operation_progress::Metric>,
    /// Substages of an operation or a stage.
    #[prost(message, repeated, tag = "4")]
    pub stages: ::prost::alloc::vec::Vec<OperationProgress>,
}
/// Nested message and enum types in `OperationProgress`.
pub mod operation_progress {
    /// Progress metric is (string, int|float|string) pair.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Metric {
        /// Required. Metric name, e.g., "nodes total", "percent done".
        #[prost(string, tag = "1")]
        pub name: ::prost::alloc::string::String,
        /// Strictly one of the values is required.
        #[prost(oneof = "metric::Value", tags = "2, 3, 4")]
        pub value: ::core::option::Option<metric::Value>,
    }
    /// Nested message and enum types in `Metric`.
    pub mod metric {
        /// Strictly one of the values is required.
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Value {
            /// For metrics with integer value.
            #[prost(int64, tag = "2")]
            IntValue(i64),
            /// For metrics with floating point value.
            #[prost(double, tag = "3")]
            DoubleValue(f64),
            /// For metrics with custom values (ratios, visual progress, etc.).
            #[prost(string, tag = "4")]
            StringValue(::prost::alloc::string::String),
        }
    }
}
/// CreateClusterRequest creates a cluster.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateClusterRequest {
    /// Required. Deprecated. The Google Developers Console [project ID or project
    /// number](<https://cloud.google.com/resource-manager/docs/creating-managing-projects>).
    /// This field has been deprecated and replaced by the parent field.
    #[deprecated]
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    /// Required. Deprecated. The name of the Google Compute Engine
    /// \[zone\](<https://cloud.google.com/compute/docs/zones#available>) in which the
    /// cluster resides. This field has been deprecated and replaced by the parent
    /// field.
    #[deprecated]
    #[prost(string, tag = "2")]
    pub zone: ::prost::alloc::string::String,
    /// Required. A [cluster
    /// resource](<https://cloud.google.com/container-engine/reference/rest/v1beta1/projects.locations.clusters>)
    #[prost(message, optional, tag = "3")]
    pub cluster: ::core::option::Option<Cluster>,
    /// The parent (project and location) where the cluster will be created.
    /// Specified in the format `projects/*/locations/*`.
    #[prost(string, tag = "5")]
    pub parent: ::prost::alloc::string::String,
}
/// GetClusterRequest gets the settings of a cluster.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetClusterRequest {
    /// Required. Deprecated. The Google Developers Console [project ID or project
    /// number](<https://cloud.google.com/resource-manager/docs/creating-managing-projects>).
    /// This field has been deprecated and replaced by the name field.
    #[deprecated]
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    /// Required. Deprecated. The name of the Google Compute Engine
    /// \[zone\](<https://cloud.google.com/compute/docs/zones#available>) in which the
    /// cluster resides. This field has been deprecated and replaced by the name
    /// field.
    #[deprecated]
    #[prost(string, tag = "2")]
    pub zone: ::prost::alloc::string::String,
    /// Required. Deprecated. The name of the cluster to retrieve.
    /// This field has been deprecated and replaced by the name field.
    #[deprecated]
    #[prost(string, tag = "3")]
    pub cluster_id: ::prost::alloc::string::String,
    /// The name (project, location, cluster) of the cluster to retrieve.
    /// Specified in the format `projects/*/locations/*/clusters/*`.
    #[prost(string, tag = "5")]
    pub name: ::prost::alloc::string::String,
}
/// UpdateClusterRequest updates the settings of a cluster.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateClusterRequest {
    /// Required. Deprecated. The Google Developers Console [project ID or project
    /// number](<https://cloud.google.com/resource-manager/docs/creating-managing-projects>).
    /// This field has been deprecated and replaced by the name field.
    #[deprecated]
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    /// Required. Deprecated. The name of the Google Compute Engine
    /// \[zone\](<https://cloud.google.com/compute/docs/zones#available>) in which the
    /// cluster resides. This field has been deprecated and replaced by the name
    /// field.
    #[deprecated]
    #[prost(string, tag = "2")]
    pub zone: ::prost::alloc::string::String,
    /// Required. Deprecated. The name of the cluster to upgrade.
    /// This field has been deprecated and replaced by the name field.
    #[deprecated]
    #[prost(string, tag = "3")]
    pub cluster_id: ::prost::alloc::string::String,
    /// Required. A description of the update.
    #[prost(message, optional, tag = "4")]
    pub update: ::core::option::Option<ClusterUpdate>,
    /// The name (project, location, cluster) of the cluster to update.
    /// Specified in the format `projects/*/locations/*/clusters/*`.
    #[prost(string, tag = "5")]
    pub name: ::prost::alloc::string::String,
}
/// SetNodePoolVersionRequest updates the version of a node pool.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateNodePoolRequest {
    /// Required. Deprecated. The Google Developers Console [project ID or project
    /// number](<https://cloud.google.com/resource-manager/docs/creating-managing-projects>).
    /// This field has been deprecated and replaced by the name field.
    #[deprecated]
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    /// Required. Deprecated. The name of the Google Compute Engine
    /// \[zone\](<https://cloud.google.com/compute/docs/zones#available>) in which the
    /// cluster resides. This field has been deprecated and replaced by the name
    /// field.
    #[deprecated]
    #[prost(string, tag = "2")]
    pub zone: ::prost::alloc::string::String,
    /// Required. Deprecated. The name of the cluster to upgrade.
    /// This field has been deprecated and replaced by the name field.
    #[deprecated]
    #[prost(string, tag = "3")]
    pub cluster_id: ::prost::alloc::string::String,
    /// Required. Deprecated. The name of the node pool to upgrade.
    /// This field has been deprecated and replaced by the name field.
    #[deprecated]
    #[prost(string, tag = "4")]
    pub node_pool_id: ::prost::alloc::string::String,
    /// Required. The Kubernetes version to change the nodes to (typically an
    /// upgrade).
    ///
    /// Users may specify either explicit versions offered by Kubernetes Engine or
    /// version aliases, which have the following behavior:
    ///
    /// - "latest": picks the highest valid Kubernetes version
    /// - "1.X": picks the highest valid patch+gke.N patch in the 1.X version
    /// - "1.X.Y": picks the highest valid gke.N patch in the 1.X.Y version
    /// - "1.X.Y-gke.N": picks an explicit Kubernetes version
    /// - "-": picks the Kubernetes master version
    #[prost(string, tag = "5")]
    pub node_version: ::prost::alloc::string::String,
    /// Required. The desired image type for the node pool.
    #[prost(string, tag = "6")]
    pub image_type: ::prost::alloc::string::String,
    /// The desired list of Google Compute Engine
    /// \[zones\](<https://cloud.google.com/compute/docs/zones#available>) in which the
    /// node pool's nodes should be located. Changing the locations for a node pool
    /// will result in nodes being either created or removed from the node pool,
    /// depending on whether locations are being added or removed.
    #[prost(string, repeated, tag = "13")]
    pub locations: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The desired workload metadata config for the node pool.
    #[prost(message, optional, tag = "14")]
    pub workload_metadata_config: ::core::option::Option<WorkloadMetadataConfig>,
    /// The name (project, location, cluster, node pool) of the node pool to
    /// update. Specified in the format
    /// `projects/*/locations/*/clusters/*/nodePools/*`.
    #[prost(string, tag = "8")]
    pub name: ::prost::alloc::string::String,
    /// Upgrade settings control disruption and speed of the upgrade.
    #[prost(message, optional, tag = "15")]
    pub upgrade_settings: ::core::option::Option<node_pool::UpgradeSettings>,
    /// The desired network tags to be applied to all nodes in the node pool.
    /// If this field is not present, the tags will not be changed. Otherwise,
    /// the existing network tags will be *replaced* with the provided tags.
    #[prost(message, optional, tag = "16")]
    pub tags: ::core::option::Option<NetworkTags>,
    /// The desired node taints to be applied to all nodes in the node pool.
    /// If this field is not present, the taints will not be changed. Otherwise,
    /// the existing node taints will be *replaced* with the provided taints.
    #[prost(message, optional, tag = "17")]
    pub taints: ::core::option::Option<NodeTaints>,
    /// The desired node labels to be applied to all nodes in the node pool.
    /// If this field is not present, the labels will not be changed. Otherwise,
    /// the existing node labels will be *replaced* with the provided labels.
    #[prost(message, optional, tag = "18")]
    pub labels: ::core::option::Option<NodeLabels>,
    /// Parameters that can be configured on Linux nodes.
    #[prost(message, optional, tag = "19")]
    pub linux_node_config: ::core::option::Option<LinuxNodeConfig>,
    /// Node kubelet configs.
    #[prost(message, optional, tag = "20")]
    pub kubelet_config: ::core::option::Option<NodeKubeletConfig>,
    /// Node network config.
    #[prost(message, optional, tag = "21")]
    pub node_network_config: ::core::option::Option<NodeNetworkConfig>,
    /// GCFS config.
    #[prost(message, optional, tag = "22")]
    pub gcfs_config: ::core::option::Option<GcfsConfig>,
    /// Confidential nodes config.
    /// All the nodes in the node pool will be Confidential VM once enabled.
    #[prost(message, optional, tag = "23")]
    pub confidential_nodes: ::core::option::Option<ConfidentialNodes>,
    /// Enable or disable gvnic on the node pool.
    #[prost(message, optional, tag = "29")]
    pub gvnic: ::core::option::Option<VirtualNic>,
    /// Enable or disable NCCL fast socket for the node pool.
    #[prost(message, optional, tag = "31")]
    pub fast_socket: ::core::option::Option<FastSocket>,
    /// Logging configuration.
    #[prost(message, optional, tag = "32")]
    pub logging_config: ::core::option::Option<NodePoolLoggingConfig>,
    /// The resource labels for the node pool to use to annotate any related
    /// Google Compute Engine resources.
    #[prost(message, optional, tag = "33")]
    pub resource_labels: ::core::option::Option<ResourceLabels>,
}
/// SetNodePoolAutoscalingRequest sets the autoscaler settings of a node pool.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetNodePoolAutoscalingRequest {
    /// Required. Deprecated. The Google Developers Console [project ID or project
    /// number](<https://cloud.google.com/resource-manager/docs/creating-managing-projects>).
    /// This field has been deprecated and replaced by the name field.
    #[deprecated]
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    /// Required. Deprecated. The name of the Google Compute Engine
    /// \[zone\](<https://cloud.google.com/compute/docs/zones#available>) in which the
    /// cluster resides. This field has been deprecated and replaced by the name
    /// field.
    #[deprecated]
    #[prost(string, tag = "2")]
    pub zone: ::prost::alloc::string::String,
    /// Required. Deprecated. The name of the cluster to upgrade.
    /// This field has been deprecated and replaced by the name field.
    #[deprecated]
    #[prost(string, tag = "3")]
    pub cluster_id: ::prost::alloc::string::String,
    /// Required. Deprecated. The name of the node pool to upgrade.
    /// This field has been deprecated and replaced by the name field.
    #[deprecated]
    #[prost(string, tag = "4")]
    pub node_pool_id: ::prost::alloc::string::String,
    /// Required. Autoscaling configuration for the node pool.
    #[prost(message, optional, tag = "5")]
    pub autoscaling: ::core::option::Option<NodePoolAutoscaling>,
    /// The name (project, location, cluster, node pool) of the node pool to set
    /// autoscaler settings. Specified in the format
    /// `projects/*/locations/*/clusters/*/nodePools/*`.
    #[prost(string, tag = "6")]
    pub name: ::prost::alloc::string::String,
}
/// SetLoggingServiceRequest sets the logging service of a cluster.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetLoggingServiceRequest {
    /// Required. Deprecated. The Google Developers Console [project ID or project
    /// number](<https://cloud.google.com/resource-manager/docs/creating-managing-projects>).
    /// This field has been deprecated and replaced by the name field.
    #[deprecated]
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    /// Required. Deprecated. The name of the Google Compute Engine
    /// \[zone\](<https://cloud.google.com/compute/docs/zones#available>) in which the
    /// cluster resides. This field has been deprecated and replaced by the name
    /// field.
    #[deprecated]
    #[prost(string, tag = "2")]
    pub zone: ::prost::alloc::string::String,
    /// Required. Deprecated. The name of the cluster to upgrade.
    /// This field has been deprecated and replaced by the name field.
    #[deprecated]
    #[prost(string, tag = "3")]
    pub cluster_id: ::prost::alloc::string::String,
    /// Required. The logging service the cluster should use to write logs.
    /// Currently available options:
    ///
    /// * `logging.googleapis.com/kubernetes` - The Cloud Logging
    /// service with a Kubernetes-native resource model
    /// * `logging.googleapis.com` - The legacy Cloud Logging service (no longer
    ///    available as of GKE 1.15).
    /// * `none` - no logs will be exported from the cluster.
    ///
    /// If left as an empty string,`logging.googleapis.com/kubernetes` will be
    /// used for GKE 1.14+ or `logging.googleapis.com` for earlier versions.
    #[prost(string, tag = "4")]
    pub logging_service: ::prost::alloc::string::String,
    /// The name (project, location, cluster) of the cluster to set logging.
    /// Specified in the format `projects/*/locations/*/clusters/*`.
    #[prost(string, tag = "5")]
    pub name: ::prost::alloc::string::String,
}
/// SetMonitoringServiceRequest sets the monitoring service of a cluster.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetMonitoringServiceRequest {
    /// Required. Deprecated. The Google Developers Console [project ID or project
    /// number](<https://cloud.google.com/resource-manager/docs/creating-managing-projects>).
    /// This field has been deprecated and replaced by the name field.
    #[deprecated]
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    /// Required. Deprecated. The name of the Google Compute Engine
    /// \[zone\](<https://cloud.google.com/compute/docs/zones#available>) in which the
    /// cluster resides. This field has been deprecated and replaced by the name
    /// field.
    #[deprecated]
    #[prost(string, tag = "2")]
    pub zone: ::prost::alloc::string::String,
    /// Required. Deprecated. The name of the cluster to upgrade.
    /// This field has been deprecated and replaced by the name field.
    #[deprecated]
    #[prost(string, tag = "3")]
    pub cluster_id: ::prost::alloc::string::String,
    /// Required. The monitoring service the cluster should use to write metrics.
    /// Currently available options:
    ///
    /// * "monitoring.googleapis.com/kubernetes" - The Cloud Monitoring
    /// service with a Kubernetes-native resource model
    /// * `monitoring.googleapis.com` - The legacy Cloud Monitoring service (no
    ///    longer available as of GKE 1.15).
    /// * `none` - No metrics will be exported from the cluster.
    ///
    /// If left as an empty string,`monitoring.googleapis.com/kubernetes` will be
    /// used for GKE 1.14+ or `monitoring.googleapis.com` for earlier versions.
    #[prost(string, tag = "4")]
    pub monitoring_service: ::prost::alloc::string::String,
    /// The name (project, location, cluster) of the cluster to set monitoring.
    /// Specified in the format `projects/*/locations/*/clusters/*`.
    #[prost(string, tag = "6")]
    pub name: ::prost::alloc::string::String,
}
/// SetAddonsRequest sets the addons associated with the cluster.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetAddonsConfigRequest {
    /// Required. Deprecated. The Google Developers Console [project ID or project
    /// number](<https://cloud.google.com/resource-manager/docs/creating-managing-projects>).
    /// This field has been deprecated and replaced by the name field.
    #[deprecated]
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    /// Required. Deprecated. The name of the Google Compute Engine
    /// \[zone\](<https://cloud.google.com/compute/docs/zones#available>) in which the
    /// cluster resides. This field has been deprecated and replaced by the name
    /// field.
    #[deprecated]
    #[prost(string, tag = "2")]
    pub zone: ::prost::alloc::string::String,
    /// Required. Deprecated. The name of the cluster to upgrade.
    /// This field has been deprecated and replaced by the name field.
    #[deprecated]
    #[prost(string, tag = "3")]
    pub cluster_id: ::prost::alloc::string::String,
    /// Required. The desired configurations for the various addons available to run in the
    /// cluster.
    #[prost(message, optional, tag = "4")]
    pub addons_config: ::core::option::Option<AddonsConfig>,
    /// The name (project, location, cluster) of the cluster to set addons.
    /// Specified in the format `projects/*/locations/*/clusters/*`.
    #[prost(string, tag = "6")]
    pub name: ::prost::alloc::string::String,
}
/// SetLocationsRequest sets the locations of the cluster.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetLocationsRequest {
    /// Required. Deprecated. The Google Developers Console [project ID or project
    /// number](<https://cloud.google.com/resource-manager/docs/creating-managing-projects>).
    /// This field has been deprecated and replaced by the name field.
    #[deprecated]
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    /// Required. Deprecated. The name of the Google Compute Engine
    /// \[zone\](<https://cloud.google.com/compute/docs/zones#available>) in which the
    /// cluster resides. This field has been deprecated and replaced by the name
    /// field.
    #[deprecated]
    #[prost(string, tag = "2")]
    pub zone: ::prost::alloc::string::String,
    /// Required. Deprecated. The name of the cluster to upgrade.
    /// This field has been deprecated and replaced by the name field.
    #[deprecated]
    #[prost(string, tag = "3")]
    pub cluster_id: ::prost::alloc::string::String,
    /// Required. The desired list of Google Compute Engine
    /// \[zones\](<https://cloud.google.com/compute/docs/zones#available>) in which the
    /// cluster's nodes should be located. Changing the locations a cluster is in
    /// will result in nodes being either created or removed from the cluster,
    /// depending on whether locations are being added or removed.
    ///
    /// This list must always include the cluster's primary zone.
    #[prost(string, repeated, tag = "4")]
    pub locations: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The name (project, location, cluster) of the cluster to set locations.
    /// Specified in the format `projects/*/locations/*/clusters/*`.
    #[prost(string, tag = "6")]
    pub name: ::prost::alloc::string::String,
}
/// UpdateMasterRequest updates the master of the cluster.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateMasterRequest {
    /// Required. Deprecated. The Google Developers Console [project ID or project
    /// number](<https://cloud.google.com/resource-manager/docs/creating-managing-projects>).
    /// This field has been deprecated and replaced by the name field.
    #[deprecated]
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    /// Required. Deprecated. The name of the Google Compute Engine
    /// \[zone\](<https://cloud.google.com/compute/docs/zones#available>) in which the
    /// cluster resides. This field has been deprecated and replaced by the name
    /// field.
    #[deprecated]
    #[prost(string, tag = "2")]
    pub zone: ::prost::alloc::string::String,
    /// Required. Deprecated. The name of the cluster to upgrade.
    /// This field has been deprecated and replaced by the name field.
    #[deprecated]
    #[prost(string, tag = "3")]
    pub cluster_id: ::prost::alloc::string::String,
    /// Required. The Kubernetes version to change the master to.
    ///
    /// Users may specify either explicit versions offered by
    /// Kubernetes Engine or version aliases, which have the following behavior:
    ///
    /// - "latest": picks the highest valid Kubernetes version
    /// - "1.X": picks the highest valid patch+gke.N patch in the 1.X version
    /// - "1.X.Y": picks the highest valid gke.N patch in the 1.X.Y version
    /// - "1.X.Y-gke.N": picks an explicit Kubernetes version
    /// - "-": picks the default Kubernetes version
    #[prost(string, tag = "4")]
    pub master_version: ::prost::alloc::string::String,
    /// The name (project, location, cluster) of the cluster to update.
    /// Specified in the format `projects/*/locations/*/clusters/*`.
    #[prost(string, tag = "7")]
    pub name: ::prost::alloc::string::String,
}
/// SetMasterAuthRequest updates the admin password of a cluster.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetMasterAuthRequest {
    /// Required. Deprecated. The Google Developers Console [project ID or project
    /// number](<https://cloud.google.com/resource-manager/docs/creating-managing-projects>).
    /// This field has been deprecated and replaced by the name field.
    #[deprecated]
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    /// Required. Deprecated. The name of the Google Compute Engine
    /// \[zone\](<https://cloud.google.com/compute/docs/zones#available>) in which the
    /// cluster resides. This field has been deprecated and replaced by the name
    /// field.
    #[deprecated]
    #[prost(string, tag = "2")]
    pub zone: ::prost::alloc::string::String,
    /// Required. Deprecated. The name of the cluster to upgrade.
    /// This field has been deprecated and replaced by the name field.
    #[deprecated]
    #[prost(string, tag = "3")]
    pub cluster_id: ::prost::alloc::string::String,
    /// Required. The exact form of action to be taken on the master auth.
    #[prost(enumeration = "set_master_auth_request::Action", tag = "4")]
    pub action: i32,
    /// Required. A description of the update.
    #[prost(message, optional, tag = "5")]
    pub update: ::core::option::Option<MasterAuth>,
    /// The name (project, location, cluster) of the cluster to set auth.
    /// Specified in the format `projects/*/locations/*/clusters/*`.
    #[prost(string, tag = "7")]
    pub name: ::prost::alloc::string::String,
}
/// Nested message and enum types in `SetMasterAuthRequest`.
pub mod set_master_auth_request {
    /// Operation type: what type update to perform.
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
        /// Operation is unknown and will error out.
        Unknown = 0,
        /// Set the password to a user generated value.
        SetPassword = 1,
        /// Generate a new password and set it to that.
        GeneratePassword = 2,
        /// Set the username.  If an empty username is provided, basic authentication
        /// is disabled for the cluster.  If a non-empty username is provided, basic
        /// authentication is enabled, with either a provided password or a generated
        /// one.
        SetUsername = 3,
    }
    impl Action {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Action::Unknown => "UNKNOWN",
                Action::SetPassword => "SET_PASSWORD",
                Action::GeneratePassword => "GENERATE_PASSWORD",
                Action::SetUsername => "SET_USERNAME",
            }
        }
    }
}
/// DeleteClusterRequest deletes a cluster.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteClusterRequest {
    /// Required. Deprecated. The Google Developers Console [project ID or project
    /// number](<https://cloud.google.com/resource-manager/docs/creating-managing-projects>).
    /// This field has been deprecated and replaced by the name field.
    #[deprecated]
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    /// Required. Deprecated. The name of the Google Compute Engine
    /// \[zone\](<https://cloud.google.com/compute/docs/zones#available>) in which the
    /// cluster resides. This field has been deprecated and replaced by the name
    /// field.
    #[deprecated]
    #[prost(string, tag = "2")]
    pub zone: ::prost::alloc::string::String,
    /// Required. Deprecated. The name of the cluster to delete.
    /// This field has been deprecated and replaced by the name field.
    #[deprecated]
    #[prost(string, tag = "3")]
    pub cluster_id: ::prost::alloc::string::String,
    /// The name (project, location, cluster) of the cluster to delete.
    /// Specified in the format `projects/*/locations/*/clusters/*`.
    #[prost(string, tag = "4")]
    pub name: ::prost::alloc::string::String,
}
/// ListClustersRequest lists clusters.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListClustersRequest {
    /// Required. Deprecated. The Google Developers Console [project ID or project
    /// number](<https://cloud.google.com/resource-manager/docs/creating-managing-projects>).
    /// This field has been deprecated and replaced by the parent field.
    #[deprecated]
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    /// Required. Deprecated. The name of the Google Compute Engine
    /// \[zone\](<https://cloud.google.com/compute/docs/zones#available>) in which the
    /// cluster resides, or "-" for all zones. This field has been deprecated and
    /// replaced by the parent field.
    #[deprecated]
    #[prost(string, tag = "2")]
    pub zone: ::prost::alloc::string::String,
    /// The parent (project and location) where the clusters will be listed.
    /// Specified in the format `projects/*/locations/*`.
    /// Location "-" matches all zones and all regions.
    #[prost(string, tag = "4")]
    pub parent: ::prost::alloc::string::String,
}
/// ListClustersResponse is the result of ListClustersRequest.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListClustersResponse {
    /// A list of clusters in the project in the specified zone, or
    /// across all ones.
    #[prost(message, repeated, tag = "1")]
    pub clusters: ::prost::alloc::vec::Vec<Cluster>,
    /// If any zones are listed here, the list of clusters returned
    /// may be missing those zones.
    #[prost(string, repeated, tag = "2")]
    pub missing_zones: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// GetOperationRequest gets a single operation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetOperationRequest {
    /// Required. Deprecated. The Google Developers Console [project ID or project
    /// number](<https://cloud.google.com/resource-manager/docs/creating-managing-projects>).
    /// This field has been deprecated and replaced by the name field.
    #[deprecated]
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    /// Required. Deprecated. The name of the Google Compute Engine
    /// \[zone\](<https://cloud.google.com/compute/docs/zones#available>) in which the
    /// cluster resides. This field has been deprecated and replaced by the name
    /// field.
    #[deprecated]
    #[prost(string, tag = "2")]
    pub zone: ::prost::alloc::string::String,
    /// Required. Deprecated. The server-assigned `name` of the operation.
    /// This field has been deprecated and replaced by the name field.
    #[deprecated]
    #[prost(string, tag = "3")]
    pub operation_id: ::prost::alloc::string::String,
    /// The name (project, location, operation id) of the operation to get.
    /// Specified in the format `projects/*/locations/*/operations/*`.
    #[prost(string, tag = "5")]
    pub name: ::prost::alloc::string::String,
}
/// ListOperationsRequest lists operations.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListOperationsRequest {
    /// Required. Deprecated. The Google Developers Console [project ID or project
    /// number](<https://cloud.google.com/resource-manager/docs/creating-managing-projects>).
    /// This field has been deprecated and replaced by the parent field.
    #[deprecated]
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    /// Required. Deprecated. The name of the Google Compute Engine
    /// \[zone\](<https://cloud.google.com/compute/docs/zones#available>) to return
    /// operations for, or `-` for all zones. This field has been deprecated and
    /// replaced by the parent field.
    #[deprecated]
    #[prost(string, tag = "2")]
    pub zone: ::prost::alloc::string::String,
    /// The parent (project and location) where the operations will be listed.
    /// Specified in the format `projects/*/locations/*`.
    /// Location "-" matches all zones and all regions.
    #[prost(string, tag = "4")]
    pub parent: ::prost::alloc::string::String,
}
/// CancelOperationRequest cancels a single operation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CancelOperationRequest {
    /// Required. Deprecated. The Google Developers Console [project ID or project
    /// number](<https://cloud.google.com/resource-manager/docs/creating-managing-projects>).
    /// This field has been deprecated and replaced by the name field.
    #[deprecated]
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    /// Required. Deprecated. The name of the Google Compute Engine
    /// \[zone\](<https://cloud.google.com/compute/docs/zones#available>) in which the
    /// operation resides. This field has been deprecated and replaced by the name
    /// field.
    #[deprecated]
    #[prost(string, tag = "2")]
    pub zone: ::prost::alloc::string::String,
    /// Required. Deprecated. The server-assigned `name` of the operation.
    /// This field has been deprecated and replaced by the name field.
    #[deprecated]
    #[prost(string, tag = "3")]
    pub operation_id: ::prost::alloc::string::String,
    /// The name (project, location, operation id) of the operation to cancel.
    /// Specified in the format `projects/*/locations/*/operations/*`.
    #[prost(string, tag = "4")]
    pub name: ::prost::alloc::string::String,
}
/// ListOperationsResponse is the result of ListOperationsRequest.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListOperationsResponse {
    /// A list of operations in the project in the specified zone.
    #[prost(message, repeated, tag = "1")]
    pub operations: ::prost::alloc::vec::Vec<Operation>,
    /// If any zones are listed here, the list of operations returned
    /// may be missing the operations from those zones.
    #[prost(string, repeated, tag = "2")]
    pub missing_zones: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Gets the current Kubernetes Engine service configuration.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetServerConfigRequest {
    /// Required. Deprecated. The Google Developers Console [project ID or project
    /// number](<https://cloud.google.com/resource-manager/docs/creating-managing-projects>).
    /// This field has been deprecated and replaced by the name field.
    #[deprecated]
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    /// Required. Deprecated. The name of the Google Compute Engine
    /// \[zone\](<https://cloud.google.com/compute/docs/zones#available>) to return
    /// operations for. This field has been deprecated and replaced by the name
    /// field.
    #[deprecated]
    #[prost(string, tag = "2")]
    pub zone: ::prost::alloc::string::String,
    /// The name (project and location) of the server config to get,
    /// specified in the format `projects/*/locations/*`.
    #[prost(string, tag = "4")]
    pub name: ::prost::alloc::string::String,
}
/// Kubernetes Engine service configuration.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerConfig {
    /// Version of Kubernetes the service deploys by default.
    #[prost(string, tag = "1")]
    pub default_cluster_version: ::prost::alloc::string::String,
    /// List of valid node upgrade target versions, in descending order.
    #[prost(string, repeated, tag = "3")]
    pub valid_node_versions: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Default image type.
    #[prost(string, tag = "4")]
    pub default_image_type: ::prost::alloc::string::String,
    /// List of valid image types.
    #[prost(string, repeated, tag = "5")]
    pub valid_image_types: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// List of valid master versions, in descending order.
    #[prost(string, repeated, tag = "6")]
    pub valid_master_versions: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// List of release channel configurations.
    #[prost(message, repeated, tag = "9")]
    pub channels: ::prost::alloc::vec::Vec<server_config::ReleaseChannelConfig>,
    /// Maps of Kubernetes version and supported Windows server versions.
    #[prost(map = "string, message", tag = "10")]
    pub windows_version_maps: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        WindowsVersions,
    >,
}
/// Nested message and enum types in `ServerConfig`.
pub mod server_config {
    /// ReleaseChannelConfig exposes configuration for a release channel.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ReleaseChannelConfig {
        /// The release channel this configuration applies to.
        #[prost(enumeration = "super::release_channel::Channel", tag = "1")]
        pub channel: i32,
        /// The default version for newly created clusters on the channel.
        #[prost(string, tag = "2")]
        pub default_version: ::prost::alloc::string::String,
        /// Deprecated.
        /// This field has been deprecated and replaced with the valid_versions
        /// field.
        #[deprecated]
        #[prost(message, repeated, tag = "3")]
        pub available_versions: ::prost::alloc::vec::Vec<
            release_channel_config::AvailableVersion,
        >,
        /// List of valid versions for the channel.
        #[prost(string, repeated, tag = "4")]
        pub valid_versions: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
    /// Nested message and enum types in `ReleaseChannelConfig`.
    pub mod release_channel_config {
        /// Deprecated.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct AvailableVersion {
            /// Kubernetes version.
            #[prost(string, tag = "1")]
            pub version: ::prost::alloc::string::String,
            /// Reason for availability.
            #[prost(string, tag = "2")]
            pub reason: ::prost::alloc::string::String,
        }
    }
}
/// Windows server versions.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WindowsVersions {
    /// List of Windows server versions.
    #[prost(message, repeated, tag = "1")]
    pub windows_versions: ::prost::alloc::vec::Vec<windows_versions::WindowsVersion>,
}
/// Nested message and enum types in `WindowsVersions`.
pub mod windows_versions {
    /// Windows server version.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct WindowsVersion {
        /// Windows server image type
        #[prost(string, tag = "1")]
        pub image_type: ::prost::alloc::string::String,
        /// Windows server build number
        #[prost(string, tag = "2")]
        pub os_version: ::prost::alloc::string::String,
        /// Mainstream support end date
        #[prost(message, optional, tag = "3")]
        pub support_end_date: ::core::option::Option<super::super::super::r#type::Date>,
    }
}
/// CreateNodePoolRequest creates a node pool for a cluster.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateNodePoolRequest {
    /// Required. Deprecated. The Google Developers Console [project ID or project
    /// number](<https://cloud.google.com/resource-manager/docs/creating-managing-projects>).
    /// This field has been deprecated and replaced by the parent field.
    #[deprecated]
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    /// Required. Deprecated. The name of the Google Compute Engine
    /// \[zone\](<https://cloud.google.com/compute/docs/zones#available>) in which the
    /// cluster resides. This field has been deprecated and replaced by the parent
    /// field.
    #[deprecated]
    #[prost(string, tag = "2")]
    pub zone: ::prost::alloc::string::String,
    /// Required. Deprecated. The name of the cluster.
    /// This field has been deprecated and replaced by the parent field.
    #[deprecated]
    #[prost(string, tag = "3")]
    pub cluster_id: ::prost::alloc::string::String,
    /// Required. The node pool to create.
    #[prost(message, optional, tag = "4")]
    pub node_pool: ::core::option::Option<NodePool>,
    /// The parent (project, location, cluster name) where the node pool will be
    /// created. Specified in the format
    /// `projects/*/locations/*/clusters/*`.
    #[prost(string, tag = "6")]
    pub parent: ::prost::alloc::string::String,
}
/// DeleteNodePoolRequest deletes a node pool for a cluster.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteNodePoolRequest {
    /// Required. Deprecated. The Google Developers Console [project ID or project
    /// number](<https://cloud.google.com/resource-manager/docs/creating-managing-projects>).
    /// This field has been deprecated and replaced by the name field.
    #[deprecated]
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    /// Required. Deprecated. The name of the Google Compute Engine
    /// \[zone\](<https://cloud.google.com/compute/docs/zones#available>) in which the
    /// cluster resides. This field has been deprecated and replaced by the name
    /// field.
    #[deprecated]
    #[prost(string, tag = "2")]
    pub zone: ::prost::alloc::string::String,
    /// Required. Deprecated. The name of the cluster.
    /// This field has been deprecated and replaced by the name field.
    #[deprecated]
    #[prost(string, tag = "3")]
    pub cluster_id: ::prost::alloc::string::String,
    /// Required. Deprecated. The name of the node pool to delete.
    /// This field has been deprecated and replaced by the name field.
    #[deprecated]
    #[prost(string, tag = "4")]
    pub node_pool_id: ::prost::alloc::string::String,
    /// The name (project, location, cluster, node pool id) of the node pool to
    /// delete. Specified in the format
    /// `projects/*/locations/*/clusters/*/nodePools/*`.
    #[prost(string, tag = "6")]
    pub name: ::prost::alloc::string::String,
}
/// ListNodePoolsRequest lists the node pool(s) for a cluster.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListNodePoolsRequest {
    /// Required. Deprecated. The Google Developers Console [project ID or project
    /// number](<https://cloud.google.com/resource-manager/docs/creating-managing-projects>).
    /// This field has been deprecated and replaced by the parent field.
    #[deprecated]
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    /// Required. Deprecated. The name of the Google Compute Engine
    /// \[zone\](<https://cloud.google.com/compute/docs/zones#available>) in which the
    /// cluster resides. This field has been deprecated and replaced by the parent
    /// field.
    #[deprecated]
    #[prost(string, tag = "2")]
    pub zone: ::prost::alloc::string::String,
    /// Required. Deprecated. The name of the cluster.
    /// This field has been deprecated and replaced by the parent field.
    #[deprecated]
    #[prost(string, tag = "3")]
    pub cluster_id: ::prost::alloc::string::String,
    /// The parent (project, location, cluster name) where the node pools will be
    /// listed. Specified in the format `projects/*/locations/*/clusters/*`.
    #[prost(string, tag = "5")]
    pub parent: ::prost::alloc::string::String,
}
/// GetNodePoolRequest retrieves a node pool for a cluster.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetNodePoolRequest {
    /// Required. Deprecated. The Google Developers Console [project ID or project
    /// number](<https://cloud.google.com/resource-manager/docs/creating-managing-projects>).
    /// This field has been deprecated and replaced by the name field.
    #[deprecated]
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    /// Required. Deprecated. The name of the Google Compute Engine
    /// \[zone\](<https://cloud.google.com/compute/docs/zones#available>) in which the
    /// cluster resides. This field has been deprecated and replaced by the name
    /// field.
    #[deprecated]
    #[prost(string, tag = "2")]
    pub zone: ::prost::alloc::string::String,
    /// Required. Deprecated. The name of the cluster.
    /// This field has been deprecated and replaced by the name field.
    #[deprecated]
    #[prost(string, tag = "3")]
    pub cluster_id: ::prost::alloc::string::String,
    /// Required. Deprecated. The name of the node pool.
    /// This field has been deprecated and replaced by the name field.
    #[deprecated]
    #[prost(string, tag = "4")]
    pub node_pool_id: ::prost::alloc::string::String,
    /// The name (project, location, cluster, node pool id) of the node pool to
    /// get. Specified in the format
    /// `projects/*/locations/*/clusters/*/nodePools/*`.
    #[prost(string, tag = "6")]
    pub name: ::prost::alloc::string::String,
}
/// Settings for blue-green upgrade.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlueGreenSettings {
    /// Time needed after draining entire blue pool. After this period, blue pool
    /// will be cleaned up.
    #[prost(message, optional, tag = "2")]
    pub node_pool_soak_duration: ::core::option::Option<::prost_types::Duration>,
    /// The rollout policy controls the general rollout progress of blue-green.
    #[prost(oneof = "blue_green_settings::RolloutPolicy", tags = "1")]
    pub rollout_policy: ::core::option::Option<blue_green_settings::RolloutPolicy>,
}
/// Nested message and enum types in `BlueGreenSettings`.
pub mod blue_green_settings {
    /// Standard rollout policy is the default policy for blue-green.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct StandardRolloutPolicy {
        /// Soak time after each batch gets drained. Default to zero.
        #[prost(message, optional, tag = "3")]
        pub batch_soak_duration: ::core::option::Option<::prost_types::Duration>,
        /// Blue pool size to drain in a batch.
        #[prost(oneof = "standard_rollout_policy::UpdateBatchSize", tags = "1, 2")]
        pub update_batch_size: ::core::option::Option<
            standard_rollout_policy::UpdateBatchSize,
        >,
    }
    /// Nested message and enum types in `StandardRolloutPolicy`.
    pub mod standard_rollout_policy {
        /// Blue pool size to drain in a batch.
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum UpdateBatchSize {
            /// Percentage of the blue pool nodes to drain in a batch.
            /// The range of this field should be (0.0, 1.0].
            #[prost(float, tag = "1")]
            BatchPercentage(f32),
            /// Number of blue nodes to drain in a batch.
            #[prost(int32, tag = "2")]
            BatchNodeCount(i32),
        }
    }
    /// The rollout policy controls the general rollout progress of blue-green.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum RolloutPolicy {
        /// Standard policy for the blue-green upgrade.
        #[prost(message, tag = "1")]
        StandardRolloutPolicy(StandardRolloutPolicy),
    }
}
/// NodePool contains the name and configuration for a cluster's node pool.
/// Node pools are a set of nodes (i.e. VM's), with a common configuration and
/// specification, under the control of the cluster master. They may have a set
/// of Kubernetes labels applied to them, which may be used to reference them
/// during pod scheduling. They may also be resized up or down, to accommodate
/// the workload.
/// These upgrade settings control the level of parallelism and the level of
/// disruption caused by an upgrade.
///
/// maxUnavailable controls the number of nodes that can be simultaneously
/// unavailable.
///
/// maxSurge controls the number of additional nodes that can be added to the
/// node pool temporarily for the time of the upgrade to increase the number of
/// available nodes.
///
/// (maxUnavailable + maxSurge) determines the level of parallelism (how many
/// nodes are being upgraded at the same time).
///
/// Note: upgrades inevitably introduce some disruption since workloads need to
/// be moved from old nodes to new, upgraded ones. Even if maxUnavailable=0,
/// this holds true. (Disruption stays within the limits of
/// PodDisruptionBudget, if it is configured.)
///
/// Consider a hypothetical node pool with 5 nodes having maxSurge=2,
/// maxUnavailable=1. This means the upgrade process upgrades 3 nodes
/// simultaneously. It creates 2 additional (upgraded) nodes, then it brings
/// down 3 old (not yet upgraded) nodes at the same time. This ensures that
/// there are always at least 4 nodes available.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NodePool {
    /// The name of the node pool.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The node configuration of the pool.
    #[prost(message, optional, tag = "2")]
    pub config: ::core::option::Option<NodeConfig>,
    /// The initial node count for the pool. You must ensure that your
    /// Compute Engine [resource quota](<https://cloud.google.com/compute/quotas>)
    /// is sufficient for this number of instances. You must also have available
    /// firewall and routes quota.
    #[prost(int32, tag = "3")]
    pub initial_node_count: i32,
    /// The list of Google Compute Engine
    /// \[zones\](<https://cloud.google.com/compute/docs/zones#available>) in which the
    /// NodePool's nodes should be located.
    ///
    /// If this value is unspecified during node pool creation, the
    /// \[Cluster.Locations\](<https://cloud.google.com/kubernetes-engine/docs/reference/rest/v1/projects.locations.clusters#Cluster.FIELDS.locations>)
    /// value will be used, instead.
    ///
    /// Warning: changing node pool locations will result in nodes being added
    /// and/or removed.
    #[prost(string, repeated, tag = "13")]
    pub locations: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Networking configuration for this NodePool. If specified, it overrides the
    /// cluster-level defaults.
    #[prost(message, optional, tag = "14")]
    pub network_config: ::core::option::Option<NodeNetworkConfig>,
    /// [Output only] Server-defined URL for the resource.
    #[prost(string, tag = "100")]
    pub self_link: ::prost::alloc::string::String,
    /// The version of the Kubernetes of this node.
    #[prost(string, tag = "101")]
    pub version: ::prost::alloc::string::String,
    /// [Output only] The resource URLs of the [managed instance
    /// groups](<https://cloud.google.com/compute/docs/instance-groups/creating-groups-of-managed-instances>)
    /// associated with this node pool.
    /// During the node pool blue-green upgrade operation, the URLs contain both
    /// blue and green resources.
    #[prost(string, repeated, tag = "102")]
    pub instance_group_urls: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// [Output only] The status of the nodes in this pool instance.
    #[prost(enumeration = "node_pool::Status", tag = "103")]
    pub status: i32,
    /// [Output only] Deprecated. Use conditions instead.
    /// Additional information about the current status of this
    /// node pool instance, if available.
    #[deprecated]
    #[prost(string, tag = "104")]
    pub status_message: ::prost::alloc::string::String,
    /// Autoscaler configuration for this NodePool. Autoscaler is enabled
    /// only if a valid configuration is present.
    #[prost(message, optional, tag = "4")]
    pub autoscaling: ::core::option::Option<NodePoolAutoscaling>,
    /// NodeManagement configuration for this NodePool.
    #[prost(message, optional, tag = "5")]
    pub management: ::core::option::Option<NodeManagement>,
    /// The constraint on the maximum number of pods that can be run
    /// simultaneously on a node in the node pool.
    #[prost(message, optional, tag = "6")]
    pub max_pods_constraint: ::core::option::Option<MaxPodsConstraint>,
    /// Which conditions caused the current node pool state.
    #[prost(message, repeated, tag = "105")]
    pub conditions: ::prost::alloc::vec::Vec<StatusCondition>,
    /// [Output only] The pod CIDR block size per node in this node pool.
    #[prost(int32, tag = "7")]
    pub pod_ipv4_cidr_size: i32,
    /// Upgrade settings control disruption and speed of the upgrade.
    #[prost(message, optional, tag = "107")]
    pub upgrade_settings: ::core::option::Option<node_pool::UpgradeSettings>,
    /// Specifies the node placement policy.
    #[prost(message, optional, tag = "108")]
    pub placement_policy: ::core::option::Option<node_pool::PlacementPolicy>,
    /// Output only. [Output only] Update info contains relevant information during a node
    /// pool update.
    #[prost(message, optional, tag = "109")]
    pub update_info: ::core::option::Option<node_pool::UpdateInfo>,
}
/// Nested message and enum types in `NodePool`.
pub mod node_pool {
    /// These upgrade settings configure the upgrade strategy for the node pool.
    /// Use strategy to switch between the strategies applied to the node pool.
    ///
    /// If the strategy is SURGE, use max_surge and max_unavailable to control
    /// the level of parallelism and the level of disruption caused by upgrade.
    /// 1. maxSurge controls the number of additional nodes that can be added to
    /// the node pool temporarily for the time of the upgrade to increase the
    /// number of available nodes.
    /// 2. maxUnavailable controls the number of nodes that can be simultaneously
    /// unavailable.
    /// 3. (maxUnavailable + maxSurge) determines the level of parallelism (how
    /// many nodes are being upgraded at the same time).
    ///
    /// If the strategy is BLUE_GREEN, use blue_green_settings to configure the
    /// blue-green upgrade related settings.
    /// 1. standard_rollout_policy is the default policy. The policy is used to
    /// control the way blue pool gets drained. The draining is executed in the
    /// batch mode. The batch size could be specified as either percentage of the
    /// node pool size or the number of nodes. batch_soak_duration is the soak
    /// time after each batch gets drained.
    /// 2. node_pool_soak_duration is the soak time after all blue nodes are
    /// drained. After this period, the blue pool nodes will be deleted.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct UpgradeSettings {
        /// The maximum number of nodes that can be created beyond the current size
        /// of the node pool during the upgrade process.
        #[prost(int32, tag = "1")]
        pub max_surge: i32,
        /// The maximum number of nodes that can be simultaneously unavailable during
        /// the upgrade process. A node is considered available if its status is
        /// Ready.
        #[prost(int32, tag = "2")]
        pub max_unavailable: i32,
        /// Update strategy of the node pool.
        #[prost(enumeration = "super::NodePoolUpdateStrategy", optional, tag = "3")]
        pub strategy: ::core::option::Option<i32>,
        /// Settings for blue-green upgrade strategy.
        #[prost(message, optional, tag = "4")]
        pub blue_green_settings: ::core::option::Option<super::BlueGreenSettings>,
    }
    /// UpdateInfo contains resource (instance groups, etc), status and other
    /// intermediate information relevant to a node pool upgrade.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct UpdateInfo {
        /// Information of a blue-green upgrade.
        #[prost(message, optional, tag = "1")]
        pub blue_green_info: ::core::option::Option<update_info::BlueGreenInfo>,
    }
    /// Nested message and enum types in `UpdateInfo`.
    pub mod update_info {
        /// Information relevant to blue-green upgrade.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct BlueGreenInfo {
            /// Current blue-green upgrade phase.
            #[prost(enumeration = "blue_green_info::Phase", tag = "1")]
            pub phase: i32,
            /// The resource URLs of the [managed instance groups]
            /// (/compute/docs/instance-groups/creating-groups-of-managed-instances)
            /// associated with blue pool.
            #[prost(string, repeated, tag = "2")]
            pub blue_instance_group_urls: ::prost::alloc::vec::Vec<
                ::prost::alloc::string::String,
            >,
            /// The resource URLs of the [managed instance groups]
            /// (/compute/docs/instance-groups/creating-groups-of-managed-instances)
            /// associated with green pool.
            #[prost(string, repeated, tag = "3")]
            pub green_instance_group_urls: ::prost::alloc::vec::Vec<
                ::prost::alloc::string::String,
            >,
            /// Time to start deleting blue pool to complete blue-green upgrade,
            /// in \[RFC3339\](<https://www.ietf.org/rfc/rfc3339.txt>) text format.
            #[prost(string, tag = "4")]
            pub blue_pool_deletion_start_time: ::prost::alloc::string::String,
            /// Version of green pool.
            #[prost(string, tag = "5")]
            pub green_pool_version: ::prost::alloc::string::String,
        }
        /// Nested message and enum types in `BlueGreenInfo`.
        pub mod blue_green_info {
            /// Phase represents the different stages blue-green upgrade is running in.
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
            pub enum Phase {
                /// Unspecified phase.
                Unspecified = 0,
                /// blue-green upgrade has been initiated.
                UpdateStarted = 1,
                /// Start creating green pool nodes.
                CreatingGreenPool = 2,
                /// Start cordoning blue pool nodes.
                CordoningBluePool = 3,
                /// Start draining blue pool nodes.
                DrainingBluePool = 4,
                /// Start soaking time after draining entire blue pool.
                NodePoolSoaking = 5,
                /// Start deleting blue nodes.
                DeletingBluePool = 6,
                /// Rollback has been initiated.
                RollbackStarted = 7,
            }
            impl Phase {
                /// String value of the enum field names used in the ProtoBuf definition.
                ///
                /// The values are not transformed in any way and thus are considered stable
                /// (if the ProtoBuf definition does not change) and safe for programmatic use.
                pub fn as_str_name(&self) -> &'static str {
                    match self {
                        Phase::Unspecified => "PHASE_UNSPECIFIED",
                        Phase::UpdateStarted => "UPDATE_STARTED",
                        Phase::CreatingGreenPool => "CREATING_GREEN_POOL",
                        Phase::CordoningBluePool => "CORDONING_BLUE_POOL",
                        Phase::DrainingBluePool => "DRAINING_BLUE_POOL",
                        Phase::NodePoolSoaking => "NODE_POOL_SOAKING",
                        Phase::DeletingBluePool => "DELETING_BLUE_POOL",
                        Phase::RollbackStarted => "ROLLBACK_STARTED",
                    }
                }
            }
        }
    }
    /// PlacementPolicy defines the placement policy used by the node pool.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PlacementPolicy {
        /// The type of placement.
        #[prost(enumeration = "placement_policy::Type", tag = "1")]
        pub r#type: i32,
    }
    /// Nested message and enum types in `PlacementPolicy`.
    pub mod placement_policy {
        /// Type defines the type of placement policy.
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
            /// TYPE_UNSPECIFIED specifies no requirements on nodes
            /// placement.
            Unspecified = 0,
            /// COMPACT specifies node placement in the same availability domain to
            /// ensure low communication latency.
            Compact = 1,
        }
        impl Type {
            /// String value of the enum field names used in the ProtoBuf definition.
            ///
            /// The values are not transformed in any way and thus are considered stable
            /// (if the ProtoBuf definition does not change) and safe for programmatic use.
            pub fn as_str_name(&self) -> &'static str {
                match self {
                    Type::Unspecified => "TYPE_UNSPECIFIED",
                    Type::Compact => "COMPACT",
                }
            }
        }
    }
    /// The current status of the node pool instance.
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
    pub enum Status {
        /// Not set.
        Unspecified = 0,
        /// The PROVISIONING state indicates the node pool is being created.
        Provisioning = 1,
        /// The RUNNING state indicates the node pool has been created
        /// and is fully usable.
        Running = 2,
        /// The RUNNING_WITH_ERROR state indicates the node pool has been created
        /// and is partially usable. Some error state has occurred and some
        /// functionality may be impaired. Customer may need to reissue a request
        /// or trigger a new update.
        RunningWithError = 3,
        /// The RECONCILING state indicates that some work is actively being done on
        /// the node pool, such as upgrading node software. Details can
        /// be found in the `statusMessage` field.
        Reconciling = 4,
        /// The STOPPING state indicates the node pool is being deleted.
        Stopping = 5,
        /// The ERROR state indicates the node pool may be unusable. Details
        /// can be found in the `statusMessage` field.
        Error = 6,
    }
    impl Status {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Status::Unspecified => "STATUS_UNSPECIFIED",
                Status::Provisioning => "PROVISIONING",
                Status::Running => "RUNNING",
                Status::RunningWithError => "RUNNING_WITH_ERROR",
                Status::Reconciling => "RECONCILING",
                Status::Stopping => "STOPPING",
                Status::Error => "ERROR",
            }
        }
    }
}
/// NodeManagement defines the set of node management services turned on for the
/// node pool.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NodeManagement {
    /// Whether the nodes will be automatically upgraded.
    #[prost(bool, tag = "1")]
    pub auto_upgrade: bool,
    /// Whether the nodes will be automatically repaired.
    #[prost(bool, tag = "2")]
    pub auto_repair: bool,
    /// Specifies the Auto Upgrade knobs for the node pool.
    #[prost(message, optional, tag = "10")]
    pub upgrade_options: ::core::option::Option<AutoUpgradeOptions>,
}
/// AutoUpgradeOptions defines the set of options for the user to control how
/// the Auto Upgrades will proceed.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AutoUpgradeOptions {
    /// [Output only] This field is set when upgrades are about to commence
    /// with the approximate start time for the upgrades, in
    /// \[RFC3339\](<https://www.ietf.org/rfc/rfc3339.txt>) text format.
    #[prost(string, tag = "1")]
    pub auto_upgrade_start_time: ::prost::alloc::string::String,
    /// [Output only] This field is set when upgrades are about to commence
    /// with the description of the upgrade.
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
}
/// MaintenancePolicy defines the maintenance policy to be used for the cluster.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MaintenancePolicy {
    /// Specifies the maintenance window in which maintenance may be performed.
    #[prost(message, optional, tag = "1")]
    pub window: ::core::option::Option<MaintenanceWindow>,
    /// A hash identifying the version of this policy, so that updates to fields of
    /// the policy won't accidentally undo intermediate changes (and so that users
    /// of the API unaware of some fields won't accidentally remove other fields).
    /// Make a `get()` request to the cluster to get the current
    /// resource version and include it with requests to set the policy.
    #[prost(string, tag = "3")]
    pub resource_version: ::prost::alloc::string::String,
}
/// MaintenanceWindow defines the maintenance window to be used for the cluster.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MaintenanceWindow {
    /// Exceptions to maintenance window. Non-emergency maintenance should not
    /// occur in these windows.
    #[prost(map = "string, message", tag = "4")]
    pub maintenance_exclusions: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        TimeWindow,
    >,
    /// Unimplemented, reserved for future use.
    /// HourlyMaintenanceWindow hourly_maintenance_window = 1;
    #[prost(oneof = "maintenance_window::Policy", tags = "2, 3")]
    pub policy: ::core::option::Option<maintenance_window::Policy>,
}
/// Nested message and enum types in `MaintenanceWindow`.
pub mod maintenance_window {
    /// Unimplemented, reserved for future use.
    /// HourlyMaintenanceWindow hourly_maintenance_window = 1;
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Policy {
        /// DailyMaintenanceWindow specifies a daily maintenance operation window.
        #[prost(message, tag = "2")]
        DailyMaintenanceWindow(super::DailyMaintenanceWindow),
        /// RecurringWindow specifies some number of recurring time periods for
        /// maintenance to occur. The time windows may be overlapping. If no
        /// maintenance windows are set, maintenance can occur at any time.
        #[prost(message, tag = "3")]
        RecurringWindow(super::RecurringTimeWindow),
    }
}
/// Represents an arbitrary window of time.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TimeWindow {
    /// The time that the window first starts.
    #[prost(message, optional, tag = "1")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The time that the window ends. The end time should take place after the
    /// start time.
    #[prost(message, optional, tag = "2")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(oneof = "time_window::Options", tags = "3")]
    pub options: ::core::option::Option<time_window::Options>,
}
/// Nested message and enum types in `TimeWindow`.
pub mod time_window {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Options {
        /// MaintenanceExclusionOptions provides maintenance exclusion related
        /// options.
        #[prost(message, tag = "3")]
        MaintenanceExclusionOptions(super::MaintenanceExclusionOptions),
    }
}
/// Represents the Maintenance exclusion option.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MaintenanceExclusionOptions {
    /// Scope specifies the upgrade scope which upgrades are blocked by the
    /// exclusion.
    #[prost(enumeration = "maintenance_exclusion_options::Scope", tag = "1")]
    pub scope: i32,
}
/// Nested message and enum types in `MaintenanceExclusionOptions`.
pub mod maintenance_exclusion_options {
    /// Scope of exclusion.
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
    pub enum Scope {
        /// NO_UPGRADES excludes all upgrades, including patch upgrades and minor
        /// upgrades across control planes and nodes. This is the default exclusion
        /// behavior.
        NoUpgrades = 0,
        /// NO_MINOR_UPGRADES excludes all minor upgrades for the cluster, only
        /// patches are allowed.
        NoMinorUpgrades = 1,
        /// NO_MINOR_OR_NODE_UPGRADES excludes all minor upgrades for the cluster,
        /// and also exclude all node pool upgrades. Only control
        /// plane patches are allowed.
        NoMinorOrNodeUpgrades = 2,
    }
    impl Scope {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Scope::NoUpgrades => "NO_UPGRADES",
                Scope::NoMinorUpgrades => "NO_MINOR_UPGRADES",
                Scope::NoMinorOrNodeUpgrades => "NO_MINOR_OR_NODE_UPGRADES",
            }
        }
    }
}
/// Represents an arbitrary window of time that recurs.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RecurringTimeWindow {
    /// The window of the first recurrence.
    #[prost(message, optional, tag = "1")]
    pub window: ::core::option::Option<TimeWindow>,
    /// An RRULE (<https://tools.ietf.org/html/rfc5545#section-3.8.5.3>) for how
    /// this window reccurs. They go on for the span of time between the start and
    /// end time.
    ///
    /// For example, to have something repeat every weekday, you'd use:
    /// `FREQ=WEEKLY;BYDAY=MO,TU,WE,TH,FR`
    ///
    /// To repeat some window daily (equivalent to the DailyMaintenanceWindow):
    /// `FREQ=DAILY`
    ///
    /// For the first weekend of every month:
    /// `FREQ=MONTHLY;BYSETPOS=1;BYDAY=SA,SU`
    ///
    /// This specifies how frequently the window starts. Eg, if you wanted to have
    /// a 9-5 UTC-4 window every weekday, you'd use something like:
    /// ```
    /// start time = 2019-01-01T09:00:00-0400
    /// end time = 2019-01-01T17:00:00-0400
    /// recurrence = FREQ=WEEKLY;BYDAY=MO,TU,WE,TH,FR
    /// ```
    ///
    /// Windows can span multiple days. Eg, to make the window encompass every
    /// weekend from midnight Saturday till the last minute of Sunday UTC:
    /// ```
    /// start time = 2019-01-05T00:00:00Z
    /// end time = 2019-01-07T23:59:00Z
    /// recurrence = FREQ=WEEKLY;BYDAY=SA
    /// ```
    ///
    /// Note the start and end time's specific dates are largely arbitrary except
    /// to specify duration of the window and when it first starts.
    /// The FREQ values of HOURLY, MINUTELY, and SECONDLY are not supported.
    #[prost(string, tag = "2")]
    pub recurrence: ::prost::alloc::string::String,
}
/// Time window specified for daily maintenance operations.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DailyMaintenanceWindow {
    /// Time within the maintenance window to start the maintenance operations.
    /// It must be in format "HH:MM", where HH : \[00-23\] and MM : \[00-59\] GMT.
    #[prost(string, tag = "2")]
    pub start_time: ::prost::alloc::string::String,
    /// [Output only] Duration of the time window, automatically chosen to be
    /// smallest possible in the given scenario.
    #[prost(string, tag = "3")]
    pub duration: ::prost::alloc::string::String,
}
/// SetNodePoolManagementRequest sets the node management properties of a node
/// pool.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetNodePoolManagementRequest {
    /// Required. Deprecated. The Google Developers Console [project ID or project
    /// number](<https://cloud.google.com/resource-manager/docs/creating-managing-projects>).
    /// This field has been deprecated and replaced by the name field.
    #[deprecated]
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    /// Required. Deprecated. The name of the Google Compute Engine
    /// \[zone\](<https://cloud.google.com/compute/docs/zones#available>) in which the
    /// cluster resides. This field has been deprecated and replaced by the name
    /// field.
    #[deprecated]
    #[prost(string, tag = "2")]
    pub zone: ::prost::alloc::string::String,
    /// Required. Deprecated. The name of the cluster to update.
    /// This field has been deprecated and replaced by the name field.
    #[deprecated]
    #[prost(string, tag = "3")]
    pub cluster_id: ::prost::alloc::string::String,
    /// Required. Deprecated. The name of the node pool to update.
    /// This field has been deprecated and replaced by the name field.
    #[deprecated]
    #[prost(string, tag = "4")]
    pub node_pool_id: ::prost::alloc::string::String,
    /// Required. NodeManagement configuration for the node pool.
    #[prost(message, optional, tag = "5")]
    pub management: ::core::option::Option<NodeManagement>,
    /// The name (project, location, cluster, node pool id) of the node pool to set
    /// management properties. Specified in the format
    /// `projects/*/locations/*/clusters/*/nodePools/*`.
    #[prost(string, tag = "7")]
    pub name: ::prost::alloc::string::String,
}
/// SetNodePoolSizeRequest sets the size of a node pool.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetNodePoolSizeRequest {
    /// Required. Deprecated. The Google Developers Console [project ID or project
    /// number](<https://cloud.google.com/resource-manager/docs/creating-managing-projects>).
    /// This field has been deprecated and replaced by the name field.
    #[deprecated]
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    /// Required. Deprecated. The name of the Google Compute Engine
    /// \[zone\](<https://cloud.google.com/compute/docs/zones#available>) in which the
    /// cluster resides. This field has been deprecated and replaced by the name
    /// field.
    #[deprecated]
    #[prost(string, tag = "2")]
    pub zone: ::prost::alloc::string::String,
    /// Required. Deprecated. The name of the cluster to update.
    /// This field has been deprecated and replaced by the name field.
    #[deprecated]
    #[prost(string, tag = "3")]
    pub cluster_id: ::prost::alloc::string::String,
    /// Required. Deprecated. The name of the node pool to update.
    /// This field has been deprecated and replaced by the name field.
    #[deprecated]
    #[prost(string, tag = "4")]
    pub node_pool_id: ::prost::alloc::string::String,
    /// Required. The desired node count for the pool.
    #[prost(int32, tag = "5")]
    pub node_count: i32,
    /// The name (project, location, cluster, node pool id) of the node pool to set
    /// size.
    /// Specified in the format `projects/*/locations/*/clusters/*/nodePools/*`.
    #[prost(string, tag = "7")]
    pub name: ::prost::alloc::string::String,
}
/// CompleteNodePoolUpgradeRequest sets the name of target node pool to complete
/// upgrade.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CompleteNodePoolUpgradeRequest {
    /// The name (project, location, cluster, node pool id) of the node pool to
    /// complete upgrade.
    /// Specified in the format `projects/*/locations/*/clusters/*/nodePools/*`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// RollbackNodePoolUpgradeRequest rollbacks the previously Aborted or Failed
/// NodePool upgrade. This will be an no-op if the last upgrade successfully
/// completed.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RollbackNodePoolUpgradeRequest {
    /// Required. Deprecated. The Google Developers Console [project ID or project
    /// number](<https://cloud.google.com/resource-manager/docs/creating-managing-projects>).
    /// This field has been deprecated and replaced by the name field.
    #[deprecated]
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    /// Required. Deprecated. The name of the Google Compute Engine
    /// \[zone\](<https://cloud.google.com/compute/docs/zones#available>) in which the
    /// cluster resides. This field has been deprecated and replaced by the name
    /// field.
    #[deprecated]
    #[prost(string, tag = "2")]
    pub zone: ::prost::alloc::string::String,
    /// Required. Deprecated. The name of the cluster to rollback.
    /// This field has been deprecated and replaced by the name field.
    #[deprecated]
    #[prost(string, tag = "3")]
    pub cluster_id: ::prost::alloc::string::String,
    /// Required. Deprecated. The name of the node pool to rollback.
    /// This field has been deprecated and replaced by the name field.
    #[deprecated]
    #[prost(string, tag = "4")]
    pub node_pool_id: ::prost::alloc::string::String,
    /// The name (project, location, cluster, node pool id) of the node poll to
    /// rollback upgrade.
    /// Specified in the format `projects/*/locations/*/clusters/*/nodePools/*`.
    #[prost(string, tag = "6")]
    pub name: ::prost::alloc::string::String,
    /// Option for rollback to ignore the PodDisruptionBudget.
    /// Default value is false.
    #[prost(bool, tag = "7")]
    pub respect_pdb: bool,
}
/// ListNodePoolsResponse is the result of ListNodePoolsRequest.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListNodePoolsResponse {
    /// A list of node pools for a cluster.
    #[prost(message, repeated, tag = "1")]
    pub node_pools: ::prost::alloc::vec::Vec<NodePool>,
}
/// ClusterAutoscaling contains global, per-cluster information
/// required by Cluster Autoscaler to automatically adjust
/// the size of the cluster and create/delete
/// node pools based on the current needs.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClusterAutoscaling {
    /// Enables automatic node pool creation and deletion.
    #[prost(bool, tag = "1")]
    pub enable_node_autoprovisioning: bool,
    /// Contains global constraints regarding minimum and maximum
    /// amount of resources in the cluster.
    #[prost(message, repeated, tag = "2")]
    pub resource_limits: ::prost::alloc::vec::Vec<ResourceLimit>,
    /// Defines autoscaling behaviour.
    #[prost(enumeration = "cluster_autoscaling::AutoscalingProfile", tag = "3")]
    pub autoscaling_profile: i32,
    /// AutoprovisioningNodePoolDefaults contains defaults for a node pool
    /// created by NAP.
    #[prost(message, optional, tag = "4")]
    pub autoprovisioning_node_pool_defaults: ::core::option::Option<
        AutoprovisioningNodePoolDefaults,
    >,
    /// The list of Google Compute Engine
    /// \[zones\](<https://cloud.google.com/compute/docs/zones#available>) in which the
    /// NodePool's nodes can be created by NAP.
    #[prost(string, repeated, tag = "5")]
    pub autoprovisioning_locations: ::prost::alloc::vec::Vec<
        ::prost::alloc::string::String,
    >,
}
/// Nested message and enum types in `ClusterAutoscaling`.
pub mod cluster_autoscaling {
    /// Defines possible options for autoscaling_profile field.
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
    pub enum AutoscalingProfile {
        /// No change to autoscaling configuration.
        ProfileUnspecified = 0,
        /// Prioritize optimizing utilization of resources.
        OptimizeUtilization = 1,
        /// Use default (balanced) autoscaling configuration.
        Balanced = 2,
    }
    impl AutoscalingProfile {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                AutoscalingProfile::ProfileUnspecified => "PROFILE_UNSPECIFIED",
                AutoscalingProfile::OptimizeUtilization => "OPTIMIZE_UTILIZATION",
                AutoscalingProfile::Balanced => "BALANCED",
            }
        }
    }
}
/// AutoprovisioningNodePoolDefaults contains defaults for a node pool created
/// by NAP.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AutoprovisioningNodePoolDefaults {
    /// The set of Google API scopes to be made available on all of the
    /// node VMs under the "default" service account.
    ///
    /// The following scopes are recommended, but not required, and by default are
    /// not included:
    ///
    /// * `<https://www.googleapis.com/auth/compute`> is required for mounting
    /// persistent storage on your nodes.
    /// * `<https://www.googleapis.com/auth/devstorage.read_only`> is required for
    /// communicating with **gcr.io**
    /// (the [Google Container
    /// Registry](<https://cloud.google.com/container-registry/>)).
    ///
    /// If unspecified, no scopes are added, unless Cloud Logging or Cloud
    /// Monitoring are enabled, in which case their required scopes will be added.
    #[prost(string, repeated, tag = "1")]
    pub oauth_scopes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The Google Cloud Platform Service Account to be used by the node VMs.
    /// Specify the email address of the Service Account; otherwise, if no Service
    /// Account is specified, the "default" service account is used.
    #[prost(string, tag = "2")]
    pub service_account: ::prost::alloc::string::String,
    /// Upgrade settings control disruption and speed of the upgrade.
    #[prost(message, optional, tag = "3")]
    pub upgrade_settings: ::core::option::Option<node_pool::UpgradeSettings>,
    /// NodeManagement configuration for this NodePool.
    #[prost(message, optional, tag = "4")]
    pub management: ::core::option::Option<NodeManagement>,
    /// Deprecated. Minimum CPU platform to be used for NAP created node pools.
    /// The instance may be scheduled on the specified or newer CPU platform.
    /// Applicable values are the friendly names of CPU platforms, such as
    /// minCpuPlatform: Intel Haswell or
    /// minCpuPlatform: Intel Sandy Bridge. For more
    /// information, read [how to specify min CPU
    /// platform](<https://cloud.google.com/compute/docs/instances/specify-min-cpu-platform>).
    /// This field is deprecated, min_cpu_platform should be specified using
    /// <https://cloud.google.com/requested-min-cpu-platform> label selector on the
    /// pod.
    /// To unset the min cpu platform field pass "automatic"
    /// as field value.
    #[deprecated]
    #[prost(string, tag = "5")]
    pub min_cpu_platform: ::prost::alloc::string::String,
    /// Size of the disk attached to each node, specified in GB.
    /// The smallest allowed disk size is 10GB.
    ///
    /// If unspecified, the default disk size is 100GB.
    #[prost(int32, tag = "6")]
    pub disk_size_gb: i32,
    /// Type of the disk attached to each node (e.g. 'pd-standard', 'pd-ssd' or
    /// 'pd-balanced')
    ///
    /// If unspecified, the default disk type is 'pd-standard'
    #[prost(string, tag = "7")]
    pub disk_type: ::prost::alloc::string::String,
    /// Shielded Instance options.
    #[prost(message, optional, tag = "8")]
    pub shielded_instance_config: ::core::option::Option<ShieldedInstanceConfig>,
    ///
    /// The Customer Managed Encryption Key used to encrypt the boot disk attached
    /// to each node in the node pool. This should be of the form
    /// projects/\[KEY_PROJECT_ID]/locations/[LOCATION]/keyRings/[RING_NAME]/cryptoKeys/[KEY_NAME\].
    /// For more information about protecting resources with Cloud KMS Keys please
    /// see:
    /// <https://cloud.google.com/compute/docs/disks/customer-managed-encryption>
    #[prost(string, tag = "9")]
    pub boot_disk_kms_key: ::prost::alloc::string::String,
    /// The image type to use for NAP created node.
    #[prost(string, tag = "10")]
    pub image_type: ::prost::alloc::string::String,
}
/// Contains information about amount of some resource in the cluster.
/// For memory, value should be in GB.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResourceLimit {
    /// Resource name "cpu", "memory" or gpu-specific string.
    #[prost(string, tag = "1")]
    pub resource_type: ::prost::alloc::string::String,
    /// Minimum amount of the resource in the cluster.
    #[prost(int64, tag = "2")]
    pub minimum: i64,
    /// Maximum amount of the resource in the cluster.
    #[prost(int64, tag = "3")]
    pub maximum: i64,
}
/// NodePoolAutoscaling contains information required by cluster autoscaler to
/// adjust the size of the node pool to the current cluster usage.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NodePoolAutoscaling {
    /// Is autoscaling enabled for this node pool.
    #[prost(bool, tag = "1")]
    pub enabled: bool,
    /// Minimum number of nodes for one location in the NodePool. Must be >= 1 and
    /// <= max_node_count.
    #[prost(int32, tag = "2")]
    pub min_node_count: i32,
    /// Maximum number of nodes for one location in the NodePool. Must be >=
    /// min_node_count. There has to be enough quota to scale up the cluster.
    #[prost(int32, tag = "3")]
    pub max_node_count: i32,
    /// Can this node pool be deleted automatically.
    #[prost(bool, tag = "4")]
    pub autoprovisioned: bool,
    /// Location policy used when scaling up a nodepool.
    #[prost(enumeration = "node_pool_autoscaling::LocationPolicy", tag = "5")]
    pub location_policy: i32,
    /// Minimum number of nodes in the node pool. Must be greater than 1 less than
    /// total_max_node_count.
    /// The total_*_node_count fields are mutually exclusive with the *_node_count
    /// fields.
    #[prost(int32, tag = "6")]
    pub total_min_node_count: i32,
    /// Maximum number of nodes in the node pool. Must be greater than
    /// total_min_node_count. There has to be enough quota to scale up the cluster.
    /// The total_*_node_count fields are mutually exclusive with the *_node_count
    /// fields.
    #[prost(int32, tag = "7")]
    pub total_max_node_count: i32,
}
/// Nested message and enum types in `NodePoolAutoscaling`.
pub mod node_pool_autoscaling {
    /// Location policy specifies how zones are picked when scaling up the
    /// nodepool.
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
    pub enum LocationPolicy {
        /// Not set.
        Unspecified = 0,
        /// BALANCED is a best effort policy that aims to balance the sizes of
        /// different zones.
        Balanced = 1,
        /// ANY policy picks zones that have the highest capacity available.
        Any = 2,
    }
    impl LocationPolicy {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                LocationPolicy::Unspecified => "LOCATION_POLICY_UNSPECIFIED",
                LocationPolicy::Balanced => "BALANCED",
                LocationPolicy::Any => "ANY",
            }
        }
    }
}
/// SetLabelsRequest sets the Google Cloud Platform labels on a Google Container
/// Engine cluster, which will in turn set them for Google Compute Engine
/// resources used by that cluster
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetLabelsRequest {
    /// Required. Deprecated. The Google Developers Console [project ID or project
    /// number](<https://cloud.google.com/resource-manager/docs/creating-managing-projects>).
    /// This field has been deprecated and replaced by the name field.
    #[deprecated]
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    /// Required. Deprecated. The name of the Google Compute Engine
    /// \[zone\](<https://cloud.google.com/compute/docs/zones#available>) in which the
    /// cluster resides. This field has been deprecated and replaced by the name
    /// field.
    #[deprecated]
    #[prost(string, tag = "2")]
    pub zone: ::prost::alloc::string::String,
    /// Required. Deprecated. The name of the cluster.
    /// This field has been deprecated and replaced by the name field.
    #[deprecated]
    #[prost(string, tag = "3")]
    pub cluster_id: ::prost::alloc::string::String,
    /// Required. The labels to set for that cluster.
    #[prost(map = "string, string", tag = "4")]
    pub resource_labels: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Required. The fingerprint of the previous set of labels for this resource,
    /// used to detect conflicts. The fingerprint is initially generated by
    /// Kubernetes Engine and changes after every request to modify or update
    /// labels. You must always provide an up-to-date fingerprint hash when
    /// updating or changing labels. Make a `get()` request to the
    /// resource to get the latest fingerprint.
    #[prost(string, tag = "5")]
    pub label_fingerprint: ::prost::alloc::string::String,
    /// The name (project, location, cluster name) of the cluster to set labels.
    /// Specified in the format `projects/*/locations/*/clusters/*`.
    #[prost(string, tag = "7")]
    pub name: ::prost::alloc::string::String,
}
/// SetLegacyAbacRequest enables or disables the ABAC authorization mechanism for
/// a cluster.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetLegacyAbacRequest {
    /// Required. Deprecated. The Google Developers Console [project ID or project
    /// number](<https://cloud.google.com/resource-manager/docs/creating-managing-projects>).
    /// This field has been deprecated and replaced by the name field.
    #[deprecated]
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    /// Required. Deprecated. The name of the Google Compute Engine
    /// \[zone\](<https://cloud.google.com/compute/docs/zones#available>) in which the
    /// cluster resides. This field has been deprecated and replaced by the name
    /// field.
    #[deprecated]
    #[prost(string, tag = "2")]
    pub zone: ::prost::alloc::string::String,
    /// Required. Deprecated. The name of the cluster to update.
    /// This field has been deprecated and replaced by the name field.
    #[deprecated]
    #[prost(string, tag = "3")]
    pub cluster_id: ::prost::alloc::string::String,
    /// Required. Whether ABAC authorization will be enabled in the cluster.
    #[prost(bool, tag = "4")]
    pub enabled: bool,
    /// The name (project, location, cluster name) of the cluster to set legacy
    /// abac. Specified in the format `projects/*/locations/*/clusters/*`.
    #[prost(string, tag = "6")]
    pub name: ::prost::alloc::string::String,
}
/// StartIPRotationRequest creates a new IP for the cluster and then performs
/// a node upgrade on each node pool to point to the new IP.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StartIpRotationRequest {
    /// Required. Deprecated. The Google Developers Console [project ID or project
    /// number](<https://cloud.google.com/resource-manager/docs/creating-managing-projects>).
    /// This field has been deprecated and replaced by the name field.
    #[deprecated]
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    /// Required. Deprecated. The name of the Google Compute Engine
    /// \[zone\](<https://cloud.google.com/compute/docs/zones#available>) in which the
    /// cluster resides. This field has been deprecated and replaced by the name
    /// field.
    #[deprecated]
    #[prost(string, tag = "2")]
    pub zone: ::prost::alloc::string::String,
    /// Required. Deprecated. The name of the cluster.
    /// This field has been deprecated and replaced by the name field.
    #[deprecated]
    #[prost(string, tag = "3")]
    pub cluster_id: ::prost::alloc::string::String,
    /// The name (project, location, cluster name) of the cluster to start IP
    /// rotation. Specified in the format `projects/*/locations/*/clusters/*`.
    #[prost(string, tag = "6")]
    pub name: ::prost::alloc::string::String,
    /// Whether to rotate credentials during IP rotation.
    #[prost(bool, tag = "7")]
    pub rotate_credentials: bool,
}
/// CompleteIPRotationRequest moves the cluster master back into single-IP mode.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CompleteIpRotationRequest {
    /// Required. Deprecated. The Google Developers Console [project ID or project
    /// number](<https://cloud.google.com/resource-manager/docs/creating-managing-projects>).
    /// This field has been deprecated and replaced by the name field.
    #[deprecated]
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    /// Required. Deprecated. The name of the Google Compute Engine
    /// \[zone\](<https://cloud.google.com/compute/docs/zones#available>) in which the
    /// cluster resides. This field has been deprecated and replaced by the name
    /// field.
    #[deprecated]
    #[prost(string, tag = "2")]
    pub zone: ::prost::alloc::string::String,
    /// Required. Deprecated. The name of the cluster.
    /// This field has been deprecated and replaced by the name field.
    #[deprecated]
    #[prost(string, tag = "3")]
    pub cluster_id: ::prost::alloc::string::String,
    /// The name (project, location, cluster name) of the cluster to complete IP
    /// rotation. Specified in the format `projects/*/locations/*/clusters/*`.
    #[prost(string, tag = "7")]
    pub name: ::prost::alloc::string::String,
}
/// AcceleratorConfig represents a Hardware Accelerator request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AcceleratorConfig {
    /// The number of the accelerator cards exposed to an instance.
    #[prost(int64, tag = "1")]
    pub accelerator_count: i64,
    /// The accelerator type resource name. List of supported accelerators
    /// \[here\](<https://cloud.google.com/compute/docs/gpus>)
    #[prost(string, tag = "2")]
    pub accelerator_type: ::prost::alloc::string::String,
    /// Size of partitions to create on the GPU. Valid values are described in the
    /// NVIDIA [mig user
    /// guide](<https://docs.nvidia.com/datacenter/tesla/mig-user-guide/#partitioning>).
    #[prost(string, tag = "3")]
    pub gpu_partition_size: ::prost::alloc::string::String,
    /// The number of time-shared GPU resources to expose for each physical GPU.
    #[deprecated]
    #[prost(int64, tag = "4")]
    pub max_time_shared_clients_per_gpu: i64,
    /// The configuration for GPU sharing options.
    #[prost(message, optional, tag = "5")]
    pub gpu_sharing_config: ::core::option::Option<GpuSharingConfig>,
}
/// GPUSharingConfig represents the GPU sharing configuration for Hardware
/// Accelerators.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GpuSharingConfig {
    /// The max number of containers that can share a physical GPU.
    #[prost(int64, tag = "1")]
    pub max_shared_clients_per_gpu: i64,
    /// The type of GPU sharing strategy to enable on the GPU node.
    #[prost(enumeration = "gpu_sharing_config::GpuSharingStrategy", optional, tag = "2")]
    pub gpu_sharing_strategy: ::core::option::Option<i32>,
}
/// Nested message and enum types in `GPUSharingConfig`.
pub mod gpu_sharing_config {
    /// The type of GPU sharing strategy currently provided.
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
    pub enum GpuSharingStrategy {
        /// Default value.
        Unspecified = 0,
        /// GPUs are time-shared between containers.
        TimeSharing = 1,
    }
    impl GpuSharingStrategy {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                GpuSharingStrategy::Unspecified => "GPU_SHARING_STRATEGY_UNSPECIFIED",
                GpuSharingStrategy::TimeSharing => "TIME_SHARING",
            }
        }
    }
}
/// ManagedPrometheusConfig defines the configuration for
/// Google Cloud Managed Service for Prometheus.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ManagedPrometheusConfig {
    /// Enable Managed Collection.
    #[prost(bool, tag = "1")]
    pub enabled: bool,
}
/// WorkloadMetadataConfig defines the metadata configuration to expose to
/// workloads on the node pool.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WorkloadMetadataConfig {
    /// NodeMetadata is the configuration for how to expose metadata to the
    /// workloads running on the node.
    #[deprecated]
    #[prost(enumeration = "workload_metadata_config::NodeMetadata", tag = "1")]
    pub node_metadata: i32,
    /// Mode is the configuration for how to expose metadata to workloads running
    /// on the node pool.
    #[prost(enumeration = "workload_metadata_config::Mode", tag = "2")]
    pub mode: i32,
}
/// Nested message and enum types in `WorkloadMetadataConfig`.
pub mod workload_metadata_config {
    /// NodeMetadata is the configuration for if and how to expose the node
    /// metadata to the workload running on the node.
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
    pub enum NodeMetadata {
        /// Not set.
        Unspecified = 0,
        /// Prevent workloads not in hostNetwork from accessing certain VM metadata,
        /// specifically kube-env, which contains Kubelet credentials, and the
        /// instance identity token.
        ///
        /// Metadata concealment is a temporary security solution available while the
        /// bootstrapping process for cluster nodes is being redesigned with
        /// significant security improvements.  This feature is scheduled to be
        /// deprecated in the future and later removed.
        Secure = 1,
        /// Expose all VM metadata to pods.
        Expose = 2,
        /// Run the GKE Metadata Server on this node. The GKE Metadata Server exposes
        /// a metadata API to workloads that is compatible with the V1 Compute
        /// Metadata APIs exposed by the Compute Engine and App Engine Metadata
        /// Servers. This feature can only be enabled if Workload Identity is enabled
        /// at the cluster level.
        GkeMetadataServer = 3,
    }
    impl NodeMetadata {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                NodeMetadata::Unspecified => "UNSPECIFIED",
                NodeMetadata::Secure => "SECURE",
                NodeMetadata::Expose => "EXPOSE",
                NodeMetadata::GkeMetadataServer => "GKE_METADATA_SERVER",
            }
        }
    }
    /// Mode is the configuration for how to expose metadata to workloads running
    /// on the node.
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
    pub enum Mode {
        /// Not set.
        Unspecified = 0,
        /// Expose all Compute Engine metadata to pods.
        GceMetadata = 1,
        /// Run the GKE Metadata Server on this node. The GKE Metadata Server exposes
        /// a metadata API to workloads that is compatible with the V1 Compute
        /// Metadata APIs exposed by the Compute Engine and App Engine Metadata
        /// Servers. This feature can only be enabled if Workload Identity is enabled
        /// at the cluster level.
        GkeMetadata = 2,
    }
    impl Mode {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Mode::Unspecified => "MODE_UNSPECIFIED",
                Mode::GceMetadata => "GCE_METADATA",
                Mode::GkeMetadata => "GKE_METADATA",
            }
        }
    }
}
/// SetNetworkPolicyRequest enables/disables network policy for a cluster.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetNetworkPolicyRequest {
    /// Required. Deprecated. The Google Developers Console [project ID or project
    /// number](<https://cloud.google.com/resource-manager/docs/creating-managing-projects>).
    /// This field has been deprecated and replaced by the name field.
    #[deprecated]
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    /// Required. Deprecated. The name of the Google Compute Engine
    /// \[zone\](<https://cloud.google.com/compute/docs/zones#available>) in which the
    /// cluster resides. This field has been deprecated and replaced by the name
    /// field.
    #[deprecated]
    #[prost(string, tag = "2")]
    pub zone: ::prost::alloc::string::String,
    /// Required. Deprecated. The name of the cluster.
    /// This field has been deprecated and replaced by the name field.
    #[deprecated]
    #[prost(string, tag = "3")]
    pub cluster_id: ::prost::alloc::string::String,
    /// Required. Configuration options for the NetworkPolicy feature.
    #[prost(message, optional, tag = "4")]
    pub network_policy: ::core::option::Option<NetworkPolicy>,
    /// The name (project, location, cluster name) of the cluster to set networking
    /// policy. Specified in the format `projects/*/locations/*/clusters/*`.
    #[prost(string, tag = "6")]
    pub name: ::prost::alloc::string::String,
}
/// SetMaintenancePolicyRequest sets the maintenance policy for a cluster.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetMaintenancePolicyRequest {
    /// Required. The Google Developers Console [project ID or project
    /// number](<https://cloud.google.com/resource-manager/docs/creating-managing-projects>).
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    /// Required. The name of the Google Compute Engine
    /// \[zone\](<https://cloud.google.com/compute/docs/zones#available>) in which the
    /// cluster resides.
    #[prost(string, tag = "2")]
    pub zone: ::prost::alloc::string::String,
    /// Required. The name of the cluster to update.
    #[prost(string, tag = "3")]
    pub cluster_id: ::prost::alloc::string::String,
    /// Required. The maintenance policy to be set for the cluster. An empty field
    /// clears the existing maintenance policy.
    #[prost(message, optional, tag = "4")]
    pub maintenance_policy: ::core::option::Option<MaintenancePolicy>,
    /// The name (project, location, cluster name) of the cluster to set
    /// maintenance policy.
    /// Specified in the format `projects/*/locations/*/clusters/*`.
    #[prost(string, tag = "5")]
    pub name: ::prost::alloc::string::String,
}
/// ListLocationsRequest is used to request the locations that offer GKE.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListLocationsRequest {
    /// Required. Contains the name of the resource requested.
    /// Specified in the format `projects/*`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
}
/// ListLocationsResponse returns the list of all GKE locations and their
/// recommendation state.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListLocationsResponse {
    /// A full list of GKE locations.
    #[prost(message, repeated, tag = "1")]
    pub locations: ::prost::alloc::vec::Vec<Location>,
    /// Only return ListLocationsResponse that occur after the page_token. This
    /// value should be populated from the ListLocationsResponse.next_page_token if
    /// that response token was set (which happens when listing more Locations than
    /// fit in a single ListLocationsResponse).
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Location returns the location name, and if the location is recommended
/// for GKE cluster scheduling.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Location {
    /// Contains the type of location this Location is for.
    /// Regional or Zonal.
    #[prost(enumeration = "location::LocationType", tag = "1")]
    pub r#type: i32,
    /// Contains the name of the resource requested.
    /// Specified in the format `projects/*/locations/*`.
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    /// Whether the location is recommended for GKE cluster scheduling.
    #[prost(bool, tag = "3")]
    pub recommended: bool,
}
/// Nested message and enum types in `Location`.
pub mod location {
    /// LocationType is the type of GKE location, regional or zonal.
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
    pub enum LocationType {
        /// LOCATION_TYPE_UNSPECIFIED means the location type was not determined.
        Unspecified = 0,
        /// A GKE Location where Zonal clusters can be created.
        Zone = 1,
        /// A GKE Location where Regional clusters can be created.
        Region = 2,
    }
    impl LocationType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                LocationType::Unspecified => "LOCATION_TYPE_UNSPECIFIED",
                LocationType::Zone => "ZONE",
                LocationType::Region => "REGION",
            }
        }
    }
}
/// StatusCondition describes why a cluster or a node pool has a certain status
/// (e.g., ERROR or DEGRADED).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StatusCondition {
    /// Machine-friendly representation of the condition
    /// Deprecated. Use canonical_code instead.
    #[deprecated]
    #[prost(enumeration = "status_condition::Code", tag = "1")]
    pub code: i32,
    /// Human-friendly representation of the condition
    #[prost(string, tag = "2")]
    pub message: ::prost::alloc::string::String,
    /// Canonical code of the condition.
    #[prost(enumeration = "super::super::rpc::Code", tag = "3")]
    pub canonical_code: i32,
}
/// Nested message and enum types in `StatusCondition`.
pub mod status_condition {
    /// Code for each condition
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
    pub enum Code {
        /// UNKNOWN indicates a generic condition.
        Unknown = 0,
        /// GCE_STOCKOUT indicates that Google Compute Engine resources are
        /// temporarily unavailable.
        GceStockout = 1,
        /// GKE_SERVICE_ACCOUNT_DELETED indicates that the user deleted their robot
        /// service account.
        GkeServiceAccountDeleted = 2,
        /// Google Compute Engine quota was exceeded.
        GceQuotaExceeded = 3,
        /// Cluster state was manually changed by an SRE due to a system logic error.
        SetByOperator = 4,
        /// Unable to perform an encrypt operation against the CloudKMS key used for
        /// etcd level encryption.
        CloudKmsKeyError = 7,
        /// Cluster CA is expiring soon.
        /// More codes TBA
        CaExpiring = 9,
    }
    impl Code {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Code::Unknown => "UNKNOWN",
                Code::GceStockout => "GCE_STOCKOUT",
                Code::GkeServiceAccountDeleted => "GKE_SERVICE_ACCOUNT_DELETED",
                Code::GceQuotaExceeded => "GCE_QUOTA_EXCEEDED",
                Code::SetByOperator => "SET_BY_OPERATOR",
                Code::CloudKmsKeyError => "CLOUD_KMS_KEY_ERROR",
                Code::CaExpiring => "CA_EXPIRING",
            }
        }
    }
}
/// NetworkConfig reports the relative names of network & subnetwork.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NetworkConfig {
    /// Output only. The relative name of the Google Compute Engine
    /// \[network][google.container.v1beta1.NetworkConfig.network\](<https://cloud.google.com/compute/docs/networks-and-firewalls#networks>)
    /// to which the cluster is connected. Example:
    /// projects/my-project/global/networks/my-network
    #[prost(string, tag = "1")]
    pub network: ::prost::alloc::string::String,
    /// Output only. The relative name of the Google Compute Engine
    /// \[subnetwork\](<https://cloud.google.com/compute/docs/vpc>) to which the
    /// cluster is connected. Example:
    /// projects/my-project/regions/us-central1/subnetworks/my-subnet
    #[prost(string, tag = "2")]
    pub subnetwork: ::prost::alloc::string::String,
    /// Whether Intra-node visibility is enabled for this cluster.
    /// This makes same node pod to pod traffic visible for VPC network.
    #[prost(bool, tag = "5")]
    pub enable_intra_node_visibility: bool,
    /// Whether the cluster disables default in-node sNAT rules. In-node sNAT rules
    /// will be disabled when default_snat_status is disabled. When disabled is set
    /// to false, default IP masquerade rules will be applied to the nodes to
    /// prevent sNAT on cluster internal traffic.
    #[prost(message, optional, tag = "7")]
    pub default_snat_status: ::core::option::Option<DefaultSnatStatus>,
    /// Whether L4ILB Subsetting is enabled for this cluster.
    #[prost(bool, tag = "10")]
    pub enable_l4ilb_subsetting: bool,
    /// The desired datapath provider for this cluster. By default, uses the
    /// IPTables-based kube-proxy implementation.
    #[prost(enumeration = "DatapathProvider", tag = "11")]
    pub datapath_provider: i32,
    /// The desired state of IPv6 connectivity to Google Services.
    /// By default, no private IPv6 access to or from Google Services (all access
    /// will be via IPv4)
    #[prost(enumeration = "PrivateIPv6GoogleAccess", tag = "12")]
    pub private_ipv6_google_access: i32,
    /// DNSConfig contains clusterDNS config for this cluster.
    #[prost(message, optional, tag = "13")]
    pub dns_config: ::core::option::Option<DnsConfig>,
    /// ServiceExternalIPsConfig specifies if services with externalIPs field are
    /// blocked or not.
    #[prost(message, optional, tag = "15")]
    pub service_external_ips_config: ::core::option::Option<ServiceExternalIPsConfig>,
    /// GatewayAPIConfig contains the desired config of Gateway API on this
    /// cluster.
    #[prost(message, optional, tag = "16")]
    pub gateway_api_config: ::core::option::Option<GatewayApiConfig>,
}
/// GatewayAPIConfig contains the desired config of Gateway API on this cluster.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GatewayApiConfig {
    /// The Gateway API release channel to use for Gateway API.
    #[prost(enumeration = "gateway_api_config::Channel", tag = "1")]
    pub channel: i32,
}
/// Nested message and enum types in `GatewayAPIConfig`.
pub mod gateway_api_config {
    /// Channel describes if/how Gateway API should be installed and implemented in
    /// a cluster.
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
    pub enum Channel {
        /// Default value.
        Unspecified = 0,
        /// Gateway API support is disabled
        Disabled = 1,
        /// Gateway API support is enabled, experimental CRDs are installed
        Experimental = 3,
        /// Gateway API support is enabled, standard CRDs are installed
        Standard = 4,
    }
    impl Channel {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Channel::Unspecified => "CHANNEL_UNSPECIFIED",
                Channel::Disabled => "CHANNEL_DISABLED",
                Channel::Experimental => "CHANNEL_EXPERIMENTAL",
                Channel::Standard => "CHANNEL_STANDARD",
            }
        }
    }
}
/// Config to block services with externalIPs field.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServiceExternalIPsConfig {
    /// Whether Services with ExternalIPs field are allowed or not.
    #[prost(bool, tag = "1")]
    pub enabled: bool,
}
/// ListUsableSubnetworksRequest requests the list of usable subnetworks.
/// available to a user for creating clusters.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListUsableSubnetworksRequest {
    /// Required. The parent project where subnetworks are usable.
    /// Specified in the format `projects/*`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Filtering currently only supports equality on the networkProjectId and must
    /// be in the form: "networkProjectId=\[PROJECTID\]", where `networkProjectId`
    /// is the project which owns the listed subnetworks. This defaults to the
    /// parent project ID.
    #[prost(string, tag = "2")]
    pub filter: ::prost::alloc::string::String,
    /// The max number of results per page that should be returned. If the number
    /// of available results is larger than `page_size`, a `next_page_token` is
    /// returned which can be used to get the next page of results in subsequent
    /// requests. Acceptable values are 0 to 500, inclusive. (Default: 500)
    #[prost(int32, tag = "3")]
    pub page_size: i32,
    /// Specifies a page token to use. Set this to the nextPageToken returned by
    /// previous list requests to get the next page of results.
    #[prost(string, tag = "4")]
    pub page_token: ::prost::alloc::string::String,
}
/// ListUsableSubnetworksResponse is the response of
/// ListUsableSubnetworksRequest.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListUsableSubnetworksResponse {
    /// A list of usable subnetworks in the specified network project.
    #[prost(message, repeated, tag = "1")]
    pub subnetworks: ::prost::alloc::vec::Vec<UsableSubnetwork>,
    /// This token allows you to get the next page of results for list requests.
    /// If the number of results is larger than `page_size`, use the
    /// `next_page_token` as a value for the query parameter `page_token` in the
    /// next request. The value will become empty when there are no more pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Secondary IP range of a usable subnetwork.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UsableSubnetworkSecondaryRange {
    /// The name associated with this subnetwork secondary range, used when adding
    /// an alias IP range to a VM instance.
    #[prost(string, tag = "1")]
    pub range_name: ::prost::alloc::string::String,
    /// The range of IP addresses belonging to this subnetwork secondary range.
    #[prost(string, tag = "2")]
    pub ip_cidr_range: ::prost::alloc::string::String,
    /// This field is to determine the status of the secondary range programmably.
    #[prost(enumeration = "usable_subnetwork_secondary_range::Status", tag = "3")]
    pub status: i32,
}
/// Nested message and enum types in `UsableSubnetworkSecondaryRange`.
pub mod usable_subnetwork_secondary_range {
    /// Status shows the current usage of a secondary IP range.
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
    pub enum Status {
        /// UNKNOWN is the zero value of the Status enum. It's not a valid status.
        Unknown = 0,
        /// UNUSED denotes that this range is unclaimed by any cluster.
        Unused = 1,
        /// IN_USE_SERVICE denotes that this range is claimed by a cluster for
        /// services. It cannot be used for other clusters.
        InUseService = 2,
        /// IN_USE_SHAREABLE_POD denotes this range was created by the network admin
        /// and is currently claimed by a cluster for pods. It can only be used by
        /// other clusters as a pod range.
        InUseShareablePod = 3,
        /// IN_USE_MANAGED_POD denotes this range was created by GKE and is claimed
        /// for pods. It cannot be used for other clusters.
        InUseManagedPod = 4,
    }
    impl Status {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Status::Unknown => "UNKNOWN",
                Status::Unused => "UNUSED",
                Status::InUseService => "IN_USE_SERVICE",
                Status::InUseShareablePod => "IN_USE_SHAREABLE_POD",
                Status::InUseManagedPod => "IN_USE_MANAGED_POD",
            }
        }
    }
}
/// UsableSubnetwork resource returns the subnetwork name, its associated network
/// and the primary CIDR range.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UsableSubnetwork {
    /// Subnetwork Name.
    /// Example: projects/my-project/regions/us-central1/subnetworks/my-subnet
    #[prost(string, tag = "1")]
    pub subnetwork: ::prost::alloc::string::String,
    /// Network Name.
    /// Example: projects/my-project/global/networks/my-network
    #[prost(string, tag = "2")]
    pub network: ::prost::alloc::string::String,
    /// The range of internal addresses that are owned by this subnetwork.
    #[prost(string, tag = "3")]
    pub ip_cidr_range: ::prost::alloc::string::String,
    /// Secondary IP ranges.
    #[prost(message, repeated, tag = "4")]
    pub secondary_ip_ranges: ::prost::alloc::vec::Vec<UsableSubnetworkSecondaryRange>,
    /// A human readable status message representing the reasons for cases where
    /// the caller cannot use the secondary ranges under the subnet. For example if
    /// the secondary_ip_ranges is empty due to a permission issue, an insufficient
    /// permission message will be given by status_message.
    #[prost(string, tag = "5")]
    pub status_message: ::prost::alloc::string::String,
}
/// VerticalPodAutoscaling contains global, per-cluster information
/// required by Vertical Pod Autoscaler to automatically adjust
/// the resources of pods controlled by it.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VerticalPodAutoscaling {
    /// Enables vertical pod autoscaling.
    #[prost(bool, tag = "1")]
    pub enabled: bool,
}
/// DefaultSnatStatus contains the desired state of whether default sNAT should
/// be disabled on the cluster.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DefaultSnatStatus {
    /// Disables cluster default sNAT rules.
    #[prost(bool, tag = "1")]
    pub disabled: bool,
}
/// IntraNodeVisibilityConfig contains the desired config of the intra-node
/// visibility on this cluster.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IntraNodeVisibilityConfig {
    /// Enables intra node visibility for this cluster.
    #[prost(bool, tag = "1")]
    pub enabled: bool,
}
/// ILBSubsettingConfig contains the desired config of L4 Internal LoadBalancer
/// subsetting on this cluster.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IlbSubsettingConfig {
    /// Enables l4 ILB subsetting for this cluster
    #[prost(bool, tag = "1")]
    pub enabled: bool,
}
/// DNSConfig contains the desired set of options for configuring clusterDNS.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DnsConfig {
    /// cluster_dns indicates which in-cluster DNS provider should be used.
    #[prost(enumeration = "dns_config::Provider", tag = "1")]
    pub cluster_dns: i32,
    /// cluster_dns_scope indicates the scope of access to cluster DNS records.
    #[prost(enumeration = "dns_config::DnsScope", tag = "2")]
    pub cluster_dns_scope: i32,
    /// cluster_dns_domain is the suffix used for all cluster service records.
    #[prost(string, tag = "3")]
    pub cluster_dns_domain: ::prost::alloc::string::String,
}
/// Nested message and enum types in `DNSConfig`.
pub mod dns_config {
    /// Provider lists the various in-cluster DNS providers.
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
    pub enum Provider {
        /// Default value
        Unspecified = 0,
        /// Use GKE default DNS provider(kube-dns) for DNS resolution.
        PlatformDefault = 1,
        /// Use CloudDNS for DNS resolution.
        CloudDns = 2,
    }
    impl Provider {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Provider::Unspecified => "PROVIDER_UNSPECIFIED",
                Provider::PlatformDefault => "PLATFORM_DEFAULT",
                Provider::CloudDns => "CLOUD_DNS",
            }
        }
    }
    /// DNSScope lists the various scopes of access to cluster DNS records.
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
    pub enum DnsScope {
        /// Default value, will be inferred as cluster scope.
        Unspecified = 0,
        /// DNS records are accessible from within the cluster.
        ClusterScope = 1,
        /// DNS records are accessible from within the VPC.
        VpcScope = 2,
    }
    impl DnsScope {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                DnsScope::Unspecified => "DNS_SCOPE_UNSPECIFIED",
                DnsScope::ClusterScope => "CLUSTER_SCOPE",
                DnsScope::VpcScope => "VPC_SCOPE",
            }
        }
    }
}
/// Constraints applied to pods.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MaxPodsConstraint {
    /// Constraint enforced on the max num of pods per node.
    #[prost(int64, tag = "1")]
    pub max_pods_per_node: i64,
}
/// Configuration for the use of Kubernetes Service Accounts in GCP IAM
/// policies.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WorkloadIdentityConfig {
    /// IAM Identity Namespace to attach all Kubernetes Service Accounts to.
    #[deprecated]
    #[prost(string, tag = "1")]
    pub identity_namespace: ::prost::alloc::string::String,
    /// The workload pool to attach all Kubernetes service accounts to.
    #[prost(string, tag = "2")]
    pub workload_pool: ::prost::alloc::string::String,
    /// identity provider is the third party identity provider.
    #[prost(string, tag = "3")]
    pub identity_provider: ::prost::alloc::string::String,
}
/// Configuration for direct-path (via ALTS) with workload identity.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WorkloadAltsConfig {
    /// enable_alts controls whether the alts handshaker should be enabled or not
    /// for direct-path.
    ///
    /// Requires Workload Identity
    /// (\[workload_pool][google.container.v1beta1.WorkloadIdentityConfig.workload_pool\]
    /// must be non-empty).
    #[prost(message, optional, tag = "1")]
    pub enable_alts: ::core::option::Option<bool>,
}
/// Configuration for issuance of mTLS keys and certificates to Kubernetes pods.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WorkloadCertificates {
    /// enable_certificates controls issuance of workload mTLS certificates.
    ///
    /// If set, the GKE Workload Identity Certificates controller and node agent
    /// will be deployed in the cluster, which can then be configured by creating a
    /// WorkloadCertificateConfig Custom Resource.
    ///
    /// Requires Workload Identity
    /// (\[workload_pool][google.container.v1beta1.WorkloadIdentityConfig.workload_pool\]
    /// must be non-empty).
    #[prost(message, optional, tag = "1")]
    pub enable_certificates: ::core::option::Option<bool>,
}
/// Configuration for issuance of mTLS keys and certificates to Kubernetes pods.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MeshCertificates {
    /// enable_certificates controls issuance of workload mTLS certificates.
    ///
    /// If set, the GKE Workload Identity Certificates controller and node agent
    /// will be deployed in the cluster, which can then be configured by creating a
    /// WorkloadCertificateConfig Custom Resource.
    ///
    /// Requires Workload Identity
    /// (\[workload_pool][google.container.v1alpha1.WorkloadIdentityConfig.workload_pool\]
    /// must be non-empty).
    #[prost(message, optional, tag = "1")]
    pub enable_certificates: ::core::option::Option<bool>,
}
/// Configuration of etcd encryption.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DatabaseEncryption {
    /// Denotes the state of etcd encryption.
    #[prost(enumeration = "database_encryption::State", tag = "2")]
    pub state: i32,
    /// Name of CloudKMS key to use for the encryption of secrets in etcd.
    /// Ex. projects/my-project/locations/global/keyRings/my-ring/cryptoKeys/my-key
    #[prost(string, tag = "1")]
    pub key_name: ::prost::alloc::string::String,
}
/// Nested message and enum types in `DatabaseEncryption`.
pub mod database_encryption {
    /// State of etcd encryption.
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
        /// Should never be set
        Unknown = 0,
        /// Secrets in etcd are encrypted.
        Encrypted = 1,
        /// Secrets in etcd are stored in plain text (at etcd level) - this is
        /// unrelated to Compute Engine level full disk encryption.
        Decrypted = 2,
    }
    impl State {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                State::Unknown => "UNKNOWN",
                State::Encrypted => "ENCRYPTED",
                State::Decrypted => "DECRYPTED",
            }
        }
    }
}
/// Configuration for exporting cluster resource usages.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResourceUsageExportConfig {
    /// Configuration to use BigQuery as usage export destination.
    #[prost(message, optional, tag = "1")]
    pub bigquery_destination: ::core::option::Option<
        resource_usage_export_config::BigQueryDestination,
    >,
    /// Whether to enable network egress metering for this cluster. If enabled, a
    /// daemonset will be created in the cluster to meter network egress traffic.
    #[prost(bool, tag = "2")]
    pub enable_network_egress_metering: bool,
    /// Configuration to enable resource consumption metering.
    #[prost(message, optional, tag = "3")]
    pub consumption_metering_config: ::core::option::Option<
        resource_usage_export_config::ConsumptionMeteringConfig,
    >,
}
/// Nested message and enum types in `ResourceUsageExportConfig`.
pub mod resource_usage_export_config {
    /// Parameters for using BigQuery as the destination of resource usage export.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct BigQueryDestination {
        /// The ID of a BigQuery Dataset.
        #[prost(string, tag = "1")]
        pub dataset_id: ::prost::alloc::string::String,
    }
    /// Parameters for controlling consumption metering.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ConsumptionMeteringConfig {
        /// Whether to enable consumption metering for this cluster. If enabled, a
        /// second BigQuery table will be created to hold resource consumption
        /// records.
        #[prost(bool, tag = "1")]
        pub enabled: bool,
    }
}
/// Configuration of Shielded Nodes feature.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShieldedNodes {
    /// Whether Shielded Nodes features are enabled on all nodes in this cluster.
    #[prost(bool, tag = "1")]
    pub enabled: bool,
}
/// Configuration of gVNIC feature.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VirtualNic {
    /// Whether gVNIC features are enabled in the node pool.
    #[prost(bool, tag = "1")]
    pub enabled: bool,
}
/// Configuration of Fast Socket feature.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FastSocket {
    /// Whether Fast Socket features are enabled in the node pool.
    #[prost(bool, tag = "1")]
    pub enabled: bool,
}
/// GetOpenIDConfigRequest gets the OIDC discovery document for the
/// cluster. See the OpenID Connect Discovery 1.0 specification for details.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetOpenIdConfigRequest {
    /// The cluster (project, location, cluster name) to get the discovery document
    /// for. Specified in the format `projects/*/locations/*/clusters/*`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
}
/// GetOpenIDConfigResponse is an OIDC discovery document for the cluster.
/// See the OpenID Connect Discovery 1.0 specification for details.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetOpenIdConfigResponse {
    /// OIDC Issuer.
    #[prost(string, tag = "1")]
    pub issuer: ::prost::alloc::string::String,
    /// JSON Web Key uri.
    #[prost(string, tag = "2")]
    pub jwks_uri: ::prost::alloc::string::String,
    /// Supported response types.
    #[prost(string, repeated, tag = "3")]
    pub response_types_supported: ::prost::alloc::vec::Vec<
        ::prost::alloc::string::String,
    >,
    /// Supported subject types.
    #[prost(string, repeated, tag = "4")]
    pub subject_types_supported: ::prost::alloc::vec::Vec<
        ::prost::alloc::string::String,
    >,
    /// supported ID Token signing Algorithms.
    #[prost(string, repeated, tag = "5")]
    pub id_token_signing_alg_values_supported: ::prost::alloc::vec::Vec<
        ::prost::alloc::string::String,
    >,
    /// Supported claims.
    #[prost(string, repeated, tag = "6")]
    pub claims_supported: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Supported grant types.
    #[prost(string, repeated, tag = "7")]
    pub grant_types: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// GetJSONWebKeysRequest gets the public component of the keys used by the
/// cluster to sign token requests. This will be the jwks_uri for the discover
/// document returned by getOpenIDConfig. See the OpenID Connect
/// Discovery 1.0 specification for details.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetJsonWebKeysRequest {
    /// The cluster (project, location, cluster name) to get keys for. Specified in
    /// the format `projects/*/locations/*/clusters/*`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
}
/// Jwk is a JSON Web Key as specified in RFC 7517
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Jwk {
    /// Key Type.
    #[prost(string, tag = "1")]
    pub kty: ::prost::alloc::string::String,
    /// Algorithm.
    #[prost(string, tag = "2")]
    pub alg: ::prost::alloc::string::String,
    /// Permitted uses for the public keys.
    #[prost(string, tag = "3")]
    pub r#use: ::prost::alloc::string::String,
    /// Key ID.
    #[prost(string, tag = "4")]
    pub kid: ::prost::alloc::string::String,
    /// Used for RSA keys.
    #[prost(string, tag = "5")]
    pub n: ::prost::alloc::string::String,
    /// Used for RSA keys.
    #[prost(string, tag = "6")]
    pub e: ::prost::alloc::string::String,
    /// Used for ECDSA keys.
    #[prost(string, tag = "7")]
    pub x: ::prost::alloc::string::String,
    /// Used for ECDSA keys.
    #[prost(string, tag = "8")]
    pub y: ::prost::alloc::string::String,
    /// Used for ECDSA keys.
    #[prost(string, tag = "9")]
    pub crv: ::prost::alloc::string::String,
}
/// GetJSONWebKeysResponse is a valid JSON Web Key Set as specififed in rfc 7517
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetJsonWebKeysResponse {
    /// The public component of the keys used by the cluster to sign token
    /// requests.
    #[prost(message, repeated, tag = "1")]
    pub keys: ::prost::alloc::vec::Vec<Jwk>,
}
/// ReleaseChannel indicates which release channel a cluster is
/// subscribed to. Release channels are arranged in order of risk.
///
/// When a cluster is subscribed to a release channel, Google maintains
/// both the master version and the node version. Node auto-upgrade
/// defaults to true and cannot be disabled.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReleaseChannel {
    /// channel specifies which release channel the cluster is subscribed to.
    #[prost(enumeration = "release_channel::Channel", tag = "1")]
    pub channel: i32,
}
/// Nested message and enum types in `ReleaseChannel`.
pub mod release_channel {
    /// Possible values for 'channel'.
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
    pub enum Channel {
        /// No channel specified.
        Unspecified = 0,
        /// RAPID channel is offered on an early access basis for customers who want
        /// to test new releases.
        ///
        /// WARNING: Versions available in the RAPID Channel may be subject to
        /// unresolved issues with no known workaround and are not subject to any
        /// SLAs.
        Rapid = 1,
        /// Clusters subscribed to REGULAR receive versions that are considered GA
        /// quality. REGULAR is intended for production users who want to take
        /// advantage of new features.
        Regular = 2,
        /// Clusters subscribed to STABLE receive versions that are known to be
        /// stable and reliable in production.
        Stable = 3,
    }
    impl Channel {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Channel::Unspecified => "UNSPECIFIED",
                Channel::Rapid => "RAPID",
                Channel::Regular => "REGULAR",
                Channel::Stable => "STABLE",
            }
        }
    }
}
/// Configuration for fine-grained cost management feature.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CostManagementConfig {
    /// Whether the feature is enabled or not.
    #[prost(bool, tag = "1")]
    pub enabled: bool,
}
/// Configuration for Cloud TPU.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TpuConfig {
    /// Whether Cloud TPU integration is enabled or not.
    #[prost(bool, tag = "1")]
    pub enabled: bool,
    /// Whether to use service networking for Cloud TPU or not.
    #[prost(bool, tag = "2")]
    pub use_service_networking: bool,
    /// IPv4 CIDR block reserved for Cloud TPU in the VPC.
    #[prost(string, tag = "3")]
    pub ipv4_cidr_block: ::prost::alloc::string::String,
}
/// Master is the configuration for components on master.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Master {}
/// Autopilot is the configuration for Autopilot settings on the cluster.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Autopilot {
    /// Enable Autopilot
    #[prost(bool, tag = "1")]
    pub enabled: bool,
}
/// NotificationConfig is the configuration of notifications.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NotificationConfig {
    /// Notification config for Pub/Sub.
    #[prost(message, optional, tag = "1")]
    pub pubsub: ::core::option::Option<notification_config::PubSub>,
}
/// Nested message and enum types in `NotificationConfig`.
pub mod notification_config {
    /// Pub/Sub specific notification config.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PubSub {
        /// Enable notifications for Pub/Sub.
        #[prost(bool, tag = "1")]
        pub enabled: bool,
        /// The desired Pub/Sub topic to which notifications will be
        /// sent by GKE. Format is `projects/{project}/topics/{topic}`.
        #[prost(string, tag = "2")]
        pub topic: ::prost::alloc::string::String,
        /// Allows filtering to one or more specific event types. If no filter is
        /// specified, or if a filter is specified with no event types, all event
        /// types will be sent
        #[prost(message, optional, tag = "3")]
        pub filter: ::core::option::Option<Filter>,
    }
    /// Allows filtering to one or more specific event types. If event types are
    /// present, those and only those event types will be transmitted to the
    /// cluster. Other types will be skipped. If no filter is specified, or no
    /// event types are present, all event types will be sent
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Filter {
        /// Event types to allowlist.
        #[prost(enumeration = "EventType", repeated, tag = "1")]
        pub event_type: ::prost::alloc::vec::Vec<i32>,
    }
    /// Types of notifications currently supported. Can be used to filter what
    /// notifications are sent.
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
    pub enum EventType {
        /// Not set, will be ignored.
        Unspecified = 0,
        /// Corresponds with UpgradeAvailableEvent.
        UpgradeAvailableEvent = 1,
        /// Corresponds with UpgradeEvent.
        UpgradeEvent = 2,
        /// Corresponds with SecurityBulletinEvent.
        SecurityBulletinEvent = 3,
    }
    impl EventType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                EventType::Unspecified => "EVENT_TYPE_UNSPECIFIED",
                EventType::UpgradeAvailableEvent => "UPGRADE_AVAILABLE_EVENT",
                EventType::UpgradeEvent => "UPGRADE_EVENT",
                EventType::SecurityBulletinEvent => "SECURITY_BULLETIN_EVENT",
            }
        }
    }
}
/// ConfidentialNodes is configuration for the confidential nodes feature, which
/// makes nodes run on confidential VMs.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConfidentialNodes {
    /// Whether Confidential Nodes feature is enabled.
    #[prost(bool, tag = "1")]
    pub enabled: bool,
}
/// UpgradeEvent is a notification sent to customers by the cluster server when
/// a resource is upgrading.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpgradeEvent {
    /// The resource type that is upgrading.
    #[prost(enumeration = "UpgradeResourceType", tag = "1")]
    pub resource_type: i32,
    /// The operation associated with this upgrade.
    #[prost(string, tag = "2")]
    pub operation: ::prost::alloc::string::String,
    /// The time when the operation was started.
    #[prost(message, optional, tag = "3")]
    pub operation_start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The current version before the upgrade.
    #[prost(string, tag = "4")]
    pub current_version: ::prost::alloc::string::String,
    /// The target version for the upgrade.
    #[prost(string, tag = "5")]
    pub target_version: ::prost::alloc::string::String,
    /// Optional relative path to the resource. For example in node pool upgrades,
    /// the relative path of the node pool.
    #[prost(string, tag = "6")]
    pub resource: ::prost::alloc::string::String,
}
/// UpgradeAvailableEvent is a notification sent to customers when a new
/// available version is released.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpgradeAvailableEvent {
    /// The release version available for upgrade.
    #[prost(string, tag = "1")]
    pub version: ::prost::alloc::string::String,
    /// The resource type of the release version.
    #[prost(enumeration = "UpgradeResourceType", tag = "2")]
    pub resource_type: i32,
    /// The release channel of the version. If empty, it means a non-channel
    /// release.
    #[prost(message, optional, tag = "3")]
    pub release_channel: ::core::option::Option<ReleaseChannel>,
    /// Optional relative path to the resource. For example, the relative path of
    /// the node pool.
    #[prost(string, tag = "4")]
    pub resource: ::prost::alloc::string::String,
    /// Windows node versions info.
    #[prost(message, optional, tag = "5")]
    pub windows_versions: ::core::option::Option<WindowsVersions>,
}
/// SecurityBulletinEvent is a notification sent to customers when a security
/// bulletin has been posted that they are vulnerable to.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SecurityBulletinEvent {
    /// The resource type (node/control plane) that has the vulnerability. Multiple
    /// notifications (1 notification per resource type) will be sent for a
    /// vulnerability that affects > 1 resource type.
    #[prost(string, tag = "1")]
    pub resource_type_affected: ::prost::alloc::string::String,
    /// The ID of the bulletin corresponding to the vulnerability.
    #[prost(string, tag = "2")]
    pub bulletin_id: ::prost::alloc::string::String,
    /// The CVEs associated with this bulletin.
    #[prost(string, repeated, tag = "3")]
    pub cve_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The severity of this bulletin as it relates to GKE.
    #[prost(string, tag = "4")]
    pub severity: ::prost::alloc::string::String,
    /// The URI link to the bulletin on the website for more information.
    #[prost(string, tag = "5")]
    pub bulletin_uri: ::prost::alloc::string::String,
    /// A brief description of the bulletin. See the bulletin pointed to by the
    /// bulletin_uri field for an expanded description.
    #[prost(string, tag = "6")]
    pub brief_description: ::prost::alloc::string::String,
    /// The GKE minor versions affected by this vulnerability.
    #[prost(string, repeated, tag = "7")]
    pub affected_supported_minors: ::prost::alloc::vec::Vec<
        ::prost::alloc::string::String,
    >,
    /// The GKE versions where this vulnerability is patched.
    #[prost(string, repeated, tag = "8")]
    pub patched_versions: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// This represents a version selected from the patched_versions field that
    /// the cluster receiving this notification should most likely want to upgrade
    /// to based on its current version. Note that if this notification is being
    /// received by a given cluster, it means that this version is currently
    /// available as an upgrade target in that cluster's location.
    #[prost(string, tag = "9")]
    pub suggested_upgrade_target: ::prost::alloc::string::String,
    /// If this field is specified, it means there are manual steps that the user
    /// must take to make their clusters safe.
    #[prost(bool, tag = "10")]
    pub manual_steps_required: bool,
}
/// IdentityServiceConfig is configuration for Identity Service which allows
/// customers to use external identity providers with the K8S API
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IdentityServiceConfig {
    /// Whether to enable the Identity Service component
    #[prost(bool, tag = "1")]
    pub enabled: bool,
}
/// LoggingConfig is cluster logging configuration.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LoggingConfig {
    /// Logging components configuration
    #[prost(message, optional, tag = "1")]
    pub component_config: ::core::option::Option<LoggingComponentConfig>,
}
/// LoggingComponentConfig is cluster logging component configuration.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LoggingComponentConfig {
    /// Select components to collect logs. An empty set would disable all logging.
    #[prost(enumeration = "logging_component_config::Component", repeated, tag = "1")]
    pub enable_components: ::prost::alloc::vec::Vec<i32>,
}
/// Nested message and enum types in `LoggingComponentConfig`.
pub mod logging_component_config {
    /// GKE components exposing logs
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
    pub enum Component {
        /// Default value. This shouldn't be used.
        Unspecified = 0,
        /// system components
        SystemComponents = 1,
        /// workloads
        Workloads = 2,
        /// kube-apiserver
        Apiserver = 3,
        /// kube-scheduler
        Scheduler = 4,
        /// kube-controller-manager
        ControllerManager = 5,
    }
    impl Component {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Component::Unspecified => "COMPONENT_UNSPECIFIED",
                Component::SystemComponents => "SYSTEM_COMPONENTS",
                Component::Workloads => "WORKLOADS",
                Component::Apiserver => "APISERVER",
                Component::Scheduler => "SCHEDULER",
                Component::ControllerManager => "CONTROLLER_MANAGER",
            }
        }
    }
}
/// MonitoringConfig is cluster monitoring configuration.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MonitoringConfig {
    /// Monitoring components configuration
    #[prost(message, optional, tag = "1")]
    pub component_config: ::core::option::Option<MonitoringComponentConfig>,
    /// Enable Google Cloud Managed Service for Prometheus
    /// in the cluster.
    #[prost(message, optional, tag = "2")]
    pub managed_prometheus_config: ::core::option::Option<ManagedPrometheusConfig>,
}
/// NodePoolLoggingConfig specifies logging configuration for nodepools.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NodePoolLoggingConfig {
    /// Logging variant configuration.
    #[prost(message, optional, tag = "1")]
    pub variant_config: ::core::option::Option<LoggingVariantConfig>,
}
/// LoggingVariantConfig specifies the behaviour of the logging component.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LoggingVariantConfig {
    /// Logging variant deployed on nodes.
    #[prost(enumeration = "logging_variant_config::Variant", tag = "1")]
    pub variant: i32,
}
/// Nested message and enum types in `LoggingVariantConfig`.
pub mod logging_variant_config {
    /// Logging component variants.
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
    pub enum Variant {
        /// Default value. This shouldn't be used.
        Unspecified = 0,
        /// default logging variant.
        Default = 1,
        /// maximum logging throughput variant.
        MaxThroughput = 2,
    }
    impl Variant {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Variant::Unspecified => "VARIANT_UNSPECIFIED",
                Variant::Default => "DEFAULT",
                Variant::MaxThroughput => "MAX_THROUGHPUT",
            }
        }
    }
}
/// MonitoringComponentConfig is cluster monitoring component configuration.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MonitoringComponentConfig {
    /// Select components to collect metrics. An empty set would disable all
    /// monitoring.
    #[prost(enumeration = "monitoring_component_config::Component", repeated, tag = "1")]
    pub enable_components: ::prost::alloc::vec::Vec<i32>,
}
/// Nested message and enum types in `MonitoringComponentConfig`.
pub mod monitoring_component_config {
    /// GKE components exposing metrics
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
    pub enum Component {
        /// Default value. This shouldn't be used.
        Unspecified = 0,
        /// system components
        SystemComponents = 1,
        /// Deprecated: Use Google Cloud Managed Service for Prometheus.
        Workloads = 2,
        /// kube-apiserver
        Apiserver = 3,
        /// kube-scheduler
        Scheduler = 4,
        /// kube-controller-manager
        ControllerManager = 5,
    }
    impl Component {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Component::Unspecified => "COMPONENT_UNSPECIFIED",
                Component::SystemComponents => "SYSTEM_COMPONENTS",
                Component::Workloads => "WORKLOADS",
                Component::Apiserver => "APISERVER",
                Component::Scheduler => "SCHEDULER",
                Component::ControllerManager => "CONTROLLER_MANAGER",
            }
        }
    }
}
/// PrivateIPv6GoogleAccess controls whether and how the pods can communicate
/// with Google Services through gRPC over IPv6.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PrivateIPv6GoogleAccess {
    /// Default value. Same as DISABLED
    PrivateIpv6GoogleAccessUnspecified = 0,
    /// No private access to or from Google Services
    PrivateIpv6GoogleAccessDisabled = 1,
    /// Enables private IPv6 access to Google Services from GKE
    PrivateIpv6GoogleAccessToGoogle = 2,
    /// Enables priate IPv6 access to and from Google Services
    PrivateIpv6GoogleAccessBidirectional = 3,
}
impl PrivateIPv6GoogleAccess {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PrivateIPv6GoogleAccess::PrivateIpv6GoogleAccessUnspecified => {
                "PRIVATE_IPV6_GOOGLE_ACCESS_UNSPECIFIED"
            }
            PrivateIPv6GoogleAccess::PrivateIpv6GoogleAccessDisabled => {
                "PRIVATE_IPV6_GOOGLE_ACCESS_DISABLED"
            }
            PrivateIPv6GoogleAccess::PrivateIpv6GoogleAccessToGoogle => {
                "PRIVATE_IPV6_GOOGLE_ACCESS_TO_GOOGLE"
            }
            PrivateIPv6GoogleAccess::PrivateIpv6GoogleAccessBidirectional => {
                "PRIVATE_IPV6_GOOGLE_ACCESS_BIDIRECTIONAL"
            }
        }
    }
}
/// UpgradeResourceType is the resource type that is upgrading. It is used
/// in upgrade notifications.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum UpgradeResourceType {
    /// Default value. This shouldn't be used.
    Unspecified = 0,
    /// Master / control plane
    Master = 1,
    /// Node pool
    NodePool = 2,
}
impl UpgradeResourceType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            UpgradeResourceType::Unspecified => "UPGRADE_RESOURCE_TYPE_UNSPECIFIED",
            UpgradeResourceType::Master => "MASTER",
            UpgradeResourceType::NodePool => "NODE_POOL",
        }
    }
}
/// Strategy used for node pool update.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum NodePoolUpdateStrategy {
    /// Default value.
    Unspecified = 0,
    /// blue-green upgrade.
    BlueGreen = 2,
    /// SURGE is the traditional way of upgrading a node pool.
    /// max_surge and max_unavailable determines the level of upgrade parallelism.
    Surge = 3,
}
impl NodePoolUpdateStrategy {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            NodePoolUpdateStrategy::Unspecified => {
                "NODE_POOL_UPDATE_STRATEGY_UNSPECIFIED"
            }
            NodePoolUpdateStrategy::BlueGreen => "BLUE_GREEN",
            NodePoolUpdateStrategy::Surge => "SURGE",
        }
    }
}
/// The datapath provider selects the implementation of the Kubernetes networking
/// model for service resolution and network policy enforcement.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DatapathProvider {
    /// Default value.
    Unspecified = 0,
    /// Use the IPTables implementation based on kube-proxy.
    LegacyDatapath = 1,
    /// Use the eBPF based GKE Dataplane V2 with additional features. See the [GKE
    /// Dataplane V2
    /// documentation](<https://cloud.google.com/kubernetes-engine/docs/how-to/dataplane-v2>)
    /// for more.
    AdvancedDatapath = 2,
}
impl DatapathProvider {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            DatapathProvider::Unspecified => "DATAPATH_PROVIDER_UNSPECIFIED",
            DatapathProvider::LegacyDatapath => "LEGACY_DATAPATH",
            DatapathProvider::AdvancedDatapath => "ADVANCED_DATAPATH",
        }
    }
}
/// Generated client implementations.
pub mod cluster_manager_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Google Kubernetes Engine Cluster Manager v1beta1
    #[derive(Debug, Clone)]
    pub struct ClusterManagerClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ClusterManagerClient<tonic::transport::Channel> {
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
    impl<T> ClusterManagerClient<T>
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
        ) -> ClusterManagerClient<InterceptedService<T, F>>
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
            ClusterManagerClient::new(InterceptedService::new(inner, interceptor))
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
        /// Lists all clusters owned by a project in either the specified zone or all
        /// zones.
        pub async fn list_clusters(
            &mut self,
            request: impl tonic::IntoRequest<super::ListClustersRequest>,
        ) -> Result<tonic::Response<super::ListClustersResponse>, tonic::Status> {
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
                "/google.container.v1beta1.ClusterManager/ListClusters",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets the details for a specific cluster.
        pub async fn get_cluster(
            &mut self,
            request: impl tonic::IntoRequest<super::GetClusterRequest>,
        ) -> Result<tonic::Response<super::Cluster>, tonic::Status> {
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
                "/google.container.v1beta1.ClusterManager/GetCluster",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates a cluster, consisting of the specified number and type of Google
        /// Compute Engine instances.
        ///
        /// By default, the cluster is created in the project's
        /// [default
        /// network](https://cloud.google.com/compute/docs/networks-and-firewalls#networks).
        ///
        /// One firewall is added for the cluster. After cluster creation,
        /// the Kubelet creates routes for each node to allow the containers
        /// on that node to communicate with all other instances in the
        /// cluster.
        ///
        /// Finally, an entry is added to the project's global metadata indicating
        /// which CIDR range the cluster is using.
        pub async fn create_cluster(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateClusterRequest>,
        ) -> Result<tonic::Response<super::Operation>, tonic::Status> {
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
                "/google.container.v1beta1.ClusterManager/CreateCluster",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates the settings for a specific cluster.
        pub async fn update_cluster(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateClusterRequest>,
        ) -> Result<tonic::Response<super::Operation>, tonic::Status> {
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
                "/google.container.v1beta1.ClusterManager/UpdateCluster",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates the version and/or image type of a specific node pool.
        pub async fn update_node_pool(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateNodePoolRequest>,
        ) -> Result<tonic::Response<super::Operation>, tonic::Status> {
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
                "/google.container.v1beta1.ClusterManager/UpdateNodePool",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Sets the autoscaling settings of a specific node pool.
        pub async fn set_node_pool_autoscaling(
            &mut self,
            request: impl tonic::IntoRequest<super::SetNodePoolAutoscalingRequest>,
        ) -> Result<tonic::Response<super::Operation>, tonic::Status> {
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
                "/google.container.v1beta1.ClusterManager/SetNodePoolAutoscaling",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Sets the logging service for a specific cluster.
        pub async fn set_logging_service(
            &mut self,
            request: impl tonic::IntoRequest<super::SetLoggingServiceRequest>,
        ) -> Result<tonic::Response<super::Operation>, tonic::Status> {
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
                "/google.container.v1beta1.ClusterManager/SetLoggingService",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Sets the monitoring service for a specific cluster.
        pub async fn set_monitoring_service(
            &mut self,
            request: impl tonic::IntoRequest<super::SetMonitoringServiceRequest>,
        ) -> Result<tonic::Response<super::Operation>, tonic::Status> {
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
                "/google.container.v1beta1.ClusterManager/SetMonitoringService",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Sets the addons for a specific cluster.
        pub async fn set_addons_config(
            &mut self,
            request: impl tonic::IntoRequest<super::SetAddonsConfigRequest>,
        ) -> Result<tonic::Response<super::Operation>, tonic::Status> {
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
                "/google.container.v1beta1.ClusterManager/SetAddonsConfig",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Sets the locations for a specific cluster.
        /// Deprecated. Use
        /// [projects.locations.clusters.update](https://cloud.google.com/kubernetes-engine/docs/reference/rest/v1beta1/projects.locations.clusters/update)
        /// instead.
        pub async fn set_locations(
            &mut self,
            request: impl tonic::IntoRequest<super::SetLocationsRequest>,
        ) -> Result<tonic::Response<super::Operation>, tonic::Status> {
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
                "/google.container.v1beta1.ClusterManager/SetLocations",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates the master for a specific cluster.
        pub async fn update_master(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateMasterRequest>,
        ) -> Result<tonic::Response<super::Operation>, tonic::Status> {
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
                "/google.container.v1beta1.ClusterManager/UpdateMaster",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Sets master auth materials. Currently supports changing the admin password
        /// or a specific cluster, either via password generation or explicitly setting
        /// the password.
        pub async fn set_master_auth(
            &mut self,
            request: impl tonic::IntoRequest<super::SetMasterAuthRequest>,
        ) -> Result<tonic::Response<super::Operation>, tonic::Status> {
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
                "/google.container.v1beta1.ClusterManager/SetMasterAuth",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes the cluster, including the Kubernetes endpoint and all worker
        /// nodes.
        ///
        /// Firewalls and routes that were configured during cluster creation
        /// are also deleted.
        ///
        /// Other Google Compute Engine resources that might be in use by the cluster,
        /// such as load balancer resources, are not deleted if they weren't present
        /// when the cluster was initially created.
        pub async fn delete_cluster(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteClusterRequest>,
        ) -> Result<tonic::Response<super::Operation>, tonic::Status> {
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
                "/google.container.v1beta1.ClusterManager/DeleteCluster",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists all operations in a project in the specified zone or all zones.
        pub async fn list_operations(
            &mut self,
            request: impl tonic::IntoRequest<super::ListOperationsRequest>,
        ) -> Result<tonic::Response<super::ListOperationsResponse>, tonic::Status> {
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
                "/google.container.v1beta1.ClusterManager/ListOperations",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets the specified operation.
        pub async fn get_operation(
            &mut self,
            request: impl tonic::IntoRequest<super::GetOperationRequest>,
        ) -> Result<tonic::Response<super::Operation>, tonic::Status> {
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
                "/google.container.v1beta1.ClusterManager/GetOperation",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Cancels the specified operation.
        pub async fn cancel_operation(
            &mut self,
            request: impl tonic::IntoRequest<super::CancelOperationRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
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
                "/google.container.v1beta1.ClusterManager/CancelOperation",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Returns configuration info about the Google Kubernetes Engine service.
        pub async fn get_server_config(
            &mut self,
            request: impl tonic::IntoRequest<super::GetServerConfigRequest>,
        ) -> Result<tonic::Response<super::ServerConfig>, tonic::Status> {
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
                "/google.container.v1beta1.ClusterManager/GetServerConfig",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists the node pools for a cluster.
        pub async fn list_node_pools(
            &mut self,
            request: impl tonic::IntoRequest<super::ListNodePoolsRequest>,
        ) -> Result<tonic::Response<super::ListNodePoolsResponse>, tonic::Status> {
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
                "/google.container.v1beta1.ClusterManager/ListNodePools",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets the public component of the cluster signing keys in
        /// JSON Web Key format.
        /// This API is not yet intended for general use, and is not available for all
        /// clusters.
        pub async fn get_json_web_keys(
            &mut self,
            request: impl tonic::IntoRequest<super::GetJsonWebKeysRequest>,
        ) -> Result<tonic::Response<super::GetJsonWebKeysResponse>, tonic::Status> {
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
                "/google.container.v1beta1.ClusterManager/GetJSONWebKeys",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Retrieves the requested node pool.
        pub async fn get_node_pool(
            &mut self,
            request: impl tonic::IntoRequest<super::GetNodePoolRequest>,
        ) -> Result<tonic::Response<super::NodePool>, tonic::Status> {
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
                "/google.container.v1beta1.ClusterManager/GetNodePool",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates a node pool for a cluster.
        pub async fn create_node_pool(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateNodePoolRequest>,
        ) -> Result<tonic::Response<super::Operation>, tonic::Status> {
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
                "/google.container.v1beta1.ClusterManager/CreateNodePool",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes a node pool from a cluster.
        pub async fn delete_node_pool(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteNodePoolRequest>,
        ) -> Result<tonic::Response<super::Operation>, tonic::Status> {
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
                "/google.container.v1beta1.ClusterManager/DeleteNodePool",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// CompleteNodePoolUpgrade will signal an on-going node pool upgrade to
        /// complete.
        pub async fn complete_node_pool_upgrade(
            &mut self,
            request: impl tonic::IntoRequest<super::CompleteNodePoolUpgradeRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
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
                "/google.container.v1beta1.ClusterManager/CompleteNodePoolUpgrade",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Rolls back a previously Aborted or Failed NodePool upgrade.
        /// This makes no changes if the last upgrade successfully completed.
        pub async fn rollback_node_pool_upgrade(
            &mut self,
            request: impl tonic::IntoRequest<super::RollbackNodePoolUpgradeRequest>,
        ) -> Result<tonic::Response<super::Operation>, tonic::Status> {
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
                "/google.container.v1beta1.ClusterManager/RollbackNodePoolUpgrade",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Sets the NodeManagement options for a node pool.
        pub async fn set_node_pool_management(
            &mut self,
            request: impl tonic::IntoRequest<super::SetNodePoolManagementRequest>,
        ) -> Result<tonic::Response<super::Operation>, tonic::Status> {
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
                "/google.container.v1beta1.ClusterManager/SetNodePoolManagement",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Sets labels on a cluster.
        pub async fn set_labels(
            &mut self,
            request: impl tonic::IntoRequest<super::SetLabelsRequest>,
        ) -> Result<tonic::Response<super::Operation>, tonic::Status> {
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
                "/google.container.v1beta1.ClusterManager/SetLabels",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Enables or disables the ABAC authorization mechanism on a cluster.
        pub async fn set_legacy_abac(
            &mut self,
            request: impl tonic::IntoRequest<super::SetLegacyAbacRequest>,
        ) -> Result<tonic::Response<super::Operation>, tonic::Status> {
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
                "/google.container.v1beta1.ClusterManager/SetLegacyAbac",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Starts master IP rotation.
        pub async fn start_ip_rotation(
            &mut self,
            request: impl tonic::IntoRequest<super::StartIpRotationRequest>,
        ) -> Result<tonic::Response<super::Operation>, tonic::Status> {
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
                "/google.container.v1beta1.ClusterManager/StartIPRotation",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Completes master IP rotation.
        pub async fn complete_ip_rotation(
            &mut self,
            request: impl tonic::IntoRequest<super::CompleteIpRotationRequest>,
        ) -> Result<tonic::Response<super::Operation>, tonic::Status> {
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
                "/google.container.v1beta1.ClusterManager/CompleteIPRotation",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// SetNodePoolSizeRequest sets the size of a node pool. The new size will be
        /// used for all replicas, including future replicas created by modifying
        /// [NodePool.locations][google.container.v1beta1.NodePool.locations].
        pub async fn set_node_pool_size(
            &mut self,
            request: impl tonic::IntoRequest<super::SetNodePoolSizeRequest>,
        ) -> Result<tonic::Response<super::Operation>, tonic::Status> {
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
                "/google.container.v1beta1.ClusterManager/SetNodePoolSize",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Enables or disables Network Policy for a cluster.
        pub async fn set_network_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::SetNetworkPolicyRequest>,
        ) -> Result<tonic::Response<super::Operation>, tonic::Status> {
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
                "/google.container.v1beta1.ClusterManager/SetNetworkPolicy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Sets the maintenance policy for a cluster.
        pub async fn set_maintenance_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::SetMaintenancePolicyRequest>,
        ) -> Result<tonic::Response<super::Operation>, tonic::Status> {
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
                "/google.container.v1beta1.ClusterManager/SetMaintenancePolicy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists subnetworks that can be used for creating clusters in a project.
        pub async fn list_usable_subnetworks(
            &mut self,
            request: impl tonic::IntoRequest<super::ListUsableSubnetworksRequest>,
        ) -> Result<
            tonic::Response<super::ListUsableSubnetworksResponse>,
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
                "/google.container.v1beta1.ClusterManager/ListUsableSubnetworks",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Fetches locations that offer Google Kubernetes Engine.
        pub async fn list_locations(
            &mut self,
            request: impl tonic::IntoRequest<super::ListLocationsRequest>,
        ) -> Result<tonic::Response<super::ListLocationsResponse>, tonic::Status> {
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
                "/google.container.v1beta1.ClusterManager/ListLocations",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
