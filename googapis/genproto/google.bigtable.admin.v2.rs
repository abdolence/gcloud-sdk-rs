/// Encapsulates progress related information for a Cloud Bigtable long
/// running operation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationProgress {
    /// Percent completion of the operation.
    /// Values are between 0 and 100 inclusive.
    #[prost(int32, tag = "1")]
    pub progress_percent: i32,
    /// Time the request was received.
    #[prost(message, optional, tag = "2")]
    pub start_time: ::std::option::Option<::prost_types::Timestamp>,
    /// If set, the time at which this operation failed or was completed
    /// successfully.
    #[prost(message, optional, tag = "3")]
    pub end_time: ::std::option::Option<::prost_types::Timestamp>,
}
/// Storage media types for persisting Bigtable data.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum StorageType {
    /// The user did not specify a storage type.
    Unspecified = 0,
    /// Flash (SSD) storage should be used.
    Ssd = 1,
    /// Magnetic drive (HDD) storage should be used.
    Hdd = 2,
}
/// A collection of Bigtable [Tables][google.bigtable.admin.v2.Table] and
/// the resources that serve them.
/// All tables in an instance are served from all
/// [Clusters][google.bigtable.admin.v2.Cluster] in the instance.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Instance {
    /// The unique name of the instance. Values are of the form
    /// `projects/{project}/instances/[a-z][a-z0-9\\-]+[a-z0-9]`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Required. The descriptive name for this instance as it appears in UIs.
    /// Can be changed at any time, but should be kept globally unique
    /// to avoid confusion.
    #[prost(string, tag = "2")]
    pub display_name: std::string::String,
    /// (`OutputOnly`)
    /// The current state of the instance.
    #[prost(enumeration = "instance::State", tag = "3")]
    pub state: i32,
    /// The type of the instance. Defaults to `PRODUCTION`.
    #[prost(enumeration = "instance::Type", tag = "4")]
    pub r#type: i32,
    /// Labels are a flexible and lightweight mechanism for organizing cloud
    /// resources into groups that reflect a customer's organizational needs and
    /// deployment strategies. They can be used to filter resources and aggregate
    /// metrics.
    ///
    /// * Label keys must be between 1 and 63 characters long and must conform to
    ///   the regular expression: `[\p{Ll}\p{Lo}][\p{Ll}\p{Lo}\p{N}_-]{0,62}`.
    /// * Label values must be between 0 and 63 characters long and must conform to
    ///   the regular expression: `[\p{Ll}\p{Lo}\p{N}_-]{0,63}`.
    /// * No more than 64 labels can be associated with a given resource.
    /// * Keys and values must both be under 128 bytes.
    #[prost(map = "string, string", tag = "5")]
    pub labels: ::std::collections::HashMap<std::string::String, std::string::String>,
}
pub mod instance {
    /// Possible states of an instance.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// The state of the instance could not be determined.
        NotKnown = 0,
        /// The instance has been successfully created and can serve requests
        /// to its tables.
        Ready = 1,
        /// The instance is currently being created, and may be destroyed
        /// if the creation process encounters an error.
        Creating = 2,
    }
    /// The type of the instance.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Type {
        /// The type of the instance is unspecified. If set when creating an
        /// instance, a `PRODUCTION` instance will be created. If set when updating
        /// an instance, the type will be left unchanged.
        Unspecified = 0,
        /// An instance meant for production use. `serve_nodes` must be set
        /// on the cluster.
        Production = 1,
        /// The instance is meant for development and testing purposes only; it has
        /// no performance or uptime guarantees and is not covered by SLA.
        /// After a development instance is created, it can be upgraded by
        /// updating the instance to type `PRODUCTION`. An instance created
        /// as a production instance cannot be changed to a development instance.
        /// When creating a development instance, `serve_nodes` on the cluster must
        /// not be set.
        Development = 2,
    }
}
/// A resizable group of nodes in a particular cloud location, capable
/// of serving all [Tables][google.bigtable.admin.v2.Table] in the parent
/// [Instance][google.bigtable.admin.v2.Instance].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Cluster {
    /// The unique name of the cluster. Values are of the form
    /// `projects/{project}/instances/{instance}/clusters/[a-z][-a-z0-9]*`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// (`CreationOnly`)
    /// The location where this cluster's nodes and storage reside. For best
    /// performance, clients should be located as close as possible to this
    /// cluster. Currently only zones are supported, so values should be of the
    /// form `projects/{project}/locations/{zone}`.
    #[prost(string, tag = "2")]
    pub location: std::string::String,
    /// The current state of the cluster.
    #[prost(enumeration = "cluster::State", tag = "3")]
    pub state: i32,
    /// Required. The number of nodes allocated to this cluster. More nodes enable
    /// higher throughput and more consistent performance.
    #[prost(int32, tag = "4")]
    pub serve_nodes: i32,
    /// (`CreationOnly`)
    /// The type of storage used by this cluster to serve its
    /// parent instance's tables, unless explicitly overridden.
    #[prost(enumeration = "StorageType", tag = "5")]
    pub default_storage_type: i32,
}
pub mod cluster {
    /// Possible states of a cluster.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// The state of the cluster could not be determined.
        NotKnown = 0,
        /// The cluster has been successfully created and is ready to serve requests.
        Ready = 1,
        /// The cluster is currently being created, and may be destroyed
        /// if the creation process encounters an error.
        /// A cluster may not be able to serve requests while being created.
        Creating = 2,
        /// The cluster is currently being resized, and may revert to its previous
        /// node count if the process encounters an error.
        /// A cluster is still capable of serving requests while being resized,
        /// but may exhibit performance as if its number of allocated nodes is
        /// between the starting and requested states.
        Resizing = 3,
        /// The cluster has no backing nodes. The data (tables) still
        /// exist, but no operations can be performed on the cluster.
        Disabled = 4,
    }
}
/// A configuration object describing how Cloud Bigtable should treat traffic
/// from a particular end user application.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AppProfile {
    /// (`OutputOnly`)
    /// The unique name of the app profile. Values are of the form
    /// `projects/<project>/instances/<instance>/appProfiles/[_a-zA-Z0-9][-_.a-zA-Z0-9]*`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Strongly validated etag for optimistic concurrency control. Preserve the
    /// value returned from `GetAppProfile` when calling `UpdateAppProfile` to
    /// fail the request if there has been a modification in the mean time. The
    /// `update_mask` of the request need not include `etag` for this protection
    /// to apply.
    /// See [Wikipedia](https://en.wikipedia.org/wiki/HTTP_ETag) and
    /// [RFC 7232](https://tools.ietf.org/html/rfc7232#section-2.3) for more
    /// details.
    #[prost(string, tag = "2")]
    pub etag: std::string::String,
    /// Optional long form description of the use case for this AppProfile.
    #[prost(string, tag = "3")]
    pub description: std::string::String,
    /// The routing policy for all read/write requests that use this app profile.
    /// A value must be explicitly set.
    #[prost(oneof = "app_profile::RoutingPolicy", tags = "5, 6")]
    pub routing_policy: ::std::option::Option<app_profile::RoutingPolicy>,
}
pub mod app_profile {
    /// Read/write requests are routed to the nearest cluster in the instance, and
    /// will fail over to the nearest cluster that is available in the event of
    /// transient errors or delays. Clusters in a region are considered
    /// equidistant. Choosing this option sacrifices read-your-writes consistency
    /// to improve availability.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct MultiClusterRoutingUseAny {}
    /// Unconditionally routes all read/write requests to a specific cluster.
    /// This option preserves read-your-writes consistency but does not improve
    /// availability.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SingleClusterRouting {
        /// The cluster to which read/write requests should be routed.
        #[prost(string, tag = "1")]
        pub cluster_id: std::string::String,
        /// Whether or not `CheckAndMutateRow` and `ReadModifyWriteRow` requests are
        /// allowed by this app profile. It is unsafe to send these requests to
        /// the same table/row/column in multiple clusters.
        #[prost(bool, tag = "2")]
        pub allow_transactional_writes: bool,
    }
    /// The routing policy for all read/write requests that use this app profile.
    /// A value must be explicitly set.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum RoutingPolicy {
        /// Use a multi-cluster routing policy.
        #[prost(message, tag = "5")]
        MultiClusterRoutingUseAny(MultiClusterRoutingUseAny),
        /// Use a single-cluster routing policy.
        #[prost(message, tag = "6")]
        SingleClusterRouting(SingleClusterRouting),
    }
}
/// Request message for BigtableInstanceAdmin.CreateInstance.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateInstanceRequest {
    /// Required. The unique name of the project in which to create the new instance.
    /// Values are of the form `projects/{project}`.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Required. The ID to be used when referring to the new instance within its project,
    /// e.g., just `myinstance` rather than
    /// `projects/myproject/instances/myinstance`.
    #[prost(string, tag = "2")]
    pub instance_id: std::string::String,
    /// Required. The instance to create.
    /// Fields marked `OutputOnly` must be left blank.
    #[prost(message, optional, tag = "3")]
    pub instance: ::std::option::Option<Instance>,
    /// Required. The clusters to be created within the instance, mapped by desired
    /// cluster ID, e.g., just `mycluster` rather than
    /// `projects/myproject/instances/myinstance/clusters/mycluster`.
    /// Fields marked `OutputOnly` must be left blank.
    /// Currently, at most four clusters can be specified.
    #[prost(map = "string, message", tag = "4")]
    pub clusters: ::std::collections::HashMap<std::string::String, Cluster>,
}
/// Request message for BigtableInstanceAdmin.GetInstance.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetInstanceRequest {
    /// Required. The unique name of the requested instance. Values are of the form
    /// `projects/{project}/instances/{instance}`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Request message for BigtableInstanceAdmin.ListInstances.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListInstancesRequest {
    /// Required. The unique name of the project for which a list of instances is requested.
    /// Values are of the form `projects/{project}`.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// DEPRECATED: This field is unused and ignored.
    #[prost(string, tag = "2")]
    pub page_token: std::string::String,
}
/// Response message for BigtableInstanceAdmin.ListInstances.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListInstancesResponse {
    /// The list of requested instances.
    #[prost(message, repeated, tag = "1")]
    pub instances: ::std::vec::Vec<Instance>,
    /// Locations from which Instance information could not be retrieved,
    /// due to an outage or some other transient condition.
    /// Instances whose Clusters are all in one of the failed locations
    /// may be missing from `instances`, and Instances with at least one
    /// Cluster in a failed location may only have partial information returned.
    /// Values are of the form `projects/<project>/locations/<zone_id>`
    #[prost(string, repeated, tag = "2")]
    pub failed_locations: ::std::vec::Vec<std::string::String>,
    /// DEPRECATED: This field is unused and ignored.
    #[prost(string, tag = "3")]
    pub next_page_token: std::string::String,
}
/// Request message for BigtableInstanceAdmin.PartialUpdateInstance.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PartialUpdateInstanceRequest {
    /// Required. The Instance which will (partially) replace the current value.
    #[prost(message, optional, tag = "1")]
    pub instance: ::std::option::Option<Instance>,
    /// Required. The subset of Instance fields which should be replaced.
    /// Must be explicitly set.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
}
/// Request message for BigtableInstanceAdmin.DeleteInstance.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteInstanceRequest {
    /// Required. The unique name of the instance to be deleted.
    /// Values are of the form `projects/{project}/instances/{instance}`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Request message for BigtableInstanceAdmin.CreateCluster.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateClusterRequest {
    /// Required. The unique name of the instance in which to create the new cluster.
    /// Values are of the form
    /// `projects/{project}/instances/{instance}`.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Required. The ID to be used when referring to the new cluster within its instance,
    /// e.g., just `mycluster` rather than
    /// `projects/myproject/instances/myinstance/clusters/mycluster`.
    #[prost(string, tag = "2")]
    pub cluster_id: std::string::String,
    /// Required. The cluster to be created.
    /// Fields marked `OutputOnly` must be left blank.
    #[prost(message, optional, tag = "3")]
    pub cluster: ::std::option::Option<Cluster>,
}
/// Request message for BigtableInstanceAdmin.GetCluster.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetClusterRequest {
    /// Required. The unique name of the requested cluster. Values are of the form
    /// `projects/{project}/instances/{instance}/clusters/{cluster}`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Request message for BigtableInstanceAdmin.ListClusters.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListClustersRequest {
    /// Required. The unique name of the instance for which a list of clusters is requested.
    /// Values are of the form `projects/{project}/instances/{instance}`.
    /// Use `{instance} = '-'` to list Clusters for all Instances in a project,
    /// e.g., `projects/myproject/instances/-`.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// DEPRECATED: This field is unused and ignored.
    #[prost(string, tag = "2")]
    pub page_token: std::string::String,
}
/// Response message for BigtableInstanceAdmin.ListClusters.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListClustersResponse {
    /// The list of requested clusters.
    #[prost(message, repeated, tag = "1")]
    pub clusters: ::std::vec::Vec<Cluster>,
    /// Locations from which Cluster information could not be retrieved,
    /// due to an outage or some other transient condition.
    /// Clusters from these locations may be missing from `clusters`,
    /// or may only have partial information returned.
    /// Values are of the form `projects/<project>/locations/<zone_id>`
    #[prost(string, repeated, tag = "2")]
    pub failed_locations: ::std::vec::Vec<std::string::String>,
    /// DEPRECATED: This field is unused and ignored.
    #[prost(string, tag = "3")]
    pub next_page_token: std::string::String,
}
/// Request message for BigtableInstanceAdmin.DeleteCluster.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteClusterRequest {
    /// Required. The unique name of the cluster to be deleted. Values are of the form
    /// `projects/{project}/instances/{instance}/clusters/{cluster}`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// The metadata for the Operation returned by CreateInstance.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateInstanceMetadata {
    /// The request that prompted the initiation of this CreateInstance operation.
    #[prost(message, optional, tag = "1")]
    pub original_request: ::std::option::Option<CreateInstanceRequest>,
    /// The time at which the original request was received.
    #[prost(message, optional, tag = "2")]
    pub request_time: ::std::option::Option<::prost_types::Timestamp>,
    /// The time at which the operation failed or was completed successfully.
    #[prost(message, optional, tag = "3")]
    pub finish_time: ::std::option::Option<::prost_types::Timestamp>,
}
/// The metadata for the Operation returned by UpdateInstance.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateInstanceMetadata {
    /// The request that prompted the initiation of this UpdateInstance operation.
    #[prost(message, optional, tag = "1")]
    pub original_request: ::std::option::Option<PartialUpdateInstanceRequest>,
    /// The time at which the original request was received.
    #[prost(message, optional, tag = "2")]
    pub request_time: ::std::option::Option<::prost_types::Timestamp>,
    /// The time at which the operation failed or was completed successfully.
    #[prost(message, optional, tag = "3")]
    pub finish_time: ::std::option::Option<::prost_types::Timestamp>,
}
/// The metadata for the Operation returned by CreateCluster.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateClusterMetadata {
    /// The request that prompted the initiation of this CreateCluster operation.
    #[prost(message, optional, tag = "1")]
    pub original_request: ::std::option::Option<CreateClusterRequest>,
    /// The time at which the original request was received.
    #[prost(message, optional, tag = "2")]
    pub request_time: ::std::option::Option<::prost_types::Timestamp>,
    /// The time at which the operation failed or was completed successfully.
    #[prost(message, optional, tag = "3")]
    pub finish_time: ::std::option::Option<::prost_types::Timestamp>,
}
/// The metadata for the Operation returned by UpdateCluster.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateClusterMetadata {
    /// The request that prompted the initiation of this UpdateCluster operation.
    #[prost(message, optional, tag = "1")]
    pub original_request: ::std::option::Option<Cluster>,
    /// The time at which the original request was received.
    #[prost(message, optional, tag = "2")]
    pub request_time: ::std::option::Option<::prost_types::Timestamp>,
    /// The time at which the operation failed or was completed successfully.
    #[prost(message, optional, tag = "3")]
    pub finish_time: ::std::option::Option<::prost_types::Timestamp>,
}
/// Request message for BigtableInstanceAdmin.CreateAppProfile.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateAppProfileRequest {
    /// Required. The unique name of the instance in which to create the new app profile.
    /// Values are of the form
    /// `projects/{project}/instances/{instance}`.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Required. The ID to be used when referring to the new app profile within its
    /// instance, e.g., just `myprofile` rather than
    /// `projects/myproject/instances/myinstance/appProfiles/myprofile`.
    #[prost(string, tag = "2")]
    pub app_profile_id: std::string::String,
    /// Required. The app profile to be created.
    /// Fields marked `OutputOnly` will be ignored.
    #[prost(message, optional, tag = "3")]
    pub app_profile: ::std::option::Option<AppProfile>,
    /// If true, ignore safety checks when creating the app profile.
    #[prost(bool, tag = "4")]
    pub ignore_warnings: bool,
}
/// Request message for BigtableInstanceAdmin.GetAppProfile.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAppProfileRequest {
    /// Required. The unique name of the requested app profile. Values are of the form
    /// `projects/{project}/instances/{instance}/appProfiles/{app_profile}`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Request message for BigtableInstanceAdmin.ListAppProfiles.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAppProfilesRequest {
    /// Required. The unique name of the instance for which a list of app profiles is
    /// requested. Values are of the form
    /// `projects/{project}/instances/{instance}`.
    /// Use `{instance} = '-'` to list AppProfiles for all Instances in a project,
    /// e.g., `projects/myproject/instances/-`.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Maximum number of results per page.
    ///
    /// A page_size of zero lets the server choose the number of items to return.
    /// A page_size which is strictly positive will return at most that many items.
    /// A negative page_size will cause an error.
    ///
    /// Following the first request, subsequent paginated calls are not required
    /// to pass a page_size. If a page_size is set in subsequent calls, it must
    /// match the page_size given in the first request.
    #[prost(int32, tag = "3")]
    pub page_size: i32,
    /// The value of `next_page_token` returned by a previous call.
    #[prost(string, tag = "2")]
    pub page_token: std::string::String,
}
/// Response message for BigtableInstanceAdmin.ListAppProfiles.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAppProfilesResponse {
    /// The list of requested app profiles.
    #[prost(message, repeated, tag = "1")]
    pub app_profiles: ::std::vec::Vec<AppProfile>,
    /// Set if not all app profiles could be returned in a single response.
    /// Pass this value to `page_token` in another request to get the next
    /// page of results.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
    /// Locations from which AppProfile information could not be retrieved,
    /// due to an outage or some other transient condition.
    /// AppProfiles from these locations may be missing from `app_profiles`.
    /// Values are of the form `projects/<project>/locations/<zone_id>`
    #[prost(string, repeated, tag = "3")]
    pub failed_locations: ::std::vec::Vec<std::string::String>,
}
/// Request message for BigtableInstanceAdmin.UpdateAppProfile.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateAppProfileRequest {
    /// Required. The app profile which will (partially) replace the current value.
    #[prost(message, optional, tag = "1")]
    pub app_profile: ::std::option::Option<AppProfile>,
    /// Required. The subset of app profile fields which should be replaced.
    /// If unset, all fields will be replaced.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
    /// If true, ignore safety checks when updating the app profile.
    #[prost(bool, tag = "3")]
    pub ignore_warnings: bool,
}
/// Request message for BigtableInstanceAdmin.DeleteAppProfile.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteAppProfileRequest {
    /// Required. The unique name of the app profile to be deleted. Values are of the form
    /// `projects/{project}/instances/{instance}/appProfiles/{app_profile}`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// If true, ignore safety checks when deleting the app profile.
    #[prost(bool, tag = "2")]
    pub ignore_warnings: bool,
}
/// The metadata for the Operation returned by UpdateAppProfile.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateAppProfileMetadata {}
#[doc = r" Generated client implementations."]
pub mod bigtable_instance_admin_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Service for creating, configuring, and deleting Cloud Bigtable Instances and"]
    #[doc = " Clusters. Provides access to the Instance and Cluster schemas only, not the"]
    #[doc = " tables' metadata or data stored in those tables."]
    pub struct BigtableInstanceAdminClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> BigtableInstanceAdminClient<T>
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
        #[doc = " Create an instance within a project."]
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
                "/google.bigtable.admin.v2.BigtableInstanceAdmin/CreateInstance",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets information about an instance."]
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
                "/google.bigtable.admin.v2.BigtableInstanceAdmin/GetInstance",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists information about instances in a project."]
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
                "/google.bigtable.admin.v2.BigtableInstanceAdmin/ListInstances",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates an instance within a project. This method updates only the display"]
        #[doc = " name and type for an Instance. To update other Instance properties, such as"]
        #[doc = " labels, use PartialUpdateInstance."]
        pub async fn update_instance(
            &mut self,
            request: impl tonic::IntoRequest<super::Instance>,
        ) -> Result<tonic::Response<super::Instance>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.bigtable.admin.v2.BigtableInstanceAdmin/UpdateInstance",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Partially updates an instance within a project. This method can modify all"]
        #[doc = " fields of an Instance and is the preferred way to update an Instance."]
        pub async fn partial_update_instance(
            &mut self,
            request: impl tonic::IntoRequest<super::PartialUpdateInstanceRequest>,
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
                "/google.bigtable.admin.v2.BigtableInstanceAdmin/PartialUpdateInstance",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Delete an instance from a project."]
        pub async fn delete_instance(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteInstanceRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.bigtable.admin.v2.BigtableInstanceAdmin/DeleteInstance",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a cluster within an instance."]
        pub async fn create_cluster(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateClusterRequest>,
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
                "/google.bigtable.admin.v2.BigtableInstanceAdmin/CreateCluster",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets information about a cluster."]
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
                "/google.bigtable.admin.v2.BigtableInstanceAdmin/GetCluster",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists information about clusters in an instance."]
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
                "/google.bigtable.admin.v2.BigtableInstanceAdmin/ListClusters",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates a cluster within an instance."]
        pub async fn update_cluster(
            &mut self,
            request: impl tonic::IntoRequest<super::Cluster>,
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
                "/google.bigtable.admin.v2.BigtableInstanceAdmin/UpdateCluster",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes a cluster from an instance."]
        pub async fn delete_cluster(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteClusterRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.bigtable.admin.v2.BigtableInstanceAdmin/DeleteCluster",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates an app profile within an instance."]
        pub async fn create_app_profile(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateAppProfileRequest>,
        ) -> Result<tonic::Response<super::AppProfile>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.bigtable.admin.v2.BigtableInstanceAdmin/CreateAppProfile",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets information about an app profile."]
        pub async fn get_app_profile(
            &mut self,
            request: impl tonic::IntoRequest<super::GetAppProfileRequest>,
        ) -> Result<tonic::Response<super::AppProfile>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.bigtable.admin.v2.BigtableInstanceAdmin/GetAppProfile",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists information about app profiles in an instance."]
        pub async fn list_app_profiles(
            &mut self,
            request: impl tonic::IntoRequest<super::ListAppProfilesRequest>,
        ) -> Result<tonic::Response<super::ListAppProfilesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.bigtable.admin.v2.BigtableInstanceAdmin/ListAppProfiles",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates an app profile within an instance."]
        pub async fn update_app_profile(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateAppProfileRequest>,
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
                "/google.bigtable.admin.v2.BigtableInstanceAdmin/UpdateAppProfile",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes an app profile from an instance."]
        pub async fn delete_app_profile(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteAppProfileRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.bigtable.admin.v2.BigtableInstanceAdmin/DeleteAppProfile",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets the access control policy for an instance resource. Returns an empty"]
        #[doc = " policy if an instance exists but does not have a policy set."]
        pub async fn get_iam_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::super::super::super::iam::v1::GetIamPolicyRequest>,
        ) -> Result<tonic::Response<super::super::super::super::iam::v1::Policy>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.bigtable.admin.v2.BigtableInstanceAdmin/GetIamPolicy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Sets the access control policy on an instance resource. Replaces any"]
        #[doc = " existing policy."]
        pub async fn set_iam_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::super::super::super::iam::v1::SetIamPolicyRequest>,
        ) -> Result<tonic::Response<super::super::super::super::iam::v1::Policy>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.bigtable.admin.v2.BigtableInstanceAdmin/SetIamPolicy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns permissions that the caller has on the specified instance resource."]
        pub async fn test_iam_permissions(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::iam::v1::TestIamPermissionsRequest,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::iam::v1::TestIamPermissionsResponse>,
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
                "/google.bigtable.admin.v2.BigtableInstanceAdmin/TestIamPermissions",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for BigtableInstanceAdminClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for BigtableInstanceAdminClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "BigtableInstanceAdminClient {{ ... }}")
        }
    }
}
/// Information about a table restore.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RestoreInfo {
    /// The type of the restore source.
    #[prost(enumeration = "RestoreSourceType", tag = "1")]
    pub source_type: i32,
    /// Information about the source used to restore the table.
    #[prost(oneof = "restore_info::SourceInfo", tags = "2")]
    pub source_info: ::std::option::Option<restore_info::SourceInfo>,
}
pub mod restore_info {
    /// Information about the source used to restore the table.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum SourceInfo {
        /// Information about the backup used to restore the table. The backup
        /// may no longer exist.
        #[prost(message, tag = "2")]
        BackupInfo(super::BackupInfo),
    }
}
/// A collection of user data indexed by row, column, and timestamp.
/// Each table is served using the resources of its parent cluster.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Table {
    /// Output only. The unique name of the table. Values are of the form
    /// `projects/<project>/instances/<instance>/tables/[_a-zA-Z0-9][-_.a-zA-Z0-9]*`.
    /// Views: `NAME_ONLY`, `SCHEMA_VIEW`, `REPLICATION_VIEW`, `FULL`
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Output only. Map from cluster ID to per-cluster table state.
    /// If it could not be determined whether or not the table has data in a
    /// particular cluster (for example, if its zone is unavailable), then
    /// there will be an entry for the cluster with UNKNOWN `replication_status`.
    /// Views: `REPLICATION_VIEW`, `FULL`
    #[prost(map = "string, message", tag = "2")]
    pub cluster_states: ::std::collections::HashMap<std::string::String, table::ClusterState>,
    /// (`CreationOnly`)
    /// The column families configured for this table, mapped by column family ID.
    /// Views: `SCHEMA_VIEW`, `FULL`
    #[prost(map = "string, message", tag = "3")]
    pub column_families: ::std::collections::HashMap<std::string::String, ColumnFamily>,
    /// (`CreationOnly`)
    /// The granularity (i.e. `MILLIS`) at which timestamps are stored in
    /// this table. Timestamps not matching the granularity will be rejected.
    /// If unspecified at creation time, the value will be set to `MILLIS`.
    /// Views: `SCHEMA_VIEW`, `FULL`.
    #[prost(enumeration = "table::TimestampGranularity", tag = "4")]
    pub granularity: i32,
    /// Output only. If this table was restored from another data source (e.g. a
    /// backup), this field will be populated with information about the restore.
    #[prost(message, optional, tag = "6")]
    pub restore_info: ::std::option::Option<RestoreInfo>,
}
pub mod table {
    /// The state of a table's data in a particular cluster.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ClusterState {
        /// Output only. The state of replication for the table in this cluster.
        #[prost(enumeration = "cluster_state::ReplicationState", tag = "1")]
        pub replication_state: i32,
    }
    pub mod cluster_state {
        /// Table replication states.
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
        #[repr(i32)]
        pub enum ReplicationState {
            /// The replication state of the table is unknown in this cluster.
            StateNotKnown = 0,
            /// The cluster was recently created, and the table must finish copying
            /// over pre-existing data from other clusters before it can begin
            /// receiving live replication updates and serving Data API requests.
            Initializing = 1,
            /// The table is temporarily unable to serve Data API requests from this
            /// cluster due to planned internal maintenance.
            PlannedMaintenance = 2,
            /// The table is temporarily unable to serve Data API requests from this
            /// cluster due to unplanned or emergency maintenance.
            UnplannedMaintenance = 3,
            /// The table can serve Data API requests from this cluster. Depending on
            /// replication delay, reads may not immediately reflect the state of the
            /// table in other clusters.
            Ready = 4,
            /// The table is fully created and ready for use after a restore, and is
            /// being optimized for performance. When optimizations are complete, the
            /// table will transition to `READY` state.
            ReadyOptimizing = 5,
        }
    }
    /// Possible timestamp granularities to use when keeping multiple versions
    /// of data in a table.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum TimestampGranularity {
        /// The user did not specify a granularity. Should not be returned.
        /// When specified during table creation, MILLIS will be used.
        Unspecified = 0,
        /// The table keeps data versioned at a granularity of 1ms.
        Millis = 1,
    }
    /// Defines a view over a table's fields.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum View {
        /// Uses the default view for each method as documented in its request.
        Unspecified = 0,
        /// Only populates `name`.
        NameOnly = 1,
        /// Only populates `name` and fields related to the table's schema.
        SchemaView = 2,
        /// Only populates `name` and fields related to the table's replication
        /// state.
        ReplicationView = 3,
        /// Populates all fields.
        Full = 4,
    }
}
/// A set of columns within a table which share a common configuration.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ColumnFamily {
    /// Garbage collection rule specified as a protobuf.
    /// Must serialize to at most 500 bytes.
    ///
    /// NOTE: Garbage collection executes opportunistically in the background, and
    /// so it's possible for reads to return a cell even if it matches the active
    /// GC expression for its family.
    #[prost(message, optional, tag = "1")]
    pub gc_rule: ::std::option::Option<GcRule>,
}
/// Rule for determining which cells to delete during garbage collection.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GcRule {
    /// Garbage collection rules.
    #[prost(oneof = "gc_rule::Rule", tags = "1, 2, 3, 4")]
    pub rule: ::std::option::Option<gc_rule::Rule>,
}
pub mod gc_rule {
    /// A GcRule which deletes cells matching all of the given rules.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Intersection {
        /// Only delete cells which would be deleted by every element of `rules`.
        #[prost(message, repeated, tag = "1")]
        pub rules: ::std::vec::Vec<super::GcRule>,
    }
    /// A GcRule which deletes cells matching any of the given rules.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Union {
        /// Delete cells which would be deleted by any element of `rules`.
        #[prost(message, repeated, tag = "1")]
        pub rules: ::std::vec::Vec<super::GcRule>,
    }
    /// Garbage collection rules.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Rule {
        /// Delete all cells in a column except the most recent N.
        #[prost(int32, tag = "1")]
        MaxNumVersions(i32),
        /// Delete cells in a column older than the given age.
        /// Values must be at least one millisecond, and will be truncated to
        /// microsecond granularity.
        #[prost(message, tag = "2")]
        MaxAge(::prost_types::Duration),
        /// Delete cells that would be deleted by every nested rule.
        #[prost(message, tag = "3")]
        Intersection(Intersection),
        /// Delete cells that would be deleted by any nested rule.
        #[prost(message, tag = "4")]
        Union(Union),
    }
}
/// A snapshot of a table at a particular time. A snapshot can be used as a
/// checkpoint for data restoration or a data source for a new table.
///
/// Note: This is a private alpha release of Cloud Bigtable snapshots. This
/// feature is not currently available to most Cloud Bigtable customers. This
/// feature might be changed in backward-incompatible ways and is not recommended
/// for production use. It is not subject to any SLA or deprecation policy.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Snapshot {
    /// Output only. The unique name of the snapshot.
    /// Values are of the form
    /// `projects/<project>/instances/<instance>/clusters/<cluster>/snapshots/<snapshot>`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Output only. The source table at the time the snapshot was taken.
    #[prost(message, optional, tag = "2")]
    pub source_table: ::std::option::Option<Table>,
    /// Output only. The size of the data in the source table at the time the
    /// snapshot was taken. In some cases, this value may be computed
    /// asynchronously via a background process and a placeholder of 0 will be used
    /// in the meantime.
    #[prost(int64, tag = "3")]
    pub data_size_bytes: i64,
    /// Output only. The time when the snapshot is created.
    #[prost(message, optional, tag = "4")]
    pub create_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Output only. The time when the snapshot will be deleted. The maximum amount
    /// of time a snapshot can stay active is 365 days. If 'ttl' is not specified,
    /// the default maximum of 365 days will be used.
    #[prost(message, optional, tag = "5")]
    pub delete_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Output only. The current state of the snapshot.
    #[prost(enumeration = "snapshot::State", tag = "6")]
    pub state: i32,
    /// Output only. Description of the snapshot.
    #[prost(string, tag = "7")]
    pub description: std::string::String,
}
pub mod snapshot {
    /// Possible states of a snapshot.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// The state of the snapshot could not be determined.
        NotKnown = 0,
        /// The snapshot has been successfully created and can serve all requests.
        Ready = 1,
        /// The snapshot is currently being created, and may be destroyed if the
        /// creation process encounters an error. A snapshot may not be restored to a
        /// table while it is being created.
        Creating = 2,
    }
}
/// A backup of a Cloud Bigtable table.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Backup {
    /// Output only. A globally unique identifier for the backup which cannot be
    /// changed. Values are of the form
    /// `projects/{project}/instances/{instance}/clusters/{cluster}/
    ///    backups/[_a-zA-Z0-9][-_.a-zA-Z0-9]*`
    /// The final segment of the name must be between 1 and 50 characters
    /// in length.
    ///
    /// The backup is stored in the cluster identified by the prefix of the backup
    /// name of the form
    /// `projects/{project}/instances/{instance}/clusters/{cluster}`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Required. Immutable. Name of the table from which this backup was created.
    /// This needs to be in the same instance as the backup. Values are of the form
    /// `projects/{project}/instances/{instance}/tables/{source_table}`.
    #[prost(string, tag = "2")]
    pub source_table: std::string::String,
    /// Required. The expiration time of the backup, with microseconds
    /// granularity that must be at least 6 hours and at most 30 days
    /// from the time the request is received. Once the `expire_time`
    /// has passed, Cloud Bigtable will delete the backup and free the
    /// resources used by the backup.
    #[prost(message, optional, tag = "3")]
    pub expire_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Output only. `start_time` is the time that the backup was started
    /// (i.e. approximately the time the
    /// [CreateBackup][google.bigtable.admin.v2.BigtableTableAdmin.CreateBackup]
    /// request is received).  The row data in this backup will be no older than
    /// this timestamp.
    #[prost(message, optional, tag = "4")]
    pub start_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Output only. `end_time` is the time that the backup was finished. The row
    /// data in the backup will be no newer than this timestamp.
    #[prost(message, optional, tag = "5")]
    pub end_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Output only. Size of the backup in bytes.
    #[prost(int64, tag = "6")]
    pub size_bytes: i64,
    /// Output only. The current state of the backup.
    #[prost(enumeration = "backup::State", tag = "7")]
    pub state: i32,
}
pub mod backup {
    /// Indicates the current state of the backup.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// Not specified.
        Unspecified = 0,
        /// The pending backup is still being created. Operations on the
        /// backup may fail with `FAILED_PRECONDITION` in this state.
        Creating = 1,
        /// The backup is complete and ready for use.
        Ready = 2,
    }
}
/// Information about a backup.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BackupInfo {
    /// Output only. Name of the backup.
    #[prost(string, tag = "1")]
    pub backup: std::string::String,
    /// Output only. The time that the backup was started. Row data in the backup
    /// will be no older than this timestamp.
    #[prost(message, optional, tag = "2")]
    pub start_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Output only. This time that the backup was finished. Row data in the
    /// backup will be no newer than this timestamp.
    #[prost(message, optional, tag = "3")]
    pub end_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Output only. Name of the table the backup was created from.
    #[prost(string, tag = "4")]
    pub source_table: std::string::String,
}
/// Indicates the type of the restore source.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum RestoreSourceType {
    /// No restore associated.
    Unspecified = 0,
    /// A backup was used as the source of the restore.
    Backup = 1,
}
/// Request message for
/// [google.bigtable.admin.v2.BigtableTableAdmin.CreateTable][google.bigtable.admin.v2.BigtableTableAdmin.CreateTable]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateTableRequest {
    /// Required. The unique name of the instance in which to create the table.
    /// Values are of the form `projects/{project}/instances/{instance}`.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Required. The name by which the new table should be referred to within the
    /// parent instance, e.g., `foobar` rather than `{parent}/tables/foobar`.
    /// Maximum 50 characters.
    #[prost(string, tag = "2")]
    pub table_id: std::string::String,
    /// Required. The Table to create.
    #[prost(message, optional, tag = "3")]
    pub table: ::std::option::Option<Table>,
    /// The optional list of row keys that will be used to initially split the
    /// table into several tablets (tablets are similar to HBase regions).
    /// Given two split keys, `s1` and `s2`, three tablets will be created,
    /// spanning the key ranges: `[, s1), [s1, s2), [s2, )`.
    ///
    /// Example:
    ///
    /// * Row keys := `["a", "apple", "custom", "customer_1", "customer_2",`
    ///                `"other", "zz"]`
    /// * initial_split_keys := `["apple", "customer_1", "customer_2", "other"]`
    /// * Key assignment:
    ///     - Tablet 1 `[, apple)                => {"a"}.`
    ///     - Tablet 2 `[apple, customer_1)      => {"apple", "custom"}.`
    ///     - Tablet 3 `[customer_1, customer_2) => {"customer_1"}.`
    ///     - Tablet 4 `[customer_2, other)      => {"customer_2"}.`
    ///     - Tablet 5 `[other, )                => {"other", "zz"}.`
    #[prost(message, repeated, tag = "4")]
    pub initial_splits: ::std::vec::Vec<create_table_request::Split>,
}
pub mod create_table_request {
    /// An initial split point for a newly created table.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Split {
        /// Row key to use as an initial tablet boundary.
        #[prost(bytes, tag = "1")]
        pub key: std::vec::Vec<u8>,
    }
}
/// Request message for
/// [google.bigtable.admin.v2.BigtableTableAdmin.CreateTableFromSnapshot][google.bigtable.admin.v2.BigtableTableAdmin.CreateTableFromSnapshot]
///
/// Note: This is a private alpha release of Cloud Bigtable snapshots. This
/// feature is not currently available to most Cloud Bigtable customers. This
/// feature might be changed in backward-incompatible ways and is not recommended
/// for production use. It is not subject to any SLA or deprecation policy.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateTableFromSnapshotRequest {
    /// Required. The unique name of the instance in which to create the table.
    /// Values are of the form `projects/{project}/instances/{instance}`.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Required. The name by which the new table should be referred to within the
    /// parent instance, e.g., `foobar` rather than `{parent}/tables/foobar`.
    #[prost(string, tag = "2")]
    pub table_id: std::string::String,
    /// Required. The unique name of the snapshot from which to restore the table.
    /// The snapshot and the table must be in the same instance. Values are of the
    /// form
    /// `projects/{project}/instances/{instance}/clusters/{cluster}/snapshots/{snapshot}`.
    #[prost(string, tag = "3")]
    pub source_snapshot: std::string::String,
}
/// Request message for
/// [google.bigtable.admin.v2.BigtableTableAdmin.DropRowRange][google.bigtable.admin.v2.BigtableTableAdmin.DropRowRange]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DropRowRangeRequest {
    /// Required. The unique name of the table on which to drop a range of rows.
    /// Values are of the form
    /// `projects/{project}/instances/{instance}/tables/{table}`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Delete all rows or by prefix.
    #[prost(oneof = "drop_row_range_request::Target", tags = "2, 3")]
    pub target: ::std::option::Option<drop_row_range_request::Target>,
}
pub mod drop_row_range_request {
    /// Delete all rows or by prefix.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Target {
        /// Delete all rows that start with this row key prefix. Prefix cannot be
        /// zero length.
        #[prost(bytes, tag = "2")]
        RowKeyPrefix(std::vec::Vec<u8>),
        /// Delete all rows in the table. Setting this to false is a no-op.
        #[prost(bool, tag = "3")]
        DeleteAllDataFromTable(bool),
    }
}
/// Request message for
/// [google.bigtable.admin.v2.BigtableTableAdmin.ListTables][google.bigtable.admin.v2.BigtableTableAdmin.ListTables]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTablesRequest {
    /// Required. The unique name of the instance for which tables should be
    /// listed. Values are of the form `projects/{project}/instances/{instance}`.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// The view to be applied to the returned tables' fields.
    /// Only NAME_ONLY view (default) and REPLICATION_VIEW are supported.
    #[prost(enumeration = "table::View", tag = "2")]
    pub view: i32,
    /// Maximum number of results per page.
    ///
    /// A page_size of zero lets the server choose the number of items to return.
    /// A page_size which is strictly positive will return at most that many items.
    /// A negative page_size will cause an error.
    ///
    /// Following the first request, subsequent paginated calls are not required
    /// to pass a page_size. If a page_size is set in subsequent calls, it must
    /// match the page_size given in the first request.
    #[prost(int32, tag = "4")]
    pub page_size: i32,
    /// The value of `next_page_token` returned by a previous call.
    #[prost(string, tag = "3")]
    pub page_token: std::string::String,
}
/// Response message for
/// [google.bigtable.admin.v2.BigtableTableAdmin.ListTables][google.bigtable.admin.v2.BigtableTableAdmin.ListTables]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTablesResponse {
    /// The tables present in the requested instance.
    #[prost(message, repeated, tag = "1")]
    pub tables: ::std::vec::Vec<Table>,
    /// Set if not all tables could be returned in a single response.
    /// Pass this value to `page_token` in another request to get the next
    /// page of results.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
}
/// Request message for
/// [google.bigtable.admin.v2.BigtableTableAdmin.GetTable][google.bigtable.admin.v2.BigtableTableAdmin.GetTable]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTableRequest {
    /// Required. The unique name of the requested table.
    /// Values are of the form
    /// `projects/{project}/instances/{instance}/tables/{table}`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// The view to be applied to the returned table's fields.
    /// Defaults to `SCHEMA_VIEW` if unspecified.
    #[prost(enumeration = "table::View", tag = "2")]
    pub view: i32,
}
/// Request message for
/// [google.bigtable.admin.v2.BigtableTableAdmin.DeleteTable][google.bigtable.admin.v2.BigtableTableAdmin.DeleteTable]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteTableRequest {
    /// Required. The unique name of the table to be deleted.
    /// Values are of the form
    /// `projects/{project}/instances/{instance}/tables/{table}`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Request message for
/// [google.bigtable.admin.v2.BigtableTableAdmin.ModifyColumnFamilies][google.bigtable.admin.v2.BigtableTableAdmin.ModifyColumnFamilies]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModifyColumnFamiliesRequest {
    /// Required. The unique name of the table whose families should be modified.
    /// Values are of the form
    /// `projects/{project}/instances/{instance}/tables/{table}`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Required. Modifications to be atomically applied to the specified table's
    /// families. Entries are applied in order, meaning that earlier modifications
    /// can be masked by later ones (in the case of repeated updates to the same
    /// family, for example).
    #[prost(message, repeated, tag = "2")]
    pub modifications: ::std::vec::Vec<modify_column_families_request::Modification>,
}
pub mod modify_column_families_request {
    /// A create, update, or delete of a particular column family.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Modification {
        /// The ID of the column family to be modified.
        #[prost(string, tag = "1")]
        pub id: std::string::String,
        /// Column familiy modifications.
        #[prost(oneof = "modification::Mod", tags = "2, 3, 4")]
        pub r#mod: ::std::option::Option<modification::Mod>,
    }
    pub mod modification {
        /// Column familiy modifications.
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Mod {
            /// Create a new column family with the specified schema, or fail if
            /// one already exists with the given ID.
            #[prost(message, tag = "2")]
            Create(super::super::ColumnFamily),
            /// Update an existing column family to the specified schema, or fail
            /// if no column family exists with the given ID.
            #[prost(message, tag = "3")]
            Update(super::super::ColumnFamily),
            /// Drop (delete) the column family with the given ID, or fail if no such
            /// family exists.
            #[prost(bool, tag = "4")]
            Drop(bool),
        }
    }
}
/// Request message for
/// [google.bigtable.admin.v2.BigtableTableAdmin.GenerateConsistencyToken][google.bigtable.admin.v2.BigtableTableAdmin.GenerateConsistencyToken]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateConsistencyTokenRequest {
    /// Required. The unique name of the Table for which to create a consistency
    /// token. Values are of the form
    /// `projects/{project}/instances/{instance}/tables/{table}`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Response message for
/// [google.bigtable.admin.v2.BigtableTableAdmin.GenerateConsistencyToken][google.bigtable.admin.v2.BigtableTableAdmin.GenerateConsistencyToken]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateConsistencyTokenResponse {
    /// The generated consistency token.
    #[prost(string, tag = "1")]
    pub consistency_token: std::string::String,
}
/// Request message for
/// [google.bigtable.admin.v2.BigtableTableAdmin.CheckConsistency][google.bigtable.admin.v2.BigtableTableAdmin.CheckConsistency]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckConsistencyRequest {
    /// Required. The unique name of the Table for which to check replication
    /// consistency. Values are of the form
    /// `projects/{project}/instances/{instance}/tables/{table}`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Required. The token created using GenerateConsistencyToken for the Table.
    #[prost(string, tag = "2")]
    pub consistency_token: std::string::String,
}
/// Response message for
/// [google.bigtable.admin.v2.BigtableTableAdmin.CheckConsistency][google.bigtable.admin.v2.BigtableTableAdmin.CheckConsistency]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckConsistencyResponse {
    /// True only if the token is consistent. A token is consistent if replication
    /// has caught up with the restrictions specified in the request.
    #[prost(bool, tag = "1")]
    pub consistent: bool,
}
/// Request message for
/// [google.bigtable.admin.v2.BigtableTableAdmin.SnapshotTable][google.bigtable.admin.v2.BigtableTableAdmin.SnapshotTable]
///
/// Note: This is a private alpha release of Cloud Bigtable snapshots. This
/// feature is not currently available to most Cloud Bigtable customers. This
/// feature might be changed in backward-incompatible ways and is not recommended
/// for production use. It is not subject to any SLA or deprecation policy.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SnapshotTableRequest {
    /// Required. The unique name of the table to have the snapshot taken.
    /// Values are of the form
    /// `projects/{project}/instances/{instance}/tables/{table}`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Required. The name of the cluster where the snapshot will be created in.
    /// Values are of the form
    /// `projects/{project}/instances/{instance}/clusters/{cluster}`.
    #[prost(string, tag = "2")]
    pub cluster: std::string::String,
    /// Required. The ID by which the new snapshot should be referred to within the
    /// parent cluster, e.g., `mysnapshot` of the form:
    /// `[_a-zA-Z0-9][-_.a-zA-Z0-9]*` rather than
    /// `projects/{project}/instances/{instance}/clusters/{cluster}/snapshots/mysnapshot`.
    #[prost(string, tag = "3")]
    pub snapshot_id: std::string::String,
    /// The amount of time that the new snapshot can stay active after it is
    /// created. Once 'ttl' expires, the snapshot will get deleted. The maximum
    /// amount of time a snapshot can stay active is 7 days. If 'ttl' is not
    /// specified, the default value of 24 hours will be used.
    #[prost(message, optional, tag = "4")]
    pub ttl: ::std::option::Option<::prost_types::Duration>,
    /// Description of the snapshot.
    #[prost(string, tag = "5")]
    pub description: std::string::String,
}
/// Request message for
/// [google.bigtable.admin.v2.BigtableTableAdmin.GetSnapshot][google.bigtable.admin.v2.BigtableTableAdmin.GetSnapshot]
///
/// Note: This is a private alpha release of Cloud Bigtable snapshots. This
/// feature is not currently available to most Cloud Bigtable customers. This
/// feature might be changed in backward-incompatible ways and is not recommended
/// for production use. It is not subject to any SLA or deprecation policy.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSnapshotRequest {
    /// Required. The unique name of the requested snapshot.
    /// Values are of the form
    /// `projects/{project}/instances/{instance}/clusters/{cluster}/snapshots/{snapshot}`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Request message for
/// [google.bigtable.admin.v2.BigtableTableAdmin.ListSnapshots][google.bigtable.admin.v2.BigtableTableAdmin.ListSnapshots]
///
/// Note: This is a private alpha release of Cloud Bigtable snapshots. This
/// feature is not currently available to most Cloud Bigtable customers. This
/// feature might be changed in backward-incompatible ways and is not recommended
/// for production use. It is not subject to any SLA or deprecation policy.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSnapshotsRequest {
    /// Required. The unique name of the cluster for which snapshots should be
    /// listed. Values are of the form
    /// `projects/{project}/instances/{instance}/clusters/{cluster}`.
    /// Use `{cluster} = '-'` to list snapshots for all clusters in an instance,
    /// e.g., `projects/{project}/instances/{instance}/clusters/-`.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// The maximum number of snapshots to return per page.
    /// CURRENTLY UNIMPLEMENTED AND IGNORED.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The value of `next_page_token` returned by a previous call.
    #[prost(string, tag = "3")]
    pub page_token: std::string::String,
}
/// Response message for
/// [google.bigtable.admin.v2.BigtableTableAdmin.ListSnapshots][google.bigtable.admin.v2.BigtableTableAdmin.ListSnapshots]
///
/// Note: This is a private alpha release of Cloud Bigtable snapshots. This
/// feature is not currently available to most Cloud Bigtable customers. This
/// feature might be changed in backward-incompatible ways and is not recommended
/// for production use. It is not subject to any SLA or deprecation policy.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSnapshotsResponse {
    /// The snapshots present in the requested cluster.
    #[prost(message, repeated, tag = "1")]
    pub snapshots: ::std::vec::Vec<Snapshot>,
    /// Set if not all snapshots could be returned in a single response.
    /// Pass this value to `page_token` in another request to get the next
    /// page of results.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
}
/// Request message for
/// [google.bigtable.admin.v2.BigtableTableAdmin.DeleteSnapshot][google.bigtable.admin.v2.BigtableTableAdmin.DeleteSnapshot]
///
/// Note: This is a private alpha release of Cloud Bigtable snapshots. This
/// feature is not currently available to most Cloud Bigtable customers. This
/// feature might be changed in backward-incompatible ways and is not recommended
/// for production use. It is not subject to any SLA or deprecation policy.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteSnapshotRequest {
    /// Required. The unique name of the snapshot to be deleted.
    /// Values are of the form
    /// `projects/{project}/instances/{instance}/clusters/{cluster}/snapshots/{snapshot}`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// The metadata for the Operation returned by SnapshotTable.
///
/// Note: This is a private alpha release of Cloud Bigtable snapshots. This
/// feature is not currently available to most Cloud Bigtable customers. This
/// feature might be changed in backward-incompatible ways and is not recommended
/// for production use. It is not subject to any SLA or deprecation policy.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SnapshotTableMetadata {
    /// The request that prompted the initiation of this SnapshotTable operation.
    #[prost(message, optional, tag = "1")]
    pub original_request: ::std::option::Option<SnapshotTableRequest>,
    /// The time at which the original request was received.
    #[prost(message, optional, tag = "2")]
    pub request_time: ::std::option::Option<::prost_types::Timestamp>,
    /// The time at which the operation failed or was completed successfully.
    #[prost(message, optional, tag = "3")]
    pub finish_time: ::std::option::Option<::prost_types::Timestamp>,
}
/// The metadata for the Operation returned by CreateTableFromSnapshot.
///
/// Note: This is a private alpha release of Cloud Bigtable snapshots. This
/// feature is not currently available to most Cloud Bigtable customers. This
/// feature might be changed in backward-incompatible ways and is not recommended
/// for production use. It is not subject to any SLA or deprecation policy.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateTableFromSnapshotMetadata {
    /// The request that prompted the initiation of this CreateTableFromSnapshot
    /// operation.
    #[prost(message, optional, tag = "1")]
    pub original_request: ::std::option::Option<CreateTableFromSnapshotRequest>,
    /// The time at which the original request was received.
    #[prost(message, optional, tag = "2")]
    pub request_time: ::std::option::Option<::prost_types::Timestamp>,
    /// The time at which the operation failed or was completed successfully.
    #[prost(message, optional, tag = "3")]
    pub finish_time: ::std::option::Option<::prost_types::Timestamp>,
}
/// The request for
/// [CreateBackup][google.bigtable.admin.v2.BigtableTableAdmin.CreateBackup].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateBackupRequest {
    /// Required. This must be one of the clusters in the instance in which this
    /// table is located. The backup will be stored in this cluster. Values are
    /// of the form `projects/{project}/instances/{instance}/clusters/{cluster}`.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Required. The id of the backup to be created. The `backup_id` along with
    /// the parent `parent` are combined as {parent}/backups/{backup_id} to create
    /// the full backup name, of the form:
    /// `projects/{project}/instances/{instance}/clusters/{cluster}/backups/{backup_id}`.
    /// This string must be between 1 and 50 characters in length and match the
    /// regex [_a-zA-Z0-9][-_.a-zA-Z0-9]*.
    #[prost(string, tag = "2")]
    pub backup_id: std::string::String,
    /// Required. The backup to create.
    #[prost(message, optional, tag = "3")]
    pub backup: ::std::option::Option<Backup>,
}
/// Metadata type for the operation returned by
/// [CreateBackup][google.bigtable.admin.v2.BigtableTableAdmin.CreateBackup].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateBackupMetadata {
    /// The name of the backup being created.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// The name of the table the backup is created from.
    #[prost(string, tag = "2")]
    pub source_table: std::string::String,
    /// The time at which this operation started.
    #[prost(message, optional, tag = "3")]
    pub start_time: ::std::option::Option<::prost_types::Timestamp>,
    /// If set, the time at which this operation finished or was cancelled.
    #[prost(message, optional, tag = "4")]
    pub end_time: ::std::option::Option<::prost_types::Timestamp>,
}
/// The request for
/// [GetBackup][google.bigtable.admin.v2.BigtableTableAdmin.GetBackup].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBackupRequest {
    /// Required. Name of the backup.
    /// Values are of the form
    /// `projects/{project}/instances/{instance}/clusters/{cluster}/backups/{backup}`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// The request for
/// [UpdateBackup][google.bigtable.admin.v2.BigtableTableAdmin.UpdateBackup].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateBackupRequest {
    /// Required. The backup to update. `backup.name`, and the fields to be updated
    /// as specified by `update_mask` are required. Other fields are ignored.
    /// Update is only supported for the following fields:
    ///  * `backup.expire_time`.
    #[prost(message, optional, tag = "1")]
    pub backup: ::std::option::Option<Backup>,
    /// Required. A mask specifying which fields (e.g. `expire_time`) in the
    /// Backup resource should be updated. This mask is relative to the Backup
    /// resource, not to the request message. The field mask must always be
    /// specified; this prevents any future fields from being erased accidentally
    /// by clients that do not know about them.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
}
/// The request for
/// [DeleteBackup][google.bigtable.admin.v2.BigtableTableAdmin.DeleteBackup].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteBackupRequest {
    /// Required. Name of the backup to delete.
    /// Values are of the form
    /// `projects/{project}/instances/{instance}/clusters/{cluster}/backups/{backup}`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// The request for
/// [ListBackups][google.bigtable.admin.v2.BigtableTableAdmin.ListBackups].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListBackupsRequest {
    /// Required. The cluster to list backups from. Values are of the
    /// form `projects/{project}/instances/{instance}/clusters/{cluster}`.
    /// Use `{cluster} = '-'` to list backups for all clusters in an instance,
    /// e.g., `projects/{project}/instances/{instance}/clusters/-`.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// A filter expression that filters backups listed in the response.
    /// The expression must specify the field name, a comparison operator,
    /// and the value that you want to use for filtering. The value must be a
    /// string, a number, or a boolean. The comparison operator must be
    /// <, >, <=, >=, !=, =, or :. Colon ‘:’ represents a HAS operator which is
    /// roughly synonymous with equality. Filter rules are case insensitive.
    ///
    /// The fields eligible for filtering are:
    ///   * `name`
    ///   * `source_table`
    ///   * `state`
    ///   * `start_time` (and values are of the format YYYY-MM-DDTHH:MM:SSZ)
    ///   * `end_time` (and values are of the format YYYY-MM-DDTHH:MM:SSZ)
    ///   * `expire_time` (and values are of the format YYYY-MM-DDTHH:MM:SSZ)
    ///   * `size_bytes`
    ///
    /// To filter on multiple expressions, provide each separate expression within
    /// parentheses. By default, each expression is an AND expression. However,
    /// you can include AND, OR, and NOT expressions explicitly.
    ///
    /// Some examples of using filters are:
    ///
    ///   * `name:"exact"` --> The backup's name is the string "exact".
    ///   * `name:howl` --> The backup's name contains the string "howl".
    ///   * `source_table:prod`
    ///          --> The source_table's name contains the string "prod".
    ///   * `state:CREATING` --> The backup is pending creation.
    ///   * `state:READY` --> The backup is fully created and ready for use.
    ///   * `(name:howl) AND (start_time < \"2018-03-28T14:50:00Z\")`
    ///          --> The backup name contains the string "howl" and start_time
    ///              of the backup is before 2018-03-28T14:50:00Z.
    ///   * `size_bytes > 10000000000` --> The backup's size is greater than 10GB
    #[prost(string, tag = "2")]
    pub filter: std::string::String,
    /// An expression for specifying the sort order of the results of the request.
    /// The string value should specify one or more fields in
    /// [Backup][google.bigtable.admin.v2.Backup]. The full syntax is described at
    /// https://aip.dev/132#ordering.
    ///
    /// Fields supported are:
    ///    * name
    ///    * source_table
    ///    * expire_time
    ///    * start_time
    ///    * end_time
    ///    * size_bytes
    ///    * state
    ///
    /// For example, "start_time". The default sorting order is ascending.
    /// To specify descending order for the field, a suffix " desc" should
    /// be appended to the field name. For example, "start_time desc".
    /// Redundant space characters in the syntax are insigificant.
    ///
    /// If order_by is empty, results will be sorted by `start_time` in descending
    /// order starting from the most recently created backup.
    #[prost(string, tag = "3")]
    pub order_by: std::string::String,
    /// Number of backups to be returned in the response. If 0 or
    /// less, defaults to the server's maximum allowed page size.
    #[prost(int32, tag = "4")]
    pub page_size: i32,
    /// If non-empty, `page_token` should contain a
    /// [next_page_token][google.bigtable.admin.v2.ListBackupsResponse.next_page_token]
    /// from a previous
    /// [ListBackupsResponse][google.bigtable.admin.v2.ListBackupsResponse] to the
    /// same `parent` and with the same `filter`.
    #[prost(string, tag = "5")]
    pub page_token: std::string::String,
}
/// The response for
/// [ListBackups][google.bigtable.admin.v2.BigtableTableAdmin.ListBackups].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListBackupsResponse {
    /// The list of matching backups.
    #[prost(message, repeated, tag = "1")]
    pub backups: ::std::vec::Vec<Backup>,
    /// `next_page_token` can be sent in a subsequent
    /// [ListBackups][google.bigtable.admin.v2.BigtableTableAdmin.ListBackups] call
    /// to fetch more of the matching backups.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
}
/// The request for
/// [RestoreTable][google.bigtable.admin.v2.BigtableTableAdmin.RestoreTable].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RestoreTableRequest {
    /// Required. The name of the instance in which to create the restored
    /// table. This instance must be the parent of the source backup. Values are
    /// of the form `projects/<project>/instances/<instance>`.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Required. The id of the table to create and restore to. This
    /// table must not already exist. The `table_id` appended to
    /// `parent` forms the full table name of the form
    /// `projects/<project>/instances/<instance>/tables/<table_id>`.
    #[prost(string, tag = "2")]
    pub table_id: std::string::String,
    /// Required. The source from which to restore.
    #[prost(oneof = "restore_table_request::Source", tags = "3")]
    pub source: ::std::option::Option<restore_table_request::Source>,
}
pub mod restore_table_request {
    /// Required. The source from which to restore.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Source {
        /// Name of the backup from which to restore. Values are of the form
        /// `projects/<project>/instances/<instance>/clusters/<cluster>/backups/<backup>`.
        #[prost(string, tag = "3")]
        Backup(std::string::String),
    }
}
/// Metadata type for the long-running operation returned by
/// [RestoreTable][google.bigtable.admin.v2.BigtableTableAdmin.RestoreTable].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RestoreTableMetadata {
    /// Name of the table being created and restored to.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// The type of the restore source.
    #[prost(enumeration = "RestoreSourceType", tag = "2")]
    pub source_type: i32,
    /// If exists, the name of the long-running operation that will be used to
    /// track the post-restore optimization process to optimize the performance of
    /// the restored table. The metadata type of the long-running operation is
    /// [OptimizeRestoreTableMetadata][]. The response type is
    /// [Empty][google.protobuf.Empty]. This long-running operation may be
    /// automatically created by the system if applicable after the
    /// RestoreTable long-running operation completes successfully. This operation
    /// may not be created if the table is already optimized or the restore was
    /// not successful.
    #[prost(string, tag = "4")]
    pub optimize_table_operation_name: std::string::String,
    /// The progress of the
    /// [RestoreTable][google.bigtable.admin.v2.BigtableTableAdmin.RestoreTable]
    /// operation.
    #[prost(message, optional, tag = "5")]
    pub progress: ::std::option::Option<OperationProgress>,
    /// Information about the source used to restore the table, as specified by
    /// `source` in
    /// [RestoreTableRequest][google.bigtable.admin.v2.RestoreTableRequest].
    #[prost(oneof = "restore_table_metadata::SourceInfo", tags = "3")]
    pub source_info: ::std::option::Option<restore_table_metadata::SourceInfo>,
}
pub mod restore_table_metadata {
    /// Information about the source used to restore the table, as specified by
    /// `source` in
    /// [RestoreTableRequest][google.bigtable.admin.v2.RestoreTableRequest].
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum SourceInfo {
        #[prost(message, tag = "3")]
        BackupInfo(super::BackupInfo),
    }
}
/// Metadata type for the long-running operation used to track the progress
/// of optimizations performed on a newly restored table. This long-running
/// operation is automatically created by the system after the successful
/// completion of a table restore, and cannot be cancelled.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OptimizeRestoredTableMetadata {
    /// Name of the restored table being optimized.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// The progress of the post-restore optimizations.
    #[prost(message, optional, tag = "2")]
    pub progress: ::std::option::Option<OperationProgress>,
}
#[doc = r" Generated client implementations."]
pub mod bigtable_table_admin_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Service for creating, configuring, and deleting Cloud Bigtable tables."]
    #[doc = ""]
    #[doc = ""]
    #[doc = " Provides access to the table schemas only, not the data stored within"]
    #[doc = " the tables."]
    pub struct BigtableTableAdminClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> BigtableTableAdminClient<T>
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
        #[doc = " Creates a new table in the specified instance."]
        #[doc = " The table can be created with a full set of initial column families,"]
        #[doc = " specified in the request."]
        pub async fn create_table(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateTableRequest>,
        ) -> Result<tonic::Response<super::Table>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.bigtable.admin.v2.BigtableTableAdmin/CreateTable",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a new table from the specified snapshot. The target table must"]
        #[doc = " not exist. The snapshot and the table must be in the same instance."]
        #[doc = ""]
        #[doc = " Note: This is a private alpha release of Cloud Bigtable snapshots. This"]
        #[doc = " feature is not currently available to most Cloud Bigtable customers. This"]
        #[doc = " feature might be changed in backward-incompatible ways and is not"]
        #[doc = " recommended for production use. It is not subject to any SLA or deprecation"]
        #[doc = " policy."]
        pub async fn create_table_from_snapshot(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateTableFromSnapshotRequest>,
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
                "/google.bigtable.admin.v2.BigtableTableAdmin/CreateTableFromSnapshot",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists all tables served from a specified instance."]
        pub async fn list_tables(
            &mut self,
            request: impl tonic::IntoRequest<super::ListTablesRequest>,
        ) -> Result<tonic::Response<super::ListTablesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.bigtable.admin.v2.BigtableTableAdmin/ListTables",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets metadata information about the specified table."]
        pub async fn get_table(
            &mut self,
            request: impl tonic::IntoRequest<super::GetTableRequest>,
        ) -> Result<tonic::Response<super::Table>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.bigtable.admin.v2.BigtableTableAdmin/GetTable",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Permanently deletes a specified table and all of its data."]
        pub async fn delete_table(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteTableRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.bigtable.admin.v2.BigtableTableAdmin/DeleteTable",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Performs a series of column family modifications on the specified table."]
        #[doc = " Either all or none of the modifications will occur before this method"]
        #[doc = " returns, but data requests received prior to that point may see a table"]
        #[doc = " where only some modifications have taken effect."]
        pub async fn modify_column_families(
            &mut self,
            request: impl tonic::IntoRequest<super::ModifyColumnFamiliesRequest>,
        ) -> Result<tonic::Response<super::Table>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.bigtable.admin.v2.BigtableTableAdmin/ModifyColumnFamilies",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Permanently drop/delete a row range from a specified table. The request can"]
        #[doc = " specify whether to delete all rows in a table, or only those that match a"]
        #[doc = " particular prefix."]
        pub async fn drop_row_range(
            &mut self,
            request: impl tonic::IntoRequest<super::DropRowRangeRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.bigtable.admin.v2.BigtableTableAdmin/DropRowRange",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Generates a consistency token for a Table, which can be used in"]
        #[doc = " CheckConsistency to check whether mutations to the table that finished"]
        #[doc = " before this call started have been replicated. The tokens will be available"]
        #[doc = " for 90 days."]
        pub async fn generate_consistency_token(
            &mut self,
            request: impl tonic::IntoRequest<super::GenerateConsistencyTokenRequest>,
        ) -> Result<tonic::Response<super::GenerateConsistencyTokenResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.bigtable.admin.v2.BigtableTableAdmin/GenerateConsistencyToken",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Checks replication consistency based on a consistency token, that is, if"]
        #[doc = " replication has caught up based on the conditions specified in the token"]
        #[doc = " and the check request."]
        pub async fn check_consistency(
            &mut self,
            request: impl tonic::IntoRequest<super::CheckConsistencyRequest>,
        ) -> Result<tonic::Response<super::CheckConsistencyResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.bigtable.admin.v2.BigtableTableAdmin/CheckConsistency",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a new snapshot in the specified cluster from the specified"]
        #[doc = " source table. The cluster and the table must be in the same instance."]
        #[doc = ""]
        #[doc = " Note: This is a private alpha release of Cloud Bigtable snapshots. This"]
        #[doc = " feature is not currently available to most Cloud Bigtable customers. This"]
        #[doc = " feature might be changed in backward-incompatible ways and is not"]
        #[doc = " recommended for production use. It is not subject to any SLA or deprecation"]
        #[doc = " policy."]
        pub async fn snapshot_table(
            &mut self,
            request: impl tonic::IntoRequest<super::SnapshotTableRequest>,
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
                "/google.bigtable.admin.v2.BigtableTableAdmin/SnapshotTable",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets metadata information about the specified snapshot."]
        #[doc = ""]
        #[doc = " Note: This is a private alpha release of Cloud Bigtable snapshots. This"]
        #[doc = " feature is not currently available to most Cloud Bigtable customers. This"]
        #[doc = " feature might be changed in backward-incompatible ways and is not"]
        #[doc = " recommended for production use. It is not subject to any SLA or deprecation"]
        #[doc = " policy."]
        pub async fn get_snapshot(
            &mut self,
            request: impl tonic::IntoRequest<super::GetSnapshotRequest>,
        ) -> Result<tonic::Response<super::Snapshot>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.bigtable.admin.v2.BigtableTableAdmin/GetSnapshot",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists all snapshots associated with the specified cluster."]
        #[doc = ""]
        #[doc = " Note: This is a private alpha release of Cloud Bigtable snapshots. This"]
        #[doc = " feature is not currently available to most Cloud Bigtable customers. This"]
        #[doc = " feature might be changed in backward-incompatible ways and is not"]
        #[doc = " recommended for production use. It is not subject to any SLA or deprecation"]
        #[doc = " policy."]
        pub async fn list_snapshots(
            &mut self,
            request: impl tonic::IntoRequest<super::ListSnapshotsRequest>,
        ) -> Result<tonic::Response<super::ListSnapshotsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.bigtable.admin.v2.BigtableTableAdmin/ListSnapshots",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Permanently deletes the specified snapshot."]
        #[doc = ""]
        #[doc = " Note: This is a private alpha release of Cloud Bigtable snapshots. This"]
        #[doc = " feature is not currently available to most Cloud Bigtable customers. This"]
        #[doc = " feature might be changed in backward-incompatible ways and is not"]
        #[doc = " recommended for production use. It is not subject to any SLA or deprecation"]
        #[doc = " policy."]
        pub async fn delete_snapshot(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteSnapshotRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.bigtable.admin.v2.BigtableTableAdmin/DeleteSnapshot",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Starts creating a new Cloud Bigtable Backup. The returned backup"]
        #[doc = " [long-running operation][google.longrunning.Operation] can be used to"]
        #[doc = " track creation of the backup. The"]
        #[doc = " [metadata][google.longrunning.Operation.metadata] field type is"]
        #[doc = " [CreateBackupMetadata][google.bigtable.admin.v2.CreateBackupMetadata]. The"]
        #[doc = " [response][google.longrunning.Operation.response] field type is"]
        #[doc = " [Backup][google.bigtable.admin.v2.Backup], if successful. Cancelling the"]
        #[doc = " returned operation will stop the creation and delete the backup."]
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
                "/google.bigtable.admin.v2.BigtableTableAdmin/CreateBackup",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets metadata on a pending or completed Cloud Bigtable Backup."]
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
                "/google.bigtable.admin.v2.BigtableTableAdmin/GetBackup",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates a pending or completed Cloud Bigtable Backup."]
        pub async fn update_backup(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateBackupRequest>,
        ) -> Result<tonic::Response<super::Backup>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.bigtable.admin.v2.BigtableTableAdmin/UpdateBackup",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes a pending or completed Cloud Bigtable backup."]
        pub async fn delete_backup(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteBackupRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.bigtable.admin.v2.BigtableTableAdmin/DeleteBackup",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists Cloud Bigtable backups. Returns both completed and pending"]
        #[doc = " backups."]
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
                "/google.bigtable.admin.v2.BigtableTableAdmin/ListBackups",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Create a new table by restoring from a completed backup. The new table"]
        #[doc = " must be in the same instance as the instance containing the backup. The"]
        #[doc = " returned table [long-running operation][google.longrunning.Operation] can"]
        #[doc = " be used to track the progress of the operation, and to cancel it. The"]
        #[doc = " [metadata][google.longrunning.Operation.metadata] field type is"]
        #[doc = " [RestoreTableMetadata][google.bigtable.admin.RestoreTableMetadata]. The"]
        #[doc = " [response][google.longrunning.Operation.response] type is"]
        #[doc = " [Table][google.bigtable.admin.v2.Table], if successful."]
        pub async fn restore_table(
            &mut self,
            request: impl tonic::IntoRequest<super::RestoreTableRequest>,
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
                "/google.bigtable.admin.v2.BigtableTableAdmin/RestoreTable",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets the access control policy for a resource."]
        #[doc = " Returns an empty policy if the resource exists but does not have a policy"]
        #[doc = " set."]
        pub async fn get_iam_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::super::super::super::iam::v1::GetIamPolicyRequest>,
        ) -> Result<tonic::Response<super::super::super::super::iam::v1::Policy>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.bigtable.admin.v2.BigtableTableAdmin/GetIamPolicy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Sets the access control policy on a Table or Backup resource."]
        #[doc = " Replaces any existing policy."]
        pub async fn set_iam_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::super::super::super::iam::v1::SetIamPolicyRequest>,
        ) -> Result<tonic::Response<super::super::super::super::iam::v1::Policy>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.bigtable.admin.v2.BigtableTableAdmin/SetIamPolicy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns permissions that the caller has on the specified table resource."]
        pub async fn test_iam_permissions(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::iam::v1::TestIamPermissionsRequest,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::iam::v1::TestIamPermissionsResponse>,
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
                "/google.bigtable.admin.v2.BigtableTableAdmin/TestIamPermissions",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for BigtableTableAdminClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for BigtableTableAdminClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "BigtableTableAdminClient {{ ... }}")
        }
    }
}
