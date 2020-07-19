/// A physical location in which a particular project can allocate Cloud BigTable
/// resources.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Zone {
    /// A permanent unique identifier for the zone.
    /// Values are of the form projects/<project>/zones/[a-z][-a-z0-9]*
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// The name of this zone as it appears in UIs.
    #[prost(string, tag = "2")]
    pub display_name: std::string::String,
    /// The current state of this zone.
    #[prost(enumeration = "zone::Status", tag = "3")]
    pub status: i32,
}
pub mod zone {
    /// Possible states of a zone.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Status {
        /// The state of the zone is unknown or unspecified.
        Unknown = 0,
        /// The zone is in a good state.
        Ok = 1,
        /// The zone is down for planned maintenance.
        PlannedMaintenance = 2,
        /// The zone is down for emergency or unplanned maintenance.
        EmergencyMainenance = 3,
    }
}
/// An isolated set of Cloud BigTable resources on which tables can be hosted.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Cluster {
    /// A permanent unique identifier for the cluster. For technical reasons, the
    /// zone in which the cluster resides is included here.
    /// Values are of the form
    /// projects/<project>/zones/<zone>/clusters/[a-z][-a-z0-9]*
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// The operation currently running on the cluster, if any.
    /// This cannot be set directly, only through CreateCluster, UpdateCluster,
    /// or UndeleteCluster. Calls to these methods will be rejected if
    /// "current_operation" is already set.
    #[prost(message, optional, tag = "3")]
    pub current_operation:
        ::std::option::Option<super::super::super::super::longrunning::Operation>,
    /// The descriptive name for this cluster as it appears in UIs.
    /// Must be unique per zone.
    #[prost(string, tag = "4")]
    pub display_name: std::string::String,
    /// The number of serve nodes allocated to this cluster.
    #[prost(int32, tag = "5")]
    pub serve_nodes: i32,
    /// What storage type to use for tables in this cluster. Only configurable at
    /// cluster creation time. If unspecified, STORAGE_SSD will be used.
    #[prost(enumeration = "StorageType", tag = "8")]
    pub default_storage_type: i32,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum StorageType {
    /// The storage type used is unspecified.
    StorageUnspecified = 0,
    /// Data will be stored in SSD, providing low and consistent latencies.
    StorageSsd = 1,
    /// Data will be stored in HDD, providing high and less predictable
    /// latencies.
    StorageHdd = 2,
}
/// Request message for BigtableClusterService.ListZones.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListZonesRequest {
    /// The unique name of the project for which a list of supported zones is
    /// requested.
    /// Values are of the form projects/<project>
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Response message for BigtableClusterService.ListZones.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListZonesResponse {
    /// The list of requested zones.
    #[prost(message, repeated, tag = "1")]
    pub zones: ::std::vec::Vec<Zone>,
}
/// Request message for BigtableClusterService.GetCluster.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetClusterRequest {
    /// The unique name of the requested cluster.
    /// Values are of the form projects/<project>/zones/<zone>/clusters/<cluster>
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Request message for BigtableClusterService.ListClusters.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListClustersRequest {
    /// The unique name of the project for which a list of clusters is requested.
    /// Values are of the form projects/<project>
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Response message for BigtableClusterService.ListClusters.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListClustersResponse {
    /// The list of requested Clusters.
    #[prost(message, repeated, tag = "1")]
    pub clusters: ::std::vec::Vec<Cluster>,
    /// The zones for which clusters could not be retrieved.
    #[prost(message, repeated, tag = "2")]
    pub failed_zones: ::std::vec::Vec<Zone>,
}
/// Request message for BigtableClusterService.CreateCluster.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateClusterRequest {
    /// The unique name of the zone in which to create the cluster.
    /// Values are of the form projects/<project>/zones/<zone>
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// The id to be used when referring to the new cluster within its zone,
    /// e.g. just the "test-cluster" section of the full name
    /// "projects/<project>/zones/<zone>/clusters/test-cluster".
    #[prost(string, tag = "2")]
    pub cluster_id: std::string::String,
    /// The cluster to create.
    /// The "name", "delete_time", and "current_operation" fields must be left
    /// blank.
    #[prost(message, optional, tag = "3")]
    pub cluster: ::std::option::Option<Cluster>,
}
/// Metadata type for the operation returned by
/// BigtableClusterService.CreateCluster.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateClusterMetadata {
    /// The request which prompted the creation of this operation.
    #[prost(message, optional, tag = "1")]
    pub original_request: ::std::option::Option<CreateClusterRequest>,
    /// The time at which original_request was received.
    #[prost(message, optional, tag = "2")]
    pub request_time: ::std::option::Option<::prost_types::Timestamp>,
    /// The time at which this operation failed or was completed successfully.
    #[prost(message, optional, tag = "3")]
    pub finish_time: ::std::option::Option<::prost_types::Timestamp>,
}
/// Metadata type for the operation returned by
/// BigtableClusterService.UpdateCluster.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateClusterMetadata {
    /// The request which prompted the creation of this operation.
    #[prost(message, optional, tag = "1")]
    pub original_request: ::std::option::Option<Cluster>,
    /// The time at which original_request was received.
    #[prost(message, optional, tag = "2")]
    pub request_time: ::std::option::Option<::prost_types::Timestamp>,
    /// The time at which this operation was cancelled. If set, this operation is
    /// in the process of undoing itself (which is guaranteed to succeed) and
    /// cannot be cancelled again.
    #[prost(message, optional, tag = "3")]
    pub cancel_time: ::std::option::Option<::prost_types::Timestamp>,
    /// The time at which this operation failed or was completed successfully.
    #[prost(message, optional, tag = "4")]
    pub finish_time: ::std::option::Option<::prost_types::Timestamp>,
}
/// Request message for BigtableClusterService.DeleteCluster.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteClusterRequest {
    /// The unique name of the cluster to be deleted.
    /// Values are of the form projects/<project>/zones/<zone>/clusters/<cluster>
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Request message for BigtableClusterService.UndeleteCluster.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UndeleteClusterRequest {
    /// The unique name of the cluster to be un-deleted.
    /// Values are of the form projects/<project>/zones/<zone>/clusters/<cluster>
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Metadata type for the operation returned by
/// BigtableClusterService.UndeleteCluster.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UndeleteClusterMetadata {
    /// The time at which the original request was received.
    #[prost(message, optional, tag = "1")]
    pub request_time: ::std::option::Option<::prost_types::Timestamp>,
    /// The time at which this operation failed or was completed successfully.
    #[prost(message, optional, tag = "2")]
    pub finish_time: ::std::option::Option<::prost_types::Timestamp>,
}
/// Metadata type for operations initiated by the V2 BigtableAdmin service.
/// More complete information for such operations is available via the V2 API.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct V2OperationMetadata {}
#[doc = r" Generated client implementations."]
pub mod bigtable_cluster_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Service for managing zonal Cloud Bigtable resources."]
    pub struct BigtableClusterServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> BigtableClusterServiceClient<T>
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
        #[doc = " Lists the supported zones for the given project."]
        pub async fn list_zones(
            &mut self,
            request: impl tonic::IntoRequest<super::ListZonesRequest>,
        ) -> Result<tonic::Response<super::ListZonesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.bigtable.admin.cluster.v1.BigtableClusterService/ListZones",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets information about a particular cluster."]
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
                "/google.bigtable.admin.cluster.v1.BigtableClusterService/GetCluster",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists all clusters in the given project, along with any zones for which"]
        #[doc = " cluster information could not be retrieved."]
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
                "/google.bigtable.admin.cluster.v1.BigtableClusterService/ListClusters",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a cluster and begins preparing it to begin serving. The returned"]
        #[doc = " cluster embeds as its \"current_operation\" a long-running operation which"]
        #[doc = " can be used to track the progress of turning up the new cluster."]
        #[doc = " Immediately upon completion of this request:"]
        #[doc = "  * The cluster will be readable via the API, with all requested attributes"]
        #[doc = "    but no allocated resources."]
        #[doc = " Until completion of the embedded operation:"]
        #[doc = "  * Cancelling the operation will render the cluster immediately unreadable"]
        #[doc = "    via the API."]
        #[doc = "  * All other attempts to modify or delete the cluster will be rejected."]
        #[doc = " Upon completion of the embedded operation:"]
        #[doc = "  * Billing for all successfully-allocated resources will begin (some types"]
        #[doc = "    may have lower than the requested levels)."]
        #[doc = "  * New tables can be created in the cluster."]
        #[doc = "  * The cluster's allocated resource levels will be readable via the API."]
        #[doc = " The embedded operation's \"metadata\" field type is"]
        #[doc = " [CreateClusterMetadata][google.bigtable.admin.cluster.v1.CreateClusterMetadata]"]
        #[doc = " The embedded operation's \"response\" field type is"]
        #[doc = " [Cluster][google.bigtable.admin.cluster.v1.Cluster], if successful."]
        pub async fn create_cluster(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateClusterRequest>,
        ) -> Result<tonic::Response<super::Cluster>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.bigtable.admin.cluster.v1.BigtableClusterService/CreateCluster",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates a cluster, and begins allocating or releasing resources as"]
        #[doc = " requested. The returned cluster embeds as its \"current_operation\" a"]
        #[doc = " long-running operation which can be used to track the progress of updating"]
        #[doc = " the cluster."]
        #[doc = " Immediately upon completion of this request:"]
        #[doc = "  * For resource types where a decrease in the cluster's allocation has been"]
        #[doc = "    requested, billing will be based on the newly-requested level."]
        #[doc = " Until completion of the embedded operation:"]
        #[doc = "  * Cancelling the operation will set its metadata's \"cancelled_at_time\","]
        #[doc = "    and begin restoring resources to their pre-request values. The operation"]
        #[doc = "    is guaranteed to succeed at undoing all resource changes, after which"]
        #[doc = "    point it will terminate with a CANCELLED status."]
        #[doc = "  * All other attempts to modify or delete the cluster will be rejected."]
        #[doc = "  * Reading the cluster via the API will continue to give the pre-request"]
        #[doc = "    resource levels."]
        #[doc = " Upon completion of the embedded operation:"]
        #[doc = "  * Billing will begin for all successfully-allocated resources (some types"]
        #[doc = "    may have lower than the requested levels)."]
        #[doc = "  * All newly-reserved resources will be available for serving the cluster's"]
        #[doc = "    tables."]
        #[doc = "  * The cluster's new resource levels will be readable via the API."]
        #[doc = " [UpdateClusterMetadata][google.bigtable.admin.cluster.v1.UpdateClusterMetadata]"]
        #[doc = " The embedded operation's \"response\" field type is"]
        #[doc = " [Cluster][google.bigtable.admin.cluster.v1.Cluster], if successful."]
        pub async fn update_cluster(
            &mut self,
            request: impl tonic::IntoRequest<super::Cluster>,
        ) -> Result<tonic::Response<super::Cluster>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.bigtable.admin.cluster.v1.BigtableClusterService/UpdateCluster",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Marks a cluster and all of its tables for permanent deletion in 7 days."]
        #[doc = " Immediately upon completion of the request:"]
        #[doc = "  * Billing will cease for all of the cluster's reserved resources."]
        #[doc = "  * The cluster's \"delete_time\" field will be set 7 days in the future."]
        #[doc = " Soon afterward:"]
        #[doc = "  * All tables within the cluster will become unavailable."]
        #[doc = " Prior to the cluster's \"delete_time\":"]
        #[doc = "  * The cluster can be recovered with a call to UndeleteCluster."]
        #[doc = "  * All other attempts to modify or delete the cluster will be rejected."]
        #[doc = " At the cluster's \"delete_time\":"]
        #[doc = "  * The cluster and *all of its tables* will immediately and irrevocably"]
        #[doc = "    disappear from the API, and their data will be permanently deleted."]
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
                "/google.bigtable.admin.cluster.v1.BigtableClusterService/DeleteCluster",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Cancels the scheduled deletion of an cluster and begins preparing it to"]
        #[doc = " resume serving. The returned operation will also be embedded as the"]
        #[doc = " cluster's \"current_operation\"."]
        #[doc = " Immediately upon completion of this request:"]
        #[doc = "  * The cluster's \"delete_time\" field will be unset, protecting it from"]
        #[doc = "    automatic deletion."]
        #[doc = " Until completion of the returned operation:"]
        #[doc = "  * The operation cannot be cancelled."]
        #[doc = " Upon completion of the returned operation:"]
        #[doc = "  * Billing for the cluster's resources will resume."]
        #[doc = "  * All tables within the cluster will be available."]
        #[doc = " [UndeleteClusterMetadata][google.bigtable.admin.cluster.v1.UndeleteClusterMetadata]"]
        #[doc = " The embedded operation's \"response\" field type is"]
        #[doc = " [Cluster][google.bigtable.admin.cluster.v1.Cluster], if successful."]
        pub async fn undelete_cluster(
            &mut self,
            request: impl tonic::IntoRequest<super::UndeleteClusterRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::super::longrunning::Operation>,
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
                "/google.bigtable.admin.cluster.v1.BigtableClusterService/UndeleteCluster",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for BigtableClusterServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for BigtableClusterServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "BigtableClusterServiceClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod bigtable_cluster_service_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with BigtableClusterServiceServer."]
    #[async_trait]
    pub trait BigtableClusterService: Send + Sync + 'static {
        #[doc = " Lists the supported zones for the given project."]
        async fn list_zones(
            &self,
            request: tonic::Request<super::ListZonesRequest>,
        ) -> Result<tonic::Response<super::ListZonesResponse>, tonic::Status>;
        #[doc = " Gets information about a particular cluster."]
        async fn get_cluster(
            &self,
            request: tonic::Request<super::GetClusterRequest>,
        ) -> Result<tonic::Response<super::Cluster>, tonic::Status>;
        #[doc = " Lists all clusters in the given project, along with any zones for which"]
        #[doc = " cluster information could not be retrieved."]
        async fn list_clusters(
            &self,
            request: tonic::Request<super::ListClustersRequest>,
        ) -> Result<tonic::Response<super::ListClustersResponse>, tonic::Status>;
        #[doc = " Creates a cluster and begins preparing it to begin serving. The returned"]
        #[doc = " cluster embeds as its \"current_operation\" a long-running operation which"]
        #[doc = " can be used to track the progress of turning up the new cluster."]
        #[doc = " Immediately upon completion of this request:"]
        #[doc = "  * The cluster will be readable via the API, with all requested attributes"]
        #[doc = "    but no allocated resources."]
        #[doc = " Until completion of the embedded operation:"]
        #[doc = "  * Cancelling the operation will render the cluster immediately unreadable"]
        #[doc = "    via the API."]
        #[doc = "  * All other attempts to modify or delete the cluster will be rejected."]
        #[doc = " Upon completion of the embedded operation:"]
        #[doc = "  * Billing for all successfully-allocated resources will begin (some types"]
        #[doc = "    may have lower than the requested levels)."]
        #[doc = "  * New tables can be created in the cluster."]
        #[doc = "  * The cluster's allocated resource levels will be readable via the API."]
        #[doc = " The embedded operation's \"metadata\" field type is"]
        #[doc = " [CreateClusterMetadata][google.bigtable.admin.cluster.v1.CreateClusterMetadata]"]
        #[doc = " The embedded operation's \"response\" field type is"]
        #[doc = " [Cluster][google.bigtable.admin.cluster.v1.Cluster], if successful."]
        async fn create_cluster(
            &self,
            request: tonic::Request<super::CreateClusterRequest>,
        ) -> Result<tonic::Response<super::Cluster>, tonic::Status>;
        #[doc = " Updates a cluster, and begins allocating or releasing resources as"]
        #[doc = " requested. The returned cluster embeds as its \"current_operation\" a"]
        #[doc = " long-running operation which can be used to track the progress of updating"]
        #[doc = " the cluster."]
        #[doc = " Immediately upon completion of this request:"]
        #[doc = "  * For resource types where a decrease in the cluster's allocation has been"]
        #[doc = "    requested, billing will be based on the newly-requested level."]
        #[doc = " Until completion of the embedded operation:"]
        #[doc = "  * Cancelling the operation will set its metadata's \"cancelled_at_time\","]
        #[doc = "    and begin restoring resources to their pre-request values. The operation"]
        #[doc = "    is guaranteed to succeed at undoing all resource changes, after which"]
        #[doc = "    point it will terminate with a CANCELLED status."]
        #[doc = "  * All other attempts to modify or delete the cluster will be rejected."]
        #[doc = "  * Reading the cluster via the API will continue to give the pre-request"]
        #[doc = "    resource levels."]
        #[doc = " Upon completion of the embedded operation:"]
        #[doc = "  * Billing will begin for all successfully-allocated resources (some types"]
        #[doc = "    may have lower than the requested levels)."]
        #[doc = "  * All newly-reserved resources will be available for serving the cluster's"]
        #[doc = "    tables."]
        #[doc = "  * The cluster's new resource levels will be readable via the API."]
        #[doc = " [UpdateClusterMetadata][google.bigtable.admin.cluster.v1.UpdateClusterMetadata]"]
        #[doc = " The embedded operation's \"response\" field type is"]
        #[doc = " [Cluster][google.bigtable.admin.cluster.v1.Cluster], if successful."]
        async fn update_cluster(
            &self,
            request: tonic::Request<super::Cluster>,
        ) -> Result<tonic::Response<super::Cluster>, tonic::Status>;
        #[doc = " Marks a cluster and all of its tables for permanent deletion in 7 days."]
        #[doc = " Immediately upon completion of the request:"]
        #[doc = "  * Billing will cease for all of the cluster's reserved resources."]
        #[doc = "  * The cluster's \"delete_time\" field will be set 7 days in the future."]
        #[doc = " Soon afterward:"]
        #[doc = "  * All tables within the cluster will become unavailable."]
        #[doc = " Prior to the cluster's \"delete_time\":"]
        #[doc = "  * The cluster can be recovered with a call to UndeleteCluster."]
        #[doc = "  * All other attempts to modify or delete the cluster will be rejected."]
        #[doc = " At the cluster's \"delete_time\":"]
        #[doc = "  * The cluster and *all of its tables* will immediately and irrevocably"]
        #[doc = "    disappear from the API, and their data will be permanently deleted."]
        async fn delete_cluster(
            &self,
            request: tonic::Request<super::DeleteClusterRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status>;
        #[doc = " Cancels the scheduled deletion of an cluster and begins preparing it to"]
        #[doc = " resume serving. The returned operation will also be embedded as the"]
        #[doc = " cluster's \"current_operation\"."]
        #[doc = " Immediately upon completion of this request:"]
        #[doc = "  * The cluster's \"delete_time\" field will be unset, protecting it from"]
        #[doc = "    automatic deletion."]
        #[doc = " Until completion of the returned operation:"]
        #[doc = "  * The operation cannot be cancelled."]
        #[doc = " Upon completion of the returned operation:"]
        #[doc = "  * Billing for the cluster's resources will resume."]
        #[doc = "  * All tables within the cluster will be available."]
        #[doc = " [UndeleteClusterMetadata][google.bigtable.admin.cluster.v1.UndeleteClusterMetadata]"]
        #[doc = " The embedded operation's \"response\" field type is"]
        #[doc = " [Cluster][google.bigtable.admin.cluster.v1.Cluster], if successful."]
        async fn undelete_cluster(
            &self,
            request: tonic::Request<super::UndeleteClusterRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::super::longrunning::Operation>,
            tonic::Status,
        >;
    }
    #[doc = " Service for managing zonal Cloud Bigtable resources."]
    #[derive(Debug)]
    pub struct BigtableClusterServiceServer<T: BigtableClusterService> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: BigtableClusterService> BigtableClusterServiceServer<T> {
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
    impl<T, B> Service<http::Request<B>> for BigtableClusterServiceServer<T>
    where
        T: BigtableClusterService,
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
                "/google.bigtable.admin.cluster.v1.BigtableClusterService/ListZones" => {
                    #[allow(non_camel_case_types)]
                    struct ListZonesSvc<T: BigtableClusterService>(pub Arc<T>);
                    impl<T: BigtableClusterService>
                        tonic::server::UnaryService<super::ListZonesRequest> for ListZonesSvc<T>
                    {
                        type Response = super::ListZonesResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListZonesRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).list_zones(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ListZonesSvc(inner);
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
                "/google.bigtable.admin.cluster.v1.BigtableClusterService/GetCluster" => {
                    #[allow(non_camel_case_types)]
                    struct GetClusterSvc<T: BigtableClusterService>(pub Arc<T>);
                    impl<T: BigtableClusterService>
                        tonic::server::UnaryService<super::GetClusterRequest> for GetClusterSvc<T>
                    {
                        type Response = super::Cluster;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetClusterRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_cluster(request).await };
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
                "/google.bigtable.admin.cluster.v1.BigtableClusterService/ListClusters" => {
                    #[allow(non_camel_case_types)]
                    struct ListClustersSvc<T: BigtableClusterService>(pub Arc<T>);
                    impl<T: BigtableClusterService>
                        tonic::server::UnaryService<super::ListClustersRequest>
                        for ListClustersSvc<T>
                    {
                        type Response = super::ListClustersResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListClustersRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).list_clusters(request).await };
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
                "/google.bigtable.admin.cluster.v1.BigtableClusterService/CreateCluster" => {
                    #[allow(non_camel_case_types)]
                    struct CreateClusterSvc<T: BigtableClusterService>(pub Arc<T>);
                    impl<T: BigtableClusterService>
                        tonic::server::UnaryService<super::CreateClusterRequest>
                        for CreateClusterSvc<T>
                    {
                        type Response = super::Cluster;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateClusterRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).create_cluster(request).await };
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
                "/google.bigtable.admin.cluster.v1.BigtableClusterService/UpdateCluster" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateClusterSvc<T: BigtableClusterService>(pub Arc<T>);
                    impl<T: BigtableClusterService> tonic::server::UnaryService<super::Cluster>
                        for UpdateClusterSvc<T>
                    {
                        type Response = super::Cluster;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::Cluster>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).update_cluster(request).await };
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
                "/google.bigtable.admin.cluster.v1.BigtableClusterService/DeleteCluster" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteClusterSvc<T: BigtableClusterService>(pub Arc<T>);
                    impl<T: BigtableClusterService>
                        tonic::server::UnaryService<super::DeleteClusterRequest>
                        for DeleteClusterSvc<T>
                    {
                        type Response = ();
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteClusterRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).delete_cluster(request).await };
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
                "/google.bigtable.admin.cluster.v1.BigtableClusterService/UndeleteCluster" => {
                    #[allow(non_camel_case_types)]
                    struct UndeleteClusterSvc<T: BigtableClusterService>(pub Arc<T>);
                    impl<T: BigtableClusterService>
                        tonic::server::UnaryService<super::UndeleteClusterRequest>
                        for UndeleteClusterSvc<T>
                    {
                        type Response = super::super::super::super::super::longrunning::Operation;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UndeleteClusterRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).undelete_cluster(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = UndeleteClusterSvc(inner);
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
    impl<T: BigtableClusterService> Clone for BigtableClusterServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: BigtableClusterService> Clone for _Inner<T> {
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
