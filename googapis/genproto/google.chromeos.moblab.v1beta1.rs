/// Resource that represents a build target.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BuildTarget {
    /// The resource name of the build target.
    /// Format: buildTargets/{build_target}
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Resource that represents a model. Each model belongs to a build target. For
/// non-unified build, the model name is the same as its build target name.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Model {
    /// The resource name of the model.
    /// Format: buildTargets/{build_target}/models/{model}
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Resource that represents a chrome OS milestone.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Milestone {
    /// The resource name of the milestone.
    /// Format: milestones/{milestone}
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Resource that represents a build for the given build target, model, milestone
/// and build version.
/// NEXT_TAG: 4
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Build {
    /// The resource name of the build.
    /// Format: buildTargets/{build_target}/models/{model}/builds/{build}
    /// Example: buildTargets/octopus/models/bobba/builds/1234.0.0
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// The milestone that owns the build.
    /// Format: milestones/{milestone}
    #[prost(string, tag = "2")]
    pub milestone: std::string::String,
    /// The build version of the build, e.g. 1234.0.0.
    #[prost(string, tag = "3")]
    pub build_version: std::string::String,
}
/// Resource that represents a build artifact stored in Google Cloud Storage for
/// the given build target, model, build version and bucket. NEXT_TAG: 6
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BuildArtifact {
    /// The resource name of the build artifact.
    /// Format:
    /// buildTargets/{build_target}/models/{model}/builds/{build}/artifacts/{artifact}
    /// Example:
    /// buildTargets/octopus/models/bobba/builds/1234.0.0/artifacts/chromeos-moblab-peng-staging
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// The build metadata of the build artifact.
    #[prost(string, tag = "2")]
    pub build: std::string::String,
    /// The bucket that stores the build artifact.
    #[prost(string, tag = "3")]
    pub bucket: std::string::String,
    /// The path of the build artifact in the bucket.
    #[prost(string, tag = "4")]
    pub path: std::string::String,
    /// The number of objects in the build artifact folder. The object number can
    /// be used to calculated the stage progress by comparing the source build
    /// artifact with the destination build artifact.
    #[prost(uint32, tag = "5")]
    pub object_count: u32,
}
/// Request message for listing builds.
/// NEXT_TAG: 7
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListBuildsRequest {
    /// Required. The full resource name of the model. The model id is the same as
    /// the build target id for non-unified builds.
    /// For example,
    /// 'buildTargets/octopus/models/bobba'.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Optional. The number of builds to return in a page.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. A page token, received from a previous `ListBuilds` call. Provide this to
    /// retrieve the subsequent page.
    #[prost(string, tag = "3")]
    pub page_token: std::string::String,
    /// Optional. Filter that specifies value constraints of fields. For example, the
    /// filter can be set as "filter='milestone=milestones/80'" to only select
    /// builds in milestone 80.
    #[prost(string, tag = "4")]
    pub filter: std::string::String,
    /// Optional. Read mask that specifies which Build fields to return. If empty, all Build
    /// fields will be returned.
    /// Valid fields: name, milestone, build_version.
    /// For example, if the read_mask is set as "read_mask='milestone'", the
    /// ListBuilds will return a list of Builds object with only the milestone
    /// field.
    #[prost(message, optional, tag = "5")]
    pub read_mask: ::std::option::Option<::prost_types::FieldMask>,
    /// Optional. The operation that groups by all the Build fields specified in the read
    /// mask. The group_by field should be the same as the read_mask field in
    /// convention of SQL.
    #[prost(message, optional, tag = "6")]
    pub group_by: ::std::option::Option<::prost_types::FieldMask>,
}
/// Response message for listing builds.
/// NEXT_TAG: 4
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListBuildsResponse {
    /// The list of builds.
    #[prost(message, repeated, tag = "1")]
    pub builds: ::std::vec::Vec<Build>,
    /// Token to retrieve the next page of builds. If this field is omitted, there
    /// are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
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
    pub name: std::string::String,
}
/// Response message for checking the stage status of a build artifact.
/// NEXT_TAG: 4
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckBuildStageStatusResponse {
    /// The status to represent if the build is staged or not.
    #[prost(bool, tag = "1")]
    pub is_build_staged: bool,
    /// The staged build artifact in the destination bucket.
    #[prost(message, optional, tag = "2")]
    pub staged_build_artifact: ::std::option::Option<BuildArtifact>,
    /// The source build artifact in the source bucket.
    #[prost(message, optional, tag = "3")]
    pub source_build_artifact: ::std::option::Option<BuildArtifact>,
}
/// Request message for staging a build artifact.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StageBuildRequest {
    /// Required. The full resource name of the build artifact.
    /// For example,
    /// 'buildTargets/octopus/models/bobba/builds/12607.6.0/artifacts/chromeos-moblab-peng-staging'.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Response message for staging a build artifact.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StageBuildResponse {
    /// The staged build in the destination bucket.
    #[prost(message, optional, tag = "1")]
    pub staged_build_artifact: ::std::option::Option<BuildArtifact>,
}
/// Metadata message for staging a build artifact.
/// NEXT_TAG: 4
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StageBuildMetadata {
    /// Approximate percentage of progress, e.g. "50" means 50%.
    #[prost(float, tag = "1")]
    pub progress_percent: f32,
    /// Build stage start time.
    #[prost(message, optional, tag = "2")]
    pub start_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Build stage end time.
    #[prost(message, optional, tag = "3")]
    pub end_time: ::std::option::Option<::prost_types::Timestamp>,
}
#[doc = r" Generated client implementations."]
pub mod build_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Manages Chrome OS build services."]
    pub struct BuildServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> BuildServiceClient<T>
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
    }
    impl<T: Clone> Clone for BuildServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for BuildServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "BuildServiceClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod build_service_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with BuildServiceServer."]
    #[async_trait]
    pub trait BuildService: Send + Sync + 'static {
        #[doc = " Lists all builds for the given build target and model in descending order"]
        #[doc = " for the milestones and build versions."]
        async fn list_builds(
            &self,
            request: tonic::Request<super::ListBuildsRequest>,
        ) -> Result<tonic::Response<super::ListBuildsResponse>, tonic::Status>;
        #[doc = " Checks the stage status for a given build artifact in a partner Google"]
        #[doc = " Cloud Storage bucket."]
        async fn check_build_stage_status(
            &self,
            request: tonic::Request<super::CheckBuildStageStatusRequest>,
        ) -> Result<tonic::Response<super::CheckBuildStageStatusResponse>, tonic::Status>;
        #[doc = " Stages a given build artifact from a internal Google Cloud Storage bucket"]
        #[doc = " to a partner Google Cloud Storage bucket. If any of objects has already"]
        #[doc = " been copied, it will overwrite the previous objects. Operation <response:"]
        #[doc = " [StageBuildResponse][google.chromeos.moblab.v1beta1.StageBuildResponse],"]
        #[doc = "            metadata: [StageBuildMetadata][google.chromeos.moblab.v1beta1.StageBuildMetadata]>"]
        async fn stage_build(
            &self,
            request: tonic::Request<super::StageBuildRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        >;
    }
    #[doc = " Manages Chrome OS build services."]
    #[derive(Debug)]
    pub struct BuildServiceServer<T: BuildService> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: BuildService> BuildServiceServer<T> {
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
    impl<T, B> Service<http::Request<B>> for BuildServiceServer<T>
    where
        T: BuildService,
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
                "/google.chromeos.moblab.v1beta1.BuildService/ListBuilds" => {
                    #[allow(non_camel_case_types)]
                    struct ListBuildsSvc<T: BuildService>(pub Arc<T>);
                    impl<T: BuildService> tonic::server::UnaryService<super::ListBuildsRequest> for ListBuildsSvc<T> {
                        type Response = super::ListBuildsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListBuildsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).list_builds(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ListBuildsSvc(inner);
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
                "/google.chromeos.moblab.v1beta1.BuildService/CheckBuildStageStatus" => {
                    #[allow(non_camel_case_types)]
                    struct CheckBuildStageStatusSvc<T: BuildService>(pub Arc<T>);
                    impl<T: BuildService>
                        tonic::server::UnaryService<super::CheckBuildStageStatusRequest>
                        for CheckBuildStageStatusSvc<T>
                    {
                        type Response = super::CheckBuildStageStatusResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CheckBuildStageStatusRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).check_build_stage_status(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = CheckBuildStageStatusSvc(inner);
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
                "/google.chromeos.moblab.v1beta1.BuildService/StageBuild" => {
                    #[allow(non_camel_case_types)]
                    struct StageBuildSvc<T: BuildService>(pub Arc<T>);
                    impl<T: BuildService> tonic::server::UnaryService<super::StageBuildRequest> for StageBuildSvc<T> {
                        type Response = super::super::super::super::longrunning::Operation;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::StageBuildRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).stage_build(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = StageBuildSvc(inner);
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
    impl<T: BuildService> Clone for BuildServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: BuildService> Clone for _Inner<T> {
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
