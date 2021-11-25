/// A budget is a plan that describes what you expect to spend on Cloud
/// projects, plus the rules to execute as spend is tracked against that plan,
/// (for example, send an alert when 90% of the target spend is met).
/// The budget time period is configurable, with options such as month (default),
/// quarter, year, or custom time period.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Budget {
    /// Output only. Resource name of the budget.
    /// The resource name implies the scope of a budget. Values are of the form
    /// `billingAccounts/{billingAccountId}/budgets/{budgetId}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// User data for display name in UI. The name must be less than or equal to 60
    /// characters.
    #[prost(string, tag = "2")]
    pub display_name: ::prost::alloc::string::String,
    /// Optional. Filters that define which resources are used to compute the
    /// actual spend against the budget amount, such as projects, services, and the
    /// budget's time period, as well as other filters.
    #[prost(message, optional, tag = "3")]
    pub budget_filter: ::core::option::Option<Filter>,
    /// Required. Budgeted amount.
    #[prost(message, optional, tag = "4")]
    pub amount: ::core::option::Option<BudgetAmount>,
    /// Optional. Rules that trigger alerts (notifications of thresholds
    /// being crossed) when spend exceeds the specified percentages of the budget.
    #[prost(message, repeated, tag = "5")]
    pub threshold_rules: ::prost::alloc::vec::Vec<ThresholdRule>,
    /// Optional. Rules to apply to notifications sent based on budget spend and
    /// thresholds.
    #[prost(message, optional, tag = "6")]
    pub notifications_rule: ::core::option::Option<NotificationsRule>,
    /// Optional. Etag to validate that the object is unchanged for a
    /// read-modify-write operation.
    /// An empty etag will cause an update to overwrite other changes.
    #[prost(string, tag = "7")]
    pub etag: ::prost::alloc::string::String,
}
/// The budgeted amount for each usage period.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BudgetAmount {
    /// Specification for what amount to use as the budget.
    #[prost(oneof = "budget_amount::BudgetAmount", tags = "1, 2")]
    pub budget_amount: ::core::option::Option<budget_amount::BudgetAmount>,
}
/// Nested message and enum types in `BudgetAmount`.
pub mod budget_amount {
    /// Specification for what amount to use as the budget.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum BudgetAmount {
        /// A specified amount to use as the budget.
        /// `currency_code` is optional. If specified when creating a budget, it must
        /// match the currency of the billing account. If specified when updating a
        /// budget, it must match the currency_code of the existing budget.
        /// The `currency_code` is provided on output.
        #[prost(message, tag = "1")]
        SpecifiedAmount(super::super::super::super::super::r#type::Money),
        /// Use the last period's actual spend as the budget for the present period.
        /// LastPeriodAmount can only be set when the budget's time period is a
        /// \[Filter.calendar_period][google.cloud.billing.budgets.v1.Filter.calendar_period\].
        /// It cannot be set in combination with
        /// \[Filter.custom_period][google.cloud.billing.budgets.v1.Filter.custom_period\].
        #[prost(message, tag = "2")]
        LastPeriodAmount(super::LastPeriodAmount),
    }
}
/// Describes a budget amount targeted to the last
/// \[Filter.calendar_period][google.cloud.billing.budgets.v1.Filter.calendar_period\]
/// spend. At this time, the amount is automatically 100% of the last calendar
/// period's spend; that is, there are no other options yet.
/// Future configuration options will be described here (for example, configuring
/// a percentage of last period's spend).
/// LastPeriodAmount cannot be set for a budget configured with
/// a
/// \[Filter.custom_period][google.cloud.billing.budgets.v1.Filter.custom_period\].
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
/// Nested message and enum types in `ThresholdRule`.
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
        /// FORECASTED_SPEND can only be set when the budget's time period is a
        /// \[Filter.calendar_period][google.cloud.billing.budgets.v1.Filter.calendar_period\].
        /// It cannot be set in combination with
        /// \[Filter.custom_period][google.cloud.billing.budgets.v1.Filter.custom_period\].
        ForecastedSpend = 2,
    }
}
/// NotificationsRule defines notifications that are sent based on budget spend
/// and thresholds.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NotificationsRule {
    /// Optional. The name of the Pub/Sub topic where budget related messages will
    /// be published, in the form `projects/{project_id}/topics/{topic_id}`.
    /// Updates are sent at regular intervals to the topic. The topic needs to be
    /// created before the budget is created; see
    /// <https://cloud.google.com/billing/docs/how-to/budgets#manage-notifications>
    /// for more details.
    /// Caller is expected to have
    /// `pubsub.topics.setIamPolicy` permission on the topic when it's set for a
    /// budget, otherwise, the API call will fail with PERMISSION_DENIED. See
    /// <https://cloud.google.com/billing/docs/how-to/budgets-programmatic-notifications>
    /// for more details on Pub/Sub roles and permissions.
    #[prost(string, tag = "1")]
    pub pubsub_topic: ::prost::alloc::string::String,
    /// Optional. Required when
    /// \[NotificationsRule.pubsub_topic][google.cloud.billing.budgets.v1.NotificationsRule.pubsub_topic\]
    /// is set. The schema version of the notification sent to
    /// \[NotificationsRule.pubsub_topic][google.cloud.billing.budgets.v1.NotificationsRule.pubsub_topic\].
    /// Only "1.0" is accepted. It represents the JSON schema as defined in
    /// <https://cloud.google.com/billing/docs/how-to/budgets-programmatic-notifications#notification_format.>
    #[prost(string, tag = "2")]
    pub schema_version: ::prost::alloc::string::String,
    /// Optional. Targets to send notifications to when a threshold is exceeded.
    /// This is in addition to default recipients who have billing account IAM
    /// roles. The value is the full REST resource name of a monitoring
    /// notification channel with the form
    /// `projects/{project_id}/notificationChannels/{channel_id}`. A maximum of 5
    /// channels are allowed. See
    /// <https://cloud.google.com/billing/docs/how-to/budgets-notification-recipients>
    /// for more details.
    #[prost(string, repeated, tag = "3")]
    pub monitoring_notification_channels: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
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
    pub projects: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Optional. If
    /// \[Filter.credit_types_treatment][google.cloud.billing.budgets.v1.Filter.credit_types_treatment\]
    /// is INCLUDE_SPECIFIED_CREDITS, this is a list of credit types to be
    /// subtracted from gross cost to determine the spend for threshold
    /// calculations. See [a list of acceptable credit type
    /// values](<https://cloud.google.com/billing/docs/how-to/export-data-bigquery-tables#credits-type>).
    ///
    /// If
    /// \[Filter.credit_types_treatment][google.cloud.billing.budgets.v1.Filter.credit_types_treatment\]
    /// is **not** INCLUDE_SPECIFIED_CREDITS, this field must be empty.
    #[prost(string, repeated, tag = "7")]
    pub credit_types: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Optional. If not set, default behavior is `INCLUDE_ALL_CREDITS`.
    #[prost(enumeration = "filter::CreditTypesTreatment", tag = "4")]
    pub credit_types_treatment: i32,
    /// Optional. A set of services of the form `services/{service_id}`,
    /// specifying that usage from only this set of services should be
    /// included in the budget. If omitted, the report will include usage for
    /// all the services.
    /// The service names are available through the Catalog API:
    /// <https://cloud.google.com/billing/v1/how-tos/catalog-api.>
    #[prost(string, repeated, tag = "3")]
    pub services: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Optional. A set of subaccounts of the form `billingAccounts/{account_id}`,
    /// specifying that usage from only this set of subaccounts should be included
    /// in the budget. If a subaccount is set to the name of the parent account,
    /// usage from the parent account will be included. If the field is omitted,
    /// the report will include usage from the parent account and all subaccounts,
    /// if they exist.
    #[prost(string, repeated, tag = "5")]
    pub subaccounts: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Optional. A single label and value pair specifying that usage from only
    /// this set of labeled resources should be included in the budget. Currently,
    /// multiple entries or multiple values per entry are not allowed. If omitted,
    /// the report will include all labeled and unlabeled usage.
    #[prost(map = "string, message", tag = "6")]
    pub labels:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost_types::ListValue>,
    /// Multiple options to choose the budget's time period, specifying that only
    /// usage that occurs during this time period should be included in the budget.
    /// If not set, the `usage_period` defaults to CalendarPeriod.MONTH.
    #[prost(oneof = "filter::UsagePeriod", tags = "8, 9")]
    pub usage_period: ::core::option::Option<filter::UsagePeriod>,
}
/// Nested message and enum types in `Filter`.
pub mod filter {
    /// Specifies how credits are applied when determining the spend for
    /// threshold calculations. Budgets track the total cost minus any applicable
    /// selected credits.
    /// [See the documentation for a list of credit
    /// types](<https://cloud.google.com/billing/docs/how-to/export-data-bigquery-tables#credits-type>).
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
        /// [Credit
        /// types](<https://cloud.google.com/billing/docs/how-to/export-data-bigquery-tables#credits-type>)
        /// specified in the credit_types field are subtracted from the
        /// gross cost to determine the spend for threshold calculations.
        IncludeSpecifiedCredits = 3,
    }
    /// Multiple options to choose the budget's time period, specifying that only
    /// usage that occurs during this time period should be included in the budget.
    /// If not set, the `usage_period` defaults to CalendarPeriod.MONTH.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum UsagePeriod {
        /// Optional. Specifies to track usage for recurring calendar period.
        /// For example, assume that CalendarPeriod.QUARTER is set. The budget will
        /// track usage from April 1 to June 30, when the current calendar month is
        /// April, May, June. After that, it will track usage from July 1 to
        /// September 30 when the current calendar month is July, August, September,
        /// so on.
        #[prost(enumeration = "super::CalendarPeriod", tag = "8")]
        CalendarPeriod(i32),
        /// Optional. Specifies to track usage from any start date (required) to any
        /// end date (optional). This time period is static, it does not recur.
        #[prost(message, tag = "9")]
        CustomPeriod(super::CustomPeriod),
    }
}
/// All date times begin at 12 AM US and Canadian Pacific Time (UTC-8).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomPeriod {
    /// Required. The start date must be after January 1, 2017.
    #[prost(message, optional, tag = "1")]
    pub start_date: ::core::option::Option<super::super::super::super::r#type::Date>,
    /// Optional. The end date of the time period. Budgets with elapsed end date
    /// won't be processed. If unset, specifies to track all usage incurred since
    /// the start_date.
    #[prost(message, optional, tag = "2")]
    pub end_date: ::core::option::Option<super::super::super::super::r#type::Date>,
}
/// A `CalendarPeriod` represents the abstract concept of a time period that
/// has a canonical start. Grammatically, "the start of the current
/// `CalendarPeriod`". All calendar times begin at 12 AM US and Canadian
/// Pacific Time (UTC-8).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CalendarPeriod {
    Unspecified = 0,
    /// A month. Month starts on the first day of each month, such as January 1,
    /// February 1, March 1, and so on.
    Month = 1,
    /// A quarter. Quarters start on dates January 1, April 1, July 1, and October
    /// 1 of each year.
    Quarter = 2,
    /// A year. Year starts on January 1.
    Year = 3,
}
/// Request for CreateBudget
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateBudgetRequest {
    /// Required. The name of the billing account to create the budget in. Values
    /// are of the form `billingAccounts/{billingAccountId}`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Required. Budget to create.
    #[prost(message, optional, tag = "2")]
    pub budget: ::core::option::Option<Budget>,
}
/// Request for UpdateBudget
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateBudgetRequest {
    /// Required. The updated budget object.
    /// The budget to update is specified by the budget name in the budget.
    #[prost(message, optional, tag = "1")]
    pub budget: ::core::option::Option<Budget>,
    /// Optional. Indicates which fields in the provided budget to update.
    /// Read-only fields (such as `name`) cannot be changed. If this is not
    /// provided, then only fields with non-default values from the request are
    /// updated. See
    /// <https://developers.google.com/protocol-buffers/docs/proto3#default> for more
    /// details about default values.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::core::option::Option<::prost_types::FieldMask>,
}
/// Request for GetBudget
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBudgetRequest {
    /// Required. Name of budget to get. Values are of the form
    /// `billingAccounts/{billingAccountId}/budgets/{budgetId}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// Request for ListBudgets
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListBudgetsRequest {
    /// Required. Name of billing account to list budgets under. Values
    /// are of the form `billingAccounts/{billingAccountId}`.
    #[prost(string, tag = "1")]
    pub parent: ::prost::alloc::string::String,
    /// Optional. The maximum number of budgets to return per page.
    /// The default and maximum value are 100.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. The value returned by the last `ListBudgetsResponse` which
    /// indicates that this is a continuation of a prior `ListBudgets` call,
    /// and that the system should return the next page of data.
    #[prost(string, tag = "3")]
    pub page_token: ::prost::alloc::string::String,
}
/// Response for ListBudgets
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListBudgetsResponse {
    /// List of the budgets owned by the requested billing account.
    #[prost(message, repeated, tag = "1")]
    pub budgets: ::prost::alloc::vec::Vec<Budget>,
    /// If not empty, indicates that there may be more budgets that match the
    /// request; this value should be passed in a new `ListBudgetsRequest`.
    #[prost(string, tag = "2")]
    pub next_page_token: ::prost::alloc::string::String,
}
/// Request for DeleteBudget
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteBudgetRequest {
    /// Required. Name of the budget to delete. Values are of the form
    /// `billingAccounts/{billingAccountId}/budgets/{budgetId}`.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
#[doc = r" Generated client implementations."]
pub mod budget_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " BudgetService stores Cloud Billing budgets, which define a"]
    #[doc = " budget plan and rules to execute as we track spend against that plan."]
    #[derive(Debug, Clone)]
    pub struct BudgetServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> BudgetServiceClient<T>
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
        ) -> BudgetServiceClient<InterceptedService<T, F>>
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
            BudgetServiceClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = " Creates a new budget. See"]
        #[doc = " [Quotas and limits](https://cloud.google.com/billing/quotas)"]
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
                "/google.cloud.billing.budgets.v1.BudgetService/CreateBudget",
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
                "/google.cloud.billing.budgets.v1.BudgetService/UpdateBudget",
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
                "/google.cloud.billing.budgets.v1.BudgetService/GetBudget",
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
                "/google.cloud.billing.budgets.v1.BudgetService/ListBudgets",
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
                "/google.cloud.billing.budgets.v1.BudgetService/DeleteBudget",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
