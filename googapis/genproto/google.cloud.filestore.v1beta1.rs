/// Network configuration for the instance.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NetworkConfig {
    /// The name of the Google Compute Engine
    /// [VPC network](<https://cloud.google.com/vpc/docs/vpc>) to which the
    /// instance is connected.
    #[prost(string, tag = "1")]
    pub network: ::prost::alloc::string::String,
    /// Internet protocol versions for which the instance has IP addresses
    /// assigned. For this version, only MODE_IPV4 is supported.
    #[prost(enumeration = "network_config::AddressMode", repeated, tag = "3")]
    pub modes: ::prost::alloc::vec::Vec<i32>,
    /// A /29 CIDR block for Basic or a /23 CIDR block for High Scale in one of the
    /// [internal IP address
    /// ranges](<https://www.arin.net/reference/research/statistics/address_filters/>)
    /// that identifies the range of IP addresses reserved for this instance. For
    /// example, 10.0.0.0/29 or 192.168.0.0/23. The range you specify can't overlap
    /// with either existing subnets or assigned IP address ranges for other Cloud
    /// Filestore instances in the selected VPC network.
    #[prost(string, tag = "4")]
    pub reserved_ip_range: ::prost::alloc::string::String,
    /// Output only. IPv4 addresses in the format
    /// `{octet1}.{octet2}.{octet3}.{octet4}` or IPv6 addresses in the format
    /// `{block1}:{block2}:{block3}:{block4}:{block5}:{block6}:{block7}:{block8}`.
    #[prost(string, repeated, tag = "5")]
    pub ip_addresses: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Nested message and enum types in `NetworkConfig`.
pub mod network_config {
    /// Internet protocol versions supported by Cloud Filestore.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum AddressMode {
        /// Internet protocol not set.
        Unspecified = 0,
        /// Use the IPv4 internet protocol.
        ModeIpv4 = 1,
    }
}
/// File share configuration for the instance.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FileShareConfig {
    /// The name of the file share (must be 16 characters or less).
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// File share capacity in gigabytes (GB).
    /// Cloud Filestore defines 1 GB as 1024^3 bytes.
    #[prost(int64, tag = "2")]
    pub capacity_gb: i64,
    /// Nfs Export Options.
    /// There is a limit of 10 export options per file share.
    #[prost(message, repeated, tag = "8")]
    pub nfs_export_options: ::prost::alloc::vec::Vec<NfsExportOptions>,
    /// The source that this file share has been restored from. Empty if the file
    /// share is created from scratch.
    #[prost(oneof = "file_share_config::Source", tags = "9")]
    pub source: ::core::option::Option<file_share_config::Source>,
}
/// Nested message and enum types in `FileShareConfig`.
pub mod file_share_config {
    /// The source that this file share has been restored from. Empty if the file
    /// share is created from scratch.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Source {
        /// The resource name of the backup, in the format
        /// `projects/{project_id}/locations/{location_id}/backups/{backup_id}`, that
        /// this file share has been restored from.
        #[prost(string, tag = "9")]
        SourceBackup(::prost::alloc::string::String),
    }
}
/// NFS export options specifications.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NfsExportOptions {
    /// List of either an IPv4 addresses in the format
    /// `{octet1}.{octet2}.{octet3}.{octet4}` or CIDR ranges in the format
    /// `{octet1}.{octet2}.{octet3}.{octet4}/{mask size}` which may mount the
    /// file share.
    /// Overlapping IP ranges are not allowed, both within and across
    /// NfsExportOptions. An error will be returned.
    /// The limit is 64 IP ranges/addresses for each FileShareConfig among all
    /// NfsExportOptions.
    #[prost(string, repeated, tag = "1")]
    pub ip_ranges: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Either READ_ONLY, for allowing only read requests on the exported
    /// directory, or READ_WRITE, for allowing both read and write requests.
    /// The default is READ_WRITE.
    #[prost(enumeration = "nfs_export_options::AccessMode", tag = "2")]
    pub access_mode: i32,
    /// Either NO_ROOT_SQUASH, for allowing root access on the exported directory,
    /// or ROOT_SQUASH, for not allowing root access. The default is
    /// NO_ROOT_SQUASH.
    #[prost(enumeration = "nfs_export_options::SquashMode", tag = "3")]
    pub squash_mode: i32,
    /// An integer representing the anonymous user id with a default value of
    /// 65534.
    /// Anon_uid may only be set with squash_mode of ROOT_SQUASH.  An error will be
    /// returned if this field is specified for other squash_mode settings.
    #[prost(int64, tag = "4")]
    pub anon_uid: i64,
    /// An integer representing the anonymous group id with a default value of
    /// 65534.
    /// Anon_gid may only be set with squash_mode of ROOT_SQUASH.  An error will be
    /// returned if this field is specified for other squash_mode settings.
    #[prost(int64, tag = "5")]
    pub anon_gid: i64,
}
/// Nested message and enum types in `NfsExportOptions`.
pub mod nfs_export_options {
    /// The access mode.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum AccessMode {
        /// AccessMode not set.
        Unspecified = 0,
        /// The client can only read the file share.
        ReadOnly = 1,
        /// The client can read and write the file share (default).
        ReadWrite = 2,
    }
    /// The squash mode.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum SquashMode {
        /// SquashMode not set.
        Unspecified = 0,
        /// The Root user has root access to the file share (default).
        NoRootSquash = 1,
        /// The Root user has squashed access to the anonymous uid/gid.
        RootSquash = 2,
    }
}
/// A Cloud Filestore instance.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Instance {
    /// Output only. The resource name of the instance, in the format
    /// `projects/{project_id}/locations/{location_id}/instances/{instance_id}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The description of the instance (2048 characters or less).
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// Output only. The instance state.
    #[prost(enumeration = "instance::State", tag = "5")]
    pub state: i32,
    /// Output only. Additional information about the instance state, if available.
    #[prost(string, tag = "6")]
    pub status_message: ::prost::alloc::string::String,
    /// Output only. The time when the instance was created.
    #[prost(message, optional, tag = "7")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The service tier of the instance.
    #[prost(enumeration = "instance::Tier", tag = "8")]
    pub tier: i32,
    /// Resource labels to represent user provided metadata.
    #[prost(map = "string, string", tag = "9")]
    pub labels:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// File system shares on the instance.
    /// For this version, only a single file share is supported.
    #[prost(message, repeated, tag = "10")]
    pub file_shares: ::prost::alloc::vec::Vec<FileShareConfig>,
    /// VPC networks to which the instance is connected.
    /// For this version, only a single network is supported.
    #[prost(message, repeated, tag = "11")]
    pub networks: ::prost::alloc::vec::Vec<NetworkConfig>,
    /// Server-specified ETag for the instance resource to prevent simultaneous
    /// updates from overwriting each other.
    #[prost(string, tag = "12")]
    pub etag: ::prost::alloc::string::String,
    /// Output only. Reserved for future use.
    #[prost(message, optional, tag = "13")]
    pub satisfies_pzs: ::core::option::Option<bool>,
}
/// Nested message and enum types in `Instance`.
pub mod instance {
    /// The instance state.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// State not set.
        Unspecified = 0,
        /// The instance is being created.
        Creating = 1,
        /// The instance is available for use.
        Ready = 2,
        /// Work is being done on the instance. You can get further details from the
        /// `statusMessage` field of the `Instance` resource.
        Repairing = 3,
        /// The instance is shutting down.
        Deleting = 4,
        /// The instance is experiencing an issue and might be unusable. You can get
        /// further details from the `statusMessage` field of the `Instance`
        /// resource.
        Error = 6,
        /// The instance is restoring a snapshot or backup to an existing file share
        /// and may be unusable during this time.
        Restoring = 7,
    }
    /// Available service tiers.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Tier {
        /// Not set.
        Unspecified = 0,
        /// STANDARD tier. BASIC_HDD is the preferred term for this tier.
        Standard = 1,
        /// PREMIUM tier. BASIC_SSD is the preferred term for this tier.
        Premium = 2,
        /// BASIC instances offer a maximum capacity of 63.9 TB.
        /// BASIC_HDD is an alias for STANDARD Tier, offering economical
        /// performance backed by HDD.
        BasicHdd = 3,
        /// BASIC instances offer a maximum capacity of 63.9 TB.
        /// BASIC_SSD is an alias for PREMIUM Tier, and offers improved
        /// performance backed by SSD.
        BasicSsd = 4,
        /// HIGH_SCALE instances offer expanded capacity and performance scaling
        /// capabilities.
        HighScaleSsd = 6,
    }
}
/// CreateInstanceRequest creates an instance.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateInstanceRequest {
    /// Required. The instance's project and location, in the format
    /// `projects/{project_id}/locations/{location}`. In Cloud Filestore,
    /// locations map to GCP zones, for example **us-west1-b**.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The ID of the instance to create.
    /// The ID must be unique within the specified project and location.
    ///
    /// This value must start with a lowercase letter followed by up to 62
    /// lowercase letters, numbers, or hyphens, and cannot end with a hyphen.
    #[prost(string, tag = "2")]
    pub instance_id: ::prost::alloc::string::String,
    /// Required. An [instance resource]\[google.cloud.filestore.v1beta1.Instance\]
    #[prost(message, optional, tag = "3")]
    pub instance: ::core::option::Option<Instance>,
}
/// GetInstanceRequest gets the state of an instance.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetInstanceRequest {
    /// Required. The instance resource name, in the format
    /// `projects/{project_id}/locations/{location}/instances/{instance_id}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// UpdateInstanceRequest updates the settings of an instance.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateInstanceRequest {
    /// Required. Mask of fields to update.  At least one path must be supplied in this
    /// field.  The elements of the repeated paths field may only include these
    /// fields:
    ///
    /// * "description"
    /// * "file_shares"
    /// * "labels"
    #[prost(message, optional, tag = "1")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Required. Only fields specified in update_mask are updated.
    #[prost(message, optional, tag = "2")]
    pub instance: ::core::option::Option<Instance>,
}
/// RestoreInstanceRequest restores an existing instances's file share from a
/// snapshot or backup.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RestoreInstanceRequest {
    /// Required. The resource name of the instance, in the format
    /// `projects/{project_id}/locations/{location_id}/instances/{instance_id}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. Name of the file share in the Cloud Filestore instance that the snapshot
    /// is being restored to.
    #[prost(string, tag = "2")]
    pub file_share: ::prost::alloc::string::String,
    #[prost(oneof = "restore_instance_request::Source", tags = "3, 4")]
    pub source: ::core::option::Option<restore_instance_request::Source>,
}
/// Nested message and enum types in `RestoreInstanceRequest`.
pub mod restore_instance_request {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Source {
        /// The resource name of the snapshot, in the format
        /// `projects/{project_id}/locations/{location_id}/snapshots/{snapshot_id}`.
        #[prost(string, tag = "3")]
        SourceSnapshot(::prost::alloc::string::String),
        /// The resource name of the backup, in the format
        /// `projects/{project_id}/locations/{location_id}/backups/{backup_id}`.
        #[prost(string, tag = "4")]
        SourceBackup(::prost::alloc::string::String),
    }
}
/// DeleteInstanceRequest deletes an instance.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteInstanceRequest {
    /// Required. The instance resource name, in the format
    /// `projects/{project_id}/locations/{location}/instances/{instance_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// ListInstancesRequest lists instances.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListInstancesRequest {
    /// Required. The project and location for which to retrieve instance information,
    /// in the format `projects/{project_id}/locations/{location}`. In Cloud
    /// Filestore, locations map to GCP zones, for example **us-west1-b**. To
    /// retrieve instance information for all locations, use "-" for the
    /// `{location}` value.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of items to return.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The next_page_token value to use if there are additional
    /// results to retrieve for this list request.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Sort results. Supported values are "name", "name desc" or "" (unsorted).
    #[prost(string, tag = "4")]
    pub order_by: ::prost::alloc::string::String,
    /// List filter.
    #[prost(string, tag = "5")]
    pub filter: ::prost::alloc::string::String,
}
/// ListInstancesResponse is the result of ListInstancesRequest.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListInstancesResponse {
    /// A list of instances in the project for the specified location.
    ///
    /// If the `{location}` value in the request is "-", the response contains a
    /// list of instances from all locations. If any location is unreachable, the
    /// response will only return instances in reachable locations and the
    /// "unreachable" field will be populated with a list of unreachable locations.
    #[prost(message, repeated, tag = "1")]
    pub instances: ::prost::alloc::vec::Vec<Instance>,
    /// The token you can use to retrieve the next page of results. Not returned
    /// if there are no more results in the list.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// A Cloud Filestore backup.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Backup {
    /// Output only. The resource name of the backup, in the format
    /// `projects/{project_id}/locations/{location_id}/backups/{backup_id}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// A description of the backup with 2048 characters or less.
    /// Requests with longer descriptions will be rejected.
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// Output only. The backup state.
    #[prost(enumeration = "backup::State", tag = "3")]
    pub state: i32,
    /// Output only. The time when the backup was created.
    #[prost(message, optional, tag = "4")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Resource labels to represent user provided metadata.
    #[prost(map = "string, string", tag = "5")]
    pub labels:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Output only. Capacity of the source file share when the backup was created.
    #[prost(int64, tag = "6")]
    pub capacity_gb: i64,
    /// Output only. The size of the storage used by the backup. As backups share storage,
    /// this number is expected to change with backup creation/deletion.
    #[prost(int64, tag = "7")]
    pub storage_bytes: i64,
    /// The resource name of the source Cloud Filestore instance, in the format
    /// `projects/{project_id}/locations/{location_id}/instances/{instance_id}`,
    /// used to create this backup.
    #[prost(string, tag = "8")]
    pub source_instance: ::prost::alloc::string::String,
    /// Name of the file share in the source Cloud Filestore instance that the
    /// backup is created from.
    #[prost(string, tag = "9")]
    pub source_file_share: ::prost::alloc::string::String,
    /// Output only. The service tier of the source Cloud Filestore instance that this backup
    /// is created from.
    #[prost(enumeration = "instance::Tier", tag = "10")]
    pub source_instance_tier: i32,
    /// Output only. Amount of bytes that will be downloaded if the backup is restored
    #[prost(int64, tag = "11")]
    pub download_bytes: i64,
    /// Output only. Reserved for future use.
    #[prost(message, optional, tag = "12")]
    pub satisfies_pzs: ::core::option::Option<bool>,
}
/// Nested message and enum types in `Backup`.
pub mod backup {
    /// The backup state.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// State not set.
        Unspecified = 0,
        /// Backup is being created.
        Creating = 1,
        /// Backup has been taken and the operation is being finalized. At this
        /// point, changes to the file share will not be reflected in the backup.
        Finalizing = 2,
        /// Backup is available for use.
        Ready = 3,
        /// Backup is being deleted.
        Deleting = 4,
    }
}
/// CreateBackupRequest creates a backup.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateBackupRequest {
    /// Required. The backup's project and location, in the format
    /// `projects/{project_id}/locations/{location}`. In Cloud Filestore,
    /// backup locations map to GCP regions, for example **us-west1**.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. A [backup resource]\[google.cloud.filestore.v1beta1.Backup\]
    #[prost(message, optional, tag = "2")]
    pub backup: ::core::option::Option<Backup>,
    /// Required. The ID to use for the backup.
    /// The ID must be unique within the specified project and location.
    ///
    /// This value must start with a lowercase letter followed by up to 62
    /// lowercase letters, numbers, or hyphens, and cannot end with a hyphen.
    #[prost(string, tag = "3")]
    pub backup_id: ::prost::alloc::string::String,
}
/// DeleteBackupRequest deletes a backup.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteBackupRequest {
    /// Required. The backup resource name, in the format
    /// `projects/{project_id}/locations/{location}/backups/{backup_id}`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// UpdateBackupRequest updates description and/or labels for a backup.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateBackupRequest {
    /// Required. A [backup resource]\[google.cloud.filestore.v1beta1.Backup\]
    #[prost(message, optional, tag = "1")]
    pub backup: ::core::option::Option<Backup>,
    /// Required. Mask of fields to update.  At least one path must be supplied in this
    /// field.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// GetBackupRequest gets the state of a backup.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBackupRequest {
    /// Required. The backup resource name, in the format
    /// `projects/{project_id}/locations/{location}/backups/{backup_id}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// ListBackupsRequest lists backups.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListBackupsRequest {
    /// Required. The project and location for which to retrieve backup information,
    /// in the format `projects/{project_id}/locations/{location}`.
    /// In Cloud Filestore, backup locations map to GCP regions,
    /// for example **us-west1**.
    /// To retrieve backup information for all locations, use "-" for the
    /// `{location}` value.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of items to return.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The next_page_token value to use if there are additional
    /// results to retrieve for this list request.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Sort results. Supported values are "name", "name desc" or "" (unsorted).
    #[prost(string, tag = "4")]
    pub order_by: ::prost::alloc::string::String,
    /// List filter.
    #[prost(string, tag = "5")]
    pub filter: ::prost::alloc::string::String,
}
/// ListBackupsResponse is the result of ListBackupsRequest.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListBackupsResponse {
    /// A list of backups in the project for the specified location.
    ///
    /// If the `{location}` value in the request is "-", the response contains a
    /// list of backups from all locations. If any location is unreachable, the
    /// response will only return backups in reachable locations and the
    /// "unreachable" field will be populated with a list of unreachable
    /// locations.
    #[prost(message, repeated, tag = "1")]
    pub backups: ::prost::alloc::vec::Vec<Backup>,
    /// The token you can use to retrieve the next page of results. Not returned
    /// if there are no more results in the list.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[doc = r" Generated client implementations."]
pub mod cloud_filestore_manager_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Configures and manages Cloud Filestore resources."]
    #[doc = ""]
    #[doc = " Cloud Filestore Manager v1beta1."]
    #[doc = ""]
    #[doc = " The `file.googleapis.com` service implements the Cloud Filestore API and"]
    #[doc = " defines the following model for managing resources:"]
    #[doc = " * The service works with a collection of cloud projects, named: `/projects/*`"]
    #[doc = " * Each project has a collection of available locations, named: `/locations/*`"]
    #[doc = " * Each location has a collection of instances and backups, named:"]
    #[doc = " `/instances/*` and `/backups/*` respectively."]
    #[doc = " * As such, Cloud Filestore instances are resources of the form:"]
    #[doc = "   `/projects/{project_id}/locations/{location_id}/instances/{instance_id}`"]
    #[doc = "   backups are resources of the form:"]
    #[doc = "   `/projects/{project_id}/locations/{location_id}/backup/{backup_id}`"]
    #[doc = ""]
    #[doc = " Note that location_id can represent a GCP `zone` or `region` depending on the"]
    #[doc = " resource."]
    #[doc = " for example:"]
    #[doc = " A zonal Filestore instance:"]
    #[doc = " * `projects/my-project/locations/us-central1-c/instances/my-basic-tier-filer`"]
    #[doc = " A regional Filestore instance:"]
    #[doc = " * `projects/my-project/locations/us-central1/instances/my-enterprise-filer`"]
    #[derive(Debug, Clone)]
    pub struct CloudFilestoreManagerClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> CloudFilestoreManagerClient<T>
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
        ) -> CloudFilestoreManagerClient<InterceptedService<T, F>>
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
            CloudFilestoreManagerClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Lists all instances in a project for either a specified location"]
        #[doc = " or for all locations."]
        pub async fn list_instances(
            &mut self,
            request: impl tonic::IntoRequest<super::ListInstancesRequest>,
        ) -> Result<tonic::Response<super::ListInstancesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.filestore.v1beta1.CloudFilestoreManager/ListInstances",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets the details of a specific instance."]
        pub async fn get_instance(
            &mut self,
            request: impl tonic::IntoRequest<super::GetInstanceRequest>,
        ) -> Result<tonic::Response<super::Instance>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.filestore.v1beta1.CloudFilestoreManager/GetInstance",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates an instance."]
        #[doc = " When creating from a backup, the capacity of the new instance needs to be"]
        #[doc = " equal to or larger than the capacity of the backup (and also equal to or"]
        #[doc = " larger than the minimum capacity of the tier)."]
        pub async fn create_instance(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateInstanceRequest>,
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
                "/google.cloud.filestore.v1beta1.CloudFilestoreManager/CreateInstance",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates the settings of a specific instance."]
        pub async fn update_instance(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateInstanceRequest>,
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
                "/google.cloud.filestore.v1beta1.CloudFilestoreManager/UpdateInstance",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Restores an existing instance's file share from a backup."]
        #[doc = ""]
        #[doc = " The capacity of the instance needs to be equal to or larger than the"]
        #[doc = " capacity of the backup (and also equal to or larger than the minimum"]
        #[doc = " capacity of the tier)."]
        pub async fn restore_instance(
            &mut self,
            request: impl tonic::IntoRequest<super::RestoreInstanceRequest>,
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
                "/google.cloud.filestore.v1beta1.CloudFilestoreManager/RestoreInstance",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes an instance."]
        pub async fn delete_instance(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteInstanceRequest>,
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
                "/google.cloud.filestore.v1beta1.CloudFilestoreManager/DeleteInstance",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists all backups in a project for either a specified location or for all"]
        #[doc = " locations."]
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
                "/google.cloud.filestore.v1beta1.CloudFilestoreManager/ListBackups",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets the details of a specific backup."]
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
                "/google.cloud.filestore.v1beta1.CloudFilestoreManager/GetBackup",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a backup."]
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
                "/google.cloud.filestore.v1beta1.CloudFilestoreManager/CreateBackup",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes a backup."]
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
                "/google.cloud.filestore.v1beta1.CloudFilestoreManager/DeleteBackup",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates the settings of a specific backup."]
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
                "/google.cloud.filestore.v1beta1.CloudFilestoreManager/UpdateBackup",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
