/// Request message for
/// [AccountBudgetProposalService.GetAccountBudgetProposal][google.ads.googleads.v3.services.AccountBudgetProposalService.GetAccountBudgetProposal].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAccountBudgetProposalRequest {
    /// Required. The resource name of the account-level budget proposal to fetch.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
/// Request message for
/// [AccountBudgetProposalService.MutateAccountBudgetProposal][google.ads.googleads.v3.services.AccountBudgetProposalService.MutateAccountBudgetProposal].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateAccountBudgetProposalRequest {
    /// Required. The ID of the customer.
    #[prost(string, tag = "1")]
    pub customer_id: std::string::String,
    /// Required. The operation to perform on an individual account-level budget proposal.
    #[prost(message, optional, tag = "2")]
    pub operation: ::std::option::Option<AccountBudgetProposalOperation>,
    /// If true, the request is validated but not executed. Only errors are
    /// returned, not results.
    #[prost(bool, tag = "3")]
    pub validate_only: bool,
}
/// A single operation to propose the creation of a new account-level budget or
/// edit/end/remove an existing one.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountBudgetProposalOperation {
    /// FieldMask that determines which budget fields are modified.  While budgets
    /// may be modified, proposals that propose such modifications are final.
    /// Therefore, update operations are not supported for proposals.
    ///
    /// Proposals that modify budgets have the 'update' proposal type.  Specifying
    /// a mask for any other proposal type is considered an error.
    #[prost(message, optional, tag = "3")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
    /// The mutate operation.
    #[prost(oneof = "account_budget_proposal_operation::Operation", tags = "2, 1")]
    pub operation: ::std::option::Option<account_budget_proposal_operation::Operation>,
}
pub mod account_budget_proposal_operation {
    /// The mutate operation.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Operation {
        /// Create operation: A new proposal to create a new budget, edit an
        /// existing budget, end an actively running budget, or remove an approved
        /// budget scheduled to start in the future.
        /// No resource name is expected for the new proposal.
        #[prost(message, tag = "2")]
        Create(super::super::resources::AccountBudgetProposal),
        /// Remove operation: A resource name for the removed proposal is expected,
        /// in this format:
        ///
        /// `customers/{customer_id}/accountBudgetProposals/{account_budget_proposal_id}`
        /// A request may be cancelled iff it is pending.
        #[prost(string, tag = "1")]
        Remove(std::string::String),
    }
}
/// Response message for account-level budget mutate operations.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateAccountBudgetProposalResponse {
    /// The result of the mutate.
    #[prost(message, optional, tag = "2")]
    pub result: ::std::option::Option<MutateAccountBudgetProposalResult>,
}
/// The result for the account budget proposal mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateAccountBudgetProposalResult {
    /// Returned for successful operations.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
#[doc = r" Generated client implementations."]
pub mod account_budget_proposal_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " A service for managing account-level budgets via proposals."]
    #[doc = ""]
    #[doc = " A proposal is a request to create a new budget or make changes to an"]
    #[doc = " existing one."]
    #[doc = ""]
    #[doc = " Reads for account-level budgets managed by these proposals will be"]
    #[doc = " supported in a future version. Until then, please use the"]
    #[doc = " BudgetOrderService from the AdWords API. Learn more at"]
    #[doc = " https://developers.google.com/adwords/api/docs/guides/budget-order"]
    #[doc = ""]
    #[doc = " Mutates:"]
    #[doc = " The CREATE operation creates a new proposal."]
    #[doc = " UPDATE operations aren't supported."]
    #[doc = " The REMOVE operation cancels a pending proposal."]
    pub struct AccountBudgetProposalServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> AccountBudgetProposalServiceClient<T>
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
        #[doc = " Returns an account-level budget proposal in full detail."]
        pub async fn get_account_budget_proposal(
            &mut self,
            request: impl tonic::IntoRequest<super::GetAccountBudgetProposalRequest>,
        ) -> Result<tonic::Response<super::super::resources::AccountBudgetProposal>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ( "/google.ads.googleads.v3.services.AccountBudgetProposalService/GetAccountBudgetProposal" ) ;
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates, updates, or removes account budget proposals.  Operation statuses"]
        #[doc = " are returned."]
        pub async fn mutate_account_budget_proposal(
            &mut self,
            request: impl tonic::IntoRequest<super::MutateAccountBudgetProposalRequest>,
        ) -> Result<tonic::Response<super::MutateAccountBudgetProposalResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ( "/google.ads.googleads.v3.services.AccountBudgetProposalService/MutateAccountBudgetProposal" ) ;
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for AccountBudgetProposalServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for AccountBudgetProposalServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "AccountBudgetProposalServiceClient {{ ... }}")
        }
    }
}
/// Request message for
/// [AccountBudgetService.GetAccountBudget][google.ads.googleads.v3.services.AccountBudgetService.GetAccountBudget].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAccountBudgetRequest {
    /// Required. The resource name of the account-level budget to fetch.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
#[doc = r" Generated client implementations."]
pub mod account_budget_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " A service for fetching an account-level budget."]
    #[doc = ""]
    #[doc = " Account-level budgets are mutated by creating proposal resources."]
    pub struct AccountBudgetServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> AccountBudgetServiceClient<T>
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
        #[doc = " Returns an account-level budget in full detail."]
        pub async fn get_account_budget(
            &mut self,
            request: impl tonic::IntoRequest<super::GetAccountBudgetRequest>,
        ) -> Result<tonic::Response<super::super::resources::AccountBudget>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ads.googleads.v3.services.AccountBudgetService/GetAccountBudget",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for AccountBudgetServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for AccountBudgetServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "AccountBudgetServiceClient {{ ... }}")
        }
    }
}
/// Request message for [AdGroupAdAssetViewService.GetAdGroupAdAssetView][google.ads.googleads.v3.services.AdGroupAdAssetViewService.GetAdGroupAdAssetView].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAdGroupAdAssetViewRequest {
    /// Required. The resource name of the ad group ad asset view to fetch.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
#[doc = r" Generated client implementations."]
pub mod ad_group_ad_asset_view_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Service to fetch ad group ad asset views."]
    pub struct AdGroupAdAssetViewServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> AdGroupAdAssetViewServiceClient<T>
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
        #[doc = " Returns the requested ad group ad asset view in full detail."]
        pub async fn get_ad_group_ad_asset_view(
            &mut self,
            request: impl tonic::IntoRequest<super::GetAdGroupAdAssetViewRequest>,
        ) -> Result<tonic::Response<super::super::resources::AdGroupAdAssetView>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ads.googleads.v3.services.AdGroupAdAssetViewService/GetAdGroupAdAssetView",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for AdGroupAdAssetViewServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for AdGroupAdAssetViewServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "AdGroupAdAssetViewServiceClient {{ ... }}")
        }
    }
}
/// Request message for [AdGroupAdLabelService.GetAdGroupAdLabel][google.ads.googleads.v3.services.AdGroupAdLabelService.GetAdGroupAdLabel].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAdGroupAdLabelRequest {
    /// Required. The resource name of the ad group ad label to fetch.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
/// Request message for [AdGroupAdLabelService.MutateAdGroupAdLabels][google.ads.googleads.v3.services.AdGroupAdLabelService.MutateAdGroupAdLabels].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateAdGroupAdLabelsRequest {
    /// Required. ID of the customer whose ad group ad labels are being modified.
    #[prost(string, tag = "1")]
    pub customer_id: std::string::String,
    /// Required. The list of operations to perform on ad group ad labels.
    #[prost(message, repeated, tag = "2")]
    pub operations: ::std::vec::Vec<AdGroupAdLabelOperation>,
    /// If true, successful operations will be carried out and invalid
    /// operations will return errors. If false, all operations will be carried
    /// out in one transaction if and only if they are all valid.
    /// Default is false.
    #[prost(bool, tag = "3")]
    pub partial_failure: bool,
    /// If true, the request is validated but not executed. Only errors are
    /// returned, not results.
    #[prost(bool, tag = "4")]
    pub validate_only: bool,
}
/// A single operation (create, remove) on an ad group ad label.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdGroupAdLabelOperation {
    /// The mutate operation.
    #[prost(oneof = "ad_group_ad_label_operation::Operation", tags = "1, 2")]
    pub operation: ::std::option::Option<ad_group_ad_label_operation::Operation>,
}
pub mod ad_group_ad_label_operation {
    /// The mutate operation.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Operation {
        /// Create operation: No resource name is expected for the new ad group ad
        /// label.
        #[prost(message, tag = "1")]
        Create(super::super::resources::AdGroupAdLabel),
        /// Remove operation: A resource name for the ad group ad label
        /// being removed, in this format:
        ///
        /// `customers/{customer_id}/adGroupAdLabels/{ad_group_id}~{ad_id}
        /// _{label_id}`
        #[prost(string, tag = "2")]
        Remove(std::string::String),
    }
}
/// Response message for an ad group ad labels mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateAdGroupAdLabelsResponse {
    /// Errors that pertain to operation failures in the partial failure mode.
    /// Returned only when partial_failure = true and all errors occur inside the
    /// operations. If any errors occur outside the operations (e.g. auth errors),
    /// we return an RPC level error.
    #[prost(message, optional, tag = "3")]
    pub partial_failure_error: ::std::option::Option<super::super::super::super::rpc::Status>,
    /// All results for the mutate.
    #[prost(message, repeated, tag = "2")]
    pub results: ::std::vec::Vec<MutateAdGroupAdLabelResult>,
}
/// The result for an ad group ad label mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateAdGroupAdLabelResult {
    /// Returned for successful operations.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
#[doc = r" Generated client implementations."]
pub mod ad_group_ad_label_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Service to manage labels on ad group ads."]
    pub struct AdGroupAdLabelServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> AdGroupAdLabelServiceClient<T>
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
        #[doc = " Returns the requested ad group ad label in full detail."]
        pub async fn get_ad_group_ad_label(
            &mut self,
            request: impl tonic::IntoRequest<super::GetAdGroupAdLabelRequest>,
        ) -> Result<tonic::Response<super::super::resources::AdGroupAdLabel>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ads.googleads.v3.services.AdGroupAdLabelService/GetAdGroupAdLabel",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates and removes ad group ad labels."]
        #[doc = " Operation statuses are returned."]
        pub async fn mutate_ad_group_ad_labels(
            &mut self,
            request: impl tonic::IntoRequest<super::MutateAdGroupAdLabelsRequest>,
        ) -> Result<tonic::Response<super::MutateAdGroupAdLabelsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ads.googleads.v3.services.AdGroupAdLabelService/MutateAdGroupAdLabels",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for AdGroupAdLabelServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for AdGroupAdLabelServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "AdGroupAdLabelServiceClient {{ ... }}")
        }
    }
}
/// Request message for [AdGroupAdService.GetAdGroupAd][google.ads.googleads.v3.services.AdGroupAdService.GetAdGroupAd].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAdGroupAdRequest {
    /// Required. The resource name of the ad to fetch.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
/// Request message for [AdGroupAdService.MutateAdGroupAds][google.ads.googleads.v3.services.AdGroupAdService.MutateAdGroupAds].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateAdGroupAdsRequest {
    /// Required. The ID of the customer whose ads are being modified.
    #[prost(string, tag = "1")]
    pub customer_id: std::string::String,
    /// Required. The list of operations to perform on individual ads.
    #[prost(message, repeated, tag = "2")]
    pub operations: ::std::vec::Vec<AdGroupAdOperation>,
    /// If true, successful operations will be carried out and invalid
    /// operations will return errors. If false, all operations will be carried
    /// out in one transaction if and only if they are all valid.
    /// Default is false.
    #[prost(bool, tag = "3")]
    pub partial_failure: bool,
    /// If true, the request is validated but not executed. Only errors are
    /// returned, not results.
    #[prost(bool, tag = "4")]
    pub validate_only: bool,
}
/// A single operation (create, update, remove) on an ad group ad.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdGroupAdOperation {
    /// FieldMask that determines which resource fields are modified in an update.
    #[prost(message, optional, tag = "4")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
    /// Configuration for how policies are validated.
    #[prost(message, optional, tag = "5")]
    pub policy_validation_parameter:
        ::std::option::Option<super::common::PolicyValidationParameter>,
    /// The mutate operation.
    #[prost(oneof = "ad_group_ad_operation::Operation", tags = "1, 2, 3")]
    pub operation: ::std::option::Option<ad_group_ad_operation::Operation>,
}
pub mod ad_group_ad_operation {
    /// The mutate operation.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Operation {
        /// Create operation: No resource name is expected for the new ad.
        #[prost(message, tag = "1")]
        Create(super::super::resources::AdGroupAd),
        /// Update operation: The ad is expected to have a valid resource name.
        #[prost(message, tag = "2")]
        Update(super::super::resources::AdGroupAd),
        /// Remove operation: A resource name for the removed ad is expected,
        /// in this format:
        ///
        /// `customers/{customer_id}/adGroupAds/{ad_group_id}~{ad_id}`
        #[prost(string, tag = "3")]
        Remove(std::string::String),
    }
}
/// Response message for an ad group ad mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateAdGroupAdsResponse {
    /// Errors that pertain to operation failures in the partial failure mode.
    /// Returned only when partial_failure = true and all errors occur inside the
    /// operations. If any errors occur outside the operations (e.g. auth errors),
    /// we return an RPC level error.
    #[prost(message, optional, tag = "3")]
    pub partial_failure_error: ::std::option::Option<super::super::super::super::rpc::Status>,
    /// All results for the mutate.
    #[prost(message, repeated, tag = "2")]
    pub results: ::std::vec::Vec<MutateAdGroupAdResult>,
}
/// The result for the ad mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateAdGroupAdResult {
    /// The resource name returned for successful operations.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
#[doc = r" Generated client implementations."]
pub mod ad_group_ad_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Service to manage ads in an ad group."]
    pub struct AdGroupAdServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> AdGroupAdServiceClient<T>
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
        #[doc = " Returns the requested ad in full detail."]
        pub async fn get_ad_group_ad(
            &mut self,
            request: impl tonic::IntoRequest<super::GetAdGroupAdRequest>,
        ) -> Result<tonic::Response<super::super::resources::AdGroupAd>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ads.googleads.v3.services.AdGroupAdService/GetAdGroupAd",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates, updates, or removes ads. Operation statuses are returned."]
        pub async fn mutate_ad_group_ads(
            &mut self,
            request: impl tonic::IntoRequest<super::MutateAdGroupAdsRequest>,
        ) -> Result<tonic::Response<super::MutateAdGroupAdsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ads.googleads.v3.services.AdGroupAdService/MutateAdGroupAds",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for AdGroupAdServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for AdGroupAdServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "AdGroupAdServiceClient {{ ... }}")
        }
    }
}
/// Request message for [AdGroupAudienceViewService.GetAdGroupAudienceView][google.ads.googleads.v3.services.AdGroupAudienceViewService.GetAdGroupAudienceView].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAdGroupAudienceViewRequest {
    /// Required. The resource name of the ad group audience view to fetch.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
#[doc = r" Generated client implementations."]
pub mod ad_group_audience_view_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Service to manage ad group audience views."]
    pub struct AdGroupAudienceViewServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> AdGroupAudienceViewServiceClient<T>
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
        #[doc = " Returns the requested ad group audience view in full detail."]
        pub async fn get_ad_group_audience_view(
            &mut self,
            request: impl tonic::IntoRequest<super::GetAdGroupAudienceViewRequest>,
        ) -> Result<tonic::Response<super::super::resources::AdGroupAudienceView>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ( "/google.ads.googleads.v3.services.AdGroupAudienceViewService/GetAdGroupAudienceView" ) ;
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for AdGroupAudienceViewServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for AdGroupAudienceViewServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "AdGroupAudienceViewServiceClient {{ ... }}")
        }
    }
}
/// Request message for [AdGroupBidModifierService.GetAdGroupBidModifier][google.ads.googleads.v3.services.AdGroupBidModifierService.GetAdGroupBidModifier].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAdGroupBidModifierRequest {
    /// Required. The resource name of the ad group bid modifier to fetch.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
/// Request message for [AdGroupBidModifierService.MutateAdGroupBidModifiers][google.ads.googleads.v3.services.AdGroupBidModifierService.MutateAdGroupBidModifiers].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateAdGroupBidModifiersRequest {
    /// Required. ID of the customer whose ad group bid modifiers are being modified.
    #[prost(string, tag = "1")]
    pub customer_id: std::string::String,
    /// Required. The list of operations to perform on individual ad group bid modifiers.
    #[prost(message, repeated, tag = "2")]
    pub operations: ::std::vec::Vec<AdGroupBidModifierOperation>,
    /// If true, successful operations will be carried out and invalid
    /// operations will return errors. If false, all operations will be carried
    /// out in one transaction if and only if they are all valid.
    /// Default is false.
    #[prost(bool, tag = "3")]
    pub partial_failure: bool,
    /// If true, the request is validated but not executed. Only errors are
    /// returned, not results.
    #[prost(bool, tag = "4")]
    pub validate_only: bool,
}
/// A single operation (create, remove, update) on an ad group bid modifier.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdGroupBidModifierOperation {
    /// FieldMask that determines which resource fields are modified in an update.
    #[prost(message, optional, tag = "4")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
    /// The mutate operation.
    #[prost(oneof = "ad_group_bid_modifier_operation::Operation", tags = "1, 2, 3")]
    pub operation: ::std::option::Option<ad_group_bid_modifier_operation::Operation>,
}
pub mod ad_group_bid_modifier_operation {
    /// The mutate operation.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Operation {
        /// Create operation: No resource name is expected for the new ad group bid
        /// modifier.
        #[prost(message, tag = "1")]
        Create(super::super::resources::AdGroupBidModifier),
        /// Update operation: The ad group bid modifier is expected to have a valid
        /// resource name.
        #[prost(message, tag = "2")]
        Update(super::super::resources::AdGroupBidModifier),
        /// Remove operation: A resource name for the removed ad group bid modifier
        /// is expected, in this format:
        ///
        /// `customers/{customer_id}/adGroupBidModifiers/{ad_group_id}~{criterion_id}`
        #[prost(string, tag = "3")]
        Remove(std::string::String),
    }
}
/// Response message for ad group bid modifiers mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateAdGroupBidModifiersResponse {
    /// Errors that pertain to operation failures in the partial failure mode.
    /// Returned only when partial_failure = true and all errors occur inside the
    /// operations. If any errors occur outside the operations (e.g. auth errors),
    /// we return an RPC level error.
    #[prost(message, optional, tag = "3")]
    pub partial_failure_error: ::std::option::Option<super::super::super::super::rpc::Status>,
    /// All results for the mutate.
    #[prost(message, repeated, tag = "2")]
    pub results: ::std::vec::Vec<MutateAdGroupBidModifierResult>,
}
/// The result for the criterion mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateAdGroupBidModifierResult {
    /// Returned for successful operations.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
#[doc = r" Generated client implementations."]
pub mod ad_group_bid_modifier_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Service to manage ad group bid modifiers."]
    pub struct AdGroupBidModifierServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> AdGroupBidModifierServiceClient<T>
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
        #[doc = " Returns the requested ad group bid modifier in full detail."]
        pub async fn get_ad_group_bid_modifier(
            &mut self,
            request: impl tonic::IntoRequest<super::GetAdGroupBidModifierRequest>,
        ) -> Result<tonic::Response<super::super::resources::AdGroupBidModifier>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ads.googleads.v3.services.AdGroupBidModifierService/GetAdGroupBidModifier",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates, updates, or removes ad group bid modifiers."]
        #[doc = " Operation statuses are returned."]
        pub async fn mutate_ad_group_bid_modifiers(
            &mut self,
            request: impl tonic::IntoRequest<super::MutateAdGroupBidModifiersRequest>,
        ) -> Result<tonic::Response<super::MutateAdGroupBidModifiersResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ( "/google.ads.googleads.v3.services.AdGroupBidModifierService/MutateAdGroupBidModifiers" ) ;
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for AdGroupBidModifierServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for AdGroupBidModifierServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "AdGroupBidModifierServiceClient {{ ... }}")
        }
    }
}
/// Request message for
/// [AdGroupCriterionLabelService.GetAdGroupCriterionLabel][google.ads.googleads.v3.services.AdGroupCriterionLabelService.GetAdGroupCriterionLabel].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAdGroupCriterionLabelRequest {
    /// Required. The resource name of the ad group criterion label to fetch.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
/// Request message for
/// [AdGroupCriterionLabelService.MutateAdGroupCriterionLabels][google.ads.googleads.v3.services.AdGroupCriterionLabelService.MutateAdGroupCriterionLabels].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateAdGroupCriterionLabelsRequest {
    /// Required. ID of the customer whose ad group criterion labels are being modified.
    #[prost(string, tag = "1")]
    pub customer_id: std::string::String,
    /// Required. The list of operations to perform on ad group criterion labels.
    #[prost(message, repeated, tag = "2")]
    pub operations: ::std::vec::Vec<AdGroupCriterionLabelOperation>,
    /// If true, successful operations will be carried out and invalid
    /// operations will return errors. If false, all operations will be carried
    /// out in one transaction if and only if they are all valid.
    /// Default is false.
    #[prost(bool, tag = "3")]
    pub partial_failure: bool,
    /// If true, the request is validated but not executed. Only errors are
    /// returned, not results.
    #[prost(bool, tag = "4")]
    pub validate_only: bool,
}
/// A single operation (create, remove) on an ad group criterion label.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdGroupCriterionLabelOperation {
    /// The mutate operation.
    #[prost(oneof = "ad_group_criterion_label_operation::Operation", tags = "1, 2")]
    pub operation: ::std::option::Option<ad_group_criterion_label_operation::Operation>,
}
pub mod ad_group_criterion_label_operation {
    /// The mutate operation.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Operation {
        /// Create operation: No resource name is expected for the new ad group
        /// label.
        #[prost(message, tag = "1")]
        Create(super::super::resources::AdGroupCriterionLabel),
        /// Remove operation: A resource name for the ad group criterion label
        /// being removed, in this format:
        ///
        /// `customers/{customer_id}/adGroupCriterionLabels/{ad_group_id}~{criterion_id}~{label_id}`
        #[prost(string, tag = "2")]
        Remove(std::string::String),
    }
}
/// Response message for an ad group criterion labels mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateAdGroupCriterionLabelsResponse {
    /// Errors that pertain to operation failures in the partial failure mode.
    /// Returned only when partial_failure = true and all errors occur inside the
    /// operations. If any errors occur outside the operations (e.g. auth errors),
    /// we return an RPC level error.
    #[prost(message, optional, tag = "3")]
    pub partial_failure_error: ::std::option::Option<super::super::super::super::rpc::Status>,
    /// All results for the mutate.
    #[prost(message, repeated, tag = "2")]
    pub results: ::std::vec::Vec<MutateAdGroupCriterionLabelResult>,
}
/// The result for an ad group criterion label mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateAdGroupCriterionLabelResult {
    /// Returned for successful operations.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
#[doc = r" Generated client implementations."]
pub mod ad_group_criterion_label_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Service to manage labels on ad group criteria."]
    pub struct AdGroupCriterionLabelServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> AdGroupCriterionLabelServiceClient<T>
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
        #[doc = " Returns the requested ad group criterion label in full detail."]
        pub async fn get_ad_group_criterion_label(
            &mut self,
            request: impl tonic::IntoRequest<super::GetAdGroupCriterionLabelRequest>,
        ) -> Result<tonic::Response<super::super::resources::AdGroupCriterionLabel>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ( "/google.ads.googleads.v3.services.AdGroupCriterionLabelService/GetAdGroupCriterionLabel" ) ;
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates and removes ad group criterion labels."]
        #[doc = " Operation statuses are returned."]
        pub async fn mutate_ad_group_criterion_labels(
            &mut self,
            request: impl tonic::IntoRequest<super::MutateAdGroupCriterionLabelsRequest>,
        ) -> Result<tonic::Response<super::MutateAdGroupCriterionLabelsResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ( "/google.ads.googleads.v3.services.AdGroupCriterionLabelService/MutateAdGroupCriterionLabels" ) ;
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for AdGroupCriterionLabelServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for AdGroupCriterionLabelServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "AdGroupCriterionLabelServiceClient {{ ... }}")
        }
    }
}
/// Request message for [AdGroupCriterionService.GetAdGroupCriterion][google.ads.googleads.v3.services.AdGroupCriterionService.GetAdGroupCriterion].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAdGroupCriterionRequest {
    /// Required. The resource name of the criterion to fetch.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
/// Request message for [AdGroupCriterionService.MutateAdGroupCriteria][google.ads.googleads.v3.services.AdGroupCriterionService.MutateAdGroupCriteria].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateAdGroupCriteriaRequest {
    /// Required. ID of the customer whose criteria are being modified.
    #[prost(string, tag = "1")]
    pub customer_id: std::string::String,
    /// Required. The list of operations to perform on individual criteria.
    #[prost(message, repeated, tag = "2")]
    pub operations: ::std::vec::Vec<AdGroupCriterionOperation>,
    /// If true, successful operations will be carried out and invalid
    /// operations will return errors. If false, all operations will be carried
    /// out in one transaction if and only if they are all valid.
    /// Default is false.
    #[prost(bool, tag = "3")]
    pub partial_failure: bool,
    /// If true, the request is validated but not executed. Only errors are
    /// returned, not results.
    #[prost(bool, tag = "4")]
    pub validate_only: bool,
}
/// A single operation (create, remove, update) on an ad group criterion.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdGroupCriterionOperation {
    /// FieldMask that determines which resource fields are modified in an update.
    #[prost(message, optional, tag = "4")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
    /// The list of policy violation keys that should not cause a
    /// PolicyViolationError to be reported. Not all policy violations are
    /// exemptable, please refer to the is_exemptible field in the returned
    /// PolicyViolationError.
    ///
    /// Resources violating these polices will be saved, but will not be eligible
    /// to serve. They may begin serving at a later time due to a change in
    /// policies, re-review of the resource, or a change in advertiser
    /// certificates.
    #[prost(message, repeated, tag = "5")]
    pub exempt_policy_violation_keys: ::std::vec::Vec<super::common::PolicyViolationKey>,
    /// The mutate operation.
    #[prost(oneof = "ad_group_criterion_operation::Operation", tags = "1, 2, 3")]
    pub operation: ::std::option::Option<ad_group_criterion_operation::Operation>,
}
pub mod ad_group_criterion_operation {
    /// The mutate operation.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Operation {
        /// Create operation: No resource name is expected for the new criterion.
        #[prost(message, tag = "1")]
        Create(super::super::resources::AdGroupCriterion),
        /// Update operation: The criterion is expected to have a valid resource
        /// name.
        #[prost(message, tag = "2")]
        Update(super::super::resources::AdGroupCriterion),
        /// Remove operation: A resource name for the removed criterion is expected,
        /// in this format:
        ///
        /// `customers/{customer_id}/adGroupCriteria/{ad_group_id}~{criterion_id}`
        #[prost(string, tag = "3")]
        Remove(std::string::String),
    }
}
/// Response message for an ad group criterion mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateAdGroupCriteriaResponse {
    /// Errors that pertain to operation failures in the partial failure mode.
    /// Returned only when partial_failure = true and all errors occur inside the
    /// operations. If any errors occur outside the operations (e.g. auth errors),
    /// we return an RPC level error.
    #[prost(message, optional, tag = "3")]
    pub partial_failure_error: ::std::option::Option<super::super::super::super::rpc::Status>,
    /// All results for the mutate.
    #[prost(message, repeated, tag = "2")]
    pub results: ::std::vec::Vec<MutateAdGroupCriterionResult>,
}
/// The result for the criterion mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateAdGroupCriterionResult {
    /// Returned for successful operations.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
#[doc = r" Generated client implementations."]
pub mod ad_group_criterion_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Service to manage ad group criteria."]
    pub struct AdGroupCriterionServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> AdGroupCriterionServiceClient<T>
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
        #[doc = " Returns the requested criterion in full detail."]
        pub async fn get_ad_group_criterion(
            &mut self,
            request: impl tonic::IntoRequest<super::GetAdGroupCriterionRequest>,
        ) -> Result<tonic::Response<super::super::resources::AdGroupCriterion>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ads.googleads.v3.services.AdGroupCriterionService/GetAdGroupCriterion",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates, updates, or removes criteria. Operation statuses are returned."]
        pub async fn mutate_ad_group_criteria(
            &mut self,
            request: impl tonic::IntoRequest<super::MutateAdGroupCriteriaRequest>,
        ) -> Result<tonic::Response<super::MutateAdGroupCriteriaResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ads.googleads.v3.services.AdGroupCriterionService/MutateAdGroupCriteria",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for AdGroupCriterionServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for AdGroupCriterionServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "AdGroupCriterionServiceClient {{ ... }}")
        }
    }
}
/// Request message for
/// [AdGroupCriterionSimulationService.GetAdGroupCriterionSimulation][google.ads.googleads.v3.services.AdGroupCriterionSimulationService.GetAdGroupCriterionSimulation].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAdGroupCriterionSimulationRequest {
    /// Required. The resource name of the ad group criterion simulation to fetch.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
#[doc = r" Generated client implementations."]
pub mod ad_group_criterion_simulation_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Service to fetch ad group criterion simulations."]
    pub struct AdGroupCriterionSimulationServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> AdGroupCriterionSimulationServiceClient<T>
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
        #[doc = " Returns the requested ad group criterion simulation in full detail."]
        pub async fn get_ad_group_criterion_simulation(
            &mut self,
            request: impl tonic::IntoRequest<super::GetAdGroupCriterionSimulationRequest>,
        ) -> Result<
            tonic::Response<super::super::resources::AdGroupCriterionSimulation>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ( "/google.ads.googleads.v3.services.AdGroupCriterionSimulationService/GetAdGroupCriterionSimulation" ) ;
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for AdGroupCriterionSimulationServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for AdGroupCriterionSimulationServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "AdGroupCriterionSimulationServiceClient {{ ... }}")
        }
    }
}
/// Request message for
/// [AdGroupExtensionSettingService.GetAdGroupExtensionSetting][google.ads.googleads.v3.services.AdGroupExtensionSettingService.GetAdGroupExtensionSetting].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAdGroupExtensionSettingRequest {
    /// Required. The resource name of the ad group extension setting to fetch.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
/// Request message for
/// [AdGroupExtensionSettingService.MutateAdGroupExtensionSettings][google.ads.googleads.v3.services.AdGroupExtensionSettingService.MutateAdGroupExtensionSettings].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateAdGroupExtensionSettingsRequest {
    /// Required. The ID of the customer whose ad group extension settings are being
    /// modified.
    #[prost(string, tag = "1")]
    pub customer_id: std::string::String,
    /// Required. The list of operations to perform on individual ad group extension
    /// settings.
    #[prost(message, repeated, tag = "2")]
    pub operations: ::std::vec::Vec<AdGroupExtensionSettingOperation>,
    /// If true, successful operations will be carried out and invalid
    /// operations will return errors. If false, all operations will be carried
    /// out in one transaction if and only if they are all valid.
    /// Default is false.
    #[prost(bool, tag = "3")]
    pub partial_failure: bool,
    /// If true, the request is validated but not executed. Only errors are
    /// returned, not results.
    #[prost(bool, tag = "4")]
    pub validate_only: bool,
}
/// A single operation (create, update, remove) on an ad group extension setting.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdGroupExtensionSettingOperation {
    /// FieldMask that determines which resource fields are modified in an update.
    #[prost(message, optional, tag = "4")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
    /// The mutate operation.
    #[prost(
        oneof = "ad_group_extension_setting_operation::Operation",
        tags = "1, 2, 3"
    )]
    pub operation: ::std::option::Option<ad_group_extension_setting_operation::Operation>,
}
pub mod ad_group_extension_setting_operation {
    /// The mutate operation.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Operation {
        /// Create operation: No resource name is expected for the new ad group
        /// extension setting.
        #[prost(message, tag = "1")]
        Create(super::super::resources::AdGroupExtensionSetting),
        /// Update operation: The ad group extension setting is expected to have a
        /// valid resource name.
        #[prost(message, tag = "2")]
        Update(super::super::resources::AdGroupExtensionSetting),
        /// Remove operation: A resource name for the removed ad group extension
        /// setting is expected, in this format:
        ///
        /// `customers/{customer_id}/adGroupExtensionSettings/{ad_group_id}~{extension_type}`
        #[prost(string, tag = "3")]
        Remove(std::string::String),
    }
}
/// Response message for an ad group extension setting mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateAdGroupExtensionSettingsResponse {
    /// Errors that pertain to operation failures in the partial failure mode.
    /// Returned only when partial_failure = true and all errors occur inside the
    /// operations. If any errors occur outside the operations (e.g. auth errors),
    /// we return an RPC level error.
    #[prost(message, optional, tag = "3")]
    pub partial_failure_error: ::std::option::Option<super::super::super::super::rpc::Status>,
    /// All results for the mutate.
    #[prost(message, repeated, tag = "2")]
    pub results: ::std::vec::Vec<MutateAdGroupExtensionSettingResult>,
}
/// The result for the ad group extension setting mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateAdGroupExtensionSettingResult {
    /// Returned for successful operations.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
#[doc = r" Generated client implementations."]
pub mod ad_group_extension_setting_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Service to manage ad group extension settings."]
    pub struct AdGroupExtensionSettingServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> AdGroupExtensionSettingServiceClient<T>
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
        #[doc = " Returns the requested ad group extension setting in full detail."]
        pub async fn get_ad_group_extension_setting(
            &mut self,
            request: impl tonic::IntoRequest<super::GetAdGroupExtensionSettingRequest>,
        ) -> Result<tonic::Response<super::super::resources::AdGroupExtensionSetting>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ( "/google.ads.googleads.v3.services.AdGroupExtensionSettingService/GetAdGroupExtensionSetting" ) ;
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates, updates, or removes ad group extension settings. Operation"]
        #[doc = " statuses are returned."]
        pub async fn mutate_ad_group_extension_settings(
            &mut self,
            request: impl tonic::IntoRequest<super::MutateAdGroupExtensionSettingsRequest>,
        ) -> Result<tonic::Response<super::MutateAdGroupExtensionSettingsResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ( "/google.ads.googleads.v3.services.AdGroupExtensionSettingService/MutateAdGroupExtensionSettings" ) ;
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for AdGroupExtensionSettingServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for AdGroupExtensionSettingServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "AdGroupExtensionSettingServiceClient {{ ... }}")
        }
    }
}
/// Request message for [AdGroupFeedService.GetAdGroupFeed][google.ads.googleads.v3.services.AdGroupFeedService.GetAdGroupFeed].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAdGroupFeedRequest {
    /// Required. The resource name of the ad group feed to fetch.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
/// Request message for [AdGroupFeedService.MutateAdGroupFeeds][google.ads.googleads.v3.services.AdGroupFeedService.MutateAdGroupFeeds].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateAdGroupFeedsRequest {
    /// Required. The ID of the customer whose ad group feeds are being modified.
    #[prost(string, tag = "1")]
    pub customer_id: std::string::String,
    /// Required. The list of operations to perform on individual ad group feeds.
    #[prost(message, repeated, tag = "2")]
    pub operations: ::std::vec::Vec<AdGroupFeedOperation>,
    /// If true, successful operations will be carried out and invalid
    /// operations will return errors. If false, all operations will be carried
    /// out in one transaction if and only if they are all valid.
    /// Default is false.
    #[prost(bool, tag = "3")]
    pub partial_failure: bool,
    /// If true, the request is validated but not executed. Only errors are
    /// returned, not results.
    #[prost(bool, tag = "4")]
    pub validate_only: bool,
}
/// A single operation (create, update, remove) on an ad group feed.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdGroupFeedOperation {
    /// FieldMask that determines which resource fields are modified in an update.
    #[prost(message, optional, tag = "4")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
    /// The mutate operation.
    #[prost(oneof = "ad_group_feed_operation::Operation", tags = "1, 2, 3")]
    pub operation: ::std::option::Option<ad_group_feed_operation::Operation>,
}
pub mod ad_group_feed_operation {
    /// The mutate operation.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Operation {
        /// Create operation: No resource name is expected for the new ad group feed.
        #[prost(message, tag = "1")]
        Create(super::super::resources::AdGroupFeed),
        /// Update operation: The ad group feed is expected to have a valid resource
        /// name.
        #[prost(message, tag = "2")]
        Update(super::super::resources::AdGroupFeed),
        /// Remove operation: A resource name for the removed ad group feed is
        /// expected, in this format:
        ///
        /// `customers/{customer_id}/adGroupFeeds/{ad_group_id}~{feed_id}`
        #[prost(string, tag = "3")]
        Remove(std::string::String),
    }
}
/// Response message for an ad group feed mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateAdGroupFeedsResponse {
    /// Errors that pertain to operation failures in the partial failure mode.
    /// Returned only when partial_failure = true and all errors occur inside the
    /// operations. If any errors occur outside the operations (e.g. auth errors),
    /// we return an RPC level error.
    #[prost(message, optional, tag = "3")]
    pub partial_failure_error: ::std::option::Option<super::super::super::super::rpc::Status>,
    /// All results for the mutate.
    #[prost(message, repeated, tag = "2")]
    pub results: ::std::vec::Vec<MutateAdGroupFeedResult>,
}
/// The result for the ad group feed mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateAdGroupFeedResult {
    /// Returned for successful operations.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
#[doc = r" Generated client implementations."]
pub mod ad_group_feed_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Service to manage ad group feeds."]
    pub struct AdGroupFeedServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> AdGroupFeedServiceClient<T>
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
        #[doc = " Returns the requested ad group feed in full detail."]
        pub async fn get_ad_group_feed(
            &mut self,
            request: impl tonic::IntoRequest<super::GetAdGroupFeedRequest>,
        ) -> Result<tonic::Response<super::super::resources::AdGroupFeed>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ads.googleads.v3.services.AdGroupFeedService/GetAdGroupFeed",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates, updates, or removes ad group feeds. Operation statuses are"]
        #[doc = " returned."]
        pub async fn mutate_ad_group_feeds(
            &mut self,
            request: impl tonic::IntoRequest<super::MutateAdGroupFeedsRequest>,
        ) -> Result<tonic::Response<super::MutateAdGroupFeedsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ads.googleads.v3.services.AdGroupFeedService/MutateAdGroupFeeds",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for AdGroupFeedServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for AdGroupFeedServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "AdGroupFeedServiceClient {{ ... }}")
        }
    }
}
/// Request message for [AdGroupLabelService.GetAdGroupLabel][google.ads.googleads.v3.services.AdGroupLabelService.GetAdGroupLabel].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAdGroupLabelRequest {
    /// Required. The resource name of the ad group label to fetch.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
/// Request message for [AdGroupLabelService.MutateAdGroupLabels][google.ads.googleads.v3.services.AdGroupLabelService.MutateAdGroupLabels].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateAdGroupLabelsRequest {
    /// Required. ID of the customer whose ad group labels are being modified.
    #[prost(string, tag = "1")]
    pub customer_id: std::string::String,
    /// Required. The list of operations to perform on ad group labels.
    #[prost(message, repeated, tag = "2")]
    pub operations: ::std::vec::Vec<AdGroupLabelOperation>,
    /// If true, successful operations will be carried out and invalid
    /// operations will return errors. If false, all operations will be carried
    /// out in one transaction if and only if they are all valid.
    /// Default is false.
    #[prost(bool, tag = "3")]
    pub partial_failure: bool,
    /// If true, the request is validated but not executed. Only errors are
    /// returned, not results.
    #[prost(bool, tag = "4")]
    pub validate_only: bool,
}
/// A single operation (create, remove) on an ad group label.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdGroupLabelOperation {
    /// The mutate operation.
    #[prost(oneof = "ad_group_label_operation::Operation", tags = "1, 2")]
    pub operation: ::std::option::Option<ad_group_label_operation::Operation>,
}
pub mod ad_group_label_operation {
    /// The mutate operation.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Operation {
        /// Create operation: No resource name is expected for the new ad group
        /// label.
        #[prost(message, tag = "1")]
        Create(super::super::resources::AdGroupLabel),
        /// Remove operation: A resource name for the ad group label
        /// being removed, in this format:
        ///
        /// `customers/{customer_id}/adGroupLabels/{ad_group_id}~{label_id}`
        #[prost(string, tag = "2")]
        Remove(std::string::String),
    }
}
/// Response message for an ad group labels mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateAdGroupLabelsResponse {
    /// Errors that pertain to operation failures in the partial failure mode.
    /// Returned only when partial_failure = true and all errors occur inside the
    /// operations. If any errors occur outside the operations (e.g. auth errors),
    /// we return an RPC level error.
    #[prost(message, optional, tag = "3")]
    pub partial_failure_error: ::std::option::Option<super::super::super::super::rpc::Status>,
    /// All results for the mutate.
    #[prost(message, repeated, tag = "2")]
    pub results: ::std::vec::Vec<MutateAdGroupLabelResult>,
}
/// The result for an ad group label mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateAdGroupLabelResult {
    /// Returned for successful operations.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
#[doc = r" Generated client implementations."]
pub mod ad_group_label_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Service to manage labels on ad groups."]
    pub struct AdGroupLabelServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> AdGroupLabelServiceClient<T>
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
        #[doc = " Returns the requested ad group label in full detail."]
        pub async fn get_ad_group_label(
            &mut self,
            request: impl tonic::IntoRequest<super::GetAdGroupLabelRequest>,
        ) -> Result<tonic::Response<super::super::resources::AdGroupLabel>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ads.googleads.v3.services.AdGroupLabelService/GetAdGroupLabel",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates and removes ad group labels."]
        #[doc = " Operation statuses are returned."]
        pub async fn mutate_ad_group_labels(
            &mut self,
            request: impl tonic::IntoRequest<super::MutateAdGroupLabelsRequest>,
        ) -> Result<tonic::Response<super::MutateAdGroupLabelsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ads.googleads.v3.services.AdGroupLabelService/MutateAdGroupLabels",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for AdGroupLabelServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for AdGroupLabelServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "AdGroupLabelServiceClient {{ ... }}")
        }
    }
}
/// Request message for [AdGroupService.GetAdGroup][google.ads.googleads.v3.services.AdGroupService.GetAdGroup].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAdGroupRequest {
    /// Required. The resource name of the ad group to fetch.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
/// Request message for [AdGroupService.MutateAdGroups][google.ads.googleads.v3.services.AdGroupService.MutateAdGroups].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateAdGroupsRequest {
    /// Required. The ID of the customer whose ad groups are being modified.
    #[prost(string, tag = "1")]
    pub customer_id: std::string::String,
    /// Required. The list of operations to perform on individual ad groups.
    #[prost(message, repeated, tag = "2")]
    pub operations: ::std::vec::Vec<AdGroupOperation>,
    /// If true, successful operations will be carried out and invalid
    /// operations will return errors. If false, all operations will be carried
    /// out in one transaction if and only if they are all valid.
    /// Default is false.
    #[prost(bool, tag = "3")]
    pub partial_failure: bool,
    /// If true, the request is validated but not executed. Only errors are
    /// returned, not results.
    #[prost(bool, tag = "4")]
    pub validate_only: bool,
}
/// A single operation (create, update, remove) on an ad group.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdGroupOperation {
    /// FieldMask that determines which resource fields are modified in an update.
    #[prost(message, optional, tag = "4")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
    /// The mutate operation.
    #[prost(oneof = "ad_group_operation::Operation", tags = "1, 2, 3")]
    pub operation: ::std::option::Option<ad_group_operation::Operation>,
}
pub mod ad_group_operation {
    /// The mutate operation.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Operation {
        /// Create operation: No resource name is expected for the new ad group.
        #[prost(message, tag = "1")]
        Create(super::super::resources::AdGroup),
        /// Update operation: The ad group is expected to have a valid resource name.
        #[prost(message, tag = "2")]
        Update(super::super::resources::AdGroup),
        /// Remove operation: A resource name for the removed ad group is expected,
        /// in this format:
        ///
        /// `customers/{customer_id}/adGroups/{ad_group_id}`
        #[prost(string, tag = "3")]
        Remove(std::string::String),
    }
}
/// Response message for an ad group mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateAdGroupsResponse {
    /// Errors that pertain to operation failures in the partial failure mode.
    /// Returned only when partial_failure = true and all errors occur inside the
    /// operations. If any errors occur outside the operations (e.g. auth errors),
    /// we return an RPC level error.
    #[prost(message, optional, tag = "3")]
    pub partial_failure_error: ::std::option::Option<super::super::super::super::rpc::Status>,
    /// All results for the mutate.
    #[prost(message, repeated, tag = "2")]
    pub results: ::std::vec::Vec<MutateAdGroupResult>,
}
/// The result for the ad group mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateAdGroupResult {
    /// Returned for successful operations.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
#[doc = r" Generated client implementations."]
pub mod ad_group_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Service to manage ad groups."]
    pub struct AdGroupServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> AdGroupServiceClient<T>
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
        #[doc = " Returns the requested ad group in full detail."]
        pub async fn get_ad_group(
            &mut self,
            request: impl tonic::IntoRequest<super::GetAdGroupRequest>,
        ) -> Result<tonic::Response<super::super::resources::AdGroup>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ads.googleads.v3.services.AdGroupService/GetAdGroup",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates, updates, or removes ad groups. Operation statuses are returned."]
        pub async fn mutate_ad_groups(
            &mut self,
            request: impl tonic::IntoRequest<super::MutateAdGroupsRequest>,
        ) -> Result<tonic::Response<super::MutateAdGroupsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ads.googleads.v3.services.AdGroupService/MutateAdGroups",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for AdGroupServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for AdGroupServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "AdGroupServiceClient {{ ... }}")
        }
    }
}
/// Request message for [AdGroupSimulationService.GetAdGroupSimulation][google.ads.googleads.v3.services.AdGroupSimulationService.GetAdGroupSimulation].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAdGroupSimulationRequest {
    /// Required. The resource name of the ad group simulation to fetch.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
#[doc = r" Generated client implementations."]
pub mod ad_group_simulation_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Service to fetch ad group simulations."]
    pub struct AdGroupSimulationServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> AdGroupSimulationServiceClient<T>
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
        #[doc = " Returns the requested ad group simulation in full detail."]
        pub async fn get_ad_group_simulation(
            &mut self,
            request: impl tonic::IntoRequest<super::GetAdGroupSimulationRequest>,
        ) -> Result<tonic::Response<super::super::resources::AdGroupSimulation>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ads.googleads.v3.services.AdGroupSimulationService/GetAdGroupSimulation",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for AdGroupSimulationServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for AdGroupSimulationServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "AdGroupSimulationServiceClient {{ ... }}")
        }
    }
}
/// Request message for [AdParameterService.GetAdParameter][google.ads.googleads.v3.services.AdParameterService.GetAdParameter]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAdParameterRequest {
    /// Required. The resource name of the ad parameter to fetch.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
/// Request message for [AdParameterService.MutateAdParameters][google.ads.googleads.v3.services.AdParameterService.MutateAdParameters]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateAdParametersRequest {
    /// Required. The ID of the customer whose ad parameters are being modified.
    #[prost(string, tag = "1")]
    pub customer_id: std::string::String,
    /// Required. The list of operations to perform on individual ad parameters.
    #[prost(message, repeated, tag = "2")]
    pub operations: ::std::vec::Vec<AdParameterOperation>,
    /// If true, successful operations will be carried out and invalid
    /// operations will return errors. If false, all operations will be carried
    /// out in one transaction if and only if they are all valid.
    /// Default is false.
    #[prost(bool, tag = "3")]
    pub partial_failure: bool,
    /// If true, the request is validated but not executed. Only errors are
    /// returned, not results.
    #[prost(bool, tag = "4")]
    pub validate_only: bool,
}
/// A single operation (create, update, remove) on ad parameter.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdParameterOperation {
    /// FieldMask that determines which resource fields are modified in an update.
    #[prost(message, optional, tag = "4")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
    /// The mutate operation.
    #[prost(oneof = "ad_parameter_operation::Operation", tags = "1, 2, 3")]
    pub operation: ::std::option::Option<ad_parameter_operation::Operation>,
}
pub mod ad_parameter_operation {
    /// The mutate operation.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Operation {
        /// Create operation: No resource name is expected for the new ad parameter.
        #[prost(message, tag = "1")]
        Create(super::super::resources::AdParameter),
        /// Update operation: The ad parameter is expected to have a valid resource
        /// name.
        #[prost(message, tag = "2")]
        Update(super::super::resources::AdParameter),
        /// Remove operation: A resource name for the ad parameter to remove is
        /// expected in this format:
        ///
        /// `customers/{customer_id}/adParameters/{ad_group_id}~{criterion_id}~{parameter_index}`
        #[prost(string, tag = "3")]
        Remove(std::string::String),
    }
}
/// Response message for an ad parameter mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateAdParametersResponse {
    /// Errors that pertain to operation failures in the partial failure mode.
    /// Returned only when partial_failure = true and all errors occur inside the
    /// operations. If any errors occur outside the operations (e.g. auth errors),
    /// we return an RPC level error.
    #[prost(message, optional, tag = "3")]
    pub partial_failure_error: ::std::option::Option<super::super::super::super::rpc::Status>,
    /// All results for the mutate.
    #[prost(message, repeated, tag = "2")]
    pub results: ::std::vec::Vec<MutateAdParameterResult>,
}
/// The result for the ad parameter mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateAdParameterResult {
    /// The resource name returned for successful operations.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
#[doc = r" Generated client implementations."]
pub mod ad_parameter_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Service to manage ad parameters."]
    pub struct AdParameterServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> AdParameterServiceClient<T>
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
        #[doc = " Returns the requested ad parameter in full detail."]
        pub async fn get_ad_parameter(
            &mut self,
            request: impl tonic::IntoRequest<super::GetAdParameterRequest>,
        ) -> Result<tonic::Response<super::super::resources::AdParameter>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ads.googleads.v3.services.AdParameterService/GetAdParameter",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates, updates, or removes ad parameters. Operation statuses are"]
        #[doc = " returned."]
        pub async fn mutate_ad_parameters(
            &mut self,
            request: impl tonic::IntoRequest<super::MutateAdParametersRequest>,
        ) -> Result<tonic::Response<super::MutateAdParametersResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ads.googleads.v3.services.AdParameterService/MutateAdParameters",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for AdParameterServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for AdParameterServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "AdParameterServiceClient {{ ... }}")
        }
    }
}
/// Request message for [AdScheduleViewService.GetAdScheduleView][google.ads.googleads.v3.services.AdScheduleViewService.GetAdScheduleView].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAdScheduleViewRequest {
    /// Required. The resource name of the ad schedule view to fetch.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
#[doc = r" Generated client implementations."]
pub mod ad_schedule_view_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Service to fetch ad schedule views."]
    pub struct AdScheduleViewServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> AdScheduleViewServiceClient<T>
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
        #[doc = " Returns the requested ad schedule view in full detail."]
        pub async fn get_ad_schedule_view(
            &mut self,
            request: impl tonic::IntoRequest<super::GetAdScheduleViewRequest>,
        ) -> Result<tonic::Response<super::super::resources::AdScheduleView>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ads.googleads.v3.services.AdScheduleViewService/GetAdScheduleView",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for AdScheduleViewServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for AdScheduleViewServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "AdScheduleViewServiceClient {{ ... }}")
        }
    }
}
/// Request message for [AdService.GetAd][google.ads.googleads.v3.services.AdService.GetAd].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAdRequest {
    /// Required. The resource name of the ad to fetch.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
/// Request message for [AdService.MutateAds][google.ads.googleads.v3.services.AdService.MutateAds].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateAdsRequest {
    /// Required. The ID of the customer whose ads are being modified.
    #[prost(string, tag = "1")]
    pub customer_id: std::string::String,
    /// Required. The list of operations to perform on individual ads.
    #[prost(message, repeated, tag = "2")]
    pub operations: ::std::vec::Vec<AdOperation>,
}
/// A single update operation on an ad.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdOperation {
    /// FieldMask that determines which resource fields are modified in an update.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
    /// The mutate operation.
    #[prost(oneof = "ad_operation::Operation", tags = "1")]
    pub operation: ::std::option::Option<ad_operation::Operation>,
}
pub mod ad_operation {
    /// The mutate operation.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Operation {
        /// Update operation: The ad is expected to have a valid resource name
        /// in this format:
        ///
        /// `customers/{customer_id}/ads/{ad_id}`
        #[prost(message, tag = "1")]
        Update(super::super::resources::Ad),
    }
}
/// Response message for an ad mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateAdsResponse {
    /// All results for the mutate.
    #[prost(message, repeated, tag = "2")]
    pub results: ::std::vec::Vec<MutateAdResult>,
}
/// The result for the ad mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateAdResult {
    /// The resource name returned for successful operations.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
#[doc = r" Generated client implementations."]
pub mod ad_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Service to manage ads."]
    pub struct AdServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> AdServiceClient<T>
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
        #[doc = " Returns the requested ad in full detail."]
        pub async fn get_ad(
            &mut self,
            request: impl tonic::IntoRequest<super::GetAdRequest>,
        ) -> Result<tonic::Response<super::super::resources::Ad>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ads.googleads.v3.services.AdService/GetAd",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates ads. Operation statuses are returned. Updating ads is not supported"]
        #[doc = " for TextAd, ExpandedDynamicSearchAd, GmailAd and ImageAd."]
        pub async fn mutate_ads(
            &mut self,
            request: impl tonic::IntoRequest<super::MutateAdsRequest>,
        ) -> Result<tonic::Response<super::MutateAdsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ads.googleads.v3.services.AdService/MutateAds",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for AdServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for AdServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "AdServiceClient {{ ... }}")
        }
    }
}
/// Request message for [AgeRangeViewService.GetAgeRangeView][google.ads.googleads.v3.services.AgeRangeViewService.GetAgeRangeView].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAgeRangeViewRequest {
    /// Required. The resource name of the age range view to fetch.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
#[doc = r" Generated client implementations."]
pub mod age_range_view_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Service to manage age range views."]
    pub struct AgeRangeViewServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> AgeRangeViewServiceClient<T>
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
        #[doc = " Returns the requested age range view in full detail."]
        pub async fn get_age_range_view(
            &mut self,
            request: impl tonic::IntoRequest<super::GetAgeRangeViewRequest>,
        ) -> Result<tonic::Response<super::super::resources::AgeRangeView>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ads.googleads.v3.services.AgeRangeViewService/GetAgeRangeView",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for AgeRangeViewServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for AgeRangeViewServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "AgeRangeViewServiceClient {{ ... }}")
        }
    }
}
/// Request message for [AssetService.GetAsset][google.ads.googleads.v3.services.AssetService.GetAsset]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAssetRequest {
    /// Required. The resource name of the asset to fetch.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
/// Request message for [AssetService.MutateAssets][google.ads.googleads.v3.services.AssetService.MutateAssets]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateAssetsRequest {
    /// Required. The ID of the customer whose assets are being modified.
    #[prost(string, tag = "1")]
    pub customer_id: std::string::String,
    /// Required. The list of operations to perform on individual assets.
    #[prost(message, repeated, tag = "2")]
    pub operations: ::std::vec::Vec<AssetOperation>,
}
/// A single operation to create an asset. Supported asset types are
/// YoutubeVideoAsset, MediaBundleAsset, ImageAsset, and LeadFormAsset. TextAsset
/// should be created with Ad inline.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetOperation {
    /// The mutate operation.
    #[prost(oneof = "asset_operation::Operation", tags = "1")]
    pub operation: ::std::option::Option<asset_operation::Operation>,
}
pub mod asset_operation {
    /// The mutate operation.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Operation {
        /// Create operation: No resource name is expected for the new asset.
        #[prost(message, tag = "1")]
        Create(super::super::resources::Asset),
    }
}
/// Response message for an asset mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateAssetsResponse {
    /// All results for the mutate.
    #[prost(message, repeated, tag = "2")]
    pub results: ::std::vec::Vec<MutateAssetResult>,
}
/// The result for the asset mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateAssetResult {
    /// The resource name returned for successful operations.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
#[doc = r" Generated client implementations."]
pub mod asset_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Service to manage assets. Asset types can be created with AssetService are"]
    #[doc = " YoutubeVideoAsset, MediaBundleAsset and ImageAsset. TextAsset should be"]
    #[doc = " created with Ad inline."]
    pub struct AssetServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> AssetServiceClient<T>
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
        #[doc = " Returns the requested asset in full detail."]
        pub async fn get_asset(
            &mut self,
            request: impl tonic::IntoRequest<super::GetAssetRequest>,
        ) -> Result<tonic::Response<super::super::resources::Asset>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ads.googleads.v3.services.AssetService/GetAsset",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates assets. Operation statuses are returned."]
        pub async fn mutate_assets(
            &mut self,
            request: impl tonic::IntoRequest<super::MutateAssetsRequest>,
        ) -> Result<tonic::Response<super::MutateAssetsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ads.googleads.v3.services.AssetService/MutateAssets",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for AssetServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for AssetServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "AssetServiceClient {{ ... }}")
        }
    }
}
/// Request message for [BiddingStrategyService.GetBiddingStrategy][google.ads.googleads.v3.services.BiddingStrategyService.GetBiddingStrategy].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBiddingStrategyRequest {
    /// Required. The resource name of the bidding strategy to fetch.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
/// Request message for [BiddingStrategyService.MutateBiddingStrategies][google.ads.googleads.v3.services.BiddingStrategyService.MutateBiddingStrategies].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateBiddingStrategiesRequest {
    /// Required. The ID of the customer whose bidding strategies are being modified.
    #[prost(string, tag = "1")]
    pub customer_id: std::string::String,
    /// Required. The list of operations to perform on individual bidding strategies.
    #[prost(message, repeated, tag = "2")]
    pub operations: ::std::vec::Vec<BiddingStrategyOperation>,
    /// If true, successful operations will be carried out and invalid
    /// operations will return errors. If false, all operations will be carried
    /// out in one transaction if and only if they are all valid.
    /// Default is false.
    #[prost(bool, tag = "3")]
    pub partial_failure: bool,
    /// If true, the request is validated but not executed. Only errors are
    /// returned, not results.
    #[prost(bool, tag = "4")]
    pub validate_only: bool,
}
/// A single operation (create, update, remove) on a bidding strategy.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BiddingStrategyOperation {
    /// FieldMask that determines which resource fields are modified in an update.
    #[prost(message, optional, tag = "4")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
    /// The mutate operation.
    #[prost(oneof = "bidding_strategy_operation::Operation", tags = "1, 2, 3")]
    pub operation: ::std::option::Option<bidding_strategy_operation::Operation>,
}
pub mod bidding_strategy_operation {
    /// The mutate operation.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Operation {
        /// Create operation: No resource name is expected for the new bidding
        /// strategy.
        #[prost(message, tag = "1")]
        Create(super::super::resources::BiddingStrategy),
        /// Update operation: The bidding strategy is expected to have a valid
        /// resource name.
        #[prost(message, tag = "2")]
        Update(super::super::resources::BiddingStrategy),
        /// Remove operation: A resource name for the removed bidding strategy is
        /// expected, in this format:
        ///
        /// `customers/{customer_id}/biddingStrategies/{bidding_strategy_id}`
        #[prost(string, tag = "3")]
        Remove(std::string::String),
    }
}
/// Response message for bidding strategy mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateBiddingStrategiesResponse {
    /// Errors that pertain to operation failures in the partial failure mode.
    /// Returned only when partial_failure = true and all errors occur inside the
    /// operations. If any errors occur outside the operations (e.g. auth errors),
    /// we return an RPC level error.
    #[prost(message, optional, tag = "3")]
    pub partial_failure_error: ::std::option::Option<super::super::super::super::rpc::Status>,
    /// All results for the mutate.
    #[prost(message, repeated, tag = "2")]
    pub results: ::std::vec::Vec<MutateBiddingStrategyResult>,
}
/// The result for the bidding strategy mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateBiddingStrategyResult {
    /// Returned for successful operations.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
#[doc = r" Generated client implementations."]
pub mod bidding_strategy_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Service to manage bidding strategies."]
    pub struct BiddingStrategyServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> BiddingStrategyServiceClient<T>
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
        #[doc = " Returns the requested bidding strategy in full detail."]
        pub async fn get_bidding_strategy(
            &mut self,
            request: impl tonic::IntoRequest<super::GetBiddingStrategyRequest>,
        ) -> Result<tonic::Response<super::super::resources::BiddingStrategy>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ads.googleads.v3.services.BiddingStrategyService/GetBiddingStrategy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates, updates, or removes bidding strategies. Operation statuses are"]
        #[doc = " returned."]
        pub async fn mutate_bidding_strategies(
            &mut self,
            request: impl tonic::IntoRequest<super::MutateBiddingStrategiesRequest>,
        ) -> Result<tonic::Response<super::MutateBiddingStrategiesResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ads.googleads.v3.services.BiddingStrategyService/MutateBiddingStrategies",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for BiddingStrategyServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for BiddingStrategyServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "BiddingStrategyServiceClient {{ ... }}")
        }
    }
}
/// Request message for
/// [BillingSetupService.GetBillingSetup][google.ads.googleads.v3.services.BillingSetupService.GetBillingSetup].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBillingSetupRequest {
    /// Required. The resource name of the billing setup to fetch.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
/// Request message for billing setup mutate operations.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateBillingSetupRequest {
    /// Required. Id of the customer to apply the billing setup mutate operation to.
    #[prost(string, tag = "1")]
    pub customer_id: std::string::String,
    /// Required. The operation to perform.
    #[prost(message, optional, tag = "2")]
    pub operation: ::std::option::Option<BillingSetupOperation>,
}
/// A single operation on a billing setup, which describes the cancellation of an
/// existing billing setup.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BillingSetupOperation {
    /// Only one of these operations can be set. "Update" operations are not
    /// supported.
    #[prost(oneof = "billing_setup_operation::Operation", tags = "2, 1")]
    pub operation: ::std::option::Option<billing_setup_operation::Operation>,
}
pub mod billing_setup_operation {
    /// Only one of these operations can be set. "Update" operations are not
    /// supported.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Operation {
        /// Creates a billing setup. No resource name is expected for the new billing
        /// setup.
        #[prost(message, tag = "2")]
        Create(super::super::resources::BillingSetup),
        /// Resource name of the billing setup to remove. A setup cannot be
        /// removed unless it is in a pending state or its scheduled start time is in
        /// the future. The resource name looks like
        /// `customers/{customer_id}/billingSetups/{billing_id}`.
        #[prost(string, tag = "1")]
        Remove(std::string::String),
    }
}
/// Response message for a billing setup operation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateBillingSetupResponse {
    /// A result that identifies the resource affected by the mutate request.
    #[prost(message, optional, tag = "1")]
    pub result: ::std::option::Option<MutateBillingSetupResult>,
}
/// Result for a single billing setup mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateBillingSetupResult {
    /// Returned for successful operations.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
#[doc = r" Generated client implementations."]
pub mod billing_setup_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " A service for designating the business entity responsible for accrued costs."]
    #[doc = ""]
    #[doc = " A billing setup is associated with a payments account.  Billing-related"]
    #[doc = " activity for all billing setups associated with a particular payments account"]
    #[doc = " will appear on a single invoice generated monthly."]
    #[doc = ""]
    #[doc = " Mutates:"]
    #[doc = " The REMOVE operation cancels a pending billing setup."]
    #[doc = " The CREATE operation creates a new billing setup."]
    pub struct BillingSetupServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> BillingSetupServiceClient<T>
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
        #[doc = " Returns a billing setup."]
        pub async fn get_billing_setup(
            &mut self,
            request: impl tonic::IntoRequest<super::GetBillingSetupRequest>,
        ) -> Result<tonic::Response<super::super::resources::BillingSetup>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ads.googleads.v3.services.BillingSetupService/GetBillingSetup",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a billing setup, or cancels an existing billing setup."]
        pub async fn mutate_billing_setup(
            &mut self,
            request: impl tonic::IntoRequest<super::MutateBillingSetupRequest>,
        ) -> Result<tonic::Response<super::MutateBillingSetupResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ads.googleads.v3.services.BillingSetupService/MutateBillingSetup",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for BillingSetupServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for BillingSetupServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "BillingSetupServiceClient {{ ... }}")
        }
    }
}
/// Request message for [CampaignAudienceViewService.GetCampaignAudienceView][google.ads.googleads.v3.services.CampaignAudienceViewService.GetCampaignAudienceView].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCampaignAudienceViewRequest {
    /// Required. The resource name of the campaign audience view to fetch.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
#[doc = r" Generated client implementations."]
pub mod campaign_audience_view_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Service to manage campaign audience views."]
    pub struct CampaignAudienceViewServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> CampaignAudienceViewServiceClient<T>
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
        #[doc = " Returns the requested campaign audience view in full detail."]
        pub async fn get_campaign_audience_view(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCampaignAudienceViewRequest>,
        ) -> Result<tonic::Response<super::super::resources::CampaignAudienceView>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ( "/google.ads.googleads.v3.services.CampaignAudienceViewService/GetCampaignAudienceView" ) ;
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for CampaignAudienceViewServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for CampaignAudienceViewServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "CampaignAudienceViewServiceClient {{ ... }}")
        }
    }
}
/// Request message for [CampaignBidModifierService.GetCampaignBidModifier][google.ads.googleads.v3.services.CampaignBidModifierService.GetCampaignBidModifier].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCampaignBidModifierRequest {
    /// Required. The resource name of the campaign bid modifier to fetch.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
/// Request message for
/// [CampaignBidModifierService.MutateCampaignBidModifiers][google.ads.googleads.v3.services.CampaignBidModifierService.MutateCampaignBidModifiers].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateCampaignBidModifiersRequest {
    /// Required. ID of the customer whose campaign bid modifiers are being modified.
    #[prost(string, tag = "1")]
    pub customer_id: std::string::String,
    /// Required. The list of operations to perform on individual campaign bid modifiers.
    #[prost(message, repeated, tag = "2")]
    pub operations: ::std::vec::Vec<CampaignBidModifierOperation>,
    /// If true, successful operations will be carried out and invalid
    /// operations will return errors. If false, all operations will be carried
    /// out in one transaction if and only if they are all valid.
    /// Default is false.
    #[prost(bool, tag = "3")]
    pub partial_failure: bool,
    /// If true, the request is validated but not executed. Only errors are
    /// returned, not results.
    #[prost(bool, tag = "4")]
    pub validate_only: bool,
}
/// A single operation (create, remove, update) on a campaign bid modifier.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CampaignBidModifierOperation {
    /// FieldMask that determines which resource fields are modified in an update.
    #[prost(message, optional, tag = "4")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
    /// The mutate operation.
    #[prost(oneof = "campaign_bid_modifier_operation::Operation", tags = "1, 2, 3")]
    pub operation: ::std::option::Option<campaign_bid_modifier_operation::Operation>,
}
pub mod campaign_bid_modifier_operation {
    /// The mutate operation.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Operation {
        /// Create operation: No resource name is expected for the new campaign bid
        /// modifier.
        #[prost(message, tag = "1")]
        Create(super::super::resources::CampaignBidModifier),
        /// Update operation: The campaign bid modifier is expected to have a valid
        /// resource name.
        #[prost(message, tag = "2")]
        Update(super::super::resources::CampaignBidModifier),
        /// Remove operation: A resource name for the removed campaign bid modifier
        /// is expected, in this format:
        ///
        /// `customers/{customer_id}/CampaignBidModifiers/{campaign_id}~{criterion_id}`
        #[prost(string, tag = "3")]
        Remove(std::string::String),
    }
}
/// Response message for campaign bid modifiers mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateCampaignBidModifiersResponse {
    /// Errors that pertain to operation failures in the partial failure mode.
    /// Returned only when partial_failure = true and all errors occur inside the
    /// operations. If any errors occur outside the operations (e.g. auth errors),
    /// we return an RPC level error.
    #[prost(message, optional, tag = "3")]
    pub partial_failure_error: ::std::option::Option<super::super::super::super::rpc::Status>,
    /// All results for the mutate.
    #[prost(message, repeated, tag = "2")]
    pub results: ::std::vec::Vec<MutateCampaignBidModifierResult>,
}
/// The result for the criterion mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateCampaignBidModifierResult {
    /// Returned for successful operations.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
#[doc = r" Generated client implementations."]
pub mod campaign_bid_modifier_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Service to manage campaign bid modifiers."]
    pub struct CampaignBidModifierServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> CampaignBidModifierServiceClient<T>
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
        #[doc = " Returns the requested campaign bid modifier in full detail."]
        pub async fn get_campaign_bid_modifier(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCampaignBidModifierRequest>,
        ) -> Result<tonic::Response<super::super::resources::CampaignBidModifier>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ( "/google.ads.googleads.v3.services.CampaignBidModifierService/GetCampaignBidModifier" ) ;
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates, updates, or removes campaign bid modifiers."]
        #[doc = " Operation statuses are returned."]
        pub async fn mutate_campaign_bid_modifiers(
            &mut self,
            request: impl tonic::IntoRequest<super::MutateCampaignBidModifiersRequest>,
        ) -> Result<tonic::Response<super::MutateCampaignBidModifiersResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ( "/google.ads.googleads.v3.services.CampaignBidModifierService/MutateCampaignBidModifiers" ) ;
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for CampaignBidModifierServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for CampaignBidModifierServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "CampaignBidModifierServiceClient {{ ... }}")
        }
    }
}
/// Request message for [CampaignBudgetService.GetCampaignBudget][google.ads.googleads.v3.services.CampaignBudgetService.GetCampaignBudget].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCampaignBudgetRequest {
    /// Required. The resource name of the campaign budget to fetch.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
/// Request message for [CampaignBudgetService.MutateCampaignBudgets][google.ads.googleads.v3.services.CampaignBudgetService.MutateCampaignBudgets].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateCampaignBudgetsRequest {
    /// Required. The ID of the customer whose campaign budgets are being modified.
    #[prost(string, tag = "1")]
    pub customer_id: std::string::String,
    /// Required. The list of operations to perform on individual campaign budgets.
    #[prost(message, repeated, tag = "2")]
    pub operations: ::std::vec::Vec<CampaignBudgetOperation>,
    /// If true, successful operations will be carried out and invalid
    /// operations will return errors. If false, all operations will be carried
    /// out in one transaction if and only if they are all valid.
    /// Default is false.
    #[prost(bool, tag = "3")]
    pub partial_failure: bool,
    /// If true, the request is validated but not executed. Only errors are
    /// returned, not results.
    #[prost(bool, tag = "4")]
    pub validate_only: bool,
}
/// A single operation (create, update, remove) on a campaign budget.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CampaignBudgetOperation {
    /// FieldMask that determines which resource fields are modified in an update.
    #[prost(message, optional, tag = "4")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
    /// The mutate operation.
    #[prost(oneof = "campaign_budget_operation::Operation", tags = "1, 2, 3")]
    pub operation: ::std::option::Option<campaign_budget_operation::Operation>,
}
pub mod campaign_budget_operation {
    /// The mutate operation.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Operation {
        /// Create operation: No resource name is expected for the new budget.
        #[prost(message, tag = "1")]
        Create(super::super::resources::CampaignBudget),
        /// Update operation: The campaign budget is expected to have a valid
        /// resource name.
        #[prost(message, tag = "2")]
        Update(super::super::resources::CampaignBudget),
        /// Remove operation: A resource name for the removed budget is expected, in
        /// this format:
        ///
        /// `customers/{customer_id}/campaignBudgets/{budget_id}`
        #[prost(string, tag = "3")]
        Remove(std::string::String),
    }
}
/// Response message for campaign budget mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateCampaignBudgetsResponse {
    /// Errors that pertain to operation failures in the partial failure mode.
    /// Returned only when partial_failure = true and all errors occur inside the
    /// operations. If any errors occur outside the operations (e.g. auth errors),
    /// we return an RPC level error.
    #[prost(message, optional, tag = "3")]
    pub partial_failure_error: ::std::option::Option<super::super::super::super::rpc::Status>,
    /// All results for the mutate.
    #[prost(message, repeated, tag = "2")]
    pub results: ::std::vec::Vec<MutateCampaignBudgetResult>,
}
/// The result for the campaign budget mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateCampaignBudgetResult {
    /// Returned for successful operations.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
#[doc = r" Generated client implementations."]
pub mod campaign_budget_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Service to manage campaign budgets."]
    pub struct CampaignBudgetServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> CampaignBudgetServiceClient<T>
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
        #[doc = " Returns the requested Campaign Budget in full detail."]
        pub async fn get_campaign_budget(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCampaignBudgetRequest>,
        ) -> Result<tonic::Response<super::super::resources::CampaignBudget>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ads.googleads.v3.services.CampaignBudgetService/GetCampaignBudget",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates, updates, or removes campaign budgets. Operation statuses are"]
        #[doc = " returned."]
        pub async fn mutate_campaign_budgets(
            &mut self,
            request: impl tonic::IntoRequest<super::MutateCampaignBudgetsRequest>,
        ) -> Result<tonic::Response<super::MutateCampaignBudgetsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ads.googleads.v3.services.CampaignBudgetService/MutateCampaignBudgets",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for CampaignBudgetServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for CampaignBudgetServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "CampaignBudgetServiceClient {{ ... }}")
        }
    }
}
/// Request message for [CampaignCriterionService.GetCampaignCriterion][google.ads.googleads.v3.services.CampaignCriterionService.GetCampaignCriterion].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCampaignCriterionRequest {
    /// Required. The resource name of the criterion to fetch.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
/// Request message for [CampaignCriterionService.MutateCampaignCriteria][google.ads.googleads.v3.services.CampaignCriterionService.MutateCampaignCriteria].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateCampaignCriteriaRequest {
    /// Required. The ID of the customer whose criteria are being modified.
    #[prost(string, tag = "1")]
    pub customer_id: std::string::String,
    /// Required. The list of operations to perform on individual criteria.
    #[prost(message, repeated, tag = "2")]
    pub operations: ::std::vec::Vec<CampaignCriterionOperation>,
    /// If true, successful operations will be carried out and invalid
    /// operations will return errors. If false, all operations will be carried
    /// out in one transaction if and only if they are all valid.
    /// Default is false.
    #[prost(bool, tag = "3")]
    pub partial_failure: bool,
    /// If true, the request is validated but not executed. Only errors are
    /// returned, not results.
    #[prost(bool, tag = "4")]
    pub validate_only: bool,
}
/// A single operation (create, update, remove) on a campaign criterion.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CampaignCriterionOperation {
    /// FieldMask that determines which resource fields are modified in an update.
    #[prost(message, optional, tag = "4")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
    /// The mutate operation.
    #[prost(oneof = "campaign_criterion_operation::Operation", tags = "1, 2, 3")]
    pub operation: ::std::option::Option<campaign_criterion_operation::Operation>,
}
pub mod campaign_criterion_operation {
    /// The mutate operation.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Operation {
        /// Create operation: No resource name is expected for the new criterion.
        #[prost(message, tag = "1")]
        Create(super::super::resources::CampaignCriterion),
        /// Update operation: The criterion is expected to have a valid resource
        /// name.
        #[prost(message, tag = "2")]
        Update(super::super::resources::CampaignCriterion),
        /// Remove operation: A resource name for the removed criterion is expected,
        /// in this format:
        ///
        /// `customers/{customer_id}/campaignCriteria/{campaign_id}~{criterion_id}`
        #[prost(string, tag = "3")]
        Remove(std::string::String),
    }
}
/// Response message for campaign criterion mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateCampaignCriteriaResponse {
    /// Errors that pertain to operation failures in the partial failure mode.
    /// Returned only when partial_failure = true and all errors occur inside the
    /// operations. If any errors occur outside the operations (e.g. auth errors),
    /// we return an RPC level error.
    #[prost(message, optional, tag = "3")]
    pub partial_failure_error: ::std::option::Option<super::super::super::super::rpc::Status>,
    /// All results for the mutate.
    #[prost(message, repeated, tag = "2")]
    pub results: ::std::vec::Vec<MutateCampaignCriterionResult>,
}
/// The result for the criterion mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateCampaignCriterionResult {
    /// Returned for successful operations.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
#[doc = r" Generated client implementations."]
pub mod campaign_criterion_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Service to manage campaign criteria."]
    pub struct CampaignCriterionServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> CampaignCriterionServiceClient<T>
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
        #[doc = " Returns the requested criterion in full detail."]
        pub async fn get_campaign_criterion(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCampaignCriterionRequest>,
        ) -> Result<tonic::Response<super::super::resources::CampaignCriterion>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ads.googleads.v3.services.CampaignCriterionService/GetCampaignCriterion",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates, updates, or removes criteria. Operation statuses are returned."]
        pub async fn mutate_campaign_criteria(
            &mut self,
            request: impl tonic::IntoRequest<super::MutateCampaignCriteriaRequest>,
        ) -> Result<tonic::Response<super::MutateCampaignCriteriaResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ads.googleads.v3.services.CampaignCriterionService/MutateCampaignCriteria",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for CampaignCriterionServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for CampaignCriterionServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "CampaignCriterionServiceClient {{ ... }}")
        }
    }
}
/// Request message for
/// [CampaignCriterionSimulationService.GetCampaignCriterionSimulation][google.ads.googleads.v3.services.CampaignCriterionSimulationService.GetCampaignCriterionSimulation].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCampaignCriterionSimulationRequest {
    /// Required. The resource name of the campaign criterion simulation to fetch.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
#[doc = r" Generated client implementations."]
pub mod campaign_criterion_simulation_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Service to fetch campaign criterion simulations."]
    pub struct CampaignCriterionSimulationServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> CampaignCriterionSimulationServiceClient<T>
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
        #[doc = " Returns the requested campaign criterion simulation in full detail."]
        pub async fn get_campaign_criterion_simulation(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCampaignCriterionSimulationRequest>,
        ) -> Result<
            tonic::Response<super::super::resources::CampaignCriterionSimulation>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ( "/google.ads.googleads.v3.services.CampaignCriterionSimulationService/GetCampaignCriterionSimulation" ) ;
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for CampaignCriterionSimulationServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for CampaignCriterionSimulationServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "CampaignCriterionSimulationServiceClient {{ ... }}")
        }
    }
}
/// Request message for [CampaignDraftService.GetCampaignDraft][google.ads.googleads.v3.services.CampaignDraftService.GetCampaignDraft].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCampaignDraftRequest {
    /// Required. The resource name of the campaign draft to fetch.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
/// Request message for [CampaignDraftService.MutateCampaignDrafts][google.ads.googleads.v3.services.CampaignDraftService.MutateCampaignDrafts].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateCampaignDraftsRequest {
    /// Required. The ID of the customer whose campaign drafts are being modified.
    #[prost(string, tag = "1")]
    pub customer_id: std::string::String,
    /// Required. The list of operations to perform on individual campaign drafts.
    #[prost(message, repeated, tag = "2")]
    pub operations: ::std::vec::Vec<CampaignDraftOperation>,
    /// If true, successful operations will be carried out and invalid
    /// operations will return errors. If false, all operations will be carried
    /// out in one transaction if and only if they are all valid.
    /// Default is false.
    #[prost(bool, tag = "3")]
    pub partial_failure: bool,
    /// If true, the request is validated but not executed. Only errors are
    /// returned, not results.
    #[prost(bool, tag = "4")]
    pub validate_only: bool,
}
/// Request message for [CampaignDraftService.PromoteCampaignDraft][google.ads.googleads.v3.services.CampaignDraftService.PromoteCampaignDraft].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PromoteCampaignDraftRequest {
    /// Required. The resource name of the campaign draft to promote.
    #[prost(string, tag = "1")]
    pub campaign_draft: std::string::String,
}
/// A single operation (create, update, remove) on a campaign draft.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CampaignDraftOperation {
    /// FieldMask that determines which resource fields are modified in an update.
    #[prost(message, optional, tag = "4")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
    /// The mutate operation.
    #[prost(oneof = "campaign_draft_operation::Operation", tags = "1, 2, 3")]
    pub operation: ::std::option::Option<campaign_draft_operation::Operation>,
}
pub mod campaign_draft_operation {
    /// The mutate operation.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Operation {
        /// Create operation: No resource name is expected for the new campaign
        /// draft.
        #[prost(message, tag = "1")]
        Create(super::super::resources::CampaignDraft),
        /// Update operation: The campaign draft is expected to have a valid
        /// resource name.
        #[prost(message, tag = "2")]
        Update(super::super::resources::CampaignDraft),
        /// Remove operation: The campaign draft is expected to have a valid
        /// resource name, in this format:
        ///
        /// `customers/{customer_id}/campaignDrafts/{base_campaign_id}~{draft_id}`
        #[prost(string, tag = "3")]
        Remove(std::string::String),
    }
}
/// Response message for campaign draft mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateCampaignDraftsResponse {
    /// Errors that pertain to operation failures in the partial failure mode.
    /// Returned only when partial_failure = true and all errors occur inside the
    /// operations. If any errors occur outside the operations (e.g. auth errors),
    /// we return an RPC level error.
    #[prost(message, optional, tag = "3")]
    pub partial_failure_error: ::std::option::Option<super::super::super::super::rpc::Status>,
    /// All results for the mutate.
    #[prost(message, repeated, tag = "2")]
    pub results: ::std::vec::Vec<MutateCampaignDraftResult>,
}
/// The result for the campaign draft mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateCampaignDraftResult {
    /// Returned for successful operations.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
/// Request message for [CampaignDraftService.ListCampaignDraftAsyncErrors][google.ads.googleads.v3.services.CampaignDraftService.ListCampaignDraftAsyncErrors].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCampaignDraftAsyncErrorsRequest {
    /// Required. The name of the campaign draft from which to retrieve the async errors.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
    /// Token of the page to retrieve. If not specified, the first
    /// page of results will be returned. Use the value obtained from
    /// `next_page_token` in the previous response in order to request
    /// the next page of results.
    #[prost(string, tag = "2")]
    pub page_token: std::string::String,
    /// Number of elements to retrieve in a single page.
    /// When a page request is too large, the server may decide to
    /// further limit the number of returned resources.
    #[prost(int32, tag = "3")]
    pub page_size: i32,
}
/// Response message for [CampaignDraftService.ListCampaignDraftAsyncErrors][google.ads.googleads.v3.services.CampaignDraftService.ListCampaignDraftAsyncErrors].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCampaignDraftAsyncErrorsResponse {
    /// Details of the errors when performing the asynchronous operation.
    #[prost(message, repeated, tag = "1")]
    pub errors: ::std::vec::Vec<super::super::super::super::rpc::Status>,
    /// Pagination token used to retrieve the next page of results.
    /// Pass the content of this string as the `page_token` attribute of
    /// the next request. `next_page_token` is not returned for the last
    /// page.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
}
#[doc = r" Generated client implementations."]
pub mod campaign_draft_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Service to manage campaign drafts."]
    pub struct CampaignDraftServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> CampaignDraftServiceClient<T>
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
        #[doc = " Returns the requested campaign draft in full detail."]
        pub async fn get_campaign_draft(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCampaignDraftRequest>,
        ) -> Result<tonic::Response<super::super::resources::CampaignDraft>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ads.googleads.v3.services.CampaignDraftService/GetCampaignDraft",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates, updates, or removes campaign drafts. Operation statuses are"]
        #[doc = " returned."]
        pub async fn mutate_campaign_drafts(
            &mut self,
            request: impl tonic::IntoRequest<super::MutateCampaignDraftsRequest>,
        ) -> Result<tonic::Response<super::MutateCampaignDraftsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ads.googleads.v3.services.CampaignDraftService/MutateCampaignDrafts",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Promotes the changes in a draft back to the base campaign."]
        #[doc = ""]
        #[doc = " This method returns a Long Running Operation (LRO) indicating if the"]
        #[doc = " Promote is done. Use [Operations.GetOperation] to poll the LRO until it"]
        #[doc = " is done. Only a done status is returned in the response. See the status"]
        #[doc = " in the Campaign Draft resource to determine if the promotion was"]
        #[doc = " successful. If the LRO failed, use"]
        #[doc = " [CampaignDraftService.ListCampaignDraftAsyncErrors][google.ads.googleads.v3.services.CampaignDraftService.ListCampaignDraftAsyncErrors] to view the list of"]
        #[doc = " error reasons."]
        pub async fn promote_campaign_draft(
            &mut self,
            request: impl tonic::IntoRequest<super::PromoteCampaignDraftRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::super::longrunning::Operation>,
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
                "/google.ads.googleads.v3.services.CampaignDraftService/PromoteCampaignDraft",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns all errors that occurred during CampaignDraft promote. Throws an"]
        #[doc = " error if called before campaign draft is promoted."]
        #[doc = " Supports standard list paging."]
        pub async fn list_campaign_draft_async_errors(
            &mut self,
            request: impl tonic::IntoRequest<super::ListCampaignDraftAsyncErrorsRequest>,
        ) -> Result<tonic::Response<super::ListCampaignDraftAsyncErrorsResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ( "/google.ads.googleads.v3.services.CampaignDraftService/ListCampaignDraftAsyncErrors" ) ;
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for CampaignDraftServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for CampaignDraftServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "CampaignDraftServiceClient {{ ... }}")
        }
    }
}
/// Request message for [CampaignExperimentService.GetCampaignExperiment][google.ads.googleads.v3.services.CampaignExperimentService.GetCampaignExperiment].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCampaignExperimentRequest {
    /// Required. The resource name of the campaign experiment to fetch.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
/// Request message for [CampaignExperimentService.MutateCampaignExperiments][google.ads.googleads.v3.services.CampaignExperimentService.MutateCampaignExperiments].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateCampaignExperimentsRequest {
    /// Required. The ID of the customer whose campaign experiments are being modified.
    #[prost(string, tag = "1")]
    pub customer_id: std::string::String,
    /// Required. The list of operations to perform on individual campaign experiments.
    #[prost(message, repeated, tag = "2")]
    pub operations: ::std::vec::Vec<CampaignExperimentOperation>,
    /// If true, successful operations will be carried out and invalid
    /// operations will return errors. If false, all operations will be carried
    /// out in one transaction if and only if they are all valid.
    /// Default is false.
    #[prost(bool, tag = "3")]
    pub partial_failure: bool,
    /// If true, the request is validated but not executed. Only errors are
    /// returned, not results.
    #[prost(bool, tag = "4")]
    pub validate_only: bool,
}
/// A single update operation on a campaign experiment.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CampaignExperimentOperation {
    /// FieldMask that determines which resource fields are modified in an update.
    #[prost(message, optional, tag = "3")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
    /// The mutate operation.
    #[prost(oneof = "campaign_experiment_operation::Operation", tags = "1, 2")]
    pub operation: ::std::option::Option<campaign_experiment_operation::Operation>,
}
pub mod campaign_experiment_operation {
    /// The mutate operation.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Operation {
        /// Update operation: The campaign experiment is expected to have a valid
        /// resource name.
        #[prost(message, tag = "1")]
        Update(super::super::resources::CampaignExperiment),
        /// Remove operation: The campaign experiment is expected to have a valid
        /// resource name, in this format:
        ///
        /// `customers/{customer_id}/campaignExperiments/{campaign_experiment_id}`
        #[prost(string, tag = "2")]
        Remove(std::string::String),
    }
}
/// Response message for campaign experiment mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateCampaignExperimentsResponse {
    /// Errors that pertain to operation failures in the partial failure mode.
    /// Returned only when partial_failure = true and all errors occur inside the
    /// operations. If any errors occur outside the operations (e.g. auth errors),
    /// we return an RPC level error.
    #[prost(message, optional, tag = "3")]
    pub partial_failure_error: ::std::option::Option<super::super::super::super::rpc::Status>,
    /// All results for the mutate.
    #[prost(message, repeated, tag = "2")]
    pub results: ::std::vec::Vec<MutateCampaignExperimentResult>,
}
/// The result for the campaign experiment mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateCampaignExperimentResult {
    /// Returned for successful operations.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
/// Request message for [CampaignExperimentService.CreateCampaignExperiment][google.ads.googleads.v3.services.CampaignExperimentService.CreateCampaignExperiment].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateCampaignExperimentRequest {
    /// Required. The ID of the customer whose campaign experiment is being created.
    #[prost(string, tag = "1")]
    pub customer_id: std::string::String,
    /// Required. The campaign experiment to be created.
    #[prost(message, optional, tag = "2")]
    pub campaign_experiment: ::std::option::Option<super::resources::CampaignExperiment>,
    /// If true, the request is validated but not executed. Only errors are
    /// returned, not results.
    #[prost(bool, tag = "3")]
    pub validate_only: bool,
}
/// Message used as metadata returned in Long Running Operations for
/// CreateCampaignExperimentRequest
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateCampaignExperimentMetadata {
    /// Resource name of campaign experiment created.
    #[prost(string, tag = "1")]
    pub campaign_experiment: std::string::String,
}
/// Request message for [CampaignExperimentService.GraduateCampaignExperiment][google.ads.googleads.v3.services.CampaignExperimentService.GraduateCampaignExperiment].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GraduateCampaignExperimentRequest {
    /// Required. The resource name of the campaign experiment to graduate.
    #[prost(string, tag = "1")]
    pub campaign_experiment: std::string::String,
    /// Required. Resource name of the budget to attach to the campaign graduated from the
    /// experiment.
    #[prost(string, tag = "2")]
    pub campaign_budget: std::string::String,
}
/// Response message for campaign experiment graduate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GraduateCampaignExperimentResponse {
    /// The resource name of the campaign from the graduated experiment.
    /// This campaign is the same one as CampaignExperiment.experiment_campaign.
    #[prost(string, tag = "1")]
    pub graduated_campaign: std::string::String,
}
/// Request message for [CampaignExperimentService.PromoteCampaignExperiment][google.ads.googleads.v3.services.CampaignExperimentService.PromoteCampaignExperiment].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PromoteCampaignExperimentRequest {
    /// Required. The resource name of the campaign experiment to promote.
    #[prost(string, tag = "1")]
    pub campaign_experiment: std::string::String,
}
/// Request message for [CampaignExperimentService.EndCampaignExperiment][google.ads.googleads.v3.services.CampaignExperimentService.EndCampaignExperiment].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EndCampaignExperimentRequest {
    /// Required. The resource name of the campaign experiment to end.
    #[prost(string, tag = "1")]
    pub campaign_experiment: std::string::String,
}
/// Request message for
/// [CampaignExperimentService.ListCampaignExperimentAsyncErrors][google.ads.googleads.v3.services.CampaignExperimentService.ListCampaignExperimentAsyncErrors].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCampaignExperimentAsyncErrorsRequest {
    /// Required. The name of the campaign experiment from which to retrieve the async
    /// errors.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
    /// Token of the page to retrieve. If not specified, the first
    /// page of results will be returned. Use the value obtained from
    /// `next_page_token` in the previous response in order to request
    /// the next page of results.
    #[prost(string, tag = "2")]
    pub page_token: std::string::String,
    /// Number of elements to retrieve in a single page.
    /// When a page request is too large, the server may decide to
    /// further limit the number of returned resources.
    #[prost(int32, tag = "3")]
    pub page_size: i32,
}
/// Response message for
/// [CampaignExperimentService.ListCampaignExperimentAsyncErrors][google.ads.googleads.v3.services.CampaignExperimentService.ListCampaignExperimentAsyncErrors].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCampaignExperimentAsyncErrorsResponse {
    /// Details of the errors when performing the asynchronous operation.
    #[prost(message, repeated, tag = "1")]
    pub errors: ::std::vec::Vec<super::super::super::super::rpc::Status>,
    /// Pagination token used to retrieve the next page of results.
    /// Pass the content of this string as the `page_token` attribute of
    /// the next request. `next_page_token` is not returned for the last
    /// page.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
}
#[doc = r" Generated client implementations."]
pub mod campaign_experiment_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " CampaignExperimentService manages the life cycle of campaign experiments."]
    #[doc = " It is used to create new experiments from drafts, modify experiment"]
    #[doc = " properties, promote changes in an experiment back to its base campaign,"]
    #[doc = " graduate experiments into new stand-alone campaigns, and to remove an"]
    #[doc = " experiment."]
    #[doc = ""]
    #[doc = " An experiment consists of two variants or arms - the base campaign and the"]
    #[doc = " experiment campaign, directing a fixed share of traffic to each arm."]
    #[doc = " A campaign experiment is created from a draft of changes to the base campaign"]
    #[doc = " and will be a snapshot of changes in the draft at the time of creation."]
    pub struct CampaignExperimentServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> CampaignExperimentServiceClient<T>
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
        #[doc = " Returns the requested campaign experiment in full detail."]
        pub async fn get_campaign_experiment(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCampaignExperimentRequest>,
        ) -> Result<tonic::Response<super::super::resources::CampaignExperiment>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ads.googleads.v3.services.CampaignExperimentService/GetCampaignExperiment",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a campaign experiment based on a campaign draft. The draft campaign"]
        #[doc = " will be forked into a real campaign (called the experiment campaign) that"]
        #[doc = " will begin serving ads if successfully created."]
        #[doc = ""]
        #[doc = " The campaign experiment is created immediately with status INITIALIZING."]
        #[doc = " This method return a long running operation that tracks the forking of the"]
        #[doc = " draft campaign. If the forking fails, a list of errors can be retrieved"]
        #[doc = " using the ListCampaignExperimentAsyncErrors method. The operation's"]
        #[doc = " metadata will be a StringValue containing the resource name of the created"]
        #[doc = " campaign experiment."]
        pub async fn create_campaign_experiment(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateCampaignExperimentRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ( "/google.ads.googleads.v3.services.CampaignExperimentService/CreateCampaignExperiment" ) ;
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates campaign experiments. Operation statuses are returned."]
        pub async fn mutate_campaign_experiments(
            &mut self,
            request: impl tonic::IntoRequest<super::MutateCampaignExperimentsRequest>,
        ) -> Result<tonic::Response<super::MutateCampaignExperimentsResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ( "/google.ads.googleads.v3.services.CampaignExperimentService/MutateCampaignExperiments" ) ;
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Graduates a campaign experiment to a full campaign. The base and experiment"]
        #[doc = " campaigns will start running independently with their own budgets."]
        pub async fn graduate_campaign_experiment(
            &mut self,
            request: impl tonic::IntoRequest<super::GraduateCampaignExperimentRequest>,
        ) -> Result<tonic::Response<super::GraduateCampaignExperimentResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ( "/google.ads.googleads.v3.services.CampaignExperimentService/GraduateCampaignExperiment" ) ;
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Promotes the changes in a experiment campaign back to the base campaign."]
        #[doc = ""]
        #[doc = " The campaign experiment is updated immediately with status PROMOTING."]
        #[doc = " This method return a long running operation that tracks the promoting of"]
        #[doc = " the experiment campaign. If the promoting fails, a list of errors can be"]
        #[doc = " retrieved using the ListCampaignExperimentAsyncErrors method."]
        pub async fn promote_campaign_experiment(
            &mut self,
            request: impl tonic::IntoRequest<super::PromoteCampaignExperimentRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ( "/google.ads.googleads.v3.services.CampaignExperimentService/PromoteCampaignExperiment" ) ;
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Immediately ends a campaign experiment, changing the experiment's scheduled"]
        #[doc = " end date and without waiting for end of day. End date is updated to be the"]
        #[doc = " time of the request."]
        pub async fn end_campaign_experiment(
            &mut self,
            request: impl tonic::IntoRequest<super::EndCampaignExperimentRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ads.googleads.v3.services.CampaignExperimentService/EndCampaignExperiment",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns all errors that occurred during CampaignExperiment create or"]
        #[doc = " promote (whichever occurred last)."]
        #[doc = " Supports standard list paging."]
        pub async fn list_campaign_experiment_async_errors(
            &mut self,
            request: impl tonic::IntoRequest<super::ListCampaignExperimentAsyncErrorsRequest>,
        ) -> Result<tonic::Response<super::ListCampaignExperimentAsyncErrorsResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ( "/google.ads.googleads.v3.services.CampaignExperimentService/ListCampaignExperimentAsyncErrors" ) ;
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for CampaignExperimentServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for CampaignExperimentServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "CampaignExperimentServiceClient {{ ... }}")
        }
    }
}
/// Request message for
/// [CampaignExtensionSettingService.GetCampaignExtensionSetting][google.ads.googleads.v3.services.CampaignExtensionSettingService.GetCampaignExtensionSetting].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCampaignExtensionSettingRequest {
    /// Required. The resource name of the campaign extension setting to fetch.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
/// Request message for
/// [CampaignExtensionSettingService.MutateCampaignExtensionSettings][google.ads.googleads.v3.services.CampaignExtensionSettingService.MutateCampaignExtensionSettings].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateCampaignExtensionSettingsRequest {
    /// Required. The ID of the customer whose campaign extension settings are being
    /// modified.
    #[prost(string, tag = "1")]
    pub customer_id: std::string::String,
    /// Required. The list of operations to perform on individual campaign extension
    /// settings.
    #[prost(message, repeated, tag = "2")]
    pub operations: ::std::vec::Vec<CampaignExtensionSettingOperation>,
    /// If true, successful operations will be carried out and invalid
    /// operations will return errors. If false, all operations will be carried
    /// out in one transaction if and only if they are all valid.
    /// Default is false.
    #[prost(bool, tag = "3")]
    pub partial_failure: bool,
    /// If true, the request is validated but not executed. Only errors are
    /// returned, not results.
    #[prost(bool, tag = "4")]
    pub validate_only: bool,
}
/// A single operation (create, update, remove) on a campaign extension setting.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CampaignExtensionSettingOperation {
    /// FieldMask that determines which resource fields are modified in an update.
    #[prost(message, optional, tag = "4")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
    /// The mutate operation.
    #[prost(
        oneof = "campaign_extension_setting_operation::Operation",
        tags = "1, 2, 3"
    )]
    pub operation: ::std::option::Option<campaign_extension_setting_operation::Operation>,
}
pub mod campaign_extension_setting_operation {
    /// The mutate operation.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Operation {
        /// Create operation: No resource name is expected for the new campaign
        /// extension setting.
        #[prost(message, tag = "1")]
        Create(super::super::resources::CampaignExtensionSetting),
        /// Update operation: The campaign extension setting is expected to have a
        /// valid resource name.
        #[prost(message, tag = "2")]
        Update(super::super::resources::CampaignExtensionSetting),
        /// Remove operation: A resource name for the removed campaign extension
        /// setting is expected, in this format:
        ///
        /// `customers/{customer_id}/campaignExtensionSettings/{campaign_id}~{extension_type}`
        #[prost(string, tag = "3")]
        Remove(std::string::String),
    }
}
/// Response message for a campaign extension setting mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateCampaignExtensionSettingsResponse {
    /// Errors that pertain to operation failures in the partial failure mode.
    /// Returned only when partial_failure = true and all errors occur inside the
    /// operations. If any errors occur outside the operations (e.g. auth errors),
    /// we return an RPC level error.
    #[prost(message, optional, tag = "3")]
    pub partial_failure_error: ::std::option::Option<super::super::super::super::rpc::Status>,
    /// All results for the mutate.
    #[prost(message, repeated, tag = "2")]
    pub results: ::std::vec::Vec<MutateCampaignExtensionSettingResult>,
}
/// The result for the campaign extension setting mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateCampaignExtensionSettingResult {
    /// Returned for successful operations.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
#[doc = r" Generated client implementations."]
pub mod campaign_extension_setting_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Service to manage campaign extension settings."]
    pub struct CampaignExtensionSettingServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> CampaignExtensionSettingServiceClient<T>
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
        #[doc = " Returns the requested campaign extension setting in full detail."]
        pub async fn get_campaign_extension_setting(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCampaignExtensionSettingRequest>,
        ) -> Result<tonic::Response<super::super::resources::CampaignExtensionSetting>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ( "/google.ads.googleads.v3.services.CampaignExtensionSettingService/GetCampaignExtensionSetting" ) ;
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates, updates, or removes campaign extension settings. Operation"]
        #[doc = " statuses are returned."]
        pub async fn mutate_campaign_extension_settings(
            &mut self,
            request: impl tonic::IntoRequest<super::MutateCampaignExtensionSettingsRequest>,
        ) -> Result<tonic::Response<super::MutateCampaignExtensionSettingsResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ( "/google.ads.googleads.v3.services.CampaignExtensionSettingService/MutateCampaignExtensionSettings" ) ;
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for CampaignExtensionSettingServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for CampaignExtensionSettingServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "CampaignExtensionSettingServiceClient {{ ... }}")
        }
    }
}
/// Request message for [CampaignFeedService.GetCampaignFeed][google.ads.googleads.v3.services.CampaignFeedService.GetCampaignFeed].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCampaignFeedRequest {
    /// Required. The resource name of the campaign feed to fetch.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
/// Request message for [CampaignFeedService.MutateCampaignFeeds][google.ads.googleads.v3.services.CampaignFeedService.MutateCampaignFeeds].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateCampaignFeedsRequest {
    /// Required. The ID of the customer whose campaign feeds are being modified.
    #[prost(string, tag = "1")]
    pub customer_id: std::string::String,
    /// Required. The list of operations to perform on individual campaign feeds.
    #[prost(message, repeated, tag = "2")]
    pub operations: ::std::vec::Vec<CampaignFeedOperation>,
    /// If true, successful operations will be carried out and invalid
    /// operations will return errors. If false, all operations will be carried
    /// out in one transaction if and only if they are all valid.
    /// Default is false.
    #[prost(bool, tag = "3")]
    pub partial_failure: bool,
    /// If true, the request is validated but not executed. Only errors are
    /// returned, not results.
    #[prost(bool, tag = "4")]
    pub validate_only: bool,
}
/// A single operation (create, update, remove) on a campaign feed.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CampaignFeedOperation {
    /// FieldMask that determines which resource fields are modified in an update.
    #[prost(message, optional, tag = "4")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
    /// The mutate operation.
    #[prost(oneof = "campaign_feed_operation::Operation", tags = "1, 2, 3")]
    pub operation: ::std::option::Option<campaign_feed_operation::Operation>,
}
pub mod campaign_feed_operation {
    /// The mutate operation.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Operation {
        /// Create operation: No resource name is expected for the new campaign feed.
        #[prost(message, tag = "1")]
        Create(super::super::resources::CampaignFeed),
        /// Update operation: The campaign feed is expected to have a valid resource
        /// name.
        #[prost(message, tag = "2")]
        Update(super::super::resources::CampaignFeed),
        /// Remove operation: A resource name for the removed campaign feed is
        /// expected, in this format:
        ///
        /// `customers/{customer_id}/campaignFeeds/{campaign_id}~{feed_id}`
        #[prost(string, tag = "3")]
        Remove(std::string::String),
    }
}
/// Response message for a campaign feed mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateCampaignFeedsResponse {
    /// Errors that pertain to operation failures in the partial failure mode.
    /// Returned only when partial_failure = true and all errors occur inside the
    /// operations. If any errors occur outside the operations (e.g. auth errors),
    /// we return an RPC level error.
    #[prost(message, optional, tag = "3")]
    pub partial_failure_error: ::std::option::Option<super::super::super::super::rpc::Status>,
    /// All results for the mutate.
    #[prost(message, repeated, tag = "2")]
    pub results: ::std::vec::Vec<MutateCampaignFeedResult>,
}
/// The result for the campaign feed mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateCampaignFeedResult {
    /// Returned for successful operations.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
#[doc = r" Generated client implementations."]
pub mod campaign_feed_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Service to manage campaign feeds."]
    pub struct CampaignFeedServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> CampaignFeedServiceClient<T>
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
        #[doc = " Returns the requested campaign feed in full detail."]
        pub async fn get_campaign_feed(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCampaignFeedRequest>,
        ) -> Result<tonic::Response<super::super::resources::CampaignFeed>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ads.googleads.v3.services.CampaignFeedService/GetCampaignFeed",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates, updates, or removes campaign feeds. Operation statuses are"]
        #[doc = " returned."]
        pub async fn mutate_campaign_feeds(
            &mut self,
            request: impl tonic::IntoRequest<super::MutateCampaignFeedsRequest>,
        ) -> Result<tonic::Response<super::MutateCampaignFeedsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ads.googleads.v3.services.CampaignFeedService/MutateCampaignFeeds",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for CampaignFeedServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for CampaignFeedServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "CampaignFeedServiceClient {{ ... }}")
        }
    }
}
/// Request message for [CampaignLabelService.GetCampaignLabel][google.ads.googleads.v3.services.CampaignLabelService.GetCampaignLabel].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCampaignLabelRequest {
    /// Required. The resource name of the campaign-label relationship to fetch.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
/// Request message for [CampaignLabelService.MutateCampaignLabels][google.ads.googleads.v3.services.CampaignLabelService.MutateCampaignLabels].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateCampaignLabelsRequest {
    /// Required. ID of the customer whose campaign-label relationships are being modified.
    #[prost(string, tag = "1")]
    pub customer_id: std::string::String,
    /// Required. The list of operations to perform on campaign-label relationships.
    #[prost(message, repeated, tag = "2")]
    pub operations: ::std::vec::Vec<CampaignLabelOperation>,
    /// If true, successful operations will be carried out and invalid
    /// operations will return errors. If false, all operations will be carried
    /// out in one transaction if and only if they are all valid.
    /// Default is false.
    #[prost(bool, tag = "3")]
    pub partial_failure: bool,
    /// If true, the request is validated but not executed. Only errors are
    /// returned, not results.
    #[prost(bool, tag = "4")]
    pub validate_only: bool,
}
/// A single operation (create, remove) on a campaign-label relationship.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CampaignLabelOperation {
    /// The mutate operation.
    #[prost(oneof = "campaign_label_operation::Operation", tags = "1, 2")]
    pub operation: ::std::option::Option<campaign_label_operation::Operation>,
}
pub mod campaign_label_operation {
    /// The mutate operation.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Operation {
        /// Create operation: No resource name is expected for the new campaign-label
        /// relationship.
        #[prost(message, tag = "1")]
        Create(super::super::resources::CampaignLabel),
        /// Remove operation: A resource name for the campaign-label relationship
        /// being removed, in this format:
        ///
        /// `customers/{customer_id}/campaignLabels/{campaign_id}~{label_id}`
        #[prost(string, tag = "2")]
        Remove(std::string::String),
    }
}
/// Response message for a campaign labels mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateCampaignLabelsResponse {
    /// Errors that pertain to operation failures in the partial failure mode.
    /// Returned only when partial_failure = true and all errors occur inside the
    /// operations. If any errors occur outside the operations (e.g. auth errors),
    /// we return an RPC level error.
    #[prost(message, optional, tag = "3")]
    pub partial_failure_error: ::std::option::Option<super::super::super::super::rpc::Status>,
    /// All results for the mutate.
    #[prost(message, repeated, tag = "2")]
    pub results: ::std::vec::Vec<MutateCampaignLabelResult>,
}
/// The result for a campaign label mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateCampaignLabelResult {
    /// Returned for successful operations.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
#[doc = r" Generated client implementations."]
pub mod campaign_label_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Service to manage labels on campaigns."]
    pub struct CampaignLabelServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> CampaignLabelServiceClient<T>
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
        #[doc = " Returns the requested campaign-label relationship in full detail."]
        pub async fn get_campaign_label(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCampaignLabelRequest>,
        ) -> Result<tonic::Response<super::super::resources::CampaignLabel>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ads.googleads.v3.services.CampaignLabelService/GetCampaignLabel",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates and removes campaign-label relationships."]
        #[doc = " Operation statuses are returned."]
        pub async fn mutate_campaign_labels(
            &mut self,
            request: impl tonic::IntoRequest<super::MutateCampaignLabelsRequest>,
        ) -> Result<tonic::Response<super::MutateCampaignLabelsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ads.googleads.v3.services.CampaignLabelService/MutateCampaignLabels",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for CampaignLabelServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for CampaignLabelServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "CampaignLabelServiceClient {{ ... }}")
        }
    }
}
/// Request message for [CampaignService.GetCampaign][google.ads.googleads.v3.services.CampaignService.GetCampaign].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCampaignRequest {
    /// Required. The resource name of the campaign to fetch.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
/// Request message for [CampaignService.MutateCampaigns][google.ads.googleads.v3.services.CampaignService.MutateCampaigns].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateCampaignsRequest {
    /// Required. The ID of the customer whose campaigns are being modified.
    #[prost(string, tag = "1")]
    pub customer_id: std::string::String,
    /// Required. The list of operations to perform on individual campaigns.
    #[prost(message, repeated, tag = "2")]
    pub operations: ::std::vec::Vec<CampaignOperation>,
    /// If true, successful operations will be carried out and invalid
    /// operations will return errors. If false, all operations will be carried
    /// out in one transaction if and only if they are all valid.
    /// Default is false.
    #[prost(bool, tag = "3")]
    pub partial_failure: bool,
    /// If true, the request is validated but not executed. Only errors are
    /// returned, not results.
    #[prost(bool, tag = "4")]
    pub validate_only: bool,
}
/// A single operation (create, update, remove) on a campaign.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CampaignOperation {
    /// FieldMask that determines which resource fields are modified in an update.
    #[prost(message, optional, tag = "4")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
    /// The mutate operation.
    #[prost(oneof = "campaign_operation::Operation", tags = "1, 2, 3")]
    pub operation: ::std::option::Option<campaign_operation::Operation>,
}
pub mod campaign_operation {
    /// The mutate operation.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Operation {
        /// Create operation: No resource name is expected for the new campaign.
        #[prost(message, tag = "1")]
        Create(super::super::resources::Campaign),
        /// Update operation: The campaign is expected to have a valid
        /// resource name.
        #[prost(message, tag = "2")]
        Update(super::super::resources::Campaign),
        /// Remove operation: A resource name for the removed campaign is
        /// expected, in this format:
        ///
        /// `customers/{customer_id}/campaigns/{campaign_id}`
        #[prost(string, tag = "3")]
        Remove(std::string::String),
    }
}
/// Response message for campaign mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateCampaignsResponse {
    /// Errors that pertain to operation failures in the partial failure mode.
    /// Returned only when partial_failure = true and all errors occur inside the
    /// operations. If any errors occur outside the operations (e.g. auth errors),
    /// we return an RPC level error.
    #[prost(message, optional, tag = "3")]
    pub partial_failure_error: ::std::option::Option<super::super::super::super::rpc::Status>,
    /// All results for the mutate.
    #[prost(message, repeated, tag = "2")]
    pub results: ::std::vec::Vec<MutateCampaignResult>,
}
/// The result for the campaign mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateCampaignResult {
    /// Returned for successful operations.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
#[doc = r" Generated client implementations."]
pub mod campaign_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Service to manage campaigns."]
    pub struct CampaignServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> CampaignServiceClient<T>
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
        #[doc = " Returns the requested campaign in full detail."]
        pub async fn get_campaign(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCampaignRequest>,
        ) -> Result<tonic::Response<super::super::resources::Campaign>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ads.googleads.v3.services.CampaignService/GetCampaign",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates, updates, or removes campaigns. Operation statuses are returned."]
        pub async fn mutate_campaigns(
            &mut self,
            request: impl tonic::IntoRequest<super::MutateCampaignsRequest>,
        ) -> Result<tonic::Response<super::MutateCampaignsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ads.googleads.v3.services.CampaignService/MutateCampaigns",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for CampaignServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for CampaignServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "CampaignServiceClient {{ ... }}")
        }
    }
}
/// Request message for [CampaignSharedSetService.GetCampaignSharedSet][google.ads.googleads.v3.services.CampaignSharedSetService.GetCampaignSharedSet].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCampaignSharedSetRequest {
    /// Required. The resource name of the campaign shared set to fetch.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
/// Request message for [CampaignSharedSetService.MutateCampaignSharedSets][google.ads.googleads.v3.services.CampaignSharedSetService.MutateCampaignSharedSets].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateCampaignSharedSetsRequest {
    /// Required. The ID of the customer whose campaign shared sets are being modified.
    #[prost(string, tag = "1")]
    pub customer_id: std::string::String,
    /// Required. The list of operations to perform on individual campaign shared sets.
    #[prost(message, repeated, tag = "2")]
    pub operations: ::std::vec::Vec<CampaignSharedSetOperation>,
    /// If true, successful operations will be carried out and invalid
    /// operations will return errors. If false, all operations will be carried
    /// out in one transaction if and only if they are all valid.
    /// Default is false.
    #[prost(bool, tag = "3")]
    pub partial_failure: bool,
    /// If true, the request is validated but not executed. Only errors are
    /// returned, not results.
    #[prost(bool, tag = "4")]
    pub validate_only: bool,
}
/// A single operation (create, remove) on an campaign shared set.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CampaignSharedSetOperation {
    /// The mutate operation.
    #[prost(oneof = "campaign_shared_set_operation::Operation", tags = "1, 3")]
    pub operation: ::std::option::Option<campaign_shared_set_operation::Operation>,
}
pub mod campaign_shared_set_operation {
    /// The mutate operation.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Operation {
        /// Create operation: No resource name is expected for the new campaign
        /// shared set.
        #[prost(message, tag = "1")]
        Create(super::super::resources::CampaignSharedSet),
        /// Remove operation: A resource name for the removed campaign shared set is
        /// expected, in this format:
        ///
        /// `customers/{customer_id}/campaignSharedSets/{campaign_id}~{shared_set_id}`
        #[prost(string, tag = "3")]
        Remove(std::string::String),
    }
}
/// Response message for a campaign shared set mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateCampaignSharedSetsResponse {
    /// Errors that pertain to operation failures in the partial failure mode.
    /// Returned only when partial_failure = true and all errors occur inside the
    /// operations. If any errors occur outside the operations (e.g. auth errors),
    /// we return an RPC level error.
    #[prost(message, optional, tag = "3")]
    pub partial_failure_error: ::std::option::Option<super::super::super::super::rpc::Status>,
    /// All results for the mutate.
    #[prost(message, repeated, tag = "2")]
    pub results: ::std::vec::Vec<MutateCampaignSharedSetResult>,
}
/// The result for the campaign shared set mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateCampaignSharedSetResult {
    /// Returned for successful operations.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
#[doc = r" Generated client implementations."]
pub mod campaign_shared_set_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Service to manage campaign shared sets."]
    pub struct CampaignSharedSetServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> CampaignSharedSetServiceClient<T>
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
        #[doc = " Returns the requested campaign shared set in full detail."]
        pub async fn get_campaign_shared_set(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCampaignSharedSetRequest>,
        ) -> Result<tonic::Response<super::super::resources::CampaignSharedSet>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ads.googleads.v3.services.CampaignSharedSetService/GetCampaignSharedSet",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates or removes campaign shared sets. Operation statuses are returned."]
        pub async fn mutate_campaign_shared_sets(
            &mut self,
            request: impl tonic::IntoRequest<super::MutateCampaignSharedSetsRequest>,
        ) -> Result<tonic::Response<super::MutateCampaignSharedSetsResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ( "/google.ads.googleads.v3.services.CampaignSharedSetService/MutateCampaignSharedSets" ) ;
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for CampaignSharedSetServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for CampaignSharedSetServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "CampaignSharedSetServiceClient {{ ... }}")
        }
    }
}
/// Request message for [CarrierConstantService.GetCarrierConstant][google.ads.googleads.v3.services.CarrierConstantService.GetCarrierConstant].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCarrierConstantRequest {
    /// Required. Resource name of the carrier constant to fetch.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
#[doc = r" Generated client implementations."]
pub mod carrier_constant_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Service to fetch carrier constants."]
    pub struct CarrierConstantServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> CarrierConstantServiceClient<T>
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
        #[doc = " Returns the requested carrier constant in full detail."]
        pub async fn get_carrier_constant(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCarrierConstantRequest>,
        ) -> Result<tonic::Response<super::super::resources::CarrierConstant>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ads.googleads.v3.services.CarrierConstantService/GetCarrierConstant",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for CarrierConstantServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for CarrierConstantServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "CarrierConstantServiceClient {{ ... }}")
        }
    }
}
/// Request message for '[ChangeStatusService.GetChangeStatus][google.ads.googleads.v3.services.ChangeStatusService.GetChangeStatus]'.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetChangeStatusRequest {
    /// Required. The resource name of the change status to fetch.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
#[doc = r" Generated client implementations."]
pub mod change_status_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Service to fetch change statuses."]
    pub struct ChangeStatusServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> ChangeStatusServiceClient<T>
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
        #[doc = " Returns the requested change status in full detail."]
        pub async fn get_change_status(
            &mut self,
            request: impl tonic::IntoRequest<super::GetChangeStatusRequest>,
        ) -> Result<tonic::Response<super::super::resources::ChangeStatus>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ads.googleads.v3.services.ChangeStatusService/GetChangeStatus",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for ChangeStatusServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for ChangeStatusServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "ChangeStatusServiceClient {{ ... }}")
        }
    }
}
/// Request message for [ClickViewService.GetClickView][google.ads.googleads.v3.services.ClickViewService.GetClickView].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetClickViewRequest {
    /// Required. The resource name of the click view to fetch.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
#[doc = r" Generated client implementations."]
pub mod click_view_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Service to fetch click views."]
    pub struct ClickViewServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> ClickViewServiceClient<T>
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
        #[doc = " Returns the requested click view in full detail."]
        pub async fn get_click_view(
            &mut self,
            request: impl tonic::IntoRequest<super::GetClickViewRequest>,
        ) -> Result<tonic::Response<super::super::resources::ClickView>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ads.googleads.v3.services.ClickViewService/GetClickView",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for ClickViewServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for ClickViewServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "ClickViewServiceClient {{ ... }}")
        }
    }
}
/// Request message for [ConversionActionService.GetConversionAction][google.ads.googleads.v3.services.ConversionActionService.GetConversionAction].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetConversionActionRequest {
    /// Required. The resource name of the conversion action to fetch.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
/// Request message for [ConversionActionService.MutateConversionActions][google.ads.googleads.v3.services.ConversionActionService.MutateConversionActions].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateConversionActionsRequest {
    /// Required. The ID of the customer whose conversion actions are being modified.
    #[prost(string, tag = "1")]
    pub customer_id: std::string::String,
    /// Required. The list of operations to perform on individual conversion actions.
    #[prost(message, repeated, tag = "2")]
    pub operations: ::std::vec::Vec<ConversionActionOperation>,
    /// If true, successful operations will be carried out and invalid
    /// operations will return errors. If false, all operations will be carried
    /// out in one transaction if and only if they are all valid.
    /// Default is false.
    #[prost(bool, tag = "3")]
    pub partial_failure: bool,
    /// If true, the request is validated but not executed. Only errors are
    /// returned, not results.
    #[prost(bool, tag = "4")]
    pub validate_only: bool,
}
/// A single operation (create, update, remove) on a conversion action.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConversionActionOperation {
    /// FieldMask that determines which resource fields are modified in an update.
    #[prost(message, optional, tag = "4")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
    /// The mutate operation.
    #[prost(oneof = "conversion_action_operation::Operation", tags = "1, 2, 3")]
    pub operation: ::std::option::Option<conversion_action_operation::Operation>,
}
pub mod conversion_action_operation {
    /// The mutate operation.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Operation {
        /// Create operation: No resource name is expected for the new conversion
        /// action.
        #[prost(message, tag = "1")]
        Create(super::super::resources::ConversionAction),
        /// Update operation: The conversion action is expected to have a valid
        /// resource name.
        #[prost(message, tag = "2")]
        Update(super::super::resources::ConversionAction),
        /// Remove operation: A resource name for the removed conversion action is
        /// expected, in this format:
        ///
        /// `customers/{customer_id}/conversionActions/{conversion_action_id}`
        #[prost(string, tag = "3")]
        Remove(std::string::String),
    }
}
/// Response message for [ConversionActionService.MutateConversionActions][google.ads.googleads.v3.services.ConversionActionService.MutateConversionActions].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateConversionActionsResponse {
    /// Errors that pertain to operation failures in the partial failure mode.
    /// Returned only when partial_failure = true and all errors occur inside the
    /// operations. If any errors occur outside the operations (e.g. auth errors),
    /// we return an RPC level error.
    #[prost(message, optional, tag = "3")]
    pub partial_failure_error: ::std::option::Option<super::super::super::super::rpc::Status>,
    /// All results for the mutate.
    #[prost(message, repeated, tag = "2")]
    pub results: ::std::vec::Vec<MutateConversionActionResult>,
}
/// The result for the conversion action mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateConversionActionResult {
    /// Returned for successful operations.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
#[doc = r" Generated client implementations."]
pub mod conversion_action_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Service to manage conversion actions."]
    pub struct ConversionActionServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> ConversionActionServiceClient<T>
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
        #[doc = " Returns the requested conversion action."]
        pub async fn get_conversion_action(
            &mut self,
            request: impl tonic::IntoRequest<super::GetConversionActionRequest>,
        ) -> Result<tonic::Response<super::super::resources::ConversionAction>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ads.googleads.v3.services.ConversionActionService/GetConversionAction",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates, updates or removes conversion actions. Operation statuses are"]
        #[doc = " returned."]
        pub async fn mutate_conversion_actions(
            &mut self,
            request: impl tonic::IntoRequest<super::MutateConversionActionsRequest>,
        ) -> Result<tonic::Response<super::MutateConversionActionsResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ads.googleads.v3.services.ConversionActionService/MutateConversionActions",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for ConversionActionServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for ConversionActionServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "ConversionActionServiceClient {{ ... }}")
        }
    }
}
/// Request message for
/// [ConversionAdjustmentUploadService.UploadConversionAdjustments][google.ads.googleads.v3.services.ConversionAdjustmentUploadService.UploadConversionAdjustments].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UploadConversionAdjustmentsRequest {
    /// Required. The ID of the customer performing the upload.
    #[prost(string, tag = "1")]
    pub customer_id: std::string::String,
    /// Required. The conversion adjustments that are being uploaded.
    #[prost(message, repeated, tag = "2")]
    pub conversion_adjustments: ::std::vec::Vec<ConversionAdjustment>,
    /// Required. If true, successful operations will be carried out and invalid
    /// operations will return errors. If false, all operations will be carried out
    /// in one transaction if and only if they are all valid. This should always be
    /// set to true.
    /// See
    /// https://developers.google.com/google-ads/api/docs/best-practices/partial-failures
    /// for more information about partial failure.
    #[prost(bool, tag = "3")]
    pub partial_failure: bool,
    /// If true, the request is validated but not executed. Only errors are
    /// returned, not results.
    #[prost(bool, tag = "4")]
    pub validate_only: bool,
}
/// Response message for
/// [ConversionAdjustmentUploadService.UploadConversionAdjustments][google.ads.googleads.v3.services.ConversionAdjustmentUploadService.UploadConversionAdjustments].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UploadConversionAdjustmentsResponse {
    /// Errors that pertain to conversion adjustment failures in the partial
    /// failure mode. Returned when all errors occur inside the adjustments. If any
    /// errors occur outside the adjustments (e.g. auth errors), we return an RPC
    /// level error.
    /// See
    /// https://developers.google.com/google-ads/api/docs/best-practices/partial-failures
    /// for more information about partial failure.
    #[prost(message, optional, tag = "1")]
    pub partial_failure_error: ::std::option::Option<super::super::super::super::rpc::Status>,
    /// Returned for successfully processed conversion adjustments. Proto will be
    /// empty for rows that received an error. Results are not returned when
    /// validate_only is true.
    #[prost(message, repeated, tag = "2")]
    pub results: ::std::vec::Vec<ConversionAdjustmentResult>,
}
/// A conversion adjustment.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConversionAdjustment {
    /// Resource name of the conversion action associated with this conversion
    /// adjustment. Note: Although this resource name consists of a customer id and
    /// a conversion action id, validation will ignore the customer id and use the
    /// conversion action id as the sole identifier of the conversion action.
    #[prost(message, optional, tag = "3")]
    pub conversion_action: ::std::option::Option<::std::string::String>,
    /// The date time at which the adjustment occurred. Must be after the
    /// conversion_date_time. The timezone must be specified. The format is
    /// "yyyy-mm-dd hh:mm:ss+|-hh:mm", e.g. "2019-01-01 12:32:45-08:00".
    #[prost(message, optional, tag = "4")]
    pub adjustment_date_time: ::std::option::Option<::std::string::String>,
    /// The adjustment type.
    #[prost(
        enumeration = "super::enums::conversion_adjustment_type_enum::ConversionAdjustmentType",
        tag = "5"
    )]
    pub adjustment_type: i32,
    /// Information needed to restate the conversion's value.
    /// Required for restatements. Should not be supplied for retractions. An error
    /// will be returned if provided for a retraction.
    /// NOTE: If you want to upload a second restatement with a different adjusted
    /// value, it must have a new, more recent, adjustment occurrence time.
    /// Otherwise, it will be treated as a duplicate of the previous restatement
    /// and ignored.
    #[prost(message, optional, tag = "6")]
    pub restatement_value: ::std::option::Option<RestatementValue>,
    /// Identifies the conversion to be adjusted.
    #[prost(oneof = "conversion_adjustment::ConversionIdentifier", tags = "1, 2")]
    pub conversion_identifier: ::std::option::Option<conversion_adjustment::ConversionIdentifier>,
}
pub mod conversion_adjustment {
    /// Identifies the conversion to be adjusted.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ConversionIdentifier {
        /// Uniquely identifies a conversion that was reported without an order ID
        /// specified.
        #[prost(message, tag = "1")]
        GclidDateTimePair(super::GclidDateTimePair),
        /// The order ID of the conversion to be adjusted. If the conversion was
        /// reported with an order ID specified, that order ID must be used as the
        /// identifier here.
        #[prost(message, tag = "2")]
        OrderId(::std::string::String),
    }
}
/// Contains information needed to restate a conversion's value.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RestatementValue {
    /// The restated conversion value. This is the value of the conversion after
    /// restatement. For example, to change the value of a conversion from 100 to
    /// 70, an adjusted value of 70 should be reported.
    /// NOTE: If you want to upload a second restatement with a different adjusted
    /// value, it must have a new, more recent, adjustment occurrence time.
    /// Otherwise, it will be treated as a duplicate of the previous restatement
    /// and ignored.
    #[prost(message, optional, tag = "1")]
    pub adjusted_value: ::std::option::Option<f64>,
    /// The currency of the restated value. If not provided, then the default
    /// currency from the conversion action is used, and if that is not set then
    /// the account currency is used. This is the ISO 4217 3-character currency
    /// code e.g. USD or EUR.
    #[prost(message, optional, tag = "2")]
    pub currency_code: ::std::option::Option<::std::string::String>,
}
/// Uniquely identifies a conversion that was reported without an order ID
/// specified.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GclidDateTimePair {
    /// Google click ID (gclid) associated with the original conversion for this
    /// adjustment.
    #[prost(message, optional, tag = "1")]
    pub gclid: ::std::option::Option<::std::string::String>,
    /// The date time at which the original conversion for this adjustment
    /// occurred. The timezone must be specified. The format is "yyyy-mm-dd
    /// hh:mm:ss+|-hh:mm", e.g. "2019-01-01 12:32:45-08:00".
    #[prost(message, optional, tag = "2")]
    pub conversion_date_time: ::std::option::Option<::std::string::String>,
}
/// Information identifying a successfully processed ConversionAdjustment.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConversionAdjustmentResult {
    /// Resource name of the conversion action associated with this conversion
    /// adjustment.
    #[prost(message, optional, tag = "3")]
    pub conversion_action: ::std::option::Option<::std::string::String>,
    /// The date time at which the adjustment occurred. The format is
    /// "yyyy-mm-dd hh:mm:ss+|-hh:mm", e.g. "2019-01-01 12:32:45-08:00".
    #[prost(message, optional, tag = "4")]
    pub adjustment_date_time: ::std::option::Option<::std::string::String>,
    /// The adjustment type.
    #[prost(
        enumeration = "super::enums::conversion_adjustment_type_enum::ConversionAdjustmentType",
        tag = "5"
    )]
    pub adjustment_type: i32,
    /// Identifies the conversion that was adjusted.
    #[prost(
        oneof = "conversion_adjustment_result::ConversionIdentifier",
        tags = "1, 2"
    )]
    pub conversion_identifier:
        ::std::option::Option<conversion_adjustment_result::ConversionIdentifier>,
}
pub mod conversion_adjustment_result {
    /// Identifies the conversion that was adjusted.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ConversionIdentifier {
        /// Uniquely identifies a conversion that was reported without an order ID
        /// specified.
        #[prost(message, tag = "1")]
        GclidDateTimePair(super::GclidDateTimePair),
        /// The order ID of the conversion that was adjusted.
        #[prost(message, tag = "2")]
        OrderId(::std::string::String),
    }
}
#[doc = r" Generated client implementations."]
pub mod conversion_adjustment_upload_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Service to upload conversion adjustments."]
    pub struct ConversionAdjustmentUploadServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> ConversionAdjustmentUploadServiceClient<T>
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
        #[doc = " Processes the given conversion adjustments."]
        pub async fn upload_conversion_adjustments(
            &mut self,
            request: impl tonic::IntoRequest<super::UploadConversionAdjustmentsRequest>,
        ) -> Result<tonic::Response<super::UploadConversionAdjustmentsResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ( "/google.ads.googleads.v3.services.ConversionAdjustmentUploadService/UploadConversionAdjustments" ) ;
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for ConversionAdjustmentUploadServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for ConversionAdjustmentUploadServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "ConversionAdjustmentUploadServiceClient {{ ... }}")
        }
    }
}
/// Request message for [ConversionUploadService.UploadClickConversions][google.ads.googleads.v3.services.ConversionUploadService.UploadClickConversions].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UploadClickConversionsRequest {
    /// Required. The ID of the customer performing the upload.
    #[prost(string, tag = "1")]
    pub customer_id: std::string::String,
    /// Required. The conversions that are being uploaded.
    #[prost(message, repeated, tag = "2")]
    pub conversions: ::std::vec::Vec<ClickConversion>,
    /// Required. If true, successful operations will be carried out and invalid
    /// operations will return errors. If false, all operations will be carried
    /// out in one transaction if and only if they are all valid.
    /// This should always be set to true.
    /// See
    /// https://developers.google.com/google-ads/api/docs/best-practices/partial-failures
    /// for more information about partial failure.
    #[prost(bool, tag = "3")]
    pub partial_failure: bool,
    /// If true, the request is validated but not executed. Only errors are
    /// returned, not results.
    #[prost(bool, tag = "4")]
    pub validate_only: bool,
}
/// Response message for [ConversionUploadService.UploadClickConversions][google.ads.googleads.v3.services.ConversionUploadService.UploadClickConversions].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UploadClickConversionsResponse {
    /// Errors that pertain to conversion failures in the partial failure mode.
    /// Returned when all errors occur inside the conversions. If any errors occur
    /// outside the conversions (e.g. auth errors), we return an RPC level error.
    /// See
    /// https://developers.google.com/google-ads/api/docs/best-practices/partial-failures
    /// for more information about partial failure.
    #[prost(message, optional, tag = "1")]
    pub partial_failure_error: ::std::option::Option<super::super::super::super::rpc::Status>,
    /// Returned for successfully processed conversions. Proto will be empty for
    /// rows that received an error. Results are not returned when validate_only is
    /// true.
    #[prost(message, repeated, tag = "2")]
    pub results: ::std::vec::Vec<ClickConversionResult>,
}
/// Request message for [ConversionUploadService.UploadCallConversions][google.ads.googleads.v3.services.ConversionUploadService.UploadCallConversions].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UploadCallConversionsRequest {
    /// Required. The ID of the customer performing the upload.
    #[prost(string, tag = "1")]
    pub customer_id: std::string::String,
    /// Required. The conversions that are being uploaded.
    #[prost(message, repeated, tag = "2")]
    pub conversions: ::std::vec::Vec<CallConversion>,
    /// Required. If true, successful operations will be carried out and invalid
    /// operations will return errors. If false, all operations will be carried
    /// out in one transaction if and only if they are all valid.
    /// This should always be set to true.
    /// See
    /// https://developers.google.com/google-ads/api/docs/best-practices/partial-failures
    /// for more information about partial failure.
    #[prost(bool, tag = "3")]
    pub partial_failure: bool,
    /// If true, the request is validated but not executed. Only errors are
    /// returned, not results.
    #[prost(bool, tag = "4")]
    pub validate_only: bool,
}
/// Response message for [ConversionUploadService.UploadCallConversions][google.ads.googleads.v3.services.ConversionUploadService.UploadCallConversions].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UploadCallConversionsResponse {
    /// Errors that pertain to conversion failures in the partial failure mode.
    /// Returned when all errors occur inside the conversions. If any errors occur
    /// outside the conversions (e.g. auth errors), we return an RPC level error.
    /// See
    /// https://developers.google.com/google-ads/api/docs/best-practices/partial-failures
    /// for more information about partial failure.
    #[prost(message, optional, tag = "1")]
    pub partial_failure_error: ::std::option::Option<super::super::super::super::rpc::Status>,
    /// Returned for successfully processed conversions. Proto will be empty for
    /// rows that received an error. Results are not returned when validate_only is
    /// true.
    #[prost(message, repeated, tag = "2")]
    pub results: ::std::vec::Vec<CallConversionResult>,
}
/// A click conversion.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClickConversion {
    /// The Google click ID (gclid) associated with this conversion.
    #[prost(message, optional, tag = "1")]
    pub gclid: ::std::option::Option<::std::string::String>,
    /// Resource name of the conversion action associated with this conversion.
    /// Note: Although this resource name consists of a customer id and a
    /// conversion action id, validation will ignore the customer id and use the
    /// conversion action id as the sole identifier of the conversion action.
    #[prost(message, optional, tag = "2")]
    pub conversion_action: ::std::option::Option<::std::string::String>,
    /// The date time at which the conversion occurred. Must be after
    /// the click time. The timezone must be specified. The format is
    /// "yyyy-mm-dd hh:mm:ss+|-hh:mm", e.g. “2019-01-01 12:32:45-08:00”.
    #[prost(message, optional, tag = "3")]
    pub conversion_date_time: ::std::option::Option<::std::string::String>,
    /// The value of the conversion for the advertiser.
    #[prost(message, optional, tag = "4")]
    pub conversion_value: ::std::option::Option<f64>,
    /// Currency associated with the conversion value. This is the ISO 4217
    /// 3-character currency code. For example: USD, EUR.
    #[prost(message, optional, tag = "5")]
    pub currency_code: ::std::option::Option<::std::string::String>,
    /// The order ID associated with the conversion. An order id can only be used
    /// for one conversion per conversion action.
    #[prost(message, optional, tag = "6")]
    pub order_id: ::std::option::Option<::std::string::String>,
    /// Additional data about externally attributed conversions. This field
    /// is required for conversions with an externally attributed conversion
    /// action, but should not be set otherwise.
    #[prost(message, optional, tag = "7")]
    pub external_attribution_data: ::std::option::Option<ExternalAttributionData>,
}
/// A call conversion.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CallConversion {
    /// The caller id from which this call was placed. Caller id is expected to be
    /// in E.164 format with preceding '+' sign. e.g. "+16502531234".
    #[prost(message, optional, tag = "1")]
    pub caller_id: ::std::option::Option<::std::string::String>,
    /// The date time at which the call occurred. The timezone must be specified.
    /// The format is "yyyy-mm-dd hh:mm:ss+|-hh:mm",
    /// e.g. "2019-01-01 12:32:45-08:00".
    #[prost(message, optional, tag = "2")]
    pub call_start_date_time: ::std::option::Option<::std::string::String>,
    /// Resource name of the conversion action associated with this conversion.
    /// Note: Although this resource name consists of a customer id and a
    /// conversion action id, validation will ignore the customer id and use the
    /// conversion action id as the sole identifier of the conversion action.
    #[prost(message, optional, tag = "3")]
    pub conversion_action: ::std::option::Option<::std::string::String>,
    /// The date time at which the conversion occurred. Must be after the call
    /// time. The timezone must be specified. The format is
    /// "yyyy-mm-dd hh:mm:ss+|-hh:mm", e.g. "2019-01-01 12:32:45-08:00".
    #[prost(message, optional, tag = "4")]
    pub conversion_date_time: ::std::option::Option<::std::string::String>,
    /// The value of the conversion for the advertiser.
    #[prost(message, optional, tag = "5")]
    pub conversion_value: ::std::option::Option<f64>,
    /// Currency associated with the conversion value. This is the ISO 4217
    /// 3-character currency code. For example: USD, EUR.
    #[prost(message, optional, tag = "6")]
    pub currency_code: ::std::option::Option<::std::string::String>,
}
/// Contains additional information about externally attributed conversions.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExternalAttributionData {
    /// Represents the fraction of the conversion that is attributed to the
    /// Google Ads click.
    #[prost(message, optional, tag = "1")]
    pub external_attribution_credit: ::std::option::Option<f64>,
    /// Specifies the attribution model name.
    #[prost(message, optional, tag = "2")]
    pub external_attribution_model: ::std::option::Option<::std::string::String>,
}
/// Identifying information for a successfully processed ClickConversion.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClickConversionResult {
    /// The Google Click ID (gclid) associated with this conversion.
    #[prost(message, optional, tag = "1")]
    pub gclid: ::std::option::Option<::std::string::String>,
    /// Resource name of the conversion action associated with this conversion.
    #[prost(message, optional, tag = "2")]
    pub conversion_action: ::std::option::Option<::std::string::String>,
    /// The date time at which the conversion occurred. The format is
    /// "yyyy-mm-dd hh:mm:ss+|-hh:mm", e.g. “2019-01-01 12:32:45-08:00”.
    #[prost(message, optional, tag = "3")]
    pub conversion_date_time: ::std::option::Option<::std::string::String>,
}
/// Identifying information for a successfully processed CallConversionUpload.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CallConversionResult {
    /// The caller id from which this call was placed. Caller id is expected to be
    /// in E.164 format with preceding '+' sign.
    #[prost(message, optional, tag = "1")]
    pub caller_id: ::std::option::Option<::std::string::String>,
    /// The date time at which the call occurred. The format is
    /// "yyyy-mm-dd hh:mm:ss+|-hh:mm", e.g. "2019-01-01 12:32:45-08:00".
    #[prost(message, optional, tag = "2")]
    pub call_start_date_time: ::std::option::Option<::std::string::String>,
    /// Resource name of the conversion action associated with this conversion.
    #[prost(message, optional, tag = "3")]
    pub conversion_action: ::std::option::Option<::std::string::String>,
    /// The date time at which the conversion occurred. The format is
    /// "yyyy-mm-dd hh:mm:ss+|-hh:mm", e.g. "2019-01-01 12:32:45-08:00".
    #[prost(message, optional, tag = "4")]
    pub conversion_date_time: ::std::option::Option<::std::string::String>,
}
#[doc = r" Generated client implementations."]
pub mod conversion_upload_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Service to upload conversions."]
    pub struct ConversionUploadServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> ConversionUploadServiceClient<T>
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
        #[doc = " Processes the given click conversions."]
        pub async fn upload_click_conversions(
            &mut self,
            request: impl tonic::IntoRequest<super::UploadClickConversionsRequest>,
        ) -> Result<tonic::Response<super::UploadClickConversionsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ads.googleads.v3.services.ConversionUploadService/UploadClickConversions",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Processes the given call conversions."]
        pub async fn upload_call_conversions(
            &mut self,
            request: impl tonic::IntoRequest<super::UploadCallConversionsRequest>,
        ) -> Result<tonic::Response<super::UploadCallConversionsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ads.googleads.v3.services.ConversionUploadService/UploadCallConversions",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for ConversionUploadServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for ConversionUploadServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "ConversionUploadServiceClient {{ ... }}")
        }
    }
}
/// Request message for [CurrencyConstantService.GetCurrencyConstant][google.ads.googleads.v3.services.CurrencyConstantService.GetCurrencyConstant].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCurrencyConstantRequest {
    /// Required. Resource name of the currency constant to fetch.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
#[doc = r" Generated client implementations."]
pub mod currency_constant_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Service to fetch currency constants."]
    pub struct CurrencyConstantServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> CurrencyConstantServiceClient<T>
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
        #[doc = " Returns the requested currency constant."]
        pub async fn get_currency_constant(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCurrencyConstantRequest>,
        ) -> Result<tonic::Response<super::super::resources::CurrencyConstant>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ads.googleads.v3.services.CurrencyConstantService/GetCurrencyConstant",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for CurrencyConstantServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for CurrencyConstantServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "CurrencyConstantServiceClient {{ ... }}")
        }
    }
}
/// Request message for [CustomInterestService.GetCustomInterest][google.ads.googleads.v3.services.CustomInterestService.GetCustomInterest].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCustomInterestRequest {
    /// Required. The resource name of the custom interest to fetch.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
/// Request message for [CustomInterestService.MutateCustomInterests][google.ads.googleads.v3.services.CustomInterestService.MutateCustomInterests].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateCustomInterestsRequest {
    /// Required. The ID of the customer whose custom interests are being modified.
    #[prost(string, tag = "1")]
    pub customer_id: std::string::String,
    /// Required. The list of operations to perform on individual custom interests.
    #[prost(message, repeated, tag = "2")]
    pub operations: ::std::vec::Vec<CustomInterestOperation>,
    /// If true, the request is validated but not executed. Only errors are
    /// returned, not results.
    #[prost(bool, tag = "4")]
    pub validate_only: bool,
}
/// A single operation (create, update) on a custom interest.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomInterestOperation {
    /// FieldMask that determines which resource fields are modified in an update.
    #[prost(message, optional, tag = "4")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
    /// The mutate operation.
    #[prost(oneof = "custom_interest_operation::Operation", tags = "1, 2")]
    pub operation: ::std::option::Option<custom_interest_operation::Operation>,
}
pub mod custom_interest_operation {
    /// The mutate operation.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Operation {
        /// Create operation: No resource name is expected for the new custom
        /// interest.
        #[prost(message, tag = "1")]
        Create(super::super::resources::CustomInterest),
        /// Update operation: The custom interest is expected to have a valid
        /// resource name.
        #[prost(message, tag = "2")]
        Update(super::super::resources::CustomInterest),
    }
}
/// Response message for custom interest mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateCustomInterestsResponse {
    /// All results for the mutate.
    #[prost(message, repeated, tag = "2")]
    pub results: ::std::vec::Vec<MutateCustomInterestResult>,
}
/// The result for the custom interest mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateCustomInterestResult {
    /// Returned for successful operations.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
#[doc = r" Generated client implementations."]
pub mod custom_interest_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Service to manage custom interests."]
    pub struct CustomInterestServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> CustomInterestServiceClient<T>
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
        #[doc = " Returns the requested custom interest in full detail."]
        pub async fn get_custom_interest(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCustomInterestRequest>,
        ) -> Result<tonic::Response<super::super::resources::CustomInterest>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ads.googleads.v3.services.CustomInterestService/GetCustomInterest",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates or updates custom interests. Operation statuses are returned."]
        pub async fn mutate_custom_interests(
            &mut self,
            request: impl tonic::IntoRequest<super::MutateCustomInterestsRequest>,
        ) -> Result<tonic::Response<super::MutateCustomInterestsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ads.googleads.v3.services.CustomInterestService/MutateCustomInterests",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for CustomInterestServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for CustomInterestServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "CustomInterestServiceClient {{ ... }}")
        }
    }
}
/// Request message for [CustomerClientLinkService.GetCustomerClientLink][google.ads.googleads.v3.services.CustomerClientLinkService.GetCustomerClientLink].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCustomerClientLinkRequest {
    /// Required. The resource name of the customer client link to fetch.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
/// Request message for [CustomerClientLinkService.MutateCustomerClientLink][google.ads.googleads.v3.services.CustomerClientLinkService.MutateCustomerClientLink].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateCustomerClientLinkRequest {
    /// Required. The ID of the customer whose customer link are being modified.
    #[prost(string, tag = "1")]
    pub customer_id: std::string::String,
    /// Required. The operation to perform on the individual CustomerClientLink.
    #[prost(message, optional, tag = "2")]
    pub operation: ::std::option::Option<CustomerClientLinkOperation>,
}
/// A single operation (create, update) on a CustomerClientLink.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomerClientLinkOperation {
    /// FieldMask that determines which resource fields are modified in an update.
    #[prost(message, optional, tag = "4")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
    /// The mutate operation.
    #[prost(oneof = "customer_client_link_operation::Operation", tags = "1, 2")]
    pub operation: ::std::option::Option<customer_client_link_operation::Operation>,
}
pub mod customer_client_link_operation {
    /// The mutate operation.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Operation {
        /// Create operation: No resource name is expected for the new link.
        #[prost(message, tag = "1")]
        Create(super::super::resources::CustomerClientLink),
        /// Update operation: The link is expected to have a valid resource name.
        #[prost(message, tag = "2")]
        Update(super::super::resources::CustomerClientLink),
    }
}
/// Response message for a CustomerClientLink mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateCustomerClientLinkResponse {
    /// A result that identifies the resource affected by the mutate request.
    #[prost(message, optional, tag = "1")]
    pub result: ::std::option::Option<MutateCustomerClientLinkResult>,
}
/// The result for a single customer client link mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateCustomerClientLinkResult {
    /// Returned for successful operations.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
#[doc = r" Generated client implementations."]
pub mod customer_client_link_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Service to manage customer client links."]
    pub struct CustomerClientLinkServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> CustomerClientLinkServiceClient<T>
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
        #[doc = " Returns the requested CustomerClientLink in full detail."]
        pub async fn get_customer_client_link(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCustomerClientLinkRequest>,
        ) -> Result<tonic::Response<super::super::resources::CustomerClientLink>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ads.googleads.v3.services.CustomerClientLinkService/GetCustomerClientLink",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates or updates a customer client link. Operation statuses are returned."]
        pub async fn mutate_customer_client_link(
            &mut self,
            request: impl tonic::IntoRequest<super::MutateCustomerClientLinkRequest>,
        ) -> Result<tonic::Response<super::MutateCustomerClientLinkResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ( "/google.ads.googleads.v3.services.CustomerClientLinkService/MutateCustomerClientLink" ) ;
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for CustomerClientLinkServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for CustomerClientLinkServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "CustomerClientLinkServiceClient {{ ... }}")
        }
    }
}
/// Request message for [CustomerClientService.GetCustomerClient][google.ads.googleads.v3.services.CustomerClientService.GetCustomerClient].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCustomerClientRequest {
    /// Required. The resource name of the client to fetch.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
#[doc = r" Generated client implementations."]
pub mod customer_client_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Service to get clients in a customer's hierarchy."]
    pub struct CustomerClientServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> CustomerClientServiceClient<T>
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
        #[doc = " Returns the requested client in full detail."]
        pub async fn get_customer_client(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCustomerClientRequest>,
        ) -> Result<tonic::Response<super::super::resources::CustomerClient>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ads.googleads.v3.services.CustomerClientService/GetCustomerClient",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for CustomerClientServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for CustomerClientServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "CustomerClientServiceClient {{ ... }}")
        }
    }
}
/// Request message for
/// [CustomerExtensionSettingService.GetCustomerExtensionSetting][google.ads.googleads.v3.services.CustomerExtensionSettingService.GetCustomerExtensionSetting].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCustomerExtensionSettingRequest {
    /// Required. The resource name of the customer extension setting to fetch.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
/// Request message for
/// [CustomerExtensionSettingService.MutateCustomerExtensionSettings][google.ads.googleads.v3.services.CustomerExtensionSettingService.MutateCustomerExtensionSettings].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateCustomerExtensionSettingsRequest {
    /// Required. The ID of the customer whose customer extension settings are being
    /// modified.
    #[prost(string, tag = "1")]
    pub customer_id: std::string::String,
    /// Required. The list of operations to perform on individual customer extension
    /// settings.
    #[prost(message, repeated, tag = "2")]
    pub operations: ::std::vec::Vec<CustomerExtensionSettingOperation>,
    /// If true, successful operations will be carried out and invalid
    /// operations will return errors. If false, all operations will be carried
    /// out in one transaction if and only if they are all valid.
    /// Default is false.
    #[prost(bool, tag = "3")]
    pub partial_failure: bool,
    /// If true, the request is validated but not executed. Only errors are
    /// returned, not results.
    #[prost(bool, tag = "4")]
    pub validate_only: bool,
}
/// A single operation (create, update, remove) on a customer extension setting.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomerExtensionSettingOperation {
    /// FieldMask that determines which resource fields are modified in an update.
    #[prost(message, optional, tag = "4")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
    /// The mutate operation.
    #[prost(
        oneof = "customer_extension_setting_operation::Operation",
        tags = "1, 2, 3"
    )]
    pub operation: ::std::option::Option<customer_extension_setting_operation::Operation>,
}
pub mod customer_extension_setting_operation {
    /// The mutate operation.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Operation {
        /// Create operation: No resource name is expected for the new customer
        /// extension setting.
        #[prost(message, tag = "1")]
        Create(super::super::resources::CustomerExtensionSetting),
        /// Update operation: The customer extension setting is expected to have a
        /// valid resource name.
        #[prost(message, tag = "2")]
        Update(super::super::resources::CustomerExtensionSetting),
        /// Remove operation: A resource name for the removed customer extension
        /// setting is expected, in this format:
        ///
        /// `customers/{customer_id}/customerExtensionSettings/{extension_type}`
        #[prost(string, tag = "3")]
        Remove(std::string::String),
    }
}
/// Response message for a customer extension setting mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateCustomerExtensionSettingsResponse {
    /// Errors that pertain to operation failures in the partial failure mode.
    /// Returned only when partial_failure = true and all errors occur inside the
    /// operations. If any errors occur outside the operations (e.g. auth errors),
    /// we return an RPC level error.
    #[prost(message, optional, tag = "3")]
    pub partial_failure_error: ::std::option::Option<super::super::super::super::rpc::Status>,
    /// All results for the mutate.
    #[prost(message, repeated, tag = "2")]
    pub results: ::std::vec::Vec<MutateCustomerExtensionSettingResult>,
}
/// The result for the customer extension setting mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateCustomerExtensionSettingResult {
    /// Returned for successful operations.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
#[doc = r" Generated client implementations."]
pub mod customer_extension_setting_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Service to manage customer extension settings."]
    pub struct CustomerExtensionSettingServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> CustomerExtensionSettingServiceClient<T>
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
        #[doc = " Returns the requested customer extension setting in full detail."]
        pub async fn get_customer_extension_setting(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCustomerExtensionSettingRequest>,
        ) -> Result<tonic::Response<super::super::resources::CustomerExtensionSetting>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ( "/google.ads.googleads.v3.services.CustomerExtensionSettingService/GetCustomerExtensionSetting" ) ;
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates, updates, or removes customer extension settings. Operation"]
        #[doc = " statuses are returned."]
        pub async fn mutate_customer_extension_settings(
            &mut self,
            request: impl tonic::IntoRequest<super::MutateCustomerExtensionSettingsRequest>,
        ) -> Result<tonic::Response<super::MutateCustomerExtensionSettingsResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ( "/google.ads.googleads.v3.services.CustomerExtensionSettingService/MutateCustomerExtensionSettings" ) ;
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for CustomerExtensionSettingServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for CustomerExtensionSettingServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "CustomerExtensionSettingServiceClient {{ ... }}")
        }
    }
}
/// Request message for [CustomerFeedService.GetCustomerFeed][google.ads.googleads.v3.services.CustomerFeedService.GetCustomerFeed].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCustomerFeedRequest {
    /// Required. The resource name of the customer feed to fetch.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
/// Request message for [CustomerFeedService.MutateCustomerFeeds][google.ads.googleads.v3.services.CustomerFeedService.MutateCustomerFeeds].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateCustomerFeedsRequest {
    /// Required. The ID of the customer whose customer feeds are being modified.
    #[prost(string, tag = "1")]
    pub customer_id: std::string::String,
    /// Required. The list of operations to perform on individual customer feeds.
    #[prost(message, repeated, tag = "2")]
    pub operations: ::std::vec::Vec<CustomerFeedOperation>,
    /// If true, successful operations will be carried out and invalid
    /// operations will return errors. If false, all operations will be carried
    /// out in one transaction if and only if they are all valid.
    /// Default is false.
    #[prost(bool, tag = "3")]
    pub partial_failure: bool,
    /// If true, the request is validated but not executed. Only errors are
    /// returned, not results.
    #[prost(bool, tag = "4")]
    pub validate_only: bool,
}
/// A single operation (create, update, remove) on a customer feed.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomerFeedOperation {
    /// FieldMask that determines which resource fields are modified in an update.
    #[prost(message, optional, tag = "4")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
    /// The mutate operation.
    #[prost(oneof = "customer_feed_operation::Operation", tags = "1, 2, 3")]
    pub operation: ::std::option::Option<customer_feed_operation::Operation>,
}
pub mod customer_feed_operation {
    /// The mutate operation.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Operation {
        /// Create operation: No resource name is expected for the new customer feed.
        #[prost(message, tag = "1")]
        Create(super::super::resources::CustomerFeed),
        /// Update operation: The customer feed is expected to have a valid resource
        /// name.
        #[prost(message, tag = "2")]
        Update(super::super::resources::CustomerFeed),
        /// Remove operation: A resource name for the removed customer feed is
        /// expected, in this format:
        ///
        /// `customers/{customer_id}/customerFeeds/{feed_id}`
        #[prost(string, tag = "3")]
        Remove(std::string::String),
    }
}
/// Response message for a customer feed mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateCustomerFeedsResponse {
    /// Errors that pertain to operation failures in the partial failure mode.
    /// Returned only when partial_failure = true and all errors occur inside the
    /// operations. If any errors occur outside the operations (e.g. auth errors),
    /// we return an RPC level error.
    #[prost(message, optional, tag = "3")]
    pub partial_failure_error: ::std::option::Option<super::super::super::super::rpc::Status>,
    /// All results for the mutate.
    #[prost(message, repeated, tag = "2")]
    pub results: ::std::vec::Vec<MutateCustomerFeedResult>,
}
/// The result for the customer feed mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateCustomerFeedResult {
    /// Returned for successful operations.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
#[doc = r" Generated client implementations."]
pub mod customer_feed_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Service to manage customer feeds."]
    pub struct CustomerFeedServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> CustomerFeedServiceClient<T>
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
        #[doc = " Returns the requested customer feed in full detail."]
        pub async fn get_customer_feed(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCustomerFeedRequest>,
        ) -> Result<tonic::Response<super::super::resources::CustomerFeed>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ads.googleads.v3.services.CustomerFeedService/GetCustomerFeed",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates, updates, or removes customer feeds. Operation statuses are"]
        #[doc = " returned."]
        pub async fn mutate_customer_feeds(
            &mut self,
            request: impl tonic::IntoRequest<super::MutateCustomerFeedsRequest>,
        ) -> Result<tonic::Response<super::MutateCustomerFeedsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ads.googleads.v3.services.CustomerFeedService/MutateCustomerFeeds",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for CustomerFeedServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for CustomerFeedServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "CustomerFeedServiceClient {{ ... }}")
        }
    }
}
/// Request message for [CustomerLabelService.GetCustomerLabel][google.ads.googleads.v3.services.CustomerLabelService.GetCustomerLabel].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCustomerLabelRequest {
    /// Required. The resource name of the customer-label relationship to fetch.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
/// Request message for [CustomerLabelService.MutateCustomerLabels][google.ads.googleads.v3.services.CustomerLabelService.MutateCustomerLabels].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateCustomerLabelsRequest {
    /// Required. ID of the customer whose customer-label relationships are being modified.
    #[prost(string, tag = "1")]
    pub customer_id: std::string::String,
    /// Required. The list of operations to perform on customer-label relationships.
    #[prost(message, repeated, tag = "2")]
    pub operations: ::std::vec::Vec<CustomerLabelOperation>,
    /// If true, successful operations will be carried out and invalid
    /// operations will return errors. If false, all operations will be carried
    /// out in one transaction if and only if they are all valid.
    /// Default is false.
    #[prost(bool, tag = "3")]
    pub partial_failure: bool,
    /// If true, the request is validated but not executed. Only errors are
    /// returned, not results.
    #[prost(bool, tag = "4")]
    pub validate_only: bool,
}
/// A single operation (create, remove) on a customer-label relationship.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomerLabelOperation {
    /// The mutate operation.
    #[prost(oneof = "customer_label_operation::Operation", tags = "1, 2")]
    pub operation: ::std::option::Option<customer_label_operation::Operation>,
}
pub mod customer_label_operation {
    /// The mutate operation.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Operation {
        /// Create operation: No resource name is expected for the new customer-label
        /// relationship.
        #[prost(message, tag = "1")]
        Create(super::super::resources::CustomerLabel),
        /// Remove operation: A resource name for the customer-label relationship
        /// being removed, in this format:
        ///
        /// `customers/{customer_id}/customerLabels/{label_id}`
        #[prost(string, tag = "2")]
        Remove(std::string::String),
    }
}
/// Response message for a customer labels mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateCustomerLabelsResponse {
    /// Errors that pertain to operation failures in the partial failure mode.
    /// Returned only when partial_failure = true and all errors occur inside the
    /// operations. If any errors occur outside the operations (e.g. auth errors),
    /// we return an RPC level error.
    #[prost(message, optional, tag = "3")]
    pub partial_failure_error: ::std::option::Option<super::super::super::super::rpc::Status>,
    /// All results for the mutate.
    #[prost(message, repeated, tag = "2")]
    pub results: ::std::vec::Vec<MutateCustomerLabelResult>,
}
/// The result for a customer label mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateCustomerLabelResult {
    /// Returned for successful operations.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
#[doc = r" Generated client implementations."]
pub mod customer_label_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Service to manage labels on customers."]
    pub struct CustomerLabelServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> CustomerLabelServiceClient<T>
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
        #[doc = " Returns the requested customer-label relationship in full detail."]
        pub async fn get_customer_label(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCustomerLabelRequest>,
        ) -> Result<tonic::Response<super::super::resources::CustomerLabel>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ads.googleads.v3.services.CustomerLabelService/GetCustomerLabel",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates and removes customer-label relationships."]
        #[doc = " Operation statuses are returned."]
        pub async fn mutate_customer_labels(
            &mut self,
            request: impl tonic::IntoRequest<super::MutateCustomerLabelsRequest>,
        ) -> Result<tonic::Response<super::MutateCustomerLabelsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ads.googleads.v3.services.CustomerLabelService/MutateCustomerLabels",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for CustomerLabelServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for CustomerLabelServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "CustomerLabelServiceClient {{ ... }}")
        }
    }
}
/// Request message for [CustomerManagerLinkService.GetCustomerManagerLink][google.ads.googleads.v3.services.CustomerManagerLinkService.GetCustomerManagerLink].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCustomerManagerLinkRequest {
    /// Required. The resource name of the CustomerManagerLink to fetch.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
/// Request message for [CustomerManagerLinkService.MutateCustomerManagerLink][google.ads.googleads.v3.services.CustomerManagerLinkService.MutateCustomerManagerLink].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateCustomerManagerLinkRequest {
    /// Required. The ID of the customer whose customer manager links are being modified.
    #[prost(string, tag = "1")]
    pub customer_id: std::string::String,
    /// Required. The list of operations to perform on individual customer manager links.
    #[prost(message, repeated, tag = "2")]
    pub operations: ::std::vec::Vec<CustomerManagerLinkOperation>,
}
/// Request message for [CustomerManagerLinkService.MoveManagerLink][google.ads.googleads.v3.services.CustomerManagerLinkService.MoveManagerLink].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MoveManagerLinkRequest {
    /// Required. The ID of the client customer that is being moved.
    #[prost(string, tag = "1")]
    pub customer_id: std::string::String,
    /// Required. The resource name of the previous CustomerManagerLink.
    /// The resource name has the form:
    /// `customers/{customer_id}/customerManagerLinks/{manager_customer_id}~{manager_link_id}`
    #[prost(string, tag = "2")]
    pub previous_customer_manager_link: std::string::String,
    /// Required. The resource name of the new manager customer that the client wants to move
    /// to. Customer resource names have the format: "customers/{customer_id}"
    #[prost(string, tag = "3")]
    pub new_manager: std::string::String,
}
/// Updates the status of a CustomerManagerLink.
/// The following actions are possible:
/// 1. Update operation with status ACTIVE accepts a pending invitation.
/// 2. Update operation with status REFUSED declines a pending invitation.
/// 3. Update operation with status INACTIVE terminates link to manager.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomerManagerLinkOperation {
    /// FieldMask that determines which resource fields are modified in an update.
    #[prost(message, optional, tag = "4")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
    /// The mutate operation.
    #[prost(oneof = "customer_manager_link_operation::Operation", tags = "2")]
    pub operation: ::std::option::Option<customer_manager_link_operation::Operation>,
}
pub mod customer_manager_link_operation {
    /// The mutate operation.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Operation {
        /// Update operation: The link is expected to have a valid resource name.
        #[prost(message, tag = "2")]
        Update(super::super::resources::CustomerManagerLink),
    }
}
/// Response message for a CustomerManagerLink mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateCustomerManagerLinkResponse {
    /// A result that identifies the resource affected by the mutate request.
    #[prost(message, repeated, tag = "1")]
    pub results: ::std::vec::Vec<MutateCustomerManagerLinkResult>,
}
/// Response message for a CustomerManagerLink moveManagerLink.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MoveManagerLinkResponse {
    /// Returned for successful operations. Represents a CustomerManagerLink
    /// resource of the newly created link between client customer and new manager
    /// customer.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
/// The result for the customer manager link mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateCustomerManagerLinkResult {
    /// Returned for successful operations.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
#[doc = r" Generated client implementations."]
pub mod customer_manager_link_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Service to manage customer-manager links."]
    pub struct CustomerManagerLinkServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> CustomerManagerLinkServiceClient<T>
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
        #[doc = " Returns the requested CustomerManagerLink in full detail."]
        pub async fn get_customer_manager_link(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCustomerManagerLinkRequest>,
        ) -> Result<tonic::Response<super::super::resources::CustomerManagerLink>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ( "/google.ads.googleads.v3.services.CustomerManagerLinkService/GetCustomerManagerLink" ) ;
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates or updates customer manager links. Operation statuses are returned."]
        pub async fn mutate_customer_manager_link(
            &mut self,
            request: impl tonic::IntoRequest<super::MutateCustomerManagerLinkRequest>,
        ) -> Result<tonic::Response<super::MutateCustomerManagerLinkResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ( "/google.ads.googleads.v3.services.CustomerManagerLinkService/MutateCustomerManagerLink" ) ;
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Moves a client customer to a new manager customer."]
        #[doc = " This simplifies the complex request that requires two operations to move"]
        #[doc = " a client customer to a new manager. i.e:"]
        #[doc = " 1. Update operation with Status INACTIVE (previous manager) and,"]
        #[doc = " 2. Update operation with Status ACTIVE (new manager)."]
        pub async fn move_manager_link(
            &mut self,
            request: impl tonic::IntoRequest<super::MoveManagerLinkRequest>,
        ) -> Result<tonic::Response<super::MoveManagerLinkResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ads.googleads.v3.services.CustomerManagerLinkService/MoveManagerLink",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for CustomerManagerLinkServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for CustomerManagerLinkServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "CustomerManagerLinkServiceClient {{ ... }}")
        }
    }
}
/// Request message for
/// [CustomerNegativeCriterionService.GetCustomerNegativeCriterion][google.ads.googleads.v3.services.CustomerNegativeCriterionService.GetCustomerNegativeCriterion].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCustomerNegativeCriterionRequest {
    /// Required. The resource name of the criterion to fetch.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
/// Request message for
/// [CustomerNegativeCriterionService.MutateCustomerNegativeCriteria][google.ads.googleads.v3.services.CustomerNegativeCriterionService.MutateCustomerNegativeCriteria].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateCustomerNegativeCriteriaRequest {
    /// Required. The ID of the customer whose criteria are being modified.
    #[prost(string, tag = "1")]
    pub customer_id: std::string::String,
    /// Required. The list of operations to perform on individual criteria.
    #[prost(message, repeated, tag = "2")]
    pub operations: ::std::vec::Vec<CustomerNegativeCriterionOperation>,
    /// If true, successful operations will be carried out and invalid
    /// operations will return errors. If false, all operations will be carried
    /// out in one transaction if and only if they are all valid.
    /// Default is false.
    #[prost(bool, tag = "3")]
    pub partial_failure: bool,
    /// If true, the request is validated but not executed. Only errors are
    /// returned, not results.
    #[prost(bool, tag = "4")]
    pub validate_only: bool,
}
/// A single operation (create or remove) on a customer level negative criterion.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomerNegativeCriterionOperation {
    /// The mutate operation.
    #[prost(
        oneof = "customer_negative_criterion_operation::Operation",
        tags = "1, 2"
    )]
    pub operation: ::std::option::Option<customer_negative_criterion_operation::Operation>,
}
pub mod customer_negative_criterion_operation {
    /// The mutate operation.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Operation {
        /// Create operation: No resource name is expected for the new criterion.
        #[prost(message, tag = "1")]
        Create(super::super::resources::CustomerNegativeCriterion),
        /// Remove operation: A resource name for the removed criterion is expected,
        /// in this format:
        ///
        /// `customers/{customer_id}/customerNegativeCriteria/{criterion_id}`
        #[prost(string, tag = "2")]
        Remove(std::string::String),
    }
}
/// Response message for customer negative criterion mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateCustomerNegativeCriteriaResponse {
    /// Errors that pertain to operation failures in the partial failure mode.
    /// Returned only when partial_failure = true and all errors occur inside the
    /// operations. If any errors occur outside the operations (e.g. auth errors),
    /// we return an RPC level error.
    #[prost(message, optional, tag = "3")]
    pub partial_failure_error: ::std::option::Option<super::super::super::super::rpc::Status>,
    /// All results for the mutate.
    #[prost(message, repeated, tag = "2")]
    pub results: ::std::vec::Vec<MutateCustomerNegativeCriteriaResult>,
}
/// The result for the criterion mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateCustomerNegativeCriteriaResult {
    /// Returned for successful operations.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
#[doc = r" Generated client implementations."]
pub mod customer_negative_criterion_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Service to manage customer negative criteria."]
    pub struct CustomerNegativeCriterionServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> CustomerNegativeCriterionServiceClient<T>
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
        #[doc = " Returns the requested criterion in full detail."]
        pub async fn get_customer_negative_criterion(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCustomerNegativeCriterionRequest>,
        ) -> Result<
            tonic::Response<super::super::resources::CustomerNegativeCriterion>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ( "/google.ads.googleads.v3.services.CustomerNegativeCriterionService/GetCustomerNegativeCriterion" ) ;
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates or removes criteria. Operation statuses are returned."]
        pub async fn mutate_customer_negative_criteria(
            &mut self,
            request: impl tonic::IntoRequest<super::MutateCustomerNegativeCriteriaRequest>,
        ) -> Result<tonic::Response<super::MutateCustomerNegativeCriteriaResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ( "/google.ads.googleads.v3.services.CustomerNegativeCriterionService/MutateCustomerNegativeCriteria" ) ;
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for CustomerNegativeCriterionServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for CustomerNegativeCriterionServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "CustomerNegativeCriterionServiceClient {{ ... }}")
        }
    }
}
/// Request message for [CustomerService.GetCustomer][google.ads.googleads.v3.services.CustomerService.GetCustomer].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCustomerRequest {
    /// Required. The resource name of the customer to fetch.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
/// Request message for [CustomerService.MutateCustomer][google.ads.googleads.v3.services.CustomerService.MutateCustomer].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateCustomerRequest {
    /// Required. The ID of the customer being modified.
    #[prost(string, tag = "1")]
    pub customer_id: std::string::String,
    /// Required. The operation to perform on the customer
    #[prost(message, optional, tag = "4")]
    pub operation: ::std::option::Option<CustomerOperation>,
    /// If true, the request is validated but not executed. Only errors are
    /// returned, not results.
    #[prost(bool, tag = "5")]
    pub validate_only: bool,
}
/// Request message for [CustomerService.CreateCustomerClient][google.ads.googleads.v3.services.CustomerService.CreateCustomerClient].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateCustomerClientRequest {
    /// Required. The ID of the Manager under whom client customer is being created.
    #[prost(string, tag = "1")]
    pub customer_id: std::string::String,
    /// Required. The new client customer to create. The resource name on this customer
    /// will be ignored.
    #[prost(message, optional, tag = "2")]
    pub customer_client: ::std::option::Option<super::resources::Customer>,
    /// Email address of the user who should be invited on the created client
    /// customer. Accessible only to customers on the allow-list.
    #[prost(message, optional, tag = "3")]
    pub email_address: ::std::option::Option<::std::string::String>,
    /// The proposed role of user on the created client customer.
    /// Accessible only to customers on the allow-list.
    #[prost(enumeration = "super::enums::access_role_enum::AccessRole", tag = "4")]
    pub access_role: i32,
}
/// A single update on a customer.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CustomerOperation {
    /// Mutate operation. Only updates are supported for customer.
    #[prost(message, optional, tag = "1")]
    pub update: ::std::option::Option<super::resources::Customer>,
    /// FieldMask that determines which resource fields are modified in an update.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
}
/// Response message for CreateCustomerClient mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateCustomerClientResponse {
    /// The resource name of the newly created customer client.
    #[prost(string, tag = "2")]
    pub resource_name: std::string::String,
}
/// Response message for customer mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateCustomerResponse {
    /// Result for the mutate.
    #[prost(message, optional, tag = "2")]
    pub result: ::std::option::Option<MutateCustomerResult>,
}
/// The result for the customer mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateCustomerResult {
    /// Returned for successful operations.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
/// Request message for [CustomerService.ListAccessibleCustomers][google.ads.googleads.v3.services.CustomerService.ListAccessibleCustomers].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAccessibleCustomersRequest {}
/// Response message for [CustomerService.ListAccessibleCustomers][google.ads.googleads.v3.services.CustomerService.ListAccessibleCustomers].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAccessibleCustomersResponse {
    /// Resource name of customers directly accessible by the
    /// user authenticating the call.
    #[prost(string, repeated, tag = "1")]
    pub resource_names: ::std::vec::Vec<std::string::String>,
}
#[doc = r" Generated client implementations."]
pub mod customer_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Service to manage customers."]
    pub struct CustomerServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> CustomerServiceClient<T>
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
        #[doc = " Returns the requested customer in full detail."]
        pub async fn get_customer(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCustomerRequest>,
        ) -> Result<tonic::Response<super::super::resources::Customer>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ads.googleads.v3.services.CustomerService/GetCustomer",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates a customer. Operation statuses are returned."]
        pub async fn mutate_customer(
            &mut self,
            request: impl tonic::IntoRequest<super::MutateCustomerRequest>,
        ) -> Result<tonic::Response<super::MutateCustomerResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ads.googleads.v3.services.CustomerService/MutateCustomer",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns resource names of customers directly accessible by the"]
        #[doc = " user authenticating the call."]
        pub async fn list_accessible_customers(
            &mut self,
            request: impl tonic::IntoRequest<super::ListAccessibleCustomersRequest>,
        ) -> Result<tonic::Response<super::ListAccessibleCustomersResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ads.googleads.v3.services.CustomerService/ListAccessibleCustomers",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a new client under manager. The new client customer is returned."]
        pub async fn create_customer_client(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateCustomerClientRequest>,
        ) -> Result<tonic::Response<super::CreateCustomerClientResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ads.googleads.v3.services.CustomerService/CreateCustomerClient",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for CustomerServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for CustomerServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "CustomerServiceClient {{ ... }}")
        }
    }
}
/// Request message for [DetailPlacementViewService.GetDetailPlacementView][google.ads.googleads.v3.services.DetailPlacementViewService.GetDetailPlacementView].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDetailPlacementViewRequest {
    /// Required. The resource name of the Detail Placement view to fetch.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
#[doc = r" Generated client implementations."]
pub mod detail_placement_view_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Service to fetch Detail Placement views."]
    pub struct DetailPlacementViewServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> DetailPlacementViewServiceClient<T>
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
        #[doc = " Returns the requested Detail Placement view in full detail."]
        pub async fn get_detail_placement_view(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDetailPlacementViewRequest>,
        ) -> Result<tonic::Response<super::super::resources::DetailPlacementView>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ( "/google.ads.googleads.v3.services.DetailPlacementViewService/GetDetailPlacementView" ) ;
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for DetailPlacementViewServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for DetailPlacementViewServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "DetailPlacementViewServiceClient {{ ... }}")
        }
    }
}
/// Request message for [DisplayKeywordViewService.GetDisplayKeywordView][google.ads.googleads.v3.services.DisplayKeywordViewService.GetDisplayKeywordView].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDisplayKeywordViewRequest {
    /// Required. The resource name of the display keyword view to fetch.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
#[doc = r" Generated client implementations."]
pub mod display_keyword_view_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Service to manage display keyword views."]
    pub struct DisplayKeywordViewServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> DisplayKeywordViewServiceClient<T>
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
        #[doc = " Returns the requested display keyword view in full detail."]
        pub async fn get_display_keyword_view(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDisplayKeywordViewRequest>,
        ) -> Result<tonic::Response<super::super::resources::DisplayKeywordView>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ads.googleads.v3.services.DisplayKeywordViewService/GetDisplayKeywordView",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for DisplayKeywordViewServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for DisplayKeywordViewServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "DisplayKeywordViewServiceClient {{ ... }}")
        }
    }
}
/// Request message for [DistanceViewService.GetDistanceView][google.ads.googleads.v3.services.DistanceViewService.GetDistanceView].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDistanceViewRequest {
    /// Required. The resource name of the distance view to fetch.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
#[doc = r" Generated client implementations."]
pub mod distance_view_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Service to fetch distance views."]
    pub struct DistanceViewServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> DistanceViewServiceClient<T>
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
        #[doc = " Returns the attributes of the requested distance view."]
        pub async fn get_distance_view(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDistanceViewRequest>,
        ) -> Result<tonic::Response<super::super::resources::DistanceView>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ads.googleads.v3.services.DistanceViewService/GetDistanceView",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for DistanceViewServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for DistanceViewServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "DistanceViewServiceClient {{ ... }}")
        }
    }
}
/// Request message for
/// [DomainCategoryService.GetDomainCategory][google.ads.googleads.v3.services.DomainCategoryService.GetDomainCategory].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDomainCategoryRequest {
    /// Required. Resource name of the domain category to fetch.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
#[doc = r" Generated client implementations."]
pub mod domain_category_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Service to fetch domain categories."]
    pub struct DomainCategoryServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> DomainCategoryServiceClient<T>
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
        #[doc = " Returns the requested domain category."]
        pub async fn get_domain_category(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDomainCategoryRequest>,
        ) -> Result<tonic::Response<super::super::resources::DomainCategory>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ads.googleads.v3.services.DomainCategoryService/GetDomainCategory",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for DomainCategoryServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for DomainCategoryServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "DomainCategoryServiceClient {{ ... }}")
        }
    }
}
/// Request message for
/// [DynamicSearchAdsSearchTermViewService.GetDynamicSearchAdsSearchTermView][google.ads.googleads.v3.services.DynamicSearchAdsSearchTermViewService.GetDynamicSearchAdsSearchTermView].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDynamicSearchAdsSearchTermViewRequest {
    /// Required. The resource name of the dynamic search ads search term view to fetch.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
#[doc = r" Generated client implementations."]
pub mod dynamic_search_ads_search_term_view_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Service to fetch dynamic search ads views."]
    pub struct DynamicSearchAdsSearchTermViewServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> DynamicSearchAdsSearchTermViewServiceClient<T>
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
        #[doc = " Returns the requested dynamic search ads search term view in full detail."]
        pub async fn get_dynamic_search_ads_search_term_view(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDynamicSearchAdsSearchTermViewRequest>,
        ) -> Result<
            tonic::Response<super::super::resources::DynamicSearchAdsSearchTermView>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ( "/google.ads.googleads.v3.services.DynamicSearchAdsSearchTermViewService/GetDynamicSearchAdsSearchTermView" ) ;
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for DynamicSearchAdsSearchTermViewServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for DynamicSearchAdsSearchTermViewServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "DynamicSearchAdsSearchTermViewServiceClient {{ ... }}")
        }
    }
}
/// Request message for
/// [ExpandedLandingPageViewService.GetExpandedLandingPageView][google.ads.googleads.v3.services.ExpandedLandingPageViewService.GetExpandedLandingPageView].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetExpandedLandingPageViewRequest {
    /// Required. The resource name of the expanded landing page view to fetch.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
#[doc = r" Generated client implementations."]
pub mod expanded_landing_page_view_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Service to fetch expanded landing page views."]
    pub struct ExpandedLandingPageViewServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> ExpandedLandingPageViewServiceClient<T>
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
        #[doc = " Returns the requested expanded landing page view in full detail."]
        pub async fn get_expanded_landing_page_view(
            &mut self,
            request: impl tonic::IntoRequest<super::GetExpandedLandingPageViewRequest>,
        ) -> Result<tonic::Response<super::super::resources::ExpandedLandingPageView>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ( "/google.ads.googleads.v3.services.ExpandedLandingPageViewService/GetExpandedLandingPageView" ) ;
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for ExpandedLandingPageViewServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for ExpandedLandingPageViewServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "ExpandedLandingPageViewServiceClient {{ ... }}")
        }
    }
}
/// Request message for [ExtensionFeedItemService.GetExtensionFeedItem][google.ads.googleads.v3.services.ExtensionFeedItemService.GetExtensionFeedItem].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetExtensionFeedItemRequest {
    /// Required. The resource name of the extension feed item to fetch.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
/// Request message for [ExtensionFeedItemService.MutateExtensionFeedItems][google.ads.googleads.v3.services.ExtensionFeedItemService.MutateExtensionFeedItems].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateExtensionFeedItemsRequest {
    /// Required. The ID of the customer whose extension feed items are being
    /// modified.
    #[prost(string, tag = "1")]
    pub customer_id: std::string::String,
    /// Required. The list of operations to perform on individual extension feed items.
    #[prost(message, repeated, tag = "2")]
    pub operations: ::std::vec::Vec<ExtensionFeedItemOperation>,
    /// If true, successful operations will be carried out and invalid
    /// operations will return errors. If false, all operations will be carried
    /// out in one transaction if and only if they are all valid.
    /// Default is false.
    #[prost(bool, tag = "3")]
    pub partial_failure: bool,
    /// If true, the request is validated but not executed. Only errors are
    /// returned, not results.
    #[prost(bool, tag = "4")]
    pub validate_only: bool,
}
/// A single operation (create, update, remove) on an extension feed item.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExtensionFeedItemOperation {
    /// FieldMask that determines which resource fields are modified in an update.
    #[prost(message, optional, tag = "4")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
    /// The mutate operation.
    #[prost(oneof = "extension_feed_item_operation::Operation", tags = "1, 2, 3")]
    pub operation: ::std::option::Option<extension_feed_item_operation::Operation>,
}
pub mod extension_feed_item_operation {
    /// The mutate operation.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Operation {
        /// Create operation: No resource name is expected for the new extension
        /// feed item.
        #[prost(message, tag = "1")]
        Create(super::super::resources::ExtensionFeedItem),
        /// Update operation: The extension feed item is expected to have a
        /// valid resource name.
        #[prost(message, tag = "2")]
        Update(super::super::resources::ExtensionFeedItem),
        /// Remove operation: A resource name for the removed extension feed item
        /// is expected, in this format:
        ///
        /// `customers/{customer_id}/extensionFeedItems/{feed_item_id}`
        #[prost(string, tag = "3")]
        Remove(std::string::String),
    }
}
/// Response message for an extension feed item mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateExtensionFeedItemsResponse {
    /// Errors that pertain to operation failures in the partial failure mode.
    /// Returned only when partial_failure = true and all errors occur inside the
    /// operations. If any errors occur outside the operations (e.g. auth errors),
    /// we return an RPC level error.
    #[prost(message, optional, tag = "3")]
    pub partial_failure_error: ::std::option::Option<super::super::super::super::rpc::Status>,
    /// All results for the mutate.
    #[prost(message, repeated, tag = "2")]
    pub results: ::std::vec::Vec<MutateExtensionFeedItemResult>,
}
/// The result for the extension feed item mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateExtensionFeedItemResult {
    /// Returned for successful operations.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
#[doc = r" Generated client implementations."]
pub mod extension_feed_item_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Service to manage extension feed items."]
    pub struct ExtensionFeedItemServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> ExtensionFeedItemServiceClient<T>
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
        #[doc = " Returns the requested extension feed item in full detail."]
        pub async fn get_extension_feed_item(
            &mut self,
            request: impl tonic::IntoRequest<super::GetExtensionFeedItemRequest>,
        ) -> Result<tonic::Response<super::super::resources::ExtensionFeedItem>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ads.googleads.v3.services.ExtensionFeedItemService/GetExtensionFeedItem",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates, updates, or removes extension feed items. Operation"]
        #[doc = " statuses are returned."]
        pub async fn mutate_extension_feed_items(
            &mut self,
            request: impl tonic::IntoRequest<super::MutateExtensionFeedItemsRequest>,
        ) -> Result<tonic::Response<super::MutateExtensionFeedItemsResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ( "/google.ads.googleads.v3.services.ExtensionFeedItemService/MutateExtensionFeedItems" ) ;
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for ExtensionFeedItemServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for ExtensionFeedItemServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "ExtensionFeedItemServiceClient {{ ... }}")
        }
    }
}
/// Request message for [FeedItemService.GetFeedItem][google.ads.googleads.v3.services.FeedItemService.GetFeedItem].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetFeedItemRequest {
    /// Required. The resource name of the feed item to fetch.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
/// Request message for [FeedItemService.MutateFeedItems][google.ads.googleads.v3.services.FeedItemService.MutateFeedItems].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateFeedItemsRequest {
    /// Required. The ID of the customer whose feed items are being modified.
    #[prost(string, tag = "1")]
    pub customer_id: std::string::String,
    /// Required. The list of operations to perform on individual feed items.
    #[prost(message, repeated, tag = "2")]
    pub operations: ::std::vec::Vec<FeedItemOperation>,
    /// If true, successful operations will be carried out and invalid
    /// operations will return errors. If false, all operations will be carried
    /// out in one transaction if and only if they are all valid.
    /// Default is false.
    #[prost(bool, tag = "3")]
    pub partial_failure: bool,
    /// If true, the request is validated but not executed. Only errors are
    /// returned, not results.
    #[prost(bool, tag = "4")]
    pub validate_only: bool,
}
/// A single operation (create, update, remove) on an feed item.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeedItemOperation {
    /// FieldMask that determines which resource fields are modified in an update.
    #[prost(message, optional, tag = "4")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
    /// The mutate operation.
    #[prost(oneof = "feed_item_operation::Operation", tags = "1, 2, 3")]
    pub operation: ::std::option::Option<feed_item_operation::Operation>,
}
pub mod feed_item_operation {
    /// The mutate operation.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Operation {
        /// Create operation: No resource name is expected for the new feed item.
        #[prost(message, tag = "1")]
        Create(super::super::resources::FeedItem),
        /// Update operation: The feed item is expected to have a valid resource
        /// name.
        #[prost(message, tag = "2")]
        Update(super::super::resources::FeedItem),
        /// Remove operation: A resource name for the removed feed item is
        /// expected, in this format:
        ///
        /// `customers/{customer_id}/feedItems/{feed_id}~{feed_item_id}`
        #[prost(string, tag = "3")]
        Remove(std::string::String),
    }
}
/// Response message for an feed item mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateFeedItemsResponse {
    /// Errors that pertain to operation failures in the partial failure mode.
    /// Returned only when partial_failure = true and all errors occur inside the
    /// operations. If any errors occur outside the operations (e.g. auth errors),
    /// we return an RPC level error.
    #[prost(message, optional, tag = "3")]
    pub partial_failure_error: ::std::option::Option<super::super::super::super::rpc::Status>,
    /// All results for the mutate.
    #[prost(message, repeated, tag = "2")]
    pub results: ::std::vec::Vec<MutateFeedItemResult>,
}
/// The result for the feed item mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateFeedItemResult {
    /// Returned for successful operations.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
#[doc = r" Generated client implementations."]
pub mod feed_item_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Service to manage feed items."]
    pub struct FeedItemServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> FeedItemServiceClient<T>
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
        #[doc = " Returns the requested feed item in full detail."]
        pub async fn get_feed_item(
            &mut self,
            request: impl tonic::IntoRequest<super::GetFeedItemRequest>,
        ) -> Result<tonic::Response<super::super::resources::FeedItem>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ads.googleads.v3.services.FeedItemService/GetFeedItem",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates, updates, or removes feed items. Operation statuses are"]
        #[doc = " returned."]
        pub async fn mutate_feed_items(
            &mut self,
            request: impl tonic::IntoRequest<super::MutateFeedItemsRequest>,
        ) -> Result<tonic::Response<super::MutateFeedItemsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ads.googleads.v3.services.FeedItemService/MutateFeedItems",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for FeedItemServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for FeedItemServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "FeedItemServiceClient {{ ... }}")
        }
    }
}
/// Request message for [FeedItemTargetService.GetFeedItemTarget][google.ads.googleads.v3.services.FeedItemTargetService.GetFeedItemTarget].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetFeedItemTargetRequest {
    /// Required. The resource name of the feed item targets to fetch.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
/// Request message for [FeedItemTargetService.MutateFeedItemTargets][google.ads.googleads.v3.services.FeedItemTargetService.MutateFeedItemTargets].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateFeedItemTargetsRequest {
    /// Required. The ID of the customer whose feed item targets are being modified.
    #[prost(string, tag = "1")]
    pub customer_id: std::string::String,
    /// Required. The list of operations to perform on individual feed item targets.
    #[prost(message, repeated, tag = "2")]
    pub operations: ::std::vec::Vec<FeedItemTargetOperation>,
}
/// A single operation (create, remove) on an feed item target.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeedItemTargetOperation {
    /// The mutate operation.
    #[prost(oneof = "feed_item_target_operation::Operation", tags = "1, 2")]
    pub operation: ::std::option::Option<feed_item_target_operation::Operation>,
}
pub mod feed_item_target_operation {
    /// The mutate operation.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Operation {
        /// Create operation: No resource name is expected for the new feed item
        /// target.
        #[prost(message, tag = "1")]
        Create(super::super::resources::FeedItemTarget),
        /// Remove operation: A resource name for the removed feed item target is
        /// expected, in this format:
        ///
        /// `customers/{customer_id}/feedItemTargets/{feed_id}~{feed_item_id}~{feed_item_target_type}~{feed_item_target_id}`
        #[prost(string, tag = "2")]
        Remove(std::string::String),
    }
}
/// Response message for an feed item target mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateFeedItemTargetsResponse {
    /// All results for the mutate.
    #[prost(message, repeated, tag = "2")]
    pub results: ::std::vec::Vec<MutateFeedItemTargetResult>,
}
/// The result for the feed item target mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateFeedItemTargetResult {
    /// Returned for successful operations.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
#[doc = r" Generated client implementations."]
pub mod feed_item_target_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Service to manage feed item targets."]
    pub struct FeedItemTargetServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> FeedItemTargetServiceClient<T>
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
        #[doc = " Returns the requested feed item targets in full detail."]
        pub async fn get_feed_item_target(
            &mut self,
            request: impl tonic::IntoRequest<super::GetFeedItemTargetRequest>,
        ) -> Result<tonic::Response<super::super::resources::FeedItemTarget>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ads.googleads.v3.services.FeedItemTargetService/GetFeedItemTarget",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates or removes feed item targets. Operation statuses are returned."]
        pub async fn mutate_feed_item_targets(
            &mut self,
            request: impl tonic::IntoRequest<super::MutateFeedItemTargetsRequest>,
        ) -> Result<tonic::Response<super::MutateFeedItemTargetsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ads.googleads.v3.services.FeedItemTargetService/MutateFeedItemTargets",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for FeedItemTargetServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for FeedItemTargetServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "FeedItemTargetServiceClient {{ ... }}")
        }
    }
}
/// Request message for [FeedMappingService.GetFeedMapping][google.ads.googleads.v3.services.FeedMappingService.GetFeedMapping].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetFeedMappingRequest {
    /// Required. The resource name of the feed mapping to fetch.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
/// Request message for [FeedMappingService.MutateFeedMappings][google.ads.googleads.v3.services.FeedMappingService.MutateFeedMappings].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateFeedMappingsRequest {
    /// Required. The ID of the customer whose feed mappings are being modified.
    #[prost(string, tag = "1")]
    pub customer_id: std::string::String,
    /// Required. The list of operations to perform on individual feed mappings.
    #[prost(message, repeated, tag = "2")]
    pub operations: ::std::vec::Vec<FeedMappingOperation>,
    /// If true, successful operations will be carried out and invalid
    /// operations will return errors. If false, all operations will be carried
    /// out in one transaction if and only if they are all valid.
    /// Default is false.
    #[prost(bool, tag = "3")]
    pub partial_failure: bool,
    /// If true, the request is validated but not executed. Only errors are
    /// returned, not results.
    #[prost(bool, tag = "4")]
    pub validate_only: bool,
}
/// A single operation (create, remove) on a feed mapping.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeedMappingOperation {
    /// The mutate operation.
    #[prost(oneof = "feed_mapping_operation::Operation", tags = "1, 3")]
    pub operation: ::std::option::Option<feed_mapping_operation::Operation>,
}
pub mod feed_mapping_operation {
    /// The mutate operation.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Operation {
        /// Create operation: No resource name is expected for the new feed mapping.
        #[prost(message, tag = "1")]
        Create(super::super::resources::FeedMapping),
        /// Remove operation: A resource name for the removed feed mapping is
        /// expected, in this format:
        ///
        /// `customers/{customer_id}/feedMappings/{feed_id}~{feed_mapping_id}`
        #[prost(string, tag = "3")]
        Remove(std::string::String),
    }
}
/// Response message for a feed mapping mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateFeedMappingsResponse {
    /// Errors that pertain to operation failures in the partial failure mode.
    /// Returned only when partial_failure = true and all errors occur inside the
    /// operations. If any errors occur outside the operations (e.g. auth errors),
    /// we return an RPC level error.
    #[prost(message, optional, tag = "3")]
    pub partial_failure_error: ::std::option::Option<super::super::super::super::rpc::Status>,
    /// All results for the mutate.
    #[prost(message, repeated, tag = "2")]
    pub results: ::std::vec::Vec<MutateFeedMappingResult>,
}
/// The result for the feed mapping mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateFeedMappingResult {
    /// Returned for successful operations.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
#[doc = r" Generated client implementations."]
pub mod feed_mapping_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Service to manage feed mappings."]
    pub struct FeedMappingServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> FeedMappingServiceClient<T>
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
        #[doc = " Returns the requested feed mapping in full detail."]
        pub async fn get_feed_mapping(
            &mut self,
            request: impl tonic::IntoRequest<super::GetFeedMappingRequest>,
        ) -> Result<tonic::Response<super::super::resources::FeedMapping>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ads.googleads.v3.services.FeedMappingService/GetFeedMapping",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates or removes feed mappings. Operation statuses are"]
        #[doc = " returned."]
        pub async fn mutate_feed_mappings(
            &mut self,
            request: impl tonic::IntoRequest<super::MutateFeedMappingsRequest>,
        ) -> Result<tonic::Response<super::MutateFeedMappingsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ads.googleads.v3.services.FeedMappingService/MutateFeedMappings",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for FeedMappingServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for FeedMappingServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "FeedMappingServiceClient {{ ... }}")
        }
    }
}
/// Request message for [FeedPlaceholderViewService.GetFeedPlaceholderView][google.ads.googleads.v3.services.FeedPlaceholderViewService.GetFeedPlaceholderView].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetFeedPlaceholderViewRequest {
    /// Required. The resource name of the feed placeholder view to fetch.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
#[doc = r" Generated client implementations."]
pub mod feed_placeholder_view_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Service to fetch feed placeholder views."]
    pub struct FeedPlaceholderViewServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> FeedPlaceholderViewServiceClient<T>
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
        #[doc = " Returns the requested feed placeholder view in full detail."]
        pub async fn get_feed_placeholder_view(
            &mut self,
            request: impl tonic::IntoRequest<super::GetFeedPlaceholderViewRequest>,
        ) -> Result<tonic::Response<super::super::resources::FeedPlaceholderView>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ( "/google.ads.googleads.v3.services.FeedPlaceholderViewService/GetFeedPlaceholderView" ) ;
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for FeedPlaceholderViewServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for FeedPlaceholderViewServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "FeedPlaceholderViewServiceClient {{ ... }}")
        }
    }
}
/// Request message for [FeedService.GetFeed][google.ads.googleads.v3.services.FeedService.GetFeed].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetFeedRequest {
    /// Required. The resource name of the feed to fetch.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
/// Request message for [FeedService.MutateFeeds][google.ads.googleads.v3.services.FeedService.MutateFeeds].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateFeedsRequest {
    /// Required. The ID of the customer whose feeds are being modified.
    #[prost(string, tag = "1")]
    pub customer_id: std::string::String,
    /// Required. The list of operations to perform on individual feeds.
    #[prost(message, repeated, tag = "2")]
    pub operations: ::std::vec::Vec<FeedOperation>,
    /// If true, successful operations will be carried out and invalid
    /// operations will return errors. If false, all operations will be carried
    /// out in one transaction if and only if they are all valid.
    /// Default is false.
    #[prost(bool, tag = "3")]
    pub partial_failure: bool,
    /// If true, the request is validated but not executed. Only errors are
    /// returned, not results.
    #[prost(bool, tag = "4")]
    pub validate_only: bool,
}
/// A single operation (create, update, remove) on an feed.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeedOperation {
    /// FieldMask that determines which resource fields are modified in an update.
    #[prost(message, optional, tag = "4")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
    /// The mutate operation.
    #[prost(oneof = "feed_operation::Operation", tags = "1, 2, 3")]
    pub operation: ::std::option::Option<feed_operation::Operation>,
}
pub mod feed_operation {
    /// The mutate operation.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Operation {
        /// Create operation: No resource name is expected for the new feed.
        #[prost(message, tag = "1")]
        Create(super::super::resources::Feed),
        /// Update operation: The feed is expected to have a valid resource
        /// name.
        #[prost(message, tag = "2")]
        Update(super::super::resources::Feed),
        /// Remove operation: A resource name for the removed feed is
        /// expected, in this format:
        ///
        /// `customers/{customer_id}/feeds/{feed_id}`
        #[prost(string, tag = "3")]
        Remove(std::string::String),
    }
}
/// Response message for an feed mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateFeedsResponse {
    /// Errors that pertain to operation failures in the partial failure mode.
    /// Returned only when partial_failure = true and all errors occur inside the
    /// operations. If any errors occur outside the operations (e.g. auth errors),
    /// we return an RPC level error.
    #[prost(message, optional, tag = "3")]
    pub partial_failure_error: ::std::option::Option<super::super::super::super::rpc::Status>,
    /// All results for the mutate.
    #[prost(message, repeated, tag = "2")]
    pub results: ::std::vec::Vec<MutateFeedResult>,
}
/// The result for the feed mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateFeedResult {
    /// Returned for successful operations.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
#[doc = r" Generated client implementations."]
pub mod feed_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Service to manage feeds."]
    pub struct FeedServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> FeedServiceClient<T>
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
        #[doc = " Returns the requested feed in full detail."]
        pub async fn get_feed(
            &mut self,
            request: impl tonic::IntoRequest<super::GetFeedRequest>,
        ) -> Result<tonic::Response<super::super::resources::Feed>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ads.googleads.v3.services.FeedService/GetFeed",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates, updates, or removes feeds. Operation statuses are"]
        #[doc = " returned."]
        pub async fn mutate_feeds(
            &mut self,
            request: impl tonic::IntoRequest<super::MutateFeedsRequest>,
        ) -> Result<tonic::Response<super::MutateFeedsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ads.googleads.v3.services.FeedService/MutateFeeds",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for FeedServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for FeedServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "FeedServiceClient {{ ... }}")
        }
    }
}
/// Request message for [GenderViewService.GetGenderView][google.ads.googleads.v3.services.GenderViewService.GetGenderView].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetGenderViewRequest {
    /// Required. The resource name of the gender view to fetch.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
#[doc = r" Generated client implementations."]
pub mod gender_view_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Service to manage gender views."]
    pub struct GenderViewServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> GenderViewServiceClient<T>
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
        #[doc = " Returns the requested gender view in full detail."]
        pub async fn get_gender_view(
            &mut self,
            request: impl tonic::IntoRequest<super::GetGenderViewRequest>,
        ) -> Result<tonic::Response<super::super::resources::GenderView>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ads.googleads.v3.services.GenderViewService/GetGenderView",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for GenderViewServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for GenderViewServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "GenderViewServiceClient {{ ... }}")
        }
    }
}
/// Request message for [GeoTargetConstantService.GetGeoTargetConstant][google.ads.googleads.v3.services.GeoTargetConstantService.GetGeoTargetConstant].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetGeoTargetConstantRequest {
    /// Required. The resource name of the geo target constant to fetch.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
/// Request message for
/// [GeoTargetConstantService.SuggestGeoTargetConstants][google.ads.googleads.v3.services.GeoTargetConstantService.SuggestGeoTargetConstants].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SuggestGeoTargetConstantsRequest {
    /// If possible, returned geo targets are translated using this locale. If not,
    /// en is used by default. This is also used as a hint for returned geo
    /// targets.
    #[prost(message, optional, tag = "3")]
    pub locale: ::std::option::Option<::std::string::String>,
    /// Returned geo targets are restricted to this country code.
    #[prost(message, optional, tag = "5")]
    pub country_code: ::std::option::Option<::std::string::String>,
    /// Required. A selector of geo target constants.
    #[prost(oneof = "suggest_geo_target_constants_request::Query", tags = "1, 2")]
    pub query: ::std::option::Option<suggest_geo_target_constants_request::Query>,
}
pub mod suggest_geo_target_constants_request {
    /// A list of location names.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct LocationNames {
        /// A list of location names.
        #[prost(message, repeated, tag = "1")]
        pub names: ::std::vec::Vec<::std::string::String>,
    }
    /// A list of geo target constant resource names.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GeoTargets {
        /// A list of geo target constant resource names.
        #[prost(message, repeated, tag = "1")]
        pub geo_target_constants: ::std::vec::Vec<::std::string::String>,
    }
    /// Required. A selector of geo target constants.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Query {
        /// The location names to search by. At most 25 names can be set.
        #[prost(message, tag = "1")]
        LocationNames(LocationNames),
        /// The geo target constant resource names to filter by.
        #[prost(message, tag = "2")]
        GeoTargets(GeoTargets),
    }
}
/// Response message for [GeoTargetConstantService.SuggestGeoTargetConstants][google.ads.googleads.v3.services.GeoTargetConstantService.SuggestGeoTargetConstants].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SuggestGeoTargetConstantsResponse {
    /// Geo target constant suggestions.
    #[prost(message, repeated, tag = "1")]
    pub geo_target_constant_suggestions: ::std::vec::Vec<GeoTargetConstantSuggestion>,
}
/// A geo target constant suggestion.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GeoTargetConstantSuggestion {
    /// The language this GeoTargetConstantSuggestion is currently translated to.
    /// It affects the name of geo target fields. For example, if locale=en, then
    /// name=Spain. If locale=es, then name=España. The default locale will be
    /// returned if no translation exists for the locale in the request.
    #[prost(message, optional, tag = "1")]
    pub locale: ::std::option::Option<::std::string::String>,
    /// Approximate user population that will be targeted, rounded to the
    /// nearest 100.
    #[prost(message, optional, tag = "2")]
    pub reach: ::std::option::Option<i64>,
    /// If the request searched by location name, this is the location name that
    /// matched the geo target.
    #[prost(message, optional, tag = "3")]
    pub search_term: ::std::option::Option<::std::string::String>,
    /// The GeoTargetConstant result.
    #[prost(message, optional, tag = "4")]
    pub geo_target_constant: ::std::option::Option<super::resources::GeoTargetConstant>,
    /// The list of parents of the geo target constant.
    #[prost(message, repeated, tag = "5")]
    pub geo_target_constant_parents: ::std::vec::Vec<super::resources::GeoTargetConstant>,
}
#[doc = r" Generated client implementations."]
pub mod geo_target_constant_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Service to fetch geo target constants."]
    pub struct GeoTargetConstantServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> GeoTargetConstantServiceClient<T>
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
        #[doc = " Returns the requested geo target constant in full detail."]
        pub async fn get_geo_target_constant(
            &mut self,
            request: impl tonic::IntoRequest<super::GetGeoTargetConstantRequest>,
        ) -> Result<tonic::Response<super::super::resources::GeoTargetConstant>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ads.googleads.v3.services.GeoTargetConstantService/GetGeoTargetConstant",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns GeoTargetConstant suggestions by location name or by resource name."]
        pub async fn suggest_geo_target_constants(
            &mut self,
            request: impl tonic::IntoRequest<super::SuggestGeoTargetConstantsRequest>,
        ) -> Result<tonic::Response<super::SuggestGeoTargetConstantsResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ( "/google.ads.googleads.v3.services.GeoTargetConstantService/SuggestGeoTargetConstants" ) ;
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for GeoTargetConstantServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for GeoTargetConstantServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "GeoTargetConstantServiceClient {{ ... }}")
        }
    }
}
/// Request message for [GeographicViewService.GetGeographicView][google.ads.googleads.v3.services.GeographicViewService.GetGeographicView].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetGeographicViewRequest {
    /// Required. The resource name of the geographic view to fetch.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
#[doc = r" Generated client implementations."]
pub mod geographic_view_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Service to manage geographic views."]
    pub struct GeographicViewServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> GeographicViewServiceClient<T>
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
        #[doc = " Returns the requested geographic view in full detail."]
        pub async fn get_geographic_view(
            &mut self,
            request: impl tonic::IntoRequest<super::GetGeographicViewRequest>,
        ) -> Result<tonic::Response<super::super::resources::GeographicView>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ads.googleads.v3.services.GeographicViewService/GetGeographicView",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for GeographicViewServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for GeographicViewServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "GeographicViewServiceClient {{ ... }}")
        }
    }
}
/// Request message for [GoogleAdsFieldService.GetGoogleAdsField][google.ads.googleads.v3.services.GoogleAdsFieldService.GetGoogleAdsField].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetGoogleAdsFieldRequest {
    /// Required. The resource name of the field to get.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
/// Request message for [GoogleAdsFieldService.SearchGoogleAdsFields][google.ads.googleads.v3.services.GoogleAdsFieldService.SearchGoogleAdsFields].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchGoogleAdsFieldsRequest {
    /// Required. The query string.
    #[prost(string, tag = "1")]
    pub query: std::string::String,
    /// Token of the page to retrieve. If not specified, the first page of
    /// results will be returned. Use the value obtained from `next_page_token`
    /// in the previous response in order to request the next page of results.
    #[prost(string, tag = "2")]
    pub page_token: std::string::String,
    /// Number of elements to retrieve in a single page.
    /// When too large a page is requested, the server may decide to further
    /// limit the number of returned resources.
    #[prost(int32, tag = "3")]
    pub page_size: i32,
}
/// Response message for [GoogleAdsFieldService.SearchGoogleAdsFields][google.ads.googleads.v3.services.GoogleAdsFieldService.SearchGoogleAdsFields].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchGoogleAdsFieldsResponse {
    /// The list of fields that matched the query.
    #[prost(message, repeated, tag = "1")]
    pub results: ::std::vec::Vec<super::resources::GoogleAdsField>,
    /// Pagination token used to retrieve the next page of results. Pass the
    /// content of this string as the `page_token` attribute of the next request.
    /// `next_page_token` is not returned for the last page.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
    /// Total number of results that match the query ignoring the LIMIT clause.
    #[prost(int64, tag = "3")]
    pub total_results_count: i64,
}
#[doc = r" Generated client implementations."]
pub mod google_ads_field_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Service to fetch Google Ads API fields."]
    pub struct GoogleAdsFieldServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> GoogleAdsFieldServiceClient<T>
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
        #[doc = " Returns just the requested field."]
        pub async fn get_google_ads_field(
            &mut self,
            request: impl tonic::IntoRequest<super::GetGoogleAdsFieldRequest>,
        ) -> Result<tonic::Response<super::super::resources::GoogleAdsField>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ads.googleads.v3.services.GoogleAdsFieldService/GetGoogleAdsField",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns all fields that match the search query."]
        pub async fn search_google_ads_fields(
            &mut self,
            request: impl tonic::IntoRequest<super::SearchGoogleAdsFieldsRequest>,
        ) -> Result<tonic::Response<super::SearchGoogleAdsFieldsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ads.googleads.v3.services.GoogleAdsFieldService/SearchGoogleAdsFields",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for GoogleAdsFieldServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for GoogleAdsFieldServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "GoogleAdsFieldServiceClient {{ ... }}")
        }
    }
}
/// Request message for [LabelService.GetLabel][google.ads.googleads.v3.services.LabelService.GetLabel].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLabelRequest {
    /// Required. The resource name of the label to fetch.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
/// Request message for [LabelService.MutateLabels][google.ads.googleads.v3.services.LabelService.MutateLabels].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateLabelsRequest {
    /// Required. ID of the customer whose labels are being modified.
    #[prost(string, tag = "1")]
    pub customer_id: std::string::String,
    /// Required. The list of operations to perform on labels.
    #[prost(message, repeated, tag = "2")]
    pub operations: ::std::vec::Vec<LabelOperation>,
    /// If true, successful operations will be carried out and invalid
    /// operations will return errors. If false, all operations will be carried
    /// out in one transaction if and only if they are all valid.
    /// Default is false.
    #[prost(bool, tag = "3")]
    pub partial_failure: bool,
    /// If true, the request is validated but not executed. Only errors are
    /// returned, not results.
    #[prost(bool, tag = "4")]
    pub validate_only: bool,
}
/// A single operation (create, remove, update) on a label.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LabelOperation {
    /// FieldMask that determines which resource fields are modified in an update.
    #[prost(message, optional, tag = "4")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
    /// The mutate operation.
    #[prost(oneof = "label_operation::Operation", tags = "1, 2, 3")]
    pub operation: ::std::option::Option<label_operation::Operation>,
}
pub mod label_operation {
    /// The mutate operation.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Operation {
        /// Create operation: No resource name is expected for the new label.
        #[prost(message, tag = "1")]
        Create(super::super::resources::Label),
        /// Update operation: The label is expected to have a valid resource name.
        #[prost(message, tag = "2")]
        Update(super::super::resources::Label),
        /// Remove operation: A resource name for the label being removed, in
        /// this format:
        ///
        /// `customers/{customer_id}/labels/{label_id}`
        #[prost(string, tag = "3")]
        Remove(std::string::String),
    }
}
/// Response message for a labels mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateLabelsResponse {
    /// Errors that pertain to operation failures in the partial failure mode.
    /// Returned only when partial_failure = true and all errors occur inside the
    /// operations. If any errors occur outside the operations (e.g. auth errors),
    /// we return an RPC level error.
    #[prost(message, optional, tag = "3")]
    pub partial_failure_error: ::std::option::Option<super::super::super::super::rpc::Status>,
    /// All results for the mutate.
    #[prost(message, repeated, tag = "2")]
    pub results: ::std::vec::Vec<MutateLabelResult>,
}
/// The result for a label mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateLabelResult {
    /// Returned for successful operations.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
#[doc = r" Generated client implementations."]
pub mod label_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Service to manage labels."]
    pub struct LabelServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> LabelServiceClient<T>
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
        #[doc = " Returns the requested label in full detail."]
        pub async fn get_label(
            &mut self,
            request: impl tonic::IntoRequest<super::GetLabelRequest>,
        ) -> Result<tonic::Response<super::super::resources::Label>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ads.googleads.v3.services.LabelService/GetLabel",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates, updates, or removes labels. Operation statuses are returned."]
        pub async fn mutate_labels(
            &mut self,
            request: impl tonic::IntoRequest<super::MutateLabelsRequest>,
        ) -> Result<tonic::Response<super::MutateLabelsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ads.googleads.v3.services.LabelService/MutateLabels",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for LabelServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for LabelServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "LabelServiceClient {{ ... }}")
        }
    }
}
/// Request message for [MediaFileService.GetMediaFile][google.ads.googleads.v3.services.MediaFileService.GetMediaFile]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMediaFileRequest {
    /// Required. The resource name of the media file to fetch.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
/// Request message for [MediaFileService.MutateMediaFiles][google.ads.googleads.v3.services.MediaFileService.MutateMediaFiles]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateMediaFilesRequest {
    /// Required. The ID of the customer whose media files are being modified.
    #[prost(string, tag = "1")]
    pub customer_id: std::string::String,
    /// Required. The list of operations to perform on individual media file.
    #[prost(message, repeated, tag = "2")]
    pub operations: ::std::vec::Vec<MediaFileOperation>,
    /// If true, successful operations will be carried out and invalid
    /// operations will return errors. If false, all operations will be carried
    /// out in one transaction if and only if they are all valid.
    /// Default is false.
    #[prost(bool, tag = "3")]
    pub partial_failure: bool,
    /// If true, the request is validated but not executed. Only errors are
    /// returned, not results.
    #[prost(bool, tag = "4")]
    pub validate_only: bool,
}
/// A single operation to create media file.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MediaFileOperation {
    /// The mutate operation.
    #[prost(oneof = "media_file_operation::Operation", tags = "1")]
    pub operation: ::std::option::Option<media_file_operation::Operation>,
}
pub mod media_file_operation {
    /// The mutate operation.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Operation {
        /// Create operation: No resource name is expected for the new media file.
        #[prost(message, tag = "1")]
        Create(super::super::resources::MediaFile),
    }
}
/// Response message for a media file mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateMediaFilesResponse {
    /// Errors that pertain to operation failures in the partial failure mode.
    /// Returned only when partial_failure = true and all errors occur inside the
    /// operations. If any errors occur outside the operations (e.g. auth errors),
    /// we return an RPC level error.
    #[prost(message, optional, tag = "3")]
    pub partial_failure_error: ::std::option::Option<super::super::super::super::rpc::Status>,
    /// All results for the mutate.
    #[prost(message, repeated, tag = "2")]
    pub results: ::std::vec::Vec<MutateMediaFileResult>,
}
/// The result for the media file mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateMediaFileResult {
    /// The resource name returned for successful operations.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
#[doc = r" Generated client implementations."]
pub mod media_file_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Service to manage media files."]
    pub struct MediaFileServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> MediaFileServiceClient<T>
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
        #[doc = " Returns the requested media file in full detail."]
        pub async fn get_media_file(
            &mut self,
            request: impl tonic::IntoRequest<super::GetMediaFileRequest>,
        ) -> Result<tonic::Response<super::super::resources::MediaFile>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ads.googleads.v3.services.MediaFileService/GetMediaFile",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates media files. Operation statuses are returned."]
        pub async fn mutate_media_files(
            &mut self,
            request: impl tonic::IntoRequest<super::MutateMediaFilesRequest>,
        ) -> Result<tonic::Response<super::MutateMediaFilesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ads.googleads.v3.services.MediaFileService/MutateMediaFiles",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for MediaFileServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for MediaFileServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "MediaFileServiceClient {{ ... }}")
        }
    }
}
/// Request message for [RemarketingActionService.GetRemarketingAction][google.ads.googleads.v3.services.RemarketingActionService.GetRemarketingAction].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRemarketingActionRequest {
    /// Required. The resource name of the remarketing action to fetch.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
/// Request message for [RemarketingActionService.MutateRemarketingActions][google.ads.googleads.v3.services.RemarketingActionService.MutateRemarketingActions].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateRemarketingActionsRequest {
    /// Required. The ID of the customer whose remarketing actions are being modified.
    #[prost(string, tag = "1")]
    pub customer_id: std::string::String,
    /// Required. The list of operations to perform on individual remarketing actions.
    #[prost(message, repeated, tag = "2")]
    pub operations: ::std::vec::Vec<RemarketingActionOperation>,
    /// If true, successful operations will be carried out and invalid
    /// operations will return errors. If false, all operations will be carried
    /// out in one transaction if and only if they are all valid.
    /// Default is false.
    #[prost(bool, tag = "3")]
    pub partial_failure: bool,
    /// If true, the request is validated but not executed. Only errors are
    /// returned, not results.
    #[prost(bool, tag = "4")]
    pub validate_only: bool,
}
/// A single operation (create, update) on a remarketing action.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemarketingActionOperation {
    /// FieldMask that determines which resource fields are modified in an update.
    #[prost(message, optional, tag = "4")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
    /// The mutate operation.
    #[prost(oneof = "remarketing_action_operation::Operation", tags = "1, 2")]
    pub operation: ::std::option::Option<remarketing_action_operation::Operation>,
}
pub mod remarketing_action_operation {
    /// The mutate operation.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Operation {
        /// Create operation: No resource name is expected for the new remarketing
        /// action.
        #[prost(message, tag = "1")]
        Create(super::super::resources::RemarketingAction),
        /// Update operation: The remarketing action is expected to have a valid
        /// resource name.
        #[prost(message, tag = "2")]
        Update(super::super::resources::RemarketingAction),
    }
}
/// Response message for remarketing action mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateRemarketingActionsResponse {
    /// Errors that pertain to operation failures in the partial failure mode.
    /// Returned only when partial_failure = true and all errors occur inside the
    /// operations. If any errors occur outside the operations (e.g. auth errors),
    /// we return an RPC level error.
    #[prost(message, optional, tag = "3")]
    pub partial_failure_error: ::std::option::Option<super::super::super::super::rpc::Status>,
    /// All results for the mutate.
    #[prost(message, repeated, tag = "2")]
    pub results: ::std::vec::Vec<MutateRemarketingActionResult>,
}
/// The result for the remarketing action mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateRemarketingActionResult {
    /// Returned for successful operations.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
#[doc = r" Generated client implementations."]
pub mod remarketing_action_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Service to manage remarketing actions."]
    pub struct RemarketingActionServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> RemarketingActionServiceClient<T>
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
        #[doc = " Returns the requested remarketing action in full detail."]
        pub async fn get_remarketing_action(
            &mut self,
            request: impl tonic::IntoRequest<super::GetRemarketingActionRequest>,
        ) -> Result<tonic::Response<super::super::resources::RemarketingAction>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ads.googleads.v3.services.RemarketingActionService/GetRemarketingAction",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates or updates remarketing actions. Operation statuses are returned."]
        pub async fn mutate_remarketing_actions(
            &mut self,
            request: impl tonic::IntoRequest<super::MutateRemarketingActionsRequest>,
        ) -> Result<tonic::Response<super::MutateRemarketingActionsResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ( "/google.ads.googleads.v3.services.RemarketingActionService/MutateRemarketingActions" ) ;
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for RemarketingActionServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for RemarketingActionServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "RemarketingActionServiceClient {{ ... }}")
        }
    }
}
/// Request message for [SharedCriterionService.GetSharedCriterion][google.ads.googleads.v3.services.SharedCriterionService.GetSharedCriterion].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSharedCriterionRequest {
    /// Required. The resource name of the shared criterion to fetch.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
/// Request message for [SharedCriterionService.MutateSharedCriteria][google.ads.googleads.v3.services.SharedCriterionService.MutateSharedCriteria].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateSharedCriteriaRequest {
    /// Required. The ID of the customer whose shared criteria are being modified.
    #[prost(string, tag = "1")]
    pub customer_id: std::string::String,
    /// Required. The list of operations to perform on individual shared criteria.
    #[prost(message, repeated, tag = "2")]
    pub operations: ::std::vec::Vec<SharedCriterionOperation>,
    /// If true, successful operations will be carried out and invalid
    /// operations will return errors. If false, all operations will be carried
    /// out in one transaction if and only if they are all valid.
    /// Default is false.
    #[prost(bool, tag = "3")]
    pub partial_failure: bool,
    /// If true, the request is validated but not executed. Only errors are
    /// returned, not results.
    #[prost(bool, tag = "4")]
    pub validate_only: bool,
}
/// A single operation (create, remove) on an shared criterion.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SharedCriterionOperation {
    /// The mutate operation.
    #[prost(oneof = "shared_criterion_operation::Operation", tags = "1, 3")]
    pub operation: ::std::option::Option<shared_criterion_operation::Operation>,
}
pub mod shared_criterion_operation {
    /// The mutate operation.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Operation {
        /// Create operation: No resource name is expected for the new shared
        /// criterion.
        #[prost(message, tag = "1")]
        Create(super::super::resources::SharedCriterion),
        /// Remove operation: A resource name for the removed shared criterion is
        /// expected, in this format:
        ///
        /// `customers/{customer_id}/sharedCriteria/{shared_set_id}~{criterion_id}`
        #[prost(string, tag = "3")]
        Remove(std::string::String),
    }
}
/// Response message for a shared criterion mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateSharedCriteriaResponse {
    /// Errors that pertain to operation failures in the partial failure mode.
    /// Returned only when partial_failure = true and all errors occur inside the
    /// operations. If any errors occur outside the operations (e.g. auth errors),
    /// we return an RPC level error.
    #[prost(message, optional, tag = "3")]
    pub partial_failure_error: ::std::option::Option<super::super::super::super::rpc::Status>,
    /// All results for the mutate.
    #[prost(message, repeated, tag = "2")]
    pub results: ::std::vec::Vec<MutateSharedCriterionResult>,
}
/// The result for the shared criterion mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateSharedCriterionResult {
    /// Returned for successful operations.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
#[doc = r" Generated client implementations."]
pub mod shared_criterion_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Service to manage shared criteria."]
    pub struct SharedCriterionServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> SharedCriterionServiceClient<T>
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
        #[doc = " Returns the requested shared criterion in full detail."]
        pub async fn get_shared_criterion(
            &mut self,
            request: impl tonic::IntoRequest<super::GetSharedCriterionRequest>,
        ) -> Result<tonic::Response<super::super::resources::SharedCriterion>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ads.googleads.v3.services.SharedCriterionService/GetSharedCriterion",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates or removes shared criteria. Operation statuses are returned."]
        pub async fn mutate_shared_criteria(
            &mut self,
            request: impl tonic::IntoRequest<super::MutateSharedCriteriaRequest>,
        ) -> Result<tonic::Response<super::MutateSharedCriteriaResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ads.googleads.v3.services.SharedCriterionService/MutateSharedCriteria",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for SharedCriterionServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for SharedCriterionServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "SharedCriterionServiceClient {{ ... }}")
        }
    }
}
/// Request message for [SharedSetService.GetSharedSet][google.ads.googleads.v3.services.SharedSetService.GetSharedSet].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSharedSetRequest {
    /// Required. The resource name of the shared set to fetch.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
/// Request message for [SharedSetService.MutateSharedSets][google.ads.googleads.v3.services.SharedSetService.MutateSharedSets].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateSharedSetsRequest {
    /// Required. The ID of the customer whose shared sets are being modified.
    #[prost(string, tag = "1")]
    pub customer_id: std::string::String,
    /// Required. The list of operations to perform on individual shared sets.
    #[prost(message, repeated, tag = "2")]
    pub operations: ::std::vec::Vec<SharedSetOperation>,
    /// If true, successful operations will be carried out and invalid
    /// operations will return errors. If false, all operations will be carried
    /// out in one transaction if and only if they are all valid.
    /// Default is false.
    #[prost(bool, tag = "3")]
    pub partial_failure: bool,
    /// If true, the request is validated but not executed. Only errors are
    /// returned, not results.
    #[prost(bool, tag = "4")]
    pub validate_only: bool,
}
/// A single operation (create, update, remove) on an shared set.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SharedSetOperation {
    /// FieldMask that determines which resource fields are modified in an update.
    #[prost(message, optional, tag = "4")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
    /// The mutate operation.
    #[prost(oneof = "shared_set_operation::Operation", tags = "1, 2, 3")]
    pub operation: ::std::option::Option<shared_set_operation::Operation>,
}
pub mod shared_set_operation {
    /// The mutate operation.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Operation {
        /// Create operation: No resource name is expected for the new shared set.
        #[prost(message, tag = "1")]
        Create(super::super::resources::SharedSet),
        /// Update operation: The shared set is expected to have a valid resource
        /// name.
        #[prost(message, tag = "2")]
        Update(super::super::resources::SharedSet),
        /// Remove operation: A resource name for the removed shared set is expected,
        /// in this format:
        ///
        /// `customers/{customer_id}/sharedSets/{shared_set_id}`
        #[prost(string, tag = "3")]
        Remove(std::string::String),
    }
}
/// Response message for a shared set mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateSharedSetsResponse {
    /// Errors that pertain to operation failures in the partial failure mode.
    /// Returned only when partial_failure = true and all errors occur inside the
    /// operations. If any errors occur outside the operations (e.g. auth errors),
    /// we return an RPC level error.
    #[prost(message, optional, tag = "3")]
    pub partial_failure_error: ::std::option::Option<super::super::super::super::rpc::Status>,
    /// All results for the mutate.
    #[prost(message, repeated, tag = "2")]
    pub results: ::std::vec::Vec<MutateSharedSetResult>,
}
/// The result for the shared set mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateSharedSetResult {
    /// Returned for successful operations.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
#[doc = r" Generated client implementations."]
pub mod shared_set_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Service to manage shared sets."]
    pub struct SharedSetServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> SharedSetServiceClient<T>
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
        #[doc = " Returns the requested shared set in full detail."]
        pub async fn get_shared_set(
            &mut self,
            request: impl tonic::IntoRequest<super::GetSharedSetRequest>,
        ) -> Result<tonic::Response<super::super::resources::SharedSet>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ads.googleads.v3.services.SharedSetService/GetSharedSet",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates, updates, or removes shared sets. Operation statuses are returned."]
        pub async fn mutate_shared_sets(
            &mut self,
            request: impl tonic::IntoRequest<super::MutateSharedSetsRequest>,
        ) -> Result<tonic::Response<super::MutateSharedSetsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ads.googleads.v3.services.SharedSetService/MutateSharedSets",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for SharedSetServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for SharedSetServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "SharedSetServiceClient {{ ... }}")
        }
    }
}
/// Request message for [UserListService.GetUserList][google.ads.googleads.v3.services.UserListService.GetUserList].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetUserListRequest {
    /// Required. The resource name of the user list to fetch.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
/// Request message for [UserListService.MutateUserLists][google.ads.googleads.v3.services.UserListService.MutateUserLists].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateUserListsRequest {
    /// Required. The ID of the customer whose user lists are being modified.
    #[prost(string, tag = "1")]
    pub customer_id: std::string::String,
    /// Required. The list of operations to perform on individual user lists.
    #[prost(message, repeated, tag = "2")]
    pub operations: ::std::vec::Vec<UserListOperation>,
    /// If true, successful operations will be carried out and invalid
    /// operations will return errors. If false, all operations will be carried
    /// out in one transaction if and only if they are all valid.
    /// Default is false.
    #[prost(bool, tag = "3")]
    pub partial_failure: bool,
    /// If true, the request is validated but not executed. Only errors are
    /// returned, not results.
    #[prost(bool, tag = "4")]
    pub validate_only: bool,
}
/// A single operation (create, update) on a user list.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserListOperation {
    /// FieldMask that determines which resource fields are modified in an update.
    #[prost(message, optional, tag = "4")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
    /// The mutate operation.
    #[prost(oneof = "user_list_operation::Operation", tags = "1, 2, 3")]
    pub operation: ::std::option::Option<user_list_operation::Operation>,
}
pub mod user_list_operation {
    /// The mutate operation.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Operation {
        /// Create operation: No resource name is expected for the new user list.
        #[prost(message, tag = "1")]
        Create(super::super::resources::UserList),
        /// Update operation: The user list is expected to have a valid resource
        /// name.
        #[prost(message, tag = "2")]
        Update(super::super::resources::UserList),
        /// Remove operation: A resource name for the removed user list is expected,
        /// in this format:
        ///
        /// `customers/{customer_id}/userLists/{user_list_id}`
        #[prost(string, tag = "3")]
        Remove(std::string::String),
    }
}
/// Response message for user list mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateUserListsResponse {
    /// Errors that pertain to operation failures in the partial failure mode.
    /// Returned only when partial_failure = true and all errors occur inside the
    /// operations. If any errors occur outside the operations (e.g. auth errors),
    /// we return an RPC level error.
    #[prost(message, optional, tag = "3")]
    pub partial_failure_error: ::std::option::Option<super::super::super::super::rpc::Status>,
    /// All results for the mutate.
    #[prost(message, repeated, tag = "2")]
    pub results: ::std::vec::Vec<MutateUserListResult>,
}
/// The result for the user list mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateUserListResult {
    /// Returned for successful operations.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
#[doc = r" Generated client implementations."]
pub mod user_list_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Service to manage user lists."]
    pub struct UserListServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> UserListServiceClient<T>
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
        #[doc = " Returns the requested user list."]
        pub async fn get_user_list(
            &mut self,
            request: impl tonic::IntoRequest<super::GetUserListRequest>,
        ) -> Result<tonic::Response<super::super::resources::UserList>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ads.googleads.v3.services.UserListService/GetUserList",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates or updates user lists. Operation statuses are returned."]
        pub async fn mutate_user_lists(
            &mut self,
            request: impl tonic::IntoRequest<super::MutateUserListsRequest>,
        ) -> Result<tonic::Response<super::MutateUserListsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ads.googleads.v3.services.UserListService/MutateUserLists",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for UserListServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for UserListServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "UserListServiceClient {{ ... }}")
        }
    }
}
/// Request message for [GoogleAdsService.Search][google.ads.googleads.v3.services.GoogleAdsService.Search].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchGoogleAdsRequest {
    /// Required. The ID of the customer being queried.
    #[prost(string, tag = "1")]
    pub customer_id: std::string::String,
    /// Required. The query string.
    #[prost(string, tag = "2")]
    pub query: std::string::String,
    /// Token of the page to retrieve. If not specified, the first
    /// page of results will be returned. Use the value obtained from
    /// `next_page_token` in the previous response in order to request
    /// the next page of results.
    #[prost(string, tag = "3")]
    pub page_token: std::string::String,
    /// Number of elements to retrieve in a single page.
    /// When too large a page is requested, the server may decide to
    /// further limit the number of returned resources.
    #[prost(int32, tag = "4")]
    pub page_size: i32,
    /// If true, the request is validated but not executed.
    #[prost(bool, tag = "5")]
    pub validate_only: bool,
    /// If true, the total number of results that match the query ignoring the
    /// LIMIT clause will be included in the response.
    /// Default is false.
    #[prost(bool, tag = "7")]
    pub return_total_results_count: bool,
    /// Determines whether a summary row will be returned. By default, summary row
    /// is not returned. If requested, the summary row will be sent in a response
    /// by itself after all other query results are returned.
    #[prost(
        enumeration = "super::enums::summary_row_setting_enum::SummaryRowSetting",
        tag = "8"
    )]
    pub summary_row_setting: i32,
}
/// Response message for [GoogleAdsService.Search][google.ads.googleads.v3.services.GoogleAdsService.Search].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchGoogleAdsResponse {
    /// The list of rows that matched the query.
    #[prost(message, repeated, tag = "1")]
    pub results: ::std::vec::Vec<GoogleAdsRow>,
    /// Pagination token used to retrieve the next page of results.
    /// Pass the content of this string as the `page_token` attribute of
    /// the next request. `next_page_token` is not returned for the last
    /// page.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
    /// Total number of results that match the query ignoring the LIMIT
    /// clause.
    #[prost(int64, tag = "3")]
    pub total_results_count: i64,
    /// FieldMask that represents what fields were requested by the user.
    #[prost(message, optional, tag = "5")]
    pub field_mask: ::std::option::Option<::prost_types::FieldMask>,
    /// Summary row that contains summary of metrics in results.
    /// Summary of metrics means aggregation of metrics across all results,
    /// here aggregation could be sum, average, rate, etc.
    #[prost(message, optional, tag = "6")]
    pub summary_row: ::std::option::Option<GoogleAdsRow>,
}
/// Request message for [GoogleAdsService.SearchStream][google.ads.googleads.v3.services.GoogleAdsService.SearchStream].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchGoogleAdsStreamRequest {
    /// Required. The ID of the customer being queried.
    #[prost(string, tag = "1")]
    pub customer_id: std::string::String,
    /// Required. The query string.
    #[prost(string, tag = "2")]
    pub query: std::string::String,
    /// Determines whether a summary row will be returned. By default, summary row
    /// is not returned. If requested, the summary row will be sent in a response
    /// by itself after all other query results are returned.
    #[prost(
        enumeration = "super::enums::summary_row_setting_enum::SummaryRowSetting",
        tag = "3"
    )]
    pub summary_row_setting: i32,
}
/// Response message for [GoogleAdsService.SearchStream][google.ads.googleads.v3.services.GoogleAdsService.SearchStream].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchGoogleAdsStreamResponse {
    /// The list of rows that matched the query.
    #[prost(message, repeated, tag = "1")]
    pub results: ::std::vec::Vec<GoogleAdsRow>,
    /// FieldMask that represents what fields were requested by the user.
    #[prost(message, optional, tag = "2")]
    pub field_mask: ::std::option::Option<::prost_types::FieldMask>,
    /// Summary row that contains summary of metrics in results.
    /// Summary of metrics means aggregation of metrics across all results,
    /// here aggregation could be sum, average, rate, etc.
    #[prost(message, optional, tag = "3")]
    pub summary_row: ::std::option::Option<GoogleAdsRow>,
}
/// A returned row from the query.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GoogleAdsRow {
    /// The account budget in the query.
    #[prost(message, optional, tag = "42")]
    pub account_budget: ::std::option::Option<super::resources::AccountBudget>,
    /// The account budget proposal referenced in the query.
    #[prost(message, optional, tag = "43")]
    pub account_budget_proposal: ::std::option::Option<super::resources::AccountBudgetProposal>,
    /// The ad group referenced in the query.
    #[prost(message, optional, tag = "3")]
    pub ad_group: ::std::option::Option<super::resources::AdGroup>,
    /// The ad referenced in the query.
    #[prost(message, optional, tag = "16")]
    pub ad_group_ad: ::std::option::Option<super::resources::AdGroupAd>,
    /// The ad group ad asset view in the query.
    #[prost(message, optional, tag = "131")]
    pub ad_group_ad_asset_view: ::std::option::Option<super::resources::AdGroupAdAssetView>,
    /// The ad group ad label referenced in the query.
    #[prost(message, optional, tag = "120")]
    pub ad_group_ad_label: ::std::option::Option<super::resources::AdGroupAdLabel>,
    /// The ad group audience view referenced in the query.
    #[prost(message, optional, tag = "57")]
    pub ad_group_audience_view: ::std::option::Option<super::resources::AdGroupAudienceView>,
    /// The bid modifier referenced in the query.
    #[prost(message, optional, tag = "24")]
    pub ad_group_bid_modifier: ::std::option::Option<super::resources::AdGroupBidModifier>,
    /// The criterion referenced in the query.
    #[prost(message, optional, tag = "17")]
    pub ad_group_criterion: ::std::option::Option<super::resources::AdGroupCriterion>,
    /// The ad group criterion label referenced in the query.
    #[prost(message, optional, tag = "121")]
    pub ad_group_criterion_label: ::std::option::Option<super::resources::AdGroupCriterionLabel>,
    /// The ad group criterion simulation referenced in the query.
    #[prost(message, optional, tag = "110")]
    pub ad_group_criterion_simulation:
        ::std::option::Option<super::resources::AdGroupCriterionSimulation>,
    /// The ad group extension setting referenced in the query.
    #[prost(message, optional, tag = "112")]
    pub ad_group_extension_setting:
        ::std::option::Option<super::resources::AdGroupExtensionSetting>,
    /// The ad group feed referenced in the query.
    #[prost(message, optional, tag = "67")]
    pub ad_group_feed: ::std::option::Option<super::resources::AdGroupFeed>,
    /// The ad group label referenced in the query.
    #[prost(message, optional, tag = "115")]
    pub ad_group_label: ::std::option::Option<super::resources::AdGroupLabel>,
    /// The ad group simulation referenced in the query.
    #[prost(message, optional, tag = "107")]
    pub ad_group_simulation: ::std::option::Option<super::resources::AdGroupSimulation>,
    /// The ad parameter referenced in the query.
    #[prost(message, optional, tag = "130")]
    pub ad_parameter: ::std::option::Option<super::resources::AdParameter>,
    /// The age range view referenced in the query.
    #[prost(message, optional, tag = "48")]
    pub age_range_view: ::std::option::Option<super::resources::AgeRangeView>,
    /// The ad schedule view referenced in the query.
    #[prost(message, optional, tag = "89")]
    pub ad_schedule_view: ::std::option::Option<super::resources::AdScheduleView>,
    /// The domain category referenced in the query.
    #[prost(message, optional, tag = "91")]
    pub domain_category: ::std::option::Option<super::resources::DomainCategory>,
    /// The asset referenced in the query.
    #[prost(message, optional, tag = "105")]
    pub asset: ::std::option::Option<super::resources::Asset>,
    /// The bidding strategy referenced in the query.
    #[prost(message, optional, tag = "18")]
    pub bidding_strategy: ::std::option::Option<super::resources::BiddingStrategy>,
    /// The billing setup referenced in the query.
    #[prost(message, optional, tag = "41")]
    pub billing_setup: ::std::option::Option<super::resources::BillingSetup>,
    /// The campaign budget referenced in the query.
    #[prost(message, optional, tag = "19")]
    pub campaign_budget: ::std::option::Option<super::resources::CampaignBudget>,
    /// The campaign referenced in the query.
    #[prost(message, optional, tag = "2")]
    pub campaign: ::std::option::Option<super::resources::Campaign>,
    /// The campaign audience view referenced in the query.
    #[prost(message, optional, tag = "69")]
    pub campaign_audience_view: ::std::option::Option<super::resources::CampaignAudienceView>,
    /// The campaign bid modifier referenced in the query.
    #[prost(message, optional, tag = "26")]
    pub campaign_bid_modifier: ::std::option::Option<super::resources::CampaignBidModifier>,
    /// The campaign criterion referenced in the query.
    #[prost(message, optional, tag = "20")]
    pub campaign_criterion: ::std::option::Option<super::resources::CampaignCriterion>,
    /// The campaign criterion simulation referenced in the query.
    #[prost(message, optional, tag = "111")]
    pub campaign_criterion_simulation:
        ::std::option::Option<super::resources::CampaignCriterionSimulation>,
    /// The campaign draft referenced in the query.
    #[prost(message, optional, tag = "49")]
    pub campaign_draft: ::std::option::Option<super::resources::CampaignDraft>,
    /// The campaign experiment referenced in the query.
    #[prost(message, optional, tag = "84")]
    pub campaign_experiment: ::std::option::Option<super::resources::CampaignExperiment>,
    /// The campaign extension setting referenced in the query.
    #[prost(message, optional, tag = "113")]
    pub campaign_extension_setting:
        ::std::option::Option<super::resources::CampaignExtensionSetting>,
    /// The campaign feed referenced in the query.
    #[prost(message, optional, tag = "63")]
    pub campaign_feed: ::std::option::Option<super::resources::CampaignFeed>,
    /// The campaign label referenced in the query.
    #[prost(message, optional, tag = "108")]
    pub campaign_label: ::std::option::Option<super::resources::CampaignLabel>,
    /// Campaign Shared Set referenced in AWQL query.
    #[prost(message, optional, tag = "30")]
    pub campaign_shared_set: ::std::option::Option<super::resources::CampaignSharedSet>,
    /// The carrier constant referenced in the query.
    #[prost(message, optional, tag = "66")]
    pub carrier_constant: ::std::option::Option<super::resources::CarrierConstant>,
    /// The ChangeStatus referenced in the query.
    #[prost(message, optional, tag = "37")]
    pub change_status: ::std::option::Option<super::resources::ChangeStatus>,
    /// The conversion action referenced in the query.
    #[prost(message, optional, tag = "103")]
    pub conversion_action: ::std::option::Option<super::resources::ConversionAction>,
    /// The ClickView referenced in the query.
    #[prost(message, optional, tag = "122")]
    pub click_view: ::std::option::Option<super::resources::ClickView>,
    /// The currency constant referenced in the query.
    #[prost(message, optional, tag = "134")]
    pub currency_constant: ::std::option::Option<super::resources::CurrencyConstant>,
    /// The CustomInterest referenced in the query.
    #[prost(message, optional, tag = "104")]
    pub custom_interest: ::std::option::Option<super::resources::CustomInterest>,
    /// The customer referenced in the query.
    #[prost(message, optional, tag = "1")]
    pub customer: ::std::option::Option<super::resources::Customer>,
    /// The CustomerManagerLink referenced in the query.
    #[prost(message, optional, tag = "61")]
    pub customer_manager_link: ::std::option::Option<super::resources::CustomerManagerLink>,
    /// The CustomerClientLink referenced in the query.
    #[prost(message, optional, tag = "62")]
    pub customer_client_link: ::std::option::Option<super::resources::CustomerClientLink>,
    /// The CustomerClient referenced in the query.
    #[prost(message, optional, tag = "70")]
    pub customer_client: ::std::option::Option<super::resources::CustomerClient>,
    /// The customer extension setting referenced in the query.
    #[prost(message, optional, tag = "114")]
    pub customer_extension_setting:
        ::std::option::Option<super::resources::CustomerExtensionSetting>,
    /// The customer feed referenced in the query.
    #[prost(message, optional, tag = "64")]
    pub customer_feed: ::std::option::Option<super::resources::CustomerFeed>,
    /// The customer label referenced in the query.
    #[prost(message, optional, tag = "124")]
    pub customer_label: ::std::option::Option<super::resources::CustomerLabel>,
    /// The customer negative criterion referenced in the query.
    #[prost(message, optional, tag = "88")]
    pub customer_negative_criterion:
        ::std::option::Option<super::resources::CustomerNegativeCriterion>,
    /// The detail placement view referenced in the query.
    #[prost(message, optional, tag = "118")]
    pub detail_placement_view: ::std::option::Option<super::resources::DetailPlacementView>,
    /// The display keyword view referenced in the query.
    #[prost(message, optional, tag = "47")]
    pub display_keyword_view: ::std::option::Option<super::resources::DisplayKeywordView>,
    /// The distance view referenced in the query.
    #[prost(message, optional, tag = "132")]
    pub distance_view: ::std::option::Option<super::resources::DistanceView>,
    /// The dynamic search ads search term view referenced in the query.
    #[prost(message, optional, tag = "106")]
    pub dynamic_search_ads_search_term_view:
        ::std::option::Option<super::resources::DynamicSearchAdsSearchTermView>,
    /// The expanded landing page view referenced in the query.
    #[prost(message, optional, tag = "128")]
    pub expanded_landing_page_view:
        ::std::option::Option<super::resources::ExpandedLandingPageView>,
    /// The extension feed item referenced in the query.
    #[prost(message, optional, tag = "85")]
    pub extension_feed_item: ::std::option::Option<super::resources::ExtensionFeedItem>,
    /// The feed referenced in the query.
    #[prost(message, optional, tag = "46")]
    pub feed: ::std::option::Option<super::resources::Feed>,
    /// The feed item referenced in the query.
    #[prost(message, optional, tag = "50")]
    pub feed_item: ::std::option::Option<super::resources::FeedItem>,
    /// The feed item target referenced in the query.
    #[prost(message, optional, tag = "116")]
    pub feed_item_target: ::std::option::Option<super::resources::FeedItemTarget>,
    /// The feed mapping referenced in the query.
    #[prost(message, optional, tag = "58")]
    pub feed_mapping: ::std::option::Option<super::resources::FeedMapping>,
    /// The feed placeholder view referenced in the query.
    #[prost(message, optional, tag = "97")]
    pub feed_placeholder_view: ::std::option::Option<super::resources::FeedPlaceholderView>,
    /// The gender view referenced in the query.
    #[prost(message, optional, tag = "40")]
    pub gender_view: ::std::option::Option<super::resources::GenderView>,
    /// The geo target constant referenced in the query.
    #[prost(message, optional, tag = "23")]
    pub geo_target_constant: ::std::option::Option<super::resources::GeoTargetConstant>,
    /// The geographic view referenced in the query.
    #[prost(message, optional, tag = "125")]
    pub geographic_view: ::std::option::Option<super::resources::GeographicView>,
    /// The group placement view referenced in the query.
    #[prost(message, optional, tag = "119")]
    pub group_placement_view: ::std::option::Option<super::resources::GroupPlacementView>,
    /// The hotel group view referenced in the query.
    #[prost(message, optional, tag = "51")]
    pub hotel_group_view: ::std::option::Option<super::resources::HotelGroupView>,
    /// The hotel performance view referenced in the query.
    #[prost(message, optional, tag = "71")]
    pub hotel_performance_view: ::std::option::Option<super::resources::HotelPerformanceView>,
    /// The keyword view referenced in the query.
    #[prost(message, optional, tag = "21")]
    pub keyword_view: ::std::option::Option<super::resources::KeywordView>,
    /// The keyword plan referenced in the query.
    #[prost(message, optional, tag = "32")]
    pub keyword_plan: ::std::option::Option<super::resources::KeywordPlan>,
    /// The keyword plan campaign referenced in the query.
    #[prost(message, optional, tag = "33")]
    pub keyword_plan_campaign: ::std::option::Option<super::resources::KeywordPlanCampaign>,
    /// The keyword plan negative keyword referenced in the query.
    #[prost(message, optional, tag = "34")]
    pub keyword_plan_negative_keyword:
        ::std::option::Option<super::resources::KeywordPlanNegativeKeyword>,
    /// The keyword plan ad group referenced in the query.
    #[prost(message, optional, tag = "35")]
    pub keyword_plan_ad_group: ::std::option::Option<super::resources::KeywordPlanAdGroup>,
    /// The keyword plan keyword referenced in the query.
    #[prost(message, optional, tag = "36")]
    pub keyword_plan_keyword: ::std::option::Option<super::resources::KeywordPlanKeyword>,
    /// The label referenced in the query.
    #[prost(message, optional, tag = "52")]
    pub label: ::std::option::Option<super::resources::Label>,
    /// The landing page view referenced in the query.
    #[prost(message, optional, tag = "126")]
    pub landing_page_view: ::std::option::Option<super::resources::LandingPageView>,
    /// The language constant referenced in the query.
    #[prost(message, optional, tag = "55")]
    pub language_constant: ::std::option::Option<super::resources::LanguageConstant>,
    /// The location view referenced in the query.
    #[prost(message, optional, tag = "123")]
    pub location_view: ::std::option::Option<super::resources::LocationView>,
    /// The managed placement view referenced in the query.
    #[prost(message, optional, tag = "53")]
    pub managed_placement_view: ::std::option::Option<super::resources::ManagedPlacementView>,
    /// The media file referenced in the query.
    #[prost(message, optional, tag = "90")]
    pub media_file: ::std::option::Option<super::resources::MediaFile>,
    /// The mobile app category constant referenced in the query.
    #[prost(message, optional, tag = "87")]
    pub mobile_app_category_constant:
        ::std::option::Option<super::resources::MobileAppCategoryConstant>,
    /// The mobile device constant referenced in the query.
    #[prost(message, optional, tag = "98")]
    pub mobile_device_constant: ::std::option::Option<super::resources::MobileDeviceConstant>,
    /// The mutate job referenced in the query.
    #[prost(message, optional, tag = "127")]
    pub mutate_job: ::std::option::Option<super::resources::MutateJob>,
    /// The offline user data job referenced in the query.
    #[prost(message, optional, tag = "137")]
    pub offline_user_data_job: ::std::option::Option<super::resources::OfflineUserDataJob>,
    /// The operating system version constant referenced in the query.
    #[prost(message, optional, tag = "86")]
    pub operating_system_version_constant:
        ::std::option::Option<super::resources::OperatingSystemVersionConstant>,
    /// The paid organic search term view referenced in the query.
    #[prost(message, optional, tag = "129")]
    pub paid_organic_search_term_view:
        ::std::option::Option<super::resources::PaidOrganicSearchTermView>,
    /// The parental status view referenced in the query.
    #[prost(message, optional, tag = "45")]
    pub parental_status_view: ::std::option::Option<super::resources::ParentalStatusView>,
    /// The Product Bidding Category referenced in the query.
    #[prost(message, optional, tag = "109")]
    pub product_bidding_category_constant:
        ::std::option::Option<super::resources::ProductBiddingCategoryConstant>,
    /// The product group view referenced in the query.
    #[prost(message, optional, tag = "54")]
    pub product_group_view: ::std::option::Option<super::resources::ProductGroupView>,
    /// The recommendation referenced in the query.
    #[prost(message, optional, tag = "22")]
    pub recommendation: ::std::option::Option<super::resources::Recommendation>,
    /// The search term view referenced in the query.
    #[prost(message, optional, tag = "68")]
    pub search_term_view: ::std::option::Option<super::resources::SearchTermView>,
    /// The shared set referenced in the query.
    #[prost(message, optional, tag = "29")]
    pub shared_criterion: ::std::option::Option<super::resources::SharedCriterion>,
    /// The shared set referenced in the query.
    #[prost(message, optional, tag = "27")]
    pub shared_set: ::std::option::Option<super::resources::SharedSet>,
    /// The shopping performance view referenced in the query.
    #[prost(message, optional, tag = "117")]
    pub shopping_performance_view: ::std::option::Option<super::resources::ShoppingPerformanceView>,
    /// The topic view referenced in the query.
    #[prost(message, optional, tag = "44")]
    pub topic_view: ::std::option::Option<super::resources::TopicView>,
    /// The user interest referenced in the query.
    #[prost(message, optional, tag = "59")]
    pub user_interest: ::std::option::Option<super::resources::UserInterest>,
    /// The user list referenced in the query.
    #[prost(message, optional, tag = "38")]
    pub user_list: ::std::option::Option<super::resources::UserList>,
    /// The user location view referenced in the query.
    #[prost(message, optional, tag = "135")]
    pub user_location_view: ::std::option::Option<super::resources::UserLocationView>,
    /// The remarketing action referenced in the query.
    #[prost(message, optional, tag = "60")]
    pub remarketing_action: ::std::option::Option<super::resources::RemarketingAction>,
    /// The topic constant referenced in the query.
    #[prost(message, optional, tag = "31")]
    pub topic_constant: ::std::option::Option<super::resources::TopicConstant>,
    /// The video referenced in the query.
    #[prost(message, optional, tag = "39")]
    pub video: ::std::option::Option<super::resources::Video>,
    /// The metrics.
    #[prost(message, optional, tag = "4")]
    pub metrics: ::std::option::Option<super::common::Metrics>,
    /// The segments.
    #[prost(message, optional, tag = "102")]
    pub segments: ::std::option::Option<super::common::Segments>,
}
/// Request message for [GoogleAdsService.Mutate][google.ads.googleads.v3.services.GoogleAdsService.Mutate].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateGoogleAdsRequest {
    /// Required. The ID of the customer whose resources are being modified.
    #[prost(string, tag = "1")]
    pub customer_id: std::string::String,
    /// Required. The list of operations to perform on individual resources.
    #[prost(message, repeated, tag = "2")]
    pub mutate_operations: ::std::vec::Vec<MutateOperation>,
    /// If true, successful operations will be carried out and invalid
    /// operations will return errors. If false, all operations will be carried
    /// out in one transaction if and only if they are all valid.
    /// Default is false.
    #[prost(bool, tag = "3")]
    pub partial_failure: bool,
    /// If true, the request is validated but not executed. Only errors are
    /// returned, not results.
    #[prost(bool, tag = "4")]
    pub validate_only: bool,
}
/// Response message for [GoogleAdsService.Mutate][google.ads.googleads.v3.services.GoogleAdsService.Mutate].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateGoogleAdsResponse {
    /// Errors that pertain to operation failures in the partial failure mode.
    /// Returned only when partial_failure = true and all errors occur inside the
    /// operations. If any errors occur outside the operations (e.g., auth errors),
    /// we return an RPC level error.
    #[prost(message, optional, tag = "3")]
    pub partial_failure_error: ::std::option::Option<super::super::super::super::rpc::Status>,
    /// All responses for the mutate.
    #[prost(message, repeated, tag = "1")]
    pub mutate_operation_responses: ::std::vec::Vec<MutateOperationResponse>,
}
/// A single operation (create, update, remove) on a resource.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateOperation {
    /// The mutate operation.
    #[prost(
        oneof = "mutate_operation::Operation",
        tags = "17, 1, 2, 18, 3, 19, 20, 21, 5, 49, 22, 23, 6, 7, 8, 13, 24, 25, 26, 27, 28, 10, 11, 12, 30, 31, 32, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 14, 15, 16"
    )]
    pub operation: ::std::option::Option<mutate_operation::Operation>,
}
pub mod mutate_operation {
    /// The mutate operation.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Operation {
        /// An ad group ad label mutate operation.
        #[prost(message, tag = "17")]
        AdGroupAdLabelOperation(super::AdGroupAdLabelOperation),
        /// An ad group ad mutate operation.
        #[prost(message, tag = "1")]
        AdGroupAdOperation(super::AdGroupAdOperation),
        /// An ad group bid modifier mutate operation.
        #[prost(message, tag = "2")]
        AdGroupBidModifierOperation(super::AdGroupBidModifierOperation),
        /// An ad group criterion label mutate operation.
        #[prost(message, tag = "18")]
        AdGroupCriterionLabelOperation(super::AdGroupCriterionLabelOperation),
        /// An ad group criterion mutate operation.
        #[prost(message, tag = "3")]
        AdGroupCriterionOperation(super::AdGroupCriterionOperation),
        /// An ad group extension setting mutate operation.
        #[prost(message, tag = "19")]
        AdGroupExtensionSettingOperation(super::AdGroupExtensionSettingOperation),
        /// An ad group feed mutate operation.
        #[prost(message, tag = "20")]
        AdGroupFeedOperation(super::AdGroupFeedOperation),
        /// An ad group label mutate operation.
        #[prost(message, tag = "21")]
        AdGroupLabelOperation(super::AdGroupLabelOperation),
        /// An ad group mutate operation.
        #[prost(message, tag = "5")]
        AdGroupOperation(super::AdGroupOperation),
        /// An ad mutate operation.
        #[prost(message, tag = "49")]
        AdOperation(super::AdOperation),
        /// An ad parameter mutate operation.
        #[prost(message, tag = "22")]
        AdParameterOperation(super::AdParameterOperation),
        /// An asset mutate operation.
        #[prost(message, tag = "23")]
        AssetOperation(super::AssetOperation),
        /// A bidding strategy mutate operation.
        #[prost(message, tag = "6")]
        BiddingStrategyOperation(super::BiddingStrategyOperation),
        /// A campaign bid modifier mutate operation.
        #[prost(message, tag = "7")]
        CampaignBidModifierOperation(super::CampaignBidModifierOperation),
        /// A campaign budget mutate operation.
        #[prost(message, tag = "8")]
        CampaignBudgetOperation(super::CampaignBudgetOperation),
        /// A campaign criterion mutate operation.
        #[prost(message, tag = "13")]
        CampaignCriterionOperation(super::CampaignCriterionOperation),
        /// A campaign draft mutate operation.
        #[prost(message, tag = "24")]
        CampaignDraftOperation(super::CampaignDraftOperation),
        /// A campaign experiment mutate operation.
        #[prost(message, tag = "25")]
        CampaignExperimentOperation(super::CampaignExperimentOperation),
        /// A campaign extension setting mutate operation.
        #[prost(message, tag = "26")]
        CampaignExtensionSettingOperation(super::CampaignExtensionSettingOperation),
        /// A campaign feed mutate operation.
        #[prost(message, tag = "27")]
        CampaignFeedOperation(super::CampaignFeedOperation),
        /// A campaign label mutate operation.
        #[prost(message, tag = "28")]
        CampaignLabelOperation(super::CampaignLabelOperation),
        /// A campaign mutate operation.
        #[prost(message, tag = "10")]
        CampaignOperation(super::CampaignOperation),
        /// A campaign shared set mutate operation.
        #[prost(message, tag = "11")]
        CampaignSharedSetOperation(super::CampaignSharedSetOperation),
        /// A conversion action mutate operation.
        #[prost(message, tag = "12")]
        ConversionActionOperation(super::ConversionActionOperation),
        /// A customer extension setting mutate operation.
        #[prost(message, tag = "30")]
        CustomerExtensionSettingOperation(super::CustomerExtensionSettingOperation),
        /// A customer feed mutate operation.
        #[prost(message, tag = "31")]
        CustomerFeedOperation(super::CustomerFeedOperation),
        /// A customer label mutate operation.
        #[prost(message, tag = "32")]
        CustomerLabelOperation(super::CustomerLabelOperation),
        /// A customer negative criterion mutate operation.
        #[prost(message, tag = "34")]
        CustomerNegativeCriterionOperation(super::CustomerNegativeCriterionOperation),
        /// A customer mutate operation.
        #[prost(message, tag = "35")]
        CustomerOperation(super::CustomerOperation),
        /// An extension feed item mutate operation.
        #[prost(message, tag = "36")]
        ExtensionFeedItemOperation(super::ExtensionFeedItemOperation),
        /// A feed item mutate operation.
        #[prost(message, tag = "37")]
        FeedItemOperation(super::FeedItemOperation),
        /// A feed item target mutate operation.
        #[prost(message, tag = "38")]
        FeedItemTargetOperation(super::FeedItemTargetOperation),
        /// A feed mapping mutate operation.
        #[prost(message, tag = "39")]
        FeedMappingOperation(super::FeedMappingOperation),
        /// A feed mutate operation.
        #[prost(message, tag = "40")]
        FeedOperation(super::FeedOperation),
        /// A label mutate operation.
        #[prost(message, tag = "41")]
        LabelOperation(super::LabelOperation),
        /// A media file mutate operation.
        #[prost(message, tag = "42")]
        MediaFileOperation(super::MediaFileOperation),
        /// A remarketing action mutate operation.
        #[prost(message, tag = "43")]
        RemarketingActionOperation(super::RemarketingActionOperation),
        /// A shared criterion mutate operation.
        #[prost(message, tag = "14")]
        SharedCriterionOperation(super::SharedCriterionOperation),
        /// A shared set mutate operation.
        #[prost(message, tag = "15")]
        SharedSetOperation(super::SharedSetOperation),
        /// A user list mutate operation.
        #[prost(message, tag = "16")]
        UserListOperation(super::UserListOperation),
    }
}
/// Response message for the resource mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateOperationResponse {
    /// The mutate response.
    #[prost(
        oneof = "mutate_operation_response::Response",
        tags = "17, 1, 2, 18, 3, 19, 20, 21, 5, 22, 49, 23, 6, 7, 8, 13, 24, 25, 26, 27, 28, 10, 11, 12, 30, 31, 32, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 14, 15, 16"
    )]
    pub response: ::std::option::Option<mutate_operation_response::Response>,
}
pub mod mutate_operation_response {
    /// The mutate response.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Response {
        /// The result for the ad group ad label mutate.
        #[prost(message, tag = "17")]
        AdGroupAdLabelResult(super::MutateAdGroupAdLabelResult),
        /// The result for the ad group ad mutate.
        #[prost(message, tag = "1")]
        AdGroupAdResult(super::MutateAdGroupAdResult),
        /// The result for the ad group bid modifier mutate.
        #[prost(message, tag = "2")]
        AdGroupBidModifierResult(super::MutateAdGroupBidModifierResult),
        /// The result for the ad group criterion label mutate.
        #[prost(message, tag = "18")]
        AdGroupCriterionLabelResult(super::MutateAdGroupCriterionLabelResult),
        /// The result for the ad group criterion mutate.
        #[prost(message, tag = "3")]
        AdGroupCriterionResult(super::MutateAdGroupCriterionResult),
        /// The result for the ad group extension setting mutate.
        #[prost(message, tag = "19")]
        AdGroupExtensionSettingResult(super::MutateAdGroupExtensionSettingResult),
        /// The result for the ad group feed mutate.
        #[prost(message, tag = "20")]
        AdGroupFeedResult(super::MutateAdGroupFeedResult),
        /// The result for the ad group label mutate.
        #[prost(message, tag = "21")]
        AdGroupLabelResult(super::MutateAdGroupLabelResult),
        /// The result for the ad group mutate.
        #[prost(message, tag = "5")]
        AdGroupResult(super::MutateAdGroupResult),
        /// The result for the ad parameter mutate.
        #[prost(message, tag = "22")]
        AdParameterResult(super::MutateAdParameterResult),
        /// The result for the ad mutate.
        #[prost(message, tag = "49")]
        AdResult(super::MutateAdResult),
        /// The result for the asset mutate.
        #[prost(message, tag = "23")]
        AssetResult(super::MutateAssetResult),
        /// The result for the bidding strategy mutate.
        #[prost(message, tag = "6")]
        BiddingStrategyResult(super::MutateBiddingStrategyResult),
        /// The result for the campaign bid modifier mutate.
        #[prost(message, tag = "7")]
        CampaignBidModifierResult(super::MutateCampaignBidModifierResult),
        /// The result for the campaign budget mutate.
        #[prost(message, tag = "8")]
        CampaignBudgetResult(super::MutateCampaignBudgetResult),
        /// The result for the campaign criterion mutate.
        #[prost(message, tag = "13")]
        CampaignCriterionResult(super::MutateCampaignCriterionResult),
        /// The result for the campaign draft mutate.
        #[prost(message, tag = "24")]
        CampaignDraftResult(super::MutateCampaignDraftResult),
        /// The result for the campaign experiment mutate.
        #[prost(message, tag = "25")]
        CampaignExperimentResult(super::MutateCampaignExperimentResult),
        /// The result for the campaign extension setting mutate.
        #[prost(message, tag = "26")]
        CampaignExtensionSettingResult(super::MutateCampaignExtensionSettingResult),
        /// The result for the campaign feed mutate.
        #[prost(message, tag = "27")]
        CampaignFeedResult(super::MutateCampaignFeedResult),
        /// The result for the campaign label mutate.
        #[prost(message, tag = "28")]
        CampaignLabelResult(super::MutateCampaignLabelResult),
        /// The result for the campaign mutate.
        #[prost(message, tag = "10")]
        CampaignResult(super::MutateCampaignResult),
        /// The result for the campaign shared set mutate.
        #[prost(message, tag = "11")]
        CampaignSharedSetResult(super::MutateCampaignSharedSetResult),
        /// The result for the conversion action mutate.
        #[prost(message, tag = "12")]
        ConversionActionResult(super::MutateConversionActionResult),
        /// The result for the customer extension setting mutate.
        #[prost(message, tag = "30")]
        CustomerExtensionSettingResult(super::MutateCustomerExtensionSettingResult),
        /// The result for the customer feed mutate.
        #[prost(message, tag = "31")]
        CustomerFeedResult(super::MutateCustomerFeedResult),
        /// The result for the customer label mutate.
        #[prost(message, tag = "32")]
        CustomerLabelResult(super::MutateCustomerLabelResult),
        /// The result for the customer negative criterion mutate.
        #[prost(message, tag = "34")]
        CustomerNegativeCriterionResult(super::MutateCustomerNegativeCriteriaResult),
        /// The result for the customer mutate.
        #[prost(message, tag = "35")]
        CustomerResult(super::MutateCustomerResult),
        /// The result for the extension feed item mutate.
        #[prost(message, tag = "36")]
        ExtensionFeedItemResult(super::MutateExtensionFeedItemResult),
        /// The result for the feed item mutate.
        #[prost(message, tag = "37")]
        FeedItemResult(super::MutateFeedItemResult),
        /// The result for the feed item target mutate.
        #[prost(message, tag = "38")]
        FeedItemTargetResult(super::MutateFeedItemTargetResult),
        /// The result for the feed mapping mutate.
        #[prost(message, tag = "39")]
        FeedMappingResult(super::MutateFeedMappingResult),
        /// The result for the feed mutate.
        #[prost(message, tag = "40")]
        FeedResult(super::MutateFeedResult),
        /// The result for the label mutate.
        #[prost(message, tag = "41")]
        LabelResult(super::MutateLabelResult),
        /// The result for the media file mutate.
        #[prost(message, tag = "42")]
        MediaFileResult(super::MutateMediaFileResult),
        /// The result for the remarketing action mutate.
        #[prost(message, tag = "43")]
        RemarketingActionResult(super::MutateRemarketingActionResult),
        /// The result for the shared criterion mutate.
        #[prost(message, tag = "14")]
        SharedCriterionResult(super::MutateSharedCriterionResult),
        /// The result for the shared set mutate.
        #[prost(message, tag = "15")]
        SharedSetResult(super::MutateSharedSetResult),
        /// The result for the user list mutate.
        #[prost(message, tag = "16")]
        UserListResult(super::MutateUserListResult),
    }
}
#[doc = r" Generated client implementations."]
pub mod google_ads_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Service to fetch data and metrics across resources."]
    pub struct GoogleAdsServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> GoogleAdsServiceClient<T>
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
        #[doc = " Returns all rows that match the search query."]
        pub async fn search(
            &mut self,
            request: impl tonic::IntoRequest<super::SearchGoogleAdsRequest>,
        ) -> Result<tonic::Response<super::SearchGoogleAdsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ads.googleads.v3.services.GoogleAdsService/Search",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns all rows that match the search stream query."]
        pub async fn search_stream(
            &mut self,
            request: impl tonic::IntoRequest<super::SearchGoogleAdsStreamRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::SearchGoogleAdsStreamResponse>>,
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
                "/google.ads.googleads.v3.services.GoogleAdsService/SearchStream",
            );
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        #[doc = " Creates, updates, or removes resources. This method supports atomic"]
        #[doc = " transactions with multiple types of resources. For example, you can"]
        #[doc = " atomically create a campaign and a campaign budget, or perform up to"]
        #[doc = " thousands of mutates atomically."]
        #[doc = ""]
        #[doc = " This method is essentially a wrapper around a series of mutate methods. The"]
        #[doc = " only features it offers over calling those methods directly are:"]
        #[doc = ""]
        #[doc = " - Atomic transactions"]
        #[doc = " - Temp resource names (described below)"]
        #[doc = " - Somewhat reduced latency over making a series of mutate calls"]
        #[doc = ""]
        #[doc = " Note: Only resources that support atomic transactions are included, so this"]
        #[doc = " method can't replace all calls to individual services."]
        #[doc = ""]
        #[doc = " ## Atomic Transaction Benefits"]
        #[doc = ""]
        #[doc = " Atomicity makes error handling much easier. If you're making a series of"]
        #[doc = " changes and one fails, it can leave your account in an inconsistent state."]
        #[doc = " With atomicity, you either reach the desired state directly, or the request"]
        #[doc = " fails and you can retry."]
        #[doc = ""]
        #[doc = " ## Temp Resource Names"]
        #[doc = ""]
        #[doc = " Temp resource names are a special type of resource name used to create a"]
        #[doc = " resource and reference that resource in the same request. For example, if a"]
        #[doc = " campaign budget is created with `resource_name` equal to"]
        #[doc = " `customers/123/campaignBudgets/-1`, that resource name can be reused in"]
        #[doc = " the `Campaign.budget` field in the same request. That way, the two"]
        #[doc = " resources are created and linked atomically."]
        #[doc = ""]
        #[doc = " To create a temp resource name, put a negative number in the part of the"]
        #[doc = " name that the server would normally allocate."]
        #[doc = ""]
        #[doc = " Note:"]
        #[doc = ""]
        #[doc = " - Resources must be created with a temp name before the name can be reused."]
        #[doc = "   For example, the previous CampaignBudget+Campaign example would fail if"]
        #[doc = "   the mutate order was reversed."]
        #[doc = " - Temp names are not remembered across requests."]
        #[doc = " - There's no limit to the number of temp names in a request."]
        #[doc = " - Each temp name must use a unique negative number, even if the resource"]
        #[doc = "   types differ."]
        #[doc = ""]
        #[doc = " ## Latency"]
        #[doc = ""]
        #[doc = " It's important to group mutates by resource type or the request may time"]
        #[doc = " out and fail. Latency is roughly equal to a series of calls to individual"]
        #[doc = " mutate methods, where each change in resource type is a new call. For"]
        #[doc = " example, mutating 10 campaigns then 10 ad groups is like 2 calls, while"]
        #[doc = " mutating 1 campaign, 1 ad group, 1 campaign, 1 ad group is like 4 calls."]
        pub async fn mutate(
            &mut self,
            request: impl tonic::IntoRequest<super::MutateGoogleAdsRequest>,
        ) -> Result<tonic::Response<super::MutateGoogleAdsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ads.googleads.v3.services.GoogleAdsService/Mutate",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for GoogleAdsServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for GoogleAdsServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "GoogleAdsServiceClient {{ ... }}")
        }
    }
}
/// Request message for [GroupPlacementViewService.GetGroupPlacementView][google.ads.googleads.v3.services.GroupPlacementViewService.GetGroupPlacementView].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetGroupPlacementViewRequest {
    /// Required. The resource name of the Group Placement view to fetch.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
#[doc = r" Generated client implementations."]
pub mod group_placement_view_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Service to fetch Group Placement views."]
    pub struct GroupPlacementViewServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> GroupPlacementViewServiceClient<T>
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
        #[doc = " Returns the requested Group Placement view in full detail."]
        pub async fn get_group_placement_view(
            &mut self,
            request: impl tonic::IntoRequest<super::GetGroupPlacementViewRequest>,
        ) -> Result<tonic::Response<super::super::resources::GroupPlacementView>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ads.googleads.v3.services.GroupPlacementViewService/GetGroupPlacementView",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for GroupPlacementViewServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for GroupPlacementViewServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "GroupPlacementViewServiceClient {{ ... }}")
        }
    }
}
/// Request message for [HotelGroupViewService.GetHotelGroupView][google.ads.googleads.v3.services.HotelGroupViewService.GetHotelGroupView].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetHotelGroupViewRequest {
    /// Required. Resource name of the Hotel Group View to fetch.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
#[doc = r" Generated client implementations."]
pub mod hotel_group_view_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Service to manage Hotel Group Views."]
    pub struct HotelGroupViewServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> HotelGroupViewServiceClient<T>
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
        #[doc = " Returns the requested Hotel Group View in full detail."]
        pub async fn get_hotel_group_view(
            &mut self,
            request: impl tonic::IntoRequest<super::GetHotelGroupViewRequest>,
        ) -> Result<tonic::Response<super::super::resources::HotelGroupView>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ads.googleads.v3.services.HotelGroupViewService/GetHotelGroupView",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for HotelGroupViewServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for HotelGroupViewServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "HotelGroupViewServiceClient {{ ... }}")
        }
    }
}
/// Request message for [HotelPerformanceViewService.GetHotelPerformanceView][google.ads.googleads.v3.services.HotelPerformanceViewService.GetHotelPerformanceView].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetHotelPerformanceViewRequest {
    /// Required. Resource name of the Hotel Performance View to fetch.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
#[doc = r" Generated client implementations."]
pub mod hotel_performance_view_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Service to manage Hotel Performance Views."]
    pub struct HotelPerformanceViewServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> HotelPerformanceViewServiceClient<T>
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
        #[doc = " Returns the requested Hotel Performance View in full detail."]
        pub async fn get_hotel_performance_view(
            &mut self,
            request: impl tonic::IntoRequest<super::GetHotelPerformanceViewRequest>,
        ) -> Result<tonic::Response<super::super::resources::HotelPerformanceView>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ( "/google.ads.googleads.v3.services.HotelPerformanceViewService/GetHotelPerformanceView" ) ;
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for HotelPerformanceViewServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for HotelPerformanceViewServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "HotelPerformanceViewServiceClient {{ ... }}")
        }
    }
}
/// Request message for fetching the invoices of a given billing setup that were
/// issued during a given month.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListInvoicesRequest {
    /// Required. The ID of the customer to fetch invoices for.
    #[prost(string, tag = "1")]
    pub customer_id: std::string::String,
    /// Required. The billing setup resource name of the requested invoices.
    ///
    /// `customers/{customer_id}/billingSetups/{billing_setup_id}`
    #[prost(string, tag = "2")]
    pub billing_setup: std::string::String,
    /// Required. The issue year to retrieve invoices, in yyyy format. Only
    /// invoices issued in 2019 or later can be retrieved.
    #[prost(string, tag = "3")]
    pub issue_year: std::string::String,
    /// Required. The issue month to retrieve invoices.
    #[prost(
        enumeration = "super::enums::month_of_year_enum::MonthOfYear",
        tag = "4"
    )]
    pub issue_month: i32,
}
/// Response message for [InvoiceService.ListInvoices][google.ads.googleads.v3.services.InvoiceService.ListInvoices].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListInvoicesResponse {
    /// The list of invoices that match the billing setup and time period.
    #[prost(message, repeated, tag = "1")]
    pub invoices: ::std::vec::Vec<super::resources::Invoice>,
}
#[doc = r" Generated client implementations."]
pub mod invoice_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " A service to fetch invoices issued for a billing setup during a given month."]
    pub struct InvoiceServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> InvoiceServiceClient<T>
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
        #[doc = " Returns all invoices associated with a billing setup, for a given month."]
        pub async fn list_invoices(
            &mut self,
            request: impl tonic::IntoRequest<super::ListInvoicesRequest>,
        ) -> Result<tonic::Response<super::ListInvoicesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ads.googleads.v3.services.InvoiceService/ListInvoices",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for InvoiceServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for InvoiceServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "InvoiceServiceClient {{ ... }}")
        }
    }
}
/// Request message for [KeywordPlanAdGroupService.GetKeywordPlanAdGroup][google.ads.googleads.v3.services.KeywordPlanAdGroupService.GetKeywordPlanAdGroup].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetKeywordPlanAdGroupRequest {
    /// Required. The resource name of the Keyword Plan ad group to fetch.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
/// Request message for [KeywordPlanAdGroupService.MutateKeywordPlanAdGroups][google.ads.googleads.v3.services.KeywordPlanAdGroupService.MutateKeywordPlanAdGroups].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateKeywordPlanAdGroupsRequest {
    /// Required. The ID of the customer whose Keyword Plan ad groups are being modified.
    #[prost(string, tag = "1")]
    pub customer_id: std::string::String,
    /// Required. The list of operations to perform on individual Keyword Plan ad groups.
    #[prost(message, repeated, tag = "2")]
    pub operations: ::std::vec::Vec<KeywordPlanAdGroupOperation>,
    /// If true, successful operations will be carried out and invalid
    /// operations will return errors. If false, all operations will be carried
    /// out in one transaction if and only if they are all valid.
    /// Default is false.
    #[prost(bool, tag = "3")]
    pub partial_failure: bool,
    /// If true, the request is validated but not executed. Only errors are
    /// returned, not results.
    #[prost(bool, tag = "4")]
    pub validate_only: bool,
}
/// A single operation (create, update, remove) on a Keyword Plan ad group.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeywordPlanAdGroupOperation {
    /// The FieldMask that determines which resource fields are modified in an
    /// update.
    #[prost(message, optional, tag = "4")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
    /// The mutate operation.
    #[prost(oneof = "keyword_plan_ad_group_operation::Operation", tags = "1, 2, 3")]
    pub operation: ::std::option::Option<keyword_plan_ad_group_operation::Operation>,
}
pub mod keyword_plan_ad_group_operation {
    /// The mutate operation.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Operation {
        /// Create operation: No resource name is expected for the new Keyword Plan
        /// ad group.
        #[prost(message, tag = "1")]
        Create(super::super::resources::KeywordPlanAdGroup),
        /// Update operation: The Keyword Plan ad group is expected to have a valid
        /// resource name.
        #[prost(message, tag = "2")]
        Update(super::super::resources::KeywordPlanAdGroup),
        /// Remove operation: A resource name for the removed Keyword Plan ad group
        /// is expected, in this format:
        ///
        /// `customers/{customer_id}/keywordPlanAdGroups/{kp_ad_group_id}`
        #[prost(string, tag = "3")]
        Remove(std::string::String),
    }
}
/// Response message for a Keyword Plan ad group mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateKeywordPlanAdGroupsResponse {
    /// Errors that pertain to operation failures in the partial failure mode.
    /// Returned only when partial_failure = true and all errors occur inside the
    /// operations. If any errors occur outside the operations (e.g. auth errors),
    /// we return an RPC level error.
    #[prost(message, optional, tag = "3")]
    pub partial_failure_error: ::std::option::Option<super::super::super::super::rpc::Status>,
    /// All results for the mutate. The order of the results is determined by the
    /// order of the keywords in the original request.
    #[prost(message, repeated, tag = "2")]
    pub results: ::std::vec::Vec<MutateKeywordPlanAdGroupResult>,
}
/// The result for the Keyword Plan ad group mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateKeywordPlanAdGroupResult {
    /// Returned for successful operations.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
#[doc = r" Generated client implementations."]
pub mod keyword_plan_ad_group_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Service to manage Keyword Plan ad groups."]
    pub struct KeywordPlanAdGroupServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> KeywordPlanAdGroupServiceClient<T>
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
        #[doc = " Returns the requested Keyword Plan ad group in full detail."]
        pub async fn get_keyword_plan_ad_group(
            &mut self,
            request: impl tonic::IntoRequest<super::GetKeywordPlanAdGroupRequest>,
        ) -> Result<tonic::Response<super::super::resources::KeywordPlanAdGroup>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ads.googleads.v3.services.KeywordPlanAdGroupService/GetKeywordPlanAdGroup",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates, updates, or removes Keyword Plan ad groups. Operation statuses are"]
        #[doc = " returned."]
        pub async fn mutate_keyword_plan_ad_groups(
            &mut self,
            request: impl tonic::IntoRequest<super::MutateKeywordPlanAdGroupsRequest>,
        ) -> Result<tonic::Response<super::MutateKeywordPlanAdGroupsResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ( "/google.ads.googleads.v3.services.KeywordPlanAdGroupService/MutateKeywordPlanAdGroups" ) ;
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for KeywordPlanAdGroupServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for KeywordPlanAdGroupServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "KeywordPlanAdGroupServiceClient {{ ... }}")
        }
    }
}
/// Request message for [KeywordPlanCampaignService.GetKeywordPlanCampaign][google.ads.googleads.v3.services.KeywordPlanCampaignService.GetKeywordPlanCampaign].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetKeywordPlanCampaignRequest {
    /// Required. The resource name of the Keyword Plan campaign to fetch.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
/// Request message for
/// [KeywordPlanCampaignService.MutateKeywordPlanCampaigns][google.ads.googleads.v3.services.KeywordPlanCampaignService.MutateKeywordPlanCampaigns].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateKeywordPlanCampaignsRequest {
    /// Required. The ID of the customer whose Keyword Plan campaigns are being modified.
    #[prost(string, tag = "1")]
    pub customer_id: std::string::String,
    /// Required. The list of operations to perform on individual Keyword Plan campaigns.
    #[prost(message, repeated, tag = "2")]
    pub operations: ::std::vec::Vec<KeywordPlanCampaignOperation>,
    /// If true, successful operations will be carried out and invalid
    /// operations will return errors. If false, all operations will be carried
    /// out in one transaction if and only if they are all valid.
    /// Default is false.
    #[prost(bool, tag = "3")]
    pub partial_failure: bool,
    /// If true, the request is validated but not executed. Only errors are
    /// returned, not results.
    #[prost(bool, tag = "4")]
    pub validate_only: bool,
}
/// A single operation (create, update, remove) on a Keyword Plan campaign.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeywordPlanCampaignOperation {
    /// The FieldMask that determines which resource fields are modified in an
    /// update.
    #[prost(message, optional, tag = "4")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
    /// The mutate operation.
    #[prost(oneof = "keyword_plan_campaign_operation::Operation", tags = "1, 2, 3")]
    pub operation: ::std::option::Option<keyword_plan_campaign_operation::Operation>,
}
pub mod keyword_plan_campaign_operation {
    /// The mutate operation.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Operation {
        /// Create operation: No resource name is expected for the new Keyword Plan
        /// campaign.
        #[prost(message, tag = "1")]
        Create(super::super::resources::KeywordPlanCampaign),
        /// Update operation: The Keyword Plan campaign is expected to have a valid
        /// resource name.
        #[prost(message, tag = "2")]
        Update(super::super::resources::KeywordPlanCampaign),
        /// Remove operation: A resource name for the removed Keyword Plan campaign
        /// is expected, in this format:
        ///
        /// `customers/{customer_id}/keywordPlanCampaigns/{keywordPlan_campaign_id}`
        #[prost(string, tag = "3")]
        Remove(std::string::String),
    }
}
/// Response message for a Keyword Plan campaign mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateKeywordPlanCampaignsResponse {
    /// Errors that pertain to operation failures in the partial failure mode.
    /// Returned only when partial_failure = true and all errors occur inside the
    /// operations. If any errors occur outside the operations (e.g. auth errors),
    /// we return an RPC level error.
    #[prost(message, optional, tag = "3")]
    pub partial_failure_error: ::std::option::Option<super::super::super::super::rpc::Status>,
    /// All results for the mutate.
    #[prost(message, repeated, tag = "2")]
    pub results: ::std::vec::Vec<MutateKeywordPlanCampaignResult>,
}
/// The result for the Keyword Plan campaign mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateKeywordPlanCampaignResult {
    /// Returned for successful operations.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
#[doc = r" Generated client implementations."]
pub mod keyword_plan_campaign_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Service to manage Keyword Plan campaigns."]
    pub struct KeywordPlanCampaignServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> KeywordPlanCampaignServiceClient<T>
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
        #[doc = " Returns the requested Keyword Plan campaign in full detail."]
        pub async fn get_keyword_plan_campaign(
            &mut self,
            request: impl tonic::IntoRequest<super::GetKeywordPlanCampaignRequest>,
        ) -> Result<tonic::Response<super::super::resources::KeywordPlanCampaign>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ( "/google.ads.googleads.v3.services.KeywordPlanCampaignService/GetKeywordPlanCampaign" ) ;
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates, updates, or removes Keyword Plan campaigns. Operation statuses are"]
        #[doc = " returned."]
        pub async fn mutate_keyword_plan_campaigns(
            &mut self,
            request: impl tonic::IntoRequest<super::MutateKeywordPlanCampaignsRequest>,
        ) -> Result<tonic::Response<super::MutateKeywordPlanCampaignsResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ( "/google.ads.googleads.v3.services.KeywordPlanCampaignService/MutateKeywordPlanCampaigns" ) ;
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for KeywordPlanCampaignServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for KeywordPlanCampaignServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "KeywordPlanCampaignServiceClient {{ ... }}")
        }
    }
}
/// Request message for [KeywordPlanIdeaService.GenerateKeywordIdeas][google.ads.googleads.v3.services.KeywordPlanIdeaService.GenerateKeywordIdeas].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateKeywordIdeasRequest {
    /// The ID of the customer with the recommendation.
    #[prost(string, tag = "1")]
    pub customer_id: std::string::String,
    /// Required. The resource name of the language to target.
    /// Required
    #[prost(message, optional, tag = "7")]
    pub language: ::std::option::Option<::std::string::String>,
    /// The resource names of the location to target.
    /// Max 10
    #[prost(message, repeated, tag = "8")]
    pub geo_target_constants: ::std::vec::Vec<::std::string::String>,
    /// Targeting network.
    #[prost(
        enumeration = "super::enums::keyword_plan_network_enum::KeywordPlanNetwork",
        tag = "9"
    )]
    pub keyword_plan_network: i32,
    /// The type of seed to generate keyword ideas.
    #[prost(oneof = "generate_keyword_ideas_request::Seed", tags = "2, 3, 5")]
    pub seed: ::std::option::Option<generate_keyword_ideas_request::Seed>,
}
pub mod generate_keyword_ideas_request {
    /// The type of seed to generate keyword ideas.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Seed {
        /// A Keyword and a specific Url to generate ideas from
        /// e.g. cars, www.example.com/cars.
        #[prost(message, tag = "2")]
        KeywordAndUrlSeed(super::KeywordAndUrlSeed),
        /// A Keyword or phrase to generate ideas from, e.g. cars.
        #[prost(message, tag = "3")]
        KeywordSeed(super::KeywordSeed),
        /// A specific url to generate ideas from, e.g. www.example.com/cars.
        #[prost(message, tag = "5")]
        UrlSeed(super::UrlSeed),
    }
}
/// Keyword And Url Seed
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeywordAndUrlSeed {
    /// The URL to crawl in order to generate keyword ideas.
    #[prost(message, optional, tag = "1")]
    pub url: ::std::option::Option<::std::string::String>,
    /// Requires at least one keyword.
    #[prost(message, repeated, tag = "2")]
    pub keywords: ::std::vec::Vec<::std::string::String>,
}
/// Keyword Seed
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeywordSeed {
    /// Requires at least one keyword.
    #[prost(message, repeated, tag = "1")]
    pub keywords: ::std::vec::Vec<::std::string::String>,
}
/// Url Seed
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UrlSeed {
    /// The URL to crawl in order to generate keyword ideas.
    #[prost(message, optional, tag = "1")]
    pub url: ::std::option::Option<::std::string::String>,
}
/// Response message for [KeywordPlanIdeaService.GenerateKeywordIdeas][google.ads.googleads.v3.services.KeywordPlanIdeaService.GenerateKeywordIdeas].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateKeywordIdeaResponse {
    /// Results of generating keyword ideas.
    #[prost(message, repeated, tag = "1")]
    pub results: ::std::vec::Vec<GenerateKeywordIdeaResult>,
}
/// The result of generating keyword ideas.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateKeywordIdeaResult {
    /// Text of the keyword idea.
    /// As in Keyword Plan historical metrics, this text may not be an actual
    /// keyword, but the canonical form of multiple keywords.
    /// See KeywordPlanKeywordHistoricalMetrics message in KeywordPlanService.
    #[prost(message, optional, tag = "2")]
    pub text: ::std::option::Option<::std::string::String>,
    /// The historical metrics for the keyword
    #[prost(message, optional, tag = "3")]
    pub keyword_idea_metrics: ::std::option::Option<super::common::KeywordPlanHistoricalMetrics>,
}
#[doc = r" Generated client implementations."]
pub mod keyword_plan_idea_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Service to generate keyword ideas."]
    pub struct KeywordPlanIdeaServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> KeywordPlanIdeaServiceClient<T>
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
        #[doc = " Returns a list of keyword ideas."]
        pub async fn generate_keyword_ideas(
            &mut self,
            request: impl tonic::IntoRequest<super::GenerateKeywordIdeasRequest>,
        ) -> Result<tonic::Response<super::GenerateKeywordIdeaResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ads.googleads.v3.services.KeywordPlanIdeaService/GenerateKeywordIdeas",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for KeywordPlanIdeaServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for KeywordPlanIdeaServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "KeywordPlanIdeaServiceClient {{ ... }}")
        }
    }
}
/// Request message for [KeywordPlanKeywordService.GetKeywordPlanKeyword][google.ads.googleads.v3.services.KeywordPlanKeywordService.GetKeywordPlanKeyword].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetKeywordPlanKeywordRequest {
    /// Required. The resource name of the ad group keyword to fetch.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
/// Request message for [KeywordPlanKeywordService.MutateKeywordPlanKeywords][google.ads.googleads.v3.services.KeywordPlanKeywordService.MutateKeywordPlanKeywords].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateKeywordPlanKeywordsRequest {
    /// Required. The ID of the customer whose Keyword Plan keywords are being modified.
    #[prost(string, tag = "1")]
    pub customer_id: std::string::String,
    /// Required. The list of operations to perform on individual Keyword Plan keywords.
    #[prost(message, repeated, tag = "2")]
    pub operations: ::std::vec::Vec<KeywordPlanKeywordOperation>,
    /// If true, successful operations will be carried out and invalid
    /// operations will return errors. If false, all operations will be carried
    /// out in one transaction if and only if they are all valid.
    /// Default is false.
    #[prost(bool, tag = "3")]
    pub partial_failure: bool,
    /// If true, the request is validated but not executed. Only errors are
    /// returned, not results.
    #[prost(bool, tag = "4")]
    pub validate_only: bool,
}
/// A single operation (create, update, remove) on a Keyword Plan keyword.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeywordPlanKeywordOperation {
    /// The FieldMask that determines which resource fields are modified in an
    /// update.
    #[prost(message, optional, tag = "4")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
    /// The mutate operation.
    #[prost(oneof = "keyword_plan_keyword_operation::Operation", tags = "1, 2, 3")]
    pub operation: ::std::option::Option<keyword_plan_keyword_operation::Operation>,
}
pub mod keyword_plan_keyword_operation {
    /// The mutate operation.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Operation {
        /// Create operation: No resource name is expected for the new Keyword Plan
        /// ad group keyword.
        #[prost(message, tag = "1")]
        Create(super::super::resources::KeywordPlanKeyword),
        /// Update operation: The Keyword Plan keyword is expected to have a valid
        /// resource name.
        #[prost(message, tag = "2")]
        Update(super::super::resources::KeywordPlanKeyword),
        /// Remove operation: A resource name for the removed Keyword Plan keyword is
        /// expected, in this format:
        ///
        /// `customers/{customer_id}/keywordPlanKeywords/{kp_ad_group_keyword_id}`
        #[prost(string, tag = "3")]
        Remove(std::string::String),
    }
}
/// Response message for a Keyword Plan keyword mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateKeywordPlanKeywordsResponse {
    /// Errors that pertain to operation failures in the partial failure mode.
    /// Returned only when partial_failure = true and all errors occur inside the
    /// operations. If any errors occur outside the operations (e.g. auth errors),
    /// we return an RPC level error.
    #[prost(message, optional, tag = "3")]
    pub partial_failure_error: ::std::option::Option<super::super::super::super::rpc::Status>,
    /// All results for the mutate.
    #[prost(message, repeated, tag = "2")]
    pub results: ::std::vec::Vec<MutateKeywordPlanKeywordResult>,
}
/// The result for the Keyword Plan keyword mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateKeywordPlanKeywordResult {
    /// Returned for successful operations.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
#[doc = r" Generated client implementations."]
pub mod keyword_plan_keyword_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Service to manage Keyword Plan ad group keywords."]
    pub struct KeywordPlanKeywordServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> KeywordPlanKeywordServiceClient<T>
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
        #[doc = " Returns the requested Keyword Plan keyword in full detail."]
        pub async fn get_keyword_plan_keyword(
            &mut self,
            request: impl tonic::IntoRequest<super::GetKeywordPlanKeywordRequest>,
        ) -> Result<tonic::Response<super::super::resources::KeywordPlanKeyword>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ads.googleads.v3.services.KeywordPlanKeywordService/GetKeywordPlanKeyword",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates, updates, or removes Keyword Plan keywords. Operation statuses are"]
        #[doc = " returned."]
        pub async fn mutate_keyword_plan_keywords(
            &mut self,
            request: impl tonic::IntoRequest<super::MutateKeywordPlanKeywordsRequest>,
        ) -> Result<tonic::Response<super::MutateKeywordPlanKeywordsResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ( "/google.ads.googleads.v3.services.KeywordPlanKeywordService/MutateKeywordPlanKeywords" ) ;
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for KeywordPlanKeywordServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for KeywordPlanKeywordServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "KeywordPlanKeywordServiceClient {{ ... }}")
        }
    }
}
/// Request message for
/// [KeywordPlanNegativeKeywordService.GetKeywordPlanNegativeKeyword][google.ads.googleads.v3.services.KeywordPlanNegativeKeywordService.GetKeywordPlanNegativeKeyword].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetKeywordPlanNegativeKeywordRequest {
    /// Required. The resource name of the plan to fetch.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
/// Request message for
/// [KeywordPlanNegativeKeywordService.MutateKeywordPlanNegativeKeywords][google.ads.googleads.v3.services.KeywordPlanNegativeKeywordService.MutateKeywordPlanNegativeKeywords].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateKeywordPlanNegativeKeywordsRequest {
    /// Required. The ID of the customer whose negative keywords are being modified.
    #[prost(string, tag = "1")]
    pub customer_id: std::string::String,
    /// Required. The list of operations to perform on individual Keyword Plan negative
    /// keywords.
    #[prost(message, repeated, tag = "2")]
    pub operations: ::std::vec::Vec<KeywordPlanNegativeKeywordOperation>,
    /// If true, successful operations will be carried out and invalid
    /// operations will return errors. If false, all operations will be carried
    /// out in one transaction if and only if they are all valid.
    /// Default is false.
    #[prost(bool, tag = "3")]
    pub partial_failure: bool,
    /// If true, the request is validated but not executed. Only errors are
    /// returned, not results.
    #[prost(bool, tag = "4")]
    pub validate_only: bool,
}
/// A single operation (create, update, remove) on a Keyword Plan negative
/// keyword.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeywordPlanNegativeKeywordOperation {
    /// The FieldMask that determines which resource fields are modified in an
    /// update.
    #[prost(message, optional, tag = "4")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
    /// The mutate operation.
    #[prost(
        oneof = "keyword_plan_negative_keyword_operation::Operation",
        tags = "1, 2, 3"
    )]
    pub operation: ::std::option::Option<keyword_plan_negative_keyword_operation::Operation>,
}
pub mod keyword_plan_negative_keyword_operation {
    /// The mutate operation.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Operation {
        /// Create operation: No resource name is expected for the new Keyword Plan
        /// negative keyword.
        #[prost(message, tag = "1")]
        Create(super::super::resources::KeywordPlanNegativeKeyword),
        /// Update operation: The Keyword Plan negative keyword expected to have a
        /// valid resource name.
        #[prost(message, tag = "2")]
        Update(super::super::resources::KeywordPlanNegativeKeyword),
        /// Remove operation: A resource name for the removed Keyword Plan negative
        /// keywords expected in this format:
        ///
        /// `customers/{customer_id}/keywordPlanNegativeKeywords/{kp_negative_keyword_id}`
        #[prost(string, tag = "3")]
        Remove(std::string::String),
    }
}
/// Response message for a Keyword Plan negative keyword mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateKeywordPlanNegativeKeywordsResponse {
    /// Errors that pertain to operation failures in the partial failure mode.
    /// Returned only when partial_failure = true and all errors occur inside the
    /// operations. If any errors occur outside the operations (e.g. auth errors),
    /// we return an RPC level error.
    #[prost(message, optional, tag = "3")]
    pub partial_failure_error: ::std::option::Option<super::super::super::super::rpc::Status>,
    /// All results for the mutate.
    #[prost(message, repeated, tag = "2")]
    pub results: ::std::vec::Vec<MutateKeywordPlanNegativeKeywordResult>,
}
/// The result for the Keyword Plan negative keyword mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateKeywordPlanNegativeKeywordResult {
    /// Returned for successful operations.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
#[doc = r" Generated client implementations."]
pub mod keyword_plan_negative_keyword_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Service to manage Keyword Plan negative keywords."]
    pub struct KeywordPlanNegativeKeywordServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> KeywordPlanNegativeKeywordServiceClient<T>
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
        #[doc = " Returns the requested plan in full detail."]
        pub async fn get_keyword_plan_negative_keyword(
            &mut self,
            request: impl tonic::IntoRequest<super::GetKeywordPlanNegativeKeywordRequest>,
        ) -> Result<
            tonic::Response<super::super::resources::KeywordPlanNegativeKeyword>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ( "/google.ads.googleads.v3.services.KeywordPlanNegativeKeywordService/GetKeywordPlanNegativeKeyword" ) ;
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates, updates, or removes Keyword Plan negative keywords. Operation"]
        #[doc = " statuses are returned."]
        pub async fn mutate_keyword_plan_negative_keywords(
            &mut self,
            request: impl tonic::IntoRequest<super::MutateKeywordPlanNegativeKeywordsRequest>,
        ) -> Result<tonic::Response<super::MutateKeywordPlanNegativeKeywordsResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ( "/google.ads.googleads.v3.services.KeywordPlanNegativeKeywordService/MutateKeywordPlanNegativeKeywords" ) ;
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for KeywordPlanNegativeKeywordServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for KeywordPlanNegativeKeywordServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "KeywordPlanNegativeKeywordServiceClient {{ ... }}")
        }
    }
}
/// Request message for [KeywordPlanService.GetKeywordPlan][google.ads.googleads.v3.services.KeywordPlanService.GetKeywordPlan].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetKeywordPlanRequest {
    /// Required. The resource name of the plan to fetch.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
/// Request message for [KeywordPlanService.MutateKeywordPlans][google.ads.googleads.v3.services.KeywordPlanService.MutateKeywordPlans].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateKeywordPlansRequest {
    /// Required. The ID of the customer whose keyword plans are being modified.
    #[prost(string, tag = "1")]
    pub customer_id: std::string::String,
    /// Required. The list of operations to perform on individual keyword plans.
    #[prost(message, repeated, tag = "2")]
    pub operations: ::std::vec::Vec<KeywordPlanOperation>,
    /// If true, successful operations will be carried out and invalid
    /// operations will return errors. If false, all operations will be carried
    /// out in one transaction if and only if they are all valid.
    /// Default is false.
    #[prost(bool, tag = "3")]
    pub partial_failure: bool,
    /// If true, the request is validated but not executed. Only errors are
    /// returned, not results.
    #[prost(bool, tag = "4")]
    pub validate_only: bool,
}
/// A single operation (create, update, remove) on a keyword plan.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeywordPlanOperation {
    /// The FieldMask that determines which resource fields are modified in an
    /// update.
    #[prost(message, optional, tag = "4")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
    /// The mutate operation.
    #[prost(oneof = "keyword_plan_operation::Operation", tags = "1, 2, 3")]
    pub operation: ::std::option::Option<keyword_plan_operation::Operation>,
}
pub mod keyword_plan_operation {
    /// The mutate operation.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Operation {
        /// Create operation: No resource name is expected for the new keyword plan.
        #[prost(message, tag = "1")]
        Create(super::super::resources::KeywordPlan),
        /// Update operation: The keyword plan is expected to have a valid resource
        /// name.
        #[prost(message, tag = "2")]
        Update(super::super::resources::KeywordPlan),
        /// Remove operation: A resource name for the removed keyword plan is
        /// expected in this format:
        ///
        /// `customers/{customer_id}/keywordPlans/{keyword_plan_id}`
        #[prost(string, tag = "3")]
        Remove(std::string::String),
    }
}
/// Response message for a keyword plan mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateKeywordPlansResponse {
    /// Errors that pertain to operation failures in the partial failure mode.
    /// Returned only when partial_failure = true and all errors occur inside the
    /// operations. If any errors occur outside the operations (e.g. auth errors),
    /// we return an RPC level error.
    #[prost(message, optional, tag = "3")]
    pub partial_failure_error: ::std::option::Option<super::super::super::super::rpc::Status>,
    /// All results for the mutate.
    #[prost(message, repeated, tag = "2")]
    pub results: ::std::vec::Vec<MutateKeywordPlansResult>,
}
/// The result for the keyword plan mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateKeywordPlansResult {
    /// Returned for successful operations.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
/// Request message for [KeywordPlanService.GenerateForecastMetrics][google.ads.googleads.v3.services.KeywordPlanService.GenerateForecastMetrics].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateForecastMetricsRequest {
    /// Required. The resource name of the keyword plan to be forecasted.
    #[prost(string, tag = "1")]
    pub keyword_plan: std::string::String,
}
/// Response message for [KeywordPlanService.GenerateForecastMetrics][google.ads.googleads.v3.services.KeywordPlanService.GenerateForecastMetrics].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateForecastMetricsResponse {
    /// List of campaign forecasts.
    /// One maximum.
    #[prost(message, repeated, tag = "1")]
    pub campaign_forecasts: ::std::vec::Vec<KeywordPlanCampaignForecast>,
    /// List of ad group forecasts.
    #[prost(message, repeated, tag = "2")]
    pub ad_group_forecasts: ::std::vec::Vec<KeywordPlanAdGroupForecast>,
    /// List of keyword forecasts.
    #[prost(message, repeated, tag = "3")]
    pub keyword_forecasts: ::std::vec::Vec<KeywordPlanKeywordForecast>,
}
/// A campaign forecast.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeywordPlanCampaignForecast {
    /// The resource name of the Keyword Plan campaign related to the forecast.
    ///
    /// `customers/{customer_id}/keywordPlanCampaigns/{keyword_plan_campaign_id}`
    #[prost(message, optional, tag = "1")]
    pub keyword_plan_campaign: ::std::option::Option<::std::string::String>,
    /// The forecast for the Keyword Plan campaign.
    #[prost(message, optional, tag = "2")]
    pub campaign_forecast: ::std::option::Option<ForecastMetrics>,
}
/// An ad group forecast.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeywordPlanAdGroupForecast {
    /// The resource name of the Keyword Plan ad group related to the forecast.
    ///
    /// `customers/{customer_id}/keywordPlanAdGroups/{keyword_plan_ad_group_id}`
    #[prost(message, optional, tag = "1")]
    pub keyword_plan_ad_group: ::std::option::Option<::std::string::String>,
    /// The forecast for the Keyword Plan ad group.
    #[prost(message, optional, tag = "2")]
    pub ad_group_forecast: ::std::option::Option<ForecastMetrics>,
}
/// A keyword forecast.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeywordPlanKeywordForecast {
    /// The resource name of the Keyword Plan keyword related to the forecast.
    ///
    /// `customers/{customer_id}/keywordPlanAdGroupKeywords/{keyword_plan_ad_group_keyword_id}`
    #[prost(message, optional, tag = "1")]
    pub keyword_plan_ad_group_keyword: ::std::option::Option<::std::string::String>,
    /// The forecast for the Keyword Plan keyword.
    #[prost(message, optional, tag = "2")]
    pub keyword_forecast: ::std::option::Option<ForecastMetrics>,
}
/// Forecast metrics.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ForecastMetrics {
    /// Impressions
    #[prost(message, optional, tag = "1")]
    pub impressions: ::std::option::Option<f64>,
    /// Ctr
    #[prost(message, optional, tag = "2")]
    pub ctr: ::std::option::Option<f64>,
    /// AVG cpc
    #[prost(message, optional, tag = "3")]
    pub average_cpc: ::std::option::Option<i64>,
    /// Clicks
    #[prost(message, optional, tag = "5")]
    pub clicks: ::std::option::Option<f64>,
    /// Cost
    #[prost(message, optional, tag = "6")]
    pub cost_micros: ::std::option::Option<i64>,
}
/// Request message for [KeywordPlanService.GenerateHistoricalMetrics][google.ads.googleads.v3.services.KeywordPlanService.GenerateHistoricalMetrics].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateHistoricalMetricsRequest {
    /// Required. The resource name of the keyword plan of which historical metrics are
    /// requested.
    #[prost(string, tag = "1")]
    pub keyword_plan: std::string::String,
}
/// Response message for [KeywordPlanService.GenerateHistoricalMetrics][google.ads.googleads.v3.services.KeywordPlanService.GenerateHistoricalMetrics].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateHistoricalMetricsResponse {
    /// List of keyword historical metrics.
    #[prost(message, repeated, tag = "1")]
    pub metrics: ::std::vec::Vec<KeywordPlanKeywordHistoricalMetrics>,
}
/// A keyword historical metrics.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeywordPlanKeywordHistoricalMetrics {
    /// The text of the query associated with one or more ad_group_keywords in the
    /// plan.
    ///
    /// Note that we de-dupe your keywords list, eliminating close variants before
    /// returning the plan's keywords as text. For example, if your plan originally
    /// contained the keywords 'car' and 'cars', the returned search query will
    /// only contain 'cars'.
    /// Starting V5, the list of de-duped queries will be included in
    /// close_variants field.
    #[prost(message, optional, tag = "1")]
    pub search_query: ::std::option::Option<::std::string::String>,
    /// The historical metrics for the query associated with one or more
    /// ad_group_keywords in the plan.
    #[prost(message, optional, tag = "2")]
    pub keyword_metrics: ::std::option::Option<super::common::KeywordPlanHistoricalMetrics>,
}
#[doc = r" Generated client implementations."]
pub mod keyword_plan_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Service to manage keyword plans."]
    pub struct KeywordPlanServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> KeywordPlanServiceClient<T>
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
        #[doc = " Returns the requested plan in full detail."]
        pub async fn get_keyword_plan(
            &mut self,
            request: impl tonic::IntoRequest<super::GetKeywordPlanRequest>,
        ) -> Result<tonic::Response<super::super::resources::KeywordPlan>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ads.googleads.v3.services.KeywordPlanService/GetKeywordPlan",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates, updates, or removes keyword plans. Operation statuses are"]
        #[doc = " returned."]
        pub async fn mutate_keyword_plans(
            &mut self,
            request: impl tonic::IntoRequest<super::MutateKeywordPlansRequest>,
        ) -> Result<tonic::Response<super::MutateKeywordPlansResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ads.googleads.v3.services.KeywordPlanService/MutateKeywordPlans",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns the requested Keyword Plan forecasts."]
        pub async fn generate_forecast_metrics(
            &mut self,
            request: impl tonic::IntoRequest<super::GenerateForecastMetricsRequest>,
        ) -> Result<tonic::Response<super::GenerateForecastMetricsResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ads.googleads.v3.services.KeywordPlanService/GenerateForecastMetrics",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns the requested Keyword Plan historical metrics."]
        pub async fn generate_historical_metrics(
            &mut self,
            request: impl tonic::IntoRequest<super::GenerateHistoricalMetricsRequest>,
        ) -> Result<tonic::Response<super::GenerateHistoricalMetricsResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ads.googleads.v3.services.KeywordPlanService/GenerateHistoricalMetrics",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for KeywordPlanServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for KeywordPlanServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "KeywordPlanServiceClient {{ ... }}")
        }
    }
}
/// Request message for [KeywordViewService.GetKeywordView][google.ads.googleads.v3.services.KeywordViewService.GetKeywordView].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetKeywordViewRequest {
    /// Required. The resource name of the keyword view to fetch.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
#[doc = r" Generated client implementations."]
pub mod keyword_view_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Service to manage keyword views."]
    pub struct KeywordViewServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> KeywordViewServiceClient<T>
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
        #[doc = " Returns the requested keyword view in full detail."]
        pub async fn get_keyword_view(
            &mut self,
            request: impl tonic::IntoRequest<super::GetKeywordViewRequest>,
        ) -> Result<tonic::Response<super::super::resources::KeywordView>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ads.googleads.v3.services.KeywordViewService/GetKeywordView",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for KeywordViewServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for KeywordViewServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "KeywordViewServiceClient {{ ... }}")
        }
    }
}
/// Request message for [LandingPageViewService.GetLandingPageView][google.ads.googleads.v3.services.LandingPageViewService.GetLandingPageView].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLandingPageViewRequest {
    /// Required. The resource name of the landing page view to fetch.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
#[doc = r" Generated client implementations."]
pub mod landing_page_view_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Service to fetch landing page views."]
    pub struct LandingPageViewServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> LandingPageViewServiceClient<T>
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
        #[doc = " Returns the requested landing page view in full detail."]
        pub async fn get_landing_page_view(
            &mut self,
            request: impl tonic::IntoRequest<super::GetLandingPageViewRequest>,
        ) -> Result<tonic::Response<super::super::resources::LandingPageView>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ads.googleads.v3.services.LandingPageViewService/GetLandingPageView",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for LandingPageViewServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for LandingPageViewServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "LandingPageViewServiceClient {{ ... }}")
        }
    }
}
/// Request message for [LanguageConstantService.GetLanguageConstant][google.ads.googleads.v3.services.LanguageConstantService.GetLanguageConstant].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLanguageConstantRequest {
    /// Required. Resource name of the language constant to fetch.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
#[doc = r" Generated client implementations."]
pub mod language_constant_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Service to fetch language constants."]
    pub struct LanguageConstantServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> LanguageConstantServiceClient<T>
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
        #[doc = " Returns the requested language constant."]
        pub async fn get_language_constant(
            &mut self,
            request: impl tonic::IntoRequest<super::GetLanguageConstantRequest>,
        ) -> Result<tonic::Response<super::super::resources::LanguageConstant>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ads.googleads.v3.services.LanguageConstantService/GetLanguageConstant",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for LanguageConstantServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for LanguageConstantServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "LanguageConstantServiceClient {{ ... }}")
        }
    }
}
/// Request message for [LocationViewService.GetLocationView][google.ads.googleads.v3.services.LocationViewService.GetLocationView].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLocationViewRequest {
    /// Required. The resource name of the location view to fetch.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
#[doc = r" Generated client implementations."]
pub mod location_view_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Service to fetch location views."]
    pub struct LocationViewServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> LocationViewServiceClient<T>
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
        #[doc = " Returns the requested location view in full detail."]
        pub async fn get_location_view(
            &mut self,
            request: impl tonic::IntoRequest<super::GetLocationViewRequest>,
        ) -> Result<tonic::Response<super::super::resources::LocationView>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ads.googleads.v3.services.LocationViewService/GetLocationView",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for LocationViewServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for LocationViewServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "LocationViewServiceClient {{ ... }}")
        }
    }
}
/// Request message for [ManagedPlacementViewService.GetManagedPlacementView][google.ads.googleads.v3.services.ManagedPlacementViewService.GetManagedPlacementView].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetManagedPlacementViewRequest {
    /// Required. The resource name of the Managed Placement View to fetch.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
#[doc = r" Generated client implementations."]
pub mod managed_placement_view_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Service to manage Managed Placement views."]
    pub struct ManagedPlacementViewServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> ManagedPlacementViewServiceClient<T>
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
        #[doc = " Returns the requested Managed Placement view in full detail."]
        pub async fn get_managed_placement_view(
            &mut self,
            request: impl tonic::IntoRequest<super::GetManagedPlacementViewRequest>,
        ) -> Result<tonic::Response<super::super::resources::ManagedPlacementView>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ( "/google.ads.googleads.v3.services.ManagedPlacementViewService/GetManagedPlacementView" ) ;
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for ManagedPlacementViewServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for ManagedPlacementViewServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "ManagedPlacementViewServiceClient {{ ... }}")
        }
    }
}
/// Request message for [MerchantCenterLinkService.ListMerchantCenterLinks][google.ads.googleads.v3.services.MerchantCenterLinkService.ListMerchantCenterLinks].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListMerchantCenterLinksRequest {
    /// Required. The ID of the customer onto which to apply the Merchant Center link list
    /// operation.
    #[prost(string, tag = "1")]
    pub customer_id: std::string::String,
}
/// Response message for [MerchantCenterLinkService.ListMerchantCenterLinks][google.ads.googleads.v3.services.MerchantCenterLinkService.ListMerchantCenterLinks].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListMerchantCenterLinksResponse {
    /// Merchant Center links available for the requested customer
    #[prost(message, repeated, tag = "1")]
    pub merchant_center_links: ::std::vec::Vec<super::resources::MerchantCenterLink>,
}
/// Request message for [MerchantCenterLinkService.GetMerchantCenterLink][google.ads.googleads.v3.services.MerchantCenterLinkService.GetMerchantCenterLink].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMerchantCenterLinkRequest {
    /// Required. Resource name of the Merchant Center link.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
/// Request message for [MerchantCenterLinkService.MutateMerchantCenterLink][google.ads.googleads.v3.services.MerchantCenterLinkService.MutateMerchantCenterLink].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateMerchantCenterLinkRequest {
    /// Required. The ID of the customer being modified.
    #[prost(string, tag = "1")]
    pub customer_id: std::string::String,
    /// Required. The operation to perform on the link
    #[prost(message, optional, tag = "2")]
    pub operation: ::std::option::Option<MerchantCenterLinkOperation>,
}
/// A single update on a Merchant Center link.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MerchantCenterLinkOperation {
    /// FieldMask that determines which resource fields are modified in an update.
    #[prost(message, optional, tag = "3")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
    /// The operation to perform
    #[prost(oneof = "merchant_center_link_operation::Operation", tags = "1, 2")]
    pub operation: ::std::option::Option<merchant_center_link_operation::Operation>,
}
pub mod merchant_center_link_operation {
    /// The operation to perform
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Operation {
        /// Update operation: The merchant center link is expected to have a valid
        /// resource name.
        #[prost(message, tag = "1")]
        Update(super::super::resources::MerchantCenterLink),
        /// Remove operation: A resource name for the removed merchant center link is
        /// expected, in this format:
        ///
        /// `customers/{customer_id}/merchantCenterLinks/{merchant_center_id}`
        #[prost(string, tag = "2")]
        Remove(std::string::String),
    }
}
/// Response message for Merchant Center link mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateMerchantCenterLinkResponse {
    /// Result for the mutate.
    #[prost(message, optional, tag = "2")]
    pub result: ::std::option::Option<MutateMerchantCenterLinkResult>,
}
/// The result for the Merchant Center link mutate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateMerchantCenterLinkResult {
    /// Returned for successful operations.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
#[doc = r" Generated client implementations."]
pub mod merchant_center_link_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " This service allows management of links between Google Ads and Google"]
    #[doc = " Merchant Center."]
    pub struct MerchantCenterLinkServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> MerchantCenterLinkServiceClient<T>
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
        #[doc = " Returns Merchant Center links available for this customer."]
        pub async fn list_merchant_center_links(
            &mut self,
            request: impl tonic::IntoRequest<super::ListMerchantCenterLinksRequest>,
        ) -> Result<tonic::Response<super::ListMerchantCenterLinksResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ( "/google.ads.googleads.v3.services.MerchantCenterLinkService/ListMerchantCenterLinks" ) ;
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns the Merchant Center link in full detail."]
        pub async fn get_merchant_center_link(
            &mut self,
            request: impl tonic::IntoRequest<super::GetMerchantCenterLinkRequest>,
        ) -> Result<tonic::Response<super::super::resources::MerchantCenterLink>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ads.googleads.v3.services.MerchantCenterLinkService/GetMerchantCenterLink",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates status or removes a Merchant Center link."]
        pub async fn mutate_merchant_center_link(
            &mut self,
            request: impl tonic::IntoRequest<super::MutateMerchantCenterLinkRequest>,
        ) -> Result<tonic::Response<super::MutateMerchantCenterLinkResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ( "/google.ads.googleads.v3.services.MerchantCenterLinkService/MutateMerchantCenterLink" ) ;
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for MerchantCenterLinkServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for MerchantCenterLinkServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "MerchantCenterLinkServiceClient {{ ... }}")
        }
    }
}
/// Request message for
/// [MobileAppCategoryConstantService.GetMobileAppCategoryConstant][google.ads.googleads.v3.services.MobileAppCategoryConstantService.GetMobileAppCategoryConstant].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMobileAppCategoryConstantRequest {
    /// Required. Resource name of the mobile app category constant to fetch.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
#[doc = r" Generated client implementations."]
pub mod mobile_app_category_constant_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Service to fetch mobile app category constants."]
    pub struct MobileAppCategoryConstantServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> MobileAppCategoryConstantServiceClient<T>
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
        #[doc = " Returns the requested mobile app category constant."]
        pub async fn get_mobile_app_category_constant(
            &mut self,
            request: impl tonic::IntoRequest<super::GetMobileAppCategoryConstantRequest>,
        ) -> Result<
            tonic::Response<super::super::resources::MobileAppCategoryConstant>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ( "/google.ads.googleads.v3.services.MobileAppCategoryConstantService/GetMobileAppCategoryConstant" ) ;
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for MobileAppCategoryConstantServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for MobileAppCategoryConstantServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "MobileAppCategoryConstantServiceClient {{ ... }}")
        }
    }
}
/// Request message for [MobileDeviceConstantService.GetMobileDeviceConstant][google.ads.googleads.v3.services.MobileDeviceConstantService.GetMobileDeviceConstant].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMobileDeviceConstantRequest {
    /// Required. Resource name of the mobile device to fetch.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
#[doc = r" Generated client implementations."]
pub mod mobile_device_constant_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Service to fetch mobile device constants."]
    pub struct MobileDeviceConstantServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> MobileDeviceConstantServiceClient<T>
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
        #[doc = " Returns the requested mobile device constant in full detail."]
        pub async fn get_mobile_device_constant(
            &mut self,
            request: impl tonic::IntoRequest<super::GetMobileDeviceConstantRequest>,
        ) -> Result<tonic::Response<super::super::resources::MobileDeviceConstant>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ( "/google.ads.googleads.v3.services.MobileDeviceConstantService/GetMobileDeviceConstant" ) ;
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for MobileDeviceConstantServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for MobileDeviceConstantServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "MobileDeviceConstantServiceClient {{ ... }}")
        }
    }
}
/// Request message for [MutateJobService.CreateMutateJobRequest][]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateMutateJobRequest {
    /// Required. The ID of the customer for which to create a mutate job.
    #[prost(string, tag = "1")]
    pub customer_id: std::string::String,
}
/// Response message for [MutateJobService.CreateMutateJobResponse][]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateMutateJobResponse {
    /// The resource name of the MutateJob.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
/// Request message for [MutateJobService.GetMutateJob][google.ads.googleads.v3.services.MutateJobService.GetMutateJob]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMutateJobRequest {
    /// Required. The resource name of the MutateJob to get.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
/// Request message for [MutateJobService.RunMutateJob][google.ads.googleads.v3.services.MutateJobService.RunMutateJob]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RunMutateJobRequest {
    /// Required. The resource name of the MutateJob to run.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
/// Request message for [MutateJobService.AddMutateJobOperations][google.ads.googleads.v3.services.MutateJobService.AddMutateJobOperations]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddMutateJobOperationsRequest {
    /// Required. The resource name of the MutateJob.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
    /// A token used to enforce sequencing.
    ///
    /// The first AddMutateJobOperations request for a MutateJob should not set
    /// sequence_token. Subsequent requests must set sequence_token to the value of
    /// next_sequence_token received in the previous AddMutateJobOperations
    /// response.
    #[prost(string, tag = "2")]
    pub sequence_token: std::string::String,
    /// Required. The list of mutates being added.
    ///
    /// Operations can use negative integers as temp ids to signify dependencies
    /// between entities created in this MutateJob. For example, a customer with
    /// id = 1234 can create a campaign and an ad group in that same campaign by
    /// creating a campaign in the first operation with the resource name
    /// explicitly set to "customers/1234/campaigns/-1", and creating an ad group
    /// in the second operation with the campaign field also set to
    /// "customers/1234/campaigns/-1".
    #[prost(message, repeated, tag = "3")]
    pub mutate_operations: ::std::vec::Vec<MutateOperation>,
}
/// Response message for [MutateJobService.AddMutateJobOperations][google.ads.googleads.v3.services.MutateJobService.AddMutateJobOperations]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddMutateJobOperationsResponse {
    /// The total number of operations added so far for this job.
    #[prost(int64, tag = "1")]
    pub total_operations: i64,
    /// The sequence token to be used when calling AddMutateJobOperations again if
    /// more operations need to be added. The next AddMutateJobOperations request
    /// must set the sequence_token field to the value of this field.
    #[prost(string, tag = "2")]
    pub next_sequence_token: std::string::String,
}
/// Request message for [MutateJobService.ListMutateJobResults][google.ads.googleads.v3.services.MutateJobService.ListMutateJobResults].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListMutateJobResultsRequest {
    /// Required. The resource name of the MutateJob whose results are being listed.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
    /// Token of the page to retrieve. If not specified, the first
    /// page of results will be returned. Use the value obtained from
    /// `next_page_token` in the previous response in order to request
    /// the next page of results.
    #[prost(string, tag = "2")]
    pub page_token: std::string::String,
    /// Number of elements to retrieve in a single page.
    /// When a page request is too large, the server may decide to
    /// further limit the number of returned resources.
    #[prost(int32, tag = "3")]
    pub page_size: i32,
}
/// Response message for [MutateJobService.ListMutateJobResults][google.ads.googleads.v3.services.MutateJobService.ListMutateJobResults].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListMutateJobResultsResponse {
    /// The list of rows that matched the query.
    #[prost(message, repeated, tag = "1")]
    pub results: ::std::vec::Vec<MutateJobResult>,
    /// Pagination token used to retrieve the next page of results.
    /// Pass the content of this string as the `page_token` attribute of
    /// the next request. `next_page_token` is not returned for the last
    /// page.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
}
/// MutateJob result.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MutateJobResult {
    /// Index of the mutate operation.
    #[prost(int64, tag = "1")]
    pub operation_index: i64,
    /// Response for the mutate.
    /// May be empty if errors occurred.
    #[prost(message, optional, tag = "2")]
    pub mutate_operation_response: ::std::option::Option<MutateOperationResponse>,
    /// Details of the errors when processing the operation.
    #[prost(message, optional, tag = "3")]
    pub status: ::std::option::Option<super::super::super::super::rpc::Status>,
}
#[doc = r" Generated client implementations."]
pub mod mutate_job_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Service to manage mutate jobs."]
    pub struct MutateJobServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> MutateJobServiceClient<T>
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
        #[doc = " Creates a mutate job."]
        pub async fn create_mutate_job(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateMutateJobRequest>,
        ) -> Result<tonic::Response<super::CreateMutateJobResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ads.googleads.v3.services.MutateJobService/CreateMutateJob",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns the mutate job."]
        pub async fn get_mutate_job(
            &mut self,
            request: impl tonic::IntoRequest<super::GetMutateJobRequest>,
        ) -> Result<tonic::Response<super::super::resources::MutateJob>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ads.googleads.v3.services.MutateJobService/GetMutateJob",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns the results of the mutate job. The job must be done."]
        #[doc = " Supports standard list paging."]
        pub async fn list_mutate_job_results(
            &mut self,
            request: impl tonic::IntoRequest<super::ListMutateJobResultsRequest>,
        ) -> Result<tonic::Response<super::ListMutateJobResultsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ads.googleads.v3.services.MutateJobService/ListMutateJobResults",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Runs the mutate job."]
        #[doc = ""]
        #[doc = " The Operation.metadata field type is MutateJobMetadata. When finished, the"]
        #[doc = " long running operation will not contain errors or a response. Instead, use"]
        #[doc = " ListMutateJobResults to get the results of the job."]
        pub async fn run_mutate_job(
            &mut self,
            request: impl tonic::IntoRequest<super::RunMutateJobRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::super::longrunning::Operation>,
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
                "/google.ads.googleads.v3.services.MutateJobService/RunMutateJob",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Add operations to the mutate job."]
        pub async fn add_mutate_job_operations(
            &mut self,
            request: impl tonic::IntoRequest<super::AddMutateJobOperationsRequest>,
        ) -> Result<tonic::Response<super::AddMutateJobOperationsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ads.googleads.v3.services.MutateJobService/AddMutateJobOperations",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for MutateJobServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for MutateJobServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "MutateJobServiceClient {{ ... }}")
        }
    }
}
/// Request message for
/// [OfflineUserDataJobService.CreateOfflineUserDataJob][google.ads.googleads.v3.services.OfflineUserDataJobService.CreateOfflineUserDataJob].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateOfflineUserDataJobRequest {
    /// Required. The ID of the customer for which to create an offline user data job.
    #[prost(string, tag = "1")]
    pub customer_id: std::string::String,
    /// Required. The offline user data job to be created.
    #[prost(message, optional, tag = "2")]
    pub job: ::std::option::Option<super::resources::OfflineUserDataJob>,
}
/// Response message for
/// [OfflineUserDataJobService.CreateOfflineUserDataJob][google.ads.googleads.v3.services.OfflineUserDataJobService.CreateOfflineUserDataJob].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateOfflineUserDataJobResponse {
    /// The resource name of the OfflineUserDataJob.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
/// Request message for [OfflineUserDataJobService.GetOfflineUserDataJob][google.ads.googleads.v3.services.OfflineUserDataJobService.GetOfflineUserDataJob].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetOfflineUserDataJobRequest {
    /// Required. The resource name of the OfflineUserDataJob to get.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
/// Request message for [OfflineUserDataJobService.RunOfflineUserDataJob][google.ads.googleads.v3.services.OfflineUserDataJobService.RunOfflineUserDataJob].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RunOfflineUserDataJobRequest {
    /// Required. The resource name of the OfflineUserDataJob to run.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
/// Request message for
/// [OfflineUserDataJobService.AddOfflineUserDataJobOperations][google.ads.googleads.v3.services.OfflineUserDataJobService.AddOfflineUserDataJobOperations].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddOfflineUserDataJobOperationsRequest {
    /// Required. The resource name of the OfflineUserDataJob.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
    /// True to enable partial failure for the offline user data job.
    #[prost(message, optional, tag = "2")]
    pub enable_partial_failure: ::std::option::Option<bool>,
    /// Required. The list of operations to be done.
    #[prost(message, repeated, tag = "3")]
    pub operations: ::std::vec::Vec<OfflineUserDataJobOperation>,
}
/// Operation to be made for the AddOfflineUserDataJobOperationsRequest.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OfflineUserDataJobOperation {
    /// Operation to be made for the AddOfflineUserDataJobOperationsRequest.
    #[prost(oneof = "offline_user_data_job_operation::Operation", tags = "1, 2, 3")]
    pub operation: ::std::option::Option<offline_user_data_job_operation::Operation>,
}
pub mod offline_user_data_job_operation {
    /// Operation to be made for the AddOfflineUserDataJobOperationsRequest.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Operation {
        /// Add the provided data to the transaction. Data cannot be retrieved after
        /// being uploaded.
        #[prost(message, tag = "1")]
        Create(super::super::common::UserData),
        /// Remove the provided data from the transaction. Data cannot be retrieved
        /// after being uploaded.
        #[prost(message, tag = "2")]
        Remove(super::super::common::UserData),
        /// Remove all previously provided data. This is only supported for Customer
        /// Match.
        #[prost(bool, tag = "3")]
        RemoveAll(bool),
    }
}
/// Response message for
/// [OfflineUserDataJobService.AddOfflineUserDataJobOperations][google.ads.googleads.v3.services.OfflineUserDataJobService.AddOfflineUserDataJobOperations].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddOfflineUserDataJobOperationsResponse {
    /// Errors that pertain to operation failures in the partial failure mode.
    /// Returned only when partial_failure = true and all errors occur inside the
    /// operations. If any errors occur outside the operations (e.g. auth errors),
    /// we return an RPC level error.
    #[prost(message, optional, tag = "1")]
    pub partial_failure_error: ::std::option::Option<super::super::super::super::rpc::Status>,
}
#[doc = r" Generated client implementations."]
pub mod offline_user_data_job_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Service to manage offline user data jobs."]
    pub struct OfflineUserDataJobServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> OfflineUserDataJobServiceClient<T>
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
        #[doc = " Creates an offline user data job."]
        pub async fn create_offline_user_data_job(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateOfflineUserDataJobRequest>,
        ) -> Result<tonic::Response<super::CreateOfflineUserDataJobResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ( "/google.ads.googleads.v3.services.OfflineUserDataJobService/CreateOfflineUserDataJob" ) ;
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns the offline user data job."]
        pub async fn get_offline_user_data_job(
            &mut self,
            request: impl tonic::IntoRequest<super::GetOfflineUserDataJobRequest>,
        ) -> Result<tonic::Response<super::super::resources::OfflineUserDataJob>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ads.googleads.v3.services.OfflineUserDataJobService/GetOfflineUserDataJob",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Adds operations to the offline user data job."]
        pub async fn add_offline_user_data_job_operations(
            &mut self,
            request: impl tonic::IntoRequest<super::AddOfflineUserDataJobOperationsRequest>,
        ) -> Result<tonic::Response<super::AddOfflineUserDataJobOperationsResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ( "/google.ads.googleads.v3.services.OfflineUserDataJobService/AddOfflineUserDataJobOperations" ) ;
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Runs the offline user data job."]
        #[doc = ""]
        #[doc = " When finished, the long running operation will contain the processing"]
        #[doc = " result or failure information, if any."]
        pub async fn run_offline_user_data_job(
            &mut self,
            request: impl tonic::IntoRequest<super::RunOfflineUserDataJobRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::super::longrunning::Operation>,
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
                "/google.ads.googleads.v3.services.OfflineUserDataJobService/RunOfflineUserDataJob",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for OfflineUserDataJobServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for OfflineUserDataJobServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "OfflineUserDataJobServiceClient {{ ... }}")
        }
    }
}
/// Request message for
/// [OperatingSystemVersionConstantService.GetOperatingSystemVersionConstant][google.ads.googleads.v3.services.OperatingSystemVersionConstantService.GetOperatingSystemVersionConstant].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetOperatingSystemVersionConstantRequest {
    /// Required. Resource name of the OS version to fetch.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
#[doc = r" Generated client implementations."]
pub mod operating_system_version_constant_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Service to fetch Operating System Version constants."]
    pub struct OperatingSystemVersionConstantServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> OperatingSystemVersionConstantServiceClient<T>
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
        #[doc = " Returns the requested OS version constant in full detail."]
        pub async fn get_operating_system_version_constant(
            &mut self,
            request: impl tonic::IntoRequest<super::GetOperatingSystemVersionConstantRequest>,
        ) -> Result<
            tonic::Response<super::super::resources::OperatingSystemVersionConstant>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ( "/google.ads.googleads.v3.services.OperatingSystemVersionConstantService/GetOperatingSystemVersionConstant" ) ;
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for OperatingSystemVersionConstantServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for OperatingSystemVersionConstantServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "OperatingSystemVersionConstantServiceClient {{ ... }}")
        }
    }
}
/// Request message for
/// [PaidOrganicSearchTermViewService.GetPaidOrganicSearchTermView][google.ads.googleads.v3.services.PaidOrganicSearchTermViewService.GetPaidOrganicSearchTermView].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPaidOrganicSearchTermViewRequest {
    /// Required. The resource name of the paid organic search term view to fetch.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
#[doc = r" Generated client implementations."]
pub mod paid_organic_search_term_view_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Service to fetch paid organic search term views."]
    pub struct PaidOrganicSearchTermViewServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> PaidOrganicSearchTermViewServiceClient<T>
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
        #[doc = " Returns the requested paid organic search term view in full detail."]
        pub async fn get_paid_organic_search_term_view(
            &mut self,
            request: impl tonic::IntoRequest<super::GetPaidOrganicSearchTermViewRequest>,
        ) -> Result<
            tonic::Response<super::super::resources::PaidOrganicSearchTermView>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ( "/google.ads.googleads.v3.services.PaidOrganicSearchTermViewService/GetPaidOrganicSearchTermView" ) ;
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for PaidOrganicSearchTermViewServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for PaidOrganicSearchTermViewServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "PaidOrganicSearchTermViewServiceClient {{ ... }}")
        }
    }
}
/// Request message for [ParentalStatusViewService.GetParentalStatusView][google.ads.googleads.v3.services.ParentalStatusViewService.GetParentalStatusView].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetParentalStatusViewRequest {
    /// Required. The resource name of the parental status view to fetch.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
#[doc = r" Generated client implementations."]
pub mod parental_status_view_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Service to manage parental status views."]
    pub struct ParentalStatusViewServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> ParentalStatusViewServiceClient<T>
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
        #[doc = " Returns the requested parental status view in full detail."]
        pub async fn get_parental_status_view(
            &mut self,
            request: impl tonic::IntoRequest<super::GetParentalStatusViewRequest>,
        ) -> Result<tonic::Response<super::super::resources::ParentalStatusView>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ads.googleads.v3.services.ParentalStatusViewService/GetParentalStatusView",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for ParentalStatusViewServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for ParentalStatusViewServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "ParentalStatusViewServiceClient {{ ... }}")
        }
    }
}
/// Request message for fetching all accessible payments accounts.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPaymentsAccountsRequest {
    /// Required. The ID of the customer to apply the PaymentsAccount list operation to.
    #[prost(string, tag = "1")]
    pub customer_id: std::string::String,
}
/// Response message for [PaymentsAccountService.ListPaymentsAccounts][google.ads.googleads.v3.services.PaymentsAccountService.ListPaymentsAccounts].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPaymentsAccountsResponse {
    /// The list of accessible payments accounts.
    #[prost(message, repeated, tag = "1")]
    pub payments_accounts: ::std::vec::Vec<super::resources::PaymentsAccount>,
}
#[doc = r" Generated client implementations."]
pub mod payments_account_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Service to provide payments accounts that can be used to set up consolidated"]
    #[doc = " billing."]
    pub struct PaymentsAccountServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> PaymentsAccountServiceClient<T>
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
        #[doc = " Returns all payments accounts associated with all managers"]
        #[doc = " between the login customer ID and specified serving customer in the"]
        #[doc = " hierarchy, inclusive."]
        pub async fn list_payments_accounts(
            &mut self,
            request: impl tonic::IntoRequest<super::ListPaymentsAccountsRequest>,
        ) -> Result<tonic::Response<super::ListPaymentsAccountsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ads.googleads.v3.services.PaymentsAccountService/ListPaymentsAccounts",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for PaymentsAccountServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for PaymentsAccountServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "PaymentsAccountServiceClient {{ ... }}")
        }
    }
}
/// Request message for
/// [ProductBiddingCategoryConstantService.GetProductBiddingCategoryConstant][google.ads.googleads.v3.services.ProductBiddingCategoryConstantService.GetProductBiddingCategoryConstant].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetProductBiddingCategoryConstantRequest {
    /// Required. Resource name of the Product Bidding Category to fetch.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
#[doc = r" Generated client implementations."]
pub mod product_bidding_category_constant_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Service to fetch Product Bidding Categories."]
    pub struct ProductBiddingCategoryConstantServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> ProductBiddingCategoryConstantServiceClient<T>
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
        #[doc = " Returns the requested Product Bidding Category in full detail."]
        pub async fn get_product_bidding_category_constant(
            &mut self,
            request: impl tonic::IntoRequest<super::GetProductBiddingCategoryConstantRequest>,
        ) -> Result<
            tonic::Response<super::super::resources::ProductBiddingCategoryConstant>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ( "/google.ads.googleads.v3.services.ProductBiddingCategoryConstantService/GetProductBiddingCategoryConstant" ) ;
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for ProductBiddingCategoryConstantServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for ProductBiddingCategoryConstantServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "ProductBiddingCategoryConstantServiceClient {{ ... }}")
        }
    }
}
/// Request message for [ProductGroupViewService.GetProductGroupView][google.ads.googleads.v3.services.ProductGroupViewService.GetProductGroupView].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetProductGroupViewRequest {
    /// Required. The resource name of the product group view to fetch.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
#[doc = r" Generated client implementations."]
pub mod product_group_view_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Service to manage product group views."]
    pub struct ProductGroupViewServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> ProductGroupViewServiceClient<T>
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
        #[doc = " Returns the requested product group view in full detail."]
        pub async fn get_product_group_view(
            &mut self,
            request: impl tonic::IntoRequest<super::GetProductGroupViewRequest>,
        ) -> Result<tonic::Response<super::super::resources::ProductGroupView>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ads.googleads.v3.services.ProductGroupViewService/GetProductGroupView",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for ProductGroupViewServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for ProductGroupViewServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "ProductGroupViewServiceClient {{ ... }}")
        }
    }
}
/// Request message for [ReachPlanService.ListPlannableLocations][google.ads.googleads.v3.services.ReachPlanService.ListPlannableLocations].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPlannableLocationsRequest {}
/// The list of plannable locations.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPlannableLocationsResponse {
    /// The list of locations available for planning (Countries, DMAs,
    /// sub-countries).
    /// For locations like Countries, DMAs see
    /// https://developers.google.com/adwords/api/docs/appendix/geotargeting for
    /// more information.
    #[prost(message, repeated, tag = "1")]
    pub plannable_locations: ::std::vec::Vec<PlannableLocation>,
}
/// A plannable location: a country, a DMA, a metro region, a tv region,
/// a province.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlannableLocation {
    /// The location identifier.
    #[prost(message, optional, tag = "1")]
    pub id: ::std::option::Option<::std::string::String>,
    /// The unique location name in english.
    #[prost(message, optional, tag = "2")]
    pub name: ::std::option::Option<::std::string::String>,
    /// The parent country code, not present if location is a country.
    /// If present will always be a criterion id: additional information, such as
    /// country name are returned both via ListPlannableLocations or directly by
    /// accessing GeoTargetConstantService with the criterion id.
    #[prost(message, optional, tag = "3")]
    pub parent_country_id: ::std::option::Option<i64>,
}
/// Request to list available products in a given location.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPlannableProductsRequest {
    /// Required. The ID of the selected location for planning. To list the available
    /// plannable location ids use ListPlannableLocations.
    #[prost(message, optional, tag = "1")]
    pub plannable_location_id: ::std::option::Option<::std::string::String>,
}
/// A response with all available products.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListPlannableProductsResponse {
    /// The list of products available for planning and related targeting metadata.
    #[prost(message, repeated, tag = "1")]
    pub product_metadata: ::std::vec::Vec<ProductMetadata>,
}
/// The metadata associated with an available plannable product.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProductMetadata {
    /// The code associated with the ad product. E.g. Trueview, Bumper
    /// To list the available plannable product codes use ListPlannableProducts.
    #[prost(message, optional, tag = "1")]
    pub plannable_product_code: ::std::option::Option<::std::string::String>,
    /// The allowed plannable targeting for this product.
    #[prost(message, optional, tag = "2")]
    pub plannable_targeting: ::std::option::Option<PlannableTargeting>,
}
/// The targeting for which traffic metrics will be reported.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlannableTargeting {
    /// Allowed plannable age ranges for the product for which metrics will be
    /// reported. Actual targeting is computed by mapping this age range onto
    /// standard Google common.AgeRangeInfo values.
    #[prost(
        enumeration = "super::enums::reach_plan_age_range_enum::ReachPlanAgeRange",
        repeated,
        tag = "1"
    )]
    pub age_ranges: ::std::vec::Vec<i32>,
    /// Targetable genders for the ad product.
    #[prost(message, repeated, tag = "2")]
    pub genders: ::std::vec::Vec<super::common::GenderInfo>,
    /// Targetable devices for the ad product.
    #[prost(message, repeated, tag = "3")]
    pub devices: ::std::vec::Vec<super::common::DeviceInfo>,
    /// Targetable networks for the ad product.
    #[prost(
        enumeration = "super::enums::reach_plan_network_enum::ReachPlanNetwork",
        repeated,
        tag = "4"
    )]
    pub networks: ::std::vec::Vec<i32>,
}
/// Request message for [ReachPlanService.GenerateProductMixIdeas][google.ads.googleads.v3.services.ReachPlanService.GenerateProductMixIdeas].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateProductMixIdeasRequest {
    /// Required. The ID of the customer.
    #[prost(string, tag = "1")]
    pub customer_id: std::string::String,
    /// Required. The ID of the location, this is one of the ids returned by
    /// ListPlannableLocations.
    #[prost(message, optional, tag = "2")]
    pub plannable_location_id: ::std::option::Option<::std::string::String>,
    /// Required. Currency code.
    /// Three-character ISO 4217 currency code.
    #[prost(message, optional, tag = "3")]
    pub currency_code: ::std::option::Option<::std::string::String>,
    /// Required. Total budget.
    /// Amount in micros. One million is equivalent to one unit.
    #[prost(message, optional, tag = "4")]
    pub budget_micros: ::std::option::Option<i64>,
    /// The preferences of the suggested product mix.
    /// An unset preference is interpreted as all possible values are allowed,
    /// unless explicitly specified.
    #[prost(message, optional, tag = "5")]
    pub preferences: ::std::option::Option<Preferences>,
}
/// Set of preferences about the planned mix.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Preferences {
    /// True if ad skippable.
    /// If not set, default is any value.
    #[prost(message, optional, tag = "1")]
    pub is_skippable: ::std::option::Option<bool>,
    /// True if ad start with sound.
    /// If not set, default is any value.
    #[prost(message, optional, tag = "2")]
    pub starts_with_sound: ::std::option::Option<bool>,
    /// The length of the ad.
    /// If not set, default is any value.
    #[prost(
        enumeration = "super::enums::reach_plan_ad_length_enum::ReachPlanAdLength",
        tag = "3"
    )]
    pub ad_length: i32,
    /// True if ad will only show on the top content.
    /// If not set, default is false.
    #[prost(message, optional, tag = "4")]
    pub top_content_only: ::std::option::Option<bool>,
    /// True if the price guaranteed. The cost of serving the ad is agreed upfront
    /// and not subject to an auction.
    /// If not set, default is any value.
    #[prost(message, optional, tag = "5")]
    pub has_guaranteed_price: ::std::option::Option<bool>,
}
/// The suggested product mix.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateProductMixIdeasResponse {
    /// A list of products (ad formats) and the associated budget allocation idea.
    #[prost(message, repeated, tag = "1")]
    pub product_allocation: ::std::vec::Vec<ProductAllocation>,
}
/// An allocation of a part of the budget on a given product.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProductAllocation {
    /// Selected product for planning. The product codes returned are within the
    /// set of the ones returned by ListPlannableProducts when using the same
    /// location id.
    #[prost(message, optional, tag = "1")]
    pub plannable_product_code: ::std::option::Option<::std::string::String>,
    /// The value to be allocated for the suggested product in requested currency.
    /// Amount in micros. One million is equivalent to one unit.
    #[prost(message, optional, tag = "2")]
    pub budget_micros: ::std::option::Option<i64>,
}
/// Request message for [ReachPlanService.GenerateReachForecast][google.ads.googleads.v3.services.ReachPlanService.GenerateReachForecast].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateReachForecastRequest {
    /// Required. The ID of the customer.
    #[prost(string, tag = "1")]
    pub customer_id: std::string::String,
    /// The currency code.
    /// Three-character ISO 4217 currency code.
    #[prost(message, optional, tag = "2")]
    pub currency_code: ::std::option::Option<::std::string::String>,
    /// Required. Campaign duration.
    #[prost(message, optional, tag = "3")]
    pub campaign_duration: ::std::option::Option<CampaignDuration>,
    /// Desired cookie frequency cap that will be applied to each planned product.
    /// This is equivalent to the frequency cap exposed in Google Ads when creating
    /// a campaign, it represents the maximum number of times an ad can be shown to
    /// the same user.
    /// If not specified no cap is applied.
    ///
    /// This field is deprecated in v4 and will eventually be removed.
    /// Please use cookie_frequency_cap_setting instead.
    #[prost(message, optional, tag = "4")]
    pub cookie_frequency_cap: ::std::option::Option<i32>,
    /// Desired minimum effective frequency (the number of times a person was
    /// exposed to the ad) for the reported reach metrics [1-10].
    /// This won't affect the targeting, but just the reporting.
    /// If not specified, a default of 1 is applied.
    #[prost(message, optional, tag = "5")]
    pub min_effective_frequency: ::std::option::Option<i32>,
    /// The targeting to be applied to all products selected in the product mix.
    ///
    /// This is planned targeting: execution details might vary based on the
    /// advertising product, please consult an implementation specialist.
    ///
    /// See specific metrics for details on how targeting affects them.
    ///
    /// In some cases, targeting may be overridden using the
    /// PlannedProduct.advanced_product_targeting field.
    #[prost(message, optional, tag = "6")]
    pub targeting: ::std::option::Option<Targeting>,
    /// Required. The products to be forecast.
    /// The max number of allowed planned products is 15.
    #[prost(message, repeated, tag = "7")]
    pub planned_products: ::std::vec::Vec<PlannedProduct>,
}
/// The targeting for which traffic metrics will be reported.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Targeting {
    /// Required. The ID of the selected location.
    /// Plannable locations ID can be obtained from ListPlannableLocations.
    #[prost(message, optional, tag = "1")]
    pub plannable_location_id: ::std::option::Option<::std::string::String>,
    /// Targeted age range.
    /// If not specified, targets all age ranges.
    #[prost(
        enumeration = "super::enums::reach_plan_age_range_enum::ReachPlanAgeRange",
        tag = "2"
    )]
    pub age_range: i32,
    /// Targeted genders.
    /// If not specified, targets all genders.
    #[prost(message, repeated, tag = "3")]
    pub genders: ::std::vec::Vec<super::common::GenderInfo>,
    /// Targeted devices.
    /// If not specified, targets all applicable devices. Applicable devices vary
    /// by product and region and can be obtained from ListPlannableProducts.
    #[prost(message, repeated, tag = "4")]
    pub devices: ::std::vec::Vec<super::common::DeviceInfo>,
    /// Targetable network for the ad product.
    /// If not specified, targets all applicable networks. Applicable networks vary
    /// by product and region and can be obtained from ListPlannableProducts.
    #[prost(
        enumeration = "super::enums::reach_plan_network_enum::ReachPlanNetwork",
        tag = "5"
    )]
    pub network: i32,
}
/// The duration of a planned campaign.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CampaignDuration {
    /// The duration value in days.
    #[prost(message, optional, tag = "1")]
    pub duration_in_days: ::std::option::Option<i32>,
}
/// A product being planned for reach.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlannedProduct {
    /// Required. Selected product for planning.
    /// Plannable products codes can be obtained from ListPlannableProducts.
    #[prost(message, optional, tag = "1")]
    pub plannable_product_code: ::std::option::Option<::std::string::String>,
    /// Required. Maximum budget allocation in micros for the selected product.
    /// The value is specified in the selected planning currency_code.
    /// E.g. 1 000 000$ = 1 000 000 000 000 micros.
    #[prost(message, optional, tag = "2")]
    pub budget_micros: ::std::option::Option<i64>,
}
/// Response message containing the generated reach curve.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateReachForecastResponse {
    /// Reference on target audiences for this curve.
    #[prost(message, optional, tag = "1")]
    pub on_target_audience_metrics: ::std::option::Option<OnTargetAudienceMetrics>,
    /// The generated reach curve for the planned product mix.
    #[prost(message, optional, tag = "2")]
    pub reach_curve: ::std::option::Option<ReachCurve>,
}
/// The reach curve for the planned products.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReachCurve {
    /// All points on the reach curve.
    #[prost(message, repeated, tag = "1")]
    pub reach_forecasts: ::std::vec::Vec<ReachForecast>,
}
/// A point on reach curve.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReachForecast {
    /// The cost in micros.
    #[prost(message, optional, tag = "1")]
    pub cost_micros: ::std::option::Option<i64>,
    /// Forecasted traffic metrics for this point.
    #[prost(message, optional, tag = "2")]
    pub forecast: ::std::option::Option<Forecast>,
    /// The forecasted allocation. This differs from the input allocation if one
    /// or more product cannot fulfill the budget because of limited inventory.
    #[prost(message, repeated, tag = "3")]
    pub forecasted_product_allocations: ::std::vec::Vec<ProductAllocation>,
}
/// Forecasted traffic metrics for the planned products and targeting.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Forecast {
    /// Number of unique people reached at least
    /// GenerateReachForecastRequest.min_effective_frequency times that exactly
    /// matches the Targeting.
    #[prost(message, optional, tag = "1")]
    pub on_target_reach: ::std::option::Option<i64>,
    /// Total number of unique people reached at least
    /// GenerateReachForecastRequest.min_effective_frequency times. This includes
    /// people that may fall outside the specified Targeting.
    #[prost(message, optional, tag = "2")]
    pub total_reach: ::std::option::Option<i64>,
    /// Number of ad impressions that exactly matches the Targeting.
    #[prost(message, optional, tag = "3")]
    pub on_target_impressions: ::std::option::Option<i64>,
    /// Total number of ad impressions. This includes impressions that may fall
    /// outside the specified Targeting, due to insufficient information on
    /// signed-in users.
    #[prost(message, optional, tag = "4")]
    pub total_impressions: ::std::option::Option<i64>,
}
/// Audience metrics for the planned products.
/// These metrics consider the following targeting dimensions:
///
/// - Location
/// - PlannableAgeRange
/// - Gender
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OnTargetAudienceMetrics {
    /// Reference audience size matching the considered targeting for YouTube.
    #[prost(message, optional, tag = "1")]
    pub youtube_audience_size: ::std::option::Option<i64>,
    /// Reference audience size matching the considered targeting for Census.
    #[prost(message, optional, tag = "2")]
    pub census_audience_size: ::std::option::Option<i64>,
}
#[doc = r" Generated client implementations."]
pub mod reach_plan_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Reach Plan Service gives users information about audience size that can"]
    #[doc = " be reached through advertisement on YouTube. In particular,"]
    #[doc = " GenerateReachForecast provides estimated number of people of specified"]
    #[doc = " demographics that can be reached by an ad in a given market by a campaign of"]
    #[doc = " certain duration with a defined budget."]
    pub struct ReachPlanServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> ReachPlanServiceClient<T>
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
        #[doc = " Returns the list of plannable locations (for example, countries & DMAs)."]
        pub async fn list_plannable_locations(
            &mut self,
            request: impl tonic::IntoRequest<super::ListPlannableLocationsRequest>,
        ) -> Result<tonic::Response<super::ListPlannableLocationsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ads.googleads.v3.services.ReachPlanService/ListPlannableLocations",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns the list of per-location plannable YouTube ad formats with allowed"]
        #[doc = " targeting."]
        pub async fn list_plannable_products(
            &mut self,
            request: impl tonic::IntoRequest<super::ListPlannableProductsRequest>,
        ) -> Result<tonic::Response<super::ListPlannableProductsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ads.googleads.v3.services.ReachPlanService/ListPlannableProducts",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Generates a product mix ideas given a set of preferences. This method"]
        #[doc = " helps the advertiser to obtain a good mix of ad formats and budget"]
        #[doc = " allocations based on its preferences."]
        pub async fn generate_product_mix_ideas(
            &mut self,
            request: impl tonic::IntoRequest<super::GenerateProductMixIdeasRequest>,
        ) -> Result<tonic::Response<super::GenerateProductMixIdeasResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ads.googleads.v3.services.ReachPlanService/GenerateProductMixIdeas",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Generates a reach forecast for a given targeting / product mix."]
        pub async fn generate_reach_forecast(
            &mut self,
            request: impl tonic::IntoRequest<super::GenerateReachForecastRequest>,
        ) -> Result<tonic::Response<super::GenerateReachForecastResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ads.googleads.v3.services.ReachPlanService/GenerateReachForecast",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for ReachPlanServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for ReachPlanServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "ReachPlanServiceClient {{ ... }}")
        }
    }
}
/// Request message for [RecommendationService.GetRecommendation][google.ads.googleads.v3.services.RecommendationService.GetRecommendation].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRecommendationRequest {
    /// Required. The resource name of the recommendation to fetch.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
/// Request message for [RecommendationService.ApplyRecommendation][google.ads.googleads.v3.services.RecommendationService.ApplyRecommendation].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApplyRecommendationRequest {
    /// Required. The ID of the customer with the recommendation.
    #[prost(string, tag = "1")]
    pub customer_id: std::string::String,
    /// Required. The list of operations to apply recommendations.
    /// If partial_failure=false all recommendations should be of the same type
    /// There is a limit of 100 operations per request.
    #[prost(message, repeated, tag = "2")]
    pub operations: ::std::vec::Vec<ApplyRecommendationOperation>,
    /// If true, successful operations will be carried out and invalid
    /// operations will return errors. If false, operations will be carried
    /// out as a transaction if and only if they are all valid.
    /// Default is false.
    #[prost(bool, tag = "3")]
    pub partial_failure: bool,
}
/// Information about the operation to apply a recommendation and any parameters
/// to customize it.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApplyRecommendationOperation {
    /// The resource name of the recommendation to apply.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
    /// Parameters to use when applying the recommendation.
    #[prost(
        oneof = "apply_recommendation_operation::ApplyParameters",
        tags = "2, 3, 4, 5, 6, 7, 8, 9"
    )]
    pub apply_parameters: ::std::option::Option<apply_recommendation_operation::ApplyParameters>,
}
pub mod apply_recommendation_operation {
    /// Parameters to use when applying a campaign budget recommendation.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CampaignBudgetParameters {
        /// New budget amount to set for target budget resource. This is a required
        /// field.
        #[prost(message, optional, tag = "1")]
        pub new_budget_amount_micros: ::std::option::Option<i64>,
    }
    /// Parameters to use when applying a text ad recommendation.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TextAdParameters {
        /// New ad to add to recommended ad group. All necessary fields need to be
        /// set in this message. This is a required field.
        #[prost(message, optional, tag = "1")]
        pub ad: ::std::option::Option<super::super::resources::Ad>,
    }
    /// Parameters to use when applying keyword recommendation.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct KeywordParameters {
        /// The ad group resource to add keyword to. This is a required field.
        #[prost(message, optional, tag = "1")]
        pub ad_group: ::std::option::Option<::std::string::String>,
        /// The match type of the keyword. This is a required field.
        #[prost(
            enumeration = "super::super::enums::keyword_match_type_enum::KeywordMatchType",
            tag = "2"
        )]
        pub match_type: i32,
        /// Optional, CPC bid to set for the keyword. If not set, keyword will use
        /// bid based on bidding strategy used by target ad group.
        #[prost(message, optional, tag = "3")]
        pub cpc_bid_micros: ::std::option::Option<i64>,
    }
    /// Parameters to use when applying Target CPA recommendation.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TargetCpaOptInParameters {
        /// Average CPA to use for Target CPA bidding strategy. This is a required
        /// field.
        #[prost(message, optional, tag = "1")]
        pub target_cpa_micros: ::std::option::Option<i64>,
        /// Optional, budget amount to set for the campaign.
        #[prost(message, optional, tag = "2")]
        pub new_campaign_budget_amount_micros: ::std::option::Option<i64>,
    }
    /// Parameters to use when applying callout extension recommendation.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CalloutExtensionParameters {
        /// Callout extensions to be added. This is a required field.
        #[prost(message, repeated, tag = "1")]
        pub callout_extensions: ::std::vec::Vec<super::super::common::CalloutFeedItem>,
    }
    /// Parameters to use when applying call extension recommendation.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CallExtensionParameters {
        /// Call extensions to be added. This is a required field.
        #[prost(message, repeated, tag = "1")]
        pub call_extensions: ::std::vec::Vec<super::super::common::CallFeedItem>,
    }
    /// Parameters to use when applying sitelink extension recommendation.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SitelinkExtensionParameters {
        /// Sitelink extensions to be added. This is a required field.
        #[prost(message, repeated, tag = "1")]
        pub sitelink_extensions: ::std::vec::Vec<super::super::common::SitelinkFeedItem>,
    }
    /// Parameters to use when applying move unused budget recommendation.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct MoveUnusedBudgetParameters {
        /// Budget amount to move from excess budget to constrained budget. This is
        /// a required field.
        #[prost(message, optional, tag = "1")]
        pub budget_micros_to_move: ::std::option::Option<i64>,
    }
    /// Parameters to use when applying the recommendation.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ApplyParameters {
        /// Optional parameters to use when applying a campaign budget
        /// recommendation.
        #[prost(message, tag = "2")]
        CampaignBudget(CampaignBudgetParameters),
        /// Optional parameters to use when applying a text ad recommendation.
        #[prost(message, tag = "3")]
        TextAd(TextAdParameters),
        /// Optional parameters to use when applying keyword recommendation.
        #[prost(message, tag = "4")]
        Keyword(KeywordParameters),
        /// Optional parameters to use when applying target CPA opt-in
        /// recommendation.
        #[prost(message, tag = "5")]
        TargetCpaOptIn(TargetCpaOptInParameters),
        /// Parameters to use when applying callout extension recommendation.
        #[prost(message, tag = "6")]
        CalloutExtension(CalloutExtensionParameters),
        /// Parameters to use when applying call extension recommendation.
        #[prost(message, tag = "7")]
        CallExtension(CallExtensionParameters),
        /// Parameters to use when applying sitelink extension recommendation.
        #[prost(message, tag = "8")]
        SitelinkExtension(SitelinkExtensionParameters),
        /// Parameters to use when applying move unused budget recommendation.
        #[prost(message, tag = "9")]
        MoveUnusedBudget(MoveUnusedBudgetParameters),
    }
}
/// Response message for [RecommendationService.ApplyRecommendation][google.ads.googleads.v3.services.RecommendationService.ApplyRecommendation].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApplyRecommendationResponse {
    /// Results of operations to apply recommendations.
    #[prost(message, repeated, tag = "1")]
    pub results: ::std::vec::Vec<ApplyRecommendationResult>,
    /// Errors that pertain to operation failures in the partial failure mode.
    /// Returned only when partial_failure = true and all errors occur inside the
    /// operations. If any errors occur outside the operations (e.g. auth errors)
    /// we return the RPC level error.
    #[prost(message, optional, tag = "2")]
    pub partial_failure_error: ::std::option::Option<super::super::super::super::rpc::Status>,
}
/// The result of applying a recommendation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ApplyRecommendationResult {
    /// Returned for successful applies.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
/// Request message for [RecommendationService.DismissRecommendation][google.ads.googleads.v3.services.RecommendationService.DismissRecommendation].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DismissRecommendationRequest {
    /// Required. The ID of the customer with the recommendation.
    #[prost(string, tag = "1")]
    pub customer_id: std::string::String,
    /// Required. The list of operations to dismiss recommendations.
    /// If partial_failure=false all recommendations should be of the same type
    /// There is a limit of 100 operations per request.
    #[prost(message, repeated, tag = "3")]
    pub operations: ::std::vec::Vec<dismiss_recommendation_request::DismissRecommendationOperation>,
    /// If true, successful operations will be carried out and invalid
    /// operations will return errors. If false, operations will be carried in a
    /// single transaction if and only if they are all valid.
    /// Default is false.
    #[prost(bool, tag = "2")]
    pub partial_failure: bool,
}
pub mod dismiss_recommendation_request {
    /// Operation to dismiss a single recommendation identified by resource_name.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DismissRecommendationOperation {
        /// The resource name of the recommendation to dismiss.
        #[prost(string, tag = "1")]
        pub resource_name: std::string::String,
    }
}
/// Response message for [RecommendationService.DismissRecommendation][google.ads.googleads.v3.services.RecommendationService.DismissRecommendation].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DismissRecommendationResponse {
    /// Results of operations to dismiss recommendations.
    #[prost(message, repeated, tag = "1")]
    pub results: ::std::vec::Vec<dismiss_recommendation_response::DismissRecommendationResult>,
    /// Errors that pertain to operation failures in the partial failure mode.
    /// Returned only when partial_failure = true and all errors occur inside the
    /// operations. If any errors occur outside the operations (e.g. auth errors)
    /// we return the RPC level error.
    #[prost(message, optional, tag = "2")]
    pub partial_failure_error: ::std::option::Option<super::super::super::super::rpc::Status>,
}
pub mod dismiss_recommendation_response {
    /// The result of dismissing a recommendation.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DismissRecommendationResult {
        /// Returned for successful dismissals.
        #[prost(string, tag = "1")]
        pub resource_name: std::string::String,
    }
}
#[doc = r" Generated client implementations."]
pub mod recommendation_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Service to manage recommendations."]
    pub struct RecommendationServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> RecommendationServiceClient<T>
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
        #[doc = " Returns the requested recommendation in full detail."]
        pub async fn get_recommendation(
            &mut self,
            request: impl tonic::IntoRequest<super::GetRecommendationRequest>,
        ) -> Result<tonic::Response<super::super::resources::Recommendation>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ads.googleads.v3.services.RecommendationService/GetRecommendation",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Applies given recommendations with corresponding apply parameters."]
        pub async fn apply_recommendation(
            &mut self,
            request: impl tonic::IntoRequest<super::ApplyRecommendationRequest>,
        ) -> Result<tonic::Response<super::ApplyRecommendationResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ads.googleads.v3.services.RecommendationService/ApplyRecommendation",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Dismisses given recommendations."]
        pub async fn dismiss_recommendation(
            &mut self,
            request: impl tonic::IntoRequest<super::DismissRecommendationRequest>,
        ) -> Result<tonic::Response<super::DismissRecommendationResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ads.googleads.v3.services.RecommendationService/DismissRecommendation",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for RecommendationServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for RecommendationServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "RecommendationServiceClient {{ ... }}")
        }
    }
}
/// Request message for [SearchTermViewService.GetSearchTermView][google.ads.googleads.v3.services.SearchTermViewService.GetSearchTermView].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSearchTermViewRequest {
    /// Required. The resource name of the search term view to fetch.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
#[doc = r" Generated client implementations."]
pub mod search_term_view_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Service to manage search term views."]
    pub struct SearchTermViewServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> SearchTermViewServiceClient<T>
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
        #[doc = " Returns the attributes of the requested search term view."]
        pub async fn get_search_term_view(
            &mut self,
            request: impl tonic::IntoRequest<super::GetSearchTermViewRequest>,
        ) -> Result<tonic::Response<super::super::resources::SearchTermView>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ads.googleads.v3.services.SearchTermViewService/GetSearchTermView",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for SearchTermViewServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for SearchTermViewServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "SearchTermViewServiceClient {{ ... }}")
        }
    }
}
/// Request message for
/// [ShoppingPerformanceViewService.GetShoppingPerformanceView][google.ads.googleads.v3.services.ShoppingPerformanceViewService.GetShoppingPerformanceView].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetShoppingPerformanceViewRequest {
    /// Required. The resource name of the Shopping performance view to fetch.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
#[doc = r" Generated client implementations."]
pub mod shopping_performance_view_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Service to fetch Shopping performance views."]
    pub struct ShoppingPerformanceViewServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> ShoppingPerformanceViewServiceClient<T>
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
        #[doc = " Returns the requested Shopping performance view in full detail."]
        pub async fn get_shopping_performance_view(
            &mut self,
            request: impl tonic::IntoRequest<super::GetShoppingPerformanceViewRequest>,
        ) -> Result<tonic::Response<super::super::resources::ShoppingPerformanceView>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ( "/google.ads.googleads.v3.services.ShoppingPerformanceViewService/GetShoppingPerformanceView" ) ;
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for ShoppingPerformanceViewServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for ShoppingPerformanceViewServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "ShoppingPerformanceViewServiceClient {{ ... }}")
        }
    }
}
/// Request message for [TopicConstantService.GetTopicConstant][google.ads.googleads.v3.services.TopicConstantService.GetTopicConstant].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTopicConstantRequest {
    /// Required. Resource name of the Topic to fetch.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
#[doc = r" Generated client implementations."]
pub mod topic_constant_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Service to fetch topic constants."]
    pub struct TopicConstantServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> TopicConstantServiceClient<T>
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
        #[doc = " Returns the requested topic constant in full detail."]
        pub async fn get_topic_constant(
            &mut self,
            request: impl tonic::IntoRequest<super::GetTopicConstantRequest>,
        ) -> Result<tonic::Response<super::super::resources::TopicConstant>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ads.googleads.v3.services.TopicConstantService/GetTopicConstant",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for TopicConstantServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for TopicConstantServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "TopicConstantServiceClient {{ ... }}")
        }
    }
}
/// Request message for [TopicViewService.GetTopicView][google.ads.googleads.v3.services.TopicViewService.GetTopicView].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTopicViewRequest {
    /// Required. The resource name of the topic view to fetch.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
#[doc = r" Generated client implementations."]
pub mod topic_view_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Service to manage topic views."]
    pub struct TopicViewServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> TopicViewServiceClient<T>
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
        #[doc = " Returns the requested topic view in full detail."]
        pub async fn get_topic_view(
            &mut self,
            request: impl tonic::IntoRequest<super::GetTopicViewRequest>,
        ) -> Result<tonic::Response<super::super::resources::TopicView>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ads.googleads.v3.services.TopicViewService/GetTopicView",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for TopicViewServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for TopicViewServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "TopicViewServiceClient {{ ... }}")
        }
    }
}
/// Request message for [UserDataService.UploadUserData][google.ads.googleads.v3.services.UserDataService.UploadUserData]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UploadUserDataRequest {
    /// Required. The ID of the customer for which to update the user data.
    #[prost(string, tag = "1")]
    pub customer_id: std::string::String,
    /// Required. The list of operations to be done.
    #[prost(message, repeated, tag = "3")]
    pub operations: ::std::vec::Vec<UserDataOperation>,
    /// Metadata of the request.
    #[prost(oneof = "upload_user_data_request::Metadata", tags = "2")]
    pub metadata: ::std::option::Option<upload_user_data_request::Metadata>,
}
pub mod upload_user_data_request {
    /// Metadata of the request.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Metadata {
        /// Metadata for data updates to a Customer Match user list.
        #[prost(message, tag = "2")]
        CustomerMatchUserListMetadata(super::super::common::CustomerMatchUserListMetadata),
    }
}
/// Operation to be made for the UploadUserDataRequest.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserDataOperation {
    /// Operation to be made for the UploadUserDataRequest.
    #[prost(oneof = "user_data_operation::Operation", tags = "1")]
    pub operation: ::std::option::Option<user_data_operation::Operation>,
}
pub mod user_data_operation {
    /// Operation to be made for the UploadUserDataRequest.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Operation {
        /// The list of user data to be appended to the user list.
        #[prost(message, tag = "1")]
        Create(super::super::common::UserData),
    }
}
/// Response message for [UserDataService.UploadUserData][google.ads.googleads.v3.services.UserDataService.UploadUserData]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UploadUserDataResponse {
    /// The date time at which the request was received by API, formatted as
    /// "yyyy-mm-dd hh:mm:ss+|-hh:mm", e.g. "2019-01-01 12:32:45-08:00".
    #[prost(message, optional, tag = "1")]
    pub upload_date_time: ::std::option::Option<::std::string::String>,
    /// Number of upload data operations received by API.
    #[prost(message, optional, tag = "2")]
    pub received_operations_count: ::std::option::Option<i32>,
}
#[doc = r" Generated client implementations."]
pub mod user_data_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Service to manage user data uploads."]
    #[doc = " Accessible only to customers on the allow-list."]
    pub struct UserDataServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> UserDataServiceClient<T>
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
        #[doc = " Uploads the given user data."]
        pub async fn upload_user_data(
            &mut self,
            request: impl tonic::IntoRequest<super::UploadUserDataRequest>,
        ) -> Result<tonic::Response<super::UploadUserDataResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ads.googleads.v3.services.UserDataService/UploadUserData",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for UserDataServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for UserDataServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "UserDataServiceClient {{ ... }}")
        }
    }
}
/// Request message for [UserInterestService.GetUserInterest][google.ads.googleads.v3.services.UserInterestService.GetUserInterest].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetUserInterestRequest {
    /// Required. Resource name of the UserInterest to fetch.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
#[doc = r" Generated client implementations."]
pub mod user_interest_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Service to fetch Google Ads User Interest."]
    pub struct UserInterestServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> UserInterestServiceClient<T>
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
        #[doc = " Returns the requested user interest in full detail"]
        pub async fn get_user_interest(
            &mut self,
            request: impl tonic::IntoRequest<super::GetUserInterestRequest>,
        ) -> Result<tonic::Response<super::super::resources::UserInterest>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ads.googleads.v3.services.UserInterestService/GetUserInterest",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for UserInterestServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for UserInterestServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "UserInterestServiceClient {{ ... }}")
        }
    }
}
/// Request message for [UserLocationViewService.GetUserLocationView][google.ads.googleads.v3.services.UserLocationViewService.GetUserLocationView].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetUserLocationViewRequest {
    /// Required. The resource name of the user location view to fetch.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
#[doc = r" Generated client implementations."]
pub mod user_location_view_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Service to manage user location views."]
    pub struct UserLocationViewServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> UserLocationViewServiceClient<T>
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
        #[doc = " Returns the requested user location view in full detail."]
        pub async fn get_user_location_view(
            &mut self,
            request: impl tonic::IntoRequest<super::GetUserLocationViewRequest>,
        ) -> Result<tonic::Response<super::super::resources::UserLocationView>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ads.googleads.v3.services.UserLocationViewService/GetUserLocationView",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for UserLocationViewServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for UserLocationViewServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "UserLocationViewServiceClient {{ ... }}")
        }
    }
}
/// Request message for [VideoService.GetVideo][google.ads.googleads.v3.services.VideoService.GetVideo].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetVideoRequest {
    /// Required. The resource name of the video to fetch.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
#[doc = r" Generated client implementations."]
pub mod video_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Service to manage videos."]
    pub struct VideoServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl<T> VideoServiceClient<T>
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
        #[doc = " Returns the requested video in full detail."]
        pub async fn get_video(
            &mut self,
            request: impl tonic::IntoRequest<super::GetVideoRequest>,
        ) -> Result<tonic::Response<super::super::resources::Video>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.ads.googleads.v3.services.VideoService/GetVideo",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for VideoServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for VideoServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "VideoServiceClient {{ ... }}")
        }
    }
}
