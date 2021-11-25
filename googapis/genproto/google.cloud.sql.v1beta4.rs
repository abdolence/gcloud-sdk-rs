// NOTE: No sensitive PII logging is allowed. If you are adding a field/enum
// value that is sensitive PII, add corresponding datapol annotation to
// it. For more information, see
// <https://g3doc.corp.google.com/storage/speckle/g3doc/purple_team/data_pol_annotations.md?cl=head>

/// An entry for an Access Control list.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AclEntry {
    /// The allowlisted value for the access control list.
    #[prost(string, tag = "1")]
    pub value: ::prost::alloc::string::String,
    /// The time when this access control entry expires in <a
    /// href="<https://tools.ietf.org/html/rfc3339">RFC> 3339</a> format, for example
    /// <b>2012-11-15T16:19:00.094Z</b>.
    #[prost(message, optional, tag = "2")]
    pub expiration_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Optional. A label to identify this entry.
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
    /// This is always <b>sql#aclEntry</b>.
    #[prost(string, tag = "4")]
    pub kind: ::prost::alloc::string::String,
}
/// An Admin API warning message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApiWarning {
    /// Code to uniquely identify the warning type.
    #[prost(enumeration = "api_warning::SqlApiWarningCode", tag = "1")]
    pub code: i32,
    /// The warning message.
    #[prost(string, tag = "2")]
    pub message: ::prost::alloc::string::String,
    /// The region name for REGION_UNREACHABLE warning.
    #[prost(string, tag = "3")]
    pub region: ::prost::alloc::string::String,
}
/// Nested message and enum types in `ApiWarning`.
pub mod api_warning {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum SqlApiWarningCode {
        /// An unknown or unset warning type from Cloud SQL API.
        Unspecified = 0,
        /// Warning when one or more regions are not reachable.  The returned result
        /// set may be incomplete.
        RegionUnreachable = 1,
    }
}
/// We currently only support backup retention by specifying the number
/// of backups we will retain.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BackupRetentionSettings {
    /// The unit that 'retained_backups' represents.
    #[prost(enumeration = "backup_retention_settings::RetentionUnit", tag = "1")]
    pub retention_unit: i32,
    /// Depending on the value of retention_unit, this is used to determine
    /// if a backup needs to be deleted.  If retention_unit is 'COUNT', we will
    /// retain this many backups.
    #[prost(message, optional, tag = "2")]
    pub retained_backups: ::core::option::Option<i32>,
}
/// Nested message and enum types in `BackupRetentionSettings`.
pub mod backup_retention_settings {
    /// The units that retained_backups specifies, we only support COUNT.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum RetentionUnit {
        /// Backup retention unit is unspecified, will be treated as COUNT.
        Unspecified = 0,
        /// Retention will be by count, eg. "retain the most recent 7 backups".
        Count = 1,
    }
}
/// Database instance backup configuration.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BackupConfiguration {
    /// Start time for the daily backup configuration in UTC timezone in the 24
    /// hour format - <b>HH:MM</b>.
    #[prost(string, tag = "1")]
    pub start_time: ::prost::alloc::string::String,
    /// Whether this configuration is enabled.
    #[prost(message, optional, tag = "2")]
    pub enabled: ::core::option::Option<bool>,
    /// This is always <b>sql#backupConfiguration</b>.
    #[prost(string, tag = "3")]
    pub kind: ::prost::alloc::string::String,
    /// (MySQL only) Whether binary log is enabled. If backup configuration is
    /// disabled, binarylog must be disabled as well.
    #[prost(message, optional, tag = "4")]
    pub binary_log_enabled: ::core::option::Option<bool>,
    /// Reserved for future use.
    #[prost(message, optional, tag = "5")]
    pub replication_log_archiving_enabled: ::core::option::Option<bool>,
    /// Location of the backup
    #[prost(string, tag = "6")]
    pub location: ::prost::alloc::string::String,
    /// Reserved for future use.
    #[prost(message, optional, tag = "7")]
    pub point_in_time_recovery_enabled: ::core::option::Option<bool>,
    /// The number of days of transaction logs we retain for point in time
    /// restore, from 1-7.
    #[prost(message, optional, tag = "9")]
    pub transaction_log_retention_days: ::core::option::Option<i32>,
    /// Backup retention settings.
    #[prost(message, optional, tag = "10")]
    pub backup_retention_settings: ::core::option::Option<BackupRetentionSettings>,
}
/// A BackupRun resource.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BackupRun {
    /// This is always <b>sql#backupRun</b>.
    #[prost(string, tag = "1")]
    pub kind: ::prost::alloc::string::String,
    /// The status of this run.
    #[prost(enumeration = "SqlBackupRunStatus", tag = "2")]
    pub status: i32,
    /// The time the run was enqueued in UTC timezone in <a
    /// href="<https://tools.ietf.org/html/rfc3339">RFC> 3339</a> format, for example
    /// <b>2012-11-15T16:19:00.094Z</b>.
    #[prost(message, optional, tag = "3")]
    pub enqueued_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The identifier for this backup run. Unique only for a specific Cloud SQL
    /// instance.
    #[prost(int64, tag = "4")]
    pub id: i64,
    /// The time the backup operation actually started in UTC timezone in <a
    /// href="<https://tools.ietf.org/html/rfc3339">RFC> 3339</a> format, for example
    /// <b>2012-11-15T16:19:00.094Z</b>.
    #[prost(message, optional, tag = "5")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The time the backup operation completed in UTC timezone in <a
    /// href="<https://tools.ietf.org/html/rfc3339">RFC> 3339</a> format, for example
    /// <b>2012-11-15T16:19:00.094Z</b>.
    #[prost(message, optional, tag = "6")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Information about why the backup operation failed. This is only present if
    /// the run has the FAILED status.
    #[prost(message, optional, tag = "7")]
    pub error: ::core::option::Option<OperationError>,
    /// The type of this run; can be either "AUTOMATED" or "ON_DEMAND". This field
    /// defaults to "ON_DEMAND" and is ignored, when specified for insert requests.
    #[prost(enumeration = "SqlBackupRunType", tag = "8")]
    pub r#type: i32,
    /// The description of this run, only applicable to on-demand backups.
    #[prost(string, tag = "9")]
    pub description: ::prost::alloc::string::String,
    /// The start time of the backup window during which this the backup was
    /// attempted in <a href="<https://tools.ietf.org/html/rfc3339">RFC> 3339</a>
    /// format, for example <b>2012-11-15T16:19:00.094Z</b>.
    #[prost(message, optional, tag = "10")]
    pub window_start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Name of the database instance.
    #[prost(string, tag = "11")]
    pub instance: ::prost::alloc::string::String,
    /// The URI of this resource.
    #[prost(string, tag = "12")]
    pub self_link: ::prost::alloc::string::String,
    /// Location of the backups.
    #[prost(string, tag = "13")]
    pub location: ::prost::alloc::string::String,
    /// Encryption configuration specific to a backup.
    /// Applies only to Second Generation instances.
    #[prost(message, optional, tag = "16")]
    pub disk_encryption_configuration: ::core::option::Option<DiskEncryptionConfiguration>,
    /// Encryption status specific to a backup.
    /// Applies only to Second Generation instances.
    #[prost(message, optional, tag = "17")]
    pub disk_encryption_status: ::core::option::Option<DiskEncryptionStatus>,
    /// Specifies the kind of backup, PHYSICAL or DEFAULT_SNAPSHOT.
    #[prost(enumeration = "SqlBackupKind", tag = "19")]
    pub backup_kind: i32,
}
/// Backup run list results.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BackupRunsListResponse {
    /// This is always <b>sql#backupRunsList</b>.
    #[prost(string, tag = "1")]
    pub kind: ::prost::alloc::string::String,
    /// A list of backup runs in reverse chronological order of the enqueued time.
    #[prost(message, repeated, tag = "2")]
    pub items: ::prost::alloc::vec::Vec<BackupRun>,
    /// The continuation token, used to page through large result sets. Provide
    /// this value in a subsequent request to return the next page of results.
    #[prost(string, tag = "3")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Binary log coordinates.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BinLogCoordinates {
    /// Name of the binary log file for a Cloud SQL instance.
    #[prost(string, tag = "1")]
    pub bin_log_file_name: ::prost::alloc::string::String,
    /// Position (offset) within the binary log file.
    #[prost(int64, tag = "2")]
    pub bin_log_position: i64,
    /// This is always <b>sql#binLogCoordinates</b>.
    #[prost(string, tag = "3")]
    pub kind: ::prost::alloc::string::String,
}
/// Backup context.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BackupContext {
    /// The identifier of the backup.
    #[prost(int64, tag = "1")]
    pub backup_id: i64,
    /// This is always <b>sql#backupContext</b>.
    #[prost(string, tag = "2")]
    pub kind: ::prost::alloc::string::String,
}
/// Database instance clone context.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CloneContext {
    /// This is always <b>sql#cloneContext</b>.
    #[prost(string, tag = "1")]
    pub kind: ::prost::alloc::string::String,
    /// Reserved for future use.
    #[prost(int64, tag = "2")]
    pub pitr_timestamp_ms: i64,
    /// Name of the Cloud SQL instance to be created as a clone.
    #[prost(string, tag = "3")]
    pub destination_instance_name: ::prost::alloc::string::String,
    /// Binary log coordinates, if specified, identify the position up to which the
    /// source instance is cloned. If not specified, the source instance is
    /// cloned up to the most recent binary log coordinates.
    #[prost(message, optional, tag = "4")]
    pub bin_log_coordinates: ::core::option::Option<BinLogCoordinates>,
    /// Reserved for future use.
    #[prost(message, optional, tag = "5")]
    pub point_in_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Represents a SQL database on the Cloud SQL instance.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Database {
    /// This is always <b>sql#database</b>.
    #[prost(string, tag = "1")]
    pub kind: ::prost::alloc::string::String,
    /// The Cloud SQL charset value.
    #[prost(string, tag = "2")]
    pub charset: ::prost::alloc::string::String,
    /// The Cloud SQL collation value.
    #[prost(string, tag = "3")]
    pub collation: ::prost::alloc::string::String,
    /// This field is deprecated and will be removed from a future version of the
    /// API.
    #[prost(string, tag = "4")]
    pub etag: ::prost::alloc::string::String,
    /// The name of the database in the Cloud SQL instance. This does not include
    /// the project ID or instance name.
    #[prost(string, tag = "5")]
    pub name: ::prost::alloc::string::String,
    /// The name of the Cloud SQL instance. This does not include the project ID.
    #[prost(string, tag = "6")]
    pub instance: ::prost::alloc::string::String,
    /// The URI of this resource.
    #[prost(string, tag = "7")]
    pub self_link: ::prost::alloc::string::String,
    /// The project ID of the project containing the Cloud SQL database. The Google
    /// apps domain is prefixed if applicable.
    #[prost(string, tag = "8")]
    pub project: ::prost::alloc::string::String,
    #[prost(oneof = "database::DatabaseDetails", tags = "9")]
    pub database_details: ::core::option::Option<database::DatabaseDetails>,
}
/// Nested message and enum types in `Database`.
pub mod database {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum DatabaseDetails {
        #[prost(message, tag = "9")]
        SqlserverDatabaseDetails(super::SqlServerDatabaseDetails),
    }
}
/// Represents a Sql Server database on the Cloud SQL instance.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SqlServerDatabaseDetails {
    /// The version of SQL Server with which the database is to be made compatible
    #[prost(int32, tag = "1")]
    pub compatibility_level: i32,
    /// The recovery model of a SQL Server database
    #[prost(string, tag = "2")]
    pub recovery_model: ::prost::alloc::string::String,
}
/// Database flags for Cloud SQL instances.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DatabaseFlags {
    /// The name of the flag. These flags are passed at instance startup, so
    /// include both server options and system variables for MySQL. Flags are
    /// specified with underscores, not hyphens. For more information, see <a
    /// href="/sql/docs/mysql/flags">Configuring Database Flags</a> in the Cloud
    /// SQL documentation.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The value of the flag. Booleans are set to <b>on</b> for true
    /// and <b>off</b> for false. This field must be omitted if the flag
    /// doesn't take a value.
    #[prost(string, tag = "2")]
    pub value: ::prost::alloc::string::String,
}
/// A Cloud SQL instance resource.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DatabaseInstance {
    /// This is always <b>sql#instance</b>.
    #[prost(string, tag = "1")]
    pub kind: ::prost::alloc::string::String,
    /// The current serving state of the Cloud SQL instance. This can be one of the
    /// following. <br><b>SQL_INSTANCE_STATE_UNSPECIFIED</b>: The state of the
    /// instance is unknown. <br><b>RUNNABLE</b>: The instance is running, or has
    /// been stopped by owner.
    /// <br><b>SUSPENDED</b>: The instance is not available, for example due to
    /// problems with billing.
    /// <br><b>PENDING_DELETE</b>: The instance is being deleted.
    /// <br><b>PENDING_CREATE</b>: The instance is being created.
    /// <br><b>MAINTENANCE</b>: The instance is down for maintenance.
    /// <br><b>FAILED</b>: The instance creation failed.
    #[prost(enumeration = "database_instance::SqlInstanceState", tag = "2")]
    pub state: i32,
    /// The database engine type and version. The <b>databaseVersion</b>
    /// field cannot be changed after instance creation.
    /// <br>MySQL instances: <b>MYSQL_8_0</b>, <b>MYSQL_5_7</b> (default),
    /// or <b>MYSQL_5_6</b>.
    /// <br>PostgreSQL instances: <b>POSTGRES_9_6</b>, <b>POSTGRES_10</b>,
    /// <b>POSTGRES_11</b>, <b>POSTGRES_12</b>, or <b>POSTGRES_13</b> (default).
    /// <br>SQL Server instances: <b>SQLSERVER_2017_STANDARD</b> (default),
    /// <b>SQLSERVER_2017_ENTERPRISE</b>, <b>SQLSERVER_2017_EXPRESS</b>, or
    /// <b>SQLSERVER_2017_WEB</b>.
    #[prost(enumeration = "SqlDatabaseVersion", tag = "3")]
    pub database_version: i32,
    /// The user settings.
    #[prost(message, optional, tag = "4")]
    pub settings: ::core::option::Option<Settings>,
    /// This field is deprecated and will be removed from a future version of the
    /// API. Use the <b>settings.settingsVersion</b> field instead.
    #[prost(string, tag = "5")]
    pub etag: ::prost::alloc::string::String,
    /// The name and status of the failover replica. This property is applicable
    /// only to Second Generation instances.
    #[prost(message, optional, tag = "6")]
    pub failover_replica: ::core::option::Option<database_instance::SqlFailoverReplica>,
    /// The name of the instance which will act as primary in the replication
    /// setup.
    #[prost(string, tag = "7")]
    pub master_instance_name: ::prost::alloc::string::String,
    /// The replicas of the instance.
    #[prost(string, repeated, tag = "8")]
    pub replica_names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The maximum disk size of the instance in bytes.
    #[deprecated]
    #[prost(message, optional, tag = "9")]
    pub max_disk_size: ::core::option::Option<i64>,
    /// The current disk usage of the instance in bytes. This property has been
    /// deprecated. Use the
    /// "cloudsql.googleapis.com/database/disk/bytes_used" metric in Cloud
    /// Monitoring API instead. Please see <a
    /// href="<https://groups.google.com/d/msg/google-cloud-sql-announce/I_7-F9EBhT0/BtvFtdFeAgAJ">this>
    /// announcement</a> for details.
    #[deprecated]
    #[prost(message, optional, tag = "10")]
    pub current_disk_size: ::core::option::Option<i64>,
    /// The assigned IP addresses for the instance.
    #[prost(message, repeated, tag = "11")]
    pub ip_addresses: ::prost::alloc::vec::Vec<IpMapping>,
    /// SSL configuration.
    #[prost(message, optional, tag = "12")]
    pub server_ca_cert: ::core::option::Option<SslCert>,
    /// The instance type. This can be one of the following.
    /// <br><b>CLOUD_SQL_INSTANCE</b>: A Cloud SQL instance that is not replicating
    /// from a primary instance. <br><b>ON_PREMISES_INSTANCE</b>: An instance
    /// running on the customer's premises. <br><b>READ_REPLICA_INSTANCE</b>: A
    /// Cloud SQL instance configured as a read-replica.
    #[prost(enumeration = "SqlInstanceType", tag = "13")]
    pub instance_type: i32,
    /// The project ID of the project containing the Cloud SQL instance. The Google
    /// apps domain is prefixed if applicable.
    #[prost(string, tag = "14")]
    pub project: ::prost::alloc::string::String,
    /// The IPv6 address assigned to the instance.
    /// (Deprecated) This property was applicable only
    /// to First Generation instances.
    #[deprecated]
    #[prost(string, tag = "15")]
    pub ipv6_address: ::prost::alloc::string::String,
    /// The service account email address assigned to the instance. <br>This
    /// property is read-only.
    #[prost(string, tag = "16")]
    pub service_account_email_address: ::prost::alloc::string::String,
    /// Configuration specific to on-premises instances.
    #[prost(message, optional, tag = "17")]
    pub on_premises_configuration: ::core::option::Option<OnPremisesConfiguration>,
    /// Configuration specific to failover replicas and read replicas.
    #[prost(message, optional, tag = "18")]
    pub replica_configuration: ::core::option::Option<ReplicaConfiguration>,
    /// <br><b>SECOND_GEN</b>: Cloud SQL database instance.
    /// <br><b>EXTERNAL</b>: A database server that is not
    /// managed by Google. <br>This property is read-only; use the
    /// <b>tier</b> property in the <b>settings</b> object to determine
    /// the database type.
    #[prost(enumeration = "SqlBackendType", tag = "19")]
    pub backend_type: i32,
    /// The URI of this resource.
    #[prost(string, tag = "20")]
    pub self_link: ::prost::alloc::string::String,
    /// If the instance state is SUSPENDED, the reason for the suspension.
    #[prost(enumeration = "SqlSuspensionReason", repeated, tag = "21")]
    pub suspension_reason: ::prost::alloc::vec::Vec<i32>,
    /// Connection name of the Cloud SQL instance used in connection strings.
    #[prost(string, tag = "22")]
    pub connection_name: ::prost::alloc::string::String,
    /// Name of the Cloud SQL instance. This does not include the project ID.
    #[prost(string, tag = "23")]
    pub name: ::prost::alloc::string::String,
    /// The geographical region. Can be
    /// <br><b>us-central</b> (<b>FIRST_GEN</b> instances only)
    /// <br><b>us-central1</b> (<b>SECOND_GEN</b> instances only)
    /// <br><b>asia-east1</b> or <b>europe-west1</b>.
    /// <br>Defaults to <b>us-central</b> or
    /// <b>us-central1</b> depending on the instance type.
    /// The region cannot be changed after instance creation.
    #[prost(string, tag = "24")]
    pub region: ::prost::alloc::string::String,
    /// The Compute Engine zone that the instance is currently serving from. This
    /// value could be different from the zone that was specified when the instance
    /// was created if the instance has failed over to its secondary zone.
    #[prost(string, tag = "25")]
    pub gce_zone: ::prost::alloc::string::String,
    /// The Compute Engine zone that the failover instance is currently serving
    /// from for a regional instance. This value could be different
    /// from the zone that was specified when the instance
    /// was created if the instance has failed over to its secondary/failover zone.
    /// Reserved for future use.
    #[prost(string, tag = "34")]
    pub secondary_gce_zone: ::prost::alloc::string::String,
    /// Disk encryption configuration specific to an instance.
    /// Applies only to Second Generation instances.
    #[prost(message, optional, tag = "26")]
    pub disk_encryption_configuration: ::core::option::Option<DiskEncryptionConfiguration>,
    /// Disk encryption status specific to an instance.
    /// Applies only to Second Generation instances.
    #[prost(message, optional, tag = "27")]
    pub disk_encryption_status: ::core::option::Option<DiskEncryptionStatus>,
    /// Initial root password. Use only on creation.
    #[prost(string, tag = "29")]
    pub root_password: ::prost::alloc::string::String,
    /// The start time of any upcoming scheduled maintenance for this instance.
    #[prost(message, optional, tag = "30")]
    pub scheduled_maintenance: ::core::option::Option<database_instance::SqlScheduledMaintenance>,
    /// The status indicating if instance satisfiesPzs.
    /// Reserved for future use.
    #[prost(message, optional, tag = "35")]
    pub satisfies_pzs: ::core::option::Option<bool>,
    /// This field represents the report generated by the proactive database
    /// wellness job for OutOfDisk issues.
    /// Writers:
    ///    -- the proactive database wellness job for OOD.
    /// Readers:
    ///    -- the proactive database wellness job
    #[prost(message, optional, tag = "38")]
    pub out_of_disk_report: ::core::option::Option<database_instance::SqlOutOfDiskReport>,
}
/// Nested message and enum types in `DatabaseInstance`.
pub mod database_instance {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SqlFailoverReplica {
        /// The name of the failover replica. If specified at instance creation, a
        /// failover replica is created for the instance. The name
        /// doesn't include the project ID. This property is applicable only to
        /// Second Generation instances.
        #[prost(string, tag = "1")]
        pub name: ::prost::alloc::string::String,
        /// The availability status of the failover replica. A false status indicates
        /// that the failover replica is out of sync. The primary instance can only
        /// failover to the failover replica when the status is true.
        #[prost(message, optional, tag = "2")]
        pub available: ::core::option::Option<bool>,
    }
    /// Any scheduled maintenancce for this instance.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SqlScheduledMaintenance {
        /// The start time of any upcoming scheduled maintenance for this instance.
        #[prost(message, optional, tag = "1")]
        pub start_time: ::core::option::Option<::prost_types::Timestamp>,
        #[deprecated]
        #[prost(bool, tag = "2")]
        pub can_defer: bool,
        /// If the scheduled maintenance can be rescheduled.
        #[prost(bool, tag = "3")]
        pub can_reschedule: bool,
    }
    /// This message wraps up the information written by out-of-disk detection job.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SqlOutOfDiskReport {
        /// This field represents the state generated by the proactive database
        /// wellness job for OutOfDisk issues.
        /// Writers:
        ///    -- the proactive database wellness job for OOD.
        /// Readers:
        ///    -- the proactive database wellness job
        #[prost(enumeration = "sql_out_of_disk_report::SqlOutOfDiskState", optional, tag = "1")]
        pub sql_out_of_disk_state: ::core::option::Option<i32>,
        /// The minimum recommended increase size in GigaBytes
        /// This field is consumed by the frontend
        /// Writers:
        ///    -- the proactive database wellness job for OOD.
        #[prost(int32, optional, tag = "2")]
        pub sql_min_recommended_increase_size_gb: ::core::option::Option<i32>,
    }
    /// Nested message and enum types in `SqlOutOfDiskReport`.
    pub mod sql_out_of_disk_report {
        /// This enum lists all possible states regarding out-of-disk issues.
        #[derive(
            Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration,
        )]
        #[repr(i32)]
        pub enum SqlOutOfDiskState {
            /// Unspecified state
            Unspecified = 0,
            /// The instance has plenty space on data disk
            Normal = 1,
            /// Data disk is almost used up. It is shutdown to prevent data
            /// corruption.
            SoftShutdown = 2,
        }
    }
    /// The current serving state of the database instance.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum SqlInstanceState {
        /// The state of the instance is unknown.
        Unspecified = 0,
        /// The instance is running, or has been stopped by owner.
        Runnable = 1,
        /// The instance is not available, for example due to problems with billing.
        Suspended = 2,
        /// The instance is being deleted.
        PendingDelete = 3,
        /// The instance is being created.
        PendingCreate = 4,
        /// The instance is down for maintenance.
        Maintenance = 5,
        /// The creation of the instance failed or a fatal error occurred during
        /// maintenance.
        Failed = 6,
    }
}
/// Database list response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DatabasesListResponse {
    /// This is always <b>sql#databasesList</b>.
    #[prost(string, tag = "1")]
    pub kind: ::prost::alloc::string::String,
    /// List of database resources in the instance.
    #[prost(message, repeated, tag = "2")]
    pub items: ::prost::alloc::vec::Vec<Database>,
}
/// Read-replica configuration for connecting to the on-premises primary
/// instance.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DemoteMasterConfiguration {
    /// This is always <b>sql#demoteMasterConfiguration</b>.
    #[prost(string, tag = "1")]
    pub kind: ::prost::alloc::string::String,
    /// MySQL specific configuration when replicating from a MySQL on-premises
    /// primary instance. Replication configuration information such as the
    /// username, password, certificates, and keys are not stored in the instance
    /// metadata. The configuration information is used only to set up the
    /// replication connection and is stored by MySQL in a file named
    /// <b>master.info</b> in the data directory.
    #[prost(message, optional, tag = "2")]
    pub mysql_replica_configuration: ::core::option::Option<DemoteMasterMySqlReplicaConfiguration>,
}
/// Database instance demote primary instance context.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DemoteMasterContext {
    /// This is always <b>sql#demoteMasterContext</b>.
    #[prost(string, tag = "1")]
    pub kind: ::prost::alloc::string::String,
    /// Verify GTID consistency for demote operation. Default value:
    /// <b>True</b>. Second Generation instances only.  Setting this flag to
    /// false enables you to bypass GTID consistency check between on-premises
    /// primary instance and Cloud SQL instance during the demotion operation but
    /// also exposes you to the risk of future replication failures. Change the
    /// value only if you know the reason for the GTID divergence and are confident
    /// that doing so will not cause any replication issues.
    #[prost(message, optional, tag = "2")]
    pub verify_gtid_consistency: ::core::option::Option<bool>,
    /// The name of the instance which will act as on-premises primary instance
    /// in the replication setup.
    #[prost(string, tag = "3")]
    pub master_instance_name: ::prost::alloc::string::String,
    /// Configuration specific to read-replicas replicating from the on-premises
    /// primary instance.
    #[prost(message, optional, tag = "4")]
    pub replica_configuration: ::core::option::Option<DemoteMasterConfiguration>,
}
/// Read-replica configuration specific to MySQL databases.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DemoteMasterMySqlReplicaConfiguration {
    /// This is always <b>sql#demoteMasterMysqlReplicaConfiguration</b>.
    #[prost(string, tag = "1")]
    pub kind: ::prost::alloc::string::String,
    /// The username for the replication connection.
    #[prost(string, tag = "2")]
    pub username: ::prost::alloc::string::String,
    /// The password for the replication connection.
    #[prost(string, tag = "3")]
    pub password: ::prost::alloc::string::String,
    /// PEM representation of the replica's private key. The corresponsing public
    /// key is encoded in the client's certificate. The format of the replica's
    /// private key can be either PKCS #1 or PKCS #8.
    #[prost(string, tag = "4")]
    pub client_key: ::prost::alloc::string::String,
    /// PEM representation of the replica's x509 certificate.
    #[prost(string, tag = "5")]
    pub client_certificate: ::prost::alloc::string::String,
    /// PEM representation of the trusted CA's x509 certificate.
    #[prost(string, tag = "6")]
    pub ca_certificate: ::prost::alloc::string::String,
}
/// Database instance export context.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportContext {
    /// The path to the file in Google Cloud Storage where the export will be
    /// stored. The URI is in the form <b>gs://bucketName/fileName</b>. If the file
    /// already exists, the request succeeds, but the operation fails.
    /// <br>If <b>fileType</b> is <b>SQL</b> and the filename ends with .gz,
    /// the contents are compressed.
    #[prost(string, tag = "1")]
    pub uri: ::prost::alloc::string::String,
    /// Databases to be exported. <br /> <b>MySQL instances:</b> If
    /// <b>fileType</b> is <b>SQL</b> and no database is specified, all
    /// databases are exported, except for the <b>mysql</b> system database.
    /// If <b>fileType</b> is <b>CSV</b>, you can specify one database,
    /// either by using this property or by using the
    /// <b>csvExportOptions.selectQuery</b> property, which takes precedence
    /// over this property. <br /> <b>PostgreSQL instances:</b> You must specify
    /// one database to be exported. If <b>fileType</b> is <b>CSV</b>,
    /// this database must match the one specified in the
    /// <b>csvExportOptions.selectQuery</b> property. <br /> <b>SQL Server
    /// instances:</b> You must specify one database to be exported, and the
    /// <b>fileType</b> must be <b>BAK</b>.
    #[prost(string, repeated, tag = "2")]
    pub databases: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// This is always <b>sql#exportContext</b>.
    #[prost(string, tag = "3")]
    pub kind: ::prost::alloc::string::String,
    /// Options for exporting data as SQL statements.
    #[prost(message, optional, tag = "4")]
    pub sql_export_options: ::core::option::Option<export_context::SqlExportOptions>,
    /// Options for exporting data as CSV. <b>MySQL</b> and <b>PostgreSQL</b>
    /// instances only.
    #[prost(message, optional, tag = "5")]
    pub csv_export_options: ::core::option::Option<export_context::SqlCsvExportOptions>,
    /// The file type for the specified uri. <br><b>SQL</b>: The file
    /// contains SQL statements. <br><b>CSV</b>: The file contains CSV data.
    /// <br><b>BAK</b>: The file contains backup data for a SQL Server instance.
    #[prost(enumeration = "SqlFileType", tag = "6")]
    pub file_type: i32,
    /// Option for export offload.
    #[prost(message, optional, tag = "8")]
    pub offload: ::core::option::Option<bool>,
}
/// Nested message and enum types in `ExportContext`.
pub mod export_context {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SqlCsvExportOptions {
        /// The select query used to extract the data.
        #[prost(string, tag = "1")]
        pub select_query: ::prost::alloc::string::String,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SqlExportOptions {
        /// Tables to export, or that were exported, from the specified database. If
        /// you specify tables, specify one and only one database. For PostgreSQL
        /// instances, you can specify only one table.
        #[prost(string, repeated, tag = "1")]
        pub tables: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// Export only schemas.
        #[prost(message, optional, tag = "2")]
        pub schema_only: ::core::option::Option<bool>,
        #[prost(message, optional, tag = "3")]
        pub mysql_export_options: ::core::option::Option<sql_export_options::MysqlExportOptions>,
    }
    /// Nested message and enum types in `SqlExportOptions`.
    pub mod sql_export_options {
        /// Options for exporting from MySQL.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct MysqlExportOptions {
            /// Option to include SQL statement required to set up replication.
            /// <br>If set to <b>1</b>, the dump file includes
            ///  a CHANGE MASTER TO statement with the binary log coordinates,
            ///  and --set-gtid-purged is set to ON.
            /// <br>If set to <b>2</b>, the CHANGE MASTER TO statement is written as
            ///  a SQL comment and has no effect.
            /// <br>If set to any value other than <b>1</b>, --set-gtid-purged is set
            /// to OFF.
            #[prost(message, optional, tag = "1")]
            pub master_data: ::core::option::Option<i32>,
        }
    }
}
/// Database instance failover context.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FailoverContext {
    /// The current settings version of this instance. Request will be rejected if
    /// this version doesn't match the current settings version.
    #[prost(int64, tag = "1")]
    pub settings_version: i64,
    /// This is always <b>sql#failoverContext</b>.
    #[prost(string, tag = "2")]
    pub kind: ::prost::alloc::string::String,
}
/// A flag resource.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Flag {
    /// This is the name of the flag. Flag names always use underscores, not
    /// hyphens, for example: <b>max_allowed_packet</b>
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The type of the flag. Flags are typed to being <b>BOOLEAN</b>,
    /// <b>STRING</b>, <b>INTEGER</b> or <b>NONE</b>.
    /// <b>NONE</b> is used for flags which do not take a value, such as
    /// <b>skip_grant_tables</b>.
    #[prost(enumeration = "SqlFlagType", tag = "2")]
    pub r#type: i32,
    /// The database version this flag applies to. Can be <b>MYSQL_8_0</b>,
    /// <b>MYSQL_5_6</b>, or <b>MYSQL_5_7</b>.
    #[prost(enumeration = "SqlDatabaseVersion", repeated, tag = "3")]
    pub applies_to: ::prost::alloc::vec::Vec<i32>,
    /// For <b>STRING</b> flags, a list of strings that the value can be set
    /// to.
    #[prost(string, repeated, tag = "4")]
    pub allowed_string_values: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// For <b>INTEGER</b> flags, the minimum allowed value.
    #[prost(message, optional, tag = "5")]
    pub min_value: ::core::option::Option<i64>,
    /// For <b>INTEGER</b> flags, the maximum allowed value.
    #[prost(message, optional, tag = "6")]
    pub max_value: ::core::option::Option<i64>,
    /// Indicates whether changing this flag will trigger a database restart. Only
    /// applicable to Second Generation instances.
    #[prost(message, optional, tag = "7")]
    pub requires_restart: ::core::option::Option<bool>,
    /// This is always <b>sql#flag</b>.
    #[prost(string, tag = "8")]
    pub kind: ::prost::alloc::string::String,
    /// Whether or not the flag is considered in beta.
    #[prost(message, optional, tag = "9")]
    pub in_beta: ::core::option::Option<bool>,
    /// Use this field if only certain integers are accepted. Can be combined
    /// with min_value and max_value to add additional values.
    #[prost(int64, repeated, tag = "10")]
    pub allowed_int_values: ::prost::alloc::vec::Vec<i64>,
}
/// Flags list response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FlagsListResponse {
    /// This is always <b>sql#flagsList</b>.
    #[prost(string, tag = "1")]
    pub kind: ::prost::alloc::string::String,
    /// List of flags.
    #[prost(message, repeated, tag = "2")]
    pub items: ::prost::alloc::vec::Vec<Flag>,
}
/// Database instance import context.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportContext {
    /// Path to the import file in Cloud Storage, in the form
    /// <b>gs://bucketName/fileName</b>. Compressed gzip files (.gz) are supported
    /// when <b>fileType</b> is <b>SQL</b>. The instance must have
    /// write permissions to the bucket and read access to the file.
    #[prost(string, tag = "1")]
    pub uri: ::prost::alloc::string::String,
    /// The target database for the import. If <b>fileType</b> is
    /// <b>SQL</b>, this field is required only if the import file does not
    /// specify a database, and is overridden by any database specification in the
    /// import file. If <b>fileType</b> is <b>CSV</b>, one database
    /// must be specified.
    #[prost(string, tag = "2")]
    pub database: ::prost::alloc::string::String,
    /// This is always <b>sql#importContext</b>.
    #[prost(string, tag = "3")]
    pub kind: ::prost::alloc::string::String,
    /// The file type for the specified uri. <br><b>SQL</b>: The file
    /// contains SQL statements. <br><b>CSV</b>: The file contains CSV data.
    #[prost(enumeration = "SqlFileType", tag = "4")]
    pub file_type: i32,
    /// Options for importing data as CSV.
    #[prost(message, optional, tag = "5")]
    pub csv_import_options: ::core::option::Option<import_context::SqlCsvImportOptions>,
    /// The PostgreSQL user for this import operation. PostgreSQL instances only.
    #[prost(string, tag = "6")]
    pub import_user: ::prost::alloc::string::String,
    /// Import parameters specific to SQL Server .BAK files
    #[prost(message, optional, tag = "7")]
    pub bak_import_options: ::core::option::Option<import_context::SqlBakImportOptions>,
}
/// Nested message and enum types in `ImportContext`.
pub mod import_context {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SqlCsvImportOptions {
        /// The table to which CSV data is imported.
        #[prost(string, tag = "1")]
        pub table: ::prost::alloc::string::String,
        /// The columns to which CSV data is imported. If not specified, all columns
        /// of the database table are loaded with CSV data.
        #[prost(string, repeated, tag = "2")]
        pub columns: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SqlBakImportOptions {
        #[prost(message, optional, tag = "1")]
        pub encryption_options: ::core::option::Option<sql_bak_import_options::EncryptionOptions>,
    }
    /// Nested message and enum types in `SqlBakImportOptions`.
    pub mod sql_bak_import_options {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct EncryptionOptions {
            /// Path to the Certificate (.cer) in Cloud Storage, in the form
            /// <b>gs://bucketName/fileName</b>. The instance must have
            /// write permissions to the bucket and read access to the file.
            #[prost(string, tag = "1")]
            pub cert_path: ::prost::alloc::string::String,
            /// Path to the Certificate Private Key (.pvk)  in Cloud Storage, in the
            /// form <b>gs://bucketName/fileName</b>. The instance must have
            /// write permissions to the bucket and read access to the file.
            #[prost(string, tag = "2")]
            pub pvk_path: ::prost::alloc::string::String,
            /// Password that encrypts the private key
            #[prost(string, tag = "3")]
            pub pvk_password: ::prost::alloc::string::String,
        }
    }
}
/// Database instance clone request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InstancesCloneRequest {
    /// Contains details about the clone operation.
    #[prost(message, optional, tag = "1")]
    pub clone_context: ::core::option::Option<CloneContext>,
}
/// Database demote primary instance request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InstancesDemoteMasterRequest {
    /// Contains details about the demoteMaster operation.
    #[prost(message, optional, tag = "1")]
    pub demote_master_context: ::core::option::Option<DemoteMasterContext>,
}
/// Database instance export request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InstancesExportRequest {
    /// Contains details about the export operation.
    #[prost(message, optional, tag = "1")]
    pub export_context: ::core::option::Option<ExportContext>,
}
/// Instance failover request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InstancesFailoverRequest {
    /// Failover Context.
    #[prost(message, optional, tag = "1")]
    pub failover_context: ::core::option::Option<FailoverContext>,
}
/// Database instance import request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InstancesImportRequest {
    /// Contains details about the import operation.
    #[prost(message, optional, tag = "1")]
    pub import_context: ::core::option::Option<ImportContext>,
}
/// Database instances list response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InstancesListResponse {
    /// This is always <b>sql#instancesList</b>.
    #[prost(string, tag = "1")]
    pub kind: ::prost::alloc::string::String,
    /// List of warnings that occurred while handling the request.
    #[prost(message, repeated, tag = "2")]
    pub warnings: ::prost::alloc::vec::Vec<ApiWarning>,
    /// List of database instance resources.
    #[prost(message, repeated, tag = "3")]
    pub items: ::prost::alloc::vec::Vec<DatabaseInstance>,
    /// The continuation token, used to page through large result sets. Provide
    /// this value in a subsequent request to return the next page of results.
    #[prost(string, tag = "4")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Instances ListServerCas response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InstancesListServerCasResponse {
    /// List of server CA certificates for the instance.
    #[prost(message, repeated, tag = "1")]
    pub certs: ::prost::alloc::vec::Vec<SslCert>,
    #[prost(string, tag = "2")]
    pub active_version: ::prost::alloc::string::String,
    /// This is always <b>sql#instancesListServerCas</b>.
    #[prost(string, tag = "3")]
    pub kind: ::prost::alloc::string::String,
}
/// Database instance restore backup request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InstancesRestoreBackupRequest {
    /// Parameters required to perform the restore backup operation.
    #[prost(message, optional, tag = "1")]
    pub restore_backup_context: ::core::option::Option<RestoreBackupContext>,
}
/// Rotate Server CA request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InstancesRotateServerCaRequest {
    /// Contains details about the rotate server CA operation.
    #[prost(message, optional, tag = "1")]
    pub rotate_server_ca_context: ::core::option::Option<RotateServerCaContext>,
}
/// Instance truncate log request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InstancesTruncateLogRequest {
    /// Contains details about the truncate log operation.
    #[prost(message, optional, tag = "1")]
    pub truncate_log_context: ::core::option::Option<TruncateLogContext>,
}
/// Instance verify external sync settings response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SqlInstancesVerifyExternalSyncSettingsResponse {
    /// This is always <b>sql#migrationSettingErrorList</b>.
    #[prost(string, tag = "1")]
    pub kind: ::prost::alloc::string::String,
    /// List of migration violations.
    #[prost(message, repeated, tag = "2")]
    pub errors: ::prost::alloc::vec::Vec<SqlExternalSyncSettingError>,
    /// List of migration warnings.
    #[prost(message, repeated, tag = "3")]
    pub warnings: ::prost::alloc::vec::Vec<SqlExternalSyncSettingError>,
}
/// External primary instance migration setting error/warning.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SqlExternalSyncSettingError {
    /// Can be <b>sql#externalSyncSettingError</b> or
    /// <b>sql#externalSyncSettingWarning</b>.
    #[prost(string, tag = "1")]
    pub kind: ::prost::alloc::string::String,
    /// Identifies the specific error that occurred.
    #[prost(
        enumeration = "sql_external_sync_setting_error::SqlExternalSyncSettingErrorType",
        tag = "2"
    )]
    pub r#type: i32,
    /// Additional information about the error encountered.
    #[prost(string, tag = "3")]
    pub detail: ::prost::alloc::string::String,
}
/// Nested message and enum types in `SqlExternalSyncSettingError`.
pub mod sql_external_sync_setting_error {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum SqlExternalSyncSettingErrorType {
        Unspecified = 0,
        ConnectionFailure = 1,
        BinlogNotEnabled = 2,
        IncompatibleDatabaseVersion = 3,
        ReplicaAlreadySetup = 4,
        InsufficientPrivilege = 5,
        /// Unsupported migration type.
        UnsupportedMigrationType = 6,
        /// No pglogical extension installed on databases, applicable for postgres.
        NoPglogicalInstalled = 7,
        /// pglogical node already exists on databases, applicable for postgres.
        PglogicalNodeAlreadyExists = 8,
        /// The value of parameter wal_level is not set to logical.
        InvalidWalLevel = 9,
        /// The value of parameter shared_preload_libraries does not include
        /// pglogical.
        InvalidSharedPreloadLibrary = 10,
        /// The value of parameter max_replication_slots is not sufficient.
        InsufficientMaxReplicationSlots = 11,
        /// The value of parameter max_wal_senders is not sufficient.
        InsufficientMaxWalSenders = 12,
        /// The value of parameter max_worker_processes is not sufficient.
        InsufficientMaxWorkerProcesses = 13,
        /// Extensions installed are either not supported or having unsupported
        /// versions
        UnsupportedExtensions = 14,
        /// The value of parameter rds.logical_replication is not set to 1.
        InvalidRdsLogicalReplication = 15,
        /// The primary instance logging setup doesn't allow EM sync.
        InvalidLoggingSetup = 16,
        /// The primary instance database parameter setup doesn't allow EM sync.
        InvalidDbParam = 17,
        /// The gtid_mode is not supported, applicable for MySQL.
        UnsupportedGtidMode = 18,
        /// SQL Server Agent is not running.
        SqlserverAgentNotRunning = 19,
        /// The table definition is not support due to missing primary key or replica
        /// identity, applicable for postgres.
        UnsupportedTableDefinition = 20,
        /// The customer has a definer that will break EM setup.
        UnsupportedDefiner = 21,
    }
}
/// IP Management configuration.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IpConfiguration {
    /// Whether the instance is assigned a public IP address or not.
    #[prost(message, optional, tag = "1")]
    pub ipv4_enabled: ::core::option::Option<bool>,
    /// The resource link for the VPC network from which the Cloud SQL instance is
    /// accessible for private IP. For example,
    /// <b>/projects/myProject/global/networks/default</b>. This setting can
    /// be updated, but it cannot be removed after it is set.
    #[prost(string, tag = "2")]
    pub private_network: ::prost::alloc::string::String,
    /// Whether SSL connections over IP are enforced or not.
    #[prost(message, optional, tag = "3")]
    pub require_ssl: ::core::option::Option<bool>,
    /// The list of external networks that are allowed to connect to the instance
    /// using the IP. In 'CIDR' notation, also known as 'slash' notation (for
    /// example: <b>192.168.100.0/24</b>).
    #[prost(message, repeated, tag = "4")]
    pub authorized_networks: ::prost::alloc::vec::Vec<AclEntry>,
}
/// Database instance IP Mapping.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IpMapping {
    /// The type of this IP address. A <b>PRIMARY</b> address is a public
    /// address that can accept incoming connections. A <b>PRIVATE</b>
    /// address is a private address that can accept incoming connections. An
    /// <b>OUTGOING</b> address is the source address of connections
    /// originating from the instance, if supported.
    #[prost(enumeration = "SqlIpAddressType", tag = "1")]
    pub r#type: i32,
    /// The IP address assigned.
    #[prost(string, tag = "2")]
    pub ip_address: ::prost::alloc::string::String,
    /// The due time for this IP to be retired in <a
    /// href="<https://tools.ietf.org/html/rfc3339">RFC> 3339</a> format, for example
    /// <b>2012-11-15T16:19:00.094Z</b>. This field is only available when
    /// the IP is scheduled to be retired.
    #[prost(message, optional, tag = "3")]
    pub time_to_retire: ::core::option::Option<::prost_types::Timestamp>,
}
/// Preferred location. This specifies where a Cloud SQL instance
/// is located, either in a specific Compute Engine zone, or
/// co-located with an App Engine application. Note that if the preferred
/// location is not available, the instance will be located as close as possible
/// within the region. Only one location may be specified.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LocationPreference {
    /// The App Engine application to follow, it must be in the same region as the
    /// Cloud SQL instance.
    #[deprecated]
    #[prost(string, tag = "1")]
    pub follow_gae_application: ::prost::alloc::string::String,
    /// The preferred Compute Engine zone (for example: us-central1-a,
    /// us-central1-b, etc.).
    #[prost(string, tag = "2")]
    pub zone: ::prost::alloc::string::String,
    /// The preferred Compute Engine zone for the secondary/failover
    /// (for example: us-central1-a, us-central1-b, etc.).
    /// Reserved for future use.
    #[prost(string, tag = "4")]
    pub secondary_zone: ::prost::alloc::string::String,
    /// This is always <b>sql#locationPreference</b>.
    #[prost(string, tag = "3")]
    pub kind: ::prost::alloc::string::String,
}
/// Maintenance window. This specifies when a Cloud SQL instance
/// is restarted for system maintenance purposes.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MaintenanceWindow {
    /// hour of day - 0 to 23.
    #[prost(message, optional, tag = "1")]
    pub hour: ::core::option::Option<i32>,
    /// day of week (1-7), starting on Monday.
    #[prost(message, optional, tag = "2")]
    pub day: ::core::option::Option<i32>,
    /// Maintenance timing setting: <b>canary</b> (Earlier) or
    /// <b>stable</b> (Later). <br /><a
    /// href="/sql/docs/db_path/instance-settings#maintenance-timing-2ndgen">
    /// Learn more</a>.
    #[prost(enumeration = "SqlUpdateTrack", tag = "3")]
    pub update_track: i32,
    /// This is always <b>sql#maintenanceWindow</b>.
    #[prost(string, tag = "4")]
    pub kind: ::prost::alloc::string::String,
}
/// Deny Maintenance Periods. This specifies a date range during when all CSA
/// rollout will be denied.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DenyMaintenancePeriod {
    /// "deny maintenance period" start date. If the year of the start date is
    /// empty, the year of the end date also must be empty. In this case, it means
    /// the deny maintenance period recurs every year. The date is in format
    /// yyyy-mm-dd i.e., 2020-11-01, or mm-dd, i.e., 11-01
    #[prost(string, tag = "1")]
    pub start_date: ::prost::alloc::string::String,
    /// "deny maintenance period" end date. If the year of the end date is empty,
    /// the year of the start date also must be empty. In this case, it means the
    /// deny maintenance period recurs every year. The date is in format yyyy-mm-dd
    /// i.e., 2020-11-01, or mm-dd, i.e., 11-01
    #[prost(string, tag = "2")]
    pub end_date: ::prost::alloc::string::String,
    /// Time in UTC when the "deny maintenance period" starts on start_date and
    /// ends on end_date. The time is in format: HH:mm:SS, i.e., 00:00:00
    #[prost(string, tag = "3")]
    pub time: ::prost::alloc::string::String,
}
/// Insights configuration. This specifies when Cloud SQL Insights feature is
/// enabled and optional configuration.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InsightsConfig {
    /// Whether Query Insights feature is enabled.
    #[prost(bool, tag = "1")]
    pub query_insights_enabled: bool,
    /// Whether Query Insights will record client address when enabled.
    #[prost(bool, tag = "2")]
    pub record_client_address: bool,
    /// Whether Query Insights will record application tags from query when
    /// enabled.
    #[prost(bool, tag = "3")]
    pub record_application_tags: bool,
    /// Maximum query length stored in bytes. Default value: 1024 bytes.
    /// Range: 256-4500 bytes. Query length more than this field value will be
    /// truncated to this value. When unset, query length will be the default
    /// value. Changing query length will restart the database.
    #[prost(message, optional, tag = "4")]
    pub query_string_length: ::core::option::Option<i32>,
    /// Number of query plans generated by Insights per minute. Default is 5.
    /// Changing this will restart the database.
    #[prost(message, optional, tag = "5")]
    pub query_plans_per_minute: ::core::option::Option<i32>,
}
/// Read-replica configuration specific to MySQL databases.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MySqlReplicaConfiguration {
    /// Path to a SQL dump file in Google Cloud Storage from which the replica
    /// instance is to be created. The URI is in the form gs://bucketName/fileName.
    /// Compressed gzip files (.gz) are also supported.
    /// Dumps have the binlog co-ordinates from which replication
    /// begins. This can be accomplished by setting --master-data to 1 when using
    /// mysqldump.
    #[prost(string, tag = "1")]
    pub dump_file_path: ::prost::alloc::string::String,
    /// The username for the replication connection.
    #[prost(string, tag = "2")]
    pub username: ::prost::alloc::string::String,
    /// The password for the replication connection.
    #[prost(string, tag = "3")]
    pub password: ::prost::alloc::string::String,
    /// Seconds to wait between connect retries. MySQL's default is 60 seconds.
    #[prost(message, optional, tag = "4")]
    pub connect_retry_interval: ::core::option::Option<i32>,
    /// Interval in milliseconds between replication heartbeats.
    #[prost(message, optional, tag = "5")]
    pub master_heartbeat_period: ::core::option::Option<i64>,
    /// PEM representation of the trusted CA's x509 certificate.
    #[prost(string, tag = "6")]
    pub ca_certificate: ::prost::alloc::string::String,
    /// PEM representation of the replica's x509 certificate.
    #[prost(string, tag = "7")]
    pub client_certificate: ::prost::alloc::string::String,
    /// PEM representation of the replica's private key. The corresponsing public
    /// key is encoded in the client's certificate.
    #[prost(string, tag = "8")]
    pub client_key: ::prost::alloc::string::String,
    /// A list of permissible ciphers to use for SSL encryption.
    #[prost(string, tag = "9")]
    pub ssl_cipher: ::prost::alloc::string::String,
    /// Whether or not to check the primary instance's Common Name value in the
    /// certificate that it sends during the SSL handshake.
    #[prost(message, optional, tag = "10")]
    pub verify_server_certificate: ::core::option::Option<bool>,
    /// This is always <b>sql#mysqlReplicaConfiguration</b>.
    #[prost(string, tag = "11")]
    pub kind: ::prost::alloc::string::String,
}
/// On-premises instance configuration.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OnPremisesConfiguration {
    /// The host and port of the on-premises instance in host:port format
    #[prost(string, tag = "1")]
    pub host_port: ::prost::alloc::string::String,
    /// This is always <b>sql#onPremisesConfiguration</b>.
    #[prost(string, tag = "2")]
    pub kind: ::prost::alloc::string::String,
    /// The username for connecting to on-premises instance.
    #[prost(string, tag = "3")]
    pub username: ::prost::alloc::string::String,
    /// The password for connecting to on-premises instance.
    #[prost(string, tag = "4")]
    pub password: ::prost::alloc::string::String,
    /// PEM representation of the trusted CA's x509 certificate.
    #[prost(string, tag = "5")]
    pub ca_certificate: ::prost::alloc::string::String,
    /// PEM representation of the replica's x509 certificate.
    #[prost(string, tag = "6")]
    pub client_certificate: ::prost::alloc::string::String,
    /// PEM representation of the replica's private key. The corresponsing public
    /// key is encoded in the client's certificate.
    #[prost(string, tag = "7")]
    pub client_key: ::prost::alloc::string::String,
    /// The dump file to create the Cloud SQL replica.
    #[prost(string, tag = "8")]
    pub dump_file_path: ::prost::alloc::string::String,
}
/// Disk encryption configuration for an instance.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DiskEncryptionConfiguration {
    /// Resource name of KMS key for disk encryption
    #[prost(string, tag = "1")]
    pub kms_key_name: ::prost::alloc::string::String,
    /// This is always <b>sql#diskEncryptionConfiguration</b>.
    #[prost(string, tag = "2")]
    pub kind: ::prost::alloc::string::String,
}
/// Disk encryption status for an instance.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DiskEncryptionStatus {
    /// KMS key version used to encrypt the Cloud SQL instance resource
    #[prost(string, tag = "1")]
    pub kms_key_version_name: ::prost::alloc::string::String,
    /// This is always <b>sql#diskEncryptionStatus</b>.
    #[prost(string, tag = "2")]
    pub kind: ::prost::alloc::string::String,
}
/// An Operation resource.&nbsp;For successful operations that return an
/// Operation resource, only the fields relevant to the operation are populated
/// in the resource.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Operation {
    /// This is always <b>sql#operation</b>.
    #[prost(string, tag = "1")]
    pub kind: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub target_link: ::prost::alloc::string::String,
    /// The status of an operation. Valid values are:
    /// <br><b>PENDING</b>
    /// <br><b>RUNNING</b>
    /// <br><b>DONE</b>
    /// <br><b>SQL_OPERATION_STATUS_UNSPECIFIED</b>
    #[prost(enumeration = "operation::SqlOperationStatus", tag = "3")]
    pub status: i32,
    /// The email address of the user who initiated this operation.
    #[prost(string, tag = "4")]
    pub user: ::prost::alloc::string::String,
    /// The time this operation was enqueued in UTC timezone in <a
    /// href="<https://tools.ietf.org/html/rfc3339">RFC> 3339</a> format, for example
    /// <b>2012-11-15T16:19:00.094Z</b>.
    #[prost(message, optional, tag = "5")]
    pub insert_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The time this operation actually started in UTC timezone in <a
    /// href="<https://tools.ietf.org/html/rfc3339">RFC> 3339</a> format, for example
    /// <b>2012-11-15T16:19:00.094Z</b>.
    #[prost(message, optional, tag = "6")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The time this operation finished in UTC timezone in <a
    /// href="<https://tools.ietf.org/html/rfc3339">RFC> 3339</a> format, for example
    /// <b>2012-11-15T16:19:00.094Z</b>.
    #[prost(message, optional, tag = "7")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// If errors occurred during processing of this operation, this field will be
    /// populated.
    #[prost(message, optional, tag = "8")]
    pub error: ::core::option::Option<OperationErrors>,
    /// The type of the operation. Valid values are:
    /// <br><b>CREATE</b>
    /// <br><b>DELETE</b>
    /// <br><b>UPDATE</b>
    /// <br><b>RESTART</b>
    /// <br><b>IMPORT</b>
    /// <br><b>EXPORT</b>
    /// <br><b>BACKUP_VOLUME</b>
    /// <br><b>RESTORE_VOLUME</b>
    /// <br><b>CREATE_USER</b>
    /// <br><b>DELETE_USER</b>
    /// <br><b>CREATE_DATABASE</b>
    /// <br><b>DELETE_DATABASE</b>
    #[prost(enumeration = "operation::SqlOperationType", tag = "9")]
    pub operation_type: i32,
    /// The context for import operation, if applicable.
    #[prost(message, optional, tag = "10")]
    pub import_context: ::core::option::Option<ImportContext>,
    /// The context for export operation, if applicable.
    #[prost(message, optional, tag = "11")]
    pub export_context: ::core::option::Option<ExportContext>,
    /// The context for backup operation, if applicable.
    #[prost(message, optional, tag = "17")]
    pub backup_context: ::core::option::Option<BackupContext>,
    /// An identifier that uniquely identifies the operation. You can use this
    /// identifier to retrieve the Operations resource that has information about
    /// the operation.
    #[prost(string, tag = "12")]
    pub name: ::prost::alloc::string::String,
    /// Name of the database instance related to this operation.
    #[prost(string, tag = "13")]
    pub target_id: ::prost::alloc::string::String,
    /// The URI of this resource.
    #[prost(string, tag = "14")]
    pub self_link: ::prost::alloc::string::String,
    /// The project ID of the target instance related to this operation.
    #[prost(string, tag = "15")]
    pub target_project: ::prost::alloc::string::String,
}
/// Nested message and enum types in `Operation`.
pub mod operation {
    /// The type of Cloud SQL operation.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum SqlOperationType {
        /// Unknown operation type.
        Unspecified = 0,
        /// Imports data into a Cloud SQL instance.
        Import = 1,
        /// Exports data from a Cloud SQL instance to a Cloud Storage
        /// bucket.
        Export = 2,
        /// Creates a new Cloud SQL instance.
        Create = 3,
        /// Updates the settings of a Cloud SQL instance.
        Update = 4,
        /// Deletes a Cloud SQL instance.
        Delete = 5,
        /// Restarts the Cloud SQL instance.
        Restart = 6,
        Backup = 7,
        Snapshot = 8,
        /// Performs instance backup.
        BackupVolume = 9,
        /// Deletes an instance backup.
        DeleteVolume = 10,
        /// Restores an instance backup.
        RestoreVolume = 11,
        /// Injects a privileged user in mysql for MOB instances.
        InjectUser = 12,
        /// Clones a Cloud SQL instance.
        Clone = 14,
        /// Stops replication on a Cloud SQL read replica instance.
        StopReplica = 15,
        /// Starts replication on a Cloud SQL read replica instance.
        StartReplica = 16,
        /// Promotes a Cloud SQL replica instance.
        PromoteReplica = 17,
        /// Creates a Cloud SQL replica instance.
        CreateReplica = 18,
        /// Creates a new user in a Cloud SQL instance.
        CreateUser = 19,
        /// Deletes a user from a Cloud SQL instance.
        DeleteUser = 20,
        /// Updates an existing user in a Cloud SQL instance.
        UpdateUser = 21,
        /// Creates a database in the Cloud SQL instance.
        CreateDatabase = 22,
        /// Deletes a database in the Cloud SQL instance.
        DeleteDatabase = 23,
        /// Updates a database in the Cloud SQL instance.
        UpdateDatabase = 24,
        /// Performs failover of an HA-enabled Cloud SQL
        /// failover replica.
        Failover = 25,
        /// Deletes the backup taken by a backup run.
        DeleteBackup = 26,
        RecreateReplica = 27,
        /// Truncates a general or slow log table in MySQL.
        TruncateLog = 28,
        /// Demotes the stand-alone instance to be a Cloud SQL
        /// read replica for an external database server.
        DemoteMaster = 29,
        /// Indicates that the instance is currently in maintenance. Maintenance
        /// typically causes the instance to be unavailable for 1-3 minutes.
        Maintenance = 30,
        /// This field is deprecated, and will be removed in future version of API.
        EnablePrivateIp = 31,
        DeferMaintenance = 32,
        /// Creates clone instance.
        CreateClone = 33,
        /// Reschedule maintenance to another time.
        RescheduleMaintenance = 34,
        /// Starts external sync of a Cloud SQL EM replica to an external primary
        /// instance.
        StartExternalSync = 35,
    }
    /// The status of an operation.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum SqlOperationStatus {
        /// The state of the operation is unknown.
        Unspecified = 0,
        /// The operation has been queued, but has not started yet.
        Pending = 1,
        /// The operation is running.
        Running = 2,
        /// The operation completed.
        Done = 3,
    }
}
/// Database instance operation error.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationError {
    /// This is always <b>sql#operationError</b>.
    #[prost(string, tag = "1")]
    pub kind: ::prost::alloc::string::String,
    /// Identifies the specific error that occurred.
    #[prost(string, tag = "2")]
    pub code: ::prost::alloc::string::String,
    /// Additional information about the error encountered.
    #[prost(string, tag = "3")]
    pub message: ::prost::alloc::string::String,
}
/// Database instance operation errors list wrapper.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationErrors {
    /// This is always <b>sql#operationErrors</b>.
    #[prost(string, tag = "1")]
    pub kind: ::prost::alloc::string::String,
    /// The list of errors encountered while processing this operation.
    #[prost(message, repeated, tag = "2")]
    pub errors: ::prost::alloc::vec::Vec<OperationError>,
}
/// Database instance list operations response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationsListResponse {
    /// This is always <b>sql#operationsList</b>.
    #[prost(string, tag = "1")]
    pub kind: ::prost::alloc::string::String,
    /// List of operation resources.
    #[prost(message, repeated, tag = "2")]
    pub items: ::prost::alloc::vec::Vec<Operation>,
    /// The continuation token, used to page through large result sets. Provide
    /// this value in a subsequent request to return the next page of results.
    #[prost(string, tag = "3")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Read-replica configuration for connecting to the primary instance.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReplicaConfiguration {
    /// This is always <b>sql#replicaConfiguration</b>.
    #[prost(string, tag = "1")]
    pub kind: ::prost::alloc::string::String,
    /// MySQL specific configuration when replicating from a MySQL on-premises
    /// primary instance. Replication configuration information such as the
    /// username, password, certificates, and keys are not stored in the instance
    /// metadata. The configuration information is used only to set up the
    /// replication connection and is stored by MySQL in a file named
    /// <b>master.info</b> in the data directory.
    #[prost(message, optional, tag = "2")]
    pub mysql_replica_configuration: ::core::option::Option<MySqlReplicaConfiguration>,
    /// Specifies if the replica is the failover target. If the field is set to
    /// <b>true</b> the replica will be designated as a failover replica. In
    /// case the primary instance fails, the replica instance will be promoted as
    /// the new primary instance.  <p>Only one replica can be specified as failover
    /// target, and the replica has to be in different zone with the primary
    /// instance.
    #[prost(message, optional, tag = "3")]
    pub failover_target: ::core::option::Option<bool>,
}
/// Database instance restore from backup context.
/// Backup context contains source instance id and project id.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RestoreBackupContext {
    /// This is always <b>sql#restoreBackupContext</b>.
    #[prost(string, tag = "1")]
    pub kind: ::prost::alloc::string::String,
    /// The ID of the backup run to restore from.
    #[prost(int64, tag = "2")]
    pub backup_run_id: i64,
    /// The ID of the instance that the backup was taken from.
    #[prost(string, tag = "3")]
    pub instance_id: ::prost::alloc::string::String,
    /// The full project ID of the source instance.
    #[prost(string, tag = "4")]
    pub project: ::prost::alloc::string::String,
}
/// Instance rotate server CA context.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RotateServerCaContext {
    /// This is always <b>sql#rotateServerCaContext</b>.
    #[prost(string, tag = "1")]
    pub kind: ::prost::alloc::string::String,
    /// The fingerprint of the next version to be rotated to. If left unspecified,
    /// will be rotated to the most recently added server CA version.
    #[prost(string, tag = "2")]
    pub next_version: ::prost::alloc::string::String,
}
/// Database instance settings.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Settings {
    /// The version of instance settings. This is a required field for update
    /// method to make sure concurrent updates are handled properly. During update,
    /// use the most recent settingsVersion value for this instance and do not try
    /// to update this value.
    #[prost(message, optional, tag = "1")]
    pub settings_version: ::core::option::Option<i64>,
    /// The App Engine app IDs that can access this instance.
    /// (Deprecated) Applied to First Generation instances only.
    #[deprecated]
    #[prost(string, repeated, tag = "2")]
    pub authorized_gae_applications: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The tier (or machine type) for this instance, for example
    /// <b>db-custom-1-3840</b>.
    #[prost(string, tag = "3")]
    pub tier: ::prost::alloc::string::String,
    /// This is always <b>sql#settings</b>.
    #[prost(string, tag = "4")]
    pub kind: ::prost::alloc::string::String,
    /// User-provided labels, represented as a dictionary where each label is a
    /// single key value pair.
    #[prost(map = "string, string", tag = "5")]
    pub user_labels:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Availability type. Potential values:
    ///   <br><b>ZONAL</b>: The instance serves data from only one zone.
    /// Outages in that zone affect data accessibility.
    ///   <br><b>REGIONAL</b>: The instance can serve data from more than one zone
    ///   in a region
    /// (it is highly available). <br>For more information, see
    /// <a href="/sql/docs/postgres/high-availability">
    /// Overview of the High Availability Configuration</a>.
    #[prost(enumeration = "SqlAvailabilityType", tag = "6")]
    pub availability_type: i32,
    /// The pricing plan for this instance. This can be either <b>PER_USE</b>
    /// or <b>PACKAGE</b>. Only <b>PER_USE</b> is supported for Second
    /// Generation instances.
    #[prost(enumeration = "SqlPricingPlan", tag = "7")]
    pub pricing_plan: i32,
    /// The type of replication this instance uses. This can be either
    /// <b>ASYNCHRONOUS</b> or <b>SYNCHRONOUS</b>.
    /// (Deprecated_ This property was only applicable to
    /// First Generation instances.
    #[deprecated]
    #[prost(enumeration = "SqlReplicationType", tag = "8")]
    pub replication_type: i32,
    /// The maximum size to which storage capacity can be automatically increased.
    /// The default value is 0, which specifies that there is no limit.
    #[prost(message, optional, tag = "9")]
    pub storage_auto_resize_limit: ::core::option::Option<i64>,
    /// The activation policy specifies when the instance is activated; it is
    /// applicable only when the instance state is RUNNABLE. Valid values:
    ///   <br><b>ALWAYS</b>: The instance is on, and remains so even in
    /// the absence of connection requests.
    ///   <br><b>NEVER</b>: The instance is off; it is not activated, even if a
    ///   connection request arrives.
    #[prost(enumeration = "settings::SqlActivationPolicy", tag = "10")]
    pub activation_policy: i32,
    /// The settings for IP Management. This allows to enable or disable the
    /// instance IP and manage which external networks can connect to the instance.
    /// The IPv4 address cannot be disabled for Second Generation instances.
    #[prost(message, optional, tag = "11")]
    pub ip_configuration: ::core::option::Option<IpConfiguration>,
    /// Configuration to increase storage size automatically. The default value is
    /// true.
    #[prost(message, optional, tag = "12")]
    pub storage_auto_resize: ::core::option::Option<bool>,
    /// The location preference settings. This allows the instance to be located as
    /// near as possible to either an App Engine app or Compute Engine zone for
    /// better performance. App Engine co-location was only applicable to First
    /// Generation instances.
    #[prost(message, optional, tag = "13")]
    pub location_preference: ::core::option::Option<LocationPreference>,
    /// The database flags passed to the instance at startup.
    #[prost(message, repeated, tag = "14")]
    pub database_flags: ::prost::alloc::vec::Vec<DatabaseFlags>,
    /// The type of data disk: PD_SSD (default) or
    /// PD_HDD. Not used for First Generation instances.
    #[prost(enumeration = "SqlDataDiskType", tag = "15")]
    pub data_disk_type: i32,
    /// The maintenance window for this instance. This specifies when the instance
    /// can be restarted for maintenance purposes.
    #[prost(message, optional, tag = "16")]
    pub maintenance_window: ::core::option::Option<MaintenanceWindow>,
    /// The daily backup configuration for the instance.
    #[prost(message, optional, tag = "17")]
    pub backup_configuration: ::core::option::Option<BackupConfiguration>,
    /// Configuration specific to read replica instances. Indicates whether
    /// replication is enabled or not.
    #[prost(message, optional, tag = "18")]
    pub database_replication_enabled: ::core::option::Option<bool>,
    /// Configuration specific to read replica instances. Indicates whether
    /// database flags for crash-safe replication are enabled. This property was
    /// only applicable to First Generation instances.
    #[deprecated]
    #[prost(message, optional, tag = "19")]
    pub crash_safe_replication_enabled: ::core::option::Option<bool>,
    /// The size of data disk, in GB. The data disk size minimum is 10GB.
    #[prost(message, optional, tag = "20")]
    pub data_disk_size_gb: ::core::option::Option<i64>,
    /// Active Directory configuration, relevant only for Cloud SQL for SQL Server.
    #[prost(message, optional, tag = "22")]
    pub active_directory_config: ::core::option::Option<SqlActiveDirectoryConfig>,
    /// The name of server Instance collation.
    #[prost(string, tag = "23")]
    pub collation: ::prost::alloc::string::String,
    /// Deny maintenance periods
    #[prost(message, repeated, tag = "24")]
    pub deny_maintenance_periods: ::prost::alloc::vec::Vec<DenyMaintenancePeriod>,
    /// Insights configuration, for now relevant only for Postgres.
    #[prost(message, optional, tag = "25")]
    pub insights_config: ::core::option::Option<InsightsConfig>,
}
/// Nested message and enum types in `Settings`.
pub mod settings {
    /// Specifies when the instance is activated.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum SqlActivationPolicy {
        /// Unknown activation plan.
        Unspecified = 0,
        /// The instance is always up and running.
        Always = 1,
        /// The instance never starts.
        Never = 2,
        /// The instance starts upon receiving requests.
        OnDemand = 3,
    }
}
/// SslCerts Resource
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SslCert {
    /// This is always <b>sql#sslCert</b>.
    #[prost(string, tag = "1")]
    pub kind: ::prost::alloc::string::String,
    /// Serial number, as extracted from the certificate.
    #[prost(string, tag = "2")]
    pub cert_serial_number: ::prost::alloc::string::String,
    /// PEM representation.
    #[prost(string, tag = "3")]
    pub cert: ::prost::alloc::string::String,
    /// The time when the certificate was created in <a
    /// href="<https://tools.ietf.org/html/rfc3339">RFC> 3339</a> format, for example
    /// <b>2012-11-15T16:19:00.094Z</b>
    #[prost(message, optional, tag = "4")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// User supplied name.  Constrained to [a-zA-Z.-_ ]+.
    #[prost(string, tag = "5")]
    pub common_name: ::prost::alloc::string::String,
    /// The time when the certificate expires in <a
    /// href="<https://tools.ietf.org/html/rfc3339">RFC> 3339</a> format, for example
    /// <b>2012-11-15T16:19:00.094Z</b>.
    #[prost(message, optional, tag = "6")]
    pub expiration_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Sha1 Fingerprint.
    #[prost(string, tag = "7")]
    pub sha1_fingerprint: ::prost::alloc::string::String,
    /// Name of the database instance.
    #[prost(string, tag = "8")]
    pub instance: ::prost::alloc::string::String,
    /// The URI of this resource.
    #[prost(string, tag = "9")]
    pub self_link: ::prost::alloc::string::String,
}
/// SslCertDetail.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SslCertDetail {
    /// The public information about the cert.
    #[prost(message, optional, tag = "1")]
    pub cert_info: ::core::option::Option<SslCert>,
    /// The private key for the client cert, in pem format.  Keep private in order
    /// to protect your security.
    #[prost(string, tag = "2")]
    pub cert_private_key: ::prost::alloc::string::String,
}
/// SslCerts create ephemeral certificate request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SslCertsCreateEphemeralRequest {
    /// PEM encoded public key to include in the signed certificate.
    #[prost(string, tag = "1")]
    pub public_key: ::prost::alloc::string::String,
    /// Access token to include in the signed certificate.
    #[prost(string, tag = "2")]
    pub access_token: ::prost::alloc::string::String,
}
/// SslCerts insert request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SslCertsInsertRequest {
    /// User supplied name.  Must be a distinct name from the other certificates
    /// for this instance.
    #[prost(string, tag = "1")]
    pub common_name: ::prost::alloc::string::String,
}
/// Reschedule options for maintenance windows.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SqlInstancesRescheduleMaintenanceRequestBody {
    /// Required. The type of the reschedule the user wants.
    #[prost(message, optional, tag = "3")]
    pub reschedule:
        ::core::option::Option<sql_instances_reschedule_maintenance_request_body::Reschedule>,
}
/// Nested message and enum types in `SqlInstancesRescheduleMaintenanceRequestBody`.
pub mod sql_instances_reschedule_maintenance_request_body {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Reschedule {
        /// Required. The type of the reschedule.
        #[prost(enumeration = "RescheduleType", tag = "1")]
        pub reschedule_type: i32,
        /// Optional. Timestamp when the maintenance shall be rescheduled to if
        /// reschedule_type=SPECIFIC_TIME, in <a
        /// href="<https://tools.ietf.org/html/rfc3339">RFC> 3339</a> format, for
        /// example <b>2012-11-15T16:19:00.094Z</b>.
        #[prost(message, optional, tag = "2")]
        pub schedule_time: ::core::option::Option<::prost_types::Timestamp>,
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum RescheduleType {
        Unspecified = 0,
        /// If the user wants to schedule the maintenance to happen now.
        Immediate = 1,
        /// If the user wants to use the existing maintenance policy to find the
        /// next available window.
        NextAvailableWindow = 2,
        /// If the user wants to reschedule the maintenance to a specific time.
        SpecificTime = 3,
    }
}
/// SslCert insert response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SslCertsInsertResponse {
    /// This is always <b>sql#sslCertsInsert</b>.
    #[prost(string, tag = "1")]
    pub kind: ::prost::alloc::string::String,
    /// The operation to track the ssl certs insert request.
    #[prost(message, optional, tag = "2")]
    pub operation: ::core::option::Option<Operation>,
    /// The server Certificate Authority's certificate.  If this is missing you can
    /// force a new one to be generated by calling resetSslConfig method on
    /// instances resource.
    #[prost(message, optional, tag = "3")]
    pub server_ca_cert: ::core::option::Option<SslCert>,
    /// The new client certificate and private key.
    #[prost(message, optional, tag = "4")]
    pub client_cert: ::core::option::Option<SslCertDetail>,
}
/// SslCerts list response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SslCertsListResponse {
    /// This is always <b>sql#sslCertsList</b>.
    #[prost(string, tag = "1")]
    pub kind: ::prost::alloc::string::String,
    /// List of client certificates for the instance.
    #[prost(message, repeated, tag = "2")]
    pub items: ::prost::alloc::vec::Vec<SslCert>,
}
/// Database Instance truncate log context.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TruncateLogContext {
    /// This is always <b>sql#truncateLogContext</b>.
    #[prost(string, tag = "1")]
    pub kind: ::prost::alloc::string::String,
    /// The type of log to truncate. Valid values are
    /// <b>MYSQL_GENERAL_TABLE</b> and <b>MYSQL_SLOW_TABLE</b>.
    #[prost(string, tag = "2")]
    pub log_type: ::prost::alloc::string::String,
}
/// Active Directory configuration, relevant only for Cloud SQL for SQL Server.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SqlActiveDirectoryConfig {
    /// This is always sql#activeDirectoryConfig.
    #[prost(string, tag = "1")]
    pub kind: ::prost::alloc::string::String,
    /// The name of the domain (e.g., mydomain.com).
    #[prost(string, tag = "2")]
    pub domain: ::prost::alloc::string::String,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SqlFileType {
    /// Unknown file type.
    Unspecified = 0,
    /// File containing SQL statements.
    Sql = 1,
    /// File in CSV format.
    Csv = 2,
    Bak = 4,
}
/// The status of a backup run.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SqlBackupRunStatus {
    /// The status of the run is unknown.
    Unspecified = 0,
    /// The backup operation was enqueued.
    Enqueued = 1,
    /// The backup is overdue across a given backup window. Indicates a
    /// problem. Example: Long-running operation in progress during
    /// the whole window.
    Overdue = 2,
    /// The backup is in progress.
    Running = 3,
    /// The backup failed.
    Failed = 4,
    /// The backup was successful.
    Successful = 5,
    /// The backup was skipped (without problems) for a given backup
    /// window. Example: Instance was idle.
    Skipped = 6,
    /// The backup is about to be deleted.
    DeletionPending = 7,
    /// The backup deletion failed.
    DeletionFailed = 8,
    /// The backup has been deleted.
    Deleted = 9,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SqlBackupRunType {
    /// This is an unknown BackupRun type.
    Unspecified = 0,
    /// The backup schedule automatically triggers a backup.
    Automated = 1,
    /// The user manually triggers a backup.
    OnDemand = 2,
}
/// Defines the supported backup kinds
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SqlBackupKind {
    /// This is an unknown BackupKind.
    Unspecified = 0,
    /// The snapshot based backups
    Snapshot = 1,
    /// Physical backups
    Physical = 2,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SqlBackendType {
    /// This is an unknown backend type for instance.
    Unspecified = 0,
    /// V1 speckle instance.
    FirstGen = 1,
    /// V2 speckle instance.
    SecondGen = 2,
    /// On premises instance.
    External = 3,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SqlIpAddressType {
    /// This is an unknown IP address type.
    Unspecified = 0,
    /// IP address the customer is supposed to connect to. Usually this is the
    /// load balancer's IP address
    Primary = 1,
    /// Source IP address of the connection a read replica establishes to its
    /// external primary instance. This IP address can be allowlisted by the
    /// customer in case it has a firewall that filters incoming connection to its
    /// on premises primary instance.
    Outgoing = 2,
    /// Private IP used when using private IPs and network peering.
    Private = 3,
    /// V1 IP of a migrated instance. We want the user to
    /// decommission this IP as soon as the migration is complete.
    /// Note: V1 instances with V1 ip addresses will be counted as PRIMARY.
    Migrated1stGen = 4,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SqlInstanceType {
    /// This is an unknown Cloud SQL instance type.
    Unspecified = 0,
    /// A regular Cloud SQL instance.
    CloudSqlInstance = 1,
    /// An instance running on the customer's premises that is not managed by
    /// Cloud SQL.
    OnPremisesInstance = 2,
    /// A Cloud SQL instance acting as a read-replica.
    ReadReplicaInstance = 3,
}
/// The database engine type and version.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SqlDatabaseVersion {
    /// This is an unknown database version.
    Unspecified = 0,
    /// The database version is MySQL 5.1.
    Mysql51 = 2,
    /// The database version is MySQL 5.5.
    Mysql55 = 3,
    /// The database version is MySQL 5.6.
    Mysql56 = 5,
    /// The database version is MySQL 5.7.
    Mysql57 = 6,
    /// The database version is PostgreSQL 9.6.
    Postgres96 = 9,
    /// The database version is PostgreSQL 11.
    Postgres11 = 10,
    /// The database version is SQL Server 2017 Standard.
    Sqlserver2017Standard = 11,
    /// The database version is SQL Server 2017 Enterprise.
    Sqlserver2017Enterprise = 14,
    /// The database version is SQL Server 2017 Express.
    Sqlserver2017Express = 15,
    /// The database version is SQL Server 2017 Web.
    Sqlserver2017Web = 16,
    /// The database version is PostgreSQL 10.
    Postgres10 = 18,
    /// The database version is PostgreSQL 12.
    Postgres12 = 19,
    /// The database version is MySQL 8.
    Mysql80 = 20,
    /// The database version is PostgreSQL 13.
    Postgres13 = 23,
}
/// The suspension reason of the database instance if the state is SUSPENDED.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SqlSuspensionReason {
    /// This is an unknown suspension reason.
    Unspecified = 0,
    /// The instance is suspended due to billing issues (for example:, GCP account
    /// issue)
    BillingIssue = 2,
    /// The instance is suspended due to illegal content (for example:, child
    /// pornography, copyrighted material, etc.).
    LegalIssue = 3,
    /// The instance is causing operational issues (for example:, causing the
    /// database to crash).
    OperationalIssue = 4,
    /// The KMS key used by the instance is either revoked or denied access to
    KmsKeyIssue = 5,
}
/// The pricing plan for this instance.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SqlPricingPlan {
    /// This is an unknown pricing plan for this instance.
    Unspecified = 0,
    /// The instance is billed at a monthly flat rate.
    Package = 1,
    /// The instance is billed per usage.
    PerUse = 2,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SqlReplicationType {
    /// This is an unknown replication type for a Cloud SQL instance.
    Unspecified = 0,
    /// The synchronous replication mode for First Generation instances. It is the
    /// default value.
    Synchronous = 1,
    /// The asynchronous replication mode for First Generation instances. It
    /// provides a slight performance gain, but if an outage occurs while this
    /// option is set to asynchronous, you can lose up to a few seconds of updates
    /// to your data.
    Asynchronous = 2,
}
/// The type of disk that is used for a v2 instance to use.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SqlDataDiskType {
    /// This is an unknown data disk type.
    Unspecified = 0,
    /// An SSD data disk.
    PdSsd = 1,
    /// An HDD data disk.
    PdHdd = 2,
    /// This field is deprecated and will be removed from a future version of the
    /// API.
    ObsoleteLocalSsd = 3,
}
/// The availability type of the given Cloud SQL instance.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SqlAvailabilityType {
    /// This is an unknown Availability type.
    Unspecified = 0,
    /// Zonal available instance.
    Zonal = 1,
    /// Regional available instance.
    Regional = 2,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SqlUpdateTrack {
    /// This is an unknown maintenance timing preference.
    Unspecified = 0,
    /// For instance update that requires a restart, this update track indicates
    /// your instance prefer to restart for new version early in maintenance
    /// window.
    Canary = 1,
    /// For instance update that requires a restart, this update track indicates
    /// your instance prefer to let Cloud SQL choose the timing of restart (within
    /// its Maintenance window, if applicable).
    Stable = 2,
}
/// LINT.IfChange(sql_flag_type)
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SqlFlagType {
    /// This is an unknown flag type.
    Unspecified = 0,
    /// Boolean type flag.
    Boolean = 1,
    /// String type flag.
    String = 2,
    /// Integer type flag.
    Integer = 3,
    /// Flag type used for a server startup option.
    None = 4,
    /// Type introduced specially for MySQL TimeZone offset. Accept a string value
    /// with the format [-12:59, 13:00].
    MysqlTimezoneOffset = 5,
    /// Float type flag.
    Float = 6,
    /// Comma-separated list of the strings in a SqlFlagType enum.
    RepeatedString = 7,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SqlBackupRunsDeleteRequest {
    /// The ID of the Backup Run to delete. To find a Backup Run ID, use the <a
    /// href="/sql/docs/db_path/admin-api/rest/v1beta4/backupRuns/list">list</a>
    /// method.
    #[prost(int64, tag = "1")]
    pub id: i64,
    /// Cloud SQL instance ID. This does not include the project ID.
    #[prost(string, tag = "2")]
    pub instance: ::prost::alloc::string::String,
    /// Project ID of the project that contains the instance.
    #[prost(string, tag = "3")]
    pub project: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SqlBackupRunsGetRequest {
    /// The ID of this Backup Run.
    #[prost(int64, tag = "1")]
    pub id: i64,
    /// Cloud SQL instance ID. This does not include the project ID.
    #[prost(string, tag = "2")]
    pub instance: ::prost::alloc::string::String,
    /// Project ID of the project that contains the instance.
    #[prost(string, tag = "3")]
    pub project: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SqlBackupRunsInsertRequest {
    /// Cloud SQL instance ID. This does not include the project ID.
    #[prost(string, tag = "1")]
    pub instance: ::prost::alloc::string::String,
    /// Project ID of the project that contains the instance.
    #[prost(string, tag = "2")]
    pub project: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "100")]
    pub body: ::core::option::Option<BackupRun>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SqlBackupRunsListRequest {
    /// Cloud SQL instance ID. This does not include the project ID.
    #[prost(string, tag = "1")]
    pub instance: ::prost::alloc::string::String,
    /// Maximum number of backup runs per response.
    #[prost(int32, tag = "2")]
    pub max_results: i32,
    /// A previously-returned page token representing part of the larger set of
    /// results to view.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Project ID of the project that contains the instance.
    #[prost(string, tag = "4")]
    pub project: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SqlDatabasesDeleteRequest {
    /// Name of the database to be deleted in the instance.
    #[prost(string, tag = "1")]
    pub database: ::prost::alloc::string::String,
    /// Database instance ID. This does not include the project ID.
    #[prost(string, tag = "2")]
    pub instance: ::prost::alloc::string::String,
    /// Project ID of the project that contains the instance.
    #[prost(string, tag = "3")]
    pub project: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SqlDatabasesGetRequest {
    /// Name of the database in the instance.
    #[prost(string, tag = "1")]
    pub database: ::prost::alloc::string::String,
    /// Database instance ID. This does not include the project ID.
    #[prost(string, tag = "2")]
    pub instance: ::prost::alloc::string::String,
    /// Project ID of the project that contains the instance.
    #[prost(string, tag = "3")]
    pub project: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SqlDatabasesInsertRequest {
    /// Database instance ID. This does not include the project ID.
    #[prost(string, tag = "1")]
    pub instance: ::prost::alloc::string::String,
    /// Project ID of the project that contains the instance.
    #[prost(string, tag = "2")]
    pub project: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "100")]
    pub body: ::core::option::Option<Database>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SqlDatabasesListRequest {
    /// Cloud SQL instance ID. This does not include the project ID.
    #[prost(string, tag = "1")]
    pub instance: ::prost::alloc::string::String,
    /// Project ID of the project that contains the instance.
    #[prost(string, tag = "2")]
    pub project: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SqlDatabasesUpdateRequest {
    /// Name of the database to be updated in the instance.
    #[prost(string, tag = "1")]
    pub database: ::prost::alloc::string::String,
    /// Database instance ID. This does not include the project ID.
    #[prost(string, tag = "2")]
    pub instance: ::prost::alloc::string::String,
    /// Project ID of the project that contains the instance.
    #[prost(string, tag = "3")]
    pub project: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "100")]
    pub body: ::core::option::Option<Database>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SqlFlagsListRequest {
    /// Database type and version you want to retrieve flags for. By default, this
    /// method returns flags for all database types and versions.
    #[prost(string, tag = "1")]
    pub database_version: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SqlInstancesAddServerCaRequest {
    /// Cloud SQL instance ID. This does not include the project ID.
    #[prost(string, tag = "1")]
    pub instance: ::prost::alloc::string::String,
    /// Project ID of the project that contains the instance.
    #[prost(string, tag = "2")]
    pub project: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SqlInstancesCloneRequest {
    /// The ID of the Cloud SQL instance to be cloned (source). This does not
    /// include the project ID.
    #[prost(string, tag = "1")]
    pub instance: ::prost::alloc::string::String,
    /// Project ID of the source as well as the clone Cloud SQL instance.
    #[prost(string, tag = "2")]
    pub project: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "100")]
    pub body: ::core::option::Option<InstancesCloneRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SqlInstancesDeleteRequest {
    /// Cloud SQL instance ID. This does not include the project ID.
    #[prost(string, tag = "1")]
    pub instance: ::prost::alloc::string::String,
    /// Project ID of the project that contains the instance to be deleted.
    #[prost(string, tag = "2")]
    pub project: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SqlInstancesDemoteMasterRequest {
    /// Cloud SQL instance name.
    #[prost(string, tag = "1")]
    pub instance: ::prost::alloc::string::String,
    /// ID of the project that contains the instance.
    #[prost(string, tag = "2")]
    pub project: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "100")]
    pub body: ::core::option::Option<InstancesDemoteMasterRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SqlInstancesExportRequest {
    /// Cloud SQL instance ID. This does not include the project ID.
    #[prost(string, tag = "1")]
    pub instance: ::prost::alloc::string::String,
    /// Project ID of the project that contains the instance to be exported.
    #[prost(string, tag = "2")]
    pub project: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "100")]
    pub body: ::core::option::Option<InstancesExportRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SqlInstancesFailoverRequest {
    /// Cloud SQL instance ID. This does not include the project ID.
    #[prost(string, tag = "1")]
    pub instance: ::prost::alloc::string::String,
    /// ID of the project that contains the read replica.
    #[prost(string, tag = "2")]
    pub project: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "100")]
    pub body: ::core::option::Option<InstancesFailoverRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SqlInstancesGetRequest {
    /// Database instance ID. This does not include the project ID.
    #[prost(string, tag = "1")]
    pub instance: ::prost::alloc::string::String,
    /// Project ID of the project that contains the instance.
    #[prost(string, tag = "2")]
    pub project: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SqlInstancesImportRequest {
    /// Cloud SQL instance ID. This does not include the project ID.
    #[prost(string, tag = "1")]
    pub instance: ::prost::alloc::string::String,
    /// Project ID of the project that contains the instance.
    #[prost(string, tag = "2")]
    pub project: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "100")]
    pub body: ::core::option::Option<InstancesImportRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SqlInstancesInsertRequest {
    /// Project ID of the project to which the newly created Cloud SQL instances
    /// should belong.
    #[prost(string, tag = "1")]
    pub project: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "100")]
    pub body: ::core::option::Option<DatabaseInstance>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SqlInstancesListRequest {
    /// A filter expression that filters resources listed in the response.
    /// The expression is in the form of field:value. For example,
    /// 'instanceType:CLOUD_SQL_INSTANCE'. Fields can be nested as needed as per
    /// their JSON representation, such as 'settings.userLabels.auto_start:true'.
    ///
    /// Multiple filter queries are space-separated. For example.
    /// 'state:RUNNABLE instanceType:CLOUD_SQL_INSTANCE'. By default, each
    /// expression is an AND expression. However, you can include AND and OR
    /// expressions explicitly.
    #[prost(string, tag = "1")]
    pub filter: ::prost::alloc::string::String,
    /// The maximum number of results to return per response.
    #[prost(uint32, tag = "2")]
    pub max_results: u32,
    /// A previously-returned page token representing part of the larger set of
    /// results to view.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Project ID of the project for which to list Cloud SQL instances.
    #[prost(string, tag = "4")]
    pub project: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SqlInstancesListServerCasRequest {
    /// Cloud SQL instance ID. This does not include the project ID.
    #[prost(string, tag = "1")]
    pub instance: ::prost::alloc::string::String,
    /// Project ID of the project that contains the instance.
    #[prost(string, tag = "2")]
    pub project: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SqlInstancesPatchRequest {
    /// Cloud SQL instance ID. This does not include the project ID.
    #[prost(string, tag = "1")]
    pub instance: ::prost::alloc::string::String,
    /// Project ID of the project that contains the instance.
    #[prost(string, tag = "2")]
    pub project: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "100")]
    pub body: ::core::option::Option<DatabaseInstance>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SqlInstancesPromoteReplicaRequest {
    /// Cloud SQL read replica instance name.
    #[prost(string, tag = "1")]
    pub instance: ::prost::alloc::string::String,
    /// ID of the project that contains the read replica.
    #[prost(string, tag = "2")]
    pub project: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SqlInstancesResetSslConfigRequest {
    /// Cloud SQL instance ID. This does not include the project ID.
    #[prost(string, tag = "1")]
    pub instance: ::prost::alloc::string::String,
    /// Project ID of the project that contains the instance.
    #[prost(string, tag = "2")]
    pub project: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SqlInstancesRestartRequest {
    /// Cloud SQL instance ID. This does not include the project ID.
    #[prost(string, tag = "1")]
    pub instance: ::prost::alloc::string::String,
    /// Project ID of the project that contains the instance to be restarted.
    #[prost(string, tag = "2")]
    pub project: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SqlInstancesRestoreBackupRequest {
    /// Cloud SQL instance ID. This does not include the project ID.
    #[prost(string, tag = "1")]
    pub instance: ::prost::alloc::string::String,
    /// Project ID of the project that contains the instance.
    #[prost(string, tag = "2")]
    pub project: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "100")]
    pub body: ::core::option::Option<InstancesRestoreBackupRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SqlInstancesRotateServerCaRequest {
    /// Cloud SQL instance ID. This does not include the project ID.
    #[prost(string, tag = "1")]
    pub instance: ::prost::alloc::string::String,
    /// Project ID of the project that contains the instance.
    #[prost(string, tag = "2")]
    pub project: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "100")]
    pub body: ::core::option::Option<InstancesRotateServerCaRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SqlInstancesStartReplicaRequest {
    /// Cloud SQL read replica instance name.
    #[prost(string, tag = "1")]
    pub instance: ::prost::alloc::string::String,
    /// ID of the project that contains the read replica.
    #[prost(string, tag = "2")]
    pub project: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SqlInstancesStopReplicaRequest {
    /// Cloud SQL read replica instance name.
    #[prost(string, tag = "1")]
    pub instance: ::prost::alloc::string::String,
    /// ID of the project that contains the read replica.
    #[prost(string, tag = "2")]
    pub project: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SqlInstancesTruncateLogRequest {
    /// Cloud SQL instance ID. This does not include the project ID.
    #[prost(string, tag = "1")]
    pub instance: ::prost::alloc::string::String,
    /// Project ID of the Cloud SQL project.
    #[prost(string, tag = "2")]
    pub project: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "100")]
    pub body: ::core::option::Option<InstancesTruncateLogRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SqlInstancesUpdateRequest {
    /// Cloud SQL instance ID. This does not include the project ID.
    #[prost(string, tag = "1")]
    pub instance: ::prost::alloc::string::String,
    /// Project ID of the project that contains the instance.
    #[prost(string, tag = "2")]
    pub project: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "100")]
    pub body: ::core::option::Option<DatabaseInstance>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SqlInstancesRescheduleMaintenanceRequest {
    /// Cloud SQL instance ID. This does not include the project ID.
    #[prost(string, tag = "1")]
    pub instance: ::prost::alloc::string::String,
    /// ID of the project that contains the instance.
    #[prost(string, tag = "2")]
    pub project: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "100")]
    pub body: ::core::option::Option<SqlInstancesRescheduleMaintenanceRequestBody>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SqlInstancesVerifyExternalSyncSettingsRequest {
    /// Cloud SQL instance ID. This does not include the project ID.
    #[prost(string, tag = "1")]
    pub instance: ::prost::alloc::string::String,
    /// Project ID of the project that contains the instance.
    #[prost(string, tag = "2")]
    pub project: ::prost::alloc::string::String,
    /// Flag to enable verifying connection only
    #[prost(bool, tag = "3")]
    pub verify_connection_only: bool,
    /// External sync mode
    #[prost(
        enumeration = "sql_instances_verify_external_sync_settings_request::ExternalSyncMode",
        tag = "4"
    )]
    pub sync_mode: i32,
}
/// Nested message and enum types in `SqlInstancesVerifyExternalSyncSettingsRequest`.
pub mod sql_instances_verify_external_sync_settings_request {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ExternalSyncMode {
        /// Unknown external sync mode, will be defaulted to ONLINE mode
        Unspecified = 0,
        /// Online external sync will set up replication after initial data external
        /// sync
        Online = 1,
        /// Offline external sync only dumps and loads a one-time snapshot of
        /// the primary instance's data
        Offline = 2,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SqlInstancesStartExternalSyncRequest {
    /// Cloud SQL instance ID. This does not include the project ID.
    #[prost(string, tag = "1")]
    pub instance: ::prost::alloc::string::String,
    /// ID of the project that contains the instance.
    #[prost(string, tag = "2")]
    pub project: ::prost::alloc::string::String,
    /// External sync mode.
    #[prost(
        enumeration = "sql_instances_verify_external_sync_settings_request::ExternalSyncMode",
        tag = "3"
    )]
    pub sync_mode: i32,
    /// Whether to skip the verification step (VESS).
    #[prost(bool, tag = "4")]
    pub skip_verification: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SqlOperationsGetRequest {
    /// Instance operation ID.
    #[prost(string, tag = "1")]
    pub operation: ::prost::alloc::string::String,
    /// Project ID of the project that contains the instance.
    #[prost(string, tag = "2")]
    pub project: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SqlOperationsListRequest {
    /// Cloud SQL instance ID. This does not include the project ID.
    #[prost(string, tag = "1")]
    pub instance: ::prost::alloc::string::String,
    /// Maximum number of operations per response.
    #[prost(uint32, tag = "2")]
    pub max_results: u32,
    /// A previously-returned page token representing part of the larger set of
    /// results to view.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Project ID of the project that contains the instance.
    #[prost(string, tag = "4")]
    pub project: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SqlInstancesCreateEphemeralCertRequest {
    /// Cloud SQL instance ID. This does not include the project ID.
    #[prost(string, tag = "1")]
    pub instance: ::prost::alloc::string::String,
    /// Project ID of the Cloud SQL project.
    #[prost(string, tag = "2")]
    pub project: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "100")]
    pub body: ::core::option::Option<SslCertsCreateEphemeralRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SqlSslCertsDeleteRequest {
    /// Cloud SQL instance ID. This does not include the project ID.
    #[prost(string, tag = "1")]
    pub instance: ::prost::alloc::string::String,
    /// Project ID of the project that contains the instance.
    #[prost(string, tag = "2")]
    pub project: ::prost::alloc::string::String,
    /// Sha1 FingerPrint.
    #[prost(string, tag = "3")]
    pub sha1_fingerprint: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SqlSslCertsGetRequest {
    /// Cloud SQL instance ID. This does not include the project ID.
    #[prost(string, tag = "1")]
    pub instance: ::prost::alloc::string::String,
    /// Project ID of the project that contains the instance.
    #[prost(string, tag = "2")]
    pub project: ::prost::alloc::string::String,
    /// Sha1 FingerPrint.
    #[prost(string, tag = "3")]
    pub sha1_fingerprint: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SqlSslCertsInsertRequest {
    /// Cloud SQL instance ID. This does not include the project ID.
    #[prost(string, tag = "1")]
    pub instance: ::prost::alloc::string::String,
    /// Project ID of the project that contains the instance.
    #[prost(string, tag = "2")]
    pub project: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "100")]
    pub body: ::core::option::Option<SslCertsInsertRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SqlSslCertsListRequest {
    /// Cloud SQL instance ID. This does not include the project ID.
    #[prost(string, tag = "1")]
    pub instance: ::prost::alloc::string::String,
    /// Project ID of the project that contains the instance.
    #[prost(string, tag = "2")]
    pub project: ::prost::alloc::string::String,
}
#[doc = r" Generated client implementations."]
pub mod sql_backup_runs_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct SqlBackupRunsServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> SqlBackupRunsServiceClient<T>
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
        ) -> SqlBackupRunsServiceClient<InterceptedService<T, F>>
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
            SqlBackupRunsServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Deletes the backup taken by a backup run."]
        pub async fn delete(
            &mut self,
            request: impl tonic::IntoRequest<super::SqlBackupRunsDeleteRequest>,
        ) -> Result<tonic::Response<super::Operation>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.sql.v1beta4.SqlBackupRunsService/Delete",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Retrieves a resource containing information about a backup run."]
        pub async fn get(
            &mut self,
            request: impl tonic::IntoRequest<super::SqlBackupRunsGetRequest>,
        ) -> Result<tonic::Response<super::BackupRun>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.sql.v1beta4.SqlBackupRunsService/Get",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a new backup run on demand. This method is applicable only to"]
        #[doc = " Second Generation instances."]
        pub async fn insert(
            &mut self,
            request: impl tonic::IntoRequest<super::SqlBackupRunsInsertRequest>,
        ) -> Result<tonic::Response<super::Operation>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.sql.v1beta4.SqlBackupRunsService/Insert",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists all backup runs associated with a given instance and configuration in"]
        #[doc = " the reverse chronological order of the backup initiation time."]
        pub async fn list(
            &mut self,
            request: impl tonic::IntoRequest<super::SqlBackupRunsListRequest>,
        ) -> Result<tonic::Response<super::BackupRunsListResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.sql.v1beta4.SqlBackupRunsService/List",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
#[doc = r" Generated client implementations."]
pub mod sql_databases_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct SqlDatabasesServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> SqlDatabasesServiceClient<T>
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
        ) -> SqlDatabasesServiceClient<InterceptedService<T, F>>
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
            SqlDatabasesServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Deletes a database from a Cloud SQL instance."]
        pub async fn delete(
            &mut self,
            request: impl tonic::IntoRequest<super::SqlDatabasesDeleteRequest>,
        ) -> Result<tonic::Response<super::Operation>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.sql.v1beta4.SqlDatabasesService/Delete",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Retrieves a resource containing information about a database inside a Cloud"]
        #[doc = " SQL instance."]
        pub async fn get(
            &mut self,
            request: impl tonic::IntoRequest<super::SqlDatabasesGetRequest>,
        ) -> Result<tonic::Response<super::Database>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.sql.v1beta4.SqlDatabasesService/Get",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Inserts a resource containing information about a database inside a Cloud"]
        #[doc = " SQL instance."]
        pub async fn insert(
            &mut self,
            request: impl tonic::IntoRequest<super::SqlDatabasesInsertRequest>,
        ) -> Result<tonic::Response<super::Operation>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.sql.v1beta4.SqlDatabasesService/Insert",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists databases in the specified Cloud SQL instance."]
        pub async fn list(
            &mut self,
            request: impl tonic::IntoRequest<super::SqlDatabasesListRequest>,
        ) -> Result<tonic::Response<super::DatabasesListResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.sql.v1beta4.SqlDatabasesService/List",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Partially updates a resource containing information about a database inside"]
        #[doc = " a Cloud SQL instance. This method supports patch semantics."]
        pub async fn patch(
            &mut self,
            request: impl tonic::IntoRequest<super::SqlDatabasesUpdateRequest>,
        ) -> Result<tonic::Response<super::Operation>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.sql.v1beta4.SqlDatabasesService/Patch",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates a resource containing information about a database inside a Cloud"]
        #[doc = " SQL instance."]
        pub async fn update(
            &mut self,
            request: impl tonic::IntoRequest<super::SqlDatabasesUpdateRequest>,
        ) -> Result<tonic::Response<super::Operation>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.sql.v1beta4.SqlDatabasesService/Update",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
#[doc = r" Generated client implementations."]
pub mod sql_flags_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct SqlFlagsServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> SqlFlagsServiceClient<T>
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
        ) -> SqlFlagsServiceClient<InterceptedService<T, F>>
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
            SqlFlagsServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " List all available database flags for Cloud SQL instances."]
        pub async fn list(
            &mut self,
            request: impl tonic::IntoRequest<super::SqlFlagsListRequest>,
        ) -> Result<tonic::Response<super::FlagsListResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.sql.v1beta4.SqlFlagsService/List",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
#[doc = r" Generated client implementations."]
pub mod sql_instances_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct SqlInstancesServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> SqlInstancesServiceClient<T>
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
        ) -> SqlInstancesServiceClient<InterceptedService<T, F>>
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
            SqlInstancesServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Add a new trusted Certificate Authority (CA) version for the specified"]
        #[doc = " instance. Required to prepare for a certificate rotation. If a CA version"]
        #[doc = " was previously added but never used in a certificate rotation, this"]
        #[doc = " operation replaces that version. There cannot be more than one CA version"]
        #[doc = " waiting to be rotated in."]
        pub async fn add_server_ca(
            &mut self,
            request: impl tonic::IntoRequest<super::SqlInstancesAddServerCaRequest>,
        ) -> Result<tonic::Response<super::Operation>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.sql.v1beta4.SqlInstancesService/AddServerCa",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a Cloud SQL instance as a clone of the source instance. Using this"]
        #[doc = " operation might cause your instance to restart."]
        pub async fn clone(
            &mut self,
            request: impl tonic::IntoRequest<super::SqlInstancesCloneRequest>,
        ) -> Result<tonic::Response<super::Operation>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.sql.v1beta4.SqlInstancesService/Clone",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes a Cloud SQL instance."]
        pub async fn delete(
            &mut self,
            request: impl tonic::IntoRequest<super::SqlInstancesDeleteRequest>,
        ) -> Result<tonic::Response<super::Operation>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.sql.v1beta4.SqlInstancesService/Delete",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Demotes the stand-alone instance to be a Cloud SQL read replica for an"]
        #[doc = " external database server."]
        pub async fn demote_master(
            &mut self,
            request: impl tonic::IntoRequest<super::SqlInstancesDemoteMasterRequest>,
        ) -> Result<tonic::Response<super::Operation>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.sql.v1beta4.SqlInstancesService/DemoteMaster",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Exports data from a Cloud SQL instance to a Cloud Storage bucket as a SQL"]
        #[doc = " dump or CSV file."]
        pub async fn export(
            &mut self,
            request: impl tonic::IntoRequest<super::SqlInstancesExportRequest>,
        ) -> Result<tonic::Response<super::Operation>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.sql.v1beta4.SqlInstancesService/Export",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Failover the instance to its failover replica instance. Using this"]
        #[doc = " operation might cause your instance to restart."]
        pub async fn failover(
            &mut self,
            request: impl tonic::IntoRequest<super::SqlInstancesFailoverRequest>,
        ) -> Result<tonic::Response<super::Operation>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.sql.v1beta4.SqlInstancesService/Failover",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Retrieves a resource containing information about a Cloud SQL instance."]
        pub async fn get(
            &mut self,
            request: impl tonic::IntoRequest<super::SqlInstancesGetRequest>,
        ) -> Result<tonic::Response<super::DatabaseInstance>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.sql.v1beta4.SqlInstancesService/Get",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Imports data into a Cloud SQL instance from a SQL dump  or CSV file in"]
        #[doc = " Cloud Storage."]
        pub async fn import(
            &mut self,
            request: impl tonic::IntoRequest<super::SqlInstancesImportRequest>,
        ) -> Result<tonic::Response<super::Operation>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.sql.v1beta4.SqlInstancesService/Import",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a new Cloud SQL instance."]
        pub async fn insert(
            &mut self,
            request: impl tonic::IntoRequest<super::SqlInstancesInsertRequest>,
        ) -> Result<tonic::Response<super::Operation>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.sql.v1beta4.SqlInstancesService/Insert",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists instances under a given project."]
        pub async fn list(
            &mut self,
            request: impl tonic::IntoRequest<super::SqlInstancesListRequest>,
        ) -> Result<tonic::Response<super::InstancesListResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.sql.v1beta4.SqlInstancesService/List",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists all of the trusted Certificate Authorities (CAs) for the specified"]
        #[doc = " instance. There can be up to three CAs listed: the CA that was used to sign"]
        #[doc = " the certificate that is currently in use, a CA that has been added but not"]
        #[doc = " yet used to sign a certificate, and a CA used to sign a certificate that"]
        #[doc = " has previously rotated out."]
        pub async fn list_server_cas(
            &mut self,
            request: impl tonic::IntoRequest<super::SqlInstancesListServerCasRequest>,
        ) -> Result<tonic::Response<super::InstancesListServerCasResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.sql.v1beta4.SqlInstancesService/ListServerCas",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates settings of a Cloud SQL instance."]
        #[doc = " This method supports patch semantics."]
        pub async fn patch(
            &mut self,
            request: impl tonic::IntoRequest<super::SqlInstancesPatchRequest>,
        ) -> Result<tonic::Response<super::Operation>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.sql.v1beta4.SqlInstancesService/Patch",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Promotes the read replica instance to be a stand-alone Cloud SQL instance."]
        #[doc = " Using this operation might cause your instance to restart."]
        pub async fn promote_replica(
            &mut self,
            request: impl tonic::IntoRequest<super::SqlInstancesPromoteReplicaRequest>,
        ) -> Result<tonic::Response<super::Operation>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.sql.v1beta4.SqlInstancesService/PromoteReplica",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes all client certificates and generates a new server SSL certificate"]
        #[doc = " for the instance."]
        pub async fn reset_ssl_config(
            &mut self,
            request: impl tonic::IntoRequest<super::SqlInstancesResetSslConfigRequest>,
        ) -> Result<tonic::Response<super::Operation>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.sql.v1beta4.SqlInstancesService/ResetSslConfig",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Restarts a Cloud SQL instance."]
        pub async fn restart(
            &mut self,
            request: impl tonic::IntoRequest<super::SqlInstancesRestartRequest>,
        ) -> Result<tonic::Response<super::Operation>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.sql.v1beta4.SqlInstancesService/Restart",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Restores a backup of a Cloud SQL instance. Using this operation might cause"]
        #[doc = " your instance to restart."]
        pub async fn restore_backup(
            &mut self,
            request: impl tonic::IntoRequest<super::SqlInstancesRestoreBackupRequest>,
        ) -> Result<tonic::Response<super::Operation>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.sql.v1beta4.SqlInstancesService/RestoreBackup",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Rotates the server certificate to one signed by the Certificate Authority"]
        #[doc = " (CA) version previously added with the addServerCA method."]
        pub async fn rotate_server_ca(
            &mut self,
            request: impl tonic::IntoRequest<super::SqlInstancesRotateServerCaRequest>,
        ) -> Result<tonic::Response<super::Operation>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.sql.v1beta4.SqlInstancesService/RotateServerCa",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Starts the replication in the read replica instance."]
        pub async fn start_replica(
            &mut self,
            request: impl tonic::IntoRequest<super::SqlInstancesStartReplicaRequest>,
        ) -> Result<tonic::Response<super::Operation>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.sql.v1beta4.SqlInstancesService/StartReplica",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Stops the replication in the read replica instance."]
        pub async fn stop_replica(
            &mut self,
            request: impl tonic::IntoRequest<super::SqlInstancesStopReplicaRequest>,
        ) -> Result<tonic::Response<super::Operation>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.sql.v1beta4.SqlInstancesService/StopReplica",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Truncate MySQL general and slow query log tables"]
        #[doc = " MySQL only."]
        pub async fn truncate_log(
            &mut self,
            request: impl tonic::IntoRequest<super::SqlInstancesTruncateLogRequest>,
        ) -> Result<tonic::Response<super::Operation>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.sql.v1beta4.SqlInstancesService/TruncateLog",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates settings of a Cloud SQL instance. Using this operation might cause"]
        #[doc = " your instance to restart."]
        pub async fn update(
            &mut self,
            request: impl tonic::IntoRequest<super::SqlInstancesUpdateRequest>,
        ) -> Result<tonic::Response<super::Operation>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.sql.v1beta4.SqlInstancesService/Update",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Generates a short-lived X509 certificate containing the provided public key"]
        #[doc = " and signed by a private key specific to the target instance. Users may use"]
        #[doc = " the certificate to authenticate as themselves when connecting to the"]
        #[doc = " database."]
        pub async fn create_ephemeral(
            &mut self,
            request: impl tonic::IntoRequest<super::SqlInstancesCreateEphemeralCertRequest>,
        ) -> Result<tonic::Response<super::SslCert>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.sql.v1beta4.SqlInstancesService/CreateEphemeral",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Reschedules the maintenance on the given instance."]
        pub async fn reschedule_maintenance(
            &mut self,
            request: impl tonic::IntoRequest<super::SqlInstancesRescheduleMaintenanceRequest>,
        ) -> Result<tonic::Response<super::Operation>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.sql.v1beta4.SqlInstancesService/RescheduleMaintenance",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Verify External primary instance external sync settings."]
        pub async fn verify_external_sync_settings(
            &mut self,
            request: impl tonic::IntoRequest<super::SqlInstancesVerifyExternalSyncSettingsRequest>,
        ) -> Result<
            tonic::Response<super::SqlInstancesVerifyExternalSyncSettingsResponse>,
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
                "/google.cloud.sql.v1beta4.SqlInstancesService/VerifyExternalSyncSettings",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Start External primary instance migration."]
        pub async fn start_external_sync(
            &mut self,
            request: impl tonic::IntoRequest<super::SqlInstancesStartExternalSyncRequest>,
        ) -> Result<tonic::Response<super::Operation>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.sql.v1beta4.SqlInstancesService/StartExternalSync",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
#[doc = r" Generated client implementations."]
pub mod sql_operations_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct SqlOperationsServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> SqlOperationsServiceClient<T>
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
        ) -> SqlOperationsServiceClient<InterceptedService<T, F>>
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
            SqlOperationsServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Retrieves an instance operation that has been performed on an instance."]
        pub async fn get(
            &mut self,
            request: impl tonic::IntoRequest<super::SqlOperationsGetRequest>,
        ) -> Result<tonic::Response<super::Operation>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.sql.v1beta4.SqlOperationsService/Get",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists all instance operations that have been performed on the given Cloud"]
        #[doc = " SQL instance in the reverse chronological order of the start time."]
        pub async fn list(
            &mut self,
            request: impl tonic::IntoRequest<super::SqlOperationsListRequest>,
        ) -> Result<tonic::Response<super::OperationsListResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.sql.v1beta4.SqlOperationsService/List",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
#[doc = r" Generated client implementations."]
pub mod sql_ssl_certs_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct SqlSslCertsServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> SqlSslCertsServiceClient<T>
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
        ) -> SqlSslCertsServiceClient<InterceptedService<T, F>>
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
            SqlSslCertsServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Deletes the SSL certificate. For First Generation instances, the"]
        #[doc = " certificate remains valid until the instance is restarted."]
        pub async fn delete(
            &mut self,
            request: impl tonic::IntoRequest<super::SqlSslCertsDeleteRequest>,
        ) -> Result<tonic::Response<super::Operation>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.sql.v1beta4.SqlSslCertsService/Delete",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Retrieves a particular SSL certificate.  Does not include the private key"]
        #[doc = " (required for usage).  The private key must be saved from the response to"]
        #[doc = " initial creation."]
        pub async fn get(
            &mut self,
            request: impl tonic::IntoRequest<super::SqlSslCertsGetRequest>,
        ) -> Result<tonic::Response<super::SslCert>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.sql.v1beta4.SqlSslCertsService/Get",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates an SSL certificate and returns it along with the private key and"]
        #[doc = " server certificate authority.  The new certificate will not be usable until"]
        #[doc = " the instance is restarted."]
        pub async fn insert(
            &mut self,
            request: impl tonic::IntoRequest<super::SqlSslCertsInsertRequest>,
        ) -> Result<tonic::Response<super::SslCertsInsertResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.sql.v1beta4.SqlSslCertsService/Insert",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists all of the current SSL certificates for the instance."]
        pub async fn list(
            &mut self,
            request: impl tonic::IntoRequest<super::SqlSslCertsListRequest>,
        ) -> Result<tonic::Response<super::SslCertsListResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.sql.v1beta4.SqlSslCertsService/List",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SqlTiersListRequest {
    /// Project ID of the project for which to list tiers.
    #[prost(string, tag = "1")]
    pub project: ::prost::alloc::string::String,
}
/// Tiers list response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TiersListResponse {
    /// This is always <b>sql#tiersList</b>.
    #[prost(string, tag = "1")]
    pub kind: ::prost::alloc::string::String,
    /// List of tiers.
    #[prost(message, repeated, tag = "2")]
    pub items: ::prost::alloc::vec::Vec<Tier>,
}
/// A Google Cloud SQL service tier resource.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Tier {
    /// An identifier for the machine type, for example, db-custom-1-3840. For
    /// related information, see <a href="/sql/pricing">Pricing</a>.
    #[prost(string, tag = "1")]
    pub tier: ::prost::alloc::string::String,
    /// The maximum RAM usage of this tier in bytes.
    #[prost(int64, tag = "2")]
    pub ram: i64,
    /// This is always <b>sql#tier</b>.
    #[prost(string, tag = "3")]
    pub kind: ::prost::alloc::string::String,
    /// The maximum disk size of this tier in bytes.
    #[prost(int64, tag = "4")]
    pub disk_quota: i64,
    /// The applicable regions for this tier.
    #[prost(string, repeated, tag = "5")]
    pub region: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[doc = r" Generated client implementations."]
pub mod sql_tiers_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Service for providing machine types (tiers) for Cloud SQL."]
    #[derive(Debug, Clone)]
    pub struct SqlTiersServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> SqlTiersServiceClient<T>
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
        ) -> SqlTiersServiceClient<InterceptedService<T, F>>
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
            SqlTiersServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Lists all available machine types (tiers) for Cloud SQL, for example,"]
        #[doc = " db-custom-1-3840. For related information, see <a"]
        #[doc = " href=\"/sql/pricing\">Pricing</a>."]
        pub async fn list(
            &mut self,
            request: impl tonic::IntoRequest<super::SqlTiersListRequest>,
        ) -> Result<tonic::Response<super::TiersListResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.sql.v1beta4.SqlTiersService/List",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SqlUsersDeleteRequest {
    /// Host of the user in the instance.
    #[prost(string, tag = "1")]
    pub host: ::prost::alloc::string::String,
    /// Database instance ID. This does not include the project ID.
    #[prost(string, tag = "2")]
    pub instance: ::prost::alloc::string::String,
    /// Name of the user in the instance.
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
    /// Project ID of the project that contains the instance.
    #[prost(string, tag = "4")]
    pub project: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SqlUsersUpdateRequest {
    /// Optional. Host of the user in the instance.
    #[prost(string, tag = "1")]
    pub host: ::prost::alloc::string::String,
    /// Database instance ID. This does not include the project ID.
    #[prost(string, tag = "2")]
    pub instance: ::prost::alloc::string::String,
    /// Name of the user in the instance.
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
    /// Project ID of the project that contains the instance.
    #[prost(string, tag = "4")]
    pub project: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "100")]
    pub body: ::core::option::Option<User>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SqlUsersInsertRequest {
    /// Database instance ID. This does not include the project ID.
    #[prost(string, tag = "1")]
    pub instance: ::prost::alloc::string::String,
    /// Project ID of the project that contains the instance.
    #[prost(string, tag = "2")]
    pub project: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "100")]
    pub body: ::core::option::Option<User>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SqlUsersListRequest {
    /// Database instance ID. This does not include the project ID.
    #[prost(string, tag = "1")]
    pub instance: ::prost::alloc::string::String,
    /// Project ID of the project that contains the instance.
    #[prost(string, tag = "2")]
    pub project: ::prost::alloc::string::String,
}
/// A Cloud SQL user resource.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct User {
    /// This is always <b>sql#user</b>.
    #[prost(string, tag = "1")]
    pub kind: ::prost::alloc::string::String,
    /// The password for the user.
    #[prost(string, tag = "2")]
    pub password: ::prost::alloc::string::String,
    /// This field is deprecated and will be removed from a future version of the
    /// API.
    #[prost(string, tag = "3")]
    pub etag: ::prost::alloc::string::String,
    /// The name of the user in the Cloud SQL instance. Can be omitted for
    /// <b>update</b> since it is already specified in the URL.
    #[prost(string, tag = "4")]
    pub name: ::prost::alloc::string::String,
    /// The host name from which the user can connect. For <b>insert</b>
    /// operations, host defaults to an empty string. For <b>update</b>
    /// operations, host is specified as part of the request URL. The host name
    /// cannot be updated after insertion.
    #[prost(string, tag = "5")]
    pub host: ::prost::alloc::string::String,
    /// The name of the Cloud SQL instance. This does not include the project ID.
    /// Can be omitted for <b>update</b> since it is already specified on the
    /// URL.
    #[prost(string, tag = "6")]
    pub instance: ::prost::alloc::string::String,
    /// The project ID of the project containing the Cloud SQL database. The Google
    /// apps domain is prefixed if applicable. Can be omitted for
    /// <b>update</b> since it is already specified on the URL.
    #[prost(string, tag = "7")]
    pub project: ::prost::alloc::string::String,
    /// The user type. It determines the method to authenticate the user during
    /// login. The default is the database's built-in user type.
    #[prost(enumeration = "user::SqlUserType", tag = "8")]
    pub r#type: i32,
    /// User details for specific database type
    #[prost(oneof = "user::UserDetails", tags = "9")]
    pub user_details: ::core::option::Option<user::UserDetails>,
}
/// Nested message and enum types in `User`.
pub mod user {
    /// The user type.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum SqlUserType {
        /// The database's built-in user type.
        BuiltIn = 0,
        /// Cloud IAM user.
        CloudIamUser = 1,
        /// Cloud IAM service account.
        CloudIamServiceAccount = 2,
    }
    /// User details for specific database type
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum UserDetails {
        #[prost(message, tag = "9")]
        SqlserverUserDetails(super::SqlServerUserDetails),
    }
}
/// Represents a Sql Server user on the Cloud SQL instance.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SqlServerUserDetails {
    /// If the user has been disabled
    #[prost(bool, tag = "1")]
    pub disabled: bool,
    /// The server roles for this user
    #[prost(string, repeated, tag = "2")]
    pub server_roles: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// User list response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UsersListResponse {
    /// This is always <b>sql#usersList</b>.
    #[prost(string, tag = "1")]
    pub kind: ::prost::alloc::string::String,
    /// List of user resources in the instance.
    #[prost(message, repeated, tag = "2")]
    pub items: ::prost::alloc::vec::Vec<User>,
    /// An identifier that uniquely identifies the operation. You can use this
    /// identifier to retrieve the Operations resource that has information about
    /// the operation.
    #[prost(string, tag = "3")]
    pub next_page_token: ::prost::alloc::string::String,
}
#[doc = r" Generated client implementations."]
pub mod sql_users_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct SqlUsersServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> SqlUsersServiceClient<T>
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
        ) -> SqlUsersServiceClient<InterceptedService<T, F>>
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
            SqlUsersServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Deletes a user from a Cloud SQL instance."]
        pub async fn delete(
            &mut self,
            request: impl tonic::IntoRequest<super::SqlUsersDeleteRequest>,
        ) -> Result<tonic::Response<super::Operation>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.sql.v1beta4.SqlUsersService/Delete",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a new user in a Cloud SQL instance."]
        pub async fn insert(
            &mut self,
            request: impl tonic::IntoRequest<super::SqlUsersInsertRequest>,
        ) -> Result<tonic::Response<super::Operation>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.sql.v1beta4.SqlUsersService/Insert",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists users in the specified Cloud SQL instance."]
        pub async fn list(
            &mut self,
            request: impl tonic::IntoRequest<super::SqlUsersListRequest>,
        ) -> Result<tonic::Response<super::UsersListResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.sql.v1beta4.SqlUsersService/List",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates an existing user in a Cloud SQL instance."]
        pub async fn update(
            &mut self,
            request: impl tonic::IntoRequest<super::SqlUsersUpdateRequest>,
        ) -> Result<tonic::Response<super::Operation>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.sql.v1beta4.SqlUsersService/Update",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
