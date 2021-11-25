/// A billing account in [GCP Console](<https://console.cloud.google.com/>).
/// You can assign a billing account to one or more projects.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BillingAccount {
    /// The resource name of the billing account. The resource name has the form
    /// `billingAccounts/{billing_account_id}`. For example,
    /// `billingAccounts/012345-567890-ABCDEF` would be the resource name for
    /// billing account `012345-567890-ABCDEF`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Output only. True if the billing account is open, and will therefore be charged for any
    /// usage on associated projects. False if the billing account is closed, and
    /// therefore projects associated with it will be unable to use paid services.
    #[prost(bool, tag = "2")]
    pub open: bool,
    /// The display name given to the billing account, such as `My Billing
    /// Account`. This name is displayed in the GCP Console.
    #[prost(string, tag = "3")]
    pub display_name: ::prost::alloc::string::String,
    /// If this account is a
    /// \[subaccount\](<https://cloud.google.com/billing/docs/concepts>), then this
    /// will be the resource name of the master billing account that it is being
    /// resold through.
    /// Otherwise this will be empty.
    #[prost(string, tag = "4")]
    pub master_billing_account: ::prost::alloc::string::String,
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
    pub name: ::prost::alloc::string::String,
    /// The ID of the project that this `ProjectBillingInfo` represents, such as
    /// `tokyo-rain-123`. This is a convenience field so that you don't need to
    /// parse the `name` field to obtain a project ID. This field is read-only.
    #[prost(string, tag = "2")]
    pub project_id: ::prost::alloc::string::String,
    /// The resource name of the billing account associated with the project, if
    /// any. For example, `billingAccounts/012345-567890-ABCDEF`.
    #[prost(string, tag = "3")]
    pub billing_account_name: ::prost::alloc::string::String,
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
    pub name: ::prost::alloc::string::String,
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
    pub page_token: ::prost::alloc::string::String,
    /// Options for how to filter the returned billing accounts.
    /// Currently this only supports filtering for
    /// \[subaccounts\](<https://cloud.google.com/billing/docs/concepts>) under a
    /// single provided reseller billing account.
    /// (e.g. "master_billing_account=billingAccounts/012345-678901-ABCDEF").
    /// Boolean algebra and other fields are not currently supported.
    #[prost(string, tag = "3")]
    pub filter: ::prost::alloc::string::String,
}
/// Response message for `ListBillingAccounts`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListBillingAccountsResponse {
    /// A list of billing accounts.
    #[prost(message, repeated, tag = "1")]
    pub billing_accounts: ::prost::alloc::vec::Vec<BillingAccount>,
    /// A token to retrieve the next page of results. To retrieve the next page,
    /// call `ListBillingAccounts` again with the `page_token` field set to this
    /// value. This field is empty if there are no more results to retrieve.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for `CreateBillingAccount`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateBillingAccountRequest {
    /// Required. The billing account resource to create.
    /// Currently CreateBillingAccount only supports subaccount creation, so
    /// any created billing accounts must be under a provided master billing
    /// account.
    #[prost(message, optional, tag = "1")]
    pub billing_account: ::core::option::Option<BillingAccount>,
}
/// Request message for `UpdateBillingAccount`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateBillingAccountRequest {
    /// Required. The name of the billing account resource to be updated.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Required. The billing account resource to replace the resource on the server.
    #[prost(message, optional, tag = "2")]
    pub account: ::core::option::Option<BillingAccount>,
    /// The update mask applied to the resource.
    /// Only "display_name" is currently supported.
    #[prost(message, optional, tag = "3")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request message for `ListProjectBillingInfo`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListProjectBillingInfoRequest {
    /// Required. The resource name of the billing account associated with the projects that
    /// you want to list. For example, `billingAccounts/012345-567890-ABCDEF`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Requested page size. The maximum page size is 100; this is also the
    /// default.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// A token identifying a page of results to be returned. This should be a
    /// `next_page_token` value returned from a previous `ListProjectBillingInfo`
    /// call. If unspecified, the first page of results is returned.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Request message for `ListProjectBillingInfoResponse`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListProjectBillingInfoResponse {
    /// A list of `ProjectBillingInfo` resources representing the projects
    /// associated with the billing account.
    #[prost(message, repeated, tag = "1")]
    pub project_billing_info: ::prost::alloc::vec::Vec<ProjectBillingInfo>,
    /// A token to retrieve the next page of results. To retrieve the next page,
    /// call `ListProjectBillingInfo` again with the `page_token` field set to this
    /// value. This field is empty if there are no more results to retrieve.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for `GetProjectBillingInfo`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetProjectBillingInfoRequest {
    /// Required. The resource name of the project for which billing information is
    /// retrieved. For example, `projects/tokyo-rain-123`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request message for `UpdateProjectBillingInfo`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateProjectBillingInfoRequest {
    /// Required. The resource name of the project associated with the billing information
    /// that you want to update. For example, `projects/tokyo-rain-123`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The new billing information for the project. Read-only fields are ignored;
    /// thus, you can leave empty all fields except `billing_account_name`.
    #[prost(message, optional, tag = "2")]
    pub project_billing_info: ::core::option::Option<ProjectBillingInfo>,
}
#[doc = r" Generated client implementations."]
pub mod cloud_billing_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Retrieves GCP Console billing accounts and associates them with projects."]
    #[derive(Debug, Clone)]
    pub struct CloudBillingClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> CloudBillingClient<T>
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
        ) -> CloudBillingClient<InterceptedService<T, F>>
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
            CloudBillingClient::new(InterceptedService::new(inner, interceptor))
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
}
/// Encapsulates a single service in Google Cloud Platform.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Service {
    /// The resource name for the service.
    /// Example: "services/DA34-426B-A397"
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The identifier for the service.
    /// Example: "DA34-426B-A397"
    #[prost(string, tag = "2")]
    pub service_id: ::prost::alloc::string::String,
    /// A human readable display name for this service.
    #[prost(string, tag = "3")]
    pub display_name: ::prost::alloc::string::String,
    /// The business under which the service is offered.
    /// Ex. "businessEntities/GCP", "businessEntities/Maps"
    #[prost(string, tag = "4")]
    pub business_entity_name: ::prost::alloc::string::String,
}
/// Encapsulates a single SKU in Google Cloud Platform
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Sku {
    /// The resource name for the SKU.
    /// Example: "services/DA34-426B-A397/skus/AA95-CD31-42FE"
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// The identifier for the SKU.
    /// Example: "AA95-CD31-42FE"
    #[prost(string, tag = "2")]
    pub sku_id: ::prost::alloc::string::String,
    /// A human readable description of the SKU, has a maximum length of 256
    /// characters.
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    /// The category hierarchy of this SKU, purely for organizational purpose.
    #[prost(message, optional, tag = "4")]
    pub category: ::core::option::Option<Category>,
    /// List of service regions this SKU is offered at.
    /// Example: "asia-east1"
    /// Service regions can be found at <https://cloud.google.com/about/locations/>
    #[prost(string, repeated, tag = "5")]
    pub service_regions: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// A timeline of pricing info for this SKU in chronological order.
    #[prost(message, repeated, tag = "6")]
    pub pricing_info: ::prost::alloc::vec::Vec<PricingInfo>,
    /// Identifies the service provider.
    /// This is 'Google' for first party services in Google Cloud Platform.
    #[prost(string, tag = "7")]
    pub service_provider_name: ::prost::alloc::string::String,
}
/// Represents the category hierarchy of a SKU.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Category {
    /// The display name of the service this SKU belongs to.
    #[prost(string, tag = "1")]
    pub service_display_name: ::prost::alloc::string::String,
    /// The type of product the SKU refers to.
    /// Example: "Compute", "Storage", "Network", "ApplicationServices" etc.
    #[prost(string, tag = "2")]
    pub resource_family: ::prost::alloc::string::String,
    /// A group classification for related SKUs.
    /// Example: "RAM", "GPU", "Prediction", "Ops", "GoogleEgress" etc.
    #[prost(string, tag = "3")]
    pub resource_group: ::prost::alloc::string::String,
    /// Represents how the SKU is consumed.
    /// Example: "OnDemand", "Preemptible", "Commit1Mo", "Commit1Yr" etc.
    #[prost(string, tag = "4")]
    pub usage_type: ::prost::alloc::string::String,
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
    pub effective_time: ::core::option::Option<::prost_types::Timestamp>,
    /// An optional human readable summary of the pricing information, has a
    /// maximum length of 256 characters.
    #[prost(string, tag = "2")]
    pub summary: ::prost::alloc::string::String,
    /// Expresses the pricing formula. See `PricingExpression` for an example.
    #[prost(message, optional, tag = "3")]
    pub pricing_expression: ::core::option::Option<PricingExpression>,
    /// Aggregation Info. This can be left unspecified if the pricing expression
    /// doesn't require aggregation.
    #[prost(message, optional, tag = "4")]
    pub aggregation_info: ::core::option::Option<AggregationInfo>,
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
    pub usage_unit: ::prost::alloc::string::String,
    /// The unit of usage in human readable form.
    /// Example: "gibi byte".
    #[prost(string, tag = "4")]
    pub usage_unit_description: ::prost::alloc::string::String,
    /// The base unit for the SKU which is the unit used in usage exports.
    /// Example: "By"
    #[prost(string, tag = "5")]
    pub base_unit: ::prost::alloc::string::String,
    /// The base unit in human readable form.
    /// Example: "byte".
    #[prost(string, tag = "6")]
    pub base_unit_description: ::prost::alloc::string::String,
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
    pub tiered_rates: ::prost::alloc::vec::Vec<pricing_expression::TierRate>,
}
/// Nested message and enum types in `PricingExpression`.
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
        pub unit_price: ::core::option::Option<super::super::super::super::r#type::Money>,
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
/// Nested message and enum types in `AggregationInfo`.
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
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for `ListServices`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListServicesResponse {
    /// A list of services.
    #[prost(message, repeated, tag = "1")]
    pub services: ::prost::alloc::vec::Vec<Service>,
    /// A token to retrieve the next page of results. To retrieve the next page,
    /// call `ListServices` again with the `page_token` field set to this
    /// value. This field is empty if there are no more results to retrieve.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request message for `ListSkus`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSkusRequest {
    /// Required. The name of the service.
    /// Example: "services/DA34-426B-A397"
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional inclusive start time of the time range for which the pricing
    /// versions will be returned. Timestamps in the future are not allowed.
    /// The time range has to be within a single calendar month in
    /// America/Los_Angeles timezone. Time range as a whole is optional. If not
    /// specified, the latest pricing will be returned (up to 12 hours old at
    /// most).
    #[prost(message, optional, tag = "2")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Optional exclusive end time of the time range for which the pricing
    /// versions will be returned. Timestamps in the future are not allowed.
    /// The time range has to be within a single calendar month in
    /// America/Los_Angeles timezone. Time range as a whole is optional. If not
    /// specified, the latest pricing will be returned (up to 12 hours old at
    /// most).
    #[prost(message, optional, tag = "3")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// The ISO 4217 currency code for the pricing info in the response proto.
    /// Will use the conversion rate as of start_time.
    /// Optional. If not specified USD will be used.
    #[prost(string, tag = "4")]
    pub currency_code: ::prost::alloc::string::String,
    /// Requested page size. Defaults to 5000.
    #[prost(int32, tag = "5")]
    pub page_size: i32,
    /// A token identifying a page of results to return. This should be a
    /// `next_page_token` value returned from a previous `ListSkus`
    /// call. If unspecified, the first page of results is returned.
    #[prost(string, tag = "6")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response message for `ListSkus`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSkusResponse {
    /// The list of public SKUs of the given service.
    #[prost(message, repeated, tag = "1")]
    pub skus: ::prost::alloc::vec::Vec<Sku>,
    /// A token to retrieve the next page of results. To retrieve the next page,
    /// call `ListSkus` again with the `page_token` field set to this
    /// value. This field is empty if there are no more results to retrieve.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
#[doc = r" Generated client implementations."]
pub mod cloud_catalog_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " A catalog of Google Cloud Platform services and SKUs."]
    #[doc = " Provides pricing information and metadata on Google Cloud Platform services"]
    #[doc = " and SKUs."]
    #[derive(Debug, Clone)]
    pub struct CloudCatalogClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> CloudCatalogClient<T>
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
        ) -> CloudCatalogClient<InterceptedService<T, F>>
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
            CloudCatalogClient::new(InterceptedService::new(inner, interceptor))
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
}
