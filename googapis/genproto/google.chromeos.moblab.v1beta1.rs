/// Resource that represents a build target.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BuildTarget {
    /// The resource name of the build target.
    /// Format: buildTargets/{build_target}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Resource that represents a model. Each model belongs to a build target. For
/// non-unified build, the model name is the same as its build target name.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Model {
    /// The resource name of the model.
    /// Format: buildTargets/{build_target}/models/{model}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Resource that represents a chrome OS milestone.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Milestone {
    /// The resource name of the milestone.
    /// Format: milestones/{milestone}
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Resource that represents a build for the given build target, model, milestone
/// and build version.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Build {
    /// The resource name of the build.
    /// Format: buildTargets/{build_target}/models/{model}/builds/{build}
    /// Example: buildTargets/octopus/models/bobba/builds/1234.0.0
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The milestone that owns the build.
    /// Format: milestones/{milestone}
    #[prost(string, tag = "2")]
    pub milestone: ::prost::alloc::string::String,
    /// The build version of the build, e.g. 1234.0.0.
    #[prost(string, tag = "3")]
    pub build_version: ::prost::alloc::string::String,
    /// The status of the build.
    #[prost(enumeration = "build::BuildStatus", tag = "4")]
    pub status: i32,
    /// The type of the build.
    #[prost(enumeration = "build::BuildType", tag = "5")]
    pub r#type: i32,
    /// The branch of the build.
    #[prost(string, tag = "6")]
    pub branch: ::prost::alloc::string::String,
    /// The read write firmware version of the software that is flashed to the chip
    /// on the Chrome OS device.
    #[prost(string, tag = "7")]
    pub rw_firmware_version: ::prost::alloc::string::String,
}
/// Nested message and enum types in `Build`.
pub mod build {
    /// The build status types.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum BuildStatus {
        /// No build status is specified.
        Unspecified = 0,
        /// Complete Status: The build passed.
        Pass = 1,
        /// Complete Status: The build failed.
        Fail = 2,
        /// Intermediate Status: The build is still running.
        Running = 3,
        /// Complete Status: The build was aborted.
        Aborted = 4,
    }
    /// The build types.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum BuildType {
        /// Invalid build type.
        Unspecified = 0,
        /// The release build.
        Release = 1,
        /// The firmware build.
        Firmware = 2,
    }
}
/// Resource that represents a build artifact stored in Google Cloud Storage for
/// the given build target, model, build version and bucket.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BuildArtifact {
    /// The resource name of the build artifact.
    /// Format:
    /// buildTargets/{build_target}/models/{model}/builds/{build}/artifacts/{artifact}
    /// Example:
    /// buildTargets/octopus/models/bobba/builds/1234.0.0/artifacts/chromeos-moblab-peng-staging
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The build metadata of the build artifact.
    #[prost(string, tag = "2")]
    pub build: ::prost::alloc::string::String,
    /// The bucket that stores the build artifact.
    #[prost(string, tag = "3")]
    pub bucket: ::prost::alloc::string::String,
    /// The path of the build artifact in the bucket.
    #[prost(string, tag = "4")]
    pub path: ::prost::alloc::string::String,
    /// The number of objects in the build artifact folder. The object number can
    /// be used to calculated the stage progress by comparing the source build
    /// artifact with the destination build artifact.
    #[prost(uint32, tag = "5")]
    pub object_count: u32,
}
/// Request message for finding the most stable build.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FindMostStableBuildRequest {
    /// Required. The full resource name of the build target.
    /// For example,
    /// 'buildTargets/octopus'.
    #[prost(string, tag = "1")]
    pub build_target: ::prost::alloc::string::String,
}
/// Response message for finding the most stable build.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FindMostStableBuildResponse {
    /// The most stable build.
    #[prost(message, optional, tag = "1")]
    pub build: ::core::option::Option<Build>,
}
/// Request message for listing build targets.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListBuildTargetsRequest {
    /// Optional. The number of build targets to return in a page.
    #[prost(int32, tag = "1")]
    pub page_size: i32,
    /// Optional. A page token, received from a previous `ListBuildTargets` call. Provide
    /// this to retrieve the subsequent page.
    #[prost(string, tag = "2")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for listing build targets.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListBuildTargetsResponse {
    /// The list of build targets.
    #[prost(message, repeated, tag = "1")]
    pub build_targets: ::prost::alloc::vec::Vec<BuildTarget>,
    /// Token to retrieve the next page of build targets. If this field is omitted,
    /// there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Total number of build targets.
    #[prost(int32, tag = "3")]
    pub total_size: i32,
}
/// Request message for listing models.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListModelsRequest {
    /// Required. The full resource name of build target.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. The number of models to return in a page.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. A page token, received from a previous `ListModels` call. Provide
    /// this to retrieve the subsequent page.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for listing models.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListModelsResponse {
    /// The list of models.
    #[prost(message, repeated, tag = "1")]
    pub models: ::prost::alloc::vec::Vec<Model>,
    /// Token to retrieve the next page of models. If this field is omitted, there
    /// are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Total number of models.
    #[prost(int32, tag = "3")]
    pub total_size: i32,
}
/// Request message for listing builds.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListBuildsRequest {
    /// Required. The full resource name of the model. The model id is the same as
    /// the build target id for non-unified builds.
    /// For example,
    /// 'buildTargets/octopus/models/bobba'.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. The number of builds to return in a page.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. A page token, received from a previous `ListBuilds` call. Provide this to
    /// retrieve the subsequent page.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
    /// Optional. Filter that specifies value constraints of fields. For example, the
    /// filter can be set as "filter='milestone=milestones/80'" to only select
    /// builds in milestone 80.
    #[prost(string, tag = "4")]
    pub filter: ::prost::alloc::string::String,
    /// Optional. Read mask that specifies which Build fields to return. If empty, all Build
    /// fields will be returned.
    /// Valid fields: name, milestone, build_version.
    /// For example, if the read_mask is set as "read_mask='milestone'", the
    /// ListBuilds will return a list of Builds object with only the milestone
    /// field.
    #[prost(message, optional, tag = "5")]
    pub read_mask: ::core::option::Option<::prost_types::FieldMask>,
    /// Optional. The operation that groups by all the Build fields specified in the read
    /// mask. The group_by field should be the same as the read_mask field in
    /// convention of SQL.
    #[prost(message, optional, tag = "6")]
    pub group_by: ::core::option::Option<::prost_types::FieldMask>,
}
/// Response message for listing builds.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListBuildsResponse {
    /// The list of builds.
    #[prost(message, repeated, tag = "1")]
    pub builds: ::prost::alloc::vec::Vec<Build>,
    /// Token to retrieve the next page of builds. If this field is omitted, there
    /// are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
    /// Total number of builds.
    #[prost(int32, tag = "3")]
    pub total_size: i32,
}
/// Request message for checking if the build artifact is staged.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckBuildStageStatusRequest {
    /// Required. The full resource name of the build artifact.
    /// For example,
    /// 'buildTargets/octopus/models/bobba/builds/12607.6.0/artifacts/chromeos-moblab-peng-staging'.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. Filter that specifies value constraints of fields. For example, the
    /// filter can be set as "filter='type=release'" to only check the release
    /// builds.
    #[prost(string, tag = "2")]
    pub filter: ::prost::alloc::string::String,
}
/// Response message for checking the stage status of a build artifact.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckBuildStageStatusResponse {
    /// The status to represent if the build is staged or not.
    #[prost(bool, tag = "1")]
    pub is_build_staged: bool,
    /// The staged build artifact in the destination bucket.
    #[prost(message, optional, tag = "2")]
    pub staged_build_artifact: ::core::option::Option<BuildArtifact>,
    /// The source build artifact in the source bucket.
    #[prost(message, optional, tag = "3")]
    pub source_build_artifact: ::core::option::Option<BuildArtifact>,
}
/// Request message for staging a build artifact.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StageBuildRequest {
    /// Required. The full resource name of the build artifact.
    /// For example,
    /// 'buildTargets/octopus/models/bobba/builds/12607.6.0/artifacts/chromeos-moblab-peng-staging'.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Optional. Filter that specifies value constraints of fields. For example, the
    /// filter can be set as "filter='type=release'" to only check the release
    /// builds.
    #[prost(string, tag = "2")]
    pub filter: ::prost::alloc::string::String,
}
/// Response message for staging a build artifact.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StageBuildResponse {
    /// The staged build in the destination bucket.
    #[prost(message, optional, tag = "1")]
    pub staged_build_artifact: ::core::option::Option<BuildArtifact>,
}
/// Metadata message for staging a build artifact.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StageBuildMetadata {
    /// Approximate percentage of progress, e.g. "50" means 50%.
    #[prost(float, tag = "1")]
    pub progress_percent: f32,
    /// Build stage start time.
    #[prost(message, optional, tag = "2")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Build stage end time.
    #[prost(message, optional, tag = "3")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
}
#[doc = r" Generated client implementations."]
pub mod build_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Manages Chrome OS build services."]
    #[derive(Debug, Clone)]
    pub struct BuildServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> BuildServiceClient<T>
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
        ) -> BuildServiceClient<InterceptedService<T, F>>
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
            BuildServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Lists all build targets that a user has access to."]
        pub async fn list_build_targets(
            &mut self,
            request: impl tonic::IntoRequest<super::ListBuildTargetsRequest>,
        ) -> Result<tonic::Response<super::ListBuildTargetsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.chromeos.moblab.v1beta1.BuildService/ListBuildTargets",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists all models for the given build target."]
        pub async fn list_models(
            &mut self,
            request: impl tonic::IntoRequest<super::ListModelsRequest>,
        ) -> Result<tonic::Response<super::ListModelsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.chromeos.moblab.v1beta1.BuildService/ListModels",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists all builds for the given build target and model in descending order"]
        #[doc = " for the milestones and build versions."]
        pub async fn list_builds(
            &mut self,
            request: impl tonic::IntoRequest<super::ListBuildsRequest>,
        ) -> Result<tonic::Response<super::ListBuildsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.chromeos.moblab.v1beta1.BuildService/ListBuilds",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Checks the stage status for a given build artifact in a partner Google"]
        #[doc = " Cloud Storage bucket."]
        pub async fn check_build_stage_status(
            &mut self,
            request: impl tonic::IntoRequest<super::CheckBuildStageStatusRequest>,
        ) -> Result<tonic::Response<super::CheckBuildStageStatusResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.chromeos.moblab.v1beta1.BuildService/CheckBuildStageStatus",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Stages a given build artifact from a internal Google Cloud Storage bucket"]
        #[doc = " to a partner Google Cloud Storage bucket. If any of objects has already"]
        #[doc = " been copied, it will overwrite the previous objects. Operation <response:"]
        #[doc = " [StageBuildResponse][google.chromeos.moblab.v1beta1.StageBuildResponse],"]
        #[doc = "            metadata: [StageBuildMetadata][google.chromeos.moblab.v1beta1.StageBuildMetadata]>"]
        pub async fn stage_build(
            &mut self,
            request: impl tonic::IntoRequest<super::StageBuildRequest>,
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
                "/google.chromeos.moblab.v1beta1.BuildService/StageBuild",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Finds the most stable build for the given build target. The definition of"]
        #[doc = " the most stable build is determined by evaluating the following rules in"]
        #[doc = " order until one is true. If none are true, then there is no stable build"]
        #[doc = " and it will return an empty response."]
        #[doc = ""]
        #[doc = " Evaluation rules:"]
        #[doc = "   1. Stable channel build with label “Live”"]
        #[doc = "   2. Beta channel build with label “Live”"]
        #[doc = "   3. Dev channel build with label “Live”"]
        #[doc = "   4. Most recent stable channel build with build status Pass"]
        #[doc = "   5. Most recent beta channel build with build status Pass"]
        #[doc = "   6. Most recent dev channel build with build status Pass"]
        pub async fn find_most_stable_build(
            &mut self,
            request: impl tonic::IntoRequest<super::FindMostStableBuildRequest>,
        ) -> Result<tonic::Response<super::FindMostStableBuildResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.chromeos.moblab.v1beta1.BuildService/FindMostStableBuild",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
