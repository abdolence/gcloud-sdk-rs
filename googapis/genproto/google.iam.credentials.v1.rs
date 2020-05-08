#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateAccessTokenRequest {
    /// Required. The resource name of the service account for which the credentials
    /// are requested, in the following format:
    /// `projects/-/serviceAccounts/{ACCOUNT_EMAIL_OR_UNIQUEID}`. The `-` wildcard
    /// character is required; replacing it with a project ID is invalid.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// The sequence of service accounts in a delegation chain. Each service
    /// account must be granted the `roles/iam.serviceAccountTokenCreator` role
    /// on its next service account in the chain. The last service account in the
    /// chain must be granted the `roles/iam.serviceAccountTokenCreator` role
    /// on the service account that is specified in the `name` field of the
    /// request.
    ///
    /// The delegates must have the following format:
    /// `projects/-/serviceAccounts/{ACCOUNT_EMAIL_OR_UNIQUEID}`. The `-` wildcard
    /// character is required; replacing it with a project ID is invalid.
    #[prost(string, repeated, tag = "2")]
    pub delegates: ::std::vec::Vec<std::string::String>,
    /// Required. Code to identify the scopes to be included in the OAuth 2.0 access token.
    /// See https://developers.google.com/identity/protocols/googlescopes for more
    /// information.
    /// At least one value required.
    #[prost(string, repeated, tag = "4")]
    pub scope: ::std::vec::Vec<std::string::String>,
    /// The desired lifetime duration of the access token in seconds.
    /// Must be set to a value less than or equal to 3600 (1 hour). If a value is
    /// not specified, the token's lifetime will be set to a default value of one
    /// hour.
    #[prost(message, optional, tag = "7")]
    pub lifetime: ::std::option::Option<::prost_types::Duration>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateAccessTokenResponse {
    /// The OAuth 2.0 access token.
    #[prost(string, tag = "1")]
    pub access_token: std::string::String,
    /// Token expiration time.
    /// The expiration time is always set.
    #[prost(message, optional, tag = "3")]
    pub expire_time: ::std::option::Option<::prost_types::Timestamp>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignBlobRequest {
    /// Required. The resource name of the service account for which the credentials
    /// are requested, in the following format:
    /// `projects/-/serviceAccounts/{ACCOUNT_EMAIL_OR_UNIQUEID}`. The `-` wildcard
    /// character is required; replacing it with a project ID is invalid.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// The sequence of service accounts in a delegation chain. Each service
    /// account must be granted the `roles/iam.serviceAccountTokenCreator` role
    /// on its next service account in the chain. The last service account in the
    /// chain must be granted the `roles/iam.serviceAccountTokenCreator` role
    /// on the service account that is specified in the `name` field of the
    /// request.
    ///
    /// The delegates must have the following format:
    /// `projects/-/serviceAccounts/{ACCOUNT_EMAIL_OR_UNIQUEID}`. The `-` wildcard
    /// character is required; replacing it with a project ID is invalid.
    #[prost(string, repeated, tag = "3")]
    pub delegates: ::std::vec::Vec<std::string::String>,
    /// Required. The bytes to sign.
    #[prost(bytes, tag = "5")]
    pub payload: std::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignBlobResponse {
    /// The ID of the key used to sign the blob.
    #[prost(string, tag = "1")]
    pub key_id: std::string::String,
    /// The signed blob.
    #[prost(bytes, tag = "4")]
    pub signed_blob: std::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignJwtRequest {
    /// Required. The resource name of the service account for which the credentials
    /// are requested, in the following format:
    /// `projects/-/serviceAccounts/{ACCOUNT_EMAIL_OR_UNIQUEID}`. The `-` wildcard
    /// character is required; replacing it with a project ID is invalid.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// The sequence of service accounts in a delegation chain. Each service
    /// account must be granted the `roles/iam.serviceAccountTokenCreator` role
    /// on its next service account in the chain. The last service account in the
    /// chain must be granted the `roles/iam.serviceAccountTokenCreator` role
    /// on the service account that is specified in the `name` field of the
    /// request.
    ///
    /// The delegates must have the following format:
    /// `projects/-/serviceAccounts/{ACCOUNT_EMAIL_OR_UNIQUEID}`. The `-` wildcard
    /// character is required; replacing it with a project ID is invalid.
    #[prost(string, repeated, tag = "3")]
    pub delegates: ::std::vec::Vec<std::string::String>,
    /// Required. The JWT payload to sign: a JSON object that contains a JWT Claims Set.
    #[prost(string, tag = "5")]
    pub payload: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignJwtResponse {
    /// The ID of the key used to sign the JWT.
    #[prost(string, tag = "1")]
    pub key_id: std::string::String,
    /// The signed JWT.
    #[prost(string, tag = "2")]
    pub signed_jwt: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateIdTokenRequest {
    /// Required. The resource name of the service account for which the credentials
    /// are requested, in the following format:
    /// `projects/-/serviceAccounts/{ACCOUNT_EMAIL_OR_UNIQUEID}`. The `-` wildcard
    /// character is required; replacing it with a project ID is invalid.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// The sequence of service accounts in a delegation chain. Each service
    /// account must be granted the `roles/iam.serviceAccountTokenCreator` role
    /// on its next service account in the chain. The last service account in the
    /// chain must be granted the `roles/iam.serviceAccountTokenCreator` role
    /// on the service account that is specified in the `name` field of the
    /// request.
    ///
    /// The delegates must have the following format:
    /// `projects/-/serviceAccounts/{ACCOUNT_EMAIL_OR_UNIQUEID}`. The `-` wildcard
    /// character is required; replacing it with a project ID is invalid.
    #[prost(string, repeated, tag = "2")]
    pub delegates: ::std::vec::Vec<std::string::String>,
    /// Required. The audience for the token, such as the API or account that this token
    /// grants access to.
    #[prost(string, tag = "3")]
    pub audience: std::string::String,
    /// Include the service account email in the token. If set to `true`, the
    /// token will contain `email` and `email_verified` claims.
    #[prost(bool, tag = "4")]
    pub include_email: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateIdTokenResponse {
    /// The OpenId Connect ID token.
    #[prost(string, tag = "1")]
    pub token: std::string::String,
}
#[doc = r" Generated client implementations."]
pub mod iam_credentials_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " A service account is a special type of Google account that belongs to your"]
    #[doc = " application or a virtual machine (VM), instead of to an individual end user."]
    #[doc = " Your application assumes the identity of the service account to call Google"]
    #[doc = " APIs, so that the users aren't directly involved."]
    #[doc = ""]
    #[doc = " Service account credentials are used to temporarily assume the identity"]
    #[doc = " of the service account. Supported credential types include OAuth 2.0 access"]
    #[doc = " tokens, OpenID Connect ID tokens, self-signed JSON Web Tokens (JWTs), and"]
    #[doc = " more."]
    pub struct IamCredentialsClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl IamCredentialsClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> IamCredentialsClient<T>
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
        #[doc = " Generates an OAuth 2.0 access token for a service account."]
        pub async fn generate_access_token(
            &mut self,
            request: impl tonic::IntoRequest<super::GenerateAccessTokenRequest>,
        ) -> Result<tonic::Response<super::GenerateAccessTokenResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.iam.credentials.v1.IAMCredentials/GenerateAccessToken",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Generates an OpenID Connect ID token for a service account."]
        pub async fn generate_id_token(
            &mut self,
            request: impl tonic::IntoRequest<super::GenerateIdTokenRequest>,
        ) -> Result<tonic::Response<super::GenerateIdTokenResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.iam.credentials.v1.IAMCredentials/GenerateIdToken",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Signs a blob using a service account's system-managed private key."]
        pub async fn sign_blob(
            &mut self,
            request: impl tonic::IntoRequest<super::SignBlobRequest>,
        ) -> Result<tonic::Response<super::SignBlobResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.iam.credentials.v1.IAMCredentials/SignBlob",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Signs a JWT using a service account's system-managed private key."]
        pub async fn sign_jwt(
            &mut self,
            request: impl tonic::IntoRequest<super::SignJwtRequest>,
        ) -> Result<tonic::Response<super::SignJwtResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.iam.credentials.v1.IAMCredentials/SignJwt",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for IamCredentialsClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for IamCredentialsClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "IamCredentialsClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod iam_credentials_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with IamCredentialsServer."]
    #[async_trait]
    pub trait IamCredentials: Send + Sync + 'static {
        #[doc = " Generates an OAuth 2.0 access token for a service account."]
        async fn generate_access_token(
            &self,
            request: tonic::Request<super::GenerateAccessTokenRequest>,
        ) -> Result<tonic::Response<super::GenerateAccessTokenResponse>, tonic::Status>;
        #[doc = " Generates an OpenID Connect ID token for a service account."]
        async fn generate_id_token(
            &self,
            request: tonic::Request<super::GenerateIdTokenRequest>,
        ) -> Result<tonic::Response<super::GenerateIdTokenResponse>, tonic::Status>;
        #[doc = " Signs a blob using a service account's system-managed private key."]
        async fn sign_blob(
            &self,
            request: tonic::Request<super::SignBlobRequest>,
        ) -> Result<tonic::Response<super::SignBlobResponse>, tonic::Status>;
        #[doc = " Signs a JWT using a service account's system-managed private key."]
        async fn sign_jwt(
            &self,
            request: tonic::Request<super::SignJwtRequest>,
        ) -> Result<tonic::Response<super::SignJwtResponse>, tonic::Status>;
    }
    #[doc = " A service account is a special type of Google account that belongs to your"]
    #[doc = " application or a virtual machine (VM), instead of to an individual end user."]
    #[doc = " Your application assumes the identity of the service account to call Google"]
    #[doc = " APIs, so that the users aren't directly involved."]
    #[doc = ""]
    #[doc = " Service account credentials are used to temporarily assume the identity"]
    #[doc = " of the service account. Supported credential types include OAuth 2.0 access"]
    #[doc = " tokens, OpenID Connect ID tokens, self-signed JSON Web Tokens (JWTs), and"]
    #[doc = " more."]
    #[derive(Debug)]
    #[doc(hidden)]
    pub struct IamCredentialsServer<T: IamCredentials> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: IamCredentials> IamCredentialsServer<T> {
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
    impl<T, B> Service<http::Request<B>> for IamCredentialsServer<T>
    where
        T: IamCredentials,
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
                "/google.iam.credentials.v1.IAMCredentials/GenerateAccessToken" => {
                    #[allow(non_camel_case_types)]
                    struct GenerateAccessTokenSvc<T: IamCredentials>(pub Arc<T>);
                    impl<T: IamCredentials>
                        tonic::server::UnaryService<super::GenerateAccessTokenRequest>
                        for GenerateAccessTokenSvc<T>
                    {
                        type Response = super::GenerateAccessTokenResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GenerateAccessTokenRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.generate_access_token(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GenerateAccessTokenSvc(inner);
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
                "/google.iam.credentials.v1.IAMCredentials/GenerateIdToken" => {
                    #[allow(non_camel_case_types)]
                    struct GenerateIdTokenSvc<T: IamCredentials>(pub Arc<T>);
                    impl<T: IamCredentials>
                        tonic::server::UnaryService<super::GenerateIdTokenRequest>
                        for GenerateIdTokenSvc<T>
                    {
                        type Response = super::GenerateIdTokenResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GenerateIdTokenRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.generate_id_token(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GenerateIdTokenSvc(inner);
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
                "/google.iam.credentials.v1.IAMCredentials/SignBlob" => {
                    #[allow(non_camel_case_types)]
                    struct SignBlobSvc<T: IamCredentials>(pub Arc<T>);
                    impl<T: IamCredentials> tonic::server::UnaryService<super::SignBlobRequest> for SignBlobSvc<T> {
                        type Response = super::SignBlobResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SignBlobRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.sign_blob(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = SignBlobSvc(inner);
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
                "/google.iam.credentials.v1.IAMCredentials/SignJwt" => {
                    #[allow(non_camel_case_types)]
                    struct SignJwtSvc<T: IamCredentials>(pub Arc<T>);
                    impl<T: IamCredentials> tonic::server::UnaryService<super::SignJwtRequest> for SignJwtSvc<T> {
                        type Response = super::SignJwtResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SignJwtRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.sign_jwt(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = SignJwtSvc(inner);
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
    impl<T: IamCredentials> Clone for IamCredentialsServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: IamCredentials> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: IamCredentials> tonic::transport::NamedService for IamCredentialsServer<T> {
        const NAME: &'static str = "google.iam.credentials.v1.IAMCredentials";
    }
}
