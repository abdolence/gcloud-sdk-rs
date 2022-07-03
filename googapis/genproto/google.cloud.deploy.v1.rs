/// A `DeliveryPipeline` resource in the Google Cloud Deploy API.
///
/// A `DeliveryPipeline` defines a pipeline through which a Skaffold
/// configuration can progress.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeliveryPipeline {
    /// Optional. Name of the `DeliveryPipeline`. Format is projects/{project}/
    /// locations/{location}/deliveryPipelines/\[a-z][a-z0-9\-\]{0,62}.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. Unique identifier of the `DeliveryPipeline`.
    #[prost(string, tag = "2")]
    pub uid: ::prost::alloc::string::String,
    /// Description of the `DeliveryPipeline`. Max length is 255 characters.
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    /// User annotations. These attributes can only be set and used by the
    /// user, and not by Google Cloud Deploy. See
    /// <https://google.aip.dev/128#annotations> for more details such as format and
    /// size limitations.
    #[prost(map = "string, string", tag = "4")]
    pub annotations:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Labels are attributes that can be set and used by both the
    /// user and by Google Cloud Deploy. Labels must meet the following
    /// constraints:
    ///
    /// * Keys and values can contain only lowercase letters, numeric characters,
    /// underscores, and dashes.
    /// * All characters must use UTF-8 encoding, and international characters are
    /// allowed.
    /// * Keys must start with a lowercase letter or international character.
    /// * Each resource is limited to a maximum of 64 labels.
    ///
    /// Both keys and values are additionally constrained to be <= 128 bytes.
    #[prost(map = "string, string", tag = "5")]
    pub labels:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Output only. Time at which the pipeline was created.
    #[prost(message, optional, tag = "6")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Most recent time at which the pipeline was updated.
    #[prost(message, optional, tag = "7")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Information around the state of the Delivery Pipeline.
    #[prost(message, optional, tag = "11")]
    pub condition: ::core::option::Option<PipelineCondition>,
    /// This checksum is computed by the server based on the value of other
    /// fields, and may be sent on update and delete requests to ensure the
    /// client has an up-to-date value before proceeding.
    #[prost(string, tag = "10")]
    pub etag: ::prost::alloc::string::String,
    /// The ordering configuration of the `DeliveryPipeline`.
    #[prost(oneof = "delivery_pipeline::Pipeline", tags = "8")]
    pub pipeline: ::core::option::Option<delivery_pipeline::Pipeline>,
}
/// Nested message and enum types in `DeliveryPipeline`.
pub mod delivery_pipeline {
    /// The ordering configuration of the `DeliveryPipeline`.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Pipeline {
        /// SerialPipeline defines a sequential set of stages for a
        /// `DeliveryPipeline`.
        #[prost(message, tag = "8")]
        SerialPipeline(super::SerialPipeline),
    }
}
/// SerialPipeline defines a sequential set of stages for a `DeliveryPipeline`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SerialPipeline {
    /// Each stage specifies configuration for a `Target`. The ordering
    /// of this list defines the promotion flow.
    #[prost(message, repeated, tag = "1")]
    pub stages: ::prost::alloc::vec::Vec<Stage>,
}
/// Stage specifies a location to which to deploy.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Stage {
    /// The target_id to which this stage points. This field refers exclusively to
    /// the last segment of a target name. For example, this field would just be
    /// `my-target` (rather than
    /// `projects/project/locations/location/targets/my-target`). The location of
    /// the `Target` is inferred to be the same as the location of the
    /// `DeliveryPipeline` that contains this `Stage`.
    #[prost(string, tag = "1")]
    pub target_id: ::prost::alloc::string::String,
    /// Skaffold profiles to use when rendering the manifest for this stage's
    /// `Target`.
    #[prost(string, repeated, tag = "2")]
    pub profiles: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// PipelineReadyCondition contains information around the status of the
/// Pipeline.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PipelineReadyCondition {
    /// True if the Pipeline is in a valid state. Otherwise at least one condition
    /// in `PipelineCondition` is in an invalid state. Iterate over those
    /// conditions and see which condition(s) has status = false to find out what
    /// is wrong with the Pipeline.
    #[prost(bool, tag = "3")]
    pub status: bool,
    /// Last time the condition was updated.
    #[prost(message, optional, tag = "4")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// TargetsPresentCondition contains information on any Targets defined in
/// the Delivery Pipeline that do not actually exist.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TargetsPresentCondition {
    /// True if there aren't any missing Targets.
    #[prost(bool, tag = "1")]
    pub status: bool,
    /// The list of Target names that are missing. For example,
    /// projects/{project_id}/locations/{location_name}/targets/{target_name}.
    #[prost(string, repeated, tag = "2")]
    pub missing_targets: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Last time the condition was updated.
    #[prost(message, optional, tag = "4")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// PipelineCondition contains all conditions relevant to a Delivery Pipeline.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PipelineCondition {
    /// Details around the Pipeline's overall status.
    #[prost(message, optional, tag = "1")]
    pub pipeline_ready_condition: ::core::option::Option<PipelineReadyCondition>,
    /// Detalis around targets enumerated in the pipeline.
    #[prost(message, optional, tag = "3")]
    pub targets_present_condition: ::core::option::Option<TargetsPresentCondition>,
}
/// The request object for `ListDeliveryPipelines`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDeliveryPipelinesRequest {
    /// Required. The parent, which owns this collection of pipelines. Format must be
    /// projects/{project_id}/locations/{location_name}.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of pipelines to return. The service may return
    /// fewer than this value. If unspecified, at most 50 pipelines will
    /// be returned. The maximum value is 1000; values above 1000 will be set
    /// to 1000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A page token, received from a previous `ListDeliveryPipelines` call.
    /// Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other provided parameters match
    /// the call that provided the page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Filter pipelines to be returned. See <https://google.aip.dev/160> for more
    /// details.
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Field to sort by. See <https://google.aip.dev/132#ordering> for more details.
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
/// The response object from `ListDeliveryPipelines`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDeliveryPipelinesResponse {
    /// The `DeliveryPipeline` objects.
    #[prost(message, repeated, tag = "1")]
    pub delivery_pipelines: ::prost::alloc::vec::Vec<DeliveryPipeline>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// The request object for `GetDeliveryPipeline`
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDeliveryPipelineRequest {
    /// Required. Name of the `DeliveryPipeline`. Format must be
    /// projects/{project_id}/locations/{location_name}/deliveryPipelines/{pipeline_name}.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// The request object for `CreateDeliveryPipeline`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateDeliveryPipelineRequest {
    /// Required. The parent collection in which the `DeliveryPipeline` should be created.
    /// Format should be projects/{project_id}/locations/{location_name}.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. ID of the `DeliveryPipeline`.
    #[prost(string, tag = "2")]
    pub delivery_pipeline_id: ::prost::alloc::string::String,
    /// Required. The `DeliveryPipeline` to create.
    #[prost(message, optional, tag = "3")]
    pub delivery_pipeline: ::core::option::Option<DeliveryPipeline>,
    /// Optional. A request ID to identify requests. Specify a unique request ID
    /// so that if you must retry your request, the server will know to ignore
    /// the request if it has already been completed. The server will guarantee
    /// that for at least 60 minutes since the first request.
    ///
    /// For example, consider a situation where you make an initial request and the
    /// request times out. If you make the request again with the same request ID,
    /// the server can check if original operation with the same request ID was
    /// received, and if so, will ignore the second request. This prevents clients
    /// from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "4")]
    pub request_id: ::prost::alloc::string::String,
    /// Optional. If set to true, the request is validated and the user is provided with
    /// an expected result, but no actual change is made.
    #[prost(bool, tag = "5")]
    pub validate_only: bool,
}
/// The request object for `UpdateDeliveryPipeline`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateDeliveryPipelineRequest {
    /// Required. Field mask is used to specify the fields to be overwritten in the
    /// `DeliveryPipeline` resource by the update.
    /// The fields specified in the update_mask are relative to the resource, not
    /// the full request. A field will be overwritten if it is in the mask. If the
    /// user does not provide a mask then all fields will be overwritten.
    #[prost(message, optional, tag = "1")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Required. The `DeliveryPipeline` to update.
    #[prost(message, optional, tag = "2")]
    pub delivery_pipeline: ::core::option::Option<DeliveryPipeline>,
    /// Optional. A request ID to identify requests. Specify a unique request ID
    /// so that if you must retry your request, the server will know to ignore
    /// the request if it has already been completed. The server will guarantee
    /// that for at least 60 minutes since the first request.
    ///
    /// For example, consider a situation where you make an initial request and the
    /// request times out. If you make the request again with the same request ID,
    /// the server can check if original operation with the same request ID was
    /// received, and if so, will ignore the second request. This prevents clients
    /// from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "3")]
    pub request_id: ::prost::alloc::string::String,
    /// Optional. If set to true, updating a `DeliveryPipeline` that does not exist will
    /// result in the creation of a new `DeliveryPipeline`.
    #[prost(bool, tag = "4")]
    pub allow_missing: bool,
    /// Optional. If set to true, the request is validated and the user is provided with
    /// an expected result, but no actual change is made.
    #[prost(bool, tag = "5")]
    pub validate_only: bool,
}
/// The request object for `DeleteDeliveryPipeline`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteDeliveryPipelineRequest {
    /// Required. The name of the `DeliveryPipeline` to delete. Format should be
    /// projects/{project_id}/locations/{location_name}/deliveryPipelines/{pipeline_name}.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. A request ID to identify requests. Specify a unique request ID
    /// so that if you must retry your request, the server will know to ignore
    /// the request if it has already been completed. The server will guarantee
    /// that for at least 60 minutes after the first request.
    ///
    /// For example, consider a situation where you make an initial request and the
    /// request times out. If you make the request again with the same request ID,
    /// the server can check if original operation with the same request ID was
    /// received, and if so, will ignore the second request. This prevents clients
    /// from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "2")]
    pub request_id: ::prost::alloc::string::String,
    /// Optional. If set to true, then deleting an already deleted or non-existing
    /// `DeliveryPipeline` will succeed.
    #[prost(bool, tag = "3")]
    pub allow_missing: bool,
    /// Optional. If set, validate the request and preview the review, but do not actually
    /// post it.
    #[prost(bool, tag = "4")]
    pub validate_only: bool,
    /// Optional. If set to true, all child resources under this pipeline will also be
    /// deleted. Otherwise, the request will only work if the pipeline has
    /// no child resources.
    #[prost(bool, tag = "6")]
    pub force: bool,
    /// Optional. This checksum is computed by the server based on the value of other
    /// fields, and may be sent on update and delete requests to ensure the
    /// client has an up-to-date value before proceeding.
    #[prost(string, tag = "5")]
    pub etag: ::prost::alloc::string::String,
}
/// A `Target` resource in the Google Cloud Deploy API.
///
/// A `Target` defines a location to which a Skaffold configuration
/// can be deployed.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Target {
    /// Optional. Name of the `Target`. Format is
    /// projects/{project}/locations/{location}/targets/\[a-z][a-z0-9\-\]{0,62}.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. Resource id of the `Target`.
    #[prost(string, tag = "2")]
    pub target_id: ::prost::alloc::string::String,
    /// Output only. Unique identifier of the `Target`.
    #[prost(string, tag = "3")]
    pub uid: ::prost::alloc::string::String,
    /// Optional. Description of the `Target`. Max length is 255 characters.
    #[prost(string, tag = "4")]
    pub description: ::prost::alloc::string::String,
    /// Optional. User annotations. These attributes can only be set and used by the
    /// user, and not by Google Cloud Deploy. See
    /// <https://google.aip.dev/128#annotations> for more details such as format and
    /// size limitations.
    #[prost(map = "string, string", tag = "5")]
    pub annotations:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Optional. Labels are attributes that can be set and used by both the
    /// user and by Google Cloud Deploy. Labels must meet the following
    /// constraints:
    ///
    /// * Keys and values can contain only lowercase letters, numeric characters,
    /// underscores, and dashes.
    /// * All characters must use UTF-8 encoding, and international characters are
    /// allowed.
    /// * Keys must start with a lowercase letter or international character.
    /// * Each resource is limited to a maximum of 64 labels.
    ///
    /// Both keys and values are additionally constrained to be <= 128 bytes.
    #[prost(map = "string, string", tag = "6")]
    pub labels:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Optional. Whether or not the `Target` requires approval.
    #[prost(bool, tag = "13")]
    pub require_approval: bool,
    /// Output only. Time at which the `Target` was created.
    #[prost(message, optional, tag = "8")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Most recent time at which the `Target` was updated.
    #[prost(message, optional, tag = "9")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Optional. This checksum is computed by the server based on the value of other
    /// fields, and may be sent on update and delete requests to ensure the
    /// client has an up-to-date value before proceeding.
    #[prost(string, tag = "12")]
    pub etag: ::prost::alloc::string::String,
    /// Configurations for all execution that relates to this `Target`.
    /// Each `ExecutionEnvironmentUsage` value may only be used in a single
    /// configuration; using the same value multiple times is an error.
    /// When one or more configurations are specified, they must include the
    /// `RENDER` and `DEPLOY` `ExecutionEnvironmentUsage` values.
    /// When no configurations are specified, execution will use the default
    /// specified in `DefaultPool`.
    #[prost(message, repeated, tag = "16")]
    pub execution_configs: ::prost::alloc::vec::Vec<ExecutionConfig>,
    /// Destination to which the Skaffold configuration is applied during a
    /// rollout.
    #[prost(oneof = "target::DeploymentTarget", tags = "15, 17")]
    pub deployment_target: ::core::option::Option<target::DeploymentTarget>,
}
/// Nested message and enum types in `Target`.
pub mod target {
    /// Destination to which the Skaffold configuration is applied during a
    /// rollout.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum DeploymentTarget {
        /// Information specifying a GKE Cluster.
        #[prost(message, tag = "15")]
        Gke(super::GkeCluster),
        /// Information specifying an Anthos Cluster.
        #[prost(message, tag = "17")]
        AnthosCluster(super::AnthosCluster),
    }
}
/// Configuration of the environment to use when calling Skaffold.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecutionConfig {
    /// Required. Usages when this configuration should be applied.
    #[prost(
        enumeration = "execution_config::ExecutionEnvironmentUsage",
        repeated,
        packed = "false",
        tag = "1"
    )]
    pub usages: ::prost::alloc::vec::Vec<i32>,
    /// Optional. The resource name of the `WorkerPool`, with the format
    /// `projects/{project}/locations/{location}/workerPools/{worker_pool}`.
    /// If this optional field is unspecified, the default Cloud Build pool will be
    /// used.
    #[prost(string, tag = "4")]
    pub worker_pool: ::prost::alloc::string::String,
    /// Optional. Google service account to use for execution. If unspecified,
    /// the project execution service account
    /// (<PROJECT_NUMBER>-compute@developer.gserviceaccount.com) is used.
    #[prost(string, tag = "5")]
    pub service_account: ::prost::alloc::string::String,
    /// Optional. Cloud Storage location in which to store execution outputs. This can
    /// either be a bucket ("gs://my-bucket") or a path within a bucket
    /// ("gs://my-bucket/my-dir").
    /// If unspecified, a default bucket located in the same region will be used.
    #[prost(string, tag = "6")]
    pub artifact_storage: ::prost::alloc::string::String,
    /// Details of the environment.
    #[prost(oneof = "execution_config::ExecutionEnvironment", tags = "2, 3")]
    pub execution_environment: ::core::option::Option<execution_config::ExecutionEnvironment>,
}
/// Nested message and enum types in `ExecutionConfig`.
pub mod execution_config {
    /// Possible usages of this configuration.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ExecutionEnvironmentUsage {
        /// Default value. This value is unused.
        Unspecified = 0,
        /// Use for rendering.
        Render = 1,
        /// Use for deploying and deployment hooks.
        Deploy = 2,
    }
    /// Details of the environment.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ExecutionEnvironment {
        /// Optional. Use default Cloud Build pool.
        #[prost(message, tag = "2")]
        DefaultPool(super::DefaultPool),
        /// Optional. Use private Cloud Build pool.
        #[prost(message, tag = "3")]
        PrivatePool(super::PrivatePool),
    }
}
/// Execution using the default Cloud Build pool.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DefaultPool {
    /// Optional. Google service account to use for execution. If unspecified,
    /// the project execution service account
    /// (<PROJECT_NUMBER>-compute@developer.gserviceaccount.com) will be used.
    #[prost(string, tag = "1")]
    pub service_account: ::prost::alloc::string::String,
    /// Optional. Cloud Storage location where execution outputs should be stored. This can
    /// either be a bucket ("gs://my-bucket") or a path within a bucket
    /// ("gs://my-bucket/my-dir").
    /// If unspecified, a default bucket located in the same region will be used.
    #[prost(string, tag = "2")]
    pub artifact_storage: ::prost::alloc::string::String,
}
/// Execution using a private Cloud Build pool.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrivatePool {
    /// Required. Resource name of the Cloud Build worker pool to use. The format is
    /// `projects/{project}/locations/{location}/workerPools/{pool}`.
    #[prost(string, tag = "1")]
    pub worker_pool: ::prost::alloc::string::String,
    /// Optional. Google service account to use for execution. If unspecified,
    /// the project execution service account
    /// (<PROJECT_NUMBER>-compute@developer.gserviceaccount.com) will be used.
    #[prost(string, tag = "2")]
    pub service_account: ::prost::alloc::string::String,
    /// Optional. Cloud Storage location where execution outputs should be stored. This can
    /// either be a bucket ("gs://my-bucket") or a path within a bucket
    /// ("gs://my-bucket/my-dir").
    /// If unspecified, a default bucket located in the same region will be used.
    #[prost(string, tag = "3")]
    pub artifact_storage: ::prost::alloc::string::String,
}
/// Information specifying a GKE Cluster.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GkeCluster {
    /// Information specifying a GKE Cluster. Format is
    /// `projects/{project_id}/locations/{location_id}/clusters/{cluster_id}.
    #[prost(string, tag = "1")]
    pub cluster: ::prost::alloc::string::String,
    /// Optional. If true, `cluster` is accessed using the private IP address of the control
    /// plane endpoint. Otherwise, the default IP address of the control plane
    /// endpoint is used. The default IP address is the private IP address for
    /// clusters with private control-plane endpoints and the public IP address
    /// otherwise.
    ///
    /// Only specify this option when `cluster` is a [private GKE
    /// cluster](<https://cloud.google.com/kubernetes-engine/docs/concepts/private-cluster-concept>).
    #[prost(bool, tag = "2")]
    pub internal_ip: bool,
}
/// Information specifying an Anthos Cluster.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnthosCluster {
    /// Membership of the GKE Hub-registered cluster to which to apply the Skaffold
    /// configuration. Format is
    /// `projects/{project}/locations/{location}/memberships/{membership_name}`.
    #[prost(string, tag = "1")]
    pub membership: ::prost::alloc::string::String,
}
/// The request object for `ListTargets`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTargetsRequest {
    /// Required. The parent, which owns this collection of targets. Format must be
    /// projects/{project_id}/locations/{location_name}.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. The maximum number of `Target` objects to return. The service may return
    /// fewer than this value. If unspecified, at most 50 `Target` objects will be
    /// returned. The maximum value is 1000; values above 1000 will be set to 1000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. A page token, received from a previous `ListTargets` call.
    /// Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other provided parameters match
    /// the call that provided the page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. Filter targets to be returned. See <https://google.aip.dev/160> for more
    /// details.
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Optional. Field to sort by. See <https://google.aip.dev/132#ordering> for more details.
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
/// The response object from `ListTargets`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListTargetsResponse {
    /// The `Target` objects.
    #[prost(message, repeated, tag = "1")]
    pub targets: ::prost::alloc::vec::Vec<Target>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// The request object for `GetTarget`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTargetRequest {
    /// Required. Name of the `Target`. Format must be
    /// projects/{project_id}/locations/{location_name}/targets/{target_name}.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// The request object for `CreateTarget`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateTargetRequest {
    /// Required. The parent collection in which the `Target` should be created.
    /// Format should be
    /// projects/{project_id}/locations/{location_name}.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. ID of the `Target`.
    #[prost(string, tag = "2")]
    pub target_id: ::prost::alloc::string::String,
    /// Required. The `Target` to create.
    #[prost(message, optional, tag = "3")]
    pub target: ::core::option::Option<Target>,
    /// Optional. A request ID to identify requests. Specify a unique request ID
    /// so that if you must retry your request, the server will know to ignore
    /// the request if it has already been completed. The server will guarantee
    /// that for at least 60 minutes since the first request.
    ///
    /// For example, consider a situation where you make an initial request and the
    /// request times out. If you make the request again with the same request ID,
    /// the server can check if original operation with the same request ID was
    /// received, and if so, will ignore the second request. This prevents clients
    /// from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "4")]
    pub request_id: ::prost::alloc::string::String,
    /// Optional. If set to true, the request is validated and the user is provided with
    /// an expected result, but no actual change is made.
    #[prost(bool, tag = "5")]
    pub validate_only: bool,
}
/// The request object for `UpdateTarget`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateTargetRequest {
    /// Required. Field mask is used to specify the fields to be overwritten in the
    /// Target resource by the update.
    /// The fields specified in the update_mask are relative to the resource, not
    /// the full request. A field will be overwritten if it is in the mask. If the
    /// user does not provide a mask then all fields will be overwritten.
    #[prost(message, optional, tag = "1")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Required. The `Target` to update.
    #[prost(message, optional, tag = "2")]
    pub target: ::core::option::Option<Target>,
    /// Optional. A request ID to identify requests. Specify a unique request ID
    /// so that if you must retry your request, the server will know to ignore
    /// the request if it has already been completed. The server will guarantee
    /// that for at least 60 minutes since the first request.
    ///
    /// For example, consider a situation where you make an initial request and the
    /// request times out. If you make the request again with the same request ID,
    /// the server can check if original operation with the same request ID was
    /// received, and if so, will ignore the second request. This prevents clients
    /// from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "3")]
    pub request_id: ::prost::alloc::string::String,
    /// Optional. If set to true, updating a `Target` that does not exist will
    /// result in the creation of a new `Target`.
    #[prost(bool, tag = "4")]
    pub allow_missing: bool,
    /// Optional. If set to true, the request is validated and the user is provided with
    /// an expected result, but no actual change is made.
    #[prost(bool, tag = "5")]
    pub validate_only: bool,
}
/// The request object for `DeleteTarget`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteTargetRequest {
    /// Required. The name of the `Target` to delete. Format should be
    /// projects/{project_id}/locations/{location_name}/targets/{target_name}.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. A request ID to identify requests. Specify a unique request ID
    /// so that if you must retry your request, the server will know to ignore
    /// the request if it has already been completed. The server will guarantee
    /// that for at least 60 minutes after the first request.
    ///
    /// For example, consider a situation where you make an initial request and the
    /// request times out. If you make the request again with the same request ID,
    /// the server can check if original operation with the same request ID was
    /// received, and if so, will ignore the second request. This prevents clients
    /// from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "2")]
    pub request_id: ::prost::alloc::string::String,
    /// Optional. If set to true, then deleting an already deleted or non-existing
    /// DeliveryPipeline will succeed.
    #[prost(bool, tag = "3")]
    pub allow_missing: bool,
    /// Optional. If set, validate the request and preview the review, but do not actually
    /// post it.
    #[prost(bool, tag = "4")]
    pub validate_only: bool,
    /// Optional. This checksum is computed by the server based on the value of other
    /// fields, and may be sent on update and delete requests to ensure the
    /// client has an up-to-date value before proceeding.
    #[prost(string, tag = "5")]
    pub etag: ::prost::alloc::string::String,
}
/// A `Release` resource in the Google Cloud Deploy API.
///
/// A `Release` defines a specific Skaffold configuration instance
/// that can be deployed.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Release {
    /// Optional. Name of the `Release`. Format is projects/{project}/
    /// locations/{location}/deliveryPipelines/{deliveryPipeline}/
    /// releases/\[a-z][a-z0-9\-\]{0,62}.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. Unique identifier of the `Release`.
    #[prost(string, tag = "2")]
    pub uid: ::prost::alloc::string::String,
    /// Description of the `Release`. Max length is 255 characters.
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    /// User annotations. These attributes can only be set and used by the
    /// user, and not by Google Cloud Deploy. See
    /// <https://google.aip.dev/128#annotations> for more details such as format and
    /// size limitations.
    #[prost(map = "string, string", tag = "4")]
    pub annotations:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Labels are attributes that can be set and used by both the
    /// user and by Google Cloud Deploy. Labels must meet the following
    /// constraints:
    ///
    /// * Keys and values can contain only lowercase letters, numeric characters,
    /// underscores, and dashes.
    /// * All characters must use UTF-8 encoding, and international characters are
    /// allowed.
    /// * Keys must start with a lowercase letter or international character.
    /// * Each resource is limited to a maximum of 64 labels.
    ///
    /// Both keys and values are additionally constrained to be <= 128 bytes.
    #[prost(map = "string, string", tag = "5")]
    pub labels:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Output only. Time at which the `Release` was created.
    #[prost(message, optional, tag = "6")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Time at which the render began.
    #[prost(message, optional, tag = "7")]
    pub render_start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Time at which the render completed.
    #[prost(message, optional, tag = "8")]
    pub render_end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Cloud Storage URI of tar.gz archive containing Skaffold configuration.
    #[prost(string, tag = "17")]
    pub skaffold_config_uri: ::prost::alloc::string::String,
    /// Filepath of the Skaffold config inside of the config URI.
    #[prost(string, tag = "9")]
    pub skaffold_config_path: ::prost::alloc::string::String,
    /// List of artifacts to pass through to Skaffold command.
    #[prost(message, repeated, tag = "10")]
    pub build_artifacts: ::prost::alloc::vec::Vec<BuildArtifact>,
    /// Output only. Snapshot of the parent pipeline taken at release creation time.
    #[prost(message, optional, tag = "11")]
    pub delivery_pipeline_snapshot: ::core::option::Option<DeliveryPipeline>,
    /// Output only. Snapshot of the targets taken at release creation time.
    #[prost(message, repeated, tag = "12")]
    pub target_snapshots: ::prost::alloc::vec::Vec<Target>,
    /// Output only. Current state of the render operation.
    #[prost(enumeration = "release::RenderState", tag = "13")]
    pub render_state: i32,
    /// This checksum is computed by the server based on the value of other
    /// fields, and may be sent on update and delete requests to ensure the
    /// client has an up-to-date value before proceeding.
    #[prost(string, tag = "16")]
    pub etag: ::prost::alloc::string::String,
    /// The Skaffold version to use when operating on this release, such as
    /// "1.20.0". Not all versions are valid; Google Cloud Deploy supports a
    /// specific set of versions.
    ///
    /// If unset, the most recent supported Skaffold version will be used.
    #[prost(string, tag = "19")]
    pub skaffold_version: ::prost::alloc::string::String,
    /// Output only. Map from target ID to the target artifacts created
    /// during the render operation.
    #[prost(map = "string, message", tag = "20")]
    pub target_artifacts:
        ::std::collections::HashMap<::prost::alloc::string::String, TargetArtifact>,
    /// Output only. Map from target ID to details of the render operation for that target.
    #[prost(map = "string, message", tag = "22")]
    pub target_renders:
        ::std::collections::HashMap<::prost::alloc::string::String, release::TargetRender>,
}
/// Nested message and enum types in `Release`.
pub mod release {
    /// Details of rendering for a single target.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TargetRender {
        /// Output only. The resource name of the Cloud Build `Build` object that is used to
        /// render the manifest for this target. Format is
        /// `projects/{project}/locations/{location}/builds/{build}`.
        #[prost(string, tag = "1")]
        pub rendering_build: ::prost::alloc::string::String,
        /// Output only. Current state of the render operation for this Target.
        #[prost(enumeration = "target_render::TargetRenderState", tag = "2")]
        pub rendering_state: i32,
        /// Output only. Reason this render failed. This will always be unspecified while the
        /// render in progress.
        #[prost(enumeration = "target_render::FailureCause", tag = "4")]
        pub failure_cause: i32,
    }
    /// Nested message and enum types in `TargetRender`.
    pub mod target_render {
        /// Valid states of the render operation.
        #[derive(
            Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration,
        )]
        #[repr(i32)]
        pub enum TargetRenderState {
            /// The render operation state is unspecified.
            Unspecified = 0,
            /// The render operation has completed successfully.
            Succeeded = 1,
            /// The render operation has failed.
            Failed = 2,
            /// The render operation is in progress.
            InProgress = 3,
        }
        /// Well-known rendering failures.
        #[derive(
            Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration,
        )]
        #[repr(i32)]
        pub enum FailureCause {
            /// No reason for failure is specified.
            Unspecified = 0,
            /// Cloud Build is not available, either because it is not enabled or
            /// because Cloud Deploy has insufficient permissions. See [required
            /// permission](/deploy/docs/cloud-deploy-service-account#required_permissions).
            CloudBuildUnavailable = 1,
            /// The render operation did not complete successfully; check Cloud Build
            /// logs.
            ExecutionFailed = 2,
        }
    }
    /// Valid states of the render operation.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum RenderState {
        /// The render state is unspecified.
        Unspecified = 0,
        /// All rendering operations have completed successfully.
        Succeeded = 1,
        /// All rendering operations have completed, and one or more have failed.
        Failed = 2,
        /// Rendering has started and is not complete.
        InProgress = 3,
    }
}
/// Description of an a image to use during Skaffold rendering.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BuildArtifact {
    /// Image name in Skaffold configuration.
    #[prost(string, tag = "3")]
    pub image: ::prost::alloc::string::String,
    /// Image tag to use. This will generally be the full path to an image, such
    /// as "gcr.io/my-project/busybox:1.2.3" or
    /// "gcr.io/my-project/busybox@sha256:abc123".
    #[prost(string, tag = "2")]
    pub tag: ::prost::alloc::string::String,
}
/// The artifacts produced by a target render operation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TargetArtifact {
    /// Output only. File path of the resolved Skaffold configuration relative to the URI.
    #[prost(string, tag = "2")]
    pub skaffold_config_path: ::prost::alloc::string::String,
    /// Output only. File path of the rendered manifest relative to the URI.
    #[prost(string, tag = "3")]
    pub manifest_path: ::prost::alloc::string::String,
    #[prost(oneof = "target_artifact::Uri", tags = "4")]
    pub uri: ::core::option::Option<target_artifact::Uri>,
}
/// Nested message and enum types in `TargetArtifact`.
pub mod target_artifact {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Uri {
        /// Output only. URI of a directory containing the artifacts. This contains
        /// deployment configuration used by Skaffold during a rollout, and all
        /// paths are relative to this location.
        #[prost(string, tag = "4")]
        ArtifactUri(::prost::alloc::string::String),
    }
}
/// The request object for `ListReleases`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListReleasesRequest {
    /// Required. The `DeliveryPipeline` which owns this collection of `Release` objects.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. The maximum number of `Release` objects to return. The service may return
    /// fewer than this value. If unspecified, at most 50 `Release` objects will be
    /// returned. The maximum value is 1000; values above 1000 will be set to 1000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. A page token, received from a previous `ListReleases` call.
    /// Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other provided parameters match
    /// the call that provided the page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. Filter releases to be returned. See <https://google.aip.dev/160> for more
    /// details.
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Optional. Field to sort by. See <https://google.aip.dev/132#ordering> for more details.
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
/// The response object from `ListReleases`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListReleasesResponse {
    /// The `Release` objects.
    #[prost(message, repeated, tag = "1")]
    pub releases: ::prost::alloc::vec::Vec<Release>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// The request object for `GetRelease`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetReleaseRequest {
    /// Required. Name of the `Release`. Format must be
    /// projects/{project_id}/locations/{location_name}/deliveryPipelines/{pipeline_name}/releases/{release_name}.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// The request object for `CreateRelease`,
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateReleaseRequest {
    /// Required. The parent collection in which the `Release` should be created.
    /// Format should be
    /// projects/{project_id}/locations/{location_name}/deliveryPipelines/{pipeline_name}.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. ID of the `Release`.
    #[prost(string, tag = "2")]
    pub release_id: ::prost::alloc::string::String,
    /// Required. The `Release` to create.
    #[prost(message, optional, tag = "3")]
    pub release: ::core::option::Option<Release>,
    /// Optional. A request ID to identify requests. Specify a unique request ID
    /// so that if you must retry your request, the server will know to ignore
    /// the request if it has already been completed. The server will guarantee
    /// that for at least 60 minutes since the first request.
    ///
    /// For example, consider a situation where you make an initial request and the
    /// request times out. If you make the request again with the same request ID,
    /// the server can check if original operation with the same request ID was
    /// received, and if so, will ignore the second request. This prevents clients
    /// from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "4")]
    pub request_id: ::prost::alloc::string::String,
    /// Optional. If set to true, the request is validated and the user is provided with
    /// an expected result, but no actual change is made.
    #[prost(bool, tag = "5")]
    pub validate_only: bool,
}
/// A `Rollout` resource in the Google Cloud Deploy API.
///
/// A `Rollout` contains information around a specific deployment to a `Target`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Rollout {
    /// Optional. Name of the `Rollout`. Format is projects/{project}/
    /// locations/{location}/deliveryPipelines/{deliveryPipeline}/
    /// releases/{release}/rollouts/\[a-z][a-z0-9\-\]{0,62}.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. Unique identifier of the `Rollout`.
    #[prost(string, tag = "2")]
    pub uid: ::prost::alloc::string::String,
    /// Description of the `Rollout` for user purposes. Max length is 255
    /// characters.
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    /// User annotations. These attributes can only be set and used by the
    /// user, and not by Google Cloud Deploy. See
    /// <https://google.aip.dev/128#annotations> for more details such as format and
    /// size limitations.
    #[prost(map = "string, string", tag = "4")]
    pub annotations:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Labels are attributes that can be set and used by both the
    /// user and by Google Cloud Deploy. Labels must meet the following
    /// constraints:
    ///
    /// * Keys and values can contain only lowercase letters, numeric characters,
    /// underscores, and dashes.
    /// * All characters must use UTF-8 encoding, and international characters are
    /// allowed.
    /// * Keys must start with a lowercase letter or international character.
    /// * Each resource is limited to a maximum of 64 labels.
    ///
    /// Both keys and values are additionally constrained to be <= 128 bytes.
    #[prost(map = "string, string", tag = "5")]
    pub labels:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// Output only. Time at which the `Rollout` was created.
    #[prost(message, optional, tag = "6")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Time at which the `Rollout` was approved.
    #[prost(message, optional, tag = "7")]
    pub approve_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Time at which the `Rollout` was enqueued.
    #[prost(message, optional, tag = "8")]
    pub enqueue_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Time at which the `Rollout` started deploying.
    #[prost(message, optional, tag = "9")]
    pub deploy_start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Time at which the `Rollout` finished deploying.
    #[prost(message, optional, tag = "10")]
    pub deploy_end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Required. The ID of Target to which this `Rollout` is deploying.
    #[prost(string, tag = "18")]
    pub target_id: ::prost::alloc::string::String,
    /// Output only. Approval state of the `Rollout`.
    #[prost(enumeration = "rollout::ApprovalState", tag = "12")]
    pub approval_state: i32,
    /// Output only. Current state of the `Rollout`.
    #[prost(enumeration = "rollout::State", tag = "13")]
    pub state: i32,
    /// Output only. Reason the build failed. Empty if the build succeeded.
    #[prost(string, tag = "14")]
    pub failure_reason: ::prost::alloc::string::String,
    /// Output only. The resource name of the Cloud Build `Build` object that is used to deploy
    /// the Rollout. Format is
    /// `projects/{project}/locations/{location}/builds/{build}`.
    #[prost(string, tag = "17")]
    pub deploying_build: ::prost::alloc::string::String,
    /// This checksum is computed by the server based on the value of other
    /// fields, and may be sent on update and delete requests to ensure the
    /// client has an up-to-date value before proceeding.
    #[prost(string, tag = "16")]
    pub etag: ::prost::alloc::string::String,
    /// Output only. The reason this deploy failed. This will always be unspecified while the
    /// deploy in progress.
    #[prost(enumeration = "rollout::FailureCause", tag = "19")]
    pub deploy_failure_cause: i32,
}
/// Nested message and enum types in `Rollout`.
pub mod rollout {
    /// Valid approval states of a `Rollout`.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ApprovalState {
        /// The `Rollout` has an unspecified approval state.
        Unspecified = 0,
        /// The `Rollout` requires approval.
        NeedsApproval = 1,
        /// The `Rollout` does not require approval.
        DoesNotNeedApproval = 2,
        /// The `Rollout` has been approved.
        Approved = 3,
        /// The `Rollout` has been rejected.
        Rejected = 4,
    }
    /// Valid states of a `Rollout`.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// The `Rollout` has an unspecified state.
        Unspecified = 0,
        /// The `Rollout` has completed successfully.
        Succeeded = 1,
        /// The `Rollout` has failed.
        Failed = 2,
        /// The `Rollout` is being deployed.
        InProgress = 3,
        /// The `Rollout` needs approval.
        PendingApproval = 4,
        /// An approver rejected the `Rollout`.
        ApprovalRejected = 5,
        /// The `Rollout` is waiting for an earlier Rollout(s) to complete on this
        /// `Target`.
        Pending = 6,
        /// The `Rollout` is waiting for the `Release` to be fully rendered.
        PendingRelease = 7,
    }
    /// Well-known deployment failures.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum FailureCause {
        /// No reason for failure is specified.
        Unspecified = 0,
        /// Cloud Build is not available, either because it is not enabled or because
        /// Cloud Deploy has insufficient permissions. See [required
        /// permission](/deploy/docs/cloud-deploy-service-account#required_permissions).
        CloudBuildUnavailable = 1,
        /// The deploy operation did not complete successfully; check Cloud Build
        /// logs.
        ExecutionFailed = 2,
        /// Deployment did not complete within the alloted time.
        DeadlineExceeded = 3,
        /// Release is in a failed state.
        ReleaseFailed = 4,
    }
}
/// ListRolloutsRequest is the request object used by `ListRollouts`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListRolloutsRequest {
    /// Required. The `Release` which owns this collection of `Rollout` objects.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. The maximum number of `Rollout` objects to return. The service may return
    /// fewer than this value. If unspecified, at most 50 `Rollout` objects will be
    /// returned. The maximum value is 1000; values above 1000 will be set to 1000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. A page token, received from a previous `ListRollouts` call.
    /// Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other provided parameters match
    /// the call that provided the page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. Filter rollouts to be returned. See <https://google.aip.dev/160> for more
    /// details.
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Optional. Field to sort by. See <https://google.aip.dev/132#ordering> for more details.
    #[prost(string, tag = "5")]
    pub order_by: ::prost::alloc::string::String,
}
/// ListRolloutsResponse is the response object reutrned by `ListRollouts`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListRolloutsResponse {
    /// The `Rollout` objects.
    #[prost(message, repeated, tag = "1")]
    pub rollouts: ::prost::alloc::vec::Vec<Rollout>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// GetRolloutRequest is the request object used by `GetRollout`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRolloutRequest {
    /// Required. Name of the `Rollout`. Format must be
    /// projects/{project_id}/locations/{location_name}/deliveryPipelines/{pipeline_name}/releases/{release_name}/rollouts/{rollout_name}.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// CreateRolloutRequest is the request object used by `CreateRollout`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateRolloutRequest {
    /// Required. The parent collection in which the `Rollout` should be created.
    /// Format should be
    /// projects/{project_id}/locations/{location_name}/deliveryPipelines/{pipeline_name}/releases/{release_name}.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. ID of the `Rollout`.
    #[prost(string, tag = "2")]
    pub rollout_id: ::prost::alloc::string::String,
    /// Required. The `Rollout` to create.
    #[prost(message, optional, tag = "3")]
    pub rollout: ::core::option::Option<Rollout>,
    /// Optional. A request ID to identify requests. Specify a unique request ID
    /// so that if you must retry your request, the server will know to ignore
    /// the request if it has already been completed. The server will guarantee
    /// that for at least 60 minutes since the first request.
    ///
    /// For example, consider a situation where you make an initial request and the
    /// request times out. If you make the request again with the same request ID,
    /// the server can check if original operation with the same request ID was
    /// received, and if so, will ignore the second request. This prevents clients
    /// from accidentally creating duplicate commitments.
    ///
    /// The request ID must be a valid UUID with the exception that zero UUID is
    /// not supported (00000000-0000-0000-0000-000000000000).
    #[prost(string, tag = "4")]
    pub request_id: ::prost::alloc::string::String,
    /// Optional. If set to true, the request is validated and the user is provided with
    /// an expected result, but no actual change is made.
    #[prost(bool, tag = "5")]
    pub validate_only: bool,
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
/// The request object used by `ApproveRollout`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApproveRolloutRequest {
    /// Required. Name of the Rollout. Format is
    /// projects/{project}/locations/{location}/deliveryPipelines/{deliveryPipeline}/
    /// releases/{release}/rollouts/{rollout}.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. True = approve; false = reject
    #[prost(bool, tag = "2")]
    pub approved: bool,
}
/// The response object from `ApproveRollout`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApproveRolloutResponse {}
/// Service-wide configuration.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Config {
    /// Name of the configuration.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. All supported versions of Skaffold.
    #[prost(message, repeated, tag = "2")]
    pub supported_versions: ::prost::alloc::vec::Vec<SkaffoldVersion>,
    /// Output only. Default Skaffold version that is assigned when a Release is created without
    /// specifying a Skaffold version.
    #[prost(string, tag = "3")]
    pub default_skaffold_version: ::prost::alloc::string::String,
}
/// Details of a supported Skaffold version.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SkaffoldVersion {
    /// Release version number. For example, "1.20.3".
    #[prost(string, tag = "1")]
    pub version: ::prost::alloc::string::String,
    /// Date when this version is expected to no longer be supported.
    #[prost(message, optional, tag = "2")]
    pub support_end_date: ::core::option::Option<super::super::super::r#type::Date>,
}
/// Request to get a configuration.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetConfigRequest {
    /// Required. Name of requested configuration.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
#[doc = r" Generated client implementations."]
pub mod cloud_deploy_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " CloudDeploy service creates and manages Continuous Delivery operations"]
    #[doc = " on Google Cloud Platform via Skaffold (https://skaffold.dev)."]
    #[derive(Debug, Clone)]
    pub struct CloudDeployClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> CloudDeployClient<T>
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
        ) -> CloudDeployClient<InterceptedService<T, F>>
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
            CloudDeployClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Lists DeliveryPipelines in a given project and location."]
        pub async fn list_delivery_pipelines(
            &mut self,
            request: impl tonic::IntoRequest<super::ListDeliveryPipelinesRequest>,
        ) -> Result<tonic::Response<super::ListDeliveryPipelinesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.deploy.v1.CloudDeploy/ListDeliveryPipelines",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets details of a single DeliveryPipeline."]
        pub async fn get_delivery_pipeline(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDeliveryPipelineRequest>,
        ) -> Result<tonic::Response<super::DeliveryPipeline>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.deploy.v1.CloudDeploy/GetDeliveryPipeline",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a new DeliveryPipeline in a given project and location."]
        pub async fn create_delivery_pipeline(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateDeliveryPipelineRequest>,
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
                "/google.cloud.deploy.v1.CloudDeploy/CreateDeliveryPipeline",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates the parameters of a single DeliveryPipeline."]
        pub async fn update_delivery_pipeline(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateDeliveryPipelineRequest>,
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
                "/google.cloud.deploy.v1.CloudDeploy/UpdateDeliveryPipeline",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes a single DeliveryPipeline."]
        pub async fn delete_delivery_pipeline(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteDeliveryPipelineRequest>,
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
                "/google.cloud.deploy.v1.CloudDeploy/DeleteDeliveryPipeline",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists Targets in a given project and location."]
        pub async fn list_targets(
            &mut self,
            request: impl tonic::IntoRequest<super::ListTargetsRequest>,
        ) -> Result<tonic::Response<super::ListTargetsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.deploy.v1.CloudDeploy/ListTargets",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets details of a single Target."]
        pub async fn get_target(
            &mut self,
            request: impl tonic::IntoRequest<super::GetTargetRequest>,
        ) -> Result<tonic::Response<super::Target>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.deploy.v1.CloudDeploy/GetTarget",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a new Target in a given project and location."]
        pub async fn create_target(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateTargetRequest>,
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
                "/google.cloud.deploy.v1.CloudDeploy/CreateTarget",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates the parameters of a single Target."]
        pub async fn update_target(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateTargetRequest>,
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
                "/google.cloud.deploy.v1.CloudDeploy/UpdateTarget",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes a single Target."]
        pub async fn delete_target(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteTargetRequest>,
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
                "/google.cloud.deploy.v1.CloudDeploy/DeleteTarget",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists Releases in a given project and location."]
        pub async fn list_releases(
            &mut self,
            request: impl tonic::IntoRequest<super::ListReleasesRequest>,
        ) -> Result<tonic::Response<super::ListReleasesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.deploy.v1.CloudDeploy/ListReleases",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets details of a single Release."]
        pub async fn get_release(
            &mut self,
            request: impl tonic::IntoRequest<super::GetReleaseRequest>,
        ) -> Result<tonic::Response<super::Release>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.deploy.v1.CloudDeploy/GetRelease",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a new Release in a given project and location."]
        pub async fn create_release(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateReleaseRequest>,
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
                "/google.cloud.deploy.v1.CloudDeploy/CreateRelease",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Approves a Rollout."]
        pub async fn approve_rollout(
            &mut self,
            request: impl tonic::IntoRequest<super::ApproveRolloutRequest>,
        ) -> Result<tonic::Response<super::ApproveRolloutResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.deploy.v1.CloudDeploy/ApproveRollout",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists Rollouts in a given project and location."]
        pub async fn list_rollouts(
            &mut self,
            request: impl tonic::IntoRequest<super::ListRolloutsRequest>,
        ) -> Result<tonic::Response<super::ListRolloutsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.deploy.v1.CloudDeploy/ListRollouts",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets details of a single Rollout."]
        pub async fn get_rollout(
            &mut self,
            request: impl tonic::IntoRequest<super::GetRolloutRequest>,
        ) -> Result<tonic::Response<super::Rollout>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.deploy.v1.CloudDeploy/GetRollout",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a new Rollout in a given project and location."]
        pub async fn create_rollout(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateRolloutRequest>,
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
                "/google.cloud.deploy.v1.CloudDeploy/CreateRollout",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets the configuration for a location."]
        pub async fn get_config(
            &mut self,
            request: impl tonic::IntoRequest<super::GetConfigRequest>,
        ) -> Result<tonic::Response<super::Config>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.deploy.v1.CloudDeploy/GetConfig",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Type indicates the type of the log entry and can be used as a filter.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Type {
    /// Type is unspecified.
    Unspecified = 0,
    /// A Pub/Sub notification failed to be sent.
    PubsubNotificationFailure = 1,
    /// Release render status changed notification.
    RenderStatuesChange = 2,
}
/// Payload proto for "clouddeploy.googleapis.com/deliverypipeline_notification"
/// Platform Log event that describes the failure to send delivery pipeline
/// status change Pub/Sub notification.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeliveryPipelineNotificationEvent {
    /// Debug message for when a notification fails to send.
    #[prost(string, tag = "1")]
    pub message: ::prost::alloc::string::String,
    /// The name of the `Delivery Pipeline`.
    #[prost(string, tag = "2")]
    pub delivery_pipeline: ::prost::alloc::string::String,
    /// Type of this notification, e.g. for a Pub/Sub failure.
    #[prost(enumeration = "Type", tag = "3")]
    pub r#type: i32,
}
/// Payload proto for "clouddeploy.googleapis.com/release_notification"
/// Platform Log event that describes the failure to send release status change
/// Pub/Sub notification.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReleaseNotificationEvent {
    /// Debug message for when a notification fails to send.
    #[prost(string, tag = "1")]
    pub message: ::prost::alloc::string::String,
    /// The name of the `Release`.
    #[prost(string, tag = "2")]
    pub release: ::prost::alloc::string::String,
    /// Type of this notification, e.g. for a Pub/Sub failure.
    #[prost(enumeration = "Type", tag = "3")]
    pub r#type: i32,
}
/// Payload proto for "clouddeploy.googleapis.com/release_render"
/// Platform Log event that describes the render status change.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReleaseRenderEvent {
    /// Debug message for when a render transition occurs. Provides further
    /// details as rendering progresses through render states.
    #[prost(string, tag = "1")]
    pub message: ::prost::alloc::string::String,
    /// The name of the `Release`.
    #[prost(string, tag = "2")]
    pub release: ::prost::alloc::string::String,
}
/// Payload proto for "clouddeploy.googleapis.com/rollout_notification"
/// Platform Log event that describes the failure to send rollout status change
/// Pub/Sub notification.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RolloutNotificationEvent {
    /// Debug message for when a notification fails to send.
    #[prost(string, tag = "1")]
    pub message: ::prost::alloc::string::String,
    /// Unique identifier of the `DeliveryPipeline`.
    #[prost(string, tag = "2")]
    pub pipeline_uid: ::prost::alloc::string::String,
    /// Unique identifier of the `Release`.
    #[prost(string, tag = "3")]
    pub release_uid: ::prost::alloc::string::String,
    /// The name of the `Rollout`.
    #[prost(string, tag = "4")]
    pub rollout: ::prost::alloc::string::String,
    /// Type of this notification, e.g. for a Pub/Sub failure.
    #[prost(enumeration = "Type", tag = "5")]
    pub r#type: i32,
    /// ID of the `Target` that the rollout is deployed to.
    #[prost(string, tag = "6")]
    pub target_id: ::prost::alloc::string::String,
}
/// Payload proto for "clouddeploy.googleapis.com/target_notification"
/// Platform Log event that describes the failure to send target status change
/// Pub/Sub notification.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TargetNotificationEvent {
    /// Debug message for when a notification fails to send.
    #[prost(string, tag = "1")]
    pub message: ::prost::alloc::string::String,
    /// The name of the `Target`.
    #[prost(string, tag = "2")]
    pub target: ::prost::alloc::string::String,
    /// Type of this notification, e.g. for a Pub/Sub failure.
    #[prost(enumeration = "Type", tag = "3")]
    pub r#type: i32,
}
