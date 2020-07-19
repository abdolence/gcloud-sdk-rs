/// The ReportPhishing request message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReportPhishingRequest {
    /// Required. The name of the project for which the report will be created,
    /// in the format "projects/{project_number}".
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Required. The URI that is being reported for phishing content to be analyzed.
    #[prost(string, tag = "2")]
    pub uri: std::string::String,
}
/// The ReportPhishing (empty) response message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReportPhishingResponse {}
#[doc = r" Generated client implementations."]
pub mod phishing_protection_service_v1_beta1_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Service to report phishing URIs."]
    pub struct PhishingProtectionServiceV1Beta1Client<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> PhishingProtectionServiceV1Beta1Client<T>
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
            let path = http :: uri :: PathAndQuery :: from_static ( "/google.cloud.phishingprotection.v1beta1.PhishingProtectionServiceV1Beta1/ReportPhishing" ) ;
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for PhishingProtectionServiceV1Beta1Client<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for PhishingProtectionServiceV1Beta1Client<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "PhishingProtectionServiceV1Beta1Client {{ ... }}")
        }
    }
}
