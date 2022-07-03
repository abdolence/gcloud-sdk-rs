/// SSL configuration information.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SslConfig {
    /// Output only. The ssl config type according to 'client_key', 'client_certificate' and
    /// 'ca_certificate'.
    #[prost(enumeration = "ssl_config::SslType", tag = "1")]
    pub r#type: i32,
    /// Input only. The unencrypted PKCS#1 or PKCS#8 PEM-encoded private key associated with
    /// the Client Certificate. If this field is used then the 'client_certificate'
    /// field is mandatory.
    #[prost(string, tag = "2")]
    pub client_key: ::prost::alloc::string::String,
    /// Input only. The x509 PEM-encoded certificate that will be used by the replica to
    /// authenticate against the source database server.If this field is used then
    /// the 'client_key' field is mandatory.
    #[prost(string, tag = "3")]
    pub client_certificate: ::prost::alloc::string::String,
    /// Required. Input only. The x509 PEM-encoded certificate of the CA that signed the source database
    /// server's certificate. The replica will use this certificate to verify
    /// it's connecting to the right host.
    #[prost(string, tag = "4")]
    pub ca_certificate: ::prost::alloc::string::String,
}
/// Nested message and enum types in `SslConfig`.
pub mod ssl_config {
    /// Specifies The kind of ssl configuration used.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum SslType {
        /// Unspecified.
        Unspecified = 0,
        /// Only 'ca_certificate' specified.
        ServerOnly = 1,
        /// Both server ('ca_certificate'), and client ('client_key',
        /// 'client_certificate') specified.
        ServerClient = 2,
    }
}
/// Specifies connection parameters required specifically for MySQL databases.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MySqlConnectionProfile {
    /// Required. The IP or hostname of the source MySQL database.
    #[prost(string, tag = "1")]
    pub host: ::prost::alloc::string::String,
    /// Required. The network port of the source MySQL database.
    #[prost(int32, tag = "2")]
    pub port: i32,
    /// Required. The username that Database Migration Service will use to connect to the
    /// database. The value is encrypted when stored in Database Migration Service.
    #[prost(string, tag = "3")]
    pub username: ::prost::alloc::string::String,
    /// Required. Input only. The password for the user that Database Migration Service will be using to
    /// connect to the database. This field is not returned on request, and the
    /// value is encrypted when stored in Database Migration Service.
    #[prost(string, tag = "4")]
    pub password: ::prost::alloc::string::String,
    /// Output only. Indicates If this connection profile password is stored.
    #[prost(bool, tag = "5")]
    pub password_set: bool,
    /// SSL configuration for the destination to connect to the source database.
    #[prost(message, optional, tag = "6")]
    pub ssl: ::core::option::Option<SslConfig>,
    /// If the source is a Cloud SQL database, use this field to
    /// provide the Cloud SQL instance ID of the source.
    #[prost(string, tag = "7")]
    pub cloud_sql_id: ::prost::alloc::string::String,
}
/// Specifies connection parameters required specifically for PostgreSQL
/// databases.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PostgreSqlConnectionProfile {
    /// Required. The IP or hostname of the source PostgreSQL database.
    #[prost(string, tag = "1")]
    pub host: ::prost::alloc::string::String,
    /// Required. The network port of the source PostgreSQL database.
    #[prost(int32, tag = "2")]
    pub port: i32,
    /// Required. The username that Database Migration Service will use to connect to the
    /// database. The value is encrypted when stored in Database Migration Service.
    #[prost(string, tag = "3")]
    pub username: ::prost::alloc::string::String,
    /// Required. Input only. The password for the user that Database Migration Service will be using to
    /// connect to the database. This field is not returned on request, and the
    /// value is encrypted when stored in Database Migration Service.
    #[prost(string, tag = "4")]
    pub password: ::prost::alloc::string::String,
    /// Output only. Indicates If this connection profile password is stored.
    #[prost(bool, tag = "5")]
    pub password_set: bool,
    /// SSL configuration for the destination to connect to the source database.
    #[prost(message, optional, tag = "6")]
    pub ssl: ::core::option::Option<SslConfig>,
    /// If the source is a Cloud SQL database, use this field to
    /// provide the Cloud SQL instance ID of the source.
    #[prost(string, tag = "7")]
    pub cloud_sql_id: ::prost::alloc::string::String,
}
/// Specifies required connection parameters, and, optionally, the parameters
/// required to create a Cloud SQL destination database instance.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CloudSqlConnectionProfile {
    /// Output only. The Cloud SQL instance ID that this connection profile is associated with.
    #[prost(string, tag = "1")]
    pub cloud_sql_id: ::prost::alloc::string::String,
    /// Immutable. Metadata used to create the destination Cloud SQL database.
    #[prost(message, optional, tag = "2")]
    pub settings: ::core::option::Option<CloudSqlSettings>,
    /// Output only. The Cloud SQL database instance's private IP.
    #[prost(string, tag = "3")]
    pub private_ip: ::prost::alloc::string::String,
    /// Output only. The Cloud SQL database instance's public IP.
    #[prost(string, tag = "4")]
    pub public_ip: ::prost::alloc::string::String,
}
/// An entry for an Access Control list.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SqlAclEntry {
    /// The allowlisted value for the access control list.
    #[prost(string, tag = "1")]
    pub value: ::prost::alloc::string::String,
    /// A label to identify this entry.
    #[prost(string, tag = "3")]
    pub label: ::prost::alloc::string::String,
    /// The access control entry entry expiration.
    #[prost(oneof = "sql_acl_entry::Expiration", tags = "10, 11")]
    pub expiration: ::core::option::Option<sql_acl_entry::Expiration>,
}
/// Nested message and enum types in `SqlAclEntry`.
pub mod sql_acl_entry {
    /// The access control entry entry expiration.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Expiration {
        /// The time when this access control entry expires in
        /// [RFC 3339](<https://tools.ietf.org/html/rfc3339>) format, for example:
        /// `2012-11-15T16:19:00.094Z`.
        #[prost(message, tag = "10")]
        ExpireTime(::prost_types::Timestamp),
        /// Input only. The time-to-leave of this access control entry.
        #[prost(message, tag = "11")]
        Ttl(::prost_types::Duration),
    }
}
/// IP Management configuration.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SqlIpConfig {
    /// Whether the instance should be assigned an IPv4 address or not.
    #[prost(message, optional, tag = "1")]
    pub enable_ipv4: ::core::option::Option<bool>,
    /// The resource link for the VPC network from which the Cloud SQL instance is
    /// accessible for private IP. For example,
    /// `projects/myProject/global/networks/default`. This setting can
    /// be updated, but it cannot be removed after it is set.
    #[prost(string, tag = "2")]
    pub private_network: ::prost::alloc::string::String,
    /// Whether SSL connections over IP should be enforced or not.
    #[prost(message, optional, tag = "3")]
    pub require_ssl: ::core::option::Option<bool>,
    /// The list of external networks that are allowed to connect to the instance
    /// using the IP. See
    /// <https://en.wikipedia.org/wiki/CIDR_notation#CIDR_notation,> also known as
    /// 'slash' notation (e.g. `192.168.100.0/24`).
    #[prost(message, repeated, tag = "4")]
    pub authorized_networks: ::prost::alloc::vec::Vec<SqlAclEntry>,
}
/// Settings for creating a Cloud SQL database instance.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CloudSqlSettings {
    /// The database engine type and version.
    #[prost(enumeration = "cloud_sql_settings::SqlDatabaseVersion", tag = "1")]
    pub database_version: i32,
    /// The resource labels for a Cloud SQL instance to use to annotate any related
    /// underlying resources such as Compute Engine VMs.
    /// An object containing a list of "key": "value" pairs.
    ///
    /// Example: `{ "name": "wrench", "mass": "18kg", "count": "3" }`.
    #[prost(map = "string, string", tag = "2")]
    pub user_labels:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// The tier (or machine type) for this instance, for example:
    /// `db-n1-standard-1` (MySQL instances) or
    /// `db-custom-1-3840` (PostgreSQL instances).
    /// For more information, see
    /// [Cloud SQL Instance
    /// Settings](<https://cloud.google.com/sql/docs/mysql/instance-settings>).
    #[prost(string, tag = "3")]
    pub tier: ::prost::alloc::string::String,
    /// The maximum size to which storage capacity can be automatically increased.
    /// The default value is 0, which specifies that there is no limit.
    #[prost(message, optional, tag = "4")]
    pub storage_auto_resize_limit: ::core::option::Option<i64>,
    /// The activation policy specifies when the instance is activated; it is
    /// applicable only when the instance state is 'RUNNABLE'. Valid values:
    ///
    /// 'ALWAYS': The instance is on, and remains so even in
    /// the absence of connection requests.
    ///
    /// `NEVER`: The instance is off; it is not activated, even if a
    /// connection request arrives.
    #[prost(enumeration = "cloud_sql_settings::SqlActivationPolicy", tag = "5")]
    pub activation_policy: i32,
    /// The settings for IP Management. This allows to enable or disable the
    /// instance IP and manage which external networks can connect to the instance.
    /// The IPv4 address cannot be disabled.
    #[prost(message, optional, tag = "6")]
    pub ip_config: ::core::option::Option<SqlIpConfig>,
    /// [default: ON] If you enable this setting, Cloud SQL checks your available
    /// storage every 30 seconds. If the available storage falls below a threshold
    /// size, Cloud SQL automatically adds additional storage capacity. If the
    /// available storage repeatedly falls below the threshold size, Cloud SQL
    /// continues to add storage until it reaches the maximum of 30 TB.
    #[prost(message, optional, tag = "7")]
    pub auto_storage_increase: ::core::option::Option<bool>,
    /// The database flags passed to the Cloud SQL instance at startup.
    /// An object containing a list of "key": value pairs.
    /// Example: { "name": "wrench", "mass": "1.3kg", "count": "3" }.
    #[prost(map = "string, string", tag = "8")]
    pub database_flags:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// The type of storage: `PD_SSD` (default) or `PD_HDD`.
    #[prost(enumeration = "cloud_sql_settings::SqlDataDiskType", tag = "9")]
    pub data_disk_type: i32,
    /// The storage capacity available to the database, in GB.
    /// The minimum (and default) size is 10GB.
    #[prost(message, optional, tag = "10")]
    pub data_disk_size_gb: ::core::option::Option<i64>,
    /// The Google Cloud Platform zone where your Cloud SQL datdabse instance is
    /// located.
    #[prost(string, tag = "11")]
    pub zone: ::prost::alloc::string::String,
    /// The Database Migration Service source connection profile ID,
    /// in the format:
    /// `projects/my_project_name/locations/us-central1/connectionProfiles/connection_profile_ID`
    #[prost(string, tag = "12")]
    pub source_id: ::prost::alloc::string::String,
    /// Input only. Initial root password.
    #[prost(string, tag = "13")]
    pub root_password: ::prost::alloc::string::String,
    /// Output only. Indicates If this connection profile root password is stored.
    #[prost(bool, tag = "14")]
    pub root_password_set: bool,
    /// The Cloud SQL default instance level collation.
    #[prost(string, tag = "15")]
    pub collation: ::prost::alloc::string::String,
}
/// Nested message and enum types in `CloudSqlSettings`.
pub mod cloud_sql_settings {
    /// Specifies when the instance should be activated.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum SqlActivationPolicy {
        /// unspecified policy.
        Unspecified = 0,
        /// The instance is always up and running.
        Always = 1,
        /// The instance should never spin up.
        Never = 2,
    }
    /// The storage options for Cloud SQL databases.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum SqlDataDiskType {
        /// Unspecified.
        Unspecified = 0,
        /// SSD disk.
        PdSsd = 1,
        /// HDD disk.
        PdHdd = 2,
    }
    /// The database engine type and version.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum SqlDatabaseVersion {
        /// Unspecified version.
        Unspecified = 0,
        /// MySQL 5.6.
        Mysql56 = 1,
        /// MySQL 5.7.
        Mysql57 = 2,
        /// PostgreSQL 9.6.
        Postgres96 = 3,
        /// PostgreSQL 11.
        Postgres11 = 4,
        /// PostgreSQL 10.
        Postgres10 = 5,
        /// MySQL 8.0.
        Mysql80 = 6,
        /// PostgreSQL 12.
        Postgres12 = 7,
        /// PostgreSQL 13.
        Postgres13 = 8,
    }
}
/// The source database will allow incoming connections from the destination
/// database's public IP. You can retrieve the Cloud SQL instance's public IP
/// from the Cloud SQL console or using Cloud SQL APIs. No additional
/// configuration is required.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StaticIpConnectivity {}
/// The details needed to configure a reverse SSH tunnel between the source and
/// destination databases. These details will be used when calling the
/// generateSshScript method (see
/// <https://cloud.google.com/database-migration/docs/reference/rest/v1/projects.locations.migrationJobs/generateSshScript>)
/// to produce the script that will help set up the reverse SSH tunnel, and to
/// set up the VPC peering between the Cloud SQL private network and the VPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReverseSshConnectivity {
    /// Required. The IP of the virtual machine (Compute Engine) used as the bastion server
    /// for the SSH tunnel.
    #[prost(string, tag = "1")]
    pub vm_ip: ::prost::alloc::string::String,
    /// Required. The forwarding port of the virtual machine (Compute Engine) used as the
    /// bastion server for the SSH tunnel.
    #[prost(int32, tag = "2")]
    pub vm_port: i32,
    /// The name of the virtual machine (Compute Engine) used as the bastion server
    /// for the SSH tunnel.
    #[prost(string, tag = "3")]
    pub vm: ::prost::alloc::string::String,
    /// The name of the VPC to peer with the Cloud SQL private network.
    #[prost(string, tag = "4")]
    pub vpc: ::prost::alloc::string::String,
}
/// The details of the VPC where the source database is located in Google Cloud.
/// We will use this information to set up the VPC peering connection between
/// Cloud SQL and this VPC.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VpcPeeringConnectivity {
    /// The name of the VPC network to peer with the Cloud SQL private network.
    #[prost(string, tag = "1")]
    pub vpc: ::prost::alloc::string::String,
}
/// A message defining the database engine and provider.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DatabaseType {
    /// The database provider.
    #[prost(enumeration = "DatabaseProvider", tag = "1")]
    pub provider: i32,
    /// The database engine.
    #[prost(enumeration = "DatabaseEngine", tag = "2")]
    pub engine: i32,
}
/// Represents a Database Migration Service migration job object.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MigrationJob {
    /// The name (URI) of this migration job resource, in the form of:
    /// projects/{project}/locations/{location}/instances/{instance}.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. The timestamp when the migration job resource was created.
    /// A timestamp in RFC3339 UTC "Zulu" format, accurate to nanoseconds.
    /// Example: "2014-10-02T15:01:23.045123456Z".
    #[prost(message, optional, tag = "2")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The timestamp when the migration job resource was last updated.
    /// A timestamp in RFC3339 UTC "Zulu" format, accurate to nanoseconds.
    /// Example: "2014-10-02T15:01:23.045123456Z".
    #[prost(message, optional, tag = "3")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The resource labels for migration job to use to annotate any related
    /// underlying resources such as Compute Engine VMs. An object containing a
    /// list of "key": "value" pairs.
    ///
    /// Example: `{ "name": "wrench", "mass": "1.3kg", "count": "3" }`.
    #[prost(map = "string, string", tag = "4")]
    pub labels:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// The migration job display name.
    #[prost(string, tag = "5")]
    pub display_name: ::prost::alloc::string::String,
    /// The current migration job state.
    #[prost(enumeration = "migration_job::State", tag = "6")]
    pub state: i32,
    /// Output only. The current migration job phase.
    #[prost(enumeration = "migration_job::Phase", tag = "7")]
    pub phase: i32,
    /// Required. The migration job type.
    #[prost(enumeration = "migration_job::Type", tag = "8")]
    pub r#type: i32,
    /// The path to the dump file in Google Cloud Storage,
    /// in the format: (gs://\[BUCKET_NAME]/[OBJECT_NAME\]).
    #[prost(string, tag = "9")]
    pub dump_path: ::prost::alloc::string::String,
    /// Required. The resource name (URI) of the source connection profile.
    #[prost(string, tag = "10")]
    pub source: ::prost::alloc::string::String,
    /// Required. The resource name (URI) of the destination connection profile.
    #[prost(string, tag = "11")]
    pub destination: ::prost::alloc::string::String,
    /// Output only. The duration of the migration job (in seconds). A duration in seconds
    /// with up to nine fractional digits, terminated by 's'. Example: "3.5s".
    #[prost(message, optional, tag = "12")]
    pub duration: ::core::option::Option<::prost_types::Duration>,
    /// Output only. The error details in case of state FAILED.
    #[prost(message, optional, tag = "13")]
    pub error: ::core::option::Option<super::super::super::rpc::Status>,
    /// The database engine type and provider of the source.
    #[prost(message, optional, tag = "14")]
    pub source_database: ::core::option::Option<DatabaseType>,
    /// The database engine type and provider of the destination.
    #[prost(message, optional, tag = "15")]
    pub destination_database: ::core::option::Option<DatabaseType>,
    /// Output only. If the migration job is completed, the time when it was completed.
    #[prost(message, optional, tag = "16")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The connectivity method.
    #[prost(oneof = "migration_job::Connectivity", tags = "101, 102, 103")]
    pub connectivity: ::core::option::Option<migration_job::Connectivity>,
}
/// Nested message and enum types in `MigrationJob`.
pub mod migration_job {
    /// The current migration job states.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// The state of the migration job is unknown.
        Unspecified = 0,
        /// The migration job is down for maintenance.
        Maintenance = 1,
        /// The migration job is in draft mode and no resources are created.
        Draft = 2,
        /// The migration job is being created.
        Creating = 3,
        /// The migration job is created and not started.
        NotStarted = 4,
        /// The migration job is running.
        Running = 5,
        /// The migration job failed.
        Failed = 6,
        /// The migration job has been completed.
        Completed = 7,
        /// The migration job is being deleted.
        Deleting = 8,
        /// The migration job is being stopped.
        Stopping = 9,
        /// The migration job is currently stopped.
        Stopped = 10,
        /// The migration job has been deleted.
        Deleted = 11,
        /// The migration job is being updated.
        Updating = 12,
        /// The migration job is starting.
        Starting = 13,
        /// The migration job is restarting.
        Restarting = 14,
        /// The migration job is resuming.
        Resuming = 15,
    }
    /// The current migration job phase.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Phase {
        /// The phase of the migration job is unknown.
        Unspecified = 0,
        /// The migration job is in the full dump phase.
        FullDump = 1,
        /// The migration job is CDC phase.
        Cdc = 2,
        /// The migration job is running the promote phase.
        PromoteInProgress = 3,
        /// Only RDS flow - waiting for source writes to stop
        WaitingForSourceWritesToStop = 4,
        /// Only RDS flow - the sources writes stopped, waiting for dump to begin
        PreparingTheDump = 5,
    }
    /// The type of migration job (one-time or continuous).
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Type {
        /// The type of the migration job is unknown.
        Unspecified = 0,
        /// The migration job is a one time migration.
        OneTime = 1,
        /// The migration job is a continuous migration.
        Continuous = 2,
    }
    /// The connectivity method.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Connectivity {
        /// The details needed to communicate to the source over Reverse SSH
        /// tunnel connectivity.
        #[prost(message, tag = "101")]
        ReverseSshConnectivity(super::ReverseSshConnectivity),
        /// The details of the VPC network that the source database is located in.
        #[prost(message, tag = "102")]
        VpcPeeringConnectivity(super::VpcPeeringConnectivity),
        /// static ip connectivity data (default, no additional details needed).
        #[prost(message, tag = "103")]
        StaticIpConnectivity(super::StaticIpConnectivity),
    }
}
/// A connection profile definition.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConnectionProfile {
    /// The name of this connection profile resource in the form of
    /// projects/{project}/locations/{location}/instances/{instance}.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. The timestamp when the resource was created.
    /// A timestamp in RFC3339 UTC "Zulu" format, accurate to nanoseconds.
    /// Example: "2014-10-02T15:01:23.045123456Z".
    #[prost(message, optional, tag = "2")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The timestamp when the resource was last updated.
    /// A timestamp in RFC3339 UTC "Zulu" format, accurate to nanoseconds.
    /// Example: "2014-10-02T15:01:23.045123456Z".
    #[prost(message, optional, tag = "3")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The resource labels for connection profile to use to annotate any related
    /// underlying resources such as Compute Engine VMs. An object containing a
    /// list of "key": "value" pairs.
    ///
    /// Example: `{ "name": "wrench", "mass": "1.3kg", "count": "3" }`.
    #[prost(map = "string, string", tag = "4")]
    pub labels:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// The current connection profile state (e.g. DRAFT, READY, or FAILED).
    #[prost(enumeration = "connection_profile::State", tag = "5")]
    pub state: i32,
    /// The connection profile display name.
    #[prost(string, tag = "6")]
    pub display_name: ::prost::alloc::string::String,
    /// Output only. The error details in case of state FAILED.
    #[prost(message, optional, tag = "7")]
    pub error: ::core::option::Option<super::super::super::rpc::Status>,
    /// The database provider.
    #[prost(enumeration = "DatabaseProvider", tag = "8")]
    pub provider: i32,
    /// The connection profile definition.
    #[prost(
        oneof = "connection_profile::ConnectionProfile",
        tags = "100, 101, 102"
    )]
    pub connection_profile: ::core::option::Option<connection_profile::ConnectionProfile>,
}
/// Nested message and enum types in `ConnectionProfile`.
pub mod connection_profile {
    /// The current connection profile state (e.g. DRAFT, READY, or FAILED).
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// The state of the connection profile is unknown.
        Unspecified = 0,
        /// The connection profile is in draft mode and fully editable.
        Draft = 1,
        /// The connection profile is being created.
        Creating = 2,
        /// The connection profile is ready.
        Ready = 3,
        /// The connection profile is being updated.
        Updating = 4,
        /// The connection profile is being deleted.
        Deleting = 5,
        /// The connection profile has been deleted.
        Deleted = 6,
        /// The last action on the connection profile failed.
        Failed = 7,
    }
    /// The connection profile definition.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ConnectionProfile {
        /// A MySQL database connection profile.
        #[prost(message, tag = "100")]
        Mysql(super::MySqlConnectionProfile),
        /// A PostgreSQL database connection profile.
        #[prost(message, tag = "101")]
        Postgresql(super::PostgreSqlConnectionProfile),
        /// A CloudSQL database connection profile.
        #[prost(message, tag = "102")]
        Cloudsql(super::CloudSqlConnectionProfile),
    }
}
/// Error message of a verification Migration job.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MigrationJobVerificationError {
    /// Output only. An instance of ErrorCode specifying the error that occurred.
    #[prost(enumeration = "migration_job_verification_error::ErrorCode", tag = "1")]
    pub error_code: i32,
    /// Output only. A formatted message with further details about the error and a CTA.
    #[prost(string, tag = "2")]
    pub error_message: ::prost::alloc::string::String,
    /// Output only. A specific detailed error message, if supplied by the engine.
    #[prost(string, tag = "3")]
    pub error_detail_message: ::prost::alloc::string::String,
}
/// Nested message and enum types in `MigrationJobVerificationError`.
pub mod migration_job_verification_error {
    /// A general error code describing the type of error that occurred.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ErrorCode {
        /// An unknown error occurred
        Unspecified = 0,
        /// We failed to connect to one of the connection profile.
        ConnectionFailure = 1,
        /// We failed to authenticate to one of the connection profile.
        AuthenticationFailure = 2,
        /// One of the involved connection profiles has an invalid configuration.
        InvalidConnectionProfileConfig = 3,
        /// The versions of the source and the destination are incompatible.
        VersionIncompatibility = 4,
        /// The types of the source and the destination are incompatible.
        ConnectionProfileTypesIncompatibility = 5,
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
        /// versions.
        UnsupportedExtensions = 14,
        /// Unsupported migration type.
        UnsupportedMigrationType = 15,
        /// Invalid RDS logical replication.
        InvalidRdsLogicalReplication = 16,
        /// The gtid_mode is not supported, applicable for MySQL.
        UnsupportedGtidMode = 17,
        /// The table definition is not support due to missing primary key or replica
        /// identity.
        UnsupportedTableDefinition = 18,
        /// The definer is not supported.
        UnsupportedDefiner = 19,
        /// Migration is already running at the time of restart request.
        CantRestartRunningMigration = 21,
    }
}
/// The database engine types.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DatabaseEngine {
    /// The source database engine of the migration job is unknown.
    Unspecified = 0,
    /// The source engine is MySQL.
    Mysql = 1,
    /// The source engine is PostgreSQL.
    Postgresql = 2,
}
/// The database providers.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DatabaseProvider {
    /// The database provider is unknown.
    Unspecified = 0,
    /// CloudSQL runs the database.
    Cloudsql = 1,
    /// RDS runs the database.
    Rds = 2,
}
/// Retrieve a list of all migration jobs in a given project and location.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListMigrationJobsRequest {
    /// Required. The parent, which owns this collection of migrationJobs.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of migration jobs to return. The service may return
    /// fewer than this value. If unspecified, at most 50 migration jobs will be
    /// returned. The maximum value is 1000; values above 1000 will be coerced to
    /// 1000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The nextPageToken value received in the previous call to
    /// migrationJobs.list, used in the subsequent request to retrieve the next
    /// page of results. On first call this should be left blank. When paginating,
    /// all other parameters provided to migrationJobs.list must match the call
    /// that provided the page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// A filter expression that filters migration jobs listed in the response.
    /// The expression must specify the field name, a comparison operator, and the
    /// value that you want to use for filtering. The value must be a string,
    /// a number, or a boolean. The comparison operator must be
    /// either =, !=, >, or <. For example, list migration jobs created this year
    /// by specifying **createTime %gt; 2020-01-01T00:00:00.000000000Z.**
    /// You can also filter nested fields. For example, you could specify
    /// **reverseSshConnectivity.vmIp = "1.2.3.4"** to select all migration
    /// jobs connecting through the specific SSH tunnel bastion.
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Sort the results based on the migration job name.
    /// Valid values are: "name", "name asc", and "name desc".
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
/// Response message for 'ListMigrationJobs' request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListMigrationJobsResponse {
    /// The list of migration jobs objects.
    #[prost(message, repeated, tag = "1")]
    pub migration_jobs: ::prost::alloc::vec::Vec<MigrationJob>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Request message for 'GetMigrationJob' request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMigrationJobRequest {
    /// Required. Name of the migration job resource to get.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message to create a new Database Migration Service migration job
/// in the specified project and region.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateMigrationJobRequest {
    /// Required. The parent, which owns this collection of migration jobs.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The ID of the instance to create.
    #[prost(string, tag = "2")]
    pub migration_job_id: ::prost::alloc::string::String,
    /// Required. Represents a [migration
    /// job](<https://cloud.google.com/database-migration/docs/reference/rest/v1/projects.locations.migrationJobs>)
    /// object.
    #[prost(message, optional, tag = "3")]
    pub migration_job: ::core::option::Option<MigrationJob>,
    /// A unique id used to identify the request. If the server receives two
    /// requests with the same id, then the second request will be ignored.
    ///
    /// It is recommended to always set this value to a UUID.
    ///
    /// The id must contain only letters (a-z, A-Z), numbers (0-9), underscores
    /// (_), and hyphens (-). The maximum length is 40 characters.
    #[prost(string, tag = "4")]
    pub request_id: ::prost::alloc::string::String,
}
/// Request message for 'UpdateMigrationJob' request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateMigrationJobRequest {
    /// Required. Field mask is used to specify the fields to be overwritten in the
    /// migration job resource by the update.
    #[prost(message, optional, tag = "1")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Required. The migration job parameters to update.
    #[prost(message, optional, tag = "2")]
    pub migration_job: ::core::option::Option<MigrationJob>,
    /// A unique id used to identify the request. If the server receives two
    /// requests with the same id, then the second request will be ignored.
    ///
    /// It is recommended to always set this value to a UUID.
    ///
    /// The id must contain only letters (a-z, A-Z), numbers (0-9), underscores
    /// (_), and hyphens (-). The maximum length is 40 characters.
    #[prost(string, tag = "3")]
    pub request_id: ::prost::alloc::string::String,
}
/// Request message for 'DeleteMigrationJob' request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteMigrationJobRequest {
    /// Required. Name of the migration job resource to delete.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// A unique id used to identify the request. If the server receives two
    /// requests with the same id, then the second request will be ignored.
    ///
    /// It is recommended to always set this value to a UUID.
    ///
    /// The id must contain only letters (a-z, A-Z), numbers (0-9), underscores
    /// (_), and hyphens (-). The maximum length is 40 characters.
    #[prost(string, tag = "2")]
    pub request_id: ::prost::alloc::string::String,
    /// The destination CloudSQL connection profile is always deleted with the
    /// migration job. In case of force delete, the destination CloudSQL replica
    /// database is also deleted.
    #[prost(bool, tag = "3")]
    pub force: bool,
}
/// Request message for 'StartMigrationJob' request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StartMigrationJobRequest {
    /// Name of the migration job resource to start.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for 'StopMigrationJob' request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StopMigrationJobRequest {
    /// Name of the migration job resource to stop.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for 'ResumeMigrationJob' request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResumeMigrationJobRequest {
    /// Name of the migration job resource to resume.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for 'PromoteMigrationJob' request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PromoteMigrationJobRequest {
    /// Name of the migration job resource to promote.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for 'VerifyMigrationJob' request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VerifyMigrationJobRequest {
    /// Name of the migration job resource to verify.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for 'RestartMigrationJob' request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RestartMigrationJobRequest {
    /// Name of the migration job resource to restart.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for 'GenerateSshScript' request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateSshScriptRequest {
    /// Name of the migration job resource to generate the SSH script.
    #[prost(string, tag = "1")]
    pub migration_job: ::prost::alloc::string::String,
    /// Required. Bastion VM Instance name to use or to create.
    #[prost(string, tag = "2")]
    pub vm: ::prost::alloc::string::String,
    /// The port that will be open on the bastion host
    #[prost(int32, tag = "3")]
    pub vm_port: i32,
    /// The VM configuration
    #[prost(oneof = "generate_ssh_script_request::VmConfig", tags = "100, 101")]
    pub vm_config: ::core::option::Option<generate_ssh_script_request::VmConfig>,
}
/// Nested message and enum types in `GenerateSshScriptRequest`.
pub mod generate_ssh_script_request {
    /// The VM configuration
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum VmConfig {
        /// The VM creation configuration
        #[prost(message, tag = "100")]
        VmCreationConfig(super::VmCreationConfig),
        /// The VM selection configuration
        #[prost(message, tag = "101")]
        VmSelectionConfig(super::VmSelectionConfig),
    }
}
/// VM creation configuration message
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VmCreationConfig {
    /// Required. VM instance machine type to create.
    #[prost(string, tag = "1")]
    pub vm_machine_type: ::prost::alloc::string::String,
    /// The Google Cloud Platform zone to create the VM in.
    #[prost(string, tag = "2")]
    pub vm_zone: ::prost::alloc::string::String,
    /// The subnet name the vm needs to be created in.
    #[prost(string, tag = "3")]
    pub subnet: ::prost::alloc::string::String,
}
/// VM selection configuration message
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VmSelectionConfig {
    /// Required. The Google Cloud Platform zone the VM is located.
    #[prost(string, tag = "1")]
    pub vm_zone: ::prost::alloc::string::String,
}
/// Response message for 'GenerateSshScript' request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SshScript {
    /// The ssh configuration script.
    #[prost(string, tag = "1")]
    pub script: ::prost::alloc::string::String,
}
/// Request message for 'ListConnectionProfiles' request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListConnectionProfilesRequest {
    /// Required. The parent, which owns this collection of connection profiles.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of connection profiles to return. The service may return
    /// fewer than this value. If unspecified, at most 50 connection profiles will
    /// be returned. The maximum value is 1000; values above 1000 will be coerced
    /// to 1000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A page token, received from a previous `ListConnectionProfiles` call.
    /// Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to `ListConnectionProfiles`
    /// must match the call that provided the page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// A filter expression that filters connection profiles listed in the
    /// response. The expression must specify the field name, a comparison
    /// operator, and the value that you want to use for filtering. The value must
    /// be a string, a number, or a boolean. The comparison operator must be either
    /// =, !=, >, or <. For example, list connection profiles created this year by
    /// specifying **createTime %gt; 2020-01-01T00:00:00.000000000Z**. You can
    /// also filter nested fields. For example, you could specify **mySql.username
    /// = %lt;my_username%gt;** to list all connection profiles configured to
    /// connect with a specific username.
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// the order by fields for the result.
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
/// Response message for 'ListConnectionProfiles' request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListConnectionProfilesResponse {
    /// The response list of connection profiles.
    #[prost(message, repeated, tag = "1")]
    pub connection_profiles: ::prost::alloc::vec::Vec<ConnectionProfile>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Request message for 'GetConnectionProfile' request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetConnectionProfileRequest {
    /// Required. Name of the connection profile resource to get.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for 'CreateConnectionProfile' request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateConnectionProfileRequest {
    /// Required. The parent, which owns this collection of connection profiles.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The connection profile identifier.
    #[prost(string, tag = "2")]
    pub connection_profile_id: ::prost::alloc::string::String,
    /// Required. The create request body including the connection profile data
    #[prost(message, optional, tag = "3")]
    pub connection_profile: ::core::option::Option<ConnectionProfile>,
    /// A unique id used to identify the request. If the server receives two
    /// requests with the same id, then the second request will be ignored.
    ///
    /// It is recommended to always set this value to a UUID.
    ///
    /// The id must contain only letters (a-z, A-Z), numbers (0-9), underscores
    /// (_), and hyphens (-). The maximum length is 40 characters.
    #[prost(string, tag = "4")]
    pub request_id: ::prost::alloc::string::String,
}
/// Request message for 'UpdateConnectionProfile' request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateConnectionProfileRequest {
    /// Required. Field mask is used to specify the fields to be overwritten in the
    /// connection profile resource by the update.
    #[prost(message, optional, tag = "1")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Required. The connection profile parameters to update.
    #[prost(message, optional, tag = "2")]
    pub connection_profile: ::core::option::Option<ConnectionProfile>,
    /// A unique id used to identify the request. If the server receives two
    /// requests with the same id, then the second request will be ignored.
    ///
    /// It is recommended to always set this value to a UUID.
    ///
    /// The id must contain only letters (a-z, A-Z), numbers (0-9), underscores
    /// (_), and hyphens (-). The maximum length is 40 characters.
    #[prost(string, tag = "3")]
    pub request_id: ::prost::alloc::string::String,
}
/// Request message for 'DeleteConnectionProfile' request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteConnectionProfileRequest {
    /// Required. Name of the connection profile resource to delete.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// A unique id used to identify the request. If the server receives two
    /// requests with the same id, then the second request will be ignored.
    ///
    /// It is recommended to always set this value to a UUID.
    ///
    /// The id must contain only letters (a-z, A-Z), numbers (0-9), underscores
    /// (_), and hyphens (-). The maximum length is 40 characters.
    #[prost(string, tag = "2")]
    pub request_id: ::prost::alloc::string::String,
    /// In case of force delete, the CloudSQL replica database is also deleted
    /// (only for CloudSQL connection profile).
    #[prost(bool, tag = "3")]
    pub force: bool,
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
#[doc = r" Generated client implementations."]
pub mod data_migration_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Database Migration service"]
    #[derive(Debug, Clone)]
    pub struct DataMigrationServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> DataMigrationServiceClient<T>
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
        ) -> DataMigrationServiceClient<InterceptedService<T, F>>
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
            DataMigrationServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Lists migration jobs in a given project and location."]
        pub async fn list_migration_jobs(
            &mut self,
            request: impl tonic::IntoRequest<super::ListMigrationJobsRequest>,
        ) -> Result<tonic::Response<super::ListMigrationJobsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.clouddms.v1.DataMigrationService/ListMigrationJobs",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets details of a single migration job."]
        pub async fn get_migration_job(
            &mut self,
            request: impl tonic::IntoRequest<super::GetMigrationJobRequest>,
        ) -> Result<tonic::Response<super::MigrationJob>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.clouddms.v1.DataMigrationService/GetMigrationJob",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a new migration job in a given project and location."]
        pub async fn create_migration_job(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateMigrationJobRequest>,
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
                "/google.cloud.clouddms.v1.DataMigrationService/CreateMigrationJob",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates the parameters of a single migration job."]
        pub async fn update_migration_job(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateMigrationJobRequest>,
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
                "/google.cloud.clouddms.v1.DataMigrationService/UpdateMigrationJob",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes a single migration job."]
        pub async fn delete_migration_job(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteMigrationJobRequest>,
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
                "/google.cloud.clouddms.v1.DataMigrationService/DeleteMigrationJob",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Start an already created migration job."]
        pub async fn start_migration_job(
            &mut self,
            request: impl tonic::IntoRequest<super::StartMigrationJobRequest>,
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
                "/google.cloud.clouddms.v1.DataMigrationService/StartMigrationJob",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Stops a running migration job."]
        pub async fn stop_migration_job(
            &mut self,
            request: impl tonic::IntoRequest<super::StopMigrationJobRequest>,
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
                "/google.cloud.clouddms.v1.DataMigrationService/StopMigrationJob",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Resume a migration job that is currently stopped and is resumable (was"]
        #[doc = " stopped during CDC phase)."]
        pub async fn resume_migration_job(
            &mut self,
            request: impl tonic::IntoRequest<super::ResumeMigrationJobRequest>,
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
                "/google.cloud.clouddms.v1.DataMigrationService/ResumeMigrationJob",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Promote a migration job, stopping replication to the destination and"]
        #[doc = " promoting the destination to be a standalone database."]
        pub async fn promote_migration_job(
            &mut self,
            request: impl tonic::IntoRequest<super::PromoteMigrationJobRequest>,
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
                "/google.cloud.clouddms.v1.DataMigrationService/PromoteMigrationJob",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Verify a migration job, making sure the destination can reach the source"]
        #[doc = " and that all configuration and prerequisites are met."]
        pub async fn verify_migration_job(
            &mut self,
            request: impl tonic::IntoRequest<super::VerifyMigrationJobRequest>,
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
                "/google.cloud.clouddms.v1.DataMigrationService/VerifyMigrationJob",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Restart a stopped or failed migration job, resetting the destination"]
        #[doc = " instance to its original state and starting the migration process from"]
        #[doc = " scratch."]
        pub async fn restart_migration_job(
            &mut self,
            request: impl tonic::IntoRequest<super::RestartMigrationJobRequest>,
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
                "/google.cloud.clouddms.v1.DataMigrationService/RestartMigrationJob",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Generate a SSH configuration script to configure the reverse SSH"]
        #[doc = " connectivity."]
        pub async fn generate_ssh_script(
            &mut self,
            request: impl tonic::IntoRequest<super::GenerateSshScriptRequest>,
        ) -> Result<tonic::Response<super::SshScript>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.clouddms.v1.DataMigrationService/GenerateSshScript",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Retrieve a list of all connection profiles in a given project and location."]
        pub async fn list_connection_profiles(
            &mut self,
            request: impl tonic::IntoRequest<super::ListConnectionProfilesRequest>,
        ) -> Result<tonic::Response<super::ListConnectionProfilesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.clouddms.v1.DataMigrationService/ListConnectionProfiles",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets details of a single connection profile."]
        pub async fn get_connection_profile(
            &mut self,
            request: impl tonic::IntoRequest<super::GetConnectionProfileRequest>,
        ) -> Result<tonic::Response<super::ConnectionProfile>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.clouddms.v1.DataMigrationService/GetConnectionProfile",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a new connection profile in a given project and location."]
        pub async fn create_connection_profile(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateConnectionProfileRequest>,
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
                "/google.cloud.clouddms.v1.DataMigrationService/CreateConnectionProfile",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Update the configuration of a single connection profile."]
        pub async fn update_connection_profile(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateConnectionProfileRequest>,
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
                "/google.cloud.clouddms.v1.DataMigrationService/UpdateConnectionProfile",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes a single Database Migration Service connection profile."]
        #[doc = " A connection profile can only be deleted if it is not in use by any"]
        #[doc = " active migration jobs."]
        pub async fn delete_connection_profile(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteConnectionProfileRequest>,
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
                "/google.cloud.clouddms.v1.DataMigrationService/DeleteConnectionProfile",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
