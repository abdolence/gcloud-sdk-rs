/// Describes the state of a job.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum JobState {
    /// The job state is unspecified.
    Unspecified = 0,
    /// The service is preparing to run the job.
    Pending = 9,
    /// The job is in progress.
    Running = 1,
    /// The job completed successfully.
    Succeeded = 2,
    /// The job failed.
    Failed = 3,
    /// The job has been cancelled.
    Cancelled = 4,
    /// Entity Recon API: The knowledge extraction job is running.
    KnowledgeExtraction = 5,
    /// Entity Recon API: The preprocessing job is running.
    ReconPreprocessing = 6,
    /// Entity Recon API: The clustering job is running.
    Clustering = 7,
    /// Entity Recon API: The exporting clusters job is running.
    ExportingClusters = 8,
}
impl JobState {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            JobState::Unspecified => "JOB_STATE_UNSPECIFIED",
            JobState::Pending => "JOB_STATE_PENDING",
            JobState::Running => "JOB_STATE_RUNNING",
            JobState::Succeeded => "JOB_STATE_SUCCEEDED",
            JobState::Failed => "JOB_STATE_FAILED",
            JobState::Cancelled => "JOB_STATE_CANCELLED",
            JobState::KnowledgeExtraction => "JOB_STATE_KNOWLEDGE_EXTRACTION",
            JobState::ReconPreprocessing => "JOB_STATE_RECON_PREPROCESSING",
            JobState::Clustering => "JOB_STATE_CLUSTERING",
            JobState::ExportingClusters => "JOB_STATE_EXPORTING_CLUSTERS",
        }
    }
}
/// The common metadata for long running operations.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommonOperationMetadata {
    /// The state of the operation.
    #[prost(enumeration="common_operation_metadata::State", tag="1")]
    pub state: i32,
    /// The creation time of the operation.
    #[prost(message, optional, tag="2")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The last update time of the operation.
    #[prost(message, optional, tag="3")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Nested message and enum types in `CommonOperationMetadata`.
pub mod common_operation_metadata {
    /// State of the longrunning operation.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// Unspecified state.
        Unspecified = 0,
        /// Operation is still running.
        Running = 1,
        /// Operation is being cancelled.
        Cancelling = 2,
        /// Operation succeeded.
        Succeeded = 3,
        /// Operation failed.
        Failed = 4,
        /// Operation is cancelled.
        Cancelled = 5,
        /// Operation is pending not running yet.
        Pending = 6,
    }
    impl State {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                State::Unspecified => "STATE_UNSPECIFIED",
                State::Running => "RUNNING",
                State::Cancelling => "CANCELLING",
                State::Succeeded => "SUCCEEDED",
                State::Failed => "FAILED",
                State::Cancelled => "CANCELLED",
                State::Pending => "PENDING",
            }
        }
    }
}
/// The desired input location and metadata.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InputConfig {
    /// Set of input BigQuery tables.
    #[prost(message, repeated, tag="1")]
    pub bigquery_input_configs: ::prost::alloc::vec::Vec<BigQueryInputConfig>,
    /// Entity type
    #[prost(enumeration="input_config::EntityType", tag="2")]
    pub entity_type: i32,
    /// Optional. Provide the bigquery table containing the previous results if
    /// cluster ID stability is desired. Format is
    /// “projects/*/datasets/*/tables/*".
    #[prost(string, tag="3")]
    pub previous_result_bigquery_table: ::prost::alloc::string::String,
}
/// Nested message and enum types in `InputConfig`.
pub mod input_config {
    /// The type of entities we will support. Currently, we only support people,
    /// establishment, property, and product types. If the type is
    /// unspecified, it will be generic type.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum EntityType {
        /// The default value.
        Unspecified = 0,
        /// People entity.
        People = 1,
        /// Establishment entity.
        Establishment = 2,
        /// Property entity. e.g. real estate property.
        Property = 3,
        /// Product entity.
        Product = 4,
        /// Organization entity.
        Organization = 5,
        /// Local Business entity.
        LocalBusiness = 6,
        /// Person entity.
        Person = 7,
    }
    impl EntityType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                EntityType::Unspecified => "ENTITY_TYPE_UNSPECIFIED",
                EntityType::People => "PEOPLE",
                EntityType::Establishment => "ESTABLISHMENT",
                EntityType::Property => "PROPERTY",
                EntityType::Product => "PRODUCT",
                EntityType::Organization => "ORGANIZATION",
                EntityType::LocalBusiness => "LOCAL_BUSINESS",
                EntityType::Person => "PERSON",
            }
        }
    }
}
/// The input config for BigQuery tables.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BigQueryInputConfig {
    /// Required. Format is “projects/*/datasets/*/tables/*”.
    #[prost(string, tag="1")]
    pub bigquery_table: ::prost::alloc::string::String,
    /// Required. Schema mapping file
    #[prost(string, tag="2")]
    pub gcs_uri: ::prost::alloc::string::String,
}
/// The desired output location and metadata.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OutputConfig {
    /// Format is “projects/*/datasets/*”.
    #[prost(string, tag="1")]
    pub bigquery_dataset: ::prost::alloc::string::String,
}
/// Recon configs
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReconConfig {
    /// Extra options that affect entity clustering behavior.
    #[prost(message, optional, tag="3")]
    pub options: ::core::option::Option<recon_config::Options>,
    /// Model Configs
    #[prost(message, optional, tag="4")]
    pub model_config: ::core::option::Option<recon_config::ModelConfig>,
    /// Choice of clustering algorithm. Default is ConnectedComponentsConfig.
    #[prost(oneof="recon_config::ClusteringConfig", tags="1, 2")]
    pub clustering_config: ::core::option::Option<recon_config::ClusteringConfig>,
}
/// Nested message and enum types in `ReconConfig`.
pub mod recon_config {
    /// Options for experimental changes on entity clustering behavior.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Options {
        /// If true, separate clusters by their geographic region (from geocoding).
        /// Uses the following entity features:
        ///   - schema.org/addressLocality
        ///   - schema.org/addressRegion
        ///   - schema.org/addressCountry
        /// Warning: processing will no longer be regionalized!
        #[prost(bool, tag="100")]
        pub enable_geocoding_separation: bool,
    }
    /// Model Configs
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ModelConfig {
        /// Model name. Refer to external documentation for valid names.
        /// If unspecified, it defaults to the one mentioned in the documentation.
        #[prost(string, tag="1")]
        pub model_name: ::prost::alloc::string::String,
        /// Model version tag. Refer to external documentation for valid tags.
        /// If unspecified, it defaults to the one mentioned in the documentation.
        #[prost(string, tag="2")]
        pub version_tag: ::prost::alloc::string::String,
    }
    /// Choice of clustering algorithm. Default is ConnectedComponentsConfig.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ClusteringConfig {
        /// Configs for connected components.
        #[prost(message, tag="1")]
        ConnectedComponentsConfig(super::ConnectedComponentsConfig),
        /// Configs for affinity clustering.
        #[prost(message, tag="2")]
        AffinityClusteringConfig(super::AffinityClusteringConfig),
    }
}
/// Options for connected components.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConnectedComponentsConfig {
    /// Threshold used for connected components. Default value is 0.85.
    #[prost(float, tag="1")]
    pub weight_threshold: f32,
}
/// Options for affinity clustering.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AffinityClusteringConfig {
    /// Number of iterations to perform. Default value is 1.
    #[prost(int64, tag="1")]
    pub compression_round_count: i64,
}
/// Details of operations that perform deletes of any entities.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteOperationMetadata {
    /// The common part of the operation metadata.
    #[prost(message, optional, tag="1")]
    pub common_metadata: ::core::option::Option<CommonOperationMetadata>,
}
/// Request message for CreateEntityReconciliationJob.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateEntityReconciliationJobRequest {
    /// Required. The resource name of the Location to create the
    /// EntityReconciliationJob in. Format:
    /// `projects/{project}/locations/{location}`
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The EntityReconciliationJob to create.
    #[prost(message, optional, tag="2")]
    pub entity_reconciliation_job: ::core::option::Option<EntityReconciliationJob>,
}
/// Request message for GetEntityReconciliationJob.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetEntityReconciliationJobRequest {
    /// Required. The name of the EntityReconciliationJob resource.
    /// Format:
    /// `projects/{project}/locations/{location}/entityReconciliationJobs/{entity_reconciliation_job}`
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for
/// \[EnterpriseKnowledgeGraphService.ListEntityReconciliationJobs][google.cloud.enterpriseknowledgegraph.v1.EnterpriseKnowledgeGraphService.ListEntityReconciliationJobs\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListEntityReconciliationJobsRequest {
    /// Required. The name of the EntityReconciliationJob's parent resource.
    /// Format: `projects/{project}/locations/{location}`
    #[prost(string, tag="1")]
    pub parent: ::prost::alloc::string::String,
    /// An expression for filtering the results of the request. For field names
    /// both snake_case and camelCase are supported.
    #[prost(string, tag="2")]
    pub filter: ::prost::alloc::string::String,
    /// The standard list page size.
    #[prost(int32, tag="3")]
    pub page_size: i32,
    /// The standard list page token.
    #[prost(string, tag="4")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for
/// \[EnterpriseKnowledgeGraphService.ListEntityReconciliationJobs][google.cloud.enterpriseknowledgegraph.v1.EnterpriseKnowledgeGraphService.ListEntityReconciliationJobs\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListEntityReconciliationJobsResponse {
    /// A list of EntityReconciliationJobs that matches the specified filter in the
    /// request.
    #[prost(message, repeated, tag="1")]
    pub entity_reconciliation_jobs: ::prost::alloc::vec::Vec<EntityReconciliationJob>,
    /// The standard List next-page token.
    #[prost(string, tag="2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for CancelEntityReconciliationJob.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CancelEntityReconciliationJobRequest {
    /// Required. The name of the EntityReconciliationJob resource.
    /// Format:
    /// `projects/{project}/locations/{location}/entityReconciliationJobs/{entity_reconciliation_job}`
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for DeleteEntityReconciliationJob.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteEntityReconciliationJobRequest {
    /// Required. The name of the EntityReconciliationJob resource.
    /// Format:
    /// `projects/{project}/locations/{location}/entityReconciliationJobs/{entity_reconciliation_job}`
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
/// Entity reconciliation job message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EntityReconciliationJob {
    /// Output only. Resource name of the EntityReconciliationJob.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Required. Information about the input BigQuery tables.
    #[prost(message, optional, tag="2")]
    pub input_config: ::core::option::Option<InputConfig>,
    /// Required. The desired output location.
    #[prost(message, optional, tag="3")]
    pub output_config: ::core::option::Option<OutputConfig>,
    /// Output only. The detailed state of the job.
    #[prost(enumeration="JobState", tag="4")]
    pub state: i32,
    /// Output only. Only populated when the job's state is JOB_STATE_FAILED or
    /// JOB_STATE_CANCELLED.
    #[prost(message, optional, tag="5")]
    pub error: ::core::option::Option<super::super::super::rpc::Status>,
    /// Output only. Time when the EntityReconciliationJob was created.
    #[prost(message, optional, tag="6")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Time when the EntityReconciliationJob entered any of the
    /// following states: `JOB_STATE_SUCCEEDED`, `JOB_STATE_FAILED`,
    /// `JOB_STATE_CANCELLED`.
    #[prost(message, optional, tag="7")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Output only. Time when the EntityReconciliationJob was most recently
    /// updated.
    #[prost(message, optional, tag="8")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Optional. Recon configs to adjust the clustering behavior.
    #[prost(message, optional, tag="9")]
    pub recon_config: ::core::option::Option<ReconConfig>,
}
/// Generated client implementations.
pub mod enterprise_knowledge_graph_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// APIs for enterprise knowledge graph product.
    #[derive(Debug, Clone)]
    pub struct EnterpriseKnowledgeGraphServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl EnterpriseKnowledgeGraphServiceClient<tonic::transport::Channel> {
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
    impl<T> EnterpriseKnowledgeGraphServiceClient<T>
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
        ) -> EnterpriseKnowledgeGraphServiceClient<InterceptedService<T, F>>
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
            EnterpriseKnowledgeGraphServiceClient::new(
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
        /// Creates a EntityReconciliationJob. A EntityReconciliationJob once created
        /// will right away be attempted to start.
        pub async fn create_entity_reconciliation_job(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateEntityReconciliationJobRequest>,
        ) -> Result<tonic::Response<super::EntityReconciliationJob>, tonic::Status> {
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
                "/google.cloud.enterpriseknowledgegraph.v1.EnterpriseKnowledgeGraphService/CreateEntityReconciliationJob",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Gets a EntityReconciliationJob.
        pub async fn get_entity_reconciliation_job(
            &mut self,
            request: impl tonic::IntoRequest<super::GetEntityReconciliationJobRequest>,
        ) -> Result<tonic::Response<super::EntityReconciliationJob>, tonic::Status> {
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
                "/google.cloud.enterpriseknowledgegraph.v1.EnterpriseKnowledgeGraphService/GetEntityReconciliationJob",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Lists Entity Reconciliation Jobs.
        pub async fn list_entity_reconciliation_jobs(
            &mut self,
            request: impl tonic::IntoRequest<super::ListEntityReconciliationJobsRequest>,
        ) -> Result<
            tonic::Response<super::ListEntityReconciliationJobsResponse>,
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
                "/google.cloud.enterpriseknowledgegraph.v1.EnterpriseKnowledgeGraphService/ListEntityReconciliationJobs",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Cancels a EntityReconciliationJob. Success of cancellation is not
        /// guaranteed.
        pub async fn cancel_entity_reconciliation_job(
            &mut self,
            request: impl tonic::IntoRequest<super::CancelEntityReconciliationJobRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
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
                "/google.cloud.enterpriseknowledgegraph.v1.EnterpriseKnowledgeGraphService/CancelEntityReconciliationJob",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deletes a EntityReconciliationJob.
        /// It only deletes the job when the job state is in FAILED, SUCCEEDED, and
        /// CANCELLED.
        pub async fn delete_entity_reconciliation_job(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteEntityReconciliationJobRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
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
                "/google.cloud.enterpriseknowledgegraph.v1.EnterpriseKnowledgeGraphService/DeleteEntityReconciliationJob",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
