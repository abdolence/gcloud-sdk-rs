/// A Cloud Shell environment, which is defined as the combination of a Docker
/// image specifying what is installed on the environment and a home directory
/// containing the user's data that will remain across sessions. Each user has
/// at least an environment with the ID "default".
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Environment {
    /// Immutable. Full name of this resource, in the format
    /// `users/{owner_email}/environments/{environment_id}`. `{owner_email}` is the
    /// email address of the user to whom this environment belongs, and
    /// `{environment_id}` is the identifier of this environment. For example,
    /// `users/someone@example.com/environments/default`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. The environment's identifier, unique among the user's
    /// environments.
    #[prost(string, tag = "2")]
    pub id: ::prost::alloc::string::String,
    /// Required. Immutable. Full path to the Docker image used to run this environment, e.g.
    /// "gcr.io/dev-con/cloud-devshell:latest".
    #[prost(string, tag = "3")]
    pub docker_image: ::prost::alloc::string::String,
    /// Output only. Current execution state of this environment.
    #[prost(enumeration = "environment::State", tag = "4")]
    pub state: i32,
    /// Output only. Host to which clients can connect to initiate HTTPS or WSS
    /// connections with the environment.
    #[prost(string, tag = "12")]
    pub web_host: ::prost::alloc::string::String,
    /// Output only. Username that clients should use when initiating SSH sessions
    /// with the environment.
    #[prost(string, tag = "5")]
    pub ssh_username: ::prost::alloc::string::String,
    /// Output only. Host to which clients can connect to initiate SSH sessions
    /// with the environment.
    #[prost(string, tag = "6")]
    pub ssh_host: ::prost::alloc::string::String,
    /// Output only. Port to which clients can connect to initiate SSH sessions
    /// with the environment.
    #[prost(int32, tag = "7")]
    pub ssh_port: i32,
    /// Output only. Public keys associated with the environment. Clients can
    /// connect to this environment via SSH only if they possess a private key
    /// corresponding to at least one of these public keys. Keys can be added to or
    /// removed from the environment using the AddPublicKey and RemovePublicKey
    /// methods.
    #[prost(string, repeated, tag = "8")]
    pub public_keys: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Nested message and enum types in `Environment`.
pub mod environment {
    /// Possible execution states for an environment.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// The environment's states is unknown.
        Unspecified = 0,
        /// The environment is not running and can't be connected to. Starting the
        /// environment will transition it to the PENDING state.
        Suspended = 1,
        /// The environment is being started but is not yet ready to accept
        /// connections.
        Pending = 2,
        /// The environment is running and ready to accept connections. It will
        /// automatically transition back to DISABLED after a period of inactivity or
        /// if another environment is started.
        Running = 3,
        /// The environment is being deleted and can't be connected to.
        Deleting = 4,
    }
}
/// Request message for
/// \[GetEnvironment][google.cloud.shell.v1.CloudShellService.GetEnvironment\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetEnvironmentRequest {
    /// Required. Name of the requested resource, for example `users/me/environments/default`
    /// or `users/someone@example.com/environments/default`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Message included in the metadata field of operations returned from
/// \[CreateEnvironment][google.cloud.shell.v1.CloudShellService.CreateEnvironment\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateEnvironmentMetadata {}
/// Message included in the metadata field of operations returned from
/// \[DeleteEnvironment][google.cloud.shell.v1.CloudShellService.DeleteEnvironment\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteEnvironmentMetadata {}
/// Request message for
/// \[StartEnvironment][google.cloud.shell.v1.CloudShellService.StartEnvironment\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StartEnvironmentRequest {
    /// Name of the resource that should be started, for example
    /// `users/me/environments/default` or
    /// `users/someone@example.com/environments/default`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The initial access token passed to the environment. If this is present and
    /// valid, the environment will be pre-authenticated with gcloud so that the
    /// user can run gcloud commands in Cloud Shell without having to log in. This
    /// code can be updated later by calling AuthorizeEnvironment.
    #[prost(string, tag = "2")]
    pub access_token: ::prost::alloc::string::String,
    /// Public keys that should be added to the environment before it is started.
    #[prost(string, repeated, tag = "3")]
    pub public_keys: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Request message for
/// \[AuthorizeEnvironment][google.cloud.shell.v1.CloudShellService.AuthorizeEnvironment\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthorizeEnvironmentRequest {
    /// Name of the resource that should receive the credentials, for example
    /// `users/me/environments/default` or
    /// `users/someone@example.com/environments/default`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The OAuth access token that should be sent to the environment.
    #[prost(string, tag = "2")]
    pub access_token: ::prost::alloc::string::String,
    /// The OAuth ID token that should be sent to the environment.
    #[prost(string, tag = "4")]
    pub id_token: ::prost::alloc::string::String,
    /// The time when the credentials expire. If not set, defaults to one hour from
    /// when the server received the request.
    #[prost(message, optional, tag = "3")]
    pub expire_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Response message for
/// \[AuthorizeEnvironment][google.cloud.shell.v1.CloudShellService.AuthorizeEnvironment\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthorizeEnvironmentResponse {}
/// Message included in the metadata field of operations returned from
/// \[AuthorizeEnvironment][google.cloud.shell.v1.CloudShellService.AuthorizeEnvironment\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthorizeEnvironmentMetadata {}
/// Message included in the metadata field of operations returned from
/// \[StartEnvironment][google.cloud.shell.v1.CloudShellService.StartEnvironment\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StartEnvironmentMetadata {
    /// Current state of the environment being started.
    #[prost(enumeration = "start_environment_metadata::State", tag = "1")]
    pub state: i32,
}
/// Nested message and enum types in `StartEnvironmentMetadata`.
pub mod start_environment_metadata {
    /// Possible states an environment might transition between during startup.
    /// These states are not normally actionable by clients, but may be used to
    /// show a progress message to the user. An environment won't necessarily go
    /// through all of these states when starting. More states are likely to be
    /// added in the future.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// The environment's start state is unknown.
        Unspecified = 0,
        /// The environment is in the process of being started, but no additional
        /// details are available.
        Starting = 1,
        /// Startup is waiting for the user's disk to be unarchived. This can happen
        /// when the user returns to Cloud Shell after not having used it for a
        /// while, and suggests that startup will take longer than normal.
        UnarchivingDisk = 2,
        /// Startup is waiting for compute resources to be assigned to the
        /// environment. This should normally happen very quickly, but an environment
        /// might stay in this state for an extended period of time if the system is
        /// experiencing heavy load.
        AwaitingComputeResources = 4,
        /// Startup has completed. If the start operation was successful, the user
        /// should be able to establish an SSH connection to their environment.
        /// Otherwise, the operation will contain details of the failure.
        Finished = 3,
    }
}
/// Message included in the response field of operations returned from
/// \[StartEnvironment][google.cloud.shell.v1.CloudShellService.StartEnvironment\]
/// once the operation is complete.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StartEnvironmentResponse {
    /// Environment that was started.
    #[prost(message, optional, tag = "1")]
    pub environment: ::core::option::Option<Environment>,
}
/// Request message for
/// \[AddPublicKey][google.cloud.shell.v1.CloudShellService.AddPublicKey\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddPublicKeyRequest {
    /// Environment this key should be added to, e.g.
    /// `users/me/environments/default`.
    #[prost(string, tag = "1")]
    pub environment: ::prost::alloc::string::String,
    /// Key that should be added to the environment. Supported formats are
    /// `ssh-dss` (see RFC4253), `ssh-rsa` (see RFC4253), `ecdsa-sha2-nistp256`
    /// (see RFC5656), `ecdsa-sha2-nistp384` (see RFC5656) and
    /// `ecdsa-sha2-nistp521` (see RFC5656). It should be structured as
    /// &lt;format&gt; &lt;content&gt;, where &lt;content&gt; part is encoded with
    /// Base64.
    #[prost(string, tag = "2")]
    pub key: ::prost::alloc::string::String,
}
/// Response message for
/// \[AddPublicKey][google.cloud.shell.v1.CloudShellService.AddPublicKey\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddPublicKeyResponse {
    /// Key that was added to the environment.
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
}
/// Message included in the metadata field of operations returned from
/// \[AddPublicKey][google.cloud.shell.v1.CloudShellService.AddPublicKey\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddPublicKeyMetadata {}
/// Request message for
/// \[RemovePublicKey][google.cloud.shell.v1.CloudShellService.RemovePublicKey\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemovePublicKeyRequest {
    /// Environment this key should be removed from, e.g.
    /// `users/me/environments/default`.
    #[prost(string, tag = "1")]
    pub environment: ::prost::alloc::string::String,
    /// Key that should be removed from the environment.
    #[prost(string, tag = "2")]
    pub key: ::prost::alloc::string::String,
}
/// Response message for
/// \[RemovePublicKey][google.cloud.shell.v1.CloudShellService.RemovePublicKey\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemovePublicKeyResponse {}
/// Message included in the metadata field of operations returned from
/// \[RemovePublicKey][google.cloud.shell.v1.CloudShellService.RemovePublicKey\].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemovePublicKeyMetadata {}
/// Cloud-shell specific information that will be included as details in failure
/// responses.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CloudShellErrorDetails {
    /// Code indicating the specific error the occurred.
    #[prost(
        enumeration = "cloud_shell_error_details::CloudShellErrorCode",
        tag = "1"
    )]
    pub code: i32,
}
/// Nested message and enum types in `CloudShellErrorDetails`.
pub mod cloud_shell_error_details {
    /// Set of possible errors returned from API calls.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum CloudShellErrorCode {
        /// An unknown error occurred.
        Unspecified = 0,
        /// The image used by the Cloud Shell environment either does not exist or
        /// the user does not have access to it.
        ImageUnavailable = 1,
        /// Cloud Shell has been disabled by an administrator for the user making the
        /// request.
        CloudShellDisabled = 2,
        /// Cloud Shell has been permanently disabled due to a Terms of Service
        /// violation by the user.
        TosViolation = 4,
        /// The user has exhausted their weekly Cloud Shell quota, and Cloud Shell
        /// will be disabled until the quota resets.
        QuotaExceeded = 5,
    }
}
#[doc = r" Generated client implementations."]
pub mod cloud_shell_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " API for interacting with Google Cloud Shell. Each user of Cloud Shell has at"]
    #[doc = " least one environment, which has the ID \"default\". Environment consists of a"]
    #[doc = " Docker image defining what is installed on the environment and a home"]
    #[doc = " directory containing the user's data that will remain across sessions."]
    #[doc = " Clients use this API to start and fetch information about their environment,"]
    #[doc = " which can then be used to connect to that environment via a separate SSH"]
    #[doc = " client."]
    #[derive(Debug, Clone)]
    pub struct CloudShellServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> CloudShellServiceClient<T>
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
        ) -> CloudShellServiceClient<InterceptedService<T, F>>
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
            CloudShellServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Gets an environment. Returns NOT_FOUND if the environment does not exist."]
        pub async fn get_environment(
            &mut self,
            request: impl tonic::IntoRequest<super::GetEnvironmentRequest>,
        ) -> Result<tonic::Response<super::Environment>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.shell.v1.CloudShellService/GetEnvironment",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Starts an existing environment, allowing clients to connect to it. The"]
        #[doc = " returned operation will contain an instance of StartEnvironmentMetadata in"]
        #[doc = " its metadata field. Users can wait for the environment to start by polling"]
        #[doc = " this operation via GetOperation. Once the environment has finished starting"]
        #[doc = " and is ready to accept connections, the operation will contain a"]
        #[doc = " StartEnvironmentResponse in its response field."]
        pub async fn start_environment(
            &mut self,
            request: impl tonic::IntoRequest<super::StartEnvironmentRequest>,
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
                "/google.cloud.shell.v1.CloudShellService/StartEnvironment",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Sends OAuth credentials to a running environment on behalf of a user. When"]
        #[doc = " this completes, the environment will be authorized to run various Google"]
        #[doc = " Cloud command line tools without requiring the user to manually"]
        #[doc = " authenticate."]
        pub async fn authorize_environment(
            &mut self,
            request: impl tonic::IntoRequest<super::AuthorizeEnvironmentRequest>,
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
                "/google.cloud.shell.v1.CloudShellService/AuthorizeEnvironment",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Adds a public SSH key to an environment, allowing clients with the"]
        #[doc = " corresponding private key to connect to that environment via SSH. If a key"]
        #[doc = " with the same content already exists, this will error with ALREADY_EXISTS."]
        pub async fn add_public_key(
            &mut self,
            request: impl tonic::IntoRequest<super::AddPublicKeyRequest>,
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
                "/google.cloud.shell.v1.CloudShellService/AddPublicKey",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Removes a public SSH key from an environment. Clients will no longer be"]
        #[doc = " able to connect to the environment using the corresponding private key."]
        #[doc = " If a key with the same content is not present, this will error with"]
        #[doc = " NOT_FOUND."]
        pub async fn remove_public_key(
            &mut self,
            request: impl tonic::IntoRequest<super::RemovePublicKeyRequest>,
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
                "/google.cloud.shell.v1.CloudShellService/RemovePublicKey",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
