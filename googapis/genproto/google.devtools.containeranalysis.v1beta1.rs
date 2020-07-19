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
