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
#[doc = r" Generated server implementations."]
pub mod phishing_protection_service_v1_beta1_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with PhishingProtectionServiceV1Beta1Server."]
    #[async_trait]
    pub trait PhishingProtectionServiceV1Beta1: Send + Sync + 'static {
        #[doc = " Reports a URI suspected of containing phishing content to be reviewed. Once"]
        #[doc = " the report review is complete, its result can be found in the Cloud"]
        #[doc = " Security Command Center findings dashboard for Phishing Protection. If the"]
        #[doc = " result verifies the existence of malicious phishing content, the site will"]
        #[doc = " be added the to [Google's Social Engineering"]
        #[doc = " lists](https://support.google.com/webmasters/answer/6350487/) in order to"]
        #[doc = " protect users that could get exposed to this threat in the future."]
        async fn report_phishing(
            &self,
            request: tonic::Request<super::ReportPhishingRequest>,
        ) -> Result<tonic::Response<super::ReportPhishingResponse>, tonic::Status>;
    }
    #[doc = " Service to report phishing URIs."]
    #[derive(Debug)]
    #[doc(hidden)]
    pub struct PhishingProtectionServiceV1Beta1Server<T: PhishingProtectionServiceV1Beta1> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: PhishingProtectionServiceV1Beta1> PhishingProtectionServiceV1Beta1Server<T> {
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
    impl<T, B> Service<http::Request<B>> for PhishingProtectionServiceV1Beta1Server<T>
    where
        T: PhishingProtectionServiceV1Beta1,
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
            match req . uri ( ) . path ( ) { "/google.cloud.phishingprotection.v1beta1.PhishingProtectionServiceV1Beta1/ReportPhishing" => { # [ allow ( non_camel_case_types ) ] struct ReportPhishingSvc < T : PhishingProtectionServiceV1Beta1 > ( pub Arc < T > ) ; impl < T : PhishingProtectionServiceV1Beta1 > tonic :: server :: UnaryService < super :: ReportPhishingRequest > for ReportPhishingSvc < T > { type Response = super :: ReportPhishingResponse ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: ReportPhishingRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { inner . report_phishing ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = ReportPhishingSvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } _ => Box :: pin ( async move { Ok ( http :: Response :: builder ( ) . status ( 200 ) . header ( "grpc-status" , "12" ) . body ( tonic :: body :: BoxBody :: empty ( ) ) . unwrap ( ) ) } ) , }
        }
    }
    impl<T: PhishingProtectionServiceV1Beta1> Clone for PhishingProtectionServiceV1Beta1Server<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: PhishingProtectionServiceV1Beta1> Clone for _Inner<T> {
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
