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
    /// Optional. Rules to apply to notifications sent based on budget spend and
    /// thresholds.
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
/// AllUpdatesRule defines notifications that are sent based on budget spend
/// and thresholds.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AllUpdatesRule {
    /// Optional. The name of the Pub/Sub topic where budget related messages will
    /// be published, in the form `projects/{project_id}/topics/{topic_id}`.
    /// Updates are sent at regular intervals to the topic. The topic needs to be
    /// created before the budget is created; see
    /// https://cloud.google.com/billing/docs/how-to/budgets#manage-notifications
    /// for more details.
    /// Caller is expected to have
    /// `pubsub.topics.setIamPolicy` permission on the topic when it's set for a
    /// budget, otherwise, the API call will fail with PERMISSION_DENIED. See
    /// https://cloud.google.com/billing/docs/how-to/budgets-programmatic-notifications
    /// for more details on Pub/Sub roles and permissions.
    #[prost(string, tag = "1")]
    pub pubsub_topic: std::string::String,
    /// Optional. The schema version of the notification sent to `pubsub_topic`.
    /// Only "1.0" is accepted. It represents the JSON schema as defined in
    /// https://cloud.google.com/billing/docs/how-to/budgets-programmatic-notifications#notification_format
    #[prost(string, tag = "2")]
    pub schema_version: std::string::String,
    /// Optional. Targets to send notifications to when a threshold is exceeded.
    /// This is in addition to default recipients who have billing account IAM
    /// roles. The value is the full REST resource name of a monitoring
    /// notification channel with the form
    /// `projects/{project_id}/notificationChannels/{channel_id}`. A maximum of 5
    /// channels are allowed. See
    /// https://cloud.google.com/billing/docs/how-to/budgets-notification-recipients
    /// for more details.
    #[prost(string, repeated, tag = "3")]
    pub monitoring_notification_channels: ::std::vec::Vec<std::string::String>,
    /// Optional. When set to true, disables default notifications sent when a
    /// threshold is exceeded. Default notifications are sent to those with Billing
    /// Account Administrator and Billing Account User IAM roles for the target
    /// account.
    #[prost(bool, tag = "4")]
    pub disable_default_iam_recipients: bool,
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
    /// Optional. A list of credit types to be subtracted from gross cost to
    /// determine the spend for threshold calculations if and only if
    /// credit_types_treatment is INCLUDE_SPECIFIED_CREDITS. If
    /// credit_types_treatment is not INCLUDE_SPECIFIED_CREDITS, this field must be
    /// empty. See credits.type at
    /// https://cloud.google.com/billing/docs/how-to/export-data-bigquery-tables#data-schema
    /// for a list of acceptable credit type values in this field.
    #[prost(string, repeated, tag = "7")]
    pub credit_types: ::std::vec::Vec<std::string::String>,
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
    /// Optional. A set of subaccounts of the form `billingAccounts/{account_id}`,
    /// specifying that usage from only this set of subaccounts should be included
    /// in the budget. If a subaccount is set to the name of the parent account,
    /// usage from the parent account will be included. If omitted, the
    /// report will include usage from the parent account and all
    /// subaccounts, if they exist.
    #[prost(string, repeated, tag = "5")]
    pub subaccounts: ::std::vec::Vec<std::string::String>,
    /// Optional. A single label and value pair specifying that usage from only
    /// this set of labeled resources should be included in the budget. Currently,
    /// multiple entries or multiple values per entry are not allowed. If omitted,
    /// the report will include all labeled and unlabeled usage.
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
        /// Credit types specified in the credit_types field are subtracted from the
        /// gross cost to determine the spend for threshold calculations.
        IncludeSpecifiedCredits = 3,
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
                "/google.cloud.billing.budgets.v1alpha1.BudgetService/CreateBudget",
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
                "/google.cloud.billing.budgets.v1alpha1.BudgetService/UpdateBudget",
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
                "/google.cloud.billing.budgets.v1alpha1.BudgetService/GetBudget",
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
                "/google.cloud.billing.budgets.v1alpha1.BudgetService/ListBudgets",
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
                "/google.cloud.billing.budgets.v1alpha1.BudgetService/DeleteBudget",
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
