/// Request message to get Google Workspace Add-ons authorization information.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAuthorizationRequest {
    /// Required. Name of the project for which to get the Google Workspace Add-ons
    /// authorization information.
    ///
    /// Example: `projects/my_project/authorization`.
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
}
/// The authorization information used when invoking deployment endpoints.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Authorization {
    /// The canonical full name of this resource.
    /// Example:  `projects/123/authorization`
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The email address of the service account used to authenticate requests to
    /// add-on callback endpoints.
    #[prost(string, tag = "2")]
    pub service_account_email: ::prost::alloc::string::String,
    /// The OAuth client ID used to obtain OAuth access tokens for a user on the
    /// add-on's behalf.
    #[prost(string, tag = "3")]
    pub oauth_client_id: ::prost::alloc::string::String,
}
/// Request message to create a deployment.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateDeploymentRequest {
    /// Required. Name of the project in which to create the deployment.
    ///
    /// Example: `projects/my_project`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. The id to use for this deployment.  The full name of the created
    /// resource will be `projects/<project_number>/deployments/<deployment_id>`.
    #[prost(string, tag = "2")]
    pub deployment_id: ::prost::alloc::string::String,
    /// Required. The deployment to create (deployment.name cannot be set).
    #[prost(message, optional, tag = "3")]
    pub deployment: ::core::option::Option<Deployment>,
}
/// Request message to create or replace a deployment.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReplaceDeploymentRequest {
    /// Required. The deployment to create or replace.
    #[prost(message, optional, tag = "2")]
    pub deployment: ::core::option::Option<Deployment>,
}
/// Request message to get a deployment.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDeploymentRequest {
    /// Required. The full resource name of the deployment to get.
    ///
    /// Example:  `projects/my_project/deployments/my_deployment`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message to list deployments for a project.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDeploymentsRequest {
    /// Required. Name of the project in which to create the deployment.
    ///
    /// Example: `projects/my_project`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// The maximum number of deployments to return. The service may return fewer
    /// than this value.
    /// If unspecified, at most 1000 deployments will be returned.
    /// The maximum value is 1000; values above 1000 will be coerced to 1000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A page token, received from a previous `ListDeployments` call.
    /// Provide this to retrieve the subsequent page.
    ///
    /// When paginating, all other parameters provided to `ListDeployments` must
    /// match the call that provided the page token.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message to list deployments.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDeploymentsResponse {
    /// The list of deployments for the given project.
    #[prost(message, repeated, tag = "1")]
    pub deployments: ::prost::alloc::vec::Vec<Deployment>,
    /// A token, which can be sent as `page_token` to retrieve the next page.
    /// If this field is omitted, there are no subsequent pages.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message to delete a deployment.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteDeploymentRequest {
    /// Required. The full resource name of the deployment to delete.
    ///
    /// Example:  `projects/my_project/deployments/my_deployment`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The etag of the deployment to delete.
    /// If this is provided, it must match the server's etag.
    #[prost(string, tag = "2")]
    pub etag: ::prost::alloc::string::String,
}
/// Request message to install a developer mode deployment.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InstallDeploymentRequest {
    /// Required. The full resource name of the deployment to install.
    ///
    /// Example:  `projects/my_project/deployments/my_deployment`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message to uninstall a developer mode deployment.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UninstallDeploymentRequest {
    /// Required. The full resource name of the deployment to install.
    ///
    /// Example:  `projects/my_project/deployments/my_deployment`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message to get the install status of a developer mode deployment.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetInstallStatusRequest {
    /// Required. The full resource name of the deployment.
    ///
    /// Example:  `projects/my_project/deployments/my_deployment/installStatus`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Developer mode install status of a deployment
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InstallStatus {
    /// The canonical full resource name of the deployment install status.
    ///
    /// Example:  `projects/123/deployments/my_deployment/installStatus`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// True if the deployment is installed for the user
    #[prost(message, optional, tag = "2")]
    pub installed: ::core::option::Option<bool>,
}
/// A Google Workspace Add-on deployment
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Deployment {
    /// The deployment resource name.
    /// Example:  projects/123/deployments/my_deployment.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The list of Google OAuth scopes for which to request consent from the end
    /// user before executing an add-on endpoint.
    #[prost(string, repeated, tag = "2")]
    pub oauth_scopes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// The Google Workspace Add-on configuration.
    #[prost(message, optional, tag = "3")]
    pub add_ons: ::core::option::Option<AddOns>,
    /// This value is computed by the server based on the version of the
    /// deployment in storage, and may be sent on update and delete requests to
    /// ensure the client has an up-to-date value before proceeding.
    #[prost(string, tag = "5")]
    pub etag: ::prost::alloc::string::String,
}
/// A Google Workspace Add-on configuration.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddOns {
    /// Configuration that is common across all Google Workspace Add-ons.
    #[prost(message, optional, tag = "1")]
    pub common:
        ::core::option::Option<super::super::super::apps::script::r#type::CommonAddOnManifest>,
    /// Gmail add-on configuration.
    #[prost(message, optional, tag = "2")]
    pub gmail: ::core::option::Option<
        super::super::super::apps::script::r#type::gmail::GmailAddOnManifest,
    >,
    /// Drive add-on configuration.
    #[prost(message, optional, tag = "5")]
    pub drive: ::core::option::Option<
        super::super::super::apps::script::r#type::drive::DriveAddOnManifest,
    >,
    /// Calendar add-on configuration.
    #[prost(message, optional, tag = "6")]
    pub calendar: ::core::option::Option<
        super::super::super::apps::script::r#type::calendar::CalendarAddOnManifest,
    >,
    /// Docs add-on configuration.
    #[prost(message, optional, tag = "7")]
    pub docs:
        ::core::option::Option<super::super::super::apps::script::r#type::docs::DocsAddOnManifest>,
    /// Sheets add-on configuration.
    #[prost(message, optional, tag = "8")]
    pub sheets: ::core::option::Option<
        super::super::super::apps::script::r#type::sheets::SheetsAddOnManifest,
    >,
    /// Slides add-on configuration.
    #[prost(message, optional, tag = "10")]
    pub slides: ::core::option::Option<
        super::super::super::apps::script::r#type::slides::SlidesAddOnManifest,
    >,
    /// Options for sending requests to add-on HTTP endpoints
    #[prost(message, optional, tag = "15")]
    pub http_options:
        ::core::option::Option<super::super::super::apps::script::r#type::HttpOptions>,
}
#[doc = r" Generated client implementations."]
pub mod g_suite_add_ons_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " A service for managing Google Workspace Add-ons deployments."]
    #[doc = ""]
    #[doc = " A Google Workspace Add-on is a third-party embedded component that can be"]
    #[doc = " installed in Google Workspace Applications like Gmail, Calendar, Drive, and"]
    #[doc = " the Google Docs, Sheets, and Slides editors. Google Workspace Add-ons can"]
    #[doc = " display UI cards, receive contextual information from the host application,"]
    #[doc = " and perform actions in the host application (See:"]
    #[doc = " https://developers.google.com/gsuite/add-ons/overview for more information)."]
    #[doc = ""]
    #[doc = " A Google Workspace Add-on deployment resource specifies metadata about the"]
    #[doc = " add-on, including a specification of the entry points in the host application"]
    #[doc = " that trigger add-on executions (see:"]
    #[doc = " https://developers.google.com/gsuite/add-ons/concepts/gsuite-manifests)."]
    #[doc = " Add-on deployments defined via the Google Workspace Add-ons API define their"]
    #[doc = " entrypoints using HTTPS URLs (See:"]
    #[doc = " https://developers.google.com/gsuite/add-ons/guides/alternate-runtimes),"]
    #[doc = ""]
    #[doc = " A Google Workspace Add-on deployment can be installed in developer mode,"]
    #[doc = " which allows an add-on developer to test the experience an end-user would see"]
    #[doc = " when installing and running the add-on in their G Suite applications.  When"]
    #[doc = " running in developer mode, more detailed error messages are exposed in the"]
    #[doc = " add-on UI to aid in debugging."]
    #[doc = ""]
    #[doc = " A Google Workspace Add-on deployment can be published to Google Workspace"]
    #[doc = " Marketplace, which allows other Google Workspace users to discover and"]
    #[doc = " install the add-on.  See:"]
    #[doc = " https://developers.google.com/gsuite/add-ons/how-tos/publish-add-on-overview"]
    #[doc = " for details."]
    #[derive(Debug, Clone)]
    pub struct GSuiteAddOnsClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> GSuiteAddOnsClient<T>
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
        ) -> GSuiteAddOnsClient<InterceptedService<T, F>>
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
            GSuiteAddOnsClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Gets the authorization information for deployments in a given project."]
        pub async fn get_authorization(
            &mut self,
            request: impl tonic::IntoRequest<super::GetAuthorizationRequest>,
        ) -> Result<tonic::Response<super::Authorization>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.gsuiteaddons.v1.GSuiteAddOns/GetAuthorization",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a deployment with the specified name and configuration."]
        pub async fn create_deployment(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateDeploymentRequest>,
        ) -> Result<tonic::Response<super::Deployment>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.gsuiteaddons.v1.GSuiteAddOns/CreateDeployment",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates or replaces a deployment with the specified name."]
        pub async fn replace_deployment(
            &mut self,
            request: impl tonic::IntoRequest<super::ReplaceDeploymentRequest>,
        ) -> Result<tonic::Response<super::Deployment>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.gsuiteaddons.v1.GSuiteAddOns/ReplaceDeployment",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets the deployment with the specified name."]
        pub async fn get_deployment(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDeploymentRequest>,
        ) -> Result<tonic::Response<super::Deployment>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.gsuiteaddons.v1.GSuiteAddOns/GetDeployment",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists all deployments in a particular project."]
        pub async fn list_deployments(
            &mut self,
            request: impl tonic::IntoRequest<super::ListDeploymentsRequest>,
        ) -> Result<tonic::Response<super::ListDeploymentsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.gsuiteaddons.v1.GSuiteAddOns/ListDeployments",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes the deployment with the given name."]
        pub async fn delete_deployment(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteDeploymentRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.gsuiteaddons.v1.GSuiteAddOns/DeleteDeployment",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Installs a deployment in developer mode."]
        #[doc = " See:"]
        #[doc = " https://developers.google.com/gsuite/add-ons/how-tos/testing-gsuite-addons."]
        pub async fn install_deployment(
            &mut self,
            request: impl tonic::IntoRequest<super::InstallDeploymentRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.gsuiteaddons.v1.GSuiteAddOns/InstallDeployment",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Uninstalls a developer mode deployment."]
        #[doc = " See:"]
        #[doc = " https://developers.google.com/gsuite/add-ons/how-tos/testing-gsuite-addons."]
        pub async fn uninstall_deployment(
            &mut self,
            request: impl tonic::IntoRequest<super::UninstallDeploymentRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.gsuiteaddons.v1.GSuiteAddOns/UninstallDeployment",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Fetches the install status of a developer mode deployment."]
        pub async fn get_install_status(
            &mut self,
            request: impl tonic::IntoRequest<super::GetInstallStatusRequest>,
        ) -> Result<tonic::Response<super::InstallStatus>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.gsuiteaddons.v1.GSuiteAddOns/GetInstallStatus",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
