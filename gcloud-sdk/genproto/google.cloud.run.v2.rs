/// Defines a status condition for a resource.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Condition {
    /// type is used to communicate the status of the reconciliation process.
    /// See also:
    /// <https://github.com/knative/serving/blob/main/docs/spec/errors.md#error-conditions-and-reporting>
    /// Types common to all resources include:
    /// * "Ready": True when the Resource is ready.
    #[prost(string, tag="1")]
    pub r#type: ::prost::alloc::string::String,
    /// State of the condition.
    #[prost(enumeration="condition::State", tag="2")]
    pub state: i32,
    /// Human readable message indicating details about the current status.
    #[prost(string, tag="3")]
    pub message: ::prost::alloc::string::String,
    /// Last time the condition transitioned from one status to another.
    #[prost(message, optional, tag="4")]
    pub last_transition_time: ::core::option::Option<::prost_types::Timestamp>,
    /// How to interpret failures of this condition, one of Error, Warning, Info
    #[prost(enumeration="condition::Severity", tag="5")]
    pub severity: i32,
    /// The reason for this condition. Depending on the condition type,
    /// it will populate one of these fields.
    /// Successful conditions may not have a reason.
    #[prost(oneof="condition::Reasons", tags="6, 9, 11")]
    pub reasons: ::core::option::Option<condition::Reasons>,
}
/// Nested message and enum types in `Condition`.
pub mod condition {
    /// Represents the possible Condition states.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// The default value. This value is used if the state is omitted.
        Unspecified = 0,
        /// Transient state: Reconciliation has not started yet.
        ConditionPending = 1,
        /// Transient state: reconciliation is still in progress.
        ConditionReconciling = 2,
        /// Terminal state: Reconciliation did not succeed.
        ConditionFailed = 3,
        /// Terminal state: Reconciliation completed successfully.
        ConditionSucceeded = 4,
    }
    impl State {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                State::Unspecified => "STATE_UNSPECIFIED",
                State::ConditionPending => "CONDITION_PENDING",
                State::ConditionReconciling => "CONDITION_RECONCILING",
                State::ConditionFailed => "CONDITION_FAILED",
                State::ConditionSucceeded => "CONDITION_SUCCEEDED",
            }
        }
    }
    /// Represents the severity of the condition failures.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Severity {
        /// Unspecified severity
        Unspecified = 0,
        /// Error severity.
        Error = 1,
        /// Warning severity.
        Warning = 2,
        /// Info severity.
        Info = 3,
    }
    impl Severity {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Severity::Unspecified => "SEVERITY_UNSPECIFIED",
                Severity::Error => "ERROR",
                Severity::Warning => "WARNING",
                Severity::Info => "INFO",
            }
        }
    }
    /// Reasons common to all types of conditions.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum CommonReason {
        /// Default value.
        Undefined = 0,
        /// Reason unknown. Further details will be in message.
        Unknown = 1,
        /// Revision creation process failed.
        RevisionFailed = 3,
        /// Timed out waiting for completion.
        ProgressDeadlineExceeded = 4,
        /// The container image path is incorrect.
        ContainerMissing = 6,
        /// Insufficient permissions on the container image.
        ContainerPermissionDenied = 7,
        /// Container image is not authorized by policy.
        ContainerImageUnauthorized = 8,
        /// Container image policy authorization check failed.
        ContainerImageAuthorizationCheckFailed = 9,
        /// Insufficient permissions on encryption key.
        EncryptionKeyPermissionDenied = 10,
        /// Permission check on encryption key failed.
        EncryptionKeyCheckFailed = 11,
        /// At least one Access check on secrets failed.
        SecretsAccessCheckFailed = 12,
        /// Waiting for operation to complete.
        WaitingForOperation = 13,
        /// System will retry immediately.
        ImmediateRetry = 14,
        /// System will retry later; current attempt failed.
        PostponedRetry = 15,
        /// An internal error occurred. Further information may be in the message.
        Internal = 16,
    }
    impl CommonReason {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                CommonReason::Undefined => "COMMON_REASON_UNDEFINED",
                CommonReason::Unknown => "UNKNOWN",
                CommonReason::RevisionFailed => "REVISION_FAILED",
                CommonReason::ProgressDeadlineExceeded => "PROGRESS_DEADLINE_EXCEEDED",
                CommonReason::ContainerMissing => "CONTAINER_MISSING",
                CommonReason::ContainerPermissionDenied => "CONTAINER_PERMISSION_DENIED",
                CommonReason::ContainerImageUnauthorized => "CONTAINER_IMAGE_UNAUTHORIZED",
                CommonReason::ContainerImageAuthorizationCheckFailed => "CONTAINER_IMAGE_AUTHORIZATION_CHECK_FAILED",
                CommonReason::EncryptionKeyPermissionDenied => "ENCRYPTION_KEY_PERMISSION_DENIED",
                CommonReason::EncryptionKeyCheckFailed => "ENCRYPTION_KEY_CHECK_FAILED",
                CommonReason::SecretsAccessCheckFailed => "SECRETS_ACCESS_CHECK_FAILED",
                CommonReason::WaitingForOperation => "WAITING_FOR_OPERATION",
                CommonReason::ImmediateRetry => "IMMEDIATE_RETRY",
                CommonReason::PostponedRetry => "POSTPONED_RETRY",
                CommonReason::Internal => "INTERNAL",
            }
        }
    }
    /// Reasons specific to Revision resource.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum RevisionReason {
        /// Default value.
        Undefined = 0,
        /// Revision in Pending state.
        Pending = 1,
        /// Revision is in Reserve state.
        Reserve = 2,
        /// Revision is Retired.
        Retired = 3,
        /// Revision is being retired.
        Retiring = 4,
        /// Revision is being recreated.
        Recreating = 5,
        /// There was a health check error.
        HealthCheckContainerError = 6,
        /// Health check failed due to user error from customized path of the
        /// container. System will retry.
        CustomizedPathResponsePending = 7,
        /// A revision with min_instance_count > 0 was created and is reserved, but
        /// it was not configured to serve traffic, so it's not live. This can also
        /// happen momentarily during traffic migration.
        MinInstancesNotProvisioned = 8,
        /// The maximum allowed number of active revisions has been reached.
        ActiveRevisionLimitReached = 9,
        /// There was no deployment defined.
        /// This value is no longer used, but Services created in older versions of
        /// the API might contain this value.
        NoDeployment = 10,
        /// A revision's container has no port specified since the revision is of a
        /// manually scaled service with 0 instance count
        HealthCheckSkipped = 11,
    }
    impl RevisionReason {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                RevisionReason::Undefined => "REVISION_REASON_UNDEFINED",
                RevisionReason::Pending => "PENDING",
                RevisionReason::Reserve => "RESERVE",
                RevisionReason::Retired => "RETIRED",
                RevisionReason::Retiring => "RETIRING",
                RevisionReason::Recreating => "RECREATING",
                RevisionReason::HealthCheckContainerError => "HEALTH_CHECK_CONTAINER_ERROR",
                RevisionReason::CustomizedPathResponsePending => "CUSTOMIZED_PATH_RESPONSE_PENDING",
                RevisionReason::MinInstancesNotProvisioned => "MIN_INSTANCES_NOT_PROVISIONED",
                RevisionReason::ActiveRevisionLimitReached => "ACTIVE_REVISION_LIMIT_REACHED",
                RevisionReason::NoDeployment => "NO_DEPLOYMENT",
                RevisionReason::HealthCheckSkipped => "HEALTH_CHECK_SKIPPED",
            }
        }
    }
    /// Reasons specific to Execution resource.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ExecutionReason {
        /// Default value.
        Undefined = 0,
        /// Internal system error getting execution status. System will retry.
        JobStatusServicePollingError = 1,
        /// A task reached its retry limit and the last attempt failed due to the
        /// user container exiting with a non-zero exit code.
        NonZeroExitCode = 2,
    }
    impl ExecutionReason {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ExecutionReason::Undefined => "EXECUTION_REASON_UNDEFINED",
                ExecutionReason::JobStatusServicePollingError => "JOB_STATUS_SERVICE_POLLING_ERROR",
                ExecutionReason::NonZeroExitCode => "NON_ZERO_EXIT_CODE",
            }
        }
    }
    /// The reason for this condition. Depending on the condition type,
    /// it will populate one of these fields.
    /// Successful conditions may not have a reason.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Reasons {
        /// A common (service-level) reason for this condition.
        #[prost(enumeration="CommonReason", tag="6")]
        Reason(i32),
        /// A reason for the revision condition.
        #[prost(enumeration="RevisionReason", tag="9")]
        RevisionReason(i32),
        /// A reason for the execution condition.
        #[prost(enumeration="ExecutionReason", tag="11")]
        ExecutionReason(i32),
    }
}
/// A single application container.
/// This specifies both the container to run, the command to run in the container
/// and the arguments to supply to it.
/// Note that additional arguments may be supplied by the system to the container
/// at runtime.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Container {
    /// Name of the container specified as a DNS_LABEL.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Required. URL of the Container image in Google Container Registry or Google Artifact
    /// Registry. More info: <https://kubernetes.io/docs/concepts/containers/images>
    #[prost(string, tag="2")]
    pub image: ::prost::alloc::string::String,
    /// Entrypoint array. Not executed within a shell.
    /// The docker image's ENTRYPOINT is used if this is not provided.
    /// Variable references $(VAR_NAME) are expanded using the container's
    /// environment. If a variable cannot be resolved, the reference in the input
    /// string will be unchanged. The $(VAR_NAME) syntax can be escaped with a
    /// double $$, ie: $$(VAR_NAME). Escaped references will never be expanded,
    /// regardless of whether the variable exists or not.
    /// More info:
    /// <https://kubernetes.io/docs/tasks/inject-data-application/define-command-argument-container/#running-a-command-in-a-shell>
    #[prost(string, repeated, tag="3")]
    pub command: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Arguments to the entrypoint.
    /// The docker image's CMD is used if this is not provided.
    /// Variable references $(VAR_NAME) are expanded using the container's
    /// environment. If a variable cannot be resolved, the reference in the input
    /// string will be unchanged. The $(VAR_NAME) syntax can be escaped with a
    /// double $$, ie: $$(VAR_NAME). Escaped references will never be expanded,
    /// regardless of whether the variable exists or not.
    /// More info:
    /// <https://kubernetes.io/docs/tasks/inject-data-application/define-command-argument-container/#running-a-command-in-a-shell>
    #[prost(string, repeated, tag="4")]
    pub args: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// List of environment variables to set in the container.
    #[prost(message, repeated, tag="5")]
    pub env: ::prost::alloc::vec::Vec<EnvVar>,
    /// Compute Resource requirements by this container.
    /// More info:
    /// <https://kubernetes.io/docs/concepts/storage/persistent-volumes#resources>
    #[prost(message, optional, tag="6")]
    pub resources: ::core::option::Option<ResourceRequirements>,
    /// List of ports to expose from the container. Only a single port can be
    /// specified. The specified ports must be listening on all interfaces
    /// (0.0.0.0) within the container to be accessible.
    ///
    /// If omitted, a port number will be chosen and passed to the container
    /// through the PORT environment variable for the container to listen on.
    #[prost(message, repeated, tag="7")]
    pub ports: ::prost::alloc::vec::Vec<ContainerPort>,
    /// Volume to mount into the container's filesystem.
    #[prost(message, repeated, tag="8")]
    pub volume_mounts: ::prost::alloc::vec::Vec<VolumeMount>,
}
/// ResourceRequirements describes the compute resource requirements.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResourceRequirements {
    /// Only memory and CPU are supported. Note: The only
    /// supported values for CPU are '1', '2', and '4'. Setting 4 CPU requires at
    /// least 2Gi of memory.
    /// The values of the map is string form of the 'quantity' k8s type:
    /// <https://github.com/kubernetes/kubernetes/blob/master/staging/src/k8s.io/apimachinery/pkg/api/resource/quantity.go>
    #[prost(map="string, string", tag="1")]
    pub limits: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Determines whether CPU should be throttled or not outside of requests.
    #[prost(bool, tag="2")]
    pub cpu_idle: bool,
}
/// EnvVar represents an environment variable present in a Container.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EnvVar {
    /// Required. Name of the environment variable. Must be a C_IDENTIFIER, and mnay not
    /// exceed 32768 characters.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(oneof="env_var::Values", tags="2, 3")]
    pub values: ::core::option::Option<env_var::Values>,
}
/// Nested message and enum types in `EnvVar`.
pub mod env_var {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Values {
        /// Variable references $(VAR_NAME) are expanded
        /// using the previous defined environment variables in the container and
        /// any route environment variables. If a variable cannot be resolved,
        /// the reference in the input string will be unchanged. The $(VAR_NAME)
        /// syntax can be escaped with a double $$, ie: $$(VAR_NAME). Escaped
        /// references will never be expanded, regardless of whether the variable
        /// exists or not.
        /// Defaults to "", and the maximum length is 32768 bytes.
        #[prost(string, tag="2")]
        Value(::prost::alloc::string::String),
        /// Source for the environment variable's value.
        #[prost(message, tag="3")]
        ValueSource(super::EnvVarSource),
    }
}
/// EnvVarSource represents a source for the value of an EnvVar.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EnvVarSource {
    /// Selects a secret and a specific version from Cloud Secret Manager.
    #[prost(message, optional, tag="1")]
    pub secret_key_ref: ::core::option::Option<SecretKeySelector>,
}
/// SecretEnvVarSource represents a source for the value of an EnvVar.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SecretKeySelector {
    /// Required. The name of the secret in Cloud Secret Manager.
    /// Format: {secret_name} if the secret is in the same project.
    /// projects/{project}/secrets/{secret_name} if the secret is
    /// in a different project.
    #[prost(string, tag="1")]
    pub secret: ::prost::alloc::string::String,
    /// The Cloud Secret Manager secret version.
    /// Can be 'latest' for the latest value or an integer for a specific version.
    #[prost(string, tag="2")]
    pub version: ::prost::alloc::string::String,
}
/// ContainerPort represents a network port in a single container.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContainerPort {
    /// If specified, used to specify which protocol to use.
    /// Allowed values are "http1" and "h2c".
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Port number the container listens on.
    /// This must be a valid TCP port number, 0 < container_port < 65536.
    #[prost(int32, tag="3")]
    pub container_port: i32,
}
/// VolumeMount describes a mounting of a Volume within a container.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VolumeMount {
    /// Required. This must match the Name of a Volume.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Required. Path within the container at which the volume should be mounted.  Must
    /// not contain ':'. For Cloud SQL volumes, it can be left empty, or must
    /// otherwise be `/cloudsql`. All instances defined in the Volume will be
    /// available as `/cloudsql/\[instance\]`. For more information on Cloud SQL
    /// volumes, visit <https://cloud.google.com/sql/docs/mysql/connect-run>
    #[prost(string, tag="3")]
    pub mount_path: ::prost::alloc::string::String,
}
/// Volume represents a named volume in a container.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Volume {
    /// Required. Volume's name.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(oneof="volume::VolumeType", tags="2, 3")]
    pub volume_type: ::core::option::Option<volume::VolumeType>,
}
/// Nested message and enum types in `Volume`.
pub mod volume {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum VolumeType {
        /// Secret represents a secret that should populate this volume.
        /// More info: <https://kubernetes.io/docs/concepts/storage/volumes#secret>
        #[prost(message, tag="2")]
        Secret(super::SecretVolumeSource),
        /// For Cloud SQL volumes, contains the specific instances that should be
        /// mounted. Visit <https://cloud.google.com/sql/docs/mysql/connect-run> for
        /// more information on how to connect Cloud SQL and Cloud Run.
        #[prost(message, tag="3")]
        CloudSqlInstance(super::CloudSqlInstance),
    }
}
/// The secret's value will be presented as the content of a file whose
/// name is defined in the item path. If no items are defined, the name of
/// the file is the secret.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SecretVolumeSource {
    /// Required. The name of the secret in Cloud Secret Manager.
    /// Format: {secret} if the secret is in the same project.
    /// projects/{project}/secrets/{secret} if the secret is
    /// in a different project.
    #[prost(string, tag="1")]
    pub secret: ::prost::alloc::string::String,
    /// If unspecified, the volume will expose a file whose name is the
    /// secret, relative to VolumeMount.mount_path.
    /// If specified, the key will be used as the version to fetch from Cloud
    /// Secret Manager and the path will be the name of the file exposed in the
    /// volume. When items are defined, they must specify a path and a version.
    #[prost(message, repeated, tag="2")]
    pub items: ::prost::alloc::vec::Vec<VersionToPath>,
    /// Integer representation of mode bits to use on created files by default.
    /// Must be a value between 0000 and 0777 (octal), defaulting to 0644.
    /// Directories within the path are not affected by  this setting.
    ///
    /// Notes
    ///
    /// * Internally, a umask of 0222 will be applied to any non-zero value.
    /// * This is an integer representation of the mode bits. So, the octal
    /// integer value should look exactly as the chmod numeric notation with a
    /// leading zero. Some examples: for chmod 777 (a=rwx), set to 0777 (octal) or
    /// 511 (base-10). For chmod 640 (u=rw,g=r), set to 0640 (octal) or
    /// 416 (base-10). For chmod 755 (u=rwx,g=rx,o=rx), set to 0755 (octal) or 493
    /// (base-10).
    /// * This might be in conflict with other options that affect the
    /// file mode, like fsGroup, and the result can be other mode bits set.
    ///
    /// This might be in conflict with other options that affect the
    /// file mode, like fsGroup, and as a result, other mode bits could be set.
    #[prost(int32, tag="3")]
    pub default_mode: i32,
}
/// VersionToPath maps a specific version of a secret to a relative file to mount
/// to, relative to VolumeMount's mount_path.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VersionToPath {
    /// Required. The relative path of the secret in the container.
    #[prost(string, tag="1")]
    pub path: ::prost::alloc::string::String,
    /// The Cloud Secret Manager secret version.
    /// Can be 'latest' for the latest value or an integer for a specific version.
    #[prost(string, tag="2")]
    pub version: ::prost::alloc::string::String,
    /// Integer octal mode bits to use on this file, must be a value between
    /// 01 and 0777 (octal). If 0 or not set, the Volume's default mode will be
    /// used.
    ///
    /// Notes
    ///
    /// * Internally, a umask of 0222 will be applied to any non-zero value.
    /// * This is an integer representation of the mode bits. So, the octal
    /// integer value should look exactly as the chmod numeric notation with a
    /// leading zero. Some examples: for chmod 777 (a=rwx), set to 0777 (octal) or
    /// 511 (base-10). For chmod 640 (u=rw,g=r), set to 0640 (octal) or
    /// 416 (base-10). For chmod 755 (u=rwx,g=rx,o=rx), set to 0755 (octal) or 493
    /// (base-10).
    /// * This might be in conflict with other options that affect the
    /// file mode, like fsGroup, and the result can be other mode bits set.
    #[prost(int32, tag="3")]
    pub mode: i32,
}
/// Represents a specific Cloud SQL instance.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CloudSqlInstance {
    /// The Cloud SQL instance connection names, as can be found in
    /// <https://console.cloud.google.com/sql/instances.> Visit
    /// <https://cloud.google.com/sql/docs/mysql/connect-run> for more information on
    /// how to connect Cloud SQL and Cloud Run. Format:
    /// {project}:{location}:{instance}
    #[prost(string, repeated, tag="1")]
    pub instances: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// VPC Access settings. For more information on creating a VPC Connector, visit
/// <https://cloud.google.com/vpc/docs/configure-serverless-vpc-access> For
/// information on how to configure Cloud Run with an existing VPC Connector,
/// visit <https://cloud.google.com/run/docs/configuring/connecting-vpc>
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VpcAccess {
    /// VPC Access connector name.
    /// Format: projects/{project}/locations/{location}/connectors/{connector}
    #[prost(string, tag="1")]
    pub connector: ::prost::alloc::string::String,
    /// Traffic VPC egress settings.
    #[prost(enumeration="vpc_access::VpcEgress", tag="2")]
    pub egress: i32,
}
/// Nested message and enum types in `VpcAccess`.
pub mod vpc_access {
    /// Egress options for VPC access.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum VpcEgress {
        /// Unspecified
        Unspecified = 0,
        /// All outbound traffic is routed through the VPC connector.
        AllTraffic = 1,
        /// Only private IP ranges are routed through the VPC connector.
        PrivateRangesOnly = 2,
    }
    impl VpcEgress {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                VpcEgress::Unspecified => "VPC_EGRESS_UNSPECIFIED",
                VpcEgress::AllTraffic => "ALL_TRAFFIC",
                VpcEgress::PrivateRangesOnly => "PRIVATE_RANGES_ONLY",
            }
        }
    }
}
/// Settings for Binary Authorization feature.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BinaryAuthorization {
    /// If present, indicates to use Breakglass using this justification.
    /// If use_default is False, then it must be empty.
    /// For more information on breakglass, see
    /// <https://cloud.google.com/binary-authorization/docs/using-breakglass>
    #[prost(string, tag="2")]
    pub breakglass_justification: ::prost::alloc::string::String,
    #[prost(oneof="binary_authorization::BinauthzMethod", tags="1")]
    pub binauthz_method: ::core::option::Option<binary_authorization::BinauthzMethod>,
}
/// Nested message and enum types in `BinaryAuthorization`.
pub mod binary_authorization {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum BinauthzMethod {
        /// If True, indicates to use the default project's binary authorization
        /// policy. If False, binary authorization will be disabled.
        #[prost(bool, tag="1")]
        UseDefault(bool),
    }
}
/// Settings for revision-level scaling settings.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RevisionScaling {
    /// Minimum number of serving instances that this resource should have.
    #[prost(int32, tag="1")]
    pub min_instance_count: i32,
    /// Maximum number of serving instances that this resource should have.
    #[prost(int32, tag="2")]
    pub max_instance_count: i32,
}
/// Allowed ingress traffic for the Container.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum IngressTraffic {
    /// Unspecified
    Unspecified = 0,
    /// All inbound traffic is allowed.
    All = 1,
    /// Only internal traffic is allowed.
    InternalOnly = 2,
    /// Both internal and Google Cloud Load Balancer traffic is allowed.
    InternalLoadBalancer = 3,
}
impl IngressTraffic {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            IngressTraffic::Unspecified => "INGRESS_TRAFFIC_UNSPECIFIED",
            IngressTraffic::All => "INGRESS_TRAFFIC_ALL",
            IngressTraffic::InternalOnly => "INGRESS_TRAFFIC_INTERNAL_ONLY",
            IngressTraffic::InternalLoadBalancer => "INGRESS_TRAFFIC_INTERNAL_LOAD_BALANCER",
        }
    }
}
/// Alternatives for execution environments.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ExecutionEnvironment {
    /// Unspecified
    Unspecified = 0,
    /// Uses the First Generation environment.
    Gen1 = 1,
    /// Uses Second Generation environment.
    Gen2 = 2,
}
impl ExecutionEnvironment {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ExecutionEnvironment::Unspecified => "EXECUTION_ENVIRONMENT_UNSPECIFIED",
            ExecutionEnvironment::Gen1 => "EXECUTION_ENVIRONMENT_GEN1",
            ExecutionEnvironment::Gen2 => "EXECUTION_ENVIRONMENT_GEN2",
        }
    }
}
/// Request message for obtaining a Revision by its full name.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRevisionRequest {
    /// Required. The full name of the Revision.
    /// Format:
    /// projects/{project}/locations/{location}/services/{service}/revisions/{revision}
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for retrieving a list of Revisions.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListRevisionsRequest {
    /// Required. The Service from which the Revisions should be listed.
    /// To list all Revisions across Services, use "-" instead of Service name.
    /// Format:
    /// projects/{project}/locations/{location}/services/{service}
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Maximum number of revisions to return in this call.
    #[prost(int32, tag="2")]
    pub page_size: i32,
    /// A page token received from a previous call to ListRevisions.
    /// All other parameters must match.
    #[prost(string, tag="3")]
    pub page_token: ::prost::alloc::string::String,
    /// If true, returns deleted (but unexpired) resources along with active ones.
    #[prost(bool, tag="4")]
    pub show_deleted: bool,
}
/// Response message containing a list of Revisions.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListRevisionsResponse {
    /// The resulting list of Revisions.
    #[prost(message, repeated, tag="1")]
    pub revisions: ::prost::alloc::vec::Vec<Revision>,
    /// A token indicating there are more items than page_size. Use it in the next
    /// ListRevisions request to continue.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for deleting a retired Revision.
/// Revision lifecycle is usually managed by making changes to the parent
/// Service. Only retired revisions can be deleted with this API.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteRevisionRequest {
    /// Required. The name of the Revision to delete.
    /// Format:
    /// projects/{project}/locations/{location}/services/{service}/revisions/{revision}
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Indicates that the request should be validated without actually
    /// deleting any resources.
    #[prost(bool, tag="2")]
    pub validate_only: bool,
    /// A system-generated fingerprint for this version of the
    /// resource. This may be used to detect modification conflict during updates.
    #[prost(string, tag="3")]
    pub etag: ::prost::alloc::string::String,
}
/// A Revision is an immutable snapshot of code and configuration.  A Revision
/// references a container image. Revisions are only created by updates to its
/// parent Service.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Revision {
    /// Output only. The unique name of this Revision.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. Server assigned unique identifier for the Revision. The value is a UUID4
    /// string and guaranteed to remain unchanged until the resource is deleted.
    #[prost(string, tag="2")]
    pub uid: ::prost::alloc::string::String,
    /// Output only. A number that monotonically increases every time the user
    /// modifies the desired state.
    #[prost(int64, tag="3")]
    pub generation: i64,
    /// KRM-style labels for the resource.
    /// User-provided labels are shared with Google's billing system, so they can
    /// be used to filter, or break down billing charges by team, component,
    /// environment, state, etc. For more information, visit
    /// <https://cloud.google.com/resource-manager/docs/creating-managing-labels> or
    /// <https://cloud.google.com/run/docs/configuring/labels>
    /// Cloud Run will populate some labels with 'run.googleapis.com' or
    /// 'serving.knative.dev' namespaces. Those labels are read-only, and user
    /// changes will not be preserved.
    #[prost(map="string, string", tag="4")]
    pub labels: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// KRM-style annotations for the resource.
    #[prost(map="string, string", tag="5")]
    pub annotations: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Output only. The creation time.
    #[prost(message, optional, tag="6")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The last-modified time.
    #[prost(message, optional, tag="7")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. For a deleted resource, the deletion time. It is only
    /// populated as a response to a Delete request.
    #[prost(message, optional, tag="8")]
    pub delete_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. For a deleted resource, the time after which it will be
    /// permamently deleted. It is only populated as a response to a Delete
    /// request.
    #[prost(message, optional, tag="9")]
    pub expire_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Set the launch stage to a preview stage on write to allow use of preview
    /// features in that stage. On read, describes whether the resource uses
    /// preview features. Launch Stages are defined at [Google Cloud Platform
    /// Launch Stages](<https://cloud.google.com/terms/launch-stages>).
    #[prost(enumeration="super::super::super::api::LaunchStage", tag="10")]
    pub launch_stage: i32,
    /// Output only. The name of the parent service.
    #[prost(string, tag="11")]
    pub service: ::prost::alloc::string::String,
    /// Scaling settings for this revision.
    #[prost(message, optional, tag="12")]
    pub scaling: ::core::option::Option<RevisionScaling>,
    /// VPC Access configuration for this Revision. For more information, visit
    /// <https://cloud.google.com/run/docs/configuring/connecting-vpc.>
    #[prost(message, optional, tag="13")]
    pub vpc_access: ::core::option::Option<VpcAccess>,
    /// Sets the maximum number of requests that each serving instance can receive.
    #[prost(int32, tag="34")]
    pub max_instance_request_concurrency: i32,
    /// Max allowed time for an instance to respond to a request.
    #[prost(message, optional, tag="15")]
    pub timeout: ::core::option::Option<::prost_types::Duration>,
    /// Email address of the IAM service account associated with the revision of
    /// the service. The service account represents the identity of the running
    /// revision, and determines what permissions the revision has.
    #[prost(string, tag="16")]
    pub service_account: ::prost::alloc::string::String,
    /// Holds the single container that defines the unit of execution for this
    /// Revision.
    #[prost(message, repeated, tag="17")]
    pub containers: ::prost::alloc::vec::Vec<Container>,
    /// A list of Volumes to make available to containers.
    #[prost(message, repeated, tag="18")]
    pub volumes: ::prost::alloc::vec::Vec<Volume>,
    /// The execution environment being used to host this Revision.
    #[prost(enumeration="ExecutionEnvironment", tag="20")]
    pub execution_environment: i32,
    /// A reference to a customer managed encryption key (CMEK) to use to encrypt
    /// this container image. For more information, go to
    /// <https://cloud.google.com/run/docs/securing/using-cmek>
    #[prost(string, tag="21")]
    pub encryption_key: ::prost::alloc::string::String,
    /// Output only. Indicates whether the resource's reconciliation is still in progress.
    /// See comments in `Service.reconciling` for additional information on
    /// reconciliation process in Cloud Run.
    #[prost(bool, tag="30")]
    pub reconciling: bool,
    /// Output only. The Condition of this Revision, containing its readiness status, and
    /// detailed error information in case it did not reach a serving state.
    #[prost(message, repeated, tag="31")]
    pub conditions: ::prost::alloc::vec::Vec<Condition>,
    /// Output only. The generation of this Revision currently serving traffic. See comments in
    /// `reconciling` for additional information on reconciliation process in Cloud
    /// Run.
    #[prost(int64, tag="32")]
    pub observed_generation: i64,
    /// Output only. The Google Console URI to obtain logs for the Revision.
    #[prost(string, tag="33")]
    pub log_uri: ::prost::alloc::string::String,
    /// Output only. A system-generated fingerprint for this version of the
    /// resource. May be used to detect modification conflict during updates.
    #[prost(string, tag="99")]
    pub etag: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod revisions_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Cloud Run Revision Control Plane API.
    #[derive(Debug, Clone)]
    pub struct RevisionsClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl RevisionsClient<tonic::transport::Channel> {
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
    impl<T> RevisionsClient<T>
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
        ) -> RevisionsClient<InterceptedService<T, F>>
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
            RevisionsClient::new(InterceptedService::new(inner, interceptor))
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
        /// Gets information about a Revision.
        pub async fn get_revision(
            &mut self,
            request: impl tonic::IntoRequest<super::GetRevisionRequest>,
        ) -> Result<tonic::Response<super::Revision>, tonic::Status> {
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
                "/google.cloud.run.v2.Revisions/GetRevision",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// List Revisions from a given Service, or from a given location.
        pub async fn list_revisions(
            &mut self,
            request: impl tonic::IntoRequest<super::ListRevisionsRequest>,
        ) -> Result<tonic::Response<super::ListRevisionsResponse>, tonic::Status> {
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
                "/google.cloud.run.v2.Revisions/ListRevisions",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Delete a Revision.
        pub async fn delete_revision(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteRevisionRequest>,
        ) -> Result<
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
                "/google.cloud.run.v2.Revisions/DeleteRevision",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// RevisionTemplate describes the data a revision should have when created from
/// a template.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RevisionTemplate {
    /// The unique name for the revision. If this field is omitted, it will be
    /// automatically generated based on the Service name.
    #[prost(string, tag="1")]
    pub revision: ::prost::alloc::string::String,
    /// KRM-style labels for the resource.
    #[prost(map="string, string", tag="2")]
    pub labels: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// KRM-style annotations for the resource.
    #[prost(map="string, string", tag="3")]
    pub annotations: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Scaling settings for this Revision.
    #[prost(message, optional, tag="4")]
    pub scaling: ::core::option::Option<RevisionScaling>,
    /// VPC Access configuration to use for this Revision. For more information,
    /// visit <https://cloud.google.com/run/docs/configuring/connecting-vpc.>
    #[prost(message, optional, tag="6")]
    pub vpc_access: ::core::option::Option<VpcAccess>,
    /// Max allowed time for an instance to respond to a request.
    #[prost(message, optional, tag="8")]
    pub timeout: ::core::option::Option<::prost_types::Duration>,
    /// Email address of the IAM service account associated with the revision of
    /// the service. The service account represents the identity of the running
    /// revision, and determines what permissions the revision has. If not
    /// provided, the revision will use the project's default service account.
    #[prost(string, tag="9")]
    pub service_account: ::prost::alloc::string::String,
    /// Holds the single container that defines the unit of execution for this
    /// Revision.
    #[prost(message, repeated, tag="10")]
    pub containers: ::prost::alloc::vec::Vec<Container>,
    /// A list of Volumes to make available to containers.
    #[prost(message, repeated, tag="11")]
    pub volumes: ::prost::alloc::vec::Vec<Volume>,
    /// The sandbox environment to host this Revision.
    #[prost(enumeration="ExecutionEnvironment", tag="13")]
    pub execution_environment: i32,
    /// A reference to a customer managed encryption key (CMEK) to use to encrypt
    /// this container image. For more information, go to
    /// <https://cloud.google.com/run/docs/securing/using-cmek>
    #[prost(string, tag="14")]
    pub encryption_key: ::prost::alloc::string::String,
    /// Sets the maximum number of requests that each serving instance can receive.
    #[prost(int32, tag="15")]
    pub max_instance_request_concurrency: i32,
}
/// Holds a single traffic routing entry for the Service. Allocations can be done
/// to a specific Revision name, or pointing to the latest Ready Revision.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TrafficTarget {
    /// The allocation type for this traffic target.
    #[prost(enumeration="TrafficTargetAllocationType", tag="1")]
    pub r#type: i32,
    /// Revision to which to send this portion of traffic, if traffic allocation is
    /// by revision.
    #[prost(string, tag="2")]
    pub revision: ::prost::alloc::string::String,
    /// Specifies percent of the traffic to this Revision.
    /// This defaults to zero if unspecified.
    #[prost(int32, tag="3")]
    pub percent: i32,
    /// Indicates a string to be part of the URI to exclusively reference this
    /// target.
    #[prost(string, tag="4")]
    pub tag: ::prost::alloc::string::String,
}
/// Represents the observed state of a single `TrafficTarget` entry.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TrafficTargetStatus {
    /// The allocation type for this traffic target.
    #[prost(enumeration="TrafficTargetAllocationType", tag="1")]
    pub r#type: i32,
    /// Revision to which this traffic is sent.
    #[prost(string, tag="2")]
    pub revision: ::prost::alloc::string::String,
    /// Specifies percent of the traffic to this Revision.
    #[prost(int32, tag="3")]
    pub percent: i32,
    /// Indicates the string used in the URI to exclusively reference this target.
    #[prost(string, tag="4")]
    pub tag: ::prost::alloc::string::String,
    /// Displays the target URI.
    #[prost(string, tag="5")]
    pub uri: ::prost::alloc::string::String,
}
/// The type of instance allocation.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TrafficTargetAllocationType {
    /// Unspecified instance allocation type.
    Unspecified = 0,
    /// Allocates instances to the Service's latest ready Revision.
    Latest = 1,
    /// Allocates instances to a Revision by name.
    Revision = 2,
}
impl TrafficTargetAllocationType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            TrafficTargetAllocationType::Unspecified => "TRAFFIC_TARGET_ALLOCATION_TYPE_UNSPECIFIED",
            TrafficTargetAllocationType::Latest => "TRAFFIC_TARGET_ALLOCATION_TYPE_LATEST",
            TrafficTargetAllocationType::Revision => "TRAFFIC_TARGET_ALLOCATION_TYPE_REVISION",
        }
    }
}
/// Request message for creating a Service.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateServiceRequest {
    /// Required. The location and project in which this service should be created.
    /// Format: projects/{projectnumber}/locations/{location}
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The Service instance to create.
    #[prost(message, optional, tag="2")]
    pub service: ::core::option::Option<Service>,
    /// Required. The unique identifier for the Service. The name of the service becomes
    /// {parent}/services/{service_id}.
    #[prost(string, tag="3")]
    pub service_id: ::prost::alloc::string::String,
    /// Indicates that the request should be validated and default values
    /// populated, without persisting the request or creating any resources.
    #[prost(bool, tag="4")]
    pub validate_only: bool,
}
/// Request message for updating a service.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateServiceRequest {
    /// Required. The Service to be updated.
    #[prost(message, optional, tag="1")]
    pub service: ::core::option::Option<Service>,
    /// Indicates that the request should be validated and default values
    /// populated, without persisting the request or updating any resources.
    #[prost(bool, tag="3")]
    pub validate_only: bool,
    /// If set to true, and if the Service does not exist, it will create a new
    /// one. Caller must have both create and update permissions for this call if
    /// this is set to true.
    #[prost(bool, tag="4")]
    pub allow_missing: bool,
}
/// Request message for retrieving a list of Services.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListServicesRequest {
    /// Required. The location and project to list resources on.
    /// Location must be a valid GCP region, and may not be the "-" wildcard.
    /// Format: projects/{projectnumber}/locations/{location}
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Maximum number of Services to return in this call.
    #[prost(int32, tag="2")]
    pub page_size: i32,
    /// A page token received from a previous call to ListServices.
    /// All other parameters must match.
    #[prost(string, tag="3")]
    pub page_token: ::prost::alloc::string::String,
    /// If true, returns deleted (but unexpired) resources along with active ones.
    #[prost(bool, tag="4")]
    pub show_deleted: bool,
}
/// Response message containing a list of Services.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListServicesResponse {
    /// The resulting list of Services.
    #[prost(message, repeated, tag="1")]
    pub services: ::prost::alloc::vec::Vec<Service>,
    /// A token indicating there are more items than page_size. Use it in the next
    /// ListServices request to continue.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for obtaining a Service by its full name.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetServiceRequest {
    /// Required. The full name of the Service.
    /// Format: projects/{projectnumber}/locations/{location}/services/{service}
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message to delete a Service by its full name.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteServiceRequest {
    /// Required. The full name of the Service.
    /// Format: projects/{projectnumber}/locations/{location}/services/{service}
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Indicates that the request should be validated without actually
    /// deleting any resources.
    #[prost(bool, tag="2")]
    pub validate_only: bool,
    /// A system-generated fingerprint for this version of the
    /// resource. May be used to detect modification conflict during updates.
    #[prost(string, tag="3")]
    pub etag: ::prost::alloc::string::String,
}
/// Service acts as a top-level container that manages a set of
/// configurations and revision templates which implement a network service.
/// Service exists to provide a singular abstraction which can be access
/// controlled, reasoned about, and which encapsulates software lifecycle
/// decisions such as rollout policy and team resource ownership.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Service {
    /// The fully qualified name of this Service. In CreateServiceRequest, this
    /// field is ignored, and instead composed from CreateServiceRequest.parent and
    /// CreateServiceRequest.service_id.
    ///
    /// Format:
    /// projects/{project}/locations/{location}/services/{service_id}
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// User-provided description of the Service. This field currently has a
    /// 512-character limit.
    #[prost(string, tag="2")]
    pub description: ::prost::alloc::string::String,
    /// Output only. Server assigned unique identifier for the trigger. The value is a UUID4
    /// string and guaranteed to remain unchanged until the resource is deleted.
    #[prost(string, tag="3")]
    pub uid: ::prost::alloc::string::String,
    /// Output only. A number that monotonically increases every time the user
    /// modifies the desired state.
    /// Please note that unlike v1, this is an int64 value. As with most Google
    /// APIs, its JSON representation will be a `string` instead of an `integer`.
    #[prost(int64, tag="4")]
    pub generation: i64,
    /// Map of string keys and values that can be used to organize and categorize
    /// objects.
    /// User-provided labels are shared with Google's billing system, so they can
    /// be used to filter, or break down billing charges by team, component,
    /// environment, state, etc. For more information, visit
    /// <https://cloud.google.com/resource-manager/docs/creating-managing-labels> or
    /// <https://cloud.google.com/run/docs/configuring/labels>
    /// Cloud Run will populate some labels with 'run.googleapis.com' or
    /// 'serving.knative.dev' namespaces. Those labels are read-only, and user
    /// changes will not be preserved.
    #[prost(map="string, string", tag="5")]
    pub labels: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Unstructured key value map that may be set by external tools to store and
    /// arbitrary metadata. They are not queryable and should be preserved
    /// when modifying objects. Cloud Run will populate some annotations using
    /// 'run.googleapis.com' or 'serving.knative.dev' namespaces. This field
    /// follows Kubernetes annotations' namespacing, limits, and rules. More info:
    /// <https://kubernetes.io/docs/user-guide/annotations>
    #[prost(map="string, string", tag="6")]
    pub annotations: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Output only. The creation time.
    #[prost(message, optional, tag="7")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The last-modified time.
    #[prost(message, optional, tag="8")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The deletion time.
    #[prost(message, optional, tag="9")]
    pub delete_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. For a deleted resource, the time after which it will be
    /// permamently deleted.
    #[prost(message, optional, tag="10")]
    pub expire_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Email address of the authenticated creator.
    #[prost(string, tag="11")]
    pub creator: ::prost::alloc::string::String,
    /// Output only. Email address of the last authenticated modifier.
    #[prost(string, tag="12")]
    pub last_modifier: ::prost::alloc::string::String,
    /// Arbitrary identifier for the API client.
    #[prost(string, tag="13")]
    pub client: ::prost::alloc::string::String,
    /// Arbitrary version identifier for the API client.
    #[prost(string, tag="14")]
    pub client_version: ::prost::alloc::string::String,
    /// Provides the ingress settings for this Service. On output, returns the
    /// currently observed ingress settings, or INGRESS_TRAFFIC_UNSPECIFIED if no
    /// revision is active.
    #[prost(enumeration="IngressTraffic", tag="15")]
    pub ingress: i32,
    /// The launch stage as defined by [Google Cloud Platform
    /// Launch Stages](<https://cloud.google.com/terms/launch-stages>).
    /// Cloud Run supports `ALPHA`, `BETA`, and `GA`. If no value is specified, GA
    /// is assumed.
    #[prost(enumeration="super::super::super::api::LaunchStage", tag="16")]
    pub launch_stage: i32,
    /// Settings for the Binary Authorization feature.
    #[prost(message, optional, tag="17")]
    pub binary_authorization: ::core::option::Option<BinaryAuthorization>,
    /// Required. The template used to create revisions for this Service.
    #[prost(message, optional, tag="18")]
    pub template: ::core::option::Option<RevisionTemplate>,
    /// Specifies how to distribute traffic over a collection of Revisions
    /// belonging to the Service. If traffic is empty or not provided, defaults to
    /// 100% traffic to the latest `Ready` Revision.
    #[prost(message, repeated, tag="19")]
    pub traffic: ::prost::alloc::vec::Vec<TrafficTarget>,
    /// Output only. The generation of this Service currently serving traffic. See comments in
    /// `reconciling` for additional information on reconciliation process in Cloud
    /// Run.
    /// Please note that unlike v1, this is an int64 value. As with most Google
    /// APIs, its JSON representation will be a `string` instead of an `integer`.
    #[prost(int64, tag="30")]
    pub observed_generation: i64,
    /// Output only. The Condition of this Service, containing its readiness status, and
    /// detailed error information in case it did not reach a serving state. See
    /// comments in `reconciling` for additional information on reconciliation
    /// process in Cloud Run.
    #[prost(message, optional, tag="31")]
    pub terminal_condition: ::core::option::Option<Condition>,
    /// Output only. The Conditions of all other associated sub-resources. They contain
    /// additional diagnostics information in case the Service does not reach its
    /// Serving state. See comments in `reconciling` for additional information on
    /// reconciliation process in Cloud Run.
    #[prost(message, repeated, tag="32")]
    pub conditions: ::prost::alloc::vec::Vec<Condition>,
    /// Output only. Name of the latest revision that is serving traffic. See comments in
    /// `reconciling` for additional information on reconciliation process in Cloud
    /// Run.
    #[prost(string, tag="33")]
    pub latest_ready_revision: ::prost::alloc::string::String,
    /// Output only. Name of the last created revision. See comments in `reconciling` for
    /// additional information on reconciliation process in Cloud Run.
    #[prost(string, tag="34")]
    pub latest_created_revision: ::prost::alloc::string::String,
    /// Output only. Detailed status information for corresponding traffic targets. See comments
    /// in `reconciling` for additional information on reconciliation process in
    /// Cloud Run.
    #[prost(message, repeated, tag="35")]
    pub traffic_statuses: ::prost::alloc::vec::Vec<TrafficTargetStatus>,
    /// Output only. The main URI in which this Service is serving traffic.
    #[prost(string, tag="36")]
    pub uri: ::prost::alloc::string::String,
    /// Output only. Returns true if the Service is currently being acted upon by the system to
    /// bring it into the desired state.
    ///
    /// When a new Service is created, or an existing one is updated, Cloud Run
    /// will asynchronously perform all necessary steps to bring the Service to the
    /// desired serving state. This process is called reconciliation.
    /// While reconciliation is in process, `observed_generation`,
    /// `latest_ready_revison`, `traffic_statuses`, and `uri` will have transient
    /// values that might mismatch the intended state: Once reconciliation is over
    /// (and this field is false), there are two possible outcomes: reconciliation
    /// succeeded and the serving state matches the Service, or there was an error,
    /// and reconciliation failed. This state can be found in
    /// `terminal_condition.state`.
    ///
    /// If reconciliation succeeded, the following fields will match: `traffic` and
    /// `traffic_statuses`, `observed_generation` and `generation`,
    /// `latest_ready_revision` and `latest_created_revision`.
    ///
    /// If reconciliation failed, `traffic_statuses`, `observed_generation`, and
    /// `latest_ready_revision` will have the state of the last serving revision,
    /// or empty for newly created Services. Additional information on the failure
    /// can be found in `terminal_condition` and `conditions`.
    #[prost(bool, tag="98")]
    pub reconciling: bool,
    /// Output only. A system-generated fingerprint for this version of the
    /// resource. May be used to detect modification conflict during updates.
    #[prost(string, tag="99")]
    pub etag: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod services_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Cloud Run Service Control Plane API
    #[derive(Debug, Clone)]
    pub struct ServicesClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ServicesClient<tonic::transport::Channel> {
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
    impl<T> ServicesClient<T>
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
        ) -> ServicesClient<InterceptedService<T, F>>
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
            ServicesClient::new(InterceptedService::new(inner, interceptor))
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
        /// Creates a new Service in a given project and location.
        pub async fn create_service(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateServiceRequest>,
        ) -> Result<
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
                "/google.cloud.run.v2.Services/CreateService",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets information about a Service.
        pub async fn get_service(
            &mut self,
            request: impl tonic::IntoRequest<super::GetServiceRequest>,
        ) -> Result<tonic::Response<super::Service>, tonic::Status> {
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
                "/google.cloud.run.v2.Services/GetService",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// List Services.
        pub async fn list_services(
            &mut self,
            request: impl tonic::IntoRequest<super::ListServicesRequest>,
        ) -> Result<tonic::Response<super::ListServicesResponse>, tonic::Status> {
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
                "/google.cloud.run.v2.Services/ListServices",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Updates a Service.
        pub async fn update_service(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateServiceRequest>,
        ) -> Result<
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
                "/google.cloud.run.v2.Services/UpdateService",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes a Service.
        /// This will cause the Service to stop serving traffic and will delete all
        /// revisions.
        pub async fn delete_service(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteServiceRequest>,
        ) -> Result<
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
                "/google.cloud.run.v2.Services/DeleteService",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Get the IAM Access Control policy currently in effect for the given
        /// Cloud Run Service. This result does not include any inherited policies.
        pub async fn get_iam_policy(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::iam::v1::GetIamPolicyRequest,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::iam::v1::Policy>,
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
                "/google.cloud.run.v2.Services/GetIamPolicy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Sets the IAM Access control policy for the specified Service. Overwrites
        /// any existing policy.
        pub async fn set_iam_policy(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::iam::v1::SetIamPolicyRequest,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::iam::v1::Policy>,
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
                "/google.cloud.run.v2.Services/SetIamPolicy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Returns permissions that a caller has on the specified Project.
        ///
        /// There are no permissions required for making this API call.
        pub async fn test_iam_permissions(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::iam::v1::TestIamPermissionsRequest,
            >,
        ) -> Result<
            tonic::Response<
                super::super::super::super::iam::v1::TestIamPermissionsResponse,
            >,
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
                "/google.cloud.run.v2.Services/TestIamPermissions",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
