/// The ReportPhishing request message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReportPhishingRequest {
    /// Required. The name of the project for which the report will be created,
    /// in the format "projects/{project_number}".
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The URI that is being reported for phishing content to be analyzed.
    #[prost(string, tag = "2")]
    pub uri: ::prost::alloc::string::String,
}
/// The ReportPhishing (empty) response message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReportPhishingResponse {}
#[doc = r" Generated client implementations."]
pub mod phishing_protection_service_v1_beta1_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Service to report phishing URIs."]
    #[derive(Debug, Clone)]
    pub struct PhishingProtectionServiceV1Beta1Client<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> PhishingProtectionServiceV1Beta1Client<T>
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
        ) -> PhishingProtectionServiceV1Beta1Client<InterceptedService<T, F>>
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
            PhishingProtectionServiceV1Beta1Client::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Reports a URI suspected of containing phishing content to be reviewed. Once"]
        #[doc = " the report review is complete, its result can be found in the Cloud"]
        #[doc = " Security Command Center findings dashboard for Phishing Protection. If the"]
        #[doc = " result verifies the existence of malicious phishing content, the site will"]
        #[doc = " be added the to [Google's Social Engineering"]
        #[doc = " lists](https://support.google.com/webmasters/answer/6350487/) in order to"]
        #[doc = " protect users that could get exposed to this threat in the future."]
        pub async fn report_phishing(
            &mut self,
            request: impl tonic::IntoRequest<super::ReportPhishingRequest>,
        ) -> Result<tonic::Response<super::ReportPhishingResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ("/google.cloud.phishingprotection.v1beta1.PhishingProtectionServiceV1Beta1/ReportPhishing") ;
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
