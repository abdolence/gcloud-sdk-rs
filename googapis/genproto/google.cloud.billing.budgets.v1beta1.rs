/// A budget is a plan that describes what you expect to spend on Cloud
/// projects, plus the rules to execute as spend is tracked against that plan,
/// (for example, send an alert when 90% of the target spend is met).
/// Currently all plans are monthly budgets so the usage period(s) tracked are
/// implied (calendar months of usage back-to-back).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Budget {
    /// Output only. Resource name of the budget.
    /// The resource name implies the scope of a budget. Values are of the form
    /// `billingAccounts/{billingAccountId}/budgets/{budgetId}`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// User data for display name in UI.
    /// Validation: <= 60 chars.
    #[prost(string, tag = "2")]
    pub display_name: std::string::String,
    /// Optional. Filters that define which resources are used to compute
    /// the actual spend against the budget.
    #[prost(message, optional, tag = "3")]
    pub budget_filter: ::std::option::Option<Filter>,
    /// Required. Budgeted amount.
    #[prost(message, optional, tag = "4")]
    pub amount: ::std::option::Option<BudgetAmount>,
    /// Optional. Rules that trigger alerts (notifications of thresholds
    /// being crossed) when spend exceeds the specified percentages of the budget.
    #[prost(message, repeated, tag = "5")]
    pub threshold_rules: ::std::vec::Vec<ThresholdRule>,
    /// Optional. Rules to apply to all updates to the actual spend, regardless
    /// of the thresholds set in `threshold_rules`.
    #[prost(message, optional, tag = "6")]
    pub all_updates_rule: ::std::option::Option<AllUpdatesRule>,
    /// Optional. Etag to validate that the object is unchanged for a
    /// read-modify-write operation.
    /// An empty etag will cause an update to overwrite other changes.
    #[prost(string, tag = "7")]
    pub etag: std::string::String,
}
/// The budgeted amount for each usage period.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BudgetAmount {
    /// Specification for what amount to use as the budget.
    #[prost(oneof = "budget_amount::BudgetAmount", tags = "1, 2")]
    pub budget_amount: ::std::option::Option<budget_amount::BudgetAmount>,
}
pub mod budget_amount {
    /// Specification for what amount to use as the budget.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum BudgetAmount {
        /// A specified amount to use as the budget.
        /// `currency_code` is optional. If specified, it must match the
        /// currency of the billing account. The `currency_code` is provided on
        /// output.
        #[prost(message, tag = "1")]
        SpecifiedAmount(super::super::super::super::super::r#type::Money),
        /// Use the last period's actual spend as the budget for the present period.
        #[prost(message, tag = "2")]
        LastPeriodAmount(super::LastPeriodAmount),
    }
}
/// Describes a budget amount targeted to last period's spend.
/// At this time, the amount is automatically 100% of last period's spend;
/// that is, there are no other options yet.
/// Future configuration will be described here (for example, configuring a
/// percentage of last period's spend).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LastPeriodAmount {}
/// ThresholdRule contains a definition of a threshold which triggers
/// an alert (a notification of a threshold being crossed) to be sent when
/// spend goes above the specified amount.
/// Alerts are automatically e-mailed to users with the Billing Account
/// Administrator role or the Billing Account User role.
/// The thresholds here have no effect on notifications sent to anything
/// configured under `Budget.all_updates_rule`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ThresholdRule {
    /// Required. Send an alert when this threshold is exceeded.
    /// This is a 1.0-based percentage, so 0.5 = 50%.
    /// Validation: non-negative number.
    #[prost(double, tag = "1")]
    pub threshold_percent: f64,
    /// Optional. The type of basis used to determine if spend has passed the
    /// threshold. Behavior defaults to CURRENT_SPEND if not set.
    #[prost(enumeration = "threshold_rule::Basis", tag = "2")]
    pub spend_basis: i32,
}
pub mod threshold_rule {
    /// The type of basis used to determine if spend has passed the threshold.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Basis {
        /// Unspecified threshold basis.
        Unspecified = 0,
        /// Use current spend as the basis for comparison against the threshold.
        CurrentSpend = 1,
        /// Use forecasted spend for the period as the basis for comparison against
        /// the threshold.
        ForecastedSpend = 2,
    }
}
/// AllUpdatesRule defines notifications that are sent on every update to the
/// billing account's spend, regardless of the thresholds defined using
/// threshold rules.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AllUpdatesRule {
    /// Required. The name of the Cloud Pub/Sub topic where budget related messages will be
    /// published, in the form `projects/{project_id}/topics/{topic_id}`. Updates
    /// are sent at regular intervals to the topic.
    /// The topic needs to be created before the budget is created; see
    /// https://cloud.google.com/billing/docs/how-to/budgets#manage-notifications
    /// for more details.
    /// Caller is expected to have
    /// `pubsub.topics.setIamPolicy` permission on the topic when it's set for a
    /// budget, otherwise, the API call will fail with PERMISSION_DENIED. See
    /// https://cloud.google.com/pubsub/docs/access-control for more details on
    /// Pub/Sub roles and permissions.
    #[prost(string, tag = "1")]
    pub pubsub_topic: std::string::String,
    /// Required. The schema version of the notification.
    /// Only "1.0" is accepted. It represents the JSON schema as defined in
    /// https://cloud.google.com/billing/docs/how-to/budgets#notification_format
    #[prost(string, tag = "2")]
    pub schema_version: std::string::String,
}
/// A filter for a budget, limiting the scope of the cost to calculate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Filter {
    /// Optional. A set of projects of the form `projects/{project}`,
    /// specifying that usage from only this set of projects should be
    /// included in the budget. If omitted, the report will include all usage for
    /// the billing account, regardless of which project the usage occurred on.
    /// Only zero or one project can be specified currently.
    #[prost(string, repeated, tag = "1")]
    pub projects: ::std::vec::Vec<std::string::String>,
    /// Optional. If not set, default behavior is `INCLUDE_ALL_CREDITS`.
    #[prost(enumeration = "filter::CreditTypesTreatment", tag = "4")]
    pub credit_types_treatment: i32,
    /// Optional. A set of services of the form `services/{service_id}`,
    /// specifying that usage from only this set of services should be
    /// included in the budget. If omitted, the report will include usage for
    /// all the services.
    /// The service names are available through the Catalog API:
    /// https://cloud.google.com/billing/v1/how-tos/catalog-api.
    #[prost(string, repeated, tag = "3")]
    pub services: ::std::vec::Vec<std::string::String>,
    /// Optional. A set of subaccounts of the form `billingAccounts/{account_id}`, specifying
    /// that usage from only this set of subaccounts should be included in the
    /// budget. If a subaccount is set to the name of the master account, usage
    /// from the master account will be included. If omitted, the report will
    /// include usage from the master account and all subaccounts, if they exist.
    #[prost(string, repeated, tag = "5")]
    pub subaccounts: ::std::vec::Vec<std::string::String>,
    /// Optional. A single label and value pair specifying that usage from only this set of
    /// labeled resources should be included in the budget. Multiple entries or
    /// multiple values per entry are not allowed. If omitted, the report will
    /// include all labeled and unlabeled usage.
    #[prost(map = "string, message", tag = "6")]
    pub labels: ::std::collections::HashMap<std::string::String, ::prost_types::ListValue>,
}
pub mod filter {
    /// Specifies how credits should be treated when determining spend for
    /// threshold calculations.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum CreditTypesTreatment {
        Unspecified = 0,
        /// All types of credit are subtracted from the gross cost to determine the
        /// spend for threshold calculations.
        IncludeAllCredits = 1,
        /// All types of credit are added to the net cost to determine the spend for
        /// threshold calculations.
        ExcludeAllCredits = 2,
    }
}
/// Request for CreateBudget
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateBudgetRequest {
    /// Required. The name of the billing account to create the budget in. Values
    /// are of the form `billingAccounts/{billingAccountId}`.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Required. Budget to create.
    #[prost(message, optional, tag = "2")]
    pub budget: ::std::option::Option<Budget>,
}
/// Request for UpdateBudget
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateBudgetRequest {
    /// Required. The updated budget object.
    /// The budget to update is specified by the budget name in the budget.
    #[prost(message, optional, tag = "1")]
    pub budget: ::std::option::Option<Budget>,
    /// Optional. Indicates which fields in the provided budget to update.
    /// Read-only fields (such as `name`) cannot be changed. If this is not
    /// provided, then only fields with non-default values from the request are
    /// updated. See
    /// https://developers.google.com/protocol-buffers/docs/proto3#default for more
    /// details about default values.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
}
/// Request for GetBudget
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBudgetRequest {
    /// Required. Name of budget to get. Values are of the form
    /// `billingAccounts/{billingAccountId}/budgets/{budgetId}`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Request for ListBudgets
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListBudgetsRequest {
    /// Required. Name of billing account to list budgets under. Values
    /// are of the form `billingAccounts/{billingAccountId}`.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Optional. The maximum number of budgets to return per page.
    /// The default and maximum value are 100.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. The value returned by the last `ListBudgetsResponse` which
    /// indicates that this is a continuation of a prior `ListBudgets` call,
    /// and that the system should return the next page of data.
    #[prost(string, tag = "3")]
    pub page_token: std::string::String,
}
/// Response for ListBudgets
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListBudgetsResponse {
    /// List of the budgets owned by the requested billing account.
    #[prost(message, repeated, tag = "1")]
    pub budgets: ::std::vec::Vec<Budget>,
    /// If not empty, indicates that there may be more budgets that match the
    /// request; this value should be passed in a new `ListBudgetsRequest`.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
}
/// Request for DeleteBudget
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteBudgetRequest {
    /// Required. Name of the budget to delete. Values are of the form
    /// `billingAccounts/{billingAccountId}/budgets/{budgetId}`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
#[doc = r" Generated client implementations."]
pub mod budget_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " BudgetService stores Cloud Billing budgets, which define a"]
    #[doc = " budget plan and rules to execute as we track spend against that plan."]
    pub struct BudgetServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl BudgetServiceClient<tonic::transport::Channel> {
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
    impl<T> BudgetServiceClient<T>
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
        #[doc = " Creates a new budget. See"]
        #[doc = " <a href=\"https://cloud.google.com/billing/quotas\">Quotas and limits</a>"]
        #[doc = " for more information on the limits of the number of budgets you can create."]
        pub async fn create_budget(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateBudgetRequest>,
        ) -> Result<tonic::Response<super::Budget>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.billing.budgets.v1beta1.BudgetService/CreateBudget",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates a budget and returns the updated budget."]
        #[doc = ""]
        #[doc = " WARNING: There are some fields exposed on the Google Cloud Console that"]
        #[doc = " aren't available on this API. Budget fields that are not exposed in"]
        #[doc = " this API will not be changed by this method."]
        pub async fn update_budget(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateBudgetRequest>,
        ) -> Result<tonic::Response<super::Budget>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.billing.budgets.v1beta1.BudgetService/UpdateBudget",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns a budget."]
        #[doc = ""]
        #[doc = " WARNING: There are some fields exposed on the Google Cloud Console that"]
        #[doc = " aren't available on this API. When reading from the API, you will not"]
        #[doc = " see these fields in the return value, though they may have been set"]
        #[doc = " in the Cloud Console."]
        pub async fn get_budget(
            &mut self,
            request: impl tonic::IntoRequest<super::GetBudgetRequest>,
        ) -> Result<tonic::Response<super::Budget>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.billing.budgets.v1beta1.BudgetService/GetBudget",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns a list of budgets for a billing account."]
        #[doc = ""]
        #[doc = " WARNING: There are some fields exposed on the Google Cloud Console that"]
        #[doc = " aren't available on this API. When reading from the API, you will not"]
        #[doc = " see these fields in the return value, though they may have been set"]
        #[doc = " in the Cloud Console."]
        pub async fn list_budgets(
            &mut self,
            request: impl tonic::IntoRequest<super::ListBudgetsRequest>,
        ) -> Result<tonic::Response<super::ListBudgetsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.billing.budgets.v1beta1.BudgetService/ListBudgets",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes a budget. Returns successfully if already deleted."]
        pub async fn delete_budget(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteBudgetRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.billing.budgets.v1beta1.BudgetService/DeleteBudget",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for BudgetServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for BudgetServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "BudgetServiceClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod budget_service_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with BudgetServiceServer."]
    #[async_trait]
    pub trait BudgetService: Send + Sync + 'static {
        #[doc = " Creates a new budget. See"]
        #[doc = " <a href=\"https://cloud.google.com/billing/quotas\">Quotas and limits</a>"]
        #[doc = " for more information on the limits of the number of budgets you can create."]
        async fn create_budget(
            &self,
            request: tonic::Request<super::CreateBudgetRequest>,
        ) -> Result<tonic::Response<super::Budget>, tonic::Status>;
        #[doc = " Updates a budget and returns the updated budget."]
        #[doc = ""]
        #[doc = " WARNING: There are some fields exposed on the Google Cloud Console that"]
        #[doc = " aren't available on this API. Budget fields that are not exposed in"]
        #[doc = " this API will not be changed by this method."]
        async fn update_budget(
            &self,
            request: tonic::Request<super::UpdateBudgetRequest>,
        ) -> Result<tonic::Response<super::Budget>, tonic::Status>;
        #[doc = " Returns a budget."]
        #[doc = ""]
        #[doc = " WARNING: There are some fields exposed on the Google Cloud Console that"]
        #[doc = " aren't available on this API. When reading from the API, you will not"]
        #[doc = " see these fields in the return value, though they may have been set"]
        #[doc = " in the Cloud Console."]
        async fn get_budget(
            &self,
            request: tonic::Request<super::GetBudgetRequest>,
        ) -> Result<tonic::Response<super::Budget>, tonic::Status>;
        #[doc = " Returns a list of budgets for a billing account."]
        #[doc = ""]
        #[doc = " WARNING: There are some fields exposed on the Google Cloud Console that"]
        #[doc = " aren't available on this API. When reading from the API, you will not"]
        #[doc = " see these fields in the return value, though they may have been set"]
        #[doc = " in the Cloud Console."]
        async fn list_budgets(
            &self,
            request: tonic::Request<super::ListBudgetsRequest>,
        ) -> Result<tonic::Response<super::ListBudgetsResponse>, tonic::Status>;
        #[doc = " Deletes a budget. Returns successfully if already deleted."]
        async fn delete_budget(
            &self,
            request: tonic::Request<super::DeleteBudgetRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status>;
    }
    #[doc = " BudgetService stores Cloud Billing budgets, which define a"]
    #[doc = " budget plan and rules to execute as we track spend against that plan."]
    #[derive(Debug)]
    #[doc(hidden)]
    pub struct BudgetServiceServer<T: BudgetService> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: BudgetService> BudgetServiceServer<T> {
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
    impl<T, B> Service<http::Request<B>> for BudgetServiceServer<T>
    where
        T: BudgetService,
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
                "/google.cloud.billing.budgets.v1beta1.BudgetService/CreateBudget" => {
                    #[allow(non_camel_case_types)]
                    struct CreateBudgetSvc<T: BudgetService>(pub Arc<T>);
                    impl<T: BudgetService> tonic::server::UnaryService<super::CreateBudgetRequest>
                        for CreateBudgetSvc<T>
                    {
                        type Response = super::Budget;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateBudgetRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.create_budget(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = CreateBudgetSvc(inner);
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
                "/google.cloud.billing.budgets.v1beta1.BudgetService/UpdateBudget" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateBudgetSvc<T: BudgetService>(pub Arc<T>);
                    impl<T: BudgetService> tonic::server::UnaryService<super::UpdateBudgetRequest>
                        for UpdateBudgetSvc<T>
                    {
                        type Response = super::Budget;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateBudgetRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.update_budget(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = UpdateBudgetSvc(inner);
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
                "/google.cloud.billing.budgets.v1beta1.BudgetService/GetBudget" => {
                    #[allow(non_camel_case_types)]
                    struct GetBudgetSvc<T: BudgetService>(pub Arc<T>);
                    impl<T: BudgetService> tonic::server::UnaryService<super::GetBudgetRequest> for GetBudgetSvc<T> {
                        type Response = super::Budget;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetBudgetRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.get_budget(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetBudgetSvc(inner);
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
                "/google.cloud.billing.budgets.v1beta1.BudgetService/ListBudgets" => {
                    #[allow(non_camel_case_types)]
                    struct ListBudgetsSvc<T: BudgetService>(pub Arc<T>);
                    impl<T: BudgetService> tonic::server::UnaryService<super::ListBudgetsRequest>
                        for ListBudgetsSvc<T>
                    {
                        type Response = super::ListBudgetsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListBudgetsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.list_budgets(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ListBudgetsSvc(inner);
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
                "/google.cloud.billing.budgets.v1beta1.BudgetService/DeleteBudget" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteBudgetSvc<T: BudgetService>(pub Arc<T>);
                    impl<T: BudgetService> tonic::server::UnaryService<super::DeleteBudgetRequest>
                        for DeleteBudgetSvc<T>
                    {
                        type Response = ();
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteBudgetRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.delete_budget(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = DeleteBudgetSvc(inner);
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
    impl<T: BudgetService> Clone for BudgetServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: BudgetService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: BudgetService> tonic::transport::NamedService for BudgetServiceServer<T> {
        const NAME: &'static str = "google.cloud.billing.budgets.v1beta1.BudgetService";
    }
}
