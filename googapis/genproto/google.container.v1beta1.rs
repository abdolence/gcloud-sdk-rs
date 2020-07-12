/// Parameters that describe the nodes in a cluster.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NodeConfig {
    /// The name of a Google Compute Engine [machine
    /// type](https://cloud.google.com/compute/docs/machine-types) (e.g.
    /// `n1-standard-1`).
    ///
    /// If unspecified, the default machine type is
    /// `n1-standard-1`.
    #[prost(string, tag = "1")]
    pub machine_type: std::string::String,
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
    /// * `https://www.googleapis.com/auth/compute` is required for mounting
    /// persistent storage on your nodes.
    /// * `https://www.googleapis.com/auth/devstorage.read_only` is required for
    /// communicating with **gcr.io**
    /// (the [Google Container Registry](https://cloud.google.com/container-registry/)).
    ///
    /// If unspecified, no scopes are added, unless Cloud Logging or Cloud
    /// Monitoring are enabled, in which case their required scopes will be added.
    #[prost(string, repeated, tag = "3")]
    pub oauth_scopes: ::std::vec::Vec<std::string::String>,
    /// The Google Cloud Platform Service Account to be used by the node VMs. If
    /// no Service Account is specified, the "default" service account is used.
    #[prost(string, tag = "9")]
    pub service_account: std::string::String,
    /// The metadata key/value pairs assigned to instances in the cluster.
    ///
    /// Keys must conform to the regexp [a-zA-Z0-9-_]+ and be less than 128 bytes
    /// in length. These are reflected as part of a URL in the metadata server.
    /// Additionally, to avoid ambiguity, keys must not conflict with any other
    /// metadata keys for the project or be one of the reserved keys:
    ///  "cluster-location"
    ///  "cluster-name"
    ///  "cluster-uid"
    ///  "configure-sh"
    ///  "containerd-configure-sh"
    ///  "enable-oslogin"
    ///  "gci-ensure-gke-docker"
    ///  "gci-metrics-enabled"
    ///  "gci-update-strategy"
    ///  "instance-template"
    ///  "kube-env"
    ///  "startup-script"
    ///  "user-data"
    ///  "disable-address-manager"
    ///  "windows-startup-script-ps1"
    ///  "common-psm1"
    ///  "k8s-node-setup-psm1"
    ///  "install-ssh-psm1"
    ///  "user-profile-psm1"
    ///  "serial-port-logging-enable"
    /// Values are free-form strings, and only have meaning as interpreted by
    /// the image running in the instance. The only restriction placed on them is
    /// that each value's size must be less than or equal to 32 KB.
    ///
    /// The total size of all keys and values must be less than 512 KB.
    #[prost(map = "string, string", tag = "4")]
    pub metadata: ::std::collections::HashMap<std::string::String, std::string::String>,
    /// The image type to use for this node. Note that for a given image type,
    /// the latest version of it will be used.
    #[prost(string, tag = "5")]
    pub image_type: std::string::String,
    /// The map of Kubernetes labels (key/value pairs) to be applied to each node.
    /// These will added in addition to any default label(s) that
    /// Kubernetes may apply to the node.
    /// In case of conflict in label keys, the applied set may differ depending on
    /// the Kubernetes version -- it's best to assume the behavior is undefined
    /// and conflicts should be avoided.
    /// For more information, including usage and the valid values, see:
    /// https://kubernetes.io/docs/concepts/overview/working-with-objects/labels/
    #[prost(map = "string, string", tag = "6")]
    pub labels: ::std::collections::HashMap<std::string::String, std::string::String>,
    /// The number of local SSD disks to be attached to the node.
    ///
    /// The limit for this value is dependent upon the maximum number of
    /// disks available on a machine per zone. See:
    /// https://cloud.google.com/compute/docs/disks/local-ssd
    /// for more information.
    #[prost(int32, tag = "7")]
    pub local_ssd_count: i32,
    /// The list of instance tags applied to all nodes. Tags are used to identify
    /// valid sources or targets for network firewalls and are specified by
    /// the client during cluster or node pool creation. Each tag within the list
    /// must comply with RFC1035.
    #[prost(string, repeated, tag = "8")]
    pub tags: ::std::vec::Vec<std::string::String>,
    /// Whether the nodes are created as preemptible VM instances. See:
    /// https://cloud.google.com/compute/docs/instances/preemptible for more
    /// inforamtion about preemptible VM instances.
    #[prost(bool, tag = "10")]
    pub preemptible: bool,
    /// A list of hardware accelerators to be attached to each node.
    /// See https://cloud.google.com/compute/docs/gpus for more information about
    /// support for GPUs.
    #[prost(message, repeated, tag = "11")]
    pub accelerators: ::std::vec::Vec<AcceleratorConfig>,
    /// Type of the disk attached to each node (e.g. 'pd-standard' or 'pd-ssd')
    ///
    /// If unspecified, the default disk type is 'pd-standard'
    #[prost(string, tag = "12")]
    pub disk_type: std::string::String,
    /// Minimum CPU platform to be used by this instance. The instance may be
    /// scheduled on the specified or newer CPU platform. Applicable values are the
    /// friendly names of CPU platforms, such as
    /// <code>minCpuPlatform: &quot;Intel Haswell&quot;</code> or
    /// <code>minCpuPlatform: &quot;Intel Sandy Bridge&quot;</code>. For more
    /// information, read [how to specify min CPU
    /// platform](https://cloud.google.com/compute/docs/instances/specify-min-cpu-platform)
    /// To unset the min cpu platform field pass "automatic" as field value.
    #[prost(string, tag = "13")]
    pub min_cpu_platform: std::string::String,
    /// The workload metadata configuration for this node.
    #[prost(message, optional, tag = "14")]
    pub workload_metadata_config: ::std::option::Option<WorkloadMetadataConfig>,
    /// List of kubernetes taints to be applied to each node.
    ///
    /// For more information, including usage and the valid values, see:
    /// https://kubernetes.io/docs/concepts/configuration/taint-and-toleration/
    #[prost(message, repeated, tag = "15")]
    pub taints: ::std::vec::Vec<NodeTaint>,
    /// Shielded Instance options.
    #[prost(message, optional, tag = "20")]
    pub shielded_instance_config: ::std::option::Option<ShieldedInstanceConfig>,
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
/// Kubernetes taint is comprised of three fields: key, value, and effect. Effect
/// can only be one of three types:  NoSchedule, PreferNoSchedule or NoExecute.
///
/// For more information, including usage and the valid values, see:
/// https://kubernetes.io/docs/concepts/configuration/taint-and-toleration/
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NodeTaint {
    /// Key for taint.
    #[prost(string, tag = "1")]
    pub key: std::string::String,
    /// Value for taint.
    #[prost(string, tag = "2")]
    pub value: std::string::String,
    /// Effect for taint.
    #[prost(enumeration = "node_taint::Effect", tag = "3")]
    pub effect: i32,
}
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
    #[prost(string, tag = "1")]
    pub username: std::string::String,
    /// The password to use for HTTP basic authentication to the master endpoint.
    /// Because the master endpoint is open to the Internet, you should create a
    /// strong password.  If a password is provided for cluster creation, username
    /// must be non-empty.
    #[prost(string, tag = "2")]
    pub password: std::string::String,
    /// Configuration for client certificate authentication on the cluster. For
    /// clusters before v1.12, if no configuration is specified, a client
    /// certificate is issued.
    #[prost(message, optional, tag = "3")]
    pub client_certificate_config: ::std::option::Option<ClientCertificateConfig>,
    /// [Output only] Base64-encoded public certificate that is the root of
    /// trust for the cluster.
    #[prost(string, tag = "100")]
    pub cluster_ca_certificate: std::string::String,
    /// [Output only] Base64-encoded public certificate used by clients to
    /// authenticate to the cluster endpoint.
    #[prost(string, tag = "101")]
    pub client_certificate: std::string::String,
    /// [Output only] Base64-encoded private key used by clients to authenticate
    /// to the cluster endpoint.
    #[prost(string, tag = "102")]
    pub client_key: std::string::String,
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
    pub http_load_balancing: ::std::option::Option<HttpLoadBalancing>,
    /// Configuration for the horizontal pod autoscaling feature, which
    /// increases or decreases the number of replica pods a replication controller
    /// has based on the resource usage of the existing pods.
    #[prost(message, optional, tag = "2")]
    pub horizontal_pod_autoscaling: ::std::option::Option<HorizontalPodAutoscaling>,
    /// Configuration for the Kubernetes Dashboard.
    /// This addon is deprecated, and will be disabled in 1.15. It is recommended
    /// to use the Cloud Console to manage and monitor your Kubernetes clusters,
    /// workloads and applications. For more information, see:
    /// https://cloud.google.com/kubernetes-engine/docs/concepts/dashboards
    #[prost(message, optional, tag = "3")]
    pub kubernetes_dashboard: ::std::option::Option<KubernetesDashboard>,
    /// Configuration for NetworkPolicy. This only tracks whether the addon
    /// is enabled or not on the Master, it does not track whether network policy
    /// is enabled for the nodes.
    #[prost(message, optional, tag = "4")]
    pub network_policy_config: ::std::option::Option<NetworkPolicyConfig>,
    /// Configuration for Istio, an open platform to connect, manage, and secure
    /// microservices.
    #[prost(message, optional, tag = "5")]
    pub istio_config: ::std::option::Option<IstioConfig>,
    /// Configuration for the Cloud Run addon. The `IstioConfig` addon must be
    /// enabled in order to enable Cloud Run addon. This option can only be enabled
    /// at cluster creation time.
    #[prost(message, optional, tag = "7")]
    pub cloud_run_config: ::std::option::Option<CloudRunConfig>,
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
    /// When enabled, it ensures that a Heapster pod is running in the cluster,
    /// which is also used by the Cloud Monitoring service.
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
    pub master_ipv4_cidr_block: std::string::String,
    /// Output only. The internal IP address of this cluster's master endpoint.
    #[prost(string, tag = "4")]
    pub private_endpoint: std::string::String,
    /// Output only. The external IP address of this cluster's master endpoint.
    #[prost(string, tag = "5")]
    pub public_endpoint: std::string::String,
}
/// Configuration options for Istio addon.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IstioConfig {
    /// Whether Istio is enabled for this cluster.
    #[prost(bool, tag = "1")]
    pub disabled: bool,
    /// The specified Istio auth mode, either none, or mutual TLS.
    #[prost(enumeration = "istio_config::IstioAuthMode", tag = "2")]
    pub auth: i32,
}
pub mod istio_config {
    /// Istio auth mode, https://istio.io/docs/concepts/security/mutual-tls.html
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum IstioAuthMode {
        /// auth not enabled
        AuthNone = 0,
        /// auth mutual TLS enabled
        AuthMutualTls = 1,
    }
}
/// Configuration options for the Cloud Run feature.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CloudRunConfig {
    /// Whether Cloud Run addon is enabled for this cluster.
    #[prost(bool, tag = "1")]
    pub disabled: bool,
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
    pub cidr_blocks: ::std::vec::Vec<master_authorized_networks_config::CidrBlock>,
}
pub mod master_authorized_networks_config {
    /// CidrBlock contains an optional name and one CIDR block.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CidrBlock {
        /// display_name is an optional field for users to identify CIDR blocks.
        #[prost(string, tag = "1")]
        pub display_name: std::string::String,
        /// cidr_block must be specified in CIDR notation.
        #[prost(string, tag = "2")]
        pub cidr_block: std::string::String,
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
/// https://kubernetes.io/docs/concepts/services-networking/networkpolicies/
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NetworkPolicy {
    /// The selected network policy provider.
    #[prost(enumeration = "network_policy::Provider", tag = "1")]
    pub provider: i32,
    /// Whether network policy is enabled on the cluster.
    #[prost(bool, tag = "2")]
    pub enabled: bool,
}
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
/// Configuration for controlling how IPs are allocated in the cluster.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IpAllocationPolicy {
    /// Whether alias IPs will be used for pod IPs in the cluster.
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
    pub subnetwork_name: std::string::String,
    /// This field is deprecated, use cluster_ipv4_cidr_block.
    #[prost(string, tag = "4")]
    pub cluster_ipv4_cidr: std::string::String,
    /// This field is deprecated, use node_ipv4_cidr_block.
    #[prost(string, tag = "5")]
    pub node_ipv4_cidr: std::string::String,
    /// This field is deprecated, use services_ipv4_cidr_block.
    #[prost(string, tag = "6")]
    pub services_ipv4_cidr: std::string::String,
    /// The name of the secondary range to be used for the cluster CIDR
    /// block.  The secondary range will be used for pod IP
    /// addresses. This must be an existing secondary range associated
    /// with the cluster subnetwork.
    ///
    /// This field is only applicable with use_ip_aliases and
    /// create_subnetwork is false.
    #[prost(string, tag = "7")]
    pub cluster_secondary_range_name: std::string::String,
    /// The name of the secondary range to be used as for the services
    /// CIDR block.  The secondary range will be used for service
    /// ClusterIPs. This must be an existing secondary range associated
    /// with the cluster subnetwork.
    ///
    /// This field is only applicable with use_ip_aliases and
    /// create_subnetwork is false.
    #[prost(string, tag = "8")]
    pub services_secondary_range_name: std::string::String,
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
    /// [CIDR](http://en.wikipedia.org/wiki/Classless_Inter-Domain_Routing)
    /// notation (e.g. `10.96.0.0/14`) from the RFC-1918 private networks (e.g.
    /// `10.0.0.0/8`, `172.16.0.0/12`, `192.168.0.0/16`) to pick a specific range
    /// to use.
    #[prost(string, tag = "9")]
    pub cluster_ipv4_cidr_block: std::string::String,
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
    /// [CIDR](http://en.wikipedia.org/wiki/Classless_Inter-Domain_Routing)
    /// notation (e.g. `10.96.0.0/14`) from the RFC-1918 private networks (e.g.
    /// `10.0.0.0/8`, `172.16.0.0/12`, `192.168.0.0/16`) to pick a specific range
    /// to use.
    #[prost(string, tag = "10")]
    pub node_ipv4_cidr_block: std::string::String,
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
    /// [CIDR](http://en.wikipedia.org/wiki/Classless_Inter-Domain_Routing)
    /// notation (e.g. `10.96.0.0/14`) from the RFC-1918 private networks (e.g.
    /// `10.0.0.0/8`, `172.16.0.0/12`, `192.168.0.0/16`) to pick a specific range
    /// to use.
    #[prost(string, tag = "11")]
    pub services_ipv4_cidr_block: std::string::String,
    /// If true, allow allocation of cluster CIDR ranges that overlap with certain
    /// kinds of network routes. By default we do not allow cluster CIDR ranges to
    /// intersect with any user declared routes. With allow_route_overlap == true,
    /// we allow overlapping with CIDR ranges that are larger than the cluster CIDR
    /// range.
    ///
    /// If this field is set to true, then cluster and services CIDRs must be
    /// fully-specified (e.g. `10.96.0.0/14`, but not `/14`), which means:
    /// 1) When `use_ip_aliases` is true, `cluster_ipv4_cidr_block` and
    ///    `services_ipv4_cidr_block` must be fully-specified.
    /// 2) When `use_ip_aliases` is false, `cluster.cluster_ipv4_cidr` muse be
    ///    fully-specified.
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
    /// [CIDR](http://en.wikipedia.org/wiki/Classless_Inter-Domain_Routing)
    /// notation (e.g. `10.96.0.0/14`) from the RFC-1918 private networks (e.g.
    /// `10.0.0.0/8`, `172.16.0.0/12`, `192.168.0.0/16`) to pick a specific range
    /// to use.
    #[prost(string, tag = "13")]
    pub tpu_ipv4_cidr_block: std::string::String,
}
/// Configuration for Binary Authorization.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BinaryAuthorization {
    /// Enable Binary Authorization for this cluster. If enabled, all container
    /// images will be validated by Google Binauthz.
    #[prost(bool, tag = "1")]
    pub enabled: bool,
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
    pub security_group: std::string::String,
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
    pub name: std::string::String,
    /// An optional description of this cluster.
    #[prost(string, tag = "2")]
    pub description: std::string::String,
    /// The number of nodes to create in this cluster. You must ensure that your
    /// Compute Engine [resource quota](https://cloud.google.com/compute/quotas)
    /// is sufficient for this number of instances. You must also have available
    /// firewall and routes quota.
    /// For requests, this field should only be used in lieu of a
    /// "node_pool" object, since this configuration (along with the
    /// "node_config") will be used to create a "NodePool" object with an
    /// auto-generated name. Do not use this and a node_pool at the same time.
    ///
    /// This field is deprecated, use node_pool.initial_node_count instead.
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
    #[prost(message, optional, tag = "4")]
    pub node_config: ::std::option::Option<NodeConfig>,
    /// The authentication information for accessing the master endpoint.
    /// If unspecified, the defaults are used:
    /// For clusters before v1.12, if master_auth is unspecified, `username` will
    /// be set to "admin", a random password will be generated, and a client
    /// certificate will be issued.
    #[prost(message, optional, tag = "5")]
    pub master_auth: ::std::option::Option<MasterAuth>,
    /// The logging service the cluster should use to write logs.
    /// Currently available options:
    ///
    /// * `logging.googleapis.com` - the Google Cloud Logging service.
    /// * `none` - no logs will be exported from the cluster.
    /// * if left as an empty string,`logging.googleapis.com` will be used.
    #[prost(string, tag = "6")]
    pub logging_service: std::string::String,
    /// The monitoring service the cluster should use to write metrics.
    /// Currently available options:
    ///
    /// * `monitoring.googleapis.com` - the Google Cloud Monitoring service.
    /// * `none` - no metrics will be exported from the cluster.
    /// * if left as an empty string, `monitoring.googleapis.com` will be used.
    #[prost(string, tag = "7")]
    pub monitoring_service: std::string::String,
    /// The name of the Google Compute Engine
    /// [network](https://cloud.google.com/compute/docs/networks-and-firewalls#networks) to which the
    /// cluster is connected. If left unspecified, the `default` network
    /// will be used. On output this shows the network ID instead of
    /// the name.
    #[prost(string, tag = "8")]
    pub network: std::string::String,
    /// The IP address range of the container pods in this cluster, in
    /// [CIDR](http://en.wikipedia.org/wiki/Classless_Inter-Domain_Routing)
    /// notation (e.g. `10.96.0.0/14`). Leave blank to have
    /// one automatically chosen or specify a `/14` block in `10.0.0.0/8`.
    #[prost(string, tag = "9")]
    pub cluster_ipv4_cidr: std::string::String,
    /// Configurations for the various addons available to run in the cluster.
    #[prost(message, optional, tag = "10")]
    pub addons_config: ::std::option::Option<AddonsConfig>,
    /// The name of the Google Compute Engine
    /// [subnetwork](https://cloud.google.com/compute/docs/subnetworks) to which the
    /// cluster is connected. On output this shows the subnetwork ID instead of
    /// the name.
    #[prost(string, tag = "11")]
    pub subnetwork: std::string::String,
    /// The node pools associated with this cluster.
    /// This field should not be set if "node_config" or "initial_node_count" are
    /// specified.
    #[prost(message, repeated, tag = "12")]
    pub node_pools: ::std::vec::Vec<NodePool>,
    /// The list of Google Compute Engine
    /// [zones](https://cloud.google.com/compute/docs/zones#available) in which the cluster's nodes
    /// should be located.
    #[prost(string, repeated, tag = "13")]
    pub locations: ::std::vec::Vec<std::string::String>,
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
    pub resource_labels: ::std::collections::HashMap<std::string::String, std::string::String>,
    /// The fingerprint of the set of labels for this cluster.
    #[prost(string, tag = "16")]
    pub label_fingerprint: std::string::String,
    /// Configuration for the legacy ABAC authorization mode.
    #[prost(message, optional, tag = "18")]
    pub legacy_abac: ::std::option::Option<LegacyAbac>,
    /// Configuration options for the NetworkPolicy feature.
    #[prost(message, optional, tag = "19")]
    pub network_policy: ::std::option::Option<NetworkPolicy>,
    /// Configuration for cluster IP allocation.
    #[prost(message, optional, tag = "20")]
    pub ip_allocation_policy: ::std::option::Option<IpAllocationPolicy>,
    /// The configuration options for master authorized networks feature.
    #[prost(message, optional, tag = "22")]
    pub master_authorized_networks_config: ::std::option::Option<MasterAuthorizedNetworksConfig>,
    /// Configure the maintenance policy for this cluster.
    #[prost(message, optional, tag = "23")]
    pub maintenance_policy: ::std::option::Option<MaintenancePolicy>,
    /// Configuration for Binary Authorization.
    #[prost(message, optional, tag = "24")]
    pub binary_authorization: ::std::option::Option<BinaryAuthorization>,
    /// Configuration for the PodSecurityPolicy feature.
    #[prost(message, optional, tag = "25")]
    pub pod_security_policy_config: ::std::option::Option<PodSecurityPolicyConfig>,
    /// Cluster-level autoscaling configuration.
    #[prost(message, optional, tag = "26")]
    pub autoscaling: ::std::option::Option<ClusterAutoscaling>,
    /// Configuration for cluster networking.
    #[prost(message, optional, tag = "27")]
    pub network_config: ::std::option::Option<NetworkConfig>,
    /// If this is a private cluster setup. Private clusters are clusters that, by
    /// default have no external IP addresses on the nodes and where nodes and the
    /// master communicate over private IP addresses.
    /// This field is deprecated, use private_cluster_config.enable_private_nodes
    /// instead.
    #[prost(bool, tag = "28")]
    pub private_cluster: bool,
    /// The IP prefix in CIDR notation to use for the hosted master network.
    /// This prefix will be used for assigning private IP addresses to the
    /// master or set of masters, as well as the ILB VIP.
    /// This field is deprecated, use
    /// private_cluster_config.master_ipv4_cidr_block instead.
    #[prost(string, tag = "29")]
    pub master_ipv4_cidr_block: std::string::String,
    /// The default constraint on the maximum number of pods that can be run
    /// simultaneously on a node in the node pool of this cluster. Only honored
    /// if cluster created with IP Alias support.
    #[prost(message, optional, tag = "30")]
    pub default_max_pods_constraint: ::std::option::Option<MaxPodsConstraint>,
    /// Configuration for exporting resource usages. Resource usage export is
    /// disabled when this config unspecified.
    #[prost(message, optional, tag = "33")]
    pub resource_usage_export_config: ::std::option::Option<ResourceUsageExportConfig>,
    /// Configuration controlling RBAC group membership information.
    #[prost(message, optional, tag = "34")]
    pub authenticator_groups_config: ::std::option::Option<AuthenticatorGroupsConfig>,
    /// Configuration for private cluster.
    #[prost(message, optional, tag = "37")]
    pub private_cluster_config: ::std::option::Option<PrivateClusterConfig>,
    /// Cluster-level Vertical Pod Autoscaling configuration.
    #[prost(message, optional, tag = "39")]
    pub vertical_pod_autoscaling: ::std::option::Option<VerticalPodAutoscaling>,
    /// [Output only] Server-defined URL for the resource.
    #[prost(string, tag = "100")]
    pub self_link: std::string::String,
    /// [Output only] The name of the Google Compute Engine
    /// [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster
    /// resides.
    /// This field is deprecated, use location instead.
    #[prost(string, tag = "101")]
    pub zone: std::string::String,
    /// [Output only] The IP address of this cluster's master endpoint.
    /// The endpoint can be accessed from the internet at
    /// `https://username:password@endpoint/`.
    ///
    /// See the `masterAuth` property of this resource for username and
    /// password information.
    #[prost(string, tag = "102")]
    pub endpoint: std::string::String,
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
    pub initial_cluster_version: std::string::String,
    /// [Output only] The current software version of the master endpoint.
    #[prost(string, tag = "104")]
    pub current_master_version: std::string::String,
    /// [Output only] Deprecated, use
    /// [NodePool.version](https://cloud.google.com/kubernetes-engine/docs/reference/rest/v1beta1/projects.locations.clusters.nodePools)
    /// instead. The current version of the node software components.
    /// If they are currently at multiple versions because they're in the process
    /// of being upgraded, this reflects the minimum version of all nodes.
    #[prost(string, tag = "105")]
    pub current_node_version: std::string::String,
    /// [Output only] The time the cluster was created, in
    /// [RFC3339](https://www.ietf.org/rfc/rfc3339.txt) text format.
    #[prost(string, tag = "106")]
    pub create_time: std::string::String,
    /// [Output only] The current status of this cluster.
    #[prost(enumeration = "cluster::Status", tag = "107")]
    pub status: i32,
    /// [Output only] Additional information about the current status of this
    /// cluster, if available.
    #[prost(string, tag = "108")]
    pub status_message: std::string::String,
    /// [Output only] The size of the address space on each node for hosting
    /// containers. This is provisioned from within the `container_ipv4_cidr`
    /// range. This field will only be set when cluster is in route-based network
    /// mode.
    #[prost(int32, tag = "109")]
    pub node_ipv4_cidr_size: i32,
    /// [Output only] The IP address range of the Kubernetes services in
    /// this cluster, in
    /// [CIDR](http://en.wikipedia.org/wiki/Classless_Inter-Domain_Routing)
    /// notation (e.g. `1.2.3.4/29`). Service addresses are
    /// typically put in the last `/16` from the container CIDR.
    #[prost(string, tag = "110")]
    pub services_ipv4_cidr: std::string::String,
    /// Deprecated. Use node_pools.instance_group_urls.
    #[prost(string, repeated, tag = "111")]
    pub instance_group_urls: ::std::vec::Vec<std::string::String>,
    /// [Output only]  The number of nodes currently in the cluster. Deprecated.
    /// Call Kubernetes API directly to retrieve node information.
    #[prost(int32, tag = "112")]
    pub current_node_count: i32,
    /// [Output only] The time the cluster will be automatically
    /// deleted in [RFC3339](https://www.ietf.org/rfc/rfc3339.txt) text format.
    #[prost(string, tag = "113")]
    pub expire_time: std::string::String,
    /// [Output only] The name of the Google Compute Engine
    /// [zone](https://cloud.google.com/compute/docs/regions-zones/regions-zones#available) or
    /// [region](https://cloud.google.com/compute/docs/regions-zones/regions-zones#available) in which
    /// the cluster resides.
    #[prost(string, tag = "114")]
    pub location: std::string::String,
    /// Enable the ability to use Cloud TPUs in this cluster.
    #[prost(bool, tag = "115")]
    pub enable_tpu: bool,
    /// [Output only] The IP address range of the Cloud TPUs in this cluster, in
    /// [CIDR](http://en.wikipedia.org/wiki/Classless_Inter-Domain_Routing)
    /// notation (e.g. `1.2.3.4/29`).
    #[prost(string, tag = "116")]
    pub tpu_ipv4_cidr_block: std::string::String,
    /// Configuration of etcd encryption.
    #[prost(message, optional, tag = "38")]
    pub database_encryption: ::std::option::Option<DatabaseEncryption>,
    /// Which conditions caused the current cluster state.
    #[prost(message, repeated, tag = "118")]
    pub conditions: ::std::vec::Vec<StatusCondition>,
}
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
        /// The ERROR state indicates the cluster may be unusable. Details
        /// can be found in the `statusMessage` field.
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
    pub desired_node_version: std::string::String,
    /// The monitoring service the cluster should use to write metrics.
    /// Currently available options:
    ///
    /// * "monitoring.googleapis.com/kubernetes" - the Google Cloud Monitoring
    /// service with Kubernetes-native resource model
    /// * "monitoring.googleapis.com" - the Google Cloud Monitoring service
    /// * "none" - no metrics will be exported from the cluster
    #[prost(string, tag = "5")]
    pub desired_monitoring_service: std::string::String,
    /// Configurations for the various addons available to run in the cluster.
    #[prost(message, optional, tag = "6")]
    pub desired_addons_config: ::std::option::Option<AddonsConfig>,
    /// The node pool to be upgraded. This field is mandatory if
    /// "desired_node_version", "desired_image_family",
    /// "desired_node_pool_autoscaling", or "desired_workload_metadata_config"
    /// is specified and there is more than one node pool on the cluster.
    #[prost(string, tag = "7")]
    pub desired_node_pool_id: std::string::String,
    /// The desired image type for the node pool.
    /// NOTE: Set the "desired_node_pool" field as well.
    #[prost(string, tag = "8")]
    pub desired_image_type: std::string::String,
    /// Autoscaler configuration for the node pool specified in
    /// desired_node_pool_id. If there is only one pool in the
    /// cluster and desired_node_pool_id is not provided then
    /// the change applies to that single node pool.
    #[prost(message, optional, tag = "9")]
    pub desired_node_pool_autoscaling: ::std::option::Option<NodePoolAutoscaling>,
    /// The desired list of Google Compute Engine
    /// [zones](https://cloud.google.com/compute/docs/zones#available) in which the cluster's nodes
    /// should be located. Changing the locations a cluster is in will result
    /// in nodes being either created or removed from the cluster, depending on
    /// whether locations are being added or removed.
    ///
    /// This list must always include the cluster's primary zone.
    #[prost(string, repeated, tag = "10")]
    pub desired_locations: ::std::vec::Vec<std::string::String>,
    /// The desired configuration options for master authorized networks feature.
    #[prost(message, optional, tag = "12")]
    pub desired_master_authorized_networks_config:
        ::std::option::Option<MasterAuthorizedNetworksConfig>,
    /// The desired configuration options for the PodSecurityPolicy feature.
    #[prost(message, optional, tag = "14")]
    pub desired_pod_security_policy_config: ::std::option::Option<PodSecurityPolicyConfig>,
    /// Cluster-level autoscaling configuration.
    #[prost(message, optional, tag = "15")]
    pub desired_cluster_autoscaling: ::std::option::Option<ClusterAutoscaling>,
    /// The desired configuration options for the Binary Authorization feature.
    #[prost(message, optional, tag = "16")]
    pub desired_binary_authorization: ::std::option::Option<BinaryAuthorization>,
    /// The logging service the cluster should use to write metrics.
    /// Currently available options:
    ///
    /// * "logging.googleapis.com/kubernetes" - the Google Cloud Logging
    /// service with Kubernetes-native resource model
    /// * "logging.googleapis.com" - the Google Cloud Logging service
    /// * "none" - no logs will be exported from the cluster
    #[prost(string, tag = "19")]
    pub desired_logging_service: std::string::String,
    /// The desired configuration for exporting resource usage.
    #[prost(message, optional, tag = "21")]
    pub desired_resource_usage_export_config: ::std::option::Option<ResourceUsageExportConfig>,
    /// Cluster-level Vertical Pod Autoscaling configuration.
    #[prost(message, optional, tag = "22")]
    pub desired_vertical_pod_autoscaling: ::std::option::Option<VerticalPodAutoscaling>,
    /// The desired config of Intra-node visibility.
    #[prost(message, optional, tag = "26")]
    pub desired_intra_node_visibility_config: ::std::option::Option<IntraNodeVisibilityConfig>,
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
    pub desired_master_version: std::string::String,
}
/// This operation resource represents operations that may have happened or are
/// happening on the cluster. All fields are output only.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Operation {
    /// The server-assigned ID for the operation.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// The name of the Google Compute Engine
    /// [zone](https://cloud.google.com/compute/docs/zones#available) in which the operation
    /// is taking place.
    /// This field is deprecated, use location instead.
    #[prost(string, tag = "2")]
    pub zone: std::string::String,
    /// The operation type.
    #[prost(enumeration = "operation::Type", tag = "3")]
    pub operation_type: i32,
    /// The current status of the operation.
    #[prost(enumeration = "operation::Status", tag = "4")]
    pub status: i32,
    /// Detailed operation progress, if available.
    #[prost(string, tag = "8")]
    pub detail: std::string::String,
    /// If an error has occurred, a textual description of the error.
    #[prost(string, tag = "5")]
    pub status_message: std::string::String,
    /// Server-defined URL for the resource.
    #[prost(string, tag = "6")]
    pub self_link: std::string::String,
    /// Server-defined URL for the target of the operation.
    #[prost(string, tag = "7")]
    pub target_link: std::string::String,
    /// [Output only] The name of the Google Compute Engine
    /// [zone](https://cloud.google.com/compute/docs/regions-zones/regions-zones#available) or
    /// [region](https://cloud.google.com/compute/docs/regions-zones/regions-zones#available) in which
    /// the cluster resides.
    #[prost(string, tag = "9")]
    pub location: std::string::String,
    /// [Output only] The time the operation started, in
    /// [RFC3339](https://www.ietf.org/rfc/rfc3339.txt) text format.
    #[prost(string, tag = "10")]
    pub start_time: std::string::String,
    /// [Output only] The time the operation completed, in
    /// [RFC3339](https://www.ietf.org/rfc/rfc3339.txt) text format.
    #[prost(string, tag = "11")]
    pub end_time: std::string::String,
    /// [Output only] Progress information for an operation.
    #[prost(message, optional, tag = "12")]
    pub progress: ::std::option::Option<OperationProgress>,
    /// Which conditions caused the current cluster state.
    #[prost(message, repeated, tag = "13")]
    pub cluster_conditions: ::std::vec::Vec<StatusCondition>,
    /// Which conditions caused the current node pool state.
    #[prost(message, repeated, tag = "14")]
    pub nodepool_conditions: ::std::vec::Vec<StatusCondition>,
}
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
    pub name: std::string::String,
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
    pub metrics: ::std::vec::Vec<operation_progress::Metric>,
    /// Substages of an operation or a stage.
    #[prost(message, repeated, tag = "4")]
    pub stages: ::std::vec::Vec<OperationProgress>,
}
pub mod operation_progress {
    /// Progress metric is (string, int|float|string) pair.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Metric {
        /// Metric name, required.
        /// e.g., "nodes total", "percent done"
        #[prost(string, tag = "1")]
        pub name: std::string::String,
        /// Strictly one of the values is required.
        #[prost(oneof = "metric::Value", tags = "2, 3, 4")]
        pub value: ::std::option::Option<metric::Value>,
    }
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
            StringValue(std::string::String),
        }
    }
}
/// CreateClusterRequest creates a cluster.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateClusterRequest {
    /// Required. Deprecated. The Google Developers Console [project ID or project
    /// number](https://support.google.com/cloud/answer/6158840).
    /// This field has been deprecated and replaced by the parent field.
    #[prost(string, tag = "1")]
    pub project_id: std::string::String,
    /// Required. Deprecated. The name of the Google Compute Engine
    /// [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster
    /// resides.
    /// This field has been deprecated and replaced by the parent field.
    #[prost(string, tag = "2")]
    pub zone: std::string::String,
    /// Required. A [cluster
    /// resource](https://cloud.google.com/container-engine/reference/rest/v1beta1/projects.zones.clusters)
    #[prost(message, optional, tag = "3")]
    pub cluster: ::std::option::Option<Cluster>,
    /// The parent (project and location) where the cluster will be created.
    /// Specified in the format `projects/*/locations/*`.
    #[prost(string, tag = "5")]
    pub parent: std::string::String,
}
/// GetClusterRequest gets the settings of a cluster.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetClusterRequest {
    /// Required. Deprecated. The Google Developers Console [project ID or project
    /// number](https://support.google.com/cloud/answer/6158840).
    /// This field has been deprecated and replaced by the name field.
    #[prost(string, tag = "1")]
    pub project_id: std::string::String,
    /// Required. Deprecated. The name of the Google Compute Engine
    /// [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster
    /// resides.
    /// This field has been deprecated and replaced by the name field.
    #[prost(string, tag = "2")]
    pub zone: std::string::String,
    /// Required. Deprecated. The name of the cluster to retrieve.
    /// This field has been deprecated and replaced by the name field.
    #[prost(string, tag = "3")]
    pub cluster_id: std::string::String,
    /// The name (project, location, cluster) of the cluster to retrieve.
    /// Specified in the format `projects/*/locations/*/clusters/*`.
    #[prost(string, tag = "5")]
    pub name: std::string::String,
}
/// UpdateClusterRequest updates the settings of a cluster.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateClusterRequest {
    /// Required. Deprecated. The Google Developers Console [project ID or project
    /// number](https://support.google.com/cloud/answer/6158840).
    /// This field has been deprecated and replaced by the name field.
    #[prost(string, tag = "1")]
    pub project_id: std::string::String,
    /// Required. Deprecated. The name of the Google Compute Engine
    /// [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster
    /// resides.
    /// This field has been deprecated and replaced by the name field.
    #[prost(string, tag = "2")]
    pub zone: std::string::String,
    /// Required. Deprecated. The name of the cluster to upgrade.
    /// This field has been deprecated and replaced by the name field.
    #[prost(string, tag = "3")]
    pub cluster_id: std::string::String,
    /// Required. A description of the update.
    #[prost(message, optional, tag = "4")]
    pub update: ::std::option::Option<ClusterUpdate>,
    /// The name (project, location, cluster) of the cluster to update.
    /// Specified in the format `projects/*/locations/*/clusters/*`.
    #[prost(string, tag = "5")]
    pub name: std::string::String,
}
/// SetNodePoolVersionRequest updates the version of a node pool.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateNodePoolRequest {
    /// Required. Deprecated. The Google Developers Console [project ID or project
    /// number](https://support.google.com/cloud/answer/6158840).
    /// This field has been deprecated and replaced by the name field.
    #[prost(string, tag = "1")]
    pub project_id: std::string::String,
    /// Required. Deprecated. The name of the Google Compute Engine
    /// [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster
    /// resides.
    /// This field has been deprecated and replaced by the name field.
    #[prost(string, tag = "2")]
    pub zone: std::string::String,
    /// Required. Deprecated. The name of the cluster to upgrade.
    /// This field has been deprecated and replaced by the name field.
    #[prost(string, tag = "3")]
    pub cluster_id: std::string::String,
    /// Required. Deprecated. The name of the node pool to upgrade.
    /// This field has been deprecated and replaced by the name field.
    #[prost(string, tag = "4")]
    pub node_pool_id: std::string::String,
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
    pub node_version: std::string::String,
    /// Required. The desired image type for the node pool.
    #[prost(string, tag = "6")]
    pub image_type: std::string::String,
    /// The desired image type for the node pool.
    #[prost(message, optional, tag = "14")]
    pub workload_metadata_config: ::std::option::Option<WorkloadMetadataConfig>,
    /// The name (project, location, cluster, node pool) of the node pool to
    /// update. Specified in the format
    /// `projects/*/locations/*/clusters/*/nodePools/*`.
    #[prost(string, tag = "8")]
    pub name: std::string::String,
}
/// SetNodePoolAutoscalingRequest sets the autoscaler settings of a node pool.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetNodePoolAutoscalingRequest {
    /// Required. Deprecated. The Google Developers Console [project ID or project
    /// number](https://support.google.com/cloud/answer/6158840).
    /// This field has been deprecated and replaced by the name field.
    #[prost(string, tag = "1")]
    pub project_id: std::string::String,
    /// Required. Deprecated. The name of the Google Compute Engine
    /// [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster
    /// resides.
    /// This field has been deprecated and replaced by the name field.
    #[prost(string, tag = "2")]
    pub zone: std::string::String,
    /// Required. Deprecated. The name of the cluster to upgrade.
    /// This field has been deprecated and replaced by the name field.
    #[prost(string, tag = "3")]
    pub cluster_id: std::string::String,
    /// Required. Deprecated. The name of the node pool to upgrade.
    /// This field has been deprecated and replaced by the name field.
    #[prost(string, tag = "4")]
    pub node_pool_id: std::string::String,
    /// Required. Autoscaling configuration for the node pool.
    #[prost(message, optional, tag = "5")]
    pub autoscaling: ::std::option::Option<NodePoolAutoscaling>,
    /// The name (project, location, cluster, node pool) of the node pool to set
    /// autoscaler settings. Specified in the format
    /// `projects/*/locations/*/clusters/*/nodePools/*`.
    #[prost(string, tag = "6")]
    pub name: std::string::String,
}
/// SetLoggingServiceRequest sets the logging service of a cluster.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetLoggingServiceRequest {
    /// Required. Deprecated. The Google Developers Console [project ID or project
    /// number](https://support.google.com/cloud/answer/6158840).
    /// This field has been deprecated and replaced by the name field.
    #[prost(string, tag = "1")]
    pub project_id: std::string::String,
    /// Required. Deprecated. The name of the Google Compute Engine
    /// [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster
    /// resides.
    /// This field has been deprecated and replaced by the name field.
    #[prost(string, tag = "2")]
    pub zone: std::string::String,
    /// Required. Deprecated. The name of the cluster to upgrade.
    /// This field has been deprecated and replaced by the name field.
    #[prost(string, tag = "3")]
    pub cluster_id: std::string::String,
    /// Required. The logging service the cluster should use to write metrics.
    /// Currently available options:
    ///
    /// * "logging.googleapis.com" - the Google Cloud Logging service
    /// * "none" - no metrics will be exported from the cluster
    #[prost(string, tag = "4")]
    pub logging_service: std::string::String,
    /// The name (project, location, cluster) of the cluster to set logging.
    /// Specified in the format `projects/*/locations/*/clusters/*`.
    #[prost(string, tag = "5")]
    pub name: std::string::String,
}
/// SetMonitoringServiceRequest sets the monitoring service of a cluster.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetMonitoringServiceRequest {
    /// Required. Deprecated. The Google Developers Console [project ID or project
    /// number](https://support.google.com/cloud/answer/6158840).
    /// This field has been deprecated and replaced by the name field.
    #[prost(string, tag = "1")]
    pub project_id: std::string::String,
    /// Required. Deprecated. The name of the Google Compute Engine
    /// [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster
    /// resides.
    /// This field has been deprecated and replaced by the name field.
    #[prost(string, tag = "2")]
    pub zone: std::string::String,
    /// Required. Deprecated. The name of the cluster to upgrade.
    /// This field has been deprecated and replaced by the name field.
    #[prost(string, tag = "3")]
    pub cluster_id: std::string::String,
    /// Required. The monitoring service the cluster should use to write metrics.
    /// Currently available options:
    ///
    /// * "monitoring.googleapis.com" - the Google Cloud Monitoring service
    /// * "none" - no metrics will be exported from the cluster
    #[prost(string, tag = "4")]
    pub monitoring_service: std::string::String,
    /// The name (project, location, cluster) of the cluster to set monitoring.
    /// Specified in the format `projects/*/locations/*/clusters/*`.
    #[prost(string, tag = "6")]
    pub name: std::string::String,
}
/// SetAddonsRequest sets the addons associated with the cluster.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetAddonsConfigRequest {
    /// Required. Deprecated. The Google Developers Console [project ID or project
    /// number](https://support.google.com/cloud/answer/6158840).
    /// This field has been deprecated and replaced by the name field.
    #[prost(string, tag = "1")]
    pub project_id: std::string::String,
    /// Required. Deprecated. The name of the Google Compute Engine
    /// [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster
    /// resides.
    /// This field has been deprecated and replaced by the name field.
    #[prost(string, tag = "2")]
    pub zone: std::string::String,
    /// Required. Deprecated. The name of the cluster to upgrade.
    /// This field has been deprecated and replaced by the name field.
    #[prost(string, tag = "3")]
    pub cluster_id: std::string::String,
    /// Required. The desired configurations for the various addons available to run in the
    /// cluster.
    #[prost(message, optional, tag = "4")]
    pub addons_config: ::std::option::Option<AddonsConfig>,
    /// The name (project, location, cluster) of the cluster to set addons.
    /// Specified in the format `projects/*/locations/*/clusters/*`.
    #[prost(string, tag = "6")]
    pub name: std::string::String,
}
/// SetLocationsRequest sets the locations of the cluster.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetLocationsRequest {
    /// Required. Deprecated. The Google Developers Console [project ID or project
    /// number](https://support.google.com/cloud/answer/6158840).
    /// This field has been deprecated and replaced by the name field.
    #[prost(string, tag = "1")]
    pub project_id: std::string::String,
    /// Required. Deprecated. The name of the Google Compute Engine
    /// [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster
    /// resides.
    /// This field has been deprecated and replaced by the name field.
    #[prost(string, tag = "2")]
    pub zone: std::string::String,
    /// Required. Deprecated. The name of the cluster to upgrade.
    /// This field has been deprecated and replaced by the name field.
    #[prost(string, tag = "3")]
    pub cluster_id: std::string::String,
    /// Required. The desired list of Google Compute Engine
    /// [zones](https://cloud.google.com/compute/docs/zones#available) in which the cluster's nodes
    /// should be located. Changing the locations a cluster is in will result
    /// in nodes being either created or removed from the cluster, depending on
    /// whether locations are being added or removed.
    ///
    /// This list must always include the cluster's primary zone.
    #[prost(string, repeated, tag = "4")]
    pub locations: ::std::vec::Vec<std::string::String>,
    /// The name (project, location, cluster) of the cluster to set locations.
    /// Specified in the format `projects/*/locations/*/clusters/*`.
    #[prost(string, tag = "6")]
    pub name: std::string::String,
}
/// UpdateMasterRequest updates the master of the cluster.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateMasterRequest {
    /// Required. Deprecated. The Google Developers Console [project ID or project
    /// number](https://support.google.com/cloud/answer/6158840).
    /// This field has been deprecated and replaced by the name field.
    #[prost(string, tag = "1")]
    pub project_id: std::string::String,
    /// Required. Deprecated. The name of the Google Compute Engine
    /// [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster
    /// resides.
    /// This field has been deprecated and replaced by the name field.
    #[prost(string, tag = "2")]
    pub zone: std::string::String,
    /// Required. Deprecated. The name of the cluster to upgrade.
    /// This field has been deprecated and replaced by the name field.
    #[prost(string, tag = "3")]
    pub cluster_id: std::string::String,
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
    pub master_version: std::string::String,
    /// The name (project, location, cluster) of the cluster to update.
    /// Specified in the format `projects/*/locations/*/clusters/*`.
    #[prost(string, tag = "7")]
    pub name: std::string::String,
}
/// SetMasterAuthRequest updates the admin password of a cluster.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetMasterAuthRequest {
    /// Required. Deprecated. The Google Developers Console [project ID or project
    /// number](https://support.google.com/cloud/answer/6158840).
    /// This field has been deprecated and replaced by the name field.
    #[prost(string, tag = "1")]
    pub project_id: std::string::String,
    /// Required. Deprecated. The name of the Google Compute Engine
    /// [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster
    /// resides.
    /// This field has been deprecated and replaced by the name field.
    #[prost(string, tag = "2")]
    pub zone: std::string::String,
    /// Required. Deprecated. The name of the cluster to upgrade.
    /// This field has been deprecated and replaced by the name field.
    #[prost(string, tag = "3")]
    pub cluster_id: std::string::String,
    /// Required. The exact form of action to be taken on the master auth.
    #[prost(enumeration = "set_master_auth_request::Action", tag = "4")]
    pub action: i32,
    /// Required. A description of the update.
    #[prost(message, optional, tag = "5")]
    pub update: ::std::option::Option<MasterAuth>,
    /// The name (project, location, cluster) of the cluster to set auth.
    /// Specified in the format `projects/*/locations/*/clusters/*`.
    #[prost(string, tag = "7")]
    pub name: std::string::String,
}
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
    /// Required. Deprecated. The Google Developers Console [project ID or project
    /// number](https://support.google.com/cloud/answer/6158840).
    /// This field has been deprecated and replaced by the name field.
    #[prost(string, tag = "1")]
    pub project_id: std::string::String,
    /// Required. Deprecated. The name of the Google Compute Engine
    /// [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster
    /// resides.
    /// This field has been deprecated and replaced by the name field.
    #[prost(string, tag = "2")]
    pub zone: std::string::String,
    /// Required. Deprecated. The name of the cluster to delete.
    /// This field has been deprecated and replaced by the name field.
    #[prost(string, tag = "3")]
    pub cluster_id: std::string::String,
    /// The name (project, location, cluster) of the cluster to delete.
    /// Specified in the format `projects/*/locations/*/clusters/*`.
    #[prost(string, tag = "4")]
    pub name: std::string::String,
}
/// ListClustersRequest lists clusters.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListClustersRequest {
    /// Required. Deprecated. The Google Developers Console [project ID or project
    /// number](https://support.google.com/cloud/answer/6158840).
    /// This field has been deprecated and replaced by the parent field.
    #[prost(string, tag = "1")]
    pub project_id: std::string::String,
    /// Required. Deprecated. The name of the Google Compute Engine
    /// [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster
    /// resides, or "-" for all zones.
    /// This field has been deprecated and replaced by the parent field.
    #[prost(string, tag = "2")]
    pub zone: std::string::String,
    /// The parent (project and location) where the clusters will be listed.
    /// Specified in the format `projects/*/locations/*`.
    /// Location "-" matches all zones and all regions.
    #[prost(string, tag = "4")]
    pub parent: std::string::String,
}
/// ListClustersResponse is the result of ListClustersRequest.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListClustersResponse {
    /// A list of clusters in the project in the specified zone, or
    /// across all ones.
    #[prost(message, repeated, tag = "1")]
    pub clusters: ::std::vec::Vec<Cluster>,
    /// If any zones are listed here, the list of clusters returned
    /// may be missing those zones.
    #[prost(string, repeated, tag = "2")]
    pub missing_zones: ::std::vec::Vec<std::string::String>,
}
/// GetOperationRequest gets a single operation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetOperationRequest {
    /// Required. Deprecated. The Google Developers Console [project ID or project
    /// number](https://support.google.com/cloud/answer/6158840).
    /// This field has been deprecated and replaced by the name field.
    #[prost(string, tag = "1")]
    pub project_id: std::string::String,
    /// Required. Deprecated. The name of the Google Compute Engine
    /// [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster
    /// resides.
    /// This field has been deprecated and replaced by the name field.
    #[prost(string, tag = "2")]
    pub zone: std::string::String,
    /// Required. Deprecated. The server-assigned `name` of the operation.
    /// This field has been deprecated and replaced by the name field.
    #[prost(string, tag = "3")]
    pub operation_id: std::string::String,
    /// The name (project, location, operation id) of the operation to get.
    /// Specified in the format `projects/*/locations/*/operations/*`.
    #[prost(string, tag = "5")]
    pub name: std::string::String,
}
/// ListOperationsRequest lists operations.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListOperationsRequest {
    /// Required. Deprecated. The Google Developers Console [project ID or project
    /// number](https://support.google.com/cloud/answer/6158840).
    /// This field has been deprecated and replaced by the parent field.
    #[prost(string, tag = "1")]
    pub project_id: std::string::String,
    /// Required. Deprecated. The name of the Google Compute Engine
    /// [zone](https://cloud.google.com/compute/docs/zones#available) to return operations for, or `-` for
    /// all zones. This field has been deprecated and replaced by the parent field.
    #[prost(string, tag = "2")]
    pub zone: std::string::String,
    /// The parent (project and location) where the operations will be listed.
    /// Specified in the format `projects/*/locations/*`.
    /// Location "-" matches all zones and all regions.
    #[prost(string, tag = "4")]
    pub parent: std::string::String,
}
/// CancelOperationRequest cancels a single operation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CancelOperationRequest {
    /// Required. Deprecated. The Google Developers Console [project ID or project
    /// number](https://support.google.com/cloud/answer/6158840).
    /// This field has been deprecated and replaced by the name field.
    #[prost(string, tag = "1")]
    pub project_id: std::string::String,
    /// Required. Deprecated. The name of the Google Compute Engine
    /// [zone](https://cloud.google.com/compute/docs/zones#available) in which the operation resides.
    /// This field has been deprecated and replaced by the name field.
    #[prost(string, tag = "2")]
    pub zone: std::string::String,
    /// Required. Deprecated. The server-assigned `name` of the operation.
    /// This field has been deprecated and replaced by the name field.
    #[prost(string, tag = "3")]
    pub operation_id: std::string::String,
    /// The name (project, location, operation id) of the operation to cancel.
    /// Specified in the format `projects/*/locations/*/operations/*`.
    #[prost(string, tag = "4")]
    pub name: std::string::String,
}
/// ListOperationsResponse is the result of ListOperationsRequest.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListOperationsResponse {
    /// A list of operations in the project in the specified zone.
    #[prost(message, repeated, tag = "1")]
    pub operations: ::std::vec::Vec<Operation>,
    /// If any zones are listed here, the list of operations returned
    /// may be missing the operations from those zones.
    #[prost(string, repeated, tag = "2")]
    pub missing_zones: ::std::vec::Vec<std::string::String>,
}
/// Gets the current Kubernetes Engine service configuration.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetServerConfigRequest {
    /// Required. Deprecated. The Google Developers Console [project ID or project
    /// number](https://support.google.com/cloud/answer/6158840).
    /// This field has been deprecated and replaced by the name field.
    #[prost(string, tag = "1")]
    pub project_id: std::string::String,
    /// Required. Deprecated. The name of the Google Compute Engine
    /// [zone](https://cloud.google.com/compute/docs/zones#available) to return operations for.
    /// This field has been deprecated and replaced by the name field.
    #[prost(string, tag = "2")]
    pub zone: std::string::String,
    /// The name (project and location) of the server config to get,
    /// specified in the format `projects/*/locations/*`.
    #[prost(string, tag = "4")]
    pub name: std::string::String,
}
/// Kubernetes Engine service configuration.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ServerConfig {
    /// Version of Kubernetes the service deploys by default.
    #[prost(string, tag = "1")]
    pub default_cluster_version: std::string::String,
    /// List of valid node upgrade target versions.
    #[prost(string, repeated, tag = "3")]
    pub valid_node_versions: ::std::vec::Vec<std::string::String>,
    /// Default image type.
    #[prost(string, tag = "4")]
    pub default_image_type: std::string::String,
    /// List of valid image types.
    #[prost(string, repeated, tag = "5")]
    pub valid_image_types: ::std::vec::Vec<std::string::String>,
    /// List of valid master versions.
    #[prost(string, repeated, tag = "6")]
    pub valid_master_versions: ::std::vec::Vec<std::string::String>,
}
/// CreateNodePoolRequest creates a node pool for a cluster.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateNodePoolRequest {
    /// Required. Deprecated. The Google Developers Console [project ID or project
    /// number](https://developers.google.com/console/help/new/#projectnumber).
    /// This field has been deprecated and replaced by the parent field.
    #[prost(string, tag = "1")]
    pub project_id: std::string::String,
    /// Required. Deprecated. The name of the Google Compute Engine
    /// [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster
    /// resides.
    /// This field has been deprecated and replaced by the parent field.
    #[prost(string, tag = "2")]
    pub zone: std::string::String,
    /// Required. Deprecated. The name of the cluster.
    /// This field has been deprecated and replaced by the parent field.
    #[prost(string, tag = "3")]
    pub cluster_id: std::string::String,
    /// Required. The node pool to create.
    #[prost(message, optional, tag = "4")]
    pub node_pool: ::std::option::Option<NodePool>,
    /// The parent (project, location, cluster id) where the node pool will be
    /// created. Specified in the format
    /// `projects/*/locations/*/clusters/*`.
    #[prost(string, tag = "6")]
    pub parent: std::string::String,
}
/// DeleteNodePoolRequest deletes a node pool for a cluster.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteNodePoolRequest {
    /// Required. Deprecated. The Google Developers Console [project ID or project
    /// number](https://developers.google.com/console/help/new/#projectnumber).
    /// This field has been deprecated and replaced by the name field.
    #[prost(string, tag = "1")]
    pub project_id: std::string::String,
    /// Required. Deprecated. The name of the Google Compute Engine
    /// [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster
    /// resides.
    /// This field has been deprecated and replaced by the name field.
    #[prost(string, tag = "2")]
    pub zone: std::string::String,
    /// Required. Deprecated. The name of the cluster.
    /// This field has been deprecated and replaced by the name field.
    #[prost(string, tag = "3")]
    pub cluster_id: std::string::String,
    /// Required. Deprecated. The name of the node pool to delete.
    /// This field has been deprecated and replaced by the name field.
    #[prost(string, tag = "4")]
    pub node_pool_id: std::string::String,
    /// The name (project, location, cluster, node pool id) of the node pool to
    /// delete. Specified in the format
    /// `projects/*/locations/*/clusters/*/nodePools/*`.
    #[prost(string, tag = "6")]
    pub name: std::string::String,
}
/// ListNodePoolsRequest lists the node pool(s) for a cluster.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListNodePoolsRequest {
    /// Required. Deprecated. The Google Developers Console [project ID or project
    /// number](https://developers.google.com/console/help/new/#projectnumber).
    /// This field has been deprecated and replaced by the parent field.
    #[prost(string, tag = "1")]
    pub project_id: std::string::String,
    /// Required. Deprecated. The name of the Google Compute Engine
    /// [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster
    /// resides.
    /// This field has been deprecated and replaced by the parent field.
    #[prost(string, tag = "2")]
    pub zone: std::string::String,
    /// Required. Deprecated. The name of the cluster.
    /// This field has been deprecated and replaced by the parent field.
    #[prost(string, tag = "3")]
    pub cluster_id: std::string::String,
    /// The parent (project, location, cluster id) where the node pools will be
    /// listed. Specified in the format `projects/*/locations/*/clusters/*`.
    #[prost(string, tag = "5")]
    pub parent: std::string::String,
}
/// GetNodePoolRequest retrieves a node pool for a cluster.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetNodePoolRequest {
    /// Required. Deprecated. The Google Developers Console [project ID or project
    /// number](https://developers.google.com/console/help/new/#projectnumber).
    /// This field has been deprecated and replaced by the name field.
    #[prost(string, tag = "1")]
    pub project_id: std::string::String,
    /// Required. Deprecated. The name of the Google Compute Engine
    /// [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster
    /// resides.
    /// This field has been deprecated and replaced by the name field.
    #[prost(string, tag = "2")]
    pub zone: std::string::String,
    /// Required. Deprecated. The name of the cluster.
    /// This field has been deprecated and replaced by the name field.
    #[prost(string, tag = "3")]
    pub cluster_id: std::string::String,
    /// Required. Deprecated. The name of the node pool.
    /// This field has been deprecated and replaced by the name field.
    #[prost(string, tag = "4")]
    pub node_pool_id: std::string::String,
    /// The name (project, location, cluster, node pool id) of the node pool to
    /// get. Specified in the format
    /// `projects/*/locations/*/clusters/*/nodePools/*`.
    #[prost(string, tag = "6")]
    pub name: std::string::String,
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
    pub name: std::string::String,
    /// The node configuration of the pool.
    #[prost(message, optional, tag = "2")]
    pub config: ::std::option::Option<NodeConfig>,
    /// The initial node count for the pool. You must ensure that your
    /// Compute Engine [resource quota](https://cloud.google.com/compute/quotas)
    /// is sufficient for this number of instances. You must also have available
    /// firewall and routes quota.
    #[prost(int32, tag = "3")]
    pub initial_node_count: i32,
    /// [Output only] Server-defined URL for the resource.
    #[prost(string, tag = "100")]
    pub self_link: std::string::String,
    /// The version of the Kubernetes of this node.
    #[prost(string, tag = "101")]
    pub version: std::string::String,
    /// [Output only] The resource URLs of the [managed instance
    /// groups](https://cloud.google.com/compute/docs/instance-groups/creating-groups-of-managed-instances)
    /// associated with this node pool.
    #[prost(string, repeated, tag = "102")]
    pub instance_group_urls: ::std::vec::Vec<std::string::String>,
    /// [Output only] The status of the nodes in this pool instance.
    #[prost(enumeration = "node_pool::Status", tag = "103")]
    pub status: i32,
    /// [Output only] Additional information about the current status of this
    /// node pool instance, if available.
    #[prost(string, tag = "104")]
    pub status_message: std::string::String,
    /// Autoscaler configuration for this NodePool. Autoscaler is enabled
    /// only if a valid configuration is present.
    #[prost(message, optional, tag = "4")]
    pub autoscaling: ::std::option::Option<NodePoolAutoscaling>,
    /// NodeManagement configuration for this NodePool.
    #[prost(message, optional, tag = "5")]
    pub management: ::std::option::Option<NodeManagement>,
    /// The constraint on the maximum number of pods that can be run
    /// simultaneously on a node in the node pool.
    #[prost(message, optional, tag = "6")]
    pub max_pods_constraint: ::std::option::Option<MaxPodsConstraint>,
    /// Which conditions caused the current node pool state.
    #[prost(message, repeated, tag = "105")]
    pub conditions: ::std::vec::Vec<StatusCondition>,
    /// [Output only] The pod CIDR block size per node in this node pool.
    #[prost(int32, tag = "7")]
    pub pod_ipv4_cidr_size: i32,
}
pub mod node_pool {
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
    /// Whether the nodes will be automatically upgraded.
    #[prost(bool, tag = "1")]
    pub auto_upgrade: bool,
    /// Whether the nodes will be automatically repaired.
    #[prost(bool, tag = "2")]
    pub auto_repair: bool,
    /// Specifies the Auto Upgrade knobs for the node pool.
    #[prost(message, optional, tag = "10")]
    pub upgrade_options: ::std::option::Option<AutoUpgradeOptions>,
}
/// AutoUpgradeOptions defines the set of options for the user to control how
/// the Auto Upgrades will proceed.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AutoUpgradeOptions {
    /// [Output only] This field is set when upgrades are about to commence
    /// with the approximate start time for the upgrades, in
    /// [RFC3339](https://www.ietf.org/rfc/rfc3339.txt) text format.
    #[prost(string, tag = "1")]
    pub auto_upgrade_start_time: std::string::String,
    /// [Output only] This field is set when upgrades are about to commence
    /// with the description of the upgrade.
    #[prost(string, tag = "2")]
    pub description: std::string::String,
}
/// MaintenancePolicy defines the maintenance policy to be used for the cluster.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MaintenancePolicy {
    /// Specifies the maintenance window in which maintenance may be performed.
    #[prost(message, optional, tag = "1")]
    pub window: ::std::option::Option<MaintenanceWindow>,
    /// A hash identifying the version of this policy, so that updates to fields of
    /// the policy won't accidentally undo intermediate changes (and so that users
    /// of the API unaware of some fields won't accidentally remove other fields).
    /// Make a <code>get()</code> request to the cluster to get the current
    /// resource version and include it with requests to set the policy.
    #[prost(string, tag = "3")]
    pub resource_version: std::string::String,
}
/// MaintenanceWindow defines the maintenance window to be used for the cluster.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MaintenanceWindow {
    /// Exceptions to maintenance window. Non-emergency maintenance should not
    /// occur in these windows.
    #[prost(map = "string, message", tag = "4")]
    pub maintenance_exclusions: ::std::collections::HashMap<std::string::String, TimeWindow>,
    /// Unimplemented, reserved for future use.
    /// HourlyMaintenanceWindow hourly_maintenance_window = 1;
    #[prost(oneof = "maintenance_window::Policy", tags = "2, 3")]
    pub policy: ::std::option::Option<maintenance_window::Policy>,
}
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
    pub start_time: ::std::option::Option<::prost_types::Timestamp>,
    /// The time that the window ends. The end time should take place after the
    /// start time.
    #[prost(message, optional, tag = "2")]
    pub end_time: ::std::option::Option<::prost_types::Timestamp>,
}
/// Represents an arbitrary window of time that recurs.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RecurringTimeWindow {
    /// The window of the first recurrence.
    #[prost(message, optional, tag = "1")]
    pub window: ::std::option::Option<TimeWindow>,
    /// An RRULE (https://tools.ietf.org/html/rfc5545#section-3.8.5.3) for how
    /// this window reccurs. They go on for the span of time between the start and
    /// end time.
    ///
    /// For example, to have something repeat every weekday, you'd use:
    ///   <code>FREQ=WEEKLY;BYDAY=MO,TU,WE,TH,FR</code>
    /// To repeat some window daily (equivalent to the DailyMaintenanceWindow):
    ///   <code>FREQ=DAILY</code>
    /// For the first weekend of every month:
    ///   <code>FREQ=MONTHLY;BYSETPOS=1;BYDAY=SA,SU</code>
    /// This specifies how frequently the window starts. Eg, if you wanted to have
    /// a 9-5 UTC-4 window every weekday, you'd use something like:
    /// <code>
    ///   start time = 2019-01-01T09:00:00-0400
    ///   end time = 2019-01-01T17:00:00-0400
    ///   recurrence = FREQ=WEEKLY;BYDAY=MO,TU,WE,TH,FR
    /// </code>
    /// Windows can span multiple days. Eg, to make the window encompass every
    /// weekend from midnight Saturday till the last minute of Sunday UTC:
    /// <code>
    ///   start time = 2019-01-05T00:00:00Z
    ///   end time = 2019-01-07T23:59:00Z
    ///   recurrence = FREQ=WEEKLY;BYDAY=SA
    /// </code>
    /// Note the start and end time's specific dates are largely arbitrary except
    /// to specify duration of the window and when it first starts.
    /// The FREQ values of HOURLY, MINUTELY, and SECONDLY are not supported.
    #[prost(string, tag = "2")]
    pub recurrence: std::string::String,
}
/// Time window specified for daily maintenance operations.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DailyMaintenanceWindow {
    /// Time within the maintenance window to start the maintenance operations.
    /// It must be in format "HH:MM", where HH : [00-23] and MM : [00-59] GMT.
    #[prost(string, tag = "2")]
    pub start_time: std::string::String,
    /// [Output only] Duration of the time window, automatically chosen to be
    /// smallest possible in the given scenario.
    #[prost(string, tag = "3")]
    pub duration: std::string::String,
}
/// SetNodePoolManagementRequest sets the node management properties of a node
/// pool.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetNodePoolManagementRequest {
    /// Required. Deprecated. The Google Developers Console [project ID or project
    /// number](https://support.google.com/cloud/answer/6158840).
    /// This field has been deprecated and replaced by the name field.
    #[prost(string, tag = "1")]
    pub project_id: std::string::String,
    /// Required. Deprecated. The name of the Google Compute Engine
    /// [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster
    /// resides.
    /// This field has been deprecated and replaced by the name field.
    #[prost(string, tag = "2")]
    pub zone: std::string::String,
    /// Required. Deprecated. The name of the cluster to update.
    /// This field has been deprecated and replaced by the name field.
    #[prost(string, tag = "3")]
    pub cluster_id: std::string::String,
    /// Required. Deprecated. The name of the node pool to update.
    /// This field has been deprecated and replaced by the name field.
    #[prost(string, tag = "4")]
    pub node_pool_id: std::string::String,
    /// Required. NodeManagement configuration for the node pool.
    #[prost(message, optional, tag = "5")]
    pub management: ::std::option::Option<NodeManagement>,
    /// The name (project, location, cluster, node pool id) of the node pool to set
    /// management properties. Specified in the format
    /// `projects/*/locations/*/clusters/*/nodePools/*`.
    #[prost(string, tag = "7")]
    pub name: std::string::String,
}
/// SetNodePoolSizeRequest sets the size a node
/// pool.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetNodePoolSizeRequest {
    /// Required. Deprecated. The Google Developers Console [project ID or project
    /// number](https://support.google.com/cloud/answer/6158840).
    /// This field has been deprecated and replaced by the name field.
    #[prost(string, tag = "1")]
    pub project_id: std::string::String,
    /// Required. Deprecated. The name of the Google Compute Engine
    /// [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster
    /// resides.
    /// This field has been deprecated and replaced by the name field.
    #[prost(string, tag = "2")]
    pub zone: std::string::String,
    /// Required. Deprecated. The name of the cluster to update.
    /// This field has been deprecated and replaced by the name field.
    #[prost(string, tag = "3")]
    pub cluster_id: std::string::String,
    /// Required. Deprecated. The name of the node pool to update.
    /// This field has been deprecated and replaced by the name field.
    #[prost(string, tag = "4")]
    pub node_pool_id: std::string::String,
    /// Required. The desired node count for the pool.
    #[prost(int32, tag = "5")]
    pub node_count: i32,
    /// The name (project, location, cluster, node pool id) of the node pool to set
    /// size.
    /// Specified in the format `projects/*/locations/*/clusters/*/nodePools/*`.
    #[prost(string, tag = "7")]
    pub name: std::string::String,
}
/// RollbackNodePoolUpgradeRequest rollbacks the previously Aborted or Failed
/// NodePool upgrade. This will be an no-op if the last upgrade successfully
/// completed.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RollbackNodePoolUpgradeRequest {
    /// Required. Deprecated. The Google Developers Console [project ID or project
    /// number](https://support.google.com/cloud/answer/6158840).
    /// This field has been deprecated and replaced by the name field.
    #[prost(string, tag = "1")]
    pub project_id: std::string::String,
    /// Required. Deprecated. The name of the Google Compute Engine
    /// [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster
    /// resides.
    /// This field has been deprecated and replaced by the name field.
    #[prost(string, tag = "2")]
    pub zone: std::string::String,
    /// Required. Deprecated. The name of the cluster to rollback.
    /// This field has been deprecated and replaced by the name field.
    #[prost(string, tag = "3")]
    pub cluster_id: std::string::String,
    /// Required. Deprecated. The name of the node pool to rollback.
    /// This field has been deprecated and replaced by the name field.
    #[prost(string, tag = "4")]
    pub node_pool_id: std::string::String,
    /// The name (project, location, cluster, node pool id) of the node poll to
    /// rollback upgrade.
    /// Specified in the format `projects/*/locations/*/clusters/*/nodePools/*`.
    #[prost(string, tag = "6")]
    pub name: std::string::String,
}
/// ListNodePoolsResponse is the result of ListNodePoolsRequest.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListNodePoolsResponse {
    /// A list of node pools for a cluster.
    #[prost(message, repeated, tag = "1")]
    pub node_pools: ::std::vec::Vec<NodePool>,
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
    pub resource_limits: ::std::vec::Vec<ResourceLimit>,
    /// AutoprovisioningNodePoolDefaults contains defaults for a node pool
    /// created by NAP.
    #[prost(message, optional, tag = "4")]
    pub autoprovisioning_node_pool_defaults:
        ::std::option::Option<AutoprovisioningNodePoolDefaults>,
    /// The list of Google Compute Engine [zones](https://cloud.google.com/compute/docs/zones#available)
    /// in which the NodePool's nodes can be created by NAP.
    #[prost(string, repeated, tag = "5")]
    pub autoprovisioning_locations: ::std::vec::Vec<std::string::String>,
}
/// AutoprovisioningNodePoolDefaults contains defaults for a node pool created
/// by NAP.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AutoprovisioningNodePoolDefaults {
    /// Scopes that are used by NAP when creating node pools. If oauth_scopes are
    /// specified, service_account should be empty.
    #[prost(string, repeated, tag = "1")]
    pub oauth_scopes: ::std::vec::Vec<std::string::String>,
    /// The Google Cloud Platform Service Account to be used by the node VMs. If
    /// service_account is specified, scopes should be empty.
    #[prost(string, tag = "2")]
    pub service_account: std::string::String,
}
/// Contains information about amount of some resource in the cluster.
/// For memory, value should be in GB.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResourceLimit {
    /// Resource name "cpu", "memory" or gpu-specific string.
    #[prost(string, tag = "1")]
    pub resource_type: std::string::String,
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
    /// Required. Deprecated. The Google Developers Console [project ID or project
    /// number](https://developers.google.com/console/help/new/#projectnumber).
    /// This field has been deprecated and replaced by the name field.
    #[prost(string, tag = "1")]
    pub project_id: std::string::String,
    /// Required. Deprecated. The name of the Google Compute Engine
    /// [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster
    /// resides.
    /// This field has been deprecated and replaced by the name field.
    #[prost(string, tag = "2")]
    pub zone: std::string::String,
    /// Required. Deprecated. The name of the cluster.
    /// This field has been deprecated and replaced by the name field.
    #[prost(string, tag = "3")]
    pub cluster_id: std::string::String,
    /// Required. The labels to set for that cluster.
    #[prost(map = "string, string", tag = "4")]
    pub resource_labels: ::std::collections::HashMap<std::string::String, std::string::String>,
    /// Required. The fingerprint of the previous set of labels for this resource,
    /// used to detect conflicts. The fingerprint is initially generated by
    /// Kubernetes Engine and changes after every request to modify or update
    /// labels. You must always provide an up-to-date fingerprint hash when
    /// updating or changing labels. Make a <code>get()</code> request to the
    /// resource to get the latest fingerprint.
    #[prost(string, tag = "5")]
    pub label_fingerprint: std::string::String,
    /// The name (project, location, cluster id) of the cluster to set labels.
    /// Specified in the format `projects/*/locations/*/clusters/*`.
    #[prost(string, tag = "7")]
    pub name: std::string::String,
}
/// SetLegacyAbacRequest enables or disables the ABAC authorization mechanism for
/// a cluster.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetLegacyAbacRequest {
    /// Required. Deprecated. The Google Developers Console [project ID or project
    /// number](https://support.google.com/cloud/answer/6158840).
    /// This field has been deprecated and replaced by the name field.
    #[prost(string, tag = "1")]
    pub project_id: std::string::String,
    /// Required. Deprecated. The name of the Google Compute Engine
    /// [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster
    /// resides.
    /// This field has been deprecated and replaced by the name field.
    #[prost(string, tag = "2")]
    pub zone: std::string::String,
    /// Required. Deprecated. The name of the cluster to update.
    /// This field has been deprecated and replaced by the name field.
    #[prost(string, tag = "3")]
    pub cluster_id: std::string::String,
    /// Required. Whether ABAC authorization will be enabled in the cluster.
    #[prost(bool, tag = "4")]
    pub enabled: bool,
    /// The name (project, location, cluster id) of the cluster to set legacy abac.
    /// Specified in the format `projects/*/locations/*/clusters/*`.
    #[prost(string, tag = "6")]
    pub name: std::string::String,
}
/// StartIPRotationRequest creates a new IP for the cluster and then performs
/// a node upgrade on each node pool to point to the new IP.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StartIpRotationRequest {
    /// Required. Deprecated. The Google Developers Console [project ID or project
    /// number](https://developers.google.com/console/help/new/#projectnumber).
    /// This field has been deprecated and replaced by the name field.
    #[prost(string, tag = "1")]
    pub project_id: std::string::String,
    /// Required. Deprecated. The name of the Google Compute Engine
    /// [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster
    /// resides.
    /// This field has been deprecated and replaced by the name field.
    #[prost(string, tag = "2")]
    pub zone: std::string::String,
    /// Required. Deprecated. The name of the cluster.
    /// This field has been deprecated and replaced by the name field.
    #[prost(string, tag = "3")]
    pub cluster_id: std::string::String,
    /// The name (project, location, cluster id) of the cluster to start IP
    /// rotation. Specified in the format `projects/*/locations/*/clusters/*`.
    #[prost(string, tag = "6")]
    pub name: std::string::String,
    /// Whether to rotate credentials during IP rotation.
    #[prost(bool, tag = "7")]
    pub rotate_credentials: bool,
}
/// CompleteIPRotationRequest moves the cluster master back into single-IP mode.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CompleteIpRotationRequest {
    /// Required. Deprecated. The Google Developers Console [project ID or project
    /// number](https://developers.google.com/console/help/new/#projectnumber).
    /// This field has been deprecated and replaced by the name field.
    #[prost(string, tag = "1")]
    pub project_id: std::string::String,
    /// Required. Deprecated. The name of the Google Compute Engine
    /// [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster
    /// resides.
    /// This field has been deprecated and replaced by the name field.
    #[prost(string, tag = "2")]
    pub zone: std::string::String,
    /// Required. Deprecated. The name of the cluster.
    /// This field has been deprecated and replaced by the name field.
    #[prost(string, tag = "3")]
    pub cluster_id: std::string::String,
    /// The name (project, location, cluster id) of the cluster to complete IP
    /// rotation. Specified in the format `projects/*/locations/*/clusters/*`.
    #[prost(string, tag = "7")]
    pub name: std::string::String,
}
/// AcceleratorConfig represents a Hardware Accelerator request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AcceleratorConfig {
    /// The number of the accelerator cards exposed to an instance.
    #[prost(int64, tag = "1")]
    pub accelerator_count: i64,
    /// The accelerator type resource name. List of supported accelerators
    /// [here](https://cloud.google.com/compute/docs/gpus)
    #[prost(string, tag = "2")]
    pub accelerator_type: std::string::String,
}
/// WorkloadMetadataConfig defines the metadata configuration to expose to
/// workloads on the node pool.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WorkloadMetadataConfig {
    /// NodeMetadata is the configuration for how to expose metadata to the
    /// workloads running on the node.
    #[prost(enumeration = "workload_metadata_config::NodeMetadata", tag = "1")]
    pub node_metadata: i32,
}
pub mod workload_metadata_config {
    /// NodeMetadata is the configuration for if and how to expose the node
    /// metadata to the workload running on the node.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
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
    }
}
/// SetNetworkPolicyRequest enables/disables network policy for a cluster.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetNetworkPolicyRequest {
    /// Required. Deprecated. The Google Developers Console [project ID or project
    /// number](https://developers.google.com/console/help/new/#projectnumber).
    /// This field has been deprecated and replaced by the name field.
    #[prost(string, tag = "1")]
    pub project_id: std::string::String,
    /// Required. Deprecated. The name of the Google Compute Engine
    /// [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster
    /// resides.
    /// This field has been deprecated and replaced by the name field.
    #[prost(string, tag = "2")]
    pub zone: std::string::String,
    /// Required. Deprecated. The name of the cluster.
    /// This field has been deprecated and replaced by the name field.
    #[prost(string, tag = "3")]
    pub cluster_id: std::string::String,
    /// Required. Configuration options for the NetworkPolicy feature.
    #[prost(message, optional, tag = "4")]
    pub network_policy: ::std::option::Option<NetworkPolicy>,
    /// The name (project, location, cluster id) of the cluster to set networking
    /// policy. Specified in the format `projects/*/locations/*/clusters/*`.
    #[prost(string, tag = "6")]
    pub name: std::string::String,
}
/// SetMaintenancePolicyRequest sets the maintenance policy for a cluster.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetMaintenancePolicyRequest {
    /// Required. The Google Developers Console [project ID or project
    /// number](https://support.google.com/cloud/answer/6158840).
    #[prost(string, tag = "1")]
    pub project_id: std::string::String,
    /// Required. The name of the Google Compute Engine
    /// [zone](https://cloud.google.com/compute/docs/zones#available) in which the cluster
    /// resides.
    #[prost(string, tag = "2")]
    pub zone: std::string::String,
    /// Required. The name of the cluster to update.
    #[prost(string, tag = "3")]
    pub cluster_id: std::string::String,
    /// Required. The maintenance policy to be set for the cluster. An empty field
    /// clears the existing maintenance policy.
    #[prost(message, optional, tag = "4")]
    pub maintenance_policy: ::std::option::Option<MaintenancePolicy>,
    /// The name (project, location, cluster id) of the cluster to set maintenance
    /// policy.
    /// Specified in the format `projects/*/locations/*/clusters/*`.
    #[prost(string, tag = "5")]
    pub name: std::string::String,
}
/// ListLocationsRequest is used to request the locations that offer GKE.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListLocationsRequest {
    /// Required. Contains the name of the resource requested.
    /// Specified in the format `projects/*`.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
}
/// ListLocationsResponse returns the list of all GKE locations and their
/// recommendation state.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListLocationsResponse {
    /// A full list of GKE locations.
    #[prost(message, repeated, tag = "1")]
    pub locations: ::std::vec::Vec<Location>,
    /// Only return ListLocationsResponse that occur after the page_token. This
    /// value should be populated from the ListLocationsResponse.next_page_token if
    /// that response token was set (which happens when listing more Locations than
    /// fit in a single ListLocationsResponse).
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
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
    pub name: std::string::String,
    /// Whether the location is recomended for GKE cluster scheduling.
    #[prost(bool, tag = "3")]
    pub recommended: bool,
}
pub mod location {
    /// LocationType is the type of GKE location, regional or zonal.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum LocationType {
        /// LOCATION_TYPE_UNSPECIFIED means the location type was not determined.
        Unspecified = 0,
        /// A GKE Location where Zonal clusters can be created.
        Zone = 1,
        /// A GKE Location where Regional clusters can be created.
        Region = 2,
    }
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
    pub message: std::string::String,
}
pub mod status_condition {
    /// Code for each condition
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Code {
        /// UNKNOWN indicates a generic condition.
        Unknown = 0,
        /// GCE_STOCKOUT indicates a Google Compute Engine stockout.
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
    /// [network][google.container.v1beta1.NetworkConfig.network](https://cloud.google.com/compute/docs/networks-and-firewalls#networks) to which
    /// the cluster is connected.
    /// Example: projects/my-project/global/networks/my-network
    #[prost(string, tag = "1")]
    pub network: std::string::String,
    /// Output only. The relative name of the Google Compute Engine
    /// [subnetwork](https://cloud.google.com/compute/docs/vpc) to which the cluster is connected.
    /// Example: projects/my-project/regions/us-central1/subnetworks/my-subnet
    #[prost(string, tag = "2")]
    pub subnetwork: std::string::String,
    /// Whether Intra-node visibility is enabled for this cluster.
    /// This makes same node pod to pod traffic visible for VPC network.
    #[prost(bool, tag = "5")]
    pub enable_intra_node_visibility: bool,
}
/// ListUsableSubnetworksRequest requests the list of usable subnetworks.
/// available to a user for creating clusters.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListUsableSubnetworksRequest {
    /// Required. The parent project where subnetworks are usable.
    /// Specified in the format `projects/*`.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Filtering currently only supports equality on the networkProjectId and must
    /// be in the form: "networkProjectId=[PROJECTID]", where `networkProjectId`
    /// is the project which owns the listed subnetworks. This defaults to the
    /// parent project ID.
    #[prost(string, tag = "2")]
    pub filter: std::string::String,
    /// The max number of results per page that should be returned. If the number
    /// of available results is larger than `page_size`, a `next_page_token` is
    /// returned which can be used to get the next page of results in subsequent
    /// requests. Acceptable values are 0 to 500, inclusive. (Default: 500)
    #[prost(int32, tag = "3")]
    pub page_size: i32,
    /// Specifies a page token to use. Set this to the nextPageToken returned by
    /// previous list requests to get the next page of results.
    #[prost(string, tag = "4")]
    pub page_token: std::string::String,
}
/// ListUsableSubnetworksResponse is the response of
/// ListUsableSubnetworksRequest.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListUsableSubnetworksResponse {
    /// A list of usable subnetworks in the specified network project.
    #[prost(message, repeated, tag = "1")]
    pub subnetworks: ::std::vec::Vec<UsableSubnetwork>,
    /// This token allows you to get the next page of results for list requests.
    /// If the number of results is larger than `page_size`, use the
    /// `next_page_token` as a value for the query parameter `page_token` in the
    /// next request. The value will become empty when there are no more pages.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
}
/// Secondary IP range of a usable subnetwork.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UsableSubnetworkSecondaryRange {
    /// The name associated with this subnetwork secondary range, used when adding
    /// an alias IP range to a VM instance.
    #[prost(string, tag = "1")]
    pub range_name: std::string::String,
    /// The range of IP addresses belonging to this subnetwork secondary range.
    #[prost(string, tag = "2")]
    pub ip_cidr_range: std::string::String,
    /// This field is to determine the status of the secondary range programmably.
    #[prost(enumeration = "usable_subnetwork_secondary_range::Status", tag = "3")]
    pub status: i32,
}
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
    pub subnetwork: std::string::String,
    /// Network Name.
    /// Example: projects/my-project/global/networks/my-network
    #[prost(string, tag = "2")]
    pub network: std::string::String,
    /// The range of internal addresses that are owned by this subnetwork.
    #[prost(string, tag = "3")]
    pub ip_cidr_range: std::string::String,
    /// Secondary IP ranges.
    #[prost(message, repeated, tag = "4")]
    pub secondary_ip_ranges: ::std::vec::Vec<UsableSubnetworkSecondaryRange>,
    /// A human readable status message representing the reasons for cases where
    /// the caller cannot use the secondary ranges under the subnet. For example if
    /// the secondary_ip_ranges is empty due to a permission issue, an insufficient
    /// permission message will be given by status_message.
    #[prost(string, tag = "5")]
    pub status_message: std::string::String,
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
/// Configuration of etcd encryption.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DatabaseEncryption {
    /// Denotes the state of etcd encryption.
    #[prost(enumeration = "database_encryption::State", tag = "2")]
    pub state: i32,
    /// Name of CloudKMS key to use for the encryption of secrets in etcd.
    /// Ex. projects/my-project/locations/global/keyRings/my-ring/cryptoKeys/my-key
    #[prost(string, tag = "1")]
    pub key_name: std::string::String,
}
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
        /// unrelated to Google Compute Engine level full disk encryption.
        Decrypted = 2,
    }
}
/// Configuration for exporting cluster resource usages.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResourceUsageExportConfig {
    /// Configuration to use BigQuery as usage export destination.
    #[prost(message, optional, tag = "1")]
    pub bigquery_destination:
        ::std::option::Option<resource_usage_export_config::BigQueryDestination>,
    /// Whether to enable network egress metering for this cluster. If enabled, a
    /// daemonset will be created in the cluster to meter network egress traffic.
    #[prost(bool, tag = "2")]
    pub enable_network_egress_metering: bool,
    /// Configuration to enable resource consumption metering.
    #[prost(message, optional, tag = "3")]
    pub consumption_metering_config:
        ::std::option::Option<resource_usage_export_config::ConsumptionMeteringConfig>,
}
pub mod resource_usage_export_config {
    /// Parameters for using BigQuery as the destination of resource usage export.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct BigQueryDestination {
        /// The ID of a BigQuery Dataset.
        #[prost(string, tag = "1")]
        pub dataset_id: std::string::String,
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
#[doc = r" Generated client implementations."]
pub mod cluster_manager_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Google Kubernetes Engine Cluster Manager v1beta1"]
    pub struct ClusterManagerClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> ClusterManagerClient<T>
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
                "/google.container.v1beta1.ClusterManager/ListClusters",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets the details for a specific cluster."]
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
                "/google.container.v1beta1.ClusterManager/GetCluster",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a cluster, consisting of the specified number and type of Google"]
        #[doc = " Compute Engine instances."]
        #[doc = ""]
        #[doc = " By default, the cluster is created in the project's"]
        #[doc = " [default network](https://cloud.google.com/compute/docs/networks-and-firewalls#networks)."]
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
                "/google.container.v1beta1.ClusterManager/CreateCluster",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates the settings for a specific cluster."]
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
                "/google.container.v1beta1.ClusterManager/UpdateCluster",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates the version and/or image type of a specific node pool."]
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
                "/google.container.v1beta1.ClusterManager/UpdateNodePool",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Sets the autoscaling settings of a specific node pool."]
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
                "/google.container.v1beta1.ClusterManager/SetNodePoolAutoscaling",
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
                "/google.container.v1beta1.ClusterManager/SetLoggingService",
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
                "/google.container.v1beta1.ClusterManager/SetMonitoringService",
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
                "/google.container.v1beta1.ClusterManager/SetAddonsConfig",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Sets the locations for a specific cluster."]
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
                "/google.container.v1beta1.ClusterManager/SetLocations",
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
                "/google.container.v1beta1.ClusterManager/UpdateMaster",
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
                "/google.container.v1beta1.ClusterManager/SetMasterAuth",
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
                "/google.container.v1beta1.ClusterManager/DeleteCluster",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists all operations in a project in the specified zone or all zones."]
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
                "/google.container.v1beta1.ClusterManager/ListOperations",
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
                "/google.container.v1beta1.ClusterManager/GetOperation",
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
                "/google.container.v1beta1.ClusterManager/CancelOperation",
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
                "/google.container.v1beta1.ClusterManager/GetServerConfig",
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
                "/google.container.v1beta1.ClusterManager/ListNodePools",
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
                "/google.container.v1beta1.ClusterManager/GetNodePool",
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
                "/google.container.v1beta1.ClusterManager/CreateNodePool",
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
                "/google.container.v1beta1.ClusterManager/DeleteNodePool",
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
                "/google.container.v1beta1.ClusterManager/RollbackNodePoolUpgrade",
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
                "/google.container.v1beta1.ClusterManager/SetNodePoolManagement",
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
                "/google.container.v1beta1.ClusterManager/SetLabels",
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
                "/google.container.v1beta1.ClusterManager/SetLegacyAbac",
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
                "/google.container.v1beta1.ClusterManager/StartIPRotation",
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
                "/google.container.v1beta1.ClusterManager/CompleteIPRotation",
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
                "/google.container.v1beta1.ClusterManager/SetNodePoolSize",
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
                "/google.container.v1beta1.ClusterManager/SetNetworkPolicy",
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
                "/google.container.v1beta1.ClusterManager/SetMaintenancePolicy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists subnetworks that can be used for creating clusters in a project."]
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
                "/google.container.v1beta1.ClusterManager/ListUsableSubnetworks",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Fetches locations that offer Google Kubernetes Engine."]
        pub async fn list_locations(
            &mut self,
            request: impl tonic::IntoRequest<super::ListLocationsRequest>,
        ) -> Result<tonic::Response<super::ListLocationsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
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
    impl<T: Clone> Clone for ClusterManagerClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for ClusterManagerClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "ClusterManagerClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod cluster_manager_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with ClusterManagerServer."]
    #[async_trait]
    pub trait ClusterManager: Send + Sync + 'static {
        #[doc = " Lists all clusters owned by a project in either the specified zone or all"]
        #[doc = " zones."]
        async fn list_clusters(
            &self,
            request: tonic::Request<super::ListClustersRequest>,
        ) -> Result<tonic::Response<super::ListClustersResponse>, tonic::Status>;
        #[doc = " Gets the details for a specific cluster."]
        async fn get_cluster(
            &self,
            request: tonic::Request<super::GetClusterRequest>,
        ) -> Result<tonic::Response<super::Cluster>, tonic::Status>;
        #[doc = " Creates a cluster, consisting of the specified number and type of Google"]
        #[doc = " Compute Engine instances."]
        #[doc = ""]
        #[doc = " By default, the cluster is created in the project's"]
        #[doc = " [default network](https://cloud.google.com/compute/docs/networks-and-firewalls#networks)."]
        #[doc = ""]
        #[doc = " One firewall is added for the cluster. After cluster creation,"]
        #[doc = " the Kubelet creates routes for each node to allow the containers"]
        #[doc = " on that node to communicate with all other instances in the"]
        #[doc = " cluster."]
        #[doc = ""]
        #[doc = " Finally, an entry is added to the project's global metadata indicating"]
        #[doc = " which CIDR range the cluster is using."]
        async fn create_cluster(
            &self,
            request: tonic::Request<super::CreateClusterRequest>,
        ) -> Result<tonic::Response<super::Operation>, tonic::Status>;
        #[doc = " Updates the settings for a specific cluster."]
        async fn update_cluster(
            &self,
            request: tonic::Request<super::UpdateClusterRequest>,
        ) -> Result<tonic::Response<super::Operation>, tonic::Status>;
        #[doc = " Updates the version and/or image type of a specific node pool."]
        async fn update_node_pool(
            &self,
            request: tonic::Request<super::UpdateNodePoolRequest>,
        ) -> Result<tonic::Response<super::Operation>, tonic::Status>;
        #[doc = " Sets the autoscaling settings of a specific node pool."]
        async fn set_node_pool_autoscaling(
            &self,
            request: tonic::Request<super::SetNodePoolAutoscalingRequest>,
        ) -> Result<tonic::Response<super::Operation>, tonic::Status>;
        #[doc = " Sets the logging service for a specific cluster."]
        async fn set_logging_service(
            &self,
            request: tonic::Request<super::SetLoggingServiceRequest>,
        ) -> Result<tonic::Response<super::Operation>, tonic::Status>;
        #[doc = " Sets the monitoring service for a specific cluster."]
        async fn set_monitoring_service(
            &self,
            request: tonic::Request<super::SetMonitoringServiceRequest>,
        ) -> Result<tonic::Response<super::Operation>, tonic::Status>;
        #[doc = " Sets the addons for a specific cluster."]
        async fn set_addons_config(
            &self,
            request: tonic::Request<super::SetAddonsConfigRequest>,
        ) -> Result<tonic::Response<super::Operation>, tonic::Status>;
        #[doc = " Sets the locations for a specific cluster."]
        async fn set_locations(
            &self,
            request: tonic::Request<super::SetLocationsRequest>,
        ) -> Result<tonic::Response<super::Operation>, tonic::Status>;
        #[doc = " Updates the master for a specific cluster."]
        async fn update_master(
            &self,
            request: tonic::Request<super::UpdateMasterRequest>,
        ) -> Result<tonic::Response<super::Operation>, tonic::Status>;
        #[doc = " Sets master auth materials. Currently supports changing the admin password"]
        #[doc = " or a specific cluster, either via password generation or explicitly setting"]
        #[doc = " the password."]
        async fn set_master_auth(
            &self,
            request: tonic::Request<super::SetMasterAuthRequest>,
        ) -> Result<tonic::Response<super::Operation>, tonic::Status>;
        #[doc = " Deletes the cluster, including the Kubernetes endpoint and all worker"]
        #[doc = " nodes."]
        #[doc = ""]
        #[doc = " Firewalls and routes that were configured during cluster creation"]
        #[doc = " are also deleted."]
        #[doc = ""]
        #[doc = " Other Google Compute Engine resources that might be in use by the cluster,"]
        #[doc = " such as load balancer resources, are not deleted if they weren't present"]
        #[doc = " when the cluster was initially created."]
        async fn delete_cluster(
            &self,
            request: tonic::Request<super::DeleteClusterRequest>,
        ) -> Result<tonic::Response<super::Operation>, tonic::Status>;
        #[doc = " Lists all operations in a project in the specified zone or all zones."]
        async fn list_operations(
            &self,
            request: tonic::Request<super::ListOperationsRequest>,
        ) -> Result<tonic::Response<super::ListOperationsResponse>, tonic::Status>;
        #[doc = " Gets the specified operation."]
        async fn get_operation(
            &self,
            request: tonic::Request<super::GetOperationRequest>,
        ) -> Result<tonic::Response<super::Operation>, tonic::Status>;
        #[doc = " Cancels the specified operation."]
        async fn cancel_operation(
            &self,
            request: tonic::Request<super::CancelOperationRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status>;
        #[doc = " Returns configuration info about the Google Kubernetes Engine service."]
        async fn get_server_config(
            &self,
            request: tonic::Request<super::GetServerConfigRequest>,
        ) -> Result<tonic::Response<super::ServerConfig>, tonic::Status>;
        #[doc = " Lists the node pools for a cluster."]
        async fn list_node_pools(
            &self,
            request: tonic::Request<super::ListNodePoolsRequest>,
        ) -> Result<tonic::Response<super::ListNodePoolsResponse>, tonic::Status>;
        #[doc = " Retrieves the requested node pool."]
        async fn get_node_pool(
            &self,
            request: tonic::Request<super::GetNodePoolRequest>,
        ) -> Result<tonic::Response<super::NodePool>, tonic::Status>;
        #[doc = " Creates a node pool for a cluster."]
        async fn create_node_pool(
            &self,
            request: tonic::Request<super::CreateNodePoolRequest>,
        ) -> Result<tonic::Response<super::Operation>, tonic::Status>;
        #[doc = " Deletes a node pool from a cluster."]
        async fn delete_node_pool(
            &self,
            request: tonic::Request<super::DeleteNodePoolRequest>,
        ) -> Result<tonic::Response<super::Operation>, tonic::Status>;
        #[doc = " Rolls back a previously Aborted or Failed NodePool upgrade."]
        #[doc = " This makes no changes if the last upgrade successfully completed."]
        async fn rollback_node_pool_upgrade(
            &self,
            request: tonic::Request<super::RollbackNodePoolUpgradeRequest>,
        ) -> Result<tonic::Response<super::Operation>, tonic::Status>;
        #[doc = " Sets the NodeManagement options for a node pool."]
        async fn set_node_pool_management(
            &self,
            request: tonic::Request<super::SetNodePoolManagementRequest>,
        ) -> Result<tonic::Response<super::Operation>, tonic::Status>;
        #[doc = " Sets labels on a cluster."]
        async fn set_labels(
            &self,
            request: tonic::Request<super::SetLabelsRequest>,
        ) -> Result<tonic::Response<super::Operation>, tonic::Status>;
        #[doc = " Enables or disables the ABAC authorization mechanism on a cluster."]
        async fn set_legacy_abac(
            &self,
            request: tonic::Request<super::SetLegacyAbacRequest>,
        ) -> Result<tonic::Response<super::Operation>, tonic::Status>;
        #[doc = " Starts master IP rotation."]
        async fn start_ip_rotation(
            &self,
            request: tonic::Request<super::StartIpRotationRequest>,
        ) -> Result<tonic::Response<super::Operation>, tonic::Status>;
        #[doc = " Completes master IP rotation."]
        async fn complete_ip_rotation(
            &self,
            request: tonic::Request<super::CompleteIpRotationRequest>,
        ) -> Result<tonic::Response<super::Operation>, tonic::Status>;
        #[doc = " Sets the size for a specific node pool."]
        async fn set_node_pool_size(
            &self,
            request: tonic::Request<super::SetNodePoolSizeRequest>,
        ) -> Result<tonic::Response<super::Operation>, tonic::Status>;
        #[doc = " Enables or disables Network Policy for a cluster."]
        async fn set_network_policy(
            &self,
            request: tonic::Request<super::SetNetworkPolicyRequest>,
        ) -> Result<tonic::Response<super::Operation>, tonic::Status>;
        #[doc = " Sets the maintenance policy for a cluster."]
        async fn set_maintenance_policy(
            &self,
            request: tonic::Request<super::SetMaintenancePolicyRequest>,
        ) -> Result<tonic::Response<super::Operation>, tonic::Status>;
        #[doc = " Lists subnetworks that can be used for creating clusters in a project."]
        async fn list_usable_subnetworks(
            &self,
            request: tonic::Request<super::ListUsableSubnetworksRequest>,
        ) -> Result<tonic::Response<super::ListUsableSubnetworksResponse>, tonic::Status>;
        #[doc = " Fetches locations that offer Google Kubernetes Engine."]
        async fn list_locations(
            &self,
            request: tonic::Request<super::ListLocationsRequest>,
        ) -> Result<tonic::Response<super::ListLocationsResponse>, tonic::Status>;
    }
    #[doc = " Google Kubernetes Engine Cluster Manager v1beta1"]
    #[derive(Debug)]
    #[doc(hidden)]
    pub struct ClusterManagerServer<T: ClusterManager> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: ClusterManager> ClusterManagerServer<T> {
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
    impl<T, B> Service<http::Request<B>> for ClusterManagerServer<T>
    where
        T: ClusterManager,
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
                "/google.container.v1beta1.ClusterManager/ListClusters" => {
                    #[allow(non_camel_case_types)]
                    struct ListClustersSvc<T: ClusterManager>(pub Arc<T>);
                    impl<T: ClusterManager> tonic::server::UnaryService<super::ListClustersRequest>
                        for ListClustersSvc<T>
                    {
                        type Response = super::ListClustersResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListClustersRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.list_clusters(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ListClustersSvc(inner);
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
                "/google.container.v1beta1.ClusterManager/GetCluster" => {
                    #[allow(non_camel_case_types)]
                    struct GetClusterSvc<T: ClusterManager>(pub Arc<T>);
                    impl<T: ClusterManager> tonic::server::UnaryService<super::GetClusterRequest> for GetClusterSvc<T> {
                        type Response = super::Cluster;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetClusterRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.get_cluster(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetClusterSvc(inner);
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
                "/google.container.v1beta1.ClusterManager/CreateCluster" => {
                    #[allow(non_camel_case_types)]
                    struct CreateClusterSvc<T: ClusterManager>(pub Arc<T>);
                    impl<T: ClusterManager> tonic::server::UnaryService<super::CreateClusterRequest>
                        for CreateClusterSvc<T>
                    {
                        type Response = super::Operation;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateClusterRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.create_cluster(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = CreateClusterSvc(inner);
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
                "/google.container.v1beta1.ClusterManager/UpdateCluster" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateClusterSvc<T: ClusterManager>(pub Arc<T>);
                    impl<T: ClusterManager> tonic::server::UnaryService<super::UpdateClusterRequest>
                        for UpdateClusterSvc<T>
                    {
                        type Response = super::Operation;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateClusterRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.update_cluster(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = UpdateClusterSvc(inner);
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
                "/google.container.v1beta1.ClusterManager/UpdateNodePool" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateNodePoolSvc<T: ClusterManager>(pub Arc<T>);
                    impl<T: ClusterManager>
                        tonic::server::UnaryService<super::UpdateNodePoolRequest>
                        for UpdateNodePoolSvc<T>
                    {
                        type Response = super::Operation;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateNodePoolRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.update_node_pool(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = UpdateNodePoolSvc(inner);
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
                "/google.container.v1beta1.ClusterManager/SetNodePoolAutoscaling" => {
                    #[allow(non_camel_case_types)]
                    struct SetNodePoolAutoscalingSvc<T: ClusterManager>(pub Arc<T>);
                    impl<T: ClusterManager>
                        tonic::server::UnaryService<super::SetNodePoolAutoscalingRequest>
                        for SetNodePoolAutoscalingSvc<T>
                    {
                        type Response = super::Operation;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SetNodePoolAutoscalingRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.set_node_pool_autoscaling(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = SetNodePoolAutoscalingSvc(inner);
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
                "/google.container.v1beta1.ClusterManager/SetLoggingService" => {
                    #[allow(non_camel_case_types)]
                    struct SetLoggingServiceSvc<T: ClusterManager>(pub Arc<T>);
                    impl<T: ClusterManager>
                        tonic::server::UnaryService<super::SetLoggingServiceRequest>
                        for SetLoggingServiceSvc<T>
                    {
                        type Response = super::Operation;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SetLoggingServiceRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.set_logging_service(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = SetLoggingServiceSvc(inner);
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
                "/google.container.v1beta1.ClusterManager/SetMonitoringService" => {
                    #[allow(non_camel_case_types)]
                    struct SetMonitoringServiceSvc<T: ClusterManager>(pub Arc<T>);
                    impl<T: ClusterManager>
                        tonic::server::UnaryService<super::SetMonitoringServiceRequest>
                        for SetMonitoringServiceSvc<T>
                    {
                        type Response = super::Operation;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SetMonitoringServiceRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.set_monitoring_service(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = SetMonitoringServiceSvc(inner);
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
                "/google.container.v1beta1.ClusterManager/SetAddonsConfig" => {
                    #[allow(non_camel_case_types)]
                    struct SetAddonsConfigSvc<T: ClusterManager>(pub Arc<T>);
                    impl<T: ClusterManager>
                        tonic::server::UnaryService<super::SetAddonsConfigRequest>
                        for SetAddonsConfigSvc<T>
                    {
                        type Response = super::Operation;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SetAddonsConfigRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.set_addons_config(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = SetAddonsConfigSvc(inner);
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
                "/google.container.v1beta1.ClusterManager/SetLocations" => {
                    #[allow(non_camel_case_types)]
                    struct SetLocationsSvc<T: ClusterManager>(pub Arc<T>);
                    impl<T: ClusterManager> tonic::server::UnaryService<super::SetLocationsRequest>
                        for SetLocationsSvc<T>
                    {
                        type Response = super::Operation;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SetLocationsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.set_locations(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = SetLocationsSvc(inner);
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
                "/google.container.v1beta1.ClusterManager/UpdateMaster" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateMasterSvc<T: ClusterManager>(pub Arc<T>);
                    impl<T: ClusterManager> tonic::server::UnaryService<super::UpdateMasterRequest>
                        for UpdateMasterSvc<T>
                    {
                        type Response = super::Operation;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateMasterRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.update_master(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = UpdateMasterSvc(inner);
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
                "/google.container.v1beta1.ClusterManager/SetMasterAuth" => {
                    #[allow(non_camel_case_types)]
                    struct SetMasterAuthSvc<T: ClusterManager>(pub Arc<T>);
                    impl<T: ClusterManager> tonic::server::UnaryService<super::SetMasterAuthRequest>
                        for SetMasterAuthSvc<T>
                    {
                        type Response = super::Operation;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SetMasterAuthRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.set_master_auth(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = SetMasterAuthSvc(inner);
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
                "/google.container.v1beta1.ClusterManager/DeleteCluster" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteClusterSvc<T: ClusterManager>(pub Arc<T>);
                    impl<T: ClusterManager> tonic::server::UnaryService<super::DeleteClusterRequest>
                        for DeleteClusterSvc<T>
                    {
                        type Response = super::Operation;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteClusterRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.delete_cluster(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = DeleteClusterSvc(inner);
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
                "/google.container.v1beta1.ClusterManager/ListOperations" => {
                    #[allow(non_camel_case_types)]
                    struct ListOperationsSvc<T: ClusterManager>(pub Arc<T>);
                    impl<T: ClusterManager>
                        tonic::server::UnaryService<super::ListOperationsRequest>
                        for ListOperationsSvc<T>
                    {
                        type Response = super::ListOperationsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListOperationsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.list_operations(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ListOperationsSvc(inner);
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
                "/google.container.v1beta1.ClusterManager/GetOperation" => {
                    #[allow(non_camel_case_types)]
                    struct GetOperationSvc<T: ClusterManager>(pub Arc<T>);
                    impl<T: ClusterManager> tonic::server::UnaryService<super::GetOperationRequest>
                        for GetOperationSvc<T>
                    {
                        type Response = super::Operation;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetOperationRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.get_operation(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetOperationSvc(inner);
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
                "/google.container.v1beta1.ClusterManager/CancelOperation" => {
                    #[allow(non_camel_case_types)]
                    struct CancelOperationSvc<T: ClusterManager>(pub Arc<T>);
                    impl<T: ClusterManager>
                        tonic::server::UnaryService<super::CancelOperationRequest>
                        for CancelOperationSvc<T>
                    {
                        type Response = ();
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CancelOperationRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.cancel_operation(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = CancelOperationSvc(inner);
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
                "/google.container.v1beta1.ClusterManager/GetServerConfig" => {
                    #[allow(non_camel_case_types)]
                    struct GetServerConfigSvc<T: ClusterManager>(pub Arc<T>);
                    impl<T: ClusterManager>
                        tonic::server::UnaryService<super::GetServerConfigRequest>
                        for GetServerConfigSvc<T>
                    {
                        type Response = super::ServerConfig;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetServerConfigRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.get_server_config(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetServerConfigSvc(inner);
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
                "/google.container.v1beta1.ClusterManager/ListNodePools" => {
                    #[allow(non_camel_case_types)]
                    struct ListNodePoolsSvc<T: ClusterManager>(pub Arc<T>);
                    impl<T: ClusterManager> tonic::server::UnaryService<super::ListNodePoolsRequest>
                        for ListNodePoolsSvc<T>
                    {
                        type Response = super::ListNodePoolsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListNodePoolsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.list_node_pools(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ListNodePoolsSvc(inner);
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
                "/google.container.v1beta1.ClusterManager/GetNodePool" => {
                    #[allow(non_camel_case_types)]
                    struct GetNodePoolSvc<T: ClusterManager>(pub Arc<T>);
                    impl<T: ClusterManager> tonic::server::UnaryService<super::GetNodePoolRequest>
                        for GetNodePoolSvc<T>
                    {
                        type Response = super::NodePool;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetNodePoolRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.get_node_pool(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetNodePoolSvc(inner);
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
                "/google.container.v1beta1.ClusterManager/CreateNodePool" => {
                    #[allow(non_camel_case_types)]
                    struct CreateNodePoolSvc<T: ClusterManager>(pub Arc<T>);
                    impl<T: ClusterManager>
                        tonic::server::UnaryService<super::CreateNodePoolRequest>
                        for CreateNodePoolSvc<T>
                    {
                        type Response = super::Operation;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateNodePoolRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.create_node_pool(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = CreateNodePoolSvc(inner);
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
                "/google.container.v1beta1.ClusterManager/DeleteNodePool" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteNodePoolSvc<T: ClusterManager>(pub Arc<T>);
                    impl<T: ClusterManager>
                        tonic::server::UnaryService<super::DeleteNodePoolRequest>
                        for DeleteNodePoolSvc<T>
                    {
                        type Response = super::Operation;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteNodePoolRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.delete_node_pool(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = DeleteNodePoolSvc(inner);
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
                "/google.container.v1beta1.ClusterManager/RollbackNodePoolUpgrade" => {
                    #[allow(non_camel_case_types)]
                    struct RollbackNodePoolUpgradeSvc<T: ClusterManager>(pub Arc<T>);
                    impl<T: ClusterManager>
                        tonic::server::UnaryService<super::RollbackNodePoolUpgradeRequest>
                        for RollbackNodePoolUpgradeSvc<T>
                    {
                        type Response = super::Operation;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RollbackNodePoolUpgradeRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { inner.rollback_node_pool_upgrade(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = RollbackNodePoolUpgradeSvc(inner);
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
                "/google.container.v1beta1.ClusterManager/SetNodePoolManagement" => {
                    #[allow(non_camel_case_types)]
                    struct SetNodePoolManagementSvc<T: ClusterManager>(pub Arc<T>);
                    impl<T: ClusterManager>
                        tonic::server::UnaryService<super::SetNodePoolManagementRequest>
                        for SetNodePoolManagementSvc<T>
                    {
                        type Response = super::Operation;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SetNodePoolManagementRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.set_node_pool_management(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = SetNodePoolManagementSvc(inner);
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
                "/google.container.v1beta1.ClusterManager/SetLabels" => {
                    #[allow(non_camel_case_types)]
                    struct SetLabelsSvc<T: ClusterManager>(pub Arc<T>);
                    impl<T: ClusterManager> tonic::server::UnaryService<super::SetLabelsRequest> for SetLabelsSvc<T> {
                        type Response = super::Operation;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SetLabelsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.set_labels(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = SetLabelsSvc(inner);
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
                "/google.container.v1beta1.ClusterManager/SetLegacyAbac" => {
                    #[allow(non_camel_case_types)]
                    struct SetLegacyAbacSvc<T: ClusterManager>(pub Arc<T>);
                    impl<T: ClusterManager> tonic::server::UnaryService<super::SetLegacyAbacRequest>
                        for SetLegacyAbacSvc<T>
                    {
                        type Response = super::Operation;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SetLegacyAbacRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.set_legacy_abac(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = SetLegacyAbacSvc(inner);
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
                "/google.container.v1beta1.ClusterManager/StartIPRotation" => {
                    #[allow(non_camel_case_types)]
                    struct StartIPRotationSvc<T: ClusterManager>(pub Arc<T>);
                    impl<T: ClusterManager>
                        tonic::server::UnaryService<super::StartIpRotationRequest>
                        for StartIPRotationSvc<T>
                    {
                        type Response = super::Operation;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::StartIpRotationRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.start_ip_rotation(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = StartIPRotationSvc(inner);
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
                "/google.container.v1beta1.ClusterManager/CompleteIPRotation" => {
                    #[allow(non_camel_case_types)]
                    struct CompleteIPRotationSvc<T: ClusterManager>(pub Arc<T>);
                    impl<T: ClusterManager>
                        tonic::server::UnaryService<super::CompleteIpRotationRequest>
                        for CompleteIPRotationSvc<T>
                    {
                        type Response = super::Operation;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CompleteIpRotationRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.complete_ip_rotation(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = CompleteIPRotationSvc(inner);
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
                "/google.container.v1beta1.ClusterManager/SetNodePoolSize" => {
                    #[allow(non_camel_case_types)]
                    struct SetNodePoolSizeSvc<T: ClusterManager>(pub Arc<T>);
                    impl<T: ClusterManager>
                        tonic::server::UnaryService<super::SetNodePoolSizeRequest>
                        for SetNodePoolSizeSvc<T>
                    {
                        type Response = super::Operation;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SetNodePoolSizeRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.set_node_pool_size(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = SetNodePoolSizeSvc(inner);
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
                "/google.container.v1beta1.ClusterManager/SetNetworkPolicy" => {
                    #[allow(non_camel_case_types)]
                    struct SetNetworkPolicySvc<T: ClusterManager>(pub Arc<T>);
                    impl<T: ClusterManager>
                        tonic::server::UnaryService<super::SetNetworkPolicyRequest>
                        for SetNetworkPolicySvc<T>
                    {
                        type Response = super::Operation;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SetNetworkPolicyRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.set_network_policy(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = SetNetworkPolicySvc(inner);
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
                "/google.container.v1beta1.ClusterManager/SetMaintenancePolicy" => {
                    #[allow(non_camel_case_types)]
                    struct SetMaintenancePolicySvc<T: ClusterManager>(pub Arc<T>);
                    impl<T: ClusterManager>
                        tonic::server::UnaryService<super::SetMaintenancePolicyRequest>
                        for SetMaintenancePolicySvc<T>
                    {
                        type Response = super::Operation;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SetMaintenancePolicyRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.set_maintenance_policy(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = SetMaintenancePolicySvc(inner);
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
                "/google.container.v1beta1.ClusterManager/ListUsableSubnetworks" => {
                    #[allow(non_camel_case_types)]
                    struct ListUsableSubnetworksSvc<T: ClusterManager>(pub Arc<T>);
                    impl<T: ClusterManager>
                        tonic::server::UnaryService<super::ListUsableSubnetworksRequest>
                        for ListUsableSubnetworksSvc<T>
                    {
                        type Response = super::ListUsableSubnetworksResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListUsableSubnetworksRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.list_usable_subnetworks(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ListUsableSubnetworksSvc(inner);
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
                "/google.container.v1beta1.ClusterManager/ListLocations" => {
                    #[allow(non_camel_case_types)]
                    struct ListLocationsSvc<T: ClusterManager>(pub Arc<T>);
                    impl<T: ClusterManager> tonic::server::UnaryService<super::ListLocationsRequest>
                        for ListLocationsSvc<T>
                    {
                        type Response = super::ListLocationsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListLocationsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.list_locations(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ListLocationsSvc(inner);
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
    impl<T: ClusterManager> Clone for ClusterManagerServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: ClusterManager> Clone for _Inner<T> {
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
