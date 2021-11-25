/// The request sent to GetIapSettings.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetIapSettingsRequest {
    /// Required. The resource name for which to retrieve the settings.
    /// Authorization: Requires the `getSettings` permission for the associated
    /// resource.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// The request sent to UpdateIapSettings.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateIapSettingsRequest {
    /// Required. The new values for the IAP settings to be updated.
    /// Authorization: Requires the `updateSettings` permission for the associated
    /// resource.
    #[prost(message, optional, tag = "1")]
    pub iap_settings: ::core::option::Option<IapSettings>,
    /// The field mask specifying which IAP settings should be updated.
    /// If omitted, the all of the settings are updated. See
    /// <https://developers.google.com/protocol-buffers/docs/reference/google.protobuf#fieldmask>
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// The IAP configurable settings.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IapSettings {
    /// Required. The resource name of the IAP protected resource.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Top level wrapper for all access related setting in IAP
    #[prost(message, optional, tag = "5")]
    pub access_settings: ::core::option::Option<AccessSettings>,
    /// Top level wrapper for all application related settings in IAP
    #[prost(message, optional, tag = "6")]
    pub application_settings: ::core::option::Option<ApplicationSettings>,
}
/// Access related settings for IAP protected apps.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccessSettings {
    /// GCIP claims and endpoint configurations for 3p identity providers.
    #[prost(message, optional, tag = "1")]
    pub gcip_settings: ::core::option::Option<GcipSettings>,
    /// Configuration to allow cross-origin requests via IAP.
    #[prost(message, optional, tag = "2")]
    pub cors_settings: ::core::option::Option<CorsSettings>,
    /// Settings to configure IAP's OAuth behavior.
    #[prost(message, optional, tag = "3")]
    pub oauth_settings: ::core::option::Option<OAuthSettings>,
}
/// Allows customers to configure tenant_id for GCIP instance per-app.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GcipSettings {
    /// GCIP tenant ids that are linked to the IAP resource.
    /// tenant_ids could be a string beginning with a number character to indicate
    /// authenticating with GCIP tenant flow, or in the format of _<ProjectNumber>
    /// to indicate authenticating with GCIP agent flow.
    /// If agent flow is used, tenant_ids should only contain one single element,
    /// while for tenant flow, tenant_ids can contain multiple elements.
    #[prost(string, repeated, tag = "1")]
    pub tenant_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Login page URI associated with the GCIP tenants.
    /// Typically, all resources within the same project share the same login page,
    /// though it could be overridden at the sub resource level.
    #[prost(message, optional, tag = "2")]
    pub login_page_uri: ::core::option::Option<::prost::alloc::string::String>,
}
/// Allows customers to configure HTTP request paths that'll allow HTTP OPTIONS
/// call to bypass authentication and authorization.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CorsSettings {
    /// Configuration to allow HTTP OPTIONS calls to skip authorization. If
    /// undefined, IAP will not apply any special logic to OPTIONS requests.
    #[prost(message, optional, tag = "1")]
    pub allow_http_options: ::core::option::Option<bool>,
}
/// Configuration for OAuth login&consent flow behavior as well as for OAuth
/// Credentials.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OAuthSettings {
    /// Domain hint to send as hd=? parameter in OAuth request flow. Enables
    /// redirect to primary IDP by skipping Google's login screen.
    /// <https://developers.google.com/identity/protocols/OpenIDConnect#hd-param>
    /// Note: IAP does not verify that the id token's hd claim matches this value
    /// since access behavior is managed by IAM policies.
    #[prost(message, optional, tag = "2")]
    pub login_hint: ::core::option::Option<::prost::alloc::string::String>,
}
/// Wrapper over application specific settings for IAP.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApplicationSettings {
    /// Settings to configure IAP's behavior for a CSM mesh.
    #[prost(message, optional, tag = "1")]
    pub csm_settings: ::core::option::Option<CsmSettings>,
    /// Customization for Access Denied page.
    #[prost(message, optional, tag = "2")]
    pub access_denied_page_settings: ::core::option::Option<AccessDeniedPageSettings>,
    /// The Domain value to set for cookies generated by IAP. This value is not
    /// validated by the API, but will be ignored at runtime if invalid.
    #[prost(message, optional, tag = "3")]
    pub cookie_domain: ::core::option::Option<::prost::alloc::string::String>,
}
/// Configuration for RCTokens generated for CSM workloads protected by IAP.
/// RCTokens are IAP generated JWTs that can be verified at the application. The
/// RCToken is primarily used for ISTIO deployments, and can be scoped to a
/// single mesh by configuring the audience field accordingly
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CsmSettings {
    /// Audience claim set in the generated RCToken. This value is not validated by
    /// IAP.
    #[prost(message, optional, tag = "1")]
    pub rctoken_aud: ::core::option::Option<::prost::alloc::string::String>,
}
/// Custom content configuration for access denied page.
/// IAP allows customers to define a custom URI to use as the error page when
/// access is denied to users. If IAP prevents access to this page, the default
/// IAP error page will be displayed instead.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccessDeniedPageSettings {
    /// The URI to be redirected to when access is denied.
    #[prost(message, optional, tag = "1")]
    pub access_denied_page_uri: ::core::option::Option<::prost::alloc::string::String>,
    /// Whether to generate a troubleshooting URL on access denied events to this
    /// application.
    #[prost(message, optional, tag = "2")]
    pub generate_troubleshooting_uri: ::core::option::Option<bool>,
}
/// The request sent to ListBrands.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListBrandsRequest {
    /// Required. GCP Project number/id.
    /// In the following format: projects/{project_number/id}.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
}
/// Response message for ListBrands.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListBrandsResponse {
    /// Brands existing in the project.
    #[prost(message, repeated, tag = "1")]
    pub brands: ::prost::alloc::vec::Vec<Brand>,
}
/// The request sent to CreateBrand.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateBrandRequest {
    /// Required. GCP Project number/id under which the brand is to be created.
    /// In the following format: projects/{project_number/id}.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The brand to be created.
    #[prost(message, optional, tag = "2")]
    pub brand: ::core::option::Option<Brand>,
}
/// The request sent to GetBrand.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBrandRequest {
    /// Required. Name of the brand to be fetched.
    /// In the following format: projects/{project_number/id}/brands/{brand}.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// The request sent to ListIdentityAwareProxyClients.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListIdentityAwareProxyClientsRequest {
    /// Required. Full brand path.
    /// In the following format: projects/{project_number/id}/brands/{brand}.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of clients to return. The service may return fewer than
    /// this value.
    /// If unspecified, at most 100 clients will be returned.
    /// The maximum value is 1000; values above 1000 will be coerced to 1000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A page token, received from a previous `ListIdentityAwareProxyClients`
    /// call. Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to
    /// `ListIdentityAwareProxyClients` must match the call that provided the page
    /// token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for ListIdentityAwareProxyClients.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListIdentityAwareProxyClientsResponse {
    /// Clients existing in the brand.
    #[prost(message, repeated, tag = "1")]
    pub identity_aware_proxy_clients: ::prost::alloc::vec::Vec<IdentityAwareProxyClient>,
    /// A token, which can be send as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// The request sent to CreateIdentityAwareProxyClient.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateIdentityAwareProxyClientRequest {
    /// Required. Path to create the client in.
    /// In the following format:
    /// projects/{project_number/id}/brands/{brand}.
    /// The project must belong to a G Suite account.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. Identity Aware Proxy Client to be created.
    #[prost(message, optional, tag = "2")]
    pub identity_aware_proxy_client: ::core::option::Option<IdentityAwareProxyClient>,
}
/// The request sent to GetIdentityAwareProxyClient.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetIdentityAwareProxyClientRequest {
    /// Required. Name of the Identity Aware Proxy client to be fetched.
    /// In the following format:
    /// projects/{project_number/id}/brands/{brand}/identityAwareProxyClients/{client_id}.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// The request sent to ResetIdentityAwareProxyClientSecret.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResetIdentityAwareProxyClientSecretRequest {
    /// Required. Name of the Identity Aware Proxy client to that will have its
    /// secret reset. In the following format:
    /// projects/{project_number/id}/brands/{brand}/identityAwareProxyClients/{client_id}.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// The request sent to DeleteIdentityAwareProxyClient.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteIdentityAwareProxyClientRequest {
    /// Required. Name of the Identity Aware Proxy client to be deleted.
    /// In the following format:
    /// projects/{project_number/id}/brands/{brand}/identityAwareProxyClients/{client_id}.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// OAuth brand data.
/// NOTE: Only contains a portion of the data that describes a brand.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Brand {
    /// Output only. Identifier of the brand.
    /// NOTE: GCP project number achieves the same brand identification purpose as
    /// only one brand per project can be created.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Support email displayed on the OAuth consent screen.
    #[prost(string, tag = "2")]
    pub support_email: ::prost::alloc::string::String,
    /// Application name displayed on OAuth consent screen.
    #[prost(string, tag = "3")]
    pub application_title: ::prost::alloc::string::String,
    /// Output only. Whether the brand is only intended for usage inside the
    /// G Suite organization only.
    #[prost(bool, tag = "4")]
    pub org_internal_only: bool,
}
/// Contains the data that describes an Identity Aware Proxy owned client.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IdentityAwareProxyClient {
    /// Output only. Unique identifier of the OAuth client.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. Client secret of the OAuth client.
    #[prost(string, tag = "2")]
    pub secret: ::prost::alloc::string::String,
    /// Human-friendly name given to the OAuth client.
    #[prost(string, tag = "3")]
    pub display_name: ::prost::alloc::string::String,
}
#[doc = r" Generated client implementations."]
pub mod identity_aware_proxy_admin_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " APIs for Identity-Aware Proxy Admin configurations."]
    #[derive(Debug, Clone)]
    pub struct IdentityAwareProxyAdminServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> IdentityAwareProxyAdminServiceClient<T>
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
        ) -> IdentityAwareProxyAdminServiceClient<InterceptedService<T, F>>
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
            IdentityAwareProxyAdminServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Sets the access control policy for an Identity-Aware Proxy protected"]
        #[doc = " resource. Replaces any existing policy."]
        #[doc = " More information about managing access via IAP can be found at:"]
        #[doc = " https://cloud.google.com/iap/docs/managing-access#managing_access_via_the_api"]
        pub async fn set_iam_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::super::super::super::iam::v1::SetIamPolicyRequest>,
        ) -> Result<tonic::Response<super::super::super::super::iam::v1::Policy>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.iap.v1.IdentityAwareProxyAdminService/SetIamPolicy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets the access control policy for an Identity-Aware Proxy protected"]
        #[doc = " resource."]
        #[doc = " More information about managing access via IAP can be found at:"]
        #[doc = " https://cloud.google.com/iap/docs/managing-access#managing_access_via_the_api"]
        pub async fn get_iam_policy(
            &mut self,
            request: impl tonic::IntoRequest<super::super::super::super::iam::v1::GetIamPolicyRequest>,
        ) -> Result<tonic::Response<super::super::super::super::iam::v1::Policy>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.iap.v1.IdentityAwareProxyAdminService/GetIamPolicy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns permissions that a caller has on the Identity-Aware Proxy protected"]
        #[doc = " resource."]
        #[doc = " More information about managing access via IAP can be found at:"]
        #[doc = " https://cloud.google.com/iap/docs/managing-access#managing_access_via_the_api"]
        pub async fn test_iam_permissions(
            &mut self,
            request: impl tonic::IntoRequest<
                super::super::super::super::iam::v1::TestIamPermissionsRequest,
            >,
        ) -> Result<
            tonic::Response<super::super::super::super::iam::v1::TestIamPermissionsResponse>,
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
                "/google.cloud.iap.v1.IdentityAwareProxyAdminService/TestIamPermissions",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets the IAP settings on a particular IAP protected resource."]
        pub async fn get_iap_settings(
            &mut self,
            request: impl tonic::IntoRequest<super::GetIapSettingsRequest>,
        ) -> Result<tonic::Response<super::IapSettings>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.iap.v1.IdentityAwareProxyAdminService/GetIapSettings",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates the IAP settings on a particular IAP protected resource. It"]
        #[doc = " replaces all fields unless the `update_mask` is set."]
        pub async fn update_iap_settings(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateIapSettingsRequest>,
        ) -> Result<tonic::Response<super::IapSettings>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.iap.v1.IdentityAwareProxyAdminService/UpdateIapSettings",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
#[doc = r" Generated client implementations."]
pub mod identity_aware_proxy_o_auth_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " API to programmatically create, list and retrieve Identity Aware Proxy (IAP)"]
    #[doc = " OAuth brands; and create, retrieve, delete and reset-secret of IAP OAuth"]
    #[doc = " clients."]
    #[derive(Debug, Clone)]
    pub struct IdentityAwareProxyOAuthServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> IdentityAwareProxyOAuthServiceClient<T>
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
        ) -> IdentityAwareProxyOAuthServiceClient<InterceptedService<T, F>>
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
            IdentityAwareProxyOAuthServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Lists the existing brands for the project."]
        pub async fn list_brands(
            &mut self,
            request: impl tonic::IntoRequest<super::ListBrandsRequest>,
        ) -> Result<tonic::Response<super::ListBrandsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.iap.v1.IdentityAwareProxyOAuthService/ListBrands",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Constructs a new OAuth brand for the project if one does not exist."]
        #[doc = " The created brand is \"internal only\", meaning that OAuth clients created"]
        #[doc = " under it only accept requests from users who belong to the same G Suite"]
        #[doc = " organization as the project. The brand is created in an un-reviewed status."]
        #[doc = " NOTE: The \"internal only\" status can be manually changed in the Google"]
        #[doc = " Cloud console. Requires that a brand does not already exist for the"]
        #[doc = " project, and that the specified support email is owned by the caller."]
        pub async fn create_brand(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateBrandRequest>,
        ) -> Result<tonic::Response<super::Brand>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.iap.v1.IdentityAwareProxyOAuthService/CreateBrand",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Retrieves the OAuth brand of the project."]
        pub async fn get_brand(
            &mut self,
            request: impl tonic::IntoRequest<super::GetBrandRequest>,
        ) -> Result<tonic::Response<super::Brand>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.iap.v1.IdentityAwareProxyOAuthService/GetBrand",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates an Identity Aware Proxy (IAP) OAuth client. The client is owned"]
        #[doc = " by IAP. Requires that the brand for the project exists and that it is"]
        #[doc = " set for internal-only use."]
        pub async fn create_identity_aware_proxy_client(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateIdentityAwareProxyClientRequest>,
        ) -> Result<tonic::Response<super::IdentityAwareProxyClient>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ("/google.cloud.iap.v1.IdentityAwareProxyOAuthService/CreateIdentityAwareProxyClient") ;
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists the existing clients for the brand."]
        pub async fn list_identity_aware_proxy_clients(
            &mut self,
            request: impl tonic::IntoRequest<super::ListIdentityAwareProxyClientsRequest>,
        ) -> Result<tonic::Response<super::ListIdentityAwareProxyClientsResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.iap.v1.IdentityAwareProxyOAuthService/ListIdentityAwareProxyClients",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Retrieves an Identity Aware Proxy (IAP) OAuth client."]
        #[doc = " Requires that the client is owned by IAP."]
        pub async fn get_identity_aware_proxy_client(
            &mut self,
            request: impl tonic::IntoRequest<super::GetIdentityAwareProxyClientRequest>,
        ) -> Result<tonic::Response<super::IdentityAwareProxyClient>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.iap.v1.IdentityAwareProxyOAuthService/GetIdentityAwareProxyClient",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Resets an Identity Aware Proxy (IAP) OAuth client secret. Useful if the"]
        #[doc = " secret was compromised. Requires that the client is owned by IAP."]
        pub async fn reset_identity_aware_proxy_client_secret(
            &mut self,
            request: impl tonic::IntoRequest<super::ResetIdentityAwareProxyClientSecretRequest>,
        ) -> Result<tonic::Response<super::IdentityAwareProxyClient>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ("/google.cloud.iap.v1.IdentityAwareProxyOAuthService/ResetIdentityAwareProxyClientSecret") ;
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes an Identity Aware Proxy (IAP) OAuth client. Useful for removing"]
        #[doc = " obsolete clients, managing the number of clients in a given project, and"]
        #[doc = " cleaning up after tests. Requires that the client is owned by IAP."]
        pub async fn delete_identity_aware_proxy_client(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteIdentityAwareProxyClientRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ("/google.cloud.iap.v1.IdentityAwareProxyOAuthService/DeleteIdentityAwareProxyClient") ;
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
