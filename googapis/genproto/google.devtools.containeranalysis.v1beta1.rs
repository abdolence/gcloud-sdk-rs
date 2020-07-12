/// A scan configuration specifies whether Cloud components in a project have a
/// particular type of analysis being run. For example, it can configure whether
/// vulnerability scanning is being done on Docker images or not.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScanConfig {
    /// Output only. The name of the scan configuration in the form of
    /// `projects/[PROJECT_ID]/scanConfigs/[SCAN_CONFIG_ID]`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Output only. A human-readable description of what the scan configuration
    /// does.
    #[prost(string, tag = "2")]
    pub description: std::string::String,
    /// Whether the scan is enabled.
    #[prost(bool, tag = "3")]
    pub enabled: bool,
    /// Output only. The time this scan config was created.
    #[prost(message, optional, tag = "4")]
    pub create_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Output only. The time this scan config was last updated.
    #[prost(message, optional, tag = "5")]
    pub update_time: ::std::option::Option<::prost_types::Timestamp>,
}
/// Request to get a scan configuration.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetScanConfigRequest {
    /// Required. The name of the scan configuration in the form of
    /// `projects/[PROJECT_ID]/scanConfigs/[SCAN_CONFIG_ID]`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Request to list scan configurations.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListScanConfigsRequest {
    /// Required. The name of the project to list scan configurations for in the form of
    /// `projects/[PROJECT_ID]`.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Required. The filter expression.
    #[prost(string, tag = "2")]
    pub filter: std::string::String,
    /// The number of scan configs to return in the list.
    #[prost(int32, tag = "3")]
    pub page_size: i32,
    /// Token to provide to skip to a particular spot in the list.
    #[prost(string, tag = "4")]
    pub page_token: std::string::String,
}
/// Response for listing scan configurations.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListScanConfigsResponse {
    /// The scan configurations requested.
    #[prost(message, repeated, tag = "1")]
    pub scan_configs: ::std::vec::Vec<ScanConfig>,
    /// The next pagination token in the list response. It should be used as
    /// `page_token` for the following request. An empty value means no more
    /// results.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
}
/// A request to update a scan configuration.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateScanConfigRequest {
    /// Required. The name of the scan configuration in the form of
    /// `projects/[PROJECT_ID]/scanConfigs/[SCAN_CONFIG_ID]`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Required. The updated scan configuration.
    #[prost(message, optional, tag = "2")]
    pub scan_config: ::std::option::Option<ScanConfig>,
}
#[doc = r" Generated client implementations."]
pub mod container_analysis_v1_beta1_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Retrieves analysis results of Cloud components such as Docker container"]
    #[doc = " images. The Container Analysis API is an implementation of the"]
    #[doc = " [Grafeas](grafeas.io) API."]
    #[doc = ""]
    #[doc = " Analysis results are stored as a series of occurrences. An `Occurrence`"]
    #[doc = " contains information about a specific analysis instance on a resource. An"]
    #[doc = " occurrence refers to a `Note`. A note contains details describing the"]
    #[doc = " analysis and is generally stored in a separate project, called a `Provider`."]
    #[doc = " Multiple occurrences can refer to the same note."]
    #[doc = ""]
    #[doc = " For example, an SSL vulnerability could affect multiple images. In this case,"]
    #[doc = " there would be one note for the vulnerability and an occurrence for each"]
    #[doc = " image with the vulnerability referring to that note."]
    pub struct ContainerAnalysisV1Beta1Client<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> ContainerAnalysisV1Beta1Client<T>
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
        #[doc = " Sets the access control policy on the specified note or occurrence."]
        #[doc = " Requires `containeranalysis.notes.setIamPolicy` or"]
        #[doc = " `containeranalysis.occurrences.setIamPolicy` permission if the resource is"]
        #[doc = " a note or an occurrence, respectively."]
        #[doc = ""]
        #[doc = " The resource takes the format `projects/[PROJECT_ID]/notes/[NOTE_ID]` for"]
        #[doc = " notes and `projects/[PROJECT_ID]/occurrences/[OCCURRENCE_ID]` for"]
        #[doc = " occurrences."]
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
                "/google.devtools.containeranalysis.v1beta1.ContainerAnalysisV1Beta1/SetIamPolicy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets the access control policy for a note or an occurrence resource."]
        #[doc = " Requires `containeranalysis.notes.setIamPolicy` or"]
        #[doc = " `containeranalysis.occurrences.setIamPolicy` permission if the resource is"]
        #[doc = " a note or occurrence, respectively."]
        #[doc = ""]
        #[doc = " The resource takes the format `projects/[PROJECT_ID]/notes/[NOTE_ID]` for"]
        #[doc = " notes and `projects/[PROJECT_ID]/occurrences/[OCCURRENCE_ID]` for"]
        #[doc = " occurrences."]
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
                "/google.devtools.containeranalysis.v1beta1.ContainerAnalysisV1Beta1/GetIamPolicy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns the permissions that a caller has on the specified note or"]
        #[doc = " occurrence. Requires list permission on the project (for example,"]
        #[doc = " `containeranalysis.notes.list`)."]
        #[doc = ""]
        #[doc = " The resource takes the format `projects/[PROJECT_ID]/notes/[NOTE_ID]` for"]
        #[doc = " notes and `projects/[PROJECT_ID]/occurrences/[OCCURRENCE_ID]` for"]
        #[doc = " occurrences."]
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
            let path = http :: uri :: PathAndQuery :: from_static ( "/google.devtools.containeranalysis.v1beta1.ContainerAnalysisV1Beta1/TestIamPermissions" ) ;
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets the specified scan configuration."]
        pub async fn get_scan_config(
            &mut self,
            request: impl tonic::IntoRequest<super::GetScanConfigRequest>,
        ) -> Result<tonic::Response<super::ScanConfig>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.devtools.containeranalysis.v1beta1.ContainerAnalysisV1Beta1/GetScanConfig",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists scan configurations for the specified project."]
        pub async fn list_scan_configs(
            &mut self,
            request: impl tonic::IntoRequest<super::ListScanConfigsRequest>,
        ) -> Result<tonic::Response<super::ListScanConfigsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ( "/google.devtools.containeranalysis.v1beta1.ContainerAnalysisV1Beta1/ListScanConfigs" ) ;
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates the specified scan configuration."]
        pub async fn update_scan_config(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateScanConfigRequest>,
        ) -> Result<tonic::Response<super::ScanConfig>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ( "/google.devtools.containeranalysis.v1beta1.ContainerAnalysisV1Beta1/UpdateScanConfig" ) ;
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for ContainerAnalysisV1Beta1Client<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for ContainerAnalysisV1Beta1Client<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "ContainerAnalysisV1Beta1Client {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod container_analysis_v1_beta1_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with ContainerAnalysisV1Beta1Server."]
    #[async_trait]
    pub trait ContainerAnalysisV1Beta1: Send + Sync + 'static {
        #[doc = " Sets the access control policy on the specified note or occurrence."]
        #[doc = " Requires `containeranalysis.notes.setIamPolicy` or"]
        #[doc = " `containeranalysis.occurrences.setIamPolicy` permission if the resource is"]
        #[doc = " a note or an occurrence, respectively."]
        #[doc = ""]
        #[doc = " The resource takes the format `projects/[PROJECT_ID]/notes/[NOTE_ID]` for"]
        #[doc = " notes and `projects/[PROJECT_ID]/occurrences/[OCCURRENCE_ID]` for"]
        #[doc = " occurrences."]
        async fn set_iam_policy(
            &self,
            request: tonic::Request<super::super::super::super::iam::v1::SetIamPolicyRequest>,
        ) -> Result<tonic::Response<super::super::super::super::iam::v1::Policy>, tonic::Status>;
        #[doc = " Gets the access control policy for a note or an occurrence resource."]
        #[doc = " Requires `containeranalysis.notes.setIamPolicy` or"]
        #[doc = " `containeranalysis.occurrences.setIamPolicy` permission if the resource is"]
        #[doc = " a note or occurrence, respectively."]
        #[doc = ""]
        #[doc = " The resource takes the format `projects/[PROJECT_ID]/notes/[NOTE_ID]` for"]
        #[doc = " notes and `projects/[PROJECT_ID]/occurrences/[OCCURRENCE_ID]` for"]
        #[doc = " occurrences."]
        async fn get_iam_policy(
            &self,
            request: tonic::Request<super::super::super::super::iam::v1::GetIamPolicyRequest>,
        ) -> Result<tonic::Response<super::super::super::super::iam::v1::Policy>, tonic::Status>;
        #[doc = " Returns the permissions that a caller has on the specified note or"]
        #[doc = " occurrence. Requires list permission on the project (for example,"]
        #[doc = " `containeranalysis.notes.list`)."]
        #[doc = ""]
        #[doc = " The resource takes the format `projects/[PROJECT_ID]/notes/[NOTE_ID]` for"]
        #[doc = " notes and `projects/[PROJECT_ID]/occurrences/[OCCURRENCE_ID]` for"]
        #[doc = " occurrences."]
        async fn test_iam_permissions(
            &self,
            request: tonic::Request<super::super::super::super::iam::v1::TestIamPermissionsRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::iam::v1::TestIamPermissionsResponse>,
            tonic::Status,
        >;
        #[doc = " Gets the specified scan configuration."]
        async fn get_scan_config(
            &self,
            request: tonic::Request<super::GetScanConfigRequest>,
        ) -> Result<tonic::Response<super::ScanConfig>, tonic::Status>;
        #[doc = " Lists scan configurations for the specified project."]
        async fn list_scan_configs(
            &self,
            request: tonic::Request<super::ListScanConfigsRequest>,
        ) -> Result<tonic::Response<super::ListScanConfigsResponse>, tonic::Status>;
        #[doc = " Updates the specified scan configuration."]
        async fn update_scan_config(
            &self,
            request: tonic::Request<super::UpdateScanConfigRequest>,
        ) -> Result<tonic::Response<super::ScanConfig>, tonic::Status>;
    }
    #[doc = " Retrieves analysis results of Cloud components such as Docker container"]
    #[doc = " images. The Container Analysis API is an implementation of the"]
    #[doc = " [Grafeas](grafeas.io) API."]
    #[doc = ""]
    #[doc = " Analysis results are stored as a series of occurrences. An `Occurrence`"]
    #[doc = " contains information about a specific analysis instance on a resource. An"]
    #[doc = " occurrence refers to a `Note`. A note contains details describing the"]
    #[doc = " analysis and is generally stored in a separate project, called a `Provider`."]
    #[doc = " Multiple occurrences can refer to the same note."]
    #[doc = ""]
    #[doc = " For example, an SSL vulnerability could affect multiple images. In this case,"]
    #[doc = " there would be one note for the vulnerability and an occurrence for each"]
    #[doc = " image with the vulnerability referring to that note."]
    #[derive(Debug)]
    #[doc(hidden)]
    pub struct ContainerAnalysisV1Beta1Server<T: ContainerAnalysisV1Beta1> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: ContainerAnalysisV1Beta1> ContainerAnalysisV1Beta1Server<T> {
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
    impl<T, B> Service<http::Request<B>> for ContainerAnalysisV1Beta1Server<T>
    where
        T: ContainerAnalysisV1Beta1,
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
            match req . uri ( ) . path ( ) { "/google.devtools.containeranalysis.v1beta1.ContainerAnalysisV1Beta1/SetIamPolicy" => { # [ allow ( non_camel_case_types ) ] struct SetIamPolicySvc < T : ContainerAnalysisV1Beta1 > ( pub Arc < T > ) ; impl < T : ContainerAnalysisV1Beta1 > tonic :: server :: UnaryService < super :: super :: super :: super :: iam :: v1 :: SetIamPolicyRequest > for SetIamPolicySvc < T > { type Response = super :: super :: super :: super :: iam :: v1 :: Policy ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: super :: super :: super :: iam :: v1 :: SetIamPolicyRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { inner . set_iam_policy ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = SetIamPolicySvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } "/google.devtools.containeranalysis.v1beta1.ContainerAnalysisV1Beta1/GetIamPolicy" => { # [ allow ( non_camel_case_types ) ] struct GetIamPolicySvc < T : ContainerAnalysisV1Beta1 > ( pub Arc < T > ) ; impl < T : ContainerAnalysisV1Beta1 > tonic :: server :: UnaryService < super :: super :: super :: super :: iam :: v1 :: GetIamPolicyRequest > for GetIamPolicySvc < T > { type Response = super :: super :: super :: super :: iam :: v1 :: Policy ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: super :: super :: super :: iam :: v1 :: GetIamPolicyRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { inner . get_iam_policy ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = GetIamPolicySvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } "/google.devtools.containeranalysis.v1beta1.ContainerAnalysisV1Beta1/TestIamPermissions" => { # [ allow ( non_camel_case_types ) ] struct TestIamPermissionsSvc < T : ContainerAnalysisV1Beta1 > ( pub Arc < T > ) ; impl < T : ContainerAnalysisV1Beta1 > tonic :: server :: UnaryService < super :: super :: super :: super :: iam :: v1 :: TestIamPermissionsRequest > for TestIamPermissionsSvc < T > { type Response = super :: super :: super :: super :: iam :: v1 :: TestIamPermissionsResponse ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: super :: super :: super :: iam :: v1 :: TestIamPermissionsRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { inner . test_iam_permissions ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = TestIamPermissionsSvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } "/google.devtools.containeranalysis.v1beta1.ContainerAnalysisV1Beta1/GetScanConfig" => { # [ allow ( non_camel_case_types ) ] struct GetScanConfigSvc < T : ContainerAnalysisV1Beta1 > ( pub Arc < T > ) ; impl < T : ContainerAnalysisV1Beta1 > tonic :: server :: UnaryService < super :: GetScanConfigRequest > for GetScanConfigSvc < T > { type Response = super :: ScanConfig ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: GetScanConfigRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { inner . get_scan_config ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = GetScanConfigSvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } "/google.devtools.containeranalysis.v1beta1.ContainerAnalysisV1Beta1/ListScanConfigs" => { # [ allow ( non_camel_case_types ) ] struct ListScanConfigsSvc < T : ContainerAnalysisV1Beta1 > ( pub Arc < T > ) ; impl < T : ContainerAnalysisV1Beta1 > tonic :: server :: UnaryService < super :: ListScanConfigsRequest > for ListScanConfigsSvc < T > { type Response = super :: ListScanConfigsResponse ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: ListScanConfigsRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { inner . list_scan_configs ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = ListScanConfigsSvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } "/google.devtools.containeranalysis.v1beta1.ContainerAnalysisV1Beta1/UpdateScanConfig" => { # [ allow ( non_camel_case_types ) ] struct UpdateScanConfigSvc < T : ContainerAnalysisV1Beta1 > ( pub Arc < T > ) ; impl < T : ContainerAnalysisV1Beta1 > tonic :: server :: UnaryService < super :: UpdateScanConfigRequest > for UpdateScanConfigSvc < T > { type Response = super :: ScanConfig ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: UpdateScanConfigRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { inner . update_scan_config ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = UpdateScanConfigSvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } _ => Box :: pin ( async move { Ok ( http :: Response :: builder ( ) . status ( 200 ) . header ( "grpc-status" , "12" ) . body ( tonic :: body :: BoxBody :: empty ( ) ) . unwrap ( ) ) } ) , }
        }
    }
    impl<T: ContainerAnalysisV1Beta1> Clone for ContainerAnalysisV1Beta1Server<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: ContainerAnalysisV1Beta1> Clone for _Inner<T> {
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
