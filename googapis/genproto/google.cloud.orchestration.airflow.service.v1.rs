/// Create a new environment.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateEnvironmentRequest {
    /// The parent must be of the form
    /// "projects/{projectId}/locations/{locationId}".
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The environment to create.
    #[prost(message, optional, tag = "2")]
    pub environment: ::core::option::Option<Environment>,
}
/// Get an environment.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetEnvironmentRequest {
    /// The resource name of the environment to get, in the form:
    /// "projects/{projectId}/locations/{locationId}/environments/{environmentId}"
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// List environments in a project and location.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListEnvironmentsRequest {
    /// List environments in the given project and location, in the form:
    /// "projects/{projectId}/locations/{locationId}"
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of environments to return.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The next_page_token value returned from a previous List request, if any.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// The environments in a project and location.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListEnvironmentsResponse {
    /// The list of environments returned by a ListEnvironmentsRequest.
    #[prost(message, repeated, tag = "1")]
    pub environments: ::prost::alloc::vec::Vec<Environment>,
    /// The page token used to query for the next page if one exists.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Delete an environment.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteEnvironmentRequest {
    /// The environment to delete, in the form:
    /// "projects/{projectId}/locations/{locationId}/environments/{environmentId}"
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Update an environment.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateEnvironmentRequest {
    /// The relative resource name of the environment to update, in the form:
    /// "projects/{projectId}/locations/{locationId}/environments/{environmentId}"
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    /// A patch environment. Fields specified by the `updateMask` will be copied
    /// from the patch environment into the environment under update.
    #[prost(message, optional, tag = "1")]
    pub environment: ::core::option::Option<Environment>,
    /// Required. A comma-separated list of paths, relative to `Environment`, of
    /// fields to update.
    /// For example, to set the version of scikit-learn to install in the
    /// environment to 0.19.0 and to remove an existing installation of
    /// numpy, the `updateMask` parameter would include the following two
    /// `paths` values: "config.softwareConfig.pypiPackages.scikit-learn" and
    /// "config.softwareConfig.pypiPackages.numpy". The included patch
    /// environment would specify the scikit-learn version as follows:
    ///
    ///     {
    ///       "config":{
    ///         "softwareConfig":{
    ///           "pypiPackages":{
    ///             "scikit-learn":"==0.19.0"
    ///           }
    ///         }
    ///       }
    ///     }
    ///
    /// Note that in the above example, any existing PyPI packages
    /// other than scikit-learn and numpy will be unaffected.
    ///
    /// Only one update type may be included in a single request's `updateMask`.
    /// For example, one cannot update both the PyPI packages and
    /// labels in the same request. However, it is possible to update multiple
    /// members of a map field simultaneously in the same request. For example,
    /// to set the labels "label1" and "label2" while clearing "label3" (assuming
    /// it already exists), one can
    /// provide the paths "labels.label1", "labels.label2", and "labels.label3"
    /// and populate the patch environment as follows:
    ///
    ///     {
    ///       "labels":{
    ///         "label1":"new-label1-value"
    ///         "label2":"new-label2-value"
    ///       }
    ///     }
    ///
    /// Note that in the above example, any existing labels that are not
    /// included in the `updateMask` will be unaffected.
    ///
    /// It is also possible to replace an entire map field by providing the
    /// map field's path in the `updateMask`. The new value of the field will
    /// be that which is provided in the patch environment. For example, to
    /// delete all pre-existing user-specified PyPI packages and
    /// install botocore at version 1.7.14, the `updateMask` would contain
    /// the path "config.softwareConfig.pypiPackages", and
    /// the patch environment would be the following:
    ///
    ///     {
    ///       "config":{
    ///         "softwareConfig":{
    ///           "pypiPackages":{
    ///             "botocore":"==1.7.14"
    ///           }
    ///         }
    ///       }
    ///     }
    ///
    /// **Note:** Only the following fields can be updated:
    ///
    /// * `config.softwareConfig.pypiPackages`
    ///     * Replace all custom custom PyPI packages. If a replacement
    ///       package map is not included in `environment`, all custom
    ///       PyPI packages are cleared. It is an error to provide both
    ///       this mask and a mask specifying an individual package.
    /// * `config.softwareConfig.pypiPackages.`packagename
    ///     * Update the custom PyPI package *packagename*,
    ///       preserving other packages. To delete the package, include it in
    ///       `updateMask`, and omit the mapping for it in
    ///       `environment.config.softwareConfig.pypiPackages`. It is an error
    ///       to provide both a mask of this form and the
    ///       `config.softwareConfig.pypiPackages` mask.
    /// * `labels`
    ///     * Replace all environment labels. If a replacement labels map is not
    ///       included in `environment`, all labels are cleared. It is an error to
    ///       provide both this mask and a mask specifying one or more individual
    ///       labels.
    /// * `labels.`labelName
    ///     * Set the label named *labelName*, while preserving other
    ///       labels. To delete the label, include it in `updateMask` and omit its
    ///       mapping in `environment.labels`. It is an error to provide both a
    ///       mask of this form and the `labels` mask.
    /// * `config.nodeCount`
    ///     * Horizontally scale the number of nodes in the environment. An integer
    ///       greater than or equal to 3 must be provided in the `config.nodeCount`
    ///       field.
    /// * `config.webServerNetworkAccessControl`
    ///     * Replace the environment's current `WebServerNetworkAccessControl`.
    /// * `config.databaseConfig`
    ///     * Replace the environment's current `DatabaseConfig`.
    /// * `config.webServerConfig`
    ///     * Replace the environment's current `WebServerConfig`.
    /// * `config.softwareConfig.airflowConfigOverrides`
    ///     * Replace all Apache Airflow config overrides. If a replacement config
    ///       overrides map is not included in `environment`, all config overrides
    ///       are cleared.
    ///       It is an error to provide both this mask and a mask specifying one or
    ///       more individual config overrides.
    /// * `config.softwareConfig.airflowConfigOverrides.`section-name
    ///     * Override the Apache Airflow config property *name* in the
    ///       section named *section*, preserving other properties. To
    ///       delete the property override, include it in `updateMask` and omit its
    ///       mapping in
    ///       `environment.config.softwareConfig.airflowConfigOverrides`.
    ///       It is an error to provide both a mask of this form and the
    ///       `config.softwareConfig.airflowConfigOverrides` mask.
    /// * `config.softwareConfig.envVariables`
    ///     * Replace all environment variables. If a replacement environment
    ///       variable map is not included in `environment`, all custom environment
    ///       variables  are cleared.
    ///       It is an error to provide both this mask and a mask specifying one or
    ///       more individual environment variables.
    #[prost(message, optional, tag = "3")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Configuration information for an environment.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EnvironmentConfig {
    /// Output only. The Kubernetes Engine cluster used to run this environment.
    #[prost(string, tag = "1")]
    pub gke_cluster: ::prost::alloc::string::String,
    /// Output only. The Cloud Storage prefix of the DAGs for this environment. Although Cloud
    /// Storage objects reside in a flat namespace, a hierarchical file tree
    /// can be simulated using "/"-delimited object name prefixes. DAG objects for
    /// this environment reside in a simulated directory with the given prefix.
    #[prost(string, tag = "2")]
    pub dag_gcs_prefix: ::prost::alloc::string::String,
    /// The number of nodes in the Kubernetes Engine cluster that will be
    /// used to run this environment.
    #[prost(int32, tag = "3")]
    pub node_count: i32,
    /// The configuration settings for software inside the environment.
    #[prost(message, optional, tag = "4")]
    pub software_config: ::core::option::Option<SoftwareConfig>,
    /// The configuration used for the Kubernetes Engine cluster.
    #[prost(message, optional, tag = "5")]
    pub node_config: ::core::option::Option<NodeConfig>,
    /// The configuration used for the Private IP Cloud Composer environment.
    #[prost(message, optional, tag = "7")]
    pub private_environment_config: ::core::option::Option<PrivateEnvironmentConfig>,
    /// Optional. The network-level access control policy for the Airflow web server. If
    /// unspecified, no network-level access restrictions will be applied.
    #[prost(message, optional, tag = "8")]
    pub web_server_network_access_control: ::core::option::Option<WebServerNetworkAccessControl>,
    /// Optional. The configuration settings for Cloud SQL instance used internally by Apache
    /// Airflow software.
    #[prost(message, optional, tag = "9")]
    pub database_config: ::core::option::Option<DatabaseConfig>,
    /// Optional. The configuration settings for the Airflow web server App Engine instance.
    #[prost(message, optional, tag = "10")]
    pub web_server_config: ::core::option::Option<WebServerConfig>,
    /// Optional. The encryption options for the Cloud Composer environment
    /// and its dependencies. Cannot be updated.
    #[prost(message, optional, tag = "11")]
    pub encryption_config: ::core::option::Option<EncryptionConfig>,
    /// Output only. The URI of the Apache Airflow Web UI hosted within this environment (see
    /// [Airflow web
    /// interface](/composer/docs/how-to/accessing/airflow-web-interface)).
    #[prost(string, tag = "6")]
    pub airflow_uri: ::prost::alloc::string::String,
}
/// Network-level access control policy for the Airflow web server.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WebServerNetworkAccessControl {
    /// A collection of allowed IP ranges with descriptions.
    #[prost(message, repeated, tag = "1")]
    pub allowed_ip_ranges:
        ::prost::alloc::vec::Vec<web_server_network_access_control::AllowedIpRange>,
}
/// Nested message and enum types in `WebServerNetworkAccessControl`.
pub mod web_server_network_access_control {
    /// Allowed IP range with user-provided description.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct AllowedIpRange {
        /// IP address or range, defined using CIDR notation, of requests that this
        /// rule applies to.
        /// Examples: `192.168.1.1` or `192.168.0.0/16` or `2001:db8::/32`
        ///           or `2001:0db8:0000:0042:0000:8a2e:0370:7334`.
        ///
        /// IP range prefixes should be properly truncated. For example,
        /// `1.2.3.4/24` should be truncated to `1.2.3.0/24`. Similarly, for IPv6,
        /// `2001:db8::1/32` should be truncated to `2001:db8::/32`.
        #[prost(string, tag = "1")]
        pub value: ::prost::alloc::string::String,
        /// Optional. User-provided description. It must contain at most 300 characters.
        #[prost(string, tag = "2")]
        pub description: ::prost::alloc::string::String,
    }
}
/// The configuration of Cloud SQL instance that is used by the Apache Airflow
/// software.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DatabaseConfig {
    /// Optional. Cloud SQL machine type used by Airflow database.
    /// It has to be one of: db-n1-standard-2, db-n1-standard-4, db-n1-standard-8
    /// or db-n1-standard-16. If not specified, db-n1-standard-2 will be used.
    #[prost(string, tag = "1")]
    pub machine_type: ::prost::alloc::string::String,
}
/// The configuration settings for the Airflow web server App Engine instance.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WebServerConfig {
    /// Optional. Machine type on which Airflow web server is running.
    /// It has to be one of: composer-n1-webserver-2, composer-n1-webserver-4 or
    /// composer-n1-webserver-8.
    /// If not specified, composer-n1-webserver-2 will be used.
    /// Value custom is returned only in response, if Airflow web server parameters
    /// were manually changed to a non-standard values.
    #[prost(string, tag = "1")]
    pub machine_type: ::prost::alloc::string::String,
}
/// The encryption options for the Cloud Composer environment
/// and its dependencies.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EncryptionConfig {
    /// Optional. Customer-managed Encryption Key available through Google's Key Management
    /// Service. Cannot be updated.
    /// If not specified, Google-managed key will be used.
    #[prost(string, tag = "1")]
    pub kms_key_name: ::prost::alloc::string::String,
}
/// Specifies the selection and configuration of software inside the environment.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SoftwareConfig {
    /// The version of the software running in the environment.
    /// This encapsulates both the version of Cloud Composer functionality and the
    /// version of Apache Airflow. It must match the regular expression
    /// `composer-(\[0-9]+\.[0-9]+\.[0-9]+|latest)-airflow-[0-9]+\.[0-9]+(\.[0-9\]+.*)?`.
    /// When used as input, the server also checks if the provided version is
    /// supported and denies the request for an unsupported version.
    ///
    /// The Cloud Composer portion of the version is a
    /// [semantic version](<https://semver.org>) or `latest`. When the patch version
    /// is omitted, the current Cloud Composer patch version is selected.
    /// When `latest` is provided instead of an explicit version number,
    /// the server replaces `latest` with the current Cloud Composer version
    /// and stores that version number in the same field.
    ///
    /// The portion of the image version that follows *airflow-* is an
    /// official Apache Airflow repository
    /// [release name](<https://github.com/apache/incubator-airflow/releases>).
    ///
    /// See also [Version
    /// List](/composer/docs/concepts/versioning/composer-versions).
    #[prost(string, tag = "1")]
    pub image_version: ::prost::alloc::string::String,
    /// Optional. Apache Airflow configuration properties to override.
    ///
    /// Property keys contain the section and property names, separated by a
    /// hyphen, for example "core-dags_are_paused_at_creation". Section names must
    /// not contain hyphens ("-"), opening square brackets ("["),  or closing
    /// square brackets ("]"). The property name must not be empty and must not
    /// contain an equals sign ("=") or semicolon (";"). Section and property names
    /// must not contain a period ("."). Apache Airflow configuration property
    /// names must be written in
    /// \[snake_case\](<https://en.wikipedia.org/wiki/Snake_case>). Property values can
    /// contain any character, and can be written in any lower/upper case format.
    ///
    /// Certain Apache Airflow configuration property values are
    /// \[blocked\](/composer/docs/concepts/airflow-configurations),
    /// and cannot be overridden.
    #[prost(map = "string, string", tag = "2")]
    pub airflow_config_overrides:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Optional. Custom Python Package Index (PyPI) packages to be installed in
    /// the environment.
    ///
    /// Keys refer to the lowercase package name such as "numpy"
    /// and values are the lowercase extras and version specifier such as
    /// "==1.12.0", "\[devel,gcp_api\]", or "\[devel\]>=1.8.2, <1.9.2". To specify a
    /// package without pinning it to a version specifier, use the empty string as
    /// the value.
    #[prost(map = "string, string", tag = "3")]
    pub pypi_packages:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Optional. Additional environment variables to provide to the Apache Airflow
    /// scheduler, worker, and webserver processes.
    ///
    /// Environment variable names must match the regular expression
    /// `\[a-zA-Z_][a-zA-Z0-9_\]*`. They cannot specify Apache Airflow
    /// software configuration overrides (they cannot match the regular expression
    /// `AIRFLOW__\[A-Z0-9_]+__[A-Z0-9_\]+`), and they cannot match any of the
    /// following reserved names:
    ///
    /// * `AIRFLOW_HOME`
    /// * `C_FORCE_ROOT`
    /// * `CONTAINER_NAME`
    /// * `DAGS_FOLDER`
    /// * `GCP_PROJECT`
    /// * `GCS_BUCKET`
    /// * `GKE_CLUSTER_NAME`
    /// * `SQL_DATABASE`
    /// * `SQL_INSTANCE`
    /// * `SQL_PASSWORD`
    /// * `SQL_PROJECT`
    /// * `SQL_REGION`
    /// * `SQL_USER`
    #[prost(map = "string, string", tag = "4")]
    pub env_variables:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Optional. The major version of Python used to run the Apache Airflow
    /// scheduler, worker, and webserver processes.
    ///
    /// Can be set to '2' or '3'. If not specified, the default is '3'. Cannot be
    /// updated.
    #[prost(string, tag = "6")]
    pub python_version: ::prost::alloc::string::String,
}
/// Configuration for controlling how IPs are allocated in the
/// GKE cluster running the Apache Airflow software.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IpAllocationPolicy {
    /// Optional. Whether or not to enable Alias IPs in the GKE cluster.
    /// If `true`, a VPC-native cluster is created.
    #[prost(bool, tag = "1")]
    pub use_ip_aliases: bool,
    /// Configuration of allocating IP addresses for pods in the GKE cluster.
    #[prost(oneof = "ip_allocation_policy::ClusterIpAllocation", tags = "2, 4")]
    pub cluster_ip_allocation: ::core::option::Option<ip_allocation_policy::ClusterIpAllocation>,
    /// Configuration of allocating IP addresses for services in the GKE cluster.
    #[prost(oneof = "ip_allocation_policy::ServicesIpAllocation", tags = "3, 5")]
    pub services_ip_allocation: ::core::option::Option<ip_allocation_policy::ServicesIpAllocation>,
}
/// Nested message and enum types in `IPAllocationPolicy`.
pub mod ip_allocation_policy {
    /// Configuration of allocating IP addresses for pods in the GKE cluster.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ClusterIpAllocation {
        /// Optional. The name of the GKE cluster's secondary range used to allocate
        /// IP addresses to pods.
        ///
        /// This field is applicable only when `use_ip_aliases` is true.
        #[prost(string, tag = "2")]
        ClusterSecondaryRangeName(::prost::alloc::string::String),
        /// Optional. The IP address range used to allocate IP addresses to pods in
        /// the GKE cluster.
        ///
        /// This field is applicable only when `use_ip_aliases` is true.
        ///
        /// Set to blank to have GKE choose a range with the default size.
        ///
        /// Set to /netmask (e.g. `/14`) to have GKE choose a range with a specific
        /// netmask.
        ///
        /// Set to a
        /// \[CIDR\](<http://en.wikipedia.org/wiki/Classless_Inter-Domain_Routing>)
        /// notation (e.g. `10.96.0.0/14`) from the RFC-1918 private networks (e.g.
        /// `10.0.0.0/8`, `172.16.0.0/12`, `192.168.0.0/16`) to pick a specific range
        /// to use.
        #[prost(string, tag = "4")]
        ClusterIpv4CidrBlock(::prost::alloc::string::String),
    }
    /// Configuration of allocating IP addresses for services in the GKE cluster.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ServicesIpAllocation {
        /// Optional. The name of the services' secondary range used to allocate
        /// IP addresses to the GKE cluster.
        ///
        /// This field is applicable only when `use_ip_aliases` is true.
        #[prost(string, tag = "3")]
        ServicesSecondaryRangeName(::prost::alloc::string::String),
        /// Optional. The IP address range of the services IP addresses in this
        /// GKE cluster.
        ///
        /// This field is applicable only when `use_ip_aliases` is true.
        ///
        /// Set to blank to have GKE choose a range with the default size.
        ///
        /// Set to /netmask (e.g. `/14`) to have GKE choose a range with a specific
        /// netmask.
        ///
        /// Set to a
        /// \[CIDR\](<http://en.wikipedia.org/wiki/Classless_Inter-Domain_Routing>)
        /// notation (e.g. `10.96.0.0/14`) from the RFC-1918 private networks (e.g.
        /// `10.0.0.0/8`, `172.16.0.0/12`, `192.168.0.0/16`) to pick a specific range
        /// to use.
        #[prost(string, tag = "5")]
        ServicesIpv4CidrBlock(::prost::alloc::string::String),
    }
}
/// The configuration information for the Kubernetes Engine nodes running
/// the Apache Airflow software.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NodeConfig {
    /// Optional. The Compute Engine \[zone\](/compute/docs/regions-zones) in which
    /// to deploy the VMs used to run the Apache Airflow software, specified as a
    /// [relative resource
    /// name](/apis/design/resource_names#relative_resource_name). For example:
    /// "projects/{projectId}/zones/{zoneId}".
    ///
    /// This `location` must belong to the enclosing environment's project and
    /// location. If both this field and `nodeConfig.machineType` are specified,
    /// `nodeConfig.machineType` must belong to this `location`; if both are
    /// unspecified, the service will pick a zone in the Compute Engine region
    /// corresponding to the Cloud Composer location, and propagate that choice to
    /// both fields. If only one field (`location` or `nodeConfig.machineType`) is
    /// specified, the location information from the specified field will be
    /// propagated to the unspecified field.
    #[prost(string, tag = "1")]
    pub location: ::prost::alloc::string::String,
    /// Optional. The Compute Engine
    /// [machine type](/compute/docs/machine-types) used for cluster instances,
    /// specified as a
    /// [relative resource
    /// name](/apis/design/resource_names#relative_resource_name). For example:
    /// "projects/{projectId}/zones/{zoneId}/machineTypes/{machineTypeId}".
    ///
    /// The `machineType` must belong to the enclosing environment's project and
    /// location. If both this field and `nodeConfig.location` are specified,
    /// this `machineType` must belong to the `nodeConfig.location`; if both are
    /// unspecified, the service will pick a zone in the Compute Engine region
    /// corresponding to the Cloud Composer location, and propagate that choice to
    /// both fields. If exactly one of this field and `nodeConfig.location` is
    /// specified, the location information from the specified field will be
    /// propagated to the unspecified field.
    ///
    /// The `machineTypeId` must not be a [shared-core machine
    /// type](/compute/docs/machine-types#sharedcore).
    ///
    /// If this field is unspecified, the `machineTypeId` defaults
    /// to "n1-standard-1".
    #[prost(string, tag = "2")]
    pub machine_type: ::prost::alloc::string::String,
    /// Optional. The Compute Engine network to be used for machine
    /// communications, specified as a
    /// [relative resource
    /// name](/apis/design/resource_names#relative_resource_name). For example:
    /// "projects/{projectId}/global/networks/{networkId}".
    ///
    /// If unspecified, the "default" network ID in the environment's project is
    /// used. If a [Custom Subnet Network](/vpc/docs/vpc#vpc_networks_and_subnets)
    /// is provided, `nodeConfig.subnetwork` must also be provided. For
    /// [Shared VPC](/vpc/docs/shared-vpc) subnetwork requirements, see
    /// `nodeConfig.subnetwork`.
    #[prost(string, tag = "3")]
    pub network: ::prost::alloc::string::String,
    /// Optional. The Compute Engine subnetwork to be used for machine
    /// communications, specified as a
    /// [relative resource
    /// name](/apis/design/resource_names#relative_resource_name). For example:
    /// "projects/{projectId}/regions/{regionId}/subnetworks/{subnetworkId}"
    ///
    /// If a subnetwork is provided, `nodeConfig.network` must also be provided,
    /// and the subnetwork must belong to the enclosing environment's project and
    /// location.
    #[prost(string, tag = "4")]
    pub subnetwork: ::prost::alloc::string::String,
    /// Optional. The disk size in GB used for node VMs. Minimum size is 20GB.
    /// If unspecified, defaults to 100GB. Cannot be updated.
    #[prost(int32, tag = "5")]
    pub disk_size_gb: i32,
    /// Optional. The set of Google API scopes to be made available on all
    /// node VMs. If `oauth_scopes` is empty, defaults to
    /// \["<https://www.googleapis.com/auth/cloud-platform"\].> Cannot be updated.
    #[prost(string, repeated, tag = "6")]
    pub oauth_scopes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Optional. The Google Cloud Platform Service Account to be used by the node
    /// VMs. If a service account is not specified, the "default" Compute Engine
    /// service account is used. Cannot be updated.
    #[prost(string, tag = "7")]
    pub service_account: ::prost::alloc::string::String,
    /// Optional. The list of instance tags applied to all node VMs. Tags are used
    /// to identify valid sources or targets for network firewalls. Each tag within
    /// the list must comply with \[RFC1035\](<https://www.ietf.org/rfc/rfc1035.txt>).
    /// Cannot be updated.
    #[prost(string, repeated, tag = "8")]
    pub tags: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Optional. The configuration for controlling how IPs are allocated in the GKE cluster.
    #[prost(message, optional, tag = "9")]
    pub ip_allocation_policy: ::core::option::Option<IpAllocationPolicy>,
}
/// Configuration options for the private GKE cluster in a Cloud Composer
/// environment.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrivateClusterConfig {
    /// Optional. If `true`, access to the public endpoint of the GKE cluster is
    /// denied.
    #[prost(bool, tag = "1")]
    pub enable_private_endpoint: bool,
    /// Optional. The CIDR block from which IPv4 range for GKE master will be reserved. If
    /// left blank, the default value of '172.16.0.0/23' is used.
    #[prost(string, tag = "2")]
    pub master_ipv4_cidr_block: ::prost::alloc::string::String,
    /// Output only. The IP range in CIDR notation to use for the hosted master network. This
    /// range is used for assigning internal IP addresses to the GKE cluster
    /// master or set of masters and to the internal load balancer virtual IP.
    /// This range must not overlap with any other ranges in use
    /// within the cluster's network.
    #[prost(string, tag = "3")]
    pub master_ipv4_reserved_range: ::prost::alloc::string::String,
}
/// The configuration information for configuring a Private IP Cloud Composer
/// environment.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrivateEnvironmentConfig {
    /// Optional. If `true`, a Private IP Cloud Composer environment is created.
    /// If this field is set to true, `IPAllocationPolicy.use_ip_aliases` must be
    /// set to true.
    #[prost(bool, tag = "1")]
    pub enable_private_environment: bool,
    /// Optional. Configuration for the private GKE cluster for a Private IP
    /// Cloud Composer environment.
    #[prost(message, optional, tag = "2")]
    pub private_cluster_config: ::core::option::Option<PrivateClusterConfig>,
    /// Optional. The CIDR block from which IP range for web server will be reserved. Needs
    /// to be disjoint from `private_cluster_config.master_ipv4_cidr_block` and
    /// `cloud_sql_ipv4_cidr_block`.
    #[prost(string, tag = "3")]
    pub web_server_ipv4_cidr_block: ::prost::alloc::string::String,
    /// Optional. The CIDR block from which IP range in tenant project will be reserved for
    /// Cloud SQL. Needs to be disjoint from `web_server_ipv4_cidr_block`.
    #[prost(string, tag = "4")]
    pub cloud_sql_ipv4_cidr_block: ::prost::alloc::string::String,
    /// Output only. The IP range reserved for the tenant project's App Engine VMs.
    #[prost(string, tag = "5")]
    pub web_server_ipv4_reserved_range: ::prost::alloc::string::String,
}
/// An environment for running orchestration tasks.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Environment {
    /// The resource name of the environment, in the form:
    /// "projects/{projectId}/locations/{locationId}/environments/{environmentId}"
    ///
    /// EnvironmentId must start with a lowercase letter followed by up to 63
    /// lowercase letters, numbers, or hyphens, and cannot end with a hyphen.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Configuration parameters for this environment.
    #[prost(message, optional, tag = "2")]
    pub config: ::core::option::Option<EnvironmentConfig>,
    /// Output only. The UUID (Universally Unique IDentifier) associated with this environment.
    /// This value is generated when the environment is created.
    #[prost(string, tag = "3")]
    pub uuid: ::prost::alloc::string::String,
    /// The current state of the environment.
    #[prost(enumeration = "environment::State", tag = "4")]
    pub state: i32,
    /// Output only. The time at which this environment was created.
    #[prost(message, optional, tag = "5")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The time at which this environment was last modified.
    #[prost(message, optional, tag = "6")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Optional. User-defined labels for this environment.
    /// The labels map can contain no more than 64 entries. Entries of the labels
    /// map are UTF8 strings that comply with the following restrictions:
    ///
    /// * Keys must conform to regexp: \[\p{Ll}\p{Lo}][\p{Ll}\p{Lo}\p{N}_-\]{0,62}
    /// * Values must conform to regexp:  \[\p{Ll}\p{Lo}\p{N}_-\]{0,63}
    /// * Both keys and values are additionally constrained to be <= 128 bytes in
    /// size.
    #[prost(map = "string, string", tag = "7")]
    pub labels:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
}
/// Nested message and enum types in `Environment`.
pub mod environment {
    /// State of the environment.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// The state of the environment is unknown.
        Unspecified = 0,
        /// The environment is in the process of being created.
        Creating = 1,
        /// The environment is currently running and healthy. It is ready for use.
        Running = 2,
        /// The environment is being updated. It remains usable but cannot receive
        /// additional update requests or be deleted at this time.
        Updating = 3,
        /// The environment is undergoing deletion. It cannot be used.
        Deleting = 4,
        /// The environment has encountered an error and cannot be used.
        Error = 5,
    }
}
/// Message containing information about the result of an upgrade check
/// operation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckUpgradeResponse {
    /// Output only. Url for a docker build log of an upgraded image.
    #[prost(string, tag = "1")]
    pub build_log_uri: ::prost::alloc::string::String,
    /// Output only. Whether build has succeeded or failed on modules conflicts.
    #[prost(enumeration = "check_upgrade_response::ConflictResult", tag = "4")]
    pub contains_pypi_modules_conflict: i32,
    /// Output only. Extract from a docker image build log containing information about pypi
    /// modules conflicts.
    #[prost(string, tag = "3")]
    pub pypi_conflict_build_log_extract: ::prost::alloc::string::String,
    /// Composer image for which the build was happening.
    #[prost(string, tag = "5")]
    pub image_version: ::prost::alloc::string::String,
    /// Pypi dependencies specified in the environment configuration, at the time
    /// when the build was triggered.
    #[prost(map = "string, string", tag = "6")]
    pub pypi_dependencies:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
}
/// Nested message and enum types in `CheckUpgradeResponse`.
pub mod check_upgrade_response {
    /// Whether there were python modules conflict during image build.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ConflictResult {
        /// It is unknown whether build had conflicts or not.
        Unspecified = 0,
        /// There were python packages conflicts.
        Conflict = 1,
        /// There were no python packages conflicts.
        NoConflict = 2,
    }
}
#[doc = r" Generated client implementations."]
pub mod environments_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Managed Apache Airflow Environments."]
    #[derive(Debug, Clone)]
    pub struct EnvironmentsClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> EnvironmentsClient<T>
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
        ) -> EnvironmentsClient<InterceptedService<T, F>>
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
            EnvironmentsClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Create a new environment."]
        pub async fn create_environment(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateEnvironmentRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.orchestration.airflow.service.v1.Environments/CreateEnvironment",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Get an existing environment."]
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
                "/google.cloud.orchestration.airflow.service.v1.Environments/GetEnvironment",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " List environments."]
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
                "/google.cloud.orchestration.airflow.service.v1.Environments/ListEnvironments",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Update an environment."]
        pub async fn update_environment(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateEnvironmentRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.orchestration.airflow.service.v1.Environments/UpdateEnvironment",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Delete an environment."]
        pub async fn delete_environment(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteEnvironmentRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::super::super::longrunning::Operation>,
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
                "/google.cloud.orchestration.airflow.service.v1.Environments/DeleteEnvironment",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// List ImageVersions in a project and location.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListImageVersionsRequest {
    /// List ImageVersions in the given project and location, in the form:
    /// "projects/{projectId}/locations/{locationId}"
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of image_versions to return.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The next_page_token value returned from a previous List request, if any.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Whether or not image versions from old releases should be included.
    #[prost(bool, tag = "4")]
    pub include_past_releases: bool,
}
/// The ImageVersions in a project and location.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListImageVersionsResponse {
    /// The list of supported ImageVersions in a location.
    #[prost(message, repeated, tag = "1")]
    pub image_versions: ::prost::alloc::vec::Vec<ImageVersion>,
    /// The page token used to query for the next page if one exists.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// ImageVersion information
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImageVersion {
    /// The string identifier of the ImageVersion, in the form:
    /// "composer-x.y.z-airflow-a.b(.c)"
    #[prost(string, tag = "1")]
    pub image_version_id: ::prost::alloc::string::String,
    /// Whether this is the default ImageVersion used by Composer during
    /// environment creation if no input ImageVersion is specified.
    #[prost(bool, tag = "2")]
    pub is_default: bool,
    /// supported python versions
    #[prost(string, repeated, tag = "3")]
    pub supported_python_versions: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The date of the version release.
    #[prost(message, optional, tag = "4")]
    pub release_date: ::core::option::Option<super::super::super::super::super::r#type::Date>,
    /// Whether it is impossible to create an environment with the image version.
    #[prost(bool, tag = "5")]
    pub creation_disabled: bool,
    /// Whether it is impossible to upgrade an environment running with the image
    /// version.
    #[prost(bool, tag = "6")]
    pub upgrade_disabled: bool,
}
#[doc = r" Generated client implementations."]
pub mod image_versions_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Readonly service to query available ImageVersions."]
    #[derive(Debug, Clone)]
    pub struct ImageVersionsClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> ImageVersionsClient<T>
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
        ) -> ImageVersionsClient<InterceptedService<T, F>>
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
            ImageVersionsClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " List ImageVersions for provided location."]
        pub async fn list_image_versions(
            &mut self,
            request: impl tonic::IntoRequest<super::ListImageVersionsRequest>,
        ) -> Result<tonic::Response<super::ListImageVersionsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.orchestration.airflow.service.v1.ImageVersions/ListImageVersions",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Metadata describing an operation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationMetadata {
    /// Output only. The current operation state.
    #[prost(enumeration = "operation_metadata::State", tag = "1")]
    pub state: i32,
    /// Output only. The type of operation being performed.
    #[prost(enumeration = "operation_metadata::Type", tag = "2")]
    pub operation_type: i32,
    /// Output only. The resource being operated on, as a [relative resource name](
    /// /apis/design/resource_names#relative_resource_name).
    #[prost(string, tag = "3")]
    pub resource: ::prost::alloc::string::String,
    /// Output only. The UUID of the resource being operated on.
    #[prost(string, tag = "4")]
    pub resource_uuid: ::prost::alloc::string::String,
    /// Output only. The time the operation was submitted to the server.
    #[prost(message, optional, tag = "5")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The time when the operation terminated, regardless of its success.
    /// This field is unset if the operation is still ongoing.
    #[prost(message, optional, tag = "6")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Nested message and enum types in `OperationMetadata`.
pub mod operation_metadata {
    /// An enum describing the overall state of an operation.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// Unused.
        Unspecified = 0,
        /// The operation has been created but is not yet started.
        Pending = 1,
        /// The operation is underway.
        Running = 2,
        /// The operation completed successfully.
        Succeeded = 3,
        /// The operation is no longer running but did not succeed.
        Failed = 4,
    }
    /// Type of longrunning operation.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Type {
        /// Unused.
        Unspecified = 0,
        /// A resource creation operation.
        Create = 1,
        /// A resource deletion operation.
        Delete = 2,
        /// A resource update operation.
        Update = 3,
        /// A resource check operation.
        Check = 4,
    }
}
