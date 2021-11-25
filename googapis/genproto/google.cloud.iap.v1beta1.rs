#[doc = r" Generated client implementations."]
pub mod identity_aware_proxy_admin_v1_beta1_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " APIs for Identity-Aware Proxy Admin configurations."]
    #[derive(Debug, Clone)]
    pub struct IdentityAwareProxyAdminV1Beta1Client<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> IdentityAwareProxyAdminV1Beta1Client<T>
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
        ) -> IdentityAwareProxyAdminV1Beta1Client<InterceptedService<T, F>>
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
            IdentityAwareProxyAdminV1Beta1Client::new(InterceptedService::new(inner, interceptor))
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
                "/google.cloud.iap.v1beta1.IdentityAwareProxyAdminV1Beta1/SetIamPolicy",
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
                "/google.cloud.iap.v1beta1.IdentityAwareProxyAdminV1Beta1/GetIamPolicy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns permissions that a caller has on the Identity-Aware Proxy protected"]
        #[doc = " resource. If the resource does not exist or the caller does not have"]
        #[doc = " Identity-Aware Proxy permissions a [google.rpc.Code.PERMISSION_DENIED]"]
        #[doc = " will be returned."]
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
                "/google.cloud.iap.v1beta1.IdentityAwareProxyAdminV1Beta1/TestIamPermissions",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
