/// CreateProfileRequest describes a profile resource online creation request.
/// The deployment field must be populated. The profile_type specifies the list
/// of profile types supported by the agent. The creation call will hang until a
/// profile of one of these types needs to be collected.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateProfileRequest {
    /// Parent project to create the profile in.
    #[prost(string, tag = "4")]
    pub parent: ::prost::alloc::string::String,
    /// Deployment details.
    #[prost(message, optional, tag = "1")]
    pub deployment: ::core::option::Option<Deployment>,
    /// One or more profile types that the agent is capable of providing.
    #[prost(enumeration = "ProfileType", repeated, tag = "2")]
    pub profile_type: ::prost::alloc::vec::Vec<i32>,
}
/// CreateOfflineProfileRequest describes a profile resource offline creation
/// request. Profile field must be set.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateOfflineProfileRequest {
    /// Parent project to create the profile in.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Contents of the profile to create.
    #[prost(message, optional, tag = "2")]
    pub profile: ::core::option::Option<Profile>,
}
/// UpdateProfileRequest contains the profile to update.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateProfileRequest {
    /// Profile to update
    #[prost(message, optional, tag = "1")]
    pub profile: ::core::option::Option<Profile>,
    /// Field mask used to specify the fields to be overwritten. Currently only
    /// profile_bytes and labels fields are supported by UpdateProfile, so only
    /// those fields can be specified in the mask. When no mask is provided, all
    /// fields are overwritten.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Profile resource.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Profile {
    /// Output only. Opaque, server-assigned, unique ID for this profile.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Type of profile.
    /// For offline mode, this must be specified when creating the profile. For
    /// online mode it is assigned and returned by the server.
    #[prost(enumeration = "ProfileType", tag = "2")]
    pub profile_type: i32,
    /// Deployment this profile corresponds to.
    #[prost(message, optional, tag = "3")]
    pub deployment: ::core::option::Option<Deployment>,
    /// Duration of the profiling session.
    /// Input (for the offline mode) or output (for the online mode).
    /// The field represents requested profiling duration. It may slightly differ
    /// from the effective profiling duration, which is recorded in the profile
    /// data, in case the profiling can't be stopped immediately (e.g. in case
    /// stopping the profiling is handled asynchronously).
    #[prost(message, optional, tag = "4")]
    pub duration: ::core::option::Option<::prost_types::Duration>,
    /// Input only. Profile bytes, as a gzip compressed serialized proto, the
    /// format is <https://github.com/google/pprof/blob/master/proto/profile.proto.>
    #[prost(bytes = "vec", tag = "5")]
    pub profile_bytes: ::prost::alloc::vec::Vec<u8>,
    /// Input only. Labels associated to this specific profile. These labels will
    /// get merged with the deployment labels for the final data set.  See
    /// documentation on deployment labels for validation rules and limits.
    #[prost(map = "string, string", tag = "6")]
    pub labels:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
}
/// Deployment contains the deployment identification information.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Deployment {
    /// Project ID is the ID of a cloud project.
    /// Validation regex: `^\[a-z][-a-z0-9:.]{4,61}[a-z0-9\]$`.
    #[prost(string, tag = "1")]
    pub project_id: ::prost::alloc::string::String,
    /// Target is the service name used to group related deployments:
    /// * Service name for GAE Flex / Standard.
    /// * Cluster and container name for GKE.
    /// * User-specified string for direct GCE profiling (e.g. Java).
    /// * Job name for Dataflow.
    /// Validation regex: `^\[a-z]([-a-z0-9_.]{0,253}[a-z0-9\])?$`.
    #[prost(string, tag = "2")]
    pub target: ::prost::alloc::string::String,
    /// Labels identify the deployment within the user universe and same target.
    /// Validation regex for label names: `^\[a-z0-9]([a-z0-9-]{0,61}[a-z0-9\])?$`.
    /// Value for an individual label must be <= 512 bytes, the total
    /// size of all label names and values must be <= 1024 bytes.
    ///
    /// Label named "language" can be used to record the programming language of
    /// the profiled deployment. The standard choices for the value include "java",
    /// "go", "python", "ruby", "nodejs", "php", "dotnet".
    ///
    /// For deployments running on Google Cloud Platform, "zone" or "region" label
    /// should be present describing the deployment location. An example of a zone
    /// is "us-central1-a", an example of a region is "us-central1" or
    /// "us-central".
    #[prost(map = "string, string", tag = "3")]
    pub labels:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
}
/// ProfileType is type of profiling data.
/// NOTE: the enumeration member names are used (in lowercase) as unique string
/// identifiers of profile types, so they must not be renamed.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ProfileType {
    /// Unspecified profile type.
    Unspecified = 0,
    /// Thread CPU time sampling.
    Cpu = 1,
    /// Wallclock time sampling. More expensive as stops all threads.
    Wall = 2,
    /// In-use heap profile. Represents a snapshot of the allocations that are
    /// live at the time of the profiling.
    Heap = 3,
    /// Single-shot collection of all thread stacks.
    Threads = 4,
    /// Synchronization contention profile.
    Contention = 5,
    /// Peak heap profile.
    PeakHeap = 6,
    /// Heap allocation profile. It represents the aggregation of all allocations
    /// made over the duration of the profile. All allocations are included,
    /// including those that might have been freed by the end of the profiling
    /// interval. The profile is in particular useful for garbage collecting
    /// languages to understand which parts of the code create most of the garbage
    /// collection pressure to see if those can be optimized.
    HeapAlloc = 7,
}
#[doc = r" Generated client implementations."]
pub mod profiler_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Manage the collection of continuous profiling data provided by profiling"]
    #[doc = " agents running in the cloud or by an offline provider of profiling data."]
    #[doc = ""]
    #[doc = " General guidelines:"]
    #[doc = " * Profiles for a single deployment must be created in ascending time order."]
    #[doc = " * Profiles can be created in either online or offline mode, see below."]
    #[derive(Debug, Clone)]
    pub struct ProfilerServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> ProfilerServiceClient<T>
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
        ) -> ProfilerServiceClient<InterceptedService<T, F>>
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
            ProfilerServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " CreateProfile creates a new profile resource in the online mode."]
        #[doc = ""]
        #[doc = " The server ensures that the new profiles are created at a constant rate per"]
        #[doc = " deployment, so the creation request may hang for some time until the next"]
        #[doc = " profile session is available."]
        #[doc = ""]
        #[doc = " The request may fail with ABORTED error if the creation is not available"]
        #[doc = " within ~1m, the response will indicate the duration of the backoff the"]
        #[doc = " client should take before attempting creating a profile again. The backoff"]
        #[doc = " duration is returned in google.rpc.RetryInfo extension on the response"]
        #[doc = " status. To a gRPC client, the extension will be return as a"]
        #[doc = " binary-serialized proto in the trailing metadata item named"]
        #[doc = " \"google.rpc.retryinfo-bin\"."]
        pub async fn create_profile(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateProfileRequest>,
        ) -> Result<tonic::Response<super::Profile>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.devtools.cloudprofiler.v2.ProfilerService/CreateProfile",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " CreateOfflineProfile creates a new profile resource in the offline mode."]
        #[doc = " The client provides the profile to create along with the profile bytes, the"]
        #[doc = " server records it."]
        pub async fn create_offline_profile(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateOfflineProfileRequest>,
        ) -> Result<tonic::Response<super::Profile>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.devtools.cloudprofiler.v2.ProfilerService/CreateOfflineProfile",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " UpdateProfile updates the profile bytes and labels on the profile resource"]
        #[doc = " created in the online mode. Updating the bytes for profiles created in the"]
        #[doc = " offline mode is currently not supported: the profile content must be"]
        #[doc = " provided at the time of the profile creation."]
        pub async fn update_profile(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateProfileRequest>,
        ) -> Result<tonic::Response<super::Profile>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.devtools.cloudprofiler.v2.ProfilerService/UpdateProfile",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
