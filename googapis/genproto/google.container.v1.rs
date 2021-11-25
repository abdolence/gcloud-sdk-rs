/// Parameters that describe the nodes in a cluster.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NodeConfig {
    /// The name of a Google Compute Engine [machine
    /// type](<https://cloud.google.com/compute/docs/machine-types>)
    ///
    /// If unspecified, the default machine type is `e2-medium`.
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
    ///  - "cluster-location"
    ///  - "cluster-name"
    ///  - "cluster-uid"
    ///  - "configure-sh"
    ///  - "containerd-configure-sh"
    ///  - "enable-os-login"
    ///  - "gci-ensure-gke-docker"
    ///  - "gci-metrics-enabled"
    ///  - "gci-update-strategy"
    ///  - "instance-template"
    ///  - "kube-env"
    ///  - "startup-script"
    ///  - "user-data"
    ///  - "disable-address-manager"
    ///  - "windows-startup-script-ps1"
    ///  - "common-psm1"
    ///  - "k8s-node-setup-psm1"
    ///  - "install-ssh-psm1"
    ///  - "user-profile-psm1"
    ///
    /// The following keys are reserved for Windows nodes:
    ///  - "serial-port-logging-enable"
    ///
    /// Values are free-form strings, and only have meaning as interpreted by
    /// the image running in the instance. The only restriction placed on them is
    /// that each value's size must be less than or equal to 32 KB.
    ///
    /// The total size of all keys and values must be less than 512 KB.
    #[prost(map = "string, string", tag = "4")]
    pub metadata:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
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
    pub labels:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
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
    /// information about preemptible VM instances.
    #[prost(bool, tag = "10")]
    pub preemptible: bool,
    /// A list of hardware accelerators to be attached to each node.
    /// See <https://cloud.google.com/compute/docs/gpus> for more information about
    /// support for GPUs.
    #[prost(message, repeated, tag = "11")]
    pub accelerators: ::prost::alloc::vec::Vec<AcceleratorConfig>,
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
    /// platform](<https://cloud.google.com/compute/docs/instances/specify-min-cpu-platform>)
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
    /// Shielded Instance options.
    #[prost(message, optional, tag = "20")]
    pub shielded_instance_config: ::core::option::Option<ShieldedInstanceConfig>,
    ///
    /// The Customer Managed Encryption Key used to encrypt the boot disk attached
    /// to each node in the node pool. This should be of the form
    /// projects/\[KEY_PROJECT_ID]/locations/[LOCATION]/keyRings/[RING_NAME]/cryptoKeys/[KEY_NAME\].
    /// For more information about protecting resources with Cloud KMS Keys please
    /// see:
    /// <https://cloud.google.com/compute/docs/disks/customer-managed-encryption>
    #[prost(string, tag = "23")]
    pub boot_disk_kms_key: ::prost::alloc::string::String,
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
    /// Type of the sandbox to use for the node.
    #[prost(enumeration = "sandbox_config::Type", tag = "2")]
    pub r#type: i32,
}
/// Nested message and enum types in `SandboxConfig`.
pub mod sandbox_config {
    /// Possible types of sandboxes.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Type {
        /// Default value. This should not be used.
        Unspecified = 0,
        /// Run sandbox using gvisor.
        Gvisor = 1,
    }
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
    /// SPECIFIC_RESERVATION by name, specify "googleapis.com/reservation-name" as
    /// the key and specify the name of your reservation as its value.
    #[prost(string, tag = "2")]
    pub key: ::prost::alloc::string::String,
    /// Corresponds to the label value(s) of reservation resource(s).
    #[prost(string, repeated, tag = "3")]
    pub values: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Nested message and enum types in `ReservationAffinity`.
pub mod reservation_affinity {
    /// Indicates whether to consume capacity from a reservation or not.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
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
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
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
    /// [Output only] Base64-encoded public certificate that is the root of
    /// trust for the cluster.
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
    /// Configuration for the Cloud Run addon, which allows the user to use a
    /// managed Knative service.
    #[prost(message, optional, tag = "7")]
    pub cloud_run_config: ::core::option::Option<CloudRunConfig>,
    /// Configuration for NodeLocalDNS, a dns cache running on cluster nodes
    #[prost(message, optional, tag = "8")]
    pub dns_cache_config: ::core::option::Option<DnsCacheConfig>,
    /// Configuration for the ConfigConnector add-on, a Kubernetes
    /// extension to manage hosted GCP services through the Kubernetes API
    #[prost(message, optional, tag = "10")]
    pub config_connector_config: ::core::option::Option<ConfigConnectorConfig>,
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
    pub master_global_access_config: ::core::option::Option<PrivateClusterMasterGlobalAccessConfig>,
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
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum LoadBalancerType {
        /// Load balancer type for Cloud Run is unspecified.
        Unspecified = 0,
        /// Install external load balancer for Cloud Run.
        External = 1,
        /// Install internal load balancer for Cloud Run.
        Internal = 2,
    }
}
/// Configuration options for the Config Connector add-on.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConfigConnectorConfig {
    /// Whether Cloud Connector is enabled for this cluster.
    #[prost(bool, tag = "1")]
    pub enabled: bool,
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
    /// cidr_blocks define up to 50 external networks that could access
    /// Kubernetes master through HTTPS.
    #[prost(message, repeated, tag = "2")]
    pub cidr_blocks: ::prost::alloc::vec::Vec<master_authorized_networks_config::CidrBlock>,
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
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Provider {
        /// Not set
        Unspecified = 0,
        /// Tigera (Calico Felix).
        Calico = 1,
    }
}
/// Configuration for Binary Authorization.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BinaryAuthorization {
    /// Enable Binary Authorization for this cluster. If enabled, all container
    /// images will be validated by Binary Authorization.
    #[prost(bool, tag = "1")]
    pub enabled: bool,
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
    /// This field is only applicable with use_ip_aliases is true and
    /// create_subnetwork is false.
    #[prost(string, tag = "7")]
    pub cluster_secondary_range_name: ::prost::alloc::string::String,
    /// The name of the secondary range to be used as for the services
    /// CIDR block.  The secondary range will be used for service
    /// ClusterIPs. This must be an existing secondary range associated
    /// with the cluster subnetwork.
    ///
    /// This field is only applicable with use_ip_aliases is true and
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
    #[prost(string, tag = "13")]
    pub tpu_ipv4_cidr_block: ::prost::alloc::string::String,
    /// Whether routes will be used for pod IPs in the cluster.
    /// This is used in conjunction with use_ip_aliases. It cannot be true if
    /// use_ip_aliases is true. If both use_ip_aliases and use_routes are false,
    /// then the server picks the default IP allocation mode
    #[prost(bool, tag = "15")]
    pub use_routes: bool,
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
    ///   available as of GKE 1.15).
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
    ///   longer available as of GKE 1.15).
    /// * `none` - No metrics will be exported from the cluster.
    ///
    /// If left as an empty string,`monitoring.googleapis.com/kubernetes` will be
    /// used for GKE 1.14+ or `monitoring.googleapis.com` for earlier versions.
    #[prost(string, tag = "7")]
    pub monitoring_service: ::prost::alloc::string::String,
    /// The name of the Google Compute Engine
    /// \[network\](<https://cloud.google.com/compute/docs/networks-and-firewalls#networks>)
    /// to which the cluster is connected. If left unspecified, the `default`
    /// network will be used.
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
    /// the cluster is connected.
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
    /// API groups (e.g. v1alpha1) and features that may not be production ready in
    /// the kubernetes version of the master and nodes.
    /// The cluster has no SLA for uptime and master/node upgrades are disabled.
    /// Alpha enabled clusters are automatically deleted thirty days after
    /// creation.
    #[prost(bool, tag = "14")]
    pub enable_kubernetes_alpha: bool,
    /// The resource labels for the cluster to use to annotate any related
    /// Google Compute Engine resources.
    #[prost(map = "string, string", tag = "15")]
    pub resource_labels:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
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
    pub master_authorized_networks_config: ::core::option::Option<MasterAuthorizedNetworksConfig>,
    /// Configure the maintenance policy for this cluster.
    #[prost(message, optional, tag = "23")]
    pub maintenance_policy: ::core::option::Option<MaintenancePolicy>,
    /// Configuration for Binary Authorization.
    #[prost(message, optional, tag = "24")]
    pub binary_authorization: ::core::option::Option<BinaryAuthorization>,
    /// Cluster-level autoscaling configuration.
    #[prost(message, optional, tag = "26")]
    pub autoscaling: ::core::option::Option<ClusterAutoscaling>,
    /// Configuration for cluster networking.
    #[prost(message, optional, tag = "27")]
    pub network_config: ::core::option::Option<NetworkConfig>,
    /// The default constraint on the maximum number of pods that can be run
    /// simultaneously on a node in the node pool of this cluster. Only honored
    /// if cluster created with IP Alias support.
    #[prost(message, optional, tag = "30")]
    pub default_max_pods_constraint: ::core::option::Option<MaxPodsConstraint>,
    /// Configuration for exporting resource usages. Resource usage export is
    /// disabled when this config is unspecified.
    #[prost(message, optional, tag = "33")]
    pub resource_usage_export_config: ::core::option::Option<ResourceUsageExportConfig>,
    /// Configuration controlling RBAC group membership information.
    #[prost(message, optional, tag = "34")]
    pub authenticator_groups_config: ::core::option::Option<AuthenticatorGroupsConfig>,
    /// Configuration for private cluster.
    #[prost(message, optional, tag = "37")]
    pub private_cluster_config: ::core::option::Option<PrivateClusterConfig>,
    /// Configuration of etcd encryption.
    #[prost(message, optional, tag = "38")]
    pub database_encryption: ::core::option::Option<DatabaseEncryption>,
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
    /// \[NodePools.version\](<https://cloud.google.com/kubernetes-engine/docs/reference/rest/v1/projects.locations.clusters.nodePools>)
    /// instead. The current version of the node software components. If they are
    /// currently at multiple versions because they're in the process of being
    /// upgraded, this reflects the minimum version of all nodes.
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
    #[prost(bool, tag = "115")]
    pub enable_tpu: bool,
    /// [Output only] The IP address range of the Cloud TPUs in this cluster, in
    /// \[CIDR\](<http://en.wikipedia.org/wiki/Classless_Inter-Domain_Routing>)
    /// notation (e.g. `1.2.3.4/29`).
    #[prost(string, tag = "116")]
    pub tpu_ipv4_cidr_block: ::prost::alloc::string::String,
    /// Which conditions caused the current cluster state.
    #[prost(message, repeated, tag = "118")]
    pub conditions: ::prost::alloc::vec::Vec<StatusCondition>,
}
/// Nested message and enum types in `Cluster`.
pub mod cluster {
    /// The current status of the cluster.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
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
        /// The ERROR state indicates the cluster is unusable. It will be
        /// automatically deleted. Details can be found in the `statusMessage` field.
        Error = 5,
        /// The DEGRADED state indicates the cluster requires user action to restore
        /// full functionality. Details can be found in the `statusMessage` field.
        Degraded = 6,
    }
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
    ///   longer available as of GKE 1.15).
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
    /// "desired_node_version", "desired_image_family" or
    /// "desired_node_pool_autoscaling" is specified and there is more than one
    /// node pool on the cluster.
    #[prost(string, tag = "7")]
    pub desired_node_pool_id: ::prost::alloc::string::String,
    /// The desired image type for the node pool.
    /// NOTE: Set the "desired_node_pool" field as well.
    #[prost(string, tag = "8")]
    pub desired_image_type: ::prost::alloc::string::String,
    /// Configuration of etcd encryption.
    #[prost(message, optional, tag = "46")]
    pub desired_database_encryption: ::core::option::Option<DatabaseEncryption>,
    /// Configuration for Workload Identity.
    #[prost(message, optional, tag = "47")]
    pub desired_workload_identity_config: ::core::option::Option<WorkloadIdentityConfig>,
    /// Configuration for Shielded Nodes.
    #[prost(message, optional, tag = "48")]
    pub desired_shielded_nodes: ::core::option::Option<ShieldedNodes>,
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
    pub desired_master_authorized_networks_config:
        ::core::option::Option<MasterAuthorizedNetworksConfig>,
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
    ///   available as of GKE 1.15).
    /// * `none` - no logs will be exported from the cluster.
    ///
    /// If left as an empty string,`logging.googleapis.com/kubernetes` will be
    /// used for GKE 1.14+ or `logging.googleapis.com` for earlier versions.
    #[prost(string, tag = "19")]
    pub desired_logging_service: ::prost::alloc::string::String,
    /// The desired configuration for exporting resource usage.
    #[prost(message, optional, tag = "21")]
    pub desired_resource_usage_export_config: ::core::option::Option<ResourceUsageExportConfig>,
    /// Cluster-level Vertical Pod Autoscaling configuration.
    #[prost(message, optional, tag = "22")]
    pub desired_vertical_pod_autoscaling: ::core::option::Option<VerticalPodAutoscaling>,
    /// The desired private cluster configuration.
    #[prost(message, optional, tag = "25")]
    pub desired_private_cluster_config: ::core::option::Option<PrivateClusterConfig>,
    /// The desired config of Intra-node visibility.
    #[prost(message, optional, tag = "26")]
    pub desired_intra_node_visibility_config: ::core::option::Option<IntraNodeVisibilityConfig>,
    /// The desired status of whether to disable default sNAT for this cluster.
    #[prost(message, optional, tag = "28")]
    pub desired_default_snat_status: ::core::option::Option<DefaultSnatStatus>,
    /// The desired release channel configuration.
    #[prost(message, optional, tag = "31")]
    pub desired_release_channel: ::core::option::Option<ReleaseChannel>,
    /// The desired authenticator groups config for the cluster.
    #[prost(message, optional, tag = "63")]
    pub desired_authenticator_groups_config: ::core::option::Option<AuthenticatorGroupsConfig>,
    /// The Kubernetes version to change the master to.
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
    #[prost(message, repeated, tag = "13")]
    pub cluster_conditions: ::prost::alloc::vec::Vec<StatusCondition>,
    /// Which conditions caused the current node pool state.
    #[prost(message, repeated, tag = "14")]
    pub nodepool_conditions: ::prost::alloc::vec::Vec<StatusCondition>,
}
/// Nested message and enum types in `Operation`.
pub mod operation {
    /// Current status of the operation.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
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
    /// Operation type.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
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
    ///   metrics: [{name: "nodes done",     int_value: 15},
    ///             {name: "nodes total",    int_value: 32}]
    /// or
    ///   metrics: [{name: "progress",       double_value: 0.56},
    ///             {name: "progress scale", double_value: 1.0}]
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
    /// Deprecated. The Google Developers Console [project ID or project
    /// number](<https://support.google.com/cloud/answer/6158840>).
    /// This field has been deprecated and replaced by the parent field.
    #[deprecated]
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    /// Deprecated. The name of the Google Compute Engine
    /// \[zone\](<https://cloud.google.com/compute/docs/zones#available>) in which the
    /// cluster resides. This field has been deprecated and replaced by the parent
    /// field.
    #[deprecated]
    #[prost(string, tag = "2")]
    pub zone: ::prost::alloc::string::String,
    /// Required. A [cluster
    /// resource](<https://cloud.google.com/container-engine/reference/rest/v1/projects.locations.clusters>)
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
    /// Deprecated. The Google Developers Console [project ID or project
    /// number](<https://support.google.com/cloud/answer/6158840>).
    /// This field has been deprecated and replaced by the name field.
    #[deprecated]
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    /// Deprecated. The name of the Google Compute Engine
    /// \[zone\](<https://cloud.google.com/compute/docs/zones#available>) in which the
    /// cluster resides. This field has been deprecated and replaced by the name
    /// field.
    #[deprecated]
    #[prost(string, tag = "2")]
    pub zone: ::prost::alloc::string::String,
    /// Deprecated. The name of the cluster to retrieve.
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
    /// Deprecated. The Google Developers Console [project ID or project
    /// number](<https://support.google.com/cloud/answer/6158840>).
    /// This field has been deprecated and replaced by the name field.
    #[deprecated]
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    /// Deprecated. The name of the Google Compute Engine
    /// \[zone\](<https://cloud.google.com/compute/docs/zones#available>) in which the
    /// cluster resides. This field has been deprecated and replaced by the name
    /// field.
    #[deprecated]
    #[prost(string, tag = "2")]
    pub zone: ::prost::alloc::string::String,
    /// Deprecated. The name of the cluster to upgrade.
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
/// UpdateNodePoolRequests update a node pool's image and/or version.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateNodePoolRequest {
    /// Deprecated. The Google Developers Console [project ID or project
    /// number](<https://support.google.com/cloud/answer/6158840>).
    /// This field has been deprecated and replaced by the name field.
    #[deprecated]
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    /// Deprecated. The name of the Google Compute Engine
    /// \[zone\](<https://cloud.google.com/compute/docs/zones#available>) in which the
    /// cluster resides. This field has been deprecated and replaced by the name
    /// field.
    #[deprecated]
    #[prost(string, tag = "2")]
    pub zone: ::prost::alloc::string::String,
    /// Deprecated. The name of the cluster to upgrade.
    /// This field has been deprecated and replaced by the name field.
    #[deprecated]
    #[prost(string, tag = "3")]
    pub cluster_id: ::prost::alloc::string::String,
    /// Deprecated. The name of the node pool to upgrade.
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
    /// The name (project, location, cluster, node pool) of the node pool to
    /// update. Specified in the format
    /// `projects/*/locations/*/clusters/*/nodePools/*`.
    #[prost(string, tag = "8")]
    pub name: ::prost::alloc::string::String,
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
    /// Upgrade settings control disruption and speed of the upgrade.
    #[prost(message, optional, tag = "15")]
    pub upgrade_settings: ::core::option::Option<node_pool::UpgradeSettings>,
}
/// SetNodePoolAutoscalingRequest sets the autoscaler settings of a node pool.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetNodePoolAutoscalingRequest {
    /// Deprecated. The Google Developers Console [project ID or project
    /// number](<https://support.google.com/cloud/answer/6158840>).
    /// This field has been deprecated and replaced by the name field.
    #[deprecated]
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    /// Deprecated. The name of the Google Compute Engine
    /// \[zone\](<https://cloud.google.com/compute/docs/zones#available>) in which the
    /// cluster resides. This field has been deprecated and replaced by the name
    /// field.
    #[deprecated]
    #[prost(string, tag = "2")]
    pub zone: ::prost::alloc::string::String,
    /// Deprecated. The name of the cluster to upgrade.
    /// This field has been deprecated and replaced by the name field.
    #[deprecated]
    #[prost(string, tag = "3")]
    pub cluster_id: ::prost::alloc::string::String,
    /// Deprecated. The name of the node pool to upgrade.
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
    /// Deprecated. The Google Developers Console [project ID or project
    /// number](<https://support.google.com/cloud/answer/6158840>).
    /// This field has been deprecated and replaced by the name field.
    #[deprecated]
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    /// Deprecated. The name of the Google Compute Engine
    /// \[zone\](<https://cloud.google.com/compute/docs/zones#available>) in which the
    /// cluster resides. This field has been deprecated and replaced by the name
    /// field.
    #[deprecated]
    #[prost(string, tag = "2")]
    pub zone: ::prost::alloc::string::String,
    /// Deprecated. The name of the cluster to upgrade.
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
    ///   available as of GKE 1.15).
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
    /// Deprecated. The Google Developers Console [project ID or project
    /// number](<https://support.google.com/cloud/answer/6158840>).
    /// This field has been deprecated and replaced by the name field.
    #[deprecated]
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    /// Deprecated. The name of the Google Compute Engine
    /// \[zone\](<https://cloud.google.com/compute/docs/zones#available>) in which the
    /// cluster resides. This field has been deprecated and replaced by the name
    /// field.
    #[deprecated]
    #[prost(string, tag = "2")]
    pub zone: ::prost::alloc::string::String,
    /// Deprecated. The name of the cluster to upgrade.
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
    ///   longer available as of GKE 1.15).
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
/// SetAddonsConfigRequest sets the addons associated with the cluster.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetAddonsConfigRequest {
    /// Deprecated. The Google Developers Console [project ID or project
    /// number](<https://support.google.com/cloud/answer/6158840>).
    /// This field has been deprecated and replaced by the name field.
    #[deprecated]
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    /// Deprecated. The name of the Google Compute Engine
    /// \[zone\](<https://cloud.google.com/compute/docs/zones#available>) in which the
    /// cluster resides. This field has been deprecated and replaced by the name
    /// field.
    #[deprecated]
    #[prost(string, tag = "2")]
    pub zone: ::prost::alloc::string::String,
    /// Deprecated. The name of the cluster to upgrade.
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
    /// Deprecated. The Google Developers Console [project ID or project
    /// number](<https://support.google.com/cloud/answer/6158840>).
    /// This field has been deprecated and replaced by the name field.
    #[deprecated]
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    /// Deprecated. The name of the Google Compute Engine
    /// \[zone\](<https://cloud.google.com/compute/docs/zones#available>) in which the
    /// cluster resides. This field has been deprecated and replaced by the name
    /// field.
    #[deprecated]
    #[prost(string, tag = "2")]
    pub zone: ::prost::alloc::string::String,
    /// Deprecated. The name of the cluster to upgrade.
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
    /// Deprecated. The Google Developers Console [project ID or project
    /// number](<https://support.google.com/cloud/answer/6158840>).
    /// This field has been deprecated and replaced by the name field.
    #[deprecated]
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    /// Deprecated. The name of the Google Compute Engine
    /// \[zone\](<https://cloud.google.com/compute/docs/zones#available>) in which the
    /// cluster resides. This field has been deprecated and replaced by the name
    /// field.
    #[deprecated]
    #[prost(string, tag = "2")]
    pub zone: ::prost::alloc::string::String,
    /// Deprecated. The name of the cluster to upgrade.
    /// This field has been deprecated and replaced by the name field.
    #[deprecated]
    #[prost(string, tag = "3")]
    pub cluster_id: ::prost::alloc::string::String,
    /// Required. The Kubernetes version to change the master to.
    ///
    /// Users may specify either explicit versions offered by Kubernetes Engine or
    /// version aliases, which have the following behavior:
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
    /// Deprecated. The Google Developers Console [project ID or project
    /// number](<https://support.google.com/cloud/answer/6158840>).
    /// This field has been deprecated and replaced by the name field.
    #[deprecated]
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    /// Deprecated. The name of the Google Compute Engine
    /// \[zone\](<https://cloud.google.com/compute/docs/zones#available>) in which the
    /// cluster resides. This field has been deprecated and replaced by the name
    /// field.
    #[deprecated]
    #[prost(string, tag = "2")]
    pub zone: ::prost::alloc::string::String,
    /// Deprecated. The name of the cluster to upgrade.
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
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
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
}
/// DeleteClusterRequest deletes a cluster.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteClusterRequest {
    /// Deprecated. The Google Developers Console [project ID or project
    /// number](<https://support.google.com/cloud/answer/6158840>).
    /// This field has been deprecated and replaced by the name field.
    #[deprecated]
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    /// Deprecated. The name of the Google Compute Engine
    /// \[zone\](<https://cloud.google.com/compute/docs/zones#available>) in which the
    /// cluster resides. This field has been deprecated and replaced by the name
    /// field.
    #[deprecated]
    #[prost(string, tag = "2")]
    pub zone: ::prost::alloc::string::String,
    /// Deprecated. The name of the cluster to delete.
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
    /// Deprecated. The Google Developers Console [project ID or project
    /// number](<https://support.google.com/cloud/answer/6158840>).
    /// This field has been deprecated and replaced by the parent field.
    #[deprecated]
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    /// Deprecated. The name of the Google Compute Engine
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
    /// Deprecated. The Google Developers Console [project ID or project
    /// number](<https://support.google.com/cloud/answer/6158840>).
    /// This field has been deprecated and replaced by the name field.
    #[deprecated]
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    /// Deprecated. The name of the Google Compute Engine
    /// \[zone\](<https://cloud.google.com/compute/docs/zones#available>) in which the
    /// cluster resides. This field has been deprecated and replaced by the name
    /// field.
    #[deprecated]
    #[prost(string, tag = "2")]
    pub zone: ::prost::alloc::string::String,
    /// Deprecated. The server-assigned `name` of the operation.
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
    /// Deprecated. The Google Developers Console [project ID or project
    /// number](<https://support.google.com/cloud/answer/6158840>).
    /// This field has been deprecated and replaced by the parent field.
    #[deprecated]
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    /// Deprecated. The name of the Google Compute Engine
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
    /// Deprecated. The Google Developers Console [project ID or project
    /// number](<https://support.google.com/cloud/answer/6158840>).
    /// This field has been deprecated and replaced by the name field.
    #[deprecated]
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    /// Deprecated. The name of the Google Compute Engine
    /// \[zone\](<https://cloud.google.com/compute/docs/zones#available>) in which the
    /// operation resides. This field has been deprecated and replaced by the name
    /// field.
    #[deprecated]
    #[prost(string, tag = "2")]
    pub zone: ::prost::alloc::string::String,
    /// Deprecated. The server-assigned `name` of the operation.
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
    /// Deprecated. The Google Developers Console [project ID or project
    /// number](<https://support.google.com/cloud/answer/6158840>).
    /// This field has been deprecated and replaced by the name field.
    #[deprecated]
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    /// Deprecated. The name of the Google Compute Engine
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
        /// List of valid versions for the channel.
        #[prost(string, repeated, tag = "4")]
        pub valid_versions: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
}
/// CreateNodePoolRequest creates a node pool for a cluster.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateNodePoolRequest {
    /// Deprecated. The Google Developers Console [project ID or project
    /// number](<https://developers.google.com/console/help/new/#projectnumber>).
    /// This field has been deprecated and replaced by the parent field.
    #[deprecated]
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    /// Deprecated. The name of the Google Compute Engine
    /// \[zone\](<https://cloud.google.com/compute/docs/zones#available>) in which the
    /// cluster resides. This field has been deprecated and replaced by the parent
    /// field.
    #[deprecated]
    #[prost(string, tag = "2")]
    pub zone: ::prost::alloc::string::String,
    /// Deprecated. The name of the cluster.
    /// This field has been deprecated and replaced by the parent field.
    #[deprecated]
    #[prost(string, tag = "3")]
    pub cluster_id: ::prost::alloc::string::String,
    /// Required. The node pool to create.
    #[prost(message, optional, tag = "4")]
    pub node_pool: ::core::option::Option<NodePool>,
    /// The parent (project, location, cluster id) where the node pool will be
    /// created. Specified in the format
    /// `projects/*/locations/*/clusters/*`.
    #[prost(string, tag = "6")]
    pub parent: ::prost::alloc::string::String,
}
/// DeleteNodePoolRequest deletes a node pool for a cluster.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteNodePoolRequest {
    /// Deprecated. The Google Developers Console [project ID or project
    /// number](<https://developers.google.com/console/help/new/#projectnumber>).
    /// This field has been deprecated and replaced by the name field.
    #[deprecated]
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    /// Deprecated. The name of the Google Compute Engine
    /// \[zone\](<https://cloud.google.com/compute/docs/zones#available>) in which the
    /// cluster resides. This field has been deprecated and replaced by the name
    /// field.
    #[deprecated]
    #[prost(string, tag = "2")]
    pub zone: ::prost::alloc::string::String,
    /// Deprecated. The name of the cluster.
    /// This field has been deprecated and replaced by the name field.
    #[deprecated]
    #[prost(string, tag = "3")]
    pub cluster_id: ::prost::alloc::string::String,
    /// Deprecated. The name of the node pool to delete.
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
    /// Deprecated. The Google Developers Console [project ID or project
    /// number](<https://developers.google.com/console/help/new/#projectnumber>).
    /// This field has been deprecated and replaced by the parent field.
    #[deprecated]
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    /// Deprecated. The name of the Google Compute Engine
    /// \[zone\](<https://cloud.google.com/compute/docs/zones#available>) in which the
    /// cluster resides. This field has been deprecated and replaced by the parent
    /// field.
    #[deprecated]
    #[prost(string, tag = "2")]
    pub zone: ::prost::alloc::string::String,
    /// Deprecated. The name of the cluster.
    /// This field has been deprecated and replaced by the parent field.
    #[deprecated]
    #[prost(string, tag = "3")]
    pub cluster_id: ::prost::alloc::string::String,
    /// The parent (project, location, cluster id) where the node pools will be
    /// listed. Specified in the format `projects/*/locations/*/clusters/*`.
    #[prost(string, tag = "5")]
    pub parent: ::prost::alloc::string::String,
}
/// GetNodePoolRequest retrieves a node pool for a cluster.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetNodePoolRequest {
    /// Deprecated. The Google Developers Console [project ID or project
    /// number](<https://developers.google.com/console/help/new/#projectnumber>).
    /// This field has been deprecated and replaced by the name field.
    #[deprecated]
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    /// Deprecated. The name of the Google Compute Engine
    /// \[zone\](<https://cloud.google.com/compute/docs/zones#available>) in which the
    /// cluster resides. This field has been deprecated and replaced by the name
    /// field.
    #[deprecated]
    #[prost(string, tag = "2")]
    pub zone: ::prost::alloc::string::String,
    /// Deprecated. The name of the cluster.
    /// This field has been deprecated and replaced by the name field.
    #[deprecated]
    #[prost(string, tag = "3")]
    pub cluster_id: ::prost::alloc::string::String,
    /// Deprecated. The name of the node pool.
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
/// NodePool contains the name and configuration for a cluster's node pool.
/// Node pools are a set of nodes (i.e. VM's), with a common configuration and
/// specification, under the control of the cluster master. They may have a set
/// of Kubernetes labels applied to them, which may be used to reference them
/// during pod scheduling. They may also be resized up or down, to accommodate
/// the workload.
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
    /// [Output only] Server-defined URL for the resource.
    #[prost(string, tag = "100")]
    pub self_link: ::prost::alloc::string::String,
    /// The version of the Kubernetes of this node.
    #[prost(string, tag = "101")]
    pub version: ::prost::alloc::string::String,
    /// [Output only] The resource URLs of the [managed instance
    /// groups](<https://cloud.google.com/compute/docs/instance-groups/creating-groups-of-managed-instances>)
    /// associated with this node pool.
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
}
/// Nested message and enum types in `NodePool`.
pub mod node_pool {
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
    }
    /// The current status of the node pool instance.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
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
}
/// NodeManagement defines the set of node management services turned on for the
/// node pool.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NodeManagement {
    /// A flag that specifies whether node auto-upgrade is enabled for the node
    /// pool. If enabled, node auto-upgrade helps keep the nodes in your node pool
    /// up to date with the latest release version of Kubernetes.
    #[prost(bool, tag = "1")]
    pub auto_upgrade: bool,
    /// A flag that specifies whether the node auto-repair is enabled for the node
    /// pool. If enabled, the nodes in this node pool will be monitored and, if
    /// they fail health checks too many times, an automatic repair action will be
    /// triggered.
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
    pub maintenance_exclusions:
        ::std::collections::HashMap<::prost::alloc::string::String, TimeWindow>,
    #[prost(oneof = "maintenance_window::Policy", tags = "2, 3")]
    pub policy: ::core::option::Option<maintenance_window::Policy>,
}
/// Nested message and enum types in `MaintenanceWindow`.
pub mod maintenance_window {
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
    /// Time format should be in \[RFC3339\](<https://www.ietf.org/rfc/rfc3339.txt>)
    /// format "HH:MM", where HH : \[00-23\] and MM : \[00-59\] GMT.
    #[prost(string, tag = "2")]
    pub start_time: ::prost::alloc::string::String,
    /// [Output only] Duration of the time window, automatically chosen to be
    /// smallest possible in the given scenario.
    /// Duration will be in \[RFC3339\](<https://www.ietf.org/rfc/rfc3339.txt>)
    /// format "PTnHnMnS".
    #[prost(string, tag = "3")]
    pub duration: ::prost::alloc::string::String,
}
/// SetNodePoolManagementRequest sets the node management properties of a node
/// pool.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetNodePoolManagementRequest {
    /// Deprecated. The Google Developers Console [project ID or project
    /// number](<https://support.google.com/cloud/answer/6158840>).
    /// This field has been deprecated and replaced by the name field.
    #[deprecated]
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    /// Deprecated. The name of the Google Compute Engine
    /// \[zone\](<https://cloud.google.com/compute/docs/zones#available>) in which the
    /// cluster resides. This field has been deprecated and replaced by the name
    /// field.
    #[deprecated]
    #[prost(string, tag = "2")]
    pub zone: ::prost::alloc::string::String,
    /// Deprecated. The name of the cluster to update.
    /// This field has been deprecated and replaced by the name field.
    #[deprecated]
    #[prost(string, tag = "3")]
    pub cluster_id: ::prost::alloc::string::String,
    /// Deprecated. The name of the node pool to update.
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
/// SetNodePoolSizeRequest sets the size a node
/// pool.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetNodePoolSizeRequest {
    /// Deprecated. The Google Developers Console [project ID or project
    /// number](<https://support.google.com/cloud/answer/6158840>).
    /// This field has been deprecated and replaced by the name field.
    #[deprecated]
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    /// Deprecated. The name of the Google Compute Engine
    /// \[zone\](<https://cloud.google.com/compute/docs/zones#available>) in which the
    /// cluster resides. This field has been deprecated and replaced by the name
    /// field.
    #[deprecated]
    #[prost(string, tag = "2")]
    pub zone: ::prost::alloc::string::String,
    /// Deprecated. The name of the cluster to update.
    /// This field has been deprecated and replaced by the name field.
    #[deprecated]
    #[prost(string, tag = "3")]
    pub cluster_id: ::prost::alloc::string::String,
    /// Deprecated. The name of the node pool to update.
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
/// RollbackNodePoolUpgradeRequest rollbacks the previously Aborted or Failed
/// NodePool upgrade. This will be an no-op if the last upgrade successfully
/// completed.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RollbackNodePoolUpgradeRequest {
    /// Deprecated. The Google Developers Console [project ID or project
    /// number](<https://support.google.com/cloud/answer/6158840>).
    /// This field has been deprecated and replaced by the name field.
    #[deprecated]
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    /// Deprecated. The name of the Google Compute Engine
    /// \[zone\](<https://cloud.google.com/compute/docs/zones#available>) in which the
    /// cluster resides. This field has been deprecated and replaced by the name
    /// field.
    #[deprecated]
    #[prost(string, tag = "2")]
    pub zone: ::prost::alloc::string::String,
    /// Deprecated. The name of the cluster to rollback.
    /// This field has been deprecated and replaced by the name field.
    #[deprecated]
    #[prost(string, tag = "3")]
    pub cluster_id: ::prost::alloc::string::String,
    /// Deprecated. The name of the node pool to rollback.
    /// This field has been deprecated and replaced by the name field.
    #[deprecated]
    #[prost(string, tag = "4")]
    pub node_pool_id: ::prost::alloc::string::String,
    /// The name (project, location, cluster, node pool id) of the node poll to
    /// rollback upgrade.
    /// Specified in the format `projects/*/locations/*/clusters/*/nodePools/*`.
    #[prost(string, tag = "6")]
    pub name: ::prost::alloc::string::String,
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
    /// AutoprovisioningNodePoolDefaults contains defaults for a node pool
    /// created by NAP.
    #[prost(message, optional, tag = "4")]
    pub autoprovisioning_node_pool_defaults:
        ::core::option::Option<AutoprovisioningNodePoolDefaults>,
    /// The list of Google Compute Engine
    /// \[zones\](<https://cloud.google.com/compute/docs/zones#available>) in which the
    /// NodePool's nodes can be created by NAP.
    #[prost(string, repeated, tag = "5")]
    pub autoprovisioning_locations: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// AutoprovisioningNodePoolDefaults contains defaults for a node pool created
/// by NAP.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AutoprovisioningNodePoolDefaults {
    /// Scopes that are used by NAP when creating node pools.
    #[prost(string, repeated, tag = "1")]
    pub oauth_scopes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The Google Cloud Platform Service Account to be used by the node VMs.
    #[prost(string, tag = "2")]
    pub service_account: ::prost::alloc::string::String,
    /// Specifies the upgrade settings for NAP created node pools
    #[prost(message, optional, tag = "3")]
    pub upgrade_settings: ::core::option::Option<node_pool::UpgradeSettings>,
    /// Specifies the node management options for NAP created node-pools.
    #[prost(message, optional, tag = "4")]
    pub management: ::core::option::Option<NodeManagement>,
    /// Minimum CPU platform to be used for NAP created node pools.
    /// The instance may be scheduled on the specified or newer CPU platform.
    /// Applicable values are the friendly names of CPU platforms, such as
    /// minCpuPlatform: Intel Haswell or
    /// minCpuPlatform: Intel Sandy Bridge. For more
    /// information, read [how to specify min CPU
    /// platform](<https://cloud.google.com/compute/docs/instances/specify-min-cpu-platform>)
    /// To unset the min cpu platform field pass "automatic"
    /// as field value.
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
    /// Minimum number of nodes in the NodePool. Must be >= 1 and <=
    /// max_node_count.
    #[prost(int32, tag = "2")]
    pub min_node_count: i32,
    /// Maximum number of nodes in the NodePool. Must be >= min_node_count. There
    /// has to enough quota to scale up the cluster.
    #[prost(int32, tag = "3")]
    pub max_node_count: i32,
    /// Can this node pool be deleted automatically.
    #[prost(bool, tag = "4")]
    pub autoprovisioned: bool,
}
/// SetLabelsRequest sets the Google Cloud Platform labels on a Google Container
/// Engine cluster, which will in turn set them for Google Compute Engine
/// resources used by that cluster
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetLabelsRequest {
    /// Deprecated. The Google Developers Console [project ID or project
    /// number](<https://developers.google.com/console/help/new/#projectnumber>).
    /// This field has been deprecated and replaced by the name field.
    #[deprecated]
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    /// Deprecated. The name of the Google Compute Engine
    /// \[zone\](<https://cloud.google.com/compute/docs/zones#available>) in which the
    /// cluster resides. This field has been deprecated and replaced by the name
    /// field.
    #[deprecated]
    #[prost(string, tag = "2")]
    pub zone: ::prost::alloc::string::String,
    /// Deprecated. The name of the cluster.
    /// This field has been deprecated and replaced by the name field.
    #[deprecated]
    #[prost(string, tag = "3")]
    pub cluster_id: ::prost::alloc::string::String,
    /// Required. The labels to set for that cluster.
    #[prost(map = "string, string", tag = "4")]
    pub resource_labels:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Required. The fingerprint of the previous set of labels for this resource,
    /// used to detect conflicts. The fingerprint is initially generated by
    /// Kubernetes Engine and changes after every request to modify or update
    /// labels. You must always provide an up-to-date fingerprint hash when
    /// updating or changing labels. Make a `get()` request to the
    /// resource to get the latest fingerprint.
    #[prost(string, tag = "5")]
    pub label_fingerprint: ::prost::alloc::string::String,
    /// The name (project, location, cluster id) of the cluster to set labels.
    /// Specified in the format `projects/*/locations/*/clusters/*`.
    #[prost(string, tag = "7")]
    pub name: ::prost::alloc::string::String,
}
/// SetLegacyAbacRequest enables or disables the ABAC authorization mechanism for
/// a cluster.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetLegacyAbacRequest {
    /// Deprecated. The Google Developers Console [project ID or project
    /// number](<https://support.google.com/cloud/answer/6158840>).
    /// This field has been deprecated and replaced by the name field.
    #[deprecated]
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    /// Deprecated. The name of the Google Compute Engine
    /// \[zone\](<https://cloud.google.com/compute/docs/zones#available>) in which the
    /// cluster resides. This field has been deprecated and replaced by the name
    /// field.
    #[deprecated]
    #[prost(string, tag = "2")]
    pub zone: ::prost::alloc::string::String,
    /// Deprecated. The name of the cluster to update.
    /// This field has been deprecated and replaced by the name field.
    #[deprecated]
    #[prost(string, tag = "3")]
    pub cluster_id: ::prost::alloc::string::String,
    /// Required. Whether ABAC authorization will be enabled in the cluster.
    #[prost(bool, tag = "4")]
    pub enabled: bool,
    /// The name (project, location, cluster id) of the cluster to set legacy abac.
    /// Specified in the format `projects/*/locations/*/clusters/*`.
    #[prost(string, tag = "6")]
    pub name: ::prost::alloc::string::String,
}
/// StartIPRotationRequest creates a new IP for the cluster and then performs
/// a node upgrade on each node pool to point to the new IP.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StartIpRotationRequest {
    /// Deprecated. The Google Developers Console [project ID or project
    /// number](<https://developers.google.com/console/help/new/#projectnumber>).
    /// This field has been deprecated and replaced by the name field.
    #[deprecated]
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    /// Deprecated. The name of the Google Compute Engine
    /// \[zone\](<https://cloud.google.com/compute/docs/zones#available>) in which the
    /// cluster resides. This field has been deprecated and replaced by the name
    /// field.
    #[deprecated]
    #[prost(string, tag = "2")]
    pub zone: ::prost::alloc::string::String,
    /// Deprecated. The name of the cluster.
    /// This field has been deprecated and replaced by the name field.
    #[deprecated]
    #[prost(string, tag = "3")]
    pub cluster_id: ::prost::alloc::string::String,
    /// The name (project, location, cluster id) of the cluster to start IP
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
    /// Deprecated. The Google Developers Console [project ID or project
    /// number](<https://developers.google.com/console/help/new/#projectnumber>).
    /// This field has been deprecated and replaced by the name field.
    #[deprecated]
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    /// Deprecated. The name of the Google Compute Engine
    /// \[zone\](<https://cloud.google.com/compute/docs/zones#available>) in which the
    /// cluster resides. This field has been deprecated and replaced by the name
    /// field.
    #[deprecated]
    #[prost(string, tag = "2")]
    pub zone: ::prost::alloc::string::String,
    /// Deprecated. The name of the cluster.
    /// This field has been deprecated and replaced by the name field.
    #[deprecated]
    #[prost(string, tag = "3")]
    pub cluster_id: ::prost::alloc::string::String,
    /// The name (project, location, cluster id) of the cluster to complete IP
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
}
/// WorkloadMetadataConfig defines the metadata configuration to expose to
/// workloads on the node pool.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WorkloadMetadataConfig {
    /// Mode is the configuration for how to expose metadata to workloads running
    /// on the node pool.
    #[prost(enumeration = "workload_metadata_config::Mode", tag = "2")]
    pub mode: i32,
}
/// Nested message and enum types in `WorkloadMetadataConfig`.
pub mod workload_metadata_config {
    /// Mode is the configuration for how to expose metadata to workloads running
    /// on the node.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
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
}
/// SetNetworkPolicyRequest enables/disables network policy for a cluster.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetNetworkPolicyRequest {
    /// Deprecated. The Google Developers Console [project ID or project
    /// number](<https://developers.google.com/console/help/new/#projectnumber>).
    /// This field has been deprecated and replaced by the name field.
    #[deprecated]
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    /// Deprecated. The name of the Google Compute Engine
    /// \[zone\](<https://cloud.google.com/compute/docs/zones#available>) in which the
    /// cluster resides. This field has been deprecated and replaced by the name
    /// field.
    #[deprecated]
    #[prost(string, tag = "2")]
    pub zone: ::prost::alloc::string::String,
    /// Deprecated. The name of the cluster.
    /// This field has been deprecated and replaced by the name field.
    #[deprecated]
    #[prost(string, tag = "3")]
    pub cluster_id: ::prost::alloc::string::String,
    /// Required. Configuration options for the NetworkPolicy feature.
    #[prost(message, optional, tag = "4")]
    pub network_policy: ::core::option::Option<NetworkPolicy>,
    /// The name (project, location, cluster id) of the cluster to set networking
    /// policy. Specified in the format `projects/*/locations/*/clusters/*`.
    #[prost(string, tag = "6")]
    pub name: ::prost::alloc::string::String,
}
/// SetMaintenancePolicyRequest sets the maintenance policy for a cluster.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetMaintenancePolicyRequest {
    /// Required. The Google Developers Console [project ID or project
    /// number](<https://support.google.com/cloud/answer/6158840>).
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
    /// The name (project, location, cluster id) of the cluster to set maintenance
    /// policy.
    /// Specified in the format `projects/*/locations/*/clusters/*`.
    #[prost(string, tag = "5")]
    pub name: ::prost::alloc::string::String,
}
/// StatusCondition describes why a cluster or a node pool has a certain status
/// (e.g., ERROR or DEGRADED).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StatusCondition {
    /// Machine-friendly representation of the condition
    #[prost(enumeration = "status_condition::Code", tag = "1")]
    pub code: i32,
    /// Human-friendly representation of the condition
    #[prost(string, tag = "2")]
    pub message: ::prost::alloc::string::String,
}
/// Nested message and enum types in `StatusCondition`.
pub mod status_condition {
    /// Code for each condition
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
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
        /// More codes TBA
        CloudKmsKeyError = 7,
    }
}
/// NetworkConfig reports the relative names of network & subnetwork.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NetworkConfig {
    /// Output only. The relative name of the Google Compute Engine
    /// \[network][google.container.v1.NetworkConfig.network\](<https://cloud.google.com/compute/docs/networks-and-firewalls#networks>)
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
}
/// GetOpenIDConfigRequest gets the OIDC discovery document for the
/// cluster. See the OpenID Connect Discovery 1.0 specification for details.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetOpenIdConfigRequest {
    /// The cluster (project, location, cluster id) to get the discovery document
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
    pub response_types_supported: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Supported subject types.
    #[prost(string, repeated, tag = "4")]
    pub subject_types_supported: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// supported ID Token signing Algorithms.
    #[prost(string, repeated, tag = "5")]
    pub id_token_signing_alg_values_supported:
        ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
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
    /// The cluster (project, location, cluster id) to get keys for. Specified in
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
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
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
}
/// IntraNodeVisibilityConfig contains the desired config of the intra-node
/// visibility on this cluster.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IntraNodeVisibilityConfig {
    /// Enables intra node visibility for this cluster.
    #[prost(bool, tag = "1")]
    pub enabled: bool,
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
    /// The workload pool to attach all Kubernetes service accounts to.
    #[prost(string, tag = "2")]
    pub workload_pool: ::prost::alloc::string::String,
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
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
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
}
/// ListUsableSubnetworksRequest requests the list of usable subnetworks
/// available to a user for creating clusters.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListUsableSubnetworksRequest {
    /// The parent project where subnetworks are usable.
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
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
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
/// Configuration for exporting cluster resource usages.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResourceUsageExportConfig {
    /// Configuration to use BigQuery as usage export destination.
    #[prost(message, optional, tag = "1")]
    pub bigquery_destination:
        ::core::option::Option<resource_usage_export_config::BigQueryDestination>,
    /// Whether to enable network egress metering for this cluster. If enabled, a
    /// daemonset will be created in the cluster to meter network egress traffic.
    #[prost(bool, tag = "2")]
    pub enable_network_egress_metering: bool,
    /// Configuration to enable resource consumption metering.
    #[prost(message, optional, tag = "3")]
    pub consumption_metering_config:
        ::core::option::Option<resource_usage_export_config::ConsumptionMeteringConfig>,
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
/// Configuration of Shielded Nodes feature.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShieldedNodes {
    /// Whether Shielded Nodes features are enabled on all nodes in this cluster.
    #[prost(bool, tag = "1")]
    pub enabled: bool,
}
#[doc = r" Generated client implementations."]
pub mod cluster_manager_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Google Kubernetes Engine Cluster Manager v1"]
    #[derive(Debug, Clone)]
    pub struct ClusterManagerClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> ClusterManagerClient<T>
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
        ) -> ClusterManagerClient<InterceptedService<T, F>>
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
            ClusterManagerClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Lists all clusters owned by a project in either the specified zone or all"]
        #[doc = " zones."]
        pub async fn list_clusters(
            &mut self,
            request: impl tonic::IntoRequest<super::ListClustersRequest>,
        ) -> Result<tonic::Response<super::ListClustersResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.container.v1.ClusterManager/ListClusters",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets the details of a specific cluster."]
        pub async fn get_cluster(
            &mut self,
            request: impl tonic::IntoRequest<super::GetClusterRequest>,
        ) -> Result<tonic::Response<super::Cluster>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.container.v1.ClusterManager/GetCluster",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a cluster, consisting of the specified number and type of Google"]
        #[doc = " Compute Engine instances."]
        #[doc = ""]
        #[doc = " By default, the cluster is created in the project's"]
        #[doc = " [default"]
        #[doc = " network](https://cloud.google.com/compute/docs/networks-and-firewalls#networks)."]
        #[doc = ""]
        #[doc = " One firewall is added for the cluster. After cluster creation,"]
        #[doc = " the Kubelet creates routes for each node to allow the containers"]
        #[doc = " on that node to communicate with all other instances in the"]
        #[doc = " cluster."]
        #[doc = ""]
        #[doc = " Finally, an entry is added to the project's global metadata indicating"]
        #[doc = " which CIDR range the cluster is using."]
        pub async fn create_cluster(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateClusterRequest>,
        ) -> Result<tonic::Response<super::Operation>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.container.v1.ClusterManager/CreateCluster",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates the settings of a specific cluster."]
        pub async fn update_cluster(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateClusterRequest>,
        ) -> Result<tonic::Response<super::Operation>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.container.v1.ClusterManager/UpdateCluster",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates the version and/or image type for the specified node pool."]
        pub async fn update_node_pool(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateNodePoolRequest>,
        ) -> Result<tonic::Response<super::Operation>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.container.v1.ClusterManager/UpdateNodePool",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Sets the autoscaling settings for the specified node pool."]
        pub async fn set_node_pool_autoscaling(
            &mut self,
            request: impl tonic::IntoRequest<super::SetNodePoolAutoscalingRequest>,
        ) -> Result<tonic::Response<super::Operation>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.container.v1.ClusterManager/SetNodePoolAutoscaling",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Sets the logging service for a specific cluster."]
        pub async fn set_logging_service(
            &mut self,
            request: impl tonic::IntoRequest<super::SetLoggingServiceRequest>,
        ) -> Result<tonic::Response<super::Operation>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.container.v1.ClusterManager/SetLoggingService",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Sets the monitoring service for a specific cluster."]
        pub async fn set_monitoring_service(
            &mut self,
            request: impl tonic::IntoRequest<super::SetMonitoringServiceRequest>,
        ) -> Result<tonic::Response<super::Operation>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.container.v1.ClusterManager/SetMonitoringService",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Sets the addons for a specific cluster."]
        pub async fn set_addons_config(
            &mut self,
            request: impl tonic::IntoRequest<super::SetAddonsConfigRequest>,
        ) -> Result<tonic::Response<super::Operation>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.container.v1.ClusterManager/SetAddonsConfig",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Sets the locations for a specific cluster."]
        #[doc = " Deprecated. Use"]
        #[doc = " [projects.locations.clusters.update](https://cloud.google.com/kubernetes-engine/docs/reference/rest/v1/projects.locations.clusters/update)"]
        #[doc = " instead."]
        pub async fn set_locations(
            &mut self,
            request: impl tonic::IntoRequest<super::SetLocationsRequest>,
        ) -> Result<tonic::Response<super::Operation>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.container.v1.ClusterManager/SetLocations",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates the master for a specific cluster."]
        pub async fn update_master(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateMasterRequest>,
        ) -> Result<tonic::Response<super::Operation>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.container.v1.ClusterManager/UpdateMaster",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Sets master auth materials. Currently supports changing the admin password"]
        #[doc = " or a specific cluster, either via password generation or explicitly setting"]
        #[doc = " the password."]
        pub async fn set_master_auth(
            &mut self,
            request: impl tonic::IntoRequest<super::SetMasterAuthRequest>,
        ) -> Result<tonic::Response<super::Operation>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.container.v1.ClusterManager/SetMasterAuth",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes the cluster, including the Kubernetes endpoint and all worker"]
        #[doc = " nodes."]
        #[doc = ""]
        #[doc = " Firewalls and routes that were configured during cluster creation"]
        #[doc = " are also deleted."]
        #[doc = ""]
        #[doc = " Other Google Compute Engine resources that might be in use by the cluster,"]
        #[doc = " such as load balancer resources, are not deleted if they weren't present"]
        #[doc = " when the cluster was initially created."]
        pub async fn delete_cluster(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteClusterRequest>,
        ) -> Result<tonic::Response<super::Operation>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.container.v1.ClusterManager/DeleteCluster",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists all operations in a project in a specific zone or all zones."]
        pub async fn list_operations(
            &mut self,
            request: impl tonic::IntoRequest<super::ListOperationsRequest>,
        ) -> Result<tonic::Response<super::ListOperationsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.container.v1.ClusterManager/ListOperations",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets the specified operation."]
        pub async fn get_operation(
            &mut self,
            request: impl tonic::IntoRequest<super::GetOperationRequest>,
        ) -> Result<tonic::Response<super::Operation>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.container.v1.ClusterManager/GetOperation",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Cancels the specified operation."]
        pub async fn cancel_operation(
            &mut self,
            request: impl tonic::IntoRequest<super::CancelOperationRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.container.v1.ClusterManager/CancelOperation",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns configuration info about the Google Kubernetes Engine service."]
        pub async fn get_server_config(
            &mut self,
            request: impl tonic::IntoRequest<super::GetServerConfigRequest>,
        ) -> Result<tonic::Response<super::ServerConfig>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.container.v1.ClusterManager/GetServerConfig",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets the public component of the cluster signing keys in"]
        #[doc = " JSON Web Key format."]
        #[doc = " This API is not yet intended for general use, and is not available for all"]
        #[doc = " clusters."]
        pub async fn get_json_web_keys(
            &mut self,
            request: impl tonic::IntoRequest<super::GetJsonWebKeysRequest>,
        ) -> Result<tonic::Response<super::GetJsonWebKeysResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.container.v1.ClusterManager/GetJSONWebKeys",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists the node pools for a cluster."]
        pub async fn list_node_pools(
            &mut self,
            request: impl tonic::IntoRequest<super::ListNodePoolsRequest>,
        ) -> Result<tonic::Response<super::ListNodePoolsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.container.v1.ClusterManager/ListNodePools",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Retrieves the requested node pool."]
        pub async fn get_node_pool(
            &mut self,
            request: impl tonic::IntoRequest<super::GetNodePoolRequest>,
        ) -> Result<tonic::Response<super::NodePool>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.container.v1.ClusterManager/GetNodePool",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a node pool for a cluster."]
        pub async fn create_node_pool(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateNodePoolRequest>,
        ) -> Result<tonic::Response<super::Operation>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.container.v1.ClusterManager/CreateNodePool",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes a node pool from a cluster."]
        pub async fn delete_node_pool(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteNodePoolRequest>,
        ) -> Result<tonic::Response<super::Operation>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.container.v1.ClusterManager/DeleteNodePool",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Rolls back a previously Aborted or Failed NodePool upgrade."]
        #[doc = " This makes no changes if the last upgrade successfully completed."]
        pub async fn rollback_node_pool_upgrade(
            &mut self,
            request: impl tonic::IntoRequest<super::RollbackNodePoolUpgradeRequest>,
        ) -> Result<tonic::Response<super::Operation>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.container.v1.ClusterManager/RollbackNodePoolUpgrade",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Sets the NodeManagement options for a node pool."]
        pub async fn set_node_pool_management(
            &mut self,
            request: impl tonic::IntoRequest<super::SetNodePoolManagementRequest>,
        ) -> Result<tonic::Response<super::Operation>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.container.v1.ClusterManager/SetNodePoolManagement",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Sets labels on a cluster."]
        pub async fn set_labels(
            &mut self,
            request: impl tonic::IntoRequest<super::SetLabelsRequest>,
        ) -> Result<tonic::Response<super::Operation>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.container.v1.ClusterManager/SetLabels",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Enables or disables the ABAC authorization mechanism on a cluster."]
        pub async fn set_legacy_abac(
            &mut self,
            request: impl tonic::IntoRequest<super::SetLegacyAbacRequest>,
        ) -> Result<tonic::Response<super::Operation>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.container.v1.ClusterManager/SetLegacyAbac",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Starts master IP rotation."]
        pub async fn start_ip_rotation(
            &mut self,
            request: impl tonic::IntoRequest<super::StartIpRotationRequest>,
        ) -> Result<tonic::Response<super::Operation>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.container.v1.ClusterManager/StartIPRotation",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Completes master IP rotation."]
        pub async fn complete_ip_rotation(
            &mut self,
            request: impl tonic::IntoRequest<super::CompleteIpRotationRequest>,
        ) -> Result<tonic::Response<super::Operation>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.container.v1.ClusterManager/CompleteIPRotation",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Sets the size for a specific node pool."]
        pub async fn set_node_pool_size(
            &mut self,
            request: impl tonic::IntoRequest<super::SetNodePoolSizeRequest>,
        ) -> Result<tonic::Response<super::Operation>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.container.v1.ClusterManager/SetNodePoolSize",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Enables or disables Network Policy for a cluster."]
        pub async fn set_network_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::SetNetworkPolicyRequest>,
        ) -> Result<tonic::Response<super::Operation>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.container.v1.ClusterManager/SetNetworkPolicy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Sets the maintenance policy for a cluster."]
        pub async fn set_maintenance_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::SetMaintenancePolicyRequest>,
        ) -> Result<tonic::Response<super::Operation>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.container.v1.ClusterManager/SetMaintenancePolicy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists subnetworks that are usable for creating clusters in a project."]
        pub async fn list_usable_subnetworks(
            &mut self,
            request: impl tonic::IntoRequest<super::ListUsableSubnetworksRequest>,
        ) -> Result<tonic::Response<super::ListUsableSubnetworksResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.container.v1.ClusterManager/ListUsableSubnetworks",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
