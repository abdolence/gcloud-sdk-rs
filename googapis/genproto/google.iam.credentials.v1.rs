#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateAccessTokenRequest {
    /// Required. The resource name of the service account for which the credentials
    /// are requested, in the following format:
    /// `projects/-/serviceAccounts/{ACCOUNT_EMAIL_OR_UNIQUEID}`. The `-` wildcard
    /// character is required; replacing it with a project ID is invalid.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
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
    pub delegates: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Required. Code to identify the scopes to be included in the OAuth 2.0 access token.
    /// See <https://developers.google.com/identity/protocols/googlescopes> for more
    /// information.
    /// At least one value required.
    #[prost(string, repeated, tag = "4")]
    pub scope: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The desired lifetime duration of the access token in seconds.
    /// Must be set to a value less than or equal to 3600 (1 hour). If a value is
    /// not specified, the token's lifetime will be set to a default value of one
    /// hour.
    #[prost(message, optional, tag = "7")]
    pub lifetime: ::core::option::Option<::prost_types::Duration>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateAccessTokenResponse {
    /// The OAuth 2.0 access token.
    #[prost(string, tag = "1")]
    pub access_token: ::prost::alloc::string::String,
    /// Token expiration time.
    /// The expiration time is always set.
    #[prost(message, optional, tag = "3")]
    pub expire_time: ::core::option::Option<::prost_types::Timestamp>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignBlobRequest {
    /// Required. The resource name of the service account for which the credentials
    /// are requested, in the following format:
    /// `projects/-/serviceAccounts/{ACCOUNT_EMAIL_OR_UNIQUEID}`. The `-` wildcard
    /// character is required; replacing it with a project ID is invalid.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
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
    pub delegates: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Required. The bytes to sign.
    #[prost(bytes = "vec", tag = "5")]
    pub payload: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignBlobResponse {
    /// The ID of the key used to sign the blob.
    #[prost(string, tag = "1")]
    pub key_id: ::prost::alloc::string::String,
    /// The signed blob.
    #[prost(bytes = "vec", tag = "4")]
    pub signed_blob: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignJwtRequest {
    /// Required. The resource name of the service account for which the credentials
    /// are requested, in the following format:
    /// `projects/-/serviceAccounts/{ACCOUNT_EMAIL_OR_UNIQUEID}`. The `-` wildcard
    /// character is required; replacing it with a project ID is invalid.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
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
    pub delegates: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Required. The JWT payload to sign: a JSON object that contains a JWT Claims Set.
    #[prost(string, tag = "5")]
    pub payload: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignJwtResponse {
    /// The ID of the key used to sign the JWT.
    #[prost(string, tag = "1")]
    pub key_id: ::prost::alloc::string::String,
    /// The signed JWT.
    #[prost(string, tag = "2")]
    pub signed_jwt: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateIdTokenRequest {
    /// Required. The resource name of the service account for which the credentials
    /// are requested, in the following format:
    /// `projects/-/serviceAccounts/{ACCOUNT_EMAIL_OR_UNIQUEID}`. The `-` wildcard
    /// character is required; replacing it with a project ID is invalid.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
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
    pub delegates: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Required. The audience for the token, such as the API or account that this token
    /// grants access to.
    #[prost(string, tag = "3")]
    pub audience: ::prost::alloc::string::String,
    /// Include the service account email in the token. If set to `true`, the
    /// token will contain `email` and `email_verified` claims.
    #[prost(bool, tag = "4")]
    pub include_email: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateIdTokenResponse {
    /// The OpenId Connect ID token.
    #[prost(string, tag = "1")]
    pub token: ::prost::alloc::string::String,
}
#[doc = r" Generated client implementations."]
pub mod iam_credentials_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
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
    #[derive(Debug, Clone)]
    pub struct IamCredentialsClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> IamCredentialsClient<T>
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
        ) -> IamCredentialsClient<InterceptedService<T, F>>
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
            IamCredentialsClient::new(InterceptedService::new(inner, interceptor))
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
}
