/// A Google Cloud Redis instance.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Instance {
    /// Required. Unique name of the resource in this scope including project and
    /// location using the form:
    ///     `projects/{project_id}/locations/{location_id}/instances/{instance_id}`
    ///
    /// Note: Redis instances are managed and addressed at regional level so
    /// location_id here refers to a GCP region; however, users may choose which
    /// specific zone (or collection of zones for cross-zone instances) an instance
    /// should be provisioned in. Refer to [location_id][google.cloud.redis.v1.Instance.location_id] and
    /// [alternative_location_id][google.cloud.redis.v1.Instance.alternative_location_id] fields for more details.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// An arbitrary and optional user-provided name for the instance.
    #[prost(string, tag = "2")]
    pub display_name: std::string::String,
    /// Resource labels to represent user provided metadata
    #[prost(map = "string, string", tag = "3")]
    pub labels: ::std::collections::HashMap<std::string::String, std::string::String>,
    /// Optional. The zone where the instance will be provisioned. If not provided,
    /// the service will choose a zone for the instance. For STANDARD_HA tier,
    /// instances will be created across two zones for protection against zonal
    /// failures. If [alternative_location_id][google.cloud.redis.v1.Instance.alternative_location_id] is also provided, it must be
    /// different from [location_id][google.cloud.redis.v1.Instance.location_id].
    #[prost(string, tag = "4")]
    pub location_id: std::string::String,
    /// Optional. Only applicable to STANDARD_HA tier which protects the instance
    /// against zonal failures by provisioning it across two zones. If provided, it
    /// must be a different zone from the one provided in [location_id][google.cloud.redis.v1.Instance.location_id].
    #[prost(string, tag = "5")]
    pub alternative_location_id: std::string::String,
    /// Optional. The version of Redis software.
    /// If not provided, latest supported version will be used. Currently, the
    /// supported values are:
    ///
    ///  *   `REDIS_3_2` for Redis 3.2 compatibility
    ///  *   `REDIS_4_0` for Redis 4.0 compatibility (default)
    ///  *   `REDIS_5_0` for Redis 5.0 compatibility
    #[prost(string, tag = "7")]
    pub redis_version: std::string::String,
    /// Optional. The CIDR range of internal addresses that are reserved for this
    /// instance. If not provided, the service will choose an unused /29 block,
    /// for example, 10.0.0.0/29 or 192.168.0.0/29. Ranges must be unique
    /// and non-overlapping with existing subnets in an authorized network.
    #[prost(string, tag = "9")]
    pub reserved_ip_range: std::string::String,
    /// Output only. Hostname or IP address of the exposed Redis endpoint used by
    /// clients to connect to the service.
    #[prost(string, tag = "10")]
    pub host: std::string::String,
    /// Output only. The port number of the exposed Redis endpoint.
    #[prost(int32, tag = "11")]
    pub port: i32,
    /// Output only. The current zone where the Redis endpoint is placed. For Basic
    /// Tier instances, this will always be the same as the [location_id][google.cloud.redis.v1.Instance.location_id]
    /// provided by the user at creation time. For Standard Tier instances,
    /// this can be either [location_id][google.cloud.redis.v1.Instance.location_id] or [alternative_location_id][google.cloud.redis.v1.Instance.alternative_location_id] and can
    /// change after a failover event.
    #[prost(string, tag = "12")]
    pub current_location_id: std::string::String,
    /// Output only. The time the instance was created.
    #[prost(message, optional, tag = "13")]
    pub create_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Output only. The current state of this instance.
    #[prost(enumeration = "instance::State", tag = "14")]
    pub state: i32,
    /// Output only. Additional information about the current status of this
    /// instance, if available.
    #[prost(string, tag = "15")]
    pub status_message: std::string::String,
    /// Optional. Redis configuration parameters, according to
    /// http://redis.io/topics/config. Currently, the only supported parameters
    /// are:
    ///
    ///  Redis version 3.2 and newer:
    ///
    ///  *   maxmemory-policy
    ///  *   notify-keyspace-events
    ///
    ///  Redis version 4.0 and newer:
    ///
    ///  *   activedefrag
    ///  *   lfu-decay-time
    ///  *   lfu-log-factor
    ///  *   maxmemory-gb
    ///
    ///  Redis version 5.0 and newer:
    ///
    ///  *   stream-node-max-bytes
    ///  *   stream-node-max-entries
    #[prost(map = "string, string", tag = "16")]
    pub redis_configs: ::std::collections::HashMap<std::string::String, std::string::String>,
    /// Required. The service tier of the instance.
    #[prost(enumeration = "instance::Tier", tag = "17")]
    pub tier: i32,
    /// Required. Redis memory size in GiB.
    #[prost(int32, tag = "18")]
    pub memory_size_gb: i32,
    /// Optional. The full name of the Google Compute Engine
    /// [network](/compute/docs/networks-and-firewalls#networks) to which the
    /// instance is connected. If left unspecified, the `default` network
    /// will be used.
    #[prost(string, tag = "20")]
    pub authorized_network: std::string::String,
    /// Output only. Cloud IAM identity used by import / export operations to
    /// transfer data to/from Cloud Storage. Format is
    /// "serviceAccount:<service_account_email>". The value may change over time
    /// for a given instance so should be checked before each import/export
    /// operation.
    #[prost(string, tag = "21")]
    pub persistence_iam_identity: std::string::String,
    /// Optional. The connect mode of Redis instance.
    /// If not provided, default one will be used.
    /// Current default: DIRECT_PEERING.
    #[prost(enumeration = "instance::ConnectMode", tag = "22")]
    pub connect_mode: i32,
}
pub mod instance {
    /// Represents the different states of a Redis instance.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// Not set.
        Unspecified = 0,
        /// Redis instance is being created.
        Creating = 1,
        /// Redis instance has been created and is fully usable.
        Ready = 2,
        /// Redis instance configuration is being updated. Certain kinds of updates
        /// may cause the instance to become unusable while the update is in
        /// progress.
        Updating = 3,
        /// Redis instance is being deleted.
        Deleting = 4,
        /// Redis instance is being repaired and may be unusable.
        Repairing = 5,
        /// Maintenance is being performed on this Redis instance.
        Maintenance = 6,
        /// Redis instance is importing data (availability may be affected).
        Importing = 8,
        /// Redis instance is failing over (availability may be affected).
        FailingOver = 9,
    }
    /// Available service tiers to choose from
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Tier {
        /// Not set.
        Unspecified = 0,
        /// BASIC tier: standalone instance
        Basic = 1,
        /// STANDARD_HA tier: highly available primary/replica instances
        StandardHa = 3,
    }
    /// Available connection modes.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ConnectMode {
        /// Not set.
        Unspecified = 0,
        /// Connect via directly peering with memorystore redis hosted service.
        DirectPeering = 1,
        /// Connect with google via private service access and share connection
        /// across google managed services.
        PrivateServiceAccess = 2,
    }
}
/// Request for [ListInstances][google.cloud.redis.v1.CloudRedis.ListInstances].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListInstancesRequest {
    /// Required. The resource name of the instance location using the form:
    ///     `projects/{project_id}/locations/{location_id}`
    /// where `location_id` refers to a GCP region.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// The maximum number of items to return.
    ///
    /// If not specified, a default value of 1000 will be used by the service.
    /// Regardless of the page_size value, the response may include a partial list
    /// and a caller should only rely on response's
    /// [`next_page_token`][google.cloud.redis.v1.ListInstancesResponse.next_page_token]
    /// to determine if there are more instances left to be queried.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The `next_page_token` value returned from a previous
    /// [ListInstances][google.cloud.redis.v1.CloudRedis.ListInstances] request, if any.
    #[prost(string, tag = "3")]
    pub page_token: std::string::String,
}
/// Response for [ListInstances][google.cloud.redis.v1.CloudRedis.ListInstances].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListInstancesResponse {
    /// A list of Redis instances in the project in the specified location,
    /// or across all locations.
    ///
    /// If the `location_id` in the parent field of the request is "-", all regions
    /// available to the project are queried, and the results aggregated.
    /// If in such an aggregated query a location is unavailable, a dummy Redis
    /// entry is included in the response with the `name` field set to a value of
    /// the form `projects/{project_id}/locations/{location_id}/instances/`- and
    /// the `status` field set to ERROR and `status_message` field set to "location
    /// not available for ListInstances".
    #[prost(message, repeated, tag = "1")]
    pub instances: ::std::vec::Vec<Instance>,
    /// Token to retrieve the next page of results, or empty if there are no more
    /// results in the list.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::std::vec::Vec<std::string::String>,
}
/// Request for [GetInstance][google.cloud.redis.v1.CloudRedis.GetInstance].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetInstanceRequest {
    /// Required. Redis instance resource name using the form:
    ///     `projects/{project_id}/locations/{location_id}/instances/{instance_id}`
    /// where `location_id` refers to a GCP region.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Request for [CreateInstance][google.cloud.redis.v1.CloudRedis.CreateInstance].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateInstanceRequest {
    /// Required. The resource name of the instance location using the form:
    ///     `projects/{project_id}/locations/{location_id}`
    /// where `location_id` refers to a GCP region.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Required. The logical name of the Redis instance in the customer project
    /// with the following restrictions:
    ///
    /// * Must contain only lowercase letters, numbers, and hyphens.
    /// * Must start with a letter.
    /// * Must be between 1-40 characters.
    /// * Must end with a number or a letter.
    /// * Must be unique within the customer project / location
    #[prost(string, tag = "2")]
    pub instance_id: std::string::String,
    /// Required. A Redis [Instance] resource
    #[prost(message, optional, tag = "3")]
    pub instance: ::std::option::Option<Instance>,
}
/// Request for [UpdateInstance][google.cloud.redis.v1.CloudRedis.UpdateInstance].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateInstanceRequest {
    /// Required. Mask of fields to update. At least one path must be supplied in
    /// this field. The elements of the repeated paths field may only include these
    /// fields from [Instance][google.cloud.redis.v1.Instance]:
    ///
    ///  *   `displayName`
    ///  *   `labels`
    ///  *   `memorySizeGb`
    ///  *   `redisConfig`
    #[prost(message, optional, tag = "1")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
    /// Required. Update description.
    /// Only fields specified in update_mask are updated.
    #[prost(message, optional, tag = "2")]
    pub instance: ::std::option::Option<Instance>,
}
/// Request for [DeleteInstance][google.cloud.redis.v1.CloudRedis.DeleteInstance].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteInstanceRequest {
    /// Required. Redis instance resource name using the form:
    ///     `projects/{project_id}/locations/{location_id}/instances/{instance_id}`
    /// where `location_id` refers to a GCP region.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// The Cloud Storage location for the input content
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GcsSource {
    /// Required. Source data URI. (e.g. 'gs://my_bucket/my_object').
    #[prost(string, tag = "1")]
    pub uri: std::string::String,
}
/// The input content
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InputConfig {
    /// Required. Specify source location of input data
    #[prost(oneof = "input_config::Source", tags = "1")]
    pub source: ::std::option::Option<input_config::Source>,
}
pub mod input_config {
    /// Required. Specify source location of input data
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Source {
        /// Google Cloud Storage location where input content is located.
        #[prost(message, tag = "1")]
        GcsSource(super::GcsSource),
    }
}
/// Request for [Import][google.cloud.redis.v1.CloudRedis.ImportInstance].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportInstanceRequest {
    /// Required. Redis instance resource name using the form:
    ///     `projects/{project_id}/locations/{location_id}/instances/{instance_id}`
    /// where `location_id` refers to a GCP region.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Required. Specify data to be imported.
    #[prost(message, optional, tag = "3")]
    pub input_config: ::std::option::Option<InputConfig>,
}
/// The Cloud Storage location for the output content
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GcsDestination {
    /// Required. Data destination URI (e.g.
    /// 'gs://my_bucket/my_object'). Existing files will be overwritten.
    #[prost(string, tag = "1")]
    pub uri: std::string::String,
}
/// The output content
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OutputConfig {
    /// Required. Specify destination location of output data
    #[prost(oneof = "output_config::Destination", tags = "1")]
    pub destination: ::std::option::Option<output_config::Destination>,
}
pub mod output_config {
    /// Required. Specify destination location of output data
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Destination {
        /// Google Cloud Storage destination for output content.
        #[prost(message, tag = "1")]
        GcsDestination(super::GcsDestination),
    }
}
/// Request for [Export][google.cloud.redis.v1.CloudRedis.ExportInstance].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportInstanceRequest {
    /// Required. Redis instance resource name using the form:
    ///     `projects/{project_id}/locations/{location_id}/instances/{instance_id}`
    /// where `location_id` refers to a GCP region.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Required. Specify data to be exported.
    #[prost(message, optional, tag = "3")]
    pub output_config: ::std::option::Option<OutputConfig>,
}
/// Request for [Failover][google.cloud.redis.v1.CloudRedis.FailoverInstance].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FailoverInstanceRequest {
    /// Required. Redis instance resource name using the form:
    ///     `projects/{project_id}/locations/{location_id}/instances/{instance_id}`
    /// where `location_id` refers to a GCP region.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Optional. Available data protection modes that the user can choose. If it's
    /// unspecified, data protection mode will be LIMITED_DATA_LOSS by default.
    #[prost(
        enumeration = "failover_instance_request::DataProtectionMode",
        tag = "2"
    )]
    pub data_protection_mode: i32,
}
pub mod failover_instance_request {
    /// Specifies different modes of operation in relation to the data retention.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum DataProtectionMode {
        /// Defaults to LIMITED_DATA_LOSS if a data protection mode is not
        /// specified.
        Unspecified = 0,
        /// Instance failover will be protected with data loss control. More
        /// specifically, the failover will only be performed if the current
        /// replication offset diff between master and replica is under a certain
        /// threshold.
        LimitedDataLoss = 1,
        /// Instance failover will be performed without data loss control.
        ForceDataLoss = 2,
    }
}
/// Represents the v1 metadata of the long-running operation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationMetadata {
    /// Creation timestamp.
    #[prost(message, optional, tag = "1")]
    pub create_time: ::std::option::Option<::prost_types::Timestamp>,
    /// End timestamp.
    #[prost(message, optional, tag = "2")]
    pub end_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Operation target.
    #[prost(string, tag = "3")]
    pub target: std::string::String,
    /// Operation verb.
    #[prost(string, tag = "4")]
    pub verb: std::string::String,
    /// Operation status details.
    #[prost(string, tag = "5")]
    pub status_detail: std::string::String,
    /// Specifies if cancellation was requested for the operation.
    #[prost(bool, tag = "6")]
    pub cancel_requested: bool,
    /// API version.
    #[prost(string, tag = "7")]
    pub api_version: std::string::String,
}
/// This location metadata represents additional configuration options for a
/// given location where a Redis instance may be created. All fields are output
/// only. It is returned as content of the
/// `google.cloud.location.Location.metadata` field.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LocationMetadata {
    /// Output only. The set of available zones in the location. The map is keyed
    /// by the lowercase ID of each zone, as defined by GCE. These keys can be
    /// specified in `location_id` or `alternative_location_id` fields when
    /// creating a Redis instance.
    #[prost(map = "string, message", tag = "1")]
    pub available_zones: ::std::collections::HashMap<std::string::String, ZoneMetadata>,
}
/// Defines specific information for a particular zone. Currently empty and
/// reserved for future use only.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ZoneMetadata {}
#[doc = r" Generated client implementations."]
pub mod cloud_redis_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Configures and manages Cloud Memorystore for Redis instances"]
    #[doc = ""]
    #[doc = " Google Cloud Memorystore for Redis v1"]
    #[doc = ""]
    #[doc = " The `redis.googleapis.com` service implements the Google Cloud Memorystore"]
    #[doc = " for Redis API and defines the following resource model for managing Redis"]
    #[doc = " instances:"]
    #[doc = " * The service works with a collection of cloud projects, named: `/projects/*`"]
    #[doc = " * Each project has a collection of available locations, named: `/locations/*`"]
    #[doc = " * Each location has a collection of Redis instances, named: `/instances/*`"]
    #[doc = " * As such, Redis instances are resources of the form:"]
    #[doc = "   `/projects/{project_id}/locations/{location_id}/instances/{instance_id}`"]
    #[doc = ""]
    #[doc = " Note that location_id must be referring to a GCP `region`; for example:"]
    #[doc = " * `projects/redpepper-1290/locations/us-central1/instances/my-redis`"]
    pub struct CloudRedisClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl CloudRedisClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> CloudRedisClient<T>
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
        #[doc = " Lists all Redis instances owned by a project in either the specified"]
        #[doc = " location (region) or all locations."]
        #[doc = ""]
        #[doc = " The location should have the following format:"]
        #[doc = ""]
        #[doc = " * `projects/{project_id}/locations/{location_id}`"]
        #[doc = ""]
        #[doc = " If `location_id` is specified as `-` (wildcard), then all regions"]
        #[doc = " available to the project are queried, and the results are aggregated."]
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
                "/google.cloud.redis.v1.CloudRedis/ListInstances",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets the details of a specific Redis instance."]
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
                "/google.cloud.redis.v1.CloudRedis/GetInstance",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a Redis instance based on the specified tier and memory size."]
        #[doc = ""]
        #[doc = " By default, the instance is accessible from the project's"]
        #[doc = " [default network](/compute/docs/networks-and-firewalls#networks)."]
        #[doc = ""]
        #[doc = " The creation is executed asynchronously and callers may check the returned"]
        #[doc = " operation to track its progress. Once the operation is completed the Redis"]
        #[doc = " instance will be fully functional. Completed longrunning.Operation will"]
        #[doc = " contain the new instance object in the response field."]
        #[doc = ""]
        #[doc = " The returned operation is automatically deleted after a few hours, so there"]
        #[doc = " is no need to call DeleteOperation."]
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
                "/google.cloud.redis.v1.CloudRedis/CreateInstance",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates the metadata and configuration of a specific Redis instance."]
        #[doc = ""]
        #[doc = " Completed longrunning.Operation will contain the new instance object"]
        #[doc = " in the response field. The returned operation is automatically deleted"]
        #[doc = " after a few hours, so there is no need to call DeleteOperation."]
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
                "/google.cloud.redis.v1.CloudRedis/UpdateInstance",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Import a Redis RDB snapshot file from Cloud Storage into a Redis instance."]
        #[doc = ""]
        #[doc = " Redis may stop serving during this operation. Instance state will be"]
        #[doc = " IMPORTING for entire operation. When complete, the instance will contain"]
        #[doc = " only data from the imported file."]
        #[doc = ""]
        #[doc = " The returned operation is automatically deleted after a few hours, so"]
        #[doc = " there is no need to call DeleteOperation."]
        pub async fn import_instance(
            &mut self,
            request: impl tonic::IntoRequest<super::ImportInstanceRequest>,
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
                "/google.cloud.redis.v1.CloudRedis/ImportInstance",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Export Redis instance data into a Redis RDB format file in Cloud Storage."]
        #[doc = ""]
        #[doc = " Redis will continue serving during this operation."]
        #[doc = ""]
        #[doc = " The returned operation is automatically deleted after a few hours, so"]
        #[doc = " there is no need to call DeleteOperation."]
        pub async fn export_instance(
            &mut self,
            request: impl tonic::IntoRequest<super::ExportInstanceRequest>,
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
                "/google.cloud.redis.v1.CloudRedis/ExportInstance",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Initiates a failover of the master node to current replica node for a"]
        #[doc = " specific STANDARD tier Cloud Memorystore for Redis instance."]
        pub async fn failover_instance(
            &mut self,
            request: impl tonic::IntoRequest<super::FailoverInstanceRequest>,
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
                "/google.cloud.redis.v1.CloudRedis/FailoverInstance",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes a specific Redis instance.  Instance stops serving and data is"]
        #[doc = " deleted."]
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
                "/google.cloud.redis.v1.CloudRedis/DeleteInstance",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for CloudRedisClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for CloudRedisClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "CloudRedisClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod cloud_redis_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with CloudRedisServer."]
    #[async_trait]
    pub trait CloudRedis: Send + Sync + 'static {
        #[doc = " Lists all Redis instances owned by a project in either the specified"]
        #[doc = " location (region) or all locations."]
        #[doc = ""]
        #[doc = " The location should have the following format:"]
        #[doc = ""]
        #[doc = " * `projects/{project_id}/locations/{location_id}`"]
        #[doc = ""]
        #[doc = " If `location_id` is specified as `-` (wildcard), then all regions"]
        #[doc = " available to the project are queried, and the results are aggregated."]
        async fn list_instances(
            &self,
            request: tonic::Request<super::ListInstancesRequest>,
        ) -> Result<tonic::Response<super::ListInstancesResponse>, tonic::Status>;
        #[doc = " Gets the details of a specific Redis instance."]
        async fn get_instance(
            &self,
            request: tonic::Request<super::GetInstanceRequest>,
        ) -> Result<tonic::Response<super::Instance>, tonic::Status>;
        #[doc = " Creates a Redis instance based on the specified tier and memory size."]
        #[doc = ""]
        #[doc = " By default, the instance is accessible from the project's"]
        #[doc = " [default network](/compute/docs/networks-and-firewalls#networks)."]
        #[doc = ""]
        #[doc = " The creation is executed asynchronously and callers may check the returned"]
        #[doc = " operation to track its progress. Once the operation is completed the Redis"]
        #[doc = " instance will be fully functional. Completed longrunning.Operation will"]
        #[doc = " contain the new instance object in the response field."]
        #[doc = ""]
        #[doc = " The returned operation is automatically deleted after a few hours, so there"]
        #[doc = " is no need to call DeleteOperation."]
        async fn create_instance(
            &self,
            request: tonic::Request<super::CreateInstanceRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        >;
        #[doc = " Updates the metadata and configuration of a specific Redis instance."]
        #[doc = ""]
        #[doc = " Completed longrunning.Operation will contain the new instance object"]
        #[doc = " in the response field. The returned operation is automatically deleted"]
        #[doc = " after a few hours, so there is no need to call DeleteOperation."]
        async fn update_instance(
            &self,
            request: tonic::Request<super::UpdateInstanceRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        >;
        #[doc = " Import a Redis RDB snapshot file from Cloud Storage into a Redis instance."]
        #[doc = ""]
        #[doc = " Redis may stop serving during this operation. Instance state will be"]
        #[doc = " IMPORTING for entire operation. When complete, the instance will contain"]
        #[doc = " only data from the imported file."]
        #[doc = ""]
        #[doc = " The returned operation is automatically deleted after a few hours, so"]
        #[doc = " there is no need to call DeleteOperation."]
        async fn import_instance(
            &self,
            request: tonic::Request<super::ImportInstanceRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        >;
        #[doc = " Export Redis instance data into a Redis RDB format file in Cloud Storage."]
        #[doc = ""]
        #[doc = " Redis will continue serving during this operation."]
        #[doc = ""]
        #[doc = " The returned operation is automatically deleted after a few hours, so"]
        #[doc = " there is no need to call DeleteOperation."]
        async fn export_instance(
            &self,
            request: tonic::Request<super::ExportInstanceRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        >;
        #[doc = " Initiates a failover of the master node to current replica node for a"]
        #[doc = " specific STANDARD tier Cloud Memorystore for Redis instance."]
        async fn failover_instance(
            &self,
            request: tonic::Request<super::FailoverInstanceRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        >;
        #[doc = " Deletes a specific Redis instance.  Instance stops serving and data is"]
        #[doc = " deleted."]
        async fn delete_instance(
            &self,
            request: tonic::Request<super::DeleteInstanceRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        >;
    }
    #[doc = " Configures and manages Cloud Memorystore for Redis instances"]
    #[doc = ""]
    #[doc = " Google Cloud Memorystore for Redis v1"]
    #[doc = ""]
    #[doc = " The `redis.googleapis.com` service implements the Google Cloud Memorystore"]
    #[doc = " for Redis API and defines the following resource model for managing Redis"]
    #[doc = " instances:"]
    #[doc = " * The service works with a collection of cloud projects, named: `/projects/*`"]
    #[doc = " * Each project has a collection of available locations, named: `/locations/*`"]
    #[doc = " * Each location has a collection of Redis instances, named: `/instances/*`"]
    #[doc = " * As such, Redis instances are resources of the form:"]
    #[doc = "   `/projects/{project_id}/locations/{location_id}/instances/{instance_id}`"]
    #[doc = ""]
    #[doc = " Note that location_id must be referring to a GCP `region`; for example:"]
    #[doc = " * `projects/redpepper-1290/locations/us-central1/instances/my-redis`"]
    #[derive(Debug)]
    #[doc(hidden)]
    pub struct CloudRedisServer<T: CloudRedis> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: CloudRedis> CloudRedisServer<T> {
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
    impl<T, B> Service<http::Request<B>> for CloudRedisServer<T>
    where
        T: CloudRedis,
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
                "/google.cloud.redis.v1.CloudRedis/ListInstances" => {
                    #[allow(non_camel_case_types)]
                    struct ListInstancesSvc<T: CloudRedis>(pub Arc<T>);
                    impl<T: CloudRedis> tonic::server::UnaryService<super::ListInstancesRequest>
                        for ListInstancesSvc<T>
                    {
                        type Response = super::ListInstancesResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListInstancesRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.list_instances(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ListInstancesSvc(inner);
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
                "/google.cloud.redis.v1.CloudRedis/GetInstance" => {
                    #[allow(non_camel_case_types)]
                    struct GetInstanceSvc<T: CloudRedis>(pub Arc<T>);
                    impl<T: CloudRedis> tonic::server::UnaryService<super::GetInstanceRequest> for GetInstanceSvc<T> {
                        type Response = super::Instance;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetInstanceRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.get_instance(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetInstanceSvc(inner);
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
                "/google.cloud.redis.v1.CloudRedis/CreateInstance" => {
                    #[allow(non_camel_case_types)]
                    struct CreateInstanceSvc<T: CloudRedis>(pub Arc<T>);
                    impl<T: CloudRedis> tonic::server::UnaryService<super::CreateInstanceRequest>
                        for CreateInstanceSvc<T>
                    {
                        type Response = super::super::super::super::longrunning::Operation;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateInstanceRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.create_instance(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = CreateInstanceSvc(inner);
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
                "/google.cloud.redis.v1.CloudRedis/UpdateInstance" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateInstanceSvc<T: CloudRedis>(pub Arc<T>);
                    impl<T: CloudRedis> tonic::server::UnaryService<super::UpdateInstanceRequest>
                        for UpdateInstanceSvc<T>
                    {
                        type Response = super::super::super::super::longrunning::Operation;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateInstanceRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.update_instance(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = UpdateInstanceSvc(inner);
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
                "/google.cloud.redis.v1.CloudRedis/ImportInstance" => {
                    #[allow(non_camel_case_types)]
                    struct ImportInstanceSvc<T: CloudRedis>(pub Arc<T>);
                    impl<T: CloudRedis> tonic::server::UnaryService<super::ImportInstanceRequest>
                        for ImportInstanceSvc<T>
                    {
                        type Response = super::super::super::super::longrunning::Operation;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ImportInstanceRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.import_instance(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ImportInstanceSvc(inner);
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
                "/google.cloud.redis.v1.CloudRedis/ExportInstance" => {
                    #[allow(non_camel_case_types)]
                    struct ExportInstanceSvc<T: CloudRedis>(pub Arc<T>);
                    impl<T: CloudRedis> tonic::server::UnaryService<super::ExportInstanceRequest>
                        for ExportInstanceSvc<T>
                    {
                        type Response = super::super::super::super::longrunning::Operation;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ExportInstanceRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.export_instance(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ExportInstanceSvc(inner);
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
                "/google.cloud.redis.v1.CloudRedis/FailoverInstance" => {
                    #[allow(non_camel_case_types)]
                    struct FailoverInstanceSvc<T: CloudRedis>(pub Arc<T>);
                    impl<T: CloudRedis> tonic::server::UnaryService<super::FailoverInstanceRequest>
                        for FailoverInstanceSvc<T>
                    {
                        type Response = super::super::super::super::longrunning::Operation;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::FailoverInstanceRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.failover_instance(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = FailoverInstanceSvc(inner);
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
                "/google.cloud.redis.v1.CloudRedis/DeleteInstance" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteInstanceSvc<T: CloudRedis>(pub Arc<T>);
                    impl<T: CloudRedis> tonic::server::UnaryService<super::DeleteInstanceRequest>
                        for DeleteInstanceSvc<T>
                    {
                        type Response = super::super::super::super::longrunning::Operation;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteInstanceRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.delete_instance(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = DeleteInstanceSvc(inner);
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
    impl<T: CloudRedis> Clone for CloudRedisServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: CloudRedis> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: CloudRedis> tonic::transport::NamedService for CloudRedisServer<T> {
        const NAME: &'static str = "google.cloud.redis.v1.CloudRedis";
    }
}
