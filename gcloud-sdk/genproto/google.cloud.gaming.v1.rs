/// Represents the metadata of the long-running operation.
#[allow(clippy::derive_partial_eq_without_eq)]
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
    /// Output only. List of Locations that could not be reached.
    #[prost(string, repeated, tag = "8")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Output only. Operation status for Game Services API operations. Operation status is in
    /// the form of key-value pairs where keys are resource IDs and the values show
    /// the status of the operation. In case of failures, the value includes an
    /// error code and error message.
    #[prost(map = "string, message", tag = "9")]
    pub operation_status: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        OperationStatus,
    >,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationStatus {
    /// Output only. Whether the operation is done or still in progress.
    #[prost(bool, tag = "1")]
    pub done: bool,
    /// The error code in case of failures.
    #[prost(enumeration = "operation_status::ErrorCode", tag = "2")]
    pub error_code: i32,
    /// The human-readable error message.
    #[prost(string, tag = "3")]
    pub error_message: ::prost::alloc::string::String,
}
/// Nested message and enum types in `OperationStatus`.
pub mod operation_status {
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum ErrorCode {
        Unspecified = 0,
        InternalError = 1,
        PermissionDenied = 2,
        ClusterConnection = 3,
    }
    impl ErrorCode {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ErrorCode::Unspecified => "ERROR_CODE_UNSPECIFIED",
                ErrorCode::InternalError => "INTERNAL_ERROR",
                ErrorCode::PermissionDenied => "PERMISSION_DENIED",
                ErrorCode::ClusterConnection => "CLUSTER_CONNECTION",
            }
        }
    }
}
/// The label selector, used to group labels on the resources.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LabelSelector {
    /// Resource labels for this selector.
    #[prost(map = "string, string", tag = "1")]
    pub labels: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
}
/// The realm selector, used to match realm resources.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RealmSelector {
    /// List of realms to match.
    #[prost(string, repeated, tag = "1")]
    pub realms: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// The schedule of a recurring or one time event. The event's time span is
/// specified by start_time and end_time. If the scheduled event's timespan is
/// larger than the cron_spec + cron_job_duration, the event will be recurring.
/// If only cron_spec + cron_job_duration are specified, the event is effective
/// starting at the local time specified by cron_spec, and is recurring.
///
/// ```
/// start_time|-------[cron job]-------[cron job]-------[cron job]---|end_time
/// cron job: cron spec start time + duration
/// ```
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Schedule {
    /// The start time of the event.
    #[prost(message, optional, tag = "1")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The end time of the event.
    #[prost(message, optional, tag = "2")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The duration for the cron job event. The duration of the event is effective
    /// after the cron job's start time.
    #[prost(message, optional, tag = "3")]
    pub cron_job_duration: ::core::option::Option<::prost_types::Duration>,
    /// The cron definition of the scheduled event. See
    /// <https://en.wikipedia.org/wiki/Cron.> Cron spec specifies the local time as
    /// defined by the realm.
    #[prost(string, tag = "4")]
    pub cron_spec: ::prost::alloc::string::String,
}
/// Encapsulates Agones fleet spec and Agones autoscaler spec sources.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpecSource {
    /// The game server config resource. Uses the form:
    /// `projects/{project}/locations/{location}/gameServerDeployments/{deployment_id}/configs/{config_id}`.
    #[prost(string, tag = "1")]
    pub game_server_config_name: ::prost::alloc::string::String,
    /// The name of the Agones leet config or Agones scaling config used to derive
    /// the Agones fleet or Agones autoscaler spec.
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
}
/// Details about the Agones resources.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TargetDetails {
    /// The game server cluster name. Uses the form:
    /// `projects/{project}/locations/{location}/realms/{realm}/gameServerClusters/{cluster}`.
    #[prost(string, tag = "1")]
    pub game_server_cluster_name: ::prost::alloc::string::String,
    /// The game server deployment name. Uses the form:
    /// `projects/{project}/locations/{location}/gameServerDeployments/{deployment_id}`.
    #[prost(string, tag = "2")]
    pub game_server_deployment_name: ::prost::alloc::string::String,
    /// Agones fleet details for game server clusters and game server deployments.
    #[prost(message, repeated, tag = "3")]
    pub fleet_details: ::prost::alloc::vec::Vec<target_details::TargetFleetDetails>,
}
/// Nested message and enum types in `TargetDetails`.
pub mod target_details {
    /// Details of the target Agones fleet.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TargetFleetDetails {
        /// Reference to target Agones fleet.
        #[prost(message, optional, tag = "1")]
        pub fleet: ::core::option::Option<target_fleet_details::TargetFleet>,
        /// Reference to target Agones fleet autoscaling policy.
        #[prost(message, optional, tag = "2")]
        pub autoscaler: ::core::option::Option<
            target_fleet_details::TargetFleetAutoscaler,
        >,
    }
    /// Nested message and enum types in `TargetFleetDetails`.
    pub mod target_fleet_details {
        /// Target Agones fleet specification.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct TargetFleet {
            /// The name of the Agones fleet.
            #[prost(string, tag = "1")]
            pub name: ::prost::alloc::string::String,
            /// Encapsulates the source of the Agones fleet spec.
            /// The Agones fleet spec source.
            #[prost(message, optional, tag = "2")]
            pub spec_source: ::core::option::Option<super::super::SpecSource>,
        }
        /// Target Agones autoscaler policy reference.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct TargetFleetAutoscaler {
            /// The name of the Agones autoscaler.
            #[prost(string, tag = "1")]
            pub name: ::prost::alloc::string::String,
            /// Encapsulates the source of the Agones fleet spec.
            /// Details about the Agones autoscaler spec.
            #[prost(message, optional, tag = "2")]
            pub spec_source: ::core::option::Option<super::super::SpecSource>,
        }
    }
}
/// Encapsulates the Target state.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TargetState {
    /// Details about Agones fleets.
    #[prost(message, repeated, tag = "1")]
    pub details: ::prost::alloc::vec::Vec<TargetDetails>,
}
/// Details of the deployed Agones fleet.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeployedFleetDetails {
    /// Information about the Agones fleet.
    #[prost(message, optional, tag = "1")]
    pub deployed_fleet: ::core::option::Option<deployed_fleet_details::DeployedFleet>,
    /// Information about the Agones autoscaler for that fleet.
    #[prost(message, optional, tag = "2")]
    pub deployed_autoscaler: ::core::option::Option<
        deployed_fleet_details::DeployedFleetAutoscaler,
    >,
}
/// Nested message and enum types in `DeployedFleetDetails`.
pub mod deployed_fleet_details {
    /// Agones fleet specification and details.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DeployedFleet {
        /// The name of the Agones fleet.
        #[prost(string, tag = "1")]
        pub fleet: ::prost::alloc::string::String,
        /// The fleet spec retrieved from the Agones fleet.
        #[prost(string, tag = "2")]
        pub fleet_spec: ::prost::alloc::string::String,
        /// The source spec that is used to create the Agones fleet.
        /// The GameServerConfig resource may no longer exist in the system.
        #[prost(message, optional, tag = "3")]
        pub spec_source: ::core::option::Option<super::SpecSource>,
        /// The current status of the Agones fleet.
        /// Includes count of game servers in various states.
        #[prost(message, optional, tag = "5")]
        pub status: ::core::option::Option<deployed_fleet::DeployedFleetStatus>,
    }
    /// Nested message and enum types in `DeployedFleet`.
    pub mod deployed_fleet {
        /// DeployedFleetStatus has details about the Agones fleets such as how many
        /// are running, how many allocated, and so on.
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct DeployedFleetStatus {
            /// The number of GameServer replicas in the READY state in this fleet.
            #[prost(int64, tag = "1")]
            pub ready_replicas: i64,
            /// The number of GameServer replicas in the ALLOCATED state in this fleet.
            #[prost(int64, tag = "2")]
            pub allocated_replicas: i64,
            /// The number of GameServer replicas in the RESERVED state in this fleet.
            /// Reserved instances won't be deleted on scale down, but won't cause
            /// an autoscaler to scale up.
            #[prost(int64, tag = "3")]
            pub reserved_replicas: i64,
            /// The total number of current GameServer replicas in this fleet.
            #[prost(int64, tag = "4")]
            pub replicas: i64,
        }
    }
    /// Details about the Agones autoscaler.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DeployedFleetAutoscaler {
        /// The name of the Agones autoscaler.
        #[prost(string, tag = "1")]
        pub autoscaler: ::prost::alloc::string::String,
        /// The source spec that is used to create the autoscaler.
        /// The GameServerConfig resource may no longer exist in the system.
        #[prost(message, optional, tag = "4")]
        pub spec_source: ::core::option::Option<super::SpecSource>,
        /// The autoscaler spec retrieved from Agones.
        #[prost(string, tag = "3")]
        pub fleet_autoscaler_spec: ::prost::alloc::string::String,
    }
}
/// Request message for GameServerClustersService.ListGameServerClusters.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListGameServerClustersRequest {
    /// Required. The parent resource name, in the following form:
    /// "projects/{project}/locations/{location}/realms/{realm}".
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. The maximum number of items to return. If unspecified, the server
    /// will pick an appropriate default. The server may return fewer items than
    /// requested. A caller should only rely on response's
    /// \[next_page_token][google.cloud.gaming.v1.ListGameServerClustersResponse.next_page_token\] to
    /// determine if there are more GameServerClusters left to be queried.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. The next_page_token value returned from a previous List request, if any.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. The filter to apply to list results.
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Optional. Specifies the ordering of results following syntax at
    /// <https://cloud.google.com/apis/design/design_patterns#sorting_order.>
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
    /// Optional. View for the returned GameServerCluster objects. When `FULL` is
    /// specified, the `cluster_state` field is also returned in the
    /// GameServerCluster object, which includes the state of the referenced
    /// Kubernetes cluster such as versions and provider info. The default/unset
    /// value is GAME_SERVER_CLUSTER_VIEW_UNSPECIFIED, same as BASIC, which does
    /// not return the `cluster_state` field.
    #[prost(enumeration = "GameServerClusterView", tag = "6")]
    pub view: i32,
}
/// Response message for GameServerClustersService.ListGameServerClusters.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListGameServerClustersResponse {
    /// The list of game server clusters.
    #[prost(message, repeated, tag = "1")]
    pub game_server_clusters: ::prost::alloc::vec::Vec<GameServerCluster>,
    /// Token to retrieve the next page of results, or empty if there are no more
    /// results in the list.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// List of locations that could not be reached.
    #[prost(string, repeated, tag = "4")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Request message for GameServerClustersService.GetGameServerCluster.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetGameServerClusterRequest {
    /// Required. The name of the game server cluster to retrieve, in the following form:
    /// `projects/{project}/locations/{location}/realms/{realm-id}/gameServerClusters/{cluster}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. View for the returned GameServerCluster objects. When `FULL` is
    /// specified, the `cluster_state` field is also returned in the
    /// GameServerCluster object, which includes the state of the referenced
    /// Kubernetes cluster such as versions and provider info. The default/unset
    /// value is GAME_SERVER_CLUSTER_VIEW_UNSPECIFIED, same as BASIC, which does
    /// not return the `cluster_state` field.
    #[prost(enumeration = "GameServerClusterView", tag = "6")]
    pub view: i32,
}
/// Request message for GameServerClustersService.CreateGameServerCluster.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateGameServerClusterRequest {
    /// Required. The parent resource name, in the following form:
    /// `projects/{project}/locations/{location}/realms/{realm-id}`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The ID of the game server cluster resource to be created.
    #[prost(string, tag = "2")]
    pub game_server_cluster_id: ::prost::alloc::string::String,
    /// Required. The game server cluster resource to be created.
    #[prost(message, optional, tag = "3")]
    pub game_server_cluster: ::core::option::Option<GameServerCluster>,
}
/// Request message for GameServerClustersService.PreviewCreateGameServerCluster.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PreviewCreateGameServerClusterRequest {
    /// Required. The parent resource name, in the following form:
    /// `projects/{project}/locations/{location}/realms/{realm}`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The ID of the game server cluster resource to be created.
    #[prost(string, tag = "2")]
    pub game_server_cluster_id: ::prost::alloc::string::String,
    /// Required. The game server cluster resource to be created.
    #[prost(message, optional, tag = "3")]
    pub game_server_cluster: ::core::option::Option<GameServerCluster>,
    /// Optional. The target timestamp to compute the preview.
    #[prost(message, optional, tag = "4")]
    pub preview_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Optional. This field is deprecated, preview will always return
    /// KubernetesClusterState.
    #[deprecated]
    #[prost(enumeration = "GameServerClusterView", tag = "6")]
    pub view: i32,
}
/// Response message for
/// GameServerClustersService.PreviewCreateGameServerCluster.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PreviewCreateGameServerClusterResponse {
    /// The ETag of the game server cluster.
    #[prost(string, tag = "2")]
    pub etag: ::prost::alloc::string::String,
    /// The target state.
    #[prost(message, optional, tag = "3")]
    pub target_state: ::core::option::Option<TargetState>,
    /// Output only. The state of the Kubernetes cluster in preview, this will be available if
    /// 'view' is set to `FULL` in the relevant List/Get/Preview request.
    #[prost(message, optional, tag = "4")]
    pub cluster_state: ::core::option::Option<KubernetesClusterState>,
}
/// Request message for GameServerClustersService.DeleteGameServerCluster.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteGameServerClusterRequest {
    /// Required. The name of the game server cluster to delete, in the following form:
    /// `projects/{project}/locations/{location}/gameServerClusters/{cluster}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for GameServerClustersService.PreviewDeleteGameServerCluster.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PreviewDeleteGameServerClusterRequest {
    /// Required. The name of the game server cluster to delete, in the following form:
    /// `projects/{project}/locations/{location}/gameServerClusters/{cluster}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. The target timestamp to compute the preview.
    #[prost(message, optional, tag = "2")]
    pub preview_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Response message for
/// GameServerClustersService.PreviewDeleteGameServerCluster.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PreviewDeleteGameServerClusterResponse {
    /// The ETag of the game server cluster.
    #[prost(string, tag = "2")]
    pub etag: ::prost::alloc::string::String,
    /// The target state.
    #[prost(message, optional, tag = "3")]
    pub target_state: ::core::option::Option<TargetState>,
}
/// Request message for GameServerClustersService.UpdateGameServerCluster.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateGameServerClusterRequest {
    /// Required. The game server cluster to be updated.
    /// Only fields specified in update_mask are updated.
    #[prost(message, optional, tag = "1")]
    pub game_server_cluster: ::core::option::Option<GameServerCluster>,
    /// Required. Mask of fields to update. At least one path must be supplied in
    /// this field. For the `FieldMask` definition, see
    /// <https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#fieldmask>
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for GameServerClustersService.UpdateGameServerCluster.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PreviewUpdateGameServerClusterRequest {
    /// Required. The game server cluster to be updated.
    /// Only fields specified in update_mask are updated.
    #[prost(message, optional, tag = "1")]
    pub game_server_cluster: ::core::option::Option<GameServerCluster>,
    /// Required. Mask of fields to update. At least one path must be supplied in
    /// this field. For the `FieldMask` definition, see
    /// <https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#fieldmask>
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Optional. The target timestamp to compute the preview.
    #[prost(message, optional, tag = "3")]
    pub preview_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Response message for GameServerClustersService.PreviewUpdateGameServerCluster
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PreviewUpdateGameServerClusterResponse {
    /// The ETag of the game server cluster.
    #[prost(string, tag = "2")]
    pub etag: ::prost::alloc::string::String,
    /// The target state.
    #[prost(message, optional, tag = "3")]
    pub target_state: ::core::option::Option<TargetState>,
}
/// The game server cluster connection information.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GameServerClusterConnectionInfo {
    /// Namespace designated on the game server cluster where the Agones game
    /// server instances will be created. Existence of the namespace will be
    /// validated during creation.
    #[prost(string, tag = "5")]
    pub namespace: ::prost::alloc::string::String,
    /// The location of the Kubernetes cluster.
    #[prost(oneof = "game_server_cluster_connection_info::ClusterReference", tags = "7")]
    pub cluster_reference: ::core::option::Option<
        game_server_cluster_connection_info::ClusterReference,
    >,
}
/// Nested message and enum types in `GameServerClusterConnectionInfo`.
pub mod game_server_cluster_connection_info {
    /// The location of the Kubernetes cluster.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ClusterReference {
        /// Reference to the GKE cluster where the game servers are installed.
        #[prost(message, tag = "7")]
        GkeClusterReference(super::GkeClusterReference),
    }
}
/// A reference to a GKE cluster.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GkeClusterReference {
    /// The full or partial name of a GKE cluster, using one of the following
    /// forms:
    ///   * `projects/{project}/locations/{location}/clusters/{cluster}`
    ///   * `locations/{location}/clusters/{cluster}`
    ///   * `{cluster}`
    /// If project and location are not specified, the project and location of the
    /// GameServerCluster resource are used to generate the full name of the
    /// GKE cluster.
    #[prost(string, tag = "1")]
    pub cluster: ::prost::alloc::string::String,
}
/// A game server cluster resource.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GameServerCluster {
    /// Required. The resource name of the game server cluster, in the following form:
    /// `projects/{project}/locations/{location}/realms/{realm}/gameServerClusters/{cluster}`.
    /// For example,
    /// `projects/my-project/locations/{location}/realms/zanzibar/gameServerClusters/my-onprem-cluster`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. The creation time.
    #[prost(message, optional, tag = "2")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The last-modified time.
    #[prost(message, optional, tag = "3")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The labels associated with this game server cluster. Each label is a
    /// key-value pair.
    #[prost(map = "string, string", tag = "4")]
    pub labels: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// The game server cluster connection information. This information is used to
    /// manage game server clusters.
    #[prost(message, optional, tag = "5")]
    pub connection_info: ::core::option::Option<GameServerClusterConnectionInfo>,
    /// ETag of the resource.
    #[prost(string, tag = "6")]
    pub etag: ::prost::alloc::string::String,
    /// Human readable description of the cluster.
    #[prost(string, tag = "7")]
    pub description: ::prost::alloc::string::String,
    /// Output only. The state of the Kubernetes cluster, this will be available if
    /// 'view' is set to `FULL` in the relevant List/Get/Preview request.
    #[prost(message, optional, tag = "11")]
    pub cluster_state: ::core::option::Option<KubernetesClusterState>,
}
/// The state of the Kubernetes cluster.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KubernetesClusterState {
    /// Output only. The version of Agones currently installed in the registered Kubernetes
    /// cluster.
    #[prost(string, tag = "1")]
    pub agones_version_installed: ::prost::alloc::string::String,
    /// Output only. The version of Kubernetes that is currently used in the registered
    /// Kubernetes cluster (as detected by the Cloud Game Servers service).
    #[prost(string, tag = "2")]
    pub kubernetes_version_installed: ::prost::alloc::string::String,
    /// Output only. The state for the installed versions of Agones/Kubernetes.
    #[prost(enumeration = "kubernetes_cluster_state::InstallationState", tag = "3")]
    pub installation_state: i32,
    /// Output only. The detailed error message for the installed versions of Agones/Kubernetes.
    #[prost(string, tag = "4")]
    pub version_installed_error_message: ::prost::alloc::string::String,
    /// Output only. The cloud provider type reported by the first node's providerID in the list
    /// of nodes on the Kubernetes endpoint. On Kubernetes platforms that support
    /// zero-node clusters (like GKE-on-GCP), the provider type will be empty.
    #[prost(string, tag = "5")]
    pub provider: ::prost::alloc::string::String,
    /// Output only. The version of Agones that is targeted to be installed in the cluster.
    #[prost(string, tag = "6")]
    pub agones_version_targeted: ::prost::alloc::string::String,
}
/// Nested message and enum types in `KubernetesClusterState`.
pub mod kubernetes_cluster_state {
    /// The state of the installed versions of Agones/Kubernetes. See also
    /// <https://cloud.google.com/game-servers/docs/versions-and-upgrades.>
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum InstallationState {
        /// The default value. This value is used if the state is omitted.
        Unspecified = 0,
        /// The combination of Agones and Kubernetes versions is supported by Google
        /// Cloud Game Servers.
        AgonesKubernetesVersionSupported = 1,
        /// The installed version of Agones is not supported by Google Cloud Game
        /// Servers.
        AgonesVersionUnsupported = 2,
        /// The installed version of Agones is supported by Google Cloud Game
        /// Servers, but the installed version of Kubernetes is not recommended or
        /// supported by the version of Agones.
        AgonesKubernetesVersionUnsupported = 3,
        /// The installed version of Agones is not recognized because the Agones
        /// controller's image name does not have a version string reported as
        /// {major}.{minor}(.{patch}).
        AgonesVersionUnrecognized = 4,
        /// The server version of Kubernetes cluster is not recognized because the
        /// API server didn't return parsable version info on path/version.
        KubernetesVersionUnrecognized = 5,
        /// Failed to read or verify the version of Agones or Kubernetes. See
        /// version_installed_error_message for details.
        VersionVerificationFailed = 6,
        /// Agones is not installed.
        AgonesNotInstalled = 7,
    }
    impl InstallationState {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                InstallationState::Unspecified => "INSTALLATION_STATE_UNSPECIFIED",
                InstallationState::AgonesKubernetesVersionSupported => {
                    "AGONES_KUBERNETES_VERSION_SUPPORTED"
                }
                InstallationState::AgonesVersionUnsupported => {
                    "AGONES_VERSION_UNSUPPORTED"
                }
                InstallationState::AgonesKubernetesVersionUnsupported => {
                    "AGONES_KUBERNETES_VERSION_UNSUPPORTED"
                }
                InstallationState::AgonesVersionUnrecognized => {
                    "AGONES_VERSION_UNRECOGNIZED"
                }
                InstallationState::KubernetesVersionUnrecognized => {
                    "KUBERNETES_VERSION_UNRECOGNIZED"
                }
                InstallationState::VersionVerificationFailed => {
                    "VERSION_VERIFICATION_FAILED"
                }
                InstallationState::AgonesNotInstalled => "AGONES_NOT_INSTALLED",
            }
        }
    }
}
/// A view for GameServerCluster objects.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum GameServerClusterView {
    /// The default / unset value.
    /// The API will default to the BASIC view.
    Unspecified = 0,
    /// Include basic information of a GameServerCluster resource and omit
    /// `cluster_state`. This is the default value (for ListGameServerClusters,
    /// GetGameServerCluster and PreviewCreateGameServerCluster).
    Basic = 1,
    /// Include everything.
    Full = 2,
}
impl GameServerClusterView {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            GameServerClusterView::Unspecified => "GAME_SERVER_CLUSTER_VIEW_UNSPECIFIED",
            GameServerClusterView::Basic => "BASIC",
            GameServerClusterView::Full => "FULL",
        }
    }
}
/// Generated client implementations.
pub mod game_server_clusters_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// The game server cluster maps to Kubernetes clusters running Agones and is
    /// used to manage fleets within clusters.
    #[derive(Debug, Clone)]
    pub struct GameServerClustersServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl GameServerClustersServiceClient<tonic::transport::Channel> {
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
    impl<T> GameServerClustersServiceClient<T>
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
        ) -> GameServerClustersServiceClient<InterceptedService<T, F>>
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
            GameServerClustersServiceClient::new(
                InterceptedService::new(inner, interceptor),
            )
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
        /// Lists game server clusters in a given project and location.
        pub async fn list_game_server_clusters(
            &mut self,
            request: impl tonic::IntoRequest<super::ListGameServerClustersRequest>,
        ) -> Result<
            tonic::Response<super::ListGameServerClustersResponse>,
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
                "/google.cloud.gaming.v1.GameServerClustersService/ListGameServerClusters",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets details of a single game server cluster.
        pub async fn get_game_server_cluster(
            &mut self,
            request: impl tonic::IntoRequest<super::GetGameServerClusterRequest>,
        ) -> Result<tonic::Response<super::GameServerCluster>, tonic::Status> {
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
                "/google.cloud.gaming.v1.GameServerClustersService/GetGameServerCluster",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates a new game server cluster in a given project and location.
        pub async fn create_game_server_cluster(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateGameServerClusterRequest>,
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
                "/google.cloud.gaming.v1.GameServerClustersService/CreateGameServerCluster",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Previews creation of a new game server cluster in a given project and
        /// location.
        pub async fn preview_create_game_server_cluster(
            &mut self,
            request: impl tonic::IntoRequest<
                super::PreviewCreateGameServerClusterRequest,
            >,
        ) -> Result<
            tonic::Response<super::PreviewCreateGameServerClusterResponse>,
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
                "/google.cloud.gaming.v1.GameServerClustersService/PreviewCreateGameServerCluster",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes a single game server cluster.
        pub async fn delete_game_server_cluster(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteGameServerClusterRequest>,
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
                "/google.cloud.gaming.v1.GameServerClustersService/DeleteGameServerCluster",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Previews deletion of a single game server cluster.
        pub async fn preview_delete_game_server_cluster(
            &mut self,
            request: impl tonic::IntoRequest<
                super::PreviewDeleteGameServerClusterRequest,
            >,
        ) -> Result<
            tonic::Response<super::PreviewDeleteGameServerClusterResponse>,
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
                "/google.cloud.gaming.v1.GameServerClustersService/PreviewDeleteGameServerCluster",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Patches a single game server cluster.
        pub async fn update_game_server_cluster(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateGameServerClusterRequest>,
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
                "/google.cloud.gaming.v1.GameServerClustersService/UpdateGameServerCluster",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Previews updating a GameServerCluster.
        pub async fn preview_update_game_server_cluster(
            &mut self,
            request: impl tonic::IntoRequest<
                super::PreviewUpdateGameServerClusterRequest,
            >,
        ) -> Result<
            tonic::Response<super::PreviewUpdateGameServerClusterResponse>,
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
                "/google.cloud.gaming.v1.GameServerClustersService/PreviewUpdateGameServerCluster",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Request message for GameServerConfigsService.ListGameServerConfigs.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListGameServerConfigsRequest {
    /// Required. The parent resource name, in the following form:
    /// `projects/{project}/locations/{location}/gameServerDeployments/{deployment}/configs/*`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. The maximum number of items to return.  If unspecified, server
    /// will pick an appropriate default. Server may return fewer items than
    /// requested. A caller should only rely on response's
    /// \[next_page_token][google.cloud.gaming.v1.ListGameServerConfigsResponse.next_page_token\] to
    /// determine if there are more GameServerConfigs left to be queried.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. The next_page_token value returned from a previous list request, if any.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. The filter to apply to list results.
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Optional. Specifies the ordering of results following syntax at
    /// <https://cloud.google.com/apis/design/design_patterns#sorting_order.>
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
/// Response message for GameServerConfigsService.ListGameServerConfigs.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListGameServerConfigsResponse {
    /// The list of game server configs.
    #[prost(message, repeated, tag = "1")]
    pub game_server_configs: ::prost::alloc::vec::Vec<GameServerConfig>,
    /// Token to retrieve the next page of results, or empty if there are no more
    /// results in the list.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// List of locations that could not be reached.
    #[prost(string, repeated, tag = "4")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Request message for GameServerConfigsService.GetGameServerConfig.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetGameServerConfigRequest {
    /// Required. The name of the game server config to retrieve, in the following form:
    /// `projects/{project}/locations/{location}/gameServerDeployments/{deployment}/configs/{config}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for GameServerConfigsService.CreateGameServerConfig.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateGameServerConfigRequest {
    /// Required. The parent resource name, in the following form:
    /// `projects/{project}/locations/{location}/gameServerDeployments/{deployment}/`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The ID of the game server config resource to be created.
    #[prost(string, tag = "2")]
    pub config_id: ::prost::alloc::string::String,
    /// Required. The game server config resource to be created.
    #[prost(message, optional, tag = "3")]
    pub game_server_config: ::core::option::Option<GameServerConfig>,
}
/// Request message for GameServerConfigsService.DeleteGameServerConfig.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteGameServerConfigRequest {
    /// Required. The name of the game server config to delete, in the following form:
    /// `projects/{project}/locations/{location}/gameServerDeployments/{deployment}/configs/{config}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Autoscaling config for an Agones fleet.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScalingConfig {
    /// Required. The name of the Scaling Config
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. Agones fleet autoscaler spec. Example spec:
    /// <https://agones.dev/site/docs/reference/fleetautoscaler/>
    #[prost(string, tag = "2")]
    pub fleet_autoscaler_spec: ::prost::alloc::string::String,
    /// Labels used to identify the game server clusters to which this Agones
    /// scaling config applies. A game server cluster is subject to this Agones
    /// scaling config if its labels match any of the selector entries.
    #[prost(message, repeated, tag = "4")]
    pub selectors: ::prost::alloc::vec::Vec<LabelSelector>,
    /// The schedules to which this Scaling Config applies.
    #[prost(message, repeated, tag = "5")]
    pub schedules: ::prost::alloc::vec::Vec<Schedule>,
}
/// Fleet configs for Agones.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FleetConfig {
    /// Agones fleet spec. Example spec:
    /// `<https://agones.dev/site/docs/reference/fleet/`.>
    #[prost(string, tag = "1")]
    pub fleet_spec: ::prost::alloc::string::String,
    /// The name of the FleetConfig.
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
}
/// A game server config resource.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GameServerConfig {
    /// The resource name of the game server config, in the following form:
    /// `projects/{project}/locations/{location}/gameServerDeployments/{deployment}/configs/{config}`.
    /// For example,
    /// `projects/my-project/locations/global/gameServerDeployments/my-game/configs/my-config`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. The creation time.
    #[prost(message, optional, tag = "2")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The last-modified time.
    #[prost(message, optional, tag = "3")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The labels associated with this game server config. Each label is a
    /// key-value pair.
    #[prost(map = "string, string", tag = "4")]
    pub labels: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// FleetConfig contains a list of Agones fleet specs. Only one FleetConfig
    /// is allowed.
    #[prost(message, repeated, tag = "5")]
    pub fleet_configs: ::prost::alloc::vec::Vec<FleetConfig>,
    /// The autoscaling settings.
    #[prost(message, repeated, tag = "6")]
    pub scaling_configs: ::prost::alloc::vec::Vec<ScalingConfig>,
    /// The description of the game server config.
    #[prost(string, tag = "7")]
    pub description: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod game_server_configs_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// The game server config configures the game servers in an Agones fleet.
    #[derive(Debug, Clone)]
    pub struct GameServerConfigsServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl GameServerConfigsServiceClient<tonic::transport::Channel> {
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
    impl<T> GameServerConfigsServiceClient<T>
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
        ) -> GameServerConfigsServiceClient<InterceptedService<T, F>>
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
            GameServerConfigsServiceClient::new(
                InterceptedService::new(inner, interceptor),
            )
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
        /// Lists game server configs in a given project, location, and game server
        /// deployment.
        pub async fn list_game_server_configs(
            &mut self,
            request: impl tonic::IntoRequest<super::ListGameServerConfigsRequest>,
        ) -> Result<
            tonic::Response<super::ListGameServerConfigsResponse>,
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
                "/google.cloud.gaming.v1.GameServerConfigsService/ListGameServerConfigs",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets details of a single game server config.
        pub async fn get_game_server_config(
            &mut self,
            request: impl tonic::IntoRequest<super::GetGameServerConfigRequest>,
        ) -> Result<tonic::Response<super::GameServerConfig>, tonic::Status> {
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
                "/google.cloud.gaming.v1.GameServerConfigsService/GetGameServerConfig",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates a new game server config in a given project, location, and game
        /// server deployment. Game server configs are immutable, and are not applied
        /// until referenced in the game server deployment rollout resource.
        pub async fn create_game_server_config(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateGameServerConfigRequest>,
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
                "/google.cloud.gaming.v1.GameServerConfigsService/CreateGameServerConfig",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes a single game server config. The deletion will fail if the game
        /// server config is referenced in a game server deployment rollout.
        pub async fn delete_game_server_config(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteGameServerConfigRequest>,
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
                "/google.cloud.gaming.v1.GameServerConfigsService/DeleteGameServerConfig",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Request message for GameServerDeploymentsService.ListGameServerDeployments.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListGameServerDeploymentsRequest {
    /// Required. The parent resource name, in the following form:
    /// `projects/{project}/locations/{location}`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. The maximum number of items to return.  If unspecified, the server
    /// will pick an appropriate default. The server may return fewer items than
    /// requested. A caller should only rely on response's
    /// \[next_page_token][google.cloud.gaming.v1.ListGameServerDeploymentsResponse.next_page_token\] to
    /// determine if there are more GameServerDeployments left to be queried.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. The next_page_token value returned from a previous List request,
    /// if any.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. The filter to apply to list results.
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Optional. Specifies the ordering of results following syntax at
    /// <https://cloud.google.com/apis/design/design_patterns#sorting_order.>
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
/// Response message for GameServerDeploymentsService.ListGameServerDeployments.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListGameServerDeploymentsResponse {
    /// The list of game server deployments.
    #[prost(message, repeated, tag = "1")]
    pub game_server_deployments: ::prost::alloc::vec::Vec<GameServerDeployment>,
    /// Token to retrieve the next page of results, or empty if there are no more
    /// results in the list.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// List of locations that could not be reached.
    #[prost(string, repeated, tag = "4")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Request message for GameServerDeploymentsService.GetGameServerDeployment.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetGameServerDeploymentRequest {
    /// Required. The name of the game server delpoyment to retrieve, in the following form:
    /// `projects/{project}/locations/{location}/gameServerDeployments/{deployment}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for
/// GameServerDeploymentsService.GetGameServerDeploymentRollout.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetGameServerDeploymentRolloutRequest {
    /// Required. The name of the game server delpoyment to retrieve, in the following form:
    /// `projects/{project}/locations/{location}/gameServerDeployments/{deployment}/rollout`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for GameServerDeploymentsService.CreateGameServerDeployment.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateGameServerDeploymentRequest {
    /// Required. The parent resource name, in the following form:
    /// `projects/{project}/locations/{location}`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The ID of the game server delpoyment resource to be created.
    #[prost(string, tag = "2")]
    pub deployment_id: ::prost::alloc::string::String,
    /// Required. The game server delpoyment resource to be created.
    #[prost(message, optional, tag = "3")]
    pub game_server_deployment: ::core::option::Option<GameServerDeployment>,
}
/// Request message for GameServerDeploymentsService.DeleteGameServerDeployment.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteGameServerDeploymentRequest {
    /// Required. The name of the game server delpoyment to delete, in the following form:
    /// `projects/{project}/locations/{location}/gameServerDeployments/{deployment}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for GameServerDeploymentsService.UpdateGameServerDeployment.
/// Only allows updates for labels.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateGameServerDeploymentRequest {
    /// Required. The game server delpoyment to be updated.
    /// Only fields specified in update_mask are updated.
    #[prost(message, optional, tag = "1")]
    pub game_server_deployment: ::core::option::Option<GameServerDeployment>,
    /// Required. Mask of fields to update. At least one path must be supplied in
    /// this field. For the `FieldMask` definition, see
    /// <https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#fieldmask>
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for
/// GameServerDeploymentsService.UpdateGameServerRolloutDeployment.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateGameServerDeploymentRolloutRequest {
    /// Required. The game server delpoyment rollout to be updated.
    /// Only fields specified in update_mask are updated.
    #[prost(message, optional, tag = "1")]
    pub rollout: ::core::option::Option<GameServerDeploymentRollout>,
    /// Required. Mask of fields to update. At least one path must be supplied in
    /// this field. For the `FieldMask` definition, see
    /// <https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#fieldmask>
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for GameServerDeploymentsService.FetchDeploymentState.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FetchDeploymentStateRequest {
    /// Required. The name of the game server delpoyment, in the following form:
    /// `projects/{project}/locations/{location}/gameServerDeployments/{deployment}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Response message for GameServerDeploymentsService.FetchDeploymentState.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FetchDeploymentStateResponse {
    /// The state of the game server deployment in each game server cluster.
    #[prost(message, repeated, tag = "1")]
    pub cluster_state: ::prost::alloc::vec::Vec<
        fetch_deployment_state_response::DeployedClusterState,
    >,
    /// List of locations that could not be reached.
    #[prost(string, repeated, tag = "2")]
    pub unavailable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Nested message and enum types in `FetchDeploymentStateResponse`.
pub mod fetch_deployment_state_response {
    /// The game server cluster changes made by the game server deployment.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DeployedClusterState {
        /// The name of the cluster.
        #[prost(string, tag = "1")]
        pub cluster: ::prost::alloc::string::String,
        /// The details about the Agones fleets and autoscalers created in the
        /// game server cluster.
        #[prost(message, repeated, tag = "2")]
        pub fleet_details: ::prost::alloc::vec::Vec<super::DeployedFleetDetails>,
    }
}
/// A game server deployment resource.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GameServerDeployment {
    /// The resource name of the game server deployment, in the following form:
    /// `projects/{project}/locations/{location}/gameServerDeployments/{deployment}`.
    /// For example,
    /// `projects/my-project/locations/global/gameServerDeployments/my-deployment`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. The creation time.
    #[prost(message, optional, tag = "2")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The last-modified time.
    #[prost(message, optional, tag = "3")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The labels associated with this game server deployment. Each label is a
    /// key-value pair.
    #[prost(map = "string, string", tag = "4")]
    pub labels: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// ETag of the resource.
    #[prost(string, tag = "7")]
    pub etag: ::prost::alloc::string::String,
    /// Human readable description of the game server delpoyment.
    #[prost(string, tag = "8")]
    pub description: ::prost::alloc::string::String,
}
/// A game server config override.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GameServerConfigOverride {
    /// Selector chooses the game server config targets.
    #[prost(oneof = "game_server_config_override::Selector", tags = "1")]
    pub selector: ::core::option::Option<game_server_config_override::Selector>,
    /// Selects the game server config and how it should be applied.
    #[prost(oneof = "game_server_config_override::Change", tags = "100")]
    pub change: ::core::option::Option<game_server_config_override::Change>,
}
/// Nested message and enum types in `GameServerConfigOverride`.
pub mod game_server_config_override {
    /// Selector chooses the game server config targets.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Selector {
        /// Selector for choosing applicable realms.
        #[prost(message, tag = "1")]
        RealmsSelector(super::RealmSelector),
    }
    /// Selects the game server config and how it should be applied.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Change {
        /// The game server config for this override.
        #[prost(string, tag = "100")]
        ConfigVersion(::prost::alloc::string::String),
    }
}
/// The game server deployment rollout which represents the desired rollout
/// state.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GameServerDeploymentRollout {
    /// The resource name of the game server deployment rollout, in the following
    /// form:
    /// `projects/{project}/locations/{location}/gameServerDeployments/{deployment}/rollout`.
    /// For example,
    /// `projects/my-project/locations/global/gameServerDeployments/my-deployment/rollout`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. The creation time.
    #[prost(message, optional, tag = "2")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The last-modified time.
    #[prost(message, optional, tag = "3")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The default game server config is applied to all realms unless overridden
    /// in the rollout. For example,
    /// `projects/my-project/locations/global/gameServerDeployments/my-game/configs/my-config`.
    #[prost(string, tag = "4")]
    pub default_game_server_config: ::prost::alloc::string::String,
    /// Contains the game server config rollout overrides. Overrides are processed
    /// in the order they are listed. Once a match is found for a realm, the rest
    /// of the list is not processed.
    #[prost(message, repeated, tag = "5")]
    pub game_server_config_overrides: ::prost::alloc::vec::Vec<GameServerConfigOverride>,
    /// ETag of the resource.
    #[prost(string, tag = "6")]
    pub etag: ::prost::alloc::string::String,
}
/// Request message for PreviewGameServerDeploymentRollout.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PreviewGameServerDeploymentRolloutRequest {
    /// Required. The game server deployment rollout to be updated.
    /// Only fields specified in update_mask are updated.
    #[prost(message, optional, tag = "1")]
    pub rollout: ::core::option::Option<GameServerDeploymentRollout>,
    /// Optional. Mask of fields to update. At least one path must be supplied in
    /// this field. For the `FieldMask` definition, see
    /// <https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#fieldmask>
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Optional. The target timestamp to compute the preview. Defaults to the immediately
    /// after the proposed rollout completes.
    #[prost(message, optional, tag = "3")]
    pub preview_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Response message for PreviewGameServerDeploymentRollout.
/// This has details about the Agones fleet and autoscaler to be actuated.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PreviewGameServerDeploymentRolloutResponse {
    /// Locations that could not be reached on this request.
    #[prost(string, repeated, tag = "2")]
    pub unavailable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// ETag of the game server deployment.
    #[prost(string, tag = "3")]
    pub etag: ::prost::alloc::string::String,
    /// The target state.
    #[prost(message, optional, tag = "4")]
    pub target_state: ::core::option::Option<TargetState>,
}
/// Generated client implementations.
pub mod game_server_deployments_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// The game server deployment is used to control the deployment of Agones
    /// fleets.
    #[derive(Debug, Clone)]
    pub struct GameServerDeploymentsServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl GameServerDeploymentsServiceClient<tonic::transport::Channel> {
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
    impl<T> GameServerDeploymentsServiceClient<T>
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
        ) -> GameServerDeploymentsServiceClient<InterceptedService<T, F>>
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
            GameServerDeploymentsServiceClient::new(
                InterceptedService::new(inner, interceptor),
            )
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
        /// Lists game server deployments in a given project and location.
        pub async fn list_game_server_deployments(
            &mut self,
            request: impl tonic::IntoRequest<super::ListGameServerDeploymentsRequest>,
        ) -> Result<
            tonic::Response<super::ListGameServerDeploymentsResponse>,
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
                "/google.cloud.gaming.v1.GameServerDeploymentsService/ListGameServerDeployments",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets details of a single game server deployment.
        pub async fn get_game_server_deployment(
            &mut self,
            request: impl tonic::IntoRequest<super::GetGameServerDeploymentRequest>,
        ) -> Result<tonic::Response<super::GameServerDeployment>, tonic::Status> {
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
                "/google.cloud.gaming.v1.GameServerDeploymentsService/GetGameServerDeployment",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates a new game server deployment in a given project and location.
        pub async fn create_game_server_deployment(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateGameServerDeploymentRequest>,
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
                "/google.cloud.gaming.v1.GameServerDeploymentsService/CreateGameServerDeployment",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes a single game server deployment.
        pub async fn delete_game_server_deployment(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteGameServerDeploymentRequest>,
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
                "/google.cloud.gaming.v1.GameServerDeploymentsService/DeleteGameServerDeployment",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Patches a game server deployment.
        pub async fn update_game_server_deployment(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateGameServerDeploymentRequest>,
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
                "/google.cloud.gaming.v1.GameServerDeploymentsService/UpdateGameServerDeployment",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets details a single game server deployment rollout.
        pub async fn get_game_server_deployment_rollout(
            &mut self,
            request: impl tonic::IntoRequest<
                super::GetGameServerDeploymentRolloutRequest,
            >,
        ) -> Result<tonic::Response<super::GameServerDeploymentRollout>, tonic::Status> {
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
                "/google.cloud.gaming.v1.GameServerDeploymentsService/GetGameServerDeploymentRollout",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Patches a single game server deployment rollout.
        /// The method will not return an error if the update does not affect any
        /// existing realms. For example - if the default_game_server_config is changed
        /// but all existing realms use the override, that is valid. Similarly, if a
        /// non existing realm is explicitly called out in game_server_config_overrides
        /// field, that will also not result in an error.
        pub async fn update_game_server_deployment_rollout(
            &mut self,
            request: impl tonic::IntoRequest<
                super::UpdateGameServerDeploymentRolloutRequest,
            >,
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
                "/google.cloud.gaming.v1.GameServerDeploymentsService/UpdateGameServerDeploymentRollout",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Previews the game server deployment rollout. This API does not mutate the
        /// rollout resource.
        pub async fn preview_game_server_deployment_rollout(
            &mut self,
            request: impl tonic::IntoRequest<
                super::PreviewGameServerDeploymentRolloutRequest,
            >,
        ) -> Result<
            tonic::Response<super::PreviewGameServerDeploymentRolloutResponse>,
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
                "/google.cloud.gaming.v1.GameServerDeploymentsService/PreviewGameServerDeploymentRollout",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Retrieves information about the current state of the game server
        /// deployment. Gathers all the Agones fleets and Agones autoscalers,
        /// including fleets running an older version of the game server deployment.
        pub async fn fetch_deployment_state(
            &mut self,
            request: impl tonic::IntoRequest<super::FetchDeploymentStateRequest>,
        ) -> Result<
            tonic::Response<super::FetchDeploymentStateResponse>,
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
                "/google.cloud.gaming.v1.GameServerDeploymentsService/FetchDeploymentState",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Request message for RealmsService.ListRealms.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListRealmsRequest {
    /// Required. The parent resource name, in the following form:
    /// `projects/{project}/locations/{location}`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. The maximum number of items to return.  If unspecified, server
    /// will pick an appropriate default. Server may return fewer items than
    /// requested. A caller should only rely on response's
    /// \[next_page_token][google.cloud.gaming.v1.ListRealmsResponse.next_page_token\] to
    /// determine if there are more realms left to be queried.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. The next_page_token value returned from a previous List request,
    /// if any.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. The filter to apply to list results.
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Optional. Specifies the ordering of results following syntax at
    /// <https://cloud.google.com/apis/design/design_patterns#sorting_order.>
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
/// Response message for RealmsService.ListRealms.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListRealmsResponse {
    /// The list of realms.
    #[prost(message, repeated, tag = "1")]
    pub realms: ::prost::alloc::vec::Vec<Realm>,
    /// Token to retrieve the next page of results, or empty if there are no more
    /// results in the list.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// List of locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Request message for RealmsService.GetRealm.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRealmRequest {
    /// Required. The name of the realm to retrieve, in the following form:
    /// `projects/{project}/locations/{location}/realms/{realm}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for RealmsService.CreateRealm.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateRealmRequest {
    /// Required. The parent resource name, in the following form:
    /// `projects/{project}/locations/{location}`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The ID of the realm resource to be created.
    #[prost(string, tag = "2")]
    pub realm_id: ::prost::alloc::string::String,
    /// Required. The realm resource to be created.
    #[prost(message, optional, tag = "3")]
    pub realm: ::core::option::Option<Realm>,
}
/// Request message for RealmsService.DeleteRealm.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteRealmRequest {
    /// Required. The name of the realm to delete, in the following form:
    /// `projects/{project}/locations/{location}/realms/{realm}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for RealmsService.UpdateRealm.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateRealmRequest {
    /// Required. The realm to be updated.
    /// Only fields specified in update_mask are updated.
    #[prost(message, optional, tag = "1")]
    pub realm: ::core::option::Option<Realm>,
    /// Required. The update mask applies to the resource. For the `FieldMask`
    /// definition, see
    /// <https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#fieldmask>
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for RealmsService.PreviewRealmUpdate.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PreviewRealmUpdateRequest {
    /// Required. The realm to be updated.
    /// Only fields specified in update_mask are updated.
    #[prost(message, optional, tag = "1")]
    pub realm: ::core::option::Option<Realm>,
    /// Required. The update mask applies to the resource. For the `FieldMask`
    /// definition, see
    /// <https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#fieldmask>
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Optional. The target timestamp to compute the preview.
    #[prost(message, optional, tag = "3")]
    pub preview_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Response message for RealmsService.PreviewRealmUpdate.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PreviewRealmUpdateResponse {
    /// ETag of the realm.
    #[prost(string, tag = "2")]
    pub etag: ::prost::alloc::string::String,
    /// The target state.
    #[prost(message, optional, tag = "3")]
    pub target_state: ::core::option::Option<TargetState>,
}
/// A realm resource.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Realm {
    /// The resource name of the realm, in the following form:
    /// `projects/{project}/locations/{location}/realms/{realm}`. For
    /// example, `projects/my-project/locations/{location}/realms/my-realm`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. The creation time.
    #[prost(message, optional, tag = "2")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. The last-modified time.
    #[prost(message, optional, tag = "3")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The labels associated with this realm. Each label is a key-value pair.
    #[prost(map = "string, string", tag = "4")]
    pub labels: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
    /// Required. Time zone where all policies targeting this realm are evaluated. The value
    /// of this field must be from the IANA time zone database:
    /// <https://www.iana.org/time-zones.>
    #[prost(string, tag = "6")]
    pub time_zone: ::prost::alloc::string::String,
    /// ETag of the resource.
    #[prost(string, tag = "7")]
    pub etag: ::prost::alloc::string::String,
    /// Human readable description of the realm.
    #[prost(string, tag = "8")]
    pub description: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod realms_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// A realm is a grouping of game server clusters that are considered
    /// interchangeable.
    #[derive(Debug, Clone)]
    pub struct RealmsServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl RealmsServiceClient<tonic::transport::Channel> {
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
    impl<T> RealmsServiceClient<T>
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
        ) -> RealmsServiceClient<InterceptedService<T, F>>
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
            RealmsServiceClient::new(InterceptedService::new(inner, interceptor))
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
        /// Lists realms in a given project and location.
        pub async fn list_realms(
            &mut self,
            request: impl tonic::IntoRequest<super::ListRealmsRequest>,
        ) -> Result<tonic::Response<super::ListRealmsResponse>, tonic::Status> {
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
                "/google.cloud.gaming.v1.RealmsService/ListRealms",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets details of a single realm.
        pub async fn get_realm(
            &mut self,
            request: impl tonic::IntoRequest<super::GetRealmRequest>,
        ) -> Result<tonic::Response<super::Realm>, tonic::Status> {
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
                "/google.cloud.gaming.v1.RealmsService/GetRealm",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Creates a new realm in a given project and location.
        pub async fn create_realm(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateRealmRequest>,
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
                "/google.cloud.gaming.v1.RealmsService/CreateRealm",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes a single realm.
        pub async fn delete_realm(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteRealmRequest>,
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
                "/google.cloud.gaming.v1.RealmsService/DeleteRealm",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Patches a single realm.
        pub async fn update_realm(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateRealmRequest>,
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
                "/google.cloud.gaming.v1.RealmsService/UpdateRealm",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Previews patches to a single realm.
        pub async fn preview_realm_update(
            &mut self,
            request: impl tonic::IntoRequest<super::PreviewRealmUpdateRequest>,
        ) -> Result<tonic::Response<super::PreviewRealmUpdateResponse>, tonic::Status> {
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
                "/google.cloud.gaming.v1.RealmsService/PreviewRealmUpdate",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
