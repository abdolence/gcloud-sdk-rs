/// The short version of cluster configuration for Cloud logging.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClusterSize {
    /// The number of primary workers in the cluster.
    #[prost(int32, tag="1")]
    pub primary_worker_count: i32,
    /// The number of secondary workers in the cluster.
    #[prost(int32, tag="2")]
    pub secondary_worker_count: i32,
}
/// The main proto that will be converted to JSON format and then written to
/// Logging.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AutoscalerLog {
    /// The current Autoscaler status.
    #[prost(message, optional, tag="1")]
    pub status: ::core::option::Option<AutoscalerStatus>,
    /// Optional. The autoscaling recommendation including its inputs, outputs,
    /// scaling decision, and detailed explanation.
    #[prost(message, optional, tag="2")]
    pub recommendation: ::core::option::Option<AutoscalerRecommendation>,
}
/// The Autoscaler's status, including its state and other details.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AutoscalerStatus {
    /// The high-level Autoscaler state.
    #[prost(enumeration="AutoscalerState", tag="1")]
    pub state: i32,
    /// The detailed description of Autoscaler status.
    #[prost(string, tag="2")]
    pub details: ::prost::alloc::string::String,
    /// The cluster update operation ID.
    #[prost(string, tag="3")]
    pub update_cluster_operation_id: ::prost::alloc::string::String,
    /// Error message from an Autoscaler exception, if any.
    #[prost(string, tag="4")]
    pub error: ::prost::alloc::string::String,
}
/// The inputs, outputs, and detailed explanation of the Autoscaling
/// recommendation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AutoscalerRecommendation {
    /// The autoscaling algorithm inputs.
    #[prost(message, optional, tag="1")]
    pub inputs: ::core::option::Option<autoscaler_recommendation::Inputs>,
    /// The algorithm outputs for the recommended cluster size.
    #[prost(message, optional, tag="2")]
    pub outputs: ::core::option::Option<autoscaler_recommendation::Outputs>,
}
/// Nested message and enum types in `AutoscalerRecommendation`.
pub mod autoscaler_recommendation {
    /// The input values for the Autoscaling recommendation alogirthm.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Inputs {
        /// The metrics collected by the Dataproc agent running on the cluster.
        /// For example, {"avg-yarn-pending-memory": "1040 MB"}
        #[prost(map="string, string", tag="1")]
        pub cluster_metrics: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
        /// The cluster configuration before updating the cluster.
        #[prost(message, optional, tag="2")]
        pub current_cluster_size: ::core::option::Option<super::ClusterSize>,
        /// The minimum worker counts for each instance group.
        #[prost(message, optional, tag="3")]
        pub min_worker_counts: ::core::option::Option<super::ClusterSize>,
        /// The maximum worker counts for each instance group.
        #[prost(message, optional, tag="4")]
        pub max_worker_counts: ::core::option::Option<super::ClusterSize>,
    }
    /// Autoscaler recommendations.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Outputs {
        /// The high-level autoscaling decision, such as SCALE_UP, SCALE_DOWN,
        /// NO_OP.
        #[prost(enumeration="super::ScalingDecisionType", tag="1")]
        pub decision: i32,
        /// The recommended cluster size.
        #[prost(message, optional, tag="2")]
        pub recommended_cluster_size: ::core::option::Option<super::ClusterSize>,
        /// The graceful decommission timeout for downscaling operations.
        #[prost(message, optional, tag="3")]
        pub graceful_decommission_timeout: ::core::option::Option<::prost_types::Duration>,
        /// Reasons why the Autoscaler didn't add or remove more workers.
        #[prost(enumeration="super::ConstrainingFactor", repeated, tag="4")]
        pub constraints_reached: ::prost::alloc::vec::Vec<i32>,
        /// Less significant recommendations that are not included in the
        /// `AutoscalerStatus.details` message.
        #[prost(string, repeated, tag="5")]
        pub additional_recommendation_details: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        /// A unique id for this recommendation that should be included when opening
        /// a support ticket.
        #[prost(string, tag="6")]
        pub recommendation_id: ::prost::alloc::string::String,
    }
}
/// The Autoscaler state.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AutoscalerState {
    Unspecified = 0,
    /// The Autoscaler is sleeping and waiting for the next update.
    Cooldown = 1,
    /// The Autoscaler is in the process of calculating its recommendation on
    /// whether to scale the cluster, and if so, how to autoscale.
    Recommending = 6,
    /// The Autoscaler is scaling the cluster.
    Scaling = 2,
    /// The Autoscaler has stopped.
    Stopped = 3,
    /// The Autoscaler has failed.
    Failed = 4,
    /// The Autoscaler is initializing.
    Initializing = 5,
}
impl AutoscalerState {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            AutoscalerState::Unspecified => "AUTOSCALER_STATE_UNSPECIFIED",
            AutoscalerState::Cooldown => "COOLDOWN",
            AutoscalerState::Recommending => "RECOMMENDING",
            AutoscalerState::Scaling => "SCALING",
            AutoscalerState::Stopped => "STOPPED",
            AutoscalerState::Failed => "FAILED",
            AutoscalerState::Initializing => "INITIALIZING",
        }
    }
}
/// The Autoscaling decision type.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ScalingDecisionType {
    Unspecified = 0,
    /// Increase the number of primary and/or secondary workers.
    ScaleUp = 1,
    /// Decrease the number of primary and/or secondary workers.
    ScaleDown = 2,
    /// Not changing the number of primary or secondary workers.
    NoScale = 3,
    /// Scale the primary and secondary worker groups in different directions.
    Mixed = 4,
}
impl ScalingDecisionType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ScalingDecisionType::Unspecified => "SCALING_DECISION_TYPE_UNSPECIFIED",
            ScalingDecisionType::ScaleUp => "SCALE_UP",
            ScalingDecisionType::ScaleDown => "SCALE_DOWN",
            ScalingDecisionType::NoScale => "NO_SCALE",
            ScalingDecisionType::Mixed => "MIXED",
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ConstrainingFactor {
    Unspecified = 0,
    /// The project does not have sufficient regional, global, and or preemptible
    /// quota to allocate a new VM.
    ScalingCappedDueToLackOfQuota = 1,
    /// All worker groups have reached maximum size. This message will not be
    /// issued if one group reached maximum size, but workers were able to be
    /// allocated to another group.
    ReachedMaximumClusterSize = 2,
    /// All worker groups have reached minimum size. This message will not be
    /// issued if workers were able to be removed from another group that had not
    /// reached minimum size.
    ReachedMinimumClusterSize = 3,
}
impl ConstrainingFactor {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ConstrainingFactor::Unspecified => "CONSTRAINING_FACTOR_UNSPECIFIED",
            ConstrainingFactor::ScalingCappedDueToLackOfQuota => "SCALING_CAPPED_DUE_TO_LACK_OF_QUOTA",
            ConstrainingFactor::ReachedMaximumClusterSize => "REACHED_MAXIMUM_CLUSTER_SIZE",
            ConstrainingFactor::ReachedMinimumClusterSize => "REACHED_MINIMUM_CLUSTER_SIZE",
        }
    }
}
