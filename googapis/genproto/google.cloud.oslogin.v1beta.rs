/// The user profile information used for logging in to a virtual machine on
/// Google Compute Engine.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LoginProfile {
    /// Required. A unique user ID.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The list of POSIX accounts associated with the user.
    #[prost(message, repeated, tag = "2")]
    pub posix_accounts: ::prost::alloc::vec::Vec<super::common::PosixAccount>,
    /// A map from SSH public key fingerprint to the associated key object.
    #[prost(map = "string, message", tag = "3")]
    pub ssh_public_keys:
        ::std::collections::HashMap<::prost::alloc::string::String, super::common::SshPublicKey>,
}
/// A request message for deleting a POSIX account entry.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeletePosixAccountRequest {
    /// Required. A reference to the POSIX account to update. POSIX accounts are identified
    /// by the project ID they are associated with. A reference to the POSIX
    /// account is in format `users/{user}/projects/{project}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// A request message for deleting an SSH public key.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteSshPublicKeyRequest {
    /// Required. The fingerprint of the public key to update. Public keys are identified by
    /// their SHA-256 fingerprint. The fingerprint of the public key is in format
    /// `users/{user}/sshPublicKeys/{fingerprint}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// A request message for retrieving the login profile information for a user.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLoginProfileRequest {
    /// Required. The unique ID for the user in format `users/{user}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The project ID of the Google Cloud Platform project.
    #[prost(string, tag = "2")]
    pub project_id: ::prost::alloc::string::String,
    /// A system ID for filtering the results of the request.
    #[prost(string, tag = "3")]
    pub system_id: ::prost::alloc::string::String,
}
/// A request message for retrieving an SSH public key.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSshPublicKeyRequest {
    /// Required. The fingerprint of the public key to retrieve. Public keys are identified
    /// by their SHA-256 fingerprint. The fingerprint of the public key is in
    /// format `users/{user}/sshPublicKeys/{fingerprint}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// A request message for importing an SSH public key.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportSshPublicKeyRequest {
    /// The unique ID for the user in format `users/{user}`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The SSH public key and expiration time.
    #[prost(message, optional, tag = "2")]
    pub ssh_public_key: ::core::option::Option<super::common::SshPublicKey>,
    /// The project ID of the Google Cloud Platform project.
    #[prost(string, tag = "3")]
    pub project_id: ::prost::alloc::string::String,
}
/// A response message for importing an SSH public key.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportSshPublicKeyResponse {
    /// The login profile information for the user.
    #[prost(message, optional, tag = "1")]
    pub login_profile: ::core::option::Option<LoginProfile>,
}
/// A request message for updating an SSH public key.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateSshPublicKeyRequest {
    /// Required. The fingerprint of the public key to update. Public keys are identified by
    /// their SHA-256 fingerprint. The fingerprint of the public key is in format
    /// `users/{user}/sshPublicKeys/{fingerprint}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The SSH public key and expiration time.
    #[prost(message, optional, tag = "2")]
    pub ssh_public_key: ::core::option::Option<super::common::SshPublicKey>,
    /// Mask to control which fields get updated. Updates all if not present.
    #[prost(message, optional, tag = "3")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
#[doc = r" Generated client implementations."]
pub mod os_login_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Cloud OS Login API"]
    #[doc = ""]
    #[doc = " The Cloud OS Login API allows you to manage users and their associated SSH"]
    #[doc = " public keys for logging into virtual machines on Google Cloud Platform."]
    #[derive(Debug, Clone)]
    pub struct OsLoginServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> OsLoginServiceClient<T>
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
        ) -> OsLoginServiceClient<InterceptedService<T, F>>
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
            OsLoginServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Deletes a POSIX account."]
        pub async fn delete_posix_account(
            &mut self,
            request: impl tonic::IntoRequest<super::DeletePosixAccountRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.oslogin.v1beta.OsLoginService/DeletePosixAccount",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes an SSH public key."]
        pub async fn delete_ssh_public_key(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteSshPublicKeyRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.oslogin.v1beta.OsLoginService/DeleteSshPublicKey",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Retrieves the profile information used for logging in to a virtual machine"]
        #[doc = " on Google Compute Engine."]
        pub async fn get_login_profile(
            &mut self,
            request: impl tonic::IntoRequest<super::GetLoginProfileRequest>,
        ) -> Result<tonic::Response<super::LoginProfile>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.oslogin.v1beta.OsLoginService/GetLoginProfile",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Retrieves an SSH public key."]
        pub async fn get_ssh_public_key(
            &mut self,
            request: impl tonic::IntoRequest<super::GetSshPublicKeyRequest>,
        ) -> Result<tonic::Response<super::super::common::SshPublicKey>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.oslogin.v1beta.OsLoginService/GetSshPublicKey",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Adds an SSH public key and returns the profile information. Default POSIX"]
        #[doc = " account information is set when no username and UID exist as part of the"]
        #[doc = " login profile."]
        pub async fn import_ssh_public_key(
            &mut self,
            request: impl tonic::IntoRequest<super::ImportSshPublicKeyRequest>,
        ) -> Result<tonic::Response<super::ImportSshPublicKeyResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.oslogin.v1beta.OsLoginService/ImportSshPublicKey",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates an SSH public key and returns the profile information. This method"]
        #[doc = " supports patch semantics."]
        pub async fn update_ssh_public_key(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateSshPublicKeyRequest>,
        ) -> Result<tonic::Response<super::super::common::SshPublicKey>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.oslogin.v1beta.OsLoginService/UpdateSshPublicKey",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
