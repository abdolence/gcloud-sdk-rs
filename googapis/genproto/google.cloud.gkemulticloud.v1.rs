/// Workload Identity settings.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WorkloadIdentityConfig {
    /// The OIDC issuer URL for this cluster.
    #[prost(string, tag = "1")]
    pub issuer_uri: ::prost::alloc::string::String,
    /// The Workload Identity Pool associated to the cluster.
    #[prost(string, tag = "2")]
    pub workload_pool: ::prost::alloc::string::String,
    /// The ID of the OIDC Identity Provider (IdP) associated to the Workload
    /// Identity Pool.
    #[prost(string, tag = "3")]
    pub identity_provider: ::prost::alloc::string::String,
}
/// Constraints applied to pods.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MaxPodsConstraint {
    /// Required. The maximum number of pods to schedule on a single node.
    #[prost(int64, tag = "1")]
    pub max_pods_per_node: i64,
}
/// Metadata about a long-running operation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationMetadata {
    /// Output only. The time at which this operation was created.
    #[prost(message, optional, tag = "1")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The time at which this operation was completed.
    #[prost(message, optional, tag = "2")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The name of the resource associated to this operation.
    #[prost(string, tag = "3")]
    pub target: ::prost::alloc::string::String,
    /// Output only. Human-readable status of the operation, if any.
    #[prost(string, tag = "4")]
    pub status_detail: ::prost::alloc::string::String,
    /// Output only. Human-readable status of any error that occurred during the operation.
    #[prost(string, tag = "5")]
    pub error_detail: ::prost::alloc::string::String,
}
/// The taint content for the node taint.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NodeTaint {
    /// Required. Key for the taint.
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
    /// Required. Value for the taint.
    #[prost(string, tag = "2")]
    pub value: ::prost::alloc::string::String,
    /// Required. The taint effect.
    #[prost(enumeration = "node_taint::Effect", tag = "3")]
    pub effect: i32,
}
/// Nested message and enum types in `NodeTaint`.
pub mod node_taint {
    /// The taint effect.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Effect {
        /// Not set.
        Unspecified = 0,
        /// Do not allow new pods to schedule onto the node unless they tolerate the
        /// taint, but allow all pods submitted to Kubelet without going through the
        /// scheduler to start, and allow all already-running pods to continue
        /// running. Enforced by the scheduler.
        NoSchedule = 1,
        /// Like TaintEffectNoSchedule, but the scheduler tries not to schedule
        /// new pods onto the node, rather than prohibiting new pods from scheduling
        /// onto the node entirely. Enforced by the scheduler.
        PreferNoSchedule = 2,
        /// Evict any already-running pods that do not tolerate the taint.
        /// Currently enforced by NodeController.
        NoExecute = 3,
    }
}
/// Fleet related configuration.
///
/// Fleets are a Google Cloud concept for logically organizing clusters,
/// letting you use and manage multi-cluster capabilities and apply
/// consistent policies across your systems.
///
/// See [Anthos
/// Fleets](<https://cloud.google.com/anthos/multicluster-management/fleets>) for
/// more details on Anthos multi-cluster capabilities using Fleets.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Fleet {
    /// Required. The name of the Fleet host project where this cluster will be registered.
    ///
    /// Project names are formatted as
    /// `projects/<project-number>`.
    #[prost(string, tag = "1")]
    pub project: ::prost::alloc::string::String,
    /// Output only. The name of the managed Hub Membership resource associated to this cluster.
    ///
    /// Membership names are formatted as
    /// `projects/<project-number>/locations/global/membership/<cluster-id>`.
    #[prost(string, tag = "2")]
    pub membership: ::prost::alloc::string::String,
}
/// Parameters that describe the Logging configuration in a cluster.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LoggingConfig {
    /// The configuration of the logging components;
    #[prost(message, optional, tag = "1")]
    pub component_config: ::core::option::Option<LoggingComponentConfig>,
}
/// Parameters that describe the Logging component configuration in a cluster.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LoggingComponentConfig {
    /// The components to be enabled.
    #[prost(
        enumeration = "logging_component_config::Component",
        repeated,
        tag = "1"
    )]
    pub enable_components: ::prost::alloc::vec::Vec<i32>,
}
/// Nested message and enum types in `LoggingComponentConfig`.
pub mod logging_component_config {
    /// The components of the logging configuration;
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Component {
        /// No component is specified
        Unspecified = 0,
        /// This indicates that system logging components is enabled.
        SystemComponents = 1,
        /// This indicates that user workload logging component is enabled.
        Workloads = 2,
    }
}
/// An Anthos cluster running on AWS.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AwsCluster {
    /// The name of this resource.
    ///
    /// Cluster names are formatted as
    /// `projects/<project-number>/locations/<region>/awsClusters/<cluster-id>`.
    ///
    /// See [Resource Names](<https://cloud.google.com/apis/design/resource_names>)
    /// for more details on GCP resource names.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. A human readable description of this cluster.
    /// Cannot be longer than 255 UTF-8 encoded bytes.
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// Required. Cluster-wide networking configuration.
    #[prost(message, optional, tag = "3")]
    pub networking: ::core::option::Option<AwsClusterNetworking>,
    /// Required. The AWS region where the cluster runs.
    ///
    /// Each Google Cloud region supports a subset of nearby AWS regions.
    /// You can call
    /// \[GetAwsServerConfig][google.cloud.gkemulticloud.v1.AwsClusters.GetAwsServerConfig\]
    /// to list all supported AWS regions within a given Google Cloud region.
    #[prost(string, tag = "4")]
    pub aws_region: ::prost::alloc::string::String,
    /// Required. Configuration related to the cluster control plane.
    #[prost(message, optional, tag = "5")]
    pub control_plane: ::core::option::Option<AwsControlPlane>,
    /// Required. Configuration related to the cluster RBAC settings.
    #[prost(message, optional, tag = "15")]
    pub authorization: ::core::option::Option<AwsAuthorization>,
    /// Output only. The current state of the cluster.
    #[prost(enumeration = "aws_cluster::State", tag = "7")]
    pub state: i32,
    /// Output only. The endpoint of the cluster's API server.
    #[prost(string, tag = "8")]
    pub endpoint: ::prost::alloc::string::String,
    /// Output only. A globally unique identifier for the cluster.
    #[prost(string, tag = "9")]
    pub uid: ::prost::alloc::string::String,
    /// Output only. If set, there are currently changes in flight to the cluster.
    #[prost(bool, tag = "10")]
    pub reconciling: bool,
    /// Output only. The time at which this cluster was created.
    #[prost(message, optional, tag = "11")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The time at which this cluster was last updated.
    #[prost(message, optional, tag = "12")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Allows clients to perform consistent read-modify-writes
    /// through optimistic concurrency control.
    ///
    /// Can be sent on update and delete requests to ensure the
    /// client has an up-to-date value before proceeding.
    #[prost(string, tag = "13")]
    pub etag: ::prost::alloc::string::String,
    /// Optional. Annotations on the cluster.
    ///
    /// This field has the same restrictions as Kubernetes annotations.
    /// The total size of all keys and values combined is limited to 256k.
    /// Key can have 2 segments: prefix (optional) and name (required),
    /// separated by a slash (/).
    /// Prefix must be a DNS subdomain.
    /// Name must be 63 characters or less, begin and end with alphanumerics,
    /// with dashes (-), underscores (_), dots (.), and alphanumerics between.
    #[prost(map = "string, string", tag = "14")]
    pub annotations:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Output only. Workload Identity settings.
    #[prost(message, optional, tag = "16")]
    pub workload_identity_config: ::core::option::Option<WorkloadIdentityConfig>,
    /// Output only. PEM encoded x509 certificate of the cluster root of trust.
    #[prost(string, tag = "17")]
    pub cluster_ca_certificate: ::prost::alloc::string::String,
    /// Optional. Fleet configuration.
    #[prost(message, optional, tag = "18")]
    pub fleet: ::core::option::Option<Fleet>,
    /// Optional. Logging configuration for this cluster.
    #[prost(message, optional, tag = "19")]
    pub logging_config: ::core::option::Option<LoggingConfig>,
}
/// Nested message and enum types in `AwsCluster`.
pub mod aws_cluster {
    /// The lifecycle state of the cluster.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// Not set.
        Unspecified = 0,
        /// The PROVISIONING state indicates the cluster is being created.
        Provisioning = 1,
        /// The RUNNING state indicates the cluster has been created and is fully
        /// usable.
        Running = 2,
        /// The RECONCILING state indicates that some work is actively being done on
        /// the cluster, such as upgrading the control plane replicas.
        Reconciling = 3,
        /// The STOPPING state indicates the cluster is being deleted.
        Stopping = 4,
        /// The ERROR state indicates the cluster is in a broken unrecoverable
        /// state.
        Error = 5,
        /// The DEGRADED state indicates the cluster requires user action to
        /// restore full functionality.
        Degraded = 6,
    }
}
/// ControlPlane defines common parameters between control plane nodes.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AwsControlPlane {
    /// Required. The Kubernetes version to run on control plane replicas
    /// (e.g. `1.19.10-gke.1000`).
    ///
    /// You can list all supported versions on a given Google Cloud region by
    /// calling
    /// \[GetAwsServerConfig][google.cloud.gkemulticloud.v1.AwsClusters.GetAwsServerConfig\].
    #[prost(string, tag = "1")]
    pub version: ::prost::alloc::string::String,
    /// Optional. The AWS instance type.
    ///
    /// When unspecified, it uses a default based on the cluster's version.
    #[prost(string, tag = "2")]
    pub instance_type: ::prost::alloc::string::String,
    /// Optional. SSH configuration for how to access the underlying control plane
    /// machines.
    #[prost(message, optional, tag = "14")]
    pub ssh_config: ::core::option::Option<AwsSshConfig>,
    /// Required. The list of subnets where control plane replicas will run.
    /// A replica will be provisioned on each subnet and up to three values
    /// can be provided.
    /// Each subnet must be in a different AWS Availability Zone (AZ).
    #[prost(string, repeated, tag = "4")]
    pub subnet_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Optional. The IDs of additional security groups to add to control plane
    /// replicas. The Anthos Multi-Cloud API will automatically create and manage
    /// security groups with the minimum rules needed for a functioning cluster.
    #[prost(string, repeated, tag = "5")]
    pub security_group_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Required. The name or ARN of the AWS IAM instance profile to assign to each control
    /// plane replica.
    #[prost(string, tag = "7")]
    pub iam_instance_profile: ::prost::alloc::string::String,
    /// Optional. Configuration related to the root volume provisioned for each
    /// control plane replica.
    ///
    /// Volumes will be provisioned in the availability zone associated
    /// with the corresponding subnet.
    ///
    /// When unspecified, it defaults to 32 GiB with the GP2 volume type.
    #[prost(message, optional, tag = "8")]
    pub root_volume: ::core::option::Option<AwsVolumeTemplate>,
    /// Optional. Configuration related to the main volume provisioned for each
    /// control plane replica.
    /// The main volume is in charge of storing all of the cluster's etcd state.
    ///
    /// Volumes will be provisioned in the availability zone associated
    /// with the corresponding subnet.
    ///
    /// When unspecified, it defaults to 8 GiB with the GP2 volume type.
    #[prost(message, optional, tag = "9")]
    pub main_volume: ::core::option::Option<AwsVolumeTemplate>,
    /// Required. The ARN of the AWS KMS key used to encrypt cluster secrets.
    #[prost(message, optional, tag = "10")]
    pub database_encryption: ::core::option::Option<AwsDatabaseEncryption>,
    /// Optional. A set of AWS resource tags to propagate to all underlying managed AWS
    /// resources.
    ///
    /// Specify at most 50 pairs containing alphanumerics, spaces, and symbols
    /// (.+-=_:@/). Keys can be up to 127 Unicode characters. Values can be up to
    /// 255 Unicode characters.
    #[prost(map = "string, string", tag = "11")]
    pub tags:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Required. Authentication configuration for management of AWS resources.
    #[prost(message, optional, tag = "12")]
    pub aws_services_authentication: ::core::option::Option<AwsServicesAuthentication>,
    /// Optional. Proxy configuration for outbound HTTP(S) traffic.
    #[prost(message, optional, tag = "16")]
    pub proxy_config: ::core::option::Option<AwsProxyConfig>,
    /// Required. Config encryption for user data.
    #[prost(message, optional, tag = "17")]
    pub config_encryption: ::core::option::Option<AwsConfigEncryption>,
    /// Optional. The placement to use on control plane instances.
    /// When unspecified, the VPC's default tenancy will be used.
    #[prost(message, optional, tag = "18")]
    pub instance_placement: ::core::option::Option<AwsInstancePlacement>,
}
/// Authentication configuration for the management of AWS resources.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AwsServicesAuthentication {
    /// Required. The Amazon Resource Name (ARN) of the role that the Anthos Multi-Cloud API
    /// will assume when managing AWS resources on your account.
    #[prost(string, tag = "1")]
    pub role_arn: ::prost::alloc::string::String,
    /// Optional. An identifier for the assumed role session.
    ///
    /// When unspecified, it defaults to `multicloud-service-agent`.
    #[prost(string, tag = "2")]
    pub role_session_name: ::prost::alloc::string::String,
}
/// Configuration related to the cluster RBAC settings.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AwsAuthorization {
    /// Required. Users that can perform operations as a cluster admin. A managed
    /// ClusterRoleBinding will be created to grant the `cluster-admin` ClusterRole
    /// to the users. Up to ten admin users can be provided.
    ///
    /// For more info on RBAC, see
    /// <https://kubernetes.io/docs/reference/access-authn-authz/rbac/#user-facing-roles>
    #[prost(message, repeated, tag = "1")]
    pub admin_users: ::prost::alloc::vec::Vec<AwsClusterUser>,
}
/// Identities of a user-type subject for AWS clusters.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AwsClusterUser {
    /// Required. The name of the user, e.g. `my-gcp-id@gmail.com`.
    #[prost(string, tag = "1")]
    pub username: ::prost::alloc::string::String,
}
/// Configuration related to application-layer secrets encryption.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AwsDatabaseEncryption {
    /// Required. The ARN of the AWS KMS key used to encrypt cluster secrets.
    #[prost(string, tag = "1")]
    pub kms_key_arn: ::prost::alloc::string::String,
}
/// Configuration template for AWS EBS volumes.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AwsVolumeTemplate {
    /// Optional. The size of the volume, in GiBs.
    ///
    /// When unspecified, a default value is provided. See the specific reference
    /// in the parent resource.
    #[prost(int32, tag = "1")]
    pub size_gib: i32,
    /// Optional. Type of the EBS volume.
    ///
    /// When unspecified, it defaults to GP2 volume.
    #[prost(enumeration = "aws_volume_template::VolumeType", tag = "2")]
    pub volume_type: i32,
    /// Optional. The number of I/O operations per second (IOPS) to provision for GP3 volume.
    #[prost(int32, tag = "3")]
    pub iops: i32,
    /// Optional. The Amazon Resource Name (ARN) of the Customer Managed Key (CMK) used to
    /// encrypt AWS EBS volumes.
    ///
    /// If not specified, the default Amazon managed key associated to
    /// the AWS region where this cluster runs will be used.
    #[prost(string, tag = "4")]
    pub kms_key_arn: ::prost::alloc::string::String,
}
/// Nested message and enum types in `AwsVolumeTemplate`.
pub mod aws_volume_template {
    /// Types of supported EBS volumes. We currently only support GP2 or GP3
    /// volumes.
    /// See <https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/EBSVolumeTypes.html>
    /// for more information.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum VolumeType {
        /// Not set.
        Unspecified = 0,
        /// GP2 (General Purpose SSD volume type).
        Gp2 = 1,
        /// GP3 (General Purpose SSD volume type).
        Gp3 = 2,
    }
}
/// ClusterNetworking defines cluster-wide networking configuration.
///
/// Anthos clusters on AWS run on a single VPC. This includes control
/// plane replicas and node pool nodes.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AwsClusterNetworking {
    /// Required. The VPC associated with the cluster. All component clusters
    /// (i.e. control plane and node pools) run on a single VPC.
    ///
    /// This field cannot be changed after creation.
    #[prost(string, tag = "1")]
    pub vpc_id: ::prost::alloc::string::String,
    /// Required. All pods in the cluster are assigned an IPv4 address from these ranges.
    /// Only a single range is supported.
    /// This field cannot be changed after creation.
    #[prost(string, repeated, tag = "2")]
    pub pod_address_cidr_blocks: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Required. All services in the cluster are assigned an IPv4 address from these ranges.
    /// Only a single range is supported.
    /// This field cannot be changed after creation.
    #[prost(string, repeated, tag = "3")]
    pub service_address_cidr_blocks: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// An Anthos node pool running on AWS.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AwsNodePool {
    /// The name of this resource.
    ///
    /// Node pool names are formatted as
    /// `projects/<project-number>/locations/<region>/awsClusters/<cluster-id>/awsNodePools/<node-pool-id>`.
    ///
    /// For more details on Google Cloud resource names,
    /// see [Resource Names](<https://cloud.google.com/apis/design/resource_names>)
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The Kubernetes version to run on this node pool (e.g. `1.19.10-gke.1000`).
    ///
    /// You can list all supported versions on a given Google Cloud region by
    /// calling
    /// \[GetAwsServerConfig][google.cloud.gkemulticloud.v1.AwsClusters.GetAwsServerConfig\].
    #[prost(string, tag = "3")]
    pub version: ::prost::alloc::string::String,
    /// Required. The configuration of the node pool.
    #[prost(message, optional, tag = "28")]
    pub config: ::core::option::Option<AwsNodeConfig>,
    /// Required. Autoscaler configuration for this node pool.
    #[prost(message, optional, tag = "25")]
    pub autoscaling: ::core::option::Option<AwsNodePoolAutoscaling>,
    /// Required. The subnet where the node pool node run.
    #[prost(string, tag = "6")]
    pub subnet_id: ::prost::alloc::string::String,
    /// Output only. The lifecycle state of the node pool.
    #[prost(enumeration = "aws_node_pool::State", tag = "16")]
    pub state: i32,
    /// Output only. A globally unique identifier for the node pool.
    #[prost(string, tag = "17")]
    pub uid: ::prost::alloc::string::String,
    /// Output only. If set, there are currently changes in flight to the node
    /// pool.
    #[prost(bool, tag = "18")]
    pub reconciling: bool,
    /// Output only. The time at which this node pool was created.
    #[prost(message, optional, tag = "19")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The time at which this node pool was last updated.
    #[prost(message, optional, tag = "20")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Allows clients to perform consistent read-modify-writes
    /// through optimistic concurrency control.
    ///
    /// Can be sent on update and delete requests to ensure the
    /// client has an up-to-date value before proceeding.
    #[prost(string, tag = "21")]
    pub etag: ::prost::alloc::string::String,
    /// Optional. Annotations on the node pool.
    ///
    /// This field has the same restrictions as Kubernetes annotations.
    /// The total size of all keys and values combined is limited to 256k.
    /// Key can have 2 segments: prefix (optional) and name (required),
    /// separated by a slash (/).
    /// Prefix must be a DNS subdomain.
    /// Name must be 63 characters or less, begin and end with alphanumerics,
    /// with dashes (-), underscores (_), dots (.), and alphanumerics between.
    #[prost(map = "string, string", tag = "22")]
    pub annotations:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Required. The constraint on the maximum number of pods that can be run
    /// simultaneously on a node in the node pool.
    #[prost(message, optional, tag = "27")]
    pub max_pods_constraint: ::core::option::Option<MaxPodsConstraint>,
}
/// Nested message and enum types in `AwsNodePool`.
pub mod aws_node_pool {
    /// The lifecycle state of the node pool.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// Not set.
        Unspecified = 0,
        /// The PROVISIONING state indicates the node pool is being created.
        Provisioning = 1,
        /// The RUNNING state indicates the node pool has been created
        /// and is fully usable.
        Running = 2,
        /// The RECONCILING state indicates that the node pool is being reconciled.
        Reconciling = 3,
        /// The STOPPING state indicates the node pool is being deleted.
        Stopping = 4,
        /// The ERROR state indicates the node pool is in a broken unrecoverable
        /// state.
        Error = 5,
        /// The DEGRADED state indicates the node pool requires user action to
        /// restore full functionality.
        Degraded = 6,
    }
}
/// Parameters that describe the nodes in a cluster.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AwsNodeConfig {
    /// Optional. The AWS instance type.
    ///
    /// When unspecified, it uses a default based on the node pool's version.
    #[prost(string, tag = "1")]
    pub instance_type: ::prost::alloc::string::String,
    /// Optional. Template for the root volume provisioned for node pool nodes.
    /// Volumes will be provisioned in the availability zone assigned
    /// to the node pool subnet.
    ///
    /// When unspecified, it defaults to 32 GiB with the GP2 volume type.
    #[prost(message, optional, tag = "2")]
    pub root_volume: ::core::option::Option<AwsVolumeTemplate>,
    /// Optional. The initial taints assigned to nodes of this node pool.
    #[prost(message, repeated, tag = "3")]
    pub taints: ::prost::alloc::vec::Vec<NodeTaint>,
    /// Optional. The initial labels assigned to nodes of this node pool. An object
    /// containing a list of "key": value pairs. Example: { "name": "wrench",
    /// "mass": "1.3kg", "count": "3" }.
    #[prost(map = "string, string", tag = "4")]
    pub labels:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Optional. Key/value metadata to assign to each underlying AWS resource. Specify at
    /// most 50 pairs containing alphanumerics, spaces, and symbols (.+-=_:@/).
    /// Keys can be up to 127 Unicode characters.
    /// Values can be up to 255 Unicode characters.
    #[prost(map = "string, string", tag = "5")]
    pub tags:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Required. The name or ARN of the AWS IAM role assigned to nodes in the pool.
    #[prost(string, tag = "6")]
    pub iam_instance_profile: ::prost::alloc::string::String,
    /// Optional. The OS image type to use on node pool instances.
    /// Can have a value of `ubuntu`, or `windows` if the cluster enables
    /// the Windows node pool preview feature.
    ///
    /// When unspecified, it defaults to `ubuntu`.
    #[prost(string, tag = "11")]
    pub image_type: ::prost::alloc::string::String,
    /// Optional. The SSH configuration.
    #[prost(message, optional, tag = "9")]
    pub ssh_config: ::core::option::Option<AwsSshConfig>,
    /// Optional. The IDs of additional security groups to add to nodes in this pool. The
    /// manager will automatically create security groups with minimum rules
    /// needed for a functioning cluster.
    #[prost(string, repeated, tag = "10")]
    pub security_group_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Optional. Proxy configuration for outbound HTTP(S) traffic.
    #[prost(message, optional, tag = "12")]
    pub proxy_config: ::core::option::Option<AwsProxyConfig>,
    /// Required. Config encryption for user data.
    #[prost(message, optional, tag = "13")]
    pub config_encryption: ::core::option::Option<AwsConfigEncryption>,
    /// Optional. Placement related info for this node.
    /// When unspecified, the VPC's default tenancy will be used.
    #[prost(message, optional, tag = "14")]
    pub instance_placement: ::core::option::Option<AwsInstancePlacement>,
}
/// AwsNodePoolAutoscaling contains information required by cluster autoscaler
/// to adjust the size of the node pool to the current cluster usage.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AwsNodePoolAutoscaling {
    /// Required. Minimum number of nodes in the node pool. Must be greater than or equal to
    /// 1 and less than or equal to max_node_count.
    #[prost(int32, tag = "1")]
    pub min_node_count: i32,
    /// Required. Maximum number of nodes in the node pool. Must be greater than or equal to
    /// min_node_count and less than or equal to 50.
    #[prost(int32, tag = "2")]
    pub max_node_count: i32,
}
/// AwsServerConfig is the configuration of GKE cluster on AWS.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AwsServerConfig {
    /// The resource name of the config.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// List of valid Kubernetes versions.
    #[prost(message, repeated, tag = "2")]
    pub valid_versions: ::prost::alloc::vec::Vec<AwsK8sVersionInfo>,
    /// The list of supported AWS regions.
    #[prost(string, repeated, tag = "3")]
    pub supported_aws_regions: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Kubernetes version information of GKE cluster on AWS.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AwsK8sVersionInfo {
    /// Kubernetes version name.
    #[prost(string, tag = "1")]
    pub version: ::prost::alloc::string::String,
}
/// SSH configuration for AWS resources.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AwsSshConfig {
    /// Required. The name of the EC2 key pair used to login into cluster machines.
    #[prost(string, tag = "1")]
    pub ec2_key_pair: ::prost::alloc::string::String,
}
/// Details of a proxy config stored in AWS Secret Manager.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AwsProxyConfig {
    /// The ARN of the AWS Secret Manager secret that contains the HTTP(S) proxy
    /// configuration.
    #[prost(string, tag = "1")]
    pub secret_arn: ::prost::alloc::string::String,
    /// The version string of the AWS Secret Manager secret that contains the
    /// HTTP(S) proxy configuration.
    #[prost(string, tag = "2")]
    pub secret_version: ::prost::alloc::string::String,
}
/// Config encryption for user data.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AwsConfigEncryption {
    /// Required. The ARN of the AWS KMS key used to encrypt user data.
    #[prost(string, tag = "1")]
    pub kms_key_arn: ::prost::alloc::string::String,
}
/// Details of placement information for an instance.
/// Limitations for using the `host` tenancy:
///
///  * T3 instances that use the unlimited CPU credit option don't support host
///  tenancy.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AwsInstancePlacement {
    /// Required. The tenancy for instance.
    #[prost(enumeration = "aws_instance_placement::Tenancy", tag = "1")]
    pub tenancy: i32,
}
/// Nested message and enum types in `AwsInstancePlacement`.
pub mod aws_instance_placement {
    /// Tenancy defines how EC2 instances are distributed across physical hardware.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Tenancy {
        /// Not set.
        Unspecified = 0,
        /// Use default VPC tenancy.
        Default = 1,
        /// Run a dedicated instance.
        Dedicated = 2,
        /// Launch this instance to a dedicated host.
        Host = 3,
    }
}
/// Request message for `AwsClusters.CreateAwsCluster` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateAwsClusterRequest {
    /// Required. The parent location where this \[AwsCluster][google.cloud.gkemulticloud.v1.AwsCluster\] resource
    /// will be created.
    ///
    /// Location names are formatted as `projects/<project-id>/locations/<region>`.
    ///
    /// See [Resource Names](<https://cloud.google.com/apis/design/resource_names>)
    /// for more details on Google Cloud resource names.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The specification of the \[AwsCluster][google.cloud.gkemulticloud.v1.AwsCluster\] to create.
    #[prost(message, optional, tag = "2")]
    pub aws_cluster: ::core::option::Option<AwsCluster>,
    /// Required. A client provided ID the resource. Must be unique within the parent
    /// resource.
    ///
    /// The provided ID will be part of the \[AwsCluster][google.cloud.gkemulticloud.v1.AwsCluster\]
    /// resource name formatted as
    /// `projects/<project-id>/locations/<region>/awsClusters/<cluster-id>`.
    ///
    /// Valid characters are `/\[a-z][0-9\]-/`. Cannot be longer than 40 characters.
    #[prost(string, tag = "3")]
    pub aws_cluster_id: ::prost::alloc::string::String,
    /// If set, only validate the request, but do not actually create the cluster.
    #[prost(bool, tag = "4")]
    pub validate_only: bool,
}
/// Request message for `AwsClusters.UpdateAwsCluster` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateAwsClusterRequest {
    /// Required. The \[AwsCluster][google.cloud.gkemulticloud.v1.AwsCluster\] resource to update.
    #[prost(message, optional, tag = "1")]
    pub aws_cluster: ::core::option::Option<AwsCluster>,
    /// If set, only validate the request, but do not actually update the cluster.
    #[prost(bool, tag = "2")]
    pub validate_only: bool,
    /// Required. Mask of fields to update. At least one path must be supplied in
    /// this field. The elements of the repeated paths field can only include these
    /// fields from \[AwsCluster][google.cloud.gkemulticloud.v1.AwsCluster\]:
    ///
    ///  *   `description`.
    ///  *   `annotations`.
    ///  *   `control_plane.version`.
    ///  *   `authorization.admin_users`.
    ///  *   `control_plane.aws_services_authentication.role_arn`.
    ///  *   `control_plane.aws_services_authentication.role_session_name`.
    ///  *   `control_plane.config_encryption.kms_key_arn`.
    ///  *   `control_plane.instance_type`.
    ///  *   `control_plane.security_group_ids`.
    ///  *   `control_plane.proxy_config`.
    ///  *   `control_plane.proxy_config.secret_arn`.
    ///  *   `control_plane.proxy_config.secret_version`.
    ///  *   `control_plane.root_volume.iops`.
    ///  *   `control_plane.root_volume.kms_key_arn`.
    ///  *   `control_plane.root_volume.volume_type`.
    ///  *   `control_plane.root_volume.size_gib`.
    ///  *   `control_plane.ssh_config`.
    ///  *   `control_plane.ssh_config.ec2_key_pair`.
    ///  *   `control_plane.instance_placement.tenancy`.
    ///  *   `logging_config`.
    #[prost(message, optional, tag = "4")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for `AwsClusters.GetAwsCluster` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAwsClusterRequest {
    /// Required. The name of the \[AwsCluster][google.cloud.gkemulticloud.v1.AwsCluster\] resource to describe.
    ///
    /// `AwsCluster` names are formatted as
    /// `projects/<project-id>/locations/<region>/awsClusters/<cluster-id>`.
    ///
    /// See [Resource Names](<https://cloud.google.com/apis/design/resource_names>)
    /// for more details on GCP resource names.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for `AwsClusters.ListAwsClusters` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAwsClustersRequest {
    /// Required. The parent location which owns this collection of
    /// \[AwsCluster][google.cloud.gkemulticloud.v1.AwsCluster\] resources.
    ///
    /// Location names are formatted as `projects/<project-id>/locations/<region>`.
    ///
    /// See [Resource Names](<https://cloud.google.com/apis/design/resource_names>)
    /// for more details on GCP resource names.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of items to return.
    ///
    /// If not specified, a default value of 50 will be used by the service.
    /// Regardless of the pageSize value, the response can include a partial list
    /// and a caller should only rely on response's
    /// \[nextPageToken][google.cloud.gkemulticloud.v1.ListAwsClustersResponse.next_page_token\] to determine if
    /// there are more instances left to be queried.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The `nextPageToken` value returned from a previous
    /// \[awsClusters.list][google.cloud.gkemulticloud.v1.AwsClusters.ListAwsClusters\] request, if any.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for `AwsClusters.ListAwsClusters` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAwsClustersResponse {
    /// A list of \[AwsCluster][google.cloud.gkemulticloud.v1.AwsCluster\] resources in the specified GCP
    /// project and region region.
    #[prost(message, repeated, tag = "1")]
    pub aws_clusters: ::prost::alloc::vec::Vec<AwsCluster>,
    /// Token to retrieve the next page of results, or empty if there are no more
    /// results in the list.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for `AwsClusters.DeleteAwsCluster` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteAwsClusterRequest {
    /// Required. The resource name the \[AwsCluster][google.cloud.gkemulticloud.v1.AwsCluster\] to delete.
    ///
    /// `AwsCluster` names are formatted as
    /// `projects/<project-id>/locations/<region>/awsClusters/<cluster-id>`.
    ///
    /// See [Resource Names](<https://cloud.google.com/apis/design/resource_names>)
    /// for more details on GCP resource names.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// If set, only validate the request, but do not actually delete the resource.
    #[prost(bool, tag = "2")]
    pub validate_only: bool,
    /// If set to true, and the \[AwsCluster][google.cloud.gkemulticloud.v1.AwsCluster\] resource is not found,
    /// the request will succeed but no action will be taken on the server and a
    /// completed \[Operation][google.longrunning.Operation\] will be returned.
    ///
    /// Useful for idempotent deletion.
    #[prost(bool, tag = "3")]
    pub allow_missing: bool,
    /// The current etag of the \[AwsCluster][google.cloud.gkemulticloud.v1.AwsCluster\].
    ///
    /// Allows clients to perform deletions through optimistic concurrency control.
    ///
    /// If the provided etag does not match the current etag of the cluster,
    /// the request will fail and an ABORTED error will be returned.
    #[prost(string, tag = "4")]
    pub etag: ::prost::alloc::string::String,
}
/// Response message for `AwsClusters.CreateAwsNodePool` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateAwsNodePoolRequest {
    /// Required. The \[AwsCluster][google.cloud.gkemulticloud.v1.AwsCluster\] resource where this node pool will be created.
    ///
    /// `AwsCluster` names are formatted as
    /// `projects/<project-id>/locations/<region>/awsClusters/<cluster-id>`.
    ///
    /// See [Resource Names](<https://cloud.google.com/apis/design/resource_names>)
    /// for more details on Google Cloud resource names.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The specification of the \[AwsNodePool][google.cloud.gkemulticloud.v1.AwsNodePool\] to create.
    #[prost(message, optional, tag = "2")]
    pub aws_node_pool: ::core::option::Option<AwsNodePool>,
    /// Required. A client provided ID the resource. Must be unique within the parent
    /// resource.
    ///
    /// The provided ID will be part of the \[AwsNodePool][google.cloud.gkemulticloud.v1.AwsNodePool\]
    /// resource name formatted as
    /// `projects/<project-id>/locations/<region>/awsClusters/<cluster-id>/awsNodePools/<node-pool-id>`.
    ///
    /// Valid characters are `/\[a-z][0-9\]-/`. Cannot be longer than 40 characters.
    #[prost(string, tag = "3")]
    pub aws_node_pool_id: ::prost::alloc::string::String,
    /// If set, only validate the request, but do not actually create the node
    /// pool.
    #[prost(bool, tag = "4")]
    pub validate_only: bool,
}
/// Request message for `AwsClusters.UpdateAwsNodePool` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateAwsNodePoolRequest {
    /// Required. The \[AwsNodePool][google.cloud.gkemulticloud.v1.AwsNodePool\] resource to update.
    #[prost(message, optional, tag = "1")]
    pub aws_node_pool: ::core::option::Option<AwsNodePool>,
    /// If set, only validate the request, but don't actually update the node pool.
    #[prost(bool, tag = "2")]
    pub validate_only: bool,
    /// Required. Mask of fields to update. At least one path must be supplied in
    /// this field. The elements of the repeated paths field can only include these
    /// fields from \[AwsNodePool][google.cloud.gkemulticloud.v1.AwsNodePool\]:
    ///
    ///  *   `annotations`.
    ///  *   `version`.
    ///  *   `autoscaling.min_node_count`.
    ///  *   `autoscaling.max_node_count`.
    ///  *   `config.config_encryption.kms_key_arn`.
    ///  *   `config.security_group_ids`.
    ///  *   `config.root_volume.iops`.
    ///  *   `config.root_volume.kms_key_arn`.
    ///  *   `config.root_volume.volume_type`.
    ///  *   `config.root_volume.size_gib`.
    ///  *   `config.proxy_config`.
    ///  *   `config.proxy_config.secret_arn`.
    ///  *   `config.proxy_config.secret_version`.
    ///  *   `config.ssh_config`.
    ///  *   `config.ssh_config.ec2_key_pair`.
    #[prost(message, optional, tag = "3")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for `AwsClusters.GetAwsNodePool` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAwsNodePoolRequest {
    /// Required. The name of the \[AwsNodePool][google.cloud.gkemulticloud.v1.AwsNodePool\] resource to describe.
    ///
    /// `AwsNodePool` names are formatted as
    /// `projects/<project-id>/locations/<region>/awsClusters/<cluster-id>/awsNodePools/<node-pool-id>`.
    ///
    /// See [Resource Names](<https://cloud.google.com/apis/design/resource_names>)
    /// for more details on Google Cloud resource names.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for `AwsClusters.ListAwsNodePools` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAwsNodePoolsRequest {
    /// Required. The parent `AwsCluster` which owns this collection of
    /// \[AwsNodePool][google.cloud.gkemulticloud.v1.AwsNodePool\] resources.
    ///
    /// `AwsCluster` names are formatted as
    /// `projects/<project-id>/locations/<region>/awsClusters/<cluster-id>`.
    ///
    /// See [Resource Names](<https://cloud.google.com/apis/design/resource_names>)
    /// for more details on Google Cloud resource names.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of items to return.
    ///
    /// If not specified, a default value of 50 will be used by the service.
    /// Regardless of the pageSize value, the response can include a partial list
    /// and a caller should only rely on response's
    /// \[nextPageToken][google.cloud.gkemulticloud.v1.ListAwsNodePoolsResponse.next_page_token\] to determine if
    /// there are more instances left to be queried.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The `nextPageToken` value returned from a previous
    /// \[awsNodePools.list][google.cloud.gkemulticloud.v1.AwsClusters.ListAwsNodePools\] request, if any.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for `AwsClusters.ListAwsNodePools` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAwsNodePoolsResponse {
    /// A list of \[AwsNodePool][google.cloud.gkemulticloud.v1.AwsNodePool\] resources in the specified `AwsCluster`.
    #[prost(message, repeated, tag = "1")]
    pub aws_node_pools: ::prost::alloc::vec::Vec<AwsNodePool>,
    /// Token to retrieve the next page of results, or empty if there are no more
    /// results in the list.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for `AwsClusters.DeleteNodePool` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteAwsNodePoolRequest {
    /// Required. The resource name the \[AwsNodePool][google.cloud.gkemulticloud.v1.AwsNodePool\] to delete.
    ///
    /// `AwsNodePool` names are formatted as
    /// `projects/<project-id>/locations/<region>/awsClusters/<cluster-id>/awsNodePools/<node-pool-id>`.
    ///
    /// See [Resource Names](<https://cloud.google.com/apis/design/resource_names>)
    /// for more details on Google Cloud resource names.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// If set, only validate the request, but do not actually delete the node
    /// pool.
    #[prost(bool, tag = "2")]
    pub validate_only: bool,
    /// If set to true, and the \[AwsNodePool][google.cloud.gkemulticloud.v1.AwsNodePool\] resource is not found,
    /// the request will succeed but no action will be taken on the server and a
    /// completed \[Operation][google.longrunning.Operation\] will be returned.
    ///
    /// Useful for idempotent deletion.
    #[prost(bool, tag = "3")]
    pub allow_missing: bool,
    /// The current ETag of the \[AwsNodePool][google.cloud.gkemulticloud.v1.AwsNodePool\].
    ///
    /// Allows clients to perform deletions through optimistic concurrency control.
    ///
    /// If the provided ETag does not match the current etag of the node pool,
    /// the request will fail and an ABORTED error will be returned.
    #[prost(string, tag = "4")]
    pub etag: ::prost::alloc::string::String,
}
/// GetAwsServerConfigRequest gets the server config of GKE cluster on AWS.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAwsServerConfigRequest {
    /// Required. The name of the \[AwsServerConfig][google.cloud.gkemulticloud.v1.AwsServerConfig\] resource to describe.
    ///
    /// `AwsServerConfig` names are formatted as
    /// `projects/<project-id>/locations/<region>/awsServerConfig`.
    ///
    /// See [Resource Names](<https://cloud.google.com/apis/design/resource_names>)
    /// for more details on Google Cloud resource names.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for `AwsClusters.GenerateAwsAccessToken` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateAwsAccessTokenRequest {
    /// Required. The name of the \[AwsCluster][google.cloud.gkemulticloud.v1.AwsCluster\] resource to authenticate to.
    ///
    /// `AwsCluster` names are formatted as
    /// `projects/<project-id>/locations/<region>/awsClusters/<cluster-id>`.
    ///
    /// See [Resource Names](<https://cloud.google.com/apis/design/resource_names>)
    /// for more details on Google Cloud resource names.
    #[prost(string, tag = "1")]
    pub aws_cluster: ::prost::alloc::string::String,
}
/// Response message for `AwsClusters.GenerateAwsAccessToken` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateAwsAccessTokenResponse {
    /// Output only. Access token to authenticate to k8s api-server.
    #[prost(string, tag = "1")]
    pub access_token: ::prost::alloc::string::String,
    /// Output only. Timestamp at which the token will expire.
    #[prost(message, optional, tag = "2")]
    pub expiration_time: ::core::option::Option<::prost_types::Timestamp>,
}
#[doc = r" Generated client implementations."]
pub mod aws_clusters_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " The AwsClusters API provides a single centrally managed service"]
    #[doc = " to create and manage Anthos clusters that run on AWS infrastructure."]
    #[derive(Debug, Clone)]
    pub struct AwsClustersClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> AwsClustersClient<T>
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
        ) -> AwsClustersClient<InterceptedService<T, F>>
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
            AwsClustersClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Creates a new [AwsCluster][google.cloud.gkemulticloud.v1.AwsCluster] resource on a given GCP project and region."]
        #[doc = ""]
        #[doc = " If successful, the response contains a newly created"]
        #[doc = " [Operation][google.longrunning.Operation] resource that can be"]
        #[doc = " described to track the status of the operation."]
        pub async fn create_aws_cluster(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateAwsClusterRequest>,
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
                "/google.cloud.gkemulticloud.v1.AwsClusters/CreateAwsCluster",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates an [AwsCluster][google.cloud.gkemulticloud.v1.AwsCluster]."]
        pub async fn update_aws_cluster(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateAwsClusterRequest>,
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
                "/google.cloud.gkemulticloud.v1.AwsClusters/UpdateAwsCluster",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Describes a specific [AwsCluster][google.cloud.gkemulticloud.v1.AwsCluster] resource."]
        pub async fn get_aws_cluster(
            &mut self,
            request: impl tonic::IntoRequest<super::GetAwsClusterRequest>,
        ) -> Result<tonic::Response<super::AwsCluster>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.gkemulticloud.v1.AwsClusters/GetAwsCluster",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists all [AwsCluster][google.cloud.gkemulticloud.v1.AwsCluster] resources on a given Google Cloud project and"]
        #[doc = " region."]
        pub async fn list_aws_clusters(
            &mut self,
            request: impl tonic::IntoRequest<super::ListAwsClustersRequest>,
        ) -> Result<tonic::Response<super::ListAwsClustersResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.gkemulticloud.v1.AwsClusters/ListAwsClusters",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes a specific [AwsCluster][google.cloud.gkemulticloud.v1.AwsCluster] resource."]
        #[doc = ""]
        #[doc = " Fails if the cluster has one or more associated [AwsNodePool][google.cloud.gkemulticloud.v1.AwsNodePool]"]
        #[doc = " resources."]
        #[doc = ""]
        #[doc = " If successful, the response contains a newly created"]
        #[doc = " [Operation][google.longrunning.Operation] resource that can be"]
        #[doc = " described to track the status of the operation."]
        pub async fn delete_aws_cluster(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteAwsClusterRequest>,
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
                "/google.cloud.gkemulticloud.v1.AwsClusters/DeleteAwsCluster",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Generates a short-lived access token to authenticate to a given"]
        #[doc = " [AwsCluster][google.cloud.gkemulticloud.v1.AwsCluster] resource."]
        pub async fn generate_aws_access_token(
            &mut self,
            request: impl tonic::IntoRequest<super::GenerateAwsAccessTokenRequest>,
        ) -> Result<tonic::Response<super::GenerateAwsAccessTokenResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.gkemulticloud.v1.AwsClusters/GenerateAwsAccessToken",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a new [AwsNodePool][google.cloud.gkemulticloud.v1.AwsNodePool], attached to a given [AwsCluster][google.cloud.gkemulticloud.v1.AwsCluster]."]
        #[doc = ""]
        #[doc = " If successful, the response contains a newly created"]
        #[doc = " [Operation][google.longrunning.Operation] resource that can be"]
        #[doc = " described to track the status of the operation."]
        pub async fn create_aws_node_pool(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateAwsNodePoolRequest>,
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
                "/google.cloud.gkemulticloud.v1.AwsClusters/CreateAwsNodePool",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates an [AwsNodePool][google.cloud.gkemulticloud.v1.AwsNodePool]."]
        pub async fn update_aws_node_pool(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateAwsNodePoolRequest>,
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
                "/google.cloud.gkemulticloud.v1.AwsClusters/UpdateAwsNodePool",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Describes a specific [AwsNodePool][google.cloud.gkemulticloud.v1.AwsNodePool] resource."]
        pub async fn get_aws_node_pool(
            &mut self,
            request: impl tonic::IntoRequest<super::GetAwsNodePoolRequest>,
        ) -> Result<tonic::Response<super::AwsNodePool>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.gkemulticloud.v1.AwsClusters/GetAwsNodePool",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists all [AwsNodePool][google.cloud.gkemulticloud.v1.AwsNodePool] resources on a given [AwsCluster][google.cloud.gkemulticloud.v1.AwsCluster]."]
        pub async fn list_aws_node_pools(
            &mut self,
            request: impl tonic::IntoRequest<super::ListAwsNodePoolsRequest>,
        ) -> Result<tonic::Response<super::ListAwsNodePoolsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.gkemulticloud.v1.AwsClusters/ListAwsNodePools",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes a specific [AwsNodePool][google.cloud.gkemulticloud.v1.AwsNodePool] resource."]
        #[doc = ""]
        #[doc = " If successful, the response contains a newly created"]
        #[doc = " [Operation][google.longrunning.Operation] resource that can be"]
        #[doc = " described to track the status of the operation."]
        pub async fn delete_aws_node_pool(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteAwsNodePoolRequest>,
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
                "/google.cloud.gkemulticloud.v1.AwsClusters/DeleteAwsNodePool",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns information, such as supported AWS regions and Kubernetes"]
        #[doc = " versions, on a given Google Cloud location."]
        pub async fn get_aws_server_config(
            &mut self,
            request: impl tonic::IntoRequest<super::GetAwsServerConfigRequest>,
        ) -> Result<tonic::Response<super::AwsServerConfig>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.gkemulticloud.v1.AwsClusters/GetAwsServerConfig",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// An Anthos cluster running on Azure.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AzureCluster {
    /// The name of this resource.
    ///
    /// Cluster names are formatted as
    /// `projects/<project-number>/locations/<region>/azureClusters/<cluster-id>`.
    ///
    /// See [Resource Names](<https://cloud.google.com/apis/design/resource_names>)
    /// for more details on GCP resource names.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. A human readable description of this cluster.
    /// Cannot be longer than 255 UTF-8 encoded bytes.
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// Required. The Azure region where the cluster runs.
    ///
    /// Each Google Cloud region supports a subset of nearby Azure regions.
    /// You can call
    /// \[GetAzureServerConfig][google.cloud.gkemulticloud.v1.AzureClusters.GetAzureServerConfig\]
    /// to list all supported Azure regions within a given Google Cloud region.
    #[prost(string, tag = "3")]
    pub azure_region: ::prost::alloc::string::String,
    /// Required. The ARM ID of the resource group where the cluster resources are deployed.
    /// For example:
    /// `/subscriptions/<subscription-id>/resourceGroups/<resource-group-name>`
    #[prost(string, tag = "17")]
    pub resource_group_id: ::prost::alloc::string::String,
    /// Required. Name of the \[AzureClient][google.cloud.gkemulticloud.v1.AzureClient\] that contains authentication configuration for
    /// how the Anthos Multi-Cloud API connects to Azure APIs.
    ///
    /// The `AzureClient` resource must reside on the same GCP project and region
    /// as the `AzureCluster`.
    ///
    /// `AzureClient` names are formatted as
    /// `projects/<project-number>/locations/<region>/azureClients/<client-id>`.
    ///
    /// See [Resource Names](<https://cloud.google.com/apis/design/resource_names>)
    /// for more details on Google Cloud resource names.
    #[prost(string, tag = "16")]
    pub azure_client: ::prost::alloc::string::String,
    /// Required. Cluster-wide networking configuration.
    #[prost(message, optional, tag = "4")]
    pub networking: ::core::option::Option<AzureClusterNetworking>,
    /// Required. Configuration related to the cluster control plane.
    #[prost(message, optional, tag = "5")]
    pub control_plane: ::core::option::Option<AzureControlPlane>,
    /// Required. Configuration related to the cluster RBAC settings.
    #[prost(message, optional, tag = "6")]
    pub authorization: ::core::option::Option<AzureAuthorization>,
    /// Output only. The current state of the cluster.
    #[prost(enumeration = "azure_cluster::State", tag = "7")]
    pub state: i32,
    /// Output only. The endpoint of the cluster's API server.
    #[prost(string, tag = "8")]
    pub endpoint: ::prost::alloc::string::String,
    /// Output only. A globally unique identifier for the cluster.
    #[prost(string, tag = "9")]
    pub uid: ::prost::alloc::string::String,
    /// Output only. If set, there are currently changes in flight to the cluster.
    #[prost(bool, tag = "10")]
    pub reconciling: bool,
    /// Output only. The time at which this cluster was created.
    #[prost(message, optional, tag = "11")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The time at which this cluster was last updated.
    #[prost(message, optional, tag = "12")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Allows clients to perform consistent read-modify-writes
    /// through optimistic concurrency control.
    ///
    /// Can be sent on update and delete requests to ensure the
    /// client has an up-to-date value before proceeding.
    #[prost(string, tag = "13")]
    pub etag: ::prost::alloc::string::String,
    /// Optional. Annotations on the cluster.
    ///
    /// This field has the same restrictions as Kubernetes annotations.
    /// The total size of all keys and values combined is limited to 256k.
    /// Keys can have 2 segments: prefix (optional) and name (required),
    /// separated by a slash (/).
    /// Prefix must be a DNS subdomain.
    /// Name must be 63 characters or less, begin and end with alphanumerics,
    /// with dashes (-), underscores (_), dots (.), and alphanumerics between.
    #[prost(map = "string, string", tag = "14")]
    pub annotations:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Output only. Workload Identity settings.
    #[prost(message, optional, tag = "18")]
    pub workload_identity_config: ::core::option::Option<WorkloadIdentityConfig>,
    /// Output only. PEM encoded x509 certificate of the cluster root of trust.
    #[prost(string, tag = "19")]
    pub cluster_ca_certificate: ::prost::alloc::string::String,
    /// Optional. Fleet configuration.
    #[prost(message, optional, tag = "20")]
    pub fleet: ::core::option::Option<Fleet>,
    /// Output only. Mananged Azure resources for this cluster.
    #[prost(message, optional, tag = "21")]
    pub managed_resources: ::core::option::Option<AzureClusterResources>,
    /// Optional. Logging configuration for this cluster.
    #[prost(message, optional, tag = "23")]
    pub logging_config: ::core::option::Option<LoggingConfig>,
}
/// Nested message and enum types in `AzureCluster`.
pub mod azure_cluster {
    /// The lifecycle state of the cluster.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// Not set.
        Unspecified = 0,
        /// The PROVISIONING state indicates the cluster is being created.
        Provisioning = 1,
        /// The RUNNING state indicates the cluster has been created and is fully
        /// usable.
        Running = 2,
        /// The RECONCILING state indicates that some work is actively being done on
        /// the cluster, such as upgrading the control plane replicas.
        Reconciling = 3,
        /// The STOPPING state indicates the cluster is being deleted.
        Stopping = 4,
        /// The ERROR state indicates the cluster is in a broken unrecoverable
        /// state.
        Error = 5,
        /// The DEGRADED state indicates the cluster requires user action to
        /// restore full functionality.
        Degraded = 6,
    }
}
/// ClusterNetworking contains cluster-wide networking configuration.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AzureClusterNetworking {
    /// Required. The Azure Resource Manager (ARM) ID of the VNet associated with your
    /// cluster.
    ///
    /// All components in the cluster (i.e. control plane and node pools) run on a
    /// single VNet.
    ///
    /// Example:
    /// `/subscriptions/<subscription-id>/resourceGroups/<resource-group-id>/providers/Microsoft.Network/virtualNetworks/<vnet-id>`
    ///
    /// This field cannot be changed after creation.
    #[prost(string, tag = "1")]
    pub virtual_network_id: ::prost::alloc::string::String,
    /// Required. The IP address range of the pods in this cluster, in CIDR
    /// notation (e.g. `10.96.0.0/14`).
    ///
    /// All pods in the cluster get assigned a unique IPv4 address from these
    /// ranges. Only a single range is supported.
    ///
    /// This field cannot be changed after creation.
    #[prost(string, repeated, tag = "2")]
    pub pod_address_cidr_blocks: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Required. The IP address range for services in this cluster, in CIDR
    /// notation (e.g. `10.96.0.0/14`).
    ///
    /// All services in the cluster get assigned a unique IPv4 address from these
    /// ranges. Only a single range is supported.
    ///
    /// This field cannot be changed after creating a cluster.
    #[prost(string, repeated, tag = "3")]
    pub service_address_cidr_blocks: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Optional. The ARM ID of the subnet where Kubernetes private service type load
    /// balancers are deployed. When unspecified, it defaults to
    /// AzureControlPlane.subnet_id.
    ///
    /// Example:
    /// "/subscriptions/d00494d6-6f3c-4280-bbb2-899e163d1d30/resourceGroups/anthos_cluster_gkeust4/providers/Microsoft.Network/virtualNetworks/gke-vnet-gkeust4/subnets/subnetid456"
    #[prost(string, tag = "5")]
    pub service_load_balancer_subnet_id: ::prost::alloc::string::String,
}
/// AzureControlPlane represents the control plane configurations.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AzureControlPlane {
    /// Required. The Kubernetes version to run on control plane replicas
    /// (e.g. `1.19.10-gke.1000`).
    ///
    /// You can list all supported versions on a given Google Cloud region by
    /// calling
    /// \[GetAzureServerConfig][google.cloud.gkemulticloud.v1.AzureClusters.GetAzureServerConfig\].
    #[prost(string, tag = "1")]
    pub version: ::prost::alloc::string::String,
    /// Optional. The ARM ID of the default subnet for the control plane. The control plane
    /// VMs are deployed in this subnet, unless
    /// `AzureControlPlane.replica_placements` is specified. This subnet will also
    /// be used as default for `AzureControlPlane.endpoint_subnet_id` if
    /// `AzureControlPlane.endpoint_subnet_id` is not specified. Similarly it will
    /// be used as default for
    /// `AzureClusterNetworking.service_load_balancer_subnet_id`.
    ///
    /// Example:
    /// `/subscriptions/<subscription-id>/resourceGroups/<resource-group-id>/providers/Microsoft.Network/virtualNetworks/<vnet-id>/subnets/default`.
    #[prost(string, tag = "2")]
    pub subnet_id: ::prost::alloc::string::String,
    /// Optional. The Azure VM size name. Example: `Standard_DS2_v2`.
    ///
    /// For available VM sizes, see
    /// <https://docs.microsoft.com/en-us/azure/virtual-machines/vm-naming-conventions.>
    ///
    /// When unspecified, it defaults to `Standard_DS2_v2`.
    #[prost(string, tag = "3")]
    pub vm_size: ::prost::alloc::string::String,
    /// Required. SSH configuration for how to access the underlying control plane
    /// machines.
    #[prost(message, optional, tag = "11")]
    pub ssh_config: ::core::option::Option<AzureSshConfig>,
    /// Optional. Configuration related to the root volume provisioned for each
    /// control plane replica.
    ///
    /// When unspecified, it defaults to 32-GiB Azure Disk.
    #[prost(message, optional, tag = "4")]
    pub root_volume: ::core::option::Option<AzureDiskTemplate>,
    /// Optional. Configuration related to the main volume provisioned for each
    /// control plane replica.
    /// The main volume is in charge of storing all of the cluster's etcd state.
    ///
    /// When unspecified, it defaults to a 8-GiB Azure Disk.
    #[prost(message, optional, tag = "5")]
    pub main_volume: ::core::option::Option<AzureDiskTemplate>,
    /// Optional. Configuration related to application-layer secrets encryption.
    #[prost(message, optional, tag = "10")]
    pub database_encryption: ::core::option::Option<AzureDatabaseEncryption>,
    /// Optional. Proxy configuration for outbound HTTP(S) traffic.
    #[prost(message, optional, tag = "12")]
    pub proxy_config: ::core::option::Option<AzureProxyConfig>,
    /// Optional. Configuration related to vm config encryption.
    #[prost(message, optional, tag = "14")]
    pub config_encryption: ::core::option::Option<AzureConfigEncryption>,
    /// Optional. A set of tags to apply to all underlying control plane Azure resources.
    #[prost(map = "string, string", tag = "7")]
    pub tags:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Optional. Configuration for where to place the control plane replicas.
    ///
    /// Up to three replica placement instances can be specified. If
    /// replica_placements is set, the replica placement instances will be applied
    /// to the three control plane replicas as evenly as possible.
    #[prost(message, repeated, tag = "13")]
    pub replica_placements: ::prost::alloc::vec::Vec<ReplicaPlacement>,
    /// Optional. The ARM ID of the subnet where the control plane load balancer is deployed.
    /// When unspecified, it defaults to AzureControlPlane.subnet_id.
    ///
    /// Example:
    /// "/subscriptions/d00494d6-6f3c-4280-bbb2-899e163d1d30/resourceGroups/anthos_cluster_gkeust4/providers/Microsoft.Network/virtualNetworks/gke-vnet-gkeust4/subnets/subnetid123"
    #[prost(string, tag = "15")]
    pub endpoint_subnet_id: ::prost::alloc::string::String,
}
/// Configuration for the placement of a control plane replica.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReplicaPlacement {
    /// Required. For a given replica, the ARM ID of the subnet where the control plane VM is
    /// deployed. Make sure it's a subnet under the virtual network in the cluster
    /// configuration.
    #[prost(string, tag = "1")]
    pub subnet_id: ::prost::alloc::string::String,
    /// Required. For a given replica, the Azure availability zone where to provision the
    /// control plane VM and the ETCD disk.
    #[prost(string, tag = "2")]
    pub azure_availability_zone: ::prost::alloc::string::String,
}
/// Details of a proxy config stored in Azure Key Vault.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AzureProxyConfig {
    /// The ARM ID the of the resource group containing proxy keyvault.
    ///
    /// Resource group ids are formatted as
    /// `/subscriptions/<subscription-id>/resourceGroups/<resource-group-name>`.
    #[prost(string, tag = "1")]
    pub resource_group_id: ::prost::alloc::string::String,
    /// The URL the of the proxy setting secret with its version.
    ///
    /// Secret ids are formatted as
    /// `<https://<key-vault-name>.vault.azure.net/secrets/<secret-name>/<secret-version>`.>
    #[prost(string, tag = "2")]
    pub secret_id: ::prost::alloc::string::String,
}
/// Configuration related to application-layer secrets encryption.
///
/// Anthos clusters on Azure encrypts your Kubernetes data at rest
/// in etcd using Azure Key Vault.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AzureDatabaseEncryption {
    /// Required. The ARM ID of the Azure Key Vault key to encrypt / decrypt data.
    ///
    /// For example:
    /// `/subscriptions/<subscription-id>/resourceGroups/<resource-group-id>/providers/Microsoft.KeyVault/vaults/<key-vault-id>/keys/<key-name>`
    /// Encryption will always take the latest version of the key and hence
    /// specific version is not supported.
    #[prost(string, tag = "3")]
    pub key_id: ::prost::alloc::string::String,
}
/// Configuration related to config data encryption.
///
/// Azure VM bootstrap secret is envelope encrypted with the provided key vault
/// key.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AzureConfigEncryption {
    /// Required. The ARM ID of the Azure Key Vault key to encrypt / decrypt config data.
    ///
    /// For example:
    /// `/subscriptions/<subscription-id>/resourceGroups/<resource-group-id>/providers/Microsoft.KeyVault/vaults/<key-vault-id>/keys/<key-name>`
    #[prost(string, tag = "2")]
    pub key_id: ::prost::alloc::string::String,
    /// Optional. RSA key of the Azure Key Vault public key to use for encrypting the data.
    ///
    /// This key must be formatted as a PEM-encoded SubjectPublicKeyInfo (RFC 5280)
    /// in ASN.1 DER form. The string must be comprised of a single PEM block of
    /// type "PUBLIC KEY".
    #[prost(string, tag = "3")]
    pub public_key: ::prost::alloc::string::String,
}
/// Configuration for Azure Disks.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AzureDiskTemplate {
    /// Optional. The size of the disk, in GiBs.
    ///
    /// When unspecified, a default value is provided. See the specific reference
    /// in the parent resource.
    #[prost(int32, tag = "1")]
    pub size_gib: i32,
}
/// `AzureClient` resources hold client authentication information needed by the
/// Anthos Multi-Cloud API to manage Azure resources on your Azure subscription.
///
/// When an \[AzureCluster][google.cloud.gkemulticloud.v1.AzureCluster\] is created, an `AzureClient` resource needs to be
/// provided and all operations on Azure resources associated to that cluster
/// will authenticate to Azure services using the given client.
///
/// `AzureClient` resources are immutable and cannot be modified upon creation.
///
/// Each `AzureClient` resource is bound to a single Azure Active Directory
/// Application and tenant.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AzureClient {
    /// The name of this resource.
    ///
    /// `AzureClient` resource names are formatted as
    /// `projects/<project-number>/locations/<region>/azureClients/<client-id>`.
    ///
    /// See [Resource Names](<https://cloud.google.com/apis/design/resource_names>)
    /// for more details on Google Cloud resource names.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The Azure Active Directory Tenant ID.
    #[prost(string, tag = "2")]
    pub tenant_id: ::prost::alloc::string::String,
    /// Required. The Azure Active Directory Application ID.
    #[prost(string, tag = "3")]
    pub application_id: ::prost::alloc::string::String,
    /// Optional. Annotations on the resource.
    ///
    /// This field has the same restrictions as Kubernetes annotations.
    /// The total size of all keys and values combined is limited to 256k.
    /// Keys can have 2 segments: prefix (optional) and name (required),
    /// separated by a slash (/).
    /// Prefix must be a DNS subdomain.
    /// Name must be 63 characters or less, begin and end with alphanumerics,
    /// with dashes (-), underscores (_), dots (.), and alphanumerics between.
    #[prost(map = "string, string", tag = "8")]
    pub annotations:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Output only. The PEM encoded x509 certificate.
    #[prost(string, tag = "7")]
    pub pem_certificate: ::prost::alloc::string::String,
    /// Output only. A globally unique identifier for the client.
    #[prost(string, tag = "5")]
    pub uid: ::prost::alloc::string::String,
    /// Output only. The time at which this resource was created.
    #[prost(message, optional, tag = "6")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Configuration related to the cluster RBAC settings.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AzureAuthorization {
    /// Required. Users that can perform operations as a cluster admin. A managed
    /// ClusterRoleBinding will be created to grant the `cluster-admin` ClusterRole
    /// to the users. Up to ten admin users can be provided.
    ///
    /// For more info on RBAC, see
    /// <https://kubernetes.io/docs/reference/access-authn-authz/rbac/#user-facing-roles>
    #[prost(message, repeated, tag = "1")]
    pub admin_users: ::prost::alloc::vec::Vec<AzureClusterUser>,
}
/// Identities of a user-type subject for Azure clusters.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AzureClusterUser {
    /// Required. The name of the user, e.g. `my-gcp-id@gmail.com`.
    #[prost(string, tag = "1")]
    pub username: ::prost::alloc::string::String,
}
/// An Anthos node pool running on Azure.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AzureNodePool {
    /// The name of this resource.
    ///
    /// Node pool names are formatted as
    /// `projects/<project-number>/locations/<region>/azureClusters/<cluster-id>/azureNodePools/<node-pool-id>`.
    ///
    /// For more details on Google Cloud resource names,
    /// see [Resource Names](<https://cloud.google.com/apis/design/resource_names>)
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The Kubernetes version (e.g. `1.19.10-gke.1000`) running on this node pool.
    #[prost(string, tag = "2")]
    pub version: ::prost::alloc::string::String,
    /// Required. The node configuration of the node pool.
    #[prost(message, optional, tag = "22")]
    pub config: ::core::option::Option<AzureNodeConfig>,
    /// Required. The ARM ID of the subnet where the node pool VMs run. Make sure it's a
    /// subnet under the virtual network in the cluster configuration.
    #[prost(string, tag = "3")]
    pub subnet_id: ::prost::alloc::string::String,
    /// Required. Autoscaler configuration for this node pool.
    #[prost(message, optional, tag = "4")]
    pub autoscaling: ::core::option::Option<AzureNodePoolAutoscaling>,
    /// Output only. The current state of the node pool.
    #[prost(enumeration = "azure_node_pool::State", tag = "6")]
    pub state: i32,
    /// Output only. A globally unique identifier for the node pool.
    #[prost(string, tag = "8")]
    pub uid: ::prost::alloc::string::String,
    /// Output only. If set, there are currently pending changes to the node
    /// pool.
    #[prost(bool, tag = "9")]
    pub reconciling: bool,
    /// Output only. The time at which this node pool was created.
    #[prost(message, optional, tag = "10")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The time at which this node pool was last updated.
    #[prost(message, optional, tag = "11")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Allows clients to perform consistent read-modify-writes
    /// through optimistic concurrency control.
    ///
    /// Can be sent on update and delete requests to ensure the
    /// client has an up-to-date value before proceeding.
    #[prost(string, tag = "12")]
    pub etag: ::prost::alloc::string::String,
    /// Optional. Annotations on the node pool.
    ///
    /// This field has the same restrictions as Kubernetes annotations.
    /// The total size of all keys and values combined is limited to 256k.
    /// Keys can have 2 segments: prefix (optional) and name (required),
    /// separated by a slash (/).
    /// Prefix must be a DNS subdomain.
    /// Name must be 63 characters or less, begin and end with alphanumerics,
    /// with dashes (-), underscores (_), dots (.), and alphanumerics between.
    #[prost(map = "string, string", tag = "13")]
    pub annotations:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Required. The constraint on the maximum number of pods that can be run
    /// simultaneously on a node in the node pool.
    #[prost(message, optional, tag = "21")]
    pub max_pods_constraint: ::core::option::Option<MaxPodsConstraint>,
    /// Optional. The Azure availability zone of the nodes in this nodepool.
    ///
    /// When unspecified, it defaults to `1`.
    #[prost(string, tag = "23")]
    pub azure_availability_zone: ::prost::alloc::string::String,
}
/// Nested message and enum types in `AzureNodePool`.
pub mod azure_node_pool {
    /// The lifecycle state of the node pool.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// Not set.
        Unspecified = 0,
        /// The PROVISIONING state indicates the node pool is being created.
        Provisioning = 1,
        /// The RUNNING state indicates the node pool has been created and is fully
        /// usable.
        Running = 2,
        /// The RECONCILING state indicates that the node pool is being reconciled.
        Reconciling = 3,
        /// The STOPPING state indicates the node pool is being deleted.
        Stopping = 4,
        /// The ERROR state indicates the node pool is in a broken unrecoverable
        /// state.
        Error = 5,
        /// The DEGRADED state indicates the node pool requires user action to
        /// restore full functionality.
        Degraded = 6,
    }
}
/// Parameters that describe the configuration of all node machines
/// on a given node pool.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AzureNodeConfig {
    /// Optional. The Azure VM size name. Example: `Standard_DS2_v2`.
    ///
    /// See [Supported VM
    /// sizes](/anthos/clusters/docs/azure/reference/supported-vms) for options.
    ///
    /// When unspecified, it defaults to `Standard_DS2_v2`.
    #[prost(string, tag = "1")]
    pub vm_size: ::prost::alloc::string::String,
    /// Optional. Configuration related to the root volume provisioned for each
    /// node pool machine.
    ///
    /// When unspecified, it defaults to a 32-GiB Azure Disk.
    #[prost(message, optional, tag = "2")]
    pub root_volume: ::core::option::Option<AzureDiskTemplate>,
    /// Optional. A set of tags to apply to all underlying Azure resources for this node
    /// pool. This currently only includes Virtual Machine Scale Sets.
    ///
    /// Specify at most 50 pairs containing alphanumerics, spaces, and symbols
    /// (.+-=_:@/). Keys can be up to 127 Unicode characters. Values can be up to
    /// 255 Unicode characters.
    #[prost(map = "string, string", tag = "3")]
    pub tags:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Optional. The OS image type to use on node pool instances.
    /// Can have a value of `ubuntu`, or `windows` if the cluster enables
    /// the Windows node pool preview feature.
    ///
    /// When unspecified, it defaults to `ubuntu`.
    #[prost(string, tag = "8")]
    pub image_type: ::prost::alloc::string::String,
    /// Required. SSH configuration for how to access the node pool machines.
    #[prost(message, optional, tag = "7")]
    pub ssh_config: ::core::option::Option<AzureSshConfig>,
    /// Optional. Proxy configuration for outbound HTTP(S) traffic.
    #[prost(message, optional, tag = "9")]
    pub proxy_config: ::core::option::Option<AzureProxyConfig>,
    /// Optional. Configuration related to vm config encryption.
    #[prost(message, optional, tag = "12")]
    pub config_encryption: ::core::option::Option<AzureConfigEncryption>,
    /// Optional. The initial taints assigned to nodes of this node pool.
    #[prost(message, repeated, tag = "10")]
    pub taints: ::prost::alloc::vec::Vec<NodeTaint>,
    /// Optional. The initial labels assigned to nodes of this node pool. An object
    /// containing a list of "key": value pairs. Example: { "name": "wrench",
    /// "mass": "1.3kg", "count": "3" }.
    #[prost(map = "string, string", tag = "11")]
    pub labels:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
}
/// Configuration related to Kubernetes cluster autoscaler.
///
/// The Kubernetes cluster autoscaler will automatically adjust the
/// size of the node pool based on the cluster load.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AzureNodePoolAutoscaling {
    /// Required. Minimum number of nodes in the node pool. Must be greater than or equal to
    /// 1 and less than or equal to max_node_count.
    #[prost(int32, tag = "1")]
    pub min_node_count: i32,
    /// Required. Maximum number of nodes in the node pool. Must be greater than or equal to
    /// min_node_count and less than or equal to 50.
    #[prost(int32, tag = "2")]
    pub max_node_count: i32,
}
/// AzureServerConfig contains information about a Google Cloud location, such as
/// supported Azure regions and Kubernetes versions.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AzureServerConfig {
    /// The `AzureServerConfig` resource name.
    ///
    /// `AzureServerConfig` names are formatted as
    /// `projects/<project-number>/locations/<region>/azureServerConfig`.
    ///
    /// See [Resource Names](<https://cloud.google.com/apis/design/resource_names>)
    /// for more details on GCP resource names.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// List of valid Kubernetes versions.
    #[prost(message, repeated, tag = "2")]
    pub valid_versions: ::prost::alloc::vec::Vec<AzureK8sVersionInfo>,
    /// The list of supported Azure regions.
    #[prost(string, repeated, tag = "3")]
    pub supported_azure_regions: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Information about a supported Kubernetes version.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AzureK8sVersionInfo {
    /// A supported Kubernetes version (for example, `1.19.10-gke.1000`)
    #[prost(string, tag = "1")]
    pub version: ::prost::alloc::string::String,
}
/// SSH configuration for Azure resources.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AzureSshConfig {
    /// Required. The SSH public key data for VMs managed by Anthos. This accepts the
    /// authorized_keys file format used in OpenSSH according to the sshd(8) manual
    /// page.
    #[prost(string, tag = "1")]
    pub authorized_key: ::prost::alloc::string::String,
}
/// Managed Azure resources for the cluster.
///
/// The values could change and be empty, depending on the state of the cluster.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AzureClusterResources {
    /// Output only. The ARM ID of the cluster network security group.
    #[prost(string, tag = "1")]
    pub network_security_group_id: ::prost::alloc::string::String,
    /// Output only. The ARM ID of the control plane application security group.
    #[prost(string, tag = "2")]
    pub control_plane_application_security_group_id: ::prost::alloc::string::String,
}
/// Request message for `AzureClusters.CreateAzureCluster` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateAzureClusterRequest {
    /// Required. The parent location where this \[AzureCluster][google.cloud.gkemulticloud.v1.AzureCluster\] resource
    /// will be created.
    ///
    /// Location names are formatted as `projects/<project-id>/locations/<region>`.
    ///
    /// See [Resource Names](<https://cloud.google.com/apis/design/resource_names>)
    /// for more details on Google Cloud resource names.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The specification of the \[AzureCluster][google.cloud.gkemulticloud.v1.AzureCluster\] to create.
    #[prost(message, optional, tag = "2")]
    pub azure_cluster: ::core::option::Option<AzureCluster>,
    /// Required. A client provided ID the resource. Must be unique within the parent
    /// resource.
    ///
    /// The provided ID will be part of the \[AzureCluster][google.cloud.gkemulticloud.v1.AzureCluster\]
    /// resource name formatted as
    /// `projects/<project-id>/locations/<region>/azureClusters/<cluster-id>`.
    ///
    /// Valid characters are `/\[a-z][0-9\]-/`. Cannot be longer than 40 characters.
    #[prost(string, tag = "3")]
    pub azure_cluster_id: ::prost::alloc::string::String,
    /// If set, only validate the request, but do not actually create the cluster.
    #[prost(bool, tag = "4")]
    pub validate_only: bool,
}
/// Request message for `AzureClusters.UpdateAzureCluster` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateAzureClusterRequest {
    /// Required. The \[AzureCluster][google.cloud.gkemulticloud.v1.AzureCluster\] resource to update.
    #[prost(message, optional, tag = "1")]
    pub azure_cluster: ::core::option::Option<AzureCluster>,
    /// If set, only validate the request, but do not actually update the cluster.
    #[prost(bool, tag = "2")]
    pub validate_only: bool,
    /// Required. Mask of fields to update. At least one path must be supplied in
    /// this field. The elements of the repeated paths field can only include these
    /// fields from \[AzureCluster][google.cloud.gkemulticloud.v1.AzureCluster\]:
    ///
    ///  *   `description`.
    ///  *   `annotations`.
    ///  *   `azureClient`.
    ///  *   `control_plane.version`.
    ///  *   `control_plane.vm_size`.
    ///  *   `authorization.admin_users`.
    ///  *   `control_plane.root_volume.size_gib`.
    ///  *   `logging_config`
    #[prost(message, optional, tag = "4")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for `AzureClusters.GetAzureCluster` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAzureClusterRequest {
    /// Required. The name of the \[AzureCluster][google.cloud.gkemulticloud.v1.AzureCluster\] resource to describe.
    ///
    /// `AzureCluster` names are formatted as
    /// `projects/<project-id>/locations/<region>/azureClusters/<cluster-id>`.
    ///
    /// See [Resource Names](<https://cloud.google.com/apis/design/resource_names>)
    /// for more details on GCP resource names.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for `AzureClusters.ListAzureClusters` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAzureClustersRequest {
    /// Required. The parent location which owns this collection of
    /// \[AzureCluster][google.cloud.gkemulticloud.v1.AzureCluster\] resources.
    ///
    /// Location names are formatted as `projects/<project-id>/locations/<region>`.
    ///
    /// See [Resource Names](<https://cloud.google.com/apis/design/resource_names>)
    /// for more details on GCP resource names.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of items to return.
    ///
    /// If not specified, a default value of 50 will be used by the service.
    /// Regardless of the pageSize value, the response can include a partial list
    /// and a caller should only rely on response's
    /// \[nextPageToken][google.cloud.gkemulticloud.v1.ListAzureClustersResponse.next_page_token\] to determine if
    /// there are more instances left to be queried.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The `nextPageToken` value returned from a previous
    /// \[azureClusters.list][google.cloud.gkemulticloud.v1.AzureClusters.ListAzureClusters\] request, if any.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for `AzureClusters.ListAzureClusters` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAzureClustersResponse {
    /// A list of \[AzureCluster][google.cloud.gkemulticloud.v1.AzureCluster\] resources in the specified GCP
    /// project and region region.
    #[prost(message, repeated, tag = "1")]
    pub azure_clusters: ::prost::alloc::vec::Vec<AzureCluster>,
    /// Token to retrieve the next page of results, or empty if there are no more
    /// results in the list.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for `Clusters.DeleteAzureCluster` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteAzureClusterRequest {
    /// Required. The resource name the \[AzureCluster][google.cloud.gkemulticloud.v1.AzureCluster\] to delete.
    ///
    /// `AzureCluster` names are formatted as
    /// `projects/<project-id>/locations/<region>/azureClusters/<cluster-id>`.
    ///
    /// See [Resource Names](<https://cloud.google.com/apis/design/resource_names>)
    /// for more details on GCP resource names.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// If set to true, and the \[AzureCluster][google.cloud.gkemulticloud.v1.AzureCluster\] resource is not found,
    /// the request will succeed but no action will be taken on the server and a
    /// completed \[Operation][google.longrunning.Operation\] will be returned.
    ///
    /// Useful for idempotent deletion.
    #[prost(bool, tag = "2")]
    pub allow_missing: bool,
    /// If set, only validate the request, but do not actually delete the resource.
    #[prost(bool, tag = "3")]
    pub validate_only: bool,
    /// The current etag of the \[AzureCluster][google.cloud.gkemulticloud.v1.AzureCluster\].
    ///
    /// Allows clients to perform deletions through optimistic concurrency control.
    ///
    /// If the provided etag does not match the current etag of the cluster,
    /// the request will fail and an ABORTED error will be returned.
    #[prost(string, tag = "4")]
    pub etag: ::prost::alloc::string::String,
}
/// Response message for `AzureClusters.CreateAzureNodePool` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateAzureNodePoolRequest {
    /// Required. The \[AzureCluster][google.cloud.gkemulticloud.v1.AzureCluster\] resource where this node pool will be created.
    ///
    /// Location names are formatted as `projects/<project-id>/locations/<region>`.
    ///
    /// See [Resource Names](<https://cloud.google.com/apis/design/resource_names>)
    /// for more details on Google Cloud resource names.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The specification of the \[AzureNodePool][google.cloud.gkemulticloud.v1.AzureNodePool\] to create.
    #[prost(message, optional, tag = "2")]
    pub azure_node_pool: ::core::option::Option<AzureNodePool>,
    /// Required. A client provided ID the resource. Must be unique within the parent
    /// resource.
    ///
    /// The provided ID will be part of the \[AzureNodePool][google.cloud.gkemulticloud.v1.AzureNodePool\]
    /// resource name formatted as
    /// `projects/<project-id>/locations/<region>/azureClusters/<cluster-id>/azureNodePools/<node-pool-id>`.
    ///
    /// Valid characters are `/\[a-z][0-9\]-/`. Cannot be longer than 40 characters.
    #[prost(string, tag = "3")]
    pub azure_node_pool_id: ::prost::alloc::string::String,
    /// If set, only validate the request, but do not actually create the node
    /// pool.
    #[prost(bool, tag = "4")]
    pub validate_only: bool,
}
/// Request message for `AzureClusters.UpdateAzureNodePool` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateAzureNodePoolRequest {
    /// Required. The \[AzureNodePool][google.cloud.gkemulticloud.v1.AzureNodePool\] resource to update.
    #[prost(message, optional, tag = "1")]
    pub azure_node_pool: ::core::option::Option<AzureNodePool>,
    /// If set, only validate the request, but don't actually update the node pool.
    #[prost(bool, tag = "2")]
    pub validate_only: bool,
    /// Required. Mask of fields to update. At least one path must be supplied in
    /// this field. The elements of the repeated paths field can only include these
    /// fields from \[AzureNodePool][google.cloud.gkemulticloud.v1.AzureNodePool\]:
    ///
    ///  *.  `annotations`.
    ///  *   `version`.
    ///  *   `autoscaling.min_node_count`.
    ///  *   `autoscaling.max_node_count`.
    ///  *   `config.vm_size`.
    #[prost(message, optional, tag = "3")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for `AzureClusters.GetAzureNodePool` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAzureNodePoolRequest {
    /// Required. The name of the \[AzureNodePool][google.cloud.gkemulticloud.v1.AzureNodePool\] resource to describe.
    ///
    /// `AzureNodePool` names are formatted as
    /// `projects/<project-id>/locations/<region>/azureClusters/<cluster-id>/azureNodePools/<node-pool-id>`.
    ///
    /// See [Resource Names](<https://cloud.google.com/apis/design/resource_names>)
    /// for more details on Google Cloud resource names.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for `AzureClusters.ListAzureNodePools` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAzureNodePoolsRequest {
    /// Required. The parent `AzureCluster` which owns this collection of
    /// \[AzureNodePool][google.cloud.gkemulticloud.v1.AzureNodePool\] resources.
    ///
    /// `AzureCluster` names are formatted as
    /// `projects/<project-id>/locations/<region>/azureClusters/<cluster-id>`.
    ///
    /// See [Resource Names](<https://cloud.google.com/apis/design/resource_names>)
    /// for more details on Google Cloud resource names.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of items to return.
    ///
    /// If not specified, a default value of 50 will be used by the service.
    /// Regardless of the pageSize value, the response can include a partial list
    /// and a caller should only rely on response's
    /// \[nextPageToken][google.cloud.gkemulticloud.v1.ListAzureNodePoolsResponse.next_page_token\] to determine if
    /// there are more instances left to be queried.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The `nextPageToken` value returned from a previous
    /// \[azureNodePools.list][google.cloud.gkemulticloud.v1.AzureClusters.ListAzureNodePools\] request, if any.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for `AzureClusters.ListAzureNodePools` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAzureNodePoolsResponse {
    /// A list of \[AzureNodePool][google.cloud.gkemulticloud.v1.AzureNodePool\] resources in the specified `AzureCluster`.
    #[prost(message, repeated, tag = "1")]
    pub azure_node_pools: ::prost::alloc::vec::Vec<AzureNodePool>,
    /// Token to retrieve the next page of results, or empty if there are no more
    /// results in the list.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Delete message for `AzureClusters.DeleteNodePool` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteAzureNodePoolRequest {
    /// Required. The resource name the \[AzureNodePool][google.cloud.gkemulticloud.v1.AzureNodePool\] to delete.
    ///
    /// `AzureNodePool` names are formatted as
    /// `projects/<project-id>/locations/<region>/azureClusters/<cluster-id>/azureNodePools/<node-pool-id>`.
    ///
    /// See [Resource Names](<https://cloud.google.com/apis/design/resource_names>)
    /// for more details on Google Cloud resource names.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// If set, only validate the request, but do not actually delete the node
    /// pool.
    #[prost(bool, tag = "2")]
    pub validate_only: bool,
    /// If set to true, and the \[AzureNodePool][google.cloud.gkemulticloud.v1.AzureNodePool\] resource is not found,
    /// the request will succeed but no action will be taken on the server and a
    /// completed \[Operation][google.longrunning.Operation\] will be returned.
    ///
    /// Useful for idempotent deletion.
    #[prost(bool, tag = "3")]
    pub allow_missing: bool,
    /// The current ETag of the \[AzureNodePool][google.cloud.gkemulticloud.v1.AzureNodePool\].
    ///
    /// Allows clients to perform deletions through optimistic concurrency control.
    ///
    /// If the provided ETag does not match the current etag of the node pool,
    /// the request will fail and an ABORTED error will be returned.
    #[prost(string, tag = "4")]
    pub etag: ::prost::alloc::string::String,
}
/// GetAzureServerConfigRequest gets the server config of GKE cluster on Azure.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAzureServerConfigRequest {
    /// Required. The name of the \[AzureServerConfig][google.cloud.gkemulticloud.v1.AzureServerConfig\] resource to describe.
    ///
    /// `AzureServerConfig` names are formatted as
    /// `projects/<project-id>/locations/<region>/azureServerConfig`.
    ///
    /// See [Resource Names](<https://cloud.google.com/apis/design/resource_names>)
    /// for more details on Google Cloud resource names.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for `AzureClusters.CreateAzureClient` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateAzureClientRequest {
    /// Required. The parent location where this \[AzureClient][google.cloud.gkemulticloud.v1.AzureClient\] resource
    /// will be created.
    ///
    /// Location names are formatted as `projects/<project-id>/locations/<region>`.
    ///
    /// See [Resource Names](<https://cloud.google.com/apis/design/resource_names>)
    /// for more details on Google Cloud resource names.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The specification of the \[AzureClient][google.cloud.gkemulticloud.v1.AzureClient\] to create.
    #[prost(message, optional, tag = "2")]
    pub azure_client: ::core::option::Option<AzureClient>,
    /// Required. A client provided ID the resource. Must be unique within the parent
    /// resource.
    ///
    /// The provided ID will be part of the \[AzureClient][google.cloud.gkemulticloud.v1.AzureClient\]
    /// resource name formatted as
    /// `projects/<project-id>/locations/<region>/azureClients/<client-id>`.
    ///
    /// Valid characters are `/\[a-z][0-9\]-/`. Cannot be longer than 40 characters.
    #[prost(string, tag = "4")]
    pub azure_client_id: ::prost::alloc::string::String,
    /// If set, only validate the request, but do not actually create the client.
    #[prost(bool, tag = "3")]
    pub validate_only: bool,
}
/// Request message for `AzureClusters.GetAzureClient` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAzureClientRequest {
    /// Required. The name of the \[AzureClient][google.cloud.gkemulticloud.v1.AzureClient\] resource to describe.
    ///
    /// \[AzureClient][google.cloud.gkemulticloud.v1.AzureClient\] names are formatted as
    /// `projects/<project-id>/locations/<region>/azureClients/<client-id>`.
    ///
    /// See [Resource Names](<https://cloud.google.com/apis/design/resource_names>)
    /// for more details on Google Cloud resource names.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for `AzureClusters.ListAzureClients` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAzureClientsRequest {
    /// Required. The parent location which owns this collection of
    /// \[AzureClient][google.cloud.gkemulticloud.v1.AzureClient\] resources.
    ///
    /// Location names are formatted as `projects/<project-id>/locations/<region>`.
    ///
    /// See [Resource Names](<https://cloud.google.com/apis/design/resource_names>)
    /// for more details on GCP resource names.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of items to return.
    ///
    /// If not specified, a default value of 50 will be used by the service.
    /// Regardless of the pageSize value, the response can include a partial list
    /// and a caller should only rely on response's
    /// \[nextPageToken][google.cloud.gkemulticloud.v1.ListAzureClientsResponse.next_page_token\] to determine if
    /// there are more instances left to be queried.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The `nextPageToken` value returned from a previous
    /// \[azureClients.list][google.cloud.gkemulticloud.v1.AzureClusters.ListAzureClients\] request, if any.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for `AzureClusters.ListAzureClients` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAzureClientsResponse {
    /// A list of \[AzureClient][google.cloud.gkemulticloud.v1.AzureClient\] resources in the specified Google Cloud
    /// project and region region.
    #[prost(message, repeated, tag = "1")]
    pub azure_clients: ::prost::alloc::vec::Vec<AzureClient>,
    /// Token to retrieve the next page of results, or empty if there are no more
    /// results in the list.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for `AzureClusters.DeleteAzureClient` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteAzureClientRequest {
    /// Required. The resource name the \[AzureClient][google.cloud.gkemulticloud.v1.AzureClient\] to delete.
    ///
    /// \[AzureClient][google.cloud.gkemulticloud.v1.AzureClient\] names are formatted as
    /// `projects/<project-id>/locations/<region>/azureClients/<client-id>`.
    ///
    /// See [Resource Names](<https://cloud.google.com/apis/design/resource_names>)
    /// for more details on Google Cloud resource names.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// If set to true, and the \[AzureClient][google.cloud.gkemulticloud.v1.AzureClient\] resource is not found,
    /// the request will succeed but no action will be taken on the server and a
    /// completed \[Operation][google.longrunning.Operation\] will be returned.
    ///
    /// Useful for idempotent deletion.
    #[prost(bool, tag = "2")]
    pub allow_missing: bool,
    /// If set, only validate the request, but do not actually delete the resource.
    #[prost(bool, tag = "3")]
    pub validate_only: bool,
}
/// Request message for `AzureClusters.GenerateAzureAccessToken` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateAzureAccessTokenRequest {
    /// Required. The name of the \[AzureCluster][google.cloud.gkemulticloud.v1.AzureCluster\] resource to authenticate to.
    ///
    /// `AzureCluster` names are formatted as
    /// `projects/<project-id>/locations/<region>/AzureClusters/<cluster-id>`.
    ///
    /// See [Resource Names](<https://cloud.google.com/apis/design/resource_names>)
    /// for more details on Google Cloud resource names.
    #[prost(string, tag = "1")]
    pub azure_cluster: ::prost::alloc::string::String,
}
/// Response message for `AzureClusters.GenerateAzureAccessToken` method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateAzureAccessTokenResponse {
    /// Output only. Access token to authenticate to k8s api-server.
    #[prost(string, tag = "1")]
    pub access_token: ::prost::alloc::string::String,
    /// Output only. Timestamp at which the token will expire.
    #[prost(message, optional, tag = "2")]
    pub expiration_time: ::core::option::Option<::prost_types::Timestamp>,
}
#[doc = r" Generated client implementations."]
pub mod azure_clusters_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " The AzureClusters API provides a single centrally managed service"]
    #[doc = " to create and manage Anthos clusters that run on Azure infrastructure."]
    #[derive(Debug, Clone)]
    pub struct AzureClustersClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> AzureClustersClient<T>
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
        ) -> AzureClustersClient<InterceptedService<T, F>>
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
            AzureClustersClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Creates a new [AzureClient][google.cloud.gkemulticloud.v1.AzureClient] resource on a given Google Cloud project"]
        #[doc = " and region."]
        #[doc = ""]
        #[doc = " `AzureClient` resources hold client authentication"]
        #[doc = " information needed by the Anthos Multicloud API to manage Azure resources"]
        #[doc = " on your Azure subscription on your behalf."]
        #[doc = ""]
        #[doc = " If successful, the response contains a newly created"]
        #[doc = " [Operation][google.longrunning.Operation] resource that can be"]
        #[doc = " described to track the status of the operation."]
        pub async fn create_azure_client(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateAzureClientRequest>,
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
                "/google.cloud.gkemulticloud.v1.AzureClusters/CreateAzureClient",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Describes a specific [AzureClient][google.cloud.gkemulticloud.v1.AzureClient] resource."]
        pub async fn get_azure_client(
            &mut self,
            request: impl tonic::IntoRequest<super::GetAzureClientRequest>,
        ) -> Result<tonic::Response<super::AzureClient>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.gkemulticloud.v1.AzureClusters/GetAzureClient",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists all [AzureClient][google.cloud.gkemulticloud.v1.AzureClient] resources on a given Google Cloud project and"]
        #[doc = " region."]
        pub async fn list_azure_clients(
            &mut self,
            request: impl tonic::IntoRequest<super::ListAzureClientsRequest>,
        ) -> Result<tonic::Response<super::ListAzureClientsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.gkemulticloud.v1.AzureClusters/ListAzureClients",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes a specific [AzureClient][google.cloud.gkemulticloud.v1.AzureClient] resource."]
        #[doc = ""]
        #[doc = " If the client is used by one or more clusters, deletion will"]
        #[doc = " fail and a `FAILED_PRECONDITION` error will be returned."]
        #[doc = ""]
        #[doc = " If successful, the response contains a newly created"]
        #[doc = " [Operation][google.longrunning.Operation] resource that can be"]
        #[doc = " described to track the status of the operation."]
        pub async fn delete_azure_client(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteAzureClientRequest>,
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
                "/google.cloud.gkemulticloud.v1.AzureClusters/DeleteAzureClient",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a new [AzureCluster][google.cloud.gkemulticloud.v1.AzureCluster] resource on a given GCP project and region."]
        #[doc = ""]
        #[doc = " If successful, the response contains a newly created"]
        #[doc = " [Operation][google.longrunning.Operation] resource that can be"]
        #[doc = " described to track the status of the operation."]
        pub async fn create_azure_cluster(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateAzureClusterRequest>,
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
                "/google.cloud.gkemulticloud.v1.AzureClusters/CreateAzureCluster",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates an [AzureCluster][google.cloud.gkemulticloud.v1.AzureCluster]."]
        pub async fn update_azure_cluster(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateAzureClusterRequest>,
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
                "/google.cloud.gkemulticloud.v1.AzureClusters/UpdateAzureCluster",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Describes a specific [AzureCluster][google.cloud.gkemulticloud.v1.AzureCluster] resource."]
        pub async fn get_azure_cluster(
            &mut self,
            request: impl tonic::IntoRequest<super::GetAzureClusterRequest>,
        ) -> Result<tonic::Response<super::AzureCluster>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.gkemulticloud.v1.AzureClusters/GetAzureCluster",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists all [AzureCluster][google.cloud.gkemulticloud.v1.AzureCluster] resources on a given Google Cloud project and"]
        #[doc = " region."]
        pub async fn list_azure_clusters(
            &mut self,
            request: impl tonic::IntoRequest<super::ListAzureClustersRequest>,
        ) -> Result<tonic::Response<super::ListAzureClustersResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.gkemulticloud.v1.AzureClusters/ListAzureClusters",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes a specific [AzureCluster][google.cloud.gkemulticloud.v1.AzureCluster] resource."]
        #[doc = ""]
        #[doc = " Fails if the cluster has one or more associated [AzureNodePool][google.cloud.gkemulticloud.v1.AzureNodePool]"]
        #[doc = " resources."]
        #[doc = ""]
        #[doc = " If successful, the response contains a newly created"]
        #[doc = " [Operation][google.longrunning.Operation] resource that can be"]
        #[doc = " described to track the status of the operation."]
        pub async fn delete_azure_cluster(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteAzureClusterRequest>,
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
                "/google.cloud.gkemulticloud.v1.AzureClusters/DeleteAzureCluster",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Generates a short-lived access token to authenticate to a given"]
        #[doc = " [AzureCluster][google.cloud.gkemulticloud.v1.AzureCluster] resource."]
        pub async fn generate_azure_access_token(
            &mut self,
            request: impl tonic::IntoRequest<super::GenerateAzureAccessTokenRequest>,
        ) -> Result<tonic::Response<super::GenerateAzureAccessTokenResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.gkemulticloud.v1.AzureClusters/GenerateAzureAccessToken",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a new [AzureNodePool][google.cloud.gkemulticloud.v1.AzureNodePool], attached to a given [AzureCluster][google.cloud.gkemulticloud.v1.AzureCluster]."]
        #[doc = ""]
        #[doc = " If successful, the response contains a newly created"]
        #[doc = " [Operation][google.longrunning.Operation] resource that can be"]
        #[doc = " described to track the status of the operation."]
        pub async fn create_azure_node_pool(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateAzureNodePoolRequest>,
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
                "/google.cloud.gkemulticloud.v1.AzureClusters/CreateAzureNodePool",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates an [AzureNodePool][google.cloud.gkemulticloud.v1.AzureNodePool]."]
        pub async fn update_azure_node_pool(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateAzureNodePoolRequest>,
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
                "/google.cloud.gkemulticloud.v1.AzureClusters/UpdateAzureNodePool",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Describes a specific [AzureNodePool][google.cloud.gkemulticloud.v1.AzureNodePool] resource."]
        pub async fn get_azure_node_pool(
            &mut self,
            request: impl tonic::IntoRequest<super::GetAzureNodePoolRequest>,
        ) -> Result<tonic::Response<super::AzureNodePool>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.gkemulticloud.v1.AzureClusters/GetAzureNodePool",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists all [AzureNodePool][google.cloud.gkemulticloud.v1.AzureNodePool] resources on a given [AzureCluster][google.cloud.gkemulticloud.v1.AzureCluster]."]
        pub async fn list_azure_node_pools(
            &mut self,
            request: impl tonic::IntoRequest<super::ListAzureNodePoolsRequest>,
        ) -> Result<tonic::Response<super::ListAzureNodePoolsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.gkemulticloud.v1.AzureClusters/ListAzureNodePools",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes a specific [AzureNodePool][google.cloud.gkemulticloud.v1.AzureNodePool] resource."]
        #[doc = ""]
        #[doc = " If successful, the response contains a newly created"]
        #[doc = " [Operation][google.longrunning.Operation] resource that can be"]
        #[doc = " described to track the status of the operation."]
        pub async fn delete_azure_node_pool(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteAzureNodePoolRequest>,
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
                "/google.cloud.gkemulticloud.v1.AzureClusters/DeleteAzureNodePool",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns information, such as supported Azure regions and Kubernetes"]
        #[doc = " versions, on a given Google Cloud location."]
        pub async fn get_azure_server_config(
            &mut self,
            request: impl tonic::IntoRequest<super::GetAzureServerConfigRequest>,
        ) -> Result<tonic::Response<super::AzureServerConfig>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.gkemulticloud.v1.AzureClusters/GetAzureServerConfig",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
