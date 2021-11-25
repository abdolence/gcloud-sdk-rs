/// DockerImage represents a docker artifact.
/// The following fields are returned as untyped metadata in the Version
/// resource, using camelcase keys (i.e. metadata.imageSizeBytes):
/// * imageSizeBytes
/// * mediaType
/// * buildTime
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DockerImage {
    /// Required. registry_location, project_id, repository_name and image id forms a unique
    /// image
    /// name:`projects/<project_id>/locations/<location>/repository/<repository_name>/dockerImages/<docker_image>`.
    /// For example,
    /// "projects/test-project/locations/us-west4/repositories/test-repo/dockerImages/
    /// nginx@sha256:e9954c1fc875017be1c3e36eca16be2d9e9bccc4bf072163515467d6a823c7cf",
    /// where "us-west4" is the registry_location, "test-project" is the
    /// project_id, "test-repo" is the repository_name and
    /// "nginx@sha256:e9954c1fc875017be1c3e36eca16be2d9e9bccc4bf072163515467d6a823c7cf"
    /// is the image's digest.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. URL to access the image.
    /// Example:
    /// us-west4-docker.pkg.dev/test-project/test-repo/nginx@sha256:e9954c1fc875017be1c3e36eca16be2d9e9bccc4bf072163515467d6a823c7cf
    #[prost(string, tag = "2")]
    pub uri: ::prost::alloc::string::String,
    /// Tags attached to this image.
    #[prost(string, repeated, tag = "3")]
    pub tags: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Calculated size of the image.
    /// This field is returned as the 'metadata.imageSizeBytes' field in the
    /// Version resource.
    #[prost(int64, tag = "4")]
    pub image_size_bytes: i64,
    /// Time the image was uploaded.
    #[prost(message, optional, tag = "5")]
    pub upload_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Media type of this image, e.g.
    /// "application/vnd.docker.distribution.manifest.v2+json".
    /// This field is returned as the 'metadata.mediaType' field in the
    /// Version resource.
    #[prost(string, tag = "6")]
    pub media_type: ::prost::alloc::string::String,
    /// The time this image was built.
    /// This field is returned as the 'metadata.buildTime' field in the
    /// Version resource.
    /// The build time is returned to the client as an RFC 3339 string, which can
    /// be easily used with the JavaScript Date constructor.
    #[prost(message, optional, tag = "7")]
    pub build_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// The request to list docker images.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDockerImagesRequest {
    /// Required. The name of the parent resource whose docker images will be listed.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of artifacts to return.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The next_page_token value returned from a previous list request, if any.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// The response from listing docker images.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDockerImagesResponse {
    /// The docker images returned.
    #[prost(message, repeated, tag = "1")]
    pub docker_images: ::prost::alloc::vec::Vec<DockerImage>,
    /// The token to retrieve the next page of artifacts, or empty if there are no
    /// more artifacts to return.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// A Repository for storing artifacts with a specific format.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Repository {
    /// The name of the repository, for example:
    /// "projects/p1/locations/us-central1/repositories/repo1".
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The format of packages that are stored in the repository.
    #[prost(enumeration = "repository::Format", tag = "2")]
    pub format: i32,
    /// The user-provided description of the repository.
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    /// Labels with user-defined metadata.
    /// This field may contain up to 64 entries. Label keys and values may be no
    /// longer than 63 characters. Label keys must begin with a lowercase letter
    /// and may only contain lowercase letters, numeric characters, underscores,
    /// and dashes.
    #[prost(map = "string, string", tag = "4")]
    pub labels:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    /// The time when the repository was created.
    #[prost(message, optional, tag = "5")]
    pub create_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The time when the repository was last updated.
    #[prost(message, optional, tag = "6")]
    pub update_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The Cloud KMS resource name of the customer managed encryption key that’s
    /// used to encrypt the contents of the Repository. Has the form:
    /// `projects/my-project/locations/my-region/keyRings/my-kr/cryptoKeys/my-key`.
    /// This value may not be changed after the Repository has been created.
    #[prost(string, tag = "8")]
    pub kms_key_name: ::prost::alloc::string::String,
}
/// Nested message and enum types in `Repository`.
pub mod repository {
    /// A package format.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Format {
        /// Unspecified package format.
        Unspecified = 0,
        /// Docker package format.
        Docker = 1,
        /// Maven package format.
        Maven = 2,
        /// NPM package format.
        Npm = 3,
        /// APT package format.
        Apt = 5,
        /// YUM package format.
        Yum = 6,
        /// Python package format.
        Python = 8,
    }
}
/// The request to list repositories.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListRepositoriesRequest {
    /// Required. The name of the parent resource whose repositories will be listed.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of repositories to return.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The next_page_token value returned from a previous list request, if any.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// The response from listing repositories.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListRepositoriesResponse {
    /// The repositories returned.
    #[prost(message, repeated, tag = "1")]
    pub repositories: ::prost::alloc::vec::Vec<Repository>,
    /// The token to retrieve the next page of repositories, or empty if there are
    /// no more repositories to return.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// The request to retrieve a repository.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRepositoryRequest {
    /// Required. The name of the repository to retrieve.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
#[doc = r" Generated client implementations."]
pub mod artifact_registry_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " The Artifact Registry API service."]
    #[doc = ""]
    #[doc = " Artifact Registry is an artifact management system for storing artifacts"]
    #[doc = " from different package management systems."]
    #[doc = ""]
    #[doc = " The resources managed by this API are:"]
    #[doc = ""]
    #[doc = " * Repositories, which group packages and their data."]
    #[doc = " * Packages, which group versions and their tags."]
    #[doc = " * Versions, which are specific forms of a package."]
    #[doc = " * Tags, which represent alternative names for versions."]
    #[doc = " * Files, which contain content and are optionally associated with a Package"]
    #[doc = "   or Version."]
    #[derive(Debug, Clone)]
    pub struct ArtifactRegistryClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> ArtifactRegistryClient<T>
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
        ) -> ArtifactRegistryClient<InterceptedService<T, F>>
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
            ArtifactRegistryClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Lists docker images."]
        pub async fn list_docker_images(
            &mut self,
            request: impl tonic::IntoRequest<super::ListDockerImagesRequest>,
        ) -> Result<tonic::Response<super::ListDockerImagesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.devtools.artifactregistry.v1.ArtifactRegistry/ListDockerImages",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists repositories."]
        pub async fn list_repositories(
            &mut self,
            request: impl tonic::IntoRequest<super::ListRepositoriesRequest>,
        ) -> Result<tonic::Response<super::ListRepositoriesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.devtools.artifactregistry.v1.ArtifactRegistry/ListRepositories",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets a repository."]
        pub async fn get_repository(
            &mut self,
            request: impl tonic::IntoRequest<super::GetRepositoryRequest>,
        ) -> Result<tonic::Response<super::Repository>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.devtools.artifactregistry.v1.ArtifactRegistry/GetRepository",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
