/// A billing account in [GCP Console](https://console.cloud.google.com/).
/// You can assign a billing account to one or more projects.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BillingAccount {
    /// The resource name of the billing account. The resource name has the form
    /// `billingAccounts/{billing_account_id}`. For example,
    /// `billingAccounts/012345-567890-ABCDEF` would be the resource name for
    /// billing account `012345-567890-ABCDEF`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Output only. True if the billing account is open, and will therefore be charged for any
    /// usage on associated projects. False if the billing account is closed, and
    /// therefore projects associated with it will be unable to use paid services.
    #[prost(bool, tag = "2")]
    pub open: bool,
    /// The display name given to the billing account, such as `My Billing
    /// Account`. This name is displayed in the GCP Console.
    #[prost(string, tag = "3")]
    pub display_name: std::string::String,
    /// If this account is a
    /// [subaccount](https://cloud.google.com/billing/docs/concepts), then this
    /// will be the resource name of the master billing account that it is being
    /// resold through.
    /// Otherwise this will be empty.
    #[prost(string, tag = "4")]
    pub master_billing_account: std::string::String,
}
/// Encapsulation of billing information for a GCP Console project. A project
/// has at most one associated billing account at a time (but a billing account
/// can be assigned to multiple projects).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProjectBillingInfo {
    /// The resource name for the `ProjectBillingInfo`; has the form
    /// `projects/{project_id}/billingInfo`. For example, the resource name for the
    /// billing information for project `tokyo-rain-123` would be
    /// `projects/tokyo-rain-123/billingInfo`. This field is read-only.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// The ID of the project that this `ProjectBillingInfo` represents, such as
    /// `tokyo-rain-123`. This is a convenience field so that you don't need to
    /// parse the `name` field to obtain a project ID. This field is read-only.
    #[prost(string, tag = "2")]
    pub project_id: std::string::String,
    /// The resource name of the billing account associated with the project, if
    /// any. For example, `billingAccounts/012345-567890-ABCDEF`.
    #[prost(string, tag = "3")]
    pub billing_account_name: std::string::String,
    /// True if the project is associated with an open billing account, to which
    /// usage on the project is charged. False if the project is associated with a
    /// closed billing account, or no billing account at all, and therefore cannot
    /// use paid services. This field is read-only.
    #[prost(bool, tag = "4")]
    pub billing_enabled: bool,
}
/// Request message for `GetBillingAccount`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBillingAccountRequest {
    /// Required. The resource name of the billing account to retrieve. For example,
    /// `billingAccounts/012345-567890-ABCDEF`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Request message for `ListBillingAccounts`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListBillingAccountsRequest {
    /// Requested page size. The maximum page size is 100; this is also the
    /// default.
    #[prost(int32, tag = "1")]
    pub page_size: i32,
    /// A token identifying a page of results to return. This should be a
    /// `next_page_token` value returned from a previous `ListBillingAccounts`
    /// call. If unspecified, the first page of results is returned.
    #[prost(string, tag = "2")]
    pub page_token: std::string::String,
    /// Options for how to filter the returned billing accounts.
    /// Currently this only supports filtering for
    /// [subaccounts](https://cloud.google.com/billing/docs/concepts) under a
    /// single provided reseller billing account.
    /// (e.g. "master_billing_account=billingAccounts/012345-678901-ABCDEF").
    /// Boolean algebra and other fields are not currently supported.
    #[prost(string, tag = "3")]
    pub filter: std::string::String,
}
/// Response message for `ListBillingAccounts`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListBillingAccountsResponse {
    /// A list of billing accounts.
    #[prost(message, repeated, tag = "1")]
    pub billing_accounts: ::std::vec::Vec<BillingAccount>,
    /// A token to retrieve the next page of results. To retrieve the next page,
    /// call `ListBillingAccounts` again with the `page_token` field set to this
    /// value. This field is empty if there are no more results to retrieve.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
}
/// Request message for `CreateBillingAccount`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateBillingAccountRequest {
    /// Required. The billing account resource to create.
    /// Currently CreateBillingAccount only supports subaccount creation, so
    /// any created billing accounts must be under a provided master billing
    /// account.
    #[prost(message, optional, tag = "1")]
    pub billing_account: ::std::option::Option<BillingAccount>,
}
/// Request message for `UpdateBillingAccount`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateBillingAccountRequest {
    /// Required. The name of the billing account resource to be updated.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Required. The billing account resource to replace the resource on the server.
    #[prost(message, optional, tag = "2")]
    pub account: ::std::option::Option<BillingAccount>,
    /// The update mask applied to the resource.
    /// Only "display_name" is currently supported.
    #[prost(message, optional, tag = "3")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
}
/// Request message for `ListProjectBillingInfo`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListProjectBillingInfoRequest {
    /// Required. The resource name of the billing account associated with the projects that
    /// you want to list. For example, `billingAccounts/012345-567890-ABCDEF`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Requested page size. The maximum page size is 100; this is also the
    /// default.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A token identifying a page of results to be returned. This should be a
    /// `next_page_token` value returned from a previous `ListProjectBillingInfo`
    /// call. If unspecified, the first page of results is returned.
    #[prost(string, tag = "3")]
    pub page_token: std::string::String,
}
/// Request message for `ListProjectBillingInfoResponse`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListProjectBillingInfoResponse {
    /// A list of `ProjectBillingInfo` resources representing the projects
    /// associated with the billing account.
    #[prost(message, repeated, tag = "1")]
    pub project_billing_info: ::std::vec::Vec<ProjectBillingInfo>,
    /// A token to retrieve the next page of results. To retrieve the next page,
    /// call `ListProjectBillingInfo` again with the `page_token` field set to this
    /// value. This field is empty if there are no more results to retrieve.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
}
/// Request message for `GetProjectBillingInfo`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetProjectBillingInfoRequest {
    /// Required. The resource name of the project for which billing information is
    /// retrieved. For example, `projects/tokyo-rain-123`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Request message for `UpdateProjectBillingInfo`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateProjectBillingInfoRequest {
    /// Required. The resource name of the project associated with the billing information
    /// that you want to update. For example, `projects/tokyo-rain-123`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// The new billing information for the project. Read-only fields are ignored;
    /// thus, you can leave empty all fields except `billing_account_name`.
    #[prost(message, optional, tag = "2")]
    pub project_billing_info: ::std::option::Option<ProjectBillingInfo>,
}
#[doc = r" Generated client implementations."]
pub mod cloud_billing_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Retrieves GCP Console billing accounts and associates them with projects."]
    pub struct CloudBillingClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl CloudBillingClient<tonic::transport::Channel> {
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
    impl<T> CloudBillingClient<T>
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
        #[doc = " Gets information about a billing account. The current authenticated user"]
        #[doc = " must be a [viewer of the billing"]
        #[doc = " account](https://cloud.google.com/billing/docs/how-to/billing-access)."]
        pub async fn get_billing_account(
            &mut self,
            request: impl tonic::IntoRequest<super::GetBillingAccountRequest>,
        ) -> Result<tonic::Response<super::BillingAccount>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.billing.v1.CloudBilling/GetBillingAccount",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists the billing accounts that the current authenticated user has"]
        #[doc = " permission to"]
        #[doc = " [view](https://cloud.google.com/billing/docs/how-to/billing-access)."]
        pub async fn list_billing_accounts(
            &mut self,
            request: impl tonic::IntoRequest<super::ListBillingAccountsRequest>,
        ) -> Result<tonic::Response<super::ListBillingAccountsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.billing.v1.CloudBilling/ListBillingAccounts",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates a billing account's fields."]
        #[doc = " Currently the only field that can be edited is `display_name`."]
        #[doc = " The current authenticated user must have the `billing.accounts.update`"]
        #[doc = " IAM permission, which is typically given to the"]
        #[doc = " [administrator](https://cloud.google.com/billing/docs/how-to/billing-access)"]
        #[doc = " of the billing account."]
        pub async fn update_billing_account(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateBillingAccountRequest>,
        ) -> Result<tonic::Response<super::BillingAccount>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.billing.v1.CloudBilling/UpdateBillingAccount",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a billing account."]
        #[doc = " This method can only be used to create"]
        #[doc = " [billing subaccounts](https://cloud.google.com/billing/docs/concepts)"]
        #[doc = " by GCP resellers."]
        #[doc = " When creating a subaccount, the current authenticated user must have the"]
        #[doc = " `billing.accounts.update` IAM permission on the master account, which is"]
        #[doc = " typically given to billing account"]
        #[doc = " [administrators](https://cloud.google.com/billing/docs/how-to/billing-access)."]
        #[doc = " This method will return an error if the master account has not been"]
        #[doc = " provisioned as a reseller account."]
        pub async fn create_billing_account(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateBillingAccountRequest>,
        ) -> Result<tonic::Response<super::BillingAccount>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.billing.v1.CloudBilling/CreateBillingAccount",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists the projects associated with a billing account. The current"]
        #[doc = " authenticated user must have the `billing.resourceAssociations.list` IAM"]
        #[doc = " permission, which is often given to billing account"]
        #[doc = " [viewers](https://cloud.google.com/billing/docs/how-to/billing-access)."]
        pub async fn list_project_billing_info(
            &mut self,
            request: impl tonic::IntoRequest<super::ListProjectBillingInfoRequest>,
        ) -> Result<tonic::Response<super::ListProjectBillingInfoResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.billing.v1.CloudBilling/ListProjectBillingInfo",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets the billing information for a project. The current authenticated user"]
        #[doc = " must have [permission to view the"]
        #[doc = " project](https://cloud.google.com/docs/permissions-overview#h.bgs0oxofvnoo"]
        #[doc = " )."]
        pub async fn get_project_billing_info(
            &mut self,
            request: impl tonic::IntoRequest<super::GetProjectBillingInfoRequest>,
        ) -> Result<tonic::Response<super::ProjectBillingInfo>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.billing.v1.CloudBilling/GetProjectBillingInfo",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Sets or updates the billing account associated with a project. You specify"]
        #[doc = " the new billing account by setting the `billing_account_name` in the"]
        #[doc = " `ProjectBillingInfo` resource to the resource name of a billing account."]
        #[doc = " Associating a project with an open billing account enables billing on the"]
        #[doc = " project and allows charges for resource usage. If the project already had a"]
        #[doc = " billing account, this method changes the billing account used for resource"]
        #[doc = " usage charges."]
        #[doc = ""]
        #[doc = " *Note:* Incurred charges that have not yet been reported in the transaction"]
        #[doc = " history of the GCP Console might be billed to the new billing"]
        #[doc = " account, even if the charge occurred before the new billing account was"]
        #[doc = " assigned to the project."]
        #[doc = ""]
        #[doc = " The current authenticated user must have ownership privileges for both the"]
        #[doc = " [project](https://cloud.google.com/docs/permissions-overview#h.bgs0oxofvnoo"]
        #[doc = " ) and the [billing"]
        #[doc = " account](https://cloud.google.com/billing/docs/how-to/billing-access)."]
        #[doc = ""]
        #[doc = " You can disable billing on the project by setting the"]
        #[doc = " `billing_account_name` field to empty. This action disassociates the"]
        #[doc = " current billing account from the project. Any billable activity of your"]
        #[doc = " in-use services will stop, and your application could stop functioning as"]
        #[doc = " expected. Any unbilled charges to date will be billed to the previously"]
        #[doc = " associated account. The current authenticated user must be either an owner"]
        #[doc = " of the project or an owner of the billing account for the project."]
        #[doc = ""]
        #[doc = " Note that associating a project with a *closed* billing account will have"]
        #[doc = " much the same effect as disabling billing on the project: any paid"]
        #[doc = " resources used by the project will be shut down. Thus, unless you wish to"]
        #[doc = " disable billing, you should always call this method with the name of an"]
        #[doc = " *open* billing account."]
        pub async fn update_project_billing_info(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateProjectBillingInfoRequest>,
        ) -> Result<tonic::Response<super::ProjectBillingInfo>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.billing.v1.CloudBilling/UpdateProjectBillingInfo",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets the access control policy for a billing account."]
        #[doc = " The caller must have the `billing.accounts.getIamPolicy` permission on the"]
        #[doc = " account, which is often given to billing account"]
        #[doc = " [viewers](https://cloud.google.com/billing/docs/how-to/billing-access)."]
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
                "/google.cloud.billing.v1.CloudBilling/GetIamPolicy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Sets the access control policy for a billing account. Replaces any existing"]
        #[doc = " policy."]
        #[doc = " The caller must have the `billing.accounts.setIamPolicy` permission on the"]
        #[doc = " account, which is often given to billing account"]
        #[doc = " [administrators](https://cloud.google.com/billing/docs/how-to/billing-access)."]
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
                "/google.cloud.billing.v1.CloudBilling/SetIamPolicy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Tests the access control policy for a billing account. This method takes"]
        #[doc = " the resource and a set of permissions as input and returns the subset of"]
        #[doc = " the input permissions that the caller is allowed for that resource."]
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
                "/google.cloud.billing.v1.CloudBilling/TestIamPermissions",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for CloudBillingClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for CloudBillingClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "CloudBillingClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod cloud_billing_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with CloudBillingServer."]
    #[async_trait]
    pub trait CloudBilling: Send + Sync + 'static {
        #[doc = " Gets information about a billing account. The current authenticated user"]
        #[doc = " must be a [viewer of the billing"]
        #[doc = " account](https://cloud.google.com/billing/docs/how-to/billing-access)."]
        async fn get_billing_account(
            &self,
            request: tonic::Request<super::GetBillingAccountRequest>,
        ) -> Result<tonic::Response<super::BillingAccount>, tonic::Status>;
        #[doc = " Lists the billing accounts that the current authenticated user has"]
        #[doc = " permission to"]
        #[doc = " [view](https://cloud.google.com/billing/docs/how-to/billing-access)."]
        async fn list_billing_accounts(
            &self,
            request: tonic::Request<super::ListBillingAccountsRequest>,
        ) -> Result<tonic::Response<super::ListBillingAccountsResponse>, tonic::Status>;
        #[doc = " Updates a billing account's fields."]
        #[doc = " Currently the only field that can be edited is `display_name`."]
        #[doc = " The current authenticated user must have the `billing.accounts.update`"]
        #[doc = " IAM permission, which is typically given to the"]
        #[doc = " [administrator](https://cloud.google.com/billing/docs/how-to/billing-access)"]
        #[doc = " of the billing account."]
        async fn update_billing_account(
            &self,
            request: tonic::Request<super::UpdateBillingAccountRequest>,
        ) -> Result<tonic::Response<super::BillingAccount>, tonic::Status>;
        #[doc = " Creates a billing account."]
        #[doc = " This method can only be used to create"]
        #[doc = " [billing subaccounts](https://cloud.google.com/billing/docs/concepts)"]
        #[doc = " by GCP resellers."]
        #[doc = " When creating a subaccount, the current authenticated user must have the"]
        #[doc = " `billing.accounts.update` IAM permission on the master account, which is"]
        #[doc = " typically given to billing account"]
        #[doc = " [administrators](https://cloud.google.com/billing/docs/how-to/billing-access)."]
        #[doc = " This method will return an error if the master account has not been"]
        #[doc = " provisioned as a reseller account."]
        async fn create_billing_account(
            &self,
            request: tonic::Request<super::CreateBillingAccountRequest>,
        ) -> Result<tonic::Response<super::BillingAccount>, tonic::Status>;
        #[doc = " Lists the projects associated with a billing account. The current"]
        #[doc = " authenticated user must have the `billing.resourceAssociations.list` IAM"]
        #[doc = " permission, which is often given to billing account"]
        #[doc = " [viewers](https://cloud.google.com/billing/docs/how-to/billing-access)."]
        async fn list_project_billing_info(
            &self,
            request: tonic::Request<super::ListProjectBillingInfoRequest>,
        ) -> Result<tonic::Response<super::ListProjectBillingInfoResponse>, tonic::Status>;
        #[doc = " Gets the billing information for a project. The current authenticated user"]
        #[doc = " must have [permission to view the"]
        #[doc = " project](https://cloud.google.com/docs/permissions-overview#h.bgs0oxofvnoo"]
        #[doc = " )."]
        async fn get_project_billing_info(
            &self,
            request: tonic::Request<super::GetProjectBillingInfoRequest>,
        ) -> Result<tonic::Response<super::ProjectBillingInfo>, tonic::Status>;
        #[doc = " Sets or updates the billing account associated with a project. You specify"]
        #[doc = " the new billing account by setting the `billing_account_name` in the"]
        #[doc = " `ProjectBillingInfo` resource to the resource name of a billing account."]
        #[doc = " Associating a project with an open billing account enables billing on the"]
        #[doc = " project and allows charges for resource usage. If the project already had a"]
        #[doc = " billing account, this method changes the billing account used for resource"]
        #[doc = " usage charges."]
        #[doc = ""]
        #[doc = " *Note:* Incurred charges that have not yet been reported in the transaction"]
        #[doc = " history of the GCP Console might be billed to the new billing"]
        #[doc = " account, even if the charge occurred before the new billing account was"]
        #[doc = " assigned to the project."]
        #[doc = ""]
        #[doc = " The current authenticated user must have ownership privileges for both the"]
        #[doc = " [project](https://cloud.google.com/docs/permissions-overview#h.bgs0oxofvnoo"]
        #[doc = " ) and the [billing"]
        #[doc = " account](https://cloud.google.com/billing/docs/how-to/billing-access)."]
        #[doc = ""]
        #[doc = " You can disable billing on the project by setting the"]
        #[doc = " `billing_account_name` field to empty. This action disassociates the"]
        #[doc = " current billing account from the project. Any billable activity of your"]
        #[doc = " in-use services will stop, and your application could stop functioning as"]
        #[doc = " expected. Any unbilled charges to date will be billed to the previously"]
        #[doc = " associated account. The current authenticated user must be either an owner"]
        #[doc = " of the project or an owner of the billing account for the project."]
        #[doc = ""]
        #[doc = " Note that associating a project with a *closed* billing account will have"]
        #[doc = " much the same effect as disabling billing on the project: any paid"]
        #[doc = " resources used by the project will be shut down. Thus, unless you wish to"]
        #[doc = " disable billing, you should always call this method with the name of an"]
        #[doc = " *open* billing account."]
        async fn update_project_billing_info(
            &self,
            request: tonic::Request<super::UpdateProjectBillingInfoRequest>,
        ) -> Result<tonic::Response<super::ProjectBillingInfo>, tonic::Status>;
        #[doc = " Gets the access control policy for a billing account."]
        #[doc = " The caller must have the `billing.accounts.getIamPolicy` permission on the"]
        #[doc = " account, which is often given to billing account"]
        #[doc = " [viewers](https://cloud.google.com/billing/docs/how-to/billing-access)."]
        async fn get_iam_policy(
            &self,
            request: tonic::Request<super::super::super::super::iam::v1::GetIamPolicyRequest>,
        ) -> Result<tonic::Response<super::super::super::super::iam::v1::Policy>, tonic::Status>;
        #[doc = " Sets the access control policy for a billing account. Replaces any existing"]
        #[doc = " policy."]
        #[doc = " The caller must have the `billing.accounts.setIamPolicy` permission on the"]
        #[doc = " account, which is often given to billing account"]
        #[doc = " [administrators](https://cloud.google.com/billing/docs/how-to/billing-access)."]
        async fn set_iam_policy(
            &self,
            request: tonic::Request<super::super::super::super::iam::v1::SetIamPolicyRequest>,
        ) -> Result<tonic::Response<super::super::super::super::iam::v1::Policy>, tonic::Status>;
        #[doc = " Tests the access control policy for a billing account. This method takes"]
        #[doc = " the resource and a set of permissions as input and returns the subset of"]
        #[doc = " the input permissions that the caller is allowed for that resource."]
        async fn test_iam_permissions(
            &self,
            request: tonic::Request<super::super::super::super::iam::v1::TestIamPermissionsRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::iam::v1::TestIamPermissionsResponse>,
            tonic::Status,
        >;
    }
    #[doc = " Retrieves GCP Console billing accounts and associates them with projects."]
    #[derive(Debug)]
    #[doc(hidden)]
    pub struct CloudBillingServer<T: CloudBilling> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: CloudBilling> CloudBillingServer<T> {
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
    impl<T, B> Service<http::Request<B>> for CloudBillingServer<T>
    where
        T: CloudBilling,
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
                "/google.cloud.billing.v1.CloudBilling/GetBillingAccount" => {
                    #[allow(non_camel_case_types)]
                    struct GetBillingAccountSvc<T: CloudBilling>(pub Arc<T>);
                    impl<T: CloudBilling>
                        tonic::server::UnaryService<super::GetBillingAccountRequest>
                        for GetBillingAccountSvc<T>
                    {
                        type Response = super::BillingAccount;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetBillingAccountRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.get_billing_account(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetBillingAccountSvc(inner);
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
                "/google.cloud.billing.v1.CloudBilling/ListBillingAccounts" => {
                    #[allow(non_camel_case_types)]
                    struct ListBillingAccountsSvc<T: CloudBilling>(pub Arc<T>);
                    impl<T: CloudBilling>
                        tonic::server::UnaryService<super::ListBillingAccountsRequest>
                        for ListBillingAccountsSvc<T>
                    {
                        type Response = super::ListBillingAccountsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListBillingAccountsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.list_billing_accounts(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ListBillingAccountsSvc(inner);
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
                "/google.cloud.billing.v1.CloudBilling/UpdateBillingAccount" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateBillingAccountSvc<T: CloudBilling>(pub Arc<T>);
                    impl<T: CloudBilling>
                        tonic::server::UnaryService<super::UpdateBillingAccountRequest>
                        for UpdateBillingAccountSvc<T>
                    {
                        type Response = super::BillingAccount;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateBillingAccountRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.update_billing_account(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = UpdateBillingAccountSvc(inner);
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
                "/google.cloud.billing.v1.CloudBilling/CreateBillingAccount" => {
                    #[allow(non_camel_case_types)]
                    struct CreateBillingAccountSvc<T: CloudBilling>(pub Arc<T>);
                    impl<T: CloudBilling>
                        tonic::server::UnaryService<super::CreateBillingAccountRequest>
                        for CreateBillingAccountSvc<T>
                    {
                        type Response = super::BillingAccount;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateBillingAccountRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.create_billing_account(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = CreateBillingAccountSvc(inner);
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
                "/google.cloud.billing.v1.CloudBilling/ListProjectBillingInfo" => {
                    #[allow(non_camel_case_types)]
                    struct ListProjectBillingInfoSvc<T: CloudBilling>(pub Arc<T>);
                    impl<T: CloudBilling>
                        tonic::server::UnaryService<super::ListProjectBillingInfoRequest>
                        for ListProjectBillingInfoSvc<T>
                    {
                        type Response = super::ListProjectBillingInfoResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListProjectBillingInfoRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.list_project_billing_info(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ListProjectBillingInfoSvc(inner);
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
                "/google.cloud.billing.v1.CloudBilling/GetProjectBillingInfo" => {
                    #[allow(non_camel_case_types)]
                    struct GetProjectBillingInfoSvc<T: CloudBilling>(pub Arc<T>);
                    impl<T: CloudBilling>
                        tonic::server::UnaryService<super::GetProjectBillingInfoRequest>
                        for GetProjectBillingInfoSvc<T>
                    {
                        type Response = super::ProjectBillingInfo;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetProjectBillingInfoRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.get_project_billing_info(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetProjectBillingInfoSvc(inner);
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
                "/google.cloud.billing.v1.CloudBilling/UpdateProjectBillingInfo" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateProjectBillingInfoSvc<T: CloudBilling>(pub Arc<T>);
                    impl<T: CloudBilling>
                        tonic::server::UnaryService<super::UpdateProjectBillingInfoRequest>
                        for UpdateProjectBillingInfoSvc<T>
                    {
                        type Response = super::ProjectBillingInfo;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateProjectBillingInfoRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { inner.update_project_billing_info(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = UpdateProjectBillingInfoSvc(inner);
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
                "/google.cloud.billing.v1.CloudBilling/GetIamPolicy" => {
                    #[allow(non_camel_case_types)]
                    struct GetIamPolicySvc<T: CloudBilling>(pub Arc<T>);
                    impl<T: CloudBilling>
                        tonic::server::UnaryService<
                            super::super::super::super::iam::v1::GetIamPolicyRequest,
                        > for GetIamPolicySvc<T>
                    {
                        type Response = super::super::super::super::iam::v1::Policy;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::super::super::iam::v1::GetIamPolicyRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.get_iam_policy(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetIamPolicySvc(inner);
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
                "/google.cloud.billing.v1.CloudBilling/SetIamPolicy" => {
                    #[allow(non_camel_case_types)]
                    struct SetIamPolicySvc<T: CloudBilling>(pub Arc<T>);
                    impl<T: CloudBilling>
                        tonic::server::UnaryService<
                            super::super::super::super::iam::v1::SetIamPolicyRequest,
                        > for SetIamPolicySvc<T>
                    {
                        type Response = super::super::super::super::iam::v1::Policy;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::super::super::iam::v1::SetIamPolicyRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.set_iam_policy(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = SetIamPolicySvc(inner);
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
                "/google.cloud.billing.v1.CloudBilling/TestIamPermissions" => {
                    #[allow(non_camel_case_types)]
                    struct TestIamPermissionsSvc<T: CloudBilling>(pub Arc<T>);
                    impl<T: CloudBilling>
                        tonic::server::UnaryService<
                            super::super::super::super::iam::v1::TestIamPermissionsRequest,
                        > for TestIamPermissionsSvc<T>
                    {
                        type Response =
                            super::super::super::super::iam::v1::TestIamPermissionsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::super::super::iam::v1::TestIamPermissionsRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.test_iam_permissions(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = TestIamPermissionsSvc(inner);
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
    impl<T: CloudBilling> Clone for CloudBillingServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: CloudBilling> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: CloudBilling> tonic::transport::NamedService for CloudBillingServer<T> {
        const NAME: &'static str = "google.cloud.billing.v1.CloudBilling";
    }
}
/// Encapsulates a single service in Google Cloud Platform.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Service {
    /// The resource name for the service.
    /// Example: "services/DA34-426B-A397"
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// The identifier for the service.
    /// Example: "DA34-426B-A397"
    #[prost(string, tag = "2")]
    pub service_id: std::string::String,
    /// A human readable display name for this service.
    #[prost(string, tag = "3")]
    pub display_name: std::string::String,
    /// The business under which the service is offered.
    /// Ex. "businessEntities/GCP", "businessEntities/Maps"
    #[prost(string, tag = "4")]
    pub business_entity_name: std::string::String,
}
/// Encapsulates a single SKU in Google Cloud Platform
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Sku {
    /// The resource name for the SKU.
    /// Example: "services/DA34-426B-A397/skus/AA95-CD31-42FE"
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// The identifier for the SKU.
    /// Example: "AA95-CD31-42FE"
    #[prost(string, tag = "2")]
    pub sku_id: std::string::String,
    /// A human readable description of the SKU, has a maximum length of 256
    /// characters.
    #[prost(string, tag = "3")]
    pub description: std::string::String,
    /// The category hierarchy of this SKU, purely for organizational purpose.
    #[prost(message, optional, tag = "4")]
    pub category: ::std::option::Option<Category>,
    /// List of service regions this SKU is offered at.
    /// Example: "asia-east1"
    /// Service regions can be found at https://cloud.google.com/about/locations/
    #[prost(string, repeated, tag = "5")]
    pub service_regions: ::std::vec::Vec<std::string::String>,
    /// A timeline of pricing info for this SKU in chronological order.
    #[prost(message, repeated, tag = "6")]
    pub pricing_info: ::std::vec::Vec<PricingInfo>,
    /// Identifies the service provider.
    /// This is 'Google' for first party services in Google Cloud Platform.
    #[prost(string, tag = "7")]
    pub service_provider_name: std::string::String,
}
/// Represents the category hierarchy of a SKU.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Category {
    /// The display name of the service this SKU belongs to.
    #[prost(string, tag = "1")]
    pub service_display_name: std::string::String,
    /// The type of product the SKU refers to.
    /// Example: "Compute", "Storage", "Network", "ApplicationServices" etc.
    #[prost(string, tag = "2")]
    pub resource_family: std::string::String,
    /// A group classification for related SKUs.
    /// Example: "RAM", "GPU", "Prediction", "Ops", "GoogleEgress" etc.
    #[prost(string, tag = "3")]
    pub resource_group: std::string::String,
    /// Represents how the SKU is consumed.
    /// Example: "OnDemand", "Preemptible", "Commit1Mo", "Commit1Yr" etc.
    #[prost(string, tag = "4")]
    pub usage_type: std::string::String,
}
/// Represents the pricing information for a SKU at a single point of time.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PricingInfo {
    /// The timestamp from which this pricing was effective within the requested
    /// time range. This is guaranteed to be greater than or equal to the
    /// start_time field in the request and less than the end_time field in the
    /// request. If a time range was not specified in the request this field will
    /// be equivalent to a time within the last 12 hours, indicating the latest
    /// pricing info.
    #[prost(message, optional, tag = "1")]
    pub effective_time: ::std::option::Option<::prost_types::Timestamp>,
    /// An optional human readable summary of the pricing information, has a
    /// maximum length of 256 characters.
    #[prost(string, tag = "2")]
    pub summary: std::string::String,
    /// Expresses the pricing formula. See `PricingExpression` for an example.
    #[prost(message, optional, tag = "3")]
    pub pricing_expression: ::std::option::Option<PricingExpression>,
    /// Aggregation Info. This can be left unspecified if the pricing expression
    /// doesn't require aggregation.
    #[prost(message, optional, tag = "4")]
    pub aggregation_info: ::std::option::Option<AggregationInfo>,
    /// Conversion rate used for currency conversion, from USD to the currency
    /// specified in the request. This includes any surcharge collected for billing
    /// in non USD currency. If a currency is not specified in the request this
    /// defaults to 1.0.
    /// Example: USD * currency_conversion_rate = JPY
    #[prost(double, tag = "5")]
    pub currency_conversion_rate: f64,
}
/// Expresses a mathematical pricing formula. For Example:-
///
/// `usage_unit: GBy`
/// `tiered_rates:`
///    `[start_usage_amount: 20, unit_price: $10]`
///    `[start_usage_amount: 100, unit_price: $5]`
///
/// The above expresses a pricing formula where the first 20GB is free, the
/// next 80GB is priced at $10 per GB followed by $5 per GB for additional
/// usage.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PricingExpression {
    /// The short hand for unit of usage this pricing is specified in.
    /// Example: usage_unit of "GiBy" means that usage is specified in "Gibi Byte".
    #[prost(string, tag = "1")]
    pub usage_unit: std::string::String,
    /// The unit of usage in human readable form.
    /// Example: "gibi byte".
    #[prost(string, tag = "4")]
    pub usage_unit_description: std::string::String,
    /// The base unit for the SKU which is the unit used in usage exports.
    /// Example: "By"
    #[prost(string, tag = "5")]
    pub base_unit: std::string::String,
    /// The base unit in human readable form.
    /// Example: "byte".
    #[prost(string, tag = "6")]
    pub base_unit_description: std::string::String,
    /// Conversion factor for converting from price per usage_unit to price per
    /// base_unit, and start_usage_amount to start_usage_amount in base_unit.
    /// unit_price / base_unit_conversion_factor = price per base_unit.
    /// start_usage_amount * base_unit_conversion_factor = start_usage_amount in
    /// base_unit.
    #[prost(double, tag = "7")]
    pub base_unit_conversion_factor: f64,
    /// The recommended quantity of units for displaying pricing info. When
    /// displaying pricing info it is recommended to display:
    /// (unit_price * display_quantity) per display_quantity usage_unit.
    /// This field does not affect the pricing formula and is for display purposes
    /// only.
    /// Example: If the unit_price is "0.0001 USD", the usage_unit is "GB" and
    /// the display_quantity is "1000" then the recommended way of displaying the
    /// pricing info is "0.10 USD per 1000 GB"
    #[prost(double, tag = "2")]
    pub display_quantity: f64,
    /// The list of tiered rates for this pricing. The total cost is computed by
    /// applying each of the tiered rates on usage. This repeated list is sorted
    /// by ascending order of start_usage_amount.
    #[prost(message, repeated, tag = "3")]
    pub tiered_rates: ::std::vec::Vec<pricing_expression::TierRate>,
}
pub mod pricing_expression {
    /// The price rate indicating starting usage and its corresponding price.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TierRate {
        /// Usage is priced at this rate only after this amount.
        /// Example: start_usage_amount of 10 indicates that the usage will be priced
        /// at the unit_price after the first 10 usage_units.
        #[prost(double, tag = "1")]
        pub start_usage_amount: f64,
        /// The price per unit of usage.
        /// Example: unit_price of amount $10 indicates that each unit will cost $10.
        #[prost(message, optional, tag = "2")]
        pub unit_price: ::std::option::Option<super::super::super::super::r#type::Money>,
    }
}
/// Represents the aggregation level and interval for pricing of a single SKU.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AggregationInfo {
    #[prost(enumeration = "aggregation_info::AggregationLevel", tag = "1")]
    pub aggregation_level: i32,
    #[prost(enumeration = "aggregation_info::AggregationInterval", tag = "2")]
    pub aggregation_interval: i32,
    /// The number of intervals to aggregate over.
    /// Example: If aggregation_level is "DAILY" and aggregation_count is 14,
    /// aggregation will be over 14 days.
    #[prost(int32, tag = "3")]
    pub aggregation_count: i32,
}
pub mod aggregation_info {
    /// The level at which usage is aggregated to compute cost.
    /// Example: "ACCOUNT" aggregation level indicates that usage for tiered
    /// pricing is aggregated across all projects in a single account.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum AggregationLevel {
        Unspecified = 0,
        Account = 1,
        Project = 2,
    }
    /// The interval at which usage is aggregated to compute cost.
    /// Example: "MONTHLY" aggregation interval indicates that usage for tiered
    /// pricing is aggregated every month.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum AggregationInterval {
        Unspecified = 0,
        Daily = 1,
        Monthly = 2,
    }
}
/// Request message for `ListServices`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListServicesRequest {
    /// Requested page size. Defaults to 5000.
    #[prost(int32, tag = "1")]
    pub page_size: i32,
    /// A token identifying a page of results to return. This should be a
    /// `next_page_token` value returned from a previous `ListServices`
    /// call. If unspecified, the first page of results is returned.
    #[prost(string, tag = "2")]
    pub page_token: std::string::String,
}
/// Response message for `ListServices`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListServicesResponse {
    /// A list of services.
    #[prost(message, repeated, tag = "1")]
    pub services: ::std::vec::Vec<Service>,
    /// A token to retrieve the next page of results. To retrieve the next page,
    /// call `ListServices` again with the `page_token` field set to this
    /// value. This field is empty if there are no more results to retrieve.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
}
/// Request message for `ListSkus`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSkusRequest {
    /// Required. The name of the service.
    /// Example: "services/DA34-426B-A397"
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Optional inclusive start time of the time range for which the pricing
    /// versions will be returned. Timestamps in the future are not allowed.
    /// The time range has to be within a single calendar month in
    /// America/Los_Angeles timezone. Time range as a whole is optional. If not
    /// specified, the latest pricing will be returned (up to 12 hours old at
    /// most).
    #[prost(message, optional, tag = "2")]
    pub start_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Optional exclusive end time of the time range for which the pricing
    /// versions will be returned. Timestamps in the future are not allowed.
    /// The time range has to be within a single calendar month in
    /// America/Los_Angeles timezone. Time range as a whole is optional. If not
    /// specified, the latest pricing will be returned (up to 12 hours old at
    /// most).
    #[prost(message, optional, tag = "3")]
    pub end_time: ::std::option::Option<::prost_types::Timestamp>,
    /// The ISO 4217 currency code for the pricing info in the response proto.
    /// Will use the conversion rate as of start_time.
    /// Optional. If not specified USD will be used.
    #[prost(string, tag = "4")]
    pub currency_code: std::string::String,
    /// Requested page size. Defaults to 5000.
    #[prost(int32, tag = "5")]
    pub page_size: i32,
    /// A token identifying a page of results to return. This should be a
    /// `next_page_token` value returned from a previous `ListSkus`
    /// call. If unspecified, the first page of results is returned.
    #[prost(string, tag = "6")]
    pub page_token: std::string::String,
}
/// Response message for `ListSkus`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSkusResponse {
    /// The list of public SKUs of the given service.
    #[prost(message, repeated, tag = "1")]
    pub skus: ::std::vec::Vec<Sku>,
    /// A token to retrieve the next page of results. To retrieve the next page,
    /// call `ListSkus` again with the `page_token` field set to this
    /// value. This field is empty if there are no more results to retrieve.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
}
#[doc = r" Generated client implementations."]
pub mod cloud_catalog_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " A catalog of Google Cloud Platform services and SKUs."]
    #[doc = " Provides pricing information and metadata on Google Cloud Platform services"]
    #[doc = " and SKUs."]
    pub struct CloudCatalogClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl CloudCatalogClient<tonic::transport::Channel> {
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
    impl<T> CloudCatalogClient<T>
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
        #[doc = " Lists all public cloud services."]
        pub async fn list_services(
            &mut self,
            request: impl tonic::IntoRequest<super::ListServicesRequest>,
        ) -> Result<tonic::Response<super::ListServicesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.billing.v1.CloudCatalog/ListServices",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists all publicly available SKUs for a given cloud service."]
        pub async fn list_skus(
            &mut self,
            request: impl tonic::IntoRequest<super::ListSkusRequest>,
        ) -> Result<tonic::Response<super::ListSkusResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.billing.v1.CloudCatalog/ListSkus",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for CloudCatalogClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for CloudCatalogClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "CloudCatalogClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod cloud_catalog_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with CloudCatalogServer."]
    #[async_trait]
    pub trait CloudCatalog: Send + Sync + 'static {
        #[doc = " Lists all public cloud services."]
        async fn list_services(
            &self,
            request: tonic::Request<super::ListServicesRequest>,
        ) -> Result<tonic::Response<super::ListServicesResponse>, tonic::Status>;
        #[doc = " Lists all publicly available SKUs for a given cloud service."]
        async fn list_skus(
            &self,
            request: tonic::Request<super::ListSkusRequest>,
        ) -> Result<tonic::Response<super::ListSkusResponse>, tonic::Status>;
    }
    #[doc = " A catalog of Google Cloud Platform services and SKUs."]
    #[doc = " Provides pricing information and metadata on Google Cloud Platform services"]
    #[doc = " and SKUs."]
    #[derive(Debug)]
    #[doc(hidden)]
    pub struct CloudCatalogServer<T: CloudCatalog> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: CloudCatalog> CloudCatalogServer<T> {
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
    impl<T, B> Service<http::Request<B>> for CloudCatalogServer<T>
    where
        T: CloudCatalog,
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
                "/google.cloud.billing.v1.CloudCatalog/ListServices" => {
                    #[allow(non_camel_case_types)]
                    struct ListServicesSvc<T: CloudCatalog>(pub Arc<T>);
                    impl<T: CloudCatalog> tonic::server::UnaryService<super::ListServicesRequest>
                        for ListServicesSvc<T>
                    {
                        type Response = super::ListServicesResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListServicesRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.list_services(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ListServicesSvc(inner);
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
                "/google.cloud.billing.v1.CloudCatalog/ListSkus" => {
                    #[allow(non_camel_case_types)]
                    struct ListSkusSvc<T: CloudCatalog>(pub Arc<T>);
                    impl<T: CloudCatalog> tonic::server::UnaryService<super::ListSkusRequest> for ListSkusSvc<T> {
                        type Response = super::ListSkusResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListSkusRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.list_skus(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ListSkusSvc(inner);
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
    impl<T: CloudCatalog> Clone for CloudCatalogServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: CloudCatalog> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: CloudCatalog> tonic::transport::NamedService for CloudCatalogServer<T> {
        const NAME: &'static str = "google.cloud.billing.v1.CloudCatalog";
    }
}
