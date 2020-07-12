/// The user profile information used for logging in to a virtual machine on
/// Google Compute Engine.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LoginProfile {
    /// Required. A unique user ID.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// The list of POSIX accounts associated with the user.
    #[prost(message, repeated, tag = "2")]
    pub posix_accounts: ::std::vec::Vec<super::common::PosixAccount>,
    /// A map from SSH public key fingerprint to the associated key object.
    #[prost(map = "string, message", tag = "3")]
    pub ssh_public_keys:
        ::std::collections::HashMap<std::string::String, super::common::SshPublicKey>,
}
/// A request message for deleting a POSIX account entry.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeletePosixAccountRequest {
    /// Required. A reference to the POSIX account to update. POSIX accounts are identified
    /// by the project ID they are associated with. A reference to the POSIX
    /// account is in format `users/{user}/projects/{project}`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// A request message for deleting an SSH public key.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteSshPublicKeyRequest {
    /// Required. The fingerprint of the public key to update. Public keys are identified by
    /// their SHA-256 fingerprint. The fingerprint of the public key is in format
    /// `users/{user}/sshPublicKeys/{fingerprint}`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// A request message for retrieving the login profile information for a user.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLoginProfileRequest {
    /// Required. The unique ID for the user in format `users/{user}`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// The project ID of the Google Cloud Platform project.
    #[prost(string, tag = "2")]
    pub project_id: std::string::String,
    /// A system ID for filtering the results of the request.
    #[prost(string, tag = "3")]
    pub system_id: std::string::String,
}
/// A request message for retrieving an SSH public key.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSshPublicKeyRequest {
    /// Required. The fingerprint of the public key to retrieve. Public keys are identified
    /// by their SHA-256 fingerprint. The fingerprint of the public key is in
    /// format `users/{user}/sshPublicKeys/{fingerprint}`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// A request message for importing an SSH public key.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportSshPublicKeyRequest {
    /// Required. The unique ID for the user in format `users/{user}`.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Optional. The SSH public key and expiration time.
    #[prost(message, optional, tag = "2")]
    pub ssh_public_key: ::std::option::Option<super::common::SshPublicKey>,
    /// The project ID of the Google Cloud Platform project.
    #[prost(string, tag = "3")]
    pub project_id: std::string::String,
}
/// A response message for importing an SSH public key.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportSshPublicKeyResponse {
    /// The login profile information for the user.
    #[prost(message, optional, tag = "1")]
    pub login_profile: ::std::option::Option<LoginProfile>,
}
/// A request message for updating an SSH public key.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateSshPublicKeyRequest {
    /// Required. The fingerprint of the public key to update. Public keys are identified by
    /// their SHA-256 fingerprint. The fingerprint of the public key is in format
    /// `users/{user}/sshPublicKeys/{fingerprint}`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Required. The SSH public key and expiration time.
    #[prost(message, optional, tag = "2")]
    pub ssh_public_key: ::std::option::Option<super::common::SshPublicKey>,
    /// Mask to control which fields get updated. Updates all if not present.
    #[prost(message, optional, tag = "3")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
}
#[doc = r" Generated client implementations."]
pub mod os_login_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Cloud OS Login API"]
    #[doc = ""]
    #[doc = " The Cloud OS Login API allows you to manage users and their associated SSH"]
    #[doc = " public keys for logging into virtual machines on Google Cloud Platform."]
    pub struct OsLoginServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> OsLoginServiceClient<T>
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
                "/google.cloud.oslogin.v1.OsLoginService/DeletePosixAccount",
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
                "/google.cloud.oslogin.v1.OsLoginService/DeleteSshPublicKey",
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
                "/google.cloud.oslogin.v1.OsLoginService/GetLoginProfile",
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
                "/google.cloud.oslogin.v1.OsLoginService/GetSshPublicKey",
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
                "/google.cloud.oslogin.v1.OsLoginService/ImportSshPublicKey",
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
                "/google.cloud.oslogin.v1.OsLoginService/UpdateSshPublicKey",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for OsLoginServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for OsLoginServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "OsLoginServiceClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod os_login_service_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with OsLoginServiceServer."]
    #[async_trait]
    pub trait OsLoginService: Send + Sync + 'static {
        #[doc = " Deletes a POSIX account."]
        async fn delete_posix_account(
            &self,
            request: tonic::Request<super::DeletePosixAccountRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status>;
        #[doc = " Deletes an SSH public key."]
        async fn delete_ssh_public_key(
            &self,
            request: tonic::Request<super::DeleteSshPublicKeyRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status>;
        #[doc = " Retrieves the profile information used for logging in to a virtual machine"]
        #[doc = " on Google Compute Engine."]
        async fn get_login_profile(
            &self,
            request: tonic::Request<super::GetLoginProfileRequest>,
        ) -> Result<tonic::Response<super::LoginProfile>, tonic::Status>;
        #[doc = " Retrieves an SSH public key."]
        async fn get_ssh_public_key(
            &self,
            request: tonic::Request<super::GetSshPublicKeyRequest>,
        ) -> Result<tonic::Response<super::super::common::SshPublicKey>, tonic::Status>;
        #[doc = " Adds an SSH public key and returns the profile information. Default POSIX"]
        #[doc = " account information is set when no username and UID exist as part of the"]
        #[doc = " login profile."]
        async fn import_ssh_public_key(
            &self,
            request: tonic::Request<super::ImportSshPublicKeyRequest>,
        ) -> Result<tonic::Response<super::ImportSshPublicKeyResponse>, tonic::Status>;
        #[doc = " Updates an SSH public key and returns the profile information. This method"]
        #[doc = " supports patch semantics."]
        async fn update_ssh_public_key(
            &self,
            request: tonic::Request<super::UpdateSshPublicKeyRequest>,
        ) -> Result<tonic::Response<super::super::common::SshPublicKey>, tonic::Status>;
    }
    #[doc = " Cloud OS Login API"]
    #[doc = ""]
    #[doc = " The Cloud OS Login API allows you to manage users and their associated SSH"]
    #[doc = " public keys for logging into virtual machines on Google Cloud Platform."]
    #[derive(Debug)]
    #[doc(hidden)]
    pub struct OsLoginServiceServer<T: OsLoginService> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: OsLoginService> OsLoginServiceServer<T> {
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
    impl<T, B> Service<http::Request<B>> for OsLoginServiceServer<T>
    where
        T: OsLoginService,
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
                "/google.cloud.oslogin.v1.OsLoginService/DeletePosixAccount" => {
                    #[allow(non_camel_case_types)]
                    struct DeletePosixAccountSvc<T: OsLoginService>(pub Arc<T>);
                    impl<T: OsLoginService>
                        tonic::server::UnaryService<super::DeletePosixAccountRequest>
                        for DeletePosixAccountSvc<T>
                    {
                        type Response = ();
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeletePosixAccountRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.delete_posix_account(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = DeletePosixAccountSvc(inner);
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
                "/google.cloud.oslogin.v1.OsLoginService/DeleteSshPublicKey" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteSshPublicKeySvc<T: OsLoginService>(pub Arc<T>);
                    impl<T: OsLoginService>
                        tonic::server::UnaryService<super::DeleteSshPublicKeyRequest>
                        for DeleteSshPublicKeySvc<T>
                    {
                        type Response = ();
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteSshPublicKeyRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.delete_ssh_public_key(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = DeleteSshPublicKeySvc(inner);
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
                "/google.cloud.oslogin.v1.OsLoginService/GetLoginProfile" => {
                    #[allow(non_camel_case_types)]
                    struct GetLoginProfileSvc<T: OsLoginService>(pub Arc<T>);
                    impl<T: OsLoginService>
                        tonic::server::UnaryService<super::GetLoginProfileRequest>
                        for GetLoginProfileSvc<T>
                    {
                        type Response = super::LoginProfile;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetLoginProfileRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.get_login_profile(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetLoginProfileSvc(inner);
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
                "/google.cloud.oslogin.v1.OsLoginService/GetSshPublicKey" => {
                    #[allow(non_camel_case_types)]
                    struct GetSshPublicKeySvc<T: OsLoginService>(pub Arc<T>);
                    impl<T: OsLoginService>
                        tonic::server::UnaryService<super::GetSshPublicKeyRequest>
                        for GetSshPublicKeySvc<T>
                    {
                        type Response = super::super::common::SshPublicKey;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetSshPublicKeyRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.get_ssh_public_key(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetSshPublicKeySvc(inner);
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
                "/google.cloud.oslogin.v1.OsLoginService/ImportSshPublicKey" => {
                    #[allow(non_camel_case_types)]
                    struct ImportSshPublicKeySvc<T: OsLoginService>(pub Arc<T>);
                    impl<T: OsLoginService>
                        tonic::server::UnaryService<super::ImportSshPublicKeyRequest>
                        for ImportSshPublicKeySvc<T>
                    {
                        type Response = super::ImportSshPublicKeyResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ImportSshPublicKeyRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.import_ssh_public_key(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ImportSshPublicKeySvc(inner);
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
                "/google.cloud.oslogin.v1.OsLoginService/UpdateSshPublicKey" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateSshPublicKeySvc<T: OsLoginService>(pub Arc<T>);
                    impl<T: OsLoginService>
                        tonic::server::UnaryService<super::UpdateSshPublicKeyRequest>
                        for UpdateSshPublicKeySvc<T>
                    {
                        type Response = super::super::common::SshPublicKey;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateSshPublicKeyRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.update_ssh_public_key(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = UpdateSshPublicKeySvc(inner);
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
    impl<T: OsLoginService> Clone for OsLoginServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: OsLoginService> Clone for _Inner<T> {
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
