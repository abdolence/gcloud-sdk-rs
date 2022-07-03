/// A list of Kubernetes Namespaces
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Namespaces {
    /// A list of Kubernetes Namespaces
    #[prost(string, repeated, tag = "1")]
    pub namespaces: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// A reference to a namespaced resource in Kubernetes.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NamespacedName {
    /// The Namespace of the Kubernetes resource.
    #[prost(string, tag = "1")]
    pub namespace: ::prost::alloc::string::String,
    /// The name of the Kubernetes resource.
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
}
/// A list of namespaced Kubernetes resources.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NamespacedNames {
    /// A list of namespaced Kubernetes resources.
    #[prost(message, repeated, tag = "1")]
    pub namespaced_names: ::prost::alloc::vec::Vec<NamespacedName>,
}
/// Defined a customer managed encryption key that will be used to encrypt Backup
/// artifacts.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EncryptionKey {
    /// Google Cloud KMS encryption key. Format:
    /// projects/*/locations/*/keyRings/*/cryptoKeys/*
    #[prost(string, tag = "1")]
    pub gcp_kms_encryption_key: ::prost::alloc::string::String,
}
/// Represents a request to perform a single point-in-time capture of
/// some portion of the state of a GKE cluster, the record of the backup
/// operation itself, and an anchor for the underlying artifacts that
/// comprise the Backup (the config backup and VolumeBackups).
/// Next id: 28
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Backup {
    /// Output only. The fully qualified name of the Backup.
    /// projects/*/locations/*/backupPlans/*/backups/*
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. Server generated global unique identifier of
    /// \[UUID4\](<https://en.wikipedia.org/wiki/Universally_unique_identifier>)
    #[prost(string, tag = "2")]
    pub uid: ::prost::alloc::string::String,
    /// Output only. The timestamp when this Backup resource was created.
    #[prost(message, optional, tag = "3")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The timestamp when this Backup resource was last updated.
    #[prost(message, optional, tag = "4")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. This flag indicates whether this Backup resource was created manually
    /// by a user or via a schedule in the BackupPlan. A value of True means that
    /// the Backup was created manually.
    #[prost(bool, tag = "5")]
    pub manual: bool,
    /// A set of custom labels supplied by user.
    #[prost(map = "string, string", tag = "6")]
    pub labels:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Minimum age for this Backup (in days). If this field is set to a non-zero
    /// value, the Backup will be "locked" against deletion (either manual or
    /// automatic deletion) for the number of days provided (measured from the
    /// creation time of the Backup).  MUST be an integer value between 0-90
    /// (inclusive).
    ///
    /// Defaults to parent BackupPlan's
    /// \[backup_delete_lock_days][google.cloud.gkebackup.v1.BackupPlan.RetentionPolicy.backup_delete_lock_days\]
    /// setting and may only be increased
    /// (either at creation time or in a subsequent update).
    #[prost(int32, tag = "7")]
    pub delete_lock_days: i32,
    /// Output only. The time at which an existing delete lock will expire for this backup
    /// (calculated from create_time + \[delete_lock_days][google.cloud.gkebackup.v1.Backup.delete_lock_days\]).
    #[prost(message, optional, tag = "8")]
    pub delete_lock_expire_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The age (in days) after which this Backup will be automatically deleted.
    /// Must be an integer value >= 0:
    ///
    /// - If 0, no automatic deletion will occur for this Backup.
    /// - If not 0, this must be >= \[delete_lock_days][google.cloud.gkebackup.v1.Backup.delete_lock_days\].
    ///
    /// Once a Backup is created, this value may only be increased.
    ///
    /// Defaults to the parent BackupPlan's
    /// \[backup_retain_days][google.cloud.gkebackup.v1.BackupPlan.RetentionPolicy.backup_retain_days\] value.
    #[prost(int32, tag = "9")]
    pub retain_days: i32,
    /// Output only. The time at which this Backup will be automatically deleted (calculated
    /// from create_time + \[retain_days][google.cloud.gkebackup.v1.Backup.retain_days\]).
    #[prost(message, optional, tag = "10")]
    pub retain_expire_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The customer managed encryption key that was used to encrypt the Backup's
    /// artifacts.  Inherited from the parent BackupPlan's
    /// \[encryption_key][google.cloud.gkebackup.v1.BackupPlan.BackupConfig.encryption_key\] value.
    #[prost(message, optional, tag = "11")]
    pub encryption_key: ::core::option::Option<EncryptionKey>,
    /// Output only. Whether or not the Backup contains volume data.  Controlled by the parent
    /// BackupPlan's
    /// \[include_volume_data][google.cloud.gkebackup.v1.BackupPlan.BackupConfig.include_volume_data\] value.
    #[prost(bool, tag = "15")]
    pub contains_volume_data: bool,
    /// Output only. Whether or not the Backup contains Kubernetes Secrets.  Controlled by the
    /// parent BackupPlan's
    /// \[include_secrets][google.cloud.gkebackup.v1.BackupPlan.BackupConfig.include_secrets\] value.
    #[prost(bool, tag = "16")]
    pub contains_secrets: bool,
    /// Output only. Information about the GKE cluster from which this Backup was created.
    #[prost(message, optional, tag = "17")]
    pub cluster_metadata: ::core::option::Option<backup::ClusterMetadata>,
    /// Output only. Current state of the Backup
    #[prost(enumeration = "backup::State", tag = "18")]
    pub state: i32,
    /// Output only. Human-readable description of why the backup is in the current `state`.
    #[prost(string, tag = "19")]
    pub state_reason: ::prost::alloc::string::String,
    /// Output only. Completion time of the Backup
    #[prost(message, optional, tag = "20")]
    pub complete_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The total number of Kubernetes resources included in the Backup.
    #[prost(int32, tag = "21")]
    pub resource_count: i32,
    /// Output only. The total number of volume backups contained in the Backup.
    #[prost(int32, tag = "22")]
    pub volume_count: i32,
    /// Output only. The total size of the Backup in bytes = config backup size + sum(volume
    /// backup sizes)
    #[prost(int64, tag = "23")]
    pub size_bytes: i64,
    /// Output only. `etag` is used for optimistic concurrency control as a way to help
    /// prevent simultaneous updates of a backup from overwriting each other.
    /// It is strongly suggested that systems make use of the `etag` in the
    /// read-modify-write cycle to perform backup updates in order to avoid
    /// race conditions: An `etag` is returned in the response to `GetBackup`,
    /// and systems are expected to put that etag in the request to
    /// `UpdateBackup` or `DeleteBackup` to ensure that their change will be
    /// applied to the same version of the resource.
    #[prost(string, tag = "24")]
    pub etag: ::prost::alloc::string::String,
    /// User specified descriptive string for this Backup.
    #[prost(string, tag = "25")]
    pub description: ::prost::alloc::string::String,
    /// Output only. The total number of Kubernetes Pods contained in the Backup.
    #[prost(int32, tag = "26")]
    pub pod_count: i32,
    /// Output only. The size of the config backup in bytes.
    #[prost(int64, tag = "27")]
    pub config_backup_size_bytes: i64,
    /// Defines the "scope" of the Backup - which namespaced resources in the
    /// cluster were included in the Backup.  Inherited from the parent
    /// BackupPlan's \[backup_scope][google.cloud.gkebackup.v1.BackupPlan.BackupConfig.backup_scope\] value.
    #[prost(oneof = "backup::BackupScope", tags = "12, 13, 14")]
    pub backup_scope: ::core::option::Option<backup::BackupScope>,
}
/// Nested message and enum types in `Backup`.
pub mod backup {
    /// Information about the GKE cluster from which this Backup was created.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ClusterMetadata {
        /// The source cluster from which this Backup was created.
        /// Valid formats:
        ///
        ///   - projects/*/locations/*/clusters/*
        ///   - projects/*/zones/*/clusters/*
        ///
        /// This is inherited from the parent BackupPlan's
        /// \[cluster][google.cloud.gkebackup.v1.BackupPlan.cluster\] field.
        #[prost(string, tag = "1")]
        pub cluster: ::prost::alloc::string::String,
        /// The Kubernetes server version of the source cluster.
        #[prost(string, tag = "2")]
        pub k8s_version: ::prost::alloc::string::String,
        /// A list of the Backup for GKE CRD versions found in the cluster.
        #[prost(map = "string, string", tag = "3")]
        pub backup_crd_versions: ::std::collections::HashMap<
            ::prost::alloc::string::String,
            ::prost::alloc::string::String,
        >,
        /// Platform-specific version
        #[prost(oneof = "cluster_metadata::PlatformVersion", tags = "4, 5")]
        pub platform_version: ::core::option::Option<cluster_metadata::PlatformVersion>,
    }
    /// Nested message and enum types in `ClusterMetadata`.
    pub mod cluster_metadata {
        /// Platform-specific version
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum PlatformVersion {
            /// GKE version
            #[prost(string, tag = "4")]
            GkeVersion(::prost::alloc::string::String),
            /// Anthos version
            #[prost(string, tag = "5")]
            AnthosVersion(::prost::alloc::string::String),
        }
    }
    /// State
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// The Backup resource is in the process of being created.
        Unspecified = 0,
        /// The Backup resource has been created and the associated BackupJob
        /// Kubernetes resource has been injected into the source cluster.
        Creating = 1,
        /// The gkebackup agent in the cluster has begun executing the backup
        /// operation.
        InProgress = 2,
        /// The backup operation has completed successfully.
        Succeeded = 3,
        /// The backup operation has failed.
        Failed = 4,
        /// This Backup resource (and its associated artifacts) is in the process
        /// of being deleted.
        Deleting = 5,
    }
    /// Defines the "scope" of the Backup - which namespaced resources in the
    /// cluster were included in the Backup.  Inherited from the parent
    /// BackupPlan's \[backup_scope][google.cloud.gkebackup.v1.BackupPlan.BackupConfig.backup_scope\] value.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum BackupScope {
        /// Output only. If True, all namespaces were included in the Backup.
        #[prost(bool, tag = "12")]
        AllNamespaces(bool),
        /// Output only. If set, the list of namespaces that were included in the Backup.
        #[prost(message, tag = "13")]
        SelectedNamespaces(super::Namespaces),
        /// Output only. If set, the list of ProtectedApplications whose resources were included
        /// in the Backup.
        #[prost(message, tag = "14")]
        SelectedApplications(super::NamespacedNames),
    }
}
/// Defines the configuration and scheduling for a "line" of Backups.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BackupPlan {
    /// Output only. The full name of the BackupPlan resource.
    /// Format: projects/*/locations/*/backupPlans/*
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. Server generated global unique identifier of
    /// \[UUID\](<https://en.wikipedia.org/wiki/Universally_unique_identifier>) format.
    #[prost(string, tag = "2")]
    pub uid: ::prost::alloc::string::String,
    /// Output only. The timestamp when this BackupPlan resource was created.
    #[prost(message, optional, tag = "3")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The timestamp when this BackupPlan resource was last
    /// updated.
    #[prost(message, optional, tag = "4")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// User specified descriptive string for this BackupPlan.
    #[prost(string, tag = "5")]
    pub description: ::prost::alloc::string::String,
    /// Required. Immutable. The source cluster from which Backups will be created via
    /// this BackupPlan.
    /// Valid formats:
    ///
    /// - projects/*/locations/*/clusters/*
    /// - projects/*/zones/*/clusters/*
    #[prost(string, tag = "6")]
    pub cluster: ::prost::alloc::string::String,
    /// RetentionPolicy governs lifecycle of Backups created under this plan.
    #[prost(message, optional, tag = "7")]
    pub retention_policy: ::core::option::Option<backup_plan::RetentionPolicy>,
    /// A set of custom labels supplied by user.
    #[prost(map = "string, string", tag = "8")]
    pub labels:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Defines a schedule for automatic Backup creation via this BackupPlan.
    #[prost(message, optional, tag = "9")]
    pub backup_schedule: ::core::option::Option<backup_plan::Schedule>,
    /// Output only. `etag` is used for optimistic concurrency control as a way to help
    /// prevent simultaneous updates of a backup plan from overwriting each other.
    /// It is strongly suggested that systems make use of the 'etag' in the
    /// read-modify-write cycle to perform BackupPlan updates in order to avoid
    /// race conditions: An `etag` is returned in the response to `GetBackupPlan`,
    /// and systems are expected to put that etag in the request to
    /// `UpdateBackupPlan` or `DeleteBackupPlan` to ensure that their change
    /// will be applied to the same version of the resource.
    #[prost(string, tag = "10")]
    pub etag: ::prost::alloc::string::String,
    /// This flag indicates whether this BackupPlan has been deactivated.
    /// Setting this field to True locks the BackupPlan such that no further
    /// updates will be allowed (except deletes), including the deactivated field
    /// itself. It also prevents any new Backups from being created via this
    /// BackupPlan (including scheduled Backups).
    ///
    /// Default: False
    #[prost(bool, tag = "11")]
    pub deactivated: bool,
    /// Defines the configuration of Backups created via this BackupPlan.
    #[prost(message, optional, tag = "12")]
    pub backup_config: ::core::option::Option<backup_plan::BackupConfig>,
    /// Output only. The number of Kubernetes Pods backed up in the
    /// last successful Backup created via this BackupPlan.
    #[prost(int32, tag = "13")]
    pub protected_pod_count: i32,
}
/// Nested message and enum types in `BackupPlan`.
pub mod backup_plan {
    /// RetentionPolicy defines a Backup retention policy for a BackupPlan.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct RetentionPolicy {
        /// Minimum age for Backups created via this BackupPlan (in days).
        /// This field MUST be an integer value between 0-90 (inclusive).
        /// A Backup created under this BackupPlan will NOT be deletable until it
        /// reaches Backup's (create_time + backup_delete_lock_days).
        /// Updating this field of a BackupPlan does NOT affect existing Backups
        /// under it. Backups created AFTER a successful update will inherit
        /// the new value.
        ///
        /// Default: 0 (no delete blocking)
        #[prost(int32, tag = "1")]
        pub backup_delete_lock_days: i32,
        /// The default maximum age of a Backup created via this BackupPlan.
        /// This field MUST be an integer value >= 0.
        /// If specified, a Backup created under this BackupPlan will be
        /// automatically deleted after its age reaches (create_time +
        /// backup_retain_days).
        /// If not specified, Backups created under this BackupPlan will NOT be
        /// subject to automatic deletion.
        /// Updating this field does NOT affect existing Backups under it. Backups
        /// created AFTER a successful update will automatically pick up the new
        /// value.
        /// NOTE: backup_retain_days must be >= \[backup_delete_lock_days][google.cloud.gkebackup.v1.BackupPlan.RetentionPolicy.backup_delete_lock_days\].
        ///
        /// Default: 0 (no automatic deletion)
        #[prost(int32, tag = "2")]
        pub backup_retain_days: i32,
        /// This flag denotes whether the retention policy of this BackupPlan is
        /// locked.  If set to True, no further update is allowed on this policy,
        /// including the `locked` field itself.
        ///
        /// Default: False
        #[prost(bool, tag = "3")]
        pub locked: bool,
    }
    /// Schedule defines scheduling parameters for automatically creating Backups
    /// via this BackupPlan.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Schedule {
        /// A standard \[cron\](<https://wikipedia.com/wiki/cron>) string that defines a
        /// repeating schedule for creating Backups via this BackupPlan.
        ///
        /// Default (empty): no automatic backup creation will occur.
        #[prost(string, tag = "1")]
        pub cron_schedule: ::prost::alloc::string::String,
        /// This flag denotes whether automatic Backup creation is paused for this
        /// BackupPlan.
        ///
        /// Default: False
        #[prost(bool, tag = "2")]
        pub paused: bool,
    }
    /// BackupConfig defines the configuration of Backups created via this
    /// BackupPlan.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct BackupConfig {
        /// This flag specifies whether volume data should be backed up when
        /// PVCs are included in the scope of a Backup.
        ///
        /// Default: False
        #[prost(bool, tag = "4")]
        pub include_volume_data: bool,
        /// This flag specifies whether Kubernetes Secret resources should be
        /// included when they fall into the scope of Backups.
        ///
        /// Default: False
        #[prost(bool, tag = "5")]
        pub include_secrets: bool,
        /// This defines a customer managed encryption key that will be used to
        /// encrypt the "config" portion (the Kubernetes resources) of Backups
        /// created via this plan.
        ///
        /// Default (empty): Config backup artifacts will not be encrypted.
        #[prost(message, optional, tag = "6")]
        pub encryption_key: ::core::option::Option<super::EncryptionKey>,
        /// This defines the "scope" of the Backup - which namespaced
        /// resources in the cluster will be included in a Backup.
        /// Exactly one of the fields of backup_scope MUST be specified.
        #[prost(oneof = "backup_config::BackupScope", tags = "1, 2, 3")]
        pub backup_scope: ::core::option::Option<backup_config::BackupScope>,
    }
    /// Nested message and enum types in `BackupConfig`.
    pub mod backup_config {
        /// This defines the "scope" of the Backup - which namespaced
        /// resources in the cluster will be included in a Backup.
        /// Exactly one of the fields of backup_scope MUST be specified.
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum BackupScope {
            /// If True, include all namespaced resources
            #[prost(bool, tag = "1")]
            AllNamespaces(bool),
            /// If set, include just the resources in the listed namespaces.
            #[prost(message, tag = "2")]
            SelectedNamespaces(super::super::Namespaces),
            /// If set, include just the resources referenced by the listed
            /// ProtectedApplications.
            #[prost(message, tag = "3")]
            SelectedApplications(super::super::NamespacedNames),
        }
    }
}
/// Represents both a request to Restore some portion of a Backup into
/// a target GKE cluster and a record of the restore operation itself.
/// Next id: 18
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Restore {
    /// Output only. The full name of the Restore resource.
    /// Format: projects/*/locations/*/restorePlans/*/restores/*
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. Server generated global unique identifier of
    /// \[UUID\](<https://en.wikipedia.org/wiki/Universally_unique_identifier>) format.
    #[prost(string, tag = "2")]
    pub uid: ::prost::alloc::string::String,
    /// Output only. The timestamp when this Restore resource was created.
    #[prost(message, optional, tag = "3")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The timestamp when this Restore resource was last
    /// updated.
    #[prost(message, optional, tag = "4")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// User specified descriptive string for this Restore.
    #[prost(string, tag = "5")]
    pub description: ::prost::alloc::string::String,
    /// Required. Immutable. A reference to the \[Backup][google.cloud.gkebackup.v1.Backup\] used as the source from which this Restore
    /// will restore. Note that this Backup must be a sub-resource of the
    /// RestorePlan's \[backup_plan][google.cloud.gkebackup.v1.RestorePlan.backup_plan\].
    /// Format: projects/*/locations/*/backupPlans/*/backups/*.
    #[prost(string, tag = "6")]
    pub backup: ::prost::alloc::string::String,
    /// Output only. The target cluster into which this Restore will restore data.
    /// Valid formats:
    ///
    ///   - projects/*/locations/*/clusters/*
    ///   - projects/*/zones/*/clusters/*
    ///
    /// Inherited from parent RestorePlan's \[cluster][google.cloud.gkebackup.v1.RestorePlan.cluster\] value.
    #[prost(string, tag = "7")]
    pub cluster: ::prost::alloc::string::String,
    /// Output only. Configuration of the Restore.  Inherited from parent RestorePlan's
    /// \[restore_config][google.cloud.gkebackup.v1.RestorePlan.restore_config\].
    #[prost(message, optional, tag = "8")]
    pub restore_config: ::core::option::Option<RestoreConfig>,
    /// A set of custom labels supplied by user.
    #[prost(map = "string, string", tag = "9")]
    pub labels:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Output only. The current state of the Restore.
    #[prost(enumeration = "restore::State", tag = "10")]
    pub state: i32,
    /// Output only. Human-readable description of why the Restore is in its current state.
    #[prost(string, tag = "11")]
    pub state_reason: ::prost::alloc::string::String,
    /// Output only. Timestamp of when the restore operation completed.
    #[prost(message, optional, tag = "12")]
    pub complete_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Number of resources restored during the restore execution.
    #[prost(int32, tag = "13")]
    pub resources_restored_count: i32,
    /// Output only. Number of resources excluded during the restore execution.
    #[prost(int32, tag = "14")]
    pub resources_excluded_count: i32,
    /// Output only. Number of resources that failed to be restored during the restore
    /// execution.
    #[prost(int32, tag = "15")]
    pub resources_failed_count: i32,
    /// Output only. Number of volumes restored during the restore execution.
    #[prost(int32, tag = "16")]
    pub volumes_restored_count: i32,
    /// Output only. `etag` is used for optimistic concurrency control as a way to help
    /// prevent simultaneous updates of a restore from overwriting each other.
    /// It is strongly suggested that systems make use of the `etag` in the
    /// read-modify-write cycle to perform restore updates in order to avoid
    /// race conditions: An `etag` is returned in the response to `GetRestore`,
    /// and systems are expected to put that etag in the request to
    /// `UpdateRestore` or `DeleteRestore` to ensure that their change will be
    /// applied to the same version of the resource.
    #[prost(string, tag = "17")]
    pub etag: ::prost::alloc::string::String,
}
/// Nested message and enum types in `Restore`.
pub mod restore {
    /// Possible values for state of the Restore.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// The Restore resource is in the process of being created.
        Unspecified = 0,
        /// The Restore resource has been created and the associated RestoreJob
        /// Kubernetes resource has been injected into target cluster.
        Creating = 1,
        /// The gkebackup agent in the cluster has begun executing the restore
        /// operation.
        InProgress = 2,
        /// The restore operation has completed successfully. Restored workloads may
        /// not yet be operational.
        Succeeded = 3,
        /// The restore operation has failed.
        Failed = 4,
        /// This Restore resource is in the process of being deleted.
        Deleting = 5,
    }
}
/// Configuration of a restore.
/// Next id: 9
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RestoreConfig {
    /// Specifies the mechanism to be used to restore volume data.
    /// Default: VOLUME_DATA_RESTORE_POLICY_UNSPECIFIED (will be treated as
    /// NO_VOLUME_DATA_RESTORATION).
    #[prost(enumeration = "restore_config::VolumeDataRestorePolicy", tag = "1")]
    pub volume_data_restore_policy: i32,
    /// Defines the behavior for handling the situation where cluster-scoped
    /// resources being restored already exist in the target cluster. This MUST be
    /// set to a value other than CLUSTER_RESOURCE_CONFLICT_POLICY_UNSPECIFIED if
    /// \[cluster_resource_restore_scope][google.cloud.gkebackup.v1.RestoreConfig.cluster_resource_restore_scope\] is not empty.
    #[prost(
        enumeration = "restore_config::ClusterResourceConflictPolicy",
        tag = "2"
    )]
    pub cluster_resource_conflict_policy: i32,
    /// Defines the behavior for handling the situation where sets of namespaced
    /// resources being restored already exist in the target cluster. This MUST be
    /// set to a value other than NAMESPACED_RESOURCE_RESTORE_MODE_UNSPECIFIED.
    #[prost(
        enumeration = "restore_config::NamespacedResourceRestoreMode",
        tag = "3"
    )]
    pub namespaced_resource_restore_mode: i32,
    /// Identifies the cluster-scoped resources to restore from the Backup.
    /// Not specifying it means NO cluster resource will be restored.
    #[prost(message, optional, tag = "4")]
    pub cluster_resource_restore_scope:
        ::core::option::Option<restore_config::ClusterResourceRestoreScope>,
    /// A list of transformation rules to be applied against Kubernetes resources
    /// as they are selected for restoration from a Backup. Rules are executed in
    /// order defined - this order matters, as changes made by a rule may impact
    /// the filtering logic of subsequent rules. An empty list means no
    /// substitution will occur.
    #[prost(message, repeated, tag = "8")]
    pub substitution_rules: ::prost::alloc::vec::Vec<restore_config::SubstitutionRule>,
    /// Specifies the namespaced resources to restore from the Backup.
    /// Only one of the entries may be specified. If not specified, NO namespaced
    /// resources will be restored.
    #[prost(
        oneof = "restore_config::NamespacedResourceRestoreScope",
        tags = "5, 6, 7"
    )]
    pub namespaced_resource_restore_scope:
        ::core::option::Option<restore_config::NamespacedResourceRestoreScope>,
}
/// Nested message and enum types in `RestoreConfig`.
pub mod restore_config {
    /// This is a direct map to the Kubernetes GroupKind type
    /// \[GroupKind\](<https://godoc.org/k8s.io/apimachinery/pkg/runtime/schema#GroupKind>)
    /// and is used for identifying specific "types" of resources to restore.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GroupKind {
        /// API group string of a Kubernetes resource, e.g.
        /// "apiextensions.k8s.io", "storage.k8s.io", etc.
        /// Note: use empty string for core API group
        #[prost(string, tag = "1")]
        pub resource_group: ::prost::alloc::string::String,
        /// Kind of a Kubernetes resource, e.g.
        /// "CustomResourceDefinition", "StorageClass", etc.
        #[prost(string, tag = "2")]
        pub resource_kind: ::prost::alloc::string::String,
    }
    /// Identifies the cluster-scoped resources to restore from the Backup.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ClusterResourceRestoreScope {
        /// A list of "types" of cluster-scoped resources to be restored from the
        /// Backup.  An empty list means that NO cluster-scoped resources will be
        /// restored. Note that Namespaces and PersistentVolume restoration is
        /// handled separately and is not governed by this field.
        #[prost(message, repeated, tag = "1")]
        pub selected_group_kinds: ::prost::alloc::vec::Vec<GroupKind>,
    }
    /// A transformation rule to be applied against Kubernetes resources as they
    /// are selected for restoration from a Backup. A rule contains both filtering
    /// logic (which resources are subject to substitution) and substitution logic.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SubstitutionRule {
        /// (Filtering parameter) Any resource subject to substitution must be
        /// contained within one of the listed Kubernetes Namespace in the Backup.
        /// If this field is not provided, no namespace filtering will be performed
        /// (all resources in all Namespaces, including all cluster-scoped resources,
        /// will be candidates for substitution).
        /// To mix cluster-scoped and namespaced resources in the same rule, use an
        /// empty string ("") as one of the target namespaces.
        #[prost(string, repeated, tag = "1")]
        pub target_namespaces: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// (Filtering parameter) Any resource subject to substitution must belong to
        /// one of the listed "types".
        /// If this field is not provided, no type filtering will be performed (all
        /// resources of all types matching previous filtering parameters will be
        /// candidates for substitution).
        #[prost(message, repeated, tag = "2")]
        pub target_group_kinds: ::prost::alloc::vec::Vec<GroupKind>,
        /// Required. This is a \[JSONPath\]
        /// (<https://kubernetes.io/docs/reference/kubectl/jsonpath/>)
        /// expression that matches specific fields of candidate
        /// resources and it operates as both a filtering parameter (resources that
        /// are not matched with this expression will not be candidates for
        /// substitution) as well as a field identifier (identifies exactly which
        /// fields out of the candidate resources will be modified).
        #[prost(string, tag = "3")]
        pub target_json_path: ::prost::alloc::string::String,
        /// (Filtering parameter) This is a [regular expression]
        /// (<https://en.wikipedia.org/wiki/Regular_expression>)
        /// that is compared against the fields matched by the target_json_path
        /// expression (and must also have passed the previous filters).
        /// Substitution will not be performed against fields whose
        /// value does not match this expression. If this field is NOT specified,
        /// then ALL fields matched by the target_json_path expression will undergo
        /// substitution. Note that an empty (e.g., "", rather than unspecified)
        /// value for for this field will only match empty fields.
        #[prost(string, tag = "4")]
        pub original_value_pattern: ::prost::alloc::string::String,
        /// This is the new value to set for any fields that pass the filtering and
        /// selection criteria. To remove a value from a Kubernetes resource, either
        /// leave this field unspecified, or set it to the empty string ("").
        #[prost(string, tag = "5")]
        pub new_value: ::prost::alloc::string::String,
    }
    /// Defines how volume data should be restored
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum VolumeDataRestorePolicy {
        /// Unspecified (illegal).
        Unspecified = 0,
        /// For each PVC to be restored, will create a new underlying volume (and PV)
        /// from the corresponding VolumeBackup contained within the Backup.
        RestoreVolumeDataFromBackup = 1,
        /// For each PVC to be restored, attempt to reuse the original PV contained
        /// in the Backup (with its original underlying volume).  Note that option
        /// is likely only usable when restoring a workload to its original cluster.
        ReuseVolumeHandleFromBackup = 2,
        /// For each PVC to be restored, PVCs will be created without any particular
        /// action to restore data.  In this case, the normal Kubernetes provisioning
        /// logic would kick in, and this would likely result in either dynamically
        /// provisioning blank PVs or binding to statically provisioned PVs.
        NoVolumeDataRestoration = 3,
    }
    /// Defines the behavior for handling the situation where cluster-scoped
    /// resources being restored already exist in the target cluster.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ClusterResourceConflictPolicy {
        /// Unspecified. Only allowed if no cluster-scoped resources will be
        /// restored.
        Unspecified = 0,
        /// Do not attempt to restore the conflicting resource.
        UseExistingVersion = 1,
        /// Delete the existing version before re-creating it from the Backup.
        /// Note that this is a dangerous option which could cause unintentional
        /// data loss if used inappropriately - for example, deleting a CRD will
        /// cause Kubernetes to delete all CRs of that type.
        UseBackupVersion = 2,
    }
    /// Defines the behavior for handling the situation where sets of namespaced
    /// resources being restored already exist in the target cluster.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum NamespacedResourceRestoreMode {
        /// Unspecified (invalid).
        Unspecified = 0,
        /// When conflicting top-level resources (either Namespaces or
        /// ProtectedApplications, depending upon the scope) are encountered, this
        /// will first trigger a delete of the conflicting resource AND ALL OF ITS
        /// REFERENCED RESOURCES (e.g., all resources in the Namespace or all
        /// resources referenced by the ProtectedApplication) before restoring the
        /// resources from the Backup. This mode should only be used when you are
        /// intending to revert some portion of a cluster to an earlier state.
        DeleteAndRestore = 1,
        /// If conflicting top-level resources (either Namespaces or
        /// ProtectedApplications, depending upon the scope) are encountered at the
        /// beginning of a restore process, the Restore will fail.  If a conflict
        /// occurs during the restore process itself (e.g., because an out of band
        /// process creates conflicting resources), a conflict will be reported.
        FailOnConflict = 2,
    }
    /// Specifies the namespaced resources to restore from the Backup.
    /// Only one of the entries may be specified. If not specified, NO namespaced
    /// resources will be restored.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum NamespacedResourceRestoreScope {
        /// Restore all namespaced resources in the Backup if set to "True".
        /// Specifying this field to "False" is an error.
        #[prost(bool, tag = "5")]
        AllNamespaces(bool),
        /// A list of selected Namespaces to restore from the Backup. The listed
        /// Namespaces and all resources contained in them will be restored.
        #[prost(message, tag = "6")]
        SelectedNamespaces(super::Namespaces),
        /// A list of selected ProtectedApplications to restore. The listed
        /// ProtectedApplications and all the resources to which they refer will be
        /// restored.
        #[prost(message, tag = "7")]
        SelectedApplications(super::NamespacedNames),
    }
}
/// The configuration of a potential series of Restore operations to be performed
/// against Backups belong to a particular BackupPlan.
/// Next id: 11
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RestorePlan {
    /// Output only. The full name of the RestorePlan resource.
    /// Format: projects/*/locations/*/restorePlans/*.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. Server generated global unique identifier of
    /// \[UUID\](<https://en.wikipedia.org/wiki/Universally_unique_identifier>) format.
    #[prost(string, tag = "2")]
    pub uid: ::prost::alloc::string::String,
    /// Output only. The timestamp when this RestorePlan resource was
    /// created.
    #[prost(message, optional, tag = "3")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The timestamp when this RestorePlan resource was last
    /// updated.
    #[prost(message, optional, tag = "4")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// User specified descriptive string for this RestorePlan.
    #[prost(string, tag = "5")]
    pub description: ::prost::alloc::string::String,
    /// Required. Immutable. A reference to the \[BackupPlan][google.cloud.gkebackup.v1.BackupPlan\] from which Backups may be used as the
    /// source for Restores created via this RestorePlan.
    /// Format: projects/*/locations/*/backupPlans/*.
    #[prost(string, tag = "6")]
    pub backup_plan: ::prost::alloc::string::String,
    /// Required. Immutable. The target cluster into which Restores created via this RestorePlan
    /// will restore data. NOTE: the cluster's region must be the same as the
    /// RestorePlan.
    /// Valid formats:
    ///
    ///   - projects/*/locations/*/clusters/*
    ///   - projects/*/zones/*/clusters/*
    #[prost(string, tag = "7")]
    pub cluster: ::prost::alloc::string::String,
    /// Required. Configuration of Restores created via this RestorePlan.
    #[prost(message, optional, tag = "8")]
    pub restore_config: ::core::option::Option<RestoreConfig>,
    /// A set of custom labels supplied by user.
    #[prost(map = "string, string", tag = "9")]
    pub labels:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Output only. `etag` is used for optimistic concurrency control as a way to help
    /// prevent simultaneous updates of a restore from overwriting each other.
    /// It is strongly suggested that systems make use of the `etag` in the
    /// read-modify-write cycle to perform restore updates in order to avoid
    /// race conditions: An `etag` is returned in the response to `GetRestorePlan`,
    /// and systems are expected to put that etag in the request to
    /// `UpdateRestorePlan` or `DeleteRestorePlan` to ensure that their change
    /// will be applied to the same version of the resource.
    #[prost(string, tag = "10")]
    pub etag: ::prost::alloc::string::String,
}
/// Represents the backup of a specific persistent volume as a component of a
/// Backup - both the record of the operation and a pointer to the underlying
/// storage-specific artifacts.
/// Next id: 14
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VolumeBackup {
    /// Output only. The full name of the VolumeBackup resource.
    /// Format: projects/*/locations/*/backupPlans/*/backups/*/volumeBackups/*.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. Server generated global unique identifier of
    /// \[UUID\](<https://en.wikipedia.org/wiki/Universally_unique_identifier>) format.
    #[prost(string, tag = "2")]
    pub uid: ::prost::alloc::string::String,
    /// Output only. The timestamp when this VolumeBackup resource was
    /// created.
    #[prost(message, optional, tag = "3")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The timestamp when this VolumeBackup resource was last
    /// updated.
    #[prost(message, optional, tag = "4")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. A reference to the source Kubernetes PVC from which this VolumeBackup
    /// was created.
    #[prost(message, optional, tag = "5")]
    pub source_pvc: ::core::option::Option<NamespacedName>,
    /// Output only. A storage system-specific opaque handle to the underlying volume backup.
    #[prost(string, tag = "6")]
    pub volume_backup_handle: ::prost::alloc::string::String,
    /// Output only. The format used for the volume backup.
    #[prost(enumeration = "volume_backup::VolumeBackupFormat", tag = "7")]
    pub format: i32,
    /// Output only. The aggregate size of the underlying artifacts associated with this
    /// VolumeBackup in the backup storage. This may change over time when
    /// multiple backups of the same volume share the same backup storage
    /// location. In particular, this is likely to increase in size when
    /// the immediately preceding backup of the same volume is deleted.
    #[prost(int64, tag = "8")]
    pub storage_bytes: i64,
    /// Output only. The minimum size of the disk to which this VolumeBackup can be restored.
    #[prost(int64, tag = "9")]
    pub disk_size_bytes: i64,
    /// Output only. The timestamp when the associated underlying volume backup
    /// operation completed.
    #[prost(message, optional, tag = "10")]
    pub complete_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The current state of this VolumeBackup.
    #[prost(enumeration = "volume_backup::State", tag = "11")]
    pub state: i32,
    /// Output only. A human readable message explaining why the VolumeBackup is in its current
    /// state.
    #[prost(string, tag = "12")]
    pub state_message: ::prost::alloc::string::String,
    /// Output only. `etag` is used for optimistic concurrency control as a way to help
    /// prevent simultaneous updates of a volume backup from overwriting each
    /// other. It is strongly suggested that systems make use of the `etag` in the
    /// read-modify-write cycle to perform volume backup updates in order to avoid
    /// race conditions.
    #[prost(string, tag = "13")]
    pub etag: ::prost::alloc::string::String,
}
/// Nested message and enum types in `VolumeBackup`.
pub mod volume_backup {
    /// Identifies the format used for the volume backup.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum VolumeBackupFormat {
        /// Default value, not specified.
        Unspecified = 0,
        /// Compute Engine Persistent Disk snapshot based volume backup.
        GcePersistentDisk = 1,
    }
    /// The current state of a VolumeBackup
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// This is an illegal state and should not be encountered.
        Unspecified = 0,
        /// A volume for the backup was identified and backup process is about to
        /// start.
        Creating = 1,
        /// The volume backup operation has begun and is in the initial "snapshot"
        /// phase of the process. Any defined ProtectedApplication "pre" hooks will
        /// be executed before entering this state and "post" hooks will be executed
        /// upon leaving this state.
        Snapshotting = 2,
        /// The snapshot phase of the volume backup operation has completed and
        /// the snapshot is now being uploaded to backup storage.
        Uploading = 3,
        /// The volume backup operation has completed successfully.
        Succeeded = 4,
        /// The volume backup operation has failed.
        Failed = 5,
        /// This VolumeBackup resource (and its associated artifacts) is in the
        /// process of being deleted.
        Deleting = 6,
    }
}
/// Represents the operation of restoring a volume from a VolumeBackup.
/// Next id: 13
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VolumeRestore {
    /// Output only. Full name of the VolumeRestore resource.
    /// Format: projects/*/locations/*/restorePlans/*/restores/*/volumeRestores/*.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. Server generated global unique identifier of
    /// \[UUID\](<https://en.wikipedia.org/wiki/Universally_unique_identifier>) format.
    #[prost(string, tag = "2")]
    pub uid: ::prost::alloc::string::String,
    /// Output only. The timestamp when this VolumeRestore resource was
    /// created.
    #[prost(message, optional, tag = "3")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The timestamp when this VolumeRestore resource was last
    /// updated.
    #[prost(message, optional, tag = "4")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The full name of the VolumeBackup from which the volume will be restored.
    /// Format: projects/*/locations/*/backupPlans/*/backups/*/volumeBackups/*.
    #[prost(string, tag = "5")]
    pub volume_backup: ::prost::alloc::string::String,
    /// Output only. The reference to the target Kubernetes PVC to be restored.
    #[prost(message, optional, tag = "6")]
    pub target_pvc: ::core::option::Option<NamespacedName>,
    /// Output only. A storage system-specific opaque handler to the underlying volume created
    /// for the target PVC from the volume backup.
    #[prost(string, tag = "7")]
    pub volume_handle: ::prost::alloc::string::String,
    /// Output only. The type of volume provisioned
    #[prost(enumeration = "volume_restore::VolumeType", tag = "8")]
    pub volume_type: i32,
    /// Output only. The timestamp when the associated underlying volume
    /// restoration completed.
    #[prost(message, optional, tag = "9")]
    pub complete_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The current state of this VolumeRestore.
    #[prost(enumeration = "volume_restore::State", tag = "10")]
    pub state: i32,
    /// Output only. A human readable message explaining why the VolumeRestore is in its
    /// current state.
    #[prost(string, tag = "11")]
    pub state_message: ::prost::alloc::string::String,
    /// Output only. `etag` is used for optimistic concurrency control as a way to help
    /// prevent simultaneous updates of a volume restore from overwriting each
    /// other. It is strongly suggested that systems make use of the `etag` in the
    /// read-modify-write cycle to perform volume restore updates in order to avoid
    /// race conditions.
    #[prost(string, tag = "12")]
    pub etag: ::prost::alloc::string::String,
}
/// Nested message and enum types in `VolumeRestore`.
pub mod volume_restore {
    /// Supported volume types.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum VolumeType {
        /// Default
        Unspecified = 0,
        /// Compute Engine Persistent Disk volume
        GcePersistentDisk = 1,
    }
    /// The current state of a VolumeRestore
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// This is an illegal state and should not be encountered.
        Unspecified = 0,
        /// A volume for the restore was identified and restore process is about to
        /// start.
        Creating = 1,
        /// The volume is currently being restored.
        Restoring = 2,
        /// The volume has been successfully restored.
        Succeeded = 3,
        /// The volume restoration process failed.
        Failed = 4,
        /// This VolumeRestore resource is in the process of being deleted.
        Deleting = 5,
    }
}
/// Represents the metadata of the long-running operation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationMetadata {
    /// Output only. The time the operation was created.
    #[prost(message, optional, tag = "1")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The time the operation finished running.
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
    /// of the operation. Operations that have successfully been cancelled
    /// have \[Operation.error][\] value with a \[google.rpc.Status.code][google.rpc.Status.code\] of 1,
    /// corresponding to `Code.CANCELLED`.
    #[prost(bool, tag = "6")]
    pub requested_cancellation: bool,
    /// Output only. API version used to start the operation.
    #[prost(string, tag = "7")]
    pub api_version: ::prost::alloc::string::String,
}
/// Request message for CreateBackupPlan.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateBackupPlanRequest {
    /// Required. The location within which to create the BackupPlan.
    /// Format: projects/*/locations/*
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The BackupPlan resource object to create.
    #[prost(message, optional, tag = "2")]
    pub backup_plan: ::core::option::Option<BackupPlan>,
    /// Required. The client-provided short name for the BackupPlan resource.
    /// This name must:
    ///
    /// - be between 1 and 63 characters long (inclusive)
    /// - consist of only lower-case ASCII letters, numbers, and dashes
    /// - start with a lower-case letter
    /// - end with a lower-case letter or number
    /// - be unique within the set of BackupPlans in this location
    #[prost(string, tag = "3")]
    pub backup_plan_id: ::prost::alloc::string::String,
}
/// Request message for ListBackupPlans.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListBackupPlansRequest {
    /// Required. The location that contains the BackupPlans to list.
    /// Format: projects/*/locations/*
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The target number of results to return in a single response.
    /// If not specified, a default value will be chosen by the service.
    /// Note that the response may inclue a partial list and a caller should
    /// only rely on the response's
    /// \[next_page_token][google.cloud.gkebackup.v1.ListBackupPlansResponse.next_page_token\]
    /// to determine if there are more instances left to be queried.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The value of
    /// \[next_page_token][google.cloud.gkebackup.v1.ListBackupPlansResponse.next_page_token\]
    /// received from a previous `ListBackupPlans` call.
    /// Provide this to retrieve the subsequent page in a multi-page list of
    /// results. When paginating, all other parameters provided to
    /// `ListBackupPlans` must match the call that provided the page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Field match expression used to filter the results.
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Field by which to sort the results.
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
/// Response message for ListBackupPlans.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListBackupPlansResponse {
    /// The list of BackupPlans matching the given criteria.
    #[prost(message, repeated, tag = "1")]
    pub backup_plans: ::prost::alloc::vec::Vec<BackupPlan>,
    /// A token which may be sent as
    /// \[page_token][google.cloud.gkebackup.v1.ListBackupPlansRequest.page_token\] in a subsequent
    /// `ListBackupPlans` call to retrieve the next page of results.
    /// If this field is omitted or empty, then there are no more results to
    /// return.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Request message for GetBackupPlan.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBackupPlanRequest {
    /// Required. Fully qualified BackupPlan name.
    /// Format: projects/*/locations/*/backupPlans/*
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for UpdateBackupPlan.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateBackupPlanRequest {
    /// Required. A new version of the BackupPlan resource that contains updated fields.
    /// This may be sparsely populated if an `update_mask` is provided.
    #[prost(message, optional, tag = "1")]
    pub backup_plan: ::core::option::Option<BackupPlan>,
    /// This is used to specify the fields to be overwritten in the
    /// BackupPlan targeted for update. The values for each of these
    /// updated fields will be taken from the `backup_plan` provided
    /// with this request. Field names are relative to the root of the resource
    /// (e.g., `description`, `backup_config.include_volume_data`, etc.)
    /// If no `update_mask` is provided, all fields in `backup_plan` will be
    /// written to the target BackupPlan resource.
    /// Note that OUTPUT_ONLY and IMMUTABLE fields in `backup_plan` are ignored
    /// and are not used to update the target BackupPlan.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for DeleteBackupPlan.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteBackupPlanRequest {
    /// Required. Fully qualified BackupPlan name.
    /// Format: projects/*/locations/*/backupPlans/*
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// If provided, this value must match the current value of the
    /// target BackupPlan's \[etag][google.cloud.gkebackup.v1.BackupPlan.etag\] field or the request is
    /// rejected.
    #[prost(string, tag = "2")]
    pub etag: ::prost::alloc::string::String,
}
/// Request message for CreateBackup.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateBackupRequest {
    /// Required. The BackupPlan within which to create the Backup.
    /// Format: projects/*/locations/*/backupPlans/*
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The Backup resource to create.
    #[prost(message, optional, tag = "2")]
    pub backup: ::core::option::Option<Backup>,
    /// The client-provided short name for the Backup resource.
    /// This name must:
    ///
    ///  - be between 1 and 63 characters long (inclusive)
    ///  - consist of only lower-case ASCII letters, numbers, and dashes
    ///  - start with a lower-case letter
    ///  - end with a lower-case letter or number
    ///  - be unique within the set of Backups in this BackupPlan
    #[prost(string, tag = "3")]
    pub backup_id: ::prost::alloc::string::String,
}
/// Request message for ListBackups.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListBackupsRequest {
    /// Required. The BackupPlan that contains the Backups to list.
    /// Format: projects/*/locations/*/backupPlans/*
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The target number of results to return in a single response.
    /// If not specified, a default value will be chosen by the service.
    /// Note that the response may inclue a partial list and a caller should
    /// only rely on the response's
    /// \[next_page_token][google.cloud.gkebackup.v1.ListBackupsResponse.next_page_token\]
    /// to determine if there are more instances left to be queried.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The value of
    /// \[next_page_token][google.cloud.gkebackup.v1.ListBackupsResponse.next_page_token\]
    /// received from a previous `ListBackups` call.
    /// Provide this to retrieve the subsequent page in a multi-page list of
    /// results. When paginating, all other parameters provided to
    /// `ListBackups` must match the call that provided the page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Field match expression used to filter the results.
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Field by which to sort the results.
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
/// Response message for ListBackups.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListBackupsResponse {
    /// The list of Backups matching the given criteria.
    #[prost(message, repeated, tag = "1")]
    pub backups: ::prost::alloc::vec::Vec<Backup>,
    /// A token which may be sent as \[page_token][google.cloud.gkebackup.v1.ListBackupsRequest.page_token\] in
    /// a subsequent `ListBackups` call to retrieve the next page of results. If
    /// this field is omitted or empty, then there are no more results to return.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for GetBackup.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBackupRequest {
    /// Required. Full name of the Backup resource.
    /// Format: projects/*/locations/*/backupPlans/*/backups/*
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for UpdateBackup.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateBackupRequest {
    /// Required. A new version of the Backup resource that contains updated fields.
    /// This may be sparsely populated if an `update_mask` is provided.
    #[prost(message, optional, tag = "1")]
    pub backup: ::core::option::Option<Backup>,
    /// This is used to specify the fields to be overwritten in the
    /// Backup targeted for update. The values for each of these
    /// updated fields will be taken from the `backup_plan` provided
    /// with this request. Field names are relative to the root of the resource.
    /// If no `update_mask` is provided, all fields in `backup` will be
    /// written to the target Backup resource.
    /// Note that OUTPUT_ONLY and IMMUTABLE fields in `backup` are ignored
    /// and are not used to update the target Backup.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for DeleteBackup.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteBackupRequest {
    /// Required. Name of the Backup resource.
    /// Format: projects/*/locations/*/backupPlans/*/backups/*
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// If provided, this value must match the current value of the
    /// target Backup's \[etag][google.cloud.gkebackup.v1.Backup.etag\] field or the request is
    /// rejected.
    #[prost(string, tag = "2")]
    pub etag: ::prost::alloc::string::String,
    /// If set to true, any VolumeBackups below this Backup will also be deleted.
    /// Otherwise, the request will only succeed if the Backup has no
    /// VolumeBackups.
    #[prost(bool, tag = "3")]
    pub force: bool,
}
/// Request message for ListVolumeBackups.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListVolumeBackupsRequest {
    /// Required. The Backup that contains the VolumeBackups to list.
    /// Format: projects/*/locations/*/backupPlans/*/backups/*
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The target number of results to return in a single response.
    /// If not specified, a default value will be chosen by the service.
    /// Note that the response may inclue a partial list and a caller should
    /// only rely on the response's
    /// \[next_page_token][google.cloud.gkebackup.v1.ListVolumeBackupsResponse.next_page_token\]
    /// to determine if there are more instances left to be queried.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The value of
    /// \[next_page_token][google.cloud.gkebackup.v1.ListVolumeBackupsResponse.next_page_token\]
    /// received from a previous `ListVolumeBackups` call.
    /// Provide this to retrieve the subsequent page in a multi-page list of
    /// results. When paginating, all other parameters provided to
    /// `ListVolumeBackups` must match the call that provided the page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Field match expression used to filter the results.
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Field by which to sort the results.
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
/// Response message for ListVolumeBackups.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListVolumeBackupsResponse {
    /// The list of VolumeBackups matching the given criteria.
    #[prost(message, repeated, tag = "1")]
    pub volume_backups: ::prost::alloc::vec::Vec<VolumeBackup>,
    /// A token which may be sent as
    /// \[page_token][google.cloud.gkebackup.v1.ListVolumeBackupsRequest.page_token\] in a subsequent
    /// `ListVolumeBackups` call to retrieve the next page of results.
    /// If this field is omitted or empty, then there are no more results to
    /// return.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for GetVolumeBackup.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetVolumeBackupRequest {
    /// Required. Full name of the VolumeBackup resource.
    /// Format: projects/*/locations/*/backupPlans/*/backups/*/volumeBackups/*
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for CreateRestorePlan.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateRestorePlanRequest {
    /// Required. The location within which to create the RestorePlan.
    /// Format: projects/*/locations/*
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The RestorePlan resource object to create.
    #[prost(message, optional, tag = "2")]
    pub restore_plan: ::core::option::Option<RestorePlan>,
    /// Required. The client-provided short name for the RestorePlan resource.
    /// This name must:
    ///
    ///  - be between 1 and 63 characters long (inclusive)
    ///  - consist of only lower-case ASCII letters, numbers, and dashes
    ///  - start with a lower-case letter
    ///  - end with a lower-case letter or number
    ///  - be unique within the set of RestorePlans in this location
    #[prost(string, tag = "3")]
    pub restore_plan_id: ::prost::alloc::string::String,
}
/// Request message for ListRestorePlans.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListRestorePlansRequest {
    /// Required. The location that contains the RestorePlans to list.
    /// Format: projects/*/locations/*
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The target number of results to return in a single response.
    /// If not specified, a default value will be chosen by the service.
    /// Note that the response may inclue a partial list and a caller should
    /// only rely on the response's
    /// \[next_page_token][google.cloud.gkebackup.v1.ListRestorePlansResponse.next_page_token\]
    /// to determine if there are more instances left to be queried.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The value of
    /// \[next_page_token][google.cloud.gkebackup.v1.ListRestorePlansResponse.next_page_token\]
    /// received from a previous `ListRestorePlans` call.
    /// Provide this to retrieve the subsequent page in a multi-page list of
    /// results. When paginating, all other parameters provided to
    /// `ListRestorePlans` must match the call that provided the page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Field match expression used to filter the results.
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Field by which to sort the results.
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
/// Response message for ListRestorePlans.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListRestorePlansResponse {
    /// The list of RestorePlans matching the given criteria.
    #[prost(message, repeated, tag = "1")]
    pub restore_plans: ::prost::alloc::vec::Vec<RestorePlan>,
    /// A token which may be sent as
    /// \[page_token][google.cloud.gkebackup.v1.ListRestorePlansRequest.page_token\] in a subsequent
    /// `ListRestorePlans` call to retrieve the next page of results.
    /// If this field is omitted or empty, then there are no more results to
    /// return.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Request message for GetRestorePlan.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRestorePlanRequest {
    /// Required. Fully qualified RestorePlan name.
    /// Format: projects/*/locations/*/restorePlans/*
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for UpdateRestorePlan.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateRestorePlanRequest {
    /// Required. A new version of the RestorePlan resource that contains updated fields.
    /// This may be sparsely populated if an `update_mask` is provided.
    #[prost(message, optional, tag = "1")]
    pub restore_plan: ::core::option::Option<RestorePlan>,
    /// This is used to specify the fields to be overwritten in the
    /// RestorePlan targeted for update. The values for each of these
    /// updated fields will be taken from the `restore_plan` provided
    /// with this request. Field names are relative to the root of the resource.
    /// If no `update_mask` is provided, all fields in `restore_plan` will be
    /// written to the target RestorePlan resource.
    /// Note that OUTPUT_ONLY and IMMUTABLE fields in `restore_plan` are ignored
    /// and are not used to update the target RestorePlan.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for DeleteRestorePlan.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteRestorePlanRequest {
    /// Required. Fully qualified RestorePlan name.
    /// Format: projects/*/locations/*/restorePlans/*
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// If provided, this value must match the current value of the
    /// target RestorePlan's \[etag][google.cloud.gkebackup.v1.RestorePlan.etag\] field or the request is
    /// rejected.
    #[prost(string, tag = "2")]
    pub etag: ::prost::alloc::string::String,
    /// If set to true, any Restores below this RestorePlan will also be deleted.
    /// Otherwise, the request will only succeed if the RestorePlan has no
    /// Restores.
    #[prost(bool, tag = "3")]
    pub force: bool,
}
/// Request message for CreateRestore.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateRestoreRequest {
    /// Required. The RestorePlan within which to create the Restore.
    /// Format: projects/*/locations/*/restorePlans/*
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The restore resource to create.
    #[prost(message, optional, tag = "2")]
    pub restore: ::core::option::Option<Restore>,
    /// Required. The client-provided short name for the Restore resource.
    /// This name must:
    ///
    ///  - be between 1 and 63 characters long (inclusive)
    ///  - consist of only lower-case ASCII letters, numbers, and dashes
    ///  - start with a lower-case letter
    ///  - end with a lower-case letter or number
    ///  - be unique within the set of Restores in this RestorePlan.
    #[prost(string, tag = "3")]
    pub restore_id: ::prost::alloc::string::String,
}
/// Request message for ListRestores.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListRestoresRequest {
    /// Required. The RestorePlan that contains the Restores to list.
    /// Format: projects/*/locations/*/restorePlans/*
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The target number of results to return in a single response.
    /// If not specified, a default value will be chosen by the service.
    /// Note that the response may inclue a partial list and a caller should
    /// only rely on the response's
    /// \[next_page_token][google.cloud.gkebackup.v1.ListRestoresResponse.next_page_token\]
    /// to determine if there are more instances left to be queried.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The value of
    /// \[next_page_token][google.cloud.gkebackup.v1.ListRestoresResponse.next_page_token\]
    /// received from a previous `ListRestores` call.
    /// Provide this to retrieve the subsequent page in a multi-page list of
    /// results. When paginating, all other parameters provided to `ListRestores`
    /// must match the call that provided the page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Field match expression used to filter the results.
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Field by which to sort the results.
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
/// Response message for ListRestores.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListRestoresResponse {
    /// The list of Restores matching the given criteria.
    #[prost(message, repeated, tag = "1")]
    pub restores: ::prost::alloc::vec::Vec<Restore>,
    /// A token which may be sent as \[page_token][google.cloud.gkebackup.v1.ListRestoresRequest.page_token\]
    /// in a subsequent `ListRestores` call to retrieve the next page of results.
    /// If this field is omitted or empty, then there are no more results to
    /// return.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Request message for GetRestore.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRestoreRequest {
    /// Required. Name of the restore resource.
    /// Format: projects/*/locations/*/restorePlans/*/restores/*
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for UpdateRestore.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateRestoreRequest {
    /// Required. A new version of the Restore resource that contains updated fields.
    /// This may be sparsely populated if an `update_mask` is provided.
    #[prost(message, optional, tag = "1")]
    pub restore: ::core::option::Option<Restore>,
    /// This is used to specify the fields to be overwritten in the
    /// Restore targeted for update. The values for each of these
    /// updated fields will be taken from the `restore` provided
    /// with this request. Field names are relative to the root of the resource.
    /// If no `update_mask` is provided, all fields in `restore` will be
    /// written to the target Restore resource.
    /// Note that OUTPUT_ONLY and IMMUTABLE fields in `restore` are ignored
    /// and are not used to update the target Restore.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for DeleteRestore.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteRestoreRequest {
    /// Required. Full name of the Restore
    /// Format: projects/*/locations/*/restorePlans/*/restores/*
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// If provided, this value must match the current value of the
    /// target Restore's \[etag][google.cloud.gkebackup.v1.Restore.etag\] field or the request is
    /// rejected.
    #[prost(string, tag = "2")]
    pub etag: ::prost::alloc::string::String,
    /// If set to true, any VolumeRestores below this restore will also be deleted.
    /// Otherwise, the request will only succeed if the restore has no
    /// VolumeRestores.
    #[prost(bool, tag = "3")]
    pub force: bool,
}
/// Request message for ListVolumeRestores.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListVolumeRestoresRequest {
    /// Required. The Restore that contains the VolumeRestores to list.
    /// Format: projects/*/locations/*/restorePlans/*/restores/*
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The target number of results to return in a single response.
    /// If not specified, a default value will be chosen by the service.
    /// Note that the response may inclue a partial list and a caller should
    /// only rely on the response's
    /// \[next_page_token][google.cloud.gkebackup.v1.ListVolumeRestoresResponse.next_page_token\]
    /// to determine if there are more instances left to be queried.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The value of
    /// \[next_page_token][google.cloud.gkebackup.v1.ListVolumeRestoresResponse.next_page_token\]
    /// received from a previous `ListVolumeRestores` call.
    /// Provide this to retrieve the subsequent page in a multi-page list of
    /// results. When paginating, all other parameters provided to
    /// `ListVolumeRestores` must match the call that provided the page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Field match expression used to filter the results.
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Field by which to sort the results.
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
/// Response message for ListVolumeRestores.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListVolumeRestoresResponse {
    /// The list of VolumeRestores matching the given criteria.
    #[prost(message, repeated, tag = "1")]
    pub volume_restores: ::prost::alloc::vec::Vec<VolumeRestore>,
    /// A token which may be sent as
    /// \[page_token][google.cloud.gkebackup.v1.ListVolumeRestoresRequest.page_token\] in a subsequent
    /// `ListVolumeRestores` call to retrieve the next page of results.
    /// If this field is omitted or empty, then there are no more results to
    /// return.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for GetVolumeRestore.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetVolumeRestoreRequest {
    /// Required. Full name of the VolumeRestore resource.
    /// Format: projects/*/locations/*/restorePlans/*/restores/*/volumeRestores/*
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
#[doc = r" Generated client implementations."]
pub mod backup_for_gke_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " BackupForGKE allows Kubernetes administrators to configure, execute, and"]
    #[doc = " manage backup and restore operations for their GKE clusters."]
    #[derive(Debug, Clone)]
    pub struct BackupForGkeClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> BackupForGkeClient<T>
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
        ) -> BackupForGkeClient<InterceptedService<T, F>>
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
            BackupForGkeClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Creates a new BackupPlan in a given location."]
        pub async fn create_backup_plan(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateBackupPlanRequest>,
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
                "/google.cloud.gkebackup.v1.BackupForGKE/CreateBackupPlan",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists BackupPlans in a given location."]
        pub async fn list_backup_plans(
            &mut self,
            request: impl tonic::IntoRequest<super::ListBackupPlansRequest>,
        ) -> Result<tonic::Response<super::ListBackupPlansResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.gkebackup.v1.BackupForGKE/ListBackupPlans",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Retrieve the details of a single BackupPlan."]
        pub async fn get_backup_plan(
            &mut self,
            request: impl tonic::IntoRequest<super::GetBackupPlanRequest>,
        ) -> Result<tonic::Response<super::BackupPlan>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.gkebackup.v1.BackupForGKE/GetBackupPlan",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Update a BackupPlan."]
        pub async fn update_backup_plan(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateBackupPlanRequest>,
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
                "/google.cloud.gkebackup.v1.BackupForGKE/UpdateBackupPlan",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes an existing BackupPlan."]
        pub async fn delete_backup_plan(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteBackupPlanRequest>,
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
                "/google.cloud.gkebackup.v1.BackupForGKE/DeleteBackupPlan",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a Backup for the given BackupPlan."]
        pub async fn create_backup(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateBackupRequest>,
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
                "/google.cloud.gkebackup.v1.BackupForGKE/CreateBackup",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists the Backups for a given BackupPlan."]
        pub async fn list_backups(
            &mut self,
            request: impl tonic::IntoRequest<super::ListBackupsRequest>,
        ) -> Result<tonic::Response<super::ListBackupsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.gkebackup.v1.BackupForGKE/ListBackups",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Retrieve the details of a single Backup."]
        pub async fn get_backup(
            &mut self,
            request: impl tonic::IntoRequest<super::GetBackupRequest>,
        ) -> Result<tonic::Response<super::Backup>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.gkebackup.v1.BackupForGKE/GetBackup",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Update a Backup."]
        pub async fn update_backup(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateBackupRequest>,
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
                "/google.cloud.gkebackup.v1.BackupForGKE/UpdateBackup",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes an existing Backup."]
        pub async fn delete_backup(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteBackupRequest>,
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
                "/google.cloud.gkebackup.v1.BackupForGKE/DeleteBackup",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists the VolumeBackups for a given Backup."]
        pub async fn list_volume_backups(
            &mut self,
            request: impl tonic::IntoRequest<super::ListVolumeBackupsRequest>,
        ) -> Result<tonic::Response<super::ListVolumeBackupsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.gkebackup.v1.BackupForGKE/ListVolumeBackups",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Retrieve the details of a single VolumeBackup."]
        pub async fn get_volume_backup(
            &mut self,
            request: impl tonic::IntoRequest<super::GetVolumeBackupRequest>,
        ) -> Result<tonic::Response<super::VolumeBackup>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.gkebackup.v1.BackupForGKE/GetVolumeBackup",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a new RestorePlan in a given location."]
        pub async fn create_restore_plan(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateRestorePlanRequest>,
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
                "/google.cloud.gkebackup.v1.BackupForGKE/CreateRestorePlan",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists RestorePlans in a given location."]
        pub async fn list_restore_plans(
            &mut self,
            request: impl tonic::IntoRequest<super::ListRestorePlansRequest>,
        ) -> Result<tonic::Response<super::ListRestorePlansResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.gkebackup.v1.BackupForGKE/ListRestorePlans",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Retrieve the details of a single RestorePlan."]
        pub async fn get_restore_plan(
            &mut self,
            request: impl tonic::IntoRequest<super::GetRestorePlanRequest>,
        ) -> Result<tonic::Response<super::RestorePlan>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.gkebackup.v1.BackupForGKE/GetRestorePlan",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Update a RestorePlan."]
        pub async fn update_restore_plan(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateRestorePlanRequest>,
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
                "/google.cloud.gkebackup.v1.BackupForGKE/UpdateRestorePlan",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes an existing RestorePlan."]
        pub async fn delete_restore_plan(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteRestorePlanRequest>,
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
                "/google.cloud.gkebackup.v1.BackupForGKE/DeleteRestorePlan",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a new Restore for the given RestorePlan."]
        pub async fn create_restore(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateRestoreRequest>,
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
                "/google.cloud.gkebackup.v1.BackupForGKE/CreateRestore",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists the Restores for a given RestorePlan."]
        pub async fn list_restores(
            &mut self,
            request: impl tonic::IntoRequest<super::ListRestoresRequest>,
        ) -> Result<tonic::Response<super::ListRestoresResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.gkebackup.v1.BackupForGKE/ListRestores",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Retrieves the details of a single Restore."]
        pub async fn get_restore(
            &mut self,
            request: impl tonic::IntoRequest<super::GetRestoreRequest>,
        ) -> Result<tonic::Response<super::Restore>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.gkebackup.v1.BackupForGKE/GetRestore",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Update a Restore."]
        pub async fn update_restore(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateRestoreRequest>,
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
                "/google.cloud.gkebackup.v1.BackupForGKE/UpdateRestore",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes an existing Restore."]
        pub async fn delete_restore(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteRestoreRequest>,
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
                "/google.cloud.gkebackup.v1.BackupForGKE/DeleteRestore",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists the VolumeRestores for a given Restore."]
        pub async fn list_volume_restores(
            &mut self,
            request: impl tonic::IntoRequest<super::ListVolumeRestoresRequest>,
        ) -> Result<tonic::Response<super::ListVolumeRestoresResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.gkebackup.v1.BackupForGKE/ListVolumeRestores",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Retrieve the details of a single VolumeRestore."]
        pub async fn get_volume_restore(
            &mut self,
            request: impl tonic::IntoRequest<super::GetVolumeRestoreRequest>,
        ) -> Result<tonic::Response<super::VolumeRestore>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.gkebackup.v1.BackupForGKE/GetVolumeRestore",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
